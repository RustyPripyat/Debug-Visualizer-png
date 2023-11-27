use image::{Rgb, RgbImage};
use noise::{NoiseFn, Perlin};
use rand::{Rng, RngCore};
use robotics_lib::energy::Energy;
use robotics_lib::interface::{debug, destroy, look_at_sky};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{run, Robot, Runnable};
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType::{Rainy, Sunny};
use robotics_lib::world::tile::Content::{Bank, Bin, Coin, Crate, Fire, Garbage, Rock, Tree, Water};
use robotics_lib::world::tile::TileType::{DeepWater, Grass, Hill, Lava, Mountain, Sand, ShallowWater, Snow, Street};
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::worldgenerator::Generator;
use robotics_lib::world::World;
use strum::IntoEnumIterator;
use robotics_lib::visualizer::{save_world_image};
use std::collections::HashMap;


fn main() {
    struct MyRobot(Robot);
    struct WorldGenerator {
        size: usize,
    }

    impl WorldGenerator {
        fn init(size: usize) -> Self {
            WorldGenerator { size }
        }


        fn generate_rounded_terrain(&self, noise_map: Vec<Vec<f64>>) -> Vec<Vec<Tile>> {
            let height = noise_map.len();
            let width = if height > 0 { noise_map[0].len() } else { 0 };

            let mut world = vec![vec![Tile {
                tile_type: TileType::Grass,
                content: Content::None,
            }; width]; height];

            for (y, row) in noise_map.iter().enumerate() {
                for (x, &value) in row.iter().enumerate() {
                    let mut tile_type = match value {
                        v if v < -0.1 => TileType::DeepWater,
                        v if v < 0.1 => TileType::ShallowWater,
                        v if v < 0.3 => TileType::Sand,
                        v if v < 0.6 => TileType::Grass,
                        v if v < 0.8 => TileType::Hill,
                        v if v < 0.9 => TileType::Mountain,
                        _ => TileType::Snow,
                    };

                    world[y][x] = Tile {
                        tile_type,
                        content: Content::None,
                    };
                }
            }

            world
        }


        fn generate_perlin_noise(&self, width: usize, height: usize, scale: f64) -> Vec<Vec<f64>> {
            let perlin = Perlin::new(rand::thread_rng().next_u32());

            (0..height)
                .map(|y| {
                    (0..width)
                        .map(|x| {
                            let x_normalized = x as f64 / (width as f64 * scale);
                            let y_normalized = y as f64 / (height as f64 * scale);
                            perlin.get([x_normalized, y_normalized, 0.0])
                        })
                        .collect()
                })
                .collect()
        }
    }

    impl Generator for WorldGenerator {
        fn gen(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions) {
            let width = 100; // Set your desired width
            let height = 100; // Set your desired height

            let noise_map = self.generate_perlin_noise(width, height, 0.42f64);
            let world = self.generate_rounded_terrain(noise_map);
            // Return the generated world, dimensions, and environmental conditions
            (world, (width, height), EnvironmentalConditions::new(&vec![Sunny, Rainy], 15, 12))
        }
    }

    // impl WorldGenerator {
    //     fn init(size: usize) -> Self {
    //         WorldGenerator { size }
    //     }
    // }
    // impl Generator for WorldGenerator {
    //     fn gen(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions) {
    //         let mut rng = rand::thread_rng();
    //         let mut map: Vec<Vec<Tile>> = Vec::new();
    //         // Initialize the map with default tiles
    //         for _ in 0..self.size {
    //             let mut row: Vec<Tile> = Vec::new();
    //             for _ in 0..self.size {
    //                 let i_tiletype = rng.gen_range(0..TileType::iter().len());
    //                 let i_content = rng.gen_range(0..Content::iter().len());
    //                 let tile_type = match i_tiletype {
    //                     | 0 => DeepWater,
    //                     | 1 => ShallowWater,
    //                     | 2 => Sand,
    //                     | 3 => Grass,
    //                     | 4 => Street,
    //                     | 5 => Hill,
    //                     | 6 => Mountain,
    //                     | 7 => Snow,
    //                     | 8 => Lava,
    //                     | _ => Grass,
    //                 };
    //                 let content = match i_content {
    //                     | 0 => Rock(0),
    //                     | 1 => Tree(2),
    //                     | 2 => Garbage(2),
    //                     | 3 => Fire,
    //                     | 4 => Coin(2),
    //                     | 5 => Bin(2..3),
    //                     | 6 => Crate(2..3),
    //                     | 7 => Bank(3..54),
    //                     | 8 => Content::Water(20),
    //                     | 9 => Content::None,
    //                     | _ => Content::None,
    //                 };
    //                 row.push(Tile { tile_type, content });
    //             }
    //             map.push(row);
    //         }
    //         let environmental_conditions = EnvironmentalConditions::new(&vec![Sunny, Rainy], 15, 12);
    //         (map, (0, 0), environmental_conditions)
    //     }
    // }

    impl Runnable for MyRobot {
        fn process_tick(&mut self, world: &mut World) {
            for _ in 0..1 {
                let (tmp, a, b) = debug(self, world);
                let environmental_conditions = look_at_sky(self, world);
                println!(
                    "Daytime: {:?}, Time:{:?}, Weather: {:?}\n",
                    environmental_conditions.get_time_of_day(),
                    environmental_conditions.get_time_of_day_string(),
                    environmental_conditions.get_weather_condition()
                );
                for elem in tmp.iter() {
                    for tile in elem.iter() {
                        match tile.tile_type {
                            | DeepWater => {
                                print!("DW");
                            }
                            | ShallowWater => {
                                print!("SW");
                            }
                            | Sand => {
                                print!("Sa");
                            }
                            | Grass => {
                                print!("Gr");
                            }
                            | Street => {
                                print!("St");
                            }
                            | Hill => {
                                print!("Hi");
                            }
                            | Mountain => {
                                print!("Mt");
                            }
                            | Snow => {
                                print!("Sn");
                            }
                            | Lava => {
                                print!("La");
                            }
                        }
                        match &tile.content {
                            | Rock(quantity) => print!("->Ro {}", quantity),
                            | Tree(quantity) => print!("->Tr {}", quantity),
                            | Garbage(quantity) => print!("->Gr {}", quantity),
                            | Fire => print!("->Fi -"),
                            | Coin(quantity) => print!("->Co {}", quantity),
                            | Bin(range) => print!("->Bi {}-{}", range.start, range.end),
                            | Crate(range) => print!("->Cr {}-{}", range.start, range.end),
                            | Bank(range) => print!("->Ba {}-{}", range.start, range.end),
                            | Water(quantity) => print!("->Wa {}", quantity),
                            | Content::None => print!("->No -"),
                        }
                        print!("\t| ");
                    }
                    println!();
                }
                // println!("{:?}, {:?}", a, b);
                // match ris {
                //     | Ok(values) => println!("Ok"),
                //     | Err(e) => println!("{:?}", e),
                // }
            }
            // println!(
            //     "Destroy HERE {:?}",
            //     destroy(self, world, robotics_lib::interface::Direction::Down)
            // );
        }

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

    let mut r = MyRobot(Robot::new());
    let mut generator = WorldGenerator::init(10);
    save_world_image(&generator.gen().0, (0, 0), "test.png");
    // println!("Last print: {:?}", run(&mut r, &mut generator));
}
