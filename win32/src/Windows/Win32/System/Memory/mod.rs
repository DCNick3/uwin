#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    clippy::all
)]
#[allow(unused)]
use win32::core::prelude::*;
pub struct CFG_CALL_TARGET_INFO {
    pub Offset: PtrRepr,
    pub Flags: PtrRepr,
}
impl ::core::marker::Copy for CFG_CALL_TARGET_INFO {}
impl ::core::clone::Clone for CFG_CALL_TARGET_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CFG_CALL_TARGET_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CFG_CALL_TARGET_INFO")
            .field("Offset", &self.Offset)
            .field("Flags", &self.Flags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CFG_CALL_TARGET_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for CFG_CALL_TARGET_INFO {}
impl FromIntoMemory for CFG_CALL_TARGET_INFO {
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
pub const FILE_CACHE_MAX_HARD_DISABLE: u32 = 2u32;
pub const FILE_CACHE_MAX_HARD_ENABLE: u32 = 1u32;
pub const FILE_CACHE_MIN_HARD_DISABLE: u32 = 8u32;
pub const FILE_CACHE_MIN_HARD_ENABLE: u32 = 4u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FILE_MAP(pub u32);
pub const FILE_MAP_WRITE: FILE_MAP = FILE_MAP(2u32);
pub const FILE_MAP_READ: FILE_MAP = FILE_MAP(4u32);
pub const FILE_MAP_ALL_ACCESS: FILE_MAP = FILE_MAP(983071u32);
pub const FILE_MAP_EXECUTE: FILE_MAP = FILE_MAP(32u32);
pub const FILE_MAP_COPY: FILE_MAP = FILE_MAP(1u32);
pub const FILE_MAP_RESERVE: FILE_MAP = FILE_MAP(2147483648u32);
pub const FILE_MAP_TARGETS_INVALID: FILE_MAP = FILE_MAP(1073741824u32);
pub const FILE_MAP_LARGE_PAGES: FILE_MAP = FILE_MAP(536870912u32);
impl ::core::marker::Copy for FILE_MAP {}
impl ::core::clone::Clone for FILE_MAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_MAP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_MAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_MAP").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FILE_MAP {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FILE_MAP {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FILE_MAP {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FILE_MAP {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FILE_MAP {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for FILE_MAP {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GLOBAL_ALLOC_FLAGS(pub u32);
pub const GHND: GLOBAL_ALLOC_FLAGS = GLOBAL_ALLOC_FLAGS(66u32);
pub const GMEM_FIXED: GLOBAL_ALLOC_FLAGS = GLOBAL_ALLOC_FLAGS(0u32);
pub const GMEM_MOVEABLE: GLOBAL_ALLOC_FLAGS = GLOBAL_ALLOC_FLAGS(2u32);
pub const GMEM_ZEROINIT: GLOBAL_ALLOC_FLAGS = GLOBAL_ALLOC_FLAGS(64u32);
pub const GPTR: GLOBAL_ALLOC_FLAGS = GLOBAL_ALLOC_FLAGS(64u32);
impl ::core::marker::Copy for GLOBAL_ALLOC_FLAGS {}
impl ::core::clone::Clone for GLOBAL_ALLOC_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GLOBAL_ALLOC_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GLOBAL_ALLOC_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLOBAL_ALLOC_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GLOBAL_ALLOC_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GLOBAL_ALLOC_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GLOBAL_ALLOC_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GLOBAL_ALLOC_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GLOBAL_ALLOC_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for GLOBAL_ALLOC_FLAGS {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HEAP_FLAGS(pub u32);
pub const HEAP_NONE: HEAP_FLAGS = HEAP_FLAGS(0u32);
pub const HEAP_NO_SERIALIZE: HEAP_FLAGS = HEAP_FLAGS(1u32);
pub const HEAP_GROWABLE: HEAP_FLAGS = HEAP_FLAGS(2u32);
pub const HEAP_GENERATE_EXCEPTIONS: HEAP_FLAGS = HEAP_FLAGS(4u32);
pub const HEAP_ZERO_MEMORY: HEAP_FLAGS = HEAP_FLAGS(8u32);
pub const HEAP_REALLOC_IN_PLACE_ONLY: HEAP_FLAGS = HEAP_FLAGS(16u32);
pub const HEAP_TAIL_CHECKING_ENABLED: HEAP_FLAGS = HEAP_FLAGS(32u32);
pub const HEAP_FREE_CHECKING_ENABLED: HEAP_FLAGS = HEAP_FLAGS(64u32);
pub const HEAP_DISABLE_COALESCE_ON_FREE: HEAP_FLAGS = HEAP_FLAGS(128u32);
pub const HEAP_CREATE_ALIGN_16: HEAP_FLAGS = HEAP_FLAGS(65536u32);
pub const HEAP_CREATE_ENABLE_TRACING: HEAP_FLAGS = HEAP_FLAGS(131072u32);
pub const HEAP_CREATE_ENABLE_EXECUTE: HEAP_FLAGS = HEAP_FLAGS(262144u32);
pub const HEAP_MAXIMUM_TAG: HEAP_FLAGS = HEAP_FLAGS(4095u32);
pub const HEAP_PSEUDO_TAG_FLAG: HEAP_FLAGS = HEAP_FLAGS(32768u32);
pub const HEAP_TAG_SHIFT: HEAP_FLAGS = HEAP_FLAGS(18u32);
pub const HEAP_CREATE_SEGMENT_HEAP: HEAP_FLAGS = HEAP_FLAGS(256u32);
pub const HEAP_CREATE_HARDENED: HEAP_FLAGS = HEAP_FLAGS(512u32);
impl ::core::marker::Copy for HEAP_FLAGS {}
impl ::core::clone::Clone for HEAP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HEAP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HEAP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HEAP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for HEAP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for HEAP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for HEAP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for HEAP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for HEAP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for HEAP_FLAGS {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HEAP_INFORMATION_CLASS(pub i32);
pub const HeapCompatibilityInformation: HEAP_INFORMATION_CLASS = HEAP_INFORMATION_CLASS(0i32);
pub const HeapEnableTerminationOnCorruption: HEAP_INFORMATION_CLASS = HEAP_INFORMATION_CLASS(1i32);
pub const HeapOptimizeResources: HEAP_INFORMATION_CLASS = HEAP_INFORMATION_CLASS(3i32);
pub const HeapTag: HEAP_INFORMATION_CLASS = HEAP_INFORMATION_CLASS(7i32);
impl ::core::marker::Copy for HEAP_INFORMATION_CLASS {}
impl ::core::clone::Clone for HEAP_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HEAP_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HEAP_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HEAP_INFORMATION_CLASS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for HEAP_INFORMATION_CLASS {
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
pub struct HEAP_SUMMARY {
    pub cb: u32,
    pub cbAllocated: PtrRepr,
    pub cbCommitted: PtrRepr,
    pub cbReserved: PtrRepr,
    pub cbMaxReserve: PtrRepr,
}
impl ::core::marker::Copy for HEAP_SUMMARY {}
impl ::core::clone::Clone for HEAP_SUMMARY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HEAP_SUMMARY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HEAP_SUMMARY")
            .field("cb", &self.cb)
            .field("cbAllocated", &self.cbAllocated)
            .field("cbCommitted", &self.cbCommitted)
            .field("cbReserved", &self.cbReserved)
            .field("cbMaxReserve", &self.cbMaxReserve)
            .finish()
    }
}
impl ::core::cmp::PartialEq for HEAP_SUMMARY {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb
            && self.cbAllocated == other.cbAllocated
            && self.cbCommitted == other.cbCommitted
            && self.cbReserved == other.cbReserved
            && self.cbMaxReserve == other.cbMaxReserve
    }
}
impl ::core::cmp::Eq for HEAP_SUMMARY {}
impl FromIntoMemory for HEAP_SUMMARY {
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
pub struct HeapHandle(pub PtrDiffRepr);
impl HeapHandle {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HeapHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HeapHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HeapHandle {}
impl ::core::fmt::Debug for HeapHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HeapHandle").field(&self.0).finish()
    }
}
impl FromIntoMemory for HeapHandle {
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
pub struct LOCAL_ALLOC_FLAGS(pub u32);
pub const LHND: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(66u32);
pub const LMEM_FIXED: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(0u32);
pub const LMEM_MOVEABLE: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(2u32);
pub const LMEM_ZEROINIT: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(64u32);
pub const LPTR: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(64u32);
pub const NONZEROLHND: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(2u32);
pub const NONZEROLPTR: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(0u32);
impl ::core::marker::Copy for LOCAL_ALLOC_FLAGS {}
impl ::core::clone::Clone for LOCAL_ALLOC_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LOCAL_ALLOC_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LOCAL_ALLOC_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOCAL_ALLOC_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LOCAL_ALLOC_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LOCAL_ALLOC_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LOCAL_ALLOC_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LOCAL_ALLOC_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LOCAL_ALLOC_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for LOCAL_ALLOC_FLAGS {
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
pub const MEHC_PATROL_SCRUBBER_PRESENT: u32 = 1u32;
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct MEMORY_BASIC_INFORMATION {
    pub BaseAddress: MutPtr<::core::ffi::c_void>,
    pub AllocationBase: MutPtr<::core::ffi::c_void>,
    pub AllocationProtect: PAGE_PROTECTION_FLAGS,
    pub PartitionId: u16,
    pub RegionSize: PtrRepr,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: PAGE_PROTECTION_FLAGS,
    pub Type: PAGE_TYPE,
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for MEMORY_BASIC_INFORMATION {}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for MEMORY_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for MEMORY_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEMORY_BASIC_INFORMATION")
            .field("BaseAddress", &self.BaseAddress)
            .field("AllocationBase", &self.AllocationBase)
            .field("AllocationProtect", &self.AllocationProtect)
            .field("PartitionId", &self.PartitionId)
            .field("RegionSize", &self.RegionSize)
            .field("State", &self.State)
            .field("Protect", &self.Protect)
            .field("Type", &self.Type)
            .finish()
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for MEMORY_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.BaseAddress == other.BaseAddress
            && self.AllocationBase == other.AllocationBase
            && self.AllocationProtect == other.AllocationProtect
            && self.PartitionId == other.PartitionId
            && self.RegionSize == other.RegionSize
            && self.State == other.State
            && self.Protect == other.Protect
            && self.Type == other.Type
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for MEMORY_BASIC_INFORMATION {}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for MEMORY_BASIC_INFORMATION {
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
pub struct MEMORY_BASIC_INFORMATION {
    pub BaseAddress: MutPtr<::core::ffi::c_void>,
    pub AllocationBase: MutPtr<::core::ffi::c_void>,
    pub AllocationProtect: PAGE_PROTECTION_FLAGS,
    pub RegionSize: PtrRepr,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: PAGE_PROTECTION_FLAGS,
    pub Type: PAGE_TYPE,
}
impl ::core::marker::Copy for MEMORY_BASIC_INFORMATION {}
impl ::core::clone::Clone for MEMORY_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MEMORY_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEMORY_BASIC_INFORMATION")
            .field("BaseAddress", &self.BaseAddress)
            .field("AllocationBase", &self.AllocationBase)
            .field("AllocationProtect", &self.AllocationProtect)
            .field("RegionSize", &self.RegionSize)
            .field("State", &self.State)
            .field("Protect", &self.Protect)
            .field("Type", &self.Type)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MEMORY_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.BaseAddress == other.BaseAddress
            && self.AllocationBase == other.AllocationBase
            && self.AllocationProtect == other.AllocationProtect
            && self.RegionSize == other.RegionSize
            && self.State == other.State
            && self.Protect == other.Protect
            && self.Type == other.Type
    }
}
impl ::core::cmp::Eq for MEMORY_BASIC_INFORMATION {}
impl FromIntoMemory for MEMORY_BASIC_INFORMATION {
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
pub struct MEMORY_BASIC_INFORMATION32 {
    pub BaseAddress: u32,
    pub AllocationBase: u32,
    pub AllocationProtect: PAGE_PROTECTION_FLAGS,
    pub RegionSize: u32,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: PAGE_PROTECTION_FLAGS,
    pub Type: PAGE_TYPE,
}
impl ::core::marker::Copy for MEMORY_BASIC_INFORMATION32 {}
impl ::core::clone::Clone for MEMORY_BASIC_INFORMATION32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MEMORY_BASIC_INFORMATION32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEMORY_BASIC_INFORMATION32")
            .field("BaseAddress", &self.BaseAddress)
            .field("AllocationBase", &self.AllocationBase)
            .field("AllocationProtect", &self.AllocationProtect)
            .field("RegionSize", &self.RegionSize)
            .field("State", &self.State)
            .field("Protect", &self.Protect)
            .field("Type", &self.Type)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MEMORY_BASIC_INFORMATION32 {
    fn eq(&self, other: &Self) -> bool {
        self.BaseAddress == other.BaseAddress
            && self.AllocationBase == other.AllocationBase
            && self.AllocationProtect == other.AllocationProtect
            && self.RegionSize == other.RegionSize
            && self.State == other.State
            && self.Protect == other.Protect
            && self.Type == other.Type
    }
}
impl ::core::cmp::Eq for MEMORY_BASIC_INFORMATION32 {}
impl FromIntoMemory for MEMORY_BASIC_INFORMATION32 {
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
pub struct MEMORY_BASIC_INFORMATION64 {
    pub BaseAddress: u64,
    pub AllocationBase: u64,
    pub AllocationProtect: PAGE_PROTECTION_FLAGS,
    pub __alignment1: u32,
    pub RegionSize: u64,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: PAGE_PROTECTION_FLAGS,
    pub Type: PAGE_TYPE,
    pub __alignment2: u32,
}
impl ::core::marker::Copy for MEMORY_BASIC_INFORMATION64 {}
impl ::core::clone::Clone for MEMORY_BASIC_INFORMATION64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MEMORY_BASIC_INFORMATION64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEMORY_BASIC_INFORMATION64")
            .field("BaseAddress", &self.BaseAddress)
            .field("AllocationBase", &self.AllocationBase)
            .field("AllocationProtect", &self.AllocationProtect)
            .field("__alignment1", &self.__alignment1)
            .field("RegionSize", &self.RegionSize)
            .field("State", &self.State)
            .field("Protect", &self.Protect)
            .field("Type", &self.Type)
            .field("__alignment2", &self.__alignment2)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MEMORY_BASIC_INFORMATION64 {
    fn eq(&self, other: &Self) -> bool {
        self.BaseAddress == other.BaseAddress
            && self.AllocationBase == other.AllocationBase
            && self.AllocationProtect == other.AllocationProtect
            && self.__alignment1 == other.__alignment1
            && self.RegionSize == other.RegionSize
            && self.State == other.State
            && self.Protect == other.Protect
            && self.Type == other.Type
            && self.__alignment2 == other.__alignment2
    }
}
impl ::core::cmp::Eq for MEMORY_BASIC_INFORMATION64 {}
impl FromIntoMemory for MEMORY_BASIC_INFORMATION64 {
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
pub struct MEMORY_RESOURCE_NOTIFICATION_TYPE(pub i32);
pub const LowMemoryResourceNotification: MEMORY_RESOURCE_NOTIFICATION_TYPE =
    MEMORY_RESOURCE_NOTIFICATION_TYPE(0i32);
pub const HighMemoryResourceNotification: MEMORY_RESOURCE_NOTIFICATION_TYPE =
    MEMORY_RESOURCE_NOTIFICATION_TYPE(1i32);
impl ::core::marker::Copy for MEMORY_RESOURCE_NOTIFICATION_TYPE {}
impl ::core::clone::Clone for MEMORY_RESOURCE_NOTIFICATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MEMORY_RESOURCE_NOTIFICATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MEMORY_RESOURCE_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEMORY_RESOURCE_NOTIFICATION_TYPE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for MEMORY_RESOURCE_NOTIFICATION_TYPE {
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
pub struct MEM_ADDRESS_REQUIREMENTS {
    pub LowestStartingAddress: MutPtr<::core::ffi::c_void>,
    pub HighestEndingAddress: MutPtr<::core::ffi::c_void>,
    pub Alignment: PtrRepr,
}
impl ::core::marker::Copy for MEM_ADDRESS_REQUIREMENTS {}
impl ::core::clone::Clone for MEM_ADDRESS_REQUIREMENTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MEM_ADDRESS_REQUIREMENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEM_ADDRESS_REQUIREMENTS")
            .field("LowestStartingAddress", &self.LowestStartingAddress)
            .field("HighestEndingAddress", &self.HighestEndingAddress)
            .field("Alignment", &self.Alignment)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MEM_ADDRESS_REQUIREMENTS {
    fn eq(&self, other: &Self) -> bool {
        self.LowestStartingAddress == other.LowestStartingAddress
            && self.HighestEndingAddress == other.HighestEndingAddress
            && self.Alignment == other.Alignment
    }
}
impl ::core::cmp::Eq for MEM_ADDRESS_REQUIREMENTS {}
impl FromIntoMemory for MEM_ADDRESS_REQUIREMENTS {
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
pub struct MEM_EXTENDED_PARAMETER {
    pub Anonymous1: MEM_EXTENDED_PARAMETER_0,
    pub Anonymous2: MEM_EXTENDED_PARAMETER_1,
}
impl ::core::marker::Copy for MEM_EXTENDED_PARAMETER {}
impl ::core::clone::Clone for MEM_EXTENDED_PARAMETER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for MEM_EXTENDED_PARAMETER {
    fn eq(&self, other: &Self) -> bool {
        self.Anonymous1 == other.Anonymous1 && self.Anonymous2 == other.Anonymous2
    }
}
impl ::core::cmp::Eq for MEM_EXTENDED_PARAMETER {}
impl FromIntoMemory for MEM_EXTENDED_PARAMETER {
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
pub struct MEM_EXTENDED_PARAMETER_0 {
    pub _bitfield: u64,
}
impl ::core::marker::Copy for MEM_EXTENDED_PARAMETER_0 {}
impl ::core::clone::Clone for MEM_EXTENDED_PARAMETER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MEM_EXTENDED_PARAMETER_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEM_EXTENDED_PARAMETER_0")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MEM_EXTENDED_PARAMETER_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for MEM_EXTENDED_PARAMETER_0 {}
impl FromIntoMemory for MEM_EXTENDED_PARAMETER_0 {
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
pub struct MEM_EXTENDED_PARAMETER_1 {
    pub ULong64: u64,
    pub Pointer: MutPtr<::core::ffi::c_void>,
    pub Size: PtrRepr,
    pub Handle: super::super::Foundation::HANDLE,
    pub ULong: u32,
}
impl ::core::marker::Copy for MEM_EXTENDED_PARAMETER_1 {}
impl ::core::clone::Clone for MEM_EXTENDED_PARAMETER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for MEM_EXTENDED_PARAMETER_1 {
    fn eq(&self, other: &Self) -> bool {
        self.ULong64 == other.ULong64
            && self.Pointer == other.Pointer
            && self.Size == other.Size
            && self.Handle == other.Handle
            && self.ULong == other.ULong
    }
}
impl ::core::cmp::Eq for MEM_EXTENDED_PARAMETER_1 {}
impl FromIntoMemory for MEM_EXTENDED_PARAMETER_1 {
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
pub struct MEM_EXTENDED_PARAMETER_TYPE(pub i32);
pub const MemExtendedParameterInvalidType: MEM_EXTENDED_PARAMETER_TYPE =
    MEM_EXTENDED_PARAMETER_TYPE(0i32);
pub const MemExtendedParameterAddressRequirements: MEM_EXTENDED_PARAMETER_TYPE =
    MEM_EXTENDED_PARAMETER_TYPE(1i32);
pub const MemExtendedParameterNumaNode: MEM_EXTENDED_PARAMETER_TYPE =
    MEM_EXTENDED_PARAMETER_TYPE(2i32);
pub const MemExtendedParameterPartitionHandle: MEM_EXTENDED_PARAMETER_TYPE =
    MEM_EXTENDED_PARAMETER_TYPE(3i32);
pub const MemExtendedParameterUserPhysicalHandle: MEM_EXTENDED_PARAMETER_TYPE =
    MEM_EXTENDED_PARAMETER_TYPE(4i32);
pub const MemExtendedParameterAttributeFlags: MEM_EXTENDED_PARAMETER_TYPE =
    MEM_EXTENDED_PARAMETER_TYPE(5i32);
pub const MemExtendedParameterImageMachine: MEM_EXTENDED_PARAMETER_TYPE =
    MEM_EXTENDED_PARAMETER_TYPE(6i32);
pub const MemExtendedParameterMax: MEM_EXTENDED_PARAMETER_TYPE = MEM_EXTENDED_PARAMETER_TYPE(7i32);
impl ::core::marker::Copy for MEM_EXTENDED_PARAMETER_TYPE {}
impl ::core::clone::Clone for MEM_EXTENDED_PARAMETER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MEM_EXTENDED_PARAMETER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MEM_EXTENDED_PARAMETER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEM_EXTENDED_PARAMETER_TYPE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for MEM_EXTENDED_PARAMETER_TYPE {
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
pub struct OFFER_PRIORITY(pub i32);
pub const VmOfferPriorityVeryLow: OFFER_PRIORITY = OFFER_PRIORITY(1i32);
pub const VmOfferPriorityLow: OFFER_PRIORITY = OFFER_PRIORITY(2i32);
pub const VmOfferPriorityBelowNormal: OFFER_PRIORITY = OFFER_PRIORITY(3i32);
pub const VmOfferPriorityNormal: OFFER_PRIORITY = OFFER_PRIORITY(4i32);
impl ::core::marker::Copy for OFFER_PRIORITY {}
impl ::core::clone::Clone for OFFER_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OFFER_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OFFER_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFER_PRIORITY").field(&self.0).finish()
    }
}
impl FromIntoMemory for OFFER_PRIORITY {
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
pub struct PAGE_PROTECTION_FLAGS(pub u32);
pub const PAGE_NOACCESS: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(1u32);
pub const PAGE_READONLY: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(2u32);
pub const PAGE_READWRITE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(4u32);
pub const PAGE_WRITECOPY: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(8u32);
pub const PAGE_EXECUTE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(16u32);
pub const PAGE_EXECUTE_READ: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(32u32);
pub const PAGE_EXECUTE_READWRITE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(64u32);
pub const PAGE_EXECUTE_WRITECOPY: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(128u32);
pub const PAGE_GUARD: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(256u32);
pub const PAGE_NOCACHE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(512u32);
pub const PAGE_WRITECOMBINE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(1024u32);
pub const PAGE_GRAPHICS_NOACCESS: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(2048u32);
pub const PAGE_GRAPHICS_READONLY: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(4096u32);
pub const PAGE_GRAPHICS_READWRITE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(8192u32);
pub const PAGE_GRAPHICS_EXECUTE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(16384u32);
pub const PAGE_GRAPHICS_EXECUTE_READ: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(32768u32);
pub const PAGE_GRAPHICS_EXECUTE_READWRITE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(65536u32);
pub const PAGE_GRAPHICS_COHERENT: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(131072u32);
pub const PAGE_GRAPHICS_NOCACHE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(262144u32);
pub const PAGE_ENCLAVE_THREAD_CONTROL: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(2147483648u32);
pub const PAGE_REVERT_TO_FILE_MAP: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(2147483648u32);
pub const PAGE_TARGETS_NO_UPDATE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(1073741824u32);
pub const PAGE_TARGETS_INVALID: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(1073741824u32);
pub const PAGE_ENCLAVE_UNVALIDATED: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(536870912u32);
pub const PAGE_ENCLAVE_MASK: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(268435456u32);
pub const PAGE_ENCLAVE_DECOMMIT: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(268435456u32);
pub const PAGE_ENCLAVE_SS_FIRST: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(268435457u32);
pub const PAGE_ENCLAVE_SS_REST: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(268435458u32);
pub const SEC_PARTITION_OWNER_HANDLE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(262144u32);
pub const SEC_64K_PAGES: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(524288u32);
pub const SEC_FILE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(8388608u32);
pub const SEC_IMAGE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(16777216u32);
pub const SEC_PROTECTED_IMAGE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(33554432u32);
pub const SEC_RESERVE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(67108864u32);
pub const SEC_COMMIT: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(134217728u32);
pub const SEC_NOCACHE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(268435456u32);
pub const SEC_WRITECOMBINE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(1073741824u32);
pub const SEC_LARGE_PAGES: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(2147483648u32);
pub const SEC_IMAGE_NO_EXECUTE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(285212672u32);
impl ::core::marker::Copy for PAGE_PROTECTION_FLAGS {}
impl ::core::clone::Clone for PAGE_PROTECTION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PAGE_PROTECTION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PAGE_PROTECTION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PAGE_PROTECTION_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for PAGE_PROTECTION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PAGE_PROTECTION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PAGE_PROTECTION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PAGE_PROTECTION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PAGE_PROTECTION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for PAGE_PROTECTION_FLAGS {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PAGE_TYPE(pub u32);
pub const MEM_PRIVATE: PAGE_TYPE = PAGE_TYPE(131072u32);
pub const MEM_MAPPED: PAGE_TYPE = PAGE_TYPE(262144u32);
pub const MEM_IMAGE: PAGE_TYPE = PAGE_TYPE(16777216u32);
impl ::core::marker::Copy for PAGE_TYPE {}
impl ::core::clone::Clone for PAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PAGE_TYPE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PAGE_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PAGE_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PAGE_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PAGE_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PAGE_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for PAGE_TYPE {
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
pub type PBAD_MEMORY_CALLBACK_ROUTINE = ::core::option::Option<unsafe extern "system" fn()>;
pub struct PROCESS_HEAP_ENTRY {
    pub lpData: MutPtr<::core::ffi::c_void>,
    pub cbData: u32,
    pub cbOverhead: u8,
    pub iRegionIndex: u8,
    pub wFlags: u16,
    pub Anonymous: PROCESS_HEAP_ENTRY_0,
}
impl ::core::marker::Copy for PROCESS_HEAP_ENTRY {}
impl ::core::clone::Clone for PROCESS_HEAP_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for PROCESS_HEAP_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.lpData == other.lpData
            && self.cbData == other.cbData
            && self.cbOverhead == other.cbOverhead
            && self.iRegionIndex == other.iRegionIndex
            && self.wFlags == other.wFlags
            && self.Anonymous == other.Anonymous
    }
}
impl ::core::cmp::Eq for PROCESS_HEAP_ENTRY {}
impl FromIntoMemory for PROCESS_HEAP_ENTRY {
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
pub struct PROCESS_HEAP_ENTRY_0 {
    pub Block: PROCESS_HEAP_ENTRY_0_0,
    pub Region: PROCESS_HEAP_ENTRY_0_1,
}
impl ::core::marker::Copy for PROCESS_HEAP_ENTRY_0 {}
impl ::core::clone::Clone for PROCESS_HEAP_ENTRY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for PROCESS_HEAP_ENTRY_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Block == other.Block && self.Region == other.Region
    }
}
impl ::core::cmp::Eq for PROCESS_HEAP_ENTRY_0 {}
impl FromIntoMemory for PROCESS_HEAP_ENTRY_0 {
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
pub struct PROCESS_HEAP_ENTRY_0_0 {
    pub hMem: super::super::Foundation::HANDLE,
    pub dwReserved: [u32; 3],
}
impl ::core::marker::Copy for PROCESS_HEAP_ENTRY_0_0 {}
impl ::core::clone::Clone for PROCESS_HEAP_ENTRY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_HEAP_ENTRY_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_HEAP_ENTRY_0_0")
            .field("hMem", &self.hMem)
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PROCESS_HEAP_ENTRY_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.hMem == other.hMem && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for PROCESS_HEAP_ENTRY_0_0 {}
impl FromIntoMemory for PROCESS_HEAP_ENTRY_0_0 {
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
pub struct PROCESS_HEAP_ENTRY_0_1 {
    pub dwCommittedSize: u32,
    pub dwUnCommittedSize: u32,
    pub lpFirstBlock: MutPtr<::core::ffi::c_void>,
    pub lpLastBlock: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for PROCESS_HEAP_ENTRY_0_1 {}
impl ::core::clone::Clone for PROCESS_HEAP_ENTRY_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_HEAP_ENTRY_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_HEAP_ENTRY_0_1")
            .field("dwCommittedSize", &self.dwCommittedSize)
            .field("dwUnCommittedSize", &self.dwUnCommittedSize)
            .field("lpFirstBlock", &self.lpFirstBlock)
            .field("lpLastBlock", &self.lpLastBlock)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PROCESS_HEAP_ENTRY_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwCommittedSize == other.dwCommittedSize
            && self.dwUnCommittedSize == other.dwUnCommittedSize
            && self.lpFirstBlock == other.lpFirstBlock
            && self.lpLastBlock == other.lpLastBlock
    }
}
impl ::core::cmp::Eq for PROCESS_HEAP_ENTRY_0_1 {}
impl FromIntoMemory for PROCESS_HEAP_ENTRY_0_1 {
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
pub type PSECURE_MEMORY_CACHE_CALLBACK = ::core::option::Option<
    unsafe extern "system" fn(
        addr: ConstPtr<::core::ffi::c_void>,
        range: PtrRepr,
    ) -> super::super::Foundation::BOOLEAN,
>;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UNMAP_VIEW_OF_FILE_FLAGS(pub u32);
pub const MEM_UNMAP_NONE: UNMAP_VIEW_OF_FILE_FLAGS = UNMAP_VIEW_OF_FILE_FLAGS(0u32);
pub const MEM_UNMAP_WITH_TRANSIENT_BOOST: UNMAP_VIEW_OF_FILE_FLAGS = UNMAP_VIEW_OF_FILE_FLAGS(1u32);
pub const MEM_PRESERVE_PLACEHOLDER: UNMAP_VIEW_OF_FILE_FLAGS = UNMAP_VIEW_OF_FILE_FLAGS(2u32);
impl ::core::marker::Copy for UNMAP_VIEW_OF_FILE_FLAGS {}
impl ::core::clone::Clone for UNMAP_VIEW_OF_FILE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UNMAP_VIEW_OF_FILE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNMAP_VIEW_OF_FILE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNMAP_VIEW_OF_FILE_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UNMAP_VIEW_OF_FILE_FLAGS {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VIRTUAL_ALLOCATION_TYPE(pub u32);
pub const MEM_COMMIT: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(4096u32);
pub const MEM_RESERVE: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(8192u32);
pub const MEM_RESET: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(524288u32);
pub const MEM_RESET_UNDO: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(16777216u32);
pub const MEM_REPLACE_PLACEHOLDER: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(16384u32);
pub const MEM_LARGE_PAGES: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(536870912u32);
pub const MEM_RESERVE_PLACEHOLDER: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(262144u32);
pub const MEM_FREE: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(65536u32);
impl ::core::marker::Copy for VIRTUAL_ALLOCATION_TYPE {}
impl ::core::clone::Clone for VIRTUAL_ALLOCATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VIRTUAL_ALLOCATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VIRTUAL_ALLOCATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VIRTUAL_ALLOCATION_TYPE")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for VIRTUAL_ALLOCATION_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for VIRTUAL_ALLOCATION_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for VIRTUAL_ALLOCATION_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for VIRTUAL_ALLOCATION_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for VIRTUAL_ALLOCATION_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for VIRTUAL_ALLOCATION_TYPE {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VIRTUAL_FREE_TYPE(pub u32);
pub const MEM_DECOMMIT: VIRTUAL_FREE_TYPE = VIRTUAL_FREE_TYPE(16384u32);
pub const MEM_RELEASE: VIRTUAL_FREE_TYPE = VIRTUAL_FREE_TYPE(32768u32);
impl ::core::marker::Copy for VIRTUAL_FREE_TYPE {}
impl ::core::clone::Clone for VIRTUAL_FREE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VIRTUAL_FREE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VIRTUAL_FREE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VIRTUAL_FREE_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for VIRTUAL_FREE_TYPE {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WIN32_MEMORY_INFORMATION_CLASS(pub i32);
pub const MemoryRegionInfo: WIN32_MEMORY_INFORMATION_CLASS = WIN32_MEMORY_INFORMATION_CLASS(0i32);
impl ::core::marker::Copy for WIN32_MEMORY_INFORMATION_CLASS {}
impl ::core::clone::Clone for WIN32_MEMORY_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WIN32_MEMORY_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WIN32_MEMORY_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WIN32_MEMORY_INFORMATION_CLASS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for WIN32_MEMORY_INFORMATION_CLASS {
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
pub struct WIN32_MEMORY_PARTITION_INFORMATION {
    pub Flags: u32,
    pub NumaNode: u32,
    pub Channel: u32,
    pub NumberOfNumaNodes: u32,
    pub ResidentAvailablePages: u64,
    pub CommittedPages: u64,
    pub CommitLimit: u64,
    pub PeakCommitment: u64,
    pub TotalNumberOfPages: u64,
    pub AvailablePages: u64,
    pub ZeroPages: u64,
    pub FreePages: u64,
    pub StandbyPages: u64,
    pub Reserved: [u64; 16],
    pub MaximumCommitLimit: u64,
    pub Reserved2: u64,
    pub PartitionId: u32,
}
impl ::core::marker::Copy for WIN32_MEMORY_PARTITION_INFORMATION {}
impl ::core::clone::Clone for WIN32_MEMORY_PARTITION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIN32_MEMORY_PARTITION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN32_MEMORY_PARTITION_INFORMATION")
            .field("Flags", &self.Flags)
            .field("NumaNode", &self.NumaNode)
            .field("Channel", &self.Channel)
            .field("NumberOfNumaNodes", &self.NumberOfNumaNodes)
            .field("ResidentAvailablePages", &self.ResidentAvailablePages)
            .field("CommittedPages", &self.CommittedPages)
            .field("CommitLimit", &self.CommitLimit)
            .field("PeakCommitment", &self.PeakCommitment)
            .field("TotalNumberOfPages", &self.TotalNumberOfPages)
            .field("AvailablePages", &self.AvailablePages)
            .field("ZeroPages", &self.ZeroPages)
            .field("FreePages", &self.FreePages)
            .field("StandbyPages", &self.StandbyPages)
            .field("Reserved", &self.Reserved)
            .field("MaximumCommitLimit", &self.MaximumCommitLimit)
            .field("Reserved2", &self.Reserved2)
            .field("PartitionId", &self.PartitionId)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WIN32_MEMORY_PARTITION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
            && self.NumaNode == other.NumaNode
            && self.Channel == other.Channel
            && self.NumberOfNumaNodes == other.NumberOfNumaNodes
            && self.ResidentAvailablePages == other.ResidentAvailablePages
            && self.CommittedPages == other.CommittedPages
            && self.CommitLimit == other.CommitLimit
            && self.PeakCommitment == other.PeakCommitment
            && self.TotalNumberOfPages == other.TotalNumberOfPages
            && self.AvailablePages == other.AvailablePages
            && self.ZeroPages == other.ZeroPages
            && self.FreePages == other.FreePages
            && self.StandbyPages == other.StandbyPages
            && self.Reserved == other.Reserved
            && self.MaximumCommitLimit == other.MaximumCommitLimit
            && self.Reserved2 == other.Reserved2
            && self.PartitionId == other.PartitionId
    }
}
impl ::core::cmp::Eq for WIN32_MEMORY_PARTITION_INFORMATION {}
impl FromIntoMemory for WIN32_MEMORY_PARTITION_INFORMATION {
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
pub struct WIN32_MEMORY_PARTITION_INFORMATION_CLASS(pub i32);
pub const MemoryPartitionInfo: WIN32_MEMORY_PARTITION_INFORMATION_CLASS =
    WIN32_MEMORY_PARTITION_INFORMATION_CLASS(0i32);
pub const MemoryPartitionDedicatedMemoryInfo: WIN32_MEMORY_PARTITION_INFORMATION_CLASS =
    WIN32_MEMORY_PARTITION_INFORMATION_CLASS(1i32);
impl ::core::marker::Copy for WIN32_MEMORY_PARTITION_INFORMATION_CLASS {}
impl ::core::clone::Clone for WIN32_MEMORY_PARTITION_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WIN32_MEMORY_PARTITION_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WIN32_MEMORY_PARTITION_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WIN32_MEMORY_PARTITION_INFORMATION_CLASS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for WIN32_MEMORY_PARTITION_INFORMATION_CLASS {
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
pub struct WIN32_MEMORY_RANGE_ENTRY {
    pub VirtualAddress: MutPtr<::core::ffi::c_void>,
    pub NumberOfBytes: PtrRepr,
}
impl ::core::marker::Copy for WIN32_MEMORY_RANGE_ENTRY {}
impl ::core::clone::Clone for WIN32_MEMORY_RANGE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIN32_MEMORY_RANGE_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN32_MEMORY_RANGE_ENTRY")
            .field("VirtualAddress", &self.VirtualAddress)
            .field("NumberOfBytes", &self.NumberOfBytes)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WIN32_MEMORY_RANGE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualAddress == other.VirtualAddress && self.NumberOfBytes == other.NumberOfBytes
    }
}
impl ::core::cmp::Eq for WIN32_MEMORY_RANGE_ENTRY {}
impl FromIntoMemory for WIN32_MEMORY_RANGE_ENTRY {
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
pub struct WIN32_MEMORY_REGION_INFORMATION {
    pub AllocationBase: MutPtr<::core::ffi::c_void>,
    pub AllocationProtect: u32,
    pub Anonymous: WIN32_MEMORY_REGION_INFORMATION_0,
    pub RegionSize: PtrRepr,
    pub CommitSize: PtrRepr,
}
impl ::core::marker::Copy for WIN32_MEMORY_REGION_INFORMATION {}
impl ::core::clone::Clone for WIN32_MEMORY_REGION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for WIN32_MEMORY_REGION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.AllocationBase == other.AllocationBase
            && self.AllocationProtect == other.AllocationProtect
            && self.Anonymous == other.Anonymous
            && self.RegionSize == other.RegionSize
            && self.CommitSize == other.CommitSize
    }
}
impl ::core::cmp::Eq for WIN32_MEMORY_REGION_INFORMATION {}
impl FromIntoMemory for WIN32_MEMORY_REGION_INFORMATION {
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
pub struct WIN32_MEMORY_REGION_INFORMATION_0 {
    pub Flags: u32,
    pub Anonymous: WIN32_MEMORY_REGION_INFORMATION_0_0,
}
impl ::core::marker::Copy for WIN32_MEMORY_REGION_INFORMATION_0 {}
impl ::core::clone::Clone for WIN32_MEMORY_REGION_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for WIN32_MEMORY_REGION_INFORMATION_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Anonymous == other.Anonymous
    }
}
impl ::core::cmp::Eq for WIN32_MEMORY_REGION_INFORMATION_0 {}
impl FromIntoMemory for WIN32_MEMORY_REGION_INFORMATION_0 {
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
pub struct WIN32_MEMORY_REGION_INFORMATION_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for WIN32_MEMORY_REGION_INFORMATION_0_0 {}
impl ::core::clone::Clone for WIN32_MEMORY_REGION_INFORMATION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIN32_MEMORY_REGION_INFORMATION_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN32_MEMORY_REGION_INFORMATION_0_0")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WIN32_MEMORY_REGION_INFORMATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for WIN32_MEMORY_REGION_INFORMATION_0_0 {}
impl FromIntoMemory for WIN32_MEMORY_REGION_INFORMATION_0_0 {
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
    fn AddSecureMemoryCacheCallback(
        &self,
        pfn_call_back: PSECURE_MEMORY_CACHE_CALLBACK,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn AllocateUserPhysicalPages(
        &self,
        h_process: super::super::Foundation::HANDLE,
        number_of_pages: MutPtr<PtrRepr>,
        page_array: MutPtr<PtrRepr>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn AllocateUserPhysicalPages2(
        &self,
        object_handle: super::super::Foundation::HANDLE,
        number_of_pages: MutPtr<PtrRepr>,
        page_array: MutPtr<PtrRepr>,
        extended_parameters: MutPtr<MEM_EXTENDED_PARAMETER>,
        extended_parameter_count: u32,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn AllocateUserPhysicalPagesNuma(
        &self,
        h_process: super::super::Foundation::HANDLE,
        number_of_pages: MutPtr<PtrRepr>,
        page_array: MutPtr<PtrRepr>,
        nnd_preferred: u32,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateFileMapping2(
        &self,
        file: super::super::Foundation::HANDLE,
        security_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        desired_access: u32,
        page_protection: PAGE_PROTECTION_FLAGS,
        allocation_attributes: u32,
        maximum_size: u64,
        name: crate::core::PCWSTR,
        extended_parameters: MutPtr<MEM_EXTENDED_PARAMETER>,
        parameter_count: u32,
    ) -> super::super::Foundation::HANDLE {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateFileMappingA(
        &self,
        h_file: super::super::Foundation::HANDLE,
        lp_file_mapping_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        fl_protect: PAGE_PROTECTION_FLAGS,
        dw_maximum_size_high: u32,
        dw_maximum_size_low: u32,
        lp_name: crate::core::PCSTR,
    ) -> super::super::Foundation::HANDLE {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateFileMappingFromApp(
        &self,
        h_file: super::super::Foundation::HANDLE,
        security_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        page_protection: PAGE_PROTECTION_FLAGS,
        maximum_size: u64,
        name: crate::core::PCWSTR,
    ) -> super::super::Foundation::HANDLE {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateFileMappingNumaA(
        &self,
        h_file: super::super::Foundation::HANDLE,
        lp_file_mapping_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        fl_protect: PAGE_PROTECTION_FLAGS,
        dw_maximum_size_high: u32,
        dw_maximum_size_low: u32,
        lp_name: crate::core::PCSTR,
        nnd_preferred: u32,
    ) -> super::super::Foundation::HANDLE {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateFileMappingNumaW(
        &self,
        h_file: super::super::Foundation::HANDLE,
        lp_file_mapping_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        fl_protect: PAGE_PROTECTION_FLAGS,
        dw_maximum_size_high: u32,
        dw_maximum_size_low: u32,
        lp_name: crate::core::PCWSTR,
        nnd_preferred: u32,
    ) -> super::super::Foundation::HANDLE {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateFileMappingW(
        &self,
        h_file: super::super::Foundation::HANDLE,
        lp_file_mapping_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        fl_protect: PAGE_PROTECTION_FLAGS,
        dw_maximum_size_high: u32,
        dw_maximum_size_low: u32,
        lp_name: crate::core::PCWSTR,
    ) -> super::super::Foundation::HANDLE {
        todo!()
    }
    fn CreateMemoryResourceNotification(
        &self,
        notification_type: MEMORY_RESOURCE_NOTIFICATION_TYPE,
    ) -> super::super::Foundation::HANDLE {
        todo!()
    }
    fn DiscardVirtualMemory(
        &self,
        virtual_address: MutPtr<::core::ffi::c_void>,
        size: PtrRepr,
    ) -> u32 {
        todo!()
    }
    fn FlushViewOfFile(
        &self,
        lp_base_address: ConstPtr<::core::ffi::c_void>,
        dw_number_of_bytes_to_flush: PtrRepr,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn FreeUserPhysicalPages(
        &self,
        h_process: super::super::Foundation::HANDLE,
        number_of_pages: MutPtr<PtrRepr>,
        page_array: ConstPtr<PtrRepr>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetLargePageMinimum(&self) -> PtrRepr {
        todo!()
    }
    fn GetMemoryErrorHandlingCapabilities(
        &self,
        capabilities: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetProcessHeap(&self) -> HeapHandle {
        todo!()
    }
    fn GetProcessHeaps(&self, number_of_heaps: u32, process_heaps: MutPtr<HeapHandle>) -> u32 {
        todo!()
    }
    fn GetProcessWorkingSetSizeEx(
        &self,
        h_process: super::super::Foundation::HANDLE,
        lp_minimum_working_set_size: MutPtr<PtrRepr>,
        lp_maximum_working_set_size: MutPtr<PtrRepr>,
        flags: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetSystemFileCacheSize(
        &self,
        lp_minimum_file_cache_size: MutPtr<PtrRepr>,
        lp_maximum_file_cache_size: MutPtr<PtrRepr>,
        lp_flags: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetWriteWatch(
        &self,
        dw_flags: u32,
        lp_base_address: ConstPtr<::core::ffi::c_void>,
        dw_region_size: PtrRepr,
        lp_addresses: MutPtr<ConstPtr<::core::ffi::c_void>>,
        lpdw_count: MutPtr<PtrRepr>,
        lpdw_granularity: MutPtr<u32>,
    ) -> u32 {
        todo!()
    }
    fn GlobalAlloc(&self, u_flags: GLOBAL_ALLOC_FLAGS, dw_bytes: PtrRepr) -> PtrDiffRepr {
        todo!()
    }
    fn GlobalFlags(&self, h_mem: PtrDiffRepr) -> u32 {
        todo!()
    }
    fn GlobalFree(&self, h_mem: PtrDiffRepr) -> PtrDiffRepr {
        todo!()
    }
    fn GlobalHandle(&self, p_mem: ConstPtr<::core::ffi::c_void>) -> PtrDiffRepr {
        todo!()
    }
    fn GlobalLock(&self, h_mem: PtrDiffRepr) -> MutPtr<::core::ffi::c_void> {
        todo!()
    }
    fn GlobalReAlloc(&self, h_mem: PtrDiffRepr, dw_bytes: PtrRepr, u_flags: u32) -> PtrDiffRepr {
        todo!()
    }
    fn GlobalSize(&self, h_mem: PtrDiffRepr) -> PtrRepr {
        todo!()
    }
    fn GlobalUnlock(&self, h_mem: PtrDiffRepr) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn HeapAlloc(
        &self,
        h_heap: HeapHandle,
        dw_flags: HEAP_FLAGS,
        dw_bytes: PtrRepr,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!()
    }
    fn HeapCompact(&self, h_heap: HeapHandle, dw_flags: HEAP_FLAGS) -> PtrRepr {
        todo!()
    }
    fn HeapCreate(
        &self,
        fl_options: HEAP_FLAGS,
        dw_initial_size: PtrRepr,
        dw_maximum_size: PtrRepr,
    ) -> HeapHandle {
        todo!()
    }
    fn HeapDestroy(&self, h_heap: HeapHandle) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn HeapFree(
        &self,
        h_heap: HeapHandle,
        dw_flags: HEAP_FLAGS,
        lp_mem: ConstPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn HeapLock(&self, h_heap: HeapHandle) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn HeapQueryInformation(
        &self,
        heap_handle: HeapHandle,
        heap_information_class: HEAP_INFORMATION_CLASS,
        heap_information: MutPtr<::core::ffi::c_void>,
        heap_information_length: PtrRepr,
        return_length: MutPtr<PtrRepr>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn HeapReAlloc(
        &self,
        h_heap: HeapHandle,
        dw_flags: HEAP_FLAGS,
        lp_mem: ConstPtr<::core::ffi::c_void>,
        dw_bytes: PtrRepr,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!()
    }
    fn HeapSetInformation(
        &self,
        heap_handle: HeapHandle,
        heap_information_class: HEAP_INFORMATION_CLASS,
        heap_information: ConstPtr<::core::ffi::c_void>,
        heap_information_length: PtrRepr,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn HeapSize(
        &self,
        h_heap: HeapHandle,
        dw_flags: HEAP_FLAGS,
        lp_mem: ConstPtr<::core::ffi::c_void>,
    ) -> PtrRepr {
        todo!()
    }
    fn HeapSummary(
        &self,
        h_heap: super::super::Foundation::HANDLE,
        dw_flags: u32,
        lp_summary: MutPtr<HEAP_SUMMARY>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn HeapUnlock(&self, h_heap: HeapHandle) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn HeapValidate(
        &self,
        h_heap: HeapHandle,
        dw_flags: HEAP_FLAGS,
        lp_mem: ConstPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn HeapWalk(
        &self,
        h_heap: HeapHandle,
        lp_entry: MutPtr<PROCESS_HEAP_ENTRY>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsBadCodePtr(
        &self,
        lpfn: super::super::Foundation::FARPROC,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsBadReadPtr(
        &self,
        lp: ConstPtr<::core::ffi::c_void>,
        ucb: PtrRepr,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsBadStringPtrA(
        &self,
        lpsz: crate::core::PCSTR,
        ucch_max: PtrRepr,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsBadStringPtrW(
        &self,
        lpsz: crate::core::PCWSTR,
        ucch_max: PtrRepr,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsBadWritePtr(
        &self,
        lp: ConstPtr<::core::ffi::c_void>,
        ucb: PtrRepr,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn LocalAlloc(&self, u_flags: LOCAL_ALLOC_FLAGS, u_bytes: PtrRepr) -> PtrDiffRepr {
        todo!()
    }
    fn LocalFlags(&self, h_mem: PtrDiffRepr) -> u32 {
        todo!()
    }
    fn LocalFree(&self, h_mem: PtrDiffRepr) -> PtrDiffRepr {
        todo!()
    }
    fn LocalHandle(&self, p_mem: ConstPtr<::core::ffi::c_void>) -> PtrDiffRepr {
        todo!()
    }
    fn LocalLock(&self, h_mem: PtrDiffRepr) -> MutPtr<::core::ffi::c_void> {
        todo!()
    }
    fn LocalReAlloc(&self, h_mem: PtrDiffRepr, u_bytes: PtrRepr, u_flags: u32) -> PtrDiffRepr {
        todo!()
    }
    fn LocalSize(&self, h_mem: PtrDiffRepr) -> PtrRepr {
        todo!()
    }
    fn LocalUnlock(&self, h_mem: PtrDiffRepr) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn MapUserPhysicalPages(
        &self,
        virtual_address: ConstPtr<::core::ffi::c_void>,
        number_of_pages: PtrRepr,
        page_array: ConstPtr<PtrRepr>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn MapUserPhysicalPagesScatter(
        &self,
        virtual_addresses: ConstPtr<ConstPtr<::core::ffi::c_void>>,
        number_of_pages: PtrRepr,
        page_array: ConstPtr<PtrRepr>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn MapViewOfFile(
        &self,
        h_file_mapping_object: super::super::Foundation::HANDLE,
        dw_desired_access: FILE_MAP,
        dw_file_offset_high: u32,
        dw_file_offset_low: u32,
        dw_number_of_bytes_to_map: PtrRepr,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!()
    }
    fn MapViewOfFile3(
        &self,
        file_mapping: super::super::Foundation::HANDLE,
        process: super::super::Foundation::HANDLE,
        base_address: ConstPtr<::core::ffi::c_void>,
        offset: u64,
        view_size: PtrRepr,
        allocation_type: VIRTUAL_ALLOCATION_TYPE,
        page_protection: u32,
        extended_parameters: MutPtr<MEM_EXTENDED_PARAMETER>,
        parameter_count: u32,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!()
    }
    fn MapViewOfFile3FromApp(
        &self,
        file_mapping: super::super::Foundation::HANDLE,
        process: super::super::Foundation::HANDLE,
        base_address: ConstPtr<::core::ffi::c_void>,
        offset: u64,
        view_size: PtrRepr,
        allocation_type: VIRTUAL_ALLOCATION_TYPE,
        page_protection: u32,
        extended_parameters: MutPtr<MEM_EXTENDED_PARAMETER>,
        parameter_count: u32,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!()
    }
    fn MapViewOfFileEx(
        &self,
        h_file_mapping_object: super::super::Foundation::HANDLE,
        dw_desired_access: FILE_MAP,
        dw_file_offset_high: u32,
        dw_file_offset_low: u32,
        dw_number_of_bytes_to_map: PtrRepr,
        lp_base_address: ConstPtr<::core::ffi::c_void>,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!()
    }
    fn MapViewOfFileExNuma(
        &self,
        h_file_mapping_object: super::super::Foundation::HANDLE,
        dw_desired_access: FILE_MAP,
        dw_file_offset_high: u32,
        dw_file_offset_low: u32,
        dw_number_of_bytes_to_map: PtrRepr,
        lp_base_address: ConstPtr<::core::ffi::c_void>,
        nnd_preferred: u32,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!()
    }
    fn MapViewOfFileFromApp(
        &self,
        h_file_mapping_object: super::super::Foundation::HANDLE,
        desired_access: FILE_MAP,
        file_offset: u64,
        number_of_bytes_to_map: PtrRepr,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!()
    }
    fn MapViewOfFileNuma2(
        &self,
        file_mapping_handle: super::super::Foundation::HANDLE,
        process_handle: super::super::Foundation::HANDLE,
        offset: u64,
        base_address: ConstPtr<::core::ffi::c_void>,
        view_size: PtrRepr,
        allocation_type: u32,
        page_protection: u32,
        preferred_node: u32,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!()
    }
    fn OfferVirtualMemory(
        &self,
        virtual_address: MutPtr<::core::ffi::c_void>,
        size: PtrRepr,
        priority: OFFER_PRIORITY,
    ) -> u32 {
        todo!()
    }
    fn OpenDedicatedMemoryPartition(
        &self,
        partition: super::super::Foundation::HANDLE,
        dedicated_memory_type_id: u64,
        desired_access: u32,
        inherit_handle: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::HANDLE {
        todo!()
    }
    fn OpenFileMappingA(
        &self,
        dw_desired_access: u32,
        b_inherit_handle: super::super::Foundation::BOOL,
        lp_name: crate::core::PCSTR,
    ) -> super::super::Foundation::HANDLE {
        todo!()
    }
    fn OpenFileMappingFromApp(
        &self,
        desired_access: u32,
        inherit_handle: super::super::Foundation::BOOL,
        name: crate::core::PCWSTR,
    ) -> super::super::Foundation::HANDLE {
        todo!()
    }
    fn OpenFileMappingW(
        &self,
        dw_desired_access: u32,
        b_inherit_handle: super::super::Foundation::BOOL,
        lp_name: crate::core::PCWSTR,
    ) -> super::super::Foundation::HANDLE {
        todo!()
    }
    fn PrefetchVirtualMemory(
        &self,
        h_process: super::super::Foundation::HANDLE,
        number_of_entries: PtrRepr,
        virtual_addresses: ConstPtr<WIN32_MEMORY_RANGE_ENTRY>,
        flags: u32,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn QueryMemoryResourceNotification(
        &self,
        resource_notification_handle: super::super::Foundation::HANDLE,
        resource_state: MutPtr<super::super::Foundation::BOOL>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn QueryPartitionInformation(
        &self,
        partition: super::super::Foundation::HANDLE,
        partition_information_class: WIN32_MEMORY_PARTITION_INFORMATION_CLASS,
        partition_information: MutPtr<::core::ffi::c_void>,
        partition_information_length: u32,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn QueryVirtualMemoryInformation(
        &self,
        process: super::super::Foundation::HANDLE,
        virtual_address: ConstPtr<::core::ffi::c_void>,
        memory_information_class: WIN32_MEMORY_INFORMATION_CLASS,
        memory_information: MutPtr<::core::ffi::c_void>,
        memory_information_size: PtrRepr,
        return_size: MutPtr<PtrRepr>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn ReclaimVirtualMemory(
        &self,
        virtual_address: ConstPtr<::core::ffi::c_void>,
        size: PtrRepr,
    ) -> u32 {
        todo!()
    }
    fn RegisterBadMemoryNotification(
        &self,
        callback: PBAD_MEMORY_CALLBACK_ROUTINE,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!()
    }
    fn RemoveSecureMemoryCacheCallback(
        &self,
        pfn_call_back: PSECURE_MEMORY_CACHE_CALLBACK,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn ResetWriteWatch(
        &self,
        lp_base_address: ConstPtr<::core::ffi::c_void>,
        dw_region_size: PtrRepr,
    ) -> u32 {
        todo!()
    }
    fn RtlCompareMemory(
        &self,
        source_1: ConstPtr<::core::ffi::c_void>,
        source_2: ConstPtr<::core::ffi::c_void>,
        length: PtrRepr,
    ) -> PtrRepr {
        todo!()
    }
    fn RtlCrc32(
        &self,
        buffer: ConstPtr<::core::ffi::c_void>,
        size: PtrRepr,
        initial_crc: u32,
    ) -> u32 {
        todo!()
    }
    fn RtlCrc64(
        &self,
        buffer: ConstPtr<::core::ffi::c_void>,
        size: PtrRepr,
        initial_crc: u64,
    ) -> u64 {
        todo!()
    }
    fn RtlIsZeroMemory(
        &self,
        buffer: ConstPtr<::core::ffi::c_void>,
        length: PtrRepr,
    ) -> super::super::Foundation::BOOLEAN {
        todo!()
    }
    fn SetProcessValidCallTargets(
        &self,
        h_process: super::super::Foundation::HANDLE,
        virtual_address: ConstPtr<::core::ffi::c_void>,
        region_size: PtrRepr,
        number_of_offsets: u32,
        offset_information: MutPtr<CFG_CALL_TARGET_INFO>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetProcessValidCallTargetsForMappedView(
        &self,
        process: super::super::Foundation::HANDLE,
        virtual_address: ConstPtr<::core::ffi::c_void>,
        region_size: PtrRepr,
        number_of_offsets: u32,
        offset_information: MutPtr<CFG_CALL_TARGET_INFO>,
        section: super::super::Foundation::HANDLE,
        expected_file_offset: u64,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetProcessWorkingSetSizeEx(
        &self,
        h_process: super::super::Foundation::HANDLE,
        dw_minimum_working_set_size: PtrRepr,
        dw_maximum_working_set_size: PtrRepr,
        flags: u32,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetSystemFileCacheSize(
        &self,
        minimum_file_cache_size: PtrRepr,
        maximum_file_cache_size: PtrRepr,
        flags: u32,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn UnmapViewOfFile(
        &self,
        lp_base_address: ConstPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn UnmapViewOfFile2(
        &self,
        process: super::super::Foundation::HANDLE,
        base_address: ConstPtr<::core::ffi::c_void>,
        unmap_flags: UNMAP_VIEW_OF_FILE_FLAGS,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn UnmapViewOfFileEx(
        &self,
        base_address: ConstPtr<::core::ffi::c_void>,
        unmap_flags: UNMAP_VIEW_OF_FILE_FLAGS,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn UnregisterBadMemoryNotification(
        &self,
        registration_handle: ConstPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn VirtualAlloc(
        &self,
        lp_address: ConstPtr<::core::ffi::c_void>,
        dw_size: PtrRepr,
        fl_allocation_type: VIRTUAL_ALLOCATION_TYPE,
        fl_protect: PAGE_PROTECTION_FLAGS,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!()
    }
    fn VirtualAlloc2(
        &self,
        process: super::super::Foundation::HANDLE,
        base_address: ConstPtr<::core::ffi::c_void>,
        size: PtrRepr,
        allocation_type: VIRTUAL_ALLOCATION_TYPE,
        page_protection: u32,
        extended_parameters: MutPtr<MEM_EXTENDED_PARAMETER>,
        parameter_count: u32,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!()
    }
    fn VirtualAlloc2FromApp(
        &self,
        process: super::super::Foundation::HANDLE,
        base_address: ConstPtr<::core::ffi::c_void>,
        size: PtrRepr,
        allocation_type: VIRTUAL_ALLOCATION_TYPE,
        page_protection: u32,
        extended_parameters: MutPtr<MEM_EXTENDED_PARAMETER>,
        parameter_count: u32,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!()
    }
    fn VirtualAllocEx(
        &self,
        h_process: super::super::Foundation::HANDLE,
        lp_address: ConstPtr<::core::ffi::c_void>,
        dw_size: PtrRepr,
        fl_allocation_type: VIRTUAL_ALLOCATION_TYPE,
        fl_protect: PAGE_PROTECTION_FLAGS,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!()
    }
    fn VirtualAllocExNuma(
        &self,
        h_process: super::super::Foundation::HANDLE,
        lp_address: ConstPtr<::core::ffi::c_void>,
        dw_size: PtrRepr,
        fl_allocation_type: VIRTUAL_ALLOCATION_TYPE,
        fl_protect: u32,
        nnd_preferred: u32,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!()
    }
    fn VirtualAllocFromApp(
        &self,
        base_address: ConstPtr<::core::ffi::c_void>,
        size: PtrRepr,
        allocation_type: VIRTUAL_ALLOCATION_TYPE,
        protection: u32,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!()
    }
    fn VirtualFree(
        &self,
        lp_address: MutPtr<::core::ffi::c_void>,
        dw_size: PtrRepr,
        dw_free_type: VIRTUAL_FREE_TYPE,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn VirtualFreeEx(
        &self,
        h_process: super::super::Foundation::HANDLE,
        lp_address: MutPtr<::core::ffi::c_void>,
        dw_size: PtrRepr,
        dw_free_type: VIRTUAL_FREE_TYPE,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn VirtualLock(
        &self,
        lp_address: ConstPtr<::core::ffi::c_void>,
        dw_size: PtrRepr,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn VirtualProtect(
        &self,
        lp_address: ConstPtr<::core::ffi::c_void>,
        dw_size: PtrRepr,
        fl_new_protect: PAGE_PROTECTION_FLAGS,
        lpfl_old_protect: MutPtr<PAGE_PROTECTION_FLAGS>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn VirtualProtectEx(
        &self,
        h_process: super::super::Foundation::HANDLE,
        lp_address: ConstPtr<::core::ffi::c_void>,
        dw_size: PtrRepr,
        fl_new_protect: PAGE_PROTECTION_FLAGS,
        lpfl_old_protect: MutPtr<PAGE_PROTECTION_FLAGS>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn VirtualProtectFromApp(
        &self,
        address: ConstPtr<::core::ffi::c_void>,
        size: PtrRepr,
        new_protection: u32,
        old_protection: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn VirtualQuery(
        &self,
        lp_address: ConstPtr<::core::ffi::c_void>,
        lp_buffer: MutPtr<MEMORY_BASIC_INFORMATION>,
        dw_length: PtrRepr,
    ) -> PtrRepr {
        todo!()
    }
    fn VirtualQueryEx(
        &self,
        h_process: super::super::Foundation::HANDLE,
        lp_address: ConstPtr<::core::ffi::c_void>,
        lp_buffer: MutPtr<MEMORY_BASIC_INFORMATION>,
        dw_length: PtrRepr,
    ) -> PtrRepr {
        todo!()
    }
    fn VirtualUnlock(
        &self,
        lp_address: ConstPtr<::core::ffi::c_void>,
        dw_size: PtrRepr,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn VirtualUnlockEx(
        &self,
        process: super::super::Foundation::HANDLE,
        address: ConstPtr<::core::ffi::c_void>,
        size: PtrRepr,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
}
pub fn get_api(ctx: &crate::core::Win32Context) -> &dyn Api {
    ctx.get::<dyn Api>()
}
