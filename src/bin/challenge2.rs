use cryptopals_rs::byte_buffer;
use cryptopals_rs::byte_buffer::{ByteBuffer, ByteBufferDisplayFormat};
use cryptopals_rs::hex;

fn main() {
    let buffer_a =
        hex::decode(&("1c0111001f010100061a024b53535009181c".as_bytes().to_vec())).unwrap();
    let buffer_b =
        hex::decode(&("686974207468652062756c6c277320657965".as_bytes().to_vec())).unwrap();

    let mut result = byte_buffer::xor(&buffer_a, &buffer_b);
    result = hex::encode(&result);

    let expected = "746865206b696420646f6e277420706c6179";
    let got = result.to_string(ByteBufferDisplayFormat::String);

    println!("expected:\t{}", expected);
    println!("got:\t\t{}", got);
    assert_eq!(expected, &got);
}
