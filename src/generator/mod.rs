use std::collections::{HashMap, HashSet};

use chrono::Utc;
use debug_print::debug_println;
use noise::MultiFractal;
use noise::NoiseFn;
use noise::{Fbm, Perlin, RidgedMulti};
use rand::seq::SliceRandom;
use rand::{thread_rng, RngCore, Rng};
use rayon::iter::IntoParallelIterator;
use rayon::iter::*;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType::{Foggy, Rainy, Sunny, TrentinoSnow, TropicalMonsoon};
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::world_generator::Generator;
use serde::{Deserialize, Serialize};

use crate::content::bank::{spawn_bank, BankSettings};
use crate::content::bin::{spawn_bin, BinSettings};
use crate::content::coin::CoinSettings;
use crate::content::fire::{spawn_fire, FireSettings};
use crate::content::garbage::{spawn_garbage, GarbageSettings};
use crate::content::tree::{spawn_tree, TreeSettings};
use crate::content::wood_crate::{spawn_crate, CrateSettings};
use crate::tile_type::lava::{spawn_lava, LavaSettings};
use crate::tile_type::street::street_spawn;
use crate::utils::{find_max_value, find_min_value, percentage, SerializedWorld};

/// Contains the tile types and the content used to define generation order
#[derive(Eq, PartialEq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Spawnables {
    Street,
    Lava,
    Rock,
    Tree,
    Garbage,
    Fire,
    Coin,
    Bin,
    Crate,
    Bank,
    Market,
    Fish,
    Building,
    JollyBlock,
    City,
}

/// Set of content and tile type defining the order of element generation,
/// if an element appears twice, the second is ignored and <b>if an element does not appear, it is not spawned</b>
pub type SpawnOrder = Vec<Spawnables>;

/// Generates a randomized order for spawning `TileType` and `Content`.
///
/// # Returns
/// A `SpawnOrder` (Vec<Spawnables>), which is a vector of `Spawnables` enum variants.
/// This vector represents the order in which different elements (like Bank, Bin, Building, etc.) should be spawned.
/// The order is randomized each time the function is called.
///
/// # Behavior
/// - Initializes a mutable vector `elements` with all variants of the `Spawnables` enum.
/// - Shuffles this vector using a random number generator (`thread_rng()`), ensuring that the order of elements is randomized.
/// - The randomized vector is then returned as the spawn order.
///
/// # Usage Notes
/// - The returned `SpawnOrder` dictates the order in which elements are generated in the game or simulation environment.
/// - If an element appears twice in a provided `SpawnOrder`, only the first occurrence is considered.
/// - Elements not included in the `SpawnOrder` will not be spawned.
/// - Uses thread-local random number generator for shuffling, making each call to this function likely to produce a different order.
///
/// # Example
/// ```
/// use exclusion_zone::content::bank::BankSettings;
/// use exclusion_zone::content::bin::BinSettings;
/// use exclusion_zone::content::coin::CoinSettings;
/// use exclusion_zone::content::fire::FireSettings;
/// use exclusion_zone::content::garbage::GarbageSettings;
/// use exclusion_zone::content::tree::TreeSettings;
/// use exclusion_zone::content::wood_crate::CrateSettings;
/// use exclusion_zone::generator::{get_default_spawn_order, NoiseSettings, Thresholds, WorldGenerator};
/// use exclusion_zone::generator::Spawnables::Tree;
/// use exclusion_zone::tile_type::lava::LavaSettings;
/// let size = 1000;
/// let world_gen = WorldGenerator {
///             size,
///             spawn_order: get_default_spawn_order(),
///             noise_settings: NoiseSettings::default(),
///             thresholds: Thresholds::default(),
///             lava_settings: LavaSettings::default(size),
///             bank_settings: BankSettings::default(size),
///             bin_settings: BinSettings::default(size),
///             crate_settings: CrateSettings::default(size),
///             garbage_settings: GarbageSettings::default(size),
///             fire_settings: FireSettings::default(size),
///             tree_settings: TreeSettings::default(size),
///             coin_settings: CoinSettings::default(size)
///         };
/// // The `spawn_order` now contains a randomized order of elements to be spawned.
/// ```
#[inline(always)]
pub fn get_default_spawn_order() -> SpawnOrder {
    let mut elements = vec![
        Spawnables::Bank,
        Spawnables::Bin,
        Spawnables::Building,
        Spawnables::Coin,
        Spawnables::Crate,
        Spawnables::Fire,
        Spawnables::Fish,
        Spawnables::Garbage,
        Spawnables::JollyBlock,
        Spawnables::Lava,
        Spawnables::Market,
        Spawnables::Rock,
        Spawnables::Street,
        Spawnables::Tree,
        Spawnables::City,
    ];
    elements.shuffle(&mut thread_rng());
    elements
}

