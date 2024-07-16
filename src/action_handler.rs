use crate::keyboard;
use crate::{Arrow, Message};

#[allow(private_interfaces)]
pub fn handle_menu_keys(key: keyboard::Key, _modifiers: keyboard::Modifiers) -> Option<Message> {
    use keyboard::key;

    match key.as_ref() {
        keyboard::Key::Named(key::Named::ArrowRight) => Some(Message::Arrow(Arrow::Right)),
        keyboard::Key::Named(key::Named::ArrowLeft) => Some(Message::Arrow(Arrow::Left)),
        keyboard::Key::Named(key::Named::ArrowDown) => Some(Message::Arrow(Arrow::Down)),
        keyboard::Key::Named(key::Named::ArrowUp) => Some(Message::Arrow(Arrow::Up)),

        keyboard::Key::Named(key::Named::Enter) => Some(Message::Start),
        _ => None,
    }
}

#[allow(private_interfaces)]
pub fn handle_running_keys(key: keyboard::Key, _modifiers: keyboard::Modifiers) -> Option<Message> {
    //use keyboard::key;

    match key.as_ref() {
        keyboard::Key::Character("p") => Some(Message::Pause),
        keyboard::Key::Character("s") => Some(Message::Stop),
        _ => None,
    }
}
