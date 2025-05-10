//! ## Instructions
//! 
//! Dynamic instructions component that updates based on app state

use tuirealm::command::{Cmd, CmdResult};
use tuirealm::event::NoUserEvent;
use tuirealm::props::{Alignment, Color, Style, TextModifiers};
use tuirealm::ratatui::layout::Rect;
use tuirealm::ratatui::widgets::Paragraph;
use tuirealm::{AttrValue, Attribute, Component, Event, Frame, MockComponent, Props, State};

use crate::ui::msg::Msg;

/// Instructions state for the component
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionsState {
    /// Need to select FROM asset
    SelectFromAsset,
    /// Need to select TO asset
    SelectToAsset,
    /// Need to select FROM amount
    SelectFromAmount,
}

impl Default for InstructionsState {
    fn default() -> Self {
        Self::SelectFromAsset
    }
}

/// Instructions component that provides contextual guidance
pub struct Instructions {
    props: Props,
    state: InstructionsState,
}

impl Default for Instructions {
    fn default() -> Self {
        Self {
            props: Props::default(),
            state: InstructionsState::default(),
        }
    }
}

impl Instructions {
    /// Create a new Instructions component
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Update the current state
    pub fn set_state(&mut self, state: InstructionsState) {
        self.state = state;
    }
    
    /// Get instruction text based on current state
    fn get_instruction_text(&self) -> String {
        match self.state {
            InstructionsState::SelectFromAsset => "Select FROM asset".to_string(),
            InstructionsState::SelectToAsset => "Select TO asset".to_string(),
            InstructionsState::SelectFromAmount => "Set FROM amount".to_string(),
        }
    }
}

impl MockComponent for Instructions {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        // Check if visible
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            // Get properties
            let instruction_text = self.get_instruction_text();
            let alignment = Alignment::Left;
            let foreground = Color::Green;
            let background = Color::Reset;
            let modifiers = TextModifiers::BOLD;

            frame.render_widget(
                Paragraph::new(format!("Instructions: {}", instruction_text))
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
            Attribute::Custom(state_str) if state_str == "state" || state_str == "state" => {
                if let AttrValue::Number(state_num) = value {
                    match state_num {
                        0 => self.set_state(InstructionsState::SelectFromAsset),
                        1 => self.set_state(InstructionsState::SelectToAsset),
                        2 => self.set_state(InstructionsState::SelectFromAmount),
                        _ => {}
                    }
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

impl Component<Msg, NoUserEvent> for Instructions {
    fn on(&mut self, _: Event<NoUserEvent>) -> Option<Msg> {
        // This component doesn't react to events
        None
    }
}