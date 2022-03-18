use openssl::symm::Mode;
use cryptopals_rs::{base64, cypher};
use cryptopals_rs::byte_buffer::{ByteBuffer, ByteBufferDisplayFormat};

fn main() {
    let mut file_contents = ByteBuffer::from_file("./src/bin/challenge10.txt");
    file_contents.remove_all(b'\n');
    let cyphertext = base64::decode(&file_contents).unwrap();

    let key = ByteBuffer::from_ascii("YELLOW SUBMARINE");
    let iv = ByteBuffer::from_slice(&[0u8; 16]);

    let plaintext = cypher::aes_cbc(&cyphertext, &key, &iv, Mode::Decrypt);

    println!("{}", plaintext.to_string(ByteBufferDisplayFormat::String));
}
