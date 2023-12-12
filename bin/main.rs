use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::World;
use robotics_lib::world::world_generator::Generator;

use exclusion_zone::content::bank::BankSettings;
use exclusion_zone::content::bin::BinSettings;
use exclusion_zone::content::fire::FireSettings;
use exclusion_zone::content::garbage::GarbageSettings;
use exclusion_zone::content::wood_crate::CrateSettings;
use exclusion_zone::generator::{NoiseSettings, Thresholds};
use exclusion_zone::tile_type::lava::LavaSettings;

mod visualizer;

fn main() {
    struct MyRobot(Robot);

    impl Runnable for MyRobot {
        fn process_tick(&mut self, _world: &mut World) {
            // Do nothing
        }

        fn handle_event(&mut self, _: Event) {
            todo!()
        }
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

    let _r = MyRobot(Robot::new());
    let size = 1000;
    let mut generator = exclusion_zone::generator::WorldGenerator::new(
        size,
        NoiseSettings::default(),
        Thresholds::default(),
        LavaSettings::default(size),
        BankSettings::default(size),
        BinSettings::default(size),
        CrateSettings::default(size),
        GarbageSettings::default(size),
        FireSettings::default(size),
    );

    let tiles = generator.gen().0;

    visualizer::save_world_image(&tiles, (0, 0), "img.png", 4);
}
