use crate::errors::utils::DecodeStringError;
use std::fmt;

#[derive(Debug)]
pub enum IdentityError {
    CannotCreateBasicIdentityWithIdentifier(String),
    IdentitySerializationFailed(String),
    IdentityDeserializationFailed(String),
    IdentityFromStringFailed(String),
}

impl From<DecodeStringError> for IdentityError {
    fn from(error: DecodeStringError) -> Self {
        Self::IdentityFromStringFailed(error.to_string())
    }
}

impl fmt::Display for IdentityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
