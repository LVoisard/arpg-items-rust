use crate::model::stat::StatBlock;
use crate::ui::ratatui::view_models::stat::StatViewModel;

pub struct StatBlockViewModel {
    pub stats: Vec<StatViewModel>,
}

impl From<&StatBlock> for StatBlockViewModel {
    fn from(value: &StatBlock) -> Self {
        let mut stats = Vec::new();
        for stat in value.stats.iter() {
            stats.push(StatViewModel::from(stat));
        }

        Self { stats }
    }
}