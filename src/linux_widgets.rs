use raylib::prelude::*;
use std::collections::HashMap;
use crate::fonts::get_font;
use crate::data::SensorData;

pub fn draw_cpu_panel(mut d: &mut RaylibDrawHandle, x: i32, y: i32, fonts: &HashMap<String, Font>, data: &Vec<&SensorData>) {

    let xf = x as f32;
    let yf = y as f32;
    let latest_data = data.last().unwrap();

    let max_core_frequency = (1..=16).into_iter()
        .map(|core_number| format!("cpu_core_frequency_{}", core_number))
        .map(|core_key| latest_data.values.get(&core_key))
        .filter(|core_value| core_value.is_some())
        .map(|core_value| core_value.unwrap().parse::<f32>().unwrap() as f32)
        .map(|float_val| format!("{:.0}", float_val).parse::<i32>().unwrap() as i32)
        .max()
        .unwrap_or(0);

    let cpu_utilization: f32 = latest_data.values.get("cpu_utilization").unwrap_or(&"0".to_string()).parse().unwrap();
    let cpu_die_temp: f32 = latest_data.values.get("cpu_temp").unwrap_or(&"0".to_string()).parse().unwrap();
    let cpu_package_temp: f32 = latest_data.values.get("cpu_package_temp").unwrap_or(&"0".to_string()).parse().unwrap();
    let cpu_power: f32 = latest_data.values.get("cpu_power").unwrap_or(&"0".to_string()).parse().unwrap();

    d.draw_text_ex(get_font(fonts, "calibri_40_bold"), "CPU", Vector2::new(xf + 10.0, yf + 10.0), 40.0, 0.0, Color::WHITE);
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

pub fn draw_gpu_panel(mut d: &mut RaylibDrawHandle, x: i32, y: i32, fonts: &HashMap<String, Font>, data: &Vec<&SensorData>, sub_title: Option<&str>, should_draw_graph: bool) {

    let xf = x as f32;
    let yf = y as f32;

    let latest_data = data.last().unwrap();

    let gpu_utilization: f32 = latest_data.values.get("gpu_utilization").unwrap_or(&"0".to_string()).parse().unwrap();
    let gpu_die_temp: f32 = latest_data.values.get("gpu_edge_temp")
        .or(latest_data.values.get("gpu_die_temp"))
        .unwrap_or(&"0".to_string())
        .parse().unwrap();

    let gpu_package_temp: f32 = latest_data.values.get("gpu_junction_temp")
        .or(latest_data.values.get("gpu_package_temp"))
        .unwrap_or(&"0".to_string())
        .parse().unwrap();

    let gpu_power: f32 = latest_data.values.get("gpu_power").unwrap_or(&"0".to_string()).parse().unwrap();
    let gpu_voltage: f32 = latest_data.values.get("gpu_voltage").unwrap_or(&"0".to_string()).parse().unwrap();
    let gpu_frequency: f32 = latest_data.values.get("gpu_frequency").unwrap_or(&"0".to_string()).parse().unwrap();
    let gpu_fps: f32 = latest_data.values.get("gpu_fps").unwrap_or(&"0".to_string()).parse().unwrap();

    if sub_title.is_some() {
        d.draw_text_ex(get_font(fonts, "calibri_25_bold"), "GPU", Vector2::new(xf + 10.0, yf + 10.0), 25.0, 0.0, Color::WHITE);
        d.draw_text_ex(get_font(fonts, "calibri_20"), sub_title.unwrap(), Vector2::new(xf + 10.0, yf + 30.0), 20.0, 0.0, Color::WHITE);
    } else {
        d.draw_text_ex(get_font(fonts, "calibri_50_bold"), "GPU", Vector2::new(xf + 10.0, yf + 10.0), 50.0, 0.0, Color::WHITE);
    }
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

    if should_draw_graph {
        draw_graph_grid(&mut d, x + 10, y + 100);

        let usage_graph_values = &data.iter()
            .map(|d| d.values.get("gpu_utilization"))
            .filter(|util| util.is_some())
            .map(|v| v.unwrap().parse::<f32>().unwrap())
            .collect();

        draw_graph(&mut d, x + 10, y + 100, usage_graph_values, Color::RED);
    }
}

pub fn draw_mem_panel(mut d: &mut RaylibDrawHandle, x: i32, y: i32, fonts: &HashMap<String, Font>, data: &Vec<&SensorData>) {

    let xf = x as f32;
    let yf = y as f32;
    let latest_data = data.last().unwrap();

    let mem_available: f32 = latest_data.values.get("mem_available").unwrap_or(&"0".to_string()).parse().unwrap();
    let mem_total : f32 = latest_data.values.get("mem_total").unwrap_or(&"0".to_string()).parse().unwrap();
    let mem_used = mem_total - mem_available;
    let mem_used_percent = mem_used / mem_total;

    d.draw_text_ex(get_font(fonts, "calibri_40_bold"), "Mem", Vector2::new(xf + 10.0, yf + 10.0), 40.0, 0.0, Color::WHITE);
    d.draw_text_ex(get_font(fonts, "calibri_20"), &*format!("Total: {:.2} GB", mem_total), Vector2::new(xf + 110.0,  yf + 12.0), 20.0, 0.0, Color::WHITE);
    d.draw_text_ex(get_font(fonts, "calibri_20"), &*format!("Used: {:.2} GB", mem_used), Vector2::new(xf + 110.0,  yf + 30.0), 20.0, 0.0, Color::WHITE);

    let gradient_color_1 = Color::BLUE;
    let gradient_color_2 = Color::new(10, 10, 50, 255);
    d.draw_text_ex(get_font(fonts, "calibri_25_bold"), "Usage", Vector2::new(xf + 10.0, yf + 55.0), 25.0, 0.0, Color::WHITE);
    draw_meter_bar(&mut d, x + 80, y + 55, 390, 23, (mem_used_percent * 100.0) as i32, 100, (gradient_color_1, gradient_color_2), fonts);
}

pub fn draw_core_panel(mut d: &mut RaylibDrawHandle, x: i32, y: i32, fonts: &HashMap<String, Font>, data: &Vec<&SensorData>) {

    let gradient_color_1 = Color::new(0, 200, 0, 255);
    let gradient_color_2 = Color::new(0, 40, 0, 255);

    let latest_data = data.last().unwrap();

    d.draw_text_ex(get_font(fonts, "calibri_40_bold"), "CPU Cores", Vector2::new(x as f32, y as f32 + 10.0), 40.0, 0.0, Color::WHITE);
    for core in 1..9 {
        let core_load: f32 = latest_data.values.get(&*format!("cpu_core_load_{}", core)).unwrap_or(&"0".to_string()).parse().unwrap();
        let core_frequency: f32 = latest_data.values.get(&*format!("cpu_core_frequency_{}", core)).unwrap_or(&"0".to_string()).parse().unwrap();
        let core_y = y + (core - 1) * 28 + 55;
        let core_frequency = format!("{:.0} MHz", core_frequency);
        d.draw_text_ex(get_font(fonts, "calibri_20"), &*format!("#{}", core), Vector2::new(x as f32, core_y as f32), 20.0, 0.0, Color::WHITE);
        draw_meter_bar_with_label(&mut d, x + 30, core_y, 195, 23, core_load as i32, 100, (gradient_color_1, gradient_color_2), fonts, core_frequency, 60.0, Color::WHITE);
    }
    for core in 9..17 {
        let core_load: f32 = latest_data.values.get(&*format!("cpu_core_load_{}", core)).unwrap_or(&"0".to_string()).parse().unwrap();
        let core_frequency: f32 = latest_data.values.get(&*format!("cpu_core_frequency_{}", core)).unwrap_or(&"0".to_string()).parse().unwrap();
        let core_y = y + (core - 9) * 28 + 55;
        let core_frequency = format!("{:.0} MHz", core_frequency);
        d.draw_text_ex(get_font(fonts, "calibri_20"), &*format!("#{}", core), Vector2::new(x as f32 + 240.0, core_y as f32), 20.0, 0.0, Color::WHITE);
        draw_meter_bar_with_label(&mut d, x + 275, core_y, 195, 23, core_load as i32, 100, (gradient_color_1, gradient_color_2), fonts, core_frequency, 60.0, Color::WHITE);
    }
}

pub fn draw_net_panel(mut d: &mut RaylibDrawHandle, x: i32, y: i32, fonts: &HashMap<String, Font>, host_data: &Vec<&SensorData>, guest_data: &Vec<&SensorData>) {

    let xf = x as f32;
    let yf = y as f32;

    let host_receive_gradient_color_1 = Color::new(200, 150, 0, 255);
    let host_receive_gradient_color_2 = Color::new(40, 25, 0, 255);
    let host_send_gradient_color_1 = Color::new(0, 200, 200, 255);
    let host_send_gradient_color_2 = Color::new(0, 40, 40, 255);

    let guest_receive_gradient_color_1 = Color::PURPLE;
    let guest_receive_gradient_color_2 = Color::new(40, 20, 50, 255);
    let guest_send_gradient_color_1 = Color::BLUE;
    let guest_send_gradient_color_2 = Color::new(0, 25, 50, 255);

    let cyan = Color::new(0, 200, 200, 255);

    let has_guest_data = guest_data.last().is_some();
    let host_recv_legend = if has_guest_data { "Host Receive" } else { "Receive" };
    let host_send_legend = if has_guest_data { "Host Send" } else { "Send" };

    d.draw_text_ex(get_font(fonts, "calibri_40_bold"), "Network", Vector2::new(xf + 10.0, yf + 15.0), 40.0, 0.0, Color::WHITE);
    d.draw_rectangle(x + 210, y + 18, 10, 10, host_receive_gradient_color_1);
    d.draw_text_ex(get_font(fonts, "calibri_20"), host_recv_legend, Vector2::new(xf + 230.0, yf + 14.0), 20.0, 0.0, Color::WHITE);
    d.draw_rectangle(x + 210, y + 38, 10, 10, host_send_gradient_color_1);
    d.draw_text_ex(get_font(fonts, "calibri_20"), host_send_legend, Vector2::new(xf + 230.0, yf + 34.0), 20.0, 0.0, Color::WHITE);

    if has_guest_data {
        d.draw_rectangle(x + 340, y + 18, 10, 10, Color::PURPLE);
        d.draw_text_ex(get_font(fonts, "calibri_20"), "Guest Receive", Vector2::new(xf + 360.0, yf + 14.0), 20.0, 0.0, Color::WHITE);
        d.draw_rectangle(x + 340, y + 38, 10, 10, Color::BLUE);
        d.draw_text_ex(get_font(fonts, "calibri_20"), "Guest Send", Vector2::new(xf + 360.0, yf + 34.0), 20.0, 0.0, Color::WHITE);
    }

    draw_graph_grid(&mut d, x + 10, y + 100);

    draw_graphs(&mut d, x, y, fonts, &host_data, host_receive_gradient_color_1, host_receive_gradient_color_2, host_send_gradient_color_1, host_send_gradient_color_2, cyan, Color::ORANGE);
    draw_graphs(&mut d, x, y, fonts, &guest_data, guest_receive_gradient_color_1, guest_receive_gradient_color_2, guest_send_gradient_color_1, guest_send_gradient_color_2, Color::BLUE, Color::PURPLE);
}

fn draw_graphs(mut d: &mut &mut RaylibDrawHandle, x: i32, y: i32, fonts: &HashMap<String, Font>, data: &&Vec<&SensorData>, receive_gradient_color_1: Color, receive_gradient_color_2: Color, send_gradient_color_1: Color, send_gradient_color_2: Color, send_color: Color, recv_color: Color) {
    if data.last().is_none() {
        return
    }

    let latest_data = data.last().unwrap();

    for index in 1..=10 {
        let network_name_key = format!("network_name_{}", index);
        if latest_data.values.contains_key(&network_name_key) && latest_data.values.get(&network_name_key).unwrap() == "ethernet" {
            let network_received_key = format!("network_received_bytes_{}", index);
            let network_sent_key = format!("network_sent_bytes_{}", index);
            let received_bytes_per_sec: i64 = latest_data.values.get(&network_received_key).unwrap_or(&"0".to_string()).parse().unwrap();
            let sent_bytes_per_sec: i64 = latest_data.values.get(&network_sent_key).unwrap_or(&"0".to_string()).parse().unwrap();

            let received_label = format!("{:.2} Mbit/s", bytes_to_mbit(received_bytes_per_sec));
            let sent_label = format!("{:.2} Mbit/s", bytes_to_mbit(sent_bytes_per_sec));
            draw_meter_bar_with_label(&mut d, x + 10, y + 65, 225, 23, bytes_to_mbit(received_bytes_per_sec) as i32, 100, (receive_gradient_color_1, receive_gradient_color_2), fonts, received_label, 70.0, Color::WHITE);
            draw_meter_bar_with_label(&mut d, x + 245, y + 65, 225, 23, bytes_to_mbit(sent_bytes_per_sec) as i32, 100, (send_gradient_color_1, send_gradient_color_2), fonts, sent_label, 70.0, Color::WHITE);

            let received_graph_values = &data.iter()
                .map(|d| d.values.get(&network_received_key))
                .filter(|util| util.is_some())
                .map(|v| (bytes_to_mbit(v.unwrap().parse::<i64>().unwrap()) as f32))
                .map(|v| if v > 100.0 { 100.0 } else { v })
                .collect();

            let sent_graph_values = &data.iter()
                .map(|d| d.values.get(&network_sent_key))
                .filter(|util| util.is_some())
                .map(|v| (bytes_to_mbit(v.unwrap().parse::<i64>().unwrap()) as f32))
                .map(|v| if v > 100.0 { 100.0 } else { v })
                .collect();

            draw_graph(&mut d, x + 10, y + 100, sent_graph_values, send_color);
            draw_graph(&mut d, x + 10, y + 100, received_graph_values, recv_color);
        }
    }
}

fn bytes_to_mbit(bytes: i64) -> f32 { (bytes * 8) as f32 / 1000000.0 }

pub fn draw_temp_panel(mut d: &mut RaylibDrawHandle, x: i32, y: i32, fonts: &HashMap<String, Font>, data: &Vec<&SensorData>) {

    let latest_data = data.last().unwrap();

    let pump_temp: f32 = latest_data.values.get("pump_temp").unwrap_or(&"0".to_string()).parse().unwrap();
    let front_intake_temp: f32 = latest_data.values.get("front_intake_temp").unwrap_or(&"0".to_string()).parse().unwrap();
    let exhaust_temp: f32 = latest_data.values.get("exhaust_temp").unwrap_or(&"0".to_string()).parse().unwrap();
    let ambient_temp: f32 = latest_data.values.get("ambient_temp").unwrap_or(&"0".to_string()).parse().unwrap();

    let xf = x as f32;
    let yf = y as f32;

    d.draw_text_ex(get_font(fonts, "calibri_40_bold"), "Temps", Vector2::new(xf + 10.0, yf + 10.0), 40.0, 0.0, Color::WHITE);

    draw_temperature_gauge(&mut d, x + 150, y , pump_temp as i32, get_font(fonts, "calibri_20"), get_font(fonts, "calibri_13"));
    d.draw_text_ex(get_font(fonts, "calibri_20"), "Pump", Vector2::new(xf + 152.0, yf + 55.0), 20.0, 0.0, Color::WHITE);

    draw_temperature_gauge(&mut d, x + 230, y , front_intake_temp as i32, get_font(fonts, "calibri_20"), get_font(fonts, "calibri_13"));
    d.draw_text_ex(get_font(fonts, "calibri_20"), "Intake", Vector2::new(xf + 232.0, yf + 55.0), 20.0, 0.0, Color::WHITE);

    draw_temperature_gauge(&mut d, x + 310, y, exhaust_temp as i32, get_font(fonts, "calibri_20"), get_font(fonts, "calibri_13"));
    d.draw_text_ex(get_font(fonts, "calibri_20"), "Exhaust", Vector2::new(xf + 306.0, yf + 55.0), 20.0, 0.0, Color::WHITE);

    draw_temperature_gauge(&mut d, x + 390, y, ambient_temp as i32, get_font(fonts, "calibri_20"), get_font(fonts, "calibri_13"));
    d.draw_text_ex(get_font(fonts, "calibri_20"), "Ambient", Vector2::new(xf + 382.0, yf + 55.0), 20.0, 0.0, Color::WHITE);
}

pub fn draw_rpm_panel(mut d: &mut RaylibDrawHandle, x: i32, y: i32, fonts: &HashMap<String, Font>, data: &Vec<&SensorData>) {

    let latest_data = data.last().unwrap();

    let top_1 = 806;
    let top_2 = 807;
    let top_3 = 806;
    let front_1 = 989;
    let front_2 = 999;
    let pump = 2097;

    let xf = x as f32;
    let yf = y as f32;

    d.draw_text_ex(get_font(fonts, "calibri_40_bold"), "RPM", Vector2::new(xf + 10.0, yf + 10.0), 40.0, 0.0, Color::WHITE);

    draw_rpm_gauge(&mut d, x + 100, y , top_1, 1500, get_font(fonts, "calibri_15"), get_font(fonts, "calibri_13"));

    draw_rpm_gauge(&mut d, x + 165, y , top_2, 1500, get_font(fonts, "calibri_15"), get_font(fonts, "calibri_13"));
    d.draw_text_ex(get_font(fonts, "calibri_20"), "Top", Vector2::new(xf + 175.0, yf + 55.0), 20.0, 0.0, Color::WHITE);

    draw_rpm_gauge(&mut d, x + 230, y, top_3, 1500, get_font(fonts, "calibri_15"), get_font(fonts, "calibri_13"));

    d.draw_line(x + 100 , y + 54, x + 280, y + 54, Color::WHITE);
    d.draw_line(x + 100 , y + 54, x + 100, y + 49, Color::WHITE);
    d.draw_line(x + 280, y + 54, x + 280, y + 49, Color::WHITE);

    draw_rpm_gauge(&mut d, x + 295, y, front_1, 2000, get_font(fonts, "calibri_15"), get_font(fonts, "calibri_13"));

    draw_rpm_gauge(&mut d, x + 360, y, front_2, 2000, get_font(fonts, "calibri_15"), get_font(fonts, "calibri_13"));
    d.draw_text_ex(get_font(fonts, "calibri_20"), "Front", Vector2::new(xf + 333.0, yf + 55.0), 20.0, 0.0, Color::WHITE);

    d.draw_line(x + 295, y + 54, x + 410, y + 54, Color::WHITE);
    d.draw_line(x + 295, y + 54, x + 295, y + 49, Color::WHITE);
    d.draw_line(x + 410, y + 54, x + 410, y + 49, Color::WHITE);

    draw_rpm_gauge(&mut d, x + 425, y, pump, 5000, get_font(fonts, "calibri_15"), get_font(fonts, "calibri_13"));
    d.draw_text_ex(get_font(fonts, "calibri_20"), "Pump", Vector2::new(xf + 428.0, yf + 55.0), 20.0, 0.0, Color::WHITE);

    d.draw_line(x + 425, y + 54, x + 475, y + 54, Color::WHITE);
    d.draw_line(x + 425, y + 54, x + 425, y + 49, Color::WHITE);
    d.draw_line(x + 475, y + 54, x + 475, y + 49, Color::WHITE);
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
        if last >= 1.0 || value >= 1.0 {
            d.draw_line(x + 460 - 1 - scew * 2 as i32, y + 79 - (last * 0.77) as i32, x + 460 - 1 - (scew + 1) * 2 as i32, y + 79 - (value * 0.77) as i32, color);
        }
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

    let value_with_ceiling = if value > max_value { max_value } else { value };
    let bar_width = width * value_with_ceiling / max_value;
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

    d.draw_circle_sector(Vector2::new(x as f32 + 25.0, y as f32 + 25.0), 20.0, 680.0 - end_angle as f32, 680.0, 1000, color);
    d.draw_circle(x + 25, y + 25, 13.0, Color::BLACK);

    d.draw_text_ex(font, &value.to_string(), Vector2::new(x as f32 + 15.0, y as f32 + 17.0), 20.0, 0.0, Color::WHITE);

    let degree_color = Color::new(220, 220, 220, 255);
    d.draw_circle_lines(x + 22, y + 37, 2.05, degree_color);
    d.draw_text_ex(font2, "C", Vector2::new(x as f32 + 25.0, y as f32 + 34.0), 13.0, 0.0, degree_color);
}

pub fn draw_rpm_gauge(d: &mut RaylibDrawHandle, x: i32, y: i32, value: i32, max: i32, font: &Font, font2: &Font) {
    let background = Color::new(0, 100, 20, 255);
    let background1 = Color::new(100, 100, 20, 255);
    let background2 = Color::new(100, 0, 20, 255);

    d.draw_circle(x + 25, y + 25, 25.0, Color::LIGHTGRAY);
    d.draw_circle(x + 25, y + 25, 23.0, Color::BLACK);

    let end_angle = 280 * value / 100;
    let color = match value {
        v if v > 80 => Color::RED,
        v if v > 70 => Color::ORANGE,
        _ => Color::GREEN
    };

    let angle = 180.0 - (value as f32 / max as f32) * 180.0 + 90.0;
    let value_str = &*format!("{}", value);
    let text_adjust = (4 - value_str.len()) * 4;

    d.draw_circle_sector(Vector2::new(x as f32 + 25.0, y as f32 + 25.0), 20.0, 90.0, 270.0, 100, background);
    d.draw_circle_sector(Vector2::new(x as f32 + 25.0, y as f32 + 25.0), 20.0, 90.0, 140.0, 100, background1);
    d.draw_circle_sector(Vector2::new(x as f32 + 25.0, y as f32 + 25.0), 20.0, 90.0, 110.0, 100, background2);
    d.draw_circle_sector(Vector2::new(x as f32 + 25.0, y as f32 + 25.0), 20.0, angle, angle + 10.0, 10, Color::WHITE);
    d.draw_text_ex(font, &*format!("{}", value), Vector2::new(x as f32 + 11.0 + text_adjust as f32, y as f32 + 27.0), 15.0, 0.0, Color::WHITE);
}