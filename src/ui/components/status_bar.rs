//! ## StatusBar
//! 
//! Status bar component for the application

use tuirealm::command::{Cmd, CmdResult};
use tuirealm::event::NoUserEvent;
use tuirealm::props::{Alignment, Color, Style, TextModifiers};
use tuirealm::ratatui::layout::Rect;
use tuirealm::ratatui::widgets::Paragraph;
use tuirealm::{AttrValue, Attribute, Component, Event, Frame, MockComponent, Props, State};

use crate::ui::msg::Msg;

/// StatusBar component that displays status information
/// This is a visual-only component that doesn't handle any events
#[derive(Default)]
pub struct StatusBar {
    props: Props,
}

impl StatusBar {
    /// Create a new StatusBar
    pub fn new() -> Self {
        Self {
            props: Props::default(),
        }
    }
}

impl MockComponent for StatusBar {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        // Check if visible
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            // Get properties
            let status_text = "Ready";
            let alignment = Alignment::Right;
            let foreground = Color::Gray;
            let background = Color::Reset;
            let modifiers = TextModifiers::empty();

            frame.render_widget(
                Paragraph::new(status_text)
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
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, _: Cmd) -> CmdResult {
        CmdResult::None
    }
}

impl Component<Msg, NoUserEvent> for StatusBar {
    fn on(&mut self, _: Event<NoUserEvent>) -> Option<Msg> {
        // This component doesn't react to events
        None
    }
}