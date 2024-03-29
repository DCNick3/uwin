#![doc = r" do not edit! File auto-generated with win32-bindgen"]
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
pub struct CACHE_DESCRIPTOR {
    pub Level: u8,
    pub Associativity: u8,
    pub LineSize: u16,
    pub Size: u32,
    pub Type: PROCESSOR_CACHE_TYPE,
}
impl ::core::marker::Copy for CACHE_DESCRIPTOR {}
impl ::core::clone::Clone for CACHE_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CACHE_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CACHE_DESCRIPTOR")
            .field("Level", &self.Level)
            .field("Associativity", &self.Associativity)
            .field("LineSize", &self.LineSize)
            .field("Size", &self.Size)
            .field("Type", &self.Type)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CACHE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Level == other.Level
            && self.Associativity == other.Associativity
            && self.LineSize == other.LineSize
            && self.Size == other.Size
            && self.Type == other.Type
    }
}
impl ::core::cmp::Eq for CACHE_DESCRIPTOR {}
impl FromIntoMemory for CACHE_DESCRIPTOR {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Level = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_Associativity = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_LineSize = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_Size = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Type = <PROCESSOR_CACHE_TYPE as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            Level: f_Level,
            Associativity: f_Associativity,
            LineSize: f_LineSize,
            Size: f_Size,
            Type: f_Type,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Level, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.Associativity, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.LineSize, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.Size, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Type, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct CACHE_RELATIONSHIP {
    pub Level: u8,
    pub Associativity: u8,
    pub LineSize: u16,
    pub CacheSize: u32,
    pub Type: PROCESSOR_CACHE_TYPE,
    pub Reserved: [u8; 18],
    pub GroupCount: u16,
    pub Anonymous: CACHE_RELATIONSHIP_0,
}
impl ::core::marker::Copy for CACHE_RELATIONSHIP {}
impl ::core::clone::Clone for CACHE_RELATIONSHIP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CACHE_RELATIONSHIP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CACHE_RELATIONSHIP")
            .field("Level", &self.Level)
            .field("Associativity", &self.Associativity)
            .field("LineSize", &self.LineSize)
            .field("CacheSize", &self.CacheSize)
            .field("Type", &self.Type)
            .field("Reserved", &self.Reserved)
            .field("GroupCount", &self.GroupCount)
            .field("Anonymous", &self.Anonymous)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CACHE_RELATIONSHIP {
    fn eq(&self, other: &Self) -> bool {
        self.Level == other.Level
            && self.Associativity == other.Associativity
            && self.LineSize == other.LineSize
            && self.CacheSize == other.CacheSize
            && self.Type == other.Type
            && self.Reserved == other.Reserved
            && self.GroupCount == other.GroupCount
            && self.Anonymous == other.Anonymous
    }
}
impl ::core::cmp::Eq for CACHE_RELATIONSHIP {}
impl FromIntoMemory for CACHE_RELATIONSHIP {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 44);
        let f_Level = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_Associativity = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_LineSize = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_CacheSize = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Type = <PROCESSOR_CACHE_TYPE as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_Reserved = <[u8; 18] as FromIntoMemory>::from_bytes(&from[12..12 + 18]);
        let f_GroupCount = <u16 as FromIntoMemory>::from_bytes(&from[30..30 + 2]);
        let f_Anonymous = <CACHE_RELATIONSHIP_0 as FromIntoMemory>::from_bytes(&from[32..32 + 12]);
        Self {
            Level: f_Level,
            Associativity: f_Associativity,
            LineSize: f_LineSize,
            CacheSize: f_CacheSize,
            Type: f_Type,
            Reserved: f_Reserved,
            GroupCount: f_GroupCount,
            Anonymous: f_Anonymous,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 44);
        FromIntoMemory::into_bytes(self.Level, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.Associativity, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.LineSize, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.CacheSize, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Type, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[12..12 + 18]);
        FromIntoMemory::into_bytes(self.GroupCount, &mut into[30..30 + 2]);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[32..32 + 12]);
    }
    fn size() -> usize {
        44
    }
}
pub struct CACHE_RELATIONSHIP_0 {
    data: [u8; 12],
}
impl ::core::default::Default for CACHE_RELATIONSHIP_0 {
    fn default() -> Self {
        Self { data: [0u8; 12] }
    }
}
impl ::core::marker::Copy for CACHE_RELATIONSHIP_0 {}
impl ::core::clone::Clone for CACHE_RELATIONSHIP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CACHE_RELATIONSHIP_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CACHE_RELATIONSHIP_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CACHE_RELATIONSHIP_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for CACHE_RELATIONSHIP_0 {}
impl FromIntoMemory for CACHE_RELATIONSHIP_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 12];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        12
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMPUTER_NAME_FORMAT(pub i32);
pub const ComputerNameNetBIOS: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(0i32);
pub const ComputerNameDnsHostname: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(1i32);
pub const ComputerNameDnsDomain: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(2i32);
pub const ComputerNameDnsFullyQualified: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(3i32);
pub const ComputerNamePhysicalNetBIOS: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(4i32);
pub const ComputerNamePhysicalDnsHostname: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(5i32);
pub const ComputerNamePhysicalDnsDomain: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(6i32);
pub const ComputerNamePhysicalDnsFullyQualified: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(7i32);
pub const ComputerNameMax: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(8i32);
impl ::core::marker::Copy for COMPUTER_NAME_FORMAT {}
impl ::core::clone::Clone for COMPUTER_NAME_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMPUTER_NAME_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COMPUTER_NAME_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMPUTER_NAME_FORMAT")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for COMPUTER_NAME_FORMAT {
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
pub struct CPU_SET_INFORMATION_TYPE(pub i32);
pub const CpuSetInformation: CPU_SET_INFORMATION_TYPE = CPU_SET_INFORMATION_TYPE(0i32);
impl ::core::marker::Copy for CPU_SET_INFORMATION_TYPE {}
impl ::core::clone::Clone for CPU_SET_INFORMATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CPU_SET_INFORMATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CPU_SET_INFORMATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CPU_SET_INFORMATION_TYPE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for CPU_SET_INFORMATION_TYPE {
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
pub struct DEP_SYSTEM_POLICY_TYPE(pub i32);
pub const DEPPolicyAlwaysOff: DEP_SYSTEM_POLICY_TYPE = DEP_SYSTEM_POLICY_TYPE(0i32);
pub const DEPPolicyAlwaysOn: DEP_SYSTEM_POLICY_TYPE = DEP_SYSTEM_POLICY_TYPE(1i32);
pub const DEPPolicyOptIn: DEP_SYSTEM_POLICY_TYPE = DEP_SYSTEM_POLICY_TYPE(2i32);
pub const DEPPolicyOptOut: DEP_SYSTEM_POLICY_TYPE = DEP_SYSTEM_POLICY_TYPE(3i32);
pub const DEPTotalPolicyCount: DEP_SYSTEM_POLICY_TYPE = DEP_SYSTEM_POLICY_TYPE(4i32);
impl ::core::marker::Copy for DEP_SYSTEM_POLICY_TYPE {}
impl ::core::clone::Clone for DEP_SYSTEM_POLICY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEP_SYSTEM_POLICY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEP_SYSTEM_POLICY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEP_SYSTEM_POLICY_TYPE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for DEP_SYSTEM_POLICY_TYPE {
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
pub struct DEVICEFAMILYDEVICEFORM(pub u32);
pub const DEVICEFAMILYDEVICEFORM_UNKNOWN: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(0u32);
pub const DEVICEFAMILYDEVICEFORM_PHONE: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(1u32);
pub const DEVICEFAMILYDEVICEFORM_TABLET: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(2u32);
pub const DEVICEFAMILYDEVICEFORM_DESKTOP: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(3u32);
pub const DEVICEFAMILYDEVICEFORM_NOTEBOOK: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(4u32);
pub const DEVICEFAMILYDEVICEFORM_CONVERTIBLE: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(5u32);
pub const DEVICEFAMILYDEVICEFORM_DETACHABLE: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(6u32);
pub const DEVICEFAMILYDEVICEFORM_ALLINONE: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(7u32);
pub const DEVICEFAMILYDEVICEFORM_STICKPC: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(8u32);
pub const DEVICEFAMILYDEVICEFORM_PUCK: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(9u32);
pub const DEVICEFAMILYDEVICEFORM_LARGESCREEN: DEVICEFAMILYDEVICEFORM =
    DEVICEFAMILYDEVICEFORM(10u32);
pub const DEVICEFAMILYDEVICEFORM_HMD: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(11u32);
pub const DEVICEFAMILYDEVICEFORM_INDUSTRY_HANDHELD: DEVICEFAMILYDEVICEFORM =
    DEVICEFAMILYDEVICEFORM(12u32);
pub const DEVICEFAMILYDEVICEFORM_INDUSTRY_TABLET: DEVICEFAMILYDEVICEFORM =
    DEVICEFAMILYDEVICEFORM(13u32);
pub const DEVICEFAMILYDEVICEFORM_BANKING: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(14u32);
pub const DEVICEFAMILYDEVICEFORM_BUILDING_AUTOMATION: DEVICEFAMILYDEVICEFORM =
    DEVICEFAMILYDEVICEFORM(15u32);
pub const DEVICEFAMILYDEVICEFORM_DIGITAL_SIGNAGE: DEVICEFAMILYDEVICEFORM =
    DEVICEFAMILYDEVICEFORM(16u32);
pub const DEVICEFAMILYDEVICEFORM_GAMING: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(17u32);
pub const DEVICEFAMILYDEVICEFORM_HOME_AUTOMATION: DEVICEFAMILYDEVICEFORM =
    DEVICEFAMILYDEVICEFORM(18u32);
pub const DEVICEFAMILYDEVICEFORM_INDUSTRIAL_AUTOMATION: DEVICEFAMILYDEVICEFORM =
    DEVICEFAMILYDEVICEFORM(19u32);
pub const DEVICEFAMILYDEVICEFORM_KIOSK: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(20u32);
pub const DEVICEFAMILYDEVICEFORM_MAKER_BOARD: DEVICEFAMILYDEVICEFORM =
    DEVICEFAMILYDEVICEFORM(21u32);
pub const DEVICEFAMILYDEVICEFORM_MEDICAL: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(22u32);
pub const DEVICEFAMILYDEVICEFORM_NETWORKING: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(23u32);
pub const DEVICEFAMILYDEVICEFORM_POINT_OF_SERVICE: DEVICEFAMILYDEVICEFORM =
    DEVICEFAMILYDEVICEFORM(24u32);
pub const DEVICEFAMILYDEVICEFORM_PRINTING: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(25u32);
pub const DEVICEFAMILYDEVICEFORM_THIN_CLIENT: DEVICEFAMILYDEVICEFORM =
    DEVICEFAMILYDEVICEFORM(26u32);
pub const DEVICEFAMILYDEVICEFORM_TOY: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(27u32);
pub const DEVICEFAMILYDEVICEFORM_VENDING: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(28u32);
pub const DEVICEFAMILYDEVICEFORM_INDUSTRY_OTHER: DEVICEFAMILYDEVICEFORM =
    DEVICEFAMILYDEVICEFORM(29u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(30u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE_S: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(31u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE_X: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(32u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE_X_DEVKIT: DEVICEFAMILYDEVICEFORM =
    DEVICEFAMILYDEVICEFORM(33u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_SERIES_X: DEVICEFAMILYDEVICEFORM =
    DEVICEFAMILYDEVICEFORM(34u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_SERIES_X_DEVKIT: DEVICEFAMILYDEVICEFORM =
    DEVICEFAMILYDEVICEFORM(35u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_00: DEVICEFAMILYDEVICEFORM =
    DEVICEFAMILYDEVICEFORM(36u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_01: DEVICEFAMILYDEVICEFORM =
    DEVICEFAMILYDEVICEFORM(37u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_02: DEVICEFAMILYDEVICEFORM =
    DEVICEFAMILYDEVICEFORM(38u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_03: DEVICEFAMILYDEVICEFORM =
    DEVICEFAMILYDEVICEFORM(39u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_04: DEVICEFAMILYDEVICEFORM =
    DEVICEFAMILYDEVICEFORM(40u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_05: DEVICEFAMILYDEVICEFORM =
    DEVICEFAMILYDEVICEFORM(41u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_06: DEVICEFAMILYDEVICEFORM =
    DEVICEFAMILYDEVICEFORM(42u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_07: DEVICEFAMILYDEVICEFORM =
    DEVICEFAMILYDEVICEFORM(43u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_08: DEVICEFAMILYDEVICEFORM =
    DEVICEFAMILYDEVICEFORM(44u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_09: DEVICEFAMILYDEVICEFORM =
    DEVICEFAMILYDEVICEFORM(45u32);
pub const DEVICEFAMILYDEVICEFORM_MAX: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(45u32);
impl ::core::marker::Copy for DEVICEFAMILYDEVICEFORM {}
impl ::core::clone::Clone for DEVICEFAMILYDEVICEFORM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEVICEFAMILYDEVICEFORM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEVICEFAMILYDEVICEFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVICEFAMILYDEVICEFORM")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for DEVICEFAMILYDEVICEFORM {
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
pub struct DEVICEFAMILYINFOENUM(pub u32);
pub const DEVICEFAMILYINFOENUM_UAP: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(0u32);
pub const DEVICEFAMILYINFOENUM_WINDOWS_8X: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(1u32);
pub const DEVICEFAMILYINFOENUM_WINDOWS_PHONE_8X: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(2u32);
pub const DEVICEFAMILYINFOENUM_DESKTOP: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(3u32);
pub const DEVICEFAMILYINFOENUM_MOBILE: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(4u32);
pub const DEVICEFAMILYINFOENUM_XBOX: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(5u32);
pub const DEVICEFAMILYINFOENUM_TEAM: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(6u32);
pub const DEVICEFAMILYINFOENUM_IOT: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(7u32);
pub const DEVICEFAMILYINFOENUM_IOT_HEADLESS: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(8u32);
pub const DEVICEFAMILYINFOENUM_SERVER: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(9u32);
pub const DEVICEFAMILYINFOENUM_HOLOGRAPHIC: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(10u32);
pub const DEVICEFAMILYINFOENUM_XBOXSRA: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(11u32);
pub const DEVICEFAMILYINFOENUM_XBOXERA: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(12u32);
pub const DEVICEFAMILYINFOENUM_SERVER_NANO: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(13u32);
pub const DEVICEFAMILYINFOENUM_8828080: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(14u32);
pub const DEVICEFAMILYINFOENUM_7067329: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(15u32);
pub const DEVICEFAMILYINFOENUM_WINDOWS_CORE: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(16u32);
pub const DEVICEFAMILYINFOENUM_WINDOWS_CORE_HEADLESS: DEVICEFAMILYINFOENUM =
    DEVICEFAMILYINFOENUM(17u32);
pub const DEVICEFAMILYINFOENUM_MAX: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(17u32);
impl ::core::marker::Copy for DEVICEFAMILYINFOENUM {}
impl ::core::clone::Clone for DEVICEFAMILYINFOENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEVICEFAMILYINFOENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEVICEFAMILYINFOENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVICEFAMILYINFOENUM")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for DEVICEFAMILYINFOENUM {
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
pub struct FIRMWARE_TABLE_ID(pub u32);
impl FIRMWARE_TABLE_ID {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FIRMWARE_TABLE_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for FIRMWARE_TABLE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for FIRMWARE_TABLE_ID {}
impl ::core::hash::Hash for FIRMWARE_TABLE_ID {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for FIRMWARE_TABLE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FIRMWARE_TABLE_ID").field(&self.0).finish()
    }
}
impl FromIntoMemory for FIRMWARE_TABLE_ID {
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
pub struct FIRMWARE_TABLE_PROVIDER(pub u32);
pub const ACPI: FIRMWARE_TABLE_PROVIDER = FIRMWARE_TABLE_PROVIDER(1094930505u32);
pub const FIRM: FIRMWARE_TABLE_PROVIDER = FIRMWARE_TABLE_PROVIDER(1179210317u32);
pub const RSMB: FIRMWARE_TABLE_PROVIDER = FIRMWARE_TABLE_PROVIDER(1381190978u32);
impl ::core::marker::Copy for FIRMWARE_TABLE_PROVIDER {}
impl ::core::clone::Clone for FIRMWARE_TABLE_PROVIDER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FIRMWARE_TABLE_PROVIDER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FIRMWARE_TABLE_PROVIDER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FIRMWARE_TABLE_PROVIDER")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for FIRMWARE_TABLE_PROVIDER {
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
pub struct FIRMWARE_TYPE(pub i32);
pub const FirmwareTypeUnknown: FIRMWARE_TYPE = FIRMWARE_TYPE(0i32);
pub const FirmwareTypeBios: FIRMWARE_TYPE = FIRMWARE_TYPE(1i32);
pub const FirmwareTypeUefi: FIRMWARE_TYPE = FIRMWARE_TYPE(2i32);
pub const FirmwareTypeMax: FIRMWARE_TYPE = FIRMWARE_TYPE(3i32);
impl ::core::marker::Copy for FIRMWARE_TYPE {}
impl ::core::clone::Clone for FIRMWARE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FIRMWARE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FIRMWARE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FIRMWARE_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for FIRMWARE_TYPE {
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
pub struct GROUP_AFFINITY {
    pub Mask: PtrRepr,
    pub Group: u16,
    pub Reserved: [u16; 3],
}
impl ::core::marker::Copy for GROUP_AFFINITY {}
impl ::core::clone::Clone for GROUP_AFFINITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GROUP_AFFINITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_AFFINITY")
            .field("Mask", &self.Mask)
            .field("Group", &self.Group)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for GROUP_AFFINITY {
    fn eq(&self, other: &Self) -> bool {
        self.Mask == other.Mask && self.Group == other.Group && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for GROUP_AFFINITY {}
impl FromIntoMemory for GROUP_AFFINITY {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Mask = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Group = <u16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_Reserved = <[u16; 3] as FromIntoMemory>::from_bytes(&from[6..6 + 6]);
        Self {
            Mask: f_Mask,
            Group: f_Group,
            Reserved: f_Reserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Mask, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Group, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[6..6 + 6]);
    }
    fn size() -> usize {
        12
    }
}
pub struct GROUP_RELATIONSHIP {
    pub MaximumGroupCount: u16,
    pub ActiveGroupCount: u16,
    pub Reserved: [u8; 20],
    pub GroupInfo: [PROCESSOR_GROUP_INFO; 1],
}
impl ::core::marker::Copy for GROUP_RELATIONSHIP {}
impl ::core::clone::Clone for GROUP_RELATIONSHIP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GROUP_RELATIONSHIP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_RELATIONSHIP")
            .field("MaximumGroupCount", &self.MaximumGroupCount)
            .field("ActiveGroupCount", &self.ActiveGroupCount)
            .field("Reserved", &self.Reserved)
            .field("GroupInfo", &self.GroupInfo)
            .finish()
    }
}
impl ::core::cmp::PartialEq for GROUP_RELATIONSHIP {
    fn eq(&self, other: &Self) -> bool {
        self.MaximumGroupCount == other.MaximumGroupCount
            && self.ActiveGroupCount == other.ActiveGroupCount
            && self.Reserved == other.Reserved
            && self.GroupInfo == other.GroupInfo
    }
}
impl ::core::cmp::Eq for GROUP_RELATIONSHIP {}
impl FromIntoMemory for GROUP_RELATIONSHIP {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 68);
        let f_MaximumGroupCount = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_ActiveGroupCount = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_Reserved = <[u8; 20] as FromIntoMemory>::from_bytes(&from[4..4 + 20]);
        let f_GroupInfo =
            <[PROCESSOR_GROUP_INFO; 1] as FromIntoMemory>::from_bytes(&from[24..24 + 44]);
        Self {
            MaximumGroupCount: f_MaximumGroupCount,
            ActiveGroupCount: f_ActiveGroupCount,
            Reserved: f_Reserved,
            GroupInfo: f_GroupInfo,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 68);
        FromIntoMemory::into_bytes(self.MaximumGroupCount, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.ActiveGroupCount, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[4..4 + 20]);
        FromIntoMemory::into_bytes(self.GroupInfo, &mut into[24..24 + 44]);
    }
    fn size() -> usize {
        68
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LOGICAL_PROCESSOR_RELATIONSHIP(pub i32);
pub const RelationProcessorCore: LOGICAL_PROCESSOR_RELATIONSHIP =
    LOGICAL_PROCESSOR_RELATIONSHIP(0i32);
pub const RelationNumaNode: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(1i32);
pub const RelationCache: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(2i32);
pub const RelationProcessorPackage: LOGICAL_PROCESSOR_RELATIONSHIP =
    LOGICAL_PROCESSOR_RELATIONSHIP(3i32);
pub const RelationGroup: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(4i32);
pub const RelationProcessorDie: LOGICAL_PROCESSOR_RELATIONSHIP =
    LOGICAL_PROCESSOR_RELATIONSHIP(5i32);
pub const RelationNumaNodeEx: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(6i32);
pub const RelationProcessorModule: LOGICAL_PROCESSOR_RELATIONSHIP =
    LOGICAL_PROCESSOR_RELATIONSHIP(7i32);
pub const RelationAll: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(65535i32);
impl ::core::marker::Copy for LOGICAL_PROCESSOR_RELATIONSHIP {}
impl ::core::clone::Clone for LOGICAL_PROCESSOR_RELATIONSHIP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LOGICAL_PROCESSOR_RELATIONSHIP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LOGICAL_PROCESSOR_RELATIONSHIP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOGICAL_PROCESSOR_RELATIONSHIP")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for LOGICAL_PROCESSOR_RELATIONSHIP {
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
pub struct MEMORYSTATUS {
    pub dwLength: u32,
    pub dwMemoryLoad: u32,
    pub dwTotalPhys: PtrRepr,
    pub dwAvailPhys: PtrRepr,
    pub dwTotalPageFile: PtrRepr,
    pub dwAvailPageFile: PtrRepr,
    pub dwTotalVirtual: PtrRepr,
    pub dwAvailVirtual: PtrRepr,
}
impl ::core::marker::Copy for MEMORYSTATUS {}
impl ::core::clone::Clone for MEMORYSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MEMORYSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEMORYSTATUS")
            .field("dwLength", &self.dwLength)
            .field("dwMemoryLoad", &self.dwMemoryLoad)
            .field("dwTotalPhys", &self.dwTotalPhys)
            .field("dwAvailPhys", &self.dwAvailPhys)
            .field("dwTotalPageFile", &self.dwTotalPageFile)
            .field("dwAvailPageFile", &self.dwAvailPageFile)
            .field("dwTotalVirtual", &self.dwTotalVirtual)
            .field("dwAvailVirtual", &self.dwAvailVirtual)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MEMORYSTATUS {
    fn eq(&self, other: &Self) -> bool {
        self.dwLength == other.dwLength
            && self.dwMemoryLoad == other.dwMemoryLoad
            && self.dwTotalPhys == other.dwTotalPhys
            && self.dwAvailPhys == other.dwAvailPhys
            && self.dwTotalPageFile == other.dwTotalPageFile
            && self.dwAvailPageFile == other.dwAvailPageFile
            && self.dwTotalVirtual == other.dwTotalVirtual
            && self.dwAvailVirtual == other.dwAvailVirtual
    }
}
impl ::core::cmp::Eq for MEMORYSTATUS {}
impl FromIntoMemory for MEMORYSTATUS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_dwLength = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwMemoryLoad = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwTotalPhys = <PtrRepr as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwAvailPhys = <PtrRepr as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwTotalPageFile = <PtrRepr as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwAvailPageFile = <PtrRepr as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwTotalVirtual = <PtrRepr as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_dwAvailVirtual = <PtrRepr as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            dwLength: f_dwLength,
            dwMemoryLoad: f_dwMemoryLoad,
            dwTotalPhys: f_dwTotalPhys,
            dwAvailPhys: f_dwAvailPhys,
            dwTotalPageFile: f_dwTotalPageFile,
            dwAvailPageFile: f_dwAvailPageFile,
            dwTotalVirtual: f_dwTotalVirtual,
            dwAvailVirtual: f_dwAvailVirtual,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.dwLength, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwMemoryLoad, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwTotalPhys, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwAvailPhys, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwTotalPageFile, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwAvailPageFile, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwTotalVirtual, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.dwAvailVirtual, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
pub struct MEMORYSTATUSEX {
    pub dwLength: u32,
    pub dwMemoryLoad: u32,
    pub ullTotalPhys: u64,
    pub ullAvailPhys: u64,
    pub ullTotalPageFile: u64,
    pub ullAvailPageFile: u64,
    pub ullTotalVirtual: u64,
    pub ullAvailVirtual: u64,
    pub ullAvailExtendedVirtual: u64,
}
impl ::core::marker::Copy for MEMORYSTATUSEX {}
impl ::core::clone::Clone for MEMORYSTATUSEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MEMORYSTATUSEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEMORYSTATUSEX")
            .field("dwLength", &self.dwLength)
            .field("dwMemoryLoad", &self.dwMemoryLoad)
            .field("ullTotalPhys", &self.ullTotalPhys)
            .field("ullAvailPhys", &self.ullAvailPhys)
            .field("ullTotalPageFile", &self.ullTotalPageFile)
            .field("ullAvailPageFile", &self.ullAvailPageFile)
            .field("ullTotalVirtual", &self.ullTotalVirtual)
            .field("ullAvailVirtual", &self.ullAvailVirtual)
            .field("ullAvailExtendedVirtual", &self.ullAvailExtendedVirtual)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MEMORYSTATUSEX {
    fn eq(&self, other: &Self) -> bool {
        self.dwLength == other.dwLength
            && self.dwMemoryLoad == other.dwMemoryLoad
            && self.ullTotalPhys == other.ullTotalPhys
            && self.ullAvailPhys == other.ullAvailPhys
            && self.ullTotalPageFile == other.ullTotalPageFile
            && self.ullAvailPageFile == other.ullAvailPageFile
            && self.ullTotalVirtual == other.ullTotalVirtual
            && self.ullAvailVirtual == other.ullAvailVirtual
            && self.ullAvailExtendedVirtual == other.ullAvailExtendedVirtual
    }
}
impl ::core::cmp::Eq for MEMORYSTATUSEX {}
impl FromIntoMemory for MEMORYSTATUSEX {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 64);
        let f_dwLength = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwMemoryLoad = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_ullTotalPhys = <u64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_ullAvailPhys = <u64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_ullTotalPageFile = <u64 as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        let f_ullAvailPageFile = <u64 as FromIntoMemory>::from_bytes(&from[32..32 + 8]);
        let f_ullTotalVirtual = <u64 as FromIntoMemory>::from_bytes(&from[40..40 + 8]);
        let f_ullAvailVirtual = <u64 as FromIntoMemory>::from_bytes(&from[48..48 + 8]);
        let f_ullAvailExtendedVirtual = <u64 as FromIntoMemory>::from_bytes(&from[56..56 + 8]);
        Self {
            dwLength: f_dwLength,
            dwMemoryLoad: f_dwMemoryLoad,
            ullTotalPhys: f_ullTotalPhys,
            ullAvailPhys: f_ullAvailPhys,
            ullTotalPageFile: f_ullTotalPageFile,
            ullAvailPageFile: f_ullAvailPageFile,
            ullTotalVirtual: f_ullTotalVirtual,
            ullAvailVirtual: f_ullAvailVirtual,
            ullAvailExtendedVirtual: f_ullAvailExtendedVirtual,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 64);
        FromIntoMemory::into_bytes(self.dwLength, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwMemoryLoad, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.ullTotalPhys, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.ullAvailPhys, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.ullTotalPageFile, &mut into[24..24 + 8]);
        FromIntoMemory::into_bytes(self.ullAvailPageFile, &mut into[32..32 + 8]);
        FromIntoMemory::into_bytes(self.ullTotalVirtual, &mut into[40..40 + 8]);
        FromIntoMemory::into_bytes(self.ullAvailVirtual, &mut into[48..48 + 8]);
        FromIntoMemory::into_bytes(self.ullAvailExtendedVirtual, &mut into[56..56 + 8]);
    }
    fn size() -> usize {
        64
    }
}
pub const NTDDI_LONGHORN: u32 = 100663296u32;
pub const NTDDI_VERSION: u32 = 167772171u32;
pub const NTDDI_VISTA: u32 = 100663296u32;
pub const NTDDI_VISTASP1: u32 = 100663552u32;
pub const NTDDI_VISTASP2: u32 = 100663808u32;
pub const NTDDI_VISTASP3: u32 = 100664064u32;
pub const NTDDI_VISTASP4: u32 = 100664320u32;
pub const NTDDI_WIN10: u32 = 167772160u32;
pub const NTDDI_WIN10_19H1: u32 = 167772167u32;
pub const NTDDI_WIN10_CO: u32 = 167772171u32;
pub const NTDDI_WIN10_FE: u32 = 167772170u32;
pub const NTDDI_WIN10_MN: u32 = 167772169u32;
pub const NTDDI_WIN10_RS1: u32 = 167772162u32;
pub const NTDDI_WIN10_RS2: u32 = 167772163u32;
pub const NTDDI_WIN10_RS3: u32 = 167772164u32;
pub const NTDDI_WIN10_RS4: u32 = 167772165u32;
pub const NTDDI_WIN10_RS5: u32 = 167772166u32;
pub const NTDDI_WIN10_TH2: u32 = 167772161u32;
pub const NTDDI_WIN10_VB: u32 = 167772168u32;
pub const NTDDI_WIN2K: u32 = 83886080u32;
pub const NTDDI_WIN2KSP1: u32 = 83886336u32;
pub const NTDDI_WIN2KSP2: u32 = 83886592u32;
pub const NTDDI_WIN2KSP3: u32 = 83886848u32;
pub const NTDDI_WIN2KSP4: u32 = 83887104u32;
pub const NTDDI_WIN4: u32 = 67108864u32;
pub const NTDDI_WIN6: u32 = 100663296u32;
pub const NTDDI_WIN6SP1: u32 = 100663552u32;
pub const NTDDI_WIN6SP2: u32 = 100663808u32;
pub const NTDDI_WIN6SP3: u32 = 100664064u32;
pub const NTDDI_WIN6SP4: u32 = 100664320u32;
pub const NTDDI_WIN7: u32 = 100728832u32;
pub const NTDDI_WIN8: u32 = 100794368u32;
pub const NTDDI_WINBLUE: u32 = 100859904u32;
pub const NTDDI_WINTHRESHOLD: u32 = 167772160u32;
pub const NTDDI_WINXP: u32 = 83951616u32;
pub const NTDDI_WINXPSP1: u32 = 83951872u32;
pub const NTDDI_WINXPSP2: u32 = 83952128u32;
pub const NTDDI_WINXPSP3: u32 = 83952384u32;
pub const NTDDI_WINXPSP4: u32 = 83952640u32;
pub const NTDDI_WS03: u32 = 84017152u32;
pub const NTDDI_WS03SP1: u32 = 84017408u32;
pub const NTDDI_WS03SP2: u32 = 84017664u32;
pub const NTDDI_WS03SP3: u32 = 84017920u32;
pub const NTDDI_WS03SP4: u32 = 84018176u32;
pub const NTDDI_WS08: u32 = 100663552u32;
pub const NTDDI_WS08SP2: u32 = 100663808u32;
pub const NTDDI_WS08SP3: u32 = 100664064u32;
pub const NTDDI_WS08SP4: u32 = 100664320u32;
pub struct NUMA_NODE_RELATIONSHIP {
    pub NodeNumber: u32,
    pub Reserved: [u8; 18],
    pub GroupCount: u16,
    pub Anonymous: NUMA_NODE_RELATIONSHIP_0,
}
impl ::core::marker::Copy for NUMA_NODE_RELATIONSHIP {}
impl ::core::clone::Clone for NUMA_NODE_RELATIONSHIP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NUMA_NODE_RELATIONSHIP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NUMA_NODE_RELATIONSHIP")
            .field("NodeNumber", &self.NodeNumber)
            .field("Reserved", &self.Reserved)
            .field("GroupCount", &self.GroupCount)
            .field("Anonymous", &self.Anonymous)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NUMA_NODE_RELATIONSHIP {
    fn eq(&self, other: &Self) -> bool {
        self.NodeNumber == other.NodeNumber
            && self.Reserved == other.Reserved
            && self.GroupCount == other.GroupCount
            && self.Anonymous == other.Anonymous
    }
}
impl ::core::cmp::Eq for NUMA_NODE_RELATIONSHIP {}
impl FromIntoMemory for NUMA_NODE_RELATIONSHIP {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 36);
        let f_NodeNumber = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Reserved = <[u8; 18] as FromIntoMemory>::from_bytes(&from[4..4 + 18]);
        let f_GroupCount = <u16 as FromIntoMemory>::from_bytes(&from[22..22 + 2]);
        let f_Anonymous =
            <NUMA_NODE_RELATIONSHIP_0 as FromIntoMemory>::from_bytes(&from[24..24 + 12]);
        Self {
            NodeNumber: f_NodeNumber,
            Reserved: f_Reserved,
            GroupCount: f_GroupCount,
            Anonymous: f_Anonymous,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 36);
        FromIntoMemory::into_bytes(self.NodeNumber, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[4..4 + 18]);
        FromIntoMemory::into_bytes(self.GroupCount, &mut into[22..22 + 2]);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[24..24 + 12]);
    }
    fn size() -> usize {
        36
    }
}
pub struct NUMA_NODE_RELATIONSHIP_0 {
    data: [u8; 12],
}
impl ::core::default::Default for NUMA_NODE_RELATIONSHIP_0 {
    fn default() -> Self {
        Self { data: [0u8; 12] }
    }
}
impl ::core::marker::Copy for NUMA_NODE_RELATIONSHIP_0 {}
impl ::core::clone::Clone for NUMA_NODE_RELATIONSHIP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NUMA_NODE_RELATIONSHIP_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NUMA_NODE_RELATIONSHIP_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NUMA_NODE_RELATIONSHIP_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for NUMA_NODE_RELATIONSHIP_0 {}
impl FromIntoMemory for NUMA_NODE_RELATIONSHIP_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 12];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        12
    }
}
pub struct OSVERSIONINFOA {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [super::super::Foundation::CHAR; 128],
}
impl ::core::marker::Copy for OSVERSIONINFOA {}
impl ::core::clone::Clone for OSVERSIONINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OSVERSIONINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OSVERSIONINFOA")
            .field("dwOSVersionInfoSize", &self.dwOSVersionInfoSize)
            .field("dwMajorVersion", &self.dwMajorVersion)
            .field("dwMinorVersion", &self.dwMinorVersion)
            .field("dwBuildNumber", &self.dwBuildNumber)
            .field("dwPlatformId", &self.dwPlatformId)
            .field("szCSDVersion", &self.szCSDVersion)
            .finish()
    }
}
impl ::core::cmp::PartialEq for OSVERSIONINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwOSVersionInfoSize == other.dwOSVersionInfoSize
            && self.dwMajorVersion == other.dwMajorVersion
            && self.dwMinorVersion == other.dwMinorVersion
            && self.dwBuildNumber == other.dwBuildNumber
            && self.dwPlatformId == other.dwPlatformId
            && self.szCSDVersion == other.szCSDVersion
    }
}
impl ::core::cmp::Eq for OSVERSIONINFOA {}
impl FromIntoMemory for OSVERSIONINFOA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 148);
        let f_dwOSVersionInfoSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwMajorVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwMinorVersion = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwBuildNumber = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwPlatformId = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_szCSDVersion = <[super::super::Foundation::CHAR; 128] as FromIntoMemory>::from_bytes(
            &from[20..20 + 128],
        );
        Self {
            dwOSVersionInfoSize: f_dwOSVersionInfoSize,
            dwMajorVersion: f_dwMajorVersion,
            dwMinorVersion: f_dwMinorVersion,
            dwBuildNumber: f_dwBuildNumber,
            dwPlatformId: f_dwPlatformId,
            szCSDVersion: f_szCSDVersion,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 148);
        FromIntoMemory::into_bytes(self.dwOSVersionInfoSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwMajorVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwMinorVersion, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwBuildNumber, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwPlatformId, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.szCSDVersion, &mut into[20..20 + 128]);
    }
    fn size() -> usize {
        148
    }
}
pub struct OSVERSIONINFOEXA {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [super::super::Foundation::CHAR; 128],
    pub wServicePackMajor: u16,
    pub wServicePackMinor: u16,
    pub wSuiteMask: u16,
    pub wProductType: u8,
    pub wReserved: u8,
}
impl ::core::marker::Copy for OSVERSIONINFOEXA {}
impl ::core::clone::Clone for OSVERSIONINFOEXA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OSVERSIONINFOEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OSVERSIONINFOEXA")
            .field("dwOSVersionInfoSize", &self.dwOSVersionInfoSize)
            .field("dwMajorVersion", &self.dwMajorVersion)
            .field("dwMinorVersion", &self.dwMinorVersion)
            .field("dwBuildNumber", &self.dwBuildNumber)
            .field("dwPlatformId", &self.dwPlatformId)
            .field("szCSDVersion", &self.szCSDVersion)
            .field("wServicePackMajor", &self.wServicePackMajor)
            .field("wServicePackMinor", &self.wServicePackMinor)
            .field("wSuiteMask", &self.wSuiteMask)
            .field("wProductType", &self.wProductType)
            .field("wReserved", &self.wReserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for OSVERSIONINFOEXA {
    fn eq(&self, other: &Self) -> bool {
        self.dwOSVersionInfoSize == other.dwOSVersionInfoSize
            && self.dwMajorVersion == other.dwMajorVersion
            && self.dwMinorVersion == other.dwMinorVersion
            && self.dwBuildNumber == other.dwBuildNumber
            && self.dwPlatformId == other.dwPlatformId
            && self.szCSDVersion == other.szCSDVersion
            && self.wServicePackMajor == other.wServicePackMajor
            && self.wServicePackMinor == other.wServicePackMinor
            && self.wSuiteMask == other.wSuiteMask
            && self.wProductType == other.wProductType
            && self.wReserved == other.wReserved
    }
}
impl ::core::cmp::Eq for OSVERSIONINFOEXA {}
impl FromIntoMemory for OSVERSIONINFOEXA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 156);
        let f_dwOSVersionInfoSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwMajorVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwMinorVersion = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwBuildNumber = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwPlatformId = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_szCSDVersion = <[super::super::Foundation::CHAR; 128] as FromIntoMemory>::from_bytes(
            &from[20..20 + 128],
        );
        let f_wServicePackMajor = <u16 as FromIntoMemory>::from_bytes(&from[148..148 + 2]);
        let f_wServicePackMinor = <u16 as FromIntoMemory>::from_bytes(&from[150..150 + 2]);
        let f_wSuiteMask = <u16 as FromIntoMemory>::from_bytes(&from[152..152 + 2]);
        let f_wProductType = <u8 as FromIntoMemory>::from_bytes(&from[154..154 + 1]);
        let f_wReserved = <u8 as FromIntoMemory>::from_bytes(&from[155..155 + 1]);
        Self {
            dwOSVersionInfoSize: f_dwOSVersionInfoSize,
            dwMajorVersion: f_dwMajorVersion,
            dwMinorVersion: f_dwMinorVersion,
            dwBuildNumber: f_dwBuildNumber,
            dwPlatformId: f_dwPlatformId,
            szCSDVersion: f_szCSDVersion,
            wServicePackMajor: f_wServicePackMajor,
            wServicePackMinor: f_wServicePackMinor,
            wSuiteMask: f_wSuiteMask,
            wProductType: f_wProductType,
            wReserved: f_wReserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 156);
        FromIntoMemory::into_bytes(self.dwOSVersionInfoSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwMajorVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwMinorVersion, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwBuildNumber, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwPlatformId, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.szCSDVersion, &mut into[20..20 + 128]);
        FromIntoMemory::into_bytes(self.wServicePackMajor, &mut into[148..148 + 2]);
        FromIntoMemory::into_bytes(self.wServicePackMinor, &mut into[150..150 + 2]);
        FromIntoMemory::into_bytes(self.wSuiteMask, &mut into[152..152 + 2]);
        FromIntoMemory::into_bytes(self.wProductType, &mut into[154..154 + 1]);
        FromIntoMemory::into_bytes(self.wReserved, &mut into[155..155 + 1]);
    }
    fn size() -> usize {
        156
    }
}
pub struct OSVERSIONINFOEXW {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [u16; 128],
    pub wServicePackMajor: u16,
    pub wServicePackMinor: u16,
    pub wSuiteMask: u16,
    pub wProductType: u8,
    pub wReserved: u8,
}
impl ::core::marker::Copy for OSVERSIONINFOEXW {}
impl ::core::clone::Clone for OSVERSIONINFOEXW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OSVERSIONINFOEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OSVERSIONINFOEXW")
            .field("dwOSVersionInfoSize", &self.dwOSVersionInfoSize)
            .field("dwMajorVersion", &self.dwMajorVersion)
            .field("dwMinorVersion", &self.dwMinorVersion)
            .field("dwBuildNumber", &self.dwBuildNumber)
            .field("dwPlatformId", &self.dwPlatformId)
            .field("szCSDVersion", &self.szCSDVersion)
            .field("wServicePackMajor", &self.wServicePackMajor)
            .field("wServicePackMinor", &self.wServicePackMinor)
            .field("wSuiteMask", &self.wSuiteMask)
            .field("wProductType", &self.wProductType)
            .field("wReserved", &self.wReserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for OSVERSIONINFOEXW {
    fn eq(&self, other: &Self) -> bool {
        self.dwOSVersionInfoSize == other.dwOSVersionInfoSize
            && self.dwMajorVersion == other.dwMajorVersion
            && self.dwMinorVersion == other.dwMinorVersion
            && self.dwBuildNumber == other.dwBuildNumber
            && self.dwPlatformId == other.dwPlatformId
            && self.szCSDVersion == other.szCSDVersion
            && self.wServicePackMajor == other.wServicePackMajor
            && self.wServicePackMinor == other.wServicePackMinor
            && self.wSuiteMask == other.wSuiteMask
            && self.wProductType == other.wProductType
            && self.wReserved == other.wReserved
    }
}
impl ::core::cmp::Eq for OSVERSIONINFOEXW {}
impl FromIntoMemory for OSVERSIONINFOEXW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 156);
        let f_dwOSVersionInfoSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwMajorVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwMinorVersion = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwBuildNumber = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwPlatformId = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_szCSDVersion = <[u16; 128] as FromIntoMemory>::from_bytes(&from[20..20 + 128]);
        let f_wServicePackMajor = <u16 as FromIntoMemory>::from_bytes(&from[148..148 + 2]);
        let f_wServicePackMinor = <u16 as FromIntoMemory>::from_bytes(&from[150..150 + 2]);
        let f_wSuiteMask = <u16 as FromIntoMemory>::from_bytes(&from[152..152 + 2]);
        let f_wProductType = <u8 as FromIntoMemory>::from_bytes(&from[154..154 + 1]);
        let f_wReserved = <u8 as FromIntoMemory>::from_bytes(&from[155..155 + 1]);
        Self {
            dwOSVersionInfoSize: f_dwOSVersionInfoSize,
            dwMajorVersion: f_dwMajorVersion,
            dwMinorVersion: f_dwMinorVersion,
            dwBuildNumber: f_dwBuildNumber,
            dwPlatformId: f_dwPlatformId,
            szCSDVersion: f_szCSDVersion,
            wServicePackMajor: f_wServicePackMajor,
            wServicePackMinor: f_wServicePackMinor,
            wSuiteMask: f_wSuiteMask,
            wProductType: f_wProductType,
            wReserved: f_wReserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 156);
        FromIntoMemory::into_bytes(self.dwOSVersionInfoSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwMajorVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwMinorVersion, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwBuildNumber, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwPlatformId, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.szCSDVersion, &mut into[20..20 + 128]);
        FromIntoMemory::into_bytes(self.wServicePackMajor, &mut into[148..148 + 2]);
        FromIntoMemory::into_bytes(self.wServicePackMinor, &mut into[150..150 + 2]);
        FromIntoMemory::into_bytes(self.wSuiteMask, &mut into[152..152 + 2]);
        FromIntoMemory::into_bytes(self.wProductType, &mut into[154..154 + 1]);
        FromIntoMemory::into_bytes(self.wReserved, &mut into[155..155 + 1]);
    }
    fn size() -> usize {
        156
    }
}
pub struct OSVERSIONINFOW {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [u16; 128],
}
impl ::core::marker::Copy for OSVERSIONINFOW {}
impl ::core::clone::Clone for OSVERSIONINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OSVERSIONINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OSVERSIONINFOW")
            .field("dwOSVersionInfoSize", &self.dwOSVersionInfoSize)
            .field("dwMajorVersion", &self.dwMajorVersion)
            .field("dwMinorVersion", &self.dwMinorVersion)
            .field("dwBuildNumber", &self.dwBuildNumber)
            .field("dwPlatformId", &self.dwPlatformId)
            .field("szCSDVersion", &self.szCSDVersion)
            .finish()
    }
}
impl ::core::cmp::PartialEq for OSVERSIONINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwOSVersionInfoSize == other.dwOSVersionInfoSize
            && self.dwMajorVersion == other.dwMajorVersion
            && self.dwMinorVersion == other.dwMinorVersion
            && self.dwBuildNumber == other.dwBuildNumber
            && self.dwPlatformId == other.dwPlatformId
            && self.szCSDVersion == other.szCSDVersion
    }
}
impl ::core::cmp::Eq for OSVERSIONINFOW {}
impl FromIntoMemory for OSVERSIONINFOW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 148);
        let f_dwOSVersionInfoSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwMajorVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwMinorVersion = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwBuildNumber = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwPlatformId = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_szCSDVersion = <[u16; 128] as FromIntoMemory>::from_bytes(&from[20..20 + 128]);
        Self {
            dwOSVersionInfoSize: f_dwOSVersionInfoSize,
            dwMajorVersion: f_dwMajorVersion,
            dwMinorVersion: f_dwMinorVersion,
            dwBuildNumber: f_dwBuildNumber,
            dwPlatformId: f_dwPlatformId,
            szCSDVersion: f_szCSDVersion,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 148);
        FromIntoMemory::into_bytes(self.dwOSVersionInfoSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwMajorVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwMinorVersion, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwBuildNumber, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwPlatformId, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.szCSDVersion, &mut into[20..20 + 128]);
    }
    fn size() -> usize {
        148
    }
}
pub const OSVERSION_MASK: u32 = 4294901760u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct OS_DEPLOYEMENT_STATE_VALUES(pub i32);
pub const OS_DEPLOYMENT_STANDARD: OS_DEPLOYEMENT_STATE_VALUES = OS_DEPLOYEMENT_STATE_VALUES(1i32);
pub const OS_DEPLOYMENT_COMPACT: OS_DEPLOYEMENT_STATE_VALUES = OS_DEPLOYEMENT_STATE_VALUES(2i32);
impl ::core::marker::Copy for OS_DEPLOYEMENT_STATE_VALUES {}
impl ::core::clone::Clone for OS_DEPLOYEMENT_STATE_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OS_DEPLOYEMENT_STATE_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OS_DEPLOYEMENT_STATE_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OS_DEPLOYEMENT_STATE_VALUES")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for OS_DEPLOYEMENT_STATE_VALUES {
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
pub struct OS_PRODUCT_TYPE(pub u32);
pub const PRODUCT_BUSINESS: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(6u32);
pub const PRODUCT_BUSINESS_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(16u32);
pub const PRODUCT_CLUSTER_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(18u32);
pub const PRODUCT_CLUSTER_SERVER_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(64u32);
pub const PRODUCT_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(101u32);
pub const PRODUCT_CORE_COUNTRYSPECIFIC: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(99u32);
pub const PRODUCT_CORE_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(98u32);
pub const PRODUCT_CORE_SINGLELANGUAGE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(100u32);
pub const PRODUCT_DATACENTER_EVALUATION_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(80u32);
pub const PRODUCT_DATACENTER_A_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(145u32);
pub const PRODUCT_STANDARD_A_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(146u32);
pub const PRODUCT_DATACENTER_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(8u32);
pub const PRODUCT_DATACENTER_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(12u32);
pub const PRODUCT_DATACENTER_SERVER_CORE_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(39u32);
pub const PRODUCT_DATACENTER_SERVER_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(37u32);
pub const PRODUCT_EDUCATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(121u32);
pub const PRODUCT_EDUCATION_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(122u32);
pub const PRODUCT_ENTERPRISE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(4u32);
pub const PRODUCT_ENTERPRISE_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(70u32);
pub const PRODUCT_ENTERPRISE_EVALUATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(72u32);
pub const PRODUCT_ENTERPRISE_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(27u32);
pub const PRODUCT_ENTERPRISE_N_EVALUATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(84u32);
pub const PRODUCT_ENTERPRISE_S: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(125u32);
pub const PRODUCT_ENTERPRISE_S_EVALUATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(129u32);
pub const PRODUCT_ENTERPRISE_S_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(126u32);
pub const PRODUCT_ENTERPRISE_S_N_EVALUATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(130u32);
pub const PRODUCT_ENTERPRISE_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(10u32);
pub const PRODUCT_ENTERPRISE_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(14u32);
pub const PRODUCT_ENTERPRISE_SERVER_CORE_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(41u32);
pub const PRODUCT_ENTERPRISE_SERVER_IA64: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(15u32);
pub const PRODUCT_ENTERPRISE_SERVER_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(38u32);
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_ADDL: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(60u32);
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_ADDLSVC: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(62u32);
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_MGMT: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(59u32);
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_MGMTSVC: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(61u32);
pub const PRODUCT_HOME_BASIC: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(2u32);
pub const PRODUCT_HOME_BASIC_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(67u32);
pub const PRODUCT_HOME_BASIC_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(5u32);
pub const PRODUCT_HOME_PREMIUM: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(3u32);
pub const PRODUCT_HOME_PREMIUM_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(68u32);
pub const PRODUCT_HOME_PREMIUM_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(26u32);
pub const PRODUCT_HOME_PREMIUM_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(34u32);
pub const PRODUCT_HOME_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(19u32);
pub const PRODUCT_HYPERV: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(42u32);
pub const PRODUCT_IOTUAP: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(123u32);
pub const PRODUCT_IOTUAPCOMMERCIAL: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(131u32);
pub const PRODUCT_MEDIUMBUSINESS_SERVER_MANAGEMENT: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(30u32);
pub const PRODUCT_MEDIUMBUSINESS_SERVER_MESSAGING: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(32u32);
pub const PRODUCT_MEDIUMBUSINESS_SERVER_SECURITY: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(31u32);
pub const PRODUCT_MOBILE_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(104u32);
pub const PRODUCT_MOBILE_ENTERPRISE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(133u32);
pub const PRODUCT_MULTIPOINT_PREMIUM_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(77u32);
pub const PRODUCT_MULTIPOINT_STANDARD_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(76u32);
pub const PRODUCT_PRO_WORKSTATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(161u32);
pub const PRODUCT_PRO_WORKSTATION_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(162u32);
pub const PRODUCT_PROFESSIONAL: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(48u32);
pub const PRODUCT_PROFESSIONAL_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(69u32);
pub const PRODUCT_PROFESSIONAL_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(49u32);
pub const PRODUCT_PROFESSIONAL_WMC: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(103u32);
pub const PRODUCT_SB_SOLUTION_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(50u32);
pub const PRODUCT_SB_SOLUTION_SERVER_EM: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(54u32);
pub const PRODUCT_SERVER_FOR_SB_SOLUTIONS: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(51u32);
pub const PRODUCT_SERVER_FOR_SB_SOLUTIONS_EM: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(55u32);
pub const PRODUCT_SERVER_FOR_SMALLBUSINESS: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(24u32);
pub const PRODUCT_SERVER_FOR_SMALLBUSINESS_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(35u32);
pub const PRODUCT_SERVER_FOUNDATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(33u32);
pub const PRODUCT_SMALLBUSINESS_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(9u32);
pub const PRODUCT_SMALLBUSINESS_SERVER_PREMIUM: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(25u32);
pub const PRODUCT_SMALLBUSINESS_SERVER_PREMIUM_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(63u32);
pub const PRODUCT_SOLUTION_EMBEDDEDSERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(56u32);
pub const PRODUCT_STANDARD_EVALUATION_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(79u32);
pub const PRODUCT_STANDARD_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(7u32);
pub const PRODUCT_STANDARD_SERVER_CORE_: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(13u32);
pub const PRODUCT_STANDARD_SERVER_CORE_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(40u32);
pub const PRODUCT_STANDARD_SERVER_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(36u32);
pub const PRODUCT_STANDARD_SERVER_SOLUTIONS: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(52u32);
pub const PRODUCT_STANDARD_SERVER_SOLUTIONS_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(53u32);
pub const PRODUCT_STARTER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(11u32);
pub const PRODUCT_STARTER_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(66u32);
pub const PRODUCT_STARTER_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(47u32);
pub const PRODUCT_STORAGE_ENTERPRISE_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(23u32);
pub const PRODUCT_STORAGE_ENTERPRISE_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(46u32);
pub const PRODUCT_STORAGE_EXPRESS_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(20u32);
pub const PRODUCT_STORAGE_EXPRESS_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(43u32);
pub const PRODUCT_STORAGE_STANDARD_EVALUATION_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(96u32);
pub const PRODUCT_STORAGE_STANDARD_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(21u32);
pub const PRODUCT_STORAGE_STANDARD_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(44u32);
pub const PRODUCT_STORAGE_WORKGROUP_EVALUATION_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(95u32);
pub const PRODUCT_STORAGE_WORKGROUP_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(22u32);
pub const PRODUCT_STORAGE_WORKGROUP_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(45u32);
pub const PRODUCT_ULTIMATE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(1u32);
pub const PRODUCT_ULTIMATE_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(71u32);
pub const PRODUCT_ULTIMATE_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(28u32);
pub const PRODUCT_UNDEFINED: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(0u32);
pub const PRODUCT_WEB_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(17u32);
pub const PRODUCT_WEB_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(29u32);
impl ::core::marker::Copy for OS_PRODUCT_TYPE {}
impl ::core::clone::Clone for OS_PRODUCT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OS_PRODUCT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OS_PRODUCT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OS_PRODUCT_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for OS_PRODUCT_TYPE {
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
pub type PGET_SYSTEM_WOW64_DIRECTORY_A = StdCallFnPtr<(PSTR, u32), u32>;
pub type PGET_SYSTEM_WOW64_DIRECTORY_W = StdCallFnPtr<(PWSTR, u32), u32>;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROCESSOR_CACHE_TYPE(pub i32);
pub const CacheUnified: PROCESSOR_CACHE_TYPE = PROCESSOR_CACHE_TYPE(0i32);
pub const CacheInstruction: PROCESSOR_CACHE_TYPE = PROCESSOR_CACHE_TYPE(1i32);
pub const CacheData: PROCESSOR_CACHE_TYPE = PROCESSOR_CACHE_TYPE(2i32);
pub const CacheTrace: PROCESSOR_CACHE_TYPE = PROCESSOR_CACHE_TYPE(3i32);
impl ::core::marker::Copy for PROCESSOR_CACHE_TYPE {}
impl ::core::clone::Clone for PROCESSOR_CACHE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESSOR_CACHE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROCESSOR_CACHE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESSOR_CACHE_TYPE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for PROCESSOR_CACHE_TYPE {
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
pub struct PROCESSOR_GROUP_INFO {
    pub MaximumProcessorCount: u8,
    pub ActiveProcessorCount: u8,
    pub Reserved: [u8; 38],
    pub ActiveProcessorMask: PtrRepr,
}
impl ::core::marker::Copy for PROCESSOR_GROUP_INFO {}
impl ::core::clone::Clone for PROCESSOR_GROUP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESSOR_GROUP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESSOR_GROUP_INFO")
            .field("MaximumProcessorCount", &self.MaximumProcessorCount)
            .field("ActiveProcessorCount", &self.ActiveProcessorCount)
            .field("Reserved", &self.Reserved)
            .field("ActiveProcessorMask", &self.ActiveProcessorMask)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PROCESSOR_GROUP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.MaximumProcessorCount == other.MaximumProcessorCount
            && self.ActiveProcessorCount == other.ActiveProcessorCount
            && self.Reserved == other.Reserved
            && self.ActiveProcessorMask == other.ActiveProcessorMask
    }
}
impl ::core::cmp::Eq for PROCESSOR_GROUP_INFO {}
impl FromIntoMemory for PROCESSOR_GROUP_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 44);
        let f_MaximumProcessorCount = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_ActiveProcessorCount = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_Reserved = <[u8; 38] as FromIntoMemory>::from_bytes(&from[2..2 + 38]);
        let f_ActiveProcessorMask = <PtrRepr as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        Self {
            MaximumProcessorCount: f_MaximumProcessorCount,
            ActiveProcessorCount: f_ActiveProcessorCount,
            Reserved: f_Reserved,
            ActiveProcessorMask: f_ActiveProcessorMask,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 44);
        FromIntoMemory::into_bytes(self.MaximumProcessorCount, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.ActiveProcessorCount, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[2..2 + 38]);
        FromIntoMemory::into_bytes(self.ActiveProcessorMask, &mut into[40..40 + 4]);
    }
    fn size() -> usize {
        44
    }
}
pub struct PROCESSOR_RELATIONSHIP {
    pub Flags: u8,
    pub EfficiencyClass: u8,
    pub Reserved: [u8; 20],
    pub GroupCount: u16,
    pub GroupMask: [GROUP_AFFINITY; 1],
}
impl ::core::marker::Copy for PROCESSOR_RELATIONSHIP {}
impl ::core::clone::Clone for PROCESSOR_RELATIONSHIP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESSOR_RELATIONSHIP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESSOR_RELATIONSHIP")
            .field("Flags", &self.Flags)
            .field("EfficiencyClass", &self.EfficiencyClass)
            .field("Reserved", &self.Reserved)
            .field("GroupCount", &self.GroupCount)
            .field("GroupMask", &self.GroupMask)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PROCESSOR_RELATIONSHIP {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
            && self.EfficiencyClass == other.EfficiencyClass
            && self.Reserved == other.Reserved
            && self.GroupCount == other.GroupCount
            && self.GroupMask == other.GroupMask
    }
}
impl ::core::cmp::Eq for PROCESSOR_RELATIONSHIP {}
impl FromIntoMemory for PROCESSOR_RELATIONSHIP {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 36);
        let f_Flags = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_EfficiencyClass = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_Reserved = <[u8; 20] as FromIntoMemory>::from_bytes(&from[2..2 + 20]);
        let f_GroupCount = <u16 as FromIntoMemory>::from_bytes(&from[22..22 + 2]);
        let f_GroupMask = <[GROUP_AFFINITY; 1] as FromIntoMemory>::from_bytes(&from[24..24 + 12]);
        Self {
            Flags: f_Flags,
            EfficiencyClass: f_EfficiencyClass,
            Reserved: f_Reserved,
            GroupCount: f_GroupCount,
            GroupMask: f_GroupMask,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 36);
        FromIntoMemory::into_bytes(self.Flags, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.EfficiencyClass, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[2..2 + 20]);
        FromIntoMemory::into_bytes(self.GroupCount, &mut into[22..22 + 2]);
        FromIntoMemory::into_bytes(self.GroupMask, &mut into[24..24 + 12]);
    }
    fn size() -> usize {
        36
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTL_SYSTEM_GLOBAL_DATA_ID(pub i32);
pub const GlobalDataIdUnknown: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(0i32);
pub const GlobalDataIdRngSeedVersion: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(1i32);
pub const GlobalDataIdInterruptTime: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(2i32);
pub const GlobalDataIdTimeZoneBias: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(3i32);
pub const GlobalDataIdImageNumberLow: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(4i32);
pub const GlobalDataIdImageNumberHigh: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(5i32);
pub const GlobalDataIdTimeZoneId: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(6i32);
pub const GlobalDataIdNtMajorVersion: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(7i32);
pub const GlobalDataIdNtMinorVersion: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(8i32);
pub const GlobalDataIdSystemExpirationDate: RTL_SYSTEM_GLOBAL_DATA_ID =
    RTL_SYSTEM_GLOBAL_DATA_ID(9i32);
pub const GlobalDataIdKdDebuggerEnabled: RTL_SYSTEM_GLOBAL_DATA_ID =
    RTL_SYSTEM_GLOBAL_DATA_ID(10i32);
pub const GlobalDataIdCyclesPerYield: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(11i32);
pub const GlobalDataIdSafeBootMode: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(12i32);
pub const GlobalDataIdLastSystemRITEventTickCount: RTL_SYSTEM_GLOBAL_DATA_ID =
    RTL_SYSTEM_GLOBAL_DATA_ID(13i32);
impl ::core::marker::Copy for RTL_SYSTEM_GLOBAL_DATA_ID {}
impl ::core::clone::Clone for RTL_SYSTEM_GLOBAL_DATA_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTL_SYSTEM_GLOBAL_DATA_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTL_SYSTEM_GLOBAL_DATA_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTL_SYSTEM_GLOBAL_DATA_ID")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for RTL_SYSTEM_GLOBAL_DATA_ID {
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
pub const SCEX2_ALT_NETBIOS_NAME: u32 = 1u32;
pub const SPVERSION_MASK: u32 = 65280u32;
pub const SUBVERSION_MASK: u32 = 255u32;
pub struct SYSTEM_CPU_SET_INFORMATION {
    pub Size: u32,
    pub Type: CPU_SET_INFORMATION_TYPE,
    pub Anonymous: SYSTEM_CPU_SET_INFORMATION_0,
}
impl ::core::marker::Copy for SYSTEM_CPU_SET_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_CPU_SET_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_CPU_SET_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_CPU_SET_INFORMATION")
            .field("Size", &self.Size)
            .field("Type", &self.Type)
            .field("Anonymous", &self.Anonymous)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_CPU_SET_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Type == other.Type && self.Anonymous == other.Anonymous
    }
}
impl ::core::cmp::Eq for SYSTEM_CPU_SET_INFORMATION {}
impl FromIntoMemory for SYSTEM_CPU_SET_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_Size = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Type = <CPU_SET_INFORMATION_TYPE as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Anonymous =
            <SYSTEM_CPU_SET_INFORMATION_0 as FromIntoMemory>::from_bytes(&from[8..8 + 24]);
        Self {
            Size: f_Size,
            Type: f_Type,
            Anonymous: f_Anonymous,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.Size, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Type, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[8..8 + 24]);
    }
    fn size() -> usize {
        32
    }
}
pub struct SYSTEM_CPU_SET_INFORMATION_0 {
    data: [u8; 24],
}
impl ::core::default::Default for SYSTEM_CPU_SET_INFORMATION_0 {
    fn default() -> Self {
        Self { data: [0u8; 24] }
    }
}
impl ::core::marker::Copy for SYSTEM_CPU_SET_INFORMATION_0 {}
impl ::core::clone::Clone for SYSTEM_CPU_SET_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_CPU_SET_INFORMATION_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_CPU_SET_INFORMATION_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_CPU_SET_INFORMATION_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for SYSTEM_CPU_SET_INFORMATION_0 {}
impl FromIntoMemory for SYSTEM_CPU_SET_INFORMATION_0 {
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
pub struct SYSTEM_CPU_SET_INFORMATION_0_0 {
    pub Id: u32,
    pub Group: u16,
    pub LogicalProcessorIndex: u8,
    pub CoreIndex: u8,
    pub LastLevelCacheIndex: u8,
    pub NumaNodeIndex: u8,
    pub EfficiencyClass: u8,
    pub Anonymous1: SYSTEM_CPU_SET_INFORMATION_0_0_0,
    pub Anonymous2: SYSTEM_CPU_SET_INFORMATION_0_0_1,
    pub AllocationTag: u64,
}
impl ::core::marker::Copy for SYSTEM_CPU_SET_INFORMATION_0_0 {}
impl ::core::clone::Clone for SYSTEM_CPU_SET_INFORMATION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_CPU_SET_INFORMATION_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_CPU_SET_INFORMATION_0_0")
            .field("Id", &self.Id)
            .field("Group", &self.Group)
            .field("LogicalProcessorIndex", &self.LogicalProcessorIndex)
            .field("CoreIndex", &self.CoreIndex)
            .field("LastLevelCacheIndex", &self.LastLevelCacheIndex)
            .field("NumaNodeIndex", &self.NumaNodeIndex)
            .field("EfficiencyClass", &self.EfficiencyClass)
            .field("Anonymous1", &self.Anonymous1)
            .field("Anonymous2", &self.Anonymous2)
            .field("AllocationTag", &self.AllocationTag)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_CPU_SET_INFORMATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Id == other.Id
            && self.Group == other.Group
            && self.LogicalProcessorIndex == other.LogicalProcessorIndex
            && self.CoreIndex == other.CoreIndex
            && self.LastLevelCacheIndex == other.LastLevelCacheIndex
            && self.NumaNodeIndex == other.NumaNodeIndex
            && self.EfficiencyClass == other.EfficiencyClass
            && self.Anonymous1 == other.Anonymous1
            && self.Anonymous2 == other.Anonymous2
            && self.AllocationTag == other.AllocationTag
    }
}
impl ::core::cmp::Eq for SYSTEM_CPU_SET_INFORMATION_0_0 {}
impl FromIntoMemory for SYSTEM_CPU_SET_INFORMATION_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_Id = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Group = <u16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_LogicalProcessorIndex = <u8 as FromIntoMemory>::from_bytes(&from[6..6 + 1]);
        let f_CoreIndex = <u8 as FromIntoMemory>::from_bytes(&from[7..7 + 1]);
        let f_LastLevelCacheIndex = <u8 as FromIntoMemory>::from_bytes(&from[8..8 + 1]);
        let f_NumaNodeIndex = <u8 as FromIntoMemory>::from_bytes(&from[9..9 + 1]);
        let f_EfficiencyClass = <u8 as FromIntoMemory>::from_bytes(&from[10..10 + 1]);
        let f_Anonymous1 =
            <SYSTEM_CPU_SET_INFORMATION_0_0_0 as FromIntoMemory>::from_bytes(&from[11..11 + 1]);
        let f_Anonymous2 =
            <SYSTEM_CPU_SET_INFORMATION_0_0_1 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_AllocationTag = <u64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        Self {
            Id: f_Id,
            Group: f_Group,
            LogicalProcessorIndex: f_LogicalProcessorIndex,
            CoreIndex: f_CoreIndex,
            LastLevelCacheIndex: f_LastLevelCacheIndex,
            NumaNodeIndex: f_NumaNodeIndex,
            EfficiencyClass: f_EfficiencyClass,
            Anonymous1: f_Anonymous1,
            Anonymous2: f_Anonymous2,
            AllocationTag: f_AllocationTag,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.Id, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Group, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.LogicalProcessorIndex, &mut into[6..6 + 1]);
        FromIntoMemory::into_bytes(self.CoreIndex, &mut into[7..7 + 1]);
        FromIntoMemory::into_bytes(self.LastLevelCacheIndex, &mut into[8..8 + 1]);
        FromIntoMemory::into_bytes(self.NumaNodeIndex, &mut into[9..9 + 1]);
        FromIntoMemory::into_bytes(self.EfficiencyClass, &mut into[10..10 + 1]);
        FromIntoMemory::into_bytes(self.Anonymous1, &mut into[11..11 + 1]);
        FromIntoMemory::into_bytes(self.Anonymous2, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.AllocationTag, &mut into[16..16 + 8]);
    }
    fn size() -> usize {
        24
    }
}
pub struct SYSTEM_CPU_SET_INFORMATION_0_0_0 {
    data: [u8; 1],
}
impl ::core::default::Default for SYSTEM_CPU_SET_INFORMATION_0_0_0 {
    fn default() -> Self {
        Self { data: [0u8; 1] }
    }
}
impl ::core::marker::Copy for SYSTEM_CPU_SET_INFORMATION_0_0_0 {}
impl ::core::clone::Clone for SYSTEM_CPU_SET_INFORMATION_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_CPU_SET_INFORMATION_0_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_CPU_SET_INFORMATION_0_0_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_CPU_SET_INFORMATION_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for SYSTEM_CPU_SET_INFORMATION_0_0_0 {}
impl FromIntoMemory for SYSTEM_CPU_SET_INFORMATION_0_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 1];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        1
    }
}
pub struct SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {}
impl ::core::clone::Clone for SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_CPU_SET_INFORMATION_0_0_0_0")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {}
impl FromIntoMemory for SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 1);
        let f__bitfield = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        Self {
            _bitfield: f__bitfield,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 1);
        FromIntoMemory::into_bytes(self._bitfield, &mut into[0..0 + 1]);
    }
    fn size() -> usize {
        1
    }
}
pub struct SYSTEM_CPU_SET_INFORMATION_0_0_1 {
    data: [u8; 4],
}
impl ::core::default::Default for SYSTEM_CPU_SET_INFORMATION_0_0_1 {
    fn default() -> Self {
        Self { data: [0u8; 4] }
    }
}
impl ::core::marker::Copy for SYSTEM_CPU_SET_INFORMATION_0_0_1 {}
impl ::core::clone::Clone for SYSTEM_CPU_SET_INFORMATION_0_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_CPU_SET_INFORMATION_0_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_CPU_SET_INFORMATION_0_0_1")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_CPU_SET_INFORMATION_0_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for SYSTEM_CPU_SET_INFORMATION_0_0_1 {}
impl FromIntoMemory for SYSTEM_CPU_SET_INFORMATION_0_0_1 {
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
pub const SYSTEM_CPU_SET_INFORMATION_ALLOCATED: u32 = 2u32;
pub const SYSTEM_CPU_SET_INFORMATION_ALLOCATED_TO_TARGET_PROCESS: u32 = 4u32;
pub const SYSTEM_CPU_SET_INFORMATION_PARKED: u32 = 1u32;
pub const SYSTEM_CPU_SET_INFORMATION_REALTIME: u32 = 8u32;
pub struct SYSTEM_INFO {
    pub Anonymous: SYSTEM_INFO_0,
    pub dwPageSize: u32,
    pub lpMinimumApplicationAddress: MutPtr<::core::ffi::c_void>,
    pub lpMaximumApplicationAddress: MutPtr<::core::ffi::c_void>,
    pub dwActiveProcessorMask: PtrRepr,
    pub dwNumberOfProcessors: u32,
    pub dwProcessorType: u32,
    pub dwAllocationGranularity: u32,
    pub wProcessorLevel: u16,
    pub wProcessorRevision: u16,
}
impl ::core::marker::Copy for SYSTEM_INFO {}
impl ::core::clone::Clone for SYSTEM_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_INFO")
            .field("Anonymous", &self.Anonymous)
            .field("dwPageSize", &self.dwPageSize)
            .field(
                "lpMinimumApplicationAddress",
                &self.lpMinimumApplicationAddress,
            )
            .field(
                "lpMaximumApplicationAddress",
                &self.lpMaximumApplicationAddress,
            )
            .field("dwActiveProcessorMask", &self.dwActiveProcessorMask)
            .field("dwNumberOfProcessors", &self.dwNumberOfProcessors)
            .field("dwProcessorType", &self.dwProcessorType)
            .field("dwAllocationGranularity", &self.dwAllocationGranularity)
            .field("wProcessorLevel", &self.wProcessorLevel)
            .field("wProcessorRevision", &self.wProcessorRevision)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Anonymous == other.Anonymous
            && self.dwPageSize == other.dwPageSize
            && self.lpMinimumApplicationAddress == other.lpMinimumApplicationAddress
            && self.lpMaximumApplicationAddress == other.lpMaximumApplicationAddress
            && self.dwActiveProcessorMask == other.dwActiveProcessorMask
            && self.dwNumberOfProcessors == other.dwNumberOfProcessors
            && self.dwProcessorType == other.dwProcessorType
            && self.dwAllocationGranularity == other.dwAllocationGranularity
            && self.wProcessorLevel == other.wProcessorLevel
            && self.wProcessorRevision == other.wProcessorRevision
    }
}
impl ::core::cmp::Eq for SYSTEM_INFO {}
impl FromIntoMemory for SYSTEM_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 36);
        let f_Anonymous = <SYSTEM_INFO_0 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwPageSize = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpMinimumApplicationAddress =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpMaximumApplicationAddress =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwActiveProcessorMask = <PtrRepr as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwNumberOfProcessors = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwProcessorType = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_dwAllocationGranularity = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_wProcessorLevel = <u16 as FromIntoMemory>::from_bytes(&from[32..32 + 2]);
        let f_wProcessorRevision = <u16 as FromIntoMemory>::from_bytes(&from[34..34 + 2]);
        Self {
            Anonymous: f_Anonymous,
            dwPageSize: f_dwPageSize,
            lpMinimumApplicationAddress: f_lpMinimumApplicationAddress,
            lpMaximumApplicationAddress: f_lpMaximumApplicationAddress,
            dwActiveProcessorMask: f_dwActiveProcessorMask,
            dwNumberOfProcessors: f_dwNumberOfProcessors,
            dwProcessorType: f_dwProcessorType,
            dwAllocationGranularity: f_dwAllocationGranularity,
            wProcessorLevel: f_wProcessorLevel,
            wProcessorRevision: f_wProcessorRevision,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 36);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwPageSize, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpMinimumApplicationAddress, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpMaximumApplicationAddress, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwActiveProcessorMask, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfProcessors, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwProcessorType, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.dwAllocationGranularity, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.wProcessorLevel, &mut into[32..32 + 2]);
        FromIntoMemory::into_bytes(self.wProcessorRevision, &mut into[34..34 + 2]);
    }
    fn size() -> usize {
        36
    }
}
pub struct SYSTEM_INFO_0 {
    data: [u8; 4],
}
impl ::core::default::Default for SYSTEM_INFO_0 {
    fn default() -> Self {
        Self { data: [0u8; 4] }
    }
}
impl ::core::marker::Copy for SYSTEM_INFO_0 {}
impl ::core::clone::Clone for SYSTEM_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_INFO_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for SYSTEM_INFO_0 {}
impl FromIntoMemory for SYSTEM_INFO_0 {
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
pub struct SYSTEM_INFO_0_0 {
    pub wProcessorArchitecture: super::Diagnostics::Debug::PROCESSOR_ARCHITECTURE,
    pub wReserved: u16,
}
impl ::core::marker::Copy for SYSTEM_INFO_0_0 {}
impl ::core::clone::Clone for SYSTEM_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_INFO_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_INFO_0_0")
            .field("wProcessorArchitecture", &self.wProcessorArchitecture)
            .field("wReserved", &self.wReserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.wProcessorArchitecture == other.wProcessorArchitecture
            && self.wReserved == other.wReserved
    }
}
impl ::core::cmp::Eq for SYSTEM_INFO_0_0 {}
impl FromIntoMemory for SYSTEM_INFO_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_wProcessorArchitecture =
            <super::Diagnostics::Debug::PROCESSOR_ARCHITECTURE as FromIntoMemory>::from_bytes(
                &from[0..0 + 2],
            );
        let f_wReserved = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        Self {
            wProcessorArchitecture: f_wProcessorArchitecture,
            wReserved: f_wReserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.wProcessorArchitecture, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wReserved, &mut into[2..2 + 2]);
    }
    fn size() -> usize {
        4
    }
}
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION {
    pub ProcessorMask: PtrRepr,
    pub Relationship: LOGICAL_PROCESSOR_RELATIONSHIP,
    pub Anonymous: SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0,
}
impl ::core::marker::Copy for SYSTEM_LOGICAL_PROCESSOR_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_LOGICAL_PROCESSOR_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_LOGICAL_PROCESSOR_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_LOGICAL_PROCESSOR_INFORMATION")
            .field("ProcessorMask", &self.ProcessorMask)
            .field("Relationship", &self.Relationship)
            .field("Anonymous", &self.Anonymous)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ProcessorMask == other.ProcessorMask
            && self.Relationship == other.Relationship
            && self.Anonymous == other.Anonymous
    }
}
impl ::core::cmp::Eq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION {}
impl FromIntoMemory for SYSTEM_LOGICAL_PROCESSOR_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_ProcessorMask = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Relationship =
            <LOGICAL_PROCESSOR_RELATIONSHIP as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Anonymous = <SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 as FromIntoMemory>::from_bytes(
            &from[8..8 + 16],
        );
        Self {
            ProcessorMask: f_ProcessorMask,
            Relationship: f_Relationship,
            Anonymous: f_Anonymous,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.ProcessorMask, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Relationship, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[8..8 + 16]);
    }
    fn size() -> usize {
        24
    }
}
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {
    data: [u8; 16],
}
impl ::core::default::Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {
    fn default() -> Self {
        Self { data: [0u8; 16] }
    }
}
impl ::core::marker::Copy for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {}
impl ::core::clone::Clone for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {}
impl FromIntoMemory for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {
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
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {
    pub NodeNumber: u32,
}
impl ::core::marker::Copy for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {}
impl ::core::clone::Clone for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0")
            .field("NodeNumber", &self.NodeNumber)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.NodeNumber == other.NodeNumber
    }
}
impl ::core::cmp::Eq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {}
impl FromIntoMemory for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_NodeNumber = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            NodeNumber: f_NodeNumber,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.NodeNumber, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {
    pub Flags: u8,
}
impl ::core::marker::Copy for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {}
impl ::core::clone::Clone for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1")
            .field("Flags", &self.Flags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {}
impl FromIntoMemory for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 1);
        let f_Flags = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        Self { Flags: f_Flags }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 1);
        FromIntoMemory::into_bytes(self.Flags, &mut into[0..0 + 1]);
    }
    fn size() -> usize {
        1
    }
}
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {
    pub Relationship: LOGICAL_PROCESSOR_RELATIONSHIP,
    pub Size: u32,
    pub Anonymous: SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0,
}
impl ::core::marker::Copy for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {}
impl ::core::clone::Clone for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX")
            .field("Relationship", &self.Relationship)
            .field("Size", &self.Size)
            .field("Anonymous", &self.Anonymous)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Relationship == other.Relationship
            && self.Size == other.Size
            && self.Anonymous == other.Anonymous
    }
}
impl ::core::cmp::Eq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {}
impl FromIntoMemory for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 76);
        let f_Relationship =
            <LOGICAL_PROCESSOR_RELATIONSHIP as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Size = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Anonymous = <SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 as FromIntoMemory>::from_bytes(
            &from[8..8 + 68],
        );
        Self {
            Relationship: f_Relationship,
            Size: f_Size,
            Anonymous: f_Anonymous,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 76);
        FromIntoMemory::into_bytes(self.Relationship, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Size, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[8..8 + 68]);
    }
    fn size() -> usize {
        76
    }
}
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {
    data: [u8; 68],
}
impl ::core::default::Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {
    fn default() -> Self {
        Self { data: [0u8; 68] }
    }
}
impl ::core::marker::Copy for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {}
impl ::core::clone::Clone for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {}
impl FromIntoMemory for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 68];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        68
    }
}
pub struct SYSTEM_POOL_ZEROING_INFORMATION {
    pub PoolZeroingSupportPresent: super::super::Foundation::BOOLEAN,
}
impl ::core::marker::Copy for SYSTEM_POOL_ZEROING_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_POOL_ZEROING_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_POOL_ZEROING_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_POOL_ZEROING_INFORMATION")
            .field("PoolZeroingSupportPresent", &self.PoolZeroingSupportPresent)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_POOL_ZEROING_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.PoolZeroingSupportPresent == other.PoolZeroingSupportPresent
    }
}
impl ::core::cmp::Eq for SYSTEM_POOL_ZEROING_INFORMATION {}
impl FromIntoMemory for SYSTEM_POOL_ZEROING_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 1);
        let f_PoolZeroingSupportPresent =
            <super::super::Foundation::BOOLEAN as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        Self {
            PoolZeroingSupportPresent: f_PoolZeroingSupportPresent,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 1);
        FromIntoMemory::into_bytes(self.PoolZeroingSupportPresent, &mut into[0..0 + 1]);
    }
    fn size() -> usize {
        1
    }
}
pub struct SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    pub CycleTime: u64,
}
impl ::core::marker::Copy for SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION")
            .field("CycleTime", &self.CycleTime)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.CycleTime == other.CycleTime
    }
}
impl ::core::cmp::Eq for SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {}
impl FromIntoMemory for SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_CycleTime = <u64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        Self {
            CycleTime: f_CycleTime,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.CycleTime, &mut into[0..0 + 8]);
    }
    fn size() -> usize {
        8
    }
}
pub struct SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {}
impl FromIntoMemory for SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct USER_CET_ENVIRONMENT(pub u32);
pub const USER_CET_ENVIRONMENT_WIN32_PROCESS: USER_CET_ENVIRONMENT = USER_CET_ENVIRONMENT(0u32);
pub const USER_CET_ENVIRONMENT_SGX2_ENCLAVE: USER_CET_ENVIRONMENT = USER_CET_ENVIRONMENT(2u32);
pub const USER_CET_ENVIRONMENT_VBS_ENCLAVE: USER_CET_ENVIRONMENT = USER_CET_ENVIRONMENT(16u32);
pub const USER_CET_ENVIRONMENT_VBS_BASIC_ENCLAVE: USER_CET_ENVIRONMENT =
    USER_CET_ENVIRONMENT(17u32);
