use rand::Rng;
use strum::IntoEnumIterator;

use crate::energy::Energy;
use crate::world::environmental_conditions::WeatherType::{Rainy, Sunny};
use crate::{
    runner::{Robot, Runnable},
    world::{
        environmental_conditions::EnvironmentalConditions,
        tile::{Content::*, Tile, TileType::*},
        worldgenerator::Generator,
    },
};

struct TestRobot(Robot);

struct TestWorld {
    size: usize,
}

impl TestWorld {
    fn init(size: usize) -> Self {
        TestWorld { size }
    }
}

impl Generator for TestWorld {
    fn gen(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions) {
        let _rng = rand::thread_rng();
        let mut map: Vec<Vec<Tile>> = Vec::new();
        // Initialize the map with default tiles
        for _ in 0..self.size {
            let mut row: Vec<Tile> = Vec::new();
            for _ in 0..self.size {
                row.push(Tile {
                    tile_type: Sand,
                    content: Rock(1),
                });
            }
            map.push(row);
        }
        let environmental_conditions = EnvironmentalConditions::new(&[Sunny, Rainy], 15, 12);
        (map, (0, 0), environmental_conditions)
    }
}

impl Runnable for TestRobot {
    fn process_tick(&mut self, _world: &mut crate::world::World) {
        println!("I am just the TestRobot, I won't change the world 🥲.")
    }

    fn get_energy(&self) -> &crate::energy::Energy {
        &self.0.energy
    }

    fn get_energy_mut(&mut self) -> &mut crate::energy::Energy {
        &mut self.0.energy
    }

    fn get_coordinate(&self) -> &crate::world::coordinates::Coordinate {
        &self.0.coordinate
    }

    fn get_coordinate_mut(&mut self) -> &mut crate::world::coordinates::Coordinate {
        &mut self.0.coordinate
    }

    fn get_backpack(&self) -> &crate::runner::backpack::BackPack {
        &self.0.backpack
    }

    fn get_backpack_mut(&mut self) -> &mut crate::runner::backpack::BackPack {
        &mut self.0.backpack
    }
}

// This function always returns Ok(()) maybe needs some additional implementation
#[test]
fn robot_unit_test() {
    use super::*;

    let mut robot = TestRobot(Robot::new());
    let mut dummy_world = TestWorld::init(10);

    assert_eq!(run(&mut robot, &mut dummy_world), Ok(()));
}
