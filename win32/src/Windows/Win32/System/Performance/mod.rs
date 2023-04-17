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
pub const AppearPropPage: crate::core::GUID =
    crate::core::GUID::from_u128(0xe49741e9_93a8_4ab1_8e96_bf4482282e9c);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AutoPathFormat(pub i32);
pub const plaNone: AutoPathFormat = AutoPathFormat(0i32);
pub const plaPattern: AutoPathFormat = AutoPathFormat(1i32);
pub const plaComputer: AutoPathFormat = AutoPathFormat(2i32);
pub const plaMonthDayHour: AutoPathFormat = AutoPathFormat(256i32);
pub const plaSerialNumber: AutoPathFormat = AutoPathFormat(512i32);
pub const plaYearDayOfYear: AutoPathFormat = AutoPathFormat(1024i32);
pub const plaYearMonth: AutoPathFormat = AutoPathFormat(2048i32);
pub const plaYearMonthDay: AutoPathFormat = AutoPathFormat(4096i32);
pub const plaYearMonthDayHour: AutoPathFormat = AutoPathFormat(8192i32);
pub const plaMonthDayHourMinute: AutoPathFormat = AutoPathFormat(16384i32);
impl ::core::marker::Copy for AutoPathFormat {}
impl ::core::clone::Clone for AutoPathFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutoPathFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AutoPathFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutoPathFormat").field(&self.0).finish()
    }
}
impl FromIntoMemory for AutoPathFormat {
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
pub const BootTraceSession: crate::core::GUID =
    crate::core::GUID::from_u128(0x03837538_098b_11d8_9414_505054503030);
pub const BootTraceSessionCollection: crate::core::GUID =
    crate::core::GUID::from_u128(0x03837539_098b_11d8_9414_505054503030);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ClockType(pub i32);
pub const plaTimeStamp: ClockType = ClockType(0i32);
pub const plaPerformance: ClockType = ClockType(1i32);
pub const plaSystem: ClockType = ClockType(2i32);
pub const plaCycle: ClockType = ClockType(3i32);
impl ::core::marker::Copy for ClockType {}
impl ::core::clone::Clone for ClockType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ClockType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ClockType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClockType").field(&self.0).finish()
    }
}
impl FromIntoMemory for ClockType {
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
pub struct CommitMode(pub i32);
pub const plaCreateNew: CommitMode = CommitMode(1i32);
pub const plaModify: CommitMode = CommitMode(2i32);
pub const plaCreateOrModify: CommitMode = CommitMode(3i32);
pub const plaUpdateRunningInstance: CommitMode = CommitMode(16i32);
pub const plaFlushTrace: CommitMode = CommitMode(32i32);
pub const plaValidateOnly: CommitMode = CommitMode(4096i32);
impl ::core::marker::Copy for CommitMode {}
impl ::core::clone::Clone for CommitMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CommitMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CommitMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CommitMode").field(&self.0).finish()
    }
}
impl FromIntoMemory for CommitMode {
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
pub const CounterItem: crate::core::GUID =
    crate::core::GUID::from_u128(0xc4d2d8e0_d1dd_11ce_940f_008029004348);
pub const CounterItem2: crate::core::GUID =
    crate::core::GUID::from_u128(0x43196c62_c31f_4ce3_a02e_79efe0f6a525);
pub type CounterPathCallBack = StdCallFnPtr<(PtrRepr,), i32>;
pub const CounterPropPage: crate::core::GUID =
    crate::core::GUID::from_u128(0xcf948561_ede8_11ce_941e_008029004347);
pub const Counters: crate::core::GUID =
    crate::core::GUID::from_u128(0xb2b066d2_2aac_11cf_942f_008029004347);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct DICounterItem(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait DICounterItem_Trait: super::Com::IDispatch_Trait {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for DICounterItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for DICounterItem {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for DICounterItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for DICounterItem {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for DICounterItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DICounterItem").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for DICounterItem {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for DICounterItem {
    type Super = super::Com::IDispatch;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xc08c4ff2_0e2e_11cf_942c_008029004347);
}
pub const DIID_DICounterItem: crate::core::GUID =
    crate::core::GUID::from_u128(0xc08c4ff2_0e2e_11cf_942c_008029004347);
pub const DIID_DILogFileItem: crate::core::GUID =
    crate::core::GUID::from_u128(0x8d093ffc_f777_4917_82d1_833fbc54c58f);
pub const DIID_DISystemMonitor: crate::core::GUID =
    crate::core::GUID::from_u128(0x13d73d81_c32e_11cf_9398_00aa00a3ddea);
pub const DIID_DISystemMonitorEvents: crate::core::GUID =
    crate::core::GUID::from_u128(0x84979930_4ab3_11cf_943a_008029004347);
pub const DIID_DISystemMonitorInternal: crate::core::GUID =
    crate::core::GUID::from_u128(0x194eb242_c32c_11cf_9398_00aa00a3ddea);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct DILogFileItem(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait DILogFileItem_Trait: super::Com::IDispatch_Trait {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for DILogFileItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for DILogFileItem {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for DILogFileItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for DILogFileItem {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for DILogFileItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DILogFileItem").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for DILogFileItem {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for DILogFileItem {
    type Super = super::Com::IDispatch;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x8d093ffc_f777_4917_82d1_833fbc54c58f);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct DISystemMonitor(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait DISystemMonitor_Trait: super::Com::IDispatch_Trait {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for DISystemMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for DISystemMonitor {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for DISystemMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for DISystemMonitor {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for DISystemMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISystemMonitor").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for DISystemMonitor {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for DISystemMonitor {
    type Super = super::Com::IDispatch;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x13d73d81_c32e_11cf_9398_00aa00a3ddea);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct DISystemMonitorEvents(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait DISystemMonitorEvents_Trait: super::Com::IDispatch_Trait {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for DISystemMonitorEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for DISystemMonitorEvents {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for DISystemMonitorEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for DISystemMonitorEvents {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for DISystemMonitorEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISystemMonitorEvents")
            .field(&self.0)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for DISystemMonitorEvents {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for DISystemMonitorEvents {
    type Super = super::Com::IDispatch;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x84979930_4ab3_11cf_943a_008029004347);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct DISystemMonitorInternal(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait DISystemMonitorInternal_Trait: super::Com::IDispatch_Trait {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for DISystemMonitorInternal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for DISystemMonitorInternal {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for DISystemMonitorInternal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for DISystemMonitorInternal {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for DISystemMonitorInternal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISystemMonitorInternal")
            .field(&self.0)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for DISystemMonitorInternal {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for DISystemMonitorInternal {
    type Super = super::Com::IDispatch;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x194eb242_c32c_11cf_9398_00aa00a3ddea);
}
pub const DataCollectorSet: crate::core::GUID =
    crate::core::GUID::from_u128(0x03837521_098b_11d8_9414_505054503030);
pub const DataCollectorSetCollection: crate::core::GUID =
    crate::core::GUID::from_u128(0x03837525_098b_11d8_9414_505054503030);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DataCollectorSetStatus(pub i32);
pub const plaStopped: DataCollectorSetStatus = DataCollectorSetStatus(0i32);
pub const plaRunning: DataCollectorSetStatus = DataCollectorSetStatus(1i32);
pub const plaCompiling: DataCollectorSetStatus = DataCollectorSetStatus(2i32);
pub const plaPending: DataCollectorSetStatus = DataCollectorSetStatus(3i32);
pub const plaUndefined: DataCollectorSetStatus = DataCollectorSetStatus(4i32);
impl ::core::marker::Copy for DataCollectorSetStatus {}
impl ::core::clone::Clone for DataCollectorSetStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DataCollectorSetStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DataCollectorSetStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataCollectorSetStatus")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for DataCollectorSetStatus {
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
pub struct DataCollectorType(pub i32);
pub const plaPerformanceCounter: DataCollectorType = DataCollectorType(0i32);
pub const plaTrace: DataCollectorType = DataCollectorType(1i32);
pub const plaConfiguration: DataCollectorType = DataCollectorType(2i32);
pub const plaAlert: DataCollectorType = DataCollectorType(3i32);
pub const plaApiTrace: DataCollectorType = DataCollectorType(4i32);
impl ::core::marker::Copy for DataCollectorType {}
impl ::core::clone::Clone for DataCollectorType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DataCollectorType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DataCollectorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataCollectorType").field(&self.0).finish()
    }
}
impl FromIntoMemory for DataCollectorType {
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
pub struct DataManagerSteps(pub i32);
pub const plaCreateReport: DataManagerSteps = DataManagerSteps(1i32);
pub const plaRunRules: DataManagerSteps = DataManagerSteps(2i32);
pub const plaCreateHtml: DataManagerSteps = DataManagerSteps(4i32);
pub const plaFolderActions: DataManagerSteps = DataManagerSteps(8i32);
pub const plaResourceFreeing: DataManagerSteps = DataManagerSteps(16i32);
impl ::core::marker::Copy for DataManagerSteps {}
impl ::core::clone::Clone for DataManagerSteps {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DataManagerSteps {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DataManagerSteps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataManagerSteps").field(&self.0).finish()
    }
}
impl FromIntoMemory for DataManagerSteps {
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
pub struct DataSourceTypeConstants(pub i32);
pub const sysmonNullDataSource: DataSourceTypeConstants = DataSourceTypeConstants(-1i32);
pub const sysmonCurrentActivity: DataSourceTypeConstants = DataSourceTypeConstants(1i32);
pub const sysmonLogFiles: DataSourceTypeConstants = DataSourceTypeConstants(2i32);
pub const sysmonSqlLog: DataSourceTypeConstants = DataSourceTypeConstants(3i32);
impl ::core::marker::Copy for DataSourceTypeConstants {}
impl ::core::clone::Clone for DataSourceTypeConstants {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DataSourceTypeConstants {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DataSourceTypeConstants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataSourceTypeConstants")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for DataSourceTypeConstants {
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
pub struct DisplayTypeConstants(pub i32);
pub const sysmonLineGraph: DisplayTypeConstants = DisplayTypeConstants(1i32);
pub const sysmonHistogram: DisplayTypeConstants = DisplayTypeConstants(2i32);
pub const sysmonReport: DisplayTypeConstants = DisplayTypeConstants(3i32);
pub const sysmonChartArea: DisplayTypeConstants = DisplayTypeConstants(4i32);
pub const sysmonChartStackedArea: DisplayTypeConstants = DisplayTypeConstants(5i32);
impl ::core::marker::Copy for DisplayTypeConstants {}
impl ::core::clone::Clone for DisplayTypeConstants {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayTypeConstants {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DisplayTypeConstants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayTypeConstants")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for DisplayTypeConstants {
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
pub struct FileFormat(pub i32);
pub const plaCommaSeparated: FileFormat = FileFormat(0i32);
pub const plaTabSeparated: FileFormat = FileFormat(1i32);
pub const plaSql: FileFormat = FileFormat(2i32);
pub const plaBinary: FileFormat = FileFormat(3i32);
impl ::core::marker::Copy for FileFormat {}
impl ::core::clone::Clone for FileFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FileFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FileFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileFormat").field(&self.0).finish()
    }
}
impl FromIntoMemory for FileFormat {
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
pub struct FolderActionSteps(pub i32);
pub const plaCreateCab: FolderActionSteps = FolderActionSteps(1i32);
pub const plaDeleteData: FolderActionSteps = FolderActionSteps(2i32);
pub const plaSendCab: FolderActionSteps = FolderActionSteps(4i32);
pub const plaDeleteCab: FolderActionSteps = FolderActionSteps(8i32);
pub const plaDeleteReport: FolderActionSteps = FolderActionSteps(16i32);
impl ::core::marker::Copy for FolderActionSteps {}
impl ::core::clone::Clone for FolderActionSteps {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FolderActionSteps {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FolderActionSteps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FolderActionSteps").field(&self.0).finish()
    }
}
impl FromIntoMemory for FolderActionSteps {
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
pub const GeneralPropPage: crate::core::GUID =
    crate::core::GUID::from_u128(0xc3e5d3d2_1a03_11cf_942d_008029004347);
pub const GraphPropPage: crate::core::GUID =
    crate::core::GUID::from_u128(0xc3e5d3d3_1a03_11cf_942d_008029004347);
pub const H_WBEM_DATASOURCE: i32 = -1i32;
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IAlertDataCollector(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IAlertDataCollector_Trait: IDataCollector_Trait {
    fn get_AlertThresholds(
        &self,
        alerts: MutPtr<ConstPtr<super::Com::SAFEARRAY>>,
    ) -> crate::core::HRESULT {
        todo!("get_AlertThresholds")
    }
    fn put_AlertThresholds(&self, alerts: ConstPtr<super::Com::SAFEARRAY>) -> crate::core::HRESULT {
        todo!("put_AlertThresholds")
    }
    fn get_EventLog(&self, log: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_EventLog")
    }
    fn put_EventLog(&self, log: i16) -> crate::core::HRESULT {
        todo!("put_EventLog")
    }
    fn get_SampleInterval(&self, interval: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_SampleInterval")
    }
    fn put_SampleInterval(&self, interval: u32) -> crate::core::HRESULT {
        todo!("put_SampleInterval")
    }
    fn get_Task(&self, task: MutPtr<super::super::Foundation::BSTR>) -> crate::core::HRESULT {
        todo!("get_Task")
    }
    fn put_Task(&self, task: super::super::Foundation::BSTR) -> crate::core::HRESULT {
        todo!("put_Task")
    }
    fn get_TaskRunAsSelf(&self, run_as_self: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_TaskRunAsSelf")
    }
    fn put_TaskRunAsSelf(&self, run_as_self: i16) -> crate::core::HRESULT {
        todo!("put_TaskRunAsSelf")
    }
    fn get_TaskArguments(
        &self,
        task: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_TaskArguments")
    }
    fn put_TaskArguments(&self, task: super::super::Foundation::BSTR) -> crate::core::HRESULT {
        todo!("put_TaskArguments")
    }
    fn get_TaskUserTextArguments(
        &self,
        task: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_TaskUserTextArguments")
    }
    fn put_TaskUserTextArguments(
        &self,
        task: super::super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("put_TaskUserTextArguments")
    }
    fn get_TriggerDataCollectorSet(
        &self,
        name: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_TriggerDataCollectorSet")
    }
    fn put_TriggerDataCollectorSet(
        &self,
        name: super::super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("put_TriggerDataCollectorSet")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IAlertDataCollector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IAlertDataCollector {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IAlertDataCollector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IAlertDataCollector {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IAlertDataCollector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAlertDataCollector").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IAlertDataCollector {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IAlertDataCollector {
    type Super = IDataCollector;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x03837516_098b_11d8_9414_505054503030);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IApiTracingDataCollector(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IApiTracingDataCollector_Trait: IDataCollector_Trait {
    fn get_LogApiNamesOnly(&self, logapinames: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_LogApiNamesOnly")
    }
    fn put_LogApiNamesOnly(&self, logapinames: i16) -> crate::core::HRESULT {
        todo!("put_LogApiNamesOnly")
    }
    fn get_LogApisRecursively(&self, logrecursively: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_LogApisRecursively")
    }
    fn put_LogApisRecursively(&self, logrecursively: i16) -> crate::core::HRESULT {
        todo!("put_LogApisRecursively")
    }
    fn get_ExePath(&self, exepath: MutPtr<super::super::Foundation::BSTR>) -> crate::core::HRESULT {
        todo!("get_ExePath")
    }
    fn put_ExePath(&self, exepath: super::super::Foundation::BSTR) -> crate::core::HRESULT {
        todo!("put_ExePath")
    }
    fn get_LogFilePath(
        &self,
        logfilepath: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_LogFilePath")
    }
    fn put_LogFilePath(&self, logfilepath: super::super::Foundation::BSTR) -> crate::core::HRESULT {
        todo!("put_LogFilePath")
    }
    fn get_IncludeModules(
        &self,
        includemodules: MutPtr<ConstPtr<super::Com::SAFEARRAY>>,
    ) -> crate::core::HRESULT {
        todo!("get_IncludeModules")
    }
    fn put_IncludeModules(
        &self,
        includemodules: ConstPtr<super::Com::SAFEARRAY>,
    ) -> crate::core::HRESULT {
        todo!("put_IncludeModules")
    }
    fn get_IncludeApis(
        &self,
        includeapis: MutPtr<ConstPtr<super::Com::SAFEARRAY>>,
    ) -> crate::core::HRESULT {
        todo!("get_IncludeApis")
    }
    fn put_IncludeApis(
        &self,
        includeapis: ConstPtr<super::Com::SAFEARRAY>,
    ) -> crate::core::HRESULT {
        todo!("put_IncludeApis")
    }
    fn get_ExcludeApis(
        &self,
        excludeapis: MutPtr<ConstPtr<super::Com::SAFEARRAY>>,
    ) -> crate::core::HRESULT {
        todo!("get_ExcludeApis")
    }
    fn put_ExcludeApis(
        &self,
        excludeapis: ConstPtr<super::Com::SAFEARRAY>,
    ) -> crate::core::HRESULT {
        todo!("put_ExcludeApis")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IApiTracingDataCollector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IApiTracingDataCollector {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IApiTracingDataCollector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IApiTracingDataCollector {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IApiTracingDataCollector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApiTracingDataCollector")
            .field(&self.0)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IApiTracingDataCollector {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IApiTracingDataCollector {
    type Super = IDataCollector;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x0383751a_098b_11d8_9414_505054503030);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IConfigurationDataCollector(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IConfigurationDataCollector_Trait: IDataCollector_Trait {
    fn get_FileMaxCount(&self, count: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_FileMaxCount")
    }
    fn put_FileMaxCount(&self, count: u32) -> crate::core::HRESULT {
        todo!("put_FileMaxCount")
    }
    fn get_FileMaxRecursiveDepth(&self, depth: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_FileMaxRecursiveDepth")
    }
    fn put_FileMaxRecursiveDepth(&self, depth: u32) -> crate::core::HRESULT {
        todo!("put_FileMaxRecursiveDepth")
    }
    fn get_FileMaxTotalSize(&self, size: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_FileMaxTotalSize")
    }
    fn put_FileMaxTotalSize(&self, size: u32) -> crate::core::HRESULT {
        todo!("put_FileMaxTotalSize")
    }
    fn get_Files(&self, files: MutPtr<ConstPtr<super::Com::SAFEARRAY>>) -> crate::core::HRESULT {
        todo!("get_Files")
    }
    fn put_Files(&self, files: ConstPtr<super::Com::SAFEARRAY>) -> crate::core::HRESULT {
        todo!("put_Files")
    }
    fn get_ManagementQueries(
        &self,
        queries: MutPtr<ConstPtr<super::Com::SAFEARRAY>>,
    ) -> crate::core::HRESULT {
        todo!("get_ManagementQueries")
    }
    fn put_ManagementQueries(
        &self,
        queries: ConstPtr<super::Com::SAFEARRAY>,
    ) -> crate::core::HRESULT {
        todo!("put_ManagementQueries")
    }
    fn get_QueryNetworkAdapters(&self, network: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_QueryNetworkAdapters")
    }
    fn put_QueryNetworkAdapters(&self, network: i16) -> crate::core::HRESULT {
        todo!("put_QueryNetworkAdapters")
    }
    fn get_RegistryKeys(
        &self,
        query: MutPtr<ConstPtr<super::Com::SAFEARRAY>>,
    ) -> crate::core::HRESULT {
        todo!("get_RegistryKeys")
    }
    fn put_RegistryKeys(&self, query: ConstPtr<super::Com::SAFEARRAY>) -> crate::core::HRESULT {
        todo!("put_RegistryKeys")
    }
    fn get_RegistryMaxRecursiveDepth(&self, depth: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_RegistryMaxRecursiveDepth")
    }
    fn put_RegistryMaxRecursiveDepth(&self, depth: u32) -> crate::core::HRESULT {
        todo!("put_RegistryMaxRecursiveDepth")
    }
    fn get_SystemStateFile(
        &self,
        file_name: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_SystemStateFile")
    }
    fn put_SystemStateFile(
        &self,
        file_name: super::super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("put_SystemStateFile")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IConfigurationDataCollector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IConfigurationDataCollector {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IConfigurationDataCollector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IConfigurationDataCollector {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IConfigurationDataCollector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConfigurationDataCollector")
            .field(&self.0)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IConfigurationDataCollector {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IConfigurationDataCollector {
    type Super = IDataCollector;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x03837514_098b_11d8_9414_505054503030);
}
pub struct ICounterItem(pub crate::core::IUnknown);
pub trait ICounterItem_Trait: crate::core::IUnknown_Trait {
    fn get_Value(&self, pdbl_value: MutPtr<f64>) -> crate::core::HRESULT {
        todo!("get_Value")
    }
    fn put_Color(&self, color: u32) -> crate::core::HRESULT {
        todo!("put_Color")
    }
    fn get_Color(&self, p_color: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_Color")
    }
    fn put_Width(&self, i_width: i32) -> crate::core::HRESULT {
        todo!("put_Width")
    }
    fn get_Width(&self, pi_value: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("get_Width")
    }
    fn put_LineStyle(&self, i_line_style: i32) -> crate::core::HRESULT {
        todo!("put_LineStyle")
    }
    fn get_LineStyle(&self, pi_value: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("get_LineStyle")
    }
    fn put_ScaleFactor(&self, i_scale: i32) -> crate::core::HRESULT {
        todo!("put_ScaleFactor")
    }
    fn get_ScaleFactor(&self, pi_value: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("get_ScaleFactor")
    }
    fn get_Path(&self, pstr_value: MutPtr<super::super::Foundation::BSTR>) -> crate::core::HRESULT {
        todo!("get_Path")
    }
    fn GetValue(&self, value: MutPtr<f64>, status: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("GetValue")
    }
    fn GetStatistics(
        &self,
        max: MutPtr<f64>,
        min: MutPtr<f64>,
        avg: MutPtr<f64>,
        status: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("GetStatistics")
    }
}
impl ::core::clone::Clone for ICounterItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ICounterItem {}
impl ::core::cmp::PartialEq for ICounterItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICounterItem {}
impl ::core::fmt::Debug for ICounterItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICounterItem").field(&self.0).finish()
    }
}
impl FromIntoMemory for ICounterItem {
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
impl crate::core::ComInterface for ICounterItem {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x771a9520_ee28_11ce_941e_008029004347);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ICounterItem2(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait ICounterItem2_Trait: ICounterItem_Trait {
    fn put_Selected(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_Selected")
    }
    fn get_Selected(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_Selected")
    }
    fn put_Visible(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_Visible")
    }
    fn get_Visible(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_Visible")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetDataAt(
        &self,
        i_index: i32,
        i_which: SysmonDataType,
        p_variant: MutPtr<super::Com::VARIANT>,
    ) -> crate::core::HRESULT {
        todo!("GetDataAt")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ICounterItem2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ICounterItem2 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ICounterItem2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ICounterItem2 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ICounterItem2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICounterItem2").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ICounterItem2 {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for ICounterItem2 {
    type Super = ICounterItem;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xeefcd4e1_ea1c_4435_b7f4_e341ba03b4f9);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ICounters(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait ICounters_Trait: super::Com::IDispatch_Trait {
    fn get_Count(&self, p_long: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("get_Count")
    }
    fn get__NewEnum(&self, pp_iunk: MutPtr<crate::core::IUnknown>) -> crate::core::HRESULT {
        todo!("get__NewEnum")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_Item(
        &self,
        index: super::Com::VARIANT,
        pp_i: MutPtr<DICounterItem>,
    ) -> crate::core::HRESULT {
        todo!("get_Item")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Add(
        &self,
        pathname: super::super::Foundation::BSTR,
        pp_i: MutPtr<DICounterItem>,
    ) -> crate::core::HRESULT {
        todo!("Add")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Remove(&self, index: super::Com::VARIANT) -> crate::core::HRESULT {
        todo!("Remove")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ICounters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ICounters {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ICounters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ICounters {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ICounters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICounters").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ICounters {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for ICounters {
    type Super = super::Com::IDispatch;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x79167962_28fc_11cf_942f_008029004347);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IDataCollector(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IDataCollector_Trait: super::Com::IDispatch_Trait {
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_DataCollectorSet(&self, group: MutPtr<IDataCollectorSet>) -> crate::core::HRESULT {
        todo!("get_DataCollectorSet")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn put_DataCollectorSet(&self, group: IDataCollectorSet) -> crate::core::HRESULT {
        todo!("put_DataCollectorSet")
    }
    fn get_DataCollectorType(&self, r#type: MutPtr<DataCollectorType>) -> crate::core::HRESULT {
        todo!("get_DataCollectorType")
    }
    fn get_FileName(&self, name: MutPtr<super::super::Foundation::BSTR>) -> crate::core::HRESULT {
        todo!("get_FileName")
    }
    fn put_FileName(&self, name: super::super::Foundation::BSTR) -> crate::core::HRESULT {
        todo!("put_FileName")
    }
    fn get_FileNameFormat(&self, format: MutPtr<AutoPathFormat>) -> crate::core::HRESULT {
        todo!("get_FileNameFormat")
    }
    fn put_FileNameFormat(&self, format: AutoPathFormat) -> crate::core::HRESULT {
        todo!("put_FileNameFormat")
    }
    fn get_FileNameFormatPattern(
        &self,
        pattern: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_FileNameFormatPattern")
    }
    fn put_FileNameFormatPattern(
        &self,
        pattern: super::super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("put_FileNameFormatPattern")
    }
    fn get_LatestOutputLocation(
        &self,
        path: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_LatestOutputLocation")
    }
    fn put_LatestOutputLocation(
        &self,
        path: super::super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("put_LatestOutputLocation")
    }
    fn get_LogAppend(&self, append: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_LogAppend")
    }
    fn put_LogAppend(&self, append: i16) -> crate::core::HRESULT {
        todo!("put_LogAppend")
    }
    fn get_LogCircular(&self, circular: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_LogCircular")
    }
    fn put_LogCircular(&self, circular: i16) -> crate::core::HRESULT {
        todo!("put_LogCircular")
    }
    fn get_LogOverwrite(&self, overwrite: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_LogOverwrite")
    }
    fn put_LogOverwrite(&self, overwrite: i16) -> crate::core::HRESULT {
        todo!("put_LogOverwrite")
    }
    fn get_Name(&self, name: MutPtr<super::super::Foundation::BSTR>) -> crate::core::HRESULT {
        todo!("get_Name")
    }
    fn put_Name(&self, name: super::super::Foundation::BSTR) -> crate::core::HRESULT {
        todo!("put_Name")
    }
    fn get_OutputLocation(
        &self,
        path: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_OutputLocation")
    }
    fn get_Index(&self, index: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("get_Index")
    }
    fn put_Index(&self, index: i32) -> crate::core::HRESULT {
        todo!("put_Index")
    }
    fn get_Xml(&self, xml: MutPtr<super::super::Foundation::BSTR>) -> crate::core::HRESULT {
        todo!("get_Xml")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn SetXml(
        &self,
        xml: super::super::Foundation::BSTR,
        validation: MutPtr<IValueMap>,
    ) -> crate::core::HRESULT {
        todo!("SetXml")
    }
    fn CreateOutputLocation(
        &self,
        latest: i16,
        location: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("CreateOutputLocation")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IDataCollector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IDataCollector {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IDataCollector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IDataCollector {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IDataCollector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataCollector").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IDataCollector {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IDataCollector {
    type Super = super::Com::IDispatch;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x038374ff_098b_11d8_9414_505054503030);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IDataCollectorCollection(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IDataCollectorCollection_Trait: super::Com::IDispatch_Trait {
    fn get_Count(&self, ret_val: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("get_Count")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_Item(
        &self,
        index: super::Com::VARIANT,
        collector: MutPtr<IDataCollector>,
    ) -> crate::core::HRESULT {
        todo!("get_Item")
    }
    fn get__NewEnum(&self, ret_val: MutPtr<crate::core::IUnknown>) -> crate::core::HRESULT {
        todo!("get__NewEnum")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Add(&self, collector: IDataCollector) -> crate::core::HRESULT {
        todo!("Add")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Remove(&self, collector: super::Com::VARIANT) -> crate::core::HRESULT {
        todo!("Remove")
    }
    fn Clear(&self) -> crate::core::HRESULT {
        todo!("Clear")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn AddRange(&self, collectors: IDataCollectorCollection) -> crate::core::HRESULT {
        todo!("AddRange")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateDataCollectorFromXml(
        &self,
        bstr_xml: super::super::Foundation::BSTR,
        p_validation: MutPtr<IValueMap>,
        p_collector: MutPtr<IDataCollector>,
    ) -> crate::core::HRESULT {
        todo!("CreateDataCollectorFromXml")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateDataCollector(
        &self,
        r#type: DataCollectorType,
        collector: MutPtr<IDataCollector>,
    ) -> crate::core::HRESULT {
        todo!("CreateDataCollector")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IDataCollectorCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IDataCollectorCollection {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IDataCollectorCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IDataCollectorCollection {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IDataCollectorCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataCollectorCollection")
            .field(&self.0)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IDataCollectorCollection {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IDataCollectorCollection {
    type Super = super::Com::IDispatch;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x03837502_098b_11d8_9414_505054503030);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IDataCollectorSet(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IDataCollectorSet_Trait: super::Com::IDispatch_Trait {
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_DataCollectors(
        &self,
        collectors: MutPtr<IDataCollectorCollection>,
    ) -> crate::core::HRESULT {
        todo!("get_DataCollectors")
    }
    fn get_Duration(&self, seconds: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_Duration")
    }
    fn put_Duration(&self, seconds: u32) -> crate::core::HRESULT {
        todo!("put_Duration")
    }
    fn get_Description(
        &self,
        description: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_Description")
    }
    fn put_Description(&self, description: super::super::Foundation::BSTR) -> crate::core::HRESULT {
        todo!("put_Description")
    }
    fn get_DescriptionUnresolved(
        &self,
        descr: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_DescriptionUnresolved")
    }
    fn get_DisplayName(
        &self,
        display_name: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_DisplayName")
    }
    fn put_DisplayName(
        &self,
        display_name: super::super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("put_DisplayName")
    }
    fn get_DisplayNameUnresolved(
        &self,
        name: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_DisplayNameUnresolved")
    }
    fn get_Keywords(
        &self,
        keywords: MutPtr<ConstPtr<super::Com::SAFEARRAY>>,
    ) -> crate::core::HRESULT {
        todo!("get_Keywords")
    }
    fn put_Keywords(&self, keywords: ConstPtr<super::Com::SAFEARRAY>) -> crate::core::HRESULT {
        todo!("put_Keywords")
    }
    fn get_LatestOutputLocation(
        &self,
        path: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_LatestOutputLocation")
    }
    fn put_LatestOutputLocation(
        &self,
        path: super::super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("put_LatestOutputLocation")
    }
    fn get_Name(&self, name: MutPtr<super::super::Foundation::BSTR>) -> crate::core::HRESULT {
        todo!("get_Name")
    }
    fn get_OutputLocation(
        &self,
        path: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_OutputLocation")
    }
    fn get_RootPath(&self, folder: MutPtr<super::super::Foundation::BSTR>) -> crate::core::HRESULT {
        todo!("get_RootPath")
    }
    fn put_RootPath(&self, folder: super::super::Foundation::BSTR) -> crate::core::HRESULT {
        todo!("put_RootPath")
    }
    fn get_Segment(&self, segment: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_Segment")
    }
    fn put_Segment(&self, segment: i16) -> crate::core::HRESULT {
        todo!("put_Segment")
    }
    fn get_SegmentMaxDuration(&self, seconds: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_SegmentMaxDuration")
    }
    fn put_SegmentMaxDuration(&self, seconds: u32) -> crate::core::HRESULT {
        todo!("put_SegmentMaxDuration")
    }
    fn get_SegmentMaxSize(&self, size: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_SegmentMaxSize")
    }
    fn put_SegmentMaxSize(&self, size: u32) -> crate::core::HRESULT {
        todo!("put_SegmentMaxSize")
    }
    fn get_SerialNumber(&self, index: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_SerialNumber")
    }
    fn put_SerialNumber(&self, index: u32) -> crate::core::HRESULT {
        todo!("put_SerialNumber")
    }
    fn get_Server(&self, server: MutPtr<super::super::Foundation::BSTR>) -> crate::core::HRESULT {
        todo!("get_Server")
    }
    fn get_Status(&self, status: MutPtr<DataCollectorSetStatus>) -> crate::core::HRESULT {
        todo!("get_Status")
    }
    fn get_Subdirectory(
        &self,
        folder: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_Subdirectory")
    }
    fn put_Subdirectory(&self, folder: super::super::Foundation::BSTR) -> crate::core::HRESULT {
        todo!("put_Subdirectory")
    }
    fn get_SubdirectoryFormat(&self, format: MutPtr<AutoPathFormat>) -> crate::core::HRESULT {
        todo!("get_SubdirectoryFormat")
    }
    fn put_SubdirectoryFormat(&self, format: AutoPathFormat) -> crate::core::HRESULT {
        todo!("put_SubdirectoryFormat")
    }
    fn get_SubdirectoryFormatPattern(
        &self,
        pattern: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_SubdirectoryFormatPattern")
    }
    fn put_SubdirectoryFormatPattern(
        &self,
        pattern: super::super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("put_SubdirectoryFormatPattern")
    }
    fn get_Task(&self, task: MutPtr<super::super::Foundation::BSTR>) -> crate::core::HRESULT {
        todo!("get_Task")
    }
    fn put_Task(&self, task: super::super::Foundation::BSTR) -> crate::core::HRESULT {
        todo!("put_Task")
    }
    fn get_TaskRunAsSelf(&self, run_as_self: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_TaskRunAsSelf")
    }
    fn put_TaskRunAsSelf(&self, run_as_self: i16) -> crate::core::HRESULT {
        todo!("put_TaskRunAsSelf")
    }
    fn get_TaskArguments(
        &self,
        task: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_TaskArguments")
    }
    fn put_TaskArguments(&self, task: super::super::Foundation::BSTR) -> crate::core::HRESULT {
        todo!("put_TaskArguments")
    }
    fn get_TaskUserTextArguments(
        &self,
        user_text: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_TaskUserTextArguments")
    }
    fn put_TaskUserTextArguments(
        &self,
        user_text: super::super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("put_TaskUserTextArguments")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_Schedules(&self, pp_schedules: MutPtr<IScheduleCollection>) -> crate::core::HRESULT {
        todo!("get_Schedules")
    }
    fn get_SchedulesEnabled(&self, enabled: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_SchedulesEnabled")
    }
    fn put_SchedulesEnabled(&self, enabled: i16) -> crate::core::HRESULT {
        todo!("put_SchedulesEnabled")
    }
    fn get_UserAccount(
        &self,
        user: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_UserAccount")
    }
    fn get_Xml(&self, xml: MutPtr<super::super::Foundation::BSTR>) -> crate::core::HRESULT {
        todo!("get_Xml")
    }
    fn get_Security(
        &self,
        pbstr_security: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_Security")
    }
    fn put_Security(&self, bstr_security: super::super::Foundation::BSTR) -> crate::core::HRESULT {
        todo!("put_Security")
    }
    fn get_StopOnCompletion(&self, stop: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_StopOnCompletion")
    }
    fn put_StopOnCompletion(&self, stop: i16) -> crate::core::HRESULT {
        todo!("put_StopOnCompletion")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_DataManager(&self, data_manager: MutPtr<IDataManager>) -> crate::core::HRESULT {
        todo!("get_DataManager")
    }
    fn SetCredentials(
        &self,
        user: super::super::Foundation::BSTR,
        password: super::super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("SetCredentials")
    }
    fn Query(
        &self,
        name: super::super::Foundation::BSTR,
        server: super::super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("Query")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Commit(
        &self,
        name: super::super::Foundation::BSTR,
        server: super::super::Foundation::BSTR,
        mode: CommitMode,
        validation: MutPtr<IValueMap>,
    ) -> crate::core::HRESULT {
        todo!("Commit")
    }
    fn Delete(&self) -> crate::core::HRESULT {
        todo!("Delete")
    }
    fn Start(&self, synchronous: i16) -> crate::core::HRESULT {
        todo!("Start")
    }
    fn Stop(&self, synchronous: i16) -> crate::core::HRESULT {
        todo!("Stop")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn SetXml(
        &self,
        xml: super::super::Foundation::BSTR,
        validation: MutPtr<IValueMap>,
    ) -> crate::core::HRESULT {
        todo!("SetXml")
    }
    fn SetValue(
        &self,
        key: super::super::Foundation::BSTR,
        value: super::super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("SetValue")
    }
    fn GetValue(
        &self,
        key: super::super::Foundation::BSTR,
        value: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetValue")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IDataCollectorSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IDataCollectorSet {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IDataCollectorSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IDataCollectorSet {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IDataCollectorSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataCollectorSet").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IDataCollectorSet {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IDataCollectorSet {
    type Super = super::Com::IDispatch;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x03837520_098b_11d8_9414_505054503030);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IDataCollectorSetCollection(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IDataCollectorSetCollection_Trait: super::Com::IDispatch_Trait {
    fn get_Count(&self, ret_val: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("get_Count")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_Item(
        &self,
        index: super::Com::VARIANT,
        set: MutPtr<IDataCollectorSet>,
    ) -> crate::core::HRESULT {
        todo!("get_Item")
    }
    fn get__NewEnum(&self, ret_val: MutPtr<crate::core::IUnknown>) -> crate::core::HRESULT {
        todo!("get__NewEnum")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Add(&self, set: IDataCollectorSet) -> crate::core::HRESULT {
        todo!("Add")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Remove(&self, set: super::Com::VARIANT) -> crate::core::HRESULT {
        todo!("Remove")
    }
    fn Clear(&self) -> crate::core::HRESULT {
        todo!("Clear")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn AddRange(&self, sets: IDataCollectorSetCollection) -> crate::core::HRESULT {
        todo!("AddRange")
    }
    fn GetDataCollectorSets(
        &self,
        server: super::super::Foundation::BSTR,
        filter: super::super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("GetDataCollectorSets")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IDataCollectorSetCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IDataCollectorSetCollection {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IDataCollectorSetCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IDataCollectorSetCollection {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IDataCollectorSetCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataCollectorSetCollection")
            .field(&self.0)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IDataCollectorSetCollection {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IDataCollectorSetCollection {
    type Super = super::Com::IDispatch;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x03837524_098b_11d8_9414_505054503030);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IDataManager(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IDataManager_Trait: super::Com::IDispatch_Trait {
    fn get_Enabled(&self, pf_enabled: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_Enabled")
    }
    fn put_Enabled(&self, f_enabled: i16) -> crate::core::HRESULT {
        todo!("put_Enabled")
    }
    fn get_CheckBeforeRunning(&self, pf_check: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_CheckBeforeRunning")
    }
    fn put_CheckBeforeRunning(&self, f_check: i16) -> crate::core::HRESULT {
        todo!("put_CheckBeforeRunning")
    }
    fn get_MinFreeDisk(&self, min_free_disk: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_MinFreeDisk")
    }
    fn put_MinFreeDisk(&self, min_free_disk: u32) -> crate::core::HRESULT {
        todo!("put_MinFreeDisk")
    }
    fn get_MaxSize(&self, pul_max_size: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_MaxSize")
    }
    fn put_MaxSize(&self, ul_max_size: u32) -> crate::core::HRESULT {
        todo!("put_MaxSize")
    }
    fn get_MaxFolderCount(&self, pul_max_folder_count: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_MaxFolderCount")
    }
    fn put_MaxFolderCount(&self, ul_max_folder_count: u32) -> crate::core::HRESULT {
        todo!("put_MaxFolderCount")
    }
    fn get_ResourcePolicy(&self, p_policy: MutPtr<ResourcePolicy>) -> crate::core::HRESULT {
        todo!("get_ResourcePolicy")
    }
    fn put_ResourcePolicy(&self, policy: ResourcePolicy) -> crate::core::HRESULT {
        todo!("put_ResourcePolicy")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_FolderActions(&self, actions: MutPtr<IFolderActionCollection>) -> crate::core::HRESULT {
        todo!("get_FolderActions")
    }
    fn get_ReportSchema(
        &self,
        report_schema: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_ReportSchema")
    }
    fn put_ReportSchema(
        &self,
        report_schema: super::super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("put_ReportSchema")
    }
    fn get_ReportFileName(
        &self,
        pbstr_filename: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_ReportFileName")
    }
    fn put_ReportFileName(
        &self,
        pbstr_filename: super::super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("put_ReportFileName")
    }
    fn get_RuleTargetFileName(
        &self,
        filename: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_RuleTargetFileName")
    }
    fn put_RuleTargetFileName(
        &self,
        filename: super::super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("put_RuleTargetFileName")
    }
    fn get_EventsFileName(
        &self,
        pbstr_filename: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_EventsFileName")
    }
    fn put_EventsFileName(
        &self,
        pbstr_filename: super::super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("put_EventsFileName")
    }
    fn get_Rules(&self, pbstr_xml: MutPtr<super::super::Foundation::BSTR>) -> crate::core::HRESULT {
        todo!("get_Rules")
    }
    fn put_Rules(&self, bstr_xml: super::super::Foundation::BSTR) -> crate::core::HRESULT {
        todo!("put_Rules")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Run(
        &self,
        steps: DataManagerSteps,
        bstr_folder: super::super::Foundation::BSTR,
        errors: MutPtr<IValueMap>,
    ) -> crate::core::HRESULT {
        todo!("Run")
    }
    fn Extract(
        &self,
        cab_filename: super::super::Foundation::BSTR,
        destination_path: super::super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("Extract")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IDataManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IDataManager {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IDataManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IDataManager {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IDataManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataManager").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IDataManager {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IDataManager {
    type Super = super::Com::IDispatch;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x03837541_098b_11d8_9414_505054503030);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IFolderAction(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IFolderAction_Trait: super::Com::IDispatch_Trait {
    fn get_Age(&self, pul_age: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_Age")
    }
    fn put_Age(&self, ul_age: u32) -> crate::core::HRESULT {
        todo!("put_Age")
    }
    fn get_Size(&self, pul_age: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_Size")
    }
    fn put_Size(&self, ul_age: u32) -> crate::core::HRESULT {
        todo!("put_Size")
    }
    fn get_Actions(&self, steps: MutPtr<FolderActionSteps>) -> crate::core::HRESULT {
        todo!("get_Actions")
    }
    fn put_Actions(&self, steps: FolderActionSteps) -> crate::core::HRESULT {
        todo!("put_Actions")
    }
    fn get_SendCabTo(
        &self,
        pbstr_destination: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_SendCabTo")
    }
    fn put_SendCabTo(
        &self,
        bstr_destination: super::super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("put_SendCabTo")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IFolderAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IFolderAction {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IFolderAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IFolderAction {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IFolderAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFolderAction").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IFolderAction {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IFolderAction {
    type Super = super::Com::IDispatch;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x03837543_098b_11d8_9414_505054503030);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IFolderActionCollection(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IFolderActionCollection_Trait: super::Com::IDispatch_Trait {
    fn get_Count(&self, count: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_Count")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_Item(
        &self,
        index: super::Com::VARIANT,
        action: MutPtr<IFolderAction>,
    ) -> crate::core::HRESULT {
        todo!("get_Item")
    }
    fn get__NewEnum(&self, r#enum: MutPtr<crate::core::IUnknown>) -> crate::core::HRESULT {
        todo!("get__NewEnum")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Add(&self, action: IFolderAction) -> crate::core::HRESULT {
        todo!("Add")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Remove(&self, index: super::Com::VARIANT) -> crate::core::HRESULT {
        todo!("Remove")
    }
    fn Clear(&self) -> crate::core::HRESULT {
        todo!("Clear")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn AddRange(&self, actions: IFolderActionCollection) -> crate::core::HRESULT {
        todo!("AddRange")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateFolderAction(&self, folder_action: MutPtr<IFolderAction>) -> crate::core::HRESULT {
        todo!("CreateFolderAction")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IFolderActionCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IFolderActionCollection {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IFolderActionCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IFolderActionCollection {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IFolderActionCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFolderActionCollection")
            .field(&self.0)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IFolderActionCollection {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IFolderActionCollection {
    type Super = super::Com::IDispatch;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x03837544_098b_11d8_9414_505054503030);
}
pub struct ILogFileItem(pub crate::core::IUnknown);
pub trait ILogFileItem_Trait: crate::core::IUnknown_Trait {
    fn get_Path(&self, pstr_value: MutPtr<super::super::Foundation::BSTR>) -> crate::core::HRESULT {
        todo!("get_Path")
    }
}
impl ::core::clone::Clone for ILogFileItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ILogFileItem {}
impl ::core::cmp::PartialEq for ILogFileItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILogFileItem {}
impl ::core::fmt::Debug for ILogFileItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILogFileItem").field(&self.0).finish()
    }
}
impl FromIntoMemory for ILogFileItem {
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
impl crate::core::ComInterface for ILogFileItem {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xd6b518dd_05c7_418a_89e6_4f9ce8c6841e);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ILogFiles(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait ILogFiles_Trait: super::Com::IDispatch_Trait {
    fn get_Count(&self, p_long: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("get_Count")
    }
    fn get__NewEnum(&self, pp_iunk: MutPtr<crate::core::IUnknown>) -> crate::core::HRESULT {
        todo!("get__NewEnum")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_Item(
        &self,
        index: super::Com::VARIANT,
        pp_i: MutPtr<DILogFileItem>,
    ) -> crate::core::HRESULT {
        todo!("get_Item")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Add(
        &self,
        pathname: super::super::Foundation::BSTR,
        pp_i: MutPtr<DILogFileItem>,
    ) -> crate::core::HRESULT {
        todo!("Add")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Remove(&self, index: super::Com::VARIANT) -> crate::core::HRESULT {
        todo!("Remove")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ILogFiles {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ILogFiles {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ILogFiles {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ILogFiles {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ILogFiles {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILogFiles").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ILogFiles {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for ILogFiles {
    type Super = super::Com::IDispatch;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x6a2a97e6_6851_41ea_87ad_2a8225335865);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IPerformanceCounterDataCollector(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IPerformanceCounterDataCollector_Trait: IDataCollector_Trait {
    fn get_DataSourceName(
        &self,
        dsn: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_DataSourceName")
    }
    fn put_DataSourceName(&self, dsn: super::super::Foundation::BSTR) -> crate::core::HRESULT {
        todo!("put_DataSourceName")
    }
    fn get_PerformanceCounters(
        &self,
        counters: MutPtr<ConstPtr<super::Com::SAFEARRAY>>,
    ) -> crate::core::HRESULT {
        todo!("get_PerformanceCounters")
    }
    fn put_PerformanceCounters(
        &self,
        counters: ConstPtr<super::Com::SAFEARRAY>,
    ) -> crate::core::HRESULT {
        todo!("put_PerformanceCounters")
    }
    fn get_LogFileFormat(&self, format: MutPtr<FileFormat>) -> crate::core::HRESULT {
        todo!("get_LogFileFormat")
    }
    fn put_LogFileFormat(&self, format: FileFormat) -> crate::core::HRESULT {
        todo!("put_LogFileFormat")
    }
    fn get_SampleInterval(&self, interval: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_SampleInterval")
    }
    fn put_SampleInterval(&self, interval: u32) -> crate::core::HRESULT {
        todo!("put_SampleInterval")
    }
    fn get_SegmentMaxRecords(&self, records: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_SegmentMaxRecords")
    }
    fn put_SegmentMaxRecords(&self, records: u32) -> crate::core::HRESULT {
        todo!("put_SegmentMaxRecords")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IPerformanceCounterDataCollector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IPerformanceCounterDataCollector {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IPerformanceCounterDataCollector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IPerformanceCounterDataCollector {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IPerformanceCounterDataCollector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPerformanceCounterDataCollector")
            .field(&self.0)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IPerformanceCounterDataCollector {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IPerformanceCounterDataCollector {
    type Super = IDataCollector;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x03837506_098b_11d8_9414_505054503030);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ISchedule(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait ISchedule_Trait: super::Com::IDispatch_Trait {
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_StartDate(&self, start: MutPtr<super::Com::VARIANT>) -> crate::core::HRESULT {
        todo!("get_StartDate")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn put_StartDate(&self, start: super::Com::VARIANT) -> crate::core::HRESULT {
        todo!("put_StartDate")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_EndDate(&self, end: MutPtr<super::Com::VARIANT>) -> crate::core::HRESULT {
        todo!("get_EndDate")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn put_EndDate(&self, end: super::Com::VARIANT) -> crate::core::HRESULT {
        todo!("put_EndDate")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_StartTime(&self, start: MutPtr<super::Com::VARIANT>) -> crate::core::HRESULT {
        todo!("get_StartTime")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn put_StartTime(&self, start: super::Com::VARIANT) -> crate::core::HRESULT {
        todo!("put_StartTime")
    }
    fn get_Days(&self, days: MutPtr<WeekDays>) -> crate::core::HRESULT {
        todo!("get_Days")
    }
    fn put_Days(&self, days: WeekDays) -> crate::core::HRESULT {
        todo!("put_Days")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ISchedule {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ISchedule {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ISchedule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ISchedule {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ISchedule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISchedule").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ISchedule {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for ISchedule {
    type Super = super::Com::IDispatch;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x0383753a_098b_11d8_9414_505054503030);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IScheduleCollection(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IScheduleCollection_Trait: super::Com::IDispatch_Trait {
    fn get_Count(&self, ret_val: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("get_Count")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_Item(
        &self,
        index: super::Com::VARIANT,
        pp_schedule: MutPtr<ISchedule>,
    ) -> crate::core::HRESULT {
        todo!("get_Item")
    }
    fn get__NewEnum(&self, ienum: MutPtr<crate::core::IUnknown>) -> crate::core::HRESULT {
        todo!("get__NewEnum")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Add(&self, p_schedule: ISchedule) -> crate::core::HRESULT {
        todo!("Add")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Remove(&self, v_schedule: super::Com::VARIANT) -> crate::core::HRESULT {
        todo!("Remove")
    }
    fn Clear(&self) -> crate::core::HRESULT {
        todo!("Clear")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn AddRange(&self, p_schedules: IScheduleCollection) -> crate::core::HRESULT {
        todo!("AddRange")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateSchedule(&self, schedule: MutPtr<ISchedule>) -> crate::core::HRESULT {
        todo!("CreateSchedule")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IScheduleCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IScheduleCollection {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IScheduleCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IScheduleCollection {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IScheduleCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IScheduleCollection").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IScheduleCollection {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IScheduleCollection {
    type Super = super::Com::IDispatch;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x0383753d_098b_11d8_9414_505054503030);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ISystemMonitor(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait ISystemMonitor_Trait: crate::core::IUnknown_Trait {
    fn get_Appearance(&self, i_appearance: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("get_Appearance")
    }
    fn put_Appearance(&self, i_appearance: i32) -> crate::core::HRESULT {
        todo!("put_Appearance")
    }
    fn get_BackColor(&self, p_color: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_BackColor")
    }
    fn put_BackColor(&self, color: u32) -> crate::core::HRESULT {
        todo!("put_BackColor")
    }
    fn get_BorderStyle(&self, i_border_style: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("get_BorderStyle")
    }
    fn put_BorderStyle(&self, i_border_style: i32) -> crate::core::HRESULT {
        todo!("put_BorderStyle")
    }
    fn get_ForeColor(&self, p_color: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_ForeColor")
    }
    fn put_ForeColor(&self, color: u32) -> crate::core::HRESULT {
        todo!("put_ForeColor")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_Font(&self, pp_font: MutPtr<super::Ole::IFontDisp>) -> crate::core::HRESULT {
        todo!("get_Font")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn putref_Font(&self, p_font: super::Ole::IFontDisp) -> crate::core::HRESULT {
        todo!("putref_Font")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_Counters(&self, pp_i_counters: MutPtr<ICounters>) -> crate::core::HRESULT {
        todo!("get_Counters")
    }
    fn put_ShowVerticalGrid(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_ShowVerticalGrid")
    }
    fn get_ShowVerticalGrid(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_ShowVerticalGrid")
    }
    fn put_ShowHorizontalGrid(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_ShowHorizontalGrid")
    }
    fn get_ShowHorizontalGrid(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_ShowHorizontalGrid")
    }
    fn put_ShowLegend(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_ShowLegend")
    }
    fn get_ShowLegend(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_ShowLegend")
    }
    fn put_ShowScaleLabels(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_ShowScaleLabels")
    }
    fn get_ShowScaleLabels(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_ShowScaleLabels")
    }
    fn put_ShowValueBar(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_ShowValueBar")
    }
    fn get_ShowValueBar(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_ShowValueBar")
    }
    fn put_MaximumScale(&self, i_value: i32) -> crate::core::HRESULT {
        todo!("put_MaximumScale")
    }
    fn get_MaximumScale(&self, pi_value: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("get_MaximumScale")
    }
    fn put_MinimumScale(&self, i_value: i32) -> crate::core::HRESULT {
        todo!("put_MinimumScale")
    }
    fn get_MinimumScale(&self, pi_value: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("get_MinimumScale")
    }
    fn put_UpdateInterval(&self, f_value: f32) -> crate::core::HRESULT {
        todo!("put_UpdateInterval")
    }
    fn get_UpdateInterval(&self, pf_value: MutPtr<f32>) -> crate::core::HRESULT {
        todo!("get_UpdateInterval")
    }
    fn put_DisplayType(&self, e_display_type: DisplayTypeConstants) -> crate::core::HRESULT {
        todo!("put_DisplayType")
    }
    fn get_DisplayType(
        &self,
        pe_display_type: MutPtr<DisplayTypeConstants>,
    ) -> crate::core::HRESULT {
        todo!("get_DisplayType")
    }
    fn put_ManualUpdate(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_ManualUpdate")
    }
    fn get_ManualUpdate(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_ManualUpdate")
    }
    fn put_GraphTitle(&self, bs_title: super::super::Foundation::BSTR) -> crate::core::HRESULT {
        todo!("put_GraphTitle")
    }
    fn get_GraphTitle(
        &self,
        pbs_title: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_GraphTitle")
    }
    fn put_YAxisLabel(&self, bs_title: super::super::Foundation::BSTR) -> crate::core::HRESULT {
        todo!("put_YAxisLabel")
    }
    fn get_YAxisLabel(
        &self,
        pbs_title: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_YAxisLabel")
    }
    fn CollectSample(&self) -> crate::core::HRESULT {
        todo!("CollectSample")
    }
    fn UpdateGraph(&self) -> crate::core::HRESULT {
        todo!("UpdateGraph")
    }
    fn BrowseCounters(&self) -> crate::core::HRESULT {
        todo!("BrowseCounters")
    }
    fn DisplayProperties(&self) -> crate::core::HRESULT {
        todo!("DisplayProperties")
    }
    fn Counter(&self, i_index: i32, pp_i_counter: MutPtr<ICounterItem>) -> crate::core::HRESULT {
        todo!("Counter")
    }
    fn AddCounter(
        &self,
        bs_path: super::super::Foundation::BSTR,
        pp_i_counter: MutPtr<ICounterItem>,
    ) -> crate::core::HRESULT {
        todo!("AddCounter")
    }
    fn DeleteCounter(&self, p_ctr: ICounterItem) -> crate::core::HRESULT {
        todo!("DeleteCounter")
    }
    fn get_BackColorCtl(&self, p_color: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_BackColorCtl")
    }
    fn put_BackColorCtl(&self, color: u32) -> crate::core::HRESULT {
        todo!("put_BackColorCtl")
    }
    fn put_LogFileName(
        &self,
        bs_file_name: super::super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("put_LogFileName")
    }
    fn get_LogFileName(
        &self,
        bs_file_name: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_LogFileName")
    }
    fn put_LogViewStart(&self, start_time: f64) -> crate::core::HRESULT {
        todo!("put_LogViewStart")
    }
    fn get_LogViewStart(&self, start_time: MutPtr<f64>) -> crate::core::HRESULT {
        todo!("get_LogViewStart")
    }
    fn put_LogViewStop(&self, stop_time: f64) -> crate::core::HRESULT {
        todo!("put_LogViewStop")
    }
    fn get_LogViewStop(&self, stop_time: MutPtr<f64>) -> crate::core::HRESULT {
        todo!("get_LogViewStop")
    }
    fn get_GridColor(&self, p_color: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_GridColor")
    }
    fn put_GridColor(&self, color: u32) -> crate::core::HRESULT {
        todo!("put_GridColor")
    }
    fn get_TimeBarColor(&self, p_color: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_TimeBarColor")
    }
    fn put_TimeBarColor(&self, color: u32) -> crate::core::HRESULT {
        todo!("put_TimeBarColor")
    }
    fn get_Highlight(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_Highlight")
    }
    fn put_Highlight(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_Highlight")
    }
    fn get_ShowToolbar(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_ShowToolbar")
    }
    fn put_ShowToolbar(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_ShowToolbar")
    }
    fn Paste(&self) -> crate::core::HRESULT {
        todo!("Paste")
    }
    fn Copy(&self) -> crate::core::HRESULT {
        todo!("Copy")
    }
    fn Reset(&self) -> crate::core::HRESULT {
        todo!("Reset")
    }
    fn put_ReadOnly(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_ReadOnly")
    }
    fn get_ReadOnly(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_ReadOnly")
    }
    fn put_ReportValueType(
        &self,
        e_report_value_type: ReportValueTypeConstants,
    ) -> crate::core::HRESULT {
        todo!("put_ReportValueType")
    }
    fn get_ReportValueType(
        &self,
        pe_report_value_type: MutPtr<ReportValueTypeConstants>,
    ) -> crate::core::HRESULT {
        todo!("get_ReportValueType")
    }
    fn put_MonitorDuplicateInstances(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_MonitorDuplicateInstances")
    }
    fn get_MonitorDuplicateInstances(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_MonitorDuplicateInstances")
    }
    fn put_DisplayFilter(&self, i_value: i32) -> crate::core::HRESULT {
        todo!("put_DisplayFilter")
    }
    fn get_DisplayFilter(&self, pi_value: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("get_DisplayFilter")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_LogFiles(&self, pp_i_log_files: MutPtr<ILogFiles>) -> crate::core::HRESULT {
        todo!("get_LogFiles")
    }
    fn put_DataSourceType(
        &self,
        e_data_source_type: DataSourceTypeConstants,
    ) -> crate::core::HRESULT {
        todo!("put_DataSourceType")
    }
    fn get_DataSourceType(
        &self,
        pe_data_source_type: MutPtr<DataSourceTypeConstants>,
    ) -> crate::core::HRESULT {
        todo!("get_DataSourceType")
    }
    fn put_SqlDsnName(
        &self,
        bs_sql_dsn_name: super::super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("put_SqlDsnName")
    }
    fn get_SqlDsnName(
        &self,
        bs_sql_dsn_name: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_SqlDsnName")
    }
    fn put_SqlLogSetName(
        &self,
        bs_sql_log_set_name: super::super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("put_SqlLogSetName")
    }
    fn get_SqlLogSetName(
        &self,
        bs_sql_log_set_name: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_SqlLogSetName")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ISystemMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ISystemMonitor {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ISystemMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ISystemMonitor {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ISystemMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISystemMonitor").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ISystemMonitor {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for ISystemMonitor {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x194eb241_c32c_11cf_9398_00aa00a3ddea);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ISystemMonitor2(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait ISystemMonitor2_Trait: ISystemMonitor_Trait {
    fn put_EnableDigitGrouping(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_EnableDigitGrouping")
    }
    fn get_EnableDigitGrouping(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_EnableDigitGrouping")
    }
    fn put_EnableToolTips(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_EnableToolTips")
    }
    fn get_EnableToolTips(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_EnableToolTips")
    }
    fn put_ShowTimeAxisLabels(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_ShowTimeAxisLabels")
    }
    fn get_ShowTimeAxisLabels(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_ShowTimeAxisLabels")
    }
    fn put_ChartScroll(&self, b_scroll: i16) -> crate::core::HRESULT {
        todo!("put_ChartScroll")
    }
    fn get_ChartScroll(&self, pb_scroll: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_ChartScroll")
    }
    fn put_DataPointCount(&self, i_new_count: i32) -> crate::core::HRESULT {
        todo!("put_DataPointCount")
    }
    fn get_DataPointCount(&self, pi_data_point_count: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("get_DataPointCount")
    }
    fn ScaleToFit(&self, b_selected_counters_only: i16) -> crate::core::HRESULT {
        todo!("ScaleToFit")
    }
    fn SaveAs(
        &self,
        bstr_file_name: super::super::Foundation::BSTR,
        e_sysmon_file_type: SysmonFileType,
    ) -> crate::core::HRESULT {
        todo!("SaveAs")
    }
    fn Relog(
        &self,
        bstr_file_name: super::super::Foundation::BSTR,
        e_sysmon_file_type: SysmonFileType,
        i_filter: i32,
    ) -> crate::core::HRESULT {
        todo!("Relog")
    }
    fn ClearData(&self) -> crate::core::HRESULT {
        todo!("ClearData")
    }
    fn get_LogSourceStartTime(&self, p_date: MutPtr<f64>) -> crate::core::HRESULT {
        todo!("get_LogSourceStartTime")
    }
    fn get_LogSourceStopTime(&self, p_date: MutPtr<f64>) -> crate::core::HRESULT {
        todo!("get_LogSourceStopTime")
    }
    fn SetLogViewRange(&self, start_time: f64, stop_time: f64) -> crate::core::HRESULT {
        todo!("SetLogViewRange")
    }
    fn GetLogViewRange(
        &self,
        start_time: MutPtr<f64>,
        stop_time: MutPtr<f64>,
    ) -> crate::core::HRESULT {
        todo!("GetLogViewRange")
    }
    fn BatchingLock(&self, f_lock: i16, e_batch_reason: SysmonBatchReason) -> crate::core::HRESULT {
        todo!("BatchingLock")
    }
    fn LoadSettings(
        &self,
        bstr_setting_file_name: super::super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("LoadSettings")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ISystemMonitor2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ISystemMonitor2 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ISystemMonitor2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ISystemMonitor2 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ISystemMonitor2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISystemMonitor2").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ISystemMonitor2 {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for ISystemMonitor2 {
    type Super = ISystemMonitor;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x08e3206a_5fd2_4fde_a8a5_8cb3b63d2677);
}
pub struct ISystemMonitorEvents(pub crate::core::IUnknown);
pub trait ISystemMonitorEvents_Trait: crate::core::IUnknown_Trait {
    fn OnCounterSelected(&self, index: i32) {
        todo!("OnCounterSelected")
    }
    fn OnCounterAdded(&self, index: i32) {
        todo!("OnCounterAdded")
    }
    fn OnCounterDeleted(&self, index: i32) {
        todo!("OnCounterDeleted")
    }
    fn OnSampleCollected(&self) {
        todo!("OnSampleCollected")
    }
    fn OnDblClick(&self, index: i32) {
        todo!("OnDblClick")
    }
}
impl ::core::clone::Clone for ISystemMonitorEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ISystemMonitorEvents {}
impl ::core::cmp::PartialEq for ISystemMonitorEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISystemMonitorEvents {}
impl ::core::fmt::Debug for ISystemMonitorEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISystemMonitorEvents")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ISystemMonitorEvents {
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
impl crate::core::ComInterface for ISystemMonitorEvents {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xee660ea0_4abd_11cf_943a_008029004347);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ITraceDataCollector(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait ITraceDataCollector_Trait: IDataCollector_Trait {
    fn get_BufferSize(&self, size: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_BufferSize")
    }
    fn put_BufferSize(&self, size: u32) -> crate::core::HRESULT {
        todo!("put_BufferSize")
    }
    fn get_BuffersLost(&self, buffers: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_BuffersLost")
    }
    fn put_BuffersLost(&self, buffers: u32) -> crate::core::HRESULT {
        todo!("put_BuffersLost")
    }
    fn get_BuffersWritten(&self, buffers: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_BuffersWritten")
    }
    fn put_BuffersWritten(&self, buffers: u32) -> crate::core::HRESULT {
        todo!("put_BuffersWritten")
    }
    fn get_ClockType(&self, clock: MutPtr<ClockType>) -> crate::core::HRESULT {
        todo!("get_ClockType")
    }
    fn put_ClockType(&self, clock: ClockType) -> crate::core::HRESULT {
        todo!("put_ClockType")
    }
    fn get_EventsLost(&self, events: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_EventsLost")
    }
    fn put_EventsLost(&self, events: u32) -> crate::core::HRESULT {
        todo!("put_EventsLost")
    }
    fn get_ExtendedModes(&self, mode: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_ExtendedModes")
    }
    fn put_ExtendedModes(&self, mode: u32) -> crate::core::HRESULT {
        todo!("put_ExtendedModes")
    }
    fn get_FlushTimer(&self, seconds: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_FlushTimer")
    }
    fn put_FlushTimer(&self, seconds: u32) -> crate::core::HRESULT {
        todo!("put_FlushTimer")
    }
    fn get_FreeBuffers(&self, buffers: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_FreeBuffers")
    }
    fn put_FreeBuffers(&self, buffers: u32) -> crate::core::HRESULT {
        todo!("put_FreeBuffers")
    }
    fn get_Guid(&self, guid: MutPtr<crate::core::GUID>) -> crate::core::HRESULT {
        todo!("get_Guid")
    }
    fn put_Guid(&self, guid: crate::core::GUID) -> crate::core::HRESULT {
        todo!("put_Guid")
    }
    fn get_IsKernelTrace(&self, kernel: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_IsKernelTrace")
    }
    fn get_MaximumBuffers(&self, buffers: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_MaximumBuffers")
    }
    fn put_MaximumBuffers(&self, buffers: u32) -> crate::core::HRESULT {
        todo!("put_MaximumBuffers")
    }
    fn get_MinimumBuffers(&self, buffers: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_MinimumBuffers")
    }
    fn put_MinimumBuffers(&self, buffers: u32) -> crate::core::HRESULT {
        todo!("put_MinimumBuffers")
    }
    fn get_NumberOfBuffers(&self, buffers: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_NumberOfBuffers")
    }
    fn put_NumberOfBuffers(&self, buffers: u32) -> crate::core::HRESULT {
        todo!("put_NumberOfBuffers")
    }
    fn get_PreallocateFile(&self, allocate: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_PreallocateFile")
    }
    fn put_PreallocateFile(&self, allocate: i16) -> crate::core::HRESULT {
        todo!("put_PreallocateFile")
    }
    fn get_ProcessMode(&self, process: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_ProcessMode")
    }
    fn put_ProcessMode(&self, process: i16) -> crate::core::HRESULT {
        todo!("put_ProcessMode")
    }
    fn get_RealTimeBuffersLost(&self, buffers: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_RealTimeBuffersLost")
    }
    fn put_RealTimeBuffersLost(&self, buffers: u32) -> crate::core::HRESULT {
        todo!("put_RealTimeBuffersLost")
    }
    fn get_SessionId(&self, id: MutPtr<u64>) -> crate::core::HRESULT {
        todo!("get_SessionId")
    }
    fn put_SessionId(&self, id: u64) -> crate::core::HRESULT {
        todo!("put_SessionId")
    }
    fn get_SessionName(
        &self,
        name: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_SessionName")
    }
    fn put_SessionName(&self, name: super::super::Foundation::BSTR) -> crate::core::HRESULT {
        todo!("put_SessionName")
    }
    fn get_SessionThreadId(&self, tid: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_SessionThreadId")
    }
    fn put_SessionThreadId(&self, tid: u32) -> crate::core::HRESULT {
        todo!("put_SessionThreadId")
    }
    fn get_StreamMode(&self, mode: MutPtr<StreamMode>) -> crate::core::HRESULT {
        todo!("get_StreamMode")
    }
    fn put_StreamMode(&self, mode: StreamMode) -> crate::core::HRESULT {
        todo!("put_StreamMode")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_TraceDataProviders(
        &self,
        providers: MutPtr<ITraceDataProviderCollection>,
    ) -> crate::core::HRESULT {
        todo!("get_TraceDataProviders")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ITraceDataCollector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ITraceDataCollector {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ITraceDataCollector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ITraceDataCollector {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ITraceDataCollector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITraceDataCollector").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ITraceDataCollector {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for ITraceDataCollector {
    type Super = IDataCollector;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x0383750b_098b_11d8_9414_505054503030);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ITraceDataProvider(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait ITraceDataProvider_Trait: super::Com::IDispatch_Trait {
    fn get_DisplayName(
        &self,
        name: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_DisplayName")
    }
    fn put_DisplayName(&self, name: super::super::Foundation::BSTR) -> crate::core::HRESULT {
        todo!("put_DisplayName")
    }
    fn get_Guid(&self, guid: MutPtr<crate::core::GUID>) -> crate::core::HRESULT {
        todo!("get_Guid")
    }
    fn put_Guid(&self, guid: crate::core::GUID) -> crate::core::HRESULT {
        todo!("put_Guid")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_Level(&self, pp_level: MutPtr<IValueMap>) -> crate::core::HRESULT {
        todo!("get_Level")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_KeywordsAny(&self, pp_keywords: MutPtr<IValueMap>) -> crate::core::HRESULT {
        todo!("get_KeywordsAny")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_KeywordsAll(&self, pp_keywords: MutPtr<IValueMap>) -> crate::core::HRESULT {
        todo!("get_KeywordsAll")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_Properties(&self, pp_properties: MutPtr<IValueMap>) -> crate::core::HRESULT {
        todo!("get_Properties")
    }
    fn get_FilterEnabled(&self, filter_enabled: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_FilterEnabled")
    }
    fn put_FilterEnabled(&self, filter_enabled: i16) -> crate::core::HRESULT {
        todo!("put_FilterEnabled")
    }
    fn get_FilterType(&self, pul_type: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_FilterType")
    }
    fn put_FilterType(&self, ul_type: u32) -> crate::core::HRESULT {
        todo!("put_FilterType")
    }
    fn get_FilterData(
        &self,
        pp_data: MutPtr<ConstPtr<super::Com::SAFEARRAY>>,
    ) -> crate::core::HRESULT {
        todo!("get_FilterData")
    }
    fn put_FilterData(&self, p_data: ConstPtr<super::Com::SAFEARRAY>) -> crate::core::HRESULT {
        todo!("put_FilterData")
    }
    fn Query(
        &self,
        bstr_name: super::super::Foundation::BSTR,
        bstr_server: super::super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("Query")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Resolve(&self, p_from: super::Com::IDispatch) -> crate::core::HRESULT {
        todo!("Resolve")
    }
    fn SetSecurity(&self, sddl: super::super::Foundation::BSTR) -> crate::core::HRESULT {
        todo!("SetSecurity")
    }
    fn GetSecurity(
        &self,
        security_info: u32,
        sddl: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetSecurity")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetRegisteredProcesses(&self, processes: MutPtr<IValueMap>) -> crate::core::HRESULT {
        todo!("GetRegisteredProcesses")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ITraceDataProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ITraceDataProvider {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ITraceDataProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ITraceDataProvider {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ITraceDataProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITraceDataProvider").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ITraceDataProvider {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for ITraceDataProvider {
    type Super = super::Com::IDispatch;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x03837512_098b_11d8_9414_505054503030);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ITraceDataProviderCollection(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait ITraceDataProviderCollection_Trait: super::Com::IDispatch_Trait {
    fn get_Count(&self, ret_val: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("get_Count")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_Item(
        &self,
        index: super::Com::VARIANT,
        pp_provider: MutPtr<ITraceDataProvider>,
    ) -> crate::core::HRESULT {
        todo!("get_Item")
    }
    fn get__NewEnum(&self, ret_val: MutPtr<crate::core::IUnknown>) -> crate::core::HRESULT {
        todo!("get__NewEnum")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Add(&self, p_provider: ITraceDataProvider) -> crate::core::HRESULT {
        todo!("Add")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Remove(&self, v_provider: super::Com::VARIANT) -> crate::core::HRESULT {
        todo!("Remove")
    }
    fn Clear(&self) -> crate::core::HRESULT {
        todo!("Clear")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn AddRange(&self, providers: ITraceDataProviderCollection) -> crate::core::HRESULT {
        todo!("AddRange")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateTraceDataProvider(
        &self,
        provider: MutPtr<ITraceDataProvider>,
    ) -> crate::core::HRESULT {
        todo!("CreateTraceDataProvider")
    }
    fn GetTraceDataProviders(
        &self,
        server: super::super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("GetTraceDataProviders")
    }
    fn GetTraceDataProvidersByProcess(
        &self,
        server: super::super::Foundation::BSTR,
        pid: u32,
    ) -> crate::core::HRESULT {
        todo!("GetTraceDataProvidersByProcess")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ITraceDataProviderCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ITraceDataProviderCollection {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ITraceDataProviderCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ITraceDataProviderCollection {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ITraceDataProviderCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITraceDataProviderCollection")
            .field(&self.0)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ITraceDataProviderCollection {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for ITraceDataProviderCollection {
    type Super = super::Com::IDispatch;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x03837510_098b_11d8_9414_505054503030);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IValueMap(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IValueMap_Trait: super::Com::IDispatch_Trait {
    fn get_Count(&self, ret_val: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("get_Count")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_Item(
        &self,
        index: super::Com::VARIANT,
        value: MutPtr<IValueMapItem>,
    ) -> crate::core::HRESULT {
        todo!("get_Item")
    }
    fn get__NewEnum(&self, ret_val: MutPtr<crate::core::IUnknown>) -> crate::core::HRESULT {
        todo!("get__NewEnum")
    }
    fn get_Description(
        &self,
        description: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_Description")
    }
    fn put_Description(&self, description: super::super::Foundation::BSTR) -> crate::core::HRESULT {
        todo!("put_Description")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_Value(&self, value: MutPtr<super::Com::VARIANT>) -> crate::core::HRESULT {
        todo!("get_Value")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn put_Value(&self, value: super::Com::VARIANT) -> crate::core::HRESULT {
        todo!("put_Value")
    }
    fn get_ValueMapType(&self, r#type: MutPtr<ValueMapType>) -> crate::core::HRESULT {
        todo!("get_ValueMapType")
    }
    fn put_ValueMapType(&self, r#type: ValueMapType) -> crate::core::HRESULT {
        todo!("put_ValueMapType")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Add(&self, value: super::Com::VARIANT) -> crate::core::HRESULT {
        todo!("Add")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Remove(&self, value: super::Com::VARIANT) -> crate::core::HRESULT {
        todo!("Remove")
    }
    fn Clear(&self) -> crate::core::HRESULT {
        todo!("Clear")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn AddRange(&self, map: IValueMap) -> crate::core::HRESULT {
        todo!("AddRange")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateValueMapItem(&self, item: MutPtr<IValueMapItem>) -> crate::core::HRESULT {
        todo!("CreateValueMapItem")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IValueMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IValueMap {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IValueMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IValueMap {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IValueMap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IValueMap").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IValueMap {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IValueMap {
    type Super = super::Com::IDispatch;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x03837534_098b_11d8_9414_505054503030);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IValueMapItem(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IValueMapItem_Trait: super::Com::IDispatch_Trait {
    fn get_Description(
        &self,
        description: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_Description")
    }
    fn put_Description(&self, description: super::super::Foundation::BSTR) -> crate::core::HRESULT {
        todo!("put_Description")
    }
    fn get_Enabled(&self, enabled: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_Enabled")
    }
    fn put_Enabled(&self, enabled: i16) -> crate::core::HRESULT {
        todo!("put_Enabled")
    }
    fn get_Key(&self, key: MutPtr<super::super::Foundation::BSTR>) -> crate::core::HRESULT {
        todo!("get_Key")
    }
    fn put_Key(&self, key: super::super::Foundation::BSTR) -> crate::core::HRESULT {
        todo!("put_Key")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_Value(&self, value: MutPtr<super::Com::VARIANT>) -> crate::core::HRESULT {
        todo!("get_Value")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn put_Value(&self, value: super::Com::VARIANT) -> crate::core::HRESULT {
        todo!("put_Value")
    }
    fn get_ValueMapType(&self, r#type: MutPtr<ValueMapType>) -> crate::core::HRESULT {
        todo!("get_ValueMapType")
    }
    fn put_ValueMapType(&self, r#type: ValueMapType) -> crate::core::HRESULT {
        todo!("put_ValueMapType")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IValueMapItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IValueMapItem {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IValueMapItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IValueMapItem {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IValueMapItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IValueMapItem").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IValueMapItem {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IValueMapItem {
    type Super = super::Com::IDispatch;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x03837533_098b_11d8_9414_505054503030);
}
pub const LIBID_SystemMonitor: crate::core::GUID =
    crate::core::GUID::from_u128(0x1b773e42_2509_11cf_942f_008029004347);
pub const LegacyDataCollectorSet: crate::core::GUID =
    crate::core::GUID::from_u128(0x03837526_098b_11d8_9414_505054503030);
pub const LegacyDataCollectorSetCollection: crate::core::GUID =
    crate::core::GUID::from_u128(0x03837527_098b_11d8_9414_505054503030);
pub const LegacyTraceSession: crate::core::GUID =
    crate::core::GUID::from_u128(0x03837528_098b_11d8_9414_505054503030);
pub const LegacyTraceSessionCollection: crate::core::GUID =
    crate::core::GUID::from_u128(0x03837529_098b_11d8_9414_505054503030);
pub const LogFileItem: crate::core::GUID =
    crate::core::GUID::from_u128(0x16ec5be8_df93_4237_94e4_9ee918111d71);
pub const LogFiles: crate::core::GUID =
    crate::core::GUID::from_u128(0x2735d9fd_f6b9_4f19_a5d9_e2d068584bc5);
pub const MAX_COUNTER_PATH: u32 = 256u32;
pub const MAX_PERF_OBJECTS_IN_QUERY_FUNCTION: i32 = 64i32;
pub const PDH_ACCESS_DENIED: i32 = -1073738789i32;
pub const PDH_ASYNC_QUERY_TIMEOUT: i32 = -2147481637i32;
pub const PDH_BINARY_LOG_CORRUPT: i32 = -1073738761i32;
pub struct PDH_BROWSE_DLG_CONFIG_A {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub szDataSource: PSTR,
    pub szReturnPathBuffer: PSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: PtrRepr,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: PSTR,
}
impl ::core::marker::Copy for PDH_BROWSE_DLG_CONFIG_A {}
impl ::core::clone::Clone for PDH_BROWSE_DLG_CONFIG_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_BROWSE_DLG_CONFIG_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_BROWSE_DLG_CONFIG_A")
            .field("_bitfield", &self._bitfield)
            .field("hWndOwner", &self.hWndOwner)
            .field("szDataSource", &self.szDataSource)
            .field("szReturnPathBuffer", &self.szReturnPathBuffer)
            .field("cchReturnPathLength", &self.cchReturnPathLength)
            .field("pCallBack", &self.pCallBack)
            .field("dwCallBackArg", &self.dwCallBackArg)
            .field("CallBackStatus", &self.CallBackStatus)
            .field("dwDefaultDetailLevel", &self.dwDefaultDetailLevel)
            .field("szDialogBoxCaption", &self.szDialogBoxCaption)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_BROWSE_DLG_CONFIG_A {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
            && self.hWndOwner == other.hWndOwner
            && self.szDataSource == other.szDataSource
            && self.szReturnPathBuffer == other.szReturnPathBuffer
            && self.cchReturnPathLength == other.cchReturnPathLength
            && self.pCallBack == other.pCallBack
            && self.dwCallBackArg == other.dwCallBackArg
            && self.CallBackStatus == other.CallBackStatus
            && self.dwDefaultDetailLevel == other.dwDefaultDetailLevel
            && self.szDialogBoxCaption == other.szDialogBoxCaption
    }
}
impl ::core::cmp::Eq for PDH_BROWSE_DLG_CONFIG_A {}
impl FromIntoMemory for PDH_BROWSE_DLG_CONFIG_A {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f__bitfield = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_hWndOwner =
            <super::super::Foundation::HWND as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szDataSource = <PSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_szReturnPathBuffer = <PSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_cchReturnPathLength = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_pCallBack = <CounterPathCallBack as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwCallBackArg = <PtrRepr as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_CallBackStatus = <i32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_dwDefaultDetailLevel = <PERF_DETAIL as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_szDialogBoxCaption = <PSTR as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        Self {
            _bitfield: f__bitfield,
            hWndOwner: f_hWndOwner,
            szDataSource: f_szDataSource,
            szReturnPathBuffer: f_szReturnPathBuffer,
            cchReturnPathLength: f_cchReturnPathLength,
            pCallBack: f_pCallBack,
            dwCallBackArg: f_dwCallBackArg,
            CallBackStatus: f_CallBackStatus,
            dwDefaultDetailLevel: f_dwDefaultDetailLevel,
            szDialogBoxCaption: f_szDialogBoxCaption,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self._bitfield, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.hWndOwner, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szDataSource, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.szReturnPathBuffer, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.cchReturnPathLength, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.pCallBack, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwCallBackArg, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.CallBackStatus, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.dwDefaultDetailLevel, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.szDialogBoxCaption, &mut into[36..36 + 4]);
    }
    fn size() -> usize {
        40
    }
}
pub struct PDH_BROWSE_DLG_CONFIG_HA {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub hDataSource: PtrDiffRepr,
    pub szReturnPathBuffer: PSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: PtrRepr,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: PSTR,
}
impl ::core::marker::Copy for PDH_BROWSE_DLG_CONFIG_HA {}
impl ::core::clone::Clone for PDH_BROWSE_DLG_CONFIG_HA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_BROWSE_DLG_CONFIG_HA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_BROWSE_DLG_CONFIG_HA")
            .field("_bitfield", &self._bitfield)
            .field("hWndOwner", &self.hWndOwner)
            .field("hDataSource", &self.hDataSource)
            .field("szReturnPathBuffer", &self.szReturnPathBuffer)
            .field("cchReturnPathLength", &self.cchReturnPathLength)
            .field("pCallBack", &self.pCallBack)
            .field("dwCallBackArg", &self.dwCallBackArg)
            .field("CallBackStatus", &self.CallBackStatus)
            .field("dwDefaultDetailLevel", &self.dwDefaultDetailLevel)
            .field("szDialogBoxCaption", &self.szDialogBoxCaption)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_BROWSE_DLG_CONFIG_HA {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
            && self.hWndOwner == other.hWndOwner
            && self.hDataSource == other.hDataSource
            && self.szReturnPathBuffer == other.szReturnPathBuffer
            && self.cchReturnPathLength == other.cchReturnPathLength
            && self.pCallBack == other.pCallBack
            && self.dwCallBackArg == other.dwCallBackArg
            && self.CallBackStatus == other.CallBackStatus
            && self.dwDefaultDetailLevel == other.dwDefaultDetailLevel
            && self.szDialogBoxCaption == other.szDialogBoxCaption
    }
}
impl ::core::cmp::Eq for PDH_BROWSE_DLG_CONFIG_HA {}
impl FromIntoMemory for PDH_BROWSE_DLG_CONFIG_HA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f__bitfield = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_hWndOwner =
            <super::super::Foundation::HWND as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_hDataSource = <PtrDiffRepr as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_szReturnPathBuffer = <PSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_cchReturnPathLength = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_pCallBack = <CounterPathCallBack as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwCallBackArg = <PtrRepr as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_CallBackStatus = <i32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_dwDefaultDetailLevel = <PERF_DETAIL as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_szDialogBoxCaption = <PSTR as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        Self {
            _bitfield: f__bitfield,
            hWndOwner: f_hWndOwner,
            hDataSource: f_hDataSource,
            szReturnPathBuffer: f_szReturnPathBuffer,
            cchReturnPathLength: f_cchReturnPathLength,
            pCallBack: f_pCallBack,
            dwCallBackArg: f_dwCallBackArg,
            CallBackStatus: f_CallBackStatus,
            dwDefaultDetailLevel: f_dwDefaultDetailLevel,
            szDialogBoxCaption: f_szDialogBoxCaption,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self._bitfield, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.hWndOwner, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.hDataSource, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.szReturnPathBuffer, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.cchReturnPathLength, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.pCallBack, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwCallBackArg, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.CallBackStatus, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.dwDefaultDetailLevel, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.szDialogBoxCaption, &mut into[36..36 + 4]);
    }
    fn size() -> usize {
        40
    }
}
pub struct PDH_BROWSE_DLG_CONFIG_HW {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub hDataSource: PtrDiffRepr,
    pub szReturnPathBuffer: PWSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: PtrRepr,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: PWSTR,
}
impl ::core::marker::Copy for PDH_BROWSE_DLG_CONFIG_HW {}
impl ::core::clone::Clone for PDH_BROWSE_DLG_CONFIG_HW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_BROWSE_DLG_CONFIG_HW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_BROWSE_DLG_CONFIG_HW")
            .field("_bitfield", &self._bitfield)
            .field("hWndOwner", &self.hWndOwner)
            .field("hDataSource", &self.hDataSource)
            .field("szReturnPathBuffer", &self.szReturnPathBuffer)
            .field("cchReturnPathLength", &self.cchReturnPathLength)
            .field("pCallBack", &self.pCallBack)
            .field("dwCallBackArg", &self.dwCallBackArg)
            .field("CallBackStatus", &self.CallBackStatus)
            .field("dwDefaultDetailLevel", &self.dwDefaultDetailLevel)
            .field("szDialogBoxCaption", &self.szDialogBoxCaption)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_BROWSE_DLG_CONFIG_HW {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
            && self.hWndOwner == other.hWndOwner
            && self.hDataSource == other.hDataSource
            && self.szReturnPathBuffer == other.szReturnPathBuffer
            && self.cchReturnPathLength == other.cchReturnPathLength
            && self.pCallBack == other.pCallBack
            && self.dwCallBackArg == other.dwCallBackArg
            && self.CallBackStatus == other.CallBackStatus
            && self.dwDefaultDetailLevel == other.dwDefaultDetailLevel
            && self.szDialogBoxCaption == other.szDialogBoxCaption
    }
}
impl ::core::cmp::Eq for PDH_BROWSE_DLG_CONFIG_HW {}
impl FromIntoMemory for PDH_BROWSE_DLG_CONFIG_HW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f__bitfield = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_hWndOwner =
            <super::super::Foundation::HWND as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_hDataSource = <PtrDiffRepr as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_szReturnPathBuffer = <PWSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_cchReturnPathLength = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_pCallBack = <CounterPathCallBack as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwCallBackArg = <PtrRepr as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_CallBackStatus = <i32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_dwDefaultDetailLevel = <PERF_DETAIL as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_szDialogBoxCaption = <PWSTR as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        Self {
            _bitfield: f__bitfield,
            hWndOwner: f_hWndOwner,
            hDataSource: f_hDataSource,
            szReturnPathBuffer: f_szReturnPathBuffer,
            cchReturnPathLength: f_cchReturnPathLength,
            pCallBack: f_pCallBack,
            dwCallBackArg: f_dwCallBackArg,
            CallBackStatus: f_CallBackStatus,
            dwDefaultDetailLevel: f_dwDefaultDetailLevel,
            szDialogBoxCaption: f_szDialogBoxCaption,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self._bitfield, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.hWndOwner, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.hDataSource, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.szReturnPathBuffer, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.cchReturnPathLength, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.pCallBack, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwCallBackArg, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.CallBackStatus, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.dwDefaultDetailLevel, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.szDialogBoxCaption, &mut into[36..36 + 4]);
    }
    fn size() -> usize {
        40
    }
}
pub struct PDH_BROWSE_DLG_CONFIG_W {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub szDataSource: PWSTR,
    pub szReturnPathBuffer: PWSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: PtrRepr,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: PWSTR,
}
impl ::core::marker::Copy for PDH_BROWSE_DLG_CONFIG_W {}
impl ::core::clone::Clone for PDH_BROWSE_DLG_CONFIG_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_BROWSE_DLG_CONFIG_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_BROWSE_DLG_CONFIG_W")
            .field("_bitfield", &self._bitfield)
            .field("hWndOwner", &self.hWndOwner)
            .field("szDataSource", &self.szDataSource)
            .field("szReturnPathBuffer", &self.szReturnPathBuffer)
            .field("cchReturnPathLength", &self.cchReturnPathLength)
            .field("pCallBack", &self.pCallBack)
            .field("dwCallBackArg", &self.dwCallBackArg)
            .field("CallBackStatus", &self.CallBackStatus)
            .field("dwDefaultDetailLevel", &self.dwDefaultDetailLevel)
            .field("szDialogBoxCaption", &self.szDialogBoxCaption)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_BROWSE_DLG_CONFIG_W {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
            && self.hWndOwner == other.hWndOwner
            && self.szDataSource == other.szDataSource
            && self.szReturnPathBuffer == other.szReturnPathBuffer
            && self.cchReturnPathLength == other.cchReturnPathLength
            && self.pCallBack == other.pCallBack
            && self.dwCallBackArg == other.dwCallBackArg
            && self.CallBackStatus == other.CallBackStatus
            && self.dwDefaultDetailLevel == other.dwDefaultDetailLevel
            && self.szDialogBoxCaption == other.szDialogBoxCaption
    }
}
impl ::core::cmp::Eq for PDH_BROWSE_DLG_CONFIG_W {}
impl FromIntoMemory for PDH_BROWSE_DLG_CONFIG_W {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f__bitfield = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_hWndOwner =
            <super::super::Foundation::HWND as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szDataSource = <PWSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_szReturnPathBuffer = <PWSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_cchReturnPathLength = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_pCallBack = <CounterPathCallBack as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwCallBackArg = <PtrRepr as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_CallBackStatus = <i32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_dwDefaultDetailLevel = <PERF_DETAIL as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_szDialogBoxCaption = <PWSTR as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        Self {
            _bitfield: f__bitfield,
            hWndOwner: f_hWndOwner,
            szDataSource: f_szDataSource,
            szReturnPathBuffer: f_szReturnPathBuffer,
            cchReturnPathLength: f_cchReturnPathLength,
            pCallBack: f_pCallBack,
            dwCallBackArg: f_dwCallBackArg,
            CallBackStatus: f_CallBackStatus,
            dwDefaultDetailLevel: f_dwDefaultDetailLevel,
            szDialogBoxCaption: f_szDialogBoxCaption,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self._bitfield, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.hWndOwner, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szDataSource, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.szReturnPathBuffer, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.cchReturnPathLength, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.pCallBack, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwCallBackArg, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.CallBackStatus, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.dwDefaultDetailLevel, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.szDialogBoxCaption, &mut into[36..36 + 4]);
    }
    fn size() -> usize {
        40
    }
}
pub const PDH_CALC_NEGATIVE_DENOMINATOR: i32 = -2147481642i32;
pub const PDH_CALC_NEGATIVE_TIMEBASE: i32 = -2147481641i32;
pub const PDH_CALC_NEGATIVE_VALUE: i32 = -2147481640i32;
pub const PDH_CANNOT_CONNECT_MACHINE: i32 = -1073738813i32;
pub const PDH_CANNOT_CONNECT_WMI_SERVER: i32 = -1073738776i32;
pub const PDH_CANNOT_READ_NAME_STRINGS: i32 = -1073738808i32;
pub const PDH_CANNOT_SET_DEFAULT_REALTIME_DATASOURCE: i32 = -2147481636i32;
pub const PDH_COUNTER_ALREADY_IN_QUERY: i32 = -1073738762i32;
pub struct PDH_COUNTER_INFO_A {
    pub dwLength: u32,
    pub dwType: u32,
    pub CVersion: u32,
    pub CStatus: u32,
    pub lScale: i32,
    pub lDefaultScale: i32,
    pub dwUserData: PtrRepr,
    pub dwQueryUserData: PtrRepr,
    pub szFullPath: PSTR,
    pub Anonymous: PDH_COUNTER_INFO_A_0,
    pub szExplainText: PSTR,
    pub DataBuffer: [u32; 1],
}
impl ::core::marker::Copy for PDH_COUNTER_INFO_A {}
impl ::core::clone::Clone for PDH_COUNTER_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_COUNTER_INFO_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_COUNTER_INFO_A")
            .field("dwLength", &self.dwLength)
            .field("dwType", &self.dwType)
            .field("CVersion", &self.CVersion)
            .field("CStatus", &self.CStatus)
            .field("lScale", &self.lScale)
            .field("lDefaultScale", &self.lDefaultScale)
            .field("dwUserData", &self.dwUserData)
            .field("dwQueryUserData", &self.dwQueryUserData)
            .field("szFullPath", &self.szFullPath)
            .field("Anonymous", &self.Anonymous)
            .field("szExplainText", &self.szExplainText)
            .field("DataBuffer", &self.DataBuffer)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_COUNTER_INFO_A {
    fn eq(&self, other: &Self) -> bool {
        self.dwLength == other.dwLength
            && self.dwType == other.dwType
            && self.CVersion == other.CVersion
            && self.CStatus == other.CStatus
            && self.lScale == other.lScale
            && self.lDefaultScale == other.lDefaultScale
            && self.dwUserData == other.dwUserData
            && self.dwQueryUserData == other.dwQueryUserData
            && self.szFullPath == other.szFullPath
            && self.Anonymous == other.Anonymous
            && self.szExplainText == other.szExplainText
            && self.DataBuffer == other.DataBuffer
    }
}
impl ::core::cmp::Eq for PDH_COUNTER_INFO_A {}
impl FromIntoMemory for PDH_COUNTER_INFO_A {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 68);
        let f_dwLength = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwType = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_CVersion = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_CStatus = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lScale = <i32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_lDefaultScale = <i32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwUserData = <PtrRepr as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_dwQueryUserData = <PtrRepr as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_szFullPath = <PSTR as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_Anonymous = <PDH_COUNTER_INFO_A_0 as FromIntoMemory>::from_bytes(&from[36..36 + 24]);
        let f_szExplainText = <PSTR as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_DataBuffer = <[u32; 1] as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        Self {
            dwLength: f_dwLength,
            dwType: f_dwType,
            CVersion: f_CVersion,
            CStatus: f_CStatus,
            lScale: f_lScale,
            lDefaultScale: f_lDefaultScale,
            dwUserData: f_dwUserData,
            dwQueryUserData: f_dwQueryUserData,
            szFullPath: f_szFullPath,
            Anonymous: f_Anonymous,
            szExplainText: f_szExplainText,
            DataBuffer: f_DataBuffer,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 68);
        FromIntoMemory::into_bytes(self.dwLength, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwType, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.CVersion, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.CStatus, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lScale, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.lDefaultScale, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwUserData, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.dwQueryUserData, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.szFullPath, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[36..36 + 24]);
        FromIntoMemory::into_bytes(self.szExplainText, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.DataBuffer, &mut into[64..64 + 4]);
    }
    fn size() -> usize {
        68
    }
}
pub struct PDH_COUNTER_INFO_A_0 {
    data: [u8; 24],
}
impl ::core::default::Default for PDH_COUNTER_INFO_A_0 {
    fn default() -> Self {
        Self { data: [0u8; 24] }
    }
}
impl ::core::marker::Copy for PDH_COUNTER_INFO_A_0 {}
impl ::core::clone::Clone for PDH_COUNTER_INFO_A_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_COUNTER_INFO_A_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_COUNTER_INFO_A_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_COUNTER_INFO_A_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for PDH_COUNTER_INFO_A_0 {}
impl FromIntoMemory for PDH_COUNTER_INFO_A_0 {
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
pub struct PDH_COUNTER_INFO_A_0_0 {
    pub szMachineName: PSTR,
    pub szObjectName: PSTR,
    pub szInstanceName: PSTR,
    pub szParentInstance: PSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: PSTR,
}
impl ::core::marker::Copy for PDH_COUNTER_INFO_A_0_0 {}
impl ::core::clone::Clone for PDH_COUNTER_INFO_A_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_COUNTER_INFO_A_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_COUNTER_INFO_A_0_0")
            .field("szMachineName", &self.szMachineName)
            .field("szObjectName", &self.szObjectName)
            .field("szInstanceName", &self.szInstanceName)
            .field("szParentInstance", &self.szParentInstance)
            .field("dwInstanceIndex", &self.dwInstanceIndex)
            .field("szCounterName", &self.szCounterName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_COUNTER_INFO_A_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.szMachineName == other.szMachineName
            && self.szObjectName == other.szObjectName
            && self.szInstanceName == other.szInstanceName
            && self.szParentInstance == other.szParentInstance
            && self.dwInstanceIndex == other.dwInstanceIndex
            && self.szCounterName == other.szCounterName
    }
}
impl ::core::cmp::Eq for PDH_COUNTER_INFO_A_0_0 {}
impl FromIntoMemory for PDH_COUNTER_INFO_A_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_szMachineName = <PSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_szObjectName = <PSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szInstanceName = <PSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_szParentInstance = <PSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwInstanceIndex = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_szCounterName = <PSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            szMachineName: f_szMachineName,
            szObjectName: f_szObjectName,
            szInstanceName: f_szInstanceName,
            szParentInstance: f_szParentInstance,
            dwInstanceIndex: f_dwInstanceIndex,
            szCounterName: f_szCounterName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.szMachineName, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.szObjectName, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szInstanceName, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.szParentInstance, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwInstanceIndex, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.szCounterName, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct PDH_COUNTER_INFO_W {
    pub dwLength: u32,
    pub dwType: u32,
    pub CVersion: u32,
    pub CStatus: u32,
    pub lScale: i32,
    pub lDefaultScale: i32,
    pub dwUserData: PtrRepr,
    pub dwQueryUserData: PtrRepr,
    pub szFullPath: PWSTR,
    pub Anonymous: PDH_COUNTER_INFO_W_0,
    pub szExplainText: PWSTR,
    pub DataBuffer: [u32; 1],
}
impl ::core::marker::Copy for PDH_COUNTER_INFO_W {}
impl ::core::clone::Clone for PDH_COUNTER_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_COUNTER_INFO_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_COUNTER_INFO_W")
            .field("dwLength", &self.dwLength)
            .field("dwType", &self.dwType)
            .field("CVersion", &self.CVersion)
            .field("CStatus", &self.CStatus)
            .field("lScale", &self.lScale)
            .field("lDefaultScale", &self.lDefaultScale)
            .field("dwUserData", &self.dwUserData)
            .field("dwQueryUserData", &self.dwQueryUserData)
            .field("szFullPath", &self.szFullPath)
            .field("Anonymous", &self.Anonymous)
            .field("szExplainText", &self.szExplainText)
            .field("DataBuffer", &self.DataBuffer)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_COUNTER_INFO_W {
    fn eq(&self, other: &Self) -> bool {
        self.dwLength == other.dwLength
            && self.dwType == other.dwType
            && self.CVersion == other.CVersion
            && self.CStatus == other.CStatus
            && self.lScale == other.lScale
            && self.lDefaultScale == other.lDefaultScale
            && self.dwUserData == other.dwUserData
            && self.dwQueryUserData == other.dwQueryUserData
            && self.szFullPath == other.szFullPath
            && self.Anonymous == other.Anonymous
            && self.szExplainText == other.szExplainText
            && self.DataBuffer == other.DataBuffer
    }
}
impl ::core::cmp::Eq for PDH_COUNTER_INFO_W {}
impl FromIntoMemory for PDH_COUNTER_INFO_W {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 68);
        let f_dwLength = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwType = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_CVersion = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_CStatus = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lScale = <i32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_lDefaultScale = <i32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwUserData = <PtrRepr as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_dwQueryUserData = <PtrRepr as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_szFullPath = <PWSTR as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_Anonymous = <PDH_COUNTER_INFO_W_0 as FromIntoMemory>::from_bytes(&from[36..36 + 24]);
        let f_szExplainText = <PWSTR as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_DataBuffer = <[u32; 1] as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        Self {
            dwLength: f_dwLength,
            dwType: f_dwType,
            CVersion: f_CVersion,
            CStatus: f_CStatus,
            lScale: f_lScale,
            lDefaultScale: f_lDefaultScale,
            dwUserData: f_dwUserData,
            dwQueryUserData: f_dwQueryUserData,
            szFullPath: f_szFullPath,
            Anonymous: f_Anonymous,
            szExplainText: f_szExplainText,
            DataBuffer: f_DataBuffer,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 68);
        FromIntoMemory::into_bytes(self.dwLength, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwType, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.CVersion, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.CStatus, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lScale, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.lDefaultScale, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwUserData, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.dwQueryUserData, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.szFullPath, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[36..36 + 24]);
        FromIntoMemory::into_bytes(self.szExplainText, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.DataBuffer, &mut into[64..64 + 4]);
    }
    fn size() -> usize {
        68
    }
}
pub struct PDH_COUNTER_INFO_W_0 {
    data: [u8; 24],
}
impl ::core::default::Default for PDH_COUNTER_INFO_W_0 {
    fn default() -> Self {
        Self { data: [0u8; 24] }
    }
}
impl ::core::marker::Copy for PDH_COUNTER_INFO_W_0 {}
impl ::core::clone::Clone for PDH_COUNTER_INFO_W_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_COUNTER_INFO_W_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_COUNTER_INFO_W_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_COUNTER_INFO_W_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for PDH_COUNTER_INFO_W_0 {}
impl FromIntoMemory for PDH_COUNTER_INFO_W_0 {
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
pub struct PDH_COUNTER_INFO_W_0_0 {
    pub szMachineName: PWSTR,
    pub szObjectName: PWSTR,
    pub szInstanceName: PWSTR,
    pub szParentInstance: PWSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: PWSTR,
}
impl ::core::marker::Copy for PDH_COUNTER_INFO_W_0_0 {}
impl ::core::clone::Clone for PDH_COUNTER_INFO_W_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_COUNTER_INFO_W_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_COUNTER_INFO_W_0_0")
            .field("szMachineName", &self.szMachineName)
            .field("szObjectName", &self.szObjectName)
            .field("szInstanceName", &self.szInstanceName)
            .field("szParentInstance", &self.szParentInstance)
            .field("dwInstanceIndex", &self.dwInstanceIndex)
            .field("szCounterName", &self.szCounterName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_COUNTER_INFO_W_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.szMachineName == other.szMachineName
            && self.szObjectName == other.szObjectName
            && self.szInstanceName == other.szInstanceName
            && self.szParentInstance == other.szParentInstance
            && self.dwInstanceIndex == other.dwInstanceIndex
            && self.szCounterName == other.szCounterName
    }
}
impl ::core::cmp::Eq for PDH_COUNTER_INFO_W_0_0 {}
impl FromIntoMemory for PDH_COUNTER_INFO_W_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_szMachineName = <PWSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_szObjectName = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szInstanceName = <PWSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_szParentInstance = <PWSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwInstanceIndex = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_szCounterName = <PWSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            szMachineName: f_szMachineName,
            szObjectName: f_szObjectName,
            szInstanceName: f_szInstanceName,
            szParentInstance: f_szParentInstance,
            dwInstanceIndex: f_dwInstanceIndex,
            szCounterName: f_szCounterName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.szMachineName, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.szObjectName, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szInstanceName, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.szParentInstance, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwInstanceIndex, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.szCounterName, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct PDH_COUNTER_PATH_ELEMENTS_A {
    pub szMachineName: PSTR,
    pub szObjectName: PSTR,
    pub szInstanceName: PSTR,
    pub szParentInstance: PSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: PSTR,
}
impl ::core::marker::Copy for PDH_COUNTER_PATH_ELEMENTS_A {}
impl ::core::clone::Clone for PDH_COUNTER_PATH_ELEMENTS_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_COUNTER_PATH_ELEMENTS_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_COUNTER_PATH_ELEMENTS_A")
            .field("szMachineName", &self.szMachineName)
            .field("szObjectName", &self.szObjectName)
            .field("szInstanceName", &self.szInstanceName)
            .field("szParentInstance", &self.szParentInstance)
            .field("dwInstanceIndex", &self.dwInstanceIndex)
            .field("szCounterName", &self.szCounterName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_COUNTER_PATH_ELEMENTS_A {
    fn eq(&self, other: &Self) -> bool {
        self.szMachineName == other.szMachineName
            && self.szObjectName == other.szObjectName
            && self.szInstanceName == other.szInstanceName
            && self.szParentInstance == other.szParentInstance
            && self.dwInstanceIndex == other.dwInstanceIndex
            && self.szCounterName == other.szCounterName
    }
}
impl ::core::cmp::Eq for PDH_COUNTER_PATH_ELEMENTS_A {}
impl FromIntoMemory for PDH_COUNTER_PATH_ELEMENTS_A {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_szMachineName = <PSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_szObjectName = <PSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szInstanceName = <PSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_szParentInstance = <PSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwInstanceIndex = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_szCounterName = <PSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            szMachineName: f_szMachineName,
            szObjectName: f_szObjectName,
            szInstanceName: f_szInstanceName,
            szParentInstance: f_szParentInstance,
            dwInstanceIndex: f_dwInstanceIndex,
            szCounterName: f_szCounterName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.szMachineName, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.szObjectName, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szInstanceName, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.szParentInstance, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwInstanceIndex, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.szCounterName, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct PDH_COUNTER_PATH_ELEMENTS_W {
    pub szMachineName: PWSTR,
    pub szObjectName: PWSTR,
    pub szInstanceName: PWSTR,
    pub szParentInstance: PWSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: PWSTR,
}
impl ::core::marker::Copy for PDH_COUNTER_PATH_ELEMENTS_W {}
impl ::core::clone::Clone for PDH_COUNTER_PATH_ELEMENTS_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_COUNTER_PATH_ELEMENTS_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_COUNTER_PATH_ELEMENTS_W")
            .field("szMachineName", &self.szMachineName)
            .field("szObjectName", &self.szObjectName)
            .field("szInstanceName", &self.szInstanceName)
            .field("szParentInstance", &self.szParentInstance)
            .field("dwInstanceIndex", &self.dwInstanceIndex)
            .field("szCounterName", &self.szCounterName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_COUNTER_PATH_ELEMENTS_W {
    fn eq(&self, other: &Self) -> bool {
        self.szMachineName == other.szMachineName
            && self.szObjectName == other.szObjectName
            && self.szInstanceName == other.szInstanceName
            && self.szParentInstance == other.szParentInstance
            && self.dwInstanceIndex == other.dwInstanceIndex
            && self.szCounterName == other.szCounterName
    }
}
impl ::core::cmp::Eq for PDH_COUNTER_PATH_ELEMENTS_W {}
impl FromIntoMemory for PDH_COUNTER_PATH_ELEMENTS_W {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_szMachineName = <PWSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_szObjectName = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szInstanceName = <PWSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_szParentInstance = <PWSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwInstanceIndex = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_szCounterName = <PWSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            szMachineName: f_szMachineName,
            szObjectName: f_szObjectName,
            szInstanceName: f_szInstanceName,
            szParentInstance: f_szParentInstance,
            dwInstanceIndex: f_dwInstanceIndex,
            szCounterName: f_szCounterName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.szMachineName, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.szObjectName, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szInstanceName, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.szParentInstance, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwInstanceIndex, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.szCounterName, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub const PDH_CSTATUS_BAD_COUNTERNAME: i32 = -1073738816i32;
pub const PDH_CSTATUS_INVALID_DATA: i32 = -1073738822i32;
pub const PDH_CSTATUS_ITEM_NOT_VALIDATED: i32 = -2147481645i32;
pub const PDH_CSTATUS_NEW_DATA: i32 = 1i32;
pub const PDH_CSTATUS_NO_COUNTER: i32 = -1073738823i32;
pub const PDH_CSTATUS_NO_COUNTERNAME: i32 = -1073738817i32;
pub const PDH_CSTATUS_NO_INSTANCE: i32 = -2147481647i32;
pub const PDH_CSTATUS_NO_MACHINE: i32 = -2147481648i32;
pub const PDH_CSTATUS_NO_OBJECT: i32 = -1073738824i32;
pub const PDH_CSTATUS_VALID_DATA: i32 = 0i32;
pub struct PDH_DATA_ITEM_PATH_ELEMENTS_A {
    pub szMachineName: PSTR,
    pub ObjectGUID: crate::core::GUID,
    pub dwItemId: u32,
    pub szInstanceName: PSTR,
}
impl ::core::marker::Copy for PDH_DATA_ITEM_PATH_ELEMENTS_A {}
impl ::core::clone::Clone for PDH_DATA_ITEM_PATH_ELEMENTS_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_DATA_ITEM_PATH_ELEMENTS_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_DATA_ITEM_PATH_ELEMENTS_A")
            .field("szMachineName", &self.szMachineName)
            .field("ObjectGUID", &self.ObjectGUID)
            .field("dwItemId", &self.dwItemId)
            .field("szInstanceName", &self.szInstanceName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_DATA_ITEM_PATH_ELEMENTS_A {
    fn eq(&self, other: &Self) -> bool {
        self.szMachineName == other.szMachineName
            && self.ObjectGUID == other.ObjectGUID
            && self.dwItemId == other.dwItemId
            && self.szInstanceName == other.szInstanceName
    }
}
impl ::core::cmp::Eq for PDH_DATA_ITEM_PATH_ELEMENTS_A {}
impl FromIntoMemory for PDH_DATA_ITEM_PATH_ELEMENTS_A {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_szMachineName = <PSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ObjectGUID = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[4..4 + 16]);
        let f_dwItemId = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_szInstanceName = <PSTR as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        Self {
            szMachineName: f_szMachineName,
            ObjectGUID: f_ObjectGUID,
            dwItemId: f_dwItemId,
            szInstanceName: f_szInstanceName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.szMachineName, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ObjectGUID, &mut into[4..4 + 16]);
        FromIntoMemory::into_bytes(self.dwItemId, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.szInstanceName, &mut into[24..24 + 4]);
    }
    fn size() -> usize {
        28
    }
}
pub struct PDH_DATA_ITEM_PATH_ELEMENTS_W {
    pub szMachineName: PWSTR,
    pub ObjectGUID: crate::core::GUID,
    pub dwItemId: u32,
    pub szInstanceName: PWSTR,
}
impl ::core::marker::Copy for PDH_DATA_ITEM_PATH_ELEMENTS_W {}
impl ::core::clone::Clone for PDH_DATA_ITEM_PATH_ELEMENTS_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_DATA_ITEM_PATH_ELEMENTS_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_DATA_ITEM_PATH_ELEMENTS_W")
            .field("szMachineName", &self.szMachineName)
            .field("ObjectGUID", &self.ObjectGUID)
            .field("dwItemId", &self.dwItemId)
            .field("szInstanceName", &self.szInstanceName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_DATA_ITEM_PATH_ELEMENTS_W {
    fn eq(&self, other: &Self) -> bool {
        self.szMachineName == other.szMachineName
            && self.ObjectGUID == other.ObjectGUID
            && self.dwItemId == other.dwItemId
            && self.szInstanceName == other.szInstanceName
    }
}
impl ::core::cmp::Eq for PDH_DATA_ITEM_PATH_ELEMENTS_W {}
impl FromIntoMemory for PDH_DATA_ITEM_PATH_ELEMENTS_W {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_szMachineName = <PWSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ObjectGUID = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[4..4 + 16]);
        let f_dwItemId = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_szInstanceName = <PWSTR as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        Self {
            szMachineName: f_szMachineName,
            ObjectGUID: f_ObjectGUID,
            dwItemId: f_dwItemId,
            szInstanceName: f_szInstanceName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.szMachineName, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ObjectGUID, &mut into[4..4 + 16]);
        FromIntoMemory::into_bytes(self.dwItemId, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.szInstanceName, &mut into[24..24 + 4]);
    }
    fn size() -> usize {
        28
    }
}
pub const PDH_DATA_SOURCE_IS_LOG_FILE: i32 = -1073738802i32;
pub const PDH_DATA_SOURCE_IS_REAL_TIME: i32 = -1073738801i32;
pub const PDH_DIALOG_CANCELLED: i32 = -2147481639i32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PDH_DLL_VERSION(pub u32);
pub const PDH_CVERSION_WIN50: PDH_DLL_VERSION = PDH_DLL_VERSION(1280u32);
pub const PDH_VERSION: PDH_DLL_VERSION = PDH_DLL_VERSION(1283u32);
impl ::core::marker::Copy for PDH_DLL_VERSION {}
impl ::core::clone::Clone for PDH_DLL_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PDH_DLL_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PDH_DLL_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PDH_DLL_VERSION").field(&self.0).finish()
    }
}
impl FromIntoMemory for PDH_DLL_VERSION {
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
pub const PDH_END_OF_LOG_FILE: i32 = -2147481638i32;
pub const PDH_ENTRY_NOT_IN_LOG_FILE: i32 = -1073738803i32;
pub const PDH_FILE_ALREADY_EXISTS: i32 = -1073738798i32;
pub const PDH_FILE_NOT_FOUND: i32 = -1073738799i32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PDH_FMT(pub u32);
pub const PDH_FMT_DOUBLE: PDH_FMT = PDH_FMT(512u32);
pub const PDH_FMT_LARGE: PDH_FMT = PDH_FMT(1024u32);
pub const PDH_FMT_LONG: PDH_FMT = PDH_FMT(256u32);
impl ::core::marker::Copy for PDH_FMT {}
impl ::core::clone::Clone for PDH_FMT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PDH_FMT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PDH_FMT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PDH_FMT").field(&self.0).finish()
    }
}
impl FromIntoMemory for PDH_FMT {
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
pub struct PDH_FMT_COUNTERVALUE {
    pub CStatus: u32,
    pub Anonymous: PDH_FMT_COUNTERVALUE_0,
}
impl ::core::marker::Copy for PDH_FMT_COUNTERVALUE {}
impl ::core::clone::Clone for PDH_FMT_COUNTERVALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_FMT_COUNTERVALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_FMT_COUNTERVALUE")
            .field("CStatus", &self.CStatus)
            .field("Anonymous", &self.Anonymous)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_FMT_COUNTERVALUE {
    fn eq(&self, other: &Self) -> bool {
        self.CStatus == other.CStatus && self.Anonymous == other.Anonymous
    }
}
impl ::core::cmp::Eq for PDH_FMT_COUNTERVALUE {}
impl FromIntoMemory for PDH_FMT_COUNTERVALUE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_CStatus = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Anonymous = <PDH_FMT_COUNTERVALUE_0 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        Self {
            CStatus: f_CStatus,
            Anonymous: f_Anonymous,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.CStatus, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[8..8 + 8]);
    }
    fn size() -> usize {
        16
    }
}
pub struct PDH_FMT_COUNTERVALUE_0 {
    data: [u8; 8],
}
impl ::core::default::Default for PDH_FMT_COUNTERVALUE_0 {
    fn default() -> Self {
        Self { data: [0u8; 8] }
    }
}
impl ::core::marker::Copy for PDH_FMT_COUNTERVALUE_0 {}
impl ::core::clone::Clone for PDH_FMT_COUNTERVALUE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_FMT_COUNTERVALUE_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_FMT_COUNTERVALUE_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_FMT_COUNTERVALUE_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for PDH_FMT_COUNTERVALUE_0 {}
impl FromIntoMemory for PDH_FMT_COUNTERVALUE_0 {
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
pub struct PDH_FMT_COUNTERVALUE_ITEM_A {
    pub szName: PSTR,
    pub FmtValue: PDH_FMT_COUNTERVALUE,
}
impl ::core::marker::Copy for PDH_FMT_COUNTERVALUE_ITEM_A {}
impl ::core::clone::Clone for PDH_FMT_COUNTERVALUE_ITEM_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_FMT_COUNTERVALUE_ITEM_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_FMT_COUNTERVALUE_ITEM_A")
            .field("szName", &self.szName)
            .field("FmtValue", &self.FmtValue)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_FMT_COUNTERVALUE_ITEM_A {
    fn eq(&self, other: &Self) -> bool {
        self.szName == other.szName && self.FmtValue == other.FmtValue
    }
}
impl ::core::cmp::Eq for PDH_FMT_COUNTERVALUE_ITEM_A {}
impl FromIntoMemory for PDH_FMT_COUNTERVALUE_ITEM_A {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_szName = <PSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_FmtValue = <PDH_FMT_COUNTERVALUE as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        Self {
            szName: f_szName,
            FmtValue: f_FmtValue,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.szName, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.FmtValue, &mut into[8..8 + 16]);
    }
    fn size() -> usize {
        24
    }
}
pub struct PDH_FMT_COUNTERVALUE_ITEM_W {
    pub szName: PWSTR,
    pub FmtValue: PDH_FMT_COUNTERVALUE,
}
impl ::core::marker::Copy for PDH_FMT_COUNTERVALUE_ITEM_W {}
impl ::core::clone::Clone for PDH_FMT_COUNTERVALUE_ITEM_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_FMT_COUNTERVALUE_ITEM_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_FMT_COUNTERVALUE_ITEM_W")
            .field("szName", &self.szName)
            .field("FmtValue", &self.FmtValue)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_FMT_COUNTERVALUE_ITEM_W {
    fn eq(&self, other: &Self) -> bool {
        self.szName == other.szName && self.FmtValue == other.FmtValue
    }
}
impl ::core::cmp::Eq for PDH_FMT_COUNTERVALUE_ITEM_W {}
impl FromIntoMemory for PDH_FMT_COUNTERVALUE_ITEM_W {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_szName = <PWSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_FmtValue = <PDH_FMT_COUNTERVALUE as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        Self {
            szName: f_szName,
            FmtValue: f_FmtValue,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.szName, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.FmtValue, &mut into[8..8 + 16]);
    }
    fn size() -> usize {
        24
    }
}
pub const PDH_FUNCTION_NOT_FOUND: i32 = -1073738818i32;
pub const PDH_INCORRECT_APPEND_TIME: i32 = -1073738757i32;
pub const PDH_INSUFFICIENT_BUFFER: i32 = -1073738814i32;
pub const PDH_INVALID_ARGUMENT: i32 = -1073738819i32;
pub const PDH_INVALID_BUFFER: i32 = -1073738815i32;
pub const PDH_INVALID_DATA: i32 = -1073738810i32;
pub const PDH_INVALID_DATASOURCE: i32 = -1073738787i32;
pub const PDH_INVALID_HANDLE: i32 = -1073738820i32;
pub const PDH_INVALID_INSTANCE: i32 = -1073738811i32;
pub const PDH_INVALID_PATH: i32 = -1073738812i32;
pub const PDH_INVALID_SQLDB: i32 = -1073738786i32;
pub const PDH_INVALID_SQL_LOG_FORMAT: i32 = -1073738763i32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PDH_LOG(pub u32);
pub const PDH_LOG_READ_ACCESS: PDH_LOG = PDH_LOG(65536u32);
pub const PDH_LOG_WRITE_ACCESS: PDH_LOG = PDH_LOG(131072u32);
pub const PDH_LOG_UPDATE_ACCESS: PDH_LOG = PDH_LOG(262144u32);
impl ::core::marker::Copy for PDH_LOG {}
impl ::core::clone::Clone for PDH_LOG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PDH_LOG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PDH_LOG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PDH_LOG").field(&self.0).finish()
    }
}
impl FromIntoMemory for PDH_LOG {
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
pub const PDH_LOGSVC_NOT_OPENED: i32 = -1073738791i32;
pub const PDH_LOGSVC_QUERY_NOT_FOUND: i32 = -1073738792i32;
pub const PDH_LOG_FILE_CREATE_ERROR: i32 = -1073738807i32;
pub const PDH_LOG_FILE_OPEN_ERROR: i32 = -1073738806i32;
pub const PDH_LOG_FILE_TOO_SMALL: i32 = -1073738788i32;
pub const PDH_LOG_SAMPLE_TOO_SMALL: i32 = -1073738760i32;
pub struct PDH_LOG_SERVICE_QUERY_INFO_A {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwLogQuota: u32,
    pub szLogFileCaption: PSTR,
    pub szDefaultDir: PSTR,
    pub szBaseFileName: PSTR,
    pub dwFileType: u32,
    pub dwReserved: u32,
    pub Anonymous: PDH_LOG_SERVICE_QUERY_INFO_A_0,
}
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_A {}
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_LOG_SERVICE_QUERY_INFO_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_LOG_SERVICE_QUERY_INFO_A")
            .field("dwSize", &self.dwSize)
            .field("dwFlags", &self.dwFlags)
            .field("dwLogQuota", &self.dwLogQuota)
            .field("szLogFileCaption", &self.szLogFileCaption)
            .field("szDefaultDir", &self.szDefaultDir)
            .field("szBaseFileName", &self.szBaseFileName)
            .field("dwFileType", &self.dwFileType)
            .field("dwReserved", &self.dwReserved)
            .field("Anonymous", &self.Anonymous)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_A {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwFlags == other.dwFlags
            && self.dwLogQuota == other.dwLogQuota
            && self.szLogFileCaption == other.szLogFileCaption
            && self.szDefaultDir == other.szDefaultDir
            && self.szBaseFileName == other.szBaseFileName
            && self.dwFileType == other.dwFileType
            && self.dwReserved == other.dwReserved
            && self.Anonymous == other.Anonymous
    }
}
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_A {}
impl FromIntoMemory for PDH_LOG_SERVICE_QUERY_INFO_A {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 72);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwLogQuota = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_szLogFileCaption = <PSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_szDefaultDir = <PSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_szBaseFileName = <PSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwFileType = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_dwReserved = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_Anonymous =
            <PDH_LOG_SERVICE_QUERY_INFO_A_0 as FromIntoMemory>::from_bytes(&from[32..32 + 40]);
        Self {
            dwSize: f_dwSize,
            dwFlags: f_dwFlags,
            dwLogQuota: f_dwLogQuota,
            szLogFileCaption: f_szLogFileCaption,
            szDefaultDir: f_szDefaultDir,
            szBaseFileName: f_szBaseFileName,
            dwFileType: f_dwFileType,
            dwReserved: f_dwReserved,
            Anonymous: f_Anonymous,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 72);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwLogQuota, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.szLogFileCaption, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.szDefaultDir, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.szBaseFileName, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwFileType, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.dwReserved, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[32..32 + 40]);
    }
    fn size() -> usize {
        72
    }
}
pub struct PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    data: [u8; 40],
}
impl ::core::default::Default for PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    fn default() -> Self {
        Self { data: [0u8; 40] }
    }
}
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_A_0 {}
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_LOG_SERVICE_QUERY_INFO_A_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_A_0 {}
impl FromIntoMemory for PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 40];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        40
    }
}
pub struct PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    pub PdlAutoNameInterval: u32,
    pub PdlAutoNameUnits: u32,
    pub PdlCommandFilename: PSTR,
    pub PdlCounterList: PSTR,
    pub PdlAutoNameFormat: u32,
    pub PdlSampleInterval: u32,
    pub PdlLogStartTime: super::super::Foundation::FILETIME,
    pub PdlLogEndTime: super::super::Foundation::FILETIME,
}
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {}
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_LOG_SERVICE_QUERY_INFO_A_0_0")
            .field("PdlAutoNameInterval", &self.PdlAutoNameInterval)
            .field("PdlAutoNameUnits", &self.PdlAutoNameUnits)
            .field("PdlCommandFilename", &self.PdlCommandFilename)
            .field("PdlCounterList", &self.PdlCounterList)
            .field("PdlAutoNameFormat", &self.PdlAutoNameFormat)
            .field("PdlSampleInterval", &self.PdlSampleInterval)
            .field("PdlLogStartTime", &self.PdlLogStartTime)
            .field("PdlLogEndTime", &self.PdlLogEndTime)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.PdlAutoNameInterval == other.PdlAutoNameInterval
            && self.PdlAutoNameUnits == other.PdlAutoNameUnits
            && self.PdlCommandFilename == other.PdlCommandFilename
            && self.PdlCounterList == other.PdlCounterList
            && self.PdlAutoNameFormat == other.PdlAutoNameFormat
            && self.PdlSampleInterval == other.PdlSampleInterval
            && self.PdlLogStartTime == other.PdlLogStartTime
            && self.PdlLogEndTime == other.PdlLogEndTime
    }
}
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {}
impl FromIntoMemory for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_PdlAutoNameInterval = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_PdlAutoNameUnits = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_PdlCommandFilename = <PSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_PdlCounterList = <PSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_PdlAutoNameFormat = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_PdlSampleInterval = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_PdlLogStartTime =
            <super::super::Foundation::FILETIME as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        let f_PdlLogEndTime =
            <super::super::Foundation::FILETIME as FromIntoMemory>::from_bytes(&from[32..32 + 8]);
        Self {
            PdlAutoNameInterval: f_PdlAutoNameInterval,
            PdlAutoNameUnits: f_PdlAutoNameUnits,
            PdlCommandFilename: f_PdlCommandFilename,
            PdlCounterList: f_PdlCounterList,
            PdlAutoNameFormat: f_PdlAutoNameFormat,
            PdlSampleInterval: f_PdlSampleInterval,
            PdlLogStartTime: f_PdlLogStartTime,
            PdlLogEndTime: f_PdlLogEndTime,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.PdlAutoNameInterval, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.PdlAutoNameUnits, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.PdlCommandFilename, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.PdlCounterList, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.PdlAutoNameFormat, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.PdlSampleInterval, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.PdlLogStartTime, &mut into[24..24 + 8]);
        FromIntoMemory::into_bytes(self.PdlLogEndTime, &mut into[32..32 + 8]);
    }
    fn size() -> usize {
        40
    }
}
pub struct PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    pub TlNumberOfBuffers: u32,
    pub TlMinimumBuffers: u32,
    pub TlMaximumBuffers: u32,
    pub TlFreeBuffers: u32,
    pub TlBufferSize: u32,
    pub TlEventsLost: u32,
    pub TlLoggerThreadId: u32,
    pub TlBuffersWritten: u32,
    pub TlLogHandle: u32,
    pub TlLogFileName: PSTR,
}
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {}
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_LOG_SERVICE_QUERY_INFO_A_0_1")
            .field("TlNumberOfBuffers", &self.TlNumberOfBuffers)
            .field("TlMinimumBuffers", &self.TlMinimumBuffers)
            .field("TlMaximumBuffers", &self.TlMaximumBuffers)
            .field("TlFreeBuffers", &self.TlFreeBuffers)
            .field("TlBufferSize", &self.TlBufferSize)
            .field("TlEventsLost", &self.TlEventsLost)
            .field("TlLoggerThreadId", &self.TlLoggerThreadId)
            .field("TlBuffersWritten", &self.TlBuffersWritten)
            .field("TlLogHandle", &self.TlLogHandle)
            .field("TlLogFileName", &self.TlLogFileName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.TlNumberOfBuffers == other.TlNumberOfBuffers
            && self.TlMinimumBuffers == other.TlMinimumBuffers
            && self.TlMaximumBuffers == other.TlMaximumBuffers
            && self.TlFreeBuffers == other.TlFreeBuffers
            && self.TlBufferSize == other.TlBufferSize
            && self.TlEventsLost == other.TlEventsLost
            && self.TlLoggerThreadId == other.TlLoggerThreadId
            && self.TlBuffersWritten == other.TlBuffersWritten
            && self.TlLogHandle == other.TlLogHandle
            && self.TlLogFileName == other.TlLogFileName
    }
}
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {}
impl FromIntoMemory for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_TlNumberOfBuffers = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_TlMinimumBuffers = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_TlMaximumBuffers = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_TlFreeBuffers = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_TlBufferSize = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_TlEventsLost = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_TlLoggerThreadId = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_TlBuffersWritten = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_TlLogHandle = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_TlLogFileName = <PSTR as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        Self {
            TlNumberOfBuffers: f_TlNumberOfBuffers,
            TlMinimumBuffers: f_TlMinimumBuffers,
            TlMaximumBuffers: f_TlMaximumBuffers,
            TlFreeBuffers: f_TlFreeBuffers,
            TlBufferSize: f_TlBufferSize,
            TlEventsLost: f_TlEventsLost,
            TlLoggerThreadId: f_TlLoggerThreadId,
            TlBuffersWritten: f_TlBuffersWritten,
            TlLogHandle: f_TlLogHandle,
            TlLogFileName: f_TlLogFileName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.TlNumberOfBuffers, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.TlMinimumBuffers, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.TlMaximumBuffers, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.TlFreeBuffers, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.TlBufferSize, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.TlEventsLost, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.TlLoggerThreadId, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.TlBuffersWritten, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.TlLogHandle, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.TlLogFileName, &mut into[36..36 + 4]);
    }
    fn size() -> usize {
        40
    }
}
pub struct PDH_LOG_SERVICE_QUERY_INFO_W {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwLogQuota: u32,
    pub szLogFileCaption: PWSTR,
    pub szDefaultDir: PWSTR,
    pub szBaseFileName: PWSTR,
    pub dwFileType: u32,
    pub dwReserved: u32,
    pub Anonymous: PDH_LOG_SERVICE_QUERY_INFO_W_0,
}
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_W {}
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_LOG_SERVICE_QUERY_INFO_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_LOG_SERVICE_QUERY_INFO_W")
            .field("dwSize", &self.dwSize)
            .field("dwFlags", &self.dwFlags)
            .field("dwLogQuota", &self.dwLogQuota)
            .field("szLogFileCaption", &self.szLogFileCaption)
            .field("szDefaultDir", &self.szDefaultDir)
            .field("szBaseFileName", &self.szBaseFileName)
            .field("dwFileType", &self.dwFileType)
            .field("dwReserved", &self.dwReserved)
            .field("Anonymous", &self.Anonymous)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_W {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwFlags == other.dwFlags
            && self.dwLogQuota == other.dwLogQuota
            && self.szLogFileCaption == other.szLogFileCaption
            && self.szDefaultDir == other.szDefaultDir
            && self.szBaseFileName == other.szBaseFileName
            && self.dwFileType == other.dwFileType
            && self.dwReserved == other.dwReserved
            && self.Anonymous == other.Anonymous
    }
}
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_W {}
impl FromIntoMemory for PDH_LOG_SERVICE_QUERY_INFO_W {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 72);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwLogQuota = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_szLogFileCaption = <PWSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_szDefaultDir = <PWSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_szBaseFileName = <PWSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwFileType = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_dwReserved = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_Anonymous =
            <PDH_LOG_SERVICE_QUERY_INFO_W_0 as FromIntoMemory>::from_bytes(&from[32..32 + 40]);
        Self {
            dwSize: f_dwSize,
            dwFlags: f_dwFlags,
            dwLogQuota: f_dwLogQuota,
            szLogFileCaption: f_szLogFileCaption,
            szDefaultDir: f_szDefaultDir,
            szBaseFileName: f_szBaseFileName,
            dwFileType: f_dwFileType,
            dwReserved: f_dwReserved,
            Anonymous: f_Anonymous,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 72);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwLogQuota, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.szLogFileCaption, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.szDefaultDir, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.szBaseFileName, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwFileType, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.dwReserved, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[32..32 + 40]);
    }
    fn size() -> usize {
        72
    }
}
pub struct PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    data: [u8; 40],
}
impl ::core::default::Default for PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    fn default() -> Self {
        Self { data: [0u8; 40] }
    }
}
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_W_0 {}
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_LOG_SERVICE_QUERY_INFO_W_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_W_0 {}
impl FromIntoMemory for PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 40];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        40
    }
}
pub struct PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    pub PdlAutoNameInterval: u32,
    pub PdlAutoNameUnits: u32,
    pub PdlCommandFilename: PWSTR,
    pub PdlCounterList: PWSTR,
    pub PdlAutoNameFormat: u32,
    pub PdlSampleInterval: u32,
    pub PdlLogStartTime: super::super::Foundation::FILETIME,
    pub PdlLogEndTime: super::super::Foundation::FILETIME,
}
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {}
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_LOG_SERVICE_QUERY_INFO_W_0_0")
            .field("PdlAutoNameInterval", &self.PdlAutoNameInterval)
            .field("PdlAutoNameUnits", &self.PdlAutoNameUnits)
            .field("PdlCommandFilename", &self.PdlCommandFilename)
            .field("PdlCounterList", &self.PdlCounterList)
            .field("PdlAutoNameFormat", &self.PdlAutoNameFormat)
            .field("PdlSampleInterval", &self.PdlSampleInterval)
            .field("PdlLogStartTime", &self.PdlLogStartTime)
            .field("PdlLogEndTime", &self.PdlLogEndTime)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.PdlAutoNameInterval == other.PdlAutoNameInterval
            && self.PdlAutoNameUnits == other.PdlAutoNameUnits
            && self.PdlCommandFilename == other.PdlCommandFilename
            && self.PdlCounterList == other.PdlCounterList
            && self.PdlAutoNameFormat == other.PdlAutoNameFormat
            && self.PdlSampleInterval == other.PdlSampleInterval
            && self.PdlLogStartTime == other.PdlLogStartTime
            && self.PdlLogEndTime == other.PdlLogEndTime
    }
}
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {}
impl FromIntoMemory for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_PdlAutoNameInterval = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_PdlAutoNameUnits = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_PdlCommandFilename = <PWSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_PdlCounterList = <PWSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_PdlAutoNameFormat = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_PdlSampleInterval = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_PdlLogStartTime =
            <super::super::Foundation::FILETIME as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        let f_PdlLogEndTime =
            <super::super::Foundation::FILETIME as FromIntoMemory>::from_bytes(&from[32..32 + 8]);
        Self {
            PdlAutoNameInterval: f_PdlAutoNameInterval,
            PdlAutoNameUnits: f_PdlAutoNameUnits,
            PdlCommandFilename: f_PdlCommandFilename,
            PdlCounterList: f_PdlCounterList,
            PdlAutoNameFormat: f_PdlAutoNameFormat,
            PdlSampleInterval: f_PdlSampleInterval,
            PdlLogStartTime: f_PdlLogStartTime,
            PdlLogEndTime: f_PdlLogEndTime,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.PdlAutoNameInterval, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.PdlAutoNameUnits, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.PdlCommandFilename, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.PdlCounterList, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.PdlAutoNameFormat, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.PdlSampleInterval, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.PdlLogStartTime, &mut into[24..24 + 8]);
        FromIntoMemory::into_bytes(self.PdlLogEndTime, &mut into[32..32 + 8]);
    }
    fn size() -> usize {
        40
    }
}
pub struct PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    pub TlNumberOfBuffers: u32,
    pub TlMinimumBuffers: u32,
    pub TlMaximumBuffers: u32,
    pub TlFreeBuffers: u32,
    pub TlBufferSize: u32,
    pub TlEventsLost: u32,
    pub TlLoggerThreadId: u32,
    pub TlBuffersWritten: u32,
    pub TlLogHandle: u32,
    pub TlLogFileName: PWSTR,
}
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {}
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_LOG_SERVICE_QUERY_INFO_W_0_1")
            .field("TlNumberOfBuffers", &self.TlNumberOfBuffers)
            .field("TlMinimumBuffers", &self.TlMinimumBuffers)
            .field("TlMaximumBuffers", &self.TlMaximumBuffers)
            .field("TlFreeBuffers", &self.TlFreeBuffers)
            .field("TlBufferSize", &self.TlBufferSize)
            .field("TlEventsLost", &self.TlEventsLost)
            .field("TlLoggerThreadId", &self.TlLoggerThreadId)
            .field("TlBuffersWritten", &self.TlBuffersWritten)
            .field("TlLogHandle", &self.TlLogHandle)
            .field("TlLogFileName", &self.TlLogFileName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.TlNumberOfBuffers == other.TlNumberOfBuffers
            && self.TlMinimumBuffers == other.TlMinimumBuffers
            && self.TlMaximumBuffers == other.TlMaximumBuffers
            && self.TlFreeBuffers == other.TlFreeBuffers
            && self.TlBufferSize == other.TlBufferSize
            && self.TlEventsLost == other.TlEventsLost
            && self.TlLoggerThreadId == other.TlLoggerThreadId
            && self.TlBuffersWritten == other.TlBuffersWritten
            && self.TlLogHandle == other.TlLogHandle
            && self.TlLogFileName == other.TlLogFileName
    }
}
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {}
impl FromIntoMemory for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_TlNumberOfBuffers = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_TlMinimumBuffers = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_TlMaximumBuffers = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_TlFreeBuffers = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_TlBufferSize = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_TlEventsLost = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_TlLoggerThreadId = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_TlBuffersWritten = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_TlLogHandle = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_TlLogFileName = <PWSTR as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        Self {
            TlNumberOfBuffers: f_TlNumberOfBuffers,
            TlMinimumBuffers: f_TlMinimumBuffers,
            TlMaximumBuffers: f_TlMaximumBuffers,
            TlFreeBuffers: f_TlFreeBuffers,
            TlBufferSize: f_TlBufferSize,
            TlEventsLost: f_TlEventsLost,
            TlLoggerThreadId: f_TlLoggerThreadId,
            TlBuffersWritten: f_TlBuffersWritten,
            TlLogHandle: f_TlLogHandle,
            TlLogFileName: f_TlLogFileName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.TlNumberOfBuffers, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.TlMinimumBuffers, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.TlMaximumBuffers, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.TlFreeBuffers, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.TlBufferSize, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.TlEventsLost, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.TlLoggerThreadId, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.TlBuffersWritten, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.TlLogHandle, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.TlLogFileName, &mut into[36..36 + 4]);
    }
    fn size() -> usize {
        40
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PDH_LOG_TYPE(pub u32);
pub const PDH_LOG_TYPE_UNDEFINED: PDH_LOG_TYPE = PDH_LOG_TYPE(0u32);
pub const PDH_LOG_TYPE_CSV: PDH_LOG_TYPE = PDH_LOG_TYPE(1u32);
pub const PDH_LOG_TYPE_SQL: PDH_LOG_TYPE = PDH_LOG_TYPE(7u32);
pub const PDH_LOG_TYPE_TSV: PDH_LOG_TYPE = PDH_LOG_TYPE(2u32);
pub const PDH_LOG_TYPE_BINARY: PDH_LOG_TYPE = PDH_LOG_TYPE(8u32);
pub const PDH_LOG_TYPE_PERFMON: PDH_LOG_TYPE = PDH_LOG_TYPE(6u32);
impl ::core::marker::Copy for PDH_LOG_TYPE {}
impl ::core::clone::Clone for PDH_LOG_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PDH_LOG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PDH_LOG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PDH_LOG_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for PDH_LOG_TYPE {
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
pub const PDH_LOG_TYPE_NOT_FOUND: i32 = -1073738805i32;
pub const PDH_LOG_TYPE_RETIRED_BIN: u32 = 3u32;
pub const PDH_LOG_TYPE_TRACE_GENERIC: u32 = 5u32;
pub const PDH_LOG_TYPE_TRACE_KERNEL: u32 = 4u32;
pub const PDH_MAX_COUNTER_NAME: u32 = 1024u32;
pub const PDH_MAX_COUNTER_PATH: u32 = 2048u32;
pub const PDH_MAX_DATASOURCE_PATH: u32 = 1024u32;
pub const PDH_MAX_INSTANCE_NAME: u32 = 1024u32;
pub const PDH_MAX_SCALE: i32 = 7i32;
pub const PDH_MEMORY_ALLOCATION_FAILURE: i32 = -1073738821i32;
pub const PDH_MIN_SCALE: i32 = -7i32;
pub const PDH_MORE_DATA: i32 = -2147481646i32;
pub const PDH_NOEXPANDCOUNTERS: u32 = 1u32;
pub const PDH_NOEXPANDINSTANCES: u32 = 2u32;
pub const PDH_NOT_IMPLEMENTED: i32 = -1073738797i32;
pub const PDH_NO_COUNTERS: i32 = -1073738785i32;
pub const PDH_NO_DATA: i32 = -2147481643i32;
pub const PDH_NO_DIALOG_DATA: i32 = -1073738809i32;
pub const PDH_NO_MORE_DATA: i32 = -1073738804i32;
pub const PDH_OS_EARLIER_VERSION: i32 = -1073738758i32;
pub const PDH_OS_LATER_VERSION: i32 = -1073738759i32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PDH_PATH_FLAGS(pub u32);
pub const PDH_PATH_WBEM_RESULT: PDH_PATH_FLAGS = PDH_PATH_FLAGS(1u32);
pub const PDH_PATH_WBEM_INPUT: PDH_PATH_FLAGS = PDH_PATH_FLAGS(2u32);
pub const PDH_PATH_WBEM_NONE: PDH_PATH_FLAGS = PDH_PATH_FLAGS(0u32);
impl ::core::marker::Copy for PDH_PATH_FLAGS {}
impl ::core::clone::Clone for PDH_PATH_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PDH_PATH_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PDH_PATH_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PDH_PATH_FLAGS").field(&self.0).finish()
    }
}
impl FromIntoMemory for PDH_PATH_FLAGS {
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
pub const PDH_PLA_COLLECTION_ALREADY_RUNNING: i32 = -1073738775i32;
pub const PDH_PLA_COLLECTION_NOT_FOUND: i32 = -1073738773i32;
pub const PDH_PLA_ERROR_ALREADY_EXISTS: i32 = -1073738770i32;
pub const PDH_PLA_ERROR_FILEPATH: i32 = -1073738768i32;
pub const PDH_PLA_ERROR_NAME_TOO_LONG: i32 = -1073738764i32;
pub const PDH_PLA_ERROR_NOSTART: i32 = -1073738771i32;
pub const PDH_PLA_ERROR_SCHEDULE_ELAPSED: i32 = -1073738772i32;
pub const PDH_PLA_ERROR_SCHEDULE_OVERLAP: i32 = -1073738774i32;
pub const PDH_PLA_ERROR_TYPE_MISMATCH: i32 = -1073738769i32;
pub const PDH_PLA_SERVICE_ERROR: i32 = -1073738767i32;
pub const PDH_PLA_VALIDATION_ERROR: i32 = -1073738766i32;
pub const PDH_PLA_VALIDATION_WARNING: i32 = -2147480589i32;
pub const PDH_QUERY_PERF_DATA_TIMEOUT: i32 = -1073738754i32;
pub struct PDH_RAW_COUNTER {
    pub CStatus: u32,
    pub TimeStamp: super::super::Foundation::FILETIME,
    pub FirstValue: i64,
    pub SecondValue: i64,
    pub MultiCount: u32,
}
impl ::core::marker::Copy for PDH_RAW_COUNTER {}
impl ::core::clone::Clone for PDH_RAW_COUNTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_RAW_COUNTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_RAW_COUNTER")
            .field("CStatus", &self.CStatus)
            .field("TimeStamp", &self.TimeStamp)
            .field("FirstValue", &self.FirstValue)
            .field("SecondValue", &self.SecondValue)
            .field("MultiCount", &self.MultiCount)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_RAW_COUNTER {
    fn eq(&self, other: &Self) -> bool {
        self.CStatus == other.CStatus
            && self.TimeStamp == other.TimeStamp
            && self.FirstValue == other.FirstValue
            && self.SecondValue == other.SecondValue
            && self.MultiCount == other.MultiCount
    }
}
impl ::core::cmp::Eq for PDH_RAW_COUNTER {}
impl FromIntoMemory for PDH_RAW_COUNTER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_CStatus = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_TimeStamp =
            <super::super::Foundation::FILETIME as FromIntoMemory>::from_bytes(&from[4..4 + 8]);
        let f_FirstValue = <i64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_SecondValue = <i64 as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        let f_MultiCount = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        Self {
            CStatus: f_CStatus,
            TimeStamp: f_TimeStamp,
            FirstValue: f_FirstValue,
            SecondValue: f_SecondValue,
            MultiCount: f_MultiCount,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.CStatus, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.TimeStamp, &mut into[4..4 + 8]);
        FromIntoMemory::into_bytes(self.FirstValue, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.SecondValue, &mut into[24..24 + 8]);
        FromIntoMemory::into_bytes(self.MultiCount, &mut into[32..32 + 4]);
    }
    fn size() -> usize {
        40
    }
}
pub struct PDH_RAW_COUNTER_ITEM_A {
    pub szName: PSTR,
    pub RawValue: PDH_RAW_COUNTER,
}
impl ::core::marker::Copy for PDH_RAW_COUNTER_ITEM_A {}
impl ::core::clone::Clone for PDH_RAW_COUNTER_ITEM_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_RAW_COUNTER_ITEM_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_RAW_COUNTER_ITEM_A")
            .field("szName", &self.szName)
            .field("RawValue", &self.RawValue)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_RAW_COUNTER_ITEM_A {
    fn eq(&self, other: &Self) -> bool {
        self.szName == other.szName && self.RawValue == other.RawValue
    }
}
impl ::core::cmp::Eq for PDH_RAW_COUNTER_ITEM_A {}
impl FromIntoMemory for PDH_RAW_COUNTER_ITEM_A {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_szName = <PSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_RawValue = <PDH_RAW_COUNTER as FromIntoMemory>::from_bytes(&from[8..8 + 40]);
        Self {
            szName: f_szName,
            RawValue: f_RawValue,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.szName, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.RawValue, &mut into[8..8 + 40]);
    }
    fn size() -> usize {
        48
    }
}
pub struct PDH_RAW_COUNTER_ITEM_W {
    pub szName: PWSTR,
    pub RawValue: PDH_RAW_COUNTER,
}
impl ::core::marker::Copy for PDH_RAW_COUNTER_ITEM_W {}
impl ::core::clone::Clone for PDH_RAW_COUNTER_ITEM_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_RAW_COUNTER_ITEM_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_RAW_COUNTER_ITEM_W")
            .field("szName", &self.szName)
            .field("RawValue", &self.RawValue)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_RAW_COUNTER_ITEM_W {
    fn eq(&self, other: &Self) -> bool {
        self.szName == other.szName && self.RawValue == other.RawValue
    }
}
impl ::core::cmp::Eq for PDH_RAW_COUNTER_ITEM_W {}
impl FromIntoMemory for PDH_RAW_COUNTER_ITEM_W {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_szName = <PWSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_RawValue = <PDH_RAW_COUNTER as FromIntoMemory>::from_bytes(&from[8..8 + 40]);
        Self {
            szName: f_szName,
            RawValue: f_RawValue,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.szName, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.RawValue, &mut into[8..8 + 40]);
    }
    fn size() -> usize {
        48
    }
}
pub struct PDH_RAW_LOG_RECORD {
    pub dwStructureSize: u32,
    pub dwRecordType: PDH_LOG_TYPE,
    pub dwItems: u32,
    pub RawBytes: [u8; 1],
}
impl ::core::marker::Copy for PDH_RAW_LOG_RECORD {}
impl ::core::clone::Clone for PDH_RAW_LOG_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_RAW_LOG_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_RAW_LOG_RECORD")
            .field("dwStructureSize", &self.dwStructureSize)
            .field("dwRecordType", &self.dwRecordType)
            .field("dwItems", &self.dwItems)
            .field("RawBytes", &self.RawBytes)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_RAW_LOG_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructureSize == other.dwStructureSize
            && self.dwRecordType == other.dwRecordType
            && self.dwItems == other.dwItems
            && self.RawBytes == other.RawBytes
    }
}
impl ::core::cmp::Eq for PDH_RAW_LOG_RECORD {}
impl FromIntoMemory for PDH_RAW_LOG_RECORD {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dwStructureSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwRecordType = <PDH_LOG_TYPE as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwItems = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_RawBytes = <[u8; 1] as FromIntoMemory>::from_bytes(&from[12..12 + 1]);
        Self {
            dwStructureSize: f_dwStructureSize,
            dwRecordType: f_dwRecordType,
            dwItems: f_dwItems,
            RawBytes: f_RawBytes,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dwStructureSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwRecordType, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwItems, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.RawBytes, &mut into[12..12 + 1]);
    }
    fn size() -> usize {
        16
    }
}
pub const PDH_REFRESHCOUNTERS: u32 = 4u32;
pub const PDH_RETRY: i32 = -2147481644i32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PDH_SELECT_DATA_SOURCE_FLAGS(pub u32);
pub const PDH_FLAGS_FILE_BROWSER_ONLY: PDH_SELECT_DATA_SOURCE_FLAGS =
    PDH_SELECT_DATA_SOURCE_FLAGS(1u32);
pub const PDH_FLAGS_NONE: PDH_SELECT_DATA_SOURCE_FLAGS = PDH_SELECT_DATA_SOURCE_FLAGS(0u32);
impl ::core::marker::Copy for PDH_SELECT_DATA_SOURCE_FLAGS {}
impl ::core::clone::Clone for PDH_SELECT_DATA_SOURCE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PDH_SELECT_DATA_SOURCE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PDH_SELECT_DATA_SOURCE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PDH_SELECT_DATA_SOURCE_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for PDH_SELECT_DATA_SOURCE_FLAGS {
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
pub const PDH_SQL_ALLOCCON_FAILED: i32 = -1073738783i32;
pub const PDH_SQL_ALLOC_FAILED: i32 = -1073738784i32;
pub const PDH_SQL_ALTER_DETAIL_FAILED: i32 = -1073738755i32;
pub const PDH_SQL_BIND_FAILED: i32 = -1073738777i32;
pub const PDH_SQL_CONNECT_FAILED: i32 = -1073738778i32;
pub const PDH_SQL_EXEC_DIRECT_FAILED: i32 = -1073738782i32;
pub const PDH_SQL_FETCH_FAILED: i32 = -1073738781i32;
pub const PDH_SQL_MORE_RESULTS_FAILED: i32 = -1073738779i32;
pub const PDH_SQL_ROWCOUNT_FAILED: i32 = -1073738780i32;
pub struct PDH_STATISTICS {
    pub dwFormat: u32,
    pub count: u32,
    pub min: PDH_FMT_COUNTERVALUE,
    pub max: PDH_FMT_COUNTERVALUE,
    pub mean: PDH_FMT_COUNTERVALUE,
}
impl ::core::marker::Copy for PDH_STATISTICS {}
impl ::core::clone::Clone for PDH_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_STATISTICS")
            .field("dwFormat", &self.dwFormat)
            .field("count", &self.count)
            .field("min", &self.min)
            .field("max", &self.max)
            .field("mean", &self.mean)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.dwFormat == other.dwFormat
            && self.count == other.count
            && self.min == other.min
            && self.max == other.max
            && self.mean == other.mean
    }
}
impl ::core::cmp::Eq for PDH_STATISTICS {}
impl FromIntoMemory for PDH_STATISTICS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 56);
        let f_dwFormat = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_count = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_min = <PDH_FMT_COUNTERVALUE as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        let f_max = <PDH_FMT_COUNTERVALUE as FromIntoMemory>::from_bytes(&from[24..24 + 16]);
        let f_mean = <PDH_FMT_COUNTERVALUE as FromIntoMemory>::from_bytes(&from[40..40 + 16]);
        Self {
            dwFormat: f_dwFormat,
            count: f_count,
            min: f_min,
            max: f_max,
            mean: f_mean,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 56);
        FromIntoMemory::into_bytes(self.dwFormat, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.count, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.min, &mut into[8..8 + 16]);
        FromIntoMemory::into_bytes(self.max, &mut into[24..24 + 16]);
        FromIntoMemory::into_bytes(self.mean, &mut into[40..40 + 16]);
    }
    fn size() -> usize {
        56
    }
}
pub const PDH_STRING_NOT_FOUND: i32 = -1073738796i32;
pub struct PDH_TIME_INFO {
    pub StartTime: i64,
    pub EndTime: i64,
    pub SampleCount: u32,
}
impl ::core::marker::Copy for PDH_TIME_INFO {}
impl ::core::clone::Clone for PDH_TIME_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PDH_TIME_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDH_TIME_INFO")
            .field("StartTime", &self.StartTime)
            .field("EndTime", &self.EndTime)
            .field("SampleCount", &self.SampleCount)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PDH_TIME_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.StartTime == other.StartTime
            && self.EndTime == other.EndTime
            && self.SampleCount == other.SampleCount
    }
}
impl ::core::cmp::Eq for PDH_TIME_INFO {}
impl FromIntoMemory for PDH_TIME_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_StartTime = <i64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_EndTime = <i64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_SampleCount = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            StartTime: f_StartTime,
            EndTime: f_EndTime,
            SampleCount: f_SampleCount,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.StartTime, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.EndTime, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.SampleCount, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub const PDH_UNABLE_MAP_NAME_FILES: i32 = -2147480619i32;
pub const PDH_UNABLE_READ_LOG_HEADER: i32 = -1073738800i32;
pub const PDH_UNKNOWN_LOGSVC_COMMAND: i32 = -1073738793i32;
pub const PDH_UNKNOWN_LOG_FORMAT: i32 = -1073738794i32;
pub const PDH_UNMATCHED_APPEND_COUNTER: i32 = -1073738756i32;
pub const PDH_WBEM_ERROR: i32 = -1073738790i32;
pub type PERFLIBREQUEST = StdCallFnPtr<(u32, MutPtr<::core::ffi::c_void>, u32), u32>;
pub const PERF_ADD_COUNTER: u32 = 1u32;
pub const PERF_AGGREGATE_INSTANCE: &'static str = "_Total";
pub const PERF_AGGREGATE_MAX: u32 = 4u32;
pub const PERF_ATTRIB_BY_REFERENCE: u64 = 1u64;
pub const PERF_ATTRIB_DISPLAY_AS_HEX: u64 = 16u64;
pub const PERF_ATTRIB_DISPLAY_AS_REAL: u64 = 8u64;
pub const PERF_ATTRIB_NO_DISPLAYABLE: u64 = 2u64;
pub const PERF_ATTRIB_NO_GROUP_SEPARATOR: u64 = 4u64;
pub const PERF_COLLECT_END: u32 = 6u32;
pub const PERF_COLLECT_START: u32 = 5u32;
pub const PERF_COUNTERSET_FLAG_AGGREGATE: u32 = 4u32;
pub const PERF_COUNTERSET_FLAG_HISTORY: u32 = 8u32;
pub const PERF_COUNTERSET_FLAG_INSTANCE: u32 = 16u32;
pub const PERF_COUNTERSET_FLAG_MULTIPLE: u32 = 2u32;
pub struct PERF_COUNTERSET_INFO {
    pub CounterSetGuid: crate::core::GUID,
    pub ProviderGuid: crate::core::GUID,
    pub NumCounters: u32,
    pub InstanceType: u32,
}
impl ::core::marker::Copy for PERF_COUNTERSET_INFO {}
impl ::core::clone::Clone for PERF_COUNTERSET_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTERSET_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTERSET_INFO")
            .field("CounterSetGuid", &self.CounterSetGuid)
            .field("ProviderGuid", &self.ProviderGuid)
            .field("NumCounters", &self.NumCounters)
            .field("InstanceType", &self.InstanceType)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PERF_COUNTERSET_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.CounterSetGuid == other.CounterSetGuid
            && self.ProviderGuid == other.ProviderGuid
            && self.NumCounters == other.NumCounters
            && self.InstanceType == other.InstanceType
    }
}
impl ::core::cmp::Eq for PERF_COUNTERSET_INFO {}
impl FromIntoMemory for PERF_COUNTERSET_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_CounterSetGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_ProviderGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[16..16 + 16]);
        let f_NumCounters = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_InstanceType = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        Self {
            CounterSetGuid: f_CounterSetGuid,
            ProviderGuid: f_ProviderGuid,
            NumCounters: f_NumCounters,
            InstanceType: f_InstanceType,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.CounterSetGuid, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.ProviderGuid, &mut into[16..16 + 16]);
        FromIntoMemory::into_bytes(self.NumCounters, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.InstanceType, &mut into[36..36 + 4]);
    }
    fn size() -> usize {
        40
    }
}
pub struct PERF_COUNTERSET_INSTANCE {
    pub CounterSetGuid: crate::core::GUID,
    pub dwSize: u32,
    pub InstanceId: u32,
    pub InstanceNameOffset: u32,
    pub InstanceNameSize: u32,
}
impl ::core::marker::Copy for PERF_COUNTERSET_INSTANCE {}
impl ::core::clone::Clone for PERF_COUNTERSET_INSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTERSET_INSTANCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTERSET_INSTANCE")
            .field("CounterSetGuid", &self.CounterSetGuid)
            .field("dwSize", &self.dwSize)
            .field("InstanceId", &self.InstanceId)
            .field("InstanceNameOffset", &self.InstanceNameOffset)
            .field("InstanceNameSize", &self.InstanceNameSize)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PERF_COUNTERSET_INSTANCE {
    fn eq(&self, other: &Self) -> bool {
        self.CounterSetGuid == other.CounterSetGuid
            && self.dwSize == other.dwSize
            && self.InstanceId == other.InstanceId
            && self.InstanceNameOffset == other.InstanceNameOffset
            && self.InstanceNameSize == other.InstanceNameSize
    }
}
impl ::core::cmp::Eq for PERF_COUNTERSET_INSTANCE {}
impl FromIntoMemory for PERF_COUNTERSET_INSTANCE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_CounterSetGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_InstanceId = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_InstanceNameOffset = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_InstanceNameSize = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            CounterSetGuid: f_CounterSetGuid,
            dwSize: f_dwSize,
            InstanceId: f_InstanceId,
            InstanceNameOffset: f_InstanceNameOffset,
            InstanceNameSize: f_InstanceNameSize,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.CounterSetGuid, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.InstanceId, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.InstanceNameOffset, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.InstanceNameSize, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
pub const PERF_COUNTERSET_MULTI_INSTANCES: u32 = 2u32;
pub struct PERF_COUNTERSET_REG_INFO {
    pub CounterSetGuid: crate::core::GUID,
    pub CounterSetType: u32,
    pub DetailLevel: u32,
    pub NumCounters: u32,
    pub InstanceType: u32,
}
impl ::core::marker::Copy for PERF_COUNTERSET_REG_INFO {}
impl ::core::clone::Clone for PERF_COUNTERSET_REG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTERSET_REG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTERSET_REG_INFO")
            .field("CounterSetGuid", &self.CounterSetGuid)
            .field("CounterSetType", &self.CounterSetType)
            .field("DetailLevel", &self.DetailLevel)
            .field("NumCounters", &self.NumCounters)
            .field("InstanceType", &self.InstanceType)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PERF_COUNTERSET_REG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.CounterSetGuid == other.CounterSetGuid
            && self.CounterSetType == other.CounterSetType
            && self.DetailLevel == other.DetailLevel
            && self.NumCounters == other.NumCounters
            && self.InstanceType == other.InstanceType
    }
}
impl ::core::cmp::Eq for PERF_COUNTERSET_REG_INFO {}
impl FromIntoMemory for PERF_COUNTERSET_REG_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_CounterSetGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_CounterSetType = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_DetailLevel = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_NumCounters = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_InstanceType = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            CounterSetGuid: f_CounterSetGuid,
            CounterSetType: f_CounterSetType,
            DetailLevel: f_DetailLevel,
            NumCounters: f_NumCounters,
            InstanceType: f_InstanceType,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.CounterSetGuid, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.CounterSetType, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.DetailLevel, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.NumCounters, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.InstanceType, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
pub const PERF_COUNTERSET_SINGLE_AGGREGATE: u32 = 4u32;
pub const PERF_COUNTERSET_SINGLE_INSTANCE: u32 = 0u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PERF_COUNTER_AGGREGATE_FUNC(pub u32);
pub const PERF_AGGREGATE_UNDEFINED: PERF_COUNTER_AGGREGATE_FUNC = PERF_COUNTER_AGGREGATE_FUNC(0u32);
pub const PERF_AGGREGATE_TOTAL: PERF_COUNTER_AGGREGATE_FUNC = PERF_COUNTER_AGGREGATE_FUNC(1u32);
pub const PERF_AGGREGATE_AVG: PERF_COUNTER_AGGREGATE_FUNC = PERF_COUNTER_AGGREGATE_FUNC(2u32);
pub const PERF_AGGREGATE_MIN: PERF_COUNTER_AGGREGATE_FUNC = PERF_COUNTER_AGGREGATE_FUNC(3u32);
impl ::core::marker::Copy for PERF_COUNTER_AGGREGATE_FUNC {}
impl ::core::clone::Clone for PERF_COUNTER_AGGREGATE_FUNC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PERF_COUNTER_AGGREGATE_FUNC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_AGGREGATE_FUNC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PERF_COUNTER_AGGREGATE_FUNC")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for PERF_COUNTER_AGGREGATE_FUNC {
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
pub const PERF_COUNTER_BASE: u32 = 196608u32;
pub struct PERF_COUNTER_BLOCK {
    pub ByteLength: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_BLOCK {}
impl ::core::clone::Clone for PERF_COUNTER_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_BLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTER_BLOCK")
            .field("ByteLength", &self.ByteLength)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PERF_COUNTER_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.ByteLength == other.ByteLength
    }
}
impl ::core::cmp::Eq for PERF_COUNTER_BLOCK {}
impl FromIntoMemory for PERF_COUNTER_BLOCK {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_ByteLength = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            ByteLength: f_ByteLength,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.ByteLength, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct PERF_COUNTER_DATA {
    pub dwDataSize: u32,
    pub dwSize: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_DATA {}
impl ::core::clone::Clone for PERF_COUNTER_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTER_DATA")
            .field("dwDataSize", &self.dwDataSize)
            .field("dwSize", &self.dwSize)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PERF_COUNTER_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwDataSize == other.dwDataSize && self.dwSize == other.dwSize
    }
}
impl ::core::cmp::Eq for PERF_COUNTER_DATA {}
impl FromIntoMemory for PERF_COUNTER_DATA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_dwDataSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            dwDataSize: f_dwDataSize,
            dwSize: f_dwSize,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.dwDataSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct PERF_COUNTER_DEFINITION {
    pub ByteLength: u32,
    pub CounterNameTitleIndex: u32,
    pub CounterNameTitle: u32,
    pub CounterHelpTitleIndex: u32,
    pub CounterHelpTitle: u32,
    pub DefaultScale: i32,
    pub DetailLevel: u32,
    pub CounterType: u32,
    pub CounterSize: u32,
    pub CounterOffset: u32,
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for PERF_COUNTER_DEFINITION {}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for PERF_COUNTER_DEFINITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for PERF_COUNTER_DEFINITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTER_DEFINITION")
            .field("ByteLength", &self.ByteLength)
            .field("CounterNameTitleIndex", &self.CounterNameTitleIndex)
            .field("CounterNameTitle", &self.CounterNameTitle)
            .field("CounterHelpTitleIndex", &self.CounterHelpTitleIndex)
            .field("CounterHelpTitle", &self.CounterHelpTitle)
            .field("DefaultScale", &self.DefaultScale)
            .field("DetailLevel", &self.DetailLevel)
            .field("CounterType", &self.CounterType)
            .field("CounterSize", &self.CounterSize)
            .field("CounterOffset", &self.CounterOffset)
            .finish()
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for PERF_COUNTER_DEFINITION {
    fn eq(&self, other: &Self) -> bool {
        self.ByteLength == other.ByteLength
            && self.CounterNameTitleIndex == other.CounterNameTitleIndex
            && self.CounterNameTitle == other.CounterNameTitle
            && self.CounterHelpTitleIndex == other.CounterHelpTitleIndex
            && self.CounterHelpTitle == other.CounterHelpTitle
            && self.DefaultScale == other.DefaultScale
            && self.DetailLevel == other.DetailLevel
            && self.CounterType == other.CounterType
            && self.CounterSize == other.CounterSize
            && self.CounterOffset == other.CounterOffset
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for PERF_COUNTER_DEFINITION {}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for PERF_COUNTER_DEFINITION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_ByteLength = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_CounterNameTitleIndex = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_CounterNameTitle = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_CounterHelpTitleIndex = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_CounterHelpTitle = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_DefaultScale = <i32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_DetailLevel = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_CounterType = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_CounterSize = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_CounterOffset = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        Self {
            ByteLength: f_ByteLength,
            CounterNameTitleIndex: f_CounterNameTitleIndex,
            CounterNameTitle: f_CounterNameTitle,
            CounterHelpTitleIndex: f_CounterHelpTitleIndex,
            CounterHelpTitle: f_CounterHelpTitle,
            DefaultScale: f_DefaultScale,
            DetailLevel: f_DetailLevel,
            CounterType: f_CounterType,
            CounterSize: f_CounterSize,
            CounterOffset: f_CounterOffset,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.ByteLength, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.CounterNameTitleIndex, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.CounterNameTitle, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.CounterHelpTitleIndex, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.CounterHelpTitle, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.DefaultScale, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.DetailLevel, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.CounterType, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.CounterSize, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.CounterOffset, &mut into[36..36 + 4]);
    }
    fn size() -> usize {
        40
    }
}
pub struct PERF_COUNTER_DEFINITION {
    pub ByteLength: u32,
    pub CounterNameTitleIndex: u32,
    pub CounterNameTitle: PWSTR,
    pub CounterHelpTitleIndex: u32,
    pub CounterHelpTitle: PWSTR,
    pub DefaultScale: i32,
    pub DetailLevel: u32,
    pub CounterType: u32,
    pub CounterSize: u32,
    pub CounterOffset: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_DEFINITION {}
impl ::core::clone::Clone for PERF_COUNTER_DEFINITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_DEFINITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTER_DEFINITION")
            .field("ByteLength", &self.ByteLength)
            .field("CounterNameTitleIndex", &self.CounterNameTitleIndex)
            .field("CounterNameTitle", &self.CounterNameTitle)
            .field("CounterHelpTitleIndex", &self.CounterHelpTitleIndex)
            .field("CounterHelpTitle", &self.CounterHelpTitle)
            .field("DefaultScale", &self.DefaultScale)
            .field("DetailLevel", &self.DetailLevel)
            .field("CounterType", &self.CounterType)
            .field("CounterSize", &self.CounterSize)
            .field("CounterOffset", &self.CounterOffset)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PERF_COUNTER_DEFINITION {
    fn eq(&self, other: &Self) -> bool {
        self.ByteLength == other.ByteLength
            && self.CounterNameTitleIndex == other.CounterNameTitleIndex
            && self.CounterNameTitle == other.CounterNameTitle
            && self.CounterHelpTitleIndex == other.CounterHelpTitleIndex
            && self.CounterHelpTitle == other.CounterHelpTitle
            && self.DefaultScale == other.DefaultScale
            && self.DetailLevel == other.DetailLevel
            && self.CounterType == other.CounterType
            && self.CounterSize == other.CounterSize
            && self.CounterOffset == other.CounterOffset
    }
}
impl ::core::cmp::Eq for PERF_COUNTER_DEFINITION {}
impl FromIntoMemory for PERF_COUNTER_DEFINITION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_ByteLength = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_CounterNameTitleIndex = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_CounterNameTitle = <PWSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_CounterHelpTitleIndex = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_CounterHelpTitle = <PWSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_DefaultScale = <i32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_DetailLevel = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_CounterType = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_CounterSize = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_CounterOffset = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        Self {
            ByteLength: f_ByteLength,
            CounterNameTitleIndex: f_CounterNameTitleIndex,
            CounterNameTitle: f_CounterNameTitle,
            CounterHelpTitleIndex: f_CounterHelpTitleIndex,
            CounterHelpTitle: f_CounterHelpTitle,
            DefaultScale: f_DefaultScale,
            DetailLevel: f_DetailLevel,
            CounterType: f_CounterType,
            CounterSize: f_CounterSize,
            CounterOffset: f_CounterOffset,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.ByteLength, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.CounterNameTitleIndex, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.CounterNameTitle, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.CounterHelpTitleIndex, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.CounterHelpTitle, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.DefaultScale, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.DetailLevel, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.CounterType, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.CounterSize, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.CounterOffset, &mut into[36..36 + 4]);
    }
    fn size() -> usize {
        40
    }
}
pub const PERF_COUNTER_ELAPSED: u32 = 262144u32;
pub const PERF_COUNTER_FRACTION: u32 = 131072u32;
pub struct PERF_COUNTER_HEADER {
    pub dwStatus: u32,
    pub dwType: PerfCounterDataType,
    pub dwSize: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_HEADER {}
impl ::core::clone::Clone for PERF_COUNTER_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTER_HEADER")
            .field("dwStatus", &self.dwStatus)
            .field("dwType", &self.dwType)
            .field("dwSize", &self.dwSize)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PERF_COUNTER_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.dwStatus == other.dwStatus
            && self.dwType == other.dwType
            && self.dwSize == other.dwSize
            && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for PERF_COUNTER_HEADER {}
impl FromIntoMemory for PERF_COUNTER_HEADER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dwStatus = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwType = <PerfCounterDataType as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_Reserved = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            dwStatus: f_dwStatus,
            dwType: f_dwType,
            dwSize: f_dwSize,
            Reserved: f_Reserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dwStatus, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwType, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub const PERF_COUNTER_HISTOGRAM: u32 = 393216u32;
pub const PERF_COUNTER_HISTOGRAM_TYPE: u32 = 2147483648u32;
pub struct PERF_COUNTER_IDENTIFIER {
    pub CounterSetGuid: crate::core::GUID,
    pub Status: u32,
    pub Size: u32,
    pub CounterId: u32,
    pub InstanceId: u32,
    pub Index: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_IDENTIFIER {}
impl ::core::clone::Clone for PERF_COUNTER_IDENTIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_IDENTIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTER_IDENTIFIER")
            .field("CounterSetGuid", &self.CounterSetGuid)
            .field("Status", &self.Status)
            .field("Size", &self.Size)
            .field("CounterId", &self.CounterId)
            .field("InstanceId", &self.InstanceId)
            .field("Index", &self.Index)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PERF_COUNTER_IDENTIFIER {
    fn eq(&self, other: &Self) -> bool {
        self.CounterSetGuid == other.CounterSetGuid
            && self.Status == other.Status
            && self.Size == other.Size
            && self.CounterId == other.CounterId
            && self.InstanceId == other.InstanceId
            && self.Index == other.Index
            && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for PERF_COUNTER_IDENTIFIER {}
impl FromIntoMemory for PERF_COUNTER_IDENTIFIER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_CounterSetGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_Status = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_Size = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_CounterId = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_InstanceId = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_Index = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_Reserved = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        Self {
            CounterSetGuid: f_CounterSetGuid,
            Status: f_Status,
            Size: f_Size,
            CounterId: f_CounterId,
            InstanceId: f_InstanceId,
            Index: f_Index,
            Reserved: f_Reserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.CounterSetGuid, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.Status, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.Size, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.CounterId, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.InstanceId, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.Index, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[36..36 + 4]);
    }
    fn size() -> usize {
        40
    }
}
pub struct PERF_COUNTER_IDENTITY {
    pub CounterSetGuid: crate::core::GUID,
    pub BufferSize: u32,
    pub CounterId: u32,
    pub InstanceId: u32,
    pub MachineOffset: u32,
    pub NameOffset: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_IDENTITY {}
impl ::core::clone::Clone for PERF_COUNTER_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTER_IDENTITY")
            .field("CounterSetGuid", &self.CounterSetGuid)
            .field("BufferSize", &self.BufferSize)
            .field("CounterId", &self.CounterId)
            .field("InstanceId", &self.InstanceId)
            .field("MachineOffset", &self.MachineOffset)
            .field("NameOffset", &self.NameOffset)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PERF_COUNTER_IDENTITY {
    fn eq(&self, other: &Self) -> bool {
        self.CounterSetGuid == other.CounterSetGuid
            && self.BufferSize == other.BufferSize
            && self.CounterId == other.CounterId
            && self.InstanceId == other.InstanceId
            && self.MachineOffset == other.MachineOffset
            && self.NameOffset == other.NameOffset
            && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for PERF_COUNTER_IDENTITY {}
impl FromIntoMemory for PERF_COUNTER_IDENTITY {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_CounterSetGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_BufferSize = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_CounterId = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_InstanceId = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_MachineOffset = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_NameOffset = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_Reserved = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        Self {
            CounterSetGuid: f_CounterSetGuid,
            BufferSize: f_BufferSize,
            CounterId: f_CounterId,
            InstanceId: f_InstanceId,
            MachineOffset: f_MachineOffset,
            NameOffset: f_NameOffset,
            Reserved: f_Reserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.CounterSetGuid, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.BufferSize, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.CounterId, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.InstanceId, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.MachineOffset, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.NameOffset, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[36..36 + 4]);
    }
    fn size() -> usize {
        40
    }
}
pub struct PERF_COUNTER_INFO {
    pub CounterId: u32,
    pub Type: u32,
    pub Attrib: u64,
    pub Size: u32,
    pub DetailLevel: u32,
    pub Scale: i32,
    pub Offset: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_INFO {}
impl ::core::clone::Clone for PERF_COUNTER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTER_INFO")
            .field("CounterId", &self.CounterId)
            .field("Type", &self.Type)
            .field("Attrib", &self.Attrib)
            .field("Size", &self.Size)
            .field("DetailLevel", &self.DetailLevel)
            .field("Scale", &self.Scale)
            .field("Offset", &self.Offset)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PERF_COUNTER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.CounterId == other.CounterId
            && self.Type == other.Type
            && self.Attrib == other.Attrib
            && self.Size == other.Size
            && self.DetailLevel == other.DetailLevel
            && self.Scale == other.Scale
            && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for PERF_COUNTER_INFO {}
impl FromIntoMemory for PERF_COUNTER_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_CounterId = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Type = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Attrib = <u64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_Size = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_DetailLevel = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_Scale = <i32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_Offset = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            CounterId: f_CounterId,
            Type: f_Type,
            Attrib: f_Attrib,
            Size: f_Size,
            DetailLevel: f_DetailLevel,
            Scale: f_Scale,
            Offset: f_Offset,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.CounterId, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Type, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Attrib, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.Size, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.DetailLevel, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.Scale, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.Offset, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
pub const PERF_COUNTER_PRECISION: u32 = 458752u32;
pub const PERF_COUNTER_QUEUELEN: u32 = 327680u32;
pub const PERF_COUNTER_RATE: u32 = 65536u32;
pub struct PERF_COUNTER_REG_INFO {
    pub CounterId: u32,
    pub Type: u32,
    pub Attrib: u64,
    pub DetailLevel: u32,
    pub DefaultScale: i32,
    pub BaseCounterId: u32,
    pub PerfTimeId: u32,
    pub PerfFreqId: u32,
    pub MultiId: u32,
    pub AggregateFunc: PERF_COUNTER_AGGREGATE_FUNC,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_REG_INFO {}
impl ::core::clone::Clone for PERF_COUNTER_REG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_REG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_COUNTER_REG_INFO")
            .field("CounterId", &self.CounterId)
            .field("Type", &self.Type)
            .field("Attrib", &self.Attrib)
            .field("DetailLevel", &self.DetailLevel)
            .field("DefaultScale", &self.DefaultScale)
            .field("BaseCounterId", &self.BaseCounterId)
            .field("PerfTimeId", &self.PerfTimeId)
            .field("PerfFreqId", &self.PerfFreqId)
            .field("MultiId", &self.MultiId)
            .field("AggregateFunc", &self.AggregateFunc)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PERF_COUNTER_REG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.CounterId == other.CounterId
            && self.Type == other.Type
            && self.Attrib == other.Attrib
            && self.DetailLevel == other.DetailLevel
            && self.DefaultScale == other.DefaultScale
            && self.BaseCounterId == other.BaseCounterId
            && self.PerfTimeId == other.PerfTimeId
            && self.PerfFreqId == other.PerfFreqId
            && self.MultiId == other.MultiId
            && self.AggregateFunc == other.AggregateFunc
            && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for PERF_COUNTER_REG_INFO {}
impl FromIntoMemory for PERF_COUNTER_REG_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_CounterId = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Type = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Attrib = <u64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_DetailLevel = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_DefaultScale = <i32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_BaseCounterId = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_PerfTimeId = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_PerfFreqId = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_MultiId = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_AggregateFunc =
            <PERF_COUNTER_AGGREGATE_FUNC as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_Reserved = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        Self {
            CounterId: f_CounterId,
            Type: f_Type,
            Attrib: f_Attrib,
            DetailLevel: f_DetailLevel,
            DefaultScale: f_DefaultScale,
            BaseCounterId: f_BaseCounterId,
            PerfTimeId: f_PerfTimeId,
            PerfFreqId: f_PerfFreqId,
            MultiId: f_MultiId,
            AggregateFunc: f_AggregateFunc,
            Reserved: f_Reserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.CounterId, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Type, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Attrib, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.DetailLevel, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.DefaultScale, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.BaseCounterId, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.PerfTimeId, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.PerfFreqId, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.MultiId, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.AggregateFunc, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[44..44 + 4]);
    }
    fn size() -> usize {
        48
    }
}
pub const PERF_COUNTER_VALUE: u32 = 0u32;
pub struct PERF_DATA_BLOCK {
    pub Signature: [u16; 4],
    pub LittleEndian: u32,
    pub Version: u32,
    pub Revision: u32,
    pub TotalByteLength: u32,
    pub HeaderLength: u32,
    pub NumObjectTypes: u32,
    pub DefaultObject: i32,
    pub SystemTime: super::super::Foundation::SYSTEMTIME,
    pub PerfTime: i64,
    pub PerfFreq: i64,
    pub PerfTime100nSec: i64,
    pub SystemNameLength: u32,
    pub SystemNameOffset: u32,
}
impl ::core::marker::Copy for PERF_DATA_BLOCK {}
impl ::core::clone::Clone for PERF_DATA_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_DATA_BLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_DATA_BLOCK")
            .field("Signature", &self.Signature)
            .field("LittleEndian", &self.LittleEndian)
            .field("Version", &self.Version)
            .field("Revision", &self.Revision)
            .field("TotalByteLength", &self.TotalByteLength)
            .field("HeaderLength", &self.HeaderLength)
            .field("NumObjectTypes", &self.NumObjectTypes)
            .field("DefaultObject", &self.DefaultObject)
            .field("SystemTime", &self.SystemTime)
            .field("PerfTime", &self.PerfTime)
            .field("PerfFreq", &self.PerfFreq)
            .field("PerfTime100nSec", &self.PerfTime100nSec)
            .field("SystemNameLength", &self.SystemNameLength)
            .field("SystemNameOffset", &self.SystemNameOffset)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PERF_DATA_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.Signature == other.Signature
            && self.LittleEndian == other.LittleEndian
            && self.Version == other.Version
            && self.Revision == other.Revision
            && self.TotalByteLength == other.TotalByteLength
            && self.HeaderLength == other.HeaderLength
            && self.NumObjectTypes == other.NumObjectTypes
            && self.DefaultObject == other.DefaultObject
            && self.SystemTime == other.SystemTime
            && self.PerfTime == other.PerfTime
            && self.PerfFreq == other.PerfFreq
            && self.PerfTime100nSec == other.PerfTime100nSec
            && self.SystemNameLength == other.SystemNameLength
            && self.SystemNameOffset == other.SystemNameOffset
    }
}
impl ::core::cmp::Eq for PERF_DATA_BLOCK {}
impl FromIntoMemory for PERF_DATA_BLOCK {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 80);
        let f_Signature = <[u16; 4] as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_LittleEndian = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Version = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_Revision = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_TotalByteLength = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_HeaderLength = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_NumObjectTypes = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_DefaultObject = <i32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_SystemTime = <super::super::Foundation::SYSTEMTIME as FromIntoMemory>::from_bytes(
            &from[32..32 + 16],
        );
        let f_PerfTime = <i64 as FromIntoMemory>::from_bytes(&from[48..48 + 8]);
        let f_PerfFreq = <i64 as FromIntoMemory>::from_bytes(&from[56..56 + 8]);
        let f_PerfTime100nSec = <i64 as FromIntoMemory>::from_bytes(&from[64..64 + 8]);
        let f_SystemNameLength = <u32 as FromIntoMemory>::from_bytes(&from[72..72 + 4]);
        let f_SystemNameOffset = <u32 as FromIntoMemory>::from_bytes(&from[76..76 + 4]);
        Self {
            Signature: f_Signature,
            LittleEndian: f_LittleEndian,
            Version: f_Version,
            Revision: f_Revision,
            TotalByteLength: f_TotalByteLength,
            HeaderLength: f_HeaderLength,
            NumObjectTypes: f_NumObjectTypes,
            DefaultObject: f_DefaultObject,
            SystemTime: f_SystemTime,
            PerfTime: f_PerfTime,
            PerfFreq: f_PerfFreq,
            PerfTime100nSec: f_PerfTime100nSec,
            SystemNameLength: f_SystemNameLength,
            SystemNameOffset: f_SystemNameOffset,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 80);
        FromIntoMemory::into_bytes(self.Signature, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.LittleEndian, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Version, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.Revision, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.TotalByteLength, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.HeaderLength, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.NumObjectTypes, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.DefaultObject, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.SystemTime, &mut into[32..32 + 16]);
        FromIntoMemory::into_bytes(self.PerfTime, &mut into[48..48 + 8]);
        FromIntoMemory::into_bytes(self.PerfFreq, &mut into[56..56 + 8]);
        FromIntoMemory::into_bytes(self.PerfTime100nSec, &mut into[64..64 + 8]);
        FromIntoMemory::into_bytes(self.SystemNameLength, &mut into[72..72 + 4]);
        FromIntoMemory::into_bytes(self.SystemNameOffset, &mut into[76..76 + 4]);
    }
    fn size() -> usize {
        80
    }
}
pub struct PERF_DATA_HEADER {
    pub dwTotalSize: u32,
    pub dwNumCounters: u32,
    pub PerfTimeStamp: i64,
    pub PerfTime100NSec: i64,
    pub PerfFreq: i64,
    pub SystemTime: super::super::Foundation::SYSTEMTIME,
}
impl ::core::marker::Copy for PERF_DATA_HEADER {}
impl ::core::clone::Clone for PERF_DATA_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_DATA_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_DATA_HEADER")
            .field("dwTotalSize", &self.dwTotalSize)
            .field("dwNumCounters", &self.dwNumCounters)
            .field("PerfTimeStamp", &self.PerfTimeStamp)
            .field("PerfTime100NSec", &self.PerfTime100NSec)
            .field("PerfFreq", &self.PerfFreq)
            .field("SystemTime", &self.SystemTime)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PERF_DATA_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.dwTotalSize == other.dwTotalSize
            && self.dwNumCounters == other.dwNumCounters
            && self.PerfTimeStamp == other.PerfTimeStamp
            && self.PerfTime100NSec == other.PerfTime100NSec
            && self.PerfFreq == other.PerfFreq
            && self.SystemTime == other.SystemTime
    }
}
impl ::core::cmp::Eq for PERF_DATA_HEADER {}
impl FromIntoMemory for PERF_DATA_HEADER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_dwTotalSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwNumCounters = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_PerfTimeStamp = <i64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_PerfTime100NSec = <i64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_PerfFreq = <i64 as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        let f_SystemTime = <super::super::Foundation::SYSTEMTIME as FromIntoMemory>::from_bytes(
            &from[32..32 + 16],
        );
        Self {
            dwTotalSize: f_dwTotalSize,
            dwNumCounters: f_dwNumCounters,
            PerfTimeStamp: f_PerfTimeStamp,
            PerfTime100NSec: f_PerfTime100NSec,
            PerfFreq: f_PerfFreq,
            SystemTime: f_SystemTime,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.dwTotalSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwNumCounters, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.PerfTimeStamp, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.PerfTime100NSec, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.PerfFreq, &mut into[24..24 + 8]);
        FromIntoMemory::into_bytes(self.SystemTime, &mut into[32..32 + 16]);
    }
    fn size() -> usize {
        48
    }
}
pub const PERF_DATA_REVISION: u32 = 1u32;
pub const PERF_DATA_VERSION: u32 = 1u32;
pub const PERF_DELTA_BASE: u32 = 8388608u32;
pub const PERF_DELTA_COUNTER: u32 = 4194304u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PERF_DETAIL(pub u32);
pub const PERF_DETAIL_NOVICE: PERF_DETAIL = PERF_DETAIL(100u32);
pub const PERF_DETAIL_ADVANCED: PERF_DETAIL = PERF_DETAIL(200u32);
pub const PERF_DETAIL_EXPERT: PERF_DETAIL = PERF_DETAIL(300u32);
pub const PERF_DETAIL_WIZARD: PERF_DETAIL = PERF_DETAIL(400u32);
impl ::core::marker::Copy for PERF_DETAIL {}
impl ::core::clone::Clone for PERF_DETAIL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PERF_DETAIL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PERF_DETAIL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PERF_DETAIL").field(&self.0).finish()
    }
}
impl FromIntoMemory for PERF_DETAIL {
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
pub const PERF_DISPLAY_NOSHOW: u32 = 1073741824u32;
pub const PERF_DISPLAY_NO_SUFFIX: u32 = 0u32;
pub const PERF_DISPLAY_PERCENT: u32 = 536870912u32;
pub const PERF_DISPLAY_PER_SEC: u32 = 268435456u32;
pub const PERF_DISPLAY_SECONDS: u32 = 805306368u32;
pub const PERF_ENUM_INSTANCES: u32 = 3u32;
pub const PERF_FILTER: u32 = 9u32;
pub struct PERF_INSTANCE_DEFINITION {
    pub ByteLength: u32,
    pub ParentObjectTitleIndex: u32,
    pub ParentObjectInstance: u32,
    pub UniqueID: i32,
    pub NameOffset: u32,
    pub NameLength: u32,
}
impl ::core::marker::Copy for PERF_INSTANCE_DEFINITION {}
impl ::core::clone::Clone for PERF_INSTANCE_DEFINITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_INSTANCE_DEFINITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_INSTANCE_DEFINITION")
            .field("ByteLength", &self.ByteLength)
            .field("ParentObjectTitleIndex", &self.ParentObjectTitleIndex)
            .field("ParentObjectInstance", &self.ParentObjectInstance)
            .field("UniqueID", &self.UniqueID)
            .field("NameOffset", &self.NameOffset)
            .field("NameLength", &self.NameLength)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PERF_INSTANCE_DEFINITION {
    fn eq(&self, other: &Self) -> bool {
        self.ByteLength == other.ByteLength
            && self.ParentObjectTitleIndex == other.ParentObjectTitleIndex
            && self.ParentObjectInstance == other.ParentObjectInstance
            && self.UniqueID == other.UniqueID
            && self.NameOffset == other.NameOffset
            && self.NameLength == other.NameLength
    }
}
impl ::core::cmp::Eq for PERF_INSTANCE_DEFINITION {}
impl FromIntoMemory for PERF_INSTANCE_DEFINITION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_ByteLength = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ParentObjectTitleIndex = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_ParentObjectInstance = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_UniqueID = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_NameOffset = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_NameLength = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            ByteLength: f_ByteLength,
            ParentObjectTitleIndex: f_ParentObjectTitleIndex,
            ParentObjectInstance: f_ParentObjectInstance,
            UniqueID: f_UniqueID,
            NameOffset: f_NameOffset,
            NameLength: f_NameLength,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.ByteLength, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ParentObjectTitleIndex, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.ParentObjectInstance, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.UniqueID, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.NameOffset, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.NameLength, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct PERF_INSTANCE_HEADER {
    pub Size: u32,
    pub InstanceId: u32,
}
impl ::core::marker::Copy for PERF_INSTANCE_HEADER {}
impl ::core::clone::Clone for PERF_INSTANCE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_INSTANCE_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_INSTANCE_HEADER")
            .field("Size", &self.Size)
            .field("InstanceId", &self.InstanceId)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PERF_INSTANCE_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.InstanceId == other.InstanceId
    }
}
impl ::core::cmp::Eq for PERF_INSTANCE_HEADER {}
impl FromIntoMemory for PERF_INSTANCE_HEADER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Size = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_InstanceId = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            Size: f_Size,
            InstanceId: f_InstanceId,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Size, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.InstanceId, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub const PERF_INVERSE_COUNTER: u32 = 16777216u32;
pub const PERF_MAX_INSTANCE_NAME: u32 = 1024u32;
pub type PERF_MEM_ALLOC =
    StdCallFnPtr<(PtrRepr, MutPtr<::core::ffi::c_void>), MutPtr<::core::ffi::c_void>>;
pub type PERF_MEM_FREE =
    StdCallFnPtr<(MutPtr<::core::ffi::c_void>, MutPtr<::core::ffi::c_void>), ()>;
pub const PERF_METADATA_MULTIPLE_INSTANCES: i32 = -2i32;
pub const PERF_METADATA_NO_INSTANCES: i32 = -3i32;
pub const PERF_MULTI_COUNTER: u32 = 33554432u32;
pub struct PERF_MULTI_COUNTERS {
    pub dwSize: u32,
    pub dwCounters: u32,
}
impl ::core::marker::Copy for PERF_MULTI_COUNTERS {}
impl ::core::clone::Clone for PERF_MULTI_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_MULTI_COUNTERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_MULTI_COUNTERS")
            .field("dwSize", &self.dwSize)
            .field("dwCounters", &self.dwCounters)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PERF_MULTI_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwCounters == other.dwCounters
    }
}
impl ::core::cmp::Eq for PERF_MULTI_COUNTERS {}
impl FromIntoMemory for PERF_MULTI_COUNTERS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwCounters = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            dwSize: f_dwSize,
            dwCounters: f_dwCounters,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwCounters, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct PERF_MULTI_INSTANCES {
    pub dwTotalSize: u32,
    pub dwInstances: u32,
}
impl ::core::marker::Copy for PERF_MULTI_INSTANCES {}
impl ::core::clone::Clone for PERF_MULTI_INSTANCES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_MULTI_INSTANCES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_MULTI_INSTANCES")
            .field("dwTotalSize", &self.dwTotalSize)
            .field("dwInstances", &self.dwInstances)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PERF_MULTI_INSTANCES {
    fn eq(&self, other: &Self) -> bool {
        self.dwTotalSize == other.dwTotalSize && self.dwInstances == other.dwInstances
    }
}
impl ::core::cmp::Eq for PERF_MULTI_INSTANCES {}
impl FromIntoMemory for PERF_MULTI_INSTANCES {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_dwTotalSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwInstances = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            dwTotalSize: f_dwTotalSize,
            dwInstances: f_dwInstances,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.dwTotalSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwInstances, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub const PERF_NO_INSTANCES: i32 = -1i32;
pub const PERF_NO_UNIQUE_ID: i32 = -1i32;
pub const PERF_NUMBER_DECIMAL: u32 = 65536u32;
pub const PERF_NUMBER_DEC_1000: u32 = 131072u32;
pub const PERF_NUMBER_HEX: u32 = 0u32;
pub const PERF_OBJECT_TIMER: u32 = 2097152u32;
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct PERF_OBJECT_TYPE {
    pub TotalByteLength: u32,
    pub DefinitionLength: u32,
    pub HeaderLength: u32,
    pub ObjectNameTitleIndex: u32,
    pub ObjectNameTitle: u32,
    pub ObjectHelpTitleIndex: u32,
    pub ObjectHelpTitle: u32,
    pub DetailLevel: u32,
    pub NumCounters: u32,
    pub DefaultCounter: i32,
    pub NumInstances: i32,
    pub CodePage: u32,
    pub PerfTime: i64,
    pub PerfFreq: i64,
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for PERF_OBJECT_TYPE {}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for PERF_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for PERF_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_OBJECT_TYPE")
            .field("TotalByteLength", &self.TotalByteLength)
            .field("DefinitionLength", &self.DefinitionLength)
            .field("HeaderLength", &self.HeaderLength)
            .field("ObjectNameTitleIndex", &self.ObjectNameTitleIndex)
            .field("ObjectNameTitle", &self.ObjectNameTitle)
            .field("ObjectHelpTitleIndex", &self.ObjectHelpTitleIndex)
            .field("ObjectHelpTitle", &self.ObjectHelpTitle)
            .field("DetailLevel", &self.DetailLevel)
            .field("NumCounters", &self.NumCounters)
            .field("DefaultCounter", &self.DefaultCounter)
            .field("NumInstances", &self.NumInstances)
            .field("CodePage", &self.CodePage)
            .field("PerfTime", &self.PerfTime)
            .field("PerfFreq", &self.PerfFreq)
            .finish()
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for PERF_OBJECT_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.TotalByteLength == other.TotalByteLength
            && self.DefinitionLength == other.DefinitionLength
            && self.HeaderLength == other.HeaderLength
            && self.ObjectNameTitleIndex == other.ObjectNameTitleIndex
            && self.ObjectNameTitle == other.ObjectNameTitle
            && self.ObjectHelpTitleIndex == other.ObjectHelpTitleIndex
            && self.ObjectHelpTitle == other.ObjectHelpTitle
            && self.DetailLevel == other.DetailLevel
            && self.NumCounters == other.NumCounters
            && self.DefaultCounter == other.DefaultCounter
            && self.NumInstances == other.NumInstances
            && self.CodePage == other.CodePage
            && self.PerfTime == other.PerfTime
            && self.PerfFreq == other.PerfFreq
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for PERF_OBJECT_TYPE {}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for PERF_OBJECT_TYPE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 64);
        let f_TotalByteLength = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_DefinitionLength = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_HeaderLength = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ObjectNameTitleIndex = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_ObjectNameTitle = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_ObjectHelpTitleIndex = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_ObjectHelpTitle = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_DetailLevel = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_NumCounters = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_DefaultCounter = <i32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_NumInstances = <i32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_CodePage = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_PerfTime = <i64 as FromIntoMemory>::from_bytes(&from[48..48 + 8]);
        let f_PerfFreq = <i64 as FromIntoMemory>::from_bytes(&from[56..56 + 8]);
        Self {
            TotalByteLength: f_TotalByteLength,
            DefinitionLength: f_DefinitionLength,
            HeaderLength: f_HeaderLength,
            ObjectNameTitleIndex: f_ObjectNameTitleIndex,
            ObjectNameTitle: f_ObjectNameTitle,
            ObjectHelpTitleIndex: f_ObjectHelpTitleIndex,
            ObjectHelpTitle: f_ObjectHelpTitle,
            DetailLevel: f_DetailLevel,
            NumCounters: f_NumCounters,
            DefaultCounter: f_DefaultCounter,
            NumInstances: f_NumInstances,
            CodePage: f_CodePage,
            PerfTime: f_PerfTime,
            PerfFreq: f_PerfFreq,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 64);
        FromIntoMemory::into_bytes(self.TotalByteLength, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.DefinitionLength, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.HeaderLength, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ObjectNameTitleIndex, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.ObjectNameTitle, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.ObjectHelpTitleIndex, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.ObjectHelpTitle, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.DetailLevel, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.NumCounters, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.DefaultCounter, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.NumInstances, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.CodePage, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.PerfTime, &mut into[48..48 + 8]);
        FromIntoMemory::into_bytes(self.PerfFreq, &mut into[56..56 + 8]);
    }
    fn size() -> usize {
        64
    }
}
pub struct PERF_OBJECT_TYPE {
    pub TotalByteLength: u32,
    pub DefinitionLength: u32,
    pub HeaderLength: u32,
    pub ObjectNameTitleIndex: u32,
    pub ObjectNameTitle: PWSTR,
    pub ObjectHelpTitleIndex: u32,
    pub ObjectHelpTitle: PWSTR,
    pub DetailLevel: u32,
    pub NumCounters: u32,
    pub DefaultCounter: i32,
    pub NumInstances: i32,
    pub CodePage: u32,
    pub PerfTime: i64,
    pub PerfFreq: i64,
}
impl ::core::marker::Copy for PERF_OBJECT_TYPE {}
impl ::core::clone::Clone for PERF_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_OBJECT_TYPE")
            .field("TotalByteLength", &self.TotalByteLength)
            .field("DefinitionLength", &self.DefinitionLength)
            .field("HeaderLength", &self.HeaderLength)
            .field("ObjectNameTitleIndex", &self.ObjectNameTitleIndex)
            .field("ObjectNameTitle", &self.ObjectNameTitle)
            .field("ObjectHelpTitleIndex", &self.ObjectHelpTitleIndex)
            .field("ObjectHelpTitle", &self.ObjectHelpTitle)
            .field("DetailLevel", &self.DetailLevel)
            .field("NumCounters", &self.NumCounters)
            .field("DefaultCounter", &self.DefaultCounter)
            .field("NumInstances", &self.NumInstances)
            .field("CodePage", &self.CodePage)
            .field("PerfTime", &self.PerfTime)
            .field("PerfFreq", &self.PerfFreq)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PERF_OBJECT_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.TotalByteLength == other.TotalByteLength
            && self.DefinitionLength == other.DefinitionLength
            && self.HeaderLength == other.HeaderLength
            && self.ObjectNameTitleIndex == other.ObjectNameTitleIndex
            && self.ObjectNameTitle == other.ObjectNameTitle
            && self.ObjectHelpTitleIndex == other.ObjectHelpTitleIndex
            && self.ObjectHelpTitle == other.ObjectHelpTitle
            && self.DetailLevel == other.DetailLevel
            && self.NumCounters == other.NumCounters
            && self.DefaultCounter == other.DefaultCounter
            && self.NumInstances == other.NumInstances
            && self.CodePage == other.CodePage
            && self.PerfTime == other.PerfTime
            && self.PerfFreq == other.PerfFreq
    }
}
impl ::core::cmp::Eq for PERF_OBJECT_TYPE {}
impl FromIntoMemory for PERF_OBJECT_TYPE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 64);
        let f_TotalByteLength = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_DefinitionLength = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_HeaderLength = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ObjectNameTitleIndex = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_ObjectNameTitle = <PWSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_ObjectHelpTitleIndex = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_ObjectHelpTitle = <PWSTR as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_DetailLevel = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_NumCounters = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_DefaultCounter = <i32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_NumInstances = <i32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_CodePage = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_PerfTime = <i64 as FromIntoMemory>::from_bytes(&from[48..48 + 8]);
        let f_PerfFreq = <i64 as FromIntoMemory>::from_bytes(&from[56..56 + 8]);
        Self {
            TotalByteLength: f_TotalByteLength,
            DefinitionLength: f_DefinitionLength,
            HeaderLength: f_HeaderLength,
            ObjectNameTitleIndex: f_ObjectNameTitleIndex,
            ObjectNameTitle: f_ObjectNameTitle,
            ObjectHelpTitleIndex: f_ObjectHelpTitleIndex,
            ObjectHelpTitle: f_ObjectHelpTitle,
            DetailLevel: f_DetailLevel,
            NumCounters: f_NumCounters,
            DefaultCounter: f_DefaultCounter,
            NumInstances: f_NumInstances,
            CodePage: f_CodePage,
            PerfTime: f_PerfTime,
            PerfFreq: f_PerfFreq,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 64);
        FromIntoMemory::into_bytes(self.TotalByteLength, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.DefinitionLength, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.HeaderLength, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ObjectNameTitleIndex, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.ObjectNameTitle, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.ObjectHelpTitleIndex, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.ObjectHelpTitle, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.DetailLevel, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.NumCounters, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.DefaultCounter, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.NumInstances, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.CodePage, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.PerfTime, &mut into[48..48 + 8]);
        FromIntoMemory::into_bytes(self.PerfFreq, &mut into[56..56 + 8]);
    }
    fn size() -> usize {
        64
    }
}
pub struct PERF_PROVIDER_CONTEXT {
    pub ContextSize: u32,
    pub Reserved: u32,
    pub ControlCallback: PERFLIBREQUEST,
    pub MemAllocRoutine: PERF_MEM_ALLOC,
    pub MemFreeRoutine: PERF_MEM_FREE,
    pub pMemContext: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for PERF_PROVIDER_CONTEXT {}
impl ::core::clone::Clone for PERF_PROVIDER_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_PROVIDER_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_PROVIDER_CONTEXT")
            .field("ContextSize", &self.ContextSize)
            .field("Reserved", &self.Reserved)
            .field("ControlCallback", &self.ControlCallback)
            .field("MemAllocRoutine", &self.MemAllocRoutine)
            .field("MemFreeRoutine", &self.MemFreeRoutine)
            .field("pMemContext", &self.pMemContext)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PERF_PROVIDER_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.ContextSize == other.ContextSize
            && self.Reserved == other.Reserved
            && self.ControlCallback == other.ControlCallback
            && self.MemAllocRoutine == other.MemAllocRoutine
            && self.MemFreeRoutine == other.MemFreeRoutine
            && self.pMemContext == other.pMemContext
    }
}
impl ::core::cmp::Eq for PERF_PROVIDER_CONTEXT {}
impl FromIntoMemory for PERF_PROVIDER_CONTEXT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_ContextSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Reserved = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_ControlCallback = <PERFLIBREQUEST as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_MemAllocRoutine = <PERF_MEM_ALLOC as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_MemFreeRoutine = <PERF_MEM_FREE as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_pMemContext =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            ContextSize: f_ContextSize,
            Reserved: f_Reserved,
            ControlCallback: f_ControlCallback,
            MemAllocRoutine: f_MemAllocRoutine,
            MemFreeRoutine: f_MemFreeRoutine,
            pMemContext: f_pMemContext,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.ContextSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.ControlCallback, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.MemAllocRoutine, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.MemFreeRoutine, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.pMemContext, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub const PERF_PROVIDER_DRIVER: u32 = 2u32;
pub const PERF_PROVIDER_KERNEL_MODE: u32 = 1u32;
pub const PERF_PROVIDER_USER_MODE: u32 = 0u32;
pub const PERF_REMOVE_COUNTER: u32 = 2u32;
pub const PERF_SIZE_DWORD: u32 = 0u32;
pub const PERF_SIZE_LARGE: u32 = 256u32;
pub const PERF_SIZE_VARIABLE_LEN: u32 = 768u32;
pub const PERF_SIZE_ZERO: u32 = 512u32;
pub struct PERF_STRING_BUFFER_HEADER {
    pub dwSize: u32,
    pub dwCounters: u32,
}
impl ::core::marker::Copy for PERF_STRING_BUFFER_HEADER {}
impl ::core::clone::Clone for PERF_STRING_BUFFER_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_STRING_BUFFER_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_STRING_BUFFER_HEADER")
            .field("dwSize", &self.dwSize)
            .field("dwCounters", &self.dwCounters)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PERF_STRING_BUFFER_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwCounters == other.dwCounters
    }
}
impl ::core::cmp::Eq for PERF_STRING_BUFFER_HEADER {}
impl FromIntoMemory for PERF_STRING_BUFFER_HEADER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwCounters = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            dwSize: f_dwSize,
            dwCounters: f_dwCounters,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwCounters, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct PERF_STRING_COUNTER_HEADER {
    pub dwCounterId: u32,
    pub dwOffset: u32,
}
impl ::core::marker::Copy for PERF_STRING_COUNTER_HEADER {}
impl ::core::clone::Clone for PERF_STRING_COUNTER_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERF_STRING_COUNTER_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_STRING_COUNTER_HEADER")
            .field("dwCounterId", &self.dwCounterId)
            .field("dwOffset", &self.dwOffset)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PERF_STRING_COUNTER_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.dwCounterId == other.dwCounterId && self.dwOffset == other.dwOffset
    }
}
impl ::core::cmp::Eq for PERF_STRING_COUNTER_HEADER {}
impl FromIntoMemory for PERF_STRING_COUNTER_HEADER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_dwCounterId = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwOffset = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            dwCounterId: f_dwCounterId,
            dwOffset: f_dwOffset,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.dwCounterId, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwOffset, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub const PERF_TEXT_ASCII: u32 = 65536u32;
pub const PERF_TEXT_UNICODE: u32 = 0u32;
pub const PERF_TIMER_100NS: u32 = 1048576u32;
pub const PERF_TIMER_TICK: u32 = 0u32;
pub const PERF_TYPE_COUNTER: u32 = 1024u32;
pub const PERF_TYPE_NUMBER: u32 = 0u32;
pub const PERF_TYPE_TEXT: u32 = 2048u32;
pub const PERF_TYPE_ZERO: u32 = 3072u32;
pub const PERF_WILDCARD_COUNTER: u32 = 4294967295u32;
pub const PERF_WILDCARD_INSTANCE: &'static str = "*";
pub type PLA_CABEXTRACT_CALLBACK = StdCallFnPtr<(PCWSTR, MutPtr<::core::ffi::c_void>), ()>;
pub const PLA_CAPABILITY_AUTOLOGGER: u32 = 32u32;
pub const PLA_CAPABILITY_LEGACY_SESSION: u32 = 8u32;
pub const PLA_CAPABILITY_LEGACY_SVC: u32 = 16u32;
pub const PLA_CAPABILITY_LOCAL: u32 = 268435456u32;
pub const PLA_CAPABILITY_V1_SESSION: u32 = 2u32;
pub const PLA_CAPABILITY_V1_SVC: u32 = 1u32;
pub const PLA_CAPABILITY_V1_SYSTEM: u32 = 4u32;
pub type PM_CLOSE_PROC = StdCallFnPtr<(), u32>;
pub type PM_COLLECT_PROC = StdCallFnPtr<
    (
        PCWSTR,
        MutPtr<ConstPtr<::core::ffi::c_void>>,
        MutPtr<u32>,
        MutPtr<u32>,
    ),
    u32,
>;
pub type PM_OPEN_PROC = StdCallFnPtr<(PCWSTR,), u32>;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PerfCounterDataType(pub i32);
pub const PERF_ERROR_RETURN: PerfCounterDataType = PerfCounterDataType(0i32);
pub const PERF_SINGLE_COUNTER: PerfCounterDataType = PerfCounterDataType(1i32);
pub const PERF_MULTIPLE_COUNTERS: PerfCounterDataType = PerfCounterDataType(2i32);
pub const PERF_MULTIPLE_INSTANCES: PerfCounterDataType = PerfCounterDataType(4i32);
pub const PERF_COUNTERSET: PerfCounterDataType = PerfCounterDataType(6i32);
impl ::core::marker::Copy for PerfCounterDataType {}
impl ::core::clone::Clone for PerfCounterDataType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PerfCounterDataType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PerfCounterDataType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerfCounterDataType").field(&self.0).finish()
    }
}
impl FromIntoMemory for PerfCounterDataType {
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
pub struct PerfProviderHandle(pub PtrDiffRepr);
impl PerfProviderHandle {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PerfProviderHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for PerfProviderHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for PerfProviderHandle {}
impl ::core::hash::Hash for PerfProviderHandle {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for PerfProviderHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerfProviderHandle").field(&self.0).finish()
    }
}
impl FromIntoMemory for PerfProviderHandle {
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
pub struct PerfQueryHandle(pub PtrDiffRepr);
impl PerfQueryHandle {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PerfQueryHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for PerfQueryHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for PerfQueryHandle {}
impl ::core::hash::Hash for PerfQueryHandle {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for PerfQueryHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerfQueryHandle").field(&self.0).finish()
    }
}
impl FromIntoMemory for PerfQueryHandle {
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
pub struct PerfRegInfoType(pub i32);
pub const PERF_REG_COUNTERSET_STRUCT: PerfRegInfoType = PerfRegInfoType(1i32);
pub const PERF_REG_COUNTER_STRUCT: PerfRegInfoType = PerfRegInfoType(2i32);
pub const PERF_REG_COUNTERSET_NAME_STRING: PerfRegInfoType = PerfRegInfoType(3i32);
pub const PERF_REG_COUNTERSET_HELP_STRING: PerfRegInfoType = PerfRegInfoType(4i32);
pub const PERF_REG_COUNTER_NAME_STRINGS: PerfRegInfoType = PerfRegInfoType(5i32);
pub const PERF_REG_COUNTER_HELP_STRINGS: PerfRegInfoType = PerfRegInfoType(6i32);
pub const PERF_REG_PROVIDER_NAME: PerfRegInfoType = PerfRegInfoType(7i32);
pub const PERF_REG_PROVIDER_GUID: PerfRegInfoType = PerfRegInfoType(8i32);
pub const PERF_REG_COUNTERSET_ENGLISH_NAME: PerfRegInfoType = PerfRegInfoType(9i32);
pub const PERF_REG_COUNTER_ENGLISH_NAMES: PerfRegInfoType = PerfRegInfoType(10i32);
impl ::core::marker::Copy for PerfRegInfoType {}
impl ::core::clone::Clone for PerfRegInfoType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PerfRegInfoType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PerfRegInfoType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerfRegInfoType").field(&self.0).finish()
    }
}
impl FromIntoMemory for PerfRegInfoType {
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
pub struct REAL_TIME_DATA_SOURCE_ID_FLAGS(pub u32);
pub const DATA_SOURCE_REGISTRY: REAL_TIME_DATA_SOURCE_ID_FLAGS =
    REAL_TIME_DATA_SOURCE_ID_FLAGS(1u32);
pub const DATA_SOURCE_WBEM: REAL_TIME_DATA_SOURCE_ID_FLAGS = REAL_TIME_DATA_SOURCE_ID_FLAGS(4u32);
impl ::core::marker::Copy for REAL_TIME_DATA_SOURCE_ID_FLAGS {}
impl ::core::clone::Clone for REAL_TIME_DATA_SOURCE_ID_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REAL_TIME_DATA_SOURCE_ID_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for REAL_TIME_DATA_SOURCE_ID_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REAL_TIME_DATA_SOURCE_ID_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for REAL_TIME_DATA_SOURCE_ID_FLAGS {
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
pub struct ReportValueTypeConstants(pub i32);
pub const sysmonDefaultValue: ReportValueTypeConstants = ReportValueTypeConstants(0i32);
pub const sysmonCurrentValue: ReportValueTypeConstants = ReportValueTypeConstants(1i32);
pub const sysmonAverage: ReportValueTypeConstants = ReportValueTypeConstants(2i32);
pub const sysmonMinimum: ReportValueTypeConstants = ReportValueTypeConstants(3i32);
pub const sysmonMaximum: ReportValueTypeConstants = ReportValueTypeConstants(4i32);
impl ::core::marker::Copy for ReportValueTypeConstants {}
impl ::core::clone::Clone for ReportValueTypeConstants {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ReportValueTypeConstants {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ReportValueTypeConstants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ReportValueTypeConstants")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ReportValueTypeConstants {
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
pub struct ResourcePolicy(pub i32);
pub const plaDeleteLargest: ResourcePolicy = ResourcePolicy(0i32);
pub const plaDeleteOldest: ResourcePolicy = ResourcePolicy(1i32);
impl ::core::marker::Copy for ResourcePolicy {}
impl ::core::clone::Clone for ResourcePolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ResourcePolicy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ResourcePolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourcePolicy").field(&self.0).finish()
    }
}
impl FromIntoMemory for ResourcePolicy {
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
pub const S_PDH: crate::core::GUID =
    crate::core::GUID::from_u128(0x04d66358_c4a1_419b_8023_23b73902de2c);
pub const ServerDataCollectorSet: crate::core::GUID =
    crate::core::GUID::from_u128(0x03837531_098b_11d8_9414_505054503030);
pub const ServerDataCollectorSetCollection: crate::core::GUID =
    crate::core::GUID::from_u128(0x03837532_098b_11d8_9414_505054503030);
pub const SourcePropPage: crate::core::GUID =
    crate::core::GUID::from_u128(0x0cf32aa1_7571_11d0_93c4_00aa00a3ddea);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StreamMode(pub i32);
pub const plaFile: StreamMode = StreamMode(1i32);
pub const plaRealTime: StreamMode = StreamMode(2i32);
pub const plaBoth: StreamMode = StreamMode(3i32);
pub const plaBuffering: StreamMode = StreamMode(4i32);
impl ::core::marker::Copy for StreamMode {}
impl ::core::clone::Clone for StreamMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StreamMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for StreamMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreamMode").field(&self.0).finish()
    }
}
impl FromIntoMemory for StreamMode {
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
pub struct SysmonBatchReason(pub i32);
pub const sysmonBatchNone: SysmonBatchReason = SysmonBatchReason(0i32);
pub const sysmonBatchAddFiles: SysmonBatchReason = SysmonBatchReason(1i32);
pub const sysmonBatchAddCounters: SysmonBatchReason = SysmonBatchReason(2i32);
pub const sysmonBatchAddFilesAutoCounters: SysmonBatchReason = SysmonBatchReason(3i32);
impl ::core::marker::Copy for SysmonBatchReason {}
impl ::core::clone::Clone for SysmonBatchReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SysmonBatchReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SysmonBatchReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SysmonBatchReason").field(&self.0).finish()
    }
}
impl FromIntoMemory for SysmonBatchReason {
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
pub struct SysmonDataType(pub i32);
pub const sysmonDataAvg: SysmonDataType = SysmonDataType(1i32);
pub const sysmonDataMin: SysmonDataType = SysmonDataType(2i32);
pub const sysmonDataMax: SysmonDataType = SysmonDataType(3i32);
pub const sysmonDataTime: SysmonDataType = SysmonDataType(4i32);
pub const sysmonDataCount: SysmonDataType = SysmonDataType(5i32);
impl ::core::marker::Copy for SysmonDataType {}
impl ::core::clone::Clone for SysmonDataType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SysmonDataType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SysmonDataType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SysmonDataType").field(&self.0).finish()
    }
}
impl FromIntoMemory for SysmonDataType {
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
pub struct SysmonFileType(pub i32);
pub const sysmonFileHtml: SysmonFileType = SysmonFileType(1i32);
pub const sysmonFileReport: SysmonFileType = SysmonFileType(2i32);
pub const sysmonFileCsv: SysmonFileType = SysmonFileType(3i32);
pub const sysmonFileTsv: SysmonFileType = SysmonFileType(4i32);
pub const sysmonFileBlg: SysmonFileType = SysmonFileType(5i32);
pub const sysmonFileRetiredBlg: SysmonFileType = SysmonFileType(6i32);
pub const sysmonFileGif: SysmonFileType = SysmonFileType(7i32);
impl ::core::marker::Copy for SysmonFileType {}
impl ::core::clone::Clone for SysmonFileType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SysmonFileType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SysmonFileType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SysmonFileType").field(&self.0).finish()
    }
}
impl FromIntoMemory for SysmonFileType {
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
pub const SystemDataCollectorSet: crate::core::GUID =
    crate::core::GUID::from_u128(0x03837546_098b_11d8_9414_505054503030);
pub const SystemDataCollectorSetCollection: crate::core::GUID =
    crate::core::GUID::from_u128(0x03837547_098b_11d8_9414_505054503030);
pub const SystemMonitor: crate::core::GUID =
    crate::core::GUID::from_u128(0xc4d2d8e0_d1dd_11ce_940f_008029004347);
pub const SystemMonitor2: crate::core::GUID =
    crate::core::GUID::from_u128(0x7f30578c_5f38_4612_acfe_6ed04c7b7af8);
pub const TraceDataProvider: crate::core::GUID =
    crate::core::GUID::from_u128(0x03837513_098b_11d8_9414_505054503030);
pub const TraceDataProviderCollection: crate::core::GUID =
    crate::core::GUID::from_u128(0x03837511_098b_11d8_9414_505054503030);
pub const TraceSession: crate::core::GUID =
    crate::core::GUID::from_u128(0x0383751c_098b_11d8_9414_505054503030);
pub const TraceSessionCollection: crate::core::GUID =
    crate::core::GUID::from_u128(0x03837530_098b_11d8_9414_505054503030);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ValueMapType(pub i32);
pub const plaIndex: ValueMapType = ValueMapType(1i32);
pub const plaFlag: ValueMapType = ValueMapType(2i32);
pub const plaFlagArray: ValueMapType = ValueMapType(3i32);
pub const plaValidation: ValueMapType = ValueMapType(4i32);
impl ::core::marker::Copy for ValueMapType {}
impl ::core::clone::Clone for ValueMapType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ValueMapType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ValueMapType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ValueMapType").field(&self.0).finish()
    }
}
impl FromIntoMemory for ValueMapType {
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
pub const WINPERF_LOG_DEBUG: u32 = 2u32;
pub const WINPERF_LOG_NONE: u32 = 0u32;
pub const WINPERF_LOG_USER: u32 = 1u32;
pub const WINPERF_LOG_VERBOSE: u32 = 3u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WeekDays(pub i32);
pub const plaRunOnce: WeekDays = WeekDays(0i32);
pub const plaSunday: WeekDays = WeekDays(1i32);
pub const plaMonday: WeekDays = WeekDays(2i32);
pub const plaTuesday: WeekDays = WeekDays(4i32);
pub const plaWednesday: WeekDays = WeekDays(8i32);
pub const plaThursday: WeekDays = WeekDays(16i32);
pub const plaFriday: WeekDays = WeekDays(32i32);
pub const plaSaturday: WeekDays = WeekDays(64i32);
pub const plaEveryday: WeekDays = WeekDays(127i32);
impl ::core::marker::Copy for WeekDays {}
impl ::core::clone::Clone for WeekDays {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WeekDays {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WeekDays {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WeekDays").field(&self.0).finish()
    }
}
impl FromIntoMemory for WeekDays {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct _ICounterItemUnion(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait _ICounterItemUnion_Trait: crate::core::IUnknown_Trait {
    fn get_Value(&self, pdbl_value: MutPtr<f64>) -> crate::core::HRESULT {
        todo!("get_Value")
    }
    fn put_Color(&self, color: u32) -> crate::core::HRESULT {
        todo!("put_Color")
    }
    fn get_Color(&self, p_color: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_Color")
    }
    fn put_Width(&self, i_width: i32) -> crate::core::HRESULT {
        todo!("put_Width")
    }
    fn get_Width(&self, pi_value: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("get_Width")
    }
    fn put_LineStyle(&self, i_line_style: i32) -> crate::core::HRESULT {
        todo!("put_LineStyle")
    }
    fn get_LineStyle(&self, pi_value: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("get_LineStyle")
    }
    fn put_ScaleFactor(&self, i_scale: i32) -> crate::core::HRESULT {
        todo!("put_ScaleFactor")
    }
    fn get_ScaleFactor(&self, pi_value: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("get_ScaleFactor")
    }
    fn get_Path(&self, pstr_value: MutPtr<super::super::Foundation::BSTR>) -> crate::core::HRESULT {
        todo!("get_Path")
    }
    fn GetValue(&self, value: MutPtr<f64>, status: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("GetValue")
    }
    fn GetStatistics(
        &self,
        max: MutPtr<f64>,
        min: MutPtr<f64>,
        avg: MutPtr<f64>,
        status: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("GetStatistics")
    }
    fn put_Selected(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_Selected")
    }
    fn get_Selected(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_Selected")
    }
    fn put_Visible(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_Visible")
    }
    fn get_Visible(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_Visible")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetDataAt(
        &self,
        i_index: i32,
        i_which: SysmonDataType,
        p_variant: MutPtr<super::Com::VARIANT>,
    ) -> crate::core::HRESULT {
        todo!("GetDataAt")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for _ICounterItemUnion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for _ICounterItemUnion {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for _ICounterItemUnion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for _ICounterItemUnion {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for _ICounterItemUnion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_ICounterItemUnion").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for _ICounterItemUnion {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for _ICounterItemUnion {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xde1a6b74_9182_4c41_8e2c_24c2cd30ee83);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct _ISystemMonitorUnion(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait _ISystemMonitorUnion_Trait: crate::core::IUnknown_Trait {
    fn get_Appearance(&self, i_appearance: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("get_Appearance")
    }
    fn put_Appearance(&self, i_appearance: i32) -> crate::core::HRESULT {
        todo!("put_Appearance")
    }
    fn get_BackColor(&self, p_color: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_BackColor")
    }
    fn put_BackColor(&self, color: u32) -> crate::core::HRESULT {
        todo!("put_BackColor")
    }
    fn get_BorderStyle(&self, i_border_style: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("get_BorderStyle")
    }
    fn put_BorderStyle(&self, i_border_style: i32) -> crate::core::HRESULT {
        todo!("put_BorderStyle")
    }
    fn get_ForeColor(&self, p_color: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_ForeColor")
    }
    fn put_ForeColor(&self, color: u32) -> crate::core::HRESULT {
        todo!("put_ForeColor")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_Font(&self, pp_font: MutPtr<super::Ole::IFontDisp>) -> crate::core::HRESULT {
        todo!("get_Font")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn putref_Font(&self, p_font: super::Ole::IFontDisp) -> crate::core::HRESULT {
        todo!("putref_Font")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_Counters(&self, pp_i_counters: MutPtr<ICounters>) -> crate::core::HRESULT {
        todo!("get_Counters")
    }
    fn put_ShowVerticalGrid(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_ShowVerticalGrid")
    }
    fn get_ShowVerticalGrid(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_ShowVerticalGrid")
    }
    fn put_ShowHorizontalGrid(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_ShowHorizontalGrid")
    }
    fn get_ShowHorizontalGrid(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_ShowHorizontalGrid")
    }
    fn put_ShowLegend(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_ShowLegend")
    }
    fn get_ShowLegend(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_ShowLegend")
    }
    fn put_ShowScaleLabels(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_ShowScaleLabels")
    }
    fn get_ShowScaleLabels(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_ShowScaleLabels")
    }
    fn put_ShowValueBar(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_ShowValueBar")
    }
    fn get_ShowValueBar(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_ShowValueBar")
    }
    fn put_MaximumScale(&self, i_value: i32) -> crate::core::HRESULT {
        todo!("put_MaximumScale")
    }
    fn get_MaximumScale(&self, pi_value: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("get_MaximumScale")
    }
    fn put_MinimumScale(&self, i_value: i32) -> crate::core::HRESULT {
        todo!("put_MinimumScale")
    }
    fn get_MinimumScale(&self, pi_value: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("get_MinimumScale")
    }
    fn put_UpdateInterval(&self, f_value: f32) -> crate::core::HRESULT {
        todo!("put_UpdateInterval")
    }
    fn get_UpdateInterval(&self, pf_value: MutPtr<f32>) -> crate::core::HRESULT {
        todo!("get_UpdateInterval")
    }
    fn put_DisplayType(&self, e_display_type: DisplayTypeConstants) -> crate::core::HRESULT {
        todo!("put_DisplayType")
    }
    fn get_DisplayType(
        &self,
        pe_display_type: MutPtr<DisplayTypeConstants>,
    ) -> crate::core::HRESULT {
        todo!("get_DisplayType")
    }
    fn put_ManualUpdate(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_ManualUpdate")
    }
    fn get_ManualUpdate(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_ManualUpdate")
    }
    fn put_GraphTitle(&self, bs_title: super::super::Foundation::BSTR) -> crate::core::HRESULT {
        todo!("put_GraphTitle")
    }
    fn get_GraphTitle(
        &self,
        pbs_title: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_GraphTitle")
    }
    fn put_YAxisLabel(&self, bs_title: super::super::Foundation::BSTR) -> crate::core::HRESULT {
        todo!("put_YAxisLabel")
    }
    fn get_YAxisLabel(
        &self,
        pbs_title: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_YAxisLabel")
    }
    fn CollectSample(&self) -> crate::core::HRESULT {
        todo!("CollectSample")
    }
    fn UpdateGraph(&self) -> crate::core::HRESULT {
        todo!("UpdateGraph")
    }
    fn BrowseCounters(&self) -> crate::core::HRESULT {
        todo!("BrowseCounters")
    }
    fn DisplayProperties(&self) -> crate::core::HRESULT {
        todo!("DisplayProperties")
    }
    fn Counter(&self, i_index: i32, pp_i_counter: MutPtr<ICounterItem>) -> crate::core::HRESULT {
        todo!("Counter")
    }
    fn AddCounter(
        &self,
        bs_path: super::super::Foundation::BSTR,
        pp_i_counter: MutPtr<ICounterItem>,
    ) -> crate::core::HRESULT {
        todo!("AddCounter")
    }
    fn DeleteCounter(&self, p_ctr: ICounterItem) -> crate::core::HRESULT {
        todo!("DeleteCounter")
    }
    fn get_BackColorCtl(&self, p_color: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_BackColorCtl")
    }
    fn put_BackColorCtl(&self, color: u32) -> crate::core::HRESULT {
        todo!("put_BackColorCtl")
    }
    fn put_LogFileName(
        &self,
        bs_file_name: super::super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("put_LogFileName")
    }
    fn get_LogFileName(
        &self,
        bs_file_name: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_LogFileName")
    }
    fn put_LogViewStart(&self, start_time: f64) -> crate::core::HRESULT {
        todo!("put_LogViewStart")
    }
    fn get_LogViewStart(&self, start_time: MutPtr<f64>) -> crate::core::HRESULT {
        todo!("get_LogViewStart")
    }
    fn put_LogViewStop(&self, stop_time: f64) -> crate::core::HRESULT {
        todo!("put_LogViewStop")
    }
    fn get_LogViewStop(&self, stop_time: MutPtr<f64>) -> crate::core::HRESULT {
        todo!("get_LogViewStop")
    }
    fn get_GridColor(&self, p_color: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_GridColor")
    }
    fn put_GridColor(&self, color: u32) -> crate::core::HRESULT {
        todo!("put_GridColor")
    }
    fn get_TimeBarColor(&self, p_color: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_TimeBarColor")
    }
    fn put_TimeBarColor(&self, color: u32) -> crate::core::HRESULT {
        todo!("put_TimeBarColor")
    }
    fn get_Highlight(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_Highlight")
    }
    fn put_Highlight(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_Highlight")
    }
    fn get_ShowToolbar(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_ShowToolbar")
    }
    fn put_ShowToolbar(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_ShowToolbar")
    }
    fn Paste(&self) -> crate::core::HRESULT {
        todo!("Paste")
    }
    fn Copy(&self) -> crate::core::HRESULT {
        todo!("Copy")
    }
    fn Reset(&self) -> crate::core::HRESULT {
        todo!("Reset")
    }
    fn put_ReadOnly(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_ReadOnly")
    }
    fn get_ReadOnly(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_ReadOnly")
    }
    fn put_ReportValueType(
        &self,
        e_report_value_type: ReportValueTypeConstants,
    ) -> crate::core::HRESULT {
        todo!("put_ReportValueType")
    }
    fn get_ReportValueType(
        &self,
        pe_report_value_type: MutPtr<ReportValueTypeConstants>,
    ) -> crate::core::HRESULT {
        todo!("get_ReportValueType")
    }
    fn put_MonitorDuplicateInstances(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_MonitorDuplicateInstances")
    }
    fn get_MonitorDuplicateInstances(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_MonitorDuplicateInstances")
    }
    fn put_DisplayFilter(&self, i_value: i32) -> crate::core::HRESULT {
        todo!("put_DisplayFilter")
    }
    fn get_DisplayFilter(&self, pi_value: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("get_DisplayFilter")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_LogFiles(&self, pp_i_log_files: MutPtr<ILogFiles>) -> crate::core::HRESULT {
        todo!("get_LogFiles")
    }
    fn put_DataSourceType(
        &self,
        e_data_source_type: DataSourceTypeConstants,
    ) -> crate::core::HRESULT {
        todo!("put_DataSourceType")
    }
    fn get_DataSourceType(
        &self,
        pe_data_source_type: MutPtr<DataSourceTypeConstants>,
    ) -> crate::core::HRESULT {
        todo!("get_DataSourceType")
    }
    fn put_SqlDsnName(
        &self,
        bs_sql_dsn_name: super::super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("put_SqlDsnName")
    }
    fn get_SqlDsnName(
        &self,
        bs_sql_dsn_name: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_SqlDsnName")
    }
    fn put_SqlLogSetName(
        &self,
        bs_sql_log_set_name: super::super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("put_SqlLogSetName")
    }
    fn get_SqlLogSetName(
        &self,
        bs_sql_log_set_name: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("get_SqlLogSetName")
    }
    fn put_EnableDigitGrouping(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_EnableDigitGrouping")
    }
    fn get_EnableDigitGrouping(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_EnableDigitGrouping")
    }
    fn put_EnableToolTips(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_EnableToolTips")
    }
    fn get_EnableToolTips(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_EnableToolTips")
    }
    fn put_ShowTimeAxisLabels(&self, b_state: i16) -> crate::core::HRESULT {
        todo!("put_ShowTimeAxisLabels")
    }
    fn get_ShowTimeAxisLabels(&self, pb_state: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_ShowTimeAxisLabels")
    }
    fn put_ChartScroll(&self, b_scroll: i16) -> crate::core::HRESULT {
        todo!("put_ChartScroll")
    }
    fn get_ChartScroll(&self, pb_scroll: MutPtr<i16>) -> crate::core::HRESULT {
        todo!("get_ChartScroll")
    }
    fn put_DataPointCount(&self, i_new_count: i32) -> crate::core::HRESULT {
        todo!("put_DataPointCount")
    }
    fn get_DataPointCount(&self, pi_data_point_count: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("get_DataPointCount")
    }
    fn ScaleToFit(&self, b_selected_counters_only: i16) -> crate::core::HRESULT {
        todo!("ScaleToFit")
    }
    fn SaveAs(
        &self,
        bstr_file_name: super::super::Foundation::BSTR,
        e_sysmon_file_type: SysmonFileType,
    ) -> crate::core::HRESULT {
        todo!("SaveAs")
    }
    fn Relog(
        &self,
        bstr_file_name: super::super::Foundation::BSTR,
        e_sysmon_file_type: SysmonFileType,
        i_filter: i32,
    ) -> crate::core::HRESULT {
        todo!("Relog")
    }
    fn ClearData(&self) -> crate::core::HRESULT {
        todo!("ClearData")
    }
    fn get_LogSourceStartTime(&self, p_date: MutPtr<f64>) -> crate::core::HRESULT {
        todo!("get_LogSourceStartTime")
    }
    fn get_LogSourceStopTime(&self, p_date: MutPtr<f64>) -> crate::core::HRESULT {
        todo!("get_LogSourceStopTime")
    }
    fn SetLogViewRange(&self, start_time: f64, stop_time: f64) -> crate::core::HRESULT {
        todo!("SetLogViewRange")
    }
    fn GetLogViewRange(
        &self,
        start_time: MutPtr<f64>,
        stop_time: MutPtr<f64>,
    ) -> crate::core::HRESULT {
        todo!("GetLogViewRange")
    }
    fn BatchingLock(&self, f_lock: i16, e_batch_reason: SysmonBatchReason) -> crate::core::HRESULT {
        todo!("BatchingLock")
    }
    fn LoadSettings(
        &self,
        bstr_setting_file_name: super::super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("LoadSettings")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for _ISystemMonitorUnion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for _ISystemMonitorUnion {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for _ISystemMonitorUnion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for _ISystemMonitorUnion {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for _ISystemMonitorUnion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_ISystemMonitorUnion")
            .field(&self.0)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for _ISystemMonitorUnion {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for _ISystemMonitorUnion {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xc8a77338_265f_4de5_aa25_c7da1ce5a8f4);
}
pub trait Api {
    #[doc = "PerfAddCounters from ADVAPI32"]
    fn PerfAddCounters(
        &self,
        h_query: PerfQueryHandle,
        p_counters: ConstPtr<PERF_COUNTER_IDENTIFIER>,
        cb_counters: u32,
    ) -> u32 {
        todo!("PerfAddCounters")
    }
    #[doc = "PerfCloseQueryHandle from ADVAPI32"]
    fn PerfCloseQueryHandle(&self, h_query: super::super::Foundation::HANDLE) -> u32 {
        todo!("PerfCloseQueryHandle")
    }
    #[doc = "PerfCreateInstance from ADVAPI32"]
    fn PerfCreateInstance(
        &self,
        provider_handle: PerfProviderHandle,
        counter_set_guid: ConstPtr<crate::core::GUID>,
        name: PCWSTR,
        id: u32,
    ) -> MutPtr<PERF_COUNTERSET_INSTANCE> {
        todo!("PerfCreateInstance")
    }
    #[doc = "PerfDecrementULongCounterValue from ADVAPI32"]
    fn PerfDecrementULongCounterValue(
        &self,
        provider: super::super::Foundation::HANDLE,
        instance: MutPtr<PERF_COUNTERSET_INSTANCE>,
        counter_id: u32,
        value: u32,
    ) -> u32 {
        todo!("PerfDecrementULongCounterValue")
    }
    #[doc = "PerfDecrementULongLongCounterValue from ADVAPI32"]
    fn PerfDecrementULongLongCounterValue(
        &self,
        provider: super::super::Foundation::HANDLE,
        instance: MutPtr<PERF_COUNTERSET_INSTANCE>,
        counter_id: u32,
        value: u64,
    ) -> u32 {
        todo!("PerfDecrementULongLongCounterValue")
    }
    #[doc = "PerfDeleteCounters from ADVAPI32"]
    fn PerfDeleteCounters(
        &self,
        h_query: PerfQueryHandle,
        p_counters: ConstPtr<PERF_COUNTER_IDENTIFIER>,
        cb_counters: u32,
    ) -> u32 {
        todo!("PerfDeleteCounters")
    }
    #[doc = "PerfDeleteInstance from ADVAPI32"]
    fn PerfDeleteInstance(
        &self,
        provider: PerfProviderHandle,
        instance_block: ConstPtr<PERF_COUNTERSET_INSTANCE>,
    ) -> u32 {
        todo!("PerfDeleteInstance")
    }
    #[doc = "PerfEnumerateCounterSet from ADVAPI32"]
    fn PerfEnumerateCounterSet(
        &self,
        sz_machine: PCWSTR,
        p_counter_set_ids: MutPtr<crate::core::GUID>,
        c_counter_set_ids: u32,
        pc_counter_set_ids_actual: MutPtr<u32>,
    ) -> u32 {
        todo!("PerfEnumerateCounterSet")
    }
    #[doc = "PerfEnumerateCounterSetInstances from ADVAPI32"]
    fn PerfEnumerateCounterSetInstances(
        &self,
        sz_machine: PCWSTR,
        p_counter_set_id: ConstPtr<crate::core::GUID>,
        p_instances: MutPtr<PERF_INSTANCE_HEADER>,
        cb_instances: u32,
        pcb_instances_actual: MutPtr<u32>,
    ) -> u32 {
        todo!("PerfEnumerateCounterSetInstances")
    }
    #[doc = "PerfIncrementULongCounterValue from ADVAPI32"]
    fn PerfIncrementULongCounterValue(
        &self,
        provider: super::super::Foundation::HANDLE,
        instance: MutPtr<PERF_COUNTERSET_INSTANCE>,
        counter_id: u32,
        value: u32,
    ) -> u32 {
        todo!("PerfIncrementULongCounterValue")
    }
    #[doc = "PerfIncrementULongLongCounterValue from ADVAPI32"]
    fn PerfIncrementULongLongCounterValue(
        &self,
        provider: super::super::Foundation::HANDLE,
        instance: MutPtr<PERF_COUNTERSET_INSTANCE>,
        counter_id: u32,
        value: u64,
    ) -> u32 {
        todo!("PerfIncrementULongLongCounterValue")
    }
    #[doc = "PerfOpenQueryHandle from ADVAPI32"]
    fn PerfOpenQueryHandle(&self, sz_machine: PCWSTR, ph_query: MutPtr<PerfQueryHandle>) -> u32 {
        todo!("PerfOpenQueryHandle")
    }
    #[doc = "PerfQueryCounterData from ADVAPI32"]
    fn PerfQueryCounterData(
        &self,
        h_query: PerfQueryHandle,
        p_counter_block: MutPtr<PERF_DATA_HEADER>,
        cb_counter_block: u32,
        pcb_counter_block_actual: MutPtr<u32>,
    ) -> u32 {
        todo!("PerfQueryCounterData")
    }
    #[doc = "PerfQueryCounterInfo from ADVAPI32"]
    fn PerfQueryCounterInfo(
        &self,
        h_query: PerfQueryHandle,
        p_counters: MutPtr<PERF_COUNTER_IDENTIFIER>,
        cb_counters: u32,
        pcb_counters_actual: MutPtr<u32>,
    ) -> u32 {
        todo!("PerfQueryCounterInfo")
    }
    #[doc = "PerfQueryCounterSetRegistrationInfo from ADVAPI32"]
    fn PerfQueryCounterSetRegistrationInfo(
        &self,
        sz_machine: PCWSTR,
        p_counter_set_id: ConstPtr<crate::core::GUID>,
        request_code: PerfRegInfoType,
        request_lang_id: u32,
        pb_reg_info: MutPtr<u8>,
        cb_reg_info: u32,
        pcb_reg_info_actual: MutPtr<u32>,
    ) -> u32 {
        todo!("PerfQueryCounterSetRegistrationInfo")
    }
    #[doc = "PerfQueryInstance from ADVAPI32"]
    fn PerfQueryInstance(
        &self,
        provider_handle: super::super::Foundation::HANDLE,
        counter_set_guid: ConstPtr<crate::core::GUID>,
        name: PCWSTR,
        id: u32,
    ) -> MutPtr<PERF_COUNTERSET_INSTANCE> {
        todo!("PerfQueryInstance")
    }
    #[doc = "PerfSetCounterRefValue from ADVAPI32"]
    fn PerfSetCounterRefValue(
        &self,
        provider: super::super::Foundation::HANDLE,
        instance: MutPtr<PERF_COUNTERSET_INSTANCE>,
        counter_id: u32,
        address: ConstPtr<::core::ffi::c_void>,
    ) -> u32 {
        todo!("PerfSetCounterRefValue")
    }
    #[doc = "PerfSetCounterSetInfo from ADVAPI32"]
    fn PerfSetCounterSetInfo(
        &self,
        provider_handle: super::super::Foundation::HANDLE,
        template: MutPtr<PERF_COUNTERSET_INFO>,
        template_size: u32,
    ) -> u32 {
        todo!("PerfSetCounterSetInfo")
    }
    #[doc = "PerfSetULongCounterValue from ADVAPI32"]
    fn PerfSetULongCounterValue(
        &self,
        provider: super::super::Foundation::HANDLE,
        instance: MutPtr<PERF_COUNTERSET_INSTANCE>,
        counter_id: u32,
        value: u32,
    ) -> u32 {
        todo!("PerfSetULongCounterValue")
    }
    #[doc = "PerfSetULongLongCounterValue from ADVAPI32"]
    fn PerfSetULongLongCounterValue(
        &self,
        provider: super::super::Foundation::HANDLE,
        instance: MutPtr<PERF_COUNTERSET_INSTANCE>,
        counter_id: u32,
        value: u64,
    ) -> u32 {
        todo!("PerfSetULongLongCounterValue")
    }
    #[doc = "PerfStartProvider from ADVAPI32"]
    fn PerfStartProvider(
        &self,
        provider_guid: ConstPtr<crate::core::GUID>,
        control_callback: PERFLIBREQUEST,
        ph_provider: MutPtr<PerfProviderHandle>,
    ) -> u32 {
        todo!("PerfStartProvider")
    }
    #[doc = "PerfStartProviderEx from ADVAPI32"]
    fn PerfStartProviderEx(
        &self,
        provider_guid: ConstPtr<crate::core::GUID>,
        provider_context: ConstPtr<PERF_PROVIDER_CONTEXT>,
        provider: MutPtr<PerfProviderHandle>,
    ) -> u32 {
        todo!("PerfStartProviderEx")
    }
    #[doc = "PerfStopProvider from ADVAPI32"]
    fn PerfStopProvider(&self, provider_handle: PerfProviderHandle) -> u32 {
        todo!("PerfStopProvider")
    }
    #[doc = "QueryPerformanceCounter from KERNEL32"]
    fn QueryPerformanceCounter(
        &self,
        lp_performance_count: MutPtr<i64>,
    ) -> super::super::Foundation::BOOL {
        todo!("QueryPerformanceCounter")
    }
    #[doc = "QueryPerformanceFrequency from KERNEL32"]
    fn QueryPerformanceFrequency(
        &self,
        lp_frequency: MutPtr<i64>,
    ) -> super::super::Foundation::BOOL {
        todo!("QueryPerformanceFrequency")
    }
}
pub fn get_api(ctx: &crate::core::Win32Context) -> std::sync::Arc<dyn Api> {
    ctx.get::<dyn Api>()
}
