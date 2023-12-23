use chrono::Utc;
use debug_print::debug_println;
use image::{ImageFormat, Rgb, RgbImage};

use robotics_lib::world::tile::*;

mod colors;

/// Fill random pixels or all based on number of content with the appropriate color
#[inline(always)]
fn checkerboard_pattern(p: &mut Vec<Vec<Rgb<u8>>>, c: Rgb<u8>) {
    let mut b = true;
    for row in 0..p.len() {
        b = if p.len() % 2 == 0 { !b } else { b };
        for col in 0..p.len() {
            if b {
                p[row][col] = c;
            }
            b = !b;
        }
    }
}

/// Associates each tile with its color
#[inline(always)]
fn choose_tile_color(t: &TileType) -> Rgb<u8> {
    match *t {
        | TileType::DeepWater => colors::tile::DEEP_WATER,
        | TileType::ShallowWater => colors::tile::SHALLOW_WATER,
        | TileType::Sand => colors::tile::SAND,
        | TileType::Grass => colors::tile::GRASS,
        | TileType::Street => colors::tile::STREET,
        | TileType::Hill => colors::tile::HILL,
        | TileType::Mountain => colors::tile::MOUNTAIN,
        | TileType::Snow => colors::tile::SNOW,
        | TileType::Lava => colors::tile::LAVA,
        | TileType::Wall => colors::tile::BRICK,
        | _ => colors::BLACK,
    }
}

/// Associates each tile content with its color
#[inline(always)]
fn set_content_color(c: &Content, p: &mut Vec<Vec<Rgb<u8>>>) {
    match *c {
        | Content::Rock(_) => checkerboard_pattern(p, colors::content::ROCK),
        | Content::Tree(_) => checkerboard_pattern(p, colors::content::TREE),
        | Content::Garbage(_) => checkerboard_pattern(p, colors::BLACK),
        | Content::Fire => checkerboard_pattern(p, colors::content::FIRE),
        | Content::Coin(_) => checkerboard_pattern(p, colors::content::COIN),
        | Content::Bin(_) => checkerboard_pattern(p, colors::content::BIN),
        | Content::Crate(_) => checkerboard_pattern(p, colors::content::CRATE),
        | Content::Bank(_) => checkerboard_pattern(p, colors::content::BANK),
        | Content::Water(_) => checkerboard_pattern(p, colors::tile::SHALLOW_WATER),
        | Content::Market(_) => checkerboard_pattern(p, colors::content::MARKET),
        | Content::Fish(_) => checkerboard_pattern(p, colors::content::FISH),
        | Content::Building => checkerboard_pattern(p, colors::content::BUILDING),
        | Content::Bush(_) => checkerboard_pattern(p, colors::content::BUSH),
        | Content::JollyBlock(_) => checkerboard_pattern(p, colors::content::JOLLYBLOCK),
        | Content::Scarecrow => checkerboard_pattern(p, colors::content::SCARECROW),
        | _ => checkerboard_pattern(p, colors::BLACK),
    }
}

fn create_image_from_tiles(tiles: &[Vec<Tile>], _bot_position: (usize, usize), tile_size: usize) -> RgbImage {
    // get the image final size
    let size: u32 = (tile_size * tiles.len()) as u32;
    let mut img: RgbImage = RgbImage::new(size, size);

    for (y, row) in tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            // set the base tile color as tile type color
            let mut pixels: Vec<Vec<Rgb<u8>>> = vec![vec![choose_tile_color(&tile.tile_type); tile_size]; tile_size];

            // set the content color as checkerboard of the tile
            if tile.content != Content::None {
                set_content_color(&tile.content, &mut pixels);
            }

            for my in 0..tile_size {
                for mx in 0..tile_size {
                    img.put_pixel((x * tile_size + mx) as u32, (y * tile_size + my) as u32, pixels[my][mx]);
                }
            }
        }
    }
    img
}

pub fn save_world_image(tiles: &[Vec<Tile>], bot_position: (usize, usize), file_name: &str, tile_size: usize) {
    debug_println!("Start: saving world as png");
    let start = Utc::now();
    let img = create_image_from_tiles(tiles, bot_position, tile_size);

    if let Err(e) = img.save_with_format(file_name, ImageFormat::Png) {
        panic!("Error saving the image, {}", e);
    }
    debug_println!("Done: saving world as png {}ms", (Utc::now() - start).num_milliseconds());
}
