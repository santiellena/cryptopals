use hex::FromHexError;
use std::fs;

#[path = "./challenge3.rs"]
pub mod challenge3;

/// Cool way to support receiving multiple errors is creating
/// a custom error that supports every one possible by
/// implementing the From function for each error
#[allow(dead_code)]
pub enum Error {
    HexError(FromHexError),
    IoError(std::io::Error),
}

impl From<hex::FromHexError> for Error {
    fn from(err: hex::FromHexError) -> Self {
        Error::HexError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

#[allow(dead_code)]
pub fn detect_single_character_xor() -> Result<challenge3::Decoded, Error> {
    let text: String = fs::read_to_string("src/set1/challenge4.txt")?;

    let lines: Vec<&str> = text.lines().collect();

    let mut result: challenge3::Decoded = challenge3::Decoded::new(' ', String::new(), 0);

    for line in lines {
        let ranks: [challenge3::Decoded; 255] = challenge3::rank_string_from_encoded(line)?;

        if result.score < ranks[0].score {
            result = ranks[0].clone();
        }
    }

    Ok(result)
}
