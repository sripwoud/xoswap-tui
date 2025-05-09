use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
    Frame,
};

use crate::models::{App, InputMode, MOCK_ASSETS, MOCK_PRICES};

/// Renders the user interface
pub fn ui<B: Backend>(f: &mut Frame, app: &App) {
    // Create the layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),  // Title
                Constraint::Length(3),  // Instructions
                Constraint::Length(7),  // From asset
                Constraint::Length(7),  // To asset
                Constraint::Length(3),  // Address
                Constraint::Length(3),  // Amount
                Constraint::Min(10),    // Quotes
                Constraint::Length(3),  // Status
            ]
            .as_ref(),
        )
        .split(f.area());

    // Title
    let title = Paragraph::new(Text::styled(
        "XOSwap TUI",
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    ))
    .alignment(ratatui::layout::Alignment::Center)
    .block(Block::default().borders(Borders::ALL).title("XOSwap"));
    f.render_widget(title, chunks[0]);

    // Instructions
    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            "Press 'f' to select from asset, 't' to select to asset, 'a' to enter address, 'm' to enter amount, 'p' to select provider, 'g' to generate QR, 'q' to quit",
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::SelectingFrom => (
            "Press Enter to select from asset, Esc to cancel",
            Style::default(),
        ),
        InputMode::SelectingTo => (
            "Press Enter to select to asset, Esc to cancel",
            Style::default(),
        ),
        InputMode::EnteringAddress => (
            "Enter an address, press Enter when done, Esc to cancel",
            Style::default(),
        ),
        InputMode::EnteringAmount => (
            "Enter an amount, press Enter when done, Esc to cancel",
            Style::default(),
        ),
        InputMode::SelectingProvider => (
            "Use Up/Down keys to select provider, Enter to confirm, Esc to cancel",
            Style::default(),
        ),
    };
    let instructions = Paragraph::new(Text::styled(msg, style))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Instructions"));
    f.render_widget(instructions, chunks[1]);

    // From asset
    let from_asset_rows = MOCK_ASSETS
        .iter()
        .enumerate()
        .map(|(i, &asset)| {
            let price = MOCK_PRICES[i];
            Row::new(vec![
                Cell::from(asset),
                Cell::from(format!("${:.2}", price)),
            ])
        })
        .collect::<Vec<_>>();

    let from_asset_table = Table::new(from_asset_rows, vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .header(Row::new(vec!["Asset", "Price"]).style(Style::default().fg(Color::Yellow)))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("From Asset")
                .style(
                    if matches!(app.input_mode, InputMode::SelectingFrom) {
                        Style::default().fg(Color::Cyan)
                    } else {
                        Style::default()
                    },
                ),
        )
        .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    let mut from_state = app.from_asset_table_state.clone();
    f.render_stateful_widget(from_asset_table, chunks[2], &mut from_state);

    // To asset
    let to_asset_rows = MOCK_ASSETS
        .iter()
        .enumerate()
        .map(|(i, &asset)| {
            let price = MOCK_PRICES[i];
            Row::new(vec![
                Cell::from(asset),
                Cell::from(format!("${:.2}", price)),
            ])
        })
        .collect::<Vec<_>>();

    let to_asset_table = Table::new(to_asset_rows, vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .header(Row::new(vec!["Asset", "Price"]).style(Style::default().fg(Color::Yellow)))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("To Asset")
                .style(
                    if matches!(app.input_mode, InputMode::SelectingTo) {
                        Style::default().fg(Color::Cyan)
                    } else {
                        Style::default()
                    },
                ),
        )
        .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    let mut to_state = app.to_asset_table_state.clone();
    f.render_stateful_widget(to_asset_table, chunks[3], &mut to_state);

    // Address
    let address = Paragraph::new(Text::from(app.address.as_str()))
        .style(
            if matches!(app.input_mode, InputMode::EnteringAddress) {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            },
        )
        .block(Block::default().borders(Borders::ALL).title("Address"));
    f.render_widget(address, chunks[4]);

    // Amount
    let amount = Paragraph::new(Text::from(app.amount.as_str()))
        .style(
            if matches!(app.input_mode, InputMode::EnteringAmount) {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            },
        )
        .block(Block::default().borders(Borders::ALL).title("Amount"));
    f.render_widget(amount, chunks[5]);

    // Quotes or QR Code
    if app.show_qr {
        let qr_block = Block::default().borders(Borders::ALL).title("QR Code");
        let inner_area = qr_block.inner(chunks[6]);
        f.render_widget(qr_block, chunks[6]);
        
        // Display the stored QR code if available, otherwise show a message
        let qr_display = match &app.qr_code {
            Some(qr_code) => qr_code.clone(),
            None => {
                if let (Some(_), Some(_)) = (&app.from_asset, &app.to_asset) {
                    if !app.address.is_empty() && !app.amount.is_empty() && app.selected_provider.is_some() {
                        "QR code not generated yet. Press 'g' to generate.".to_string()
                    } else {
                        "Missing required information for QR code\n[Press Esc or q to go back]".to_string()
                    }
                } else {
                    "Missing required information for QR code\n[Press Esc or q to go back]".to_string()
                }
            }
        };
        
        let qr_text = Paragraph::new(Text::from(qr_display))
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(qr_text, inner_area);
    } else if !app.quotes.is_empty() {
        let quote_rows: Vec<Row> = app
            .quotes
            .iter()
            .map(|(provider, quote)| {
                Row::new(vec![
                    Cell::from(provider.clone()),
                    Cell::from(format!("{:.8}", quote)),
                ])
            })
            .collect();

        let quote_table = Table::new(quote_rows, vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .header(Row::new(vec!["Provider", "Quote"]).style(Style::default().fg(Color::Yellow)))
            .block(Block::default().borders(Borders::ALL).title("Quotes"));

        f.render_widget(quote_table, chunks[6]);
    } else {
        // Provider list
        let provider_items: Vec<Row> = app
            .providers
            .iter()
            .enumerate()
            .map(|(i, (name, url))| {
                let style = if app.selected_provider == Some(i) {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default()
                };
                Row::new(vec![
                    Cell::from(name.clone()).style(style),
                    Cell::from(url.clone()).style(style)
                ])
            })
            .collect();

        let providers_table = Table::new(provider_items, vec![Constraint::Percentage(30), Constraint::Percentage(70)])
            .header(Row::new(vec!["Provider", "API Endpoint"]).style(Style::default().fg(Color::Yellow)))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Providers")
                    .style(
                        if matches!(app.input_mode, InputMode::SelectingProvider) {
                            Style::default().fg(Color::Cyan)
                        } else {
                            Style::default()
                        },
                    )
            )
            .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED));

        let mut provider_state = TableState::default();
        if let Some(selected) = app.selected_provider {
            provider_state.select(Some(selected));
        }

        f.render_stateful_widget(providers_table, chunks[6], &mut provider_state);
    }

    // Status
    let status = Paragraph::new(Text::from(app.message.as_str()))
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Status"));
    f.render_widget(status, chunks[7]);
}