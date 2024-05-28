use ncurses::*;

pub enum Action {
    Quit,
    Pause,
    Err,
}

pub fn get_actions() -> Action {
    let key_pressed: i32 = getch();

    match key_pressed {
        80 | 112 => return Action::Pause,
        81 | 113 => return Action::Quit,
        _ => return Action::Err,
    }
}
