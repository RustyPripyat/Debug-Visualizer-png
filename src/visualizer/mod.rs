use image::{ImageFormat, RgbImage};
use robotics_lib::world::tile::*;

mod colors;
/// Associates each tile with its color
macro_rules! get_tile_color {
    ($t:expr) => {
        match $t {
            TileType::DeepWater => colors::tile::DEEP_WATER,
            TileType::ShallowWater => colors::tile::SHALLOW_WATER,
            TileType::Sand => colors::tile::SAND,
            TileType::Grass => colors::tile::GRASS,
            TileType::Street => colors::tile::STREET,
            TileType::Hill => colors::tile::HILL,
            TileType::Mountain => colors::tile::MOUNTAIN,
            TileType::Snow => colors::tile::SNOW,
            TileType::Lava => colors::tile::LAVA,
            TileType::Wall => colors::tile::BRICK,
            _ => colors::tile::BLACK,
        }
    }
}

fn create_image_from_tiles(tiles: &[Vec<Tile>], _bot_position: (usize, usize), tile_size: usize) -> RgbImage {
    let size: u32 = (tile_size * tiles.len()) as u32;
    let mut img = RgbImage::new(size, size);

    for (y, col) in tiles.iter().enumerate() {
        for (x, tile) in col.iter().enumerate() {
            let color = get_tile_color!(tile.tile_type);
            img.put_pixel((x * tile_size) as u32, (y * tile_size) as u32, color);
        }
    }
    // for y in 0..tiles.len() {
    //     for x in 0..tiles[y].len() {
    //         let color = match tiles[y][x].tile_type {
    //             TileType::DeepWater => colors::tile::DEEP_WATER,
    //             TileType::ShallowWater => colors::tile::SHALLOW_WATER,
    //             TileType::Sand => colors::tile::SAND,
    //             TileType::Grass => colors::tile::GRASS,
    //             TileType::Street => colors::tile::STREET,
    //             TileType::Hill => colors::tile::HILL,
    //             TileType::Mountain => colors::tile::MOUNTAIN,
    //             TileType::Snow => colors::tile::SNOW,
    //             TileType::Lava => colors::tile::LAVA,
    //             _ => colors::tile::BLACK,
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