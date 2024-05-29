use std::time;

pub struct Timer {
    pub start_time: time::Instant,
    pub time_elapsed: time::Duration,
    pub start_pause_time: time::Instant,
    pub time_paused: time::Duration,

    pub time_limit: u64,
    pub time_now: u64,

    pub session: Session,
    pub state: State,
}

impl Timer {
    pub fn update_timer(&mut self) {
        self.time_elapsed = self.start_time.elapsed();
        self.update_time();
    }

    fn update_time(&mut self) {
        if matches!(self.state, State::Paused) {
            return;
        }

        let time_as_seconds: u64 = (self.time_elapsed - self.time_paused).as_secs();
        if self.time_now != time_as_seconds {
            self.time_now = time_as_seconds;
        }
    }

    pub fn update_events(&mut self) {
        if self.time_now >= self.time_limit {
            self.state = State::Finished;
        }
    }

    pub fn pause(&mut self) {
        if matches!(self.state, State::Paused) {
            self.state = State::Running;
            self.time_paused += self.start_pause_time.elapsed();
        } else {
            self.state = State::Paused;
            self.start_pause_time = time::Instant::now();
        }
    }
}

pub enum State {
    Running,
    Finished,
    Paused,
}

pub fn create_default_timer() -> Timer {
    let session_time_in_minutes: u64 = 2;
    let session_time_in_seconds: u64 = session_time_in_minutes * 60;

    let timer = Timer {
        start_time: time::Instant::now(),
        time_elapsed: time::Duration::ZERO,
        start_pause_time: time::Instant::now(),
        time_paused: time::Duration::ZERO,

        time_limit: session_time_in_seconds,
        time_now: 0,

        session: create_default_session(),
        state: State::Running,
    };

    return timer;
}

pub struct Session {
    pub short_break_time: u8,
    pub long_break_time: u8,
    pub time_limit: u8,

    pub stage_limit: u8,
    pub stage_now: u8,

    pub is_break_time: bool,
}

fn create_default_session() -> Session {
    let session = Session {
        short_break_time: 5,
        long_break_time: 30,
        time_limit: 25,
        stage_limit: 4,
        stage_now: 0,
        is_break_time: false,
    };

    return session;
}
