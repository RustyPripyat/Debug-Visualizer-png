use std::ops::Range;

use rand::{Rng, thread_rng};
use robotics_lib::world::tile::{Content, Tile};

#[derive(Clone)]
pub(crate) struct GarbageSettings {
    pub(crate) spawn_points_quantity: usize,
    pub(crate) garbage_pile_size: Range<usize>,
    pub(crate) garbage_per_tile_quantity: Range<usize>,
    pub(crate) spawn_in_near_tiles_probability: f64,
    pub(crate) probability_step_by: f64,
}

impl GarbageSettings {
    /// Initialize the struct with optimal parameters given the world size
    pub(crate) fn default(size: usize) -> Self {
        GarbageSettings {
            spawn_points_quantity: size,
            garbage_pile_size: 1..size/5,
            garbage_per_tile_quantity: 1..size/300,
            spawn_in_near_tiles_probability: 0.8,
            probability_step_by: 0.2,
        }
    }
}

pub(crate) fn spawn_garbage(world: &mut Vec<Vec<Tile>>, settings: &GarbageSettings) {
    //TODO: check if at least some of the build-up can spawn (or even the whole build-up) [pls follow the code]
    //TODO: check if some of the build-ups are overlapping

    let mut rng = thread_rng();
    let mut i = 0;
    let mut placed = 0;

    while i < settings.spawn_points_quantity {
        // Get size of garbage pile
        let pile_range = rng.gen_range(settings.garbage_pile_size.clone());

        // Note that the matrix size will be rounded to greater odd number
        let probability_matrix = generate_prob_matrix(pile_range, settings.probability_step_by);

        // get random x and y coordinates, the base indexes where matrix garbage will starts
        let map_range = 0..world.len();

        let base_y = rng.gen_range(map_range.clone());
        let base_x = rng.gen_range(map_range.clone());

        //(x,y) will be the (0,0) of the probability matrix (not the center cause im lazy)
        for (row_index, row) in probability_matrix.iter().enumerate() {
            for col_index in 0..row.len() {
                // get the random value for the spawn
                let value: f64 = thread_rng().gen_range(0.1..=settings.spawn_in_near_tiles_probability);

                // assign if the probability is satisfied
                if value > (1. - probability_matrix[row_index][col_index]) {
                    // get random amount of garbage fot the tile content
                    let amount = rng.gen_range(settings.garbage_per_tile_quantity.clone());
                    if set_content(world, base_y + col_index, base_x + row_index, amount) {
                        i += 1;
                        placed += amount;
                    }
                }
            }
        }
    }
    println!("placed {}", placed)
}

#[inline(always)]
fn set_content(world: &mut [Vec<Tile>], y: usize, x: usize, amount: usize) -> bool {
    if y == 0 || y >= world.len() || x == 0 || x >= world.len() {
        return false;
    }

    if world[y][x].tile_type.properties().can_hold(&Content::Garbage(0)) {
        world[y][x].content = Content::Garbage(amount);
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
