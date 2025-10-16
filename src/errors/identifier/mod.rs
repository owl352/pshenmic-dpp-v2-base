use crate::errors::utils::DecodeStringError;
use std::fmt;

#[derive(Debug)]
pub enum IdentifierError {
    IdentifierFromStringFailed(String),
    IdentifierFromBytesFailed(String),
}

impl From<DecodeStringError> for IdentifierError {
    fn from(error: DecodeStringError) -> Self {
        Self::IdentifierFromStringFailed(error.to_string())
    }
}

impl fmt::Display for IdentifierError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
