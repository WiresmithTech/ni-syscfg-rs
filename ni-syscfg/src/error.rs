use ni_syscfg_sys::*;
use paste::paste;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NiSystemConfigurationError {
    #[error("API Error: {0:?}")]
    ApiError(NiSysCfgApiStatus),
    #[error("String Conversion Error From The API: {0}")]
    IntoStringError(#[from] std::ffi::IntoStringError),
    #[error("Null in String from API")]
    NulStringError(#[from] std::ffi::NulError),
    #[error("Unexpected Enum Value from API: {0}")]
    UnexpectedEnumValue(i32),
}

macro_rules! syscfg_error {
    ( $( $err:ident ),* ) => {
        /// A rust representation of the different codes the API can return.
        #[derive(Debug, PartialEq)]
        pub enum NiSysCfgApiStatus {
            Unknown(NISysCfgStatus),
            $(
                $err,
            )*
        }

        impl From<i32> for NiSysCfgApiStatus {
            fn from(item: i32) -> Self {
                #![allow(non_upper_case_globals)]
                #![allow(non_snake_case)]
                match item {
                    $(
                        paste!{[<NISysCfgStatus_NISysCfg_ $err>]} => Self::$err,
                )*
                    _ => Self::Unknown(item),
                }
            }
        }



    };
}

// I've used a macro and then a vim search/replace to extract the specific names from the bindings.
syscfg_error!(
    OK,
    EndOfEnum,
    SelfTestBasicOnly,
    FoundCachedOfflineSystem,
    RestartLocalhostInitiated,
    ChangedPropertyNotSaved,
    NotImplemented,
    NullPointer,
    Fail,
    Unexpected,
    OutOfMemory,
    InvalidArg,
    OperationTimedOut,
    FileNotFound,
    InvalidMACFormat,
    PropMismatch,
    PropDoesNotExist,
    UriIllegalSyntax,
    UriTargetDoesNotExist,
    UriExpertDoesNotExist,
    ItemDoesNotExist,
    InvalidMode,
    SysConfigAPINotInstalled,
    NameSyntaxIllegal,
    NameCollision,
    NoPropValidated,
    UriUnauthorized,
    RenameResourceDependencies,
    ValueInvalid,
    ValuesInconsistent,
    Canceled,
    ResponseSyntax,
    ResourceIsNotPresent,
    ResourceIsSimulated,
    NotInFirmwareUpdateState,
    FirmwareImageDeviceMismatch,
    FirmwareImageCorrupt,
    InvalidFirmwareVersion,
    OlderFirmwareVersion,
    InvalidLoginCredentials,
    FirmwareUpdateAttemptFailed,
    EncryptionFailed,
    SomePropsNotValidated,
    InvalidCalibrationCredentials,
    CannotDeletePresentResource,
    UriTargetTransmitError,
    DecryptionFailed,
    FirmwareExpertVersionMismatch,
    AmbiguousImportAction,
    RequiredItemFailedImport,
    ItemInUse,
    ItemTypeNotSupported,
    PermissionDenied,
    SystemNotFound,
    TransformFailed,
    NotInstalled,
    LaunchFailure,
    InternalTimeout,
    MissingTransform,
    IncorrectExtension,
    FileReadOnly,
    ReportOverwrite,
    DirectoryError,
    CannotOpenFile,
    InsufficientPermissions,
    NCECopierFailed,
    FileOperationFailed,
    NameCollisionError,
    UnexpectedError,
    NCENoStreamError,
    NCECompressionError,
    NCEStreamReadError,
    NCEStreamWriteError,
    NCEStreamSeekError,
    NCERepoNotReady,
    NCERepoInvalid,
    NCERepoIncompat,
    NCENoImportStorage,
    NCENoExportStorage,
    NCENoObjCopier,
    CopyInProgress,
    FileNotRecognized,
    SystemNotSupported,
    SystemNotReachable,
    ProductSoftwareNotInstalled,
    ProductSoftwareTooOld,
    ProductSoftwareTooNew,
    DataTooOld,
    DataTooNew,
    NoItemsToCopy,
    OrphanItems,
    DirtyItems,
    FileOverwrite,
    ItemOverwrite,
    MissingDependency,
    OperationCanceled,
    WarningConflicts,
    ErrorConflicts,
    ItemsRequireUserInput,
    ProductExpertNotReady,
    OrphanFiles,
    IsConst,
    UnsupportedProductMode,
    HostSoftwareTooOld,
    OpkgUpdateFeedFailure,
    FeedNotFound,
    FeedAlreadyExists,
    InstallOptionNotSupported,
    FirmwareTooOld,
    SoftwareTooOld,
    RequiresSSH,
    OpkgResponseSyntax,
    WrongSoftwareSetType,
    RequiresOpkg,
    HDFormatEncryptNotSupported,
    HDFormatNoRecoveryKeyDevice,
    RestartLocalhostAmbiguous,
    ImageInvalidCorrupt,
    SafeOrInstallModeRequired,
    EncryptPhraseMismatch,
    InvalidIP,
    InvalidGateway,
    InvalidDNS,
    InvalidSubnet,
    CmdNotSupported,
    ConfigFailed,
    Locked,
    BadPassword,
    NotConfigurable,
    UnlockFailed,
    LockFailed,
    InstallFailed,
    InstallationCorrupt,
    EmptyFile,
    UnconfiguredIP,
    InstallationGenericFailure,
    DownloadAlreadyStarted,
    Aborted,
    DiskFull,
    HDFormatFailed,
    HDFormatNotSafeMode,
    HDFormatRebootFailed,
    ConnectionRefused,
    GetRemoteFilesFailed,
    PutRemoteFilesFailed,
    InvalidImage,
    ImageDeviceCodeMismatch,
    SystemMismatch,
    HDFormatWrongFS,
    CustomInstallNotSupported,
    FTPFailed,
    Timeout,
    DirNotFound,
    PathNotFound,
    NoSoftwareAvailable,
    OverwriteError,
    HDFormatCannotKeepCfg,
    FileOrPathTooLong,
    DDPInternalTimeout,
    IOPermissionDenied,
    PathAlreadyExists,
    ExecutionFailure,
    DownloadError,
    NetSendFailed,
    ContactHostDisconnected,
    NetSvcDown,
    NotConfirmed,
    HostNotResolved,
    RebootTimeout,
    NoConfirmationFP1600,
    DuplicateStartup,
    RemoteInvalidArgument,
    NotUninstallable,
    DuplicatesNotAllowed,
    NotInstallable,
    WrongDevice,
    WrongOS,
    OSVersionTooOld,
    IOError,
    CorruptConfig,
    BufferOverflow,
    UnsupportedCDFVersion,
    InvalidStack,
    IncompleteStack,
    StackItemMissing,
    TopLevelHiddenComponentError,
    InvalidAddon,
    NoRTImagesFolder,
    NoRTImagesRegistry,
    NoRTS2CDF,
    UnsupportedOS,
    ExactVersionRequired,
    InvalidStartup
);

pub type Result<T> = std::result::Result<T, NiSystemConfigurationError>;

pub fn api_status(status: NISysCfgStatus) -> Result<NiSysCfgApiStatus> {
    if status >= 0 {
        return Ok(status.into());
    } else {
        return Err(NiSystemConfigurationError::ApiError(status.into()));
    }
}

#[cfg(test)]
mod tests {
    use super::NiSysCfgApiStatus;

    #[test]
    fn basic_error_test() {
        assert_eq!(
            NiSysCfgApiStatus::from(-2147418113i32),
            NiSysCfgApiStatus::Unexpected
        );
        assert_eq!(
            NiSysCfgApiStatus::from(-2147220277i32),
            NiSysCfgApiStatus::InvalidStartup
        )
    }
}
