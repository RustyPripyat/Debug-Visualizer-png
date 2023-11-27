use crate::runner::Runnable;
use crate::utils::LibError::*;
use crate::utils::*;
use crate::world::coordinates::Coordinate;
use crate::world::environmental_conditions::EnvironmentalConditions;
use crate::world::tile::Content::Water;
use crate::world::tile::TileType::{DeepWater, ShallowWater};
use crate::world::tile::{Content, Tile, TileType};
use crate::world::World;
use lazy_static::lazy_static;
use rand::Rng;
use std::cmp::min;
use std::sync::Mutex;
use strum_macros::EnumIter;

/// Direction enum
/// Given to the right functions will move the robot in the given direction, remove the content of a tile
/// and more.
///
/// # Usage
/// ```
/// use robotics_lib::interface::Direction;
/// let direction_up= Direction::Up;
/// ```
///
/// # Variants
/// - `Up`: Move the robot up
/// - `Down`: Move the robot down
/// - `Left`: Move the robot left
/// - `Right`: Move the robot right
///

#[derive(Debug, Clone, Eq, PartialEq, EnumIter)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

lazy_static! {
    static ref PLOT: Mutex<Vec<(usize, usize)>> = Mutex::new(vec![]);
}

//Interface to move
// Go ----------------------------------------------------
/// Given the robot, the world and the direction, will move the robot in the given direction
///
/// # Usage
/// ```
/// use robotics_lib::interface::go;
/// ```
///
/// # Arguments
/// - `robot`: The robot that will be moved
/// - `world`: The world in which the robot is
/// - `direction`: The direction in which the robot will be moved
///
/// # Returns
/// - `Ok`: The view of the robot from is new position and the new position
/// - `Err`: The robot couldn't be moved
///
/// # Errors:
/// - `NoTileTypeProps`: The TileTypeProp of the target cell is not set properly
/// - `OutOfBounds`: The robot couldn't be moved cause it's on the border an the chosen direction is out of bounds
/// - `CannotWalk`: The robot cannot walk on the desired tiletype
/// - `NotEnoughEnergy`: The robot doesn't have enough energy to move in the desired direction
///
/// # Examples
/// ```
/// use robotics_lib::interface::{Direction, go};
/// use robotics_lib::runner::Runnable;
/// use robotics_lib::utils::LibError;
/// use robotics_lib::world::World;
///
/// fn go_example(mut world: &mut World, mut robot: &mut impl Runnable, direction: Direction)-> Result<(), LibError> {
///     let updated_view = match go(robot, world, direction) {
///         Ok((view, _)) => view,
///         Err(e) => { return Err(e); }
///     };
///     for row in updated_view.iter(){
///         for col in row.iter() {
///             match col {
///                 | None => print!("default_unknown_tile"),
///                 | Some(tile) => print!("{:?}", tile.content),
///             }
///         }
///     }
///     Ok(())
/// }
/// ```
pub fn go(
    robot: &mut impl Runnable,
    world: &mut World,
    direction: Direction,
) -> Result<(Vec<Vec<Option<Tile>>>, Coordinate), LibError> {
    match go_allowed(robot, world, &direction) {
        | Ok(_) => {
            let cost = world.map[robot.get_coordinate().get_row()][robot.get_coordinate().get_col()]
                .tile_type
                .properties()
                .cost();
            robot.get_energy_mut().consume_energy(cost)?;
            let (row, col) = get_coords_row_col(robot, &direction);
            *robot.get_coordinate_mut() = Coordinate::new(row, col);
            Ok(where_am_i(robot, world))
        }
        | Err(e) => Err(e),
    }
}

