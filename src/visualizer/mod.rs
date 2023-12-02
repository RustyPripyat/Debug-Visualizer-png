use image::{ImageFormat, RgbImage};
use robotics_lib::world::tile::*;
mod colors;

fn create_image_from_tiles(tiles: &[Vec<Tile>], _bot_position: (usize, usize), _tile_size: usize) -> RgbImage {
    let mut img = RgbImage::new(tiles.len() as u32, tiles.len() as u32);

    for y in 0..tiles.len() {
        for x in 0..tiles[y].len() {
            let color = match tiles[y][x].tile_type {
                TileType::DeepWater => colors::DEEP_WATER,
                TileType::ShallowWater => colors::SHALLOW_WATER,
                TileType::Sand => colors::SAND,
                TileType::Grass => colors::GRASS,
                TileType::Street => colors::STREET,
                TileType::Hill => colors::HILL,
                TileType::Mountain => colors::MOUNTAIN,
                TileType::Snow => colors::SNOW,
                TileType::Lava => colors::LAVA,
                _ => colors::BLACK,
            };

            img.put_pixel(x as u32, y as u32, color);
        }
    }

    img
}

pub fn save_world_image(tiles: &[Vec<Tile>], bot_position: (usize, usize), file_name: &str) {
    let img = create_image_from_tiles(tiles, bot_position, 4);

    if let Err(e) = img.save_with_format(file_name, ImageFormat::Png) {
        println!("Error saving the image: {}", e);
    } else {
        println!("Image saved successfully as {}", file_name);
    }
}