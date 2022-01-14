use std::collections::HashMap;
use chrono::Local;
use raylib::color::Color;
use raylib::drawing::RaylibDraw;
use raylib::math::Vector2;
use raylib::prelude::{Font, RaylibDrawHandle};
use crate::data::SensorData;
use crate::fonts::get_font;

pub fn draw_time_panel(d: &mut RaylibDrawHandle, x: i32, y: i32, fonts: &HashMap<String, Font>, hue: &Vec<&SensorData>, crypto: &Vec<&SensorData>) {

    let latest_hue = hue.last();
    let latest_crypto = crypto.last();

    if latest_hue.is_some() {
        let office_temp: f32 = latest_hue.unwrap().values.get("hue_temperature").unwrap_or(&"0".to_string()).parse().unwrap();

        let temp = format!("{:.1}   C", office_temp);
        d.draw_text_ex(get_font(fonts, "calibri_30"), &temp, Vector2::new((x + 265) as f32, y as f32), 30.0, 0.0, Color::WHITE);
        d.draw_circle(x + 329, y + 7, 4.0, Color::WHITE);
        d.draw_circle(x + 329, y + 7, 2.0, Color::new(1,0,240, 255));
    }

    if latest_crypto.is_some() {
        let bitcoin: f32 = latest_crypto.unwrap().values.get("bitcoin_price").unwrap_or(&"0".to_string()).parse().unwrap();
        let ethereum: f32 = latest_crypto.unwrap().values.get("ethereum_price").unwrap_or(&"0".to_string()).parse().unwrap();
        let mut bitcoin_color = Color::WHITE;
        let mut ethereum_color = Color::WHITE;

        let previous_crypto = crypto.get(crypto.len().wrapping_sub(2));
        if previous_crypto.is_some() {
            let previous_bitcoin: f32 = previous_crypto.unwrap().values.get("bitcoin_price").unwrap_or(&"0".to_string()).parse().unwrap();
            bitcoin_color = get_diff_color(bitcoin - previous_bitcoin);

            let previous_ethereum: f32 = previous_crypto.unwrap().values.get("ethereum_price").unwrap_or(&"0".to_string()).parse().unwrap();
            ethereum_color = get_diff_color(ethereum - previous_ethereum);
        }

        d.draw_text_ex(get_font(fonts, "calibri_20"), "BTC", Vector2::new((x) as f32, (y + 5) as f32), 20.0, 0.0, Color::GRAY);
        d.draw_text_ex(get_font(fonts, "calibri_20"), &format!("${:.0}", bitcoin), Vector2::new((x + 40) as f32, (y + 5) as f32), 20.0, 0.0, bitcoin_color);

        d.draw_text_ex(get_font(fonts, "calibri_20"), "ETH", Vector2::new((x + 120) as f32, (y + 5) as f32), 20.0, 0.0, Color::GRAY);
        d.draw_text_ex(get_font(fonts, "calibri_20"), &format!("${:.0}", ethereum), Vector2::new((x + 160) as f32, (y + 5) as f32), 20.0, 0.0, ethereum_color);
    }

    let date = Local::now().format("%H:%M:%S").to_string();
    d.draw_text_ex(get_font(fonts, "calibri_30"), &date, Vector2::new((x + 375) as f32, y as f32), 30.0, 0.0, Color::WHITE);
}

fn get_diff_color(diff: f32) -> Color {
    return match diff {
        d if d > 0.0 => Color::GREEN,
        d if d < 0.0 => Color::RED,
        _ => Color::WHITE
    };
}
