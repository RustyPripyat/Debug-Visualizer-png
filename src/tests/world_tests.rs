mod coordinate_test {
    use crate::world::coordinates::Coordinate;

    #[test]
    fn test_coordinate_new() {
        let coordinate = Coordinate::new(1, 2);

        assert_eq!(coordinate.get_row(), 1);
        assert_eq!(coordinate.get_col(), 2);
    }
}

#[cfg(test)]
mod environmental_conditions_tests {
    use crate::world::environmental_conditions::{DayTime, EnvironmentalConditions, TimeOfDay, WeatherType};

    #[test]
    fn test_environmental_conditions_new() {
        let weather_forecast = vec![
            WeatherType::Sunny,
            WeatherType::Rainy,
            WeatherType::Foggy,
            WeatherType::TropicalMonsoon,
            WeatherType::TrentinoWinter,
        ];
        let environmental_conditions = EnvironmentalConditions::new(&weather_forecast, 240, 12);

        assert_eq!(environmental_conditions.get_time_of_day(), DayTime::Afternoon);
        assert_eq!(environmental_conditions.get_time_of_day_string(), "12:00".to_owned());
        assert_eq!(environmental_conditions.get_weather_condition(), WeatherType::Sunny);
    }

    #[test]
    fn found_error_with_time_progression() {
        let weather_forecast = vec![WeatherType::Sunny];
        let mut environmental_conditions = EnvironmentalConditions::new(&weather_forecast, 120, 12);

        environmental_conditions.tick();
        assert_eq!(environmental_conditions.get_time_of_day_string(), "14:00".to_owned());

        environmental_conditions.tick();
        assert_eq!(environmental_conditions.get_time_of_day_string(), "16:00".to_owned());
    }

    #[test]
    fn found_error_with_time_progression_2() {
        let weather_forecast = vec![WeatherType::Sunny];
        let mut environmental_conditions = EnvironmentalConditions::new(&weather_forecast, 120, 10);

        environmental_conditions.tick();
        assert_eq!(environmental_conditions.get_time_of_day(), DayTime::Afternoon);
    }

    #[test]
    fn test_environmental_conditions_tick() {
        let weather_forecast = vec![
            WeatherType::Sunny,
            WeatherType::Rainy,
            WeatherType::Foggy,
            WeatherType::TropicalMonsoon,
            WeatherType::TrentinoWinter,
        ];
        let mut environmental_conditions = EnvironmentalConditions::new(&weather_forecast, 60, 12);

        (0..2).into_iter().for_each(|_| environmental_conditions.tick());
        assert_eq!(environmental_conditions.get_time_of_day(), DayTime::Afternoon);
        assert_eq!(environmental_conditions.get_time_of_day_string(), "14:00".to_owned());
        assert_eq!(environmental_conditions.get_weather_condition(), WeatherType::Sunny);

        (0..2).into_iter().for_each(|_| environmental_conditions.tick());
        assert_eq!(environmental_conditions.get_time_of_day(), DayTime::Afternoon);
        assert_eq!(environmental_conditions.get_time_of_day_string(), "16:00".to_owned());
        assert_eq!(environmental_conditions.get_weather_condition(), WeatherType::Sunny);

        (0..2).into_iter().for_each(|_| environmental_conditions.tick());
        assert_eq!(environmental_conditions.get_time_of_day(), DayTime::Afternoon);
        assert_eq!(environmental_conditions.get_time_of_day_string(), "18:00".to_owned());
        assert_eq!(environmental_conditions.get_weather_condition(), WeatherType::Sunny);

        (0..6).into_iter().for_each(|_| environmental_conditions.tick());
        assert_eq!(environmental_conditions.get_time_of_day(), DayTime::Night);
        assert_eq!(environmental_conditions.get_time_of_day_string(), "00:00".to_owned());
        assert_eq!(environmental_conditions.get_weather_condition(), WeatherType::Rainy);
    }

    #[test]
    fn test_environmental_conditions_next_day() {
        let weather_forecast = vec![
            WeatherType::Sunny,
            WeatherType::Rainy,
            WeatherType::Foggy,
            WeatherType::TropicalMonsoon,
        ];
        let mut environmental_conditions = EnvironmentalConditions::new(&weather_forecast, 30, 12);
        environmental_conditions.next_day();
        assert_eq!(environmental_conditions.get_weather_condition(), WeatherType::Rainy);
    }
    #[test]
    fn test_advance() {
        let mut time = TimeOfDay { hour: 0, minute: 0 };

        assert_eq!(time.advance(120), false);
        assert_eq!(time.hour, 2);
        assert_eq!(time.minute, 0);
    }
}

