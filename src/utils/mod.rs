use std::fs::File;
use std::io::{self, Read};

use noise::Perlin;
use rand::Rng;
use robotics_lib::world::tile::Content;
use serde::{Deserialize, Serialize};
use zstd::stream::copy_encode;
use zstd::stream::read::Decoder;

use crate::generator::{GenResult, WorldGenerator};
use crate::generator::TileMatrix;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coordinate {
    pub(crate) row: usize,
    pub(crate) col: usize,
}

pub(crate) struct Slice {
    pub(crate) start: Coordinate,
    pub(crate) end: Coordinate,
}

//slice a 2d vector into n_slice x n_slice slices (leaving the last slice with the remaining elements)
#[inline(always)]
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

#[allow(dead_code)]
pub(crate) fn distance(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    ((x1 as isize - x2 as isize).abs() + (y1 as isize - y2 as isize).abs()) as usize
}

#[inline(always)]
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

#[inline(always)]
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

#[inline(always)]
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

#[allow(dead_code)]
pub(crate) fn map_value_to_range(value: f64, from: std::ops::Range<f64>, to: std::ops::Range<f64>) -> f64 {
    let from_min = from.start;
    let from_max = from.end;
    let to_min = to.start;
    let to_max = to.end;

    (value - from_min) * (to_max - to_min) / (from_max - from_min) + to_min
}

#[inline(always)]
pub(crate) fn spawn_content_randomly(world: &mut TileMatrix, mut number_of_spawn_points: usize, content: Content) -> Vec<Coordinate> {
    let mut rng = rand::thread_rng();

    let mut spawn_points = Vec::new();

    while number_of_spawn_points > 0 {
        let c = Coordinate{ row: rng.gen_range(0..world.len()), col: rng.gen_range(0..world.len()) };

        if world[c.row][c.col].tile_type.properties().can_hold(&content) && world[c.row][c.col].content == Content::None {
            number_of_spawn_points -= 1;
            spawn_points.push(c);
        }
    }
    spawn_points
}

#[inline(always)]
pub(crate) fn get_random_seeded_noise() -> Perlin {
    // setting noise with random seed
    let mut rng = rand::thread_rng();
    Perlin::new(rng.gen())
}

#[derive(Serialize, Deserialize)]
pub(crate) struct SerializedWorld {
    pub(crate) world: GenResult,
    pub(crate) settings: WorldGenerator,
}

impl SerializedWorld {
    #[inline(always)]
    pub(crate) fn serialize(&self, file_path: &str, compression_level: i32) -> Result<(), String> {
        let serialized = match bincode::serialize(self) {
            Ok(r) => { r }
            Err(e) => {
                return Err(format!("{e}"));
            }
        };

        let mut file = match File::create(format!("{file_path}.zst")) {
            Ok(r) => { r }
            Err(e) => {
                return Err(format!("{e}"));
            }
        };

        match copy_encode(&*serialized, &mut file, compression_level) {
            Ok(r) => { r }
            Err(e) => {
                return Err(format!("{e}"));
            }
        };

        Ok(())
    }
    #[inline(always)]
    pub(crate) fn deserialize(file_path: &str) -> io::Result<Self> {
        let file = File::open(file_path)?;

        let mut buffer = Vec::new();
        let mut decoder = Decoder::new(file)?;
        decoder.read_to_end(&mut buffer)?;

        let deserialized: SerializedWorld = bincode::deserialize(&buffer)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Deserialization failed: {}", e)))?;

        Ok(deserialized)
    }
}

