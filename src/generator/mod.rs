use std::collections::HashMap;

use chrono::Utc;
use debug_print::debug_println;
use noise::{Fbm, Perlin, RidgedMulti};
use noise::MultiFractal;
use noise::NoiseFn;
use rand::{RngCore, thread_rng};
use rayon::iter::*;
use rayon::iter::IntoParallelIterator;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType::{Foggy, Rainy, Sunny, TrentinoSnow, TropicalMonsoon};
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::world_generator::Generator;

use crate::content::bank::{BankSettings, spawn_bank};
use crate::content::bin::{BinSettings, spawn_bin};
use crate::content::fire::{FireSettings, spawn_fires};
use crate::content::garbage::{GarbageSettings, spawn_garbage};
use crate::content::wood_crate::{CrateSettings, spawn_crate};
use crate::tile_type::lava::{LavaSettings, spawn_lava};
use crate::tile_type::street::street_spawn;
use crate::utils::{find_max_value, find_min_value, percentage};

impl NoiseSettings {
    /// Provides an instance of `NoiseSettings` with the default parameter and give seed
    pub fn from_seed(seed: u32) -> Self {
        Self {
            seed,
            octaves: 12,
            frequency: 2.5,
            lacunarity: 2.0,
            persistence: 1.25,
            attenuation: 2.5,
        }
    }
}

impl Default for NoiseSettings {
    /// Provides an instance of `NoiseSettings` with the default parameters, seed is generated randomly
    fn default() -> Self {
        Self {
            seed: thread_rng().next_u32(),
            octaves: 12,
            frequency: 2.5,
            lacunarity: 2.0,
            persistence: 1.25,
            attenuation: 2.5,
        }
    }
}

/// Defines the settings that the noise generator uses to give rise to the noise map
pub struct NoiseSettings {
    /// define the world generator seed, used to build the noise map, normally a random value
    seed: u32,
    /// Total number of frequency octaves to generate the noise with.
    /// The number of octaves control the amount of detail in the noise function. Adding more octaves increases the detail, with the drawback of increasing the calculation time.
    pub octaves: usize,
    /// The number of cycles per unit length that the noise function outputs.
    pub frequency: f64,
    /// A multiplier that determines how quickly the frequency increases for each successive octave in the noise function.
    /// The frequency of each successive octave is equal to the product of the previous octave’s frequency and the lacunarity value.
    /// A lacunarity of 2.0 results in the frequency doubling every octave. For almost all cases, 2.0 is a good value to use.
    pub lacunarity: f64,
    ///A multiplier that determines how quickly the amplitudes diminish for each successive octave in the noise function.
    /// The amplitude of each successive octave is equal to the product of the previous octave’s amplitude and the persistence value. Increasing the persistence produces “rougher” noise.
    pub persistence: f64,
    /// The attenuation to apply to the weight on each octave. This reduces the strength of each successive octave, making their respective ridges smaller. The default attenuation is 2.0, making each octave half the height of the previous.
    pub attenuation: f64,
}

impl NoiseSettings {
    /// Creates a new instance of `NoiseSettings` with the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `seed` - The world generator seed used to build the noise map.
    /// * `octaves` - Total number of frequency octaves.
    /// * `frequency` - Number of cycles per unit length.
    /// * `lacunarity` - Multiplier determining frequency increase.
    /// * `persistence` - Multiplier determining amplitude decrease.
    /// * `attenuation` - Attenuation applied to octave weights.
    ///
    /// # Returns
    ///
    /// A new instance of `NoiseSettings` initialized with the provided parameters.
    ///
    /// # Example
    ///
    /// ```
    /// use exclusion_zone::generator::NoiseSettings;
    ///
    /// let settings = NoiseSettings::new(123, 5, 1.0, 2.0, 0.5, 1.0);
    /// ```
    pub fn new(
        seed: u32,
        octaves: usize,
        frequency: f64,
        lacunarity: f64,
        persistence: f64,
        attenuation: f64,
    ) -> Self {
        NoiseSettings {
            seed,
            octaves,
            frequency,
            lacunarity,
            persistence,
            attenuation,
        }
    }
}

/// Define the thresholds within which tile types are assigned
pub struct Thresholds {
    /// define at what depth the land will be considered deep water
    pub threshold_deep_water: f64,
    /// define at what depth the land will be considered shallow water
    pub threshold_shallow_water: f64,
    /// define at what height the land will be considered sand
    pub threshold_sand: f64,
    /// define at what height the land will be considered grass
    pub threshold_grass: f64,
    /// define at what height the land will be considered hill
    pub threshold_hill: f64,
    /// define at what height the land will be considered mountain
    pub threshold_mountain: f64,
}

