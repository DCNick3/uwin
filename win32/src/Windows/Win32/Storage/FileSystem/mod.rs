#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    unused_assignments,
    clippy::all
)]
#[allow(unused)]
use win32::core::prelude::*;
pub struct BY_HANDLE_FILE_INFORMATION {
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::super::Foundation::FILETIME,
    pub ftLastAccessTime: super::super::Foundation::FILETIME,
    pub ftLastWriteTime: super::super::Foundation::FILETIME,
    pub dwVolumeSerialNumber: u32,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
    pub nNumberOfLinks: u32,
    pub nFileIndexHigh: u32,
    pub nFileIndexLow: u32,
}
impl ::core::marker::Copy for BY_HANDLE_FILE_INFORMATION {}
impl ::core::clone::Clone for BY_HANDLE_FILE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BY_HANDLE_FILE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BY_HANDLE_FILE_INFORMATION")
            .field("dwFileAttributes", &self.dwFileAttributes)
            .field("ftCreationTime", &self.ftCreationTime)
            .field("ftLastAccessTime", &self.ftLastAccessTime)
            .field("ftLastWriteTime", &self.ftLastWriteTime)
            .field("dwVolumeSerialNumber", &self.dwVolumeSerialNumber)
            .field("nFileSizeHigh", &self.nFileSizeHigh)
            .field("nFileSizeLow", &self.nFileSizeLow)
            .field("nNumberOfLinks", &self.nNumberOfLinks)
            .field("nFileIndexHigh", &self.nFileIndexHigh)
            .field("nFileIndexLow", &self.nFileIndexLow)
            .finish()
    }
}
impl ::core::cmp::PartialEq for BY_HANDLE_FILE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwFileAttributes == other.dwFileAttributes
            && self.ftCreationTime == other.ftCreationTime
            && self.ftLastAccessTime == other.ftLastAccessTime
            && self.ftLastWriteTime == other.ftLastWriteTime
            && self.dwVolumeSerialNumber == other.dwVolumeSerialNumber
            && self.nFileSizeHigh == other.nFileSizeHigh
            && self.nFileSizeLow == other.nFileSizeLow
            && self.nNumberOfLinks == other.nNumberOfLinks
            && self.nFileIndexHigh == other.nFileIndexHigh
            && self.nFileIndexLow == other.nFileIndexLow
    }
}
impl ::core::cmp::Eq for BY_HANDLE_FILE_INFORMATION {}
impl FromIntoMemory for BY_HANDLE_FILE_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 52);
        let f_dwFileAttributes = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ftCreationTime =
            <super::super::Foundation::FILETIME as FromIntoMemory>::from_bytes(&from[4..4 + 8]);
        let f_ftLastAccessTime =
            <super::super::Foundation::FILETIME as FromIntoMemory>::from_bytes(&from[12..12 + 8]);
        let f_ftLastWriteTime =
            <super::super::Foundation::FILETIME as FromIntoMemory>::from_bytes(&from[20..20 + 8]);
        let f_dwVolumeSerialNumber = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_nFileSizeHigh = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_nFileSizeLow = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_nNumberOfLinks = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_nFileIndexHigh = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_nFileIndexLow = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        Self {
            dwFileAttributes: f_dwFileAttributes,
            ftCreationTime: f_ftCreationTime,
            ftLastAccessTime: f_ftLastAccessTime,
            ftLastWriteTime: f_ftLastWriteTime,
            dwVolumeSerialNumber: f_dwVolumeSerialNumber,
            nFileSizeHigh: f_nFileSizeHigh,
            nFileSizeLow: f_nFileSizeLow,
            nNumberOfLinks: f_nNumberOfLinks,
            nFileIndexHigh: f_nFileIndexHigh,
            nFileIndexLow: f_nFileIndexLow,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 52);
        FromIntoMemory::into_bytes(self.dwFileAttributes, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ftCreationTime, &mut into[4..4 + 8]);
        FromIntoMemory::into_bytes(self.ftLastAccessTime, &mut into[12..12 + 8]);
        FromIntoMemory::into_bytes(self.ftLastWriteTime, &mut into[20..20 + 8]);
        FromIntoMemory::into_bytes(self.dwVolumeSerialNumber, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.nFileSizeHigh, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.nFileSizeLow, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.nNumberOfLinks, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.nFileIndexHigh, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.nFileIndexLow, &mut into[48..48 + 4]);
    }
    fn size() -> usize {
        52
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub type CACHE_ACCESS_CHECK = StdCallFnPtr<
    (
        MutPtr<super::super::Security::SECURITY_DESCRIPTOR>,
        super::super::Foundation::HANDLE,
        u32,
        MutPtr<super::super::Security::GENERIC_MAPPING>,
        MutPtr<super::super::Security::PRIVILEGE_SET>,
        MutPtr<u32>,
        MutPtr<u32>,
        MutPtr<i32>,
    ),
    super::super::Foundation::BOOL,
>;
pub type CACHE_DESTROY_CALLBACK = StdCallFnPtr<(u32, MutPtr<u8>), ()>;
pub type CACHE_KEY_COMPARE = StdCallFnPtr<(u32, MutPtr<u8>, u32, MutPtr<u8>), i32>;
pub type CACHE_KEY_HASH = StdCallFnPtr<(MutPtr<u8>, u32), u32>;
pub type CACHE_READ_CALLBACK =
    StdCallFnPtr<(u32, MutPtr<u8>, MutPtr<::core::ffi::c_void>), super::super::Foundation::BOOL>;
pub type CLAIMMEDIALABEL = StdCallFnPtr<(ConstPtr<u8>, u32, MutPtr<MediaLabelInfo>), u32>;
pub type CLAIMMEDIALABELEX = StdCallFnPtr<
    (
        ConstPtr<u8>,
        u32,
        MutPtr<MediaLabelInfo>,
        MutPtr<crate::core::GUID>,
    ),
    u32,
>;
pub const CLFS_BASELOG_EXTENSION: &'static str = ".blf";
pub type CLFS_BLOCK_ALLOCATION =
    StdCallFnPtr<(u32, MutPtr<::core::ffi::c_void>), MutPtr<::core::ffi::c_void>>;
pub type CLFS_BLOCK_DEALLOCATION =
    StdCallFnPtr<(MutPtr<::core::ffi::c_void>, MutPtr<::core::ffi::c_void>), ()>;
pub const CLFS_CONTAINER_RELATIVE_PREFIX: &'static str = "%BLF%\\";
pub const CLFS_CONTAINER_STREAM_PREFIX: &'static str = "%BLF%:";
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CLFS_CONTEXT_MODE(pub i32);
pub const ClfsContextNone: CLFS_CONTEXT_MODE = CLFS_CONTEXT_MODE(0i32);
pub const ClfsContextUndoNext: CLFS_CONTEXT_MODE = CLFS_CONTEXT_MODE(1i32);
pub const ClfsContextPrevious: CLFS_CONTEXT_MODE = CLFS_CONTEXT_MODE(2i32);
pub const ClfsContextForward: CLFS_CONTEXT_MODE = CLFS_CONTEXT_MODE(3i32);
impl ::core::marker::Copy for CLFS_CONTEXT_MODE {}
impl ::core::clone::Clone for CLFS_CONTEXT_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLFS_CONTEXT_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLFS_CONTEXT_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLFS_CONTEXT_MODE").field(&self.0).finish()
    }
}
impl FromIntoMemory for CLFS_CONTEXT_MODE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CLFS_FLAG(pub u32);
pub const CLFS_FLAG_FORCE_APPEND: CLFS_FLAG = CLFS_FLAG(1u32);
pub const CLFS_FLAG_FORCE_FLUSH: CLFS_FLAG = CLFS_FLAG(2u32);
pub const CLFS_FLAG_NO_FLAGS: CLFS_FLAG = CLFS_FLAG(0u32);
pub const CLFS_FLAG_USE_RESERVATION: CLFS_FLAG = CLFS_FLAG(4u32);
impl ::core::marker::Copy for CLFS_FLAG {}
impl ::core::clone::Clone for CLFS_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLFS_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLFS_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLFS_FLAG").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CLFS_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CLFS_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CLFS_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CLFS_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CLFS_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for CLFS_FLAG {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub const CLFS_FLAG_FILTER_INTERMEDIATE_LEVEL: u32 = 16u32;
pub const CLFS_FLAG_FILTER_TOP_LEVEL: u32 = 32u32;
pub const CLFS_FLAG_HIDDEN_SYSTEM_LOG: u32 = 512u32;
pub const CLFS_FLAG_IGNORE_SHARE_ACCESS: u32 = 64u32;
pub const CLFS_FLAG_MINIFILTER_LEVEL: u32 = 256u32;
pub const CLFS_FLAG_NON_REENTRANT_FILTER: u32 = 16u32;
pub const CLFS_FLAG_READ_IN_PROGRESS: u32 = 128u32;
pub const CLFS_FLAG_REENTRANT_FILE_SYSTEM: u32 = 8u32;
pub const CLFS_FLAG_REENTRANT_FILTER: u32 = 32u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CLFS_IOSTATS_CLASS(pub i32);
pub const ClfsIoStatsDefault: CLFS_IOSTATS_CLASS = CLFS_IOSTATS_CLASS(0i32);
pub const ClfsIoStatsMax: CLFS_IOSTATS_CLASS = CLFS_IOSTATS_CLASS(65535i32);
impl ::core::marker::Copy for CLFS_IOSTATS_CLASS {}
impl ::core::clone::Clone for CLFS_IOSTATS_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLFS_IOSTATS_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLFS_IOSTATS_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLFS_IOSTATS_CLASS").field(&self.0).finish()
    }
}
impl FromIntoMemory for CLFS_IOSTATS_CLASS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CLFS_LOG_ARCHIVE_MODE(pub i32);
pub const ClfsLogArchiveEnabled: CLFS_LOG_ARCHIVE_MODE = CLFS_LOG_ARCHIVE_MODE(1i32);
pub const ClfsLogArchiveDisabled: CLFS_LOG_ARCHIVE_MODE = CLFS_LOG_ARCHIVE_MODE(2i32);
impl ::core::marker::Copy for CLFS_LOG_ARCHIVE_MODE {}
impl ::core::clone::Clone for CLFS_LOG_ARCHIVE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLFS_LOG_ARCHIVE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLFS_LOG_ARCHIVE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLFS_LOG_ARCHIVE_MODE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for CLFS_LOG_ARCHIVE_MODE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct CLFS_LOG_NAME_INFORMATION {
    pub NameLengthInBytes: u16,
    pub Name: [u16; 1],
}
impl ::core::marker::Copy for CLFS_LOG_NAME_INFORMATION {}
impl ::core::clone::Clone for CLFS_LOG_NAME_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_LOG_NAME_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_LOG_NAME_INFORMATION")
            .field("NameLengthInBytes", &self.NameLengthInBytes)
            .field("Name", &self.Name)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLFS_LOG_NAME_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NameLengthInBytes == other.NameLengthInBytes && self.Name == other.Name
    }
}
impl ::core::cmp::Eq for CLFS_LOG_NAME_INFORMATION {}
impl FromIntoMemory for CLFS_LOG_NAME_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_NameLengthInBytes = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_Name = <[u16; 1] as FromIntoMemory>::from_bytes(&from[2..2 + 1]);
        Self {
            NameLengthInBytes: f_NameLengthInBytes,
            Name: f_Name,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.NameLengthInBytes, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.Name, &mut into[2..2 + 1]);
    }
    fn size() -> usize {
        4
    }
}
pub const CLFS_MARSHALLING_FLAG_DISABLE_BUFF_INIT: u32 = 1u32;
pub const CLFS_MARSHALLING_FLAG_NONE: u32 = 0u32;
pub const CLFS_MAX_CONTAINER_INFO: u32 = 256u32;
pub const CLFS_MGMT_CLIENT_REGISTRATION_VERSION: u32 = 1u32;
pub struct CLFS_MGMT_NOTIFICATION {
    pub Notification: CLFS_MGMT_NOTIFICATION_TYPE,
    pub Lsn: CLS_LSN,
    pub LogIsPinned: u16,
}
impl ::core::marker::Copy for CLFS_MGMT_NOTIFICATION {}
impl ::core::clone::Clone for CLFS_MGMT_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_MGMT_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_MGMT_NOTIFICATION")
            .field("Notification", &self.Notification)
            .field("Lsn", &self.Lsn)
            .field("LogIsPinned", &self.LogIsPinned)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLFS_MGMT_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.Notification == other.Notification
            && self.Lsn == other.Lsn
            && self.LogIsPinned == other.LogIsPinned
    }
}
impl ::core::cmp::Eq for CLFS_MGMT_NOTIFICATION {}
impl FromIntoMemory for CLFS_MGMT_NOTIFICATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_Notification =
            <CLFS_MGMT_NOTIFICATION_TYPE as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Lsn = <CLS_LSN as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_LogIsPinned = <u16 as FromIntoMemory>::from_bytes(&from[16..16 + 2]);
        Self {
            Notification: f_Notification,
            Lsn: f_Lsn,
            LogIsPinned: f_LogIsPinned,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.Notification, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Lsn, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.LogIsPinned, &mut into[16..16 + 2]);
    }
    fn size() -> usize {
        24
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CLFS_MGMT_NOTIFICATION_TYPE(pub i32);
pub const ClfsMgmtAdvanceTailNotification: CLFS_MGMT_NOTIFICATION_TYPE =
    CLFS_MGMT_NOTIFICATION_TYPE(0i32);
pub const ClfsMgmtLogFullHandlerNotification: CLFS_MGMT_NOTIFICATION_TYPE =
    CLFS_MGMT_NOTIFICATION_TYPE(1i32);
pub const ClfsMgmtLogUnpinnedNotification: CLFS_MGMT_NOTIFICATION_TYPE =
    CLFS_MGMT_NOTIFICATION_TYPE(2i32);
pub const ClfsMgmtLogWriteNotification: CLFS_MGMT_NOTIFICATION_TYPE =
    CLFS_MGMT_NOTIFICATION_TYPE(3i32);
impl ::core::marker::Copy for CLFS_MGMT_NOTIFICATION_TYPE {}
impl ::core::clone::Clone for CLFS_MGMT_NOTIFICATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLFS_MGMT_NOTIFICATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLFS_MGMT_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLFS_MGMT_NOTIFICATION_TYPE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for CLFS_MGMT_NOTIFICATION_TYPE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct CLFS_MGMT_POLICY {
    pub Version: u32,
    pub LengthInBytes: u32,
    pub PolicyFlags: u32,
    pub PolicyType: CLFS_MGMT_POLICY_TYPE,
    pub PolicyParameters: CLFS_MGMT_POLICY_0,
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for CLFS_MGMT_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.LengthInBytes == other.LengthInBytes
            && self.PolicyFlags == other.PolicyFlags
            && self.PolicyType == other.PolicyType
            && self.PolicyParameters == other.PolicyParameters
    }
}
impl ::core::cmp::Eq for CLFS_MGMT_POLICY {}
impl FromIntoMemory for CLFS_MGMT_POLICY {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_Version = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_LengthInBytes = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_PolicyFlags = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_PolicyType = <CLFS_MGMT_POLICY_TYPE as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_PolicyParameters =
            <CLFS_MGMT_POLICY_0 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        Self {
            Version: f_Version,
            LengthInBytes: f_LengthInBytes,
            PolicyFlags: f_PolicyFlags,
            PolicyType: f_PolicyType,
            PolicyParameters: f_PolicyParameters,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.Version, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.LengthInBytes, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.PolicyFlags, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.PolicyType, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.PolicyParameters, &mut into[16..16 + 8]);
    }
    fn size() -> usize {
        24
    }
}
pub struct CLFS_MGMT_POLICY_0 {
    data: [u8; 8],
}
impl ::core::default::Default for CLFS_MGMT_POLICY_0 {
    fn default() -> Self {
        Self { data: [0u8; 8] }
    }
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for CLFS_MGMT_POLICY_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for CLFS_MGMT_POLICY_0 {}
impl FromIntoMemory for CLFS_MGMT_POLICY_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 8];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        8
    }
}
pub struct CLFS_MGMT_POLICY_0_0 {
    pub Enabled: u32,
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_0 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_MGMT_POLICY_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_MGMT_POLICY_0_0")
            .field("Enabled", &self.Enabled)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLFS_MGMT_POLICY_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Enabled == other.Enabled
    }
}
impl ::core::cmp::Eq for CLFS_MGMT_POLICY_0_0 {}
impl FromIntoMemory for CLFS_MGMT_POLICY_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_Enabled = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self { Enabled: f_Enabled }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.Enabled, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct CLFS_MGMT_POLICY_0_1 {
    pub Percentage: u32,
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_1 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_MGMT_POLICY_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_MGMT_POLICY_0_1")
            .field("Percentage", &self.Percentage)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLFS_MGMT_POLICY_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Percentage == other.Percentage
    }
}
impl ::core::cmp::Eq for CLFS_MGMT_POLICY_0_1 {}
impl FromIntoMemory for CLFS_MGMT_POLICY_0_1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_Percentage = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            Percentage: f_Percentage,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.Percentage, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct CLFS_MGMT_POLICY_0_2 {
    pub AbsoluteGrowthInContainers: u32,
    pub RelativeGrowthPercentage: u32,
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_2 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_MGMT_POLICY_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_MGMT_POLICY_0_2")
            .field(
                "AbsoluteGrowthInContainers",
                &self.AbsoluteGrowthInContainers,
            )
            .field("RelativeGrowthPercentage", &self.RelativeGrowthPercentage)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLFS_MGMT_POLICY_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.AbsoluteGrowthInContainers == other.AbsoluteGrowthInContainers
            && self.RelativeGrowthPercentage == other.RelativeGrowthPercentage
    }
}
impl ::core::cmp::Eq for CLFS_MGMT_POLICY_0_2 {}
impl FromIntoMemory for CLFS_MGMT_POLICY_0_2 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_AbsoluteGrowthInContainers = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_RelativeGrowthPercentage = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            AbsoluteGrowthInContainers: f_AbsoluteGrowthInContainers,
            RelativeGrowthPercentage: f_RelativeGrowthPercentage,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.AbsoluteGrowthInContainers, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.RelativeGrowthPercentage, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct CLFS_MGMT_POLICY_0_3 {
    pub MinimumAvailablePercentage: u32,
    pub MinimumAvailableContainers: u32,
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_3 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_MGMT_POLICY_0_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_MGMT_POLICY_0_3")
            .field(
                "MinimumAvailablePercentage",
                &self.MinimumAvailablePercentage,
            )
            .field(
                "MinimumAvailableContainers",
                &self.MinimumAvailableContainers,
            )
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLFS_MGMT_POLICY_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.MinimumAvailablePercentage == other.MinimumAvailablePercentage
            && self.MinimumAvailableContainers == other.MinimumAvailableContainers
    }
}
impl ::core::cmp::Eq for CLFS_MGMT_POLICY_0_3 {}
impl FromIntoMemory for CLFS_MGMT_POLICY_0_3 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_MinimumAvailablePercentage = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_MinimumAvailableContainers = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            MinimumAvailablePercentage: f_MinimumAvailablePercentage,
            MinimumAvailableContainers: f_MinimumAvailableContainers,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.MinimumAvailablePercentage, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.MinimumAvailableContainers, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct CLFS_MGMT_POLICY_0_4 {
    pub Containers: u32,
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_4 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_MGMT_POLICY_0_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_MGMT_POLICY_0_4")
            .field("Containers", &self.Containers)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLFS_MGMT_POLICY_0_4 {
    fn eq(&self, other: &Self) -> bool {
        self.Containers == other.Containers
    }
}
impl ::core::cmp::Eq for CLFS_MGMT_POLICY_0_4 {}
impl FromIntoMemory for CLFS_MGMT_POLICY_0_4 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_Containers = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            Containers: f_Containers,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.Containers, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct CLFS_MGMT_POLICY_0_5 {
    pub Containers: u32,
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_5 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_5 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_MGMT_POLICY_0_5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_MGMT_POLICY_0_5")
            .field("Containers", &self.Containers)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLFS_MGMT_POLICY_0_5 {
    fn eq(&self, other: &Self) -> bool {
        self.Containers == other.Containers
    }
}
impl ::core::cmp::Eq for CLFS_MGMT_POLICY_0_5 {}
impl FromIntoMemory for CLFS_MGMT_POLICY_0_5 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_Containers = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            Containers: f_Containers,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.Containers, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct CLFS_MGMT_POLICY_0_6 {
    pub ExtensionLengthInBytes: u16,
    pub ExtensionString: [u16; 1],
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_6 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_MGMT_POLICY_0_6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_MGMT_POLICY_0_6")
            .field("ExtensionLengthInBytes", &self.ExtensionLengthInBytes)
            .field("ExtensionString", &self.ExtensionString)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLFS_MGMT_POLICY_0_6 {
    fn eq(&self, other: &Self) -> bool {
        self.ExtensionLengthInBytes == other.ExtensionLengthInBytes
            && self.ExtensionString == other.ExtensionString
    }
}
impl ::core::cmp::Eq for CLFS_MGMT_POLICY_0_6 {}
impl FromIntoMemory for CLFS_MGMT_POLICY_0_6 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_ExtensionLengthInBytes = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_ExtensionString = <[u16; 1] as FromIntoMemory>::from_bytes(&from[2..2 + 1]);
        Self {
            ExtensionLengthInBytes: f_ExtensionLengthInBytes,
            ExtensionString: f_ExtensionString,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.ExtensionLengthInBytes, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.ExtensionString, &mut into[2..2 + 1]);
    }
    fn size() -> usize {
        4
    }
}
pub struct CLFS_MGMT_POLICY_0_7 {
    pub PrefixLengthInBytes: u16,
    pub PrefixString: [u16; 1],
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_7 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_7 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_MGMT_POLICY_0_7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_MGMT_POLICY_0_7")
            .field("PrefixLengthInBytes", &self.PrefixLengthInBytes)
            .field("PrefixString", &self.PrefixString)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLFS_MGMT_POLICY_0_7 {
    fn eq(&self, other: &Self) -> bool {
        self.PrefixLengthInBytes == other.PrefixLengthInBytes
            && self.PrefixString == other.PrefixString
    }
}
impl ::core::cmp::Eq for CLFS_MGMT_POLICY_0_7 {}
impl FromIntoMemory for CLFS_MGMT_POLICY_0_7 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_PrefixLengthInBytes = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_PrefixString = <[u16; 1] as FromIntoMemory>::from_bytes(&from[2..2 + 1]);
        Self {
            PrefixLengthInBytes: f_PrefixLengthInBytes,
            PrefixString: f_PrefixString,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.PrefixLengthInBytes, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.PrefixString, &mut into[2..2 + 1]);
    }
    fn size() -> usize {
        4
    }
}
pub struct CLFS_MGMT_POLICY_0_8 {
    pub SizeInBytes: u32,
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_8 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_8 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_MGMT_POLICY_0_8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_MGMT_POLICY_0_8")
            .field("SizeInBytes", &self.SizeInBytes)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLFS_MGMT_POLICY_0_8 {
    fn eq(&self, other: &Self) -> bool {
        self.SizeInBytes == other.SizeInBytes
    }
}
impl ::core::cmp::Eq for CLFS_MGMT_POLICY_0_8 {}
impl FromIntoMemory for CLFS_MGMT_POLICY_0_8 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_SizeInBytes = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            SizeInBytes: f_SizeInBytes,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.SizeInBytes, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct CLFS_MGMT_POLICY_0_9 {
    pub NextContainerSuffix: u64,
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_9 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_9 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_MGMT_POLICY_0_9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_MGMT_POLICY_0_9")
            .field("NextContainerSuffix", &self.NextContainerSuffix)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLFS_MGMT_POLICY_0_9 {
    fn eq(&self, other: &Self) -> bool {
        self.NextContainerSuffix == other.NextContainerSuffix
    }
}
impl ::core::cmp::Eq for CLFS_MGMT_POLICY_0_9 {}
impl FromIntoMemory for CLFS_MGMT_POLICY_0_9 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_NextContainerSuffix = <u64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        Self {
            NextContainerSuffix: f_NextContainerSuffix,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.NextContainerSuffix, &mut into[0..0 + 8]);
    }
    fn size() -> usize {
        8
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CLFS_MGMT_POLICY_TYPE(pub i32);
pub const ClfsMgmtPolicyMaximumSize: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(0i32);
pub const ClfsMgmtPolicyMinimumSize: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(1i32);
pub const ClfsMgmtPolicyNewContainerSize: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(2i32);
pub const ClfsMgmtPolicyGrowthRate: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(3i32);
pub const ClfsMgmtPolicyLogTail: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(4i32);
pub const ClfsMgmtPolicyAutoShrink: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(5i32);
pub const ClfsMgmtPolicyAutoGrow: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(6i32);
pub const ClfsMgmtPolicyNewContainerPrefix: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(7i32);
pub const ClfsMgmtPolicyNewContainerSuffix: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(8i32);
pub const ClfsMgmtPolicyNewContainerExtension: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(9i32);
pub const ClfsMgmtPolicyInvalid: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(10i32);
impl ::core::marker::Copy for CLFS_MGMT_POLICY_TYPE {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLFS_MGMT_POLICY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLFS_MGMT_POLICY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLFS_MGMT_POLICY_TYPE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for CLFS_MGMT_POLICY_TYPE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub const CLFS_MGMT_POLICY_VERSION: u32 = 1u32;
pub struct CLFS_NODE_ID {
    pub cType: u32,
    pub cbNode: u32,
}
impl ::core::marker::Copy for CLFS_NODE_ID {}
impl ::core::clone::Clone for CLFS_NODE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_NODE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_NODE_ID")
            .field("cType", &self.cType)
            .field("cbNode", &self.cbNode)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLFS_NODE_ID {
    fn eq(&self, other: &Self) -> bool {
        self.cType == other.cType && self.cbNode == other.cbNode
    }
}
impl ::core::cmp::Eq for CLFS_NODE_ID {}
impl FromIntoMemory for CLFS_NODE_ID {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_cType = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_cbNode = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            cType: f_cType,
            cbNode: f_cbNode,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.cType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.cbNode, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct CLFS_PHYSICAL_LSN_INFORMATION {
    pub StreamIdentifier: u8,
    pub VirtualLsn: CLS_LSN,
    pub PhysicalLsn: CLS_LSN,
}
impl ::core::marker::Copy for CLFS_PHYSICAL_LSN_INFORMATION {}
impl ::core::clone::Clone for CLFS_PHYSICAL_LSN_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_PHYSICAL_LSN_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_PHYSICAL_LSN_INFORMATION")
            .field("StreamIdentifier", &self.StreamIdentifier)
            .field("VirtualLsn", &self.VirtualLsn)
            .field("PhysicalLsn", &self.PhysicalLsn)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLFS_PHYSICAL_LSN_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.StreamIdentifier == other.StreamIdentifier
            && self.VirtualLsn == other.VirtualLsn
            && self.PhysicalLsn == other.PhysicalLsn
    }
}
impl ::core::cmp::Eq for CLFS_PHYSICAL_LSN_INFORMATION {}
impl FromIntoMemory for CLFS_PHYSICAL_LSN_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_StreamIdentifier = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_VirtualLsn = <CLS_LSN as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_PhysicalLsn = <CLS_LSN as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        Self {
            StreamIdentifier: f_StreamIdentifier,
            VirtualLsn: f_VirtualLsn,
            PhysicalLsn: f_PhysicalLsn,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.StreamIdentifier, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.VirtualLsn, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.PhysicalLsn, &mut into[16..16 + 8]);
    }
    fn size() -> usize {
        24
    }
}
pub const CLFS_SCAN_BACKWARD: u8 = 4u8;
pub const CLFS_SCAN_BUFFERED: u8 = 32u8;
pub const CLFS_SCAN_CLOSE: u8 = 8u8;
pub const CLFS_SCAN_FORWARD: u8 = 2u8;
pub const CLFS_SCAN_INIT: u8 = 1u8;
pub const CLFS_SCAN_INITIALIZED: u8 = 16u8;
pub struct CLFS_STREAM_ID_INFORMATION {
    pub StreamIdentifier: u8,
}
impl ::core::marker::Copy for CLFS_STREAM_ID_INFORMATION {}
impl ::core::clone::Clone for CLFS_STREAM_ID_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_STREAM_ID_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_STREAM_ID_INFORMATION")
            .field("StreamIdentifier", &self.StreamIdentifier)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLFS_STREAM_ID_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.StreamIdentifier == other.StreamIdentifier
    }
}
impl ::core::cmp::Eq for CLFS_STREAM_ID_INFORMATION {}
impl FromIntoMemory for CLFS_STREAM_ID_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 1);
        let f_StreamIdentifier = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        Self {
            StreamIdentifier: f_StreamIdentifier,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 1);
        FromIntoMemory::into_bytes(self.StreamIdentifier, &mut into[0..0 + 1]);
    }
    fn size() -> usize {
        1
    }
}
pub const CLSID_DiskQuotaControl: crate::core::GUID =
    crate::core::GUID::from_u128(0x7988b571_ec89_11cf_9c00_00aa00a14f56);
pub struct CLS_ARCHIVE_DESCRIPTOR {
    pub coffLow: u64,
    pub coffHigh: u64,
    pub infoContainer: CLS_CONTAINER_INFORMATION,
}
impl ::core::marker::Copy for CLS_ARCHIVE_DESCRIPTOR {}
impl ::core::clone::Clone for CLS_ARCHIVE_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLS_ARCHIVE_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLS_ARCHIVE_DESCRIPTOR")
            .field("coffLow", &self.coffLow)
            .field("coffHigh", &self.coffHigh)
            .field("infoContainer", &self.infoContainer)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLS_ARCHIVE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.coffLow == other.coffLow
            && self.coffHigh == other.coffHigh
            && self.infoContainer == other.infoContainer
    }
}
impl ::core::cmp::Eq for CLS_ARCHIVE_DESCRIPTOR {}
impl FromIntoMemory for CLS_ARCHIVE_DESCRIPTOR {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 336);
        let f_coffLow = <u64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_coffHigh = <u64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_infoContainer =
            <CLS_CONTAINER_INFORMATION as FromIntoMemory>::from_bytes(&from[16..16 + 320]);
        Self {
            coffLow: f_coffLow,
            coffHigh: f_coffHigh,
            infoContainer: f_infoContainer,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 336);
        FromIntoMemory::into_bytes(self.coffLow, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.coffHigh, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.infoContainer, &mut into[16..16 + 320]);
    }
    fn size() -> usize {
        336
    }
}
pub struct CLS_CONTAINER_INFORMATION {
    pub FileAttributes: u32,
    pub CreationTime: u64,
    pub LastAccessTime: u64,
    pub LastWriteTime: u64,
    pub ContainerSize: i64,
    pub FileNameActualLength: u32,
    pub FileNameLength: u32,
    pub FileName: [u16; 256],
    pub State: u32,
    pub PhysicalContainerId: u32,
    pub LogicalContainerId: u32,
}
impl ::core::marker::Copy for CLS_CONTAINER_INFORMATION {}
impl ::core::clone::Clone for CLS_CONTAINER_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLS_CONTAINER_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLS_CONTAINER_INFORMATION")
            .field("FileAttributes", &self.FileAttributes)
            .field("CreationTime", &self.CreationTime)
            .field("LastAccessTime", &self.LastAccessTime)
            .field("LastWriteTime", &self.LastWriteTime)
            .field("ContainerSize", &self.ContainerSize)
            .field("FileNameActualLength", &self.FileNameActualLength)
            .field("FileNameLength", &self.FileNameLength)
            .field("FileName", &self.FileName)
            .field("State", &self.State)
            .field("PhysicalContainerId", &self.PhysicalContainerId)
            .field("LogicalContainerId", &self.LogicalContainerId)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLS_CONTAINER_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.FileAttributes == other.FileAttributes
            && self.CreationTime == other.CreationTime
            && self.LastAccessTime == other.LastAccessTime
            && self.LastWriteTime == other.LastWriteTime
            && self.ContainerSize == other.ContainerSize
            && self.FileNameActualLength == other.FileNameActualLength
            && self.FileNameLength == other.FileNameLength
            && self.FileName == other.FileName
            && self.State == other.State
            && self.PhysicalContainerId == other.PhysicalContainerId
            && self.LogicalContainerId == other.LogicalContainerId
    }
}
impl ::core::cmp::Eq for CLS_CONTAINER_INFORMATION {}
impl FromIntoMemory for CLS_CONTAINER_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 320);
        let f_FileAttributes = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_CreationTime = <u64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_LastAccessTime = <u64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_LastWriteTime = <u64 as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        let f_ContainerSize = <i64 as FromIntoMemory>::from_bytes(&from[32..32 + 8]);
        let f_FileNameActualLength = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_FileNameLength = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_FileName = <[u16; 256] as FromIntoMemory>::from_bytes(&from[48..48 + 256]);
        let f_State = <u32 as FromIntoMemory>::from_bytes(&from[304..304 + 4]);
        let f_PhysicalContainerId = <u32 as FromIntoMemory>::from_bytes(&from[308..308 + 4]);
        let f_LogicalContainerId = <u32 as FromIntoMemory>::from_bytes(&from[312..312 + 4]);
        Self {
            FileAttributes: f_FileAttributes,
            CreationTime: f_CreationTime,
            LastAccessTime: f_LastAccessTime,
            LastWriteTime: f_LastWriteTime,
            ContainerSize: f_ContainerSize,
            FileNameActualLength: f_FileNameActualLength,
            FileNameLength: f_FileNameLength,
            FileName: f_FileName,
            State: f_State,
            PhysicalContainerId: f_PhysicalContainerId,
            LogicalContainerId: f_LogicalContainerId,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 320);
        FromIntoMemory::into_bytes(self.FileAttributes, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.CreationTime, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.LastAccessTime, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.LastWriteTime, &mut into[24..24 + 8]);
        FromIntoMemory::into_bytes(self.ContainerSize, &mut into[32..32 + 8]);
        FromIntoMemory::into_bytes(self.FileNameActualLength, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.FileNameLength, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.FileName, &mut into[48..48 + 256]);
        FromIntoMemory::into_bytes(self.State, &mut into[304..304 + 4]);
        FromIntoMemory::into_bytes(self.PhysicalContainerId, &mut into[308..308 + 4]);
        FromIntoMemory::into_bytes(self.LogicalContainerId, &mut into[312..312 + 4]);
    }
    fn size() -> usize {
        320
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CLS_CONTEXT_MODE(pub i32);
pub const ClsContextNone: CLS_CONTEXT_MODE = CLS_CONTEXT_MODE(0i32);
pub const ClsContextUndoNext: CLS_CONTEXT_MODE = CLS_CONTEXT_MODE(1i32);
pub const ClsContextPrevious: CLS_CONTEXT_MODE = CLS_CONTEXT_MODE(2i32);
pub const ClsContextForward: CLS_CONTEXT_MODE = CLS_CONTEXT_MODE(3i32);
impl ::core::marker::Copy for CLS_CONTEXT_MODE {}
impl ::core::clone::Clone for CLS_CONTEXT_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLS_CONTEXT_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLS_CONTEXT_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLS_CONTEXT_MODE").field(&self.0).finish()
    }
}
impl FromIntoMemory for CLS_CONTEXT_MODE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct CLS_INFORMATION {
    pub TotalAvailable: i64,
    pub CurrentAvailable: i64,
    pub TotalReservation: i64,
    pub BaseFileSize: u64,
    pub ContainerSize: u64,
    pub TotalContainers: u32,
    pub FreeContainers: u32,
    pub TotalClients: u32,
    pub Attributes: u32,
    pub FlushThreshold: u32,
    pub SectorSize: u32,
    pub MinArchiveTailLsn: CLS_LSN,
    pub BaseLsn: CLS_LSN,
    pub LastFlushedLsn: CLS_LSN,
    pub LastLsn: CLS_LSN,
    pub RestartLsn: CLS_LSN,
    pub Identity: crate::core::GUID,
}
impl ::core::marker::Copy for CLS_INFORMATION {}
impl ::core::clone::Clone for CLS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLS_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLS_INFORMATION")
            .field("TotalAvailable", &self.TotalAvailable)
            .field("CurrentAvailable", &self.CurrentAvailable)
            .field("TotalReservation", &self.TotalReservation)
            .field("BaseFileSize", &self.BaseFileSize)
            .field("ContainerSize", &self.ContainerSize)
            .field("TotalContainers", &self.TotalContainers)
            .field("FreeContainers", &self.FreeContainers)
            .field("TotalClients", &self.TotalClients)
            .field("Attributes", &self.Attributes)
            .field("FlushThreshold", &self.FlushThreshold)
            .field("SectorSize", &self.SectorSize)
            .field("MinArchiveTailLsn", &self.MinArchiveTailLsn)
            .field("BaseLsn", &self.BaseLsn)
            .field("LastFlushedLsn", &self.LastFlushedLsn)
            .field("LastLsn", &self.LastLsn)
            .field("RestartLsn", &self.RestartLsn)
            .field("Identity", &self.Identity)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLS_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.TotalAvailable == other.TotalAvailable
            && self.CurrentAvailable == other.CurrentAvailable
            && self.TotalReservation == other.TotalReservation
            && self.BaseFileSize == other.BaseFileSize
            && self.ContainerSize == other.ContainerSize
            && self.TotalContainers == other.TotalContainers
            && self.FreeContainers == other.FreeContainers
            && self.TotalClients == other.TotalClients
            && self.Attributes == other.Attributes
            && self.FlushThreshold == other.FlushThreshold
            && self.SectorSize == other.SectorSize
            && self.MinArchiveTailLsn == other.MinArchiveTailLsn
            && self.BaseLsn == other.BaseLsn
            && self.LastFlushedLsn == other.LastFlushedLsn
            && self.LastLsn == other.LastLsn
            && self.RestartLsn == other.RestartLsn
            && self.Identity == other.Identity
    }
}
impl ::core::cmp::Eq for CLS_INFORMATION {}
impl FromIntoMemory for CLS_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 120);
        let f_TotalAvailable = <i64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_CurrentAvailable = <i64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_TotalReservation = <i64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_BaseFileSize = <u64 as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        let f_ContainerSize = <u64 as FromIntoMemory>::from_bytes(&from[32..32 + 8]);
        let f_TotalContainers = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_FreeContainers = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_TotalClients = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_Attributes = <u32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_FlushThreshold = <u32 as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_SectorSize = <u32 as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_MinArchiveTailLsn = <CLS_LSN as FromIntoMemory>::from_bytes(&from[64..64 + 8]);
        let f_BaseLsn = <CLS_LSN as FromIntoMemory>::from_bytes(&from[72..72 + 8]);
        let f_LastFlushedLsn = <CLS_LSN as FromIntoMemory>::from_bytes(&from[80..80 + 8]);
        let f_LastLsn = <CLS_LSN as FromIntoMemory>::from_bytes(&from[88..88 + 8]);
        let f_RestartLsn = <CLS_LSN as FromIntoMemory>::from_bytes(&from[96..96 + 8]);
        let f_Identity = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[104..104 + 16]);
        Self {
            TotalAvailable: f_TotalAvailable,
            CurrentAvailable: f_CurrentAvailable,
            TotalReservation: f_TotalReservation,
            BaseFileSize: f_BaseFileSize,
            ContainerSize: f_ContainerSize,
            TotalContainers: f_TotalContainers,
            FreeContainers: f_FreeContainers,
            TotalClients: f_TotalClients,
            Attributes: f_Attributes,
            FlushThreshold: f_FlushThreshold,
            SectorSize: f_SectorSize,
            MinArchiveTailLsn: f_MinArchiveTailLsn,
            BaseLsn: f_BaseLsn,
            LastFlushedLsn: f_LastFlushedLsn,
            LastLsn: f_LastLsn,
            RestartLsn: f_RestartLsn,
            Identity: f_Identity,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 120);
        FromIntoMemory::into_bytes(self.TotalAvailable, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.CurrentAvailable, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.TotalReservation, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.BaseFileSize, &mut into[24..24 + 8]);
        FromIntoMemory::into_bytes(self.ContainerSize, &mut into[32..32 + 8]);
        FromIntoMemory::into_bytes(self.TotalContainers, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.FreeContainers, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.TotalClients, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.Attributes, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.FlushThreshold, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.SectorSize, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.MinArchiveTailLsn, &mut into[64..64 + 8]);
        FromIntoMemory::into_bytes(self.BaseLsn, &mut into[72..72 + 8]);
        FromIntoMemory::into_bytes(self.LastFlushedLsn, &mut into[80..80 + 8]);
        FromIntoMemory::into_bytes(self.LastLsn, &mut into[88..88 + 8]);
        FromIntoMemory::into_bytes(self.RestartLsn, &mut into[96..96 + 8]);
        FromIntoMemory::into_bytes(self.Identity, &mut into[104..104 + 16]);
    }
    fn size() -> usize {
        120
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CLS_IOSTATS_CLASS(pub i32);
pub const ClsIoStatsDefault: CLS_IOSTATS_CLASS = CLS_IOSTATS_CLASS(0i32);
pub const ClsIoStatsMax: CLS_IOSTATS_CLASS = CLS_IOSTATS_CLASS(65535i32);
impl ::core::marker::Copy for CLS_IOSTATS_CLASS {}
impl ::core::clone::Clone for CLS_IOSTATS_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLS_IOSTATS_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLS_IOSTATS_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLS_IOSTATS_CLASS").field(&self.0).finish()
    }
}
impl FromIntoMemory for CLS_IOSTATS_CLASS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct CLS_IO_STATISTICS {
    pub hdrIoStats: CLS_IO_STATISTICS_HEADER,
    pub cFlush: u64,
    pub cbFlush: u64,
    pub cMetaFlush: u64,
    pub cbMetaFlush: u64,
}
impl ::core::marker::Copy for CLS_IO_STATISTICS {}
impl ::core::clone::Clone for CLS_IO_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLS_IO_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLS_IO_STATISTICS")
            .field("hdrIoStats", &self.hdrIoStats)
            .field("cFlush", &self.cFlush)
            .field("cbFlush", &self.cbFlush)
            .field("cMetaFlush", &self.cMetaFlush)
            .field("cbMetaFlush", &self.cbMetaFlush)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLS_IO_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.hdrIoStats == other.hdrIoStats
            && self.cFlush == other.cFlush
            && self.cbFlush == other.cbFlush
            && self.cMetaFlush == other.cMetaFlush
            && self.cbMetaFlush == other.cbMetaFlush
    }
}
impl ::core::cmp::Eq for CLS_IO_STATISTICS {}
impl FromIntoMemory for CLS_IO_STATISTICS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_hdrIoStats =
            <CLS_IO_STATISTICS_HEADER as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_cFlush = <u64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_cbFlush = <u64 as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        let f_cMetaFlush = <u64 as FromIntoMemory>::from_bytes(&from[32..32 + 8]);
        let f_cbMetaFlush = <u64 as FromIntoMemory>::from_bytes(&from[40..40 + 8]);
        Self {
            hdrIoStats: f_hdrIoStats,
            cFlush: f_cFlush,
            cbFlush: f_cbFlush,
            cMetaFlush: f_cMetaFlush,
            cbMetaFlush: f_cbMetaFlush,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.hdrIoStats, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.cFlush, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.cbFlush, &mut into[24..24 + 8]);
        FromIntoMemory::into_bytes(self.cMetaFlush, &mut into[32..32 + 8]);
        FromIntoMemory::into_bytes(self.cbMetaFlush, &mut into[40..40 + 8]);
    }
    fn size() -> usize {
        48
    }
}
pub struct CLS_IO_STATISTICS_HEADER {
    pub ubMajorVersion: u8,
    pub ubMinorVersion: u8,
    pub eStatsClass: CLFS_IOSTATS_CLASS,
    pub cbLength: u16,
    pub coffData: u32,
}
impl ::core::marker::Copy for CLS_IO_STATISTICS_HEADER {}
impl ::core::clone::Clone for CLS_IO_STATISTICS_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLS_IO_STATISTICS_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLS_IO_STATISTICS_HEADER")
            .field("ubMajorVersion", &self.ubMajorVersion)
            .field("ubMinorVersion", &self.ubMinorVersion)
            .field("eStatsClass", &self.eStatsClass)
            .field("cbLength", &self.cbLength)
            .field("coffData", &self.coffData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLS_IO_STATISTICS_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.ubMajorVersion == other.ubMajorVersion
            && self.ubMinorVersion == other.ubMinorVersion
            && self.eStatsClass == other.eStatsClass
            && self.cbLength == other.cbLength
            && self.coffData == other.coffData
    }
}
impl ::core::cmp::Eq for CLS_IO_STATISTICS_HEADER {}
impl FromIntoMemory for CLS_IO_STATISTICS_HEADER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_ubMajorVersion = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_ubMinorVersion = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_eStatsClass = <CLFS_IOSTATS_CLASS as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_cbLength = <u16 as FromIntoMemory>::from_bytes(&from[8..8 + 2]);
        let f_coffData = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            ubMajorVersion: f_ubMajorVersion,
            ubMinorVersion: f_ubMinorVersion,
            eStatsClass: f_eStatsClass,
            cbLength: f_cbLength,
            coffData: f_coffData,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.ubMajorVersion, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.ubMinorVersion, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.eStatsClass, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.cbLength, &mut into[8..8 + 2]);
        FromIntoMemory::into_bytes(self.coffData, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CLS_LOG_INFORMATION_CLASS(pub i32);
pub const ClfsLogBasicInformation: CLS_LOG_INFORMATION_CLASS = CLS_LOG_INFORMATION_CLASS(0i32);
pub const ClfsLogBasicInformationPhysical: CLS_LOG_INFORMATION_CLASS =
    CLS_LOG_INFORMATION_CLASS(1i32);
pub const ClfsLogPhysicalNameInformation: CLS_LOG_INFORMATION_CLASS =
    CLS_LOG_INFORMATION_CLASS(2i32);
pub const ClfsLogStreamIdentifierInformation: CLS_LOG_INFORMATION_CLASS =
    CLS_LOG_INFORMATION_CLASS(3i32);
pub const ClfsLogSystemMarkingInformation: CLS_LOG_INFORMATION_CLASS =
    CLS_LOG_INFORMATION_CLASS(4i32);
pub const ClfsLogPhysicalLsnInformation: CLS_LOG_INFORMATION_CLASS =
    CLS_LOG_INFORMATION_CLASS(5i32);
impl ::core::marker::Copy for CLS_LOG_INFORMATION_CLASS {}
impl ::core::clone::Clone for CLS_LOG_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLS_LOG_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLS_LOG_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLS_LOG_INFORMATION_CLASS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for CLS_LOG_INFORMATION_CLASS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct CLS_LSN {
    pub Internal: u64,
}
impl ::core::marker::Copy for CLS_LSN {}
impl ::core::clone::Clone for CLS_LSN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLS_LSN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLS_LSN")
            .field("Internal", &self.Internal)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLS_LSN {
    fn eq(&self, other: &Self) -> bool {
        self.Internal == other.Internal
    }
}
impl ::core::cmp::Eq for CLS_LSN {}
impl FromIntoMemory for CLS_LSN {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Internal = <u64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        Self {
            Internal: f_Internal,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Internal, &mut into[0..0 + 8]);
    }
    fn size() -> usize {
        8
    }
}
pub struct CLS_SCAN_CONTEXT {
    pub cidNode: CLFS_NODE_ID,
    pub hLog: super::super::Foundation::HANDLE,
    pub cIndex: u32,
    pub cContainers: u32,
    pub cContainersReturned: u32,
    pub eScanMode: u8,
    pub pinfoContainer: MutPtr<CLS_CONTAINER_INFORMATION>,
}
impl ::core::marker::Copy for CLS_SCAN_CONTEXT {}
impl ::core::clone::Clone for CLS_SCAN_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLS_SCAN_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLS_SCAN_CONTEXT")
            .field("cidNode", &self.cidNode)
            .field("hLog", &self.hLog)
            .field("cIndex", &self.cIndex)
            .field("cContainers", &self.cContainers)
            .field("cContainersReturned", &self.cContainersReturned)
            .field("eScanMode", &self.eScanMode)
            .field("pinfoContainer", &self.pinfoContainer)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLS_SCAN_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.cidNode == other.cidNode
            && self.hLog == other.hLog
            && self.cIndex == other.cIndex
            && self.cContainers == other.cContainers
            && self.cContainersReturned == other.cContainersReturned
            && self.eScanMode == other.eScanMode
            && self.pinfoContainer == other.pinfoContainer
    }
}
impl ::core::cmp::Eq for CLS_SCAN_CONTEXT {}
impl FromIntoMemory for CLS_SCAN_CONTEXT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_cidNode = <CLFS_NODE_ID as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_hLog =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_cIndex = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_cContainers = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_cContainersReturned = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_eScanMode = <u8 as FromIntoMemory>::from_bytes(&from[24..24 + 1]);
        let f_pinfoContainer =
            <MutPtr<CLS_CONTAINER_INFORMATION> as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            cidNode: f_cidNode,
            hLog: f_hLog,
            cIndex: f_cIndex,
            cContainers: f_cContainers,
            cContainersReturned: f_cContainersReturned,
            eScanMode: f_eScanMode,
            pinfoContainer: f_pinfoContainer,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.cidNode, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.hLog, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.cIndex, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.cContainers, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.cContainersReturned, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.eScanMode, &mut into[24..24 + 1]);
        FromIntoMemory::into_bytes(self.pinfoContainer, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
pub struct CLS_WRITE_ENTRY {
    pub Buffer: MutPtr<::core::ffi::c_void>,
    pub ByteLength: u32,
}
impl ::core::marker::Copy for CLS_WRITE_ENTRY {}
impl ::core::clone::Clone for CLS_WRITE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLS_WRITE_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLS_WRITE_ENTRY")
            .field("Buffer", &self.Buffer)
            .field("ByteLength", &self.ByteLength)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLS_WRITE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Buffer == other.Buffer && self.ByteLength == other.ByteLength
    }
}
impl ::core::cmp::Eq for CLS_WRITE_ENTRY {}
impl FromIntoMemory for CLS_WRITE_ENTRY {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Buffer = <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ByteLength = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            Buffer: f_Buffer,
            ByteLength: f_ByteLength,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Buffer, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ByteLength, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct CONNECTION_INFO_0 {
    pub coni0_id: u32,
}
impl ::core::marker::Copy for CONNECTION_INFO_0 {}
impl ::core::clone::Clone for CONNECTION_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONNECTION_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONNECTION_INFO_0")
            .field("coni0_id", &self.coni0_id)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CONNECTION_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.coni0_id == other.coni0_id
    }
}
impl ::core::cmp::Eq for CONNECTION_INFO_0 {}
impl FromIntoMemory for CONNECTION_INFO_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_coni0_id = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            coni0_id: f_coni0_id,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.coni0_id, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct CONNECTION_INFO_1 {
    pub coni1_id: u32,
    pub coni1_type: SHARE_TYPE,
    pub coni1_num_opens: u32,
    pub coni1_num_users: u32,
    pub coni1_time: u32,
    pub coni1_username: PWSTR,
    pub coni1_netname: PWSTR,
}
impl ::core::marker::Copy for CONNECTION_INFO_1 {}
impl ::core::clone::Clone for CONNECTION_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONNECTION_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONNECTION_INFO_1")
            .field("coni1_id", &self.coni1_id)
            .field("coni1_type", &self.coni1_type)
            .field("coni1_num_opens", &self.coni1_num_opens)
            .field("coni1_num_users", &self.coni1_num_users)
            .field("coni1_time", &self.coni1_time)
            .field("coni1_username", &self.coni1_username)
            .field("coni1_netname", &self.coni1_netname)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CONNECTION_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.coni1_id == other.coni1_id
            && self.coni1_type == other.coni1_type
            && self.coni1_num_opens == other.coni1_num_opens
            && self.coni1_num_users == other.coni1_num_users
            && self.coni1_time == other.coni1_time
            && self.coni1_username == other.coni1_username
            && self.coni1_netname == other.coni1_netname
    }
}
impl ::core::cmp::Eq for CONNECTION_INFO_1 {}
impl FromIntoMemory for CONNECTION_INFO_1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_coni1_id = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_coni1_type = <SHARE_TYPE as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_coni1_num_opens = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_coni1_num_users = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_coni1_time = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_coni1_username = <PWSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_coni1_netname = <PWSTR as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        Self {
            coni1_id: f_coni1_id,
            coni1_type: f_coni1_type,
            coni1_num_opens: f_coni1_num_opens,
            coni1_num_users: f_coni1_num_users,
            coni1_time: f_coni1_time,
            coni1_username: f_coni1_username,
            coni1_netname: f_coni1_netname,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.coni1_id, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.coni1_type, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.coni1_num_opens, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.coni1_num_users, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.coni1_time, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.coni1_username, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.coni1_netname, &mut into[24..24 + 4]);
    }
    fn size() -> usize {
        28
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COPYFILE2_COPY_PHASE(pub i32);
pub const COPYFILE2_PHASE_NONE: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(0i32);
pub const COPYFILE2_PHASE_PREPARE_SOURCE: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(1i32);
pub const COPYFILE2_PHASE_PREPARE_DEST: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(2i32);
pub const COPYFILE2_PHASE_READ_SOURCE: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(3i32);
pub const COPYFILE2_PHASE_WRITE_DESTINATION: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(4i32);
pub const COPYFILE2_PHASE_SERVER_COPY: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(5i32);
pub const COPYFILE2_PHASE_NAMEGRAFT_COPY: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(6i32);
pub const COPYFILE2_PHASE_MAX: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(7i32);
impl ::core::marker::Copy for COPYFILE2_COPY_PHASE {}
impl ::core::clone::Clone for COPYFILE2_COPY_PHASE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COPYFILE2_COPY_PHASE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COPYFILE2_COPY_PHASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COPYFILE2_COPY_PHASE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for COPYFILE2_COPY_PHASE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct COPYFILE2_EXTENDED_PARAMETERS {
    pub dwSize: u32,
    pub dwCopyFlags: u32,
    pub pfCancel: MutPtr<super::super::Foundation::BOOL>,
    pub pProgressRoutine: PCOPYFILE2_PROGRESS_ROUTINE,
    pub pvCallbackContext: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for COPYFILE2_EXTENDED_PARAMETERS {}
impl ::core::clone::Clone for COPYFILE2_EXTENDED_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COPYFILE2_EXTENDED_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COPYFILE2_EXTENDED_PARAMETERS")
            .field("dwSize", &self.dwSize)
            .field("dwCopyFlags", &self.dwCopyFlags)
            .field("pfCancel", &self.pfCancel)
            .field("pProgressRoutine", &self.pProgressRoutine)
            .field("pvCallbackContext", &self.pvCallbackContext)
            .finish()
    }
}
impl ::core::cmp::PartialEq for COPYFILE2_EXTENDED_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwCopyFlags == other.dwCopyFlags
            && self.pfCancel == other.pfCancel
            && self.pProgressRoutine == other.pProgressRoutine
            && self.pvCallbackContext == other.pvCallbackContext
    }
}
impl ::core::cmp::Eq for COPYFILE2_EXTENDED_PARAMETERS {}
impl FromIntoMemory for COPYFILE2_EXTENDED_PARAMETERS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwCopyFlags = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_pfCancel =
            <MutPtr<super::super::Foundation::BOOL> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_pProgressRoutine =
            <PCOPYFILE2_PROGRESS_ROUTINE as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_pvCallbackContext =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            dwSize: f_dwSize,
            dwCopyFlags: f_dwCopyFlags,
            pfCancel: f_pfCancel,
            pProgressRoutine: f_pProgressRoutine,
            pvCallbackContext: f_pvCallbackContext,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwCopyFlags, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.pfCancel, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.pProgressRoutine, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.pvCallbackContext, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub struct COPYFILE2_EXTENDED_PARAMETERS_V2 {
    pub dwSize: u32,
    pub dwCopyFlags: u32,
    pub pfCancel: MutPtr<super::super::Foundation::BOOL>,
    pub pProgressRoutine: PCOPYFILE2_PROGRESS_ROUTINE,
    pub pvCallbackContext: MutPtr<::core::ffi::c_void>,
    pub dwCopyFlagsV2: u32,
    pub ioDesiredSize: u32,
    pub ioDesiredRate: u32,
    pub reserved: [MutPtr<::core::ffi::c_void>; 8],
}
impl ::core::marker::Copy for COPYFILE2_EXTENDED_PARAMETERS_V2 {}
impl ::core::clone::Clone for COPYFILE2_EXTENDED_PARAMETERS_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COPYFILE2_EXTENDED_PARAMETERS_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COPYFILE2_EXTENDED_PARAMETERS_V2")
            .field("dwSize", &self.dwSize)
            .field("dwCopyFlags", &self.dwCopyFlags)
            .field("pfCancel", &self.pfCancel)
            .field("pProgressRoutine", &self.pProgressRoutine)
            .field("pvCallbackContext", &self.pvCallbackContext)
            .field("dwCopyFlagsV2", &self.dwCopyFlagsV2)
            .field("ioDesiredSize", &self.ioDesiredSize)
            .field("ioDesiredRate", &self.ioDesiredRate)
            .field("reserved", &self.reserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for COPYFILE2_EXTENDED_PARAMETERS_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwCopyFlags == other.dwCopyFlags
            && self.pfCancel == other.pfCancel
            && self.pProgressRoutine == other.pProgressRoutine
            && self.pvCallbackContext == other.pvCallbackContext
            && self.dwCopyFlagsV2 == other.dwCopyFlagsV2
            && self.ioDesiredSize == other.ioDesiredSize
            && self.ioDesiredRate == other.ioDesiredRate
            && self.reserved == other.reserved
    }
}
impl ::core::cmp::Eq for COPYFILE2_EXTENDED_PARAMETERS_V2 {}
impl FromIntoMemory for COPYFILE2_EXTENDED_PARAMETERS_V2 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 64);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwCopyFlags = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_pfCancel =
            <MutPtr<super::super::Foundation::BOOL> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_pProgressRoutine =
            <PCOPYFILE2_PROGRESS_ROUTINE as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_pvCallbackContext =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwCopyFlagsV2 = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_ioDesiredSize = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_ioDesiredRate = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_reserved =
            <[MutPtr<::core::ffi::c_void>; 8] as FromIntoMemory>::from_bytes(&from[32..32 + 32]);
        Self {
            dwSize: f_dwSize,
            dwCopyFlags: f_dwCopyFlags,
            pfCancel: f_pfCancel,
            pProgressRoutine: f_pProgressRoutine,
            pvCallbackContext: f_pvCallbackContext,
            dwCopyFlagsV2: f_dwCopyFlagsV2,
            ioDesiredSize: f_ioDesiredSize,
            ioDesiredRate: f_ioDesiredRate,
            reserved: f_reserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 64);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwCopyFlags, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.pfCancel, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.pProgressRoutine, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.pvCallbackContext, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwCopyFlagsV2, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.ioDesiredSize, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.ioDesiredRate, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.reserved, &mut into[32..32 + 32]);
    }
    fn size() -> usize {
        64
    }
}
pub struct COPYFILE2_MESSAGE {
    pub Type: COPYFILE2_MESSAGE_TYPE,
    pub dwPadding: u32,
    pub Info: COPYFILE2_MESSAGE_0,
}
impl ::core::marker::Copy for COPYFILE2_MESSAGE {}
impl ::core::clone::Clone for COPYFILE2_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for COPYFILE2_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.dwPadding == other.dwPadding && self.Info == other.Info
    }
}
impl ::core::cmp::Eq for COPYFILE2_MESSAGE {}
impl FromIntoMemory for COPYFILE2_MESSAGE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 64);
        let f_Type = <COPYFILE2_MESSAGE_TYPE as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwPadding = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Info = <COPYFILE2_MESSAGE_0 as FromIntoMemory>::from_bytes(&from[8..8 + 56]);
        Self {
            Type: f_Type,
            dwPadding: f_dwPadding,
            Info: f_Info,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 64);
        FromIntoMemory::into_bytes(self.Type, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwPadding, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Info, &mut into[8..8 + 56]);
    }
    fn size() -> usize {
        64
    }
}
pub struct COPYFILE2_MESSAGE_0 {
    data: [u8; 56],
}
impl ::core::default::Default for COPYFILE2_MESSAGE_0 {
    fn default() -> Self {
        Self { data: [0u8; 56] }
    }
}
impl ::core::marker::Copy for COPYFILE2_MESSAGE_0 {}
impl ::core::clone::Clone for COPYFILE2_MESSAGE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for COPYFILE2_MESSAGE_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for COPYFILE2_MESSAGE_0 {}
impl FromIntoMemory for COPYFILE2_MESSAGE_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 56];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        56
    }
}
pub struct COPYFILE2_MESSAGE_0_0 {
    pub dwStreamNumber: u32,
    pub dwFlags: u32,
    pub hSourceFile: super::super::Foundation::HANDLE,
    pub hDestinationFile: super::super::Foundation::HANDLE,
    pub uliChunkNumber: u64,
    pub uliChunkSize: u64,
    pub uliStreamSize: u64,
    pub uliStreamBytesTransferred: u64,
    pub uliTotalFileSize: u64,
    pub uliTotalBytesTransferred: u64,
}
impl ::core::marker::Copy for COPYFILE2_MESSAGE_0_0 {}
impl ::core::clone::Clone for COPYFILE2_MESSAGE_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COPYFILE2_MESSAGE_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COPYFILE2_MESSAGE_0_0")
            .field("dwStreamNumber", &self.dwStreamNumber)
            .field("dwFlags", &self.dwFlags)
            .field("hSourceFile", &self.hSourceFile)
            .field("hDestinationFile", &self.hDestinationFile)
            .field("uliChunkNumber", &self.uliChunkNumber)
            .field("uliChunkSize", &self.uliChunkSize)
            .field("uliStreamSize", &self.uliStreamSize)
            .field("uliStreamBytesTransferred", &self.uliStreamBytesTransferred)
            .field("uliTotalFileSize", &self.uliTotalFileSize)
            .field("uliTotalBytesTransferred", &self.uliTotalBytesTransferred)
            .finish()
    }
}
impl ::core::cmp::PartialEq for COPYFILE2_MESSAGE_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwStreamNumber == other.dwStreamNumber
            && self.dwFlags == other.dwFlags
            && self.hSourceFile == other.hSourceFile
            && self.hDestinationFile == other.hDestinationFile
            && self.uliChunkNumber == other.uliChunkNumber
            && self.uliChunkSize == other.uliChunkSize
            && self.uliStreamSize == other.uliStreamSize
            && self.uliStreamBytesTransferred == other.uliStreamBytesTransferred
            && self.uliTotalFileSize == other.uliTotalFileSize
            && self.uliTotalBytesTransferred == other.uliTotalBytesTransferred
    }
}
impl ::core::cmp::Eq for COPYFILE2_MESSAGE_0_0 {}
impl FromIntoMemory for COPYFILE2_MESSAGE_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 64);
        let f_dwStreamNumber = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_hSourceFile =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_hDestinationFile =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_uliChunkNumber = <u64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_uliChunkSize = <u64 as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        let f_uliStreamSize = <u64 as FromIntoMemory>::from_bytes(&from[32..32 + 8]);
        let f_uliStreamBytesTransferred = <u64 as FromIntoMemory>::from_bytes(&from[40..40 + 8]);
        let f_uliTotalFileSize = <u64 as FromIntoMemory>::from_bytes(&from[48..48 + 8]);
        let f_uliTotalBytesTransferred = <u64 as FromIntoMemory>::from_bytes(&from[56..56 + 8]);
        Self {
            dwStreamNumber: f_dwStreamNumber,
            dwFlags: f_dwFlags,
            hSourceFile: f_hSourceFile,
            hDestinationFile: f_hDestinationFile,
            uliChunkNumber: f_uliChunkNumber,
            uliChunkSize: f_uliChunkSize,
            uliStreamSize: f_uliStreamSize,
            uliStreamBytesTransferred: f_uliStreamBytesTransferred,
            uliTotalFileSize: f_uliTotalFileSize,
            uliTotalBytesTransferred: f_uliTotalBytesTransferred,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 64);
        FromIntoMemory::into_bytes(self.dwStreamNumber, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.hSourceFile, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.hDestinationFile, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.uliChunkNumber, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.uliChunkSize, &mut into[24..24 + 8]);
        FromIntoMemory::into_bytes(self.uliStreamSize, &mut into[32..32 + 8]);
        FromIntoMemory::into_bytes(self.uliStreamBytesTransferred, &mut into[40..40 + 8]);
        FromIntoMemory::into_bytes(self.uliTotalFileSize, &mut into[48..48 + 8]);
        FromIntoMemory::into_bytes(self.uliTotalBytesTransferred, &mut into[56..56 + 8]);
    }
    fn size() -> usize {
        64
    }
}
pub struct COPYFILE2_MESSAGE_0_1 {
    pub dwStreamNumber: u32,
    pub dwReserved: u32,
    pub hSourceFile: super::super::Foundation::HANDLE,
    pub hDestinationFile: super::super::Foundation::HANDLE,
    pub uliChunkNumber: u64,
    pub uliChunkSize: u64,
    pub uliStreamSize: u64,
    pub uliTotalFileSize: u64,
}
impl ::core::marker::Copy for COPYFILE2_MESSAGE_0_1 {}
impl ::core::clone::Clone for COPYFILE2_MESSAGE_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COPYFILE2_MESSAGE_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COPYFILE2_MESSAGE_0_1")
            .field("dwStreamNumber", &self.dwStreamNumber)
            .field("dwReserved", &self.dwReserved)
            .field("hSourceFile", &self.hSourceFile)
            .field("hDestinationFile", &self.hDestinationFile)
            .field("uliChunkNumber", &self.uliChunkNumber)
            .field("uliChunkSize", &self.uliChunkSize)
            .field("uliStreamSize", &self.uliStreamSize)
            .field("uliTotalFileSize", &self.uliTotalFileSize)
            .finish()
    }
}
impl ::core::cmp::PartialEq for COPYFILE2_MESSAGE_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwStreamNumber == other.dwStreamNumber
            && self.dwReserved == other.dwReserved
            && self.hSourceFile == other.hSourceFile
            && self.hDestinationFile == other.hDestinationFile
            && self.uliChunkNumber == other.uliChunkNumber
            && self.uliChunkSize == other.uliChunkSize
            && self.uliStreamSize == other.uliStreamSize
            && self.uliTotalFileSize == other.uliTotalFileSize
    }
}
impl ::core::cmp::Eq for COPYFILE2_MESSAGE_0_1 {}
impl FromIntoMemory for COPYFILE2_MESSAGE_0_1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_dwStreamNumber = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwReserved = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_hSourceFile =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_hDestinationFile =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_uliChunkNumber = <u64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_uliChunkSize = <u64 as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        let f_uliStreamSize = <u64 as FromIntoMemory>::from_bytes(&from[32..32 + 8]);
        let f_uliTotalFileSize = <u64 as FromIntoMemory>::from_bytes(&from[40..40 + 8]);
        Self {
            dwStreamNumber: f_dwStreamNumber,
            dwReserved: f_dwReserved,
            hSourceFile: f_hSourceFile,
            hDestinationFile: f_hDestinationFile,
            uliChunkNumber: f_uliChunkNumber,
            uliChunkSize: f_uliChunkSize,
            uliStreamSize: f_uliStreamSize,
            uliTotalFileSize: f_uliTotalFileSize,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.dwStreamNumber, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwReserved, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.hSourceFile, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.hDestinationFile, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.uliChunkNumber, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.uliChunkSize, &mut into[24..24 + 8]);
        FromIntoMemory::into_bytes(self.uliStreamSize, &mut into[32..32 + 8]);
        FromIntoMemory::into_bytes(self.uliTotalFileSize, &mut into[40..40 + 8]);
    }
    fn size() -> usize {
        48
    }
}
pub struct COPYFILE2_MESSAGE_0_2 {
    pub CopyPhase: COPYFILE2_COPY_PHASE,
    pub dwStreamNumber: u32,
    pub hrFailure: crate::core::HRESULT,
    pub dwReserved: u32,
    pub uliChunkNumber: u64,
    pub uliStreamSize: u64,
    pub uliStreamBytesTransferred: u64,
    pub uliTotalFileSize: u64,
    pub uliTotalBytesTransferred: u64,
}
impl ::core::marker::Copy for COPYFILE2_MESSAGE_0_2 {}
impl ::core::clone::Clone for COPYFILE2_MESSAGE_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COPYFILE2_MESSAGE_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COPYFILE2_MESSAGE_0_2")
            .field("CopyPhase", &self.CopyPhase)
            .field("dwStreamNumber", &self.dwStreamNumber)
            .field("hrFailure", &self.hrFailure)
            .field("dwReserved", &self.dwReserved)
            .field("uliChunkNumber", &self.uliChunkNumber)
            .field("uliStreamSize", &self.uliStreamSize)
            .field("uliStreamBytesTransferred", &self.uliStreamBytesTransferred)
            .field("uliTotalFileSize", &self.uliTotalFileSize)
            .field("uliTotalBytesTransferred", &self.uliTotalBytesTransferred)
            .finish()
    }
}
impl ::core::cmp::PartialEq for COPYFILE2_MESSAGE_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.CopyPhase == other.CopyPhase
            && self.dwStreamNumber == other.dwStreamNumber
            && self.hrFailure == other.hrFailure
            && self.dwReserved == other.dwReserved
            && self.uliChunkNumber == other.uliChunkNumber
            && self.uliStreamSize == other.uliStreamSize
            && self.uliStreamBytesTransferred == other.uliStreamBytesTransferred
            && self.uliTotalFileSize == other.uliTotalFileSize
            && self.uliTotalBytesTransferred == other.uliTotalBytesTransferred
    }
}
impl ::core::cmp::Eq for COPYFILE2_MESSAGE_0_2 {}
impl FromIntoMemory for COPYFILE2_MESSAGE_0_2 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 56);
        let f_CopyPhase = <COPYFILE2_COPY_PHASE as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwStreamNumber = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_hrFailure = <crate::core::HRESULT as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwReserved = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_uliChunkNumber = <u64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_uliStreamSize = <u64 as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        let f_uliStreamBytesTransferred = <u64 as FromIntoMemory>::from_bytes(&from[32..32 + 8]);
        let f_uliTotalFileSize = <u64 as FromIntoMemory>::from_bytes(&from[40..40 + 8]);
        let f_uliTotalBytesTransferred = <u64 as FromIntoMemory>::from_bytes(&from[48..48 + 8]);
        Self {
            CopyPhase: f_CopyPhase,
            dwStreamNumber: f_dwStreamNumber,
            hrFailure: f_hrFailure,
            dwReserved: f_dwReserved,
            uliChunkNumber: f_uliChunkNumber,
            uliStreamSize: f_uliStreamSize,
            uliStreamBytesTransferred: f_uliStreamBytesTransferred,
            uliTotalFileSize: f_uliTotalFileSize,
            uliTotalBytesTransferred: f_uliTotalBytesTransferred,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 56);
        FromIntoMemory::into_bytes(self.CopyPhase, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwStreamNumber, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.hrFailure, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwReserved, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.uliChunkNumber, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.uliStreamSize, &mut into[24..24 + 8]);
        FromIntoMemory::into_bytes(self.uliStreamBytesTransferred, &mut into[32..32 + 8]);
        FromIntoMemory::into_bytes(self.uliTotalFileSize, &mut into[40..40 + 8]);
        FromIntoMemory::into_bytes(self.uliTotalBytesTransferred, &mut into[48..48 + 8]);
    }
    fn size() -> usize {
        56
    }
}
pub struct COPYFILE2_MESSAGE_0_3 {
    pub dwReserved: u32,
}
impl ::core::marker::Copy for COPYFILE2_MESSAGE_0_3 {}
impl ::core::clone::Clone for COPYFILE2_MESSAGE_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COPYFILE2_MESSAGE_0_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COPYFILE2_MESSAGE_0_3")
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for COPYFILE2_MESSAGE_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for COPYFILE2_MESSAGE_0_3 {}
impl FromIntoMemory for COPYFILE2_MESSAGE_0_3 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_dwReserved = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            dwReserved: f_dwReserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.dwReserved, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct COPYFILE2_MESSAGE_0_4 {
    pub dwStreamNumber: u32,
    pub dwReserved: u32,
    pub hSourceFile: super::super::Foundation::HANDLE,
    pub hDestinationFile: super::super::Foundation::HANDLE,
    pub uliStreamSize: u64,
    pub uliStreamBytesTransferred: u64,
    pub uliTotalFileSize: u64,
    pub uliTotalBytesTransferred: u64,
}
impl ::core::marker::Copy for COPYFILE2_MESSAGE_0_4 {}
impl ::core::clone::Clone for COPYFILE2_MESSAGE_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COPYFILE2_MESSAGE_0_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COPYFILE2_MESSAGE_0_4")
            .field("dwStreamNumber", &self.dwStreamNumber)
            .field("dwReserved", &self.dwReserved)
            .field("hSourceFile", &self.hSourceFile)
            .field("hDestinationFile", &self.hDestinationFile)
            .field("uliStreamSize", &self.uliStreamSize)
            .field("uliStreamBytesTransferred", &self.uliStreamBytesTransferred)
            .field("uliTotalFileSize", &self.uliTotalFileSize)
            .field("uliTotalBytesTransferred", &self.uliTotalBytesTransferred)
            .finish()
    }
}
impl ::core::cmp::PartialEq for COPYFILE2_MESSAGE_0_4 {
    fn eq(&self, other: &Self) -> bool {
        self.dwStreamNumber == other.dwStreamNumber
            && self.dwReserved == other.dwReserved
            && self.hSourceFile == other.hSourceFile
            && self.hDestinationFile == other.hDestinationFile
            && self.uliStreamSize == other.uliStreamSize
            && self.uliStreamBytesTransferred == other.uliStreamBytesTransferred
            && self.uliTotalFileSize == other.uliTotalFileSize
            && self.uliTotalBytesTransferred == other.uliTotalBytesTransferred
    }
}
impl ::core::cmp::Eq for COPYFILE2_MESSAGE_0_4 {}
impl FromIntoMemory for COPYFILE2_MESSAGE_0_4 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_dwStreamNumber = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwReserved = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_hSourceFile =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_hDestinationFile =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_uliStreamSize = <u64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_uliStreamBytesTransferred = <u64 as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        let f_uliTotalFileSize = <u64 as FromIntoMemory>::from_bytes(&from[32..32 + 8]);
        let f_uliTotalBytesTransferred = <u64 as FromIntoMemory>::from_bytes(&from[40..40 + 8]);
        Self {
            dwStreamNumber: f_dwStreamNumber,
            dwReserved: f_dwReserved,
            hSourceFile: f_hSourceFile,
            hDestinationFile: f_hDestinationFile,
            uliStreamSize: f_uliStreamSize,
            uliStreamBytesTransferred: f_uliStreamBytesTransferred,
            uliTotalFileSize: f_uliTotalFileSize,
            uliTotalBytesTransferred: f_uliTotalBytesTransferred,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.dwStreamNumber, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwReserved, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.hSourceFile, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.hDestinationFile, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.uliStreamSize, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.uliStreamBytesTransferred, &mut into[24..24 + 8]);
        FromIntoMemory::into_bytes(self.uliTotalFileSize, &mut into[32..32 + 8]);
        FromIntoMemory::into_bytes(self.uliTotalBytesTransferred, &mut into[40..40 + 8]);
    }
    fn size() -> usize {
        48
    }
}
pub struct COPYFILE2_MESSAGE_0_5 {
    pub dwStreamNumber: u32,
    pub dwReserved: u32,
    pub hSourceFile: super::super::Foundation::HANDLE,
    pub hDestinationFile: super::super::Foundation::HANDLE,
    pub uliStreamSize: u64,
    pub uliTotalFileSize: u64,
}
impl ::core::marker::Copy for COPYFILE2_MESSAGE_0_5 {}
impl ::core::clone::Clone for COPYFILE2_MESSAGE_0_5 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COPYFILE2_MESSAGE_0_5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COPYFILE2_MESSAGE_0_5")
            .field("dwStreamNumber", &self.dwStreamNumber)
            .field("dwReserved", &self.dwReserved)
            .field("hSourceFile", &self.hSourceFile)
            .field("hDestinationFile", &self.hDestinationFile)
            .field("uliStreamSize", &self.uliStreamSize)
            .field("uliTotalFileSize", &self.uliTotalFileSize)
            .finish()
    }
}
impl ::core::cmp::PartialEq for COPYFILE2_MESSAGE_0_5 {
    fn eq(&self, other: &Self) -> bool {
        self.dwStreamNumber == other.dwStreamNumber
            && self.dwReserved == other.dwReserved
            && self.hSourceFile == other.hSourceFile
            && self.hDestinationFile == other.hDestinationFile
            && self.uliStreamSize == other.uliStreamSize
            && self.uliTotalFileSize == other.uliTotalFileSize
    }
}
impl ::core::cmp::Eq for COPYFILE2_MESSAGE_0_5 {}
impl FromIntoMemory for COPYFILE2_MESSAGE_0_5 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_dwStreamNumber = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwReserved = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_hSourceFile =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_hDestinationFile =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_uliStreamSize = <u64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_uliTotalFileSize = <u64 as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        Self {
            dwStreamNumber: f_dwStreamNumber,
            dwReserved: f_dwReserved,
            hSourceFile: f_hSourceFile,
            hDestinationFile: f_hDestinationFile,
            uliStreamSize: f_uliStreamSize,
            uliTotalFileSize: f_uliTotalFileSize,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.dwStreamNumber, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwReserved, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.hSourceFile, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.hDestinationFile, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.uliStreamSize, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.uliTotalFileSize, &mut into[24..24 + 8]);
    }
    fn size() -> usize {
        32
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COPYFILE2_MESSAGE_ACTION(pub i32);
pub const COPYFILE2_PROGRESS_CONTINUE: COPYFILE2_MESSAGE_ACTION = COPYFILE2_MESSAGE_ACTION(0i32);
pub const COPYFILE2_PROGRESS_CANCEL: COPYFILE2_MESSAGE_ACTION = COPYFILE2_MESSAGE_ACTION(1i32);
pub const COPYFILE2_PROGRESS_STOP: COPYFILE2_MESSAGE_ACTION = COPYFILE2_MESSAGE_ACTION(2i32);
pub const COPYFILE2_PROGRESS_QUIET: COPYFILE2_MESSAGE_ACTION = COPYFILE2_MESSAGE_ACTION(3i32);
pub const COPYFILE2_PROGRESS_PAUSE: COPYFILE2_MESSAGE_ACTION = COPYFILE2_MESSAGE_ACTION(4i32);
impl ::core::marker::Copy for COPYFILE2_MESSAGE_ACTION {}
impl ::core::clone::Clone for COPYFILE2_MESSAGE_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COPYFILE2_MESSAGE_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COPYFILE2_MESSAGE_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COPYFILE2_MESSAGE_ACTION")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for COPYFILE2_MESSAGE_ACTION {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COPYFILE2_MESSAGE_TYPE(pub i32);
pub const COPYFILE2_CALLBACK_NONE: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(0i32);
pub const COPYFILE2_CALLBACK_CHUNK_STARTED: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(1i32);
pub const COPYFILE2_CALLBACK_CHUNK_FINISHED: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(2i32);
pub const COPYFILE2_CALLBACK_STREAM_STARTED: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(3i32);
pub const COPYFILE2_CALLBACK_STREAM_FINISHED: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(4i32);
pub const COPYFILE2_CALLBACK_POLL_CONTINUE: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(5i32);
pub const COPYFILE2_CALLBACK_ERROR: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(6i32);
pub const COPYFILE2_CALLBACK_MAX: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(7i32);
impl ::core::marker::Copy for COPYFILE2_MESSAGE_TYPE {}
impl ::core::clone::Clone for COPYFILE2_MESSAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COPYFILE2_MESSAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COPYFILE2_MESSAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COPYFILE2_MESSAGE_TYPE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for COPYFILE2_MESSAGE_TYPE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct CREATEFILE2_EXTENDED_PARAMETERS {
    pub dwSize: u32,
    pub dwFileAttributes: u32,
    pub dwFileFlags: u32,
    pub dwSecurityQosFlags: u32,
    pub lpSecurityAttributes: MutPtr<super::super::Security::SECURITY_ATTRIBUTES>,
    pub hTemplateFile: super::super::Foundation::HANDLE,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for CREATEFILE2_EXTENDED_PARAMETERS {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for CREATEFILE2_EXTENDED_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for CREATEFILE2_EXTENDED_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREATEFILE2_EXTENDED_PARAMETERS")
            .field("dwSize", &self.dwSize)
            .field("dwFileAttributes", &self.dwFileAttributes)
            .field("dwFileFlags", &self.dwFileFlags)
            .field("dwSecurityQosFlags", &self.dwSecurityQosFlags)
            .field("lpSecurityAttributes", &self.lpSecurityAttributes)
            .field("hTemplateFile", &self.hTemplateFile)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for CREATEFILE2_EXTENDED_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwFileAttributes == other.dwFileAttributes
            && self.dwFileFlags == other.dwFileFlags
            && self.dwSecurityQosFlags == other.dwSecurityQosFlags
            && self.lpSecurityAttributes == other.lpSecurityAttributes
            && self.hTemplateFile == other.hTemplateFile
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for CREATEFILE2_EXTENDED_PARAMETERS {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for CREATEFILE2_EXTENDED_PARAMETERS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFileAttributes = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwFileFlags = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwSecurityQosFlags = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpSecurityAttributes =
            <MutPtr<super::super::Security::SECURITY_ATTRIBUTES> as FromIntoMemory>::from_bytes(
                &from[16..16 + 4],
            );
        let f_hTemplateFile =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            dwSize: f_dwSize,
            dwFileAttributes: f_dwFileAttributes,
            dwFileFlags: f_dwFileFlags,
            dwSecurityQosFlags: f_dwSecurityQosFlags,
            lpSecurityAttributes: f_lpSecurityAttributes,
            hTemplateFile: f_hTemplateFile,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFileAttributes, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwFileFlags, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwSecurityQosFlags, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpSecurityAttributes, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.hTemplateFile, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CREATE_TAPE_PARTITION_METHOD(pub i32);
pub const TAPE_FIXED_PARTITIONS: CREATE_TAPE_PARTITION_METHOD = CREATE_TAPE_PARTITION_METHOD(0i32);
pub const TAPE_INITIATOR_PARTITIONS: CREATE_TAPE_PARTITION_METHOD =
    CREATE_TAPE_PARTITION_METHOD(2i32);
pub const TAPE_SELECT_PARTITIONS: CREATE_TAPE_PARTITION_METHOD = CREATE_TAPE_PARTITION_METHOD(1i32);
impl ::core::marker::Copy for CREATE_TAPE_PARTITION_METHOD {}
impl ::core::clone::Clone for CREATE_TAPE_PARTITION_METHOD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CREATE_TAPE_PARTITION_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CREATE_TAPE_PARTITION_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREATE_TAPE_PARTITION_METHOD")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for CREATE_TAPE_PARTITION_METHOD {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub const CRM_PROTOCOL_DYNAMIC_MARSHAL_INFO: u32 = 2u32;
pub const CRM_PROTOCOL_EXPLICIT_MARSHAL_ONLY: u32 = 1u32;
pub const CRM_PROTOCOL_MAXIMUM_OPTION: u32 = 3u32;
pub const CSC_CACHE_AUTO_REINT: u32 = 16u32;
pub const CSC_CACHE_MANUAL_REINT: u32 = 0u32;
pub const CSC_CACHE_NONE: u32 = 48u32;
pub const CSC_CACHE_VDO: u32 = 32u32;
pub const CSC_MASK: u32 = 48u32;
pub const CSC_MASK_EXT: u32 = 8240u32;
pub const CSV_BLOCK_AND_FILE_CACHE_CALLBACK_VERSION: u32 = 2u32;
pub const CSV_BLOCK_CACHE_CALLBACK_VERSION: u32 = 1u32;
pub const ClfsClientRecord: u8 = 3u8;
pub const ClfsContainerActive: u32 = 4u32;
pub const ClfsContainerActivePendingDelete: u32 = 8u32;
pub const ClfsContainerInactive: u32 = 2u32;
pub const ClfsContainerInitializing: u32 = 1u32;
pub const ClfsContainerPendingArchive: u32 = 16u32;
pub const ClfsContainerPendingArchiveAndDelete: u32 = 32u32;
pub const ClfsDataRecord: u8 = 1u8;
pub const ClfsNullRecord: u8 = 0u8;
pub const ClfsRestartRecord: u8 = 2u8;
pub const ClsContainerActive: u32 = 4u32;
pub const ClsContainerActivePendingDelete: u32 = 8u32;
pub const ClsContainerInactive: u32 = 2u32;
pub const ClsContainerInitializing: u32 = 1u32;
pub const ClsContainerPendingArchive: u32 = 16u32;
pub const ClsContainerPendingArchiveAndDelete: u32 = 32u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DEFINE_DOS_DEVICE_FLAGS(pub u32);
pub const DDD_RAW_TARGET_PATH: DEFINE_DOS_DEVICE_FLAGS = DEFINE_DOS_DEVICE_FLAGS(1u32);
pub const DDD_REMOVE_DEFINITION: DEFINE_DOS_DEVICE_FLAGS = DEFINE_DOS_DEVICE_FLAGS(2u32);
pub const DDD_EXACT_MATCH_ON_REMOVE: DEFINE_DOS_DEVICE_FLAGS = DEFINE_DOS_DEVICE_FLAGS(4u32);
pub const DDD_NO_BROADCAST_SYSTEM: DEFINE_DOS_DEVICE_FLAGS = DEFINE_DOS_DEVICE_FLAGS(8u32);
pub const DDD_LUID_BROADCAST_DRIVE: DEFINE_DOS_DEVICE_FLAGS = DEFINE_DOS_DEVICE_FLAGS(16u32);
impl ::core::marker::Copy for DEFINE_DOS_DEVICE_FLAGS {}
impl ::core::clone::Clone for DEFINE_DOS_DEVICE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEFINE_DOS_DEVICE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEFINE_DOS_DEVICE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEFINE_DOS_DEVICE_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for DEFINE_DOS_DEVICE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DEFINE_DOS_DEVICE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DEFINE_DOS_DEVICE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DEFINE_DOS_DEVICE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DEFINE_DOS_DEVICE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for DEFINE_DOS_DEVICE_FLAGS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub const DISKQUOTA_FILESTATE_INCOMPLETE: u32 = 256u32;
pub const DISKQUOTA_FILESTATE_MASK: u32 = 768u32;
pub const DISKQUOTA_FILESTATE_REBUILDING: u32 = 512u32;
pub const DISKQUOTA_LOGFLAG_USER_LIMIT: u32 = 2u32;
pub const DISKQUOTA_LOGFLAG_USER_THRESHOLD: u32 = 1u32;
pub const DISKQUOTA_STATE_DISABLED: u32 = 0u32;
pub const DISKQUOTA_STATE_ENFORCE: u32 = 2u32;
pub const DISKQUOTA_STATE_MASK: u32 = 3u32;
pub const DISKQUOTA_STATE_TRACK: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISKQUOTA_USERNAME_RESOLVE(pub u32);
pub const DISKQUOTA_USERNAME_RESOLVE_ASYNC: DISKQUOTA_USERNAME_RESOLVE =
    DISKQUOTA_USERNAME_RESOLVE(2u32);
pub const DISKQUOTA_USERNAME_RESOLVE_NONE: DISKQUOTA_USERNAME_RESOLVE =
    DISKQUOTA_USERNAME_RESOLVE(0u32);
pub const DISKQUOTA_USERNAME_RESOLVE_SYNC: DISKQUOTA_USERNAME_RESOLVE =
    DISKQUOTA_USERNAME_RESOLVE(1u32);
impl ::core::marker::Copy for DISKQUOTA_USERNAME_RESOLVE {}
impl ::core::clone::Clone for DISKQUOTA_USERNAME_RESOLVE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISKQUOTA_USERNAME_RESOLVE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISKQUOTA_USERNAME_RESOLVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISKQUOTA_USERNAME_RESOLVE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for DISKQUOTA_USERNAME_RESOLVE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub const DISKQUOTA_USER_ACCOUNT_DELETED: u32 = 2u32;
pub const DISKQUOTA_USER_ACCOUNT_INVALID: u32 = 3u32;
pub const DISKQUOTA_USER_ACCOUNT_RESOLVED: u32 = 0u32;
pub const DISKQUOTA_USER_ACCOUNT_UNAVAILABLE: u32 = 1u32;
pub const DISKQUOTA_USER_ACCOUNT_UNKNOWN: u32 = 4u32;
pub const DISKQUOTA_USER_ACCOUNT_UNRESOLVED: u32 = 5u32;
pub struct DISKQUOTA_USER_INFORMATION {
    pub QuotaUsed: i64,
    pub QuotaThreshold: i64,
    pub QuotaLimit: i64,
}
impl ::core::marker::Copy for DISKQUOTA_USER_INFORMATION {}
impl ::core::clone::Clone for DISKQUOTA_USER_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DISKQUOTA_USER_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISKQUOTA_USER_INFORMATION")
            .field("QuotaUsed", &self.QuotaUsed)
            .field("QuotaThreshold", &self.QuotaThreshold)
            .field("QuotaLimit", &self.QuotaLimit)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DISKQUOTA_USER_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.QuotaUsed == other.QuotaUsed
            && self.QuotaThreshold == other.QuotaThreshold
            && self.QuotaLimit == other.QuotaLimit
    }
}
impl ::core::cmp::Eq for DISKQUOTA_USER_INFORMATION {}
impl FromIntoMemory for DISKQUOTA_USER_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_QuotaUsed = <i64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_QuotaThreshold = <i64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_QuotaLimit = <i64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        Self {
            QuotaUsed: f_QuotaUsed,
            QuotaThreshold: f_QuotaThreshold,
            QuotaLimit: f_QuotaLimit,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.QuotaUsed, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.QuotaThreshold, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.QuotaLimit, &mut into[16..16 + 8]);
    }
    fn size() -> usize {
        24
    }
}
pub struct DISK_SPACE_INFORMATION {
    pub ActualTotalAllocationUnits: u64,
    pub ActualAvailableAllocationUnits: u64,
    pub ActualPoolUnavailableAllocationUnits: u64,
    pub CallerTotalAllocationUnits: u64,
    pub CallerAvailableAllocationUnits: u64,
    pub CallerPoolUnavailableAllocationUnits: u64,
    pub UsedAllocationUnits: u64,
    pub TotalReservedAllocationUnits: u64,
    pub VolumeStorageReserveAllocationUnits: u64,
    pub AvailableCommittedAllocationUnits: u64,
    pub PoolAvailableAllocationUnits: u64,
    pub SectorsPerAllocationUnit: u32,
    pub BytesPerSector: u32,
}
impl ::core::marker::Copy for DISK_SPACE_INFORMATION {}
impl ::core::clone::Clone for DISK_SPACE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DISK_SPACE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_SPACE_INFORMATION")
            .field(
                "ActualTotalAllocationUnits",
                &self.ActualTotalAllocationUnits,
            )
            .field(
                "ActualAvailableAllocationUnits",
                &self.ActualAvailableAllocationUnits,
            )
            .field(
                "ActualPoolUnavailableAllocationUnits",
                &self.ActualPoolUnavailableAllocationUnits,
            )
            .field(
                "CallerTotalAllocationUnits",
                &self.CallerTotalAllocationUnits,
            )
            .field(
                "CallerAvailableAllocationUnits",
                &self.CallerAvailableAllocationUnits,
            )
            .field(
                "CallerPoolUnavailableAllocationUnits",
                &self.CallerPoolUnavailableAllocationUnits,
            )
            .field("UsedAllocationUnits", &self.UsedAllocationUnits)
            .field(
                "TotalReservedAllocationUnits",
                &self.TotalReservedAllocationUnits,
            )
            .field(
                "VolumeStorageReserveAllocationUnits",
                &self.VolumeStorageReserveAllocationUnits,
            )
            .field(
                "AvailableCommittedAllocationUnits",
                &self.AvailableCommittedAllocationUnits,
            )
            .field(
                "PoolAvailableAllocationUnits",
                &self.PoolAvailableAllocationUnits,
            )
            .field("SectorsPerAllocationUnit", &self.SectorsPerAllocationUnit)
            .field("BytesPerSector", &self.BytesPerSector)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DISK_SPACE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ActualTotalAllocationUnits == other.ActualTotalAllocationUnits
            && self.ActualAvailableAllocationUnits == other.ActualAvailableAllocationUnits
            && self.ActualPoolUnavailableAllocationUnits
                == other.ActualPoolUnavailableAllocationUnits
            && self.CallerTotalAllocationUnits == other.CallerTotalAllocationUnits
            && self.CallerAvailableAllocationUnits == other.CallerAvailableAllocationUnits
            && self.CallerPoolUnavailableAllocationUnits
                == other.CallerPoolUnavailableAllocationUnits
            && self.UsedAllocationUnits == other.UsedAllocationUnits
            && self.TotalReservedAllocationUnits == other.TotalReservedAllocationUnits
            && self.VolumeStorageReserveAllocationUnits == other.VolumeStorageReserveAllocationUnits
            && self.AvailableCommittedAllocationUnits == other.AvailableCommittedAllocationUnits
            && self.PoolAvailableAllocationUnits == other.PoolAvailableAllocationUnits
            && self.SectorsPerAllocationUnit == other.SectorsPerAllocationUnit
            && self.BytesPerSector == other.BytesPerSector
    }
}
impl ::core::cmp::Eq for DISK_SPACE_INFORMATION {}
impl FromIntoMemory for DISK_SPACE_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 96);
        let f_ActualTotalAllocationUnits = <u64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_ActualAvailableAllocationUnits = <u64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_ActualPoolUnavailableAllocationUnits =
            <u64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_CallerTotalAllocationUnits = <u64 as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        let f_CallerAvailableAllocationUnits =
            <u64 as FromIntoMemory>::from_bytes(&from[32..32 + 8]);
        let f_CallerPoolUnavailableAllocationUnits =
            <u64 as FromIntoMemory>::from_bytes(&from[40..40 + 8]);
        let f_UsedAllocationUnits = <u64 as FromIntoMemory>::from_bytes(&from[48..48 + 8]);
        let f_TotalReservedAllocationUnits = <u64 as FromIntoMemory>::from_bytes(&from[56..56 + 8]);
        let f_VolumeStorageReserveAllocationUnits =
            <u64 as FromIntoMemory>::from_bytes(&from[64..64 + 8]);
        let f_AvailableCommittedAllocationUnits =
            <u64 as FromIntoMemory>::from_bytes(&from[72..72 + 8]);
        let f_PoolAvailableAllocationUnits = <u64 as FromIntoMemory>::from_bytes(&from[80..80 + 8]);
        let f_SectorsPerAllocationUnit = <u32 as FromIntoMemory>::from_bytes(&from[88..88 + 4]);
        let f_BytesPerSector = <u32 as FromIntoMemory>::from_bytes(&from[92..92 + 4]);
        Self {
            ActualTotalAllocationUnits: f_ActualTotalAllocationUnits,
            ActualAvailableAllocationUnits: f_ActualAvailableAllocationUnits,
            ActualPoolUnavailableAllocationUnits: f_ActualPoolUnavailableAllocationUnits,
            CallerTotalAllocationUnits: f_CallerTotalAllocationUnits,
            CallerAvailableAllocationUnits: f_CallerAvailableAllocationUnits,
            CallerPoolUnavailableAllocationUnits: f_CallerPoolUnavailableAllocationUnits,
            UsedAllocationUnits: f_UsedAllocationUnits,
            TotalReservedAllocationUnits: f_TotalReservedAllocationUnits,
            VolumeStorageReserveAllocationUnits: f_VolumeStorageReserveAllocationUnits,
            AvailableCommittedAllocationUnits: f_AvailableCommittedAllocationUnits,
            PoolAvailableAllocationUnits: f_PoolAvailableAllocationUnits,
            SectorsPerAllocationUnit: f_SectorsPerAllocationUnit,
            BytesPerSector: f_BytesPerSector,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 96);
        FromIntoMemory::into_bytes(self.ActualTotalAllocationUnits, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.ActualAvailableAllocationUnits, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(
            self.ActualPoolUnavailableAllocationUnits,
            &mut into[16..16 + 8],
        );
        FromIntoMemory::into_bytes(self.CallerTotalAllocationUnits, &mut into[24..24 + 8]);
        FromIntoMemory::into_bytes(self.CallerAvailableAllocationUnits, &mut into[32..32 + 8]);
        FromIntoMemory::into_bytes(
            self.CallerPoolUnavailableAllocationUnits,
            &mut into[40..40 + 8],
        );
        FromIntoMemory::into_bytes(self.UsedAllocationUnits, &mut into[48..48 + 8]);
        FromIntoMemory::into_bytes(self.TotalReservedAllocationUnits, &mut into[56..56 + 8]);
        FromIntoMemory::into_bytes(
            self.VolumeStorageReserveAllocationUnits,
            &mut into[64..64 + 8],
        );
        FromIntoMemory::into_bytes(
            self.AvailableCommittedAllocationUnits,
            &mut into[72..72 + 8],
        );
        FromIntoMemory::into_bytes(self.PoolAvailableAllocationUnits, &mut into[80..80 + 8]);
        FromIntoMemory::into_bytes(self.SectorsPerAllocationUnit, &mut into[88..88 + 4]);
        FromIntoMemory::into_bytes(self.BytesPerSector, &mut into[92..92 + 4]);
    }
    fn size() -> usize {
        96
    }
}
pub const EA_CONTAINER_NAME: &'static str = "ContainerName";
pub const EA_CONTAINER_SIZE: &'static str = "ContainerSize";
pub struct EFS_CERTIFICATE_BLOB {
    pub dwCertEncodingType: u32,
    pub cbData: u32,
    pub pbData: MutPtr<u8>,
}
impl ::core::marker::Copy for EFS_CERTIFICATE_BLOB {}
impl ::core::clone::Clone for EFS_CERTIFICATE_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EFS_CERTIFICATE_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EFS_CERTIFICATE_BLOB")
            .field("dwCertEncodingType", &self.dwCertEncodingType)
            .field("cbData", &self.cbData)
            .field("pbData", &self.pbData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EFS_CERTIFICATE_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.dwCertEncodingType == other.dwCertEncodingType
            && self.cbData == other.cbData
            && self.pbData == other.pbData
    }
}
impl ::core::cmp::Eq for EFS_CERTIFICATE_BLOB {}
impl FromIntoMemory for EFS_CERTIFICATE_BLOB {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_dwCertEncodingType = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_cbData = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_pbData = <MutPtr<u8> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            dwCertEncodingType: f_dwCertEncodingType,
            cbData: f_cbData,
            pbData: f_pbData,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.dwCertEncodingType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.cbData, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.pbData, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct EFS_COMPATIBILITY_INFO {
    pub EfsVersion: u32,
}
impl ::core::marker::Copy for EFS_COMPATIBILITY_INFO {}
impl ::core::clone::Clone for EFS_COMPATIBILITY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EFS_COMPATIBILITY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EFS_COMPATIBILITY_INFO")
            .field("EfsVersion", &self.EfsVersion)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EFS_COMPATIBILITY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.EfsVersion == other.EfsVersion
    }
}
impl ::core::cmp::Eq for EFS_COMPATIBILITY_INFO {}
impl FromIntoMemory for EFS_COMPATIBILITY_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_EfsVersion = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            EfsVersion: f_EfsVersion,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.EfsVersion, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub const EFS_COMPATIBILITY_VERSION_NCRYPT_PROTECTOR: u32 = 5u32;
pub const EFS_COMPATIBILITY_VERSION_PFILE_PROTECTOR: u32 = 6u32;
pub struct EFS_DECRYPTION_STATUS_INFO {
    pub dwDecryptionError: u32,
    pub dwHashOffset: u32,
    pub cbHash: u32,
}
impl ::core::marker::Copy for EFS_DECRYPTION_STATUS_INFO {}
impl ::core::clone::Clone for EFS_DECRYPTION_STATUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EFS_DECRYPTION_STATUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EFS_DECRYPTION_STATUS_INFO")
            .field("dwDecryptionError", &self.dwDecryptionError)
            .field("dwHashOffset", &self.dwHashOffset)
            .field("cbHash", &self.cbHash)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EFS_DECRYPTION_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwDecryptionError == other.dwDecryptionError
            && self.dwHashOffset == other.dwHashOffset
            && self.cbHash == other.cbHash
    }
}
impl ::core::cmp::Eq for EFS_DECRYPTION_STATUS_INFO {}
impl FromIntoMemory for EFS_DECRYPTION_STATUS_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_dwDecryptionError = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwHashOffset = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_cbHash = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            dwDecryptionError: f_dwDecryptionError,
            dwHashOffset: f_dwHashOffset,
            cbHash: f_cbHash,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.dwDecryptionError, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwHashOffset, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.cbHash, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub const EFS_EFS_SUBVER_EFS_CERT: u32 = 1u32;
pub struct EFS_ENCRYPTION_STATUS_INFO {
    pub bHasCurrentKey: super::super::Foundation::BOOL,
    pub dwEncryptionError: u32,
}
impl ::core::marker::Copy for EFS_ENCRYPTION_STATUS_INFO {}
impl ::core::clone::Clone for EFS_ENCRYPTION_STATUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EFS_ENCRYPTION_STATUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EFS_ENCRYPTION_STATUS_INFO")
            .field("bHasCurrentKey", &self.bHasCurrentKey)
            .field("dwEncryptionError", &self.dwEncryptionError)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EFS_ENCRYPTION_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.bHasCurrentKey == other.bHasCurrentKey
            && self.dwEncryptionError == other.dwEncryptionError
    }
}
impl ::core::cmp::Eq for EFS_ENCRYPTION_STATUS_INFO {}
impl FromIntoMemory for EFS_ENCRYPTION_STATUS_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_bHasCurrentKey =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwEncryptionError = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            bHasCurrentKey: f_bHasCurrentKey,
            dwEncryptionError: f_dwEncryptionError,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.bHasCurrentKey, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwEncryptionError, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct EFS_HASH_BLOB {
    pub cbData: u32,
    pub pbData: MutPtr<u8>,
}
impl ::core::marker::Copy for EFS_HASH_BLOB {}
impl ::core::clone::Clone for EFS_HASH_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EFS_HASH_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EFS_HASH_BLOB")
            .field("cbData", &self.cbData)
            .field("pbData", &self.pbData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EFS_HASH_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cbData == other.cbData && self.pbData == other.pbData
    }
}
impl ::core::cmp::Eq for EFS_HASH_BLOB {}
impl FromIntoMemory for EFS_HASH_BLOB {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_cbData = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_pbData = <MutPtr<u8> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            cbData: f_cbData,
            pbData: f_pbData,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.cbData, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.pbData, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct EFS_KEY_INFO {
    pub dwVersion: u32,
    pub Entropy: u32,
    pub Algorithm: u32,
    pub KeyLength: u32,
}
impl ::core::marker::Copy for EFS_KEY_INFO {}
impl ::core::clone::Clone for EFS_KEY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EFS_KEY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EFS_KEY_INFO")
            .field("dwVersion", &self.dwVersion)
            .field("Entropy", &self.Entropy)
            .field("Algorithm", &self.Algorithm)
            .field("KeyLength", &self.KeyLength)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EFS_KEY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion
            && self.Entropy == other.Entropy
            && self.Algorithm == other.Algorithm
            && self.KeyLength == other.KeyLength
    }
}
impl ::core::cmp::Eq for EFS_KEY_INFO {}
impl FromIntoMemory for EFS_KEY_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dwVersion = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Entropy = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Algorithm = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_KeyLength = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            dwVersion: f_dwVersion,
            Entropy: f_Entropy,
            Algorithm: f_Algorithm,
            KeyLength: f_KeyLength,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dwVersion, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Entropy, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Algorithm, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.KeyLength, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub const EFS_METADATA_ADD_USER: u32 = 1u32;
pub const EFS_METADATA_GENERAL_OP: u32 = 8u32;
pub const EFS_METADATA_REMOVE_USER: u32 = 2u32;
pub const EFS_METADATA_REPLACE_USER: u32 = 4u32;
pub const EFS_PFILE_SUBVER_APPX: u32 = 3u32;
pub const EFS_PFILE_SUBVER_RMS: u32 = 2u32;
pub struct EFS_PIN_BLOB {
    pub cbPadding: u32,
    pub cbData: u32,
    pub pbData: MutPtr<u8>,
}
impl ::core::marker::Copy for EFS_PIN_BLOB {}
impl ::core::clone::Clone for EFS_PIN_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EFS_PIN_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EFS_PIN_BLOB")
            .field("cbPadding", &self.cbPadding)
            .field("cbData", &self.cbData)
            .field("pbData", &self.pbData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EFS_PIN_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cbPadding == other.cbPadding
            && self.cbData == other.cbData
            && self.pbData == other.pbData
    }
}
impl ::core::cmp::Eq for EFS_PIN_BLOB {}
impl FromIntoMemory for EFS_PIN_BLOB {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_cbPadding = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_cbData = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_pbData = <MutPtr<u8> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            cbPadding: f_cbPadding,
            cbData: f_cbData,
            pbData: f_pbData,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.cbPadding, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.cbData, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.pbData, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct EFS_RPC_BLOB {
    pub cbData: u32,
    pub pbData: MutPtr<u8>,
}
impl ::core::marker::Copy for EFS_RPC_BLOB {}
impl ::core::clone::Clone for EFS_RPC_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EFS_RPC_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EFS_RPC_BLOB")
            .field("cbData", &self.cbData)
            .field("pbData", &self.pbData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EFS_RPC_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cbData == other.cbData && self.pbData == other.pbData
    }
}
impl ::core::cmp::Eq for EFS_RPC_BLOB {}
impl FromIntoMemory for EFS_RPC_BLOB {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_cbData = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_pbData = <MutPtr<u8> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            cbData: f_cbData,
            pbData: f_pbData,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.cbData, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.pbData, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub const EFS_SUBVER_UNKNOWN: u32 = 0u32;
pub struct EFS_VERSION_INFO {
    pub EfsVersion: u32,
    pub SubVersion: u32,
}
impl ::core::marker::Copy for EFS_VERSION_INFO {}
impl ::core::clone::Clone for EFS_VERSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EFS_VERSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EFS_VERSION_INFO")
            .field("EfsVersion", &self.EfsVersion)
            .field("SubVersion", &self.SubVersion)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EFS_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.EfsVersion == other.EfsVersion && self.SubVersion == other.SubVersion
    }
}
impl ::core::cmp::Eq for EFS_VERSION_INFO {}
impl FromIntoMemory for EFS_VERSION_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_EfsVersion = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_SubVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            EfsVersion: f_EfsVersion,
            SubVersion: f_SubVersion,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.EfsVersion, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.SubVersion, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ENCRYPTED_FILE_METADATA_SIGNATURE {
    pub dwEfsAccessType: u32,
    pub pCertificatesAdded: MutPtr<ENCRYPTION_CERTIFICATE_HASH_LIST>,
    pub pEncryptionCertificate: MutPtr<ENCRYPTION_CERTIFICATE>,
    pub pEfsStreamSignature: MutPtr<EFS_RPC_BLOB>,
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ENCRYPTED_FILE_METADATA_SIGNATURE {}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ENCRYPTED_FILE_METADATA_SIGNATURE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ENCRYPTED_FILE_METADATA_SIGNATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTED_FILE_METADATA_SIGNATURE")
            .field("dwEfsAccessType", &self.dwEfsAccessType)
            .field("pCertificatesAdded", &self.pCertificatesAdded)
            .field("pEncryptionCertificate", &self.pEncryptionCertificate)
            .field("pEfsStreamSignature", &self.pEfsStreamSignature)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ENCRYPTED_FILE_METADATA_SIGNATURE {
    fn eq(&self, other: &Self) -> bool {
        self.dwEfsAccessType == other.dwEfsAccessType
            && self.pCertificatesAdded == other.pCertificatesAdded
            && self.pEncryptionCertificate == other.pEncryptionCertificate
            && self.pEfsStreamSignature == other.pEfsStreamSignature
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ENCRYPTED_FILE_METADATA_SIGNATURE {}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ENCRYPTED_FILE_METADATA_SIGNATURE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dwEfsAccessType = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_pCertificatesAdded =
            <MutPtr<ENCRYPTION_CERTIFICATE_HASH_LIST> as FromIntoMemory>::from_bytes(
                &from[4..4 + 4],
            );
        let f_pEncryptionCertificate =
            <MutPtr<ENCRYPTION_CERTIFICATE> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_pEfsStreamSignature =
            <MutPtr<EFS_RPC_BLOB> as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            dwEfsAccessType: f_dwEfsAccessType,
            pCertificatesAdded: f_pCertificatesAdded,
            pEncryptionCertificate: f_pEncryptionCertificate,
            pEfsStreamSignature: f_pEfsStreamSignature,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dwEfsAccessType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.pCertificatesAdded, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.pEncryptionCertificate, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.pEfsStreamSignature, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ENCRYPTION_CERTIFICATE {
    pub cbTotalLength: u32,
    pub pUserSid: MutPtr<super::super::Security::SID>,
    pub pCertBlob: MutPtr<EFS_CERTIFICATE_BLOB>,
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ENCRYPTION_CERTIFICATE {}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ENCRYPTION_CERTIFICATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ENCRYPTION_CERTIFICATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTION_CERTIFICATE")
            .field("cbTotalLength", &self.cbTotalLength)
            .field("pUserSid", &self.pUserSid)
            .field("pCertBlob", &self.pCertBlob)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ENCRYPTION_CERTIFICATE {
    fn eq(&self, other: &Self) -> bool {
        self.cbTotalLength == other.cbTotalLength
            && self.pUserSid == other.pUserSid
            && self.pCertBlob == other.pCertBlob
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ENCRYPTION_CERTIFICATE {}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ENCRYPTION_CERTIFICATE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_cbTotalLength = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_pUserSid =
            <MutPtr<super::super::Security::SID> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_pCertBlob =
            <MutPtr<EFS_CERTIFICATE_BLOB> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            cbTotalLength: f_cbTotalLength,
            pUserSid: f_pUserSid,
            pCertBlob: f_pCertBlob,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.cbTotalLength, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.pUserSid, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.pCertBlob, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ENCRYPTION_CERTIFICATE_HASH {
    pub cbTotalLength: u32,
    pub pUserSid: MutPtr<super::super::Security::SID>,
    pub pHash: MutPtr<EFS_HASH_BLOB>,
    pub lpDisplayInformation: PWSTR,
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ENCRYPTION_CERTIFICATE_HASH {}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ENCRYPTION_CERTIFICATE_HASH {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ENCRYPTION_CERTIFICATE_HASH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTION_CERTIFICATE_HASH")
            .field("cbTotalLength", &self.cbTotalLength)
            .field("pUserSid", &self.pUserSid)
            .field("pHash", &self.pHash)
            .field("lpDisplayInformation", &self.lpDisplayInformation)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ENCRYPTION_CERTIFICATE_HASH {
    fn eq(&self, other: &Self) -> bool {
        self.cbTotalLength == other.cbTotalLength
            && self.pUserSid == other.pUserSid
            && self.pHash == other.pHash
            && self.lpDisplayInformation == other.lpDisplayInformation
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ENCRYPTION_CERTIFICATE_HASH {}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ENCRYPTION_CERTIFICATE_HASH {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_cbTotalLength = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_pUserSid =
            <MutPtr<super::super::Security::SID> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_pHash = <MutPtr<EFS_HASH_BLOB> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpDisplayInformation = <PWSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            cbTotalLength: f_cbTotalLength,
            pUserSid: f_pUserSid,
            pHash: f_pHash,
            lpDisplayInformation: f_lpDisplayInformation,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.cbTotalLength, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.pUserSid, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.pHash, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpDisplayInformation, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ENCRYPTION_CERTIFICATE_HASH_LIST {
    pub nCert_Hash: u32,
    pub pUsers: MutPtr<ConstPtr<ENCRYPTION_CERTIFICATE_HASH>>,
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ENCRYPTION_CERTIFICATE_HASH_LIST {}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ENCRYPTION_CERTIFICATE_HASH_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ENCRYPTION_CERTIFICATE_HASH_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTION_CERTIFICATE_HASH_LIST")
            .field("nCert_Hash", &self.nCert_Hash)
            .field("pUsers", &self.pUsers)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ENCRYPTION_CERTIFICATE_HASH_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.nCert_Hash == other.nCert_Hash && self.pUsers == other.pUsers
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ENCRYPTION_CERTIFICATE_HASH_LIST {}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ENCRYPTION_CERTIFICATE_HASH_LIST {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_nCert_Hash = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_pUsers =
            <MutPtr<ConstPtr<ENCRYPTION_CERTIFICATE_HASH>> as FromIntoMemory>::from_bytes(
                &from[4..4 + 4],
            );
        Self {
            nCert_Hash: f_nCert_Hash,
            pUsers: f_pUsers,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.nCert_Hash, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.pUsers, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ENCRYPTION_CERTIFICATE_LIST {
    pub nUsers: u32,
    pub pUsers: MutPtr<ConstPtr<ENCRYPTION_CERTIFICATE>>,
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ENCRYPTION_CERTIFICATE_LIST {}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ENCRYPTION_CERTIFICATE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ENCRYPTION_CERTIFICATE_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTION_CERTIFICATE_LIST")
            .field("nUsers", &self.nUsers)
            .field("pUsers", &self.pUsers)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ENCRYPTION_CERTIFICATE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.nUsers == other.nUsers && self.pUsers == other.pUsers
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ENCRYPTION_CERTIFICATE_LIST {}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ENCRYPTION_CERTIFICATE_LIST {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_nUsers = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_pUsers = <MutPtr<ConstPtr<ENCRYPTION_CERTIFICATE>> as FromIntoMemory>::from_bytes(
            &from[4..4 + 4],
        );
        Self {
            nUsers: f_nUsers,
            pUsers: f_pUsers,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.nUsers, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.pUsers, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ENCRYPTION_PROTECTOR {
    pub cbTotalLength: u32,
    pub pUserSid: MutPtr<super::super::Security::SID>,
    pub lpProtectorDescriptor: PWSTR,
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ENCRYPTION_PROTECTOR {}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ENCRYPTION_PROTECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ENCRYPTION_PROTECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTION_PROTECTOR")
            .field("cbTotalLength", &self.cbTotalLength)
            .field("pUserSid", &self.pUserSid)
            .field("lpProtectorDescriptor", &self.lpProtectorDescriptor)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ENCRYPTION_PROTECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.cbTotalLength == other.cbTotalLength
            && self.pUserSid == other.pUserSid
            && self.lpProtectorDescriptor == other.lpProtectorDescriptor
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ENCRYPTION_PROTECTOR {}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ENCRYPTION_PROTECTOR {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_cbTotalLength = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_pUserSid =
            <MutPtr<super::super::Security::SID> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpProtectorDescriptor = <PWSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            cbTotalLength: f_cbTotalLength,
            pUserSid: f_pUserSid,
            lpProtectorDescriptor: f_lpProtectorDescriptor,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.cbTotalLength, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.pUserSid, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpProtectorDescriptor, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ENCRYPTION_PROTECTOR_LIST {
    pub nProtectors: u32,
    pub pProtectors: MutPtr<ConstPtr<ENCRYPTION_PROTECTOR>>,
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ENCRYPTION_PROTECTOR_LIST {}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ENCRYPTION_PROTECTOR_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ENCRYPTION_PROTECTOR_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTION_PROTECTOR_LIST")
            .field("nProtectors", &self.nProtectors)
            .field("pProtectors", &self.pProtectors)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ENCRYPTION_PROTECTOR_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.nProtectors == other.nProtectors && self.pProtectors == other.pProtectors
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ENCRYPTION_PROTECTOR_LIST {}
#[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ENCRYPTION_PROTECTOR_LIST {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_nProtectors = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_pProtectors =
            <MutPtr<ConstPtr<ENCRYPTION_PROTECTOR>> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            nProtectors: f_nProtectors,
            pProtectors: f_pProtectors,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.nProtectors, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.pProtectors, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub const ENLISTMENT_MAXIMUM_OPTION: u32 = 1u32;
pub const ENLISTMENT_OBJECT_PATH: &'static str = "\\Enlistment\\";
pub const ENLISTMENT_SUPERIOR: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ERASE_TAPE_TYPE(pub i32);
pub const TAPE_ERASE_LONG: ERASE_TAPE_TYPE = ERASE_TAPE_TYPE(1i32);
pub const TAPE_ERASE_SHORT: ERASE_TAPE_TYPE = ERASE_TAPE_TYPE(0i32);
impl ::core::marker::Copy for ERASE_TAPE_TYPE {}
impl ::core::clone::Clone for ERASE_TAPE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ERASE_TAPE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ERASE_TAPE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ERASE_TAPE_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for ERASE_TAPE_TYPE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub type FCACHE_CREATE_CALLBACK = StdCallFnPtr<
    (PCSTR, MutPtr<::core::ffi::c_void>, MutPtr<u32>, MutPtr<u32>),
    super::super::Foundation::HANDLE,
>;
pub type FCACHE_RICHCREATE_CALLBACK = StdCallFnPtr<
    (
        PCSTR,
        MutPtr<::core::ffi::c_void>,
        MutPtr<u32>,
        MutPtr<u32>,
        MutPtr<super::super::Foundation::BOOL>,
        MutPtr<super::super::Foundation::BOOL>,
        MutPtr<super::super::Foundation::BOOL>,
        MutPtr<super::super::Foundation::BOOL>,
    ),
    super::super::Foundation::HANDLE,
>;
pub struct FH_OVERLAPPED {
    pub Internal: PtrRepr,
    pub InternalHigh: PtrRepr,
    pub Offset: u32,
    pub OffsetHigh: u32,
    pub hEvent: super::super::Foundation::HANDLE,
    pub pfnCompletion: PFN_IO_COMPLETION,
    pub Reserved1: PtrRepr,
    pub Reserved2: PtrRepr,
    pub Reserved3: PtrRepr,
    pub Reserved4: PtrRepr,
}
impl ::core::marker::Copy for FH_OVERLAPPED {}
impl ::core::clone::Clone for FH_OVERLAPPED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FH_OVERLAPPED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FH_OVERLAPPED")
            .field("Internal", &self.Internal)
            .field("InternalHigh", &self.InternalHigh)
            .field("Offset", &self.Offset)
            .field("OffsetHigh", &self.OffsetHigh)
            .field("hEvent", &self.hEvent)
            .field("pfnCompletion", &self.pfnCompletion)
            .field("Reserved1", &self.Reserved1)
            .field("Reserved2", &self.Reserved2)
            .field("Reserved3", &self.Reserved3)
            .field("Reserved4", &self.Reserved4)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FH_OVERLAPPED {
    fn eq(&self, other: &Self) -> bool {
        self.Internal == other.Internal
            && self.InternalHigh == other.InternalHigh
            && self.Offset == other.Offset
            && self.OffsetHigh == other.OffsetHigh
            && self.hEvent == other.hEvent
            && self.pfnCompletion == other.pfnCompletion
            && self.Reserved1 == other.Reserved1
            && self.Reserved2 == other.Reserved2
            && self.Reserved3 == other.Reserved3
            && self.Reserved4 == other.Reserved4
    }
}
impl ::core::cmp::Eq for FH_OVERLAPPED {}
impl FromIntoMemory for FH_OVERLAPPED {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_Internal = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_InternalHigh = <PtrRepr as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Offset = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_OffsetHigh = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_hEvent =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_pfnCompletion = <PFN_IO_COMPLETION as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_Reserved1 = <PtrRepr as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_Reserved2 = <PtrRepr as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_Reserved3 = <PtrRepr as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_Reserved4 = <PtrRepr as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        Self {
            Internal: f_Internal,
            InternalHigh: f_InternalHigh,
            Offset: f_Offset,
            OffsetHigh: f_OffsetHigh,
            hEvent: f_hEvent,
            pfnCompletion: f_pfnCompletion,
            Reserved1: f_Reserved1,
            Reserved2: f_Reserved2,
            Reserved3: f_Reserved3,
            Reserved4: f_Reserved4,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.Internal, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.InternalHigh, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Offset, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.OffsetHigh, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.hEvent, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.pfnCompletion, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.Reserved1, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.Reserved2, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.Reserved3, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.Reserved4, &mut into[36..36 + 4]);
    }
    fn size() -> usize {
        40
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FILE_ACCESS_FLAGS(pub u32);
pub const FILE_READ_DATA: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(1u32);
pub const FILE_LIST_DIRECTORY: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(1u32);
pub const FILE_WRITE_DATA: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(2u32);
pub const FILE_ADD_FILE: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(2u32);
pub const FILE_APPEND_DATA: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(4u32);
pub const FILE_ADD_SUBDIRECTORY: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(4u32);
pub const FILE_CREATE_PIPE_INSTANCE: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(4u32);
pub const FILE_READ_EA: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(8u32);
pub const FILE_WRITE_EA: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(16u32);
pub const FILE_EXECUTE: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(32u32);
pub const FILE_TRAVERSE: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(32u32);
pub const FILE_DELETE_CHILD: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(64u32);
pub const FILE_READ_ATTRIBUTES: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(128u32);
pub const FILE_WRITE_ATTRIBUTES: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(256u32);
pub const READ_CONTROL: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(131072u32);
pub const SYNCHRONIZE: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(1048576u32);
pub const STANDARD_RIGHTS_REQUIRED: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(983040u32);
pub const STANDARD_RIGHTS_READ: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(131072u32);
pub const STANDARD_RIGHTS_WRITE: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(131072u32);
pub const STANDARD_RIGHTS_EXECUTE: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(131072u32);
pub const STANDARD_RIGHTS_ALL: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(2031616u32);
pub const SPECIFIC_RIGHTS_ALL: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(65535u32);
pub const FILE_ALL_ACCESS: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(2032127u32);
pub const FILE_GENERIC_READ: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(1179785u32);
pub const FILE_GENERIC_WRITE: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(1179926u32);
pub const FILE_GENERIC_EXECUTE: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(1179808u32);
impl ::core::marker::Copy for FILE_ACCESS_FLAGS {}
impl ::core::clone::Clone for FILE_ACCESS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_ACCESS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_ACCESS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_ACCESS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FILE_ACCESS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FILE_ACCESS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FILE_ACCESS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FILE_ACCESS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FILE_ACCESS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for FILE_ACCESS_FLAGS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FILE_ACTION(pub u32);
pub const FILE_ACTION_ADDED: FILE_ACTION = FILE_ACTION(1u32);
pub const FILE_ACTION_REMOVED: FILE_ACTION = FILE_ACTION(2u32);
pub const FILE_ACTION_MODIFIED: FILE_ACTION = FILE_ACTION(3u32);
pub const FILE_ACTION_RENAMED_OLD_NAME: FILE_ACTION = FILE_ACTION(4u32);
pub const FILE_ACTION_RENAMED_NEW_NAME: FILE_ACTION = FILE_ACTION(5u32);
impl ::core::marker::Copy for FILE_ACTION {}
impl ::core::clone::Clone for FILE_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_ACTION").field(&self.0).finish()
    }
}
impl FromIntoMemory for FILE_ACTION {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct FILE_ALIGNMENT_INFO {
    pub AlignmentRequirement: u32,
}
impl ::core::marker::Copy for FILE_ALIGNMENT_INFO {}
impl ::core::clone::Clone for FILE_ALIGNMENT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_ALIGNMENT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_ALIGNMENT_INFO")
            .field("AlignmentRequirement", &self.AlignmentRequirement)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILE_ALIGNMENT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.AlignmentRequirement == other.AlignmentRequirement
    }
}
impl ::core::cmp::Eq for FILE_ALIGNMENT_INFO {}
impl FromIntoMemory for FILE_ALIGNMENT_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_AlignmentRequirement = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            AlignmentRequirement: f_AlignmentRequirement,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.AlignmentRequirement, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct FILE_ALLOCATION_INFO {
    pub AllocationSize: i64,
}
impl ::core::marker::Copy for FILE_ALLOCATION_INFO {}
impl ::core::clone::Clone for FILE_ALLOCATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_ALLOCATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_ALLOCATION_INFO")
            .field("AllocationSize", &self.AllocationSize)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILE_ALLOCATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.AllocationSize == other.AllocationSize
    }
}
impl ::core::cmp::Eq for FILE_ALLOCATION_INFO {}
impl FromIntoMemory for FILE_ALLOCATION_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_AllocationSize = <i64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        Self {
            AllocationSize: f_AllocationSize,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.AllocationSize, &mut into[0..0 + 8]);
    }
    fn size() -> usize {
        8
    }
}
pub struct FILE_ATTRIBUTE_TAG_INFO {
    pub FileAttributes: u32,
    pub ReparseTag: u32,
}
impl ::core::marker::Copy for FILE_ATTRIBUTE_TAG_INFO {}
impl ::core::clone::Clone for FILE_ATTRIBUTE_TAG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_ATTRIBUTE_TAG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_ATTRIBUTE_TAG_INFO")
            .field("FileAttributes", &self.FileAttributes)
            .field("ReparseTag", &self.ReparseTag)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILE_ATTRIBUTE_TAG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.FileAttributes == other.FileAttributes && self.ReparseTag == other.ReparseTag
    }
}
impl ::core::cmp::Eq for FILE_ATTRIBUTE_TAG_INFO {}
impl FromIntoMemory for FILE_ATTRIBUTE_TAG_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_FileAttributes = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ReparseTag = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            FileAttributes: f_FileAttributes,
            ReparseTag: f_ReparseTag,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.FileAttributes, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ReparseTag, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct FILE_BASIC_INFO {
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub FileAttributes: u32,
}
impl ::core::marker::Copy for FILE_BASIC_INFO {}
impl ::core::clone::Clone for FILE_BASIC_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_BASIC_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_BASIC_INFO")
            .field("CreationTime", &self.CreationTime)
            .field("LastAccessTime", &self.LastAccessTime)
            .field("LastWriteTime", &self.LastWriteTime)
            .field("ChangeTime", &self.ChangeTime)
            .field("FileAttributes", &self.FileAttributes)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILE_BASIC_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.CreationTime == other.CreationTime
            && self.LastAccessTime == other.LastAccessTime
            && self.LastWriteTime == other.LastWriteTime
            && self.ChangeTime == other.ChangeTime
            && self.FileAttributes == other.FileAttributes
    }
}
impl ::core::cmp::Eq for FILE_BASIC_INFO {}
impl FromIntoMemory for FILE_BASIC_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_CreationTime = <i64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_LastAccessTime = <i64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_LastWriteTime = <i64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_ChangeTime = <i64 as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        let f_FileAttributes = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        Self {
            CreationTime: f_CreationTime,
            LastAccessTime: f_LastAccessTime,
            LastWriteTime: f_LastWriteTime,
            ChangeTime: f_ChangeTime,
            FileAttributes: f_FileAttributes,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.CreationTime, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.LastAccessTime, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.LastWriteTime, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.ChangeTime, &mut into[24..24 + 8]);
        FromIntoMemory::into_bytes(self.FileAttributes, &mut into[32..32 + 4]);
    }
    fn size() -> usize {
        40
    }
}
pub struct FILE_COMPRESSION_INFO {
    pub CompressedFileSize: i64,
    pub CompressionFormat: u16,
    pub CompressionUnitShift: u8,
    pub ChunkShift: u8,
    pub ClusterShift: u8,
    pub Reserved: [u8; 3],
}
impl ::core::marker::Copy for FILE_COMPRESSION_INFO {}
impl ::core::clone::Clone for FILE_COMPRESSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_COMPRESSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_COMPRESSION_INFO")
            .field("CompressedFileSize", &self.CompressedFileSize)
            .field("CompressionFormat", &self.CompressionFormat)
            .field("CompressionUnitShift", &self.CompressionUnitShift)
            .field("ChunkShift", &self.ChunkShift)
            .field("ClusterShift", &self.ClusterShift)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILE_COMPRESSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.CompressedFileSize == other.CompressedFileSize
            && self.CompressionFormat == other.CompressionFormat
            && self.CompressionUnitShift == other.CompressionUnitShift
            && self.ChunkShift == other.ChunkShift
            && self.ClusterShift == other.ClusterShift
            && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for FILE_COMPRESSION_INFO {}
impl FromIntoMemory for FILE_COMPRESSION_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_CompressedFileSize = <i64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_CompressionFormat = <u16 as FromIntoMemory>::from_bytes(&from[8..8 + 2]);
        let f_CompressionUnitShift = <u8 as FromIntoMemory>::from_bytes(&from[10..10 + 1]);
        let f_ChunkShift = <u8 as FromIntoMemory>::from_bytes(&from[11..11 + 1]);
        let f_ClusterShift = <u8 as FromIntoMemory>::from_bytes(&from[12..12 + 1]);
        let f_Reserved = <[u8; 3] as FromIntoMemory>::from_bytes(&from[13..13 + 3]);
        Self {
            CompressedFileSize: f_CompressedFileSize,
            CompressionFormat: f_CompressionFormat,
            CompressionUnitShift: f_CompressionUnitShift,
            ChunkShift: f_ChunkShift,
            ClusterShift: f_ClusterShift,
            Reserved: f_Reserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.CompressedFileSize, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.CompressionFormat, &mut into[8..8 + 2]);
        FromIntoMemory::into_bytes(self.CompressionUnitShift, &mut into[10..10 + 1]);
        FromIntoMemory::into_bytes(self.ChunkShift, &mut into[11..11 + 1]);
        FromIntoMemory::into_bytes(self.ClusterShift, &mut into[12..12 + 1]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[13..13 + 3]);
    }
    fn size() -> usize {
        16
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FILE_CREATION_DISPOSITION(pub u32);
pub const CREATE_NEW: FILE_CREATION_DISPOSITION = FILE_CREATION_DISPOSITION(1u32);
pub const CREATE_ALWAYS: FILE_CREATION_DISPOSITION = FILE_CREATION_DISPOSITION(2u32);
pub const OPEN_EXISTING: FILE_CREATION_DISPOSITION = FILE_CREATION_DISPOSITION(3u32);
pub const OPEN_ALWAYS: FILE_CREATION_DISPOSITION = FILE_CREATION_DISPOSITION(4u32);
pub const TRUNCATE_EXISTING: FILE_CREATION_DISPOSITION = FILE_CREATION_DISPOSITION(5u32);
impl ::core::marker::Copy for FILE_CREATION_DISPOSITION {}
impl ::core::clone::Clone for FILE_CREATION_DISPOSITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_CREATION_DISPOSITION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_CREATION_DISPOSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_CREATION_DISPOSITION")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for FILE_CREATION_DISPOSITION {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FILE_DEVICE_TYPE(pub u32);
pub const FILE_DEVICE_CD_ROM: FILE_DEVICE_TYPE = FILE_DEVICE_TYPE(2u32);
pub const FILE_DEVICE_DISK: FILE_DEVICE_TYPE = FILE_DEVICE_TYPE(7u32);
pub const FILE_DEVICE_TAPE: FILE_DEVICE_TYPE = FILE_DEVICE_TYPE(31u32);
pub const FILE_DEVICE_DVD: FILE_DEVICE_TYPE = FILE_DEVICE_TYPE(51u32);
impl ::core::marker::Copy for FILE_DEVICE_TYPE {}
impl ::core::clone::Clone for FILE_DEVICE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_DEVICE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_DEVICE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_DEVICE_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for FILE_DEVICE_TYPE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct FILE_DISPOSITION_INFO {
    pub DeleteFileA: super::super::Foundation::BOOLEAN,
}
impl ::core::marker::Copy for FILE_DISPOSITION_INFO {}
impl ::core::clone::Clone for FILE_DISPOSITION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_DISPOSITION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_DISPOSITION_INFO")
            .field("DeleteFileA", &self.DeleteFileA)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILE_DISPOSITION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.DeleteFileA == other.DeleteFileA
    }
}
impl ::core::cmp::Eq for FILE_DISPOSITION_INFO {}
impl FromIntoMemory for FILE_DISPOSITION_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 1);
        let f_DeleteFileA =
            <super::super::Foundation::BOOLEAN as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        Self {
            DeleteFileA: f_DeleteFileA,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 1);
        FromIntoMemory::into_bytes(self.DeleteFileA, &mut into[0..0 + 1]);
    }
    fn size() -> usize {
        1
    }
}
pub struct FILE_END_OF_FILE_INFO {
    pub EndOfFile: i64,
}
impl ::core::marker::Copy for FILE_END_OF_FILE_INFO {}
impl ::core::clone::Clone for FILE_END_OF_FILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_END_OF_FILE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_END_OF_FILE_INFO")
            .field("EndOfFile", &self.EndOfFile)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILE_END_OF_FILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.EndOfFile == other.EndOfFile
    }
}
impl ::core::cmp::Eq for FILE_END_OF_FILE_INFO {}
impl FromIntoMemory for FILE_END_OF_FILE_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_EndOfFile = <i64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        Self {
            EndOfFile: f_EndOfFile,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.EndOfFile, &mut into[0..0 + 8]);
    }
    fn size() -> usize {
        8
    }
}
pub struct FILE_EXTENT {
    pub VolumeOffset: u64,
    pub ExtentLength: u64,
}
impl ::core::marker::Copy for FILE_EXTENT {}
impl ::core::clone::Clone for FILE_EXTENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_EXTENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_EXTENT")
            .field("VolumeOffset", &self.VolumeOffset)
            .field("ExtentLength", &self.ExtentLength)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILE_EXTENT {
    fn eq(&self, other: &Self) -> bool {
        self.VolumeOffset == other.VolumeOffset && self.ExtentLength == other.ExtentLength
    }
}
impl ::core::cmp::Eq for FILE_EXTENT {}
impl FromIntoMemory for FILE_EXTENT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_VolumeOffset = <u64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_ExtentLength = <u64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        Self {
            VolumeOffset: f_VolumeOffset,
            ExtentLength: f_ExtentLength,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.VolumeOffset, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.ExtentLength, &mut into[8..8 + 8]);
    }
    fn size() -> usize {
        16
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FILE_FLAGS_AND_ATTRIBUTES(pub u32);
pub const FILE_ATTRIBUTE_READONLY: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(1u32);
pub const FILE_ATTRIBUTE_HIDDEN: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(2u32);
pub const FILE_ATTRIBUTE_SYSTEM: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(4u32);
pub const FILE_ATTRIBUTE_DIRECTORY: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(16u32);
pub const FILE_ATTRIBUTE_ARCHIVE: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(32u32);
pub const FILE_ATTRIBUTE_DEVICE: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(64u32);
pub const FILE_ATTRIBUTE_NORMAL: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(128u32);
pub const FILE_ATTRIBUTE_TEMPORARY: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(256u32);
pub const FILE_ATTRIBUTE_SPARSE_FILE: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(512u32);
pub const FILE_ATTRIBUTE_REPARSE_POINT: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(1024u32);
pub const FILE_ATTRIBUTE_COMPRESSED: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(2048u32);
pub const FILE_ATTRIBUTE_OFFLINE: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(4096u32);
pub const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(8192u32);
pub const FILE_ATTRIBUTE_ENCRYPTED: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(16384u32);
pub const FILE_ATTRIBUTE_INTEGRITY_STREAM: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(32768u32);
pub const FILE_ATTRIBUTE_VIRTUAL: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(65536u32);
pub const FILE_ATTRIBUTE_NO_SCRUB_DATA: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(131072u32);
pub const FILE_ATTRIBUTE_EA: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(262144u32);
pub const FILE_ATTRIBUTE_PINNED: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(524288u32);
pub const FILE_ATTRIBUTE_UNPINNED: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(1048576u32);
pub const FILE_ATTRIBUTE_RECALL_ON_OPEN: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(262144u32);
pub const FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(4194304u32);
pub const FILE_FLAG_WRITE_THROUGH: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(2147483648u32);
pub const FILE_FLAG_OVERLAPPED: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(1073741824u32);
pub const FILE_FLAG_NO_BUFFERING: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(536870912u32);
pub const FILE_FLAG_RANDOM_ACCESS: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(268435456u32);
pub const FILE_FLAG_SEQUENTIAL_SCAN: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(134217728u32);
pub const FILE_FLAG_DELETE_ON_CLOSE: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(67108864u32);
pub const FILE_FLAG_BACKUP_SEMANTICS: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(33554432u32);
pub const FILE_FLAG_POSIX_SEMANTICS: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(16777216u32);
pub const FILE_FLAG_SESSION_AWARE: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(8388608u32);
pub const FILE_FLAG_OPEN_REPARSE_POINT: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(2097152u32);
pub const FILE_FLAG_OPEN_NO_RECALL: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(1048576u32);
pub const FILE_FLAG_FIRST_PIPE_INSTANCE: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(524288u32);
pub const PIPE_ACCESS_DUPLEX: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(3u32);
pub const PIPE_ACCESS_INBOUND: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(1u32);
pub const PIPE_ACCESS_OUTBOUND: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(2u32);
pub const SECURITY_ANONYMOUS: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(0u32);
pub const SECURITY_IDENTIFICATION: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(65536u32);
pub const SECURITY_IMPERSONATION: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(131072u32);
pub const SECURITY_DELEGATION: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(196608u32);
pub const SECURITY_CONTEXT_TRACKING: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(262144u32);
pub const SECURITY_EFFECTIVE_ONLY: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(524288u32);
pub const SECURITY_SQOS_PRESENT: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(1048576u32);
pub const SECURITY_VALID_SQOS_FLAGS: FILE_FLAGS_AND_ATTRIBUTES =
    FILE_FLAGS_AND_ATTRIBUTES(2031616u32);
impl ::core::marker::Copy for FILE_FLAGS_AND_ATTRIBUTES {}
impl ::core::clone::Clone for FILE_FLAGS_AND_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_FLAGS_AND_ATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_FLAGS_AND_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_FLAGS_AND_ATTRIBUTES")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for FILE_FLAGS_AND_ATTRIBUTES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FILE_FLAGS_AND_ATTRIBUTES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FILE_FLAGS_AND_ATTRIBUTES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FILE_FLAGS_AND_ATTRIBUTES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FILE_FLAGS_AND_ATTRIBUTES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for FILE_FLAGS_AND_ATTRIBUTES {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct FILE_FULL_DIR_INFO {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_FULL_DIR_INFO {}
impl ::core::clone::Clone for FILE_FULL_DIR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_FULL_DIR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_FULL_DIR_INFO")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("FileIndex", &self.FileIndex)
            .field("CreationTime", &self.CreationTime)
            .field("LastAccessTime", &self.LastAccessTime)
            .field("LastWriteTime", &self.LastWriteTime)
            .field("ChangeTime", &self.ChangeTime)
            .field("EndOfFile", &self.EndOfFile)
            .field("AllocationSize", &self.AllocationSize)
            .field("FileAttributes", &self.FileAttributes)
            .field("FileNameLength", &self.FileNameLength)
            .field("EaSize", &self.EaSize)
            .field("FileName", &self.FileName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILE_FULL_DIR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset
            && self.FileIndex == other.FileIndex
            && self.CreationTime == other.CreationTime
            && self.LastAccessTime == other.LastAccessTime
            && self.LastWriteTime == other.LastWriteTime
            && self.ChangeTime == other.ChangeTime
            && self.EndOfFile == other.EndOfFile
            && self.AllocationSize == other.AllocationSize
            && self.FileAttributes == other.FileAttributes
            && self.FileNameLength == other.FileNameLength
            && self.EaSize == other.EaSize
            && self.FileName == other.FileName
    }
}
impl ::core::cmp::Eq for FILE_FULL_DIR_INFO {}
impl FromIntoMemory for FILE_FULL_DIR_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 72);
        let f_NextEntryOffset = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_FileIndex = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_CreationTime = <i64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_LastAccessTime = <i64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_LastWriteTime = <i64 as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        let f_ChangeTime = <i64 as FromIntoMemory>::from_bytes(&from[32..32 + 8]);
        let f_EndOfFile = <i64 as FromIntoMemory>::from_bytes(&from[40..40 + 8]);
        let f_AllocationSize = <i64 as FromIntoMemory>::from_bytes(&from[48..48 + 8]);
        let f_FileAttributes = <u32 as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_FileNameLength = <u32 as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_EaSize = <u32 as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        let f_FileName = <[u16; 1] as FromIntoMemory>::from_bytes(&from[68..68 + 1]);
        Self {
            NextEntryOffset: f_NextEntryOffset,
            FileIndex: f_FileIndex,
            CreationTime: f_CreationTime,
            LastAccessTime: f_LastAccessTime,
            LastWriteTime: f_LastWriteTime,
            ChangeTime: f_ChangeTime,
            EndOfFile: f_EndOfFile,
            AllocationSize: f_AllocationSize,
            FileAttributes: f_FileAttributes,
            FileNameLength: f_FileNameLength,
            EaSize: f_EaSize,
            FileName: f_FileName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 72);
        FromIntoMemory::into_bytes(self.NextEntryOffset, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.FileIndex, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.CreationTime, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.LastAccessTime, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.LastWriteTime, &mut into[24..24 + 8]);
        FromIntoMemory::into_bytes(self.ChangeTime, &mut into[32..32 + 8]);
        FromIntoMemory::into_bytes(self.EndOfFile, &mut into[40..40 + 8]);
        FromIntoMemory::into_bytes(self.AllocationSize, &mut into[48..48 + 8]);
        FromIntoMemory::into_bytes(self.FileAttributes, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.FileNameLength, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.EaSize, &mut into[64..64 + 4]);
        FromIntoMemory::into_bytes(self.FileName, &mut into[68..68 + 1]);
    }
    fn size() -> usize {
        72
    }
}
pub struct FILE_ID_128 {
    pub Identifier: [u8; 16],
}
impl ::core::marker::Copy for FILE_ID_128 {}
impl ::core::clone::Clone for FILE_ID_128 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_ID_128 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_ID_128")
            .field("Identifier", &self.Identifier)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILE_ID_128 {
    fn eq(&self, other: &Self) -> bool {
        self.Identifier == other.Identifier
    }
}
impl ::core::cmp::Eq for FILE_ID_128 {}
impl FromIntoMemory for FILE_ID_128 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_Identifier = <[u8; 16] as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        Self {
            Identifier: f_Identifier,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.Identifier, &mut into[0..0 + 16]);
    }
    fn size() -> usize {
        16
    }
}
pub struct FILE_ID_BOTH_DIR_INFO {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ShortNameLength: i8,
    pub ShortName: [u16; 12],
    pub FileId: i64,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_ID_BOTH_DIR_INFO {}
impl ::core::clone::Clone for FILE_ID_BOTH_DIR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_ID_BOTH_DIR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_ID_BOTH_DIR_INFO")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("FileIndex", &self.FileIndex)
            .field("CreationTime", &self.CreationTime)
            .field("LastAccessTime", &self.LastAccessTime)
            .field("LastWriteTime", &self.LastWriteTime)
            .field("ChangeTime", &self.ChangeTime)
            .field("EndOfFile", &self.EndOfFile)
            .field("AllocationSize", &self.AllocationSize)
            .field("FileAttributes", &self.FileAttributes)
            .field("FileNameLength", &self.FileNameLength)
            .field("EaSize", &self.EaSize)
            .field("ShortNameLength", &self.ShortNameLength)
            .field("ShortName", &self.ShortName)
            .field("FileId", &self.FileId)
            .field("FileName", &self.FileName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILE_ID_BOTH_DIR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset
            && self.FileIndex == other.FileIndex
            && self.CreationTime == other.CreationTime
            && self.LastAccessTime == other.LastAccessTime
            && self.LastWriteTime == other.LastWriteTime
            && self.ChangeTime == other.ChangeTime
            && self.EndOfFile == other.EndOfFile
            && self.AllocationSize == other.AllocationSize
            && self.FileAttributes == other.FileAttributes
            && self.FileNameLength == other.FileNameLength
            && self.EaSize == other.EaSize
            && self.ShortNameLength == other.ShortNameLength
            && self.ShortName == other.ShortName
            && self.FileId == other.FileId
            && self.FileName == other.FileName
    }
}
impl ::core::cmp::Eq for FILE_ID_BOTH_DIR_INFO {}
impl FromIntoMemory for FILE_ID_BOTH_DIR_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 104);
        let f_NextEntryOffset = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_FileIndex = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_CreationTime = <i64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_LastAccessTime = <i64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_LastWriteTime = <i64 as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        let f_ChangeTime = <i64 as FromIntoMemory>::from_bytes(&from[32..32 + 8]);
        let f_EndOfFile = <i64 as FromIntoMemory>::from_bytes(&from[40..40 + 8]);
        let f_AllocationSize = <i64 as FromIntoMemory>::from_bytes(&from[48..48 + 8]);
        let f_FileAttributes = <u32 as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_FileNameLength = <u32 as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_EaSize = <u32 as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        let f_ShortNameLength = <i8 as FromIntoMemory>::from_bytes(&from[68..68 + 1]);
        let f_ShortName = <[u16; 12] as FromIntoMemory>::from_bytes(&from[69..69 + 12]);
        let f_FileId = <i64 as FromIntoMemory>::from_bytes(&from[88..88 + 8]);
        let f_FileName = <[u16; 1] as FromIntoMemory>::from_bytes(&from[96..96 + 1]);
        Self {
            NextEntryOffset: f_NextEntryOffset,
            FileIndex: f_FileIndex,
            CreationTime: f_CreationTime,
            LastAccessTime: f_LastAccessTime,
            LastWriteTime: f_LastWriteTime,
            ChangeTime: f_ChangeTime,
            EndOfFile: f_EndOfFile,
            AllocationSize: f_AllocationSize,
            FileAttributes: f_FileAttributes,
            FileNameLength: f_FileNameLength,
            EaSize: f_EaSize,
            ShortNameLength: f_ShortNameLength,
            ShortName: f_ShortName,
            FileId: f_FileId,
            FileName: f_FileName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 104);
        FromIntoMemory::into_bytes(self.NextEntryOffset, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.FileIndex, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.CreationTime, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.LastAccessTime, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.LastWriteTime, &mut into[24..24 + 8]);
        FromIntoMemory::into_bytes(self.ChangeTime, &mut into[32..32 + 8]);
        FromIntoMemory::into_bytes(self.EndOfFile, &mut into[40..40 + 8]);
        FromIntoMemory::into_bytes(self.AllocationSize, &mut into[48..48 + 8]);
        FromIntoMemory::into_bytes(self.FileAttributes, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.FileNameLength, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.EaSize, &mut into[64..64 + 4]);
        FromIntoMemory::into_bytes(self.ShortNameLength, &mut into[68..68 + 1]);
        FromIntoMemory::into_bytes(self.ShortName, &mut into[69..69 + 12]);
        FromIntoMemory::into_bytes(self.FileId, &mut into[88..88 + 8]);
        FromIntoMemory::into_bytes(self.FileName, &mut into[96..96 + 1]);
    }
    fn size() -> usize {
        104
    }
}
pub struct FILE_ID_DESCRIPTOR {
    pub dwSize: u32,
    pub Type: FILE_ID_TYPE,
    pub Anonymous: FILE_ID_DESCRIPTOR_0,
}
impl ::core::marker::Copy for FILE_ID_DESCRIPTOR {}
impl ::core::clone::Clone for FILE_ID_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for FILE_ID_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.Type == other.Type && self.Anonymous == other.Anonymous
    }
}
impl ::core::cmp::Eq for FILE_ID_DESCRIPTOR {}
impl FromIntoMemory for FILE_ID_DESCRIPTOR {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Type = <FILE_ID_TYPE as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Anonymous = <FILE_ID_DESCRIPTOR_0 as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        Self {
            dwSize: f_dwSize,
            Type: f_Type,
            Anonymous: f_Anonymous,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Type, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[8..8 + 16]);
    }
    fn size() -> usize {
        24
    }
}
pub struct FILE_ID_DESCRIPTOR_0 {
    data: [u8; 16],
}
impl ::core::default::Default for FILE_ID_DESCRIPTOR_0 {
    fn default() -> Self {
        Self { data: [0u8; 16] }
    }
}
impl ::core::marker::Copy for FILE_ID_DESCRIPTOR_0 {}
impl ::core::clone::Clone for FILE_ID_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for FILE_ID_DESCRIPTOR_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for FILE_ID_DESCRIPTOR_0 {}
impl FromIntoMemory for FILE_ID_DESCRIPTOR_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 16];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        16
    }
}
pub struct FILE_ID_EXTD_DIR_INFO {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ReparsePointTag: u32,
    pub FileId: FILE_ID_128,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_ID_EXTD_DIR_INFO {}
impl ::core::clone::Clone for FILE_ID_EXTD_DIR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_ID_EXTD_DIR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_ID_EXTD_DIR_INFO")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("FileIndex", &self.FileIndex)
            .field("CreationTime", &self.CreationTime)
            .field("LastAccessTime", &self.LastAccessTime)
            .field("LastWriteTime", &self.LastWriteTime)
            .field("ChangeTime", &self.ChangeTime)
            .field("EndOfFile", &self.EndOfFile)
            .field("AllocationSize", &self.AllocationSize)
            .field("FileAttributes", &self.FileAttributes)
            .field("FileNameLength", &self.FileNameLength)
            .field("EaSize", &self.EaSize)
            .field("ReparsePointTag", &self.ReparsePointTag)
            .field("FileId", &self.FileId)
            .field("FileName", &self.FileName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILE_ID_EXTD_DIR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset
            && self.FileIndex == other.FileIndex
            && self.CreationTime == other.CreationTime
            && self.LastAccessTime == other.LastAccessTime
            && self.LastWriteTime == other.LastWriteTime
            && self.ChangeTime == other.ChangeTime
            && self.EndOfFile == other.EndOfFile
            && self.AllocationSize == other.AllocationSize
            && self.FileAttributes == other.FileAttributes
            && self.FileNameLength == other.FileNameLength
            && self.EaSize == other.EaSize
            && self.ReparsePointTag == other.ReparsePointTag
            && self.FileId == other.FileId
            && self.FileName == other.FileName
    }
}
impl ::core::cmp::Eq for FILE_ID_EXTD_DIR_INFO {}
impl FromIntoMemory for FILE_ID_EXTD_DIR_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 96);
        let f_NextEntryOffset = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_FileIndex = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_CreationTime = <i64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_LastAccessTime = <i64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_LastWriteTime = <i64 as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        let f_ChangeTime = <i64 as FromIntoMemory>::from_bytes(&from[32..32 + 8]);
        let f_EndOfFile = <i64 as FromIntoMemory>::from_bytes(&from[40..40 + 8]);
        let f_AllocationSize = <i64 as FromIntoMemory>::from_bytes(&from[48..48 + 8]);
        let f_FileAttributes = <u32 as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_FileNameLength = <u32 as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_EaSize = <u32 as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        let f_ReparsePointTag = <u32 as FromIntoMemory>::from_bytes(&from[68..68 + 4]);
        let f_FileId = <FILE_ID_128 as FromIntoMemory>::from_bytes(&from[72..72 + 16]);
        let f_FileName = <[u16; 1] as FromIntoMemory>::from_bytes(&from[88..88 + 1]);
        Self {
            NextEntryOffset: f_NextEntryOffset,
            FileIndex: f_FileIndex,
            CreationTime: f_CreationTime,
            LastAccessTime: f_LastAccessTime,
            LastWriteTime: f_LastWriteTime,
            ChangeTime: f_ChangeTime,
            EndOfFile: f_EndOfFile,
            AllocationSize: f_AllocationSize,
            FileAttributes: f_FileAttributes,
            FileNameLength: f_FileNameLength,
            EaSize: f_EaSize,
            ReparsePointTag: f_ReparsePointTag,
            FileId: f_FileId,
            FileName: f_FileName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 96);
        FromIntoMemory::into_bytes(self.NextEntryOffset, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.FileIndex, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.CreationTime, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.LastAccessTime, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.LastWriteTime, &mut into[24..24 + 8]);
        FromIntoMemory::into_bytes(self.ChangeTime, &mut into[32..32 + 8]);
        FromIntoMemory::into_bytes(self.EndOfFile, &mut into[40..40 + 8]);
        FromIntoMemory::into_bytes(self.AllocationSize, &mut into[48..48 + 8]);
        FromIntoMemory::into_bytes(self.FileAttributes, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.FileNameLength, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.EaSize, &mut into[64..64 + 4]);
        FromIntoMemory::into_bytes(self.ReparsePointTag, &mut into[68..68 + 4]);
        FromIntoMemory::into_bytes(self.FileId, &mut into[72..72 + 16]);
        FromIntoMemory::into_bytes(self.FileName, &mut into[88..88 + 1]);
    }
    fn size() -> usize {
        96
    }
}
pub struct FILE_ID_INFO {
    pub VolumeSerialNumber: u64,
    pub FileId: FILE_ID_128,
}
impl ::core::marker::Copy for FILE_ID_INFO {}
impl ::core::clone::Clone for FILE_ID_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_ID_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_ID_INFO")
            .field("VolumeSerialNumber", &self.VolumeSerialNumber)
            .field("FileId", &self.FileId)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILE_ID_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.VolumeSerialNumber == other.VolumeSerialNumber && self.FileId == other.FileId
    }
}
impl ::core::cmp::Eq for FILE_ID_INFO {}
impl FromIntoMemory for FILE_ID_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_VolumeSerialNumber = <u64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_FileId = <FILE_ID_128 as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        Self {
            VolumeSerialNumber: f_VolumeSerialNumber,
            FileId: f_FileId,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.VolumeSerialNumber, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.FileId, &mut into[8..8 + 16]);
    }
    fn size() -> usize {
        24
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FILE_ID_TYPE(pub i32);
pub const FileIdType: FILE_ID_TYPE = FILE_ID_TYPE(0i32);
pub const ObjectIdType: FILE_ID_TYPE = FILE_ID_TYPE(1i32);
pub const ExtendedFileIdType: FILE_ID_TYPE = FILE_ID_TYPE(2i32);
pub const MaximumFileIdType: FILE_ID_TYPE = FILE_ID_TYPE(3i32);
impl ::core::marker::Copy for FILE_ID_TYPE {}
impl ::core::clone::Clone for FILE_ID_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_ID_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_ID_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_ID_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for FILE_ID_TYPE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct FILE_INFO_2 {
    pub fi2_id: u32,
}
impl ::core::marker::Copy for FILE_INFO_2 {}
impl ::core::clone::Clone for FILE_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_INFO_2")
            .field("fi2_id", &self.fi2_id)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILE_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.fi2_id == other.fi2_id
    }
}
impl ::core::cmp::Eq for FILE_INFO_2 {}
impl FromIntoMemory for FILE_INFO_2 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_fi2_id = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self { fi2_id: f_fi2_id }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.fi2_id, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct FILE_INFO_3 {
    pub fi3_id: u32,
    pub fi3_permissions: FILE_INFO_FLAGS_PERMISSIONS,
    pub fi3_num_locks: u32,
    pub fi3_pathname: PWSTR,
    pub fi3_username: PWSTR,
}
impl ::core::marker::Copy for FILE_INFO_3 {}
impl ::core::clone::Clone for FILE_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_INFO_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_INFO_3")
            .field("fi3_id", &self.fi3_id)
            .field("fi3_permissions", &self.fi3_permissions)
            .field("fi3_num_locks", &self.fi3_num_locks)
            .field("fi3_pathname", &self.fi3_pathname)
            .field("fi3_username", &self.fi3_username)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILE_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        self.fi3_id == other.fi3_id
            && self.fi3_permissions == other.fi3_permissions
            && self.fi3_num_locks == other.fi3_num_locks
            && self.fi3_pathname == other.fi3_pathname
            && self.fi3_username == other.fi3_username
    }
}
impl ::core::cmp::Eq for FILE_INFO_3 {}
impl FromIntoMemory for FILE_INFO_3 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_fi3_id = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_fi3_permissions =
            <FILE_INFO_FLAGS_PERMISSIONS as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_fi3_num_locks = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_fi3_pathname = <PWSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_fi3_username = <PWSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            fi3_id: f_fi3_id,
            fi3_permissions: f_fi3_permissions,
            fi3_num_locks: f_fi3_num_locks,
            fi3_pathname: f_fi3_pathname,
            fi3_username: f_fi3_username,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.fi3_id, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.fi3_permissions, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.fi3_num_locks, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.fi3_pathname, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.fi3_username, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FILE_INFO_BY_HANDLE_CLASS(pub i32);
pub const FileBasicInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(0i32);
pub const FileStandardInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(1i32);
pub const FileNameInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(2i32);
pub const FileRenameInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(3i32);
pub const FileDispositionInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(4i32);
pub const FileAllocationInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(5i32);
pub const FileEndOfFileInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(6i32);
pub const FileStreamInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(7i32);
pub const FileCompressionInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(8i32);
pub const FileAttributeTagInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(9i32);
pub const FileIdBothDirectoryInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(10i32);
pub const FileIdBothDirectoryRestartInfo: FILE_INFO_BY_HANDLE_CLASS =
    FILE_INFO_BY_HANDLE_CLASS(11i32);
pub const FileIoPriorityHintInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(12i32);
pub const FileRemoteProtocolInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(13i32);
pub const FileFullDirectoryInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(14i32);
pub const FileFullDirectoryRestartInfo: FILE_INFO_BY_HANDLE_CLASS =
    FILE_INFO_BY_HANDLE_CLASS(15i32);
pub const FileStorageInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(16i32);
pub const FileAlignmentInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(17i32);
pub const FileIdInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(18i32);
pub const FileIdExtdDirectoryInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(19i32);
pub const FileIdExtdDirectoryRestartInfo: FILE_INFO_BY_HANDLE_CLASS =
    FILE_INFO_BY_HANDLE_CLASS(20i32);
pub const FileDispositionInfoEx: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(21i32);
pub const FileRenameInfoEx: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(22i32);
pub const FileCaseSensitiveInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(23i32);
pub const FileNormalizedNameInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(24i32);
pub const MaximumFileInfoByHandleClass: FILE_INFO_BY_HANDLE_CLASS =
    FILE_INFO_BY_HANDLE_CLASS(25i32);
impl ::core::marker::Copy for FILE_INFO_BY_HANDLE_CLASS {}
impl ::core::clone::Clone for FILE_INFO_BY_HANDLE_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_INFO_BY_HANDLE_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_INFO_BY_HANDLE_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_INFO_BY_HANDLE_CLASS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for FILE_INFO_BY_HANDLE_CLASS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FILE_INFO_FLAGS_PERMISSIONS(pub u32);
pub const PERM_FILE_READ: FILE_INFO_FLAGS_PERMISSIONS = FILE_INFO_FLAGS_PERMISSIONS(1u32);
pub const PERM_FILE_WRITE: FILE_INFO_FLAGS_PERMISSIONS = FILE_INFO_FLAGS_PERMISSIONS(2u32);
pub const PERM_FILE_CREATE: FILE_INFO_FLAGS_PERMISSIONS = FILE_INFO_FLAGS_PERMISSIONS(4u32);
impl ::core::marker::Copy for FILE_INFO_FLAGS_PERMISSIONS {}
impl ::core::clone::Clone for FILE_INFO_FLAGS_PERMISSIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_INFO_FLAGS_PERMISSIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_INFO_FLAGS_PERMISSIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_INFO_FLAGS_PERMISSIONS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for FILE_INFO_FLAGS_PERMISSIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FILE_INFO_FLAGS_PERMISSIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FILE_INFO_FLAGS_PERMISSIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FILE_INFO_FLAGS_PERMISSIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FILE_INFO_FLAGS_PERMISSIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for FILE_INFO_FLAGS_PERMISSIONS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct FILE_IO_PRIORITY_HINT_INFO {
    pub PriorityHint: PRIORITY_HINT,
}
impl ::core::marker::Copy for FILE_IO_PRIORITY_HINT_INFO {}
impl ::core::clone::Clone for FILE_IO_PRIORITY_HINT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_IO_PRIORITY_HINT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_IO_PRIORITY_HINT_INFO")
            .field("PriorityHint", &self.PriorityHint)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILE_IO_PRIORITY_HINT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PriorityHint == other.PriorityHint
    }
}
impl ::core::cmp::Eq for FILE_IO_PRIORITY_HINT_INFO {}
impl FromIntoMemory for FILE_IO_PRIORITY_HINT_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_PriorityHint = <PRIORITY_HINT as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            PriorityHint: f_PriorityHint,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.PriorityHint, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FILE_NAME(pub u32);
pub const FILE_NAME_NORMALIZED: FILE_NAME = FILE_NAME(0u32);
pub const FILE_NAME_OPENED: FILE_NAME = FILE_NAME(8u32);
impl ::core::marker::Copy for FILE_NAME {}
impl ::core::clone::Clone for FILE_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_NAME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_NAME").field(&self.0).finish()
    }
}
impl FromIntoMemory for FILE_NAME {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct FILE_NAME_INFO {
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_NAME_INFO {}
impl ::core::clone::Clone for FILE_NAME_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_NAME_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_NAME_INFO")
            .field("FileNameLength", &self.FileNameLength)
            .field("FileName", &self.FileName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILE_NAME_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.FileNameLength == other.FileNameLength && self.FileName == other.FileName
    }
}
impl ::core::cmp::Eq for FILE_NAME_INFO {}
impl FromIntoMemory for FILE_NAME_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_FileNameLength = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_FileName = <[u16; 1] as FromIntoMemory>::from_bytes(&from[4..4 + 1]);
        Self {
            FileNameLength: f_FileNameLength,
            FileName: f_FileName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.FileNameLength, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.FileName, &mut into[4..4 + 1]);
    }
    fn size() -> usize {
        8
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FILE_NOTIFY_CHANGE(pub u32);
pub const FILE_NOTIFY_CHANGE_FILE_NAME: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(1u32);
pub const FILE_NOTIFY_CHANGE_DIR_NAME: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(2u32);
pub const FILE_NOTIFY_CHANGE_ATTRIBUTES: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(4u32);
pub const FILE_NOTIFY_CHANGE_SIZE: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(8u32);
pub const FILE_NOTIFY_CHANGE_LAST_WRITE: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(16u32);
pub const FILE_NOTIFY_CHANGE_LAST_ACCESS: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(32u32);
pub const FILE_NOTIFY_CHANGE_CREATION: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(64u32);
pub const FILE_NOTIFY_CHANGE_SECURITY: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(256u32);
impl ::core::marker::Copy for FILE_NOTIFY_CHANGE {}
impl ::core::clone::Clone for FILE_NOTIFY_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_NOTIFY_CHANGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_NOTIFY_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_NOTIFY_CHANGE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FILE_NOTIFY_CHANGE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FILE_NOTIFY_CHANGE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FILE_NOTIFY_CHANGE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FILE_NOTIFY_CHANGE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FILE_NOTIFY_CHANGE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for FILE_NOTIFY_CHANGE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct FILE_NOTIFY_EXTENDED_INFORMATION {
    pub NextEntryOffset: u32,
    pub Action: FILE_ACTION,
    pub CreationTime: i64,
    pub LastModificationTime: i64,
    pub LastChangeTime: i64,
    pub LastAccessTime: i64,
    pub AllocatedLength: i64,
    pub FileSize: i64,
    pub FileAttributes: u32,
    pub ReparsePointTag: u32,
    pub FileId: i64,
    pub ParentFileId: i64,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_NOTIFY_EXTENDED_INFORMATION {}
impl ::core::clone::Clone for FILE_NOTIFY_EXTENDED_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_NOTIFY_EXTENDED_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_NOTIFY_EXTENDED_INFORMATION")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("Action", &self.Action)
            .field("CreationTime", &self.CreationTime)
            .field("LastModificationTime", &self.LastModificationTime)
            .field("LastChangeTime", &self.LastChangeTime)
            .field("LastAccessTime", &self.LastAccessTime)
            .field("AllocatedLength", &self.AllocatedLength)
            .field("FileSize", &self.FileSize)
            .field("FileAttributes", &self.FileAttributes)
            .field("ReparsePointTag", &self.ReparsePointTag)
            .field("FileId", &self.FileId)
            .field("ParentFileId", &self.ParentFileId)
            .field("FileNameLength", &self.FileNameLength)
            .field("FileName", &self.FileName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILE_NOTIFY_EXTENDED_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset
            && self.Action == other.Action
            && self.CreationTime == other.CreationTime
            && self.LastModificationTime == other.LastModificationTime
            && self.LastChangeTime == other.LastChangeTime
            && self.LastAccessTime == other.LastAccessTime
            && self.AllocatedLength == other.AllocatedLength
            && self.FileSize == other.FileSize
            && self.FileAttributes == other.FileAttributes
            && self.ReparsePointTag == other.ReparsePointTag
            && self.FileId == other.FileId
            && self.ParentFileId == other.ParentFileId
            && self.FileNameLength == other.FileNameLength
            && self.FileName == other.FileName
    }
}
impl ::core::cmp::Eq for FILE_NOTIFY_EXTENDED_INFORMATION {}
impl FromIntoMemory for FILE_NOTIFY_EXTENDED_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 88);
        let f_NextEntryOffset = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Action = <FILE_ACTION as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_CreationTime = <i64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_LastModificationTime = <i64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_LastChangeTime = <i64 as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        let f_LastAccessTime = <i64 as FromIntoMemory>::from_bytes(&from[32..32 + 8]);
        let f_AllocatedLength = <i64 as FromIntoMemory>::from_bytes(&from[40..40 + 8]);
        let f_FileSize = <i64 as FromIntoMemory>::from_bytes(&from[48..48 + 8]);
        let f_FileAttributes = <u32 as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_ReparsePointTag = <u32 as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_FileId = <i64 as FromIntoMemory>::from_bytes(&from[64..64 + 8]);
        let f_ParentFileId = <i64 as FromIntoMemory>::from_bytes(&from[72..72 + 8]);
        let f_FileNameLength = <u32 as FromIntoMemory>::from_bytes(&from[80..80 + 4]);
        let f_FileName = <[u16; 1] as FromIntoMemory>::from_bytes(&from[84..84 + 1]);
        Self {
            NextEntryOffset: f_NextEntryOffset,
            Action: f_Action,
            CreationTime: f_CreationTime,
            LastModificationTime: f_LastModificationTime,
            LastChangeTime: f_LastChangeTime,
            LastAccessTime: f_LastAccessTime,
            AllocatedLength: f_AllocatedLength,
            FileSize: f_FileSize,
            FileAttributes: f_FileAttributes,
            ReparsePointTag: f_ReparsePointTag,
            FileId: f_FileId,
            ParentFileId: f_ParentFileId,
            FileNameLength: f_FileNameLength,
            FileName: f_FileName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 88);
        FromIntoMemory::into_bytes(self.NextEntryOffset, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Action, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.CreationTime, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.LastModificationTime, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.LastChangeTime, &mut into[24..24 + 8]);
        FromIntoMemory::into_bytes(self.LastAccessTime, &mut into[32..32 + 8]);
        FromIntoMemory::into_bytes(self.AllocatedLength, &mut into[40..40 + 8]);
        FromIntoMemory::into_bytes(self.FileSize, &mut into[48..48 + 8]);
        FromIntoMemory::into_bytes(self.FileAttributes, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.ReparsePointTag, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.FileId, &mut into[64..64 + 8]);
        FromIntoMemory::into_bytes(self.ParentFileId, &mut into[72..72 + 8]);
        FromIntoMemory::into_bytes(self.FileNameLength, &mut into[80..80 + 4]);
        FromIntoMemory::into_bytes(self.FileName, &mut into[84..84 + 1]);
    }
    fn size() -> usize {
        88
    }
}
pub struct FILE_NOTIFY_INFORMATION {
    pub NextEntryOffset: u32,
    pub Action: FILE_ACTION,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_NOTIFY_INFORMATION {}
impl ::core::clone::Clone for FILE_NOTIFY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_NOTIFY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_NOTIFY_INFORMATION")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("Action", &self.Action)
            .field("FileNameLength", &self.FileNameLength)
            .field("FileName", &self.FileName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILE_NOTIFY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset
            && self.Action == other.Action
            && self.FileNameLength == other.FileNameLength
            && self.FileName == other.FileName
    }
}
impl ::core::cmp::Eq for FILE_NOTIFY_INFORMATION {}
impl FromIntoMemory for FILE_NOTIFY_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_NextEntryOffset = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Action = <FILE_ACTION as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_FileNameLength = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_FileName = <[u16; 1] as FromIntoMemory>::from_bytes(&from[12..12 + 1]);
        Self {
            NextEntryOffset: f_NextEntryOffset,
            Action: f_Action,
            FileNameLength: f_FileNameLength,
            FileName: f_FileName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.NextEntryOffset, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Action, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.FileNameLength, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.FileName, &mut into[12..12 + 1]);
    }
    fn size() -> usize {
        16
    }
}
pub const FILE_PROVIDER_COMPRESSION_LZX: u32 = 1u32;
pub const FILE_PROVIDER_COMPRESSION_XPRESS16K: u32 = 3u32;
pub const FILE_PROVIDER_COMPRESSION_XPRESS4K: u32 = 0u32;
pub const FILE_PROVIDER_COMPRESSION_XPRESS8K: u32 = 2u32;
pub struct FILE_REMOTE_PROTOCOL_INFO {
    pub StructureVersion: u16,
    pub StructureSize: u16,
    pub Protocol: u32,
    pub ProtocolMajorVersion: u16,
    pub ProtocolMinorVersion: u16,
    pub ProtocolRevision: u16,
    pub Reserved: u16,
    pub Flags: u32,
    pub GenericReserved: FILE_REMOTE_PROTOCOL_INFO_0,
    pub ProtocolSpecific: FILE_REMOTE_PROTOCOL_INFO_1,
}
impl ::core::marker::Copy for FILE_REMOTE_PROTOCOL_INFO {}
impl ::core::clone::Clone for FILE_REMOTE_PROTOCOL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for FILE_REMOTE_PROTOCOL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.StructureVersion == other.StructureVersion
            && self.StructureSize == other.StructureSize
            && self.Protocol == other.Protocol
            && self.ProtocolMajorVersion == other.ProtocolMajorVersion
            && self.ProtocolMinorVersion == other.ProtocolMinorVersion
            && self.ProtocolRevision == other.ProtocolRevision
            && self.Reserved == other.Reserved
            && self.Flags == other.Flags
            && self.GenericReserved == other.GenericReserved
            && self.ProtocolSpecific == other.ProtocolSpecific
    }
}
impl ::core::cmp::Eq for FILE_REMOTE_PROTOCOL_INFO {}
impl FromIntoMemory for FILE_REMOTE_PROTOCOL_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 116);
        let f_StructureVersion = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_StructureSize = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_Protocol = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_ProtocolMajorVersion = <u16 as FromIntoMemory>::from_bytes(&from[8..8 + 2]);
        let f_ProtocolMinorVersion = <u16 as FromIntoMemory>::from_bytes(&from[10..10 + 2]);
        let f_ProtocolRevision = <u16 as FromIntoMemory>::from_bytes(&from[12..12 + 2]);
        let f_Reserved = <u16 as FromIntoMemory>::from_bytes(&from[14..14 + 2]);
        let f_Flags = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_GenericReserved =
            <FILE_REMOTE_PROTOCOL_INFO_0 as FromIntoMemory>::from_bytes(&from[20..20 + 32]);
        let f_ProtocolSpecific =
            <FILE_REMOTE_PROTOCOL_INFO_1 as FromIntoMemory>::from_bytes(&from[52..52 + 64]);
        Self {
            StructureVersion: f_StructureVersion,
            StructureSize: f_StructureSize,
            Protocol: f_Protocol,
            ProtocolMajorVersion: f_ProtocolMajorVersion,
            ProtocolMinorVersion: f_ProtocolMinorVersion,
            ProtocolRevision: f_ProtocolRevision,
            Reserved: f_Reserved,
            Flags: f_Flags,
            GenericReserved: f_GenericReserved,
            ProtocolSpecific: f_ProtocolSpecific,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 116);
        FromIntoMemory::into_bytes(self.StructureVersion, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.StructureSize, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.Protocol, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.ProtocolMajorVersion, &mut into[8..8 + 2]);
        FromIntoMemory::into_bytes(self.ProtocolMinorVersion, &mut into[10..10 + 2]);
        FromIntoMemory::into_bytes(self.ProtocolRevision, &mut into[12..12 + 2]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[14..14 + 2]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.GenericReserved, &mut into[20..20 + 32]);
        FromIntoMemory::into_bytes(self.ProtocolSpecific, &mut into[52..52 + 64]);
    }
    fn size() -> usize {
        116
    }
}
pub struct FILE_REMOTE_PROTOCOL_INFO_0 {
    pub Reserved: [u32; 8],
}
impl ::core::marker::Copy for FILE_REMOTE_PROTOCOL_INFO_0 {}
impl ::core::clone::Clone for FILE_REMOTE_PROTOCOL_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_REMOTE_PROTOCOL_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_REMOTE_PROTOCOL_INFO_0")
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILE_REMOTE_PROTOCOL_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for FILE_REMOTE_PROTOCOL_INFO_0 {}
impl FromIntoMemory for FILE_REMOTE_PROTOCOL_INFO_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_Reserved = <[u32; 8] as FromIntoMemory>::from_bytes(&from[0..0 + 32]);
        Self {
            Reserved: f_Reserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[0..0 + 32]);
    }
    fn size() -> usize {
        32
    }
}
pub struct FILE_REMOTE_PROTOCOL_INFO_1 {
    data: [u8; 64],
}
impl ::core::default::Default for FILE_REMOTE_PROTOCOL_INFO_1 {
    fn default() -> Self {
        Self { data: [0u8; 64] }
    }
}
impl ::core::marker::Copy for FILE_REMOTE_PROTOCOL_INFO_1 {}
impl ::core::clone::Clone for FILE_REMOTE_PROTOCOL_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for FILE_REMOTE_PROTOCOL_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for FILE_REMOTE_PROTOCOL_INFO_1 {}
impl FromIntoMemory for FILE_REMOTE_PROTOCOL_INFO_1 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 64];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        64
    }
}
pub struct FILE_REMOTE_PROTOCOL_INFO_1_0 {
    pub Server: FILE_REMOTE_PROTOCOL_INFO_1_0_0,
    pub Share: FILE_REMOTE_PROTOCOL_INFO_1_0_1,
}
impl ::core::marker::Copy for FILE_REMOTE_PROTOCOL_INFO_1_0 {}
impl ::core::clone::Clone for FILE_REMOTE_PROTOCOL_INFO_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_REMOTE_PROTOCOL_INFO_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_REMOTE_PROTOCOL_INFO_1_0")
            .field("Server", &self.Server)
            .field("Share", &self.Share)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILE_REMOTE_PROTOCOL_INFO_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Server == other.Server && self.Share == other.Share
    }
}
impl ::core::cmp::Eq for FILE_REMOTE_PROTOCOL_INFO_1_0 {}
impl FromIntoMemory for FILE_REMOTE_PROTOCOL_INFO_1_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Server =
            <FILE_REMOTE_PROTOCOL_INFO_1_0_0 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Share =
            <FILE_REMOTE_PROTOCOL_INFO_1_0_1 as FromIntoMemory>::from_bytes(&from[4..4 + 8]);
        Self {
            Server: f_Server,
            Share: f_Share,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Server, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Share, &mut into[4..4 + 8]);
    }
    fn size() -> usize {
        12
    }
}
pub struct FILE_REMOTE_PROTOCOL_INFO_1_0_0 {
    pub Capabilities: u32,
}
impl ::core::marker::Copy for FILE_REMOTE_PROTOCOL_INFO_1_0_0 {}
impl ::core::clone::Clone for FILE_REMOTE_PROTOCOL_INFO_1_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_REMOTE_PROTOCOL_INFO_1_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_REMOTE_PROTOCOL_INFO_1_0_0")
            .field("Capabilities", &self.Capabilities)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILE_REMOTE_PROTOCOL_INFO_1_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities
    }
}
impl ::core::cmp::Eq for FILE_REMOTE_PROTOCOL_INFO_1_0_0 {}
impl FromIntoMemory for FILE_REMOTE_PROTOCOL_INFO_1_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_Capabilities = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            Capabilities: f_Capabilities,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.Capabilities, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct FILE_REMOTE_PROTOCOL_INFO_1_0_1 {
    pub Capabilities: u32,
    pub CachingFlags: u32,
}
impl ::core::marker::Copy for FILE_REMOTE_PROTOCOL_INFO_1_0_1 {}
impl ::core::clone::Clone for FILE_REMOTE_PROTOCOL_INFO_1_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_REMOTE_PROTOCOL_INFO_1_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_REMOTE_PROTOCOL_INFO_1_0_1")
            .field("Capabilities", &self.Capabilities)
            .field("CachingFlags", &self.CachingFlags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILE_REMOTE_PROTOCOL_INFO_1_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities && self.CachingFlags == other.CachingFlags
    }
}
impl ::core::cmp::Eq for FILE_REMOTE_PROTOCOL_INFO_1_0_1 {}
impl FromIntoMemory for FILE_REMOTE_PROTOCOL_INFO_1_0_1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Capabilities = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_CachingFlags = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            Capabilities: f_Capabilities,
            CachingFlags: f_CachingFlags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Capabilities, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.CachingFlags, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct FILE_RENAME_INFO {
    pub Anonymous: FILE_RENAME_INFO_0,
    pub RootDirectory: super::super::Foundation::HANDLE,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_RENAME_INFO {}
impl ::core::clone::Clone for FILE_RENAME_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for FILE_RENAME_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Anonymous == other.Anonymous
            && self.RootDirectory == other.RootDirectory
            && self.FileNameLength == other.FileNameLength
            && self.FileName == other.FileName
    }
}
impl ::core::cmp::Eq for FILE_RENAME_INFO {}
impl FromIntoMemory for FILE_RENAME_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_Anonymous = <FILE_RENAME_INFO_0 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_RootDirectory =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_FileNameLength = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_FileName = <[u16; 1] as FromIntoMemory>::from_bytes(&from[12..12 + 1]);
        Self {
            Anonymous: f_Anonymous,
            RootDirectory: f_RootDirectory,
            FileNameLength: f_FileNameLength,
            FileName: f_FileName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.RootDirectory, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.FileNameLength, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.FileName, &mut into[12..12 + 1]);
    }
    fn size() -> usize {
        16
    }
}
pub struct FILE_RENAME_INFO_0 {
    data: [u8; 4],
}
impl ::core::default::Default for FILE_RENAME_INFO_0 {
    fn default() -> Self {
        Self { data: [0u8; 4] }
    }
}
impl ::core::marker::Copy for FILE_RENAME_INFO_0 {}
impl ::core::clone::Clone for FILE_RENAME_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for FILE_RENAME_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for FILE_RENAME_INFO_0 {}
impl FromIntoMemory for FILE_RENAME_INFO_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 4];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        4
    }
}
pub struct FILE_SEGMENT_ELEMENT {
    data: [u8; 8],
}
impl ::core::default::Default for FILE_SEGMENT_ELEMENT {
    fn default() -> Self {
        Self { data: [0u8; 8] }
    }
}
impl ::core::marker::Copy for FILE_SEGMENT_ELEMENT {}
impl ::core::clone::Clone for FILE_SEGMENT_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for FILE_SEGMENT_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for FILE_SEGMENT_ELEMENT {}
impl FromIntoMemory for FILE_SEGMENT_ELEMENT {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 8];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        8
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FILE_SHARE_MODE(pub u32);
pub const FILE_SHARE_NONE: FILE_SHARE_MODE = FILE_SHARE_MODE(0u32);
pub const FILE_SHARE_DELETE: FILE_SHARE_MODE = FILE_SHARE_MODE(4u32);
pub const FILE_SHARE_READ: FILE_SHARE_MODE = FILE_SHARE_MODE(1u32);
pub const FILE_SHARE_WRITE: FILE_SHARE_MODE = FILE_SHARE_MODE(2u32);
impl ::core::marker::Copy for FILE_SHARE_MODE {}
impl ::core::clone::Clone for FILE_SHARE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_SHARE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_SHARE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_SHARE_MODE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FILE_SHARE_MODE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FILE_SHARE_MODE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FILE_SHARE_MODE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FILE_SHARE_MODE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FILE_SHARE_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for FILE_SHARE_MODE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct FILE_STANDARD_INFO {
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub NumberOfLinks: u32,
    pub DeletePending: super::super::Foundation::BOOLEAN,
    pub Directory: super::super::Foundation::BOOLEAN,
}
impl ::core::marker::Copy for FILE_STANDARD_INFO {}
impl ::core::clone::Clone for FILE_STANDARD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_STANDARD_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_STANDARD_INFO")
            .field("AllocationSize", &self.AllocationSize)
            .field("EndOfFile", &self.EndOfFile)
            .field("NumberOfLinks", &self.NumberOfLinks)
            .field("DeletePending", &self.DeletePending)
            .field("Directory", &self.Directory)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILE_STANDARD_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.AllocationSize == other.AllocationSize
            && self.EndOfFile == other.EndOfFile
            && self.NumberOfLinks == other.NumberOfLinks
            && self.DeletePending == other.DeletePending
            && self.Directory == other.Directory
    }
}
impl ::core::cmp::Eq for FILE_STANDARD_INFO {}
impl FromIntoMemory for FILE_STANDARD_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_AllocationSize = <i64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_EndOfFile = <i64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_NumberOfLinks = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_DeletePending =
            <super::super::Foundation::BOOLEAN as FromIntoMemory>::from_bytes(&from[20..20 + 1]);
        let f_Directory =
            <super::super::Foundation::BOOLEAN as FromIntoMemory>::from_bytes(&from[21..21 + 1]);
        Self {
            AllocationSize: f_AllocationSize,
            EndOfFile: f_EndOfFile,
            NumberOfLinks: f_NumberOfLinks,
            DeletePending: f_DeletePending,
            Directory: f_Directory,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.AllocationSize, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.EndOfFile, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.NumberOfLinks, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.DeletePending, &mut into[20..20 + 1]);
        FromIntoMemory::into_bytes(self.Directory, &mut into[21..21 + 1]);
    }
    fn size() -> usize {
        24
    }
}
pub struct FILE_STORAGE_INFO {
    pub LogicalBytesPerSector: u32,
    pub PhysicalBytesPerSectorForAtomicity: u32,
    pub PhysicalBytesPerSectorForPerformance: u32,
    pub FileSystemEffectivePhysicalBytesPerSectorForAtomicity: u32,
    pub Flags: u32,
    pub ByteOffsetForSectorAlignment: u32,
    pub ByteOffsetForPartitionAlignment: u32,
}
impl ::core::marker::Copy for FILE_STORAGE_INFO {}
impl ::core::clone::Clone for FILE_STORAGE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_STORAGE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_STORAGE_INFO")
            .field("LogicalBytesPerSector", &self.LogicalBytesPerSector)
            .field(
                "PhysicalBytesPerSectorForAtomicity",
                &self.PhysicalBytesPerSectorForAtomicity,
            )
            .field(
                "PhysicalBytesPerSectorForPerformance",
                &self.PhysicalBytesPerSectorForPerformance,
            )
            .field(
                "FileSystemEffectivePhysicalBytesPerSectorForAtomicity",
                &self.FileSystemEffectivePhysicalBytesPerSectorForAtomicity,
            )
            .field("Flags", &self.Flags)
            .field(
                "ByteOffsetForSectorAlignment",
                &self.ByteOffsetForSectorAlignment,
            )
            .field(
                "ByteOffsetForPartitionAlignment",
                &self.ByteOffsetForPartitionAlignment,
            )
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILE_STORAGE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.LogicalBytesPerSector == other.LogicalBytesPerSector
            && self.PhysicalBytesPerSectorForAtomicity == other.PhysicalBytesPerSectorForAtomicity
            && self.PhysicalBytesPerSectorForPerformance
                == other.PhysicalBytesPerSectorForPerformance
            && self.FileSystemEffectivePhysicalBytesPerSectorForAtomicity
                == other.FileSystemEffectivePhysicalBytesPerSectorForAtomicity
            && self.Flags == other.Flags
            && self.ByteOffsetForSectorAlignment == other.ByteOffsetForSectorAlignment
            && self.ByteOffsetForPartitionAlignment == other.ByteOffsetForPartitionAlignment
    }
}
impl ::core::cmp::Eq for FILE_STORAGE_INFO {}
impl FromIntoMemory for FILE_STORAGE_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_LogicalBytesPerSector = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_PhysicalBytesPerSectorForAtomicity =
            <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_PhysicalBytesPerSectorForPerformance =
            <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_FileSystemEffectivePhysicalBytesPerSectorForAtomicity =
            <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_Flags = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_ByteOffsetForSectorAlignment = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_ByteOffsetForPartitionAlignment =
            <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        Self {
            LogicalBytesPerSector: f_LogicalBytesPerSector,
            PhysicalBytesPerSectorForAtomicity: f_PhysicalBytesPerSectorForAtomicity,
            PhysicalBytesPerSectorForPerformance: f_PhysicalBytesPerSectorForPerformance,
            FileSystemEffectivePhysicalBytesPerSectorForAtomicity:
                f_FileSystemEffectivePhysicalBytesPerSectorForAtomicity,
            Flags: f_Flags,
            ByteOffsetForSectorAlignment: f_ByteOffsetForSectorAlignment,
            ByteOffsetForPartitionAlignment: f_ByteOffsetForPartitionAlignment,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.LogicalBytesPerSector, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.PhysicalBytesPerSectorForAtomicity, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(
            self.PhysicalBytesPerSectorForPerformance,
            &mut into[8..8 + 4],
        );
        FromIntoMemory::into_bytes(
            self.FileSystemEffectivePhysicalBytesPerSectorForAtomicity,
            &mut into[12..12 + 4],
        );
        FromIntoMemory::into_bytes(self.Flags, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.ByteOffsetForSectorAlignment, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.ByteOffsetForPartitionAlignment, &mut into[24..24 + 4]);
    }
    fn size() -> usize {
        28
    }
}
pub struct FILE_STREAM_INFO {
    pub NextEntryOffset: u32,
    pub StreamNameLength: u32,
    pub StreamSize: i64,
    pub StreamAllocationSize: i64,
    pub StreamName: [u16; 1],
}
impl ::core::marker::Copy for FILE_STREAM_INFO {}
impl ::core::clone::Clone for FILE_STREAM_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_STREAM_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_STREAM_INFO")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("StreamNameLength", &self.StreamNameLength)
            .field("StreamSize", &self.StreamSize)
            .field("StreamAllocationSize", &self.StreamAllocationSize)
            .field("StreamName", &self.StreamName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILE_STREAM_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset
            && self.StreamNameLength == other.StreamNameLength
            && self.StreamSize == other.StreamSize
            && self.StreamAllocationSize == other.StreamAllocationSize
            && self.StreamName == other.StreamName
    }
}
impl ::core::cmp::Eq for FILE_STREAM_INFO {}
impl FromIntoMemory for FILE_STREAM_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_NextEntryOffset = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_StreamNameLength = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_StreamSize = <i64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_StreamAllocationSize = <i64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_StreamName = <[u16; 1] as FromIntoMemory>::from_bytes(&from[24..24 + 1]);
        Self {
            NextEntryOffset: f_NextEntryOffset,
            StreamNameLength: f_StreamNameLength,
            StreamSize: f_StreamSize,
            StreamAllocationSize: f_StreamAllocationSize,
            StreamName: f_StreamName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.NextEntryOffset, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.StreamNameLength, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.StreamSize, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.StreamAllocationSize, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.StreamName, &mut into[24..24 + 1]);
    }
    fn size() -> usize {
        32
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FINDEX_INFO_LEVELS(pub i32);
pub const FindExInfoStandard: FINDEX_INFO_LEVELS = FINDEX_INFO_LEVELS(0i32);
pub const FindExInfoBasic: FINDEX_INFO_LEVELS = FINDEX_INFO_LEVELS(1i32);
pub const FindExInfoMaxInfoLevel: FINDEX_INFO_LEVELS = FINDEX_INFO_LEVELS(2i32);
impl ::core::marker::Copy for FINDEX_INFO_LEVELS {}
impl ::core::clone::Clone for FINDEX_INFO_LEVELS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FINDEX_INFO_LEVELS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FINDEX_INFO_LEVELS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FINDEX_INFO_LEVELS").field(&self.0).finish()
    }
}
impl FromIntoMemory for FINDEX_INFO_LEVELS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FINDEX_SEARCH_OPS(pub i32);
pub const FindExSearchNameMatch: FINDEX_SEARCH_OPS = FINDEX_SEARCH_OPS(0i32);
pub const FindExSearchLimitToDirectories: FINDEX_SEARCH_OPS = FINDEX_SEARCH_OPS(1i32);
pub const FindExSearchLimitToDevices: FINDEX_SEARCH_OPS = FINDEX_SEARCH_OPS(2i32);
pub const FindExSearchMaxSearchOp: FINDEX_SEARCH_OPS = FINDEX_SEARCH_OPS(3i32);
impl ::core::marker::Copy for FINDEX_SEARCH_OPS {}
impl ::core::clone::Clone for FINDEX_SEARCH_OPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FINDEX_SEARCH_OPS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FINDEX_SEARCH_OPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FINDEX_SEARCH_OPS").field(&self.0).finish()
    }
}
impl FromIntoMemory for FINDEX_SEARCH_OPS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FIND_FIRST_EX_FLAGS(pub u32);
pub const FIND_FIRST_EX_CASE_SENSITIVE: FIND_FIRST_EX_FLAGS = FIND_FIRST_EX_FLAGS(1u32);
pub const FIND_FIRST_EX_LARGE_FETCH: FIND_FIRST_EX_FLAGS = FIND_FIRST_EX_FLAGS(2u32);
pub const FIND_FIRST_EX_ON_DISK_ENTRIES_ONLY: FIND_FIRST_EX_FLAGS = FIND_FIRST_EX_FLAGS(4u32);
impl ::core::marker::Copy for FIND_FIRST_EX_FLAGS {}
impl ::core::clone::Clone for FIND_FIRST_EX_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FIND_FIRST_EX_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FIND_FIRST_EX_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FIND_FIRST_EX_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FIND_FIRST_EX_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FIND_FIRST_EX_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FIND_FIRST_EX_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FIND_FIRST_EX_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FIND_FIRST_EX_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for FIND_FIRST_EX_FLAGS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct FIO_CONTEXT {
    pub m_dwTempHack: u32,
    pub m_dwSignature: u32,
    pub m_hFile: super::super::Foundation::HANDLE,
    pub m_dwLinesOffset: u32,
    pub m_dwHeaderLength: u32,
}
impl ::core::marker::Copy for FIO_CONTEXT {}
impl ::core::clone::Clone for FIO_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FIO_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FIO_CONTEXT")
            .field("m_dwTempHack", &self.m_dwTempHack)
            .field("m_dwSignature", &self.m_dwSignature)
            .field("m_hFile", &self.m_hFile)
            .field("m_dwLinesOffset", &self.m_dwLinesOffset)
            .field("m_dwHeaderLength", &self.m_dwHeaderLength)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FIO_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.m_dwTempHack == other.m_dwTempHack
            && self.m_dwSignature == other.m_dwSignature
            && self.m_hFile == other.m_hFile
            && self.m_dwLinesOffset == other.m_dwLinesOffset
            && self.m_dwHeaderLength == other.m_dwHeaderLength
    }
}
impl ::core::cmp::Eq for FIO_CONTEXT {}
impl FromIntoMemory for FIO_CONTEXT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_m_dwTempHack = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_m_dwSignature = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_m_hFile =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_m_dwLinesOffset = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_m_dwHeaderLength = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            m_dwTempHack: f_m_dwTempHack,
            m_dwSignature: f_m_dwSignature,
            m_hFile: f_m_hFile,
            m_dwLinesOffset: f_m_dwLinesOffset,
            m_dwHeaderLength: f_m_dwHeaderLength,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.m_dwTempHack, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.m_dwSignature, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.m_hFile, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.m_dwLinesOffset, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.m_dwHeaderLength, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FindChangeNotificationHandle(pub PtrDiffRepr);
impl FindChangeNotificationHandle {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FindChangeNotificationHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for FindChangeNotificationHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for FindChangeNotificationHandle {}
impl ::core::hash::Hash for FindChangeNotificationHandle {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for FindChangeNotificationHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindChangeNotificationHandle")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for FindChangeNotificationHandle {
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
pub struct FindFileHandle(pub PtrDiffRepr);
impl FindFileHandle {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FindFileHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for FindFileHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for FindFileHandle {}
impl ::core::hash::Hash for FindFileHandle {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for FindFileHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindFileHandle").field(&self.0).finish()
    }
}
impl FromIntoMemory for FindFileHandle {
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
pub struct FindFileNameHandle(pub PtrDiffRepr);
impl FindFileNameHandle {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FindFileNameHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for FindFileNameHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for FindFileNameHandle {}
impl ::core::hash::Hash for FindFileNameHandle {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for FindFileNameHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindFileNameHandle").field(&self.0).finish()
    }
}
impl FromIntoMemory for FindFileNameHandle {
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
pub struct FindStreamHandle(pub PtrDiffRepr);
impl FindStreamHandle {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FindStreamHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for FindStreamHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for FindStreamHandle {}
impl ::core::hash::Hash for FindStreamHandle {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for FindStreamHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindStreamHandle").field(&self.0).finish()
    }
}
impl FromIntoMemory for FindStreamHandle {
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
pub struct FindVolumeHandle(pub PtrDiffRepr);
impl FindVolumeHandle {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FindVolumeHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for FindVolumeHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for FindVolumeHandle {}
impl ::core::hash::Hash for FindVolumeHandle {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for FindVolumeHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindVolumeHandle").field(&self.0).finish()
    }
}
impl FromIntoMemory for FindVolumeHandle {
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
pub struct FindVolumeMointPointHandle(pub PtrDiffRepr);
impl FindVolumeMointPointHandle {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FindVolumeMointPointHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for FindVolumeMointPointHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for FindVolumeMointPointHandle {}
impl ::core::hash::Hash for FindVolumeMointPointHandle {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for FindVolumeMointPointHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindVolumeMointPointHandle")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for FindVolumeMointPointHandle {
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
pub struct GET_FILEEX_INFO_LEVELS(pub i32);
pub const GetFileExInfoStandard: GET_FILEEX_INFO_LEVELS = GET_FILEEX_INFO_LEVELS(0i32);
pub const GetFileExMaxInfoLevel: GET_FILEEX_INFO_LEVELS = GET_FILEEX_INFO_LEVELS(1i32);
impl ::core::marker::Copy for GET_FILEEX_INFO_LEVELS {}
impl ::core::clone::Clone for GET_FILEEX_INFO_LEVELS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_FILEEX_INFO_LEVELS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_FILEEX_INFO_LEVELS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_FILEEX_INFO_LEVELS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for GET_FILEEX_INFO_LEVELS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GET_FILE_VERSION_INFO_FLAGS(pub u32);
pub const FILE_VER_GET_LOCALISED: GET_FILE_VERSION_INFO_FLAGS = GET_FILE_VERSION_INFO_FLAGS(1u32);
pub const FILE_VER_GET_NEUTRAL: GET_FILE_VERSION_INFO_FLAGS = GET_FILE_VERSION_INFO_FLAGS(2u32);
pub const FILE_VER_GET_PREFETCHED: GET_FILE_VERSION_INFO_FLAGS = GET_FILE_VERSION_INFO_FLAGS(4u32);
impl ::core::marker::Copy for GET_FILE_VERSION_INFO_FLAGS {}
impl ::core::clone::Clone for GET_FILE_VERSION_INFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_FILE_VERSION_INFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_FILE_VERSION_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_FILE_VERSION_INFO_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for GET_FILE_VERSION_INFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GET_FILE_VERSION_INFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GET_FILE_VERSION_INFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GET_FILE_VERSION_INFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GET_FILE_VERSION_INFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for GET_FILE_VERSION_INFO_FLAGS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GET_TAPE_DRIVE_PARAMETERS_OPERATION(pub u32);
pub const GET_TAPE_DRIVE_INFORMATION: GET_TAPE_DRIVE_PARAMETERS_OPERATION =
    GET_TAPE_DRIVE_PARAMETERS_OPERATION(1u32);
pub const GET_TAPE_MEDIA_INFORMATION: GET_TAPE_DRIVE_PARAMETERS_OPERATION =
    GET_TAPE_DRIVE_PARAMETERS_OPERATION(0u32);
impl ::core::marker::Copy for GET_TAPE_DRIVE_PARAMETERS_OPERATION {}
impl ::core::clone::Clone for GET_TAPE_DRIVE_PARAMETERS_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_TAPE_DRIVE_PARAMETERS_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_TAPE_DRIVE_PARAMETERS_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_TAPE_DRIVE_PARAMETERS_OPERATION")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for GET_TAPE_DRIVE_PARAMETERS_OPERATION {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct HIORING__ {
    pub unused: i32,
}
impl ::core::marker::Copy for HIORING__ {}
impl ::core::clone::Clone for HIORING__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HIORING__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HIORING__")
            .field("unused", &self.unused)
            .finish()
    }
}
impl ::core::cmp::PartialEq for HIORING__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for HIORING__ {}
impl FromIntoMemory for HIORING__ {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_unused = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self { unused: f_unused }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.unused, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IDiskQuotaControl(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IDiskQuotaControl_Trait:
    super::super::System::Com::IConnectionPointContainer_Trait
{
    fn Initialize(
        &self,
        psz_path: PCWSTR,
        b_read_write: super::super::Foundation::BOOL,
    ) -> crate::core::HRESULT {
        todo!("Initialize")
    }
    fn SetQuotaState(&self, dw_state: u32) -> crate::core::HRESULT {
        todo!("SetQuotaState")
    }
    fn GetQuotaState(&self, pdw_state: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetQuotaState")
    }
    fn SetQuotaLogFlags(&self, dw_flags: u32) -> crate::core::HRESULT {
        todo!("SetQuotaLogFlags")
    }
    fn GetQuotaLogFlags(&self, pdw_flags: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetQuotaLogFlags")
    }
    fn SetDefaultQuotaThreshold(&self, ll_threshold: i64) -> crate::core::HRESULT {
        todo!("SetDefaultQuotaThreshold")
    }
    fn GetDefaultQuotaThreshold(&self, pll_threshold: MutPtr<i64>) -> crate::core::HRESULT {
        todo!("GetDefaultQuotaThreshold")
    }
    fn GetDefaultQuotaThresholdText(
        &self,
        psz_text: PCWSTR,
        cch_text: u32,
    ) -> crate::core::HRESULT {
        todo!("GetDefaultQuotaThresholdText")
    }
    fn SetDefaultQuotaLimit(&self, ll_limit: i64) -> crate::core::HRESULT {
        todo!("SetDefaultQuotaLimit")
    }
    fn GetDefaultQuotaLimit(&self, pll_limit: MutPtr<i64>) -> crate::core::HRESULT {
        todo!("GetDefaultQuotaLimit")
    }
    fn GetDefaultQuotaLimitText(&self, psz_text: PCWSTR, cch_text: u32) -> crate::core::HRESULT {
        todo!("GetDefaultQuotaLimitText")
    }
    fn AddUserSid(
        &self,
        p_user_sid: super::super::Foundation::PSID,
        f_name_resolution: DISKQUOTA_USERNAME_RESOLVE,
        pp_user: MutPtr<IDiskQuotaUser>,
    ) -> crate::core::HRESULT {
        todo!("AddUserSid")
    }
    fn AddUserName(
        &self,
        psz_logon_name: PCWSTR,
        f_name_resolution: DISKQUOTA_USERNAME_RESOLVE,
        pp_user: MutPtr<IDiskQuotaUser>,
    ) -> crate::core::HRESULT {
        todo!("AddUserName")
    }
    fn DeleteUser(&self, p_user: IDiskQuotaUser) -> crate::core::HRESULT {
        todo!("DeleteUser")
    }
    fn FindUserSid(
        &self,
        p_user_sid: super::super::Foundation::PSID,
        f_name_resolution: DISKQUOTA_USERNAME_RESOLVE,
        pp_user: MutPtr<IDiskQuotaUser>,
    ) -> crate::core::HRESULT {
        todo!("FindUserSid")
    }
    fn FindUserName(
        &self,
        psz_logon_name: PCWSTR,
        pp_user: MutPtr<IDiskQuotaUser>,
    ) -> crate::core::HRESULT {
        todo!("FindUserName")
    }
    fn CreateEnumUsers(
        &self,
        rgp_user_sids: MutPtr<super::super::Foundation::PSID>,
        cp_sids: u32,
        f_name_resolution: DISKQUOTA_USERNAME_RESOLVE,
        pp_enum: MutPtr<IEnumDiskQuotaUsers>,
    ) -> crate::core::HRESULT {
        todo!("CreateEnumUsers")
    }
    fn CreateUserBatch(&self, pp_batch: MutPtr<IDiskQuotaUserBatch>) -> crate::core::HRESULT {
        todo!("CreateUserBatch")
    }
    fn InvalidateSidNameCache(&self) -> crate::core::HRESULT {
        todo!("InvalidateSidNameCache")
    }
    fn GiveUserNameResolutionPriority(&self, p_user: IDiskQuotaUser) -> crate::core::HRESULT {
        todo!("GiveUserNameResolutionPriority")
    }
    fn ShutdownNameResolution(&self) -> crate::core::HRESULT {
        todo!("ShutdownNameResolution")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IDiskQuotaControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IDiskQuotaControl {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IDiskQuotaControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IDiskQuotaControl {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IDiskQuotaControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDiskQuotaControl").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IDiskQuotaControl {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        <crate::core::IUnknown as FromIntoMemory>::size()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IDiskQuotaControl {
    type Super = super::super::System::Com::IConnectionPointContainer;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x7988b572_ec89_11cf_9c00_00aa00a14f56);
}
pub struct IDiskQuotaEvents(pub crate::core::IUnknown);
pub trait IDiskQuotaEvents_Trait: crate::core::IUnknown_Trait {
    fn OnUserNameChanged(&self, p_user: IDiskQuotaUser) -> crate::core::HRESULT {
        todo!("OnUserNameChanged")
    }
}
impl ::core::clone::Clone for IDiskQuotaEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IDiskQuotaEvents {}
impl ::core::cmp::PartialEq for IDiskQuotaEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDiskQuotaEvents {}
impl ::core::fmt::Debug for IDiskQuotaEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDiskQuotaEvents").field(&self.0).finish()
    }
}
impl FromIntoMemory for IDiskQuotaEvents {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        <crate::core::IUnknown as FromIntoMemory>::size()
    }
}
impl crate::core::ComInterface for IDiskQuotaEvents {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x7988b579_ec89_11cf_9c00_00aa00a14f56);
}
pub struct IDiskQuotaUser(pub crate::core::IUnknown);
pub trait IDiskQuotaUser_Trait: crate::core::IUnknown_Trait {
    fn GetID(&self, pul_id: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetID")
    }
    fn GetName(
        &self,
        psz_account_container: PCWSTR,
        cch_account_container: u32,
        psz_logon_name: PCWSTR,
        cch_logon_name: u32,
        psz_display_name: PCWSTR,
        cch_display_name: u32,
    ) -> crate::core::HRESULT {
        todo!("GetName")
    }
    fn GetSidLength(&self, pdw_length: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetSidLength")
    }
    fn GetSid(&self, pb_sid_buffer: MutPtr<u8>, cb_sid_buffer: u32) -> crate::core::HRESULT {
        todo!("GetSid")
    }
    fn GetQuotaThreshold(&self, pll_threshold: MutPtr<i64>) -> crate::core::HRESULT {
        todo!("GetQuotaThreshold")
    }
    fn GetQuotaThresholdText(&self, psz_text: PCWSTR, cch_text: u32) -> crate::core::HRESULT {
        todo!("GetQuotaThresholdText")
    }
    fn GetQuotaLimit(&self, pll_limit: MutPtr<i64>) -> crate::core::HRESULT {
        todo!("GetQuotaLimit")
    }
    fn GetQuotaLimitText(&self, psz_text: PCWSTR, cch_text: u32) -> crate::core::HRESULT {
        todo!("GetQuotaLimitText")
    }
    fn GetQuotaUsed(&self, pll_used: MutPtr<i64>) -> crate::core::HRESULT {
        todo!("GetQuotaUsed")
    }
    fn GetQuotaUsedText(&self, psz_text: PCWSTR, cch_text: u32) -> crate::core::HRESULT {
        todo!("GetQuotaUsedText")
    }
    fn GetQuotaInformation(
        &self,
        pb_quota_info: MutPtr<::core::ffi::c_void>,
        cb_quota_info: u32,
    ) -> crate::core::HRESULT {
        todo!("GetQuotaInformation")
    }
    fn SetQuotaThreshold(
        &self,
        ll_threshold: i64,
        f_write_through: super::super::Foundation::BOOL,
    ) -> crate::core::HRESULT {
        todo!("SetQuotaThreshold")
    }
    fn SetQuotaLimit(
        &self,
        ll_limit: i64,
        f_write_through: super::super::Foundation::BOOL,
    ) -> crate::core::HRESULT {
        todo!("SetQuotaLimit")
    }
    fn Invalidate(&self) -> crate::core::HRESULT {
        todo!("Invalidate")
    }
    fn GetAccountStatus(&self, pdw_status: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetAccountStatus")
    }
}
impl ::core::clone::Clone for IDiskQuotaUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IDiskQuotaUser {}
impl ::core::cmp::PartialEq for IDiskQuotaUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDiskQuotaUser {}
impl ::core::fmt::Debug for IDiskQuotaUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDiskQuotaUser").field(&self.0).finish()
    }
}
impl FromIntoMemory for IDiskQuotaUser {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        <crate::core::IUnknown as FromIntoMemory>::size()
    }
}
impl crate::core::ComInterface for IDiskQuotaUser {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x7988b574_ec89_11cf_9c00_00aa00a14f56);
}
pub struct IDiskQuotaUserBatch(pub crate::core::IUnknown);
pub trait IDiskQuotaUserBatch_Trait: crate::core::IUnknown_Trait {
    fn Add(&self, p_user: IDiskQuotaUser) -> crate::core::HRESULT {
        todo!("Add")
    }
    fn Remove(&self, p_user: IDiskQuotaUser) -> crate::core::HRESULT {
        todo!("Remove")
    }
    fn RemoveAll(&self) -> crate::core::HRESULT {
        todo!("RemoveAll")
    }
    fn FlushToDisk(&self) -> crate::core::HRESULT {
        todo!("FlushToDisk")
    }
}
impl ::core::clone::Clone for IDiskQuotaUserBatch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IDiskQuotaUserBatch {}
impl ::core::cmp::PartialEq for IDiskQuotaUserBatch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDiskQuotaUserBatch {}
impl ::core::fmt::Debug for IDiskQuotaUserBatch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDiskQuotaUserBatch").field(&self.0).finish()
    }
}
impl FromIntoMemory for IDiskQuotaUserBatch {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        <crate::core::IUnknown as FromIntoMemory>::size()
    }
}
impl crate::core::ComInterface for IDiskQuotaUserBatch {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x7988b576_ec89_11cf_9c00_00aa00a14f56);
}
pub struct IEnumDiskQuotaUsers(pub crate::core::IUnknown);
pub trait IEnumDiskQuotaUsers_Trait: crate::core::IUnknown_Trait {
    fn Next(
        &self,
        c_users: u32,
        rg_users: MutPtr<IDiskQuotaUser>,
        pc_users_fetched: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("Next")
    }
    fn Skip(&self, c_users: u32) -> crate::core::HRESULT {
        todo!("Skip")
    }
    fn Reset(&self) -> crate::core::HRESULT {
        todo!("Reset")
    }
    fn Clone(&self, pp_enum: MutPtr<IEnumDiskQuotaUsers>) -> crate::core::HRESULT {
        todo!("Clone")
    }
}
impl ::core::clone::Clone for IEnumDiskQuotaUsers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IEnumDiskQuotaUsers {}
impl ::core::cmp::PartialEq for IEnumDiskQuotaUsers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDiskQuotaUsers {}
impl ::core::fmt::Debug for IEnumDiskQuotaUsers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDiskQuotaUsers").field(&self.0).finish()
    }
}
impl FromIntoMemory for IEnumDiskQuotaUsers {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        <crate::core::IUnknown as FromIntoMemory>::size()
    }
}
impl crate::core::ComInterface for IEnumDiskQuotaUsers {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x7988b577_ec89_11cf_9c00_00aa00a14f56);
}
pub const INVALID_FILE_ATTRIBUTES: u32 = 4294967295u32;
pub const INVALID_SET_FILE_POINTER: u32 = 4294967295u32;
pub const IOCTL_VOLUME_ALLOCATE_BC_STREAM: u32 = 5685312u32;
pub const IOCTL_VOLUME_BASE: u32 = 86u32;
pub const IOCTL_VOLUME_BC_VERSION: u32 = 1u32;
pub const IOCTL_VOLUME_FREE_BC_STREAM: u32 = 5685316u32;
pub const IOCTL_VOLUME_GET_BC_PROPERTIES: u32 = 5652540u32;
pub const IOCTL_VOLUME_GET_CSVBLOCKCACHE_CALLBACK: u32 = 5685352u32;
pub const IOCTL_VOLUME_GET_GPT_ATTRIBUTES: u32 = 5636152u32;
pub const IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS: u32 = 5636096u32;
pub const IOCTL_VOLUME_IS_CLUSTERED: u32 = 5636144u32;
pub const IOCTL_VOLUME_IS_CSV: u32 = 5636192u32;
pub const IOCTL_VOLUME_IS_DYNAMIC: u32 = 5636168u32;
pub const IOCTL_VOLUME_IS_IO_CAPABLE: u32 = 5636116u32;
pub const IOCTL_VOLUME_IS_OFFLINE: u32 = 5636112u32;
pub const IOCTL_VOLUME_IS_PARTITION: u32 = 5636136u32;
pub const IOCTL_VOLUME_LOGICAL_TO_PHYSICAL: u32 = 5636128u32;
pub const IOCTL_VOLUME_OFFLINE: u32 = 5685260u32;
pub const IOCTL_VOLUME_ONLINE: u32 = 5685256u32;
pub const IOCTL_VOLUME_PHYSICAL_TO_LOGICAL: u32 = 5636132u32;
pub const IOCTL_VOLUME_POST_ONLINE: u32 = 5685348u32;
pub const IOCTL_VOLUME_PREPARE_FOR_CRITICAL_IO: u32 = 5685324u32;
pub const IOCTL_VOLUME_PREPARE_FOR_SHRINK: u32 = 5685340u32;
pub const IOCTL_VOLUME_QUERY_ALLOCATION_HINT: u32 = 5652562u32;
pub const IOCTL_VOLUME_QUERY_FAILOVER_SET: u32 = 5636120u32;
pub const IOCTL_VOLUME_QUERY_MINIMUM_SHRINK_SIZE: u32 = 5652568u32;
pub const IOCTL_VOLUME_QUERY_VOLUME_NUMBER: u32 = 5636124u32;
pub const IOCTL_VOLUME_READ_PLEX: u32 = 5652526u32;
pub const IOCTL_VOLUME_SET_GPT_ATTRIBUTES: u32 = 5636148u32;
pub const IOCTL_VOLUME_SUPPORTS_ONLINE_OFFLINE: u32 = 5636100u32;
pub const IOCTL_VOLUME_UPDATE_PROPERTIES: u32 = 5636180u32;
pub struct IORING_BUFFER_INFO {
    pub Address: MutPtr<::core::ffi::c_void>,
    pub Length: u32,
}
impl ::core::marker::Copy for IORING_BUFFER_INFO {}
impl ::core::clone::Clone for IORING_BUFFER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IORING_BUFFER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IORING_BUFFER_INFO")
            .field("Address", &self.Address)
            .field("Length", &self.Length)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IORING_BUFFER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for IORING_BUFFER_INFO {}
impl FromIntoMemory for IORING_BUFFER_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Address =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Length = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            Address: f_Address,
            Length: f_Length,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Address, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Length, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct IORING_BUFFER_REF {
    pub Kind: IORING_REF_KIND,
    pub Buffer: IORING_BUFFER_REF_0,
}
impl ::core::marker::Copy for IORING_BUFFER_REF {}
impl ::core::clone::Clone for IORING_BUFFER_REF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for IORING_BUFFER_REF {
    fn eq(&self, other: &Self) -> bool {
        self.Kind == other.Kind && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for IORING_BUFFER_REF {}
impl FromIntoMemory for IORING_BUFFER_REF {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Kind = <IORING_REF_KIND as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Buffer = <IORING_BUFFER_REF_0 as FromIntoMemory>::from_bytes(&from[4..4 + 8]);
        Self {
            Kind: f_Kind,
            Buffer: f_Buffer,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Kind, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Buffer, &mut into[4..4 + 8]);
    }
    fn size() -> usize {
        12
    }
}
pub struct IORING_BUFFER_REF_0 {
    data: [u8; 8],
}
impl ::core::default::Default for IORING_BUFFER_REF_0 {
    fn default() -> Self {
        Self { data: [0u8; 8] }
    }
}
impl ::core::marker::Copy for IORING_BUFFER_REF_0 {}
impl ::core::clone::Clone for IORING_BUFFER_REF_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for IORING_BUFFER_REF_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for IORING_BUFFER_REF_0 {}
impl FromIntoMemory for IORING_BUFFER_REF_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 8];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        8
    }
}
pub struct IORING_CAPABILITIES {
    pub MaxVersion: IORING_VERSION,
    pub MaxSubmissionQueueSize: u32,
    pub MaxCompletionQueueSize: u32,
    pub FeatureFlags: IORING_FEATURE_FLAGS,
}
impl ::core::marker::Copy for IORING_CAPABILITIES {}
impl ::core::clone::Clone for IORING_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IORING_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IORING_CAPABILITIES")
            .field("MaxVersion", &self.MaxVersion)
            .field("MaxSubmissionQueueSize", &self.MaxSubmissionQueueSize)
            .field("MaxCompletionQueueSize", &self.MaxCompletionQueueSize)
            .field("FeatureFlags", &self.FeatureFlags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IORING_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.MaxVersion == other.MaxVersion
            && self.MaxSubmissionQueueSize == other.MaxSubmissionQueueSize
            && self.MaxCompletionQueueSize == other.MaxCompletionQueueSize
            && self.FeatureFlags == other.FeatureFlags
    }
}
impl ::core::cmp::Eq for IORING_CAPABILITIES {}
impl FromIntoMemory for IORING_CAPABILITIES {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_MaxVersion = <IORING_VERSION as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_MaxSubmissionQueueSize = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_MaxCompletionQueueSize = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_FeatureFlags =
            <IORING_FEATURE_FLAGS as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            MaxVersion: f_MaxVersion,
            MaxSubmissionQueueSize: f_MaxSubmissionQueueSize,
            MaxCompletionQueueSize: f_MaxCompletionQueueSize,
            FeatureFlags: f_FeatureFlags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.MaxVersion, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.MaxSubmissionQueueSize, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.MaxCompletionQueueSize, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.FeatureFlags, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct IORING_CQE {
    pub UserData: PtrRepr,
    pub ResultCode: crate::core::HRESULT,
    pub Information: PtrRepr,
}
impl ::core::marker::Copy for IORING_CQE {}
impl ::core::clone::Clone for IORING_CQE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IORING_CQE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IORING_CQE")
            .field("UserData", &self.UserData)
            .field("ResultCode", &self.ResultCode)
            .field("Information", &self.Information)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IORING_CQE {
    fn eq(&self, other: &Self) -> bool {
        self.UserData == other.UserData
            && self.ResultCode == other.ResultCode
            && self.Information == other.Information
    }
}
impl ::core::cmp::Eq for IORING_CQE {}
impl FromIntoMemory for IORING_CQE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_UserData = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ResultCode = <crate::core::HRESULT as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Information = <PtrRepr as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            UserData: f_UserData,
            ResultCode: f_ResultCode,
            Information: f_Information,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.UserData, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ResultCode, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Information, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IORING_CREATE_ADVISORY_FLAGS(pub i32);
pub const IORING_CREATE_ADVISORY_FLAGS_NONE: IORING_CREATE_ADVISORY_FLAGS =
    IORING_CREATE_ADVISORY_FLAGS(0i32);
impl ::core::marker::Copy for IORING_CREATE_ADVISORY_FLAGS {}
impl ::core::clone::Clone for IORING_CREATE_ADVISORY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IORING_CREATE_ADVISORY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IORING_CREATE_ADVISORY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IORING_CREATE_ADVISORY_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IORING_CREATE_ADVISORY_FLAGS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct IORING_CREATE_FLAGS {
    pub Required: IORING_CREATE_REQUIRED_FLAGS,
    pub Advisory: IORING_CREATE_ADVISORY_FLAGS,
}
impl ::core::marker::Copy for IORING_CREATE_FLAGS {}
impl ::core::clone::Clone for IORING_CREATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IORING_CREATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IORING_CREATE_FLAGS")
            .field("Required", &self.Required)
            .field("Advisory", &self.Advisory)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IORING_CREATE_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self.Required == other.Required && self.Advisory == other.Advisory
    }
}
impl ::core::cmp::Eq for IORING_CREATE_FLAGS {}
impl FromIntoMemory for IORING_CREATE_FLAGS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Required =
            <IORING_CREATE_REQUIRED_FLAGS as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Advisory =
            <IORING_CREATE_ADVISORY_FLAGS as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            Required: f_Required,
            Advisory: f_Advisory,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Required, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Advisory, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IORING_CREATE_REQUIRED_FLAGS(pub i32);
pub const IORING_CREATE_REQUIRED_FLAGS_NONE: IORING_CREATE_REQUIRED_FLAGS =
    IORING_CREATE_REQUIRED_FLAGS(0i32);
impl ::core::marker::Copy for IORING_CREATE_REQUIRED_FLAGS {}
impl ::core::clone::Clone for IORING_CREATE_REQUIRED_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IORING_CREATE_REQUIRED_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IORING_CREATE_REQUIRED_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IORING_CREATE_REQUIRED_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IORING_CREATE_REQUIRED_FLAGS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IORING_FEATURE_FLAGS(pub i32);
pub const IORING_FEATURE_FLAGS_NONE: IORING_FEATURE_FLAGS = IORING_FEATURE_FLAGS(0i32);
pub const IORING_FEATURE_UM_EMULATION: IORING_FEATURE_FLAGS = IORING_FEATURE_FLAGS(1i32);
pub const IORING_FEATURE_SET_COMPLETION_EVENT: IORING_FEATURE_FLAGS = IORING_FEATURE_FLAGS(2i32);
impl ::core::marker::Copy for IORING_FEATURE_FLAGS {}
impl ::core::clone::Clone for IORING_FEATURE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IORING_FEATURE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IORING_FEATURE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IORING_FEATURE_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IORING_FEATURE_FLAGS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct IORING_HANDLE_REF {
    pub Kind: IORING_REF_KIND,
    pub Handle: IORING_HANDLE_REF_0,
}
impl ::core::marker::Copy for IORING_HANDLE_REF {}
impl ::core::clone::Clone for IORING_HANDLE_REF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for IORING_HANDLE_REF {
    fn eq(&self, other: &Self) -> bool {
        self.Kind == other.Kind && self.Handle == other.Handle
    }
}
impl ::core::cmp::Eq for IORING_HANDLE_REF {}
impl FromIntoMemory for IORING_HANDLE_REF {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Kind = <IORING_REF_KIND as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Handle = <IORING_HANDLE_REF_0 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            Kind: f_Kind,
            Handle: f_Handle,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Kind, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Handle, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct IORING_HANDLE_REF_0 {
    data: [u8; 4],
}
impl ::core::default::Default for IORING_HANDLE_REF_0 {
    fn default() -> Self {
        Self { data: [0u8; 4] }
    }
}
impl ::core::marker::Copy for IORING_HANDLE_REF_0 {}
impl ::core::clone::Clone for IORING_HANDLE_REF_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for IORING_HANDLE_REF_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for IORING_HANDLE_REF_0 {}
impl FromIntoMemory for IORING_HANDLE_REF_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 4];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        4
    }
}
pub struct IORING_INFO {
    pub IoRingVersion: IORING_VERSION,
    pub Flags: IORING_CREATE_FLAGS,
    pub SubmissionQueueSize: u32,
    pub CompletionQueueSize: u32,
}
impl ::core::marker::Copy for IORING_INFO {}
impl ::core::clone::Clone for IORING_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IORING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IORING_INFO")
            .field("IoRingVersion", &self.IoRingVersion)
            .field("Flags", &self.Flags)
            .field("SubmissionQueueSize", &self.SubmissionQueueSize)
            .field("CompletionQueueSize", &self.CompletionQueueSize)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IORING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.IoRingVersion == other.IoRingVersion
            && self.Flags == other.Flags
            && self.SubmissionQueueSize == other.SubmissionQueueSize
            && self.CompletionQueueSize == other.CompletionQueueSize
    }
}
impl ::core::cmp::Eq for IORING_INFO {}
impl FromIntoMemory for IORING_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_IoRingVersion = <IORING_VERSION as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Flags = <IORING_CREATE_FLAGS as FromIntoMemory>::from_bytes(&from[4..4 + 8]);
        let f_SubmissionQueueSize = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_CompletionQueueSize = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            IoRingVersion: f_IoRingVersion,
            Flags: f_Flags,
            SubmissionQueueSize: f_SubmissionQueueSize,
            CompletionQueueSize: f_CompletionQueueSize,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.IoRingVersion, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[4..4 + 8]);
        FromIntoMemory::into_bytes(self.SubmissionQueueSize, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.CompletionQueueSize, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IORING_OP_CODE(pub i32);
pub const IORING_OP_NOP: IORING_OP_CODE = IORING_OP_CODE(0i32);
pub const IORING_OP_READ: IORING_OP_CODE = IORING_OP_CODE(1i32);
pub const IORING_OP_REGISTER_FILES: IORING_OP_CODE = IORING_OP_CODE(2i32);
pub const IORING_OP_REGISTER_BUFFERS: IORING_OP_CODE = IORING_OP_CODE(3i32);
pub const IORING_OP_CANCEL: IORING_OP_CODE = IORING_OP_CODE(4i32);
impl ::core::marker::Copy for IORING_OP_CODE {}
impl ::core::clone::Clone for IORING_OP_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IORING_OP_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IORING_OP_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IORING_OP_CODE").field(&self.0).finish()
    }
}
impl FromIntoMemory for IORING_OP_CODE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IORING_REF_KIND(pub i32);
pub const IORING_REF_RAW: IORING_REF_KIND = IORING_REF_KIND(0i32);
pub const IORING_REF_REGISTERED: IORING_REF_KIND = IORING_REF_KIND(1i32);
impl ::core::marker::Copy for IORING_REF_KIND {}
impl ::core::clone::Clone for IORING_REF_KIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IORING_REF_KIND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IORING_REF_KIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IORING_REF_KIND").field(&self.0).finish()
    }
}
impl FromIntoMemory for IORING_REF_KIND {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct IORING_REGISTERED_BUFFER {
    pub BufferIndex: u32,
    pub Offset: u32,
}
impl ::core::marker::Copy for IORING_REGISTERED_BUFFER {}
impl ::core::clone::Clone for IORING_REGISTERED_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IORING_REGISTERED_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IORING_REGISTERED_BUFFER")
            .field("BufferIndex", &self.BufferIndex)
            .field("Offset", &self.Offset)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IORING_REGISTERED_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.BufferIndex == other.BufferIndex && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for IORING_REGISTERED_BUFFER {}
impl FromIntoMemory for IORING_REGISTERED_BUFFER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_BufferIndex = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Offset = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            BufferIndex: f_BufferIndex,
            Offset: f_Offset,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.BufferIndex, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Offset, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IORING_SQE_FLAGS(pub i32);
pub const IOSQE_FLAGS_NONE: IORING_SQE_FLAGS = IORING_SQE_FLAGS(0i32);
impl ::core::marker::Copy for IORING_SQE_FLAGS {}
impl ::core::clone::Clone for IORING_SQE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IORING_SQE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IORING_SQE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IORING_SQE_FLAGS").field(&self.0).finish()
    }
}
impl FromIntoMemory for IORING_SQE_FLAGS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IORING_VERSION(pub i32);
pub const IORING_VERSION_INVALID: IORING_VERSION = IORING_VERSION(0i32);
pub const IORING_VERSION_1: IORING_VERSION = IORING_VERSION(1i32);
impl ::core::marker::Copy for IORING_VERSION {}
impl ::core::clone::Clone for IORING_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IORING_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IORING_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IORING_VERSION").field(&self.0).finish()
    }
}
impl FromIntoMemory for IORING_VERSION {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct KCRM_MARSHAL_HEADER {
    pub VersionMajor: u32,
    pub VersionMinor: u32,
    pub NumProtocols: u32,
    pub Unused: u32,
}
impl ::core::marker::Copy for KCRM_MARSHAL_HEADER {}
impl ::core::clone::Clone for KCRM_MARSHAL_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KCRM_MARSHAL_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KCRM_MARSHAL_HEADER")
            .field("VersionMajor", &self.VersionMajor)
            .field("VersionMinor", &self.VersionMinor)
            .field("NumProtocols", &self.NumProtocols)
            .field("Unused", &self.Unused)
            .finish()
    }
}
impl ::core::cmp::PartialEq for KCRM_MARSHAL_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.VersionMajor == other.VersionMajor
            && self.VersionMinor == other.VersionMinor
            && self.NumProtocols == other.NumProtocols
            && self.Unused == other.Unused
    }
}
impl ::core::cmp::Eq for KCRM_MARSHAL_HEADER {}
impl FromIntoMemory for KCRM_MARSHAL_HEADER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_VersionMajor = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_VersionMinor = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_NumProtocols = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_Unused = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            VersionMajor: f_VersionMajor,
            VersionMinor: f_VersionMinor,
            NumProtocols: f_NumProtocols,
            Unused: f_Unused,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.VersionMajor, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.VersionMinor, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.NumProtocols, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.Unused, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct KCRM_PROTOCOL_BLOB {
    pub ProtocolId: crate::core::GUID,
    pub StaticInfoLength: u32,
    pub TransactionIdInfoLength: u32,
    pub Unused1: u32,
    pub Unused2: u32,
}
impl ::core::marker::Copy for KCRM_PROTOCOL_BLOB {}
impl ::core::clone::Clone for KCRM_PROTOCOL_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KCRM_PROTOCOL_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KCRM_PROTOCOL_BLOB")
            .field("ProtocolId", &self.ProtocolId)
            .field("StaticInfoLength", &self.StaticInfoLength)
            .field("TransactionIdInfoLength", &self.TransactionIdInfoLength)
            .field("Unused1", &self.Unused1)
            .field("Unused2", &self.Unused2)
            .finish()
    }
}
impl ::core::cmp::PartialEq for KCRM_PROTOCOL_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.ProtocolId == other.ProtocolId
            && self.StaticInfoLength == other.StaticInfoLength
            && self.TransactionIdInfoLength == other.TransactionIdInfoLength
            && self.Unused1 == other.Unused1
            && self.Unused2 == other.Unused2
    }
}
impl ::core::cmp::Eq for KCRM_PROTOCOL_BLOB {}
impl FromIntoMemory for KCRM_PROTOCOL_BLOB {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_ProtocolId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_StaticInfoLength = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_TransactionIdInfoLength = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_Unused1 = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_Unused2 = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            ProtocolId: f_ProtocolId,
            StaticInfoLength: f_StaticInfoLength,
            TransactionIdInfoLength: f_TransactionIdInfoLength,
            Unused1: f_Unused1,
            Unused2: f_Unused2,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.ProtocolId, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.StaticInfoLength, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.TransactionIdInfoLength, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.Unused1, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.Unused2, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
pub struct KCRM_TRANSACTION_BLOB {
    pub UOW: crate::core::GUID,
    pub TmIdentity: crate::core::GUID,
    pub IsolationLevel: u32,
    pub IsolationFlags: u32,
    pub Timeout: u32,
    pub Description: [u16; 64],
}
impl ::core::marker::Copy for KCRM_TRANSACTION_BLOB {}
impl ::core::clone::Clone for KCRM_TRANSACTION_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KCRM_TRANSACTION_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KCRM_TRANSACTION_BLOB")
            .field("UOW", &self.UOW)
            .field("TmIdentity", &self.TmIdentity)
            .field("IsolationLevel", &self.IsolationLevel)
            .field("IsolationFlags", &self.IsolationFlags)
            .field("Timeout", &self.Timeout)
            .field("Description", &self.Description)
            .finish()
    }
}
impl ::core::cmp::PartialEq for KCRM_TRANSACTION_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.UOW == other.UOW
            && self.TmIdentity == other.TmIdentity
            && self.IsolationLevel == other.IsolationLevel
            && self.IsolationFlags == other.IsolationFlags
            && self.Timeout == other.Timeout
            && self.Description == other.Description
    }
}
impl ::core::cmp::Eq for KCRM_TRANSACTION_BLOB {}
impl FromIntoMemory for KCRM_TRANSACTION_BLOB {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 108);
        let f_UOW = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_TmIdentity = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[16..16 + 16]);
        let f_IsolationLevel = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_IsolationFlags = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_Timeout = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_Description = <[u16; 64] as FromIntoMemory>::from_bytes(&from[44..44 + 64]);
        Self {
            UOW: f_UOW,
            TmIdentity: f_TmIdentity,
            IsolationLevel: f_IsolationLevel,
            IsolationFlags: f_IsolationFlags,
            Timeout: f_Timeout,
            Description: f_Description,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 108);
        FromIntoMemory::into_bytes(self.UOW, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.TmIdentity, &mut into[16..16 + 16]);
        FromIntoMemory::into_bytes(self.IsolationLevel, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.IsolationFlags, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.Timeout, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.Description, &mut into[44..44 + 64]);
    }
    fn size() -> usize {
        108
    }
}
pub const KTM_MARSHAL_BLOB_VERSION_MAJOR: u32 = 1u32;
pub const KTM_MARSHAL_BLOB_VERSION_MINOR: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LOCK_FILE_FLAGS(pub u32);
pub const LOCKFILE_EXCLUSIVE_LOCK: LOCK_FILE_FLAGS = LOCK_FILE_FLAGS(2u32);
pub const LOCKFILE_FAIL_IMMEDIATELY: LOCK_FILE_FLAGS = LOCK_FILE_FLAGS(1u32);
impl ::core::marker::Copy for LOCK_FILE_FLAGS {}
impl ::core::clone::Clone for LOCK_FILE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LOCK_FILE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LOCK_FILE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOCK_FILE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LOCK_FILE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LOCK_FILE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LOCK_FILE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LOCK_FILE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LOCK_FILE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for LOCK_FILE_FLAGS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct LOG_MANAGEMENT_CALLBACKS {
    pub CallbackContext: MutPtr<::core::ffi::c_void>,
    pub AdvanceTailCallback: PLOG_TAIL_ADVANCE_CALLBACK,
    pub LogFullHandlerCallback: PLOG_FULL_HANDLER_CALLBACK,
    pub LogUnpinnedCallback: PLOG_UNPINNED_CALLBACK,
}
impl ::core::marker::Copy for LOG_MANAGEMENT_CALLBACKS {}
impl ::core::clone::Clone for LOG_MANAGEMENT_CALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LOG_MANAGEMENT_CALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOG_MANAGEMENT_CALLBACKS")
            .field("CallbackContext", &self.CallbackContext)
            .field("AdvanceTailCallback", &self.AdvanceTailCallback)
            .field("LogFullHandlerCallback", &self.LogFullHandlerCallback)
            .field("LogUnpinnedCallback", &self.LogUnpinnedCallback)
            .finish()
    }
}
impl ::core::cmp::PartialEq for LOG_MANAGEMENT_CALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.CallbackContext == other.CallbackContext
            && self.AdvanceTailCallback == other.AdvanceTailCallback
            && self.LogFullHandlerCallback == other.LogFullHandlerCallback
            && self.LogUnpinnedCallback == other.LogUnpinnedCallback
    }
}
impl ::core::cmp::Eq for LOG_MANAGEMENT_CALLBACKS {}
impl FromIntoMemory for LOG_MANAGEMENT_CALLBACKS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_CallbackContext =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_AdvanceTailCallback =
            <PLOG_TAIL_ADVANCE_CALLBACK as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_LogFullHandlerCallback =
            <PLOG_FULL_HANDLER_CALLBACK as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_LogUnpinnedCallback =
            <PLOG_UNPINNED_CALLBACK as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            CallbackContext: f_CallbackContext,
            AdvanceTailCallback: f_AdvanceTailCallback,
            LogFullHandlerCallback: f_LogFullHandlerCallback,
            LogUnpinnedCallback: f_LogUnpinnedCallback,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.CallbackContext, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.AdvanceTailCallback, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.LogFullHandlerCallback, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.LogUnpinnedCallback, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub const LOG_POLICY_OVERWRITE: u32 = 1u32;
pub const LOG_POLICY_PERSIST: u32 = 2u32;
pub type LPPROGRESS_ROUTINE = StdCallFnPtr<
    (
        i64,
        i64,
        i64,
        i64,
        u32,
        LPPROGRESS_ROUTINE_CALLBACK_REASON,
        super::super::Foundation::HANDLE,
        super::super::Foundation::HANDLE,
        ConstPtr<::core::ffi::c_void>,
    ),
    u32,
>;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LPPROGRESS_ROUTINE_CALLBACK_REASON(pub u32);
pub const CALLBACK_CHUNK_FINISHED: LPPROGRESS_ROUTINE_CALLBACK_REASON =
    LPPROGRESS_ROUTINE_CALLBACK_REASON(0u32);
pub const CALLBACK_STREAM_SWITCH: LPPROGRESS_ROUTINE_CALLBACK_REASON =
    LPPROGRESS_ROUTINE_CALLBACK_REASON(1u32);
impl ::core::marker::Copy for LPPROGRESS_ROUTINE_CALLBACK_REASON {}
impl ::core::clone::Clone for LPPROGRESS_ROUTINE_CALLBACK_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LPPROGRESS_ROUTINE_CALLBACK_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LPPROGRESS_ROUTINE_CALLBACK_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LPPROGRESS_ROUTINE_CALLBACK_REASON")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for LPPROGRESS_ROUTINE_CALLBACK_REASON {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub const LZERROR_BADINHANDLE: i32 = -1i32;
pub const LZERROR_BADOUTHANDLE: i32 = -2i32;
pub const LZERROR_BADVALUE: i32 = -7i32;
pub const LZERROR_GLOBALLOC: i32 = -5i32;
pub const LZERROR_GLOBLOCK: i32 = -6i32;
pub const LZERROR_READ: i32 = -3i32;
pub const LZERROR_UNKNOWNALG: i32 = -8i32;
pub const LZERROR_WRITE: i32 = -4i32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LZOPENFILE_STYLE(pub u32);
pub const OF_CANCEL: LZOPENFILE_STYLE = LZOPENFILE_STYLE(2048u32);
pub const OF_CREATE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(4096u32);
pub const OF_DELETE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(512u32);
pub const OF_EXIST: LZOPENFILE_STYLE = LZOPENFILE_STYLE(16384u32);
pub const OF_PARSE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(256u32);
pub const OF_PROMPT: LZOPENFILE_STYLE = LZOPENFILE_STYLE(8192u32);
pub const OF_READ: LZOPENFILE_STYLE = LZOPENFILE_STYLE(0u32);
pub const OF_READWRITE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(2u32);
pub const OF_REOPEN: LZOPENFILE_STYLE = LZOPENFILE_STYLE(32768u32);
pub const OF_SHARE_DENY_NONE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(64u32);
pub const OF_SHARE_DENY_READ: LZOPENFILE_STYLE = LZOPENFILE_STYLE(48u32);
pub const OF_SHARE_DENY_WRITE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(32u32);
pub const OF_SHARE_EXCLUSIVE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(16u32);
pub const OF_WRITE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(1u32);
pub const OF_SHARE_COMPAT: LZOPENFILE_STYLE = LZOPENFILE_STYLE(0u32);
pub const OF_VERIFY: LZOPENFILE_STYLE = LZOPENFILE_STYLE(1024u32);
impl ::core::marker::Copy for LZOPENFILE_STYLE {}
impl ::core::clone::Clone for LZOPENFILE_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LZOPENFILE_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LZOPENFILE_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LZOPENFILE_STYLE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LZOPENFILE_STYLE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LZOPENFILE_STYLE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LZOPENFILE_STYLE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LZOPENFILE_STYLE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LZOPENFILE_STYLE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for LZOPENFILE_STYLE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub type MAXMEDIALABEL = StdCallFnPtr<(MutPtr<u32>,), u32>;
pub const MAX_RESOURCEMANAGER_DESCRIPTION_LENGTH: u32 = 64u32;
pub const MAX_SID_SIZE: u32 = 256u32;
pub const MAX_TRANSACTION_DESCRIPTION_LENGTH: u32 = 64u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MOVE_FILE_FLAGS(pub u32);
pub const MOVEFILE_COPY_ALLOWED: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(2u32);
pub const MOVEFILE_CREATE_HARDLINK: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(16u32);
pub const MOVEFILE_DELAY_UNTIL_REBOOT: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(4u32);
pub const MOVEFILE_REPLACE_EXISTING: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(1u32);
pub const MOVEFILE_WRITE_THROUGH: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(8u32);
pub const MOVEFILE_FAIL_IF_NOT_TRACKABLE: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(32u32);
impl ::core::marker::Copy for MOVE_FILE_FLAGS {}
impl ::core::clone::Clone for MOVE_FILE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MOVE_FILE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MOVE_FILE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MOVE_FILE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MOVE_FILE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MOVE_FILE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MOVE_FILE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MOVE_FILE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MOVE_FILE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for MOVE_FILE_FLAGS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct MediaLabelInfo {
    pub LabelType: [u16; 64],
    pub LabelIDSize: u32,
    pub LabelID: [u8; 256],
    pub LabelAppDescr: [u16; 256],
}
impl ::core::marker::Copy for MediaLabelInfo {}
impl ::core::clone::Clone for MediaLabelInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MediaLabelInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MediaLabelInfo")
            .field("LabelType", &self.LabelType)
            .field("LabelIDSize", &self.LabelIDSize)
            .field("LabelID", &self.LabelID)
            .field("LabelAppDescr", &self.LabelAppDescr)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MediaLabelInfo {
    fn eq(&self, other: &Self) -> bool {
        self.LabelType == other.LabelType
            && self.LabelIDSize == other.LabelIDSize
            && self.LabelID == other.LabelID
            && self.LabelAppDescr == other.LabelAppDescr
    }
}
impl ::core::cmp::Eq for MediaLabelInfo {}
impl FromIntoMemory for MediaLabelInfo {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 580);
        let f_LabelType = <[u16; 64] as FromIntoMemory>::from_bytes(&from[0..0 + 64]);
        let f_LabelIDSize = <u32 as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        let f_LabelID = <[u8; 256] as FromIntoMemory>::from_bytes(&from[68..68 + 256]);
        let f_LabelAppDescr = <[u16; 256] as FromIntoMemory>::from_bytes(&from[324..324 + 256]);
        Self {
            LabelType: f_LabelType,
            LabelIDSize: f_LabelIDSize,
            LabelID: f_LabelID,
            LabelAppDescr: f_LabelAppDescr,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 580);
        FromIntoMemory::into_bytes(self.LabelType, &mut into[0..0 + 64]);
        FromIntoMemory::into_bytes(self.LabelIDSize, &mut into[64..64 + 4]);
        FromIntoMemory::into_bytes(self.LabelID, &mut into[68..68 + 256]);
        FromIntoMemory::into_bytes(self.LabelAppDescr, &mut into[324..324 + 256]);
    }
    fn size() -> usize {
        580
    }
}
pub struct NAME_CACHE_CONTEXT {
    pub m_dwSignature: u32,
}
impl ::core::marker::Copy for NAME_CACHE_CONTEXT {}
impl ::core::clone::Clone for NAME_CACHE_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NAME_CACHE_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NAME_CACHE_CONTEXT")
            .field("m_dwSignature", &self.m_dwSignature)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NAME_CACHE_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.m_dwSignature == other.m_dwSignature
    }
}
impl ::core::cmp::Eq for NAME_CACHE_CONTEXT {}
impl FromIntoMemory for NAME_CACHE_CONTEXT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_m_dwSignature = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            m_dwSignature: f_m_dwSignature,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.m_dwSignature, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub const NTMSMLI_MAXAPPDESCR: u32 = 256u32;
pub const NTMSMLI_MAXIDSIZE: u32 = 256u32;
pub const NTMSMLI_MAXTYPE: u32 = 64u32;
pub struct NTMS_ALLOCATION_INFORMATION {
    pub dwSize: u32,
    pub lpReserved: MutPtr<::core::ffi::c_void>,
    pub AllocatedFrom: crate::core::GUID,
}
impl ::core::marker::Copy for NTMS_ALLOCATION_INFORMATION {}
impl ::core::clone::Clone for NTMS_ALLOCATION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_ALLOCATION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_ALLOCATION_INFORMATION")
            .field("dwSize", &self.dwSize)
            .field("lpReserved", &self.lpReserved)
            .field("AllocatedFrom", &self.AllocatedFrom)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_ALLOCATION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.lpReserved == other.lpReserved
            && self.AllocatedFrom == other.AllocatedFrom
    }
}
impl ::core::cmp::Eq for NTMS_ALLOCATION_INFORMATION {}
impl FromIntoMemory for NTMS_ALLOCATION_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpReserved =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_AllocatedFrom = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        Self {
            dwSize: f_dwSize,
            lpReserved: f_lpReserved,
            AllocatedFrom: f_AllocatedFrom,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpReserved, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.AllocatedFrom, &mut into[8..8 + 16]);
    }
    fn size() -> usize {
        24
    }
}
pub const NTMS_APPLICATIONNAME_LENGTH: u32 = 64u32;
pub struct NTMS_ASYNC_IO {
    pub OperationId: crate::core::GUID,
    pub EventId: crate::core::GUID,
    pub dwOperationType: u32,
    pub dwResult: u32,
    pub dwAsyncState: u32,
    pub hEvent: super::super::Foundation::HANDLE,
    pub bOnStateChange: super::super::Foundation::BOOL,
}
impl ::core::marker::Copy for NTMS_ASYNC_IO {}
impl ::core::clone::Clone for NTMS_ASYNC_IO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_ASYNC_IO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_ASYNC_IO")
            .field("OperationId", &self.OperationId)
            .field("EventId", &self.EventId)
            .field("dwOperationType", &self.dwOperationType)
            .field("dwResult", &self.dwResult)
            .field("dwAsyncState", &self.dwAsyncState)
            .field("hEvent", &self.hEvent)
            .field("bOnStateChange", &self.bOnStateChange)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_ASYNC_IO {
    fn eq(&self, other: &Self) -> bool {
        self.OperationId == other.OperationId
            && self.EventId == other.EventId
            && self.dwOperationType == other.dwOperationType
            && self.dwResult == other.dwResult
            && self.dwAsyncState == other.dwAsyncState
            && self.hEvent == other.hEvent
            && self.bOnStateChange == other.bOnStateChange
    }
}
impl ::core::cmp::Eq for NTMS_ASYNC_IO {}
impl FromIntoMemory for NTMS_ASYNC_IO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 52);
        let f_OperationId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_EventId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[16..16 + 16]);
        let f_dwOperationType = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_dwResult = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_dwAsyncState = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_hEvent =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_bOnStateChange =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        Self {
            OperationId: f_OperationId,
            EventId: f_EventId,
            dwOperationType: f_dwOperationType,
            dwResult: f_dwResult,
            dwAsyncState: f_dwAsyncState,
            hEvent: f_hEvent,
            bOnStateChange: f_bOnStateChange,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 52);
        FromIntoMemory::into_bytes(self.OperationId, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.EventId, &mut into[16..16 + 16]);
        FromIntoMemory::into_bytes(self.dwOperationType, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.dwResult, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.dwAsyncState, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.hEvent, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.bOnStateChange, &mut into[48..48 + 4]);
    }
    fn size() -> usize {
        52
    }
}
pub const NTMS_BARCODE_LENGTH: u32 = 64u32;
pub struct NTMS_CHANGERINFORMATIONA {
    pub Number: u32,
    pub ChangerType: crate::core::GUID,
    pub szSerialNumber: [super::super::Foundation::CHAR; 32],
    pub szRevision: [super::super::Foundation::CHAR; 32],
    pub szDeviceName: [super::super::Foundation::CHAR; 64],
    pub ScsiPort: u16,
    pub ScsiBus: u16,
    pub ScsiTarget: u16,
    pub ScsiLun: u16,
    pub Library: crate::core::GUID,
}
impl ::core::marker::Copy for NTMS_CHANGERINFORMATIONA {}
impl ::core::clone::Clone for NTMS_CHANGERINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_CHANGERINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_CHANGERINFORMATIONA")
            .field("Number", &self.Number)
            .field("ChangerType", &self.ChangerType)
            .field("szSerialNumber", &self.szSerialNumber)
            .field("szRevision", &self.szRevision)
            .field("szDeviceName", &self.szDeviceName)
            .field("ScsiPort", &self.ScsiPort)
            .field("ScsiBus", &self.ScsiBus)
            .field("ScsiTarget", &self.ScsiTarget)
            .field("ScsiLun", &self.ScsiLun)
            .field("Library", &self.Library)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_CHANGERINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number
            && self.ChangerType == other.ChangerType
            && self.szSerialNumber == other.szSerialNumber
            && self.szRevision == other.szRevision
            && self.szDeviceName == other.szDeviceName
            && self.ScsiPort == other.ScsiPort
            && self.ScsiBus == other.ScsiBus
            && self.ScsiTarget == other.ScsiTarget
            && self.ScsiLun == other.ScsiLun
            && self.Library == other.Library
    }
}
impl ::core::cmp::Eq for NTMS_CHANGERINFORMATIONA {}
impl FromIntoMemory for NTMS_CHANGERINFORMATIONA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 172);
        let f_Number = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ChangerType = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[4..4 + 16]);
        let f_szSerialNumber = <[super::super::Foundation::CHAR; 32] as FromIntoMemory>::from_bytes(
            &from[20..20 + 32],
        );
        let f_szRevision = <[super::super::Foundation::CHAR; 32] as FromIntoMemory>::from_bytes(
            &from[52..52 + 32],
        );
        let f_szDeviceName = <[super::super::Foundation::CHAR; 64] as FromIntoMemory>::from_bytes(
            &from[84..84 + 64],
        );
        let f_ScsiPort = <u16 as FromIntoMemory>::from_bytes(&from[148..148 + 2]);
        let f_ScsiBus = <u16 as FromIntoMemory>::from_bytes(&from[150..150 + 2]);
        let f_ScsiTarget = <u16 as FromIntoMemory>::from_bytes(&from[152..152 + 2]);
        let f_ScsiLun = <u16 as FromIntoMemory>::from_bytes(&from[154..154 + 2]);
        let f_Library = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[156..156 + 16]);
        Self {
            Number: f_Number,
            ChangerType: f_ChangerType,
            szSerialNumber: f_szSerialNumber,
            szRevision: f_szRevision,
            szDeviceName: f_szDeviceName,
            ScsiPort: f_ScsiPort,
            ScsiBus: f_ScsiBus,
            ScsiTarget: f_ScsiTarget,
            ScsiLun: f_ScsiLun,
            Library: f_Library,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 172);
        FromIntoMemory::into_bytes(self.Number, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ChangerType, &mut into[4..4 + 16]);
        FromIntoMemory::into_bytes(self.szSerialNumber, &mut into[20..20 + 32]);
        FromIntoMemory::into_bytes(self.szRevision, &mut into[52..52 + 32]);
        FromIntoMemory::into_bytes(self.szDeviceName, &mut into[84..84 + 64]);
        FromIntoMemory::into_bytes(self.ScsiPort, &mut into[148..148 + 2]);
        FromIntoMemory::into_bytes(self.ScsiBus, &mut into[150..150 + 2]);
        FromIntoMemory::into_bytes(self.ScsiTarget, &mut into[152..152 + 2]);
        FromIntoMemory::into_bytes(self.ScsiLun, &mut into[154..154 + 2]);
        FromIntoMemory::into_bytes(self.Library, &mut into[156..156 + 16]);
    }
    fn size() -> usize {
        172
    }
}
pub struct NTMS_CHANGERINFORMATIONW {
    pub Number: u32,
    pub ChangerType: crate::core::GUID,
    pub szSerialNumber: [u16; 32],
    pub szRevision: [u16; 32],
    pub szDeviceName: [u16; 64],
    pub ScsiPort: u16,
    pub ScsiBus: u16,
    pub ScsiTarget: u16,
    pub ScsiLun: u16,
    pub Library: crate::core::GUID,
}
impl ::core::marker::Copy for NTMS_CHANGERINFORMATIONW {}
impl ::core::clone::Clone for NTMS_CHANGERINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_CHANGERINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_CHANGERINFORMATIONW")
            .field("Number", &self.Number)
            .field("ChangerType", &self.ChangerType)
            .field("szSerialNumber", &self.szSerialNumber)
            .field("szRevision", &self.szRevision)
            .field("szDeviceName", &self.szDeviceName)
            .field("ScsiPort", &self.ScsiPort)
            .field("ScsiBus", &self.ScsiBus)
            .field("ScsiTarget", &self.ScsiTarget)
            .field("ScsiLun", &self.ScsiLun)
            .field("Library", &self.Library)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_CHANGERINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number
            && self.ChangerType == other.ChangerType
            && self.szSerialNumber == other.szSerialNumber
            && self.szRevision == other.szRevision
            && self.szDeviceName == other.szDeviceName
            && self.ScsiPort == other.ScsiPort
            && self.ScsiBus == other.ScsiBus
            && self.ScsiTarget == other.ScsiTarget
            && self.ScsiLun == other.ScsiLun
            && self.Library == other.Library
    }
}
impl ::core::cmp::Eq for NTMS_CHANGERINFORMATIONW {}
impl FromIntoMemory for NTMS_CHANGERINFORMATIONW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 172);
        let f_Number = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ChangerType = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[4..4 + 16]);
        let f_szSerialNumber = <[u16; 32] as FromIntoMemory>::from_bytes(&from[20..20 + 32]);
        let f_szRevision = <[u16; 32] as FromIntoMemory>::from_bytes(&from[52..52 + 32]);
        let f_szDeviceName = <[u16; 64] as FromIntoMemory>::from_bytes(&from[84..84 + 64]);
        let f_ScsiPort = <u16 as FromIntoMemory>::from_bytes(&from[148..148 + 2]);
        let f_ScsiBus = <u16 as FromIntoMemory>::from_bytes(&from[150..150 + 2]);
        let f_ScsiTarget = <u16 as FromIntoMemory>::from_bytes(&from[152..152 + 2]);
        let f_ScsiLun = <u16 as FromIntoMemory>::from_bytes(&from[154..154 + 2]);
        let f_Library = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[156..156 + 16]);
        Self {
            Number: f_Number,
            ChangerType: f_ChangerType,
            szSerialNumber: f_szSerialNumber,
            szRevision: f_szRevision,
            szDeviceName: f_szDeviceName,
            ScsiPort: f_ScsiPort,
            ScsiBus: f_ScsiBus,
            ScsiTarget: f_ScsiTarget,
            ScsiLun: f_ScsiLun,
            Library: f_Library,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 172);
        FromIntoMemory::into_bytes(self.Number, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ChangerType, &mut into[4..4 + 16]);
        FromIntoMemory::into_bytes(self.szSerialNumber, &mut into[20..20 + 32]);
        FromIntoMemory::into_bytes(self.szRevision, &mut into[52..52 + 32]);
        FromIntoMemory::into_bytes(self.szDeviceName, &mut into[84..84 + 64]);
        FromIntoMemory::into_bytes(self.ScsiPort, &mut into[148..148 + 2]);
        FromIntoMemory::into_bytes(self.ScsiBus, &mut into[150..150 + 2]);
        FromIntoMemory::into_bytes(self.ScsiTarget, &mut into[152..152 + 2]);
        FromIntoMemory::into_bytes(self.ScsiLun, &mut into[154..154 + 2]);
        FromIntoMemory::into_bytes(self.Library, &mut into[156..156 + 16]);
    }
    fn size() -> usize {
        172
    }
}
pub struct NTMS_CHANGERTYPEINFORMATIONA {
    pub szVendor: [super::super::Foundation::CHAR; 128],
    pub szProduct: [super::super::Foundation::CHAR; 128],
    pub DeviceType: u32,
}
impl ::core::marker::Copy for NTMS_CHANGERTYPEINFORMATIONA {}
impl ::core::clone::Clone for NTMS_CHANGERTYPEINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_CHANGERTYPEINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_CHANGERTYPEINFORMATIONA")
            .field("szVendor", &self.szVendor)
            .field("szProduct", &self.szProduct)
            .field("DeviceType", &self.DeviceType)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_CHANGERTYPEINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.szVendor == other.szVendor
            && self.szProduct == other.szProduct
            && self.DeviceType == other.DeviceType
    }
}
impl ::core::cmp::Eq for NTMS_CHANGERTYPEINFORMATIONA {}
impl FromIntoMemory for NTMS_CHANGERTYPEINFORMATIONA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 260);
        let f_szVendor = <[super::super::Foundation::CHAR; 128] as FromIntoMemory>::from_bytes(
            &from[0..0 + 128],
        );
        let f_szProduct = <[super::super::Foundation::CHAR; 128] as FromIntoMemory>::from_bytes(
            &from[128..128 + 128],
        );
        let f_DeviceType = <u32 as FromIntoMemory>::from_bytes(&from[256..256 + 4]);
        Self {
            szVendor: f_szVendor,
            szProduct: f_szProduct,
            DeviceType: f_DeviceType,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 260);
        FromIntoMemory::into_bytes(self.szVendor, &mut into[0..0 + 128]);
        FromIntoMemory::into_bytes(self.szProduct, &mut into[128..128 + 128]);
        FromIntoMemory::into_bytes(self.DeviceType, &mut into[256..256 + 4]);
    }
    fn size() -> usize {
        260
    }
}
pub struct NTMS_CHANGERTYPEINFORMATIONW {
    pub szVendor: [u16; 128],
    pub szProduct: [u16; 128],
    pub DeviceType: u32,
}
impl ::core::marker::Copy for NTMS_CHANGERTYPEINFORMATIONW {}
impl ::core::clone::Clone for NTMS_CHANGERTYPEINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_CHANGERTYPEINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_CHANGERTYPEINFORMATIONW")
            .field("szVendor", &self.szVendor)
            .field("szProduct", &self.szProduct)
            .field("DeviceType", &self.DeviceType)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_CHANGERTYPEINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.szVendor == other.szVendor
            && self.szProduct == other.szProduct
            && self.DeviceType == other.DeviceType
    }
}
impl ::core::cmp::Eq for NTMS_CHANGERTYPEINFORMATIONW {}
impl FromIntoMemory for NTMS_CHANGERTYPEINFORMATIONW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 260);
        let f_szVendor = <[u16; 128] as FromIntoMemory>::from_bytes(&from[0..0 + 128]);
        let f_szProduct = <[u16; 128] as FromIntoMemory>::from_bytes(&from[128..128 + 128]);
        let f_DeviceType = <u32 as FromIntoMemory>::from_bytes(&from[256..256 + 4]);
        Self {
            szVendor: f_szVendor,
            szProduct: f_szProduct,
            DeviceType: f_DeviceType,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 260);
        FromIntoMemory::into_bytes(self.szVendor, &mut into[0..0 + 128]);
        FromIntoMemory::into_bytes(self.szProduct, &mut into[128..128 + 128]);
        FromIntoMemory::into_bytes(self.DeviceType, &mut into[256..256 + 4]);
    }
    fn size() -> usize {
        260
    }
}
pub struct NTMS_COMPUTERINFORMATION {
    pub dwLibRequestPurgeTime: u32,
    pub dwOpRequestPurgeTime: u32,
    pub dwLibRequestFlags: u32,
    pub dwOpRequestFlags: u32,
    pub dwMediaPoolPolicy: u32,
}
impl ::core::marker::Copy for NTMS_COMPUTERINFORMATION {}
impl ::core::clone::Clone for NTMS_COMPUTERINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_COMPUTERINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_COMPUTERINFORMATION")
            .field("dwLibRequestPurgeTime", &self.dwLibRequestPurgeTime)
            .field("dwOpRequestPurgeTime", &self.dwOpRequestPurgeTime)
            .field("dwLibRequestFlags", &self.dwLibRequestFlags)
            .field("dwOpRequestFlags", &self.dwOpRequestFlags)
            .field("dwMediaPoolPolicy", &self.dwMediaPoolPolicy)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_COMPUTERINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwLibRequestPurgeTime == other.dwLibRequestPurgeTime
            && self.dwOpRequestPurgeTime == other.dwOpRequestPurgeTime
            && self.dwLibRequestFlags == other.dwLibRequestFlags
            && self.dwOpRequestFlags == other.dwOpRequestFlags
            && self.dwMediaPoolPolicy == other.dwMediaPoolPolicy
    }
}
impl ::core::cmp::Eq for NTMS_COMPUTERINFORMATION {}
impl FromIntoMemory for NTMS_COMPUTERINFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_dwLibRequestPurgeTime = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwOpRequestPurgeTime = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwLibRequestFlags = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwOpRequestFlags = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwMediaPoolPolicy = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            dwLibRequestPurgeTime: f_dwLibRequestPurgeTime,
            dwOpRequestPurgeTime: f_dwOpRequestPurgeTime,
            dwLibRequestFlags: f_dwLibRequestFlags,
            dwOpRequestFlags: f_dwOpRequestFlags,
            dwMediaPoolPolicy: f_dwMediaPoolPolicy,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.dwLibRequestPurgeTime, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwOpRequestPurgeTime, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwLibRequestFlags, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwOpRequestFlags, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwMediaPoolPolicy, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub const NTMS_COMPUTERNAME_LENGTH: u32 = 64u32;
pub const NTMS_DESCRIPTION_LENGTH: u32 = 127u32;
pub const NTMS_DEVICENAME_LENGTH: u32 = 64u32;
pub struct NTMS_DRIVEINFORMATIONA {
    pub Number: u32,
    pub State: NtmsDriveState,
    pub DriveType: crate::core::GUID,
    pub szDeviceName: [super::super::Foundation::CHAR; 64],
    pub szSerialNumber: [super::super::Foundation::CHAR; 32],
    pub szRevision: [super::super::Foundation::CHAR; 32],
    pub ScsiPort: u16,
    pub ScsiBus: u16,
    pub ScsiTarget: u16,
    pub ScsiLun: u16,
    pub dwMountCount: u32,
    pub LastCleanedTs: super::super::Foundation::SYSTEMTIME,
    pub SavedPartitionId: crate::core::GUID,
    pub Library: crate::core::GUID,
    pub Reserved: crate::core::GUID,
    pub dwDeferDismountDelay: u32,
}
impl ::core::marker::Copy for NTMS_DRIVEINFORMATIONA {}
impl ::core::clone::Clone for NTMS_DRIVEINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_DRIVEINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_DRIVEINFORMATIONA")
            .field("Number", &self.Number)
            .field("State", &self.State)
            .field("DriveType", &self.DriveType)
            .field("szDeviceName", &self.szDeviceName)
            .field("szSerialNumber", &self.szSerialNumber)
            .field("szRevision", &self.szRevision)
            .field("ScsiPort", &self.ScsiPort)
            .field("ScsiBus", &self.ScsiBus)
            .field("ScsiTarget", &self.ScsiTarget)
            .field("ScsiLun", &self.ScsiLun)
            .field("dwMountCount", &self.dwMountCount)
            .field("LastCleanedTs", &self.LastCleanedTs)
            .field("SavedPartitionId", &self.SavedPartitionId)
            .field("Library", &self.Library)
            .field("Reserved", &self.Reserved)
            .field("dwDeferDismountDelay", &self.dwDeferDismountDelay)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_DRIVEINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number
            && self.State == other.State
            && self.DriveType == other.DriveType
            && self.szDeviceName == other.szDeviceName
            && self.szSerialNumber == other.szSerialNumber
            && self.szRevision == other.szRevision
            && self.ScsiPort == other.ScsiPort
            && self.ScsiBus == other.ScsiBus
            && self.ScsiTarget == other.ScsiTarget
            && self.ScsiLun == other.ScsiLun
            && self.dwMountCount == other.dwMountCount
            && self.LastCleanedTs == other.LastCleanedTs
            && self.SavedPartitionId == other.SavedPartitionId
            && self.Library == other.Library
            && self.Reserved == other.Reserved
            && self.dwDeferDismountDelay == other.dwDeferDismountDelay
    }
}
impl ::core::cmp::Eq for NTMS_DRIVEINFORMATIONA {}
impl FromIntoMemory for NTMS_DRIVEINFORMATIONA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 232);
        let f_Number = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_State = <NtmsDriveState as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_DriveType = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        let f_szDeviceName = <[super::super::Foundation::CHAR; 64] as FromIntoMemory>::from_bytes(
            &from[24..24 + 64],
        );
        let f_szSerialNumber = <[super::super::Foundation::CHAR; 32] as FromIntoMemory>::from_bytes(
            &from[88..88 + 32],
        );
        let f_szRevision = <[super::super::Foundation::CHAR; 32] as FromIntoMemory>::from_bytes(
            &from[120..120 + 32],
        );
        let f_ScsiPort = <u16 as FromIntoMemory>::from_bytes(&from[152..152 + 2]);
        let f_ScsiBus = <u16 as FromIntoMemory>::from_bytes(&from[154..154 + 2]);
        let f_ScsiTarget = <u16 as FromIntoMemory>::from_bytes(&from[156..156 + 2]);
        let f_ScsiLun = <u16 as FromIntoMemory>::from_bytes(&from[158..158 + 2]);
        let f_dwMountCount = <u32 as FromIntoMemory>::from_bytes(&from[160..160 + 4]);
        let f_LastCleanedTs = <super::super::Foundation::SYSTEMTIME as FromIntoMemory>::from_bytes(
            &from[164..164 + 16],
        );
        let f_SavedPartitionId =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[180..180 + 16]);
        let f_Library = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[196..196 + 16]);
        let f_Reserved = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[212..212 + 16]);
        let f_dwDeferDismountDelay = <u32 as FromIntoMemory>::from_bytes(&from[228..228 + 4]);
        Self {
            Number: f_Number,
            State: f_State,
            DriveType: f_DriveType,
            szDeviceName: f_szDeviceName,
            szSerialNumber: f_szSerialNumber,
            szRevision: f_szRevision,
            ScsiPort: f_ScsiPort,
            ScsiBus: f_ScsiBus,
            ScsiTarget: f_ScsiTarget,
            ScsiLun: f_ScsiLun,
            dwMountCount: f_dwMountCount,
            LastCleanedTs: f_LastCleanedTs,
            SavedPartitionId: f_SavedPartitionId,
            Library: f_Library,
            Reserved: f_Reserved,
            dwDeferDismountDelay: f_dwDeferDismountDelay,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 232);
        FromIntoMemory::into_bytes(self.Number, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.State, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.DriveType, &mut into[8..8 + 16]);
        FromIntoMemory::into_bytes(self.szDeviceName, &mut into[24..24 + 64]);
        FromIntoMemory::into_bytes(self.szSerialNumber, &mut into[88..88 + 32]);
        FromIntoMemory::into_bytes(self.szRevision, &mut into[120..120 + 32]);
        FromIntoMemory::into_bytes(self.ScsiPort, &mut into[152..152 + 2]);
        FromIntoMemory::into_bytes(self.ScsiBus, &mut into[154..154 + 2]);
        FromIntoMemory::into_bytes(self.ScsiTarget, &mut into[156..156 + 2]);
        FromIntoMemory::into_bytes(self.ScsiLun, &mut into[158..158 + 2]);
        FromIntoMemory::into_bytes(self.dwMountCount, &mut into[160..160 + 4]);
        FromIntoMemory::into_bytes(self.LastCleanedTs, &mut into[164..164 + 16]);
        FromIntoMemory::into_bytes(self.SavedPartitionId, &mut into[180..180 + 16]);
        FromIntoMemory::into_bytes(self.Library, &mut into[196..196 + 16]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[212..212 + 16]);
        FromIntoMemory::into_bytes(self.dwDeferDismountDelay, &mut into[228..228 + 4]);
    }
    fn size() -> usize {
        232
    }
}
pub struct NTMS_DRIVEINFORMATIONW {
    pub Number: u32,
    pub State: NtmsDriveState,
    pub DriveType: crate::core::GUID,
    pub szDeviceName: [u16; 64],
    pub szSerialNumber: [u16; 32],
    pub szRevision: [u16; 32],
    pub ScsiPort: u16,
    pub ScsiBus: u16,
    pub ScsiTarget: u16,
    pub ScsiLun: u16,
    pub dwMountCount: u32,
    pub LastCleanedTs: super::super::Foundation::SYSTEMTIME,
    pub SavedPartitionId: crate::core::GUID,
    pub Library: crate::core::GUID,
    pub Reserved: crate::core::GUID,
    pub dwDeferDismountDelay: u32,
}
impl ::core::marker::Copy for NTMS_DRIVEINFORMATIONW {}
impl ::core::clone::Clone for NTMS_DRIVEINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_DRIVEINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_DRIVEINFORMATIONW")
            .field("Number", &self.Number)
            .field("State", &self.State)
            .field("DriveType", &self.DriveType)
            .field("szDeviceName", &self.szDeviceName)
            .field("szSerialNumber", &self.szSerialNumber)
            .field("szRevision", &self.szRevision)
            .field("ScsiPort", &self.ScsiPort)
            .field("ScsiBus", &self.ScsiBus)
            .field("ScsiTarget", &self.ScsiTarget)
            .field("ScsiLun", &self.ScsiLun)
            .field("dwMountCount", &self.dwMountCount)
            .field("LastCleanedTs", &self.LastCleanedTs)
            .field("SavedPartitionId", &self.SavedPartitionId)
            .field("Library", &self.Library)
            .field("Reserved", &self.Reserved)
            .field("dwDeferDismountDelay", &self.dwDeferDismountDelay)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_DRIVEINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number
            && self.State == other.State
            && self.DriveType == other.DriveType
            && self.szDeviceName == other.szDeviceName
            && self.szSerialNumber == other.szSerialNumber
            && self.szRevision == other.szRevision
            && self.ScsiPort == other.ScsiPort
            && self.ScsiBus == other.ScsiBus
            && self.ScsiTarget == other.ScsiTarget
            && self.ScsiLun == other.ScsiLun
            && self.dwMountCount == other.dwMountCount
            && self.LastCleanedTs == other.LastCleanedTs
            && self.SavedPartitionId == other.SavedPartitionId
            && self.Library == other.Library
            && self.Reserved == other.Reserved
            && self.dwDeferDismountDelay == other.dwDeferDismountDelay
    }
}
impl ::core::cmp::Eq for NTMS_DRIVEINFORMATIONW {}
impl FromIntoMemory for NTMS_DRIVEINFORMATIONW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 232);
        let f_Number = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_State = <NtmsDriveState as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_DriveType = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        let f_szDeviceName = <[u16; 64] as FromIntoMemory>::from_bytes(&from[24..24 + 64]);
        let f_szSerialNumber = <[u16; 32] as FromIntoMemory>::from_bytes(&from[88..88 + 32]);
        let f_szRevision = <[u16; 32] as FromIntoMemory>::from_bytes(&from[120..120 + 32]);
        let f_ScsiPort = <u16 as FromIntoMemory>::from_bytes(&from[152..152 + 2]);
        let f_ScsiBus = <u16 as FromIntoMemory>::from_bytes(&from[154..154 + 2]);
        let f_ScsiTarget = <u16 as FromIntoMemory>::from_bytes(&from[156..156 + 2]);
        let f_ScsiLun = <u16 as FromIntoMemory>::from_bytes(&from[158..158 + 2]);
        let f_dwMountCount = <u32 as FromIntoMemory>::from_bytes(&from[160..160 + 4]);
        let f_LastCleanedTs = <super::super::Foundation::SYSTEMTIME as FromIntoMemory>::from_bytes(
            &from[164..164 + 16],
        );
        let f_SavedPartitionId =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[180..180 + 16]);
        let f_Library = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[196..196 + 16]);
        let f_Reserved = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[212..212 + 16]);
        let f_dwDeferDismountDelay = <u32 as FromIntoMemory>::from_bytes(&from[228..228 + 4]);
        Self {
            Number: f_Number,
            State: f_State,
            DriveType: f_DriveType,
            szDeviceName: f_szDeviceName,
            szSerialNumber: f_szSerialNumber,
            szRevision: f_szRevision,
            ScsiPort: f_ScsiPort,
            ScsiBus: f_ScsiBus,
            ScsiTarget: f_ScsiTarget,
            ScsiLun: f_ScsiLun,
            dwMountCount: f_dwMountCount,
            LastCleanedTs: f_LastCleanedTs,
            SavedPartitionId: f_SavedPartitionId,
            Library: f_Library,
            Reserved: f_Reserved,
            dwDeferDismountDelay: f_dwDeferDismountDelay,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 232);
        FromIntoMemory::into_bytes(self.Number, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.State, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.DriveType, &mut into[8..8 + 16]);
        FromIntoMemory::into_bytes(self.szDeviceName, &mut into[24..24 + 64]);
        FromIntoMemory::into_bytes(self.szSerialNumber, &mut into[88..88 + 32]);
        FromIntoMemory::into_bytes(self.szRevision, &mut into[120..120 + 32]);
        FromIntoMemory::into_bytes(self.ScsiPort, &mut into[152..152 + 2]);
        FromIntoMemory::into_bytes(self.ScsiBus, &mut into[154..154 + 2]);
        FromIntoMemory::into_bytes(self.ScsiTarget, &mut into[156..156 + 2]);
        FromIntoMemory::into_bytes(self.ScsiLun, &mut into[158..158 + 2]);
        FromIntoMemory::into_bytes(self.dwMountCount, &mut into[160..160 + 4]);
        FromIntoMemory::into_bytes(self.LastCleanedTs, &mut into[164..164 + 16]);
        FromIntoMemory::into_bytes(self.SavedPartitionId, &mut into[180..180 + 16]);
        FromIntoMemory::into_bytes(self.Library, &mut into[196..196 + 16]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[212..212 + 16]);
        FromIntoMemory::into_bytes(self.dwDeferDismountDelay, &mut into[228..228 + 4]);
    }
    fn size() -> usize {
        232
    }
}
pub struct NTMS_DRIVETYPEINFORMATIONA {
    pub szVendor: [super::super::Foundation::CHAR; 128],
    pub szProduct: [super::super::Foundation::CHAR; 128],
    pub NumberOfHeads: u32,
    pub DeviceType: FILE_DEVICE_TYPE,
}
impl ::core::marker::Copy for NTMS_DRIVETYPEINFORMATIONA {}
impl ::core::clone::Clone for NTMS_DRIVETYPEINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_DRIVETYPEINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_DRIVETYPEINFORMATIONA")
            .field("szVendor", &self.szVendor)
            .field("szProduct", &self.szProduct)
            .field("NumberOfHeads", &self.NumberOfHeads)
            .field("DeviceType", &self.DeviceType)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_DRIVETYPEINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.szVendor == other.szVendor
            && self.szProduct == other.szProduct
            && self.NumberOfHeads == other.NumberOfHeads
            && self.DeviceType == other.DeviceType
    }
}
impl ::core::cmp::Eq for NTMS_DRIVETYPEINFORMATIONA {}
impl FromIntoMemory for NTMS_DRIVETYPEINFORMATIONA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 264);
        let f_szVendor = <[super::super::Foundation::CHAR; 128] as FromIntoMemory>::from_bytes(
            &from[0..0 + 128],
        );
        let f_szProduct = <[super::super::Foundation::CHAR; 128] as FromIntoMemory>::from_bytes(
            &from[128..128 + 128],
        );
        let f_NumberOfHeads = <u32 as FromIntoMemory>::from_bytes(&from[256..256 + 4]);
        let f_DeviceType = <FILE_DEVICE_TYPE as FromIntoMemory>::from_bytes(&from[260..260 + 4]);
        Self {
            szVendor: f_szVendor,
            szProduct: f_szProduct,
            NumberOfHeads: f_NumberOfHeads,
            DeviceType: f_DeviceType,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 264);
        FromIntoMemory::into_bytes(self.szVendor, &mut into[0..0 + 128]);
        FromIntoMemory::into_bytes(self.szProduct, &mut into[128..128 + 128]);
        FromIntoMemory::into_bytes(self.NumberOfHeads, &mut into[256..256 + 4]);
        FromIntoMemory::into_bytes(self.DeviceType, &mut into[260..260 + 4]);
    }
    fn size() -> usize {
        264
    }
}
pub struct NTMS_DRIVETYPEINFORMATIONW {
    pub szVendor: [u16; 128],
    pub szProduct: [u16; 128],
    pub NumberOfHeads: u32,
    pub DeviceType: FILE_DEVICE_TYPE,
}
impl ::core::marker::Copy for NTMS_DRIVETYPEINFORMATIONW {}
impl ::core::clone::Clone for NTMS_DRIVETYPEINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_DRIVETYPEINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_DRIVETYPEINFORMATIONW")
            .field("szVendor", &self.szVendor)
            .field("szProduct", &self.szProduct)
            .field("NumberOfHeads", &self.NumberOfHeads)
            .field("DeviceType", &self.DeviceType)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_DRIVETYPEINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.szVendor == other.szVendor
            && self.szProduct == other.szProduct
            && self.NumberOfHeads == other.NumberOfHeads
            && self.DeviceType == other.DeviceType
    }
}
impl ::core::cmp::Eq for NTMS_DRIVETYPEINFORMATIONW {}
impl FromIntoMemory for NTMS_DRIVETYPEINFORMATIONW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 264);
        let f_szVendor = <[u16; 128] as FromIntoMemory>::from_bytes(&from[0..0 + 128]);
        let f_szProduct = <[u16; 128] as FromIntoMemory>::from_bytes(&from[128..128 + 128]);
        let f_NumberOfHeads = <u32 as FromIntoMemory>::from_bytes(&from[256..256 + 4]);
        let f_DeviceType = <FILE_DEVICE_TYPE as FromIntoMemory>::from_bytes(&from[260..260 + 4]);
        Self {
            szVendor: f_szVendor,
            szProduct: f_szProduct,
            NumberOfHeads: f_NumberOfHeads,
            DeviceType: f_DeviceType,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 264);
        FromIntoMemory::into_bytes(self.szVendor, &mut into[0..0 + 128]);
        FromIntoMemory::into_bytes(self.szProduct, &mut into[128..128 + 128]);
        FromIntoMemory::into_bytes(self.NumberOfHeads, &mut into[256..256 + 4]);
        FromIntoMemory::into_bytes(self.DeviceType, &mut into[260..260 + 4]);
    }
    fn size() -> usize {
        264
    }
}
pub struct NTMS_FILESYSTEM_INFO {
    pub FileSystemType: [u16; 64],
    pub VolumeName: [u16; 256],
    pub SerialNumber: u32,
}
impl ::core::marker::Copy for NTMS_FILESYSTEM_INFO {}
impl ::core::clone::Clone for NTMS_FILESYSTEM_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_FILESYSTEM_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_FILESYSTEM_INFO")
            .field("FileSystemType", &self.FileSystemType)
            .field("VolumeName", &self.VolumeName)
            .field("SerialNumber", &self.SerialNumber)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_FILESYSTEM_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.FileSystemType == other.FileSystemType
            && self.VolumeName == other.VolumeName
            && self.SerialNumber == other.SerialNumber
    }
}
impl ::core::cmp::Eq for NTMS_FILESYSTEM_INFO {}
impl FromIntoMemory for NTMS_FILESYSTEM_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 324);
        let f_FileSystemType = <[u16; 64] as FromIntoMemory>::from_bytes(&from[0..0 + 64]);
        let f_VolumeName = <[u16; 256] as FromIntoMemory>::from_bytes(&from[64..64 + 256]);
        let f_SerialNumber = <u32 as FromIntoMemory>::from_bytes(&from[320..320 + 4]);
        Self {
            FileSystemType: f_FileSystemType,
            VolumeName: f_VolumeName,
            SerialNumber: f_SerialNumber,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 324);
        FromIntoMemory::into_bytes(self.FileSystemType, &mut into[0..0 + 64]);
        FromIntoMemory::into_bytes(self.VolumeName, &mut into[64..64 + 256]);
        FromIntoMemory::into_bytes(self.SerialNumber, &mut into[320..320 + 4]);
    }
    fn size() -> usize {
        324
    }
}
pub struct NTMS_I1_LIBRARYINFORMATION {
    pub LibraryType: u32,
    pub CleanerSlot: crate::core::GUID,
    pub CleanerSlotDefault: crate::core::GUID,
    pub LibrarySupportsDriveCleaning: super::super::Foundation::BOOL,
    pub BarCodeReaderInstalled: super::super::Foundation::BOOL,
    pub InventoryMethod: u32,
    pub dwCleanerUsesRemaining: u32,
    pub FirstDriveNumber: u32,
    pub dwNumberOfDrives: u32,
    pub FirstSlotNumber: u32,
    pub dwNumberOfSlots: u32,
    pub FirstDoorNumber: u32,
    pub dwNumberOfDoors: u32,
    pub FirstPortNumber: u32,
    pub dwNumberOfPorts: u32,
    pub FirstChangerNumber: u32,
    pub dwNumberOfChangers: u32,
    pub dwNumberOfMedia: u32,
    pub dwNumberOfMediaTypes: u32,
    pub dwNumberOfLibRequests: u32,
    pub Reserved: crate::core::GUID,
}
impl ::core::marker::Copy for NTMS_I1_LIBRARYINFORMATION {}
impl ::core::clone::Clone for NTMS_I1_LIBRARYINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_I1_LIBRARYINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_I1_LIBRARYINFORMATION")
            .field("LibraryType", &self.LibraryType)
            .field("CleanerSlot", &self.CleanerSlot)
            .field("CleanerSlotDefault", &self.CleanerSlotDefault)
            .field(
                "LibrarySupportsDriveCleaning",
                &self.LibrarySupportsDriveCleaning,
            )
            .field("BarCodeReaderInstalled", &self.BarCodeReaderInstalled)
            .field("InventoryMethod", &self.InventoryMethod)
            .field("dwCleanerUsesRemaining", &self.dwCleanerUsesRemaining)
            .field("FirstDriveNumber", &self.FirstDriveNumber)
            .field("dwNumberOfDrives", &self.dwNumberOfDrives)
            .field("FirstSlotNumber", &self.FirstSlotNumber)
            .field("dwNumberOfSlots", &self.dwNumberOfSlots)
            .field("FirstDoorNumber", &self.FirstDoorNumber)
            .field("dwNumberOfDoors", &self.dwNumberOfDoors)
            .field("FirstPortNumber", &self.FirstPortNumber)
            .field("dwNumberOfPorts", &self.dwNumberOfPorts)
            .field("FirstChangerNumber", &self.FirstChangerNumber)
            .field("dwNumberOfChangers", &self.dwNumberOfChangers)
            .field("dwNumberOfMedia", &self.dwNumberOfMedia)
            .field("dwNumberOfMediaTypes", &self.dwNumberOfMediaTypes)
            .field("dwNumberOfLibRequests", &self.dwNumberOfLibRequests)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_I1_LIBRARYINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.LibraryType == other.LibraryType
            && self.CleanerSlot == other.CleanerSlot
            && self.CleanerSlotDefault == other.CleanerSlotDefault
            && self.LibrarySupportsDriveCleaning == other.LibrarySupportsDriveCleaning
            && self.BarCodeReaderInstalled == other.BarCodeReaderInstalled
            && self.InventoryMethod == other.InventoryMethod
            && self.dwCleanerUsesRemaining == other.dwCleanerUsesRemaining
            && self.FirstDriveNumber == other.FirstDriveNumber
            && self.dwNumberOfDrives == other.dwNumberOfDrives
            && self.FirstSlotNumber == other.FirstSlotNumber
            && self.dwNumberOfSlots == other.dwNumberOfSlots
            && self.FirstDoorNumber == other.FirstDoorNumber
            && self.dwNumberOfDoors == other.dwNumberOfDoors
            && self.FirstPortNumber == other.FirstPortNumber
            && self.dwNumberOfPorts == other.dwNumberOfPorts
            && self.FirstChangerNumber == other.FirstChangerNumber
            && self.dwNumberOfChangers == other.dwNumberOfChangers
            && self.dwNumberOfMedia == other.dwNumberOfMedia
            && self.dwNumberOfMediaTypes == other.dwNumberOfMediaTypes
            && self.dwNumberOfLibRequests == other.dwNumberOfLibRequests
            && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for NTMS_I1_LIBRARYINFORMATION {}
impl FromIntoMemory for NTMS_I1_LIBRARYINFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 120);
        let f_LibraryType = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_CleanerSlot = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[4..4 + 16]);
        let f_CleanerSlotDefault =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[20..20 + 16]);
        let f_LibrarySupportsDriveCleaning =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_BarCodeReaderInstalled =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_InventoryMethod = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_dwCleanerUsesRemaining = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_FirstDriveNumber = <u32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_dwNumberOfDrives = <u32 as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_FirstSlotNumber = <u32 as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_dwNumberOfSlots = <u32 as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        let f_FirstDoorNumber = <u32 as FromIntoMemory>::from_bytes(&from[68..68 + 4]);
        let f_dwNumberOfDoors = <u32 as FromIntoMemory>::from_bytes(&from[72..72 + 4]);
        let f_FirstPortNumber = <u32 as FromIntoMemory>::from_bytes(&from[76..76 + 4]);
        let f_dwNumberOfPorts = <u32 as FromIntoMemory>::from_bytes(&from[80..80 + 4]);
        let f_FirstChangerNumber = <u32 as FromIntoMemory>::from_bytes(&from[84..84 + 4]);
        let f_dwNumberOfChangers = <u32 as FromIntoMemory>::from_bytes(&from[88..88 + 4]);
        let f_dwNumberOfMedia = <u32 as FromIntoMemory>::from_bytes(&from[92..92 + 4]);
        let f_dwNumberOfMediaTypes = <u32 as FromIntoMemory>::from_bytes(&from[96..96 + 4]);
        let f_dwNumberOfLibRequests = <u32 as FromIntoMemory>::from_bytes(&from[100..100 + 4]);
        let f_Reserved = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[104..104 + 16]);
        Self {
            LibraryType: f_LibraryType,
            CleanerSlot: f_CleanerSlot,
            CleanerSlotDefault: f_CleanerSlotDefault,
            LibrarySupportsDriveCleaning: f_LibrarySupportsDriveCleaning,
            BarCodeReaderInstalled: f_BarCodeReaderInstalled,
            InventoryMethod: f_InventoryMethod,
            dwCleanerUsesRemaining: f_dwCleanerUsesRemaining,
            FirstDriveNumber: f_FirstDriveNumber,
            dwNumberOfDrives: f_dwNumberOfDrives,
            FirstSlotNumber: f_FirstSlotNumber,
            dwNumberOfSlots: f_dwNumberOfSlots,
            FirstDoorNumber: f_FirstDoorNumber,
            dwNumberOfDoors: f_dwNumberOfDoors,
            FirstPortNumber: f_FirstPortNumber,
            dwNumberOfPorts: f_dwNumberOfPorts,
            FirstChangerNumber: f_FirstChangerNumber,
            dwNumberOfChangers: f_dwNumberOfChangers,
            dwNumberOfMedia: f_dwNumberOfMedia,
            dwNumberOfMediaTypes: f_dwNumberOfMediaTypes,
            dwNumberOfLibRequests: f_dwNumberOfLibRequests,
            Reserved: f_Reserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 120);
        FromIntoMemory::into_bytes(self.LibraryType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.CleanerSlot, &mut into[4..4 + 16]);
        FromIntoMemory::into_bytes(self.CleanerSlotDefault, &mut into[20..20 + 16]);
        FromIntoMemory::into_bytes(self.LibrarySupportsDriveCleaning, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.BarCodeReaderInstalled, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.InventoryMethod, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.dwCleanerUsesRemaining, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.FirstDriveNumber, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfDrives, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.FirstSlotNumber, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfSlots, &mut into[64..64 + 4]);
        FromIntoMemory::into_bytes(self.FirstDoorNumber, &mut into[68..68 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfDoors, &mut into[72..72 + 4]);
        FromIntoMemory::into_bytes(self.FirstPortNumber, &mut into[76..76 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfPorts, &mut into[80..80 + 4]);
        FromIntoMemory::into_bytes(self.FirstChangerNumber, &mut into[84..84 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfChangers, &mut into[88..88 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfMedia, &mut into[92..92 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfMediaTypes, &mut into[96..96 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfLibRequests, &mut into[100..100 + 4]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[104..104 + 16]);
    }
    fn size() -> usize {
        120
    }
}
pub struct NTMS_I1_LIBREQUESTINFORMATIONA {
    pub OperationCode: u32,
    pub OperationOption: u32,
    pub State: u32,
    pub PartitionId: crate::core::GUID,
    pub DriveId: crate::core::GUID,
    pub PhysMediaId: crate::core::GUID,
    pub Library: crate::core::GUID,
    pub SlotId: crate::core::GUID,
    pub TimeQueued: super::super::Foundation::SYSTEMTIME,
    pub TimeCompleted: super::super::Foundation::SYSTEMTIME,
    pub szApplication: [super::super::Foundation::CHAR; 64],
    pub szUser: [super::super::Foundation::CHAR; 64],
    pub szComputer: [super::super::Foundation::CHAR; 64],
}
impl ::core::marker::Copy for NTMS_I1_LIBREQUESTINFORMATIONA {}
impl ::core::clone::Clone for NTMS_I1_LIBREQUESTINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_I1_LIBREQUESTINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_I1_LIBREQUESTINFORMATIONA")
            .field("OperationCode", &self.OperationCode)
            .field("OperationOption", &self.OperationOption)
            .field("State", &self.State)
            .field("PartitionId", &self.PartitionId)
            .field("DriveId", &self.DriveId)
            .field("PhysMediaId", &self.PhysMediaId)
            .field("Library", &self.Library)
            .field("SlotId", &self.SlotId)
            .field("TimeQueued", &self.TimeQueued)
            .field("TimeCompleted", &self.TimeCompleted)
            .field("szApplication", &self.szApplication)
            .field("szUser", &self.szUser)
            .field("szComputer", &self.szComputer)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_I1_LIBREQUESTINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.OperationCode == other.OperationCode
            && self.OperationOption == other.OperationOption
            && self.State == other.State
            && self.PartitionId == other.PartitionId
            && self.DriveId == other.DriveId
            && self.PhysMediaId == other.PhysMediaId
            && self.Library == other.Library
            && self.SlotId == other.SlotId
            && self.TimeQueued == other.TimeQueued
            && self.TimeCompleted == other.TimeCompleted
            && self.szApplication == other.szApplication
            && self.szUser == other.szUser
            && self.szComputer == other.szComputer
    }
}
impl ::core::cmp::Eq for NTMS_I1_LIBREQUESTINFORMATIONA {}
impl FromIntoMemory for NTMS_I1_LIBREQUESTINFORMATIONA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 316);
        let f_OperationCode = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_OperationOption = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_State = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_PartitionId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[12..12 + 16]);
        let f_DriveId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[28..28 + 16]);
        let f_PhysMediaId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[44..44 + 16]);
        let f_Library = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[60..60 + 16]);
        let f_SlotId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[76..76 + 16]);
        let f_TimeQueued = <super::super::Foundation::SYSTEMTIME as FromIntoMemory>::from_bytes(
            &from[92..92 + 16],
        );
        let f_TimeCompleted = <super::super::Foundation::SYSTEMTIME as FromIntoMemory>::from_bytes(
            &from[108..108 + 16],
        );
        let f_szApplication = <[super::super::Foundation::CHAR; 64] as FromIntoMemory>::from_bytes(
            &from[124..124 + 64],
        );
        let f_szUser = <[super::super::Foundation::CHAR; 64] as FromIntoMemory>::from_bytes(
            &from[188..188 + 64],
        );
        let f_szComputer = <[super::super::Foundation::CHAR; 64] as FromIntoMemory>::from_bytes(
            &from[252..252 + 64],
        );
        Self {
            OperationCode: f_OperationCode,
            OperationOption: f_OperationOption,
            State: f_State,
            PartitionId: f_PartitionId,
            DriveId: f_DriveId,
            PhysMediaId: f_PhysMediaId,
            Library: f_Library,
            SlotId: f_SlotId,
            TimeQueued: f_TimeQueued,
            TimeCompleted: f_TimeCompleted,
            szApplication: f_szApplication,
            szUser: f_szUser,
            szComputer: f_szComputer,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 316);
        FromIntoMemory::into_bytes(self.OperationCode, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.OperationOption, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.State, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.PartitionId, &mut into[12..12 + 16]);
        FromIntoMemory::into_bytes(self.DriveId, &mut into[28..28 + 16]);
        FromIntoMemory::into_bytes(self.PhysMediaId, &mut into[44..44 + 16]);
        FromIntoMemory::into_bytes(self.Library, &mut into[60..60 + 16]);
        FromIntoMemory::into_bytes(self.SlotId, &mut into[76..76 + 16]);
        FromIntoMemory::into_bytes(self.TimeQueued, &mut into[92..92 + 16]);
        FromIntoMemory::into_bytes(self.TimeCompleted, &mut into[108..108 + 16]);
        FromIntoMemory::into_bytes(self.szApplication, &mut into[124..124 + 64]);
        FromIntoMemory::into_bytes(self.szUser, &mut into[188..188 + 64]);
        FromIntoMemory::into_bytes(self.szComputer, &mut into[252..252 + 64]);
    }
    fn size() -> usize {
        316
    }
}
pub struct NTMS_I1_LIBREQUESTINFORMATIONW {
    pub OperationCode: u32,
    pub OperationOption: u32,
    pub State: u32,
    pub PartitionId: crate::core::GUID,
    pub DriveId: crate::core::GUID,
    pub PhysMediaId: crate::core::GUID,
    pub Library: crate::core::GUID,
    pub SlotId: crate::core::GUID,
    pub TimeQueued: super::super::Foundation::SYSTEMTIME,
    pub TimeCompleted: super::super::Foundation::SYSTEMTIME,
    pub szApplication: [u16; 64],
    pub szUser: [u16; 64],
    pub szComputer: [u16; 64],
}
impl ::core::marker::Copy for NTMS_I1_LIBREQUESTINFORMATIONW {}
impl ::core::clone::Clone for NTMS_I1_LIBREQUESTINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_I1_LIBREQUESTINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_I1_LIBREQUESTINFORMATIONW")
            .field("OperationCode", &self.OperationCode)
            .field("OperationOption", &self.OperationOption)
            .field("State", &self.State)
            .field("PartitionId", &self.PartitionId)
            .field("DriveId", &self.DriveId)
            .field("PhysMediaId", &self.PhysMediaId)
            .field("Library", &self.Library)
            .field("SlotId", &self.SlotId)
            .field("TimeQueued", &self.TimeQueued)
            .field("TimeCompleted", &self.TimeCompleted)
            .field("szApplication", &self.szApplication)
            .field("szUser", &self.szUser)
            .field("szComputer", &self.szComputer)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_I1_LIBREQUESTINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.OperationCode == other.OperationCode
            && self.OperationOption == other.OperationOption
            && self.State == other.State
            && self.PartitionId == other.PartitionId
            && self.DriveId == other.DriveId
            && self.PhysMediaId == other.PhysMediaId
            && self.Library == other.Library
            && self.SlotId == other.SlotId
            && self.TimeQueued == other.TimeQueued
            && self.TimeCompleted == other.TimeCompleted
            && self.szApplication == other.szApplication
            && self.szUser == other.szUser
            && self.szComputer == other.szComputer
    }
}
impl ::core::cmp::Eq for NTMS_I1_LIBREQUESTINFORMATIONW {}
impl FromIntoMemory for NTMS_I1_LIBREQUESTINFORMATIONW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 316);
        let f_OperationCode = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_OperationOption = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_State = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_PartitionId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[12..12 + 16]);
        let f_DriveId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[28..28 + 16]);
        let f_PhysMediaId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[44..44 + 16]);
        let f_Library = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[60..60 + 16]);
        let f_SlotId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[76..76 + 16]);
        let f_TimeQueued = <super::super::Foundation::SYSTEMTIME as FromIntoMemory>::from_bytes(
            &from[92..92 + 16],
        );
        let f_TimeCompleted = <super::super::Foundation::SYSTEMTIME as FromIntoMemory>::from_bytes(
            &from[108..108 + 16],
        );
        let f_szApplication = <[u16; 64] as FromIntoMemory>::from_bytes(&from[124..124 + 64]);
        let f_szUser = <[u16; 64] as FromIntoMemory>::from_bytes(&from[188..188 + 64]);
        let f_szComputer = <[u16; 64] as FromIntoMemory>::from_bytes(&from[252..252 + 64]);
        Self {
            OperationCode: f_OperationCode,
            OperationOption: f_OperationOption,
            State: f_State,
            PartitionId: f_PartitionId,
            DriveId: f_DriveId,
            PhysMediaId: f_PhysMediaId,
            Library: f_Library,
            SlotId: f_SlotId,
            TimeQueued: f_TimeQueued,
            TimeCompleted: f_TimeCompleted,
            szApplication: f_szApplication,
            szUser: f_szUser,
            szComputer: f_szComputer,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 316);
        FromIntoMemory::into_bytes(self.OperationCode, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.OperationOption, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.State, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.PartitionId, &mut into[12..12 + 16]);
        FromIntoMemory::into_bytes(self.DriveId, &mut into[28..28 + 16]);
        FromIntoMemory::into_bytes(self.PhysMediaId, &mut into[44..44 + 16]);
        FromIntoMemory::into_bytes(self.Library, &mut into[60..60 + 16]);
        FromIntoMemory::into_bytes(self.SlotId, &mut into[76..76 + 16]);
        FromIntoMemory::into_bytes(self.TimeQueued, &mut into[92..92 + 16]);
        FromIntoMemory::into_bytes(self.TimeCompleted, &mut into[108..108 + 16]);
        FromIntoMemory::into_bytes(self.szApplication, &mut into[124..124 + 64]);
        FromIntoMemory::into_bytes(self.szUser, &mut into[188..188 + 64]);
        FromIntoMemory::into_bytes(self.szComputer, &mut into[252..252 + 64]);
    }
    fn size() -> usize {
        316
    }
}
pub const NTMS_I1_MESSAGE_LENGTH: u32 = 127u32;
pub struct NTMS_I1_OBJECTINFORMATIONA {
    pub dwSize: u32,
    pub dwType: u32,
    pub Created: super::super::Foundation::SYSTEMTIME,
    pub Modified: super::super::Foundation::SYSTEMTIME,
    pub ObjectGuid: crate::core::GUID,
    pub Enabled: super::super::Foundation::BOOL,
    pub dwOperationalState: u32,
    pub szName: [super::super::Foundation::CHAR; 64],
    pub szDescription: [super::super::Foundation::CHAR; 127],
    pub Info: NTMS_I1_OBJECTINFORMATIONA_0,
}
impl ::core::marker::Copy for NTMS_I1_OBJECTINFORMATIONA {}
impl ::core::clone::Clone for NTMS_I1_OBJECTINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for NTMS_I1_OBJECTINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwType == other.dwType
            && self.Created == other.Created
            && self.Modified == other.Modified
            && self.ObjectGuid == other.ObjectGuid
            && self.Enabled == other.Enabled
            && self.dwOperationalState == other.dwOperationalState
            && self.szName == other.szName
            && self.szDescription == other.szDescription
            && self.Info == other.Info
    }
}
impl ::core::cmp::Eq for NTMS_I1_OBJECTINFORMATIONA {}
impl FromIntoMemory for NTMS_I1_OBJECTINFORMATIONA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 640);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwType = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Created =
            <super::super::Foundation::SYSTEMTIME as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        let f_Modified = <super::super::Foundation::SYSTEMTIME as FromIntoMemory>::from_bytes(
            &from[24..24 + 16],
        );
        let f_ObjectGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[40..40 + 16]);
        let f_Enabled =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_dwOperationalState = <u32 as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_szName = <[super::super::Foundation::CHAR; 64] as FromIntoMemory>::from_bytes(
            &from[64..64 + 64],
        );
        let f_szDescription = <[super::super::Foundation::CHAR; 127] as FromIntoMemory>::from_bytes(
            &from[128..128 + 127],
        );
        let f_Info =
            <NTMS_I1_OBJECTINFORMATIONA_0 as FromIntoMemory>::from_bytes(&from[256..256 + 384]);
        Self {
            dwSize: f_dwSize,
            dwType: f_dwType,
            Created: f_Created,
            Modified: f_Modified,
            ObjectGuid: f_ObjectGuid,
            Enabled: f_Enabled,
            dwOperationalState: f_dwOperationalState,
            szName: f_szName,
            szDescription: f_szDescription,
            Info: f_Info,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 640);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwType, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Created, &mut into[8..8 + 16]);
        FromIntoMemory::into_bytes(self.Modified, &mut into[24..24 + 16]);
        FromIntoMemory::into_bytes(self.ObjectGuid, &mut into[40..40 + 16]);
        FromIntoMemory::into_bytes(self.Enabled, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.dwOperationalState, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.szName, &mut into[64..64 + 64]);
        FromIntoMemory::into_bytes(self.szDescription, &mut into[128..128 + 127]);
        FromIntoMemory::into_bytes(self.Info, &mut into[256..256 + 384]);
    }
    fn size() -> usize {
        640
    }
}
pub struct NTMS_I1_OBJECTINFORMATIONA_0 {
    data: [u8; 384],
}
impl ::core::default::Default for NTMS_I1_OBJECTINFORMATIONA_0 {
    fn default() -> Self {
        Self { data: [0u8; 384] }
    }
}
impl ::core::marker::Copy for NTMS_I1_OBJECTINFORMATIONA_0 {}
impl ::core::clone::Clone for NTMS_I1_OBJECTINFORMATIONA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for NTMS_I1_OBJECTINFORMATIONA_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for NTMS_I1_OBJECTINFORMATIONA_0 {}
impl FromIntoMemory for NTMS_I1_OBJECTINFORMATIONA_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 384];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        384
    }
}
pub struct NTMS_I1_OBJECTINFORMATIONW {
    pub dwSize: u32,
    pub dwType: u32,
    pub Created: super::super::Foundation::SYSTEMTIME,
    pub Modified: super::super::Foundation::SYSTEMTIME,
    pub ObjectGuid: crate::core::GUID,
    pub Enabled: super::super::Foundation::BOOL,
    pub dwOperationalState: u32,
    pub szName: [u16; 64],
    pub szDescription: [u16; 127],
    pub Info: NTMS_I1_OBJECTINFORMATIONW_0,
}
impl ::core::marker::Copy for NTMS_I1_OBJECTINFORMATIONW {}
impl ::core::clone::Clone for NTMS_I1_OBJECTINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for NTMS_I1_OBJECTINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwType == other.dwType
            && self.Created == other.Created
            && self.Modified == other.Modified
            && self.ObjectGuid == other.ObjectGuid
            && self.Enabled == other.Enabled
            && self.dwOperationalState == other.dwOperationalState
            && self.szName == other.szName
            && self.szDescription == other.szDescription
            && self.Info == other.Info
    }
}
impl ::core::cmp::Eq for NTMS_I1_OBJECTINFORMATIONW {}
impl FromIntoMemory for NTMS_I1_OBJECTINFORMATIONW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 640);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwType = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Created =
            <super::super::Foundation::SYSTEMTIME as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        let f_Modified = <super::super::Foundation::SYSTEMTIME as FromIntoMemory>::from_bytes(
            &from[24..24 + 16],
        );
        let f_ObjectGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[40..40 + 16]);
        let f_Enabled =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_dwOperationalState = <u32 as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_szName = <[u16; 64] as FromIntoMemory>::from_bytes(&from[64..64 + 64]);
        let f_szDescription = <[u16; 127] as FromIntoMemory>::from_bytes(&from[128..128 + 127]);
        let f_Info =
            <NTMS_I1_OBJECTINFORMATIONW_0 as FromIntoMemory>::from_bytes(&from[256..256 + 384]);
        Self {
            dwSize: f_dwSize,
            dwType: f_dwType,
            Created: f_Created,
            Modified: f_Modified,
            ObjectGuid: f_ObjectGuid,
            Enabled: f_Enabled,
            dwOperationalState: f_dwOperationalState,
            szName: f_szName,
            szDescription: f_szDescription,
            Info: f_Info,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 640);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwType, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Created, &mut into[8..8 + 16]);
        FromIntoMemory::into_bytes(self.Modified, &mut into[24..24 + 16]);
        FromIntoMemory::into_bytes(self.ObjectGuid, &mut into[40..40 + 16]);
        FromIntoMemory::into_bytes(self.Enabled, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.dwOperationalState, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.szName, &mut into[64..64 + 64]);
        FromIntoMemory::into_bytes(self.szDescription, &mut into[128..128 + 127]);
        FromIntoMemory::into_bytes(self.Info, &mut into[256..256 + 384]);
    }
    fn size() -> usize {
        640
    }
}
pub struct NTMS_I1_OBJECTINFORMATIONW_0 {
    data: [u8; 384],
}
impl ::core::default::Default for NTMS_I1_OBJECTINFORMATIONW_0 {
    fn default() -> Self {
        Self { data: [0u8; 384] }
    }
}
impl ::core::marker::Copy for NTMS_I1_OBJECTINFORMATIONW_0 {}
impl ::core::clone::Clone for NTMS_I1_OBJECTINFORMATIONW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for NTMS_I1_OBJECTINFORMATIONW_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for NTMS_I1_OBJECTINFORMATIONW_0 {}
impl FromIntoMemory for NTMS_I1_OBJECTINFORMATIONW_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 384];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        384
    }
}
pub struct NTMS_I1_OPREQUESTINFORMATIONA {
    pub Request: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub State: u32,
    pub szMessage: [super::super::Foundation::CHAR; 127],
    pub Arg1Type: u32,
    pub Arg1: crate::core::GUID,
    pub Arg2Type: u32,
    pub Arg2: crate::core::GUID,
    pub szApplication: [super::super::Foundation::CHAR; 64],
    pub szUser: [super::super::Foundation::CHAR; 64],
    pub szComputer: [super::super::Foundation::CHAR; 64],
}
impl ::core::marker::Copy for NTMS_I1_OPREQUESTINFORMATIONA {}
impl ::core::clone::Clone for NTMS_I1_OPREQUESTINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_I1_OPREQUESTINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_I1_OPREQUESTINFORMATIONA")
            .field("Request", &self.Request)
            .field("Submitted", &self.Submitted)
            .field("State", &self.State)
            .field("szMessage", &self.szMessage)
            .field("Arg1Type", &self.Arg1Type)
            .field("Arg1", &self.Arg1)
            .field("Arg2Type", &self.Arg2Type)
            .field("Arg2", &self.Arg2)
            .field("szApplication", &self.szApplication)
            .field("szUser", &self.szUser)
            .field("szComputer", &self.szComputer)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_I1_OPREQUESTINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.Request == other.Request
            && self.Submitted == other.Submitted
            && self.State == other.State
            && self.szMessage == other.szMessage
            && self.Arg1Type == other.Arg1Type
            && self.Arg1 == other.Arg1
            && self.Arg2Type == other.Arg2Type
            && self.Arg2 == other.Arg2
            && self.szApplication == other.szApplication
            && self.szUser == other.szUser
            && self.szComputer == other.szComputer
    }
}
impl ::core::cmp::Eq for NTMS_I1_OPREQUESTINFORMATIONA {}
impl FromIntoMemory for NTMS_I1_OPREQUESTINFORMATIONA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 384);
        let f_Request = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Submitted =
            <super::super::Foundation::SYSTEMTIME as FromIntoMemory>::from_bytes(&from[4..4 + 16]);
        let f_State = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_szMessage = <[super::super::Foundation::CHAR; 127] as FromIntoMemory>::from_bytes(
            &from[24..24 + 127],
        );
        let f_Arg1Type = <u32 as FromIntoMemory>::from_bytes(&from[152..152 + 4]);
        let f_Arg1 = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[156..156 + 16]);
        let f_Arg2Type = <u32 as FromIntoMemory>::from_bytes(&from[172..172 + 4]);
        let f_Arg2 = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[176..176 + 16]);
        let f_szApplication = <[super::super::Foundation::CHAR; 64] as FromIntoMemory>::from_bytes(
            &from[192..192 + 64],
        );
        let f_szUser = <[super::super::Foundation::CHAR; 64] as FromIntoMemory>::from_bytes(
            &from[256..256 + 64],
        );
        let f_szComputer = <[super::super::Foundation::CHAR; 64] as FromIntoMemory>::from_bytes(
            &from[320..320 + 64],
        );
        Self {
            Request: f_Request,
            Submitted: f_Submitted,
            State: f_State,
            szMessage: f_szMessage,
            Arg1Type: f_Arg1Type,
            Arg1: f_Arg1,
            Arg2Type: f_Arg2Type,
            Arg2: f_Arg2,
            szApplication: f_szApplication,
            szUser: f_szUser,
            szComputer: f_szComputer,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 384);
        FromIntoMemory::into_bytes(self.Request, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Submitted, &mut into[4..4 + 16]);
        FromIntoMemory::into_bytes(self.State, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.szMessage, &mut into[24..24 + 127]);
        FromIntoMemory::into_bytes(self.Arg1Type, &mut into[152..152 + 4]);
        FromIntoMemory::into_bytes(self.Arg1, &mut into[156..156 + 16]);
        FromIntoMemory::into_bytes(self.Arg2Type, &mut into[172..172 + 4]);
        FromIntoMemory::into_bytes(self.Arg2, &mut into[176..176 + 16]);
        FromIntoMemory::into_bytes(self.szApplication, &mut into[192..192 + 64]);
        FromIntoMemory::into_bytes(self.szUser, &mut into[256..256 + 64]);
        FromIntoMemory::into_bytes(self.szComputer, &mut into[320..320 + 64]);
    }
    fn size() -> usize {
        384
    }
}
pub struct NTMS_I1_OPREQUESTINFORMATIONW {
    pub Request: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub State: u32,
    pub szMessage: [u16; 127],
    pub Arg1Type: u32,
    pub Arg1: crate::core::GUID,
    pub Arg2Type: u32,
    pub Arg2: crate::core::GUID,
    pub szApplication: [u16; 64],
    pub szUser: [u16; 64],
    pub szComputer: [u16; 64],
}
impl ::core::marker::Copy for NTMS_I1_OPREQUESTINFORMATIONW {}
impl ::core::clone::Clone for NTMS_I1_OPREQUESTINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_I1_OPREQUESTINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_I1_OPREQUESTINFORMATIONW")
            .field("Request", &self.Request)
            .field("Submitted", &self.Submitted)
            .field("State", &self.State)
            .field("szMessage", &self.szMessage)
            .field("Arg1Type", &self.Arg1Type)
            .field("Arg1", &self.Arg1)
            .field("Arg2Type", &self.Arg2Type)
            .field("Arg2", &self.Arg2)
            .field("szApplication", &self.szApplication)
            .field("szUser", &self.szUser)
            .field("szComputer", &self.szComputer)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_I1_OPREQUESTINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.Request == other.Request
            && self.Submitted == other.Submitted
            && self.State == other.State
            && self.szMessage == other.szMessage
            && self.Arg1Type == other.Arg1Type
            && self.Arg1 == other.Arg1
            && self.Arg2Type == other.Arg2Type
            && self.Arg2 == other.Arg2
            && self.szApplication == other.szApplication
            && self.szUser == other.szUser
            && self.szComputer == other.szComputer
    }
}
impl ::core::cmp::Eq for NTMS_I1_OPREQUESTINFORMATIONW {}
impl FromIntoMemory for NTMS_I1_OPREQUESTINFORMATIONW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 384);
        let f_Request = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Submitted =
            <super::super::Foundation::SYSTEMTIME as FromIntoMemory>::from_bytes(&from[4..4 + 16]);
        let f_State = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_szMessage = <[u16; 127] as FromIntoMemory>::from_bytes(&from[24..24 + 127]);
        let f_Arg1Type = <u32 as FromIntoMemory>::from_bytes(&from[152..152 + 4]);
        let f_Arg1 = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[156..156 + 16]);
        let f_Arg2Type = <u32 as FromIntoMemory>::from_bytes(&from[172..172 + 4]);
        let f_Arg2 = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[176..176 + 16]);
        let f_szApplication = <[u16; 64] as FromIntoMemory>::from_bytes(&from[192..192 + 64]);
        let f_szUser = <[u16; 64] as FromIntoMemory>::from_bytes(&from[256..256 + 64]);
        let f_szComputer = <[u16; 64] as FromIntoMemory>::from_bytes(&from[320..320 + 64]);
        Self {
            Request: f_Request,
            Submitted: f_Submitted,
            State: f_State,
            szMessage: f_szMessage,
            Arg1Type: f_Arg1Type,
            Arg1: f_Arg1,
            Arg2Type: f_Arg2Type,
            Arg2: f_Arg2,
            szApplication: f_szApplication,
            szUser: f_szUser,
            szComputer: f_szComputer,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 384);
        FromIntoMemory::into_bytes(self.Request, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Submitted, &mut into[4..4 + 16]);
        FromIntoMemory::into_bytes(self.State, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.szMessage, &mut into[24..24 + 127]);
        FromIntoMemory::into_bytes(self.Arg1Type, &mut into[152..152 + 4]);
        FromIntoMemory::into_bytes(self.Arg1, &mut into[156..156 + 16]);
        FromIntoMemory::into_bytes(self.Arg2Type, &mut into[172..172 + 4]);
        FromIntoMemory::into_bytes(self.Arg2, &mut into[176..176 + 16]);
        FromIntoMemory::into_bytes(self.szApplication, &mut into[192..192 + 64]);
        FromIntoMemory::into_bytes(self.szUser, &mut into[256..256 + 64]);
        FromIntoMemory::into_bytes(self.szComputer, &mut into[320..320 + 64]);
    }
    fn size() -> usize {
        384
    }
}
pub struct NTMS_I1_PARTITIONINFORMATIONA {
    pub PhysicalMedia: crate::core::GUID,
    pub LogicalMedia: crate::core::GUID,
    pub State: u32,
    pub Side: u16,
    pub dwOmidLabelIdLength: u32,
    pub OmidLabelId: [u8; 255],
    pub szOmidLabelType: [super::super::Foundation::CHAR; 64],
    pub szOmidLabelInfo: [super::super::Foundation::CHAR; 256],
    pub dwMountCount: u32,
    pub dwAllocateCount: u32,
}
impl ::core::marker::Copy for NTMS_I1_PARTITIONINFORMATIONA {}
impl ::core::clone::Clone for NTMS_I1_PARTITIONINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_I1_PARTITIONINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_I1_PARTITIONINFORMATIONA")
            .field("PhysicalMedia", &self.PhysicalMedia)
            .field("LogicalMedia", &self.LogicalMedia)
            .field("State", &self.State)
            .field("Side", &self.Side)
            .field("dwOmidLabelIdLength", &self.dwOmidLabelIdLength)
            .field("OmidLabelId", &self.OmidLabelId)
            .field("szOmidLabelType", &self.szOmidLabelType)
            .field("szOmidLabelInfo", &self.szOmidLabelInfo)
            .field("dwMountCount", &self.dwMountCount)
            .field("dwAllocateCount", &self.dwAllocateCount)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_I1_PARTITIONINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.PhysicalMedia == other.PhysicalMedia
            && self.LogicalMedia == other.LogicalMedia
            && self.State == other.State
            && self.Side == other.Side
            && self.dwOmidLabelIdLength == other.dwOmidLabelIdLength
            && self.OmidLabelId == other.OmidLabelId
            && self.szOmidLabelType == other.szOmidLabelType
            && self.szOmidLabelInfo == other.szOmidLabelInfo
            && self.dwMountCount == other.dwMountCount
            && self.dwAllocateCount == other.dwAllocateCount
    }
}
impl ::core::cmp::Eq for NTMS_I1_PARTITIONINFORMATIONA {}
impl FromIntoMemory for NTMS_I1_PARTITIONINFORMATIONA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 628);
        let f_PhysicalMedia = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_LogicalMedia = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[16..16 + 16]);
        let f_State = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_Side = <u16 as FromIntoMemory>::from_bytes(&from[36..36 + 2]);
        let f_dwOmidLabelIdLength = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_OmidLabelId = <[u8; 255] as FromIntoMemory>::from_bytes(&from[44..44 + 255]);
        let f_szOmidLabelType =
            <[super::super::Foundation::CHAR; 64] as FromIntoMemory>::from_bytes(
                &from[299..299 + 64],
            );
        let f_szOmidLabelInfo =
            <[super::super::Foundation::CHAR; 256] as FromIntoMemory>::from_bytes(
                &from[363..363 + 256],
            );
        let f_dwMountCount = <u32 as FromIntoMemory>::from_bytes(&from[620..620 + 4]);
        let f_dwAllocateCount = <u32 as FromIntoMemory>::from_bytes(&from[624..624 + 4]);
        Self {
            PhysicalMedia: f_PhysicalMedia,
            LogicalMedia: f_LogicalMedia,
            State: f_State,
            Side: f_Side,
            dwOmidLabelIdLength: f_dwOmidLabelIdLength,
            OmidLabelId: f_OmidLabelId,
            szOmidLabelType: f_szOmidLabelType,
            szOmidLabelInfo: f_szOmidLabelInfo,
            dwMountCount: f_dwMountCount,
            dwAllocateCount: f_dwAllocateCount,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 628);
        FromIntoMemory::into_bytes(self.PhysicalMedia, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.LogicalMedia, &mut into[16..16 + 16]);
        FromIntoMemory::into_bytes(self.State, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.Side, &mut into[36..36 + 2]);
        FromIntoMemory::into_bytes(self.dwOmidLabelIdLength, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.OmidLabelId, &mut into[44..44 + 255]);
        FromIntoMemory::into_bytes(self.szOmidLabelType, &mut into[299..299 + 64]);
        FromIntoMemory::into_bytes(self.szOmidLabelInfo, &mut into[363..363 + 256]);
        FromIntoMemory::into_bytes(self.dwMountCount, &mut into[620..620 + 4]);
        FromIntoMemory::into_bytes(self.dwAllocateCount, &mut into[624..624 + 4]);
    }
    fn size() -> usize {
        628
    }
}
pub struct NTMS_I1_PARTITIONINFORMATIONW {
    pub PhysicalMedia: crate::core::GUID,
    pub LogicalMedia: crate::core::GUID,
    pub State: u32,
    pub Side: u16,
    pub dwOmidLabelIdLength: u32,
    pub OmidLabelId: [u8; 255],
    pub szOmidLabelType: [u16; 64],
    pub szOmidLabelInfo: [u16; 256],
    pub dwMountCount: u32,
    pub dwAllocateCount: u32,
}
impl ::core::marker::Copy for NTMS_I1_PARTITIONINFORMATIONW {}
impl ::core::clone::Clone for NTMS_I1_PARTITIONINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_I1_PARTITIONINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_I1_PARTITIONINFORMATIONW")
            .field("PhysicalMedia", &self.PhysicalMedia)
            .field("LogicalMedia", &self.LogicalMedia)
            .field("State", &self.State)
            .field("Side", &self.Side)
            .field("dwOmidLabelIdLength", &self.dwOmidLabelIdLength)
            .field("OmidLabelId", &self.OmidLabelId)
            .field("szOmidLabelType", &self.szOmidLabelType)
            .field("szOmidLabelInfo", &self.szOmidLabelInfo)
            .field("dwMountCount", &self.dwMountCount)
            .field("dwAllocateCount", &self.dwAllocateCount)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_I1_PARTITIONINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.PhysicalMedia == other.PhysicalMedia
            && self.LogicalMedia == other.LogicalMedia
            && self.State == other.State
            && self.Side == other.Side
            && self.dwOmidLabelIdLength == other.dwOmidLabelIdLength
            && self.OmidLabelId == other.OmidLabelId
            && self.szOmidLabelType == other.szOmidLabelType
            && self.szOmidLabelInfo == other.szOmidLabelInfo
            && self.dwMountCount == other.dwMountCount
            && self.dwAllocateCount == other.dwAllocateCount
    }
}
impl ::core::cmp::Eq for NTMS_I1_PARTITIONINFORMATIONW {}
impl FromIntoMemory for NTMS_I1_PARTITIONINFORMATIONW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 628);
        let f_PhysicalMedia = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_LogicalMedia = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[16..16 + 16]);
        let f_State = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_Side = <u16 as FromIntoMemory>::from_bytes(&from[36..36 + 2]);
        let f_dwOmidLabelIdLength = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_OmidLabelId = <[u8; 255] as FromIntoMemory>::from_bytes(&from[44..44 + 255]);
        let f_szOmidLabelType = <[u16; 64] as FromIntoMemory>::from_bytes(&from[299..299 + 64]);
        let f_szOmidLabelInfo = <[u16; 256] as FromIntoMemory>::from_bytes(&from[363..363 + 256]);
        let f_dwMountCount = <u32 as FromIntoMemory>::from_bytes(&from[620..620 + 4]);
        let f_dwAllocateCount = <u32 as FromIntoMemory>::from_bytes(&from[624..624 + 4]);
        Self {
            PhysicalMedia: f_PhysicalMedia,
            LogicalMedia: f_LogicalMedia,
            State: f_State,
            Side: f_Side,
            dwOmidLabelIdLength: f_dwOmidLabelIdLength,
            OmidLabelId: f_OmidLabelId,
            szOmidLabelType: f_szOmidLabelType,
            szOmidLabelInfo: f_szOmidLabelInfo,
            dwMountCount: f_dwMountCount,
            dwAllocateCount: f_dwAllocateCount,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 628);
        FromIntoMemory::into_bytes(self.PhysicalMedia, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.LogicalMedia, &mut into[16..16 + 16]);
        FromIntoMemory::into_bytes(self.State, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.Side, &mut into[36..36 + 2]);
        FromIntoMemory::into_bytes(self.dwOmidLabelIdLength, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.OmidLabelId, &mut into[44..44 + 255]);
        FromIntoMemory::into_bytes(self.szOmidLabelType, &mut into[299..299 + 64]);
        FromIntoMemory::into_bytes(self.szOmidLabelInfo, &mut into[363..363 + 256]);
        FromIntoMemory::into_bytes(self.dwMountCount, &mut into[620..620 + 4]);
        FromIntoMemory::into_bytes(self.dwAllocateCount, &mut into[624..624 + 4]);
    }
    fn size() -> usize {
        628
    }
}
pub struct NTMS_I1_PMIDINFORMATIONA {
    pub CurrentLibrary: crate::core::GUID,
    pub MediaPool: crate::core::GUID,
    pub Location: crate::core::GUID,
    pub LocationType: u32,
    pub MediaType: crate::core::GUID,
    pub HomeSlot: crate::core::GUID,
    pub szBarCode: [super::super::Foundation::CHAR; 64],
    pub BarCodeState: u32,
    pub szSequenceNumber: [super::super::Foundation::CHAR; 32],
    pub MediaState: u32,
    pub dwNumberOfPartitions: u32,
}
impl ::core::marker::Copy for NTMS_I1_PMIDINFORMATIONA {}
impl ::core::clone::Clone for NTMS_I1_PMIDINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_I1_PMIDINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_I1_PMIDINFORMATIONA")
            .field("CurrentLibrary", &self.CurrentLibrary)
            .field("MediaPool", &self.MediaPool)
            .field("Location", &self.Location)
            .field("LocationType", &self.LocationType)
            .field("MediaType", &self.MediaType)
            .field("HomeSlot", &self.HomeSlot)
            .field("szBarCode", &self.szBarCode)
            .field("BarCodeState", &self.BarCodeState)
            .field("szSequenceNumber", &self.szSequenceNumber)
            .field("MediaState", &self.MediaState)
            .field("dwNumberOfPartitions", &self.dwNumberOfPartitions)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_I1_PMIDINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.CurrentLibrary == other.CurrentLibrary
            && self.MediaPool == other.MediaPool
            && self.Location == other.Location
            && self.LocationType == other.LocationType
            && self.MediaType == other.MediaType
            && self.HomeSlot == other.HomeSlot
            && self.szBarCode == other.szBarCode
            && self.BarCodeState == other.BarCodeState
            && self.szSequenceNumber == other.szSequenceNumber
            && self.MediaState == other.MediaState
            && self.dwNumberOfPartitions == other.dwNumberOfPartitions
    }
}
impl ::core::cmp::Eq for NTMS_I1_PMIDINFORMATIONA {}
impl FromIntoMemory for NTMS_I1_PMIDINFORMATIONA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 192);
        let f_CurrentLibrary = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_MediaPool = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[16..16 + 16]);
        let f_Location = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[32..32 + 16]);
        let f_LocationType = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_MediaType = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[52..52 + 16]);
        let f_HomeSlot = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[68..68 + 16]);
        let f_szBarCode = <[super::super::Foundation::CHAR; 64] as FromIntoMemory>::from_bytes(
            &from[84..84 + 64],
        );
        let f_BarCodeState = <u32 as FromIntoMemory>::from_bytes(&from[148..148 + 4]);
        let f_szSequenceNumber =
            <[super::super::Foundation::CHAR; 32] as FromIntoMemory>::from_bytes(
                &from[152..152 + 32],
            );
        let f_MediaState = <u32 as FromIntoMemory>::from_bytes(&from[184..184 + 4]);
        let f_dwNumberOfPartitions = <u32 as FromIntoMemory>::from_bytes(&from[188..188 + 4]);
        Self {
            CurrentLibrary: f_CurrentLibrary,
            MediaPool: f_MediaPool,
            Location: f_Location,
            LocationType: f_LocationType,
            MediaType: f_MediaType,
            HomeSlot: f_HomeSlot,
            szBarCode: f_szBarCode,
            BarCodeState: f_BarCodeState,
            szSequenceNumber: f_szSequenceNumber,
            MediaState: f_MediaState,
            dwNumberOfPartitions: f_dwNumberOfPartitions,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 192);
        FromIntoMemory::into_bytes(self.CurrentLibrary, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.MediaPool, &mut into[16..16 + 16]);
        FromIntoMemory::into_bytes(self.Location, &mut into[32..32 + 16]);
        FromIntoMemory::into_bytes(self.LocationType, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.MediaType, &mut into[52..52 + 16]);
        FromIntoMemory::into_bytes(self.HomeSlot, &mut into[68..68 + 16]);
        FromIntoMemory::into_bytes(self.szBarCode, &mut into[84..84 + 64]);
        FromIntoMemory::into_bytes(self.BarCodeState, &mut into[148..148 + 4]);
        FromIntoMemory::into_bytes(self.szSequenceNumber, &mut into[152..152 + 32]);
        FromIntoMemory::into_bytes(self.MediaState, &mut into[184..184 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfPartitions, &mut into[188..188 + 4]);
    }
    fn size() -> usize {
        192
    }
}
pub struct NTMS_I1_PMIDINFORMATIONW {
    pub CurrentLibrary: crate::core::GUID,
    pub MediaPool: crate::core::GUID,
    pub Location: crate::core::GUID,
    pub LocationType: u32,
    pub MediaType: crate::core::GUID,
    pub HomeSlot: crate::core::GUID,
    pub szBarCode: [u16; 64],
    pub BarCodeState: u32,
    pub szSequenceNumber: [u16; 32],
    pub MediaState: u32,
    pub dwNumberOfPartitions: u32,
}
impl ::core::marker::Copy for NTMS_I1_PMIDINFORMATIONW {}
impl ::core::clone::Clone for NTMS_I1_PMIDINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_I1_PMIDINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_I1_PMIDINFORMATIONW")
            .field("CurrentLibrary", &self.CurrentLibrary)
            .field("MediaPool", &self.MediaPool)
            .field("Location", &self.Location)
            .field("LocationType", &self.LocationType)
            .field("MediaType", &self.MediaType)
            .field("HomeSlot", &self.HomeSlot)
            .field("szBarCode", &self.szBarCode)
            .field("BarCodeState", &self.BarCodeState)
            .field("szSequenceNumber", &self.szSequenceNumber)
            .field("MediaState", &self.MediaState)
            .field("dwNumberOfPartitions", &self.dwNumberOfPartitions)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_I1_PMIDINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.CurrentLibrary == other.CurrentLibrary
            && self.MediaPool == other.MediaPool
            && self.Location == other.Location
            && self.LocationType == other.LocationType
            && self.MediaType == other.MediaType
            && self.HomeSlot == other.HomeSlot
            && self.szBarCode == other.szBarCode
            && self.BarCodeState == other.BarCodeState
            && self.szSequenceNumber == other.szSequenceNumber
            && self.MediaState == other.MediaState
            && self.dwNumberOfPartitions == other.dwNumberOfPartitions
    }
}
impl ::core::cmp::Eq for NTMS_I1_PMIDINFORMATIONW {}
impl FromIntoMemory for NTMS_I1_PMIDINFORMATIONW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 192);
        let f_CurrentLibrary = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_MediaPool = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[16..16 + 16]);
        let f_Location = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[32..32 + 16]);
        let f_LocationType = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_MediaType = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[52..52 + 16]);
        let f_HomeSlot = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[68..68 + 16]);
        let f_szBarCode = <[u16; 64] as FromIntoMemory>::from_bytes(&from[84..84 + 64]);
        let f_BarCodeState = <u32 as FromIntoMemory>::from_bytes(&from[148..148 + 4]);
        let f_szSequenceNumber = <[u16; 32] as FromIntoMemory>::from_bytes(&from[152..152 + 32]);
        let f_MediaState = <u32 as FromIntoMemory>::from_bytes(&from[184..184 + 4]);
        let f_dwNumberOfPartitions = <u32 as FromIntoMemory>::from_bytes(&from[188..188 + 4]);
        Self {
            CurrentLibrary: f_CurrentLibrary,
            MediaPool: f_MediaPool,
            Location: f_Location,
            LocationType: f_LocationType,
            MediaType: f_MediaType,
            HomeSlot: f_HomeSlot,
            szBarCode: f_szBarCode,
            BarCodeState: f_BarCodeState,
            szSequenceNumber: f_szSequenceNumber,
            MediaState: f_MediaState,
            dwNumberOfPartitions: f_dwNumberOfPartitions,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 192);
        FromIntoMemory::into_bytes(self.CurrentLibrary, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.MediaPool, &mut into[16..16 + 16]);
        FromIntoMemory::into_bytes(self.Location, &mut into[32..32 + 16]);
        FromIntoMemory::into_bytes(self.LocationType, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.MediaType, &mut into[52..52 + 16]);
        FromIntoMemory::into_bytes(self.HomeSlot, &mut into[68..68 + 16]);
        FromIntoMemory::into_bytes(self.szBarCode, &mut into[84..84 + 64]);
        FromIntoMemory::into_bytes(self.BarCodeState, &mut into[148..148 + 4]);
        FromIntoMemory::into_bytes(self.szSequenceNumber, &mut into[152..152 + 32]);
        FromIntoMemory::into_bytes(self.MediaState, &mut into[184..184 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfPartitions, &mut into[188..188 + 4]);
    }
    fn size() -> usize {
        192
    }
}
pub struct NTMS_IEDOORINFORMATION {
    pub Number: u32,
    pub State: NtmsDoorState,
    pub MaxOpenSecs: u16,
    pub Library: crate::core::GUID,
}
impl ::core::marker::Copy for NTMS_IEDOORINFORMATION {}
impl ::core::clone::Clone for NTMS_IEDOORINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_IEDOORINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_IEDOORINFORMATION")
            .field("Number", &self.Number)
            .field("State", &self.State)
            .field("MaxOpenSecs", &self.MaxOpenSecs)
            .field("Library", &self.Library)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_IEDOORINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number
            && self.State == other.State
            && self.MaxOpenSecs == other.MaxOpenSecs
            && self.Library == other.Library
    }
}
impl ::core::cmp::Eq for NTMS_IEDOORINFORMATION {}
impl FromIntoMemory for NTMS_IEDOORINFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_Number = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_State = <NtmsDoorState as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_MaxOpenSecs = <u16 as FromIntoMemory>::from_bytes(&from[8..8 + 2]);
        let f_Library = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[12..12 + 16]);
        Self {
            Number: f_Number,
            State: f_State,
            MaxOpenSecs: f_MaxOpenSecs,
            Library: f_Library,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.Number, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.State, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.MaxOpenSecs, &mut into[8..8 + 2]);
        FromIntoMemory::into_bytes(self.Library, &mut into[12..12 + 16]);
    }
    fn size() -> usize {
        28
    }
}
pub struct NTMS_IEPORTINFORMATION {
    pub Number: u32,
    pub Content: NtmsPortContent,
    pub Position: NtmsPortPosition,
    pub MaxExtendSecs: u16,
    pub Library: crate::core::GUID,
}
impl ::core::marker::Copy for NTMS_IEPORTINFORMATION {}
impl ::core::clone::Clone for NTMS_IEPORTINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_IEPORTINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_IEPORTINFORMATION")
            .field("Number", &self.Number)
            .field("Content", &self.Content)
            .field("Position", &self.Position)
            .field("MaxExtendSecs", &self.MaxExtendSecs)
            .field("Library", &self.Library)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_IEPORTINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number
            && self.Content == other.Content
            && self.Position == other.Position
            && self.MaxExtendSecs == other.MaxExtendSecs
            && self.Library == other.Library
    }
}
impl ::core::cmp::Eq for NTMS_IEPORTINFORMATION {}
impl FromIntoMemory for NTMS_IEPORTINFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_Number = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Content = <NtmsPortContent as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Position = <NtmsPortPosition as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_MaxExtendSecs = <u16 as FromIntoMemory>::from_bytes(&from[12..12 + 2]);
        let f_Library = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[16..16 + 16]);
        Self {
            Number: f_Number,
            Content: f_Content,
            Position: f_Position,
            MaxExtendSecs: f_MaxExtendSecs,
            Library: f_Library,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.Number, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Content, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Position, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.MaxExtendSecs, &mut into[12..12 + 2]);
        FromIntoMemory::into_bytes(self.Library, &mut into[16..16 + 16]);
    }
    fn size() -> usize {
        32
    }
}
pub struct NTMS_LIBRARYINFORMATION {
    pub LibraryType: NtmsLibraryType,
    pub CleanerSlot: crate::core::GUID,
    pub CleanerSlotDefault: crate::core::GUID,
    pub LibrarySupportsDriveCleaning: super::super::Foundation::BOOL,
    pub BarCodeReaderInstalled: super::super::Foundation::BOOL,
    pub InventoryMethod: NtmsInventoryMethod,
    pub dwCleanerUsesRemaining: u32,
    pub FirstDriveNumber: u32,
    pub dwNumberOfDrives: u32,
    pub FirstSlotNumber: u32,
    pub dwNumberOfSlots: u32,
    pub FirstDoorNumber: u32,
    pub dwNumberOfDoors: u32,
    pub FirstPortNumber: u32,
    pub dwNumberOfPorts: u32,
    pub FirstChangerNumber: u32,
    pub dwNumberOfChangers: u32,
    pub dwNumberOfMedia: u32,
    pub dwNumberOfMediaTypes: u32,
    pub dwNumberOfLibRequests: u32,
    pub Reserved: crate::core::GUID,
    pub AutoRecovery: super::super::Foundation::BOOL,
    pub dwFlags: NtmsLibraryFlags,
}
impl ::core::marker::Copy for NTMS_LIBRARYINFORMATION {}
impl ::core::clone::Clone for NTMS_LIBRARYINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_LIBRARYINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_LIBRARYINFORMATION")
            .field("LibraryType", &self.LibraryType)
            .field("CleanerSlot", &self.CleanerSlot)
            .field("CleanerSlotDefault", &self.CleanerSlotDefault)
            .field(
                "LibrarySupportsDriveCleaning",
                &self.LibrarySupportsDriveCleaning,
            )
            .field("BarCodeReaderInstalled", &self.BarCodeReaderInstalled)
            .field("InventoryMethod", &self.InventoryMethod)
            .field("dwCleanerUsesRemaining", &self.dwCleanerUsesRemaining)
            .field("FirstDriveNumber", &self.FirstDriveNumber)
            .field("dwNumberOfDrives", &self.dwNumberOfDrives)
            .field("FirstSlotNumber", &self.FirstSlotNumber)
            .field("dwNumberOfSlots", &self.dwNumberOfSlots)
            .field("FirstDoorNumber", &self.FirstDoorNumber)
            .field("dwNumberOfDoors", &self.dwNumberOfDoors)
            .field("FirstPortNumber", &self.FirstPortNumber)
            .field("dwNumberOfPorts", &self.dwNumberOfPorts)
            .field("FirstChangerNumber", &self.FirstChangerNumber)
            .field("dwNumberOfChangers", &self.dwNumberOfChangers)
            .field("dwNumberOfMedia", &self.dwNumberOfMedia)
            .field("dwNumberOfMediaTypes", &self.dwNumberOfMediaTypes)
            .field("dwNumberOfLibRequests", &self.dwNumberOfLibRequests)
            .field("Reserved", &self.Reserved)
            .field("AutoRecovery", &self.AutoRecovery)
            .field("dwFlags", &self.dwFlags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_LIBRARYINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.LibraryType == other.LibraryType
            && self.CleanerSlot == other.CleanerSlot
            && self.CleanerSlotDefault == other.CleanerSlotDefault
            && self.LibrarySupportsDriveCleaning == other.LibrarySupportsDriveCleaning
            && self.BarCodeReaderInstalled == other.BarCodeReaderInstalled
            && self.InventoryMethod == other.InventoryMethod
            && self.dwCleanerUsesRemaining == other.dwCleanerUsesRemaining
            && self.FirstDriveNumber == other.FirstDriveNumber
            && self.dwNumberOfDrives == other.dwNumberOfDrives
            && self.FirstSlotNumber == other.FirstSlotNumber
            && self.dwNumberOfSlots == other.dwNumberOfSlots
            && self.FirstDoorNumber == other.FirstDoorNumber
            && self.dwNumberOfDoors == other.dwNumberOfDoors
            && self.FirstPortNumber == other.FirstPortNumber
            && self.dwNumberOfPorts == other.dwNumberOfPorts
            && self.FirstChangerNumber == other.FirstChangerNumber
            && self.dwNumberOfChangers == other.dwNumberOfChangers
            && self.dwNumberOfMedia == other.dwNumberOfMedia
            && self.dwNumberOfMediaTypes == other.dwNumberOfMediaTypes
            && self.dwNumberOfLibRequests == other.dwNumberOfLibRequests
            && self.Reserved == other.Reserved
            && self.AutoRecovery == other.AutoRecovery
            && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for NTMS_LIBRARYINFORMATION {}
impl FromIntoMemory for NTMS_LIBRARYINFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 128);
        let f_LibraryType = <NtmsLibraryType as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_CleanerSlot = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[4..4 + 16]);
        let f_CleanerSlotDefault =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[20..20 + 16]);
        let f_LibrarySupportsDriveCleaning =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_BarCodeReaderInstalled =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_InventoryMethod =
            <NtmsInventoryMethod as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_dwCleanerUsesRemaining = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_FirstDriveNumber = <u32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_dwNumberOfDrives = <u32 as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_FirstSlotNumber = <u32 as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_dwNumberOfSlots = <u32 as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        let f_FirstDoorNumber = <u32 as FromIntoMemory>::from_bytes(&from[68..68 + 4]);
        let f_dwNumberOfDoors = <u32 as FromIntoMemory>::from_bytes(&from[72..72 + 4]);
        let f_FirstPortNumber = <u32 as FromIntoMemory>::from_bytes(&from[76..76 + 4]);
        let f_dwNumberOfPorts = <u32 as FromIntoMemory>::from_bytes(&from[80..80 + 4]);
        let f_FirstChangerNumber = <u32 as FromIntoMemory>::from_bytes(&from[84..84 + 4]);
        let f_dwNumberOfChangers = <u32 as FromIntoMemory>::from_bytes(&from[88..88 + 4]);
        let f_dwNumberOfMedia = <u32 as FromIntoMemory>::from_bytes(&from[92..92 + 4]);
        let f_dwNumberOfMediaTypes = <u32 as FromIntoMemory>::from_bytes(&from[96..96 + 4]);
        let f_dwNumberOfLibRequests = <u32 as FromIntoMemory>::from_bytes(&from[100..100 + 4]);
        let f_Reserved = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[104..104 + 16]);
        let f_AutoRecovery =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[120..120 + 4]);
        let f_dwFlags = <NtmsLibraryFlags as FromIntoMemory>::from_bytes(&from[124..124 + 4]);
        Self {
            LibraryType: f_LibraryType,
            CleanerSlot: f_CleanerSlot,
            CleanerSlotDefault: f_CleanerSlotDefault,
            LibrarySupportsDriveCleaning: f_LibrarySupportsDriveCleaning,
            BarCodeReaderInstalled: f_BarCodeReaderInstalled,
            InventoryMethod: f_InventoryMethod,
            dwCleanerUsesRemaining: f_dwCleanerUsesRemaining,
            FirstDriveNumber: f_FirstDriveNumber,
            dwNumberOfDrives: f_dwNumberOfDrives,
            FirstSlotNumber: f_FirstSlotNumber,
            dwNumberOfSlots: f_dwNumberOfSlots,
            FirstDoorNumber: f_FirstDoorNumber,
            dwNumberOfDoors: f_dwNumberOfDoors,
            FirstPortNumber: f_FirstPortNumber,
            dwNumberOfPorts: f_dwNumberOfPorts,
            FirstChangerNumber: f_FirstChangerNumber,
            dwNumberOfChangers: f_dwNumberOfChangers,
            dwNumberOfMedia: f_dwNumberOfMedia,
            dwNumberOfMediaTypes: f_dwNumberOfMediaTypes,
            dwNumberOfLibRequests: f_dwNumberOfLibRequests,
            Reserved: f_Reserved,
            AutoRecovery: f_AutoRecovery,
            dwFlags: f_dwFlags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 128);
        FromIntoMemory::into_bytes(self.LibraryType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.CleanerSlot, &mut into[4..4 + 16]);
        FromIntoMemory::into_bytes(self.CleanerSlotDefault, &mut into[20..20 + 16]);
        FromIntoMemory::into_bytes(self.LibrarySupportsDriveCleaning, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.BarCodeReaderInstalled, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.InventoryMethod, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.dwCleanerUsesRemaining, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.FirstDriveNumber, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfDrives, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.FirstSlotNumber, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfSlots, &mut into[64..64 + 4]);
        FromIntoMemory::into_bytes(self.FirstDoorNumber, &mut into[68..68 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfDoors, &mut into[72..72 + 4]);
        FromIntoMemory::into_bytes(self.FirstPortNumber, &mut into[76..76 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfPorts, &mut into[80..80 + 4]);
        FromIntoMemory::into_bytes(self.FirstChangerNumber, &mut into[84..84 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfChangers, &mut into[88..88 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfMedia, &mut into[92..92 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfMediaTypes, &mut into[96..96 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfLibRequests, &mut into[100..100 + 4]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[104..104 + 16]);
        FromIntoMemory::into_bytes(self.AutoRecovery, &mut into[120..120 + 4]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[124..124 + 4]);
    }
    fn size() -> usize {
        128
    }
}
pub struct NTMS_LIBREQUESTINFORMATIONA {
    pub OperationCode: NtmsLmOperation,
    pub OperationOption: u32,
    pub State: NtmsLmState,
    pub PartitionId: crate::core::GUID,
    pub DriveId: crate::core::GUID,
    pub PhysMediaId: crate::core::GUID,
    pub Library: crate::core::GUID,
    pub SlotId: crate::core::GUID,
    pub TimeQueued: super::super::Foundation::SYSTEMTIME,
    pub TimeCompleted: super::super::Foundation::SYSTEMTIME,
    pub szApplication: [super::super::Foundation::CHAR; 64],
    pub szUser: [super::super::Foundation::CHAR; 64],
    pub szComputer: [super::super::Foundation::CHAR; 64],
    pub dwErrorCode: u32,
    pub WorkItemId: crate::core::GUID,
    pub dwPriority: u32,
}
impl ::core::marker::Copy for NTMS_LIBREQUESTINFORMATIONA {}
impl ::core::clone::Clone for NTMS_LIBREQUESTINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_LIBREQUESTINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_LIBREQUESTINFORMATIONA")
            .field("OperationCode", &self.OperationCode)
            .field("OperationOption", &self.OperationOption)
            .field("State", &self.State)
            .field("PartitionId", &self.PartitionId)
            .field("DriveId", &self.DriveId)
            .field("PhysMediaId", &self.PhysMediaId)
            .field("Library", &self.Library)
            .field("SlotId", &self.SlotId)
            .field("TimeQueued", &self.TimeQueued)
            .field("TimeCompleted", &self.TimeCompleted)
            .field("szApplication", &self.szApplication)
            .field("szUser", &self.szUser)
            .field("szComputer", &self.szComputer)
            .field("dwErrorCode", &self.dwErrorCode)
            .field("WorkItemId", &self.WorkItemId)
            .field("dwPriority", &self.dwPriority)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_LIBREQUESTINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.OperationCode == other.OperationCode
            && self.OperationOption == other.OperationOption
            && self.State == other.State
            && self.PartitionId == other.PartitionId
            && self.DriveId == other.DriveId
            && self.PhysMediaId == other.PhysMediaId
            && self.Library == other.Library
            && self.SlotId == other.SlotId
            && self.TimeQueued == other.TimeQueued
            && self.TimeCompleted == other.TimeCompleted
            && self.szApplication == other.szApplication
            && self.szUser == other.szUser
            && self.szComputer == other.szComputer
            && self.dwErrorCode == other.dwErrorCode
            && self.WorkItemId == other.WorkItemId
            && self.dwPriority == other.dwPriority
    }
}
impl ::core::cmp::Eq for NTMS_LIBREQUESTINFORMATIONA {}
impl FromIntoMemory for NTMS_LIBREQUESTINFORMATIONA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 340);
        let f_OperationCode = <NtmsLmOperation as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_OperationOption = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_State = <NtmsLmState as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_PartitionId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[12..12 + 16]);
        let f_DriveId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[28..28 + 16]);
        let f_PhysMediaId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[44..44 + 16]);
        let f_Library = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[60..60 + 16]);
        let f_SlotId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[76..76 + 16]);
        let f_TimeQueued = <super::super::Foundation::SYSTEMTIME as FromIntoMemory>::from_bytes(
            &from[92..92 + 16],
        );
        let f_TimeCompleted = <super::super::Foundation::SYSTEMTIME as FromIntoMemory>::from_bytes(
            &from[108..108 + 16],
        );
        let f_szApplication = <[super::super::Foundation::CHAR; 64] as FromIntoMemory>::from_bytes(
            &from[124..124 + 64],
        );
        let f_szUser = <[super::super::Foundation::CHAR; 64] as FromIntoMemory>::from_bytes(
            &from[188..188 + 64],
        );
        let f_szComputer = <[super::super::Foundation::CHAR; 64] as FromIntoMemory>::from_bytes(
            &from[252..252 + 64],
        );
        let f_dwErrorCode = <u32 as FromIntoMemory>::from_bytes(&from[316..316 + 4]);
        let f_WorkItemId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[320..320 + 16]);
        let f_dwPriority = <u32 as FromIntoMemory>::from_bytes(&from[336..336 + 4]);
        Self {
            OperationCode: f_OperationCode,
            OperationOption: f_OperationOption,
            State: f_State,
            PartitionId: f_PartitionId,
            DriveId: f_DriveId,
            PhysMediaId: f_PhysMediaId,
            Library: f_Library,
            SlotId: f_SlotId,
            TimeQueued: f_TimeQueued,
            TimeCompleted: f_TimeCompleted,
            szApplication: f_szApplication,
            szUser: f_szUser,
            szComputer: f_szComputer,
            dwErrorCode: f_dwErrorCode,
            WorkItemId: f_WorkItemId,
            dwPriority: f_dwPriority,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 340);
        FromIntoMemory::into_bytes(self.OperationCode, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.OperationOption, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.State, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.PartitionId, &mut into[12..12 + 16]);
        FromIntoMemory::into_bytes(self.DriveId, &mut into[28..28 + 16]);
        FromIntoMemory::into_bytes(self.PhysMediaId, &mut into[44..44 + 16]);
        FromIntoMemory::into_bytes(self.Library, &mut into[60..60 + 16]);
        FromIntoMemory::into_bytes(self.SlotId, &mut into[76..76 + 16]);
        FromIntoMemory::into_bytes(self.TimeQueued, &mut into[92..92 + 16]);
        FromIntoMemory::into_bytes(self.TimeCompleted, &mut into[108..108 + 16]);
        FromIntoMemory::into_bytes(self.szApplication, &mut into[124..124 + 64]);
        FromIntoMemory::into_bytes(self.szUser, &mut into[188..188 + 64]);
        FromIntoMemory::into_bytes(self.szComputer, &mut into[252..252 + 64]);
        FromIntoMemory::into_bytes(self.dwErrorCode, &mut into[316..316 + 4]);
        FromIntoMemory::into_bytes(self.WorkItemId, &mut into[320..320 + 16]);
        FromIntoMemory::into_bytes(self.dwPriority, &mut into[336..336 + 4]);
    }
    fn size() -> usize {
        340
    }
}
pub struct NTMS_LIBREQUESTINFORMATIONW {
    pub OperationCode: NtmsLmOperation,
    pub OperationOption: u32,
    pub State: NtmsLmState,
    pub PartitionId: crate::core::GUID,
    pub DriveId: crate::core::GUID,
    pub PhysMediaId: crate::core::GUID,
    pub Library: crate::core::GUID,
    pub SlotId: crate::core::GUID,
    pub TimeQueued: super::super::Foundation::SYSTEMTIME,
    pub TimeCompleted: super::super::Foundation::SYSTEMTIME,
    pub szApplication: [u16; 64],
    pub szUser: [u16; 64],
    pub szComputer: [u16; 64],
    pub dwErrorCode: u32,
    pub WorkItemId: crate::core::GUID,
    pub dwPriority: u32,
}
impl ::core::marker::Copy for NTMS_LIBREQUESTINFORMATIONW {}
impl ::core::clone::Clone for NTMS_LIBREQUESTINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_LIBREQUESTINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_LIBREQUESTINFORMATIONW")
            .field("OperationCode", &self.OperationCode)
            .field("OperationOption", &self.OperationOption)
            .field("State", &self.State)
            .field("PartitionId", &self.PartitionId)
            .field("DriveId", &self.DriveId)
            .field("PhysMediaId", &self.PhysMediaId)
            .field("Library", &self.Library)
            .field("SlotId", &self.SlotId)
            .field("TimeQueued", &self.TimeQueued)
            .field("TimeCompleted", &self.TimeCompleted)
            .field("szApplication", &self.szApplication)
            .field("szUser", &self.szUser)
            .field("szComputer", &self.szComputer)
            .field("dwErrorCode", &self.dwErrorCode)
            .field("WorkItemId", &self.WorkItemId)
            .field("dwPriority", &self.dwPriority)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_LIBREQUESTINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.OperationCode == other.OperationCode
            && self.OperationOption == other.OperationOption
            && self.State == other.State
            && self.PartitionId == other.PartitionId
            && self.DriveId == other.DriveId
            && self.PhysMediaId == other.PhysMediaId
            && self.Library == other.Library
            && self.SlotId == other.SlotId
            && self.TimeQueued == other.TimeQueued
            && self.TimeCompleted == other.TimeCompleted
            && self.szApplication == other.szApplication
            && self.szUser == other.szUser
            && self.szComputer == other.szComputer
            && self.dwErrorCode == other.dwErrorCode
            && self.WorkItemId == other.WorkItemId
            && self.dwPriority == other.dwPriority
    }
}
impl ::core::cmp::Eq for NTMS_LIBREQUESTINFORMATIONW {}
impl FromIntoMemory for NTMS_LIBREQUESTINFORMATIONW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 340);
        let f_OperationCode = <NtmsLmOperation as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_OperationOption = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_State = <NtmsLmState as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_PartitionId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[12..12 + 16]);
        let f_DriveId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[28..28 + 16]);
        let f_PhysMediaId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[44..44 + 16]);
        let f_Library = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[60..60 + 16]);
        let f_SlotId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[76..76 + 16]);
        let f_TimeQueued = <super::super::Foundation::SYSTEMTIME as FromIntoMemory>::from_bytes(
            &from[92..92 + 16],
        );
        let f_TimeCompleted = <super::super::Foundation::SYSTEMTIME as FromIntoMemory>::from_bytes(
            &from[108..108 + 16],
        );
        let f_szApplication = <[u16; 64] as FromIntoMemory>::from_bytes(&from[124..124 + 64]);
        let f_szUser = <[u16; 64] as FromIntoMemory>::from_bytes(&from[188..188 + 64]);
        let f_szComputer = <[u16; 64] as FromIntoMemory>::from_bytes(&from[252..252 + 64]);
        let f_dwErrorCode = <u32 as FromIntoMemory>::from_bytes(&from[316..316 + 4]);
        let f_WorkItemId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[320..320 + 16]);
        let f_dwPriority = <u32 as FromIntoMemory>::from_bytes(&from[336..336 + 4]);
        Self {
            OperationCode: f_OperationCode,
            OperationOption: f_OperationOption,
            State: f_State,
            PartitionId: f_PartitionId,
            DriveId: f_DriveId,
            PhysMediaId: f_PhysMediaId,
            Library: f_Library,
            SlotId: f_SlotId,
            TimeQueued: f_TimeQueued,
            TimeCompleted: f_TimeCompleted,
            szApplication: f_szApplication,
            szUser: f_szUser,
            szComputer: f_szComputer,
            dwErrorCode: f_dwErrorCode,
            WorkItemId: f_WorkItemId,
            dwPriority: f_dwPriority,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 340);
        FromIntoMemory::into_bytes(self.OperationCode, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.OperationOption, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.State, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.PartitionId, &mut into[12..12 + 16]);
        FromIntoMemory::into_bytes(self.DriveId, &mut into[28..28 + 16]);
        FromIntoMemory::into_bytes(self.PhysMediaId, &mut into[44..44 + 16]);
        FromIntoMemory::into_bytes(self.Library, &mut into[60..60 + 16]);
        FromIntoMemory::into_bytes(self.SlotId, &mut into[76..76 + 16]);
        FromIntoMemory::into_bytes(self.TimeQueued, &mut into[92..92 + 16]);
        FromIntoMemory::into_bytes(self.TimeCompleted, &mut into[108..108 + 16]);
        FromIntoMemory::into_bytes(self.szApplication, &mut into[124..124 + 64]);
        FromIntoMemory::into_bytes(self.szUser, &mut into[188..188 + 64]);
        FromIntoMemory::into_bytes(self.szComputer, &mut into[252..252 + 64]);
        FromIntoMemory::into_bytes(self.dwErrorCode, &mut into[316..316 + 4]);
        FromIntoMemory::into_bytes(self.WorkItemId, &mut into[320..320 + 16]);
        FromIntoMemory::into_bytes(self.dwPriority, &mut into[336..336 + 4]);
    }
    fn size() -> usize {
        340
    }
}
pub struct NTMS_LMIDINFORMATION {
    pub MediaPool: crate::core::GUID,
    pub dwNumberOfPartitions: u32,
}
impl ::core::marker::Copy for NTMS_LMIDINFORMATION {}
impl ::core::clone::Clone for NTMS_LMIDINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_LMIDINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_LMIDINFORMATION")
            .field("MediaPool", &self.MediaPool)
            .field("dwNumberOfPartitions", &self.dwNumberOfPartitions)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_LMIDINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.MediaPool == other.MediaPool && self.dwNumberOfPartitions == other.dwNumberOfPartitions
    }
}
impl ::core::cmp::Eq for NTMS_LMIDINFORMATION {}
impl FromIntoMemory for NTMS_LMIDINFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_MediaPool = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_dwNumberOfPartitions = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            MediaPool: f_MediaPool,
            dwNumberOfPartitions: f_dwNumberOfPartitions,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.MediaPool, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.dwNumberOfPartitions, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub const NTMS_MAXATTR_LENGTH: u32 = 65536u32;
pub const NTMS_MAXATTR_NAMELEN: u32 = 32u32;
pub struct NTMS_MEDIAPOOLINFORMATION {
    pub PoolType: u32,
    pub MediaType: crate::core::GUID,
    pub Parent: crate::core::GUID,
    pub AllocationPolicy: u32,
    pub DeallocationPolicy: u32,
    pub dwMaxAllocates: u32,
    pub dwNumberOfPhysicalMedia: u32,
    pub dwNumberOfLogicalMedia: u32,
    pub dwNumberOfMediaPools: u32,
}
impl ::core::marker::Copy for NTMS_MEDIAPOOLINFORMATION {}
impl ::core::clone::Clone for NTMS_MEDIAPOOLINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_MEDIAPOOLINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_MEDIAPOOLINFORMATION")
            .field("PoolType", &self.PoolType)
            .field("MediaType", &self.MediaType)
            .field("Parent", &self.Parent)
            .field("AllocationPolicy", &self.AllocationPolicy)
            .field("DeallocationPolicy", &self.DeallocationPolicy)
            .field("dwMaxAllocates", &self.dwMaxAllocates)
            .field("dwNumberOfPhysicalMedia", &self.dwNumberOfPhysicalMedia)
            .field("dwNumberOfLogicalMedia", &self.dwNumberOfLogicalMedia)
            .field("dwNumberOfMediaPools", &self.dwNumberOfMediaPools)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_MEDIAPOOLINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.PoolType == other.PoolType
            && self.MediaType == other.MediaType
            && self.Parent == other.Parent
            && self.AllocationPolicy == other.AllocationPolicy
            && self.DeallocationPolicy == other.DeallocationPolicy
            && self.dwMaxAllocates == other.dwMaxAllocates
            && self.dwNumberOfPhysicalMedia == other.dwNumberOfPhysicalMedia
            && self.dwNumberOfLogicalMedia == other.dwNumberOfLogicalMedia
            && self.dwNumberOfMediaPools == other.dwNumberOfMediaPools
    }
}
impl ::core::cmp::Eq for NTMS_MEDIAPOOLINFORMATION {}
impl FromIntoMemory for NTMS_MEDIAPOOLINFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 60);
        let f_PoolType = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_MediaType = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[4..4 + 16]);
        let f_Parent = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[20..20 + 16]);
        let f_AllocationPolicy = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_DeallocationPolicy = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_dwMaxAllocates = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_dwNumberOfPhysicalMedia = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_dwNumberOfLogicalMedia = <u32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_dwNumberOfMediaPools = <u32 as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        Self {
            PoolType: f_PoolType,
            MediaType: f_MediaType,
            Parent: f_Parent,
            AllocationPolicy: f_AllocationPolicy,
            DeallocationPolicy: f_DeallocationPolicy,
            dwMaxAllocates: f_dwMaxAllocates,
            dwNumberOfPhysicalMedia: f_dwNumberOfPhysicalMedia,
            dwNumberOfLogicalMedia: f_dwNumberOfLogicalMedia,
            dwNumberOfMediaPools: f_dwNumberOfMediaPools,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 60);
        FromIntoMemory::into_bytes(self.PoolType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.MediaType, &mut into[4..4 + 16]);
        FromIntoMemory::into_bytes(self.Parent, &mut into[20..20 + 16]);
        FromIntoMemory::into_bytes(self.AllocationPolicy, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.DeallocationPolicy, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.dwMaxAllocates, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfPhysicalMedia, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfLogicalMedia, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfMediaPools, &mut into[56..56 + 4]);
    }
    fn size() -> usize {
        60
    }
}
pub struct NTMS_MEDIATYPEINFORMATION {
    pub MediaType: u32,
    pub NumberOfSides: u32,
    pub ReadWriteCharacteristics: NtmsReadWriteCharacteristics,
    pub DeviceType: FILE_DEVICE_TYPE,
}
impl ::core::marker::Copy for NTMS_MEDIATYPEINFORMATION {}
impl ::core::clone::Clone for NTMS_MEDIATYPEINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_MEDIATYPEINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_MEDIATYPEINFORMATION")
            .field("MediaType", &self.MediaType)
            .field("NumberOfSides", &self.NumberOfSides)
            .field("ReadWriteCharacteristics", &self.ReadWriteCharacteristics)
            .field("DeviceType", &self.DeviceType)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_MEDIATYPEINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.MediaType == other.MediaType
            && self.NumberOfSides == other.NumberOfSides
            && self.ReadWriteCharacteristics == other.ReadWriteCharacteristics
            && self.DeviceType == other.DeviceType
    }
}
impl ::core::cmp::Eq for NTMS_MEDIATYPEINFORMATION {}
impl FromIntoMemory for NTMS_MEDIATYPEINFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_MediaType = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_NumberOfSides = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_ReadWriteCharacteristics =
            <NtmsReadWriteCharacteristics as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_DeviceType = <FILE_DEVICE_TYPE as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            MediaType: f_MediaType,
            NumberOfSides: f_NumberOfSides,
            ReadWriteCharacteristics: f_ReadWriteCharacteristics,
            DeviceType: f_DeviceType,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.MediaType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.NumberOfSides, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.ReadWriteCharacteristics, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.DeviceType, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub const NTMS_MESSAGE_LENGTH: u32 = 256u32;
pub struct NTMS_MOUNT_INFORMATION {
    pub dwSize: u32,
    pub lpReserved: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for NTMS_MOUNT_INFORMATION {}
impl ::core::clone::Clone for NTMS_MOUNT_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_MOUNT_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_MOUNT_INFORMATION")
            .field("dwSize", &self.dwSize)
            .field("lpReserved", &self.lpReserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_MOUNT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.lpReserved == other.lpReserved
    }
}
impl ::core::cmp::Eq for NTMS_MOUNT_INFORMATION {}
impl FromIntoMemory for NTMS_MOUNT_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpReserved =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            dwSize: f_dwSize,
            lpReserved: f_lpReserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpReserved, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct NTMS_NOTIFICATIONINFORMATION {
    pub dwOperation: NtmsNotificationOperations,
    pub ObjectId: crate::core::GUID,
}
impl ::core::marker::Copy for NTMS_NOTIFICATIONINFORMATION {}
impl ::core::clone::Clone for NTMS_NOTIFICATIONINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_NOTIFICATIONINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_NOTIFICATIONINFORMATION")
            .field("dwOperation", &self.dwOperation)
            .field("ObjectId", &self.ObjectId)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_NOTIFICATIONINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwOperation == other.dwOperation && self.ObjectId == other.ObjectId
    }
}
impl ::core::cmp::Eq for NTMS_NOTIFICATIONINFORMATION {}
impl FromIntoMemory for NTMS_NOTIFICATIONINFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_dwOperation =
            <NtmsNotificationOperations as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ObjectId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[4..4 + 16]);
        Self {
            dwOperation: f_dwOperation,
            ObjectId: f_ObjectId,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.dwOperation, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ObjectId, &mut into[4..4 + 16]);
    }
    fn size() -> usize {
        20
    }
}
pub struct NTMS_OBJECTINFORMATIONA {
    pub dwSize: u32,
    pub dwType: NtmsObjectsTypes,
    pub Created: super::super::Foundation::SYSTEMTIME,
    pub Modified: super::super::Foundation::SYSTEMTIME,
    pub ObjectGuid: crate::core::GUID,
    pub Enabled: super::super::Foundation::BOOL,
    pub dwOperationalState: NtmsOperationalState,
    pub szName: [super::super::Foundation::CHAR; 64],
    pub szDescription: [super::super::Foundation::CHAR; 127],
    pub Info: NTMS_OBJECTINFORMATIONA_0,
}
impl ::core::marker::Copy for NTMS_OBJECTINFORMATIONA {}
impl ::core::clone::Clone for NTMS_OBJECTINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for NTMS_OBJECTINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwType == other.dwType
            && self.Created == other.Created
            && self.Modified == other.Modified
            && self.ObjectGuid == other.ObjectGuid
            && self.Enabled == other.Enabled
            && self.dwOperationalState == other.dwOperationalState
            && self.szName == other.szName
            && self.szDescription == other.szDescription
            && self.Info == other.Info
    }
}
impl ::core::cmp::Eq for NTMS_OBJECTINFORMATIONA {}
impl FromIntoMemory for NTMS_OBJECTINFORMATIONA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 280);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwType = <NtmsObjectsTypes as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Created =
            <super::super::Foundation::SYSTEMTIME as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        let f_Modified = <super::super::Foundation::SYSTEMTIME as FromIntoMemory>::from_bytes(
            &from[24..24 + 16],
        );
        let f_ObjectGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[40..40 + 16]);
        let f_Enabled =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_dwOperationalState =
            <NtmsOperationalState as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_szName = <[super::super::Foundation::CHAR; 64] as FromIntoMemory>::from_bytes(
            &from[64..64 + 64],
        );
        let f_szDescription = <[super::super::Foundation::CHAR; 127] as FromIntoMemory>::from_bytes(
            &from[128..128 + 127],
        );
        let f_Info =
            <NTMS_OBJECTINFORMATIONA_0 as FromIntoMemory>::from_bytes(&from[256..256 + 24]);
        Self {
            dwSize: f_dwSize,
            dwType: f_dwType,
            Created: f_Created,
            Modified: f_Modified,
            ObjectGuid: f_ObjectGuid,
            Enabled: f_Enabled,
            dwOperationalState: f_dwOperationalState,
            szName: f_szName,
            szDescription: f_szDescription,
            Info: f_Info,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 280);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwType, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Created, &mut into[8..8 + 16]);
        FromIntoMemory::into_bytes(self.Modified, &mut into[24..24 + 16]);
        FromIntoMemory::into_bytes(self.ObjectGuid, &mut into[40..40 + 16]);
        FromIntoMemory::into_bytes(self.Enabled, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.dwOperationalState, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.szName, &mut into[64..64 + 64]);
        FromIntoMemory::into_bytes(self.szDescription, &mut into[128..128 + 127]);
        FromIntoMemory::into_bytes(self.Info, &mut into[256..256 + 24]);
    }
    fn size() -> usize {
        280
    }
}
pub struct NTMS_OBJECTINFORMATIONA_0 {
    data: [u8; 24],
}
impl ::core::default::Default for NTMS_OBJECTINFORMATIONA_0 {
    fn default() -> Self {
        Self { data: [0u8; 24] }
    }
}
impl ::core::marker::Copy for NTMS_OBJECTINFORMATIONA_0 {}
impl ::core::clone::Clone for NTMS_OBJECTINFORMATIONA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for NTMS_OBJECTINFORMATIONA_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for NTMS_OBJECTINFORMATIONA_0 {}
impl FromIntoMemory for NTMS_OBJECTINFORMATIONA_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 24];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        24
    }
}
pub struct NTMS_OBJECTINFORMATIONW {
    pub dwSize: u32,
    pub dwType: NtmsObjectsTypes,
    pub Created: super::super::Foundation::SYSTEMTIME,
    pub Modified: super::super::Foundation::SYSTEMTIME,
    pub ObjectGuid: crate::core::GUID,
    pub Enabled: super::super::Foundation::BOOL,
    pub dwOperationalState: NtmsOperationalState,
    pub szName: [u16; 64],
    pub szDescription: [u16; 127],
    pub Info: NTMS_OBJECTINFORMATIONW_0,
}
impl ::core::marker::Copy for NTMS_OBJECTINFORMATIONW {}
impl ::core::clone::Clone for NTMS_OBJECTINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for NTMS_OBJECTINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwType == other.dwType
            && self.Created == other.Created
            && self.Modified == other.Modified
            && self.ObjectGuid == other.ObjectGuid
            && self.Enabled == other.Enabled
            && self.dwOperationalState == other.dwOperationalState
            && self.szName == other.szName
            && self.szDescription == other.szDescription
            && self.Info == other.Info
    }
}
impl ::core::cmp::Eq for NTMS_OBJECTINFORMATIONW {}
impl FromIntoMemory for NTMS_OBJECTINFORMATIONW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 280);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwType = <NtmsObjectsTypes as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Created =
            <super::super::Foundation::SYSTEMTIME as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        let f_Modified = <super::super::Foundation::SYSTEMTIME as FromIntoMemory>::from_bytes(
            &from[24..24 + 16],
        );
        let f_ObjectGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[40..40 + 16]);
        let f_Enabled =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_dwOperationalState =
            <NtmsOperationalState as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_szName = <[u16; 64] as FromIntoMemory>::from_bytes(&from[64..64 + 64]);
        let f_szDescription = <[u16; 127] as FromIntoMemory>::from_bytes(&from[128..128 + 127]);
        let f_Info =
            <NTMS_OBJECTINFORMATIONW_0 as FromIntoMemory>::from_bytes(&from[256..256 + 24]);
        Self {
            dwSize: f_dwSize,
            dwType: f_dwType,
            Created: f_Created,
            Modified: f_Modified,
            ObjectGuid: f_ObjectGuid,
            Enabled: f_Enabled,
            dwOperationalState: f_dwOperationalState,
            szName: f_szName,
            szDescription: f_szDescription,
            Info: f_Info,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 280);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwType, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Created, &mut into[8..8 + 16]);
        FromIntoMemory::into_bytes(self.Modified, &mut into[24..24 + 16]);
        FromIntoMemory::into_bytes(self.ObjectGuid, &mut into[40..40 + 16]);
        FromIntoMemory::into_bytes(self.Enabled, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.dwOperationalState, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.szName, &mut into[64..64 + 64]);
        FromIntoMemory::into_bytes(self.szDescription, &mut into[128..128 + 127]);
        FromIntoMemory::into_bytes(self.Info, &mut into[256..256 + 24]);
    }
    fn size() -> usize {
        280
    }
}
pub struct NTMS_OBJECTINFORMATIONW_0 {
    data: [u8; 24],
}
impl ::core::default::Default for NTMS_OBJECTINFORMATIONW_0 {
    fn default() -> Self {
        Self { data: [0u8; 24] }
    }
}
impl ::core::marker::Copy for NTMS_OBJECTINFORMATIONW_0 {}
impl ::core::clone::Clone for NTMS_OBJECTINFORMATIONW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for NTMS_OBJECTINFORMATIONW_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for NTMS_OBJECTINFORMATIONW_0 {}
impl FromIntoMemory for NTMS_OBJECTINFORMATIONW_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 24];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        24
    }
}
pub const NTMS_OBJECTNAME_LENGTH: u32 = 64u32;
pub const NTMS_OMIDLABELID_LENGTH: u32 = 255u32;
pub const NTMS_OMIDLABELINFO_LENGTH: u32 = 256u32;
pub const NTMS_OMIDLABELTYPE_LENGTH: u32 = 64u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NTMS_OMID_TYPE(pub u32);
pub const NTMS_OMID_TYPE_FILESYSTEM_INFO: NTMS_OMID_TYPE = NTMS_OMID_TYPE(2u32);
pub const NTMS_OMID_TYPE_RAW_LABEL: NTMS_OMID_TYPE = NTMS_OMID_TYPE(1u32);
impl ::core::marker::Copy for NTMS_OMID_TYPE {}
impl ::core::clone::Clone for NTMS_OMID_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NTMS_OMID_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NTMS_OMID_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NTMS_OMID_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for NTMS_OMID_TYPE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct NTMS_OPREQUESTINFORMATIONA {
    pub Request: NtmsOpreqCommand,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub State: NtmsOpreqState,
    pub szMessage: [super::super::Foundation::CHAR; 256],
    pub Arg1Type: NtmsObjectsTypes,
    pub Arg1: crate::core::GUID,
    pub Arg2Type: NtmsObjectsTypes,
    pub Arg2: crate::core::GUID,
    pub szApplication: [super::super::Foundation::CHAR; 64],
    pub szUser: [super::super::Foundation::CHAR; 64],
    pub szComputer: [super::super::Foundation::CHAR; 64],
}
impl ::core::marker::Copy for NTMS_OPREQUESTINFORMATIONA {}
impl ::core::clone::Clone for NTMS_OPREQUESTINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_OPREQUESTINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_OPREQUESTINFORMATIONA")
            .field("Request", &self.Request)
            .field("Submitted", &self.Submitted)
            .field("State", &self.State)
            .field("szMessage", &self.szMessage)
            .field("Arg1Type", &self.Arg1Type)
            .field("Arg1", &self.Arg1)
            .field("Arg2Type", &self.Arg2Type)
            .field("Arg2", &self.Arg2)
            .field("szApplication", &self.szApplication)
            .field("szUser", &self.szUser)
            .field("szComputer", &self.szComputer)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_OPREQUESTINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.Request == other.Request
            && self.Submitted == other.Submitted
            && self.State == other.State
            && self.szMessage == other.szMessage
            && self.Arg1Type == other.Arg1Type
            && self.Arg1 == other.Arg1
            && self.Arg2Type == other.Arg2Type
            && self.Arg2 == other.Arg2
            && self.szApplication == other.szApplication
            && self.szUser == other.szUser
            && self.szComputer == other.szComputer
    }
}
impl ::core::cmp::Eq for NTMS_OPREQUESTINFORMATIONA {}
impl FromIntoMemory for NTMS_OPREQUESTINFORMATIONA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 512);
        let f_Request = <NtmsOpreqCommand as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Submitted =
            <super::super::Foundation::SYSTEMTIME as FromIntoMemory>::from_bytes(&from[4..4 + 16]);
        let f_State = <NtmsOpreqState as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_szMessage = <[super::super::Foundation::CHAR; 256] as FromIntoMemory>::from_bytes(
            &from[24..24 + 256],
        );
        let f_Arg1Type = <NtmsObjectsTypes as FromIntoMemory>::from_bytes(&from[280..280 + 4]);
        let f_Arg1 = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[284..284 + 16]);
        let f_Arg2Type = <NtmsObjectsTypes as FromIntoMemory>::from_bytes(&from[300..300 + 4]);
        let f_Arg2 = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[304..304 + 16]);
        let f_szApplication = <[super::super::Foundation::CHAR; 64] as FromIntoMemory>::from_bytes(
            &from[320..320 + 64],
        );
        let f_szUser = <[super::super::Foundation::CHAR; 64] as FromIntoMemory>::from_bytes(
            &from[384..384 + 64],
        );
        let f_szComputer = <[super::super::Foundation::CHAR; 64] as FromIntoMemory>::from_bytes(
            &from[448..448 + 64],
        );
        Self {
            Request: f_Request,
            Submitted: f_Submitted,
            State: f_State,
            szMessage: f_szMessage,
            Arg1Type: f_Arg1Type,
            Arg1: f_Arg1,
            Arg2Type: f_Arg2Type,
            Arg2: f_Arg2,
            szApplication: f_szApplication,
            szUser: f_szUser,
            szComputer: f_szComputer,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 512);
        FromIntoMemory::into_bytes(self.Request, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Submitted, &mut into[4..4 + 16]);
        FromIntoMemory::into_bytes(self.State, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.szMessage, &mut into[24..24 + 256]);
        FromIntoMemory::into_bytes(self.Arg1Type, &mut into[280..280 + 4]);
        FromIntoMemory::into_bytes(self.Arg1, &mut into[284..284 + 16]);
        FromIntoMemory::into_bytes(self.Arg2Type, &mut into[300..300 + 4]);
        FromIntoMemory::into_bytes(self.Arg2, &mut into[304..304 + 16]);
        FromIntoMemory::into_bytes(self.szApplication, &mut into[320..320 + 64]);
        FromIntoMemory::into_bytes(self.szUser, &mut into[384..384 + 64]);
        FromIntoMemory::into_bytes(self.szComputer, &mut into[448..448 + 64]);
    }
    fn size() -> usize {
        512
    }
}
pub struct NTMS_OPREQUESTINFORMATIONW {
    pub Request: NtmsOpreqCommand,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub State: NtmsOpreqState,
    pub szMessage: [u16; 256],
    pub Arg1Type: NtmsObjectsTypes,
    pub Arg1: crate::core::GUID,
    pub Arg2Type: NtmsObjectsTypes,
    pub Arg2: crate::core::GUID,
    pub szApplication: [u16; 64],
    pub szUser: [u16; 64],
    pub szComputer: [u16; 64],
}
impl ::core::marker::Copy for NTMS_OPREQUESTINFORMATIONW {}
impl ::core::clone::Clone for NTMS_OPREQUESTINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_OPREQUESTINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_OPREQUESTINFORMATIONW")
            .field("Request", &self.Request)
            .field("Submitted", &self.Submitted)
            .field("State", &self.State)
            .field("szMessage", &self.szMessage)
            .field("Arg1Type", &self.Arg1Type)
            .field("Arg1", &self.Arg1)
            .field("Arg2Type", &self.Arg2Type)
            .field("Arg2", &self.Arg2)
            .field("szApplication", &self.szApplication)
            .field("szUser", &self.szUser)
            .field("szComputer", &self.szComputer)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_OPREQUESTINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.Request == other.Request
            && self.Submitted == other.Submitted
            && self.State == other.State
            && self.szMessage == other.szMessage
            && self.Arg1Type == other.Arg1Type
            && self.Arg1 == other.Arg1
            && self.Arg2Type == other.Arg2Type
            && self.Arg2 == other.Arg2
            && self.szApplication == other.szApplication
            && self.szUser == other.szUser
            && self.szComputer == other.szComputer
    }
}
impl ::core::cmp::Eq for NTMS_OPREQUESTINFORMATIONW {}
impl FromIntoMemory for NTMS_OPREQUESTINFORMATIONW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 512);
        let f_Request = <NtmsOpreqCommand as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Submitted =
            <super::super::Foundation::SYSTEMTIME as FromIntoMemory>::from_bytes(&from[4..4 + 16]);
        let f_State = <NtmsOpreqState as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_szMessage = <[u16; 256] as FromIntoMemory>::from_bytes(&from[24..24 + 256]);
        let f_Arg1Type = <NtmsObjectsTypes as FromIntoMemory>::from_bytes(&from[280..280 + 4]);
        let f_Arg1 = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[284..284 + 16]);
        let f_Arg2Type = <NtmsObjectsTypes as FromIntoMemory>::from_bytes(&from[300..300 + 4]);
        let f_Arg2 = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[304..304 + 16]);
        let f_szApplication = <[u16; 64] as FromIntoMemory>::from_bytes(&from[320..320 + 64]);
        let f_szUser = <[u16; 64] as FromIntoMemory>::from_bytes(&from[384..384 + 64]);
        let f_szComputer = <[u16; 64] as FromIntoMemory>::from_bytes(&from[448..448 + 64]);
        Self {
            Request: f_Request,
            Submitted: f_Submitted,
            State: f_State,
            szMessage: f_szMessage,
            Arg1Type: f_Arg1Type,
            Arg1: f_Arg1,
            Arg2Type: f_Arg2Type,
            Arg2: f_Arg2,
            szApplication: f_szApplication,
            szUser: f_szUser,
            szComputer: f_szComputer,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 512);
        FromIntoMemory::into_bytes(self.Request, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Submitted, &mut into[4..4 + 16]);
        FromIntoMemory::into_bytes(self.State, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.szMessage, &mut into[24..24 + 256]);
        FromIntoMemory::into_bytes(self.Arg1Type, &mut into[280..280 + 4]);
        FromIntoMemory::into_bytes(self.Arg1, &mut into[284..284 + 16]);
        FromIntoMemory::into_bytes(self.Arg2Type, &mut into[300..300 + 4]);
        FromIntoMemory::into_bytes(self.Arg2, &mut into[304..304 + 16]);
        FromIntoMemory::into_bytes(self.szApplication, &mut into[320..320 + 64]);
        FromIntoMemory::into_bytes(self.szUser, &mut into[384..384 + 64]);
        FromIntoMemory::into_bytes(self.szComputer, &mut into[448..448 + 64]);
    }
    fn size() -> usize {
        512
    }
}
pub struct NTMS_PARTITIONINFORMATIONA {
    pub PhysicalMedia: crate::core::GUID,
    pub LogicalMedia: crate::core::GUID,
    pub State: NtmsPartitionState,
    pub Side: u16,
    pub dwOmidLabelIdLength: u32,
    pub OmidLabelId: [u8; 255],
    pub szOmidLabelType: [super::super::Foundation::CHAR; 64],
    pub szOmidLabelInfo: [super::super::Foundation::CHAR; 256],
    pub dwMountCount: u32,
    pub dwAllocateCount: u32,
    pub Capacity: i64,
}
impl ::core::marker::Copy for NTMS_PARTITIONINFORMATIONA {}
impl ::core::clone::Clone for NTMS_PARTITIONINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_PARTITIONINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_PARTITIONINFORMATIONA")
            .field("PhysicalMedia", &self.PhysicalMedia)
            .field("LogicalMedia", &self.LogicalMedia)
            .field("State", &self.State)
            .field("Side", &self.Side)
            .field("dwOmidLabelIdLength", &self.dwOmidLabelIdLength)
            .field("OmidLabelId", &self.OmidLabelId)
            .field("szOmidLabelType", &self.szOmidLabelType)
            .field("szOmidLabelInfo", &self.szOmidLabelInfo)
            .field("dwMountCount", &self.dwMountCount)
            .field("dwAllocateCount", &self.dwAllocateCount)
            .field("Capacity", &self.Capacity)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_PARTITIONINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.PhysicalMedia == other.PhysicalMedia
            && self.LogicalMedia == other.LogicalMedia
            && self.State == other.State
            && self.Side == other.Side
            && self.dwOmidLabelIdLength == other.dwOmidLabelIdLength
            && self.OmidLabelId == other.OmidLabelId
            && self.szOmidLabelType == other.szOmidLabelType
            && self.szOmidLabelInfo == other.szOmidLabelInfo
            && self.dwMountCount == other.dwMountCount
            && self.dwAllocateCount == other.dwAllocateCount
            && self.Capacity == other.Capacity
    }
}
impl ::core::cmp::Eq for NTMS_PARTITIONINFORMATIONA {}
impl FromIntoMemory for NTMS_PARTITIONINFORMATIONA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 640);
        let f_PhysicalMedia = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_LogicalMedia = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[16..16 + 16]);
        let f_State = <NtmsPartitionState as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_Side = <u16 as FromIntoMemory>::from_bytes(&from[36..36 + 2]);
        let f_dwOmidLabelIdLength = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_OmidLabelId = <[u8; 255] as FromIntoMemory>::from_bytes(&from[44..44 + 255]);
        let f_szOmidLabelType =
            <[super::super::Foundation::CHAR; 64] as FromIntoMemory>::from_bytes(
                &from[299..299 + 64],
            );
        let f_szOmidLabelInfo =
            <[super::super::Foundation::CHAR; 256] as FromIntoMemory>::from_bytes(
                &from[363..363 + 256],
            );
        let f_dwMountCount = <u32 as FromIntoMemory>::from_bytes(&from[620..620 + 4]);
        let f_dwAllocateCount = <u32 as FromIntoMemory>::from_bytes(&from[624..624 + 4]);
        let f_Capacity = <i64 as FromIntoMemory>::from_bytes(&from[632..632 + 8]);
        Self {
            PhysicalMedia: f_PhysicalMedia,
            LogicalMedia: f_LogicalMedia,
            State: f_State,
            Side: f_Side,
            dwOmidLabelIdLength: f_dwOmidLabelIdLength,
            OmidLabelId: f_OmidLabelId,
            szOmidLabelType: f_szOmidLabelType,
            szOmidLabelInfo: f_szOmidLabelInfo,
            dwMountCount: f_dwMountCount,
            dwAllocateCount: f_dwAllocateCount,
            Capacity: f_Capacity,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 640);
        FromIntoMemory::into_bytes(self.PhysicalMedia, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.LogicalMedia, &mut into[16..16 + 16]);
        FromIntoMemory::into_bytes(self.State, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.Side, &mut into[36..36 + 2]);
        FromIntoMemory::into_bytes(self.dwOmidLabelIdLength, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.OmidLabelId, &mut into[44..44 + 255]);
        FromIntoMemory::into_bytes(self.szOmidLabelType, &mut into[299..299 + 64]);
        FromIntoMemory::into_bytes(self.szOmidLabelInfo, &mut into[363..363 + 256]);
        FromIntoMemory::into_bytes(self.dwMountCount, &mut into[620..620 + 4]);
        FromIntoMemory::into_bytes(self.dwAllocateCount, &mut into[624..624 + 4]);
        FromIntoMemory::into_bytes(self.Capacity, &mut into[632..632 + 8]);
    }
    fn size() -> usize {
        640
    }
}
pub struct NTMS_PARTITIONINFORMATIONW {
    pub PhysicalMedia: crate::core::GUID,
    pub LogicalMedia: crate::core::GUID,
    pub State: NtmsPartitionState,
    pub Side: u16,
    pub dwOmidLabelIdLength: u32,
    pub OmidLabelId: [u8; 255],
    pub szOmidLabelType: [u16; 64],
    pub szOmidLabelInfo: [u16; 256],
    pub dwMountCount: u32,
    pub dwAllocateCount: u32,
    pub Capacity: i64,
}
impl ::core::marker::Copy for NTMS_PARTITIONINFORMATIONW {}
impl ::core::clone::Clone for NTMS_PARTITIONINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_PARTITIONINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_PARTITIONINFORMATIONW")
            .field("PhysicalMedia", &self.PhysicalMedia)
            .field("LogicalMedia", &self.LogicalMedia)
            .field("State", &self.State)
            .field("Side", &self.Side)
            .field("dwOmidLabelIdLength", &self.dwOmidLabelIdLength)
            .field("OmidLabelId", &self.OmidLabelId)
            .field("szOmidLabelType", &self.szOmidLabelType)
            .field("szOmidLabelInfo", &self.szOmidLabelInfo)
            .field("dwMountCount", &self.dwMountCount)
            .field("dwAllocateCount", &self.dwAllocateCount)
            .field("Capacity", &self.Capacity)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_PARTITIONINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.PhysicalMedia == other.PhysicalMedia
            && self.LogicalMedia == other.LogicalMedia
            && self.State == other.State
            && self.Side == other.Side
            && self.dwOmidLabelIdLength == other.dwOmidLabelIdLength
            && self.OmidLabelId == other.OmidLabelId
            && self.szOmidLabelType == other.szOmidLabelType
            && self.szOmidLabelInfo == other.szOmidLabelInfo
            && self.dwMountCount == other.dwMountCount
            && self.dwAllocateCount == other.dwAllocateCount
            && self.Capacity == other.Capacity
    }
}
impl ::core::cmp::Eq for NTMS_PARTITIONINFORMATIONW {}
impl FromIntoMemory for NTMS_PARTITIONINFORMATIONW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 640);
        let f_PhysicalMedia = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_LogicalMedia = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[16..16 + 16]);
        let f_State = <NtmsPartitionState as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_Side = <u16 as FromIntoMemory>::from_bytes(&from[36..36 + 2]);
        let f_dwOmidLabelIdLength = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_OmidLabelId = <[u8; 255] as FromIntoMemory>::from_bytes(&from[44..44 + 255]);
        let f_szOmidLabelType = <[u16; 64] as FromIntoMemory>::from_bytes(&from[299..299 + 64]);
        let f_szOmidLabelInfo = <[u16; 256] as FromIntoMemory>::from_bytes(&from[363..363 + 256]);
        let f_dwMountCount = <u32 as FromIntoMemory>::from_bytes(&from[620..620 + 4]);
        let f_dwAllocateCount = <u32 as FromIntoMemory>::from_bytes(&from[624..624 + 4]);
        let f_Capacity = <i64 as FromIntoMemory>::from_bytes(&from[632..632 + 8]);
        Self {
            PhysicalMedia: f_PhysicalMedia,
            LogicalMedia: f_LogicalMedia,
            State: f_State,
            Side: f_Side,
            dwOmidLabelIdLength: f_dwOmidLabelIdLength,
            OmidLabelId: f_OmidLabelId,
            szOmidLabelType: f_szOmidLabelType,
            szOmidLabelInfo: f_szOmidLabelInfo,
            dwMountCount: f_dwMountCount,
            dwAllocateCount: f_dwAllocateCount,
            Capacity: f_Capacity,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 640);
        FromIntoMemory::into_bytes(self.PhysicalMedia, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.LogicalMedia, &mut into[16..16 + 16]);
        FromIntoMemory::into_bytes(self.State, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.Side, &mut into[36..36 + 2]);
        FromIntoMemory::into_bytes(self.dwOmidLabelIdLength, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.OmidLabelId, &mut into[44..44 + 255]);
        FromIntoMemory::into_bytes(self.szOmidLabelType, &mut into[299..299 + 64]);
        FromIntoMemory::into_bytes(self.szOmidLabelInfo, &mut into[363..363 + 256]);
        FromIntoMemory::into_bytes(self.dwMountCount, &mut into[620..620 + 4]);
        FromIntoMemory::into_bytes(self.dwAllocateCount, &mut into[624..624 + 4]);
        FromIntoMemory::into_bytes(self.Capacity, &mut into[632..632 + 8]);
    }
    fn size() -> usize {
        640
    }
}
pub struct NTMS_PMIDINFORMATIONA {
    pub CurrentLibrary: crate::core::GUID,
    pub MediaPool: crate::core::GUID,
    pub Location: crate::core::GUID,
    pub LocationType: u32,
    pub MediaType: crate::core::GUID,
    pub HomeSlot: crate::core::GUID,
    pub szBarCode: [super::super::Foundation::CHAR; 64],
    pub BarCodeState: NtmsBarCodeState,
    pub szSequenceNumber: [super::super::Foundation::CHAR; 32],
    pub MediaState: NtmsMediaState,
    pub dwNumberOfPartitions: u32,
    pub dwMediaTypeCode: u32,
    pub dwDensityCode: u32,
    pub MountedPartition: crate::core::GUID,
}
impl ::core::marker::Copy for NTMS_PMIDINFORMATIONA {}
impl ::core::clone::Clone for NTMS_PMIDINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_PMIDINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_PMIDINFORMATIONA")
            .field("CurrentLibrary", &self.CurrentLibrary)
            .field("MediaPool", &self.MediaPool)
            .field("Location", &self.Location)
            .field("LocationType", &self.LocationType)
            .field("MediaType", &self.MediaType)
            .field("HomeSlot", &self.HomeSlot)
            .field("szBarCode", &self.szBarCode)
            .field("BarCodeState", &self.BarCodeState)
            .field("szSequenceNumber", &self.szSequenceNumber)
            .field("MediaState", &self.MediaState)
            .field("dwNumberOfPartitions", &self.dwNumberOfPartitions)
            .field("dwMediaTypeCode", &self.dwMediaTypeCode)
            .field("dwDensityCode", &self.dwDensityCode)
            .field("MountedPartition", &self.MountedPartition)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_PMIDINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.CurrentLibrary == other.CurrentLibrary
            && self.MediaPool == other.MediaPool
            && self.Location == other.Location
            && self.LocationType == other.LocationType
            && self.MediaType == other.MediaType
            && self.HomeSlot == other.HomeSlot
            && self.szBarCode == other.szBarCode
            && self.BarCodeState == other.BarCodeState
            && self.szSequenceNumber == other.szSequenceNumber
            && self.MediaState == other.MediaState
            && self.dwNumberOfPartitions == other.dwNumberOfPartitions
            && self.dwMediaTypeCode == other.dwMediaTypeCode
            && self.dwDensityCode == other.dwDensityCode
            && self.MountedPartition == other.MountedPartition
    }
}
impl ::core::cmp::Eq for NTMS_PMIDINFORMATIONA {}
impl FromIntoMemory for NTMS_PMIDINFORMATIONA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 216);
        let f_CurrentLibrary = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_MediaPool = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[16..16 + 16]);
        let f_Location = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[32..32 + 16]);
        let f_LocationType = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_MediaType = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[52..52 + 16]);
        let f_HomeSlot = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[68..68 + 16]);
        let f_szBarCode = <[super::super::Foundation::CHAR; 64] as FromIntoMemory>::from_bytes(
            &from[84..84 + 64],
        );
        let f_BarCodeState = <NtmsBarCodeState as FromIntoMemory>::from_bytes(&from[148..148 + 4]);
        let f_szSequenceNumber =
            <[super::super::Foundation::CHAR; 32] as FromIntoMemory>::from_bytes(
                &from[152..152 + 32],
            );
        let f_MediaState = <NtmsMediaState as FromIntoMemory>::from_bytes(&from[184..184 + 4]);
        let f_dwNumberOfPartitions = <u32 as FromIntoMemory>::from_bytes(&from[188..188 + 4]);
        let f_dwMediaTypeCode = <u32 as FromIntoMemory>::from_bytes(&from[192..192 + 4]);
        let f_dwDensityCode = <u32 as FromIntoMemory>::from_bytes(&from[196..196 + 4]);
        let f_MountedPartition =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[200..200 + 16]);
        Self {
            CurrentLibrary: f_CurrentLibrary,
            MediaPool: f_MediaPool,
            Location: f_Location,
            LocationType: f_LocationType,
            MediaType: f_MediaType,
            HomeSlot: f_HomeSlot,
            szBarCode: f_szBarCode,
            BarCodeState: f_BarCodeState,
            szSequenceNumber: f_szSequenceNumber,
            MediaState: f_MediaState,
            dwNumberOfPartitions: f_dwNumberOfPartitions,
            dwMediaTypeCode: f_dwMediaTypeCode,
            dwDensityCode: f_dwDensityCode,
            MountedPartition: f_MountedPartition,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 216);
        FromIntoMemory::into_bytes(self.CurrentLibrary, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.MediaPool, &mut into[16..16 + 16]);
        FromIntoMemory::into_bytes(self.Location, &mut into[32..32 + 16]);
        FromIntoMemory::into_bytes(self.LocationType, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.MediaType, &mut into[52..52 + 16]);
        FromIntoMemory::into_bytes(self.HomeSlot, &mut into[68..68 + 16]);
        FromIntoMemory::into_bytes(self.szBarCode, &mut into[84..84 + 64]);
        FromIntoMemory::into_bytes(self.BarCodeState, &mut into[148..148 + 4]);
        FromIntoMemory::into_bytes(self.szSequenceNumber, &mut into[152..152 + 32]);
        FromIntoMemory::into_bytes(self.MediaState, &mut into[184..184 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfPartitions, &mut into[188..188 + 4]);
        FromIntoMemory::into_bytes(self.dwMediaTypeCode, &mut into[192..192 + 4]);
        FromIntoMemory::into_bytes(self.dwDensityCode, &mut into[196..196 + 4]);
        FromIntoMemory::into_bytes(self.MountedPartition, &mut into[200..200 + 16]);
    }
    fn size() -> usize {
        216
    }
}
pub struct NTMS_PMIDINFORMATIONW {
    pub CurrentLibrary: crate::core::GUID,
    pub MediaPool: crate::core::GUID,
    pub Location: crate::core::GUID,
    pub LocationType: u32,
    pub MediaType: crate::core::GUID,
    pub HomeSlot: crate::core::GUID,
    pub szBarCode: [u16; 64],
    pub BarCodeState: NtmsBarCodeState,
    pub szSequenceNumber: [u16; 32],
    pub MediaState: NtmsMediaState,
    pub dwNumberOfPartitions: u32,
    pub dwMediaTypeCode: u32,
    pub dwDensityCode: u32,
    pub MountedPartition: crate::core::GUID,
}
impl ::core::marker::Copy for NTMS_PMIDINFORMATIONW {}
impl ::core::clone::Clone for NTMS_PMIDINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_PMIDINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_PMIDINFORMATIONW")
            .field("CurrentLibrary", &self.CurrentLibrary)
            .field("MediaPool", &self.MediaPool)
            .field("Location", &self.Location)
            .field("LocationType", &self.LocationType)
            .field("MediaType", &self.MediaType)
            .field("HomeSlot", &self.HomeSlot)
            .field("szBarCode", &self.szBarCode)
            .field("BarCodeState", &self.BarCodeState)
            .field("szSequenceNumber", &self.szSequenceNumber)
            .field("MediaState", &self.MediaState)
            .field("dwNumberOfPartitions", &self.dwNumberOfPartitions)
            .field("dwMediaTypeCode", &self.dwMediaTypeCode)
            .field("dwDensityCode", &self.dwDensityCode)
            .field("MountedPartition", &self.MountedPartition)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_PMIDINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.CurrentLibrary == other.CurrentLibrary
            && self.MediaPool == other.MediaPool
            && self.Location == other.Location
            && self.LocationType == other.LocationType
            && self.MediaType == other.MediaType
            && self.HomeSlot == other.HomeSlot
            && self.szBarCode == other.szBarCode
            && self.BarCodeState == other.BarCodeState
            && self.szSequenceNumber == other.szSequenceNumber
            && self.MediaState == other.MediaState
            && self.dwNumberOfPartitions == other.dwNumberOfPartitions
            && self.dwMediaTypeCode == other.dwMediaTypeCode
            && self.dwDensityCode == other.dwDensityCode
            && self.MountedPartition == other.MountedPartition
    }
}
impl ::core::cmp::Eq for NTMS_PMIDINFORMATIONW {}
impl FromIntoMemory for NTMS_PMIDINFORMATIONW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 216);
        let f_CurrentLibrary = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_MediaPool = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[16..16 + 16]);
        let f_Location = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[32..32 + 16]);
        let f_LocationType = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_MediaType = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[52..52 + 16]);
        let f_HomeSlot = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[68..68 + 16]);
        let f_szBarCode = <[u16; 64] as FromIntoMemory>::from_bytes(&from[84..84 + 64]);
        let f_BarCodeState = <NtmsBarCodeState as FromIntoMemory>::from_bytes(&from[148..148 + 4]);
        let f_szSequenceNumber = <[u16; 32] as FromIntoMemory>::from_bytes(&from[152..152 + 32]);
        let f_MediaState = <NtmsMediaState as FromIntoMemory>::from_bytes(&from[184..184 + 4]);
        let f_dwNumberOfPartitions = <u32 as FromIntoMemory>::from_bytes(&from[188..188 + 4]);
        let f_dwMediaTypeCode = <u32 as FromIntoMemory>::from_bytes(&from[192..192 + 4]);
        let f_dwDensityCode = <u32 as FromIntoMemory>::from_bytes(&from[196..196 + 4]);
        let f_MountedPartition =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[200..200 + 16]);
        Self {
            CurrentLibrary: f_CurrentLibrary,
            MediaPool: f_MediaPool,
            Location: f_Location,
            LocationType: f_LocationType,
            MediaType: f_MediaType,
            HomeSlot: f_HomeSlot,
            szBarCode: f_szBarCode,
            BarCodeState: f_BarCodeState,
            szSequenceNumber: f_szSequenceNumber,
            MediaState: f_MediaState,
            dwNumberOfPartitions: f_dwNumberOfPartitions,
            dwMediaTypeCode: f_dwMediaTypeCode,
            dwDensityCode: f_dwDensityCode,
            MountedPartition: f_MountedPartition,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 216);
        FromIntoMemory::into_bytes(self.CurrentLibrary, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.MediaPool, &mut into[16..16 + 16]);
        FromIntoMemory::into_bytes(self.Location, &mut into[32..32 + 16]);
        FromIntoMemory::into_bytes(self.LocationType, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.MediaType, &mut into[52..52 + 16]);
        FromIntoMemory::into_bytes(self.HomeSlot, &mut into[68..68 + 16]);
        FromIntoMemory::into_bytes(self.szBarCode, &mut into[84..84 + 64]);
        FromIntoMemory::into_bytes(self.BarCodeState, &mut into[148..148 + 4]);
        FromIntoMemory::into_bytes(self.szSequenceNumber, &mut into[152..152 + 32]);
        FromIntoMemory::into_bytes(self.MediaState, &mut into[184..184 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfPartitions, &mut into[188..188 + 4]);
        FromIntoMemory::into_bytes(self.dwMediaTypeCode, &mut into[192..192 + 4]);
        FromIntoMemory::into_bytes(self.dwDensityCode, &mut into[196..196 + 4]);
        FromIntoMemory::into_bytes(self.MountedPartition, &mut into[200..200 + 16]);
    }
    fn size() -> usize {
        216
    }
}
pub const NTMS_POOLHIERARCHY_LENGTH: u32 = 512u32;
pub const NTMS_PRODUCTNAME_LENGTH: u32 = 128u32;
pub const NTMS_REVISION_LENGTH: u32 = 32u32;
pub const NTMS_SEQUENCE_LENGTH: u32 = 32u32;
pub const NTMS_SERIALNUMBER_LENGTH: u32 = 32u32;
pub struct NTMS_STORAGESLOTINFORMATION {
    pub Number: u32,
    pub State: u32,
    pub Library: crate::core::GUID,
}
impl ::core::marker::Copy for NTMS_STORAGESLOTINFORMATION {}
impl ::core::clone::Clone for NTMS_STORAGESLOTINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_STORAGESLOTINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_STORAGESLOTINFORMATION")
            .field("Number", &self.Number)
            .field("State", &self.State)
            .field("Library", &self.Library)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NTMS_STORAGESLOTINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number && self.State == other.State && self.Library == other.Library
    }
}
impl ::core::cmp::Eq for NTMS_STORAGESLOTINFORMATION {}
impl FromIntoMemory for NTMS_STORAGESLOTINFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_Number = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_State = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Library = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        Self {
            Number: f_Number,
            State: f_State,
            Library: f_Library,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.Number, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.State, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Library, &mut into[8..8 + 16]);
    }
    fn size() -> usize {
        24
    }
}
pub const NTMS_USERNAME_LENGTH: u32 = 64u32;
pub const NTMS_VENDORNAME_LENGTH: u32 = 128u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NT_CREATE_FILE_DISPOSITION(pub u32);
pub const FILE_SUPERSEDE: NT_CREATE_FILE_DISPOSITION = NT_CREATE_FILE_DISPOSITION(0u32);
pub const FILE_CREATE: NT_CREATE_FILE_DISPOSITION = NT_CREATE_FILE_DISPOSITION(2u32);
pub const FILE_OPEN: NT_CREATE_FILE_DISPOSITION = NT_CREATE_FILE_DISPOSITION(1u32);
pub const FILE_OPEN_IF: NT_CREATE_FILE_DISPOSITION = NT_CREATE_FILE_DISPOSITION(3u32);
pub const FILE_OVERWRITE: NT_CREATE_FILE_DISPOSITION = NT_CREATE_FILE_DISPOSITION(4u32);
pub const FILE_OVERWRITE_IF: NT_CREATE_FILE_DISPOSITION = NT_CREATE_FILE_DISPOSITION(5u32);
impl ::core::marker::Copy for NT_CREATE_FILE_DISPOSITION {}
impl ::core::clone::Clone for NT_CREATE_FILE_DISPOSITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NT_CREATE_FILE_DISPOSITION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NT_CREATE_FILE_DISPOSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NT_CREATE_FILE_DISPOSITION")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for NT_CREATE_FILE_DISPOSITION {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsAccessMask(pub i32);
pub const NTMS_USE_ACCESS: NtmsAccessMask = NtmsAccessMask(1i32);
pub const NTMS_MODIFY_ACCESS: NtmsAccessMask = NtmsAccessMask(2i32);
pub const NTMS_CONTROL_ACCESS: NtmsAccessMask = NtmsAccessMask(4i32);
impl ::core::marker::Copy for NtmsAccessMask {}
impl ::core::clone::Clone for NtmsAccessMask {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsAccessMask {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsAccessMask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsAccessMask").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsAccessMask {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsAllocateOptions(pub i32);
pub const NTMS_ALLOCATE_NEW: NtmsAllocateOptions = NtmsAllocateOptions(1i32);
pub const NTMS_ALLOCATE_NEXT: NtmsAllocateOptions = NtmsAllocateOptions(2i32);
pub const NTMS_ALLOCATE_ERROR_IF_UNAVAILABLE: NtmsAllocateOptions = NtmsAllocateOptions(4i32);
impl ::core::marker::Copy for NtmsAllocateOptions {}
impl ::core::clone::Clone for NtmsAllocateOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsAllocateOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsAllocateOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsAllocateOptions").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsAllocateOptions {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsAllocationPolicy(pub i32);
pub const NTMS_ALLOCATE_FROMSCRATCH: NtmsAllocationPolicy = NtmsAllocationPolicy(1i32);
impl ::core::marker::Copy for NtmsAllocationPolicy {}
impl ::core::clone::Clone for NtmsAllocationPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsAllocationPolicy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsAllocationPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsAllocationPolicy")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for NtmsAllocationPolicy {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsAsyncOperations(pub i32);
pub const NTMS_ASYNCOP_MOUNT: NtmsAsyncOperations = NtmsAsyncOperations(1i32);
impl ::core::marker::Copy for NtmsAsyncOperations {}
impl ::core::clone::Clone for NtmsAsyncOperations {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsAsyncOperations {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsAsyncOperations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsAsyncOperations").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsAsyncOperations {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsAsyncStatus(pub i32);
pub const NTMS_ASYNCSTATE_QUEUED: NtmsAsyncStatus = NtmsAsyncStatus(0i32);
pub const NTMS_ASYNCSTATE_WAIT_RESOURCE: NtmsAsyncStatus = NtmsAsyncStatus(1i32);
pub const NTMS_ASYNCSTATE_WAIT_OPERATOR: NtmsAsyncStatus = NtmsAsyncStatus(2i32);
pub const NTMS_ASYNCSTATE_INPROCESS: NtmsAsyncStatus = NtmsAsyncStatus(3i32);
pub const NTMS_ASYNCSTATE_COMPLETE: NtmsAsyncStatus = NtmsAsyncStatus(4i32);
impl ::core::marker::Copy for NtmsAsyncStatus {}
impl ::core::clone::Clone for NtmsAsyncStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsAsyncStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsAsyncStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsAsyncStatus").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsAsyncStatus {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsBarCodeState(pub i32);
pub const NTMS_BARCODESTATE_OK: NtmsBarCodeState = NtmsBarCodeState(1i32);
pub const NTMS_BARCODESTATE_UNREADABLE: NtmsBarCodeState = NtmsBarCodeState(2i32);
impl ::core::marker::Copy for NtmsBarCodeState {}
impl ::core::clone::Clone for NtmsBarCodeState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsBarCodeState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsBarCodeState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsBarCodeState").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsBarCodeState {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsCreateNtmsMediaOptions(pub i32);
pub const NTMS_ERROR_ON_DUPLICATE: NtmsCreateNtmsMediaOptions = NtmsCreateNtmsMediaOptions(1i32);
impl ::core::marker::Copy for NtmsCreateNtmsMediaOptions {}
impl ::core::clone::Clone for NtmsCreateNtmsMediaOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsCreateNtmsMediaOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsCreateNtmsMediaOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsCreateNtmsMediaOptions")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for NtmsCreateNtmsMediaOptions {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsCreateOptions(pub i32);
pub const NTMS_OPEN_EXISTING: NtmsCreateOptions = NtmsCreateOptions(1i32);
pub const NTMS_CREATE_NEW: NtmsCreateOptions = NtmsCreateOptions(2i32);
pub const NTMS_OPEN_ALWAYS: NtmsCreateOptions = NtmsCreateOptions(3i32);
impl ::core::marker::Copy for NtmsCreateOptions {}
impl ::core::clone::Clone for NtmsCreateOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsCreateOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsCreateOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsCreateOptions").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsCreateOptions {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsDeallocationPolicy(pub i32);
pub const NTMS_DEALLOCATE_TOSCRATCH: NtmsDeallocationPolicy = NtmsDeallocationPolicy(1i32);
impl ::core::marker::Copy for NtmsDeallocationPolicy {}
impl ::core::clone::Clone for NtmsDeallocationPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsDeallocationPolicy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsDeallocationPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsDeallocationPolicy")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for NtmsDeallocationPolicy {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsDismountOptions(pub i32);
pub const NTMS_DISMOUNT_DEFERRED: NtmsDismountOptions = NtmsDismountOptions(1i32);
pub const NTMS_DISMOUNT_IMMEDIATE: NtmsDismountOptions = NtmsDismountOptions(2i32);
impl ::core::marker::Copy for NtmsDismountOptions {}
impl ::core::clone::Clone for NtmsDismountOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsDismountOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsDismountOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsDismountOptions").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsDismountOptions {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsDoorState(pub i32);
pub const NTMS_DOORSTATE_UNKNOWN: NtmsDoorState = NtmsDoorState(0i32);
pub const NTMS_DOORSTATE_CLOSED: NtmsDoorState = NtmsDoorState(1i32);
pub const NTMS_DOORSTATE_OPEN: NtmsDoorState = NtmsDoorState(2i32);
impl ::core::marker::Copy for NtmsDoorState {}
impl ::core::clone::Clone for NtmsDoorState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsDoorState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsDoorState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsDoorState").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsDoorState {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsDriveState(pub i32);
pub const NTMS_DRIVESTATE_DISMOUNTED: NtmsDriveState = NtmsDriveState(0i32);
pub const NTMS_DRIVESTATE_MOUNTED: NtmsDriveState = NtmsDriveState(1i32);
pub const NTMS_DRIVESTATE_LOADED: NtmsDriveState = NtmsDriveState(2i32);
pub const NTMS_DRIVESTATE_UNLOADED: NtmsDriveState = NtmsDriveState(5i32);
pub const NTMS_DRIVESTATE_BEING_CLEANED: NtmsDriveState = NtmsDriveState(6i32);
pub const NTMS_DRIVESTATE_DISMOUNTABLE: NtmsDriveState = NtmsDriveState(7i32);
impl ::core::marker::Copy for NtmsDriveState {}
impl ::core::clone::Clone for NtmsDriveState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsDriveState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsDriveState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsDriveState").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsDriveState {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsDriveType(pub i32);
pub const NTMS_UNKNOWN_DRIVE: NtmsDriveType = NtmsDriveType(0i32);
impl ::core::marker::Copy for NtmsDriveType {}
impl ::core::clone::Clone for NtmsDriveType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsDriveType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsDriveType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsDriveType").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsDriveType {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsEjectOperation(pub i32);
pub const NTMS_EJECT_START: NtmsEjectOperation = NtmsEjectOperation(0i32);
pub const NTMS_EJECT_STOP: NtmsEjectOperation = NtmsEjectOperation(1i32);
pub const NTMS_EJECT_QUEUE: NtmsEjectOperation = NtmsEjectOperation(2i32);
pub const NTMS_EJECT_FORCE: NtmsEjectOperation = NtmsEjectOperation(3i32);
pub const NTMS_EJECT_IMMEDIATE: NtmsEjectOperation = NtmsEjectOperation(4i32);
pub const NTMS_EJECT_ASK_USER: NtmsEjectOperation = NtmsEjectOperation(5i32);
impl ::core::marker::Copy for NtmsEjectOperation {}
impl ::core::clone::Clone for NtmsEjectOperation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsEjectOperation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsEjectOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsEjectOperation").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsEjectOperation {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsEnumerateOption(pub i32);
pub const NTMS_ENUM_DEFAULT: NtmsEnumerateOption = NtmsEnumerateOption(0i32);
pub const NTMS_ENUM_ROOTPOOL: NtmsEnumerateOption = NtmsEnumerateOption(1i32);
impl ::core::marker::Copy for NtmsEnumerateOption {}
impl ::core::clone::Clone for NtmsEnumerateOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsEnumerateOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsEnumerateOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsEnumerateOption").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsEnumerateOption {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsInjectOperation(pub i32);
pub const NTMS_INJECT_START: NtmsInjectOperation = NtmsInjectOperation(0i32);
pub const NTMS_INJECT_STOP: NtmsInjectOperation = NtmsInjectOperation(1i32);
pub const NTMS_INJECT_RETRACT: NtmsInjectOperation = NtmsInjectOperation(2i32);
pub const NTMS_INJECT_STARTMANY: NtmsInjectOperation = NtmsInjectOperation(3i32);
impl ::core::marker::Copy for NtmsInjectOperation {}
impl ::core::clone::Clone for NtmsInjectOperation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsInjectOperation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsInjectOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsInjectOperation").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsInjectOperation {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsInventoryMethod(pub i32);
pub const NTMS_INVENTORY_NONE: NtmsInventoryMethod = NtmsInventoryMethod(0i32);
pub const NTMS_INVENTORY_FAST: NtmsInventoryMethod = NtmsInventoryMethod(1i32);
pub const NTMS_INVENTORY_OMID: NtmsInventoryMethod = NtmsInventoryMethod(2i32);
pub const NTMS_INVENTORY_DEFAULT: NtmsInventoryMethod = NtmsInventoryMethod(3i32);
pub const NTMS_INVENTORY_SLOT: NtmsInventoryMethod = NtmsInventoryMethod(4i32);
pub const NTMS_INVENTORY_STOP: NtmsInventoryMethod = NtmsInventoryMethod(5i32);
pub const NTMS_INVENTORY_MAX: NtmsInventoryMethod = NtmsInventoryMethod(6i32);
impl ::core::marker::Copy for NtmsInventoryMethod {}
impl ::core::clone::Clone for NtmsInventoryMethod {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsInventoryMethod {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsInventoryMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsInventoryMethod").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsInventoryMethod {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsLibRequestFlags(pub i32);
pub const NTMS_LIBREQFLAGS_NOAUTOPURGE: NtmsLibRequestFlags = NtmsLibRequestFlags(1i32);
pub const NTMS_LIBREQFLAGS_NOFAILEDPURGE: NtmsLibRequestFlags = NtmsLibRequestFlags(2i32);
impl ::core::marker::Copy for NtmsLibRequestFlags {}
impl ::core::clone::Clone for NtmsLibRequestFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsLibRequestFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsLibRequestFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsLibRequestFlags").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsLibRequestFlags {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsLibraryFlags(pub i32);
pub const NTMS_LIBRARYFLAG_FIXEDOFFLINE: NtmsLibraryFlags = NtmsLibraryFlags(1i32);
pub const NTMS_LIBRARYFLAG_CLEANERPRESENT: NtmsLibraryFlags = NtmsLibraryFlags(2i32);
pub const NTMS_LIBRARYFLAG_AUTODETECTCHANGE: NtmsLibraryFlags = NtmsLibraryFlags(4i32);
pub const NTMS_LIBRARYFLAG_IGNORECLEANERUSESREMAINING: NtmsLibraryFlags = NtmsLibraryFlags(8i32);
pub const NTMS_LIBRARYFLAG_RECOGNIZECLEANERBARCODE: NtmsLibraryFlags = NtmsLibraryFlags(16i32);
impl ::core::marker::Copy for NtmsLibraryFlags {}
impl ::core::clone::Clone for NtmsLibraryFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsLibraryFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsLibraryFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsLibraryFlags").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsLibraryFlags {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsLibraryType(pub i32);
pub const NTMS_LIBRARYTYPE_UNKNOWN: NtmsLibraryType = NtmsLibraryType(0i32);
pub const NTMS_LIBRARYTYPE_OFFLINE: NtmsLibraryType = NtmsLibraryType(1i32);
pub const NTMS_LIBRARYTYPE_ONLINE: NtmsLibraryType = NtmsLibraryType(2i32);
pub const NTMS_LIBRARYTYPE_STANDALONE: NtmsLibraryType = NtmsLibraryType(3i32);
impl ::core::marker::Copy for NtmsLibraryType {}
impl ::core::clone::Clone for NtmsLibraryType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsLibraryType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsLibraryType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsLibraryType").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsLibraryType {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsLmOperation(pub i32);
pub const NTMS_LM_REMOVE: NtmsLmOperation = NtmsLmOperation(0i32);
pub const NTMS_LM_DISABLECHANGER: NtmsLmOperation = NtmsLmOperation(1i32);
pub const NTMS_LM_DISABLELIBRARY: NtmsLmOperation = NtmsLmOperation(1i32);
pub const NTMS_LM_ENABLECHANGER: NtmsLmOperation = NtmsLmOperation(2i32);
pub const NTMS_LM_ENABLELIBRARY: NtmsLmOperation = NtmsLmOperation(2i32);
pub const NTMS_LM_DISABLEDRIVE: NtmsLmOperation = NtmsLmOperation(3i32);
pub const NTMS_LM_ENABLEDRIVE: NtmsLmOperation = NtmsLmOperation(4i32);
pub const NTMS_LM_DISABLEMEDIA: NtmsLmOperation = NtmsLmOperation(5i32);
pub const NTMS_LM_ENABLEMEDIA: NtmsLmOperation = NtmsLmOperation(6i32);
pub const NTMS_LM_UPDATEOMID: NtmsLmOperation = NtmsLmOperation(7i32);
pub const NTMS_LM_INVENTORY: NtmsLmOperation = NtmsLmOperation(8i32);
pub const NTMS_LM_DOORACCESS: NtmsLmOperation = NtmsLmOperation(9i32);
pub const NTMS_LM_EJECT: NtmsLmOperation = NtmsLmOperation(10i32);
pub const NTMS_LM_EJECTCLEANER: NtmsLmOperation = NtmsLmOperation(11i32);
pub const NTMS_LM_INJECT: NtmsLmOperation = NtmsLmOperation(12i32);
pub const NTMS_LM_INJECTCLEANER: NtmsLmOperation = NtmsLmOperation(13i32);
pub const NTMS_LM_PROCESSOMID: NtmsLmOperation = NtmsLmOperation(14i32);
pub const NTMS_LM_CLEANDRIVE: NtmsLmOperation = NtmsLmOperation(15i32);
pub const NTMS_LM_DISMOUNT: NtmsLmOperation = NtmsLmOperation(16i32);
pub const NTMS_LM_MOUNT: NtmsLmOperation = NtmsLmOperation(17i32);
pub const NTMS_LM_WRITESCRATCH: NtmsLmOperation = NtmsLmOperation(18i32);
pub const NTMS_LM_CLASSIFY: NtmsLmOperation = NtmsLmOperation(19i32);
pub const NTMS_LM_RESERVECLEANER: NtmsLmOperation = NtmsLmOperation(20i32);
pub const NTMS_LM_RELEASECLEANER: NtmsLmOperation = NtmsLmOperation(21i32);
pub const NTMS_LM_MAXWORKITEM: NtmsLmOperation = NtmsLmOperation(22i32);
impl ::core::marker::Copy for NtmsLmOperation {}
impl ::core::clone::Clone for NtmsLmOperation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsLmOperation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsLmOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsLmOperation").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsLmOperation {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsLmState(pub i32);
pub const NTMS_LM_QUEUED: NtmsLmState = NtmsLmState(0i32);
pub const NTMS_LM_INPROCESS: NtmsLmState = NtmsLmState(1i32);
pub const NTMS_LM_PASSED: NtmsLmState = NtmsLmState(2i32);
pub const NTMS_LM_FAILED: NtmsLmState = NtmsLmState(3i32);
pub const NTMS_LM_INVALID: NtmsLmState = NtmsLmState(4i32);
pub const NTMS_LM_WAITING: NtmsLmState = NtmsLmState(5i32);
pub const NTMS_LM_DEFERRED: NtmsLmState = NtmsLmState(6i32);
pub const NTMS_LM_DEFFERED: NtmsLmState = NtmsLmState(6i32);
pub const NTMS_LM_CANCELLED: NtmsLmState = NtmsLmState(7i32);
pub const NTMS_LM_STOPPED: NtmsLmState = NtmsLmState(8i32);
impl ::core::marker::Copy for NtmsLmState {}
impl ::core::clone::Clone for NtmsLmState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsLmState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsLmState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsLmState").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsLmState {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsMediaPoolPolicy(pub i32);
pub const NTMS_POOLPOLICY_PURGEOFFLINESCRATCH: NtmsMediaPoolPolicy = NtmsMediaPoolPolicy(1i32);
pub const NTMS_POOLPOLICY_KEEPOFFLINEIMPORT: NtmsMediaPoolPolicy = NtmsMediaPoolPolicy(2i32);
impl ::core::marker::Copy for NtmsMediaPoolPolicy {}
impl ::core::clone::Clone for NtmsMediaPoolPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsMediaPoolPolicy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsMediaPoolPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsMediaPoolPolicy").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsMediaPoolPolicy {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsMediaState(pub i32);
pub const NTMS_MEDIASTATE_IDLE: NtmsMediaState = NtmsMediaState(0i32);
pub const NTMS_MEDIASTATE_INUSE: NtmsMediaState = NtmsMediaState(1i32);
pub const NTMS_MEDIASTATE_MOUNTED: NtmsMediaState = NtmsMediaState(2i32);
pub const NTMS_MEDIASTATE_LOADED: NtmsMediaState = NtmsMediaState(3i32);
pub const NTMS_MEDIASTATE_UNLOADED: NtmsMediaState = NtmsMediaState(4i32);
pub const NTMS_MEDIASTATE_OPERROR: NtmsMediaState = NtmsMediaState(5i32);
pub const NTMS_MEDIASTATE_OPREQ: NtmsMediaState = NtmsMediaState(6i32);
impl ::core::marker::Copy for NtmsMediaState {}
impl ::core::clone::Clone for NtmsMediaState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsMediaState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsMediaState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsMediaState").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsMediaState {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsMountOptions(pub i32);
pub const NTMS_MOUNT_READ: NtmsMountOptions = NtmsMountOptions(1i32);
pub const NTMS_MOUNT_WRITE: NtmsMountOptions = NtmsMountOptions(2i32);
pub const NTMS_MOUNT_ERROR_NOT_AVAILABLE: NtmsMountOptions = NtmsMountOptions(4i32);
pub const NTMS_MOUNT_ERROR_IF_UNAVAILABLE: NtmsMountOptions = NtmsMountOptions(4i32);
pub const NTMS_MOUNT_ERROR_OFFLINE: NtmsMountOptions = NtmsMountOptions(8i32);
pub const NTMS_MOUNT_ERROR_IF_OFFLINE: NtmsMountOptions = NtmsMountOptions(8i32);
pub const NTMS_MOUNT_SPECIFIC_DRIVE: NtmsMountOptions = NtmsMountOptions(16i32);
pub const NTMS_MOUNT_NOWAIT: NtmsMountOptions = NtmsMountOptions(32i32);
impl ::core::marker::Copy for NtmsMountOptions {}
impl ::core::clone::Clone for NtmsMountOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsMountOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsMountOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsMountOptions").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsMountOptions {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsMountPriority(pub i32);
pub const NTMS_PRIORITY_DEFAULT: NtmsMountPriority = NtmsMountPriority(0i32);
pub const NTMS_PRIORITY_HIGHEST: NtmsMountPriority = NtmsMountPriority(15i32);
pub const NTMS_PRIORITY_HIGH: NtmsMountPriority = NtmsMountPriority(7i32);
pub const NTMS_PRIORITY_NORMAL: NtmsMountPriority = NtmsMountPriority(0i32);
pub const NTMS_PRIORITY_LOW: NtmsMountPriority = NtmsMountPriority(-7i32);
pub const NTMS_PRIORITY_LOWEST: NtmsMountPriority = NtmsMountPriority(-15i32);
impl ::core::marker::Copy for NtmsMountPriority {}
impl ::core::clone::Clone for NtmsMountPriority {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsMountPriority {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsMountPriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsMountPriority").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsMountPriority {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsNotificationOperations(pub i32);
pub const NTMS_OBJ_UPDATE: NtmsNotificationOperations = NtmsNotificationOperations(1i32);
pub const NTMS_OBJ_INSERT: NtmsNotificationOperations = NtmsNotificationOperations(2i32);
pub const NTMS_OBJ_DELETE: NtmsNotificationOperations = NtmsNotificationOperations(3i32);
pub const NTMS_EVENT_SIGNAL: NtmsNotificationOperations = NtmsNotificationOperations(4i32);
pub const NTMS_EVENT_COMPLETE: NtmsNotificationOperations = NtmsNotificationOperations(5i32);
impl ::core::marker::Copy for NtmsNotificationOperations {}
impl ::core::clone::Clone for NtmsNotificationOperations {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsNotificationOperations {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsNotificationOperations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsNotificationOperations")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for NtmsNotificationOperations {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsObjectsTypes(pub i32);
pub const NTMS_UNKNOWN: NtmsObjectsTypes = NtmsObjectsTypes(0i32);
pub const NTMS_OBJECT: NtmsObjectsTypes = NtmsObjectsTypes(1i32);
pub const NTMS_CHANGER: NtmsObjectsTypes = NtmsObjectsTypes(2i32);
pub const NTMS_CHANGER_TYPE: NtmsObjectsTypes = NtmsObjectsTypes(3i32);
pub const NTMS_COMPUTER: NtmsObjectsTypes = NtmsObjectsTypes(4i32);
pub const NTMS_DRIVE: NtmsObjectsTypes = NtmsObjectsTypes(5i32);
pub const NTMS_DRIVE_TYPE: NtmsObjectsTypes = NtmsObjectsTypes(6i32);
pub const NTMS_IEDOOR: NtmsObjectsTypes = NtmsObjectsTypes(7i32);
pub const NTMS_IEPORT: NtmsObjectsTypes = NtmsObjectsTypes(8i32);
pub const NTMS_LIBRARY: NtmsObjectsTypes = NtmsObjectsTypes(9i32);
pub const NTMS_LIBREQUEST: NtmsObjectsTypes = NtmsObjectsTypes(10i32);
pub const NTMS_LOGICAL_MEDIA: NtmsObjectsTypes = NtmsObjectsTypes(11i32);
pub const NTMS_MEDIA_POOL: NtmsObjectsTypes = NtmsObjectsTypes(12i32);
pub const NTMS_MEDIA_TYPE: NtmsObjectsTypes = NtmsObjectsTypes(13i32);
pub const NTMS_PARTITION: NtmsObjectsTypes = NtmsObjectsTypes(14i32);
pub const NTMS_PHYSICAL_MEDIA: NtmsObjectsTypes = NtmsObjectsTypes(15i32);
pub const NTMS_STORAGESLOT: NtmsObjectsTypes = NtmsObjectsTypes(16i32);
pub const NTMS_OPREQUEST: NtmsObjectsTypes = NtmsObjectsTypes(17i32);
pub const NTMS_UI_DESTINATION: NtmsObjectsTypes = NtmsObjectsTypes(18i32);
pub const NTMS_NUMBER_OF_OBJECT_TYPES: NtmsObjectsTypes = NtmsObjectsTypes(19i32);
impl ::core::marker::Copy for NtmsObjectsTypes {}
impl ::core::clone::Clone for NtmsObjectsTypes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsObjectsTypes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsObjectsTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsObjectsTypes").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsObjectsTypes {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsOpRequestFlags(pub i32);
pub const NTMS_OPREQFLAGS_NOAUTOPURGE: NtmsOpRequestFlags = NtmsOpRequestFlags(1i32);
pub const NTMS_OPREQFLAGS_NOFAILEDPURGE: NtmsOpRequestFlags = NtmsOpRequestFlags(2i32);
pub const NTMS_OPREQFLAGS_NOALERTS: NtmsOpRequestFlags = NtmsOpRequestFlags(16i32);
pub const NTMS_OPREQFLAGS_NOTRAYICON: NtmsOpRequestFlags = NtmsOpRequestFlags(32i32);
impl ::core::marker::Copy for NtmsOpRequestFlags {}
impl ::core::clone::Clone for NtmsOpRequestFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsOpRequestFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsOpRequestFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsOpRequestFlags").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsOpRequestFlags {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsOperationalState(pub i32);
pub const NTMS_READY: NtmsOperationalState = NtmsOperationalState(0i32);
pub const NTMS_INITIALIZING: NtmsOperationalState = NtmsOperationalState(10i32);
pub const NTMS_NEEDS_SERVICE: NtmsOperationalState = NtmsOperationalState(20i32);
pub const NTMS_NOT_PRESENT: NtmsOperationalState = NtmsOperationalState(21i32);
impl ::core::marker::Copy for NtmsOperationalState {}
impl ::core::clone::Clone for NtmsOperationalState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsOperationalState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsOperationalState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsOperationalState")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for NtmsOperationalState {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsOpreqCommand(pub i32);
pub const NTMS_OPREQ_UNKNOWN: NtmsOpreqCommand = NtmsOpreqCommand(0i32);
pub const NTMS_OPREQ_NEWMEDIA: NtmsOpreqCommand = NtmsOpreqCommand(1i32);
pub const NTMS_OPREQ_CLEANER: NtmsOpreqCommand = NtmsOpreqCommand(2i32);
pub const NTMS_OPREQ_DEVICESERVICE: NtmsOpreqCommand = NtmsOpreqCommand(3i32);
pub const NTMS_OPREQ_MOVEMEDIA: NtmsOpreqCommand = NtmsOpreqCommand(4i32);
pub const NTMS_OPREQ_MESSAGE: NtmsOpreqCommand = NtmsOpreqCommand(5i32);
impl ::core::marker::Copy for NtmsOpreqCommand {}
impl ::core::clone::Clone for NtmsOpreqCommand {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsOpreqCommand {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsOpreqCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsOpreqCommand").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsOpreqCommand {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsOpreqState(pub i32);
pub const NTMS_OPSTATE_UNKNOWN: NtmsOpreqState = NtmsOpreqState(0i32);
pub const NTMS_OPSTATE_SUBMITTED: NtmsOpreqState = NtmsOpreqState(1i32);
pub const NTMS_OPSTATE_ACTIVE: NtmsOpreqState = NtmsOpreqState(2i32);
pub const NTMS_OPSTATE_INPROGRESS: NtmsOpreqState = NtmsOpreqState(3i32);
pub const NTMS_OPSTATE_REFUSED: NtmsOpreqState = NtmsOpreqState(4i32);
pub const NTMS_OPSTATE_COMPLETE: NtmsOpreqState = NtmsOpreqState(5i32);
impl ::core::marker::Copy for NtmsOpreqState {}
impl ::core::clone::Clone for NtmsOpreqState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsOpreqState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsOpreqState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsOpreqState").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsOpreqState {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsPartitionState(pub i32);
pub const NTMS_PARTSTATE_UNKNOWN: NtmsPartitionState = NtmsPartitionState(0i32);
pub const NTMS_PARTSTATE_UNPREPARED: NtmsPartitionState = NtmsPartitionState(1i32);
pub const NTMS_PARTSTATE_INCOMPATIBLE: NtmsPartitionState = NtmsPartitionState(2i32);
pub const NTMS_PARTSTATE_DECOMMISSIONED: NtmsPartitionState = NtmsPartitionState(3i32);
pub const NTMS_PARTSTATE_AVAILABLE: NtmsPartitionState = NtmsPartitionState(4i32);
pub const NTMS_PARTSTATE_ALLOCATED: NtmsPartitionState = NtmsPartitionState(5i32);
pub const NTMS_PARTSTATE_COMPLETE: NtmsPartitionState = NtmsPartitionState(6i32);
pub const NTMS_PARTSTATE_FOREIGN: NtmsPartitionState = NtmsPartitionState(7i32);
pub const NTMS_PARTSTATE_IMPORT: NtmsPartitionState = NtmsPartitionState(8i32);
pub const NTMS_PARTSTATE_RESERVED: NtmsPartitionState = NtmsPartitionState(9i32);
impl ::core::marker::Copy for NtmsPartitionState {}
impl ::core::clone::Clone for NtmsPartitionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsPartitionState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsPartitionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsPartitionState").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsPartitionState {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsPoolType(pub i32);
pub const NTMS_POOLTYPE_UNKNOWN: NtmsPoolType = NtmsPoolType(0i32);
pub const NTMS_POOLTYPE_SCRATCH: NtmsPoolType = NtmsPoolType(1i32);
pub const NTMS_POOLTYPE_FOREIGN: NtmsPoolType = NtmsPoolType(2i32);
pub const NTMS_POOLTYPE_IMPORT: NtmsPoolType = NtmsPoolType(3i32);
pub const NTMS_POOLTYPE_APPLICATION: NtmsPoolType = NtmsPoolType(1000i32);
impl ::core::marker::Copy for NtmsPoolType {}
impl ::core::clone::Clone for NtmsPoolType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsPoolType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsPoolType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsPoolType").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsPoolType {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsPortContent(pub i32);
pub const NTMS_PORTCONTENT_UNKNOWN: NtmsPortContent = NtmsPortContent(0i32);
pub const NTMS_PORTCONTENT_FULL: NtmsPortContent = NtmsPortContent(1i32);
pub const NTMS_PORTCONTENT_EMPTY: NtmsPortContent = NtmsPortContent(2i32);
impl ::core::marker::Copy for NtmsPortContent {}
impl ::core::clone::Clone for NtmsPortContent {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsPortContent {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsPortContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsPortContent").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsPortContent {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsPortPosition(pub i32);
pub const NTMS_PORTPOSITION_UNKNOWN: NtmsPortPosition = NtmsPortPosition(0i32);
pub const NTMS_PORTPOSITION_EXTENDED: NtmsPortPosition = NtmsPortPosition(1i32);
pub const NTMS_PORTPOSITION_RETRACTED: NtmsPortPosition = NtmsPortPosition(2i32);
impl ::core::marker::Copy for NtmsPortPosition {}
impl ::core::clone::Clone for NtmsPortPosition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsPortPosition {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsPortPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsPortPosition").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsPortPosition {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsReadWriteCharacteristics(pub i32);
pub const NTMS_MEDIARW_UNKNOWN: NtmsReadWriteCharacteristics = NtmsReadWriteCharacteristics(0i32);
pub const NTMS_MEDIARW_REWRITABLE: NtmsReadWriteCharacteristics =
    NtmsReadWriteCharacteristics(1i32);
pub const NTMS_MEDIARW_WRITEONCE: NtmsReadWriteCharacteristics = NtmsReadWriteCharacteristics(2i32);
pub const NTMS_MEDIARW_READONLY: NtmsReadWriteCharacteristics = NtmsReadWriteCharacteristics(3i32);
impl ::core::marker::Copy for NtmsReadWriteCharacteristics {}
impl ::core::clone::Clone for NtmsReadWriteCharacteristics {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsReadWriteCharacteristics {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsReadWriteCharacteristics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsReadWriteCharacteristics")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for NtmsReadWriteCharacteristics {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsSessionOptions(pub i32);
pub const NTMS_SESSION_QUERYEXPEDITE: NtmsSessionOptions = NtmsSessionOptions(1i32);
impl ::core::marker::Copy for NtmsSessionOptions {}
impl ::core::clone::Clone for NtmsSessionOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsSessionOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsSessionOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsSessionOptions").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsSessionOptions {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsSlotState(pub i32);
pub const NTMS_SLOTSTATE_UNKNOWN: NtmsSlotState = NtmsSlotState(0i32);
pub const NTMS_SLOTSTATE_FULL: NtmsSlotState = NtmsSlotState(1i32);
pub const NTMS_SLOTSTATE_EMPTY: NtmsSlotState = NtmsSlotState(2i32);
pub const NTMS_SLOTSTATE_NOTPRESENT: NtmsSlotState = NtmsSlotState(3i32);
pub const NTMS_SLOTSTATE_NEEDSINVENTORY: NtmsSlotState = NtmsSlotState(4i32);
impl ::core::marker::Copy for NtmsSlotState {}
impl ::core::clone::Clone for NtmsSlotState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsSlotState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsSlotState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsSlotState").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsSlotState {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsUIOperations(pub i32);
pub const NTMS_UIDEST_ADD: NtmsUIOperations = NtmsUIOperations(1i32);
pub const NTMS_UIDEST_DELETE: NtmsUIOperations = NtmsUIOperations(2i32);
pub const NTMS_UIDEST_DELETEALL: NtmsUIOperations = NtmsUIOperations(3i32);
pub const NTMS_UIOPERATION_MAX: NtmsUIOperations = NtmsUIOperations(4i32);
impl ::core::marker::Copy for NtmsUIOperations {}
impl ::core::clone::Clone for NtmsUIOperations {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsUIOperations {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsUIOperations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsUIOperations").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsUIOperations {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NtmsUITypes(pub i32);
pub const NTMS_UITYPE_INVALID: NtmsUITypes = NtmsUITypes(0i32);
pub const NTMS_UITYPE_INFO: NtmsUITypes = NtmsUITypes(1i32);
pub const NTMS_UITYPE_REQ: NtmsUITypes = NtmsUITypes(2i32);
pub const NTMS_UITYPE_ERR: NtmsUITypes = NtmsUITypes(3i32);
pub const NTMS_UITYPE_MAX: NtmsUITypes = NtmsUITypes(4i32);
impl ::core::marker::Copy for NtmsUITypes {}
impl ::core::clone::Clone for NtmsUITypes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsUITypes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsUITypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsUITypes").field(&self.0).finish()
    }
}
impl FromIntoMemory for NtmsUITypes {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct OFSTRUCT {
    pub cBytes: u8,
    pub fFixedDisk: u8,
    pub nErrCode: u16,
    pub Reserved1: u16,
    pub Reserved2: u16,
    pub szPathName: [super::super::Foundation::CHAR; 128],
}
impl ::core::marker::Copy for OFSTRUCT {}
impl ::core::clone::Clone for OFSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OFSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OFSTRUCT")
            .field("cBytes", &self.cBytes)
            .field("fFixedDisk", &self.fFixedDisk)
            .field("nErrCode", &self.nErrCode)
            .field("Reserved1", &self.Reserved1)
            .field("Reserved2", &self.Reserved2)
            .field("szPathName", &self.szPathName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for OFSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.cBytes == other.cBytes
            && self.fFixedDisk == other.fFixedDisk
            && self.nErrCode == other.nErrCode
            && self.Reserved1 == other.Reserved1
            && self.Reserved2 == other.Reserved2
            && self.szPathName == other.szPathName
    }
}
impl ::core::cmp::Eq for OFSTRUCT {}
impl FromIntoMemory for OFSTRUCT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 136);
        let f_cBytes = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_fFixedDisk = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_nErrCode = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_Reserved1 = <u16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_Reserved2 = <u16 as FromIntoMemory>::from_bytes(&from[6..6 + 2]);
        let f_szPathName = <[super::super::Foundation::CHAR; 128] as FromIntoMemory>::from_bytes(
            &from[8..8 + 128],
        );
        Self {
            cBytes: f_cBytes,
            fFixedDisk: f_fFixedDisk,
            nErrCode: f_nErrCode,
            Reserved1: f_Reserved1,
            Reserved2: f_Reserved2,
            szPathName: f_szPathName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 136);
        FromIntoMemory::into_bytes(self.cBytes, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.fFixedDisk, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.nErrCode, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.Reserved1, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.Reserved2, &mut into[6..6 + 2]);
        FromIntoMemory::into_bytes(self.szPathName, &mut into[8..8 + 128]);
    }
    fn size() -> usize {
        136
    }
}
pub const PARTITION_BASIC_DATA_GUID: crate::core::GUID =
    crate::core::GUID::from_u128(0xebd0a0a2_b9e5_4433_87c0_68b6b72699c7);
pub const PARTITION_BSP_GUID: crate::core::GUID =
    crate::core::GUID::from_u128(0x57434f53_4df9_45b9_8e9e_2370f006457c);
pub const PARTITION_CLUSTER_GUID: crate::core::GUID =
    crate::core::GUID::from_u128(0xdb97dba9_0840_4bae_97f0_ffb9a327c7e1);
pub const PARTITION_DPP_GUID: crate::core::GUID =
    crate::core::GUID::from_u128(0x57434f53_94cb_43f0_a533_d73c10cfa57d);
pub const PARTITION_ENTRY_UNUSED_GUID: crate::core::GUID =
    crate::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
pub const PARTITION_LDM_DATA_GUID: crate::core::GUID =
    crate::core::GUID::from_u128(0xaf9b60a0_1431_4f62_bc68_3311714a69ad);
pub const PARTITION_LDM_METADATA_GUID: crate::core::GUID =
    crate::core::GUID::from_u128(0x5808c8aa_7e8f_42e0_85d2_e1e90434cfb3);
pub const PARTITION_LEGACY_BL_GUID: crate::core::GUID =
    crate::core::GUID::from_u128(0x424ca0e2_7cb2_4fb9_8143_c52a99398bc6);
pub const PARTITION_LEGACY_BL_GUID_BACKUP: crate::core::GUID =
    crate::core::GUID::from_u128(0x424c3e6c_d79f_49cb_935d_36d71467a288);
pub const PARTITION_MAIN_OS_GUID: crate::core::GUID =
    crate::core::GUID::from_u128(0x57434f53_8f45_405e_8a23_186d8a4330d3);
pub const PARTITION_MSFT_RECOVERY_GUID: crate::core::GUID =
    crate::core::GUID::from_u128(0xde94bba4_06d1_4d40_a16a_bfd50179d6ac);
pub const PARTITION_MSFT_RESERVED_GUID: crate::core::GUID =
    crate::core::GUID::from_u128(0xe3c9e316_0b5c_4db8_817d_f92df00215ae);
pub const PARTITION_MSFT_SNAPSHOT_GUID: crate::core::GUID =
    crate::core::GUID::from_u128(0xcaddebf1_4400_4de8_b103_12117dcf3ccf);
pub const PARTITION_OS_DATA_GUID: crate::core::GUID =
    crate::core::GUID::from_u128(0x57434f53_23f2_44d5_a830_67bbdaa609f9);
pub const PARTITION_PATCH_GUID: crate::core::GUID =
    crate::core::GUID::from_u128(0x8967a686_96aa_6aa8_9589_a84256541090);
pub const PARTITION_PRE_INSTALLED_GUID: crate::core::GUID =
    crate::core::GUID::from_u128(0x57434f53_7fe0_4196_9b42_427b51643484);
pub const PARTITION_SERVICING_FILES_GUID: crate::core::GUID =
    crate::core::GUID::from_u128(0x57434f53_432e_4014_ae4c_8deaa9c0006a);
pub const PARTITION_SERVICING_METADATA_GUID: crate::core::GUID =
    crate::core::GUID::from_u128(0x57434f53_c691_4a05_bb4e_703dafd229ce);
pub const PARTITION_SERVICING_RESERVE_GUID: crate::core::GUID =
    crate::core::GUID::from_u128(0x57434f53_4b81_460b_a319_ffb6fe136d14);
pub const PARTITION_SERVICING_STAGING_ROOT_GUID: crate::core::GUID =
    crate::core::GUID::from_u128(0x57434f53_e84d_4e84_aaf3_ecbbbd04b9df);
pub const PARTITION_SPACES_DATA_GUID: crate::core::GUID =
    crate::core::GUID::from_u128(0xe7addcb4_dc34_4539_9a76_ebbd07be6f7e);
pub const PARTITION_SPACES_GUID: crate::core::GUID =
    crate::core::GUID::from_u128(0xe75caf8f_f680_4cee_afa3_b001e56efc2d);
pub const PARTITION_SYSTEM_GUID: crate::core::GUID =
    crate::core::GUID::from_u128(0xc12a7328_f81f_11d2_ba4b_00a0c93ec93b);
pub const PARTITION_WINDOWS_SYSTEM_GUID: crate::core::GUID =
    crate::core::GUID::from_u128(0x57434f53_e3e3_4631_a5c5_26d2243873aa);
pub type PCLFS_COMPLETION_ROUTINE = StdCallFnPtr<(MutPtr<::core::ffi::c_void>, u32), ()>;
pub type PCOPYFILE2_PROGRESS_ROUTINE = StdCallFnPtr<
    (ConstPtr<COPYFILE2_MESSAGE>, ConstPtr<::core::ffi::c_void>),
    COPYFILE2_MESSAGE_ACTION,
>;
pub type PFE_EXPORT_FUNC = StdCallFnPtr<(ConstPtr<u8>, ConstPtr<::core::ffi::c_void>, u32), u32>;
pub type PFE_IMPORT_FUNC =
    StdCallFnPtr<(MutPtr<u8>, ConstPtr<::core::ffi::c_void>, MutPtr<u32>), u32>;
pub type PFN_IO_COMPLETION =
    StdCallFnPtr<(MutPtr<FIO_CONTEXT>, MutPtr<FH_OVERLAPPED>, u32, u32), ()>;
pub type PLOG_FULL_HANDLER_CALLBACK = StdCallFnPtr<
    (
        super::super::Foundation::HANDLE,
        u32,
        super::super::Foundation::BOOL,
        MutPtr<::core::ffi::c_void>,
    ),
    (),
>;
pub type PLOG_TAIL_ADVANCE_CALLBACK = StdCallFnPtr<
    (
        super::super::Foundation::HANDLE,
        CLS_LSN,
        MutPtr<::core::ffi::c_void>,
    ),
    (),
>;
pub type PLOG_UNPINNED_CALLBACK = StdCallFnPtr<
    (
        super::super::Foundation::HANDLE,
        MutPtr<::core::ffi::c_void>,
    ),
    (),
>;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PREPARE_TAPE_OPERATION(pub i32);
pub const TAPE_FORMAT: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(5i32);
pub const TAPE_LOAD: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(0i32);
pub const TAPE_LOCK: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(3i32);
pub const TAPE_TENSION: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(2i32);
pub const TAPE_UNLOAD: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(1i32);
pub const TAPE_UNLOCK: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(4i32);
impl ::core::marker::Copy for PREPARE_TAPE_OPERATION {}
impl ::core::clone::Clone for PREPARE_TAPE_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PREPARE_TAPE_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PREPARE_TAPE_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PREPARE_TAPE_OPERATION")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for PREPARE_TAPE_OPERATION {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PRIORITY_HINT(pub i32);
pub const IoPriorityHintVeryLow: PRIORITY_HINT = PRIORITY_HINT(0i32);
pub const IoPriorityHintLow: PRIORITY_HINT = PRIORITY_HINT(1i32);
pub const IoPriorityHintNormal: PRIORITY_HINT = PRIORITY_HINT(2i32);
pub const MaximumIoPriorityHintType: PRIORITY_HINT = PRIORITY_HINT(3i32);
impl ::core::marker::Copy for PRIORITY_HINT {}
impl ::core::clone::Clone for PRIORITY_HINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PRIORITY_HINT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PRIORITY_HINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRIORITY_HINT").field(&self.0).finish()
    }
}
impl FromIntoMemory for PRIORITY_HINT {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct READ_DIRECTORY_NOTIFY_INFORMATION_CLASS(pub i32);
pub const ReadDirectoryNotifyInformation: READ_DIRECTORY_NOTIFY_INFORMATION_CLASS =
    READ_DIRECTORY_NOTIFY_INFORMATION_CLASS(1i32);
pub const ReadDirectoryNotifyExtendedInformation: READ_DIRECTORY_NOTIFY_INFORMATION_CLASS =
    READ_DIRECTORY_NOTIFY_INFORMATION_CLASS(2i32);
impl ::core::marker::Copy for READ_DIRECTORY_NOTIFY_INFORMATION_CLASS {}
impl ::core::clone::Clone for READ_DIRECTORY_NOTIFY_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for READ_DIRECTORY_NOTIFY_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for READ_DIRECTORY_NOTIFY_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("READ_DIRECTORY_NOTIFY_INFORMATION_CLASS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for READ_DIRECTORY_NOTIFY_INFORMATION_CLASS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct REPARSE_GUID_DATA_BUFFER {
    pub ReparseTag: u32,
    pub ReparseDataLength: u16,
    pub Reserved: u16,
    pub ReparseGuid: crate::core::GUID,
    pub GenericReparseBuffer: REPARSE_GUID_DATA_BUFFER_0,
}
impl ::core::marker::Copy for REPARSE_GUID_DATA_BUFFER {}
impl ::core::clone::Clone for REPARSE_GUID_DATA_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REPARSE_GUID_DATA_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPARSE_GUID_DATA_BUFFER")
            .field("ReparseTag", &self.ReparseTag)
            .field("ReparseDataLength", &self.ReparseDataLength)
            .field("Reserved", &self.Reserved)
            .field("ReparseGuid", &self.ReparseGuid)
            .field("GenericReparseBuffer", &self.GenericReparseBuffer)
            .finish()
    }
}
impl ::core::cmp::PartialEq for REPARSE_GUID_DATA_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.ReparseTag == other.ReparseTag
            && self.ReparseDataLength == other.ReparseDataLength
            && self.Reserved == other.Reserved
            && self.ReparseGuid == other.ReparseGuid
            && self.GenericReparseBuffer == other.GenericReparseBuffer
    }
}
impl ::core::cmp::Eq for REPARSE_GUID_DATA_BUFFER {}
impl FromIntoMemory for REPARSE_GUID_DATA_BUFFER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_ReparseTag = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ReparseDataLength = <u16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_Reserved = <u16 as FromIntoMemory>::from_bytes(&from[6..6 + 2]);
        let f_ReparseGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        let f_GenericReparseBuffer =
            <REPARSE_GUID_DATA_BUFFER_0 as FromIntoMemory>::from_bytes(&from[24..24 + 1]);
        Self {
            ReparseTag: f_ReparseTag,
            ReparseDataLength: f_ReparseDataLength,
            Reserved: f_Reserved,
            ReparseGuid: f_ReparseGuid,
            GenericReparseBuffer: f_GenericReparseBuffer,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.ReparseTag, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ReparseDataLength, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[6..6 + 2]);
        FromIntoMemory::into_bytes(self.ReparseGuid, &mut into[8..8 + 16]);
        FromIntoMemory::into_bytes(self.GenericReparseBuffer, &mut into[24..24 + 1]);
    }
    fn size() -> usize {
        28
    }
}
pub struct REPARSE_GUID_DATA_BUFFER_0 {
    pub DataBuffer: [u8; 1],
}
impl ::core::marker::Copy for REPARSE_GUID_DATA_BUFFER_0 {}
impl ::core::clone::Clone for REPARSE_GUID_DATA_BUFFER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REPARSE_GUID_DATA_BUFFER_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPARSE_GUID_DATA_BUFFER_0")
            .field("DataBuffer", &self.DataBuffer)
            .finish()
    }
}
impl ::core::cmp::PartialEq for REPARSE_GUID_DATA_BUFFER_0 {
    fn eq(&self, other: &Self) -> bool {
        self.DataBuffer == other.DataBuffer
    }
}
impl ::core::cmp::Eq for REPARSE_GUID_DATA_BUFFER_0 {}
impl FromIntoMemory for REPARSE_GUID_DATA_BUFFER_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 1);
        let f_DataBuffer = <[u8; 1] as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        Self {
            DataBuffer: f_DataBuffer,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 1);
        FromIntoMemory::into_bytes(self.DataBuffer, &mut into[0..0 + 1]);
    }
    fn size() -> usize {
        1
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct REPLACE_FILE_FLAGS(pub u32);
pub const REPLACEFILE_WRITE_THROUGH: REPLACE_FILE_FLAGS = REPLACE_FILE_FLAGS(1u32);
pub const REPLACEFILE_IGNORE_MERGE_ERRORS: REPLACE_FILE_FLAGS = REPLACE_FILE_FLAGS(2u32);
pub const REPLACEFILE_IGNORE_ACL_ERRORS: REPLACE_FILE_FLAGS = REPLACE_FILE_FLAGS(4u32);
impl ::core::marker::Copy for REPLACE_FILE_FLAGS {}
impl ::core::clone::Clone for REPLACE_FILE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REPLACE_FILE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for REPLACE_FILE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REPLACE_FILE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for REPLACE_FILE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for REPLACE_FILE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for REPLACE_FILE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for REPLACE_FILE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for REPLACE_FILE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for REPLACE_FILE_FLAGS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub const RESOURCE_MANAGER_COMMUNICATION: u32 = 2u32;
pub const RESOURCE_MANAGER_MAXIMUM_OPTION: u32 = 3u32;
pub const RESOURCE_MANAGER_OBJECT_PATH: &'static str = "\\ResourceManager\\";
pub const RESOURCE_MANAGER_VOLATILE: u32 = 1u32;
pub struct SERVER_ALIAS_INFO_0 {
    pub srvai0_alias: PWSTR,
    pub srvai0_target: PWSTR,
    pub srvai0_default: super::super::Foundation::BOOLEAN,
    pub srvai0_reserved: u32,
}
impl ::core::marker::Copy for SERVER_ALIAS_INFO_0 {}
impl ::core::clone::Clone for SERVER_ALIAS_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_ALIAS_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_ALIAS_INFO_0")
            .field("srvai0_alias", &self.srvai0_alias)
            .field("srvai0_target", &self.srvai0_target)
            .field("srvai0_default", &self.srvai0_default)
            .field("srvai0_reserved", &self.srvai0_reserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SERVER_ALIAS_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.srvai0_alias == other.srvai0_alias
            && self.srvai0_target == other.srvai0_target
            && self.srvai0_default == other.srvai0_default
            && self.srvai0_reserved == other.srvai0_reserved
    }
}
impl ::core::cmp::Eq for SERVER_ALIAS_INFO_0 {}
impl FromIntoMemory for SERVER_ALIAS_INFO_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_srvai0_alias = <PWSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_srvai0_target = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_srvai0_default =
            <super::super::Foundation::BOOLEAN as FromIntoMemory>::from_bytes(&from[8..8 + 1]);
        let f_srvai0_reserved = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            srvai0_alias: f_srvai0_alias,
            srvai0_target: f_srvai0_target,
            srvai0_default: f_srvai0_default,
            srvai0_reserved: f_srvai0_reserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.srvai0_alias, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.srvai0_target, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.srvai0_default, &mut into[8..8 + 1]);
        FromIntoMemory::into_bytes(self.srvai0_reserved, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct SERVER_CERTIFICATE_INFO_0 {
    pub srvci0_name: PWSTR,
    pub srvci0_subject: PWSTR,
    pub srvci0_issuer: PWSTR,
    pub srvci0_thumbprint: PWSTR,
    pub srvci0_friendlyname: PWSTR,
    pub srvci0_notbefore: PWSTR,
    pub srvci0_notafter: PWSTR,
    pub srvci0_storelocation: PWSTR,
    pub srvci0_storename: PWSTR,
    pub srvci0_renewalchain: PWSTR,
    pub srvci0_type: u32,
    pub srvci0_flags: u32,
}
impl ::core::marker::Copy for SERVER_CERTIFICATE_INFO_0 {}
impl ::core::clone::Clone for SERVER_CERTIFICATE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_CERTIFICATE_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_CERTIFICATE_INFO_0")
            .field("srvci0_name", &self.srvci0_name)
            .field("srvci0_subject", &self.srvci0_subject)
            .field("srvci0_issuer", &self.srvci0_issuer)
            .field("srvci0_thumbprint", &self.srvci0_thumbprint)
            .field("srvci0_friendlyname", &self.srvci0_friendlyname)
            .field("srvci0_notbefore", &self.srvci0_notbefore)
            .field("srvci0_notafter", &self.srvci0_notafter)
            .field("srvci0_storelocation", &self.srvci0_storelocation)
            .field("srvci0_storename", &self.srvci0_storename)
            .field("srvci0_renewalchain", &self.srvci0_renewalchain)
            .field("srvci0_type", &self.srvci0_type)
            .field("srvci0_flags", &self.srvci0_flags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SERVER_CERTIFICATE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.srvci0_name == other.srvci0_name
            && self.srvci0_subject == other.srvci0_subject
            && self.srvci0_issuer == other.srvci0_issuer
            && self.srvci0_thumbprint == other.srvci0_thumbprint
            && self.srvci0_friendlyname == other.srvci0_friendlyname
            && self.srvci0_notbefore == other.srvci0_notbefore
            && self.srvci0_notafter == other.srvci0_notafter
            && self.srvci0_storelocation == other.srvci0_storelocation
            && self.srvci0_storename == other.srvci0_storename
            && self.srvci0_renewalchain == other.srvci0_renewalchain
            && self.srvci0_type == other.srvci0_type
            && self.srvci0_flags == other.srvci0_flags
    }
}
impl ::core::cmp::Eq for SERVER_CERTIFICATE_INFO_0 {}
impl FromIntoMemory for SERVER_CERTIFICATE_INFO_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_srvci0_name = <PWSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_srvci0_subject = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_srvci0_issuer = <PWSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_srvci0_thumbprint = <PWSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_srvci0_friendlyname = <PWSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_srvci0_notbefore = <PWSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_srvci0_notafter = <PWSTR as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_srvci0_storelocation = <PWSTR as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_srvci0_storename = <PWSTR as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_srvci0_renewalchain = <PWSTR as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_srvci0_type = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_srvci0_flags = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        Self {
            srvci0_name: f_srvci0_name,
            srvci0_subject: f_srvci0_subject,
            srvci0_issuer: f_srvci0_issuer,
            srvci0_thumbprint: f_srvci0_thumbprint,
            srvci0_friendlyname: f_srvci0_friendlyname,
            srvci0_notbefore: f_srvci0_notbefore,
            srvci0_notafter: f_srvci0_notafter,
            srvci0_storelocation: f_srvci0_storelocation,
            srvci0_storename: f_srvci0_storename,
            srvci0_renewalchain: f_srvci0_renewalchain,
            srvci0_type: f_srvci0_type,
            srvci0_flags: f_srvci0_flags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.srvci0_name, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.srvci0_subject, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.srvci0_issuer, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.srvci0_thumbprint, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.srvci0_friendlyname, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.srvci0_notbefore, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.srvci0_notafter, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.srvci0_storelocation, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.srvci0_storename, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.srvci0_renewalchain, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.srvci0_type, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.srvci0_flags, &mut into[44..44 + 4]);
    }
    fn size() -> usize {
        48
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SERVER_CERTIFICATE_TYPE(pub i32);
pub const QUIC: SERVER_CERTIFICATE_TYPE = SERVER_CERTIFICATE_TYPE(0i32);
impl ::core::marker::Copy for SERVER_CERTIFICATE_TYPE {}
impl ::core::clone::Clone for SERVER_CERTIFICATE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SERVER_CERTIFICATE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SERVER_CERTIFICATE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVER_CERTIFICATE_TYPE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for SERVER_CERTIFICATE_TYPE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub const SESI1_NUM_ELEMENTS: u32 = 8u32;
pub const SESI2_NUM_ELEMENTS: u32 = 9u32;
pub struct SESSION_INFO_0 {
    pub sesi0_cname: PWSTR,
}
impl ::core::marker::Copy for SESSION_INFO_0 {}
impl ::core::clone::Clone for SESSION_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SESSION_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SESSION_INFO_0")
            .field("sesi0_cname", &self.sesi0_cname)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SESSION_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.sesi0_cname == other.sesi0_cname
    }
}
impl ::core::cmp::Eq for SESSION_INFO_0 {}
impl FromIntoMemory for SESSION_INFO_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_sesi0_cname = <PWSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            sesi0_cname: f_sesi0_cname,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.sesi0_cname, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct SESSION_INFO_1 {
    pub sesi1_cname: PWSTR,
    pub sesi1_username: PWSTR,
    pub sesi1_num_opens: u32,
    pub sesi1_time: u32,
    pub sesi1_idle_time: u32,
    pub sesi1_user_flags: SESSION_INFO_USER_FLAGS,
}
impl ::core::marker::Copy for SESSION_INFO_1 {}
impl ::core::clone::Clone for SESSION_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SESSION_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SESSION_INFO_1")
            .field("sesi1_cname", &self.sesi1_cname)
            .field("sesi1_username", &self.sesi1_username)
            .field("sesi1_num_opens", &self.sesi1_num_opens)
            .field("sesi1_time", &self.sesi1_time)
            .field("sesi1_idle_time", &self.sesi1_idle_time)
            .field("sesi1_user_flags", &self.sesi1_user_flags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SESSION_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.sesi1_cname == other.sesi1_cname
            && self.sesi1_username == other.sesi1_username
            && self.sesi1_num_opens == other.sesi1_num_opens
            && self.sesi1_time == other.sesi1_time
            && self.sesi1_idle_time == other.sesi1_idle_time
            && self.sesi1_user_flags == other.sesi1_user_flags
    }
}
impl ::core::cmp::Eq for SESSION_INFO_1 {}
impl FromIntoMemory for SESSION_INFO_1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_sesi1_cname = <PWSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_sesi1_username = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_sesi1_num_opens = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_sesi1_time = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_sesi1_idle_time = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_sesi1_user_flags =
            <SESSION_INFO_USER_FLAGS as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            sesi1_cname: f_sesi1_cname,
            sesi1_username: f_sesi1_username,
            sesi1_num_opens: f_sesi1_num_opens,
            sesi1_time: f_sesi1_time,
            sesi1_idle_time: f_sesi1_idle_time,
            sesi1_user_flags: f_sesi1_user_flags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.sesi1_cname, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.sesi1_username, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.sesi1_num_opens, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.sesi1_time, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.sesi1_idle_time, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.sesi1_user_flags, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct SESSION_INFO_10 {
    pub sesi10_cname: PWSTR,
    pub sesi10_username: PWSTR,
    pub sesi10_time: u32,
    pub sesi10_idle_time: u32,
}
impl ::core::marker::Copy for SESSION_INFO_10 {}
impl ::core::clone::Clone for SESSION_INFO_10 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SESSION_INFO_10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SESSION_INFO_10")
            .field("sesi10_cname", &self.sesi10_cname)
            .field("sesi10_username", &self.sesi10_username)
            .field("sesi10_time", &self.sesi10_time)
            .field("sesi10_idle_time", &self.sesi10_idle_time)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SESSION_INFO_10 {
    fn eq(&self, other: &Self) -> bool {
        self.sesi10_cname == other.sesi10_cname
            && self.sesi10_username == other.sesi10_username
            && self.sesi10_time == other.sesi10_time
            && self.sesi10_idle_time == other.sesi10_idle_time
    }
}
impl ::core::cmp::Eq for SESSION_INFO_10 {}
impl FromIntoMemory for SESSION_INFO_10 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_sesi10_cname = <PWSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_sesi10_username = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_sesi10_time = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_sesi10_idle_time = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            sesi10_cname: f_sesi10_cname,
            sesi10_username: f_sesi10_username,
            sesi10_time: f_sesi10_time,
            sesi10_idle_time: f_sesi10_idle_time,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.sesi10_cname, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.sesi10_username, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.sesi10_time, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.sesi10_idle_time, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct SESSION_INFO_2 {
    pub sesi2_cname: PWSTR,
    pub sesi2_username: PWSTR,
    pub sesi2_num_opens: u32,
    pub sesi2_time: u32,
    pub sesi2_idle_time: u32,
    pub sesi2_user_flags: SESSION_INFO_USER_FLAGS,
    pub sesi2_cltype_name: PWSTR,
}
impl ::core::marker::Copy for SESSION_INFO_2 {}
impl ::core::clone::Clone for SESSION_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SESSION_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SESSION_INFO_2")
            .field("sesi2_cname", &self.sesi2_cname)
            .field("sesi2_username", &self.sesi2_username)
            .field("sesi2_num_opens", &self.sesi2_num_opens)
            .field("sesi2_time", &self.sesi2_time)
            .field("sesi2_idle_time", &self.sesi2_idle_time)
            .field("sesi2_user_flags", &self.sesi2_user_flags)
            .field("sesi2_cltype_name", &self.sesi2_cltype_name)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SESSION_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.sesi2_cname == other.sesi2_cname
            && self.sesi2_username == other.sesi2_username
            && self.sesi2_num_opens == other.sesi2_num_opens
            && self.sesi2_time == other.sesi2_time
            && self.sesi2_idle_time == other.sesi2_idle_time
            && self.sesi2_user_flags == other.sesi2_user_flags
            && self.sesi2_cltype_name == other.sesi2_cltype_name
    }
}
impl ::core::cmp::Eq for SESSION_INFO_2 {}
impl FromIntoMemory for SESSION_INFO_2 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_sesi2_cname = <PWSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_sesi2_username = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_sesi2_num_opens = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_sesi2_time = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_sesi2_idle_time = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_sesi2_user_flags =
            <SESSION_INFO_USER_FLAGS as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_sesi2_cltype_name = <PWSTR as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        Self {
            sesi2_cname: f_sesi2_cname,
            sesi2_username: f_sesi2_username,
            sesi2_num_opens: f_sesi2_num_opens,
            sesi2_time: f_sesi2_time,
            sesi2_idle_time: f_sesi2_idle_time,
            sesi2_user_flags: f_sesi2_user_flags,
            sesi2_cltype_name: f_sesi2_cltype_name,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.sesi2_cname, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.sesi2_username, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.sesi2_num_opens, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.sesi2_time, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.sesi2_idle_time, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.sesi2_user_flags, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.sesi2_cltype_name, &mut into[24..24 + 4]);
    }
    fn size() -> usize {
        28
    }
}
pub struct SESSION_INFO_502 {
    pub sesi502_cname: PWSTR,
    pub sesi502_username: PWSTR,
    pub sesi502_num_opens: u32,
    pub sesi502_time: u32,
    pub sesi502_idle_time: u32,
    pub sesi502_user_flags: SESSION_INFO_USER_FLAGS,
    pub sesi502_cltype_name: PWSTR,
    pub sesi502_transport: PWSTR,
}
impl ::core::marker::Copy for SESSION_INFO_502 {}
impl ::core::clone::Clone for SESSION_INFO_502 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SESSION_INFO_502 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SESSION_INFO_502")
            .field("sesi502_cname", &self.sesi502_cname)
            .field("sesi502_username", &self.sesi502_username)
            .field("sesi502_num_opens", &self.sesi502_num_opens)
            .field("sesi502_time", &self.sesi502_time)
            .field("sesi502_idle_time", &self.sesi502_idle_time)
            .field("sesi502_user_flags", &self.sesi502_user_flags)
            .field("sesi502_cltype_name", &self.sesi502_cltype_name)
            .field("sesi502_transport", &self.sesi502_transport)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SESSION_INFO_502 {
    fn eq(&self, other: &Self) -> bool {
        self.sesi502_cname == other.sesi502_cname
            && self.sesi502_username == other.sesi502_username
            && self.sesi502_num_opens == other.sesi502_num_opens
            && self.sesi502_time == other.sesi502_time
            && self.sesi502_idle_time == other.sesi502_idle_time
            && self.sesi502_user_flags == other.sesi502_user_flags
            && self.sesi502_cltype_name == other.sesi502_cltype_name
            && self.sesi502_transport == other.sesi502_transport
    }
}
impl ::core::cmp::Eq for SESSION_INFO_502 {}
impl FromIntoMemory for SESSION_INFO_502 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_sesi502_cname = <PWSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_sesi502_username = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_sesi502_num_opens = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_sesi502_time = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_sesi502_idle_time = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_sesi502_user_flags =
            <SESSION_INFO_USER_FLAGS as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_sesi502_cltype_name = <PWSTR as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_sesi502_transport = <PWSTR as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            sesi502_cname: f_sesi502_cname,
            sesi502_username: f_sesi502_username,
            sesi502_num_opens: f_sesi502_num_opens,
            sesi502_time: f_sesi502_time,
            sesi502_idle_time: f_sesi502_idle_time,
            sesi502_user_flags: f_sesi502_user_flags,
            sesi502_cltype_name: f_sesi502_cltype_name,
            sesi502_transport: f_sesi502_transport,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.sesi502_cname, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.sesi502_username, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.sesi502_num_opens, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.sesi502_time, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.sesi502_idle_time, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.sesi502_user_flags, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.sesi502_cltype_name, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.sesi502_transport, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SESSION_INFO_USER_FLAGS(pub u32);
pub const SESS_GUEST: SESSION_INFO_USER_FLAGS = SESSION_INFO_USER_FLAGS(1u32);
pub const SESS_NOENCRYPTION: SESSION_INFO_USER_FLAGS = SESSION_INFO_USER_FLAGS(2u32);
impl ::core::marker::Copy for SESSION_INFO_USER_FLAGS {}
impl ::core::clone::Clone for SESSION_INFO_USER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SESSION_INFO_USER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SESSION_INFO_USER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SESSION_INFO_USER_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for SESSION_INFO_USER_FLAGS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SET_FILE_POINTER_MOVE_METHOD(pub u32);
pub const FILE_BEGIN: SET_FILE_POINTER_MOVE_METHOD = SET_FILE_POINTER_MOVE_METHOD(0u32);
pub const FILE_CURRENT: SET_FILE_POINTER_MOVE_METHOD = SET_FILE_POINTER_MOVE_METHOD(1u32);
pub const FILE_END: SET_FILE_POINTER_MOVE_METHOD = SET_FILE_POINTER_MOVE_METHOD(2u32);
impl ::core::marker::Copy for SET_FILE_POINTER_MOVE_METHOD {}
impl ::core::clone::Clone for SET_FILE_POINTER_MOVE_METHOD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SET_FILE_POINTER_MOVE_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SET_FILE_POINTER_MOVE_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SET_FILE_POINTER_MOVE_METHOD")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for SET_FILE_POINTER_MOVE_METHOD {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub const SHARE_CURRENT_USES_PARMNUM: u32 = 7u32;
pub const SHARE_FILE_SD_PARMNUM: u32 = 501u32;
pub struct SHARE_INFO_0 {
    pub shi0_netname: PWSTR,
}
impl ::core::marker::Copy for SHARE_INFO_0 {}
impl ::core::clone::Clone for SHARE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SHARE_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_0")
            .field("shi0_netname", &self.shi0_netname)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SHARE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.shi0_netname == other.shi0_netname
    }
}
impl ::core::cmp::Eq for SHARE_INFO_0 {}
impl FromIntoMemory for SHARE_INFO_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_shi0_netname = <PWSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            shi0_netname: f_shi0_netname,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.shi0_netname, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct SHARE_INFO_1 {
    pub shi1_netname: PWSTR,
    pub shi1_type: SHARE_TYPE,
    pub shi1_remark: PWSTR,
}
impl ::core::marker::Copy for SHARE_INFO_1 {}
impl ::core::clone::Clone for SHARE_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SHARE_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_1")
            .field("shi1_netname", &self.shi1_netname)
            .field("shi1_type", &self.shi1_type)
            .field("shi1_remark", &self.shi1_remark)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SHARE_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.shi1_netname == other.shi1_netname
            && self.shi1_type == other.shi1_type
            && self.shi1_remark == other.shi1_remark
    }
}
impl ::core::cmp::Eq for SHARE_INFO_1 {}
impl FromIntoMemory for SHARE_INFO_1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_shi1_netname = <PWSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_shi1_type = <SHARE_TYPE as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_shi1_remark = <PWSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            shi1_netname: f_shi1_netname,
            shi1_type: f_shi1_type,
            shi1_remark: f_shi1_remark,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.shi1_netname, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.shi1_type, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.shi1_remark, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct SHARE_INFO_1004 {
    pub shi1004_remark: PWSTR,
}
impl ::core::marker::Copy for SHARE_INFO_1004 {}
impl ::core::clone::Clone for SHARE_INFO_1004 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SHARE_INFO_1004 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_1004")
            .field("shi1004_remark", &self.shi1004_remark)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SHARE_INFO_1004 {
    fn eq(&self, other: &Self) -> bool {
        self.shi1004_remark == other.shi1004_remark
    }
}
impl ::core::cmp::Eq for SHARE_INFO_1004 {}
impl FromIntoMemory for SHARE_INFO_1004 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_shi1004_remark = <PWSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            shi1004_remark: f_shi1004_remark,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.shi1004_remark, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct SHARE_INFO_1005 {
    pub shi1005_flags: u32,
}
impl ::core::marker::Copy for SHARE_INFO_1005 {}
impl ::core::clone::Clone for SHARE_INFO_1005 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SHARE_INFO_1005 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_1005")
            .field("shi1005_flags", &self.shi1005_flags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SHARE_INFO_1005 {
    fn eq(&self, other: &Self) -> bool {
        self.shi1005_flags == other.shi1005_flags
    }
}
impl ::core::cmp::Eq for SHARE_INFO_1005 {}
impl FromIntoMemory for SHARE_INFO_1005 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_shi1005_flags = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            shi1005_flags: f_shi1005_flags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.shi1005_flags, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct SHARE_INFO_1006 {
    pub shi1006_max_uses: u32,
}
impl ::core::marker::Copy for SHARE_INFO_1006 {}
impl ::core::clone::Clone for SHARE_INFO_1006 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SHARE_INFO_1006 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_1006")
            .field("shi1006_max_uses", &self.shi1006_max_uses)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SHARE_INFO_1006 {
    fn eq(&self, other: &Self) -> bool {
        self.shi1006_max_uses == other.shi1006_max_uses
    }
}
impl ::core::cmp::Eq for SHARE_INFO_1006 {}
impl FromIntoMemory for SHARE_INFO_1006 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_shi1006_max_uses = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            shi1006_max_uses: f_shi1006_max_uses,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.shi1006_max_uses, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct SHARE_INFO_1501 {
    pub shi1501_reserved: u32,
    pub shi1501_security_descriptor: MutPtr<super::super::Security::SECURITY_DESCRIPTOR>,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for SHARE_INFO_1501 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for SHARE_INFO_1501 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for SHARE_INFO_1501 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_1501")
            .field("shi1501_reserved", &self.shi1501_reserved)
            .field(
                "shi1501_security_descriptor",
                &self.shi1501_security_descriptor,
            )
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for SHARE_INFO_1501 {
    fn eq(&self, other: &Self) -> bool {
        self.shi1501_reserved == other.shi1501_reserved
            && self.shi1501_security_descriptor == other.shi1501_security_descriptor
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for SHARE_INFO_1501 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for SHARE_INFO_1501 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_shi1501_reserved = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_shi1501_security_descriptor =
            <MutPtr<super::super::Security::SECURITY_DESCRIPTOR> as FromIntoMemory>::from_bytes(
                &from[4..4 + 4],
            );
        Self {
            shi1501_reserved: f_shi1501_reserved,
            shi1501_security_descriptor: f_shi1501_security_descriptor,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.shi1501_reserved, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.shi1501_security_descriptor, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct SHARE_INFO_1503 {
    pub shi1503_sharefilter: crate::core::GUID,
}
impl ::core::marker::Copy for SHARE_INFO_1503 {}
impl ::core::clone::Clone for SHARE_INFO_1503 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SHARE_INFO_1503 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_1503")
            .field("shi1503_sharefilter", &self.shi1503_sharefilter)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SHARE_INFO_1503 {
    fn eq(&self, other: &Self) -> bool {
        self.shi1503_sharefilter == other.shi1503_sharefilter
    }
}
impl ::core::cmp::Eq for SHARE_INFO_1503 {}
impl FromIntoMemory for SHARE_INFO_1503 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_shi1503_sharefilter =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        Self {
            shi1503_sharefilter: f_shi1503_sharefilter,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.shi1503_sharefilter, &mut into[0..0 + 16]);
    }
    fn size() -> usize {
        16
    }
}
pub struct SHARE_INFO_2 {
    pub shi2_netname: PWSTR,
    pub shi2_type: SHARE_TYPE,
    pub shi2_remark: PWSTR,
    pub shi2_permissions: SHARE_INFO_PERMISSIONS,
    pub shi2_max_uses: u32,
    pub shi2_current_uses: u32,
    pub shi2_path: PWSTR,
    pub shi2_passwd: PWSTR,
}
impl ::core::marker::Copy for SHARE_INFO_2 {}
impl ::core::clone::Clone for SHARE_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SHARE_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_2")
            .field("shi2_netname", &self.shi2_netname)
            .field("shi2_type", &self.shi2_type)
            .field("shi2_remark", &self.shi2_remark)
            .field("shi2_permissions", &self.shi2_permissions)
            .field("shi2_max_uses", &self.shi2_max_uses)
            .field("shi2_current_uses", &self.shi2_current_uses)
            .field("shi2_path", &self.shi2_path)
            .field("shi2_passwd", &self.shi2_passwd)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SHARE_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.shi2_netname == other.shi2_netname
            && self.shi2_type == other.shi2_type
            && self.shi2_remark == other.shi2_remark
            && self.shi2_permissions == other.shi2_permissions
            && self.shi2_max_uses == other.shi2_max_uses
            && self.shi2_current_uses == other.shi2_current_uses
            && self.shi2_path == other.shi2_path
            && self.shi2_passwd == other.shi2_passwd
    }
}
impl ::core::cmp::Eq for SHARE_INFO_2 {}
impl FromIntoMemory for SHARE_INFO_2 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_shi2_netname = <PWSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_shi2_type = <SHARE_TYPE as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_shi2_remark = <PWSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_shi2_permissions =
            <SHARE_INFO_PERMISSIONS as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_shi2_max_uses = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_shi2_current_uses = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_shi2_path = <PWSTR as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_shi2_passwd = <PWSTR as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            shi2_netname: f_shi2_netname,
            shi2_type: f_shi2_type,
            shi2_remark: f_shi2_remark,
            shi2_permissions: f_shi2_permissions,
            shi2_max_uses: f_shi2_max_uses,
            shi2_current_uses: f_shi2_current_uses,
            shi2_path: f_shi2_path,
            shi2_passwd: f_shi2_passwd,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.shi2_netname, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.shi2_type, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.shi2_remark, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.shi2_permissions, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.shi2_max_uses, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.shi2_current_uses, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.shi2_path, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.shi2_passwd, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
pub struct SHARE_INFO_501 {
    pub shi501_netname: PWSTR,
    pub shi501_type: SHARE_TYPE,
    pub shi501_remark: PWSTR,
    pub shi501_flags: u32,
}
impl ::core::marker::Copy for SHARE_INFO_501 {}
impl ::core::clone::Clone for SHARE_INFO_501 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SHARE_INFO_501 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_501")
            .field("shi501_netname", &self.shi501_netname)
            .field("shi501_type", &self.shi501_type)
            .field("shi501_remark", &self.shi501_remark)
            .field("shi501_flags", &self.shi501_flags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SHARE_INFO_501 {
    fn eq(&self, other: &Self) -> bool {
        self.shi501_netname == other.shi501_netname
            && self.shi501_type == other.shi501_type
            && self.shi501_remark == other.shi501_remark
            && self.shi501_flags == other.shi501_flags
    }
}
impl ::core::cmp::Eq for SHARE_INFO_501 {}
impl FromIntoMemory for SHARE_INFO_501 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_shi501_netname = <PWSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_shi501_type = <SHARE_TYPE as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_shi501_remark = <PWSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_shi501_flags = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            shi501_netname: f_shi501_netname,
            shi501_type: f_shi501_type,
            shi501_remark: f_shi501_remark,
            shi501_flags: f_shi501_flags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.shi501_netname, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.shi501_type, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.shi501_remark, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.shi501_flags, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct SHARE_INFO_502 {
    pub shi502_netname: PWSTR,
    pub shi502_type: SHARE_TYPE,
    pub shi502_remark: PWSTR,
    pub shi502_permissions: SHARE_INFO_PERMISSIONS,
    pub shi502_max_uses: u32,
    pub shi502_current_uses: u32,
    pub shi502_path: PWSTR,
    pub shi502_passwd: PWSTR,
    pub shi502_reserved: u32,
    pub shi502_security_descriptor: MutPtr<super::super::Security::SECURITY_DESCRIPTOR>,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for SHARE_INFO_502 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for SHARE_INFO_502 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for SHARE_INFO_502 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_502")
            .field("shi502_netname", &self.shi502_netname)
            .field("shi502_type", &self.shi502_type)
            .field("shi502_remark", &self.shi502_remark)
            .field("shi502_permissions", &self.shi502_permissions)
            .field("shi502_max_uses", &self.shi502_max_uses)
            .field("shi502_current_uses", &self.shi502_current_uses)
            .field("shi502_path", &self.shi502_path)
            .field("shi502_passwd", &self.shi502_passwd)
            .field("shi502_reserved", &self.shi502_reserved)
            .field(
                "shi502_security_descriptor",
                &self.shi502_security_descriptor,
            )
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for SHARE_INFO_502 {
    fn eq(&self, other: &Self) -> bool {
        self.shi502_netname == other.shi502_netname
            && self.shi502_type == other.shi502_type
            && self.shi502_remark == other.shi502_remark
            && self.shi502_permissions == other.shi502_permissions
            && self.shi502_max_uses == other.shi502_max_uses
            && self.shi502_current_uses == other.shi502_current_uses
            && self.shi502_path == other.shi502_path
            && self.shi502_passwd == other.shi502_passwd
            && self.shi502_reserved == other.shi502_reserved
            && self.shi502_security_descriptor == other.shi502_security_descriptor
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for SHARE_INFO_502 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for SHARE_INFO_502 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_shi502_netname = <PWSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_shi502_type = <SHARE_TYPE as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_shi502_remark = <PWSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_shi502_permissions =
            <SHARE_INFO_PERMISSIONS as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_shi502_max_uses = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_shi502_current_uses = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_shi502_path = <PWSTR as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_shi502_passwd = <PWSTR as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_shi502_reserved = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_shi502_security_descriptor =
            <MutPtr<super::super::Security::SECURITY_DESCRIPTOR> as FromIntoMemory>::from_bytes(
                &from[36..36 + 4],
            );
        Self {
            shi502_netname: f_shi502_netname,
            shi502_type: f_shi502_type,
            shi502_remark: f_shi502_remark,
            shi502_permissions: f_shi502_permissions,
            shi502_max_uses: f_shi502_max_uses,
            shi502_current_uses: f_shi502_current_uses,
            shi502_path: f_shi502_path,
            shi502_passwd: f_shi502_passwd,
            shi502_reserved: f_shi502_reserved,
            shi502_security_descriptor: f_shi502_security_descriptor,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.shi502_netname, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.shi502_type, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.shi502_remark, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.shi502_permissions, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.shi502_max_uses, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.shi502_current_uses, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.shi502_path, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.shi502_passwd, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.shi502_reserved, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.shi502_security_descriptor, &mut into[36..36 + 4]);
    }
    fn size() -> usize {
        40
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct SHARE_INFO_503 {
    pub shi503_netname: PWSTR,
    pub shi503_type: SHARE_TYPE,
    pub shi503_remark: PWSTR,
    pub shi503_permissions: SHARE_INFO_PERMISSIONS,
    pub shi503_max_uses: u32,
    pub shi503_current_uses: u32,
    pub shi503_path: PWSTR,
    pub shi503_passwd: PWSTR,
    pub shi503_servername: PWSTR,
    pub shi503_reserved: u32,
    pub shi503_security_descriptor: MutPtr<super::super::Security::SECURITY_DESCRIPTOR>,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for SHARE_INFO_503 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for SHARE_INFO_503 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for SHARE_INFO_503 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_503")
            .field("shi503_netname", &self.shi503_netname)
            .field("shi503_type", &self.shi503_type)
            .field("shi503_remark", &self.shi503_remark)
            .field("shi503_permissions", &self.shi503_permissions)
            .field("shi503_max_uses", &self.shi503_max_uses)
            .field("shi503_current_uses", &self.shi503_current_uses)
            .field("shi503_path", &self.shi503_path)
            .field("shi503_passwd", &self.shi503_passwd)
            .field("shi503_servername", &self.shi503_servername)
            .field("shi503_reserved", &self.shi503_reserved)
            .field(
                "shi503_security_descriptor",
                &self.shi503_security_descriptor,
            )
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for SHARE_INFO_503 {
    fn eq(&self, other: &Self) -> bool {
        self.shi503_netname == other.shi503_netname
            && self.shi503_type == other.shi503_type
            && self.shi503_remark == other.shi503_remark
            && self.shi503_permissions == other.shi503_permissions
            && self.shi503_max_uses == other.shi503_max_uses
            && self.shi503_current_uses == other.shi503_current_uses
            && self.shi503_path == other.shi503_path
            && self.shi503_passwd == other.shi503_passwd
            && self.shi503_servername == other.shi503_servername
            && self.shi503_reserved == other.shi503_reserved
            && self.shi503_security_descriptor == other.shi503_security_descriptor
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for SHARE_INFO_503 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for SHARE_INFO_503 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 44);
        let f_shi503_netname = <PWSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_shi503_type = <SHARE_TYPE as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_shi503_remark = <PWSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_shi503_permissions =
            <SHARE_INFO_PERMISSIONS as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_shi503_max_uses = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_shi503_current_uses = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_shi503_path = <PWSTR as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_shi503_passwd = <PWSTR as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_shi503_servername = <PWSTR as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_shi503_reserved = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_shi503_security_descriptor =
            <MutPtr<super::super::Security::SECURITY_DESCRIPTOR> as FromIntoMemory>::from_bytes(
                &from[40..40 + 4],
            );
        Self {
            shi503_netname: f_shi503_netname,
            shi503_type: f_shi503_type,
            shi503_remark: f_shi503_remark,
            shi503_permissions: f_shi503_permissions,
            shi503_max_uses: f_shi503_max_uses,
            shi503_current_uses: f_shi503_current_uses,
            shi503_path: f_shi503_path,
            shi503_passwd: f_shi503_passwd,
            shi503_servername: f_shi503_servername,
            shi503_reserved: f_shi503_reserved,
            shi503_security_descriptor: f_shi503_security_descriptor,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 44);
        FromIntoMemory::into_bytes(self.shi503_netname, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.shi503_type, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.shi503_remark, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.shi503_permissions, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.shi503_max_uses, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.shi503_current_uses, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.shi503_path, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.shi503_passwd, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.shi503_servername, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.shi503_reserved, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.shi503_security_descriptor, &mut into[40..40 + 4]);
    }
    fn size() -> usize {
        44
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SHARE_INFO_PERMISSIONS(pub u32);
pub const ACCESS_READ: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(1u32);
pub const ACCESS_WRITE: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(2u32);
pub const ACCESS_CREATE: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(4u32);
pub const ACCESS_EXEC: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(8u32);
pub const ACCESS_DELETE: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(16u32);
pub const ACCESS_ATRIB: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(32u32);
pub const ACCESS_PERM: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(64u32);
pub const ACCESS_ALL: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(32768u32);
impl ::core::marker::Copy for SHARE_INFO_PERMISSIONS {}
impl ::core::clone::Clone for SHARE_INFO_PERMISSIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SHARE_INFO_PERMISSIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHARE_INFO_PERMISSIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHARE_INFO_PERMISSIONS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for SHARE_INFO_PERMISSIONS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub const SHARE_MAX_USES_PARMNUM: u32 = 6u32;
pub const SHARE_NETNAME_PARMNUM: u32 = 1u32;
pub const SHARE_PASSWD_PARMNUM: u32 = 9u32;
pub const SHARE_PATH_PARMNUM: u32 = 8u32;
pub const SHARE_PERMISSIONS_PARMNUM: u32 = 5u32;
pub const SHARE_REMARK_PARMNUM: u32 = 4u32;
pub const SHARE_SERVER_PARMNUM: u32 = 503u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SHARE_TYPE(pub u32);
pub const STYPE_DISKTREE: SHARE_TYPE = SHARE_TYPE(0u32);
pub const STYPE_PRINTQ: SHARE_TYPE = SHARE_TYPE(1u32);
pub const STYPE_DEVICE: SHARE_TYPE = SHARE_TYPE(2u32);
pub const STYPE_IPC: SHARE_TYPE = SHARE_TYPE(3u32);
pub const STYPE_SPECIAL: SHARE_TYPE = SHARE_TYPE(2147483648u32);
pub const STYPE_TEMPORARY: SHARE_TYPE = SHARE_TYPE(1073741824u32);
pub const STYPE_MASK: SHARE_TYPE = SHARE_TYPE(255u32);
impl ::core::marker::Copy for SHARE_TYPE {}
impl ::core::clone::Clone for SHARE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SHARE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHARE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHARE_TYPE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SHARE_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SHARE_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SHARE_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SHARE_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SHARE_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for SHARE_TYPE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub const SHARE_TYPE_PARMNUM: u32 = 3u32;
pub const SHI1005_FLAGS_ACCESS_BASED_DIRECTORY_ENUM: u32 = 2048u32;
pub const SHI1005_FLAGS_ALLOW_NAMESPACE_CACHING: u32 = 1024u32;
pub const SHI1005_FLAGS_CLUSTER_MANAGED: u32 = 524288u32;
pub const SHI1005_FLAGS_COMPRESS_DATA: u32 = 1048576u32;
pub const SHI1005_FLAGS_DFS: u32 = 1u32;
pub const SHI1005_FLAGS_DFS_ROOT: u32 = 2u32;
pub const SHI1005_FLAGS_DISABLE_CLIENT_BUFFERING: u32 = 131072u32;
pub const SHI1005_FLAGS_ENABLE_CA: u32 = 16384u32;
pub const SHI1005_FLAGS_ENABLE_HASH: u32 = 8192u32;
pub const SHI1005_FLAGS_ENCRYPT_DATA: u32 = 32768u32;
pub const SHI1005_FLAGS_FORCE_LEVELII_OPLOCK: u32 = 4096u32;
pub const SHI1005_FLAGS_FORCE_SHARED_DELETE: u32 = 512u32;
pub const SHI1005_FLAGS_IDENTITY_REMOTING: u32 = 262144u32;
pub const SHI1005_FLAGS_RESERVED: u32 = 65536u32;
pub const SHI1005_FLAGS_RESTRICT_EXCLUSIVE_OPENS: u32 = 256u32;
pub const SHI1_NUM_ELEMENTS: u32 = 4u32;
pub const SHI2_NUM_ELEMENTS: u32 = 10u32;
pub const SHI_USES_UNLIMITED: u32 = 4294967295u32;
pub const STATSOPT_CLR: u32 = 1u32;
pub struct STAT_SERVER_0 {
    pub sts0_start: u32,
    pub sts0_fopens: u32,
    pub sts0_devopens: u32,
    pub sts0_jobsqueued: u32,
    pub sts0_sopens: u32,
    pub sts0_stimedout: u32,
    pub sts0_serrorout: u32,
    pub sts0_pwerrors: u32,
    pub sts0_permerrors: u32,
    pub sts0_syserrors: u32,
    pub sts0_bytessent_low: u32,
    pub sts0_bytessent_high: u32,
    pub sts0_bytesrcvd_low: u32,
    pub sts0_bytesrcvd_high: u32,
    pub sts0_avresponse: u32,
    pub sts0_reqbufneed: u32,
    pub sts0_bigbufneed: u32,
}
impl ::core::marker::Copy for STAT_SERVER_0 {}
impl ::core::clone::Clone for STAT_SERVER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STAT_SERVER_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STAT_SERVER_0")
            .field("sts0_start", &self.sts0_start)
            .field("sts0_fopens", &self.sts0_fopens)
            .field("sts0_devopens", &self.sts0_devopens)
            .field("sts0_jobsqueued", &self.sts0_jobsqueued)
            .field("sts0_sopens", &self.sts0_sopens)
            .field("sts0_stimedout", &self.sts0_stimedout)
            .field("sts0_serrorout", &self.sts0_serrorout)
            .field("sts0_pwerrors", &self.sts0_pwerrors)
            .field("sts0_permerrors", &self.sts0_permerrors)
            .field("sts0_syserrors", &self.sts0_syserrors)
            .field("sts0_bytessent_low", &self.sts0_bytessent_low)
            .field("sts0_bytessent_high", &self.sts0_bytessent_high)
            .field("sts0_bytesrcvd_low", &self.sts0_bytesrcvd_low)
            .field("sts0_bytesrcvd_high", &self.sts0_bytesrcvd_high)
            .field("sts0_avresponse", &self.sts0_avresponse)
            .field("sts0_reqbufneed", &self.sts0_reqbufneed)
            .field("sts0_bigbufneed", &self.sts0_bigbufneed)
            .finish()
    }
}
impl ::core::cmp::PartialEq for STAT_SERVER_0 {
    fn eq(&self, other: &Self) -> bool {
        self.sts0_start == other.sts0_start
            && self.sts0_fopens == other.sts0_fopens
            && self.sts0_devopens == other.sts0_devopens
            && self.sts0_jobsqueued == other.sts0_jobsqueued
            && self.sts0_sopens == other.sts0_sopens
            && self.sts0_stimedout == other.sts0_stimedout
            && self.sts0_serrorout == other.sts0_serrorout
            && self.sts0_pwerrors == other.sts0_pwerrors
            && self.sts0_permerrors == other.sts0_permerrors
            && self.sts0_syserrors == other.sts0_syserrors
            && self.sts0_bytessent_low == other.sts0_bytessent_low
            && self.sts0_bytessent_high == other.sts0_bytessent_high
            && self.sts0_bytesrcvd_low == other.sts0_bytesrcvd_low
            && self.sts0_bytesrcvd_high == other.sts0_bytesrcvd_high
            && self.sts0_avresponse == other.sts0_avresponse
            && self.sts0_reqbufneed == other.sts0_reqbufneed
            && self.sts0_bigbufneed == other.sts0_bigbufneed
    }
}
impl ::core::cmp::Eq for STAT_SERVER_0 {}
impl FromIntoMemory for STAT_SERVER_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 68);
        let f_sts0_start = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_sts0_fopens = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_sts0_devopens = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_sts0_jobsqueued = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_sts0_sopens = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_sts0_stimedout = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_sts0_serrorout = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_sts0_pwerrors = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_sts0_permerrors = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_sts0_syserrors = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_sts0_bytessent_low = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_sts0_bytessent_high = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_sts0_bytesrcvd_low = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_sts0_bytesrcvd_high = <u32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_sts0_avresponse = <u32 as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_sts0_reqbufneed = <u32 as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_sts0_bigbufneed = <u32 as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        Self {
            sts0_start: f_sts0_start,
            sts0_fopens: f_sts0_fopens,
            sts0_devopens: f_sts0_devopens,
            sts0_jobsqueued: f_sts0_jobsqueued,
            sts0_sopens: f_sts0_sopens,
            sts0_stimedout: f_sts0_stimedout,
            sts0_serrorout: f_sts0_serrorout,
            sts0_pwerrors: f_sts0_pwerrors,
            sts0_permerrors: f_sts0_permerrors,
            sts0_syserrors: f_sts0_syserrors,
            sts0_bytessent_low: f_sts0_bytessent_low,
            sts0_bytessent_high: f_sts0_bytessent_high,
            sts0_bytesrcvd_low: f_sts0_bytesrcvd_low,
            sts0_bytesrcvd_high: f_sts0_bytesrcvd_high,
            sts0_avresponse: f_sts0_avresponse,
            sts0_reqbufneed: f_sts0_reqbufneed,
            sts0_bigbufneed: f_sts0_bigbufneed,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 68);
        FromIntoMemory::into_bytes(self.sts0_start, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.sts0_fopens, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.sts0_devopens, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.sts0_jobsqueued, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.sts0_sopens, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.sts0_stimedout, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.sts0_serrorout, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.sts0_pwerrors, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.sts0_permerrors, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.sts0_syserrors, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.sts0_bytessent_low, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.sts0_bytessent_high, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.sts0_bytesrcvd_low, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.sts0_bytesrcvd_high, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.sts0_avresponse, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.sts0_reqbufneed, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.sts0_bigbufneed, &mut into[64..64 + 4]);
    }
    fn size() -> usize {
        68
    }
}
pub struct STAT_WORKSTATION_0 {
    pub StatisticsStartTime: i64,
    pub BytesReceived: i64,
    pub SmbsReceived: i64,
    pub PagingReadBytesRequested: i64,
    pub NonPagingReadBytesRequested: i64,
    pub CacheReadBytesRequested: i64,
    pub NetworkReadBytesRequested: i64,
    pub BytesTransmitted: i64,
    pub SmbsTransmitted: i64,
    pub PagingWriteBytesRequested: i64,
    pub NonPagingWriteBytesRequested: i64,
    pub CacheWriteBytesRequested: i64,
    pub NetworkWriteBytesRequested: i64,
    pub InitiallyFailedOperations: u32,
    pub FailedCompletionOperations: u32,
    pub ReadOperations: u32,
    pub RandomReadOperations: u32,
    pub ReadSmbs: u32,
    pub LargeReadSmbs: u32,
    pub SmallReadSmbs: u32,
    pub WriteOperations: u32,
    pub RandomWriteOperations: u32,
    pub WriteSmbs: u32,
    pub LargeWriteSmbs: u32,
    pub SmallWriteSmbs: u32,
    pub RawReadsDenied: u32,
    pub RawWritesDenied: u32,
    pub NetworkErrors: u32,
    pub Sessions: u32,
    pub FailedSessions: u32,
    pub Reconnects: u32,
    pub CoreConnects: u32,
    pub Lanman20Connects: u32,
    pub Lanman21Connects: u32,
    pub LanmanNtConnects: u32,
    pub ServerDisconnects: u32,
    pub HungSessions: u32,
    pub UseCount: u32,
    pub FailedUseCount: u32,
    pub CurrentCommands: u32,
}
impl ::core::marker::Copy for STAT_WORKSTATION_0 {}
impl ::core::clone::Clone for STAT_WORKSTATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STAT_WORKSTATION_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STAT_WORKSTATION_0")
            .field("StatisticsStartTime", &self.StatisticsStartTime)
            .field("BytesReceived", &self.BytesReceived)
            .field("SmbsReceived", &self.SmbsReceived)
            .field("PagingReadBytesRequested", &self.PagingReadBytesRequested)
            .field(
                "NonPagingReadBytesRequested",
                &self.NonPagingReadBytesRequested,
            )
            .field("CacheReadBytesRequested", &self.CacheReadBytesRequested)
            .field("NetworkReadBytesRequested", &self.NetworkReadBytesRequested)
            .field("BytesTransmitted", &self.BytesTransmitted)
            .field("SmbsTransmitted", &self.SmbsTransmitted)
            .field("PagingWriteBytesRequested", &self.PagingWriteBytesRequested)
            .field(
                "NonPagingWriteBytesRequested",
                &self.NonPagingWriteBytesRequested,
            )
            .field("CacheWriteBytesRequested", &self.CacheWriteBytesRequested)
            .field(
                "NetworkWriteBytesRequested",
                &self.NetworkWriteBytesRequested,
            )
            .field("InitiallyFailedOperations", &self.InitiallyFailedOperations)
            .field(
                "FailedCompletionOperations",
                &self.FailedCompletionOperations,
            )
            .field("ReadOperations", &self.ReadOperations)
            .field("RandomReadOperations", &self.RandomReadOperations)
            .field("ReadSmbs", &self.ReadSmbs)
            .field("LargeReadSmbs", &self.LargeReadSmbs)
            .field("SmallReadSmbs", &self.SmallReadSmbs)
            .field("WriteOperations", &self.WriteOperations)
            .field("RandomWriteOperations", &self.RandomWriteOperations)
            .field("WriteSmbs", &self.WriteSmbs)
            .field("LargeWriteSmbs", &self.LargeWriteSmbs)
            .field("SmallWriteSmbs", &self.SmallWriteSmbs)
            .field("RawReadsDenied", &self.RawReadsDenied)
            .field("RawWritesDenied", &self.RawWritesDenied)
            .field("NetworkErrors", &self.NetworkErrors)
            .field("Sessions", &self.Sessions)
            .field("FailedSessions", &self.FailedSessions)
            .field("Reconnects", &self.Reconnects)
            .field("CoreConnects", &self.CoreConnects)
            .field("Lanman20Connects", &self.Lanman20Connects)
            .field("Lanman21Connects", &self.Lanman21Connects)
            .field("LanmanNtConnects", &self.LanmanNtConnects)
            .field("ServerDisconnects", &self.ServerDisconnects)
            .field("HungSessions", &self.HungSessions)
            .field("UseCount", &self.UseCount)
            .field("FailedUseCount", &self.FailedUseCount)
            .field("CurrentCommands", &self.CurrentCommands)
            .finish()
    }
}
impl ::core::cmp::PartialEq for STAT_WORKSTATION_0 {
    fn eq(&self, other: &Self) -> bool {
        self.StatisticsStartTime == other.StatisticsStartTime
            && self.BytesReceived == other.BytesReceived
            && self.SmbsReceived == other.SmbsReceived
            && self.PagingReadBytesRequested == other.PagingReadBytesRequested
            && self.NonPagingReadBytesRequested == other.NonPagingReadBytesRequested
            && self.CacheReadBytesRequested == other.CacheReadBytesRequested
            && self.NetworkReadBytesRequested == other.NetworkReadBytesRequested
            && self.BytesTransmitted == other.BytesTransmitted
            && self.SmbsTransmitted == other.SmbsTransmitted
            && self.PagingWriteBytesRequested == other.PagingWriteBytesRequested
            && self.NonPagingWriteBytesRequested == other.NonPagingWriteBytesRequested
            && self.CacheWriteBytesRequested == other.CacheWriteBytesRequested
            && self.NetworkWriteBytesRequested == other.NetworkWriteBytesRequested
            && self.InitiallyFailedOperations == other.InitiallyFailedOperations
            && self.FailedCompletionOperations == other.FailedCompletionOperations
            && self.ReadOperations == other.ReadOperations
            && self.RandomReadOperations == other.RandomReadOperations
            && self.ReadSmbs == other.ReadSmbs
            && self.LargeReadSmbs == other.LargeReadSmbs
            && self.SmallReadSmbs == other.SmallReadSmbs
            && self.WriteOperations == other.WriteOperations
            && self.RandomWriteOperations == other.RandomWriteOperations
            && self.WriteSmbs == other.WriteSmbs
            && self.LargeWriteSmbs == other.LargeWriteSmbs
            && self.SmallWriteSmbs == other.SmallWriteSmbs
            && self.RawReadsDenied == other.RawReadsDenied
            && self.RawWritesDenied == other.RawWritesDenied
            && self.NetworkErrors == other.NetworkErrors
            && self.Sessions == other.Sessions
            && self.FailedSessions == other.FailedSessions
            && self.Reconnects == other.Reconnects
            && self.CoreConnects == other.CoreConnects
            && self.Lanman20Connects == other.Lanman20Connects
            && self.Lanman21Connects == other.Lanman21Connects
            && self.LanmanNtConnects == other.LanmanNtConnects
            && self.ServerDisconnects == other.ServerDisconnects
            && self.HungSessions == other.HungSessions
            && self.UseCount == other.UseCount
            && self.FailedUseCount == other.FailedUseCount
            && self.CurrentCommands == other.CurrentCommands
    }
}
impl ::core::cmp::Eq for STAT_WORKSTATION_0 {}
impl FromIntoMemory for STAT_WORKSTATION_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 216);
        let f_StatisticsStartTime = <i64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_BytesReceived = <i64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_SmbsReceived = <i64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_PagingReadBytesRequested = <i64 as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        let f_NonPagingReadBytesRequested = <i64 as FromIntoMemory>::from_bytes(&from[32..32 + 8]);
        let f_CacheReadBytesRequested = <i64 as FromIntoMemory>::from_bytes(&from[40..40 + 8]);
        let f_NetworkReadBytesRequested = <i64 as FromIntoMemory>::from_bytes(&from[48..48 + 8]);
        let f_BytesTransmitted = <i64 as FromIntoMemory>::from_bytes(&from[56..56 + 8]);
        let f_SmbsTransmitted = <i64 as FromIntoMemory>::from_bytes(&from[64..64 + 8]);
        let f_PagingWriteBytesRequested = <i64 as FromIntoMemory>::from_bytes(&from[72..72 + 8]);
        let f_NonPagingWriteBytesRequested = <i64 as FromIntoMemory>::from_bytes(&from[80..80 + 8]);
        let f_CacheWriteBytesRequested = <i64 as FromIntoMemory>::from_bytes(&from[88..88 + 8]);
        let f_NetworkWriteBytesRequested = <i64 as FromIntoMemory>::from_bytes(&from[96..96 + 8]);
        let f_InitiallyFailedOperations = <u32 as FromIntoMemory>::from_bytes(&from[104..104 + 4]);
        let f_FailedCompletionOperations = <u32 as FromIntoMemory>::from_bytes(&from[108..108 + 4]);
        let f_ReadOperations = <u32 as FromIntoMemory>::from_bytes(&from[112..112 + 4]);
        let f_RandomReadOperations = <u32 as FromIntoMemory>::from_bytes(&from[116..116 + 4]);
        let f_ReadSmbs = <u32 as FromIntoMemory>::from_bytes(&from[120..120 + 4]);
        let f_LargeReadSmbs = <u32 as FromIntoMemory>::from_bytes(&from[124..124 + 4]);
        let f_SmallReadSmbs = <u32 as FromIntoMemory>::from_bytes(&from[128..128 + 4]);
        let f_WriteOperations = <u32 as FromIntoMemory>::from_bytes(&from[132..132 + 4]);
        let f_RandomWriteOperations = <u32 as FromIntoMemory>::from_bytes(&from[136..136 + 4]);
        let f_WriteSmbs = <u32 as FromIntoMemory>::from_bytes(&from[140..140 + 4]);
        let f_LargeWriteSmbs = <u32 as FromIntoMemory>::from_bytes(&from[144..144 + 4]);
        let f_SmallWriteSmbs = <u32 as FromIntoMemory>::from_bytes(&from[148..148 + 4]);
        let f_RawReadsDenied = <u32 as FromIntoMemory>::from_bytes(&from[152..152 + 4]);
        let f_RawWritesDenied = <u32 as FromIntoMemory>::from_bytes(&from[156..156 + 4]);
        let f_NetworkErrors = <u32 as FromIntoMemory>::from_bytes(&from[160..160 + 4]);
        let f_Sessions = <u32 as FromIntoMemory>::from_bytes(&from[164..164 + 4]);
        let f_FailedSessions = <u32 as FromIntoMemory>::from_bytes(&from[168..168 + 4]);
        let f_Reconnects = <u32 as FromIntoMemory>::from_bytes(&from[172..172 + 4]);
        let f_CoreConnects = <u32 as FromIntoMemory>::from_bytes(&from[176..176 + 4]);
        let f_Lanman20Connects = <u32 as FromIntoMemory>::from_bytes(&from[180..180 + 4]);
        let f_Lanman21Connects = <u32 as FromIntoMemory>::from_bytes(&from[184..184 + 4]);
        let f_LanmanNtConnects = <u32 as FromIntoMemory>::from_bytes(&from[188..188 + 4]);
        let f_ServerDisconnects = <u32 as FromIntoMemory>::from_bytes(&from[192..192 + 4]);
        let f_HungSessions = <u32 as FromIntoMemory>::from_bytes(&from[196..196 + 4]);
        let f_UseCount = <u32 as FromIntoMemory>::from_bytes(&from[200..200 + 4]);
        let f_FailedUseCount = <u32 as FromIntoMemory>::from_bytes(&from[204..204 + 4]);
        let f_CurrentCommands = <u32 as FromIntoMemory>::from_bytes(&from[208..208 + 4]);
        Self {
            StatisticsStartTime: f_StatisticsStartTime,
            BytesReceived: f_BytesReceived,
            SmbsReceived: f_SmbsReceived,
            PagingReadBytesRequested: f_PagingReadBytesRequested,
            NonPagingReadBytesRequested: f_NonPagingReadBytesRequested,
            CacheReadBytesRequested: f_CacheReadBytesRequested,
            NetworkReadBytesRequested: f_NetworkReadBytesRequested,
            BytesTransmitted: f_BytesTransmitted,
            SmbsTransmitted: f_SmbsTransmitted,
            PagingWriteBytesRequested: f_PagingWriteBytesRequested,
            NonPagingWriteBytesRequested: f_NonPagingWriteBytesRequested,
            CacheWriteBytesRequested: f_CacheWriteBytesRequested,
            NetworkWriteBytesRequested: f_NetworkWriteBytesRequested,
            InitiallyFailedOperations: f_InitiallyFailedOperations,
            FailedCompletionOperations: f_FailedCompletionOperations,
            ReadOperations: f_ReadOperations,
            RandomReadOperations: f_RandomReadOperations,
            ReadSmbs: f_ReadSmbs,
            LargeReadSmbs: f_LargeReadSmbs,
            SmallReadSmbs: f_SmallReadSmbs,
            WriteOperations: f_WriteOperations,
            RandomWriteOperations: f_RandomWriteOperations,
            WriteSmbs: f_WriteSmbs,
            LargeWriteSmbs: f_LargeWriteSmbs,
            SmallWriteSmbs: f_SmallWriteSmbs,
            RawReadsDenied: f_RawReadsDenied,
            RawWritesDenied: f_RawWritesDenied,
            NetworkErrors: f_NetworkErrors,
            Sessions: f_Sessions,
            FailedSessions: f_FailedSessions,
            Reconnects: f_Reconnects,
            CoreConnects: f_CoreConnects,
            Lanman20Connects: f_Lanman20Connects,
            Lanman21Connects: f_Lanman21Connects,
            LanmanNtConnects: f_LanmanNtConnects,
            ServerDisconnects: f_ServerDisconnects,
            HungSessions: f_HungSessions,
            UseCount: f_UseCount,
            FailedUseCount: f_FailedUseCount,
            CurrentCommands: f_CurrentCommands,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 216);
        FromIntoMemory::into_bytes(self.StatisticsStartTime, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.BytesReceived, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.SmbsReceived, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.PagingReadBytesRequested, &mut into[24..24 + 8]);
        FromIntoMemory::into_bytes(self.NonPagingReadBytesRequested, &mut into[32..32 + 8]);
        FromIntoMemory::into_bytes(self.CacheReadBytesRequested, &mut into[40..40 + 8]);
        FromIntoMemory::into_bytes(self.NetworkReadBytesRequested, &mut into[48..48 + 8]);
        FromIntoMemory::into_bytes(self.BytesTransmitted, &mut into[56..56 + 8]);
        FromIntoMemory::into_bytes(self.SmbsTransmitted, &mut into[64..64 + 8]);
        FromIntoMemory::into_bytes(self.PagingWriteBytesRequested, &mut into[72..72 + 8]);
        FromIntoMemory::into_bytes(self.NonPagingWriteBytesRequested, &mut into[80..80 + 8]);
        FromIntoMemory::into_bytes(self.CacheWriteBytesRequested, &mut into[88..88 + 8]);
        FromIntoMemory::into_bytes(self.NetworkWriteBytesRequested, &mut into[96..96 + 8]);
        FromIntoMemory::into_bytes(self.InitiallyFailedOperations, &mut into[104..104 + 4]);
        FromIntoMemory::into_bytes(self.FailedCompletionOperations, &mut into[108..108 + 4]);
        FromIntoMemory::into_bytes(self.ReadOperations, &mut into[112..112 + 4]);
        FromIntoMemory::into_bytes(self.RandomReadOperations, &mut into[116..116 + 4]);
        FromIntoMemory::into_bytes(self.ReadSmbs, &mut into[120..120 + 4]);
        FromIntoMemory::into_bytes(self.LargeReadSmbs, &mut into[124..124 + 4]);
        FromIntoMemory::into_bytes(self.SmallReadSmbs, &mut into[128..128 + 4]);
        FromIntoMemory::into_bytes(self.WriteOperations, &mut into[132..132 + 4]);
        FromIntoMemory::into_bytes(self.RandomWriteOperations, &mut into[136..136 + 4]);
        FromIntoMemory::into_bytes(self.WriteSmbs, &mut into[140..140 + 4]);
        FromIntoMemory::into_bytes(self.LargeWriteSmbs, &mut into[144..144 + 4]);
        FromIntoMemory::into_bytes(self.SmallWriteSmbs, &mut into[148..148 + 4]);
        FromIntoMemory::into_bytes(self.RawReadsDenied, &mut into[152..152 + 4]);
        FromIntoMemory::into_bytes(self.RawWritesDenied, &mut into[156..156 + 4]);
        FromIntoMemory::into_bytes(self.NetworkErrors, &mut into[160..160 + 4]);
        FromIntoMemory::into_bytes(self.Sessions, &mut into[164..164 + 4]);
        FromIntoMemory::into_bytes(self.FailedSessions, &mut into[168..168 + 4]);
        FromIntoMemory::into_bytes(self.Reconnects, &mut into[172..172 + 4]);
        FromIntoMemory::into_bytes(self.CoreConnects, &mut into[176..176 + 4]);
        FromIntoMemory::into_bytes(self.Lanman20Connects, &mut into[180..180 + 4]);
        FromIntoMemory::into_bytes(self.Lanman21Connects, &mut into[184..184 + 4]);
        FromIntoMemory::into_bytes(self.LanmanNtConnects, &mut into[188..188 + 4]);
        FromIntoMemory::into_bytes(self.ServerDisconnects, &mut into[192..192 + 4]);
        FromIntoMemory::into_bytes(self.HungSessions, &mut into[196..196 + 4]);
        FromIntoMemory::into_bytes(self.UseCount, &mut into[200..200 + 4]);
        FromIntoMemory::into_bytes(self.FailedUseCount, &mut into[204..204 + 4]);
        FromIntoMemory::into_bytes(self.CurrentCommands, &mut into[208..208 + 4]);
    }
    fn size() -> usize {
        216
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_BUS_TYPE(pub i32);
pub const BusTypeUnknown: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(0i32);
pub const BusTypeScsi: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(1i32);
pub const BusTypeAtapi: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(2i32);
pub const BusTypeAta: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(3i32);
pub const BusType1394: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(4i32);
pub const BusTypeSsa: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(5i32);
pub const BusTypeFibre: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(6i32);
pub const BusTypeUsb: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(7i32);
pub const BusTypeRAID: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(8i32);
pub const BusTypeiScsi: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(9i32);
pub const BusTypeSas: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(10i32);
pub const BusTypeSata: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(11i32);
pub const BusTypeSd: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(12i32);
pub const BusTypeMmc: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(13i32);
pub const BusTypeVirtual: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(14i32);
pub const BusTypeFileBackedVirtual: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(15i32);
pub const BusTypeSpaces: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(16i32);
pub const BusTypeNvme: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(17i32);
pub const BusTypeSCM: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(18i32);
pub const BusTypeUfs: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(19i32);
pub const BusTypeMax: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(20i32);
pub const BusTypeMaxReserved: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(127i32);
impl ::core::marker::Copy for STORAGE_BUS_TYPE {}
impl ::core::clone::Clone for STORAGE_BUS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_BUS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_BUS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_BUS_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for STORAGE_BUS_TYPE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STREAM_INFO_LEVELS(pub i32);
pub const FindStreamInfoStandard: STREAM_INFO_LEVELS = STREAM_INFO_LEVELS(0i32);
pub const FindStreamInfoMaxInfoLevel: STREAM_INFO_LEVELS = STREAM_INFO_LEVELS(1i32);
impl ::core::marker::Copy for STREAM_INFO_LEVELS {}
impl ::core::clone::Clone for STREAM_INFO_LEVELS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STREAM_INFO_LEVELS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STREAM_INFO_LEVELS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STREAM_INFO_LEVELS").field(&self.0).finish()
    }
}
impl FromIntoMemory for STREAM_INFO_LEVELS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub const STYPE_RESERVED1: u32 = 16777216u32;
pub const STYPE_RESERVED2: u32 = 33554432u32;
pub const STYPE_RESERVED3: u32 = 67108864u32;
pub const STYPE_RESERVED4: u32 = 134217728u32;
pub const STYPE_RESERVED5: u32 = 1048576u32;
pub const STYPE_RESERVED_ALL: u32 = 1073741568u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SYMBOLIC_LINK_FLAGS(pub u32);
pub const SYMBOLIC_LINK_FLAG_DIRECTORY: SYMBOLIC_LINK_FLAGS = SYMBOLIC_LINK_FLAGS(1u32);
pub const SYMBOLIC_LINK_FLAG_ALLOW_UNPRIVILEGED_CREATE: SYMBOLIC_LINK_FLAGS =
    SYMBOLIC_LINK_FLAGS(2u32);
impl ::core::marker::Copy for SYMBOLIC_LINK_FLAGS {}
impl ::core::clone::Clone for SYMBOLIC_LINK_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYMBOLIC_LINK_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYMBOLIC_LINK_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYMBOLIC_LINK_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SYMBOLIC_LINK_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SYMBOLIC_LINK_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SYMBOLIC_LINK_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SYMBOLIC_LINK_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SYMBOLIC_LINK_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for SYMBOLIC_LINK_FLAGS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TAPEMARK_TYPE(pub i32);
pub const TAPE_FILEMARKS: TAPEMARK_TYPE = TAPEMARK_TYPE(1i32);
pub const TAPE_LONG_FILEMARKS: TAPEMARK_TYPE = TAPEMARK_TYPE(3i32);
pub const TAPE_SETMARKS: TAPEMARK_TYPE = TAPEMARK_TYPE(0i32);
pub const TAPE_SHORT_FILEMARKS: TAPEMARK_TYPE = TAPEMARK_TYPE(2i32);
impl ::core::marker::Copy for TAPEMARK_TYPE {}
impl ::core::clone::Clone for TAPEMARK_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TAPEMARK_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TAPEMARK_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TAPEMARK_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for TAPEMARK_TYPE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct TAPE_ERASE {
    pub Type: ERASE_TAPE_TYPE,
    pub Immediate: super::super::Foundation::BOOLEAN,
}
impl ::core::marker::Copy for TAPE_ERASE {}
impl ::core::clone::Clone for TAPE_ERASE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TAPE_ERASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TAPE_ERASE")
            .field("Type", &self.Type)
            .field("Immediate", &self.Immediate)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TAPE_ERASE {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Immediate == other.Immediate
    }
}
impl ::core::cmp::Eq for TAPE_ERASE {}
impl FromIntoMemory for TAPE_ERASE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Type = <ERASE_TAPE_TYPE as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Immediate =
            <super::super::Foundation::BOOLEAN as FromIntoMemory>::from_bytes(&from[4..4 + 1]);
        Self {
            Type: f_Type,
            Immediate: f_Immediate,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Type, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Immediate, &mut into[4..4 + 1]);
    }
    fn size() -> usize {
        8
    }
}
pub struct TAPE_GET_POSITION {
    pub Type: TAPE_POSITION_TYPE,
    pub Partition: u32,
    pub Offset: i64,
}
impl ::core::marker::Copy for TAPE_GET_POSITION {}
impl ::core::clone::Clone for TAPE_GET_POSITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TAPE_GET_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TAPE_GET_POSITION")
            .field("Type", &self.Type)
            .field("Partition", &self.Partition)
            .field("Offset", &self.Offset)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TAPE_GET_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Partition == other.Partition && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for TAPE_GET_POSITION {}
impl FromIntoMemory for TAPE_GET_POSITION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_Type = <TAPE_POSITION_TYPE as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Partition = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Offset = <i64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        Self {
            Type: f_Type,
            Partition: f_Partition,
            Offset: f_Offset,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.Type, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Partition, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Offset, &mut into[8..8 + 8]);
    }
    fn size() -> usize {
        16
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TAPE_INFORMATION_TYPE(pub u32);
pub const SET_TAPE_DRIVE_INFORMATION: TAPE_INFORMATION_TYPE = TAPE_INFORMATION_TYPE(1u32);
pub const SET_TAPE_MEDIA_INFORMATION: TAPE_INFORMATION_TYPE = TAPE_INFORMATION_TYPE(0u32);
impl ::core::marker::Copy for TAPE_INFORMATION_TYPE {}
impl ::core::clone::Clone for TAPE_INFORMATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TAPE_INFORMATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TAPE_INFORMATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TAPE_INFORMATION_TYPE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for TAPE_INFORMATION_TYPE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TAPE_POSITION_METHOD(pub i32);
pub const TAPE_ABSOLUTE_BLOCK: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(1i32);
pub const TAPE_LOGICAL_BLOCK: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(2i32);
pub const TAPE_REWIND: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(0i32);
pub const TAPE_SPACE_END_OF_DATA: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(4i32);
pub const TAPE_SPACE_FILEMARKS: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(6i32);
pub const TAPE_SPACE_RELATIVE_BLOCKS: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(5i32);
pub const TAPE_SPACE_SEQUENTIAL_FMKS: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(7i32);
pub const TAPE_SPACE_SEQUENTIAL_SMKS: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(9i32);
pub const TAPE_SPACE_SETMARKS: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(8i32);
impl ::core::marker::Copy for TAPE_POSITION_METHOD {}
impl ::core::clone::Clone for TAPE_POSITION_METHOD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TAPE_POSITION_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TAPE_POSITION_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TAPE_POSITION_METHOD")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for TAPE_POSITION_METHOD {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TAPE_POSITION_TYPE(pub i32);
pub const TAPE_ABSOLUTE_POSITION: TAPE_POSITION_TYPE = TAPE_POSITION_TYPE(0i32);
pub const TAPE_LOGICAL_POSITION: TAPE_POSITION_TYPE = TAPE_POSITION_TYPE(1i32);
impl ::core::marker::Copy for TAPE_POSITION_TYPE {}
impl ::core::clone::Clone for TAPE_POSITION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TAPE_POSITION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TAPE_POSITION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TAPE_POSITION_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for TAPE_POSITION_TYPE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct TAPE_PREPARE {
    pub Operation: PREPARE_TAPE_OPERATION,
    pub Immediate: super::super::Foundation::BOOLEAN,
}
impl ::core::marker::Copy for TAPE_PREPARE {}
impl ::core::clone::Clone for TAPE_PREPARE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TAPE_PREPARE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TAPE_PREPARE")
            .field("Operation", &self.Operation)
            .field("Immediate", &self.Immediate)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TAPE_PREPARE {
    fn eq(&self, other: &Self) -> bool {
        self.Operation == other.Operation && self.Immediate == other.Immediate
    }
}
impl ::core::cmp::Eq for TAPE_PREPARE {}
impl FromIntoMemory for TAPE_PREPARE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Operation = <PREPARE_TAPE_OPERATION as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Immediate =
            <super::super::Foundation::BOOLEAN as FromIntoMemory>::from_bytes(&from[4..4 + 1]);
        Self {
            Operation: f_Operation,
            Immediate: f_Immediate,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Operation, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Immediate, &mut into[4..4 + 1]);
    }
    fn size() -> usize {
        8
    }
}
pub struct TAPE_SET_POSITION {
    pub Method: TAPE_POSITION_METHOD,
    pub Partition: u32,
    pub Offset: i64,
    pub Immediate: super::super::Foundation::BOOLEAN,
}
impl ::core::marker::Copy for TAPE_SET_POSITION {}
impl ::core::clone::Clone for TAPE_SET_POSITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TAPE_SET_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TAPE_SET_POSITION")
            .field("Method", &self.Method)
            .field("Partition", &self.Partition)
            .field("Offset", &self.Offset)
            .field("Immediate", &self.Immediate)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TAPE_SET_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.Method == other.Method
            && self.Partition == other.Partition
            && self.Offset == other.Offset
            && self.Immediate == other.Immediate
    }
}
impl ::core::cmp::Eq for TAPE_SET_POSITION {}
impl FromIntoMemory for TAPE_SET_POSITION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_Method = <TAPE_POSITION_METHOD as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Partition = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Offset = <i64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_Immediate =
            <super::super::Foundation::BOOLEAN as FromIntoMemory>::from_bytes(&from[16..16 + 1]);
        Self {
            Method: f_Method,
            Partition: f_Partition,
            Offset: f_Offset,
            Immediate: f_Immediate,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.Method, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Partition, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Offset, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.Immediate, &mut into[16..16 + 1]);
    }
    fn size() -> usize {
        24
    }
}
pub struct TAPE_WRITE_MARKS {
    pub Type: TAPEMARK_TYPE,
    pub Count: u32,
    pub Immediate: super::super::Foundation::BOOLEAN,
}
impl ::core::marker::Copy for TAPE_WRITE_MARKS {}
impl ::core::clone::Clone for TAPE_WRITE_MARKS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TAPE_WRITE_MARKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TAPE_WRITE_MARKS")
            .field("Type", &self.Type)
            .field("Count", &self.Count)
            .field("Immediate", &self.Immediate)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TAPE_WRITE_MARKS {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Count == other.Count && self.Immediate == other.Immediate
    }
}
impl ::core::cmp::Eq for TAPE_WRITE_MARKS {}
impl FromIntoMemory for TAPE_WRITE_MARKS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Type = <TAPEMARK_TYPE as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Count = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Immediate =
            <super::super::Foundation::BOOLEAN as FromIntoMemory>::from_bytes(&from[8..8 + 1]);
        Self {
            Type: f_Type,
            Count: f_Count,
            Immediate: f_Immediate,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Type, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Count, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Immediate, &mut into[8..8 + 1]);
    }
    fn size() -> usize {
        12
    }
}
pub const TRANSACTIONMANAGER_OBJECT_PATH: &'static str = "\\TransactionManager\\";
pub const TRANSACTION_DO_NOT_PROMOTE: u32 = 1u32;
pub const TRANSACTION_MANAGER_COMMIT_DEFAULT: u32 = 0u32;
pub const TRANSACTION_MANAGER_COMMIT_LOWEST: u32 = 8u32;
pub const TRANSACTION_MANAGER_COMMIT_SYSTEM_HIVES: u32 = 4u32;
pub const TRANSACTION_MANAGER_COMMIT_SYSTEM_VOLUME: u32 = 2u32;
pub const TRANSACTION_MANAGER_CORRUPT_FOR_PROGRESS: u32 = 32u32;
pub const TRANSACTION_MANAGER_CORRUPT_FOR_RECOVERY: u32 = 16u32;
pub const TRANSACTION_MANAGER_MAXIMUM_OPTION: u32 = 63u32;
pub const TRANSACTION_MANAGER_VOLATILE: u32 = 1u32;
pub const TRANSACTION_MAXIMUM_OPTION: u32 = 1u32;
pub struct TRANSACTION_NOTIFICATION {
    pub TransactionKey: MutPtr<::core::ffi::c_void>,
    pub TransactionNotification: u32,
    pub TmVirtualClock: i64,
    pub ArgumentLength: u32,
}
impl ::core::marker::Copy for TRANSACTION_NOTIFICATION {}
impl ::core::clone::Clone for TRANSACTION_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRANSACTION_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTION_NOTIFICATION")
            .field("TransactionKey", &self.TransactionKey)
            .field("TransactionNotification", &self.TransactionNotification)
            .field("TmVirtualClock", &self.TmVirtualClock)
            .field("ArgumentLength", &self.ArgumentLength)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TRANSACTION_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.TransactionKey == other.TransactionKey
            && self.TransactionNotification == other.TransactionNotification
            && self.TmVirtualClock == other.TmVirtualClock
            && self.ArgumentLength == other.ArgumentLength
    }
}
impl ::core::cmp::Eq for TRANSACTION_NOTIFICATION {}
impl FromIntoMemory for TRANSACTION_NOTIFICATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_TransactionKey =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_TransactionNotification = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_TmVirtualClock = <i64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_ArgumentLength = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            TransactionKey: f_TransactionKey,
            TransactionNotification: f_TransactionNotification,
            TmVirtualClock: f_TmVirtualClock,
            ArgumentLength: f_ArgumentLength,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.TransactionKey, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.TransactionNotification, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.TmVirtualClock, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.ArgumentLength, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {
    pub MarshalCookie: u32,
    pub UOW: crate::core::GUID,
}
impl ::core::marker::Copy for TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {}
impl ::core::clone::Clone for TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT")
            .field("MarshalCookie", &self.MarshalCookie)
            .field("UOW", &self.UOW)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.MarshalCookie == other.MarshalCookie && self.UOW == other.UOW
    }
}
impl ::core::cmp::Eq for TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {}
impl FromIntoMemory for TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_MarshalCookie = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_UOW = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[4..4 + 16]);
        Self {
            MarshalCookie: f_MarshalCookie,
            UOW: f_UOW,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.MarshalCookie, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.UOW, &mut into[4..4 + 16]);
    }
    fn size() -> usize {
        20
    }
}
pub struct TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {
    pub PropagationCookie: u32,
    pub UOW: crate::core::GUID,
    pub TmIdentity: crate::core::GUID,
    pub BufferLength: u32,
}
impl ::core::marker::Copy for TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {}
impl ::core::clone::Clone for TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT")
            .field("PropagationCookie", &self.PropagationCookie)
            .field("UOW", &self.UOW)
            .field("TmIdentity", &self.TmIdentity)
            .field("BufferLength", &self.BufferLength)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.PropagationCookie == other.PropagationCookie
            && self.UOW == other.UOW
            && self.TmIdentity == other.TmIdentity
            && self.BufferLength == other.BufferLength
    }
}
impl ::core::cmp::Eq for TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {}
impl FromIntoMemory for TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_PropagationCookie = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_UOW = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[4..4 + 16]);
        let f_TmIdentity = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[20..20 + 16]);
        let f_BufferLength = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        Self {
            PropagationCookie: f_PropagationCookie,
            UOW: f_UOW,
            TmIdentity: f_TmIdentity,
            BufferLength: f_BufferLength,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.PropagationCookie, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.UOW, &mut into[4..4 + 16]);
        FromIntoMemory::into_bytes(self.TmIdentity, &mut into[20..20 + 16]);
        FromIntoMemory::into_bytes(self.BufferLength, &mut into[36..36 + 4]);
    }
    fn size() -> usize {
        40
    }
}
pub struct TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {
    pub EnlistmentId: crate::core::GUID,
    pub UOW: crate::core::GUID,
}
impl ::core::marker::Copy for TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {}
impl ::core::clone::Clone for TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT")
            .field("EnlistmentId", &self.EnlistmentId)
            .field("UOW", &self.UOW)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.EnlistmentId == other.EnlistmentId && self.UOW == other.UOW
    }
}
impl ::core::cmp::Eq for TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {}
impl FromIntoMemory for TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_EnlistmentId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_UOW = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[16..16 + 16]);
        Self {
            EnlistmentId: f_EnlistmentId,
            UOW: f_UOW,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.EnlistmentId, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.UOW, &mut into[16..16 + 16]);
    }
    fn size() -> usize {
        32
    }
}
pub struct TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {
    pub SavepointId: u32,
}
impl ::core::marker::Copy for TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {}
impl ::core::clone::Clone for TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT")
            .field("SavepointId", &self.SavepointId)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.SavepointId == other.SavepointId
    }
}
impl ::core::cmp::Eq for TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {}
impl FromIntoMemory for TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_SavepointId = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            SavepointId: f_SavepointId,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.SavepointId, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {
    pub TmIdentity: crate::core::GUID,
    pub Flags: u32,
}
impl ::core::marker::Copy for TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {}
impl ::core::clone::Clone for TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT")
            .field("TmIdentity", &self.TmIdentity)
            .field("Flags", &self.Flags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.TmIdentity == other.TmIdentity && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {}
impl FromIntoMemory for TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_TmIdentity = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_Flags = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            TmIdentity: f_TmIdentity,
            Flags: f_Flags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.TmIdentity, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub const TRANSACTION_NOTIFICATION_TM_ONLINE_FLAG_IS_CLUSTERED: u32 = 1u32;
pub const TRANSACTION_NOTIFY_COMMIT: u32 = 4u32;
pub const TRANSACTION_NOTIFY_COMMIT_COMPLETE: u32 = 64u32;
pub const TRANSACTION_NOTIFY_COMMIT_FINALIZE: u32 = 1073741824u32;
pub const TRANSACTION_NOTIFY_COMMIT_REQUEST: u32 = 67108864u32;
pub const TRANSACTION_NOTIFY_DELEGATE_COMMIT: u32 = 1024u32;
pub const TRANSACTION_NOTIFY_ENLIST_MASK: u32 = 262144u32;
pub const TRANSACTION_NOTIFY_ENLIST_PREPREPARE: u32 = 4096u32;
pub const TRANSACTION_NOTIFY_INDOUBT: u32 = 16384u32;
pub const TRANSACTION_NOTIFY_LAST_RECOVER: u32 = 8192u32;
pub const TRANSACTION_NOTIFY_MARSHAL: u32 = 131072u32;
pub const TRANSACTION_NOTIFY_MASK: u32 = 1073741823u32;
pub const TRANSACTION_NOTIFY_PREPARE: u32 = 2u32;
pub const TRANSACTION_NOTIFY_PREPARE_COMPLETE: u32 = 32u32;
pub const TRANSACTION_NOTIFY_PREPREPARE: u32 = 1u32;
pub const TRANSACTION_NOTIFY_PREPREPARE_COMPLETE: u32 = 16u32;
pub const TRANSACTION_NOTIFY_PROMOTE: u32 = 134217728u32;
pub const TRANSACTION_NOTIFY_PROMOTE_NEW: u32 = 268435456u32;
pub const TRANSACTION_NOTIFY_PROPAGATE_PULL: u32 = 32768u32;
pub const TRANSACTION_NOTIFY_PROPAGATE_PUSH: u32 = 65536u32;
pub const TRANSACTION_NOTIFY_RECOVER: u32 = 256u32;
pub const TRANSACTION_NOTIFY_RECOVER_QUERY: u32 = 2048u32;
pub const TRANSACTION_NOTIFY_REQUEST_OUTCOME: u32 = 536870912u32;
pub const TRANSACTION_NOTIFY_RM_DISCONNECTED: u32 = 16777216u32;
pub const TRANSACTION_NOTIFY_ROLLBACK: u32 = 8u32;
pub const TRANSACTION_NOTIFY_ROLLBACK_COMPLETE: u32 = 128u32;
pub const TRANSACTION_NOTIFY_SINGLE_PHASE_COMMIT: u32 = 512u32;
pub const TRANSACTION_NOTIFY_TM_ONLINE: u32 = 33554432u32;
pub const TRANSACTION_OBJECT_PATH: &'static str = "\\Transaction\\";
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TRANSACTION_OUTCOME(pub i32);
pub const TransactionOutcomeUndetermined: TRANSACTION_OUTCOME = TRANSACTION_OUTCOME(1i32);
pub const TransactionOutcomeCommitted: TRANSACTION_OUTCOME = TRANSACTION_OUTCOME(2i32);
pub const TransactionOutcomeAborted: TRANSACTION_OUTCOME = TRANSACTION_OUTCOME(3i32);
impl ::core::marker::Copy for TRANSACTION_OUTCOME {}
impl ::core::clone::Clone for TRANSACTION_OUTCOME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TRANSACTION_OUTCOME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TRANSACTION_OUTCOME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRANSACTION_OUTCOME").field(&self.0).finish()
    }
}
impl FromIntoMemory for TRANSACTION_OUTCOME {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TXFS_MINIVERSION(pub u32);
pub const TXFS_MINIVERSION_COMMITTED_VIEW: TXFS_MINIVERSION = TXFS_MINIVERSION(0u32);
pub const TXFS_MINIVERSION_DIRTY_VIEW: TXFS_MINIVERSION = TXFS_MINIVERSION(65535u32);
pub const TXFS_MINIVERSION_DEFAULT_VIEW: TXFS_MINIVERSION = TXFS_MINIVERSION(65534u32);
impl ::core::marker::Copy for TXFS_MINIVERSION {}
impl ::core::clone::Clone for TXFS_MINIVERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TXFS_MINIVERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TXFS_MINIVERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TXFS_MINIVERSION").field(&self.0).finish()
    }
}
impl FromIntoMemory for TXFS_MINIVERSION {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct TXF_LOG_RECORD_BASE {
    pub Version: u16,
    pub RecordType: TXF_LOG_RECORD_TYPE,
    pub RecordLength: u32,
}
impl ::core::marker::Copy for TXF_LOG_RECORD_BASE {}
impl ::core::clone::Clone for TXF_LOG_RECORD_BASE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TXF_LOG_RECORD_BASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXF_LOG_RECORD_BASE")
            .field("Version", &self.Version)
            .field("RecordType", &self.RecordType)
            .field("RecordLength", &self.RecordLength)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TXF_LOG_RECORD_BASE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.RecordType == other.RecordType
            && self.RecordLength == other.RecordLength
    }
}
impl ::core::cmp::Eq for TXF_LOG_RECORD_BASE {}
impl FromIntoMemory for TXF_LOG_RECORD_BASE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Version = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_RecordType = <TXF_LOG_RECORD_TYPE as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_RecordLength = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            Version: f_Version,
            RecordType: f_RecordType,
            RecordLength: f_RecordLength,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Version, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.RecordType, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.RecordLength, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub const TXF_LOG_RECORD_GENERIC_TYPE_ABORT: u32 = 2u32;
pub const TXF_LOG_RECORD_GENERIC_TYPE_COMMIT: u32 = 1u32;
pub const TXF_LOG_RECORD_GENERIC_TYPE_DATA: u32 = 8u32;
pub const TXF_LOG_RECORD_GENERIC_TYPE_PREPARE: u32 = 4u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TXF_LOG_RECORD_TYPE(pub u16);
pub const TXF_LOG_RECORD_TYPE_AFFECTED_FILE: TXF_LOG_RECORD_TYPE = TXF_LOG_RECORD_TYPE(4u16);
pub const TXF_LOG_RECORD_TYPE_TRUNCATE: TXF_LOG_RECORD_TYPE = TXF_LOG_RECORD_TYPE(2u16);
pub const TXF_LOG_RECORD_TYPE_WRITE: TXF_LOG_RECORD_TYPE = TXF_LOG_RECORD_TYPE(1u16);
impl ::core::marker::Copy for TXF_LOG_RECORD_TYPE {}
impl ::core::clone::Clone for TXF_LOG_RECORD_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TXF_LOG_RECORD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TXF_LOG_RECORD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TXF_LOG_RECORD_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for TXF_LOG_RECORD_TYPE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u16 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        2
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VER_FIND_FILE_FLAGS(pub u32);
pub const VFFF_ISSHAREDFILE: VER_FIND_FILE_FLAGS = VER_FIND_FILE_FLAGS(1u32);
impl ::core::marker::Copy for VER_FIND_FILE_FLAGS {}
impl ::core::clone::Clone for VER_FIND_FILE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VER_FIND_FILE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VER_FIND_FILE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VER_FIND_FILE_FLAGS").field(&self.0).finish()
    }
}
impl FromIntoMemory for VER_FIND_FILE_FLAGS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VER_FIND_FILE_STATUS(pub u32);
pub const VFF_CURNEDEST: VER_FIND_FILE_STATUS = VER_FIND_FILE_STATUS(1u32);
pub const VFF_FILEINUSE: VER_FIND_FILE_STATUS = VER_FIND_FILE_STATUS(2u32);
pub const VFF_BUFFTOOSMALL: VER_FIND_FILE_STATUS = VER_FIND_FILE_STATUS(4u32);
impl ::core::marker::Copy for VER_FIND_FILE_STATUS {}
impl ::core::clone::Clone for VER_FIND_FILE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VER_FIND_FILE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VER_FIND_FILE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VER_FIND_FILE_STATUS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for VER_FIND_FILE_STATUS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for VER_FIND_FILE_STATUS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for VER_FIND_FILE_STATUS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for VER_FIND_FILE_STATUS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for VER_FIND_FILE_STATUS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for VER_FIND_FILE_STATUS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VER_INSTALL_FILE_FLAGS(pub u32);
pub const VIFF_FORCEINSTALL: VER_INSTALL_FILE_FLAGS = VER_INSTALL_FILE_FLAGS(1u32);
pub const VIFF_DONTDELETEOLD: VER_INSTALL_FILE_FLAGS = VER_INSTALL_FILE_FLAGS(2u32);
impl ::core::marker::Copy for VER_INSTALL_FILE_FLAGS {}
impl ::core::clone::Clone for VER_INSTALL_FILE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VER_INSTALL_FILE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VER_INSTALL_FILE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VER_INSTALL_FILE_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for VER_INSTALL_FILE_FLAGS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VER_INSTALL_FILE_STATUS(pub u32);
pub const VIF_TEMPFILE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(1u32);
pub const VIF_MISMATCH: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(2u32);
pub const VIF_SRCOLD: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(4u32);
pub const VIF_DIFFLANG: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(8u32);
pub const VIF_DIFFCODEPG: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(16u32);
pub const VIF_DIFFTYPE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(32u32);
pub const VIF_WRITEPROT: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(64u32);
pub const VIF_FILEINUSE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(128u32);
pub const VIF_OUTOFSPACE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(256u32);
pub const VIF_ACCESSVIOLATION: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(512u32);
pub const VIF_SHARINGVIOLATION: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(1024u32);
pub const VIF_CANNOTCREATE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(2048u32);
pub const VIF_CANNOTDELETE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(4096u32);
pub const VIF_CANNOTRENAME: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(8192u32);
pub const VIF_CANNOTDELETECUR: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(16384u32);
pub const VIF_OUTOFMEMORY: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(32768u32);
pub const VIF_CANNOTREADSRC: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(65536u32);
pub const VIF_CANNOTREADDST: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(131072u32);
pub const VIF_BUFFTOOSMALL: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(262144u32);
pub const VIF_CANNOTLOADLZ32: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(524288u32);
pub const VIF_CANNOTLOADCABINET: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(1048576u32);
impl ::core::marker::Copy for VER_INSTALL_FILE_STATUS {}
impl ::core::clone::Clone for VER_INSTALL_FILE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VER_INSTALL_FILE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VER_INSTALL_FILE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VER_INSTALL_FILE_STATUS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for VER_INSTALL_FILE_STATUS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for VER_INSTALL_FILE_STATUS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for VER_INSTALL_FILE_STATUS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for VER_INSTALL_FILE_STATUS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for VER_INSTALL_FILE_STATUS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for VER_INSTALL_FILE_STATUS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct VOLUME_ALLOCATE_BC_STREAM_INPUT {
    pub Version: u32,
    pub RequestsPerPeriod: u32,
    pub Period: u32,
    pub RetryFailures: super::super::Foundation::BOOLEAN,
    pub Discardable: super::super::Foundation::BOOLEAN,
    pub Reserved1: [super::super::Foundation::BOOLEAN; 2],
    pub LowestByteOffset: u64,
    pub HighestByteOffset: u64,
    pub AccessType: u32,
    pub AccessMode: u32,
}
impl ::core::marker::Copy for VOLUME_ALLOCATE_BC_STREAM_INPUT {}
impl ::core::clone::Clone for VOLUME_ALLOCATE_BC_STREAM_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VOLUME_ALLOCATE_BC_STREAM_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_ALLOCATE_BC_STREAM_INPUT")
            .field("Version", &self.Version)
            .field("RequestsPerPeriod", &self.RequestsPerPeriod)
            .field("Period", &self.Period)
            .field("RetryFailures", &self.RetryFailures)
            .field("Discardable", &self.Discardable)
            .field("Reserved1", &self.Reserved1)
            .field("LowestByteOffset", &self.LowestByteOffset)
            .field("HighestByteOffset", &self.HighestByteOffset)
            .field("AccessType", &self.AccessType)
            .field("AccessMode", &self.AccessMode)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VOLUME_ALLOCATE_BC_STREAM_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.RequestsPerPeriod == other.RequestsPerPeriod
            && self.Period == other.Period
            && self.RetryFailures == other.RetryFailures
            && self.Discardable == other.Discardable
            && self.Reserved1 == other.Reserved1
            && self.LowestByteOffset == other.LowestByteOffset
            && self.HighestByteOffset == other.HighestByteOffset
            && self.AccessType == other.AccessType
            && self.AccessMode == other.AccessMode
    }
}
impl ::core::cmp::Eq for VOLUME_ALLOCATE_BC_STREAM_INPUT {}
impl FromIntoMemory for VOLUME_ALLOCATE_BC_STREAM_INPUT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_Version = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_RequestsPerPeriod = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Period = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_RetryFailures =
            <super::super::Foundation::BOOLEAN as FromIntoMemory>::from_bytes(&from[12..12 + 1]);
        let f_Discardable =
            <super::super::Foundation::BOOLEAN as FromIntoMemory>::from_bytes(&from[13..13 + 1]);
        let f_Reserved1 = <[super::super::Foundation::BOOLEAN; 2] as FromIntoMemory>::from_bytes(
            &from[14..14 + 2],
        );
        let f_LowestByteOffset = <u64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_HighestByteOffset = <u64 as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        let f_AccessType = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_AccessMode = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        Self {
            Version: f_Version,
            RequestsPerPeriod: f_RequestsPerPeriod,
            Period: f_Period,
            RetryFailures: f_RetryFailures,
            Discardable: f_Discardable,
            Reserved1: f_Reserved1,
            LowestByteOffset: f_LowestByteOffset,
            HighestByteOffset: f_HighestByteOffset,
            AccessType: f_AccessType,
            AccessMode: f_AccessMode,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.Version, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.RequestsPerPeriod, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Period, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.RetryFailures, &mut into[12..12 + 1]);
        FromIntoMemory::into_bytes(self.Discardable, &mut into[13..13 + 1]);
        FromIntoMemory::into_bytes(self.Reserved1, &mut into[14..14 + 2]);
        FromIntoMemory::into_bytes(self.LowestByteOffset, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.HighestByteOffset, &mut into[24..24 + 8]);
        FromIntoMemory::into_bytes(self.AccessType, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.AccessMode, &mut into[36..36 + 4]);
    }
    fn size() -> usize {
        40
    }
}
pub struct VOLUME_ALLOCATE_BC_STREAM_OUTPUT {
    pub RequestSize: u64,
    pub NumOutStandingRequests: u32,
}
impl ::core::marker::Copy for VOLUME_ALLOCATE_BC_STREAM_OUTPUT {}
impl ::core::clone::Clone for VOLUME_ALLOCATE_BC_STREAM_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VOLUME_ALLOCATE_BC_STREAM_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_ALLOCATE_BC_STREAM_OUTPUT")
            .field("RequestSize", &self.RequestSize)
            .field("NumOutStandingRequests", &self.NumOutStandingRequests)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VOLUME_ALLOCATE_BC_STREAM_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.RequestSize == other.RequestSize
            && self.NumOutStandingRequests == other.NumOutStandingRequests
    }
}
impl ::core::cmp::Eq for VOLUME_ALLOCATE_BC_STREAM_OUTPUT {}
impl FromIntoMemory for VOLUME_ALLOCATE_BC_STREAM_OUTPUT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_RequestSize = <u64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_NumOutStandingRequests = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            RequestSize: f_RequestSize,
            NumOutStandingRequests: f_NumOutStandingRequests,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.RequestSize, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.NumOutStandingRequests, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct VOLUME_ALLOCATION_HINT_INPUT {
    pub ClusterSize: u32,
    pub NumberOfClusters: u32,
    pub StartingClusterNumber: i64,
}
impl ::core::marker::Copy for VOLUME_ALLOCATION_HINT_INPUT {}
impl ::core::clone::Clone for VOLUME_ALLOCATION_HINT_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VOLUME_ALLOCATION_HINT_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_ALLOCATION_HINT_INPUT")
            .field("ClusterSize", &self.ClusterSize)
            .field("NumberOfClusters", &self.NumberOfClusters)
            .field("StartingClusterNumber", &self.StartingClusterNumber)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VOLUME_ALLOCATION_HINT_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.ClusterSize == other.ClusterSize
            && self.NumberOfClusters == other.NumberOfClusters
            && self.StartingClusterNumber == other.StartingClusterNumber
    }
}
impl ::core::cmp::Eq for VOLUME_ALLOCATION_HINT_INPUT {}
impl FromIntoMemory for VOLUME_ALLOCATION_HINT_INPUT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_ClusterSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_NumberOfClusters = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_StartingClusterNumber = <i64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        Self {
            ClusterSize: f_ClusterSize,
            NumberOfClusters: f_NumberOfClusters,
            StartingClusterNumber: f_StartingClusterNumber,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.ClusterSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.NumberOfClusters, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.StartingClusterNumber, &mut into[8..8 + 8]);
    }
    fn size() -> usize {
        16
    }
}
pub struct VOLUME_ALLOCATION_HINT_OUTPUT {
    pub Bitmap: [u32; 1],
}
impl ::core::marker::Copy for VOLUME_ALLOCATION_HINT_OUTPUT {}
impl ::core::clone::Clone for VOLUME_ALLOCATION_HINT_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VOLUME_ALLOCATION_HINT_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_ALLOCATION_HINT_OUTPUT")
            .field("Bitmap", &self.Bitmap)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VOLUME_ALLOCATION_HINT_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Bitmap == other.Bitmap
    }
}
impl ::core::cmp::Eq for VOLUME_ALLOCATION_HINT_OUTPUT {}
impl FromIntoMemory for VOLUME_ALLOCATION_HINT_OUTPUT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_Bitmap = <[u32; 1] as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self { Bitmap: f_Bitmap }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.Bitmap, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct VOLUME_CRITICAL_IO {
    pub AccessType: u32,
    pub ExtentsCount: u32,
    pub Extents: [FILE_EXTENT; 1],
}
impl ::core::marker::Copy for VOLUME_CRITICAL_IO {}
impl ::core::clone::Clone for VOLUME_CRITICAL_IO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VOLUME_CRITICAL_IO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_CRITICAL_IO")
            .field("AccessType", &self.AccessType)
            .field("ExtentsCount", &self.ExtentsCount)
            .field("Extents", &self.Extents)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VOLUME_CRITICAL_IO {
    fn eq(&self, other: &Self) -> bool {
        self.AccessType == other.AccessType
            && self.ExtentsCount == other.ExtentsCount
            && self.Extents == other.Extents
    }
}
impl ::core::cmp::Eq for VOLUME_CRITICAL_IO {}
impl FromIntoMemory for VOLUME_CRITICAL_IO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_AccessType = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ExtentsCount = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Extents = <[FILE_EXTENT; 1] as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        Self {
            AccessType: f_AccessType,
            ExtentsCount: f_ExtentsCount,
            Extents: f_Extents,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.AccessType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ExtentsCount, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Extents, &mut into[8..8 + 16]);
    }
    fn size() -> usize {
        24
    }
}
pub struct VOLUME_FAILOVER_SET {
    pub NumberOfDisks: u32,
    pub DiskNumbers: [u32; 1],
}
impl ::core::marker::Copy for VOLUME_FAILOVER_SET {}
impl ::core::clone::Clone for VOLUME_FAILOVER_SET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VOLUME_FAILOVER_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_FAILOVER_SET")
            .field("NumberOfDisks", &self.NumberOfDisks)
            .field("DiskNumbers", &self.DiskNumbers)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VOLUME_FAILOVER_SET {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfDisks == other.NumberOfDisks && self.DiskNumbers == other.DiskNumbers
    }
}
impl ::core::cmp::Eq for VOLUME_FAILOVER_SET {}
impl FromIntoMemory for VOLUME_FAILOVER_SET {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_NumberOfDisks = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_DiskNumbers = <[u32; 1] as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            NumberOfDisks: f_NumberOfDisks,
            DiskNumbers: f_DiskNumbers,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.NumberOfDisks, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.DiskNumbers, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct VOLUME_GET_BC_PROPERTIES_INPUT {
    pub Version: u32,
    pub Reserved1: u32,
    pub LowestByteOffset: u64,
    pub HighestByteOffset: u64,
    pub AccessType: u32,
    pub AccessMode: u32,
}
impl ::core::marker::Copy for VOLUME_GET_BC_PROPERTIES_INPUT {}
impl ::core::clone::Clone for VOLUME_GET_BC_PROPERTIES_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VOLUME_GET_BC_PROPERTIES_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_GET_BC_PROPERTIES_INPUT")
            .field("Version", &self.Version)
            .field("Reserved1", &self.Reserved1)
            .field("LowestByteOffset", &self.LowestByteOffset)
            .field("HighestByteOffset", &self.HighestByteOffset)
            .field("AccessType", &self.AccessType)
            .field("AccessMode", &self.AccessMode)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VOLUME_GET_BC_PROPERTIES_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Reserved1 == other.Reserved1
            && self.LowestByteOffset == other.LowestByteOffset
            && self.HighestByteOffset == other.HighestByteOffset
            && self.AccessType == other.AccessType
            && self.AccessMode == other.AccessMode
    }
}
impl ::core::cmp::Eq for VOLUME_GET_BC_PROPERTIES_INPUT {}
impl FromIntoMemory for VOLUME_GET_BC_PROPERTIES_INPUT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_Version = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Reserved1 = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_LowestByteOffset = <u64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_HighestByteOffset = <u64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_AccessType = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_AccessMode = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            Version: f_Version,
            Reserved1: f_Reserved1,
            LowestByteOffset: f_LowestByteOffset,
            HighestByteOffset: f_HighestByteOffset,
            AccessType: f_AccessType,
            AccessMode: f_AccessMode,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.Version, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Reserved1, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.LowestByteOffset, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.HighestByteOffset, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.AccessType, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.AccessMode, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
pub struct VOLUME_GET_BC_PROPERTIES_OUTPUT {
    pub MaximumRequestsPerPeriod: u32,
    pub MinimumPeriod: u32,
    pub MaximumRequestSize: u64,
    pub EstimatedTimePerRequest: u32,
    pub NumOutStandingRequests: u32,
    pub RequestSize: u64,
}
impl ::core::marker::Copy for VOLUME_GET_BC_PROPERTIES_OUTPUT {}
impl ::core::clone::Clone for VOLUME_GET_BC_PROPERTIES_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VOLUME_GET_BC_PROPERTIES_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_GET_BC_PROPERTIES_OUTPUT")
            .field("MaximumRequestsPerPeriod", &self.MaximumRequestsPerPeriod)
            .field("MinimumPeriod", &self.MinimumPeriod)
            .field("MaximumRequestSize", &self.MaximumRequestSize)
            .field("EstimatedTimePerRequest", &self.EstimatedTimePerRequest)
            .field("NumOutStandingRequests", &self.NumOutStandingRequests)
            .field("RequestSize", &self.RequestSize)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VOLUME_GET_BC_PROPERTIES_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.MaximumRequestsPerPeriod == other.MaximumRequestsPerPeriod
            && self.MinimumPeriod == other.MinimumPeriod
            && self.MaximumRequestSize == other.MaximumRequestSize
            && self.EstimatedTimePerRequest == other.EstimatedTimePerRequest
            && self.NumOutStandingRequests == other.NumOutStandingRequests
            && self.RequestSize == other.RequestSize
    }
}
impl ::core::cmp::Eq for VOLUME_GET_BC_PROPERTIES_OUTPUT {}
impl FromIntoMemory for VOLUME_GET_BC_PROPERTIES_OUTPUT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_MaximumRequestsPerPeriod = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_MinimumPeriod = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_MaximumRequestSize = <u64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_EstimatedTimePerRequest = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_NumOutStandingRequests = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_RequestSize = <u64 as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        Self {
            MaximumRequestsPerPeriod: f_MaximumRequestsPerPeriod,
            MinimumPeriod: f_MinimumPeriod,
            MaximumRequestSize: f_MaximumRequestSize,
            EstimatedTimePerRequest: f_EstimatedTimePerRequest,
            NumOutStandingRequests: f_NumOutStandingRequests,
            RequestSize: f_RequestSize,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.MaximumRequestsPerPeriod, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.MinimumPeriod, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.MaximumRequestSize, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.EstimatedTimePerRequest, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.NumOutStandingRequests, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.RequestSize, &mut into[24..24 + 8]);
    }
    fn size() -> usize {
        32
    }
}
pub struct VOLUME_LOGICAL_OFFSET {
    pub LogicalOffset: i64,
}
impl ::core::marker::Copy for VOLUME_LOGICAL_OFFSET {}
impl ::core::clone::Clone for VOLUME_LOGICAL_OFFSET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VOLUME_LOGICAL_OFFSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_LOGICAL_OFFSET")
            .field("LogicalOffset", &self.LogicalOffset)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VOLUME_LOGICAL_OFFSET {
    fn eq(&self, other: &Self) -> bool {
        self.LogicalOffset == other.LogicalOffset
    }
}
impl ::core::cmp::Eq for VOLUME_LOGICAL_OFFSET {}
impl FromIntoMemory for VOLUME_LOGICAL_OFFSET {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_LogicalOffset = <i64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        Self {
            LogicalOffset: f_LogicalOffset,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.LogicalOffset, &mut into[0..0 + 8]);
    }
    fn size() -> usize {
        8
    }
}
pub struct VOLUME_NUMBER {
    pub VolumeNumber: u32,
    pub VolumeManagerName: [u16; 8],
}
impl ::core::marker::Copy for VOLUME_NUMBER {}
impl ::core::clone::Clone for VOLUME_NUMBER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VOLUME_NUMBER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_NUMBER")
            .field("VolumeNumber", &self.VolumeNumber)
            .field("VolumeManagerName", &self.VolumeManagerName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VOLUME_NUMBER {
    fn eq(&self, other: &Self) -> bool {
        self.VolumeNumber == other.VolumeNumber && self.VolumeManagerName == other.VolumeManagerName
    }
}
impl ::core::cmp::Eq for VOLUME_NUMBER {}
impl FromIntoMemory for VOLUME_NUMBER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_VolumeNumber = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_VolumeManagerName = <[u16; 8] as FromIntoMemory>::from_bytes(&from[4..4 + 8]);
        Self {
            VolumeNumber: f_VolumeNumber,
            VolumeManagerName: f_VolumeManagerName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.VolumeNumber, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.VolumeManagerName, &mut into[4..4 + 8]);
    }
    fn size() -> usize {
        12
    }
}
pub struct VOLUME_PHYSICAL_OFFSET {
    pub DiskNumber: u32,
    pub Offset: i64,
}
impl ::core::marker::Copy for VOLUME_PHYSICAL_OFFSET {}
impl ::core::clone::Clone for VOLUME_PHYSICAL_OFFSET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VOLUME_PHYSICAL_OFFSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_PHYSICAL_OFFSET")
            .field("DiskNumber", &self.DiskNumber)
            .field("Offset", &self.Offset)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VOLUME_PHYSICAL_OFFSET {
    fn eq(&self, other: &Self) -> bool {
        self.DiskNumber == other.DiskNumber && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for VOLUME_PHYSICAL_OFFSET {}
impl FromIntoMemory for VOLUME_PHYSICAL_OFFSET {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_DiskNumber = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Offset = <i64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        Self {
            DiskNumber: f_DiskNumber,
            Offset: f_Offset,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.DiskNumber, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Offset, &mut into[8..8 + 8]);
    }
    fn size() -> usize {
        16
    }
}
pub struct VOLUME_PHYSICAL_OFFSETS {
    pub NumberOfPhysicalOffsets: u32,
    pub PhysicalOffset: [VOLUME_PHYSICAL_OFFSET; 1],
}
impl ::core::marker::Copy for VOLUME_PHYSICAL_OFFSETS {}
impl ::core::clone::Clone for VOLUME_PHYSICAL_OFFSETS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VOLUME_PHYSICAL_OFFSETS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_PHYSICAL_OFFSETS")
            .field("NumberOfPhysicalOffsets", &self.NumberOfPhysicalOffsets)
            .field("PhysicalOffset", &self.PhysicalOffset)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VOLUME_PHYSICAL_OFFSETS {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfPhysicalOffsets == other.NumberOfPhysicalOffsets
            && self.PhysicalOffset == other.PhysicalOffset
    }
}
impl ::core::cmp::Eq for VOLUME_PHYSICAL_OFFSETS {}
impl FromIntoMemory for VOLUME_PHYSICAL_OFFSETS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_NumberOfPhysicalOffsets = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_PhysicalOffset =
            <[VOLUME_PHYSICAL_OFFSET; 1] as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        Self {
            NumberOfPhysicalOffsets: f_NumberOfPhysicalOffsets,
            PhysicalOffset: f_PhysicalOffset,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.NumberOfPhysicalOffsets, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.PhysicalOffset, &mut into[8..8 + 16]);
    }
    fn size() -> usize {
        24
    }
}
pub struct VOLUME_READ_PLEX_INPUT {
    pub ByteOffset: i64,
    pub Length: u32,
    pub PlexNumber: u32,
}
impl ::core::marker::Copy for VOLUME_READ_PLEX_INPUT {}
impl ::core::clone::Clone for VOLUME_READ_PLEX_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VOLUME_READ_PLEX_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_READ_PLEX_INPUT")
            .field("ByteOffset", &self.ByteOffset)
            .field("Length", &self.Length)
            .field("PlexNumber", &self.PlexNumber)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VOLUME_READ_PLEX_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.ByteOffset == other.ByteOffset
            && self.Length == other.Length
            && self.PlexNumber == other.PlexNumber
    }
}
impl ::core::cmp::Eq for VOLUME_READ_PLEX_INPUT {}
impl FromIntoMemory for VOLUME_READ_PLEX_INPUT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_ByteOffset = <i64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_Length = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_PlexNumber = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            ByteOffset: f_ByteOffset,
            Length: f_Length,
            PlexNumber: f_PlexNumber,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.ByteOffset, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.Length, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.PlexNumber, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {
    pub GptAttributes: u64,
    pub RevertOnClose: super::super::Foundation::BOOLEAN,
    pub ApplyToAllConnectedVolumes: super::super::Foundation::BOOLEAN,
    pub Reserved1: u16,
    pub Reserved2: u32,
}
impl ::core::marker::Copy for VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {}
impl ::core::clone::Clone for VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_SET_GPT_ATTRIBUTES_INFORMATION")
            .field("GptAttributes", &self.GptAttributes)
            .field("RevertOnClose", &self.RevertOnClose)
            .field(
                "ApplyToAllConnectedVolumes",
                &self.ApplyToAllConnectedVolumes,
            )
            .field("Reserved1", &self.Reserved1)
            .field("Reserved2", &self.Reserved2)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.GptAttributes == other.GptAttributes
            && self.RevertOnClose == other.RevertOnClose
            && self.ApplyToAllConnectedVolumes == other.ApplyToAllConnectedVolumes
            && self.Reserved1 == other.Reserved1
            && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {}
impl FromIntoMemory for VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_GptAttributes = <u64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_RevertOnClose =
            <super::super::Foundation::BOOLEAN as FromIntoMemory>::from_bytes(&from[8..8 + 1]);
        let f_ApplyToAllConnectedVolumes =
            <super::super::Foundation::BOOLEAN as FromIntoMemory>::from_bytes(&from[9..9 + 1]);
        let f_Reserved1 = <u16 as FromIntoMemory>::from_bytes(&from[10..10 + 2]);
        let f_Reserved2 = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            GptAttributes: f_GptAttributes,
            RevertOnClose: f_RevertOnClose,
            ApplyToAllConnectedVolumes: f_ApplyToAllConnectedVolumes,
            Reserved1: f_Reserved1,
            Reserved2: f_Reserved2,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.GptAttributes, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.RevertOnClose, &mut into[8..8 + 1]);
        FromIntoMemory::into_bytes(self.ApplyToAllConnectedVolumes, &mut into[9..9 + 1]);
        FromIntoMemory::into_bytes(self.Reserved1, &mut into[10..10 + 2]);
        FromIntoMemory::into_bytes(self.Reserved2, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct VOLUME_SHRINK_INFO {
    pub VolumeSize: u64,
}
impl ::core::marker::Copy for VOLUME_SHRINK_INFO {}
impl ::core::clone::Clone for VOLUME_SHRINK_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VOLUME_SHRINK_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_SHRINK_INFO")
            .field("VolumeSize", &self.VolumeSize)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VOLUME_SHRINK_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.VolumeSize == other.VolumeSize
    }
}
impl ::core::cmp::Eq for VOLUME_SHRINK_INFO {}
impl FromIntoMemory for VOLUME_SHRINK_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_VolumeSize = <u64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        Self {
            VolumeSize: f_VolumeSize,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.VolumeSize, &mut into[0..0 + 8]);
    }
    fn size() -> usize {
        8
    }
}
pub const VS_FFI_FILEFLAGSMASK: i32 = 63i32;
pub const VS_FFI_SIGNATURE: i32 = -17890115i32;
pub const VS_FFI_STRUCVERSION: i32 = 65536i32;
pub struct VS_FIXEDFILEINFO {
    pub dwSignature: u32,
    pub dwStrucVersion: u32,
    pub dwFileVersionMS: u32,
    pub dwFileVersionLS: u32,
    pub dwProductVersionMS: u32,
    pub dwProductVersionLS: u32,
    pub dwFileFlagsMask: u32,
    pub dwFileFlags: VS_FIXEDFILEINFO_FILE_FLAGS,
    pub dwFileOS: VS_FIXEDFILEINFO_FILE_OS,
    pub dwFileType: VS_FIXEDFILEINFO_FILE_TYPE,
    pub dwFileSubtype: VS_FIXEDFILEINFO_FILE_SUBTYPE,
    pub dwFileDateMS: u32,
    pub dwFileDateLS: u32,
}
impl ::core::marker::Copy for VS_FIXEDFILEINFO {}
impl ::core::clone::Clone for VS_FIXEDFILEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VS_FIXEDFILEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VS_FIXEDFILEINFO")
            .field("dwSignature", &self.dwSignature)
            .field("dwStrucVersion", &self.dwStrucVersion)
            .field("dwFileVersionMS", &self.dwFileVersionMS)
            .field("dwFileVersionLS", &self.dwFileVersionLS)
            .field("dwProductVersionMS", &self.dwProductVersionMS)
            .field("dwProductVersionLS", &self.dwProductVersionLS)
            .field("dwFileFlagsMask", &self.dwFileFlagsMask)
            .field("dwFileFlags", &self.dwFileFlags)
            .field("dwFileOS", &self.dwFileOS)
            .field("dwFileType", &self.dwFileType)
            .field("dwFileSubtype", &self.dwFileSubtype)
            .field("dwFileDateMS", &self.dwFileDateMS)
            .field("dwFileDateLS", &self.dwFileDateLS)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VS_FIXEDFILEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSignature == other.dwSignature
            && self.dwStrucVersion == other.dwStrucVersion
            && self.dwFileVersionMS == other.dwFileVersionMS
            && self.dwFileVersionLS == other.dwFileVersionLS
            && self.dwProductVersionMS == other.dwProductVersionMS
            && self.dwProductVersionLS == other.dwProductVersionLS
            && self.dwFileFlagsMask == other.dwFileFlagsMask
            && self.dwFileFlags == other.dwFileFlags
            && self.dwFileOS == other.dwFileOS
            && self.dwFileType == other.dwFileType
            && self.dwFileSubtype == other.dwFileSubtype
            && self.dwFileDateMS == other.dwFileDateMS
            && self.dwFileDateLS == other.dwFileDateLS
    }
}
impl ::core::cmp::Eq for VS_FIXEDFILEINFO {}
impl FromIntoMemory for VS_FIXEDFILEINFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 52);
        let f_dwSignature = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwStrucVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwFileVersionMS = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwFileVersionLS = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwProductVersionMS = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwProductVersionLS = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwFileFlagsMask = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_dwFileFlags =
            <VS_FIXEDFILEINFO_FILE_FLAGS as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_dwFileOS =
            <VS_FIXEDFILEINFO_FILE_OS as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_dwFileType =
            <VS_FIXEDFILEINFO_FILE_TYPE as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_dwFileSubtype =
            <VS_FIXEDFILEINFO_FILE_SUBTYPE as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_dwFileDateMS = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_dwFileDateLS = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        Self {
            dwSignature: f_dwSignature,
            dwStrucVersion: f_dwStrucVersion,
            dwFileVersionMS: f_dwFileVersionMS,
            dwFileVersionLS: f_dwFileVersionLS,
            dwProductVersionMS: f_dwProductVersionMS,
            dwProductVersionLS: f_dwProductVersionLS,
            dwFileFlagsMask: f_dwFileFlagsMask,
            dwFileFlags: f_dwFileFlags,
            dwFileOS: f_dwFileOS,
            dwFileType: f_dwFileType,
            dwFileSubtype: f_dwFileSubtype,
            dwFileDateMS: f_dwFileDateMS,
            dwFileDateLS: f_dwFileDateLS,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 52);
        FromIntoMemory::into_bytes(self.dwSignature, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwStrucVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwFileVersionMS, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwFileVersionLS, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwProductVersionMS, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwProductVersionLS, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwFileFlagsMask, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.dwFileFlags, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.dwFileOS, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.dwFileType, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.dwFileSubtype, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.dwFileDateMS, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.dwFileDateLS, &mut into[48..48 + 4]);
    }
    fn size() -> usize {
        52
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VS_FIXEDFILEINFO_FILE_FLAGS(pub u32);
pub const VS_FF_DEBUG: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(1u32);
pub const VS_FF_PRERELEASE: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(2u32);
pub const VS_FF_PATCHED: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(4u32);
pub const VS_FF_PRIVATEBUILD: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(8u32);
pub const VS_FF_INFOINFERRED: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(16u32);
pub const VS_FF_SPECIALBUILD: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(32u32);
impl ::core::marker::Copy for VS_FIXEDFILEINFO_FILE_FLAGS {}
impl ::core::clone::Clone for VS_FIXEDFILEINFO_FILE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VS_FIXEDFILEINFO_FILE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VS_FIXEDFILEINFO_FILE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VS_FIXEDFILEINFO_FILE_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for VS_FIXEDFILEINFO_FILE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for VS_FIXEDFILEINFO_FILE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for VS_FIXEDFILEINFO_FILE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for VS_FIXEDFILEINFO_FILE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for VS_FIXEDFILEINFO_FILE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for VS_FIXEDFILEINFO_FILE_FLAGS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VS_FIXEDFILEINFO_FILE_OS(pub i32);
pub const VOS_UNKNOWN: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(0i32);
pub const VOS_DOS: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(65536i32);
pub const VOS_OS216: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(131072i32);
pub const VOS_OS232: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(196608i32);
pub const VOS_NT: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(262144i32);
pub const VOS_WINCE: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(327680i32);
pub const VOS__BASE: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(0i32);
pub const VOS__WINDOWS16: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(1i32);
pub const VOS__PM16: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(2i32);
pub const VOS__PM32: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(3i32);
pub const VOS__WINDOWS32: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(4i32);
pub const VOS_DOS_WINDOWS16: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(65537i32);
pub const VOS_DOS_WINDOWS32: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(65540i32);
pub const VOS_OS216_PM16: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(131074i32);
pub const VOS_OS232_PM32: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(196611i32);
pub const VOS_NT_WINDOWS32: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(262148i32);
impl ::core::marker::Copy for VS_FIXEDFILEINFO_FILE_OS {}
impl ::core::clone::Clone for VS_FIXEDFILEINFO_FILE_OS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VS_FIXEDFILEINFO_FILE_OS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VS_FIXEDFILEINFO_FILE_OS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VS_FIXEDFILEINFO_FILE_OS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for VS_FIXEDFILEINFO_FILE_OS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VS_FIXEDFILEINFO_FILE_SUBTYPE(pub i32);
pub const VFT2_UNKNOWN: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(0i32);
pub const VFT2_DRV_PRINTER: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(1i32);
pub const VFT2_DRV_KEYBOARD: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(2i32);
pub const VFT2_DRV_LANGUAGE: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(3i32);
pub const VFT2_DRV_DISPLAY: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(4i32);
pub const VFT2_DRV_MOUSE: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(5i32);
pub const VFT2_DRV_NETWORK: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(6i32);
pub const VFT2_DRV_SYSTEM: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(7i32);
pub const VFT2_DRV_INSTALLABLE: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(8i32);
pub const VFT2_DRV_SOUND: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(9i32);
pub const VFT2_DRV_COMM: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(10i32);
pub const VFT2_DRV_INPUTMETHOD: VS_FIXEDFILEINFO_FILE_SUBTYPE =
    VS_FIXEDFILEINFO_FILE_SUBTYPE(11i32);
pub const VFT2_DRV_VERSIONED_PRINTER: VS_FIXEDFILEINFO_FILE_SUBTYPE =
    VS_FIXEDFILEINFO_FILE_SUBTYPE(12i32);
pub const VFT2_FONT_RASTER: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(1i32);
pub const VFT2_FONT_VECTOR: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(2i32);
pub const VFT2_FONT_TRUETYPE: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(3i32);
impl ::core::marker::Copy for VS_FIXEDFILEINFO_FILE_SUBTYPE {}
impl ::core::clone::Clone for VS_FIXEDFILEINFO_FILE_SUBTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VS_FIXEDFILEINFO_FILE_SUBTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VS_FIXEDFILEINFO_FILE_SUBTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VS_FIXEDFILEINFO_FILE_SUBTYPE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for VS_FIXEDFILEINFO_FILE_SUBTYPE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VS_FIXEDFILEINFO_FILE_TYPE(pub i32);
pub const VFT_UNKNOWN: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(0i32);
pub const VFT_APP: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(1i32);
pub const VFT_DLL: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(2i32);
pub const VFT_DRV: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(3i32);
pub const VFT_FONT: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(4i32);
pub const VFT_VXD: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(5i32);
pub const VFT_STATIC_LIB: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(7i32);
impl ::core::marker::Copy for VS_FIXEDFILEINFO_FILE_TYPE {}
impl ::core::clone::Clone for VS_FIXEDFILEINFO_FILE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VS_FIXEDFILEINFO_FILE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VS_FIXEDFILEINFO_FILE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VS_FIXEDFILEINFO_FILE_TYPE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for VS_FIXEDFILEINFO_FILE_TYPE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub const VS_USER_DEFINED: u32 = 100u32;
pub const VS_VERSION_INFO: u32 = 1u32;
pub const WIM_BOOT_NOT_OS_WIM: u32 = 0u32;
pub const WIM_BOOT_OS_WIM: u32 = 1u32;
pub const WIM_ENTRY_FLAG_NOT_ACTIVE: u32 = 1u32;
pub const WIM_ENTRY_FLAG_SUSPENDED: u32 = 2u32;
pub struct WIM_ENTRY_INFO {
    pub WimEntryInfoSize: u32,
    pub WimType: u32,
    pub DataSourceId: i64,
    pub WimGuid: crate::core::GUID,
    pub WimPath: PCWSTR,
    pub WimIndex: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for WIM_ENTRY_INFO {}
impl ::core::clone::Clone for WIM_ENTRY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIM_ENTRY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIM_ENTRY_INFO")
            .field("WimEntryInfoSize", &self.WimEntryInfoSize)
            .field("WimType", &self.WimType)
            .field("DataSourceId", &self.DataSourceId)
            .field("WimGuid", &self.WimGuid)
            .field("WimPath", &self.WimPath)
            .field("WimIndex", &self.WimIndex)
            .field("Flags", &self.Flags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WIM_ENTRY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.WimEntryInfoSize == other.WimEntryInfoSize
            && self.WimType == other.WimType
            && self.DataSourceId == other.DataSourceId
            && self.WimGuid == other.WimGuid
            && self.WimPath == other.WimPath
            && self.WimIndex == other.WimIndex
            && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for WIM_ENTRY_INFO {}
impl FromIntoMemory for WIM_ENTRY_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_WimEntryInfoSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_WimType = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_DataSourceId = <i64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_WimGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[16..16 + 16]);
        let f_WimPath = <PCWSTR as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_WimIndex = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_Flags = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        Self {
            WimEntryInfoSize: f_WimEntryInfoSize,
            WimType: f_WimType,
            DataSourceId: f_DataSourceId,
            WimGuid: f_WimGuid,
            WimPath: f_WimPath,
            WimIndex: f_WimIndex,
            Flags: f_Flags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.WimEntryInfoSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.WimType, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.DataSourceId, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.WimGuid, &mut into[16..16 + 16]);
        FromIntoMemory::into_bytes(self.WimPath, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.WimIndex, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[40..40 + 4]);
    }
    fn size() -> usize {
        48
    }
}
pub struct WIM_EXTERNAL_FILE_INFO {
    pub DataSourceId: i64,
    pub ResourceHash: [u8; 20],
    pub Flags: u32,
}
impl ::core::marker::Copy for WIM_EXTERNAL_FILE_INFO {}
impl ::core::clone::Clone for WIM_EXTERNAL_FILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIM_EXTERNAL_FILE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIM_EXTERNAL_FILE_INFO")
            .field("DataSourceId", &self.DataSourceId)
            .field("ResourceHash", &self.ResourceHash)
            .field("Flags", &self.Flags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WIM_EXTERNAL_FILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.DataSourceId == other.DataSourceId
            && self.ResourceHash == other.ResourceHash
            && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for WIM_EXTERNAL_FILE_INFO {}
impl FromIntoMemory for WIM_EXTERNAL_FILE_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_DataSourceId = <i64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_ResourceHash = <[u8; 20] as FromIntoMemory>::from_bytes(&from[8..8 + 20]);
        let f_Flags = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            DataSourceId: f_DataSourceId,
            ResourceHash: f_ResourceHash,
            Flags: f_Flags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.DataSourceId, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.ResourceHash, &mut into[8..8 + 20]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
pub const WIM_EXTERNAL_FILE_INFO_FLAG_NOT_ACTIVE: u32 = 1u32;
pub const WIM_EXTERNAL_FILE_INFO_FLAG_SUSPENDED: u32 = 2u32;
pub const WIM_PROVIDER_HASH_SIZE: u32 = 20u32;
pub struct WIN32_FILE_ATTRIBUTE_DATA {
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::super::Foundation::FILETIME,
    pub ftLastAccessTime: super::super::Foundation::FILETIME,
    pub ftLastWriteTime: super::super::Foundation::FILETIME,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
}
impl ::core::marker::Copy for WIN32_FILE_ATTRIBUTE_DATA {}
impl ::core::clone::Clone for WIN32_FILE_ATTRIBUTE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIN32_FILE_ATTRIBUTE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN32_FILE_ATTRIBUTE_DATA")
            .field("dwFileAttributes", &self.dwFileAttributes)
            .field("ftCreationTime", &self.ftCreationTime)
            .field("ftLastAccessTime", &self.ftLastAccessTime)
            .field("ftLastWriteTime", &self.ftLastWriteTime)
            .field("nFileSizeHigh", &self.nFileSizeHigh)
            .field("nFileSizeLow", &self.nFileSizeLow)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WIN32_FILE_ATTRIBUTE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwFileAttributes == other.dwFileAttributes
            && self.ftCreationTime == other.ftCreationTime
            && self.ftLastAccessTime == other.ftLastAccessTime
            && self.ftLastWriteTime == other.ftLastWriteTime
            && self.nFileSizeHigh == other.nFileSizeHigh
            && self.nFileSizeLow == other.nFileSizeLow
    }
}
impl ::core::cmp::Eq for WIN32_FILE_ATTRIBUTE_DATA {}
impl FromIntoMemory for WIN32_FILE_ATTRIBUTE_DATA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 36);
        let f_dwFileAttributes = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ftCreationTime =
            <super::super::Foundation::FILETIME as FromIntoMemory>::from_bytes(&from[4..4 + 8]);
        let f_ftLastAccessTime =
            <super::super::Foundation::FILETIME as FromIntoMemory>::from_bytes(&from[12..12 + 8]);
        let f_ftLastWriteTime =
            <super::super::Foundation::FILETIME as FromIntoMemory>::from_bytes(&from[20..20 + 8]);
        let f_nFileSizeHigh = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_nFileSizeLow = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        Self {
            dwFileAttributes: f_dwFileAttributes,
            ftCreationTime: f_ftCreationTime,
            ftLastAccessTime: f_ftLastAccessTime,
            ftLastWriteTime: f_ftLastWriteTime,
            nFileSizeHigh: f_nFileSizeHigh,
            nFileSizeLow: f_nFileSizeLow,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 36);
        FromIntoMemory::into_bytes(self.dwFileAttributes, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ftCreationTime, &mut into[4..4 + 8]);
        FromIntoMemory::into_bytes(self.ftLastAccessTime, &mut into[12..12 + 8]);
        FromIntoMemory::into_bytes(self.ftLastWriteTime, &mut into[20..20 + 8]);
        FromIntoMemory::into_bytes(self.nFileSizeHigh, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.nFileSizeLow, &mut into[32..32 + 4]);
    }
    fn size() -> usize {
        36
    }
}
pub struct WIN32_FIND_DATAA {
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::super::Foundation::FILETIME,
    pub ftLastAccessTime: super::super::Foundation::FILETIME,
    pub ftLastWriteTime: super::super::Foundation::FILETIME,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
    pub dwReserved0: u32,
    pub dwReserved1: u32,
    pub cFileName: [super::super::Foundation::CHAR; 260],
    pub cAlternateFileName: [super::super::Foundation::CHAR; 14],
}
impl ::core::marker::Copy for WIN32_FIND_DATAA {}
impl ::core::clone::Clone for WIN32_FIND_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIN32_FIND_DATAA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN32_FIND_DATAA")
            .field("dwFileAttributes", &self.dwFileAttributes)
            .field("ftCreationTime", &self.ftCreationTime)
            .field("ftLastAccessTime", &self.ftLastAccessTime)
            .field("ftLastWriteTime", &self.ftLastWriteTime)
            .field("nFileSizeHigh", &self.nFileSizeHigh)
            .field("nFileSizeLow", &self.nFileSizeLow)
            .field("dwReserved0", &self.dwReserved0)
            .field("dwReserved1", &self.dwReserved1)
            .field("cFileName", &self.cFileName)
            .field("cAlternateFileName", &self.cAlternateFileName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WIN32_FIND_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.dwFileAttributes == other.dwFileAttributes
            && self.ftCreationTime == other.ftCreationTime
            && self.ftLastAccessTime == other.ftLastAccessTime
            && self.ftLastWriteTime == other.ftLastWriteTime
            && self.nFileSizeHigh == other.nFileSizeHigh
            && self.nFileSizeLow == other.nFileSizeLow
            && self.dwReserved0 == other.dwReserved0
            && self.dwReserved1 == other.dwReserved1
            && self.cFileName == other.cFileName
            && self.cAlternateFileName == other.cAlternateFileName
    }
}
impl ::core::cmp::Eq for WIN32_FIND_DATAA {}
impl FromIntoMemory for WIN32_FIND_DATAA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 320);
        let f_dwFileAttributes = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ftCreationTime =
            <super::super::Foundation::FILETIME as FromIntoMemory>::from_bytes(&from[4..4 + 8]);
        let f_ftLastAccessTime =
            <super::super::Foundation::FILETIME as FromIntoMemory>::from_bytes(&from[12..12 + 8]);
        let f_ftLastWriteTime =
            <super::super::Foundation::FILETIME as FromIntoMemory>::from_bytes(&from[20..20 + 8]);
        let f_nFileSizeHigh = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_nFileSizeLow = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_dwReserved0 = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_dwReserved1 = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_cFileName = <[super::super::Foundation::CHAR; 260] as FromIntoMemory>::from_bytes(
            &from[44..44 + 260],
        );
        let f_cAlternateFileName =
            <[super::super::Foundation::CHAR; 14] as FromIntoMemory>::from_bytes(
                &from[304..304 + 14],
            );
        Self {
            dwFileAttributes: f_dwFileAttributes,
            ftCreationTime: f_ftCreationTime,
            ftLastAccessTime: f_ftLastAccessTime,
            ftLastWriteTime: f_ftLastWriteTime,
            nFileSizeHigh: f_nFileSizeHigh,
            nFileSizeLow: f_nFileSizeLow,
            dwReserved0: f_dwReserved0,
            dwReserved1: f_dwReserved1,
            cFileName: f_cFileName,
            cAlternateFileName: f_cAlternateFileName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 320);
        FromIntoMemory::into_bytes(self.dwFileAttributes, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ftCreationTime, &mut into[4..4 + 8]);
        FromIntoMemory::into_bytes(self.ftLastAccessTime, &mut into[12..12 + 8]);
        FromIntoMemory::into_bytes(self.ftLastWriteTime, &mut into[20..20 + 8]);
        FromIntoMemory::into_bytes(self.nFileSizeHigh, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.nFileSizeLow, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.dwReserved0, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.dwReserved1, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.cFileName, &mut into[44..44 + 260]);
        FromIntoMemory::into_bytes(self.cAlternateFileName, &mut into[304..304 + 14]);
    }
    fn size() -> usize {
        320
    }
}
pub struct WIN32_FIND_DATAW {
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::super::Foundation::FILETIME,
    pub ftLastAccessTime: super::super::Foundation::FILETIME,
    pub ftLastWriteTime: super::super::Foundation::FILETIME,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
    pub dwReserved0: u32,
    pub dwReserved1: u32,
    pub cFileName: [u16; 260],
    pub cAlternateFileName: [u16; 14],
}
impl ::core::marker::Copy for WIN32_FIND_DATAW {}
impl ::core::clone::Clone for WIN32_FIND_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIN32_FIND_DATAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN32_FIND_DATAW")
            .field("dwFileAttributes", &self.dwFileAttributes)
            .field("ftCreationTime", &self.ftCreationTime)
            .field("ftLastAccessTime", &self.ftLastAccessTime)
            .field("ftLastWriteTime", &self.ftLastWriteTime)
            .field("nFileSizeHigh", &self.nFileSizeHigh)
            .field("nFileSizeLow", &self.nFileSizeLow)
            .field("dwReserved0", &self.dwReserved0)
            .field("dwReserved1", &self.dwReserved1)
            .field("cFileName", &self.cFileName)
            .field("cAlternateFileName", &self.cAlternateFileName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WIN32_FIND_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.dwFileAttributes == other.dwFileAttributes
            && self.ftCreationTime == other.ftCreationTime
            && self.ftLastAccessTime == other.ftLastAccessTime
            && self.ftLastWriteTime == other.ftLastWriteTime
            && self.nFileSizeHigh == other.nFileSizeHigh
            && self.nFileSizeLow == other.nFileSizeLow
            && self.dwReserved0 == other.dwReserved0
            && self.dwReserved1 == other.dwReserved1
            && self.cFileName == other.cFileName
            && self.cAlternateFileName == other.cAlternateFileName
    }
}
impl ::core::cmp::Eq for WIN32_FIND_DATAW {}
impl FromIntoMemory for WIN32_FIND_DATAW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 320);
        let f_dwFileAttributes = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ftCreationTime =
            <super::super::Foundation::FILETIME as FromIntoMemory>::from_bytes(&from[4..4 + 8]);
        let f_ftLastAccessTime =
            <super::super::Foundation::FILETIME as FromIntoMemory>::from_bytes(&from[12..12 + 8]);
        let f_ftLastWriteTime =
            <super::super::Foundation::FILETIME as FromIntoMemory>::from_bytes(&from[20..20 + 8]);
        let f_nFileSizeHigh = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_nFileSizeLow = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_dwReserved0 = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_dwReserved1 = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_cFileName = <[u16; 260] as FromIntoMemory>::from_bytes(&from[44..44 + 260]);
        let f_cAlternateFileName = <[u16; 14] as FromIntoMemory>::from_bytes(&from[304..304 + 14]);
        Self {
            dwFileAttributes: f_dwFileAttributes,
            ftCreationTime: f_ftCreationTime,
            ftLastAccessTime: f_ftLastAccessTime,
            ftLastWriteTime: f_ftLastWriteTime,
            nFileSizeHigh: f_nFileSizeHigh,
            nFileSizeLow: f_nFileSizeLow,
            dwReserved0: f_dwReserved0,
            dwReserved1: f_dwReserved1,
            cFileName: f_cFileName,
            cAlternateFileName: f_cAlternateFileName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 320);
        FromIntoMemory::into_bytes(self.dwFileAttributes, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ftCreationTime, &mut into[4..4 + 8]);
        FromIntoMemory::into_bytes(self.ftLastAccessTime, &mut into[12..12 + 8]);
        FromIntoMemory::into_bytes(self.ftLastWriteTime, &mut into[20..20 + 8]);
        FromIntoMemory::into_bytes(self.nFileSizeHigh, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.nFileSizeLow, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.dwReserved0, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.dwReserved1, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.cFileName, &mut into[44..44 + 260]);
        FromIntoMemory::into_bytes(self.cAlternateFileName, &mut into[304..304 + 14]);
    }
    fn size() -> usize {
        320
    }
}
pub struct WIN32_FIND_STREAM_DATA {
    pub StreamSize: i64,
    pub cStreamName: [u16; 296],
}
impl ::core::marker::Copy for WIN32_FIND_STREAM_DATA {}
impl ::core::clone::Clone for WIN32_FIND_STREAM_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIN32_FIND_STREAM_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN32_FIND_STREAM_DATA")
            .field("StreamSize", &self.StreamSize)
            .field("cStreamName", &self.cStreamName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WIN32_FIND_STREAM_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.StreamSize == other.StreamSize && self.cStreamName == other.cStreamName
    }
}
impl ::core::cmp::Eq for WIN32_FIND_STREAM_DATA {}
impl FromIntoMemory for WIN32_FIND_STREAM_DATA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 304);
        let f_StreamSize = <i64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_cStreamName = <[u16; 296] as FromIntoMemory>::from_bytes(&from[8..8 + 296]);
        Self {
            StreamSize: f_StreamSize,
            cStreamName: f_cStreamName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 304);
        FromIntoMemory::into_bytes(self.StreamSize, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.cStreamName, &mut into[8..8 + 296]);
    }
    fn size() -> usize {
        304
    }
}
pub struct WIN32_STREAM_ID {
    pub dwStreamId: WIN_STREAM_ID,
    pub dwStreamAttributes: u32,
    pub Size: i64,
    pub dwStreamNameSize: u32,
    pub cStreamName: [u16; 1],
}
impl ::core::marker::Copy for WIN32_STREAM_ID {}
impl ::core::clone::Clone for WIN32_STREAM_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIN32_STREAM_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN32_STREAM_ID")
            .field("dwStreamId", &self.dwStreamId)
            .field("dwStreamAttributes", &self.dwStreamAttributes)
            .field("Size", &self.Size)
            .field("dwStreamNameSize", &self.dwStreamNameSize)
            .field("cStreamName", &self.cStreamName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WIN32_STREAM_ID {
    fn eq(&self, other: &Self) -> bool {
        self.dwStreamId == other.dwStreamId
            && self.dwStreamAttributes == other.dwStreamAttributes
            && self.Size == other.Size
            && self.dwStreamNameSize == other.dwStreamNameSize
            && self.cStreamName == other.cStreamName
    }
}
impl ::core::cmp::Eq for WIN32_STREAM_ID {}
impl FromIntoMemory for WIN32_STREAM_ID {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_dwStreamId = <WIN_STREAM_ID as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwStreamAttributes = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Size = <i64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_dwStreamNameSize = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_cStreamName = <[u16; 1] as FromIntoMemory>::from_bytes(&from[20..20 + 1]);
        Self {
            dwStreamId: f_dwStreamId,
            dwStreamAttributes: f_dwStreamAttributes,
            Size: f_Size,
            dwStreamNameSize: f_dwStreamNameSize,
            cStreamName: f_cStreamName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.dwStreamId, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwStreamAttributes, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Size, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.dwStreamNameSize, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.cStreamName, &mut into[20..20 + 1]);
    }
    fn size() -> usize {
        24
    }
}
pub const WINEFS_SETUSERKEY_SET_CAPABILITIES: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WIN_STREAM_ID(pub u32);
pub const BACKUP_ALTERNATE_DATA: WIN_STREAM_ID = WIN_STREAM_ID(4u32);
pub const BACKUP_DATA: WIN_STREAM_ID = WIN_STREAM_ID(1u32);
pub const BACKUP_EA_DATA: WIN_STREAM_ID = WIN_STREAM_ID(2u32);
pub const BACKUP_LINK: WIN_STREAM_ID = WIN_STREAM_ID(5u32);
pub const BACKUP_OBJECT_ID: WIN_STREAM_ID = WIN_STREAM_ID(7u32);
pub const BACKUP_PROPERTY_DATA: WIN_STREAM_ID = WIN_STREAM_ID(6u32);
pub const BACKUP_REPARSE_DATA: WIN_STREAM_ID = WIN_STREAM_ID(8u32);
pub const BACKUP_SECURITY_DATA: WIN_STREAM_ID = WIN_STREAM_ID(3u32);
pub const BACKUP_SPARSE_BLOCK: WIN_STREAM_ID = WIN_STREAM_ID(9u32);
pub const BACKUP_TXFS_DATA: WIN_STREAM_ID = WIN_STREAM_ID(10u32);
impl ::core::marker::Copy for WIN_STREAM_ID {}
impl ::core::clone::Clone for WIN_STREAM_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WIN_STREAM_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WIN_STREAM_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WIN_STREAM_ID").field(&self.0).finish()
    }
}
impl FromIntoMemory for WIN_STREAM_ID {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct WOF_FILE_COMPRESSION_INFO_V0 {
    pub Algorithm: u32,
}
impl ::core::marker::Copy for WOF_FILE_COMPRESSION_INFO_V0 {}
impl ::core::clone::Clone for WOF_FILE_COMPRESSION_INFO_V0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WOF_FILE_COMPRESSION_INFO_V0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WOF_FILE_COMPRESSION_INFO_V0")
            .field("Algorithm", &self.Algorithm)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WOF_FILE_COMPRESSION_INFO_V0 {
    fn eq(&self, other: &Self) -> bool {
        self.Algorithm == other.Algorithm
    }
}
impl ::core::cmp::Eq for WOF_FILE_COMPRESSION_INFO_V0 {}
impl FromIntoMemory for WOF_FILE_COMPRESSION_INFO_V0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_Algorithm = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            Algorithm: f_Algorithm,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.Algorithm, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct WOF_FILE_COMPRESSION_INFO_V1 {
    pub Algorithm: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for WOF_FILE_COMPRESSION_INFO_V1 {}
impl ::core::clone::Clone for WOF_FILE_COMPRESSION_INFO_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WOF_FILE_COMPRESSION_INFO_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WOF_FILE_COMPRESSION_INFO_V1")
            .field("Algorithm", &self.Algorithm)
            .field("Flags", &self.Flags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WOF_FILE_COMPRESSION_INFO_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Algorithm == other.Algorithm && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for WOF_FILE_COMPRESSION_INFO_V1 {}
impl FromIntoMemory for WOF_FILE_COMPRESSION_INFO_V1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Algorithm = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Flags = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            Algorithm: f_Algorithm,
            Flags: f_Flags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Algorithm, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub const WOF_PROVIDER_FILE: u32 = 2u32;
pub const WOF_PROVIDER_WIM: u32 = 1u32;
pub type WofEnumEntryProc = StdCallFnPtr<
    (ConstPtr<::core::ffi::c_void>, ConstPtr<::core::ffi::c_void>),
    super::super::Foundation::BOOL,
>;
pub type WofEnumFilesProc = StdCallFnPtr<
    (
        PCWSTR,
        ConstPtr<::core::ffi::c_void>,
        ConstPtr<::core::ffi::c_void>,
    ),
    super::super::Foundation::BOOL,
>;
pub const _FT_TYPES_DEFINITION_: u32 = 1u32;
pub trait Api {
    #[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn AddUsersToEncryptedFile(
        &self,
        lp_file_name: PCWSTR,
        p_encryption_certificates: ConstPtr<ENCRYPTION_CERTIFICATE_LIST>,
    ) -> u32 {
        todo!("AddUsersToEncryptedFile")
    }
    fn AreFileApisANSI(&self) -> super::super::Foundation::BOOL {
        todo!("AreFileApisANSI")
    }
    fn AreShortNamesEnabled(
        &self,
        handle: super::super::Foundation::HANDLE,
        enabled: MutPtr<super::super::Foundation::BOOL>,
    ) -> super::super::Foundation::BOOL {
        todo!("AreShortNamesEnabled")
    }
    fn BackupRead(
        &self,
        h_file: super::super::Foundation::HANDLE,
        lp_buffer: MutPtr<u8>,
        n_number_of_bytes_to_read: u32,
        lp_number_of_bytes_read: MutPtr<u32>,
        b_abort: super::super::Foundation::BOOL,
        b_process_security: super::super::Foundation::BOOL,
        lp_context: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> super::super::Foundation::BOOL {
        todo!("BackupRead")
    }
    fn BackupSeek(
        &self,
        h_file: super::super::Foundation::HANDLE,
        dw_low_bytes_to_seek: u32,
        dw_high_bytes_to_seek: u32,
        lpdw_low_byte_seeked: MutPtr<u32>,
        lpdw_high_byte_seeked: MutPtr<u32>,
        lp_context: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> super::super::Foundation::BOOL {
        todo!("BackupSeek")
    }
    fn BackupWrite(
        &self,
        h_file: super::super::Foundation::HANDLE,
        lp_buffer: ConstPtr<u8>,
        n_number_of_bytes_to_write: u32,
        lp_number_of_bytes_written: MutPtr<u32>,
        b_abort: super::super::Foundation::BOOL,
        b_process_security: super::super::Foundation::BOOL,
        lp_context: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> super::super::Foundation::BOOL {
        todo!("BackupWrite")
    }
    fn BuildIoRingRegisterBuffers(
        &self,
        io_ring: ConstPtr<HIORING__>,
        count: u32,
        buffers: ConstPtr<IORING_BUFFER_INFO>,
        user_data: PtrRepr,
    ) -> crate::core::HRESULT {
        todo!("BuildIoRingRegisterBuffers")
    }
    fn BuildIoRingRegisterFileHandles(
        &self,
        io_ring: ConstPtr<HIORING__>,
        count: u32,
        handles: ConstPtr<super::super::Foundation::HANDLE>,
        user_data: PtrRepr,
    ) -> crate::core::HRESULT {
        todo!("BuildIoRingRegisterFileHandles")
    }
    fn CheckNameLegalDOS8Dot3A(
        &self,
        lp_name: PCSTR,
        lp_oem_name: PSTR,
        oem_name_size: u32,
        pb_name_contains_spaces: MutPtr<super::super::Foundation::BOOL>,
        pb_name_legal: MutPtr<super::super::Foundation::BOOL>,
    ) -> super::super::Foundation::BOOL {
        todo!("CheckNameLegalDOS8Dot3A")
    }
    fn CheckNameLegalDOS8Dot3W(
        &self,
        lp_name: PCWSTR,
        lp_oem_name: PSTR,
        oem_name_size: u32,
        pb_name_contains_spaces: MutPtr<super::super::Foundation::BOOL>,
        pb_name_legal: MutPtr<super::super::Foundation::BOOL>,
    ) -> super::super::Foundation::BOOL {
        todo!("CheckNameLegalDOS8Dot3W")
    }
    fn CloseEncryptedFileRaw(&self, pv_context: ConstPtr<::core::ffi::c_void>) {
        todo!("CloseEncryptedFileRaw")
    }
    fn CloseIoRing(&self, io_ring: ConstPtr<HIORING__>) -> crate::core::HRESULT {
        todo!("CloseIoRing")
    }
    fn CommitComplete(
        &self,
        enlistment_handle: super::super::Foundation::HANDLE,
        tm_virtual_clock: MutPtr<i64>,
    ) -> super::super::Foundation::BOOL {
        todo!("CommitComplete")
    }
    fn CommitEnlistment(
        &self,
        enlistment_handle: super::super::Foundation::HANDLE,
        tm_virtual_clock: MutPtr<i64>,
    ) -> super::super::Foundation::BOOL {
        todo!("CommitEnlistment")
    }
    fn CommitTransaction(
        &self,
        transaction_handle: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("CommitTransaction")
    }
    fn CommitTransactionAsync(
        &self,
        transaction_handle: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("CommitTransactionAsync")
    }
    fn CompareFileTime(
        &self,
        lp_file_time_1: ConstPtr<super::super::Foundation::FILETIME>,
        lp_file_time_2: ConstPtr<super::super::Foundation::FILETIME>,
    ) -> i32 {
        todo!("CompareFileTime")
    }
    fn CopyFile2(
        &self,
        pwsz_existing_file_name: PCWSTR,
        pwsz_new_file_name: PCWSTR,
        p_extended_parameters: ConstPtr<COPYFILE2_EXTENDED_PARAMETERS>,
    ) -> crate::core::HRESULT {
        todo!("CopyFile2")
    }
    fn CopyFileA(
        &self,
        lp_existing_file_name: PCSTR,
        lp_new_file_name: PCSTR,
        b_fail_if_exists: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!("CopyFileA")
    }
    fn CopyFileExA(
        &self,
        lp_existing_file_name: PCSTR,
        lp_new_file_name: PCSTR,
        lp_progress_routine: LPPROGRESS_ROUTINE,
        lp_data: ConstPtr<::core::ffi::c_void>,
        pb_cancel: MutPtr<i32>,
        dw_copy_flags: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("CopyFileExA")
    }
    fn CopyFileExW(
        &self,
        lp_existing_file_name: PCWSTR,
        lp_new_file_name: PCWSTR,
        lp_progress_routine: LPPROGRESS_ROUTINE,
        lp_data: ConstPtr<::core::ffi::c_void>,
        pb_cancel: MutPtr<i32>,
        dw_copy_flags: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("CopyFileExW")
    }
    fn CopyFileFromAppW(
        &self,
        lp_existing_file_name: PCWSTR,
        lp_new_file_name: PCWSTR,
        b_fail_if_exists: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!("CopyFileFromAppW")
    }
    fn CopyFileTransactedA(
        &self,
        lp_existing_file_name: PCSTR,
        lp_new_file_name: PCSTR,
        lp_progress_routine: LPPROGRESS_ROUTINE,
        lp_data: ConstPtr<::core::ffi::c_void>,
        pb_cancel: ConstPtr<i32>,
        dw_copy_flags: u32,
        h_transaction: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("CopyFileTransactedA")
    }
    fn CopyFileTransactedW(
        &self,
        lp_existing_file_name: PCWSTR,
        lp_new_file_name: PCWSTR,
        lp_progress_routine: LPPROGRESS_ROUTINE,
        lp_data: ConstPtr<::core::ffi::c_void>,
        pb_cancel: ConstPtr<i32>,
        dw_copy_flags: u32,
        h_transaction: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("CopyFileTransactedW")
    }
    fn CopyFileW(
        &self,
        lp_existing_file_name: PCWSTR,
        lp_new_file_name: PCWSTR,
        b_fail_if_exists: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!("CopyFileW")
    }
    fn CopyLZFile(&self, hf_source: i32, hf_dest: i32) -> i32 {
        todo!("CopyLZFile")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateDirectoryA(
        &self,
        lp_path_name: PCSTR,
        lp_security_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
    ) -> super::super::Foundation::BOOL {
        todo!("CreateDirectoryA")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateDirectoryExA(
        &self,
        lp_template_directory: PCSTR,
        lp_new_directory: PCSTR,
        lp_security_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
    ) -> super::super::Foundation::BOOL {
        todo!("CreateDirectoryExA")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateDirectoryExW(
        &self,
        lp_template_directory: PCWSTR,
        lp_new_directory: PCWSTR,
        lp_security_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
    ) -> super::super::Foundation::BOOL {
        todo!("CreateDirectoryExW")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateDirectoryFromAppW(
        &self,
        lp_path_name: PCWSTR,
        lp_security_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
    ) -> super::super::Foundation::BOOL {
        todo!("CreateDirectoryFromAppW")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateDirectoryTransactedA(
        &self,
        lp_template_directory: PCSTR,
        lp_new_directory: PCSTR,
        lp_security_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        h_transaction: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("CreateDirectoryTransactedA")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateDirectoryTransactedW(
        &self,
        lp_template_directory: PCWSTR,
        lp_new_directory: PCWSTR,
        lp_security_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        h_transaction: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("CreateDirectoryTransactedW")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateDirectoryW(
        &self,
        lp_path_name: PCWSTR,
        lp_security_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
    ) -> super::super::Foundation::BOOL {
        todo!("CreateDirectoryW")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateEnlistment(
        &self,
        lp_enlistment_attributes: MutPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        resource_manager_handle: super::super::Foundation::HANDLE,
        transaction_handle: super::super::Foundation::HANDLE,
        notification_mask: u32,
        create_options: u32,
        enlistment_key: MutPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateEnlistment")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateFile2(
        &self,
        lp_file_name: PCWSTR,
        dw_desired_access: FILE_ACCESS_FLAGS,
        dw_share_mode: FILE_SHARE_MODE,
        dw_creation_disposition: FILE_CREATION_DISPOSITION,
        p_create_ex_params: ConstPtr<CREATEFILE2_EXTENDED_PARAMETERS>,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateFile2")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateFile2FromAppW(
        &self,
        lp_file_name: PCWSTR,
        dw_desired_access: u32,
        dw_share_mode: u32,
        dw_creation_disposition: u32,
        p_create_ex_params: ConstPtr<CREATEFILE2_EXTENDED_PARAMETERS>,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateFile2FromAppW")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateFileA(
        &self,
        lp_file_name: PCSTR,
        dw_desired_access: FILE_ACCESS_FLAGS,
        dw_share_mode: FILE_SHARE_MODE,
        lp_security_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        dw_creation_disposition: FILE_CREATION_DISPOSITION,
        dw_flags_and_attributes: FILE_FLAGS_AND_ATTRIBUTES,
        h_template_file: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateFileA")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateFileFromAppW(
        &self,
        lp_file_name: PCWSTR,
        dw_desired_access: u32,
        dw_share_mode: u32,
        lp_security_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        dw_creation_disposition: u32,
        dw_flags_and_attributes: u32,
        h_template_file: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateFileFromAppW")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateFileTransactedA(
        &self,
        lp_file_name: PCSTR,
        dw_desired_access: u32,
        dw_share_mode: FILE_SHARE_MODE,
        lp_security_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        dw_creation_disposition: FILE_CREATION_DISPOSITION,
        dw_flags_and_attributes: FILE_FLAGS_AND_ATTRIBUTES,
        h_template_file: super::super::Foundation::HANDLE,
        h_transaction: super::super::Foundation::HANDLE,
        pus_mini_version: ConstPtr<TXFS_MINIVERSION>,
        lp_extended_parameter: MutPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateFileTransactedA")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateFileTransactedW(
        &self,
        lp_file_name: PCWSTR,
        dw_desired_access: u32,
        dw_share_mode: FILE_SHARE_MODE,
        lp_security_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        dw_creation_disposition: FILE_CREATION_DISPOSITION,
        dw_flags_and_attributes: FILE_FLAGS_AND_ATTRIBUTES,
        h_template_file: super::super::Foundation::HANDLE,
        h_transaction: super::super::Foundation::HANDLE,
        pus_mini_version: ConstPtr<TXFS_MINIVERSION>,
        lp_extended_parameter: MutPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateFileTransactedW")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateFileW(
        &self,
        lp_file_name: PCWSTR,
        dw_desired_access: FILE_ACCESS_FLAGS,
        dw_share_mode: FILE_SHARE_MODE,
        lp_security_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        dw_creation_disposition: FILE_CREATION_DISPOSITION,
        dw_flags_and_attributes: FILE_FLAGS_AND_ATTRIBUTES,
        h_template_file: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateFileW")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateHardLinkA(
        &self,
        lp_file_name: PCSTR,
        lp_existing_file_name: PCSTR,
        lp_security_attributes: MutPtr<super::super::Security::SECURITY_ATTRIBUTES>,
    ) -> super::super::Foundation::BOOL {
        todo!("CreateHardLinkA")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateHardLinkTransactedA(
        &self,
        lp_file_name: PCSTR,
        lp_existing_file_name: PCSTR,
        lp_security_attributes: MutPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        h_transaction: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("CreateHardLinkTransactedA")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateHardLinkTransactedW(
        &self,
        lp_file_name: PCWSTR,
        lp_existing_file_name: PCWSTR,
        lp_security_attributes: MutPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        h_transaction: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("CreateHardLinkTransactedW")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateHardLinkW(
        &self,
        lp_file_name: PCWSTR,
        lp_existing_file_name: PCWSTR,
        lp_security_attributes: MutPtr<super::super::Security::SECURITY_ATTRIBUTES>,
    ) -> super::super::Foundation::BOOL {
        todo!("CreateHardLinkW")
    }
    fn CreateIoRing(
        &self,
        ioring_version: IORING_VERSION,
        flags: IORING_CREATE_FLAGS,
        submission_queue_size: u32,
        completion_queue_size: u32,
        h: MutPtr<ConstPtr<HIORING__>>,
    ) -> crate::core::HRESULT {
        todo!("CreateIoRing")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateResourceManager(
        &self,
        lp_resource_manager_attributes: MutPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        resource_manager_id: MutPtr<crate::core::GUID>,
        create_options: u32,
        tm_handle: super::super::Foundation::HANDLE,
        description: PCWSTR,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateResourceManager")
    }
    fn CreateSymbolicLinkA(
        &self,
        lp_symlink_file_name: PCSTR,
        lp_target_file_name: PCSTR,
        dw_flags: SYMBOLIC_LINK_FLAGS,
    ) -> super::super::Foundation::BOOLEAN {
        todo!("CreateSymbolicLinkA")
    }
    fn CreateSymbolicLinkTransactedA(
        &self,
        lp_symlink_file_name: PCSTR,
        lp_target_file_name: PCSTR,
        dw_flags: SYMBOLIC_LINK_FLAGS,
        h_transaction: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOLEAN {
        todo!("CreateSymbolicLinkTransactedA")
    }
    fn CreateSymbolicLinkTransactedW(
        &self,
        lp_symlink_file_name: PCWSTR,
        lp_target_file_name: PCWSTR,
        dw_flags: SYMBOLIC_LINK_FLAGS,
        h_transaction: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOLEAN {
        todo!("CreateSymbolicLinkTransactedW")
    }
    fn CreateSymbolicLinkW(
        &self,
        lp_symlink_file_name: PCWSTR,
        lp_target_file_name: PCWSTR,
        dw_flags: SYMBOLIC_LINK_FLAGS,
    ) -> super::super::Foundation::BOOLEAN {
        todo!("CreateSymbolicLinkW")
    }
    fn CreateTapePartition(
        &self,
        h_device: super::super::Foundation::HANDLE,
        dw_partition_method: CREATE_TAPE_PARTITION_METHOD,
        dw_count: u32,
        dw_size: u32,
    ) -> u32 {
        todo!("CreateTapePartition")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateTransaction(
        &self,
        lp_transaction_attributes: MutPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        uow: MutPtr<crate::core::GUID>,
        create_options: u32,
        isolation_level: u32,
        isolation_flags: u32,
        timeout: u32,
        description: PCWSTR,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateTransaction")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateTransactionManager(
        &self,
        lp_transaction_attributes: MutPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        log_file_name: PCWSTR,
        create_options: u32,
        commit_strength: u32,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateTransactionManager")
    }
    fn DecryptFileA(
        &self,
        lp_file_name: PCSTR,
        dw_reserved: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("DecryptFileA")
    }
    fn DecryptFileW(
        &self,
        lp_file_name: PCWSTR,
        dw_reserved: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("DecryptFileW")
    }
    fn DefineDosDeviceA(
        &self,
        dw_flags: DEFINE_DOS_DEVICE_FLAGS,
        lp_device_name: PCSTR,
        lp_target_path: PCSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("DefineDosDeviceA")
    }
    fn DefineDosDeviceW(
        &self,
        dw_flags: DEFINE_DOS_DEVICE_FLAGS,
        lp_device_name: PCWSTR,
        lp_target_path: PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("DefineDosDeviceW")
    }
    fn DeleteFileA(&self, lp_file_name: PCSTR) -> super::super::Foundation::BOOL {
        todo!("DeleteFileA")
    }
    fn DeleteFileFromAppW(&self, lp_file_name: PCWSTR) -> super::super::Foundation::BOOL {
        todo!("DeleteFileFromAppW")
    }
    fn DeleteFileTransactedA(
        &self,
        lp_file_name: PCSTR,
        h_transaction: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("DeleteFileTransactedA")
    }
    fn DeleteFileTransactedW(
        &self,
        lp_file_name: PCWSTR,
        h_transaction: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("DeleteFileTransactedW")
    }
    fn DeleteFileW(&self, lp_file_name: PCWSTR) -> super::super::Foundation::BOOL {
        todo!("DeleteFileW")
    }
    fn DeleteVolumeMountPointA(
        &self,
        lpsz_volume_mount_point: PCSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("DeleteVolumeMountPointA")
    }
    fn DeleteVolumeMountPointW(
        &self,
        lpsz_volume_mount_point: PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("DeleteVolumeMountPointW")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn DuplicateEncryptionInfoFile(
        &self,
        src_file_name: PCWSTR,
        dst_file_name: PCWSTR,
        dw_creation_distribution: u32,
        dw_attributes: u32,
        lp_security_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
    ) -> u32 {
        todo!("DuplicateEncryptionInfoFile")
    }
    fn EncryptFileA(&self, lp_file_name: PCSTR) -> super::super::Foundation::BOOL {
        todo!("EncryptFileA")
    }
    fn EncryptFileW(&self, lp_file_name: PCWSTR) -> super::super::Foundation::BOOL {
        todo!("EncryptFileW")
    }
    fn EncryptionDisable(
        &self,
        dir_path: PCWSTR,
        disable: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!("EncryptionDisable")
    }
    fn EraseTape(
        &self,
        h_device: super::super::Foundation::HANDLE,
        dw_erase_type: ERASE_TAPE_TYPE,
        b_immediate: super::super::Foundation::BOOL,
    ) -> u32 {
        todo!("EraseTape")
    }
    fn FileEncryptionStatusA(
        &self,
        lp_file_name: PCSTR,
        lp_status: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("FileEncryptionStatusA")
    }
    fn FileEncryptionStatusW(
        &self,
        lp_file_name: PCWSTR,
        lp_status: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("FileEncryptionStatusW")
    }
    fn FileTimeToLocalFileTime(
        &self,
        lp_file_time: ConstPtr<super::super::Foundation::FILETIME>,
        lp_local_file_time: MutPtr<super::super::Foundation::FILETIME>,
    ) -> super::super::Foundation::BOOL {
        todo!("FileTimeToLocalFileTime")
    }
    fn FindClose(&self, h_find_file: FindFileHandle) -> super::super::Foundation::BOOL {
        todo!("FindClose")
    }
    fn FindCloseChangeNotification(
        &self,
        h_change_handle: FindChangeNotificationHandle,
    ) -> super::super::Foundation::BOOL {
        todo!("FindCloseChangeNotification")
    }
    fn FindFirstChangeNotificationA(
        &self,
        lp_path_name: PCSTR,
        b_watch_subtree: super::super::Foundation::BOOL,
        dw_notify_filter: FILE_NOTIFY_CHANGE,
    ) -> FindChangeNotificationHandle {
        todo!("FindFirstChangeNotificationA")
    }
    fn FindFirstChangeNotificationW(
        &self,
        lp_path_name: PCWSTR,
        b_watch_subtree: super::super::Foundation::BOOL,
        dw_notify_filter: FILE_NOTIFY_CHANGE,
    ) -> FindChangeNotificationHandle {
        todo!("FindFirstChangeNotificationW")
    }
    fn FindFirstFileA(
        &self,
        lp_file_name: PCSTR,
        lp_find_file_data: MutPtr<WIN32_FIND_DATAA>,
    ) -> FindFileHandle {
        todo!("FindFirstFileA")
    }
    fn FindFirstFileExA(
        &self,
        lp_file_name: PCSTR,
        f_info_level_id: FINDEX_INFO_LEVELS,
        lp_find_file_data: MutPtr<::core::ffi::c_void>,
        f_search_op: FINDEX_SEARCH_OPS,
        lp_search_filter: MutPtr<::core::ffi::c_void>,
        dw_additional_flags: FIND_FIRST_EX_FLAGS,
    ) -> FindFileHandle {
        todo!("FindFirstFileExA")
    }
    fn FindFirstFileExFromAppW(
        &self,
        lp_file_name: PCWSTR,
        f_info_level_id: FINDEX_INFO_LEVELS,
        lp_find_file_data: MutPtr<::core::ffi::c_void>,
        f_search_op: FINDEX_SEARCH_OPS,
        lp_search_filter: MutPtr<::core::ffi::c_void>,
        dw_additional_flags: u32,
    ) -> super::super::Foundation::HANDLE {
        todo!("FindFirstFileExFromAppW")
    }
    fn FindFirstFileExW(
        &self,
        lp_file_name: PCWSTR,
        f_info_level_id: FINDEX_INFO_LEVELS,
        lp_find_file_data: MutPtr<::core::ffi::c_void>,
        f_search_op: FINDEX_SEARCH_OPS,
        lp_search_filter: MutPtr<::core::ffi::c_void>,
        dw_additional_flags: FIND_FIRST_EX_FLAGS,
    ) -> FindFileHandle {
        todo!("FindFirstFileExW")
    }
    fn FindFirstFileNameTransactedW(
        &self,
        lp_file_name: PCWSTR,
        dw_flags: u32,
        string_length: MutPtr<u32>,
        link_name: PWSTR,
        h_transaction: super::super::Foundation::HANDLE,
    ) -> FindFileNameHandle {
        todo!("FindFirstFileNameTransactedW")
    }
    fn FindFirstFileNameW(
        &self,
        lp_file_name: PCWSTR,
        dw_flags: u32,
        string_length: MutPtr<u32>,
        link_name: PWSTR,
    ) -> FindFileNameHandle {
        todo!("FindFirstFileNameW")
    }
    fn FindFirstFileTransactedA(
        &self,
        lp_file_name: PCSTR,
        f_info_level_id: FINDEX_INFO_LEVELS,
        lp_find_file_data: MutPtr<::core::ffi::c_void>,
        f_search_op: FINDEX_SEARCH_OPS,
        lp_search_filter: MutPtr<::core::ffi::c_void>,
        dw_additional_flags: u32,
        h_transaction: super::super::Foundation::HANDLE,
    ) -> FindFileHandle {
        todo!("FindFirstFileTransactedA")
    }
    fn FindFirstFileTransactedW(
        &self,
        lp_file_name: PCWSTR,
        f_info_level_id: FINDEX_INFO_LEVELS,
        lp_find_file_data: MutPtr<::core::ffi::c_void>,
        f_search_op: FINDEX_SEARCH_OPS,
        lp_search_filter: MutPtr<::core::ffi::c_void>,
        dw_additional_flags: u32,
        h_transaction: super::super::Foundation::HANDLE,
    ) -> FindFileHandle {
        todo!("FindFirstFileTransactedW")
    }
    fn FindFirstFileW(
        &self,
        lp_file_name: PCWSTR,
        lp_find_file_data: MutPtr<WIN32_FIND_DATAW>,
    ) -> FindFileHandle {
        todo!("FindFirstFileW")
    }
    fn FindFirstStreamTransactedW(
        &self,
        lp_file_name: PCWSTR,
        info_level: STREAM_INFO_LEVELS,
        lp_find_stream_data: MutPtr<::core::ffi::c_void>,
        dw_flags: u32,
        h_transaction: super::super::Foundation::HANDLE,
    ) -> FindStreamHandle {
        todo!("FindFirstStreamTransactedW")
    }
    fn FindFirstStreamW(
        &self,
        lp_file_name: PCWSTR,
        info_level: STREAM_INFO_LEVELS,
        lp_find_stream_data: MutPtr<::core::ffi::c_void>,
        dw_flags: u32,
    ) -> FindStreamHandle {
        todo!("FindFirstStreamW")
    }
    fn FindFirstVolumeA(&self, lpsz_volume_name: PSTR, cch_buffer_length: u32) -> FindVolumeHandle {
        todo!("FindFirstVolumeA")
    }
    fn FindFirstVolumeMountPointA(
        &self,
        lpsz_root_path_name: PCSTR,
        lpsz_volume_mount_point: PSTR,
        cch_buffer_length: u32,
    ) -> FindVolumeMointPointHandle {
        todo!("FindFirstVolumeMountPointA")
    }
    fn FindFirstVolumeMountPointW(
        &self,
        lpsz_root_path_name: PCWSTR,
        lpsz_volume_mount_point: PWSTR,
        cch_buffer_length: u32,
    ) -> FindVolumeMointPointHandle {
        todo!("FindFirstVolumeMountPointW")
    }
    fn FindFirstVolumeW(
        &self,
        lpsz_volume_name: PWSTR,
        cch_buffer_length: u32,
    ) -> FindVolumeHandle {
        todo!("FindFirstVolumeW")
    }
    fn FindNextChangeNotification(
        &self,
        h_change_handle: FindChangeNotificationHandle,
    ) -> super::super::Foundation::BOOL {
        todo!("FindNextChangeNotification")
    }
    fn FindNextFileA(
        &self,
        h_find_file: FindFileHandle,
        lp_find_file_data: MutPtr<WIN32_FIND_DATAA>,
    ) -> super::super::Foundation::BOOL {
        todo!("FindNextFileA")
    }
    fn FindNextFileNameW(
        &self,
        h_find_stream: FindFileNameHandle,
        string_length: MutPtr<u32>,
        link_name: PWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("FindNextFileNameW")
    }
    fn FindNextFileW(
        &self,
        h_find_file: super::super::Foundation::HANDLE,
        lp_find_file_data: MutPtr<WIN32_FIND_DATAW>,
    ) -> super::super::Foundation::BOOL {
        todo!("FindNextFileW")
    }
    fn FindNextStreamW(
        &self,
        h_find_stream: FindStreamHandle,
        lp_find_stream_data: MutPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!("FindNextStreamW")
    }
    fn FindNextVolumeA(
        &self,
        h_find_volume: FindVolumeHandle,
        lpsz_volume_name: PSTR,
        cch_buffer_length: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("FindNextVolumeA")
    }
    fn FindNextVolumeMountPointA(
        &self,
        h_find_volume_mount_point: FindVolumeMointPointHandle,
        lpsz_volume_mount_point: PSTR,
        cch_buffer_length: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("FindNextVolumeMountPointA")
    }
    fn FindNextVolumeMountPointW(
        &self,
        h_find_volume_mount_point: FindVolumeMointPointHandle,
        lpsz_volume_mount_point: PWSTR,
        cch_buffer_length: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("FindNextVolumeMountPointW")
    }
    fn FindNextVolumeW(
        &self,
        h_find_volume: FindVolumeHandle,
        lpsz_volume_name: PWSTR,
        cch_buffer_length: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("FindNextVolumeW")
    }
    fn FindVolumeClose(&self, h_find_volume: FindVolumeHandle) -> super::super::Foundation::BOOL {
        todo!("FindVolumeClose")
    }
    fn FindVolumeMountPointClose(
        &self,
        h_find_volume_mount_point: FindVolumeMointPointHandle,
    ) -> super::super::Foundation::BOOL {
        todo!("FindVolumeMountPointClose")
    }
    fn FlushFileBuffers(
        &self,
        h_file: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("FlushFileBuffers")
    }
    fn FreeEncryptedFileMetadata(&self, pb_metadata: ConstPtr<u8>) {
        todo!("FreeEncryptedFileMetadata")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn FreeEncryptionCertificateHashList(
        &self,
        p_users: ConstPtr<ENCRYPTION_CERTIFICATE_HASH_LIST>,
    ) {
        todo!("FreeEncryptionCertificateHashList")
    }
    fn GetBinaryTypeA(
        &self,
        lp_application_name: PCSTR,
        lp_binary_type: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetBinaryTypeA")
    }
    fn GetBinaryTypeW(
        &self,
        lp_application_name: PCWSTR,
        lp_binary_type: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetBinaryTypeW")
    }
    fn GetCompressedFileSizeA(&self, lp_file_name: PCSTR, lp_file_size_high: MutPtr<u32>) -> u32 {
        todo!("GetCompressedFileSizeA")
    }
    fn GetCompressedFileSizeTransactedA(
        &self,
        lp_file_name: PCSTR,
        lp_file_size_high: MutPtr<u32>,
        h_transaction: super::super::Foundation::HANDLE,
    ) -> u32 {
        todo!("GetCompressedFileSizeTransactedA")
    }
    fn GetCompressedFileSizeTransactedW(
        &self,
        lp_file_name: PCWSTR,
        lp_file_size_high: MutPtr<u32>,
        h_transaction: super::super::Foundation::HANDLE,
    ) -> u32 {
        todo!("GetCompressedFileSizeTransactedW")
    }
    fn GetCompressedFileSizeW(&self, lp_file_name: PCWSTR, lp_file_size_high: MutPtr<u32>) -> u32 {
        todo!("GetCompressedFileSizeW")
    }
    fn GetCurrentClockTransactionManager(
        &self,
        transaction_manager_handle: super::super::Foundation::HANDLE,
        tm_virtual_clock: MutPtr<i64>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetCurrentClockTransactionManager")
    }
    fn GetDiskFreeSpaceA(
        &self,
        lp_root_path_name: PCSTR,
        lp_sectors_per_cluster: MutPtr<u32>,
        lp_bytes_per_sector: MutPtr<u32>,
        lp_number_of_free_clusters: MutPtr<u32>,
        lp_total_number_of_clusters: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetDiskFreeSpaceA")
    }
    fn GetDiskFreeSpaceExA(
        &self,
        lp_directory_name: PCSTR,
        lp_free_bytes_available_to_caller: MutPtr<u64>,
        lp_total_number_of_bytes: MutPtr<u64>,
        lp_total_number_of_free_bytes: MutPtr<u64>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetDiskFreeSpaceExA")
    }
    fn GetDiskFreeSpaceExW(
        &self,
        lp_directory_name: PCWSTR,
        lp_free_bytes_available_to_caller: MutPtr<u64>,
        lp_total_number_of_bytes: MutPtr<u64>,
        lp_total_number_of_free_bytes: MutPtr<u64>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetDiskFreeSpaceExW")
    }
    fn GetDiskFreeSpaceW(
        &self,
        lp_root_path_name: PCWSTR,
        lp_sectors_per_cluster: MutPtr<u32>,
        lp_bytes_per_sector: MutPtr<u32>,
        lp_number_of_free_clusters: MutPtr<u32>,
        lp_total_number_of_clusters: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetDiskFreeSpaceW")
    }
    fn GetDiskSpaceInformationA(
        &self,
        root_path: PCSTR,
        disk_space_info: MutPtr<DISK_SPACE_INFORMATION>,
    ) -> crate::core::HRESULT {
        todo!("GetDiskSpaceInformationA")
    }
    fn GetDiskSpaceInformationW(
        &self,
        root_path: PCWSTR,
        disk_space_info: MutPtr<DISK_SPACE_INFORMATION>,
    ) -> crate::core::HRESULT {
        todo!("GetDiskSpaceInformationW")
    }
    fn GetDriveTypeA(&self, lp_root_path_name: PCSTR) -> u32 {
        todo!("GetDriveTypeA")
    }
    fn GetDriveTypeW(&self, lp_root_path_name: PCWSTR) -> u32 {
        todo!("GetDriveTypeW")
    }
    fn GetEncryptedFileMetadata(
        &self,
        lp_file_name: PCWSTR,
        pcb_metadata: MutPtr<u32>,
        ppb_metadata: MutPtr<ConstPtr<u8>>,
    ) -> u32 {
        todo!("GetEncryptedFileMetadata")
    }
    fn GetEnlistmentId(
        &self,
        enlistment_handle: super::super::Foundation::HANDLE,
        enlistment_id: MutPtr<crate::core::GUID>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetEnlistmentId")
    }
    fn GetEnlistmentRecoveryInformation(
        &self,
        enlistment_handle: super::super::Foundation::HANDLE,
        buffer_size: u32,
        buffer: MutPtr<::core::ffi::c_void>,
        buffer_used: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetEnlistmentRecoveryInformation")
    }
    fn GetExpandedNameA(&self, lpsz_source: PCSTR, lpsz_buffer: PSTR) -> i32 {
        todo!("GetExpandedNameA")
    }
    fn GetExpandedNameW(&self, lpsz_source: PCWSTR, lpsz_buffer: PWSTR) -> i32 {
        todo!("GetExpandedNameW")
    }
    fn GetFileAttributesA(&self, lp_file_name: PCSTR) -> u32 {
        todo!("GetFileAttributesA")
    }
    fn GetFileAttributesExA(
        &self,
        lp_file_name: PCSTR,
        f_info_level_id: GET_FILEEX_INFO_LEVELS,
        lp_file_information: MutPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetFileAttributesExA")
    }
    fn GetFileAttributesExFromAppW(
        &self,
        lp_file_name: PCWSTR,
        f_info_level_id: GET_FILEEX_INFO_LEVELS,
        lp_file_information: MutPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetFileAttributesExFromAppW")
    }
    fn GetFileAttributesExW(
        &self,
        lp_file_name: PCWSTR,
        f_info_level_id: GET_FILEEX_INFO_LEVELS,
        lp_file_information: MutPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetFileAttributesExW")
    }
    fn GetFileAttributesTransactedA(
        &self,
        lp_file_name: PCSTR,
        f_info_level_id: GET_FILEEX_INFO_LEVELS,
        lp_file_information: MutPtr<::core::ffi::c_void>,
        h_transaction: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("GetFileAttributesTransactedA")
    }
    fn GetFileAttributesTransactedW(
        &self,
        lp_file_name: PCWSTR,
        f_info_level_id: GET_FILEEX_INFO_LEVELS,
        lp_file_information: MutPtr<::core::ffi::c_void>,
        h_transaction: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("GetFileAttributesTransactedW")
    }
    fn GetFileAttributesW(&self, lp_file_name: PCWSTR) -> u32 {
        todo!("GetFileAttributesW")
    }
    fn GetFileBandwidthReservation(
        &self,
        h_file: super::super::Foundation::HANDLE,
        lp_period_milliseconds: MutPtr<u32>,
        lp_bytes_per_period: MutPtr<u32>,
        p_discardable: MutPtr<i32>,
        lp_transfer_size: MutPtr<u32>,
        lp_num_outstanding_requests: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetFileBandwidthReservation")
    }
    fn GetFileInformationByHandle(
        &self,
        h_file: super::super::Foundation::HANDLE,
        lp_file_information: MutPtr<BY_HANDLE_FILE_INFORMATION>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetFileInformationByHandle")
    }
    fn GetFileInformationByHandleEx(
        &self,
        h_file: super::super::Foundation::HANDLE,
        file_information_class: FILE_INFO_BY_HANDLE_CLASS,
        lp_file_information: MutPtr<::core::ffi::c_void>,
        dw_buffer_size: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("GetFileInformationByHandleEx")
    }
    fn GetFileSize(
        &self,
        h_file: super::super::Foundation::HANDLE,
        lp_file_size_high: MutPtr<u32>,
    ) -> u32 {
        todo!("GetFileSize")
    }
    fn GetFileSizeEx(
        &self,
        h_file: super::super::Foundation::HANDLE,
        lp_file_size: MutPtr<i64>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetFileSizeEx")
    }
    fn GetFileTime(
        &self,
        h_file: super::super::Foundation::HANDLE,
        lp_creation_time: MutPtr<super::super::Foundation::FILETIME>,
        lp_last_access_time: MutPtr<super::super::Foundation::FILETIME>,
        lp_last_write_time: MutPtr<super::super::Foundation::FILETIME>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetFileTime")
    }
    fn GetFileType(&self, h_file: super::super::Foundation::HANDLE) -> u32 {
        todo!("GetFileType")
    }
    fn GetFileVersionInfoA(
        &self,
        lptstr_filename: PCSTR,
        dw_handle: u32,
        dw_len: u32,
        lp_data: MutPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetFileVersionInfoA")
    }
    fn GetFileVersionInfoExA(
        &self,
        dw_flags: GET_FILE_VERSION_INFO_FLAGS,
        lpwstr_filename: PCSTR,
        dw_handle: u32,
        dw_len: u32,
        lp_data: MutPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetFileVersionInfoExA")
    }
    fn GetFileVersionInfoExW(
        &self,
        dw_flags: GET_FILE_VERSION_INFO_FLAGS,
        lpwstr_filename: PCWSTR,
        dw_handle: u32,
        dw_len: u32,
        lp_data: MutPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetFileVersionInfoExW")
    }
    fn GetFileVersionInfoSizeA(&self, lptstr_filename: PCSTR, lpdw_handle: MutPtr<u32>) -> u32 {
        todo!("GetFileVersionInfoSizeA")
    }
    fn GetFileVersionInfoSizeExA(
        &self,
        dw_flags: GET_FILE_VERSION_INFO_FLAGS,
        lpwstr_filename: PCSTR,
        lpdw_handle: MutPtr<u32>,
    ) -> u32 {
        todo!("GetFileVersionInfoSizeExA")
    }
    fn GetFileVersionInfoSizeExW(
        &self,
        dw_flags: GET_FILE_VERSION_INFO_FLAGS,
        lpwstr_filename: PCWSTR,
        lpdw_handle: MutPtr<u32>,
    ) -> u32 {
        todo!("GetFileVersionInfoSizeExW")
    }
    fn GetFileVersionInfoSizeW(&self, lptstr_filename: PCWSTR, lpdw_handle: MutPtr<u32>) -> u32 {
        todo!("GetFileVersionInfoSizeW")
    }
    fn GetFileVersionInfoW(
        &self,
        lptstr_filename: PCWSTR,
        dw_handle: u32,
        dw_len: u32,
        lp_data: MutPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetFileVersionInfoW")
    }
    fn GetFinalPathNameByHandleA(
        &self,
        h_file: super::super::Foundation::HANDLE,
        lpsz_file_path: PSTR,
        cch_file_path: u32,
        dw_flags: FILE_NAME,
    ) -> u32 {
        todo!("GetFinalPathNameByHandleA")
    }
    fn GetFinalPathNameByHandleW(
        &self,
        h_file: super::super::Foundation::HANDLE,
        lpsz_file_path: PWSTR,
        cch_file_path: u32,
        dw_flags: FILE_NAME,
    ) -> u32 {
        todo!("GetFinalPathNameByHandleW")
    }
    fn GetFullPathNameA(
        &self,
        lp_file_name: PCSTR,
        n_buffer_length: u32,
        lp_buffer: PSTR,
        lp_file_part: MutPtr<PSTR>,
    ) -> u32 {
        todo!("GetFullPathNameA")
    }
    fn GetFullPathNameTransactedA(
        &self,
        lp_file_name: PCSTR,
        n_buffer_length: u32,
        lp_buffer: PSTR,
        lp_file_part: MutPtr<PSTR>,
        h_transaction: super::super::Foundation::HANDLE,
    ) -> u32 {
        todo!("GetFullPathNameTransactedA")
    }
    fn GetFullPathNameTransactedW(
        &self,
        lp_file_name: PCWSTR,
        n_buffer_length: u32,
        lp_buffer: PWSTR,
        lp_file_part: MutPtr<PWSTR>,
        h_transaction: super::super::Foundation::HANDLE,
    ) -> u32 {
        todo!("GetFullPathNameTransactedW")
    }
    fn GetFullPathNameW(
        &self,
        lp_file_name: PCWSTR,
        n_buffer_length: u32,
        lp_buffer: PWSTR,
        lp_file_part: MutPtr<PWSTR>,
    ) -> u32 {
        todo!("GetFullPathNameW")
    }
    fn GetIoRingInfo(
        &self,
        io_ring: ConstPtr<HIORING__>,
        info: MutPtr<IORING_INFO>,
    ) -> crate::core::HRESULT {
        todo!("GetIoRingInfo")
    }
    fn GetLogicalDriveStringsA(&self, n_buffer_length: u32, lp_buffer: PSTR) -> u32 {
        todo!("GetLogicalDriveStringsA")
    }
    fn GetLogicalDriveStringsW(&self, n_buffer_length: u32, lp_buffer: PWSTR) -> u32 {
        todo!("GetLogicalDriveStringsW")
    }
    fn GetLogicalDrives(&self) -> u32 {
        todo!("GetLogicalDrives")
    }
    fn GetLongPathNameA(
        &self,
        lpsz_short_path: PCSTR,
        lpsz_long_path: PSTR,
        cch_buffer: u32,
    ) -> u32 {
        todo!("GetLongPathNameA")
    }
    fn GetLongPathNameTransactedA(
        &self,
        lpsz_short_path: PCSTR,
        lpsz_long_path: PSTR,
        cch_buffer: u32,
        h_transaction: super::super::Foundation::HANDLE,
    ) -> u32 {
        todo!("GetLongPathNameTransactedA")
    }
    fn GetLongPathNameTransactedW(
        &self,
        lpsz_short_path: PCWSTR,
        lpsz_long_path: PWSTR,
        cch_buffer: u32,
        h_transaction: super::super::Foundation::HANDLE,
    ) -> u32 {
        todo!("GetLongPathNameTransactedW")
    }
    fn GetLongPathNameW(
        &self,
        lpsz_short_path: PCWSTR,
        lpsz_long_path: PWSTR,
        cch_buffer: u32,
    ) -> u32 {
        todo!("GetLongPathNameW")
    }
    fn GetNotificationResourceManager(
        &self,
        resource_manager_handle: super::super::Foundation::HANDLE,
        transaction_notification: MutPtr<TRANSACTION_NOTIFICATION>,
        notification_length: u32,
        dw_milliseconds: u32,
        return_length: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetNotificationResourceManager")
    }
    fn GetNotificationResourceManagerAsync(
        &self,
        resource_manager_handle: super::super::Foundation::HANDLE,
        transaction_notification: MutPtr<TRANSACTION_NOTIFICATION>,
        transaction_notification_length: u32,
        return_length: MutPtr<u32>,
        lp_overlapped: MutPtr<super::super::System::IO::OVERLAPPED>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetNotificationResourceManagerAsync")
    }
    fn GetShortPathNameA(
        &self,
        lpsz_long_path: PCSTR,
        lpsz_short_path: PSTR,
        cch_buffer: u32,
    ) -> u32 {
        todo!("GetShortPathNameA")
    }
    fn GetShortPathNameW(
        &self,
        lpsz_long_path: PCWSTR,
        lpsz_short_path: PWSTR,
        cch_buffer: u32,
    ) -> u32 {
        todo!("GetShortPathNameW")
    }
    fn GetTapeParameters(
        &self,
        h_device: super::super::Foundation::HANDLE,
        dw_operation: GET_TAPE_DRIVE_PARAMETERS_OPERATION,
        lpdw_size: MutPtr<u32>,
        lp_tape_information: MutPtr<::core::ffi::c_void>,
    ) -> u32 {
        todo!("GetTapeParameters")
    }
    fn GetTapePosition(
        &self,
        h_device: super::super::Foundation::HANDLE,
        dw_position_type: TAPE_POSITION_TYPE,
        lpdw_partition: MutPtr<u32>,
        lpdw_offset_low: MutPtr<u32>,
        lpdw_offset_high: MutPtr<u32>,
    ) -> u32 {
        todo!("GetTapePosition")
    }
    fn GetTapeStatus(&self, h_device: super::super::Foundation::HANDLE) -> u32 {
        todo!("GetTapeStatus")
    }
    fn GetTempFileNameA(
        &self,
        lp_path_name: PCSTR,
        lp_prefix_string: PCSTR,
        u_unique: u32,
        lp_temp_file_name: PSTR,
    ) -> u32 {
        todo!("GetTempFileNameA")
    }
    fn GetTempFileNameW(
        &self,
        lp_path_name: PCWSTR,
        lp_prefix_string: PCWSTR,
        u_unique: u32,
        lp_temp_file_name: PWSTR,
    ) -> u32 {
        todo!("GetTempFileNameW")
    }
    fn GetTempPath2A(&self, buffer_length: u32, buffer: PSTR) -> u32 {
        todo!("GetTempPath2A")
    }
    fn GetTempPath2W(&self, buffer_length: u32, buffer: PWSTR) -> u32 {
        todo!("GetTempPath2W")
    }
    fn GetTempPathA(&self, n_buffer_length: u32, lp_buffer: PSTR) -> u32 {
        todo!("GetTempPathA")
    }
    fn GetTempPathW(&self, n_buffer_length: u32, lp_buffer: PWSTR) -> u32 {
        todo!("GetTempPathW")
    }
    fn GetTransactionId(
        &self,
        transaction_handle: super::super::Foundation::HANDLE,
        transaction_id: MutPtr<crate::core::GUID>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetTransactionId")
    }
    fn GetTransactionInformation(
        &self,
        transaction_handle: super::super::Foundation::HANDLE,
        outcome: MutPtr<u32>,
        isolation_level: MutPtr<u32>,
        isolation_flags: MutPtr<u32>,
        timeout: MutPtr<u32>,
        buffer_length: u32,
        description: PWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("GetTransactionInformation")
    }
    fn GetTransactionManagerId(
        &self,
        transaction_manager_handle: super::super::Foundation::HANDLE,
        transaction_manager_id: MutPtr<crate::core::GUID>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetTransactionManagerId")
    }
    fn GetVolumeInformationA(
        &self,
        lp_root_path_name: PCSTR,
        lp_volume_name_buffer: PSTR,
        n_volume_name_size: u32,
        lp_volume_serial_number: MutPtr<u32>,
        lp_maximum_component_length: MutPtr<u32>,
        lp_file_system_flags: MutPtr<u32>,
        lp_file_system_name_buffer: PSTR,
        n_file_system_name_size: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("GetVolumeInformationA")
    }
    fn GetVolumeInformationByHandleW(
        &self,
        h_file: super::super::Foundation::HANDLE,
        lp_volume_name_buffer: PWSTR,
        n_volume_name_size: u32,
        lp_volume_serial_number: MutPtr<u32>,
        lp_maximum_component_length: MutPtr<u32>,
        lp_file_system_flags: MutPtr<u32>,
        lp_file_system_name_buffer: PWSTR,
        n_file_system_name_size: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("GetVolumeInformationByHandleW")
    }
    fn GetVolumeInformationW(
        &self,
        lp_root_path_name: PCWSTR,
        lp_volume_name_buffer: PWSTR,
        n_volume_name_size: u32,
        lp_volume_serial_number: MutPtr<u32>,
        lp_maximum_component_length: MutPtr<u32>,
        lp_file_system_flags: MutPtr<u32>,
        lp_file_system_name_buffer: PWSTR,
        n_file_system_name_size: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("GetVolumeInformationW")
    }
    fn GetVolumeNameForVolumeMountPointA(
        &self,
        lpsz_volume_mount_point: PCSTR,
        lpsz_volume_name: PSTR,
        cch_buffer_length: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("GetVolumeNameForVolumeMountPointA")
    }
    fn GetVolumeNameForVolumeMountPointW(
        &self,
        lpsz_volume_mount_point: PCWSTR,
        lpsz_volume_name: PWSTR,
        cch_buffer_length: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("GetVolumeNameForVolumeMountPointW")
    }
    fn GetVolumePathNameA(
        &self,
        lpsz_file_name: PCSTR,
        lpsz_volume_path_name: PSTR,
        cch_buffer_length: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("GetVolumePathNameA")
    }
    fn GetVolumePathNameW(
        &self,
        lpsz_file_name: PCWSTR,
        lpsz_volume_path_name: PWSTR,
        cch_buffer_length: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("GetVolumePathNameW")
    }
    fn GetVolumePathNamesForVolumeNameA(
        &self,
        lpsz_volume_name: PCSTR,
        lpsz_volume_path_names: PSTR,
        cch_buffer_length: u32,
        lpcch_return_length: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetVolumePathNamesForVolumeNameA")
    }
    fn GetVolumePathNamesForVolumeNameW(
        &self,
        lpsz_volume_name: PCWSTR,
        lpsz_volume_path_names: PWSTR,
        cch_buffer_length: u32,
        lpcch_return_length: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetVolumePathNamesForVolumeNameW")
    }
    fn IsIoRingOpSupported(
        &self,
        io_ring: ConstPtr<HIORING__>,
        op: IORING_OP_CODE,
    ) -> super::super::Foundation::BOOL {
        todo!("IsIoRingOpSupported")
    }
    fn LZClose(&self, h_file: i32) {
        todo!("LZClose")
    }
    fn LZCopy(&self, hf_source: i32, hf_dest: i32) -> i32 {
        todo!("LZCopy")
    }
    fn LZDone(&self) {
        todo!("LZDone")
    }
    fn LZInit(&self, hf_source: i32) -> i32 {
        todo!("LZInit")
    }
    fn LZOpenFileA(
        &self,
        lp_file_name: PCSTR,
        lp_re_open_buf: MutPtr<OFSTRUCT>,
        w_style: LZOPENFILE_STYLE,
    ) -> i32 {
        todo!("LZOpenFileA")
    }
    fn LZOpenFileW(
        &self,
        lp_file_name: PCWSTR,
        lp_re_open_buf: MutPtr<OFSTRUCT>,
        w_style: LZOPENFILE_STYLE,
    ) -> i32 {
        todo!("LZOpenFileW")
    }
    fn LZRead(&self, h_file: i32, lp_buffer: PSTR, cb_read: i32) -> i32 {
        todo!("LZRead")
    }
    fn LZSeek(&self, h_file: i32, l_offset: i32, i_origin: i32) -> i32 {
        todo!("LZSeek")
    }
    fn LZStart(&self) -> i32 {
        todo!("LZStart")
    }
    fn MoveFileA(
        &self,
        lp_existing_file_name: PCSTR,
        lp_new_file_name: PCSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("MoveFileA")
    }
    fn MoveFileExA(
        &self,
        lp_existing_file_name: PCSTR,
        lp_new_file_name: PCSTR,
        dw_flags: MOVE_FILE_FLAGS,
    ) -> super::super::Foundation::BOOL {
        todo!("MoveFileExA")
    }
    fn MoveFileExW(
        &self,
        lp_existing_file_name: PCWSTR,
        lp_new_file_name: PCWSTR,
        dw_flags: MOVE_FILE_FLAGS,
    ) -> super::super::Foundation::BOOL {
        todo!("MoveFileExW")
    }
    fn MoveFileFromAppW(
        &self,
        lp_existing_file_name: PCWSTR,
        lp_new_file_name: PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("MoveFileFromAppW")
    }
    fn MoveFileTransactedA(
        &self,
        lp_existing_file_name: PCSTR,
        lp_new_file_name: PCSTR,
        lp_progress_routine: LPPROGRESS_ROUTINE,
        lp_data: ConstPtr<::core::ffi::c_void>,
        dw_flags: MOVE_FILE_FLAGS,
        h_transaction: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("MoveFileTransactedA")
    }
    fn MoveFileTransactedW(
        &self,
        lp_existing_file_name: PCWSTR,
        lp_new_file_name: PCWSTR,
        lp_progress_routine: LPPROGRESS_ROUTINE,
        lp_data: ConstPtr<::core::ffi::c_void>,
        dw_flags: MOVE_FILE_FLAGS,
        h_transaction: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("MoveFileTransactedW")
    }
    fn MoveFileW(
        &self,
        lp_existing_file_name: PCWSTR,
        lp_new_file_name: PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("MoveFileW")
    }
    fn MoveFileWithProgressA(
        &self,
        lp_existing_file_name: PCSTR,
        lp_new_file_name: PCSTR,
        lp_progress_routine: LPPROGRESS_ROUTINE,
        lp_data: ConstPtr<::core::ffi::c_void>,
        dw_flags: MOVE_FILE_FLAGS,
    ) -> super::super::Foundation::BOOL {
        todo!("MoveFileWithProgressA")
    }
    fn MoveFileWithProgressW(
        &self,
        lp_existing_file_name: PCWSTR,
        lp_new_file_name: PCWSTR,
        lp_progress_routine: LPPROGRESS_ROUTINE,
        lp_data: ConstPtr<::core::ffi::c_void>,
        dw_flags: MOVE_FILE_FLAGS,
    ) -> super::super::Foundation::BOOL {
        todo!("MoveFileWithProgressW")
    }
    fn NetConnectionEnum(
        &self,
        servername: PCWSTR,
        qualifier: PCWSTR,
        level: u32,
        bufptr: MutPtr<ConstPtr<u8>>,
        prefmaxlen: u32,
        entriesread: MutPtr<u32>,
        totalentries: MutPtr<u32>,
        resume_handle: MutPtr<u32>,
    ) -> u32 {
        todo!("NetConnectionEnum")
    }
    fn NetFileClose(&self, servername: PCWSTR, fileid: u32) -> u32 {
        todo!("NetFileClose")
    }
    fn NetFileEnum(
        &self,
        servername: PCWSTR,
        basepath: PCWSTR,
        username: PCWSTR,
        level: u32,
        bufptr: MutPtr<ConstPtr<u8>>,
        prefmaxlen: u32,
        entriesread: MutPtr<u32>,
        totalentries: MutPtr<u32>,
        resume_handle: MutPtr<PtrRepr>,
    ) -> u32 {
        todo!("NetFileEnum")
    }
    fn NetFileGetInfo(
        &self,
        servername: PCWSTR,
        fileid: u32,
        level: u32,
        bufptr: MutPtr<ConstPtr<u8>>,
    ) -> u32 {
        todo!("NetFileGetInfo")
    }
    fn NetServerAliasAdd(&self, servername: PCWSTR, level: u32, buf: ConstPtr<u8>) -> u32 {
        todo!("NetServerAliasAdd")
    }
    fn NetServerAliasDel(&self, servername: PCWSTR, level: u32, buf: ConstPtr<u8>) -> u32 {
        todo!("NetServerAliasDel")
    }
    fn NetServerAliasEnum(
        &self,
        servername: PCWSTR,
        level: u32,
        bufptr: MutPtr<ConstPtr<u8>>,
        prefmaxlen: u32,
        entriesread: MutPtr<u32>,
        totalentries: MutPtr<u32>,
        resumehandle: MutPtr<u32>,
    ) -> u32 {
        todo!("NetServerAliasEnum")
    }
    fn NetSessionDel(&self, servername: PCWSTR, unc_client_name: PCWSTR, username: PCWSTR) -> u32 {
        todo!("NetSessionDel")
    }
    fn NetSessionEnum(
        &self,
        servername: PCWSTR,
        unc_client_name: PCWSTR,
        username: PCWSTR,
        level: u32,
        bufptr: MutPtr<ConstPtr<u8>>,
        prefmaxlen: u32,
        entriesread: MutPtr<u32>,
        totalentries: MutPtr<u32>,
        resume_handle: MutPtr<u32>,
    ) -> u32 {
        todo!("NetSessionEnum")
    }
    fn NetSessionGetInfo(
        &self,
        servername: PCWSTR,
        unc_client_name: PCWSTR,
        username: PCWSTR,
        level: u32,
        bufptr: MutPtr<ConstPtr<u8>>,
    ) -> u32 {
        todo!("NetSessionGetInfo")
    }
    fn NetShareAdd(
        &self,
        servername: PCWSTR,
        level: u32,
        buf: ConstPtr<u8>,
        parm_err: MutPtr<u32>,
    ) -> u32 {
        todo!("NetShareAdd")
    }
    fn NetShareCheck(&self, servername: PCWSTR, device: PCWSTR, r#type: MutPtr<u32>) -> u32 {
        todo!("NetShareCheck")
    }
    fn NetShareDel(&self, servername: PCWSTR, netname: PCWSTR, reserved: u32) -> u32 {
        todo!("NetShareDel")
    }
    fn NetShareDelEx(&self, servername: PCWSTR, level: u32, buf: ConstPtr<u8>) -> u32 {
        todo!("NetShareDelEx")
    }
    fn NetShareDelSticky(&self, servername: PCWSTR, netname: PCWSTR, reserved: u32) -> u32 {
        todo!("NetShareDelSticky")
    }
    fn NetShareEnum(
        &self,
        servername: PCWSTR,
        level: u32,
        bufptr: MutPtr<ConstPtr<u8>>,
        prefmaxlen: u32,
        entriesread: MutPtr<u32>,
        totalentries: MutPtr<u32>,
        resume_handle: MutPtr<u32>,
    ) -> u32 {
        todo!("NetShareEnum")
    }
    fn NetShareEnumSticky(
        &self,
        servername: PCWSTR,
        level: u32,
        bufptr: MutPtr<ConstPtr<u8>>,
        prefmaxlen: u32,
        entriesread: MutPtr<u32>,
        totalentries: MutPtr<u32>,
        resume_handle: MutPtr<u32>,
    ) -> u32 {
        todo!("NetShareEnumSticky")
    }
    fn NetShareGetInfo(
        &self,
        servername: PCWSTR,
        netname: PCWSTR,
        level: u32,
        bufptr: MutPtr<ConstPtr<u8>>,
    ) -> u32 {
        todo!("NetShareGetInfo")
    }
    fn NetShareSetInfo(
        &self,
        servername: PCWSTR,
        netname: PCWSTR,
        level: u32,
        buf: ConstPtr<u8>,
        parm_err: MutPtr<u32>,
    ) -> u32 {
        todo!("NetShareSetInfo")
    }
    fn NetStatisticsGet(
        &self,
        server_name: ConstPtr<i8>,
        service: ConstPtr<i8>,
        level: u32,
        options: u32,
        buffer: MutPtr<ConstPtr<u8>>,
    ) -> u32 {
        todo!("NetStatisticsGet")
    }
    fn NtCreateFile(
        &self,
        file_handle: MutPtr<super::super::Foundation::HANDLE>,
        desired_access: u32,
        object_attributes: MutPtr<super::super::System::WindowsProgramming::OBJECT_ATTRIBUTES>,
        io_status_block: MutPtr<super::super::System::WindowsProgramming::IO_STATUS_BLOCK>,
        allocation_size: MutPtr<i64>,
        file_attributes: u32,
        share_access: FILE_SHARE_MODE,
        create_disposition: NT_CREATE_FILE_DISPOSITION,
        create_options: u32,
        ea_buffer: MutPtr<::core::ffi::c_void>,
        ea_length: u32,
    ) -> super::super::Foundation::NTSTATUS {
        todo!("NtCreateFile")
    }
    fn OpenEncryptedFileRawA(
        &self,
        lp_file_name: PCSTR,
        ul_flags: u32,
        pv_context: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> u32 {
        todo!("OpenEncryptedFileRawA")
    }
    fn OpenEncryptedFileRawW(
        &self,
        lp_file_name: PCWSTR,
        ul_flags: u32,
        pv_context: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> u32 {
        todo!("OpenEncryptedFileRawW")
    }
    fn OpenEnlistment(
        &self,
        dw_desired_access: u32,
        resource_manager_handle: super::super::Foundation::HANDLE,
        enlistment_id: MutPtr<crate::core::GUID>,
    ) -> super::super::Foundation::HANDLE {
        todo!("OpenEnlistment")
    }
    fn OpenFile(
        &self,
        lp_file_name: PCSTR,
        lp_re_open_buff: MutPtr<OFSTRUCT>,
        u_style: LZOPENFILE_STYLE,
    ) -> i32 {
        todo!("OpenFile")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn OpenFileById(
        &self,
        h_volume_hint: super::super::Foundation::HANDLE,
        lp_file_id: ConstPtr<FILE_ID_DESCRIPTOR>,
        dw_desired_access: FILE_ACCESS_FLAGS,
        dw_share_mode: FILE_SHARE_MODE,
        lp_security_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        dw_flags_and_attributes: FILE_FLAGS_AND_ATTRIBUTES,
    ) -> super::super::Foundation::HANDLE {
        todo!("OpenFileById")
    }
    fn OpenResourceManager(
        &self,
        dw_desired_access: u32,
        tm_handle: super::super::Foundation::HANDLE,
        resource_manager_id: MutPtr<crate::core::GUID>,
    ) -> super::super::Foundation::HANDLE {
        todo!("OpenResourceManager")
    }
    fn OpenTransaction(
        &self,
        dw_desired_access: u32,
        transaction_id: MutPtr<crate::core::GUID>,
    ) -> super::super::Foundation::HANDLE {
        todo!("OpenTransaction")
    }
    fn OpenTransactionManager(
        &self,
        log_file_name: PCWSTR,
        desired_access: u32,
        open_options: u32,
    ) -> super::super::Foundation::HANDLE {
        todo!("OpenTransactionManager")
    }
    fn OpenTransactionManagerById(
        &self,
        transaction_manager_id: ConstPtr<crate::core::GUID>,
        desired_access: u32,
        open_options: u32,
    ) -> super::super::Foundation::HANDLE {
        todo!("OpenTransactionManagerById")
    }
    fn PopIoRingCompletion(
        &self,
        io_ring: ConstPtr<HIORING__>,
        cqe: MutPtr<IORING_CQE>,
    ) -> crate::core::HRESULT {
        todo!("PopIoRingCompletion")
    }
    fn PrePrepareComplete(
        &self,
        enlistment_handle: super::super::Foundation::HANDLE,
        tm_virtual_clock: MutPtr<i64>,
    ) -> super::super::Foundation::BOOL {
        todo!("PrePrepareComplete")
    }
    fn PrePrepareEnlistment(
        &self,
        enlistment_handle: super::super::Foundation::HANDLE,
        tm_virtual_clock: MutPtr<i64>,
    ) -> super::super::Foundation::BOOL {
        todo!("PrePrepareEnlistment")
    }
    fn PrepareComplete(
        &self,
        enlistment_handle: super::super::Foundation::HANDLE,
        tm_virtual_clock: MutPtr<i64>,
    ) -> super::super::Foundation::BOOL {
        todo!("PrepareComplete")
    }
    fn PrepareEnlistment(
        &self,
        enlistment_handle: super::super::Foundation::HANDLE,
        tm_virtual_clock: MutPtr<i64>,
    ) -> super::super::Foundation::BOOL {
        todo!("PrepareEnlistment")
    }
    fn PrepareTape(
        &self,
        h_device: super::super::Foundation::HANDLE,
        dw_operation: PREPARE_TAPE_OPERATION,
        b_immediate: super::super::Foundation::BOOL,
    ) -> u32 {
        todo!("PrepareTape")
    }
    fn QueryDosDeviceA(&self, lp_device_name: PCSTR, lp_target_path: PSTR, ucch_max: u32) -> u32 {
        todo!("QueryDosDeviceA")
    }
    fn QueryDosDeviceW(&self, lp_device_name: PCWSTR, lp_target_path: PWSTR, ucch_max: u32) -> u32 {
        todo!("QueryDosDeviceW")
    }
    fn QueryIoRingCapabilities(
        &self,
        capabilities: MutPtr<IORING_CAPABILITIES>,
    ) -> crate::core::HRESULT {
        todo!("QueryIoRingCapabilities")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn QueryRecoveryAgentsOnEncryptedFile(
        &self,
        lp_file_name: PCWSTR,
        p_recovery_agents: MutPtr<ConstPtr<ENCRYPTION_CERTIFICATE_HASH_LIST>>,
    ) -> u32 {
        todo!("QueryRecoveryAgentsOnEncryptedFile")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn QueryUsersOnEncryptedFile(
        &self,
        lp_file_name: PCWSTR,
        p_users: MutPtr<ConstPtr<ENCRYPTION_CERTIFICATE_HASH_LIST>>,
    ) -> u32 {
        todo!("QueryUsersOnEncryptedFile")
    }
    fn ReOpenFile(
        &self,
        h_original_file: super::super::Foundation::HANDLE,
        dw_desired_access: FILE_ACCESS_FLAGS,
        dw_share_mode: FILE_SHARE_MODE,
        dw_flags_and_attributes: FILE_FLAGS_AND_ATTRIBUTES,
    ) -> super::super::Foundation::HANDLE {
        todo!("ReOpenFile")
    }
    fn ReadDirectoryChangesExW(
        &self,
        h_directory: super::super::Foundation::HANDLE,
        lp_buffer: MutPtr<::core::ffi::c_void>,
        n_buffer_length: u32,
        b_watch_subtree: super::super::Foundation::BOOL,
        dw_notify_filter: FILE_NOTIFY_CHANGE,
        lp_bytes_returned: MutPtr<u32>,
        lp_overlapped: MutPtr<super::super::System::IO::OVERLAPPED>,
        lp_completion_routine: super::super::System::IO::LPOVERLAPPED_COMPLETION_ROUTINE,
        read_directory_notify_information_class: READ_DIRECTORY_NOTIFY_INFORMATION_CLASS,
    ) -> super::super::Foundation::BOOL {
        todo!("ReadDirectoryChangesExW")
    }
    fn ReadDirectoryChangesW(
        &self,
        h_directory: super::super::Foundation::HANDLE,
        lp_buffer: MutPtr<::core::ffi::c_void>,
        n_buffer_length: u32,
        b_watch_subtree: super::super::Foundation::BOOL,
        dw_notify_filter: FILE_NOTIFY_CHANGE,
        lp_bytes_returned: MutPtr<u32>,
        lp_overlapped: MutPtr<super::super::System::IO::OVERLAPPED>,
        lp_completion_routine: super::super::System::IO::LPOVERLAPPED_COMPLETION_ROUTINE,
    ) -> super::super::Foundation::BOOL {
        todo!("ReadDirectoryChangesW")
    }
    fn ReadEncryptedFileRaw(
        &self,
        pf_export_callback: PFE_EXPORT_FUNC,
        pv_callback_context: ConstPtr<::core::ffi::c_void>,
        pv_context: ConstPtr<::core::ffi::c_void>,
    ) -> u32 {
        todo!("ReadEncryptedFileRaw")
    }
    fn ReadFile(
        &self,
        h_file: super::super::Foundation::HANDLE,
        lp_buffer: MutPtr<::core::ffi::c_void>,
        n_number_of_bytes_to_read: u32,
        lp_number_of_bytes_read: MutPtr<u32>,
        lp_overlapped: MutPtr<super::super::System::IO::OVERLAPPED>,
    ) -> super::super::Foundation::BOOL {
        todo!("ReadFile")
    }
    fn ReadFileEx(
        &self,
        h_file: super::super::Foundation::HANDLE,
        lp_buffer: MutPtr<::core::ffi::c_void>,
        n_number_of_bytes_to_read: u32,
        lp_overlapped: MutPtr<super::super::System::IO::OVERLAPPED>,
        lp_completion_routine: super::super::System::IO::LPOVERLAPPED_COMPLETION_ROUTINE,
    ) -> super::super::Foundation::BOOL {
        todo!("ReadFileEx")
    }
    fn ReadFileScatter(
        &self,
        h_file: super::super::Foundation::HANDLE,
        a_segment_array: ConstPtr<FILE_SEGMENT_ELEMENT>,
        n_number_of_bytes_to_read: u32,
        lp_reserved: MutPtr<u32>,
        lp_overlapped: MutPtr<super::super::System::IO::OVERLAPPED>,
    ) -> super::super::Foundation::BOOL {
        todo!("ReadFileScatter")
    }
    fn ReadOnlyEnlistment(
        &self,
        enlistment_handle: super::super::Foundation::HANDLE,
        tm_virtual_clock: MutPtr<i64>,
    ) -> super::super::Foundation::BOOL {
        todo!("ReadOnlyEnlistment")
    }
    fn RecoverEnlistment(
        &self,
        enlistment_handle: super::super::Foundation::HANDLE,
        enlistment_key: MutPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!("RecoverEnlistment")
    }
    fn RecoverResourceManager(
        &self,
        resource_manager_handle: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("RecoverResourceManager")
    }
    fn RecoverTransactionManager(
        &self,
        transaction_manager_handle: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("RecoverTransactionManager")
    }
    fn RemoveDirectoryA(&self, lp_path_name: PCSTR) -> super::super::Foundation::BOOL {
        todo!("RemoveDirectoryA")
    }
    fn RemoveDirectoryFromAppW(&self, lp_path_name: PCWSTR) -> super::super::Foundation::BOOL {
        todo!("RemoveDirectoryFromAppW")
    }
    fn RemoveDirectoryTransactedA(
        &self,
        lp_path_name: PCSTR,
        h_transaction: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("RemoveDirectoryTransactedA")
    }
    fn RemoveDirectoryTransactedW(
        &self,
        lp_path_name: PCWSTR,
        h_transaction: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("RemoveDirectoryTransactedW")
    }
    fn RemoveDirectoryW(&self, lp_path_name: PCWSTR) -> super::super::Foundation::BOOL {
        todo!("RemoveDirectoryW")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn RemoveUsersFromEncryptedFile(
        &self,
        lp_file_name: PCWSTR,
        p_hashes: ConstPtr<ENCRYPTION_CERTIFICATE_HASH_LIST>,
    ) -> u32 {
        todo!("RemoveUsersFromEncryptedFile")
    }
    fn RenameTransactionManager(
        &self,
        log_file_name: PCWSTR,
        existing_transaction_manager_guid: MutPtr<crate::core::GUID>,
    ) -> super::super::Foundation::BOOL {
        todo!("RenameTransactionManager")
    }
    fn ReplaceFileA(
        &self,
        lp_replaced_file_name: PCSTR,
        lp_replacement_file_name: PCSTR,
        lp_backup_file_name: PCSTR,
        dw_replace_flags: REPLACE_FILE_FLAGS,
        lp_exclude: MutPtr<::core::ffi::c_void>,
        lp_reserved: MutPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!("ReplaceFileA")
    }
    fn ReplaceFileFromAppW(
        &self,
        lp_replaced_file_name: PCWSTR,
        lp_replacement_file_name: PCWSTR,
        lp_backup_file_name: PCWSTR,
        dw_replace_flags: u32,
        lp_exclude: MutPtr<::core::ffi::c_void>,
        lp_reserved: MutPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!("ReplaceFileFromAppW")
    }
    fn ReplaceFileW(
        &self,
        lp_replaced_file_name: PCWSTR,
        lp_replacement_file_name: PCWSTR,
        lp_backup_file_name: PCWSTR,
        dw_replace_flags: REPLACE_FILE_FLAGS,
        lp_exclude: MutPtr<::core::ffi::c_void>,
        lp_reserved: MutPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!("ReplaceFileW")
    }
    fn RollbackComplete(
        &self,
        enlistment_handle: super::super::Foundation::HANDLE,
        tm_virtual_clock: MutPtr<i64>,
    ) -> super::super::Foundation::BOOL {
        todo!("RollbackComplete")
    }
    fn RollbackEnlistment(
        &self,
        enlistment_handle: super::super::Foundation::HANDLE,
        tm_virtual_clock: MutPtr<i64>,
    ) -> super::super::Foundation::BOOL {
        todo!("RollbackEnlistment")
    }
    fn RollbackTransaction(
        &self,
        transaction_handle: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("RollbackTransaction")
    }
    fn RollbackTransactionAsync(
        &self,
        transaction_handle: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("RollbackTransactionAsync")
    }
    fn RollforwardTransactionManager(
        &self,
        transaction_manager_handle: super::super::Foundation::HANDLE,
        tm_virtual_clock: MutPtr<i64>,
    ) -> super::super::Foundation::BOOL {
        todo!("RollforwardTransactionManager")
    }
    fn SearchPathA(
        &self,
        lp_path: PCSTR,
        lp_file_name: PCSTR,
        lp_extension: PCSTR,
        n_buffer_length: u32,
        lp_buffer: PSTR,
        lp_file_part: MutPtr<PSTR>,
    ) -> u32 {
        todo!("SearchPathA")
    }
    fn SearchPathW(
        &self,
        lp_path: PCWSTR,
        lp_file_name: PCWSTR,
        lp_extension: PCWSTR,
        n_buffer_length: u32,
        lp_buffer: PWSTR,
        lp_file_part: MutPtr<PWSTR>,
    ) -> u32 {
        todo!("SearchPathW")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn SetEncryptedFileMetadata(
        &self,
        lp_file_name: PCWSTR,
        pb_old_metadata: ConstPtr<u8>,
        pb_new_metadata: ConstPtr<u8>,
        p_owner_hash: ConstPtr<ENCRYPTION_CERTIFICATE_HASH>,
        dw_operation: u32,
        p_certificates_added: ConstPtr<ENCRYPTION_CERTIFICATE_HASH_LIST>,
    ) -> u32 {
        todo!("SetEncryptedFileMetadata")
    }
    fn SetEndOfFile(
        &self,
        h_file: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("SetEndOfFile")
    }
    fn SetEnlistmentRecoveryInformation(
        &self,
        enlistment_handle: super::super::Foundation::HANDLE,
        buffer_size: u32,
        buffer: MutPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!("SetEnlistmentRecoveryInformation")
    }
    fn SetFileApisToANSI(&self) {
        todo!("SetFileApisToANSI")
    }
    fn SetFileApisToOEM(&self) {
        todo!("SetFileApisToOEM")
    }
    fn SetFileAttributesA(
        &self,
        lp_file_name: PCSTR,
        dw_file_attributes: FILE_FLAGS_AND_ATTRIBUTES,
    ) -> super::super::Foundation::BOOL {
        todo!("SetFileAttributesA")
    }
    fn SetFileAttributesFromAppW(
        &self,
        lp_file_name: PCWSTR,
        dw_file_attributes: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("SetFileAttributesFromAppW")
    }
    fn SetFileAttributesTransactedA(
        &self,
        lp_file_name: PCSTR,
        dw_file_attributes: u32,
        h_transaction: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("SetFileAttributesTransactedA")
    }
    fn SetFileAttributesTransactedW(
        &self,
        lp_file_name: PCWSTR,
        dw_file_attributes: u32,
        h_transaction: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("SetFileAttributesTransactedW")
    }
    fn SetFileAttributesW(
        &self,
        lp_file_name: PCWSTR,
        dw_file_attributes: FILE_FLAGS_AND_ATTRIBUTES,
    ) -> super::super::Foundation::BOOL {
        todo!("SetFileAttributesW")
    }
    fn SetFileBandwidthReservation(
        &self,
        h_file: super::super::Foundation::HANDLE,
        n_period_milliseconds: u32,
        n_bytes_per_period: u32,
        b_discardable: super::super::Foundation::BOOL,
        lp_transfer_size: MutPtr<u32>,
        lp_num_outstanding_requests: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("SetFileBandwidthReservation")
    }
    fn SetFileCompletionNotificationModes(
        &self,
        file_handle: super::super::Foundation::HANDLE,
        flags: u8,
    ) -> super::super::Foundation::BOOL {
        todo!("SetFileCompletionNotificationModes")
    }
    fn SetFileInformationByHandle(
        &self,
        h_file: super::super::Foundation::HANDLE,
        file_information_class: FILE_INFO_BY_HANDLE_CLASS,
        lp_file_information: ConstPtr<::core::ffi::c_void>,
        dw_buffer_size: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("SetFileInformationByHandle")
    }
    fn SetFileIoOverlappedRange(
        &self,
        file_handle: super::super::Foundation::HANDLE,
        overlapped_range_start: ConstPtr<u8>,
        length: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("SetFileIoOverlappedRange")
    }
    fn SetFilePointer(
        &self,
        h_file: super::super::Foundation::HANDLE,
        l_distance_to_move: i32,
        lp_distance_to_move_high: MutPtr<i32>,
        dw_move_method: SET_FILE_POINTER_MOVE_METHOD,
    ) -> u32 {
        todo!("SetFilePointer")
    }
    fn SetFilePointerEx(
        &self,
        h_file: super::super::Foundation::HANDLE,
        li_distance_to_move: i64,
        lp_new_file_pointer: MutPtr<i64>,
        dw_move_method: SET_FILE_POINTER_MOVE_METHOD,
    ) -> super::super::Foundation::BOOL {
        todo!("SetFilePointerEx")
    }
    fn SetFileShortNameA(
        &self,
        h_file: super::super::Foundation::HANDLE,
        lp_short_name: PCSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("SetFileShortNameA")
    }
    fn SetFileShortNameW(
        &self,
        h_file: super::super::Foundation::HANDLE,
        lp_short_name: PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("SetFileShortNameW")
    }
    fn SetFileTime(
        &self,
        h_file: super::super::Foundation::HANDLE,
        lp_creation_time: ConstPtr<super::super::Foundation::FILETIME>,
        lp_last_access_time: ConstPtr<super::super::Foundation::FILETIME>,
        lp_last_write_time: ConstPtr<super::super::Foundation::FILETIME>,
    ) -> super::super::Foundation::BOOL {
        todo!("SetFileTime")
    }
    fn SetFileValidData(
        &self,
        h_file: super::super::Foundation::HANDLE,
        valid_data_length: i64,
    ) -> super::super::Foundation::BOOL {
        todo!("SetFileValidData")
    }
    fn SetIoRingCompletionEvent(
        &self,
        io_ring: ConstPtr<HIORING__>,
        h_event: super::super::Foundation::HANDLE,
    ) -> crate::core::HRESULT {
        todo!("SetIoRingCompletionEvent")
    }
    fn SetResourceManagerCompletionPort(
        &self,
        resource_manager_handle: super::super::Foundation::HANDLE,
        io_completion_port_handle: super::super::Foundation::HANDLE,
        completion_key: PtrRepr,
    ) -> super::super::Foundation::BOOL {
        todo!("SetResourceManagerCompletionPort")
    }
    fn SetSearchPathMode(&self, flags: u32) -> super::super::Foundation::BOOL {
        todo!("SetSearchPathMode")
    }
    fn SetTapeParameters(
        &self,
        h_device: super::super::Foundation::HANDLE,
        dw_operation: TAPE_INFORMATION_TYPE,
        lp_tape_information: ConstPtr<::core::ffi::c_void>,
    ) -> u32 {
        todo!("SetTapeParameters")
    }
    fn SetTapePosition(
        &self,
        h_device: super::super::Foundation::HANDLE,
        dw_position_method: TAPE_POSITION_METHOD,
        dw_partition: u32,
        dw_offset_low: u32,
        dw_offset_high: u32,
        b_immediate: super::super::Foundation::BOOL,
    ) -> u32 {
        todo!("SetTapePosition")
    }
    fn SetTransactionInformation(
        &self,
        transaction_handle: super::super::Foundation::HANDLE,
        isolation_level: u32,
        isolation_flags: u32,
        timeout: u32,
        description: PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("SetTransactionInformation")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn SetUserFileEncryptionKey(
        &self,
        p_encryption_certificate: ConstPtr<ENCRYPTION_CERTIFICATE>,
    ) -> u32 {
        todo!("SetUserFileEncryptionKey")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn SetUserFileEncryptionKeyEx(
        &self,
        p_encryption_certificate: ConstPtr<ENCRYPTION_CERTIFICATE>,
        dw_capabilities: u32,
        dw_flags: u32,
        pv_reserved: MutPtr<::core::ffi::c_void>,
    ) -> u32 {
        todo!("SetUserFileEncryptionKeyEx")
    }
    fn SetVolumeLabelA(
        &self,
        lp_root_path_name: PCSTR,
        lp_volume_name: PCSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("SetVolumeLabelA")
    }
    fn SetVolumeLabelW(
        &self,
        lp_root_path_name: PCWSTR,
        lp_volume_name: PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("SetVolumeLabelW")
    }
    fn SetVolumeMountPointA(
        &self,
        lpsz_volume_mount_point: PCSTR,
        lpsz_volume_name: PCSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("SetVolumeMountPointA")
    }
    fn SetVolumeMountPointW(
        &self,
        lpsz_volume_mount_point: PCWSTR,
        lpsz_volume_name: PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("SetVolumeMountPointW")
    }
    fn SinglePhaseReject(
        &self,
        enlistment_handle: super::super::Foundation::HANDLE,
        tm_virtual_clock: MutPtr<i64>,
    ) -> super::super::Foundation::BOOL {
        todo!("SinglePhaseReject")
    }
    fn SubmitIoRing(
        &self,
        io_ring: ConstPtr<HIORING__>,
        wait_operations: u32,
        milliseconds: u32,
        submitted_entries: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("SubmitIoRing")
    }
    fn TxfGetThreadMiniVersionForCreate(&self, mini_version: MutPtr<u16>) {
        todo!("TxfGetThreadMiniVersionForCreate")
    }
    fn UnlockFile(
        &self,
        h_file: super::super::Foundation::HANDLE,
        dw_file_offset_low: u32,
        dw_file_offset_high: u32,
        n_number_of_bytes_to_unlock_low: u32,
        n_number_of_bytes_to_unlock_high: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("UnlockFile")
    }
    fn UnlockFileEx(
        &self,
        h_file: super::super::Foundation::HANDLE,
        dw_reserved: u32,
        n_number_of_bytes_to_unlock_low: u32,
        n_number_of_bytes_to_unlock_high: u32,
        lp_overlapped: MutPtr<super::super::System::IO::OVERLAPPED>,
    ) -> super::super::Foundation::BOOL {
        todo!("UnlockFileEx")
    }
    fn VerFindFileA(
        &self,
        u_flags: VER_FIND_FILE_FLAGS,
        sz_file_name: PCSTR,
        sz_win_dir: PCSTR,
        sz_app_dir: PCSTR,
        sz_cur_dir: PSTR,
        pu_cur_dir_len: MutPtr<u32>,
        sz_dest_dir: PSTR,
        pu_dest_dir_len: MutPtr<u32>,
    ) -> VER_FIND_FILE_STATUS {
        todo!("VerFindFileA")
    }
    fn VerFindFileW(
        &self,
        u_flags: VER_FIND_FILE_FLAGS,
        sz_file_name: PCWSTR,
        sz_win_dir: PCWSTR,
        sz_app_dir: PCWSTR,
        sz_cur_dir: PWSTR,
        pu_cur_dir_len: MutPtr<u32>,
        sz_dest_dir: PWSTR,
        pu_dest_dir_len: MutPtr<u32>,
    ) -> VER_FIND_FILE_STATUS {
        todo!("VerFindFileW")
    }
    fn VerInstallFileA(
        &self,
        u_flags: VER_INSTALL_FILE_FLAGS,
        sz_src_file_name: PCSTR,
        sz_dest_file_name: PCSTR,
        sz_src_dir: PCSTR,
        sz_dest_dir: PCSTR,
        sz_cur_dir: PCSTR,
        sz_tmp_file: PSTR,
        pu_tmp_file_len: MutPtr<u32>,
    ) -> VER_INSTALL_FILE_STATUS {
        todo!("VerInstallFileA")
    }
    fn VerInstallFileW(
        &self,
        u_flags: VER_INSTALL_FILE_FLAGS,
        sz_src_file_name: PCWSTR,
        sz_dest_file_name: PCWSTR,
        sz_src_dir: PCWSTR,
        sz_dest_dir: PCWSTR,
        sz_cur_dir: PCWSTR,
        sz_tmp_file: PWSTR,
        pu_tmp_file_len: MutPtr<u32>,
    ) -> VER_INSTALL_FILE_STATUS {
        todo!("VerInstallFileW")
    }
    fn VerLanguageNameA(&self, w_lang: u32, sz_lang: PSTR, cch_lang: u32) -> u32 {
        todo!("VerLanguageNameA")
    }
    fn VerLanguageNameW(&self, w_lang: u32, sz_lang: PWSTR, cch_lang: u32) -> u32 {
        todo!("VerLanguageNameW")
    }
    fn VerQueryValueA(
        &self,
        p_block: ConstPtr<::core::ffi::c_void>,
        lp_sub_block: PCSTR,
        lplp_buffer: MutPtr<ConstPtr<::core::ffi::c_void>>,
        pu_len: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("VerQueryValueA")
    }
    fn VerQueryValueW(
        &self,
        p_block: ConstPtr<::core::ffi::c_void>,
        lp_sub_block: PCWSTR,
        lplp_buffer: MutPtr<ConstPtr<::core::ffi::c_void>>,
        pu_len: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("VerQueryValueW")
    }
    fn WofEnumEntries(
        &self,
        volume_name: PCWSTR,
        provider: u32,
        enum_proc: WofEnumEntryProc,
        user_data: ConstPtr<::core::ffi::c_void>,
    ) -> crate::core::HRESULT {
        todo!("WofEnumEntries")
    }
    fn WofFileEnumFiles(
        &self,
        volume_name: PCWSTR,
        algorithm: u32,
        enum_proc: WofEnumFilesProc,
        user_data: ConstPtr<::core::ffi::c_void>,
    ) -> crate::core::HRESULT {
        todo!("WofFileEnumFiles")
    }
    fn WofGetDriverVersion(
        &self,
        file_or_volume_handle: super::super::Foundation::HANDLE,
        provider: u32,
        wof_version: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("WofGetDriverVersion")
    }
    fn WofIsExternalFile(
        &self,
        file_path: PCWSTR,
        is_external_file: MutPtr<super::super::Foundation::BOOL>,
        provider: MutPtr<u32>,
        external_file_info: MutPtr<::core::ffi::c_void>,
        buffer_length: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("WofIsExternalFile")
    }
    fn WofSetFileDataLocation(
        &self,
        file_handle: super::super::Foundation::HANDLE,
        provider: u32,
        external_file_info: ConstPtr<::core::ffi::c_void>,
        length: u32,
    ) -> crate::core::HRESULT {
        todo!("WofSetFileDataLocation")
    }
    fn WofShouldCompressBinaries(
        &self,
        volume: PCWSTR,
        algorithm: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("WofShouldCompressBinaries")
    }
    fn WofWimAddEntry(
        &self,
        volume_name: PCWSTR,
        wim_path: PCWSTR,
        wim_type: u32,
        wim_index: u32,
        data_source_id: MutPtr<i64>,
    ) -> crate::core::HRESULT {
        todo!("WofWimAddEntry")
    }
    fn WofWimEnumFiles(
        &self,
        volume_name: PCWSTR,
        data_source_id: i64,
        enum_proc: WofEnumFilesProc,
        user_data: ConstPtr<::core::ffi::c_void>,
    ) -> crate::core::HRESULT {
        todo!("WofWimEnumFiles")
    }
    fn WofWimRemoveEntry(&self, volume_name: PCWSTR, data_source_id: i64) -> crate::core::HRESULT {
        todo!("WofWimRemoveEntry")
    }
    fn WofWimSuspendEntry(&self, volume_name: PCWSTR, data_source_id: i64) -> crate::core::HRESULT {
        todo!("WofWimSuspendEntry")
    }
    fn WofWimUpdateEntry(
        &self,
        volume_name: PCWSTR,
        data_source_id: i64,
        new_wim_path: PCWSTR,
    ) -> crate::core::HRESULT {
        todo!("WofWimUpdateEntry")
    }
    fn Wow64DisableWow64FsRedirection(
        &self,
        old_value: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> super::super::Foundation::BOOL {
        todo!("Wow64DisableWow64FsRedirection")
    }
    fn Wow64EnableWow64FsRedirection(
        &self,
        wow_64_fs_enable_redirection: super::super::Foundation::BOOLEAN,
    ) -> super::super::Foundation::BOOLEAN {
        todo!("Wow64EnableWow64FsRedirection")
    }
    fn Wow64RevertWow64FsRedirection(
        &self,
        ol_value: ConstPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!("Wow64RevertWow64FsRedirection")
    }
    fn WriteEncryptedFileRaw(
        &self,
        pf_import_callback: PFE_IMPORT_FUNC,
        pv_callback_context: ConstPtr<::core::ffi::c_void>,
        pv_context: ConstPtr<::core::ffi::c_void>,
    ) -> u32 {
        todo!("WriteEncryptedFileRaw")
    }
    fn WriteFile(
        &self,
        h_file: super::super::Foundation::HANDLE,
        lp_buffer: ConstPtr<::core::ffi::c_void>,
        n_number_of_bytes_to_write: u32,
        lp_number_of_bytes_written: MutPtr<u32>,
        lp_overlapped: MutPtr<super::super::System::IO::OVERLAPPED>,
    ) -> super::super::Foundation::BOOL {
        todo!("WriteFile")
    }
    fn WriteFileEx(
        &self,
        h_file: super::super::Foundation::HANDLE,
        lp_buffer: ConstPtr<::core::ffi::c_void>,
        n_number_of_bytes_to_write: u32,
        lp_overlapped: MutPtr<super::super::System::IO::OVERLAPPED>,
        lp_completion_routine: super::super::System::IO::LPOVERLAPPED_COMPLETION_ROUTINE,
    ) -> super::super::Foundation::BOOL {
        todo!("WriteFileEx")
    }
    fn WriteFileGather(
        &self,
        h_file: super::super::Foundation::HANDLE,
        a_segment_array: ConstPtr<FILE_SEGMENT_ELEMENT>,
        n_number_of_bytes_to_write: u32,
        lp_reserved: MutPtr<u32>,
        lp_overlapped: MutPtr<super::super::System::IO::OVERLAPPED>,
    ) -> super::super::Foundation::BOOL {
        todo!("WriteFileGather")
    }
    fn WriteTapemark(
        &self,
        h_device: super::super::Foundation::HANDLE,
        dw_tapemark_type: TAPEMARK_TYPE,
        dw_tapemark_count: u32,
        b_immediate: super::super::Foundation::BOOL,
    ) -> u32 {
        todo!("WriteTapemark")
    }
}
pub fn get_api(ctx: &crate::core::Win32Context) -> std::sync::Arc<dyn Api> {
    ctx.get::<dyn Api>()
}
