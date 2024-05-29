extern crate ncurses;
use ncurses::*;

use super::timer::State;
use super::timer::Timer;

pub fn start_ui() {
    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    noecho();
    nodelay(stdscr(), true);
}

pub fn update_running_ui(timer: &Timer) {
    let window_size = get_window_size();
    print_image(timer, window_size);
    print_data(timer, window_size);

    refresh();
}

pub fn get_window_size() -> (i32, i32) {
    let mut height: i32 = 0;
    let mut width: i32 = 0;
    getmaxyx(ncurses::stdscr(), &mut height, &mut width);

    return (height, width);
}

pub fn update_window_size(window_size: &mut (i32, i32)) {
    let current_window_size = &get_window_size();
    if window_size != current_window_size {
        window_size.0 = current_window_size.0;
        window_size.1 = current_window_size.1;
        ncurses::clear();
    }
}

fn print_image(timer: &Timer, window_size: (i32, i32)) {
    let image_position: (i32, i32) = (
        window_size.0 - 16,
        ((window_size.1 as f32) / 2.0 - 6.0) as i32,
    );

    match timer.time_now % 4 {
        0 => {
            let _ = mvaddstr(image_position.0 + 0, image_position.1, "             ");
            let _ = mvaddstr(image_position.0 + 1, image_position.1, "    #####    ");
            let _ = mvaddstr(image_position.0 + 2, image_position.1, "   #  |  #   ");
            let _ = mvaddstr(image_position.0 + 3, image_position.1, "  #   |/  #  ");
            let _ = mvaddstr(image_position.0 + 4, image_position.1, "  #   O   #  ");
            let _ = mvaddstr(image_position.0 + 5, image_position.1, "  #       #  ");
            let _ = mvaddstr(image_position.0 + 6, image_position.1, "   #     #   ");
            let _ = mvaddstr(image_position.0 + 7, image_position.1, "    #####    ");
            let _ = mvaddstr(image_position.0 + 8, image_position.1, "             ");
        }

        1 => {
            let _ = mvaddstr(image_position.0 + 0, image_position.1, "             ");
            let _ = mvaddstr(image_position.0 + 1, image_position.1, "    #####    ");
            let _ = mvaddstr(image_position.0 + 2, image_position.1, "   #     #   ");
            let _ = mvaddstr(image_position.0 + 3, image_position.1, "  #    /  #  ");
            let _ = mvaddstr(image_position.0 + 4, image_position.1, "  #   O---#  ");
            let _ = mvaddstr(image_position.0 + 5, image_position.1, "  #       #  ");
            let _ = mvaddstr(image_position.0 + 6, image_position.1, "   #     #   ");
            let _ = mvaddstr(image_position.0 + 7, image_position.1, "    #####    ");
            let _ = mvaddstr(image_position.0 + 8, image_position.1, "             ");
        }

        2 => {
            let _ = mvaddstr(image_position.0 + 0, image_position.1, "             ");
            let _ = mvaddstr(image_position.0 + 1, image_position.1, "    #####    ");
            let _ = mvaddstr(image_position.0 + 2, image_position.1, "   #     #   ");
            let _ = mvaddstr(image_position.0 + 3, image_position.1, "  #    /  #  ");
            let _ = mvaddstr(image_position.0 + 4, image_position.1, "  #   O   #  ");
            let _ = mvaddstr(image_position.0 + 5, image_position.1, "  #   |   #  ");
            let _ = mvaddstr(image_position.0 + 6, image_position.1, "   #  |  #   ");
            let _ = mvaddstr(image_position.0 + 7, image_position.1, "    #####    ");
            let _ = mvaddstr(image_position.0 + 8, image_position.1, "             ");
        }

        3 => {
            let _ = mvaddstr(image_position.0 + 0, image_position.1, "             ");
            let _ = mvaddstr(image_position.0 + 1, image_position.1, "    #####    ");
            let _ = mvaddstr(image_position.0 + 2, image_position.1, "   #     #   ");
            let _ = mvaddstr(image_position.0 + 3, image_position.1, "  #    /  #  ");
            let _ = mvaddstr(image_position.0 + 4, image_position.1, "  #---O   #  ");
            let _ = mvaddstr(image_position.0 + 5, image_position.1, "  #       #  ");
            let _ = mvaddstr(image_position.0 + 6, image_position.1, "   #     #   ");
            let _ = mvaddstr(image_position.0 + 7, image_position.1, "    #####    ");
            let _ = mvaddstr(image_position.0 + 8, image_position.1, "             ");
        }

        _default => {
            println!("nothing");
        }
    }
}

fn print_data(timer: &Timer, window_size: (i32, i32)) {
    let time_text_position = (window_size.1 - 16, window_size.0 - 2);
    let statusbar_position = (window_size.1 - 8, 0);
    print_time_text(timer, time_text_position);
    print_progressbar(timer, window_size);
    print_statusbar(timer, statusbar_position);
}

fn print_time_text(timer: &Timer, time_text_position: (i32, i32)) {
    let _ = mvaddstr(
        time_text_position.1,
        time_text_position.0,
        format!(
            "{} / {}",
            format_seconds_to_time(timer.time_now.try_into().unwrap()),
            format_seconds_to_time(timer.time_limit.try_into().unwrap())
        )
        .as_str(),
    );
}

fn print_progressbar(timer: &Timer, window_size: (i32, i32)) {
    let progressbar_position: Vec<i32> = vec![
        get_scaled_progressbar_size(timer, window_size.1),
        window_size.0 - 1,
    ];

    let _ = mvaddstr(
        progressbar_position[1],
        0,
        &"=".repeat(progressbar_position[0].try_into().unwrap()),
    );

    let _ = mvaddstr(progressbar_position[1], progressbar_position[0], ">");
}

fn get_scaled_progressbar_size(timer: &Timer, window_x: i32) -> i32 {
    let limit_x = window_x - 2;
    let scaled_progressbar_size: f32 =
        (timer.time_now as f32) / (timer.time_limit as f32) * (limit_x as f32);
    return scaled_progressbar_size as i32;
}

fn format_seconds_to_time(seconds: i32) -> String {
    let time_in_minutes: i32 = seconds / 60;
    let time_in_seconds: i32 = seconds % 60;
    let formatted_time: String = format!("{:02}:{:02}", time_in_minutes, time_in_seconds);
    return formatted_time;
}

fn print_statusbar(timer: &Timer, statusbar_position: (i32, i32)) {
    let _ = mvaddstr(
        statusbar_position.1,
        statusbar_position.0,
        format!("[{}]", get_statuskeys(timer)).as_str(),
    );
}

fn get_statuskeys(timer: &Timer) -> char {
    match timer.state {
        State::Finished => return 'a',
        State::Running => return 'r',
        State::Paused => return 'p',
        State::Stopped => return 's',
    }
}

pub fn end_ui() {
    endwin();
}
