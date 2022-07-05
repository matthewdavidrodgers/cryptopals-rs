use rand::distributions;
use rand::prelude::*;

pub trait ByteBuffer {
    fn from_rand_bytes(num_bytes: usize) -> Self;
    fn is_padded_for_blocksize(&self, blocksize: usize) -> Option<usize>;
    fn pad_for_blocksize(&mut self, blocksize: usize);
    fn xor_with(&mut self, other: &Self);
    fn dupe_blocks(&self, blocksize: usize) -> usize;
    fn to_string(&self, format: ByteBufferDisplayFormat) -> String;
}

impl ByteBuffer for Vec<u8> {
    fn from_rand_bytes(num_bytes: usize) -> Vec<u8> {
        let rng = thread_rng();
        rng.sample_iter(distributions::Standard)
            .take(num_bytes)
            .collect()
    }

    fn pad_for_blocksize(&mut self, blocksize: usize) {
        if self.len() % blocksize != 0 {
            let pad_by = blocksize - (self.len() % blocksize);
            self.append(&mut vec![pad_by as u8; pad_by]);
        }
    }

    fn is_padded_for_blocksize(&self, blocksize: usize) -> Option<usize> {
        if self.len() == 0 || self.len() % blocksize != 0 {
            return None;
        }
        let padded_by = self[self.len() - 1];
        if (padded_by as usize) >= blocksize {
            return None;
        }
        if !self.ends_with(&vec![padded_by; padded_by as usize]) {
            return None;
        }
        Some(padded_by as usize)
    }

    fn xor_with(&mut self, other: &Vec<u8>) {
        let mut other_i = 0;
        for i in 0..self.len() {
            self[i] = self[i] ^ other[other_i];
            other_i = (other_i + 1) % other.len();
        }
    }

    fn dupe_blocks(&self, blocksize: usize) -> usize {
        let mut dupe_blocks = 0;

        for x in 0..((self.len() / blocksize) - 1) {
            for y in (x + 1)..(self.len() / blocksize) {
                let block_a = &self[(x * blocksize)..((x + 1) * blocksize)];
                let block_b = &self[(y * blocksize)..((y + 1) * blocksize)];
                if block_a.iter().zip(block_b).all(|(a, b)| *a == *b) {
                    dupe_blocks += 1;
                }
            }
        }

        dupe_blocks
    }

    fn to_string(&self, format: ByteBufferDisplayFormat) -> String {
        match &format {
            ByteBufferDisplayFormat::Grid | ByteBufferDisplayFormat::GridAscii => {
                let mut s = String::new();

                let mut block = 0;
                while block * 0x10 < self.len() {
                    let block_end = if (block + 1) * 0x10 < self.len() {
                        (block + 1) * 0x10
                    } else {
                        self.len()
                    };
                    s.push_str(&format!("{:07x}", block * 0x10));

                    for i in (block * 0x10)..block_end {
                        s.push_str(&format!(" {:02x}", self[i]));
                    }
                    if format == ByteBufferDisplayFormat::GridAscii {
                        s.push_str("\n       ");
                        for i in (block * 0x10)..block_end {
                            let val = self[i];
                            if val >= 32 && val <= 126 {
                                s.push_str(&format!("  {}", val as char));
                            } else if val == 10 {
                                s.push_str(" \\n");
                            } else {
                                s.push_str("  @");
                            }
                        }
                    }
                    s.push_str("\n");

                    block += 1;
                }

                s
            }
            ByteBufferDisplayFormat::Decimal => {
                let mut s = String::from("[");

                for i in 0..self.len() {
                    let val = self[i];

                    if i > 0 {
                        s.push_str(&format!(",{}", val));
                    } else {
                        s.push_str(&format!("{}", val));
                    }
                }

                s.push_str(&format!("] (len {})", self.len()));
                s
            }
            ByteBufferDisplayFormat::Hex => {
                let mut s = String::from("[");

                for i in 0..self.len() {
                    let val = self[i];

                    if i > 0 {
                        s.push_str(&format!(",{:x}", val));
                    } else {
                        s.push_str(&format!("{:x}", val));
                    }
                }

                s.push_str(&format!("] (len {})", self.len()));
                s
            }
            ByteBufferDisplayFormat::String => {
                let char_pieces: Vec<_> = self
                    .iter()
                    .map(|c| {
                        let mut utf8_buf = [0; 4];
                        let char_piece = String::from((*c as char).encode_utf8(&mut utf8_buf));
                        char_piece
                    })
                    .collect();

                char_pieces.join("")
            }
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum ByteBufferDisplayFormat {
    String,
    Decimal,
    Hex,
    Grid,
    GridAscii,
}

pub fn xor(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let mut result = a.clone();
    result.xor_with(b);

    result
}

pub fn distance(a: &Vec<u8>, b: &Vec<u8>) -> usize {
    let len = if a.len() > b.len() { b.len() } else { a.len() };

    let mut dist = 0usize;
    for i in 0..len {
        let mut xored_byte = a[i] & b[i];
        while xored_byte != 0 {
            dist += (xored_byte & 0x01) as usize;
            xored_byte >>= 1;
        }
    }

    dist
}
