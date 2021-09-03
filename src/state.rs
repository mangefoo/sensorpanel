use crate::data::{State, Present, PresenceData, ScreenState};
use std::time::SystemTime;

pub trait StateExt {
    fn transfer_to(self: Self, _: &mut Self);
}

impl StateExt for State {
    fn transfer_to(self: Self, mut other: &mut State) {
        other.presence = self.presence;
        other.screen_state = self.screen_state;
        other.screen_on = self.screen_on;
        other.sensor_data = self.sensor_data;
    }
}

pub fn update_presence(present: bool, current_presence: &PresenceData, presence_threshold_secs: u32) -> PresenceData {
    let current_time = SystemTime::now();
    let duration_since_switch_to_false = current_time.duration_since(current_presence.last_switch_to_false).unwrap();
    let mut new_presence = current_presence.clone();

    if present && new_presence.present == Present::NO {
        println!("Presence set to YES");
        new_presence.present = Present::YES;
    } else if !present && new_presence.present == Present::PENDING && duration_since_switch_to_false.as_secs() > presence_threshold_secs as u64 {
        println!("Presence set to NO");
        new_presence.present = Present::NO;
    } else if !present && new_presence.present == Present::YES {
        println!("Presence set to PENDING");
        new_presence.present = Present::PENDING;
        new_presence.last_switch_to_false = SystemTime::now();
    } else if present {
        println!("Presence reset to YES");
        new_presence.present = Present::YES;
    }

    return new_presence;
}

pub fn update_state_presence(current_state: &State, presence: PresenceData) -> State {
    let mut new_state = current_state.clone();

    if current_state.screen_state == ScreenState::OFF {
        return new_state;
    }

    if !current_state.screen_on && presence.present == Present::YES {
        new_state.screen_on = true;
    } else if current_state.screen_on && presence.present == Present::NO {
        new_state.screen_on = false;
    }

    new_state.presence = presence;

    return new_state;
}

pub fn toggle_screen_state(current_state: &State) -> State {
    let mut new_state = current_state.clone();

    new_state.screen_state = match current_state.screen_state {
        ScreenState::OFF => ScreenState::AUTO,
        ScreenState::AUTO => if current_state.screen_on { ScreenState::OFF } else { ScreenState::AUTO }
    };

    new_state.screen_on = match new_state.screen_state {
        ScreenState::OFF => { false }
        ScreenState::AUTO => { current_state.presence.present == Present::YES }
    };

    return new_state;
}