use cryptopals_rs::byte_buffer::{ByteBuffer, ByteBufferDisplayFormat};
use cryptopals_rs::{base64, cypher};
use openssl::symm::Mode;
use std::fs;

fn main() {
    let mut file_contents = fs::read("./src/bin/challenge10.txt").unwrap();
    file_contents.retain(|byte| *byte != b'\n');
    let cyphertext = base64::decode(&file_contents).unwrap();

    let key = "YELLOW SUBMARINE".as_bytes().to_vec();
    let iv = vec![0u8; 16];

    let plaintext = cypher::aes_cbc(&cyphertext, &key, &iv, Mode::Decrypt);

    println!("{}", plaintext.to_string(ByteBufferDisplayFormat::String));
}
