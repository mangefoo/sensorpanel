use std::collections::HashMap;
use raylib::{RaylibHandle, RaylibThread};
use raylib::core::text::{Font, FontLoadEx};

pub fn get_font<'a>(fonts: &'a HashMap<String, Font>, name: &str) -> &'a Font {
    fonts.get(name).expect("Missing font")
}

pub fn load_fonts(rl: &mut RaylibHandle, thread: &RaylibThread, resources: &String) -> HashMap<String, Font> {
    let mut fonts = HashMap::<String, Font>::new();

    for font in vec!["calibri_13", "calibri_15", "calibri_20", "calibri_30", "calibri_25_bold", "calibri_40_bold", "calibri_50_bold"] {
        let font_str = font.to_string();
        fonts.insert(font_str.clone(), load_font(rl, thread, resources, &font_str));
    }

    return fonts;
}

fn load_font(rl: &mut RaylibHandle, thread: &RaylibThread, resources: &String, font: &str) -> Font {
    let font_parts: Vec<&str> = font.split("_").collect();
    let font_name = font_parts[0];
    let font_size: i32 = font_parts[1].parse().unwrap();
    let font_extension = match *font_parts.get(2).unwrap_or(&"") {
        "bold" => "b",
        _ => ""
    };
    let font_path = format!("{}/fonts/{}{}.ttf", resources, font_name, font_extension);

    return rl.load_font_ex(&thread, &font_path, font_size, FontLoadEx::Default(0))
        .expect("Failed to get font");
}

