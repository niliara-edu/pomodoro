pub mod timer;

// the ncurses code was pretty bad so i replaced it with iced (first time!)
// I am so sorry to whoever is reading this

use iced::keyboard;
use iced::time;
use iced::widget::{column, container, text};
use iced::{executor, Application, Command, Element, Length, Settings, Subscription, Theme};

/*
Project structure according to UAB:
- classes folder
- functions folder
- getting/handling actions inside pomodoro
*/

pub fn main() -> iced::Result {
    Pomodoro::run(Settings::default())
}

struct Pomodoro {
    clock: timer::Timer,
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
    Pause,
    Stop,
    Start,
}

impl Application for Pomodoro {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Pomodoro, Command<Self::Message>) {
        (
            Pomodoro {
                clock: crate::timer::Timer::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Pomodoro")
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::Tick => self.clock.update(),
            Message::Pause => self.clock.pause_trigger(),
            Message::Stop => self.clock.stop(),
            Message::Start => self.clock.start_trigger(),
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let time_text = text(format!(
            "{:02}:{:02} / {:02}:{:02}",
            self.clock.time_now / 60,
            self.clock.time_now % 60,
            self.clock.time_limit_seconds / 60,
            self.clock.time_limit_seconds % 60,
        ));

        let state = text(match self.clock.state {
            timer::State::Running => "running",
            timer::State::Paused => "paused",
            timer::State::Stopped => "stopped",
            timer::State::Finished => "finished",
        });

        let content = column![time_text, state];

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let tick = time::every(time::Duration::from_millis(10)).map(|_| Message::Tick);

        fn handle_hotkey(key: keyboard::Key, _modifiers: keyboard::Modifiers) -> Option<Message> {
            use keyboard::key;

            match key.as_ref() {
                keyboard::Key::Character("p") => Some(Message::Pause),
                keyboard::Key::Character("s") => Some(Message::Stop),
                keyboard::Key::Named(key::Named::Enter) => Some(Message::Start),
                _ => None,
            }
        }

        Subscription::batch(vec![tick, keyboard::on_key_press(handle_hotkey)])
    }

    fn theme(&self) -> Self::Theme {
        Theme::Oxocarbon
    }
}
