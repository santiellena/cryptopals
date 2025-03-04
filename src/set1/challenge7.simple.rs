use base64::{engine::general_purpose, Engine as _};
use openssl::symm::{decrypt, Cipher};

use std::fs;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    IoErr(std::io::Error),
    DecodeErr(base64::DecodeError),
    OpenSsl(openssl::error::ErrorStack),
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

impl From<openssl::error::ErrorStack> for Error {
    fn from(err: openssl::error::ErrorStack) -> Self {
        Error::OpenSsl(err)
    }
}

pub fn decrypt_7() -> Result<String, Error> {
    // Read the base64-encoded ciphertext from a file
    let ciphertext_base64 = fs::read_to_string("src/set1/challenge7.txt")?;
    let data_parsed: String = ciphertext_base64.replace("\n", "");

    // The key from the challenge (must be 16 bytes for AES-128)
    let key = b"YELLOW SUBMARINE";

    // Decode base64 to get the raw ciphertext bytes
    let ciphertext = general_purpose::STANDARD
        .decode(data_parsed.trim())
        .expect("Failed to decode base64");

    // Set up AES-128-ECB cipher
    let cipher = Cipher::aes_128_ecb();

    // Decrypt the ciphertext
    let plaintext = decrypt(
        cipher,
        key,
        None, // No IV needed for ECB mode
        &ciphertext,
    )?;

    // Convert plaintext bytes to string and print
    let plaintext_str =
        String::from_utf8(plaintext).expect("Failed to convert plaintext to string");

    Ok(plaintext_str)
}
