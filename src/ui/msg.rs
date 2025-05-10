//! ## Msg
//! 
//! Application messages

/// Messages for the application
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Msg {
    /// Application should close
    AppClose,
    /// Asset was highlighted (but not selected)
    AssetSelected(usize),
    /// Asset was chosen as the FROM asset (Enter pressed)
    AssetChosenAsFrom(usize),
    /// Asset was chosen as the TO asset (Tab pressed)
    AssetChosenAsTo(usize),
    /// Enter FROM asset selection mode
    EnterFromAssetMode,
    /// Enter TO asset selection mode
    EnterToAssetMode,
    /// Exit asset selection mode
    ExitAssetSelectionMode,
    /// No operation message
    None,
}