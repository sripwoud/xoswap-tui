pub mod errors;
pub mod services;
pub mod ui;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    ui::app::run()?;
    Ok(())
}
