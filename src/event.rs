use crate::websocket::SensorReport;
use crate::state::{State, StateExt};
use crate::config::Config;
use crate::data::SensorData;

pub struct Event();

pub trait EventExt {
    fn handle(sensor_report: SensorReport, state: &State, config: &Config) -> State;
}

impl EventExt for Event {
    fn handle(sensor_report: SensorReport, state: &State, config: &Config) -> State {
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
}

fn handle_sensor(event: SensorReport, state: &State, config: &Config) -> State {
    let historical_reports_count = 500;

    let mut new_state = state.clone();

    if let Some(presence) = event.sensors.get("hue_presence") {
        handle_presence(presence, state, config);
    }

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
    state.update_presence(presence == "true", config.presence_threshold_secs)
}

fn handle_action(sensor_report: SensorReport, state: &State) -> State {
    if sensor_report.sensors.contains_key("toggle_screen") {
        state.toggle_screen_state()
    } else {
        state.clone()
    }
}