impl ::core::marker::Copy for USER_CET_ENVIRONMENT {}
impl ::core::clone::Clone for USER_CET_ENVIRONMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for USER_CET_ENVIRONMENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USER_CET_ENVIRONMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USER_CET_ENVIRONMENT")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for USER_CET_ENVIRONMENT {
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
pub struct VER_FLAGS(pub u32);
pub const VER_MINORVERSION: VER_FLAGS = VER_FLAGS(1u32);
pub const VER_MAJORVERSION: VER_FLAGS = VER_FLAGS(2u32);
pub const VER_BUILDNUMBER: VER_FLAGS = VER_FLAGS(4u32);
pub const VER_PLATFORMID: VER_FLAGS = VER_FLAGS(8u32);
pub const VER_SERVICEPACKMINOR: VER_FLAGS = VER_FLAGS(16u32);
pub const VER_SERVICEPACKMAJOR: VER_FLAGS = VER_FLAGS(32u32);
pub const VER_SUITENAME: VER_FLAGS = VER_FLAGS(64u32);
pub const VER_PRODUCT_TYPE: VER_FLAGS = VER_FLAGS(128u32);
impl ::core::marker::Copy for VER_FLAGS {}
impl ::core::clone::Clone for VER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for VER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for VER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for VER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for VER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for VER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for VER_FLAGS {
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
pub const WDK_NTDDI_VERSION: u32 = 167772171u32;
pub const _WIN32_IE_IE100: u32 = 2560u32;
pub const _WIN32_IE_IE110: u32 = 2560u32;
pub const _WIN32_IE_IE20: u32 = 512u32;
pub const _WIN32_IE_IE30: u32 = 768u32;
pub const _WIN32_IE_IE302: u32 = 770u32;
pub const _WIN32_IE_IE40: u32 = 1024u32;
pub const _WIN32_IE_IE401: u32 = 1025u32;
pub const _WIN32_IE_IE50: u32 = 1280u32;
pub const _WIN32_IE_IE501: u32 = 1281u32;
pub const _WIN32_IE_IE55: u32 = 1360u32;
pub const _WIN32_IE_IE60: u32 = 1536u32;
pub const _WIN32_IE_IE60SP1: u32 = 1537u32;
pub const _WIN32_IE_IE60SP2: u32 = 1539u32;
pub const _WIN32_IE_IE70: u32 = 1792u32;
pub const _WIN32_IE_IE80: u32 = 2048u32;
pub const _WIN32_IE_IE90: u32 = 2304u32;
pub const _WIN32_IE_LONGHORN: u32 = 1792u32;
pub const _WIN32_IE_NT4: u32 = 512u32;
pub const _WIN32_IE_NT4SP1: u32 = 512u32;
pub const _WIN32_IE_NT4SP2: u32 = 512u32;
pub const _WIN32_IE_NT4SP3: u32 = 770u32;
pub const _WIN32_IE_NT4SP4: u32 = 1025u32;
pub const _WIN32_IE_NT4SP5: u32 = 1025u32;
pub const _WIN32_IE_NT4SP6: u32 = 1280u32;
pub const _WIN32_IE_WIN10: u32 = 2560u32;
pub const _WIN32_IE_WIN2K: u32 = 1281u32;
pub const _WIN32_IE_WIN2KSP1: u32 = 1281u32;
pub const _WIN32_IE_WIN2KSP2: u32 = 1281u32;
pub const _WIN32_IE_WIN2KSP3: u32 = 1281u32;
pub const _WIN32_IE_WIN2KSP4: u32 = 1281u32;
pub const _WIN32_IE_WIN6: u32 = 1792u32;
pub const _WIN32_IE_WIN7: u32 = 2048u32;
pub const _WIN32_IE_WIN8: u32 = 2560u32;
pub const _WIN32_IE_WIN98: u32 = 1025u32;
pub const _WIN32_IE_WIN98SE: u32 = 1280u32;
pub const _WIN32_IE_WINBLUE: u32 = 2560u32;
pub const _WIN32_IE_WINME: u32 = 1360u32;
pub const _WIN32_IE_WINTHRESHOLD: u32 = 2560u32;
pub const _WIN32_IE_WS03: u32 = 1538u32;
pub const _WIN32_IE_WS03SP1: u32 = 1539u32;
pub const _WIN32_IE_XP: u32 = 1536u32;
pub const _WIN32_IE_XPSP1: u32 = 1537u32;
pub const _WIN32_IE_XPSP2: u32 = 1539u32;
pub const _WIN32_WINNT_LONGHORN: u32 = 1536u32;
pub const _WIN32_WINNT_NT4: u32 = 1024u32;
pub const _WIN32_WINNT_VISTA: u32 = 1536u32;
pub const _WIN32_WINNT_WIN10: u32 = 2560u32;
pub const _WIN32_WINNT_WIN2K: u32 = 1280u32;
pub const _WIN32_WINNT_WIN6: u32 = 1536u32;
pub const _WIN32_WINNT_WIN7: u32 = 1537u32;
pub const _WIN32_WINNT_WIN8: u32 = 1538u32;
pub const _WIN32_WINNT_WINBLUE: u32 = 1539u32;
pub const _WIN32_WINNT_WINTHRESHOLD: u32 = 2560u32;
pub const _WIN32_WINNT_WINXP: u32 = 1281u32;
pub const _WIN32_WINNT_WS03: u32 = 1282u32;
pub const _WIN32_WINNT_WS08: u32 = 1536u32;
pub trait Api {
    #[doc = "GetComputerNameExA from KERNEL32"]
    fn GetComputerNameExA(
        &self,
        name_type: COMPUTER_NAME_FORMAT,
        lp_buffer: PSTR,
        n_size: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetComputerNameExA")
    }
    #[doc = "GetComputerNameExW from KERNEL32"]
    fn GetComputerNameExW(
        &self,
        name_type: COMPUTER_NAME_FORMAT,
        lp_buffer: PWSTR,
        n_size: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetComputerNameExW")
    }
    #[doc = "GetLocalTime from KERNEL32"]
    fn GetLocalTime(&self, lp_system_time: MutPtr<super::super::Foundation::SYSTEMTIME>) {
        todo!("GetLocalTime")
    }
    #[doc = "GetNativeSystemInfo from KERNEL32"]
    fn GetNativeSystemInfo(&self, lp_system_info: MutPtr<SYSTEM_INFO>) {
        todo!("GetNativeSystemInfo")
    }
    #[doc = "GetSystemCpuSetInformation from KERNEL32"]
    fn GetSystemCpuSetInformation(
        &self,
        information: MutPtr<SYSTEM_CPU_SET_INFORMATION>,
        buffer_length: u32,
        returned_length: MutPtr<u32>,
        process: super::super::Foundation::HANDLE,
        flags: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("GetSystemCpuSetInformation")
    }
    #[doc = "GetSystemDirectoryA from KERNEL32"]
    fn GetSystemDirectoryA(&self, lp_buffer: PSTR, u_size: u32) -> u32 {
        todo!("GetSystemDirectoryA")
    }
    #[doc = "GetSystemDirectoryW from KERNEL32"]
    fn GetSystemDirectoryW(&self, lp_buffer: PWSTR, u_size: u32) -> u32 {
        todo!("GetSystemDirectoryW")
    }
    #[doc = "GetSystemInfo from KERNEL32"]
    fn GetSystemInfo(&self, lp_system_info: MutPtr<SYSTEM_INFO>) {
        todo!("GetSystemInfo")
    }
    #[doc = "GetSystemLeapSecondInformation from KERNEL32"]
    fn GetSystemLeapSecondInformation(
        &self,
        enabled: MutPtr<super::super::Foundation::BOOL>,
        flags: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetSystemLeapSecondInformation")
    }
    #[doc = "GetSystemTime from KERNEL32"]
    fn GetSystemTime(&self, lp_system_time: MutPtr<super::super::Foundation::SYSTEMTIME>) {
        todo!("GetSystemTime")
    }
    #[doc = "GetSystemTimeAdjustment from KERNEL32"]
    fn GetSystemTimeAdjustment(
        &self,
        lp_time_adjustment: MutPtr<u32>,
        lp_time_increment: MutPtr<u32>,
        lp_time_adjustment_disabled: MutPtr<super::super::Foundation::BOOL>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetSystemTimeAdjustment")
    }
    #[doc = "GetSystemTimeAsFileTime from KERNEL32"]
    fn GetSystemTimeAsFileTime(
        &self,
        lp_system_time_as_file_time: MutPtr<super::super::Foundation::FILETIME>,
    ) {
        todo!("GetSystemTimeAsFileTime")
    }
    #[doc = "GetSystemWindowsDirectoryA from KERNEL32"]
    fn GetSystemWindowsDirectoryA(&self, lp_buffer: PSTR, u_size: u32) -> u32 {
        todo!("GetSystemWindowsDirectoryA")
    }
    #[doc = "GetSystemWindowsDirectoryW from KERNEL32"]
    fn GetSystemWindowsDirectoryW(&self, lp_buffer: PWSTR, u_size: u32) -> u32 {
        todo!("GetSystemWindowsDirectoryW")
    }
    #[doc = "GetSystemWow64DirectoryA from KERNEL32"]
    fn GetSystemWow64DirectoryA(&self, lp_buffer: PSTR, u_size: u32) -> u32 {
        todo!("GetSystemWow64DirectoryA")
    }
    #[doc = "GetSystemWow64DirectoryW from KERNEL32"]
    fn GetSystemWow64DirectoryW(&self, lp_buffer: PWSTR, u_size: u32) -> u32 {
        todo!("GetSystemWow64DirectoryW")
    }
    #[doc = "GetTickCount from KERNEL32"]
    fn GetTickCount(&self) -> u32 {
        todo!("GetTickCount")
    }
    #[doc = "GetVersion from KERNEL32"]
    fn GetVersion(&self) -> u32 {
        todo!("GetVersion")
    }
    #[doc = "GetVersionExA from KERNEL32"]
    fn GetVersionExA(
        &self,
        lp_version_information: MutPtr<OSVERSIONINFOA>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetVersionExA")
    }
    #[doc = "GetVersionExW from KERNEL32"]
    fn GetVersionExW(
        &self,
        lp_version_information: MutPtr<OSVERSIONINFOW>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetVersionExW")
    }
    #[doc = "GetWindowsDirectoryA from KERNEL32"]
    fn GetWindowsDirectoryA(&self, lp_buffer: PSTR, u_size: u32) -> u32 {
        todo!("GetWindowsDirectoryA")
    }
    #[doc = "GetWindowsDirectoryW from KERNEL32"]
    fn GetWindowsDirectoryW(&self, lp_buffer: PWSTR, u_size: u32) -> u32 {
        todo!("GetWindowsDirectoryW")
    }
    #[doc = "GlobalMemoryStatus from KERNEL32"]
    fn GlobalMemoryStatus(&self, lp_buffer: MutPtr<MEMORYSTATUS>) {
        todo!("GlobalMemoryStatus")
    }
    #[doc = "GlobalMemoryStatusEx from KERNEL32"]
    fn GlobalMemoryStatusEx(
        &self,
        lp_buffer: MutPtr<MEMORYSTATUSEX>,
    ) -> super::super::Foundation::BOOL {
        todo!("GlobalMemoryStatusEx")
    }
    #[doc = "SetComputerNameA from KERNEL32"]
    fn SetComputerNameA(&self, lp_computer_name: PCSTR) -> super::super::Foundation::BOOL {
        todo!("SetComputerNameA")
    }
    #[doc = "SetComputerNameEx2W from KERNEL32"]
    fn SetComputerNameEx2W(
        &self,
        name_type: COMPUTER_NAME_FORMAT,
        flags: u32,
        lp_buffer: PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("SetComputerNameEx2W")
    }
    #[doc = "SetComputerNameExA from KERNEL32"]
    fn SetComputerNameExA(
        &self,
        name_type: COMPUTER_NAME_FORMAT,
        lp_buffer: PCSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("SetComputerNameExA")
    }
    #[doc = "SetComputerNameExW from KERNEL32"]
    fn SetComputerNameExW(
        &self,
        name_type: COMPUTER_NAME_FORMAT,
        lp_buffer: PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("SetComputerNameExW")
    }
    #[doc = "SetComputerNameW from KERNEL32"]
    fn SetComputerNameW(&self, lp_computer_name: PCWSTR) -> super::super::Foundation::BOOL {
        todo!("SetComputerNameW")
    }
    #[doc = "SetLocalTime from KERNEL32"]
    fn SetLocalTime(
        &self,
        lp_system_time: ConstPtr<super::super::Foundation::SYSTEMTIME>,
    ) -> super::super::Foundation::BOOL {
        todo!("SetLocalTime")
    }
    #[doc = "SetSystemTime from KERNEL32"]
    fn SetSystemTime(
        &self,
        lp_system_time: ConstPtr<super::super::Foundation::SYSTEMTIME>,
    ) -> super::super::Foundation::BOOL {
        todo!("SetSystemTime")
    }
    #[doc = "SetSystemTimeAdjustment from KERNEL32"]
    fn SetSystemTimeAdjustment(
        &self,
        dw_time_adjustment: u32,
        b_time_adjustment_disabled: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!("SetSystemTimeAdjustment")
    }
    #[doc = "VerSetConditionMask from KERNEL32"]
    fn VerSetConditionMask(&self, condition_mask: u64, type_mask: VER_FLAGS, condition: u8) -> u64 {
        todo!("VerSetConditionMask")
    }
    #[doc = "VerifyVersionInfoA from KERNEL32"]
    fn VerifyVersionInfoA(
        &self,
        lp_version_information: MutPtr<OSVERSIONINFOEXA>,
        dw_type_mask: VER_FLAGS,
        dwl_condition_mask: u64,
    ) -> super::super::Foundation::BOOL {
        todo!("VerifyVersionInfoA")
    }
    #[doc = "VerifyVersionInfoW from KERNEL32"]
    fn VerifyVersionInfoW(
        &self,
        lp_version_information: MutPtr<OSVERSIONINFOEXW>,
        dw_type_mask: VER_FLAGS,
        dwl_condition_mask: u64,
    ) -> super::super::Foundation::BOOL {
        todo!("VerifyVersionInfoW")
    }
}
pub fn get_api(ctx: &crate::core::Win32Context) -> std::sync::Arc<dyn Api> {
    ctx.get::<dyn Api>()
}
