use ni_syscfg_sys::{NISysCfgIpAddressMode_NISysCfgIpAddressModeDhcpOnly, NISysCfgIpAddressMode_NISysCfgIpAddressModeDhcpOrLinkLocal, NISysCfgIpAddressMode_NISysCfgIpAddressModeLinkLocalOnly, NISysCfgIpAddressMode_NISysCfgIpAddressModeStatic, NISysCfgSystemProperty_NISysCfgSystemPropertyDnsServer, NISysCfgSystemProperty_NISysCfgSystemPropertyGateway, NISysCfgSystemProperty_NISysCfgSystemPropertyIpAddress, NISysCfgSystemProperty_NISysCfgSystemPropertyIpAddressMode, NISysCfgSystemProperty_NISysCfgSystemPropertyMacAddress, NISysCfgSystemProperty_NISysCfgSystemPropertySubnetMask};
use crate::parameters::{ReadableParameter, ValueEnum};
use crate::Session;
use crate::error::Result;
use num_derive::FromPrimitive;

impl Session {

    pub fn mac_address(&self) -> Result<String> {
        String::read_system_parameter(self.handle(), NISysCfgSystemProperty_NISysCfgSystemPropertyMacAddress)
    }

    pub fn ip_address(&self) -> Result<String> {
        String::read_system_parameter(self.handle(), NISysCfgSystemProperty_NISysCfgSystemPropertyIpAddress)
    }

    pub fn subnet_mask(&self) -> Result<String> {
        String::read_system_parameter(self.handle(), NISysCfgSystemProperty_NISysCfgSystemPropertySubnetMask)
    }

    pub fn gateway(&self) -> Result<String> {
        String::read_system_parameter(self.handle(), NISysCfgSystemProperty_NISysCfgSystemPropertyGateway)
    }

    pub fn dns_server(&self) -> Result<String> {
        String::read_system_parameter(self.handle(), NISysCfgSystemProperty_NISysCfgSystemPropertyDnsServer)
    }

    pub fn address_mode(&self) -> Result<NetworkAddressMode> {
        NetworkAddressMode::read_system_parameter(self.handle(), NISysCfgSystemProperty_NISysCfgSystemPropertyIpAddressMode)
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive)]
pub enum NetworkAddressMode {
    Static = NISysCfgIpAddressMode_NISysCfgIpAddressModeStatic,
    LinkLocalOnly = NISysCfgIpAddressMode_NISysCfgIpAddressModeLinkLocalOnly,
    DhcpOrLinkLocal = NISysCfgIpAddressMode_NISysCfgIpAddressModeDhcpOrLinkLocal,
    DhcpOnly = NISysCfgIpAddressMode_NISysCfgIpAddressModeDhcpOnly,
}

impl ValueEnum for NetworkAddressMode {}