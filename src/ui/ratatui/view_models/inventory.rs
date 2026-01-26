use ratatui::widgets::ListState;

use crate::arpg_core::item::ItemPresentation;

pub struct InventoryViewModel {
    pub items: Vec<ItemPresentation>,
    pub list_state: ListState,
}