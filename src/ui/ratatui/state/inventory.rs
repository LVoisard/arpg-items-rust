use crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::ListState;
use crate::input::input_handler::InputHandler;
use crate::model::inventory::{Inventory};

pub struct InventoryState {
    pub inventory: Inventory,
    pub state: ListState,
}

impl InventoryState {
    pub fn new(inventory: Inventory) -> Self {
        Self {
            inventory,
            state: ListState::default(),
        }
    }
}

impl InventoryState {
    fn select_next_item(&mut self) {
        if let Some(index) = self.state.selected() {
            if index > self.inventory.iter().count() - 1 {
                self.state.select_first();
            }
            self.state.select_next()

        } else {
            self.state.select_first();
        }
    }
}

impl InputHandler for InventoryState {
    fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up => {},
            KeyCode::Down => self.select_next_item(),
            _ => {}
        }
    }
}