use crate::energy::Energy;
use crate::interface::Direction;
use crate::runner::{backpack::BackPack, Robot, Runnable};
use crate::utils::LibError::NotEnoughSpace;
use crate::utils::*;
use crate::world::tile::{Content, Tile, TileType};
use crate::world::World;
use crate::world::{
    coordinates::Coordinate,
    environmental_conditions::{EnvironmentalConditions, WeatherType::*},
    tile::TileType::*,
};
use strum::IntoEnumIterator;

struct MyRobot(Robot);

impl Runnable for MyRobot {
    fn process_tick(&mut self, _world: &mut World) {}

    fn get_energy(&self) -> &Energy {
        &self.0.energy
    }
    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.0.energy
    }
    fn get_coordinate(&self) -> &Coordinate {
        &self.0.coordinate
    }
    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.0.coordinate
    }
    fn get_backpack(&self) -> &BackPack {
        &self.0.backpack
    }
    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.0.backpack
    }
}

fn generate_map_of_type_and_content(tile_type: TileType, content: Content, size: usize) -> Vec<Vec<Tile>> {
    vec![vec![Tile { tile_type, content }; size]; size]
}

fn generate_a_world_with_all_type_of_contents() -> Vec<Vec<Tile>> {
    let mut this_earth_is_flat = Vec::new();
    for content in Content::iter() {
        if content != Content::None {
            this_earth_is_flat.push(Tile {
                tile_type: Grass,
                content: content.clone(),
            });
        }
    }
    vec![this_earth_is_flat]
}
fn generate_sunny_weather() -> EnvironmentalConditions {
    EnvironmentalConditions::new(&[Sunny], 15, 12)
}

// fn generate_a_world_with_all_type_of_tiles() -> Vec<Vec<Tile>> {
//     let mut this_earth_is_flat = Vec::new();
//     for tile_type in TileType::iter() {
//         this_earth_is_flat.push(Tile {
//             tile_type: tile_type,
//             content: Content::None,
//         });
//     }
//     vec![this_earth_is_flat]
// }

#[test]
fn can_not_go_up_and_left_from_tile_zero_zero() {
    let world = World::new(
        generate_map_of_type_and_content(Grass, Content::None, 4),
        generate_sunny_weather(),
    );
    // Assuming the Robot::new method will set (0, 0) as coordinates.
    let robot = MyRobot(Robot::new());
    assert_eq!(go_allowed(&robot, &world, &Direction::Up), Err(LibError::OutOfBounds));
    assert_eq!(go_allowed(&robot, &world, &Direction::Left), Err(LibError::OutOfBounds));
}
#[test]
fn can_not_move_anywhere_if_world_size_is_one() {
    let world = World::new(
        generate_map_of_type_and_content(Grass, Content::None, 1),
        generate_sunny_weather(),
    );
    // Assuming the Robot::new method will set (0, 0) as coordinates.
    let robot = MyRobot(Robot::new());
    assert_eq!(go_allowed(&robot, &world, &Direction::Up), Err(LibError::OutOfBounds));
    assert_eq!(go_allowed(&robot, &world, &Direction::Down), Err(LibError::OutOfBounds));
    assert_eq!(
        go_allowed(&robot, &world, &Direction::Right),
        Err(LibError::OutOfBounds)
    );
    assert_eq!(go_allowed(&robot, &world, &Direction::Left), Err(LibError::OutOfBounds));
}
#[test]
fn can_move_down_and_right_from_tile_zero_zero() {
    let world = World::new(
        generate_map_of_type_and_content(Grass, Content::None, 4),
        generate_sunny_weather(),
    );
    // Assuming the Robot::new method will set (0, 0) as coordinates.
    let robot = MyRobot(Robot::new());

    assert_eq!(go_allowed(&robot, &world, &Direction::Down), Ok(true));
    assert_eq!(go_allowed(&robot, &world, &Direction::Right), Ok(true));
}

#[test]
#[ignore]
fn can_not_move_on_a_not_walkable_tile() {
    // todo needs a not walkable tile
}

// TODO! check if it needs a rework since the other coordinate implementation works differently 💀
#[test]
fn get_direction_coordinates() {
    // Assuming the Robot::new method will set (0, 0) as coordinates.
    let robot = MyRobot(Robot {
        energy: Energy::new(0),
        coordinate: Coordinate::new(1, 1),
        backpack: BackPack::new(0),
    });
    assert_eq!(get_coords_row_col(&robot, &Direction::Down), (2, 1));
    assert_eq!(get_coords_row_col(&robot, &Direction::Left), (1, 0));
    assert_eq!(get_coords_row_col(&robot, &Direction::Right), (1, 2));
    assert_eq!(get_coords_row_col(&robot, &Direction::Up), (0, 1));
}

#[test]
fn a_tree_will_be_destroyed() {
    let world = World::new(
        generate_map_of_type_and_content(Grass, Content::Tree(0), 1),
        generate_sunny_weather(),
    );
    assert_eq!(can_destroy(&world, (0, 0)), Ok(true));
}
#[test]
fn a_bin_will_not_be_destroyed() {
    let world = World::new(
        generate_map_of_type_and_content(Grass, Content::Bin(0..0), 1),
        generate_sunny_weather(),
    );
    assert_eq!(can_destroy(&world, (0, 0)), Ok(false));
}

