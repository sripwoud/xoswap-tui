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
    /// Dynamic instructions component
    Instructions,
    /// Asset table component
    AssetTable,
    /// Summary bar component
    SummaryBar,
    /// Help bar component
    HelpBar,
}

impl Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Header => write!(f, "header"),
            Self::InstructionsBar => write!(f, "instructions_bar"),
            Self::Instructions => write!(f, "instructions"),
            Self::AssetTable => write!(f, "asset_table"),
            Self::SummaryBar => write!(f, "summary_bar"),
            Self::HelpBar => write!(f, "help_bar"),
        }
    }
}