use std::collections::HashMap;
use raylib::{RaylibHandle, RaylibThread};
use raylib::core::text::{Font, FontLoadEx};
use raylib::core::texture::Texture2D;

pub fn get_texture<'a>(textures: &'a HashMap<String, Texture2D>, name: &str) -> &'a Texture2D {
    textures.get(name).expect("Missing texture")
}

pub fn load_textures(rl: &mut RaylibHandle, thread: &RaylibThread) -> HashMap<String, Texture2D> {
    let mut textures = HashMap::<String, Texture2D>::new();

    let windows_background = rl.load_texture(&thread, "resources/windows_7_1024.png")
        .expect("Failed to get background");

    textures.insert("windows_background".to_string(), windows_background);

    return textures;
}