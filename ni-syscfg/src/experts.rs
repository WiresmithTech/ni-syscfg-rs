/// Contains the code to handle various functions around the "experts" in the system configuraton API.
///

/// `ExpertType` wraps the reference names for different experts in the system API.
///
/// Those defined in [https://www.ni.com/en-gb/support/documentation/supplemental/18/valid-experts-for-the-system-configuration-api-functions.html](Valid Experts for the System Configuration API Functions) are already included here.
///
/// [`Unknown`] provides a get out for those not translated already or that are added in the future.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ExpertType {
    CanOpen,
    CRio,
    CRioWithDaqmx,
    NetworkDiscovery,
    FlexRIOModularIO,
    FlexRIOIntegratedIO,
    Ni488_2,
    Ni568x,
    Ni845x,
    NiController,
    NiDaqmx,
    NiDCPower,
    NiDCPower416x,
    NiDMM,
    NiDMM408x,
    NiFGen,
    NimmWave,
    NiOSI,
    NiPTP,
    NiPXImc,
    NiRio,
    RTSI,
    NiScope,
    NiScope5170,
    NiScope5164,
    NiScope5110,
    Serial,
    SLSC,
    STS,
    NiSync,
    NiVISA,
    NiVNA,
    NiVST,
    NiWSN,
    NiXNET,
    PXIPlatformServices,
    Softmotion,
    /// Provides a way of accessing any new or unknown types by including the string directly.
    Unknown(String),
}

impl From<&str> for ExpertType {
    fn from(text: &str) -> Self {
        match text {
            "ni-canopen" => Self::CanOpen,
            "crio" => Self::CRio,
            "crioparent" => ExpertType::CRioWithDaqmx,
            "network" => ExpertType::NetworkDiscovery,
            "niflexrio" => ExpertType::FlexRIOModularIO,
            "niflexrio2" => ExpertType::FlexRIOIntegratedIO,
            "ni-488.2" => ExpertType::Ni488_2,
            "ni-rfpowermeter" => ExpertType::Ni568x,
            "845x" => ExpertType::Ni845x,
            "ni-controller" => ExpertType::NiController,
            "daqmx" => ExpertType::NiDaqmx,
            "dcpowerscx" => ExpertType::NiDCPower,
            "nidcpower416x" => ExpertType::NiDCPower416x,
            "nidmmscx" => ExpertType::NiDMM,
            "nidmm408x" => ExpertType::NiDMM408x,
            "nifgen5433" => ExpertType::NiFGen,
            "ni-mmwave" => ExpertType::NimmWave,
            "ni-osi" => ExpertType::NiOSI,
            "ni-1588" => ExpertType::NiPTP,
            "ni-pximc" => ExpertType::NiPXImc,
            "ni-rio" => ExpertType::NiRio,
            "rtsi" => ExpertType::RTSI,
            "niscopescx" => ExpertType::NiScope,
            "niscope5170" => ExpertType::NiScope5170,
            "niscope5164" => ExpertType::NiScope5164,
            "niscope5110" => ExpertType::NiScope5110,
            "serial" => ExpertType::Serial,
            "slsc" => ExpertType::SLSC,
            "nistsrcbps" => ExpertType::STS,
            "ni-sync" => ExpertType::NiSync,
            "ni-visa" => ExpertType::NiVISA,
            "nivna" => ExpertType::NiVNA,
            "ni-vst" => ExpertType::NiVST,
            "ni-wsn" => ExpertType::NiWSN,
            "xnet" => ExpertType::NiXNET,
            "ni-pxi" => ExpertType::PXIPlatformServices,
            "mcSysApi" => ExpertType::Softmotion,
            _ => ExpertType::Unknown(text.to_string()),
        }
    }
}

