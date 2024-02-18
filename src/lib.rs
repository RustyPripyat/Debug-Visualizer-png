//! This crate defines the world generator <b>Exclusion Zone</b>
//! your robot will be swept into an arduous map, full of pitfalls, just like the Černobyl exclusion
//! zone, you will feel like you are in the 1986 USSR.
//!
//! This world generator allows for granular customisation of its content, it allows you to specify
//! the order in which the various Tile types and Tile content are generated, allowing you to define
//! priorities. To ensure the best possible performance, multi-threading is exploited wherever possible.
//!
//! There are methods to pre-generate the world, save it as a binary file and load it later.
//!
//! We recommend a size of at least <b>1000</b>.
//! Size lower than 100 will throw a panic
//!

extern crate core;

/// Contains a submodule for each tile content present in the common crate, each of which has a struct
/// to define the behavior of how it is generated, such as quantity, probability and so on
pub mod content;
/// Contains the world generator settings and method to generate the world map
pub mod generator;
/// Contains a submodule for each tile type present in the common crate, each of which has a struct
/// to define the behavior of how it is generated, such as number of lava spawn point, streets and so on
pub mod tile_type;
pub(crate) mod utils;
