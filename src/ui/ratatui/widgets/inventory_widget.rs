use crate::arpg_core::item::{ItemPresentation, ItemRarity};
use crate::view::item_view::ItemView;
use crate::view::stat_view::StatsView;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Line;
use ratatui::style::{Color, Stylize};
use ratatui::text::Span;
use ratatui::widgets::{Block, HighlightSpacing, List, ListItem, ListState, Paragraph, StatefulWidget, Widget};
use std::fmt::format;

pub struct Inventory {
    pub items: Vec<ItemPresentation>,
    pub list_state: ListState,
}

pub struct PlayerInventoryWidget<'a>{
    pub items: &'a Inventory,
}

impl<'a> Widget for PlayerInventoryWidget<'a>{
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let mut lines = Vec::<Line>::new();

        let items: Vec<ListItem> = self
            .items
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let mut n: Span;
                if let Some(name) = &item.name {
                    n = Span::from(format!("{} ({})", name, item.item_base))
                } else {
                    n = Span::from(&item.item_base)
                }
                let mut l = Line::from(n);
                l = match item.rarity {
                    ItemRarity::Magic => l.fg(Color::Indexed(69)),
                    ItemRarity::Rare => l.fg(Color::Indexed(227)),
                    ItemRarity::Unique => l.fg(Color::Indexed(208)),
                    _ => l,
                };
                ListItem::new(l)
            })
            .collect();

        let list = List::new(items)
            .block(Block::bordered().title(Line::from("Inventory").centered()))
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::WhenSelected);
        
        let mut list_state = self.items.list_state;

        StatefulWidget::render(list, area, buf, &mut list_state)
    }
}
