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
        Err(err) => panic!("{:?}", err),
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
        Err(err) => panic!("{:?}", err),
    };

    println!("Hex 1: {:?}", hex1);
    println!("Hex 2: {:?}", hex2);
    println!("XOR Hex: {:?}", xor);

    println!();

    println!("Challenge 3:");
    let hex3: &str = "0x1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let ranks: [challenges::three::Decoded; 255] =
        match challenges::three::rank_string_from_encoded(hex3) {
            Ok(result) => result,
            Err(err) => panic!("{:?}", err),
        };

    // Just show the top 10
    for i in 0..10 {
        println!("{}. {:?}", i + 1, ranks[i]);
    }

    println!();

    println!("Challenge 4:");

    println!("Looking for the encrypted hex in file './challenges4.txt'...");

    let encrypted: challenges::four::challenge3::Decoded =
        match challenges::four::detect_single_character_xor() {
            Ok(result) => result,
            Err(err) => panic!("{:?}", err),
        };

    println!("Result: {:?}", encrypted);

    println!();

    println!("Challenge 5:");
    let key: &str = "ICE";
    let data: &str = "Burning 'em, if you ain't quick and nimble I go crazy when I hear a cymbal";
    println!("Data: {:?}", data);
    println!("Key: {:?}", key);

    let result5: String = challenges::five::encrypt_repeating_key(key, data);
    println!("Encrypted data: {:?}", result5);

    println!();

    println!("Challenge 6:");

    let text1: String = String::from("this is a test");
    let text2: String = String::from("wokka wokka!!!");

    println!("Text 1: {:?}", text1);
    println!("Text 2: {:?}", text2);

    let hamming_distance: u32 =
        challenges::six::hamming_distance(text1.into_bytes(), text2.into_bytes());
    println!("Hamming Distance: {:?}", hamming_distance);

    println!();

    let result6: String = match challenges::six::break_repeating_key() {
        Ok(result) => result[0..150].to_string(),
        Err(err) => panic!("{:?}", err),
    };

    println!("Decrypted data(only first 150 characters): \n{}", result6);

    println!();

    println!("Challenge 7:");

    let key_vec: Vec<u8> = String::from("YELLOW SUBMARINE").into_bytes();
    let key: [u8; 16] = [
        key_vec[0],
        key_vec[1],
        key_vec[2],
        key_vec[3],
        key_vec[4],
        key_vec[5],
        key_vec[6],
        key_vec[7],
        key_vec[8],
        key_vec[9],
        key_vec[10],
        key_vec[11],
        key_vec[12],
        key_vec[13],
        key_vec[14],
        key_vec[15],
    ];

    println!("Original KEY: {:?}", key);

    let keys: [[[u8; 4]; 4]; 11] = challenges::seven::key_expansion(key);

    println!("ALL ROUND KEYS:");
    let mut counter: u32 = 0;
    for round in keys {
        println!("Round {}:", counter);
        for word in round {
            println!("     {:?}", word);
        }
        counter += 1;
    }

    println!();
    println!("end{}", "-".repeat(100));
}
