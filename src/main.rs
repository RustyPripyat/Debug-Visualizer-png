use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::World;
use robotics_lib::world::worldgenerator::Generator;

use crate::generator::*;
use crate::visualizer::save_world_image;

pub mod visualizer;
mod content;
mod tiletype;
mod utils;
mod generator;

fn main() {
    struct MyRobot(Robot);

    impl Runnable for MyRobot {
        fn process_tick(&mut self, _world: &mut World) {
            // Do nothing
        }

        fn handle_event(&mut self, _: Event) { todo!() }
        fn get_energy(&self) -> &Energy {
            &self.0.energy
        }

        fn get_energy_mut(&mut self) -> &mut Energy {
            &mut self.0.energy
        }
        fn get_coordinate(&self) -> &Coordinate {
            &self.0.coordinate
        }

        fn get_coordinate_mut(&mut self) -> &mut Coordinate {
            &mut self.0.coordinate
        }
        fn get_backpack(&self) -> &BackPack {
            &self.0.backpack
        }
        fn get_backpack_mut(&mut self) -> &mut BackPack {
            &mut self.0.backpack
        }
    }

    let _r = MyRobot(Robot::new());
    let size = 1000;
    let mut generator = WorldGenerator::new(size, NoiseSettings {
        seed: 0,
        octaves: 12,
        frequency: 2.5,
        lacunarity: 2.0,
        persistence: 1.25,
        attenuation: 2.5,
        scale: 0.25,
    }, Thresholds {
        threshold_deep_water: 4.0,
        threshold_shallow_water: 10.0,
        threshold_sand: 15.0,
        threshold_grass: 45.0,
        threshold_hill: 65.0,
        threshold_mountain: 77.5,
    }, LavaSettings {
        number_of_spawn_points: size / 25,
        lava_flow_range: 1..size / 25,
    },
    );
    let tiles = generator.gen().0;
    save_world_image(&tiles, (0, 0), "img.png", 4);
}
