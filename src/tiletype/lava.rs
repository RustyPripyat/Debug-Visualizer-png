use crate::generator::LavaSettings;
use rand::seq::SliceRandom;
use robotics_lib::world::tile::Tile;
use robotics_lib::world::tile::TileType;
use std::cmp::min;
use std::ops::Range;

//
pub(crate) fn spawn_lava(world: &mut Vec<Vec<Tile>>, elevation_map: &Vec<Vec<f64>>, lava_settings: LavaSettings) {
    let possible_spawn_points = get_yx_mountain_tiles(world);
    let min = min(lava_settings.number_of_spawn_points, possible_spawn_points.len());
    for i in 0..min {
        let (y, x) = possible_spawn_points[i];
        let range = lava_settings.lava_flow_range.clone();
        flow_from(world, elevation_map, y, x, range);
    }
}

//for each x,y flow the lava to the lower neighbour
pub(crate) fn flow_from(
    world: &mut Vec<Vec<Tile>>,
    elevation_map: &Vec<Vec<f64>>,
    y: usize,
    x: usize,
    remaining_range: Range<usize>,
) -> usize {
    //println!("flowing from {},{} with range {}..{}", x,y, remaining_range.start, remaining_range.end);
    world[y][x].tile_type = TileType::Lava;
    if remaining_range.start == remaining_range.end {
        0
    } else {
        // if there is a neighbour with a lower height, flow to it
        let (lowest_neighbour_y, lowest_neighbour_x) = get_lowest_neighbour(elevation_map, y, x);
        flow_from(
            world,
            elevation_map,
            lowest_neighbour_y,
            lowest_neighbour_x,
            remaining_range.start..remaining_range.end - 1,
        )
        // if elevation_map[lowest_neighbour_y][lowest_neighbour_x] < elevation_map[y][x] {
        //     return flow_from(world, elevation_map, lowest_neighbour_y, lowest_neighbour_x, remaining_range.start..remaining_range.end - 1);
        // }
        // else {
        //     return remaining_range.end - remaining_range.start;
        // }
    }
}

// return the coordinates of the lowest neighbour
pub(crate) fn get_lowest_neighbour(elevation_map: &Vec<Vec<f64>>, y: usize, x: usize) -> (usize, usize) {
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
pub(crate) fn get_yx_mountain_tiles(wordl: &mut Vec<Vec<Tile>>) -> Vec<(usize, usize)> {
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
