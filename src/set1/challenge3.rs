use hex::FromHexError;

#[path = "./challenge2.rs"]
mod challenge2;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Decoded {
    pub character: char,
    decoded: String,
    pub score: u32,
}

#[allow(dead_code)]
impl Decoded {
    pub fn new(character: char, decoded: String, score: u32) -> Self {
        Self {
            character,
            decoded,
            score,
        }
    }
}

// Function to score a string based on character frequency
// To do this one I had to look on the internet because the frecuency scores thing was unknown to me
#[allow(dead_code)]
pub fn score_english_text(text: &str) -> u32 {
    // Define approximate English letter frequencies (as a score multiplier)
    // These are rough weights based on typical English frequencies
    let mut frequency_scores: [u32; 128] = [0; 128]; // ASCII range

    // Assign higher scores to common English characters
    // ETAOIN SHRDLU 😉😉
    frequency_scores['e' as usize] = 130; // ~12.7%
    frequency_scores['t' as usize] = 90; // ~9.1%
    frequency_scores['a' as usize] = 80; // ~8.2%
    frequency_scores['o' as usize] = 75; // ~7.5%
    frequency_scores['i' as usize] = 70; // ~7.0%
    frequency_scores['n' as usize] = 67; // ~6.7%
    frequency_scores['s' as usize] = 63; // ~6.3%
    frequency_scores['h' as usize] = 61; // ~6.1%
    frequency_scores['r' as usize] = 60; // ~6.0%
    frequency_scores['d' as usize] = 43; // ~4.3%
    frequency_scores['l' as usize] = 40; // ~4.0%
    frequency_scores['u' as usize] = 28; // ~2.8%
    frequency_scores[' ' as usize] = 150; // Spaces are very common

    // Lowercase everything for scoring (English frequency is often based on lowercase)
    let text = text.to_ascii_lowercase();
    let mut score: u32 = 0;

    for c in text.chars() {
        if (c as u32) < 128 {
            // Add the frequency score for this character
            score += frequency_scores[c as usize];
        } else {
            // Chill penalty non-ASCII characters
            score = score.saturating_sub(100);
        }
    }

    // Penalize unprintable ASCII characters (except space)
    for c in text.chars() {
        if (c as u32) < 32 && c != ' ' {
            // Heavy penalty for unprintable chars
            score = score.saturating_sub(200);
        }
    }

    score
}

#[allow(dead_code)]
pub fn rank_string_from_encoded(encoded: &str) -> Result<[Decoded; 255], FromHexError> {
    // Remove "0x" prefix if present and any whitespace
    let cleaned_hex: &str = encoded.trim().trim_start_matches("0x");

    // Convert hex string to raw bytes
    let bytes: Vec<u8> = hex::decode(cleaned_hex)?;

    // Had to use that 'from_fn' function because as decoded is a String it doesn't implement the Copy trait
    // which is needed to use the syntax: "[Decoded::new(...); 255]"
    let mut result: [Decoded; 255] = std::array::from_fn(|_| Decoded::new(' ', String::new(), 0));

    for key in 0u8..255 {
        let lenght: usize = bytes.len();
        let mut bytes_xor: Vec<u8> = vec![0; lenght];

        for i in 0..lenght {
            bytes_xor[i] = bytes[i] ^ key;
        }

        // The .into_owned() function is key because I need to borrow it and use it then
        let decoded: String = String::from_utf8_lossy(&bytes_xor).into_owned();
        let score: u32 = score_english_text(&decoded);
        let character: char = key as char;

        result[key as usize] = Decoded::new(character, decoded, score);
    }

    result.sort_by(|a, b| b.score.cmp(&a.score));

    Ok(result)
}
