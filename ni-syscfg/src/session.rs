use ni_syscfg_sys::*;
use std::ffi::CString;

use crate::error::{api_status, Result};
use crate::handles::close_handle;
use crate::resources::ResourceList;

pub struct Session {
    handle: NISysCfgSessionHandle,
}

impl Session {
    pub fn new_local_session() -> Result<Self> {
        let mut handle: NISysCfgSessionHandle = std::ptr::null_mut();
        let localhost = CString::new("localhost").unwrap();

        unsafe {
            api_status(NISysCfgInitializeSession(
                localhost.as_ptr(),
                std::ptr::null(),
                std::ptr::null(),
                NISysCfgLocale_NISysCfgLocaleDefault,
                NISysCfgBool_NISysCfgBoolFalse,
                1000,
                std::ptr::null_mut(),
                &mut handle,
            ))?;
        }

        Ok(Self { handle })
    }

    pub fn handle(&self) -> &NISysCfgSessionHandle {
        &self.handle
    }

    pub fn find_hardware(&self) -> Result<ResourceList> {
        let mut list_handle: NISysCfgEnumResourceHandle = std::ptr::null_mut();

        unsafe {
            api_status(NISysCfgFindHardware(
                self.handle,
                1,
                std::ptr::null_mut(),
                std::ptr::null(),
                &mut list_handle,
            ))?;
        }

        Ok(ResourceList::from_handle(list_handle, self))
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        let _ = close_handle(self.handle);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::Session::new_local_session().unwrap();
    }
}
