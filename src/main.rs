pub mod timer;

// highly copied from some of the iced examples
// (it's my first time, please be patient)

use iced::alignment;
use iced::keyboard;
use iced::mouse;
use iced::time;
use iced::widget::canvas::{stroke, Cache, Geometry, LineCap, Path, Stroke};
use iced::widget::{canvas, column, container, text};
use iced::{
    executor, Application, Command, Degrees, Element, Length, Point, Renderer, Settings,
    Subscription, Theme, Vector,
};
use std::f32::consts::TAU;

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
    timer: timer::Timer,
    clock: Cache,
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
                timer: crate::timer::Timer::default(),
                clock: Cache::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Pomodoro")
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::Tick => {
                self.timer.update();
                self.clock.clear();
            }

            Message::Pause => self.timer.pause_trigger(),
            Message::Stop => self.timer.stop(),
            Message::Start => self.timer.start_trigger(),
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let time_text = text(format!(
            "{:02}:{:02} / {:02}:{:02}",
            self.timer.time_now / 60,
            self.timer.time_now % 60,
            self.timer.time_limit_seconds / 60,
            self.timer.time_limit_seconds % 60,
        ))
        .horizontal_alignment(alignment::Horizontal::Center);

        let state = text(match self.timer.state {
            timer::State::Running => "running",
            timer::State::Paused => "paused",
            timer::State::Stopped => "stopped",
            timer::State::Finished => "finished",
        });

        let numbers_n_shit = container(column![time_text, state])
            .width(Length::Fill)
            .height(50)
            .center_x();

        let canvas = canvas(self as &Self).width(Length::Fill).height(200);
        let content = column![canvas, numbers_n_shit].spacing(20).padding(20);

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

impl<Message> canvas::Program<Message> for Pomodoro {
    type State = ();

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let clock = self.clock.draw(renderer, bounds.size(), |frame| {
            let palette = theme.extended_palette();

            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.0;

            let background = Path::circle(center, radius);
            frame.fill(&background, palette.secondary.strong.color);

            let short_hand = Path::line(Point::ORIGIN, Point::new(0.0, -0.5 * radius));

            let long_hand = Path::line(Point::ORIGIN, Point::new(0.0, -0.8 * radius));

            let width = radius / 100.0;

            let thin_stroke = || -> Stroke {
                Stroke {
                    width,
                    style: stroke::Style::Solid(palette.primary.strong.text),
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                }
            };

            let wide_stroke = || -> Stroke {
                Stroke {
                    width: width * 3.0,
                    style: stroke::Style::Solid(palette.primary.strong.text),
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                }
            };

            frame.translate(Vector::new(center.x, center.y));

            frame.with_save(|frame| {
                frame.rotate(
                    self.timer.time_now as f32 / self.timer.time_limit_seconds as f32 * TAU,
                );
                frame.stroke(&short_hand, wide_stroke());
            });

            frame.with_save(|frame| {
                frame.rotate(self.timer.time_now as f32 / 60. * TAU);
                frame.stroke(&long_hand, thin_stroke());
            });
        });

        vec![clock]
    }
}
