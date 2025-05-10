//! ## Id
//! 
//! Component identifiers for the UI

use std::fmt::{self, Display};

/// Component identifiers
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    /// Header component
    Header,
}

impl Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Header => write!(f, "header"),
        }
    }
}