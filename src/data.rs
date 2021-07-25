use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct SensorData {
    pub reporter: String,
    pub values: HashMap<String, String>,
}

#[derive(Clone, Debug)]
pub enum ScreenState {
    ON,
    OFF,
    AUTO
}

#[derive(Clone, Debug)]
pub struct State {
    pub sensor_data: Vec<SensorData>,
    pub screen_on: bool,
    pub screen_state: ScreenState,
    pub presence: bool
}