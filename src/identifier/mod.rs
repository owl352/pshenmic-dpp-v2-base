use crate::enums::encoding::EncodingBind;
use crate::errors::identifier::IdentifierError;
use crate::errors::utils::DecodeStringError;
use dpp::prelude::Identifier;

#[derive(Clone, Debug)]
pub struct IdentifierBind(Identifier);

impl From<Identifier> for IdentifierBind {
    fn from(identifier: Identifier) -> Self {
        Self(identifier)
    }
}

impl Into<Identifier> for IdentifierBind {
    fn into(self) -> Identifier {
        self.0
    }
}

impl IdentifierBind {
    pub fn from_string(str: String, encoding: EncodingBind) -> Result<Self, IdentifierError> {
        Ok(IdentifierBind(
            Identifier::from_string(str.as_str(), encoding.into()).map_err(|err| {
                IdentifierError::from(DecodeStringError::CannotDecodeString(err.to_string()))
            })?,
        ))
    }

    pub fn from_vec(bytes: Vec<u8>) -> Result<Self, IdentifierError> {
        Ok(IdentifierBind(
            Identifier::from_bytes(bytes.as_slice())
                .map_err(|err| IdentifierError::IdentifierFromBytesFailed(err.to_string()))?,
        ))
    }

    pub fn to_string(&self, encoding_bind: EncodingBind) -> String {
        self.0.to_string(encoding_bind.into())
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.as_bytes().clone()
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}
