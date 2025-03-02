#[allow(dead_code)]
pub fn encrypt_repeating_key(key: &str, data: &str) -> String {
    let data_bytes: &[u8] = data.as_bytes();
    let key_bytes: &[u8] = key.as_bytes();

    let key_lenght: usize = key_bytes.len();

    let mut result: Vec<u8> = Vec::new();
    for i in 0..(data_bytes.len()) {
        result.push(data_bytes[i] ^ key_bytes[(i + key_lenght) % key_lenght])
    }

    hex::encode(result)
}
