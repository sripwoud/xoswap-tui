use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{error::Error, io};

use crate::models::{App, InputMode, MOCK_ASSETS};
use crate::services::fetch_quote;
use crate::ui::ui;

/// Starts the application and runs the main loop
pub fn run() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Create app state
    let mut app = App::default();

    // Run the app
    let result = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    terminal.clear()?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        println!("Error: {}", err);
    }

    Ok(())
}

/// Run the main application loop
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), Box<dyn Error>> 
where
    B: Backend,
{
    loop {
        terminal.draw(|f| ui::<B>(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') && !app.show_qr && matches!(app.input_mode, InputMode::Normal) {
                return Ok(());
            } else if app.show_qr && key.code == KeyCode::Char('q') {
                app.show_qr = false;
                continue;
            }

            match app.input_mode {
                InputMode::Normal => handle_normal_mode(app, key),
                InputMode::SelectingFrom => handle_select_from_asset(app, key),
                InputMode::SelectingTo => handle_select_to_asset(app, key),
                InputMode::EnteringAddress => handle_address_input(app, key),
                InputMode::EnteringAmount => handle_amount_input(app, key),
            }
        }
    }
}

/// Handle input in normal mode
fn handle_normal_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('f') => {
            app.input_mode = InputMode::SelectingFrom;
        }
        KeyCode::Char('t') => {
            app.input_mode = InputMode::SelectingTo;
        }
        KeyCode::Char('a') => {
            app.input_mode = InputMode::EnteringAddress;
        }
        KeyCode::Char('m') => {
            app.input_mode = InputMode::EnteringAmount;
        }
        KeyCode::Char('s') => {
            // Attempt to fetch a quote
            if app.from_asset.is_none() || app.to_asset.is_none() {
                app.message = "Error: Select from and to assets first".to_string();
                return;
            }

            if app.amount.is_empty() {
                app.message = "Error: Enter an amount first".to_string();
                return;
            }

            let amount = match app.amount.parse::<f64>() {
                Ok(amount) => amount,
                Err(_) => {
                    app.message = "Error: Invalid amount".to_string();
                    return;
                }
            };

            if app.selected_provider.is_none() {
                app.message = "Select a provider first".to_string();
                return;
            }

            match fetch_quote(
                app.from_asset.as_ref().unwrap(),
                app.to_asset.as_ref().unwrap(),
                amount,
                &app.providers,
                app.selected_provider.unwrap(),
            ) {
                Ok(quotes) => {
                    app.quotes = quotes;
                    app.message = "Quote fetched successfully".to_string();
                }
                Err(e) => {
                    app.message = format!("Error: {}", e);
                }
            }
        }
        KeyCode::Char('p') => {
            // Navigate through providers
            if app.selected_provider.is_none() {
                app.selected_provider = Some(0);
            } else {
                let next = (app.selected_provider.unwrap() + 1) % app.providers.len();
                app.selected_provider = Some(next);
            }
        }
        KeyCode::Char('g') => {
            // Generate QR code
            if app.address.is_empty() {
                app.message = "Error: Enter an address first".to_string();
                return;
            }

            app.show_qr = true;
            app.message = "Showing QR code for address".to_string();
        }
        _ => {}
    }
}

/// Handle input when selecting from asset
fn handle_select_from_asset(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Down => {
            let selected = match app.from_asset_table_state.selected() {
                Some(i) => {
                    if i >= MOCK_ASSETS.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            app.from_asset_table_state.select(Some(selected));
        }
        KeyCode::Up => {
            let selected = match app.from_asset_table_state.selected() {
                Some(i) => {
                    if i == 0 {
                        MOCK_ASSETS.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            app.from_asset_table_state.select(Some(selected));
        }
        KeyCode::Enter => {
            if let Some(i) = app.from_asset_table_state.selected() {
                app.from_asset = Some(MOCK_ASSETS[i].to_string());
                app.message = format!("Selected from asset: {}", MOCK_ASSETS[i]);
                app.input_mode = InputMode::Normal;
            }
        }
        _ => {}
    }
}

/// Handle input when selecting to asset
fn handle_select_to_asset(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Down => {
            let selected = match app.to_asset_table_state.selected() {
                Some(i) => {
                    if i >= MOCK_ASSETS.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            app.to_asset_table_state.select(Some(selected));
        }
        KeyCode::Up => {
            let selected = match app.to_asset_table_state.selected() {
                Some(i) => {
                    if i == 0 {
                        MOCK_ASSETS.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            app.to_asset_table_state.select(Some(selected));
        }
        KeyCode::Enter => {
            if let Some(i) = app.to_asset_table_state.selected() {
                app.to_asset = Some(MOCK_ASSETS[i].to_string());
                app.message = format!("Selected to asset: {}", MOCK_ASSETS[i]);
                app.input_mode = InputMode::Normal;
            }
        }
        _ => {}
    }
}

/// Handle input when entering address
fn handle_address_input(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Enter => {
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Char(c) => {
            app.address.push(c);
        }
        KeyCode::Backspace => {
            app.address.pop();
        }
        _ => {}
    }
}

/// Handle input when entering amount
fn handle_amount_input(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Enter => {
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Char(c) => {
            if c.is_digit(10) || c == '.' {
                app.amount.push(c);
            }
        }
        KeyCode::Backspace => {
            app.amount.pop();
        }
        _ => {}
    }
}