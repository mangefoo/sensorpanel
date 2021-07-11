use std::collections::HashMap;

#[derive(Clone)]
pub struct SensorData {
    pub reporter: String,
    pub values: HashMap<String, String>,
}
