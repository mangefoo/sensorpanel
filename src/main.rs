use raylib::prelude::*;
use std::iter::Map;
use std::collections::HashMap;
use crate::fonts::load_fonts;
use crate::windows_panel::draw_windows_panel;
use crate::textures::load_textures;

mod fonts;
mod textures;
mod widgets;
mod windows_panel;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1024, 600)
        .title("SensorPanel")
        .build();

    rl.set_target_fps(60);

    let fonts = load_fonts(&mut rl, &thread);
    let textures = load_textures(&mut rl, &thread);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        draw_windows_panel(&fonts, &textures, &mut d);
    }
}

