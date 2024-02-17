use std::ops::Mul;

use nannou_core::prelude::Pow;
use robotics_lib::world::tile::{Content, Tile};
use serde::{Deserialize, Serialize};

use crate::content::blob::{spawn_blob, Blob, BlobSettings, BlobTrait};
use crate::generator::TileMatrix;

#[derive(Serialize, Deserialize, Clone)]
pub struct TreeSettings {
    settings: BlobSettings,
}

pub struct Tree {
    inner: Blob,
}

impl BlobTrait for Tree {
    fn limit_on_proper_tile(&mut self, world: &[Vec<Tile>], content: &Content) {
        self.inner.limit_on_proper_tile(world, content);
    }

    fn get_extreme_points(&self) -> (usize, usize, usize, usize) {
        self.inner.get_extreme_points()
    }

    fn default(world: &[Vec<Tile>], size: usize, radius: f32, variation: f32, content: &Content) -> Self {
        Tree {
            inner: Blob::default(world, size, radius, variation, content),
        }
    }

    fn new() -> Self {
        Tree { inner: Blob::new() }
    }

    fn spread_blob(&mut self, upper_border: usize, left_border: usize, lower_border: usize, righter_border: usize) {
        self.inner.spread_blob(upper_border, left_border, lower_border, righter_border);

        // remove with a certain probability
        self.inner.points.retain(|_| rand::random::<f32>() > 0.1);
    }
}

impl TreeSettings {
    pub fn default(size: usize) -> Self {
        let radius_range = 1.0..(size as f32 / 50.0).min(4.0);
        let n_blob = (size as f32 * 0.1) as usize..(size as f32 * 0.15) as usize;
        let n_tiles = 1..(radius_range.end.ceil().mul(2.0).pow(2) as usize) * n_blob.end;
        TreeSettings {
            settings: BlobSettings {
                radius_range,
                n_blob,
                n_tiles,
            },
        }
    }
}

pub fn spawn_tree(world: &mut TileMatrix, settings: &mut TreeSettings) {
    spawn_blob(world, &mut settings.settings, Content::Tree(0))
}
