use crate::fonts::load_fonts;
use crate::windows_panel::draw_windows_panel;
use crate::textures::load_textures;
use crate::data::{SensorData};
use std::collections::HashMap;
use rand::Rng;

extern crate rand;

mod fonts;
mod textures;
mod widgets;
mod windows_panel;
mod data;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1024, 600)
        .title("SensorPanel")
        .build();

    rl.set_target_fps(1);

    let fonts = load_fonts(&mut rl, &thread);
    let textures = load_textures(&mut rl, &thread);

    let mut rng = rand::thread_rng();

    while !rl.window_should_close() {
        let mut values = HashMap::new();
        values.insert("cpu_utilization".to_string(), rng.gen_range(1..100).to_string());
        values.insert("cpu_die_temp".to_string(), rng.gen_range(30..100).to_string());
        values.insert("cpu_package_temp".to_string(), rng.gen_range(30..100).to_string());
        values.insert("cpu_power".to_string(), rng.gen_range(20.0..250.0).to_string());
        values.insert("cpu_voltage".to_string(), rng.gen_range(1.0..2.5).to_string());
        values.insert("cpu_frequency".to_string(), rng.gen_range(0..4900).to_string());

        values.insert("gpu_utilization".to_string(), rng.gen_range(1..100).to_string());
        values.insert("gpu_die_temp".to_string(), rng.gen_range(30..100).to_string());
        values.insert("gpu_package_temp".to_string(), rng.gen_range(30..100).to_string());
        values.insert("gpu_power".to_string(), rng.gen_range(20.0..250.0).to_string());
        values.insert("gpu_voltage".to_string(), rng.gen_range(1.0..2.5).to_string());
        values.insert("gpu_frequency".to_string(), rng.gen_range(0..3000).to_string());

        let sensor_data = SensorData {
            values
        };

        let mut d = rl.begin_drawing(&thread);
        draw_windows_panel(&fonts, &textures, &mut d, &sensor_data);
    }
}

