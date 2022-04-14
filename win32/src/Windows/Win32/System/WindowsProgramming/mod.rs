#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    clippy::all
)]
#[allow(unused)]
use win32::core::prelude::*;
pub const AADBE_ADD_ENTRY: u32 = 1u32;
pub const AADBE_DEL_ENTRY: u32 = 2u32;
pub const ACTCTX_FLAG_APPLICATION_NAME_VALID: u32 = 32u32;
pub const ACTCTX_FLAG_ASSEMBLY_DIRECTORY_VALID: u32 = 4u32;
pub const ACTCTX_FLAG_HMODULE_VALID: u32 = 128u32;
pub const ACTCTX_FLAG_LANGID_VALID: u32 = 2u32;
pub const ACTCTX_FLAG_PROCESSOR_ARCHITECTURE_VALID: u32 = 1u32;
pub const ACTCTX_FLAG_RESOURCE_NAME_VALID: u32 = 8u32;
pub const ACTCTX_FLAG_SET_PROCESS_DEFAULT: u32 = 16u32;
pub const ACTCTX_FLAG_SOURCE_IS_ASSEMBLYREF: u32 = 64u32;
pub struct ACTCTX_SECTION_KEYED_DATA_2600 {
    pub cbSize: u32,
    pub ulDataFormatVersion: u32,
    pub lpData: MutPtr<::core::ffi::c_void>,
    pub ulLength: u32,
    pub lpSectionGlobalData: MutPtr<::core::ffi::c_void>,
    pub ulSectionGlobalDataLength: u32,
    pub lpSectionBase: MutPtr<::core::ffi::c_void>,
    pub ulSectionTotalLength: u32,
    pub hActCtx: super::super::Foundation::HANDLE,
    pub ulAssemblyRosterIndex: u32,
}
impl ::core::marker::Copy for ACTCTX_SECTION_KEYED_DATA_2600 {}
impl ::core::clone::Clone for ACTCTX_SECTION_KEYED_DATA_2600 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTCTX_SECTION_KEYED_DATA_2600 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTCTX_SECTION_KEYED_DATA_2600")
            .field("cbSize", &self.cbSize)
            .field("ulDataFormatVersion", &self.ulDataFormatVersion)
            .field("lpData", &self.lpData)
            .field("ulLength", &self.ulLength)
            .field("lpSectionGlobalData", &self.lpSectionGlobalData)
            .field("ulSectionGlobalDataLength", &self.ulSectionGlobalDataLength)
            .field("lpSectionBase", &self.lpSectionBase)
            .field("ulSectionTotalLength", &self.ulSectionTotalLength)
            .field("hActCtx", &self.hActCtx)
            .field("ulAssemblyRosterIndex", &self.ulAssemblyRosterIndex)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACTCTX_SECTION_KEYED_DATA_2600 {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.ulDataFormatVersion == other.ulDataFormatVersion
            && self.lpData == other.lpData
            && self.ulLength == other.ulLength
            && self.lpSectionGlobalData == other.lpSectionGlobalData
            && self.ulSectionGlobalDataLength == other.ulSectionGlobalDataLength
            && self.lpSectionBase == other.lpSectionBase
            && self.ulSectionTotalLength == other.ulSectionTotalLength
            && self.hActCtx == other.hActCtx
            && self.ulAssemblyRosterIndex == other.ulAssemblyRosterIndex
    }
}
impl ::core::cmp::Eq for ACTCTX_SECTION_KEYED_DATA_2600 {}
impl FromIntoMemory for ACTCTX_SECTION_KEYED_DATA_2600 {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    pub lpInformation: MutPtr<::core::ffi::c_void>,
    pub lpSectionBase: MutPtr<::core::ffi::c_void>,
    pub ulSectionLength: u32,
    pub lpSectionGlobalDataBase: MutPtr<::core::ffi::c_void>,
    pub ulSectionGlobalDataLength: u32,
}
impl ::core::marker::Copy for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {}
impl ::core::clone::Clone for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA")
            .field("lpInformation", &self.lpInformation)
            .field("lpSectionBase", &self.lpSectionBase)
            .field("ulSectionLength", &self.ulSectionLength)
            .field("lpSectionGlobalDataBase", &self.lpSectionGlobalDataBase)
            .field("ulSectionGlobalDataLength", &self.ulSectionGlobalDataLength)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpInformation == other.lpInformation
            && self.lpSectionBase == other.lpSectionBase
            && self.ulSectionLength == other.ulSectionLength
            && self.lpSectionGlobalDataBase == other.lpSectionGlobalDataBase
            && self.ulSectionGlobalDataLength == other.ulSectionGlobalDataLength
    }
}
impl ::core::cmp::Eq for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {}
impl FromIntoMemory for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct ACTIVATION_CONTEXT_BASIC_INFORMATION {
    pub hActCtx: super::super::Foundation::HANDLE,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for ACTIVATION_CONTEXT_BASIC_INFORMATION {}
impl ::core::clone::Clone for ACTIVATION_CONTEXT_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTIVATION_CONTEXT_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTIVATION_CONTEXT_BASIC_INFORMATION")
            .field("hActCtx", &self.hActCtx)
            .field("dwFlags", &self.dwFlags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACTIVATION_CONTEXT_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.hActCtx == other.hActCtx && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for ACTIVATION_CONTEXT_BASIC_INFORMATION {}
impl FromIntoMemory for ACTIVATION_CONTEXT_BASIC_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub const ACTIVATION_CONTEXT_BASIC_INFORMATION_DEFINED: u32 = 1u32;
pub const AC_LINE_BACKUP_POWER: u32 = 2u32;
pub const AC_LINE_OFFLINE: u32 = 0u32;
pub const AC_LINE_ONLINE: u32 = 1u32;
pub const AC_LINE_UNKNOWN: u32 = 255u32;
pub const ADN_DEL_IF_EMPTY: u32 = 1u32;
pub const ADN_DEL_UNC_PATHS: u32 = 8u32;
pub const ADN_DONT_DEL_DIR: u32 = 4u32;
pub const ADN_DONT_DEL_SUBDIRS: u32 = 2u32;
pub const AFSR_BACKNEW: u32 = 2u32;
pub const AFSR_EXTRAINCREFCNT: u32 = 2048u32;
pub const AFSR_NODELETENEW: u32 = 4u32;
pub const AFSR_NOMESSAGES: u32 = 8u32;
pub const AFSR_NOPROGRESS: u32 = 16u32;
pub const AFSR_RESTORE: u32 = 1u32;
pub const AFSR_UPDREFCNT: u32 = 512u32;
pub const AFSR_USEREFCNT: u32 = 1024u32;
pub const AIF_FORCE_FILE_IN_USE: u32 = 8u32;
pub const AIF_NOLANGUAGECHECK: u32 = 268435456u32;
pub const AIF_NOOVERWRITE: u32 = 16u32;
pub const AIF_NOSKIP: u32 = 2u32;
pub const AIF_NOVERSIONCHECK: u32 = 4u32;
pub const AIF_NO_VERSION_DIALOG: u32 = 32u32;
pub const AIF_QUIET: u32 = 536870912u32;
pub const AIF_REPLACEONLY: u32 = 1024u32;
pub const AIF_WARNIFSKIP: u32 = 1u32;
pub const ALINF_BKINSTALL: u32 = 32u32;
pub const ALINF_CHECKBKDATA: u32 = 128u32;
pub const ALINF_DELAYREGISTEROCX: u32 = 512u32;
pub const ALINF_NGCONV: u32 = 8u32;
pub const ALINF_QUIET: u32 = 4u32;
pub const ALINF_ROLLBACK: u32 = 64u32;
pub const ALINF_ROLLBKDOALL: u32 = 256u32;
pub const ALINF_UPDHLPDLLS: u32 = 16u32;
pub type APPLICATION_RECOVERY_CALLBACK = ::core::option::Option<
    unsafe extern "system" fn(pv_parameter: MutPtr<::core::ffi::c_void>) -> u32,
>;
pub const ARSR_NOMESSAGES: u32 = 8u32;
pub const ARSR_REGSECTION: u32 = 128u32;
pub const ARSR_REMOVREGBKDATA: u32 = 4096u32;
pub const ARSR_RESTORE: u32 = 1u32;
pub const ATOM_FLAG_GLOBAL: u32 = 2u32;
pub const AT_ARP: u32 = 640u32;
pub const AT_NULL: u32 = 642u32;
pub const BACKUP_GHOSTED_FILE_EXTENTS: u32 = 11u32;
pub const BACKUP_INVALID: u32 = 0u32;
pub const BASE_SEARCH_PATH_DISABLE_SAFE_SEARCHMODE: u32 = 65536u32;
pub const BASE_SEARCH_PATH_ENABLE_SAFE_SEARCHMODE: u32 = 1u32;
pub const BASE_SEARCH_PATH_PERMANENT: u32 = 32768u32;
pub const BATTERY_FLAG_CHARGING: u32 = 8u32;
pub const BATTERY_FLAG_CRITICAL: u32 = 4u32;
pub const BATTERY_FLAG_HIGH: u32 = 1u32;
pub const BATTERY_FLAG_LOW: u32 = 2u32;
pub const BATTERY_FLAG_NO_BATTERY: u32 = 128u32;
pub const BATTERY_FLAG_UNKNOWN: u32 = 255u32;
pub const BATTERY_LIFE_UNKNOWN: u32 = 4294967295u32;
pub const BATTERY_PERCENTAGE_UNKNOWN: u32 = 255u32;
pub struct CABINFOA {
    pub pszCab: crate::core::PSTR,
    pub pszInf: crate::core::PSTR,
    pub pszSection: crate::core::PSTR,
    pub szSrcPath: [super::super::Foundation::CHAR; 260],
    pub dwFlags: u32,
}
impl ::core::marker::Copy for CABINFOA {}
impl ::core::clone::Clone for CABINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CABINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CABINFOA")
            .field("pszCab", &self.pszCab)
            .field("pszInf", &self.pszInf)
            .field("pszSection", &self.pszSection)
            .field("szSrcPath", &self.szSrcPath)
            .field("dwFlags", &self.dwFlags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CABINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.pszCab == other.pszCab
            && self.pszInf == other.pszInf
            && self.pszSection == other.pszSection
            && self.szSrcPath == other.szSrcPath
            && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for CABINFOA {}
impl FromIntoMemory for CABINFOA {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct CABINFOW {
    pub pszCab: crate::core::PWSTR,
    pub pszInf: crate::core::PWSTR,
    pub pszSection: crate::core::PWSTR,
    pub szSrcPath: [u16; 260],
    pub dwFlags: u32,
}
impl ::core::marker::Copy for CABINFOW {}
impl ::core::clone::Clone for CABINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CABINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CABINFOW")
            .field("pszCab", &self.pszCab)
            .field("pszInf", &self.pszInf)
            .field("pszSection", &self.pszSection)
            .field("szSrcPath", &self.szSrcPath)
            .field("dwFlags", &self.dwFlags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CABINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.pszCab == other.pszCab
            && self.pszInf == other.pszInf
            && self.pszSection == other.pszSection
            && self.szSrcPath == other.szSrcPath
            && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for CABINFOW {}
impl FromIntoMemory for CABINFOW {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub const CATID_DeleteBrowsingHistory: crate::core::GUID =
    crate::core::GUID::from_u128(0x31caf6e4_d6aa_4090_a050_a5ac8972e9ef);
pub const CBR_110: u32 = 110u32;
pub const CBR_115200: u32 = 115200u32;
pub const CBR_1200: u32 = 1200u32;
pub const CBR_128000: u32 = 128000u32;
pub const CBR_14400: u32 = 14400u32;
pub const CBR_19200: u32 = 19200u32;
pub const CBR_2400: u32 = 2400u32;
pub const CBR_256000: u32 = 256000u32;
pub const CBR_300: u32 = 300u32;
pub const CBR_38400: u32 = 38400u32;
pub const CBR_4800: u32 = 4800u32;
pub const CBR_56000: u32 = 56000u32;
pub const CBR_57600: u32 = 57600u32;
pub const CBR_600: u32 = 600u32;
pub const CBR_9600: u32 = 9600u32;
pub const CE_DNS: u32 = 2048u32;
pub const CE_IOE: u32 = 1024u32;
pub const CE_MODE: u32 = 32768u32;
pub const CE_OOP: u32 = 4096u32;
pub const CE_PTO: u32 = 512u32;
pub const CE_TXFULL: u32 = 256u32;
pub struct CLIENT_ID {
    pub UniqueProcess: super::super::Foundation::HANDLE,
    pub UniqueThread: super::super::Foundation::HANDLE,
}
impl ::core::marker::Copy for CLIENT_ID {}
impl ::core::clone::Clone for CLIENT_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLIENT_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLIENT_ID")
            .field("UniqueProcess", &self.UniqueProcess)
            .field("UniqueThread", &self.UniqueThread)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLIENT_ID {
    fn eq(&self, other: &Self) -> bool {
        self.UniqueProcess == other.UniqueProcess && self.UniqueThread == other.UniqueThread
    }
}
impl ::core::cmp::Eq for CLIENT_ID {}
impl FromIntoMemory for CLIENT_ID {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub const CL_NL_IP: u32 = 771u32;
pub const CL_NL_IPX: u32 = 769u32;
pub const CL_TL_NBF: u32 = 1025u32;
pub const CL_TL_UDP: u32 = 1027u32;
pub const CODEINTEGRITY_OPTION_DEBUGMODE_ENABLED: u32 = 128u32;
pub const CODEINTEGRITY_OPTION_ENABLED: u32 = 1u32;
pub const CODEINTEGRITY_OPTION_FLIGHTING_ENABLED: u32 = 512u32;
pub const CODEINTEGRITY_OPTION_FLIGHT_BUILD: u32 = 256u32;
pub const CODEINTEGRITY_OPTION_HVCI_IUM_ENABLED: u32 = 8192u32;
pub const CODEINTEGRITY_OPTION_HVCI_KMCI_AUDITMODE_ENABLED: u32 = 2048u32;
pub const CODEINTEGRITY_OPTION_HVCI_KMCI_ENABLED: u32 = 1024u32;
pub const CODEINTEGRITY_OPTION_HVCI_KMCI_STRICTMODE_ENABLED: u32 = 4096u32;
pub const CODEINTEGRITY_OPTION_PREPRODUCTION_BUILD: u32 = 64u32;
pub const CODEINTEGRITY_OPTION_TESTSIGN: u32 = 2u32;
pub const CODEINTEGRITY_OPTION_TEST_BUILD: u32 = 32u32;
pub const CODEINTEGRITY_OPTION_UMCI_AUDITMODE_ENABLED: u32 = 8u32;
pub const CODEINTEGRITY_OPTION_UMCI_ENABLED: u32 = 4u32;
pub const CODEINTEGRITY_OPTION_UMCI_EXCLUSIONPATHS_ENABLED: u32 = 16u32;
pub const CONTEXT_SIZE: u32 = 16u32;
pub const COPYFILE2_IO_CYCLE_SIZE_MAX: u32 = 1073741824u32;
pub const COPYFILE2_IO_CYCLE_SIZE_MIN: u32 = 4096u32;
pub const COPYFILE2_IO_RATE_MIN: u32 = 512u32;
pub const COPYFILE2_MESSAGE_COPY_OFFLOAD: i32 = 1i32;
pub const COPY_FILE_ALLOW_DECRYPTED_DESTINATION: u32 = 8u32;
pub const COPY_FILE_COPY_SYMLINK: u32 = 2048u32;
pub const COPY_FILE_DIRECTORY: u32 = 128u32;
pub const COPY_FILE_DISABLE_PRE_ALLOCATION: u32 = 67108864u32;
pub const COPY_FILE_DONT_REQUEST_DEST_WRITE_DAC: u32 = 33554432u32;
pub const COPY_FILE_ENABLE_LOW_FREE_SPACE_MODE: u32 = 134217728u32;
pub const COPY_FILE_FAIL_IF_EXISTS: u32 = 1u32;
pub const COPY_FILE_IGNORE_EDP_BLOCK: u32 = 4194304u32;
pub const COPY_FILE_IGNORE_SOURCE_ENCRYPTION: u32 = 8388608u32;
pub const COPY_FILE_NO_BUFFERING: u32 = 4096u32;
pub const COPY_FILE_NO_OFFLOAD: u32 = 262144u32;
pub const COPY_FILE_OPEN_AND_COPY_REPARSE_POINT: u32 = 2097152u32;
pub const COPY_FILE_OPEN_SOURCE_FOR_WRITE: u32 = 4u32;
pub const COPY_FILE_REQUEST_COMPRESSED_TRAFFIC: u32 = 268435456u32;
pub const COPY_FILE_REQUEST_SECURITY_PRIVILEGES: u32 = 8192u32;
pub const COPY_FILE_RESTARTABLE: u32 = 2u32;
pub const COPY_FILE_RESUME_FROM_PAUSE: u32 = 16384u32;
pub const COPY_FILE_SKIP_ALTERNATE_STREAMS: u32 = 32768u32;
pub const CO_TL_NBF: u32 = 1024u32;
pub const CO_TL_SPP: u32 = 1030u32;
pub const CO_TL_SPX: u32 = 1026u32;
pub const CO_TL_TCP: u32 = 1028u32;
pub const CP_DIRECT: u32 = 2u32;
pub const CP_HWND: u32 = 0u32;
pub const CP_LEVEL: u32 = 3u32;
pub const CP_OPEN: u32 = 1u32;
pub const CREATE_FOR_DIR: u32 = 2u32;
pub const CREATE_FOR_IMPORT: u32 = 1u32;
pub const CRITICAL_SECTION_NO_DEBUG_INFO: u32 = 16777216u32;
pub struct CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {
    pub Size: u32,
    pub TriggerId: crate::core::PCWSTR,
}
impl ::core::marker::Copy for CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {}
impl ::core::clone::Clone for CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG")
            .field("Size", &self.Size)
            .field("TriggerId", &self.TriggerId)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.TriggerId == other.TriggerId
    }
}
impl ::core::cmp::Eq for CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {}
impl FromIntoMemory for CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub const CameraUIControl: crate::core::GUID =
    crate::core::GUID::from_u128(0x16d5a2be_b1c5_47b3_8eae_ccbcf452c7e8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CameraUIControlCaptureMode(pub i32);
impl CameraUIControlCaptureMode {
    pub const PhotoOrVideo: Self = Self(0i32);
    pub const Photo: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl ::core::marker::Copy for CameraUIControlCaptureMode {}
impl ::core::clone::Clone for CameraUIControlCaptureMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraUIControlCaptureMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CameraUIControlCaptureMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraUIControlCaptureMode")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for CameraUIControlCaptureMode {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<i32>()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CameraUIControlLinearSelectionMode(pub i32);
impl CameraUIControlLinearSelectionMode {
    pub const Single: Self = Self(0i32);
    pub const Multiple: Self = Self(1i32);
}
impl ::core::marker::Copy for CameraUIControlLinearSelectionMode {}
impl ::core::clone::Clone for CameraUIControlLinearSelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraUIControlLinearSelectionMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CameraUIControlLinearSelectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraUIControlLinearSelectionMode")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for CameraUIControlLinearSelectionMode {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<i32>()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CameraUIControlMode(pub i32);
impl CameraUIControlMode {
    pub const Browse: Self = Self(0i32);
    pub const Linear: Self = Self(1i32);
}
impl ::core::marker::Copy for CameraUIControlMode {}
impl ::core::clone::Clone for CameraUIControlMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraUIControlMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CameraUIControlMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraUIControlMode").field(&self.0).finish()
    }
}
impl FromIntoMemory for CameraUIControlMode {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<i32>()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CameraUIControlPhotoFormat(pub i32);
impl CameraUIControlPhotoFormat {
    pub const Jpeg: Self = Self(0i32);
    pub const Png: Self = Self(1i32);
    pub const JpegXR: Self = Self(2i32);
}
impl ::core::marker::Copy for CameraUIControlPhotoFormat {}
impl ::core::clone::Clone for CameraUIControlPhotoFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraUIControlPhotoFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CameraUIControlPhotoFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraUIControlPhotoFormat")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for CameraUIControlPhotoFormat {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<i32>()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CameraUIControlVideoFormat(pub i32);
impl CameraUIControlVideoFormat {
    pub const Mp4: Self = Self(0i32);
    pub const Wmv: Self = Self(1i32);
}
impl ::core::marker::Copy for CameraUIControlVideoFormat {}
impl ::core::clone::Clone for CameraUIControlVideoFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraUIControlVideoFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CameraUIControlVideoFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraUIControlVideoFormat")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for CameraUIControlVideoFormat {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<i32>()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CameraUIControlViewType(pub i32);
impl CameraUIControlViewType {
    pub const SingleItem: Self = Self(0i32);
    pub const ItemList: Self = Self(1i32);
}
impl ::core::marker::Copy for CameraUIControlViewType {}
impl ::core::clone::Clone for CameraUIControlViewType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraUIControlViewType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CameraUIControlViewType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraUIControlViewType")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for CameraUIControlViewType {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<i32>()
    }
}
pub struct DATETIME {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub min: u16,
    pub sec: u16,
}
impl ::core::marker::Copy for DATETIME {}
impl ::core::clone::Clone for DATETIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DATETIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DATETIME")
            .field("year", &self.year)
            .field("month", &self.month)
            .field("day", &self.day)
            .field("hour", &self.hour)
            .field("min", &self.min)
            .field("sec", &self.sec)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DATETIME {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year
            && self.month == other.month
            && self.day == other.day
            && self.hour == other.hour
            && self.min == other.min
            && self.sec == other.sec
    }
}
impl ::core::cmp::Eq for DATETIME {}
impl FromIntoMemory for DATETIME {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct DCICMD {
    pub dwCommand: u32,
    pub dwParam1: u32,
    pub dwParam2: u32,
    pub dwVersion: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for DCICMD {}
impl ::core::clone::Clone for DCICMD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DCICMD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCICMD")
            .field("dwCommand", &self.dwCommand)
            .field("dwParam1", &self.dwParam1)
            .field("dwParam2", &self.dwParam2)
            .field("dwVersion", &self.dwVersion)
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DCICMD {
    fn eq(&self, other: &Self) -> bool {
        self.dwCommand == other.dwCommand
            && self.dwParam1 == other.dwParam1
            && self.dwParam2 == other.dwParam2
            && self.dwVersion == other.dwVersion
            && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for DCICMD {}
impl FromIntoMemory for DCICMD {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct DCICREATEINPUT {
    pub cmd: DCICMD,
    pub dwCompression: u32,
    pub dwMask: [u32; 3],
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwDCICaps: u32,
    pub dwBitCount: u32,
    pub lpSurface: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for DCICREATEINPUT {}
impl ::core::clone::Clone for DCICREATEINPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DCICREATEINPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCICREATEINPUT")
            .field("cmd", &self.cmd)
            .field("dwCompression", &self.dwCompression)
            .field("dwMask", &self.dwMask)
            .field("dwWidth", &self.dwWidth)
            .field("dwHeight", &self.dwHeight)
            .field("dwDCICaps", &self.dwDCICaps)
            .field("dwBitCount", &self.dwBitCount)
            .field("lpSurface", &self.lpSurface)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DCICREATEINPUT {
    fn eq(&self, other: &Self) -> bool {
        self.cmd == other.cmd
            && self.dwCompression == other.dwCompression
            && self.dwMask == other.dwMask
            && self.dwWidth == other.dwWidth
            && self.dwHeight == other.dwHeight
            && self.dwDCICaps == other.dwDCICaps
            && self.dwBitCount == other.dwBitCount
            && self.lpSurface == other.lpSurface
    }
}
impl ::core::cmp::Eq for DCICREATEINPUT {}
impl FromIntoMemory for DCICREATEINPUT {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub const DCICREATEOFFSCREENSURFACE: u32 = 2u32;
pub const DCICREATEOVERLAYSURFACE: u32 = 3u32;
pub const DCICREATEPRIMARYSURFACE: u32 = 1u32;
pub struct DCIENUMINPUT {
    pub cmd: DCICMD,
    pub rSrc: super::super::Foundation::RECT,
    pub rDst: super::super::Foundation::RECT,
    pub EnumCallback: PtrDiffRepr,
    pub lpContext: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for DCIENUMINPUT {}
impl ::core::clone::Clone for DCIENUMINPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DCIENUMINPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCIENUMINPUT")
            .field("cmd", &self.cmd)
            .field("rSrc", &self.rSrc)
            .field("rDst", &self.rDst)
            .field("EnumCallback", &self.EnumCallback)
            .field("lpContext", &self.lpContext)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DCIENUMINPUT {
    fn eq(&self, other: &Self) -> bool {
        self.cmd == other.cmd
            && self.rSrc == other.rSrc
            && self.rDst == other.rDst
            && self.EnumCallback == other.EnumCallback
            && self.lpContext == other.lpContext
    }
}
impl ::core::cmp::Eq for DCIENUMINPUT {}
impl FromIntoMemory for DCIENUMINPUT {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub const DCIENUMSURFACE: u32 = 4u32;
pub const DCIESCAPE: u32 = 5u32;
pub struct DCIOFFSCREEN {
    pub dciInfo: DCISURFACEINFO,
    pub Draw: PtrDiffRepr,
    pub SetClipList: PtrDiffRepr,
    pub SetDestination: PtrDiffRepr,
}
impl ::core::marker::Copy for DCIOFFSCREEN {}
impl ::core::clone::Clone for DCIOFFSCREEN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DCIOFFSCREEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCIOFFSCREEN")
            .field("dciInfo", &self.dciInfo)
            .field("Draw", &self.Draw)
            .field("SetClipList", &self.SetClipList)
            .field("SetDestination", &self.SetDestination)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DCIOFFSCREEN {
    fn eq(&self, other: &Self) -> bool {
        self.dciInfo == other.dciInfo
            && self.Draw == other.Draw
            && self.SetClipList == other.SetClipList
            && self.SetDestination == other.SetDestination
    }
}
impl ::core::cmp::Eq for DCIOFFSCREEN {}
impl FromIntoMemory for DCIOFFSCREEN {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct DCIOVERLAY {
    pub dciInfo: DCISURFACEINFO,
    pub dwChromakeyValue: u32,
    pub dwChromakeyMask: u32,
}
impl ::core::marker::Copy for DCIOVERLAY {}
impl ::core::clone::Clone for DCIOVERLAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DCIOVERLAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCIOVERLAY")
            .field("dciInfo", &self.dciInfo)
            .field("dwChromakeyValue", &self.dwChromakeyValue)
            .field("dwChromakeyMask", &self.dwChromakeyMask)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DCIOVERLAY {
    fn eq(&self, other: &Self) -> bool {
        self.dciInfo == other.dciInfo
            && self.dwChromakeyValue == other.dwChromakeyValue
            && self.dwChromakeyMask == other.dwChromakeyMask
    }
}
impl ::core::cmp::Eq for DCIOVERLAY {}
impl FromIntoMemory for DCIOVERLAY {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct DCISURFACEINFO {
    pub dwSize: u32,
    pub dwDCICaps: u32,
    pub dwCompression: u32,
    pub dwMask: [u32; 3],
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub lStride: i32,
    pub dwBitCount: u32,
    pub dwOffSurface: PtrRepr,
    pub wSelSurface: u16,
    pub wReserved: u16,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
    pub BeginAccess: PtrDiffRepr,
    pub EndAccess: PtrDiffRepr,
    pub DestroySurface: PtrDiffRepr,
}
impl ::core::marker::Copy for DCISURFACEINFO {}
impl ::core::clone::Clone for DCISURFACEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DCISURFACEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCISURFACEINFO")
            .field("dwSize", &self.dwSize)
            .field("dwDCICaps", &self.dwDCICaps)
            .field("dwCompression", &self.dwCompression)
            .field("dwMask", &self.dwMask)
            .field("dwWidth", &self.dwWidth)
            .field("dwHeight", &self.dwHeight)
            .field("lStride", &self.lStride)
            .field("dwBitCount", &self.dwBitCount)
            .field("dwOffSurface", &self.dwOffSurface)
            .field("wSelSurface", &self.wSelSurface)
            .field("wReserved", &self.wReserved)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("dwReserved3", &self.dwReserved3)
            .field("BeginAccess", &self.BeginAccess)
            .field("EndAccess", &self.EndAccess)
            .field("DestroySurface", &self.DestroySurface)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DCISURFACEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwDCICaps == other.dwDCICaps
            && self.dwCompression == other.dwCompression
            && self.dwMask == other.dwMask
            && self.dwWidth == other.dwWidth
            && self.dwHeight == other.dwHeight
            && self.lStride == other.lStride
            && self.dwBitCount == other.dwBitCount
            && self.dwOffSurface == other.dwOffSurface
            && self.wSelSurface == other.wSelSurface
            && self.wReserved == other.wReserved
            && self.dwReserved1 == other.dwReserved1
            && self.dwReserved2 == other.dwReserved2
            && self.dwReserved3 == other.dwReserved3
            && self.BeginAccess == other.BeginAccess
            && self.EndAccess == other.EndAccess
            && self.DestroySurface == other.DestroySurface
    }
}
impl ::core::cmp::Eq for DCISURFACEINFO {}
impl FromIntoMemory for DCISURFACEINFO {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub const DCI_1632_ACCESS: u32 = 64u32;
pub const DCI_ASYNC: u32 = 1024u32;
pub const DCI_CANOVERLAY: u32 = 65536u32;
pub const DCI_CAN_STRETCHX: u32 = 4096u32;
pub const DCI_CAN_STRETCHXN: u32 = 16384u32;
pub const DCI_CAN_STRETCHY: u32 = 8192u32;
pub const DCI_CAN_STRETCHYN: u32 = 32768u32;
pub const DCI_CHROMAKEY: u32 = 32u32;
pub const DCI_DWORDALIGN: u32 = 256u32;
pub const DCI_DWORDSIZE: u32 = 128u32;
pub const DCI_ERR_CURRENTLYNOTAVAIL: i32 = -5i32;
pub const DCI_ERR_HEIGHTALIGN: i32 = -21i32;
pub const DCI_ERR_INVALIDCLIPLIST: i32 = -15i32;
pub const DCI_ERR_INVALIDPOSITION: i32 = -13i32;
pub const DCI_ERR_INVALIDRECT: i32 = -6i32;
pub const DCI_ERR_INVALIDSTRETCH: i32 = -14i32;
pub const DCI_ERR_OUTOFMEMORY: i32 = -12i32;
pub const DCI_ERR_SURFACEISOBSCURED: i32 = -16i32;
pub const DCI_ERR_TOOBIGHEIGHT: i32 = -9i32;
pub const DCI_ERR_TOOBIGSIZE: i32 = -11i32;
pub const DCI_ERR_TOOBIGWIDTH: i32 = -10i32;
pub const DCI_ERR_UNSUPPORTEDFORMAT: i32 = -7i32;
pub const DCI_ERR_UNSUPPORTEDMASK: i32 = -8i32;
pub const DCI_ERR_WIDTHALIGN: i32 = -20i32;
pub const DCI_ERR_XALIGN: i32 = -17i32;
pub const DCI_ERR_XYALIGN: i32 = -19i32;
pub const DCI_ERR_YALIGN: i32 = -18i32;
pub const DCI_FAIL_GENERIC: i32 = -1i32;
pub const DCI_FAIL_INVALIDSURFACE: i32 = -3i32;
pub const DCI_FAIL_UNSUPPORTED: i32 = -4i32;
pub const DCI_FAIL_UNSUPPORTEDVERSION: i32 = -2i32;
pub const DCI_OFFSCREEN: u32 = 1u32;
pub const DCI_OK: u32 = 0u32;
pub const DCI_OVERLAY: u32 = 2u32;
pub const DCI_PRIMARY: u32 = 0u32;
pub const DCI_STATUS_CHROMAKEYCHANGED: u32 = 16u32;
pub const DCI_STATUS_FORMATCHANGED: u32 = 4u32;
pub const DCI_STATUS_POINTERCHANGED: u32 = 1u32;
pub const DCI_STATUS_STRIDECHANGED: u32 = 2u32;
pub const DCI_STATUS_SURFACEINFOCHANGED: u32 = 8u32;
pub const DCI_STATUS_WASSTILLDRAWING: u32 = 32u32;
pub const DCI_SURFACE_TYPE: u32 = 15u32;
pub const DCI_VERSION: u32 = 256u32;
pub const DCI_VISIBLE: u32 = 16u32;
pub const DCI_WRITEONLY: u32 = 512u32;
pub const DEACTIVATE_ACTCTX_FLAG_FORCE_EARLY_DEACTIVATION: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DECISION_LOCATION(pub i32);
pub const DECISION_LOCATION_REFRESH_GLOBAL_DATA: DECISION_LOCATION = DECISION_LOCATION(0i32);
pub const DECISION_LOCATION_PARAMETER_VALIDATION: DECISION_LOCATION = DECISION_LOCATION(1i32);
pub const DECISION_LOCATION_AUDIT: DECISION_LOCATION = DECISION_LOCATION(2i32);
pub const DECISION_LOCATION_FAILED_CONVERT_GUID: DECISION_LOCATION = DECISION_LOCATION(3i32);
pub const DECISION_LOCATION_ENTERPRISE_DEFINED_CLASS_ID: DECISION_LOCATION =
    DECISION_LOCATION(4i32);
pub const DECISION_LOCATION_GLOBAL_BUILT_IN_LIST: DECISION_LOCATION = DECISION_LOCATION(5i32);
pub const DECISION_LOCATION_PROVIDER_BUILT_IN_LIST: DECISION_LOCATION = DECISION_LOCATION(6i32);
pub const DECISION_LOCATION_ENFORCE_STATE_LIST: DECISION_LOCATION = DECISION_LOCATION(7i32);
pub const DECISION_LOCATION_NOT_FOUND: DECISION_LOCATION = DECISION_LOCATION(8i32);
pub const DECISION_LOCATION_UNKNOWN: DECISION_LOCATION = DECISION_LOCATION(9i32);
impl ::core::marker::Copy for DECISION_LOCATION {}
impl ::core::clone::Clone for DECISION_LOCATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DECISION_LOCATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DECISION_LOCATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DECISION_LOCATION").field(&self.0).finish()
    }
}
impl FromIntoMemory for DECISION_LOCATION {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<i32>()
    }
}
pub const DELAYLOAD_GPA_FAILURE: u32 = 4u32;
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct DELAYLOAD_INFO {
    pub Size: u32,
    pub DelayloadDescriptor: MutPtr<IMAGE_DELAYLOAD_DESCRIPTOR>,
    pub ThunkAddress: MutPtr<IMAGE_THUNK_DATA64>,
    pub TargetDllName: crate::core::PCSTR,
    pub TargetApiDescriptor: DELAYLOAD_PROC_DESCRIPTOR,
    pub TargetModuleBase: MutPtr<::core::ffi::c_void>,
    pub Unused: MutPtr<::core::ffi::c_void>,
    pub LastError: u32,
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for DELAYLOAD_INFO {}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for DELAYLOAD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for DELAYLOAD_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.DelayloadDescriptor == other.DelayloadDescriptor
            && self.ThunkAddress == other.ThunkAddress
            && self.TargetDllName == other.TargetDllName
            && self.TargetApiDescriptor == other.TargetApiDescriptor
            && self.TargetModuleBase == other.TargetModuleBase
            && self.Unused == other.Unused
            && self.LastError == other.LastError
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for DELAYLOAD_INFO {}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for DELAYLOAD_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct DELAYLOAD_INFO {
    pub Size: u32,
    pub DelayloadDescriptor: MutPtr<IMAGE_DELAYLOAD_DESCRIPTOR>,
    pub ThunkAddress: MutPtr<IMAGE_THUNK_DATA32>,
    pub TargetDllName: crate::core::PCSTR,
    pub TargetApiDescriptor: DELAYLOAD_PROC_DESCRIPTOR,
    pub TargetModuleBase: MutPtr<::core::ffi::c_void>,
    pub Unused: MutPtr<::core::ffi::c_void>,
    pub LastError: u32,
}
impl ::core::marker::Copy for DELAYLOAD_INFO {}
impl ::core::clone::Clone for DELAYLOAD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for DELAYLOAD_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.DelayloadDescriptor == other.DelayloadDescriptor
            && self.ThunkAddress == other.ThunkAddress
            && self.TargetDllName == other.TargetDllName
            && self.TargetApiDescriptor == other.TargetApiDescriptor
            && self.TargetModuleBase == other.TargetModuleBase
            && self.Unused == other.Unused
            && self.LastError == other.LastError
    }
}
impl ::core::cmp::Eq for DELAYLOAD_INFO {}
impl FromIntoMemory for DELAYLOAD_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct DELAYLOAD_PROC_DESCRIPTOR {
    pub ImportDescribedByName: u32,
    pub Description: DELAYLOAD_PROC_DESCRIPTOR_0,
}
impl ::core::marker::Copy for DELAYLOAD_PROC_DESCRIPTOR {}
impl ::core::clone::Clone for DELAYLOAD_PROC_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for DELAYLOAD_PROC_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.ImportDescribedByName == other.ImportDescribedByName
            && self.Description == other.Description
    }
}
impl ::core::cmp::Eq for DELAYLOAD_PROC_DESCRIPTOR {}
impl FromIntoMemory for DELAYLOAD_PROC_DESCRIPTOR {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct DELAYLOAD_PROC_DESCRIPTOR_0 {
    pub Name: crate::core::PCSTR,
    pub Ordinal: u32,
}
impl ::core::marker::Copy for DELAYLOAD_PROC_DESCRIPTOR_0 {}
impl ::core::clone::Clone for DELAYLOAD_PROC_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for DELAYLOAD_PROC_DESCRIPTOR_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Ordinal == other.Ordinal
    }
}
impl ::core::cmp::Eq for DELAYLOAD_PROC_DESCRIPTOR_0 {}
impl FromIntoMemory for DELAYLOAD_PROC_DESCRIPTOR_0 {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub const DELETE_BROWSING_HISTORY_COOKIES: u32 = 2u32;
pub const DELETE_BROWSING_HISTORY_DOWNLOADHISTORY: u32 = 64u32;
pub const DELETE_BROWSING_HISTORY_FORMDATA: u32 = 8u32;
pub const DELETE_BROWSING_HISTORY_HISTORY: u32 = 1u32;
pub const DELETE_BROWSING_HISTORY_PASSWORDS: u32 = 16u32;
pub const DELETE_BROWSING_HISTORY_PRESERVEFAVORITES: u32 = 32u32;
pub const DELETE_BROWSING_HISTORY_TIF: u32 = 4u32;
pub const DOCKINFO_DOCKED: u32 = 2u32;
pub const DOCKINFO_UNDOCKED: u32 = 1u32;
pub const DOCKINFO_USER_SUPPLIED: u32 = 4u32;
pub const DRIVE_CDROM: u32 = 5u32;
pub const DRIVE_FIXED: u32 = 3u32;
pub const DRIVE_NO_ROOT_DIR: u32 = 1u32;
pub const DRIVE_RAMDISK: u32 = 6u32;
pub const DRIVE_REMOTE: u32 = 4u32;
pub const DRIVE_REMOVABLE: u32 = 2u32;
pub const DRIVE_UNKNOWN: u32 = 0u32;
pub const DTR_CONTROL_DISABLE: u32 = 0u32;
pub const DTR_CONTROL_ENABLE: u32 = 1u32;
pub const DTR_CONTROL_HANDSHAKE: u32 = 2u32;
pub const DefaultBrowserSyncSettings: crate::core::GUID =
    crate::core::GUID::from_u128(0x3ac83423_3112_4aa6_9b5b_1feb23d0c5f9);
pub const EFSRPC_SECURE_ONLY: u32 = 8u32;
pub const EFS_DROP_ALTERNATE_STREAMS: u32 = 16u32;
pub const EFS_USE_RECOVERY_KEYS: u32 = 1u32;
pub const ENTITY_LIST_ID: u32 = 0u32;
pub const ENTITY_TYPE_ID: u32 = 1u32;
pub type ENUM_CALLBACK = ::core::option::Option<
    unsafe extern "system" fn(
        lp_surface_info: MutPtr<DCISURFACEINFO>,
        lp_context: MutPtr<::core::ffi::c_void>,
    ),
>;
pub const ER_ICMP: u32 = 896u32;
pub const EVENTLOG_FULL_INFO: u32 = 0u32;
pub const EditionUpgradeBroker: crate::core::GUID =
    crate::core::GUID::from_u128(0xc4270827_4f39_45df_9288_12ff6b85a921);
pub const EditionUpgradeHelper: crate::core::GUID =
    crate::core::GUID::from_u128(0x01776df3_b9af_4e50_9b1c_56e93116d704);
pub const FAIL_FAST_GENERATE_EXCEPTION_ADDRESS: u32 = 1u32;
pub const FAIL_FAST_NO_HARD_ERROR_DLG: u32 = 2u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FEATURE_CHANGE_TIME(pub i32);
pub const FEATURE_CHANGE_TIME_READ: FEATURE_CHANGE_TIME = FEATURE_CHANGE_TIME(0i32);
pub const FEATURE_CHANGE_TIME_MODULE_RELOAD: FEATURE_CHANGE_TIME = FEATURE_CHANGE_TIME(1i32);
pub const FEATURE_CHANGE_TIME_SESSION: FEATURE_CHANGE_TIME = FEATURE_CHANGE_TIME(2i32);
pub const FEATURE_CHANGE_TIME_REBOOT: FEATURE_CHANGE_TIME = FEATURE_CHANGE_TIME(3i32);
impl ::core::marker::Copy for FEATURE_CHANGE_TIME {}
impl ::core::clone::Clone for FEATURE_CHANGE_TIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEATURE_CHANGE_TIME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FEATURE_CHANGE_TIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEATURE_CHANGE_TIME").field(&self.0).finish()
    }
}
impl FromIntoMemory for FEATURE_CHANGE_TIME {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<i32>()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FEATURE_ENABLED_STATE(pub i32);
pub const FEATURE_ENABLED_STATE_DEFAULT: FEATURE_ENABLED_STATE = FEATURE_ENABLED_STATE(0i32);
pub const FEATURE_ENABLED_STATE_DISABLED: FEATURE_ENABLED_STATE = FEATURE_ENABLED_STATE(1i32);
pub const FEATURE_ENABLED_STATE_ENABLED: FEATURE_ENABLED_STATE = FEATURE_ENABLED_STATE(2i32);
impl ::core::marker::Copy for FEATURE_ENABLED_STATE {}
impl ::core::clone::Clone for FEATURE_ENABLED_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEATURE_ENABLED_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FEATURE_ENABLED_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEATURE_ENABLED_STATE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for FEATURE_ENABLED_STATE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<i32>()
    }
}
pub struct FEATURE_ERROR {
    pub hr: crate::core::HRESULT,
    pub lineNumber: u16,
    pub file: crate::core::PCSTR,
    pub process: crate::core::PCSTR,
    pub module: crate::core::PCSTR,
    pub callerReturnAddressOffset: u32,
    pub callerModule: crate::core::PCSTR,
    pub message: crate::core::PCSTR,
    pub originLineNumber: u16,
    pub originFile: crate::core::PCSTR,
    pub originModule: crate::core::PCSTR,
    pub originCallerReturnAddressOffset: u32,
    pub originCallerModule: crate::core::PCSTR,
    pub originName: crate::core::PCSTR,
}
impl ::core::marker::Copy for FEATURE_ERROR {}
impl ::core::clone::Clone for FEATURE_ERROR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FEATURE_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FEATURE_ERROR")
            .field("hr", &self.hr)
            .field("lineNumber", &self.lineNumber)
            .field("file", &self.file)
            .field("process", &self.process)
            .field("module", &self.module)
            .field("callerReturnAddressOffset", &self.callerReturnAddressOffset)
            .field("callerModule", &self.callerModule)
            .field("message", &self.message)
            .field("originLineNumber", &self.originLineNumber)
            .field("originFile", &self.originFile)
            .field("originModule", &self.originModule)
            .field(
                "originCallerReturnAddressOffset",
                &self.originCallerReturnAddressOffset,
            )
            .field("originCallerModule", &self.originCallerModule)
            .field("originName", &self.originName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FEATURE_ERROR {
    fn eq(&self, other: &Self) -> bool {
        self.hr == other.hr
            && self.lineNumber == other.lineNumber
            && self.file == other.file
            && self.process == other.process
            && self.module == other.module
            && self.callerReturnAddressOffset == other.callerReturnAddressOffset
            && self.callerModule == other.callerModule
            && self.message == other.message
            && self.originLineNumber == other.originLineNumber
            && self.originFile == other.originFile
            && self.originModule == other.originModule
            && self.originCallerReturnAddressOffset == other.originCallerReturnAddressOffset
            && self.originCallerModule == other.originCallerModule
            && self.originName == other.originName
    }
}
impl ::core::cmp::Eq for FEATURE_ERROR {}
impl FromIntoMemory for FEATURE_ERROR {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FEATURE_STATE_CHANGE_SUBSCRIPTION(pub PtrDiffRepr);
impl FEATURE_STATE_CHANGE_SUBSCRIPTION {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FEATURE_STATE_CHANGE_SUBSCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for FEATURE_STATE_CHANGE_SUBSCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for FEATURE_STATE_CHANGE_SUBSCRIPTION {}
impl ::core::fmt::Debug for FEATURE_STATE_CHANGE_SUBSCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEATURE_STATE_CHANGE_SUBSCRIPTION")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for FEATURE_STATE_CHANGE_SUBSCRIPTION {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<PtrDiffRepr as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<PtrDiffRepr>()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FH_SERVICE_PIPE_HANDLE(pub PtrDiffRepr);
impl FH_SERVICE_PIPE_HANDLE {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FH_SERVICE_PIPE_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for FH_SERVICE_PIPE_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for FH_SERVICE_PIPE_HANDLE {}
impl ::core::fmt::Debug for FH_SERVICE_PIPE_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FH_SERVICE_PIPE_HANDLE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for FH_SERVICE_PIPE_HANDLE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<PtrDiffRepr as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<PtrDiffRepr>()
    }
}
pub const FIBER_FLAG_FLOAT_SWITCH: u32 = 1u32;
pub struct FILE_CASE_SENSITIVE_INFO {
    pub Flags: u32,
}
impl ::core::marker::Copy for FILE_CASE_SENSITIVE_INFO {}
impl ::core::clone::Clone for FILE_CASE_SENSITIVE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_CASE_SENSITIVE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_CASE_SENSITIVE_INFO")
            .field("Flags", &self.Flags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILE_CASE_SENSITIVE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for FILE_CASE_SENSITIVE_INFO {}
impl FromIntoMemory for FILE_CASE_SENSITIVE_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub const FILE_COMPLETE_IF_OPLOCKED: u32 = 256u32;
pub const FILE_CREATED: u32 = 2u32;
pub const FILE_CREATE_TREE_CONNECTION: u32 = 128u32;
pub const FILE_DELETE_ON_CLOSE: u32 = 4096u32;
pub const FILE_DIRECTORY_FILE: u32 = 1u32;
pub const FILE_DIR_DISALLOWED: u32 = 9u32;
pub const FILE_DISPOSITION_FLAG_DELETE: u32 = 1u32;
pub const FILE_DISPOSITION_FLAG_DO_NOT_DELETE: u32 = 0u32;
pub const FILE_DISPOSITION_FLAG_FORCE_IMAGE_SECTION_CHECK: u32 = 4u32;
pub const FILE_DISPOSITION_FLAG_IGNORE_READONLY_ATTRIBUTE: u32 = 16u32;
pub const FILE_DISPOSITION_FLAG_ON_CLOSE: u32 = 8u32;
pub const FILE_DISPOSITION_FLAG_POSIX_SEMANTICS: u32 = 2u32;
pub struct FILE_DISPOSITION_INFO_EX {
    pub Flags: u32,
}
impl ::core::marker::Copy for FILE_DISPOSITION_INFO_EX {}
impl ::core::clone::Clone for FILE_DISPOSITION_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_DISPOSITION_INFO_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_DISPOSITION_INFO_EX")
            .field("Flags", &self.Flags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILE_DISPOSITION_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for FILE_DISPOSITION_INFO_EX {}
impl FromIntoMemory for FILE_DISPOSITION_INFO_EX {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub const FILE_DOES_NOT_EXIST: u32 = 5u32;
pub const FILE_ENCRYPTABLE: u32 = 0u32;
pub const FILE_EXISTS: u32 = 4u32;
pub const FILE_FLAG_OPEN_REQUIRING_OPLOCK: u32 = 262144u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FILE_INFORMATION_CLASS(pub i32);
pub const FileDirectoryInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(1i32);
impl ::core::marker::Copy for FILE_INFORMATION_CLASS {}
impl ::core::clone::Clone for FILE_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_INFORMATION_CLASS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for FILE_INFORMATION_CLASS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<i32>()
    }
}
pub const FILE_IS_ENCRYPTED: u32 = 1u32;
pub const FILE_MAXIMUM_DISPOSITION: u32 = 5u32;
pub const FILE_NON_DIRECTORY_FILE: u32 = 64u32;
pub const FILE_NO_COMPRESSION: u32 = 32768u32;
pub const FILE_NO_EA_KNOWLEDGE: u32 = 512u32;
pub const FILE_NO_INTERMEDIATE_BUFFERING: u32 = 8u32;
pub const FILE_OPENED: u32 = 1u32;
pub const FILE_OPEN_BY_FILE_ID: u32 = 8192u32;
pub const FILE_OPEN_FOR_BACKUP_INTENT: u32 = 16384u32;
pub const FILE_OPEN_FOR_FREE_SPACE_QUERY: u32 = 8388608u32;
pub const FILE_OPEN_NO_RECALL: u32 = 4194304u32;
pub const FILE_OPEN_REMOTE_INSTANCE: u32 = 1024u32;
pub const FILE_OPEN_REPARSE_POINT: u32 = 2097152u32;
pub const FILE_OPEN_REQUIRING_OPLOCK: u32 = 65536u32;
pub const FILE_OVERWRITTEN: u32 = 3u32;
pub const FILE_RANDOM_ACCESS: u32 = 2048u32;
pub const FILE_READ_ONLY: u32 = 8u32;
pub const FILE_RENAME_FLAG_POSIX_SEMANTICS: u32 = 2u32;
pub const FILE_RENAME_FLAG_REPLACE_IF_EXISTS: u32 = 1u32;
pub const FILE_RENAME_FLAG_SUPPRESS_PIN_STATE_INHERITANCE: u32 = 4u32;
pub const FILE_RESERVE_OPFILTER: u32 = 1048576u32;
pub const FILE_ROOT_DIR: u32 = 3u32;
pub const FILE_SEQUENTIAL_ONLY: u32 = 4u32;
pub const FILE_SKIP_COMPLETION_PORT_ON_SUCCESS: u32 = 1u32;
pub const FILE_SKIP_SET_EVENT_ON_HANDLE: u32 = 2u32;
pub const FILE_SUPERSEDED: u32 = 0u32;
pub const FILE_SYNCHRONOUS_IO_ALERT: u32 = 16u32;
pub const FILE_SYNCHRONOUS_IO_NONALERT: u32 = 32u32;
pub const FILE_SYSTEM_ATTR: u32 = 2u32;
pub const FILE_SYSTEM_DIR: u32 = 4u32;
pub const FILE_SYSTEM_NOT_SUPPORT: u32 = 6u32;
pub const FILE_TYPE_CHAR: u32 = 2u32;
pub const FILE_TYPE_DISK: u32 = 1u32;
pub const FILE_TYPE_PIPE: u32 = 3u32;
pub const FILE_TYPE_REMOTE: u32 = 32768u32;
pub const FILE_TYPE_UNKNOWN: u32 = 0u32;
pub const FILE_UNKNOWN: u32 = 5u32;
pub const FILE_USER_DISALLOWED: u32 = 7u32;
pub const FILE_VALID_MAILSLOT_OPTION_FLAGS: u32 = 50u32;
pub const FILE_VALID_OPTION_FLAGS: u32 = 16777215u32;
pub const FILE_VALID_PIPE_OPTION_FLAGS: u32 = 50u32;
pub const FILE_VALID_SET_FLAGS: u32 = 54u32;
pub const FILE_WRITE_THROUGH: u32 = 2u32;
pub const FIND_ACTCTX_SECTION_KEY_RETURN_ASSEMBLY_METADATA: u32 = 4u32;
pub const FIND_ACTCTX_SECTION_KEY_RETURN_FLAGS: u32 = 2u32;
pub const FIND_ACTCTX_SECTION_KEY_RETURN_HACTCTX: u32 = 1u32;
pub const FORMAT_MESSAGE_MAX_WIDTH_MASK: u32 = 255u32;
pub const FS_CASE_IS_PRESERVED: u32 = 2u32;
pub const FS_CASE_SENSITIVE: u32 = 1u32;
pub const FS_FILE_COMPRESSION: u32 = 16u32;
pub const FS_FILE_ENCRYPTION: u32 = 131072u32;
pub const FS_PERSISTENT_ACLS: u32 = 8u32;
pub const FS_UNICODE_STORED_ON_DISK: u32 = 4u32;
pub const FS_VOL_IS_COMPRESSED: u32 = 32768u32;
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_A_A: &'static str = "GetSystemWow64DirectoryA";
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_A_T: &'static str = "GetSystemWow64DirectoryA";
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_A_W: &'static str = "GetSystemWow64DirectoryA";
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_T_A: &'static str = "GetSystemWow64DirectoryW";
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_T_T: &'static str = "GetSystemWow64DirectoryW";
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_T_W: &'static str = "GetSystemWow64DirectoryW";
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_W_A: &'static str = "GetSystemWow64DirectoryW";
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_W_T: &'static str = "GetSystemWow64DirectoryW";
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_W_W: &'static str = "GetSystemWow64DirectoryW";
pub const GMEM_DDESHARE: u32 = 8192u32;
pub const GMEM_DISCARDABLE: u32 = 256u32;
pub const GMEM_DISCARDED: u32 = 16384u32;
pub const GMEM_INVALID_HANDLE: u32 = 32768u32;
pub const GMEM_LOCKCOUNT: u32 = 255u32;
pub const GMEM_LOWER: u32 = 4096u32;
pub const GMEM_MODIFY: u32 = 128u32;
pub const GMEM_NOCOMPACT: u32 = 16u32;
pub const GMEM_NODISCARD: u32 = 32u32;
pub const GMEM_NOTIFY: u32 = 16384u32;
pub const GMEM_NOT_BANKED: u32 = 4096u32;
pub const GMEM_SHARE: u32 = 8192u32;
pub const GMEM_VALID_FLAGS: u32 = 32626u32;
pub const HANJA_WINDOW: u32 = 2u32;
pub const HINSTANCE_ERROR: u32 = 32u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HWINWATCH(pub PtrDiffRepr);
impl HWINWATCH {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HWINWATCH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HWINWATCH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HWINWATCH {}
impl ::core::fmt::Debug for HWINWATCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HWINWATCH").field(&self.0).finish()
    }
}
impl FromIntoMemory for HWINWATCH {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<PtrDiffRepr as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<PtrDiffRepr>()
    }
}
pub const HW_PROFILE_GUIDLEN: u32 = 39u32;
pub struct HW_PROFILE_INFOA {
    pub dwDockInfo: u32,
    pub szHwProfileGuid: [super::super::Foundation::CHAR; 39],
    pub szHwProfileName: [super::super::Foundation::CHAR; 80],
}
impl ::core::marker::Copy for HW_PROFILE_INFOA {}
impl ::core::clone::Clone for HW_PROFILE_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HW_PROFILE_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HW_PROFILE_INFOA")
            .field("dwDockInfo", &self.dwDockInfo)
            .field("szHwProfileGuid", &self.szHwProfileGuid)
            .field("szHwProfileName", &self.szHwProfileName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for HW_PROFILE_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwDockInfo == other.dwDockInfo
            && self.szHwProfileGuid == other.szHwProfileGuid
            && self.szHwProfileName == other.szHwProfileName
    }
}
impl ::core::cmp::Eq for HW_PROFILE_INFOA {}
impl FromIntoMemory for HW_PROFILE_INFOA {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct HW_PROFILE_INFOW {
    pub dwDockInfo: u32,
    pub szHwProfileGuid: [u16; 39],
    pub szHwProfileName: [u16; 80],
}
impl ::core::marker::Copy for HW_PROFILE_INFOW {}
impl ::core::clone::Clone for HW_PROFILE_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HW_PROFILE_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HW_PROFILE_INFOW")
            .field("dwDockInfo", &self.dwDockInfo)
            .field("szHwProfileGuid", &self.szHwProfileGuid)
            .field("szHwProfileName", &self.szHwProfileName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for HW_PROFILE_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwDockInfo == other.dwDockInfo
            && self.szHwProfileGuid == other.szHwProfileGuid
            && self.szHwProfileName == other.szHwProfileName
    }
}
impl ::core::cmp::Eq for HW_PROFILE_INFOW {}
impl FromIntoMemory for HW_PROFILE_INFOW {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub const IE4_BACKNEW: u32 = 2u32;
pub const IE4_EXTRAINCREFCNT: u32 = 2048u32;
pub const IE4_FRDOALL: u32 = 256u32;
pub const IE4_NODELETENEW: u32 = 4u32;
pub const IE4_NOENUMKEY: u32 = 32u32;
pub const IE4_NOMESSAGES: u32 = 8u32;
pub const IE4_NOPROGRESS: u32 = 16u32;
pub const IE4_NO_CRC_MAPPING: u32 = 64u32;
pub const IE4_REGSECTION: u32 = 128u32;
pub const IE4_REMOVREGBKDATA: u32 = 4096u32;
pub const IE4_RESTORE: u32 = 1u32;
pub const IE4_UPDREFCNT: u32 = 512u32;
pub const IE4_USEREFCNT: u32 = 1024u32;
pub const IE_BADID: i32 = -1i32;
pub const IE_BAUDRATE: i32 = -12i32;
pub const IE_BYTESIZE: i32 = -11i32;
pub const IE_DEFAULT: i32 = -5i32;
pub const IE_HARDWARE: i32 = -10i32;
pub const IE_MEMORY: i32 = -4i32;
pub const IE_NOPEN: i32 = -3i32;
pub const IE_OPEN: i32 = -2i32;
pub const IF_GENERIC: u32 = 512u32;
pub const IF_MIB: u32 = 514u32;
pub const IGNORE: u32 = 0u32;
pub struct IMAGE_DELAYLOAD_DESCRIPTOR {
    pub Attributes: IMAGE_DELAYLOAD_DESCRIPTOR_0,
    pub DllNameRVA: u32,
    pub ModuleHandleRVA: u32,
    pub ImportAddressTableRVA: u32,
    pub ImportNameTableRVA: u32,
    pub BoundImportAddressTableRVA: u32,
    pub UnloadInformationTableRVA: u32,
    pub TimeDateStamp: u32,
}
impl ::core::marker::Copy for IMAGE_DELAYLOAD_DESCRIPTOR {}
impl ::core::clone::Clone for IMAGE_DELAYLOAD_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for IMAGE_DELAYLOAD_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Attributes == other.Attributes
            && self.DllNameRVA == other.DllNameRVA
            && self.ModuleHandleRVA == other.ModuleHandleRVA
            && self.ImportAddressTableRVA == other.ImportAddressTableRVA
            && self.ImportNameTableRVA == other.ImportNameTableRVA
            && self.BoundImportAddressTableRVA == other.BoundImportAddressTableRVA
            && self.UnloadInformationTableRVA == other.UnloadInformationTableRVA
            && self.TimeDateStamp == other.TimeDateStamp
    }
}
impl ::core::cmp::Eq for IMAGE_DELAYLOAD_DESCRIPTOR {}
impl FromIntoMemory for IMAGE_DELAYLOAD_DESCRIPTOR {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct IMAGE_DELAYLOAD_DESCRIPTOR_0 {
    pub AllAttributes: u32,
    pub Anonymous: IMAGE_DELAYLOAD_DESCRIPTOR_0_0,
}
impl ::core::marker::Copy for IMAGE_DELAYLOAD_DESCRIPTOR_0 {}
impl ::core::clone::Clone for IMAGE_DELAYLOAD_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for IMAGE_DELAYLOAD_DESCRIPTOR_0 {
    fn eq(&self, other: &Self) -> bool {
        self.AllAttributes == other.AllAttributes && self.Anonymous == other.Anonymous
    }
}
impl ::core::cmp::Eq for IMAGE_DELAYLOAD_DESCRIPTOR_0 {}
impl FromIntoMemory for IMAGE_DELAYLOAD_DESCRIPTOR_0 {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {}
impl ::core::clone::Clone for IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_DELAYLOAD_DESCRIPTOR_0_0")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {}
impl FromIntoMemory for IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct IMAGE_THUNK_DATA32 {
    pub u1: IMAGE_THUNK_DATA32_0,
}
impl ::core::marker::Copy for IMAGE_THUNK_DATA32 {}
impl ::core::clone::Clone for IMAGE_THUNK_DATA32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for IMAGE_THUNK_DATA32 {
    fn eq(&self, other: &Self) -> bool {
        self.u1 == other.u1
    }
}
impl ::core::cmp::Eq for IMAGE_THUNK_DATA32 {}
impl FromIntoMemory for IMAGE_THUNK_DATA32 {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct IMAGE_THUNK_DATA32_0 {
    pub ForwarderString: u32,
    pub Function: u32,
    pub Ordinal: u32,
    pub AddressOfData: u32,
}
impl ::core::marker::Copy for IMAGE_THUNK_DATA32_0 {}
impl ::core::clone::Clone for IMAGE_THUNK_DATA32_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for IMAGE_THUNK_DATA32_0 {
    fn eq(&self, other: &Self) -> bool {
        self.ForwarderString == other.ForwarderString
            && self.Function == other.Function
            && self.Ordinal == other.Ordinal
            && self.AddressOfData == other.AddressOfData
    }
}
impl ::core::cmp::Eq for IMAGE_THUNK_DATA32_0 {}
impl FromIntoMemory for IMAGE_THUNK_DATA32_0 {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct IMAGE_THUNK_DATA64 {
    pub u1: IMAGE_THUNK_DATA64_0,
}
impl ::core::marker::Copy for IMAGE_THUNK_DATA64 {}
impl ::core::clone::Clone for IMAGE_THUNK_DATA64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for IMAGE_THUNK_DATA64 {
    fn eq(&self, other: &Self) -> bool {
        self.u1 == other.u1
    }
}
impl ::core::cmp::Eq for IMAGE_THUNK_DATA64 {}
impl FromIntoMemory for IMAGE_THUNK_DATA64 {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct IMAGE_THUNK_DATA64_0 {
    pub ForwarderString: u64,
    pub Function: u64,
    pub Ordinal: u64,
    pub AddressOfData: u64,
}
impl ::core::marker::Copy for IMAGE_THUNK_DATA64_0 {}
impl ::core::clone::Clone for IMAGE_THUNK_DATA64_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for IMAGE_THUNK_DATA64_0 {
    fn eq(&self, other: &Self) -> bool {
        self.ForwarderString == other.ForwarderString
            && self.Function == other.Function
            && self.Ordinal == other.Ordinal
            && self.AddressOfData == other.AddressOfData
    }
}
impl ::core::cmp::Eq for IMAGE_THUNK_DATA64_0 {}
impl FromIntoMemory for IMAGE_THUNK_DATA64_0 {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub const IMEA_INIT: u32 = 1u32;
pub const IMEA_NEXT: u32 = 2u32;
pub const IMEA_PREV: u32 = 3u32;
pub struct IMEPROA {
    pub hWnd: super::super::Foundation::HWND,
    pub InstDate: DATETIME,
    pub wVersion: u32,
    pub szDescription: [u8; 50],
    pub szName: [u8; 80],
    pub szOptions: [u8; 30],
}
impl ::core::marker::Copy for IMEPROA {}
impl ::core::clone::Clone for IMEPROA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMEPROA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEPROA")
            .field("hWnd", &self.hWnd)
            .field("InstDate", &self.InstDate)
            .field("wVersion", &self.wVersion)
            .field("szDescription", &self.szDescription)
            .field("szName", &self.szName)
            .field("szOptions", &self.szOptions)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IMEPROA {
    fn eq(&self, other: &Self) -> bool {
        self.hWnd == other.hWnd
            && self.InstDate == other.InstDate
            && self.wVersion == other.wVersion
            && self.szDescription == other.szDescription
            && self.szName == other.szName
            && self.szOptions == other.szOptions
    }
}
impl ::core::cmp::Eq for IMEPROA {}
impl FromIntoMemory for IMEPROA {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct IMEPROW {
    pub hWnd: super::super::Foundation::HWND,
    pub InstDate: DATETIME,
    pub wVersion: u32,
    pub szDescription: [u16; 50],
    pub szName: [u16; 80],
    pub szOptions: [u16; 30],
}
impl ::core::marker::Copy for IMEPROW {}
impl ::core::clone::Clone for IMEPROW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMEPROW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEPROW")
            .field("hWnd", &self.hWnd)
            .field("InstDate", &self.InstDate)
            .field("wVersion", &self.wVersion)
            .field("szDescription", &self.szDescription)
            .field("szName", &self.szName)
            .field("szOptions", &self.szOptions)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IMEPROW {
    fn eq(&self, other: &Self) -> bool {
        self.hWnd == other.hWnd
            && self.InstDate == other.InstDate
            && self.wVersion == other.wVersion
            && self.szDescription == other.szDescription
            && self.szName == other.szName
            && self.szOptions == other.szOptions
    }
}
impl ::core::cmp::Eq for IMEPROW {}
impl FromIntoMemory for IMEPROW {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct IMESTRUCT {
    pub fnc: u32,
    pub wParam: super::super::Foundation::WPARAM,
    pub wCount: u32,
    pub dchSource: u32,
    pub dchDest: u32,
    pub lParam1: super::super::Foundation::LPARAM,
    pub lParam2: super::super::Foundation::LPARAM,
    pub lParam3: super::super::Foundation::LPARAM,
}
impl ::core::marker::Copy for IMESTRUCT {}
impl ::core::clone::Clone for IMESTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMESTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMESTRUCT")
            .field("fnc", &self.fnc)
            .field("wParam", &self.wParam)
            .field("wCount", &self.wCount)
            .field("dchSource", &self.dchSource)
            .field("dchDest", &self.dchDest)
            .field("lParam1", &self.lParam1)
            .field("lParam2", &self.lParam2)
            .field("lParam3", &self.lParam3)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IMESTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.fnc == other.fnc
            && self.wParam == other.wParam
            && self.wCount == other.wCount
            && self.dchSource == other.dchSource
            && self.dchDest == other.dchDest
            && self.lParam1 == other.lParam1
            && self.lParam2 == other.lParam2
            && self.lParam3 == other.lParam3
    }
}
impl ::core::cmp::Eq for IMESTRUCT {}
impl FromIntoMemory for IMESTRUCT {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub const IME_BANJAtoJUNJA: u32 = 19u32;
pub const IME_ENABLE_CONVERT: u32 = 2u32;
pub const IME_ENTERWORDREGISTERMODE: u32 = 24u32;
pub const IME_GETCONVERSIONMODE: u32 = 17u32;
pub const IME_GETIMECAPS: u32 = 3u32;
pub const IME_GETOPEN: u32 = 5u32;
pub const IME_GETVERSION: u32 = 7u32;
pub const IME_JOHABtoKS: u32 = 21u32;
pub const IME_JUNJAtoBANJA: u32 = 20u32;
pub const IME_KStoJOHAB: u32 = 22u32;
pub const IME_MAXPROCESS: u32 = 32u32;
pub const IME_MODE_ALPHANUMERIC: u32 = 1u32;
pub const IME_MODE_CODEINPUT: u32 = 128u32;
pub const IME_MODE_DBCSCHAR: u32 = 16u32;
pub const IME_MODE_HANJACONVERT: u32 = 4u32;
pub const IME_MODE_HIRAGANA: u32 = 4u32;
pub const IME_MODE_KATAKANA: u32 = 2u32;
pub const IME_MODE_NOCODEINPUT: u32 = 256u32;
pub const IME_MODE_NOROMAN: u32 = 64u32;
pub const IME_MODE_ROMAN: u32 = 32u32;
pub const IME_MODE_SBCSCHAR: u32 = 2u32;
pub const IME_MOVEIMEWINDOW: u32 = 8u32;
pub const IME_REQUEST_CONVERT: u32 = 1u32;
pub const IME_RS_DISKERROR: u32 = 14u32;
pub const IME_RS_ERROR: u32 = 1u32;
pub const IME_RS_ILLEGAL: u32 = 6u32;
pub const IME_RS_INVALID: u32 = 17u32;
pub const IME_RS_NEST: u32 = 18u32;
pub const IME_RS_NOIME: u32 = 2u32;
pub const IME_RS_NOROOM: u32 = 10u32;
pub const IME_RS_NOTFOUND: u32 = 7u32;
pub const IME_RS_SYSTEMMODAL: u32 = 19u32;
pub const IME_RS_TOOLONG: u32 = 5u32;
pub const IME_SENDVKEY: u32 = 19u32;
pub const IME_SETCONVERSIONFONTEX: u32 = 25u32;
pub const IME_SETCONVERSIONMODE: u32 = 16u32;
pub const IME_SETCONVERSIONWINDOW: u32 = 8u32;
pub const IME_SETOPEN: u32 = 4u32;
pub const IME_SET_MODE: u32 = 18u32;
pub const INFINITE: u32 = 4294967295u32;
pub const INFO_CLASS_GENERIC: u32 = 256u32;
pub const INFO_CLASS_IMPLEMENTATION: u32 = 768u32;
pub const INFO_CLASS_PROTOCOL: u32 = 512u32;
pub const INFO_TYPE_ADDRESS_OBJECT: u32 = 512u32;
pub const INFO_TYPE_CONNECTION: u32 = 768u32;
pub const INFO_TYPE_PROVIDER: u32 = 256u32;
pub const INTERIM_WINDOW: u32 = 0u32;
pub const INVALID_ENTITY_INSTANCE: i32 = -1i32;
pub const IOCTL_TDI_TL_IO_CONTROL_ENDPOINT: u32 = 2162744u32;
pub struct IO_STATUS_BLOCK {
    pub Anonymous: IO_STATUS_BLOCK_0,
    pub Information: PtrRepr,
}
impl ::core::marker::Copy for IO_STATUS_BLOCK {}
impl ::core::clone::Clone for IO_STATUS_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for IO_STATUS_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.Anonymous == other.Anonymous && self.Information == other.Information
    }
}
impl ::core::cmp::Eq for IO_STATUS_BLOCK {}
impl FromIntoMemory for IO_STATUS_BLOCK {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct IO_STATUS_BLOCK_0 {
    pub Status: super::super::Foundation::NTSTATUS,
    pub Pointer: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for IO_STATUS_BLOCK_0 {}
impl ::core::clone::Clone for IO_STATUS_BLOCK_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for IO_STATUS_BLOCK_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Status == other.Status && self.Pointer == other.Pointer
    }
}
impl ::core::cmp::Eq for IO_STATUS_BLOCK_0 {}
impl FromIntoMemory for IO_STATUS_BLOCK_0 {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub const IR_CHANGECONVERT: u32 = 289u32;
pub const IR_CLOSECONVERT: u32 = 290u32;
pub const IR_DBCSCHAR: u32 = 352u32;
pub const IR_FULLCONVERT: u32 = 291u32;
pub const IR_IMESELECT: u32 = 304u32;
pub const IR_MODEINFO: u32 = 400u32;
pub const IR_OPENCONVERT: u32 = 288u32;
pub const IR_STRING: u32 = 320u32;
pub const IR_STRINGEND: u32 = 257u32;
pub const IR_STRINGEX: u32 = 384u32;
pub const IR_STRINGSTART: u32 = 256u32;
pub const IR_UNDETERMINE: u32 = 368u32;
pub struct JAVA_TRUST {
    pub cbSize: u32,
    pub flag: u32,
    pub fAllActiveXPermissions: super::super::Foundation::BOOL,
    pub fAllPermissions: super::super::Foundation::BOOL,
    pub dwEncodingType: u32,
    pub pbJavaPermissions: MutPtr<u8>,
    pub cbJavaPermissions: u32,
    pub pbSigner: MutPtr<u8>,
    pub cbSigner: u32,
    pub pwszZone: crate::core::PCWSTR,
    pub guidZone: crate::core::GUID,
    pub hVerify: crate::core::HRESULT,
}
impl ::core::marker::Copy for JAVA_TRUST {}
impl ::core::clone::Clone for JAVA_TRUST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JAVA_TRUST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JAVA_TRUST")
            .field("cbSize", &self.cbSize)
            .field("flag", &self.flag)
            .field("fAllActiveXPermissions", &self.fAllActiveXPermissions)
            .field("fAllPermissions", &self.fAllPermissions)
            .field("dwEncodingType", &self.dwEncodingType)
            .field("pbJavaPermissions", &self.pbJavaPermissions)
            .field("cbJavaPermissions", &self.cbJavaPermissions)
            .field("pbSigner", &self.pbSigner)
            .field("cbSigner", &self.cbSigner)
            .field("pwszZone", &self.pwszZone)
            .field("guidZone", &self.guidZone)
            .field("hVerify", &self.hVerify)
            .finish()
    }
}
impl ::core::cmp::PartialEq for JAVA_TRUST {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.flag == other.flag
            && self.fAllActiveXPermissions == other.fAllActiveXPermissions
            && self.fAllPermissions == other.fAllPermissions
            && self.dwEncodingType == other.dwEncodingType
            && self.pbJavaPermissions == other.pbJavaPermissions
            && self.cbJavaPermissions == other.cbJavaPermissions
            && self.pbSigner == other.pbSigner
            && self.cbSigner == other.cbSigner
            && self.pwszZone == other.pwszZone
            && self.guidZone == other.guidZone
            && self.hVerify == other.hVerify
    }
}
impl ::core::cmp::Eq for JAVA_TRUST {}
impl FromIntoMemory for JAVA_TRUST {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct JIT_DEBUG_INFO {
    pub dwSize: u32,
    pub dwProcessorArchitecture: u32,
    pub dwThreadID: u32,
    pub dwReserved0: u32,
    pub lpExceptionAddress: u64,
    pub lpExceptionRecord: u64,
    pub lpContextRecord: u64,
}
impl ::core::marker::Copy for JIT_DEBUG_INFO {}
impl ::core::clone::Clone for JIT_DEBUG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JIT_DEBUG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JIT_DEBUG_INFO")
            .field("dwSize", &self.dwSize)
            .field("dwProcessorArchitecture", &self.dwProcessorArchitecture)
            .field("dwThreadID", &self.dwThreadID)
            .field("dwReserved0", &self.dwReserved0)
            .field("lpExceptionAddress", &self.lpExceptionAddress)
            .field("lpExceptionRecord", &self.lpExceptionRecord)
            .field("lpContextRecord", &self.lpContextRecord)
            .finish()
    }
}
impl ::core::cmp::PartialEq for JIT_DEBUG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwProcessorArchitecture == other.dwProcessorArchitecture
            && self.dwThreadID == other.dwThreadID
            && self.dwReserved0 == other.dwReserved0
            && self.lpExceptionAddress == other.lpExceptionAddress
            && self.lpExceptionRecord == other.lpExceptionRecord
            && self.lpContextRecord == other.lpContextRecord
    }
}
impl ::core::cmp::Eq for JIT_DEBUG_INFO {}
impl FromIntoMemory for JIT_DEBUG_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct KEY_SET_INFORMATION_CLASS(pub i32);
pub const KeyWriteTimeInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(0i32);
pub const KeyWow64FlagsInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(1i32);
pub const KeyControlFlagsInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(2i32);
pub const KeySetVirtualizationInformation: KEY_SET_INFORMATION_CLASS =
    KEY_SET_INFORMATION_CLASS(3i32);
pub const KeySetDebugInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(4i32);
pub const KeySetHandleTagsInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(5i32);
pub const MaxKeySetInfoClass: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(6i32);
impl ::core::marker::Copy for KEY_SET_INFORMATION_CLASS {}
impl ::core::clone::Clone for KEY_SET_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KEY_SET_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KEY_SET_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KEY_SET_INFORMATION_CLASS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for KEY_SET_INFORMATION_CLASS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<i32>()
    }
}
pub struct KEY_VALUE_ENTRY {
    pub ValueName: MutPtr<super::super::Foundation::UNICODE_STRING>,
    pub DataLength: u32,
    pub DataOffset: u32,
    pub Type: u32,
}
impl ::core::marker::Copy for KEY_VALUE_ENTRY {}
impl ::core::clone::Clone for KEY_VALUE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KEY_VALUE_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KEY_VALUE_ENTRY")
            .field("ValueName", &self.ValueName)
            .field("DataLength", &self.DataLength)
            .field("DataOffset", &self.DataOffset)
            .field("Type", &self.Type)
            .finish()
    }
}
impl ::core::cmp::PartialEq for KEY_VALUE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.ValueName == other.ValueName
            && self.DataLength == other.DataLength
            && self.DataOffset == other.DataOffset
            && self.Type == other.Type
    }
}
impl ::core::cmp::Eq for KEY_VALUE_ENTRY {}
impl FromIntoMemory for KEY_VALUE_ENTRY {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct LDR_DATA_TABLE_ENTRY {
    pub Reserved1: [MutPtr<::core::ffi::c_void>; 2],
    pub InMemoryOrderLinks: super::Kernel::LIST_ENTRY,
    pub Reserved2: [MutPtr<::core::ffi::c_void>; 2],
    pub DllBase: MutPtr<::core::ffi::c_void>,
    pub Reserved3: [MutPtr<::core::ffi::c_void>; 2],
    pub FullDllName: super::super::Foundation::UNICODE_STRING,
    pub Reserved4: [u8; 8],
    pub Reserved5: [MutPtr<::core::ffi::c_void>; 3],
    pub Anonymous: LDR_DATA_TABLE_ENTRY_0,
    pub TimeDateStamp: u32,
}
impl ::core::marker::Copy for LDR_DATA_TABLE_ENTRY {}
impl ::core::clone::Clone for LDR_DATA_TABLE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for LDR_DATA_TABLE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
            && self.InMemoryOrderLinks == other.InMemoryOrderLinks
            && self.Reserved2 == other.Reserved2
            && self.DllBase == other.DllBase
            && self.Reserved3 == other.Reserved3
            && self.FullDllName == other.FullDllName
            && self.Reserved4 == other.Reserved4
            && self.Reserved5 == other.Reserved5
            && self.Anonymous == other.Anonymous
            && self.TimeDateStamp == other.TimeDateStamp
    }
}
impl ::core::cmp::Eq for LDR_DATA_TABLE_ENTRY {}
impl FromIntoMemory for LDR_DATA_TABLE_ENTRY {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct LDR_DATA_TABLE_ENTRY_0 {
    pub CheckSum: u32,
    pub Reserved6: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for LDR_DATA_TABLE_ENTRY_0 {}
impl ::core::clone::Clone for LDR_DATA_TABLE_ENTRY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for LDR_DATA_TABLE_ENTRY_0 {
    fn eq(&self, other: &Self) -> bool {
        self.CheckSum == other.CheckSum && self.Reserved6 == other.Reserved6
    }
}
impl ::core::cmp::Eq for LDR_DATA_TABLE_ENTRY_0 {}
impl FromIntoMemory for LDR_DATA_TABLE_ENTRY_0 {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub const LIS_NOGRPCONV: u32 = 2u32;
pub const LIS_QUIET: u32 = 1u32;
pub const LOGON32_PROVIDER_VIRTUAL: u32 = 4u32;
pub const LOGON32_PROVIDER_WINNT35: u32 = 1u32;
pub const LOGON_ZERO_PASSWORD_BUFFER: u32 = 2147483648u32;
pub const LPTx: u32 = 128u32;
pub const MAXINTATOM: u32 = 49152u32;
pub const MAX_COMPUTERNAME_LENGTH: u32 = 15u32;
pub const MAX_TDI_ENTITIES: u32 = 4096u32;
pub const MCW_DEFAULT: u32 = 0u32;
pub const MCW_HIDDEN: u32 = 16u32;
pub const MCW_RECT: u32 = 1u32;
pub const MCW_SCREEN: u32 = 4u32;
pub const MCW_VERTICAL: u32 = 8u32;
pub const MCW_WINDOW: u32 = 2u32;
pub const MICROSOFT_WINBASE_H_DEFINE_INTERLOCKED_CPLUSPLUS_OVERLOADS: u32 = 0u32;
pub const MICROSOFT_WINDOWS_WINBASE_H_DEFINE_INTERLOCKED_CPLUSPLUS_OVERLOADS: u32 = 0u32;
pub const MODE_WINDOW: u32 = 1u32;
pub struct OBJECT_ATTRIBUTES {
    pub Length: u32,
    pub RootDirectory: super::super::Foundation::HANDLE,
    pub ObjectName: MutPtr<super::super::Foundation::UNICODE_STRING>,
    pub Attributes: u32,
    pub SecurityDescriptor: MutPtr<::core::ffi::c_void>,
    pub SecurityQualityOfService: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for OBJECT_ATTRIBUTES {}
impl ::core::clone::Clone for OBJECT_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OBJECT_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OBJECT_ATTRIBUTES")
            .field("Length", &self.Length)
            .field("RootDirectory", &self.RootDirectory)
            .field("ObjectName", &self.ObjectName)
            .field("Attributes", &self.Attributes)
            .field("SecurityDescriptor", &self.SecurityDescriptor)
            .field("SecurityQualityOfService", &self.SecurityQualityOfService)
            .finish()
    }
}
impl ::core::cmp::PartialEq for OBJECT_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
            && self.RootDirectory == other.RootDirectory
            && self.ObjectName == other.ObjectName
            && self.Attributes == other.Attributes
            && self.SecurityDescriptor == other.SecurityDescriptor
            && self.SecurityQualityOfService == other.SecurityQualityOfService
    }
}
impl ::core::cmp::Eq for OBJECT_ATTRIBUTES {}
impl FromIntoMemory for OBJECT_ATTRIBUTES {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct OBJECT_INFORMATION_CLASS(pub i32);
pub const ObjectBasicInformation: OBJECT_INFORMATION_CLASS = OBJECT_INFORMATION_CLASS(0i32);
pub const ObjectTypeInformation: OBJECT_INFORMATION_CLASS = OBJECT_INFORMATION_CLASS(2i32);
impl ::core::marker::Copy for OBJECT_INFORMATION_CLASS {}
impl ::core::clone::Clone for OBJECT_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OBJECT_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OBJECT_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OBJECT_INFORMATION_CLASS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for OBJECT_INFORMATION_CLASS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<i32>()
    }
}
pub const OFS_MAXPATHNAME: u32 = 128u32;
pub const OPERATION_API_VERSION: u32 = 1u32;
pub const OVERWRITE_HIDDEN: u32 = 4u32;
pub type PDELAYLOAD_FAILURE_DLL_CALLBACK = ::core::option::Option<
    unsafe extern "system" fn(
        notification_reason: u32,
        delayload_info: ConstPtr<DELAYLOAD_INFO>,
    ) -> MutPtr<::core::ffi::c_void>,
