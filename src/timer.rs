use super::menu::MenuSection;
use std::time;

pub struct Timer {
    pub start_time: time::Instant,
    pub time_elapsed: time::Duration,
    pub start_pause_time: time::Instant,
    pub time_paused: time::Duration,

    pub time_limit_seconds: u16,
    pub time_now: u16,

    pub session: Session,
    pub state: State,

    pub menu_section: MenuSection,
}

impl Timer {
    pub fn update(&mut self) {
        self.time_elapsed = self.start_time.elapsed();
        self.update_time();
    }

    fn update_time(&mut self) {
        if !(matches!(self.state, State::Running)) {
            return;
        }

        let time_as_seconds: u16 = (self.time_elapsed - self.time_paused).as_secs() as u16;
        if self.time_now != time_as_seconds {
            self.time_now = time_as_seconds;
        }

        self.update_events();
    }

    fn update_events(&mut self) {
        if self.time_now >= self.time_limit_seconds {
            self.next_stage();
        }
    }

    fn next_stage(&mut self) {
        self.state = State::Finished;
    }

    pub fn pause_trigger(&mut self) {
        match self.state {
            State::Running => self.pause(),
            State::Paused => self.unpause(),
            _ => return,
        }
    }

    fn pause(&mut self) {
        self.state = State::Paused;
        self.start_pause_time = time::Instant::now();
    }

    fn unpause(&mut self) {
        self.state = State::Running;
        self.time_paused += self.start_pause_time.elapsed();
    }

    pub fn stop(&mut self) {
        self.state = State::Stopped;
        self.time_now = 0;
    }

    pub fn start_trigger(&mut self) {
        if matches!(self.state, State::Stopped) {
            self.restart()
        }
    }

    fn restart(&mut self) {
        self.state = State::Running;
        self.start_time = time::Instant::now();
        self.time_now = 0;
        self.time_paused = time::Duration::ZERO;
    }
}

pub enum State {
    Running,
    Finished,
    Paused,
    Stopped,
}

pub fn create_default_timer() -> Timer {
    let timer_session: Session = create_default_session();
    let session_time_in_seconds: u16 = (timer_session.time_limit_minutes as u16) * 10; //change to 60 later

    let timer = Timer {
        start_time: time::Instant::now(),
        time_elapsed: time::Duration::ZERO,
        start_pause_time: time::Instant::now(),
        time_paused: time::Duration::ZERO,

        time_limit_seconds: session_time_in_seconds,
        time_now: 0,

        session: timer_session,
        state: State::Running, //change to Stopped later

        menu_section: MenuSection::TimeLimit,
    };

    return timer;
}

pub struct Session {
    pub short_break_time: u8,
    pub long_break_time: u8,
    pub time_limit_minutes: u8,

    pub stage_limit: u8,
    pub stage_now: u8,

    pub is_break_time: bool,
}

fn create_default_session() -> Session {
    let session = Session {
        short_break_time: 5,
        long_break_time: 30,
        time_limit_minutes: 1,
        stage_limit: 4,
        stage_now: 0,
        is_break_time: false,
    };

    return session;
}
