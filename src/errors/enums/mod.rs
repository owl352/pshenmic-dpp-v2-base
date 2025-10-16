use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum EnumsError {
    CannotParseValueFromNumber,
}

impl fmt::Display for EnumsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
