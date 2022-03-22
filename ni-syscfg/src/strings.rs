use ni_syscfg_sys::NISYSCFG_SIMPLE_STRING_LENGTH;
use std::ffi::CString;

pub fn new_simple_string() -> CString {
    CString::new(vec![1; NISYSCFG_SIMPLE_STRING_LENGTH as usize]).unwrap()
}
