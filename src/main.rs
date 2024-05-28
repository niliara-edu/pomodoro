pub mod actions;
pub mod terminal_ui;
pub mod timer;

use actions::Action;
use timer::State;
use timer::Timer;

fn main() {
    terminal_ui::start_ui();

    let mut timer: Timer = timer::create_default_timer();
    let mut window_size = terminal_ui::get_window_size();

    terminal_ui::update(&timer);

    loop {
        terminal_ui::update_window_size(&mut window_size);
        timer.update_time();
        process_actions(&mut timer);

        terminal_ui::update(&timer);
        timer.update_events();

        if matches!(timer.state, State::Finished) {
            break;
        }
    }

    terminal_ui::end_ui();
}

fn process_actions(timer: &mut Timer) {
    let action: Action = actions::get_actions();

    match action {
        Action::Quit => timer.state = State::Finished,
        Action::Err => return,
    }
}
