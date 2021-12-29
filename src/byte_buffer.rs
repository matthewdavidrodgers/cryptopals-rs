use std::fs;
use std::str;

#[derive(Clone)]
pub struct ByteBuffer {
    pub data: Vec<u8>,
}

#[derive(PartialEq, Eq)]
pub enum ByteBufferDisplayFormat {
    String,
    Decimal,
    Hex,
    Grid,
    GridAscii,
}

impl ByteBuffer {
    pub fn new() -> ByteBuffer {
        ByteBuffer { data: vec![] }
    }

    pub fn new_with_size(size: usize) -> ByteBuffer {
        ByteBuffer {
            data: vec![0; size],
        }
    }

    pub fn new_with_capacity(size: usize) -> ByteBuffer {
        ByteBuffer {
            data: Vec::with_capacity(size),
        }
    }

    pub fn from_ascii(ascii: &str) -> ByteBuffer {
        let mut buf = ByteBuffer::new_with_capacity(ascii.len());
        for c in ascii.bytes() {
            buf.data.push(c);
        }

        buf
    }

    pub fn from_file(filename: &str) -> ByteBuffer {
        let file_contents =
            fs::read_to_string(filename).expect("Could not read buffer contents from file");

        ByteBuffer::from_ascii(&file_contents)
    }

    pub fn xor_with(&mut self, other: &ByteBuffer) {
        let mut other_i = 0;
        for i in 0..self.data.len() {
            self.data[i] = self.data[i] ^ other.data[other_i];
            other_i = (other_i + 1) % other.data.len();
        }
    }

    pub fn to_string(&self, format: ByteBufferDisplayFormat) -> String {
        match &format {
            ByteBufferDisplayFormat::Grid | ByteBufferDisplayFormat::GridAscii => {
                let mut s = String::new();

                let mut block = 0;
                while block * 0x10 < self.data.len() {
                    let block_end = if (block + 1) * 0x10 < self.data.len() {
                        (block + 1) * 0x10
                    } else {
                        self.data.len()
                    };
                    s.push_str(&format!("{:07x}", block * 0x10));

                    for i in (block * 0x10)..block_end {
                        s.push_str(&format!(" {:02x}", self.data[i]));
                    }
                    if format == ByteBufferDisplayFormat::GridAscii {
                        s.push_str("\n       ");
                        for i in (block * 0x10)..block_end {
                            let val = self.data[i];
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

                for i in 0..self.data.len() {
                    let val = self.data[i];

                    if i > 0 {
                        s.push_str(&format!(",{}", val));
                    } else {
                        s.push_str(&format!("{}", val));
                    }
                }

                s.push_str(&format!("] (len {})", self.data.len()));
                s
            }
            ByteBufferDisplayFormat::Hex => {
                let mut s = String::from("[");

                for i in 0..self.data.len() {
                    let val = self.data[i];

                    if i > 0 {
                        s.push_str(&format!(",{:x}", val));
                    } else {
                        s.push_str(&format!("{:x}", val));
                    }
                }

                s.push_str(&format!("] (len {})", self.data.len()));
                s
            }
            ByteBufferDisplayFormat::String => {
                let char_pieces: Vec<_> = self
                    .data
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

pub fn xor(a: &ByteBuffer, b: &ByteBuffer) -> ByteBuffer {
    let mut result = a.clone();
    result.xor_with(b);

    result
}
