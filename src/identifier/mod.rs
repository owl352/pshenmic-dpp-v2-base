use dpp::platform_value::string_encoding::Encoding;
use dpp::prelude::Identifier;

#[derive(Clone)]
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
    pub fn from_hex(str: String) -> Result<Self, String> {
        Ok(IdentifierBind(
            Identifier::from_string(str.as_str(), Encoding::Hex).map_err(|err| err.to_string())?,
        ))
    }

    pub fn from_base58(str: String) -> Result<Self, String> {
        Ok(IdentifierBind(
            Identifier::from_string(str.as_str(), Encoding::Base58)
                .map_err(|err| err.to_string())?,
        ))
    }

    pub fn from_vec(bytes: Vec<u8>) -> Result<Self, String> {
        Ok(IdentifierBind(
            Identifier::from_bytes(bytes.as_slice()).map_err(|err| err.to_string())?,
        ))
    }
    
    
    pub fn to_hex(&self) -> String {
        self.0.to_string(Encoding::Hex)
    }

    pub fn to_base58(&self) -> String {
        self.0.to_string(Encoding::Base58)
    }
    
    pub fn to_bytes(&self) -> [u8; 32]{
        self.0.as_bytes().clone()
    }
    
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}
