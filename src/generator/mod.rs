use std::ops::Range;
// use std::os::linux::raw::stat;
use chrono::Utc;
use noise::{Fbm, Perlin, RidgedMulti};
use rayon::iter::IntoParallelIterator;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType::{Rainy, Sunny};
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::worldgenerator::Generator;
use crate::utils::{find_max_value, find_min_value, percentage};
use noise::MultiFractal;
use noise::NoiseFn;
use crate::content::water::add_default_water_content;
use rayon::iter::*;
use crate::tiletype::lava::{spawn_lava};
use crate::tiletype::street::street_spawn;


pub(crate) struct NoiseSettings {
    pub(crate) seed: u32,
    pub(crate) octaves: usize,
    pub(crate) frequency: f64,
    pub(crate) lacunarity: f64,
    pub(crate) persistence: f64,
    pub(crate) attenuation: f64,
    pub(crate) scale: f64,
}
#[derive(Clone)]
pub(crate) struct LavaSettings {
    pub(crate) number_of_spawn_points: usize,
    pub(crate) lava_flow_range: Range<usize>,
}

pub(crate) struct Thresholds {
    pub(crate) threshold_deep_water: f64,
    pub(crate) threshold_shallow_water: f64,
    pub(crate) threshold_sand: f64,
    pub(crate) threshold_grass: f64,
    pub(crate) threshold_hill: f64,
    pub(crate) threshold_mountain: f64,
}

pub(crate) struct WorldGenerator {
    pub(crate) size: usize,
    pub(crate) noise_settings: NoiseSettings,
    pub(crate) thresholds: Thresholds,
    pub(crate) lava_settings: LavaSettings,
}

impl WorldGenerator {
    fn generate_terrain(&self, noise_map: & Vec<Vec<f64>>, min: f64, max: f64) -> Vec<Vec<Tile>> {
        let mut world = vec![vec![Tile {
            tile_type: TileType::Grass,
            content: Content::None,
            elevation: 0,
        }; self.size]; self.size];

        for (y, row) in noise_map.iter().enumerate() {
            for (x, &value) in row.iter().enumerate() {
                let elevation = 0;
                let mut content = Content::None;
                let mut tile_type = match value {
                    v if v < percentage(self.thresholds.threshold_deep_water, min, max) => TileType::DeepWater,
                    v if v < percentage(self.thresholds.threshold_shallow_water, min, max) => TileType::ShallowWater,
                    v if v < percentage(self.thresholds.threshold_sand, min, max) => TileType::Sand,
                    v if v < percentage(self.thresholds.threshold_grass, min, max) => TileType::Grass,
                    v if v < percentage(self.thresholds.threshold_hill, min, max) => TileType::Hill,
                    v if v < percentage(self.thresholds.threshold_mountain, min, max) => TileType::Mountain,
                    _ => TileType::Snow,
                };

                //add Default Water Content on DeepWater and ShallowWater
                content = add_default_water_content(tile_type);

                world[y][x] = Tile {
                    tile_type,
                    content,
                    elevation,
                };
            }
        }
        //color local maxima black
        let local_maxima = street_spawn(self.size/250, noise_map, 10, 0.0);
        for (y, x) in local_maxima {
            println!("Street in: {};{}", x, y);
            world[y][x].tile_type = TileType::Street;
        }
        world
    }

    fn generate_elevation_map(&self) -> Vec<Vec<f64>> {
        let noise = RidgedMulti::<Fbm<Perlin>>::new(self.noise_settings.seed).set_octaves(self.noise_settings.octaves).set_frequency(self.noise_settings.frequency).set_lacunarity(self.noise_settings.lacunarity).set_persistence(self.noise_settings.persistence).set_attenuation(self.noise_settings.attenuation);

        (0..self.size).into_par_iter().map(|y| {
            let y_normalized = y as f64 / self.size as f64;
            (0..self.size).map(|x| {
                let x_normalized = x as f64 / self.size as f64;
                noise.get([x_normalized, y_normalized])
            }).collect()
        }).collect()
    }

    pub fn new(size: usize, noise_settings: NoiseSettings, thresholds: Thresholds, lava_settings: LavaSettings) -> Self
    {
        Self { size, noise_settings, thresholds, lava_settings }
    }
}


impl Generator for WorldGenerator {
    fn gen(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions, f32) {

        println!("Start: Generate noise map");
        let mut start = Utc::now();
        let noise_map = self.generate_elevation_map();
        println!("Done: Generate noise map: {}", (Utc::now() - start).num_milliseconds());


        println!("Start: Calculate min and max value");
        start = Utc::now();
        let min_value = find_min_value(&noise_map).unwrap_or(f64::MAX);     // get min value
        let max_value = find_max_value(&noise_map).unwrap_or(f64::MIN);     // get max value
        println!("Done: Calculate min and max value: {}", (Utc::now() - start).num_milliseconds());

        println!("Start: Generate terrain");
        start = Utc::now();
        let mut world = self.generate_terrain(&noise_map, min_value, max_value);
        println!("Done: Generate terrain: {}", (Utc::now() - start).num_milliseconds());

        // spawn lava
        println!("Start: Spawn lava");
        start = Utc::now();
        spawn_lava(&mut world, &noise_map, self.lava_settings.clone());
        println!("Done: Spawn lava: {}", (Utc::now() - start).num_milliseconds());

        // Return the generated world, dimensions, and environmental conditions
        (world, (0, 0), EnvironmentalConditions::new(&[Sunny, Rainy], 15, 12), 0.0)
    }
}