impl Default for Thresholds {
    /// Provides an instance of `Thresholds` with the default parameters
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

impl Thresholds {
    /// Creates a new instance of `Thresholds` with the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `threshold_deep_water` - Depth at which the land will be considered deep water.
    /// * `threshold_shallow_water` - Depth at which the land will be considered shallow water.
    /// * `threshold_sand` - Height at which the land will be considered sand.
    /// * `threshold_grass` - Height at which the land will be considered grass.
    /// * `threshold_hill` - Height at which the land will be considered hill.
    /// * `threshold_mountain` - Height at which the land will be considered mountain.
    ///
    /// # Returns
    ///
    /// A new instance of `Thresholds` initialized with the provided parameters.
    ///
    /// # Example
    ///
    /// ```
    /// use exclusion_zone::generator::Thresholds;
    /// let thresholds = Thresholds::new(0.0, -0.1, 0.1, 0.3, 0.6, 0.8);
    /// ```
    pub fn new(
        threshold_deep_water: f64,
        threshold_shallow_water: f64,
        threshold_sand: f64,
        threshold_grass: f64,
        threshold_hill: f64,
        threshold_mountain: f64,
    ) -> Self {
        Thresholds {
            threshold_deep_water,
            threshold_shallow_water,
            threshold_sand,
            threshold_grass,
            threshold_hill,
            threshold_mountain,
        }
    }
}

/// Groups all sub-module settings of the world generator, allowing the various aspects to be customised
pub struct WorldGenerator {
    /// the world side dimension, final size will be size²
    pub size: usize,
    /// settings of the noise generator uses to give rise to the noise map
    pub noise_settings: NoiseSettings,
    /// thresholds within which tile types are assigned
    pub thresholds: Thresholds,
    /// define how the lava will spawn
    pub lava_settings: LavaSettings,
    /// define how banks will spawn
    pub bank_settings: BankSettings,
    /// define how bin will spawn
    pub bin_settings: BinSettings,
    /// define how wood crate will spawn
    pub crate_settings: CrateSettings,
    /// define how garbage will spawn
    pub garbage_settings: GarbageSettings,
    pub fire_settings: FireSettings,
}

impl WorldGenerator {
    #[inline(always)]
    fn generate_terrain(&self, noise_map: &[Vec<f64>], min: f64, max: f64) -> TileMatrix {
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
                world[*y][*x].tile_type = TileType::Street;
            }
        }
        world
    }
    #[inline(always)]
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

    /// Provides an instance of `WorldGenerator` given the world settings
    ///
    /// # Arguments
    ///
    /// * `size` - The world side dimension, final size will be size²
    /// * `noise_settings` - settings of the noise generator uses to give rise to the noise map
    /// * `thresholds` - thresholds within which tile types are assigned
    /// * `lava_settings` - define how the lava will spawn
    /// * `bank_settings` - define how banks will spawn
    /// * `bin_settings` - define how bin will spawn
    /// * `crate_settings` - define how wood crate will spawn
    /// * `garbage_settings` - define how garbage will spawn
    ///
    /// # Returns
    ///
    /// A new instance of `WorldGenerator` initialized with the provided settings.
    ///
    /// # Example
    ///
    /// ```
    /// use rand::{RngCore, thread_rng};
    /// use exclusion_zone::content::fire::FireSettings;
    /// use exclusion_zone::generator::{WorldGenerator, NoiseSettings, Thresholds, LavaSettings, BankSettings, BinSettings, CrateSettings, GarbageSettings};
    ///
    /// let world_size = 1000;
    /// let noise_settings = NoiseSettings::from_seed(thread_rng().next_u32());
    /// let thresholds = Thresholds::default();
    /// let lava_settings = LavaSettings::default(world_size);
    /// let bank_settings = BankSettings::default(world_size);
    /// let bin_settings = BinSettings::default(world_size);
    /// let crate_settings = CrateSettings::default(world_size);
    /// let garbage_settings = GarbageSettings::default(world_size);
    /// let fire_settings = FireSettings::default(world_size);
    /// let world = WorldGenerator::new(world_size,noise_settings,thresholds,lava_settings,bank_settings,bin_settings,crate_settings,garbage_settings,fire_settings);
    /// ```
    pub fn new(
        size: usize,
        noise_settings: NoiseSettings,
        thresholds: Thresholds,
        lava_settings: LavaSettings,
        bank_settings: BankSettings,
        bin_settings: BinSettings,
        crate_settings: CrateSettings,
        garbage_settings: GarbageSettings,
        fire_settings: FireSettings,
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
            fire_settings,
        }
    }

    /// Provides an instance of `WorldGenerator` with optimal parameters for the given world size
    ///
    /// # Arguments
    ///
    /// * `size`: The size of one side of the world
    ///
    /// # Returns
    ///
    /// A new instance of `WorldGenerator` initialized with the optimal parameters for the given world size
    ///
    /// # Examples
    ///
    /// ```
    /// use exclusion_zone::generator::WorldGenerator;
    /// let world_size = 1000;
    /// let world = WorldGenerator::default(world_size);
    /// ```
    pub fn default(size: usize) -> Self {
        Self {
            size,
            noise_settings: NoiseSettings::default(),
            thresholds: Thresholds::default(),
            lava_settings: LavaSettings::default(size),
            bank_settings: BankSettings::default(size),
            bin_settings: BinSettings::default(size),
            crate_settings: CrateSettings::default(size),
            garbage_settings: GarbageSettings::default(size),
            fire_settings: FireSettings::default(size),
        }
    }
}

