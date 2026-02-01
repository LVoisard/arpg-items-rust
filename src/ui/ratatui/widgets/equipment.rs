use ratatui::{
    layout::{Constraint, Direction, Layout},
    text::Line,
    widgets::{Block, Paragraph, Widget},
};
use ratatui::prelude::{Color, Span, Style};
use ratatui::widgets::{BorderType};
use crate::ui::ratatui::state::equipment::{EquipmentSlot, EquipmentState};

pub struct PlayerEquipmentWidget<'a> {
    equipment_state: &'a EquipmentState,
}

impl<'a> PlayerEquipmentWidget<'a> {
    pub fn new(equipment_state: &'a EquipmentState) -> Self {
        Self {
            equipment_state
        }
    }
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
            let mut slot_label = vec![Span::from(slot.to_string())];
            if let Some(selected_slot) = &self.equipment_state.selected
                && slot == selected_slot {
                slot_label.insert(0, Span::from("> "));
                slot_label.push(Span::from(" <"));
            }
            let slot_name = Line::from(slot_label).centered();

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

        if self.equipment_state.ui_state.focused {
            main_block = main_block.border_style(Style::default().fg(Color::Cyan)).border_type(BorderType::Double);
        }

        main_block.render(area, buf);
    }
}
