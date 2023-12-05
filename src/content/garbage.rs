use crate::generator::GarbageSettings;
use rand::Rng;
use robotics_lib::world::tile::{Content, Tile, TileType};

pub(crate) fn spawn_garbage(world: &mut Vec<Vec<Tile>>, settings: &GarbageSettings) {
    let mut rng = rand::thread_rng();

    let mut y = rng.gen_range(settings.distance_from_borders..world.len() - settings.distance_from_borders);
    let mut x = rng.gen_range(settings.distance_from_borders..world.len() - settings.distance_from_borders);
    let mut amount = rng.gen_range(1..settings.max_amount_on_destroy);

    let mut i = 0;
    while i < settings.spawn_points_quantity {
        match world[y][x].tile_type {
            | TileType::Sand
            | TileType::Grass
            | TileType::Street
            | TileType::Hill
            | TileType::Mountain
            | TileType::Teleport(_) => {
                i += 1;
                world[y][x].content = Content::Garbage(amount);
                amount = rng.gen_range(1..settings.max_amount_on_destroy);
            }
            | _ => {}
        }

        y = rng.gen_range(settings.distance_from_borders..world.len() - settings.distance_from_borders);
        x = rng.gen_range(settings.distance_from_borders..world.len() - settings.distance_from_borders);
    }
}
