use dpp::platform_value::string_encoding::Encoding;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum EncodingBind {
    Base58,
    Base64,
    Hex,
}

impl From<Encoding> for EncodingBind {
    fn from(encoding: Encoding) -> Self {
        match encoding {
            Encoding::Base58 => EncodingBind::Base58,
            Encoding::Base64 => EncodingBind::Base64,
            Encoding::Hex => EncodingBind::Base64,
        }
    }
}

impl From<EncodingBind> for Encoding {
    fn from(encoding: EncodingBind) -> Self {
        match encoding {
            EncodingBind::Base58 => Encoding::Base58,
            EncodingBind::Base64 => Encoding::Base64,
            EncodingBind::Hex => Encoding::Hex,
        }
    }
}

impl fmt::Display for EncodingBind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
