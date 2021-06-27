use raylib::prelude::*;
use std::collections::HashMap;
use crate::fonts::get_font;
use crate::data::SensorData;
use reqwest::get;

pub fn draw_cpu_panel(mut d: &mut RaylibDrawHandle, x: i32, y: i32, fonts: &HashMap<String, Font>, data: &SensorData) {

    let xf = x as f32;
    let yf = y as f32;

    let cpu_utilization: f32 = data.values.get("cpu_utilization").unwrap_or(&"0".to_string()).parse().unwrap();
    let cpu_die_temp: f32 = data.values.get("cpu_die_temp").or(Some(&"0".to_string())).expect("No cpu_die_temp value").parse().unwrap();
    let cpu_package_temp: f32 = data.values.get("cpu_package_temp").or(Some(&"0".to_string())).expect("No cpu_package_temp value").parse().unwrap();
    let cpu_power: f32 = data.values.get("cpu_power").or(Some(&"0".to_string())).expect("No cpu_power value").parse().unwrap();
    let cpu_voltage: f32 = data.values.get("cpu_voltage").or(Some(&"0".to_string())).expect("No cpu_voltage value").parse().unwrap();
    let cpu_frequency: f32 = data.values.get("cpu_frequency").or(Some(&"0".to_string())).expect("No cpu_frequency value").parse().unwrap();

    d.draw_text_ex(get_font(fonts, "calibrib"), "CPU", Vector2::new(xf + 10.0, yf + 10.0), 50.0, 0.0, Color::WHITE);
    d.draw_text_ex(get_font(fonts, "calibri"), &*format!("{:.2} W", cpu_power), Vector2::new(xf + 110.0, yf + 12.0), 20.0, 0.0, Color::WHITE);
    d.draw_text_ex(get_font(fonts, "calibri"), &*format!("{:.2} V", cpu_voltage), Vector2::new(xf + 110.0, yf + 30.0), 20.0, 0.0, Color::WHITE);

    draw_gauge(&mut d, x + 200, y + 5, cpu_die_temp as i32, get_font(fonts, "calibri"));
    draw_gauge(&mut d, x + 275, y + 5, cpu_package_temp as i32, get_font(fonts,"calibri"));
    d.draw_text_ex(get_font(fonts, "calibril"), &*format!("{:.0} MHz", cpu_frequency), Vector2::new(xf + 340.0, yf + 18.0), 30.0, 0.0, Color::WHITE);

    d.draw_text_ex(get_font(fonts, "calibrib2"), "Usage", Vector2::new(xf + 10.0, yf + 65.0), 25.0, 0.0, Color::WHITE);
    let gradient_color_1 = Color::new(0, 200, 0, 255);
    let gradient_color_2 = Color::new(0, 40, 0, 255);
    draw_meter_bar(&mut d, x + 80, y + 65, 390, 23, cpu_utilization as i32, 100, (gradient_color_1, gradient_color_2), fonts);

    draw_graph_grid(&mut d, x + 10, y + 100)
}

pub fn draw_gpu_panel(mut d: &mut RaylibDrawHandle, x: i32, y: i32, fonts: &HashMap<String, Font>, data: &SensorData) {

    let xf = x as f32;
    let yf = y as f32;

    let gpu_utilization: f32 = data.values.get("gpu_utilization").or(Some(&"0".to_string())).expect("No gpu_utilization value").parse().unwrap();
    let gpu_die_temp: f32 = data.values.get("gpu_die_temp").or(Some(&"0".to_string())).expect("No gpu_die_temp value").parse().unwrap();
    let gpu_package_temp: f32 = data.values.get("gpu_package_temp").or(Some(&"0".to_string())).expect("No gpu_package_temp value").parse().unwrap();
    let gpu_power: f32 = data.values.get("gpu_power").or(Some(&"0".to_string())).expect("No gpu_power value").parse().unwrap();
    let gpu_voltage: f32 = data.values.get("gpu_voltage").or(Some(&"0".to_string())).expect("No gpu_voltage value").parse().unwrap();
    let gpu_frequency: f32 = data.values.get("gpu_frequency").or(Some(&"0".to_string())).expect("No gpu_frequency value").parse().unwrap();

    d.draw_text_ex(get_font(fonts, "calibrib"), "GPU", Vector2::new(xf + 10.0, yf + 10.0), 50.0, 0.0, Color::WHITE);
    d.draw_text_ex(get_font(fonts, "calibri"), &*format!("{:.2} W", gpu_power), Vector2::new(xf + 110.0,  yf + 12.0), 20.0, 0.0, Color::WHITE);
    d.draw_text_ex(get_font(fonts, "calibri"), &*format!("{:.2} V", gpu_voltage), Vector2::new(xf + 110.0, yf + 30.0), 20.0, 0.0, Color::WHITE);

    draw_gauge(&mut d, x + 200, y + 5, gpu_die_temp as i32, get_font(fonts, "calibri"));
    draw_gauge(&mut d, x + 275, y + 5, gpu_package_temp as i32, get_font(fonts,"calibri"));

    d.draw_text_ex(get_font(fonts, "calibril"), &*format!("{} MHz", gpu_frequency), Vector2::new(xf + 340.0, yf + 18.0), 30.0, 0.0, Color::WHITE);
    let gradient_color_1 = Color::new(200, 0, 0, 255);
    let gradient_color_2 = Color::new(40, 0, 0, 255);
    d.draw_text_ex(get_font(fonts, "calibrib2"), "Usage", Vector2::new(xf + 10.0, yf + 65.0), 25.0, 0.0, Color::WHITE);
    draw_meter_bar(&mut d, x + 80, y + 65, 390, 23, gpu_utilization as i32, 100, (gradient_color_1, gradient_color_2), fonts);

    draw_graph_grid(&mut d, x + 10, y + 100)
}

