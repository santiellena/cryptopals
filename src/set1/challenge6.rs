use base64::{engine::general_purpose, Engine as _};
use hex::FromHexError;
use std::fs;

#[path = "./challenge3.rs"]
mod challenge3;
use challenge3::Decoded;

#[path = "./challenge5.rs"]
mod challenge5;

/// Cool way to support receiving multiple errors is creating
/// a custom error that supports every one possible by
/// implementing the From function for each error
#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    HexErr(FromHexError),
    IoErr(std::io::Error),
    DecodeErr(base64::DecodeError),
}

impl From<hex::FromHexError> for Error {
    fn from(err: hex::FromHexError) -> Self {
        Error::HexErr(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoErr(err)
    }
}

impl From<base64::DecodeError> for Error {
    fn from(err: base64::DecodeError) -> Self {
        Error::DecodeErr(err)
    }
}

// Step 2
pub fn hamming_distance(mut bytes_x: Vec<u8>, mut bytes_y: Vec<u8>) -> u32 {
    let bytes_x_lenght: usize = bytes_x.len();
    let bytes_y_lenght: usize = bytes_y.len();

    let lenght: usize = if bytes_x_lenght > bytes_y_lenght {
        let add: usize = bytes_x_lenght - bytes_y_lenght;

        for _ in 0..add {
            bytes_y.push(0);
        }

        bytes_x_lenght
    } else {
        let add: usize = bytes_y_lenght - bytes_x_lenght;

        for _ in 0..add {
            bytes_x.push(0);
        }

        bytes_y_lenght
    };

    let mut distance: u32 = 0;

    for i in 0..lenght {
        let xi: u8 = bytes_x[i];
        let yi: u8 = bytes_y[i];

        for i in 0_u8..8 {
            let bit_x: u8 = (xi >> i) & 1;
            let bit_y: u8 = (yi >> i) & 1;

            if bit_x != bit_y {
                distance += 1
            }
        }
    }

    distance
}

/// To be honest, when doing this function I had no idea why I was doing things, I was just following the steps.
/// Then, by the end of the excercise I got what I was doing. I read some theory about it just to confirm what I was thinking
/// was right.
///
/// The part that was the hardest for me was calculating the smallest average hamming distance. To be honest, I didn't understand
/// what the tutorial meant with comparing more than two consecutive blocks.
pub fn break_repeating_key() -> Result<String, Error> {
    let data: String = fs::read_to_string("src/set1/challenge6.txt")?;
    let data_parsed: String = data.replace("\n", "");
    let base64_output: Vec<u8> = general_purpose::STANDARD.decode(&data_parsed)?;

    // Step 1: Collect keysize candidates with their normalized Hamming distances
    let mut keysizes: Vec<(u32, f32)> = Vec::new();

    for i in 2_usize..=40 {
        // Take four blocks of length i
        let mut blocks: Vec<Vec<u8>> = Vec::new();
        for block_idx in 0..4 {
            let mut block: Vec<u8> = Vec::new();
            let start = block_idx * i;
            let end = (block_idx + 1) * i;
            for j in start..end {
                if j < base64_output.len() {
                    block.push(base64_output[j]);
                } else {
                    block.push(0); // Pad with zeros if needed
                }
            }
            blocks.push(block);
        }

        // Compute Hamming distances between consecutive pairs
        let mut distances: Vec<u32> = Vec::new();
        for pair in 0..3 {
            // Pairs: (0,1), (1,2), (2,3)
            let hamming_distance = hamming_distance(blocks[pair].clone(), blocks[pair + 1].clone());
            distances.push(hamming_distance);
        }

        // Average the distances and normalize
        let average_distance: f32 = (distances.iter().sum::<u32>()) as f32 / distances.len() as f32;
        let normalized: f32 = average_distance / i as f32;

        keysizes.push((i as u32, normalized));
    }

    // Step 4: Sort by normalized distance and take the top 3 KEYSIZE values
    keysizes.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
    let top_keysizes: Vec<(u32, f32)> = keysizes.into_iter().take(3).collect();

    // For now, proceed with the best KEYSIZE (smallest normalized distance)
    let (best_keysize, _) = top_keysizes[2]; // Take the third KEYSIZE (longest size with low hamming distance)

    // Step 5
    let bytes_amount: usize = base64_output.len();
    let mut num_blocks: usize = bytes_amount / best_keysize as usize;
    if bytes_amount % best_keysize as usize != 0 {
        num_blocks += 1;
    }

    let mut blocks: Vec<Vec<u8>> = Vec::new();
    for i in 0..num_blocks {
        let mut block: Vec<u8> = Vec::new();
        let init: usize = i * (best_keysize as usize);
        for j in init..(init + (best_keysize as usize)) {
            if j < bytes_amount {
                block.push(base64_output[j]);
            } else {
                block.push(0);
            }
        }
        blocks.push(block);
    }

    // Step 6
    let mut transposed_blocks: Vec<Vec<u8>> = Vec::new();

    for i in 0..(best_keysize as usize) {
        let mut transposed_block: Vec<u8> = Vec::new();
        for block in &blocks {
            transposed_block.push(block[i]);
        }
        transposed_blocks.push(transposed_block);
    }

    // Step 7
    let mut results: Vec<Decoded> = Vec::new();
    for bytes in transposed_blocks {
        let mut result: [Decoded; 255] =
            std::array::from_fn(|_| Decoded::new(' ', String::new(), 0));

        for key in 0u8..255 {
            let lenght: usize = bytes.len();
            let mut bytes_xor: Vec<u8> = vec![0; lenght];

            for i in 0..lenght {
                bytes_xor[i] = bytes[i] ^ key;
            }

            // The .into_owned() function is key because I need to borrow it and use it then
            let decoded: String = String::from_utf8_lossy(&bytes_xor).into_owned();
            let score: u32 = challenge3::score_english_text(&decoded);
            let character: char = key as char;

            result[key as usize] = Decoded::new(character, decoded, score);
        }

        result.sort_by(|a, b| b.score.cmp(&a.score));

        results.push(result[0].clone());
    }

    let mut key: String = String::new();
    for decoded in results {
        key.push_str(&decoded.character.to_string());
    }

    println!("Key: {}", key);

    let key_lenght: usize = key.len();
    let key_bytes: &[u8] = key.as_bytes();

    let mut result: Vec<u8> = Vec::new();
    for i in 0..(base64_output.len()) {
        result.push(base64_output[i] ^ key_bytes[(i + key_lenght) % key_lenght])
    }

    Ok(String::from_utf8_lossy(&result).into_owned())
}
