//! Module for managing software on NI systems.
//!

use std::ffi::CString;
use std::path::Path;

use ni_syscfg_sys::*;

use crate::error::{api_status, NiSystemConfigurationError, Result};
use crate::types::FfiBoolean;
use crate::Session;

type Uuid = String;

/// Contains the metadata which can be saved with system images.
pub struct ImageInfo {
    /// A title for the image.
    pub title: String,
    /// A unique ID for the image. Specifies the unique identifier for the system image.
    /// If you specify this parameter, it must be in the GUID format "{00000000-0000-0000-0000-000000000000}" where each '0' represents a hexadecimal digit. This should be in the form
    pub id: Uuid,
    /// A short description for the image.
    pub description: String,
    /// A version number for the image.
    pub version: String,
}

/// The network interface settings that can be applied when setting a system image.
///
/// ## Definitions
///
/// * **Reset**: Set the adapters back to default. Normally DHCP.
/// * **Apply**: Set the adapters to the settings of the system the image was taken to.
/// * **Preserve**: Keep the adapter settings of the current system.
#[repr(i32)]
pub enum NetworkInterfaceSettings {
    ResetPrimaryResetOthers = NISysCfgNetworkInterfaceSettings_NISysCfgResetPrimaryResetOthers,
    PreservePrimaryResetOthers =
        NISysCfgNetworkInterfaceSettings_NISysCfgPreservePrimaryResetOthers,
    PreservePrimaryPreserveOthers =
        NISysCfgNetworkInterfaceSettings_NISysCfgPreservePrimaryPreserveOthers,
    PreservePrimaryApplyOthers =
        NISysCfgNetworkInterfaceSettings_NISysCfgPreservePrimaryApplyOthers,
    ApplyPrimaryResetOthers = NISysCfgNetworkInterfaceSettings_NISysCfgApplyPrimaryResetOthers,
    ApplyPrimaryPreserveOthers =
        NISysCfgNetworkInterfaceSettings_NISysCfgApplyPrimaryPreserveOthers,
    ApplyPrimaryApplyOthers = NISysCfgNetworkInterfaceSettings_NISysCfgApplyPrimaryApplyOthers,
}

impl Session {
    /// Creates an image of the system in the `image` path.
    /// The image is a folder containing metadata and the disk image.
    ///
    /// This wraps the `NISysCfgCreateSystemImageAsFolder` method from the C API.
    ///
    /// # Parameters
    /// * `image` should be a path to the directory to be created with the image contents.
    /// * `image_info` provides metadata to be saved with the image.
    /// * `encryption_passphrase` should be [Some] with a value if you wish to encrypt the image or [None] to save the image unencrypted.
    /// * `excluded_files_folders` can contain a list of files and folders which should not be captured in the image.
    /// * `auto_restart` Restarts the system into install mode by default before the operation is performed, and restarts back to a running state after the operation is complete.
    /// If you choose not to restart automatically, and the system is not in install mode, this function returns an error.
    /// * `overwrite_if_exists` defines whether to replace an existing image in the `image` path.
    pub fn get_system_image(
        &self,
        image: &Path,
        image_info: &ImageInfo,
        encryption_passphrase: Option<&str>,
        excluded_files_folders: &[&str],
        auto_restart: bool,
        overwrite_if_exists: bool,
    ) -> Result<()> {
        let handle = self.handle();
        let path = CString::new(image.as_os_str().to_string_lossy().as_ref())?;
        let password = encryption_passphrase.map(|password| CString::new(password));
        let password_ptr = if let Some(actual_password) = password {
            actual_password?.as_ptr()
        } else {
            std::ptr::null()
        };

        let title = CString::new(image_info.title.as_str())?;
        let id = CString::new(image_info.id.as_str())?;
        let version = CString::new(image_info.version.as_str())?;
        let description = CString::new(image_info.description.as_str())?;

        // Build this list. We do it in two steps to ensure Vec<CString> is owned by the function
        // for the duration of the call.
        let excluded_c = str_slice_to_cstring(excluded_files_folders)?;
        let mut excluded_ptrs = cstring_vec_to_ptr_array(&excluded_c[..]);

        unsafe {
            api_status(NISysCfgCreateSystemImageAsFolder(
                handle,
                title.as_ptr(),
                id.as_ptr(),
                version.as_ptr(),
                description.as_ptr(),
                FfiBoolean::from(auto_restart) as i32,
                path.as_ptr(),
                password_ptr,
                excluded_ptrs.len() as u32,
                excluded_ptrs.as_mut_ptr(),
                FfiBoolean::from(overwrite_if_exists) as i32,
            ))?;
        }
        Ok(())
    }

