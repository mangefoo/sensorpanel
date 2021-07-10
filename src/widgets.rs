use raylib::prelude::*;
use std::collections::HashMap;
use crate::fonts::get_font;
use crate::data::SensorData;
use chrono::Local;

pub fn draw_cpu_panel(mut d: &mut RaylibDrawHandle, x: i32, y: i32, fonts: &HashMap<String, Font>, data: &Vec<SensorData>) {

    let xf = x as f32;
    let yf = y as f32;
    let latest_data = &data[data.len() - 1];

    let max_core_frequency = (1..=16).into_iter()
        .map(|core_number| format!("cpu_core_frequency_{}", core_number))
        .map(|core_key| latest_data.values.get(&core_key))
        .filter(|core_value| core_value.is_some())
        .map(|core_value| core_value.unwrap().parse::<f32>().unwrap() as i32)
        .max()
        .unwrap_or(0);

    let cpu_utilization: f32 = latest_data.values.get("cpu_utilization").unwrap_or(&"0".to_string()).parse().unwrap();
    let cpu_die_temp: f32 = latest_data.values.get("cpu_die_temp").unwrap_or(&"0".to_string()).parse().unwrap();
    let cpu_package_temp: f32 = latest_data.values.get("cpu_package_temp").unwrap_or(&"0".to_string()).parse().unwrap();
    let cpu_power: f32 = latest_data.values.get("cpu_power").unwrap_or(&"0".to_string()).parse().unwrap();

    d.draw_text_ex(get_font(fonts, "calibri_50_bold"), "CPU", Vector2::new(xf + 10.0, yf + 10.0), 50.0, 0.0, Color::WHITE);
    d.draw_text_ex(get_font(fonts, "calibri_20"), &*format!("{:.2} W", cpu_power), Vector2::new(xf + 110.0, yf + 21.0), 20.0, 0.0, Color::WHITE);

    draw_temperature_gauge(&mut d, x + 200, y + 5, cpu_die_temp as i32, get_font(fonts, "calibri_20"), get_font(fonts, "calibri_13"));
    draw_temperature_gauge(&mut d, x + 275, y + 5, cpu_package_temp as i32, get_font(fonts, "calibri_20"), get_font(fonts, "calibri_13"));
    d.draw_text_ex(get_font(fonts, "calibri_30"), &*format!("{} MHz", max_core_frequency), Vector2::new(xf + 340.0, yf + 18.0), 30.0, 0.0, Color::WHITE);

    d.draw_text_ex(get_font(fonts, "calibri_25_bold"), "Usage", Vector2::new(xf + 10.0, yf + 65.0), 25.0, 0.0, Color::WHITE);
    let gradient_color_1 = Color::new(0, 200, 0, 255);
    let gradient_color_2 = Color::new(0, 40, 0, 255);
    draw_meter_bar(&mut d, x + 80, y + 65, 390, 23, cpu_utilization as i32, 100, (gradient_color_1, gradient_color_2), fonts);

    draw_graph_grid(&mut d, x + 10, y + 100);

    let usage_graph_values = &data.iter()
        .map(|d| d.values.get("cpu_utilization"))
        .filter(|util| util.is_some())
        .map(|v| v.unwrap().parse::<f32>().unwrap())
        .collect();

    draw_graph(&mut d, x + 10, y + 100, usage_graph_values, Color::GREEN);
}

