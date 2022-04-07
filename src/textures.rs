use std::collections::HashMap;
use raylib::{RaylibHandle, RaylibThread};
use raylib::core::texture::Texture2D;

pub fn get_texture<'a>(textures: &'a HashMap<String, Texture2D>, name: &str) -> &'a Texture2D {
    textures.get(name).expect("Missing texture")
}

pub fn load_textures(rl: &mut RaylibHandle, thread: &RaylibThread, resources: &String) -> HashMap<String, Texture2D> {
    let mut textures = HashMap::<String, Texture2D>::new();

    let windows_background = rl.load_texture(&thread, &format!("{}/{}", resources, "/images/windows_7_1024.png"))
        .expect("Failed to get background");
    let pending_background= rl.load_texture(&thread, &format!("{}/{}", resources, "/images/PM5644-1024x600.png"))
        .expect("Failed to get background");
    let linux_background= rl.load_texture(&thread, &format!("{}/{}", resources, "/images/ubuntu_1024x600.png"))
        .expect("Failed to get background");
    let ryzen_logo = rl.load_texture(&thread, &format!("{}/{}", resources, "/images/ryzen_logo.png"))
        .expect("Failed to get Ryzen logo");
    let amd_logo = rl.load_texture(&thread, &format!("{}/{}", resources, "/images/amd_logo.png"))
        .expect("Failed to get Ryzen logo");

    textures.insert("windows_background".to_string(), windows_background);
    textures.insert("pending_background".to_string(), pending_background);
    textures.insert("linux_background".to_string(), linux_background);
    textures.insert("ryzen_logo".to_string(), ryzen_logo);
    textures.insert("amd_logo".to_string(), amd_logo);

    return textures;
}