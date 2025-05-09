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
use crate::services::{fetch_quote, generate_qr_code};
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
            } else if app.show_qr && (key.code == KeyCode::Char('q') || key.code == KeyCode::Esc) {
                app.show_qr = false;
                app.message = "QR code closed".to_string();
                continue;
            }

            match app.input_mode {
                InputMode::Normal => handle_normal_mode(app, key),
                InputMode::SelectingFrom => handle_select_from_asset(app, key),
                InputMode::SelectingTo => handle_select_to_asset(app, key),
                InputMode::EnteringAddress => handle_address_input(app, key),
                InputMode::EnteringAmount => handle_amount_input(app, key),
                InputMode::SelectingProvider => handle_select_provider(app, key),
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
        KeyCode::Up | KeyCode::Char('k') => {
            if matches!(app.input_mode, InputMode::Normal) {
                // Navigate up in the from asset table
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
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if matches!(app.input_mode, InputMode::Normal) {
                // Navigate down in the from asset table
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
            app.input_mode = InputMode::SelectingProvider;
            app.message = "Use arrow keys to select provider, Enter to confirm".to_string();
        }
        KeyCode::Char('g') => {
            // Generate QR code
            if app.from_asset.is_none() || app.to_asset.is_none() {
                app.message = "Error: Select from and to assets first".to_string();
                return;
            }
            
            if app.address.is_empty() {
                app.message = "Error: Enter an address first".to_string();
                return;
            }
            
            if app.amount.is_empty() {
                app.message = "Error: Enter an amount first".to_string();
                return;
            }
            
            if app.selected_provider.is_none() {
                app.message = "Error: Select a provider first".to_string();
                return;
            }

            app.show_qr = true;
            app.message = format!(
                "Showing QR code for swap: {} {} to {}, sending to {}",
                app.amount,
                app.from_asset.as_ref().unwrap(),
                app.to_asset.as_ref().unwrap(),
                app.address
            );
            
            // Generate a QR code representation for this swap
            match generate_qr_code(
                app.from_asset.as_ref().unwrap(),
                app.to_asset.as_ref().unwrap(),
                &app.amount,
                &app.address,
                &app.providers[app.selected_provider.unwrap()].0
            ) {
                Ok(qr) => {
                    app.qr_code = Some(qr);
                    app.message = format!("QR code generated for the swap transaction");
                }
                Err(e) => {
                    app.qr_code = None;
                    app.message = format!("Failed to generate QR code: {}", e);
                }
            }
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

/// Handle input when selecting provider
fn handle_select_provider(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Down | KeyCode::Char('j') => {
            // Cycle through providers
            let next = match app.selected_provider {
                Some(current) => Some((current + 1) % app.providers.len()),
                None => Some(0),
            };
            app.selected_provider = next;
            if let Some(idx) = next {
                app.message = format!("Selected provider: {}", app.providers[idx].0);
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            // Cycle through providers (reverse)
            let prev = match app.selected_provider {
                Some(current) => {
                    if current == 0 {
                        Some(app.providers.len() - 1)
                    } else {
                        Some(current - 1)
                    }
                }
                None => Some(app.providers.len() - 1),
            };
            app.selected_provider = prev;
            if let Some(idx) = prev {
                app.message = format!("Selected provider: {}", app.providers[idx].0);
            }
        }
        KeyCode::Enter => {
            if app.selected_provider.is_some() {
                let idx = app.selected_provider.unwrap();
                app.message = format!("Confirmed provider: {}", app.providers[idx].0);
                app.input_mode = InputMode::Normal;
            }
        }
        _ => {}
    }
}