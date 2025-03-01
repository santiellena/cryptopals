#[path = "challenges.rs"]
mod challenges;

fn main() {
    // Set 1
    println!("Set 1: Basics");
    println!("{}", "-".repeat(100));

    // Challenge 1
    println!("Challenge 1:");

    let hex: &str = "0x49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let base64: String = match challenges::one::hex_to_base64(hex) {
        Ok(result) => result,
        Err(_) => panic!(),
    };

    println!("Hex: {:?}", hex);
    println!("Base64: {:?}", base64);

    println!();

    // Challenge 2
    println!("Challenge 2:");
    let hex1: &str = "0x1c0111001f010100061a024b53535009181c";
    let hex2: &str = "0x686974207468652062756c6c277320657965";

    let xor: String = match challenges::two::hex_xor(hex1, hex2) {
        Ok(result) => result,
        Err(_) => panic!(),
    };

    println!("Hex 1: {:?}", hex1);
    println!("Hex 2: {:?}", hex2);
    println!("XOR Hex: {:?}", xor);

    println!();

    println!("Challenge 3:");

    println!();
    println!("end{}", "-".repeat(100));
}