#[cfg(test)]
mod world_struct_tests {
    use crate::world::{
        environmental_conditions::{EnvironmentalConditions, WeatherType},
        tile::{Tile, TileType},
        World,
    };

    #[test]
    fn test_world_new_and_advance_time() {
        let map = vec![vec![Tile {
            tile_type: TileType::Grass,
            content: crate::world::tile::Content::None,
        }]];
        let weather_forecast = vec![
            WeatherType::Sunny,
            WeatherType::Rainy,
            WeatherType::Foggy,
            WeatherType::TropicalMonsoon,
            WeatherType::TrentinoWinter,
        ];
        let environmental_conditions = EnvironmentalConditions::new(&weather_forecast, 60, 12);

        let mut world = World::new(map.clone(), environmental_conditions);

        assert_eq!(world.dimension, map.len());
        (0..12).into_iter().for_each(|_| {
            world.advance_time();
        });

        assert_eq!(
            world.environmental_conditions.get_time_of_day_string(),
            "00:00".to_owned()
        );
        assert_eq!(
            world.environmental_conditions.get_weather_condition(),
            WeatherType::Rainy
        );

        (0..25).into_iter().for_each(|_| {
            world.advance_time();
        });

        assert_eq!(
            world.environmental_conditions.get_time_of_day_string(),
            "01:00".to_owned()
        );
        assert_eq!(
            world.environmental_conditions.get_weather_condition(),
            WeatherType::Foggy
        );
    }
}

#[cfg(test)]
mod world_generator_tests {

    use crate::world::{
        tile::{Content, Tile, TileType},
        worldgenerator::{check_world, get_content_percentage, get_tiletype_percentage},
    };

    #[test]
    fn test_check_world_valid() {
        let world = vec![vec![
            Tile {
                tile_type: TileType::Grass,
                content: Content::None,
            },
            Tile {
                tile_type: TileType::DeepWater,
                content: Content::None,
            },
            Tile {
                tile_type: TileType::ShallowWater,
                content: Content::None,
            },
            Tile {
                tile_type: TileType::Sand,
                content: Content::None,
            },
            Tile {
                tile_type: TileType::Street,
                content: Content::None,
            },
            Tile {
                tile_type: TileType::Hill,
                content: Content::None,
            },
            Tile {
                tile_type: TileType::Mountain,
                content: Content::None,
            },
            Tile {
                tile_type: TileType::Snow,
                content: Content::None,
            },
            Tile {
                tile_type: TileType::Lava,
                content: Content::None,
            },
        ]];

        let result = check_world(&world);
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn test_check_world_invalid() {
        let world = vec![vec![Tile {
            tile_type: TileType::Grass,
            content: Content::Rock(5),
        }]];

        let result = check_world(&world);
        assert_eq!(result, Ok(false));
    }

    #[test]
    fn test_get_tiletype_percentage() {
        let world = vec![vec![
            Tile {
                tile_type: TileType::Grass,
                content: Content::None,
            },
            Tile {
                tile_type: TileType::Grass,
                content: Content::Rock(5),
            },
            Tile {
                tile_type: TileType::ShallowWater,
                content: Content::Rock(5),
            },
        ]];

        let result = get_tiletype_percentage(&world);
        assert_eq!(result.get(&TileType::Grass), Some(&(2_f64 / 3_f64)));
        assert_eq!(result.get(&TileType::ShallowWater), Some(&(1_f64 / 3_f64)));
        assert_eq!(result.get(&TileType::DeepWater), None);
    }

    #[test]
    fn test_get_content_percentage() {
        let world = vec![vec![
            Tile {
                tile_type: TileType::Grass,
                content: Content::Rock(3),
            },
            Tile {
                tile_type: TileType::Grass,
                content: Content::Bin(0..2),
            },
            Tile {
                tile_type: TileType::Grass,
                content: Content::Tree(1),
            },
        ]];
        println!("{:?}", world);
        let result = get_content_percentage(&world);
        println!("{:?}", result);
        assert_eq!(result.get(&Content::Bin(0..2)), Some(&(1_f64 / 3_f64)));
        assert_eq!(result.get(&Content::Rock(3)), Some(&(1_f64 / 3_f64)));
        assert_eq!(result.get(&Content::Tree(1)), Some(&(1_f64 / 3_f64)));
    }
}
