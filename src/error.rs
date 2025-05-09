use std::{error::Error, fmt};

/// Errors that can occur during the swap process
#[derive(Debug)]
pub enum SwapError {
    NoAssetSelected,
    InvalidAmount,
    QuoteFetchFailed,
}

impl fmt::Display for SwapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SwapError::NoAssetSelected => write!(f, "No asset selected"),
            SwapError::InvalidAmount => write!(f, "Invalid amount"),
            SwapError::QuoteFetchFailed => write!(f, "Failed to fetch quote"),
        }
    }
}

impl Error for SwapError {}