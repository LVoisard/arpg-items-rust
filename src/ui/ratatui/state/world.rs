use crate::ui::focusable::Focusable;
use crate::ui::ratatui::state::ui::UIState;

pub struct WorldState {
    pub ui_state: UIState,
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            ui_state: UIState {
                focused: false
            }
        }
    }
}

impl Focusable for WorldState {
    fn on_focus_gained(&mut self) {
        self.ui_state.focused = true;
    }

    fn on_focus_lost(&mut self) {
        self.ui_state.focused = false;
    }
}