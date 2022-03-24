use cryptopals_rs::base64;
use cryptopals_rs::byte_buffer::{ByteBuffer, ByteBufferDisplayFormat};
use cryptopals_rs::hex;

fn main() {
    let hex_encoded_buffer = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".as_bytes().to_vec();
    let decoded_buffer = hex::decode(&hex_encoded_buffer).unwrap();
    let base64_encoded_buffer = base64::encode(&decoded_buffer);

    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    let got = base64_encoded_buffer.to_string(ByteBufferDisplayFormat::String);

    println!("expected:\t{}", expected);
    println!("got:\t\t{}", got);
    assert_eq!(expected, &got);
}
