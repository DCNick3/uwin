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
pub struct DYNAMIC_TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: super::super::Foundation::SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: super::super::Foundation::SYSTEMTIME,
    pub DaylightBias: i32,
    pub TimeZoneKeyName: [u16; 128],
    pub DynamicDaylightTimeDisabled: super::super::Foundation::BOOLEAN,
}
impl ::core::marker::Copy for DYNAMIC_TIME_ZONE_INFORMATION {}
impl ::core::clone::Clone for DYNAMIC_TIME_ZONE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DYNAMIC_TIME_ZONE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DYNAMIC_TIME_ZONE_INFORMATION")
            .field("Bias", &self.Bias)
            .field("StandardName", &self.StandardName)
            .field("StandardDate", &self.StandardDate)
            .field("StandardBias", &self.StandardBias)
            .field("DaylightName", &self.DaylightName)
            .field("DaylightDate", &self.DaylightDate)
            .field("DaylightBias", &self.DaylightBias)
            .field("TimeZoneKeyName", &self.TimeZoneKeyName)
            .field(
                "DynamicDaylightTimeDisabled",
                &self.DynamicDaylightTimeDisabled,
            )
            .finish()
    }
}
impl ::core::cmp::PartialEq for DYNAMIC_TIME_ZONE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Bias == other.Bias
            && self.StandardName == other.StandardName
            && self.StandardDate == other.StandardDate
            && self.StandardBias == other.StandardBias
            && self.DaylightName == other.DaylightName
            && self.DaylightDate == other.DaylightDate
            && self.DaylightBias == other.DaylightBias
            && self.TimeZoneKeyName == other.TimeZoneKeyName
            && self.DynamicDaylightTimeDisabled == other.DynamicDaylightTimeDisabled
    }
}
impl ::core::cmp::Eq for DYNAMIC_TIME_ZONE_INFORMATION {}
impl FromIntoMemory for DYNAMIC_TIME_ZONE_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 240);
        let f_Bias = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_StandardName = <[u16; 32] as FromIntoMemory>::from_bytes(&from[4..4 + 32]);
        let f_StandardDate = <super::super::Foundation::SYSTEMTIME as FromIntoMemory>::from_bytes(
            &from[36..36 + 16],
        );
        let f_StandardBias = <i32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_DaylightName = <[u16; 32] as FromIntoMemory>::from_bytes(&from[56..56 + 32]);
        let f_DaylightDate = <super::super::Foundation::SYSTEMTIME as FromIntoMemory>::from_bytes(
            &from[88..88 + 16],
        );
        let f_DaylightBias = <i32 as FromIntoMemory>::from_bytes(&from[104..104 + 4]);
        let f_TimeZoneKeyName = <[u16; 128] as FromIntoMemory>::from_bytes(&from[108..108 + 128]);
        let f_DynamicDaylightTimeDisabled =
            <super::super::Foundation::BOOLEAN as FromIntoMemory>::from_bytes(&from[236..236 + 1]);
        Self {
            Bias: f_Bias,
            StandardName: f_StandardName,
            StandardDate: f_StandardDate,
            StandardBias: f_StandardBias,
            DaylightName: f_DaylightName,
            DaylightDate: f_DaylightDate,
            DaylightBias: f_DaylightBias,
            TimeZoneKeyName: f_TimeZoneKeyName,
            DynamicDaylightTimeDisabled: f_DynamicDaylightTimeDisabled,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 240);
        FromIntoMemory::into_bytes(self.Bias, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.StandardName, &mut into[4..4 + 32]);
        FromIntoMemory::into_bytes(self.StandardDate, &mut into[36..36 + 16]);
        FromIntoMemory::into_bytes(self.StandardBias, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.DaylightName, &mut into[56..56 + 32]);
        FromIntoMemory::into_bytes(self.DaylightDate, &mut into[88..88 + 16]);
        FromIntoMemory::into_bytes(self.DaylightBias, &mut into[104..104 + 4]);
        FromIntoMemory::into_bytes(self.TimeZoneKeyName, &mut into[108..108 + 128]);
        FromIntoMemory::into_bytes(self.DynamicDaylightTimeDisabled, &mut into[236..236 + 1]);
    }
    fn size() -> usize {
        240
    }
}
pub struct TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: super::super::Foundation::SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: super::super::Foundation::SYSTEMTIME,
    pub DaylightBias: i32,
}
impl ::core::marker::Copy for TIME_ZONE_INFORMATION {}
impl ::core::clone::Clone for TIME_ZONE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TIME_ZONE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TIME_ZONE_INFORMATION")
            .field("Bias", &self.Bias)
            .field("StandardName", &self.StandardName)
            .field("StandardDate", &self.StandardDate)
            .field("StandardBias", &self.StandardBias)
            .field("DaylightName", &self.DaylightName)
            .field("DaylightDate", &self.DaylightDate)
            .field("DaylightBias", &self.DaylightBias)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TIME_ZONE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Bias == other.Bias
            && self.StandardName == other.StandardName
            && self.StandardDate == other.StandardDate
            && self.StandardBias == other.StandardBias
            && self.DaylightName == other.DaylightName
            && self.DaylightDate == other.DaylightDate
            && self.DaylightBias == other.DaylightBias
    }
}
impl ::core::cmp::Eq for TIME_ZONE_INFORMATION {}
impl FromIntoMemory for TIME_ZONE_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 108);
        let f_Bias = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_StandardName = <[u16; 32] as FromIntoMemory>::from_bytes(&from[4..4 + 32]);
        let f_StandardDate = <super::super::Foundation::SYSTEMTIME as FromIntoMemory>::from_bytes(
            &from[36..36 + 16],
        );
        let f_StandardBias = <i32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_DaylightName = <[u16; 32] as FromIntoMemory>::from_bytes(&from[56..56 + 32]);
        let f_DaylightDate = <super::super::Foundation::SYSTEMTIME as FromIntoMemory>::from_bytes(
            &from[88..88 + 16],
        );
        let f_DaylightBias = <i32 as FromIntoMemory>::from_bytes(&from[104..104 + 4]);
        Self {
            Bias: f_Bias,
            StandardName: f_StandardName,
            StandardDate: f_StandardDate,
            StandardBias: f_StandardBias,
            DaylightName: f_DaylightName,
            DaylightDate: f_DaylightDate,
            DaylightBias: f_DaylightBias,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 108);
        FromIntoMemory::into_bytes(self.Bias, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.StandardName, &mut into[4..4 + 32]);
        FromIntoMemory::into_bytes(self.StandardDate, &mut into[36..36 + 16]);
        FromIntoMemory::into_bytes(self.StandardBias, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.DaylightName, &mut into[56..56 + 32]);
        FromIntoMemory::into_bytes(self.DaylightDate, &mut into[88..88 + 16]);
        FromIntoMemory::into_bytes(self.DaylightBias, &mut into[104..104 + 4]);
    }
    fn size() -> usize {
        108
    }
}
pub const TSF_Authenticated: u32 = 2u32;
pub const TSF_Hardware: u32 = 1u32;
pub const TSF_IPv6: u32 = 4u32;
pub const TSF_SignatureAuthenticated: u32 = 8u32;
pub const wszW32TimeRegKeyPolicyTimeProviders: &'static str =
    "Software\\Policies\\Microsoft\\W32Time\\TimeProviders";
