use std::sync::{Arc, Mutex};

use chrono::Utc;
use image::{Rgb, RgbImage};
use image::ImageFormat;
use imageproc::rect::Rect;
use rayon::prelude::*;
use robotics_lib::world::tile::*;

fn create_image_from_tiles(tiles: &[Vec<Tile>], bot_position: (usize, usize), tile_size: usize) -> RgbImage {
    let width = tiles[0].len() * tile_size;
    let height = tiles.len() * tile_size;

    let img = Arc::new(Mutex::new(RgbImage::new(width as u32, height as u32)));

    const COLOR_DEEP_WATER: Rgb<u8> = Rgb([5, 25, 90]);         // DeepWater (Blu scuro)
    const COLOR_SHALLOW_WATER: Rgb<u8> = Rgb([45, 100, 160]);   // ShallowWater (Blu chiaro)
    const COLOR_SAND: Rgb<u8> = Rgb([240, 230, 140]);           // Sand (Giallo sabbia)
    const COLOR_GRASS: Rgb<u8> = Rgb([50, 205, 50]);            // Grass (Verde prato)
    const COLOR_STREET: Rgb<u8> = Rgb([100, 100, 100]);         // Street (Grigio asfalto)
    const COLOR_HILL: Rgb<u8> = Rgb([174, 105, 38]);            // Hill (Marrone)
    const COLOR_MOUNTAIN: Rgb<u8> = Rgb([160, 160, 160]);       // Mountain (Grigio montagna)
    const COLOR_SNOW: Rgb<u8> = Rgb([255, 255, 255]);           // Snow (Bianco)
    const COLOR_LAVA: Rgb<u8> = Rgb([255, 140, 0]);             // Lava (Arancione acceso)
    const COLOR_GRAY: Rgb<u8> = Rgb([128, 128, 128]);           // Gray (Grigio)
    const COLOR_BLACK: Rgb<u8> = Rgb([0, 0, 0]);                // Black (Nero)

    let start = Utc::now();
    println!("start loop");

    // Parallelize the loop using par_iter()
    tiles.par_iter().enumerate().for_each(|(y, row)| {
        row.par_iter().enumerate().for_each(|(x, tile)| {
            let color = match tile.tile_type {
                TileType::DeepWater => COLOR_DEEP_WATER,
                TileType::ShallowWater => COLOR_SHALLOW_WATER,
                TileType::Sand => COLOR_SAND,
                TileType::Grass => COLOR_GRASS,
                TileType::Street => COLOR_STREET,
                TileType::Hill => COLOR_HILL,
                TileType::Mountain => COLOR_MOUNTAIN,
                TileType::Snow => COLOR_SNOW,
                TileType::Lava => COLOR_LAVA,
                _ => COLOR_BLACK,
            };

            // Inside the inner loop
            let rect = Rect::at(x as i32 * tile_size as i32, y as i32 * tile_size as i32)
                .of_size(tile_size as u32, tile_size as u32);

            // Lock the Mutex to get a mutable reference to the image
            let mut img_ref = img.lock().unwrap();
            imageproc::drawing::draw_filled_rect_mut(&mut *img_ref, rect, color.into());
        });
    });
    let t = (Utc::now() - start);
    print!("finish loop {}", t.num_milliseconds());
    let (bot_x, bot_y) = bot_position;
    let bot_color = Rgb([213, 213, 213]);


    for dy in 0..tile_size {
        for dx in 0..tile_size {
            img.lock().unwrap().put_pixel(
                (bot_x * tile_size + dx) as u32,
                (bot_y * tile_size + dy) as u32,
                bot_color,
            );
        }
    }

    // Extract the inner RgbImage from the Arc<Mutex<RgbImage>>
    Arc::try_unwrap(img)
        .ok()
        .unwrap()
        .into_inner()
        .unwrap()
}

pub fn save_world_image(tiles: &[Vec<Tile>], bot_position: (usize, usize), file_name: &str) {
    let img = create_image_from_tiles(tiles, bot_position, 10);

    if let Err(e) = img.save_with_format(file_name, ImageFormat::Png) {
        println!("Error saving the image: {}", e);
    }
}