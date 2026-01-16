use crate::view::item_view::ItemView;
use crate::view::stat_view::{PlayerView, StatsView};

pub trait UI {
    fn display_item_view(&self, item_view: &ItemView);
    fn display_stats(&self, stats_view: &StatsView);
    fn display_player_view(&self, player_view: &PlayerView);
}