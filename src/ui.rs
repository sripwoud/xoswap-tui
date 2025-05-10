use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table},
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
                Constraint::Length(2),  // Title
                Constraint::Length(2),  // Instructions (borderless)
                Constraint::Length(10), // Asset selection table
                Constraint::Length(3),  // Swap info row [from] -> [to]
                Constraint::Min(10),    // QR Code zone
                Constraint::Length(2),  // Status
            ]
            .as_ref(),
        )
        .split(f.area());

    // Title
    let title = Paragraph::new(Text::styled(
        "xoswap tui",
        Style::default()
            .fg(Color::LightMagenta)
            .add_modifier(Modifier::BOLD),
    ))
    .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    // Instructions - with blinking effect and no borders
    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            "Press 'f' to select from asset, 't' to select to asset, 'a' to enter address, 'm' to enter amount, 'q' to quit",
            Style::default().fg(Color::LightYellow).add_modifier(Modifier::BOLD),
        ),
        InputMode::SelectingFrom => (
            "Press Enter to select from asset, Esc to cancel",
            Style::default().fg(Color::LightCyan),
        ),
        InputMode::SelectingTo => (
            "Press Enter to select to asset, Esc to cancel",
            Style::default().fg(Color::LightCyan),
        ),
        InputMode::EnteringAddress => (
            "Enter an address, press Enter when done, Esc to cancel",
            Style::default().fg(Color::LightCyan),
        ),
        InputMode::EnteringAmount => (
            "Enter an amount, press Enter when done, Esc to cancel",
            Style::default().fg(Color::LightCyan),
        ),
    };

    // During selection modes, always show instructions without blinking
    // Only blink in normal mode
    let visible = if matches!(
        app.input_mode,
        InputMode::SelectingFrom
            | InputMode::SelectingTo
            | InputMode::EnteringAddress
            | InputMode::EnteringAmount
    ) {
        true
    } else {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        (now / 800) % 2 == 0
    };

    if visible {
        let instructions = Paragraph::new(Text::styled(msg, style)).alignment(Alignment::Center);
        f.render_widget(instructions, chunks[1]);
    }

    // Asset selection table with highlighted rows
    let asset_table_header = Row::new(vec![
        Cell::from(Span::styled(
            "Asset",
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Price",
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )),
    ]);

    // Asset rows are now created directly in the asset_rows_with_indicator

    // Change highlight style based on input mode - make selection more visible 
    let highlight_style = match app.input_mode {
        InputMode::SelectingFrom => Style::default()
            .bg(Color::Red)
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
        InputMode::SelectingTo => Style::default()
            .bg(Color::Green)
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
        _ => Style::default()
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD),
    };

    // Simplify - just create new rows with indicator
    let asset_rows_with_indicator: Vec<Row> = MOCK_ASSETS
        .iter()
        .enumerate()
        .map(|(i, &asset)| {
            let from_selected = app.from_asset.as_ref().map_or(false, |a| a == asset);
            let to_selected = app.to_asset.as_ref().map_or(false, |a| a == asset);
            let currently_selected = app.asset_table_state.selected() == Some(i);

            // Define row styling based on selection
            let style = if from_selected {
                Style::default().bg(Color::Red).fg(Color::White)
            } else if to_selected {
                Style::default().bg(Color::Green).fg(Color::White)
            } else {
                Style::default()
            };

            let price = MOCK_PRICES[i];

            // Add arrow indicator if this row is currently selected
            let display_asset = if currently_selected {
                format!("→ {}", asset)
            } else {
                asset.to_string()
            };

            Row::new(vec![
                Cell::from(display_asset).style(style),
                Cell::from(format!("${:.2}", price)).style(style),
            ])
        })
        .collect::<Vec<_>>();

    let asset_table = Table::new(
        asset_rows_with_indicator,
        vec![Constraint::Percentage(60), Constraint::Percentage(40)],
    )
    .header(asset_table_header)
    .block(Block::default().borders(Borders::ALL))
    .row_highlight_style(highlight_style);

    // Render asset table with selected row highlight
    let mut table_state = app.asset_table_state.clone();

    // Always ensure there's a selection visible in the table
    // This is crucial for navigation in selection modes
    if table_state.selected().is_none() {
        table_state.select(Some(0));
    }

    f.render_stateful_widget(asset_table, chunks[2], &mut table_state);

    // Swap info row [from amount] [from ticker] --> [to amount calculated] [to ticker selected]
    let from_amount = app.amount.clone();
    let from_ticker = app.from_asset.clone().unwrap_or_else(|| "...".to_string());

    // Calculate to amount from best quote
    let to_amount_text = if !app.quotes.is_empty()
        && app.from_asset.is_some()
        && app.to_asset.is_some()
        && !app.amount.is_empty()
    {
        let best_quote = app
            .quotes
            .values()
            .fold(0.0, |acc, &quote| if quote > acc { quote } else { acc });
        format!("{:.8}", best_quote)
    } else {
        "...".to_string()
    };

    let to_ticker = app.to_asset.clone().unwrap_or_else(|| "...".to_string());

    // Create styled spans for the swap text
    let mut swap_spans = Vec::new();

    // Create a cleaner swap display showing exact values without brackets
    // Format: {from_amount} {from_ticker} --> {to_amount} {to_ticker}

    // FROM section in red
    swap_spans.push(Span::styled(from_amount, Style::default().fg(Color::Red)));

    swap_spans.push(Span::raw(" "));

    swap_spans.push(Span::styled(
        from_ticker,
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
    ));

    // Arrow in white
    swap_spans.push(Span::styled(
        " --> ",
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    ));

    // TO section in green
    swap_spans.push(Span::styled(
        to_amount_text,
        Style::default().fg(Color::Green),
    ));

    swap_spans.push(Span::raw(" "));

    swap_spans.push(Span::styled(
        to_ticker,
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    ));

    let swap_info = Paragraph::new(Line::from(swap_spans))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(swap_info, chunks[3]);

    // Address display (at top-right corner of QR zone)
    let address_text = if app.address.is_empty() {
        "No address set".to_string()
    } else {
        format!("To: {}", app.address)
    };

    let address_style = if matches!(app.input_mode, InputMode::EnteringAddress) {
        Style::default()
            .fg(Color::LightYellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    // QR Code area - with dashed borders
    let should_show_qr = (app.from_asset.is_some()
        && app.to_asset.is_some()
        && !app.address.is_empty()
        && !app.amount.is_empty())
        || app.show_qr;

    // QR code with dashed border
    let qr_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray));

    let inner_area = qr_block.inner(chunks[4]);
    f.render_widget(qr_block, chunks[4]);

    // Place "qr code zone" text in the center if no QR code
    let qr_display = if should_show_qr {
        match &app.qr_code {
            Some(qr_code) => qr_code.clone(),
            _ => "qr code zone (no border)".to_string(),
        }
    } else {
        "qr code zone (no border)".to_string()
    };

    // Add address display above QR code
    if !app.address.is_empty() {
        let addr_para =
            Paragraph::new(Text::styled(address_text, address_style)).alignment(Alignment::Left);

        let addr_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(1)].as_ref())
            .margin(1)
            .split(chunks[4])[0];

        f.render_widget(addr_para, addr_area);
    }

    let qr_text = Paragraph::new(Text::from(qr_display))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));

    f.render_widget(qr_text, inner_area);

    // Status - simple text at bottom
    let status = Paragraph::new(Text::from(app.message.as_str()))
        .style(Style::default().fg(Color::LightGreen));
    f.render_widget(status, chunks[5]);
}
