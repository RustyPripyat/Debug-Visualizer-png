use crate::utils::LibError;
use crate::world::environmental_conditions::EnvironmentalConditions;
use crate::world::tile::{Content, Tile, TileType};
use std::collections::HashMap;
// ----------------------------------------------------
// WorldGenerator

/// A trait for generating worlds.
///
/// The `WorldGenerator` trait allows the implementation of the `new` function for the given `World` struct.
///
/// # Usage
///
/// ```
/// /*
/// impl WorldGenerator for World {
///     fn new(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions) {
///         // implementation
///     }
/// }
/// */
/// ```

// to generate the Content of a tile refer to the hashmap of Contentprops for the max n of elems
pub trait Generator {
    fn gen(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions);
}

pub fn check_world(world: &Vec<Vec<Tile>>) -> Result<bool, LibError> {
    for row in world {
        for tile in row {
            let value = match &tile.content {
                | Content::Rock(value) => value,
                | Content::Tree(value) => value,
                | Content::Garbage(value) => value,
                | Content::Fire => &0,
                | Content::Coin(value) => value,
                | Content::Bin(value) => &value.end,
                | Content::Crate(value) => &value.end,
                | Content::Bank(value) => &value.end,
                | Content::Water(value) => value,
                | Content::None => &0,
            };

            //all content-enum value is lower or equal to the tiletype-enum max
            let max = tile.content.properties().max();
            if value > &max {
                return Ok(false);
            }
        }
    }
    Ok(true)
}

pub fn get_tiletype_percentage(world: &Vec<Vec<Tile>>) -> HashMap<TileType, f64> {
    let mut percentage_map = HashMap::new();
    let total = (world.len() * world[0].len()) as f64;
    dbg!(total);
    for row in world {
        for tile in row {
            *percentage_map.entry(tile.tile_type).or_insert(0.0) += 1.0;
        }
    }

    //rewrite this with classic for loop
    for element in percentage_map.iter_mut() {
        dbg!(*element.1 /= total);
    }
    percentage_map
}

pub fn get_content_percentage(world: &Vec<Vec<Tile>>) -> HashMap<Content, f64> {
    let mut percentage_map = HashMap::new();
    let total = (world.len() * world[0].len()) as f64;

    for row in world {
        for tile in row {
            *percentage_map.entry(tile.content.clone()).or_insert(0.0) += 1.0;
        }
    }

    for element in percentage_map.iter_mut() {
        *element.1 /= total;
    }

    percentage_map
}
