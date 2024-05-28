use std::time;

pub struct Timer {
    pub start_time: time::Instant,
    pub time_last_check: u64,

    pub time_limit: u64,
    pub time_now: u64,

    pub session: Session,
    pub state: State,
}

impl Timer {
    pub fn update_time(&mut self) {
        self.time_now = self.start_time.elapsed().as_secs();
    }

    pub fn time_has_changed(&mut self) -> bool {
        if self.time_now != self.time_last_check {
            self.time_last_check = self.time_now;
            return true;
        }
        return false;
    }

    pub fn update_events(&mut self) {
        if self.time_now >= self.time_limit {
            self.state = State::Finished;
        };
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

        time_limit: session_time_in_seconds,
        time_now: 0,
        time_last_check: 0,
        session: create_default_session(),
        state: State::Running,
    };

    return timer;
}

pub struct Session {
    pub short_break_time: u64,
    pub long_break_time: u64,
    pub time_limit: u64,

    pub stage_limit: u32,
    pub stage_now: u32,

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
