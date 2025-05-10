//! ## SummaryBar
//! 
//! Summary bar component for displaying transaction summary

use tuirealm::command::{Cmd, CmdResult};
use tuirealm::event::NoUserEvent;
use tuirealm::props::{Alignment, Color, Style, TextModifiers};
use tuirealm::ratatui::layout::Rect;
use tuirealm::ratatui::widgets::Paragraph;
use tuirealm::{AttrValue, Attribute, Component, Event, Frame, MockComponent, Props, State};

use crate::ui::msg::Msg;

/// SummaryBar component that displays transaction summary
/// This is a visual-only component that updates based on selected assets
#[derive(Default)]
pub struct SummaryBar {
    props: Props,
    from_ticker: Option<String>,
    to_ticker: Option<String>,
    from_amount: String,
    to_amount: String,
}

impl SummaryBar {
    /// Create a new SummaryBar
    pub fn new() -> Self {
        Self {
            props: Props::default(),
            from_ticker: None,
            to_ticker: None,
            from_amount: "1.0".to_string(), // Hardcoded for now
            to_amount: "123.45".to_string(), // Hardcoded for now
        }
    }

    /// Update from asset ticker
    pub fn set_from_ticker(&mut self, ticker: String) {
        self.from_ticker = Some(ticker);
    }

    /// Update to asset ticker
    pub fn set_to_ticker(&mut self, ticker: String) {
        self.to_ticker = Some(ticker);
    }

    /// Get formatted summary text
    fn get_summary_text(&self) -> String {
        let from_amount = &self.from_amount;
        let to_amount = &self.to_amount;
        
        let from_display = self.from_ticker.as_ref().map_or("{from_amount}".to_string(), |ticker| format!("{} {}", from_amount, ticker));
        let to_display = self.to_ticker.as_ref().map_or("{to_amount}".to_string(), |ticker| format!("{} {}", to_amount, ticker));
        
        format!("{} -> {}", from_display, to_display)
    }
}

impl MockComponent for SummaryBar {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        // Check if visible
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            // Get properties
            let summary_text = self.get_summary_text();
            let alignment = Alignment::Center;
            let foreground = Color::White;
            let background = Color::Reset;
            let modifiers = TextModifiers::BOLD;

            frame.render_widget(
                Paragraph::new(summary_text)
                    .style(
                        Style::default()
                            .fg(foreground)
                            .bg(background)
                            .add_modifier(modifiers),
                    )
                    .alignment(alignment),
                area,
            );
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        match attr {
            Attribute::Custom(custom) if custom == "from_ticker" || custom == "from_ticker" => {
                if let AttrValue::String(ticker) = value {
                    self.set_from_ticker(ticker);
                }
            },
            Attribute::Custom(custom) if custom == "to_ticker" || custom == "to_ticker" => {
                if let AttrValue::String(ticker) = value {
                    self.set_to_ticker(ticker);
                }
            },
            _ => self.props.set(attr, value),
        }
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, _: Cmd) -> CmdResult {
        CmdResult::None
    }
}

impl Component<Msg, NoUserEvent> for SummaryBar {
    fn on(&mut self, _: Event<NoUserEvent>) -> Option<Msg> {
        // This component doesn't react to events
        None
    }
}