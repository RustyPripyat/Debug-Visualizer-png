use std::ops::Range;

use crate::generator::TileMatrix;
use nannou_core::math::{deg_to_rad, map_range};
use noise::{NoiseFn, Perlin};
use rand::Rng;
use robotics_lib::world::tile::{Content, Tile};
use serde::{Deserialize, Serialize};

use crate::utils::{get_random_seeded_noise, Coordinate};
#[derive(Serialize, Deserialize, Clone)]
pub struct BlobSettings {
    pub(crate) n_tiles: Range<usize>,
    pub(crate) radius_range: Range<f32>,
    pub(crate) n_blob: Range<usize>,
}

pub(crate) struct Blob {
    pub(crate) points: Vec<Coordinate>,
    pub(crate) noise: Perlin,
    pub(crate) border_points: Vec<Coordinate>,
    pub(crate) radius: f32,
    pub(crate) variation: f32,
    pub(crate) center: Coordinate,
}

pub(crate) trait BlobTrait {
    fn limit_on_proper_tile(&mut self, world: &[Vec<Tile>], content: &Content);
    fn get_extreme_points(&self) -> (usize, usize, usize, usize);
    fn default(world: &[Vec<Tile>], size: usize, radius: f32, variation: f32, content: &Content) -> Self;

    fn new() -> Self;
    fn spread_blob(&mut self, upper_border: usize, left_border: usize, lower_border: usize, righter_border: usize);
}

impl BlobTrait for Blob {
    fn limit_on_proper_tile(&mut self, world: &[Vec<Tile>], content: &Content) {
        let mut i = 0;
        while i < self.points.len() {
            let point = self.points[i];

            if !world[point.row][point.col].tile_type.properties().can_hold(&content) || world[point.row][point.col].content != Content::None {
                // Remove the point from the blob
                self.points.swap_remove(i);
            } else {
                // Move to the next index
                i += 1;
            }
        }
    }

    fn get_extreme_points(&self) -> (usize, usize, usize, usize) {
        let min_col = self.border_points.iter().map(|f| f.col).min().unwrap();
        let max_col = self.border_points.iter().map(|f| f.col).max().unwrap();
        let min_row = self.border_points.iter().map(|f| f.row).min().unwrap();
        let max_row = self.border_points.iter().map(|f| f.row).max().unwrap();

        (min_row, min_col, max_row, max_col)
    }

    fn default(world: &[Vec<Tile>], size: usize, radius: f32, variation: f32, content: &Content) -> Self {
        let mut blob = Blob::new();

        // set the radius
        blob.radius = radius;

        // set the variation
        blob.variation = variation;

        // set the noise function
        blob.noise = get_random_seeded_noise();

        // get the center of the blob
        let mut rng = rand::thread_rng();
        let max_radius = (radius.ceil() + variation.ceil()) as usize;
        let x = rng.gen_range(max_radius..size - max_radius);
        let y = rng.gen_range(max_radius..size - max_radius);
        blob.center = Coordinate { row: y, col: x };

        // set boarder points
        blob.border_points = (0..=360)
            .map(|i| {
                // Map over an array of integers from 0 to 360 to represent the degrees in a circle.
                // Convert each degree to radians.
                let radian = deg_to_rad(i as f32);
                // Get the sine of the radian to find the x co-ordinate of this point of the circle
                // and multiply it by the radius.
                let xoff = (radian.cos() + 1.0) as f64;
                let yoff = (radian.sin() + 1.0) as f64;

                let r = map_range(blob.noise.get([xoff, yoff]), 0.0, 1.0, radius * (1. - variation), radius * (1. + variation));
                let relative_x = radian.cos() * r;
                let relative_y = radian.sin() * r;

                let border_x = (blob.center.col as f32 + relative_x) as usize;
                let border_y = (blob.center.row as f32 + relative_y) as usize;

                Coordinate {
                    row: border_y,
                    col: border_x,
                }
            })
            .collect();

        let (min_row, min_col, max_row, max_col) = blob.get_extreme_points();

        blob.spread_blob(min_row, min_col, max_row, max_col);

        blob.limit_on_proper_tile(world, content);

        blob
    }

    fn new() -> Self {
        Blob {
            points: vec![],
            noise: Perlin::new(0),
            border_points: vec![],
            radius: 0.0,
            variation: 0.0,
            center: Coordinate { row: 0, col: 0 },
        }
    }

