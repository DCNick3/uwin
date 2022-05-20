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
pub struct APP_MEMORY_INFORMATION {
    pub AvailableCommit: u64,
    pub PrivateCommitUsage: u64,
    pub PeakPrivateCommitUsage: u64,
    pub TotalCommitUsage: u64,
}
impl ::core::marker::Copy for APP_MEMORY_INFORMATION {}
impl ::core::clone::Clone for APP_MEMORY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for APP_MEMORY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APP_MEMORY_INFORMATION")
            .field("AvailableCommit", &self.AvailableCommit)
            .field("PrivateCommitUsage", &self.PrivateCommitUsage)
            .field("PeakPrivateCommitUsage", &self.PeakPrivateCommitUsage)
            .field("TotalCommitUsage", &self.TotalCommitUsage)
            .finish()
    }
}
impl ::core::cmp::PartialEq for APP_MEMORY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.AvailableCommit == other.AvailableCommit
            && self.PrivateCommitUsage == other.PrivateCommitUsage
            && self.PeakPrivateCommitUsage == other.PeakPrivateCommitUsage
            && self.TotalCommitUsage == other.TotalCommitUsage
    }
}
impl ::core::cmp::Eq for APP_MEMORY_INFORMATION {}
impl FromIntoMemory for APP_MEMORY_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_AvailableCommit = <u64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_PrivateCommitUsage = <u64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_PeakPrivateCommitUsage = <u64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_TotalCommitUsage = <u64 as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        Self {
            AvailableCommit: f_AvailableCommit,
            PrivateCommitUsage: f_PrivateCommitUsage,
            PeakPrivateCommitUsage: f_PeakPrivateCommitUsage,
            TotalCommitUsage: f_TotalCommitUsage,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.AvailableCommit, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.PrivateCommitUsage, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.PeakPrivateCommitUsage, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.TotalCommitUsage, &mut into[24..24 + 8]);
    }
    fn size() -> usize {
        32
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AVRT_PRIORITY(pub i32);
pub const AVRT_PRIORITY_VERYLOW: AVRT_PRIORITY = AVRT_PRIORITY(-2i32);
pub const AVRT_PRIORITY_LOW: AVRT_PRIORITY = AVRT_PRIORITY(-1i32);
pub const AVRT_PRIORITY_NORMAL: AVRT_PRIORITY = AVRT_PRIORITY(0i32);
pub const AVRT_PRIORITY_HIGH: AVRT_PRIORITY = AVRT_PRIORITY(1i32);
pub const AVRT_PRIORITY_CRITICAL: AVRT_PRIORITY = AVRT_PRIORITY(2i32);
impl ::core::marker::Copy for AVRT_PRIORITY {}
impl ::core::clone::Clone for AVRT_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AVRT_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AVRT_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AVRT_PRIORITY").field(&self.0).finish()
    }
}
impl FromIntoMemory for AVRT_PRIORITY {
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
pub struct BoundaryDescriptorHandle(pub PtrDiffRepr);
impl BoundaryDescriptorHandle {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for BoundaryDescriptorHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for BoundaryDescriptorHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for BoundaryDescriptorHandle {}
impl ::core::hash::Hash for BoundaryDescriptorHandle {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for BoundaryDescriptorHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BoundaryDescriptorHandle")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for BoundaryDescriptorHandle {
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
pub const CONDITION_VARIABLE_LOCKMODE_SHARED: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CREATE_EVENT(pub u32);
pub const CREATE_EVENT_INITIAL_SET: CREATE_EVENT = CREATE_EVENT(2u32);
pub const CREATE_EVENT_MANUAL_RESET: CREATE_EVENT = CREATE_EVENT(1u32);
impl ::core::marker::Copy for CREATE_EVENT {}
impl ::core::clone::Clone for CREATE_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CREATE_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CREATE_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREATE_EVENT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CREATE_EVENT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CREATE_EVENT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CREATE_EVENT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CREATE_EVENT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CREATE_EVENT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for CREATE_EVENT {
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
pub const CREATE_MUTEX_INITIAL_OWNER: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CREATE_PROCESS_LOGON_FLAGS(pub u32);
pub const LOGON_WITH_PROFILE: CREATE_PROCESS_LOGON_FLAGS = CREATE_PROCESS_LOGON_FLAGS(1u32);
pub const LOGON_NETCREDENTIALS_ONLY: CREATE_PROCESS_LOGON_FLAGS = CREATE_PROCESS_LOGON_FLAGS(2u32);
impl ::core::marker::Copy for CREATE_PROCESS_LOGON_FLAGS {}
impl ::core::clone::Clone for CREATE_PROCESS_LOGON_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CREATE_PROCESS_LOGON_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CREATE_PROCESS_LOGON_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREATE_PROCESS_LOGON_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for CREATE_PROCESS_LOGON_FLAGS {
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
pub const CREATE_WAITABLE_TIMER_HIGH_RESOLUTION: u32 = 2u32;
pub const CREATE_WAITABLE_TIMER_MANUAL_RESET: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GET_GUI_RESOURCES_FLAGS(pub u32);
pub const GR_GDIOBJECTS: GET_GUI_RESOURCES_FLAGS = GET_GUI_RESOURCES_FLAGS(0u32);
pub const GR_GDIOBJECTS_PEAK: GET_GUI_RESOURCES_FLAGS = GET_GUI_RESOURCES_FLAGS(2u32);
pub const GR_USEROBJECTS: GET_GUI_RESOURCES_FLAGS = GET_GUI_RESOURCES_FLAGS(1u32);
pub const GR_USEROBJECTS_PEAK: GET_GUI_RESOURCES_FLAGS = GET_GUI_RESOURCES_FLAGS(4u32);
impl ::core::marker::Copy for GET_GUI_RESOURCES_FLAGS {}
impl ::core::clone::Clone for GET_GUI_RESOURCES_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_GUI_RESOURCES_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_GUI_RESOURCES_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_GUI_RESOURCES_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for GET_GUI_RESOURCES_FLAGS {
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
pub const INIT_ONCE_ASYNC: u32 = 2u32;
pub const INIT_ONCE_CHECK_ONLY: u32 = 1u32;
pub const INIT_ONCE_CTX_RESERVED_BITS: u32 = 2u32;
pub const INIT_ONCE_INIT_FAILED: u32 = 4u32;
pub struct IO_COUNTERS {
    pub ReadOperationCount: u64,
    pub WriteOperationCount: u64,
    pub OtherOperationCount: u64,
    pub ReadTransferCount: u64,
    pub WriteTransferCount: u64,
    pub OtherTransferCount: u64,
}
impl ::core::marker::Copy for IO_COUNTERS {}
impl ::core::clone::Clone for IO_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IO_COUNTERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IO_COUNTERS")
            .field("ReadOperationCount", &self.ReadOperationCount)
            .field("WriteOperationCount", &self.WriteOperationCount)
            .field("OtherOperationCount", &self.OtherOperationCount)
            .field("ReadTransferCount", &self.ReadTransferCount)
            .field("WriteTransferCount", &self.WriteTransferCount)
            .field("OtherTransferCount", &self.OtherTransferCount)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IO_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        self.ReadOperationCount == other.ReadOperationCount
            && self.WriteOperationCount == other.WriteOperationCount
            && self.OtherOperationCount == other.OtherOperationCount
            && self.ReadTransferCount == other.ReadTransferCount
            && self.WriteTransferCount == other.WriteTransferCount
            && self.OtherTransferCount == other.OtherTransferCount
    }
}
impl ::core::cmp::Eq for IO_COUNTERS {}
impl FromIntoMemory for IO_COUNTERS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_ReadOperationCount = <u64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_WriteOperationCount = <u64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_OtherOperationCount = <u64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_ReadTransferCount = <u64 as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        let f_WriteTransferCount = <u64 as FromIntoMemory>::from_bytes(&from[32..32 + 8]);
        let f_OtherTransferCount = <u64 as FromIntoMemory>::from_bytes(&from[40..40 + 8]);
        Self {
            ReadOperationCount: f_ReadOperationCount,
            WriteOperationCount: f_WriteOperationCount,
            OtherOperationCount: f_OtherOperationCount,
            ReadTransferCount: f_ReadTransferCount,
            WriteTransferCount: f_WriteTransferCount,
            OtherTransferCount: f_OtherTransferCount,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.ReadOperationCount, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.WriteOperationCount, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.OtherOperationCount, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.ReadTransferCount, &mut into[24..24 + 8]);
        FromIntoMemory::into_bytes(self.WriteTransferCount, &mut into[32..32 + 8]);
        FromIntoMemory::into_bytes(self.OtherTransferCount, &mut into[40..40 + 8]);
    }
    fn size() -> usize {
        48
    }
}
pub type LPFIBER_START_ROUTINE = StdCallFnPtr<(MutPtr<::core::ffi::c_void>,), ()>;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LPPROC_THREAD_ATTRIBUTE_LIST(pub MutPtr<::core::ffi::c_void>);
impl LPPROC_THREAD_ATTRIBUTE_LIST {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LPPROC_THREAD_ATTRIBUTE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for LPPROC_THREAD_ATTRIBUTE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for LPPROC_THREAD_ATTRIBUTE_LIST {}
impl ::core::hash::Hash for LPPROC_THREAD_ATTRIBUTE_LIST {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for LPPROC_THREAD_ATTRIBUTE_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LPPROC_THREAD_ATTRIBUTE_LIST")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for LPPROC_THREAD_ATTRIBUTE_LIST {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(
            from,
        ))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<MutPtr<::core::ffi::c_void>>()
    }
}
pub type LPTHREAD_START_ROUTINE = StdCallFnPtr<(MutPtr<::core::ffi::c_void>,), u32>;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MACHINE_ATTRIBUTES(pub u32);
pub const UserEnabled: MACHINE_ATTRIBUTES = MACHINE_ATTRIBUTES(1u32);
pub const KernelEnabled: MACHINE_ATTRIBUTES = MACHINE_ATTRIBUTES(2u32);
pub const Wow64Container: MACHINE_ATTRIBUTES = MACHINE_ATTRIBUTES(4u32);
impl ::core::marker::Copy for MACHINE_ATTRIBUTES {}
impl ::core::clone::Clone for MACHINE_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MACHINE_ATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MACHINE_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MACHINE_ATTRIBUTES").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MACHINE_ATTRIBUTES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MACHINE_ATTRIBUTES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MACHINE_ATTRIBUTES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MACHINE_ATTRIBUTES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MACHINE_ATTRIBUTES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for MACHINE_ATTRIBUTES {
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
pub struct MEMORY_PRIORITY(pub u32);
pub const MEMORY_PRIORITY_VERY_LOW: MEMORY_PRIORITY = MEMORY_PRIORITY(1u32);
pub const MEMORY_PRIORITY_LOW: MEMORY_PRIORITY = MEMORY_PRIORITY(2u32);
pub const MEMORY_PRIORITY_MEDIUM: MEMORY_PRIORITY = MEMORY_PRIORITY(3u32);
pub const MEMORY_PRIORITY_BELOW_NORMAL: MEMORY_PRIORITY = MEMORY_PRIORITY(4u32);
pub const MEMORY_PRIORITY_NORMAL: MEMORY_PRIORITY = MEMORY_PRIORITY(5u32);
impl ::core::marker::Copy for MEMORY_PRIORITY {}
impl ::core::clone::Clone for MEMORY_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MEMORY_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MEMORY_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEMORY_PRIORITY").field(&self.0).finish()
    }
}
impl FromIntoMemory for MEMORY_PRIORITY {
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
pub struct MEMORY_PRIORITY_INFORMATION {
    pub MemoryPriority: MEMORY_PRIORITY,
}
impl ::core::marker::Copy for MEMORY_PRIORITY_INFORMATION {}
impl ::core::clone::Clone for MEMORY_PRIORITY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MEMORY_PRIORITY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEMORY_PRIORITY_INFORMATION")
            .field("MemoryPriority", &self.MemoryPriority)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MEMORY_PRIORITY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.MemoryPriority == other.MemoryPriority
    }
}
impl ::core::cmp::Eq for MEMORY_PRIORITY_INFORMATION {}
impl FromIntoMemory for MEMORY_PRIORITY_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_MemoryPriority = <MEMORY_PRIORITY as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            MemoryPriority: f_MemoryPriority,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.MemoryPriority, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub const MUTEX_MODIFY_STATE: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NamespaceHandle(pub PtrDiffRepr);
impl NamespaceHandle {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NamespaceHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for NamespaceHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for NamespaceHandle {}
impl ::core::hash::Hash for NamespaceHandle {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for NamespaceHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NamespaceHandle").field(&self.0).finish()
    }
}
impl FromIntoMemory for NamespaceHandle {
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
pub struct PEB {
    pub Reserved1: [u8; 2],
    pub BeingDebugged: u8,
    pub Reserved2: [u8; 1],
    pub Reserved3: [MutPtr<::core::ffi::c_void>; 2],
    pub Ldr: MutPtr<PEB_LDR_DATA>,
    pub ProcessParameters: MutPtr<RTL_USER_PROCESS_PARAMETERS>,
    pub Reserved4: [MutPtr<::core::ffi::c_void>; 3],
    pub AtlThunkSListPtr: MutPtr<::core::ffi::c_void>,
    pub Reserved5: MutPtr<::core::ffi::c_void>,
    pub Reserved6: u32,
    pub Reserved7: MutPtr<::core::ffi::c_void>,
    pub Reserved8: u32,
    pub AtlThunkSListPtr32: u32,
    pub Reserved9: [MutPtr<::core::ffi::c_void>; 45],
    pub Reserved10: [u8; 96],
    pub PostProcessInitRoutine: PPS_POST_PROCESS_INIT_ROUTINE,
    pub Reserved11: [u8; 128],
    pub Reserved12: [MutPtr<::core::ffi::c_void>; 1],
    pub SessionId: u32,
}
impl ::core::marker::Copy for PEB {}
impl ::core::clone::Clone for PEB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PEB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEB")
            .field("Reserved1", &self.Reserved1)
            .field("BeingDebugged", &self.BeingDebugged)
            .field("Reserved2", &self.Reserved2)
            .field("Reserved3", &self.Reserved3)
            .field("Ldr", &self.Ldr)
            .field("ProcessParameters", &self.ProcessParameters)
            .field("Reserved4", &self.Reserved4)
            .field("AtlThunkSListPtr", &self.AtlThunkSListPtr)
            .field("Reserved5", &self.Reserved5)
            .field("Reserved6", &self.Reserved6)
            .field("Reserved7", &self.Reserved7)
            .field("Reserved8", &self.Reserved8)
            .field("AtlThunkSListPtr32", &self.AtlThunkSListPtr32)
            .field("Reserved9", &self.Reserved9)
            .field("Reserved10", &self.Reserved10)
            .field("PostProcessInitRoutine", &self.PostProcessInitRoutine)
            .field("Reserved11", &self.Reserved11)
            .field("Reserved12", &self.Reserved12)
            .field("SessionId", &self.SessionId)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PEB {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
            && self.BeingDebugged == other.BeingDebugged
            && self.Reserved2 == other.Reserved2
            && self.Reserved3 == other.Reserved3
            && self.Ldr == other.Ldr
            && self.ProcessParameters == other.ProcessParameters
            && self.Reserved4 == other.Reserved4
            && self.AtlThunkSListPtr == other.AtlThunkSListPtr
            && self.Reserved5 == other.Reserved5
            && self.Reserved6 == other.Reserved6
            && self.Reserved7 == other.Reserved7
            && self.Reserved8 == other.Reserved8
            && self.AtlThunkSListPtr32 == other.AtlThunkSListPtr32
            && self.Reserved9 == other.Reserved9
            && self.Reserved10 == other.Reserved10
            && self.PostProcessInitRoutine == other.PostProcessInitRoutine
            && self.Reserved11 == other.Reserved11
            && self.Reserved12 == other.Reserved12
            && self.SessionId == other.SessionId
    }
}
impl ::core::cmp::Eq for PEB {}
impl FromIntoMemory for PEB {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 472);
        let f_Reserved1 = <[u8; 2] as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_BeingDebugged = <u8 as FromIntoMemory>::from_bytes(&from[2..2 + 1]);
        let f_Reserved2 = <[u8; 1] as FromIntoMemory>::from_bytes(&from[3..3 + 1]);
        let f_Reserved3 =
            <[MutPtr<::core::ffi::c_void>; 2] as FromIntoMemory>::from_bytes(&from[4..4 + 8]);
        let f_Ldr = <MutPtr<PEB_LDR_DATA> as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_ProcessParameters =
            <MutPtr<RTL_USER_PROCESS_PARAMETERS> as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_Reserved4 =
            <[MutPtr<::core::ffi::c_void>; 3] as FromIntoMemory>::from_bytes(&from[20..20 + 12]);
        let f_AtlThunkSListPtr =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_Reserved5 =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_Reserved6 = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_Reserved7 =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_Reserved8 = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_AtlThunkSListPtr32 = <u32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_Reserved9 =
            <[MutPtr<::core::ffi::c_void>; 45] as FromIntoMemory>::from_bytes(&from[56..56 + 180]);
        let f_Reserved10 = <[u8; 96] as FromIntoMemory>::from_bytes(&from[236..236 + 96]);
        let f_PostProcessInitRoutine =
            <PPS_POST_PROCESS_INIT_ROUTINE as FromIntoMemory>::from_bytes(&from[332..332 + 4]);
        let f_Reserved11 = <[u8; 128] as FromIntoMemory>::from_bytes(&from[336..336 + 128]);
        let f_Reserved12 =
            <[MutPtr<::core::ffi::c_void>; 1] as FromIntoMemory>::from_bytes(&from[464..464 + 4]);
        let f_SessionId = <u32 as FromIntoMemory>::from_bytes(&from[468..468 + 4]);
        Self {
            Reserved1: f_Reserved1,
            BeingDebugged: f_BeingDebugged,
            Reserved2: f_Reserved2,
            Reserved3: f_Reserved3,
            Ldr: f_Ldr,
            ProcessParameters: f_ProcessParameters,
            Reserved4: f_Reserved4,
            AtlThunkSListPtr: f_AtlThunkSListPtr,
            Reserved5: f_Reserved5,
            Reserved6: f_Reserved6,
            Reserved7: f_Reserved7,
            Reserved8: f_Reserved8,
            AtlThunkSListPtr32: f_AtlThunkSListPtr32,
            Reserved9: f_Reserved9,
            Reserved10: f_Reserved10,
            PostProcessInitRoutine: f_PostProcessInitRoutine,
            Reserved11: f_Reserved11,
            Reserved12: f_Reserved12,
            SessionId: f_SessionId,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 472);
        FromIntoMemory::into_bytes(self.Reserved1, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.BeingDebugged, &mut into[2..2 + 1]);
        FromIntoMemory::into_bytes(self.Reserved2, &mut into[3..3 + 1]);
        FromIntoMemory::into_bytes(self.Reserved3, &mut into[4..4 + 8]);
        FromIntoMemory::into_bytes(self.Ldr, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.ProcessParameters, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.Reserved4, &mut into[20..20 + 12]);
        FromIntoMemory::into_bytes(self.AtlThunkSListPtr, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.Reserved5, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.Reserved6, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.Reserved7, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.Reserved8, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.AtlThunkSListPtr32, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.Reserved9, &mut into[56..56 + 180]);
        FromIntoMemory::into_bytes(self.Reserved10, &mut into[236..236 + 96]);
        FromIntoMemory::into_bytes(self.PostProcessInitRoutine, &mut into[332..332 + 4]);
        FromIntoMemory::into_bytes(self.Reserved11, &mut into[336..336 + 128]);
        FromIntoMemory::into_bytes(self.Reserved12, &mut into[464..464 + 4]);
        FromIntoMemory::into_bytes(self.SessionId, &mut into[468..468 + 4]);
    }
    fn size() -> usize {
        472
    }
}
pub struct PEB_LDR_DATA {
    pub Reserved1: [u8; 8],
    pub Reserved2: [MutPtr<::core::ffi::c_void>; 3],
    pub InMemoryOrderModuleList: super::Kernel::LIST_ENTRY,
}
impl ::core::marker::Copy for PEB_LDR_DATA {}
impl ::core::clone::Clone for PEB_LDR_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PEB_LDR_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEB_LDR_DATA")
            .field("Reserved1", &self.Reserved1)
            .field("Reserved2", &self.Reserved2)
            .field("InMemoryOrderModuleList", &self.InMemoryOrderModuleList)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PEB_LDR_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
            && self.Reserved2 == other.Reserved2
            && self.InMemoryOrderModuleList == other.InMemoryOrderModuleList
    }
}
impl ::core::cmp::Eq for PEB_LDR_DATA {}
impl FromIntoMemory for PEB_LDR_DATA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_Reserved1 = <[u8; 8] as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_Reserved2 =
            <[MutPtr<::core::ffi::c_void>; 3] as FromIntoMemory>::from_bytes(&from[8..8 + 12]);
        let f_InMemoryOrderModuleList =
            <super::Kernel::LIST_ENTRY as FromIntoMemory>::from_bytes(&from[20..20 + 8]);
        Self {
            Reserved1: f_Reserved1,
            Reserved2: f_Reserved2,
            InMemoryOrderModuleList: f_InMemoryOrderModuleList,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.Reserved1, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.Reserved2, &mut into[8..8 + 12]);
        FromIntoMemory::into_bytes(self.InMemoryOrderModuleList, &mut into[20..20 + 8]);
    }
    fn size() -> usize {
        28
    }
}
pub type PFLS_CALLBACK_FUNCTION = StdCallFnPtr<(ConstPtr<::core::ffi::c_void>,), ()>;
pub type PINIT_ONCE_FN = StdCallFnPtr<
    (
        MutPtr<RTL_RUN_ONCE>,
        MutPtr<::core::ffi::c_void>,
        MutPtr<ConstPtr<::core::ffi::c_void>>,
    ),
    super::super::Foundation::BOOL,
>;
pub const PME_CURRENT_VERSION: u32 = 1u32;
pub const PME_FAILFAST_ON_COMMIT_FAIL_DISABLE: u32 = 0u32;
pub const PME_FAILFAST_ON_COMMIT_FAIL_ENABLE: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct POWER_REQUEST_CONTEXT_FLAGS(pub u32);
pub const POWER_REQUEST_CONTEXT_DETAILED_STRING: POWER_REQUEST_CONTEXT_FLAGS =
    POWER_REQUEST_CONTEXT_FLAGS(2u32);
