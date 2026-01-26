use crate::{arpg_core::{player::Player, stat::StatBlock}, ui::ratatui::state::{equipment::EquipmentState, inventory::InventoryState}};

pub struct PlayerState {
    pub base_stats: StatBlock,
    pub inventory_state: InventoryState,
    pub equippement_state: EquipmentState,
}

impl PlayerState {
    pub fn new(player: Player) -> Self {
        Self {
            base_stats: player.base_stats,
            inventory_state: InventoryState::new(player.inventory),
            equippement_state: EquipmentState::new(),
        }
    }
}

impl From<Player> for PlayerState {
    fn from(player: Player) -> Self {
        PlayerState::new(player)
    }
}