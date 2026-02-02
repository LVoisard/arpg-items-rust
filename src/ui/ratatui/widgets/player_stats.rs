use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Paragraph, Widget};
use crate::ui::ratatui::state::stats::StatState;
use crate::ui::ratatui::view_models::stat_block::StatBlockViewModel;

pub struct PlayerStatsWidget<'a> {
    pub stats: &'a StatState,
}

impl<'a> PlayerStatsWidget<'a> {
    pub fn new(stats: &'a StatState) -> Self {
        Self {
            stats,
        }
    }
}

impl<'a> Widget for PlayerStatsWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        let view = StatBlockViewModel::from(self.stats);
        let mut lines = Vec::<Line>::new();
        for stat in view.stats.iter() {
            lines.push(Line::from(stat.value.clone()))
        }

        let mut block = Block::bordered().title(Line::from("Stats").centered());

        if self.stats.ui_state.focused {
            block = block.border_type(BorderType::Double).border_style(Style::default().fg(Color::Cyan))
        }

        Paragraph::new(lines)
            .block(block)
            .render(area, buf)
    }
}