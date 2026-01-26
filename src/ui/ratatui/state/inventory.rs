use ratatui::widgets::ListState;

use crate::arpg_core::inventory::{Inventory};

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