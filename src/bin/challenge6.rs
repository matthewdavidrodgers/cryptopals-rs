use cryptopals_rs::base64;
use cryptopals_rs::cypher;
use cryptopals_rs::{ByteBuffer, ByteBufferDisplayFormat};
use std::fs;

fn main() {
    let mut file_contents = fs::read("./src/bin/challenge6.txt").unwrap();
    file_contents.retain(|byte| *byte != b'\n');
    let cyphertext = base64::decode(&file_contents).unwrap();

    let details = cypher::decode_rk_xor(&cyphertext);

    println!("decoded using key");
    println!(
        "\"{}\"",
        details
            .key_buffer
            .to_string(ByteBufferDisplayFormat::String)
    );
    println!("to");
    println!(
        "\"{}\"",
        details
            .plaintext_buffer
            .to_string(ByteBufferDisplayFormat::String)
    );
}
