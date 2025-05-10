use std::error::Error;
use std::time::Duration;

use crossterm::event::{self, Event as CEvent, KeyCode, KeyEvent};
use tuirealm::terminal::CrosstermTerminalAdapter;
use tuirealm::PollStrategy;
use tuirealm::Update;

use crate::ui::model::Model;

pub fn run() -> Result<(), Box<dyn Error>> {
    // Create terminal
    let terminal = CrosstermTerminalAdapter::new()?;

    // Setup application
    let mut model = Model::new(terminal);

    // Enter alternate screen
    model.terminal.enter_alternate_screen()?;
    model.terminal.enable_raw_mode()?;

    // Main loop
    while !model.quit {
        // We'll let the component system handle all key events
        // to avoid conflicting with component-level key handling

        // Tick
        match model.app.tick(PollStrategy::Once) {
            Err(err) => {
                eprintln!("Application error: {}", err);
                break;
            }
            Ok(messages) if !messages.is_empty() => {
                // Redraw if at least one message was processed
                model.redraw = true;
                for msg in messages.into_iter() {
                    let mut msg = Some(msg);
                    while msg.is_some() {
                        msg = model.update(msg);
                    }
                }
            }
            _ => {}
        }

        // Redraw
        if model.redraw {
            model.view();
            model.redraw = false;
        }
    }

    // Restore terminal
    model.terminal.leave_alternate_screen()?;
    model.terminal.disable_raw_mode()?;
    model.terminal.clear_screen()?;

    Ok(())
}
