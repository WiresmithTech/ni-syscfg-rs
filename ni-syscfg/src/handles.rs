use std::ffi::c_void;

use crate::error::{api_status, NiSysCfgApiStatus, Result};
use ni_syscfg_sys::NISysCfgCloseHandle;

pub type AnyHandle = *mut c_void;

pub fn close_handle(handle: AnyHandle) -> Result<NiSysCfgApiStatus> {
    unsafe { api_status(NISysCfgCloseHandle(handle)) }
}
