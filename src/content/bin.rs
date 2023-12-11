use debug_print::debug_println;
use rand::{thread_rng, Rng};
use robotics_lib::world::tile::Content::Bin;
use robotics_lib::world::tile::Tile;

// spawn bin randomly in the world
use crate::utils::spawn_content_randomly;

impl BinSettings {
    // Custom constructor that takes a size parameter
    pub fn default(size: usize) -> Self {
        BinSettings {
            number_of_spawn_points: size / 25,
        }
    }
}

#[derive(Clone)]
pub struct BinSettings {
    pub(crate) number_of_spawn_points: usize,
}

pub(crate) fn spawn_bin(world: &mut Vec<Vec<Tile>>, bin_settings: BinSettings) {
    let max = Bin(0..0).properties().max();
    let spawn_points = spawn_content_randomly(world, bin_settings.number_of_spawn_points, Bin(0..0));

    for (y, x) in spawn_points {
        let upper_bound = thread_rng().gen_range(2..=max);
        world[y][x].content = Bin(1..upper_bound);
        debug_println!("spawned bin at {},{} with upper bound {}", x, y, upper_bound);
    }
}
