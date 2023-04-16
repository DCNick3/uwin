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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ADVF(pub i32);
pub const ADVF_NODATA: ADVF = ADVF(1i32);
pub const ADVF_PRIMEFIRST: ADVF = ADVF(2i32);
pub const ADVF_ONLYONCE: ADVF = ADVF(4i32);
pub const ADVF_DATAONSTOP: ADVF = ADVF(64i32);
pub const ADVFCACHE_NOHANDLER: ADVF = ADVF(8i32);
pub const ADVFCACHE_FORCEBUILTIN: ADVF = ADVF(16i32);
pub const ADVFCACHE_ONSAVE: ADVF = ADVF(32i32);
impl ::core::marker::Copy for ADVF {}
impl ::core::clone::Clone for ADVF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADVF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADVF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADVF").field(&self.0).finish()
    }
}
impl FromIntoMemory for ADVF {
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
pub const APPIDREGFLAGS_AAA_NO_IMPLICIT_ACTIVATE_AS_IU: u32 = 2048u32;
pub const APPIDREGFLAGS_ACTIVATE_IUSERVER_INDESKTOP: u32 = 1u32;
pub const APPIDREGFLAGS_ISSUE_ACTIVATION_RPC_AT_IDENTIFY: u32 = 4u32;
pub const APPIDREGFLAGS_IUSERVER_ACTIVATE_IN_CLIENT_SESSION_ONLY: u32 = 32u32;
pub const APPIDREGFLAGS_IUSERVER_SELF_SID_IN_LAUNCH_PERMISSION: u32 = 16u32;
pub const APPIDREGFLAGS_IUSERVER_UNMODIFIED_LOGON_TOKEN: u32 = 8u32;
pub const APPIDREGFLAGS_RESERVED1: u32 = 64u32;
pub const APPIDREGFLAGS_RESERVED2: u32 = 128u32;
pub const APPIDREGFLAGS_RESERVED3: u32 = 256u32;
pub const APPIDREGFLAGS_RESERVED4: u32 = 512u32;
pub const APPIDREGFLAGS_RESERVED5: u32 = 1024u32;
pub const APPIDREGFLAGS_RESERVED7: u32 = 4096u32;
pub const APPIDREGFLAGS_RESERVED8: u32 = 8192u32;
pub const APPIDREGFLAGS_RESERVED9: u32 = 16384u32;
pub const APPIDREGFLAGS_SECURE_SERVER_PROCESS_SD_AND_BIND: u32 = 2u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct APTTYPE(pub i32);
pub const APTTYPE_CURRENT: APTTYPE = APTTYPE(-1i32);
pub const APTTYPE_STA: APTTYPE = APTTYPE(0i32);
pub const APTTYPE_MTA: APTTYPE = APTTYPE(1i32);
pub const APTTYPE_NA: APTTYPE = APTTYPE(2i32);
pub const APTTYPE_MAINSTA: APTTYPE = APTTYPE(3i32);
impl ::core::marker::Copy for APTTYPE {}
impl ::core::clone::Clone for APTTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APTTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for APTTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APTTYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for APTTYPE {
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
pub struct APTTYPEQUALIFIER(pub i32);
pub const APTTYPEQUALIFIER_NONE: APTTYPEQUALIFIER = APTTYPEQUALIFIER(0i32);
pub const APTTYPEQUALIFIER_IMPLICIT_MTA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(1i32);
pub const APTTYPEQUALIFIER_NA_ON_MTA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(2i32);
pub const APTTYPEQUALIFIER_NA_ON_STA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(3i32);
pub const APTTYPEQUALIFIER_NA_ON_IMPLICIT_MTA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(4i32);
pub const APTTYPEQUALIFIER_NA_ON_MAINSTA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(5i32);
pub const APTTYPEQUALIFIER_APPLICATION_STA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(6i32);
pub const APTTYPEQUALIFIER_RESERVED_1: APTTYPEQUALIFIER = APTTYPEQUALIFIER(7i32);
impl ::core::marker::Copy for APTTYPEQUALIFIER {}
impl ::core::clone::Clone for APTTYPEQUALIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APTTYPEQUALIFIER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for APTTYPEQUALIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APTTYPEQUALIFIER").field(&self.0).finish()
    }
}
impl FromIntoMemory for APTTYPEQUALIFIER {
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
pub const ASYNC_MODE_COMPATIBILITY: i32 = 1i32;
pub const ASYNC_MODE_DEFAULT: i32 = 0i32;
pub struct AUTHENTICATEINFO {
    pub dwFlags: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for AUTHENTICATEINFO {}
impl ::core::clone::Clone for AUTHENTICATEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUTHENTICATEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTHENTICATEINFO")
            .field("dwFlags", &self.dwFlags)
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AUTHENTICATEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for AUTHENTICATEINFO {}
impl FromIntoMemory for AUTHENTICATEINFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwReserved = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            dwFlags: f_dwFlags,
            dwReserved: f_dwReserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwReserved, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ApplicationType(pub i32);
pub const ServerApplication: ApplicationType = ApplicationType(0i32);
pub const LibraryApplication: ApplicationType = ApplicationType(1i32);
impl ::core::marker::Copy for ApplicationType {}
impl ::core::clone::Clone for ApplicationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ApplicationType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ApplicationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationType").field(&self.0).finish()
    }
}
impl FromIntoMemory for ApplicationType {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct AsyncIAdviseSink(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait AsyncIAdviseSink_Trait: crate::core::IUnknown_Trait {
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Begin_OnDataChange(&self, p_formatetc: ConstPtr<FORMATETC>, p_stgmed: ConstPtr<STGMEDIUM>) {
        todo!("Begin_OnDataChange")
    }
    fn Finish_OnDataChange(&self) {
        todo!("Finish_OnDataChange")
    }
    fn Begin_OnViewChange(&self, dw_aspect: u32, lindex: i32) {
        todo!("Begin_OnViewChange")
    }
    fn Finish_OnViewChange(&self) {
        todo!("Finish_OnViewChange")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Begin_OnRename(&self, pmk: IMoniker) {
        todo!("Begin_OnRename")
    }
    fn Finish_OnRename(&self) {
        todo!("Finish_OnRename")
    }
    fn Begin_OnSave(&self) {
        todo!("Begin_OnSave")
    }
    fn Finish_OnSave(&self) {
        todo!("Finish_OnSave")
    }
    fn Begin_OnClose(&self) {
        todo!("Begin_OnClose")
    }
    fn Finish_OnClose(&self) {
        todo!("Finish_OnClose")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for AsyncIAdviseSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for AsyncIAdviseSink {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for AsyncIAdviseSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for AsyncIAdviseSink {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for AsyncIAdviseSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIAdviseSink").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for AsyncIAdviseSink {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for AsyncIAdviseSink {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000150_0000_0000_c000_000000000046);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct AsyncIAdviseSink2(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait AsyncIAdviseSink2_Trait: AsyncIAdviseSink_Trait {
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Begin_OnLinkSrcChange(&self, pmk: IMoniker) {
        todo!("Begin_OnLinkSrcChange")
    }
    fn Finish_OnLinkSrcChange(&self) {
        todo!("Finish_OnLinkSrcChange")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for AsyncIAdviseSink2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for AsyncIAdviseSink2 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for AsyncIAdviseSink2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for AsyncIAdviseSink2 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for AsyncIAdviseSink2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIAdviseSink2").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for AsyncIAdviseSink2 {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for AsyncIAdviseSink2 {
    type Super = AsyncIAdviseSink;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000151_0000_0000_c000_000000000046);
}
pub struct AsyncIMultiQI(pub crate::core::IUnknown);
pub trait AsyncIMultiQI_Trait: crate::core::IUnknown_Trait {
    fn Begin_QueryMultipleInterfaces(
        &self,
        c_mq_is: u32,
        p_mq_is: MutPtr<MULTI_QI>,
    ) -> crate::core::HRESULT {
        todo!("Begin_QueryMultipleInterfaces")
    }
    fn Finish_QueryMultipleInterfaces(&self, p_mq_is: MutPtr<MULTI_QI>) -> crate::core::HRESULT {
        todo!("Finish_QueryMultipleInterfaces")
    }
}
impl ::core::clone::Clone for AsyncIMultiQI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for AsyncIMultiQI {}
impl ::core::cmp::PartialEq for AsyncIMultiQI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIMultiQI {}
impl ::core::fmt::Debug for AsyncIMultiQI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIMultiQI").field(&self.0).finish()
    }
}
impl FromIntoMemory for AsyncIMultiQI {
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
impl crate::core::ComInterface for AsyncIMultiQI {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x000e0020_0000_0000_c000_000000000046);
}
pub struct AsyncIPipeByte(pub crate::core::IUnknown);
pub trait AsyncIPipeByte_Trait: crate::core::IUnknown_Trait {
    fn Begin_Pull(&self, c_request: u32) -> crate::core::HRESULT {
        todo!("Begin_Pull")
    }
    fn Finish_Pull(&self, buf: MutPtr<u8>, pc_returned: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("Finish_Pull")
    }
    fn Begin_Push(&self, buf: ConstPtr<u8>, c_sent: u32) -> crate::core::HRESULT {
        todo!("Begin_Push")
    }
    fn Finish_Push(&self) -> crate::core::HRESULT {
        todo!("Finish_Push")
    }
}
impl ::core::clone::Clone for AsyncIPipeByte {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for AsyncIPipeByte {}
impl ::core::cmp::PartialEq for AsyncIPipeByte {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIPipeByte {}
impl ::core::fmt::Debug for AsyncIPipeByte {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIPipeByte").field(&self.0).finish()
    }
}
impl FromIntoMemory for AsyncIPipeByte {
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
impl crate::core::ComInterface for AsyncIPipeByte {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xdb2f3acb_2f86_11d1_8e04_00c04fb9989a);
}
pub struct AsyncIPipeDouble(pub crate::core::IUnknown);
pub trait AsyncIPipeDouble_Trait: crate::core::IUnknown_Trait {
    fn Begin_Pull(&self, c_request: u32) -> crate::core::HRESULT {
        todo!("Begin_Pull")
    }
    fn Finish_Pull(&self, buf: MutPtr<f64>, pc_returned: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("Finish_Pull")
    }
    fn Begin_Push(&self, buf: ConstPtr<f64>, c_sent: u32) -> crate::core::HRESULT {
        todo!("Begin_Push")
    }
    fn Finish_Push(&self) -> crate::core::HRESULT {
        todo!("Finish_Push")
    }
}
impl ::core::clone::Clone for AsyncIPipeDouble {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for AsyncIPipeDouble {}
impl ::core::cmp::PartialEq for AsyncIPipeDouble {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIPipeDouble {}
impl ::core::fmt::Debug for AsyncIPipeDouble {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIPipeDouble").field(&self.0).finish()
    }
}
impl FromIntoMemory for AsyncIPipeDouble {
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
impl crate::core::ComInterface for AsyncIPipeDouble {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xdb2f3acf_2f86_11d1_8e04_00c04fb9989a);
}
pub struct AsyncIPipeLong(pub crate::core::IUnknown);
pub trait AsyncIPipeLong_Trait: crate::core::IUnknown_Trait {
    fn Begin_Pull(&self, c_request: u32) -> crate::core::HRESULT {
        todo!("Begin_Pull")
    }
    fn Finish_Pull(&self, buf: MutPtr<i32>, pc_returned: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("Finish_Pull")
    }
    fn Begin_Push(&self, buf: ConstPtr<i32>, c_sent: u32) -> crate::core::HRESULT {
        todo!("Begin_Push")
    }
    fn Finish_Push(&self) -> crate::core::HRESULT {
        todo!("Finish_Push")
    }
}
impl ::core::clone::Clone for AsyncIPipeLong {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for AsyncIPipeLong {}
impl ::core::cmp::PartialEq for AsyncIPipeLong {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIPipeLong {}
impl ::core::fmt::Debug for AsyncIPipeLong {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIPipeLong").field(&self.0).finish()
    }
}
impl FromIntoMemory for AsyncIPipeLong {
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
impl crate::core::ComInterface for AsyncIPipeLong {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xdb2f3acd_2f86_11d1_8e04_00c04fb9989a);
}
pub struct AsyncIUnknown(pub crate::core::IUnknown);
pub trait AsyncIUnknown_Trait: crate::core::IUnknown_Trait {
    fn Begin_QueryInterface(&self, riid: ConstPtr<crate::core::GUID>) -> crate::core::HRESULT {
        todo!("Begin_QueryInterface")
    }
    fn Finish_QueryInterface(
        &self,
        ppv_object: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("Finish_QueryInterface")
    }
    fn Begin_AddRef(&self) -> crate::core::HRESULT {
        todo!("Begin_AddRef")
    }
    fn Finish_AddRef(&self) -> u32 {
        todo!("Finish_AddRef")
    }
    fn Begin_Release(&self) -> crate::core::HRESULT {
        todo!("Begin_Release")
    }
    fn Finish_Release(&self) -> u32 {
        todo!("Finish_Release")
    }
}
impl ::core::clone::Clone for AsyncIUnknown {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for AsyncIUnknown {}
impl ::core::cmp::PartialEq for AsyncIUnknown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIUnknown {}
impl ::core::fmt::Debug for AsyncIUnknown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIUnknown").field(&self.0).finish()
    }
}
impl FromIntoMemory for AsyncIUnknown {
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
impl crate::core::ComInterface for AsyncIUnknown {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x000e0000_0000_0000_c000_000000000046);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct BINDINFO {
    pub cbSize: u32,
    pub szExtraInfo: PWSTR,
    pub stgmedData: STGMEDIUM,
    pub grfBindInfoF: u32,
    pub dwBindVerb: u32,
    pub szCustomVerb: PWSTR,
    pub cbstgmedData: u32,
    pub dwOptions: u32,
    pub dwOptionsFlags: u32,
    pub dwCodePage: u32,
    pub securityAttributes: super::super::Security::SECURITY_ATTRIBUTES,
    pub iid: crate::core::GUID,
    pub pUnk: crate::core::IUnknown,
    pub dwReserved: u32,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for BINDINFO {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for BINDINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for BINDINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BINDINFO")
            .field("cbSize", &self.cbSize)
            .field("szExtraInfo", &self.szExtraInfo)
            .field("stgmedData", &self.stgmedData)
            .field("grfBindInfoF", &self.grfBindInfoF)
            .field("dwBindVerb", &self.dwBindVerb)
            .field("szCustomVerb", &self.szCustomVerb)
            .field("cbstgmedData", &self.cbstgmedData)
            .field("dwOptions", &self.dwOptions)
            .field("dwOptionsFlags", &self.dwOptionsFlags)
            .field("dwCodePage", &self.dwCodePage)
            .field("securityAttributes", &self.securityAttributes)
            .field("iid", &self.iid)
            .field("pUnk", &self.pUnk)
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for BINDINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.szExtraInfo == other.szExtraInfo
            && self.stgmedData == other.stgmedData
            && self.grfBindInfoF == other.grfBindInfoF
            && self.dwBindVerb == other.dwBindVerb
            && self.szCustomVerb == other.szCustomVerb
            && self.cbstgmedData == other.cbstgmedData
            && self.dwOptions == other.dwOptions
            && self.dwOptionsFlags == other.dwOptionsFlags
            && self.dwCodePage == other.dwCodePage
            && self.securityAttributes == other.securityAttributes
            && self.iid == other.iid
            && self.pUnk == other.pUnk
            && self.dwReserved == other.dwReserved
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for BINDINFO {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for BINDINFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 84);
        let f_cbSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_szExtraInfo = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_stgmedData = <STGMEDIUM as FromIntoMemory>::from_bytes(&from[8..8 + 12]);
        let f_grfBindInfoF = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwBindVerb = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_szCustomVerb = <PWSTR as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_cbstgmedData = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_dwOptions = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_dwOptionsFlags = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_dwCodePage = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_securityAttributes =
            <super::super::Security::SECURITY_ATTRIBUTES as FromIntoMemory>::from_bytes(
                &from[48..48 + 12],
            );
        let f_iid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[60..60 + 16]);
        let f_pUnk = <crate::core::IUnknown as FromIntoMemory>::from_bytes(&from[76..76 + 4]);
        let f_dwReserved = <u32 as FromIntoMemory>::from_bytes(&from[80..80 + 4]);
        Self {
            cbSize: f_cbSize,
            szExtraInfo: f_szExtraInfo,
            stgmedData: f_stgmedData,
            grfBindInfoF: f_grfBindInfoF,
            dwBindVerb: f_dwBindVerb,
            szCustomVerb: f_szCustomVerb,
            cbstgmedData: f_cbstgmedData,
            dwOptions: f_dwOptions,
            dwOptionsFlags: f_dwOptionsFlags,
            dwCodePage: f_dwCodePage,
            securityAttributes: f_securityAttributes,
            iid: f_iid,
            pUnk: f_pUnk,
            dwReserved: f_dwReserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 84);
        FromIntoMemory::into_bytes(self.cbSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.szExtraInfo, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.stgmedData, &mut into[8..8 + 12]);
        FromIntoMemory::into_bytes(self.grfBindInfoF, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwBindVerb, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.szCustomVerb, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.cbstgmedData, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.dwOptions, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.dwOptionsFlags, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.dwCodePage, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.securityAttributes, &mut into[48..48 + 12]);
        FromIntoMemory::into_bytes(self.iid, &mut into[60..60 + 16]);
        FromIntoMemory::into_bytes(self.pUnk, &mut into[76..76 + 4]);
        FromIntoMemory::into_bytes(self.dwReserved, &mut into[80..80 + 4]);
    }
    fn size() -> usize {
        84
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BINDINFOF(pub i32);
pub const BINDINFOF_URLENCODESTGMEDDATA: BINDINFOF = BINDINFOF(1i32);
pub const BINDINFOF_URLENCODEDEXTRAINFO: BINDINFOF = BINDINFOF(2i32);
impl ::core::marker::Copy for BINDINFOF {}
impl ::core::clone::Clone for BINDINFOF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BINDINFOF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BINDINFOF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BINDINFOF").field(&self.0).finish()
    }
}
impl FromIntoMemory for BINDINFOF {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct BINDPTR {
    data: [u8; 4],
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::default::Default for BINDPTR {
    fn default() -> Self {
        Self { data: [0u8; 4] }
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for BINDPTR {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for BINDPTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for BINDPTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BINDPTR").field("data", &self.data).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for BINDPTR {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for BINDPTR {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for BINDPTR {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BIND_FLAGS(pub i32);
pub const BIND_MAYBOTHERUSER: BIND_FLAGS = BIND_FLAGS(1i32);
pub const BIND_JUSTTESTEXISTENCE: BIND_FLAGS = BIND_FLAGS(2i32);
impl ::core::marker::Copy for BIND_FLAGS {}
impl ::core::clone::Clone for BIND_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BIND_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BIND_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BIND_FLAGS").field(&self.0).finish()
    }
}
impl FromIntoMemory for BIND_FLAGS {
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
pub struct BIND_OPTS {
    pub cbStruct: u32,
    pub grfFlags: u32,
    pub grfMode: u32,
    pub dwTickCountDeadline: u32,
}
impl ::core::marker::Copy for BIND_OPTS {}
impl ::core::clone::Clone for BIND_OPTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BIND_OPTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BIND_OPTS")
            .field("cbStruct", &self.cbStruct)
            .field("grfFlags", &self.grfFlags)
            .field("grfMode", &self.grfMode)
            .field("dwTickCountDeadline", &self.dwTickCountDeadline)
            .finish()
    }
}
impl ::core::cmp::PartialEq for BIND_OPTS {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.grfFlags == other.grfFlags
            && self.grfMode == other.grfMode
            && self.dwTickCountDeadline == other.dwTickCountDeadline
    }
}
impl ::core::cmp::Eq for BIND_OPTS {}
impl FromIntoMemory for BIND_OPTS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_grfFlags = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_grfMode = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwTickCountDeadline = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            cbStruct: f_cbStruct,
            grfFlags: f_grfFlags,
            grfMode: f_grfMode,
            dwTickCountDeadline: f_dwTickCountDeadline,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.grfFlags, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.grfMode, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwTickCountDeadline, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct BIND_OPTS2 {
    pub __AnonymousBase_objidl_L9017_C36: BIND_OPTS,
    pub dwTrackFlags: u32,
    pub dwClassContext: u32,
    pub locale: u32,
    pub pServerInfo: MutPtr<COSERVERINFO>,
}
impl ::core::marker::Copy for BIND_OPTS2 {}
impl ::core::clone::Clone for BIND_OPTS2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BIND_OPTS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BIND_OPTS2")
            .field(
                "__AnonymousBase_objidl_L9017_C36",
                &self.__AnonymousBase_objidl_L9017_C36,
            )
            .field("dwTrackFlags", &self.dwTrackFlags)
            .field("dwClassContext", &self.dwClassContext)
            .field("locale", &self.locale)
            .field("pServerInfo", &self.pServerInfo)
            .finish()
    }
}
impl ::core::cmp::PartialEq for BIND_OPTS2 {
    fn eq(&self, other: &Self) -> bool {
        self.__AnonymousBase_objidl_L9017_C36 == other.__AnonymousBase_objidl_L9017_C36
            && self.dwTrackFlags == other.dwTrackFlags
            && self.dwClassContext == other.dwClassContext
            && self.locale == other.locale
            && self.pServerInfo == other.pServerInfo
    }
}
impl ::core::cmp::Eq for BIND_OPTS2 {}
impl FromIntoMemory for BIND_OPTS2 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f___AnonymousBase_objidl_L9017_C36 =
            <BIND_OPTS as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_dwTrackFlags = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwClassContext = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_locale = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_pServerInfo = <MutPtr<COSERVERINFO> as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            __AnonymousBase_objidl_L9017_C36: f___AnonymousBase_objidl_L9017_C36,
            dwTrackFlags: f_dwTrackFlags,
            dwClassContext: f_dwClassContext,
            locale: f_locale,
            pServerInfo: f_pServerInfo,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.__AnonymousBase_objidl_L9017_C36, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.dwTrackFlags, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwClassContext, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.locale, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.pServerInfo, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
pub struct BIND_OPTS3 {
    pub __AnonymousBase_objidl_L9041_C36: BIND_OPTS2,
    pub hwnd: super::super::Foundation::HWND,
}
impl ::core::marker::Copy for BIND_OPTS3 {}
impl ::core::clone::Clone for BIND_OPTS3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BIND_OPTS3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BIND_OPTS3")
            .field(
                "__AnonymousBase_objidl_L9041_C36",
                &self.__AnonymousBase_objidl_L9041_C36,
            )
            .field("hwnd", &self.hwnd)
            .finish()
    }
}
impl ::core::cmp::PartialEq for BIND_OPTS3 {
    fn eq(&self, other: &Self) -> bool {
        self.__AnonymousBase_objidl_L9041_C36 == other.__AnonymousBase_objidl_L9041_C36
            && self.hwnd == other.hwnd
    }
}
impl ::core::cmp::Eq for BIND_OPTS3 {}
impl FromIntoMemory for BIND_OPTS3 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 36);
        let f___AnonymousBase_objidl_L9041_C36 =
            <BIND_OPTS2 as FromIntoMemory>::from_bytes(&from[0..0 + 32]);
        let f_hwnd =
            <super::super::Foundation::HWND as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        Self {
            __AnonymousBase_objidl_L9041_C36: f___AnonymousBase_objidl_L9041_C36,
            hwnd: f_hwnd,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 36);
        FromIntoMemory::into_bytes(self.__AnonymousBase_objidl_L9041_C36, &mut into[0..0 + 32]);
        FromIntoMemory::into_bytes(self.hwnd, &mut into[32..32 + 4]);
    }
    fn size() -> usize {
        36
    }
}
pub struct BLOB {
    pub cbSize: u32,
    pub pBlobData: MutPtr<u8>,
}
impl ::core::marker::Copy for BLOB {}
impl ::core::clone::Clone for BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BLOB")
            .field("cbSize", &self.cbSize)
            .field("pBlobData", &self.pBlobData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pBlobData == other.pBlobData
    }
}
impl ::core::cmp::Eq for BLOB {}
impl FromIntoMemory for BLOB {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_cbSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_pBlobData = <MutPtr<u8> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            cbSize: f_cbSize,
            pBlobData: f_pBlobData,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.cbSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.pBlobData, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct BYTE_BLOB {
    pub clSize: u32,
    pub abData: [u8; 1],
}
impl ::core::marker::Copy for BYTE_BLOB {}
impl ::core::clone::Clone for BYTE_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BYTE_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BYTE_BLOB")
            .field("clSize", &self.clSize)
            .field("abData", &self.abData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for BYTE_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.clSize == other.clSize && self.abData == other.abData
    }
}
impl ::core::cmp::Eq for BYTE_BLOB {}
impl FromIntoMemory for BYTE_BLOB {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_clSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_abData = <[u8; 1] as FromIntoMemory>::from_bytes(&from[4..4 + 1]);
        Self {
            clSize: f_clSize,
            abData: f_abData,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.clSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.abData, &mut into[4..4 + 1]);
    }
    fn size() -> usize {
        8
    }
}
pub struct BYTE_SIZEDARR {
    pub clSize: u32,
    pub pData: MutPtr<u8>,
}
impl ::core::marker::Copy for BYTE_SIZEDARR {}
impl ::core::clone::Clone for BYTE_SIZEDARR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BYTE_SIZEDARR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BYTE_SIZEDARR")
            .field("clSize", &self.clSize)
            .field("pData", &self.pData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for BYTE_SIZEDARR {
    fn eq(&self, other: &Self) -> bool {
        self.clSize == other.clSize && self.pData == other.pData
    }
}
impl ::core::cmp::Eq for BYTE_SIZEDARR {}
impl FromIntoMemory for BYTE_SIZEDARR {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_clSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_pData = <MutPtr<u8> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            clSize: f_clSize,
            pData: f_pData,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.clSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.pData, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CALLCONV(pub i32);
pub const CC_FASTCALL: CALLCONV = CALLCONV(0i32);
pub const CC_CDECL: CALLCONV = CALLCONV(1i32);
pub const CC_MSCPASCAL: CALLCONV = CALLCONV(2i32);
pub const CC_PASCAL: CALLCONV = CALLCONV(2i32);
pub const CC_MACPASCAL: CALLCONV = CALLCONV(3i32);
pub const CC_STDCALL: CALLCONV = CALLCONV(4i32);
pub const CC_FPFASTCALL: CALLCONV = CALLCONV(5i32);
pub const CC_SYSCALL: CALLCONV = CALLCONV(6i32);
pub const CC_MPWCDECL: CALLCONV = CALLCONV(7i32);
pub const CC_MPWPASCAL: CALLCONV = CALLCONV(8i32);
pub const CC_MAX: CALLCONV = CALLCONV(9i32);
impl ::core::marker::Copy for CALLCONV {}
impl ::core::clone::Clone for CALLCONV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CALLCONV {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CALLCONV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CALLCONV").field(&self.0).finish()
    }
}
impl FromIntoMemory for CALLCONV {
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
pub struct CALLTYPE(pub i32);
pub const CALLTYPE_TOPLEVEL: CALLTYPE = CALLTYPE(1i32);
pub const CALLTYPE_NESTED: CALLTYPE = CALLTYPE(2i32);
pub const CALLTYPE_ASYNC: CALLTYPE = CALLTYPE(3i32);
pub const CALLTYPE_TOPLEVEL_CALLPENDING: CALLTYPE = CALLTYPE(4i32);
pub const CALLTYPE_ASYNC_CALLPENDING: CALLTYPE = CALLTYPE(5i32);
impl ::core::marker::Copy for CALLTYPE {}
impl ::core::clone::Clone for CALLTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CALLTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CALLTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CALLTYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for CALLTYPE {
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
pub struct CATEGORYINFO {
    pub catid: crate::core::GUID,
    pub lcid: u32,
    pub szDescription: [u16; 128],
}
impl ::core::marker::Copy for CATEGORYINFO {}
impl ::core::clone::Clone for CATEGORYINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CATEGORYINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CATEGORYINFO")
            .field("catid", &self.catid)
            .field("lcid", &self.lcid)
            .field("szDescription", &self.szDescription)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CATEGORYINFO {
    fn eq(&self, other: &Self) -> bool {
        self.catid == other.catid
            && self.lcid == other.lcid
            && self.szDescription == other.szDescription
    }
}
impl ::core::cmp::Eq for CATEGORYINFO {}
impl FromIntoMemory for CATEGORYINFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 148);
        let f_catid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_lcid = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_szDescription = <[u16; 128] as FromIntoMemory>::from_bytes(&from[20..20 + 128]);
        Self {
            catid: f_catid,
            lcid: f_lcid,
            szDescription: f_szDescription,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 148);
        FromIntoMemory::into_bytes(self.catid, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.lcid, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.szDescription, &mut into[20..20 + 128]);
    }
    fn size() -> usize {
        148
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CLSCTX(pub u32);
pub const CLSCTX_INPROC_SERVER: CLSCTX = CLSCTX(1u32);
pub const CLSCTX_INPROC_HANDLER: CLSCTX = CLSCTX(2u32);
pub const CLSCTX_LOCAL_SERVER: CLSCTX = CLSCTX(4u32);
pub const CLSCTX_INPROC_SERVER16: CLSCTX = CLSCTX(8u32);
pub const CLSCTX_REMOTE_SERVER: CLSCTX = CLSCTX(16u32);
pub const CLSCTX_INPROC_HANDLER16: CLSCTX = CLSCTX(32u32);
pub const CLSCTX_RESERVED1: CLSCTX = CLSCTX(64u32);
pub const CLSCTX_RESERVED2: CLSCTX = CLSCTX(128u32);
pub const CLSCTX_RESERVED3: CLSCTX = CLSCTX(256u32);
pub const CLSCTX_RESERVED4: CLSCTX = CLSCTX(512u32);
pub const CLSCTX_NO_CODE_DOWNLOAD: CLSCTX = CLSCTX(1024u32);
pub const CLSCTX_RESERVED5: CLSCTX = CLSCTX(2048u32);
pub const CLSCTX_NO_CUSTOM_MARSHAL: CLSCTX = CLSCTX(4096u32);
pub const CLSCTX_ENABLE_CODE_DOWNLOAD: CLSCTX = CLSCTX(8192u32);
pub const CLSCTX_NO_FAILURE_LOG: CLSCTX = CLSCTX(16384u32);
pub const CLSCTX_DISABLE_AAA: CLSCTX = CLSCTX(32768u32);
pub const CLSCTX_ENABLE_AAA: CLSCTX = CLSCTX(65536u32);
pub const CLSCTX_FROM_DEFAULT_CONTEXT: CLSCTX = CLSCTX(131072u32);
pub const CLSCTX_ACTIVATE_X86_SERVER: CLSCTX = CLSCTX(262144u32);
pub const CLSCTX_ACTIVATE_32_BIT_SERVER: CLSCTX = CLSCTX(262144u32);
pub const CLSCTX_ACTIVATE_64_BIT_SERVER: CLSCTX = CLSCTX(524288u32);
pub const CLSCTX_ENABLE_CLOAKING: CLSCTX = CLSCTX(1048576u32);
pub const CLSCTX_APPCONTAINER: CLSCTX = CLSCTX(4194304u32);
pub const CLSCTX_ACTIVATE_AAA_AS_IU: CLSCTX = CLSCTX(8388608u32);
pub const CLSCTX_RESERVED6: CLSCTX = CLSCTX(16777216u32);
pub const CLSCTX_ACTIVATE_ARM32_SERVER: CLSCTX = CLSCTX(33554432u32);
pub const CLSCTX_PS_DLL: CLSCTX = CLSCTX(2147483648u32);
pub const CLSCTX_ALL: CLSCTX = CLSCTX(23u32);
pub const CLSCTX_SERVER: CLSCTX = CLSCTX(21u32);
impl ::core::marker::Copy for CLSCTX {}
impl ::core::clone::Clone for CLSCTX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLSCTX {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLSCTX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLSCTX").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CLSCTX {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CLSCTX {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CLSCTX {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CLSCTX {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CLSCTX {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for CLSCTX {
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
pub struct COAUTHIDENTITY {
    pub User: MutPtr<u16>,
    pub UserLength: u32,
    pub Domain: MutPtr<u16>,
    pub DomainLength: u32,
    pub Password: MutPtr<u16>,
    pub PasswordLength: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for COAUTHIDENTITY {}
impl ::core::clone::Clone for COAUTHIDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COAUTHIDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COAUTHIDENTITY")
            .field("User", &self.User)
            .field("UserLength", &self.UserLength)
            .field("Domain", &self.Domain)
            .field("DomainLength", &self.DomainLength)
            .field("Password", &self.Password)
            .field("PasswordLength", &self.PasswordLength)
            .field("Flags", &self.Flags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for COAUTHIDENTITY {
    fn eq(&self, other: &Self) -> bool {
        self.User == other.User
            && self.UserLength == other.UserLength
            && self.Domain == other.Domain
            && self.DomainLength == other.DomainLength
            && self.Password == other.Password
            && self.PasswordLength == other.PasswordLength
            && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for COAUTHIDENTITY {}
impl FromIntoMemory for COAUTHIDENTITY {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_User = <MutPtr<u16> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_UserLength = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Domain = <MutPtr<u16> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_DomainLength = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_Password = <MutPtr<u16> as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_PasswordLength = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_Flags = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        Self {
            User: f_User,
            UserLength: f_UserLength,
            Domain: f_Domain,
            DomainLength: f_DomainLength,
            Password: f_Password,
            PasswordLength: f_PasswordLength,
            Flags: f_Flags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.User, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.UserLength, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Domain, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.DomainLength, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.Password, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.PasswordLength, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[24..24 + 4]);
    }
    fn size() -> usize {
        28
    }
}
pub struct COAUTHINFO {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pwszServerPrincName: PWSTR,
    pub dwAuthnLevel: u32,
    pub dwImpersonationLevel: u32,
    pub pAuthIdentityData: MutPtr<COAUTHIDENTITY>,
    pub dwCapabilities: u32,
}
impl ::core::marker::Copy for COAUTHINFO {}
impl ::core::clone::Clone for COAUTHINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COAUTHINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COAUTHINFO")
            .field("dwAuthnSvc", &self.dwAuthnSvc)
            .field("dwAuthzSvc", &self.dwAuthzSvc)
            .field("pwszServerPrincName", &self.pwszServerPrincName)
            .field("dwAuthnLevel", &self.dwAuthnLevel)
            .field("dwImpersonationLevel", &self.dwImpersonationLevel)
            .field("pAuthIdentityData", &self.pAuthIdentityData)
            .field("dwCapabilities", &self.dwCapabilities)
            .finish()
    }
}
impl ::core::cmp::PartialEq for COAUTHINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwAuthnSvc == other.dwAuthnSvc
            && self.dwAuthzSvc == other.dwAuthzSvc
            && self.pwszServerPrincName == other.pwszServerPrincName
            && self.dwAuthnLevel == other.dwAuthnLevel
            && self.dwImpersonationLevel == other.dwImpersonationLevel
            && self.pAuthIdentityData == other.pAuthIdentityData
            && self.dwCapabilities == other.dwCapabilities
    }
}
impl ::core::cmp::Eq for COAUTHINFO {}
impl FromIntoMemory for COAUTHINFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_dwAuthnSvc = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwAuthzSvc = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_pwszServerPrincName = <PWSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwAuthnLevel = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwImpersonationLevel = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_pAuthIdentityData =
            <MutPtr<COAUTHIDENTITY> as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwCapabilities = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        Self {
            dwAuthnSvc: f_dwAuthnSvc,
            dwAuthzSvc: f_dwAuthzSvc,
            pwszServerPrincName: f_pwszServerPrincName,
            dwAuthnLevel: f_dwAuthnLevel,
            dwImpersonationLevel: f_dwImpersonationLevel,
            pAuthIdentityData: f_pAuthIdentityData,
            dwCapabilities: f_dwCapabilities,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.dwAuthnSvc, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwAuthzSvc, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.pwszServerPrincName, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwAuthnLevel, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwImpersonationLevel, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.pAuthIdentityData, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwCapabilities, &mut into[24..24 + 4]);
    }
    fn size() -> usize {
        28
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COINIT(pub u32);
pub const COINIT_APARTMENTTHREADED: COINIT = COINIT(2u32);
pub const COINIT_MULTITHREADED: COINIT = COINIT(0u32);
pub const COINIT_DISABLE_OLE1DDE: COINIT = COINIT(4u32);
pub const COINIT_SPEED_OVER_MEMORY: COINIT = COINIT(8u32);
impl ::core::marker::Copy for COINIT {}
impl ::core::clone::Clone for COINIT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COINIT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COINIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COINIT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for COINIT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for COINIT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for COINIT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for COINIT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for COINIT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for COINIT {
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
pub struct COINITBASE(pub i32);
pub const COINITBASE_MULTITHREADED: COINITBASE = COINITBASE(0i32);
impl ::core::marker::Copy for COINITBASE {}
impl ::core::clone::Clone for COINITBASE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COINITBASE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COINITBASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COINITBASE").field(&self.0).finish()
    }
}
impl FromIntoMemory for COINITBASE {
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
pub struct COMSD(pub i32);
pub const SD_LAUNCHPERMISSIONS: COMSD = COMSD(0i32);
pub const SD_ACCESSPERMISSIONS: COMSD = COMSD(1i32);
pub const SD_LAUNCHRESTRICTIONS: COMSD = COMSD(2i32);
pub const SD_ACCESSRESTRICTIONS: COMSD = COMSD(3i32);
impl ::core::marker::Copy for COMSD {}
impl ::core::clone::Clone for COMSD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMSD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COMSD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMSD").field(&self.0).finish()
    }
}
impl FromIntoMemory for COMSD {
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
pub const COM_RIGHTS_ACTIVATE_LOCAL: u32 = 8u32;
pub const COM_RIGHTS_ACTIVATE_REMOTE: u32 = 16u32;
pub const COM_RIGHTS_EXECUTE: u32 = 1u32;
pub const COM_RIGHTS_EXECUTE_LOCAL: u32 = 2u32;
pub const COM_RIGHTS_EXECUTE_REMOTE: u32 = 4u32;
pub const COM_RIGHTS_RESERVED1: u32 = 32u32;
pub const COM_RIGHTS_RESERVED2: u32 = 64u32;
pub struct CONNECTDATA {
    pub pUnk: crate::core::IUnknown,
    pub dwCookie: u32,
}
impl ::core::marker::Copy for CONNECTDATA {}
impl ::core::clone::Clone for CONNECTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONNECTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONNECTDATA")
            .field("pUnk", &self.pUnk)
            .field("dwCookie", &self.dwCookie)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CONNECTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.pUnk == other.pUnk && self.dwCookie == other.dwCookie
    }
}
impl ::core::cmp::Eq for CONNECTDATA {}
impl FromIntoMemory for CONNECTDATA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_pUnk = <crate::core::IUnknown as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwCookie = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            pUnk: f_pUnk,
            dwCookie: f_dwCookie,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.pUnk, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwCookie, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct COSERVERINFO {
    pub dwReserved1: u32,
    pub pwszName: PWSTR,
    pub pAuthInfo: MutPtr<COAUTHINFO>,
    pub dwReserved2: u32,
}
impl ::core::marker::Copy for COSERVERINFO {}
impl ::core::clone::Clone for COSERVERINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COSERVERINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COSERVERINFO")
            .field("dwReserved1", &self.dwReserved1)
            .field("pwszName", &self.pwszName)
            .field("pAuthInfo", &self.pAuthInfo)
            .field("dwReserved2", &self.dwReserved2)
            .finish()
    }
}
impl ::core::cmp::PartialEq for COSERVERINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwReserved1 == other.dwReserved1
            && self.pwszName == other.pwszName
            && self.pAuthInfo == other.pAuthInfo
            && self.dwReserved2 == other.dwReserved2
    }
}
impl ::core::cmp::Eq for COSERVERINFO {}
impl FromIntoMemory for COSERVERINFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dwReserved1 = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_pwszName = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_pAuthInfo = <MutPtr<COAUTHINFO> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwReserved2 = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            dwReserved1: f_dwReserved1,
            pwszName: f_pwszName,
            pAuthInfo: f_pAuthInfo,
            dwReserved2: f_dwReserved2,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dwReserved1, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.pwszName, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.pAuthInfo, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwReserved2, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COWAIT_FLAGS(pub i32);