impl ExpertType {
    /// Provides the string for the expert used in the system configuration API.
    pub fn to_programmatic_string(&self) -> String {
        match self {
            ExpertType::Unknown(name) => name.clone(),
            ExpertType::CanOpen => "ni-canopen".to_string(),
            ExpertType::CRio => "crio".to_string(),
            ExpertType::CRioWithDaqmx => "crioparent".to_string(),
            ExpertType::NetworkDiscovery => "network".to_string(),
            ExpertType::FlexRIOModularIO => "niflexrio".to_string(),
            ExpertType::FlexRIOIntegratedIO => "niflexrio2".to_string(),
            ExpertType::Ni488_2 => "ni-488.2".to_string(),
            ExpertType::Ni568x => "ni-rfpowermeter".to_string(),
            ExpertType::Ni845x => "845x".to_string(),
            ExpertType::NiController => "ni-controller".to_string(),
            ExpertType::NiDaqmx => "daqmx".to_string(),
            ExpertType::NiDCPower => "dcpowerscx".to_string(),
            ExpertType::NiDCPower416x => "nidcpower416x".to_string(),
            ExpertType::NiDMM => "nidmmscx".to_string(),
            ExpertType::NiDMM408x => "nidmm408x".to_string(),
            ExpertType::NiFGen => "nifgen5433".to_string(),
            ExpertType::NimmWave => "ni-mmwave".to_string(),
            ExpertType::NiOSI => "ni-osi".to_string(),
            ExpertType::NiPTP => "ni-1588".to_string(),
            ExpertType::NiPXImc => "ni-pximc".to_string(),
            ExpertType::NiRio => "ni-rio".to_string(),
            ExpertType::RTSI => "rtsi".to_string(),
            ExpertType::NiScope => "niscopescx".to_string(),
            ExpertType::NiScope5170 => "niscope5170".to_string(),
            ExpertType::NiScope5164 => "niscope5164".to_string(),
            ExpertType::NiScope5110 => "niscope5110".to_string(),
            ExpertType::Serial => "serial".to_string(),
            ExpertType::SLSC => "slsc".to_string(),
            ExpertType::STS => "nistsrcbps".to_string(),
            ExpertType::NiSync => "ni-sync".to_string(),
            ExpertType::NiVISA => "ni-visa".to_string(),
            ExpertType::NiVNA => "nivna".to_string(),
            ExpertType::NiVST => "ni-vst".to_string(),
            ExpertType::NiWSN => "ni-wsn".to_string(),
            ExpertType::NiXNET => "xnet".to_string(),
            ExpertType::PXIPlatformServices => "ni-pxi".to_string(),
            ExpertType::Softmotion => "mcSysApi".to_string(),
        }
    }
}

pub struct SystemExpert {
    expert_type: ExpertType,
    display_name: String,
    version: String,
}

#[cfg(test)]
mod test {

    use super::*;

    fn test_string_conversion(expert_type: ExpertType, programmatic_name: &str) {
        assert_eq!(
            expert_type.to_programmatic_string(),
            programmatic_name.to_string()
        );
        assert_eq!(expert_type, programmatic_name.into());
    }

    #[test]
    fn type_to_string() {
        test_string_conversion(ExpertType::Unknown("unknown".to_string()), "unknown");
        test_string_conversion(ExpertType::CanOpen, "ni-canopen");
        test_string_conversion(ExpertType::CRio, "crio");
        test_string_conversion(ExpertType::CRioWithDaqmx, "crioparent");
        test_string_conversion(ExpertType::NetworkDiscovery, "network");
        test_string_conversion(ExpertType::FlexRIOModularIO, "niflexrio");
        test_string_conversion(ExpertType::FlexRIOIntegratedIO, "niflexrio2");
        test_string_conversion(ExpertType::Ni488_2, "ni-488.2");
        test_string_conversion(ExpertType::Ni568x, "ni-rfpowermeter");
        test_string_conversion(ExpertType::Ni845x, "845x");
        test_string_conversion(ExpertType::NiController, "ni-controller");
        test_string_conversion(ExpertType::NiDaqmx, "daqmx");
        test_string_conversion(ExpertType::NiDCPower, "dcpowerscx");
        test_string_conversion(ExpertType::NiDCPower416x, "nidcpower416x");
        test_string_conversion(ExpertType::NiDMM, "nidmmscx");
        test_string_conversion(ExpertType::NiDMM408x, "nidmm408x");
        test_string_conversion(ExpertType::NiFGen, "nifgen5433");
        test_string_conversion(ExpertType::NimmWave, "ni-mmwave");
        test_string_conversion(ExpertType::NiOSI, "ni-osi");
        test_string_conversion(ExpertType::NiPTP, "ni-1588");
        test_string_conversion(ExpertType::NiPXImc, "ni-pximc");
        test_string_conversion(ExpertType::NiRio, "ni-rio");
        test_string_conversion(ExpertType::RTSI, "rtsi");
        test_string_conversion(ExpertType::NiScope, "niscopescx");
        test_string_conversion(ExpertType::NiScope5170, "niscope5170");
        test_string_conversion(ExpertType::NiScope5164, "niscope5164");
        test_string_conversion(ExpertType::NiScope5110, "niscope5110");
        test_string_conversion(ExpertType::Serial, "serial");
        test_string_conversion(ExpertType::SLSC, "slsc");
        test_string_conversion(ExpertType::STS, "nistsrcbps");
        test_string_conversion(ExpertType::NiSync, "ni-sync");
        test_string_conversion(ExpertType::NiVISA, "ni-visa");
        test_string_conversion(ExpertType::NiVNA, "nivna");
        test_string_conversion(ExpertType::NiVST, "ni-vst");
        test_string_conversion(ExpertType::NiWSN, "ni-wsn");
        test_string_conversion(ExpertType::NiXNET, "xnet");
        test_string_conversion(ExpertType::PXIPlatformServices, "ni-pxi");
        test_string_conversion(ExpertType::Softmotion, "mcSysApi");
    }
}
