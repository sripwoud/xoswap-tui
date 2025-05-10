//! ## Theme
//! 
//! Theme constants for the application

use tuirealm::props::{Alignment, Borders, Color, TextModifiers};

pub mod colors {
    use super::Color;

    pub const PRIMARY: Color = Color::Cyan;
    pub const SECONDARY: Color = Color::LightGreen;
    pub const BACKGROUND: Color = Color::Reset;
    pub const TEXT: Color = Color::White;
    pub const HIGHLIGHT: Color = Color::Yellow;
    pub const ERROR: Color = Color::Red;
}

pub mod style {
    use super::{Alignment, Borders, TextModifiers};

    pub const DEFAULT_ALIGNMENT: Alignment = Alignment::Left;
    pub const CENTER_ALIGNMENT: Alignment = Alignment::Center;
    pub const DEFAULT_MODIFIERS: TextModifiers = TextModifiers::empty();
    pub const BOLD: TextModifiers = TextModifiers::BOLD;
    pub fn default_borders() -> Borders {
        Borders::default()
    }
}