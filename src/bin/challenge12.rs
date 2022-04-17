use cryptopals_rs::base64;
use cryptopals_rs::cypher;
use cryptopals_rs::byte_buffer::{ByteBuffer, ByteBufferDisplayFormat};

const UNKNOWN_CONTENT_ENCODED: &str = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK";

fn main() {
    let unknown_content = UNKNOWN_CONTENT_ENCODED
        .as_bytes()
        .to_vec();
    let unknown_content = base64::decode(&unknown_content).expect("invalid base64 content");

    let oracle = cypher::make_oracle(&unknown_content);
    let unknown_encrypted = oracle(&Vec::new());

    let mut decoded = Vec::new();

    for byte_index in 0..unknown_encrypted.len() {
        dbg!(byte_index);
        let target_block = byte_index as usize / 16;

        let prepend_with_len = 15 - ((byte_index) % 16);
        let prepend_with = vec![b'X'; prepend_with_len];
        let oracle_output = oracle(&prepend_with)[target_block * 16..(target_block+1)*16].to_vec();

        let decoded_chunk = if decoded.len() < 15 {
            [&vec![b'X'; 15 - decoded.len()][..], &decoded[..]].concat()
        } else {
            decoded[(decoded.len() - 15)..decoded.len()].to_vec()
        };

        let mut prepend_known = vec![0u8; 15];
        prepend_known.copy_from_slice(&decoded_chunk[..]);
        prepend_known.push(0);

        for byte in 0..=255 {
            prepend_known[15] = byte;
            let oracle_crafted_output = &oracle(&prepend_known)[..16];
            if oracle_crafted_output.to_vec() == oracle_output {
                decoded.push(byte);
                break;
            }
        }
    }

    println!("{}", decoded.to_string(ByteBufferDisplayFormat::String));
}

