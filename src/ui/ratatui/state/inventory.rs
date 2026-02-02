use std::sync::Arc;
use crate::input::input_handler::InputHandler;
use crate::model::inventory::Inventory;
use crate::ui::focusable::Focusable;
use crate::ui::ratatui::observer::{UIEvent, Publisher, Observer};
use crate::ui::ratatui::state::ui::UIState;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::ListState;

pub struct InventoryState {
    pub inventory: Inventory,
    pub state: ListState,
    pub ui_state: UIState,
    pub publisher: Publisher,
}

impl InventoryState {
    pub fn new(inventory: Inventory) -> Self {
        Self {
            inventory,
            state: ListState::default(),
            ui_state: UIState { focused: false },
            publisher: Publisher::new(),
        }
    }
}

impl InventoryState {
    pub fn add_on_item_selected_listener(&mut self, listener: Arc<dyn Observer>) {
        self.publisher
            .subscribe(listener)
    }

    fn select_next_item(&mut self) {
        if self.select_if_none() {
            return;
        }
        if self.state.selected().unwrap() >= self.inventory.iter().len() - 1 {
            self.state.select(Some(0))
        } else {
            self.state.select_next()
        };
    }

    fn select_previous_item(&mut self) {
        if self.select_if_none() {
            return;
        }
        if self.state.selected().unwrap() <= 0 {
            self.state.select(Some(self.inventory.iter().len() - 1))
        } else {
            self.state.select_previous()
        };
    }

    fn select_if_none(&mut self) -> bool {
        if self.state.selected().is_some() {
            false
        } else {
            self.state.select_first();
            true
        }
    }

    fn remove_selected(&mut self) {
        self.state.select(None)
    }
}

impl InputHandler for InventoryState {
    fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up => self.select_previous_item(),
            KeyCode::Down => self.select_next_item(),
            KeyCode::Enter => {
                if let Some(index) = self.state.selected() {
                    self.publisher.notify(UIEvent::InventoryItemSelected(index))
                }
            }
            _ => {}
        }
    }
}

impl Focusable for InventoryState {
    fn on_focus_gained(&mut self) {
        self.ui_state.focused = true;
        self.select_if_none();
    }
    fn on_focus_lost(&mut self) {
        self.ui_state.focused = false;
        self.remove_selected()
    }
}