#[inline(always)]
fn remove_duplicates_spawnables(order: &mut SpawnOrder) {
    let mut seen = HashSet::with_capacity(order.len());
    order.retain(|i| seen.insert(*i));
}

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
#[derive(Serialize, Deserialize, Copy, Clone)]
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
    pub fn new(seed: u32, octaves: usize, frequency: f64, lacunarity: f64, persistence: f64, attenuation: f64) -> Self {
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
#[derive(Serialize, Deserialize, Copy, Clone)]
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
    pub fn new(threshold_deep_water: f64, threshold_shallow_water: f64, threshold_sand: f64, threshold_grass: f64, threshold_hill: f64, threshold_mountain: f64) -> Self {
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
#[derive(Serialize, Deserialize, Clone)]
pub struct WorldGenerator {
    /// the world side dimension, final size will be size²
    pub size: usize,
    /// set of content and tile type defining the order with which elements are generated
    pub spawn_order: SpawnOrder,
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
    /// define how fire will spawn
    pub fire_settings: FireSettings,
    /// define how trees will spawn
    pub tree_settings: TreeSettings,
    // define how coins will spawn
    pub coin_settings: CoinSettings
}

impl WorldGenerator {
    #[inline(always)]
    fn generate_terrain(&self, noise_map: &[Vec<f64>], min: f64, max: f64) -> TileMatrix {
        let mut world = vec![
            vec![
                Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                    elevation: 0
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

                let rock_probability = match value {
                    | v if v < percentage(self.thresholds.threshold_deep_water, min, max) => 0.0,
                    | v if v < percentage(self.thresholds.threshold_shallow_water, min, max) => 0.0,
                    | v if v < percentage(self.thresholds.threshold_sand, min, max) => 0.2,
                    | v if v < percentage(self.thresholds.threshold_grass, min, max) => 0.4,
                    | v if v < percentage(self.thresholds.threshold_hill, min, max) => 0.6,
                    | v if v < percentage(self.thresholds.threshold_mountain, min, max) => 0.8,
                    | _ => 0.9,
                };

                let rock = thread_rng().gen_bool(rock_probability);
                let mut content = Content::None;

                if rock {
                    // random quantity of rock
                    let qt = thread_rng().gen_range(0..=Content::Rock(0).properties().max());
                    content = Content::Rock(qt);
                }

                world[y][x] = Tile {
                    tile_type,
                    content,
                    elevation: 0,
                };
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
    /// use exclusion_zone::content::coin::CoinSettings;
    /// use exclusion_zone::content::fire::FireSettings;
    /// use exclusion_zone::content::tree::TreeSettings;
    /// use exclusion_zone::generator::{WorldGenerator, NoiseSettings, Thresholds, LavaSettings, BankSettings, BinSettings, CrateSettings, GarbageSettings, SpawnOrder, Spawnables};
    ///
    /// let world_size = 1000;
    /// let spawn_order : SpawnOrder = vec![
    ///         Spawnables::Bank,
    ///         Spawnables::Bin,
    ///         Spawnables::Building,
    ///         Spawnables::Coin,
    ///         Spawnables::Crate,
    ///         Spawnables::Fire,
    ///         Spawnables::Fish,
    ///         Spawnables::Garbage,
    ///         Spawnables::JollyBlock,
    ///         Spawnables::Lava,
    ///         Spawnables::Market,
    ///         Spawnables::Rock,
    ///         Spawnables::Street,
    ///         Spawnables::Tree,
    ///         Spawnables::City,
    ///     ];
    /// let noise_settings = NoiseSettings::from_seed(thread_rng().next_u32());
    /// let thresholds = Thresholds::default();
    /// let lava_settings = LavaSettings::default(world_size);
    /// let bank_settings = BankSettings::default(world_size);
    /// let bin_settings = BinSettings::default(world_size);
    /// let crate_settings = CrateSettings::default(world_size);
    /// let garbage_settings = GarbageSettings::default(world_size);
    /// let fire_settings = FireSettings::default(world_size);
    /// let tree_settings = TreeSettings::default(world_size);
    /// let coin_settings = CoinSettings::default(world_size);
    /// let world = WorldGenerator::new(world_size,spawn_order,noise_settings,thresholds,lava_settings,bank_settings,bin_settings,crate_settings,garbage_settings,fire_settings,tree_settings,coin_settings);
    /// ```
    pub fn new(
        size: usize,
        spawn_order: SpawnOrder,
        noise_settings: NoiseSettings,
        thresholds: Thresholds,
        lava_settings: LavaSettings,
        bank_settings: BankSettings,
        bin_settings: BinSettings,
        crate_settings: CrateSettings,
        garbage_settings: GarbageSettings,
        fire_settings: FireSettings,
        tree_settings: TreeSettings,
        coin_settings: CoinSettings
    ) -> Self {
        Self {
            size,
            spawn_order,
            noise_settings,
            thresholds,
            lava_settings,
            bank_settings,
            bin_settings,
            crate_settings,
            garbage_settings,
            fire_settings,
            tree_settings,
            coin_settings
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
            spawn_order: get_default_spawn_order(),
            noise_settings: NoiseSettings::default(),
            thresholds: Thresholds::default(),
            lava_settings: LavaSettings::default(size),
            bank_settings: BankSettings::default(size),
            bin_settings: BinSettings::default(size),
            crate_settings: CrateSettings::default(size),
            garbage_settings: GarbageSettings::default(size),
            fire_settings: FireSettings::default(size),
            tree_settings: TreeSettings::default(size),
            coin_settings :CoinSettings::default(size)
        }
    }
    /// Generates a new world based on the current settings and serializes it.
    ///
    /// This method generates a new world and couples it with the current settings. It then serializes this combined
    /// data into a binary format and compresses it using Zstandard for efficient storage. Finally,
    /// the compressed binary data is saved to a file specified by the file_path parameter, appending a .zst
    /// extension to the file name.
    ///
    /// # Arguments
    ///
    /// `file_path`: The path and the name of the file to generate as `&str`
    ///
    /// # Panics
    ///
    /// This method will panic if:
    /// - The file specified by `file_path` cannot be created.
    /// - There is an error in writing to the file.
    ///
    /// # Examples
    ///
    /// ```
    /// use robotics_lib::world::world_generator::Generator;
    /// use exclusion_zone::content::bank::BankSettings;
    /// use exclusion_zone::content::bin::BinSettings;
    /// use exclusion_zone::content::coin::CoinSettings;
    /// use exclusion_zone::content::fire::FireSettings;
    /// use exclusion_zone::content::garbage::GarbageSettings;
    /// use exclusion_zone::content::tree::TreeSettings;
    /// use exclusion_zone::content::wood_crate::CrateSettings;
    /// use exclusion_zone::generator::{get_default_spawn_order, NoiseSettings, Thresholds, WorldGenerator};
    /// use exclusion_zone::tile_type::lava::LavaSettings;
    ///
    /// let world_size = 1000;
    ///
    /// let mut world_generator = WorldGenerator::new(
    ///     world_size,
    ///     get_default_spawn_order(),
    ///     NoiseSettings::default(),
    ///     Thresholds::def(),
    ///     LavaSettings::default(world_size),
    ///     BankSettings::default(world_size),
    ///     BinSettings::default(world_size),
    ///     CrateSettings::default(world_size),
    ///     GarbageSettings::default(world_size),
    ///     FireSettings::default(world_size),
    ///     TreeSettings::default(world_size),
    ///     CoinSettings::default(world_size)
    /// );
    /// world_generator.generate_and_save("file/path/name").expect("Unable to save the world");
    /// ```
    pub fn generate_and_save(&mut self, file_path: &str) -> Result<(), String> {
        SerializedWorld {
            settings: self.clone(),
            world: self.gen(),
        }
        .serialize(file_path, 11)
    }

    /// Saves the current world settings along with the provided world data to a file.
    ///
    /// This function creates an instance of `SerializedWorld` using the current
    /// `WorldGenerator` settings and the provided `world` data. It then serializes
    /// this instance into a binary format, optionally compresses it, and saves it to
    /// the specified file path with a `.bsw` extension.
    ///
    /// The serialization and compression are handled by the `serialize` method of
    /// `SerializedWorld`. The compression level is set to `11`, but this can be
    /// adjusted based on the desired balance between compression efficiency and
    /// performance.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A string slice specifying the path to the file where the
    ///   serialized data will be saved. The `.bsw` extension will be appended to
    ///   this path.
    /// * `world` - The world data to be saved, represented as a `GenResult`. This
    ///   includes all relevant world information like tile matrix, coordinates,
    ///   environmental conditions, and other related data.
    ///
    /// # Returns
    ///
    /// Returns a `Result<(), String>`:
    /// - On success, it returns `Ok(())`.
    /// - On failure, it returns `Err(String)` with a description of the error.
    ///
    /// # Examples
    /// ```
    /// use robotics_lib::world::world_generator::Generator;
    /// use exclusion_zone::content::bank::BankSettings;
    /// use exclusion_zone::content::bin::BinSettings;
    /// use exclusion_zone::content::coin::CoinSettings;
    /// use exclusion_zone::content::fire::FireSettings;
    /// use exclusion_zone::content::garbage::GarbageSettings;
    /// use exclusion_zone::content::tree::TreeSettings;
    /// use exclusion_zone::content::wood_crate::CrateSettings;
    /// use exclusion_zone::generator::{get_default_spawn_order, NoiseSettings, Thresholds, WorldGenerator};
    /// use exclusion_zone::tile_type::lava::LavaSettings;
    ///
    /// let world_size = 1000;
    ///
    /// let mut world_generator = WorldGenerator::new(
    ///     world_size,
    ///     get_default_spawn_order(),
    ///     NoiseSettings::default(),
    ///     Thresholds::def(),
    ///     LavaSettings::default(world_size),
    ///     BankSettings::default(world_size),
    ///     BinSettings::default(world_size),
    ///     CrateSettings::default(world_size),
    ///     GarbageSettings::default(world_size),
    ///     FireSettings::default(world_size),
    ///     TreeSettings::default(world_size),
    ///     CoinSettings::default(world_size)
    /// );
    /// let world = world_generator.gen();
    /// /* do stuff with the world, like visualize etc...*/
    /// world_generator.save("path/to/file", world).expect("unable to save the world");
    /// ```
    ///
    /// # Errors
    ///
    /// This function may return an error if it encounters issues during the
    /// serialization process or while writing to the file. The error message will
    /// provide details on the nature of the problem encountered.
    pub fn save(&mut self, file_path: &str, world: GenResult) -> Result<(), String> {
        SerializedWorld {
            settings: self.clone(),
            world,
        }
        .serialize(file_path, 11)
    }

    /// Loads a previously saved world from file.
    ///
    /// This function attempts to load and deserialize a world and the settings used to generate it.
    ///  If successful, it extracts and returns the
    /// `WorldGenerator` settings along with the world data `(TileMatrix, Coordinates, EnvironmentalConditions, f32, Option<HashMap<Content, f32>>)`
    /// the same yuo will get when generating a new world.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A string slice that specifies the path to the binary file
    ///   containing the saved world.
    ///
    /// # Returns
    ///
    /// Returns a `Result<(WorldGenerator, (TileMatrix, Coordinates, EnvironmentalConditions, f32, Option<HashMap<Content, f32>>)), String>`:
    /// - On success, provides a tuple consisting of the `WorldGenerator` settings
    ///   and the detailed world data.
    /// - On failure, returns a `String` error message detailing the issue
    ///   encountered during the loading process.
    ///
    /// # Examples
    ///
    /// ```
    /// use exclusion_zone::generator::WorldGenerator;
    /// let file_path = "path/to/saved_world.zst";
    ///
    /// let world_and_data = match WorldGenerator::load_saved(file_path) {
    ///     Ok((settings, (tile_matrix, coordinates, environmental_conditions, metric, content_map))) => {
    ///         println!("World loaded successfully.");
    ///         // Use settings and the world data here...
    ///     }
    ///     Err(e) => {
    ///         eprintln!("Error loading world: {}", e);
    ///     }
    /// };
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error string if it encounters any issues during
    /// the deserialization process, such as problems with reading the file,
    /// decompression, or deserialization itself. The error string will contain
    /// details about the specific problem encountered.
    pub fn load_saved(file_path: &str) -> Result<(WorldGenerator, GenResult), String> {
        match SerializedWorld::deserialize(file_path) {
            | Ok(c) => Ok((c.settings, c.world)),
            | Err(e) => Err(format!("Unable to load world file {file_path}:\n{e}")),
        }
    }
}

/// Alias for `Vec<Vec<Tile>>` which is the Tile matrix representing the world
pub type TileMatrix = Vec<Vec<Tile>>;

pub(crate) type GenResult = (TileMatrix, (usize, usize), EnvironmentalConditions, f32, Option<HashMap<Content, f32>>);

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
    /// use exclusion_zone::content::bank::BankSettings;
    /// use exclusion_zone::content::bin::BinSettings;
    /// use exclusion_zone::content::coin::CoinSettings;
    /// use exclusion_zone::content::fire::FireSettings;
    /// use exclusion_zone::content::garbage::GarbageSettings;
    /// use exclusion_zone::content::tree::TreeSettings;
    /// use exclusion_zone::content::wood_crate::CrateSettings;
    /// use exclusion_zone::generator::{get_default_spawn_order, NoiseSettings, Thresholds, WorldGenerator};
    /// use exclusion_zone::tile_type::lava::LavaSettings;
    ///
    /// let world_size = 1000;
    ///
    /// let mut world_generator = WorldGenerator::new(
    ///     world_size,
    ///     get_default_spawn_order(),
    ///     NoiseSettings::default(),
    ///     Thresholds::def(),
    ///     LavaSettings::default(world_size),
    ///     BankSettings::default(world_size),
    ///     BinSettings::default(world_size),
    ///     CrateSettings::default(world_size),
    ///     GarbageSettings::default(world_size),
    ///     FireSettings::default(world_size),
    ///     TreeSettings::default(world_size),
    ///     CoinSettings::default(world_size)
    /// );
    ///
    /// let generated = world_generator.gen();
    /// ```
    fn gen(&mut self) -> GenResult {

        if self.size < 100 {
            panic!("The world size must be at least 100");
        }

        let tot = Utc::now();

        debug_println!("Start: Noise map generation");
        let mut start = Utc::now();
        let noise_map = self.generate_elevation_map();
        debug_println!("Done: Generate noise map: {} ms", (Utc::now() - start).num_milliseconds());

        debug_println!("Start: Calculate min and max value");
        start = Utc::now();
        let min_value = find_min_value(&noise_map).unwrap_or(f64::MAX);
        let max_value = find_max_value(&noise_map).unwrap_or(f64::MIN);
        debug_println!("Done: Calculate min and max value: {} ms", (Utc::now() - start).num_milliseconds());

        debug_println!("Start: Generate terrain");
        start = Utc::now();
        let mut world = self.generate_terrain(&noise_map, min_value, max_value);
        debug_println!("Done: Generate terrain: {} ms", (Utc::now() - start).num_milliseconds());

        remove_duplicates_spawnables(&mut self.spawn_order);

        for content in &self.spawn_order {
            match content {
                | Spawnables::Street => {
                    //color local maxima black
                    let polygons = street_spawn(self.size / 250, &noise_map, 10, 0.0);

                    for polygon in polygons.iter() {
                        for c in polygon {
                            world[c.row][c.col].tile_type = TileType::Street;
                        }
                    }
                }
                | Spawnables::Lava => {
                    debug_println!("Start: Spawn lava");
                    start = Utc::now();
                    spawn_lava(&mut world, &noise_map, self.lava_settings.clone());
                    debug_println!("Done: Spawn lava: {} ms", (Utc::now() - start).num_milliseconds());
                }
                | Spawnables::Tree => {
                    debug_println!("Start: Spawn trees");
                    start = Utc::now();
                    spawn_tree(&mut world, &mut self.tree_settings);
                    debug_println!("Done: Spawn trees in {} ms", (Utc::now() - start).num_milliseconds());
                }
                | Spawnables::Garbage => {
                    debug_println!("Start: Spawn garbage");
                    start = Utc::now();
                    spawn_garbage(&mut world, &self.garbage_settings);
                    debug_println!("Done: Spawn garbage in {} ms", (Utc::now() - start).num_milliseconds());
                }
                | Spawnables::Fire => {
                    debug_println!("Start: Spawn fire");
                    start = Utc::now();
                    spawn_fire(&mut world, &mut self.fire_settings);
                    debug_println!("Done: Spawn fire in {} ms", (Utc::now() - start).num_milliseconds());
                }
                | Spawnables::Bin => {
                    debug_println!("Start: Spawn bin");
                    start = Utc::now();
                    spawn_bin(&mut world, self.bin_settings.clone());
                    debug_println!("Done: Spawn bin: {} ms", (Utc::now() - start).num_milliseconds());
                }
                | Spawnables::Crate => {
                    debug_println!("Start: Spawn crate");
                    start = Utc::now();
                    spawn_crate(&mut world, self.crate_settings.clone());
                    debug_println!("Done: Spawn crate: {} ms", (Utc::now() - start).num_milliseconds());
                }
                | Spawnables::Bank => {
                    debug_println!("Start: Spawn bank");
                    start = Utc::now();
                    spawn_bank(&mut world, self.bank_settings);
                    debug_println!("Done: Spawn bank: {} ms", (Utc::now() - start).num_milliseconds());
                }
                | Spawnables::Coin => {}
                | Spawnables::Market => {}
                | Spawnables::Fish => {}
                | Spawnables::Building => {}
                | Spawnables::JollyBlock => {}
                | Spawnables::City => {}
                | Spawnables::Rock => {}
            }
        }

        // Detect the first walkable tile and set the initial position of the robot
        let mut robot_position = (0, 0);
        for (y, row) in world.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if tile.tile_type.properties().walk() {
                    robot_position = (x, y);
                    break;
                }
            }
        }

        debug_println!("World completed in: {} ms", (Utc::now() - tot).num_milliseconds());
        (
            world,
            robot_position,
            EnvironmentalConditions::new(&[Rainy, Sunny, Foggy, TropicalMonsoon, TrentinoSnow], 15, 9).unwrap(),
            100.0,
            None,
        )
    }
}
