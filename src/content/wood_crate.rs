use rand::{Rng, thread_rng};
use robotics_lib::world::tile::Content::Crate;

use crate::generator::World;
use crate::utils::spawn_content_randomly;

impl CrateSettings {
    /// Custom version of default that provides an instance of `CrateSettings` with the
    /// optimal parameters for the given world size
    pub fn default(size: usize) -> Self {
        CrateSettings {
            number_of_spawn_points: size / 25,
        }
    }
}

/// Settings defining the behavior of wood crate spawn,
/// such as the number of spawn points
#[derive(Clone)]
pub struct CrateSettings {
    pub(crate) number_of_spawn_points: usize,
}

pub(crate) fn spawn_crate(world: &mut World, crate_settings: CrateSettings) {
    let max = Crate(0..0).properties().max();
    let spawn_points = spawn_content_randomly(world, crate_settings.number_of_spawn_points, Crate(0..0));

    for (y, x) in spawn_points {
        let upper_bound = thread_rng().gen_range(2..=max);
        world[y][x].content = Crate(1..upper_bound);
    }
}