#[test]
fn a_bank_will_not_be_destroyed() {
    let world = World::new(
        generate_map_of_type_and_content(Grass, Content::Bank(0..0), 1),
        generate_sunny_weather(),
    );
    assert_eq!(can_destroy(&world, (0, 0)), Ok(false));
}

#[test]
fn a_crate_will_not_be_destroyed() {
    let world = World::new(
        generate_map_of_type_and_content(Grass, Content::Crate(0..0), 1),
        generate_sunny_weather(),
    );
    assert_eq!(can_destroy(&world, (0, 0)), Ok(false));
}

#[test]
fn a_coin_will_be_destroyed() {
    let world = World::new(
        generate_map_of_type_and_content(Grass, Content::Coin(0), 1),
        generate_sunny_weather(),
    );
    assert_eq!(can_destroy(&world, (0, 0)), Ok(true));
}

#[test]
fn fire_will_be_destroyed() {
    let world = World::new(
        generate_map_of_type_and_content(Grass, Content::Fire, 1),
        generate_sunny_weather(),
    );
    assert_eq!(can_destroy(&world, (0, 0)), Ok(true));
}

#[test]
fn garbage_will_be_destroyed() {
    let world = World::new(
        generate_map_of_type_and_content(Grass, Content::Garbage(0), 1),
        generate_sunny_weather(),
    );
    assert_eq!(can_destroy(&world, (0, 0)), Ok(true));
}

#[test]
fn none_will_not_be_destroyed() {
    let world = World::new(
        generate_map_of_type_and_content(Grass, Content::None, 1),
        generate_sunny_weather(),
    );
    assert_eq!(can_destroy(&world, (0, 0)), Ok(false));
}

#[test]
fn water_will_be_destroyed() {
    let world = World::new(
        generate_map_of_type_and_content(Grass, Content::Water(0), 1),
        generate_sunny_weather(),
    );
    assert_eq!(can_destroy(&world, (0, 0)), Ok(true));
}

// not needed anymore
//
// #[test]
// fn get_props_from_existing_contents() {
//     let flat_world = generate_a_world_with_all_type_of_contents();
//     for tile in flat_world[0].iter() {
//         assert!(get_prop_content(&tile.content).is_ok());
//     }
// }
//
// #[test]
// fn get_the_single_prop_from_existing_tile_content() {
//     let flat_world = generate_a_world_with_all_type_of_contents();
//     for tile in flat_world[0].iter() {
//         println!("Type: {:?}, content: {:?}", tile.tile_type, tile.content);
//         assert!(get_cost_content(&tile.content).is_ok());
//         assert!(get_max_content(&tile.content).is_ok());
//     }
// }
//
// #[test]
// fn none_does_not_have_props() {
//     let world = World::new(
//         generate_map_of_type_and_content(Grass, Content::None, 1),
//         generate_sunny_weather(),
//     );
//     assert!(!get_prop_content(&world.map[0][0].content).is_ok());
// }
// #[test]
// fn get_props_from_existing_tiles() {
//     let flat_world = generate_a_world_with_all_type_of_tiles();
//     for tile in flat_world[0].iter() {
//         assert!(get_prop_tiletype(&tile.tile_type).is_ok());
//     }
// }
//
// #[test]
// fn get_the_single_prop_from_existing_tile() {
//     let flat_world = generate_a_world_with_all_type_of_tiles();
//     for tile in flat_world[0].iter() {
//         println!("Type: {:?}, content: {:?}", tile.tile_type, tile.content);
//         assert!(get_cost_tiletype(&tile.tile_type).is_ok());
//         assert!(get_walk_tiletype(&tile.tile_type).is_ok());
//         assert!(get_hold_tiletype(&tile.tile_type).is_ok());
//     }
// }

#[test]
fn add_tree_to_backpack_with_exact_space() {
    let mut robot = MyRobot(Robot {
        energy: Energy::default(),
        coordinate: Coordinate::new(0, 0),
        backpack: BackPack::new(10),
    });
    assert_eq!(add_to_backpack(&mut robot, Content::Tree(0), 10), Ok(10));
}

#[test]
fn add_various_content_to_backpack_untill_full() {
    let mut robot = MyRobot(Robot {
        energy: Energy::default(),
        coordinate: Coordinate::new(0, 0),
        backpack: BackPack::new(50),
    });
    assert_eq!(add_to_backpack(&mut robot, Content::Tree(0), 10), Ok(10));
    assert_eq!(add_to_backpack(&mut robot, Content::Coin(0), 10), Ok(10));
    assert_eq!(add_to_backpack(&mut robot, Content::Water(0), 30), Ok(30));
    assert_eq!(
        add_to_backpack(&mut robot, Content::Garbage(0), 30),
        Err(NotEnoughSpace(0))
    );
}

#[test]
fn add_tree_to_backpack_with_not_enough_space() {
    let mut robot = MyRobot(Robot {
        energy: Energy::default(),
        coordinate: Coordinate::new(0, 0),
        backpack: BackPack::new(6),
    });
    assert_eq!(
        add_to_backpack(&mut robot, Content::Tree(0), 10),
        Err(LibError::NotEnoughSpace(6))
    );
}
