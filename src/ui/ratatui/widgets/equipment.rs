use ratatui::{
    layout::{Constraint, Direction, Layout},
    text::Line,
    widgets::{Block, Paragraph, Widget, canvas::Label},
};

use crate::{arpg_core::item::{ArmourType, EquipmentType, JewelleryType}, ui::ratatui::state::equipment::{EquipmentSlot, EquipmentState}};

pub struct PlayerEquipmentWidget<'a> {
    pub equipment_state: &'a EquipmentState,
}

impl<'a> Widget for PlayerEquipmentWidget<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let main_block = Block::bordered().title(Line::from("Equipment").centered());

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


        // let slot_name = Line::from("Amulet").centered();
        // let item_name = match &self.equipment_state.amulet_slot {
        //     Some(x) => match &x.name {
        //         Some(name) => name,
        //         None => &x.item_base,
        //     },
        //     None => "- empty -",
        // };
        // let slot_contained = Line::from("[ - empty - ]").centered();

        // Paragraph::new(vec![slot_name, slot_contained]).render(trinket_layout[0], buf);

        // let slot_name = Line::from("Ring").centered();
        // let slot_contained = Line::from("- empty -").centered();

        // Paragraph::new(vec![slot_name, slot_contained]).render(trinket_layout[1], buf);

        // let slot_name = Line::from("Armour").centered();
        // let slot_contained = Line::from("- empty -").centered();

        // Paragraph::new(vec![slot_name, slot_contained]).render(layout[2], buf);

        // let slot_name = Line::from("Belt").centered();
        // let slot_contained = Line::from("- empty -").centered();

        // Paragraph::new(vec![slot_name, slot_contained]).render(layout[3], buf);

        // let slot_name = Line::from("Gloves").centered();
        // let slot_contained = Line::from("- empty -").centered();

        // Paragraph::new(vec![slot_name, slot_contained]).render(gloves_boots[0], buf);

        // let slot_name = Line::from("Boots").centered();
        // let slot_contained = Line::from("- empty -").centered();

        // Paragraph::new(vec![slot_name, slot_contained]).render(gloves_boots[1], buf);

        main_block.render(area, buf);
    }
}
