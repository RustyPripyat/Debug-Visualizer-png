use std::collections::HashMap;

use chrono::Utc;
use debug_print::debug_println;
use noise::{Fbm, Perlin, RidgedMulti};
use noise::MultiFractal;
use noise::NoiseFn;
use rayon::iter::*;
use rayon::iter::IntoParallelIterator;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType::{Foggy, Rainy, Sunny};
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::world_generator::Generator;

use crate::content::bank::{BankSettings, spawn_bank};
use crate::content::bin::{BinSettings, spawn_bin};
use crate::content::garbage::{GarbageSettings, spawn_garbage};
use crate::content::wood_crate::{CrateSettings, spawn_crate};
use crate::tiletype::lava::{LavaSettings, spawn_lava};
use crate::tiletype::street::street_spawn;
use crate::utils::{find_max_value, find_min_value, percentage};

impl Default for NoiseSettings {
    fn default() -> Self {
        NoiseSettings {
            seed: 0,
            octaves: 12,
            frequency: 2.5,
            lacunarity: 2.0,
            persistence: 1.25,
            attenuation: 2.5,
            scale: 0.25,
        }
    }
}

pub struct NoiseSettings {
    pub(crate) seed: u32,
    pub(crate) octaves: usize,
    pub(crate) frequency: f64,
    pub(crate) lacunarity: f64,
    pub(crate) persistence: f64,
    pub(crate) attenuation: f64,
    pub(crate) scale: f64,
}

impl Default for Thresholds {
    fn default() -> Self {
        Thresholds {
            threshold_deep_water: 4.0,
            threshold_shallow_water: 10.0,
            threshold_sand: 15.0,
            threshold_grass: 45.0,
            threshold_hill: 65.0,
            threshold_mountain: 77.5,
        }
    }
}

pub struct Thresholds {
    pub(crate) threshold_deep_water: f64,
    pub(crate) threshold_shallow_water: f64,
    pub(crate) threshold_sand: f64,
    pub(crate) threshold_grass: f64,
    pub(crate) threshold_hill: f64,
    pub(crate) threshold_mountain: f64,
}

pub struct WorldGenerator {
    pub size: usize,
    pub noise_settings: NoiseSettings,
    pub thresholds: Thresholds,
    pub lava_settings: LavaSettings,
    pub bank_settings: BankSettings,
    pub bin_settings: BinSettings,
    pub crate_settings: CrateSettings,
    pub garbage_settings: GarbageSettings,
}

impl WorldGenerator {
    fn generate_terrain(&self, noise_map: &[Vec<f64>], min: f64, max: f64) -> Vec<Vec<Tile>> {
        let mut world = vec![vec![Tile { tile_type: TileType::Grass, content: Content::None, elevation: 0 }; self.size]; self.size];

        for (y, row) in noise_map.iter().enumerate() {
            for (x, &value) in row.iter().enumerate() {
                let tile_type = match value {
                    | v if v < percentage(self.thresholds.threshold_deep_water, min, max) => TileType::DeepWater,
                    | v if v < percentage(self.thresholds.threshold_shallow_water, min, max) => TileType::ShallowWater,
                    | v if v < percentage(self.thresholds.threshold_sand, min, max) => TileType::Sand,
                    | v if v < percentage(self.thresholds.threshold_grass, min, max) => TileType::Grass,
                    | v if v < percentage(self.thresholds.threshold_hill, min, max) => TileType::Hill,
                    | v if v < percentage(self.thresholds.threshold_mountain, min, max) => TileType::Mountain,
                    | _ => TileType::Snow,
                };

                //add Default Water Content on DeepWater and ShallowWater
                // content = add_default_water_content(tile_type);

                world[y][x] = Tile {
                    tile_type,
                    content: Content::None,
                    elevation: 0,
                };
            }
        }
        //color local maxima black
        let polygons = street_spawn(self.size / 250, noise_map, 10, 0.0);

        for polygon in polygons.iter() {
            for (y, x) in polygon {
                debug_println!("Street in: {};{}", x, y);
                world[*y][*x].tile_type = TileType::Street;
            }
        }
        world
    }

    fn generate_elevation_map(&self) -> Vec<Vec<f64>> {
        let noise = RidgedMulti::<Fbm<Perlin>>::new(self.noise_settings.seed)
            .set_octaves(self.noise_settings.octaves)
            .set_frequency(self.noise_settings.frequency)
            .set_lacunarity(self.noise_settings.lacunarity)
            .set_persistence(self.noise_settings.persistence)
            .set_attenuation(self.noise_settings.attenuation);

        (0..self.size)
            .into_par_iter()
            .map(|y| {
                let y_normalized = y as f64 / self.size as f64;
                (0..self.size)
                    .map(|x| {
                        let x_normalized = x as f64 / self.size as f64;
                        noise.get([x_normalized, y_normalized])
                    })
                    .collect()
            })
            .collect()
    }

    pub fn new(
        size: usize,
        noise_settings: NoiseSettings,
        thresholds: Thresholds,
        lava_settings: LavaSettings,
        bank_settings: BankSettings,
        bin_settings: BinSettings,
        crate_settings: CrateSettings,
        garbage_settings: GarbageSettings,
    ) -> Self {
        Self {
            size,
            noise_settings,
            thresholds,
            lava_settings,
            bank_settings,
            bin_settings,
            crate_settings,
            garbage_settings,
        }
    }
}

impl Generator for WorldGenerator {
    fn gen(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions, f32, Option<HashMap<Content, f32>>) {
        let noise_map = self.generate_elevation_map();
        let mut start = Utc::now();
        debug_println!("Done: Generate noise map: {}", (Utc::now() - start).num_milliseconds());

        debug_println!("Start: Calculate min and max value");
        start = Utc::now();
        let min_value = find_min_value(&noise_map).unwrap_or(f64::MAX); // get min value
        let max_value = find_max_value(&noise_map).unwrap_or(f64::MIN);
        // get max value
        debug_println!("Done: Calculate min and max value: {}", (Utc::now() - start).num_milliseconds());

        debug_println!("Start: Generate terrain");
        start = Utc::now();
        let mut world = self.generate_terrain(&noise_map, min_value, max_value);
        debug_println!("Done: Generate terrain: {}", (Utc::now() - start).num_milliseconds());

        // spawn lava
        debug_println!("Start: Spawn lava");
        start = Utc::now();
        spawn_lava(&mut world, &noise_map, self.lava_settings.clone());
        debug_println!("Done: Spawn lava: {}", (Utc::now() - start).num_milliseconds());

        // spawn bank
        debug_println!("Start: Spawn bank");
        spawn_bank(&mut world, self.bank_settings.clone());

        // spawn bin
        debug_println!("Start: Spawn bin");
        spawn_bin(&mut world, self.bin_settings.clone());

        // spawn wood_crate
        debug_println!("Start: Spawn crate");
        spawn_crate(&mut world, self.crate_settings.clone());

        // spawn garbage
        debug_println!("Start: Spawn garbage");
        start = Utc::now();
        spawn_garbage(&mut world, &self.garbage_settings);
        debug_println!("Done: Spawn garbage in {}ms", (Utc::now() - start).num_milliseconds());

        (world, (0, 0), EnvironmentalConditions::new(&[Rainy, Sunny, Foggy], 1, 9).unwrap(), 0.0, None)
    }
}
