pub mod backpack;

use super::energy::{Energy, MAX_ENERGY_LEVEL};
use crate::runner::backpack::BackPack;
use crate::tests::view_interface_test;
use crate::utils::LibError;
use crate::world::coordinates::Coordinate;
use crate::world::worldgenerator::Generator;
use crate::world::World;

pub struct Robot {
    pub energy: Energy,
    pub coordinate: Coordinate,
    pub backpack: BackPack,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            energy: Energy::new(MAX_ENERGY_LEVEL),
            coordinate: Coordinate::new(0, 0),
            backpack: BackPack::new(0),
        }
    }
}

impl Default for Robot {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents the necessary functionality for a robot to be able to run
/// The `Runnable` trait is used to define the necessary functionality for a robot to be able to run.
///
/// # Usage
/// ```
/// use robotics_lib::runner::{Runnable};
/// ```
///
/// # Example
/// ```
/// use robotics_lib::energy::Energy;
/// use robotics_lib::runner::{Robot, Runnable};
/// use robotics_lib::runner::backpack::BackPack;
/// use robotics_lib::world::coordinates::Coordinate;
/// use robotics_lib::world::World;
/// struct MyRobot(Robot);
/// impl Runnable for MyRobot{
///     fn process_tick(&mut self, world: &mut World) {
///         // do something
///     }
///     fn get_energy(&self) -> &Energy {
///         &self.0.energy
///     }
///     fn get_energy_mut(&mut self) -> &mut Energy {
///         &mut self.0.energy
///     }
///     fn get_coordinate(&self) -> &Coordinate {
///        &self.0.coordinate
///     }
///     fn get_coordinate_mut(&mut self) -> &mut Coordinate{
///         &mut self.0.coordinate
///     }
///     fn get_backpack(&self) -> &BackPack {
///         &self.0.backpack
///     }
///     fn get_backpack_mut(&mut self) -> &mut BackPack {
///         &mut self.0.backpack
///     }
/// }
/// ```
///
pub trait Runnable {
    fn process_tick(&mut self, world: &mut World);
    fn get_energy(&self) -> &Energy;
    fn get_energy_mut(&mut self) -> &mut Energy;
    fn get_coordinate(&self) -> &Coordinate;
    fn get_coordinate_mut(&mut self) -> &mut Coordinate;
    fn get_backpack(&self) -> &BackPack;
    fn get_backpack_mut(&mut self) -> &mut BackPack;
}

/// Runs the robot
/// The `run` function is used to run the robot.
///
/// # Usage
/// ```
/// use robotics_lib::runner::{run};
/// ```
///
/// # Example
/// ```
/// use robotics_lib::runner::{run, Runnable};
/// use robotics_lib::world::worldgenerator::Generator;
/// fn run_example(robot: &mut impl Runnable, generator: &mut impl Generator){
///      run(robot, generator);
/// }
/// ```
/// # Remarks
/// - The robot is initialized with the default values

pub fn run(robot: &mut impl Runnable, generator: &mut impl Generator) -> Result<(), LibError> {
    // world initialization
    // check that each tile content respects the hashmap of contentprops

    let (map, (robot_x, robot_y), environmental_conditions) = generator.gen();

    // match check_world(&map) {
    //     | Ok(ret_type) => {
    //         if !ret_type {
    //             return Err(LibError::InvalidWorld);
    //         }
    //     }
    //     | Err(e) => {
    //         return Err(e);
    //     }
    // }

    *robot.get_coordinate_mut() = Coordinate::new(robot_x, robot_y);

    let mut world = World::new(map, environmental_conditions);

    robot.get_backpack_mut().size = 20;
    view_interface_test(robot, &world);

    const ITERATION_LOOPS: usize = 1;

    for _i in 0..ITERATION_LOOPS {
        world.advance_time();
        robot.process_tick(&mut world);
        robot.get_energy_mut().recharge_energy(1);
        view_interface_test(robot, &world);
    }
    Ok(())
}
