use std::collections::HashMap;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct SensorData {
    pub reporter: String,
    pub values: HashMap<String, String>,
    pub received: Instant,
}
