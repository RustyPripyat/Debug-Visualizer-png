use crate::content::bank::BankSettings;
use rand::Rng;
use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::World;

pub(crate) struct Coordinate {
    pub(crate) row: usize,
    pub(crate) col: usize,
}

pub(crate) struct Slice {
    pub(crate) start: Coordinate,
    pub(crate) end: Coordinate,
}

//slice a 2d vector into n_slice x n_slice slices (leaving the last slice with the remaining elements)
pub(crate) fn slice_vec_2d(input: &[Vec<f64>], n_slice: usize) -> Vec<Slice> {
    // Calculate the number of rows and columns in each slice
    let qnt_per_slice = input.len() / n_slice;
    let mut slice: Vec<Slice> = Vec::new();

    for y in 0..n_slice {
        let start_row = y * qnt_per_slice;
        let end_row = if (start_row + qnt_per_slice) < input.len() { start_row + qnt_per_slice - 1 } else { input.len() };

        for x in 0..n_slice {
            let start_col = x * qnt_per_slice;
            let end_col = if (start_col + qnt_per_slice) < input.len() { start_col + qnt_per_slice - 1 } else { input.len() };

            slice.push(Slice {
                start: Coordinate {
                    row: start_row,
                    col: start_col,
                },
                end: Coordinate {
                    row: end_row,
                    col: end_col,
                },
            });
        }
    }

    slice
}

pub(crate) fn distance(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    ((x1 as isize - x2 as isize).abs() + (y1 as isize - y2 as isize).abs()) as usize
}

pub(crate) fn percentage(target_percentage: f64, min: f64, max: f64) -> f64 {
    // MappedValue= [(x-a)/(b-a)]⋅(d−c)+c
    let x = target_percentage;
    // let a = 0.0;
    let b = 100.0;
    let c = min;
    let d = max;
    // ((x - a) / (b - a)) * (d - c) + c
    (x / b) * (d - c) + c //simplified a = 0
}

pub(crate) fn find_min_value(matrix: &Vec<Vec<f64>>) -> Option<f64> {
    // Ensure the matrix is not empty
    if matrix.is_empty() || matrix[0].is_empty() {
        return None;
    }

    let mut min_value = matrix[0][0];

    for row in matrix {
        for &value in row {
            if value < min_value {
                min_value = value;
            }
        }
    }

    Some(min_value)
}

pub(crate) fn find_max_value(matix: &Vec<Vec<f64>>) -> Option<f64> {
    // Ensure the matrix is not empty
    if matix.is_empty() || matix[0].is_empty() {
        return None;
    }

    let mut max_value = matix[0][0];

    for row in matix {
        for &value in row {
            if value > max_value {
                max_value = value;
            }
        }
    }

    Some(max_value)
}

pub(crate) fn map_value_to_range(value: f64, from: std::ops::Range<f64>, to: std::ops::Range<f64>) -> f64 {
    let from_min = from.start;
    let from_max = from.end;
    let to_min = to.start;
    let to_max = to.end;

    (value - from_min) * (to_max - to_min) / (from_max - from_min) + to_min
}

pub(crate) fn spawn_content_randomly(world: &mut Vec<Vec<Tile>>, mut number_of_spawn_points: usize, content: Content) -> Vec<(usize, usize)> {
    let mut rng = rand::thread_rng();

    let mut spawn_points = Vec::new();

    while number_of_spawn_points > 0 {
        let y = rng.gen_range(0..world.len());
        let x = rng.gen_range(0..world.len());

        if world[y][x].tile_type.properties().can_hold(&content) && world[y][x].content == Content::None {
            number_of_spawn_points -= 1;
            spawn_points.push((y, x));
        }
    }
    spawn_points
}
