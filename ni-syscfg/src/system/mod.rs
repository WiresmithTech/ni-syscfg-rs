//! Implements system parameters on the session.

mod real_time;
mod network;

use ni_syscfg_sys::{NISysCfgSystemProperty, NISysCfgSystemProperty_NISysCfgSystemPropertyHostname, NISysCfgSystemProperty_NISysCfgSystemPropertyIsLocked, NISysCfgSystemProperty_NISysCfgSystemPropertyIsLockingSupported, NISysCfgSystemProperty_NISysCfgSystemPropertyProductId, NISysCfgSystemProperty_NISysCfgSystemPropertyProductName, NISysCfgSystemProperty_NISysCfgSystemPropertySerialNumber};
use crate::Session;
use crate::parameters::{ApiBool, ReadableParameter};
use crate::error::Result;
pub use real_time::RealTimeSession;

impl Session {

    pub fn locked(&self) -> Result<bool> {
        ApiBool::read_system_parameter(self.handle(), NISysCfgSystemProperty_NISysCfgSystemPropertyIsLocked).map(|a| a.into())
    }

    pub fn locking_supported(&self) -> Result<bool> {
        ApiBool::read_system_parameter(self.handle(), NISysCfgSystemProperty_NISysCfgSystemPropertyIsLockingSupported).map(|a| a.into())
    }

    pub fn hostname(&self) -> Result<String> {
        String::read_system_parameter(self.handle(), NISysCfgSystemProperty_NISysCfgSystemPropertyHostname)
    }

    pub fn serial_number(&self) -> Result<String> {
        String::read_system_parameter(self.handle(), NISysCfgSystemProperty_NISysCfgSystemPropertySerialNumber)
    }

    pub fn product_name(&self) -> Result<String> {
        String::read_system_parameter(self.handle(), NISysCfgSystemProperty_NISysCfgSystemPropertyProductName)
    }

    pub fn product_code(&self) -> Result<i32> {
        i32::read_system_parameter(self.handle(), NISysCfgSystemProperty_NISysCfgSystemPropertyProductId)
    }


}