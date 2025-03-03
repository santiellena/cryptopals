use hex::FromHexError;

#[allow(dead_code)]
pub fn hex_xor(hex1: &str, hex2: &str) -> Result<String, FromHexError> {
    // Remove "0x" prefix if present and any whitespace
    let cleaned_hex1: &str = hex1.trim().trim_start_matches("0x");
    let cleaned_hex2: &str = hex2.trim().trim_start_matches("0x");

    // Convert hex string to raw bytes
    let bytes1: Vec<u8> = hex::decode(cleaned_hex1)?;
    let bytes2: Vec<u8> = hex::decode(cleaned_hex2)?;

    let l1: usize = bytes1.len();
    let l2: usize = bytes2.len();

    let lenght: usize = if l1 > l2 { l1 } else { l2 };

    let mut bytes_xor: Vec<u8> = vec![0; lenght];

    for i in 0..lenght {
        bytes_xor[i] = bytes1[i] ^ bytes2[i];
    }

    let result: String = hex::encode(bytes_xor);

    Ok(result)
}
