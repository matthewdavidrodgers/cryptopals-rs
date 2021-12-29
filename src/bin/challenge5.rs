use cryptopals_rs::byte_buffer;
use cryptopals_rs::hex;
use cryptopals_rs::{ByteBuffer, ByteBufferDisplayFormat};

fn main() {
    let plaintext = ByteBuffer::from_ascii(
        "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal",
    );
    let key = ByteBuffer::from_ascii("ICE");

    let result = byte_buffer::xor(&plaintext, &key);
    let result = hex::encode(&result);

    let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
    let got = result.to_string(ByteBufferDisplayFormat::String);

    println!("expected\t{}", expected);
    println!("got\t\t{}", got);
    assert_eq!(expected, &got);
}
