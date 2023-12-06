// spawn crate randomly in the world
use rand::{Rng, thread_rng};
use robotics_lib::world::tile::Tile;
use robotics_lib::world::tile::Content::Crate;
use crate::utils::spawn_content_randomly;

impl CrateSettings {
    // Custom constructor that takes a size parameter
    pub(crate) fn default(size: usize) -> Self {
        CrateSettings {
            number_of_spawn_points: size / 25,
        }
    }
}

#[derive(Clone)]
pub(crate) struct CrateSettings {
    pub(crate) number_of_spawn_points: usize,
}

pub(crate) fn spawn_crate(world: &mut Vec<Vec<Tile>>, crate_settings: CrateSettings) {

    let rng = thread_rng();
    let max = Crate(0..0).properties().max();
    let spawn_points= spawn_content_randomly(world, crate_settings.number_of_spawn_points, Crate(0..0));

    for (y, x) in spawn_points {
        let upper_bound = thread_rng().gen_range(2..=max);
        world[y][x].content = Crate(1 ..upper_bound);
        println!("spawned crate at {},{} with upper bound {}", x, y, upper_bound)
    }
}