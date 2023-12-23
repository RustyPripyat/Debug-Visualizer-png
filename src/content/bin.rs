use rand::{Rng, thread_rng};
use robotics_lib::world::tile::Content::Bin;
use serde::{Deserialize, Serialize};

use crate::generator::TileMatrix;
use crate::utils::spawn_content_randomly;

/// Settings defining the behavior of bins spawn,
/// such as the number of spawn points
#[derive(Serialize, Deserialize, Clone)]
pub struct BinSettings {
    pub number_of_spawn_points: usize,
}

impl BinSettings {
    /// Custom version of default that provides an instance of `BinSettings` with the
    /// optimal parameters for the given world size
    pub fn default(size: usize) -> Self {
        BinSettings {
            number_of_spawn_points: size / 25,
        }
    }

    /// Creates a new instance of `BinSettings` with the given number of spawn points.
    ///
    /// # Arguments
    ///
    /// * `spawn_points` - The number of spawn points for bins within the world.
    ///
    /// # Returns
    ///
    /// A new `BinSettings` instance with the specified number of spawn points.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use exclusion_zone::content::bin::BinSettings;
    /// let settings = BinSettings::new(5);
    /// ```
    pub fn new(spawn_points: usize) -> Self {
        BinSettings {
            number_of_spawn_points: spawn_points,
        }
    }
}

pub(crate) fn spawn_bin(world: &mut TileMatrix, bin_settings: BinSettings) {
    let max = Bin(0..0).properties().max();
    let spawn_points = spawn_content_randomly(world, bin_settings.number_of_spawn_points, Bin(0..0));

    for c in spawn_points {
        let upper_bound = thread_rng().gen_range(2..=max);
        world[c.row][c.col].content = Bin(1..upper_bound);
    }
}