pub fn draw_gpu_panel(mut d: &mut RaylibDrawHandle, x: i32, y: i32, fonts: &HashMap<String, Font>, data: &Vec<SensorData>) {

    let xf = x as f32;
    let yf = y as f32;

    let latest_data = &data[data.len() - 1];

    let gpu_utilization: f32 = latest_data.values.get("gpu_utilization").unwrap_or(&"0".to_string()).parse().unwrap();
    let gpu_die_temp: f32 = latest_data.values.get("gpu_die_temp").unwrap_or(&"0".to_string()).parse().unwrap();
    let gpu_package_temp: f32 = latest_data.values.get("gpu_package_temp").unwrap_or(&"0".to_string()).parse().unwrap();
    let gpu_power: f32 = latest_data.values.get("gpu_power").unwrap_or(&"0".to_string()).parse().unwrap();
    let gpu_voltage: f32 = latest_data.values.get("gpu_voltage").unwrap_or(&"0".to_string()).parse().unwrap();
    let gpu_frequency: f32 = latest_data.values.get("gpu_frequency").unwrap_or(&"0".to_string()).parse().unwrap();
    let gpu_fps: f32 = latest_data.values.get("gpu_fps").unwrap_or(&"0".to_string()).parse().unwrap();

    d.draw_text_ex(get_font(fonts, "calibri_50_bold"), "GPU", Vector2::new(xf + 10.0, yf + 10.0), 50.0, 0.0, Color::WHITE);
    d.draw_text_ex(get_font(fonts, "calibri_20"), &*format!("{:.2} W", gpu_power), Vector2::new(xf + 110.0,  yf + 12.0), 20.0, 0.0, Color::WHITE);
    d.draw_text_ex(get_font(fonts, "calibri_20"), &*format!("{:.2} V", gpu_voltage), Vector2::new(xf + 110.0, yf + 30.0), 20.0, 0.0, Color::WHITE);

    draw_temperature_gauge(&mut d, x + 200, y + 5, gpu_die_temp as i32, get_font(fonts, "calibri_20"), get_font(fonts, "calibri_13"));
    draw_temperature_gauge(&mut d, x + 275, y + 5, gpu_package_temp as i32, get_font(fonts, "calibri_20"), get_font(fonts, "calibri_13"));

    d.draw_text_ex(get_font(fonts, "calibri_30"), &*format!("{} MHz", gpu_frequency), Vector2::new(xf + 340.0, yf + 3.0), 30.0, 0.0, Color::WHITE);
    d.draw_text_ex(get_font(fonts, "calibri_30"), &*format!("{} FPS", gpu_fps), Vector2::new(xf + 340.0, yf + 32.0), 30.0, 0.0, Color::WHITE);
    let gradient_color_1 = Color::new(200, 0, 0, 255);
    let gradient_color_2 = Color::new(40, 0, 0, 255);
    d.draw_text_ex(get_font(fonts, "calibri_25_bold"), "Usage", Vector2::new(xf + 10.0, yf + 65.0), 25.0, 0.0, Color::WHITE);
    draw_meter_bar(&mut d, x + 80, y + 65, 390, 23, gpu_utilization as i32, 100, (gradient_color_1, gradient_color_2), fonts);

    draw_graph_grid(&mut d, x + 10, y + 100);

    let usage_graph_values = &data.iter()
        .map(|d| d.values.get("gpu_utilization"))
        .filter(|util| util.is_some())
        .map(|v| v.unwrap().parse::<f32>().unwrap())
        .collect();

    draw_graph(&mut d, x + 10, y + 100, usage_graph_values, Color::RED);
}

