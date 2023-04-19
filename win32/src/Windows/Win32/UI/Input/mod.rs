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
pub mod KeyboardAndMouse;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HRAWINPUT(pub PtrDiffRepr);
impl HRAWINPUT {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HRAWINPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HRAWINPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HRAWINPUT {}
impl ::core::hash::Hash for HRAWINPUT {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HRAWINPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HRAWINPUT").field(&self.0).finish()
    }
}
impl FromIntoMemory for HRAWINPUT {
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
pub struct INPUT_MESSAGE_DEVICE_TYPE(pub i32);
pub const IMDT_UNAVAILABLE: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(0i32);
pub const IMDT_KEYBOARD: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(1i32);
pub const IMDT_MOUSE: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(2i32);
pub const IMDT_TOUCH: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(4i32);
pub const IMDT_PEN: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(8i32);
pub const IMDT_TOUCHPAD: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(16i32);
impl ::core::marker::Copy for INPUT_MESSAGE_DEVICE_TYPE {}
impl ::core::clone::Clone for INPUT_MESSAGE_DEVICE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INPUT_MESSAGE_DEVICE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for INPUT_MESSAGE_DEVICE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INPUT_MESSAGE_DEVICE_TYPE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for INPUT_MESSAGE_DEVICE_TYPE {
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
pub struct INPUT_MESSAGE_ORIGIN_ID(pub i32);
pub const IMO_UNAVAILABLE: INPUT_MESSAGE_ORIGIN_ID = INPUT_MESSAGE_ORIGIN_ID(0i32);
pub const IMO_HARDWARE: INPUT_MESSAGE_ORIGIN_ID = INPUT_MESSAGE_ORIGIN_ID(1i32);
pub const IMO_INJECTED: INPUT_MESSAGE_ORIGIN_ID = INPUT_MESSAGE_ORIGIN_ID(2i32);
pub const IMO_SYSTEM: INPUT_MESSAGE_ORIGIN_ID = INPUT_MESSAGE_ORIGIN_ID(4i32);
impl ::core::marker::Copy for INPUT_MESSAGE_ORIGIN_ID {}
impl ::core::clone::Clone for INPUT_MESSAGE_ORIGIN_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INPUT_MESSAGE_ORIGIN_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for INPUT_MESSAGE_ORIGIN_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INPUT_MESSAGE_ORIGIN_ID")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for INPUT_MESSAGE_ORIGIN_ID {
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
pub struct INPUT_MESSAGE_SOURCE {
    pub deviceType: INPUT_MESSAGE_DEVICE_TYPE,
    pub originId: INPUT_MESSAGE_ORIGIN_ID,
}
impl ::core::marker::Copy for INPUT_MESSAGE_SOURCE {}
impl ::core::clone::Clone for INPUT_MESSAGE_SOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INPUT_MESSAGE_SOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INPUT_MESSAGE_SOURCE")
            .field("deviceType", &self.deviceType)
            .field("originId", &self.originId)
            .finish()
    }
}
impl ::core::cmp::PartialEq for INPUT_MESSAGE_SOURCE {
    fn eq(&self, other: &Self) -> bool {
        self.deviceType == other.deviceType && self.originId == other.originId
    }
}
impl ::core::cmp::Eq for INPUT_MESSAGE_SOURCE {}
impl FromIntoMemory for INPUT_MESSAGE_SOURCE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_deviceType =
            <INPUT_MESSAGE_DEVICE_TYPE as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_originId = <INPUT_MESSAGE_ORIGIN_ID as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            deviceType: f_deviceType,
            originId: f_originId,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.deviceType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.originId, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct RAWHID {
    pub dwSizeHid: u32,
    pub dwCount: u32,
    pub bRawData: [u8; 1],
}
impl ::core::marker::Copy for RAWHID {}
impl ::core::clone::Clone for RAWHID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RAWHID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAWHID")
            .field("dwSizeHid", &self.dwSizeHid)
            .field("dwCount", &self.dwCount)
            .field("bRawData", &self.bRawData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RAWHID {
    fn eq(&self, other: &Self) -> bool {
        self.dwSizeHid == other.dwSizeHid
            && self.dwCount == other.dwCount
            && self.bRawData == other.bRawData
    }
}
impl ::core::cmp::Eq for RAWHID {}
impl FromIntoMemory for RAWHID {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_dwSizeHid = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwCount = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_bRawData = <[u8; 1] as FromIntoMemory>::from_bytes(&from[8..8 + 1]);
        Self {
            dwSizeHid: f_dwSizeHid,
            dwCount: f_dwCount,
            bRawData: f_bRawData,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.dwSizeHid, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwCount, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.bRawData, &mut into[8..8 + 1]);
    }
    fn size() -> usize {
        12
    }
}
pub struct RAWINPUT {
    pub header: RAWINPUTHEADER,
    pub data: RAWINPUT_0,
}
impl ::core::marker::Copy for RAWINPUT {}
impl ::core::clone::Clone for RAWINPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RAWINPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAWINPUT")
            .field("header", &self.header)
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RAWINPUT {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.data == other.data
    }
}
impl ::core::cmp::Eq for RAWINPUT {}
impl FromIntoMemory for RAWINPUT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_header = <RAWINPUTHEADER as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_data = <RAWINPUT_0 as FromIntoMemory>::from_bytes(&from[16..16 + 12]);
        Self {
            header: f_header,
            data: f_data,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.header, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.data, &mut into[16..16 + 12]);
    }
    fn size() -> usize {
        28
    }
}
pub struct RAWINPUT_0 {
    data: [u8; 12],
}
impl ::core::default::Default for RAWINPUT_0 {
    fn default() -> Self {
        Self { data: [0u8; 12] }
    }
}
impl ::core::marker::Copy for RAWINPUT_0 {}
impl ::core::clone::Clone for RAWINPUT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RAWINPUT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAWINPUT_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RAWINPUT_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for RAWINPUT_0 {}
impl FromIntoMemory for RAWINPUT_0 {
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
pub struct RAWINPUTDEVICE {
    pub usUsagePage: u16,
    pub usUsage: u16,
    pub dwFlags: RAWINPUTDEVICE_FLAGS,
    pub hwndTarget: super::super::Foundation::HWND,
}
impl ::core::marker::Copy for RAWINPUTDEVICE {}
impl ::core::clone::Clone for RAWINPUTDEVICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RAWINPUTDEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAWINPUTDEVICE")
            .field("usUsagePage", &self.usUsagePage)
            .field("usUsage", &self.usUsage)
            .field("dwFlags", &self.dwFlags)
            .field("hwndTarget", &self.hwndTarget)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RAWINPUTDEVICE {
    fn eq(&self, other: &Self) -> bool {
        self.usUsagePage == other.usUsagePage
            && self.usUsage == other.usUsage
            && self.dwFlags == other.dwFlags
            && self.hwndTarget == other.hwndTarget
    }
}
impl ::core::cmp::Eq for RAWINPUTDEVICE {}
impl FromIntoMemory for RAWINPUTDEVICE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_usUsagePage = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_usUsage = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_dwFlags = <RAWINPUTDEVICE_FLAGS as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_hwndTarget =
            <super::super::Foundation::HWND as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            usUsagePage: f_usUsagePage,
            usUsage: f_usUsage,
            dwFlags: f_dwFlags,
            hwndTarget: f_hwndTarget,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.usUsagePage, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.usUsage, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.hwndTarget, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct RAWINPUTDEVICELIST {
    pub hDevice: super::super::Foundation::HANDLE,
    pub dwType: RID_DEVICE_INFO_TYPE,
}
impl ::core::marker::Copy for RAWINPUTDEVICELIST {}
impl ::core::clone::Clone for RAWINPUTDEVICELIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RAWINPUTDEVICELIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAWINPUTDEVICELIST")
            .field("hDevice", &self.hDevice)
            .field("dwType", &self.dwType)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RAWINPUTDEVICELIST {
    fn eq(&self, other: &Self) -> bool {
        self.hDevice == other.hDevice && self.dwType == other.dwType
    }
}
impl ::core::cmp::Eq for RAWINPUTDEVICELIST {}
impl FromIntoMemory for RAWINPUTDEVICELIST {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_hDevice =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwType = <RID_DEVICE_INFO_TYPE as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            hDevice: f_hDevice,
            dwType: f_dwType,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.hDevice, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwType, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RAWINPUTDEVICE_FLAGS(pub u32);
pub const RIDEV_REMOVE: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(1u32);
pub const RIDEV_EXCLUDE: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(16u32);
pub const RIDEV_PAGEONLY: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(32u32);
pub const RIDEV_NOLEGACY: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(48u32);
pub const RIDEV_INPUTSINK: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(256u32);
pub const RIDEV_CAPTUREMOUSE: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(512u32);
pub const RIDEV_NOHOTKEYS: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(512u32);
pub const RIDEV_APPKEYS: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(1024u32);
pub const RIDEV_EXINPUTSINK: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(4096u32);
pub const RIDEV_DEVNOTIFY: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(8192u32);
impl ::core::marker::Copy for RAWINPUTDEVICE_FLAGS {}
impl ::core::clone::Clone for RAWINPUTDEVICE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RAWINPUTDEVICE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RAWINPUTDEVICE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RAWINPUTDEVICE_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for RAWINPUTDEVICE_FLAGS {
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
pub struct RAWINPUTHEADER {
    pub dwType: u32,
    pub dwSize: u32,
    pub hDevice: super::super::Foundation::HANDLE,
    pub wParam: super::super::Foundation::WPARAM,
}
impl ::core::marker::Copy for RAWINPUTHEADER {}
impl ::core::clone::Clone for RAWINPUTHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RAWINPUTHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAWINPUTHEADER")
            .field("dwType", &self.dwType)
            .field("dwSize", &self.dwSize)
            .field("hDevice", &self.hDevice)
            .field("wParam", &self.wParam)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RAWINPUTHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType
            && self.dwSize == other.dwSize
            && self.hDevice == other.hDevice
            && self.wParam == other.wParam
    }
}
impl ::core::cmp::Eq for RAWINPUTHEADER {}
impl FromIntoMemory for RAWINPUTHEADER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dwType = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_hDevice =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_wParam =
            <super::super::Foundation::WPARAM as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            dwType: f_dwType,
            dwSize: f_dwSize,
            hDevice: f_hDevice,
            wParam: f_wParam,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dwType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.hDevice, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.wParam, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct RAWKEYBOARD {
    pub MakeCode: u16,
    pub Flags: u16,
    pub Reserved: u16,
    pub VKey: u16,
    pub Message: u32,
    pub ExtraInformation: u32,
}
impl ::core::marker::Copy for RAWKEYBOARD {}
impl ::core::clone::Clone for RAWKEYBOARD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RAWKEYBOARD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAWKEYBOARD")
            .field("MakeCode", &self.MakeCode)
            .field("Flags", &self.Flags)
            .field("Reserved", &self.Reserved)
            .field("VKey", &self.VKey)
            .field("Message", &self.Message)
            .field("ExtraInformation", &self.ExtraInformation)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RAWKEYBOARD {
    fn eq(&self, other: &Self) -> bool {
        self.MakeCode == other.MakeCode
            && self.Flags == other.Flags
            && self.Reserved == other.Reserved
            && self.VKey == other.VKey
            && self.Message == other.Message
            && self.ExtraInformation == other.ExtraInformation
    }
}
impl ::core::cmp::Eq for RAWKEYBOARD {}
impl FromIntoMemory for RAWKEYBOARD {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_MakeCode = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_Flags = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_Reserved = <u16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_VKey = <u16 as FromIntoMemory>::from_bytes(&from[6..6 + 2]);
        let f_Message = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ExtraInformation = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            MakeCode: f_MakeCode,
            Flags: f_Flags,
            Reserved: f_Reserved,
            VKey: f_VKey,
            Message: f_Message,
            ExtraInformation: f_ExtraInformation,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.MakeCode, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.VKey, &mut into[6..6 + 2]);
        FromIntoMemory::into_bytes(self.Message, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ExtraInformation, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct RAWMOUSE {
    pub usFlags: u16,
    pub Anonymous: RAWMOUSE_0,
    pub ulRawButtons: u32,
    pub lLastX: i32,
    pub lLastY: i32,
    pub ulExtraInformation: u32,
}
impl ::core::marker::Copy for RAWMOUSE {}
impl ::core::clone::Clone for RAWMOUSE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RAWMOUSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAWMOUSE")
            .field("usFlags", &self.usFlags)
            .field("Anonymous", &self.Anonymous)
            .field("ulRawButtons", &self.ulRawButtons)
            .field("lLastX", &self.lLastX)
            .field("lLastY", &self.lLastY)
            .field("ulExtraInformation", &self.ulExtraInformation)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RAWMOUSE {
    fn eq(&self, other: &Self) -> bool {
        self.usFlags == other.usFlags
            && self.Anonymous == other.Anonymous
            && self.ulRawButtons == other.ulRawButtons
            && self.lLastX == other.lLastX
            && self.lLastY == other.lLastY
            && self.ulExtraInformation == other.ulExtraInformation
    }
}
impl ::core::cmp::Eq for RAWMOUSE {}
impl FromIntoMemory for RAWMOUSE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_usFlags = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_Anonymous = <RAWMOUSE_0 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_ulRawButtons = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lLastX = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lLastY = <i32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_ulExtraInformation = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            usFlags: f_usFlags,
            Anonymous: f_Anonymous,
            ulRawButtons: f_ulRawButtons,
            lLastX: f_lLastX,
            lLastY: f_lLastY,
            ulExtraInformation: f_ulExtraInformation,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.usFlags, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.ulRawButtons, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lLastX, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lLastY, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.ulExtraInformation, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct RAWMOUSE_0 {
    data: [u8; 4],
}
impl ::core::default::Default for RAWMOUSE_0 {
    fn default() -> Self {
        Self { data: [0u8; 4] }
    }
}
impl ::core::marker::Copy for RAWMOUSE_0 {}
impl ::core::clone::Clone for RAWMOUSE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RAWMOUSE_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAWMOUSE_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RAWMOUSE_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for RAWMOUSE_0 {}
impl FromIntoMemory for RAWMOUSE_0 {
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
pub struct RAWMOUSE_0_0 {
    pub usButtonFlags: u16,
    pub usButtonData: u16,
}
impl ::core::marker::Copy for RAWMOUSE_0_0 {}
impl ::core::clone::Clone for RAWMOUSE_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RAWMOUSE_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAWMOUSE_0_0")
            .field("usButtonFlags", &self.usButtonFlags)
            .field("usButtonData", &self.usButtonData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RAWMOUSE_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.usButtonFlags == other.usButtonFlags && self.usButtonData == other.usButtonData
    }
}
impl ::core::cmp::Eq for RAWMOUSE_0_0 {}
impl FromIntoMemory for RAWMOUSE_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_usButtonFlags = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_usButtonData = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        Self {
            usButtonFlags: f_usButtonFlags,
            usButtonData: f_usButtonData,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.usButtonFlags, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.usButtonData, &mut into[2..2 + 2]);
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RAW_INPUT_DATA_COMMAND_FLAGS(pub u32);
pub const RID_HEADER: RAW_INPUT_DATA_COMMAND_FLAGS = RAW_INPUT_DATA_COMMAND_FLAGS(268435461u32);
pub const RID_INPUT: RAW_INPUT_DATA_COMMAND_FLAGS = RAW_INPUT_DATA_COMMAND_FLAGS(268435459u32);
impl ::core::marker::Copy for RAW_INPUT_DATA_COMMAND_FLAGS {}
impl ::core::clone::Clone for RAW_INPUT_DATA_COMMAND_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RAW_INPUT_DATA_COMMAND_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RAW_INPUT_DATA_COMMAND_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RAW_INPUT_DATA_COMMAND_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for RAW_INPUT_DATA_COMMAND_FLAGS {
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
pub struct RAW_INPUT_DEVICE_INFO_COMMAND(pub u32);
pub const RIDI_PREPARSEDDATA: RAW_INPUT_DEVICE_INFO_COMMAND =
    RAW_INPUT_DEVICE_INFO_COMMAND(536870917u32);
pub const RIDI_DEVICENAME: RAW_INPUT_DEVICE_INFO_COMMAND =
    RAW_INPUT_DEVICE_INFO_COMMAND(536870919u32);
pub const RIDI_DEVICEINFO: RAW_INPUT_DEVICE_INFO_COMMAND =
    RAW_INPUT_DEVICE_INFO_COMMAND(536870923u32);
impl ::core::marker::Copy for RAW_INPUT_DEVICE_INFO_COMMAND {}
impl ::core::clone::Clone for RAW_INPUT_DEVICE_INFO_COMMAND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RAW_INPUT_DEVICE_INFO_COMMAND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RAW_INPUT_DEVICE_INFO_COMMAND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RAW_INPUT_DEVICE_INFO_COMMAND")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for RAW_INPUT_DEVICE_INFO_COMMAND {
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
pub struct RID_DEVICE_INFO {
    pub cbSize: u32,
    pub dwType: RID_DEVICE_INFO_TYPE,
    pub Anonymous: RID_DEVICE_INFO_0,
}
impl ::core::marker::Copy for RID_DEVICE_INFO {}
impl ::core::clone::Clone for RID_DEVICE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RID_DEVICE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RID_DEVICE_INFO")
            .field("cbSize", &self.cbSize)
            .field("dwType", &self.dwType)
            .field("Anonymous", &self.Anonymous)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RID_DEVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.dwType == other.dwType
            && self.Anonymous == other.Anonymous
    }
}
impl ::core::cmp::Eq for RID_DEVICE_INFO {}
impl FromIntoMemory for RID_DEVICE_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_cbSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwType = <RID_DEVICE_INFO_TYPE as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Anonymous = <RID_DEVICE_INFO_0 as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        Self {
            cbSize: f_cbSize,
            dwType: f_dwType,
            Anonymous: f_Anonymous,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.cbSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwType, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[8..8 + 16]);
    }
    fn size() -> usize {
        24
    }
}
pub struct RID_DEVICE_INFO_0 {
    data: [u8; 16],
}
impl ::core::default::Default for RID_DEVICE_INFO_0 {
    fn default() -> Self {
        Self { data: [0u8; 16] }
    }
}
impl ::core::marker::Copy for RID_DEVICE_INFO_0 {}
impl ::core::clone::Clone for RID_DEVICE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RID_DEVICE_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RID_DEVICE_INFO_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RID_DEVICE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for RID_DEVICE_INFO_0 {}
impl FromIntoMemory for RID_DEVICE_INFO_0 {
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
pub struct RID_DEVICE_INFO_HID {
    pub dwVendorId: u32,
    pub dwProductId: u32,
    pub dwVersionNumber: u32,
    pub usUsagePage: u16,
    pub usUsage: u16,
}
impl ::core::marker::Copy for RID_DEVICE_INFO_HID {}
impl ::core::clone::Clone for RID_DEVICE_INFO_HID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RID_DEVICE_INFO_HID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RID_DEVICE_INFO_HID")
            .field("dwVendorId", &self.dwVendorId)
            .field("dwProductId", &self.dwProductId)
            .field("dwVersionNumber", &self.dwVersionNumber)
            .field("usUsagePage", &self.usUsagePage)
            .field("usUsage", &self.usUsage)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RID_DEVICE_INFO_HID {
    fn eq(&self, other: &Self) -> bool {
        self.dwVendorId == other.dwVendorId
            && self.dwProductId == other.dwProductId
            && self.dwVersionNumber == other.dwVersionNumber
            && self.usUsagePage == other.usUsagePage
            && self.usUsage == other.usUsage
    }
}
impl ::core::cmp::Eq for RID_DEVICE_INFO_HID {}
impl FromIntoMemory for RID_DEVICE_INFO_HID {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dwVendorId = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwProductId = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwVersionNumber = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_usUsagePage = <u16 as FromIntoMemory>::from_bytes(&from[12..12 + 2]);
        let f_usUsage = <u16 as FromIntoMemory>::from_bytes(&from[14..14 + 2]);
        Self {
            dwVendorId: f_dwVendorId,
            dwProductId: f_dwProductId,
            dwVersionNumber: f_dwVersionNumber,
            usUsagePage: f_usUsagePage,
            usUsage: f_usUsage,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dwVendorId, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwProductId, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwVersionNumber, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.usUsagePage, &mut into[12..12 + 2]);
        FromIntoMemory::into_bytes(self.usUsage, &mut into[14..14 + 2]);
    }
    fn size() -> usize {
        16
    }
}
pub struct RID_DEVICE_INFO_KEYBOARD {
    pub dwType: u32,
    pub dwSubType: u32,
    pub dwKeyboardMode: u32,
    pub dwNumberOfFunctionKeys: u32,
    pub dwNumberOfIndicators: u32,
    pub dwNumberOfKeysTotal: u32,
}
impl ::core::marker::Copy for RID_DEVICE_INFO_KEYBOARD {}
impl ::core::clone::Clone for RID_DEVICE_INFO_KEYBOARD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RID_DEVICE_INFO_KEYBOARD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RID_DEVICE_INFO_KEYBOARD")
            .field("dwType", &self.dwType)
            .field("dwSubType", &self.dwSubType)
            .field("dwKeyboardMode", &self.dwKeyboardMode)
            .field("dwNumberOfFunctionKeys", &self.dwNumberOfFunctionKeys)
            .field("dwNumberOfIndicators", &self.dwNumberOfIndicators)
            .field("dwNumberOfKeysTotal", &self.dwNumberOfKeysTotal)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RID_DEVICE_INFO_KEYBOARD {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType
            && self.dwSubType == other.dwSubType
            && self.dwKeyboardMode == other.dwKeyboardMode
            && self.dwNumberOfFunctionKeys == other.dwNumberOfFunctionKeys
            && self.dwNumberOfIndicators == other.dwNumberOfIndicators
            && self.dwNumberOfKeysTotal == other.dwNumberOfKeysTotal
    }
}
impl ::core::cmp::Eq for RID_DEVICE_INFO_KEYBOARD {}
impl FromIntoMemory for RID_DEVICE_INFO_KEYBOARD {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_dwType = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwSubType = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwKeyboardMode = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwNumberOfFunctionKeys = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwNumberOfIndicators = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwNumberOfKeysTotal = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            dwType: f_dwType,
            dwSubType: f_dwSubType,
            dwKeyboardMode: f_dwKeyboardMode,
            dwNumberOfFunctionKeys: f_dwNumberOfFunctionKeys,
            dwNumberOfIndicators: f_dwNumberOfIndicators,
            dwNumberOfKeysTotal: f_dwNumberOfKeysTotal,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.dwType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwSubType, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwKeyboardMode, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfFunctionKeys, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfIndicators, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfKeysTotal, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct RID_DEVICE_INFO_MOUSE {
    pub dwId: u32,
    pub dwNumberOfButtons: u32,
    pub dwSampleRate: u32,
    pub fHasHorizontalWheel: super::super::Foundation::BOOL,
}
impl ::core::marker::Copy for RID_DEVICE_INFO_MOUSE {}
impl ::core::clone::Clone for RID_DEVICE_INFO_MOUSE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RID_DEVICE_INFO_MOUSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RID_DEVICE_INFO_MOUSE")
            .field("dwId", &self.dwId)
            .field("dwNumberOfButtons", &self.dwNumberOfButtons)
            .field("dwSampleRate", &self.dwSampleRate)
            .field("fHasHorizontalWheel", &self.fHasHorizontalWheel)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RID_DEVICE_INFO_MOUSE {
    fn eq(&self, other: &Self) -> bool {
        self.dwId == other.dwId
            && self.dwNumberOfButtons == other.dwNumberOfButtons
            && self.dwSampleRate == other.dwSampleRate
            && self.fHasHorizontalWheel == other.fHasHorizontalWheel
    }
}
impl ::core::cmp::Eq for RID_DEVICE_INFO_MOUSE {}
impl FromIntoMemory for RID_DEVICE_INFO_MOUSE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dwId = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwNumberOfButtons = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwSampleRate = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_fHasHorizontalWheel =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            dwId: f_dwId,
            dwNumberOfButtons: f_dwNumberOfButtons,
            dwSampleRate: f_dwSampleRate,
            fHasHorizontalWheel: f_fHasHorizontalWheel,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dwId, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfButtons, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwSampleRate, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.fHasHorizontalWheel, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RID_DEVICE_INFO_TYPE(pub u32);
pub const RIM_TYPEMOUSE: RID_DEVICE_INFO_TYPE = RID_DEVICE_INFO_TYPE(0u32);
pub const RIM_TYPEKEYBOARD: RID_DEVICE_INFO_TYPE = RID_DEVICE_INFO_TYPE(1u32);
pub const RIM_TYPEHID: RID_DEVICE_INFO_TYPE = RID_DEVICE_INFO_TYPE(2u32);
impl ::core::marker::Copy for RID_DEVICE_INFO_TYPE {}
impl ::core::clone::Clone for RID_DEVICE_INFO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RID_DEVICE_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RID_DEVICE_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RID_DEVICE_INFO_TYPE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for RID_DEVICE_INFO_TYPE {
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
    #[doc = "DefRawInputProc from USER32"]
    fn DefRawInputProc(
        &self,
        pa_raw_input: ConstPtr<ConstPtr<RAWINPUT>>,
        n_input: i32,
        cb_size_header: u32,
    ) -> super::super::Foundation::LRESULT {
        todo!("DefRawInputProc")
    }
    #[doc = "GetRawInputBuffer from USER32"]
    fn GetRawInputBuffer(
        &self,
        p_data: MutPtr<RAWINPUT>,
        pcb_size: MutPtr<u32>,
        cb_size_header: u32,
    ) -> u32 {
        todo!("GetRawInputBuffer")
    }
    #[doc = "GetRawInputData from USER32"]
    fn GetRawInputData(
        &self,
        h_raw_input: HRAWINPUT,
        ui_command: RAW_INPUT_DATA_COMMAND_FLAGS,
        p_data: MutPtr<::core::ffi::c_void>,
        pcb_size: MutPtr<u32>,
        cb_size_header: u32,
    ) -> u32 {
        todo!("GetRawInputData")
    }
    #[doc = "GetRawInputDeviceInfoA from USER32"]
    fn GetRawInputDeviceInfoA(
        &self,
        h_device: super::super::Foundation::HANDLE,
        ui_command: RAW_INPUT_DEVICE_INFO_COMMAND,
        p_data: MutPtr<::core::ffi::c_void>,
        pcb_size: MutPtr<u32>,
    ) -> u32 {
        todo!("GetRawInputDeviceInfoA")
    }
    #[doc = "GetRawInputDeviceInfoW from USER32"]
    fn GetRawInputDeviceInfoW(
        &self,
        h_device: super::super::Foundation::HANDLE,
        ui_command: RAW_INPUT_DEVICE_INFO_COMMAND,
        p_data: MutPtr<::core::ffi::c_void>,
        pcb_size: MutPtr<u32>,
    ) -> u32 {
        todo!("GetRawInputDeviceInfoW")
    }
    #[doc = "GetRawInputDeviceList from USER32"]
    fn GetRawInputDeviceList(
        &self,
        p_raw_input_device_list: MutPtr<RAWINPUTDEVICELIST>,
        pui_num_devices: MutPtr<u32>,
        cb_size: u32,
    ) -> u32 {
        todo!("GetRawInputDeviceList")
    }
    #[doc = "GetRegisteredRawInputDevices from USER32"]
    fn GetRegisteredRawInputDevices(
        &self,
        p_raw_input_devices: MutPtr<RAWINPUTDEVICE>,
        pui_num_devices: MutPtr<u32>,
        cb_size: u32,
    ) -> u32 {
        todo!("GetRegisteredRawInputDevices")
    }
    #[doc = "RegisterRawInputDevices from USER32"]
    fn RegisterRawInputDevices(
        &self,
        p_raw_input_devices: ConstPtr<RAWINPUTDEVICE>,
        ui_num_devices: u32,
        cb_size: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("RegisterRawInputDevices")
    }
}
pub fn get_api(ctx: &crate::core::Win32Context) -> std::sync::Arc<dyn Api> {
    ctx.get::<dyn Api>()
}