    // a function to spread from the center to the border points of the blob
    fn spread_blob(&mut self, upper_border: usize, left_border: usize, lower_border: usize, righter_border: usize) {
        let rect_width = righter_border - left_border + 1;
        let rect_height = lower_border - upper_border + 1;
        //marking `border_points` as already visited
        let mut visited: Vec<Vec<bool>> = vec![vec![false; rect_width]; rect_height];

        // set as contained and visited all the border points
        for point in self.border_points.clone() {
            let x = point.col - left_border;
            let y = point.row - upper_border;
            visited[y][x] = true;
        }

        let mut stack: Vec<Coordinate> = Vec::new();
        stack.push(Coordinate {
            row: self.center.row - upper_border,
            col: self.center.col - left_border,
        });
        // mark center as visited
        visited[self.center.row - upper_border][self.center.col - left_border] = true;
        while !stack.is_empty() {
            if let Some(current) = stack.pop() {
                let x = current.col;
                let y = current.row;

                // upper left
                if x > 0 && y > 0 && !visited[y - 1][x - 1] && !visited[y - 1][x] && !visited[y][x - 1] {
                    visited[y - 1][x - 1] = true;
                    stack.push(Coordinate {
                        row: y - 1,
                        col: x - 1,
                    });
                }
                // upper center
                if y > 0 && !visited[y - 1][x] {
                    visited[y - 1][x] = true;
                    stack.push(Coordinate { row: y - 1, col: x });
                }
                // upper right
                if x < rect_width - 1 && y > 0 && !visited[y - 1][x + 1] && !visited[y - 1][x] && !visited[y][x + 1] {
                    visited[y - 1][x + 1] = true;
                    stack.push(Coordinate {
                        row: y - 1,
                        col: x + 1,
                    });
                }
                // right center
                if x < rect_width - 1 && !visited[y][x + 1] {
                    visited[y][x + 1] = true;
                    stack.push(Coordinate { row: y, col: x + 1 });
                }
                // lower right
                if x < rect_width - 1 && y < rect_height - 1 && !visited[y + 1][x + 1] && !visited[y + 1][x] && !visited[y][x + 1] {
                    visited[y + 1][x + 1] = true;
                    stack.push(Coordinate {
                        row: y + 1,
                        col: x + 1,
                    });
                }
                // lower center
                if y < rect_height - 1 && !visited[y + 1][x] {
                    visited[y + 1][x] = true;
                    stack.push(Coordinate { row: y + 1, col: x });
                }
                // lower left
                if x > 0 && y < rect_height - 1 && !visited[y + 1][x - 1] && !visited[y + 1][x] && !visited[y][x - 1] {
                    visited[y + 1][x - 1] = true;
                    stack.push(Coordinate {
                        row: y + 1,
                        col: x - 1,
                    });
                }
                // left center
                if x > 0 && !visited[y][x - 1] {
                    visited[y][x - 1] = true;
                    stack.push(Coordinate { row: y, col: x - 1 });
                }
            }
        }

        for (y, row) in visited.iter().enumerate() {
            for (x, visited) in row.iter().enumerate() {
                if *visited {
                    self.points.push(Coordinate {
                        row: y + upper_border,
                        col: x + left_border,
                    });
                }
            }
        }
    }
}

pub(crate) fn spawn_blob(world: &mut TileMatrix, settings: &mut BlobSettings, content: Content) {
    // checks if settings are valid
    if let Err(msg) = errors(settings) {
        panic!("{}", msg);
    };

    // generate blobs and place them in the world
    loop {
        // Generate random for variation
        let mut rng = rand::thread_rng();
        let variation = rng.gen_range(0.075..0.125);
        let radius = rng.gen_range(settings.radius_range.start..settings.radius_range.end);
        let blob = Blob::default(world.as_slice(), world.len(), radius, variation, &content);

        // checks before placing the blob
        if blob.points.len() > settings.n_tiles.end || settings.n_blob.end < 1 {
            break;
        }

        // Decrease the counter of total tiles
        settings.n_tiles.end -= blob.points.len();
        // Decrease the blob counter
        settings.n_blob.end -= 1;

        // Place tiles of the blob
        for point in blob.points {
            world[point.row][point.col].content = content.clone();
        }
    }
}

fn errors(settings: &BlobSettings) -> Result<(), String> {
    if settings.radius_range.start.floor() as usize * settings.n_blob.start > settings.n_tiles.end {
        // the minimum number of tiles that could be generated would be higher than the maximum number of tiles provided
        Err(format!(
            r#"n_tiles.end: {} is too small for the given radius_range.start: {} and n_blob.start: {}.
The minimum number of tiles, that could be generated, would be higher than the maximum number of tiles provided."#,
            settings.n_tiles.end, settings.radius_range.start, settings.n_blob.start
        ))
    } else if settings.radius_range.end.ceil() as usize * settings.n_blob.end < settings.n_tiles.start {
        // the maximum number of tiles that could be generated would be lower than the minimum number of tiles provided
        Err(format!(
            r#"n_tiles.start: {} is too small for the given radius_range.end: {} and n_blob.end: {}.
The maximum number of tiles that could be generated would be lower than the minimum number of tiles provided"#,
            settings.n_tiles.start, settings.radius_range.end, settings.n_blob.end
        ))
    } else {
        Ok(())
    }
}
