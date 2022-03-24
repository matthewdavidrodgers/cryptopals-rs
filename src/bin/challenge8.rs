use cryptopals_rs::hex;
use cryptopals_rs::{ByteBuffer, ByteBufferDisplayFormat};
use std::fs;

const BLOCK_SIZE: usize = 16;

fn main() {
    let contents = fs::read_to_string("./src/bin/challenge8.txt").unwrap();

    let mut most_dupes: Option<(usize, Vec<u8>)> = None;

    for line in contents.lines() {
        let mut buf = line.as_bytes().to_vec();
        hex::decode_in_place(&mut buf).unwrap();

        let dupe_blocks = buf.dupe_blocks(BLOCK_SIZE);

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
