use crate::fonts::load_fonts;
use crate::windows_panel::draw_windows_panel;
use crate::textures::load_textures;
use crate::data::{SensorData, State, ScreenState, Present, PresenceData};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::json;
use reqwest::{blocking, Url};
use std::{thread};
use tungstenite::{connect};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{mpsc, Arc, Mutex, MutexGuard};
use crate::config::{read_config, Config};
use clap::{App, Arg};
use crate::screenctl::get_screen_control;
use std::time::{Duration, SystemTime};

mod config;
mod fonts;
mod textures;
mod widgets;
mod windows_panel;
mod data;
mod screenctl;

#[derive(Deserialize, Debug)]
struct RegisterResponse {
    id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SensorReport {
    reporter: String,
    topic: String,
    sensors: HashMap<String, String>,
}

fn ws_client_setup(config: &Config) -> Receiver<SensorReport> {
    let (tx, rx): (Sender<SensorReport>, Receiver<SensorReport>) = mpsc::channel();
    let thread_tx = tx.clone();
    let relay_host= config.relay_host.clone();
    let thread_fn = move || {
        let id = match ws_register_client(&relay_host) {
            Err(error) => panic!("Failed to get WS URL: {}", error),
            Ok(url) => url
        };

        println!("Got WS ID: {}", id);

        ws_read_loop(format!("ws://{}/ws/{}", relay_host, id), thread_tx);
    };

    thread::spawn(thread_fn);

    return rx;
}

fn ws_read_loop(url: String, value_sender: Sender<SensorReport>) {
    let (mut socket, response) =
        connect(Url::parse(&url).unwrap()).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    loop {
        let msg = socket.read_message().expect("Error reading message");
        let report: SensorReport = serde_json::from_str(msg.to_text().unwrap()).unwrap();
        let result = value_sender.send(report);
        match result {
            Err(error) => { println!("Failed to send request: {}", error)}
            _ => {}
        }
    }
}

fn ws_register_client(relay_host: &String) -> Result<String, reqwest::Error> {
    let register_body = json!({
        "topics": ["sensors", "actions"],
    });

    let request_url = format!("http://{}/register", relay_host);

    let response = blocking::Client::new()
        .post(request_url)
        .json(&register_body)
        .send();

    let response = match response {
        Err(error) => panic!("Request failed: {}", error),
        Ok(response) => { println!("Request OK"); response }
    };

    let register_response: RegisterResponse = match response.json() {
        Err(error) => panic!("Parse json failed: {:?}", error),
        Ok(json) => json
    };

    Ok(register_response.id)
}

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

    ws_receiver_setup(&config, &state);

    while !rl.window_should_close() {

        if state.lock().unwrap().screen_on {
            let mut d = rl.begin_drawing(&thread);
            draw_windows_panel(&fonts, &textures, &mut d, &(state.lock().unwrap().sensor_data));
        } else {
            thread::sleep(Duration::from_secs(1));
        }
    }
}

fn ws_receiver_setup(config: &Config, state: &Arc<Mutex<State>>) {
    let thread_state = state.clone();

    let value_receiver = ws_client_setup(&config);

    thread::spawn(move || {
        loop {
            let state = Arc::clone(&thread_state);
            match value_receiver.recv() {
                Ok(event) => {
                    println!("Got event: {:?}", event);
                    let mut locked_state = state.lock().unwrap();
                    handle_event(event, &mut locked_state)
                }
                Err(ee) => {
                    println!("Got error {}", ee);
                }
            }
        }
    });
}

fn handle_event(sensor_report: SensorReport, state: &mut MutexGuard<State>) {
    match sensor_report.topic.as_str() {
        "actions" => {
            handle_action(sensor_report, state);
        }
        "sensors" => {
            handle_sensor(sensor_report, state)
        }
        _ => {}
    }
}

fn handle_sensor(event: SensorReport, state: &mut MutexGuard<State>) {

    let historical_reports_count = 500;

    state.sensor_data.push(SensorData { reporter: event.reporter.clone(), values: event.sensors.clone() });
    if state.sensor_data.len() > historical_reports_count {
        state.sensor_data.remove(0);
    }

    if event.sensors.contains_key("hue_presence") {
        handle_presence(event.sensors.get("hue_presence").unwrap(), state);
    }
}

const PRESENCE_DURATION_THRESHOLD: u64 = 10;

fn handle_presence(presence: &String, state: &mut MutexGuard<State>) {
    let present = if presence == "true" { true } else { false };
    let current_time = SystemTime::now();
    let duration_since_switch_to_false = current_time.duration_since(state.presence.last_switch_to_false).unwrap();

    if present && state.presence.present == Present::NO {
        println!("Presence set to YES");
        get_screen_control().turn_on();
        state.screen_on = true;
        state.presence.present = Present::YES;
    } else if !present && state.presence.present == Present::PENDING && duration_since_switch_to_false.as_secs() > PRESENCE_DURATION_THRESHOLD {
        println!("Presence set to NO");
        get_screen_control().turn_off();
        state.screen_on = false;
        state.presence.present = Present::NO;
    } else if !present && state.presence.present == Present::YES {
        println!("Presence set to PENDING");
        state.presence.present = Present::PENDING;
        state.presence.last_switch_to_false = SystemTime::now();
    } else if present {
        state.presence.present = Present::YES;
    }
}

fn handle_action(sensor_report: SensorReport, state: &mut MutexGuard<State>) {
    if sensor_report.sensors.contains_key("toggle_screen") {
        let screen_was_on = state.screen_on;

        state.screen_state = match state.screen_state {
            ScreenState::ON => ScreenState::OFF,
            ScreenState::OFF => ScreenState::ON,
            ScreenState::AUTO => if state.screen_on { ScreenState::OFF } else { ScreenState::ON }
        };

        match state.screen_state {
            ScreenState::ON => state.screen_on = true,
            ScreenState::OFF => state.screen_on = false,
            _ => {}
        }

        println!("Current screen state: {:?}, screen on: {}", state.screen_state, state.screen_on);

        if !state.screen_on && screen_was_on {
            get_screen_control().turn_off();
        } else if state.screen_on && !screen_was_on {
            get_screen_control().turn_on();
        }
    }
}
