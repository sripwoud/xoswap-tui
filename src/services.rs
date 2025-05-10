use crate::error::SwapError;
use std::collections::HashMap;
use std::io;

/// Fetch a swap quote from the selected provider
pub fn fetch_quote(
    from_asset: &str, 
    to_asset: &str, 
    amount: f64, 
    providers: &[(String, String)], 
    selected_provider: usize
) -> Result<HashMap<String, f64>, SwapError> {
    // In a real application, this would make an HTTP request to a swap API
    // For now, we'll just return some mock data
    let mut quotes = HashMap::new();
    
    // Mock data generation - in a real app this would be API responses
    if amount <= 0.0 {
        return Err(SwapError::InvalidAmount);
    }
    
    // Get asset indices
    let from_idx = match from_asset {
        "BTC" => 0,
        "ETH" => 1,
        "SOL" => 2,
        _ => return Err(SwapError::NoAssetSelected),
    };
    
    let to_idx = match to_asset {
        "BTC" => 0,
        "ETH" => 1,
        "SOL" => 2,
        _ => return Err(SwapError::NoAssetSelected),
    };
    
    // Calculate the base exchange rate
    let exchange_idx = from_idx * 3 + to_idx;
    let base_rate = crate::models::MOCK_EXCHANGE_RATES[exchange_idx];
    
    // Add some variability based on provider (±5%)
    let provider_factor = match selected_provider {
        0 => 1.05,  // 0x: 5% better than base
        1 => 0.98,  // 1inch: 2% worse than base
        2 => 1.02,  // Paraswap: 2% better than base
        3 => 0.95,  // Rango: 5% worse than base
        4 => 1.00,  // Changelly: same as base
        _ => 1.00,  // Default
    };
    
    // Calculate the actual quote with the provider-specific rate
    let quote_amount = amount * base_rate * provider_factor;
    
    quotes.insert(
        providers[selected_provider].0.clone(),
        quote_amount,
    );
    
    Ok(quotes)
}

/// Generate a QR code for a swap transaction
pub fn generate_qr_code(
    from_asset: &str,
    to_asset: &str,
    amount: &str,
    address: &str,
    provider: &str
) -> Result<String, io::Error> {
    // In a real implementation, we would generate a proper QR code
    // with the transaction data. Here we'll use a mock representation.
    
    // Generate a transaction ID using a simple hash function
    let tx_id = format!(
        "TX-{:x}",
        from_asset.len() + to_asset.len() + amount.len() + address.len() + provider.len()
    );
    
    // Create a more detailed ASCII art QR code
    let qr_code = format!(
        "┌───────────────────────┐\n\
         │ █▀▀▀▀▀█ █▄█▀█ █▀▀▀▀▀█ │\n\
         │ █ ███ █  ▀▄▀█ █ ███ █ │\n\
         │ █ ▀▀▀ █ ▄█ ▄▀ █ ▀▀▀ █ │\n\
         │ ▀▀▀▀▀▀▀ ▀▄▀▄▀ ▀▀▀▀▀▀▀ │\n\
         │ ▀█ ▀▀▄▄ ▄▀▀ ▄▀█▀▀▄█▀█ │\n\
         │ ▀ █▀█▀▀▄█▄▄▀█ █▀▄▄▀▀█ │\n\
         │ █▀▀▀▀▀█ █ █ ▄ █▀▀▀▀▀█ │\n\
         │ █ ███ █ ▀█▀ ▄▀█ ███ █ │\n\
         │ █ ▀▀▀ █ ██▄▀█ █ ▀▀▀ █ │\n\
         └───────────────────────┘\n\
         Transaction: {}\n\
         Swap {} {} → {} via {}\n\
         Destination: {}\n\
         \n\
         [Press Esc or q to go back]",
        tx_id, amount, from_asset, to_asset, provider, address
    );
    
    Ok(qr_code)
}