>;
pub struct PERUSERSECTIONA {
    pub szGUID: [super::super::Foundation::CHAR; 59],
    pub szDispName: [super::super::Foundation::CHAR; 128],
    pub szLocale: [super::super::Foundation::CHAR; 10],
    pub szStub: [super::super::Foundation::CHAR; 1040],
    pub szVersion: [super::super::Foundation::CHAR; 32],
    pub szCompID: [super::super::Foundation::CHAR; 128],
    pub dwIsInstalled: u32,
    pub bRollback: super::super::Foundation::BOOL,
}
impl ::core::marker::Copy for PERUSERSECTIONA {}
impl ::core::clone::Clone for PERUSERSECTIONA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERUSERSECTIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERUSERSECTIONA")
            .field("szGUID", &self.szGUID)
            .field("szDispName", &self.szDispName)
            .field("szLocale", &self.szLocale)
            .field("szStub", &self.szStub)
            .field("szVersion", &self.szVersion)
            .field("szCompID", &self.szCompID)
            .field("dwIsInstalled", &self.dwIsInstalled)
            .field("bRollback", &self.bRollback)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PERUSERSECTIONA {
    fn eq(&self, other: &Self) -> bool {
        self.szGUID == other.szGUID
            && self.szDispName == other.szDispName
            && self.szLocale == other.szLocale
            && self.szStub == other.szStub
            && self.szVersion == other.szVersion
            && self.szCompID == other.szCompID
            && self.dwIsInstalled == other.dwIsInstalled
            && self.bRollback == other.bRollback
    }
}
impl ::core::cmp::Eq for PERUSERSECTIONA {}
impl FromIntoMemory for PERUSERSECTIONA {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct PERUSERSECTIONW {
    pub szGUID: [u16; 59],
    pub szDispName: [u16; 128],
    pub szLocale: [u16; 10],
    pub szStub: [u16; 1040],
    pub szVersion: [u16; 32],
    pub szCompID: [u16; 128],
    pub dwIsInstalled: u32,
    pub bRollback: super::super::Foundation::BOOL,
}
impl ::core::marker::Copy for PERUSERSECTIONW {}
impl ::core::clone::Clone for PERUSERSECTIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERUSERSECTIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERUSERSECTIONW")
            .field("szGUID", &self.szGUID)
            .field("szDispName", &self.szDispName)
            .field("szLocale", &self.szLocale)
            .field("szStub", &self.szStub)
            .field("szVersion", &self.szVersion)
            .field("szCompID", &self.szCompID)
            .field("dwIsInstalled", &self.dwIsInstalled)
            .field("bRollback", &self.bRollback)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PERUSERSECTIONW {
    fn eq(&self, other: &Self) -> bool {
        self.szGUID == other.szGUID
            && self.szDispName == other.szDispName
            && self.szLocale == other.szLocale
            && self.szStub == other.szStub
            && self.szVersion == other.szVersion
            && self.szCompID == other.szCompID
            && self.dwIsInstalled == other.dwIsInstalled
            && self.bRollback == other.bRollback
    }
}
impl ::core::cmp::Eq for PERUSERSECTIONW {}
impl FromIntoMemory for PERUSERSECTIONW {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub type PFEATURE_STATE_CHANGE_CALLBACK =
    ::core::option::Option<unsafe extern "system" fn(context: ConstPtr<::core::ffi::c_void>)>;
pub type PFIBER_CALLOUT_ROUTINE = ::core::option::Option<
    unsafe extern "system" fn(
        lp_parameter: MutPtr<::core::ffi::c_void>,
    ) -> MutPtr<::core::ffi::c_void>,
>;
pub type PIO_APC_ROUTINE = ::core::option::Option<
    unsafe extern "system" fn(
        apc_context: MutPtr<::core::ffi::c_void>,
        io_status_block: MutPtr<IO_STATUS_BLOCK>,
        reserved: u32,
    ),
>;
pub type PQUERYACTCTXW_FUNC = ::core::option::Option<
    unsafe extern "system" fn(
        dw_flags: u32,
        h_act_ctx: super::super::Foundation::HANDLE,
        pv_sub_instance: ConstPtr<::core::ffi::c_void>,
        ul_info_class: u32,
        pv_buffer: MutPtr<::core::ffi::c_void>,
        cb_buffer: PtrRepr,
        pcb_written_or_required: MutPtr<PtrRepr>,
    ) -> super::super::Foundation::BOOL,
>;
pub const PROCESS_CREATION_ALL_APPLICATION_PACKAGES_OPT_OUT: u32 = 1u32;
pub const PROCESS_CREATION_CHILD_PROCESS_OVERRIDE: u32 = 2u32;
pub const PROCESS_CREATION_CHILD_PROCESS_RESTRICTED: u32 = 1u32;
pub const PROCESS_CREATION_CHILD_PROCESS_RESTRICTED_UNLESS_SECURE: u32 = 4u32;
pub const PROCESS_CREATION_DESKTOP_APP_BREAKAWAY_DISABLE_PROCESS_TREE: u32 = 2u32;
pub const PROCESS_CREATION_DESKTOP_APP_BREAKAWAY_ENABLE_PROCESS_TREE: u32 = 1u32;
pub const PROCESS_CREATION_DESKTOP_APP_BREAKAWAY_OVERRIDE: u32 = 4u32;
pub const PROCESS_CREATION_MITIGATION_POLICY_DEP_ATL_THUNK_ENABLE: u32 = 2u32;
pub const PROCESS_CREATION_MITIGATION_POLICY_DEP_ENABLE: u32 = 1u32;
pub const PROCESS_CREATION_MITIGATION_POLICY_SEHOP_ENABLE: u32 = 4u32;
pub const PROC_THREAD_ATTRIBUTE_ADDITIVE: u32 = 262144u32;
pub const PROC_THREAD_ATTRIBUTE_INPUT: u32 = 131072u32;
pub const PROC_THREAD_ATTRIBUTE_NUMBER: u32 = 65535u32;
pub const PROC_THREAD_ATTRIBUTE_THREAD: u32 = 65536u32;
pub const PROGRESS_CANCEL: u32 = 1u32;
pub const PROGRESS_CONTINUE: u32 = 0u32;
pub const PROGRESS_QUIET: u32 = 3u32;
pub const PROGRESS_STOP: u32 = 2u32;
pub const PROTECTION_LEVEL_SAME: u32 = 4294967295u32;
pub struct PUBLIC_OBJECT_BASIC_INFORMATION {
    pub Attributes: u32,
    pub GrantedAccess: u32,
    pub HandleCount: u32,
    pub PointerCount: u32,
    pub Reserved: [u32; 10],
}
impl ::core::marker::Copy for PUBLIC_OBJECT_BASIC_INFORMATION {}
impl ::core::clone::Clone for PUBLIC_OBJECT_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PUBLIC_OBJECT_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PUBLIC_OBJECT_BASIC_INFORMATION")
            .field("Attributes", &self.Attributes)
            .field("GrantedAccess", &self.GrantedAccess)
            .field("HandleCount", &self.HandleCount)
            .field("PointerCount", &self.PointerCount)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PUBLIC_OBJECT_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Attributes == other.Attributes
            && self.GrantedAccess == other.GrantedAccess
            && self.HandleCount == other.HandleCount
            && self.PointerCount == other.PointerCount
            && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for PUBLIC_OBJECT_BASIC_INFORMATION {}
impl FromIntoMemory for PUBLIC_OBJECT_BASIC_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct PUBLIC_OBJECT_TYPE_INFORMATION {
    pub TypeName: super::super::Foundation::UNICODE_STRING,
    pub Reserved: [u32; 22],
}
impl ::core::marker::Copy for PUBLIC_OBJECT_TYPE_INFORMATION {}
impl ::core::clone::Clone for PUBLIC_OBJECT_TYPE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PUBLIC_OBJECT_TYPE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PUBLIC_OBJECT_TYPE_INFORMATION")
            .field("TypeName", &self.TypeName)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PUBLIC_OBJECT_TYPE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.TypeName == other.TypeName && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for PUBLIC_OBJECT_TYPE_INFORMATION {}
impl FromIntoMemory for PUBLIC_OBJECT_TYPE_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub type PWINSTATIONQUERYINFORMATIONW = ::core::option::Option<
    unsafe extern "system" fn(
        param_0: super::super::Foundation::HANDLE,
        param_1: u32,
        param_2: WINSTATIONINFOCLASS,
        param_3: MutPtr<::core::ffi::c_void>,
        param_4: u32,
        param_5: MutPtr<u32>,
    ) -> super::super::Foundation::BOOLEAN,
>;
pub type PWLDP_ISAPPAPPROVEDBYPOLICY_API = ::core::option::Option<
    unsafe extern "system" fn(
        package_family_name: crate::core::PCWSTR,
        package_version: u64,
    ) -> crate::core::HRESULT,
>;
pub type PWLDP_ISDYNAMICCODEPOLICYENABLED_API = ::core::option::Option<
    unsafe extern "system" fn(
        pb_enabled: MutPtr<super::super::Foundation::BOOL>,
    ) -> crate::core::HRESULT,
>;
pub type PWLDP_ISPRODUCTIONCONFIGURATION_API = ::core::option::Option<
    unsafe extern "system" fn(
        is_production_configuration: MutPtr<super::super::Foundation::BOOL>,
    ) -> crate::core::HRESULT,
>;
pub type PWLDP_ISWCOSPRODUCTIONCONFIGURATION_API = ::core::option::Option<
    unsafe extern "system" fn(
        is_production_configuration: MutPtr<super::super::Foundation::BOOL>,
    ) -> crate::core::HRESULT,
>;
pub type PWLDP_QUERYDEVICESECURITYINFORMATION_API = ::core::option::Option<
    unsafe extern "system" fn(
        information: MutPtr<WLDP_DEVICE_SECURITY_INFORMATION>,
        information_length: u32,
        return_length: MutPtr<u32>,
    ) -> crate::core::HRESULT,
>;
pub type PWLDP_QUERYDYNAMICODETRUST_API = ::core::option::Option<
    unsafe extern "system" fn(
        file_handle: super::super::Foundation::HANDLE,
        base_image: ConstPtr<::core::ffi::c_void>,
        image_size: u32,
    ) -> crate::core::HRESULT,
>;
pub type PWLDP_QUERYPOLICYSETTINGENABLED2_API = ::core::option::Option<
    unsafe extern "system" fn(
        setting: crate::core::PCWSTR,
        enabled: MutPtr<super::super::Foundation::BOOL>,
    ) -> crate::core::HRESULT,
>;
pub type PWLDP_QUERYPOLICYSETTINGENABLED_API = ::core::option::Option<
    unsafe extern "system" fn(
        setting: WLDP_POLICY_SETTING,
        enabled: MutPtr<super::super::Foundation::BOOL>,
    ) -> crate::core::HRESULT,
>;
pub type PWLDP_QUERYWINDOWSLOCKDOWNMODE_API = ::core::option::Option<
    unsafe extern "system" fn(
        lockdown_mode: MutPtr<WLDP_WINDOWS_LOCKDOWN_MODE>,
    ) -> crate::core::HRESULT,
>;
pub type PWLDP_QUERYWINDOWSLOCKDOWNRESTRICTION_API = ::core::option::Option<
    unsafe extern "system" fn(
        lockdown_restriction: MutPtr<WLDP_WINDOWS_LOCKDOWN_RESTRICTION>,
    ) -> crate::core::HRESULT,
>;
pub type PWLDP_RESETPRODUCTIONCONFIGURATION_API =
    ::core::option::Option<unsafe extern "system" fn() -> crate::core::HRESULT>;
pub type PWLDP_RESETWCOSPRODUCTIONCONFIGURATION_API =
    ::core::option::Option<unsafe extern "system" fn() -> crate::core::HRESULT>;
pub type PWLDP_SETDYNAMICCODETRUST_API = ::core::option::Option<
    unsafe extern "system" fn(
        h_file_handle: super::super::Foundation::HANDLE,
    ) -> crate::core::HRESULT,
>;
pub type PWLDP_SETWINDOWSLOCKDOWNRESTRICTION_API = ::core::option::Option<
    unsafe extern "system" fn(
        lockdown_restriction: WLDP_WINDOWS_LOCKDOWN_RESTRICTION,
    ) -> crate::core::HRESULT,
>;
pub const QUERY_ACTCTX_FLAG_ACTCTX_IS_ADDRESS: u32 = 16u32;
pub const QUERY_ACTCTX_FLAG_ACTCTX_IS_HMODULE: u32 = 8u32;
pub const QUERY_ACTCTX_FLAG_NO_ADDREF: u32 = 2147483648u32;
pub const QUERY_ACTCTX_FLAG_USE_ACTIVE_ACTCTX: u32 = 4u32;
pub const RECOVERY_DEFAULT_PING_INTERVAL: u32 = 5000u32;
pub type REGINSTALLA = ::core::option::Option<
    unsafe extern "system" fn(
        hm: super::super::Foundation::HINSTANCE,
        psz_section: crate::core::PCSTR,
        pst_table: MutPtr<STRTABLEA>,
    ) -> crate::core::HRESULT,
>;
pub const REG_RESTORE_LOG_KEY: &'static str = "RegRestoreLogFile";
pub const REG_SAVE_LOG_KEY: &'static str = "RegSaveLogFile";
pub const REMOTE_PROTOCOL_INFO_FLAG_LOOPBACK: u32 = 1u32;
pub const REMOTE_PROTOCOL_INFO_FLAG_OFFLINE: u32 = 2u32;
pub const REMOTE_PROTOCOL_INFO_FLAG_PERSISTENT_HANDLE: u32 = 4u32;
pub const RESETDEV: u32 = 7u32;
pub const RESTART_MAX_CMD_LINE: u32 = 1024u32;
pub const RPI_FLAG_SMB2_SHARECAP_CLUSTER: u32 = 64u32;
pub const RPI_FLAG_SMB2_SHARECAP_CONTINUOUS_AVAILABILITY: u32 = 16u32;
pub const RPI_FLAG_SMB2_SHARECAP_DFS: u32 = 8u32;
pub const RPI_FLAG_SMB2_SHARECAP_SCALEOUT: u32 = 32u32;
pub const RPI_FLAG_SMB2_SHARECAP_TIMEWARP: u32 = 2u32;
pub const RPI_SMB2_FLAG_SERVERCAP_DFS: u32 = 1u32;
pub const RPI_SMB2_FLAG_SERVERCAP_DIRECTORY_LEASING: u32 = 32u32;
pub const RPI_SMB2_FLAG_SERVERCAP_LARGEMTU: u32 = 4u32;
pub const RPI_SMB2_FLAG_SERVERCAP_LEASING: u32 = 2u32;
pub const RPI_SMB2_FLAG_SERVERCAP_MULTICHANNEL: u32 = 8u32;
pub const RPI_SMB2_FLAG_SERVERCAP_PERSISTENT_HANDLES: u32 = 16u32;
pub const RSC_FLAG_DELAYREGISTEROCX: u32 = 512u32;
pub const RSC_FLAG_INF: u32 = 1u32;
pub const RSC_FLAG_NGCONV: u32 = 8u32;
pub const RSC_FLAG_QUIET: u32 = 4u32;
pub const RSC_FLAG_SETUPAPI: u32 = 1024u32;
pub const RSC_FLAG_SKIPDISKSPACECHECK: u32 = 2u32;
pub const RSC_FLAG_UPDHLPDLLS: u32 = 16u32;
pub const RTS_CONTROL_DISABLE: u32 = 0u32;
pub const RTS_CONTROL_ENABLE: u32 = 1u32;
pub const RTS_CONTROL_HANDSHAKE: u32 = 2u32;
pub const RTS_CONTROL_TOGGLE: u32 = 3u32;
pub const RUNCMDS_DELAYPOSTCMD: u32 = 4u32;
pub const RUNCMDS_NOWAIT: u32 = 2u32;
pub const RUNCMDS_QUIET: u32 = 1u32;
pub const SCS_32BIT_BINARY: u32 = 0u32;
pub const SCS_64BIT_BINARY: u32 = 6u32;
pub const SCS_DOS_BINARY: u32 = 1u32;
pub const SCS_OS216_BINARY: u32 = 5u32;
pub const SCS_PIF_BINARY: u32 = 3u32;
pub const SCS_POSIX_BINARY: u32 = 4u32;
pub const SCS_THIS_PLATFORM_BINARY: u32 = 6u32;
pub const SCS_WOW_BINARY: u32 = 2u32;
pub const SHUTDOWN_NORETRY: u32 = 1u32;
pub const STARTF_HOLOGRAPHIC: u32 = 262144u32;
pub const STORAGE_INFO_FLAGS_ALIGNED_DEVICE: u32 = 1u32;
pub const STORAGE_INFO_FLAGS_PARTITION_ALIGNED_ON_DEVICE: u32 = 2u32;
pub const STORAGE_INFO_OFFSET_UNKNOWN: u32 = 4294967295u32;
pub const STREAM_CONTAINS_GHOSTED_FILE_EXTENTS: u32 = 16u32;
pub const STREAM_CONTAINS_PROPERTIES: u32 = 4u32;
pub const STREAM_CONTAINS_SECURITY: u32 = 2u32;
pub const STREAM_MODIFIED_WHEN_READ: u32 = 1u32;
pub const STREAM_NORMAL_ATTRIBUTE: u32 = 0u32;
pub const STREAM_SPARSE_ATTRIBUTE: u32 = 8u32;
pub struct STRENTRYA {
    pub pszName: crate::core::PSTR,
    pub pszValue: crate::core::PSTR,
}
impl ::core::marker::Copy for STRENTRYA {}
impl ::core::clone::Clone for STRENTRYA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STRENTRYA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STRENTRYA")
            .field("pszName", &self.pszName)
            .field("pszValue", &self.pszValue)
            .finish()
    }
}
impl ::core::cmp::PartialEq for STRENTRYA {
    fn eq(&self, other: &Self) -> bool {
        self.pszName == other.pszName && self.pszValue == other.pszValue
    }
}
impl ::core::cmp::Eq for STRENTRYA {}
impl FromIntoMemory for STRENTRYA {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct STRENTRYW {
    pub pszName: crate::core::PWSTR,
    pub pszValue: crate::core::PWSTR,
}
impl ::core::marker::Copy for STRENTRYW {}
impl ::core::clone::Clone for STRENTRYW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STRENTRYW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STRENTRYW")
            .field("pszName", &self.pszName)
            .field("pszValue", &self.pszValue)
            .finish()
    }
}
impl ::core::cmp::PartialEq for STRENTRYW {
    fn eq(&self, other: &Self) -> bool {
        self.pszName == other.pszName && self.pszValue == other.pszValue
    }
}
impl ::core::cmp::Eq for STRENTRYW {}
impl FromIntoMemory for STRENTRYW {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct STRINGEXSTRUCT {
    pub dwSize: u32,
    pub uDeterminePos: u32,
    pub uDetermineDelimPos: u32,
    pub uYomiPos: u32,
    pub uYomiDelimPos: u32,
}
impl ::core::marker::Copy for STRINGEXSTRUCT {}
impl ::core::clone::Clone for STRINGEXSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STRINGEXSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STRINGEXSTRUCT")
            .field("dwSize", &self.dwSize)
            .field("uDeterminePos", &self.uDeterminePos)
            .field("uDetermineDelimPos", &self.uDetermineDelimPos)
            .field("uYomiPos", &self.uYomiPos)
            .field("uYomiDelimPos", &self.uYomiDelimPos)
            .finish()
    }
}
impl ::core::cmp::PartialEq for STRINGEXSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.uDeterminePos == other.uDeterminePos
            && self.uDetermineDelimPos == other.uDetermineDelimPos
            && self.uYomiPos == other.uYomiPos
            && self.uYomiDelimPos == other.uYomiDelimPos
    }
}
impl ::core::cmp::Eq for STRINGEXSTRUCT {}
impl FromIntoMemory for STRINGEXSTRUCT {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct STRTABLEA {
    pub cEntries: u32,
    pub pse: MutPtr<STRENTRYA>,
}
impl ::core::marker::Copy for STRTABLEA {}
impl ::core::clone::Clone for STRTABLEA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STRTABLEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STRTABLEA")
            .field("cEntries", &self.cEntries)
            .field("pse", &self.pse)
            .finish()
    }
}
impl ::core::cmp::PartialEq for STRTABLEA {
    fn eq(&self, other: &Self) -> bool {
        self.cEntries == other.cEntries && self.pse == other.pse
    }
}
impl ::core::cmp::Eq for STRTABLEA {}
impl FromIntoMemory for STRTABLEA {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct STRTABLEW {
    pub cEntries: u32,
    pub pse: MutPtr<STRENTRYW>,
}
impl ::core::marker::Copy for STRTABLEW {}
impl ::core::clone::Clone for STRTABLEW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STRTABLEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STRTABLEW")
            .field("cEntries", &self.cEntries)
            .field("pse", &self.pse)
            .finish()
    }
}
impl ::core::cmp::PartialEq for STRTABLEW {
    fn eq(&self, other: &Self) -> bool {
        self.cEntries == other.cEntries && self.pse == other.pse
    }
}
impl ::core::cmp::Eq for STRTABLEW {}
impl FromIntoMemory for STRTABLEW {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct SYSTEM_BASIC_INFORMATION {
    pub Reserved1: [u8; 24],
    pub Reserved2: [MutPtr<::core::ffi::c_void>; 4],
    pub NumberOfProcessors: i8,
}
impl ::core::marker::Copy for SYSTEM_BASIC_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_BASIC_INFORMATION")
            .field("Reserved1", &self.Reserved1)
            .field("Reserved2", &self.Reserved2)
            .field("NumberOfProcessors", &self.NumberOfProcessors)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
            && self.Reserved2 == other.Reserved2
            && self.NumberOfProcessors == other.NumberOfProcessors
    }
}
impl ::core::cmp::Eq for SYSTEM_BASIC_INFORMATION {}
impl FromIntoMemory for SYSTEM_BASIC_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct SYSTEM_CODEINTEGRITY_INFORMATION {
    pub Length: u32,
    pub CodeIntegrityOptions: u32,
}
impl ::core::marker::Copy for SYSTEM_CODEINTEGRITY_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_CODEINTEGRITY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_CODEINTEGRITY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_CODEINTEGRITY_INFORMATION")
            .field("Length", &self.Length)
            .field("CodeIntegrityOptions", &self.CodeIntegrityOptions)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_CODEINTEGRITY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.CodeIntegrityOptions == other.CodeIntegrityOptions
    }
}
impl ::core::cmp::Eq for SYSTEM_CODEINTEGRITY_INFORMATION {}
impl FromIntoMemory for SYSTEM_CODEINTEGRITY_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct SYSTEM_EXCEPTION_INFORMATION {
    pub Reserved1: [u8; 16],
}
impl ::core::marker::Copy for SYSTEM_EXCEPTION_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_EXCEPTION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_EXCEPTION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_EXCEPTION_INFORMATION")
            .field("Reserved1", &self.Reserved1)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_EXCEPTION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
    }
}
impl ::core::cmp::Eq for SYSTEM_EXCEPTION_INFORMATION {}
impl FromIntoMemory for SYSTEM_EXCEPTION_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SYSTEM_INFORMATION_CLASS(pub i32);
pub const SystemBasicInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(0i32);
pub const SystemPerformanceInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(2i32);
pub const SystemTimeOfDayInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(3i32);
pub const SystemProcessInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(5i32);
pub const SystemProcessorPerformanceInformation: SYSTEM_INFORMATION_CLASS =
    SYSTEM_INFORMATION_CLASS(8i32);
