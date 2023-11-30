use rand::Rng;
use robotics_lib::world::tile::{Content, TileType};

pub(crate) fn add_default_water_content(tile_type: TileType)-> Content{
    if tile_type == TileType::DeepWater || tile_type == TileType::ShallowWater {
        //return Content::Water with random value between 0 and Content::Water max
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(0..Content::Water(0).properties().max());
        return Content::Water(value);
    }
    else {
        return Content::None;
    }
}