// spawn bank randomly in the world
use rand::{Rng, thread_rng};
use robotics_lib::world::tile::Tile;
use robotics_lib::world::tile::Content::Bank;
use crate::utils::spawn_content_randomly;

impl BankSettings {
    // Custom constructor that takes a size parameter
    pub(crate) fn default(size: usize) -> Self {
        BankSettings {
            number_of_spawn_points: size / 25,
        }
    }
}

#[derive(Clone)]
pub(crate) struct BankSettings {
    pub(crate) number_of_spawn_points: usize,
}

pub(crate) fn spawn_bank(world: &mut Vec<Vec<Tile>>, bank_settings: BankSettings) {

    let rng = thread_rng();
    let max = Bank(0..0).properties().max();
    let spawn_points= spawn_content_randomly(world, bank_settings.number_of_spawn_points, Bank(0..0));

    for (y, x) in spawn_points {
        let upper_bound = thread_rng().gen_range(2..=max);
        world[y][x].content = Bank(1 ..upper_bound);
        println!("spawned bank at {},{} with upper bound {}", x, y, upper_bound)
    }
}