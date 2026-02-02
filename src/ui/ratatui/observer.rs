use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::sync::Arc;
use crate::ui::ratatui::ratatui_app::RatatuiApp;
use crate::ui::ratatui::state::equipment::EquipmentSlot;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum UIEvent {
    InventoryItemSelected(usize),
    EquipmentSlotSelected(EquipmentSlot),

}

pub trait Observer {
    fn on_ui_event(&mut self, event: UIEvent);
}

#[derive(Default)]
pub struct Publisher {
    events: Vec<Arc<dyn Observer>>
}

impl Publisher {
    pub fn new() -> Self {
        Self {
            events: Vec::new()
        }
    }
    pub fn subscribe(&mut self, observer: Arc<dyn Observer>) {
        self.events.push(observer);
    }
    //
    // pub fn unsubscribe(&mut self, observer: Arc<dyn Observer>) {
    //     self.events
    //         .retain(|x| x != &observer)
    // }

    pub fn notify(&mut self, event: UIEvent) {
        for listener in &self.events {
            listener.to_owned().on_ui_event(event.clone())
        }
    }
}