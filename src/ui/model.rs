//! ## Model
//!
//! Application model

use std::time::Duration;

use tuirealm::event::NoUserEvent;
use tuirealm::ratatui::layout::{Constraint, Direction, Layout};
use tuirealm::terminal::{TerminalAdapter, TerminalBridge};
use tuirealm::{Application, EventListenerCfg, Update};

use crate::ui::components::header::Header;
use crate::ui::components::status_bar::StatusBar;
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

        // Mount the status bar component (visual only)
        assert!(self
            .app
            .mount(Id::StatusBar, Box::new(StatusBar::new()), Vec::default())
            .is_ok());

        // Make the header component active to receive keyboard events
        assert!(self.app.active(&Id::Header).is_ok());
    }

    /// Render the UI
    pub fn view(&mut self) {
        assert!(self
            .terminal
            .draw(|f| {
                // Create the layout
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Length(3), // Header
                            Constraint::Min(1),    // Content (unused for now)
                            Constraint::Length(1), // Status Bar
                        ]
                        .as_ref(),
                    )
                    .split(f.area());

                // Render the header
                self.app.view(&Id::Header, f, chunks[0]);
                // Render the status bar
                self.app.view(&Id::StatusBar, f, chunks[2]);
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
                Msg::None => None,
            }
        } else {
            None
        }
    }
}
