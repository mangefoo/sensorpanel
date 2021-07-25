use std::process::Command;
use std::path::Path;

pub trait ScreenControl {
    fn turn_on(&self) -> bool;
    fn turn_off(&self) -> bool;
}

const TVSERVICE_PATH: &str = "/usr/bin/tvservice";
const UHUBCTL_PATH: &str = "/home/pi/devel/uhubctl/uhubctl";

struct TvServiceScreenControl {}

impl ScreenControl for TvServiceScreenControl {
    fn turn_on(&self) -> bool {
        Command::new(TVSERVICE_PATH)
            .arg("-p")
            .status()
            .expect("Failed to turn on screen with tvservice");

        Command::new(UHUBCTL_PATH)
            .args(&["-l", "1-1", "-a", "1", "-r", "100"])
            .status()
            .expect("Failed to turn on screen with uhubctl");

        return true;
    }

    fn turn_off(&self) -> bool {
        Command::new(TVSERVICE_PATH)
            .arg("-o")
            .status()
            .expect("Failed to turn on screen with tvservice");

        Command::new(UHUBCTL_PATH)
            .args(&["-l", "1-1", "-a", "0", "-r", "100"])
            .status()
            .expect("Failed to turn on screen with uhubctl");

        return true;
    }
}

struct LoggingScreenControl {}

impl ScreenControl for LoggingScreenControl {
    fn turn_on(&self) -> bool {
        println!("Turning on screen");
        return true;
    }

    fn turn_off(&self) -> bool {
        println!("Turning off screen");
        return true;
    }
}

pub fn get_screen_control() -> Box<dyn ScreenControl> {
    if Path::new(TVSERVICE_PATH).exists() {
        return Box::new(TvServiceScreenControl{});
    }

    return Box::new(LoggingScreenControl{});
}