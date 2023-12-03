use image::{ImageFormat, RgbImage};
use robotics_lib::world::tile::*;

mod colors;

fn create_image_from_tiles(tiles: &[Vec<Tile>], _bot_position: (usize, usize), tile_size: usize) -> RgbImage {
    let size: u32 = (tile_size * tiles.len()) as u32;
    let mut img = RgbImage::new(size, size);

    for (y, col) in tiles.iter().enumerate() {
        for (x, tile) in col.iter().enumerate() {
            let color = match tile.tile_type {
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
            img.put_pixel((x * tile_size) as u32, (y * tile_size) as u32, color);
        }
    }

    // for y in 0..tiles.len() {
    //     for x in 0..tiles[y].len() {
    //         let color = match tiles[y][x].tile_type {
    //             TileType::DeepWater => colors::DEEP_WATER,
    //             TileType::ShallowWater => colors::SHALLOW_WATER,
    //             TileType::Sand => colors::SAND,
    //             TileType::Grass => colors::GRASS,
    //             TileType::Street => colors::STREET,
    //             TileType::Hill => colors::HILL,
    //             TileType::Mountain => colors::MOUNTAIN,
    //             TileType::Snow => colors::SNOW,
    //             TileType::Lava => colors::LAVA,
    //             _ => colors::BLACK,
    //         };
    //
    //         img.put_pixel((x * tile_size) as u32, (y * tile_size) as u32, color);
    //     }
    // }

    img
}

pub fn save_world_image(tiles: &[Vec<Tile>], bot_position: (usize, usize), file_name: &str, tile_size: usize) {
    let img = create_image_from_tiles(tiles, bot_position, tile_size);

    if let Err(e) = img.save_with_format(file_name, ImageFormat::Png) {
        println!("Error saving the image: {}", e);
    } else {
        println!("Image saved successfully as {}", file_name);
    }
}