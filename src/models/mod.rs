use std::collections::HashMap;
use ratatui::widgets::TableState;

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
pub const MOCK_ASSETS: [&str; 3] = ["BTC", "ETH", "XOS"];

/// Mock data for asset prices
pub const MOCK_PRICES: [f64; 3] = [40000.0, 2000.0, 1.0];

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
    pub from_asset_table_state: TableState,
    pub to_asset_table_state: TableState,
    pub providers: Vec<(String, String)>,
    pub quotes: HashMap<String, f64>,
    pub selected_provider: Option<usize>,
    pub show_qr: bool,
}

impl Default for App {
    fn default() -> Self {
        let mut from_state = TableState::default();
        from_state.select(Some(0));
        let mut to_state = TableState::default();
        to_state.select(Some(0));

        Self {
            from_asset: None,
            to_asset: None,
            address: String::new(),
            amount: String::new(),
            message: String::new(),
            input_mode: InputMode::Normal,
            from_asset_table_state: from_state,
            to_asset_table_state: to_state,
            providers: MOCK_PROVIDERS.iter().map(|&(name, url)| (name.to_string(), url.to_string())).collect(),
            quotes: HashMap::new(),
            selected_provider: None,
            show_qr: false,
        }
    }
}