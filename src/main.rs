use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(480, 800)
        .title("Hello, World")
        .build();

    rl.set_target_fps(60);

    let calibrib = rl.load_font_ex(&thread, "c:\\Windows\\Fonts\\calibrib.ttf", 50, FontLoadEx::Default(0))
        .expect("Failed to get font");
    let calibrib2 = rl.load_font_ex(&thread, "c:\\Windows\\Fonts\\calibrib.ttf", 25, FontLoadEx::Default(0))
        .expect("Failed to get font");
    let calibri = rl.load_font_ex(&thread, "c:\\Windows\\Fonts\\calibri.ttf", 20, FontLoadEx::Default(0))
        .expect("Failed to get font");
    let calibril = rl.load_font_ex(&thread, "c:\\Windows\\Fonts\\calibri.ttf", 30, FontLoadEx::Default(0))
        .expect("Failed to get font");
    let background = rl.load_texture(&thread, "c:\\temp\\brushed_steel_blue.png")
        .expect("Failed to get background");
//    rl.gui_set_font(font);
//    println!("Font {}", "fuckas");
//     match font {
//         Ok(v) => { println!("Setting font"); rl.gui_set_font(v) },
//         Err(e) => println!("Error loading font {}", e)
//     }

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.draw_texture(&background, 0, 0, Color::WHITE);
        d.clear_background(Color::WHITE);

        d.draw_text_ex(&calibrib, "CPU", Vector2::new(10.0, 10.0), 50.0, 0.0, Color::WHITE);
        d.draw_text_ex(&calibri, "94.27 W", Vector2::new(110.0, 12.0), 20.0, 0.0, Color::WHITE);
        d.draw_text_ex(&calibri, "1.232 V", Vector2::new(110.0, 30.0), 20.0, 0.0, Color::WHITE);

        d.draw_circle(225, 30, 25.0, Color::LIGHTGRAY);
        d.draw_circle(225, 30, 23.0, Color::BLACK);
        d.draw_circle_sector(Vector2::new(225.0, 30.0), 20.0, 400, 680, 1000, Color::GREEN);
        d.draw_circle(225, 30, 13.0, Color::BLACK);
        d.draw_text_ex(&calibri, "45", Vector2::new(215.0, 22.0), 20.0, 0.0, Color::WHITE);

        d.draw_circle(300, 30, 25.0, Color::LIGHTGRAY);
        d.draw_circle(300, 30, 23.0, Color::BLACK);
        d.draw_circle_sector(Vector2::new(300.0, 30.0), 20.0, 400, 680, 1000, Color::GREEN);
        d.draw_circle(300, 30, 13.0, Color::BLACK);
        d.draw_text_ex(&calibri, "75", Vector2::new(290.0, 22.0), 20.0, 0.0, Color::WHITE);

        d.draw_text_ex(&calibril, "3600 MHz", Vector2::new(340.0, 18.0), 30.0, 0.0, Color::WHITE);

        d.draw_text_ex(&calibrib2, "Usage", Vector2::new(10.0, 75.0), 25.0, 0.0, Color::WHITE);
        d.draw_rectangle(80, 75, 390, 20, Color::DARKGRAY);
        d.draw_rectangle(81, 76, 388, 18, Color::BLACK);
        d.draw_rectangle_gradient_v(81, 76, 300, 18, Color::GREEN, Color::DARKGREEN);

        d.draw_rectangle(10, 110, 460, 80, Color::DARKGRAY);
        d.draw_rectangle(11, 111, 458, 78, Color::BLACK);

        draw_gauge(d, 240, 400, String::from("99"), &calibri);
    }
}

fn draw_gauge(mut d: RaylibDrawHandle, x: i32, y: i32, text: String, font: &Font) {
    d.draw_circle(x + 25, y + 25, 25.0, Color::LIGHTGRAY);
    d.draw_circle(x + 25, y + 25, 23.0, Color::BLACK);
    d.draw_circle_sector(Vector2::new(x as f32 + 25.0, y as f32 + 25.0), 20.0, 400, 680, 1000, Color::GREEN);
    d.draw_circle(x + 25, y + 25, 13.0, Color::BLACK);

    d.draw_text_ex(font, &text, Vector2::new(x as f32 + 15.0, y as f32 + 17.0), 20.0, 0.0, Color::WHITE);
}