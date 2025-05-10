//! ## Model
//!
//! Application model

use std::time::Duration;

use tuirealm::event::NoUserEvent;
use tuirealm::props::{AttrValue, Attribute};
use tuirealm::ratatui::layout::{Constraint, Direction, Layout};
use tuirealm::terminal::{TerminalAdapter, TerminalBridge};
use tuirealm::{Application, EventListenerCfg, Update};

use crate::ui::components::asset_table::AssetTable;
use crate::ui::components::header::Header;
use crate::ui::components::help_bar::HelpBar;
use crate::ui::components::instructions::Instructions;
use crate::ui::components::instructions_bar::InstructionsBar;
use crate::ui::components::summary_bar::SummaryBar;
use crate::ui::id::Id;
use crate::ui::msg::Msg;

/// Application model
pub struct Model<T>
where
    T: TerminalAdapter,
{
    /// Application
    pub app: Application<Id, Msg, NoUserEvent>,
    /// Indicates that the application must quit
    pub quit: bool,
    /// Tells whether to redraw interface
    pub redraw: bool,
    /// Used to draw to terminal
    pub terminal: TerminalBridge<T>,
}

impl<T> Model<T>
where
    T: TerminalAdapter,
{
    /// Create a new model with the given terminal adapter
    pub fn new(terminal_adapter: T) -> Self {
        // Initialize the application with the event listener configuration
        let app = Application::init(
            EventListenerCfg::default()
                .crossterm_input_listener(Duration::from_millis(20), 3)
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_secs(1)),
        );

        let mut model = Self {
            app,
            quit: false,
            redraw: true,
            terminal: TerminalBridge::init(terminal_adapter).expect("Cannot initialize terminal"),
        };

        // Mount components
        model.mount_components();

        model
    }

    /// Mount all components
    fn mount_components(&mut self) {
        // Mount the header component and make it active
        assert!(self
            .app
            .mount(Id::Header, Box::new(Header::new()), Vec::default())
            .is_ok());

        // Mount the instructions bar component (visual only)
        assert!(self
            .app
            .mount(Id::InstructionsBar, Box::new(InstructionsBar::new()), Vec::default())
            .is_ok());

        // Mount the asset table component
        assert!(self
            .app
            .mount(Id::AssetTable, Box::new(AssetTable::new()), Vec::default())
            .is_ok());

        // Mount the dynamic instructions component
        assert!(self
            .app
            .mount(Id::Instructions, Box::new(Instructions::new()), Vec::default())
            .is_ok());

        // Mount the summary bar component
        assert!(self
            .app
            .mount(Id::SummaryBar, Box::new(SummaryBar::new()), Vec::default())
            .is_ok());

        // Mount the help bar component (visual only)
        assert!(self
            .app
            .mount(Id::HelpBar, Box::new(HelpBar::new()), Vec::default())
            .is_ok());

        // Make the asset table active to receive keyboard events
        assert!(self.app.active(&Id::AssetTable).is_ok());
    }

    /// Render the UI
    pub fn view(&mut self) {
        assert!(self
            .terminal
            .draw(|f| {
                // First, split the screen vertically for the header and the rest
                let main_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Length(3), // Header
                            Constraint::Min(1),    // Rest of the UI
                        ]
                        .as_ref(),
                    )
                    .split(f.area());

                // Render the header
                self.app.view(&Id::Header, f, main_chunks[0]);

                // Split the rest horizontally for sidebar and main content
                let body_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(
                        [
                            Constraint::Percentage(30), // Sidebar (asset table)
                            Constraint::Percentage(70), // Main content
                        ]
                        .as_ref(),
                    )
                    .split(main_chunks[1]);

                // Render the asset table as the sidebar
                self.app.view(&Id::AssetTable, f, body_chunks[0]);

                // Split the main content vertically
                let main_content_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Length(1),  // Instructions Bar
                            Constraint::Min(1),     // Main area (instructions)
                            Constraint::Length(1),  // Summary Bar
                            Constraint::Length(1),  // Help Bar
                        ]
                        .as_ref(),
                    )
                    .split(body_chunks[1]);

                // Render the instruction components
                self.app.view(&Id::InstructionsBar, f, main_content_chunks[0]);
                self.app.view(&Id::Instructions, f, main_content_chunks[1]);
                self.app.view(&Id::SummaryBar, f, main_content_chunks[2]);
                self.app.view(&Id::HelpBar, f, main_content_chunks[3]);
            })
            .is_ok());
    }
}

// Implement Update for Model
impl<T> Update<Msg> for Model<T>
where
    T: TerminalAdapter,
{
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        if let Some(msg) = msg {
            // Set redraw flag
            self.redraw = true;

            // Match message
            match msg {
                Msg::AppClose => {
                    self.quit = true;
                    None
                }
                Msg::AssetSelected(index) => {
                    // Asset was highlighted
                    None
                }
                Msg::AssetChosenAsFrom(index, ticker) => {
                    // Asset was selected as FROM asset
                    self.redraw = true;
                    
                    // Update the summary bar with FROM ticker
                    let _ = self.app.attr(
                        &Id::SummaryBar,
                        Attribute::Custom("from_ticker"),
                        AttrValue::String(ticker)
                    );
                    
                    // Update instructions state to select TO asset
                    let _ = self.app.attr(
                        &Id::Instructions,
                        Attribute::Custom("state"),
                        AttrValue::Number(1) // SelectToAsset
                    );
                    
                    None
                }
                Msg::AssetChosenAsTo(index, ticker) => {
                    // Asset was selected as TO asset
                    self.redraw = true;
                    
                    // Update the summary bar with TO ticker
                    let _ = self.app.attr(
                        &Id::SummaryBar,
                        Attribute::Custom("to_ticker"),
                        AttrValue::String(ticker)
                    );
                    
                    // Update instructions state to select FROM amount
                    let _ = self.app.attr(
                        &Id::Instructions, 
                        Attribute::Custom("state"),
                        AttrValue::Number(2) // SelectFromAmount
                    );
                    
                    None
                }
                Msg::EnterFromAssetMode => {
                    // Entering FROM asset selection mode
                    self.redraw = true;
                    
                    // Update the instructions state
                    let _ = self.app.attr(
                        &Id::Instructions,
                        Attribute::Custom("state"),
                        AttrValue::Number(0) // SelectFromAsset
                    );
                    
                    None
                }
                Msg::EnterToAssetMode => {
                    // Entering TO asset selection mode
                    self.redraw = true;
                    
                    // Update the instructions state
                    let _ = self.app.attr(
                        &Id::Instructions,
                        Attribute::Custom("state"),
                        AttrValue::Number(1) // SelectToAsset
                    );
                    
                    None
                }
                Msg::ExitAssetSelectionMode => {
                    // Exiting asset selection mode
                    self.redraw = true;
                    None
                }
                Msg::None => None,
            }
        } else {
            None
        }
    }
}
