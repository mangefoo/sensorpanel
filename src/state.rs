use crate::data::{SensorData};
use std::time::SystemTime;
use crate::log::{Log, LogExt, LogLevel};

#[derive(Clone, Debug, PartialEq)]
pub enum ScreenState {
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

#[derive(PartialEq)]
pub enum Action {
    ScreenOn,
    ScreenOff
}

pub trait StateExt {
    fn transfer_to(self: Self, _: &mut Self);
    fn update_presence(self: &Self, present: bool, presence_threshold_secs: u32) -> Self;
    fn toggle_screen_state(self: &Self) -> Self;
    fn state_change_actions(self: &Self, other: &Self) -> Vec<Action>;
    fn init() -> Self;
}

impl StateExt for State {
    fn transfer_to(self: Self, mut other: &mut State) {
        other.presence = self.presence;
        other.screen_state = self.screen_state;
        other.screen_on = self.screen_on;
        other.sensor_data = self.sensor_data;
    }

    fn update_presence(self: &State, present: bool, presence_threshold_secs: u32) -> State {
        let current_time = SystemTime::now();
        let duration_since_switch_to_false = current_time.duration_since(self.presence.last_switch_to_false).unwrap();
        let mut new_state = self.clone();
        let mut new_presence = &mut new_state.presence;

        if present && new_presence.present == Present::NO {
            new_presence.present = Present::YES;
        } else if !present && new_presence.present == Present::PENDING && duration_since_switch_to_false.as_secs() > presence_threshold_secs as u64 {
            new_presence.present = Present::NO;
        } else if !present && new_presence.present == Present::YES {
            new_presence.present = Present::PENDING;
            new_presence.last_switch_to_false = SystemTime::now();
        } else if present {
            new_presence.present = Present::YES;
        }

        if self.presence.present != new_presence.present {
            Log::log(LogLevel::TRACE, &*format!("Presence transitioned to {:?}", new_presence.present));
        }

        new_state.screen_on = new_state.screen_state == ScreenState::AUTO && new_presence.present != Present::NO;

        return new_state;
    }

    fn toggle_screen_state(self: &State) -> State {
        let mut new_state = self.clone();

        new_state.screen_state = match self.screen_state {
            ScreenState::OFF => ScreenState::AUTO,
            ScreenState::AUTO => if self.screen_on { ScreenState::OFF } else { ScreenState::AUTO }
        };

        new_state.screen_on = match new_state.screen_state {
            ScreenState::OFF => { false }
            ScreenState::AUTO => { self.presence.present != Present::NO }
        };

        return new_state;
    }

    fn state_change_actions(self: &Self, previous: &Self) -> Vec<Action> {
        let mut actions = Vec::new();

        if !self.screen_on && previous.screen_on {
            actions.push(Action::ScreenOff);
        } else if self.screen_on && !previous.screen_on {
            actions.push(Action::ScreenOn);
        }

        actions
    }

    fn init() -> State {
        State {
            sensor_data: Vec::new(),
            screen_on: true,
            screen_state: ScreenState::AUTO,
            presence: PresenceData {
                present: Present::YES,
                last_switch_to_false: SystemTime::now()
            }
        }
    }
}