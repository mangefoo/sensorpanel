use raylib::core::texture::Texture2D;
use raylib::core::drawing::{RaylibDrawHandle, RaylibDraw};
use raylib::color::Color;
use crate::widgets::{draw_cpu_panel, draw_gpu_panel, draw_mem_panel};
use raylib::core::text::Font;
use std::collections::HashMap;
use crate::textures::get_texture;

pub fn draw_windows_panel(fonts: &HashMap<String, Font>, textures: &HashMap<String, Texture2D>, mut d: &mut RaylibDrawHandle) {

    let background = get_texture(textures, "windows_background");

    d.draw_texture(&background, 0, 0, Color::WHITE);
    d.clear_background(Color::WHITE);

    draw_cpu_panel(&mut d, 0, 5, &fonts);
    draw_gpu_panel(&mut d, 0, 205, &fonts);
    draw_mem_panel(&mut d, 0, 405, &fonts);
}