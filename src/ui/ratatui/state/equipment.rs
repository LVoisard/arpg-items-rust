use crate::input::input_handler::{InputEvent, InputHandler};
use crate::model::item::Item;
use crossterm::event::{KeyCode, KeyEvent};
use std::{collections::HashMap, fmt::Display};
use strum::{EnumIter, IntoEnumIterator};
use crate::ui::focusable::Focusable;
use crate::ui::ratatui::state::ui::UIState;

pub struct EquipmentState {
    pub equipment: HashMap<EquipmentSlot, Option<Item>>,
    pub selected: Option<EquipmentSlot>,
    pub ui_state: UIState,
}

impl EquipmentState {
    pub fn new() -> Self {
        Self {
            equipment: HashMap::from([
                (EquipmentSlot::Helmet, None),
                (EquipmentSlot::Amulet, None),
                (EquipmentSlot::Ring, None),
                (EquipmentSlot::Weapon, None),
                (EquipmentSlot::Armour, None),
                (EquipmentSlot::Belt, None),
                (EquipmentSlot::Gloves, None),
                (EquipmentSlot::Boots, None),
            ]),
            selected: None,
            ui_state: UIState {
                focused: false
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, EnumIter)]
pub enum EquipmentSlot {
    Helmet,
    Ring,
    Amulet,
    Weapon,
    Armour,
    Belt,
    Gloves,
    Boots,
}

impl Display for EquipmentSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EquipmentSlot::Helmet => write!(f, "Helmet"),
            EquipmentSlot::Amulet => write!(f, "Amulet"),
            EquipmentSlot::Ring => write!(f, "Ring"),
            EquipmentSlot::Weapon => write!(f, "Weapon"),
            EquipmentSlot::Armour => write!(f, "Armour"),
            EquipmentSlot::Belt => write!(f, "Belt"),
            EquipmentSlot::Gloves => write!(f, "Gloves"),
            EquipmentSlot::Boots => write!(f, "Boots"),
        }
    }
}

impl EquipmentState {

    fn select_next_equipment_slot(&mut self) {
        if let Some(slot) = &self.selected {
            let index = EquipmentSlot::iter().position(|s| s == *slot).unwrap();
            let index = if index == EquipmentSlot::iter().len() - 1 {
                0
            } else {
                index + 1
            };
            self.selected = Some(EquipmentSlot::iter().nth(index).unwrap())
        }
    }

    fn select_previous_equipment_slot(&mut self) {
        if let Some(slot) = &self.selected {
            let index = EquipmentSlot::iter().position(|s| s == *slot).unwrap();
            let index = if index == 0 {
                EquipmentSlot::iter().len() - 1
            } else {
                index - 1
            };
            self.selected = Some(EquipmentSlot::iter().nth(index).unwrap())
        }
    }

    fn select_if_none(&mut self) -> bool {
        if self.selected.is_none() {
            self.selected = Some(EquipmentSlot::Helmet);
            return true;
        }
        false
    }

    fn remove_selection(&mut self) {
        self.selected = None;
    }

}

impl InputHandler for EquipmentState {
    fn handle_key_event(&mut self, key: KeyEvent) -> InputEvent {
        match key.code {
            KeyCode::Up => {self.select_previous_equipment_slot(); InputEvent::Consumed},
            KeyCode::Down => {self.select_next_equipment_slot(); InputEvent::Consumed},
            _ => { InputEvent::Ignored }
        }
    }
}

impl Focusable for EquipmentState {
    fn on_focus_gained(&mut self) {
        self.ui_state.focused = true;
        self.select_if_none();
    }

    fn on_focus_lost(&mut self) {
        self.ui_state.focused = false;
        self.remove_selection();
    }
}