pub const POWER_REQUEST_CONTEXT_SIMPLE_STRING: POWER_REQUEST_CONTEXT_FLAGS =
    POWER_REQUEST_CONTEXT_FLAGS(1u32);
impl ::core::marker::Copy for POWER_REQUEST_CONTEXT_FLAGS {}
impl ::core::clone::Clone for POWER_REQUEST_CONTEXT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for POWER_REQUEST_CONTEXT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POWER_REQUEST_CONTEXT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POWER_REQUEST_CONTEXT_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for POWER_REQUEST_CONTEXT_FLAGS {
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
pub type PPS_POST_PROCESS_INIT_ROUTINE = StdCallFnPtr<(), ()>;
pub const PRIVATE_NAMESPACE_FLAG_DESTROY: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROCESSINFOCLASS(pub i32);
pub const ProcessBasicInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(0i32);
pub const ProcessDebugPort: PROCESSINFOCLASS = PROCESSINFOCLASS(7i32);
pub const ProcessWow64Information: PROCESSINFOCLASS = PROCESSINFOCLASS(26i32);
pub const ProcessImageFileName: PROCESSINFOCLASS = PROCESSINFOCLASS(27i32);
pub const ProcessBreakOnTermination: PROCESSINFOCLASS = PROCESSINFOCLASS(29i32);
impl ::core::marker::Copy for PROCESSINFOCLASS {}
impl ::core::clone::Clone for PROCESSINFOCLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESSINFOCLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROCESSINFOCLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESSINFOCLASS").field(&self.0).finish()
    }
}
impl FromIntoMemory for PROCESSINFOCLASS {
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
pub struct PROCESSOR_FEATURE_ID(pub u32);
pub const PF_ARM_64BIT_LOADSTORE_ATOMIC: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(25u32);
pub const PF_ARM_DIVIDE_INSTRUCTION_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(24u32);
pub const PF_ARM_EXTERNAL_CACHE_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(26u32);
pub const PF_ARM_FMAC_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(27u32);
pub const PF_ARM_VFP_32_REGISTERS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(18u32);
pub const PF_3DNOW_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(7u32);
pub const PF_CHANNELS_ENABLED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(16u32);
pub const PF_COMPARE_EXCHANGE_DOUBLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(2u32);
pub const PF_COMPARE_EXCHANGE128: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(14u32);
pub const PF_COMPARE64_EXCHANGE128: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(15u32);
pub const PF_FASTFAIL_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(23u32);
pub const PF_FLOATING_POINT_EMULATED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(1u32);
pub const PF_FLOATING_POINT_PRECISION_ERRATA: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(0u32);
pub const PF_MMX_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(3u32);
pub const PF_NX_ENABLED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(12u32);
pub const PF_PAE_ENABLED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(9u32);
pub const PF_RDTSC_INSTRUCTION_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(8u32);
pub const PF_RDWRFSGSBASE_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(22u32);
pub const PF_SECOND_LEVEL_ADDRESS_TRANSLATION: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(20u32);
pub const PF_SSE3_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(13u32);
pub const PF_VIRT_FIRMWARE_ENABLED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(21u32);
pub const PF_XMMI_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(6u32);
pub const PF_XMMI64_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(10u32);
pub const PF_XSAVE_ENABLED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(17u32);
pub const PF_ARM_V8_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(29u32);
pub const PF_ARM_V8_CRYPTO_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID =
    PROCESSOR_FEATURE_ID(30u32);
pub const PF_ARM_V8_CRC32_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID =
    PROCESSOR_FEATURE_ID(31u32);
pub const PF_ARM_V81_ATOMIC_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID =
    PROCESSOR_FEATURE_ID(34u32);
impl ::core::marker::Copy for PROCESSOR_FEATURE_ID {}
impl ::core::clone::Clone for PROCESSOR_FEATURE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESSOR_FEATURE_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROCESSOR_FEATURE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESSOR_FEATURE_ID")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for PROCESSOR_FEATURE_ID {
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
pub struct PROCESS_ACCESS_RIGHTS(pub u32);
pub const PROCESS_TERMINATE: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(1u32);
pub const PROCESS_CREATE_THREAD: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(2u32);
pub const PROCESS_SET_SESSIONID: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(4u32);
pub const PROCESS_VM_OPERATION: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(8u32);
pub const PROCESS_VM_READ: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(16u32);
pub const PROCESS_VM_WRITE: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(32u32);
pub const PROCESS_DUP_HANDLE: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(64u32);
pub const PROCESS_CREATE_PROCESS: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(128u32);
pub const PROCESS_SET_QUOTA: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(256u32);
pub const PROCESS_SET_INFORMATION: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(512u32);
pub const PROCESS_QUERY_INFORMATION: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(1024u32);
pub const PROCESS_SUSPEND_RESUME: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(2048u32);
pub const PROCESS_QUERY_LIMITED_INFORMATION: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(4096u32);
pub const PROCESS_SET_LIMITED_INFORMATION: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(8192u32);
pub const PROCESS_ALL_ACCESS: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(2097151u32);
pub const PROCESS_DELETE: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(65536u32);
pub const PROCESS_READ_CONTROL: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(131072u32);
pub const PROCESS_WRITE_DAC: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(262144u32);
pub const PROCESS_WRITE_OWNER: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(524288u32);
pub const PROCESS_SYNCHRONIZE: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(1048576u32);
pub const PROCESS_STANDARD_RIGHTS_REQUIRED: PROCESS_ACCESS_RIGHTS =
    PROCESS_ACCESS_RIGHTS(983040u32);
impl ::core::marker::Copy for PROCESS_ACCESS_RIGHTS {}
impl ::core::clone::Clone for PROCESS_ACCESS_RIGHTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_ACCESS_RIGHTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROCESS_ACCESS_RIGHTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_ACCESS_RIGHTS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for PROCESS_ACCESS_RIGHTS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROCESS_ACCESS_RIGHTS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROCESS_ACCESS_RIGHTS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROCESS_ACCESS_RIGHTS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROCESS_ACCESS_RIGHTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for PROCESS_ACCESS_RIGHTS {
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
pub struct PROCESS_AFFINITY_AUTO_UPDATE_FLAGS(pub u32);
pub const PROCESS_AFFINITY_DISABLE_AUTO_UPDATE: PROCESS_AFFINITY_AUTO_UPDATE_FLAGS =
    PROCESS_AFFINITY_AUTO_UPDATE_FLAGS(0u32);
pub const PROCESS_AFFINITY_ENABLE_AUTO_UPDATE: PROCESS_AFFINITY_AUTO_UPDATE_FLAGS =
    PROCESS_AFFINITY_AUTO_UPDATE_FLAGS(1u32);
impl ::core::marker::Copy for PROCESS_AFFINITY_AUTO_UPDATE_FLAGS {}
impl ::core::clone::Clone for PROCESS_AFFINITY_AUTO_UPDATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_AFFINITY_AUTO_UPDATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROCESS_AFFINITY_AUTO_UPDATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_AFFINITY_AUTO_UPDATE_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for PROCESS_AFFINITY_AUTO_UPDATE_FLAGS {
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
pub struct PROCESS_BASIC_INFORMATION {
    pub Reserved1: MutPtr<::core::ffi::c_void>,
    pub PebBaseAddress: MutPtr<PEB>,
    pub Reserved2: [MutPtr<::core::ffi::c_void>; 2],
    pub UniqueProcessId: PtrRepr,
    pub Reserved3: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for PROCESS_BASIC_INFORMATION {}
impl ::core::clone::Clone for PROCESS_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_BASIC_INFORMATION")
            .field("Reserved1", &self.Reserved1)
            .field("PebBaseAddress", &self.PebBaseAddress)
            .field("Reserved2", &self.Reserved2)
            .field("UniqueProcessId", &self.UniqueProcessId)
            .field("Reserved3", &self.Reserved3)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PROCESS_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
            && self.PebBaseAddress == other.PebBaseAddress
            && self.Reserved2 == other.Reserved2
            && self.UniqueProcessId == other.UniqueProcessId
            && self.Reserved3 == other.Reserved3
    }
}
impl ::core::cmp::Eq for PROCESS_BASIC_INFORMATION {}
impl FromIntoMemory for PROCESS_BASIC_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_Reserved1 =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_PebBaseAddress = <MutPtr<PEB> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Reserved2 =
            <[MutPtr<::core::ffi::c_void>; 2] as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_UniqueProcessId = <PtrRepr as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_Reserved3 =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            Reserved1: f_Reserved1,
            PebBaseAddress: f_PebBaseAddress,
            Reserved2: f_Reserved2,
            UniqueProcessId: f_UniqueProcessId,
            Reserved3: f_Reserved3,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.Reserved1, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.PebBaseAddress, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Reserved2, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.UniqueProcessId, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.Reserved3, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROCESS_CREATION_FLAGS(pub u32);
pub const DEBUG_PROCESS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(1u32);
pub const DEBUG_ONLY_THIS_PROCESS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(2u32);
pub const CREATE_SUSPENDED: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(4u32);
pub const DETACHED_PROCESS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(8u32);
pub const CREATE_NEW_CONSOLE: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(16u32);
pub const NORMAL_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(32u32);
pub const IDLE_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(64u32);
pub const HIGH_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(128u32);
pub const REALTIME_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(256u32);
pub const CREATE_NEW_PROCESS_GROUP: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(512u32);
pub const CREATE_UNICODE_ENVIRONMENT: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(1024u32);
pub const CREATE_SEPARATE_WOW_VDM: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(2048u32);
pub const CREATE_SHARED_WOW_VDM: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(4096u32);
pub const CREATE_FORCEDOS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(8192u32);
pub const BELOW_NORMAL_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(16384u32);
pub const ABOVE_NORMAL_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(32768u32);
pub const INHERIT_PARENT_AFFINITY: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(65536u32);
pub const INHERIT_CALLER_PRIORITY: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(131072u32);
pub const CREATE_PROTECTED_PROCESS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(262144u32);
pub const EXTENDED_STARTUPINFO_PRESENT: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(524288u32);
pub const PROCESS_MODE_BACKGROUND_BEGIN: PROCESS_CREATION_FLAGS =
    PROCESS_CREATION_FLAGS(1048576u32);
pub const PROCESS_MODE_BACKGROUND_END: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(2097152u32);
pub const CREATE_SECURE_PROCESS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(4194304u32);
pub const CREATE_BREAKAWAY_FROM_JOB: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(16777216u32);
pub const CREATE_PRESERVE_CODE_AUTHZ_LEVEL: PROCESS_CREATION_FLAGS =
    PROCESS_CREATION_FLAGS(33554432u32);
pub const CREATE_DEFAULT_ERROR_MODE: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(67108864u32);
pub const CREATE_NO_WINDOW: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(134217728u32);
pub const PROFILE_USER: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(268435456u32);
pub const PROFILE_KERNEL: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(536870912u32);
pub const PROFILE_SERVER: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(1073741824u32);
pub const CREATE_IGNORE_SYSTEM_DEFAULT: PROCESS_CREATION_FLAGS =
    PROCESS_CREATION_FLAGS(2147483648u32);
impl ::core::marker::Copy for PROCESS_CREATION_FLAGS {}
impl ::core::clone::Clone for PROCESS_CREATION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_CREATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROCESS_CREATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_CREATION_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for PROCESS_CREATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROCESS_CREATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROCESS_CREATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROCESS_CREATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROCESS_CREATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for PROCESS_CREATION_FLAGS {
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
pub struct PROCESS_DEP_FLAGS(pub u32);
pub const PROCESS_DEP_ENABLE: PROCESS_DEP_FLAGS = PROCESS_DEP_FLAGS(1u32);
pub const PROCESS_DEP_DISABLE_ATL_THUNK_EMULATION: PROCESS_DEP_FLAGS = PROCESS_DEP_FLAGS(2u32);
pub const PROCESS_DEP_NONE: PROCESS_DEP_FLAGS = PROCESS_DEP_FLAGS(0u32);
impl ::core::marker::Copy for PROCESS_DEP_FLAGS {}
impl ::core::clone::Clone for PROCESS_DEP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_DEP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROCESS_DEP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_DEP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PROCESS_DEP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROCESS_DEP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROCESS_DEP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROCESS_DEP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROCESS_DEP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for PROCESS_DEP_FLAGS {
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
pub struct PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    pub TargetAddress: PtrRepr,
    pub Flags: PtrRepr,
}
impl ::core::marker::Copy for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {}
impl ::core::clone::Clone for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_DYNAMIC_EH_CONTINUATION_TARGET")
            .field("TargetAddress", &self.TargetAddress)
            .field("Flags", &self.Flags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    fn eq(&self, other: &Self) -> bool {
        self.TargetAddress == other.TargetAddress && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {}
impl FromIntoMemory for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_TargetAddress = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Flags = <PtrRepr as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            TargetAddress: f_TargetAddress,
            Flags: f_Flags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.TargetAddress, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    pub NumberOfTargets: u16,
    pub Reserved: u16,
    pub Reserved2: u32,
    pub Targets: MutPtr<PROCESS_DYNAMIC_EH_CONTINUATION_TARGET>,
}
impl ::core::marker::Copy for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {}
impl ::core::clone::Clone for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION")
            .field("NumberOfTargets", &self.NumberOfTargets)
            .field("Reserved", &self.Reserved)
            .field("Reserved2", &self.Reserved2)
            .field("Targets", &self.Targets)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfTargets == other.NumberOfTargets
            && self.Reserved == other.Reserved
            && self.Reserved2 == other.Reserved2
            && self.Targets == other.Targets
    }
}
impl ::core::cmp::Eq for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {}
impl FromIntoMemory for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_NumberOfTargets = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_Reserved = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_Reserved2 = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Targets =
            <MutPtr<PROCESS_DYNAMIC_EH_CONTINUATION_TARGET> as FromIntoMemory>::from_bytes(
                &from[8..8 + 4],
            );
        Self {
            NumberOfTargets: f_NumberOfTargets,
            Reserved: f_Reserved,
            Reserved2: f_Reserved2,
            Targets: f_Targets,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.NumberOfTargets, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.Reserved2, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Targets, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    pub BaseAddress: PtrRepr,
    pub Size: PtrRepr,
    pub Flags: u32,
}
impl ::core::marker::Copy for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {}
impl ::core::clone::Clone for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE")
            .field("BaseAddress", &self.BaseAddress)
            .field("Size", &self.Size)
            .field("Flags", &self.Flags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.BaseAddress == other.BaseAddress
            && self.Size == other.Size
            && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {}
impl FromIntoMemory for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_BaseAddress = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Size = <PtrRepr as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Flags = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            BaseAddress: f_BaseAddress,
            Size: f_Size,
            Flags: f_Flags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.BaseAddress, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Size, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    pub NumberOfRanges: u16,
    pub Reserved: u16,
    pub Reserved2: u32,
    pub Ranges: MutPtr<PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE>,
}
impl ::core::marker::Copy for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {}
impl ::core::clone::Clone for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION")
            .field("NumberOfRanges", &self.NumberOfRanges)
            .field("Reserved", &self.Reserved)
            .field("Reserved2", &self.Reserved2)
            .field("Ranges", &self.Ranges)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfRanges == other.NumberOfRanges
            && self.Reserved == other.Reserved
            && self.Reserved2 == other.Reserved2
            && self.Ranges == other.Ranges
    }
}
impl ::core::cmp::Eq for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {}
impl FromIntoMemory for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_NumberOfRanges = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_Reserved = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_Reserved2 = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Ranges =
            <MutPtr<PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE> as FromIntoMemory>::from_bytes(
                &from[8..8 + 4],
            );
        Self {
            NumberOfRanges: f_NumberOfRanges,
            Reserved: f_Reserved,
            Reserved2: f_Reserved2,
            Ranges: f_Ranges,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.NumberOfRanges, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.Reserved2, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Ranges, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct PROCESS_INFORMATION {
    pub hProcess: super::super::Foundation::HANDLE,
    pub hThread: super::super::Foundation::HANDLE,
    pub dwProcessId: u32,
    pub dwThreadId: u32,
}
impl ::core::marker::Copy for PROCESS_INFORMATION {}
impl ::core::clone::Clone for PROCESS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_INFORMATION")
            .field("hProcess", &self.hProcess)
            .field("hThread", &self.hThread)
            .field("dwProcessId", &self.dwProcessId)
            .field("dwThreadId", &self.dwThreadId)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PROCESS_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.hProcess == other.hProcess
            && self.hThread == other.hThread
            && self.dwProcessId == other.dwProcessId
            && self.dwThreadId == other.dwThreadId
    }
}
impl ::core::cmp::Eq for PROCESS_INFORMATION {}
impl FromIntoMemory for PROCESS_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_hProcess =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_hThread =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwProcessId = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwThreadId = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            hProcess: f_hProcess,
            hThread: f_hThread,
            dwProcessId: f_dwProcessId,
            dwThreadId: f_dwThreadId,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.hProcess, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.hThread, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwProcessId, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwThreadId, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROCESS_INFORMATION_CLASS(pub i32);
pub const ProcessMemoryPriority: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(0i32);
pub const ProcessMemoryExhaustionInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(1i32);
pub const ProcessAppMemoryInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(2i32);
pub const ProcessInPrivateInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(3i32);
pub const ProcessPowerThrottling: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(4i32);
pub const ProcessReservedValue1: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(5i32);
pub const ProcessTelemetryCoverageInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(6i32);
pub const ProcessProtectionLevelInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(7i32);
pub const ProcessLeapSecondInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(8i32);
pub const ProcessMachineTypeInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(9i32);
pub const ProcessInformationClassMax: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(10i32);
impl ::core::marker::Copy for PROCESS_INFORMATION_CLASS {}
impl ::core::clone::Clone for PROCESS_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROCESS_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_INFORMATION_CLASS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for PROCESS_INFORMATION_CLASS {
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
pub struct PROCESS_LEAP_SECOND_INFO {
    pub Flags: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PROCESS_LEAP_SECOND_INFO {}
impl ::core::clone::Clone for PROCESS_LEAP_SECOND_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_LEAP_SECOND_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_LEAP_SECOND_INFO")
            .field("Flags", &self.Flags)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PROCESS_LEAP_SECOND_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for PROCESS_LEAP_SECOND_INFO {}
impl FromIntoMemory for PROCESS_LEAP_SECOND_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Flags = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Reserved = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            Flags: f_Flags,
            Reserved: f_Reserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Flags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub const PROCESS_LEAP_SECOND_INFO_FLAG_ENABLE_SIXTY_SECOND: u32 = 1u32;
pub const PROCESS_LEAP_SECOND_INFO_VALID_FLAGS: u32 = 1u32;
pub struct PROCESS_MACHINE_INFORMATION {
    pub ProcessMachine: u16,
    pub Res0: u16,
    pub MachineAttributes: MACHINE_ATTRIBUTES,
}
impl ::core::marker::Copy for PROCESS_MACHINE_INFORMATION {}
impl ::core::clone::Clone for PROCESS_MACHINE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_MACHINE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_MACHINE_INFORMATION")
            .field("ProcessMachine", &self.ProcessMachine)
            .field("Res0", &self.Res0)
            .field("MachineAttributes", &self.MachineAttributes)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PROCESS_MACHINE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ProcessMachine == other.ProcessMachine
            && self.Res0 == other.Res0
            && self.MachineAttributes == other.MachineAttributes
    }
}
impl ::core::cmp::Eq for PROCESS_MACHINE_INFORMATION {}
impl FromIntoMemory for PROCESS_MACHINE_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_ProcessMachine = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_Res0 = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_MachineAttributes =
            <MACHINE_ATTRIBUTES as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            ProcessMachine: f_ProcessMachine,
            Res0: f_Res0,
            MachineAttributes: f_MachineAttributes,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.ProcessMachine, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.Res0, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.MachineAttributes, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct PROCESS_MEMORY_EXHAUSTION_INFO {
    pub Version: u16,
    pub Reserved: u16,
    pub Type: PROCESS_MEMORY_EXHAUSTION_TYPE,
    pub Value: PtrRepr,
}
impl ::core::marker::Copy for PROCESS_MEMORY_EXHAUSTION_INFO {}
impl ::core::clone::Clone for PROCESS_MEMORY_EXHAUSTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_MEMORY_EXHAUSTION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_MEMORY_EXHAUSTION_INFO")
            .field("Version", &self.Version)
            .field("Reserved", &self.Reserved)
            .field("Type", &self.Type)
            .field("Value", &self.Value)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PROCESS_MEMORY_EXHAUSTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Reserved == other.Reserved
            && self.Type == other.Type
            && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for PROCESS_MEMORY_EXHAUSTION_INFO {}
impl FromIntoMemory for PROCESS_MEMORY_EXHAUSTION_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Version = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_Reserved = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_Type =
            <PROCESS_MEMORY_EXHAUSTION_TYPE as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Value = <PtrRepr as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            Version: f_Version,
            Reserved: f_Reserved,
            Type: f_Type,
            Value: f_Value,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Version, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.Type, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Value, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROCESS_MEMORY_EXHAUSTION_TYPE(pub i32);
pub const PMETypeFailFastOnCommitFailure: PROCESS_MEMORY_EXHAUSTION_TYPE =
    PROCESS_MEMORY_EXHAUSTION_TYPE(0i32);
