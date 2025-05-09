//! XOSwap TUI - A terminal-based user interface for XOSwap
//!
//! This application provides a text-based UI for performing crypto swaps
//! using various swap providers.

// Define modules
pub mod app;
pub mod error;
pub mod models;
pub mod services;
pub mod ui;

// Re-export main entry point
pub use app::run;

// Main entry point for the application
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run()
}