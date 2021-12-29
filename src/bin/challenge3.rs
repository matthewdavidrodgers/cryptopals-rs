use cryptopals_rs::byte_buffer::{ByteBuffer, ByteBufferDisplayFormat};
use cryptopals_rs::cypher;
use cryptopals_rs::hex;

fn main() {
    let cyphertext = hex::decode(&ByteBuffer::from_ascii(
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
    ))
    .unwrap();

    let decode_details = cypher::decode_sb_xor(&cyphertext);
    let key = decode_details.key_buffer.data[0];

    println!("decoded using key {} ({})", key as char, key);
    println!("with score {}", decode_details.score);
    println!(
        "{}",
        decode_details
            .plaintext_buffer
            .to_string(ByteBufferDisplayFormat::String)
    );
}
