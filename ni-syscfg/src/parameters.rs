use crate::error::{api_status, Result};
/// Put a parameter interface type for all supported types here.
/// this can be thought of as a middleware. Handling type conversions but only loosly wrapping the ffi.
///
/// so each type will still have a read and write for each handle type.
///
/// i.e. functions for readresourceproperty, redresourceporpertyindexed etc.
///
/// also combine strings into here.
///
use ni_syscfg_sys::*;
use std::ffi::{c_void, CString};

pub fn new_simple_string() -> CString {
    CString::new(vec![1; NISYSCFG_SIMPLE_STRING_LENGTH as usize]).unwrap()
}

/// Provides a common trait for all parameter types that can be read from the various parameter interfaces.
///
/// This then provides a typed parameter API that can be used by the rust code.
pub trait ReadableParameter: Sized {
    fn read_resource_parameter(
        handle: NISysCfgResourceHandle,
        id: NISysCfgResourceProperty,
    ) -> Result<Self>;
    fn read_resource_indexed_parameter(
        handle: NISysCfgResourceHandle,
        id: NISysCfgIndexedProperty,
        index: u32,
    ) -> Result<Self>;
}

impl ReadableParameter for i32 {
    fn read_resource_parameter(
        handle: NISysCfgResourceHandle,
        id: NISysCfgResourceProperty,
    ) -> Result<i32> {
        let mut value = 0i32;
        let status = unsafe {
            api_status(NISysCfgGetResourceProperty(
                handle,
                id,
                &mut value as *mut _ as *mut c_void,
            ))?
        };
        Ok(value)
    }

    fn read_resource_indexed_parameter(
        handle: NISysCfgResourceHandle,
        id: NISysCfgIndexedProperty,
        index: u32,
    ) -> Result<i32> {
        let mut value = 0i32;
        let status = unsafe {
            api_status(NISysCfgGetResourceIndexedProperty(
                handle,
                id,
                index,
                &mut value as *mut _ as *mut c_void,
            ))?
        };
        Ok(value)
    }
}

impl ReadableParameter for String {
    fn read_resource_parameter(
        handle: NISysCfgResourceHandle,
        id: NISysCfgResourceProperty,
    ) -> Result<String> {
        let mut value = new_simple_string();
        let value_ptr = value.into_raw();
        let status = unsafe {
            api_status(NISysCfgGetResourceProperty(
                handle,
                id,
                value_ptr as *mut c_void,
            ))?;
            value = CString::from_raw(value_ptr);
        };
        Ok(value.into_string()?)
    }

    fn read_resource_indexed_parameter(
        handle: NISysCfgResourceHandle,
        id: NISysCfgIndexedProperty,
        index: u32,
    ) -> Result<String> {
        let mut value = new_simple_string();
        let value_ptr = value.into_raw();
        let status = unsafe {
            api_status(NISysCfgGetResourceIndexedProperty(
                handle,
                id,
                index,
                value_ptr as *mut c_void,
            ))?;
            value = CString::from_raw(value_ptr);
        };
        Ok(value.into_string()?)
    }
}