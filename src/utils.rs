use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub enum DecodeType {
    Hex,
}

#[derive(Debug)]
pub struct DecodeError {
    decode_type: DecodeType,
    msg: String,
}

impl DecodeError {
    pub fn new(decode_type: DecodeType, msg: &str) -> DecodeError {
        DecodeError {
            decode_type,
            msg: String::from(msg),
        }
    }
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DecodeError decoding {:?}: {}",
            self.decode_type, &self.msg
        )
    }
}

impl Error for DecodeError {}
