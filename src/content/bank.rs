use rand::{Rng, thread_rng};
use robotics_lib::world::tile::Content::Bank;
use serde::{Deserialize, Serialize};

use crate::generator::TileMatrix;
use crate::utils::spawn_content_randomly;

/// Settings defining the behavior of bank spawn,
/// such as the number of spawn points
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct BankSettings {
    /// the number of banks to spawn
    pub number_of_spawn_points: usize,
}

impl BankSettings {
    /// Custom version of default that provides an instance of `BankSettings` with the
    /// optimal parameters for the given world size
    pub fn default(size: usize) -> Self {
        BankSettings {
            number_of_spawn_points: size / 25,
        }
    }

    /// Creates a new instance of `BankSettings` with the given number of spawn points.
    ///
    /// # Arguments
    ///
    /// * `number_of_spawn_points` - The number of banks to spawn within the world.
    ///
    /// # Returns
    ///
    /// A new `BankSettings` instance with the specified number of spawn points.
    ///
    /// # Examples
    ///
    /// ```
    /// use exclusion_zone::content::bank::BankSettings;
    ///
    /// let settings = BankSettings::new(10);
    /// ```
    pub fn new(number_of_spawn_points: usize) -> Self {
        BankSettings {
            number_of_spawn_points,
        }
    }
}

pub(crate) fn spawn_bank(world: &mut TileMatrix, bank_settings: BankSettings) {
    thread_rng();
    let max = Bank(0..0).properties().max();
    let spawn_points = spawn_content_randomly(world, bank_settings.number_of_spawn_points, Bank(0..0));

    for c in spawn_points {
        let upper_bound = thread_rng().gen_range(2..=max);
        world[c.row][c.col].content = Bank(1..upper_bound);
    }
}
