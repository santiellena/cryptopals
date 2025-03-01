use hex::FromHexError;

#[allow(dead_code)]
pub fn hex_xor(hex1: &str, hex2: &str) -> Result<String, FromHexError> {
    // Remove "0x" prefix if present and any whitespace
    let cleaned_hex1: &str = hex1.trim().trim_start_matches("0x");
    let cleaned_hex2: &str = hex2.trim().trim_start_matches("0x");

    // Convert hex string to raw bytes
    let bytes1: Vec<u8> = hex::decode(cleaned_hex1)?;
    let bytes2: Vec<u8> = hex::decode(cleaned_hex2)?;

    let l1: u32 = bytes1.len() as u32;
    let l2: u32 = bytes2.len() as u32;

    let lenght: u32 = if l1 > l2 { l1 } else { l2 };

    let mut bytes_xor: Vec<u8> = vec![0; lenght as usize];

    for i in 0..lenght {
        let i_u: usize = i as usize;
        bytes_xor[i_u] = bytes1[i_u] ^ bytes2[i_u];
    }

    let result: String = hex::encode(bytes_xor);

    Ok(result)
}
