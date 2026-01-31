use crate::input::input_handler::InputHandler;
use crate::model::item::ItemRarity;
use crate::ui::ratatui::state::player::PlayerState;
use crate::ui::ratatui::view_models::item::ItemViewModel;
use crate::ui::ratatui::widgets::equipment::PlayerEquipmentWidget;
use crate::ui::ratatui::widgets::inventory::PlayerInventoryWidget;
use crate::ui::ratatui::widgets::player_stats::PlayerStatsWidget;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Direction;
use ratatui::style::{Color, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Paragraph, Widget};
use ratatui::{DefaultTerminal, Frame};
use strum::{Display, EnumIter, IntoEnumIterator};

pub struct RatatuiApp {
    exit: bool,
    player_state: PlayerState,
    focus: Screen,
}

#[derive(PartialEq, EnumIter, Display)]
enum Screen {
    Stats,
    World,
    Equipment,
    Inventory,
}

impl RatatuiApp {
    pub fn new(player_state: PlayerState) -> Self {
        Self {
            exit: false,
            player_state,
            focus: Screen::Stats,
        }
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.should_exit() {
            terminal.draw(|frame| self.render(frame))?;
            match crossterm::event::read()? {
                crossterm::event::Event::Key(key) => self.handle_key_event(key),
                _ => {}
            }
        }
        Ok(())
    }

    fn should_exit(&self) -> bool {
        self.exit
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let root_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(5)])
            .split(frame.area());

        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(25),
                Constraint::Min(0),
                Constraint::Length(30),
            ])
            .split(root_layout[0]);

        let inventory_equipment_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(3), Constraint::Min(7)])
            .split(main_layout[2]);

        let player_stats = PlayerStatsWidget {
            stats: &self.player_state.base_stats,
            focused: self.focus == Screen::Stats,
        };

        let b2 = Block::bordered().title(Line::from("World").centered());

        let player_equipment = PlayerEquipmentWidget {
            equipment_state: &self.player_state.equippement_state,
            focused: self.focus == Screen::Equipment,
        };

        let player_inventory = PlayerInventoryWidget {
            inventory_state: &self.player_state.inventory_state,
            focused: self.focus == Screen::Inventory,
        };

        let footer = Block::bordered().title(Line::from("Status").centered());

        frame.render_widget(player_stats, main_layout[0]);
        frame.render_widget(b2, main_layout[1]);
        frame.render_widget(player_equipment, inventory_equipment_layout[0]);
        frame.render_widget(player_inventory, inventory_equipment_layout[1]);

        frame.render_widget(footer, root_layout[1]);

        //frame.render_widget(self, frame.area());
    }
}

impl InputHandler for RatatuiApp {
    fn handle_key_event(&mut self, key: KeyEvent) {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Esc => self.exit = true,
                KeyCode::Char('s') => self.focus = Screen::Stats,
                KeyCode::Char('w') => self.focus = Screen::World,
                KeyCode::Char('i') => self.focus = Screen::Inventory,
                KeyCode::Char('e') => self.focus = Screen::Equipment,
                KeyCode::Tab => {
                    let mut iter = Screen::iter();
                    while let item = iter.next() {
                        if let Some(i) = item && i == self.focus {
                            self.focus = iter.next().unwrap_or_else(|| Screen::Stats);
                            break;
                        }
                    }
                }
                _ => match self.focus {
                    Screen::Stats => {}
                    Screen::World => {}
                    Screen::Inventory => self.player_state.inventory_state.handle_key_event(key),
                    Screen::Equipment => self.player_state.equippement_state.handle_key_event(key),
                },
            }
        }
    }
}

impl Widget for &ItemViewModel {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let mut block = Block::bordered().white();

        block = match self.rarity {
            ItemRarity::Magic => block.border_style(Color::Indexed(69)),
            ItemRarity::Rare => block
                .border_type(BorderType::Thick)
                .border_style(Color::Indexed(227)),
            ItemRarity::Unique => block
                .border_type(BorderType::Thick)
                .border_style(Color::Indexed(208)),
            _ => block,
        };

        block = match &self.name {
            Some(name) => match self.rarity {
                ItemRarity::Unique => {
                    block.title(Line::from(name.clone() + "\n").centered().on_white().bold())
                }
                _ => block.title(Line::from(name.clone() + "\n").centered()),
            },
            _ => block,
        };

        block = match self.rarity {
            ItemRarity::Unique => block.title(
                Line::from(self.item_base.clone() + "\n")
                    .centered()
                    .on_white()
                    .bold(),
            ),
            _ => block.title(Line::from(self.item_base.clone() + "\n").centered()),
        };

        let mut item_description = Vec::<Line>::new();

        if let Some(dmg) = &self.damage {
            let start = Span::from("Damage: ");
            let mut d = Span::from(format!("{} - {}", dmg.min, dmg.max));
            if dmg.is_modified {
                d = d.fg(Color::Indexed(69));
            }
            item_description.push(Line::from(vec![start, d]).centered());
        }

        for requirement in self.requirements.iter() {
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

        for modifier in self.modifiers.iter() {
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
