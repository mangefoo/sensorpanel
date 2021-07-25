use std::collections::HashMap;
use std::time::SystemTime;

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

#[derive(Clone, Debug, PartialEq)]
pub enum Present {
    YES,
    NO,
    PENDING
}

#[derive(Clone, Debug)]
pub struct PresenceData {
    pub present: Present,
    pub last_switch_to_false: SystemTime
}

#[derive(Clone, Debug)]
pub struct State {
    pub sensor_data: Vec<SensorData>,
    pub screen_on: bool,
    pub screen_state: ScreenState,
    pub presence: PresenceData
}