pub fn draw_mem_panel(mut d: &mut RaylibDrawHandle, x: i32, y: i32, fonts: &HashMap<String, Font>, data: &Vec<SensorData>) {

    let xf = x as f32;
    let yf = y as f32;
    let latest_data = &data[data.len() - 1];

    let mem_used : f32 = latest_data.values.get("mem_used").unwrap_or(&"0".to_string()).parse().unwrap();
    let mem_available : f32 = latest_data.values.get("mem_available").unwrap_or(&"0".to_string()).parse().unwrap();
    let mem_used_percent = mem_used / (mem_used + mem_available);

    d.draw_text_ex(get_font(fonts, "calibri_40_bold"), "Mem", Vector2::new(xf + 10.0, yf + 10.0), 40.0, 0.0, Color::WHITE);
    d.draw_text_ex(get_font(fonts, "calibri_20"), &*format!("Available: {:.2} GB", mem_available), Vector2::new(xf + 110.0,  yf + 12.0), 20.0, 0.0, Color::WHITE);
    d.draw_text_ex(get_font(fonts, "calibri_20"), &*format!("Used: {:.2} GB", mem_used), Vector2::new(xf + 141.0,  yf + 30.0), 20.0, 0.0, Color::WHITE);

    let gradient_color_1 = Color::new(50, 50, 255, 255);
    let gradient_color_2 = Color::new(10, 10, 50, 255);
    d.draw_text_ex(get_font(fonts, "calibri_25_bold"), "Usage", Vector2::new(xf + 10.0, yf + 65.0), 25.0, 0.0, Color::WHITE);
    draw_meter_bar(&mut d, x + 80, y + 65, 390, 23, (mem_used_percent * 100.0) as i32, 100, (gradient_color_1, gradient_color_2), fonts);

    draw_graph_grid(&mut d, x + 10, y + 100);
    let utilizations = &data.iter()
        .map(|d| (d.values.get("mem_used"), d.values.get("mem_available")))
        .filter(|mem_vals| mem_vals.0.is_some() && mem_vals.1.is_some())
        .map(|v| (v.0.unwrap().parse::<f32>().unwrap(), v.1.unwrap().parse::<f32>().unwrap()))
        .map(|v| v.0 / (v.0 + v.1) * 100.0)
        .collect();

    draw_graph(&mut d, x + 10, y + 100, utilizations, Color::BLUE);
}

pub fn draw_core_panel(mut d: &mut RaylibDrawHandle, x: i32, y: i32, fonts: &HashMap<String, Font>, data: &Vec<SensorData>) {

    let gradient_color_1 = Color::new(0, 200, 0, 255);
    let gradient_color_2 = Color::new(0, 40, 0, 255);

    let latest_data = &data[data.len() - 1];

    d.draw_text_ex(get_font(fonts, "calibri_50_bold"), "CPU Cores", Vector2::new(x as f32, y as f32 + 10.0), 50.0, 0.0, Color::WHITE);
    for core in 1..9 {
        let core_load: f32 = latest_data.values.get(&*format!("cpu_core_load_{}", core)).unwrap_or(&"0".to_string()).parse().unwrap();
        let core_frequency: f32 = latest_data.values.get(&*format!("cpu_core_frequency_{}", core)).unwrap_or(&"0".to_string()).parse().unwrap();
        let core_y = y + (core - 1) * 28 + 65;
        let core_frequency = format!("{:.0} MHz", core_frequency);
        d.draw_text_ex(get_font(fonts, "calibri_20"), &*format!("#{}", core), Vector2::new(x as f32, core_y as f32), 20.0, 0.0, Color::WHITE);
        draw_meter_bar_with_label(&mut d, x + 30, core_y, 195, 23, core_load as i32, 100, (gradient_color_1, gradient_color_2), fonts, core_frequency, 60.0, Color::WHITE);
    }
    for core in 9..17 {
        let core_load: f32 = latest_data.values.get(&*format!("cpu_core_load_{}", core)).unwrap_or(&"0".to_string()).parse().unwrap();
        let core_frequency: f32 = latest_data.values.get(&*format!("cpu_core_frequency_{}", core)).unwrap_or(&"0".to_string()).parse().unwrap();
        let core_y = y + (core - 9) * 28 + 65;
        let core_frequency = format!("{:.0} MHz", core_frequency);
        d.draw_text_ex(get_font(fonts, "calibri_20"), &*format!("#{}", core), Vector2::new(x as f32 + 240.0, core_y as f32), 20.0, 0.0, Color::WHITE);
        draw_meter_bar_with_label(&mut d, x + 275, core_y, 195, 23, core_load as i32, 100, (gradient_color_1, gradient_color_2), fonts, core_frequency, 60.0, Color::WHITE);
    }
}

