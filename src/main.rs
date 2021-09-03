use crate::fonts::load_fonts;
use crate::textures::load_textures;
use crate::data::{SensorData};
use std::{thread, process};
use std::sync::{Arc, Mutex};
use crate::config::{read_config, Config};
use clap::{App, Arg};
use crate::screenctl::get_screen_control;
use std::time::{Duration, Instant};
use crate::state::{StateExt, init_state, State};
use raylib::core::drawing::RaylibDraw;
use raylib::color::Color;
use crate::websocket::{SensorReport, ws_receiver_loop};
use raylib::core::texture::Texture2D;
use std::collections::HashMap;
use raylib::{RaylibHandle, RaylibThread};
use raylib::core::text::Font;
use pending_panel::PendingPanel;
use crate::windows_panel::WindowsPanel;
use crate::panel::Panel;
use crate::linux_panel::LinuxPanel;

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

    let (mut rl, thread) = raylib::init()
        .size(1024, 600)
        .title("SensorPanel")
        .build();

    rl.set_target_fps(60);

    let fonts = load_fonts(&mut rl, &thread, &config.resources);
    let textures = load_textures(&mut rl, &thread, &config.resources);
    let state = Arc::new(Mutex::new(init_state()));

    ws_receiver_setup(config.clone(), &state);

    while !rl.window_should_close() {
        draw_window(&mut rl, &thread, &fonts, &textures, &state)
    }
}

fn draw_window(rl: &mut RaylibHandle, thread: &RaylibThread, fonts: &HashMap<String, Font>, textures: &HashMap<String, Texture2D>, state: &Arc<Mutex<State>>) {
    if state.lock().unwrap().screen_on {
        let mut d = rl.begin_drawing(&thread);
        let now = Instant::now();

        let has_windows_data = state.lock().unwrap().sensor_data.iter()
            .filter(|d| { d.reporter == "windows-sensor-agent" })
            .filter(|d| { now - d.received < Duration::from_secs(10) })
            .count() > 0;

        let has_linux_data = state.lock().unwrap().sensor_data.iter()
            .filter(|d| { d.reporter == "linux-sensor-agent" })
            .filter(|d| { now - d.received < Duration::from_secs(10) })
            .count() > 0;

        if has_windows_data {
            WindowsPanel::draw(&fonts, &textures, &mut d, &(state.lock().unwrap().sensor_data));
        } else if has_linux_data {
            LinuxPanel::draw(&fonts, &textures, &mut d, &(state.lock().unwrap().sensor_data));
        } else {
            PendingPanel::draw(&fonts, &textures, &mut d, &(state.lock().unwrap().sensor_data));
        }
    } else {
        if get_screen_control().should_clear_screen() {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);
        } else {
            thread::sleep(Duration::from_secs(1));
        }
    }
}

fn ws_receiver_setup(config: Config, state: &Arc<Mutex<State>>) {
    ws_receiver_loop(config, state, |event, state, config| {
        let new_state = handle_event(event, state, config);

        if !new_state.screen_on && state.screen_on {
            get_screen_control().turn_off();
        } else if new_state.screen_on && !state.screen_on {
            get_screen_control().turn_on();
        }

        new_state.transfer_to(state);
    },
    |error| {
        println!("Got error {}", error);
        process::exit(1);
    });
}

fn handle_event(sensor_report: SensorReport, state: &State, config: &Config) -> State {
    return match sensor_report.topic.as_str() {
        "actions" => {
            handle_action(sensor_report, state)
        }
        "sensors" => {
            handle_sensor(sensor_report, state, config)
        }
        _ => {
            state.clone()
        }
    }
}

fn handle_sensor(event: SensorReport, state: &State, config: &Config) -> State {
    let historical_reports_count = 500;

    let mut new_state = state.clone();

    if event.sensors.contains_key("hue_presence") {
        new_state = handle_presence(event.sensors.get("hue_presence").unwrap(), state, config);
    }

    new_state.sensor_data.push(SensorData { reporter: event.reporter.clone(), values: event.sensors.clone(), received: event.received.clone() });
    if new_state.sensor_data.len() > historical_reports_count {
        new_state.sensor_data.remove(0);
    }

    return new_state;
}

fn handle_presence(presence: &String, state: &State, config: &Config) -> State {
    let present = if presence == "true" { true } else { false };

    return state.update_presence(present, config.presence_threshold_secs);
}

fn handle_action(sensor_report: SensorReport, state: &State) -> State {
    return if sensor_report.sensors.contains_key("toggle_screen") {
        state.toggle_screen_state()
    } else {
        state.clone()
    }
}
