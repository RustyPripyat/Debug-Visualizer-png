use crate::interface::Direction;
use crate::runner::Runnable;
use crate::utils::LibError::{CannotWalk, NoContent, OutOfBounds};
use crate::world::tile::Content;
use crate::world::World;
use std::cmp::min;

use minifb::{WindowOptions, Window};
use image::{Rgb, RgbImage};
use crate::world::tile::Tile;
use crate::world::tile::TileType;

/// It contains all the errors that can be returned by the library
///
/// # Variants
/// - `NotEnoughEnergy`: The robot doesn't have enough energy to do the action
/// - `OutOfBounds`: The robot couldn't be moved
/// - `NoContent`: The content doesn't exist
/// - `NoContentProp`: The content doesn't have a property
///
/// # Examples
/// ```
/// use robotics_lib::utils::LibError;
/// fn catch_error(error: LibError) {
///     match error {
///         | LibError::NotEnoughEnergy => println!("Not enough energy"),
///         | LibError::OutOfBounds => println!("Out of bounds"),
///         | LibError::NoContent => println!("No content"),
///         | LibError::NoContentProp => println!("No content prop"),
///         | LibError::NotEnoughSpace(remainder) => println!("Not enough space: {}", remainder),
///         | LibError::CannotDestroy => println!("Cannot destroy"),
///         _ => {}
///     }
/// }
/// ```
///
/// # Remarks
/// - The errors are returned by the functions of the library
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum LibError {
    NotEnoughEnergy,
    OutOfBounds,
    NoContent,
    NoContentProp,
    NoTileTypeProp,
    NotEnoughSpace(usize),
    CannotDestroy,
    InvalidWorld,
    CannotWalk,
    WrongContentUsed,
    NotEnoughContentProvided,
    OperationNotAllowed,
    // others
}

/// This function is used to check if the robot can go in the direction passed as argument
///
/// # Arguments
/// - robot: The robot that has to move
/// - direction: The direction the robot wants to go
///
///
/// # Returns
/// Result<bool, Liberror>
/// bool: the robot can go in the direction passed as argument
///
/// # Errors:
/// - `NoTileTypeProps`: The TileTypeProp of the target cell is not set properly
/// - `OutOfBounds`: The robot couldn't be moved cause it's on the border an the chosen direction is out of bounds
/// - `CannotWalk`: The robot cannot walk on the desired tiletype
///
/// # Examples
///
/// ```
/// use robotics_lib::interface::Direction;
/// use robotics_lib::runner::Runnable;
/// use robotics_lib::utils::LibError;
/// use robotics_lib::world::World;
/// fn go_allowed_example(robot: &impl Runnable, world: &World, direction: &Direction) -> Result<bool, LibError>{
///     match robotics_lib::utils::go_allowed(robot, world, direction) {
///         | Ok(boolean) => {
///             if boolean {
///                 print!("Go allowed");
///                 Ok(true)
///             } else {
///                 print!("Go not allowed");
///                 Ok(false)
///             }
///         }
///         |Err(e) => {Err(e)}
///     }
/// }
///
/// ```
pub fn go_allowed(robot: &impl Runnable, world: &World, direction: &Direction) -> Result<bool, LibError> {
    let (robot_row, robot_col) = (robot.get_coordinate().get_row(), robot.get_coordinate().get_col());

    let inside_bounds = match direction {
        | Direction::Up => robot_col != 0,
        | Direction::Down => robot_col != world.dimension - 1,
        | Direction::Left => robot_row != 0,
        | Direction::Right => robot_row != world.dimension - 1,
    };
    if !inside_bounds {
        return Err(OutOfBounds);
    }

    let walk = world.map[robot_row][robot_col].tile_type.properties().walk();
    if !walk {
        return Err(CannotWalk);
    }

    Ok(walk)
}

