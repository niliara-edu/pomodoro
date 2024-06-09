use super::timer::Timer;

pub enum MenuSection {
    TimeLimit = 0,
}

pub struct Option<'a> {
    pub value: MenuSection,
    pub change: u8,
    pub range: std::ops::Range<u8>,
    pub name: &'a str,
    pub sufix: &'a str,
}

const _OPTIONS: [Option; 1] = [Option {
    value: MenuSection::TimeLimit,
    change: 5,
    range: (5..20),

    name: "Duration",
    sufix: "m",
}];

pub fn get_target_value(timer: &mut Timer) -> &mut u8 {
    match timer.menu_section {
        MenuSection::TimeLimit => return &mut timer.session.time_limit_minutes,
    }
}

pub fn increase(timer: &mut Timer) {
    modify_timer_value(timer, false)
}

pub fn decrease(timer: &mut Timer) {
    modify_timer_value(timer, true)
}

fn modify_timer_value(timer: &mut Timer, _pole: bool) {
    let _target_variable: &mut u8 = get_target_value(timer);
    //let result: u8 =
}