pub const PMETypeMax: PROCESS_MEMORY_EXHAUSTION_TYPE = PROCESS_MEMORY_EXHAUSTION_TYPE(1i32);
impl ::core::marker::Copy for PROCESS_MEMORY_EXHAUSTION_TYPE {}
impl ::core::clone::Clone for PROCESS_MEMORY_EXHAUSTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_MEMORY_EXHAUSTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROCESS_MEMORY_EXHAUSTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_MEMORY_EXHAUSTION_TYPE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for PROCESS_MEMORY_EXHAUSTION_TYPE {
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
pub struct PROCESS_MITIGATION_POLICY(pub i32);
pub const ProcessDEPPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(0i32);
pub const ProcessASLRPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(1i32);
pub const ProcessDynamicCodePolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(2i32);
pub const ProcessStrictHandleCheckPolicy: PROCESS_MITIGATION_POLICY =
    PROCESS_MITIGATION_POLICY(3i32);
pub const ProcessSystemCallDisablePolicy: PROCESS_MITIGATION_POLICY =
    PROCESS_MITIGATION_POLICY(4i32);
pub const ProcessMitigationOptionsMask: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(5i32);
pub const ProcessExtensionPointDisablePolicy: PROCESS_MITIGATION_POLICY =
    PROCESS_MITIGATION_POLICY(6i32);
pub const ProcessControlFlowGuardPolicy: PROCESS_MITIGATION_POLICY =
    PROCESS_MITIGATION_POLICY(7i32);
pub const ProcessSignaturePolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(8i32);
pub const ProcessFontDisablePolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(9i32);
pub const ProcessImageLoadPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(10i32);
pub const ProcessSystemCallFilterPolicy: PROCESS_MITIGATION_POLICY =
    PROCESS_MITIGATION_POLICY(11i32);
pub const ProcessPayloadRestrictionPolicy: PROCESS_MITIGATION_POLICY =
    PROCESS_MITIGATION_POLICY(12i32);
pub const ProcessChildProcessPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(13i32);
pub const ProcessSideChannelIsolationPolicy: PROCESS_MITIGATION_POLICY =
    PROCESS_MITIGATION_POLICY(14i32);
pub const ProcessUserShadowStackPolicy: PROCESS_MITIGATION_POLICY =
    PROCESS_MITIGATION_POLICY(15i32);
pub const ProcessRedirectionTrustPolicy: PROCESS_MITIGATION_POLICY =
    PROCESS_MITIGATION_POLICY(16i32);
pub const MaxProcessMitigationPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(17i32);
impl ::core::marker::Copy for PROCESS_MITIGATION_POLICY {}
impl ::core::clone::Clone for PROCESS_MITIGATION_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_MITIGATION_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROCESS_MITIGATION_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_MITIGATION_POLICY")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for PROCESS_MITIGATION_POLICY {
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
pub struct PROCESS_NAME_FORMAT(pub u32);
pub const PROCESS_NAME_WIN32: PROCESS_NAME_FORMAT = PROCESS_NAME_FORMAT(0u32);
pub const PROCESS_NAME_NATIVE: PROCESS_NAME_FORMAT = PROCESS_NAME_FORMAT(1u32);
impl ::core::marker::Copy for PROCESS_NAME_FORMAT {}
impl ::core::clone::Clone for PROCESS_NAME_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_NAME_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROCESS_NAME_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_NAME_FORMAT").field(&self.0).finish()
    }
}
impl FromIntoMemory for PROCESS_NAME_FORMAT {
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
pub const PROCESS_POWER_THROTTLING_CURRENT_VERSION: u32 = 1u32;
pub const PROCESS_POWER_THROTTLING_EXECUTION_SPEED: u32 = 1u32;
pub const PROCESS_POWER_THROTTLING_IGNORE_TIMER_RESOLUTION: u32 = 4u32;
pub struct PROCESS_POWER_THROTTLING_STATE {
    pub Version: u32,
    pub ControlMask: u32,
    pub StateMask: u32,
}
impl ::core::marker::Copy for PROCESS_POWER_THROTTLING_STATE {}
impl ::core::clone::Clone for PROCESS_POWER_THROTTLING_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_POWER_THROTTLING_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_POWER_THROTTLING_STATE")
            .field("Version", &self.Version)
            .field("ControlMask", &self.ControlMask)
            .field("StateMask", &self.StateMask)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PROCESS_POWER_THROTTLING_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.ControlMask == other.ControlMask
            && self.StateMask == other.StateMask
    }
}
impl ::core::cmp::Eq for PROCESS_POWER_THROTTLING_STATE {}
impl FromIntoMemory for PROCESS_POWER_THROTTLING_STATE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Version = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ControlMask = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_StateMask = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            Version: f_Version,
            ControlMask: f_ControlMask,
            StateMask: f_StateMask,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Version, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ControlMask, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.StateMask, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROCESS_PROTECTION_LEVEL(pub u32);
pub const PROTECTION_LEVEL_WINTCB_LIGHT: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(0u32);
pub const PROTECTION_LEVEL_WINDOWS: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(1u32);
pub const PROTECTION_LEVEL_WINDOWS_LIGHT: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(2u32);
pub const PROTECTION_LEVEL_ANTIMALWARE_LIGHT: PROCESS_PROTECTION_LEVEL =
    PROCESS_PROTECTION_LEVEL(3u32);
pub const PROTECTION_LEVEL_LSA_LIGHT: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(4u32);
pub const PROTECTION_LEVEL_WINTCB: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(5u32);
pub const PROTECTION_LEVEL_CODEGEN_LIGHT: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(6u32);
pub const PROTECTION_LEVEL_AUTHENTICODE: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(7u32);
pub const PROTECTION_LEVEL_PPL_APP: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(8u32);
pub const PROTECTION_LEVEL_NONE: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(4294967294u32);
impl ::core::marker::Copy for PROCESS_PROTECTION_LEVEL {}
impl ::core::clone::Clone for PROCESS_PROTECTION_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_PROTECTION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROCESS_PROTECTION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_PROTECTION_LEVEL")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for PROCESS_PROTECTION_LEVEL {
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
pub struct PROCESS_PROTECTION_LEVEL_INFORMATION {
    pub ProtectionLevel: PROCESS_PROTECTION_LEVEL,
}
impl ::core::marker::Copy for PROCESS_PROTECTION_LEVEL_INFORMATION {}
impl ::core::clone::Clone for PROCESS_PROTECTION_LEVEL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_PROTECTION_LEVEL_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_PROTECTION_LEVEL_INFORMATION")
            .field("ProtectionLevel", &self.ProtectionLevel)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PROCESS_PROTECTION_LEVEL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ProtectionLevel == other.ProtectionLevel
    }
}
impl ::core::cmp::Eq for PROCESS_PROTECTION_LEVEL_INFORMATION {}
impl FromIntoMemory for PROCESS_PROTECTION_LEVEL_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_ProtectionLevel =
            <PROCESS_PROTECTION_LEVEL as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            ProtectionLevel: f_ProtectionLevel,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.ProtectionLevel, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub const PROC_THREAD_ATTRIBUTE_ALL_APPLICATION_PACKAGES_POLICY: u32 = 131087u32;
pub const PROC_THREAD_ATTRIBUTE_CHILD_PROCESS_POLICY: u32 = 131086u32;
pub const PROC_THREAD_ATTRIBUTE_COMPONENT_FILTER: u32 = 131098u32;
pub const PROC_THREAD_ATTRIBUTE_DESKTOP_APP_POLICY: u32 = 131090u32;
pub const PROC_THREAD_ATTRIBUTE_ENABLE_OPTIONAL_XSTATE_FEATURES: u32 = 196635u32;
pub const PROC_THREAD_ATTRIBUTE_GROUP_AFFINITY: u32 = 196611u32;
pub const PROC_THREAD_ATTRIBUTE_HANDLE_LIST: u32 = 131074u32;
pub const PROC_THREAD_ATTRIBUTE_IDEAL_PROCESSOR: u32 = 196613u32;
pub const PROC_THREAD_ATTRIBUTE_JOB_LIST: u32 = 131085u32;
pub const PROC_THREAD_ATTRIBUTE_MACHINE_TYPE: u32 = 131097u32;
pub const PROC_THREAD_ATTRIBUTE_MITIGATION_AUDIT_POLICY: u32 = 131096u32;
pub const PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY: u32 = 131079u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROC_THREAD_ATTRIBUTE_NUM(pub u32);
pub const ProcThreadAttributeParentProcess: PROC_THREAD_ATTRIBUTE_NUM =
    PROC_THREAD_ATTRIBUTE_NUM(0u32);
pub const ProcThreadAttributeHandleList: PROC_THREAD_ATTRIBUTE_NUM =
    PROC_THREAD_ATTRIBUTE_NUM(2u32);
pub const ProcThreadAttributeGroupAffinity: PROC_THREAD_ATTRIBUTE_NUM =
    PROC_THREAD_ATTRIBUTE_NUM(3u32);
pub const ProcThreadAttributePreferredNode: PROC_THREAD_ATTRIBUTE_NUM =
    PROC_THREAD_ATTRIBUTE_NUM(4u32);
pub const ProcThreadAttributeIdealProcessor: PROC_THREAD_ATTRIBUTE_NUM =
    PROC_THREAD_ATTRIBUTE_NUM(5u32);
pub const ProcThreadAttributeUmsThread: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(6u32);
pub const ProcThreadAttributeMitigationPolicy: PROC_THREAD_ATTRIBUTE_NUM =
    PROC_THREAD_ATTRIBUTE_NUM(7u32);
pub const ProcThreadAttributeSecurityCapabilities: PROC_THREAD_ATTRIBUTE_NUM =
    PROC_THREAD_ATTRIBUTE_NUM(9u32);
pub const ProcThreadAttributeProtectionLevel: PROC_THREAD_ATTRIBUTE_NUM =
    PROC_THREAD_ATTRIBUTE_NUM(11u32);
pub const ProcThreadAttributeJobList: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(13u32);
pub const ProcThreadAttributeChildProcessPolicy: PROC_THREAD_ATTRIBUTE_NUM =
    PROC_THREAD_ATTRIBUTE_NUM(14u32);
pub const ProcThreadAttributeAllApplicationPackagesPolicy: PROC_THREAD_ATTRIBUTE_NUM =
    PROC_THREAD_ATTRIBUTE_NUM(15u32);
pub const ProcThreadAttributeWin32kFilter: PROC_THREAD_ATTRIBUTE_NUM =
    PROC_THREAD_ATTRIBUTE_NUM(16u32);
pub const ProcThreadAttributeSafeOpenPromptOriginClaim: PROC_THREAD_ATTRIBUTE_NUM =
    PROC_THREAD_ATTRIBUTE_NUM(17u32);
pub const ProcThreadAttributeDesktopAppPolicy: PROC_THREAD_ATTRIBUTE_NUM =
    PROC_THREAD_ATTRIBUTE_NUM(18u32);
pub const ProcThreadAttributePseudoConsole: PROC_THREAD_ATTRIBUTE_NUM =
    PROC_THREAD_ATTRIBUTE_NUM(22u32);
pub const ProcThreadAttributeMitigationAuditPolicy: PROC_THREAD_ATTRIBUTE_NUM =
    PROC_THREAD_ATTRIBUTE_NUM(24u32);
pub const ProcThreadAttributeMachineType: PROC_THREAD_ATTRIBUTE_NUM =
    PROC_THREAD_ATTRIBUTE_NUM(25u32);
pub const ProcThreadAttributeComponentFilter: PROC_THREAD_ATTRIBUTE_NUM =
    PROC_THREAD_ATTRIBUTE_NUM(26u32);
pub const ProcThreadAttributeEnableOptionalXStateFeatures: PROC_THREAD_ATTRIBUTE_NUM =
    PROC_THREAD_ATTRIBUTE_NUM(27u32);
impl ::core::marker::Copy for PROC_THREAD_ATTRIBUTE_NUM {}
impl ::core::clone::Clone for PROC_THREAD_ATTRIBUTE_NUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROC_THREAD_ATTRIBUTE_NUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROC_THREAD_ATTRIBUTE_NUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROC_THREAD_ATTRIBUTE_NUM")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for PROC_THREAD_ATTRIBUTE_NUM {
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
pub const PROC_THREAD_ATTRIBUTE_PARENT_PROCESS: u32 = 131072u32;
pub const PROC_THREAD_ATTRIBUTE_PREFERRED_NODE: u32 = 131076u32;
pub const PROC_THREAD_ATTRIBUTE_PROTECTION_LEVEL: u32 = 131083u32;
pub const PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE: u32 = 131094u32;
pub const PROC_THREAD_ATTRIBUTE_REPLACE_VALUE: u32 = 1u32;
pub const PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES: u32 = 131081u32;
pub const PROC_THREAD_ATTRIBUTE_UMS_THREAD: u32 = 196614u32;
pub const PROC_THREAD_ATTRIBUTE_WIN32K_FILTER: u32 = 131088u32;
pub type PRTL_UMS_SCHEDULER_ENTRY_POINT = StdCallFnPtr<
    (
        super::SystemServices::RTL_UMS_SCHEDULER_REASON,
        PtrRepr,
        ConstPtr<::core::ffi::c_void>,
    ),
    (),
>;
pub type PTIMERAPCROUTINE = StdCallFnPtr<(ConstPtr<::core::ffi::c_void>, u32, u32), ()>;
pub type PTP_CLEANUP_GROUP_CANCEL_CALLBACK =
    StdCallFnPtr<(MutPtr<::core::ffi::c_void>, MutPtr<::core::ffi::c_void>), ()>;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PTP_POOL(pub PtrDiffRepr);
