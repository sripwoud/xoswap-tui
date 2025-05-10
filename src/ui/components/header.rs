//! ## Header
//!
//! Header component for the application

use tuirealm::command::{Cmd, CmdResult};
use tuirealm::event::{Key, KeyEvent, KeyModifiers, NoUserEvent};
use tuirealm::props::{Alignment, Color, Style, TextModifiers};
use tuirealm::ratatui::layout::Rect;
use tuirealm::ratatui::widgets::Paragraph;
use tuirealm::{AttrValue, Attribute, Component, Event, Frame, MockComponent, Props, State};

use crate::ui::msg::Msg;

/// Header component that displays the application title
#[derive(Default)]
pub struct Header {
    props: Props,
}

impl Header {
    /// Create a new Header
    pub fn new() -> Self {
        Self {
            props: Props::default(),
        }
    }
}

impl MockComponent for Header {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        // Check if visible
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            // Get properties
            let title = "XOSwap TUI";
            let alignment = Alignment::Center;
            let foreground = Color::Cyan;
            let background = Color::Reset;
            let modifiers = TextModifiers::BOLD;

            frame.render_widget(
                Paragraph::new(title)
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

impl Component<Msg, NoUserEvent> for Header {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Char('q'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::AppClose),
            Event::Keyboard(KeyEvent {
                code: Key::Esc,
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::AppClose),
            _ => None,
        }
    }
}
