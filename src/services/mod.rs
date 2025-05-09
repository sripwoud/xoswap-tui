use crate::error::SwapError;
use std::collections::HashMap;
use std::io;
use std::process::{Command, Stdio};

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
    
    let from_price = match from_asset {
        "BTC" => 40000.0,
        "ETH" => 2000.0,
        "XOS" => 1.0,
        _ => return Err(SwapError::NoAssetSelected),
    };
    
    let to_price = match to_asset {
        "BTC" => 40000.0,
        "ETH" => 2000.0,
        "XOS" => 1.0,
        _ => return Err(SwapError::NoAssetSelected),
    };
    
    quotes.insert(
        providers[selected_provider].0.clone(),
        (amount * from_price) / to_price,
    );
    
    Ok(quotes)
}

/// Generate a QR code for the given data
pub fn generate_qr_code(data: &str) -> Result<String, io::Error> {
    // This function attempts to use the 'qrencode' command-line tool to generate
    // QR codes in the terminal. If qrencode is not available, it falls back to
    // a mock representation.
    
    // Try to use the qrencode command if available
    let qrencode_result = Command::new("qrencode")
        .args(["-t", "ansiutf8", data])
        .stdout(Stdio::piped())
        .spawn();
    
    match qrencode_result {
        Ok(mut child) => {
            let output = String::new();
            if let Some(stdout) = child.stdout.take() {
                let mut reader = std::io::BufReader::new(stdout);
                std::io::copy(&mut reader, &mut output.as_bytes().to_vec())?;
            }
            
            match child.wait() {
                Ok(status) if status.success() => Ok(output),
                _ => {
                    // Fallback to mock QR code
                    Ok(generate_mock_qr_code(data))
                }
            }
        }
        Err(_) => {
            // qrencode not available, generate a simple mock QR code
            Ok(generate_mock_qr_code(data))
        }
    }
}

fn generate_mock_qr_code(data: &str) -> String {
    // Simple mock QR code (just for visualization)
    let mut mock_qr = String::new();
    mock_qr.push_str("▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄\n");
    mock_qr.push_str("█ ▄▄▄▄▄ █▀█▀█ ▄▄▄▄▄ █\n");
    mock_qr.push_str("█ █   █ █▀▀▀█ █   █ █\n");
    mock_qr.push_str("█ █▄▄▄█ █▀ ▀█ █▄▄▄█ █\n");
    mock_qr.push_str("█▄▄▄▄▄▄▄█▄█ █▄▄▄▄▄▄▄█\n");
    mock_qr.push_str("█  ▄██▄▄ ▀▄  █ ▄▄▀▀ █\n");
    mock_qr.push_str(format!("█  Data: {} █\n", data).as_str());
    mock_qr.push_str("▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀\n");
    mock_qr
}