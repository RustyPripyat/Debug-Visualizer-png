use rand::{Rng, thread_rng};
use robotics_lib::world::tile::Content::Bin;

use crate::generator::World;
use crate::utils::spawn_content_randomly;

impl BinSettings {
    /// Custom version of default that provides an instance of `BinSettings` with the
    /// optimal parameters for the given world size
    pub fn default(size: usize) -> Self {
        BinSettings {
            number_of_spawn_points: size / 25,
        }
    }
}

/// Settings defining the behavior of bins spawn,
/// such as the number of spawn points
#[derive(Clone)]
pub struct BinSettings {
    pub(crate) number_of_spawn_points: usize,
}

pub(crate) fn spawn_bin(world: &mut World, bin_settings: BinSettings) {
    let max = Bin(0..0).properties().max();
    let spawn_points = spawn_content_randomly(world, bin_settings.number_of_spawn_points, Bin(0..0));

    for (y, x) in spawn_points {
        let upper_bound = thread_rng().gen_range(2..=max);
        world[y][x].content = Bin(1..upper_bound);
    }
}