impl PTP_POOL {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PTP_POOL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for PTP_POOL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for PTP_POOL {}
impl ::core::hash::Hash for PTP_POOL {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for PTP_POOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PTP_POOL").field(&self.0).finish()
    }
}
impl FromIntoMemory for PTP_POOL {
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
pub type PTP_SIMPLE_CALLBACK =
    StdCallFnPtr<(MutPtr<TP_CALLBACK_INSTANCE>, MutPtr<::core::ffi::c_void>), ()>;
pub type PTP_TIMER_CALLBACK = StdCallFnPtr<
    (
        MutPtr<TP_CALLBACK_INSTANCE>,
        MutPtr<::core::ffi::c_void>,
        MutPtr<TP_TIMER>,
    ),
    (),
>;
pub type PTP_WAIT_CALLBACK = StdCallFnPtr<
    (
        MutPtr<TP_CALLBACK_INSTANCE>,
        MutPtr<::core::ffi::c_void>,
        MutPtr<TP_WAIT>,
        u32,
    ),
    (),
>;
pub type PTP_WIN32_IO_CALLBACK = StdCallFnPtr<
    (
        MutPtr<TP_CALLBACK_INSTANCE>,
        MutPtr<::core::ffi::c_void>,
        MutPtr<::core::ffi::c_void>,
        u32,
        PtrRepr,
        MutPtr<TP_IO>,
    ),
    (),
>;
pub type PTP_WORK_CALLBACK = StdCallFnPtr<
    (
        MutPtr<TP_CALLBACK_INSTANCE>,
        MutPtr<::core::ffi::c_void>,
        MutPtr<TP_WORK>,
    ),
    (),
>;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct QUEUE_USER_APC_FLAGS(pub i32);
pub const QUEUE_USER_APC_FLAGS_NONE: QUEUE_USER_APC_FLAGS = QUEUE_USER_APC_FLAGS(0i32);
pub const QUEUE_USER_APC_FLAGS_SPECIAL_USER_APC: QUEUE_USER_APC_FLAGS = QUEUE_USER_APC_FLAGS(1i32);
impl ::core::marker::Copy for QUEUE_USER_APC_FLAGS {}
impl ::core::clone::Clone for QUEUE_USER_APC_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for QUEUE_USER_APC_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for QUEUE_USER_APC_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUEUE_USER_APC_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for QUEUE_USER_APC_FLAGS {
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
pub struct REASON_CONTEXT {
    pub Version: u32,
    pub Flags: POWER_REQUEST_CONTEXT_FLAGS,
    pub Reason: REASON_CONTEXT_0,
}
impl ::core::marker::Copy for REASON_CONTEXT {}
impl ::core::clone::Clone for REASON_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for REASON_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Flags == other.Flags && self.Reason == other.Reason
    }
}
impl ::core::cmp::Eq for REASON_CONTEXT {}
impl FromIntoMemory for REASON_CONTEXT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Version = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Flags = <POWER_REQUEST_CONTEXT_FLAGS as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Reason = <REASON_CONTEXT_0 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            Version: f_Version,
            Flags: f_Flags,
            Reason: f_Reason,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Version, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Reason, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct REASON_CONTEXT_0 {
    data: [u8; 4],
}
impl ::core::marker::Copy for REASON_CONTEXT_0 {}
impl ::core::clone::Clone for REASON_CONTEXT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for REASON_CONTEXT_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for REASON_CONTEXT_0 {}
impl FromIntoMemory for REASON_CONTEXT_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 4];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        4
    }
}
pub struct REASON_CONTEXT_0_0 {
    pub LocalizedReasonModule: super::super::Foundation::HINSTANCE,
    pub LocalizedReasonId: u32,
    pub ReasonStringCount: u32,
    pub ReasonStrings: MutPtr<PWSTR>,
}
impl ::core::marker::Copy for REASON_CONTEXT_0_0 {}
impl ::core::clone::Clone for REASON_CONTEXT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REASON_CONTEXT_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REASON_CONTEXT_0_0")
            .field("LocalizedReasonModule", &self.LocalizedReasonModule)
            .field("LocalizedReasonId", &self.LocalizedReasonId)
            .field("ReasonStringCount", &self.ReasonStringCount)
            .field("ReasonStrings", &self.ReasonStrings)
            .finish()
    }
}
impl ::core::cmp::PartialEq for REASON_CONTEXT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.LocalizedReasonModule == other.LocalizedReasonModule
            && self.LocalizedReasonId == other.LocalizedReasonId
            && self.ReasonStringCount == other.ReasonStringCount
            && self.ReasonStrings == other.ReasonStrings
    }
}
impl ::core::cmp::Eq for REASON_CONTEXT_0_0 {}
impl FromIntoMemory for REASON_CONTEXT_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_LocalizedReasonModule =
            <super::super::Foundation::HINSTANCE as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_LocalizedReasonId = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_ReasonStringCount = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ReasonStrings = <MutPtr<PWSTR> as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            LocalizedReasonModule: f_LocalizedReasonModule,
            LocalizedReasonId: f_LocalizedReasonId,
            ReasonStringCount: f_ReasonStringCount,
            ReasonStrings: f_ReasonStrings,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.LocalizedReasonModule, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.LocalizedReasonId, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.ReasonStringCount, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ReasonStrings, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct RTL_BARRIER {
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: [PtrRepr; 2],
    pub Reserved4: u32,
    pub Reserved5: u32,
}
impl ::core::marker::Copy for RTL_BARRIER {}
impl ::core::clone::Clone for RTL_BARRIER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RTL_BARRIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTL_BARRIER")
            .field("Reserved1", &self.Reserved1)
            .field("Reserved2", &self.Reserved2)
            .field("Reserved3", &self.Reserved3)
            .field("Reserved4", &self.Reserved4)
            .field("Reserved5", &self.Reserved5)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RTL_BARRIER {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
            && self.Reserved2 == other.Reserved2
            && self.Reserved3 == other.Reserved3
            && self.Reserved4 == other.Reserved4
            && self.Reserved5 == other.Reserved5
    }
}
impl ::core::cmp::Eq for RTL_BARRIER {}
impl FromIntoMemory for RTL_BARRIER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_Reserved1 = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Reserved2 = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Reserved3 = <[PtrRepr; 2] as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_Reserved4 = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_Reserved5 = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            Reserved1: f_Reserved1,
            Reserved2: f_Reserved2,
            Reserved3: f_Reserved3,
            Reserved4: f_Reserved4,
            Reserved5: f_Reserved5,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.Reserved1, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Reserved2, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Reserved3, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.Reserved4, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.Reserved5, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct RTL_CONDITION_VARIABLE {
    pub Ptr: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for RTL_CONDITION_VARIABLE {}
impl ::core::clone::Clone for RTL_CONDITION_VARIABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RTL_CONDITION_VARIABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTL_CONDITION_VARIABLE")
            .field("Ptr", &self.Ptr)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RTL_CONDITION_VARIABLE {
    fn eq(&self, other: &Self) -> bool {
        self.Ptr == other.Ptr
    }
}
impl ::core::cmp::Eq for RTL_CONDITION_VARIABLE {}
impl FromIntoMemory for RTL_CONDITION_VARIABLE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_Ptr = <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self { Ptr: f_Ptr }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.Ptr, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct RTL_CRITICAL_SECTION {
    pub DebugInfo: MutPtr<RTL_CRITICAL_SECTION_DEBUG>,
    pub LockCount: i32,
    pub RecursionCount: i32,
    pub OwningThread: super::super::Foundation::HANDLE,
    pub LockSemaphore: super::super::Foundation::HANDLE,
    pub SpinCount: PtrRepr,
}
impl ::core::marker::Copy for RTL_CRITICAL_SECTION {}
impl ::core::clone::Clone for RTL_CRITICAL_SECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RTL_CRITICAL_SECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTL_CRITICAL_SECTION")
            .field("DebugInfo", &self.DebugInfo)
            .field("LockCount", &self.LockCount)
            .field("RecursionCount", &self.RecursionCount)
            .field("OwningThread", &self.OwningThread)
            .field("LockSemaphore", &self.LockSemaphore)
            .field("SpinCount", &self.SpinCount)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RTL_CRITICAL_SECTION {
    fn eq(&self, other: &Self) -> bool {
        self.DebugInfo == other.DebugInfo
            && self.LockCount == other.LockCount
            && self.RecursionCount == other.RecursionCount
            && self.OwningThread == other.OwningThread
            && self.LockSemaphore == other.LockSemaphore
            && self.SpinCount == other.SpinCount
    }
}
impl ::core::cmp::Eq for RTL_CRITICAL_SECTION {}
impl FromIntoMemory for RTL_CRITICAL_SECTION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_DebugInfo =
            <MutPtr<RTL_CRITICAL_SECTION_DEBUG> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_LockCount = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_RecursionCount = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_OwningThread =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_LockSemaphore =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_SpinCount = <PtrRepr as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            DebugInfo: f_DebugInfo,
            LockCount: f_LockCount,
            RecursionCount: f_RecursionCount,
            OwningThread: f_OwningThread,
            LockSemaphore: f_LockSemaphore,
            SpinCount: f_SpinCount,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.DebugInfo, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.LockCount, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.RecursionCount, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.OwningThread, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.LockSemaphore, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.SpinCount, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct RTL_CRITICAL_SECTION_DEBUG {
    pub Type: u16,
    pub CreatorBackTraceIndex: u16,
    pub CriticalSection: MutPtr<RTL_CRITICAL_SECTION>,
    pub ProcessLocksList: super::Kernel::LIST_ENTRY,
    pub EntryCount: u32,
    pub ContentionCount: u32,
    pub Flags: u32,
    pub CreatorBackTraceIndexHigh: u16,
    pub SpareWORD: u16,
}
impl ::core::marker::Copy for RTL_CRITICAL_SECTION_DEBUG {}
impl ::core::clone::Clone for RTL_CRITICAL_SECTION_DEBUG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RTL_CRITICAL_SECTION_DEBUG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTL_CRITICAL_SECTION_DEBUG")
            .field("Type", &self.Type)
            .field("CreatorBackTraceIndex", &self.CreatorBackTraceIndex)
            .field("CriticalSection", &self.CriticalSection)
            .field("ProcessLocksList", &self.ProcessLocksList)
            .field("EntryCount", &self.EntryCount)
            .field("ContentionCount", &self.ContentionCount)
            .field("Flags", &self.Flags)
            .field("CreatorBackTraceIndexHigh", &self.CreatorBackTraceIndexHigh)
            .field("SpareWORD", &self.SpareWORD)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RTL_CRITICAL_SECTION_DEBUG {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type
            && self.CreatorBackTraceIndex == other.CreatorBackTraceIndex
            && self.CriticalSection == other.CriticalSection
            && self.ProcessLocksList == other.ProcessLocksList
            && self.EntryCount == other.EntryCount
            && self.ContentionCount == other.ContentionCount
            && self.Flags == other.Flags
            && self.CreatorBackTraceIndexHigh == other.CreatorBackTraceIndexHigh
            && self.SpareWORD == other.SpareWORD
    }
}
impl ::core::cmp::Eq for RTL_CRITICAL_SECTION_DEBUG {}
impl FromIntoMemory for RTL_CRITICAL_SECTION_DEBUG {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_Type = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_CreatorBackTraceIndex = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_CriticalSection =
            <MutPtr<RTL_CRITICAL_SECTION> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_ProcessLocksList =
            <super::Kernel::LIST_ENTRY as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_EntryCount = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_ContentionCount = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_Flags = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_CreatorBackTraceIndexHigh = <u16 as FromIntoMemory>::from_bytes(&from[28..28 + 2]);
        let f_SpareWORD = <u16 as FromIntoMemory>::from_bytes(&from[30..30 + 2]);
        Self {
            Type: f_Type,
            CreatorBackTraceIndex: f_CreatorBackTraceIndex,
            CriticalSection: f_CriticalSection,
            ProcessLocksList: f_ProcessLocksList,
            EntryCount: f_EntryCount,
            ContentionCount: f_ContentionCount,
            Flags: f_Flags,
            CreatorBackTraceIndexHigh: f_CreatorBackTraceIndexHigh,
            SpareWORD: f_SpareWORD,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.Type, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.CreatorBackTraceIndex, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.CriticalSection, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.ProcessLocksList, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.EntryCount, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.ContentionCount, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.CreatorBackTraceIndexHigh, &mut into[28..28 + 2]);
        FromIntoMemory::into_bytes(self.SpareWORD, &mut into[30..30 + 2]);
    }
    fn size() -> usize {
        32
    }
}
pub struct RTL_RUN_ONCE {
    data: [u8; 4],
}
impl ::core::marker::Copy for RTL_RUN_ONCE {}
impl ::core::clone::Clone for RTL_RUN_ONCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for RTL_RUN_ONCE {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for RTL_RUN_ONCE {}
impl FromIntoMemory for RTL_RUN_ONCE {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 4];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        4
    }
}
pub struct RTL_SRWLOCK {
    pub Ptr: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for RTL_SRWLOCK {}
impl ::core::clone::Clone for RTL_SRWLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RTL_SRWLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTL_SRWLOCK")
            .field("Ptr", &self.Ptr)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RTL_SRWLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.Ptr == other.Ptr
    }
}
impl ::core::cmp::Eq for RTL_SRWLOCK {}
impl FromIntoMemory for RTL_SRWLOCK {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_Ptr = <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self { Ptr: f_Ptr }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.Ptr, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTL_UMS_THREAD_INFO_CLASS(pub i32);
pub const UmsThreadInvalidInfoClass: RTL_UMS_THREAD_INFO_CLASS = RTL_UMS_THREAD_INFO_CLASS(0i32);
pub const UmsThreadUserContext: RTL_UMS_THREAD_INFO_CLASS = RTL_UMS_THREAD_INFO_CLASS(1i32);
pub const UmsThreadPriority: RTL_UMS_THREAD_INFO_CLASS = RTL_UMS_THREAD_INFO_CLASS(2i32);
pub const UmsThreadAffinity: RTL_UMS_THREAD_INFO_CLASS = RTL_UMS_THREAD_INFO_CLASS(3i32);
pub const UmsThreadTeb: RTL_UMS_THREAD_INFO_CLASS = RTL_UMS_THREAD_INFO_CLASS(4i32);
pub const UmsThreadIsSuspended: RTL_UMS_THREAD_INFO_CLASS = RTL_UMS_THREAD_INFO_CLASS(5i32);
pub const UmsThreadIsTerminated: RTL_UMS_THREAD_INFO_CLASS = RTL_UMS_THREAD_INFO_CLASS(6i32);
pub const UmsThreadMaxInfoClass: RTL_UMS_THREAD_INFO_CLASS = RTL_UMS_THREAD_INFO_CLASS(7i32);
impl ::core::marker::Copy for RTL_UMS_THREAD_INFO_CLASS {}
impl ::core::clone::Clone for RTL_UMS_THREAD_INFO_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTL_UMS_THREAD_INFO_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTL_UMS_THREAD_INFO_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTL_UMS_THREAD_INFO_CLASS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for RTL_UMS_THREAD_INFO_CLASS {
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
pub struct RTL_USER_PROCESS_PARAMETERS {
    pub Reserved1: [u8; 16],
    pub Reserved2: [MutPtr<::core::ffi::c_void>; 10],
    pub ImagePathName: super::super::Foundation::UNICODE_STRING,
    pub CommandLine: super::super::Foundation::UNICODE_STRING,
}
impl ::core::marker::Copy for RTL_USER_PROCESS_PARAMETERS {}
impl ::core::clone::Clone for RTL_USER_PROCESS_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RTL_USER_PROCESS_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTL_USER_PROCESS_PARAMETERS")
            .field("Reserved1", &self.Reserved1)
            .field("Reserved2", &self.Reserved2)
            .field("ImagePathName", &self.ImagePathName)
            .field("CommandLine", &self.CommandLine)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RTL_USER_PROCESS_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
            && self.Reserved2 == other.Reserved2
            && self.ImagePathName == other.ImagePathName
            && self.CommandLine == other.CommandLine
    }
}
impl ::core::cmp::Eq for RTL_USER_PROCESS_PARAMETERS {}
impl FromIntoMemory for RTL_USER_PROCESS_PARAMETERS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 72);
        let f_Reserved1 = <[u8; 16] as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_Reserved2 =
            <[MutPtr<::core::ffi::c_void>; 10] as FromIntoMemory>::from_bytes(&from[16..16 + 40]);
        let f_ImagePathName =
            <super::super::Foundation::UNICODE_STRING as FromIntoMemory>::from_bytes(
                &from[56..56 + 8],
            );
        let f_CommandLine =
            <super::super::Foundation::UNICODE_STRING as FromIntoMemory>::from_bytes(
                &from[64..64 + 8],
            );
        Self {
            Reserved1: f_Reserved1,
            Reserved2: f_Reserved2,
            ImagePathName: f_ImagePathName,
            CommandLine: f_CommandLine,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 72);
        FromIntoMemory::into_bytes(self.Reserved1, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.Reserved2, &mut into[16..16 + 40]);
        FromIntoMemory::into_bytes(self.ImagePathName, &mut into[56..56 + 8]);
        FromIntoMemory::into_bytes(self.CommandLine, &mut into[64..64 + 8]);
    }
    fn size() -> usize {
        72
    }
}
pub struct STARTUPINFOA {
    pub cb: u32,
    pub lpReserved: PSTR,
    pub lpDesktop: PSTR,
    pub lpTitle: PSTR,
    pub dwX: u32,
    pub dwY: u32,
    pub dwXSize: u32,
    pub dwYSize: u32,
    pub dwXCountChars: u32,
    pub dwYCountChars: u32,
    pub dwFillAttribute: u32,
    pub dwFlags: STARTUPINFOW_FLAGS,
    pub wShowWindow: u16,
    pub cbReserved2: u16,
    pub lpReserved2: MutPtr<u8>,
    pub hStdInput: super::super::Foundation::HANDLE,
    pub hStdOutput: super::super::Foundation::HANDLE,
    pub hStdError: super::super::Foundation::HANDLE,
}
impl ::core::marker::Copy for STARTUPINFOA {}
impl ::core::clone::Clone for STARTUPINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STARTUPINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STARTUPINFOA")
            .field("cb", &self.cb)
            .field("lpReserved", &self.lpReserved)
            .field("lpDesktop", &self.lpDesktop)
            .field("lpTitle", &self.lpTitle)
            .field("dwX", &self.dwX)
            .field("dwY", &self.dwY)
            .field("dwXSize", &self.dwXSize)
            .field("dwYSize", &self.dwYSize)
            .field("dwXCountChars", &self.dwXCountChars)
            .field("dwYCountChars", &self.dwYCountChars)
            .field("dwFillAttribute", &self.dwFillAttribute)
            .field("dwFlags", &self.dwFlags)
            .field("wShowWindow", &self.wShowWindow)
            .field("cbReserved2", &self.cbReserved2)
            .field("lpReserved2", &self.lpReserved2)
            .field("hStdInput", &self.hStdInput)
            .field("hStdOutput", &self.hStdOutput)
            .field("hStdError", &self.hStdError)
            .finish()
    }
}
impl ::core::cmp::PartialEq for STARTUPINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb
            && self.lpReserved == other.lpReserved
            && self.lpDesktop == other.lpDesktop
            && self.lpTitle == other.lpTitle
            && self.dwX == other.dwX
            && self.dwY == other.dwY
            && self.dwXSize == other.dwXSize
            && self.dwYSize == other.dwYSize
            && self.dwXCountChars == other.dwXCountChars
            && self.dwYCountChars == other.dwYCountChars
            && self.dwFillAttribute == other.dwFillAttribute
            && self.dwFlags == other.dwFlags
            && self.wShowWindow == other.wShowWindow
            && self.cbReserved2 == other.cbReserved2
            && self.lpReserved2 == other.lpReserved2
            && self.hStdInput == other.hStdInput
            && self.hStdOutput == other.hStdOutput
            && self.hStdError == other.hStdError
    }
}
impl ::core::cmp::Eq for STARTUPINFOA {}
impl FromIntoMemory for STARTUPINFOA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 68);
        let f_cb = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpReserved = <PSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpDesktop = <PSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpTitle = <PSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwX = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwY = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwXSize = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_dwYSize = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_dwXCountChars = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_dwYCountChars = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_dwFillAttribute = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_dwFlags = <STARTUPINFOW_FLAGS as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_wShowWindow = <u16 as FromIntoMemory>::from_bytes(&from[48..48 + 2]);
        let f_cbReserved2 = <u16 as FromIntoMemory>::from_bytes(&from[50..50 + 2]);
        let f_lpReserved2 = <MutPtr<u8> as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_hStdInput =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_hStdOutput =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_hStdError =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        Self {
            cb: f_cb,
            lpReserved: f_lpReserved,
            lpDesktop: f_lpDesktop,
            lpTitle: f_lpTitle,
            dwX: f_dwX,
            dwY: f_dwY,
            dwXSize: f_dwXSize,
            dwYSize: f_dwYSize,
            dwXCountChars: f_dwXCountChars,
            dwYCountChars: f_dwYCountChars,
            dwFillAttribute: f_dwFillAttribute,
            dwFlags: f_dwFlags,
            wShowWindow: f_wShowWindow,
            cbReserved2: f_cbReserved2,
            lpReserved2: f_lpReserved2,
            hStdInput: f_hStdInput,
            hStdOutput: f_hStdOutput,
            hStdError: f_hStdError,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 68);
        FromIntoMemory::into_bytes(self.cb, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpReserved, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpDesktop, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpTitle, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwX, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwY, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwXSize, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.dwYSize, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.dwXCountChars, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.dwYCountChars, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.dwFillAttribute, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.wShowWindow, &mut into[48..48 + 2]);
        FromIntoMemory::into_bytes(self.cbReserved2, &mut into[50..50 + 2]);
        FromIntoMemory::into_bytes(self.lpReserved2, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.hStdInput, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.hStdOutput, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.hStdError, &mut into[64..64 + 4]);
    }
    fn size() -> usize {
        68
    }
}
pub struct STARTUPINFOEXA {
    pub StartupInfo: STARTUPINFOA,
    pub lpAttributeList: LPPROC_THREAD_ATTRIBUTE_LIST,
}
impl ::core::marker::Copy for STARTUPINFOEXA {}
impl ::core::clone::Clone for STARTUPINFOEXA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STARTUPINFOEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STARTUPINFOEXA")
            .field("StartupInfo", &self.StartupInfo)
            .field("lpAttributeList", &self.lpAttributeList)
            .finish()
    }
}
impl ::core::cmp::PartialEq for STARTUPINFOEXA {
    fn eq(&self, other: &Self) -> bool {
        self.StartupInfo == other.StartupInfo && self.lpAttributeList == other.lpAttributeList
    }
}
impl ::core::cmp::Eq for STARTUPINFOEXA {}
impl FromIntoMemory for STARTUPINFOEXA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 72);
        let f_StartupInfo = <STARTUPINFOA as FromIntoMemory>::from_bytes(&from[0..0 + 68]);
        let f_lpAttributeList =
            <LPPROC_THREAD_ATTRIBUTE_LIST as FromIntoMemory>::from_bytes(&from[68..68 + 4]);
        Self {
            StartupInfo: f_StartupInfo,
            lpAttributeList: f_lpAttributeList,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 72);
        FromIntoMemory::into_bytes(self.StartupInfo, &mut into[0..0 + 68]);
        FromIntoMemory::into_bytes(self.lpAttributeList, &mut into[68..68 + 4]);
    }
    fn size() -> usize {
        72
    }
}
pub struct STARTUPINFOEXW {
    pub StartupInfo: STARTUPINFOW,
    pub lpAttributeList: LPPROC_THREAD_ATTRIBUTE_LIST,
}
impl ::core::marker::Copy for STARTUPINFOEXW {}
impl ::core::clone::Clone for STARTUPINFOEXW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STARTUPINFOEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STARTUPINFOEXW")
            .field("StartupInfo", &self.StartupInfo)
            .field("lpAttributeList", &self.lpAttributeList)
            .finish()
    }
}
impl ::core::cmp::PartialEq for STARTUPINFOEXW {
    fn eq(&self, other: &Self) -> bool {
        self.StartupInfo == other.StartupInfo && self.lpAttributeList == other.lpAttributeList
    }
}
impl ::core::cmp::Eq for STARTUPINFOEXW {}
impl FromIntoMemory for STARTUPINFOEXW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 72);
        let f_StartupInfo = <STARTUPINFOW as FromIntoMemory>::from_bytes(&from[0..0 + 68]);
        let f_lpAttributeList =
            <LPPROC_THREAD_ATTRIBUTE_LIST as FromIntoMemory>::from_bytes(&from[68..68 + 4]);
        Self {
            StartupInfo: f_StartupInfo,
            lpAttributeList: f_lpAttributeList,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 72);
        FromIntoMemory::into_bytes(self.StartupInfo, &mut into[0..0 + 68]);
        FromIntoMemory::into_bytes(self.lpAttributeList, &mut into[68..68 + 4]);
    }
    fn size() -> usize {
        72
    }
}
pub struct STARTUPINFOW {
    pub cb: u32,
    pub lpReserved: PWSTR,
    pub lpDesktop: PWSTR,
    pub lpTitle: PWSTR,
    pub dwX: u32,
    pub dwY: u32,
    pub dwXSize: u32,
    pub dwYSize: u32,
    pub dwXCountChars: u32,
    pub dwYCountChars: u32,
    pub dwFillAttribute: u32,
    pub dwFlags: STARTUPINFOW_FLAGS,
    pub wShowWindow: u16,
    pub cbReserved2: u16,
    pub lpReserved2: MutPtr<u8>,
    pub hStdInput: super::super::Foundation::HANDLE,
    pub hStdOutput: super::super::Foundation::HANDLE,
    pub hStdError: super::super::Foundation::HANDLE,
}
impl ::core::marker::Copy for STARTUPINFOW {}
impl ::core::clone::Clone for STARTUPINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STARTUPINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STARTUPINFOW")
            .field("cb", &self.cb)
            .field("lpReserved", &self.lpReserved)
            .field("lpDesktop", &self.lpDesktop)
            .field("lpTitle", &self.lpTitle)
            .field("dwX", &self.dwX)
            .field("dwY", &self.dwY)
            .field("dwXSize", &self.dwXSize)
            .field("dwYSize", &self.dwYSize)
            .field("dwXCountChars", &self.dwXCountChars)
            .field("dwYCountChars", &self.dwYCountChars)
            .field("dwFillAttribute", &self.dwFillAttribute)
            .field("dwFlags", &self.dwFlags)
            .field("wShowWindow", &self.wShowWindow)
            .field("cbReserved2", &self.cbReserved2)
            .field("lpReserved2", &self.lpReserved2)
            .field("hStdInput", &self.hStdInput)
            .field("hStdOutput", &self.hStdOutput)
            .field("hStdError", &self.hStdError)
            .finish()
    }
}
impl ::core::cmp::PartialEq for STARTUPINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb
            && self.lpReserved == other.lpReserved
            && self.lpDesktop == other.lpDesktop
            && self.lpTitle == other.lpTitle
            && self.dwX == other.dwX
            && self.dwY == other.dwY
            && self.dwXSize == other.dwXSize
            && self.dwYSize == other.dwYSize
            && self.dwXCountChars == other.dwXCountChars
            && self.dwYCountChars == other.dwYCountChars
            && self.dwFillAttribute == other.dwFillAttribute
            && self.dwFlags == other.dwFlags
            && self.wShowWindow == other.wShowWindow
            && self.cbReserved2 == other.cbReserved2
            && self.lpReserved2 == other.lpReserved2
            && self.hStdInput == other.hStdInput
            && self.hStdOutput == other.hStdOutput
            && self.hStdError == other.hStdError
    }
}
impl ::core::cmp::Eq for STARTUPINFOW {}
impl FromIntoMemory for STARTUPINFOW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 68);
        let f_cb = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpReserved = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpDesktop = <PWSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpTitle = <PWSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwX = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwY = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwXSize = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_dwYSize = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_dwXCountChars = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_dwYCountChars = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_dwFillAttribute = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_dwFlags = <STARTUPINFOW_FLAGS as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_wShowWindow = <u16 as FromIntoMemory>::from_bytes(&from[48..48 + 2]);
        let f_cbReserved2 = <u16 as FromIntoMemory>::from_bytes(&from[50..50 + 2]);
        let f_lpReserved2 = <MutPtr<u8> as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_hStdInput =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_hStdOutput =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_hStdError =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        Self {
            cb: f_cb,
            lpReserved: f_lpReserved,
            lpDesktop: f_lpDesktop,
            lpTitle: f_lpTitle,
            dwX: f_dwX,
            dwY: f_dwY,
            dwXSize: f_dwXSize,
            dwYSize: f_dwYSize,
            dwXCountChars: f_dwXCountChars,
            dwYCountChars: f_dwYCountChars,
            dwFillAttribute: f_dwFillAttribute,
            dwFlags: f_dwFlags,
            wShowWindow: f_wShowWindow,
            cbReserved2: f_cbReserved2,
            lpReserved2: f_lpReserved2,
            hStdInput: f_hStdInput,
            hStdOutput: f_hStdOutput,
            hStdError: f_hStdError,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 68);
        FromIntoMemory::into_bytes(self.cb, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpReserved, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpDesktop, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpTitle, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwX, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwY, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwXSize, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.dwYSize, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.dwXCountChars, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.dwYCountChars, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.dwFillAttribute, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.wShowWindow, &mut into[48..48 + 2]);
        FromIntoMemory::into_bytes(self.cbReserved2, &mut into[50..50 + 2]);
        FromIntoMemory::into_bytes(self.lpReserved2, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.hStdInput, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.hStdOutput, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.hStdError, &mut into[64..64 + 4]);
    }
    fn size() -> usize {
        68
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STARTUPINFOW_FLAGS(pub u32);
pub const STARTF_FORCEONFEEDBACK: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(64u32);
pub const STARTF_FORCEOFFFEEDBACK: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(128u32);
pub const STARTF_PREVENTPINNING: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(8192u32);
pub const STARTF_RUNFULLSCREEN: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(32u32);
pub const STARTF_TITLEISAPPID: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(4096u32);
pub const STARTF_TITLEISLINKNAME: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(2048u32);
pub const STARTF_UNTRUSTEDSOURCE: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(32768u32);
pub const STARTF_USECOUNTCHARS: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(8u32);
pub const STARTF_USEFILLATTRIBUTE: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(16u32);
pub const STARTF_USEHOTKEY: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(512u32);
pub const STARTF_USEPOSITION: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(4u32);
pub const STARTF_USESHOWWINDOW: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(1u32);
pub const STARTF_USESIZE: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(2u32);
pub const STARTF_USESTDHANDLES: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(256u32);
impl ::core::marker::Copy for STARTUPINFOW_FLAGS {}
impl ::core::clone::Clone for STARTUPINFOW_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STARTUPINFOW_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STARTUPINFOW_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STARTUPINFOW_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for STARTUPINFOW_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for STARTUPINFOW_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for STARTUPINFOW_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for STARTUPINFOW_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for STARTUPINFOW_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for STARTUPINFOW_FLAGS {
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
pub const SYNCHRONIZATION_BARRIER_FLAGS_BLOCK_ONLY: u32 = 2u32;
pub const SYNCHRONIZATION_BARRIER_FLAGS_NO_DELETE: u32 = 4u32;
pub const SYNCHRONIZATION_BARRIER_FLAGS_SPIN_ONLY: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct THREADINFOCLASS(pub i32);
pub const ThreadIsIoPending: THREADINFOCLASS = THREADINFOCLASS(16i32);
pub const ThreadNameInformation: THREADINFOCLASS = THREADINFOCLASS(38i32);
impl ::core::marker::Copy for THREADINFOCLASS {}
impl ::core::clone::Clone for THREADINFOCLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for THREADINFOCLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for THREADINFOCLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THREADINFOCLASS").field(&self.0).finish()
    }
}
impl FromIntoMemory for THREADINFOCLASS {
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
pub struct THREAD_ACCESS_RIGHTS(pub u32);
pub const THREAD_TERMINATE: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(1u32);
pub const THREAD_SUSPEND_RESUME: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(2u32);
pub const THREAD_GET_CONTEXT: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(8u32);
pub const THREAD_SET_CONTEXT: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(16u32);
pub const THREAD_SET_INFORMATION: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(32u32);
pub const THREAD_QUERY_INFORMATION: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(64u32);
pub const THREAD_SET_THREAD_TOKEN: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(128u32);
pub const THREAD_IMPERSONATE: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(256u32);
pub const THREAD_DIRECT_IMPERSONATION: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(512u32);
pub const THREAD_SET_LIMITED_INFORMATION: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(1024u32);
pub const THREAD_QUERY_LIMITED_INFORMATION: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(2048u32);
pub const THREAD_RESUME: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(4096u32);
pub const THREAD_ALL_ACCESS: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(2097151u32);
pub const THREAD_DELETE: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(65536u32);
pub const THREAD_READ_CONTROL: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(131072u32);
pub const THREAD_WRITE_DAC: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(262144u32);
pub const THREAD_WRITE_OWNER: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(524288u32);
pub const THREAD_SYNCHRONIZE: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(1048576u32);
pub const THREAD_STANDARD_RIGHTS_REQUIRED: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(983040u32);
impl ::core::marker::Copy for THREAD_ACCESS_RIGHTS {}
impl ::core::clone::Clone for THREAD_ACCESS_RIGHTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for THREAD_ACCESS_RIGHTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for THREAD_ACCESS_RIGHTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THREAD_ACCESS_RIGHTS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for THREAD_ACCESS_RIGHTS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for THREAD_ACCESS_RIGHTS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for THREAD_ACCESS_RIGHTS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for THREAD_ACCESS_RIGHTS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for THREAD_ACCESS_RIGHTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for THREAD_ACCESS_RIGHTS {
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
pub struct THREAD_CREATION_FLAGS(pub u32);
pub const THREAD_CREATE_RUN_IMMEDIATELY: THREAD_CREATION_FLAGS = THREAD_CREATION_FLAGS(0u32);
pub const THREAD_CREATE_SUSPENDED: THREAD_CREATION_FLAGS = THREAD_CREATION_FLAGS(4u32);
pub const STACK_SIZE_PARAM_IS_A_RESERVATION: THREAD_CREATION_FLAGS =
    THREAD_CREATION_FLAGS(65536u32);
impl ::core::marker::Copy for THREAD_CREATION_FLAGS {}
impl ::core::clone::Clone for THREAD_CREATION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for THREAD_CREATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for THREAD_CREATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THREAD_CREATION_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for THREAD_CREATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for THREAD_CREATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for THREAD_CREATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for THREAD_CREATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for THREAD_CREATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for THREAD_CREATION_FLAGS {
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
pub struct THREAD_INFORMATION_CLASS(pub i32);
pub const ThreadMemoryPriority: THREAD_INFORMATION_CLASS = THREAD_INFORMATION_CLASS(0i32);
pub const ThreadAbsoluteCpuPriority: THREAD_INFORMATION_CLASS = THREAD_INFORMATION_CLASS(1i32);
pub const ThreadDynamicCodePolicy: THREAD_INFORMATION_CLASS = THREAD_INFORMATION_CLASS(2i32);
pub const ThreadPowerThrottling: THREAD_INFORMATION_CLASS = THREAD_INFORMATION_CLASS(3i32);
pub const ThreadInformationClassMax: THREAD_INFORMATION_CLASS = THREAD_INFORMATION_CLASS(4i32);
impl ::core::marker::Copy for THREAD_INFORMATION_CLASS {}
impl ::core::clone::Clone for THREAD_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for THREAD_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for THREAD_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THREAD_INFORMATION_CLASS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for THREAD_INFORMATION_CLASS {
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
pub const THREAD_POWER_THROTTLING_CURRENT_VERSION: u32 = 1u32;
pub const THREAD_POWER_THROTTLING_EXECUTION_SPEED: u32 = 1u32;
pub struct THREAD_POWER_THROTTLING_STATE {
    pub Version: u32,
    pub ControlMask: u32,
    pub StateMask: u32,
}
impl ::core::marker::Copy for THREAD_POWER_THROTTLING_STATE {}
impl ::core::clone::Clone for THREAD_POWER_THROTTLING_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for THREAD_POWER_THROTTLING_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("THREAD_POWER_THROTTLING_STATE")
            .field("Version", &self.Version)
            .field("ControlMask", &self.ControlMask)
            .field("StateMask", &self.StateMask)
            .finish()
    }
}
impl ::core::cmp::PartialEq for THREAD_POWER_THROTTLING_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.ControlMask == other.ControlMask
            && self.StateMask == other.StateMask
    }
}
impl ::core::cmp::Eq for THREAD_POWER_THROTTLING_STATE {}
impl FromIntoMemory for THREAD_POWER_THROTTLING_STATE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Version = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ControlMask = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_StateMask = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            Version: f_Version,
            ControlMask: f_ControlMask,
            StateMask: f_StateMask,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Version, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ControlMask, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.StateMask, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub const THREAD_POWER_THROTTLING_VALID_FLAGS: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct THREAD_PRIORITY(pub i32);
