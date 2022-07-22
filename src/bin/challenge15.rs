use cryptopals_rs::byte_buffer::ByteBuffer;

fn main() {
    let buffer = "ICE ICE BABY\x04\x04\x04\x04".as_bytes().to_vec();
    assert_eq!(buffer.is_padded_for_blocksize(16), Some(4));

    let buffer = "ICE ICE BABY\x05\x05\x05\x05".as_bytes().to_vec();
    assert_eq!(buffer.is_padded_for_blocksize(16), None);

    let buffer = "ICE ICE BABY\x01\x02\x03\x04".as_bytes().to_vec();
    assert_eq!(buffer.is_padded_for_blocksize(16), None);
}