pub const SystemInterruptInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(23i32);
pub const SystemExceptionInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(33i32);
pub const SystemRegistryQuotaInformation: SYSTEM_INFORMATION_CLASS =
    SYSTEM_INFORMATION_CLASS(37i32);
pub const SystemLookasideInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(45i32);
pub const SystemCodeIntegrityInformation: SYSTEM_INFORMATION_CLASS =
    SYSTEM_INFORMATION_CLASS(103i32);
pub const SystemPolicyInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(134i32);
impl ::core::marker::Copy for SYSTEM_INFORMATION_CLASS {}
impl ::core::clone::Clone for SYSTEM_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYSTEM_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYSTEM_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSTEM_INFORMATION_CLASS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for SYSTEM_INFORMATION_CLASS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<i32>()
    }
}
pub struct SYSTEM_INTERRUPT_INFORMATION {
    pub Reserved1: [u8; 24],
}
impl ::core::marker::Copy for SYSTEM_INTERRUPT_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_INTERRUPT_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_INTERRUPT_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_INTERRUPT_INFORMATION")
            .field("Reserved1", &self.Reserved1)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_INTERRUPT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
    }
}
impl ::core::cmp::Eq for SYSTEM_INTERRUPT_INFORMATION {}
impl FromIntoMemory for SYSTEM_INTERRUPT_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct SYSTEM_LOOKASIDE_INFORMATION {
    pub Reserved1: [u8; 32],
}
impl ::core::marker::Copy for SYSTEM_LOOKASIDE_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_LOOKASIDE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_LOOKASIDE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_LOOKASIDE_INFORMATION")
            .field("Reserved1", &self.Reserved1)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_LOOKASIDE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
    }
}
impl ::core::cmp::Eq for SYSTEM_LOOKASIDE_INFORMATION {}
impl FromIntoMemory for SYSTEM_LOOKASIDE_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct SYSTEM_PERFORMANCE_INFORMATION {
    pub Reserved1: [u8; 312],
}
impl ::core::marker::Copy for SYSTEM_PERFORMANCE_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_PERFORMANCE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_PERFORMANCE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_PERFORMANCE_INFORMATION")
            .field("Reserved1", &self.Reserved1)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_PERFORMANCE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
    }
}
impl ::core::cmp::Eq for SYSTEM_PERFORMANCE_INFORMATION {}
impl FromIntoMemory for SYSTEM_PERFORMANCE_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct SYSTEM_POLICY_INFORMATION {
    pub Reserved1: [MutPtr<::core::ffi::c_void>; 2],
    pub Reserved2: [u32; 3],
}
impl ::core::marker::Copy for SYSTEM_POLICY_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_POLICY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_POLICY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_POLICY_INFORMATION")
            .field("Reserved1", &self.Reserved1)
            .field("Reserved2", &self.Reserved2)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_POLICY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for SYSTEM_POLICY_INFORMATION {}
