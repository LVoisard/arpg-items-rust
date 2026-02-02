use crate::model::item::{ItemRarity};
use crate::ui::ratatui::state::inventory::InventoryState;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Style};
use ratatui::style::{Color, Stylize};
use ratatui::text::Span;
use ratatui::widgets::{Block, BorderType, HighlightSpacing, List, ListItem, StatefulWidget, Widget};

pub struct PlayerInventoryWidget<'a>{
    pub inventory_state: &'a InventoryState,
}

impl<'a> PlayerInventoryWidget<'a> {
    pub fn new(inventory_state: &'a InventoryState) -> Self {
        Self {
            inventory_state
        }
    }
}

impl<'a> Widget for PlayerInventoryWidget<'a>{
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {

        let items: Vec<ListItem> = self
            .inventory_state
            .inventory         
            .iter()
            .map(|item| {
                let n: Span;
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

        let mut block = Block::bordered().title(Line::from("Inventory").centered());

        if self.inventory_state.ui_state.focused {
            block = block.border_style(Style::default().fg(Color::Cyan)).border_type(BorderType::Double);
        }

        let list = List::new(items)
            .block(block)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::WhenSelected);
        
        let mut list_state = self.inventory_state.state;

        StatefulWidget::render(list, area, buf, &mut list_state)
    }
}
