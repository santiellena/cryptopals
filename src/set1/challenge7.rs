use base64::{engine::general_purpose, Engine as _};
use std::fs;

#[allow(dead_code)]
pub fn xor_words(bytes1: [u8; 4], bytes2: [u8; 4]) -> [u8; 4] {
    let mut bytes_xor: [u8; 4] = [0; 4];

    for i in 0..4 {
        bytes_xor[i] = bytes1[i] ^ bytes2[i];
    }
    bytes_xor
}

#[allow(dead_code)]
pub fn shift_word_1(word: [u8; 4]) -> [u8; 4] {
    [word[1], word[2], word[3], word[0]] // Rotate left
}

/// AES S-box lookup table (256 bytes).
/// Maps each input byte (index) to its substituted output byte.
#[allow(dead_code)]
pub const SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

// Apply S-box substitution to a single byte
#[allow(dead_code)]
pub fn sub_byte(byte: u8) -> u8 {
    SBOX[byte as usize]
}

// SubWord for a 4-byte word
#[allow(dead_code)]
pub fn sub_word(word: [u8; 4]) -> [u8; 4] {
    [
        sub_byte(word[0]),
        sub_byte(word[1]),
        sub_byte(word[2]),
        sub_byte(word[3]),
    ]
}

#[allow(dead_code)]
pub const RCON: [u8; 11] = [
    0, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36,
];
pub fn rcon(index: u8, nk: u8, word: [u8; 4]) -> [u8; 4] {
    let power: u8 = index / nk;
    let first_byte: u8 = RCON[power as usize];

    let rcon_word: [u8; 4] = [first_byte, 0, 0, 0];

    xor_words(word, rcon_word)
}

/// AES KEY EXPANSION
/// Given a KEY, we will return all 11 rounds,
/// where each round is composed of 4 rows of 4 bytes
/// 16 Bytes per round and 176 Bytes in total
#[allow(dead_code)]
pub fn key_expansion(bytes_key: [u8; 16]) -> [[[u8; 4]; 4]; 11] {
    // Let's make 44 words of 4 bytes each
    let mut words: [[u8; 4]; 44] = [[0; 4]; 44];

    // 1) INITIAL SETUP: first 4 words are the original key
    words[0] = [bytes_key[0], bytes_key[1], bytes_key[2], bytes_key[3]];
    words[1] = [bytes_key[4], bytes_key[5], bytes_key[6], bytes_key[7]];
    words[2] = [bytes_key[8], bytes_key[9], bytes_key[10], bytes_key[11]];
    words[3] = [bytes_key[12], bytes_key[13], bytes_key[14], bytes_key[15]];

    // and the left 40 words are the result of operations from the first 4 words
    for i in 4..44 {
        // 2) EXPANSION LOOP
        if i % 4 == 0 {
            let prev: [u8; 4] = words[i - 1];
            let rot: [u8; 4] = shift_word_1(prev);

            let sub: [u8; 4] = sub_word(rot);

            let rcon: [u8; 4] = rcon(i as u8, 4, sub);

            words[i] = xor_words(rcon, words[i - 4]);
        } else {
            words[i] = xor_words(words[i - 1], words[i - 4]);
        }
    }

    let mut keys: [[[u8; 4]; 4]; 11] = [[[0; 4]; 4]; 11];
    for round in 0..11 {
        for row in 0..4 {
            for col in 0..4 {
                keys[round][row][col] = words[round * 4 + col][row];
            }
        }
    }

    keys
}

