use rand::{thread_rng, Rng};
use robotics_lib::world::tile::Content::Market;
use serde::{Deserialize, Serialize};

use crate::generator::TileMatrix;
use crate::utils::spawn_content_randomly;

/// Settings defining the behavior of market spawn,
/// such as the number of spawn points
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct MarketSettings {
    /// the number of markets to spawn
    pub number_of_spawn_points: usize,
}

impl MarketSettings {
    /// Custom version of default that provides an instance of `MarketSettings` with the
    /// optimal parameters for the given world size
    pub fn default(size: usize) -> Self {
        MarketSettings {
            number_of_spawn_points: usize::pow(size, 2) / 100,
        }
    }

    /// Creates a new instance of `MarketSettings` with the given number of spawn points.
    ///
    /// # Arguments
    ///
    /// * `number_of_spawn_points` - The number of markets to spawn within the world.
    ///
    /// # Returns
    ///
    /// A new `MarketSettings` instance with the specified number of spawn points.
    ///
    /// # Examples
    ///
    /// ```
    /// use exclusion_zone::content::market::MarketSettings;
    ///
    /// let settings = MarketSettings::new(10);
    /// ```
    pub fn new(number_of_spawn_points: usize) -> Self {
        MarketSettings {
            number_of_spawn_points,
        }
    }
}

pub(crate) fn spawn_market(world: &mut TileMatrix, market_settings: MarketSettings) {
    thread_rng();
    let max = Market(0).properties().max();
    let spawn_points = spawn_content_randomly(world, market_settings.number_of_spawn_points, Market(0));

    for c in spawn_points {
        world[c.row][c.col].content = Market(thread_rng().gen_range(1..=max));
    }
}
