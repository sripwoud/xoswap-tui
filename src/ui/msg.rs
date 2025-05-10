//! ## Msg
//! 
//! Application messages

/// Messages for the application
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Msg {
    /// Application should close
    AppClose,
    /// No operation message
    None,
}