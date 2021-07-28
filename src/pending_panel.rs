use raylib::core::texture::Texture2D;
use raylib::core::drawing::{RaylibDrawHandle, RaylibDraw};
use raylib::color::Color;
use raylib::core::text::Font;
use std::collections::HashMap;
use crate::textures::get_texture;
use crate::data::SensorData;
use crate::fonts::get_font;
use raylib::prelude::Vector2;
use chrono::Local;

pub fn draw_pending_panel(fonts: &HashMap<String, Font>, textures: &HashMap<String, Texture2D>, d: &mut RaylibDrawHandle, data: &Vec<SensorData>) {

    let background = get_texture(textures, "pending_background");

    d.draw_texture(&background, 0, 0, Color::WHITE);
    d.clear_background(Color::WHITE);

    let hue_data = data.iter()
        .filter(|d| { d.reporter == "hue-sensor-agent"} )
        .collect::<Vec<&SensorData>>();
    let latest_data = hue_data.last();

    if latest_data.is_some() {
        let office_temp: f32 = latest_data.unwrap().values.get("hue_temperature").unwrap_or(&"0".to_string()).parse().unwrap();

        let temp = format!("{:.1}   C", office_temp);
        d.draw_text_ex(get_font(fonts, "calibri_40_bold"), &temp, Vector2::new(457.0, 61.0), 40.0, 0.0, Color::WHITE);
        d.draw_circle(546, 71, 6.0, Color::WHITE);
        d.draw_circle(546, 71, 3.0, Color::BLACK);
    }

    let date = Local::now().format("%H:%M:%S").to_string();
    d.draw_rectangle(570, 278, 153, 44, Color::BLACK);
    d.draw_text_ex(get_font(fonts, "calibri_40_bold"), &date, Vector2::new(575.0, 282.0), 40.0, 0.0, Color::WHITE);

    d.draw_text_ex(get_font(fonts, "calibri_30"), "SENSORPANEL", Vector2::new(427.0, 465.0), 30.0, 0.0, Color::WHITE);
}