// // Destroy ----------------------------------------------------
/// Given the robot, the world and the direction, will destroy the content of the tile in the given direction
///
/// # Usage
/// ```
/// use robotics_lib::interface::destroy;
/// ```
///
/// # Arguments
/// - `robot`: The robot that will be moved
/// - `world`: The world in which the robot is
/// - `direction`: The direction in which will be destroyed the content
///
/// # Returns
/// - `Ok`: The content that was destroyed and the quantity of the content that was destroyed
/// - `Err`: The content couldn't be destroyed
///
/// # Errors
/// - `OutOfBounds`: The content couldn't be destroyed
/// - `NotEnoughEnergy`: The robot doesn't have enough energy to destroy the content
/// - `NoContentProp`: The content doesn't have a property
/// - `NoContent`: The content doesn't exist
/// - `NotEnoughSpacei32)`: The backpack of the robot doesn't have enough space to store the content
///
/// # Examples
/// ```
/// use robotics_lib::interface::{destroy, Direction};
/// use robotics_lib::runner::Runnable;
/// use robotics_lib::world::tile::Content;
/// use robotics_lib::world::World;
/// fn destroy_example(mut world: &mut World, mut robot: &mut impl Runnable, direction: Direction) {
///     match destroy(robot, world, direction){
///         Ok(quantity) => {
///             print!("{:?} quantity of the content has been added to your backpack", quantity);
///         }
///         Err(e) => {
///             print!("{:?}", e);}
///     }
/// }
/// ```
///
/// # Remarks
/// - The content that was destroyed is returned
/// - The quantity of the content that was destroyed is returned
/// - The content that was destroyed is removed from the world
/// - Destroying a content will add it to the backpack of the robot
/// - If the content quantity is more than the free space in the backpack, the content will be added to the backpack until it's full
pub fn destroy(robot: &mut impl Runnable, world: &mut World, direction: Direction) -> Result<usize, LibError> {
    let mut rng = rand::thread_rng();

    let (target_row, target_col) = get_coords_row_col(robot, &direction);
    let target_tile = &world.map[target_row][target_col];
    let content = &target_tile.content;

    // to change (match what was voted)
    if [ShallowWater, DeepWater].contains(&target_tile.tile_type) && *content == Content::None {
        let max = Content::Water(0).properties().max();
        let cost = Content::Water(0).properties().cost();
        robot.get_energy_mut().consume_energy(cost)?;
        if max == 0 {
            return Ok(0);
        } else {
            match add_to_backpack(robot, Water(0), rng.gen_range(0..=max)) {
                | Ok(quantity) => return Ok(quantity),
                | Err(e) => return Err(e),
            }
        }
    }

    if !can_destroy(world, (target_row, target_col))? {
        return Err(CannotDestroy);
    }
    let max = content.properties().max();
    let cost = content.properties().cost();
    robot.get_energy_mut().consume_energy(cost)?;

    //actually remove content from world
    let content = &mut world.map[target_row][target_col].content;
    let out = content.clone();
    *content = Content::None;

    if max == 0 {
        Ok(0)
    } else {
        match add_to_backpack(robot, out.to_default(), rng.gen_range(0..=max)) {
            | Ok(quantity) => Ok(quantity),
            | Err(e) => Err(e),
        }
    }
}

