#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("bindings.rs");

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn connect_smoke_test() {
        unsafe {
            let mut handle: NISysCfgSessionHandle = std::ptr::null_mut();
            let empty_string = CString::new("").unwrap();
            let empty_string_ptr = empty_string.as_ptr();

            NISysCfgInitializeSession(
                empty_string_ptr,
                empty_string_ptr,
                empty_string_ptr,
                NISysCfgLocale_NISysCfgLocaleDefault,
                NISysCfgBool_NISysCfgBoolFalse,
                1000,
                std::ptr::null_mut(),
                &mut handle,
            );
            NISysCfgCloseHandle(handle);
        }
    }
}