pub const THREAD_MODE_BACKGROUND_BEGIN: THREAD_PRIORITY = THREAD_PRIORITY(65536i32);
pub const THREAD_MODE_BACKGROUND_END: THREAD_PRIORITY = THREAD_PRIORITY(131072i32);
pub const THREAD_PRIORITY_ABOVE_NORMAL: THREAD_PRIORITY = THREAD_PRIORITY(1i32);
pub const THREAD_PRIORITY_BELOW_NORMAL: THREAD_PRIORITY = THREAD_PRIORITY(-1i32);
pub const THREAD_PRIORITY_HIGHEST: THREAD_PRIORITY = THREAD_PRIORITY(2i32);
pub const THREAD_PRIORITY_IDLE: THREAD_PRIORITY = THREAD_PRIORITY(-15i32);
pub const THREAD_PRIORITY_MIN: THREAD_PRIORITY = THREAD_PRIORITY(-2i32);
pub const THREAD_PRIORITY_LOWEST: THREAD_PRIORITY = THREAD_PRIORITY(-2i32);
pub const THREAD_PRIORITY_NORMAL: THREAD_PRIORITY = THREAD_PRIORITY(0i32);
pub const THREAD_PRIORITY_TIME_CRITICAL: THREAD_PRIORITY = THREAD_PRIORITY(15i32);
impl ::core::marker::Copy for THREAD_PRIORITY {}
impl ::core::clone::Clone for THREAD_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for THREAD_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for THREAD_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THREAD_PRIORITY").field(&self.0).finish()
    }
}
impl FromIntoMemory for THREAD_PRIORITY {
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
pub struct TP_CALLBACK_ENVIRON_V3 {
    pub Version: u32,
    pub Pool: PTP_POOL,
    pub CleanupGroup: PtrDiffRepr,
    pub CleanupGroupCancelCallback: PTP_CLEANUP_GROUP_CANCEL_CALLBACK,
    pub RaceDll: MutPtr<::core::ffi::c_void>,
    pub ActivationContext: PtrDiffRepr,
    pub FinalizationCallback: PTP_SIMPLE_CALLBACK,
    pub u: TP_CALLBACK_ENVIRON_V3_1,
    pub CallbackPriority: TP_CALLBACK_PRIORITY,
    pub Size: u32,
}
impl ::core::marker::Copy for TP_CALLBACK_ENVIRON_V3 {}
impl ::core::clone::Clone for TP_CALLBACK_ENVIRON_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for TP_CALLBACK_ENVIRON_V3 {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Pool == other.Pool
            && self.CleanupGroup == other.CleanupGroup
            && self.CleanupGroupCancelCallback == other.CleanupGroupCancelCallback
            && self.RaceDll == other.RaceDll
            && self.ActivationContext == other.ActivationContext
            && self.FinalizationCallback == other.FinalizationCallback
            && self.u == other.u
            && self.CallbackPriority == other.CallbackPriority
            && self.Size == other.Size
    }
}
impl ::core::cmp::Eq for TP_CALLBACK_ENVIRON_V3 {}
impl FromIntoMemory for TP_CALLBACK_ENVIRON_V3 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_Version = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Pool = <PTP_POOL as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_CleanupGroup = <PtrDiffRepr as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_CleanupGroupCancelCallback =
            <PTP_CLEANUP_GROUP_CANCEL_CALLBACK as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_RaceDll =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_ActivationContext = <PtrDiffRepr as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_FinalizationCallback =
            <PTP_SIMPLE_CALLBACK as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_u = <TP_CALLBACK_ENVIRON_V3_1 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_CallbackPriority =
            <TP_CALLBACK_PRIORITY as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_Size = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        Self {
            Version: f_Version,
            Pool: f_Pool,
            CleanupGroup: f_CleanupGroup,
            CleanupGroupCancelCallback: f_CleanupGroupCancelCallback,
            RaceDll: f_RaceDll,
            ActivationContext: f_ActivationContext,
            FinalizationCallback: f_FinalizationCallback,
            u: f_u,
            CallbackPriority: f_CallbackPriority,
            Size: f_Size,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.Version, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Pool, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.CleanupGroup, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.CleanupGroupCancelCallback, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.RaceDll, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.ActivationContext, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.FinalizationCallback, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.u, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.CallbackPriority, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.Size, &mut into[36..36 + 4]);
    }
    fn size() -> usize {
        40
    }
}
pub struct TP_CALLBACK_ENVIRON_V3_0(pub u8);
pub struct TP_CALLBACK_ENVIRON_V3_1 {
    data: [u8; 4],
}
impl ::core::marker::Copy for TP_CALLBACK_ENVIRON_V3_1 {}
impl ::core::clone::Clone for TP_CALLBACK_ENVIRON_V3_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for TP_CALLBACK_ENVIRON_V3_1 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for TP_CALLBACK_ENVIRON_V3_1 {}
impl FromIntoMemory for TP_CALLBACK_ENVIRON_V3_1 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 4];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        4
    }
}
pub struct TP_CALLBACK_ENVIRON_V3_1_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for TP_CALLBACK_ENVIRON_V3_1_0 {}
impl ::core::clone::Clone for TP_CALLBACK_ENVIRON_V3_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TP_CALLBACK_ENVIRON_V3_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TP_CALLBACK_ENVIRON_V3_1_0")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TP_CALLBACK_ENVIRON_V3_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for TP_CALLBACK_ENVIRON_V3_1_0 {}
impl FromIntoMemory for TP_CALLBACK_ENVIRON_V3_1_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f__bitfield = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            _bitfield: f__bitfield,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self._bitfield, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct TP_CALLBACK_INSTANCE(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TP_CALLBACK_PRIORITY(pub i32);
pub const TP_CALLBACK_PRIORITY_HIGH: TP_CALLBACK_PRIORITY = TP_CALLBACK_PRIORITY(0i32);
pub const TP_CALLBACK_PRIORITY_NORMAL: TP_CALLBACK_PRIORITY = TP_CALLBACK_PRIORITY(1i32);
pub const TP_CALLBACK_PRIORITY_LOW: TP_CALLBACK_PRIORITY = TP_CALLBACK_PRIORITY(2i32);
pub const TP_CALLBACK_PRIORITY_INVALID: TP_CALLBACK_PRIORITY = TP_CALLBACK_PRIORITY(3i32);
pub const TP_CALLBACK_PRIORITY_COUNT: TP_CALLBACK_PRIORITY = TP_CALLBACK_PRIORITY(3i32);
impl ::core::marker::Copy for TP_CALLBACK_PRIORITY {}
impl ::core::clone::Clone for TP_CALLBACK_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TP_CALLBACK_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TP_CALLBACK_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TP_CALLBACK_PRIORITY")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for TP_CALLBACK_PRIORITY {
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
pub struct TP_IO(pub u8);
pub struct TP_POOL_STACK_INFORMATION {
    pub StackReserve: PtrRepr,
    pub StackCommit: PtrRepr,
}
impl ::core::marker::Copy for TP_POOL_STACK_INFORMATION {}
impl ::core::clone::Clone for TP_POOL_STACK_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TP_POOL_STACK_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TP_POOL_STACK_INFORMATION")
            .field("StackReserve", &self.StackReserve)
            .field("StackCommit", &self.StackCommit)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TP_POOL_STACK_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.StackReserve == other.StackReserve && self.StackCommit == other.StackCommit
    }
}
impl ::core::cmp::Eq for TP_POOL_STACK_INFORMATION {}
impl FromIntoMemory for TP_POOL_STACK_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_StackReserve = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_StackCommit = <PtrRepr as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            StackReserve: f_StackReserve,
            StackCommit: f_StackCommit,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.StackReserve, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.StackCommit, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct TP_TIMER(pub u8);
pub struct TP_WAIT(pub u8);
pub struct TP_WORK(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TimerQueueHandle(pub PtrDiffRepr);
impl TimerQueueHandle {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TimerQueueHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for TimerQueueHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for TimerQueueHandle {}
impl ::core::hash::Hash for TimerQueueHandle {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for TimerQueueHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimerQueueHandle").field(&self.0).finish()
    }
}
impl FromIntoMemory for TimerQueueHandle {
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
pub struct UMS_SCHEDULER_STARTUP_INFO {
    pub UmsVersion: u32,
    pub CompletionList: MutPtr<::core::ffi::c_void>,
    pub SchedulerProc: PRTL_UMS_SCHEDULER_ENTRY_POINT,
    pub SchedulerParam: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for UMS_SCHEDULER_STARTUP_INFO {}
impl ::core::clone::Clone for UMS_SCHEDULER_STARTUP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UMS_SCHEDULER_STARTUP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UMS_SCHEDULER_STARTUP_INFO")
            .field("UmsVersion", &self.UmsVersion)
            .field("CompletionList", &self.CompletionList)
            .field("SchedulerProc", &self.SchedulerProc)
            .field("SchedulerParam", &self.SchedulerParam)
            .finish()
    }
}
impl ::core::cmp::PartialEq for UMS_SCHEDULER_STARTUP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.UmsVersion == other.UmsVersion
            && self.CompletionList == other.CompletionList
            && self.SchedulerProc == other.SchedulerProc
            && self.SchedulerParam == other.SchedulerParam
    }
}
impl ::core::cmp::Eq for UMS_SCHEDULER_STARTUP_INFO {}
impl FromIntoMemory for UMS_SCHEDULER_STARTUP_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_UmsVersion = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_CompletionList =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_SchedulerProc =
            <PRTL_UMS_SCHEDULER_ENTRY_POINT as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_SchedulerParam =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            UmsVersion: f_UmsVersion,
            CompletionList: f_CompletionList,
            SchedulerProc: f_SchedulerProc,
            SchedulerParam: f_SchedulerParam,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.UmsVersion, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.CompletionList, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.SchedulerProc, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.SchedulerParam, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct UMS_SYSTEM_THREAD_INFORMATION {
    pub UmsVersion: u32,
    pub Anonymous: UMS_SYSTEM_THREAD_INFORMATION_0,
}
impl ::core::marker::Copy for UMS_SYSTEM_THREAD_INFORMATION {}
impl ::core::clone::Clone for UMS_SYSTEM_THREAD_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for UMS_SYSTEM_THREAD_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.UmsVersion == other.UmsVersion && self.Anonymous == other.Anonymous
    }
}
impl ::core::cmp::Eq for UMS_SYSTEM_THREAD_INFORMATION {}
impl FromIntoMemory for UMS_SYSTEM_THREAD_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_UmsVersion = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Anonymous =
            <UMS_SYSTEM_THREAD_INFORMATION_0 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            UmsVersion: f_UmsVersion,
            Anonymous: f_Anonymous,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.UmsVersion, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct UMS_SYSTEM_THREAD_INFORMATION_0 {
    data: [u8; 4],
}
impl ::core::marker::Copy for UMS_SYSTEM_THREAD_INFORMATION_0 {}
impl ::core::clone::Clone for UMS_SYSTEM_THREAD_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for UMS_SYSTEM_THREAD_INFORMATION_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for UMS_SYSTEM_THREAD_INFORMATION_0 {}
impl FromIntoMemory for UMS_SYSTEM_THREAD_INFORMATION_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 4];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        4
    }
}
pub struct UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for UMS_SYSTEM_THREAD_INFORMATION_0_0 {}
impl ::core::clone::Clone for UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UMS_SYSTEM_THREAD_INFORMATION_0_0")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::core::cmp::PartialEq for UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for UMS_SYSTEM_THREAD_INFORMATION_0_0 {}
impl FromIntoMemory for UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f__bitfield = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            _bitfield: f__bitfield,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self._bitfield, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub type WAITORTIMERCALLBACK = StdCallFnPtr<
    (
        MutPtr<::core::ffi::c_void>,
        super::super::Foundation::BOOLEAN,
    ),
    (),
