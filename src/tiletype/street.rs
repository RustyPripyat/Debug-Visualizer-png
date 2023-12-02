use std::cmp::{max, Ordering};
use imageproc::contrast::threshold;

struct Coordinate {
    row: usize,
    col: usize,
}

struct Slice {
    start: Coordinate,
    end: Coordinate,
}

pub(crate) fn street_spawn(street_quantity:usize, elevation_map: &Vec<Vec<f64>>,n_slice_side:usize,lower_threshold: f64) -> Vec<(usize,usize)>{
    // get local maxima
    let local_maxima = get_local_maxima(&elevation_map, n_slice_side, lower_threshold);
    //combine_near_local_maxima(&elevation_map, &local_maxima, n_slice_side)
    //set combined maxima to black
    local_maxima

    // stuff with Voronoi diagram
    // magically spawn street
}

fn combine_near_local_maxima(
    elevation_map: &Vec<Vec<f64>>,
    local_maxima: &Vec<(usize, usize)>,
    num_of_slice: usize,
) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    // slice 2d vector but with initial offset
    let offset = (elevation_map.len() / num_of_slice) / 2;

    for &(row, col) in local_maxima {
        let mut combined = false;
        for &existing in &result {
            if distance(row, col, existing.0, existing.1) < offset {
                // Combine if they are close
                let (new_row, new_col) = if elevation_map[row][col] > elevation_map[existing.0][existing.1] {
                    (row + offset, col + offset)
                } else {
                    (existing.0 + offset, existing.1 + offset)
                };

                result.retain(|&x| x != existing);
                result.push((new_row, new_col));
                combined = true;
                break;
            }
        }
        if !combined {
            // If not combined with any existing, add as is
            result.push((row + offset, col + offset));
        }
    }

    result
}

fn distance(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    ((x1 as isize - x2 as isize).abs() + (y1 as isize - y2 as isize).abs()) as usize
}

fn get_local_maxima(elevation_map: &Vec<Vec<f64>>, n_slice_side: usize, lower_threshold: f64) -> Vec<(usize, usize)> {
    let mut local_maxima: Vec<(usize, usize)> = Vec::new();

    for slice in slice_vec_2d(elevation_map, n_slice_side) {
        let mut local_maximum_point = (lower_threshold,usize::MAX,usize::MAX);
        for row_index in slice.start.row..slice.end.row {
            for col_index in slice.start.col..slice.end.col {
                if elevation_map[row_index][col_index] > local_maximum_point.0 {
                    local_maximum_point = (elevation_map[row_index][col_index], row_index, col_index);
                }
            }
        }
        local_maxima.push((local_maximum_point.1, local_maximum_point.2));
    }
    local_maxima
}

fn slice_vec_2d(input: &Vec<Vec<f64>>, n_slice: usize) -> Vec<Slice> {
    // Calculate the number of rows and columns in each slice
    let qnt_per_slice = input.len() / n_slice;
    let mut slice: Vec<Slice> = Vec::new();

    for y in 0..n_slice {
        let start_row = y * qnt_per_slice;
        let end_row = if (start_row + qnt_per_slice) < input.len() {
            start_row + qnt_per_slice - 1
        } else {
            input.len()
        };

        for x in 0..n_slice {
            let start_col = x * qnt_per_slice;
            let end_col = if (start_col + qnt_per_slice) < input.len() {
                start_col + qnt_per_slice - 1
            } else {
                input.len()
            };

            slice.push(Slice{
                start: Coordinate{row: start_row, col: start_col},
                end: Coordinate{row: end_row, col: end_col},
            });
        }

    }

    slice
}

// get the maximum value from a slice
fn get_maximum(slice: &Vec<Vec<f64>>)->(usize,usize){
    slice.iter().enumerate().flat_map(|(row_index, inner)| {
        inner.iter().enumerate().map(move |(col_index, &value)| (row_index, col_index, value))
    })
        .max_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(row_index, col_index, _)| (row_index, col_index)).unwrap()
}



