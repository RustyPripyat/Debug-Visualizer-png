use image::{ImageFormat, Rgb, RgbImage};
use rand::Rng;
use robotics_lib::world::tile::*;

mod colors;

/// Fill random pixels or all based on number of content with the appropriate color
#[inline(always)]
fn fill_random_pixel_with_color(p: &mut Vec<Vec<Rgb<u8>>>, c: Rgb<u8>) {
    let mut b = false;

    for row in 0..p.len() {
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
fn choose_tile_color(t: &TileType, v: &mut Vec<Vec<Rgb<u8>>>) {
    match *t {
        | TileType::DeepWater => set_color(v, colors::tile::DEEP_WATER),
        | TileType::ShallowWater => set_color(v, colors::tile::SHALLOW_WATER),
        | TileType::Sand => set_color(v, colors::tile::SAND),
        | TileType::Grass => set_color(v, colors::tile::GRASS),
        | TileType::Street => set_color(v, colors::tile::STREET),
        | TileType::Hill => set_color(v, colors::tile::HILL),
        | TileType::Mountain => set_color(v, colors::tile::MOUNTAIN),
        | TileType::Snow => set_color(v, colors::tile::SNOW),
        | TileType::Lava => set_color(v, colors::tile::LAVA),
        | TileType::Wall => set_color(v, colors::tile::BRICK),
        | _ => set_color(v, colors::BLACK),
    }
}


#[inline(always)]
fn set_color(v: &mut Vec<Vec<Rgb<u8>>>, color: Rgb<u8>) {
    for i in 0..v.len() {
        for j in 0..v.len() {
            v[i][j] = color
        }
    }
}

/// Associates each tile content with its color
#[inline(always)]
fn set_content_color(c: &Content, p: &mut Vec<Vec<Rgb<u8>>>) {
    match *c {
        | Content::Rock(_) => fill_random_pixel_with_color(p, colors::content::ROCK),
        | Content::Tree(_) => fill_random_pixel_with_color(p, colors::content::TREE),
        | Content::Garbage(_) => fill_random_pixel_with_color(p, colors::BLACK),
        | Content::Fire => fill_random_pixel_with_color(p, colors::content::FIRE),
        | Content::Coin(_) => fill_random_pixel_with_color(p, colors::content::COIN),
        | Content::Bin(_) => fill_random_pixel_with_color(p, colors::content::BIN),
        | Content::Crate(_) => fill_random_pixel_with_color(p, colors::content::CRATE),
        | Content::Bank(_) => fill_random_pixel_with_color(p, colors::content::BANK),
        | Content::Water(_) => fill_random_pixel_with_color(p, colors::tile::SHALLOW_WATER),
        | Content::Market(_) => fill_random_pixel_with_color(p, colors::content::MARKET),
        | Content::Fish(_) => fill_random_pixel_with_color(p, colors::content::FISH),
        | Content::Building => fill_random_pixel_with_color(p, colors::content::BUILDING),
        | Content::Bush(_) => fill_random_pixel_with_color(p, colors::content::BUSH),
        | Content::JollyBlock(_) => fill_random_pixel_with_color(p, colors::content::JOLLYBLOCK),
        | Content::Scarecrow => fill_random_pixel_with_color(p, colors::content::SCARECROW),
        | _ => fill_random_pixel_with_color(p, colors::BLACK),
    }
}

fn create_image_from_tiles(tiles: &[Vec<Tile>], _bot_position: (usize, usize), tile_size: usize) -> RgbImage {
    let size: u32 = (tile_size * tiles.len()) as u32;
    let mut img = RgbImage::new(size, size);

    for (y, a) in tiles.iter().enumerate() {
        for (x, tile) in a.iter().enumerate() {
            let mut pixels: Vec<Vec<Rgb<u8>>> = vec![vec![colors::BLACK; tile_size.pow(2)]; tile_size.pow(2)];
            choose_tile_color(&tile.tile_type, &mut pixels);
            if tile.content != Content::None {
                set_content_color(&tile.content, &mut pixels);
            }

            for mx in 0..tile_size {
                for my in 0..tile_size {
                    img.put_pixel(
                        (x * tile_size + mx) as u32,
                        (y * tile_size + my) as u32,
                        pixels[mx][my],
                    );
                }
            }
        }
    }
    img
}

pub fn save_world_image(tiles: &[Vec<Tile>], bot_position: (usize, usize), file_name: &str, tile_size: usize) {
    let img = create_image_from_tiles(tiles, bot_position, tile_size);

    if let Err(e) = img.save_with_format(file_name, ImageFormat::Png) {
        println!("Error saving the image, {}", e);
    } else {
        println!("Image saved successfully, {}", file_name);
    }
}
