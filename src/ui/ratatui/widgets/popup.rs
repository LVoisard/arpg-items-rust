use crate::input::input_handler::{InputEvent, InputHandler};
use crate::model::item::{Item, ItemRarity};
use crate::ui::focusable::Focusable;
use crate::ui::ratatui::state::popup::ItemPopupState;
use crate::ui::ratatui::state::ui::UIState;
use crossterm::event::KeyEvent;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Span};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, BorderType, Paragraph, Widget};
use crate::ui::ratatui::view_models::item::ItemViewModel;

pub struct ItemPopupWidget {
    item: ItemViewModel,
}

impl ItemPopupWidget {
    pub fn new(item: ItemViewModel) -> Self {
        Self { item }
    }
}

impl Widget for ItemPopupWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let mut block = Block::bordered();

        block = match self.item.rarity {
            ItemRarity::Magic => block.border_style(Color::Indexed(69)),
            ItemRarity::Rare => block
                .border_type(BorderType::Thick)
                .border_style(Color::Indexed(227)),
            ItemRarity::Unique => block
                .border_type(BorderType::Thick)
                .border_style(Color::Indexed(208)),
            _ => block,
        };

        block = match &self.item.name {
            Some(name) => match self.item.rarity {
                ItemRarity::Unique => {
                    block.title(Line::from(name.clone() + "\n").centered().on_white().bold())
                }
                _ => block.title(Line::from(name.clone() + "\n").centered()),
            },
            _ => block,
        };

        block = match self.item.rarity {
            ItemRarity::Unique => block.title(
                Line::from(self.item.item_base.clone() + "\n")
                    .centered()
                    .on_white()
                    .bold(),
            ),
            _ => block.title(Line::from(self.item.item_base.clone() + "\n").centered()),
        };

        let mut item_description = Vec::<Line>::new();

        if let Some(dmg) = &self.item.damage {
            let start = Span::from("Damage: ");
            let mut d = Span::from(format!("{} - {}", dmg.min, dmg.max));
            if dmg.is_modified {
                d = d.fg(Color::Indexed(69));
            }
            item_description.push(Line::from(vec![start, d]).centered());
        }

        for requirement in self.item.requirements.iter() {
            let mut p1 = Span::from(format!(
                "Required {}: ",
                requirement.requirement.stat_type.to_string()
            ));
            let mut amt = Span::from(requirement.requirement.amount.to_string());
            if !requirement.is_met {
                p1 = p1.red();
                amt = amt.red();
            }

            amt = if requirement.is_modified {
                amt.fg(Color::Indexed(69))
            } else {
                amt
            };

            item_description.push(Line::from(vec![p1, amt]).centered());
        }

        for modifier in self.item.modifiers.iter() {
            item_description.push(
                Line::from(modifier.clone())
                    .fg(Color::Indexed(69))
                    .centered(),
            );
        }

        Paragraph::new(item_description)
            .block(block)
            .render(area, buf);
    }
}
