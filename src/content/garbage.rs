use crate::generator::GarbageSettings;
use rand::Rng;
use robotics_lib::world::tile::{Content, Tile, TileType};

pub(crate) fn spawn_garbage(world: &mut Vec<Vec<Tile>>, settings: &GarbageSettings) {
    let mut rng = rand::thread_rng();

    let mut y = rng.gen_range(settings.distance_from_borders..world.len() - settings.distance_from_borders);
    let mut x = rng.gen_range(settings.distance_from_borders..world.len() - settings.distance_from_borders);
    let mut amount = rng.gen_range(1..settings.max_amount_on_destroy);

    let mut i = 0;
    let neg_pos = -(settings.spawn_in_near_tiles_probability / settings.decrease_probability_by).ceil() as i32;
    let mut pos = -neg_pos;
    let mut prob = settings.spawn_in_near_tiles_probability;

    while i < settings.spawn_points_quantity {
        if set_content(world, y, x, amount) {
            i += 1;
            amount = rng.gen_range(1..settings.max_amount_on_destroy);

            while neg_pos <= pos && prob >= 0.0 {
                if rng.gen_bool(prob) {
                    set_content(world, (y as i32 + pos) as usize, x, amount);
                }
                if rng.gen_bool(prob) {
                    set_content(world, y, (x as i32 - pos) as usize, amount);
                }
                if rng.gen_bool(prob) {
                    set_content(world, (y as i32 - pos) as usize, (x as i32 - pos) as usize, amount);
                }
                if rng.gen_bool(prob) {
                    set_content(world, (y as i32 + pos) as usize, (x as i32 + pos) as usize, amount);
                }
                if rng.gen_bool(prob) {
                    set_content(world, (y as i32 - pos) as usize, (x as i32 + pos) as usize, amount);
                }
                if rng.gen_bool(prob) {
                    set_content(world, (y as i32 + pos) as usize, (x as i32 - pos) as usize, amount);
                }

                prob -= settings.decrease_probability_by;
                pos -= 1;
                amount = rng.gen_range(1..settings.max_amount_on_destroy);
            }
            pos = 1;
            prob = settings.spawn_in_near_tiles_probability;
        }

        y = rng.gen_range(settings.distance_from_borders..world.len() - settings.distance_from_borders);
        x = rng.gen_range(settings.distance_from_borders..world.len() - settings.distance_from_borders);
    }
}

#[inline(always)]
fn set_content(world: &mut [Vec<Tile>], y: usize, x: usize, amount: usize) -> bool {
    if y == 0 || y >= world.len() || x == 0 || x >= world.len() {
        return false;
    }

    match world[y][x].tile_type {
        | TileType::Sand
        | TileType::Grass
        | TileType::Street
        | TileType::Hill
        | TileType::Mountain
        | TileType::Teleport(_) => {
            world[y][x].content = Content::Garbage(amount);
            true
        }
        | _ => false,
    }
}