pub const COWAIT_DEFAULT: COWAIT_FLAGS = COWAIT_FLAGS(0i32);
pub const COWAIT_WAITALL: COWAIT_FLAGS = COWAIT_FLAGS(1i32);
pub const COWAIT_ALERTABLE: COWAIT_FLAGS = COWAIT_FLAGS(2i32);
pub const COWAIT_INPUTAVAILABLE: COWAIT_FLAGS = COWAIT_FLAGS(4i32);
pub const COWAIT_DISPATCH_CALLS: COWAIT_FLAGS = COWAIT_FLAGS(8i32);
pub const COWAIT_DISPATCH_WINDOW_MESSAGES: COWAIT_FLAGS = COWAIT_FLAGS(16i32);
impl ::core::marker::Copy for COWAIT_FLAGS {}
impl ::core::clone::Clone for COWAIT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COWAIT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COWAIT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COWAIT_FLAGS").field(&self.0).finish()
    }
}
impl FromIntoMemory for COWAIT_FLAGS {
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
pub struct CO_DEVICE_CATALOG_COOKIE(pub PtrDiffRepr);
impl CO_DEVICE_CATALOG_COOKIE {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CO_DEVICE_CATALOG_COOKIE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for CO_DEVICE_CATALOG_COOKIE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for CO_DEVICE_CATALOG_COOKIE {}
impl ::core::hash::Hash for CO_DEVICE_CATALOG_COOKIE {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for CO_DEVICE_CATALOG_COOKIE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CO_DEVICE_CATALOG_COOKIE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for CO_DEVICE_CATALOG_COOKIE {
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
pub struct CO_MARSHALING_CONTEXT_ATTRIBUTES(pub i32);
pub const CO_MARSHALING_SOURCE_IS_APP_CONTAINER: CO_MARSHALING_CONTEXT_ATTRIBUTES =
    CO_MARSHALING_CONTEXT_ATTRIBUTES(0i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_1: CO_MARSHALING_CONTEXT_ATTRIBUTES =
    CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483648i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_2: CO_MARSHALING_CONTEXT_ATTRIBUTES =
    CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483647i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_3: CO_MARSHALING_CONTEXT_ATTRIBUTES =
    CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483646i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_4: CO_MARSHALING_CONTEXT_ATTRIBUTES =
    CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483645i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_5: CO_MARSHALING_CONTEXT_ATTRIBUTES =
    CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483644i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_6: CO_MARSHALING_CONTEXT_ATTRIBUTES =
    CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483643i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_7: CO_MARSHALING_CONTEXT_ATTRIBUTES =
    CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483642i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_8: CO_MARSHALING_CONTEXT_ATTRIBUTES =
    CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483641i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_9: CO_MARSHALING_CONTEXT_ATTRIBUTES =
    CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483640i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_10: CO_MARSHALING_CONTEXT_ATTRIBUTES =
    CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483639i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_11: CO_MARSHALING_CONTEXT_ATTRIBUTES =
    CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483638i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_12: CO_MARSHALING_CONTEXT_ATTRIBUTES =
    CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483637i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_13: CO_MARSHALING_CONTEXT_ATTRIBUTES =
    CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483636i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_14: CO_MARSHALING_CONTEXT_ATTRIBUTES =
    CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483635i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_15: CO_MARSHALING_CONTEXT_ATTRIBUTES =
    CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483634i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_16: CO_MARSHALING_CONTEXT_ATTRIBUTES =
    CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483633i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_17: CO_MARSHALING_CONTEXT_ATTRIBUTES =
    CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483632i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_18: CO_MARSHALING_CONTEXT_ATTRIBUTES =
    CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483631i32);
impl ::core::marker::Copy for CO_MARSHALING_CONTEXT_ATTRIBUTES {}
impl ::core::clone::Clone for CO_MARSHALING_CONTEXT_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CO_MARSHALING_CONTEXT_ATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CO_MARSHALING_CONTEXT_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CO_MARSHALING_CONTEXT_ATTRIBUTES")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for CO_MARSHALING_CONTEXT_ATTRIBUTES {
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
pub struct CO_MTA_USAGE_COOKIE(pub PtrDiffRepr);
impl CO_MTA_USAGE_COOKIE {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CO_MTA_USAGE_COOKIE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for CO_MTA_USAGE_COOKIE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for CO_MTA_USAGE_COOKIE {}
impl ::core::hash::Hash for CO_MTA_USAGE_COOKIE {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for CO_MTA_USAGE_COOKIE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CO_MTA_USAGE_COOKIE").field(&self.0).finish()
    }
}
impl FromIntoMemory for CO_MTA_USAGE_COOKIE {
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
pub struct CSPLATFORM {
    pub dwPlatformId: u32,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
    pub dwProcessorArch: u32,
}
impl ::core::marker::Copy for CSPLATFORM {}
impl ::core::clone::Clone for CSPLATFORM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CSPLATFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSPLATFORM")
            .field("dwPlatformId", &self.dwPlatformId)
            .field("dwVersionHi", &self.dwVersionHi)
            .field("dwVersionLo", &self.dwVersionLo)
            .field("dwProcessorArch", &self.dwProcessorArch)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CSPLATFORM {
    fn eq(&self, other: &Self) -> bool {
        self.dwPlatformId == other.dwPlatformId
            && self.dwVersionHi == other.dwVersionHi
            && self.dwVersionLo == other.dwVersionLo
            && self.dwProcessorArch == other.dwProcessorArch
    }
}
impl ::core::cmp::Eq for CSPLATFORM {}
impl FromIntoMemory for CSPLATFORM {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dwPlatformId = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwVersionHi = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwVersionLo = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwProcessorArch = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            dwPlatformId: f_dwPlatformId,
            dwVersionHi: f_dwVersionHi,
            dwVersionLo: f_dwVersionLo,
            dwProcessorArch: f_dwProcessorArch,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dwPlatformId, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwVersionHi, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwVersionLo, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwProcessorArch, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct CUSTDATA {
    pub cCustData: u32,
    pub prgCustData: MutPtr<CUSTDATAITEM>,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for CUSTDATA {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for CUSTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for CUSTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CUSTDATA")
            .field("cCustData", &self.cCustData)
            .field("prgCustData", &self.prgCustData)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for CUSTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.cCustData == other.cCustData && self.prgCustData == other.prgCustData
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for CUSTDATA {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for CUSTDATA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_cCustData = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_prgCustData = <MutPtr<CUSTDATAITEM> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            cCustData: f_cCustData,
            prgCustData: f_prgCustData,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.cCustData, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.prgCustData, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct CUSTDATAITEM {
    pub guid: crate::core::GUID,
    pub varValue: VARIANT,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for CUSTDATAITEM {
    fn clone(&self) -> Self {
        Self {
            guid: self.guid,
            varValue: self.varValue.clone(),
        }
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for CUSTDATAITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CUSTDATAITEM")
            .field("guid", &self.guid)
            .field("varValue", &self.varValue)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for CUSTDATAITEM {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid && self.varValue == other.varValue
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for CUSTDATAITEM {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for CUSTDATAITEM {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_guid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_varValue = <VARIANT as FromIntoMemory>::from_bytes(&from[16..16 + 16]);
        Self {
            guid: f_guid,
            varValue: f_varValue,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.guid, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.varValue, &mut into[16..16 + 16]);
    }
    fn size() -> usize {
        32
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CWMO_FLAGS(pub i32);
pub const CWMO_DEFAULT: CWMO_FLAGS = CWMO_FLAGS(0i32);
pub const CWMO_DISPATCH_CALLS: CWMO_FLAGS = CWMO_FLAGS(1i32);
pub const CWMO_DISPATCH_WINDOW_MESSAGES: CWMO_FLAGS = CWMO_FLAGS(2i32);
impl ::core::marker::Copy for CWMO_FLAGS {}
impl ::core::clone::Clone for CWMO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CWMO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CWMO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CWMO_FLAGS").field(&self.0).finish()
    }
}
impl FromIntoMemory for CWMO_FLAGS {
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
pub const CWMO_MAX_HANDLES: u32 = 56u32;
pub struct CY {
    data: [u8; 8],
}
impl ::core::default::Default for CY {
    fn default() -> Self {
        Self { data: [0u8; 8] }
    }
}
impl ::core::marker::Copy for CY {}
impl ::core::clone::Clone for CY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CY").field("data", &self.data).finish()
    }
}
impl ::core::cmp::PartialEq for CY {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for CY {}
impl FromIntoMemory for CY {
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
pub struct CY_0 {
    pub Lo: u32,
    pub Hi: i32,
}
impl ::core::marker::Copy for CY_0 {}
impl ::core::clone::Clone for CY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CY_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CY_0")
            .field("Lo", &self.Lo)
            .field("Hi", &self.Hi)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CY_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Lo == other.Lo && self.Hi == other.Hi
    }
}
impl ::core::cmp::Eq for CY_0 {}
impl FromIntoMemory for CY_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Lo = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Hi = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self { Lo: f_Lo, Hi: f_Hi }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Lo, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Hi, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct ComCallData {
    pub dwDispid: u32,
    pub dwReserved: u32,
    pub pUserDefined: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for ComCallData {}
impl ::core::clone::Clone for ComCallData {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ComCallData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ComCallData")
            .field("dwDispid", &self.dwDispid)
            .field("dwReserved", &self.dwReserved)
            .field("pUserDefined", &self.pUserDefined)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ComCallData {
    fn eq(&self, other: &Self) -> bool {
        self.dwDispid == other.dwDispid
            && self.dwReserved == other.dwReserved
            && self.pUserDefined == other.pUserDefined
    }
}
impl ::core::cmp::Eq for ComCallData {}
impl FromIntoMemory for ComCallData {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_dwDispid = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwReserved = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_pUserDefined =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            dwDispid: f_dwDispid,
            dwReserved: f_dwReserved,
            pUserDefined: f_pUserDefined,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.dwDispid, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwReserved, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.pUserDefined, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DATADIR(pub i32);
pub const DATADIR_GET: DATADIR = DATADIR(1i32);
pub const DATADIR_SET: DATADIR = DATADIR(2i32);
impl ::core::marker::Copy for DATADIR {}
impl ::core::clone::Clone for DATADIR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DATADIR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DATADIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DATADIR").field(&self.0).finish()
    }
}
impl FromIntoMemory for DATADIR {
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
pub const DCOMSCM_ACTIVATION_DISALLOW_UNSECURE_CALL: u32 = 2u32;
pub const DCOMSCM_ACTIVATION_USE_ALL_AUTHNSERVICES: u32 = 1u32;
pub const DCOMSCM_PING_DISALLOW_UNSECURE_CALL: u32 = 32u32;
pub const DCOMSCM_PING_USE_MID_AUTHNSERVICE: u32 = 16u32;
pub const DCOMSCM_RESOLVE_DISALLOW_UNSECURE_CALL: u32 = 8u32;
pub const DCOMSCM_RESOLVE_USE_ALL_AUTHNSERVICES: u32 = 4u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DCOM_CALL_STATE(pub i32);
pub const DCOM_NONE: DCOM_CALL_STATE = DCOM_CALL_STATE(0i32);
pub const DCOM_CALL_COMPLETE: DCOM_CALL_STATE = DCOM_CALL_STATE(1i32);
pub const DCOM_CALL_CANCELED: DCOM_CALL_STATE = DCOM_CALL_STATE(2i32);
impl ::core::marker::Copy for DCOM_CALL_STATE {}
impl ::core::clone::Clone for DCOM_CALL_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCOM_CALL_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DCOM_CALL_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCOM_CALL_STATE").field(&self.0).finish()
    }
}
impl FromIntoMemory for DCOM_CALL_STATE {
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
pub struct DESCKIND(pub i32);
pub const DESCKIND_NONE: DESCKIND = DESCKIND(0i32);
pub const DESCKIND_FUNCDESC: DESCKIND = DESCKIND(1i32);
pub const DESCKIND_VARDESC: DESCKIND = DESCKIND(2i32);
pub const DESCKIND_TYPECOMP: DESCKIND = DESCKIND(3i32);
pub const DESCKIND_IMPLICITAPPOBJ: DESCKIND = DESCKIND(4i32);
pub const DESCKIND_MAX: DESCKIND = DESCKIND(5i32);
impl ::core::marker::Copy for DESCKIND {}
impl ::core::clone::Clone for DESCKIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DESCKIND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DESCKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DESCKIND").field(&self.0).finish()
    }
}
impl FromIntoMemory for DESCKIND {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct DISPPARAMS {
    pub rgvarg: MutPtr<VARIANT>,
    pub rgdispidNamedArgs: MutPtr<i32>,
    pub cArgs: u32,
    pub cNamedArgs: u32,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for DISPPARAMS {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for DISPPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for DISPPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISPPARAMS")
            .field("rgvarg", &self.rgvarg)
            .field("rgdispidNamedArgs", &self.rgdispidNamedArgs)
            .field("cArgs", &self.cArgs)
            .field("cNamedArgs", &self.cNamedArgs)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for DISPPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.rgvarg == other.rgvarg
            && self.rgdispidNamedArgs == other.rgdispidNamedArgs
            && self.cArgs == other.cArgs
            && self.cNamedArgs == other.cNamedArgs
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for DISPPARAMS {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for DISPPARAMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_rgvarg = <MutPtr<VARIANT> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_rgdispidNamedArgs = <MutPtr<i32> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_cArgs = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_cNamedArgs = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            rgvarg: f_rgvarg,
            rgdispidNamedArgs: f_rgdispidNamedArgs,
            cArgs: f_cArgs,
            cNamedArgs: f_cNamedArgs,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.rgvarg, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.rgdispidNamedArgs, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.cArgs, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.cNamedArgs, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub const DMUS_ERRBASE: u32 = 4096u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DVASPECT(pub i32);
pub const DVASPECT_CONTENT: DVASPECT = DVASPECT(1i32);
pub const DVASPECT_THUMBNAIL: DVASPECT = DVASPECT(2i32);
pub const DVASPECT_ICON: DVASPECT = DVASPECT(4i32);
pub const DVASPECT_DOCPRINT: DVASPECT = DVASPECT(8i32);
impl ::core::marker::Copy for DVASPECT {}
impl ::core::clone::Clone for DVASPECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DVASPECT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DVASPECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DVASPECT").field(&self.0).finish()
    }
}
impl FromIntoMemory for DVASPECT {
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
pub struct DVTARGETDEVICE {
    pub tdSize: u32,
    pub tdDriverNameOffset: u16,
    pub tdDeviceNameOffset: u16,
    pub tdPortNameOffset: u16,
    pub tdExtDevmodeOffset: u16,
    pub tdData: [u8; 1],
}
impl ::core::marker::Copy for DVTARGETDEVICE {}
impl ::core::clone::Clone for DVTARGETDEVICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DVTARGETDEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DVTARGETDEVICE")
            .field("tdSize", &self.tdSize)
            .field("tdDriverNameOffset", &self.tdDriverNameOffset)
            .field("tdDeviceNameOffset", &self.tdDeviceNameOffset)
            .field("tdPortNameOffset", &self.tdPortNameOffset)
            .field("tdExtDevmodeOffset", &self.tdExtDevmodeOffset)
            .field("tdData", &self.tdData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DVTARGETDEVICE {
    fn eq(&self, other: &Self) -> bool {
        self.tdSize == other.tdSize
            && self.tdDriverNameOffset == other.tdDriverNameOffset
            && self.tdDeviceNameOffset == other.tdDeviceNameOffset
            && self.tdPortNameOffset == other.tdPortNameOffset
            && self.tdExtDevmodeOffset == other.tdExtDevmodeOffset
            && self.tdData == other.tdData
    }
}
impl ::core::cmp::Eq for DVTARGETDEVICE {}
impl FromIntoMemory for DVTARGETDEVICE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_tdSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_tdDriverNameOffset = <u16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_tdDeviceNameOffset = <u16 as FromIntoMemory>::from_bytes(&from[6..6 + 2]);
        let f_tdPortNameOffset = <u16 as FromIntoMemory>::from_bytes(&from[8..8 + 2]);
        let f_tdExtDevmodeOffset = <u16 as FromIntoMemory>::from_bytes(&from[10..10 + 2]);
        let f_tdData = <[u8; 1] as FromIntoMemory>::from_bytes(&from[12..12 + 1]);
        Self {
            tdSize: f_tdSize,
            tdDriverNameOffset: f_tdDriverNameOffset,
            tdDeviceNameOffset: f_tdDeviceNameOffset,
            tdPortNameOffset: f_tdPortNameOffset,
            tdExtDevmodeOffset: f_tdExtDevmodeOffset,
            tdData: f_tdData,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.tdSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.tdDriverNameOffset, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.tdDeviceNameOffset, &mut into[6..6 + 2]);
        FromIntoMemory::into_bytes(self.tdPortNameOffset, &mut into[8..8 + 2]);
        FromIntoMemory::into_bytes(self.tdExtDevmodeOffset, &mut into[10..10 + 2]);
        FromIntoMemory::into_bytes(self.tdData, &mut into[12..12 + 1]);
    }
    fn size() -> usize {
        16
    }
}
pub struct DWORD_BLOB {
    pub clSize: u32,
    pub alData: [u32; 1],
}
impl ::core::marker::Copy for DWORD_BLOB {}
impl ::core::clone::Clone for DWORD_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWORD_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWORD_BLOB")
            .field("clSize", &self.clSize)
            .field("alData", &self.alData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DWORD_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.clSize == other.clSize && self.alData == other.alData
    }
}
impl ::core::cmp::Eq for DWORD_BLOB {}
impl FromIntoMemory for DWORD_BLOB {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_clSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_alData = <[u32; 1] as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            clSize: f_clSize,
            alData: f_alData,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.clSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.alData, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ELEMDESC {
    pub tdesc: TYPEDESC,
    pub Anonymous: ELEMDESC_0,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ELEMDESC {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ELEMDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ELEMDESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ELEMDESC")
            .field("tdesc", &self.tdesc)
            .field("Anonymous", &self.Anonymous)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ELEMDESC {
    fn eq(&self, other: &Self) -> bool {
        self.tdesc == other.tdesc && self.Anonymous == other.Anonymous
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ELEMDESC {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ELEMDESC {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_tdesc = <TYPEDESC as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_Anonymous = <ELEMDESC_0 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        Self {
            tdesc: f_tdesc,
            Anonymous: f_Anonymous,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.tdesc, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[8..8 + 8]);
    }
    fn size() -> usize {
        16
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ELEMDESC_0 {
    data: [u8; 8],
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::default::Default for ELEMDESC_0 {
    fn default() -> Self {
        Self { data: [0u8; 8] }
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ELEMDESC_0 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ELEMDESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ELEMDESC_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ELEMDESC_0")
            .field("data", &self.data)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ELEMDESC_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ELEMDESC_0 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ELEMDESC_0 {
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
pub struct EOLE_AUTHENTICATION_CAPABILITIES(pub i32);
pub const EOAC_NONE: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(0i32);
pub const EOAC_MUTUAL_AUTH: EOLE_AUTHENTICATION_CAPABILITIES =
    EOLE_AUTHENTICATION_CAPABILITIES(1i32);
pub const EOAC_STATIC_CLOAKING: EOLE_AUTHENTICATION_CAPABILITIES =
    EOLE_AUTHENTICATION_CAPABILITIES(32i32);
pub const EOAC_DYNAMIC_CLOAKING: EOLE_AUTHENTICATION_CAPABILITIES =
    EOLE_AUTHENTICATION_CAPABILITIES(64i32);
pub const EOAC_ANY_AUTHORITY: EOLE_AUTHENTICATION_CAPABILITIES =
    EOLE_AUTHENTICATION_CAPABILITIES(128i32);
pub const EOAC_MAKE_FULLSIC: EOLE_AUTHENTICATION_CAPABILITIES =
    EOLE_AUTHENTICATION_CAPABILITIES(256i32);
pub const EOAC_DEFAULT: EOLE_AUTHENTICATION_CAPABILITIES =
    EOLE_AUTHENTICATION_CAPABILITIES(2048i32);
pub const EOAC_SECURE_REFS: EOLE_AUTHENTICATION_CAPABILITIES =
    EOLE_AUTHENTICATION_CAPABILITIES(2i32);
pub const EOAC_ACCESS_CONTROL: EOLE_AUTHENTICATION_CAPABILITIES =
    EOLE_AUTHENTICATION_CAPABILITIES(4i32);
pub const EOAC_APPID: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(8i32);
pub const EOAC_DYNAMIC: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(16i32);
pub const EOAC_REQUIRE_FULLSIC: EOLE_AUTHENTICATION_CAPABILITIES =
    EOLE_AUTHENTICATION_CAPABILITIES(512i32);
pub const EOAC_AUTO_IMPERSONATE: EOLE_AUTHENTICATION_CAPABILITIES =
    EOLE_AUTHENTICATION_CAPABILITIES(1024i32);
pub const EOAC_DISABLE_AAA: EOLE_AUTHENTICATION_CAPABILITIES =
    EOLE_AUTHENTICATION_CAPABILITIES(4096i32);
pub const EOAC_NO_CUSTOM_MARSHAL: EOLE_AUTHENTICATION_CAPABILITIES =
    EOLE_AUTHENTICATION_CAPABILITIES(8192i32);
pub const EOAC_RESERVED1: EOLE_AUTHENTICATION_CAPABILITIES =
    EOLE_AUTHENTICATION_CAPABILITIES(16384i32);
impl ::core::marker::Copy for EOLE_AUTHENTICATION_CAPABILITIES {}
impl ::core::clone::Clone for EOLE_AUTHENTICATION_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EOLE_AUTHENTICATION_CAPABILITIES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EOLE_AUTHENTICATION_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EOLE_AUTHENTICATION_CAPABILITIES")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for EOLE_AUTHENTICATION_CAPABILITIES {
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
pub struct EXCEPINFO {
    pub wCode: u16,
    pub wReserved: u16,
    pub bstrSource: super::super::Foundation::BSTR,
    pub bstrDescription: super::super::Foundation::BSTR,
    pub bstrHelpFile: super::super::Foundation::BSTR,
    pub dwHelpContext: u32,
    pub pvReserved: MutPtr<::core::ffi::c_void>,
    pub pfnDeferredFillIn: LPEXCEPFINO_DEFERRED_FILLIN,
    pub scode: i32,
}
impl ::core::clone::Clone for EXCEPINFO {
    fn clone(&self) -> Self {
        Self {
            wCode: self.wCode,
            wReserved: self.wReserved,
            bstrSource: self.bstrSource.clone(),
            bstrDescription: self.bstrDescription.clone(),
            bstrHelpFile: self.bstrHelpFile.clone(),
            dwHelpContext: self.dwHelpContext,
            pvReserved: self.pvReserved,
            pfnDeferredFillIn: self.pfnDeferredFillIn,
            scode: self.scode,
        }
    }
}
impl ::core::fmt::Debug for EXCEPINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXCEPINFO")
            .field("wCode", &self.wCode)
            .field("wReserved", &self.wReserved)
            .field("bstrSource", &self.bstrSource)
            .field("bstrDescription", &self.bstrDescription)
            .field("bstrHelpFile", &self.bstrHelpFile)
            .field("dwHelpContext", &self.dwHelpContext)
            .field("pvReserved", &self.pvReserved)
            .field("pfnDeferredFillIn", &self.pfnDeferredFillIn)
            .field("scode", &self.scode)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EXCEPINFO {
    fn eq(&self, other: &Self) -> bool {
        self.wCode == other.wCode
            && self.wReserved == other.wReserved
            && self.bstrSource == other.bstrSource
            && self.bstrDescription == other.bstrDescription
            && self.bstrHelpFile == other.bstrHelpFile
            && self.dwHelpContext == other.dwHelpContext
            && self.pvReserved == other.pvReserved
            && self.pfnDeferredFillIn == other.pfnDeferredFillIn
            && self.scode == other.scode
    }
}
impl ::core::cmp::Eq for EXCEPINFO {}
impl FromIntoMemory for EXCEPINFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_wCode = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wReserved = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_bstrSource =
            <super::super::Foundation::BSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_bstrDescription =
            <super::super::Foundation::BSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_bstrHelpFile =
            <super::super::Foundation::BSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwHelpContext = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_pvReserved =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_pfnDeferredFillIn =
            <LPEXCEPFINO_DEFERRED_FILLIN as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_scode = <i32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            wCode: f_wCode,
            wReserved: f_wReserved,
            bstrSource: f_bstrSource,
            bstrDescription: f_bstrDescription,
            bstrHelpFile: f_bstrHelpFile,
            dwHelpContext: f_dwHelpContext,
            pvReserved: f_pvReserved,
            pfnDeferredFillIn: f_pfnDeferredFillIn,
            scode: f_scode,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.wCode, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wReserved, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.bstrSource, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.bstrDescription, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.bstrHelpFile, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwHelpContext, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.pvReserved, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.pfnDeferredFillIn, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.scode, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EXTCONN(pub i32);
pub const EXTCONN_STRONG: EXTCONN = EXTCONN(1i32);
pub const EXTCONN_WEAK: EXTCONN = EXTCONN(2i32);
pub const EXTCONN_CALLABLE: EXTCONN = EXTCONN(4i32);
impl ::core::marker::Copy for EXTCONN {}
impl ::core::clone::Clone for EXTCONN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EXTCONN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EXTCONN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EXTCONN").field(&self.0).finish()
    }
}
impl FromIntoMemory for EXTCONN {
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
pub struct FLAGGED_BYTE_BLOB {
    pub fFlags: u32,
    pub clSize: u32,
    pub abData: [u8; 1],
}
impl ::core::marker::Copy for FLAGGED_BYTE_BLOB {}
impl ::core::clone::Clone for FLAGGED_BYTE_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FLAGGED_BYTE_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLAGGED_BYTE_BLOB")
            .field("fFlags", &self.fFlags)
            .field("clSize", &self.clSize)
            .field("abData", &self.abData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FLAGGED_BYTE_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.fFlags == other.fFlags && self.clSize == other.clSize && self.abData == other.abData
    }
}
impl ::core::cmp::Eq for FLAGGED_BYTE_BLOB {}
impl FromIntoMemory for FLAGGED_BYTE_BLOB {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_fFlags = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_clSize = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_abData = <[u8; 1] as FromIntoMemory>::from_bytes(&from[8..8 + 1]);
        Self {
            fFlags: f_fFlags,
            clSize: f_clSize,
            abData: f_abData,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.fFlags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.clSize, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.abData, &mut into[8..8 + 1]);
    }
    fn size() -> usize {
        12
    }
}
pub struct FLAGGED_WORD_BLOB {
    pub fFlags: u32,
    pub clSize: u32,
    pub asData: [u16; 1],
}
impl ::core::marker::Copy for FLAGGED_WORD_BLOB {}
impl ::core::clone::Clone for FLAGGED_WORD_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FLAGGED_WORD_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLAGGED_WORD_BLOB")
            .field("fFlags", &self.fFlags)
            .field("clSize", &self.clSize)
            .field("asData", &self.asData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FLAGGED_WORD_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.fFlags == other.fFlags && self.clSize == other.clSize && self.asData == other.asData
    }
}
impl ::core::cmp::Eq for FLAGGED_WORD_BLOB {}
impl FromIntoMemory for FLAGGED_WORD_BLOB {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_fFlags = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_clSize = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_asData = <[u16; 1] as FromIntoMemory>::from_bytes(&from[8..8 + 2]);
        Self {
            fFlags: f_fFlags,
            clSize: f_clSize,
            asData: f_asData,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.fFlags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.clSize, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.asData, &mut into[8..8 + 2]);
    }
    fn size() -> usize {
        12
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct FLAG_STGMEDIUM {
    pub ContextFlags: i32,
    pub fPassOwnership: i32,
    pub Stgmed: STGMEDIUM,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for FLAG_STGMEDIUM {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for FLAG_STGMEDIUM {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for FLAG_STGMEDIUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLAG_STGMEDIUM")
            .field("ContextFlags", &self.ContextFlags)
            .field("fPassOwnership", &self.fPassOwnership)
            .field("Stgmed", &self.Stgmed)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for FLAG_STGMEDIUM {
    fn eq(&self, other: &Self) -> bool {
        self.ContextFlags == other.ContextFlags
            && self.fPassOwnership == other.fPassOwnership
            && self.Stgmed == other.Stgmed
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for FLAG_STGMEDIUM {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for FLAG_STGMEDIUM {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_ContextFlags = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_fPassOwnership = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Stgmed = <STGMEDIUM as FromIntoMemory>::from_bytes(&from[8..8 + 12]);
        Self {
            ContextFlags: f_ContextFlags,
            fPassOwnership: f_fPassOwnership,
            Stgmed: f_Stgmed,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.ContextFlags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.fPassOwnership, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Stgmed, &mut into[8..8 + 12]);
    }
    fn size() -> usize {
        20
    }
}
pub struct FORMATETC {
    pub cfFormat: u16,
    pub ptd: MutPtr<DVTARGETDEVICE>,
    pub dwAspect: u32,
    pub lindex: i32,
    pub tymed: u32,
}
impl ::core::marker::Copy for FORMATETC {}
impl ::core::clone::Clone for FORMATETC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FORMATETC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FORMATETC")
            .field("cfFormat", &self.cfFormat)
            .field("ptd", &self.ptd)
            .field("dwAspect", &self.dwAspect)
            .field("lindex", &self.lindex)
            .field("tymed", &self.tymed)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FORMATETC {
    fn eq(&self, other: &Self) -> bool {
        self.cfFormat == other.cfFormat
            && self.ptd == other.ptd
            && self.dwAspect == other.dwAspect
            && self.lindex == other.lindex
            && self.tymed == other.tymed
    }
}
impl ::core::cmp::Eq for FORMATETC {}
impl FromIntoMemory for FORMATETC {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_cfFormat = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_ptd = <MutPtr<DVTARGETDEVICE> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwAspect = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lindex = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_tymed = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            cfFormat: f_cfFormat,
            ptd: f_ptd,
            dwAspect: f_dwAspect,
            lindex: f_lindex,
            tymed: f_tymed,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.cfFormat, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.ptd, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwAspect, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lindex, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.tymed, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct FUNCDESC {
    pub memid: i32,
    pub lprgscode: MutPtr<i32>,
    pub lprgelemdescParam: MutPtr<ELEMDESC>,
    pub funckind: FUNCKIND,
    pub invkind: INVOKEKIND,
    pub callconv: CALLCONV,
    pub cParams: i16,
    pub cParamsOpt: i16,
    pub oVft: i16,
    pub cScodes: i16,
    pub elemdescFunc: ELEMDESC,
    pub wFuncFlags: u16,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for FUNCDESC {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for FUNCDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for FUNCDESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FUNCDESC")
            .field("memid", &self.memid)
            .field("lprgscode", &self.lprgscode)
            .field("lprgelemdescParam", &self.lprgelemdescParam)
            .field("funckind", &self.funckind)
            .field("invkind", &self.invkind)
            .field("callconv", &self.callconv)
            .field("cParams", &self.cParams)
            .field("cParamsOpt", &self.cParamsOpt)
            .field("oVft", &self.oVft)
            .field("cScodes", &self.cScodes)
            .field("elemdescFunc", &self.elemdescFunc)
            .field("wFuncFlags", &self.wFuncFlags)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for FUNCDESC {
    fn eq(&self, other: &Self) -> bool {
        self.memid == other.memid
            && self.lprgscode == other.lprgscode
            && self.lprgelemdescParam == other.lprgelemdescParam
            && self.funckind == other.funckind
            && self.invkind == other.invkind
            && self.callconv == other.callconv
            && self.cParams == other.cParams
            && self.cParamsOpt == other.cParamsOpt
            && self.oVft == other.oVft
            && self.cScodes == other.cScodes
            && self.elemdescFunc == other.elemdescFunc
            && self.wFuncFlags == other.wFuncFlags
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for FUNCDESC {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for FUNCDESC {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 52);
        let f_memid = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lprgscode = <MutPtr<i32> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lprgelemdescParam = <MutPtr<ELEMDESC> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_funckind = <FUNCKIND as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_invkind = <INVOKEKIND as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_callconv = <CALLCONV as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_cParams = <i16 as FromIntoMemory>::from_bytes(&from[24..24 + 2]);
        let f_cParamsOpt = <i16 as FromIntoMemory>::from_bytes(&from[26..26 + 2]);
        let f_oVft = <i16 as FromIntoMemory>::from_bytes(&from[28..28 + 2]);
        let f_cScodes = <i16 as FromIntoMemory>::from_bytes(&from[30..30 + 2]);
        let f_elemdescFunc = <ELEMDESC as FromIntoMemory>::from_bytes(&from[32..32 + 16]);
        let f_wFuncFlags = <u16 as FromIntoMemory>::from_bytes(&from[48..48 + 2]);
        Self {
            memid: f_memid,
            lprgscode: f_lprgscode,
            lprgelemdescParam: f_lprgelemdescParam,
            funckind: f_funckind,
            invkind: f_invkind,
            callconv: f_callconv,
            cParams: f_cParams,
            cParamsOpt: f_cParamsOpt,
            oVft: f_oVft,
            cScodes: f_cScodes,
            elemdescFunc: f_elemdescFunc,
            wFuncFlags: f_wFuncFlags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 52);
        FromIntoMemory::into_bytes(self.memid, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lprgscode, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lprgelemdescParam, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.funckind, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.invkind, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.callconv, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.cParams, &mut into[24..24 + 2]);
        FromIntoMemory::into_bytes(self.cParamsOpt, &mut into[26..26 + 2]);
        FromIntoMemory::into_bytes(self.oVft, &mut into[28..28 + 2]);
        FromIntoMemory::into_bytes(self.cScodes, &mut into[30..30 + 2]);
        FromIntoMemory::into_bytes(self.elemdescFunc, &mut into[32..32 + 16]);
        FromIntoMemory::into_bytes(self.wFuncFlags, &mut into[48..48 + 2]);
    }
    fn size() -> usize {
        52
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FUNCKIND(pub i32);
pub const FUNC_VIRTUAL: FUNCKIND = FUNCKIND(0i32);
pub const FUNC_PUREVIRTUAL: FUNCKIND = FUNCKIND(1i32);
pub const FUNC_NONVIRTUAL: FUNCKIND = FUNCKIND(2i32);
pub const FUNC_STATIC: FUNCKIND = FUNCKIND(3i32);
pub const FUNC_DISPATCH: FUNCKIND = FUNCKIND(4i32);
impl ::core::marker::Copy for FUNCKIND {}
impl ::core::clone::Clone for FUNCKIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FUNCKIND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FUNCKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FUNCKIND").field(&self.0).finish()
    }
}
impl FromIntoMemory for FUNCKIND {
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
pub struct GDI_OBJECT {
    pub ObjectType: u32,
    pub u: GDI_OBJECT_0,
}
impl ::core::marker::Copy for GDI_OBJECT {}
impl ::core::clone::Clone for GDI_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GDI_OBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GDI_OBJECT")
            .field("ObjectType", &self.ObjectType)
            .field("u", &self.u)
            .finish()
    }
}
impl ::core::cmp::PartialEq for GDI_OBJECT {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectType == other.ObjectType && self.u == other.u
    }
}
impl ::core::cmp::Eq for GDI_OBJECT {}
impl FromIntoMemory for GDI_OBJECT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_ObjectType = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_u = <GDI_OBJECT_0 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            ObjectType: f_ObjectType,
            u: f_u,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.ObjectType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.u, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct GDI_OBJECT_0 {
    data: [u8; 4],
}
impl ::core::default::Default for GDI_OBJECT_0 {
    fn default() -> Self {
        Self { data: [0u8; 4] }
    }
}
impl ::core::marker::Copy for GDI_OBJECT_0 {}
impl ::core::clone::Clone for GDI_OBJECT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GDI_OBJECT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GDI_OBJECT_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for GDI_OBJECT_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for GDI_OBJECT_0 {}
impl FromIntoMemory for GDI_OBJECT_0 {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GLOBALOPT_EH_VALUES(pub i32);
pub const COMGLB_EXCEPTION_HANDLE: GLOBALOPT_EH_VALUES = GLOBALOPT_EH_VALUES(0i32);
pub const COMGLB_EXCEPTION_DONOT_HANDLE_FATAL: GLOBALOPT_EH_VALUES = GLOBALOPT_EH_VALUES(1i32);
pub const COMGLB_EXCEPTION_DONOT_HANDLE: GLOBALOPT_EH_VALUES = GLOBALOPT_EH_VALUES(1i32);
pub const COMGLB_EXCEPTION_DONOT_HANDLE_ANY: GLOBALOPT_EH_VALUES = GLOBALOPT_EH_VALUES(2i32);
impl ::core::marker::Copy for GLOBALOPT_EH_VALUES {}
impl ::core::clone::Clone for GLOBALOPT_EH_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GLOBALOPT_EH_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GLOBALOPT_EH_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLOBALOPT_EH_VALUES").field(&self.0).finish()
    }
}
impl FromIntoMemory for GLOBALOPT_EH_VALUES {
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
pub struct GLOBALOPT_PROPERTIES(pub i32);
pub const COMGLB_EXCEPTION_HANDLING: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(1i32);
pub const COMGLB_APPID: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(2i32);
pub const COMGLB_RPC_THREADPOOL_SETTING: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(3i32);
pub const COMGLB_RO_SETTINGS: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(4i32);
pub const COMGLB_UNMARSHALING_POLICY: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(5i32);
pub const COMGLB_PROPERTIES_RESERVED1: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(6i32);
pub const COMGLB_PROPERTIES_RESERVED2: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(7i32);
pub const COMGLB_PROPERTIES_RESERVED3: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(8i32);
impl ::core::marker::Copy for GLOBALOPT_PROPERTIES {}
impl ::core::clone::Clone for GLOBALOPT_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GLOBALOPT_PROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GLOBALOPT_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLOBALOPT_PROPERTIES")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for GLOBALOPT_PROPERTIES {
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
pub struct GLOBALOPT_RO_FLAGS(pub i32);
pub const COMGLB_STA_MODALLOOP_REMOVE_TOUCH_MESSAGES: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(1i32);
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_REMOVE_INPUT_MESSAGES: GLOBALOPT_RO_FLAGS =
    GLOBALOPT_RO_FLAGS(2i32);
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_DONOT_REMOVE_INPUT_MESSAGES: GLOBALOPT_RO_FLAGS =
    GLOBALOPT_RO_FLAGS(4i32);
pub const COMGLB_FAST_RUNDOWN: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(8i32);
pub const COMGLB_RESERVED1: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(16i32);
pub const COMGLB_RESERVED2: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(32i32);
pub const COMGLB_RESERVED3: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(64i32);
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_REORDER_POINTER_MESSAGES: GLOBALOPT_RO_FLAGS =
    GLOBALOPT_RO_FLAGS(128i32);
pub const COMGLB_RESERVED4: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(256i32);
pub const COMGLB_RESERVED5: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(512i32);
pub const COMGLB_RESERVED6: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(1024i32);
impl ::core::marker::Copy for GLOBALOPT_RO_FLAGS {}
impl ::core::clone::Clone for GLOBALOPT_RO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GLOBALOPT_RO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GLOBALOPT_RO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLOBALOPT_RO_FLAGS").field(&self.0).finish()
    }
}
impl FromIntoMemory for GLOBALOPT_RO_FLAGS {
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
pub struct GLOBALOPT_RPCTP_VALUES(pub i32);
pub const COMGLB_RPC_THREADPOOL_SETTING_DEFAULT_POOL: GLOBALOPT_RPCTP_VALUES =
    GLOBALOPT_RPCTP_VALUES(0i32);
pub const COMGLB_RPC_THREADPOOL_SETTING_PRIVATE_POOL: GLOBALOPT_RPCTP_VALUES =
    GLOBALOPT_RPCTP_VALUES(1i32);
impl ::core::marker::Copy for GLOBALOPT_RPCTP_VALUES {}
impl ::core::clone::Clone for GLOBALOPT_RPCTP_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GLOBALOPT_RPCTP_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GLOBALOPT_RPCTP_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLOBALOPT_RPCTP_VALUES")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for GLOBALOPT_RPCTP_VALUES {
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
pub struct GLOBALOPT_UNMARSHALING_POLICY_VALUES(pub i32);
pub const COMGLB_UNMARSHALING_POLICY_NORMAL: GLOBALOPT_UNMARSHALING_POLICY_VALUES =
    GLOBALOPT_UNMARSHALING_POLICY_VALUES(0i32);
pub const COMGLB_UNMARSHALING_POLICY_STRONG: GLOBALOPT_UNMARSHALING_POLICY_VALUES =
    GLOBALOPT_UNMARSHALING_POLICY_VALUES(1i32);
pub const COMGLB_UNMARSHALING_POLICY_HYBRID: GLOBALOPT_UNMARSHALING_POLICY_VALUES =
    GLOBALOPT_UNMARSHALING_POLICY_VALUES(2i32);
impl ::core::marker::Copy for GLOBALOPT_UNMARSHALING_POLICY_VALUES {}
impl ::core::clone::Clone for GLOBALOPT_UNMARSHALING_POLICY_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GLOBALOPT_UNMARSHALING_POLICY_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GLOBALOPT_UNMARSHALING_POLICY_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLOBALOPT_UNMARSHALING_POLICY_VALUES")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for GLOBALOPT_UNMARSHALING_POLICY_VALUES {
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
pub struct HYPER_SIZEDARR {
    pub clSize: u32,
    pub pData: MutPtr<i64>,
}
impl ::core::marker::Copy for HYPER_SIZEDARR {}
impl ::core::clone::Clone for HYPER_SIZEDARR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HYPER_SIZEDARR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HYPER_SIZEDARR")
            .field("clSize", &self.clSize)
            .field("pData", &self.pData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for HYPER_SIZEDARR {
    fn eq(&self, other: &Self) -> bool {
        self.clSize == other.clSize && self.pData == other.pData
    }
}
impl ::core::cmp::Eq for HYPER_SIZEDARR {}
impl FromIntoMemory for HYPER_SIZEDARR {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_clSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_pData = <MutPtr<i64> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            clSize: f_clSize,
            pData: f_pData,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.clSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.pData, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct IActivationFilter(pub crate::core::IUnknown);
pub trait IActivationFilter_Trait: crate::core::IUnknown_Trait {
    fn HandleActivation(
        &self,
        dw_activation_type: u32,
        rclsid: ConstPtr<crate::core::GUID>,
        p_replacement_cls_id: MutPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("HandleActivation")
    }
}
impl ::core::clone::Clone for IActivationFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IActivationFilter {}
impl ::core::cmp::PartialEq for IActivationFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActivationFilter {}
impl ::core::fmt::Debug for IActivationFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActivationFilter").field(&self.0).finish()
    }
}
impl FromIntoMemory for IActivationFilter {
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
impl crate::core::ComInterface for IActivationFilter {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000017_0000_0000_c000_000000000046);
}
pub struct IAddrExclusionControl(pub crate::core::IUnknown);
pub trait IAddrExclusionControl_Trait: crate::core::IUnknown_Trait {
    fn GetCurrentAddrExclusionList(
        &self,
        riid: ConstPtr<crate::core::GUID>,
        pp_enumerator: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("GetCurrentAddrExclusionList")
    }
    fn UpdateAddrExclusionList(&self, p_enumerator: crate::core::IUnknown) -> crate::core::HRESULT {
        todo!("UpdateAddrExclusionList")
    }
}
impl ::core::clone::Clone for IAddrExclusionControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAddrExclusionControl {}
impl ::core::cmp::PartialEq for IAddrExclusionControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAddrExclusionControl {}
impl ::core::fmt::Debug for IAddrExclusionControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAddrExclusionControl")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IAddrExclusionControl {
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
impl crate::core::ComInterface for IAddrExclusionControl {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000148_0000_0000_c000_000000000046);
}
pub struct IAddrTrackingControl(pub crate::core::IUnknown);
pub trait IAddrTrackingControl_Trait: crate::core::IUnknown_Trait {
    fn EnableCOMDynamicAddrTracking(&self) -> crate::core::HRESULT {
        todo!("EnableCOMDynamicAddrTracking")
    }
    fn DisableCOMDynamicAddrTracking(&self) -> crate::core::HRESULT {
        todo!("DisableCOMDynamicAddrTracking")
    }
}
impl ::core::clone::Clone for IAddrTrackingControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAddrTrackingControl {}
impl ::core::cmp::PartialEq for IAddrTrackingControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAddrTrackingControl {}
impl ::core::fmt::Debug for IAddrTrackingControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAddrTrackingControl")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IAddrTrackingControl {
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
impl crate::core::ComInterface for IAddrTrackingControl {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000147_0000_0000_c000_000000000046);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IAdviseSink(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IAdviseSink_Trait: crate::core::IUnknown_Trait {
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn OnDataChange(&self, p_formatetc: ConstPtr<FORMATETC>, p_stgmed: ConstPtr<STGMEDIUM>) {
        todo!("OnDataChange")
    }
    fn OnViewChange(&self, dw_aspect: u32, lindex: i32) {
        todo!("OnViewChange")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn OnRename(&self, pmk: IMoniker) {
        todo!("OnRename")
    }
    fn OnSave(&self) {
        todo!("OnSave")
    }
    fn OnClose(&self) {
        todo!("OnClose")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IAdviseSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IAdviseSink {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IAdviseSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IAdviseSink {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IAdviseSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAdviseSink").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IAdviseSink {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IAdviseSink {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x0000010f_0000_0000_c000_000000000046);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IAdviseSink2(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IAdviseSink2_Trait: IAdviseSink_Trait {
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn OnLinkSrcChange(&self, pmk: IMoniker) {
        todo!("OnLinkSrcChange")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IAdviseSink2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IAdviseSink2 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IAdviseSink2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IAdviseSink2 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IAdviseSink2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAdviseSink2").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IAdviseSink2 {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IAdviseSink2 {
    type Super = IAdviseSink;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000125_0000_0000_c000_000000000046);
}
pub struct IAgileObject(pub crate::core::IUnknown);
pub trait IAgileObject_Trait: crate::core::IUnknown_Trait {}
impl ::core::clone::Clone for IAgileObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAgileObject {}
impl ::core::cmp::PartialEq for IAgileObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAgileObject {}
impl ::core::fmt::Debug for IAgileObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAgileObject").field(&self.0).finish()
    }
}
impl FromIntoMemory for IAgileObject {
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
impl crate::core::ComInterface for IAgileObject {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x94ea2b94_e9cc_49e0_c0ff_ee64ca8f5b90);
}
pub struct IAsyncManager(pub crate::core::IUnknown);
pub trait IAsyncManager_Trait: crate::core::IUnknown_Trait {
    fn CompleteCall(&self, result: crate::core::HRESULT) -> crate::core::HRESULT {
        todo!("CompleteCall")
    }
    fn GetCallContext(
        &self,
        riid: ConstPtr<crate::core::GUID>,
        p_interface: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("GetCallContext")
    }
    fn GetState(&self, pul_state_flags: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetState")
    }
}
impl ::core::clone::Clone for IAsyncManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAsyncManager {}
impl ::core::cmp::PartialEq for IAsyncManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAsyncManager {}
impl ::core::fmt::Debug for IAsyncManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAsyncManager").field(&self.0).finish()
    }
}
impl FromIntoMemory for IAsyncManager {
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
impl crate::core::ComInterface for IAsyncManager {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x0000002a_0000_0000_c000_000000000046);
}
pub struct IAsyncRpcChannelBuffer(pub crate::core::IUnknown);
pub trait IAsyncRpcChannelBuffer_Trait: IRpcChannelBuffer2_Trait {
    fn Send(
        &self,
        p_msg: MutPtr<RPCOLEMESSAGE>,
        p_sync: ISynchronize,
        pul_status: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("Send")
    }
    fn Receive(
        &self,
        p_msg: MutPtr<RPCOLEMESSAGE>,
        pul_status: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("Receive")
    }
    fn GetDestCtxEx(
        &self,
        p_msg: ConstPtr<RPCOLEMESSAGE>,
        pdw_dest_context: MutPtr<u32>,
        ppv_dest_context: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("GetDestCtxEx")
    }
}
impl ::core::clone::Clone for IAsyncRpcChannelBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAsyncRpcChannelBuffer {}
impl ::core::cmp::PartialEq for IAsyncRpcChannelBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAsyncRpcChannelBuffer {}
impl ::core::fmt::Debug for IAsyncRpcChannelBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAsyncRpcChannelBuffer")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IAsyncRpcChannelBuffer {
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
impl crate::core::ComInterface for IAsyncRpcChannelBuffer {
    type Super = IRpcChannelBuffer2;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xa5029fb6_3c34_11d1_9c99_00c04fb998aa);
}
pub struct IAuthenticate(pub crate::core::IUnknown);
pub trait IAuthenticate_Trait: crate::core::IUnknown_Trait {
    fn Authenticate(
        &self,
        phwnd: MutPtr<super::super::Foundation::HWND>,
        psz_username: MutPtr<PWSTR>,
        psz_password: MutPtr<PWSTR>,
    ) -> crate::core::HRESULT {
        todo!("Authenticate")
    }
}
impl ::core::clone::Clone for IAuthenticate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAuthenticate {}
impl ::core::cmp::PartialEq for IAuthenticate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAuthenticate {}
impl ::core::fmt::Debug for IAuthenticate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAuthenticate").field(&self.0).finish()
    }
}
impl FromIntoMemory for IAuthenticate {
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
impl crate::core::ComInterface for IAuthenticate {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x79eac9d0_baf9_11ce_8c82_00aa004ba90b);
}
pub struct IAuthenticateEx(pub crate::core::IUnknown);
pub trait IAuthenticateEx_Trait: IAuthenticate_Trait {
    fn AuthenticateEx(
        &self,
        phwnd: MutPtr<super::super::Foundation::HWND>,
        psz_username: MutPtr<PWSTR>,
        psz_password: MutPtr<PWSTR>,
        pauthinfo: ConstPtr<AUTHENTICATEINFO>,
    ) -> crate::core::HRESULT {
        todo!("AuthenticateEx")
    }
}
impl ::core::clone::Clone for IAuthenticateEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAuthenticateEx {}
impl ::core::cmp::PartialEq for IAuthenticateEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAuthenticateEx {}
impl ::core::fmt::Debug for IAuthenticateEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAuthenticateEx").field(&self.0).finish()
    }
}
impl FromIntoMemory for IAuthenticateEx {
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
impl crate::core::ComInterface for IAuthenticateEx {
    type Super = IAuthenticate;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x2ad1edaf_d83d_48b5_9adf_03dbe19f53bd);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IBindCtx(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IBindCtx_Trait: crate::core::IUnknown_Trait {
    fn RegisterObjectBound(&self, punk: crate::core::IUnknown) -> crate::core::HRESULT {
        todo!("RegisterObjectBound")
    }
    fn RevokeObjectBound(&self, punk: crate::core::IUnknown) -> crate::core::HRESULT {
        todo!("RevokeObjectBound")
    }
    fn ReleaseBoundObjects(&self) -> crate::core::HRESULT {
        todo!("ReleaseBoundObjects")
    }
    fn SetBindOptions(&self, pbindopts: ConstPtr<BIND_OPTS>) -> crate::core::HRESULT {
        todo!("SetBindOptions")
    }
    fn GetBindOptions(&self, pbindopts: MutPtr<BIND_OPTS>) -> crate::core::HRESULT {
        todo!("GetBindOptions")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetRunningObjectTable(&self, pprot: MutPtr<IRunningObjectTable>) -> crate::core::HRESULT {
        todo!("GetRunningObjectTable")
    }
    fn RegisterObjectParam(
        &self,
        psz_key: PCWSTR,
        punk: crate::core::IUnknown,
    ) -> crate::core::HRESULT {
        todo!("RegisterObjectParam")
    }
    fn GetObjectParam(
        &self,
        psz_key: PCWSTR,
        ppunk: MutPtr<crate::core::IUnknown>,
    ) -> crate::core::HRESULT {
        todo!("GetObjectParam")
    }
    fn EnumObjectParam(&self, ppenum: MutPtr<IEnumString>) -> crate::core::HRESULT {
        todo!("EnumObjectParam")
    }
    fn RevokeObjectParam(&self, psz_key: PCWSTR) -> crate::core::HRESULT {
        todo!("RevokeObjectParam")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IBindCtx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IBindCtx {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IBindCtx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IBindCtx {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IBindCtx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindCtx").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IBindCtx {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IBindCtx {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x0000000e_0000_0000_c000_000000000046);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IBindHost(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IBindHost_Trait: crate::core::IUnknown_Trait {
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateMoniker(
        &self,
        sz_name: PCWSTR,
        p_bc: IBindCtx,
        ppmk: MutPtr<IMoniker>,
        dw_reserved: u32,
    ) -> crate::core::HRESULT {
        todo!("CreateMoniker")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn MonikerBindToStorage(
        &self,
        p_mk: IMoniker,
        p_bc: IBindCtx,
        p_bsc: IBindStatusCallback,
        riid: ConstPtr<crate::core::GUID>,
        ppv_obj: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("MonikerBindToStorage")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn MonikerBindToObject(
        &self,
        p_mk: IMoniker,
        p_bc: IBindCtx,
        p_bsc: IBindStatusCallback,
        riid: ConstPtr<crate::core::GUID>,
        ppv_obj: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("MonikerBindToObject")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IBindHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IBindHost {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IBindHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IBindHost {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IBindHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindHost").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IBindHost {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IBindHost {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xfc4801a1_2ba9_11cf_a229_00aa003d7352);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IBindStatusCallback(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IBindStatusCallback_Trait: crate::core::IUnknown_Trait {
    fn OnStartBinding(&self, dw_reserved: u32, pib: IBinding) -> crate::core::HRESULT {
        todo!("OnStartBinding")
    }
    fn GetPriority(&self, pn_priority: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("GetPriority")
    }
    fn OnLowResource(&self, reserved: u32) -> crate::core::HRESULT {
        todo!("OnLowResource")
    }
    fn OnProgress(
        &self,
        ul_progress: u32,
        ul_progress_max: u32,
        ul_status_code: u32,
        sz_status_text: PCWSTR,
    ) -> crate::core::HRESULT {
        todo!("OnProgress")
    }
    fn OnStopBinding(
        &self,
        hresult: crate::core::HRESULT,
        sz_error: PCWSTR,
    ) -> crate::core::HRESULT {
        todo!("OnStopBinding")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetBindInfo(
        &self,
        grf_bindf: MutPtr<u32>,
        pbindinfo: MutPtr<BINDINFO>,
    ) -> crate::core::HRESULT {
        todo!("GetBindInfo")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn OnDataAvailable(
        &self,
        grf_bscf: u32,
        dw_size: u32,
        pformatetc: ConstPtr<FORMATETC>,
        pstgmed: ConstPtr<STGMEDIUM>,
    ) -> crate::core::HRESULT {
        todo!("OnDataAvailable")
    }
    fn OnObjectAvailable(
        &self,
        riid: ConstPtr<crate::core::GUID>,
        punk: crate::core::IUnknown,
    ) -> crate::core::HRESULT {
        todo!("OnObjectAvailable")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IBindStatusCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IBindStatusCallback {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IBindStatusCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IBindStatusCallback {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IBindStatusCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindStatusCallback").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IBindStatusCallback {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IBindStatusCallback {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x79eac9c1_baf9_11ce_8c82_00aa004ba90b);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IBindStatusCallbackEx(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IBindStatusCallbackEx_Trait: IBindStatusCallback_Trait {
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetBindInfoEx(
        &self,
        grf_bindf: MutPtr<u32>,
        pbindinfo: MutPtr<BINDINFO>,
        grf_bindf_2: MutPtr<u32>,
        pdw_reserved: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("GetBindInfoEx")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IBindStatusCallbackEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IBindStatusCallbackEx {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IBindStatusCallbackEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IBindStatusCallbackEx {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IBindStatusCallbackEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindStatusCallbackEx")
            .field(&self.0)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IBindStatusCallbackEx {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IBindStatusCallbackEx {
    type Super = IBindStatusCallback;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xaaa74ef9_8ee7_4659_88d9_f8c504da73cc);
}
pub struct IBinding(pub crate::core::IUnknown);
pub trait IBinding_Trait: crate::core::IUnknown_Trait {
    fn Abort(&self) -> crate::core::HRESULT {
        todo!("Abort")
    }
    fn Suspend(&self) -> crate::core::HRESULT {
        todo!("Suspend")
    }
    fn Resume(&self) -> crate::core::HRESULT {
        todo!("Resume")
    }
    fn SetPriority(&self, n_priority: i32) -> crate::core::HRESULT {
        todo!("SetPriority")
    }
    fn GetPriority(&self, pn_priority: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("GetPriority")
    }
    fn GetBindResult(
        &self,
        pclsid_protocol: MutPtr<crate::core::GUID>,
        pdw_result: MutPtr<u32>,
        psz_result: MutPtr<PWSTR>,
        pdw_reserved: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("GetBindResult")
    }
}
impl ::core::clone::Clone for IBinding {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IBinding {}
impl ::core::cmp::PartialEq for IBinding {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBinding {}
impl ::core::fmt::Debug for IBinding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBinding").field(&self.0).finish()
    }
}
impl FromIntoMemory for IBinding {
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
impl crate::core::ComInterface for IBinding {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x79eac9c0_baf9_11ce_8c82_00aa004ba90b);
}
pub struct IBlockingLock(pub crate::core::IUnknown);
pub trait IBlockingLock_Trait: crate::core::IUnknown_Trait {
    fn Lock(&self, dw_timeout: u32) -> crate::core::HRESULT {
        todo!("Lock")
    }
    fn Unlock(&self) -> crate::core::HRESULT {
        todo!("Unlock")
    }
}
impl ::core::clone::Clone for IBlockingLock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IBlockingLock {}
impl ::core::cmp::PartialEq for IBlockingLock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBlockingLock {}
impl ::core::fmt::Debug for IBlockingLock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBlockingLock").field(&self.0).finish()
    }
}
impl FromIntoMemory for IBlockingLock {
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
impl crate::core::ComInterface for IBlockingLock {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x30f3d47a_6447_11d1_8e3c_00c04fb9386d);
}
pub struct ICallFactory(pub crate::core::IUnknown);
pub trait ICallFactory_Trait: crate::core::IUnknown_Trait {
    fn CreateCall(
        &self,
        riid: ConstPtr<crate::core::GUID>,
        p_ctrl_unk: crate::core::IUnknown,
        riid_2: ConstPtr<crate::core::GUID>,
        ppv: MutPtr<crate::core::IUnknown>,
    ) -> crate::core::HRESULT {
        todo!("CreateCall")
    }
}
impl ::core::clone::Clone for ICallFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ICallFactory {}
impl ::core::cmp::PartialEq for ICallFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICallFactory {}
impl ::core::fmt::Debug for ICallFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICallFactory").field(&self.0).finish()
    }
}
impl FromIntoMemory for ICallFactory {
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
impl crate::core::ComInterface for ICallFactory {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x1c733a30_2a1c_11ce_ade5_00aa0044773d);
}
pub struct ICancelMethodCalls(pub crate::core::IUnknown);
pub trait ICancelMethodCalls_Trait: crate::core::IUnknown_Trait {
    fn Cancel(&self, ul_seconds: u32) -> crate::core::HRESULT {
        todo!("Cancel")
    }
    fn TestCancel(&self) -> crate::core::HRESULT {
        todo!("TestCancel")
    }
}
impl ::core::clone::Clone for ICancelMethodCalls {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ICancelMethodCalls {}
impl ::core::cmp::PartialEq for ICancelMethodCalls {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICancelMethodCalls {}
impl ::core::fmt::Debug for ICancelMethodCalls {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICancelMethodCalls").field(&self.0).finish()
    }
}
impl FromIntoMemory for ICancelMethodCalls {
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
impl crate::core::ComInterface for ICancelMethodCalls {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000029_0000_0000_c000_000000000046);
}
pub struct ICatInformation(pub crate::core::IUnknown);
pub trait ICatInformation_Trait: crate::core::IUnknown_Trait {
    fn EnumCategories(
        &self,
        lcid: u32,
        ppenum_category_info: MutPtr<IEnumCATEGORYINFO>,
    ) -> crate::core::HRESULT {
        todo!("EnumCategories")
    }
    fn GetCategoryDesc(
        &self,
        rcatid: ConstPtr<crate::core::GUID>,
        lcid: u32,
        psz_desc: MutPtr<PWSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetCategoryDesc")
    }
    fn EnumClassesOfCategories(
        &self,
        c_implemented: u32,
        rgcatid_impl: ConstPtr<crate::core::GUID>,
        c_required: u32,
        rgcatid_req: ConstPtr<crate::core::GUID>,
        ppenum_clsid: MutPtr<IEnumGUID>,
    ) -> crate::core::HRESULT {
        todo!("EnumClassesOfCategories")
    }
    fn IsClassOfCategories(
        &self,
        rclsid: ConstPtr<crate::core::GUID>,
        c_implemented: u32,
        rgcatid_impl: ConstPtr<crate::core::GUID>,
        c_required: u32,
        rgcatid_req: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("IsClassOfCategories")
    }
    fn EnumImplCategoriesOfClass(
        &self,
        rclsid: ConstPtr<crate::core::GUID>,
        ppenum_catid: MutPtr<IEnumGUID>,
    ) -> crate::core::HRESULT {
        todo!("EnumImplCategoriesOfClass")
    }
    fn EnumReqCategoriesOfClass(
        &self,
        rclsid: ConstPtr<crate::core::GUID>,
        ppenum_catid: MutPtr<IEnumGUID>,
    ) -> crate::core::HRESULT {
        todo!("EnumReqCategoriesOfClass")
    }
}
impl ::core::clone::Clone for ICatInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ICatInformation {}
impl ::core::cmp::PartialEq for ICatInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICatInformation {}
impl ::core::fmt::Debug for ICatInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICatInformation").field(&self.0).finish()
    }
}
impl FromIntoMemory for ICatInformation {
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
impl crate::core::ComInterface for ICatInformation {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x0002e013_0000_0000_c000_000000000046);
}
pub struct ICatRegister(pub crate::core::IUnknown);
pub trait ICatRegister_Trait: crate::core::IUnknown_Trait {
    fn RegisterCategories(
        &self,
        c_categories: u32,
        rg_category_info: ConstPtr<CATEGORYINFO>,
    ) -> crate::core::HRESULT {
        todo!("RegisterCategories")
    }
    fn UnRegisterCategories(
        &self,
        c_categories: u32,
        rgcatid: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("UnRegisterCategories")
    }
    fn RegisterClassImplCategories(
        &self,
        rclsid: ConstPtr<crate::core::GUID>,
        c_categories: u32,
        rgcatid: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("RegisterClassImplCategories")
    }
    fn UnRegisterClassImplCategories(
        &self,
        rclsid: ConstPtr<crate::core::GUID>,
        c_categories: u32,
        rgcatid: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("UnRegisterClassImplCategories")
    }
    fn RegisterClassReqCategories(
        &self,
        rclsid: ConstPtr<crate::core::GUID>,
        c_categories: u32,
        rgcatid: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("RegisterClassReqCategories")
    }
    fn UnRegisterClassReqCategories(
        &self,
        rclsid: ConstPtr<crate::core::GUID>,
        c_categories: u32,
        rgcatid: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("UnRegisterClassReqCategories")
    }
}
impl ::core::clone::Clone for ICatRegister {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ICatRegister {}
impl ::core::cmp::PartialEq for ICatRegister {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICatRegister {}
impl ::core::fmt::Debug for ICatRegister {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICatRegister").field(&self.0).finish()
    }
}
impl FromIntoMemory for ICatRegister {
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
impl crate::core::ComInterface for ICatRegister {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x0002e012_0000_0000_c000_000000000046);
}
pub struct IChannelHook(pub crate::core::IUnknown);
pub trait IChannelHook_Trait: crate::core::IUnknown_Trait {
    fn ClientGetSize(
        &self,
        u_extent: ConstPtr<crate::core::GUID>,
        riid: ConstPtr<crate::core::GUID>,
        p_data_size: MutPtr<u32>,
    ) {
        todo!("ClientGetSize")
    }
    fn ClientFillBuffer(
        &self,
        u_extent: ConstPtr<crate::core::GUID>,
        riid: ConstPtr<crate::core::GUID>,
        p_data_size: MutPtr<u32>,
        p_data_buffer: ConstPtr<::core::ffi::c_void>,
    ) {
        todo!("ClientFillBuffer")
    }
    fn ClientNotify(
        &self,
        u_extent: ConstPtr<crate::core::GUID>,
        riid: ConstPtr<crate::core::GUID>,
        cb_data_size: u32,
        p_data_buffer: ConstPtr<::core::ffi::c_void>,
        l_data_rep: u32,
        hr_fault: crate::core::HRESULT,
    ) {
        todo!("ClientNotify")
    }
    fn ServerNotify(
        &self,
        u_extent: ConstPtr<crate::core::GUID>,
        riid: ConstPtr<crate::core::GUID>,
        cb_data_size: u32,
        p_data_buffer: ConstPtr<::core::ffi::c_void>,
        l_data_rep: u32,
    ) {
        todo!("ServerNotify")
    }
    fn ServerGetSize(
        &self,
        u_extent: ConstPtr<crate::core::GUID>,
        riid: ConstPtr<crate::core::GUID>,
        hr_fault: crate::core::HRESULT,
        p_data_size: MutPtr<u32>,
    ) {
        todo!("ServerGetSize")
    }
    fn ServerFillBuffer(
        &self,
        u_extent: ConstPtr<crate::core::GUID>,
        riid: ConstPtr<crate::core::GUID>,
        p_data_size: MutPtr<u32>,
        p_data_buffer: ConstPtr<::core::ffi::c_void>,
        hr_fault: crate::core::HRESULT,
    ) {
        todo!("ServerFillBuffer")
    }
}
impl ::core::clone::Clone for IChannelHook {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IChannelHook {}
impl ::core::cmp::PartialEq for IChannelHook {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IChannelHook {}
impl ::core::fmt::Debug for IChannelHook {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IChannelHook").field(&self.0).finish()
    }
}
impl FromIntoMemory for IChannelHook {
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
impl crate::core::ComInterface for IChannelHook {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x1008c4a0_7613_11cf_9af1_0020af6e72f4);
}
pub struct IClassActivator(pub crate::core::IUnknown);
pub trait IClassActivator_Trait: crate::core::IUnknown_Trait {
    fn GetClassObject(
        &self,
        rclsid: ConstPtr<crate::core::GUID>,
        dw_class_context: u32,
        locale: u32,
        riid: ConstPtr<crate::core::GUID>,
        ppv: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("GetClassObject")
    }
}
impl ::core::clone::Clone for IClassActivator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IClassActivator {}
impl ::core::cmp::PartialEq for IClassActivator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IClassActivator {}
impl ::core::fmt::Debug for IClassActivator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IClassActivator").field(&self.0).finish()
    }
}
impl FromIntoMemory for IClassActivator {
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
impl crate::core::ComInterface for IClassActivator {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000140_0000_0000_c000_000000000046);
}
pub struct IClassFactory(pub crate::core::IUnknown);
pub trait IClassFactory_Trait: crate::core::IUnknown_Trait {
    fn CreateInstance(
        &self,
        p_unk_outer: crate::core::IUnknown,
        riid: ConstPtr<crate::core::GUID>,
        ppv_object: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("CreateInstance")
    }
    fn LockServer(&self, f_lock: super::super::Foundation::BOOL) -> crate::core::HRESULT {
        todo!("LockServer")
    }
}
impl ::core::clone::Clone for IClassFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IClassFactory {}
impl ::core::cmp::PartialEq for IClassFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IClassFactory {}
impl ::core::fmt::Debug for IClassFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IClassFactory").field(&self.0).finish()
    }
}
impl FromIntoMemory for IClassFactory {
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
impl crate::core::ComInterface for IClassFactory {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000001_0000_0000_c000_000000000046);
}
pub struct IClientSecurity(pub crate::core::IUnknown);
pub trait IClientSecurity_Trait: crate::core::IUnknown_Trait {
    fn QueryBlanket(
        &self,
        p_proxy: crate::core::IUnknown,
        p_authn_svc: MutPtr<u32>,
        p_authz_svc: MutPtr<u32>,
        p_server_princ_name: MutPtr<ConstPtr<u16>>,
        p_authn_level: MutPtr<RPC_C_AUTHN_LEVEL>,
        p_imp_level: MutPtr<RPC_C_IMP_LEVEL>,
        p_auth_info: MutPtr<ConstPtr<::core::ffi::c_void>>,
        p_capabilites: MutPtr<EOLE_AUTHENTICATION_CAPABILITIES>,
    ) -> crate::core::HRESULT {
        todo!("QueryBlanket")
    }
    fn SetBlanket(
        &self,
        p_proxy: crate::core::IUnknown,
        dw_authn_svc: u32,
        dw_authz_svc: u32,
        p_server_princ_name: PCWSTR,
        dw_authn_level: RPC_C_AUTHN_LEVEL,
        dw_imp_level: RPC_C_IMP_LEVEL,
        p_auth_info: ConstPtr<::core::ffi::c_void>,
        dw_capabilities: EOLE_AUTHENTICATION_CAPABILITIES,
    ) -> crate::core::HRESULT {
        todo!("SetBlanket")
    }
    fn CopyProxy(
        &self,
        p_proxy: crate::core::IUnknown,
        pp_copy: MutPtr<crate::core::IUnknown>,
    ) -> crate::core::HRESULT {
        todo!("CopyProxy")
    }
}
impl ::core::clone::Clone for IClientSecurity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IClientSecurity {}
impl ::core::cmp::PartialEq for IClientSecurity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IClientSecurity {}
impl ::core::fmt::Debug for IClientSecurity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IClientSecurity").field(&self.0).finish()
    }
}
impl FromIntoMemory for IClientSecurity {
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
impl crate::core::ComInterface for IClientSecurity {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x0000013d_0000_0000_c000_000000000046);
}
pub struct IComThreadingInfo(pub crate::core::IUnknown);
pub trait IComThreadingInfo_Trait: crate::core::IUnknown_Trait {
    fn GetCurrentApartmentType(&self, p_apt_type: MutPtr<APTTYPE>) -> crate::core::HRESULT {
        todo!("GetCurrentApartmentType")
    }
    fn GetCurrentThreadType(&self, p_thread_type: MutPtr<THDTYPE>) -> crate::core::HRESULT {
        todo!("GetCurrentThreadType")
    }
    fn GetCurrentLogicalThreadId(
        &self,
        pguid_logical_thread_id: MutPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("GetCurrentLogicalThreadId")
    }
    fn SetCurrentLogicalThreadId(
        &self,
        rguid: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("SetCurrentLogicalThreadId")
    }
}
impl ::core::clone::Clone for IComThreadingInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IComThreadingInfo {}
impl ::core::cmp::PartialEq for IComThreadingInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComThreadingInfo {}
impl ::core::fmt::Debug for IComThreadingInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComThreadingInfo").field(&self.0).finish()
    }
}
impl FromIntoMemory for IComThreadingInfo {
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
impl crate::core::ComInterface for IComThreadingInfo {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x000001ce_0000_0000_c000_000000000046);
}
pub struct IConnectionPoint(pub crate::core::IUnknown);
pub trait IConnectionPoint_Trait: crate::core::IUnknown_Trait {
    fn GetConnectionInterface(&self, p_iid: MutPtr<crate::core::GUID>) -> crate::core::HRESULT {
        todo!("GetConnectionInterface")
    }
    fn GetConnectionPointContainer(
        &self,
        pp_cpc: MutPtr<IConnectionPointContainer>,
    ) -> crate::core::HRESULT {
        todo!("GetConnectionPointContainer")
    }
    fn Advise(
        &self,
        p_unk_sink: crate::core::IUnknown,
        pdw_cookie: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("Advise")
    }
    fn Unadvise(&self, dw_cookie: u32) -> crate::core::HRESULT {
        todo!("Unadvise")
    }
    fn EnumConnections(&self, pp_enum: MutPtr<IEnumConnections>) -> crate::core::HRESULT {
        todo!("EnumConnections")
    }
}
impl ::core::clone::Clone for IConnectionPoint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IConnectionPoint {}
impl ::core::cmp::PartialEq for IConnectionPoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConnectionPoint {}
impl ::core::fmt::Debug for IConnectionPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConnectionPoint").field(&self.0).finish()
    }
}
impl FromIntoMemory for IConnectionPoint {
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
impl crate::core::ComInterface for IConnectionPoint {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xb196b286_bab4_101a_b69c_00aa00341d07);
}
pub struct IConnectionPointContainer(pub crate::core::IUnknown);
pub trait IConnectionPointContainer_Trait: crate::core::IUnknown_Trait {
    fn EnumConnectionPoints(&self, pp_enum: MutPtr<IEnumConnectionPoints>) -> crate::core::HRESULT {
        todo!("EnumConnectionPoints")
    }
    fn FindConnectionPoint(
        &self,
        riid: ConstPtr<crate::core::GUID>,
        pp_cp: MutPtr<IConnectionPoint>,
    ) -> crate::core::HRESULT {
        todo!("FindConnectionPoint")
    }
}
impl ::core::clone::Clone for IConnectionPointContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IConnectionPointContainer {}
impl ::core::cmp::PartialEq for IConnectionPointContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConnectionPointContainer {}
impl ::core::fmt::Debug for IConnectionPointContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConnectionPointContainer")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IConnectionPointContainer {
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
impl crate::core::ComInterface for IConnectionPointContainer {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xb196b284_bab4_101a_b69c_00aa00341d07);
}
pub struct IContext(pub u8);
pub struct IContextCallback(pub crate::core::IUnknown);
pub trait IContextCallback_Trait: crate::core::IUnknown_Trait {
    fn ContextCallback(
        &self,
        pfn_callback: PFNCONTEXTCALL,
        p_param: ConstPtr<ComCallData>,
        riid: ConstPtr<crate::core::GUID>,
        i_method: i32,
        p_unk: crate::core::IUnknown,
    ) -> crate::core::HRESULT {
        todo!("ContextCallback")
    }
}
impl ::core::clone::Clone for IContextCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IContextCallback {}
impl ::core::cmp::PartialEq for IContextCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContextCallback {}
impl ::core::fmt::Debug for IContextCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContextCallback").field(&self.0).finish()
    }
}
impl FromIntoMemory for IContextCallback {
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
impl crate::core::ComInterface for IContextCallback {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x000001da_0000_0000_c000_000000000046);
}
pub struct IDLDESC {
    pub dwReserved: PtrRepr,
    pub wIDLFlags: u16,
}
impl ::core::marker::Copy for IDLDESC {}
impl ::core::clone::Clone for IDLDESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IDLDESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IDLDESC")
            .field("dwReserved", &self.dwReserved)
            .field("wIDLFlags", &self.wIDLFlags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IDLDESC {
    fn eq(&self, other: &Self) -> bool {
        self.dwReserved == other.dwReserved && self.wIDLFlags == other.wIDLFlags
    }
}
impl ::core::cmp::Eq for IDLDESC {}
impl FromIntoMemory for IDLDESC {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_dwReserved = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_wIDLFlags = <u16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        Self {
            dwReserved: f_dwReserved,
            wIDLFlags: f_wIDLFlags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.dwReserved, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.wIDLFlags, &mut into[4..4 + 2]);
    }
    fn size() -> usize {
        8
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IDataAdviseHolder(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IDataAdviseHolder_Trait: crate::core::IUnknown_Trait {
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Advise(
        &self,
        p_data_object: IDataObject,
        p_fetc: ConstPtr<FORMATETC>,
        advf: u32,
        p_advise: IAdviseSink,
        pdw_connection: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("Advise")
    }
    fn Unadvise(&self, dw_connection: u32) -> crate::core::HRESULT {
        todo!("Unadvise")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn EnumAdvise(&self, ppenum_advise: MutPtr<IEnumSTATDATA>) -> crate::core::HRESULT {
        todo!("EnumAdvise")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn SendOnDataChange(
        &self,
        p_data_object: IDataObject,
        dw_reserved: u32,
        advf: u32,
    ) -> crate::core::HRESULT {
        todo!("SendOnDataChange")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IDataAdviseHolder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IDataAdviseHolder {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IDataAdviseHolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IDataAdviseHolder {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IDataAdviseHolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataAdviseHolder").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IDataAdviseHolder {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IDataAdviseHolder {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000110_0000_0000_c000_000000000046);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IDataObject(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IDataObject_Trait: crate::core::IUnknown_Trait {
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetData(
        &self,
        pformatetc_in: ConstPtr<FORMATETC>,
        pmedium: MutPtr<STGMEDIUM>,
    ) -> crate::core::HRESULT {
        todo!("GetData")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetDataHere(
        &self,
        pformatetc: ConstPtr<FORMATETC>,
        pmedium: MutPtr<STGMEDIUM>,
    ) -> crate::core::HRESULT {
        todo!("GetDataHere")
    }
    fn QueryGetData(&self, pformatetc: ConstPtr<FORMATETC>) -> crate::core::HRESULT {
        todo!("QueryGetData")
    }
    fn GetCanonicalFormatEtc(
        &self,
        pformatect_in: ConstPtr<FORMATETC>,
        pformatetc_out: MutPtr<FORMATETC>,
    ) -> crate::core::HRESULT {
        todo!("GetCanonicalFormatEtc")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn SetData(
        &self,
        pformatetc: ConstPtr<FORMATETC>,
        pmedium: ConstPtr<STGMEDIUM>,
        f_release: super::super::Foundation::BOOL,
    ) -> crate::core::HRESULT {
        todo!("SetData")
    }
    fn EnumFormatEtc(
        &self,
        dw_direction: u32,
        ppenum_format_etc: MutPtr<IEnumFORMATETC>,
    ) -> crate::core::HRESULT {
        todo!("EnumFormatEtc")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn DAdvise(
        &self,
        pformatetc: ConstPtr<FORMATETC>,
        advf: u32,
        p_adv_sink: IAdviseSink,
        pdw_connection: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("DAdvise")
    }
    fn DUnadvise(&self, dw_connection: u32) -> crate::core::HRESULT {
        todo!("DUnadvise")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn EnumDAdvise(&self, ppenum_advise: MutPtr<IEnumSTATDATA>) -> crate::core::HRESULT {
        todo!("EnumDAdvise")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IDataObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IDataObject {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IDataObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IDataObject {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IDataObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataObject").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IDataObject {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IDataObject {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x0000010e_0000_0000_c000_000000000046);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IDispatch(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IDispatch_Trait: crate::core::IUnknown_Trait {
    fn GetTypeInfoCount(&self, pctinfo: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetTypeInfoCount")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetTypeInfo(
        &self,
        i_t_info: u32,
        lcid: u32,
        pp_t_info: MutPtr<ITypeInfo>,
    ) -> crate::core::HRESULT {
        todo!("GetTypeInfo")
    }
    fn GetIDsOfNames(
        &self,
        riid: ConstPtr<crate::core::GUID>,
        rgsz_names: ConstPtr<PWSTR>,
        c_names: u32,
        lcid: u32,
        rg_disp_id: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("GetIDsOfNames")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Invoke(
        &self,
        disp_id_member: i32,
        riid: ConstPtr<crate::core::GUID>,
        lcid: u32,
        w_flags: u16,
        p_disp_params: ConstPtr<DISPPARAMS>,
        p_var_result: MutPtr<VARIANT>,
        p_excep_info: MutPtr<EXCEPINFO>,
        pu_arg_err: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("Invoke")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IDispatch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IDispatch {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IDispatch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IDispatch {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IDispatch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDispatch").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IDispatch {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IDispatch {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00020400_0000_0000_c000_000000000046);
}
pub struct IEnumCATEGORYINFO(pub crate::core::IUnknown);
pub trait IEnumCATEGORYINFO_Trait: crate::core::IUnknown_Trait {
    fn Next(
        &self,
        celt: u32,
        rgelt: MutPtr<CATEGORYINFO>,
        pcelt_fetched: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("Next")
    }
    fn Skip(&self, celt: u32) -> crate::core::HRESULT {
        todo!("Skip")
    }
    fn Reset(&self) -> crate::core::HRESULT {
        todo!("Reset")
    }
    fn Clone(&self, ppenum: MutPtr<IEnumCATEGORYINFO>) -> crate::core::HRESULT {
        todo!("Clone")
    }
}
impl ::core::clone::Clone for IEnumCATEGORYINFO {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IEnumCATEGORYINFO {}
impl ::core::cmp::PartialEq for IEnumCATEGORYINFO {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumCATEGORYINFO {}
impl ::core::fmt::Debug for IEnumCATEGORYINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumCATEGORYINFO").field(&self.0).finish()
    }
}
impl FromIntoMemory for IEnumCATEGORYINFO {
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
impl crate::core::ComInterface for IEnumCATEGORYINFO {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x0002e011_0000_0000_c000_000000000046);
}
pub struct IEnumConnectionPoints(pub crate::core::IUnknown);
pub trait IEnumConnectionPoints_Trait: crate::core::IUnknown_Trait {
    fn Next(
        &self,
        c_connections: u32,
        pp_cp: MutPtr<IConnectionPoint>,
        pc_fetched: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("Next")
    }
    fn Skip(&self, c_connections: u32) -> crate::core::HRESULT {
        todo!("Skip")
    }
    fn Reset(&self) -> crate::core::HRESULT {
        todo!("Reset")
    }
    fn Clone(&self, pp_enum: MutPtr<IEnumConnectionPoints>) -> crate::core::HRESULT {
        todo!("Clone")
    }
}
impl ::core::clone::Clone for IEnumConnectionPoints {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IEnumConnectionPoints {}
impl ::core::cmp::PartialEq for IEnumConnectionPoints {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumConnectionPoints {}
impl ::core::fmt::Debug for IEnumConnectionPoints {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumConnectionPoints")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IEnumConnectionPoints {
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
impl crate::core::ComInterface for IEnumConnectionPoints {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xb196b285_bab4_101a_b69c_00aa00341d07);
}
pub struct IEnumConnections(pub crate::core::IUnknown);
pub trait IEnumConnections_Trait: crate::core::IUnknown_Trait {
    fn Next(
        &self,
        c_connections: u32,
        rgcd: MutPtr<CONNECTDATA>,
        pc_fetched: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("Next")
    }
    fn Skip(&self, c_connections: u32) -> crate::core::HRESULT {
        todo!("Skip")
    }
    fn Reset(&self) -> crate::core::HRESULT {
        todo!("Reset")
    }
    fn Clone(&self, pp_enum: MutPtr<IEnumConnections>) -> crate::core::HRESULT {
        todo!("Clone")
    }
}
impl ::core::clone::Clone for IEnumConnections {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IEnumConnections {}
impl ::core::cmp::PartialEq for IEnumConnections {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumConnections {}
impl ::core::fmt::Debug for IEnumConnections {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumConnections").field(&self.0).finish()
    }
}
impl FromIntoMemory for IEnumConnections {
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
impl crate::core::ComInterface for IEnumConnections {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xb196b287_bab4_101a_b69c_00aa00341d07);
}
pub struct IEnumContextProps(pub u8);
pub struct IEnumFORMATETC(pub crate::core::IUnknown);
pub trait IEnumFORMATETC_Trait: crate::core::IUnknown_Trait {
    fn Next(
        &self,
        celt: u32,
        rgelt: MutPtr<FORMATETC>,
        pcelt_fetched: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("Next")
    }
    fn Skip(&self, celt: u32) -> crate::core::HRESULT {
        todo!("Skip")
    }
    fn Reset(&self) -> crate::core::HRESULT {
        todo!("Reset")
    }
    fn Clone(&self, ppenum: MutPtr<IEnumFORMATETC>) -> crate::core::HRESULT {
        todo!("Clone")
    }
}
impl ::core::clone::Clone for IEnumFORMATETC {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IEnumFORMATETC {}
impl ::core::cmp::PartialEq for IEnumFORMATETC {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumFORMATETC {}
impl ::core::fmt::Debug for IEnumFORMATETC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumFORMATETC").field(&self.0).finish()
    }
}
impl FromIntoMemory for IEnumFORMATETC {
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
impl crate::core::ComInterface for IEnumFORMATETC {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000103_0000_0000_c000_000000000046);
}
pub struct IEnumGUID(pub crate::core::IUnknown);
pub trait IEnumGUID_Trait: crate::core::IUnknown_Trait {
    fn Next(
        &self,
        celt: u32,
        rgelt: MutPtr<crate::core::GUID>,
        pcelt_fetched: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("Next")
    }
    fn Skip(&self, celt: u32) -> crate::core::HRESULT {
        todo!("Skip")
    }
    fn Reset(&self) -> crate::core::HRESULT {
        todo!("Reset")
    }
    fn Clone(&self, ppenum: MutPtr<IEnumGUID>) -> crate::core::HRESULT {
        todo!("Clone")
    }
}
impl ::core::clone::Clone for IEnumGUID {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IEnumGUID {}
impl ::core::cmp::PartialEq for IEnumGUID {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumGUID {}
impl ::core::fmt::Debug for IEnumGUID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumGUID").field(&self.0).finish()
    }
}
impl FromIntoMemory for IEnumGUID {
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
impl crate::core::ComInterface for IEnumGUID {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x0002e000_0000_0000_c000_000000000046);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IEnumMoniker(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IEnumMoniker_Trait: crate::core::IUnknown_Trait {
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Next(
        &self,
        celt: u32,
        rgelt: MutPtr<IMoniker>,
        pcelt_fetched: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("Next")
    }
    fn Skip(&self, celt: u32) -> crate::core::HRESULT {
        todo!("Skip")
    }
    fn Reset(&self) -> crate::core::HRESULT {
        todo!("Reset")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Clone(&self, ppenum: MutPtr<IEnumMoniker>) -> crate::core::HRESULT {
        todo!("Clone")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IEnumMoniker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IEnumMoniker {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IEnumMoniker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IEnumMoniker {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IEnumMoniker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumMoniker").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IEnumMoniker {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IEnumMoniker {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000102_0000_0000_c000_000000000046);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IEnumSTATDATA(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IEnumSTATDATA_Trait: crate::core::IUnknown_Trait {
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Next(
        &self,
        celt: u32,
        rgelt: MutPtr<STATDATA>,
        pcelt_fetched: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("Next")
    }
    fn Skip(&self, celt: u32) -> crate::core::HRESULT {
        todo!("Skip")
    }
    fn Reset(&self) -> crate::core::HRESULT {
        todo!("Reset")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Clone(&self, ppenum: MutPtr<IEnumSTATDATA>) -> crate::core::HRESULT {
        todo!("Clone")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IEnumSTATDATA {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IEnumSTATDATA {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IEnumSTATDATA {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IEnumSTATDATA {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IEnumSTATDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSTATDATA").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IEnumSTATDATA {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IEnumSTATDATA {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000105_0000_0000_c000_000000000046);
}
pub struct IEnumString(pub crate::core::IUnknown);
pub trait IEnumString_Trait: crate::core::IUnknown_Trait {
    fn Next(
        &self,
        celt: u32,
        rgelt: MutPtr<PWSTR>,
        pcelt_fetched: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("Next")
    }
    fn Skip(&self, celt: u32) -> crate::core::HRESULT {
        todo!("Skip")
    }
    fn Reset(&self) -> crate::core::HRESULT {
        todo!("Reset")
    }
    fn Clone(&self, ppenum: MutPtr<IEnumString>) -> crate::core::HRESULT {
        todo!("Clone")
    }
}
impl ::core::clone::Clone for IEnumString {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IEnumString {}
impl ::core::cmp::PartialEq for IEnumString {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumString {}
impl ::core::fmt::Debug for IEnumString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumString").field(&self.0).finish()
    }
}
impl FromIntoMemory for IEnumString {
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
impl crate::core::ComInterface for IEnumString {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000101_0000_0000_c000_000000000046);
}
pub struct IEnumUnknown(pub crate::core::IUnknown);
pub trait IEnumUnknown_Trait: crate::core::IUnknown_Trait {
    fn Next(
        &self,
        celt: u32,
        rgelt: MutPtr<crate::core::IUnknown>,
        pcelt_fetched: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("Next")
    }
    fn Skip(&self, celt: u32) -> crate::core::HRESULT {
        todo!("Skip")
    }
    fn Reset(&self) -> crate::core::HRESULT {
        todo!("Reset")
    }
    fn Clone(&self, ppenum: MutPtr<IEnumUnknown>) -> crate::core::HRESULT {
        todo!("Clone")
    }
}
impl ::core::clone::Clone for IEnumUnknown {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IEnumUnknown {}
impl ::core::cmp::PartialEq for IEnumUnknown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumUnknown {}
impl ::core::fmt::Debug for IEnumUnknown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumUnknown").field(&self.0).finish()
    }
}
impl FromIntoMemory for IEnumUnknown {
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
impl crate::core::ComInterface for IEnumUnknown {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000100_0000_0000_c000_000000000046);
}
pub struct IErrorInfo(pub crate::core::IUnknown);
pub trait IErrorInfo_Trait: crate::core::IUnknown_Trait {
    fn GetGUID(&self, p_guid: MutPtr<crate::core::GUID>) -> crate::core::HRESULT {
        todo!("GetGUID")
    }
    fn GetSource(
        &self,
        p_bstr_source: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetSource")
    }
    fn GetDescription(
        &self,
        p_bstr_description: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetDescription")
    }
    fn GetHelpFile(
        &self,
        p_bstr_help_file: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetHelpFile")
    }
    fn GetHelpContext(&self, pdw_help_context: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetHelpContext")
    }
}
impl ::core::clone::Clone for IErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IErrorInfo {}
impl ::core::cmp::PartialEq for IErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IErrorInfo {}
impl ::core::fmt::Debug for IErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IErrorInfo").field(&self.0).finish()
    }
}
impl FromIntoMemory for IErrorInfo {
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
impl crate::core::ComInterface for IErrorInfo {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x1cf2b120_547d_101b_8e65_08002b2bd119);
}
pub struct IErrorLog(pub crate::core::IUnknown);
pub trait IErrorLog_Trait: crate::core::IUnknown_Trait {
    fn AddError(
        &self,
        psz_prop_name: PCWSTR,
        p_excep_info: ConstPtr<EXCEPINFO>,
    ) -> crate::core::HRESULT {
        todo!("AddError")
    }
}
impl ::core::clone::Clone for IErrorLog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IErrorLog {}
impl ::core::cmp::PartialEq for IErrorLog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IErrorLog {}
impl ::core::fmt::Debug for IErrorLog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IErrorLog").field(&self.0).finish()
    }
}
impl FromIntoMemory for IErrorLog {
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
impl crate::core::ComInterface for IErrorLog {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x3127ca40_446e_11ce_8135_00aa004bb851);
}
pub struct IExternalConnection(pub crate::core::IUnknown);
pub trait IExternalConnection_Trait: crate::core::IUnknown_Trait {
    fn AddConnection(&self, extconn: u32, reserved: u32) -> u32 {
        todo!("AddConnection")
    }
    fn ReleaseConnection(
        &self,
        extconn: u32,
        reserved: u32,
        f_last_release_closes: super::super::Foundation::BOOL,
    ) -> u32 {
        todo!("ReleaseConnection")
    }
}
impl ::core::clone::Clone for IExternalConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IExternalConnection {}
impl ::core::cmp::PartialEq for IExternalConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExternalConnection {}
impl ::core::fmt::Debug for IExternalConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExternalConnection").field(&self.0).finish()
    }
}
impl FromIntoMemory for IExternalConnection {
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
impl crate::core::ComInterface for IExternalConnection {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000019_0000_0000_c000_000000000046);
}
pub struct IFastRundown(pub crate::core::IUnknown);
pub trait IFastRundown_Trait: crate::core::IUnknown_Trait {}
impl ::core::clone::Clone for IFastRundown {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IFastRundown {}
impl ::core::cmp::PartialEq for IFastRundown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFastRundown {}
impl ::core::fmt::Debug for IFastRundown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFastRundown").field(&self.0).finish()
    }
}
impl FromIntoMemory for IFastRundown {
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
impl crate::core::ComInterface for IFastRundown {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000040_0000_0000_c000_000000000046);
}
pub struct IForegroundTransfer(pub crate::core::IUnknown);
pub trait IForegroundTransfer_Trait: crate::core::IUnknown_Trait {
    fn AllowForegroundTransfer(
        &self,
        lpv_reserved: MutPtr<::core::ffi::c_void>,
    ) -> crate::core::HRESULT {
        todo!("AllowForegroundTransfer")
    }
}
impl ::core::clone::Clone for IForegroundTransfer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IForegroundTransfer {}
impl ::core::cmp::PartialEq for IForegroundTransfer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IForegroundTransfer {}
impl ::core::fmt::Debug for IForegroundTransfer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IForegroundTransfer").field(&self.0).finish()
    }
}
impl FromIntoMemory for IForegroundTransfer {
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
impl crate::core::ComInterface for IForegroundTransfer {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000145_0000_0000_c000_000000000046);
}
pub struct IGlobalInterfaceTable(pub crate::core::IUnknown);
pub trait IGlobalInterfaceTable_Trait: crate::core::IUnknown_Trait {
    fn RegisterInterfaceInGlobal(
        &self,
        p_unk: crate::core::IUnknown,
        riid: ConstPtr<crate::core::GUID>,
        pdw_cookie: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("RegisterInterfaceInGlobal")
    }
    fn RevokeInterfaceFromGlobal(&self, dw_cookie: u32) -> crate::core::HRESULT {
        todo!("RevokeInterfaceFromGlobal")
    }
    fn GetInterfaceFromGlobal(
        &self,
        dw_cookie: u32,
        riid: ConstPtr<crate::core::GUID>,
        ppv: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("GetInterfaceFromGlobal")
    }
}
impl ::core::clone::Clone for IGlobalInterfaceTable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IGlobalInterfaceTable {}
impl ::core::cmp::PartialEq for IGlobalInterfaceTable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGlobalInterfaceTable {}
impl ::core::fmt::Debug for IGlobalInterfaceTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGlobalInterfaceTable")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IGlobalInterfaceTable {
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
impl crate::core::ComInterface for IGlobalInterfaceTable {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000146_0000_0000_c000_000000000046);
}
pub struct IGlobalOptions(pub crate::core::IUnknown);
pub trait IGlobalOptions_Trait: crate::core::IUnknown_Trait {
    fn Set(&self, dw_property: GLOBALOPT_PROPERTIES, dw_value: PtrRepr) -> crate::core::HRESULT {
        todo!("Set")
    }
    fn Query(
        &self,
        dw_property: GLOBALOPT_PROPERTIES,
        pdw_value: MutPtr<PtrRepr>,
    ) -> crate::core::HRESULT {
        todo!("Query")
    }
}
impl ::core::clone::Clone for IGlobalOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IGlobalOptions {}
impl ::core::cmp::PartialEq for IGlobalOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGlobalOptions {}
impl ::core::fmt::Debug for IGlobalOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGlobalOptions").field(&self.0).finish()
    }
}
impl FromIntoMemory for IGlobalOptions {
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
impl crate::core::ComInterface for IGlobalOptions {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x0000015b_0000_0000_c000_000000000046);
}
pub struct IInitializeSpy(pub crate::core::IUnknown);
pub trait IInitializeSpy_Trait: crate::core::IUnknown_Trait {
    fn PreInitialize(&self, dw_co_init: u32, dw_cur_thread_apt_refs: u32) -> crate::core::HRESULT {
        todo!("PreInitialize")
    }
    fn PostInitialize(
        &self,
        hr_co_init: crate::core::HRESULT,
        dw_co_init: u32,
        dw_new_thread_apt_refs: u32,
    ) -> crate::core::HRESULT {
        todo!("PostInitialize")
    }
    fn PreUninitialize(&self, dw_cur_thread_apt_refs: u32) -> crate::core::HRESULT {
        todo!("PreUninitialize")
    }
    fn PostUninitialize(&self, dw_new_thread_apt_refs: u32) -> crate::core::HRESULT {
        todo!("PostUninitialize")
    }
}
impl ::core::clone::Clone for IInitializeSpy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IInitializeSpy {}
impl ::core::cmp::PartialEq for IInitializeSpy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInitializeSpy {}
impl ::core::fmt::Debug for IInitializeSpy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInitializeSpy").field(&self.0).finish()
    }
}
impl FromIntoMemory for IInitializeSpy {
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
impl crate::core::ComInterface for IInitializeSpy {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000034_0000_0000_c000_000000000046);
}
pub struct IInternalUnknown(pub crate::core::IUnknown);
pub trait IInternalUnknown_Trait: crate::core::IUnknown_Trait {
    fn QueryInternalInterface(
        &self,
        riid: ConstPtr<crate::core::GUID>,
        ppv: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("QueryInternalInterface")
    }
}
impl ::core::clone::Clone for IInternalUnknown {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IInternalUnknown {}
impl ::core::cmp::PartialEq for IInternalUnknown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternalUnknown {}
impl ::core::fmt::Debug for IInternalUnknown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternalUnknown").field(&self.0).finish()
    }
}
impl FromIntoMemory for IInternalUnknown {
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
impl crate::core::ComInterface for IInternalUnknown {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000021_0000_0000_c000_000000000046);
}
pub struct IMachineGlobalObjectTable(pub crate::core::IUnknown);
pub trait IMachineGlobalObjectTable_Trait: crate::core::IUnknown_Trait {
    fn RegisterObject(
        &self,
        clsid: ConstPtr<crate::core::GUID>,
        identifier: PCWSTR,
        object: crate::core::IUnknown,
        token: MutPtr<ConstPtr<MachineGlobalObjectTableRegistrationToken__>>,
    ) -> crate::core::HRESULT {
        todo!("RegisterObject")
    }
    fn GetObject(
        &self,
        clsid: ConstPtr<crate::core::GUID>,
        identifier: PCWSTR,
        riid: ConstPtr<crate::core::GUID>,
        ppv: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("GetObject")
    }
    fn RevokeObject(
        &self,
        token: ConstPtr<MachineGlobalObjectTableRegistrationToken__>,
    ) -> crate::core::HRESULT {
        todo!("RevokeObject")
    }
}
impl ::core::clone::Clone for IMachineGlobalObjectTable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IMachineGlobalObjectTable {}
impl ::core::cmp::PartialEq for IMachineGlobalObjectTable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMachineGlobalObjectTable {}
impl ::core::fmt::Debug for IMachineGlobalObjectTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMachineGlobalObjectTable")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IMachineGlobalObjectTable {
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
impl crate::core::ComInterface for IMachineGlobalObjectTable {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x26d709ac_f70b_4421_a96f_d2878fafb00d);
}
pub struct IMalloc(pub crate::core::IUnknown);
pub trait IMalloc_Trait: crate::core::IUnknown_Trait {
    fn Alloc(&self, cb: PtrRepr) -> MutPtr<::core::ffi::c_void> {
        todo!("Alloc")
    }
    fn Realloc(
        &self,
        pv: ConstPtr<::core::ffi::c_void>,
        cb: PtrRepr,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!("Realloc")
    }
    fn Free(&self, pv: ConstPtr<::core::ffi::c_void>) {
        todo!("Free")
    }
    fn GetSize(&self, pv: ConstPtr<::core::ffi::c_void>) -> PtrRepr {
        todo!("GetSize")
    }
    fn DidAlloc(&self, pv: ConstPtr<::core::ffi::c_void>) -> i32 {
        todo!("DidAlloc")
    }
    fn HeapMinimize(&self) {
        todo!("HeapMinimize")
    }
}
impl ::core::clone::Clone for IMalloc {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IMalloc {}
impl ::core::cmp::PartialEq for IMalloc {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMalloc {}
impl ::core::fmt::Debug for IMalloc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMalloc").field(&self.0).finish()
    }
}
impl FromIntoMemory for IMalloc {
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
impl crate::core::ComInterface for IMalloc {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000002_0000_0000_c000_000000000046);
}
pub struct IMallocSpy(pub crate::core::IUnknown);
pub trait IMallocSpy_Trait: crate::core::IUnknown_Trait {
    fn PreAlloc(&self, cb_request: PtrRepr) -> PtrRepr {
        todo!("PreAlloc")
    }
    fn PostAlloc(&self, p_actual: ConstPtr<::core::ffi::c_void>) -> MutPtr<::core::ffi::c_void> {
        todo!("PostAlloc")
    }
    fn PreFree(
        &self,
        p_request: ConstPtr<::core::ffi::c_void>,
        f_spyed: super::super::Foundation::BOOL,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!("PreFree")
    }
    fn PostFree(&self, f_spyed: super::super::Foundation::BOOL) {
        todo!("PostFree")
    }
    fn PreRealloc(
        &self,
        p_request: ConstPtr<::core::ffi::c_void>,
        cb_request: PtrRepr,
        pp_new_request: MutPtr<ConstPtr<::core::ffi::c_void>>,
        f_spyed: super::super::Foundation::BOOL,
    ) -> PtrRepr {
        todo!("PreRealloc")
    }
    fn PostRealloc(
        &self,
        p_actual: ConstPtr<::core::ffi::c_void>,
        f_spyed: super::super::Foundation::BOOL,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!("PostRealloc")
    }
    fn PreGetSize(
        &self,
        p_request: ConstPtr<::core::ffi::c_void>,
        f_spyed: super::super::Foundation::BOOL,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!("PreGetSize")
    }
    fn PostGetSize(&self, cb_actual: PtrRepr, f_spyed: super::super::Foundation::BOOL) -> PtrRepr {
        todo!("PostGetSize")
    }
    fn PreDidAlloc(
        &self,
        p_request: ConstPtr<::core::ffi::c_void>,
        f_spyed: super::super::Foundation::BOOL,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!("PreDidAlloc")
    }
    fn PostDidAlloc(
        &self,
        p_request: ConstPtr<::core::ffi::c_void>,
        f_spyed: super::super::Foundation::BOOL,
        f_actual: i32,
    ) -> i32 {
        todo!("PostDidAlloc")
    }
    fn PreHeapMinimize(&self) {
        todo!("PreHeapMinimize")
    }
    fn PostHeapMinimize(&self) {
        todo!("PostHeapMinimize")
    }
}
impl ::core::clone::Clone for IMallocSpy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IMallocSpy {}
impl ::core::cmp::PartialEq for IMallocSpy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMallocSpy {}
impl ::core::fmt::Debug for IMallocSpy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMallocSpy").field(&self.0).finish()
    }
}
impl FromIntoMemory for IMallocSpy {
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
impl crate::core::ComInterface for IMallocSpy {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x0000001d_0000_0000_c000_000000000046);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IMoniker(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IMoniker_Trait: IPersistStream_Trait {
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn BindToObject(
        &self,
        pbc: IBindCtx,
        pmk_to_left: IMoniker,
        riid_result: ConstPtr<crate::core::GUID>,
        ppv_result: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("BindToObject")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn BindToStorage(
        &self,
        pbc: IBindCtx,
        pmk_to_left: IMoniker,
        riid: ConstPtr<crate::core::GUID>,
        ppv_obj: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("BindToStorage")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Reduce(
        &self,
        pbc: IBindCtx,
        dw_reduce_how_far: u32,
        ppmk_to_left: MutPtr<IMoniker>,
        ppmk_reduced: MutPtr<IMoniker>,
    ) -> crate::core::HRESULT {
        todo!("Reduce")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn ComposeWith(
        &self,
        pmk_right: IMoniker,
        f_only_if_not_generic: super::super::Foundation::BOOL,
        ppmk_composite: MutPtr<IMoniker>,
    ) -> crate::core::HRESULT {
        todo!("ComposeWith")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Enum(
        &self,
        f_forward: super::super::Foundation::BOOL,
        ppenum_moniker: MutPtr<IEnumMoniker>,
    ) -> crate::core::HRESULT {
        todo!("Enum")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn IsEqual(&self, pmk_other_moniker: IMoniker) -> crate::core::HRESULT {
        todo!("IsEqual")
    }
    fn Hash(&self, pdw_hash: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("Hash")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn IsRunning(
        &self,
        pbc: IBindCtx,
        pmk_to_left: IMoniker,
        pmk_newly_running: IMoniker,
    ) -> crate::core::HRESULT {
        todo!("IsRunning")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetTimeOfLastChange(
        &self,
        pbc: IBindCtx,
        pmk_to_left: IMoniker,
        p_file_time: MutPtr<super::super::Foundation::FILETIME>,
    ) -> crate::core::HRESULT {
        todo!("GetTimeOfLastChange")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Inverse(&self, ppmk: MutPtr<IMoniker>) -> crate::core::HRESULT {
        todo!("Inverse")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CommonPrefixWith(
        &self,
        pmk_other: IMoniker,
        ppmk_prefix: MutPtr<IMoniker>,
    ) -> crate::core::HRESULT {
        todo!("CommonPrefixWith")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn RelativePathTo(
        &self,
        pmk_other: IMoniker,
        ppmk_rel_path: MutPtr<IMoniker>,
    ) -> crate::core::HRESULT {
        todo!("RelativePathTo")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetDisplayName(
        &self,
        pbc: IBindCtx,
        pmk_to_left: IMoniker,
        ppsz_display_name: MutPtr<PWSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetDisplayName")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn ParseDisplayName(
        &self,
        pbc: IBindCtx,
        pmk_to_left: IMoniker,
        psz_display_name: PCWSTR,
        pch_eaten: MutPtr<u32>,
        ppmk_out: MutPtr<IMoniker>,
    ) -> crate::core::HRESULT {
        todo!("ParseDisplayName")
    }
    fn IsSystemMoniker(&self, pdw_mksys: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("IsSystemMoniker")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IMoniker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IMoniker {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IMoniker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IMoniker {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IMoniker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMoniker").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IMoniker {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IMoniker {
    type Super = IPersistStream;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x0000000f_0000_0000_c000_000000000046);
}
pub struct IMultiQI(pub crate::core::IUnknown);
pub trait IMultiQI_Trait: crate::core::IUnknown_Trait {
    fn QueryMultipleInterfaces(
        &self,
        c_mq_is: u32,
        p_mq_is: MutPtr<MULTI_QI>,
    ) -> crate::core::HRESULT {
        todo!("QueryMultipleInterfaces")
    }
}
impl ::core::clone::Clone for IMultiQI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IMultiQI {}
impl ::core::cmp::PartialEq for IMultiQI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMultiQI {}
impl ::core::fmt::Debug for IMultiQI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMultiQI").field(&self.0).finish()
    }
}
impl FromIntoMemory for IMultiQI {
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
impl crate::core::ComInterface for IMultiQI {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000020_0000_0000_c000_000000000046);
}
pub struct INTERFACEINFO {
    pub pUnk: crate::core::IUnknown,
    pub iid: crate::core::GUID,
    pub wMethod: u16,
}
impl ::core::marker::Copy for INTERFACEINFO {}
impl ::core::clone::Clone for INTERFACEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INTERFACEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERFACEINFO")
            .field("pUnk", &self.pUnk)
            .field("iid", &self.iid)
            .field("wMethod", &self.wMethod)
            .finish()
    }
}
impl ::core::cmp::PartialEq for INTERFACEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.pUnk == other.pUnk && self.iid == other.iid && self.wMethod == other.wMethod
    }
}
impl ::core::cmp::Eq for INTERFACEINFO {}
impl FromIntoMemory for INTERFACEINFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_pUnk = <crate::core::IUnknown as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_iid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[4..4 + 16]);
        let f_wMethod = <u16 as FromIntoMemory>::from_bytes(&from[20..20 + 2]);
        Self {
            pUnk: f_pUnk,
            iid: f_iid,
            wMethod: f_wMethod,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.pUnk, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.iid, &mut into[4..4 + 16]);
        FromIntoMemory::into_bytes(self.wMethod, &mut into[20..20 + 2]);
    }
    fn size() -> usize {
        24
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct INVOKEKIND(pub i32);
pub const INVOKE_FUNC: INVOKEKIND = INVOKEKIND(1i32);
pub const INVOKE_PROPERTYGET: INVOKEKIND = INVOKEKIND(2i32);
pub const INVOKE_PROPERTYPUT: INVOKEKIND = INVOKEKIND(4i32);
pub const INVOKE_PROPERTYPUTREF: INVOKEKIND = INVOKEKIND(8i32);
impl ::core::marker::Copy for INVOKEKIND {}
impl ::core::clone::Clone for INVOKEKIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INVOKEKIND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for INVOKEKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INVOKEKIND").field(&self.0).finish()
    }
}
impl FromIntoMemory for INVOKEKIND {
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
pub struct INoMarshal(pub crate::core::IUnknown);
pub trait INoMarshal_Trait: crate::core::IUnknown_Trait {}
impl ::core::clone::Clone for INoMarshal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for INoMarshal {}
impl ::core::cmp::PartialEq for INoMarshal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INoMarshal {}
impl ::core::fmt::Debug for INoMarshal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INoMarshal").field(&self.0).finish()
    }
}
impl FromIntoMemory for INoMarshal {
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
impl crate::core::ComInterface for INoMarshal {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xecc8691b_c1db_4dc0_855e_65f6c551af49);
}
pub struct IOplockStorage(pub crate::core::IUnknown);
pub trait IOplockStorage_Trait: crate::core::IUnknown_Trait {
    fn CreateStorageEx(
        &self,
        pwcs_name: PCWSTR,
        grf_mode: u32,
        stgfmt: u32,
        grf_attrs: u32,
        riid: ConstPtr<crate::core::GUID>,
        ppstg_open: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("CreateStorageEx")
    }
    fn OpenStorageEx(
        &self,
        pwcs_name: PCWSTR,
        grf_mode: u32,
        stgfmt: u32,
        grf_attrs: u32,
        riid: ConstPtr<crate::core::GUID>,
        ppstg_open: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("OpenStorageEx")
    }
}
impl ::core::clone::Clone for IOplockStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IOplockStorage {}
impl ::core::cmp::PartialEq for IOplockStorage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOplockStorage {}
impl ::core::fmt::Debug for IOplockStorage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOplockStorage").field(&self.0).finish()
    }
}
impl FromIntoMemory for IOplockStorage {
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
impl crate::core::ComInterface for IOplockStorage {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x8d19c834_8879_11d1_83e9_00c04fc2c6d4);
}
pub struct IPSFactoryBuffer(pub crate::core::IUnknown);
pub trait IPSFactoryBuffer_Trait: crate::core::IUnknown_Trait {
    fn CreateProxy(
        &self,
        p_unk_outer: crate::core::IUnknown,
        riid: ConstPtr<crate::core::GUID>,
        pp_proxy: MutPtr<IRpcProxyBuffer>,
        ppv: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("CreateProxy")
    }
    fn CreateStub(
        &self,
        riid: ConstPtr<crate::core::GUID>,
        p_unk_server: crate::core::IUnknown,
        pp_stub: MutPtr<IRpcStubBuffer>,
    ) -> crate::core::HRESULT {
        todo!("CreateStub")
    }
}
impl ::core::clone::Clone for IPSFactoryBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IPSFactoryBuffer {}
impl ::core::cmp::PartialEq for IPSFactoryBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPSFactoryBuffer {}
impl ::core::fmt::Debug for IPSFactoryBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSFactoryBuffer").field(&self.0).finish()
    }
}
impl FromIntoMemory for IPSFactoryBuffer {
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
impl crate::core::ComInterface for IPSFactoryBuffer {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xd5f569d0_593b_101a_b569_08002b2dbf7a);
}
pub struct IPersist(pub crate::core::IUnknown);
pub trait IPersist_Trait: crate::core::IUnknown_Trait {
    fn GetClassID(&self, p_class_id: MutPtr<crate::core::GUID>) -> crate::core::HRESULT {
        todo!("GetClassID")
    }
}
impl ::core::clone::Clone for IPersist {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IPersist {}
impl ::core::cmp::PartialEq for IPersist {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersist {}
impl ::core::fmt::Debug for IPersist {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersist").field(&self.0).finish()
    }
}
impl FromIntoMemory for IPersist {
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
impl crate::core::ComInterface for IPersist {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x0000010c_0000_0000_c000_000000000046);
}
pub struct IPersistFile(pub crate::core::IUnknown);
pub trait IPersistFile_Trait: IPersist_Trait {
    fn IsDirty(&self) -> crate::core::HRESULT {
        todo!("IsDirty")
    }
    fn Load(&self, psz_file_name: PCWSTR, dw_mode: u32) -> crate::core::HRESULT {
        todo!("Load")
    }
    fn Save(
        &self,
        psz_file_name: PCWSTR,
        f_remember: super::super::Foundation::BOOL,
    ) -> crate::core::HRESULT {
        todo!("Save")
    }
    fn SaveCompleted(&self, psz_file_name: PCWSTR) -> crate::core::HRESULT {
        todo!("SaveCompleted")
    }
    fn GetCurFile(&self, ppsz_file_name: MutPtr<PWSTR>) -> crate::core::HRESULT {
        todo!("GetCurFile")
    }
}
impl ::core::clone::Clone for IPersistFile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IPersistFile {}
impl ::core::cmp::PartialEq for IPersistFile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersistFile {}
impl ::core::fmt::Debug for IPersistFile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistFile").field(&self.0).finish()
    }
}
impl FromIntoMemory for IPersistFile {
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
impl crate::core::ComInterface for IPersistFile {
    type Super = IPersist;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x0000010b_0000_0000_c000_000000000046);
}
pub struct IPersistMemory(pub crate::core::IUnknown);
pub trait IPersistMemory_Trait: IPersist_Trait {
    fn IsDirty(&self) -> crate::core::HRESULT {
        todo!("IsDirty")
    }
    fn Load(&self, p_mem: ConstPtr<::core::ffi::c_void>, cb_size: u32) -> crate::core::HRESULT {
        todo!("Load")
    }
    fn Save(
        &self,
        p_mem: MutPtr<::core::ffi::c_void>,
        f_clear_dirty: super::super::Foundation::BOOL,
        cb_size: u32,
    ) -> crate::core::HRESULT {
        todo!("Save")
    }
    fn GetSizeMax(&self, p_cb_size: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetSizeMax")
    }
    fn InitNew(&self) -> crate::core::HRESULT {
        todo!("InitNew")
    }
}
impl ::core::clone::Clone for IPersistMemory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IPersistMemory {}
impl ::core::cmp::PartialEq for IPersistMemory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersistMemory {}
impl ::core::fmt::Debug for IPersistMemory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistMemory").field(&self.0).finish()
    }
}
impl FromIntoMemory for IPersistMemory {
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
impl crate::core::ComInterface for IPersistMemory {
    type Super = IPersist;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xbd1ae5e0_a6ae_11ce_bd37_504200c10000);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IPersistStream(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IPersistStream_Trait: IPersist_Trait {
    fn IsDirty(&self) -> crate::core::HRESULT {
        todo!("IsDirty")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Load(&self, p_stm: IStream) -> crate::core::HRESULT {
        todo!("Load")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Save(
        &self,
        p_stm: IStream,
        f_clear_dirty: super::super::Foundation::BOOL,
    ) -> crate::core::HRESULT {
        todo!("Save")
    }
    fn GetSizeMax(&self, pcb_size: MutPtr<u64>) -> crate::core::HRESULT {
        todo!("GetSizeMax")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IPersistStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IPersistStream {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IPersistStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IPersistStream {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IPersistStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistStream").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IPersistStream {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IPersistStream {
    type Super = IPersist;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000109_0000_0000_c000_000000000046);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IPersistStreamInit(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IPersistStreamInit_Trait: IPersist_Trait {
    fn IsDirty(&self) -> crate::core::HRESULT {
        todo!("IsDirty")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Load(&self, p_stm: IStream) -> crate::core::HRESULT {
        todo!("Load")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Save(
        &self,
        p_stm: IStream,
        f_clear_dirty: super::super::Foundation::BOOL,
    ) -> crate::core::HRESULT {
        todo!("Save")
    }
    fn GetSizeMax(&self, p_cb_size: MutPtr<u64>) -> crate::core::HRESULT {
        todo!("GetSizeMax")
    }
    fn InitNew(&self) -> crate::core::HRESULT {
        todo!("InitNew")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IPersistStreamInit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IPersistStreamInit {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IPersistStreamInit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IPersistStreamInit {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IPersistStreamInit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistStreamInit").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IPersistStreamInit {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IPersistStreamInit {
    type Super = IPersist;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x7fd52380_4e07_101b_ae2d_08002b2ec713);
}
pub struct IPipeByte(pub crate::core::IUnknown);
pub trait IPipeByte_Trait: crate::core::IUnknown_Trait {
    fn Pull(
        &self,
        buf: MutPtr<u8>,
        c_request: u32,
        pc_returned: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("Pull")
    }
    fn Push(&self, buf: ConstPtr<u8>, c_sent: u32) -> crate::core::HRESULT {
        todo!("Push")
    }
}
impl ::core::clone::Clone for IPipeByte {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IPipeByte {}
impl ::core::cmp::PartialEq for IPipeByte {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPipeByte {}
impl ::core::fmt::Debug for IPipeByte {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPipeByte").field(&self.0).finish()
    }
}
impl FromIntoMemory for IPipeByte {
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
impl crate::core::ComInterface for IPipeByte {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xdb2f3aca_2f86_11d1_8e04_00c04fb9989a);
}
pub struct IPipeDouble(pub crate::core::IUnknown);
pub trait IPipeDouble_Trait: crate::core::IUnknown_Trait {
    fn Pull(
        &self,
        buf: MutPtr<f64>,
        c_request: u32,
        pc_returned: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("Pull")
    }
    fn Push(&self, buf: ConstPtr<f64>, c_sent: u32) -> crate::core::HRESULT {
        todo!("Push")
    }
}
impl ::core::clone::Clone for IPipeDouble {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IPipeDouble {}
impl ::core::cmp::PartialEq for IPipeDouble {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPipeDouble {}
impl ::core::fmt::Debug for IPipeDouble {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPipeDouble").field(&self.0).finish()
    }
}
impl FromIntoMemory for IPipeDouble {
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
impl crate::core::ComInterface for IPipeDouble {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xdb2f3ace_2f86_11d1_8e04_00c04fb9989a);
}
pub struct IPipeLong(pub crate::core::IUnknown);
pub trait IPipeLong_Trait: crate::core::IUnknown_Trait {
    fn Pull(
        &self,
        buf: MutPtr<i32>,
        c_request: u32,
        pc_returned: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("Pull")
    }
    fn Push(&self, buf: ConstPtr<i32>, c_sent: u32) -> crate::core::HRESULT {
        todo!("Push")
    }
}
impl ::core::clone::Clone for IPipeLong {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IPipeLong {}
impl ::core::cmp::PartialEq for IPipeLong {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPipeLong {}
impl ::core::fmt::Debug for IPipeLong {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPipeLong").field(&self.0).finish()
    }
}
impl FromIntoMemory for IPipeLong {
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
impl crate::core::ComInterface for IPipeLong {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xdb2f3acc_2f86_11d1_8e04_00c04fb9989a);
}
pub struct IProcessInitControl(pub crate::core::IUnknown);
pub trait IProcessInitControl_Trait: crate::core::IUnknown_Trait {
    fn ResetInitializerTimeout(&self, dw_seconds_remaining: u32) -> crate::core::HRESULT {
        todo!("ResetInitializerTimeout")
    }
}
impl ::core::clone::Clone for IProcessInitControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IProcessInitControl {}
impl ::core::cmp::PartialEq for IProcessInitControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProcessInitControl {}
impl ::core::fmt::Debug for IProcessInitControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProcessInitControl").field(&self.0).finish()
    }
}
impl FromIntoMemory for IProcessInitControl {
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
impl crate::core::ComInterface for IProcessInitControl {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x72380d55_8d2b_43a3_8513_2b6ef31434e9);
}
pub struct IProcessLock(pub crate::core::IUnknown);
pub trait IProcessLock_Trait: crate::core::IUnknown_Trait {
    fn AddRefOnProcess(&self) -> u32 {
        todo!("AddRefOnProcess")
    }
    fn ReleaseRefOnProcess(&self) -> u32 {
        todo!("ReleaseRefOnProcess")
    }
}
impl ::core::clone::Clone for IProcessLock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IProcessLock {}
impl ::core::cmp::PartialEq for IProcessLock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProcessLock {}
impl ::core::fmt::Debug for IProcessLock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProcessLock").field(&self.0).finish()
    }
}
impl FromIntoMemory for IProcessLock {
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
impl crate::core::ComInterface for IProcessLock {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x000001d5_0000_0000_c000_000000000046);
}
pub struct IProgressNotify(pub crate::core::IUnknown);
pub trait IProgressNotify_Trait: crate::core::IUnknown_Trait {
    fn OnProgress(
        &self,
        dw_progress_current: u32,
        dw_progress_maximum: u32,
        f_accurate: super::super::Foundation::BOOL,
        f_owner: super::super::Foundation::BOOL,
    ) -> crate::core::HRESULT {
        todo!("OnProgress")
    }
}
impl ::core::clone::Clone for IProgressNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IProgressNotify {}
impl ::core::cmp::PartialEq for IProgressNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProgressNotify {}
impl ::core::fmt::Debug for IProgressNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProgressNotify").field(&self.0).finish()
    }
}
impl FromIntoMemory for IProgressNotify {
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
impl crate::core::ComInterface for IProgressNotify {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xa9d758a0_4617_11cf_95fc_00aa00680db4);
}
pub struct IROTData(pub crate::core::IUnknown);
pub trait IROTData_Trait: crate::core::IUnknown_Trait {
    fn GetComparisonData(
        &self,
        pb_data: MutPtr<u8>,
        cb_max: u32,
        pcb_data: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("GetComparisonData")
    }
}
impl ::core::clone::Clone for IROTData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IROTData {}
impl ::core::cmp::PartialEq for IROTData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IROTData {}
impl ::core::fmt::Debug for IROTData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IROTData").field(&self.0).finish()
    }
}
impl FromIntoMemory for IROTData {
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
impl crate::core::ComInterface for IROTData {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xf29f6bc0_5021_11ce_aa15_00006901293f);
}
pub struct IReleaseMarshalBuffers(pub crate::core::IUnknown);
pub trait IReleaseMarshalBuffers_Trait: crate::core::IUnknown_Trait {
    fn ReleaseMarshalBuffer(
        &self,
        p_msg: MutPtr<RPCOLEMESSAGE>,
        dw_flags: u32,
        p_chnl: crate::core::IUnknown,
    ) -> crate::core::HRESULT {
        todo!("ReleaseMarshalBuffer")
    }
}
impl ::core::clone::Clone for IReleaseMarshalBuffers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IReleaseMarshalBuffers {}
impl ::core::cmp::PartialEq for IReleaseMarshalBuffers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReleaseMarshalBuffers {}
impl ::core::fmt::Debug for IReleaseMarshalBuffers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReleaseMarshalBuffers")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IReleaseMarshalBuffers {
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
impl crate::core::ComInterface for IReleaseMarshalBuffers {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xeb0cb9e8_7996_11d2_872e_0000f8080859);
}
pub struct IRpcChannelBuffer(pub crate::core::IUnknown);
pub trait IRpcChannelBuffer_Trait: crate::core::IUnknown_Trait {
    fn GetBuffer(
        &self,
        p_message: MutPtr<RPCOLEMESSAGE>,
        riid: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("GetBuffer")
    }
    fn SendReceive(
        &self,
        p_message: MutPtr<RPCOLEMESSAGE>,
        p_status: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("SendReceive")
    }
    fn FreeBuffer(&self, p_message: MutPtr<RPCOLEMESSAGE>) -> crate::core::HRESULT {
        todo!("FreeBuffer")
    }
    fn GetDestCtx(
        &self,
        pdw_dest_context: MutPtr<u32>,
        ppv_dest_context: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("GetDestCtx")
    }
    fn IsConnected(&self) -> crate::core::HRESULT {
        todo!("IsConnected")
    }
}
impl ::core::clone::Clone for IRpcChannelBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IRpcChannelBuffer {}
impl ::core::cmp::PartialEq for IRpcChannelBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcChannelBuffer {}
impl ::core::fmt::Debug for IRpcChannelBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcChannelBuffer").field(&self.0).finish()
    }
}
impl FromIntoMemory for IRpcChannelBuffer {
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
impl crate::core::ComInterface for IRpcChannelBuffer {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xd5f56b60_593b_101a_b569_08002b2dbf7a);
}
pub struct IRpcChannelBuffer2(pub crate::core::IUnknown);
pub trait IRpcChannelBuffer2_Trait: IRpcChannelBuffer_Trait {
    fn GetProtocolVersion(&self, pdw_version: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetProtocolVersion")
    }
}
impl ::core::clone::Clone for IRpcChannelBuffer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IRpcChannelBuffer2 {}
impl ::core::cmp::PartialEq for IRpcChannelBuffer2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcChannelBuffer2 {}
impl ::core::fmt::Debug for IRpcChannelBuffer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcChannelBuffer2").field(&self.0).finish()
    }
}
impl FromIntoMemory for IRpcChannelBuffer2 {
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
impl crate::core::ComInterface for IRpcChannelBuffer2 {
    type Super = IRpcChannelBuffer;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x594f31d0_7f19_11d0_b194_00a0c90dc8bf);
}
pub struct IRpcChannelBuffer3(pub crate::core::IUnknown);
pub trait IRpcChannelBuffer3_Trait: IRpcChannelBuffer2_Trait {
    fn Send(&self, p_msg: MutPtr<RPCOLEMESSAGE>, pul_status: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("Send")
    }
    fn Receive(
        &self,
        p_msg: MutPtr<RPCOLEMESSAGE>,
        ul_size: u32,
        pul_status: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("Receive")
    }
    fn Cancel(&self, p_msg: MutPtr<RPCOLEMESSAGE>) -> crate::core::HRESULT {
        todo!("Cancel")
    }
    fn GetCallContext(
        &self,
        p_msg: ConstPtr<RPCOLEMESSAGE>,
        riid: ConstPtr<crate::core::GUID>,
        p_interface: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("GetCallContext")
    }
    fn GetDestCtxEx(
        &self,
        p_msg: ConstPtr<RPCOLEMESSAGE>,
        pdw_dest_context: MutPtr<u32>,
        ppv_dest_context: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("GetDestCtxEx")
    }
    fn GetState(
        &self,
        p_msg: ConstPtr<RPCOLEMESSAGE>,
        p_state: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("GetState")
    }
    fn RegisterAsync(
        &self,
        p_msg: MutPtr<RPCOLEMESSAGE>,
        p_async_mgr: IAsyncManager,
    ) -> crate::core::HRESULT {
        todo!("RegisterAsync")
    }
}
impl ::core::clone::Clone for IRpcChannelBuffer3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IRpcChannelBuffer3 {}
impl ::core::cmp::PartialEq for IRpcChannelBuffer3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcChannelBuffer3 {}
impl ::core::fmt::Debug for IRpcChannelBuffer3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcChannelBuffer3").field(&self.0).finish()
    }
}
impl FromIntoMemory for IRpcChannelBuffer3 {
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
impl crate::core::ComInterface for IRpcChannelBuffer3 {
    type Super = IRpcChannelBuffer2;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x25b15600_0115_11d0_bf0d_00aa00b8dfd2);
}
pub struct IRpcHelper(pub crate::core::IUnknown);
pub trait IRpcHelper_Trait: crate::core::IUnknown_Trait {
    fn GetDCOMProtocolVersion(&self, p_com_version: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetDCOMProtocolVersion")
    }
    fn GetIIDFromOBJREF(
        &self,
        p_obj_ref: ConstPtr<::core::ffi::c_void>,
        piid: MutPtr<ConstPtr<crate::core::GUID>>,
    ) -> crate::core::HRESULT {
        todo!("GetIIDFromOBJREF")
    }
}
impl ::core::clone::Clone for IRpcHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IRpcHelper {}
impl ::core::cmp::PartialEq for IRpcHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcHelper {}
impl ::core::fmt::Debug for IRpcHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcHelper").field(&self.0).finish()
    }
}
impl FromIntoMemory for IRpcHelper {
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
impl crate::core::ComInterface for IRpcHelper {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000149_0000_0000_c000_000000000046);
}
pub struct IRpcOptions(pub crate::core::IUnknown);
pub trait IRpcOptions_Trait: crate::core::IUnknown_Trait {
    fn Set(
        &self,
        p_prx: crate::core::IUnknown,
        dw_property: RPCOPT_PROPERTIES,
        dw_value: PtrRepr,
    ) -> crate::core::HRESULT {
        todo!("Set")
    }
    fn Query(
        &self,
        p_prx: crate::core::IUnknown,
        dw_property: RPCOPT_PROPERTIES,
        pdw_value: MutPtr<PtrRepr>,
    ) -> crate::core::HRESULT {
        todo!("Query")
    }
}
impl ::core::clone::Clone for IRpcOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IRpcOptions {}
impl ::core::cmp::PartialEq for IRpcOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcOptions {}
impl ::core::fmt::Debug for IRpcOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcOptions").field(&self.0).finish()
    }
}
impl FromIntoMemory for IRpcOptions {
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
impl crate::core::ComInterface for IRpcOptions {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000144_0000_0000_c000_000000000046);
}
pub struct IRpcProxyBuffer(pub crate::core::IUnknown);
pub trait IRpcProxyBuffer_Trait: crate::core::IUnknown_Trait {
    fn Connect(&self, p_rpc_channel_buffer: IRpcChannelBuffer) -> crate::core::HRESULT {
        todo!("Connect")
    }
    fn Disconnect(&self) {
        todo!("Disconnect")
    }
}
impl ::core::clone::Clone for IRpcProxyBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IRpcProxyBuffer {}
impl ::core::cmp::PartialEq for IRpcProxyBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcProxyBuffer {}
impl ::core::fmt::Debug for IRpcProxyBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcProxyBuffer").field(&self.0).finish()
    }
}
impl FromIntoMemory for IRpcProxyBuffer {
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
impl crate::core::ComInterface for IRpcProxyBuffer {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xd5f56a34_593b_101a_b569_08002b2dbf7a);
}
pub struct IRpcStubBuffer(pub crate::core::IUnknown);
pub trait IRpcStubBuffer_Trait: crate::core::IUnknown_Trait {
    fn Connect(&self, p_unk_server: crate::core::IUnknown) -> crate::core::HRESULT {
        todo!("Connect")
    }
    fn Disconnect(&self) {
        todo!("Disconnect")
    }
    fn Invoke(
        &self,
        prpcmsg: MutPtr<RPCOLEMESSAGE>,
        p_rpc_channel_buffer: IRpcChannelBuffer,
    ) -> crate::core::HRESULT {
        todo!("Invoke")
    }
    fn IsIIDSupported(&self, riid: ConstPtr<crate::core::GUID>) -> IRpcStubBuffer {
        todo!("IsIIDSupported")
    }
    fn CountRefs(&self) -> u32 {
        todo!("CountRefs")
    }
    fn DebugServerQueryInterface(
        &self,
        ppv: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("DebugServerQueryInterface")
    }
    fn DebugServerRelease(&self, pv: ConstPtr<::core::ffi::c_void>) {
        todo!("DebugServerRelease")
    }
}
impl ::core::clone::Clone for IRpcStubBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IRpcStubBuffer {}
impl ::core::cmp::PartialEq for IRpcStubBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcStubBuffer {}
impl ::core::fmt::Debug for IRpcStubBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcStubBuffer").field(&self.0).finish()
    }
}
impl FromIntoMemory for IRpcStubBuffer {
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
impl crate::core::ComInterface for IRpcStubBuffer {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xd5f56afc_593b_101a_b569_08002b2dbf7a);
}
pub struct IRpcSyntaxNegotiate(pub crate::core::IUnknown);
pub trait IRpcSyntaxNegotiate_Trait: crate::core::IUnknown_Trait {
    fn NegotiateSyntax(&self, p_msg: MutPtr<RPCOLEMESSAGE>) -> crate::core::HRESULT {
        todo!("NegotiateSyntax")
    }
}
impl ::core::clone::Clone for IRpcSyntaxNegotiate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IRpcSyntaxNegotiate {}
impl ::core::cmp::PartialEq for IRpcSyntaxNegotiate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcSyntaxNegotiate {}
impl ::core::fmt::Debug for IRpcSyntaxNegotiate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcSyntaxNegotiate").field(&self.0).finish()
    }
}
impl FromIntoMemory for IRpcSyntaxNegotiate {
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
impl crate::core::ComInterface for IRpcSyntaxNegotiate {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x58a08519_24c8_4935_b482_3fd823333a4f);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IRunnableObject(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IRunnableObject_Trait: crate::core::IUnknown_Trait {
    fn GetRunningClass(&self, lp_clsid: MutPtr<crate::core::GUID>) -> crate::core::HRESULT {
        todo!("GetRunningClass")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Run(&self, pbc: IBindCtx) -> crate::core::HRESULT {
        todo!("Run")
    }
    fn IsRunning(&self) -> super::super::Foundation::BOOL {
        todo!("IsRunning")
    }
    fn LockRunning(
        &self,
        f_lock: super::super::Foundation::BOOL,
        f_last_unlock_closes: super::super::Foundation::BOOL,
    ) -> crate::core::HRESULT {
        todo!("LockRunning")
    }
    fn SetContainedObject(
        &self,
        f_contained: super::super::Foundation::BOOL,
    ) -> crate::core::HRESULT {
        todo!("SetContainedObject")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IRunnableObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IRunnableObject {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IRunnableObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IRunnableObject {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IRunnableObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRunnableObject").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IRunnableObject {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IRunnableObject {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000126_0000_0000_c000_000000000046);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IRunningObjectTable(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IRunningObjectTable_Trait: crate::core::IUnknown_Trait {
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Register(
        &self,
        grf_flags: u32,
        punk_object: crate::core::IUnknown,
        pmk_object_name: IMoniker,
        pdw_register: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("Register")
    }
    fn Revoke(&self, dw_register: u32) -> crate::core::HRESULT {
        todo!("Revoke")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn IsRunning(&self, pmk_object_name: IMoniker) -> crate::core::HRESULT {
        todo!("IsRunning")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetObject(
        &self,
        pmk_object_name: IMoniker,
        ppunk_object: MutPtr<crate::core::IUnknown>,
    ) -> crate::core::HRESULT {
        todo!("GetObject")
    }
    fn NoteChangeTime(
        &self,
        dw_register: u32,
        pfiletime: ConstPtr<super::super::Foundation::FILETIME>,
    ) -> crate::core::HRESULT {
        todo!("NoteChangeTime")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetTimeOfLastChange(
        &self,
        pmk_object_name: IMoniker,
        pfiletime: MutPtr<super::super::Foundation::FILETIME>,
    ) -> crate::core::HRESULT {
        todo!("GetTimeOfLastChange")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn EnumRunning(&self, ppenum_moniker: MutPtr<IEnumMoniker>) -> crate::core::HRESULT {
        todo!("EnumRunning")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IRunningObjectTable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IRunningObjectTable {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IRunningObjectTable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IRunningObjectTable {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IRunningObjectTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRunningObjectTable").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IRunningObjectTable {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IRunningObjectTable {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000010_0000_0000_c000_000000000046);
}
pub struct ISequentialStream(pub crate::core::IUnknown);
pub trait ISequentialStream_Trait: crate::core::IUnknown_Trait {
    fn Read(
        &self,
        pv: MutPtr<::core::ffi::c_void>,
        cb: u32,
        pcb_read: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("Read")
    }
    fn Write(
        &self,
        pv: ConstPtr<::core::ffi::c_void>,
        cb: u32,
        pcb_written: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("Write")
    }
}
impl ::core::clone::Clone for ISequentialStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ISequentialStream {}
impl ::core::cmp::PartialEq for ISequentialStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISequentialStream {}
impl ::core::fmt::Debug for ISequentialStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISequentialStream").field(&self.0).finish()
    }
}
impl FromIntoMemory for ISequentialStream {
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
impl crate::core::ComInterface for ISequentialStream {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x0c733a30_2a1c_11ce_ade5_00aa0044773d);
}
pub struct IServerSecurity(pub crate::core::IUnknown);
pub trait IServerSecurity_Trait: crate::core::IUnknown_Trait {
    fn QueryBlanket(
        &self,
        p_authn_svc: MutPtr<u32>,
        p_authz_svc: MutPtr<u32>,
        p_server_princ_name: MutPtr<ConstPtr<u16>>,
        p_authn_level: MutPtr<u32>,
        p_imp_level: MutPtr<u32>,
        p_privs: MutPtr<ConstPtr<::core::ffi::c_void>>,
        p_capabilities: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("QueryBlanket")
    }
    fn ImpersonateClient(&self) -> crate::core::HRESULT {
        todo!("ImpersonateClient")
    }
    fn RevertToSelf(&self) -> crate::core::HRESULT {
        todo!("RevertToSelf")
    }
    fn IsImpersonating(&self) -> super::super::Foundation::BOOL {
        todo!("IsImpersonating")
    }
}
impl ::core::clone::Clone for IServerSecurity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IServerSecurity {}
impl ::core::cmp::PartialEq for IServerSecurity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServerSecurity {}
impl ::core::fmt::Debug for IServerSecurity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServerSecurity").field(&self.0).finish()
    }
}
impl FromIntoMemory for IServerSecurity {
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
impl crate::core::ComInterface for IServerSecurity {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x0000013e_0000_0000_c000_000000000046);
}
pub struct IServiceProvider(pub crate::core::IUnknown);
pub trait IServiceProvider_Trait: crate::core::IUnknown_Trait {
    fn QueryService(
        &self,
        guid_service: ConstPtr<crate::core::GUID>,
        riid: ConstPtr<crate::core::GUID>,
        ppv_object: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("QueryService")
    }
}
impl ::core::clone::Clone for IServiceProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IServiceProvider {}
impl ::core::cmp::PartialEq for IServiceProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServiceProvider {}
impl ::core::fmt::Debug for IServiceProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServiceProvider").field(&self.0).finish()
    }
}
impl FromIntoMemory for IServiceProvider {
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
impl crate::core::ComInterface for IServiceProvider {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x6d5140c1_7436_11ce_8034_00aa006009fa);
}
pub struct IStdMarshalInfo(pub crate::core::IUnknown);
pub trait IStdMarshalInfo_Trait: crate::core::IUnknown_Trait {
    fn GetClassForHandler(
        &self,
        dw_dest_context: u32,
        pv_dest_context: MutPtr<::core::ffi::c_void>,
        p_clsid: MutPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("GetClassForHandler")
    }
}
impl ::core::clone::Clone for IStdMarshalInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IStdMarshalInfo {}
impl ::core::cmp::PartialEq for IStdMarshalInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStdMarshalInfo {}
impl ::core::fmt::Debug for IStdMarshalInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStdMarshalInfo").field(&self.0).finish()
    }
}
impl FromIntoMemory for IStdMarshalInfo {
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
impl crate::core::ComInterface for IStdMarshalInfo {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000018_0000_0000_c000_000000000046);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IStream(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IStream_Trait: ISequentialStream_Trait {
    fn Seek(
        &self,
        dlib_move: i64,
        dw_origin: STREAM_SEEK,
        plib_new_position: MutPtr<u64>,
    ) -> crate::core::HRESULT {
        todo!("Seek")
    }
    fn SetSize(&self, lib_new_size: u64) -> crate::core::HRESULT {
        todo!("SetSize")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CopyTo(
        &self,
        pstm: IStream,
        cb: u64,
        pcb_read: MutPtr<u64>,
        pcb_written: MutPtr<u64>,
    ) -> crate::core::HRESULT {
        todo!("CopyTo")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Commit(&self, grf_commit_flags: StructuredStorage::STGC) -> crate::core::HRESULT {
        todo!("Commit")
    }
    fn Revert(&self) -> crate::core::HRESULT {
        todo!("Revert")
    }
    fn LockRegion(&self, lib_offset: u64, cb: u64, dw_lock_type: u32) -> crate::core::HRESULT {
        todo!("LockRegion")
    }
    fn UnlockRegion(&self, lib_offset: u64, cb: u64, dw_lock_type: u32) -> crate::core::HRESULT {
        todo!("UnlockRegion")
    }
    fn Stat(&self, pstatstg: MutPtr<STATSTG>, grf_stat_flag: u32) -> crate::core::HRESULT {
        todo!("Stat")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Clone(&self, ppstm: MutPtr<IStream>) -> crate::core::HRESULT {
        todo!("Clone")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IStream {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IStream {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStream").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IStream {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IStream {
    type Super = ISequentialStream;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x0000000c_0000_0000_c000_000000000046);
}
pub struct ISupportErrorInfo(pub crate::core::IUnknown);
pub trait ISupportErrorInfo_Trait: crate::core::IUnknown_Trait {
    fn InterfaceSupportsErrorInfo(
        &self,
        riid: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("InterfaceSupportsErrorInfo")
    }
}
impl ::core::clone::Clone for ISupportErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ISupportErrorInfo {}
impl ::core::cmp::PartialEq for ISupportErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISupportErrorInfo {}
impl ::core::fmt::Debug for ISupportErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISupportErrorInfo").field(&self.0).finish()
    }
}
impl FromIntoMemory for ISupportErrorInfo {
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
impl crate::core::ComInterface for ISupportErrorInfo {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xdf0b3d60_548f_101b_8e65_08002b2bd119);
}
pub struct ISurrogate(pub crate::core::IUnknown);
pub trait ISurrogate_Trait: crate::core::IUnknown_Trait {
    fn LoadDllServer(&self, clsid: ConstPtr<crate::core::GUID>) -> crate::core::HRESULT {
        todo!("LoadDllServer")
    }
    fn FreeSurrogate(&self) -> crate::core::HRESULT {
        todo!("FreeSurrogate")
    }
}
impl ::core::clone::Clone for ISurrogate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ISurrogate {}
impl ::core::cmp::PartialEq for ISurrogate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISurrogate {}
impl ::core::fmt::Debug for ISurrogate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISurrogate").field(&self.0).finish()
    }
}
impl FromIntoMemory for ISurrogate {
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
impl crate::core::ComInterface for ISurrogate {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000022_0000_0000_c000_000000000046);
}
pub struct ISurrogateService(pub crate::core::IUnknown);
pub trait ISurrogateService_Trait: crate::core::IUnknown_Trait {
    fn Init(
        &self,
        rguid_process_id: ConstPtr<crate::core::GUID>,
        p_process_lock: IProcessLock,
        pf_application_aware: MutPtr<super::super::Foundation::BOOL>,
    ) -> crate::core::HRESULT {
        todo!("Init")
    }
    fn ApplicationLaunch(
        &self,
        rguid_appl_id: ConstPtr<crate::core::GUID>,
        app_type: ApplicationType,
    ) -> crate::core::HRESULT {
        todo!("ApplicationLaunch")
    }
    fn ApplicationFree(&self, rguid_appl_id: ConstPtr<crate::core::GUID>) -> crate::core::HRESULT {
        todo!("ApplicationFree")
    }
    fn CatalogRefresh(&self, ul_reserved: u32) -> crate::core::HRESULT {
        todo!("CatalogRefresh")
    }
    fn ProcessShutdown(&self, shutdown_type: ShutdownType) -> crate::core::HRESULT {
        todo!("ProcessShutdown")
    }
}
impl ::core::clone::Clone for ISurrogateService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ISurrogateService {}
impl ::core::cmp::PartialEq for ISurrogateService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISurrogateService {}
impl ::core::fmt::Debug for ISurrogateService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISurrogateService").field(&self.0).finish()
    }
}
impl FromIntoMemory for ISurrogateService {
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
impl crate::core::ComInterface for ISurrogateService {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x000001d4_0000_0000_c000_000000000046);
}
pub struct ISynchronize(pub crate::core::IUnknown);
pub trait ISynchronize_Trait: crate::core::IUnknown_Trait {
    fn Wait(&self, dw_flags: u32, dw_milliseconds: u32) -> crate::core::HRESULT {
        todo!("Wait")
    }
    fn Signal(&self) -> crate::core::HRESULT {
        todo!("Signal")
    }
    fn Reset(&self) -> crate::core::HRESULT {
        todo!("Reset")
    }
}
impl ::core::clone::Clone for ISynchronize {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ISynchronize {}
impl ::core::cmp::PartialEq for ISynchronize {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISynchronize {}
impl ::core::fmt::Debug for ISynchronize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISynchronize").field(&self.0).finish()
    }
}
impl FromIntoMemory for ISynchronize {
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
impl crate::core::ComInterface for ISynchronize {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000030_0000_0000_c000_000000000046);
}
pub struct ISynchronizeContainer(pub crate::core::IUnknown);
pub trait ISynchronizeContainer_Trait: crate::core::IUnknown_Trait {
    fn AddSynchronize(&self, p_sync: ISynchronize) -> crate::core::HRESULT {
        todo!("AddSynchronize")
    }
    fn WaitMultiple(
        &self,
        dw_flags: u32,
        dw_time_out: u32,
        pp_sync: MutPtr<ISynchronize>,
    ) -> crate::core::HRESULT {
        todo!("WaitMultiple")
    }
}
impl ::core::clone::Clone for ISynchronizeContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ISynchronizeContainer {}
impl ::core::cmp::PartialEq for ISynchronizeContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISynchronizeContainer {}
impl ::core::fmt::Debug for ISynchronizeContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISynchronizeContainer")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ISynchronizeContainer {
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
impl crate::core::ComInterface for ISynchronizeContainer {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000033_0000_0000_c000_000000000046);
}
pub struct ISynchronizeEvent(pub crate::core::IUnknown);
pub trait ISynchronizeEvent_Trait: ISynchronizeHandle_Trait {
    fn SetEventHandle(
        &self,
        ph: ConstPtr<super::super::Foundation::HANDLE>,
    ) -> crate::core::HRESULT {
        todo!("SetEventHandle")
    }
}
impl ::core::clone::Clone for ISynchronizeEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ISynchronizeEvent {}
impl ::core::cmp::PartialEq for ISynchronizeEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISynchronizeEvent {}
impl ::core::fmt::Debug for ISynchronizeEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISynchronizeEvent").field(&self.0).finish()
    }
}
impl FromIntoMemory for ISynchronizeEvent {
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
impl crate::core::ComInterface for ISynchronizeEvent {
    type Super = ISynchronizeHandle;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000032_0000_0000_c000_000000000046);
}
pub struct ISynchronizeHandle(pub crate::core::IUnknown);
pub trait ISynchronizeHandle_Trait: crate::core::IUnknown_Trait {
    fn GetHandle(&self, ph: MutPtr<super::super::Foundation::HANDLE>) -> crate::core::HRESULT {
        todo!("GetHandle")
    }
}
impl ::core::clone::Clone for ISynchronizeHandle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ISynchronizeHandle {}
impl ::core::cmp::PartialEq for ISynchronizeHandle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISynchronizeHandle {}
impl ::core::fmt::Debug for ISynchronizeHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISynchronizeHandle").field(&self.0).finish()
    }
}
impl FromIntoMemory for ISynchronizeHandle {
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
impl crate::core::ComInterface for ISynchronizeHandle {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000031_0000_0000_c000_000000000046);
}
pub struct ISynchronizeMutex(pub crate::core::IUnknown);
pub trait ISynchronizeMutex_Trait: ISynchronize_Trait {
    fn ReleaseMutex(&self) -> crate::core::HRESULT {
        todo!("ReleaseMutex")
    }
}
impl ::core::clone::Clone for ISynchronizeMutex {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ISynchronizeMutex {}
impl ::core::cmp::PartialEq for ISynchronizeMutex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISynchronizeMutex {}
impl ::core::fmt::Debug for ISynchronizeMutex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISynchronizeMutex").field(&self.0).finish()
    }
}
impl FromIntoMemory for ISynchronizeMutex {
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
impl crate::core::ComInterface for ISynchronizeMutex {
    type Super = ISynchronize;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000025_0000_0000_c000_000000000046);
}
pub struct ITimeAndNoticeControl(pub crate::core::IUnknown);
pub trait ITimeAndNoticeControl_Trait: crate::core::IUnknown_Trait {
    fn SuppressChanges(&self, res_1: u32, res_2: u32) -> crate::core::HRESULT {
        todo!("SuppressChanges")
    }
}
impl ::core::clone::Clone for ITimeAndNoticeControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ITimeAndNoticeControl {}
impl ::core::cmp::PartialEq for ITimeAndNoticeControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITimeAndNoticeControl {}
impl ::core::fmt::Debug for ITimeAndNoticeControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITimeAndNoticeControl")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ITimeAndNoticeControl {
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
impl crate::core::ComInterface for ITimeAndNoticeControl {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xbc0bf6ae_8878_11d1_83e9_00c04fc2c6d4);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ITypeComp(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait ITypeComp_Trait: crate::core::IUnknown_Trait {
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Bind(
        &self,
        sz_name: PCWSTR,
        l_hash_val: u32,
        w_flags: u16,
        pp_t_info: MutPtr<ITypeInfo>,
        p_desc_kind: MutPtr<DESCKIND>,
        p_bind_ptr: MutPtr<BINDPTR>,
    ) -> crate::core::HRESULT {
        todo!("Bind")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn BindType(
        &self,
        sz_name: PCWSTR,
        l_hash_val: u32,
        pp_t_info: MutPtr<ITypeInfo>,
        pp_t_comp: MutPtr<ITypeComp>,
    ) -> crate::core::HRESULT {
        todo!("BindType")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ITypeComp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ITypeComp {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ITypeComp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ITypeComp {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ITypeComp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeComp").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ITypeComp {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for ITypeComp {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00020403_0000_0000_c000_000000000046);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ITypeInfo(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait ITypeInfo_Trait: crate::core::IUnknown_Trait {
    #[doc = "*Required namespaces: 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetTypeAttr(&self, pp_type_attr: MutPtr<ConstPtr<TYPEATTR>>) -> crate::core::HRESULT {
        todo!("GetTypeAttr")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetTypeComp(&self, pp_t_comp: MutPtr<ITypeComp>) -> crate::core::HRESULT {
        todo!("GetTypeComp")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetFuncDesc(
        &self,
        index: u32,
        pp_func_desc: MutPtr<ConstPtr<FUNCDESC>>,
    ) -> crate::core::HRESULT {
        todo!("GetFuncDesc")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetVarDesc(
        &self,
        index: u32,
        pp_var_desc: MutPtr<ConstPtr<VARDESC>>,
    ) -> crate::core::HRESULT {
        todo!("GetVarDesc")
    }
    fn GetNames(
        &self,
        memid: i32,
        rg_bstr_names: MutPtr<super::super::Foundation::BSTR>,
        c_max_names: u32,
        pc_names: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("GetNames")
    }
    fn GetRefTypeOfImplType(&self, index: u32, p_ref_type: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetRefTypeOfImplType")
    }
    fn GetImplTypeFlags(&self, index: u32, p_impl_type_flags: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("GetImplTypeFlags")
    }
    fn GetIDsOfNames(
        &self,
        rgsz_names: ConstPtr<PWSTR>,
        c_names: u32,
        p_mem_id: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("GetIDsOfNames")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Invoke(
        &self,
        pv_instance: ConstPtr<::core::ffi::c_void>,
        memid: i32,
        w_flags: u16,
        p_disp_params: MutPtr<DISPPARAMS>,
        p_var_result: MutPtr<VARIANT>,
        p_excep_info: MutPtr<EXCEPINFO>,
        pu_arg_err: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("Invoke")
    }
    fn GetDocumentation(
        &self,
        memid: i32,
        p_bstr_name: MutPtr<super::super::Foundation::BSTR>,
        p_bstr_doc_string: MutPtr<super::super::Foundation::BSTR>,
        pdw_help_context: MutPtr<u32>,
        p_bstr_help_file: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetDocumentation")
    }
    fn GetDllEntry(
        &self,
        memid: i32,
        inv_kind: INVOKEKIND,
        p_bstr_dll_name: MutPtr<super::super::Foundation::BSTR>,
        p_bstr_name: MutPtr<super::super::Foundation::BSTR>,
        pw_ordinal: MutPtr<u16>,
    ) -> crate::core::HRESULT {
        todo!("GetDllEntry")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetRefTypeInfo(
        &self,
        h_ref_type: u32,
        pp_t_info: MutPtr<ITypeInfo>,
    ) -> crate::core::HRESULT {
        todo!("GetRefTypeInfo")
    }
    fn AddressOfMember(
        &self,
        memid: i32,
        inv_kind: INVOKEKIND,
        ppv: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("AddressOfMember")
    }
    fn CreateInstance(
        &self,
        p_unk_outer: crate::core::IUnknown,
        riid: ConstPtr<crate::core::GUID>,
        ppv_obj: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("CreateInstance")
    }
    fn GetMops(
        &self,
        memid: i32,
        p_bstr_mops: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetMops")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetContainingTypeLib(
        &self,
        pp_t_lib: MutPtr<ITypeLib>,
        p_index: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("GetContainingTypeLib")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn ReleaseTypeAttr(&self, p_type_attr: ConstPtr<TYPEATTR>) {
        todo!("ReleaseTypeAttr")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn ReleaseFuncDesc(&self, p_func_desc: ConstPtr<FUNCDESC>) {
        todo!("ReleaseFuncDesc")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn ReleaseVarDesc(&self, p_var_desc: ConstPtr<VARDESC>) {
        todo!("ReleaseVarDesc")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ITypeInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ITypeInfo {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ITypeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ITypeInfo {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ITypeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeInfo").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ITypeInfo {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for ITypeInfo {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00020401_0000_0000_c000_000000000046);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ITypeInfo2(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait ITypeInfo2_Trait: ITypeInfo_Trait {
    fn GetTypeKind(&self, p_type_kind: MutPtr<TYPEKIND>) -> crate::core::HRESULT {
        todo!("GetTypeKind")
    }
    fn GetTypeFlags(&self, p_type_flags: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetTypeFlags")
    }
    fn GetFuncIndexOfMemId(
        &self,
        memid: i32,
        inv_kind: INVOKEKIND,
        p_func_index: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("GetFuncIndexOfMemId")
    }
    fn GetVarIndexOfMemId(&self, memid: i32, p_var_index: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetVarIndexOfMemId")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetCustData(
        &self,
        guid: ConstPtr<crate::core::GUID>,
        p_var_val: MutPtr<VARIANT>,
    ) -> crate::core::HRESULT {
        todo!("GetCustData")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetFuncCustData(
        &self,
        index: u32,
        guid: ConstPtr<crate::core::GUID>,
        p_var_val: MutPtr<VARIANT>,
    ) -> crate::core::HRESULT {
        todo!("GetFuncCustData")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetParamCustData(
        &self,
        index_func: u32,
        index_param: u32,
        guid: ConstPtr<crate::core::GUID>,
        p_var_val: MutPtr<VARIANT>,
    ) -> crate::core::HRESULT {
        todo!("GetParamCustData")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetVarCustData(
        &self,
        index: u32,
        guid: ConstPtr<crate::core::GUID>,
        p_var_val: MutPtr<VARIANT>,
    ) -> crate::core::HRESULT {
        todo!("GetVarCustData")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetImplTypeCustData(
        &self,
        index: u32,
        guid: ConstPtr<crate::core::GUID>,
        p_var_val: MutPtr<VARIANT>,
    ) -> crate::core::HRESULT {
        todo!("GetImplTypeCustData")
    }
    fn GetDocumentation2(
        &self,
        memid: i32,
        lcid: u32,
        pbstr_help_string: MutPtr<super::super::Foundation::BSTR>,
        pdw_help_string_context: MutPtr<u32>,
        pbstr_help_string_dll: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetDocumentation2")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetAllCustData(&self, p_cust_data: MutPtr<CUSTDATA>) -> crate::core::HRESULT {
        todo!("GetAllCustData")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetAllFuncCustData(
        &self,
        index: u32,
        p_cust_data: MutPtr<CUSTDATA>,
    ) -> crate::core::HRESULT {
        todo!("GetAllFuncCustData")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetAllParamCustData(
        &self,
        index_func: u32,
        index_param: u32,
        p_cust_data: MutPtr<CUSTDATA>,
    ) -> crate::core::HRESULT {
        todo!("GetAllParamCustData")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetAllVarCustData(&self, index: u32, p_cust_data: MutPtr<CUSTDATA>) -> crate::core::HRESULT {
        todo!("GetAllVarCustData")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetAllImplTypeCustData(
        &self,
        index: u32,
        p_cust_data: MutPtr<CUSTDATA>,
    ) -> crate::core::HRESULT {
        todo!("GetAllImplTypeCustData")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ITypeInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ITypeInfo2 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ITypeInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ITypeInfo2 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ITypeInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeInfo2").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ITypeInfo2 {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for ITypeInfo2 {
    type Super = ITypeInfo;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00020412_0000_0000_c000_000000000046);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ITypeLib(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait ITypeLib_Trait: crate::core::IUnknown_Trait {
    fn GetTypeInfoCount(&self) -> u32 {
        todo!("GetTypeInfoCount")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetTypeInfo(&self, index: u32, pp_t_info: MutPtr<ITypeInfo>) -> crate::core::HRESULT {
        todo!("GetTypeInfo")
    }
    fn GetTypeInfoType(&self, index: u32, p_t_kind: MutPtr<TYPEKIND>) -> crate::core::HRESULT {
        todo!("GetTypeInfoType")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetTypeInfoOfGuid(
        &self,
        guid: ConstPtr<crate::core::GUID>,
        pp_tinfo: MutPtr<ITypeInfo>,
    ) -> crate::core::HRESULT {
        todo!("GetTypeInfoOfGuid")
    }
    fn GetLibAttr(&self, pp_t_lib_attr: MutPtr<ConstPtr<TLIBATTR>>) -> crate::core::HRESULT {
        todo!("GetLibAttr")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetTypeComp(&self, pp_t_comp: MutPtr<ITypeComp>) -> crate::core::HRESULT {
        todo!("GetTypeComp")
    }
    fn GetDocumentation(
        &self,
        index: i32,
        p_bstr_name: MutPtr<super::super::Foundation::BSTR>,
        p_bstr_doc_string: MutPtr<super::super::Foundation::BSTR>,
        pdw_help_context: MutPtr<u32>,
        p_bstr_help_file: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetDocumentation")
    }
    fn IsName(
        &self,
        sz_name_buf: PWSTR,
        l_hash_val: u32,
        pf_name: MutPtr<super::super::Foundation::BOOL>,
    ) -> crate::core::HRESULT {
        todo!("IsName")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn FindName(
        &self,
        sz_name_buf: PWSTR,
        l_hash_val: u32,
        pp_t_info: MutPtr<ITypeInfo>,
        rg_mem_id: MutPtr<i32>,
        pc_found: MutPtr<u16>,
    ) -> crate::core::HRESULT {
        todo!("FindName")
    }
    fn ReleaseTLibAttr(&self, p_t_lib_attr: ConstPtr<TLIBATTR>) {
        todo!("ReleaseTLibAttr")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ITypeLib {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ITypeLib {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ITypeLib {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ITypeLib {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ITypeLib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeLib").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ITypeLib {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for ITypeLib {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00020402_0000_0000_c000_000000000046);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ITypeLib2(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait ITypeLib2_Trait: ITypeLib_Trait {
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetCustData(
        &self,
        guid: ConstPtr<crate::core::GUID>,
        p_var_val: MutPtr<VARIANT>,
    ) -> crate::core::HRESULT {
        todo!("GetCustData")
    }
    fn GetLibStatistics(
        &self,
        pc_unique_names: MutPtr<u32>,
        pcch_unique_names: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("GetLibStatistics")
    }
    fn GetDocumentation2(
        &self,
        index: i32,
        lcid: u32,
        pbstr_help_string: MutPtr<super::super::Foundation::BSTR>,
        pdw_help_string_context: MutPtr<u32>,
        pbstr_help_string_dll: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetDocumentation2")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetAllCustData(&self, p_cust_data: MutPtr<CUSTDATA>) -> crate::core::HRESULT {
        todo!("GetAllCustData")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ITypeLib2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ITypeLib2 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ITypeLib2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ITypeLib2 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ITypeLib2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeLib2").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ITypeLib2 {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for ITypeLib2 {
    type Super = ITypeLib;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00020411_0000_0000_c000_000000000046);
}
pub struct ITypeLibRegistration(pub crate::core::IUnknown);
pub trait ITypeLibRegistration_Trait: crate::core::IUnknown_Trait {
    fn GetGuid(&self, p_guid: MutPtr<crate::core::GUID>) -> crate::core::HRESULT {
        todo!("GetGuid")
    }
    fn GetVersion(
        &self,
        p_version: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetVersion")
    }
    fn GetLcid(&self, p_lcid: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetLcid")
    }
    fn GetWin32Path(
        &self,
        p_win_32_path: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetWin32Path")
    }
    fn GetWin64Path(
        &self,
        p_win_64_path: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetWin64Path")
    }
    fn GetDisplayName(
        &self,
        p_display_name: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetDisplayName")
    }
    fn GetFlags(&self, p_flags: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetFlags")
    }
    fn GetHelpDir(
        &self,
        p_help_dir: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetHelpDir")
    }
}
impl ::core::clone::Clone for ITypeLibRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ITypeLibRegistration {}
impl ::core::cmp::PartialEq for ITypeLibRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeLibRegistration {}
impl ::core::fmt::Debug for ITypeLibRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeLibRegistration")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ITypeLibRegistration {
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
impl crate::core::ComInterface for ITypeLibRegistration {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x76a3e735_02df_4a12_98eb_043ad3600af3);
}
pub struct ITypeLibRegistrationReader(pub crate::core::IUnknown);
pub trait ITypeLibRegistrationReader_Trait: crate::core::IUnknown_Trait {
    fn EnumTypeLibRegistrations(
        &self,
        pp_enum_unknown: MutPtr<IEnumUnknown>,
    ) -> crate::core::HRESULT {
        todo!("EnumTypeLibRegistrations")
    }
}
impl ::core::clone::Clone for ITypeLibRegistrationReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ITypeLibRegistrationReader {}
impl ::core::cmp::PartialEq for ITypeLibRegistrationReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeLibRegistrationReader {}
impl ::core::fmt::Debug for ITypeLibRegistrationReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeLibRegistrationReader")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ITypeLibRegistrationReader {
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
impl crate::core::ComInterface for ITypeLibRegistrationReader {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xed6a8a2a_b160_4e77_8f73_aa7435cd5c27);
}
pub struct IUri(pub crate::core::IUnknown);
pub trait IUri_Trait: crate::core::IUnknown_Trait {
    fn GetPropertyBSTR(
        &self,
        uri_prop: Uri_PROPERTY,
        pbstr_property: MutPtr<super::super::Foundation::BSTR>,
        dw_flags: u32,
    ) -> crate::core::HRESULT {
        todo!("GetPropertyBSTR")
    }
    fn GetPropertyLength(
        &self,
        uri_prop: Uri_PROPERTY,
        pcch_property: MutPtr<u32>,
        dw_flags: u32,
    ) -> crate::core::HRESULT {
        todo!("GetPropertyLength")
    }
    fn GetPropertyDWORD(
        &self,
        uri_prop: Uri_PROPERTY,
        pdw_property: MutPtr<u32>,
        dw_flags: u32,
    ) -> crate::core::HRESULT {
        todo!("GetPropertyDWORD")
    }
    fn HasProperty(
        &self,
        uri_prop: Uri_PROPERTY,
        pf_has_property: MutPtr<super::super::Foundation::BOOL>,
    ) -> crate::core::HRESULT {
        todo!("HasProperty")
    }
    fn GetAbsoluteUri(
        &self,
        pbstr_absolute_uri: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetAbsoluteUri")
    }
    fn GetAuthority(
        &self,
        pbstr_authority: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetAuthority")
    }
    fn GetDisplayUri(
        &self,
        pbstr_display_string: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetDisplayUri")
    }
    fn GetDomain(
        &self,
        pbstr_domain: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetDomain")
    }
    fn GetExtension(
        &self,
        pbstr_extension: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetExtension")
    }
    fn GetFragment(
        &self,
        pbstr_fragment: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetFragment")
    }
    fn GetHost(&self, pbstr_host: MutPtr<super::super::Foundation::BSTR>) -> crate::core::HRESULT {
        todo!("GetHost")
    }
    fn GetPassword(
        &self,
        pbstr_password: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetPassword")
    }
    fn GetPath(&self, pbstr_path: MutPtr<super::super::Foundation::BSTR>) -> crate::core::HRESULT {
        todo!("GetPath")
    }
    fn GetPathAndQuery(
        &self,
        pbstr_path_and_query: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetPathAndQuery")
    }
    fn GetQuery(
        &self,
        pbstr_query: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetQuery")
    }
    fn GetRawUri(
        &self,
        pbstr_raw_uri: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetRawUri")
    }
    fn GetSchemeName(
        &self,
        pbstr_scheme_name: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetSchemeName")
    }
    fn GetUserInfo(
        &self,
        pbstr_user_info: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetUserInfo")
    }
    fn GetUserName(
        &self,
        pbstr_user_name: MutPtr<super::super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetUserName")
    }
    fn GetHostType(&self, pdw_host_type: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetHostType")
    }
    fn GetPort(&self, pdw_port: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetPort")
    }
    fn GetScheme(&self, pdw_scheme: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetScheme")
    }
    fn GetZone(&self, pdw_zone: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetZone")
    }
    fn GetProperties(&self, pdw_flags: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetProperties")
    }
    fn IsEqual(
        &self,
        p_uri: IUri,
        pf_equal: MutPtr<super::super::Foundation::BOOL>,
    ) -> crate::core::HRESULT {
        todo!("IsEqual")
    }
}
impl ::core::clone::Clone for IUri {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IUri {}
impl ::core::cmp::PartialEq for IUri {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUri {}
impl ::core::fmt::Debug for IUri {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUri").field(&self.0).finish()
    }
}
impl FromIntoMemory for IUri {
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
impl crate::core::ComInterface for IUri {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xa39ee748_6a27_4817_a6f2_13914bef5890);
}
pub struct IUriBuilder(pub crate::core::IUnknown);
pub trait IUriBuilder_Trait: crate::core::IUnknown_Trait {
    fn CreateUriSimple(
        &self,
        dw_allow_encoding_property_mask: u32,
        dw_reserved: PtrRepr,
        pp_i_uri: MutPtr<IUri>,
    ) -> crate::core::HRESULT {
        todo!("CreateUriSimple")
    }
    fn CreateUri(
        &self,
        dw_create_flags: u32,
        dw_allow_encoding_property_mask: u32,
        dw_reserved: PtrRepr,
        pp_i_uri: MutPtr<IUri>,
    ) -> crate::core::HRESULT {
        todo!("CreateUri")
    }
    fn CreateUriWithFlags(
        &self,
        dw_create_flags: u32,
        dw_uri_builder_flags: u32,
        dw_allow_encoding_property_mask: u32,
        dw_reserved: PtrRepr,
        pp_i_uri: MutPtr<IUri>,
    ) -> crate::core::HRESULT {
        todo!("CreateUriWithFlags")
    }
    fn GetIUri(&self, pp_i_uri: MutPtr<IUri>) -> crate::core::HRESULT {
        todo!("GetIUri")
    }
    fn SetIUri(&self, p_i_uri: IUri) -> crate::core::HRESULT {
        todo!("SetIUri")
    }
    fn GetFragment(
        &self,
        pcch_fragment: MutPtr<u32>,
        ppwz_fragment: MutPtr<PWSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetFragment")
    }
    fn GetHost(&self, pcch_host: MutPtr<u32>, ppwz_host: MutPtr<PWSTR>) -> crate::core::HRESULT {
        todo!("GetHost")
    }
    fn GetPassword(
        &self,
        pcch_password: MutPtr<u32>,
        ppwz_password: MutPtr<PWSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetPassword")
    }
    fn GetPath(&self, pcch_path: MutPtr<u32>, ppwz_path: MutPtr<PWSTR>) -> crate::core::HRESULT {
        todo!("GetPath")
    }
    fn GetPort(
        &self,
        pf_has_port: MutPtr<super::super::Foundation::BOOL>,
        pdw_port: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("GetPort")
    }
    fn GetQuery(&self, pcch_query: MutPtr<u32>, ppwz_query: MutPtr<PWSTR>) -> crate::core::HRESULT {
        todo!("GetQuery")
    }
    fn GetSchemeName(
        &self,
        pcch_scheme_name: MutPtr<u32>,
        ppwz_scheme_name: MutPtr<PWSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetSchemeName")
    }
    fn GetUserName(
        &self,
        pcch_user_name: MutPtr<u32>,
        ppwz_user_name: MutPtr<PWSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetUserName")
    }
    fn SetFragment(&self, pwz_new_value: PCWSTR) -> crate::core::HRESULT {
        todo!("SetFragment")
    }
    fn SetHost(&self, pwz_new_value: PCWSTR) -> crate::core::HRESULT {
        todo!("SetHost")
    }
    fn SetPassword(&self, pwz_new_value: PCWSTR) -> crate::core::HRESULT {
        todo!("SetPassword")
    }
    fn SetPath(&self, pwz_new_value: PCWSTR) -> crate::core::HRESULT {
        todo!("SetPath")
    }
    fn SetPort(
        &self,
        f_has_port: super::super::Foundation::BOOL,
        dw_new_value: u32,
    ) -> crate::core::HRESULT {
        todo!("SetPort")
    }
    fn SetQuery(&self, pwz_new_value: PCWSTR) -> crate::core::HRESULT {
        todo!("SetQuery")
    }
    fn SetSchemeName(&self, pwz_new_value: PCWSTR) -> crate::core::HRESULT {
        todo!("SetSchemeName")
    }
    fn SetUserName(&self, pwz_new_value: PCWSTR) -> crate::core::HRESULT {
        todo!("SetUserName")
    }
    fn RemoveProperties(&self, dw_property_mask: u32) -> crate::core::HRESULT {
        todo!("RemoveProperties")
    }
    fn HasBeenModified(
        &self,
        pf_modified: MutPtr<super::super::Foundation::BOOL>,
    ) -> crate::core::HRESULT {
        todo!("HasBeenModified")
    }
}
impl ::core::clone::Clone for IUriBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IUriBuilder {}
impl ::core::cmp::PartialEq for IUriBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUriBuilder {}
impl ::core::fmt::Debug for IUriBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUriBuilder").field(&self.0).finish()
    }
}
impl FromIntoMemory for IUriBuilder {
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
impl crate::core::ComInterface for IUriBuilder {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x4221b2e1_8955_46c0_bd5b_de9897565de7);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IUrlMon(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IUrlMon_Trait: crate::core::IUnknown_Trait {
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn AsyncGetClassBits(
        &self,
        rclsid: ConstPtr<crate::core::GUID>,
        psz_type: PCWSTR,
        psz_ext: PCWSTR,
        dw_file_version_ms: u32,
        dw_file_version_ls: u32,
        psz_code_base: PCWSTR,
        pbc: IBindCtx,
        dw_class_context: u32,
        riid: ConstPtr<crate::core::GUID>,
        flags: u32,
    ) -> crate::core::HRESULT {
        todo!("AsyncGetClassBits")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IUrlMon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IUrlMon {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IUrlMon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IUrlMon {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IUrlMon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUrlMon").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IUrlMon {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IUrlMon {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000026_0000_0000_c000_000000000046);
}
pub struct IWaitMultiple(pub crate::core::IUnknown);
pub trait IWaitMultiple_Trait: crate::core::IUnknown_Trait {
    fn WaitMultiple(&self, timeout: u32, p_sync: MutPtr<ISynchronize>) -> crate::core::HRESULT {
        todo!("WaitMultiple")
    }
    fn AddSynchronize(&self, p_sync: ISynchronize) -> crate::core::HRESULT {
        todo!("AddSynchronize")
    }
}
impl ::core::clone::Clone for IWaitMultiple {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IWaitMultiple {}
impl ::core::cmp::PartialEq for IWaitMultiple {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWaitMultiple {}
impl ::core::fmt::Debug for IWaitMultiple {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWaitMultiple").field(&self.0).finish()
    }
}
impl FromIntoMemory for IWaitMultiple {
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
impl crate::core::ComInterface for IWaitMultiple {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x0000002b_0000_0000_c000_000000000046);
}
pub struct LONG_SIZEDARR {
    pub clSize: u32,
    pub pData: MutPtr<u32>,
}
impl ::core::marker::Copy for LONG_SIZEDARR {}
impl ::core::clone::Clone for LONG_SIZEDARR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LONG_SIZEDARR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LONG_SIZEDARR")
            .field("clSize", &self.clSize)
            .field("pData", &self.pData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for LONG_SIZEDARR {
    fn eq(&self, other: &Self) -> bool {
        self.clSize == other.clSize && self.pData == other.pData
    }
}
impl ::core::cmp::Eq for LONG_SIZEDARR {}
impl FromIntoMemory for LONG_SIZEDARR {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_clSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_pData = <MutPtr<u32> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            clSize: f_clSize,
            pData: f_pData,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.clSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.pData, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub type LPEXCEPFINO_DEFERRED_FILLIN = StdCallFnPtr<(MutPtr<EXCEPINFO>,), crate::core::HRESULT>;
pub type LPFNCANUNLOADNOW = StdCallFnPtr<(), crate::core::HRESULT>;
pub type LPFNGETCLASSOBJECT = StdCallFnPtr<
    (
        ConstPtr<crate::core::GUID>,
        ConstPtr<crate::core::GUID>,
        MutPtr<ConstPtr<::core::ffi::c_void>>,
    ),
    crate::core::HRESULT,
>;
pub const MARSHALINTERFACE_MIN: u32 = 500u32;
pub const MAXLSN: u64 = 9223372036854775807u64;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MEMCTX(pub i32);
pub const MEMCTX_TASK: MEMCTX = MEMCTX(1i32);
pub const MEMCTX_SHARED: MEMCTX = MEMCTX(2i32);
pub const MEMCTX_MACSYSTEM: MEMCTX = MEMCTX(3i32);
pub const MEMCTX_UNKNOWN: MEMCTX = MEMCTX(-1i32);
pub const MEMCTX_SAME: MEMCTX = MEMCTX(-2i32);
impl ::core::marker::Copy for MEMCTX {}
impl ::core::clone::Clone for MEMCTX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MEMCTX {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MEMCTX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEMCTX").field(&self.0).finish()
    }
}
impl FromIntoMemory for MEMCTX {
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
pub struct MKREDUCE(pub i32);
pub const MKRREDUCE_ONE: MKREDUCE = MKREDUCE(196608i32);
pub const MKRREDUCE_TOUSER: MKREDUCE = MKREDUCE(131072i32);
pub const MKRREDUCE_THROUGHUSER: MKREDUCE = MKREDUCE(65536i32);
pub const MKRREDUCE_ALL: MKREDUCE = MKREDUCE(0i32);
impl ::core::marker::Copy for MKREDUCE {}
impl ::core::clone::Clone for MKREDUCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MKREDUCE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MKREDUCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MKREDUCE").field(&self.0).finish()
    }
}
impl FromIntoMemory for MKREDUCE {
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
pub struct MKSYS(pub i32);
pub const MKSYS_NONE: MKSYS = MKSYS(0i32);
pub const MKSYS_GENERICCOMPOSITE: MKSYS = MKSYS(1i32);
pub const MKSYS_FILEMONIKER: MKSYS = MKSYS(2i32);
pub const MKSYS_ANTIMONIKER: MKSYS = MKSYS(3i32);
pub const MKSYS_ITEMMONIKER: MKSYS = MKSYS(4i32);
pub const MKSYS_POINTERMONIKER: MKSYS = MKSYS(5i32);
pub const MKSYS_CLASSMONIKER: MKSYS = MKSYS(7i32);
pub const MKSYS_OBJREFMONIKER: MKSYS = MKSYS(8i32);
pub const MKSYS_SESSIONMONIKER: MKSYS = MKSYS(9i32);
pub const MKSYS_LUAMONIKER: MKSYS = MKSYS(10i32);
impl ::core::marker::Copy for MKSYS {}
impl ::core::clone::Clone for MKSYS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MKSYS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MKSYS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MKSYS").field(&self.0).finish()
    }
}
impl FromIntoMemory for MKSYS {
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
pub struct MSHCTX(pub i32);
pub const MSHCTX_LOCAL: MSHCTX = MSHCTX(0i32);
pub const MSHCTX_NOSHAREDMEM: MSHCTX = MSHCTX(1i32);
pub const MSHCTX_DIFFERENTMACHINE: MSHCTX = MSHCTX(2i32);
pub const MSHCTX_INPROC: MSHCTX = MSHCTX(3i32);
pub const MSHCTX_CROSSCTX: MSHCTX = MSHCTX(4i32);
pub const MSHCTX_CONTAINER: MSHCTX = MSHCTX(5i32);
impl ::core::marker::Copy for MSHCTX {}
impl ::core::clone::Clone for MSHCTX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSHCTX {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MSHCTX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSHCTX").field(&self.0).finish()
    }
}
impl FromIntoMemory for MSHCTX {
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
pub struct MSHLFLAGS(pub i32);
pub const MSHLFLAGS_NORMAL: MSHLFLAGS = MSHLFLAGS(0i32);
pub const MSHLFLAGS_TABLESTRONG: MSHLFLAGS = MSHLFLAGS(1i32);
pub const MSHLFLAGS_TABLEWEAK: MSHLFLAGS = MSHLFLAGS(2i32);
pub const MSHLFLAGS_NOPING: MSHLFLAGS = MSHLFLAGS(4i32);
pub const MSHLFLAGS_RESERVED1: MSHLFLAGS = MSHLFLAGS(8i32);
pub const MSHLFLAGS_RESERVED2: MSHLFLAGS = MSHLFLAGS(16i32);
pub const MSHLFLAGS_RESERVED3: MSHLFLAGS = MSHLFLAGS(32i32);
pub const MSHLFLAGS_RESERVED4: MSHLFLAGS = MSHLFLAGS(64i32);
impl ::core::marker::Copy for MSHLFLAGS {}
impl ::core::clone::Clone for MSHLFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSHLFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MSHLFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSHLFLAGS").field(&self.0).finish()
    }
}
impl FromIntoMemory for MSHLFLAGS {
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
pub struct MULTI_QI {
    pub pIID: ConstPtr<crate::core::GUID>,
    pub pItf: crate::core::IUnknown,
    pub hr: crate::core::HRESULT,
}
impl ::core::marker::Copy for MULTI_QI {}
impl ::core::clone::Clone for MULTI_QI {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MULTI_QI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MULTI_QI")
            .field("pIID", &self.pIID)
            .field("pItf", &self.pItf)
            .field("hr", &self.hr)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MULTI_QI {
    fn eq(&self, other: &Self) -> bool {
        self.pIID == other.pIID && self.pItf == other.pItf && self.hr == other.hr
    }
}
impl ::core::cmp::Eq for MULTI_QI {}
impl FromIntoMemory for MULTI_QI {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_pIID = <ConstPtr<crate::core::GUID> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_pItf = <crate::core::IUnknown as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_hr = <crate::core::HRESULT as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            pIID: f_pIID,
            pItf: f_pItf,
            hr: f_hr,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.pIID, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.pItf, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.hr, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct MachineGlobalObjectTableRegistrationToken__ {
    pub unused: i32,
}
impl ::core::marker::Copy for MachineGlobalObjectTableRegistrationToken__ {}
impl ::core::clone::Clone for MachineGlobalObjectTableRegistrationToken__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MachineGlobalObjectTableRegistrationToken__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MachineGlobalObjectTableRegistrationToken__")
            .field("unused", &self.unused)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MachineGlobalObjectTableRegistrationToken__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for MachineGlobalObjectTableRegistrationToken__ {}
impl FromIntoMemory for MachineGlobalObjectTableRegistrationToken__ {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PENDINGMSG(pub i32);
pub const PENDINGMSG_CANCELCALL: PENDINGMSG = PENDINGMSG(0i32);
pub const PENDINGMSG_WAITNOPROCESS: PENDINGMSG = PENDINGMSG(1i32);
pub const PENDINGMSG_WAITDEFPROCESS: PENDINGMSG = PENDINGMSG(2i32);
impl ::core::marker::Copy for PENDINGMSG {}
impl ::core::clone::Clone for PENDINGMSG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PENDINGMSG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PENDINGMSG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PENDINGMSG").field(&self.0).finish()
    }
}
impl FromIntoMemory for PENDINGMSG {
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
pub struct PENDINGTYPE(pub i32);
pub const PENDINGTYPE_TOPLEVEL: PENDINGTYPE = PENDINGTYPE(1i32);
pub const PENDINGTYPE_NESTED: PENDINGTYPE = PENDINGTYPE(2i32);
impl ::core::marker::Copy for PENDINGTYPE {}
impl ::core::clone::Clone for PENDINGTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PENDINGTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PENDINGTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PENDINGTYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for PENDINGTYPE {
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
pub type PFNCONTEXTCALL = StdCallFnPtr<(MutPtr<ComCallData>,), crate::core::HRESULT>;
pub struct QUERYCONTEXT {
    pub dwContext: u32,
    pub Platform: CSPLATFORM,
    pub Locale: u32,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
}
impl ::core::marker::Copy for QUERYCONTEXT {}
impl ::core::clone::Clone for QUERYCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUERYCONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUERYCONTEXT")
            .field("dwContext", &self.dwContext)
            .field("Platform", &self.Platform)
            .field("Locale", &self.Locale)
            .field("dwVersionHi", &self.dwVersionHi)
            .field("dwVersionLo", &self.dwVersionLo)
            .finish()
    }
}
impl ::core::cmp::PartialEq for QUERYCONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.dwContext == other.dwContext
            && self.Platform == other.Platform
            && self.Locale == other.Locale
            && self.dwVersionHi == other.dwVersionHi
            && self.dwVersionLo == other.dwVersionLo
    }
}
impl ::core::cmp::Eq for QUERYCONTEXT {}
impl FromIntoMemory for QUERYCONTEXT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_dwContext = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Platform = <CSPLATFORM as FromIntoMemory>::from_bytes(&from[4..4 + 16]);
        let f_Locale = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwVersionHi = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_dwVersionLo = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            dwContext: f_dwContext,
            Platform: f_Platform,
            Locale: f_Locale,
            dwVersionHi: f_dwVersionHi,
            dwVersionLo: f_dwVersionLo,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.dwContext, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Platform, &mut into[4..4 + 16]);
        FromIntoMemory::into_bytes(self.Locale, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwVersionHi, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.dwVersionLo, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct REGCLS(pub i32);
pub const REGCLS_SINGLEUSE: REGCLS = REGCLS(0i32);
pub const REGCLS_MULTIPLEUSE: REGCLS = REGCLS(1i32);
pub const REGCLS_MULTI_SEPARATE: REGCLS = REGCLS(2i32);
pub const REGCLS_SUSPENDED: REGCLS = REGCLS(4i32);
pub const REGCLS_SURROGATE: REGCLS = REGCLS(8i32);
pub const REGCLS_AGILE: REGCLS = REGCLS(16i32);
impl ::core::marker::Copy for REGCLS {}
impl ::core::clone::Clone for REGCLS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REGCLS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for REGCLS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REGCLS").field(&self.0).finish()
    }
}
impl FromIntoMemory for REGCLS {
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
pub const ROTREGFLAGS_ALLOWANYCLIENT: u32 = 1u32;
pub struct RPCOLEMESSAGE {
    pub reserved1: MutPtr<::core::ffi::c_void>,
    pub dataRepresentation: u32,
    pub Buffer: MutPtr<::core::ffi::c_void>,
    pub cbBuffer: u32,
    pub iMethod: u32,
    pub reserved2: [MutPtr<::core::ffi::c_void>; 5],
    pub rpcFlags: u32,
}
impl ::core::marker::Copy for RPCOLEMESSAGE {}
impl ::core::clone::Clone for RPCOLEMESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPCOLEMESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPCOLEMESSAGE")
            .field("reserved1", &self.reserved1)
            .field("dataRepresentation", &self.dataRepresentation)
            .field("Buffer", &self.Buffer)
            .field("cbBuffer", &self.cbBuffer)
            .field("iMethod", &self.iMethod)
            .field("reserved2", &self.reserved2)
            .field("rpcFlags", &self.rpcFlags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RPCOLEMESSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.reserved1 == other.reserved1
            && self.dataRepresentation == other.dataRepresentation
            && self.Buffer == other.Buffer
            && self.cbBuffer == other.cbBuffer
            && self.iMethod == other.iMethod
            && self.reserved2 == other.reserved2
            && self.rpcFlags == other.rpcFlags
    }
}
impl ::core::cmp::Eq for RPCOLEMESSAGE {}
impl FromIntoMemory for RPCOLEMESSAGE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 44);
        let f_reserved1 =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dataRepresentation = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Buffer = <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_cbBuffer = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_iMethod = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_reserved2 =
            <[MutPtr<::core::ffi::c_void>; 5] as FromIntoMemory>::from_bytes(&from[20..20 + 20]);
        let f_rpcFlags = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        Self {
            reserved1: f_reserved1,
            dataRepresentation: f_dataRepresentation,
            Buffer: f_Buffer,
            cbBuffer: f_cbBuffer,
            iMethod: f_iMethod,
            reserved2: f_reserved2,
            rpcFlags: f_rpcFlags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 44);
        FromIntoMemory::into_bytes(self.reserved1, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dataRepresentation, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Buffer, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.cbBuffer, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.iMethod, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.reserved2, &mut into[20..20 + 20]);
        FromIntoMemory::into_bytes(self.rpcFlags, &mut into[40..40 + 4]);
    }
    fn size() -> usize {
        44
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RPCOPT_PROPERTIES(pub i32);
pub const COMBND_RPCTIMEOUT: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(1i32);
pub const COMBND_SERVER_LOCALITY: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(2i32);
pub const COMBND_RESERVED1: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(4i32);
pub const COMBND_RESERVED2: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(5i32);
pub const COMBND_RESERVED3: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(8i32);
pub const COMBND_RESERVED4: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(16i32);
impl ::core::marker::Copy for RPCOPT_PROPERTIES {}
impl ::core::clone::Clone for RPCOPT_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPCOPT_PROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RPCOPT_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPCOPT_PROPERTIES").field(&self.0).finish()
    }
}
impl FromIntoMemory for RPCOPT_PROPERTIES {
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
pub struct RPCOPT_SERVER_LOCALITY_VALUES(pub i32);
pub const SERVER_LOCALITY_PROCESS_LOCAL: RPCOPT_SERVER_LOCALITY_VALUES =
    RPCOPT_SERVER_LOCALITY_VALUES(0i32);
pub const SERVER_LOCALITY_MACHINE_LOCAL: RPCOPT_SERVER_LOCALITY_VALUES =
    RPCOPT_SERVER_LOCALITY_VALUES(1i32);
pub const SERVER_LOCALITY_REMOTE: RPCOPT_SERVER_LOCALITY_VALUES =
    RPCOPT_SERVER_LOCALITY_VALUES(2i32);
impl ::core::marker::Copy for RPCOPT_SERVER_LOCALITY_VALUES {}
impl ::core::clone::Clone for RPCOPT_SERVER_LOCALITY_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPCOPT_SERVER_LOCALITY_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RPCOPT_SERVER_LOCALITY_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPCOPT_SERVER_LOCALITY_VALUES")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for RPCOPT_SERVER_LOCALITY_VALUES {
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
pub struct RPC_C_AUTHN_LEVEL(pub u32);
pub const RPC_C_AUTHN_LEVEL_DEFAULT: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(0u32);
pub const RPC_C_AUTHN_LEVEL_NONE: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(1u32);
pub const RPC_C_AUTHN_LEVEL_CONNECT: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(2u32);
pub const RPC_C_AUTHN_LEVEL_CALL: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(3u32);
pub const RPC_C_AUTHN_LEVEL_PKT: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(4u32);
pub const RPC_C_AUTHN_LEVEL_PKT_INTEGRITY: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(5u32);
pub const RPC_C_AUTHN_LEVEL_PKT_PRIVACY: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(6u32);
impl ::core::marker::Copy for RPC_C_AUTHN_LEVEL {}
impl ::core::clone::Clone for RPC_C_AUTHN_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_C_AUTHN_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RPC_C_AUTHN_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_C_AUTHN_LEVEL").field(&self.0).finish()
    }
}
impl FromIntoMemory for RPC_C_AUTHN_LEVEL {
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
pub struct RPC_C_IMP_LEVEL(pub u32);
pub const RPC_C_IMP_LEVEL_DEFAULT: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(0u32);
pub const RPC_C_IMP_LEVEL_ANONYMOUS: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(1u32);
pub const RPC_C_IMP_LEVEL_IDENTIFY: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(2u32);
pub const RPC_C_IMP_LEVEL_IMPERSONATE: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(3u32);
pub const RPC_C_IMP_LEVEL_DELEGATE: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(4u32);
impl ::core::marker::Copy for RPC_C_IMP_LEVEL {}
impl ::core::clone::Clone for RPC_C_IMP_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_C_IMP_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RPC_C_IMP_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_C_IMP_LEVEL").field(&self.0).finish()
    }
}
impl FromIntoMemory for RPC_C_IMP_LEVEL {
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
pub struct RemSTGMEDIUM {
    pub tymed: u32,
    pub dwHandleType: u32,
    pub pData: u32,
    pub pUnkForRelease: u32,
    pub cbData: u32,
    pub data: [u8; 1],
}
impl ::core::marker::Copy for RemSTGMEDIUM {}
impl ::core::clone::Clone for RemSTGMEDIUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RemSTGMEDIUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RemSTGMEDIUM")
            .field("tymed", &self.tymed)
            .field("dwHandleType", &self.dwHandleType)
            .field("pData", &self.pData)
            .field("pUnkForRelease", &self.pUnkForRelease)
            .field("cbData", &self.cbData)
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RemSTGMEDIUM {
    fn eq(&self, other: &Self) -> bool {
        self.tymed == other.tymed
            && self.dwHandleType == other.dwHandleType
            && self.pData == other.pData
            && self.pUnkForRelease == other.pUnkForRelease
            && self.cbData == other.cbData
            && self.data == other.data
    }
}
impl ::core::cmp::Eq for RemSTGMEDIUM {}
impl FromIntoMemory for RemSTGMEDIUM {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_tymed = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwHandleType = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_pData = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_pUnkForRelease = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_cbData = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_data = <[u8; 1] as FromIntoMemory>::from_bytes(&from[20..20 + 1]);
        Self {
            tymed: f_tymed,
            dwHandleType: f_dwHandleType,
            pData: f_pData,
            pUnkForRelease: f_pUnkForRelease,
            cbData: f_cbData,
            data: f_data,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.tymed, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwHandleType, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.pData, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.pUnkForRelease, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.cbData, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.data, &mut into[20..20 + 1]);
    }
    fn size() -> usize {
        24
    }
}
pub struct SAFEARRAY {
    pub cDims: u16,
    pub fFeatures: u16,
    pub cbElements: u32,
    pub cLocks: u32,
    pub pvData: MutPtr<::core::ffi::c_void>,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
impl ::core::marker::Copy for SAFEARRAY {}
impl ::core::clone::Clone for SAFEARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SAFEARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAFEARRAY")
            .field("cDims", &self.cDims)
            .field("fFeatures", &self.fFeatures)
            .field("cbElements", &self.cbElements)
            .field("cLocks", &self.cLocks)
            .field("pvData", &self.pvData)
            .field("rgsabound", &self.rgsabound)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SAFEARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.cDims == other.cDims
            && self.fFeatures == other.fFeatures
            && self.cbElements == other.cbElements
            && self.cLocks == other.cLocks
            && self.pvData == other.pvData
            && self.rgsabound == other.rgsabound
    }
}
impl ::core::cmp::Eq for SAFEARRAY {}
impl FromIntoMemory for SAFEARRAY {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_cDims = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_fFeatures = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_cbElements = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_cLocks = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_pvData =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_rgsabound = <[SAFEARRAYBOUND; 1] as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        Self {
            cDims: f_cDims,
            fFeatures: f_fFeatures,
            cbElements: f_cbElements,
            cLocks: f_cLocks,
            pvData: f_pvData,
            rgsabound: f_rgsabound,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.cDims, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.fFeatures, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.cbElements, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.cLocks, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.pvData, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.rgsabound, &mut into[16..16 + 8]);
    }
    fn size() -> usize {
        24
    }
}
pub struct SAFEARRAYBOUND {
    pub cElements: u32,
    pub lLbound: i32,
}
impl ::core::marker::Copy for SAFEARRAYBOUND {}
impl ::core::clone::Clone for SAFEARRAYBOUND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SAFEARRAYBOUND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAFEARRAYBOUND")
            .field("cElements", &self.cElements)
            .field("lLbound", &self.lLbound)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SAFEARRAYBOUND {
    fn eq(&self, other: &Self) -> bool {
        self.cElements == other.cElements && self.lLbound == other.lLbound
    }
}
impl ::core::cmp::Eq for SAFEARRAYBOUND {}
impl FromIntoMemory for SAFEARRAYBOUND {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_cElements = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lLbound = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            cElements: f_cElements,
            lLbound: f_lLbound,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.cElements, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lLbound, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct SChannelHookCallInfo {
    pub iid: crate::core::GUID,
    pub cbSize: u32,
    pub uCausality: crate::core::GUID,
    pub dwServerPid: u32,
    pub iMethod: u32,
    pub pObject: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for SChannelHookCallInfo {}
impl ::core::clone::Clone for SChannelHookCallInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SChannelHookCallInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SChannelHookCallInfo")
            .field("iid", &self.iid)
            .field("cbSize", &self.cbSize)
            .field("uCausality", &self.uCausality)
            .field("dwServerPid", &self.dwServerPid)
            .field("iMethod", &self.iMethod)
            .field("pObject", &self.pObject)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SChannelHookCallInfo {
    fn eq(&self, other: &Self) -> bool {
        self.iid == other.iid
            && self.cbSize == other.cbSize
            && self.uCausality == other.uCausality
            && self.dwServerPid == other.dwServerPid
            && self.iMethod == other.iMethod
            && self.pObject == other.pObject
    }
}
impl ::core::cmp::Eq for SChannelHookCallInfo {}
impl FromIntoMemory for SChannelHookCallInfo {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_iid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_cbSize = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_uCausality = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[20..20 + 16]);
        let f_dwServerPid = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_iMethod = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_pObject =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        Self {
            iid: f_iid,
            cbSize: f_cbSize,
            uCausality: f_uCausality,
            dwServerPid: f_dwServerPid,
            iMethod: f_iMethod,
            pObject: f_pObject,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.iid, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.cbSize, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.uCausality, &mut into[20..20 + 16]);
        FromIntoMemory::into_bytes(self.dwServerPid, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.iMethod, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.pObject, &mut into[44..44 + 4]);
    }
    fn size() -> usize {
        48
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SERVERCALL(pub i32);
pub const SERVERCALL_ISHANDLED: SERVERCALL = SERVERCALL(0i32);
pub const SERVERCALL_REJECTED: SERVERCALL = SERVERCALL(1i32);
pub const SERVERCALL_RETRYLATER: SERVERCALL = SERVERCALL(2i32);
impl ::core::marker::Copy for SERVERCALL {}
impl ::core::clone::Clone for SERVERCALL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SERVERCALL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SERVERCALL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVERCALL").field(&self.0).finish()
    }
}
impl FromIntoMemory for SERVERCALL {
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
pub struct SHORT_SIZEDARR {
    pub clSize: u32,
    pub pData: MutPtr<u16>,
}
impl ::core::marker::Copy for SHORT_SIZEDARR {}
impl ::core::clone::Clone for SHORT_SIZEDARR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SHORT_SIZEDARR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHORT_SIZEDARR")
            .field("clSize", &self.clSize)
            .field("pData", &self.pData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SHORT_SIZEDARR {
    fn eq(&self, other: &Self) -> bool {
        self.clSize == other.clSize && self.pData == other.pData
    }
}
impl ::core::cmp::Eq for SHORT_SIZEDARR {}
impl FromIntoMemory for SHORT_SIZEDARR {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_clSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_pData = <MutPtr<u16> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            clSize: f_clSize,
            pData: f_pData,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.clSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.pData, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct SOLE_AUTHENTICATION_INFO {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pAuthInfo: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for SOLE_AUTHENTICATION_INFO {}
impl ::core::clone::Clone for SOLE_AUTHENTICATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOLE_AUTHENTICATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOLE_AUTHENTICATION_INFO")
            .field("dwAuthnSvc", &self.dwAuthnSvc)
            .field("dwAuthzSvc", &self.dwAuthzSvc)
            .field("pAuthInfo", &self.pAuthInfo)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SOLE_AUTHENTICATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwAuthnSvc == other.dwAuthnSvc
            && self.dwAuthzSvc == other.dwAuthzSvc
            && self.pAuthInfo == other.pAuthInfo
    }
}
impl ::core::cmp::Eq for SOLE_AUTHENTICATION_INFO {}
impl FromIntoMemory for SOLE_AUTHENTICATION_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_dwAuthnSvc = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwAuthzSvc = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_pAuthInfo =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            dwAuthnSvc: f_dwAuthnSvc,
            dwAuthzSvc: f_dwAuthzSvc,
            pAuthInfo: f_pAuthInfo,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.dwAuthnSvc, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwAuthzSvc, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.pAuthInfo, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct SOLE_AUTHENTICATION_LIST {
    pub cAuthInfo: u32,
    pub aAuthInfo: MutPtr<SOLE_AUTHENTICATION_INFO>,
}
impl ::core::marker::Copy for SOLE_AUTHENTICATION_LIST {}
impl ::core::clone::Clone for SOLE_AUTHENTICATION_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOLE_AUTHENTICATION_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOLE_AUTHENTICATION_LIST")
            .field("cAuthInfo", &self.cAuthInfo)
            .field("aAuthInfo", &self.aAuthInfo)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SOLE_AUTHENTICATION_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.cAuthInfo == other.cAuthInfo && self.aAuthInfo == other.aAuthInfo
    }
}
impl ::core::cmp::Eq for SOLE_AUTHENTICATION_LIST {}
impl FromIntoMemory for SOLE_AUTHENTICATION_LIST {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_cAuthInfo = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_aAuthInfo =
            <MutPtr<SOLE_AUTHENTICATION_INFO> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            cAuthInfo: f_cAuthInfo,
            aAuthInfo: f_aAuthInfo,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.cAuthInfo, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.aAuthInfo, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct SOLE_AUTHENTICATION_SERVICE {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pPrincipalName: PWSTR,
    pub hr: crate::core::HRESULT,
}
impl ::core::marker::Copy for SOLE_AUTHENTICATION_SERVICE {}
impl ::core::clone::Clone for SOLE_AUTHENTICATION_SERVICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOLE_AUTHENTICATION_SERVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOLE_AUTHENTICATION_SERVICE")
            .field("dwAuthnSvc", &self.dwAuthnSvc)
            .field("dwAuthzSvc", &self.dwAuthzSvc)
            .field("pPrincipalName", &self.pPrincipalName)
            .field("hr", &self.hr)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SOLE_AUTHENTICATION_SERVICE {
    fn eq(&self, other: &Self) -> bool {
        self.dwAuthnSvc == other.dwAuthnSvc
            && self.dwAuthzSvc == other.dwAuthzSvc
            && self.pPrincipalName == other.pPrincipalName
            && self.hr == other.hr
    }
}
impl ::core::cmp::Eq for SOLE_AUTHENTICATION_SERVICE {}
impl FromIntoMemory for SOLE_AUTHENTICATION_SERVICE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dwAuthnSvc = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwAuthzSvc = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_pPrincipalName = <PWSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_hr = <crate::core::HRESULT as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            dwAuthnSvc: f_dwAuthnSvc,
            dwAuthzSvc: f_dwAuthzSvc,
            pPrincipalName: f_pPrincipalName,
            hr: f_hr,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dwAuthnSvc, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwAuthzSvc, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.pPrincipalName, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.hr, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct STATDATA {
    pub formatetc: FORMATETC,
    pub advf: u32,
    pub pAdvSink: IAdviseSink,
    pub dwConnection: u32,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for STATDATA {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for STATDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for STATDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STATDATA")
            .field("formatetc", &self.formatetc)
            .field("advf", &self.advf)
            .field("pAdvSink", &self.pAdvSink)
            .field("dwConnection", &self.dwConnection)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for STATDATA {
    fn eq(&self, other: &Self) -> bool {
        self.formatetc == other.formatetc
            && self.advf == other.advf
            && self.pAdvSink == other.pAdvSink
            && self.dwConnection == other.dwConnection
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for STATDATA {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for STATDATA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_formatetc = <FORMATETC as FromIntoMemory>::from_bytes(&from[0..0 + 20]);
        let f_advf = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_pAdvSink = <IAdviseSink as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_dwConnection = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            formatetc: f_formatetc,
            advf: f_advf,
            pAdvSink: f_pAdvSink,
            dwConnection: f_dwConnection,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.formatetc, &mut into[0..0 + 20]);
        FromIntoMemory::into_bytes(self.advf, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.pAdvSink, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.dwConnection, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
pub struct STATSTG {
    pub pwcsName: PWSTR,
    pub r#type: u32,
    pub cbSize: u64,
    pub mtime: super::super::Foundation::FILETIME,
    pub ctime: super::super::Foundation::FILETIME,
    pub atime: super::super::Foundation::FILETIME,
    pub grfMode: u32,
    pub grfLocksSupported: u32,
    pub clsid: crate::core::GUID,
    pub grfStateBits: u32,
    pub reserved: u32,
}
impl ::core::marker::Copy for STATSTG {}
impl ::core::clone::Clone for STATSTG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STATSTG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STATSTG")
            .field("pwcsName", &self.pwcsName)
            .field("type", &self.r#type)
            .field("cbSize", &self.cbSize)
            .field("mtime", &self.mtime)
            .field("ctime", &self.ctime)
            .field("atime", &self.atime)
            .field("grfMode", &self.grfMode)
            .field("grfLocksSupported", &self.grfLocksSupported)
            .field("clsid", &self.clsid)
            .field("grfStateBits", &self.grfStateBits)
            .field("reserved", &self.reserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for STATSTG {
    fn eq(&self, other: &Self) -> bool {
        self.pwcsName == other.pwcsName
            && self.r#type == other.r#type
            && self.cbSize == other.cbSize
            && self.mtime == other.mtime
            && self.ctime == other.ctime
            && self.atime == other.atime
            && self.grfMode == other.grfMode
            && self.grfLocksSupported == other.grfLocksSupported
            && self.clsid == other.clsid
            && self.grfStateBits == other.grfStateBits
            && self.reserved == other.reserved
    }
}
impl ::core::cmp::Eq for STATSTG {}
impl FromIntoMemory for STATSTG {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 72);
        let f_pwcsName = <PWSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_type = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_cbSize = <u64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_mtime =
            <super::super::Foundation::FILETIME as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_ctime =
            <super::super::Foundation::FILETIME as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        let f_atime =
            <super::super::Foundation::FILETIME as FromIntoMemory>::from_bytes(&from[32..32 + 8]);
        let f_grfMode = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_grfLocksSupported = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_clsid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[48..48 + 16]);
        let f_grfStateBits = <u32 as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        let f_reserved = <u32 as FromIntoMemory>::from_bytes(&from[68..68 + 4]);
        Self {
            pwcsName: f_pwcsName,
            r#type: f_type,
            cbSize: f_cbSize,
            mtime: f_mtime,
            ctime: f_ctime,
            atime: f_atime,
            grfMode: f_grfMode,
            grfLocksSupported: f_grfLocksSupported,
            clsid: f_clsid,
            grfStateBits: f_grfStateBits,
            reserved: f_reserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 72);
        FromIntoMemory::into_bytes(self.pwcsName, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.r#type, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.cbSize, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.mtime, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.ctime, &mut into[24..24 + 8]);
        FromIntoMemory::into_bytes(self.atime, &mut into[32..32 + 8]);
        FromIntoMemory::into_bytes(self.grfMode, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.grfLocksSupported, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.clsid, &mut into[48..48 + 16]);
        FromIntoMemory::into_bytes(self.grfStateBits, &mut into[64..64 + 4]);
        FromIntoMemory::into_bytes(self.reserved, &mut into[68..68 + 4]);
    }
    fn size() -> usize {
        72
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct STGMEDIUM {
    pub tymed: u32,
    pub Anonymous: STGMEDIUM_0,
    pub pUnkForRelease: crate::core::IUnknown,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for STGMEDIUM {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for STGMEDIUM {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for STGMEDIUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STGMEDIUM")
            .field("tymed", &self.tymed)
            .field("Anonymous", &self.Anonymous)
            .field("pUnkForRelease", &self.pUnkForRelease)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for STGMEDIUM {
    fn eq(&self, other: &Self) -> bool {
        self.tymed == other.tymed
            && self.Anonymous == other.Anonymous
            && self.pUnkForRelease == other.pUnkForRelease
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for STGMEDIUM {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for STGMEDIUM {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_tymed = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Anonymous = <STGMEDIUM_0 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_pUnkForRelease =
            <crate::core::IUnknown as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            tymed: f_tymed,
            Anonymous: f_Anonymous,
            pUnkForRelease: f_pUnkForRelease,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.tymed, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.pUnkForRelease, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct STGMEDIUM_0 {
    data: [u8; 4],
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::default::Default for STGMEDIUM_0 {
    fn default() -> Self {
        Self { data: [0u8; 4] }
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for STGMEDIUM_0 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for STGMEDIUM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for STGMEDIUM_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STGMEDIUM_0")
            .field("data", &self.data)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for STGMEDIUM_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for STGMEDIUM_0 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for STGMEDIUM_0 {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STGTY(pub i32);
pub const STGTY_STORAGE: STGTY = STGTY(1i32);
pub const STGTY_STREAM: STGTY = STGTY(2i32);
pub const STGTY_LOCKBYTES: STGTY = STGTY(3i32);
pub const STGTY_PROPERTY: STGTY = STGTY(4i32);
impl ::core::marker::Copy for STGTY {}
impl ::core::clone::Clone for STGTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STGTY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STGTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STGTY").field(&self.0).finish()
    }
}
impl FromIntoMemory for STGTY {
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
pub const STGTY_REPEAT: i32 = 256i32;
pub const STG_LAYOUT_INTERLEAVED: i32 = 1i32;
pub const STG_LAYOUT_SEQUENTIAL: i32 = 0i32;
pub const STG_TOEND: i32 = -1i32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STREAM_SEEK(pub u32);
pub const STREAM_SEEK_SET: STREAM_SEEK = STREAM_SEEK(0u32);
pub const STREAM_SEEK_CUR: STREAM_SEEK = STREAM_SEEK(1u32);
pub const STREAM_SEEK_END: STREAM_SEEK = STREAM_SEEK(2u32);
impl ::core::marker::Copy for STREAM_SEEK {}
impl ::core::clone::Clone for STREAM_SEEK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STREAM_SEEK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STREAM_SEEK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STREAM_SEEK").field(&self.0).finish()
    }
}
impl FromIntoMemory for STREAM_SEEK {
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
pub struct SYSKIND(pub i32);
pub const SYS_WIN16: SYSKIND = SYSKIND(0i32);
pub const SYS_WIN32: SYSKIND = SYSKIND(1i32);
pub const SYS_MAC: SYSKIND = SYSKIND(2i32);
pub const SYS_WIN64: SYSKIND = SYSKIND(3i32);
impl ::core::marker::Copy for SYSKIND {}
impl ::core::clone::Clone for SYSKIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYSKIND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYSKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSKIND").field(&self.0).finish()
    }
}
impl FromIntoMemory for SYSKIND {
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
pub struct ShutdownType(pub i32);
pub const IdleShutdown: ShutdownType = ShutdownType(0i32);
pub const ForcedShutdown: ShutdownType = ShutdownType(1i32);
impl ::core::marker::Copy for ShutdownType {}
impl ::core::clone::Clone for ShutdownType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ShutdownType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ShutdownType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShutdownType").field(&self.0).finish()
    }
}
impl FromIntoMemory for ShutdownType {
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
pub struct StorageLayout {
    pub LayoutType: u32,
    pub pwcsElementName: PWSTR,
    pub cOffset: i64,
    pub cBytes: i64,
}
impl ::core::marker::Copy for StorageLayout {}
impl ::core::clone::Clone for StorageLayout {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for StorageLayout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("StorageLayout")
            .field("LayoutType", &self.LayoutType)
            .field("pwcsElementName", &self.pwcsElementName)
            .field("cOffset", &self.cOffset)
            .field("cBytes", &self.cBytes)
            .finish()
    }
}
impl ::core::cmp::PartialEq for StorageLayout {
    fn eq(&self, other: &Self) -> bool {
        self.LayoutType == other.LayoutType
            && self.pwcsElementName == other.pwcsElementName
            && self.cOffset == other.cOffset
            && self.cBytes == other.cBytes
    }
}
impl ::core::cmp::Eq for StorageLayout {}
impl FromIntoMemory for StorageLayout {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_LayoutType = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_pwcsElementName = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_cOffset = <i64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_cBytes = <i64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        Self {
            LayoutType: f_LayoutType,
            pwcsElementName: f_pwcsElementName,
            cOffset: f_cOffset,
            cBytes: f_cBytes,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.LayoutType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.pwcsElementName, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.cOffset, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.cBytes, &mut into[16..16 + 8]);
    }
    fn size() -> usize {
        24
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct THDTYPE(pub i32);
pub const THDTYPE_BLOCKMESSAGES: THDTYPE = THDTYPE(0i32);
pub const THDTYPE_PROCESSMESSAGES: THDTYPE = THDTYPE(1i32);
impl ::core::marker::Copy for THDTYPE {}
impl ::core::clone::Clone for THDTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for THDTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for THDTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THDTYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for THDTYPE {
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
pub struct TLIBATTR {
    pub guid: crate::core::GUID,
    pub lcid: u32,
    pub syskind: SYSKIND,
    pub wMajorVerNum: u16,
    pub wMinorVerNum: u16,
    pub wLibFlags: u16,
}
impl ::core::marker::Copy for TLIBATTR {}
impl ::core::clone::Clone for TLIBATTR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TLIBATTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TLIBATTR")
            .field("guid", &self.guid)
            .field("lcid", &self.lcid)
            .field("syskind", &self.syskind)
            .field("wMajorVerNum", &self.wMajorVerNum)
            .field("wMinorVerNum", &self.wMinorVerNum)
            .field("wLibFlags", &self.wLibFlags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TLIBATTR {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid
            && self.lcid == other.lcid
            && self.syskind == other.syskind
            && self.wMajorVerNum == other.wMajorVerNum
            && self.wMinorVerNum == other.wMinorVerNum
            && self.wLibFlags == other.wLibFlags
    }
}
impl ::core::cmp::Eq for TLIBATTR {}
impl FromIntoMemory for TLIBATTR {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_guid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_lcid = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_syskind = <SYSKIND as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_wMajorVerNum = <u16 as FromIntoMemory>::from_bytes(&from[24..24 + 2]);
        let f_wMinorVerNum = <u16 as FromIntoMemory>::from_bytes(&from[26..26 + 2]);
        let f_wLibFlags = <u16 as FromIntoMemory>::from_bytes(&from[28..28 + 2]);
        Self {
            guid: f_guid,
            lcid: f_lcid,
            syskind: f_syskind,
            wMajorVerNum: f_wMajorVerNum,
            wMinorVerNum: f_wMinorVerNum,
            wLibFlags: f_wLibFlags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.guid, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.lcid, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.syskind, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.wMajorVerNum, &mut into[24..24 + 2]);
        FromIntoMemory::into_bytes(self.wMinorVerNum, &mut into[26..26 + 2]);
        FromIntoMemory::into_bytes(self.wLibFlags, &mut into[28..28 + 2]);
    }
    fn size() -> usize {
        32
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TYMED(pub i32);
pub const TYMED_HGLOBAL: TYMED = TYMED(1i32);
pub const TYMED_FILE: TYMED = TYMED(2i32);
pub const TYMED_ISTREAM: TYMED = TYMED(4i32);
pub const TYMED_ISTORAGE: TYMED = TYMED(8i32);
pub const TYMED_GDI: TYMED = TYMED(16i32);
pub const TYMED_MFPICT: TYMED = TYMED(32i32);
pub const TYMED_ENHMF: TYMED = TYMED(64i32);
pub const TYMED_NULL: TYMED = TYMED(0i32);
impl ::core::marker::Copy for TYMED {}
impl ::core::clone::Clone for TYMED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TYMED {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TYMED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TYMED").field(&self.0).finish()
    }
}
impl FromIntoMemory for TYMED {
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
#[doc = "*Required namespaces: 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct TYPEATTR {
    pub guid: crate::core::GUID,
    pub lcid: u32,
    pub dwReserved: u32,
    pub memidConstructor: i32,
    pub memidDestructor: i32,
    pub lpstrSchema: PWSTR,
    pub cbSizeInstance: u32,
    pub typekind: TYPEKIND,
    pub cFuncs: u16,
    pub cVars: u16,
    pub cImplTypes: u16,
    pub cbSizeVft: u16,
    pub cbAlignment: u16,
    pub wTypeFlags: u16,
    pub wMajorVerNum: u16,
    pub wMinorVerNum: u16,
    pub tdescAlias: TYPEDESC,
    pub idldescType: IDLDESC,
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for TYPEATTR {}
#[doc = "*Required namespaces: 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for TYPEATTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for TYPEATTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TYPEATTR")
            .field("guid", &self.guid)
            .field("lcid", &self.lcid)
            .field("dwReserved", &self.dwReserved)
            .field("memidConstructor", &self.memidConstructor)
            .field("memidDestructor", &self.memidDestructor)
            .field("lpstrSchema", &self.lpstrSchema)
            .field("cbSizeInstance", &self.cbSizeInstance)
            .field("typekind", &self.typekind)
            .field("cFuncs", &self.cFuncs)
            .field("cVars", &self.cVars)
            .field("cImplTypes", &self.cImplTypes)
            .field("cbSizeVft", &self.cbSizeVft)
            .field("cbAlignment", &self.cbAlignment)
            .field("wTypeFlags", &self.wTypeFlags)
            .field("wMajorVerNum", &self.wMajorVerNum)
            .field("wMinorVerNum", &self.wMinorVerNum)
            .field("tdescAlias", &self.tdescAlias)
            .field("idldescType", &self.idldescType)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for TYPEATTR {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid
            && self.lcid == other.lcid
            && self.dwReserved == other.dwReserved
            && self.memidConstructor == other.memidConstructor
            && self.memidDestructor == other.memidDestructor
            && self.lpstrSchema == other.lpstrSchema
            && self.cbSizeInstance == other.cbSizeInstance
            && self.typekind == other.typekind
            && self.cFuncs == other.cFuncs
            && self.cVars == other.cVars
            && self.cImplTypes == other.cImplTypes
            && self.cbSizeVft == other.cbSizeVft
            && self.cbAlignment == other.cbAlignment
            && self.wTypeFlags == other.wTypeFlags
            && self.wMajorVerNum == other.wMajorVerNum
            && self.wMinorVerNum == other.wMinorVerNum
            && self.tdescAlias == other.tdescAlias
            && self.idldescType == other.idldescType
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for TYPEATTR {}
#[doc = "*Required namespaces: 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for TYPEATTR {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 76);
        let f_guid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_lcid = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwReserved = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_memidConstructor = <i32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_memidDestructor = <i32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_lpstrSchema = <PWSTR as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_cbSizeInstance = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_typekind = <TYPEKIND as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_cFuncs = <u16 as FromIntoMemory>::from_bytes(&from[44..44 + 2]);
        let f_cVars = <u16 as FromIntoMemory>::from_bytes(&from[46..46 + 2]);
        let f_cImplTypes = <u16 as FromIntoMemory>::from_bytes(&from[48..48 + 2]);
        let f_cbSizeVft = <u16 as FromIntoMemory>::from_bytes(&from[50..50 + 2]);
        let f_cbAlignment = <u16 as FromIntoMemory>::from_bytes(&from[52..52 + 2]);
        let f_wTypeFlags = <u16 as FromIntoMemory>::from_bytes(&from[54..54 + 2]);
        let f_wMajorVerNum = <u16 as FromIntoMemory>::from_bytes(&from[56..56 + 2]);
        let f_wMinorVerNum = <u16 as FromIntoMemory>::from_bytes(&from[58..58 + 2]);
        let f_tdescAlias = <TYPEDESC as FromIntoMemory>::from_bytes(&from[60..60 + 8]);
        let f_idldescType = <IDLDESC as FromIntoMemory>::from_bytes(&from[68..68 + 8]);
        Self {
            guid: f_guid,
            lcid: f_lcid,
            dwReserved: f_dwReserved,
            memidConstructor: f_memidConstructor,
            memidDestructor: f_memidDestructor,
            lpstrSchema: f_lpstrSchema,
            cbSizeInstance: f_cbSizeInstance,
            typekind: f_typekind,
            cFuncs: f_cFuncs,
            cVars: f_cVars,
            cImplTypes: f_cImplTypes,
            cbSizeVft: f_cbSizeVft,
            cbAlignment: f_cbAlignment,
            wTypeFlags: f_wTypeFlags,
            wMajorVerNum: f_wMajorVerNum,
            wMinorVerNum: f_wMinorVerNum,
            tdescAlias: f_tdescAlias,
            idldescType: f_idldescType,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 76);
        FromIntoMemory::into_bytes(self.guid, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.lcid, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwReserved, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.memidConstructor, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.memidDestructor, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.lpstrSchema, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.cbSizeInstance, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.typekind, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.cFuncs, &mut into[44..44 + 2]);
        FromIntoMemory::into_bytes(self.cVars, &mut into[46..46 + 2]);
        FromIntoMemory::into_bytes(self.cImplTypes, &mut into[48..48 + 2]);
        FromIntoMemory::into_bytes(self.cbSizeVft, &mut into[50..50 + 2]);
        FromIntoMemory::into_bytes(self.cbAlignment, &mut into[52..52 + 2]);
        FromIntoMemory::into_bytes(self.wTypeFlags, &mut into[54..54 + 2]);
        FromIntoMemory::into_bytes(self.wMajorVerNum, &mut into[56..56 + 2]);
        FromIntoMemory::into_bytes(self.wMinorVerNum, &mut into[58..58 + 2]);
        FromIntoMemory::into_bytes(self.tdescAlias, &mut into[60..60 + 8]);
        FromIntoMemory::into_bytes(self.idldescType, &mut into[68..68 + 8]);
    }
    fn size() -> usize {
        76
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct TYPEDESC {
    pub Anonymous: TYPEDESC_0,
    pub vt: u16,
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for TYPEDESC {}
#[doc = "*Required namespaces: 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for TYPEDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for TYPEDESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TYPEDESC")
            .field("Anonymous", &self.Anonymous)
            .field("vt", &self.vt)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for TYPEDESC {
    fn eq(&self, other: &Self) -> bool {
        self.Anonymous == other.Anonymous && self.vt == other.vt
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for TYPEDESC {}
#[doc = "*Required namespaces: 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for TYPEDESC {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Anonymous = <TYPEDESC_0 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_vt = <u16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        Self {
            Anonymous: f_Anonymous,
            vt: f_vt,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.vt, &mut into[4..4 + 2]);
    }
    fn size() -> usize {
        8
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct TYPEDESC_0 {
    data: [u8; 4],
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::default::Default for TYPEDESC_0 {
    fn default() -> Self {
        Self { data: [0u8; 4] }
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for TYPEDESC_0 {}
#[doc = "*Required namespaces: 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for TYPEDESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for TYPEDESC_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TYPEDESC_0")
            .field("data", &self.data)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for TYPEDESC_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for TYPEDESC_0 {}
#[doc = "*Required namespaces: 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for TYPEDESC_0 {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TYPEKIND(pub i32);
pub const TKIND_ENUM: TYPEKIND = TYPEKIND(0i32);
pub const TKIND_RECORD: TYPEKIND = TYPEKIND(1i32);
pub const TKIND_MODULE: TYPEKIND = TYPEKIND(2i32);
pub const TKIND_INTERFACE: TYPEKIND = TYPEKIND(3i32);
pub const TKIND_DISPATCH: TYPEKIND = TYPEKIND(4i32);
pub const TKIND_COCLASS: TYPEKIND = TYPEKIND(5i32);
pub const TKIND_ALIAS: TYPEKIND = TYPEKIND(6i32);
pub const TKIND_UNION: TYPEKIND = TYPEKIND(7i32);
pub const TKIND_MAX: TYPEKIND = TYPEKIND(8i32);
impl ::core::marker::Copy for TYPEKIND {}
impl ::core::clone::Clone for TYPEKIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TYPEKIND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TYPEKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TYPEKIND").field(&self.0).finish()
    }
}
impl FromIntoMemory for TYPEKIND {
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
pub struct TYSPEC(pub i32);
pub const TYSPEC_CLSID: TYSPEC = TYSPEC(0i32);
pub const TYSPEC_FILEEXT: TYSPEC = TYSPEC(1i32);
pub const TYSPEC_MIMETYPE: TYSPEC = TYSPEC(2i32);
pub const TYSPEC_FILENAME: TYSPEC = TYSPEC(3i32);
pub const TYSPEC_PROGID: TYSPEC = TYSPEC(4i32);
pub const TYSPEC_PACKAGENAME: TYSPEC = TYSPEC(5i32);
pub const TYSPEC_OBJECTID: TYSPEC = TYSPEC(6i32);
impl ::core::marker::Copy for TYSPEC {}
impl ::core::clone::Clone for TYSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TYSPEC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TYSPEC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TYSPEC").field(&self.0).finish()
    }
}
impl FromIntoMemory for TYSPEC {
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
pub struct URI_CREATE_FLAGS(pub u32);
pub const Uri_CREATE_ALLOW_RELATIVE: URI_CREATE_FLAGS = URI_CREATE_FLAGS(1u32);
pub const Uri_CREATE_ALLOW_IMPLICIT_WILDCARD_SCHEME: URI_CREATE_FLAGS = URI_CREATE_FLAGS(2u32);
pub const Uri_CREATE_ALLOW_IMPLICIT_FILE_SCHEME: URI_CREATE_FLAGS = URI_CREATE_FLAGS(4u32);
pub const Uri_CREATE_NOFRAG: URI_CREATE_FLAGS = URI_CREATE_FLAGS(8u32);
pub const Uri_CREATE_NO_CANONICALIZE: URI_CREATE_FLAGS = URI_CREATE_FLAGS(16u32);
pub const Uri_CREATE_CANONICALIZE: URI_CREATE_FLAGS = URI_CREATE_FLAGS(256u32);
pub const Uri_CREATE_FILE_USE_DOS_PATH: URI_CREATE_FLAGS = URI_CREATE_FLAGS(32u32);
pub const Uri_CREATE_DECODE_EXTRA_INFO: URI_CREATE_FLAGS = URI_CREATE_FLAGS(64u32);
pub const Uri_CREATE_NO_DECODE_EXTRA_INFO: URI_CREATE_FLAGS = URI_CREATE_FLAGS(128u32);
pub const Uri_CREATE_CRACK_UNKNOWN_SCHEMES: URI_CREATE_FLAGS = URI_CREATE_FLAGS(512u32);
pub const Uri_CREATE_NO_CRACK_UNKNOWN_SCHEMES: URI_CREATE_FLAGS = URI_CREATE_FLAGS(1024u32);
pub const Uri_CREATE_PRE_PROCESS_HTML_URI: URI_CREATE_FLAGS = URI_CREATE_FLAGS(2048u32);
pub const Uri_CREATE_NO_PRE_PROCESS_HTML_URI: URI_CREATE_FLAGS = URI_CREATE_FLAGS(4096u32);
pub const Uri_CREATE_IE_SETTINGS: URI_CREATE_FLAGS = URI_CREATE_FLAGS(8192u32);
pub const Uri_CREATE_NO_IE_SETTINGS: URI_CREATE_FLAGS = URI_CREATE_FLAGS(16384u32);
pub const Uri_CREATE_NO_ENCODE_FORBIDDEN_CHARACTERS: URI_CREATE_FLAGS = URI_CREATE_FLAGS(32768u32);
pub const Uri_CREATE_NORMALIZE_INTL_CHARACTERS: URI_CREATE_FLAGS = URI_CREATE_FLAGS(65536u32);
pub const Uri_CREATE_CANONICALIZE_ABSOLUTE: URI_CREATE_FLAGS = URI_CREATE_FLAGS(131072u32);
impl ::core::marker::Copy for URI_CREATE_FLAGS {}
impl ::core::clone::Clone for URI_CREATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for URI_CREATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for URI_CREATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("URI_CREATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for URI_CREATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for URI_CREATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for URI_CREATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for URI_CREATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for URI_CREATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for URI_CREATE_FLAGS {
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
pub struct Uri_PROPERTY(pub i32);
pub const Uri_PROPERTY_ABSOLUTE_URI: Uri_PROPERTY = Uri_PROPERTY(0i32);
pub const Uri_PROPERTY_STRING_START: Uri_PROPERTY = Uri_PROPERTY(0i32);
pub const Uri_PROPERTY_AUTHORITY: Uri_PROPERTY = Uri_PROPERTY(1i32);
pub const Uri_PROPERTY_DISPLAY_URI: Uri_PROPERTY = Uri_PROPERTY(2i32);
pub const Uri_PROPERTY_DOMAIN: Uri_PROPERTY = Uri_PROPERTY(3i32);
pub const Uri_PROPERTY_EXTENSION: Uri_PROPERTY = Uri_PROPERTY(4i32);
pub const Uri_PROPERTY_FRAGMENT: Uri_PROPERTY = Uri_PROPERTY(5i32);
pub const Uri_PROPERTY_HOST: Uri_PROPERTY = Uri_PROPERTY(6i32);
pub const Uri_PROPERTY_PASSWORD: Uri_PROPERTY = Uri_PROPERTY(7i32);
pub const Uri_PROPERTY_PATH: Uri_PROPERTY = Uri_PROPERTY(8i32);
pub const Uri_PROPERTY_PATH_AND_QUERY: Uri_PROPERTY = Uri_PROPERTY(9i32);
pub const Uri_PROPERTY_QUERY: Uri_PROPERTY = Uri_PROPERTY(10i32);
pub const Uri_PROPERTY_RAW_URI: Uri_PROPERTY = Uri_PROPERTY(11i32);
pub const Uri_PROPERTY_SCHEME_NAME: Uri_PROPERTY = Uri_PROPERTY(12i32);
pub const Uri_PROPERTY_USER_INFO: Uri_PROPERTY = Uri_PROPERTY(13i32);
pub const Uri_PROPERTY_USER_NAME: Uri_PROPERTY = Uri_PROPERTY(14i32);
pub const Uri_PROPERTY_STRING_LAST: Uri_PROPERTY = Uri_PROPERTY(14i32);
pub const Uri_PROPERTY_HOST_TYPE: Uri_PROPERTY = Uri_PROPERTY(15i32);
pub const Uri_PROPERTY_DWORD_START: Uri_PROPERTY = Uri_PROPERTY(15i32);
pub const Uri_PROPERTY_PORT: Uri_PROPERTY = Uri_PROPERTY(16i32);
pub const Uri_PROPERTY_SCHEME: Uri_PROPERTY = Uri_PROPERTY(17i32);
pub const Uri_PROPERTY_ZONE: Uri_PROPERTY = Uri_PROPERTY(18i32);
pub const Uri_PROPERTY_DWORD_LAST: Uri_PROPERTY = Uri_PROPERTY(18i32);
impl ::core::marker::Copy for Uri_PROPERTY {}
impl ::core::clone::Clone for Uri_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Uri_PROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for Uri_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Uri_PROPERTY").field(&self.0).finish()
    }
}
impl FromIntoMemory for Uri_PROPERTY {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct VARDESC {
    pub memid: i32,
    pub lpstrSchema: PWSTR,
    pub Anonymous: VARDESC_0,
    pub elemdescVar: ELEMDESC,
    pub wVarFlags: u16,
    pub varkind: VARKIND,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for VARDESC {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for VARDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for VARDESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VARDESC")
            .field("memid", &self.memid)
            .field("lpstrSchema", &self.lpstrSchema)
            .field("Anonymous", &self.Anonymous)
            .field("elemdescVar", &self.elemdescVar)
            .field("wVarFlags", &self.wVarFlags)
            .field("varkind", &self.varkind)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for VARDESC {
    fn eq(&self, other: &Self) -> bool {
        self.memid == other.memid
            && self.lpstrSchema == other.lpstrSchema
            && self.Anonymous == other.Anonymous
            && self.elemdescVar == other.elemdescVar
            && self.wVarFlags == other.wVarFlags
            && self.varkind == other.varkind
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for VARDESC {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for VARDESC {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 36);
        let f_memid = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpstrSchema = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Anonymous = <VARDESC_0 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_elemdescVar = <ELEMDESC as FromIntoMemory>::from_bytes(&from[12..12 + 16]);
        let f_wVarFlags = <u16 as FromIntoMemory>::from_bytes(&from[28..28 + 2]);
        let f_varkind = <VARKIND as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        Self {
            memid: f_memid,
            lpstrSchema: f_lpstrSchema,
            Anonymous: f_Anonymous,
            elemdescVar: f_elemdescVar,
            wVarFlags: f_wVarFlags,
            varkind: f_varkind,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 36);
        FromIntoMemory::into_bytes(self.memid, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpstrSchema, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.elemdescVar, &mut into[12..12 + 16]);
        FromIntoMemory::into_bytes(self.wVarFlags, &mut into[28..28 + 2]);
        FromIntoMemory::into_bytes(self.varkind, &mut into[32..32 + 4]);
    }
    fn size() -> usize {
        36
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct VARDESC_0 {
    data: [u8; 4],
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::default::Default for VARDESC_0 {
    fn default() -> Self {
        Self { data: [0u8; 4] }
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for VARDESC_0 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for VARDESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for VARDESC_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VARDESC_0")
            .field("data", &self.data)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for VARDESC_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for VARDESC_0 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for VARDESC_0 {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct VARIANT {
    pub Anonymous: VARIANT_0,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for VARIANT {
    fn clone(&self) -> Self {
        Self {
            Anonymous: self.Anonymous.clone(),
        }
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for VARIANT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VARIANT")
            .field("Anonymous", &self.Anonymous)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for VARIANT {
    fn eq(&self, other: &Self) -> bool {
        self.Anonymous == other.Anonymous
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for VARIANT {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for VARIANT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_Anonymous = <VARIANT_0 as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        Self {
            Anonymous: f_Anonymous,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[0..0 + 16]);
    }
    fn size() -> usize {
        16
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct VARIANT_0 {
    data: [u8; 16],
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::default::Default for VARIANT_0 {
    fn default() -> Self {
        Self { data: [0u8; 16] }
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for VARIANT_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for VARIANT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VARIANT_0")
            .field("data", &self.data)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for VARIANT_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for VARIANT_0 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for VARIANT_0 {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct VARIANT_0_0 {
    pub vt: u16,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: VARIANT_0_0_0,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for VARIANT_0_0 {
    fn clone(&self) -> Self {
        Self {
            vt: self.vt,
            wReserved1: self.wReserved1,
            wReserved2: self.wReserved2,
            wReserved3: self.wReserved3,
            Anonymous: self.Anonymous.clone(),
        }
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for VARIANT_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VARIANT_0_0")
            .field("vt", &self.vt)
            .field("wReserved1", &self.wReserved1)
            .field("wReserved2", &self.wReserved2)
            .field("wReserved3", &self.wReserved3)
            .field("Anonymous", &self.Anonymous)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for VARIANT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.vt == other.vt
            && self.wReserved1 == other.wReserved1
            && self.wReserved2 == other.wReserved2
            && self.wReserved3 == other.wReserved3
            && self.Anonymous == other.Anonymous
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for VARIANT_0_0 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for VARIANT_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_vt = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wReserved1 = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_wReserved2 = <u16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_wReserved3 = <u16 as FromIntoMemory>::from_bytes(&from[6..6 + 2]);
        let f_Anonymous = <VARIANT_0_0_0 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        Self {
            vt: f_vt,
            wReserved1: f_wReserved1,
            wReserved2: f_wReserved2,
            wReserved3: f_wReserved3,
            Anonymous: f_Anonymous,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.vt, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wReserved1, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.wReserved2, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.wReserved3, &mut into[6..6 + 2]);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[8..8 + 8]);
    }
    fn size() -> usize {
        16
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct VARIANT_0_0_0 {
    data: [u8; 8],
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::default::Default for VARIANT_0_0_0 {
    fn default() -> Self {
        Self { data: [0u8; 8] }
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for VARIANT_0_0_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for VARIANT_0_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VARIANT_0_0_0")
            .field("data", &self.data)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for VARIANT_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for VARIANT_0_0_0 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for VARIANT_0_0_0 {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct VARIANT_0_0_0_0 {
    pub pvRecord: MutPtr<::core::ffi::c_void>,
    pub pRecInfo: super::Ole::IRecordInfo,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for VARIANT_0_0_0_0 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for VARIANT_0_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for VARIANT_0_0_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VARIANT_0_0_0_0")
            .field("pvRecord", &self.pvRecord)
            .field("pRecInfo", &self.pRecInfo)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for VARIANT_0_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.pvRecord == other.pvRecord && self.pRecInfo == other.pRecInfo
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for VARIANT_0_0_0_0 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for VARIANT_0_0_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_pvRecord =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_pRecInfo = <super::Ole::IRecordInfo as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            pvRecord: f_pvRecord,
            pRecInfo: f_pRecInfo,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.pvRecord, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.pRecInfo, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VARKIND(pub i32);
pub const VAR_PERINSTANCE: VARKIND = VARKIND(0i32);
pub const VAR_STATIC: VARKIND = VARKIND(1i32);
pub const VAR_CONST: VARKIND = VARKIND(2i32);
pub const VAR_DISPATCH: VARKIND = VARKIND(3i32);
impl ::core::marker::Copy for VARKIND {}
impl ::core::clone::Clone for VARKIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VARKIND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VARKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VARKIND").field(&self.0).finish()
    }
}
impl FromIntoMemory for VARKIND {
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
pub struct WORD_BLOB {
    pub clSize: u32,
    pub asData: [u16; 1],
}
impl ::core::marker::Copy for WORD_BLOB {}
impl ::core::clone::Clone for WORD_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WORD_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WORD_BLOB")
            .field("clSize", &self.clSize)
            .field("asData", &self.asData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WORD_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.clSize == other.clSize && self.asData == other.asData
    }
}
impl ::core::cmp::Eq for WORD_BLOB {}
impl FromIntoMemory for WORD_BLOB {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_clSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_asData = <[u16; 1] as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        Self {
            clSize: f_clSize,
            asData: f_asData,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.clSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.asData, &mut into[4..4 + 2]);
    }
    fn size() -> usize {
        8
    }
}
pub const _CRT_INTERNAL_COMBASE_SYMBOL_PREFIX: &'static str = "_";
pub struct uCLSSPEC {
    pub tyspec: u32,
    pub tagged_union: uCLSSPEC_0,
}
impl ::core::marker::Copy for uCLSSPEC {}
impl ::core::clone::Clone for uCLSSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for uCLSSPEC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("uCLSSPEC")
            .field("tyspec", &self.tyspec)
            .field("tagged_union", &self.tagged_union)
            .finish()
    }
}
impl ::core::cmp::PartialEq for uCLSSPEC {
    fn eq(&self, other: &Self) -> bool {
        self.tyspec == other.tyspec && self.tagged_union == other.tagged_union
    }
}
impl ::core::cmp::Eq for uCLSSPEC {}
impl FromIntoMemory for uCLSSPEC {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 36);
        let f_tyspec = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_tagged_union = <uCLSSPEC_0 as FromIntoMemory>::from_bytes(&from[4..4 + 32]);
        Self {
            tyspec: f_tyspec,
            tagged_union: f_tagged_union,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 36);
        FromIntoMemory::into_bytes(self.tyspec, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.tagged_union, &mut into[4..4 + 32]);
    }
    fn size() -> usize {
        36
    }
}
pub struct uCLSSPEC_0 {
    data: [u8; 32],
}
impl ::core::default::Default for uCLSSPEC_0 {
    fn default() -> Self {
        Self { data: [0u8; 32] }
    }
}
impl ::core::marker::Copy for uCLSSPEC_0 {}
impl ::core::clone::Clone for uCLSSPEC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for uCLSSPEC_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("uCLSSPEC_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for uCLSSPEC_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for uCLSSPEC_0 {}
impl FromIntoMemory for uCLSSPEC_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 32];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        32
    }
}
pub struct uCLSSPEC_0_0 {
    pub pPackageName: PWSTR,
    pub PolicyId: crate::core::GUID,
}
impl ::core::marker::Copy for uCLSSPEC_0_0 {}
impl ::core::clone::Clone for uCLSSPEC_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for uCLSSPEC_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("uCLSSPEC_0_0")
            .field("pPackageName", &self.pPackageName)
            .field("PolicyId", &self.PolicyId)
            .finish()
    }
}
impl ::core::cmp::PartialEq for uCLSSPEC_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.pPackageName == other.pPackageName && self.PolicyId == other.PolicyId
    }
}
impl ::core::cmp::Eq for uCLSSPEC_0_0 {}
impl FromIntoMemory for uCLSSPEC_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_pPackageName = <PWSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_PolicyId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[4..4 + 16]);
        Self {
            pPackageName: f_pPackageName,
            PolicyId: f_PolicyId,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.pPackageName, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.PolicyId, &mut into[4..4 + 16]);
    }
    fn size() -> usize {
        20
    }
}
pub struct uCLSSPEC_0_1 {
    pub ObjectId: crate::core::GUID,
    pub PolicyId: crate::core::GUID,
}
impl ::core::marker::Copy for uCLSSPEC_0_1 {}
impl ::core::clone::Clone for uCLSSPEC_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for uCLSSPEC_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("uCLSSPEC_0_1")
            .field("ObjectId", &self.ObjectId)
            .field("PolicyId", &self.PolicyId)
            .finish()
    }
}
impl ::core::cmp::PartialEq for uCLSSPEC_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectId == other.ObjectId && self.PolicyId == other.PolicyId
    }
}
impl ::core::cmp::Eq for uCLSSPEC_0_1 {}
impl FromIntoMemory for uCLSSPEC_0_1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_ObjectId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_PolicyId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[16..16 + 16]);
        Self {
            ObjectId: f_ObjectId,
            PolicyId: f_PolicyId,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.ObjectId, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.PolicyId, &mut into[16..16 + 16]);
    }
    fn size() -> usize {
        32
    }
}
pub struct userFLAG_STGMEDIUM {
    pub ContextFlags: i32,
    pub fPassOwnership: i32,
    pub Stgmed: userSTGMEDIUM,
}
impl ::core::marker::Copy for userFLAG_STGMEDIUM {}
impl ::core::clone::Clone for userFLAG_STGMEDIUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for userFLAG_STGMEDIUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("userFLAG_STGMEDIUM")
            .field("ContextFlags", &self.ContextFlags)
            .field("fPassOwnership", &self.fPassOwnership)
            .field("Stgmed", &self.Stgmed)
            .finish()
    }
}
impl ::core::cmp::PartialEq for userFLAG_STGMEDIUM {
    fn eq(&self, other: &Self) -> bool {
        self.ContextFlags == other.ContextFlags
            && self.fPassOwnership == other.fPassOwnership
            && self.Stgmed == other.Stgmed
    }
}
impl ::core::cmp::Eq for userFLAG_STGMEDIUM {}
impl FromIntoMemory for userFLAG_STGMEDIUM {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_ContextFlags = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_fPassOwnership = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Stgmed = <userSTGMEDIUM as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            ContextFlags: f_ContextFlags,
            fPassOwnership: f_fPassOwnership,
            Stgmed: f_Stgmed,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.ContextFlags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.fPassOwnership, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Stgmed, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct userSTGMEDIUM {
    pub pUnkForRelease: crate::core::IUnknown,
}
impl ::core::marker::Copy for userSTGMEDIUM {}
impl ::core::clone::Clone for userSTGMEDIUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for userSTGMEDIUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("userSTGMEDIUM")
            .field("pUnkForRelease", &self.pUnkForRelease)
            .finish()
    }
}
impl ::core::cmp::PartialEq for userSTGMEDIUM {
    fn eq(&self, other: &Self) -> bool {
        self.pUnkForRelease == other.pUnkForRelease
    }
}
impl ::core::cmp::Eq for userSTGMEDIUM {}
impl FromIntoMemory for userSTGMEDIUM {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_pUnkForRelease =
            <crate::core::IUnknown as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            pUnkForRelease: f_pUnkForRelease,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.pUnkForRelease, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct userSTGMEDIUM_0 {
    pub tymed: u32,
    pub u: userSTGMEDIUM_0_0,
}
impl ::core::marker::Copy for userSTGMEDIUM_0 {}
impl ::core::clone::Clone for userSTGMEDIUM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for userSTGMEDIUM_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("userSTGMEDIUM_0")
            .field("tymed", &self.tymed)
            .field("u", &self.u)
            .finish()
    }
}
impl ::core::cmp::PartialEq for userSTGMEDIUM_0 {
    fn eq(&self, other: &Self) -> bool {
        self.tymed == other.tymed && self.u == other.u
    }
}
impl ::core::cmp::Eq for userSTGMEDIUM_0 {}
impl FromIntoMemory for userSTGMEDIUM_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_tymed = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_u = <userSTGMEDIUM_0_0 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            tymed: f_tymed,
            u: f_u,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.tymed, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.u, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct userSTGMEDIUM_0_0 {
    data: [u8; 4],
}
impl ::core::default::Default for userSTGMEDIUM_0_0 {
    fn default() -> Self {
        Self { data: [0u8; 4] }
    }
}
impl ::core::marker::Copy for userSTGMEDIUM_0_0 {}
impl ::core::clone::Clone for userSTGMEDIUM_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for userSTGMEDIUM_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("userSTGMEDIUM_0_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for userSTGMEDIUM_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for userSTGMEDIUM_0_0 {}
impl FromIntoMemory for userSTGMEDIUM_0_0 {
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
pub trait Api {
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn BindMoniker(
        &self,
        pmk: IMoniker,
        grf_opt: u32,
        iid_result: ConstPtr<crate::core::GUID>,
        ppv_result: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("BindMoniker")
    }
    fn CLSIDFromProgID(
        &self,
        lpsz_prog_id: PCWSTR,
        lpclsid: MutPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("CLSIDFromProgID")
    }
    fn CLSIDFromProgIDEx(
        &self,
        lpsz_prog_id: PCWSTR,
        lpclsid: MutPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("CLSIDFromProgIDEx")
    }
    fn CLSIDFromString(
        &self,
        lpsz: PCWSTR,
        pclsid: MutPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("CLSIDFromString")
    }
    fn CoAddRefServerProcess(&self) -> u32 {
        todo!("CoAddRefServerProcess")
    }
    fn CoAllowSetForegroundWindow(
        &self,
        p_unk: crate::core::IUnknown,
        lpv_reserved: ConstPtr<::core::ffi::c_void>,
    ) -> crate::core::HRESULT {
        todo!("CoAllowSetForegroundWindow")
    }
    fn CoAllowUnmarshalerCLSID(&self, clsid: ConstPtr<crate::core::GUID>) -> crate::core::HRESULT {
        todo!("CoAllowUnmarshalerCLSID")
    }
    fn CoBuildVersion(&self) -> u32 {
        todo!("CoBuildVersion")
    }
    fn CoCancelCall(&self, dw_thread_id: u32, ul_timeout: u32) -> crate::core::HRESULT {
        todo!("CoCancelCall")
    }
    fn CoCopyProxy(
        &self,
        p_proxy: crate::core::IUnknown,
        pp_copy: MutPtr<crate::core::IUnknown>,
    ) -> crate::core::HRESULT {
        todo!("CoCopyProxy")
    }
    fn CoCreateFreeThreadedMarshaler(
        &self,
        punk_outer: crate::core::IUnknown,
        ppunk_marshal: MutPtr<crate::core::IUnknown>,
    ) -> crate::core::HRESULT {
        todo!("CoCreateFreeThreadedMarshaler")
    }
    fn CoCreateGuid(&self, pguid: MutPtr<crate::core::GUID>) -> crate::core::HRESULT {
        todo!("CoCreateGuid")
    }
    fn CoCreateInstance(
        &self,
        rclsid: ConstPtr<crate::core::GUID>,
        p_unk_outer: crate::core::IUnknown,
        dw_cls_context: CLSCTX,
        riid: ConstPtr<crate::core::GUID>,
        ppv: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("CoCreateInstance")
    }
    fn CoCreateInstanceEx(
        &self,
        clsid: ConstPtr<crate::core::GUID>,
        punk_outer: crate::core::IUnknown,
        dw_cls_ctx: CLSCTX,
        p_server_info: ConstPtr<COSERVERINFO>,
        dw_count: u32,
        p_results: MutPtr<MULTI_QI>,
    ) -> crate::core::HRESULT {
        todo!("CoCreateInstanceEx")
    }
    fn CoCreateInstanceFromApp(
        &self,
        clsid: ConstPtr<crate::core::GUID>,
        punk_outer: crate::core::IUnknown,
        dw_cls_ctx: CLSCTX,
        reserved: ConstPtr<::core::ffi::c_void>,
        dw_count: u32,
        p_results: MutPtr<MULTI_QI>,
    ) -> crate::core::HRESULT {
        todo!("CoCreateInstanceFromApp")
    }
    fn CoDecrementMTAUsage(&self, cookie: CO_MTA_USAGE_COOKIE) -> crate::core::HRESULT {
        todo!("CoDecrementMTAUsage")
    }
    fn CoDisableCallCancellation(
        &self,
        p_reserved: ConstPtr<::core::ffi::c_void>,
    ) -> crate::core::HRESULT {
        todo!("CoDisableCallCancellation")
    }
    fn CoDisconnectContext(&self, dw_timeout: u32) -> crate::core::HRESULT {
        todo!("CoDisconnectContext")
    }
    fn CoDisconnectObject(
        &self,
        p_unk: crate::core::IUnknown,
        dw_reserved: u32,
    ) -> crate::core::HRESULT {
        todo!("CoDisconnectObject")
    }
    fn CoDosDateTimeToFileTime(
        &self,
        n_dos_date: u16,
        n_dos_time: u16,
        lp_file_time: MutPtr<super::super::Foundation::FILETIME>,
    ) -> super::super::Foundation::BOOL {
        todo!("CoDosDateTimeToFileTime")
    }
    fn CoEnableCallCancellation(
        &self,
        p_reserved: ConstPtr<::core::ffi::c_void>,
    ) -> crate::core::HRESULT {
        todo!("CoEnableCallCancellation")
    }
    fn CoFileTimeNow(
        &self,
        lp_file_time: MutPtr<super::super::Foundation::FILETIME>,
    ) -> crate::core::HRESULT {
        todo!("CoFileTimeNow")
    }
    fn CoFileTimeToDosDateTime(
        &self,
        lp_file_time: ConstPtr<super::super::Foundation::FILETIME>,
        lp_dos_date: MutPtr<u16>,
        lp_dos_time: MutPtr<u16>,
    ) -> super::super::Foundation::BOOL {
        todo!("CoFileTimeToDosDateTime")
    }
    fn CoFreeAllLibraries(&self) {
        todo!("CoFreeAllLibraries")
    }
    fn CoFreeLibrary(&self, h_inst: super::super::Foundation::HINSTANCE) {
        todo!("CoFreeLibrary")
    }
    fn CoFreeUnusedLibraries(&self) {
        todo!("CoFreeUnusedLibraries")
    }
    fn CoFreeUnusedLibrariesEx(&self, dw_unload_delay: u32, dw_reserved: u32) {
        todo!("CoFreeUnusedLibrariesEx")
    }
    fn CoGetApartmentType(
        &self,
        p_apt_type: MutPtr<APTTYPE>,
        p_apt_qualifier: MutPtr<APTTYPEQUALIFIER>,
    ) -> crate::core::HRESULT {
        todo!("CoGetApartmentType")
    }
    fn CoGetCallContext(
        &self,
        riid: ConstPtr<crate::core::GUID>,
        pp_interface: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("CoGetCallContext")
    }
    fn CoGetCallerTID(&self, lpdw_tid: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("CoGetCallerTID")
    }
    fn CoGetCancelObject(
        &self,
        dw_thread_id: u32,
        iid: ConstPtr<crate::core::GUID>,
        pp_unk: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("CoGetCancelObject")
    }
    fn CoGetClassObject(
        &self,
        rclsid: ConstPtr<crate::core::GUID>,
        dw_cls_context: CLSCTX,
        pv_reserved: ConstPtr<::core::ffi::c_void>,
        riid: ConstPtr<crate::core::GUID>,
        ppv: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("CoGetClassObject")
    }
    fn CoGetContextToken(&self, p_token: MutPtr<PtrRepr>) -> crate::core::HRESULT {
        todo!("CoGetContextToken")
    }
    fn CoGetCurrentLogicalThreadId(
        &self,
        pguid: MutPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("CoGetCurrentLogicalThreadId")
    }
    fn CoGetCurrentProcess(&self) -> u32 {
        todo!("CoGetCurrentProcess")
    }
    fn CoGetMalloc(&self, dw_mem_context: u32, pp_malloc: MutPtr<IMalloc>) -> crate::core::HRESULT {
        todo!("CoGetMalloc")
    }
    fn CoGetObject(
        &self,
        psz_name: PCWSTR,
        p_bind_options: ConstPtr<BIND_OPTS>,
        riid: ConstPtr<crate::core::GUID>,
        ppv: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("CoGetObject")
    }
    fn CoGetObjectContext(
        &self,
        riid: ConstPtr<crate::core::GUID>,
        ppv: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("CoGetObjectContext")
    }
    fn CoGetPSClsid(
        &self,
        riid: ConstPtr<crate::core::GUID>,
        p_clsid: MutPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("CoGetPSClsid")
    }
    fn CoGetSystemSecurityPermissions(
        &self,
        com_sd_type: COMSD,
        pp_sd: MutPtr<ConstPtr<super::super::Security::SECURITY_DESCRIPTOR>>,
    ) -> crate::core::HRESULT {
        todo!("CoGetSystemSecurityPermissions")
    }
    fn CoGetTreatAsClass(
        &self,
        clsid_old: ConstPtr<crate::core::GUID>,
        p_clsid_new: MutPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("CoGetTreatAsClass")
    }
    fn CoImpersonateClient(&self) -> crate::core::HRESULT {
        todo!("CoImpersonateClient")
    }
    fn CoIncrementMTAUsage(&self, p_cookie: MutPtr<CO_MTA_USAGE_COOKIE>) -> crate::core::HRESULT {
        todo!("CoIncrementMTAUsage")
    }
    fn CoInitialize(&self, pv_reserved: ConstPtr<::core::ffi::c_void>) -> crate::core::HRESULT {
        todo!("CoInitialize")
    }
    fn CoInitializeEx(
        &self,
        pv_reserved: ConstPtr<::core::ffi::c_void>,
        dw_co_init: COINIT,
    ) -> crate::core::HRESULT {
        todo!("CoInitializeEx")
    }
    fn CoInitializeSecurity(
        &self,
        p_sec_desc: ConstPtr<super::super::Security::SECURITY_DESCRIPTOR>,
        c_auth_svc: i32,
        as_auth_svc: ConstPtr<SOLE_AUTHENTICATION_SERVICE>,
        p_reserved_1: ConstPtr<::core::ffi::c_void>,
        dw_authn_level: RPC_C_AUTHN_LEVEL,
        dw_imp_level: RPC_C_IMP_LEVEL,
        p_auth_list: ConstPtr<::core::ffi::c_void>,
        dw_capabilities: EOLE_AUTHENTICATION_CAPABILITIES,
        p_reserved_3: ConstPtr<::core::ffi::c_void>,
    ) -> crate::core::HRESULT {
        todo!("CoInitializeSecurity")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CoInstall(
        &self,
        pbc: IBindCtx,
        dw_flags: u32,
        p_class_spec: ConstPtr<uCLSSPEC>,
        p_query: ConstPtr<QUERYCONTEXT>,
        psz_code_base: PCWSTR,
    ) -> crate::core::HRESULT {
        todo!("CoInstall")
    }
    fn CoInvalidateRemoteMachineBindings(&self, psz_machine_name: PCWSTR) -> crate::core::HRESULT {
        todo!("CoInvalidateRemoteMachineBindings")
    }
    fn CoIsHandlerConnected(&self, p_unk: crate::core::IUnknown) -> super::super::Foundation::BOOL {
        todo!("CoIsHandlerConnected")
    }
    fn CoIsOle1Class(&self, rclsid: ConstPtr<crate::core::GUID>) -> super::super::Foundation::BOOL {
        todo!("CoIsOle1Class")
    }
    fn CoLoadLibrary(
        &self,
        lpsz_lib_name: PCWSTR,
        b_auto_free: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::HINSTANCE {
        todo!("CoLoadLibrary")
    }
    fn CoLockObjectExternal(
        &self,
        p_unk: crate::core::IUnknown,
        f_lock: super::super::Foundation::BOOL,
        f_last_unlock_releases: super::super::Foundation::BOOL,
    ) -> crate::core::HRESULT {
        todo!("CoLockObjectExternal")
    }
    fn CoQueryAuthenticationServices(
        &self,
        pc_auth_svc: MutPtr<u32>,
        as_auth_svc: MutPtr<ConstPtr<SOLE_AUTHENTICATION_SERVICE>>,
    ) -> crate::core::HRESULT {
        todo!("CoQueryAuthenticationServices")
    }
    fn CoQueryClientBlanket(
        &self,
        p_authn_svc: MutPtr<u32>,
        p_authz_svc: MutPtr<u32>,
        p_server_princ_name: MutPtr<PWSTR>,
        p_authn_level: MutPtr<u32>,
        p_imp_level: MutPtr<u32>,
        p_privs: MutPtr<ConstPtr<::core::ffi::c_void>>,
        p_capabilities: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("CoQueryClientBlanket")
    }
    fn CoQueryProxyBlanket(
        &self,
        p_proxy: crate::core::IUnknown,
        pw_authn_svc: MutPtr<u32>,
        p_authz_svc: MutPtr<u32>,
        p_server_princ_name: MutPtr<PWSTR>,
        p_authn_level: MutPtr<u32>,
        p_imp_level: MutPtr<u32>,
        p_auth_info: MutPtr<ConstPtr<::core::ffi::c_void>>,
        p_capabilites: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("CoQueryProxyBlanket")
    }
    fn CoRegisterActivationFilter(
        &self,
        p_activation_filter: IActivationFilter,
    ) -> crate::core::HRESULT {
        todo!("CoRegisterActivationFilter")
    }
    fn CoRegisterChannelHook(
        &self,
        extension_uuid: ConstPtr<crate::core::GUID>,
        p_channel_hook: IChannelHook,
    ) -> crate::core::HRESULT {
        todo!("CoRegisterChannelHook")
    }
    fn CoRegisterClassObject(
        &self,
        rclsid: ConstPtr<crate::core::GUID>,
        p_unk: crate::core::IUnknown,
        dw_cls_context: CLSCTX,
        flags: u32,
        lpdw_register: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("CoRegisterClassObject")
    }
    fn CoRegisterDeviceCatalog(
        &self,
        device_instance_id: PCWSTR,
        cookie: MutPtr<CO_DEVICE_CATALOG_COOKIE>,
    ) -> crate::core::HRESULT {
        todo!("CoRegisterDeviceCatalog")
    }
    fn CoRegisterInitializeSpy(
        &self,
        p_spy: IInitializeSpy,
        puli_cookie: MutPtr<u64>,
    ) -> crate::core::HRESULT {
        todo!("CoRegisterInitializeSpy")
    }
    fn CoRegisterMallocSpy(&self, p_malloc_spy: IMallocSpy) -> crate::core::HRESULT {
        todo!("CoRegisterMallocSpy")
    }
    fn CoRegisterPSClsid(
        &self,
        riid: ConstPtr<crate::core::GUID>,
        rclsid: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("CoRegisterPSClsid")
    }
    fn CoRegisterSurrogate(&self, p_surrogate: ISurrogate) -> crate::core::HRESULT {
        todo!("CoRegisterSurrogate")
    }
    fn CoReleaseServerProcess(&self) -> u32 {
        todo!("CoReleaseServerProcess")
    }
    fn CoResumeClassObjects(&self) -> crate::core::HRESULT {
        todo!("CoResumeClassObjects")
    }
    fn CoRevertToSelf(&self) -> crate::core::HRESULT {
        todo!("CoRevertToSelf")
    }
    fn CoRevokeClassObject(&self, dw_register: u32) -> crate::core::HRESULT {
        todo!("CoRevokeClassObject")
    }
    fn CoRevokeDeviceCatalog(&self, cookie: CO_DEVICE_CATALOG_COOKIE) -> crate::core::HRESULT {
        todo!("CoRevokeDeviceCatalog")
    }
    fn CoRevokeInitializeSpy(&self, uli_cookie: u64) -> crate::core::HRESULT {
        todo!("CoRevokeInitializeSpy")
    }
    fn CoRevokeMallocSpy(&self) -> crate::core::HRESULT {
        todo!("CoRevokeMallocSpy")
    }
    fn CoSetCancelObject(&self, p_unk: crate::core::IUnknown) -> crate::core::HRESULT {
        todo!("CoSetCancelObject")
    }
    fn CoSetProxyBlanket(
        &self,
        p_proxy: crate::core::IUnknown,
        dw_authn_svc: u32,
        dw_authz_svc: u32,
        p_server_princ_name: PCWSTR,
        dw_authn_level: RPC_C_AUTHN_LEVEL,
        dw_imp_level: RPC_C_IMP_LEVEL,
        p_auth_info: ConstPtr<::core::ffi::c_void>,
        dw_capabilities: EOLE_AUTHENTICATION_CAPABILITIES,
    ) -> crate::core::HRESULT {
        todo!("CoSetProxyBlanket")
    }
    fn CoSuspendClassObjects(&self) -> crate::core::HRESULT {
        todo!("CoSuspendClassObjects")
    }
    fn CoSwitchCallContext(
        &self,
        p_new_object: crate::core::IUnknown,
        pp_old_object: MutPtr<crate::core::IUnknown>,
    ) -> crate::core::HRESULT {
        todo!("CoSwitchCallContext")
    }
    fn CoTaskMemAlloc(&self, cb: PtrRepr) -> MutPtr<::core::ffi::c_void> {
        todo!("CoTaskMemAlloc")
    }
    fn CoTaskMemFree(&self, pv: ConstPtr<::core::ffi::c_void>) {
        todo!("CoTaskMemFree")
    }
    fn CoTaskMemRealloc(
        &self,
        pv: ConstPtr<::core::ffi::c_void>,
        cb: PtrRepr,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!("CoTaskMemRealloc")
    }
    fn CoTestCancel(&self) -> crate::core::HRESULT {
        todo!("CoTestCancel")
    }
    fn CoTreatAsClass(
        &self,
        clsid_old: ConstPtr<crate::core::GUID>,
        clsid_new: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("CoTreatAsClass")
    }
    fn CoUninitialize(&self) {
        todo!("CoUninitialize")
    }
    fn CoWaitForMultipleHandles(
        &self,
        dw_flags: u32,
        dw_timeout: u32,
        c_handles: u32,
        p_handles: ConstPtr<super::super::Foundation::HANDLE>,
        lpdwindex: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("CoWaitForMultipleHandles")
    }
    fn CoWaitForMultipleObjects(
        &self,
        dw_flags: u32,
        dw_timeout: u32,
        c_handles: u32,
        p_handles: ConstPtr<super::super::Foundation::HANDLE>,
        lpdwindex: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("CoWaitForMultipleObjects")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateAntiMoniker(&self, ppmk: MutPtr<IMoniker>) -> crate::core::HRESULT {
        todo!("CreateAntiMoniker")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateBindCtx(&self, reserved: u32, ppbc: MutPtr<IBindCtx>) -> crate::core::HRESULT {
        todo!("CreateBindCtx")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateClassMoniker(
        &self,
        rclsid: ConstPtr<crate::core::GUID>,
        ppmk: MutPtr<IMoniker>,
    ) -> crate::core::HRESULT {
        todo!("CreateClassMoniker")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateDataAdviseHolder(
        &self,
        pp_da_holder: MutPtr<IDataAdviseHolder>,
    ) -> crate::core::HRESULT {
        todo!("CreateDataAdviseHolder")
    }
    fn CreateDataCache(
        &self,
        p_unk_outer: crate::core::IUnknown,
        rclsid: ConstPtr<crate::core::GUID>,
        iid: ConstPtr<crate::core::GUID>,
        ppv: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("CreateDataCache")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateFileMoniker(
        &self,
        lpsz_path_name: PCWSTR,
        ppmk: MutPtr<IMoniker>,
    ) -> crate::core::HRESULT {
        todo!("CreateFileMoniker")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateGenericComposite(
        &self,
        pmk_first: IMoniker,
        pmk_rest: IMoniker,
        ppmk_composite: MutPtr<IMoniker>,
    ) -> crate::core::HRESULT {
        todo!("CreateGenericComposite")
    }
    fn CreateIUriBuilder(
        &self,
        p_i_uri: IUri,
        dw_flags: u32,
        dw_reserved: PtrRepr,
        pp_i_uri_builder: MutPtr<IUriBuilder>,
    ) -> crate::core::HRESULT {
        todo!("CreateIUriBuilder")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateItemMoniker(
        &self,
        lpsz_delim: PCWSTR,
        lpsz_item: PCWSTR,
        ppmk: MutPtr<IMoniker>,
    ) -> crate::core::HRESULT {
        todo!("CreateItemMoniker")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateObjrefMoniker(
        &self,
        punk: crate::core::IUnknown,
        ppmk: MutPtr<IMoniker>,
    ) -> crate::core::HRESULT {
        todo!("CreateObjrefMoniker")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreatePointerMoniker(
        &self,
        punk: crate::core::IUnknown,
        ppmk: MutPtr<IMoniker>,
    ) -> crate::core::HRESULT {
        todo!("CreatePointerMoniker")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi', 'Windows.Win32.Security', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateStdProgressIndicator(
        &self,
        hwnd_parent: super::super::Foundation::HWND,
        psz_title: PCWSTR,
        p_ibsc_caller: IBindStatusCallback,
        pp_ibsc: MutPtr<IBindStatusCallback>,
    ) -> crate::core::HRESULT {
        todo!("CreateStdProgressIndicator")
    }
    fn CreateUri(
        &self,
        pwz_uri: PCWSTR,
        dw_flags: URI_CREATE_FLAGS,
        dw_reserved: PtrRepr,
        pp_uri: MutPtr<IUri>,
    ) -> crate::core::HRESULT {
        todo!("CreateUri")
    }
    fn CreateUriFromMultiByteString(
        &self,
        psz_ansi_input_uri: PCSTR,
        dw_encoding_flags: u32,
        dw_code_page: u32,
        dw_create_flags: u32,
        dw_reserved: PtrRepr,
        pp_uri: MutPtr<IUri>,
    ) -> crate::core::HRESULT {
        todo!("CreateUriFromMultiByteString")
    }
    fn CreateUriWithFragment(
        &self,
        pwz_uri: PCWSTR,
        pwz_fragment: PCWSTR,
        dw_flags: u32,
        dw_reserved: PtrRepr,
        pp_uri: MutPtr<IUri>,
    ) -> crate::core::HRESULT {
        todo!("CreateUriWithFragment")
    }
    fn DcomChannelSetHResult(
        &self,
        pv_reserved: ConstPtr<::core::ffi::c_void>,
        pul_reserved: ConstPtr<u32>,
        apps_hr: crate::core::HRESULT,
    ) -> crate::core::HRESULT {
        todo!("DcomChannelSetHResult")
    }
    fn GetClassFile(
        &self,
        sz_filename: PCWSTR,
        pclsid: MutPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("GetClassFile")
    }
    fn GetErrorInfo(
        &self,
        dw_reserved: u32,
        pperrinfo: MutPtr<IErrorInfo>,
    ) -> crate::core::HRESULT {
        todo!("GetErrorInfo")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetRunningObjectTable(
        &self,
        reserved: u32,
        pprot: MutPtr<IRunningObjectTable>,
    ) -> crate::core::HRESULT {
        todo!("GetRunningObjectTable")
    }
    fn IIDFromString(
        &self,
        lpsz: PCWSTR,
        lpiid: MutPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("IIDFromString")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn MkParseDisplayName(
        &self,
        pbc: IBindCtx,
        sz_user_name: PCWSTR,
        pch_eaten: MutPtr<u32>,
        ppmk: MutPtr<IMoniker>,
    ) -> crate::core::HRESULT {
        todo!("MkParseDisplayName")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn MonikerCommonPrefixWith(
        &self,
        pmk_this: IMoniker,
        pmk_other: IMoniker,
        ppmk_common: MutPtr<IMoniker>,
    ) -> crate::core::HRESULT {
        todo!("MonikerCommonPrefixWith")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn MonikerRelativePathTo(
        &self,
        pmk_src: IMoniker,
        pmk_dest: IMoniker,
        ppmk_rel_path: MutPtr<IMoniker>,
        dw_reserved: super::super::Foundation::BOOL,
    ) -> crate::core::HRESULT {
        todo!("MonikerRelativePathTo")
    }
    fn ProgIDFromCLSID(
        &self,
        clsid: ConstPtr<crate::core::GUID>,
        lplpsz_prog_id: MutPtr<PWSTR>,
    ) -> crate::core::HRESULT {
        todo!("ProgIDFromCLSID")
    }
    fn SetErrorInfo(&self, dw_reserved: u32, perrinfo: IErrorInfo) -> crate::core::HRESULT {
        todo!("SetErrorInfo")
    }
    fn StringFromCLSID(
        &self,
        rclsid: ConstPtr<crate::core::GUID>,
        lplpsz: MutPtr<PWSTR>,
    ) -> crate::core::HRESULT {
        todo!("StringFromCLSID")
    }
    fn StringFromGUID2(
        &self,
        rguid: ConstPtr<crate::core::GUID>,
        lpsz: PWSTR,
        cch_max: i32,
    ) -> i32 {
        todo!("StringFromGUID2")
    }
    fn StringFromIID(
        &self,
        rclsid: ConstPtr<crate::core::GUID>,
        lplpsz: MutPtr<PWSTR>,
    ) -> crate::core::HRESULT {
        todo!("StringFromIID")
    }
}
pub fn get_api(ctx: &crate::core::Win32Context) -> std::sync::Arc<dyn Api> {
    ctx.get::<dyn Api>()
}
