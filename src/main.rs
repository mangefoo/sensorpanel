use crate::fonts::load_fonts;
use crate::windows_panel::draw_windows_panel;
use crate::textures::load_textures;
use crate::data::{SensorData};
use std::collections::HashMap;
use rand::Rng;
use serde::Deserialize;
use serde_json::json;
use reqwest::{Error, blocking, Url};
use tokio::runtime::Runtime;
use std::{thread, env};
use tungstenite::{connect, Message};
use std::sync::mpsc::{Sender, Receiver, TryRecvError};
use std::sync::mpsc;
use std::collections::hash_map::RandomState;
use std::time::Duration;
use raylib::ease::sine_out;

extern crate rand;

mod fonts;
mod textures;
mod widgets;
mod windows_panel;
mod data;

#[derive(Deserialize, Debug)]
struct RegisterResponse {
    url: String,
}

fn ws_client_setup() -> Receiver<HashMap<String, String>> {
    let (tx, rx): (Sender<HashMap<String, String>>, Receiver<HashMap<String, String>>) = mpsc::channel();
    let thread_tx = tx.clone();

    thread::spawn(|| {
        let url = match ws_register_client() {
            Err(error) => panic!("Failed to get WS URL"),
            Ok(url) => url
        };

        println!("Got WS URL: {}", url);

        ws_read_loop(url, thread_tx);
    });

    return rx;
}

fn ws_read_loop(url: String, valueSender: Sender<HashMap<String, String>>) {
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
        println!("Received: {}", msg);
        let values: HashMap<String, String> = serde_json::from_str(msg.to_text().unwrap()).unwrap();
        println!("Values: {:?}", values);
        valueSender.send(values);
    }
}

fn ws_register_client() -> Result<String, reqwest::Error> {
    let register_body = json!({
        "user_id": 4711,
    });

    let request_url = "http://127.0.0.1:8000/register";

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

    Ok(register_response.url)
}

fn generate_sensor_values() {

    let mut rng = rand::thread_rng();

    while true {
        let mut sensor_values: HashMap<String, String> = HashMap::new();
        sensor_values.insert("cpu_utilization".to_string(), rng.gen_range(0..100).to_string());
        sensor_values.insert("cpu_die_temp".to_string(), rng.gen_range(29..100).to_string());
        sensor_values.insert("cpu_package_temp".to_string(), rng.gen_range(29..100).to_string());
        sensor_values.insert("cpu_power".to_string(), rng.gen_range(19.0..250.0).to_string());
        sensor_values.insert("cpu_voltage".to_string(), rng.gen_range(0.0..2.5).to_string());
        sensor_values.insert("cpu_frequency".to_string(), rng.gen_range(-1..4900).to_string());

        sensor_values.insert("gpu_utilization".to_string(), rng.gen_range(0..100).to_string());
        sensor_values.insert("gpu_die_temp".to_string(), rng.gen_range(29..100).to_string());
        sensor_values.insert("gpu_package_temp".to_string(), rng.gen_range(29..100).to_string());
        sensor_values.insert("gpu_power".to_string(), rng.gen_range(19.0..250.0).to_string());
        sensor_values.insert("gpu_voltage".to_string(), rng.gen_range(0.0..2.5).to_string());
        sensor_values.insert("gpu_frequency".to_string(), rng.gen_range(-1..3000).to_string());

        let sensor_request = json!({
            "user_id": 4711,
            "topic": "cats",
            "message": serde_json::to_string(&sensor_values).unwrap()
        });

//        let sensor_request = "\"user_id\": 4711, \"topic\": \"cats\", \"message\": {}";

        let request_url = "http://127.0.0.1:8000/publish";

        println!("Sending: {}", sensor_request);
        let response = blocking::Client::new()
            .post(request_url)
            .header("Content-Type", "application/json")
            .json(&sensor_request)
            .send();

        println!("Result: {:?}", response);

        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args);
    if args.len() > 1 && args[1] == "generate" {
        generate_sensor_values();
        return
    }

    let (mut rl, thread) = raylib::init()
        .size(1024, 600)
        .title("SensorPanel")
        .build();

    rl.set_target_fps(1);

    let fonts = load_fonts(&mut rl, &thread);
    let textures = load_textures(&mut rl, &thread);

    let mut rng = rand::thread_rng();

    let valueReceiver = ws_client_setup();

    let mut lastValues: HashMap<String, String> = HashMap::new();
    lastValues.insert("cpu_utilization".to_string(), rng.gen_range(0..100).to_string());
    lastValues.insert("cpu_die_temp".to_string(), rng.gen_range(29..100).to_string());
    lastValues.insert("cpu_package_temp".to_string(), rng.gen_range(29..100).to_string());
    lastValues.insert("cpu_power".to_string(), rng.gen_range(19.0..250.0).to_string());
    lastValues.insert("cpu_voltage".to_string(), rng.gen_range(0.0..2.5).to_string());
    lastValues.insert("cpu_frequency".to_string(), rng.gen_range(-1..4900).to_string());

    lastValues.insert("gpu_utilization".to_string(), rng.gen_range(0..100).to_string());
    lastValues.insert("gpu_die_temp".to_string(), rng.gen_range(29..100).to_string());
    lastValues.insert("gpu_package_temp".to_string(), rng.gen_range(29..100).to_string());
    lastValues.insert("gpu_power".to_string(), rng.gen_range(19.0..250.0).to_string());
    lastValues.insert("gpu_voltage".to_string(), rng.gen_range(0.0..2.5).to_string());
    lastValues.insert("gpu_frequency".to_string(), rng.gen_range(-1..3000).to_string());

    while !rl.window_should_close() {

        let clonedLastValues = lastValues.clone();
        let receivedValues = match valueReceiver.try_recv() {
            Ok(values) => { lastValues = values.clone(); values }
            Err(_) => { clonedLastValues }
        };

        let sensor_data = SensorData {
            values: receivedValues
        };
        let mut d = rl.begin_drawing(&thread);
        draw_windows_panel(&fonts, &textures, &mut d, &sensor_data);
    }
}

