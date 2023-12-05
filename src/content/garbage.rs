use rand::{Rng, thread_rng};
use robotics_lib::world::tile::{Content, Tile, TileType};

use crate::generator::GarbageSettings;

pub(crate) fn spawn_garbage(world: &mut Vec<Vec<Tile>>, settings: &GarbageSettings) {
    //TODO: set to GarbageSettings the total amount of garbage to spawn
    //TODO: set to GarbageSettings the amount of garbage to spawn in each build-up
    //TODO: set to GarbageSettings the size of each build-up (maybe ask for a minimum and maximum size? -> range?)
    //TODO: set to GarbageSettings the step probability
    //TODO: rename max_amount_on_destroy to max_garbage_per_tile (or something like that) (maybe ask for the range?)
    //TODO: implement a default implementation for GarbageSettings with settings based on the world size
    //TODO: convert the body of this function to a method `spawn_garbage_build_up` and call it from here (to spawn multiple build-ups)

    let mut rng = thread_rng();

    // Note that the matrix size will be rounded to greater odd number
    let probability_matrix = generate_prob_matrix(7, 0.2);

    // get random x and y coordinates, the base indexes where matrix garbages will starts
    let map_range = 0..world.len();

    let base_y = rng.gen_range(map_range.clone());
    let base_x = rng.gen_range(map_range.clone());

    // println!("Garbage in: ({},{})", base_y, base_x);

    //(x,y) will be the (0,0) of the probability matrix (not the center cause im lazy)
    for (row_index, row) in probability_matrix.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            // get the random value for the spawn
            let value: f64 = thread_rng().gen_range(0.0..=1.0);

            // assign if the probability is satisfied
            if value > (1. - probability_matrix[row_index][col_index]) {

                // get random amount of garbage fot the tile content
                let amount = rng.gen_range(1..settings.max_amount_on_destroy);
                set_content(world, base_y + col_index, base_x + row_index, amount);
            }
        }
    }
}

#[inline(always)]
fn set_content(world: &mut [Vec<Tile>], y: usize, x: usize, amount: usize) -> bool {
    if y == 0 || y >= world.len() || x == 0 || x >= world.len() {
        return false;
    }

    match world[y][x].tile_type {
        | TileType::Sand
        | TileType::Grass
        | TileType::Street
        | TileType::Hill
        | TileType::Mountain
        | TileType::Teleport(_) => {
            world[y][x].content = Content::Garbage(amount);
            true
        }
        | _ => false,
    }
}

// probability matrix
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
        for col_index in (0 + ring)..(size - ring) {
            matrix[ring][col_index] = prob;
        }

        // iterate over the last row of the ring
        for col_index in (0 + ring)..(size - ring) {
            matrix[size - 1 - ring][col_index] = prob;
        }

        // iterate over the first column of the ring
        for row_index in (0 + ring)..(size - ring) {
            matrix[row_index][ring] = prob;
        }

        // iterate over the last column of the ring
        for row_index in (0 + ring)..(size - ring) {
            matrix[row_index][size - 1 - ring] = prob;
        }
    }
    matrix
}

