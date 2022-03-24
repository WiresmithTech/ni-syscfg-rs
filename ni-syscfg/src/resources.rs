use std::marker::PhantomData;

use crate::error::{api_status, NiSysCfgApiStatus, Result};
use crate::handles::close_handle;
use crate::parameters::{BusType, ReadableParameter};
use crate::session::Session;
use ni_syscfg_sys::*;

pub struct HardwareResourceList<'a> {
    handle: NISysCfgEnumResourceHandle,
    session: &'a Session,
}

impl<'a> Drop for HardwareResourceList<'a> {
    fn drop(&mut self) {
        //ignore result in drop.
        let _ = close_handle(self.handle);
    }
}

impl<'a> HardwareResourceList<'a> {
    pub fn from_handle(handle: NISysCfgEnumResourceHandle, session: &'a Session) -> Self {
        Self { handle, session }
    }
}

impl<'a> Iterator for HardwareResourceList<'a> {
    type Item = HardwareResource;

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
                Ok(_) => Some(HardwareResource::from_handle(resource_handle)),
                Err(_) => None,
            }
        }
    }
}

pub struct HardwareResource {
    handle: NISysCfgResourceHandle,
}

pub struct ResourceParameter<T: ReadableParameter> {
    id: i32,
    phantom: PhantomData<T>,
}
impl HardwareResource {
    pub fn from_handle(handle: NISysCfgResourceHandle) -> Self {
        Self { handle }
    }

    pub fn name(&self) -> Result<String> {
        String::read_resource_indexed_parameter(
            self.handle,
            NISysCfgIndexedProperty_NISysCfgIndexedPropertyExpertUserAlias,
            0,
        )
    }

    pub fn get_parameter<T: ReadableParameter>(
        &self,
        parameter: ResourceParameter<T>,
    ) -> Result<T> {
        T::read_resource_parameter(self.handle, parameter.id)
    }

    //Specific parameters follow.
    pub fn connects_to_bus_type(&self) -> Result<BusType> {
        BusType::read_resource_parameter(
            self.handle,
            NISysCfgResourceProperty_NISysCfgResourcePropertyConnectsToBusType,
        )
    }
}

impl Drop for HardwareResource {
    fn drop(&mut self) {
        //ignore result in drop.
        let _ = close_handle(self.handle);
    }
}
