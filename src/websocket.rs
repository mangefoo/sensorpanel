use std::time::Instant;
use serde::{Serialize, Deserialize, Deserializer};
use serde_json::json;
use std::collections::HashMap;
use crate::config::Config;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use tungstenite::connect;
use reqwest::{Url, blocking};

pub fn set_to_current_instant<'de, D>(_: D) -> Result<Instant, D::Error>
    where
        D: Deserializer<'de>,
{
    Ok(Instant::now())
}

pub fn current_instant() -> Instant {
    Instant::now()
}

#[derive(Deserialize, Debug)]
struct RegisterResponse {
    id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SensorReport {
    pub(crate) reporter: String,
    pub(crate) topic: String,
    pub(crate) sensors: HashMap<String, String>,
    #[serde(default = "current_instant", deserialize_with="set_to_current_instant", skip_serializing)]
    pub(crate) received: Instant
}

pub fn ws_client_setup(config: &Config) -> Receiver<SensorReport> {
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
