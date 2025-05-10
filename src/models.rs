use ratatui::widgets::TableState;
use std::collections::HashMap;

// Import custom modules

/// Input modes for the application
pub enum InputMode {
    SelectingFrom,
    SelectingTo,
    EnteringAddress,
    EnteringAmount,
    Normal,
}

/// Mock data for assets
pub const MOCK_ASSETS: [&str; 3] = ["BTC", "ETH", "SOL"];

/// Mock data for asset prices
pub const MOCK_PRICES: [f64; 3] = [100000.0, 2000.0, 140.0];

/// Mock data for exchange rates between assets
pub const MOCK_EXCHANGE_RATES: [f64; 9] = [
    1.0,    // BTC->BTC
    50.0,   // BTC->ETH
    714.28, // BTC->SOL
    0.02,   // ETH->BTC
    1.0,    // ETH->ETH
    14.28,  // ETH->SOL
    0.0014, // SOL->BTC
    0.07,   // SOL->ETH
    1.0,    // SOL->SOL
];

/// Mock data for swap providers
pub const MOCK_PROVIDERS: [(&str, &str); 5] = [
    ("0x", "https://api.0x.org/swap/v1/quote"),
    ("1inch", "https://api.1inch.io/v5.0/1/quote"),
    ("Paraswap", "https://api.paraswap.io/prices"),
    ("Rango", "https://api.rango.exchange/routing"),
    ("Changelly", "https://api.changelly.com/v2"),
];

/// Represents the application state
pub struct App {
    pub from_asset: Option<String>,
    pub to_asset: Option<String>,
    pub address: String,
    pub amount: String,
    pub message: String,
    pub input_mode: InputMode,
    pub asset_table_state: TableState,
    pub providers: Vec<(String, String)>,
    pub quotes: HashMap<String, f64>,
    pub selected_provider: Option<usize>,
    pub show_qr: bool,
    pub qr_code: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        let mut asset_state = TableState::default();
        asset_state.select(Some(0));

        Self {
            from_asset: None,
            to_asset: None,
            address: String::new(),
            amount: String::new(),
            message: String::new(),
            input_mode: InputMode::Normal,
            asset_table_state: asset_state,
            providers: MOCK_PROVIDERS
                .iter()
                .map(|&(name, url)| (name.to_string(), url.to_string()))
                .collect(),
            quotes: HashMap::new(),
            selected_provider: None,
            show_qr: false,
            qr_code: None,
        }
    }
}