impl FromIntoMemory for SYSTEM_POLICY_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    pub IdleTime: i64,
    pub KernelTime: i64,
    pub UserTime: i64,
    pub Reserved1: [i64; 2],
    pub Reserved2: u32,
}
impl ::core::marker::Copy for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION")
            .field("IdleTime", &self.IdleTime)
            .field("KernelTime", &self.KernelTime)
            .field("UserTime", &self.UserTime)
            .field("Reserved1", &self.Reserved1)
            .field("Reserved2", &self.Reserved2)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.IdleTime == other.IdleTime
            && self.KernelTime == other.KernelTime
            && self.UserTime == other.UserTime
            && self.Reserved1 == other.Reserved1
            && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {}
impl FromIntoMemory for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct SYSTEM_PROCESS_INFORMATION {
    pub NextEntryOffset: u32,
    pub NumberOfThreads: u32,
    pub Reserved1: [u8; 48],
    pub ImageName: super::super::Foundation::UNICODE_STRING,
    pub BasePriority: i32,
    pub UniqueProcessId: super::super::Foundation::HANDLE,
    pub Reserved2: MutPtr<::core::ffi::c_void>,
    pub HandleCount: u32,
    pub SessionId: u32,
    pub Reserved3: MutPtr<::core::ffi::c_void>,
    pub PeakVirtualSize: PtrRepr,
    pub VirtualSize: PtrRepr,
    pub Reserved4: u32,
    pub PeakWorkingSetSize: PtrRepr,
    pub WorkingSetSize: PtrRepr,
    pub Reserved5: MutPtr<::core::ffi::c_void>,
    pub QuotaPagedPoolUsage: PtrRepr,
    pub Reserved6: MutPtr<::core::ffi::c_void>,
    pub QuotaNonPagedPoolUsage: PtrRepr,
    pub PagefileUsage: PtrRepr,
    pub PeakPagefileUsage: PtrRepr,
    pub PrivatePageCount: PtrRepr,
    pub Reserved7: [i64; 6],
}
impl ::core::marker::Copy for SYSTEM_PROCESS_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_PROCESS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_PROCESS_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_PROCESS_INFORMATION")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("NumberOfThreads", &self.NumberOfThreads)
            .field("Reserved1", &self.Reserved1)
            .field("ImageName", &self.ImageName)
            .field("BasePriority", &self.BasePriority)
            .field("UniqueProcessId", &self.UniqueProcessId)
            .field("Reserved2", &self.Reserved2)
            .field("HandleCount", &self.HandleCount)
            .field("SessionId", &self.SessionId)
            .field("Reserved3", &self.Reserved3)
            .field("PeakVirtualSize", &self.PeakVirtualSize)
            .field("VirtualSize", &self.VirtualSize)
            .field("Reserved4", &self.Reserved4)
            .field("PeakWorkingSetSize", &self.PeakWorkingSetSize)
            .field("WorkingSetSize", &self.WorkingSetSize)
            .field("Reserved5", &self.Reserved5)
            .field("QuotaPagedPoolUsage", &self.QuotaPagedPoolUsage)
            .field("Reserved6", &self.Reserved6)
            .field("QuotaNonPagedPoolUsage", &self.QuotaNonPagedPoolUsage)
            .field("PagefileUsage", &self.PagefileUsage)
            .field("PeakPagefileUsage", &self.PeakPagefileUsage)
            .field("PrivatePageCount", &self.PrivatePageCount)
            .field("Reserved7", &self.Reserved7)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_PROCESS_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset
            && self.NumberOfThreads == other.NumberOfThreads
            && self.Reserved1 == other.Reserved1
            && self.ImageName == other.ImageName
            && self.BasePriority == other.BasePriority
            && self.UniqueProcessId == other.UniqueProcessId
            && self.Reserved2 == other.Reserved2
            && self.HandleCount == other.HandleCount
            && self.SessionId == other.SessionId
            && self.Reserved3 == other.Reserved3
            && self.PeakVirtualSize == other.PeakVirtualSize
            && self.VirtualSize == other.VirtualSize
            && self.Reserved4 == other.Reserved4
            && self.PeakWorkingSetSize == other.PeakWorkingSetSize
            && self.WorkingSetSize == other.WorkingSetSize
            && self.Reserved5 == other.Reserved5
            && self.QuotaPagedPoolUsage == other.QuotaPagedPoolUsage
            && self.Reserved6 == other.Reserved6
            && self.QuotaNonPagedPoolUsage == other.QuotaNonPagedPoolUsage
            && self.PagefileUsage == other.PagefileUsage
            && self.PeakPagefileUsage == other.PeakPagefileUsage
            && self.PrivatePageCount == other.PrivatePageCount
            && self.Reserved7 == other.Reserved7
    }
}
impl ::core::cmp::Eq for SYSTEM_PROCESS_INFORMATION {}
impl FromIntoMemory for SYSTEM_PROCESS_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct SYSTEM_REGISTRY_QUOTA_INFORMATION {
    pub RegistryQuotaAllowed: u32,
    pub RegistryQuotaUsed: u32,
    pub Reserved1: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for SYSTEM_REGISTRY_QUOTA_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_REGISTRY_QUOTA_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_REGISTRY_QUOTA_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_REGISTRY_QUOTA_INFORMATION")
            .field("RegistryQuotaAllowed", &self.RegistryQuotaAllowed)
            .field("RegistryQuotaUsed", &self.RegistryQuotaUsed)
            .field("Reserved1", &self.Reserved1)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_REGISTRY_QUOTA_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.RegistryQuotaAllowed == other.RegistryQuotaAllowed
            && self.RegistryQuotaUsed == other.RegistryQuotaUsed
            && self.Reserved1 == other.Reserved1
    }
}
impl ::core::cmp::Eq for SYSTEM_REGISTRY_QUOTA_INFORMATION {}
impl FromIntoMemory for SYSTEM_REGISTRY_QUOTA_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub const SYSTEM_STATUS_FLAG_POWER_SAVING_ON: u32 = 1u32;
pub struct SYSTEM_THREAD_INFORMATION {
    pub Reserved1: [i64; 3],
    pub Reserved2: u32,
    pub StartAddress: MutPtr<::core::ffi::c_void>,
    pub ClientId: CLIENT_ID,
    pub Priority: i32,
    pub BasePriority: i32,
    pub Reserved3: u32,
    pub ThreadState: u32,
    pub WaitReason: u32,
}
impl ::core::marker::Copy for SYSTEM_THREAD_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_THREAD_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_THREAD_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_THREAD_INFORMATION")
            .field("Reserved1", &self.Reserved1)
            .field("Reserved2", &self.Reserved2)
            .field("StartAddress", &self.StartAddress)
            .field("ClientId", &self.ClientId)
            .field("Priority", &self.Priority)
            .field("BasePriority", &self.BasePriority)
            .field("Reserved3", &self.Reserved3)
            .field("ThreadState", &self.ThreadState)
            .field("WaitReason", &self.WaitReason)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_THREAD_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
            && self.Reserved2 == other.Reserved2
            && self.StartAddress == other.StartAddress
            && self.ClientId == other.ClientId
            && self.Priority == other.Priority
            && self.BasePriority == other.BasePriority
            && self.Reserved3 == other.Reserved3
            && self.ThreadState == other.ThreadState
            && self.WaitReason == other.WaitReason
    }
}
impl ::core::cmp::Eq for SYSTEM_THREAD_INFORMATION {}
impl FromIntoMemory for SYSTEM_THREAD_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct SYSTEM_TIMEOFDAY_INFORMATION {
    pub Reserved1: [u8; 48],
}
impl ::core::marker::Copy for SYSTEM_TIMEOFDAY_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_TIMEOFDAY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_TIMEOFDAY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_TIMEOFDAY_INFORMATION")
            .field("Reserved1", &self.Reserved1)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_TIMEOFDAY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
    }
}
impl ::core::cmp::Eq for SYSTEM_TIMEOFDAY_INFORMATION {}
impl FromIntoMemory for SYSTEM_TIMEOFDAY_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub const S_ALLTHRESHOLD: u32 = 2u32;
pub const S_LEGATO: u32 = 1u32;
pub const S_NORMAL: u32 = 0u32;
pub const S_PERIOD1024: u32 = 1u32;
pub const S_PERIOD2048: u32 = 2u32;
pub const S_PERIOD512: u32 = 0u32;
pub const S_PERIODVOICE: u32 = 3u32;
pub const S_QUEUEEMPTY: u32 = 0u32;
pub const S_SERBDNT: i32 = -5i32;
pub const S_SERDCC: i32 = -7i32;
pub const S_SERDDR: i32 = -14i32;
pub const S_SERDFQ: i32 = -13i32;
pub const S_SERDLN: i32 = -6i32;
pub const S_SERDMD: i32 = -10i32;
pub const S_SERDPT: i32 = -12i32;
pub const S_SERDSH: i32 = -11i32;
pub const S_SERDSR: i32 = -15i32;
pub const S_SERDST: i32 = -16i32;
pub const S_SERDTP: i32 = -8i32;
pub const S_SERDVL: i32 = -9i32;
pub const S_SERDVNA: i32 = -1i32;
pub const S_SERMACT: i32 = -3i32;
pub const S_SEROFM: i32 = -2i32;
pub const S_SERQFUL: i32 = -4i32;
pub const S_STACCATO: u32 = 2u32;
pub const S_THRESHOLD: u32 = 1u32;
pub const S_WHITE1024: u32 = 5u32;
pub const S_WHITE2048: u32 = 6u32;
pub const S_WHITE512: u32 = 4u32;
pub const S_WHITEVOICE: u32 = 7u32;
pub const TC_GP_TRAP: u32 = 2u32;
pub const TC_HARDERR: u32 = 1u32;
pub const TC_NORMAL: u32 = 0u32;
pub const TC_SIGNAL: u32 = 3u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TDIENTITY_ENTITY_TYPE(pub u32);
pub const GENERIC_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(0u32);
pub const AT_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(640u32);
pub const CL_NL_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(769u32);
pub const CO_NL_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(768u32);
pub const CL_TL_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(1025u32);
pub const CO_TL_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(1024u32);
pub const ER_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(896u32);
pub const IF_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(512u32);
impl ::core::marker::Copy for TDIENTITY_ENTITY_TYPE {}
impl ::core::clone::Clone for TDIENTITY_ENTITY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TDIENTITY_ENTITY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TDIENTITY_ENTITY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TDIENTITY_ENTITY_TYPE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for TDIENTITY_ENTITY_TYPE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<u32>()
    }
}
pub struct TDIEntityID {
    pub tei_entity: TDIENTITY_ENTITY_TYPE,
    pub tei_instance: u32,
}
impl ::core::marker::Copy for TDIEntityID {}
impl ::core::clone::Clone for TDIEntityID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TDIEntityID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TDIEntityID")
            .field("tei_entity", &self.tei_entity)
            .field("tei_instance", &self.tei_instance)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TDIEntityID {
    fn eq(&self, other: &Self) -> bool {
        self.tei_entity == other.tei_entity && self.tei_instance == other.tei_instance
    }
}
impl ::core::cmp::Eq for TDIEntityID {}
impl FromIntoMemory for TDIEntityID {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct TDIObjectID {
    pub toi_entity: TDIEntityID,
    pub toi_class: u32,
    pub toi_type: u32,
    pub toi_id: u32,
}
impl ::core::marker::Copy for TDIObjectID {}
impl ::core::clone::Clone for TDIObjectID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TDIObjectID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TDIObjectID")
            .field("toi_entity", &self.toi_entity)
            .field("toi_class", &self.toi_class)
            .field("toi_type", &self.toi_type)
            .field("toi_id", &self.toi_id)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TDIObjectID {
    fn eq(&self, other: &Self) -> bool {
        self.toi_entity == other.toi_entity
            && self.toi_class == other.toi_class
            && self.toi_type == other.toi_type
            && self.toi_id == other.toi_id
    }
}
impl ::core::cmp::Eq for TDIObjectID {}
impl FromIntoMemory for TDIObjectID {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct TDI_TL_IO_CONTROL_ENDPOINT {
    pub Type: TDI_TL_IO_CONTROL_TYPE,
    pub Level: u32,
    pub Anonymous: TDI_TL_IO_CONTROL_ENDPOINT_0,
    pub InputBuffer: MutPtr<::core::ffi::c_void>,
    pub InputBufferLength: u32,
    pub OutputBuffer: MutPtr<::core::ffi::c_void>,
    pub OutputBufferLength: u32,
}
impl ::core::marker::Copy for TDI_TL_IO_CONTROL_ENDPOINT {}
impl ::core::clone::Clone for TDI_TL_IO_CONTROL_ENDPOINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for TDI_TL_IO_CONTROL_ENDPOINT {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type
            && self.Level == other.Level
            && self.Anonymous == other.Anonymous
            && self.InputBuffer == other.InputBuffer
            && self.InputBufferLength == other.InputBufferLength
            && self.OutputBuffer == other.OutputBuffer
            && self.OutputBufferLength == other.OutputBufferLength
    }
}
impl ::core::cmp::Eq for TDI_TL_IO_CONTROL_ENDPOINT {}
impl FromIntoMemory for TDI_TL_IO_CONTROL_ENDPOINT {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct TDI_TL_IO_CONTROL_ENDPOINT_0 {
    pub IoControlCode: u32,
    pub OptionName: u32,
}
impl ::core::marker::Copy for TDI_TL_IO_CONTROL_ENDPOINT_0 {}
impl ::core::clone::Clone for TDI_TL_IO_CONTROL_ENDPOINT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for TDI_TL_IO_CONTROL_ENDPOINT_0 {
    fn eq(&self, other: &Self) -> bool {
        self.IoControlCode == other.IoControlCode && self.OptionName == other.OptionName
    }
}
impl ::core::cmp::Eq for TDI_TL_IO_CONTROL_ENDPOINT_0 {}
impl FromIntoMemory for TDI_TL_IO_CONTROL_ENDPOINT_0 {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TDI_TL_IO_CONTROL_TYPE(pub i32);
pub const EndpointIoControlType: TDI_TL_IO_CONTROL_TYPE = TDI_TL_IO_CONTROL_TYPE(0i32);
pub const SetSockOptIoControlType: TDI_TL_IO_CONTROL_TYPE = TDI_TL_IO_CONTROL_TYPE(1i32);
pub const GetSockOptIoControlType: TDI_TL_IO_CONTROL_TYPE = TDI_TL_IO_CONTROL_TYPE(2i32);
pub const SocketIoControlType: TDI_TL_IO_CONTROL_TYPE = TDI_TL_IO_CONTROL_TYPE(3i32);
impl ::core::marker::Copy for TDI_TL_IO_CONTROL_TYPE {}
impl ::core::clone::Clone for TDI_TL_IO_CONTROL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TDI_TL_IO_CONTROL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TDI_TL_IO_CONTROL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TDI_TL_IO_CONTROL_TYPE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for TDI_TL_IO_CONTROL_TYPE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<i32>()
    }
}
pub struct THREAD_NAME_INFORMATION {
    pub ThreadName: super::super::Foundation::UNICODE_STRING,
}
impl ::core::marker::Copy for THREAD_NAME_INFORMATION {}
impl ::core::clone::Clone for THREAD_NAME_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for THREAD_NAME_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("THREAD_NAME_INFORMATION")
            .field("ThreadName", &self.ThreadName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for THREAD_NAME_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ThreadName == other.ThreadName
    }
}
impl ::core::cmp::Eq for THREAD_NAME_INFORMATION {}
impl FromIntoMemory for THREAD_NAME_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub const THREAD_PRIORITY_ERROR_RETURN: u32 = 2147483647u32;
pub const UMS_VERSION: u32 = 256u32;
pub struct UNDETERMINESTRUCT {
    pub dwSize: u32,
    pub uDefIMESize: u32,
    pub uDefIMEPos: u32,
    pub uUndetTextLen: u32,
    pub uUndetTextPos: u32,
    pub uUndetAttrPos: u32,
    pub uCursorPos: u32,
    pub uDeltaStart: u32,
    pub uDetermineTextLen: u32,
    pub uDetermineTextPos: u32,
    pub uDetermineDelimPos: u32,
    pub uYomiTextLen: u32,
    pub uYomiTextPos: u32,
    pub uYomiDelimPos: u32,
}
impl ::core::marker::Copy for UNDETERMINESTRUCT {}
impl ::core::clone::Clone for UNDETERMINESTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UNDETERMINESTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UNDETERMINESTRUCT")
            .field("dwSize", &self.dwSize)
            .field("uDefIMESize", &self.uDefIMESize)
            .field("uDefIMEPos", &self.uDefIMEPos)
            .field("uUndetTextLen", &self.uUndetTextLen)
            .field("uUndetTextPos", &self.uUndetTextPos)
            .field("uUndetAttrPos", &self.uUndetAttrPos)
            .field("uCursorPos", &self.uCursorPos)
            .field("uDeltaStart", &self.uDeltaStart)
            .field("uDetermineTextLen", &self.uDetermineTextLen)
            .field("uDetermineTextPos", &self.uDetermineTextPos)
            .field("uDetermineDelimPos", &self.uDetermineDelimPos)
            .field("uYomiTextLen", &self.uYomiTextLen)
            .field("uYomiTextPos", &self.uYomiTextPos)
            .field("uYomiDelimPos", &self.uYomiDelimPos)
            .finish()
    }
}
impl ::core::cmp::PartialEq for UNDETERMINESTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.uDefIMESize == other.uDefIMESize
            && self.uDefIMEPos == other.uDefIMEPos
            && self.uUndetTextLen == other.uUndetTextLen
            && self.uUndetTextPos == other.uUndetTextPos
            && self.uUndetAttrPos == other.uUndetAttrPos
            && self.uCursorPos == other.uCursorPos
            && self.uDeltaStart == other.uDeltaStart
            && self.uDetermineTextLen == other.uDetermineTextLen
            && self.uDetermineTextPos == other.uDetermineTextPos
            && self.uDetermineDelimPos == other.uDetermineDelimPos
            && self.uYomiTextLen == other.uYomiTextLen
            && self.uYomiTextPos == other.uYomiTextPos
            && self.uYomiDelimPos == other.uYomiDelimPos
    }
}
impl ::core::cmp::Eq for UNDETERMINESTRUCT {}
impl FromIntoMemory for UNDETERMINESTRUCT {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VALUENAME(pub i32);
pub const VALUENAME_UNKNOWN: VALUENAME = VALUENAME(0i32);
pub const VALUENAME_ENTERPRISE_DEFINED_CLASS_ID: VALUENAME = VALUENAME(1i32);
pub const VALUENAME_BUILT_IN_LIST: VALUENAME = VALUENAME(2i32);
impl ::core::marker::Copy for VALUENAME {}
impl ::core::clone::Clone for VALUENAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VALUENAME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VALUENAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VALUENAME").field(&self.0).finish()
    }
}
impl FromIntoMemory for VALUENAME {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<i32>()
    }
}
pub const VOLUME_NAME_DOS: u32 = 0u32;
pub const VOLUME_NAME_GUID: u32 = 1u32;
pub const VOLUME_NAME_NONE: u32 = 4u32;
pub const VOLUME_NAME_NT: u32 = 2u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WINSTATIONINFOCLASS(pub i32);
pub const WinStationInformation: WINSTATIONINFOCLASS = WINSTATIONINFOCLASS(8i32);
impl ::core::marker::Copy for WINSTATIONINFOCLASS {}
impl ::core::clone::Clone for WINSTATIONINFOCLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINSTATIONINFOCLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINSTATIONINFOCLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINSTATIONINFOCLASS").field(&self.0).finish()
    }
}
impl FromIntoMemory for WINSTATIONINFOCLASS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<i32>()
    }
}
pub struct WINSTATIONINFORMATIONW {
    pub Reserved2: [u8; 70],
    pub LogonId: u32,
    pub Reserved3: [u8; 1140],
}
impl ::core::marker::Copy for WINSTATIONINFORMATIONW {}
impl ::core::clone::Clone for WINSTATIONINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINSTATIONINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINSTATIONINFORMATIONW")
            .field("Reserved2", &self.Reserved2)
            .field("LogonId", &self.LogonId)
            .field("Reserved3", &self.Reserved3)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WINSTATIONINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved2 == other.Reserved2
            && self.LogonId == other.LogonId
            && self.Reserved3 == other.Reserved3
    }
}
impl ::core::cmp::Eq for WINSTATIONINFORMATIONW {}
impl FromIntoMemory for WINSTATIONINFORMATIONW {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub type WINWATCHNOTIFYPROC = ::core::option::Option<
    unsafe extern "system" fn(
        hww: HWINWATCH,
        hwnd: super::super::Foundation::HWND,
        code: u32,
        l_param: super::super::Foundation::LPARAM,
    ),
>;
pub const WINWATCHNOTIFY_CHANGED: u32 = 4u32;
pub const WINWATCHNOTIFY_CHANGING: u32 = 3u32;
pub const WINWATCHNOTIFY_DESTROY: u32 = 2u32;
pub const WINWATCHNOTIFY_START: u32 = 0u32;
pub const WINWATCHNOTIFY_STOP: u32 = 1u32;
pub struct WLDP_DEVICE_SECURITY_INFORMATION {
    pub UnlockIdSize: u32,
    pub UnlockId: MutPtr<u8>,
    pub ManufacturerIDLength: u32,
    pub ManufacturerID: crate::core::PWSTR,
}
impl ::core::marker::Copy for WLDP_DEVICE_SECURITY_INFORMATION {}
impl ::core::clone::Clone for WLDP_DEVICE_SECURITY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLDP_DEVICE_SECURITY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLDP_DEVICE_SECURITY_INFORMATION")
            .field("UnlockIdSize", &self.UnlockIdSize)
            .field("UnlockId", &self.UnlockId)
            .field("ManufacturerIDLength", &self.ManufacturerIDLength)
            .field("ManufacturerID", &self.ManufacturerID)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WLDP_DEVICE_SECURITY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.UnlockIdSize == other.UnlockIdSize
            && self.UnlockId == other.UnlockId
            && self.ManufacturerIDLength == other.ManufacturerIDLength
            && self.ManufacturerID == other.ManufacturerID
    }
}
impl ::core::cmp::Eq for WLDP_DEVICE_SECURITY_INFORMATION {}
impl FromIntoMemory for WLDP_DEVICE_SECURITY_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub const WLDP_DLL: &'static str = "WLDP.DLL";
pub const WLDP_FLAGS_SKIPSIGNATUREVALIDATION: u32 = 256u32;
pub const WLDP_GETLOCKDOWNPOLICY_FN: &'static str = "WldpGetLockdownPolicy";
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WLDP_HOST(pub i32);
pub const WLDP_HOST_RUNDLL32: WLDP_HOST = WLDP_HOST(0i32);
pub const WLDP_HOST_SVCHOST: WLDP_HOST = WLDP_HOST(1i32);
pub const WLDP_HOST_MAX: WLDP_HOST = WLDP_HOST(2i32);
impl ::core::marker::Copy for WLDP_HOST {}
impl ::core::clone::Clone for WLDP_HOST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLDP_HOST {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLDP_HOST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLDP_HOST").field(&self.0).finish()
    }
}
impl FromIntoMemory for WLDP_HOST {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<i32>()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WLDP_HOST_ID(pub i32);
pub const WLDP_HOST_ID_UNKNOWN: WLDP_HOST_ID = WLDP_HOST_ID(0i32);
pub const WLDP_HOST_ID_GLOBAL: WLDP_HOST_ID = WLDP_HOST_ID(1i32);
pub const WLDP_HOST_ID_VBA: WLDP_HOST_ID = WLDP_HOST_ID(2i32);
pub const WLDP_HOST_ID_WSH: WLDP_HOST_ID = WLDP_HOST_ID(3i32);
pub const WLDP_HOST_ID_POWERSHELL: WLDP_HOST_ID = WLDP_HOST_ID(4i32);
pub const WLDP_HOST_ID_IE: WLDP_HOST_ID = WLDP_HOST_ID(5i32);
pub const WLDP_HOST_ID_MSI: WLDP_HOST_ID = WLDP_HOST_ID(6i32);
pub const WLDP_HOST_ID_ALL: WLDP_HOST_ID = WLDP_HOST_ID(7i32);
pub const WLDP_HOST_ID_MAX: WLDP_HOST_ID = WLDP_HOST_ID(8i32);
impl ::core::marker::Copy for WLDP_HOST_ID {}
impl ::core::clone::Clone for WLDP_HOST_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLDP_HOST_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLDP_HOST_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLDP_HOST_ID").field(&self.0).finish()
    }
}
impl FromIntoMemory for WLDP_HOST_ID {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<i32>()
    }
}
pub struct WLDP_HOST_INFORMATION {
    pub dwRevision: u32,
    pub dwHostId: WLDP_HOST_ID,
    pub szSource: crate::core::PCWSTR,
    pub hSource: super::super::Foundation::HANDLE,
}
impl ::core::marker::Copy for WLDP_HOST_INFORMATION {}
impl ::core::clone::Clone for WLDP_HOST_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLDP_HOST_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLDP_HOST_INFORMATION")
            .field("dwRevision", &self.dwRevision)
            .field("dwHostId", &self.dwHostId)
            .field("szSource", &self.szSource)
            .field("hSource", &self.hSource)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WLDP_HOST_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwRevision == other.dwRevision
            && self.dwHostId == other.dwHostId
            && self.szSource == other.szSource
            && self.hSource == other.hSource
    }
}
impl ::core::cmp::Eq for WLDP_HOST_INFORMATION {}
impl FromIntoMemory for WLDP_HOST_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub const WLDP_HOST_INFORMATION_REVISION: u32 = 1u32;
pub const WLDP_ISAPPAPPROVEDBYPOLICY_FN: &'static str = "WldpIsAppApprovedByPolicy";
pub const WLDP_ISCLASSINAPPROVEDLIST_FN: &'static str = "WldpIsClassInApprovedList";
pub const WLDP_ISDYNAMICCODEPOLICYENABLED_FN: &'static str = "WldpIsDynamicCodePolicyEnabled";
pub const WLDP_ISPRODUCTIONCONFIGURATION_FN: &'static str = "WldpIsProductionConfiguration";
pub const WLDP_ISWCOSPRODUCTIONCONFIGURATION_FN: &'static str = "WldpIsWcosProductionConfiguration";
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WLDP_KEY(pub i32);
pub const KEY_UNKNOWN: WLDP_KEY = WLDP_KEY(0i32);
pub const KEY_OVERRIDE: WLDP_KEY = WLDP_KEY(1i32);
pub const KEY_ALL_KEYS: WLDP_KEY = WLDP_KEY(2i32);
impl ::core::marker::Copy for WLDP_KEY {}
impl ::core::clone::Clone for WLDP_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLDP_KEY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLDP_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLDP_KEY").field(&self.0).finish()
    }
}
impl FromIntoMemory for WLDP_KEY {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<i32>()
    }
}
pub const WLDP_LOCKDOWN_AUDIT_FLAG: u32 = 8u32;
pub const WLDP_LOCKDOWN_CONFIG_CI_AUDIT_FLAG: u32 = 2u32;
pub const WLDP_LOCKDOWN_CONFIG_CI_FLAG: u32 = 1u32;
pub const WLDP_LOCKDOWN_DEFINED_FLAG: u32 = 2147483648u32;
pub const WLDP_LOCKDOWN_EXCLUSION_FLAG: u32 = 16u32;
pub const WLDP_LOCKDOWN_OFF: u32 = 2147483648u32;
pub const WLDP_LOCKDOWN_UMCIENFORCE_FLAG: u32 = 4u32;
pub const WLDP_LOCKDOWN_UNDEFINED: u32 = 0u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WLDP_POLICY_SETTING(pub i32);
pub const WLDP_POLICY_SETTING_AV_PERF_MODE: WLDP_POLICY_SETTING = WLDP_POLICY_SETTING(1000i32);
impl ::core::marker::Copy for WLDP_POLICY_SETTING {}
impl ::core::clone::Clone for WLDP_POLICY_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLDP_POLICY_SETTING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLDP_POLICY_SETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLDP_POLICY_SETTING").field(&self.0).finish()
    }
}
impl FromIntoMemory for WLDP_POLICY_SETTING {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<i32>()
    }
}
pub const WLDP_QUERYDANAMICCODETRUST_FN: &'static str = "WldpQueryDynamicCodeTrust";
pub const WLDP_QUERYDEVICESECURITYINFORMATION_FN: &'static str =
    "WldpQueryDeviceSecurityInformation";
