// spawn bin randomly in the world
use rand::{Rng, thread_rng};
use robotics_lib::world::tile::Tile;
use robotics_lib::world::tile::Content::Bin;
use crate::utils::spawn_content_randomly;

impl BinSettings {
    // Custom constructor that takes a size parameter
    pub(crate) fn default(size: usize) -> Self {
        BinSettings {
            number_of_spawn_points: size / 25,
        }
    }
}

#[derive(Clone)]
pub(crate) struct BinSettings {
    pub(crate) number_of_spawn_points: usize,
}

pub(crate) fn spawn_bin(world: &mut Vec<Vec<Tile>>, bin_settings: BinSettings) {

    let rng = rand::thread_rng();
    let max = Bin(0..0).properties().max();
    let spawn_points= spawn_content_randomly(world, bin_settings.number_of_spawn_points, Bin(0..0));

    for (y, x) in spawn_points {
        let upper_bound = thread_rng().gen_range(2..=max);
        world[y][x].content = Bin(1 ..upper_bound);
        println!("spawned bin at {},{} with upper bound {}", x, y, upper_bound);
    }
}