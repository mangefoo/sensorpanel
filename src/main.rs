use crate::fonts::load_fonts;
use crate::windows_panel::draw_windows_panel;
use crate::textures::load_textures;
use crate::data::{SensorData, State, ScreenState, Present, PresenceData};
use std::collections::HashMap;
use serde::{Serialize, Deserialize, Deserializer};
use reqwest::{blocking, Url};
use std::{thread, process};
use tungstenite::{connect};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{mpsc, Arc, Mutex};
use crate::config::{read_config, Config};
use clap::{App, Arg};
use crate::screenctl::get_screen_control;
use std::time::{Duration, SystemTime, Instant};
use crate::pending_panel::draw_pending_panel;
use crate::linux_panel::draw_linux_panel;
use crate::state::{update_presence, update_state_presence, toggle_screen_state, StateExt};
use raylib::core::drawing::RaylibDraw;
use raylib::color::Color;
use crate::websocket::{SensorReport, ws_client_setup};

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

    let state = Arc::new(Mutex::new(State {
        sensor_data: Vec::new(),
        screen_on: true,
        screen_state: ScreenState::AUTO,
        presence: PresenceData {
            present: Present::YES,
            last_switch_to_false: SystemTime::now()
        }
    }));

    ws_receiver_setup(config.clone(), &state);

    while !rl.window_should_close() {

        if state.lock().unwrap().screen_on {
            let mut d = rl.begin_drawing(&thread);
            let now = Instant::now();

            let has_windows_data = state.lock().unwrap().sensor_data.iter()
                .filter(|d| { d.reporter == "windows-sensor-agent"} )
                .filter(|d| { now - d.received < Duration::from_secs(10) })
                .count() > 0;

            let has_linux_data = state.lock().unwrap().sensor_data.iter()
                .filter(|d| { d.reporter == "linux-sensor-agent"} )
                .filter(|d| { now - d.received < Duration::from_secs(10) })
                .count() > 0;

            if has_windows_data {
                draw_windows_panel(&fonts, &textures, &mut d, &(state.lock().unwrap().sensor_data));
            } else if has_linux_data {
                draw_linux_panel(&fonts, &textures, &mut d, &(state.lock().unwrap().sensor_data));
            } else {
                draw_pending_panel(&fonts, &textures, &mut d, &(state.lock().unwrap().sensor_data));
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
}

fn ws_receiver_setup(config: Config, state: &Arc<Mutex<State>>) {
    let thread_state = state.clone();

    let value_receiver = ws_client_setup(&config);

    thread::spawn(move || {
        loop {
            let state = Arc::clone(&thread_state);
            match value_receiver.recv() {
                Ok(event) => {
                    let mut locked_state = state.lock().unwrap();
                    let state = &*locked_state;
                    let new_state = handle_event(event, state, &config);

                    if !new_state.screen_on && state.screen_on {
                        get_screen_control().turn_off();
                    } else if new_state.screen_on && !state.screen_on {
                        get_screen_control().turn_on();
                    }

                    new_state.transfer_to(& mut *locked_state);
                }
                Err(ee) => {
                    println!("Got error {}", ee);
                    process::exit(1);
                }
            }
        }
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

    let new_presence = update_presence(present, &state.presence, config.presence_threshold_secs);
    return update_state_presence(&state, new_presence);
}

fn handle_action(sensor_report: SensorReport, state: &State) -> State {
    return if sensor_report.sensors.contains_key("toggle_screen") {
        toggle_screen_state(state)
    } else {
        state.clone()
    }
}
