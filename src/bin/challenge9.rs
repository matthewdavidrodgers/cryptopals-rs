use cryptopals_rs::byte_buffer::{ByteBuffer, ByteBufferDisplayFormat};

fn main() {
    let mut buffer = "YELLOW SUBMARINE".as_bytes().to_vec();
    assert_eq!(buffer.len(), 16);

    buffer.pad_for_blocksize(20);

    assert_eq!(buffer.len(), 20);
    assert_eq!(&buffer[16..], &vec![4, 4, 4, 4]);

    println!("{}", buffer.to_string(ByteBufferDisplayFormat::Grid));
}
