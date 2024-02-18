use rand::{Rng, thread_rng};
use robotics_lib::world::tile::Content::Fish;
use serde::{Deserialize, Serialize};

use crate::generator::TileMatrix;
use crate::utils::spawn_content_randomly;

/// Settings defining the behavior of fish spawn,
/// such as the number of spawn points
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct FishSettings {
    pub number_of_spawn_points: usize,
}

impl FishSettings {
    /// Custom version of default that provides an instance of `FishSettings` with the
    /// optimal parameters for the given world size
    pub fn default(size: usize) -> Self {
        FishSettings {
            number_of_spawn_points: usize::pow(size, 2) / 25,
        }
    }

    /// Creates a new instance of `FishSettings` with the given number of spawn points.
    ///
    /// # Arguments
    ///
    /// * `spawn_points` - The number of spawn points for fish within the world.
    ///
    /// # Returns
    ///
    /// A new `FishSettings` instance with the specified number of spawn points.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use exclusion_zone::content::fish::FishSettings;
    /// let settings = FishSettings::new(5);
    /// ```
    pub fn new(spawn_points: usize) -> Self {
        FishSettings {
            number_of_spawn_points: spawn_points,
        }
    }
}

pub(crate) fn spawn_fish(world: &mut TileMatrix, fish: FishSettings) {
    let max = Fish(0).properties().max();
    let spawn_points = spawn_content_randomly(world, fish.number_of_spawn_points, Fish(0).to_default());

    for c in spawn_points {
        let random = thread_rng().gen_range(1..=max);
        world[c.row][c.col].content = Fish(random);
    }
}

