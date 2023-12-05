use rand::Rng;
use robotics_lib::world::tile::Tile;
use crate::generator::GarbageSettings;

pub(crate) fn spawn_garbage(world: &mut Vec<Vec<Tile>>, elevation_map: &Vec<Vec<f64>>, settings: &GarbageSettings) {
    let mut rng = rand::thread_rng();
    let random_x = rng.gen_range(0..world.len() - 4);
    let random_y = rng.gen_range(0..world.len());
}