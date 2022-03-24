use crate::utils::{DecodeError, DecodeType};
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

pub fn encode(buffer: &Vec<u8>) -> Vec<u8> {
    use Base64DecodeState::*;

    let mut encoded_size = (buffer.len() / 3) * 4;
    if buffer.len() % 3 != 0 {
        encoded_size += 4;
    }

    let mut encoded = vec![0u8; encoded_size];

    let mut state = Base64DecodeState::A;

    let mut byte_index = 0;
    let mut encoded_index = 0;

    loop {
        let curr_byte = buffer[byte_index];
        let next_byte = if byte_index + 1 < buffer.len() {
            buffer[byte_index + 1]
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

        encoded[encoded_index] = BASE64_CHAR_INDEXES[char_index] as u8;
        encoded_index += 1;

        if matches!(
            (&state, byte_index.cmp(&buffer.len())),
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

    if buffer.len() % 3 == 2 {
        encoded[encoded_index] = b'=';
    } else if buffer.len() % 3 == 1 {
        encoded[encoded_index] = b'=';
        encoded[encoded_index + 1] = b'=';
    }

    encoded
}

fn base64_char_value(c: char) -> Result<u8, DecodeError> {
    if let Some(index) = BASE64_CHAR_INDEXES.iter().position(|cc| *cc == c) {
        Ok(index as u8)
    } else {
        Err(DecodeError::new(
            DecodeType::Base64,
            &format!("buffer contains invalid base64 characters: ({})", c),
        ))
    }
}

pub fn decode(buffer: &Vec<u8>) -> Result<Vec<u8>, DecodeError> {
    let mut decoded_len = (buffer.len() / 4) * 3;
    if buffer.len() > 0 && buffer[buffer.len() - 1] == b'=' {
        decoded_len -= 1;
    }
    if buffer.len() > 1 && buffer[buffer.len() - 2] == b'=' {
        decoded_len -= 1;
    }

    let mut decoded = vec![0u8; decoded_len];
    for i in 0..decoded_len {
        let char_offset = (i / 3) * 4;
        match i % 3 {
            0 => {
                let a = buffer[char_offset];
                let b = buffer[char_offset + 1];
                decoded[i] =
                    (base64_char_value(a as char)? << 2) | (base64_char_value(b as char)? >> 4);
            }
            1 => {
                let a = buffer[char_offset + 1];
                let b = buffer[char_offset + 2];
                decoded[i] = ((base64_char_value(a as char)? & 0x0F) << 4)
                    | (base64_char_value(b as char)? >> 2);
            }
            _ => {
                let a = buffer[char_offset + 2];
                let b = buffer[char_offset + 3];
                decoded[i] =
                    ((base64_char_value(a as char)? & 0x03) << 6) | base64_char_value(b as char)?;
            }
        }
    }

    Ok(decoded)
}
