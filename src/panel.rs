use std::collections::HashMap;
use raylib::core::text::Font;
use raylib::core::texture::Texture2D;
use raylib::core::drawing::RaylibDrawHandle;
use crate::data::SensorData;

pub trait Panel {
    fn draw(fonts: &HashMap<String, Font>, textures: &HashMap<String, Texture2D>, d: &mut RaylibDrawHandle, data: &Vec<SensorData>);
}