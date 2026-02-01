use crate::model::stat::StatBlock;
use crate::ui::focusable::Focusable;
use crate::ui::ratatui::state::ui::UIState;

pub struct StatState {
    pub stats: StatBlock,
    pub ui_state: UIState
}

impl StatState {
    fn new(stats: StatBlock) -> Self {
        Self {
            stats,
            ui_state: UIState {
                focused: false,
            }
        }
    }
}

impl From<StatBlock> for StatState {
    fn from(value: StatBlock) -> Self {
        Self::new(value)
    }
}

impl Focusable for StatState {
    fn on_focus_gained(&mut self) {
        self.ui_state.focused = true;
    }

    fn on_focus_lost(&mut self) {
        self.ui_state.focused = false;
    }
}