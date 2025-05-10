//! ## Id
//! 
//! Component identifiers for the UI

use std::fmt::{self, Display};

/// Component identifiers
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    /// Header component
    Header,
    /// Status bar component
    StatusBar,
}

impl Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Header => write!(f, "header"),
            Self::StatusBar => write!(f, "status_bar"),
        }
    }
}