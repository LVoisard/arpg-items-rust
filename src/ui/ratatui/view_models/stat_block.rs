use crate::model::stat::StatBlock;
use crate::ui::ratatui::state::stats::StatState;
use crate::ui::ratatui::view_models::stat::StatViewModel;

pub struct StatBlockViewModel {
    pub stats: Vec<StatViewModel>,
}

impl From<&StatState> for StatBlockViewModel {
    fn from(value: &StatState) -> Self {
        let mut stats = Vec::new();
        for stat in value.stats.stats.iter() {
            stats.push(StatViewModel::from(stat));
        }

        Self { stats }
    }
}