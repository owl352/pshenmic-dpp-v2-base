use dpp::version::PlatformVersion;
use dpp::version::v1::PLATFORM_V1;
use dpp::version::v2::PLATFORM_V2;
use dpp::version::v3::PLATFORM_V3;
use dpp::version::v5::PLATFORM_V5;
use dpp::version::v6::PLATFORM_V6;
use dpp::version::v7::PLATFORM_V7;
use dpp::version::v8::PLATFORM_V8;
use dpp::version::v9::PLATFORM_V9;
use crate::errors::enums::EnumsError;

#[derive(Default)]
#[allow(non_camel_case_types)]
pub enum PlatformVersionBind {
    PLATFORM_V1,
    PLATFORM_V2,
    PLATFORM_V3,
    PLATFORM_V4,
    PLATFORM_V5,
    PLATFORM_V6,
    PLATFORM_V7,
    PLATFORM_V8,
    #[default]
    PLATFORM_V9,
}

impl TryFrom<u32> for PlatformVersionBind {

    type Error = EnumsError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(PlatformVersionBind::PLATFORM_V1),
            2 => Ok(PlatformVersionBind::PLATFORM_V2),
            3 => Ok(PlatformVersionBind::PLATFORM_V3),
            4 => Ok(PlatformVersionBind::PLATFORM_V4),
            5 => Ok(PlatformVersionBind::PLATFORM_V5),
            6 => Ok(PlatformVersionBind::PLATFORM_V6),
            7 => Ok(PlatformVersionBind::PLATFORM_V7),
            8 => Ok(PlatformVersionBind::PLATFORM_V8),
            9 => Ok(PlatformVersionBind::PLATFORM_V9),
            0_u32 | 10_u32..=u32::MAX => Err(EnumsError::CannotParseValueFromNumber),
        }
    }
}

impl From<PlatformVersionBind> for PlatformVersion {
    fn from(value: PlatformVersionBind) -> Self {
        match value { 
            PlatformVersionBind::PLATFORM_V1 => PLATFORM_V1,
            PlatformVersionBind::PLATFORM_V2 => PLATFORM_V2,
            PlatformVersionBind::PLATFORM_V3 => PLATFORM_V3,
            PlatformVersionBind::PLATFORM_V4 => PLATFORM_V3,
            PlatformVersionBind::PLATFORM_V5 => PLATFORM_V5,
            PlatformVersionBind::PLATFORM_V6 => PLATFORM_V6,
            PlatformVersionBind::PLATFORM_V7 => PLATFORM_V7,
            PlatformVersionBind::PLATFORM_V8 => PLATFORM_V8,
            PlatformVersionBind::PLATFORM_V9 => PLATFORM_V9
        }
    }
}