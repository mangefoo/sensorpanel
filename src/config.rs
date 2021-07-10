use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_resources")]
    pub resources: String,
    #[serde(default = "default_relay_host")]
    pub relay_host: String
}

fn default_resources() -> String { "./resources".to_string() }
fn default_relay_host() -> String { "127.0.0.1".to_string() }

pub fn read_config(filename: &str) -> Config {
    let config_str = fs::read_to_string(filename)
        .expect("Could not read config file");

    return toml::from_str(&config_str.to_string())
        .expect("Failed to parse config file")
}