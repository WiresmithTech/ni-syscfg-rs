//! Wrappers for a few utility types.
//!

use ni_syscfg_sys::{NISysCfgBool_NISysCfgBoolFalse, NISysCfgBool_NISysCfgBoolTrue};

/// Wraps the constant values for true and false elements.
#[repr(i32)]
pub enum FfiBoolean {
    True = NISysCfgBool_NISysCfgBoolTrue,
    False = NISysCfgBool_NISysCfgBoolFalse,
}
