use std::cmp::min;
use std::ops::Range;

use rand::seq::SliceRandom;
use robotics_lib::world::tile::TileType;
use serde::{Deserialize, Serialize};

use crate::generator::{Coordinates, TileMatrix};

/// Settings defining the behavior of lava generation within the world.
///
/// This struct represents the configuration for lava, including the number of spawn points
/// and the range of lava flow.
#[derive(Serialize, Deserialize, Clone)]
pub struct LavaSettings {
    /// The number of spawn points for lava within the world.
    pub number_of_spawn_points: usize,
    /// The range representing the potential flow distance of lava.
    ///
    /// This range defines the minimum and maximum possible distance that lava can flow from
    /// its source point.
    pub lava_flow_range: Range<usize>,
}

impl LavaSettings {
    /// Custom version of default that provides an instance of `LavaSettings` with the
    /// optimal parameters for the given world size
    pub fn default(size: usize) -> Self {
        LavaSettings {
            number_of_spawn_points: size / 25,
            lava_flow_range: 1..size / 25,
        }
    }

    /// Creates a new instance of `LavaSettings` with the given number of spawn points
    /// and lava flow range.
    ///
    /// # Arguments
    ///
    /// * `spawn_points` - The number of spawn points for lava within the world.
    /// * `flow_range` - The range representing the potential flow distance of lava.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use std::ops::Range;
    /// use exclusion_zone::tile_type::lava::LavaSettings;
    ///
    /// let settings = LavaSettings::new(5, 1..15);
    /// ```
    pub fn new(spawn_points: usize, flow_range: Range<usize>) -> Self {
        LavaSettings {
            number_of_spawn_points: spawn_points,
            lava_flow_range: flow_range,
        }
    }
}

pub(crate) fn spawn_lava(world: &mut TileMatrix, elevation_map: &Vec<Vec<f64>>, lava_settings: LavaSettings) {
    let possible_spawn_points = get_yx_mountain_tiles(world);
    let min = min(lava_settings.number_of_spawn_points, possible_spawn_points.len());
    for i in 0..min {
        let (y, x) = possible_spawn_points[i];
        let range = lava_settings.lava_flow_range.clone();
        flow_from(world, elevation_map, y, x, range);
    }
}

//for each x,y flow the lava to the lower neighbour
/// fatina ricorsina
#[inline(always)]
fn flow_from(world: &mut TileMatrix, elevation_map: &Vec<Vec<f64>>, y: usize, x: usize, remaining_range: Range<usize>) -> usize {
    //debug_println!("flowing from {},{} with range {}..{}", x,y, remaining_range.start, remaining_range.end);
    world[y][x].tile_type = TileType::Lava;
    if remaining_range.start == remaining_range.end {
        0
    } else {
        // if there is a neighbour with a lower height, flow to it
        let (lowest_neighbour_y, lowest_neighbour_x) = get_lowest_neighbour(elevation_map, y, x);
        flow_from(world, elevation_map, lowest_neighbour_y, lowest_neighbour_x, remaining_range.start..remaining_range.end - 1)
        // if elevation_map[lowest_neighbour_y][lowest_neighbour_x] < elevation_map[y][x] {
        //     return flow_from(world, elevation_map, lowest_neighbour_y, lowest_neighbour_x, remaining_range.start..remaining_range.end - 1);
        // }
        // else {
        //     return remaining_range.end - remaining_range.start;
        // }
    }
}

// return the coordinates of the lowest neighbour
#[inline(always)]
fn get_lowest_neighbour(elevation_map: &Vec<Vec<f64>>, y: usize, x: usize) -> Coordinates {
    let mut neighbour_heights = Vec::new();
    if y != 0 {
        neighbour_heights.push((elevation_map[y - 1][x], y - 1, x));
    }
    if y != elevation_map.len() - 1 {
        neighbour_heights.push((elevation_map[y + 1][x], y + 1, x));
    }
    if x != 0 {
        neighbour_heights.push((elevation_map[y][x - 1], y, x - 1));
    }
    if x != elevation_map[0].len() - 1 {
        neighbour_heights.push((elevation_map[y][x + 1], y, x + 1));
    }
    // sort by height
    neighbour_heights.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    // return coordinates of the lowest neighbour
    (neighbour_heights[0].1, neighbour_heights[0].2)
}

// return vector with the coordinates of the mountain tiles in range
#[inline(always)]
fn get_yx_mountain_tiles(wordl: &mut TileMatrix) -> Vec<Coordinates> {
    let mut tiles_in_range = Vec::new();
    for (y, row) in wordl.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if tile.tile_type == TileType::Mountain {
                tiles_in_range.push((y, x));
            }
        }
    }
    tiles_in_range.as_mut_slice().shuffle(&mut rand::thread_rng());
    tiles_in_range
}
