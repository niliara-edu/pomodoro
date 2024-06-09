pub mod control;
pub mod menu;
pub mod terminal_ui;
pub mod timer;

use timer::State;
use timer::Timer;

/* todo:
- Separate UI functionality from ncurses, so we can change to another ui library later
- Use sleep threads and make the clock asyncronous
- Update things ONLY when necessary (includes checking the time and updating the ui)
*/

fn main() {
    let mut timer: Timer = timer::create_default_timer();
    //let mut window_size = terminal_ui::get_window_size();

    terminal_ui::start_ui();

    loop {
        control::process_actions(&mut timer);
        timer.update();
        terminal_ui::update(&timer);
        //terminal_ui::update_window_size(&mut window_size);

        if matches!(timer.state, State::Finished) {
            break;
        }
    }

    terminal_ui::end_ui();
}