pub const WLDP_QUERYDYNAMICCODETRUST_FN: &'static str = "WldpQueryDynamicCodeTrust";
pub const WLDP_QUERYPOLICYSETTINGENABLED2_FN: &'static str = "WldpQueryPolicySettingEnabled2";
pub const WLDP_QUERYPOLICYSETTINGENABLED_FN: &'static str = "WldpQueryPolicySettingEnabled";
pub const WLDP_QUERYWINDOWSLOCKDOWNMODE_FN: &'static str = "WldpQueryWindowsLockdownMode";
pub const WLDP_QUERYWINDOWSLOCKDOWNRESTRICTION_FN: &'static str =
    "WldpQueryWindowsLockdownRestriction";
pub const WLDP_RESETPRODUCTIONCONFIGURATION_FN: &'static str = "WldpResetProductionConfiguration";
pub const WLDP_RESETWCOSPRODUCTIONCONFIGURATION_FN: &'static str =
    "WldpResetWcosProductionConfiguration";
pub const WLDP_SETDYNAMICCODETRUST_FN: &'static str = "WldpSetDynamicCodeTrust";
pub const WLDP_SETWINDOWSLOCKDOWNRESTRICTION_FN: &'static str = "WldpSetWindowsLockdownRestriction";
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WLDP_WINDOWS_LOCKDOWN_MODE(pub i32);
pub const WLDP_WINDOWS_LOCKDOWN_MODE_UNLOCKED: WLDP_WINDOWS_LOCKDOWN_MODE =
    WLDP_WINDOWS_LOCKDOWN_MODE(0i32);
