use ggez::graphics::Color;

use lazy_static::lazy_static;

use std::collections::HashMap;

use crate::components::MaterialType;

pub const SCREEN_WIDTH: f32 = 800.0;
pub const SCREEN_HEIGHT: f32 = 600.0;
pub const SCALE: f32 = 10.0;

// lazy static allows for the creation of a static hashmap that will be used to quickly query colors for materials
lazy_static! {
    pub static ref COLORS: HashMap<MaterialType, Color> = {
        let mut map: HashMap<MaterialType, Color> = HashMap::new();
        map.insert(MaterialType::Sand, Color::new(1.0, 1.0, 0.0, 1.0));
        map.insert(MaterialType::Water, Color::new(0.0, 0.0, 1.0, 1.0));
        map.insert(MaterialType::Dirt, Color::new(0.75, 0.5, 0.0, 1.0));
        map.insert(MaterialType::Grass, Color::new(0.0, 1.0, 0.0, 1.0));
        map.insert(MaterialType::Nothing, Color::new(0.0, 0.0, 0.0, 1.0));

        map
    };
}
