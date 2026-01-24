use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::text::{Line, Span};
use ratatui::{DefaultTerminal, Frame, backend};
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, BorderType, Paragraph, Widget};

use crate::arpg_core::item::{ItemPresentation, ItemRarity};
use crate::view::item_view::ItemView;

pub struct RatatuiApp {
    pub exit: bool,
    pub inventory: Inventory
}

pub struct Inventory {
    pub items: Vec<ItemPresentation>
}

impl RatatuiApp {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.should_exit() {
            terminal.draw(|frame| self.render(frame))?;
            match crossterm::event::read()? {
                crossterm::event::Event::Key(key) => self.handle_key_event(key),
                _ => {},
            }
        }
        Ok(())
    }

    fn should_exit(&self) -> bool {
        self.exit
    } 

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) {
        if key.kind == KeyEventKind::Press && key.code == KeyCode::Esc {
            self.exit = true;
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &RatatuiApp {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {     

        Paragraph::new("Inventory")
            .alignment(Alignment::Center)
            .dark_gray()
            .bold()
            .render(area, buf);

        self.inventory.render(area, buf)
        
    }
}

impl Widget for &Inventory {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized {


        let (title_area, layout) = calculate_layout(area, self.items.iter().len() as u16);
        let mut item_iter = self.items.iter();
        for col in layout {
            for row in col {
                match item_iter.next() {
                    Some(item) => {
                        item.render(row, buf);
                    }
                    _ => {}
                    
                }
            }
        }
    }
}

impl Widget for &ItemPresentation {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized {
        
        let mut block = Block::bordered();

        block = match self.rarity {
            ItemRarity::Magic => block.border_style(Color::Indexed(69)).gray(),
            ItemRarity::Rare => block.border_type(BorderType::Double).border_style(Color::Indexed(227)).gray(),
            ItemRarity::Unique => block.border_type(BorderType::Thick).border_style(Color::Yellow).gray(),
            _ => block
        };

        block = match &self.name {
            Some(name) => match self.rarity {
                ItemRarity::Unique => block.title(Line::from(name.clone() + "\n").centered().on_white()),
                _ => block.title(Line::from(name.clone() + "\n").centered()),
            }
            _ => block
        };

        block = match self.rarity {
                ItemRarity::Unique => block.title(Line::from(self.item_base.clone() + "\n").centered().on_white()),
                _ => block.title(Line::from(self.item_base.clone() + "\n").centered()),
            };

        let mut item_description = Vec::<Line>::new();
        
        for requirement in self.requirements.iter() {
            let mut p1 = Span::from(format!("Required {} : ", requirement.requirement.stat_type.to_string()));
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
            
            item_description.push(Line::from(vec![p1, amt]));
        }

        Paragraph::new(item_description)
            .block(block)    
            .render(area, buf);
            

        

    }
}

/// Calculate the layout of the UI elements.
///
/// Returns a tuple of the title area and the main areas.
fn calculate_layout(area: Rect, n: u16) -> (Rect, Vec<Vec<Rect>>) {
    let main_layout = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]);
    let block_layout = Layout::vertical([Constraint::Max(100); 2]);
    let [title_area, main_area] = main_layout.areas(area);
    let main_areas = block_layout
        .split(main_area)
        .iter()
        .map(|&area| {
            Layout::horizontal([Constraint::Percentage(33), Constraint::Percentage(34),Constraint::Percentage(33)])
                .split(area)
                .to_vec()
        })
        .collect();
    (title_area, main_areas)
}