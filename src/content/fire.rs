use std::ops::Mul;

use nannou_core::prelude::Pow;
use robotics_lib::world::tile::{Content, Tile};
use serde::{Deserialize, Serialize};

use crate::content::blob::{spawn_blob, Blob, BlobSettings, BlobTrait};
use crate::generator::TileMatrix;

#[derive(Serialize, Deserialize, Clone)]
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
        Fire { inner: Blob::new() }
    }

    fn spread_blob(&mut self, upper_border: usize, left_border: usize, lower_border: usize, righter_border: usize) {
        self.inner.spread_blob(upper_border, left_border, lower_border, righter_border);
    }
}

impl FireSettings {
    /// Creates a new instance of `FireSettings` with optimal settings based on the provided parameters
    ///
    /// This method initializes a `FireSettings` instance with default parameters, including the size,
    /// radius, and variation of the fire. It generates a fire centered randomly within the specified
    /// `size`, setting the radius and variation, and determining the border points of the fire.
    ///
    /// # Arguments
    ///
    /// * `size` - The size of the world side.
    ///
    /// # Returns
    ///
    /// A new instance of `FireSettings` initialized with optimal settings based on the provided arguments.
    ///
    /// # Examples
    ///
    /// ```
    /// use exclusion_zone::content::fire::FireSettings;
    ///
    /// let size = 1000;
    /// let default_fire = FireSettings::default(size);
    /// ```
    pub fn default(size: usize) -> Self {
        let radius_range = size as f32 / 100.0..size as f32 / 50.0;
        let n_blob = size / 100..size / 50;
        let n_tiles = 1..(radius_range.end.ceil().mul(2.0).pow(2) as usize) * n_blob.end;
        FireSettings {
            settings: BlobSettings {
                radius_range,
                n_blob,
                n_tiles,
            },
        }
    }
}

pub fn spawn_fire(world: &mut TileMatrix, settings: &mut FireSettings) {
    spawn_blob(world, &mut settings.settings, Content::Fire)
}
