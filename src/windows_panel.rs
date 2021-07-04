use raylib::core::texture::Texture2D;
use raylib::core::drawing::{RaylibDrawHandle, RaylibDraw};
use raylib::color::Color;
use crate::widgets::{draw_cpu_panel, draw_gpu_panel, draw_mem_panel, draw_core_panel, draw_time_panel};
use raylib::core::text::Font;
use std::collections::HashMap;
use crate::textures::get_texture;
use crate::data::SensorData;

pub fn draw_windows_panel(fonts: &HashMap<String, Font>, textures: &HashMap<String, Texture2D>, mut d: &mut RaylibDrawHandle, data: &SensorData, historical_cpu_load: &Vec<f32>, historical_gpu_load: &Vec<f32>) {

    let background = get_texture(textures, "windows_background");

    d.draw_texture(&background, 0, 0, Color::WHITE);
    d.clear_background(Color::WHITE);

    draw_cpu_panel(&mut d, 10, 5, &fonts, data, historical_cpu_load);
    draw_gpu_panel(&mut d, 10, 207, &fonts, data, historical_gpu_load);
    draw_mem_panel(&mut d, 10, 409, &fonts, data);
    draw_core_panel(&mut d, 530, 5, &fonts, data);
    draw_time_panel(&mut d, 845, 570, &fonts, data);
}