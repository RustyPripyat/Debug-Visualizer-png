use rand::{thread_rng, Rng};
use robotics_lib::world::tile::Content::Coin;
use serde::{Deserialize, Serialize};

use crate::generator::TileMatrix;
use crate::utils::spawn_content_randomly;

/// Settings defining the behavior of coins spawn,
/// such as the number of spawn points
#[derive(Serialize, Deserialize, Clone)]
pub struct CoinSettings {
    pub number_of_spawn_points: usize,
}

impl CoinSettings {
    /// Custom version of default that provides an instance of `CoinSettings` with the
    /// optimal parameters for the given world size
    pub fn default(size: usize) -> Self {
        CoinSettings {
            number_of_spawn_points: size * size / 25,
        }
    }

    /// Creates a new instance of `CoinSettings` with the given number of spawn points.
    ///
    /// # Arguments
    ///
    /// * `spawn_points` - The number of spawn points for coins within the world.
    ///
    /// # Returns
    ///
    /// A new `CoinSettings` instance with the specified number of spawn points.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use exclusion_zone::content::coin::CoinSettings;
    /// let settings = CoinSettings::new(5);
    /// ```
    pub fn new(spawn_points: usize) -> Self {
        CoinSettings {
            number_of_spawn_points: spawn_points,
        }
    }
}

pub(crate) fn spawn_coin(world: &mut TileMatrix, coin_settings: CoinSettings) {
    let max = Coin(0).properties().max();
    let spawn_points = spawn_content_randomly(world, coin_settings.number_of_spawn_points, Coin(0));

    for c in spawn_points {
        let random = thread_rng().gen_range(1..=max);
        world[c.row][c.col].content = Coin(random);
    }
}
