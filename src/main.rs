use crate::fonts::load_fonts;
use crate::textures::load_textures;
use std::{thread, process};
use std::sync::{Arc, Mutex};
use crate::config::{read_config};
use clap::{App, Arg};
use crate::screenctl::get_screen_control;
use std::time::{Duration, Instant};
use crate::state::{StateExt, State};
use raylib::core::drawing::RaylibDraw;
use raylib::color::Color;
use crate::websocket::{WebSocket, WebSocketExt};
use pending_panel::PendingPanel;
use crate::windows_panel::WindowsPanel;
use crate::panel::Panel;
use crate::linux_panel::LinuxPanel;
use crate::log::{Log, LogExt, LogLevel};
use crate::context::Context;
use crate::event::{Event, EventExt};

mod config;
mod fonts;
mod textures;
mod windows_widgets;
mod linux_widgets;
mod windows_panel;
mod pending_panel;
mod linux_panel;
mod data;
mod screenctl;
mod state;
mod websocket;
mod panel;
mod log;
mod context;
mod event;

fn main() {
    #[link(name="libray", kind="dylib")]

    let matches = App::new("Sensor Panel")
        .args(&[Arg::new("configpath")
            .short('c')
            .long("configfile")
            .takes_value(true)])
        .get_matches();

    let config_path = match matches.value_of("configpath") {
        None => "config.toml",
        Some(s) => s
    };

    let config = read_config(config_path);

    let (mut handle, thread) = raylib::init()
        .size(1024, 600)
        .title("SensorPanel")
        .build();

    handle.set_target_fps(60);

    let fonts = load_fonts(&mut handle, &thread, &config.resources);
    let textures = load_textures(&mut handle, &thread, &config.resources);
    let state = Arc::new(Mutex::new(State::init()));

    let mut context = Context {
        config,
        thread,
        handle,
        fonts,
        textures,
        state
    };

    ws_receiver_setup(&context);

    while !context.handle.window_should_close() {
        draw_window(&mut context);
    }
}

fn draw_window(context: &mut Context) {
    if context.state.lock().unwrap().screen_on {
        let mut d = context.handle.begin_drawing(&context.thread);
        let now = Instant::now();

        let has_windows_data = context.state.lock().unwrap().sensor_data.iter()
            .filter(|d| { d.reporter == "windows-sensor-agent" })
            .filter(|d| { now - d.received < Duration::from_secs(10) })
            .count() > 0;

        let has_linux_data = context.state.lock().unwrap().sensor_data.iter()
            .filter(|d| { d.reporter == "linux-sensor-agent" })
            .filter(|d| { now - d.received < Duration::from_secs(10) })
            .count() > 0;

        if has_windows_data {
            WindowsPanel::draw(&context.fonts, &context.textures, &mut d, &(context.state.lock().unwrap().sensor_data));
        } else if has_linux_data {
            LinuxPanel::draw(&context.fonts, &context.textures, &mut d, &(context.state.lock().unwrap().sensor_data));
        } else {
            PendingPanel::draw(&context.fonts, &context.textures, &mut d, &(context.state.lock().unwrap().sensor_data));
        }
    } else {
        if get_screen_control().should_clear_screen() {
            let mut d = context.handle.begin_drawing(&context.thread);
            d.clear_background(Color::BLACK);
        } else {
            thread::sleep(Duration::from_secs(1));
        }
    }
}

fn ws_receiver_setup(context: &Context) {
    WebSocket::receiver_loop(&context, |event, state, config| {
        let new_state = Event::handle(event, state, config);

        if !new_state.screen_on && state.screen_on {
            get_screen_control().turn_off();
        } else if new_state.screen_on && !state.screen_on {
            get_screen_control().turn_on();
        }

        new_state.transfer_to(state);
    },
    |error| {
        Log::log(LogLevel::ERROR, &*format!("Got error {}", error));
        process::exit(1);
    });
}
