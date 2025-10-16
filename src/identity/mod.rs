use crate::enums::platform_version::PlatformVersionBind;
use crate::errors::identity::IdentityError;
use crate::errors::utils::DecodeStringError;
use crate::identifier::IdentifierBind;
use dpp::identity::accessors::{IdentityGettersV0, IdentitySettersV0};
use dpp::platform_value::string_encoding::Encoding::{Base64, Hex};
use dpp::platform_value::string_encoding::{decode, encode};
use dpp::prelude::Identity;
use dpp::serialization::{PlatformDeserializable, PlatformSerializable};

#[derive(Debug, Clone)]
pub struct IdentityBind(Identity);

impl IdentityBind {
    pub fn new(
        identifier: IdentifierBind,
        platform_version: Option<PlatformVersionBind>,
    ) -> Result<Self, IdentityError> {
        let identity = Identity::create_basic_identity(
            identifier.into(),
            &platform_version.unwrap_or_default().into(),
        )
        .map_err(|err| IdentityError::CannotCreateBasicIdentityWithIdentifier(err.to_string()))?;

        Ok(Self(identity))
    }

    pub fn set_id(&mut self, identifier: IdentifierBind) {
        self.0.set_id(identifier.into())
    }

    pub fn set_balance(&mut self, balance: u64) {
        self.0.set_balance(balance);
    }

    pub fn set_revision(&mut self, revision: u64) {
        self.0.set_revision(revision);
    }

    // pub fn add_public_key(&mut self, public_key: &IdentityPublicKeyWASM) {
    //     self.0.add_public_key(public_key.clone().into());
    // }

    pub fn get_id(&self) -> IdentifierBind {
        self.0.id().into()
    }

    pub fn get_balance(&self) -> u64 {
        self.0.balance()
    }

    pub fn get_revision(&self) -> u64 {
        self.0.revision()
    }

    // pub fn get_public_key_by_id(&self, key_id: KeyID) -> Option<IdentityPublicKeyWASM> {
    //     let identity_public_key = self.0.get_public_key_by_id(key_id);
    //     identity_public_key.map(|key| IdentityPublicKeyWASM::from(key.clone()))
    // }

    // pub fn get_public_keys(&self) -> Vec<IdentityPublicKeyWASM> {
    //     let keys = self
    //         .0
    //         .public_keys()
    //         .iter()
    //         .map(|(_index, key)| IdentityPublicKeyWASM::from(key.clone()))
    //         .collect();
    //
    //     keys
    // }

    pub fn from_hex(hex: String) -> Result<Self, IdentityError> {
        let bytes: Result<Vec<u8>, IdentityError> = decode(hex.as_str(), Hex)
            .map_err(|err| DecodeStringError::CannotDecodeString(err.to_string()).into());

        Self::from_bytes(bytes?)
    }

    pub fn from_base64(base64: String) -> Result<Self, IdentityError> {
        let bytes: Result<Vec<u8>, IdentityError> = decode(base64.as_str(), Base64)
            .map_err(|err| DecodeStringError::CannotDecodeString(err.to_string()).into());

        Self::from_bytes(bytes?)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, IdentityError> {
        self.0
            .serialize_to_bytes()
            .map_err(|err| IdentityError::IdentitySerializationFailed(err.to_string()))
    }

    pub fn to_hex(&self) -> Result<String, IdentityError> {
        Ok(encode(
            self.0
                .serialize_to_bytes()
                .map_err(|err| IdentityError::IdentitySerializationFailed(err.to_string()))?
                .as_slice(),
            Hex,
        ))
    }

    pub fn to_base64(&self) -> Result<String, IdentityError> {
        Ok(encode(
            self.0
                .serialize_to_bytes()
                .map_err(|err| IdentityError::IdentitySerializationFailed(err.to_string()))?
                .as_slice(),
            Base64,
        ))
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, IdentityError> {
        match Identity::deserialize_from_bytes(bytes.as_slice()) {
            Ok(identity) => Ok(Self(identity)),
            Err(err) => Err(IdentityError::IdentityDeserializationFailed(
                err.to_string(),
            )),
        }
    }
}
