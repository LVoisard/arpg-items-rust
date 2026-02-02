pub trait InputHandler {
    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> InputEvent;
}

pub enum InputEvent {
    Consumed,
    Selected(usize),
    Ignored,
}

impl PartialEq for InputEvent {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (InputEvent::Consumed, InputEvent::Consumed) => true,
            (InputEvent::Selected(_), InputEvent::Selected(_)) => true,
            (InputEvent::Ignored, InputEvent::Ignored) => true,
            _ => false,
        }
    }
}
