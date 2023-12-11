use nannou_core::math::{deg_to_rad, map_range};
use noise::{NoiseFn, Perlin};
use rand::Rng;
use robotics_lib::world::tile::Content;

use crate::generator::World;
use crate::utils::{Coordinate, get_random_seeded_noise};

pub(crate) struct Fire {
    pub(crate) points: Vec<Coordinate>,
    pub(crate) noise: Perlin,
    pub(crate) border_points: Vec<Coordinate>,
    pub(crate) radius: f32,
    pub(crate) variation: f32,
    pub(crate) center: Coordinate,
}

impl Fire {
    pub fn default(size: usize, radius: f32, variation: f32) -> Self {
        let mut fire = Fire::new();

        // set the radius
        fire.radius = radius;

        // set the variation
        fire.variation = variation;

        // set the noise function
        fire.noise = get_random_seeded_noise();

        // get the center of the fire
        let mut rng = rand::thread_rng();
        let max_radius = (radius.ceil() + variation.ceil()) as usize;
        let x = rng.gen_range(max_radius..size - max_radius);
        let y = rng.gen_range(max_radius..size - max_radius);
        fire.center = Coordinate { row: y, col: x };

        // set boarder points
        fire.border_points =
            (0..=360).map(|i| { // Map over an array of integers from 0 to 360 to represent the degrees in a circle.
                // Convert each degree to radians.
                let radian = deg_to_rad(i as f32);
                // Get the sine of the radian to find the x co-ordinate of this point of the circle
                // and multiply it by the radius.
                let xoff = (radian.cos() + 1.0) as f64;
                let yoff = (radian.sin() + 1.0) as f64;

                let r = map_range(fire.noise.get([xoff, yoff]), 0.0, 1.0, radius * (1. - variation), radius * (1. + variation));
                let relative_x = radian.cos() * r;
                let relative_y = radian.sin() * r;

                let border_x = (fire.center.col as f32 + relative_x) as usize;
                let border_y = (fire.center.row as f32 + relative_y) as usize;

                Coordinate {
                    row: border_y,
                    col: border_x,
                }
            }).collect();

        let (min_row, min_col, max_row, max_col) = fire.get_extreme_points();

        fire.spread_fire(min_row, min_col, max_row, max_col);

        fire
    }

    fn get_extreme_points(&self) -> (usize, usize, usize, usize) {
        let min_col = self.border_points.iter().map(|f| f.col).min().unwrap();
        let max_col = self.border_points.iter().map(|f| f.col).max().unwrap();
        let min_row = self.border_points.iter().map(|f| f.row).min().unwrap();
        let max_row = self.border_points.iter().map(|f| f.row).max().unwrap();

        (min_row, min_col, max_row, max_col)
    }

    pub(crate) fn new() -> Self {
        Fire {
            points: vec![],
            noise: Perlin::new(0),
            border_points: vec![],
            radius: 0.0,
            variation: 0.0,
            center: Coordinate { row: 0, col: 0 },
        }
    }

    fn spread_fire(&mut self, upper_border: usize, left_border: usize, lower_border: usize, righter_border: usize) {
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
        stack.push(Coordinate { row: self.center.row - upper_border, col: self.center.col - left_border });
        // mark center as visited
        visited[self.center.row - upper_border][self.center.col - left_border] = true;
        while !stack.is_empty()
        {
            if let Some(current) = stack.pop()
            {
                let x = current.col;
                let y = current.row;

                // upper left
                if x > 0 && y > 0 && !visited[y - 1][x - 1] && !visited[y - 1][x] && !visited[y][x - 1] {
                    visited[y - 1][x - 1] = true;
                    stack.push(Coordinate { row: y - 1, col: x - 1 });
                }
                // upper center
                if y > 0 && !visited[y - 1][x] {
                    visited[y - 1][x] = true;
                    stack.push(Coordinate { row: y - 1, col: x });
                }
                // upper right
                if x < rect_width - 1 && y > 0 && !visited[y - 1][x + 1] && !visited[y - 1][x] && !visited[y][x + 1] {
                    visited[y - 1][x + 1] = true;
                    stack.push(Coordinate { row: y - 1, col: x + 1 });
                }
                // right center
                if x < rect_width - 1 && !visited[y][x + 1] {
                    visited[y][x + 1] = true;
                    stack.push(Coordinate { row: y, col: x + 1 });
                }
                // lower right
                if x < rect_width - 1 && y < rect_height - 1 && !visited[y + 1][x + 1] && !visited[y + 1][x] && !visited[y][x + 1] {
                    visited[y + 1][x + 1] = true;
                    stack.push(Coordinate { row: y + 1, col: x + 1 });
                }
                // lower center
                if y < rect_height - 1 && !visited[y + 1][x] {
                    visited[y + 1][x] = true;
                    stack.push(Coordinate { row: y + 1, col: x });
                }
                // lower left
                if x > 0 && y < rect_height - 1 && !visited[y + 1][x - 1] && !visited[y + 1][x] && !visited[y][x - 1] {
                    visited[y + 1][x - 1] = true;
                    stack.push(Coordinate { row: y + 1, col: x - 1 });
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
                    self.points.push(Coordinate { row: y + upper_border, col: x + left_border });
                }
            }
        }
    }
}

pub fn spawn_fires(world: &mut World) {
    let fire = Fire::default(world.len(), 40., 0.1);
    for point in fire.points {
        world[point.row][point.col].content = Content::Fire;
    }
}

