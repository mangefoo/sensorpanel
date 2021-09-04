use std::process::Command;
use std::path::Path;
use std::thread;
use crate::log::{Log, LogExt, LogLevel};

pub trait ScreenControl {
    fn turn_on(&self) -> bool;
    fn turn_off(&self) -> bool;
    fn should_clear_screen(&self) -> bool;
}

const TVSERVICE_PATH: &str = "/usr/bin/tvservice";
const UHUBCTL_PATH: &str = "/home/pi/devel/uhubctl/uhubctl";

struct TvServiceScreenControl {}

impl ScreenControl for TvServiceScreenControl {
    fn turn_on(&self) -> bool {
        thread::spawn(|| {
            Command::new(TVSERVICE_PATH)
                .arg("-p")
                .status()
                .expect("Failed to turn on screen with tvservice");

            if Path::new(UHUBCTL_PATH).exists() {
                Command::new(UHUBCTL_PATH)
                    .args(&["-l", "1-1", "-a", "1", "-r", "100"])
                    .status()
                    .expect("Failed to turn on screen with uhubctl");
            }
        });

        return true;
    }

    fn turn_off(&self) -> bool {
        thread::spawn(|| {
            Command::new(TVSERVICE_PATH)
                .arg("-o")
                .status()
                .expect("Failed to turn on screen with tvservice");

            if Path::new(UHUBCTL_PATH).exists() {
                Command::new(UHUBCTL_PATH)
                    .args(&["-l", "1-1", "-a", "0", "-r", "100"])
                    .status()
                    .expect("Failed to turn on screen with uhubctl");
            }
        });

        return true;
    }

    fn should_clear_screen(&self) -> bool { false }
}

struct SoftwareScreenControl {}

impl ScreenControl for SoftwareScreenControl {
    fn turn_on(&self) -> bool {
        Log::log(LogLevel::INFO, "Turning on screen");
        return true;
    }

    fn turn_off(&self) -> bool {
        Log::log(LogLevel::INFO, "Turning off screen");
        return true;
    }

    fn should_clear_screen(&self) -> bool { true }
}

pub fn get_screen_control() -> Box<dyn ScreenControl> {
    if Path::new(TVSERVICE_PATH).exists() {
        return Box::new(TvServiceScreenControl{});
    }

    return Box::new(SoftwareScreenControl {});
}