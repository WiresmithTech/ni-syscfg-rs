use ni_syscfg_sys::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NiSystemConfigurationError {
    #[error("API Error")]
    ApiError(NiSysCfgApiStatus),
    #[error("String Conversion Error From The API")]
    IntoStringError(#[from] std::ffi::IntoStringError),
    #[error("Null in String from API")]
    NulStringError(#[from] std::ffi::NulError),
    #[error("Unexpected Enum Value from API: {0}")]
    UnexpectedEnumValue(i32),
}

#[derive(Clone, Copy, Debug)]
pub enum NiSysCfgApiStatus {
    Unknown(NISysCfgStatus),
    OK,
    EndOfEnum,
    SelfTestBasicOnly,
    FoundCachedOfflineSystem,
    RestartLocalhostInitiated,
    ChangedPropertyNotSaved,
    NotImplemented,
    NullPointer,
    Fail,
    Unexpected,
    OutOfMemory,
    InvalidArg,
    OperationTimedOut,
    FileNotFound,
    InvalidMACFormat,
    PropMismatch,
    PropDoesNotExist,
    UriIllegalSyntax,
    UriTargetDoesNotExist,
    UriExpertDoesNotExist,
    ItemDoesNotExist,
    InvalidMode,
    SysConfigApiNotInstalled,
    NameSyntaxIllegal,
    NameCollision,
    NoPropValidated,
    UriUnauthorized,
    RenameResourceDependencies,
    ValueInvalid,
    ValuesInconsistent,
    Canceled,
    ResponseSyntax,
    ResourceIsNotPresent,
    ResourceIsSimulated,
}

impl From<i32> for NiSysCfgApiStatus {
    fn from(item: i32) -> Self {
        #![allow(non_upper_case_globals)]
        match item {
            NISysCfgStatus_NISysCfg_OK => Self::OK,
            NISysCfgStatus_NISysCfg_EndOfEnum => Self::EndOfEnum,
            NISysCfgStatus_NISysCfg_SelfTestBasicOnly => Self::SelfTestBasicOnly,
            NISysCfgStatus_NISysCfg_FoundCachedOfflineSystem => Self::FoundCachedOfflineSystem,
            NISysCfgStatus_NISysCfg_RestartLocalhostInitiated => Self::RestartLocalhostInitiated,
            NISysCfgStatus_NISysCfg_ChangedPropertyNotSaved => Self::ChangedPropertyNotSaved,
            NISysCfgStatus_NISysCfg_NotImplemented => Self::NotImplemented,
            NISysCfgStatus_NISysCfg_NullPointer => Self::NullPointer,
            NISysCfgStatus_NISysCfg_Fail => Self::Fail,
            NISysCfgStatus_NISysCfg_Unexpected => Self::Unexpected,
            NISysCfgStatus_NISysCfg_OutOfMemory => Self::OutOfMemory,
            NISysCfgStatus_NISysCfg_InvalidArg => Self::InvalidArg,
            NISysCfgStatus_NISysCfg_OperationTimedOut => Self::OperationTimedOut,
            NISysCfgStatus_NISysCfg_FileNotFound => Self::FileNotFound,
            NISysCfgStatus_NISysCfg_InvalidMACFormat => Self::InvalidMACFormat,
            NISysCfgStatus_NISysCfg_PropMismatch => Self::PropMismatch,
            NISysCfgStatus_NISysCfg_PropDoesNotExist => Self::PropDoesNotExist,
            NISysCfgStatus_NISysCfg_UriIllegalSyntax => Self::UriIllegalSyntax,
            NISysCfgStatus_NISysCfg_UriTargetDoesNotExist => Self::UriTargetDoesNotExist,
            NISysCfgStatus_NISysCfg_UriExpertDoesNotExist => Self::UriExpertDoesNotExist,
            NISysCfgStatus_NISysCfg_ItemDoesNotExist => Self::ItemDoesNotExist,
            NISysCfgStatus_NISysCfg_InvalidMode => Self::InvalidMode,
            NISysCfgStatus_NISysCfg_SysConfigAPINotInstalled => Self::SysConfigApiNotInstalled,
            NISysCfgStatus_NISysCfg_NameSyntaxIllegal => Self::NameSyntaxIllegal,
            NISysCfgStatus_NISysCfg_NameCollision => Self::NameCollision,
            NISysCfgStatus_NISysCfg_NoPropValidated => Self::NoPropValidated,
            NISysCfgStatus_NISysCfg_UriUnauthorized => Self::UriUnauthorized,
            NISysCfgStatus_NISysCfg_RenameResourceDependencies => Self::RenameResourceDependencies,
            NISysCfgStatus_NISysCfg_ValueInvalid => Self::ValueInvalid,
            NISysCfgStatus_NISysCfg_ValuesInconsistent => Self::ValuesInconsistent,
            NISysCfgStatus_NISysCfg_Canceled => Self::Canceled,
            NISysCfgStatus_NISysCfg_ResponseSyntax => Self::ResponseSyntax,
            NISysCfgStatus_NISysCfg_ResourceIsNotPresent => Self::ResourceIsNotPresent,
            NISysCfgStatus_NISysCfg_ResourceIsSimulated => Self::ResourceIsSimulated,
            _ => Self::Unknown(item),
        }
    }
}

pub type Result<T> = std::result::Result<T, NiSystemConfigurationError>;

pub fn api_status(status: NISysCfgStatus) -> Result<NiSysCfgApiStatus> {
    if status >= 0 {
        return Ok(status.into());
    } else {
        return Err(NiSystemConfigurationError::ApiError(status.into()));
    }
}
