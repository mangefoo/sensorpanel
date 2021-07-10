use std::collections::HashMap;
use raylib::{RaylibHandle, RaylibThread};
use raylib::core::text::{Font, FontLoadEx};

pub fn get_font<'a>(fonts: &'a HashMap<String, Font>, name: &str) -> &'a Font {
    fonts.get(name).expect("Missing font")
}

pub fn load_fonts(rl: &mut RaylibHandle, thread: &RaylibThread) -> HashMap<String, Font> {
    let mut fonts = HashMap::<String, Font>::new();

    let calibrib = rl.load_font_ex(&thread, "resources/calibrib.ttf", 50, FontLoadEx::Default(0))
        .expect("Failed to get font");
    let calibrib2 = rl.load_font_ex(&thread, "resources/calibrib.ttf", 25, FontLoadEx::Default(0))
        .expect("Failed to get font");
    let calibri = rl.load_font_ex(&thread, "resources/calibri.ttf", 20, FontLoadEx::Default(0))
        .expect("Failed to get font");
    let calibril = rl.load_font_ex(&thread, "resources/calibri.ttf", 30, FontLoadEx::Default(0))
        .expect("Failed to get font");
    let calibri_bold_40 = rl.load_font_ex(&thread, "resources/calibrib.ttf", 40, FontLoadEx::Default(0))
        .expect("Failed to get font");
    let calibri_12 = rl.load_font_ex(&thread, "resources/calibri.ttf", 12, FontLoadEx::Default(0))
        .expect("Failed to get font");

    fonts.insert("calibrib".to_string(), calibrib);
    fonts.insert("calibrib2".to_string(), calibrib2);
    fonts.insert("calibri".to_string(), calibri);
    fonts.insert("calibril".to_string(), calibril);
    fonts.insert("calibri_bold_40".to_string(), calibri_bold_40);
    fonts.insert("calibri_12".to_string(), calibri_12);

    return fonts;
}