/// Alias for `Vec<Vec<Tile>>` which is the Tile matrix representing the world
pub type TileMatrix = Vec<Vec<Tile>>;

/// Alias for `(usize, usize)` which are 2D coordinates, in x, y order
pub type Coordinates = (usize, usize);

impl Generator for WorldGenerator {
    /// Generates a new world based on the specified settings.
    ///
    /// This method generates a new world using the settings specified in the `WorldGenerator` instance.
    ///
    /// # Returns
    ///
    /// Returns a tuple containing the generated world represented as a matrix of `Tile` of type `World`,
    /// initial robot coordinates of type `Coordinates`, the environmental conditions,
    /// a floating-point value representing the max score of the world and optional score_table
    /// used in score.rs. If None is provided, uses default score_table..
    ///
    /// # Examples
    ///
    /// ```
    /// use robotics_lib::world::world_generator::Generator;
    /// use exclusion_zone::content::fire::FireSettings;
    /// use exclusion_zone::generator::{
    ///     WorldGenerator, NoiseSettings, Thresholds, LavaSettings, BankSettings,
    ///     BinSettings, CrateSettings, GarbageSettings
    /// };
    ///
    /// let world_size = 1000;
    ///
    /// let mut world_generator = WorldGenerator::new(
    ///     world_size,
    ///     NoiseSettings::default(),
    ///     Thresholds::def(),
    ///     LavaSettings::default(world_size),
    ///     BankSettings::default(world_size),
    ///     BinSettings::default(world_size),
    ///     CrateSettings::default(world_size),
    ///     GarbageSettings::default(world_size),
    ///     FireSettings::default(world_size),
    /// );
    ///
    /// let generated = world_generator.gen();
    /// ```
    fn gen(&mut self) -> (TileMatrix, Coordinates, EnvironmentalConditions, f32, Option<HashMap<Content, f32>>) {
        let tot = Utc::now();
        debug_println!("Start: Noise map generation");
        let mut start = Utc::now();
        let noise_map = self.generate_elevation_map();
        debug_println!("Done: Generate noise map: {} ms", (Utc::now() - start).num_milliseconds());

        debug_println!("Start: Calculate min and max value");
        start = Utc::now();
        let min_value = find_min_value(&noise_map).unwrap_or(f64::MAX); // get min value
        let max_value = find_max_value(&noise_map).unwrap_or(f64::MIN);
        // get max value
        debug_println!("Done: Calculate min and max value: {} ms", (Utc::now() - start).num_milliseconds());

        debug_println!("Start: Generate terrain");
        start = Utc::now();
        let mut world = self.generate_terrain(&noise_map, min_value, max_value);
        debug_println!("Done: Generate terrain: {} ms", (Utc::now() - start).num_milliseconds());

        // spawn lava
        debug_println!("Start: Spawn lava");
        start = Utc::now();
        spawn_lava(&mut world, &noise_map, self.lava_settings.clone());
        debug_println!("Done: Spawn lava: {} ms", (Utc::now() - start).num_milliseconds());

        // spawn bank
        debug_println!("Start: Spawn bank");
        start = Utc::now();
        spawn_bank(&mut world, self.bank_settings.clone());
        debug_println!("Done: Spawn bank: {} ms", (Utc::now() - start).num_milliseconds());

        // spawn bin
        debug_println!("Start: Spawn bin");
        start = Utc::now();
        spawn_bin(&mut world, self.bin_settings.clone());
        debug_println!("Done: Spawn bin: {} ms", (Utc::now() - start).num_milliseconds());

        // spawn wood_crate
        debug_println!("Start: Spawn crate");
        start = Utc::now();
        spawn_crate(&mut world, self.crate_settings.clone());
        debug_println!("Done: Spawn crate: {} ms", (Utc::now() - start).num_milliseconds());

        // spawn garbage
        debug_println!("Start: Spawn garbage");
        start = Utc::now();
        spawn_garbage(&mut world, &self.garbage_settings);
        debug_println!("Done: Spawn garbage in {} ms", (Utc::now() - start).num_milliseconds());

        // spawn fires
        debug_println!("Start: Spawn fire");
        start = Utc::now();
        spawn_fires(&mut world, &self.fire_settings);
        debug_println!("Done: Spawn fire in {} ms", (Utc::now() - start).num_milliseconds());

        debug_println!("World completed in: {} ms", (Utc::now() - tot).num_milliseconds());
        (world, (0, 0), EnvironmentalConditions::new(&[Rainy, Sunny, Foggy, TropicalMonsoon, TrentinoSnow], 1, 9).unwrap(), 0.0, None)
    }
}