pub const WLDP_WINDOWS_LOCKDOWN_MODE_TRIAL: WLDP_WINDOWS_LOCKDOWN_MODE =
    WLDP_WINDOWS_LOCKDOWN_MODE(1i32);
pub const WLDP_WINDOWS_LOCKDOWN_MODE_LOCKED: WLDP_WINDOWS_LOCKDOWN_MODE =
    WLDP_WINDOWS_LOCKDOWN_MODE(2i32);
pub const WLDP_WINDOWS_LOCKDOWN_MODE_MAX: WLDP_WINDOWS_LOCKDOWN_MODE =
    WLDP_WINDOWS_LOCKDOWN_MODE(3i32);
impl ::core::marker::Copy for WLDP_WINDOWS_LOCKDOWN_MODE {}
impl ::core::clone::Clone for WLDP_WINDOWS_LOCKDOWN_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLDP_WINDOWS_LOCKDOWN_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLDP_WINDOWS_LOCKDOWN_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLDP_WINDOWS_LOCKDOWN_MODE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for WLDP_WINDOWS_LOCKDOWN_MODE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<i32>()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WLDP_WINDOWS_LOCKDOWN_RESTRICTION(pub i32);
pub const WLDP_WINDOWS_LOCKDOWN_RESTRICTION_NONE: WLDP_WINDOWS_LOCKDOWN_RESTRICTION =
    WLDP_WINDOWS_LOCKDOWN_RESTRICTION(0i32);
pub const WLDP_WINDOWS_LOCKDOWN_RESTRICTION_NOUNLOCK: WLDP_WINDOWS_LOCKDOWN_RESTRICTION =
    WLDP_WINDOWS_LOCKDOWN_RESTRICTION(1i32);
pub const WLDP_WINDOWS_LOCKDOWN_RESTRICTION_NOUNLOCK_PERMANENT: WLDP_WINDOWS_LOCKDOWN_RESTRICTION =
    WLDP_WINDOWS_LOCKDOWN_RESTRICTION(2i32);
pub const WLDP_WINDOWS_LOCKDOWN_RESTRICTION_MAX: WLDP_WINDOWS_LOCKDOWN_RESTRICTION =
    WLDP_WINDOWS_LOCKDOWN_RESTRICTION(3i32);
