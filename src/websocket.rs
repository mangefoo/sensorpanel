use std::time::Instant;
use serde::{Serialize, Deserialize, Deserializer};
use serde_json::json;
use std::collections::HashMap;
use crate::config::Config;
use std::sync::mpsc::{Sender, Receiver, RecvError};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;
use tungstenite::connect;
use reqwest::{Url, blocking};
use crate::state::State;
use crate::log::{Log, LogExt, LogLevel};
use crate::context::Context;

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

pub(crate) trait WebSocketExt {
    fn receiver_loop(context: &Context, event_handler: fn(SensorReport, &mut State, &Config), error_handler: fn(RecvError));
}

pub struct WebSocket {}

impl WebSocketExt for WebSocket {
    fn receiver_loop(context: &Context, event_handler: fn(SensorReport, &mut State, &Config), error_handler: fn(RecvError)) {
        let value_receiver = ws_client_setup(&context.config);
        let thread_state = context.state.clone();
        let thread_config = context.config.clone();

        thread::spawn(move || {
            loop {
                let state = Arc::clone(&thread_state);

                match value_receiver.recv() {
                    Ok(event) => {
                        if let Ok(mut locked_state) = state.lock() {
                            event_handler(event, &mut *locked_state, &thread_config);
                        }
                    }
                    Err(error) => {
                        error_handler(error);
                    }
                }
            }
        });
    }
}

fn ws_client_setup(config: &Config) -> Receiver<SensorReport> {
    let (tx, rx): (Sender<SensorReport>, Receiver<SensorReport>) = mpsc::channel();
    let thread_tx = tx.clone();
    let relay_host= config.relay_host.clone();
    let thread_fn = move || {
        loop {
            let id = match ws_register_client(&relay_host) {
                Err(error) => {
                    Log::log(LogLevel::ERROR, &format!("Failed to register websocket client: {}", error));
                    thread::sleep(Duration::from_secs(5));
                    continue;
                }
                Ok(url) => url
            };

            Log::log(LogLevel::DEBUG, &format!("Got WS ID: {}", id));

            let ws_url = format!("ws://{}/ws/{}", relay_host, id);
            if let Err(error) = ws_read_loop(&ws_url, &thread_tx) {
                Log::log(LogLevel::ERROR, &format!("WebSocket loop ended: {}", error));
            }

            thread::sleep(Duration::from_secs(2));
        }
    };

    thread::spawn(thread_fn);

    return rx;
}

fn ws_read_loop(url: &str, value_sender: &Sender<SensorReport>) -> Result<(), String> {
    let parsed_url = Url::parse(url)
        .map_err(|error| format!("Failed to parse WebSocket URL {}: {}", url, error))?;

    let (mut socket, response) =
        connect(parsed_url).map_err(|error| format!("Can't connect to websocket {}: {}", url, error))?;

    Log::log(LogLevel::DEBUG, "Connected to the server");
    Log::log(LogLevel::DEBUG, &format!("Response HTTP code: {}", response.status()));
    Log::log(LogLevel::DEBUG, "Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        Log::log(LogLevel::DEBUG, &format!("* {}", header));
    }

    loop {
        let msg = match socket.read_message() {
            Ok(msg) => msg,
            Err(error) => return Err(format!("Error reading websocket message: {}", error))
        };

        let payload = match msg.to_text() {
            Ok(payload) => payload,
            Err(error) => {
                Log::log(LogLevel::ERROR, &format!("Received non-text websocket payload: {}", error));
                continue;
            }
        };

        let report: SensorReport = match serde_json::from_str(payload) {
            Ok(report) => report,
            Err(error) => {
                Log::log(LogLevel::ERROR, &format!("Failed to parse sensor report JSON: {}", error));
                continue;
            }
        };

        if let Err(error) = value_sender.send(report) {
            return Err(format!("Failed to send report to receiver loop: {}", error));
        }
    }
}

fn ws_register_client(relay_host: &str) -> Result<String, String> {
    let register_body = json!({
        "topics": ["sensors", "actions"],
    });

    let request_url = format!("http://{}/register", relay_host);

    let response = blocking::Client::new()
        .post(request_url)
        .json(&register_body)
        .send()
        .map_err(|error| format!("Request failed: {}", error))?;

    if !response.status().is_success() {
        return Err(format!("Register failed with HTTP {}", response.status()));
    }

    Log::log(LogLevel::DEBUG, "Request OK");

    let register_response: RegisterResponse = response.json()
        .map_err(|error| format!("Parse json failed: {:?}", error))?;

    Ok(register_response.id)
}
