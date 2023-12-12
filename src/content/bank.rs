use rand::{Rng, thread_rng};
use robotics_lib::world::tile::Content::Bank;

use crate::generator::TileMatrix;
use crate::utils::spawn_content_randomly;

impl BankSettings {
    /// Custom version of default that provides an instance of `BankSettings` with the
    /// optimal parameters for the given world size
    pub fn default(size: usize) -> Self {
        BankSettings {
            number_of_spawn_points: size / 25,
        }
    }
}

/// Settings defining the behavior of bank spawn,
/// such as the number of spawn points
#[derive(Clone)]
pub struct BankSettings {
    pub(crate) number_of_spawn_points: usize,
}

pub(crate) fn spawn_bank(world: &mut TileMatrix, bank_settings: BankSettings) {
    thread_rng();
    let max = Bank(0..0).properties().max();
    let spawn_points = spawn_content_randomly(world, bank_settings.number_of_spawn_points, Bank(0..0));

    for (y, x) in spawn_points {
        let upper_bound = thread_rng().gen_range(2..=max);
        world[y][x].content = Bank(1..upper_bound);
    }
}
