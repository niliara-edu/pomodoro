use ncurses::{getch, KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_UP};

pub enum Action {
    Quit,
    Pause,
    Stop,
    Up,
    Down,
    Left,
    Right,
    Enter,
    Err,
}

pub fn get_action_pressed() -> Action {
    let key_pressed: i32 = getch();

    match key_pressed {
        80 | 112 => return Action::Pause,
        81 | 113 => return Action::Quit,
        83 | 115 => return Action::Stop,
        KEY_UP | 75 | 107 => return Action::Up,
        KEY_DOWN | 74 | 106 => return Action::Down,
        KEY_LEFT | 73 | 105 => return Action::Left,
        KEY_RIGHT | 76 | 108 => return Action::Right,
        10 => return Action::Enter,
        _ => return Action::Err,
    }
}
