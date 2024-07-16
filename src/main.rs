pub mod action_handler;
pub mod colored_keys;
pub mod menu;
pub mod timer;

use colored_keys::ColoredKeys;
use menu::MenuState;

use iced::alignment;
use iced::keyboard;
use iced::mouse;
use iced::time;
use iced::widget::canvas::{stroke, Cache, Geometry, LineCap, Path, Stroke};
use iced::widget::{canvas, column, container, text};
use iced::{
    executor, Application, Color, Command, Element, Length, Point, Renderer, Settings, Size,
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
    menu_state: MenuState,
    colored_keys: ColoredKeys,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    Pause,
    Stop,
    Start,
    ArrowPress(Arrow),
    ArrowRelease(Arrow),
}

#[derive(Debug, Clone)]
pub enum Arrow {
    Up,
    Down,
    Left,
    Right,
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
                menu_state: MenuState::default(),
                colored_keys: ColoredKeys::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Pomodoro")
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::Tick => self.timer.update(),
            Message::Pause => self.timer.pause_trigger(),
            Message::Stop => self.timer.stop(),
            Message::Start => self.timer.start_trigger(),
            Message::ArrowPress(arrow) => self.colored_keys.press(arrow),
            Message::ArrowRelease(arrow) => self.colored_keys.release(arrow),
        }

        self.clock.clear();
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let time_text = text(format!(
            "{:02}:{:02} / {:02}:{:02}",
            self.timer.time_now / 60,
            self.timer.time_now % 60,
            self.timer.time_limit_seconds / 60,
            self.timer.time_limit_seconds % 60,
        ));

        let state = text(match self.timer.state {
            timer::State::Running => "running",
            timer::State::Paused => "paused",
            timer::State::Stopped => "stopped",
            timer::State::Finished => "finished",
        });

        let numbers_n_shit =
            container(column![time_text, state].align_items(iced::Alignment::Center))
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
        match &self.timer.state {
            timer::State::Stopped => {
                return Subscription::batch(vec![
                    keyboard::on_key_press(action_handler::handle_menu_keys),
                    keyboard::on_key_release(action_handler::handle_arrow_key_release),
                ]);
            }
            _ => {
                let tick = time::every(time::Duration::from_millis(100)).map(|_| Message::Tick);
                return Subscription::batch(vec![
                    tick,
                    keyboard::on_key_press(action_handler::handle_running_keys),
                ]);
            }
        }
    }

    fn theme(&self) -> Self::Theme {
        Theme::Oxocarbon
    }
}

impl<Message> canvas::Program<Message> for Pomodoro {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let clock = self.clock.draw(renderer, bounds.size(), |frame| {
            let palette = theme.extended_palette();

            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.0;

            let border_clock = Path::circle(center, radius);
            //let inner_clock = Path::circle(center, radius * 0.9);
            frame.fill(&border_clock, palette.background.strong.color);
            //frame.fill(&inner_clock, palette.secondary.weak.color);

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

            if !(matches!(self.timer.state, timer::State::Stopped)) {
                return;
            }

            let sq_point = Point::new(-frame.width() / 2., -frame.height() / 4.);
            //let bd_point = Point::new(sq_point.x * 0.9, sq_point.y * 0.9);
            let sq_size = Size {
                width: frame.width(),
                height: frame.height() / 2.,
            };

            let margin: f32 = 5.;

            let inner_point =
                Point::new(-frame.width() / 2. + margin, -frame.height() / 4. + margin);
            let inner_size = Size {
                width: frame.width() - margin * 2.,
                height: frame.height() / 2. - margin * 2.,
            };

            let popup = Path::rectangle(sq_point, sq_size);
            let inner_popup = Path::rectangle(inner_point, inner_size);

            frame.fill(&popup, palette.background.base.color);
            frame.fill(&inner_popup, palette.secondary.weak.color);

            let value = menu::get_current_value(&self.timer, &self.menu_state);
            let menu_text_value = menu::get_value_name(&self.menu_state);

            let menu_text = canvas::Text {
                content: String::from(menu_text_value),
                position: Point::new(0., -30.),
                horizontal_alignment: alignment::Horizontal::Center,
                ..Default::default()
            };

            let value_text = canvas::Text {
                content: String::from(format!("{}", value)),
                position: Point::new(0., 5.),
                horizontal_alignment: alignment::Horizontal::Center,
                ..Default::default()
            };

            let default_text_color = palette.background.base.color;

            fn arrow_color(colored_keys: &ColoredKeys, arrow: Arrow, def: Color) -> Color {
                match colored_keys.get_state(arrow) {
                    true => iced::Color {
                        r: 255.,
                        g: 0.,
                        b: 0.,
                        a: 1.,
                    },
                    false => def,
                }
            }

            let left_arrow = canvas::Text {
                content: String::from("<"),
                position: Point::new(-100., -30.),
                horizontal_alignment: alignment::Horizontal::Center,
                color: arrow_color(&self.colored_keys, Arrow::Left, default_text_color),
                ..Default::default()
            };

            let right_arrow = canvas::Text {
                content: String::from(">"),
                position: Point::new(100., -30.),
                horizontal_alignment: alignment::Horizontal::Center,
                color: arrow_color(&self.colored_keys, Arrow::Right, default_text_color),
                ..Default::default()
            };

            let upper_arrow = canvas::Text {
                content: String::from("^"),
                position: Point::new(0., 0.),
                horizontal_alignment: alignment::Horizontal::Center,
                color: arrow_color(&self.colored_keys, Arrow::Up, default_text_color),
                ..Default::default()
            };

            let lower_arrow = canvas::Text {
                content: String::from("^"),
                position: Point::new(0., 0.),
                horizontal_alignment: alignment::Horizontal::Center,
                color: arrow_color(&self.colored_keys, Arrow::Down, default_text_color),
                ..Default::default()
            };

            frame.fill_text(menu_text);
            frame.fill_text(value_text);
            frame.fill_text(right_arrow);
            frame.fill_text(left_arrow);

            frame.with_save(|frame| {
                frame.translate(Vector { x: 0., y: -10. });
                frame.fill_text(upper_arrow);
            });

            frame.with_save(|frame| {
                frame.rotate(0.5 * TAU);
                frame.translate(Vector { x: 0., y: -40. });
                frame.fill_text(lower_arrow);
            });
        });

        vec![clock]
    }
}
