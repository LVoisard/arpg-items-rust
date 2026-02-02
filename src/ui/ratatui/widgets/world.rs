use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Line;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Widget};
use crate::ui::ratatui::state::world::WorldState;

pub struct WorldWidget<'a> {
    world_state: &'a WorldState,
}

impl<'a> WorldWidget<'a> {
    pub fn new(world_state: &'a WorldState) -> Self {
        Self {
            world_state,
        }
    }
}

impl<'a> Widget for WorldWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let mut block = Block::bordered()
            .title(Line::from("World").centered());

        if self.world_state.ui_state.focused {
            block = block.border_type(BorderType::Double).border_style(Style::default().fg(Color::Cyan))
        }

        block.render(area, buf);
    }
}
