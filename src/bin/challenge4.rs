use std::fs;
use cryptopals_rs::{ByteBuffer, ByteBufferDisplayFormat};
use cryptopals_rs::cypher;
use cryptopals_rs::cypher::DecodeDetails;
use cryptopals_rs::hex;

fn main() {
	let contents = fs::read_to_string("./src/bin/challenge4.txt").unwrap();

	let mut best_line_details: Option<(DecodeDetails, ByteBuffer)> = None;

	for line in contents.lines() {
		let cyphertext = hex::decode(&ByteBuffer::from_ascii(line)).unwrap();
		let line_details = cypher::decode_sb_xor(&cyphertext);

		if line_details.score >= 0.0 {
			if let Some((DecodeDetails { score, .. }, _)) = best_line_details {
				if line_details.score < score {
					best_line_details = Some((line_details, cyphertext.clone()));
				}
			} else {
				best_line_details = Some((line_details, cyphertext.clone()));
			}
		}
	}

	let (details, line) = best_line_details.unwrap();

	println!("decoded using key {} ({})", details.key_buffer.data[0] as char, details.key_buffer.data[0] as char);
	println!("score {}", details.score);
	println!("from\t{}", hex::encode(&line).to_string(ByteBufferDisplayFormat::String));
	println!("to\t{}", details.plaintext_buffer.to_string(ByteBufferDisplayFormat::String));
}