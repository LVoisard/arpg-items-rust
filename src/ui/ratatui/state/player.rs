use crate::{model::{player::Player, stat::StatBlock}, ui::ratatui::state::{equipment::EquipmentState, inventory::InventoryState}};
use crate::ui::ratatui::state::stats::StatState;

pub struct PlayerState {
    pub stats_state: StatState,
    pub inventory_state: InventoryState,
    pub equipment_state: EquipmentState,
}

impl PlayerState {
    fn new(player: Player) -> Self {
        Self {
            stats_state: StatState::from(player.base_stats),
            inventory_state: InventoryState::new(player.inventory),
            equipment_state: EquipmentState::new(),
        }
    }
}

impl From<Player> for PlayerState {
    fn from(player: Player) -> Self {
        PlayerState::new(player)
    }
}