/// This function is used to check if the robot can go in the direction passed as argument
///
/// # Arguments
///
/// - robot: The robot that has to move
/// - row_col: (row, col) coordinates
///
/// returns: bool
///
/// # Examples
///
/// ```
/// use robotics_lib::interface::Direction;
/// use robotics_lib::runner::Runnable;
/// use robotics_lib::world::World;
/// fn go_allowed_example(world: &World, row_col:(usize,usize)) {
///     if robotics_lib::utils::go_allowed_row_col(world, row_col) {
///         print!("Go allowed");
///     } else {
///         print!("Go not allowed");
///     }
/// }
///
/// ```
pub fn go_allowed_row_col(world: &World, row_col: (usize, usize)) -> bool {
    let (row, col) = row_col;
    row < world.dimension && col < world.dimension
}

/// This function returns the coordinates of the direction respect to the position of the robot
///
/// # Arguments
/// - robot: The robot
/// - direction: The direction of which you want to know the coordinates
///
/// # Returns
/// (usize, usize): The coordinates of the direction respect to the position of the robot
///
pub(crate) fn get_coords_row_col(robot: &impl Runnable, direction: &Direction) -> (usize, usize) {
    let robot_row = robot.get_coordinate().get_row();
    let robot_col = robot.get_coordinate().get_col();
    match direction {
        | Direction::Up => (robot_row - 1, robot_col),
        | Direction::Down => (robot_row + 1, robot_col),
        | Direction::Left => (robot_row, robot_col - 1),
        | Direction::Right => (robot_row, robot_col + 1),
    }
}

/// This function returns the capability to destroy or not the content
///
/// # Arguments
/// - world: The world where the robot is
/// - row_col: (row, col) coordinates
///
/// # Returns
/// -Ok(()) if the content can be destroyed <br>
/// -Err(LibError) otherwise
///
/// # Errors
/// -OutOfBounds: The coordinates are out of bounds <br>
/// -CannotDestroy: The content cannot be destroyed
pub(crate) fn can_destroy(world: &World, row_col: (usize, usize)) -> Result<bool, LibError> {
    if !go_allowed_row_col(world, row_col) {
        return Err(LibError::OutOfBounds);
    }
    Ok(world.map[row_col.0][row_col.1].content.properties().destroy())
}

/// This function let's you put content in the backpack
///
/// # Arguments
/// - robot: The robot that has to put the content in the backpack
/// - content: The content that has to be put in the backpack
/// - quantity: The quantity of the content that has to be put in the backpack
///
/// # Returns
/// Result<usize, LibError>
///
/// # Errors
/// - NotEnoughSpace: There is not enough space in the backpack
pub(crate) fn add_to_backpack(robot: &mut impl Runnable, content: Content, quantity: usize) -> Result<usize, LibError> {
    let remainder = robot.get_backpack().size - robot.get_backpack().contents.values().sum::<usize>();
    if remainder >= quantity {
        *robot
            .get_backpack_mut()
            .contents
            .entry(content.to_default())
            .or_insert(0) += quantity;
        Ok(quantity)
    } else {
        *robot
            .get_backpack_mut()
            .contents
            .entry(content.to_default())
            .or_insert(0) += remainder;
        Err(LibError::NotEnoughSpace(remainder))
    }
}

pub(crate) fn remove_from_backpack(
    robot: &mut impl Runnable,
    content: &Content,
    quantity: usize,
) -> Result<usize, LibError> {
    match robot.get_backpack_mut().contents.get_mut(&content.to_default()) {
        | None => Err(NoContent),
        | Some(value) => {
            if 0_usize == *value {
                // the robot doesn't have the value in its backpack
                Err(NoContent)
            } else if *value <= quantity {
                // an interface wants to remove more content than the robot actually has
                let tmp = *value;
                *value = 0;
                Ok(tmp)
            } else {
                // the robot has enough content to remove
                *value -= quantity;
                Ok(quantity)
            }
        }
    }
}

