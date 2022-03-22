use std::ffi::{c_void, CString};

use crate::error::{api_status, NiSysCfgApiStatus, Result};
use crate::handles::close_handle;
use crate::session::Session;
use crate::strings::new_simple_string;
use ni_syscfg_sys::*;

pub struct ResourceList<'a> {
    handle: NISysCfgEnumResourceHandle,
    session: &'a Session,
}

impl<'a> Drop for ResourceList<'a> {
    fn drop(&mut self) {
        //ignore result in drop.
        let _ = close_handle(self.handle);
    }
}

impl<'a> ResourceList<'a> {
    pub fn from_handle(handle: NISysCfgEnumResourceHandle, session: &'a Session) -> Self {
        Self { handle, session }
    }
}

impl<'a> Iterator for ResourceList<'a> {
    type Item = Resource;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let mut resource_handle = std::ptr::null_mut();
            let result = api_status(NISysCfgNextResource(
                *self.session.handle(),
                self.handle,
                &mut resource_handle,
            ));

            match result {
                Ok(NiSysCfgApiStatus::EndOfEnum) => None,
                Ok(_) => Some(Resource::from_handle(resource_handle)),
                Err(_) => None,
            }
        }
    }
}

pub struct Resource {
    handle: NISysCfgResourceHandle,
}

impl Resource {
    pub fn from_handle(handle: NISysCfgResourceHandle) -> Self {
        Self { handle }
    }

    pub fn get_name(&self) -> Result<String> {
        let mut name = new_simple_string();
        let name_ptr = name.into_raw();
        unsafe {
            api_status(NISysCfgGetResourceIndexedProperty(
                self.handle,
                NISysCfgIndexedProperty_NISysCfgIndexedPropertyExpertUserAlias,
                0,
                name_ptr as *mut c_void,
            ))?;

            name = CString::from_raw(name_ptr);
        }

        Ok(name.into_string()?)
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        //ignore result in drop.
        let _ = close_handle(self.handle);
    }
}
