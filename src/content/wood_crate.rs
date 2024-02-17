use rand::{thread_rng, Rng};
use robotics_lib::world::tile::Content::Crate;
use serde::{Deserialize, Serialize};

use crate::generator::TileMatrix;
use crate::utils::spawn_content_randomly;

/// Settings defining the behavior of wood crate spawn,
/// such as the number of spawn points
#[derive(Serialize, Deserialize, Clone)]
pub struct CrateSettings {
    pub number_of_spawn_points: usize,
}

impl CrateSettings {
    /// Custom version of default that provides an instance of `CrateSettings` with the
    /// optimal parameters for the given world size
    pub fn default(size: usize) -> Self {
        CrateSettings {
            number_of_spawn_points: size * size / 40,
        }
    }

    /// Creates a new instance of `CrateSettings` with the specified number of spawn points.
    ///
    /// # Arguments
    ///
    /// * `number_of_spawn_points` - The number of spawn points for wood crates within the world.
    ///
    /// # Returns
    ///
    /// A new instance of `CrateSettings` initialized with the provided number of spawn points.
    ///
    /// # Example
    ///
    /// ```
    /// use exclusion_zone::content::wood_crate::CrateSettings;
    ///
    /// let settings = CrateSettings::new(10);
    /// ```
    pub fn new(number_of_spawn_points: usize) -> Self {
        Self {
            number_of_spawn_points,
        }
    }
}

pub(crate) fn spawn_crate(world: &mut TileMatrix, crate_settings: CrateSettings) {
    let max = Crate(0..0).properties().max();
    let spawn_points = spawn_content_randomly(world, crate_settings.number_of_spawn_points, Crate(0..0));

    for c in spawn_points {
        let upper_bound = thread_rng().gen_range(2..=max);
        world[c.row][c.col].content = Crate(1..upper_bound);
    }
}
