use cryptopals_rs::byte_buffer::{ByteBuffer, ByteBufferDisplayFormat};

fn main() {
	let mut buffer = ByteBuffer::from_ascii("YELLOW SUBMARINE");
	assert_eq!(buffer.data.len(), 16);

	buffer.pad_for_blocksize(20);

	assert_eq!(buffer.data.len(), 20);
	assert_eq!(&buffer.data[16..], &vec![4, 4, 4, 4]);

	println!("{}", buffer.to_string(ByteBufferDisplayFormat::Grid));
}