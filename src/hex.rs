use crate::byte_buffer::ByteBuffer;
use crate::utils::{DecodeError, DecodeType};

const HEX_CHAR_INDEXES: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

pub fn decode_in_place(buffer: &mut ByteBuffer) -> Result<(), DecodeError> {
    let byte_align_offset = buffer.data.len() % 2;

    for i in 0..buffer.data.len() {
        let is_upper_nibble = (i + byte_align_offset) % 2 == 0;
        let char_index = HEX_CHAR_INDEXES
            .iter()
            .position(|c| *c == buffer.data[i] as char);
        match char_index {
            Some(char_index) => {
                let byte_index = (i + byte_align_offset) / 2;
                if is_upper_nibble {
                    buffer.data[byte_index] = (char_index << 4) as u8;
                } else {
                    buffer.data[byte_index] |= char_index as u8;
                }
            }
            None => {
                return Err(DecodeError::new(
                    DecodeType::Hex,
                    "buffer contains invalid hex characters",
                ))
            }
        }
    }

    buffer
        .data
        .truncate((buffer.data.len() / 2) + byte_align_offset);

    Ok(())
}

pub fn decode(buffer: &ByteBuffer) -> Result<ByteBuffer, DecodeError> {
    let mut decoded = buffer.clone();

    decode_in_place(&mut decoded)?;

    Ok(decoded)
}

pub fn encode(buffer: &ByteBuffer) -> ByteBuffer {
    let mut encoded = ByteBuffer::new_with_size(buffer.data.len() * 2);

    for (i, byte) in buffer.data.iter().enumerate() {
        encoded.data[i * 2] = HEX_CHAR_INDEXES[(byte >> 4) as usize] as u8;
        encoded.data[(i * 2) + 1] = HEX_CHAR_INDEXES[(byte & 0x0F) as usize] as u8;
    }

    encoded
}
