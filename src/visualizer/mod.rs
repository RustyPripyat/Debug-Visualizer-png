use image::{ImageFormat, Rgb, RgbImage};
use rand::Rng;
use robotics_lib::world::tile::*;

mod colors;


/// Fill random pixels or all based on number of content with the appropriate color
macro_rules! fill_random_pixel_with_color {
    ($p:expr, $n:expr, $c:expr) => {
        if $p.len() >= $n{
            let mut rng = rand::thread_rng();
            for _ in 0..$n {
                let i = rng.gen_range(0..$p.len());
                $p.insert(i, $c);
            }
        } else {
            $p.fill($c);
        }
    }
}
/// Associates each tile with its color
macro_rules! set_tile_color {
    ($t:expr, $v:expr) => {
        match *$t {
            TileType::DeepWater => $v.fill(colors::tile::DEEP_WATER),
            TileType::ShallowWater => $v.fill(colors::tile::SHALLOW_WATER),
            TileType::Sand => $v.fill(colors::tile::SAND),
            TileType::Grass => $v.fill(colors::tile::GRASS),
            TileType::Street => $v.fill(colors::tile::STREET),
            TileType::Hill => $v.fill(colors::tile::HILL),
            TileType::Mountain => $v.fill(colors::tile::MOUNTAIN),
            TileType::Snow => $v.fill(colors::tile::SNOW),
            TileType::Lava => $v.fill(colors::tile::LAVA),
            TileType::Wall => $v.fill(colors::tile::BRICK),
            _ => $v.fill(colors::BLACK),
        }
    }
}

/// Associates each tile content with its color
macro_rules! set_content_color {
    ($c:expr, $p:expr) => {
        match *$c {
            Content::Rock(n) => fill_random_pixel_with_color!($p, n, colors::content::ROCK),
            Content::Tree(n) => fill_random_pixel_with_color!($p, n,colors::content::TREE),
            Content::Garbage(n) => fill_random_pixel_with_color!($p, n,colors::content::ROCK),
            Content::Fire => fill_random_pixel_with_color!($p, 1, colors::content::FIRE),
            Content::Coin(n) => fill_random_pixel_with_color!($p, n, colors::content::COIN),
            Content::Bin(n) => fill_random_pixel_with_color!($p, 1, colors::content::BIN),
            Content::Crate(n) => fill_random_pixel_with_color!($p, 1, colors::content::CRATE),
            Content::Bank(n) => fill_random_pixel_with_color!($p, 1, colors::content::BANK),
            Content::Water(n) => fill_random_pixel_with_color!($p, n, colors::tile::SHALLOW_WATER),
            Content::Market(n) => fill_random_pixel_with_color!($p, n, colors::content::MARKET),
            Content::Fish(n) => fill_random_pixel_with_color!($p, n, colors::content::FISH),
            Content::Building => fill_random_pixel_with_color!($p, 1, colors::content::BUILDING),
            Content::Bush(n) => fill_random_pixel_with_color!($p, n, colors::content::BUSH),
            Content::JollyBlock(n) => fill_random_pixel_with_color!($p, n, colors::content::JOLLYBLOCK),
            Content::Scarecrow => fill_random_pixel_with_color!($p, 1, colors::content::SCARECROW),
            _ => fill_random_pixel_with_color!($p, 1, colors::BLACK),
        }
    }
}

fn create_image_from_tiles(tiles: &[Vec<Tile>], _bot_position: (usize, usize), tile_size: usize) -> RgbImage {
    let size: u32 = (tile_size * tiles.len()) as u32;
    let mut img = RgbImage::new(size, size);

    for (y, a) in tiles.iter().enumerate() {
        for (x, tile) in a.iter().enumerate() {
            let mut pixels: Vec<Rgb<u8>> = Vec::with_capacity(tile_size);
            set_tile_color!(&tile.tile_type, pixels);
            if tile.content != Content::None {
                set_content_color!(&tile.content,pixels);
            }
            img.put_pixel((x * tile_size) as u32, (y * tile_size) as u32, color);
        }
    }
    img
}

pub fn save_world_image(tiles: &[Vec<Tile>], bot_position: (usize, usize), file_name: &str, tile_size: usize) {
    let img = create_image_from_tiles(tiles, bot_position, tile_size);

    if let Err(e) = img.save_with_format(file_name, ImageFormat::Png) {
        println!("Error saving the image: $v.fill(colors::GRAY),", e);
    } else {
        println!("Image saved successfully as $v.fill(colors::GRAY),", file_name);
    }
}