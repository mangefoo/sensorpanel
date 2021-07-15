use crate::fonts::load_fonts;
use crate::windows_panel::draw_windows_panel;
use crate::textures::load_textures;
use crate::data::{SensorData};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::json;
use reqwest::{blocking, Url};
use std::{thread};
use tungstenite::{connect};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{mpsc, Arc, Mutex};
use crate::config::{read_config, Config};
use clap::{App, Arg};

mod config;
mod fonts;
mod textures;
mod widgets;
mod windows_panel;
mod data;

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
        "topics": ["sensors"],
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

    let data = Arc::new(Mutex::new(Vec::new()));

    ws_receiver_setup(&config, &data);

    while !rl.window_should_close() {

        let mut d = rl.begin_drawing(&thread);
        draw_windows_panel(&fonts, &textures, &mut d, &(*data.lock().unwrap()));
    }
}

fn ws_receiver_setup(config: &Config, data: &Arc<Mutex<Vec<SensorData>>>) {
    let thread_data = data.clone();
    thread::spawn(move || {
        let historical_reports_count = 500;

        let value_receiver = ws_client_setup(&config);

        loop {
            let data = Arc::clone(&thread_data);
            match value_receiver.recv() {
                Ok(report) => {
                    println!("Got data: {:?}", report);
                    let mut locked_data = data.lock().unwrap();
                    locked_data.push(SensorData { reporter: report.reporter.clone(), values: report.sensors.clone() });
                    if locked_data.len() > historical_reports_count {
                        locked_data.remove(0);
                    }
                }
                Err(ee) => {
                    println!("Got error {}", ee);
                }
            }
        }
    });
}

