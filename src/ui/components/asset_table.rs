//! ## AssetTable
//! 
//! Asset table component for displaying asset prices

use std::fmt;

use tuirealm::command::{Cmd, CmdResult};
use tuirealm::event::{Key, KeyEvent, KeyModifiers, NoUserEvent};
use tuirealm::props::{Color, Style, TextModifiers};
use tuirealm::ratatui::layout::Rect;
use tuirealm::ratatui::widgets::{Block, BorderType as RBorderType, Cell, Row, Table, TableState};
use tuirealm::{AttrValue, Attribute, Component, Event, Frame, MockComponent, Props, State, StateValue};

use crate::ui::msg::Msg;

/// Selection mode for the asset table
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectionMode {
    Normal,    // Regular navigation
    FromAsset, // Selecting FROM asset
    ToAsset,   // Selecting TO asset
}

/// Asset data structure
#[derive(Clone, Debug)]
pub struct Asset {
    pub name: String,
    pub price: String,
}

impl fmt::Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.price)
    }
}

/// Asset table component for displaying and selecting assets
pub struct AssetTable {
    props: Props,
    assets: Vec<Asset>,
    current_index: usize,       // Currently highlighted row
    from_asset_index: Option<usize>, // FROM asset (red)
    to_asset_index: Option<usize>,   // TO asset (green)
    mode: SelectionMode,        // Current selection mode
}

impl Default for AssetTable {
    fn default() -> Self {
        Self {
            props: Props::default(),
            assets: vec![
                Asset { name: "BTC".to_string(), price: "$100,000".to_string() },
                Asset { name: "ETH".to_string(), price: "$2,400".to_string() },
                Asset { name: "SOL".to_string(), price: "$145".to_string() },
            ],
            current_index: 0,
            from_asset_index: None,
            to_asset_index: None,
            mode: SelectionMode::FromAsset, // Start in FROM selection mode
        }
    }
}

impl AssetTable {
    /// Create a new asset table
    pub fn new() -> Self {
        Self::default()
    }

    /// Move to the next asset
    /// Next asset to choose after selecting
    fn next_asset(&mut self) {
        self.current_index = (self.current_index + 1) % self.assets.len();
        // Skip assets that are already selected in a different role
        if (Some(self.current_index) == self.from_asset_index && self.mode == SelectionMode::ToAsset) 
           || (Some(self.current_index) == self.to_asset_index && self.mode == SelectionMode::FromAsset) {
            self.next_asset();
        }
    }

    /// Move to the previous asset
    fn prev_asset(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
        } else {
            self.current_index = self.assets.len() - 1;
        }
        // Skip assets that are already selected in a different role
        if (Some(self.current_index) == self.from_asset_index && self.mode == SelectionMode::ToAsset) 
           || (Some(self.current_index) == self.to_asset_index && self.mode == SelectionMode::FromAsset) {
            self.prev_asset();
        }
    }

    /// Set the current asset as FROM asset
    fn select_as_from_asset(&mut self) {
        // Only set FROM if it's not already the TO asset
        if Some(self.current_index) != self.to_asset_index {
            self.from_asset_index = Some(self.current_index);
            
            // Automatically switch to TO asset mode if TO hasn't been selected yet
            if self.to_asset_index.is_none() {
                self.enter_to_mode();
            }
        }
    }

    /// Set the current asset as TO asset
    fn select_as_to_asset(&mut self) {
        // Only set TO if it's not already the FROM asset
        if Some(self.current_index) != self.from_asset_index {
            self.to_asset_index = Some(self.current_index);
            
            // After selecting TO asset, we could handle switching to amount mode here
            // (we'll implement this later)
            self.exit_selection_mode();
        }
    }

    /// Switch to FROM selection mode
    fn enter_from_mode(&mut self) {
        self.mode = SelectionMode::FromAsset;
        // If we have a FROM asset, navigate to it
        if let Some(idx) = self.from_asset_index {
            self.current_index = idx;
        }
    }

    /// Switch to TO selection mode
    fn enter_to_mode(&mut self) {
        self.mode = SelectionMode::ToAsset;
        // If we have a TO asset, navigate to it
        if let Some(idx) = self.to_asset_index {
            self.current_index = idx;
        }
    }

    /// Exit selection mode back to normal
    fn exit_selection_mode(&mut self) {
        self.mode = SelectionMode::Normal;
    }
}

impl MockComponent for AssetTable {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        // Check if visible
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            // Create table rows
            let rows: Vec<Row> = self.assets
                .iter()
                .enumerate()
                .map(|(i, asset)| {
                    let style = if Some(i) == self.from_asset_index {
                        // FROM asset - light red background
                        if i == self.current_index && self.mode == SelectionMode::FromAsset {
                            // Currently highlighted FROM asset
                            Style::default().bg(Color::Rgb(255, 180, 180)).fg(Color::Black)
                        } else {
                            Style::default().bg(Color::Rgb(255, 200, 200))
                        }
                    } else if Some(i) == self.to_asset_index {
                        // TO asset - light green background
                        if i == self.current_index && self.mode == SelectionMode::ToAsset {
                            // Currently highlighted TO asset
                            Style::default().bg(Color::Rgb(180, 255, 180)).fg(Color::Black)
                        } else {
                            Style::default().bg(Color::Rgb(200, 255, 200))
                        }
                    } else if i == self.current_index {
                        // Highlighted row (not selected) - light yellow
                        Style::default().bg(Color::Rgb(255, 255, 220)).fg(Color::Black)
                    } else {
                        // Normal row
                        Style::default()
                    };
                    
                    Row::new(vec![
                        Cell::from(asset.name.clone()),
                        Cell::from(asset.price.clone()),
                    ])
                    .style(style)
                })
                .collect();

