//! Functions to support the hardware filters when searching for hardware resources.
//!
use crate::error::{api_status, Result};
use crate::handles::close_handle;
use crate::Session;
use ni_syscfg_sys::*;
use std::ptr::null_mut;

/// Used with [crate::Session::find_hardware] to specify which pieces of hardware are of interest.
pub struct HardwareFilter {
    handle: NISysCfgFilterHandle,
    mode: FilterMode,
}

impl HardwareFilter {
    pub fn new(session: &Session) -> Result<Self> {
        let mut handle = null_mut();
        unsafe {
            api_status(NISysCfgCreateFilter(*session.handle(), &mut handle))?;
        }
        Ok(Self {
            handle,
            mode: FilterMode::MatchValuesAll,
        })
    }

    /// Set the mode for the filter, determining how the filter match is applied.
    pub fn set_mode(mut self, mode: FilterMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn mode(&self) -> FilterMode {
        self.mode
    }

    pub(crate) fn handle(&self) -> NISysCfgFilterHandle {
        self.handle
    }
}

impl Drop for HardwareFilter {
    fn drop(&mut self) {
        let _ = close_handle(self.handle);
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Give the way that the filter properties are used to match against items.
pub enum FilterMode {
    /// Includes all of the properties specified in the input filter.
    MatchValuesAll = NISysCfgFilterMode_NISysCfgFilterModeMatchValuesAll,
    /// Includes any of the properties specified in the input filter.
    MatchValuesAny = NISysCfgFilterMode_NISysCfgFilterModeMatchValuesAny,
    /// Includes none of the properties in the input filter.
    MatchValuesNone = NISysCfgFilterMode_NISysCfgFilterModeMatchValuesNone,
    /// Includes all of the properties specified in the input filter, regardless of the values.
    AllPropertiesExist = NISysCfgFilterMode_NISysCfgFilterModeAllPropertiesExist,
}