// Put ----------------------------------------------------
pub fn put(
    robot: &mut impl Runnable,
    world: &mut World,
    content_in: Content,
    quantity: usize,
    direction: Direction,
) -> Result<usize, LibError> {
    // check if the provided content is not None
    if content_in == Content::None {
        return Err(WrongContentUsed);
    }
    // check if you are actually putting something
    if *robot.get_backpack().contents.get(&content_in.to_default()).unwrap() < 1 {
        return Err(WrongContentUsed);
    }
    // check if the target tile is inside the map
    if !go_allowed_row_col(world, get_coords_row_col(robot, &direction)) {
        return Err(LibError::OutOfBounds);
    }
    let (target_row, target_col) = get_coords_row_col(robot, &direction);
    let mut amount = min(
        min(
            quantity,
            *robot.get_backpack().contents.get(&content_in.to_default()).unwrap(),
        ),
        content_in.properties().max(),
    );
    let input = (
        &world.map[target_row][target_col].tile_type,
        &world.map[target_row][target_col].content,
        &content_in,
    );
    match input {
        | (_, Content::Bank(range), Content::Coin(_)) => {
            let (quantity_to_remove, cost) = can_store(
                robot,
                range.end - range.start,
                &input.2.to_default(),
                &input.1.to_default(),
                quantity,
            )?;
            let removed_quantity = remove_from_backpack(robot, &content_in.to_default(), quantity_to_remove)?;
            world.map[target_row][target_col].content = Content::Bank((range.start + removed_quantity)..range.end);
            robot.get_energy_mut().consume_energy(cost)?;
            Ok(removed_quantity)
        }
        | (_, Content::None, Content::Coin(_)) => {
            if !input.0.properties().can_hold(input.2) {
                return Err(LibError::WrongContentUsed);
            }
            let cost = input.2.properties().cost() * amount;
            if !robot.get_energy().has_enough_energy(cost) {
                Err(NotEnoughEnergy)
            } else {
                let removed_quantity = remove_from_backpack(robot, &input.2.to_default(), amount)?;
                robot.get_energy_mut().consume_energy(cost)?;
                world.map[target_row][target_col].content = Content::Coin(amount);
                Ok(removed_quantity)
            }
        }
        | (_, Content::Bin(range), Content::Garbage(_)) => {
            let (quantity_to_remove, cost) = can_store(
                robot,
                range.end - range.start,
                &input.2.to_default(),
                &input.1.to_default(),
                quantity,
            )?;
            let removed_quantity = remove_from_backpack(robot, &content_in.to_default(), quantity_to_remove)?;
            world.map[target_row][target_col].content = Content::Bin((range.start + removed_quantity)..range.end);
            robot.get_energy_mut().consume_energy(cost)?;
            Ok(removed_quantity)
        }
        | (_, Content::None, Content::Garbage(_)) => {
            if !input.0.properties().can_hold(input.2) {
                return Err(LibError::WrongContentUsed);
            }
            let cost = input.2.properties().cost() * amount;
            if !robot.get_energy().has_enough_energy(cost) {
                Err(NotEnoughEnergy)
            } else {
                let removed_quantity = remove_from_backpack(robot, &input.2.to_default(), amount)?;
                robot.get_energy_mut().consume_energy(cost)?;
                world.map[target_row][target_col].content = Content::Garbage(amount);
                Ok(removed_quantity)
            }
        }
        | (_, Content::Crate(range), Content::Tree(_)) => {
            let (quantity_to_remove, cost) = can_store(
                robot,
                range.end - range.start,
                &input.2.to_default(),
                &input.1.to_default(),
                quantity,
            )?;
            let removed_quantity = remove_from_backpack(robot, &content_in.to_default(), quantity_to_remove)?;
            world.map[target_row][target_col].content = Content::Crate((range.start + removed_quantity)..range.end);
            robot.get_energy_mut().consume_energy(cost)?;
            Ok(removed_quantity)
        }
        | (_, Content::None, Content::Tree(_)) => {
            if !input.0.properties().can_hold(input.2) {
                return Err(LibError::WrongContentUsed);
            }
            let cost = input.2.properties().cost() * amount;
            if !robot.get_energy().has_enough_energy(cost) {
                Err(NotEnoughEnergy)
            } else {
                let removed_quantity = remove_from_backpack(robot, &input.2.to_default(), amount)?;
                robot.get_energy_mut().consume_energy(cost)?;
                world.map[target_row][target_col].content = Content::Tree(amount);
                Ok(removed_quantity)
            }
        }
        | (_, Content::Tree(_), Content::Fire) | (_, Content::None, Content::Fire) => {
            if !input.0.properties().can_hold(input.2) {
                return Err(LibError::WrongContentUsed);
            }
            let cost = input.2.properties().cost() * amount;
            if !robot.get_energy().has_enough_energy(cost) {
                Err(NotEnoughEnergy)
            } else {
                let removed_quantity = remove_from_backpack(robot, &input.2.to_default(), 1)?;
                robot.get_energy_mut().consume_energy(cost)?;
                world.map[target_row][target_col].content = input.2.to_default();
                Ok(removed_quantity)
            }
        }
        | (TileType::Grass | TileType::Hill, Content::None, Content::Rock(_)) => {
            let cost = input.2.properties().cost();
            if !robot.get_energy().has_enough_energy(cost) {
                Err(NotEnoughEnergy)
            } else {
                let removed_quantity = remove_from_backpack(robot, &input.2.to_default(), 1)?;
                robot.get_energy_mut().consume_energy(cost)?;
                world.map[target_row][target_col].tile_type = TileType::Street;
                Ok(removed_quantity)
            }
        }
        | (_, Content::None, Content::Rock(_)) => {
            if !input.0.properties().can_hold(input.2) {
                return Err(LibError::WrongContentUsed);
            }
            let cost = input.2.properties().cost() * amount;
            if !robot.get_energy().has_enough_energy(cost) {
                Err(NotEnoughEnergy)
            } else {
                let removed_quantity = remove_from_backpack(robot, &input.2.to_default(), amount)?;
                robot.get_energy_mut().consume_energy(cost)?;
                world.map[target_row][target_col].content = Content::Rock(amount);
                Ok(removed_quantity)
            }
        }
        | (_, Content::Fire, Content::Water(_)) => {
            let cost = input.2.properties().cost();
            if !robot.get_energy().has_enough_energy(cost) {
                Err(NotEnoughEnergy)
            } else {
                let removed_quantity = remove_from_backpack(robot, &input.2.to_default(), 1)?;
                robot.get_energy_mut().consume_energy(cost)?;
                world.map[target_row][target_col].content = Content::None;
                Ok(removed_quantity)
            }
        }
        | (_, Content::None, Content::Water(_)) => Err(WrongContentUsed),
        | (TileType::DeepWater | TileType::ShallowWater, Content::Water(size), Content::Water(_)) => {
            let theoretical_max = input.2.properties().max();
            if size + amount >= theoretical_max {
                amount = theoretical_max - size
            }
            let cost = input.2.properties().cost() * amount;
            if !robot.get_energy().has_enough_energy(cost) {
                Err(NotEnoughEnergy)
            } else {
                let removed_quantity = remove_from_backpack(robot, &input.2.to_default(), amount)?;
                robot.get_energy_mut().consume_energy(cost)?;
                world.map[target_row][target_col].content = Content::Water(size + amount);
                Ok(removed_quantity)
            }
        }
        | (_, Content::Fire, _) => {
            let cost = Content::Water(0).properties().cost() * amount;
            if !robot.get_energy().has_enough_energy(cost) {
                Err(NotEnoughEnergy)
            } else {
                let removed_quantity = remove_from_backpack(robot, &input.2.to_default(), amount)?;
                robot.get_energy_mut().consume_energy(cost)?;
                Ok(removed_quantity)
            }
        }
        | _ => Err(OperationNotAllowed),
    }
}

