use crate::input::input_handler::{InputEvent, InputHandler};
use crate::ui::focusable::Focusable;
use crate::ui::ratatui::state::ui::UIState;
use crossterm::event::KeyEvent;


pub struct ItemPopupState {
    ui_state: UIState,
    pub index: usize,
}

impl ItemPopupState {
    pub fn new(index: usize) -> Self {
        Self {
            ui_state: UIState {
                focused: false
            },
            index
        }
    }
}

impl InputHandler for ItemPopupState {
    fn handle_key_event(&mut self, key: KeyEvent) -> InputEvent {
        match key.code {
            _ => InputEvent::Ignored,
        }
    }
}

impl Focusable for ItemPopupState {
    fn on_focus_gained(&mut self) {
        self.ui_state.focused = true;
    }

    fn on_focus_lost(&mut self) {
        self.ui_state.focused = false;
    }
}
