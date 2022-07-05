use cryptopals_rs::base64;
use cryptopals_rs::byte_buffer::{ByteBuffer, ByteBufferDisplayFormat};
use cryptopals_rs::cypher;
use openssl::symm::Mode;
use std::fs;

fn main() {
    let mut file_contents = fs::read("./src/bin/challenge7.txt").unwrap();
    file_contents.retain(|byte| *byte != b'\n');
    let cyphertext = base64::decode(&file_contents).unwrap();

    let key = "YELLOW SUBMARINE".as_bytes().to_vec();

    let plaintext = cypher::aes_ecb(&cyphertext, &key, Mode::Decrypt);

    println!("{}", plaintext.to_string(ByteBufferDisplayFormat::String));
}
