use ni_syscfg_sys::*;
use std::ffi::{c_void, CString};
use std::time::Duration;

use crate::error::{api_status, Result};
use crate::handles::close_handle;
use crate::hardware_filter::{FilterMode, HardwareFilter};
use crate::parameters::ApiBool;
use crate::resources::HardwareResourceList;

#[repr(i32)]
#[derive(Clone, Copy, Debug)]
pub enum Locale {
    Default = NISysCfgLocale_NISysCfgLocaleDefault,
    ChineseSimplified = NISysCfgLocale_NISysCfgLocaleChineseSimplified,
    English = NISysCfgLocale_NISysCfgLocaleEnglish,
    French = NISysCfgLocale_NISysCfgLocaleFrench,
    German = NISysCfgLocale_NISysCfgLocaleGerman,
    Japanese = NISysCfgLocale_NISysCfgLocaleJapanese,
    Korean = NISysCfgLocale_NISysCfgLocaleKorean,
}

pub struct SessionConfig<'a> {
    target: &'a str,
    username: Option<CString>,
    password: Option<CString>,
    locale: Locale,
    force_refresh: bool,
    timeout: Duration,
}

impl<'a> SessionConfig<'a> {
    pub fn new() -> Self {
        Self {
            target: "",
            username: None,
            password: None,
            locale: Locale::Default,
            force_refresh: false,
            timeout: Duration::from_secs(1),
        }
    }

    pub fn target(mut self, target: &'a str) -> Self {
        self.target = target;
        self
    }

    pub fn username(mut self, username: &str) -> Result<Self> {
        self.username = Some(CString::new(username)?);
        Ok(self)
    }

    pub fn password(mut self, password: &str) -> Result<Self> {
        self.password = Some(CString::new(password)?);
        Ok(self)
    }

    pub fn locale(mut self, locale: Locale) -> Self {
        self.locale = locale;
        self
    }

    pub fn force_refresh(mut self, force_refresh: bool) -> Self {
        self.force_refresh = force_refresh;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn connect(&self) -> Result<Session> {
        fn optional_cstring_to_ptr(input: &Option<CString>) -> *const i8 {
            if let Some(inner) = input {
                inner.as_ptr()
            } else {
                std::ptr::null()
            }
        }

        let mut handle: NISysCfgSessionHandle = std::ptr::null_mut();

        let username = optional_cstring_to_ptr(&self.username);

        let password = optional_cstring_to_ptr(&self.password);

        unsafe {
            api_status(NISysCfgInitializeSession(
                CString::new(self.target)?.as_ptr(),
                username,
                password,
                self.locale as NISysCfgLocale,
                ApiBool::from(self.force_refresh) as NISysCfgBool,
                self.timeout.as_millis() as u32,
                std::ptr::null_mut(),
                &mut handle,
            ))?;
        }

        Ok(Session::new_from_handle(handle))
    }
}

/// Provides an interface to an active system configuration session.
///
/// This is created when you connect to a target using [SessionConfig]
/// and allows you to access hardware and software resources through the API.
pub struct Session {
    handle: NISysCfgSessionHandle,
}

impl Session {
    fn new_from_handle(handle: NISysCfgSessionHandle) -> Self {
        Self { handle }
    }

    pub(crate) fn handle(&self) -> &NISysCfgSessionHandle {
        &self.handle
    }

    /// Create a new filter for the session to use as part of [find_hardware]
    pub fn create_filter(&self) -> Result<HardwareFilter> {
        HardwareFilter::new(&self)
    }

    /// Find the hardware resources in the system, optionally with the filter settings.
    ///
    /// # Example Without Filtering
    /// ```
    /// use ni_syscfg::SessionConfig;
    ///
    /// let session = SessionConfig::new().connect().unwrap();
    ///
    /// for hardware in session.find_hardware(None).unwrap() {
    ///   println!("Found {}", hardware.name().unwrap())
    /// }
    /// ```
    ///
    /// # Example With Filtering
    /// ```
    /// use ni_syscfg::{SessionConfig, FilterMode};
    ///
    /// let session = SessionConfig::new().connect().unwrap();
    /// let mut filter = session.create_filter().unwrap();
    /// filter.mode = FilterMode::MatchValuesAny;
    ///
    /// for hardware in session.find_hardware(Some(filter)).unwrap() {
    ///   println!("Found {}", hardware.name().unwrap())
    /// }
    /// ```
    pub fn find_hardware(&self, filtering: Option<HardwareFilter>) -> Result<HardwareResourceList> {
        let mut list_handle: NISysCfgEnumResourceHandle = std::ptr::null_mut();

        let (filter_mode, filter_handle) = if let Some(filter) = filtering {
            (filter.mode(), filter.handle())
        } else {
            (
                FilterMode::MatchValuesAll,
                std::ptr::null_mut() as NISysCfgFilterHandle,
            )
        };

        unsafe {
            api_status(NISysCfgFindHardware(
                self.handle,
                filter_mode as i32,
                filter_handle,
                std::ptr::null(),
                &mut list_handle,
            ))?;
        }

        Ok(HardwareResourceList::from_handle(list_handle, self))
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        let _ = close_handle(self.handle);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::SessionConfig::new().connect().unwrap();
    }
}