pub const wszW32TimeRegKeyTimeProviders: &'static str =
    "System\\CurrentControlSet\\Services\\W32Time\\TimeProviders";
pub const wszW32TimeRegValueDllName: &'static str = "DllName";
pub const wszW32TimeRegValueEnabled: &'static str = "Enabled";
pub const wszW32TimeRegValueInputProvider: &'static str = "InputProvider";
pub const wszW32TimeRegValueMetaDataProvider: &'static str = "MetaDataProvider";
pub trait Api {
    #[doc = "FileTimeToSystemTime from KERNEL32"]
    fn FileTimeToSystemTime(
        &self,
        lp_file_time: ConstPtr<super::super::Foundation::FILETIME>,
        lp_system_time: MutPtr<super::super::Foundation::SYSTEMTIME>,
    ) -> super::super::Foundation::BOOL {
        todo!("FileTimeToSystemTime")
    }
    #[doc = "GetTimeZoneInformation from KERNEL32"]
    fn GetTimeZoneInformation(
        &self,
        lp_time_zone_information: MutPtr<TIME_ZONE_INFORMATION>,
    ) -> u32 {
        todo!("GetTimeZoneInformation")
    }
    #[doc = "LocalFileTimeToLocalSystemTime from KERNEL32"]
    fn LocalFileTimeToLocalSystemTime(
        &self,
        time_zone_information: ConstPtr<TIME_ZONE_INFORMATION>,
        local_file_time: ConstPtr<super::super::Foundation::FILETIME>,
        local_system_time: MutPtr<super::super::Foundation::SYSTEMTIME>,
    ) -> super::super::Foundation::BOOL {
        todo!("LocalFileTimeToLocalSystemTime")
    }
    #[doc = "LocalSystemTimeToLocalFileTime from KERNEL32"]
    fn LocalSystemTimeToLocalFileTime(
        &self,
        time_zone_information: ConstPtr<TIME_ZONE_INFORMATION>,
        local_system_time: ConstPtr<super::super::Foundation::SYSTEMTIME>,
        local_file_time: MutPtr<super::super::Foundation::FILETIME>,
    ) -> super::super::Foundation::BOOL {
        todo!("LocalSystemTimeToLocalFileTime")
    }
    #[doc = "SetTimeZoneInformation from KERNEL32"]
    fn SetTimeZoneInformation(
        &self,
        lp_time_zone_information: ConstPtr<TIME_ZONE_INFORMATION>,
    ) -> super::super::Foundation::BOOL {
        todo!("SetTimeZoneInformation")
    }
    #[doc = "SystemTimeToFileTime from KERNEL32"]
    fn SystemTimeToFileTime(
        &self,
        lp_system_time: ConstPtr<super::super::Foundation::SYSTEMTIME>,
        lp_file_time: MutPtr<super::super::Foundation::FILETIME>,
    ) -> super::super::Foundation::BOOL {
        todo!("SystemTimeToFileTime")
    }
    #[doc = "SystemTimeToTzSpecificLocalTime from KERNEL32"]
    fn SystemTimeToTzSpecificLocalTime(
        &self,
        lp_time_zone_information: ConstPtr<TIME_ZONE_INFORMATION>,
        lp_universal_time: ConstPtr<super::super::Foundation::SYSTEMTIME>,
        lp_local_time: MutPtr<super::super::Foundation::SYSTEMTIME>,
    ) -> super::super::Foundation::BOOL {
        todo!("SystemTimeToTzSpecificLocalTime")
    }
    #[doc = "TzSpecificLocalTimeToSystemTime from KERNEL32"]
    fn TzSpecificLocalTimeToSystemTime(
        &self,
        lp_time_zone_information: ConstPtr<TIME_ZONE_INFORMATION>,
        lp_local_time: ConstPtr<super::super::Foundation::SYSTEMTIME>,
        lp_universal_time: MutPtr<super::super::Foundation::SYSTEMTIME>,
    ) -> super::super::Foundation::BOOL {
        todo!("TzSpecificLocalTimeToSystemTime")
    }
}
pub fn get_api(ctx: &crate::core::Win32Context) -> std::sync::Arc<dyn Api> {
    ctx.get::<dyn Api>()
}
