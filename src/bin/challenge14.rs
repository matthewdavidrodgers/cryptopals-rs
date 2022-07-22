use cryptopals_rs::base64;
use cryptopals_rs::byte_buffer::{ByteBuffer, ByteBufferDisplayFormat};
use cryptopals_rs::cypher;
use cryptopals_rs::cypher::OracleMode;

const UNKNOWN_CONTENT_ENCODED: &str = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK";

fn main() {
    let unknown_content = UNKNOWN_CONTENT_ENCODED.as_bytes().to_vec();
    let unknown_content = base64::decode(&unknown_content).expect("invalid base64 content");

    let oracle = cypher::make_oracle(&unknown_content, OracleMode::Prefixing);
    let unknown_encrypted = oracle(&Vec::new());

    let mut prefix_len: Option<usize> = None;
    for vector_len in 33..48 {
        let attack_vec = vec![b'X'; vector_len];
        let oracle_output = oracle(&attack_vec);
        if oracle_output.dupe_blocks(16) > 0 {
            prefix_len = Some(48 - vector_len);
            break;
        }
    }

    let prefix_len = prefix_len.expect("couldn't deduce prefix len");

    let mut decoded = Vec::new();

    for byte_index in 0..unknown_encrypted.len() - prefix_len {
        let target_block = (byte_index as usize / 16) + 1;

        let prepend_with_len = 15 - (byte_index % 16) + (16 - prefix_len);
        let prepend_with = vec![b'X'; prepend_with_len];
        let oracle_output =
            oracle(&prepend_with)[target_block * 16..(target_block + 1) * 16].to_vec();

        let decoded_bytes = decoded.len();
        let decoded_chunk = if decoded_bytes < 15 {
            [&vec![b'X'; 15 - decoded_bytes][..], &decoded[..]].concat()
        } else {
            decoded[(decoded_bytes - 15)..decoded_bytes].to_vec()
        };

        let mut prepend_known = [&vec![b'X'; 16 - prefix_len][..], &decoded_chunk[..]].concat();
        prepend_known.push(0);
        let prepend_known_len = prepend_known.len();

        for byte in 0..=255 {
            prepend_known[prepend_known_len - 1] = byte;
            let oracle_crafted_output = &oracle(&prepend_known)[16..32];
            if oracle_crafted_output.to_vec() == oracle_output {
                decoded.push(byte);
                break;
            }
        }
    }

    println!("{}", decoded.to_string(ByteBufferDisplayFormat::String));
}
