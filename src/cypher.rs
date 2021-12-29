use crate::byte_buffer;
use crate::byte_buffer::ByteBuffer;

const ENGLISH_AVG_CHAR_FREQUENCIES: [f64; 52] = [
    0.082389258,
    0.015051398,
    0.028065007,
    0.042904556,
    0.128138650,
    0.022476217,
    0.020327458,
    0.061476691,
    0.061476691,
    0.001543474,
    0.007787989,
    0.040604477,
    0.024271893,
    0.068084376,
    0.075731132,
    0.019459884,
    0.000958366,
    0.060397268,
    0.063827211,
    0.091357551,
    0.027822893,
    0.009866131,
    0.023807842,
    0.001513210,
    0.019913847,
    0.000746517,
    0.082389258 * 0.003,
    0.015051398 * 0.003,
    0.028065007 * 0.003,
    0.042904556 * 0.003,
    0.128138650 * 0.003,
    0.022476217 * 0.003,
    0.020327458 * 0.003,
    0.061476691 * 0.003,
    0.061476691 * 0.003,
    0.001543474 * 0.003,
    0.007787989 * 0.003,
    0.040604477 * 0.003,
    0.024271893 * 0.003,
    0.068084376 * 0.003,
    0.075731132 * 0.003,
    0.019459884 * 0.003,
    0.000958366 * 0.003,
    0.060397268 * 0.003,
    0.063827211 * 0.003,
    0.091357551 * 0.003,
    0.027822893 * 0.003,
    0.009866131 * 0.003,
    0.023807842 * 0.003,
    0.001513210 * 0.003,
    0.019913847 * 0.003,
    0.000746517 * 0.003,
];

pub struct DecodeDetails {
    pub key_buffer: ByteBuffer,
    pub plaintext_buffer: ByteBuffer,
    pub score: f64,
}

fn score_buffer_as_english(buffer: &ByteBuffer) -> f64 {
    let mut buffer_char_counts = [0; 52];

    let mut countable_chars = 0;

    for byte in &buffer.data {
        let char_index = match byte {
            b'A'..=b'Z' => {
                countable_chars += 1;
                Some(byte - (65 - 26))
            }
            b'a'..=b'z' => {
                countable_chars += 1;
                Some(byte - 97)
            }
            b' ' => {
                countable_chars += 1;
                None
            }
            _ => None,
        };

        if let Some(char_index) = char_index {
            buffer_char_counts[char_index as usize] += 1;
        }
    }

    if countable_chars == 0 {
        return -1.0;
    }

    let mut score = 0.0;

    for (i, count) in buffer_char_counts.iter().enumerate() {
        let freq = *count as f64 / countable_chars as f64;
        let char_score = (ENGLISH_AVG_CHAR_FREQUENCIES[i] - freq) / 2.0;
        score += char_score.abs();
    }

    if buffer.data.len() != countable_chars {
        score *= (buffer.data.len() - countable_chars) as f64;
    }

    score
}

pub fn decode_sb_xor(cyphertext: &ByteBuffer) -> DecodeDetails {
    let mut best_details: Option<DecodeDetails> = None;

    let mut key_buffer = ByteBuffer::new_with_size(1);
    for key in 0u8..=255 {
        key_buffer.data[0] = key;

        let decoded_buffer = byte_buffer::xor(cyphertext, &key_buffer);
        let current_score = score_buffer_as_english(&decoded_buffer);

        if current_score >= 0f64 {
            if let Some(DecodeDetails { score, .. }) = best_details {
                if score > current_score {
                    best_details = Some(DecodeDetails {
                        key_buffer: key_buffer.clone(),
                        plaintext_buffer: decoded_buffer.clone(),
                        score: current_score,
                    });
                }
            } else {
                best_details = Some(DecodeDetails {
                    key_buffer: key_buffer.clone(),
                    plaintext_buffer: decoded_buffer.clone(),
                    score: current_score,
                });
            }
        }
    }

    best_details.unwrap()
}
