use std::collections::HashMap;
use std::ops::Range;

use noise::MultiFractal;
use noise::NoiseFn;
use noise::{Fbm, Perlin, RidgedMulti};
use rayon::iter::IntoParallelIterator;
use rayon::iter::*;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType;
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::worldgenerator::Generator;

use crate::content::garbage::{spawn_garbage, GarbageSettings};
use crate::tiletype::lava::spawn_lava;
use crate::utils::{find_max_value, find_min_value, percentage};

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
    pub(crate) garbage_settings: GarbageSettings,
}

impl WorldGenerator {
    fn generate_terrain(&self, noise_map: &[Vec<f64>], min: f64, max: f64) -> Vec<Vec<Tile>> {
        let mut world = vec![
            vec![
                Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                    elevation: 0,
                };
                self.size
            ];
            self.size
        ];

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

                world[y][x] = Tile {
                    tile_type,
                    content: Content::None,
                    elevation: 0,
                };
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
        garbage_settings: GarbageSettings,
    ) -> Self {
        Self {
            size,
            noise_settings,
            thresholds,
            lava_settings,
            garbage_settings,
        }
    }
}

impl Generator for WorldGenerator {
    fn gen(
        &mut self,
    ) -> (
        Vec<Vec<Tile>>,
        (usize, usize),
        EnvironmentalConditions,
        f32,
        Option<HashMap<Content, f32>>,
    ) {
        let noise_map = self.generate_elevation_map();
        let min_value = find_min_value(&noise_map).unwrap_or(f64::MIN);
        let max_value = find_max_value(&noise_map).unwrap_or(f64::MAX);
        let mut world = self.generate_terrain(&noise_map, min_value, max_value);
        spawn_lava(&mut world, &noise_map, self.lava_settings.clone());
        spawn_garbage(&mut world, &self.garbage_settings);

        (
            world,
            (0, 0),
            EnvironmentalConditions::new(&[WeatherType::Rainy, WeatherType::Sunny, WeatherType::Foggy], 1, 9).unwrap(),
            0.0,
            None,
        )
    }
}
