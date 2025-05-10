//! ## Id
//! 
//! Component identifiers for the UI

use std::fmt::{self, Display};

/// Component identifiers
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    /// Header component
    Header,
    /// Instructions bar component
    InstructionsBar,
    /// Asset table component
    AssetTable,
    /// Status bar component
    StatusBar,
}

impl Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Header => write!(f, "header"),
            Self::InstructionsBar => write!(f, "instructions_bar"),
            Self::AssetTable => write!(f, "asset_table"),
            Self::StatusBar => write!(f, "status_bar"),
        }
    }
}