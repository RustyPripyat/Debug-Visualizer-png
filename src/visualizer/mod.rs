use image::{ImageFormat, Rgb, RgbImage};
use rand::Rng;
use robotics_lib::world::tile::*;

mod colors;

/// Fill random pixels or all based on number of content with the appropriate color
#[inline(always)]
fn fill_random_pixel_with_color(p: &mut Vec<Rgb<u8>>, n: usize, c: Rgb<u8>) {
    if p.len() > n {
        let mut rng = rand::thread_rng();
        for _ in 1..n {
            let i = rng.gen_range(0..p.len());
            p.insert(i, c);
        }
    } else {
        p.fill(c);
    }
}

/// Associates each tile with its color
#[inline(always)]
fn set_tile_color(t: &TileType, v: &mut [Rgb<u8>]) {
    match *t {
        TileType::DeepWater => v.fill(colors::tile::DEEP_WATER),
        TileType::ShallowWater => v.fill(colors::tile::SHALLOW_WATER),
        TileType::Sand => v.fill(colors::tile::SAND),
        TileType::Grass => v.fill(colors::tile::GRASS),
        TileType::Street => v.fill(colors::tile::STREET),
        TileType::Hill => v.fill(colors::tile::HILL),
        TileType::Mountain => v.fill(colors::tile::MOUNTAIN),
        TileType::Snow => v.fill(colors::tile::SNOW),
        TileType::Lava => v.fill(colors::tile::LAVA),
        TileType::Wall => v.fill(colors::tile::BRICK),
        _ => v.fill(colors::BLACK),
    }
}

/// Associates each tile content with its color
#[inline(always)]
fn set_content_color(c: &Content, p: &mut Vec<Rgb<u8>>) {
    match *c {
        Content::Rock(n) => fill_random_pixel_with_color(p, n, colors::content::ROCK),
        Content::Tree(n) => fill_random_pixel_with_color(p, n, colors::content::TREE),
        Content::Garbage(n) => fill_random_pixel_with_color(p, n, colors::content::ROCK),
        Content::Fire => fill_random_pixel_with_color(p, p.len(), colors::content::FIRE),
        Content::Coin(n) => fill_random_pixel_with_color(p, n, colors::content::COIN),
        Content::Bin(_) => fill_random_pixel_with_color(p, p.len(), colors::content::BIN),
        Content::Crate(_) => fill_random_pixel_with_color(p, p.len(), colors::content::CRATE),
        Content::Bank(_) => fill_random_pixel_with_color(p, p.len(), colors::content::BANK),
        Content::Water(n) => fill_random_pixel_with_color(p, n, colors::tile::SHALLOW_WATER),
        Content::Market(_) => fill_random_pixel_with_color(p, p.len(), colors::content::MARKET),
        Content::Fish(n) => fill_random_pixel_with_color(p, n, colors::content::FISH),
        Content::Building => fill_random_pixel_with_color(p, p.len(), colors::content::BUILDING),
        Content::Bush(n) => fill_random_pixel_with_color(p, n, colors::content::BUSH),
        Content::JollyBlock(n) => fill_random_pixel_with_color(p, n, colors::content::JOLLYBLOCK),
        Content::Scarecrow => fill_random_pixel_with_color(p, 1, colors::content::SCARECROW),
        _ => fill_random_pixel_with_color(p, 1, colors::BLACK),
    }
}

fn create_image_from_tiles(tiles: &[Vec<Tile>], _bot_position: (usize, usize), tile_size: usize) -> RgbImage {
    let size: u32 = (tile_size * tiles.len()) as u32;
    let mut img = RgbImage::new(size, size);

    for (y, a) in tiles.iter().enumerate() {
        for (x, tile) in a.iter().enumerate() {
            let mut pixels: Vec<Rgb<u8>> = vec![colors::BLACK; tile_size.pow(2)];
            set_tile_color(&tile.tile_type, &mut pixels);
            if tile.content != Content::None {
                set_content_color(&tile.content, &mut pixels);
            }

            for mx in 0..tile_size {
                for my in 0..tile_size {
                    img.put_pixel((x * tile_size + mx) as u32, (y * tile_size + my) as u32, pixels[mx * tile_size + my]);
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