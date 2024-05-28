pub mod terminal_ui;
pub mod timer;

use timer::Timer;

fn main() {
    terminal_ui::start_ui();

    let mut pomodoro: Timer = timer::create_default_timer();
    let mut window_size = terminal_ui::get_window_size();
    terminal_ui::update(&pomodoro);

    loop {
        terminal_ui::update_window_size(&mut window_size);
        pomodoro.update_time();

        terminal_ui::update(&pomodoro);

        if pomodoro.is_finished() {
            break;
        }
    }

    terminal_ui::end_ui();
}
