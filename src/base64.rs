use crate::byte_buffer::ByteBuffer;
use std::cmp::Ordering;

const BASE64_CHAR_INDEXES: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

enum Base64DecodeState {
    A,
    B,
    C,
    D,
}

impl Base64DecodeState {
    fn next(self) -> Base64DecodeState {
        match self {
            Self::A => Self::B,
            Self::B => Self::C,
            Self::C => Self::D,
            Self::D => Self::A,
        }
    }
}

pub fn encode(buffer: &ByteBuffer) -> ByteBuffer {
    use Base64DecodeState::*;

    let mut encoded_size = (buffer.data.len() / 3) * 4;
    if buffer.data.len() % 3 != 0 {
        encoded_size += 4;
    }

    let mut encoded = ByteBuffer::new_with_size(encoded_size);

    let mut state = Base64DecodeState::A;

    let mut byte_index = 0;
    let mut encoded_index = 0;

    loop {
        let curr_byte = buffer.data[byte_index];
        let next_byte = if byte_index + 1 < buffer.data.len() {
            buffer.data[byte_index + 1]
        } else {
            0
        };

        let char_index = match state {
            A => (curr_byte >> 2) as usize,
            B => {
                byte_index += 1;
                (((curr_byte & 0x03) << 4) | (next_byte >> 4)) as usize
            }
            C => {
                byte_index += 1;
                (((curr_byte & 0x0f) << 2) | (next_byte >> 6)) as usize
            }
            D => {
                byte_index += 1;
                (curr_byte & 0x3f) as usize
            }
        };

        encoded.data[encoded_index] = BASE64_CHAR_INDEXES[char_index] as u8;
        encoded_index += 1;

        if matches!(
            (&state, byte_index.cmp(&buffer.data.len())),
            (B | C | D, Ordering::Equal)
        ) {
            break;
        }

        state = state.next();

        // if buffer.data.len() % 3 == 2 {
        // 	encoded.data[encoded_index] = b'=';
        // 	encoded_index += 1;
        // } else if buffer.data.len() % 3 == 1 {
        // 	encoded.data[encoded_index] = b'=';
        // 	encoded_index += 1;
        // 	encoded.data[encoded_index] = b'=';
        // 	encoded_index += 1;
        // }
    }

    if buffer.data.len() % 3 == 2 {
        encoded.data[encoded_index] = b'=';
    } else if buffer.data.len() % 3 == 1 {
        encoded.data[encoded_index] = b'=';
        encoded.data[encoded_index + 1] = b'=';
    }

    encoded
}
