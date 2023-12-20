use std::ops::Mul;

use nannou_core::prelude::Pow;
use robotics_lib::world::tile::{Content, Tile};

use crate::content::blob::{Blob, BlobSettings, BlobTrait, spawn_blob};

pub struct FireSettings {
    settings: BlobSettings,
}

pub struct Fire {
    inner: Blob,
}

impl BlobTrait for Fire {
    fn limit_on_proper_tile(&mut self, world: &[Vec<Tile>], content: &Content) {
        self.inner.limit_on_proper_tile(world, content);
    }

    fn get_extreme_points(&self) -> (usize, usize, usize, usize) {
        self.inner.get_extreme_points()
    }

    fn default(world: &[Vec<Tile>], size: usize, radius: f32, variation: f32, content: &Content) -> Self {
        Fire {
            inner: Blob::default(world, size, radius, variation, content),
        }
    }

    fn new() -> Self {
        Fire {
            inner: Blob::new(),
        }
    }

    fn spread_blob(&mut self, upper_border: usize, left_border: usize, lower_border: usize, righter_border: usize) {
        self.inner.spread_blob(upper_border, left_border, lower_border, righter_border);
    }

}

impl FireSettings {
    pub fn default(size: usize) -> Self {
        let radius_range = 5.0..size as f32 / 100.0;
        let n_blob = 1..size / 100;
        let n_tiles = 1..(radius_range.end.ceil().mul(2.0).pow(2) as usize) * n_blob.end;
        FireSettings {
            settings: BlobSettings {
                radius_range,
                n_blob,
                n_tiles,
            }
        }
    }
}

pub fn spawn_fire(world: &mut Vec<Vec<Tile>>, settings: &mut FireSettings) {
    spawn_blob(world, &mut settings.settings, Content::Fire)
}