>;
pub const WAIT_ABANDONED: u32 = 128u32;
pub const WAIT_ABANDONED_0: u32 = 128u32;
pub const WAIT_IO_COMPLETION: u32 = 192u32;
pub const WAIT_OBJECT_0: u32 = 0u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WORKER_THREAD_FLAGS(pub u32);
pub const WT_EXECUTEDEFAULT: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(0u32);
pub const WT_EXECUTEINIOTHREAD: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(1u32);
pub const WT_EXECUTEINPERSISTENTTHREAD: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(128u32);
pub const WT_EXECUTEINWAITTHREAD: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(4u32);
pub const WT_EXECUTELONGFUNCTION: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(16u32);
pub const WT_EXECUTEONLYONCE: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(8u32);
pub const WT_TRANSFER_IMPERSONATION: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(256u32);
pub const WT_EXECUTEINTIMERTHREAD: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(32u32);
impl ::core::marker::Copy for WORKER_THREAD_FLAGS {}
impl ::core::clone::Clone for WORKER_THREAD_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WORKER_THREAD_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WORKER_THREAD_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WORKER_THREAD_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WORKER_THREAD_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WORKER_THREAD_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WORKER_THREAD_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WORKER_THREAD_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WORKER_THREAD_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for WORKER_THREAD_FLAGS {
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
pub trait Api {
    fn AcquireSRWLockExclusive(&self, srw_lock: MutPtr<RTL_SRWLOCK>) {
        todo!("AcquireSRWLockExclusive")
    }
    fn AcquireSRWLockShared(&self, srw_lock: MutPtr<RTL_SRWLOCK>) {
        todo!("AcquireSRWLockShared")
    }
    fn AddIntegrityLabelToBoundaryDescriptor(
        &self,
        boundary_descriptor: MutPtr<super::super::Foundation::HANDLE>,
        integrity_label: super::super::Foundation::PSID,
    ) -> super::super::Foundation::BOOL {
        todo!("AddIntegrityLabelToBoundaryDescriptor")
    }
    fn AddSIDToBoundaryDescriptor(
        &self,
        boundary_descriptor: MutPtr<super::super::Foundation::HANDLE>,
        required_sid: super::super::Foundation::PSID,
    ) -> super::super::Foundation::BOOL {
        todo!("AddSIDToBoundaryDescriptor")
    }
    fn AttachThreadInput(
        &self,
        id_attach: u32,
        id_attach_to: u32,
        f_attach: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!("AttachThreadInput")
    }
    fn AvQuerySystemResponsiveness(
        &self,
        avrt_handle: super::super::Foundation::HANDLE,
        system_responsiveness_value: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("AvQuerySystemResponsiveness")
    }
    fn AvRevertMmThreadCharacteristics(
        &self,
        avrt_handle: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("AvRevertMmThreadCharacteristics")
    }
    fn AvRtCreateThreadOrderingGroup(
        &self,
        context: MutPtr<super::super::Foundation::HANDLE>,
        period: ConstPtr<i64>,
        thread_ordering_guid: MutPtr<crate::core::GUID>,
        timeout: ConstPtr<i64>,
    ) -> super::super::Foundation::BOOL {
        todo!("AvRtCreateThreadOrderingGroup")
    }
    fn AvRtCreateThreadOrderingGroupExA(
        &self,
        context: MutPtr<super::super::Foundation::HANDLE>,
        period: ConstPtr<i64>,
        thread_ordering_guid: MutPtr<crate::core::GUID>,
        timeout: ConstPtr<i64>,
        task_name: PCSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("AvRtCreateThreadOrderingGroupExA")
    }
    fn AvRtCreateThreadOrderingGroupExW(
        &self,
        context: MutPtr<super::super::Foundation::HANDLE>,
        period: ConstPtr<i64>,
        thread_ordering_guid: MutPtr<crate::core::GUID>,
        timeout: ConstPtr<i64>,
        task_name: PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("AvRtCreateThreadOrderingGroupExW")
    }
    fn AvRtDeleteThreadOrderingGroup(
        &self,
        context: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("AvRtDeleteThreadOrderingGroup")
    }
    fn AvRtJoinThreadOrderingGroup(
        &self,
        context: MutPtr<super::super::Foundation::HANDLE>,
        thread_ordering_guid: ConstPtr<crate::core::GUID>,
        before: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!("AvRtJoinThreadOrderingGroup")
    }
    fn AvRtLeaveThreadOrderingGroup(
        &self,
        context: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("AvRtLeaveThreadOrderingGroup")
    }
    fn AvRtWaitOnThreadOrderingGroup(
        &self,
        context: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("AvRtWaitOnThreadOrderingGroup")
    }
    fn AvSetMmMaxThreadCharacteristicsA(
        &self,
        first_task: PCSTR,
        second_task: PCSTR,
        task_index: MutPtr<u32>,
    ) -> super::super::Foundation::HANDLE {
        todo!("AvSetMmMaxThreadCharacteristicsA")
    }
    fn AvSetMmMaxThreadCharacteristicsW(
        &self,
        first_task: PCWSTR,
        second_task: PCWSTR,
        task_index: MutPtr<u32>,
    ) -> super::super::Foundation::HANDLE {
        todo!("AvSetMmMaxThreadCharacteristicsW")
    }
    fn AvSetMmThreadCharacteristicsA(
        &self,
        task_name: PCSTR,
        task_index: MutPtr<u32>,
    ) -> super::super::Foundation::HANDLE {
        todo!("AvSetMmThreadCharacteristicsA")
    }
    fn AvSetMmThreadCharacteristicsW(
        &self,
        task_name: PCWSTR,
        task_index: MutPtr<u32>,
    ) -> super::super::Foundation::HANDLE {
        todo!("AvSetMmThreadCharacteristicsW")
    }
    fn AvSetMmThreadPriority(
        &self,
        avrt_handle: super::super::Foundation::HANDLE,
        priority: AVRT_PRIORITY,
    ) -> super::super::Foundation::BOOL {
        todo!("AvSetMmThreadPriority")
    }
    fn CallbackMayRunLong(
        &self,
        pci: MutPtr<TP_CALLBACK_INSTANCE>,
    ) -> super::super::Foundation::BOOL {
        todo!("CallbackMayRunLong")
    }
    fn CancelThreadpoolIo(&self, pio: MutPtr<TP_IO>) {
        todo!("CancelThreadpoolIo")
    }
    fn CancelWaitableTimer(
        &self,
        h_timer: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("CancelWaitableTimer")
    }
    fn ChangeTimerQueueTimer(
        &self,
        timer_queue: super::super::Foundation::HANDLE,
        timer: super::super::Foundation::HANDLE,
        due_time: u32,
        period: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("ChangeTimerQueueTimer")
    }
    fn ClosePrivateNamespace(
        &self,
        handle: NamespaceHandle,
        flags: u32,
    ) -> super::super::Foundation::BOOLEAN {
        todo!("ClosePrivateNamespace")
    }
    fn CloseThreadpool(&self, ptpp: PTP_POOL) {
        todo!("CloseThreadpool")
    }
    fn CloseThreadpoolCleanupGroup(&self, ptpcg: PtrDiffRepr) {
        todo!("CloseThreadpoolCleanupGroup")
    }
    fn CloseThreadpoolCleanupGroupMembers(
        &self,
        ptpcg: PtrDiffRepr,
        f_cancel_pending_callbacks: super::super::Foundation::BOOL,
        pv_cleanup_context: MutPtr<::core::ffi::c_void>,
    ) {
        todo!("CloseThreadpoolCleanupGroupMembers")
    }
    fn CloseThreadpoolIo(&self, pio: MutPtr<TP_IO>) {
        todo!("CloseThreadpoolIo")
    }
    fn CloseThreadpoolTimer(&self, pti: MutPtr<TP_TIMER>) {
        todo!("CloseThreadpoolTimer")
    }
    fn CloseThreadpoolWait(&self, pwa: MutPtr<TP_WAIT>) {
        todo!("CloseThreadpoolWait")
    }
    fn CloseThreadpoolWork(&self, pwk: MutPtr<TP_WORK>) {
        todo!("CloseThreadpoolWork")
    }
    fn ConvertFiberToThread(&self) -> super::super::Foundation::BOOL {
        todo!("ConvertFiberToThread")
    }
    fn ConvertThreadToFiber(
        &self,
        lp_parameter: ConstPtr<::core::ffi::c_void>,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!("ConvertThreadToFiber")
    }
    fn ConvertThreadToFiberEx(
        &self,
        lp_parameter: ConstPtr<::core::ffi::c_void>,
        dw_flags: u32,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!("ConvertThreadToFiberEx")
    }
    fn CreateBoundaryDescriptorA(&self, name: PCSTR, flags: u32) -> BoundaryDescriptorHandle {
        todo!("CreateBoundaryDescriptorA")
    }
    fn CreateBoundaryDescriptorW(&self, name: PCWSTR, flags: u32) -> BoundaryDescriptorHandle {
        todo!("CreateBoundaryDescriptorW")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateEventA(
        &self,
        lp_event_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        b_manual_reset: super::super::Foundation::BOOL,
        b_initial_state: super::super::Foundation::BOOL,
        lp_name: PCSTR,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateEventA")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateEventExA(
        &self,
        lp_event_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        lp_name: PCSTR,
        dw_flags: CREATE_EVENT,
        dw_desired_access: u32,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateEventExA")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateEventExW(
        &self,
        lp_event_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        lp_name: PCWSTR,
        dw_flags: CREATE_EVENT,
        dw_desired_access: u32,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateEventExW")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateEventW(
        &self,
        lp_event_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        b_manual_reset: super::super::Foundation::BOOL,
        b_initial_state: super::super::Foundation::BOOL,
        lp_name: PCWSTR,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateEventW")
    }
    fn CreateFiber(
        &self,
        dw_stack_size: PtrRepr,
        lp_start_address: LPFIBER_START_ROUTINE,
        lp_parameter: ConstPtr<::core::ffi::c_void>,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!("CreateFiber")
    }
    fn CreateFiberEx(
        &self,
        dw_stack_commit_size: PtrRepr,
        dw_stack_reserve_size: PtrRepr,
        dw_flags: u32,
        lp_start_address: LPFIBER_START_ROUTINE,
        lp_parameter: ConstPtr<::core::ffi::c_void>,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!("CreateFiberEx")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateMutexA(
        &self,
        lp_mutex_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        b_initial_owner: super::super::Foundation::BOOL,
        lp_name: PCSTR,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateMutexA")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateMutexExA(
        &self,
        lp_mutex_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        lp_name: PCSTR,
        dw_flags: u32,
        dw_desired_access: u32,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateMutexExA")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateMutexExW(
        &self,
        lp_mutex_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        lp_name: PCWSTR,
        dw_flags: u32,
        dw_desired_access: u32,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateMutexExW")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateMutexW(
        &self,
        lp_mutex_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        b_initial_owner: super::super::Foundation::BOOL,
        lp_name: PCWSTR,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateMutexW")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreatePrivateNamespaceA(
        &self,
        lp_private_namespace_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        lp_boundary_descriptor: ConstPtr<::core::ffi::c_void>,
        lp_alias_prefix: PCSTR,
    ) -> NamespaceHandle {
        todo!("CreatePrivateNamespaceA")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreatePrivateNamespaceW(
        &self,
        lp_private_namespace_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        lp_boundary_descriptor: ConstPtr<::core::ffi::c_void>,
        lp_alias_prefix: PCWSTR,
    ) -> NamespaceHandle {
        todo!("CreatePrivateNamespaceW")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateProcessA(
        &self,
        lp_application_name: PCSTR,
        lp_command_line: PSTR,
        lp_process_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        lp_thread_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        b_inherit_handles: super::super::Foundation::BOOL,
        dw_creation_flags: PROCESS_CREATION_FLAGS,
        lp_environment: ConstPtr<::core::ffi::c_void>,
        lp_current_directory: PCSTR,
        lp_startup_info: ConstPtr<STARTUPINFOA>,
        lp_process_information: MutPtr<PROCESS_INFORMATION>,
    ) -> super::super::Foundation::BOOL {
        todo!("CreateProcessA")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateProcessAsUserA(
        &self,
        h_token: super::super::Foundation::HANDLE,
        lp_application_name: PCSTR,
        lp_command_line: PSTR,
        lp_process_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        lp_thread_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        b_inherit_handles: super::super::Foundation::BOOL,
        dw_creation_flags: u32,
        lp_environment: ConstPtr<::core::ffi::c_void>,
        lp_current_directory: PCSTR,
        lp_startup_info: ConstPtr<STARTUPINFOA>,
        lp_process_information: MutPtr<PROCESS_INFORMATION>,
    ) -> super::super::Foundation::BOOL {
        todo!("CreateProcessAsUserA")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateProcessAsUserW(
        &self,
        h_token: super::super::Foundation::HANDLE,
        lp_application_name: PCWSTR,
        lp_command_line: PWSTR,
        lp_process_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        lp_thread_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        b_inherit_handles: super::super::Foundation::BOOL,
        dw_creation_flags: u32,
        lp_environment: ConstPtr<::core::ffi::c_void>,
        lp_current_directory: PCWSTR,
        lp_startup_info: ConstPtr<STARTUPINFOW>,
        lp_process_information: MutPtr<PROCESS_INFORMATION>,
    ) -> super::super::Foundation::BOOL {
        todo!("CreateProcessAsUserW")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateProcessW(
        &self,
        lp_application_name: PCWSTR,
        lp_command_line: PWSTR,
        lp_process_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        lp_thread_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        b_inherit_handles: super::super::Foundation::BOOL,
        dw_creation_flags: PROCESS_CREATION_FLAGS,
        lp_environment: ConstPtr<::core::ffi::c_void>,
        lp_current_directory: PCWSTR,
        lp_startup_info: ConstPtr<STARTUPINFOW>,
        lp_process_information: MutPtr<PROCESS_INFORMATION>,
    ) -> super::super::Foundation::BOOL {
        todo!("CreateProcessW")
    }
    fn CreateProcessWithLogonW(
        &self,
        lp_username: PCWSTR,
        lp_domain: PCWSTR,
        lp_password: PCWSTR,
        dw_logon_flags: CREATE_PROCESS_LOGON_FLAGS,
        lp_application_name: PCWSTR,
        lp_command_line: PWSTR,
        dw_creation_flags: u32,
        lp_environment: ConstPtr<::core::ffi::c_void>,
        lp_current_directory: PCWSTR,
        lp_startup_info: ConstPtr<STARTUPINFOW>,
        lp_process_information: MutPtr<PROCESS_INFORMATION>,
    ) -> super::super::Foundation::BOOL {
        todo!("CreateProcessWithLogonW")
    }
    fn CreateProcessWithTokenW(
        &self,
        h_token: super::super::Foundation::HANDLE,
        dw_logon_flags: CREATE_PROCESS_LOGON_FLAGS,
        lp_application_name: PCWSTR,
        lp_command_line: PWSTR,
        dw_creation_flags: u32,
        lp_environment: ConstPtr<::core::ffi::c_void>,
        lp_current_directory: PCWSTR,
        lp_startup_info: ConstPtr<STARTUPINFOW>,
        lp_process_information: MutPtr<PROCESS_INFORMATION>,
    ) -> super::super::Foundation::BOOL {
        todo!("CreateProcessWithTokenW")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateRemoteThread(
        &self,
        h_process: super::super::Foundation::HANDLE,
        lp_thread_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        dw_stack_size: PtrRepr,
        lp_start_address: LPTHREAD_START_ROUTINE,
        lp_parameter: ConstPtr<::core::ffi::c_void>,
        dw_creation_flags: u32,
        lp_thread_id: MutPtr<u32>,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateRemoteThread")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateRemoteThreadEx(
        &self,
        h_process: super::super::Foundation::HANDLE,
        lp_thread_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        dw_stack_size: PtrRepr,
        lp_start_address: LPTHREAD_START_ROUTINE,
        lp_parameter: ConstPtr<::core::ffi::c_void>,
        dw_creation_flags: u32,
        lp_attribute_list: LPPROC_THREAD_ATTRIBUTE_LIST,
        lp_thread_id: MutPtr<u32>,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateRemoteThreadEx")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateSemaphoreA(
        &self,
        lp_semaphore_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        l_initial_count: i32,
        l_maximum_count: i32,
        lp_name: PCSTR,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateSemaphoreA")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateSemaphoreExA(
        &self,
        lp_semaphore_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        l_initial_count: i32,
        l_maximum_count: i32,
        lp_name: PCSTR,
        dw_flags: u32,
        dw_desired_access: u32,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateSemaphoreExA")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateSemaphoreExW(
        &self,
        lp_semaphore_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        l_initial_count: i32,
        l_maximum_count: i32,
        lp_name: PCWSTR,
        dw_flags: u32,
        dw_desired_access: u32,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateSemaphoreExW")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateSemaphoreW(
        &self,
        lp_semaphore_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        l_initial_count: i32,
        l_maximum_count: i32,
        lp_name: PCWSTR,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateSemaphoreW")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateThread(
        &self,
        lp_thread_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        dw_stack_size: PtrRepr,
        lp_start_address: LPTHREAD_START_ROUTINE,
        lp_parameter: ConstPtr<::core::ffi::c_void>,
        dw_creation_flags: THREAD_CREATION_FLAGS,
        lp_thread_id: MutPtr<u32>,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateThread")
    }
    fn CreateThreadpool(&self, reserved: MutPtr<::core::ffi::c_void>) -> PTP_POOL {
        todo!("CreateThreadpool")
    }
    fn CreateThreadpoolCleanupGroup(&self) -> PtrDiffRepr {
        todo!("CreateThreadpoolCleanupGroup")
    }
    fn CreateThreadpoolIo(
        &self,
        fl: super::super::Foundation::HANDLE,
        pfnio: PTP_WIN32_IO_CALLBACK,
        pv: MutPtr<::core::ffi::c_void>,
        pcbe: ConstPtr<TP_CALLBACK_ENVIRON_V3>,
    ) -> MutPtr<TP_IO> {
        todo!("CreateThreadpoolIo")
    }
    fn CreateThreadpoolTimer(
        &self,
        pfnti: PTP_TIMER_CALLBACK,
        pv: MutPtr<::core::ffi::c_void>,
        pcbe: ConstPtr<TP_CALLBACK_ENVIRON_V3>,
    ) -> MutPtr<TP_TIMER> {
        todo!("CreateThreadpoolTimer")
    }
    fn CreateThreadpoolWait(
        &self,
        pfnwa: PTP_WAIT_CALLBACK,
        pv: MutPtr<::core::ffi::c_void>,
        pcbe: ConstPtr<TP_CALLBACK_ENVIRON_V3>,
    ) -> MutPtr<TP_WAIT> {
        todo!("CreateThreadpoolWait")
    }
    fn CreateThreadpoolWork(
        &self,
        pfnwk: PTP_WORK_CALLBACK,
        pv: MutPtr<::core::ffi::c_void>,
        pcbe: ConstPtr<TP_CALLBACK_ENVIRON_V3>,
    ) -> MutPtr<TP_WORK> {
        todo!("CreateThreadpoolWork")
    }
    fn CreateTimerQueue(&self) -> super::super::Foundation::HANDLE {
        todo!("CreateTimerQueue")
    }
    fn CreateTimerQueueTimer(
        &self,
        ph_new_timer: MutPtr<super::super::Foundation::HANDLE>,
        timer_queue: super::super::Foundation::HANDLE,
        callback: WAITORTIMERCALLBACK,
        parameter: ConstPtr<::core::ffi::c_void>,
        due_time: u32,
        period: u32,
        flags: WORKER_THREAD_FLAGS,
    ) -> super::super::Foundation::BOOL {
        todo!("CreateTimerQueueTimer")
    }
    fn CreateUmsCompletionList(
        &self,
        ums_completion_list: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> super::super::Foundation::BOOL {
        todo!("CreateUmsCompletionList")
    }
    fn CreateUmsThreadContext(
        &self,
        lp_ums_thread: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> super::super::Foundation::BOOL {
        todo!("CreateUmsThreadContext")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateWaitableTimerExW(
        &self,
        lp_timer_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        lp_timer_name: PCWSTR,
        dw_flags: u32,
        dw_desired_access: u32,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateWaitableTimerExW")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateWaitableTimerW(
        &self,
        lp_timer_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        b_manual_reset: super::super::Foundation::BOOL,
        lp_timer_name: PCWSTR,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateWaitableTimerW")
    }
    fn DeleteBoundaryDescriptor(&self, boundary_descriptor: BoundaryDescriptorHandle) {
        todo!("DeleteBoundaryDescriptor")
    }
    fn DeleteCriticalSection(&self, lp_critical_section: MutPtr<RTL_CRITICAL_SECTION>) {
        todo!("DeleteCriticalSection")
    }
    fn DeleteFiber(&self, lp_fiber: ConstPtr<::core::ffi::c_void>) {
        todo!("DeleteFiber")
    }
    fn DeleteProcThreadAttributeList(&self, lp_attribute_list: LPPROC_THREAD_ATTRIBUTE_LIST) {
        todo!("DeleteProcThreadAttributeList")
    }
    fn DeleteSynchronizationBarrier(
        &self,
        lp_barrier: MutPtr<RTL_BARRIER>,
    ) -> super::super::Foundation::BOOL {
        todo!("DeleteSynchronizationBarrier")
    }
    fn DeleteTimerQueue(
        &self,
        timer_queue: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("DeleteTimerQueue")
    }
    fn DeleteTimerQueueEx(
        &self,
        timer_queue: super::super::Foundation::HANDLE,
        completion_event: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("DeleteTimerQueueEx")
    }
    fn DeleteTimerQueueTimer(
        &self,
        timer_queue: super::super::Foundation::HANDLE,
        timer: super::super::Foundation::HANDLE,
        completion_event: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("DeleteTimerQueueTimer")
    }
    fn DeleteUmsCompletionList(
        &self,
        ums_completion_list: ConstPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!("DeleteUmsCompletionList")
    }
    fn DeleteUmsThreadContext(
        &self,
        ums_thread: ConstPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!("DeleteUmsThreadContext")
    }
    fn DequeueUmsCompletionListItems(
        &self,
        ums_completion_list: ConstPtr<::core::ffi::c_void>,
        wait_time_out: u32,
        ums_thread_list: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> super::super::Foundation::BOOL {
        todo!("DequeueUmsCompletionListItems")
    }
    fn DisassociateCurrentThreadFromCallback(&self, pci: MutPtr<TP_CALLBACK_INSTANCE>) {
        todo!("DisassociateCurrentThreadFromCallback")
    }
    fn EnterCriticalSection(&self, lp_critical_section: MutPtr<RTL_CRITICAL_SECTION>) {
        todo!("EnterCriticalSection")
    }
    fn EnterSynchronizationBarrier(
        &self,
        lp_barrier: MutPtr<RTL_BARRIER>,
        dw_flags: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("EnterSynchronizationBarrier")
    }
    fn EnterUmsSchedulingMode(
        &self,
        scheduler_startup_info: ConstPtr<UMS_SCHEDULER_STARTUP_INFO>,
    ) -> super::super::Foundation::BOOL {
        todo!("EnterUmsSchedulingMode")
    }
    fn ExecuteUmsThread(
        &self,
        ums_thread: MutPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!("ExecuteUmsThread")
    }
    fn ExitProcess(&self, unwind_token: &mut UnwindToken, u_exit_code: u32) {
        todo!("ExitProcess")
    }
    fn ExitThread(&self, unwind_token: &mut UnwindToken, dw_exit_code: u32) {
        todo!("ExitThread")
    }
    fn FlsAlloc(&self, lp_callback: PFLS_CALLBACK_FUNCTION) -> u32 {
        todo!("FlsAlloc")
    }
    fn FlsFree(&self, dw_fls_index: u32) -> super::super::Foundation::BOOL {
        todo!("FlsFree")
    }
    fn FlsGetValue(&self, dw_fls_index: u32) -> MutPtr<::core::ffi::c_void> {
        todo!("FlsGetValue")
    }
    fn FlsSetValue(
        &self,
        dw_fls_index: u32,
        lp_fls_data: ConstPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!("FlsSetValue")
    }
    fn FlushProcessWriteBuffers(&self) {
        todo!("FlushProcessWriteBuffers")
    }
    fn FreeLibraryWhenCallbackReturns(
        &self,
        pci: MutPtr<TP_CALLBACK_INSTANCE>,
        r#mod: super::super::Foundation::HINSTANCE,
    ) {
        todo!("FreeLibraryWhenCallbackReturns")
    }
    fn GetActiveProcessorCount(&self, group_number: u16) -> u32 {
        todo!("GetActiveProcessorCount")
    }
    fn GetActiveProcessorGroupCount(&self) -> u16 {
        todo!("GetActiveProcessorGroupCount")
    }
    fn GetCurrentProcess(&self) -> super::super::Foundation::HANDLE {
        todo!("GetCurrentProcess")
    }
    fn GetCurrentProcessId(&self) -> u32 {
        todo!("GetCurrentProcessId")
    }
    fn GetCurrentProcessorNumber(&self) -> u32 {
        todo!("GetCurrentProcessorNumber")
    }
    fn GetCurrentProcessorNumberEx(&self, proc_number: MutPtr<super::Kernel::PROCESSOR_NUMBER>) {
        todo!("GetCurrentProcessorNumberEx")
    }
    fn GetCurrentThread(&self) -> super::super::Foundation::HANDLE {
        todo!("GetCurrentThread")
    }
    fn GetCurrentThreadId(&self) -> u32 {
        todo!("GetCurrentThreadId")
    }
    fn GetCurrentThreadStackLimits(&self, low_limit: MutPtr<PtrRepr>, high_limit: MutPtr<PtrRepr>) {
        todo!("GetCurrentThreadStackLimits")
    }
    fn GetCurrentUmsThread(&self) -> MutPtr<::core::ffi::c_void> {
        todo!("GetCurrentUmsThread")
    }
    fn GetExitCodeProcess(
        &self,
        h_process: super::super::Foundation::HANDLE,
        lp_exit_code: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetExitCodeProcess")
    }
    fn GetExitCodeThread(
        &self,
        h_thread: super::super::Foundation::HANDLE,
        lp_exit_code: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetExitCodeThread")
    }
    fn GetGuiResources(
        &self,
        h_process: super::super::Foundation::HANDLE,
        ui_flags: GET_GUI_RESOURCES_FLAGS,
    ) -> u32 {
        todo!("GetGuiResources")
    }
    fn GetMachineTypeAttributes(
        &self,
        machine: u16,
        machine_type_attributes: MutPtr<MACHINE_ATTRIBUTES>,
    ) -> crate::core::HRESULT {
        todo!("GetMachineTypeAttributes")
    }
    fn GetMaximumProcessorCount(&self, group_number: u16) -> u32 {
        todo!("GetMaximumProcessorCount")
    }
    fn GetMaximumProcessorGroupCount(&self) -> u16 {
        todo!("GetMaximumProcessorGroupCount")
    }
    fn GetNextUmsListItem(
        &self,
        ums_context: MutPtr<::core::ffi::c_void>,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!("GetNextUmsListItem")
    }
    fn GetNumaAvailableMemoryNode(
        &self,
        node: u8,
        available_bytes: MutPtr<u64>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetNumaAvailableMemoryNode")
    }
    fn GetNumaAvailableMemoryNodeEx(
        &self,
        node: u16,
        available_bytes: MutPtr<u64>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetNumaAvailableMemoryNodeEx")
    }
    fn GetNumaHighestNodeNumber(
        &self,
        highest_node_number: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetNumaHighestNodeNumber")
    }
    fn GetNumaNodeNumberFromHandle(
        &self,
        h_file: super::super::Foundation::HANDLE,
        node_number: MutPtr<u16>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetNumaNodeNumberFromHandle")
    }
    fn GetNumaNodeProcessorMask(
        &self,
        node: u8,
        processor_mask: MutPtr<u64>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetNumaNodeProcessorMask")
    }
    fn GetNumaNodeProcessorMask2(
        &self,
        node_number: u16,
        processor_masks: MutPtr<super::SystemInformation::GROUP_AFFINITY>,
        processor_mask_count: u16,
        required_mask_count: MutPtr<u16>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetNumaNodeProcessorMask2")
    }
    fn GetNumaNodeProcessorMaskEx(
        &self,
        node: u16,
        processor_mask: MutPtr<super::SystemInformation::GROUP_AFFINITY>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetNumaNodeProcessorMaskEx")
    }
    fn GetNumaProcessorNode(
        &self,
        processor: u8,
        node_number: MutPtr<u8>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetNumaProcessorNode")
    }
    fn GetNumaProcessorNodeEx(
        &self,
        processor: ConstPtr<super::Kernel::PROCESSOR_NUMBER>,
        node_number: MutPtr<u16>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetNumaProcessorNodeEx")
    }
    fn GetNumaProximityNode(
        &self,
        proximity_id: u32,
        node_number: MutPtr<u8>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetNumaProximityNode")
    }
    fn GetNumaProximityNodeEx(
        &self,
        proximity_id: u32,
        node_number: MutPtr<u16>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetNumaProximityNodeEx")
    }
    fn GetPriorityClass(&self, h_process: super::super::Foundation::HANDLE) -> u32 {
        todo!("GetPriorityClass")
    }
    fn GetProcessAffinityMask(
        &self,
        h_process: super::super::Foundation::HANDLE,
        lp_process_affinity_mask: MutPtr<PtrRepr>,
        lp_system_affinity_mask: MutPtr<PtrRepr>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetProcessAffinityMask")
    }
    fn GetProcessDEPPolicy(
        &self,
        h_process: super::super::Foundation::HANDLE,
        lp_flags: MutPtr<u32>,
        lp_permanent: MutPtr<super::super::Foundation::BOOL>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetProcessDEPPolicy")
    }
    fn GetProcessDefaultCpuSetMasks(
        &self,
        process: super::super::Foundation::HANDLE,
        cpu_set_masks: MutPtr<super::SystemInformation::GROUP_AFFINITY>,
        cpu_set_mask_count: u16,
        required_mask_count: MutPtr<u16>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetProcessDefaultCpuSetMasks")
    }
    fn GetProcessDefaultCpuSets(
        &self,
        process: super::super::Foundation::HANDLE,
        cpu_set_ids: MutPtr<u32>,
        cpu_set_id_count: u32,
        required_id_count: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetProcessDefaultCpuSets")
    }
    fn GetProcessGroupAffinity(
        &self,
        h_process: super::super::Foundation::HANDLE,
        group_count: MutPtr<u16>,
        group_array: MutPtr<u16>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetProcessGroupAffinity")
    }
    fn GetProcessHandleCount(
        &self,
        h_process: super::super::Foundation::HANDLE,
        pdw_handle_count: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetProcessHandleCount")
    }
    fn GetProcessId(&self, process: super::super::Foundation::HANDLE) -> u32 {
        todo!("GetProcessId")
    }
    fn GetProcessIdOfThread(&self, thread: super::super::Foundation::HANDLE) -> u32 {
        todo!("GetProcessIdOfThread")
    }
    fn GetProcessInformation(
        &self,
        h_process: super::super::Foundation::HANDLE,
        process_information_class: PROCESS_INFORMATION_CLASS,
        process_information: MutPtr<::core::ffi::c_void>,
        process_information_size: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("GetProcessInformation")
    }
    fn GetProcessIoCounters(
        &self,
        h_process: super::super::Foundation::HANDLE,
        lp_io_counters: MutPtr<IO_COUNTERS>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetProcessIoCounters")
    }
    fn GetProcessMitigationPolicy(
        &self,
        h_process: super::super::Foundation::HANDLE,
        mitigation_policy: PROCESS_MITIGATION_POLICY,
        lp_buffer: MutPtr<::core::ffi::c_void>,
        dw_length: PtrRepr,
    ) -> super::super::Foundation::BOOL {
        todo!("GetProcessMitigationPolicy")
    }
    fn GetProcessPriorityBoost(
        &self,
        h_process: super::super::Foundation::HANDLE,
        p_disable_priority_boost: MutPtr<super::super::Foundation::BOOL>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetProcessPriorityBoost")
    }
    fn GetProcessShutdownParameters(
        &self,
        lpdw_level: MutPtr<u32>,
        lpdw_flags: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetProcessShutdownParameters")
    }
    fn GetProcessTimes(
        &self,
        h_process: super::super::Foundation::HANDLE,
        lp_creation_time: MutPtr<super::super::Foundation::FILETIME>,
        lp_exit_time: MutPtr<super::super::Foundation::FILETIME>,
        lp_kernel_time: MutPtr<super::super::Foundation::FILETIME>,
        lp_user_time: MutPtr<super::super::Foundation::FILETIME>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetProcessTimes")
    }
    fn GetProcessVersion(&self, process_id: u32) -> u32 {
        todo!("GetProcessVersion")
    }
    fn GetProcessWorkingSetSize(
        &self,
        h_process: super::super::Foundation::HANDLE,
        lp_minimum_working_set_size: MutPtr<PtrRepr>,
        lp_maximum_working_set_size: MutPtr<PtrRepr>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetProcessWorkingSetSize")
    }
    fn GetStartupInfoA(&self, lp_startup_info: MutPtr<STARTUPINFOA>) {
        todo!("GetStartupInfoA")
    }
    fn GetStartupInfoW(&self, lp_startup_info: MutPtr<STARTUPINFOW>) {
        todo!("GetStartupInfoW")
    }
    fn GetSystemTimes(
        &self,
        lp_idle_time: MutPtr<super::super::Foundation::FILETIME>,
        lp_kernel_time: MutPtr<super::super::Foundation::FILETIME>,
        lp_user_time: MutPtr<super::super::Foundation::FILETIME>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetSystemTimes")
    }
    fn GetThreadDescription(
        &self,
        h_thread: super::super::Foundation::HANDLE,
        ppsz_thread_description: MutPtr<PWSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetThreadDescription")
    }
    fn GetThreadGroupAffinity(
        &self,
        h_thread: super::super::Foundation::HANDLE,
        group_affinity: MutPtr<super::SystemInformation::GROUP_AFFINITY>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetThreadGroupAffinity")
    }
    fn GetThreadIOPendingFlag(
        &self,
        h_thread: super::super::Foundation::HANDLE,
        lp_io_is_pending: MutPtr<super::super::Foundation::BOOL>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetThreadIOPendingFlag")
    }
    fn GetThreadId(&self, thread: super::super::Foundation::HANDLE) -> u32 {
        todo!("GetThreadId")
    }
    fn GetThreadIdealProcessorEx(
        &self,
        h_thread: super::super::Foundation::HANDLE,
        lp_ideal_processor: MutPtr<super::Kernel::PROCESSOR_NUMBER>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetThreadIdealProcessorEx")
    }
    fn GetThreadInformation(
        &self,
        h_thread: super::super::Foundation::HANDLE,
        thread_information_class: THREAD_INFORMATION_CLASS,
        thread_information: MutPtr<::core::ffi::c_void>,
        thread_information_size: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("GetThreadInformation")
    }
    fn GetThreadPriority(&self, h_thread: super::super::Foundation::HANDLE) -> i32 {
        todo!("GetThreadPriority")
    }
    fn GetThreadPriorityBoost(
        &self,
        h_thread: super::super::Foundation::HANDLE,
        p_disable_priority_boost: MutPtr<super::super::Foundation::BOOL>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetThreadPriorityBoost")
    }
    fn GetThreadSelectedCpuSetMasks(
        &self,
        thread: super::super::Foundation::HANDLE,
        cpu_set_masks: MutPtr<super::SystemInformation::GROUP_AFFINITY>,
        cpu_set_mask_count: u16,
        required_mask_count: MutPtr<u16>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetThreadSelectedCpuSetMasks")
    }
    fn GetThreadSelectedCpuSets(
        &self,
        thread: super::super::Foundation::HANDLE,
        cpu_set_ids: MutPtr<u32>,
        cpu_set_id_count: u32,
        required_id_count: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetThreadSelectedCpuSets")
    }
    fn GetThreadTimes(
        &self,
        h_thread: super::super::Foundation::HANDLE,
        lp_creation_time: MutPtr<super::super::Foundation::FILETIME>,
        lp_exit_time: MutPtr<super::super::Foundation::FILETIME>,
        lp_kernel_time: MutPtr<super::super::Foundation::FILETIME>,
        lp_user_time: MutPtr<super::super::Foundation::FILETIME>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetThreadTimes")
    }
    fn GetUmsCompletionListEvent(
        &self,
        ums_completion_list: ConstPtr<::core::ffi::c_void>,
        ums_completion_event: MutPtr<super::super::Foundation::HANDLE>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetUmsCompletionListEvent")
    }
    fn GetUmsSystemThreadInformation(
        &self,
        thread_handle: super::super::Foundation::HANDLE,
        system_thread_info: MutPtr<UMS_SYSTEM_THREAD_INFORMATION>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetUmsSystemThreadInformation")
    }
    fn InitOnceBeginInitialize(
        &self,
        lp_init_once: MutPtr<RTL_RUN_ONCE>,
        dw_flags: u32,
        f_pending: MutPtr<super::super::Foundation::BOOL>,
        lp_context: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> super::super::Foundation::BOOL {
        todo!("InitOnceBeginInitialize")
    }
    fn InitOnceComplete(
        &self,
        lp_init_once: MutPtr<RTL_RUN_ONCE>,
        dw_flags: u32,
        lp_context: ConstPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!("InitOnceComplete")
    }
    fn InitOnceExecuteOnce(
        &self,
        init_once: MutPtr<RTL_RUN_ONCE>,
        init_fn: PINIT_ONCE_FN,
        parameter: MutPtr<::core::ffi::c_void>,
        context: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> super::super::Foundation::BOOL {
        todo!("InitOnceExecuteOnce")
    }
    fn InitOnceInitialize(&self, init_once: MutPtr<RTL_RUN_ONCE>) {
        todo!("InitOnceInitialize")
    }
    fn InitializeConditionVariable(&self, condition_variable: MutPtr<RTL_CONDITION_VARIABLE>) {
        todo!("InitializeConditionVariable")
    }
    fn InitializeCriticalSection(&self, lp_critical_section: MutPtr<RTL_CRITICAL_SECTION>) {
        todo!("InitializeCriticalSection")
    }
    fn InitializeCriticalSectionAndSpinCount(
        &self,
        lp_critical_section: MutPtr<RTL_CRITICAL_SECTION>,
        dw_spin_count: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("InitializeCriticalSectionAndSpinCount")
    }
    fn InitializeCriticalSectionEx(
        &self,
        lp_critical_section: MutPtr<RTL_CRITICAL_SECTION>,
        dw_spin_count: u32,
        flags: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("InitializeCriticalSectionEx")
    }
    fn InitializeProcThreadAttributeList(
        &self,
        lp_attribute_list: LPPROC_THREAD_ATTRIBUTE_LIST,
        dw_attribute_count: u32,
        dw_flags: u32,
        lp_size: MutPtr<PtrRepr>,
    ) -> super::super::Foundation::BOOL {
        todo!("InitializeProcThreadAttributeList")
    }
    fn InitializeSListHead(&self, list_head: MutPtr<super::Kernel::SLIST_HEADER>) {
        todo!("InitializeSListHead")
    }
    fn InitializeSRWLock(&self, srw_lock: MutPtr<RTL_SRWLOCK>) {
        todo!("InitializeSRWLock")
    }
    fn InitializeSynchronizationBarrier(
        &self,
        lp_barrier: MutPtr<RTL_BARRIER>,
        l_total_threads: i32,
        l_spin_count: i32,
    ) -> super::super::Foundation::BOOL {
        todo!("InitializeSynchronizationBarrier")
    }
    fn InterlockedFlushSList(
        &self,
        list_head: MutPtr<super::Kernel::SLIST_HEADER>,
    ) -> MutPtr<super::Kernel::SLIST_ENTRY> {
        todo!("InterlockedFlushSList")
    }
    fn InterlockedPopEntrySList(
        &self,
        list_head: MutPtr<super::Kernel::SLIST_HEADER>,
    ) -> MutPtr<super::Kernel::SLIST_ENTRY> {
        todo!("InterlockedPopEntrySList")
    }
    fn InterlockedPushEntrySList(
        &self,
        list_head: MutPtr<super::Kernel::SLIST_HEADER>,
        list_entry: MutPtr<super::Kernel::SLIST_ENTRY>,
    ) -> MutPtr<super::Kernel::SLIST_ENTRY> {
        todo!("InterlockedPushEntrySList")
    }
    fn InterlockedPushListSListEx(
        &self,
        list_head: MutPtr<super::Kernel::SLIST_HEADER>,
        list: MutPtr<super::Kernel::SLIST_ENTRY>,
        list_end: MutPtr<super::Kernel::SLIST_ENTRY>,
        count: u32,
    ) -> MutPtr<super::Kernel::SLIST_ENTRY> {
        todo!("InterlockedPushListSListEx")
    }
    fn IsImmersiveProcess(
        &self,
        h_process: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("IsImmersiveProcess")
    }
    fn IsProcessCritical(
        &self,
        h_process: super::super::Foundation::HANDLE,
        critical: MutPtr<super::super::Foundation::BOOL>,
    ) -> super::super::Foundation::BOOL {
        todo!("IsProcessCritical")
    }
    fn IsProcessorFeaturePresent(
        &self,
        processor_feature: PROCESSOR_FEATURE_ID,
    ) -> super::super::Foundation::BOOL {
        todo!("IsProcessorFeaturePresent")
    }
    fn IsThreadAFiber(&self) -> super::super::Foundation::BOOL {
        todo!("IsThreadAFiber")
    }
    fn IsThreadpoolTimerSet(&self, pti: MutPtr<TP_TIMER>) -> super::super::Foundation::BOOL {
        todo!("IsThreadpoolTimerSet")
    }
    fn IsWow64Process(
        &self,
        h_process: super::super::Foundation::HANDLE,
        wow_64_process: MutPtr<super::super::Foundation::BOOL>,
    ) -> super::super::Foundation::BOOL {
        todo!("IsWow64Process")
    }
    fn IsWow64Process2(
        &self,
        h_process: super::super::Foundation::HANDLE,
        p_process_machine: MutPtr<u16>,
        p_native_machine: MutPtr<u16>,
    ) -> super::super::Foundation::BOOL {
        todo!("IsWow64Process2")
    }
    fn LeaveCriticalSection(&self, lp_critical_section: MutPtr<RTL_CRITICAL_SECTION>) {
        todo!("LeaveCriticalSection")
    }
    fn LeaveCriticalSectionWhenCallbackReturns(
        &self,
        pci: MutPtr<TP_CALLBACK_INSTANCE>,
        pcs: MutPtr<RTL_CRITICAL_SECTION>,
    ) {
        todo!("LeaveCriticalSectionWhenCallbackReturns")
    }
    fn NtQueryInformationProcess(
        &self,
        process_handle: super::super::Foundation::HANDLE,
        process_information_class: PROCESSINFOCLASS,
        process_information: MutPtr<::core::ffi::c_void>,
        process_information_length: u32,
        return_length: MutPtr<u32>,
    ) -> super::super::Foundation::NTSTATUS {
        todo!("NtQueryInformationProcess")
    }
    fn NtQueryInformationThread(
        &self,
        thread_handle: super::super::Foundation::HANDLE,
        thread_information_class: THREADINFOCLASS,
        thread_information: MutPtr<::core::ffi::c_void>,
        thread_information_length: u32,
        return_length: MutPtr<u32>,
    ) -> super::super::Foundation::NTSTATUS {
        todo!("NtQueryInformationThread")
    }
    fn NtSetInformationThread(
        &self,
        thread_handle: super::super::Foundation::HANDLE,
        thread_information_class: THREADINFOCLASS,
        thread_information: ConstPtr<::core::ffi::c_void>,
        thread_information_length: u32,
    ) -> super::super::Foundation::NTSTATUS {
        todo!("NtSetInformationThread")
    }
    fn OpenEventA(
        &self,
        dw_desired_access: u32,
        b_inherit_handle: super::super::Foundation::BOOL,
        lp_name: PCSTR,
    ) -> super::super::Foundation::HANDLE {
        todo!("OpenEventA")
    }
    fn OpenEventW(
        &self,
        dw_desired_access: u32,
        b_inherit_handle: super::super::Foundation::BOOL,
        lp_name: PCWSTR,
    ) -> super::super::Foundation::HANDLE {
        todo!("OpenEventW")
    }
    fn OpenMutexW(
        &self,
        dw_desired_access: u32,
        b_inherit_handle: super::super::Foundation::BOOL,
        lp_name: PCWSTR,
    ) -> super::super::Foundation::HANDLE {
        todo!("OpenMutexW")
    }
    fn OpenPrivateNamespaceA(
        &self,
        lp_boundary_descriptor: ConstPtr<::core::ffi::c_void>,
        lp_alias_prefix: PCSTR,
    ) -> NamespaceHandle {
        todo!("OpenPrivateNamespaceA")
    }
    fn OpenPrivateNamespaceW(
        &self,
        lp_boundary_descriptor: ConstPtr<::core::ffi::c_void>,
        lp_alias_prefix: PCWSTR,
    ) -> NamespaceHandle {
        todo!("OpenPrivateNamespaceW")
    }
    fn OpenProcess(
        &self,
        dw_desired_access: PROCESS_ACCESS_RIGHTS,
        b_inherit_handle: super::super::Foundation::BOOL,
        dw_process_id: u32,
    ) -> super::super::Foundation::HANDLE {
        todo!("OpenProcess")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn OpenProcessToken(
        &self,
        process_handle: super::super::Foundation::HANDLE,
        desired_access: super::super::Security::TOKEN_ACCESS_MASK,
        token_handle: MutPtr<super::super::Foundation::HANDLE>,
    ) -> super::super::Foundation::BOOL {
        todo!("OpenProcessToken")
    }
    fn OpenSemaphoreW(
        &self,
        dw_desired_access: u32,
        b_inherit_handle: super::super::Foundation::BOOL,
        lp_name: PCWSTR,
    ) -> super::super::Foundation::HANDLE {
        todo!("OpenSemaphoreW")
    }
    fn OpenThread(
        &self,
        dw_desired_access: THREAD_ACCESS_RIGHTS,
        b_inherit_handle: super::super::Foundation::BOOL,
        dw_thread_id: u32,
    ) -> super::super::Foundation::HANDLE {
        todo!("OpenThread")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn OpenThreadToken(
        &self,
        thread_handle: super::super::Foundation::HANDLE,
        desired_access: super::super::Security::TOKEN_ACCESS_MASK,
        open_as_self: super::super::Foundation::BOOL,
        token_handle: MutPtr<super::super::Foundation::HANDLE>,
    ) -> super::super::Foundation::BOOL {
        todo!("OpenThreadToken")
    }
    fn OpenWaitableTimerW(
        &self,
        dw_desired_access: u32,
        b_inherit_handle: super::super::Foundation::BOOL,
        lp_timer_name: PCWSTR,
    ) -> super::super::Foundation::HANDLE {
        todo!("OpenWaitableTimerW")
    }
    fn PulseEvent(
        &self,
        h_event: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("PulseEvent")
    }
    fn QueryDepthSList(&self, list_head: ConstPtr<super::Kernel::SLIST_HEADER>) -> u16 {
        todo!("QueryDepthSList")
    }
    fn QueryFullProcessImageNameA(
        &self,
        h_process: super::super::Foundation::HANDLE,
        dw_flags: PROCESS_NAME_FORMAT,
        lp_exe_name: PSTR,
        lpdw_size: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("QueryFullProcessImageNameA")
    }
    fn QueryFullProcessImageNameW(
        &self,
        h_process: super::super::Foundation::HANDLE,
        dw_flags: PROCESS_NAME_FORMAT,
        lp_exe_name: PWSTR,
        lpdw_size: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("QueryFullProcessImageNameW")
    }
    fn QueryProcessAffinityUpdateMode(
        &self,
        h_process: super::super::Foundation::HANDLE,
        lpdw_flags: MutPtr<PROCESS_AFFINITY_AUTO_UPDATE_FLAGS>,
    ) -> super::super::Foundation::BOOL {
        todo!("QueryProcessAffinityUpdateMode")
    }
    fn QueryProtectedPolicy(
        &self,
        policy_guid: ConstPtr<crate::core::GUID>,
        policy_value: MutPtr<PtrRepr>,
    ) -> super::super::Foundation::BOOL {
        todo!("QueryProtectedPolicy")
    }
    fn QueryThreadpoolStackInformation(
        &self,
        ptpp: PTP_POOL,
        ptpsi: MutPtr<TP_POOL_STACK_INFORMATION>,
    ) -> super::super::Foundation::BOOL {
        todo!("QueryThreadpoolStackInformation")
    }
    fn QueryUmsThreadInformation(
        &self,
        ums_thread: ConstPtr<::core::ffi::c_void>,
        ums_thread_info_class: RTL_UMS_THREAD_INFO_CLASS,
        ums_thread_information: MutPtr<::core::ffi::c_void>,
        ums_thread_information_length: u32,
        return_length: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("QueryUmsThreadInformation")
    }
    fn QueueUserAPC(
        &self,
        pfn_apc: super::super::Foundation::PAPCFUNC,
        h_thread: super::super::Foundation::HANDLE,
        dw_data: PtrRepr,
    ) -> u32 {
        todo!("QueueUserAPC")
    }
    fn QueueUserAPC2(
        &self,
        apc_routine: super::super::Foundation::PAPCFUNC,
        thread: super::super::Foundation::HANDLE,
        data: PtrRepr,
        flags: QUEUE_USER_APC_FLAGS,
    ) -> super::super::Foundation::BOOL {
        todo!("QueueUserAPC2")
    }
    fn QueueUserWorkItem(
        &self,
        function: LPTHREAD_START_ROUTINE,
        context: ConstPtr<::core::ffi::c_void>,
        flags: WORKER_THREAD_FLAGS,
    ) -> super::super::Foundation::BOOL {
        todo!("QueueUserWorkItem")
    }
    fn RegisterWaitForSingleObject(
        &self,
        ph_new_wait_object: MutPtr<super::super::Foundation::HANDLE>,
        h_object: super::super::Foundation::HANDLE,
        callback: WAITORTIMERCALLBACK,
        context: ConstPtr<::core::ffi::c_void>,
        dw_milliseconds: u32,
        dw_flags: WORKER_THREAD_FLAGS,
    ) -> super::super::Foundation::BOOL {
        todo!("RegisterWaitForSingleObject")
    }
    fn ReleaseMutex(
        &self,
        h_mutex: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("ReleaseMutex")
    }
    fn ReleaseMutexWhenCallbackReturns(
        &self,
        pci: MutPtr<TP_CALLBACK_INSTANCE>,
        r#mut: super::super::Foundation::HANDLE,
    ) {
        todo!("ReleaseMutexWhenCallbackReturns")
    }
    fn ReleaseSRWLockExclusive(&self, srw_lock: MutPtr<RTL_SRWLOCK>) {
        todo!("ReleaseSRWLockExclusive")
    }
    fn ReleaseSRWLockShared(&self, srw_lock: MutPtr<RTL_SRWLOCK>) {
        todo!("ReleaseSRWLockShared")
    }
    fn ReleaseSemaphore(
        &self,
        h_semaphore: super::super::Foundation::HANDLE,
        l_release_count: i32,
        lp_previous_count: MutPtr<i32>,
    ) -> super::super::Foundation::BOOL {
        todo!("ReleaseSemaphore")
    }
    fn ReleaseSemaphoreWhenCallbackReturns(
        &self,
        pci: MutPtr<TP_CALLBACK_INSTANCE>,
        sem: super::super::Foundation::HANDLE,
        crel: u32,
    ) {
        todo!("ReleaseSemaphoreWhenCallbackReturns")
    }
    fn ResetEvent(
        &self,
        h_event: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("ResetEvent")
    }
    fn ResumeThread(&self, h_thread: super::super::Foundation::HANDLE) -> u32 {
        todo!("ResumeThread")
    }
    fn SetCriticalSectionSpinCount(
        &self,
        lp_critical_section: MutPtr<RTL_CRITICAL_SECTION>,
        dw_spin_count: u32,
    ) -> u32 {
        todo!("SetCriticalSectionSpinCount")
    }
    fn SetEvent(
        &self,
        h_event: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("SetEvent")
    }
    fn SetEventWhenCallbackReturns(
        &self,
        pci: MutPtr<TP_CALLBACK_INSTANCE>,
        evt: super::super::Foundation::HANDLE,
    ) {
        todo!("SetEventWhenCallbackReturns")
    }
    fn SetPriorityClass(
        &self,
        h_process: super::super::Foundation::HANDLE,
        dw_priority_class: PROCESS_CREATION_FLAGS,
    ) -> super::super::Foundation::BOOL {
        todo!("SetPriorityClass")
    }
    fn SetProcessAffinityMask(
        &self,
        h_process: super::super::Foundation::HANDLE,
        dw_process_affinity_mask: PtrRepr,
    ) -> super::super::Foundation::BOOL {
        todo!("SetProcessAffinityMask")
    }
    fn SetProcessAffinityUpdateMode(
        &self,
        h_process: super::super::Foundation::HANDLE,
        dw_flags: PROCESS_AFFINITY_AUTO_UPDATE_FLAGS,
    ) -> super::super::Foundation::BOOL {
        todo!("SetProcessAffinityUpdateMode")
    }
    fn SetProcessDEPPolicy(&self, dw_flags: PROCESS_DEP_FLAGS) -> super::super::Foundation::BOOL {
        todo!("SetProcessDEPPolicy")
    }
    fn SetProcessDefaultCpuSetMasks(
        &self,
        process: super::super::Foundation::HANDLE,
        cpu_set_masks: ConstPtr<super::SystemInformation::GROUP_AFFINITY>,
        cpu_set_mask_count: u16,
    ) -> super::super::Foundation::BOOL {
        todo!("SetProcessDefaultCpuSetMasks")
    }
    fn SetProcessDefaultCpuSets(
        &self,
        process: super::super::Foundation::HANDLE,
        cpu_set_ids: ConstPtr<u32>,
        cpu_set_id_count: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("SetProcessDefaultCpuSets")
    }
    fn SetProcessDynamicEHContinuationTargets(
        &self,
        process: super::super::Foundation::HANDLE,
        number_of_targets: u16,
        targets: MutPtr<PROCESS_DYNAMIC_EH_CONTINUATION_TARGET>,
    ) -> super::super::Foundation::BOOL {
        todo!("SetProcessDynamicEHContinuationTargets")
    }
    fn SetProcessDynamicEnforcedCetCompatibleRanges(
        &self,
        process: super::super::Foundation::HANDLE,
        number_of_ranges: u16,
        ranges: MutPtr<PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE>,
    ) -> super::super::Foundation::BOOL {
        todo!("SetProcessDynamicEnforcedCetCompatibleRanges")
    }
    fn SetProcessInformation(
        &self,
        h_process: super::super::Foundation::HANDLE,
        process_information_class: PROCESS_INFORMATION_CLASS,
        process_information: ConstPtr<::core::ffi::c_void>,
        process_information_size: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("SetProcessInformation")
    }
    fn SetProcessMitigationPolicy(
        &self,
        mitigation_policy: PROCESS_MITIGATION_POLICY,
        lp_buffer: ConstPtr<::core::ffi::c_void>,
        dw_length: PtrRepr,
    ) -> super::super::Foundation::BOOL {
        todo!("SetProcessMitigationPolicy")
    }
    fn SetProcessPriorityBoost(
        &self,
        h_process: super::super::Foundation::HANDLE,
        b_disable_priority_boost: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!("SetProcessPriorityBoost")
    }
    fn SetProcessRestrictionExemption(
        &self,
        f_enable_exemption: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!("SetProcessRestrictionExemption")
    }
    fn SetProcessShutdownParameters(
        &self,
        dw_level: u32,
        dw_flags: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("SetProcessShutdownParameters")
    }
    fn SetProcessWorkingSetSize(
        &self,
        h_process: super::super::Foundation::HANDLE,
        dw_minimum_working_set_size: PtrRepr,
        dw_maximum_working_set_size: PtrRepr,
    ) -> super::super::Foundation::BOOL {
        todo!("SetProcessWorkingSetSize")
    }
    fn SetProtectedPolicy(
        &self,
        policy_guid: ConstPtr<crate::core::GUID>,
        policy_value: PtrRepr,
        old_policy_value: MutPtr<PtrRepr>,
    ) -> super::super::Foundation::BOOL {
        todo!("SetProtectedPolicy")
    }
    fn SetThreadAffinityMask(
        &self,
        h_thread: super::super::Foundation::HANDLE,
        dw_thread_affinity_mask: PtrRepr,
    ) -> PtrRepr {
        todo!("SetThreadAffinityMask")
    }
    fn SetThreadDescription(
        &self,
        h_thread: super::super::Foundation::HANDLE,
        lp_thread_description: PCWSTR,
    ) -> crate::core::HRESULT {
        todo!("SetThreadDescription")
    }
    fn SetThreadGroupAffinity(
        &self,
        h_thread: super::super::Foundation::HANDLE,
        group_affinity: ConstPtr<super::SystemInformation::GROUP_AFFINITY>,
        previous_group_affinity: MutPtr<super::SystemInformation::GROUP_AFFINITY>,
    ) -> super::super::Foundation::BOOL {
        todo!("SetThreadGroupAffinity")
    }
    fn SetThreadIdealProcessor(
        &self,
        h_thread: super::super::Foundation::HANDLE,
        dw_ideal_processor: u32,
    ) -> u32 {
        todo!("SetThreadIdealProcessor")
    }
    fn SetThreadIdealProcessorEx(
        &self,
        h_thread: super::super::Foundation::HANDLE,
        lp_ideal_processor: ConstPtr<super::Kernel::PROCESSOR_NUMBER>,
        lp_previous_ideal_processor: MutPtr<super::Kernel::PROCESSOR_NUMBER>,
    ) -> super::super::Foundation::BOOL {
        todo!("SetThreadIdealProcessorEx")
    }
    fn SetThreadInformation(
        &self,
        h_thread: super::super::Foundation::HANDLE,
        thread_information_class: THREAD_INFORMATION_CLASS,
        thread_information: ConstPtr<::core::ffi::c_void>,
        thread_information_size: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("SetThreadInformation")
    }
    fn SetThreadPriority(
        &self,
        h_thread: super::super::Foundation::HANDLE,
        n_priority: THREAD_PRIORITY,
    ) -> super::super::Foundation::BOOL {
        todo!("SetThreadPriority")
    }
    fn SetThreadPriorityBoost(
        &self,
        h_thread: super::super::Foundation::HANDLE,
        b_disable_priority_boost: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!("SetThreadPriorityBoost")
    }
    fn SetThreadSelectedCpuSetMasks(
        &self,
        thread: super::super::Foundation::HANDLE,
        cpu_set_masks: ConstPtr<super::SystemInformation::GROUP_AFFINITY>,
        cpu_set_mask_count: u16,
    ) -> super::super::Foundation::BOOL {
        todo!("SetThreadSelectedCpuSetMasks")
    }
    fn SetThreadSelectedCpuSets(
        &self,
        thread: super::super::Foundation::HANDLE,
        cpu_set_ids: ConstPtr<u32>,
        cpu_set_id_count: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("SetThreadSelectedCpuSets")
    }
    fn SetThreadStackGuarantee(
        &self,
        stack_size_in_bytes: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("SetThreadStackGuarantee")
    }
    fn SetThreadToken(
        &self,
        thread: ConstPtr<super::super::Foundation::HANDLE>,
        token: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("SetThreadToken")
    }
    fn SetThreadpoolStackInformation(
        &self,
        ptpp: PTP_POOL,
        ptpsi: ConstPtr<TP_POOL_STACK_INFORMATION>,
    ) -> super::super::Foundation::BOOL {
        todo!("SetThreadpoolStackInformation")
    }
    fn SetThreadpoolThreadMaximum(&self, ptpp: PTP_POOL, cthrd_most: u32) {
        todo!("SetThreadpoolThreadMaximum")
    }
    fn SetThreadpoolThreadMinimum(
        &self,
        ptpp: PTP_POOL,
        cthrd_mic: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("SetThreadpoolThreadMinimum")
    }
    fn SetThreadpoolTimer(
        &self,
        pti: MutPtr<TP_TIMER>,
        pft_due_time: ConstPtr<super::super::Foundation::FILETIME>,
        ms_period: u32,
        ms_window_length: u32,
    ) {
        todo!("SetThreadpoolTimer")
    }
    fn SetThreadpoolTimerEx(
        &self,
        pti: MutPtr<TP_TIMER>,
        pft_due_time: ConstPtr<super::super::Foundation::FILETIME>,
        ms_period: u32,
        ms_window_length: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("SetThreadpoolTimerEx")
    }
    fn SetThreadpoolWait(
        &self,
        pwa: MutPtr<TP_WAIT>,
        h: super::super::Foundation::HANDLE,
        pft_timeout: ConstPtr<super::super::Foundation::FILETIME>,
    ) {
        todo!("SetThreadpoolWait")
    }
    fn SetThreadpoolWaitEx(
        &self,
        pwa: MutPtr<TP_WAIT>,
        h: super::super::Foundation::HANDLE,
        pft_timeout: ConstPtr<super::super::Foundation::FILETIME>,
        reserved: MutPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!("SetThreadpoolWaitEx")
    }
    fn SetTimerQueueTimer(
        &self,
        timer_queue: super::super::Foundation::HANDLE,
        callback: WAITORTIMERCALLBACK,
        parameter: ConstPtr<::core::ffi::c_void>,
        due_time: u32,
        period: u32,
        prefer_io: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::HANDLE {
        todo!("SetTimerQueueTimer")
    }
    fn SetUmsThreadInformation(
        &self,
        ums_thread: ConstPtr<::core::ffi::c_void>,
        ums_thread_info_class: RTL_UMS_THREAD_INFO_CLASS,
        ums_thread_information: ConstPtr<::core::ffi::c_void>,
        ums_thread_information_length: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("SetUmsThreadInformation")
    }
    fn SetWaitableTimer(
        &self,
        h_timer: super::super::Foundation::HANDLE,
        lp_due_time: ConstPtr<i64>,
        l_period: i32,
        pfn_completion_routine: PTIMERAPCROUTINE,
        lp_arg_to_completion_routine: ConstPtr<::core::ffi::c_void>,
        f_resume: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!("SetWaitableTimer")
    }
    fn SetWaitableTimerEx(
        &self,
        h_timer: super::super::Foundation::HANDLE,
        lp_due_time: ConstPtr<i64>,
        l_period: i32,
        pfn_completion_routine: PTIMERAPCROUTINE,
        lp_arg_to_completion_routine: ConstPtr<::core::ffi::c_void>,
        wake_context: ConstPtr<REASON_CONTEXT>,
        tolerable_delay: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("SetWaitableTimerEx")
    }
    fn Sleep(&self, dw_milliseconds: u32) {
        todo!("Sleep")
    }
    fn SleepConditionVariableCS(
        &self,
        condition_variable: MutPtr<RTL_CONDITION_VARIABLE>,
        critical_section: MutPtr<RTL_CRITICAL_SECTION>,
        dw_milliseconds: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("SleepConditionVariableCS")
    }
    fn SleepConditionVariableSRW(
        &self,
        condition_variable: MutPtr<RTL_CONDITION_VARIABLE>,
        srw_lock: MutPtr<RTL_SRWLOCK>,
        dw_milliseconds: u32,
        flags: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("SleepConditionVariableSRW")
    }
    fn SleepEx(&self, dw_milliseconds: u32, b_alertable: super::super::Foundation::BOOL) -> u32 {
        todo!("SleepEx")
    }
    fn StartThreadpoolIo(&self, pio: MutPtr<TP_IO>) {
        todo!("StartThreadpoolIo")
    }
    fn SubmitThreadpoolWork(&self, pwk: MutPtr<TP_WORK>) {
        todo!("SubmitThreadpoolWork")
    }
    fn SuspendThread(&self, h_thread: super::super::Foundation::HANDLE) -> u32 {
        todo!("SuspendThread")
    }
    fn SwitchToFiber(&self, lp_fiber: ConstPtr<::core::ffi::c_void>) {
        todo!("SwitchToFiber")
    }
    fn SwitchToThread(&self) -> super::super::Foundation::BOOL {
        todo!("SwitchToThread")
    }
    fn TerminateProcess(
        &self,
        h_process: super::super::Foundation::HANDLE,
        u_exit_code: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("TerminateProcess")
    }
    fn TerminateThread(
        &self,
        h_thread: super::super::Foundation::HANDLE,
        dw_exit_code: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("TerminateThread")
    }
    fn TlsAlloc(&self) -> u32 {
        todo!("TlsAlloc")
    }
    fn TlsFree(&self, dw_tls_index: u32) -> super::super::Foundation::BOOL {
        todo!("TlsFree")
    }
    fn TlsGetValue(&self, dw_tls_index: u32) -> MutPtr<::core::ffi::c_void> {
        todo!("TlsGetValue")
    }
    fn TlsSetValue(
        &self,
        dw_tls_index: u32,
        lp_tls_value: ConstPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!("TlsSetValue")
    }
    fn TryAcquireSRWLockExclusive(
        &self,
        srw_lock: MutPtr<RTL_SRWLOCK>,
    ) -> super::super::Foundation::BOOLEAN {
        todo!("TryAcquireSRWLockExclusive")
    }
    fn TryAcquireSRWLockShared(
        &self,
        srw_lock: MutPtr<RTL_SRWLOCK>,
    ) -> super::super::Foundation::BOOLEAN {
        todo!("TryAcquireSRWLockShared")
    }
    fn TryEnterCriticalSection(
        &self,
        lp_critical_section: MutPtr<RTL_CRITICAL_SECTION>,
    ) -> super::super::Foundation::BOOL {
        todo!("TryEnterCriticalSection")
    }
    fn TrySubmitThreadpoolCallback(
        &self,
        pfns: PTP_SIMPLE_CALLBACK,
        pv: MutPtr<::core::ffi::c_void>,
        pcbe: ConstPtr<TP_CALLBACK_ENVIRON_V3>,
    ) -> super::super::Foundation::BOOL {
        todo!("TrySubmitThreadpoolCallback")
    }
    fn UmsThreadYield(
        &self,
        scheduler_param: ConstPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!("UmsThreadYield")
    }
    fn UnregisterWait(
        &self,
        wait_handle: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("UnregisterWait")
    }
    fn UnregisterWaitEx(
        &self,
        wait_handle: super::super::Foundation::HANDLE,
        completion_event: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("UnregisterWaitEx")
    }
    fn UpdateProcThreadAttribute(
        &self,
        lp_attribute_list: LPPROC_THREAD_ATTRIBUTE_LIST,
        dw_flags: u32,
        attribute: PtrRepr,
        lp_value: ConstPtr<::core::ffi::c_void>,
        cb_size: PtrRepr,
        lp_previous_value: MutPtr<::core::ffi::c_void>,
        lp_return_size: ConstPtr<PtrRepr>,
    ) -> super::super::Foundation::BOOL {
        todo!("UpdateProcThreadAttribute")
    }
    fn WaitForInputIdle(
        &self,
        h_process: super::super::Foundation::HANDLE,
        dw_milliseconds: u32,
    ) -> u32 {
        todo!("WaitForInputIdle")
    }
    fn WaitForMultipleObjects(
        &self,
        n_count: u32,
        lp_handles: ConstPtr<super::super::Foundation::HANDLE>,
        b_wait_all: super::super::Foundation::BOOL,
        dw_milliseconds: u32,
    ) -> u32 {
        todo!("WaitForMultipleObjects")
    }
    fn WaitForMultipleObjectsEx(
        &self,
        n_count: u32,
        lp_handles: ConstPtr<super::super::Foundation::HANDLE>,
        b_wait_all: super::super::Foundation::BOOL,
        dw_milliseconds: u32,
        b_alertable: super::super::Foundation::BOOL,
    ) -> u32 {
        todo!("WaitForMultipleObjectsEx")
    }
    fn WaitForSingleObject(
        &self,
        h_handle: super::super::Foundation::HANDLE,
        dw_milliseconds: u32,
    ) -> u32 {
        todo!("WaitForSingleObject")
    }
    fn WaitForSingleObjectEx(
        &self,
        h_handle: super::super::Foundation::HANDLE,
        dw_milliseconds: u32,
        b_alertable: super::super::Foundation::BOOL,
    ) -> u32 {
        todo!("WaitForSingleObjectEx")
    }
    fn WaitForThreadpoolIoCallbacks(
        &self,
        pio: MutPtr<TP_IO>,
        f_cancel_pending_callbacks: super::super::Foundation::BOOL,
    ) {
        todo!("WaitForThreadpoolIoCallbacks")
    }
    fn WaitForThreadpoolTimerCallbacks(
        &self,
        pti: MutPtr<TP_TIMER>,
        f_cancel_pending_callbacks: super::super::Foundation::BOOL,
    ) {
        todo!("WaitForThreadpoolTimerCallbacks")
    }
    fn WaitForThreadpoolWaitCallbacks(
        &self,
        pwa: MutPtr<TP_WAIT>,
        f_cancel_pending_callbacks: super::super::Foundation::BOOL,
    ) {
        todo!("WaitForThreadpoolWaitCallbacks")
    }
    fn WaitForThreadpoolWorkCallbacks(
        &self,
        pwk: MutPtr<TP_WORK>,
        f_cancel_pending_callbacks: super::super::Foundation::BOOL,
    ) {
        todo!("WaitForThreadpoolWorkCallbacks")
    }
    fn WaitOnAddress(
        &self,
        address: ConstPtr<::core::ffi::c_void>,
        compare_address: ConstPtr<::core::ffi::c_void>,
        address_size: PtrRepr,
        dw_milliseconds: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("WaitOnAddress")
    }
    fn WakeAllConditionVariable(&self, condition_variable: MutPtr<RTL_CONDITION_VARIABLE>) {
        todo!("WakeAllConditionVariable")
    }
    fn WakeByAddressAll(&self, address: ConstPtr<::core::ffi::c_void>) {
        todo!("WakeByAddressAll")
    }
    fn WakeByAddressSingle(&self, address: ConstPtr<::core::ffi::c_void>) {
        todo!("WakeByAddressSingle")
    }
    fn WakeConditionVariable(&self, condition_variable: MutPtr<RTL_CONDITION_VARIABLE>) {
        todo!("WakeConditionVariable")
    }
    fn WinExec(&self, lp_cmd_line: PCSTR, u_cmd_show: u32) -> u32 {
        todo!("WinExec")
    }
    fn Wow64SetThreadDefaultGuestMachine(&self, machine: u16) -> u16 {
        todo!("Wow64SetThreadDefaultGuestMachine")
    }
    fn Wow64SuspendThread(&self, h_thread: super::super::Foundation::HANDLE) -> u32 {
        todo!("Wow64SuspendThread")
    }
}
pub fn get_api(ctx: &crate::core::Win32Context) -> std::sync::Arc<dyn Api> {
    ctx.get::<dyn Api>()
}