// Look_at_sky ----------------------------------------------------
/// Given the world, will return the environmental conditions
/// It's used to see the weather conditions and the time of day
///
/// # Usage
/// ```
/// use robotics_lib::interface::look_at_sky;
/// // let environmental_conditions = look_at_sky(robot, world);
/// ```
///
/// # Arguments
/// - 'robot': The robot
/// - `world`: The world in which the robot is
///
/// # Returns
/// - `EnvironmentalConditions`: The environmental conditions struct, which you can use as you wish
pub fn look_at_sky(_: &impl Runnable, world: &mut World) -> EnvironmentalConditions {
    // TODO: decide if the robot should use energy to see weather conditions and time of day
    world.environmental_conditions.clone()
}

// Debug ----------------------------------------------------
/// Given the world, will return the map, the dimension and the position of the robot
/// It's used for debug purposed
pub fn debug(robot: &impl Runnable, world: &mut World) -> (Vec<Vec<Tile>>, usize, Coordinate) {
    (world.map.clone(), world.dimension, robot.get_coordinate().clone())
}

// Robot map -- Shouldn't we call the function robot_map?
/// Given the world, will return the map of the robot
/// It's used as private map for the robot
///
/// # Usage
/// ```
/// use robotics_lib::interface::plot;
/// ```
///
/// # Arguments
/// - `world`: The world in which the robot is
///
/// # Returns
/// - `Vec<Vec<Option<Tile>>>`: The map of the robot
///
/// # Examples
/// ```
/// use robotics_lib::interface::plot;
/// use robotics_lib::world::World;
///
/// fn robot_world_example(mut world: &mut World) {
///     let robot_world=plot(&mut world);
///     for row in robot_world.iter() {
///         for col in row.iter() {
///             match col {
///                 | None => print!("default_unknown_tile"),
///                 | Some(tile) => print!("{:?}", tile.content),
///             }
///         }
///     }
/// }
///
/// ```
///
/// # Remarks
/// - The map of the robot is returned
/// - The map of the robot is a matrix of Option<Tile>
///

//changed from &mut to & because it's not necessary to modify the world
pub fn plot(world: &World) -> Option<Vec<Vec<Option<Tile>>>> {
    let mut out: Vec<Vec<Option<Tile>>> = vec![vec![None; world.dimension]; world.dimension];
    if let Ok(plot_guard) = PLOT.lock() {
        for (x, y) in plot_guard.iter() {
            out[*x][*y] = Some(world.map[*x][*y].clone());
        }
        Some(out)
    } else {
        None
    }
}