    /// Copy a system image to the target from the `image` path.
    /// The image is a folder containing metadata and the disk image captured with [Session::get_system_image]
    ///
    /// This wraps the `NISysCfgSetSystemImageFromFolder2` method from the C API.
    ///
    /// # Parameters
    /// * `image` should be a path to the directory to be created with the image contents.
    /// * `encryption_passphrase` should be [Some] with a value if the image is encrypted or [None] if the image is unencrypted.
    /// * `excluded_files_folders` can contain a list of files and folders which should not be ovewritten on the tearget.
    /// * `auto_restart` Restarts the system into install mode by default before the operation is performed, and restarts back to a running state after the operation is complete.
    /// If you choose not to restart automatically, and the system is not in install mode, the resulting image may not be valid.
    /// * `original_system_only` defines whether this should only be applied to the same system the image came from based on the MAC address.
    /// * `network_settings` defines the state of the network configuration after the system image has been applied.
    pub fn set_system_image(
        &self,
        image: &Path,
        encryption_passphrase: Option<&str>,
        excluded_files_folders: &[&str],
        auto_restart: bool,
        original_system_only: bool,
        network_settings: NetworkInterfaceSettings,
    ) -> Result<()> {
        let handle = self.handle();
        let path = CString::new(image.as_os_str().to_string_lossy().as_ref())?;
        let password = encryption_passphrase.map(|password| CString::new(password));
        let password_ptr = if let Some(actual_password) = password {
            actual_password?.as_ptr()
        } else {
            std::ptr::null()
        };

        // Build this list. We do it in two steps to ensure Vec<CString> is owned by the function
        // for the duration of the call.
        let excluded_c = str_slice_to_cstring(excluded_files_folders)?;
        let mut excluded_ptrs = cstring_vec_to_ptr_array(&excluded_c[..]);

        unsafe {
            api_status(NISysCfgSetSystemImageFromFolder2(
                handle,
                FfiBoolean::from(auto_restart) as i32,
                path.as_ptr(),
                password_ptr,
                excluded_ptrs.len() as u32,
                excluded_ptrs.as_mut_ptr(),
                FfiBoolean::from(original_system_only) as i32,
                network_settings as i32,
            ))?;
        }
        Ok(())
    }
}

fn str_slice_to_cstring(slice: &[&str]) -> Result<Vec<CString>> {
    slice
        .iter()
        .map(|&item| CString::new(item))
        .map(|item| item.map_err(|e| NiSystemConfigurationError::NulStringError(e)))
        .collect::<Result<Vec<CString>>>()
}

fn cstring_vec_to_ptr_array(list: &[CString]) -> Vec<*const i8> {
    list.iter().map(|cstring| cstring.as_ptr()).collect()
}

#[cfg(test)]
mod test {
    use std::ffi::CStr;

    use super::{cstring_vec_to_ptr_array, str_slice_to_cstring};

    /// Test the conversion to an array of pointers by converting back and confirming contents.
    #[test]
    fn test_string_list_conversion() {
        let original = ["test1", "test2", "test3"];

        //First complete our conversion to FFI Types.

        let cstring = str_slice_to_cstring(&original).unwrap();
        let mut pointers = cstring_vec_to_ptr_array(&cstring);

        let length = pointers.len() as u32;
        let ptr = pointers.as_mut_ptr();

        // This section attempts to replicate what C will do to access this.
        let new_strings: Vec<&CStr> = unsafe {
            let new_pointers = std::slice::from_raw_parts(ptr, length as usize);
            new_pointers
                .iter()
                .map(|&ptr| CStr::from_ptr(ptr))
                .collect()
        };

        for (&new, original) in new_strings.iter().zip(original) {
            let new_string = new.to_str().unwrap();
            assert_eq!(new_string, original);
        }
    }
}
