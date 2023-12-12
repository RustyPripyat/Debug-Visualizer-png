use std::cmp::min;
use std::ops::Range;

use rand::{Rng, thread_rng};
use rand::prelude::ThreadRng;
use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::tile::Content::Garbage;

use crate::generator::TileMatrix;

/// Settings defining the behavior of garbage spawn.
///
/// This struct represents the configuration for garbage spawn, including the total quantity
/// of garbage, pile sizes, quantity per tile and the likelihood that it will spawn a pile.
#[derive(Clone)]
pub struct GarbageSettings {
    pub total_garbage_quantity: usize,
    pub garbage_pile_size: Range<usize>,
    pub garbage_per_tile_quantity: Range<usize>,
    pub spawn_in_near_tiles_probability: f64,
    pub probability_step_by: f64,
}

impl GarbageSettings {
    /// Custom version of default that provides an instance of `GarbageSettings` with the
    /// optimal parameters for the given world size. Setting a size too large may lead to
    /// loss in speed generation
    pub fn default(size: usize) -> Self {
        GarbageSettings {
            total_garbage_quantity: size / 2,
            garbage_pile_size: 1..size / 10,
            garbage_per_tile_quantity: 1..Garbage(0).properties().max(),
            spawn_in_near_tiles_probability: 1.0,
            probability_step_by: 0.2,
        }
    }

    /// Creates a new instance of `GarbageSettings` with the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `total_garbage_quantity` - The total quantity of garbage.
    /// * `garbage_pile_size` - The range representing pile sizes.
    /// * `garbage_per_tile_quantity` - The range representing quantity per tile.
    /// * `spawn_in_near_tiles_probability` - Likelihood that garbage will spawn in near tiles.
    /// * `probability_step_by` - Step by which probability increases/decreases.
    ///
    /// # Returns
    ///
    /// A new instance of `GarbageSettings` initialized with the provided parameters.
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::Range;
    /// use exclusion_zone::content::garbage::GarbageSettings;
    ///
    /// let settings = GarbageSettings::with_parameters(
    ///     1000,
    ///     5..=10,
    ///     1..=3,
    ///     0.7,
    ///     0.1,
    /// );
    /// ```
    pub fn new(
        total_garbage_quantity: usize,
        garbage_pile_size: Range<usize>,
        garbage_per_tile_quantity: Range<usize>,
        spawn_in_near_tiles_probability: f64,
        probability_step_by: f64,
    ) -> Self {
        GarbageSettings {
            total_garbage_quantity,
            garbage_pile_size,
            garbage_per_tile_quantity,
            spawn_in_near_tiles_probability,
            probability_step_by,
        }
    }
}

pub(crate) fn spawn_garbage(world: &mut TileMatrix, settings: &GarbageSettings) {
    let mut i = 0;
    let mut rng = thread_rng();
    let max_amount = min(settings.garbage_per_tile_quantity.clone().max().unwrap_or(1), Garbage(0).properties().max());
    let spawn_prob = f64::max(0.2, settings.spawn_in_near_tiles_probability);
    while i < settings.total_garbage_quantity {
        spawn_garbage_build_up(world, settings.garbage_pile_size.clone(), settings.probability_step_by, spawn_prob, &mut i, &mut rng, max_amount);
    }
}

#[inline(always)]
pub(crate) fn spawn_garbage_build_up(world: &mut TileMatrix, garbage_pile_size: Range<usize>, probability_step_by: f64, spawn_prob: f64, placed: &mut usize, rng: &mut ThreadRng, max_garbage_per_tile: usize) {
    // Get size of garbage pile
    let pile_range = rng.gen_range(garbage_pile_size);

    // Note that the matrix size will be rounded to greater odd number
    let probability_matrix = generate_prob_matrix(pile_range, probability_step_by);

    // get random x and y coordinates, the base indexes where matrix garbage will starts
    let map_range = 0..world.len();

    let base_y = rng.gen_range(map_range.clone());
    let base_x = rng.gen_range(map_range.clone());

    //(x,y) will be the (0,0) of the probability matrix (not the center cause im lazy)
    for (row_index, row) in probability_matrix.iter().enumerate() {
        for col_index in 0..row.len() {
            // get the random value for the spawn
            let value: f64 = thread_rng().gen_range(0.1..=spawn_prob);

            // assign if the probability is satisfied
            if value > (1. - probability_matrix[row_index][col_index]) {
                // get random amount of garbage fot the tile content
                let random_amount = rng.gen_range(1..max_garbage_per_tile);
                if set_content(world, base_y + col_index, base_x + row_index, random_amount, probability_matrix.len()) {
                    *placed += random_amount;
                }
            }
        }
    }
}

#[inline(always)]
fn set_content(world: &mut [Vec<Tile>], y: usize, x: usize, amount: usize, mat_size: usize) -> bool {
    if y == 0 || y >= world.len() - mat_size || x == 0 || x >= world.len() - mat_size {
        return false;
    }

    if world[y][x].tile_type.properties().can_hold(&Garbage(0)) && !(world[y][x].content != Content::None) {
        world[y][x].content = Garbage(amount);
        true
    } else {
        false
    }
}

// probability matrix
#[inline(always)]
fn generate_prob_matrix(mut size: usize, probability_step: f64) -> Vec<Vec<f64>> {
    // some edgy checks
    if size == 0 {
        return vec![vec![]];
    } else if size / 2 == 1 {
        size += 1; //we want the size to be odd
    }

    // initialize the matrix and calculate the total number of rings
    let mut matrix = vec![vec![0.0; size]; size];
    let total_rings = size / 2; // total number of ring

    // iterate over the ring
    for ring in 0..total_rings {
        // assign the probability for the ring
        let prob = 1. - probability_step * ((total_rings - ring) as f64);

        // iterate over the first row of the ring
        for col_index in ring..(size - ring) {
            matrix[ring][col_index] = prob;
            matrix[size - 1 - ring][col_index] = prob;
        }

        // iterate over the first column of the ring
        for row_index in ring..(size - ring) {
            matrix[row_index][ring] = prob;
            matrix[row_index][size - 1 - ring] = prob;
        }
    }
    matrix
}
