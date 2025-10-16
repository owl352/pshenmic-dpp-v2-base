use std::fmt;

#[derive(Debug)]
pub enum DecodeStringError {
    CannotDecodeString(String),
}

impl fmt::Display for DecodeStringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
