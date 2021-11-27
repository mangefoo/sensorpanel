use raylib::core::texture::Texture2D;
use raylib::core::drawing::{RaylibDrawHandle, RaylibDraw};
use raylib::color::Color;
use crate::linux_widgets::{draw_cpu_panel, draw_gpu_panel, draw_mem_panel, draw_core_panel, draw_time_panel, draw_net_panel};
use raylib::core::text::Font;
use std::collections::HashMap;
use crate::textures::get_texture;
use crate::data::SensorData;
use crate::panel::Panel;

pub(crate) struct LinuxPanel();

impl Panel for LinuxPanel {
    fn draw(fonts: &HashMap<String, Font>, textures: &HashMap<String, Texture2D>, mut d: &mut RaylibDrawHandle, data: &Vec<SensorData>) {
        let background = get_texture(textures, "linux_background");

        d.draw_texture(&background, 0, 0, Color::WHITE);
        d.clear_background(Color::WHITE);

        let linux_data = data.iter()
            .filter(|d| { d.reporter == "linux-sensor-agent" })
            .collect::<Vec<&SensorData>>();

        let hue_data = data.iter()
            .filter(|d| { d.reporter == "hue-sensor-agent" })
            .collect::<Vec<&SensorData>>();

        if !linux_data.is_empty() {
            draw_cpu_panel(&mut d, 10, 5, &fonts, &linux_data);
            draw_gpu_panel(&mut d, 10, 207, &fonts, &linux_data);
            draw_net_panel(&mut d, 10, 409, &fonts, &linux_data);
            draw_core_panel(&mut d, 530, 5, &fonts, &linux_data);
            draw_mem_panel(&mut d, 520, 320, &fonts, &linux_data);
        }

        draw_time_panel(&mut d, 530, 560, &fonts, &hue_data);
    }
}