/// It turns out that we need to multiply two bytes in GF(2^8)
/// with AES polynomial 0x11b (x^8 + x^4 + x^3 + x + 1).
///
/// When I developed the key_expansion function I didn't realize that
/// I needed that because all the operations were done with pre-computed
/// values from the AES standard. But now, for the MixColumns and InvMixColumns
/// part of AES, I need multiplication and addition in the GF(2^8).
///
/// But why??
/// We need operations with inversibility and the GF by definition provides us that.
///
/// When we compute the resulting column in the MixColumn step in AES, we essentially do:
/// [s0, s1, s2, s3] x M = [s0', s1', s2', s3'] (matrix multiplication)
///
/// Where M = [ 2  3  1  1 ]
///           [ 1  2  3  1 ]
///           [ 1  1  2  3 ]
///           [ 3  1  1  2 ],
/// and for us to be able to reverse the operation in the InvMixColumns step,
///
/// we need to be able to calculate:
///
/// [s0', s1', s2', s3'] x M^(-1) = [s0, s1, s2, s3]
///
/// That's why we need the GF(2^8), and not a field of just mod 256, of normal modular arithmetic.
/// the (mod 256) field has some elements that are not inversible.
#[allow(dead_code)]
/// Multiply a byte by a constant in GF(2^8)
/// Multiply a byte by a constant in GF(2^8)
fn gf_mul(mut a: u8, mut b: u8) -> u8 {
    let mut result = 0;

    for _ in 0..8 {
        if (b & 1) != 0 {
            result ^= a;
        }
        let high_bit = a & 0x80;
        a <<= 1;
        if high_bit != 0 {
            a ^= 0x1B; // AES polynomial reduction
        }
        b >>= 1;
    }

    result
}

#[allow(dead_code)]
pub fn xor_with_round(
    data: [[u8; 4]; 4],
    expanded_key: [[[u8; 4]; 4]; 11],
    round: usize,
) -> [[u8; 4]; 4] {
    let mut data_matrix: [[u8; 4]; 4] = data;
    for row in 0..4 {
        for col in 0..4 {
            data_matrix[row][col] ^= expanded_key[round][row][col];
        }
    }
    data_matrix
}

#[allow(dead_code)]
pub fn shift_word_2(word: [u8; 4]) -> [u8; 4] {
    shift_word_1(shift_word_1(word))
}

#[allow(dead_code)]
pub fn shift_word_3(word: [u8; 4]) -> [u8; 4] {
    shift_word_1(shift_word_2(word))
}

#[allow(dead_code)]
pub fn inv_shift_rows(data: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    [
        data[0],
        shift_word_3(data[1]),
        shift_word_2(data[2]),
        shift_word_1(data[3]),
    ]
}

/// AES Inverse S-box lookup table (256 bytes).
/// Maps each input byte (index) to its inverse substituted output byte for decryption.
#[allow(dead_code)]
pub const INVSBOX: [u8; 256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d,
];

// Function to apply inverse S-box substitution to a single byte
#[allow(dead_code)]
pub fn inv_sub_byte(byte: u8) -> u8 {
    INVSBOX[byte as usize]
}

// Function to apply inverse S-box to a 4-byte word (for convenience)
#[allow(dead_code)]
pub fn inv_sub_word(word: [u8; 4]) -> [u8; 4] {
    [
        inv_sub_byte(word[0]),
        inv_sub_byte(word[1]),
        inv_sub_byte(word[2]),
        inv_sub_byte(word[3]),
    ]
}

#[allow(dead_code)]
pub fn inv_sub_bytes(data: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    [
        inv_sub_word(data[0]),
        inv_sub_word(data[1]),
        inv_sub_word(data[2]),
        inv_sub_word(data[3]),
    ]
}

/// s0 = (14 × s0') XOR (11 × s1') XOR (13 × s2') XOR (9 × s3')
/// s1 = (9 × s0') XOR (14 × s1') XOR (11 × s2') XOR (13 × s3')
/// s2 = (13 × s0') XOR (9 × s1') XOR (11 × s2') XOR (14 × s3')
/// s3 = (11 × s0') XOR (13 × s1') XOR (9 × s2') XOR (14 × s3')
#[allow(dead_code)]
/// Performs the inverse MixColumns operation on a single column of 4 bytes.
fn inv_mix_column(column: [u8; 4]) -> [u8; 4] {
    [
        gf_mul(14, column[0])
            ^ gf_mul(11, column[1])
            ^ gf_mul(13, column[2])
            ^ gf_mul(9, column[3]),
        gf_mul(9, column[0])
            ^ gf_mul(14, column[1])
            ^ gf_mul(11, column[2])
            ^ gf_mul(13, column[3]),
        gf_mul(13, column[0])
            ^ gf_mul(9, column[1])
            ^ gf_mul(11, column[2])
            ^ gf_mul(14, column[3]),
        gf_mul(11, column[0])
            ^ gf_mul(13, column[1])
            ^ gf_mul(9, column[2])
            ^ gf_mul(14, column[3]),
    ]
}

