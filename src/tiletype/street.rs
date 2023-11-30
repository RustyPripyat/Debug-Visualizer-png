use std::cmp::{min, Ordering};
use robotics_lib::world::tile::Tile;
use robotics_lib::world::tile::TileType;
use crate::generator::{LavaSettings};
use std::ops::Range;
use rand::seq::SliceRandom;

pub(crate) fn street_spawn(street_quantity:usize, elevation_map: Vec<Vec<f64>>,n_slice_side:usize,lower_threshold: f64){
    // get local maxima
    // stuff with Voronoi diagram
    // magically spawn street
}

fn get_local_maxima(elevation_map: Vec<Vec<f64>>,n_slice_side:usize,lower_threshold: f64)->Vec<(usize,usize)>{
    let local_maxima: Vec<(usize,usize)> = vec![];


    return local_maxima;
}

fn slice_vec_2d(input: Vec<Vec<f64>>, n: usize) -> Vec<Vec<Vec<f64>>> {
    let mut result = Vec::new();

    // Calculate the number of rows and columns in each slice
    let rows_per_slice = input.len() / n;
    let columns_per_slice = if n > 0 { input[0].len() } else { 0 };

    for i in 0..n {
        let start_row = i * rows_per_slice;
        let end_row = if i == n - 1 {
            input.len()
        } else {
            (i + 1) * rows_per_slice
        };

        let slice = input[start_row..end_row]
            .iter()
            .map(|row| row[..columns_per_slice].to_vec())
            .collect();

        result.push(slice);
    }

    result
}

// get the maximum value from a slice
fn get_maximum(slice:Vec<Vec<f64>>)->(usize,usize){
    slice.iter().enumerate().flat_map(|(row_index, inner)| {
        inner.iter().enumerate().map(move |(col_index, &value)| (row_index, col_index, value))
    })
        .max_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(row_index, col_index, _)| (row_index, col_index)).unwrap()
}