pub fn draw_mem_panel(mut d: &mut RaylibDrawHandle, x: i32, y: i32, fonts: &HashMap<String, Font>) {

    let xf = x as f32;
    let yf = y as f32;

    d.draw_text_ex(get_font(fonts, "calibri_bold_40"), "Mem", Vector2::new(xf + 10.0, yf + 10.0), 40.0, 0.0, Color::WHITE);
    d.draw_text_ex(get_font(fonts, "calibri"), "94.27 W", Vector2::new(xf + 110.0,  yf + 12.0), 20.0, 0.0, Color::WHITE);
    d.draw_text_ex(get_font(fonts, "calibri"), "1.232 V", Vector2::new(xf + 110.0, yf + 30.0), 20.0, 0.0, Color::WHITE);

    draw_gauge(&mut d, x + 200, y + 5, 75, get_font(fonts, "calibri"));
    draw_gauge(&mut d, x + 275, y + 5, 99, get_font(fonts,"calibri"));

    d.draw_text_ex(get_font(fonts, "calibril"), "3600 MHz", Vector2::new(xf + 340.0, yf + 18.0), 30.0, 0.0, Color::WHITE);

    d.draw_text_ex(get_font(fonts, "calibrib2"), "Usage", Vector2::new(xf + 10.0, yf + 65.0), 25.0, 0.0, Color::WHITE);
    draw_meter_bar(&mut d, x + 80, y + 65, 390, 23, 50, 100, (Color::ORANGE, Color::BLACK), fonts);

    draw_graph_grid(&mut d, x + 10, y + 100)
}

pub fn draw_graph_grid(d: &mut &mut RaylibDrawHandle, x: i32, y: i32) {
    let grid_color = Color::new(49, 50, 50, 255);

    d.draw_rectangle(x, y, 460, 80, Color::DARKGRAY);
    d.draw_rectangle(x + 1, y + 1, 458, 78, Color::BLACK);

    for i in 0..(460 / 10) {
        d.draw_line(x + 9 + i * 10, y + 1, x + 9 + i * 10, y + 79, grid_color);
    }

    for i in 0..(80 / 10) {
        d.draw_line(x, y + 1 + i * 10 + 1, x + 458, y + 1 + i * 10 + 1, grid_color);
    }
}

pub fn draw_meter_bar(d: &mut &mut RaylibDrawHandle, x: i32, y: i32, width: i32, height: i32, value: i32, max_value: i32, color: (Color, Color), fonts: &HashMap<String, Font>) {

    d.draw_rectangle(x, y, width, height, Color::DARKGRAY);
    d.draw_rectangle(x + 1, y + 1, width - 2, height - 2, Color::BLACK);

    let bar_width = width * value / max_value;
    d.draw_rectangle_gradient_v(x + 1, y + 1, bar_width, height - 2, color.0, color.1);

    let percent = format!("{} %", value.to_string());
    d.draw_text_ex(get_font(fonts, "calibri"), &percent, Vector2::new((x - 15) as f32 + width as f32 / 2.0, y as f32 + 3.0), 20.0, 0.0, Color::WHITE);
}

pub fn draw_gauge(d: &mut RaylibDrawHandle, x: i32, y: i32, value: i32, font: &Font) {
    d.draw_circle(x + 25, y + 25, 25.0, Color::LIGHTGRAY);
    d.draw_circle(x + 25, y + 25, 23.0, Color::BLACK);

    let end_angle = 280 * value / 100;
    let color = match value {
        v if v > 80 => Color::RED,
        v if v > 70 => Color::ORANGE,
        _ => Color::GREEN
    };

    d.draw_circle_sector(Vector2::new(x as f32 + 25.0, y as f32 + 25.0), 20.0, 680 - end_angle, 680, 1000, color);
    d.draw_circle(x + 25, y + 25, 13.0, Color::BLACK);

    d.draw_text_ex(font, &value.to_string(), Vector2::new(x as f32 + 15.0, y as f32 + 17.0), 20.0, 0.0, Color::WHITE);
}