pub fn draw_hdd_panel(mut d: &mut RaylibDrawHandle, x: i32, y: i32, fonts: &HashMap<String, Font>, data: &Vec<SensorData>)
{
    let latest_data = &data[data.len() - 1];

    d.draw_text_ex(get_font(fonts, "calibri_50_bold"), "HDD", Vector2::new(x as f32, y as f32), 50.0, 0.0, Color::WHITE);

    for i in 1..=6 {
        let drive_name = latest_data.values.get(&format!("hdd_drive_name_{}", i));
        //let drive_label = latest_data.values.get(&format!("hdd_drive_label_{}", i));
        let drive_total:i64 = latest_data.values.get(&format!("hdd_drive_total_bytes_{}", i)).unwrap_or(&"0".to_string()).parse().unwrap();
        let drive_free:i64 = latest_data.values.get(&format!("hdd_drive_free_bytes_{}", i)).unwrap_or(&"0".to_string()).parse().unwrap();

        if drive_name.is_some() {
            let label = format!("Free: {} GB / {} GB", drive_free / 1000000000, drive_total / 1000000000);
            d.draw_text_ex(get_font(fonts, "calibri_20"), drive_name.unwrap_or(&"?".to_string()), Vector2::new((x + 3) as f32, (y + (i - 1) * 30 + 2) as f32 + 50 as f32), 20.0, 0.0, Color::WHITE);
            draw_meter_bar_with_label(&mut d, x + 25, y + (i - 1) * 30 + 50, 446, 23, (((drive_total - drive_free) as f64 / drive_total as f64) * 100.0 as f64) as i32, 100, (Color::VIOLET, Color::BLACK), fonts, label, 130.0, Color::WHITE);
        }
    }
}

pub fn draw_time_panel(d: &mut RaylibDrawHandle, x: i32, y: i32, fonts: &HashMap<String, Font>) {
    let date = Local::now().format("%H:%M:%S").to_string();
    d.draw_text_ex(get_font(fonts, "calibri_30"), &date, Vector2::new(x as f32, y as f32), 30.0, 0.0, Color::WHITE);
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

fn draw_graph(d: &mut &mut RaylibDrawHandle, x: i32, y: i32, historical: &Vec<f32>, color: Color) {

    if historical.len() == 0 {
        return;
    }

    let mut last = historical.get(historical.len() - 1).unwrap().clone();
    let mut scew = 0;
    let entries = if historical.len() > 231 { 231 } else { historical.len() };
    for i in 2..entries {
        let value = historical[historical.len() - i];
        d.draw_line(x + 460 - 1 - scew * 2 as i32, y + 80 - (last * 0.78) as i32, x + 460 - 1 - (scew + 1) * 2 as i32, y + 80 - (value * 0.78) as i32, color);
        last = value;
        scew = scew + 1;
    }
}

pub fn draw_meter_bar(d: &mut &mut RaylibDrawHandle, x: i32, y: i32, width: i32, height: i32, value: i32, max_value: i32, color: (Color, Color), fonts: &HashMap<String, Font>) {
    let label = format!("{} %", value.to_string());
    draw_meter_bar_with_label(d, x, y, width, height, value, max_value, color, fonts, label, width as f32 / 2.0 - 15.0, Color::WHITE);
}

pub fn draw_meter_bar_with_label(d: &mut &mut RaylibDrawHandle, x: i32, y: i32, width: i32, height: i32, value: i32, max_value: i32, color: (Color, Color), fonts: &HashMap<String, Font>, label: String, label_pos: f32, label_color: Color) {
    d.draw_rectangle(x, y, width, height, Color::DARKGRAY);
    d.draw_rectangle(x + 1, y + 1, width - 2, height - 2, Color::BLACK);

    let bar_width = width * value / max_value;
    d.draw_rectangle_gradient_v(x + 1, y + 1, bar_width, height - 2, color.0, color.1);

    d.draw_text_ex(get_font(fonts, "calibri_20"), &label, Vector2::new(label_pos + x as f32, y as f32 + 3.0), 20.0, 0.0, label_color);
}

pub fn draw_temperature_gauge(d: &mut RaylibDrawHandle, x: i32, y: i32, value: i32, font: &Font, font2: &Font) {
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

    let degree_color = Color::new(220, 220, 220, 255);
    d.draw_circle_lines(x + 22, y + 37, 2.05, degree_color);
    d.draw_text_ex(font2, "C", Vector2::new(x as f32 + 25.0, y as f32 + 34.0), 13.0, 0.0, degree_color);
}
