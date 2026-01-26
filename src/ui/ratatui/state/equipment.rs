use std::{collections::HashMap, fmt::Display};

use ratatui::widgets::ListState;

use crate::arpg_core::item::Item;


pub struct EquipmentState {
    pub equipment: HashMap<EquipmentSlot, Option<Item>>
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
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum EquipmentSlot {
    Helmet,
    Amulet,
    Ring,
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
