use openssl::symm::Mode;
use cryptopals_rs::base64;
use cryptopals_rs::byte_buffer::{ByteBuffer, ByteBufferDisplayFormat};
use cryptopals_rs::cypher;

fn main() {
    let mut file_contents = ByteBuffer::from_file("./src/bin/challenge7.txt");
    file_contents.remove_all(b'\n');
    let cyphertext = base64::decode(&file_contents).unwrap();

    let key = ByteBuffer::from_ascii("YELLOW SUBMARINE");

    let plaintext = cypher::aes_ecb(&cyphertext, &key, Mode::Decrypt);

    println!("{}", plaintext.to_string(ByteBufferDisplayFormat::String));
}