pub(crate) fn can_store(
    robot: &mut impl Runnable,
    available_space: usize,
    content_in: &Content,
    content: &Content,
    quantity: usize,
) -> Result<(usize, usize), LibError> {
    let cost = content.properties().cost();
    let quantity_to_remove = min(
        min(
            available_space,
            *robot.get_backpack().contents.get(&content_in.to_default()).unwrap(),
        ),
        quantity,
    );
    // check if there is enough energy
    if !robot.get_energy().has_enough_energy(cost * quantity_to_remove) {
        return Err(LibError::NotEnoughEnergy);
    }
    Ok((quantity_to_remove, cost * quantity_to_remove))
}


pub fn create_image_from_tiles(tiles: &[Vec<Tile>], bot_position: (usize, usize)) -> RgbImage {
    const TILE_SIZE: u32 = 30;
    let WIDTH: u32 = tiles[0].len() as u32 * TILE_SIZE;
    let HEIGHT: u32 = tiles.len() as u32 * TILE_SIZE;

    let mut img = RgbImage::new(WIDTH, HEIGHT);


    const COLOR_DEEP_WATER: Rgb<u8> = Rgb([0, 0, 255]);       // DeepWater (Blu scuro)
    const COLOR_SHALLOW_WATER: Rgb<u8> = Rgb([0, 100, 255]);  // ShallowWater (Blu chiaro)
    const COLOR_SAND: Rgb<u8> = Rgb([255, 255, 0]);           // Sand (Giallo)
    const COLOR_GRASS: Rgb<u8> = Rgb([0, 255, 0]);            // Grass (Verde)
    const COLOR_STREET: Rgb<u8> = Rgb([128, 128, 128]);       // Street (Grigio)
    const COLOR_HILL: Rgb<u8> = Rgb([100, 100, 100]);         // Hill (Grigio scuro)
    const COLOR_MOUNTAIN: Rgb<u8> = Rgb([169, 169, 169]);     // Mountain (Grigio chiaro)
    const COLOR_SNOW: Rgb<u8> = Rgb([255, 255, 255]);         // Snow (Bianco)
    const COLOR_LAVA: Rgb<u8> = Rgb([255, 0, 0]);             // Lava (Rosso)

    for (y, row) in tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let color = match tile.tile_type {
                TileType::DeepWater => COLOR_DEEP_WATER,
                TileType::ShallowWater => COLOR_SHALLOW_WATER,
                TileType::Sand => COLOR_SAND,
                TileType::Grass => COLOR_GRASS,
                TileType::Street => COLOR_STREET,
                TileType::Hill => COLOR_HILL,
                TileType::Mountain => COLOR_MOUNTAIN,
                TileType::Snow => COLOR_SNOW,
                TileType::Lava => COLOR_LAVA,
            };

            // Disegna la tile sull'immagine
            for dy in 0..TILE_SIZE {
                for dx in 0..TILE_SIZE {
                    img.put_pixel(
                        x as u32 * TILE_SIZE + dx,
                        y as u32 * TILE_SIZE + dy,
                        color,
                    );
                }
            }
        }
    }

    // Disegna il simbolo del bot sulla sua posizione (occupando l'intera cella)
    let (bot_x, bot_y) = bot_position;
    let bot_color = Rgb([255, 0, 0]); // Ad esempio, rosso per il bot

    for dy in 0..TILE_SIZE {
        for dx in 0..TILE_SIZE {
            img.put_pixel(
                (bot_x as u32 * TILE_SIZE + dx),
                (bot_y as u32 * TILE_SIZE + dy),
                bot_color,
            );
        }
    }

    img
}

pub fn print_world(tiles: &[Vec<Tile>], bot_position: (usize, usize)) {
    let img = create_image_from_tiles(tiles, bot_position);

    let width = img.width() as usize;
    let height = img.height() as usize;

    // Crea una finestra per mostrare l'immagine
    let mut window = Window::new(
        "Image Viewer",
        width,
        height,
        WindowOptions::default(),
    )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    let pixels_u32: Vec<u32> = img
        .pixels()
        .flat_map(|p| p.0.iter().map(|&channel| channel as u32))
        .collect();

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        window
            .update_with_buffer(&pixels_u32, width, height)
            .unwrap_or_else(|e| println!("{}", e));
    }
}