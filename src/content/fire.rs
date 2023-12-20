use std::ops::{Mul, Range};

use nannou_core::math::{deg_to_rad, map_range};
use nannou_core::prelude::Pow;
use noise::{NoiseFn, Perlin};
use rand::Rng;
use robotics_lib::world::tile::{Content, Tile};

use crate::utils::{Coordinate, get_random_seeded_noise};

#[derive(Clone)]
pub struct FireSettings {
    pub(crate) num_fire_tiles: Range<usize>,
    pub(crate) radius_range: Range<f32>,
    pub(crate) num_of_blaze: Range<usize>,
}

pub(crate) struct Blaze {
    pub(crate) points: Vec<Coordinate>,
    pub(crate) noise: Perlin,
    pub(crate) border_points: Vec<Coordinate>,
    pub(crate) radius: f32,
    pub(crate) variation: f32,
    pub(crate) center: Coordinate,
}


impl FireSettings {
    pub fn default(size: usize) -> Self {
        let radius_range = 5.0..size as f32 / 100.0;
        let num_of_blaze = 1..size / 100;
        let num_fire_tiles = 1..(radius_range.end.ceil().mul(2.0).pow(2) as usize) * num_of_blaze.end;
        FireSettings {
            radius_range,
            num_of_blaze,
            num_fire_tiles,
        }
    }
}

impl Blaze {
    pub fn default(world: &[Vec<Tile>], size: usize, radius: f32, variation: f32) -> Self {
        let mut blaze = Blaze::new();

        // set the radius
        blaze.radius = radius;

        // set the variation
        blaze.variation = variation;

        // set the noise function
        blaze.noise = get_random_seeded_noise();

        // get the center of the fire
        let mut rng = rand::thread_rng();
        let max_radius = (radius.ceil() + variation.ceil()) as usize;
        let x = rng.gen_range(max_radius..size - max_radius);
        let y = rng.gen_range(max_radius..size - max_radius);
        blaze.center = Coordinate { row: y, col: x };

        // set boarder points
        blaze.border_points =
            (0..=360).map(|i| { // Map over an array of integers from 0 to 360 to represent the degrees in a circle.
                // Convert each degree to radians.
                let radian = deg_to_rad(i as f32);
                // Get the sine of the radian to find the x co-ordinate of this point of the circle
                // and multiply it by the radius.
                let xoff = (radian.cos() + 1.0) as f64;
                let yoff = (radian.sin() + 1.0) as f64;

                let r = map_range(blaze.noise.get([xoff, yoff]), 0.0, 1.0, radius * (1. - variation), radius * (1. + variation));
                let relative_x = radian.cos() * r;
                let relative_y = radian.sin() * r;

                let border_x = (blaze.center.col as f32 + relative_x) as usize;
                let border_y = (blaze.center.row as f32 + relative_y) as usize;

                Coordinate {
                    row: border_y,
                    col: border_x,
                }
            }).collect();

        let (min_row, min_col, max_row, max_col) = blaze.get_extreme_points();

        blaze.spread_fire(min_row, min_col, max_row, max_col);

        blaze.limit_on_proper_tile(world);

        blaze
    }

    fn limit_on_proper_tile(&mut self, world: &[Vec<Tile>]) {
        let mut i = 0;
        while i < self.points.len() {
            let point = self.points[i];

            if !world[point.row][point.col].tile_type.properties().can_hold(&Content::Fire)
                || world[point.row][point.col].content != Content::None
            {
                // Remove the point from the blaze
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

    pub(crate) fn new() -> Self {
        Blaze {
            points: vec![],
            noise: Perlin::new(0),
            border_points: vec![],
            radius: 0.0,
            variation: 0.0,
            center: Coordinate { row: 0, col: 0 },
        }
    }

    // a function to spread the fire from the center to the border points of the blaze
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

pub fn spawn_fires(world: &mut Vec<Vec<Tile>>, fire_settings: &mut FireSettings) {

    // checks if fire settings are valid
    if let Err(msg) = errors(fire_settings) {
        panic!("{}", msg);
    };

    // generate blazes and place them in the world
    // while fs.num_fire_tiles.as_ref().unwrap().end > fs.num_fire_tiles.as_ref().unwrap().start
    //     && fs.num_of_blaze.as_ref().unwrap().end > fs.num_of_blaze.as_ref().unwrap().start
    loop {

        // Generate random for variation
        let mut rng = rand::thread_rng();
        let variation = rng.gen_range(0.075..0.125);
        let radius = rng.gen_range(fire_settings.radius_range.start..fire_settings.radius_range.end);
        let blaze = Blaze::default(world.as_slice(), world.len(), radius, variation);

        // checks before placing the blaze
       if blaze.points.len() > fire_settings.num_fire_tiles.end || fire_settings.num_of_blaze.end < 1 {
            break;
        }

        // Decrease the settings.num_fire_tiles.unwrap().endfire_settings.num_fire_tiles.end -= blaze.points.len();
        // Decrease the settings.num_of_blaze.unwrap().end
        fire_settings.num_of_blaze.end -= 1;


        // check to not exceed the number of blazes or the number of fire tiles
        // if fs.num_fire_tiles.as_ref().unwrap().end <= fs.num_fire_tiles.as_ref().unwrap().start ||
        //     fs.num_of_blaze.as_ref().unwrap().end <= fs.num_of_blaze.as_ref().unwrap().start
        // {
        //     break;
        // }

        // Place fires of the blaze
        for point in blaze.points {
            world[point.row][point.col].content = Content::Fire;
        }
    }
}

fn errors(fire_settings: &FireSettings) -> Result<(), String> {
    if fire_settings.radius_range.start.floor() as usize * fire_settings.num_of_blaze.start > fire_settings.num_fire_tiles.end {
        // the minimum number of fire tiles that could be generated would be higher than the maximum number of fire tiles provided
        Err(format!(r#"num_fire_tiles.end: {} is too small for the given radius_range.start:
                {} and num_of_blaze.start: {}.\nThe minimum number of fire tiles, that could be
                generated, would be higher than the maximum number of fire tiles provided."#,
                    fire_settings.num_fire_tiles.end, fire_settings.radius_range.start, fire_settings.num_of_blaze.start))
    } else if fire_settings.radius_range.end.ceil() as usize * fire_settings.num_of_blaze.end < fire_settings.num_fire_tiles.start {
        // the maximum number of fire tiles that could be generated would be lower than the minimum number of fire tiles provided
        Err(format!(r#"num_fire_tiles.start: {} is too small for the given radius_range.end:
                {} and num_of_blaze.end: {}.\nThe maximum number of fire tiles that could be
                generated would be lower than the minimum number of fire tiles provided"#,
                    fire_settings.num_fire_tiles.start, fire_settings.radius_range.end, fire_settings.num_of_blaze.end))
    } else {
        Ok(())
    }
}