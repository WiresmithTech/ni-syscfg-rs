//! System interface specific to real time systems.

use ni_syscfg_sys::{NISysCfgSessionHandle, NISysCfgSystemProperty_NISysCfgSystemPropertySystemState};
use crate::Session;
use crate::error::Result;
use crate::parameters::ReadableParameter;

pub struct RealTimeSession {
    handle: NISysCfgSessionHandle
}


impl RealTimeSession {
    pub fn from_session(session: &Session) -> RealTimeSession {
        Self {
            handle: session.handle()
        }
    }

    pub fn status(&self) -> Result<String> {
        String::read_system_parameter(self.handle, NISysCfgSystemProperty_NISysCfgSystemPropertySystemState)
    }
}