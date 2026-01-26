use crate::arpg_core::item::Item;
use crate::arpg_core::player::Player;
use crate::arpg_core::stat::{Stat, StatBlock, StatType};
use crate::view::text_view::{TextStyle, TextView};

impl From<Stat> for TextView {
    fn from(value: Stat) -> Self {
        Self {
            value: format!("{}: {}", value.stat_type, value.value),
            style: TextStyle::Normal,
        }
    }
}

pub struct StatsView {
    pub stats: Vec<TextView>,
}
impl From<StatBlock> for StatsView {
    fn from(value: StatBlock) -> Self {
        Self {
            stats: value.stats.into_iter().map(TextView::from).collect()
        }
    }
}

impl From<&StatBlock> for StatsView {
    fn from(value: &StatBlock) -> Self {
        Self {
            stats: value.stats.iter().map(|s| TextView::from(*s)).collect()
        }
    }
}

pub struct PlayerView {
    pub stats: StatsView,
    pub equipped_items: Vec<TextView>,
}

impl From<Player> for PlayerView {
    fn from(value: Player) -> Self {
        Self {
            stats: StatsView::from(value.get_derived_stats()),
            equipped_items: value.equippement.iter().map(|x| TextView {
                value: x.name.clone().unwrap_or(x.item_base.clone()),
                style: TextStyle::from_rarity(x.rarity),
            }).collect()
        }
    }
}

