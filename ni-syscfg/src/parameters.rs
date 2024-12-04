use crate::error::{api_status, NiSystemConfigurationError, Result};
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
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
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
    fn read_system_parameter(
        handle: NISysCfgSessionHandle,
        id: NISysCfgSystemProperty
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

    fn read_system_parameter(
        handle: NISysCfgSessionHandle,
        id: NISysCfgSystemProperty
    ) -> Result<i32> {
        let mut value = 0i32;
        let status = unsafe {
            api_status(NISysCfgGetSystemProperty(handle, id, &mut value as *mut _ as *mut c_void))?
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

    fn read_system_parameter(handle: NISysCfgSessionHandle, id: NISysCfgSystemProperty) -> Result<Self> {
        let mut value = new_simple_string();
        let value_ptr = value.into_raw();
        let status = unsafe {
            api_status(NISysCfgGetSystemProperty(handle, id, value_ptr as *mut c_void))?;
            value = CString::from_raw(value_ptr);
        };
        Ok(value.into_string()?)
    }
}

/// Marker trait for enums to be used as property values.
trait ValueEnum: FromPrimitive {}

impl<T> ReadableParameter for T
where
    T: ValueEnum,
{
    fn read_resource_parameter(
        handle: NISysCfgResourceHandle,
        id: NISysCfgResourceProperty,
    ) -> Result<T> {
        let mut value = 0i32;
        let status = unsafe {
            api_status(NISysCfgGetResourceProperty(
                handle,
                id,
                &mut value as *mut _ as *mut c_void,
            ))?
        };
        if let Some(inner) = T::from_i32(value) {
            Ok(inner)
        } else {
            Err(NiSystemConfigurationError::UnexpectedEnumValue(value))
        }
    }

    fn read_resource_indexed_parameter(
        handle: NISysCfgResourceHandle,
        id: NISysCfgIndexedProperty,
        index: u32,
    ) -> Result<T> {
        let mut value = 0i32;
        let status = unsafe {
            api_status(NISysCfgGetResourceIndexedProperty(
                handle,
                id,
                index,
                &mut value as *mut _ as *mut c_void,
            ))?
        };
        if let Some(inner) = T::from_i32(value) {
            Ok(inner)
        } else {
            Err(NiSystemConfigurationError::UnexpectedEnumValue(value))
        }
    }
    fn read_system_parameter(handle: NISysCfgSessionHandle, id: NISysCfgSystemProperty) -> Result<Self> {
        let mut value = 0i32;
        let status = unsafe {
            api_status(NISysCfgGetSystemProperty(
                handle,
                id,
                &mut value as *mut _ as *mut c_void,
            ))
        };
        if let Some(inner) = T::from_i32(value) {
            Ok(inner)
        } else {
            Err(NiSystemConfigurationError::UnexpectedEnumValue(value))
        }

    }
}

#[repr(i32)]
#[derive(FromPrimitive, Copy, Clone, Debug)]
pub enum BusType {
    BuiltIn = NISysCfgBusType_NISysCfgBusTypeBuiltIn,
    PciPxi = NISysCfgBusType_NISysCfgBusTypePciPxi,
    Usb = NISysCfgBusType_NISysCfgBusTypeUsb,
    Gpib = NISysCfgBusType_NISysCfgBusTypeGpib,
    Vxi = NISysCfgBusType_NISysCfgBusTypeVxi,
    Serial = NISysCfgBusType_NISysCfgBusTypeSerial,
    TcpIp = NISysCfgBusType_NISysCfgBusTypeTcpIp,
    CompactRio = NISysCfgBusType_NISysCfgBusTypeCompactRio,
    Scxi = NISysCfgBusType_NISysCfgBusTypeScxi,
    CompactDaq = NISysCfgBusType_NISysCfgBusTypeCompactDaq,
    SwitchBlock = NISysCfgBusType_NISysCfgBusTypeSwitchBlock,
    Scc = NISysCfgBusType_NISysCfgBusTypeScc,
    FireWire = NISysCfgBusType_NISysCfgBusTypeFireWire,
    Accessory = NISysCfgBusType_NISysCfgBusTypeAccessory,
    Can = NISysCfgBusType_NISysCfgBusTypeCan,
    SwitchBlockDevice = NISysCfgBusType_NISysCfgBusTypeSwitchBlockDevice,
    Slsc = 16,
}

impl ValueEnum for BusType {}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApiBool {
    False = NISysCfgBool_NISysCfgBoolFalse,
    True = NISysCfgBool_NISysCfgBoolTrue,
}

impl FromPrimitive for ApiBool {
    fn from_i32(n: i32) -> Option<Self> {
        if n == 0 {
            Some(Self::False)
        }
        else {
            Some(Self::True)
        }
    }

    fn from_i64(n: i64) -> Option<Self> {
       if n == 0 {
           Some(Self::False)
       }
        else {
            Some(Self::True)
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        if n == 0 {
            Some(Self::False)
        }
        else {
            Some(Self::True)
        }
    }
}

impl ValueEnum for ApiBool {}

impl From<bool> for ApiBool {
    fn from(input: bool) -> Self {
        if input {
            Self::True
        } else {
            Self::False
        }
    }
}

impl From<ApiBool> for bool {
    fn from(input: ApiBool) -> Self {
        match input {
            ApiBool::False => false,
            ApiBool::True => true,
        }
    }
}
