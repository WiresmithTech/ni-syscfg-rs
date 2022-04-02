//! Contains a few functions for working with the handles.
//!
//! I've not gone as far as newtyping them but conSIDERing it
//! as all handles need a drop and should not be clone/copy
//! and we could enfore that with a new type here.

use std::ffi::c_void;

use crate::error::{api_status, NiSysCfgApiStatus, Result};
use ni_syscfg_sys::NISysCfgCloseHandle;

pub type AnyHandle = *mut c_void;

pub fn close_handle(handle: AnyHandle) -> Result<NiSysCfgApiStatus> {
    unsafe { api_status(NISysCfgCloseHandle(handle)) }
}
