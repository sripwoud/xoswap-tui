//! ## Model
//!
//! Application model

use std::time::Duration;

use tuirealm::event::NoUserEvent;
use tuirealm::ratatui::layout::{Constraint, Direction, Layout};
use tuirealm::terminal::{TerminalAdapter, TerminalBridge};
use tuirealm::{Application, EventListenerCfg, Update};

use crate::ui::components::header::Header;
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
        // Mount the header and make it active
        assert!(self
            .app
            .mount(Id::Header, Box::new(Header::default()), Vec::default())
            .is_ok());

        // Make the header active so it can receive keyboard events
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
                            Constraint::Min(0),    // Content (unused for now)
                        ]
                        .as_ref(),
                    )
                    .split(f.area());

                // Render the header
                self.app.view(&Id::Header, f, chunks[0]);
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
