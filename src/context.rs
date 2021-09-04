use raylib::{RaylibThread, RaylibHandle};
use std::collections::HashMap;
use raylib::core::text::Font;
use raylib::core::texture::Texture2D;
use std::sync::{Arc, Mutex};
use crate::state::State;
use crate::config::Config;

pub(crate) struct Context {
    pub config: Config,
    pub thread: RaylibThread,
    pub handle: RaylibHandle,
    pub fonts: HashMap<String, Font>,
    pub textures: HashMap<String, Texture2D>,
    pub state: Arc<Mutex<State>>
}
