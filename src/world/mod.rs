use crate::world::environmental_conditions::EnvironmentalConditions;
use crate::world::tile::Tile;

pub mod coordinates;
pub mod tile;
pub mod worldgenerator;

pub mod environmental_conditions;

// ----------------------------------------------------
// World

/// Represents the game world.
///
/// The `World` struct is used to define the game world, which includes a map made up of `Tile`
/// instances, the dimension of the map, the position of a robot within the world, and the robot's
/// direction.
///
/// # Usage
///
/// ```
/// use robotics_lib::runner::Runnable;
/// use robotics_lib::world::World;
/// use robotics_lib::world::worldgenerator::Generator;
/// fn create_world(generator: &mut impl Generator) {
///     // let world = World::new(generator.generate_world());
///     // give the world to functions that need it
/// }
/// ```
///
/// # Fields
/// - `map`: A 2D vector representing the map made up of `Tile` instances.
/// - `dimension`: The dimension of the map (e.g., the side length of a square map).
/// - `environmental_conditions`: The environmental conditions of the world (daytime and weather).
#[derive(Debug)]
pub struct World {
    pub(crate) map: Vec<Vec<Tile>>,
    pub(crate) dimension: usize,
    pub(crate) environmental_conditions: EnvironmentalConditions,
}

impl World {
    pub(crate) fn new(map: Vec<Vec<Tile>>, environmental_conditions: EnvironmentalConditions) -> World {
        let dimension = map.len();
        World {
            map,
            dimension,
            environmental_conditions,
        }
    }
    pub fn advance_time(&mut self) {
        self.environmental_conditions.tick();
    }
}
