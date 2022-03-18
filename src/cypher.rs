use crate::byte_buffer;
use crate::byte_buffer::ByteBuffer;

use openssl::symm::{Cipher, Crypter, Mode};

const KEYSIZES_TAKEN: usize = 10;

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

#[derive(Clone)]
pub struct DecodeDetails {
    pub key_buffer: ByteBuffer,
    pub plaintext_buffer: ByteBuffer,
    pub score: f64,
}

#[derive(Debug)]
struct Keysize {
    keysize: usize,
    score: f64,
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

fn permutations(x: usize) -> usize {
    (((x - 1) * (x - 1)) + (x - 1)) / 2
}

fn pick_rk_xor_keysizes(buffer: &ByteBuffer) -> Vec<Keysize> {
    let mut keysizes: Vec<Keysize> = Vec::with_capacity(KEYSIZES_TAKEN);
    let max_keysize = if (buffer.data.len() / 2) < 40 {
        buffer.data.len() / 2
    } else {
        40
    };

    for keysize in 2..=max_keysize {
        let num_blocks = buffer.data.len() / keysize;
        let mut blocks: Vec<ByteBuffer> = Vec::with_capacity(num_blocks);

        for i in 0..num_blocks {
            blocks.push(buffer.slice(i * keysize, (i + 1) * keysize));
        }

        let mut block_dis = 0.0;
        for x in 0..(num_blocks - 1) {
            for y in x..num_blocks {
                block_dis +=
                    (byte_buffer::distance(&blocks[x], &blocks[y]) as f64) / (keysize as f64 * 8.0);
            }
        }
        block_dis /= permutations(num_blocks) as f64;

        let size = Keysize {
            keysize,
            score: block_dis,
        };

        let mut insert_at: Option<usize> = None;
        let mut index = 0;
        while index < keysizes.len() && keysizes[index].score > size.score {
            index += 1;
        }

        if keysizes.len() < KEYSIZES_TAKEN {
            insert_at = Some(index);
        } else if index < keysizes.len() {
            insert_at = Some(index);
        }

        if let Some(insert_at) = insert_at {
            keysizes.insert(insert_at, size);
        }
    }

    keysizes
}

fn break_and_transpose_blocks(buffer: &ByteBuffer, blocksize: usize) -> Vec<ByteBuffer> {
    let mut blocks = vec![];

    for x in 0..blocksize {
        let mut block = ByteBuffer::new();
        let mut y = 0;
        while (y + x) < buffer.data.len() {
            block.data.push(buffer.data[y + x]);
            y += blocksize;
        }
        blocks.push(block);
    }

    blocks
}

fn decode_rk_xor_for_size(buffer: &ByteBuffer, keysize: usize) -> DecodeDetails {
    let transposed_blocks = break_and_transpose_blocks(buffer, keysize);
    let block_details: Vec<_> = transposed_blocks
        .iter()
        .map(|block| decode_sb_xor(block))
        .collect();

    let mut key_buffer = ByteBuffer::new_with_capacity(keysize);
    for detail in block_details {
        key_buffer.data.push(detail.key_buffer.data[0]);
    }

    let plaintext_buffer = byte_buffer::xor(buffer, &key_buffer);
    let score = score_buffer_as_english(&plaintext_buffer);

    DecodeDetails {
        key_buffer,
        plaintext_buffer,
        score,
    }
}

pub fn decode_rk_xor(buffer: &ByteBuffer) -> DecodeDetails {
    let sizes = pick_rk_xor_keysizes(buffer);

    let mut best_result: Option<DecodeDetails> = None;

    for size in sizes {
        let result = decode_rk_xor_for_size(buffer, size.keysize);

        if result.score > 0.0 {
            if let Some(DecodeDetails { score, .. }) = best_result {
                if score > result.score {
                    best_result = Some(result.clone());
                }
            } else {
                best_result = Some(result.clone());
            }
        }
    }

    best_result.unwrap()
}

pub fn decode_aes_ecb(cyphertext: &ByteBuffer, key: &ByteBuffer) -> ByteBuffer {
    let block_size = Cipher::aes_128_ecb().block_size();

    let mut plaintext = ByteBuffer::new_with_size(cyphertext.data.len() + block_size);

    let mut decrypter = Crypter::new(Cipher::aes_128_ecb(), Mode::Decrypt, &key.data, None).unwrap();

    let mut written = decrypter
        .update(&cyphertext.data, &mut plaintext.data)
        .unwrap();
	written += decrypter.finalize(&mut plaintext.data[written..]).unwrap();
    plaintext.data.truncate(written);

    plaintext
}

fn aes_cbc_block(block: &ByteBuffer, prev_block: &ByteBuffer, key: &ByteBuffer, mode: Mode) -> ByteBuffer {
    let mut output = ByteBuffer::new_with_size(block.data.len() + Cipher::aes_128_ecb().block_size());
    let mut crypter = Crypter::new(Cipher::aes_128_ecb(), mode, &key.data, None).unwrap();
    crypter.pad(false);

    let written = crypter.update(&block.data, &mut output.data).unwrap();
    output.data.truncate(written);

    output.xor_with(prev_block);

    output
}

pub fn aes_cbc(input: &ByteBuffer, key: &ByteBuffer, iv: &ByteBuffer, mode: Mode) -> ByteBuffer {
    let block_size = Cipher::aes_128_ecb().block_size();

    let mut output = ByteBuffer::new_with_size(input.data.len());

    let mut prev_block = iv.clone();
    for chunk in input.data.chunks(block_size) {
        let block = ByteBuffer::from_slice(chunk);
        let output_block = aes_cbc_block(&block, &prev_block, key, mode);
        output.data = [output.data, output_block.data].concat();
        prev_block = block.clone();
    }

    output
}
