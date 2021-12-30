use cryptopals_rs::base64;
use cryptopals_rs::byte_buffer::{ByteBuffer, ByteBufferDisplayFormat};

use openssl::symm::{Cipher, Crypter, Mode};

fn main() {
    let mut file_contents = ByteBuffer::from_file("./src/bin/challenge7.txt");
    file_contents.remove_all(b'\n');
    let mut cyphertext = base64::decode(&file_contents).unwrap();

    let block_size = Cipher::aes_128_ecb().block_size();
    if cyphertext.data.len() % block_size != 0 {
        cyphertext.data.append(&mut vec![
            0u8;
            block_size - (cyphertext.data.len() % block_size)
        ]);
    }
    let mut plaintext = ByteBuffer::new_with_size(cyphertext.data.len() + block_size);

    let key = ByteBuffer::from_ascii("YELLOW SUBMARINE");

    let mut decrypter =
        Crypter::new(Cipher::aes_128_ecb(), Mode::Decrypt, &key.data, None).unwrap();

    let mut written = decrypter
        .update(&cyphertext.data, &mut plaintext.data)
        .unwrap();
    written += decrypter.finalize(&mut plaintext.data[written..]).unwrap();
    plaintext.data.truncate(written);

    println!("{}", plaintext.to_string(ByteBufferDisplayFormat::String));
}
