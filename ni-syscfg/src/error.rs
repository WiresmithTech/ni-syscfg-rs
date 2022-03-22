use ni_syscfg_sys::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NiSystemConfigurationError {
    #[error("API Error")]
    ApiError(NiSysCfgApiStatus),
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
