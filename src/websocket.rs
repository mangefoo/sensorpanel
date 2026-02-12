use std::time::{Instant, Duration};
use serde::{Serialize, Deserialize, Deserializer};
use serde_json::json;
use std::collections::HashMap;
use crate::config::Config;
use std::sync::mpsc::{Sender, Receiver, RecvError};
use std::sync::{mpsc, Arc};
use std::thread;
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
    let relay_host = config.relay_host.clone();

    thread::spawn(move || {
        ws_connect_loop(&relay_host, tx);
    });

    rx
}

fn ws_connect_loop(relay_host: &str, sender: Sender<SensorReport>) {
    loop {
        let id = match ws_register_client(relay_host) {
            Ok(id) => id,
            Err(error) => {
                Log::log(LogLevel::ERROR, &format!("Failed to register: {}", error));
                thread::sleep(Duration::from_secs(5));
                continue;
            }
        };

        Log::log(LogLevel::INFO, &format!("Registered with ID: {}", id));
        let url = format!("ws://{}/ws/{}", relay_host, id);

        match connect(Url::parse(&url).unwrap()) {
            Ok((mut socket, _)) => {
                Log::log(LogLevel::INFO, "Connected to WebSocket");
                loop {
                    match socket.read_message() {
                        Ok(msg) => {
                            if let Ok(text) = msg.to_text() {
                                match serde_json::from_str::<SensorReport>(text) {
                                    Ok(report) => {
                                        if let Err(e) = sender.send(report) {
                                            Log::log(LogLevel::ERROR, &format!("Channel send failed: {}", e));
                                            return;
                                        }
                                    }
                                    Err(e) => {
                                        Log::log(LogLevel::ERROR, &format!("Failed to parse message: {}", e));
                                    }
                                }
                            } else {
                                Log::log(LogLevel::DEBUG, "Received non-text WebSocket message");
                            }
                        }
                        Err(e) => {
                            Log::log(LogLevel::ERROR, &format!("WebSocket read error: {}", e));
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                Log::log(LogLevel::ERROR, &format!("WebSocket connect failed: {}", e));
            }
        }

        Log::log(LogLevel::INFO, "Reconnecting in 5 seconds...");
        thread::sleep(Duration::from_secs(5));
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
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    let register_response: RegisterResponse = response.json()
        .map_err(|e| format!("JSON parse failed: {}", e))?;

    Ok(register_response.id)
}
