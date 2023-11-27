#[cfg(test)]
mod backpack_test;
#[cfg(test)]
mod energy_tests;
#[cfg(test)]
mod runner_test;
#[cfg(test)]
mod utils_test;
#[cfg(test)]
mod world_tests;

use crate::interface::{debug, destroy, go, put, robot_view};

use crate::runner::{run, Robot, Runnable};

use crate::energy::Energy;
use crate::interface::Direction::{Down, Right};
use crate::runner::backpack::BackPack;
use crate::world::coordinates::Coordinate;
use crate::world::environmental_conditions::EnvironmentalConditions;
use crate::world::environmental_conditions::WeatherType::{Rainy, Sunny};
use crate::world::tile::Content::{Bank, Bin, Coin, Crate, Fire, Garbage, Rock, Tree, Water};
use crate::world::tile::TileType::{DeepWater, Grass, Hill, Lava, Mountain, Sand, ShallowWater, Snow, Street};
use crate::world::tile::{Content, Tile, TileType};
use crate::world::worldgenerator::Generator;
use crate::world::World;
use rand::Rng;
use strum::IntoEnumIterator;

// fn gen_world(dimension: usize) -> World {
//     let mut rng = rand::thread_rng();
//     let mut map: Vec<Vec<Tile>> = Vec::new();
//     // Initialize the map with default tiles
//     for _ in 0..dimension {
//         let mut row: Vec<Tile> = Vec::new();
//         for _ in 0..dimension {
//             let i_tiletype = rng.gen_range(0..TileType::iter().len());
//             let i_content = rng.gen_range(0..Content::iter().len());
//             let tile_type = match i_tiletype {
//                 | 0 => DeepWater,
//                 | 1 => ShallowWater,
//                 | 2 => Sand,
//                 | 3 => Grass,
//                 | 4 => Street,
//                 | 5 => Hill,
//                 | 6 => Mountain,
//                 | 7 => Snow,
//                 | 8 => Lava,
//                 | _ => Grass,
//             };
//             let content = match i_content {
//                 | 0 => Rock(0),
//                 | 1 => Tree(2),
//                 | 2 => Garbage(2),
//                 | 3 => Fire,
//                 | 4 => Coin(2),
//                 | 5 => Bin(0..2),
//                 | 6 => Crate(1..2),
//                 | 7 => Bank(3..54),
//                 | 8 => Content::None,
//                 | _ => Content::None,
//             };
//             row.push(Tile { tile_type, content });
//         }
//         map.push(row);
//     }
//     let environmental_conditions = EnvironmentalConditions::new(&[Sunny, Rainy], 15, 12);
//
//     World {
//         map,
//         dimension,
//         environmental_conditions,
//     }
// }

pub(crate) fn view_interface_test(robot: &impl Runnable, world: &World) {
    let tmp = robot_view(robot, world);

    for row in tmp.iter() {
        for elem in row.iter() {
            print!("{:?}", elem)
        }
        println!();
    }
}

