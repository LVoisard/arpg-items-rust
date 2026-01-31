use ratatui::{
    layout::{Constraint, Direction, Layout},
    text::Line,
    widgets::{Block, Paragraph, Widget},
};
use ratatui::prelude::{Color, Style};
use ratatui::widgets::BorderType;
use crate::ui::ratatui::state::equipment::{EquipmentSlot, EquipmentState};

pub struct PlayerEquipmentWidget<'a> {
    pub equipment_state: &'a EquipmentState,
    pub focused: bool,
}

impl<'a> Widget for PlayerEquipmentWidget<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let mut main_block = Block::bordered().title(Line::from("Equipment").centered());

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ])
            .split(main_block.inner(area));

        let trinket_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout[1]);

        let body_weapon = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout[2]);

        let gloves_boots = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout[4]);

        for (slot, item) in &self.equipment_state.equipment {
            let slot_name = Line::from(slot.to_string()).centered();
            let item_name = match item {
                Some(x) => match &x.name {
                    Some(name) => name,
                    None => &x.item_base,
                },
                None => "- empty -",
            };
            let slot_contained = Line::from(item_name).centered();

            let item_block = match slot {
                EquipmentSlot::Helmet => layout[0],
                EquipmentSlot::Amulet => trinket_layout[1],
                EquipmentSlot::Ring => trinket_layout[0],
                EquipmentSlot::Weapon => body_weapon[0],
                EquipmentSlot::Armour => body_weapon[1],
                EquipmentSlot::Belt => layout[3],
                EquipmentSlot::Gloves => gloves_boots[0],
                EquipmentSlot::Boots => gloves_boots[1],
            };
            Paragraph::new(vec![slot_name, slot_contained]).render(item_block, buf);
        }

        if self.focused {
            main_block = main_block.border_style(Style::default().fg(Color::Cyan)).border_type(BorderType::Double);
        }

        main_block.render(area, buf);
    }
}
