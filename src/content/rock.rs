use rand::{thread_rng, Rng};
use robotics_lib::world::tile::Content::Rock;
use robotics_lib::world::tile::{ TileType};
use serde::{Deserialize, Serialize};

use rand::seq::SliceRandom;

use crate::generator::{ TileMatrix};

/// Settings defining the behavior of rock spawn,
/// such as the total number of rocks in the world
/// and the probability to spawn in each environment
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct RockSettings {
    /// The spawn probability sta for each environment (deep water, sand, mountains...).
    pub probability_vector: [f64; 7],
    /// The total number of rocks available in the world.
    pub max_num_rocks: usize
}

impl RockSettings {
    /// Custom version of default that provides an instance of `RockSettings` with the
    /// optimal parameters for the given world size
    pub fn default(size: usize)-> Self {
        let max_num_rocks = usize::pow(size,2) / 20;
        let probability_vector = [0.0,0.0,0.1,0.25,0.45,0.5,0.7];

        RockSettings{
            max_num_rocks,
            probability_vector,
        }
    }
    /// Creates a new instance of `RockSettings` with the given number of spawn points
    ///
    /// # Arguments
    ///
    /// * `max_num_rocks` - The total number of rocks available in the world.
    /// * `probability_vector` - The spawn probability sta for each environment.
    ///    the order is: `DeepWater, ShallowWater, Sand, Grass, Hill, Mountain, Snow`
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use std::ops::Range;
    /// use exclusion_zone::content::rock;
    /// use exclusion_zone::content::rock::RockSettings;
    ///
    /// let settings = RockSettings::new(500, [0.0,0.0,0.1,0.25,0.45,0.5,0.7]);
    /// ```
    pub fn new(max_num_rocks : usize, probability_vector:[f64; 7]) -> Self{
        RockSettings {
            probability_vector,
            max_num_rocks,
        }
    }
}

fn match_probabilities(rock_settings: RockSettings, tile_type: TileType ) -> f64 {
    match tile_type {
        TileType::DeepWater => { rock_settings.probability_vector[0] }
        TileType::ShallowWater => { rock_settings.probability_vector[1] }
        TileType::Sand => { rock_settings.probability_vector[2] }
        TileType::Grass => { rock_settings.probability_vector[3] }
        TileType::Street => { 0.0 }
        TileType::Hill => { rock_settings.probability_vector[4] }
        TileType::Mountain => { rock_settings.probability_vector[5] }
        TileType::Snow => { rock_settings.probability_vector[6] }
        TileType::Lava => { 0.0 }
        TileType::Teleport(_) => { 0.0 }
        TileType::Wall => { 0.0 }
    }
}

#[inline(always)]
pub(crate)  fn spawn_rock(world: &mut TileMatrix, rock_settings: RockSettings) {
    let mut cnt = rock_settings.max_num_rocks;

    let mut possible_rock_tile : Vec<(usize,usize)> = vec![];


    for (y,row) in world.iter().enumerate() {
        if cnt==0 {
            break;
        }
        for (x,tile) in row.iter().enumerate() {
            let tile_type = tile.tile_type;
            let prob = match_probabilities(rock_settings, tile_type);

            let rock = thread_rng().gen_bool(prob);
            let can_hold = tile.tile_type.properties().can_hold(&Rock(0).to_default());

            if rock && can_hold && cnt > 0{
                possible_rock_tile.push((y,x));
         cnt -= 1;

            }
            else if cnt==0 {
                break;
            }
        }
    }

    possible_rock_tile.shuffle(&mut thread_rng());

    for c in possible_rock_tile.iter(){
        // random quantity of rock
        let qt = thread_rng().gen_range(1..=Rock(0).properties().max());
        world[c.0][c.1].content = Rock(qt);
    }



}


