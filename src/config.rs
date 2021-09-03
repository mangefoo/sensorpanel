use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default = "default_resources")]
    pub resources: String,
    #[serde(default = "default_relay_host")]
    pub relay_host: String,
    #[serde(default = "default_presence_threshold_secs")]
    pub presence_threshold_secs: u32
}

fn default_resources() -> String { "./resources".to_string() }
fn default_relay_host() -> String { "127.0.0.1".to_string() }
fn default_presence_threshold_secs() -> u32 { 600 as u32 }

pub fn read_config(filename: &str) -> Config {
    let config_str = fs::read_to_string(filename)
        .expect("Could not read config file");

    return toml::from_str(&config_str.to_string())
        .expect("Failed to parse config file")
}