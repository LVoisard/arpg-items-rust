use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Paragraph, Widget};
use crate::model::stat::StatBlock;
use crate::ui::ratatui::view_models::stat_block::StatBlockViewModel;

pub struct PlayerStatsWidget<'a> {
    pub stats: &'a StatBlock,
    pub focused: bool,
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

        if self.focused {
            block = block.border_style(Style::default().fg(Color::Cyan)).border_type(BorderType::Double);
        }

        Paragraph::new(lines)
            .block(block)
            .render(area, buf)
    }
}