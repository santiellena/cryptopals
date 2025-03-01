#[path = "set1/challenge1.rs"]
mod challenge1;

fn main() {
    // Set 1
    // Challenge 1
    let hex: &str = "0x49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let base64: String = match challenge1::hex_to_base64(hex) {
        Ok(result) => result,
        Err(_) => panic!(),
    };
    println!("Hex: {:?} to Base64: {:?}", hex, base64);
}
