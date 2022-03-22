use ni_syscfg_sys::*;
use std::ffi::CString;

mod error;
use error::{api_status, Result};

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
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe {
            NISysCfgCloseHandle(self.handle);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::Session::new_local_session().unwrap();
    }
}
