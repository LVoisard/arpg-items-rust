use crate::input::input_handler::{InputEvent, InputHandler};
use crate::ui::focusable::Focusable;
use crate::ui::ratatui::state::player::PlayerState;
use crate::ui::ratatui::state::popup::ItemPopupState;
use crate::ui::ratatui::state::world::WorldState;
use crate::ui::ratatui::view_models::item::ItemViewModel;
use crate::ui::ratatui::widgets::equipment::PlayerEquipmentWidget;
use crate::ui::ratatui::widgets::inventory::PlayerInventoryWidget;
use crate::ui::ratatui::widgets::player_stats::PlayerStatsWidget;
use crate::ui::ratatui::widgets::item_popup::ItemPopupWidget;
use crate::ui::ratatui::widgets::world::WorldWidget;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::prelude::Direction;
use ratatui::text::{Line};
use ratatui::widgets::{Block, Clear};
use ratatui::{DefaultTerminal, Frame};
use std::cmp::PartialEq;
use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(PartialEq, EnumIter, Display)]
enum Screen {
    Stats,
    World,
    Equipment,
    Inventory,
}

pub enum PopupType {
    Item(ItemPopupState),
}

pub struct RatatuiApp {
    exit: bool,
    player_state: PlayerState,
    world_state: WorldState,
    focus: Screen,
    popup: Option<PopupType>,
}

impl RatatuiApp {
    pub fn new(player_state: PlayerState) -> Self {
        Self {
            exit: false,
            player_state,
            world_state: WorldState::new(),
            focus: Screen::Stats,
            popup: None,
        }
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        self.change_screen(Screen::Stats);
        while !self.should_exit() {
            terminal.draw(|frame| self.render(frame))?;
            match crossterm::event::read()? {
                crossterm::event::Event::Key(key) => {
                    self.handle_key_event(key);
                }
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

        let player_stats = PlayerStatsWidget::new(&self.player_state.stats_state);

        let world = WorldWidget::new(&self.world_state);

        let player_equipment = PlayerEquipmentWidget::new(&self.player_state.equipment_state);

        let player_inventory = PlayerInventoryWidget::new(&self.player_state.inventory_state);

        let footer = Block::bordered().title(Line::from("Status").centered());

        frame.render_widget(player_stats, main_layout[0]);
        frame.render_widget(world, main_layout[1]);
        frame.render_widget(player_equipment, inventory_equipment_layout[0]);
        frame.render_widget(player_inventory, inventory_equipment_layout[1]);
        frame.render_widget(footer, root_layout[1]);

        if let Some(popup) = &self.popup {
            let area = popup_area(frame.area(), 40, 20);

            match popup {
                PopupType::Item(state) => {
                    let item = self
                        .player_state
                        .inventory_state
                        .inventory
                        .iter()
                        .nth(state.index)
                        .unwrap();

                    let popup_widget = ItemPopupWidget::new(ItemViewModel::from(
                        item,
                        &self.player_state.stats_state.stats,
                    ));
                    frame.render_widget(Clear, area);
                    frame.render_widget(popup_widget, area);
                }
            };
        }
    }

    fn forward_input(&mut self, key: KeyEvent) -> InputEvent {
        if let Some(popup) = &mut self.popup {
            return match popup {
                PopupType::Item(state) => state.handle_key_event(key),
            };
        }

        let input = match self.focus {
            Screen::Stats => InputEvent::Ignored,
            Screen::World => InputEvent::Ignored,
            Screen::Equipment => self.player_state.equipment_state.handle_key_event(key),
            Screen::Inventory => self.player_state.inventory_state.handle_key_event(key),
        };

        match input {
            InputEvent::Consumed => {}
            InputEvent::Ignored => {}
            InputEvent::Selected(index) => {
                self.on_input_select(index);
            }
        }

        input
    }

    fn change_screen(&mut self, new_screen: Screen) {
        self.get_current_focusable().on_focus_lost();
        self.focus = new_screen;
        self.get_current_focusable().on_focus_gained();
    }

    fn get_current_focusable(&mut self) -> Box<&mut dyn Focusable> {
        match self.focus {
            Screen::Stats => Box::new(&mut self.player_state.stats_state),
            Screen::World => Box::new(&mut self.world_state),
            Screen::Equipment => Box::new(&mut self.player_state.equipment_state),
            Screen::Inventory => Box::new(&mut self.player_state.inventory_state),
        }
    }

    fn on_input_select(&mut self, index: usize) {
        match self.focus {
            Screen::Stats => {}
            Screen::World => {}
            Screen::Equipment => {}
            Screen::Inventory => {
                self.popup = Some(PopupType::Item(ItemPopupState::new(index)));
            }
        }
    }
}

fn popup_area(area: Rect, percent_x: u16, length_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(length_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

impl InputHandler for RatatuiApp {
    fn handle_key_event(&mut self, key: KeyEvent) -> InputEvent {
        let input = if key.kind == KeyEventKind::Press {
            let i = match self.forward_input(key) {
                InputEvent::Consumed => return InputEvent::Consumed,
                _ => InputEvent::Ignored,
            };

            if key.code == KeyCode::Esc {
                if self.popup.is_some() {
                    self.popup = None;
                } else {
                    self.exit = true;
                }
            }

            if self.popup.is_none() {
                match key.code {
                    KeyCode::Char('s') => {
                        self.change_screen(Screen::Stats);
                        InputEvent::Consumed
                    }
                    KeyCode::Char('w') => {
                        self.change_screen(Screen::World);
                        InputEvent::Consumed
                    }
                    KeyCode::Char('i') => {
                        self.change_screen(Screen::Inventory);
                        InputEvent::Consumed
                    }
                    KeyCode::Char('e') => {
                        self.change_screen(Screen::Equipment);
                        InputEvent::Consumed
                    }
                    KeyCode::Tab => {
                        let mut iter = Screen::iter();
                        loop {
                            let item = iter.next();
                            if let Some(i) = item
                                && i == self.focus
                            {
                                let new_screen = iter.next().unwrap_or_else(|| Screen::Stats);
                                self.change_screen(new_screen);
                                break InputEvent::Consumed;
                            }
                        }
                    }
                    _ => InputEvent::Ignored,
                };
            }

            if i == InputEvent::Ignored {
                return match key.code {
                    _ => InputEvent::Consumed,
                };
            };
            i
        } else {
            InputEvent::Ignored
        };
        input
    }
}
