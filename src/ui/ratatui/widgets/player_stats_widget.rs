use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::{Block, Paragraph, Widget};
use crate::view::stat_view::StatsView;

pub struct PlayerStatsWidget<'a> {
    pub stats: &'a StatsView
}


impl<'a> Widget for PlayerStatsWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        let mut lines = Vec::<Line>::new();
        for stat in self.stats.stats.iter() {
            lines.push(Line::from(stat.value.clone()))
        }

        Paragraph::new(lines)
            .block(Block::bordered().title(Line::from("Stats").centered()))
            .render(area, buf)
    }
}