#[test]
#[ignore]
pub(crate) fn testing() {
    struct MyRobot(Robot);
    struct WorldGenerator {
        size: usize,
    }
    impl WorldGenerator {
        fn init(size: usize) -> Self {
            WorldGenerator { size }
        }
    }
    impl Generator for WorldGenerator {
        fn gen(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions) {
            let mut rng = rand::thread_rng();
            let mut map: Vec<Vec<Tile>> = Vec::new();
            // Initialize the map with default tiles
            for _ in 0..self.size {
                let mut row: Vec<Tile> = Vec::new();
                for _ in 0..self.size {
                    let i_tiletype = rng.gen_range(0..TileType::iter().len());
                    let i_content = rng.gen_range(0..Content::iter().len());
                    let tile_type = match i_tiletype {
                        | 0 => DeepWater,
                        | 1 => ShallowWater,
                        | 2 => Sand,
                        | 3 => Grass,
                        | 4 => Street,
                        | 5 => Hill,
                        | 6 => Mountain,
                        | 7 => Snow,
                        | 8 => Lava,
                        | _ => Grass,
                    };
                    let content = match i_content {
                        | 0 => Rock(0),
                        | 1 => Tree(2),
                        | 2 => Garbage(2),
                        | 3 => Fire,
                        | 4 => Coin(2),
                        | 5 => Bin(2..3),
                        | 6 => Crate(2..3),
                        | 7 => Bank(3..54),
                        | 8 => Content::Water(20),
                        | 9 => Content::None,
                        | _ => Content::None,
                    };
                    row.push(Tile { tile_type, content });
                }
                map.push(row);
            }
            let environmental_conditions = EnvironmentalConditions::new(&[Sunny, Rainy], 15, 12);
            (map, (0, 0), environmental_conditions)
        }
    }
    impl Runnable for MyRobot {
        fn process_tick(&mut self, world: &mut World) {
            // other

            for _ in 0..1 {
                let (tmp, a, b) = debug(self, world);
                for e in tmp.iter() {
                    for f in e.iter() {
                        match f.tile_type {
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
                        match &f.content {
                            | Rock(quantity) => print!("->Ro {}", quantity),
                            | Tree(quantity) => print!("->Tr {}", quantity),
                            | Garbage(quantity) => print!("->Gr {}", quantity),
                            | Fire => print!("->Fi -"),
                            | Coin(quantity) => print!("->Co {}", quantity),
                            | Bin(range) => print!("->Bi {}", range.start),
                            | Crate(range) => print!("->Cr {}", range.start),
                            | Bank(range) => print!("->Ba {}", range.start),
                            | Content::Water(quantity) => print!("->Wa {}", quantity),
                            | Content::None => print!("->No -"),
                        }
                        print!("\t| ");
                    }
                    println!();
                }
                println!("{:?}, {:?}", a, b);
                // match ris {
                //     | Ok(values) => println!("Ok"),
                //     | Err(e) => println!("{:?}", e),
                // }
            }
            println!("HERE {:?}", destroy(self, world, crate::interface::Direction::Down))
        }

        fn get_energy_mut(&mut self) -> &mut Energy {
            &mut self.0.energy
        }
        fn get_energy(&self) -> &Energy {
            &self.0.energy
        }

        fn get_backpack(&self) -> &BackPack {
            &self.0.backpack
        }
        fn get_backpack_mut(&mut self) -> &mut BackPack {
            &mut self.0.backpack
        }
        fn get_coordinate(&self) -> &Coordinate {
            &self.0.coordinate
        }
        fn get_coordinate_mut(&mut self) -> &mut Coordinate {
            &mut self.0.coordinate
        }
    }

    let mut r = MyRobot(Robot::new());
    let mut generator = WorldGenerator::init(10);
    println!("{:?}", run(&mut r, &mut generator));
}

#[test]
pub fn test_issue24() {
    struct MyRobot(Robot);
    struct WorldGenerator;
    impl Generator for WorldGenerator {
        fn gen(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions) {
            let mut map: Vec<Vec<Tile>> = Vec::new();
            // Initialize the map with default tiles
            map.push(vec![
                Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                },
                Tile {
                    tile_type: TileType::Sand,
                    content: Content::None,
                },
                Tile {
                    tile_type: TileType::DeepWater,
                    content: Content::None,
                },
            ]);
            map.push(vec![
                Tile {
                    tile_type: TileType::ShallowWater,
                    content: Content::None,
                },
                Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                },
                Tile {
                    tile_type: TileType::Sand,
                    content: Content::Tree(32),
                },
            ]);
            map.push(vec![
                Tile {
                    tile_type: TileType::ShallowWater,
                    content: Content::None,
                },
                Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                },
                Tile {
                    tile_type: TileType::Street,
                    content: Content::None,
                },
            ]);
            let environmental_conditions = EnvironmentalConditions::new(&[Sunny, Rainy], 15, 12);
            (map, (1, 1), environmental_conditions)
        }
    }
    impl Runnable for MyRobot {
        fn process_tick(&mut self, world: &mut World) {
            // other
            for _ in 0..1 {
                let (tmp, a, b) = debug(self, world);
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
                println!("{:?}, {:?}", a, b);
            }
            println!("HERE {:?}", destroy(self, world, Right));
            let _ = put(self, world, Tree(0), 2, Down);
            let (tmp, _a, _b) = debug(self, world);
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
            let _ = go(self, world, Down);
            println!("{:?}", self.get_coordinate());
        }

        fn get_energy_mut(&mut self) -> &mut Energy {
            &mut self.0.energy
        }
        fn get_energy(&self) -> &Energy {
            &self.0.energy
        }

        fn get_backpack(&self) -> &BackPack {
            &self.0.backpack
        }
        fn get_backpack_mut(&mut self) -> &mut BackPack {
            &mut self.0.backpack
        }
        fn get_coordinate(&self) -> &Coordinate {
            &self.0.coordinate
        }
        fn get_coordinate_mut(&mut self) -> &mut Coordinate {
            &mut self.0.coordinate
        }
    }

    let mut r = MyRobot(Robot::new());
    let mut generator = WorldGenerator;
    println!("{:?}", run(&mut r, &mut generator));
}
