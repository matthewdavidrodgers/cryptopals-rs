enum PercentDecodeConsumeState {
    Passthrough,
    EncodeByte(Option<u8>),
}

fn byte_is_hex_char(byte: u8) -> Option<u8> {
    match byte {
        0x30..=0x39 => Some(byte - 0x30),
        0x41..0x46 => Some(10 + byte - 0x41),
        0x61..0x66 => Some(10 + byte - 0x61),
        _ => None
    }
}

pub fn decode(buffer: &Vec<u8>) -> Vec<u8> {
    use PercentDecodeConsumeState::*;

    let decoded = Vec::new();

    let mut state = Passthrough;

    for byte in buffer {
        match state {
            Passthrough => {
                if byte == 0x25 {
                    state = EncodeByte(None, None);
                } else {
                    state.push(byte);
                }
            },
            EncodeByte(None) => {
                if let Some(upper) = byte_is_hex_char(byte) {
                    state = EncodeByte(Some(upper));
                } else {
                    decoded.push(0x25);
                    decoded.push(byte);
                    state = Passthrough;
                }
            },
            EncodeByte(Some(upper)) => {
                if let Some(lower) = byte_is_hex_char(byte) {
                    let decoded_byte = (upper << 4) | lower;
                    decoded.push(decoded_byte);
                    state = Passthrough;
                } else {
                    decoded.push(0x25);
                    decoded.push(first);
                    decoded.push(byte);
                    state = Passthrough;
                }
            },
        }
    }
}
