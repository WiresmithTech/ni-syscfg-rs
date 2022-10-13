//! Wrappers for a few utility types.
//!

use ni_syscfg_sys::{NISysCfgBool_NISysCfgBoolFalse, NISysCfgBool_NISysCfgBoolTrue};

/// Wraps the constant values for true and false elements.
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum FfiBoolean {
    True = NISysCfgBool_NISysCfgBoolTrue,
    False = NISysCfgBool_NISysCfgBoolFalse,
}

impl From<bool> for FfiBoolean {
    fn from(input: bool) -> Self {
        if input {
            FfiBoolean::True
        } else {
            FfiBoolean::False
        }
    }
}

#[cfg(test)]
mod test {
    use super::FfiBoolean;

    #[test]
    fn test_ffi_boolean_from_boolean() {
        assert_eq!(FfiBoolean::True, FfiBoolean::from(true));
        assert_eq!(FfiBoolean::False, FfiBoolean::from(false));
    }
}
