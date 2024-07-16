use crate::timer::Timer;

pub enum MenuState {
    TimeLimit,
}

impl Default for MenuState {
    fn default() -> Self {
        MenuState::TimeLimit
    }
}

pub fn get_current_value(timer: &Timer, state: &MenuState) -> String {
    String::from(format!(
        "{}",
        match state {
            MenuState::TimeLimit => timer.session.time_limit_minutes,
        }
    ))
}

pub fn get_value_name(state: &MenuState) -> String {
    String::from(match state {
        MenuState::TimeLimit => "Time Limit",
    })
}
