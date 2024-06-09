use super::timer::State;
use super::timer::Timer;

use super::menu;

mod actions;
use actions::Action;

pub fn process_actions(timer: &mut Timer) {
    let action: Action = actions::get_action_pressed();

    match action {
        Action::Quit => timer.state = State::Finished,
        Action::Pause => timer.pause_trigger(),
        Action::Stop => {
            timer.stop();
            ncurses::clear();
        }
        Action::Enter => timer.start_trigger(),
        Action::Up | Action::Down | Action::Left | Action::Right => update_values(timer, action),
        _ => (),
    }
}

fn update_values(timer: &mut Timer, action: Action) {
    if !(matches!(timer.state, State::Stopped)) {
        return;
    }

    match action {
        Action::Left => menu::increase(timer),
        Action::Right => menu::decrease(timer),
        _ => return,
    }
}
