use base64::{engine::general_purpose, Engine as _};
use hex::FromHexError;

pub fn hex_to_base64(hex_input: &str) -> Result<String, FromHexError> {
    // Remove "0x" prefix if present and any whitespace
    let cleaned_hex: &str = hex_input.trim().trim_start_matches("0x");

    // Convert hex string to raw bytes
    let bytes: Vec<u8> = hex::decode(cleaned_hex)?;

    // Convert bytes to base64
    let base64_output: String = general_purpose::STANDARD.encode(&bytes);

    Ok(base64_output)
}
