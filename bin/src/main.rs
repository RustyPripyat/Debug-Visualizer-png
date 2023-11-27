use noise::{NoiseFn, Perlin, RidgedMulti, Fbm};
use noise::MultiFractal;
use noise::Seedable;
use rand::Rng;
use rayon::iter::*;
use strum::IntoEnumIterator;

use robotics_lib::energy::Energy;
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType::{Rainy, Sunny};
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::World;
use robotics_lib::world::worldgenerator::Generator;

use crate::visualizer::save_world_image;

pub mod visualizer;

fn main() {
    struct MyRobot(Robot);
    struct WorldGenerator {
        size: usize,
        seed: u32,
        octaves: usize,
        frequency: f64,
        lacunarity: f64,
        persistence: f64,
        attenuation: f64,
        scale: f64,
    }

    impl WorldGenerator {
        fn init(size: usize) -> Self {
            WorldGenerator { size, seed: 0, octaves: 12, frequency: 2.0, lacunarity: 2.0, persistence: 1.25, attenuation: 1.5, scale: 1.0 }
        }


        fn generate_terrain(&self, noise_map: Vec<Vec<f64>>, min: f64, max: f64 ) -> Vec<Vec<Tile>> {
            // let height = self.size;
            // let width = if height > 0 { noise_map[0].len() } else { 0 };
            let mut world = vec![vec![Tile {
                tile_type: TileType::Grass,
                content: Content::None,
            }; self.size]; self.size];

            for (y, row) in noise_map.iter().enumerate() {
                for (x, &value) in row.iter().enumerate() {
                    let tile_type = match value {
                        v if v < percentage(4.0,min, max) => TileType::DeepWater,
                        v if v < percentage(10.0, min, max) => TileType::ShallowWater,
                        v if v < percentage(15.0,min,max) => TileType::Sand,
                        v if v < percentage(45.0,min,max) => TileType::Grass,
                        v if v < percentage(65.0,min,max) => TileType::Hill,
                        v if v < percentage(77.5,min,max) => TileType::Mountain,
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



        fn generate_elevation_map(&self) -> Vec<Vec<f64>> {
            let noise = RidgedMulti::<Fbm<Perlin>>::new(self.seed).set_octaves(self.octaves).set_frequency(self.frequency).set_lacunarity(self.lacunarity).set_persistence(self.persistence).set_attenuation(self.attenuation);

            (0..self.size).map(|y| {
                (0..self.size).into_par_iter().map(|x| {
                    let x_normalized = x as f64 / self.size as f64;
                    let y_normalized = y as f64 / self.size as f64;
                    noise.get([x_normalized, y_normalized, 0.0])
                }).collect()
            }).collect()
        }
        pub fn new(size: usize, seed: u32, octaves: usize, frequency: f64, lacunarity: f64, persistence: f64, attenuation: f64, scale: f64) -> Self {
            Self { size, seed, octaves, frequency, lacunarity, persistence, attenuation, scale }
        }
    }

    fn find_min_value(matrix: &Vec<Vec<f64>>) -> Option<f64> {
        // Ensure the matrix is not empty
        if matrix.is_empty() || matrix[0].is_empty() {
            return None;
        }

        let mut min_value = matrix[0][0];

        for row in matrix {
            for &value in row {
                if value < min_value {
                    min_value = value;
                }
            }
        }

        Some(min_value)
    }

    fn find_max_value(matix: &Vec<Vec<f64>>) -> Option<f64> {
        // Ensure the matrix is not empty
        if matix.is_empty() || matix[0].is_empty() {
            return None;
        }

        let mut max_value = matix[0][0];

        for row in matix {
            for &value in row {
                if value > max_value {
                    max_value = value;
                }
            }
        }

        Some(max_value)
    }

    fn map_value_to_range(value: f64, from: std::ops::Range<f64>, to: std::ops::Range<f64>) -> f64 {
        let from_min = from.start;
        let from_max = from.end;
        let to_min = to.start;
        let to_max = to.end;

        (value - from_min) * (to_max - to_min) / (from_max - from_min) + to_min
    }

    impl Generator for WorldGenerator {
        fn gen(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions) {
            let mut noise_map = self.generate_elevation_map();

            // get min and max values
            let min_value = find_min_value(&noise_map).unwrap_or(f64::MAX);
            // get max value
            let max_value = find_max_value(&noise_map).unwrap_or(f64::MIN);

            //println!("min value: {}", min_value);
            //println!("max value: {}", max_value);

            // map from min_value and max_value to [-1,1]
            // noise_map = (0..self.size).map(|y| {
            //     (0..self.size).into_par_iter().map(|x| {
            //         map_value_to_range(noise_map[x][y], min_value..max_value, -1.0..1.0)
            //     }).collect()
            // }).collect();

            let world = self.generate_terrain(noise_map, min_value, max_value);
            // Return the generated world, dimensions, and environmental conditions
            (world, (0, 0), EnvironmentalConditions::new(&[Sunny, Rainy], 15, 12))
        }
    }

    impl Runnable for MyRobot {
        fn process_tick(&mut self, _world: &mut World) {
            // for _ in 0..1 {
            //     let (tmp, _a, _b) = debug(self, world);
            //     let environmental_conditions = look_at_sky(self, world);
            //     println!(
            //         "Daytime: {:?}, Time:{:?}, Weather: {:?}\n",
            //         environmental_conditions.get_time_of_day(),
            //         environmental_conditions.get_time_of_day_string(),
            //         environmental_conditions.get_weather_condition()
            //     );
            //     for elem in tmp.iter() {
            //         for tile in elem.iter() {
            //             match tile.tile_type {
            //                 | DeepWater => {
            //                     print!("DW");
            //                 }
            //                 | ShallowWater => {
            //                     print!("SW");
            //                 }
            //                 | Sand => {
            //                     print!("Sa");
            //                 }
            //                 | Grass => {
            //                     print!("Gr");
            //                 }
            //                 | Street => {
            //                     print!("St");
            //                 }
            //                 | Hill => {
            //                     print!("Hi");
            //                 }
            //                 | Mountain => {
            //                     print!("Mt");
            //                 }
            //                 | Snow => {
            //                     print!("Sn");
            //                 }
            //                 | Lava => {
            //                     print!("La");
            //                 }
            //             }
            //             match &tile.content {
            //                 | Rock(quantity) => print!("->Ro {}", quantity),
            //                 | Tree(quantity) => print!("->Tr {}", quantity),
            //                 | Garbage(quantity) => print!("->Gr {}", quantity),
            //                 | Fire => print!("->Fi -"),
            //                 | Coin(quantity) => print!("->Co {}", quantity),
            //                 | Bin(range) => print!("->Bi {}-{}", range.start, range.end),
            //                 | Crate(range) => print!("->Cr {}-{}", range.start, range.end),
            //                 | Bank(range) => print!("->Ba {}-{}", range.start, range.end),
            //                 | Water(quantity) => print!("->Wa {}", quantity),
            //                 | Content::None => print!("->No -"),
            //             }
            //             print!("\t| ");
            //         }
            //         println!();
            //     }
            // }
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

    let _r = MyRobot(Robot::new());

    //genereate wolrd with 0.25 diff for each parameter
    // (4..12).into_par_iter().for_each(|octave| {
    //     for frequency in 6..=10 { // 5 salti in padella
    //         let frequency = f64::from(frequency) * 0.25;
    //         for lacunarity in 6..=8 {
    //             let lacunarity = f64::from(lacunarity) * 0.25;
    //             for persistence in 2..=6 {
    //                 let persistence = f64::from(persistence) * 0.25;
    //                 for attenuation in 6..=8 {
    //                     let attenuation = f64::from(attenuation) * 0.25;
    //                         let mut generator = WorldGenerator::new(500, 0, octave, frequency, lacunarity, persistence, attenuation, 0.25);
    //                         let tiles = generator.gen().0;
    //
    //                         save_world_image(&tiles, (0, 0), format!("img/o{}-f{}-l{}-p{}-a{}.png", octave, frequency, lacunarity, persistence, attenuation).as_str());
    //                 }
    //             }
    //         }
    //     }
    // });

    let mut generator = WorldGenerator::new(500, 20, 12, 2.5, 2.0, 1.25, 2.5, 0.25);
    let tiles = generator.gen().0;
    save_world_image(&tiles, (0, 0), format!("o{}-f{}-l{}-p{}-a{}.png", generator.octaves, generator.frequency, generator.lacunarity, generator.persistence, generator.attenuation).as_str());
}


pub fn percentage(target_percentage: f64, min: f64, max: f64) -> f64 {
    // MappedValue= [(x-a)/(b-a)]⋅(d−c)+c
    let x = target_percentage;
    // let a = 0.0;
    let b = 100.0;
    let c = min;
    let d = max;
    // ((x - a) / (b - a)) * (d - c) + c
    ((x/b) * (d - c) + c) //simplified a = 0
}