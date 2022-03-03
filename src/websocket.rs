use std::time::Instant;
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
                        let mut locked_state = state.lock();
                        if locked_state.is_ok() {
                            event_handler(event, &mut *locked_state.unwrap(), &thread_config);
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
        let id = match ws_register_client(&relay_host) {
            Err(error) => panic!("Failed to get WS URL: {}", error),
            Ok(url) => url
        };

        Log::log(LogLevel::DEBUG, &*format!("Got WS ID: {}", id));

        ws_read_loop(format!("ws://{}/ws/{}", relay_host, id), thread_tx);
    };

    thread::spawn(thread_fn);

    return rx;
}

fn ws_read_loop(url: String, value_sender: Sender<SensorReport>) {
    let (mut socket, response) =
        connect(Url::parse(&url).unwrap()).expect("Can't connect");

    Log::log(LogLevel::DEBUG, "Connected to the server");
    Log::log(LogLevel::DEBUG, &*format!("Response HTTP code: {}", response.status()));
    Log::log(LogLevel::DEBUG, "Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        Log::log(LogLevel::DEBUG,&*format!("* {}", header));
    }

    loop {
        let msg = socket.read_message().expect("Error reading message");
        let report: SensorReport = serde_json::from_str(msg.to_text().unwrap()).unwrap();
        let result = value_sender.send(report);
        match result {
            Err(error) => { Log::log(LogLevel::ERROR, &*format!("Failed to send request: {}", error))}
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
        Ok(response) => { Log::log(LogLevel::DEBUG, "Request OK"); response }
    };

    let register_response: RegisterResponse = match response.json() {
        Err(error) => panic!("Parse json failed: {:?}", error),
        Ok(json) => json
    };

    Ok(register_response.id)
}