/// Applies inverse MixColumns to an entire 4x4 AES state matrix.
fn inv_mix_columns(data: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut state: [[u8; 4]; 4] = data;
    for i in 0..4 {
        let column = [state[0][i], state[1][i], state[2][i], state[3][i]];
        let mixed = inv_mix_column(column);
        state[0][i] = mixed[0];
        state[1][i] = mixed[1];
        state[2][i] = mixed[2];
        state[3][i] = mixed[3];
    }
    state
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    IoErr(std::io::Error),
    DecodeErr(base64::DecodeError),
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

#[allow(dead_code)]
pub fn decrypt_128bits(key: [u8; 16], data: Vec<u8>) -> Vec<u8> {
    assert_eq!(data.len(), 16, "Block must be 16 bytes, got {}", data.len());
    let expanded_key: [[[u8; 4]; 4]; 11] = key_expansion(key);

    let mut data_matrix: [[u8; 4]; 4] = [
        [data[0], data[4], data[8], data[12]],
        [data[1], data[5], data[9], data[13]],
        [data[2], data[6], data[10], data[14]],
        [data[3], data[7], data[11], data[15]],
    ];

    // Step 1: XOR matrix with round 10 (last round of the expanded key, rounds goes 0-10)
    data_matrix = xor_with_round(data_matrix, expanded_key, 10);

    // Step 2: Final round reverse (different from next ones because here columns weren't mixed)
    // 2.1 InvShiftRows: Going back from the shifts made during encryption
    data_matrix = inv_shift_rows(data_matrix);

    // 2.2 InvSubBytes: Replacing each byte with its inverse S-box value
    data_matrix = inv_sub_bytes(data_matrix);

    // Step 3: Main Rounds (9 iterations, Rounds 9 to 1)
    for round in (1..=9).rev() {
        data_matrix = xor_with_round(data_matrix, expanded_key, round);
        data_matrix = inv_mix_columns(data_matrix);
        data_matrix = inv_shift_rows(data_matrix);
        data_matrix = inv_sub_bytes(data_matrix);
    }

    // Step 4: Final Key Addition (Reverse of Initial Round)
    data_matrix = xor_with_round(data_matrix, expanded_key, 0);

    // Flatten state to output (column-major order)
    let mut plaintext = Vec::with_capacity(16);
    for col in 0..4 {
        for row in 0..4 {
            plaintext.push(data_matrix[col][row]);
        }
    }

    plaintext
}

#[allow(dead_code)]
pub fn decrypt_base64_aes(key: [u8; 16]) -> Result<Vec<u8>, Error> {
    let data: String = fs::read_to_string("src/set1/challenge7.txt")?;
    let data_parsed: String = data.replace("\n", "");
    let decoded_base64: Vec<u8> = general_purpose::STANDARD.decode(&data_parsed)?;

    println!("First block raw: {:?}", &decoded_base64[0..16]); // Should be [16, 178, 194, ...]
    let blocks: Vec<Vec<u8>> = decoded_base64
        .chunks(16)
        .map(|chunk| chunk.to_vec())
        .collect();
    let mut result: Vec<u8> = Vec::new();
    for block in blocks {
        let block_result: Vec<u8> = decrypt_128bits(key, block);
        for elem in block_result {
            result.push(elem);
        }
    }
    Ok(result)
}
