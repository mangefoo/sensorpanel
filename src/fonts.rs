use std::collections::HashMap;
use raylib::{RaylibHandle, RaylibThread};
use raylib::core::text::{Font, FontLoadEx};

pub fn get_font<'a>(fonts: &'a HashMap<String, Font>, name: &str) -> &'a Font {
    fonts.get(name).expect("Missing font")
}

pub fn load_fonts(rl: &mut RaylibHandle, thread: &RaylibThread) -> HashMap<String, Font> {
    let mut fonts = HashMap::<String, Font>::new();

    let calibri_12 = rl.load_font_ex(&thread, "resources/calibri.ttf", 13, FontLoadEx::Default(0))
        .expect("Failed to get font");
    let calibri_20 = rl.load_font_ex(&thread, "resources/calibri.ttf", 20, FontLoadEx::Default(0))
        .expect("Failed to get font");
    let calibri_30 = rl.load_font_ex(&thread, "resources/calibri.ttf", 30, FontLoadEx::Default(0))
        .expect("Failed to get font");
    let calibri_25_bold = rl.load_font_ex(&thread, "resources/calibrib.ttf", 25, FontLoadEx::Default(0))
        .expect("Failed to get font");
    let calibri_40_bold = rl.load_font_ex(&thread, "resources/calibrib.ttf", 40, FontLoadEx::Default(0))
        .expect("Failed to get font");
    let calibri_50_bold = rl.load_font_ex(&thread, "resources/calibrib.ttf", 50, FontLoadEx::Default(0))
        .expect("Failed to get font");

    fonts.insert("calibri_12".to_string(), calibri_12);
    fonts.insert("calibri_20".to_string(), calibri_20);
    fonts.insert("calibri_30".to_string(), calibri_30);
    fonts.insert("calibri_25_bold".to_string(), calibri_25_bold);
    fonts.insert("calibri_40_bold".to_string(), calibri_40_bold);
    fonts.insert("calibri_50_bold".to_string(), calibri_50_bold);

    return fonts;
}