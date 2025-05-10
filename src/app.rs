use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{error::Error, io};

use crate::models::{App, InputMode, WorkflowStage, MOCK_ASSETS};
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
    
    // Start in SelectingFrom mode
    app.input_mode = InputMode::SelectingFrom;
    app.workflow_stage = WorkflowStage::SelectingFrom;

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
            // Handle global exit keys
            if key.code == KeyCode::Char('q')
                && !app.show_qr
                && matches!(app.input_mode, InputMode::Normal)
            {
                return Ok(());
            } else if app.show_qr && (key.code == KeyCode::Char('q') || key.code == KeyCode::Esc) {
                app.show_qr = false;
                app.message = "QR code closed".to_string();
                continue;
            }

            // Handle shortcut keys regardless of mode
            match key.code {
                KeyCode::Char('f') => {
                    app.input_mode = InputMode::SelectingFrom;
                    app.workflow_stage = WorkflowStage::SelectingFrom;
                    app.message = "Select FROM asset (navigate with up/down, Enter to select)".to_string();
                    continue;
                },
                KeyCode::Char('t') => {
                    app.input_mode = InputMode::SelectingTo;
                    app.workflow_stage = WorkflowStage::SelectingTo;
                    app.message = "Select TO asset (navigate with up/down, Enter to select)".to_string();
                    continue;
                },
                KeyCode::Char('a') => {
                    app.input_mode = InputMode::Normal;
                    app.workflow_stage = WorkflowStage::Normal;
                    app.message = "Enter destination address".to_string();
                    continue;
                },
                KeyCode::Char('m') => {
                    app.input_mode = InputMode::EnteringAmount;
                    app.workflow_stage = WorkflowStage::EnteringAmount;
                    app.message = "Enter amount to swap".to_string();
                    continue;
                },
                _ => {} // Continue to mode-specific handlers
            }

            // Handle mode-specific input
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
        // In normal mode, arrow keys automatically enter selection mode
        KeyCode::Up | KeyCode::Down => {
            // If no mode active, start with from selection
            app.input_mode = InputMode::SelectingFrom;
            app.workflow_stage = WorkflowStage::SelectingFrom;
            app.message = "Select FROM asset (navigate with up/down, Enter to select)".to_string();
            
            // Process the key in from selection mode
            handle_select_from_asset(app, key);
            return;
        }
        KeyCode::Char('f') => {
            app.input_mode = InputMode::SelectingFrom;
            app.workflow_stage = WorkflowStage::SelectingFrom;
            app.message = "Select FROM asset (navigate with up/down, Enter to select)".to_string();
            
            // Highlight the currently selected row in asset table
            if app.from_asset.is_some() {
                // Try to select the current from asset
                for (i, &asset) in MOCK_ASSETS.iter().enumerate() {
                    if Some(asset.to_string()) == app.from_asset {
                        app.asset_table_state.select(Some(i));
                        break;
                    }
                }
            } else {
                // Make sure selection is visible by selecting first item if nothing is selected
                if app.asset_table_state.selected().is_none() {
                    app.asset_table_state.select(Some(0));
                }
            }
        }
        KeyCode::Char('k') => {
            // Only navigate in normal mode
            if matches!(app.input_mode, InputMode::Normal) {
                let selected = match app.asset_table_state.selected() {
                    Some(i) => {
                        if i == 0 {
                            MOCK_ASSETS.len() - 1
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                app.asset_table_state.select(Some(selected));
            }
        }
        KeyCode::Char('j') => {
            // Only navigate in normal mode
            if matches!(app.input_mode, InputMode::Normal) {
                let selected = match app.asset_table_state.selected() {
                    Some(i) => {
                        if i >= MOCK_ASSETS.len() - 1 {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                app.asset_table_state.select(Some(selected));
            }
        }
        KeyCode::Char('t') => {
            app.input_mode = InputMode::SelectingTo;
            app.workflow_stage = WorkflowStage::SelectingTo;
            app.message = "Select TO asset (navigate with up/down, Enter to select)".to_string();
            
            // Highlight the currently selected row in asset table
            if app.to_asset.is_some() {
                // Try to select the current to asset
                for (i, &asset) in MOCK_ASSETS.iter().enumerate() {
                    if Some(asset.to_string()) == app.to_asset {
                        app.asset_table_state.select(Some(i));
                        break;
                    }
                }
            } else {
                // Make sure selection is visible by selecting first item if nothing is selected
                if app.asset_table_state.selected().is_none() {
                    app.asset_table_state.select(Some(0));
                }
            }
        }
        KeyCode::Char('a') => {
            app.input_mode = InputMode::EnteringAddress;
            app.workflow_stage = WorkflowStage::EnteringAddress;
        }
        KeyCode::Char('m') => {
            app.input_mode = InputMode::EnteringAmount;
            app.workflow_stage = WorkflowStage::EnteringAmount;
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

            // We'll handle provider selection automatically
            if app.selected_provider.is_none() {
                // Select the first provider by default for initial fetch
                app.selected_provider = Some(0);
            }

            // Use the helper function to fetch quotes from all providers
            fetch_quotes_from_all_providers(app);
        }
        KeyCode::Char('q') if app.show_qr => {
            app.show_qr = false;
            app.message = "Returned to main screen".to_string();
        }
        _ => {}
    }
}

/// Handle input when selecting from asset
fn handle_select_from_asset(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::EnteringAmount;
            app.workflow_stage = WorkflowStage::EnteringAmount;
            app.message = "Cancelled asset selection".to_string();
        }
        KeyCode::Up => {
            let selected = match app.asset_table_state.selected() {
                Some(i) => {
                    if i == 0 {
                        MOCK_ASSETS.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            app.asset_table_state.select(Some(selected));
            app.message = format!("Selecting TO asset: {}", MOCK_ASSETS[selected]);
        }
        KeyCode::Down => {
            let selected = match app.asset_table_state.selected() {
                Some(i) => {
                    if i >= MOCK_ASSETS.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            app.asset_table_state.select(Some(selected));
            app.message = format!("Selecting: {}", MOCK_ASSETS[selected]);
        }
        KeyCode::Enter => {
            if let Some(i) = app.asset_table_state.selected() {
                // Set the from asset
                app.from_asset = Some(MOCK_ASSETS[i].to_string());
                
                // Clear quotes and QR when changing from asset
                app.quotes.clear();
                app.qr_code = None;
                app.show_qr = false;
                
                // Progress to next stage in workflow
                app.workflow_stage = WorkflowStage::SelectingTo;
                app.input_mode = InputMode::SelectingTo;
                app.message = format!("FROM asset selected: {}. Now select TO asset.", MOCK_ASSETS[i]);
                
                // Log very clearly which asset was selected
                println!("FROM ASSET SET TO: {}", MOCK_ASSETS[i]);
                
                // Auto-fetch quotes when both from and to assets are selected
                if app.to_asset.is_some() {
                    // Always fetch quotes even without amount - we'll use a default amount of 1.0
                    if app.amount.is_empty() {
                        app.amount = "1.0".to_string();
                    }
                    fetch_quotes_from_all_providers(app);
                }
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
            app.message = "Cancelled asset selection".to_string();
        }
        KeyCode::Up => {
            let selected = match app.asset_table_state.selected() {
                Some(i) => {
                    if i == 0 {
                        MOCK_ASSETS.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            app.asset_table_state.select(Some(selected));
            app.message = format!("Selecting: {}", MOCK_ASSETS[selected]);
        }
        KeyCode::Down => {
            let selected = match app.asset_table_state.selected() {
                Some(i) => {
                    if i >= MOCK_ASSETS.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            app.asset_table_state.select(Some(selected));
            app.message = format!("Selecting: {}", MOCK_ASSETS[selected]);
        }
        KeyCode::Enter => {
            if let Some(i) = app.asset_table_state.selected() {
                // Set the to asset
                app.to_asset = Some(MOCK_ASSETS[i].to_string());
                
                // Progress to next stage in workflow
                app.workflow_stage = WorkflowStage::EnteringAmount;
                app.input_mode = InputMode::EnteringAmount;
                app.message = format!("TO asset selected: {}. Now enter amount to swap.", MOCK_ASSETS[i]);
                
                // Log very clearly which asset was selected as TO
                println!("TO ASSET SET TO: {}", MOCK_ASSETS[i]);
                
                // Auto-fetch quotes when both from and to assets are selected
                if app.from_asset.is_some() {
                    // Always fetch quotes even without amount - we'll use a default amount of 1.0
                    if app.amount.is_empty() {
                        app.amount = "1.0".to_string();
                    }
                    fetch_quotes_from_all_providers(app);
                }
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
            // Progress to next stage in workflow - show QR code
            app.workflow_stage = WorkflowStage::ShowingQR;
            app.message = "Address entered. Generating QR code...".to_string();

            // Auto-fetch quotes and generate QR if all info is available
            if app.from_asset.is_some() && app.to_asset.is_some() && !app.amount.is_empty() {
                fetch_quotes_from_all_providers(app);
                app.show_qr = true;
            }
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
            // Progress to next stage in workflow
            app.workflow_stage = WorkflowStage::EnteringAddress;
            app.input_mode = InputMode::EnteringAddress;
            app.message = "Amount entered. Now enter destination address.".to_string();

            // Auto-fetch quotes when amount is entered and both assets are selected
            if app.from_asset.is_some() && app.to_asset.is_some() && !app.amount.is_empty() {
                fetch_quotes_from_all_providers(app);
            }
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

/// Helper function to fetch quotes from all providers
fn fetch_quotes_from_all_providers(app: &mut App) {
    if app.from_asset.is_none() || app.to_asset.is_none() {
        app.message = "Error: Select from and to assets first".to_string();
        return;
    }

    if app.amount.is_empty() {
        // Use default amount of 1.0 if empty
        app.amount = "1.0".to_string();
    }

    let amount = match app.amount.parse::<f64>() {
        Ok(amount) => amount,
        Err(_) => {
            app.message = "Error: Invalid amount".to_string();
            return;
        }
    };

    // Clear existing quotes
    app.quotes.clear();

    // Fetch quotes from all providers
    for i in 0..app.providers.len() {
        match fetch_quote(
            app.from_asset.as_ref().unwrap(),
            app.to_asset.as_ref().unwrap(),
            amount,
            &app.providers,
            i,
        ) {
            Ok(provider_quotes) => {
                app.quotes.extend(provider_quotes);
            }
            Err(_) => {
                // Silently ignore failed providers
            }
        }
    }

    if !app.quotes.is_empty() {
        // Select the provider with the best quote
        let mut best_provider_idx = 0;
        let mut best_quote = 0.0;

        for (i, (name, _)) in app.providers.iter().enumerate() {
            if let Some(quote) = app.quotes.get(name) {
                if *quote > best_quote {
                    best_quote = *quote;
                    best_provider_idx = i;
                }
            }
        }

        app.selected_provider = Some(best_provider_idx);
        app.message = format!(
            "Found best quote from {}",
            app.providers[best_provider_idx].0
        );

        // Auto-generate QR code if address is available
        if !app.address.is_empty() {
            match generate_qr_code(
                app.from_asset.as_ref().unwrap(),
                app.to_asset.as_ref().unwrap(),
                &app.amount,
                &app.address,
                &app.providers[best_provider_idx].0,
            ) {
                Ok(qr) => {
                    app.qr_code = Some(qr);
                    app.show_qr = true;
                }
                Err(e) => {
                    app.qr_code = None;
                    app.message = format!("Failed to generate QR code: {}", e);
                }
            }
        }
    } else {
        app.message = "No quotes available from any provider".to_string();
    }
}
