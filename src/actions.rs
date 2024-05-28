use ncurses::*;

pub enum Action {
    Quit,
    Err,
}

pub fn get_actions() -> Action {
    let key_pressed: i32 = getch();

    match key_pressed {
        81 | 113 => return Action::Quit,
        _ => return Action::Err,
    }
}
