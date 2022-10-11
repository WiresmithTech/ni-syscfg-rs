//! Module for managing software on NI systems.
//!

use std::ffi::CString;
use std::os::raw::c_char;
use std::path::Path;

use ni_syscfg_sys::{NISysCfgCreateSystemImageAsFolder, NISysCfgSetSystemImageFromFolder2};

use crate::error::{api_status, Result};
use crate::types::FfiBoolean;
use crate::Session;

type Uuid = String;

struct ImageInfo {
    name: String,
    id: Uuid,
}

impl Session {
    pub fn get_software_image(&self, image: &Path) -> Result<()> {
        let handle = self.handle();
        let path = CString::new(image.as_os_str().to_string_lossy().as_ref())?;
        let title = std::ptr::null();
        let id = std::ptr::null();
        let version = std::ptr::null();
        let description = std::ptr::null();

        unsafe {
            api_status(NISysCfgCreateSystemImageAsFolder(
                *handle,
                title,
                id,
                version,
                description,
                FfiBoolean::True as i32,
                path.as_ptr(),
                std::ptr::null(),
                0,
                std::ptr::null_mut(),
                FfiBoolean::False as i32,
            ))?;
        }
        Ok(())
    }
    pub fn set_software_image(&self, image: &Path) -> Result<()> {
        let handle = self.handle();
        let path = CString::new(image.as_os_str().to_string_lossy().as_ref())?;

        unsafe {
            api_status(NISysCfgSetSystemImageFromFolder2(
                *handle,
                FfiBoolean::True as i32,
                path.as_ptr(),
                std::ptr::null(),
                0,
                std::ptr::null_mut(),
                FfiBoolean::False as i32,
                2,
            ))?;
        }
        Ok(())
    }
}