// View ----------------------------------------------------

/// Given the world, will return the area around the robot
///
/// # Usage
/// ```
/// use robotics_lib::interface::robot_view;
/// ```
///
/// # Arguments
/// - `world`: The world in which the robot is
///
/// # Returns
/// - `Vec<Vec<Option<Tile>>>`: The area around the robot (3x3)
///
/// # Examples
/// ```
/// use robotics_lib::interface::robot_view;
/// use robotics_lib::runner::Runnable;
/// use robotics_lib::world::World;
/// fn robot_view_example(robot: &impl Runnable, mut world: &mut World) {
///     let robot_view=robot_view(robot, &mut world);
///     for row in robot_view.iter() {
///         for col in row.iter() {
///             match col {
///                 | None => print!("default_unknown_tile"),
///                 | Some(tile) => print!("{:?}", tile.tile_type),
///            }
///         }
///     }
/// }
/// ```
///
/// # Remarks
/// - The area around the robot is returned
/// - The area around the robot is a matrix of Option<Tile>
/// - The area around the robot is a 3x3 matrix
/// - The area around the robot is centered on the robot

pub fn robot_view(robot: &impl Runnable, world: &World) -> Vec<Vec<Option<Tile>>> {
    let mut tmp: [[bool; 3]; 3] = [[false; 3]; 3];
    let mut out: Vec<Vec<Option<Tile>>> = vec![vec![None; 3]; 3]; //Matrix of Option <Tile>
    let (robot_row, robot_col) = (robot.get_coordinate().get_row(), robot.get_coordinate().get_col());

    if robot_row == 0 {
        tmp[0][0] = true;
        tmp[0][1] = true;
        tmp[0][2] = true;
        out[0][0] = None;
        out[0][1] = None;
        out[0][2] = None;
    }
    if robot_col == 0 {
        tmp[0][0] = true;
        tmp[1][0] = true;
        tmp[2][0] = true;
        out[0][0] = None;
        out[1][0] = None;
        out[2][0] = None;
    }
    if robot_row == world.dimension - 1 {
        tmp[2][0] = true;
        tmp[2][1] = true;
        tmp[2][2] = true;
        out[2][0] = None;
        out[2][1] = None;
        out[2][2] = None;
    }
    if robot_col == world.dimension - 1 {
        tmp[0][2] = true;
        tmp[1][2] = true;
        tmp[2][2] = true;
        out[0][2] = None;
        out[1][2] = None;
        out[2][2] = None;
    }

    tmp.iter().enumerate().for_each(|(i, vector)| {
        vector.iter().enumerate().for_each(|(j, elem)| {
            if !elem {
                let row = robot_row + i - 1;
                let col = robot_col + j - 1;
                out[i][j] = Some(world.map[row][col].clone());

                // add to plot
                if let Ok(mut plot_guard) = PLOT.lock() {
                    if !plot_guard.contains(&(row, col)) {
                        plot_guard.push((row, col));
                    }
                }
            }
        })
    });
    out
}

/// Given the world, will return the area around the robot as a matrix of Option<Tile> with the position of the robot
///
/// # Usage
/// ```
/// use robotics_lib::interface::where_am_i;
/// ```
///
/// # Arguments
/// - `world`: The world in which the robot is
/// - `robot`: The robot that is moving around the map
///
/// # Returns
/// - `Vec<Vec<Option<Tile>>>`: The area around the robot (3x3)
/// - `Coordinate`: The position of the robot
///
/// # Examples
/// ```
/// use robotics_lib::interface::where_am_i;
/// use robotics_lib::runner::Runnable;
/// use robotics_lib::world::World;
/// fn where_am_i_example(robot: &impl Runnable, mut world: &mut World) {
///    let (robot_view, robot_position)=where_am_i(robot, &mut world);
///     for row in robot_view.iter() {
///         for col in row.iter() {
///             match col {
///                 | None => print!("default_unknown_tile"),
///                 | Some(tile) => print!("{:?}", tile.tile_type),
///             }
///         }
///     }
/// }
/// ```
///
pub fn where_am_i(robot: &impl Runnable, world: &World) -> (Vec<Vec<Option<Tile>>>, Coordinate) {
    (robot_view(robot, world), robot.get_coordinate().clone())
}