impl ::core::marker::Copy for WLDP_WINDOWS_LOCKDOWN_RESTRICTION {}
impl ::core::clone::Clone for WLDP_WINDOWS_LOCKDOWN_RESTRICTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLDP_WINDOWS_LOCKDOWN_RESTRICTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLDP_WINDOWS_LOCKDOWN_RESTRICTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLDP_WINDOWS_LOCKDOWN_RESTRICTION")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for WLDP_WINDOWS_LOCKDOWN_RESTRICTION {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<i32>()
    }
}
pub const WM_CONVERTREQUEST: u32 = 266u32;
pub const WM_CONVERTRESULT: u32 = 267u32;
pub const WM_IMEKEYDOWN: u32 = 656u32;
pub const WM_IMEKEYUP: u32 = 657u32;
pub const WM_IME_REPORT: u32 = 640u32;
pub const WM_INTERIM: u32 = 268u32;
pub const WM_WNT_CONVERTREQUESTEX: u32 = 265u32;
pub struct _D3DHAL_CALLBACKS(pub u8);
pub struct _D3DHAL_GLOBALDRIVERDATA(pub u8);
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct tcp_request_query_information_ex32_xp {
    pub ID: TDIObjectID,
    pub Context: [u32; 4],
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for tcp_request_query_information_ex32_xp {}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for tcp_request_query_information_ex32_xp {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for tcp_request_query_information_ex32_xp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("tcp_request_query_information_ex32_xp")
            .field("ID", &self.ID)
            .field("Context", &self.Context)
            .finish()
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for tcp_request_query_information_ex32_xp {
    fn eq(&self, other: &Self) -> bool {
        self.ID == other.ID && self.Context == other.Context
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for tcp_request_query_information_ex32_xp {}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for tcp_request_query_information_ex32_xp {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct tcp_request_query_information_ex_w2k {
    pub ID: TDIObjectID,
    pub Context: [u8; 16],
}
impl ::core::marker::Copy for tcp_request_query_information_ex_w2k {}
impl ::core::clone::Clone for tcp_request_query_information_ex_w2k {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for tcp_request_query_information_ex_w2k {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("tcp_request_query_information_ex_w2k")
            .field("ID", &self.ID)
            .field("Context", &self.Context)
            .finish()
    }
}
impl ::core::cmp::PartialEq for tcp_request_query_information_ex_w2k {
    fn eq(&self, other: &Self) -> bool {
        self.ID == other.ID && self.Context == other.Context
    }
}
impl ::core::cmp::Eq for tcp_request_query_information_ex_w2k {}
impl FromIntoMemory for tcp_request_query_information_ex_w2k {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct tcp_request_query_information_ex_xp {
    pub ID: TDIObjectID,
    pub Context: [PtrRepr; 2],
}
impl ::core::marker::Copy for tcp_request_query_information_ex_xp {}
impl ::core::clone::Clone for tcp_request_query_information_ex_xp {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for tcp_request_query_information_ex_xp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("tcp_request_query_information_ex_xp")
            .field("ID", &self.ID)
            .field("Context", &self.Context)
            .finish()
    }
}
impl ::core::cmp::PartialEq for tcp_request_query_information_ex_xp {
    fn eq(&self, other: &Self) -> bool {
        self.ID == other.ID && self.Context == other.Context
    }
}
impl ::core::cmp::Eq for tcp_request_query_information_ex_xp {}
impl FromIntoMemory for tcp_request_query_information_ex_xp {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct tcp_request_set_information_ex {
    pub ID: TDIObjectID,
    pub BufferSize: u32,
    pub Buffer: [u8; 1],
}
impl ::core::marker::Copy for tcp_request_set_information_ex {}
impl ::core::clone::Clone for tcp_request_set_information_ex {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for tcp_request_set_information_ex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("tcp_request_set_information_ex")
            .field("ID", &self.ID)
            .field("BufferSize", &self.BufferSize)
            .field("Buffer", &self.Buffer)
            .finish()
    }
}
impl ::core::cmp::PartialEq for tcp_request_set_information_ex {
    fn eq(&self, other: &Self) -> bool {
        self.ID == other.ID && self.BufferSize == other.BufferSize && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for tcp_request_set_information_ex {}
impl FromIntoMemory for tcp_request_set_information_ex {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub trait Api {
    fn AddDelBackupEntryA(
        &self,
        lpcsz_file_list: crate::core::PCSTR,
        lpcsz_backup_dir: crate::core::PCSTR,
        lpcsz_base_name: crate::core::PCSTR,
        dw_flags: u32,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn AddDelBackupEntryW(
        &self,
        lpcsz_file_list: crate::core::PCWSTR,
        lpcsz_backup_dir: crate::core::PCWSTR,
        lpcsz_base_name: crate::core::PCWSTR,
        dw_flags: u32,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn AdvInstallFileA(
        &self,
        hwnd: super::super::Foundation::HWND,
        lpsz_source_dir: crate::core::PCSTR,
        lpsz_source_file: crate::core::PCSTR,
        lpsz_dest_dir: crate::core::PCSTR,
        lpsz_dest_file: crate::core::PCSTR,
        dw_flags: u32,
        dw_reserved: u32,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn AdvInstallFileW(
        &self,
        hwnd: super::super::Foundation::HWND,
        lpsz_source_dir: crate::core::PCWSTR,
        lpsz_source_file: crate::core::PCWSTR,
        lpsz_dest_dir: crate::core::PCWSTR,
        lpsz_dest_file: crate::core::PCWSTR,
        dw_flags: u32,
        dw_reserved: u32,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn ApphelpCheckShellObject(
        &self,
        object_clsid: ConstPtr<crate::core::GUID>,
        b_shim_if_necessary: super::super::Foundation::BOOL,
        pull_flags: MutPtr<u64>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn CancelDeviceWakeupRequest(
        &self,
        h_device: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn CancelTimerQueueTimer(
        &self,
        timer_queue: super::super::Foundation::HANDLE,
        timer: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn CloseINFEngine(&self, h_inf: MutPtr<::core::ffi::c_void>) -> crate::core::HRESULT {
        todo!()
    }
    fn ConvertAuxiliaryCounterToPerformanceCounter(
        &self,
        ull_auxiliary_counter_value: u64,
        lp_performance_counter_value: MutPtr<u64>,
        lp_conversion_error: MutPtr<u64>,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn ConvertPerformanceCounterToAuxiliaryCounter(
        &self,
        ull_performance_counter_value: u64,
        lp_auxiliary_counter_value: MutPtr<u64>,
        lp_conversion_error: MutPtr<u64>,
    ) -> crate::core::HRESULT {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateWaitableTimerA(
        &self,
        lp_timer_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        b_manual_reset: super::super::Foundation::BOOL,
        lp_timer_name: crate::core::PCSTR,
    ) -> super::super::Foundation::HANDLE {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateWaitableTimerExA(
        &self,
        lp_timer_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        lp_timer_name: crate::core::PCSTR,
        dw_flags: u32,
        dw_desired_access: u32,
    ) -> super::super::Foundation::HANDLE {
        todo!()
    }
    fn DelNodeA(
        &self,
        psz_file_or_dir_name: crate::core::PCSTR,
        dw_flags: u32,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn DelNodeRunDLL32W(
        &self,
        hwnd: super::super::Foundation::HWND,
        h_instance: super::super::Foundation::HINSTANCE,
        psz_parms: crate::core::PWSTR,
        n_show: i32,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn DelNodeW(
        &self,
        psz_file_or_dir_name: crate::core::PCWSTR,
        dw_flags: u32,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn DnsHostnameToComputerNameA(
        &self,
        hostname: crate::core::PCSTR,
        computer_name: crate::core::PSTR,
        n_size: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn DnsHostnameToComputerNameW(
        &self,
        hostname: crate::core::PCWSTR,
        computer_name: crate::core::PWSTR,
        n_size: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn DosDateTimeToFileTime(
        &self,
        w_fat_date: u16,
        w_fat_time: u16,
        lp_file_time: MutPtr<super::super::Foundation::FILETIME>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn EnableProcessOptionalXStateFeatures(&self, features: u64) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn ExecuteCabA(
        &self,
        hwnd: super::super::Foundation::HWND,
        p_cab: MutPtr<CABINFOA>,
        p_reserved: MutPtr<::core::ffi::c_void>,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn ExecuteCabW(
        &self,
        hwnd: super::super::Foundation::HWND,
        p_cab: MutPtr<CABINFOW>,
        p_reserved: MutPtr<::core::ffi::c_void>,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn ExtractFilesA(
        &self,
        psz_cab_name: crate::core::PCSTR,
        psz_expand_dir: crate::core::PCSTR,
        dw_flags: u32,
        psz_file_list: crate::core::PCSTR,
        lp_reserved: MutPtr<::core::ffi::c_void>,
        dw_reserved: u32,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn ExtractFilesW(
        &self,
        psz_cab_name: crate::core::PCWSTR,
        psz_expand_dir: crate::core::PCWSTR,
        dw_flags: u32,
        psz_file_list: crate::core::PCWSTR,
        lp_reserved: MutPtr<::core::ffi::c_void>,
        dw_reserved: u32,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn FileSaveMarkNotExistA(
        &self,
        lp_file_list: crate::core::PCSTR,
        lp_dir: crate::core::PCSTR,
        lp_base_name: crate::core::PCSTR,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn FileSaveMarkNotExistW(
        &self,
        lp_file_list: crate::core::PCWSTR,
        lp_dir: crate::core::PCWSTR,
        lp_base_name: crate::core::PCWSTR,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn FileSaveRestoreOnINFA(
        &self,
        h_wnd: super::super::Foundation::HWND,
        psz_title: crate::core::PCSTR,
        psz_inf: crate::core::PCSTR,
        psz_section: crate::core::PCSTR,
        psz_backup_dir: crate::core::PCSTR,
        psz_base_backup_file: crate::core::PCSTR,
        dw_flags: u32,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn FileSaveRestoreOnINFW(
        &self,
        h_wnd: super::super::Foundation::HWND,
        psz_title: crate::core::PCWSTR,
        psz_inf: crate::core::PCWSTR,
        psz_section: crate::core::PCWSTR,
        psz_backup_dir: crate::core::PCWSTR,
        psz_base_backup_file: crate::core::PCWSTR,
        dw_flags: u32,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn FileSaveRestoreW(
        &self,
        h_dlg: super::super::Foundation::HWND,
        lp_file_list: crate::core::PCWSTR,
        lp_dir: crate::core::PCWSTR,
        lp_base_name: crate::core::PCWSTR,
        dw_flags: u32,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn FileTimeToDosDateTime(
        &self,
        lp_file_time: ConstPtr<super::super::Foundation::FILETIME>,
        lp_fat_date: MutPtr<u16>,
        lp_fat_time: MutPtr<u16>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GdiEntry13(&self) -> u32 {
        todo!()
    }
    fn GetComputerNameA(
        &self,
        lp_buffer: crate::core::PSTR,
        n_size: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetComputerNameW(
        &self,
        lp_buffer: crate::core::PWSTR,
        n_size: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetCurrentHwProfileA(
        &self,
        lp_hw_profile_info: MutPtr<HW_PROFILE_INFOA>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetCurrentHwProfileW(
        &self,
        lp_hw_profile_info: MutPtr<HW_PROFILE_INFOW>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetFeatureEnabledState(
        &self,
        feature_id: u32,
        change_time: FEATURE_CHANGE_TIME,
    ) -> FEATURE_ENABLED_STATE {
        todo!()
    }
    fn GetFeatureVariant(
        &self,
        feature_id: u32,
        change_time: FEATURE_CHANGE_TIME,
        payload_id: MutPtr<u32>,
        has_notification: MutPtr<super::super::Foundation::BOOL>,
    ) -> u32 {
        todo!()
    }
    fn GetFirmwareEnvironmentVariableA(
        &self,
        lp_name: crate::core::PCSTR,
        lp_guid: crate::core::PCSTR,
        p_buffer: MutPtr<::core::ffi::c_void>,
        n_size: u32,
    ) -> u32 {
        todo!()
    }
    fn GetFirmwareEnvironmentVariableExA(
        &self,
        lp_name: crate::core::PCSTR,
        lp_guid: crate::core::PCSTR,
        p_buffer: MutPtr<::core::ffi::c_void>,
        n_size: u32,
        pdw_attribubutes: MutPtr<u32>,
    ) -> u32 {
        todo!()
    }
    fn GetFirmwareEnvironmentVariableExW(
        &self,
        lp_name: crate::core::PCWSTR,
        lp_guid: crate::core::PCWSTR,
        p_buffer: MutPtr<::core::ffi::c_void>,
        n_size: u32,
        pdw_attribubutes: MutPtr<u32>,
    ) -> u32 {
        todo!()
    }
    fn GetFirmwareEnvironmentVariableW(
        &self,
        lp_name: crate::core::PCWSTR,
        lp_guid: crate::core::PCWSTR,
        p_buffer: MutPtr<::core::ffi::c_void>,
        n_size: u32,
    ) -> u32 {
        todo!()
    }
    fn GetPrivateProfileIntA(
        &self,
        lp_app_name: crate::core::PCSTR,
        lp_key_name: crate::core::PCSTR,
        n_default: i32,
        lp_file_name: crate::core::PCSTR,
    ) -> u32 {
        todo!()
    }
    fn GetPrivateProfileIntW(
        &self,
        lp_app_name: crate::core::PCWSTR,
        lp_key_name: crate::core::PCWSTR,
        n_default: i32,
        lp_file_name: crate::core::PCWSTR,
    ) -> u32 {
        todo!()
    }
    fn GetPrivateProfileSectionA(
        &self,
        lp_app_name: crate::core::PCSTR,
        lp_returned_string: crate::core::PSTR,
        n_size: u32,
        lp_file_name: crate::core::PCSTR,
    ) -> u32 {
        todo!()
    }
    fn GetPrivateProfileSectionNamesA(
        &self,
        lpsz_return_buffer: crate::core::PSTR,
        n_size: u32,
        lp_file_name: crate::core::PCSTR,
    ) -> u32 {
        todo!()
    }
    fn GetPrivateProfileSectionNamesW(
        &self,
        lpsz_return_buffer: crate::core::PWSTR,
        n_size: u32,
        lp_file_name: crate::core::PCWSTR,
    ) -> u32 {
        todo!()
    }
    fn GetPrivateProfileSectionW(
        &self,
        lp_app_name: crate::core::PCWSTR,
        lp_returned_string: crate::core::PWSTR,
        n_size: u32,
        lp_file_name: crate::core::PCWSTR,
    ) -> u32 {
        todo!()
    }
    fn GetPrivateProfileStringA(
        &self,
        lp_app_name: crate::core::PCSTR,
        lp_key_name: crate::core::PCSTR,
        lp_default: crate::core::PCSTR,
        lp_returned_string: crate::core::PSTR,
        n_size: u32,
        lp_file_name: crate::core::PCSTR,
    ) -> u32 {
        todo!()
    }
    fn GetPrivateProfileStringW(
        &self,
        lp_app_name: crate::core::PCWSTR,
        lp_key_name: crate::core::PCWSTR,
        lp_default: crate::core::PCWSTR,
        lp_returned_string: crate::core::PWSTR,
        n_size: u32,
        lp_file_name: crate::core::PCWSTR,
    ) -> u32 {
        todo!()
    }
    fn GetPrivateProfileStructA(
        &self,
        lpsz_section: crate::core::PCSTR,
        lpsz_key: crate::core::PCSTR,
        lp_struct: MutPtr<::core::ffi::c_void>,
        u_size_struct: u32,
        sz_file: crate::core::PCSTR,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetPrivateProfileStructW(
        &self,
        lpsz_section: crate::core::PCWSTR,
        lpsz_key: crate::core::PCWSTR,
        lp_struct: MutPtr<::core::ffi::c_void>,
        u_size_struct: u32,
        sz_file: crate::core::PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetProfileIntA(
        &self,
        lp_app_name: crate::core::PCSTR,
        lp_key_name: crate::core::PCSTR,
        n_default: i32,
    ) -> u32 {
        todo!()
    }
    fn GetProfileIntW(
        &self,
        lp_app_name: crate::core::PCWSTR,
        lp_key_name: crate::core::PCWSTR,
        n_default: i32,
    ) -> u32 {
        todo!()
    }
    fn GetProfileSectionA(
        &self,
        lp_app_name: crate::core::PCSTR,
        lp_returned_string: crate::core::PSTR,
        n_size: u32,
    ) -> u32 {
        todo!()
    }
    fn GetProfileSectionW(
        &self,
        lp_app_name: crate::core::PCWSTR,
        lp_returned_string: crate::core::PWSTR,
        n_size: u32,
    ) -> u32 {
        todo!()
    }
    fn GetProfileStringA(
        &self,
        lp_app_name: crate::core::PCSTR,
        lp_key_name: crate::core::PCSTR,
        lp_default: crate::core::PCSTR,
        lp_returned_string: crate::core::PSTR,
        n_size: u32,
    ) -> u32 {
        todo!()
    }
    fn GetProfileStringW(
        &self,
        lp_app_name: crate::core::PCWSTR,
        lp_key_name: crate::core::PCWSTR,
        lp_default: crate::core::PCWSTR,
        lp_returned_string: crate::core::PWSTR,
        n_size: u32,
    ) -> u32 {
        todo!()
    }
    fn GetSystemRegistryQuota(
        &self,
        pdw_quota_allowed: MutPtr<u32>,
        pdw_quota_used: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetThreadEnabledXStateFeatures(&self) -> u64 {
        todo!()
    }
    fn GetUserNameA(
        &self,
        lp_buffer: crate::core::PSTR,
        pcb_buffer: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetUserNameW(
        &self,
        lp_buffer: crate::core::PWSTR,
        pcb_buffer: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetVersionFromFileA(
        &self,
        lpsz_filename: crate::core::PCSTR,
        pdw_ms_ver: MutPtr<u32>,
        pdw_ls_ver: MutPtr<u32>,
        b_version: super::super::Foundation::BOOL,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn GetVersionFromFileExA(
        &self,
        lpsz_filename: crate::core::PCSTR,
        pdw_ms_ver: MutPtr<u32>,
        pdw_ls_ver: MutPtr<u32>,
        b_version: super::super::Foundation::BOOL,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn GetVersionFromFileExW(
        &self,
        lpsz_filename: crate::core::PCWSTR,
        pdw_ms_ver: MutPtr<u32>,
        pdw_ls_ver: MutPtr<u32>,
        b_version: super::super::Foundation::BOOL,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn GetVersionFromFileW(
        &self,
        lpsz_filename: crate::core::PCWSTR,
        pdw_ms_ver: MutPtr<u32>,
        pdw_ls_ver: MutPtr<u32>,
        b_version: super::super::Foundation::BOOL,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn GlobalCompact(&self, dw_min_free: u32) -> PtrRepr {
        todo!()
    }
    fn GlobalFix(&self, h_mem: PtrDiffRepr) {
        todo!()
    }
    fn GlobalUnWire(&self, h_mem: PtrDiffRepr) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GlobalUnfix(&self, h_mem: PtrDiffRepr) {
        todo!()
    }
    fn GlobalWire(&self, h_mem: PtrDiffRepr) -> MutPtr<::core::ffi::c_void> {
        todo!()
    }
    fn IMPGetIMEA(
        &self,
        param_0: super::super::Foundation::HWND,
        param_1: MutPtr<IMEPROA>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IMPGetIMEW(
        &self,
        param_0: super::super::Foundation::HWND,
        param_1: MutPtr<IMEPROW>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IMPQueryIMEA(&self, param_0: MutPtr<IMEPROA>) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IMPQueryIMEW(&self, param_0: MutPtr<IMEPROW>) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IMPSetIMEA(
        &self,
        param_0: super::super::Foundation::HWND,
        param_1: MutPtr<IMEPROA>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IMPSetIMEW(
        &self,
        param_0: super::super::Foundation::HWND,
        param_1: MutPtr<IMEPROW>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsApiSetImplemented(&self, contract: crate::core::PCSTR) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsBadHugeReadPtr(
        &self,
        lp: ConstPtr<::core::ffi::c_void>,
        ucb: PtrRepr,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsBadHugeWritePtr(
        &self,
        lp: ConstPtr<::core::ffi::c_void>,
        ucb: PtrRepr,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsNTAdmin(
        &self,
        dw_reserved: u32,
        lpdw_reserved: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsNativeVhdBoot(
        &self,
        native_vhd_boot: MutPtr<super::super::Foundation::BOOL>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsTokenUntrusted(
        &self,
        token_handle: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn LaunchINFSectionExW(
        &self,
        hwnd: super::super::Foundation::HWND,
        h_instance: super::super::Foundation::HINSTANCE,
        psz_parms: crate::core::PCWSTR,
        n_show: i32,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn LaunchINFSectionW(
        &self,
        hwnd_owner: super::super::Foundation::HWND,
        h_instance: super::super::Foundation::HINSTANCE,
        psz_params: crate::core::PWSTR,
        n_show: i32,
    ) -> i32 {
        todo!()
    }
    fn LocalCompact(&self, u_min_free: u32) -> PtrRepr {
        todo!()
    }
    fn LocalShrink(&self, h_mem: PtrDiffRepr, cb_new_size: u32) -> PtrRepr {
        todo!()
    }
    fn MulDiv(&self, n_number: i32, n_numerator: i32, n_denominator: i32) -> i32 {
        todo!()
    }
    fn NeedReboot(&self, dw_reboot_check: u32) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn NeedRebootInit(&self) -> u32 {
        todo!()
    }
    fn NtClose(
        &self,
        handle: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::NTSTATUS {
        todo!()
    }
    fn NtDeviceIoControlFile(
        &self,
        file_handle: super::super::Foundation::HANDLE,
        event: super::super::Foundation::HANDLE,
        apc_routine: PIO_APC_ROUTINE,
        apc_context: MutPtr<::core::ffi::c_void>,
        io_status_block: MutPtr<IO_STATUS_BLOCK>,
        io_control_code: u32,
        input_buffer: MutPtr<::core::ffi::c_void>,
        input_buffer_length: u32,
        output_buffer: MutPtr<::core::ffi::c_void>,
        output_buffer_length: u32,
    ) -> super::super::Foundation::NTSTATUS {
        todo!()
    }
    fn NtNotifyChangeMultipleKeys(
        &self,
        master_key_handle: super::super::Foundation::HANDLE,
        count: u32,
        subordinate_objects: ConstPtr<OBJECT_ATTRIBUTES>,
        event: super::super::Foundation::HANDLE,
        apc_routine: PIO_APC_ROUTINE,
        apc_context: ConstPtr<::core::ffi::c_void>,
        io_status_block: MutPtr<IO_STATUS_BLOCK>,
        completion_filter: u32,
        watch_tree: super::super::Foundation::BOOLEAN,
        buffer: MutPtr<::core::ffi::c_void>,
        buffer_size: u32,
        asynchronous: super::super::Foundation::BOOLEAN,
    ) -> super::super::Foundation::NTSTATUS {
        todo!()
    }
    fn NtOpenFile(
        &self,
        file_handle: MutPtr<super::super::Foundation::HANDLE>,
        desired_access: u32,
        object_attributes: MutPtr<OBJECT_ATTRIBUTES>,
        io_status_block: MutPtr<IO_STATUS_BLOCK>,
        share_access: u32,
        open_options: u32,
    ) -> super::super::Foundation::NTSTATUS {
        todo!()
    }
    fn NtQueryMultipleValueKey(
        &self,
        key_handle: super::super::Foundation::HANDLE,
        value_entries: MutPtr<KEY_VALUE_ENTRY>,
        entry_count: u32,
        value_buffer: MutPtr<::core::ffi::c_void>,
        buffer_length: MutPtr<u32>,
        required_buffer_length: MutPtr<u32>,
    ) -> super::super::Foundation::NTSTATUS {
        todo!()
    }
    fn NtQueryObject(
        &self,
        handle: super::super::Foundation::HANDLE,
        object_information_class: OBJECT_INFORMATION_CLASS,
        object_information: MutPtr<::core::ffi::c_void>,
        object_information_length: u32,
        return_length: MutPtr<u32>,
    ) -> super::super::Foundation::NTSTATUS {
        todo!()
    }
    fn NtQuerySystemInformation(
        &self,
        system_information_class: SYSTEM_INFORMATION_CLASS,
        system_information: MutPtr<::core::ffi::c_void>,
        system_information_length: u32,
        return_length: MutPtr<u32>,
    ) -> super::super::Foundation::NTSTATUS {
        todo!()
    }
    fn NtQuerySystemTime(&self, system_time: MutPtr<i64>) -> super::super::Foundation::NTSTATUS {
        todo!()
    }
    fn NtQueryTimerResolution(
        &self,
        maximum_time: MutPtr<u32>,
        minimum_time: MutPtr<u32>,
        current_time: MutPtr<u32>,
    ) -> super::super::Foundation::NTSTATUS {
        todo!()
    }
    fn NtRenameKey(
        &self,
        key_handle: super::super::Foundation::HANDLE,
        new_name: ConstPtr<super::super::Foundation::UNICODE_STRING>,
    ) -> super::super::Foundation::NTSTATUS {
        todo!()
    }
    fn NtSetInformationKey(
        &self,
        key_handle: super::super::Foundation::HANDLE,
        key_set_information_class: KEY_SET_INFORMATION_CLASS,
        key_set_information: ConstPtr<::core::ffi::c_void>,
        key_set_information_length: u32,
    ) -> super::super::Foundation::NTSTATUS {
        todo!()
    }
    fn NtWaitForSingleObject(
        &self,
        handle: super::super::Foundation::HANDLE,
        alertable: super::super::Foundation::BOOLEAN,
        timeout: MutPtr<i64>,
    ) -> super::super::Foundation::NTSTATUS {
        todo!()
    }
    fn OpenINFEngineA(
        &self,
        psz_inf_filename: crate::core::PCSTR,
        psz_install_section: crate::core::PCSTR,
        dw_flags: u32,
        ph_inf: MutPtr<ConstPtr<::core::ffi::c_void>>,
        pv_reserved: MutPtr<::core::ffi::c_void>,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn OpenINFEngineW(
        &self,
        psz_inf_filename: crate::core::PCWSTR,
        psz_install_section: crate::core::PCWSTR,
        dw_flags: u32,
        ph_inf: MutPtr<ConstPtr<::core::ffi::c_void>>,
        pv_reserved: MutPtr<::core::ffi::c_void>,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn OpenMutexA(
        &self,
        dw_desired_access: u32,
        b_inherit_handle: super::super::Foundation::BOOL,
        lp_name: crate::core::PCSTR,
    ) -> super::super::Foundation::HANDLE {
        todo!()
    }
    fn OpenSemaphoreA(
        &self,
        dw_desired_access: u32,
        b_inherit_handle: super::super::Foundation::BOOL,
        lp_name: crate::core::PCSTR,
    ) -> super::super::Foundation::HANDLE {
        todo!()
    }
    fn OpenWaitableTimerA(
        &self,
        dw_desired_access: u32,
        b_inherit_handle: super::super::Foundation::BOOL,
        lp_timer_name: crate::core::PCSTR,
    ) -> super::super::Foundation::HANDLE {
        todo!()
    }
    fn QueryAuxiliaryCounterFrequency(
        &self,
        lp_auxiliary_counter_frequency: MutPtr<u64>,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn QueryIdleProcessorCycleTime(
        &self,
        buffer_length: MutPtr<u32>,
        processor_idle_cycle_time: MutPtr<u64>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn QueryIdleProcessorCycleTimeEx(
        &self,
        group: u16,
        buffer_length: MutPtr<u32>,
        processor_idle_cycle_time: MutPtr<u64>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn QueryInterruptTime(&self, lp_interrupt_time: MutPtr<u64>) {
        todo!()
    }
    fn QueryInterruptTimePrecise(&self, lp_interrupt_time_precise: MutPtr<u64>) {
        todo!()
    }
    fn QueryProcessCycleTime(
        &self,
        process_handle: super::super::Foundation::HANDLE,
        cycle_time: MutPtr<u64>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn QueryThreadCycleTime(
        &self,
        thread_handle: super::super::Foundation::HANDLE,
        cycle_time: MutPtr<u64>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn QueryUnbiasedInterruptTime(
        &self,
        unbiased_time: MutPtr<u64>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn QueryUnbiasedInterruptTimePrecise(&self, lp_unbiased_interrupt_time_precise: MutPtr<u64>) {
        todo!()
    }
    fn RaiseCustomSystemEventTrigger(
        &self,
        custom_system_event_trigger_config: ConstPtr<CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG>,
    ) -> u32 {
        todo!()
    }
    fn RebootCheckOnInstallA(
        &self,
        hwnd: super::super::Foundation::HWND,
        psz_inf: crate::core::PCSTR,
        psz_sec: crate::core::PCSTR,
        dw_reserved: u32,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn RebootCheckOnInstallW(
        &self,
        hwnd: super::super::Foundation::HWND,
        psz_inf: crate::core::PCWSTR,
        psz_sec: crate::core::PCWSTR,
        dw_reserved: u32,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn RecordFeatureError(&self, feature_id: u32, error: ConstPtr<FEATURE_ERROR>) {
        todo!()
    }
    fn RecordFeatureUsage(
        &self,
        feature_id: u32,
        kind: u32,
        addend: u32,
        origin_name: crate::core::PCSTR,
    ) {
        todo!()
    }
    fn RegInstallA(
        &self,
        hmod: super::super::Foundation::HINSTANCE,
        psz_section: crate::core::PCSTR,
        pst_table: ConstPtr<STRTABLEA>,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn RegInstallW(
        &self,
        hmod: super::super::Foundation::HINSTANCE,
        psz_section: crate::core::PCWSTR,
        pst_table: ConstPtr<STRTABLEW>,
    ) -> crate::core::HRESULT {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Registry'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn RegRestoreAllA(
        &self,
        h_wnd: super::super::Foundation::HWND,
        psz_title_string: crate::core::PCSTR,
        hk_bckup_key: super::Registry::HKEY,
    ) -> crate::core::HRESULT {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Registry'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn RegRestoreAllW(
        &self,
        h_wnd: super::super::Foundation::HWND,
        psz_title_string: crate::core::PCWSTR,
        hk_bckup_key: super::Registry::HKEY,
    ) -> crate::core::HRESULT {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Registry'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn RegSaveRestoreA(
        &self,
        h_wnd: super::super::Foundation::HWND,
        psz_title_string: crate::core::PCSTR,
        hk_bckup_key: super::Registry::HKEY,
        pcsz_root_key: crate::core::PCSTR,
        pcsz_sub_key: crate::core::PCSTR,
        pcsz_value_name: crate::core::PCSTR,
        dw_flags: u32,
    ) -> crate::core::HRESULT {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Registry'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn RegSaveRestoreOnINFA(
        &self,
        h_wnd: super::super::Foundation::HWND,
        psz_title: crate::core::PCSTR,
        psz_inf: crate::core::PCSTR,
        psz_section: crate::core::PCSTR,
        h_hklm_back_key: super::Registry::HKEY,
        h_hkcu_back_key: super::Registry::HKEY,
        dw_flags: u32,
    ) -> crate::core::HRESULT {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Registry'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn RegSaveRestoreOnINFW(
        &self,
        h_wnd: super::super::Foundation::HWND,
        psz_title: crate::core::PCWSTR,
        psz_inf: crate::core::PCWSTR,
        psz_section: crate::core::PCWSTR,
        h_hklm_back_key: super::Registry::HKEY,
        h_hkcu_back_key: super::Registry::HKEY,
        dw_flags: u32,
    ) -> crate::core::HRESULT {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Registry'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn RegSaveRestoreW(
        &self,
        h_wnd: super::super::Foundation::HWND,
        psz_title_string: crate::core::PCWSTR,
        hk_bckup_key: super::Registry::HKEY,
        pcsz_root_key: crate::core::PCWSTR,
        pcsz_sub_key: crate::core::PCWSTR,
        pcsz_value_name: crate::core::PCWSTR,
        dw_flags: u32,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn ReplacePartitionUnit(
        &self,
        target_partition: crate::core::PCWSTR,
        spare_partition: crate::core::PCWSTR,
        flags: u32,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn RequestDeviceWakeup(
        &self,
        h_device: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn RtlAnsiStringToUnicodeString(
        &self,
        destination_string: MutPtr<super::super::Foundation::UNICODE_STRING>,
        source_string: MutPtr<super::Kernel::STRING>,
        allocate_destination_string: super::super::Foundation::BOOLEAN,
    ) -> super::super::Foundation::NTSTATUS {
        todo!()
    }
    fn RtlCharToInteger(
        &self,
        string: MutPtr<i8>,
        base: u32,
        value: MutPtr<u32>,
    ) -> super::super::Foundation::NTSTATUS {
        todo!()
    }
    fn RtlFreeAnsiString(&self, ansi_string: MutPtr<super::Kernel::STRING>) {
        todo!()
    }
    fn RtlFreeOemString(&self, oem_string: MutPtr<super::Kernel::STRING>) {
        todo!()
    }
    fn RtlFreeUnicodeString(
        &self,
        unicode_string: MutPtr<super::super::Foundation::UNICODE_STRING>,
    ) {
        todo!()
    }
    fn RtlGetReturnAddressHijackTarget(&self) -> PtrRepr {
        todo!()
    }
    fn RtlInitAnsiString(
        &self,
        destination_string: MutPtr<super::Kernel::STRING>,
        source_string: MutPtr<i8>,
    ) {
        todo!()
    }
    fn RtlInitAnsiStringEx(
        &self,
        destination_string: MutPtr<super::Kernel::STRING>,
        source_string: MutPtr<i8>,
    ) -> super::super::Foundation::NTSTATUS {
        todo!()
    }
    fn RtlInitString(
        &self,
        destination_string: MutPtr<super::Kernel::STRING>,
        source_string: MutPtr<i8>,
    ) {
        todo!()
    }
    fn RtlInitStringEx(
        &self,
        destination_string: MutPtr<super::Kernel::STRING>,
        source_string: MutPtr<i8>,
    ) -> super::super::Foundation::NTSTATUS {
        todo!()
    }
    fn RtlInitUnicodeString(
        &self,
        destination_string: MutPtr<super::super::Foundation::UNICODE_STRING>,
        source_string: crate::core::PCWSTR,
    ) {
        todo!()
    }
    fn RtlIsNameLegalDOS8Dot3(
        &self,
        name: MutPtr<super::super::Foundation::UNICODE_STRING>,
        oem_name: MutPtr<super::Kernel::STRING>,
        name_contains_spaces: MutPtr<super::super::Foundation::BOOLEAN>,
    ) -> super::super::Foundation::BOOLEAN {
        todo!()
    }
    fn RtlLocalTimeToSystemTime(
        &self,
        local_time: MutPtr<i64>,
        system_time: MutPtr<i64>,
    ) -> super::super::Foundation::NTSTATUS {
        todo!()
    }
    fn RtlRaiseCustomSystemEventTrigger(
        &self,
        trigger_config: ConstPtr<CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG>,
    ) -> u32 {
        todo!()
    }
    fn RtlTimeToSecondsSince1970(
        &self,
        time: MutPtr<i64>,
        elapsed_seconds: MutPtr<u32>,
    ) -> super::super::Foundation::BOOLEAN {
        todo!()
    }
    fn RtlUnicodeStringToAnsiString(
        &self,
        destination_string: MutPtr<super::Kernel::STRING>,
        source_string: MutPtr<super::super::Foundation::UNICODE_STRING>,
        allocate_destination_string: super::super::Foundation::BOOLEAN,
    ) -> super::super::Foundation::NTSTATUS {
        todo!()
    }
    fn RtlUnicodeStringToOemString(
        &self,
        destination_string: MutPtr<super::Kernel::STRING>,
        source_string: MutPtr<super::super::Foundation::UNICODE_STRING>,
        allocate_destination_string: super::super::Foundation::BOOLEAN,
    ) -> super::super::Foundation::NTSTATUS {
        todo!()
    }
    fn RtlUnicodeToMultiByteSize(
        &self,
        bytes_in_multi_byte_string: MutPtr<u32>,
        unicode_string: crate::core::PCWSTR,
        bytes_in_unicode_string: u32,
    ) -> super::super::Foundation::NTSTATUS {
        todo!()
    }
    fn RtlUniform(&self, seed: MutPtr<u32>) -> u32 {
        todo!()
    }
    fn RunSetupCommandA(
        &self,
        h_wnd: super::super::Foundation::HWND,
        sz_cmd_name: crate::core::PCSTR,
        sz_inf_section: crate::core::PCSTR,
        sz_dir: crate::core::PCSTR,
        lpsz_title: crate::core::PCSTR,
        ph_exe: MutPtr<super::super::Foundation::HANDLE>,
        dw_flags: u32,
        pv_reserved: MutPtr<::core::ffi::c_void>,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn RunSetupCommandW(
        &self,
        h_wnd: super::super::Foundation::HWND,
        sz_cmd_name: crate::core::PCWSTR,
        sz_inf_section: crate::core::PCWSTR,
        sz_dir: crate::core::PCWSTR,
        lpsz_title: crate::core::PCWSTR,
        ph_exe: MutPtr<super::super::Foundation::HANDLE>,
        dw_flags: u32,
        pv_reserved: MutPtr<::core::ffi::c_void>,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn SendIMEMessageExA(
        &self,
        param_0: super::super::Foundation::HWND,
        param_1: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::LRESULT {
        todo!()
    }
    fn SendIMEMessageExW(
        &self,
        param_0: super::super::Foundation::HWND,
        param_1: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::LRESULT {
        todo!()
    }
    fn SetEnvironmentStringsA(
        &self,
        new_environment: crate::core::PCSTR,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetFirmwareEnvironmentVariableA(
        &self,
        lp_name: crate::core::PCSTR,
        lp_guid: crate::core::PCSTR,
        p_value: ConstPtr<::core::ffi::c_void>,
        n_size: u32,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetFirmwareEnvironmentVariableExA(
        &self,
        lp_name: crate::core::PCSTR,
        lp_guid: crate::core::PCSTR,
        p_value: ConstPtr<::core::ffi::c_void>,
        n_size: u32,
        dw_attributes: u32,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetFirmwareEnvironmentVariableExW(
        &self,
        lp_name: crate::core::PCWSTR,
        lp_guid: crate::core::PCWSTR,
        p_value: ConstPtr<::core::ffi::c_void>,
        n_size: u32,
        dw_attributes: u32,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetFirmwareEnvironmentVariableW(
        &self,
        lp_name: crate::core::PCWSTR,
        lp_guid: crate::core::PCWSTR,
        p_value: ConstPtr<::core::ffi::c_void>,
        n_size: u32,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetHandleCount(&self, u_number: u32) -> u32 {
        todo!()
    }
    fn SetMessageWaitingIndicator(
        &self,
        h_msg_indicator: super::super::Foundation::HANDLE,
        ul_msg_count: u32,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetPerUserSecValuesA(&self, p_per_user: MutPtr<PERUSERSECTIONA>) -> crate::core::HRESULT {
        todo!()
    }
    fn SetPerUserSecValuesW(&self, p_per_user: MutPtr<PERUSERSECTIONW>) -> crate::core::HRESULT {
        todo!()
    }
    fn SignalObjectAndWait(
        &self,
        h_object_to_signal: super::super::Foundation::HANDLE,
        h_object_to_wait_on: super::super::Foundation::HANDLE,
        dw_milliseconds: u32,
        b_alertable: super::super::Foundation::BOOL,
    ) -> u32 {
        todo!()
    }
    fn SubscribeFeatureStateChangeNotification(
        &self,
        subscription: MutPtr<FEATURE_STATE_CHANGE_SUBSCRIPTION>,
        callback: PFEATURE_STATE_CHANGE_CALLBACK,
        context: ConstPtr<::core::ffi::c_void>,
    ) {
        todo!()
    }
    fn TranslateInfStringA(
        &self,
        psz_inf_filename: crate::core::PCSTR,
        psz_install_section: crate::core::PCSTR,
        psz_translate_section: crate::core::PCSTR,
        psz_translate_key: crate::core::PCSTR,
        psz_buffer: crate::core::PSTR,
        cch_buffer: u32,
        pdw_required_size: MutPtr<u32>,
        pv_reserved: MutPtr<::core::ffi::c_void>,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn TranslateInfStringExA(
        &self,
        h_inf: MutPtr<::core::ffi::c_void>,
        psz_inf_filename: crate::core::PCSTR,
        psz_translate_section: crate::core::PCSTR,
        psz_translate_key: crate::core::PCSTR,
        psz_buffer: crate::core::PSTR,
        dw_buffer_size: u32,
        pdw_required_size: MutPtr<u32>,
        pv_reserved: MutPtr<::core::ffi::c_void>,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn TranslateInfStringExW(
        &self,
        h_inf: MutPtr<::core::ffi::c_void>,
        psz_inf_filename: crate::core::PCWSTR,
        psz_translate_section: crate::core::PCWSTR,
        psz_translate_key: crate::core::PCWSTR,
        psz_buffer: crate::core::PWSTR,
        dw_buffer_size: u32,
        pdw_required_size: MutPtr<u32>,
        pv_reserved: MutPtr<::core::ffi::c_void>,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn TranslateInfStringW(
        &self,
        psz_inf_filename: crate::core::PCWSTR,
        psz_install_section: crate::core::PCWSTR,
        psz_translate_section: crate::core::PCWSTR,
        psz_translate_key: crate::core::PCWSTR,
        psz_buffer: crate::core::PWSTR,
        cch_buffer: u32,
        pdw_required_size: MutPtr<u32>,
        pv_reserved: MutPtr<::core::ffi::c_void>,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn UnsubscribeFeatureStateChangeNotification(
        &self,
        subscription: FEATURE_STATE_CHANGE_SUBSCRIPTION,
    ) {
        todo!()
    }
    fn UserInstStubWrapperA(
        &self,
        hwnd: super::super::Foundation::HWND,
        h_instance: super::super::Foundation::HINSTANCE,
        psz_parms: crate::core::PCSTR,
        n_show: i32,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn UserInstStubWrapperW(
        &self,
        hwnd: super::super::Foundation::HWND,
        h_instance: super::super::Foundation::HINSTANCE,
        psz_parms: crate::core::PCWSTR,
        n_show: i32,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn UserUnInstStubWrapperA(
        &self,
        hwnd: super::super::Foundation::HWND,
        h_instance: super::super::Foundation::HINSTANCE,
        psz_parms: crate::core::PCSTR,
        n_show: i32,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn UserUnInstStubWrapperW(
        &self,
        hwnd: super::super::Foundation::HWND,
        h_instance: super::super::Foundation::HINSTANCE,
        psz_parms: crate::core::PCWSTR,
        n_show: i32,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn WINNLSEnableIME(
        &self,
        param_0: super::super::Foundation::HWND,
        param_1: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn WINNLSGetEnableStatus(
        &self,
        param_0: super::super::Foundation::HWND,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn WINNLSGetIMEHotkey(&self, param_0: super::super::Foundation::HWND) -> u32 {
        todo!()
    }
    fn WldpGetLockdownPolicy(
        &self,
        host_information: ConstPtr<WLDP_HOST_INFORMATION>,
        lockdown_state: MutPtr<u32>,
        lockdown_flags: u32,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn WldpIsClassInApprovedList(
        &self,
        class_id: ConstPtr<crate::core::GUID>,
        host_information: ConstPtr<WLDP_HOST_INFORMATION>,
        is_approved: MutPtr<super::super::Foundation::BOOL>,
        optional_flags: u32,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn WldpIsDynamicCodePolicyEnabled(
        &self,
        is_enabled: MutPtr<super::super::Foundation::BOOL>,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn WldpQueryDeviceSecurityInformation(
        &self,
        information: MutPtr<WLDP_DEVICE_SECURITY_INFORMATION>,
        information_length: u32,
        return_length: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn WldpQueryDynamicCodeTrust(
        &self,
        file_handle: super::super::Foundation::HANDLE,
        base_image: ConstPtr<::core::ffi::c_void>,
        image_size: u32,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn WldpSetDynamicCodeTrust(
        &self,
        file_handle: super::super::Foundation::HANDLE,
    ) -> crate::core::HRESULT {
        todo!()
    }
    fn WritePrivateProfileSectionA(
        &self,
        lp_app_name: crate::core::PCSTR,
        lp_string: crate::core::PCSTR,
        lp_file_name: crate::core::PCSTR,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn WritePrivateProfileSectionW(
        &self,
        lp_app_name: crate::core::PCWSTR,
        lp_string: crate::core::PCWSTR,
        lp_file_name: crate::core::PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn WritePrivateProfileStringA(
        &self,
        lp_app_name: crate::core::PCSTR,
        lp_key_name: crate::core::PCSTR,
        lp_string: crate::core::PCSTR,
        lp_file_name: crate::core::PCSTR,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn WritePrivateProfileStringW(
        &self,
        lp_app_name: crate::core::PCWSTR,
        lp_key_name: crate::core::PCWSTR,
        lp_string: crate::core::PCWSTR,
        lp_file_name: crate::core::PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn WritePrivateProfileStructA(
        &self,
        lpsz_section: crate::core::PCSTR,
        lpsz_key: crate::core::PCSTR,
        lp_struct: ConstPtr<::core::ffi::c_void>,
        u_size_struct: u32,
        sz_file: crate::core::PCSTR,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn WritePrivateProfileStructW(
        &self,
        lpsz_section: crate::core::PCWSTR,
        lpsz_key: crate::core::PCWSTR,
        lp_struct: ConstPtr<::core::ffi::c_void>,
        u_size_struct: u32,
        sz_file: crate::core::PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn WriteProfileSectionA(
        &self,
        lp_app_name: crate::core::PCSTR,
        lp_string: crate::core::PCSTR,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn WriteProfileSectionW(
        &self,
        lp_app_name: crate::core::PCWSTR,
        lp_string: crate::core::PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn WriteProfileStringA(
        &self,
        lp_app_name: crate::core::PCSTR,
        lp_key_name: crate::core::PCSTR,
        lp_string: crate::core::PCSTR,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn WriteProfileStringW(
        &self,
        lp_app_name: crate::core::PCWSTR,
        lp_key_name: crate::core::PCWSTR,
        lp_string: crate::core::PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn _hread(&self, h_file: i32, lp_buffer: MutPtr<::core::ffi::c_void>, l_bytes: i32) -> i32 {
        todo!()
    }
    fn _hwrite(&self, h_file: i32, lp_buffer: crate::core::PCSTR, l_bytes: i32) -> i32 {
        todo!()
    }
    fn _lclose(&self, h_file: i32) -> i32 {
        todo!()
    }
    fn _lcreat(&self, lp_path_name: crate::core::PCSTR, i_attribute: i32) -> i32 {
        todo!()
    }
    fn _llseek(&self, h_file: i32, l_offset: i32, i_origin: i32) -> i32 {
        todo!()
    }
    fn _lopen(&self, lp_path_name: crate::core::PCSTR, i_read_write: i32) -> i32 {
        todo!()
    }
    fn _lread(&self, h_file: i32, lp_buffer: MutPtr<::core::ffi::c_void>, u_bytes: u32) -> u32 {
        todo!()
    }
    fn _lwrite(&self, h_file: i32, lp_buffer: crate::core::PCSTR, u_bytes: u32) -> u32 {
        todo!()
    }
    #[doc = "*Required namespaces: *"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn uaw_lstrcmpW(&self, string_1: ConstPtr<u16>, string_2: ConstPtr<u16>) -> i32 {
        todo!()
    }
    #[doc = "*Required namespaces: *"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn uaw_lstrcmpiW(&self, string_1: ConstPtr<u16>, string_2: ConstPtr<u16>) -> i32 {
        todo!()
    }
    #[doc = "*Required namespaces: *"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn uaw_lstrlenW(&self, string: ConstPtr<u16>) -> i32 {
        todo!()
    }
    #[doc = "*Required namespaces: *"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn uaw_wcschr(&self, string: ConstPtr<u16>, character: u16) -> MutPtr<u16> {
        todo!()
    }
    #[doc = "*Required namespaces: *"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn uaw_wcscpy(&self, destination: MutPtr<u16>, source: ConstPtr<u16>) -> MutPtr<u16> {
        todo!()
    }
    #[doc = "*Required namespaces: *"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn uaw_wcsicmp(&self, string_1: ConstPtr<u16>, string_2: ConstPtr<u16>) -> i32 {
        todo!()
    }
    #[doc = "*Required namespaces: *"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn uaw_wcslen(&self, string: ConstPtr<u16>) -> PtrRepr {
        todo!()
    }
    #[doc = "*Required namespaces: *"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn uaw_wcsrchr(&self, string: ConstPtr<u16>, character: u16) -> MutPtr<u16> {
        todo!()
    }
}
pub fn get_api(ctx: &crate::core::Win32Context) -> &dyn Api {
    ctx.get::<dyn Api>()
}
