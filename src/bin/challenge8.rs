use cryptopals_rs::hex;
use cryptopals_rs::{ByteBuffer, ByteBufferDisplayFormat};
use std::fs;

const BLOCK_SIZE: usize = 16;

fn main() {
    let contents = fs::read_to_string("./src/bin/challenge8.txt").unwrap();

    let mut most_dupes: Option<(usize, ByteBuffer)> = None;

    for line in contents.lines() {
        let mut buf = ByteBuffer::from_ascii(line);
        hex::decode_in_place(&mut buf).unwrap();

        let mut dupe_blocks = 0;

        for x in 0..((buf.data.len() / BLOCK_SIZE) - 1) {
            for y in (x + 1)..(buf.data.len() / BLOCK_SIZE) {
                let block_a = &buf.data[(x * BLOCK_SIZE)..((x + 1) * BLOCK_SIZE)];
                let block_b = &buf.data[(y * BLOCK_SIZE)..((y + 1) * BLOCK_SIZE)];
                if block_a.iter().zip(block_b).all(|(a, b)| *a == *b) {
                    dupe_blocks += 1;
                }
            }
        }

        if let Some((most_dupe_count, _)) = most_dupes {
            if most_dupe_count < dupe_blocks {
                most_dupes = Some((dupe_blocks, buf.clone()));
            }
        } else {
            most_dupes = Some((dupe_blocks, buf.clone()));
        }
    }

    let (dupe_blocks, buf) = most_dupes.unwrap();

    println!("best buffer has {} duplicate blocks", dupe_blocks);
    println!("{}", buf.to_string(ByteBufferDisplayFormat::Grid))
}
