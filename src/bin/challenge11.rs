use cryptopals_rs::cypher::{self, BlockMode};
use cryptopals_rs::byte_buffer::ByteBuffer;

fn main() {
    let buffer = ByteBuffer::from_ascii("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
    let (encrypted, block_mode) = cypher::encryption_oracle(&buffer);
    let expected_block_mode = match encrypted.dupe_blocks(16) {
        0 => BlockMode::CBC,
        _ => BlockMode::ECB,
    };

    assert_eq!(block_mode, expected_block_mode);
}
