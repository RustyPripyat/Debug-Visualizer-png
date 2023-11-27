use std::process::Command;
use image::{Rgb, RgbImage};
use image::DynamicImage;
use image::ImageFormat;
use imageproc::rect::Rect;
use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use robotics_lib::world::tile::*;

fn create_image_from_tiles(tiles: &[Vec<Tile>], bot_position: (usize, usize)) -> RgbImage {
    const TILE_SIZE: u32 = 10;
    let WIDTH: u32 = tiles[0].len() as u32 * TILE_SIZE;
    let HEIGHT: u32 = tiles.len() as u32 * TILE_SIZE;

    let img = Arc::new(Mutex::new(RgbImage::new(WIDTH, HEIGHT)));

    const COLOR_DEEP_WATER: Rgb<u8> = Rgb([5, 25, 90]);         // DeepWater (Blu scuro)
    const COLOR_SHALLOW_WATER: Rgb<u8> = Rgb([45, 100, 160]);   // ShallowWater (Blu chiaro)
    const COLOR_SAND: Rgb<u8> = Rgb([240, 230, 140]);           // Sand (Giallo sabbia)
    const COLOR_GRASS: Rgb<u8> = Rgb([50, 205, 50]);            // Grass (Verde prato)
    const COLOR_STREET: Rgb<u8> = Rgb([100, 100, 100]);         // Street (Grigio asfalto)
    const COLOR_HILL: Rgb<u8> = Rgb([174, 105, 38]);            // Hill (Marrone)
    const COLOR_MOUNTAIN: Rgb<u8> = Rgb([160, 160, 160]);       // Mountain (Grigio montagna)
    const COLOR_SNOW: Rgb<u8> = Rgb([255, 255, 255]);           // Snow (Bianco)
    const COLOR_LAVA: Rgb<u8> = Rgb([255, 140, 0]);             // Lava (Arancione acceso)

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
            };

            // Inside the inner loop
            let rect = Rect::at(x as i32 * TILE_SIZE as i32, y as i32 * TILE_SIZE as i32)
                .of_size(TILE_SIZE, TILE_SIZE);

            // Lock the Mutex to get a mutable reference to the image
            let mut img_ref = img.lock().unwrap();
            imageproc::drawing::draw_filled_rect_mut(&mut *img_ref, rect, color.into());
        });
    });

    // Draw the symbol of the bot on its position
    let (bot_x, bot_y) = bot_position;
    let bot_color = Rgb([213, 213, 213]);



    for dy in 0..TILE_SIZE {
        for dx in 0..TILE_SIZE {
            img.lock().unwrap().put_pixel(
                (bot_x as u32 * TILE_SIZE + dx),
                (bot_y as u32 * TILE_SIZE + dy),
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
    let img: DynamicImage = create_image_from_tiles(tiles, bot_position).into();

    if let Err(e) = img.save_with_format(file_name, ImageFormat::Png) {
        println!("Error saving the image: {}", e);
    } else {
        println!("Image saved successfully as {}", file_name);
    }

    // letteralmente apre imv con l'immagine come argomento.
    // imv è un visualizzatore di immagini, cambia il comando
    // se vuoi usare un altro visualizzatore (eog, se su gnome).

    let output = Command::new("imv")
        .args(&[file_name])
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Command executed successfully:\n{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error executing command:\n{}", stderr);
    }
}