            // Create header row
            let header_cells = ["Asset", "Price"]
                .iter()
                .map(|h| Cell::from(*h).style(
                    Style::default()
                        .fg(Color::White)
                        .bg(Color::DarkGray)
                        .add_modifier(TextModifiers::BOLD)
                ));
            
            let header = Row::new(header_cells)
                .style(Style::default().bg(Color::DarkGray))
                .height(1);

            // Create bordered block
            let focus = self.props
                .get_or(Attribute::Focus, AttrValue::Flag(false))
                .unwrap_flag();
            
            let block_title = match self.mode {
                SelectionMode::Normal => "Assets",
                SelectionMode::FromAsset => "Select FROM Asset",
                SelectionMode::ToAsset => "Select TO Asset",
            };
            
            let border_color = match self.mode {
                SelectionMode::Normal => Color::White,
                SelectionMode::FromAsset => Color::LightRed,
                SelectionMode::ToAsset => Color::LightGreen,
            };
            
            let border_style = if focus {
                Style::default().fg(border_color)
            } else {
                Style::default().fg(Color::Gray)
            };
            
            let block = Block::default()
                .borders(tuirealm::ratatui::widgets::Borders::ALL)
                .border_type(RBorderType::Rounded)
                .border_style(border_style)
                .title(block_title);

            // Create table with widths
            let widths = [
                tuirealm::ratatui::layout::Constraint::Percentage(50),
                tuirealm::ratatui::layout::Constraint::Percentage(50),
            ];
            
            let table = Table::new(rows, widths)
                .header(header)
                .block(block)
                .row_highlight_style(Style::default().add_modifier(TextModifiers::BOLD));

            // Create a mutable table state to track selection
            let mut state = TableState::default();
            state.select(Some(self.current_index));

            // Render the table with selection
            frame.render_stateful_widget(table, area, &mut state);
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        match attr {
            Attribute::Value => {
                // Return currently highlighted asset name
                if let Some(asset) = self.assets.get(self.current_index) {
                    Some(AttrValue::String(asset.name.clone()))
                } else {
                    None
                }
            },
            _ => self.props.get(attr),
        }
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        // Return current asset name
        if let Some(asset) = self.assets.get(self.current_index) {
            State::One(StateValue::String(asset.name.clone()))
        } else {
            State::None
        }
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Submit => {
                // Handle submission based on current mode
                match self.mode {
                    SelectionMode::Normal => {
                        self.enter_from_mode();
                        CmdResult::Changed(self.state())
                    },
                    SelectionMode::FromAsset => {
                        self.select_as_from_asset();
                        CmdResult::Changed(self.state())
                    },
                    SelectionMode::ToAsset => {
                        self.select_as_to_asset();
                        CmdResult::Changed(self.state())
                    },
                }
            },
            _ => CmdResult::None,
        }
    }
}

impl Component<Msg, NoUserEvent> for AssetTable {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Char('f'),
                modifiers: KeyModifiers::NONE,
            }) => {
                // Always switch to FROM mode on 'f'
                self.enter_from_mode();
                Some(Msg::EnterFromAssetMode)
            },
            Event::Keyboard(KeyEvent {
                code: Key::Char('t'),
                modifiers: KeyModifiers::NONE,
            }) => {
                // Always switch to TO mode on 't'
                self.enter_to_mode();
                Some(Msg::EnterToAssetMode)
            },
            Event::Keyboard(KeyEvent {
                code: Key::Down,
                modifiers: KeyModifiers::NONE,
            }) | Event::Keyboard(KeyEvent {
                code: Key::Char('j'),
                modifiers: KeyModifiers::NONE,
            }) => {
                self.next_asset();
                Some(Msg::AssetSelected(self.current_index))
            },
            Event::Keyboard(KeyEvent {
                code: Key::Up,
                modifiers: KeyModifiers::NONE,
            }) | Event::Keyboard(KeyEvent {
                code: Key::Char('k'),
                modifiers: KeyModifiers::NONE,
            }) => {
                self.prev_asset();
                Some(Msg::AssetSelected(self.current_index))
            },
            Event::Keyboard(KeyEvent {
                code: Key::Enter,
                modifiers: KeyModifiers::NONE,
            }) => {
                match self.mode {
                    SelectionMode::Normal | SelectionMode::FromAsset => {
                        self.select_as_from_asset();
                        Some(Msg::AssetChosenAsFrom(self.current_index))
                    },
                    SelectionMode::ToAsset => {
                        self.select_as_to_asset();
                        Some(Msg::AssetChosenAsTo(self.current_index))
                    },
                }
            },
            Event::Keyboard(KeyEvent {
                code: Key::Tab,
                modifiers: KeyModifiers::NONE,
            }) => {
                // Tab always selects TO asset
                self.select_as_to_asset();
                Some(Msg::AssetChosenAsTo(self.current_index))
            },
            Event::Keyboard(KeyEvent {
                code: Key::Esc,
                modifiers: KeyModifiers::NONE,
            }) => {
                // In selection mode, Esc returns to normal mode
                if self.mode != SelectionMode::Normal {
                    self.exit_selection_mode();
                    Some(Msg::ExitAssetSelectionMode)
                } else {
                    // In normal mode, Esc quits
                    Some(Msg::AppClose)
                }
            },
            Event::Keyboard(KeyEvent {
                code: Key::Char('q'),
                modifiers: KeyModifiers::NONE,
            }) => {
                // 'q' always quits the application
                Some(Msg::AppClose)
            },
            _ => None,
        }
    }
}