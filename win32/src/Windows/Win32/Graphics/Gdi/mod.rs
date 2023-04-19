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
pub struct ABC {
    pub abcA: i32,
    pub abcB: u32,
    pub abcC: i32,
}
impl ::core::marker::Copy for ABC {}
impl ::core::clone::Clone for ABC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ABC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ABC")
            .field("abcA", &self.abcA)
            .field("abcB", &self.abcB)
            .field("abcC", &self.abcC)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ABC {
    fn eq(&self, other: &Self) -> bool {
        self.abcA == other.abcA && self.abcB == other.abcB && self.abcC == other.abcC
    }
}
impl ::core::cmp::Eq for ABC {}
impl FromIntoMemory for ABC {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_abcA = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_abcB = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_abcC = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            abcA: f_abcA,
            abcB: f_abcB,
            abcC: f_abcC,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.abcA, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.abcB, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.abcC, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub const ABORTDOC: u32 = 2u32;
pub struct ABORTPATH {
    pub emr: EMR,
}
impl ::core::marker::Copy for ABORTPATH {}
impl ::core::clone::Clone for ABORTPATH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ABORTPATH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ABORTPATH").field("emr", &self.emr).finish()
    }
}
impl ::core::cmp::PartialEq for ABORTPATH {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr
    }
}
impl ::core::cmp::Eq for ABORTPATH {}
impl FromIntoMemory for ABORTPATH {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        Self { emr: f_emr }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
    }
    fn size() -> usize {
        8
    }
}
pub const ABSOLUTE: u32 = 1u32;
pub const AC_SRC_ALPHA: u32 = 1u32;
pub const AC_SRC_OVER: u32 = 0u32;
pub const ANSI_CHARSET: u32 = 0u32;
pub const ARABIC_CHARSET: u32 = 178u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ARC_DIRECTION(pub u32);
pub const AD_COUNTERCLOCKWISE: ARC_DIRECTION = ARC_DIRECTION(1u32);
pub const AD_CLOCKWISE: ARC_DIRECTION = ARC_DIRECTION(2u32);
impl ::core::marker::Copy for ARC_DIRECTION {}
impl ::core::clone::Clone for ARC_DIRECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ARC_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ARC_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ARC_DIRECTION").field(&self.0).finish()
    }
}
impl FromIntoMemory for ARC_DIRECTION {
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
pub const ASPECT_FILTERING: u32 = 1u32;
pub struct AXESLISTA {
    pub axlReserved: u32,
    pub axlNumAxes: u32,
    pub axlAxisInfo: [AXISINFOA; 16],
}
impl ::core::marker::Copy for AXESLISTA {}
impl ::core::clone::Clone for AXESLISTA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AXESLISTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AXESLISTA")
            .field("axlReserved", &self.axlReserved)
            .field("axlNumAxes", &self.axlNumAxes)
            .field("axlAxisInfo", &self.axlAxisInfo)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AXESLISTA {
    fn eq(&self, other: &Self) -> bool {
        self.axlReserved == other.axlReserved
            && self.axlNumAxes == other.axlNumAxes
            && self.axlAxisInfo == other.axlAxisInfo
    }
}
impl ::core::cmp::Eq for AXESLISTA {}
impl FromIntoMemory for AXESLISTA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 392);
        let f_axlReserved = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_axlNumAxes = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_axlAxisInfo = <[AXISINFOA; 16] as FromIntoMemory>::from_bytes(&from[8..8 + 384]);
        Self {
            axlReserved: f_axlReserved,
            axlNumAxes: f_axlNumAxes,
            axlAxisInfo: f_axlAxisInfo,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 392);
        FromIntoMemory::into_bytes(self.axlReserved, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.axlNumAxes, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.axlAxisInfo, &mut into[8..8 + 384]);
    }
    fn size() -> usize {
        392
    }
}
pub struct AXESLISTW {
    pub axlReserved: u32,
    pub axlNumAxes: u32,
    pub axlAxisInfo: [AXISINFOW; 16],
}
impl ::core::marker::Copy for AXESLISTW {}
impl ::core::clone::Clone for AXESLISTW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AXESLISTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AXESLISTW")
            .field("axlReserved", &self.axlReserved)
            .field("axlNumAxes", &self.axlNumAxes)
            .field("axlAxisInfo", &self.axlAxisInfo)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AXESLISTW {
    fn eq(&self, other: &Self) -> bool {
        self.axlReserved == other.axlReserved
            && self.axlNumAxes == other.axlNumAxes
            && self.axlAxisInfo == other.axlAxisInfo
    }
}
impl ::core::cmp::Eq for AXESLISTW {}
impl FromIntoMemory for AXESLISTW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 392);
        let f_axlReserved = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_axlNumAxes = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_axlAxisInfo = <[AXISINFOW; 16] as FromIntoMemory>::from_bytes(&from[8..8 + 384]);
        Self {
            axlReserved: f_axlReserved,
            axlNumAxes: f_axlNumAxes,
            axlAxisInfo: f_axlAxisInfo,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 392);
        FromIntoMemory::into_bytes(self.axlReserved, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.axlNumAxes, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.axlAxisInfo, &mut into[8..8 + 384]);
    }
    fn size() -> usize {
        392
    }
}
pub struct AXISINFOA {
    pub axMinValue: i32,
    pub axMaxValue: i32,
    pub axAxisName: [u8; 16],
}
impl ::core::marker::Copy for AXISINFOA {}
impl ::core::clone::Clone for AXISINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AXISINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AXISINFOA")
            .field("axMinValue", &self.axMinValue)
            .field("axMaxValue", &self.axMaxValue)
            .field("axAxisName", &self.axAxisName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AXISINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.axMinValue == other.axMinValue
            && self.axMaxValue == other.axMaxValue
            && self.axAxisName == other.axAxisName
    }
}
impl ::core::cmp::Eq for AXISINFOA {}
impl FromIntoMemory for AXISINFOA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_axMinValue = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_axMaxValue = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_axAxisName = <[u8; 16] as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        Self {
            axMinValue: f_axMinValue,
            axMaxValue: f_axMaxValue,
            axAxisName: f_axAxisName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.axMinValue, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.axMaxValue, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.axAxisName, &mut into[8..8 + 16]);
    }
    fn size() -> usize {
        24
    }
}
pub struct AXISINFOW {
    pub axMinValue: i32,
    pub axMaxValue: i32,
    pub axAxisName: [u16; 16],
}
impl ::core::marker::Copy for AXISINFOW {}
impl ::core::clone::Clone for AXISINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AXISINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AXISINFOW")
            .field("axMinValue", &self.axMinValue)
            .field("axMaxValue", &self.axMaxValue)
            .field("axAxisName", &self.axAxisName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AXISINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.axMinValue == other.axMinValue
            && self.axMaxValue == other.axMaxValue
            && self.axAxisName == other.axAxisName
    }
}
impl ::core::cmp::Eq for AXISINFOW {}
impl FromIntoMemory for AXISINFOW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_axMinValue = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_axMaxValue = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_axAxisName = <[u16; 16] as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        Self {
            axMinValue: f_axMinValue,
            axMaxValue: f_axMaxValue,
            axAxisName: f_axAxisName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.axMinValue, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.axMaxValue, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.axAxisName, &mut into[8..8 + 16]);
    }
    fn size() -> usize {
        24
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BACKGROUND_MODE(pub u32);
pub const OPAQUE: BACKGROUND_MODE = BACKGROUND_MODE(2u32);
pub const TRANSPARENT: BACKGROUND_MODE = BACKGROUND_MODE(1u32);
impl ::core::marker::Copy for BACKGROUND_MODE {}
impl ::core::clone::Clone for BACKGROUND_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BACKGROUND_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BACKGROUND_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BACKGROUND_MODE").field(&self.0).finish()
    }
}
impl FromIntoMemory for BACKGROUND_MODE {
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
pub const BALTIC_CHARSET: u32 = 186u32;
pub const BANDINFO: u32 = 24u32;
pub const BEGIN_PATH: u32 = 4096u32;
pub struct BITMAP {
    pub bmType: i32,
    pub bmWidth: i32,
    pub bmHeight: i32,
    pub bmWidthBytes: i32,
    pub bmPlanes: u16,
    pub bmBitsPixel: u16,
    pub bmBits: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for BITMAP {}
impl ::core::clone::Clone for BITMAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BITMAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BITMAP")
            .field("bmType", &self.bmType)
            .field("bmWidth", &self.bmWidth)
            .field("bmHeight", &self.bmHeight)
            .field("bmWidthBytes", &self.bmWidthBytes)
            .field("bmPlanes", &self.bmPlanes)
            .field("bmBitsPixel", &self.bmBitsPixel)
            .field("bmBits", &self.bmBits)
            .finish()
    }
}
impl ::core::cmp::PartialEq for BITMAP {
    fn eq(&self, other: &Self) -> bool {
        self.bmType == other.bmType
            && self.bmWidth == other.bmWidth
            && self.bmHeight == other.bmHeight
            && self.bmWidthBytes == other.bmWidthBytes
            && self.bmPlanes == other.bmPlanes
            && self.bmBitsPixel == other.bmBitsPixel
            && self.bmBits == other.bmBits
    }
}
impl ::core::cmp::Eq for BITMAP {}
impl FromIntoMemory for BITMAP {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_bmType = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_bmWidth = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_bmHeight = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_bmWidthBytes = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_bmPlanes = <u16 as FromIntoMemory>::from_bytes(&from[16..16 + 2]);
        let f_bmBitsPixel = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        let f_bmBits =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            bmType: f_bmType,
            bmWidth: f_bmWidth,
            bmHeight: f_bmHeight,
            bmWidthBytes: f_bmWidthBytes,
            bmPlanes: f_bmPlanes,
            bmBitsPixel: f_bmBitsPixel,
            bmBits: f_bmBits,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.bmType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.bmWidth, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.bmHeight, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.bmWidthBytes, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.bmPlanes, &mut into[16..16 + 2]);
        FromIntoMemory::into_bytes(self.bmBitsPixel, &mut into[18..18 + 2]);
        FromIntoMemory::into_bytes(self.bmBits, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct BITMAPCOREHEADER {
    pub bcSize: u32,
    pub bcWidth: u16,
    pub bcHeight: u16,
    pub bcPlanes: u16,
    pub bcBitCount: u16,
}
impl ::core::marker::Copy for BITMAPCOREHEADER {}
impl ::core::clone::Clone for BITMAPCOREHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BITMAPCOREHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BITMAPCOREHEADER")
            .field("bcSize", &self.bcSize)
            .field("bcWidth", &self.bcWidth)
            .field("bcHeight", &self.bcHeight)
            .field("bcPlanes", &self.bcPlanes)
            .field("bcBitCount", &self.bcBitCount)
            .finish()
    }
}
impl ::core::cmp::PartialEq for BITMAPCOREHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.bcSize == other.bcSize
            && self.bcWidth == other.bcWidth
            && self.bcHeight == other.bcHeight
            && self.bcPlanes == other.bcPlanes
            && self.bcBitCount == other.bcBitCount
    }
}
impl ::core::cmp::Eq for BITMAPCOREHEADER {}
impl FromIntoMemory for BITMAPCOREHEADER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_bcSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_bcWidth = <u16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_bcHeight = <u16 as FromIntoMemory>::from_bytes(&from[6..6 + 2]);
        let f_bcPlanes = <u16 as FromIntoMemory>::from_bytes(&from[8..8 + 2]);
        let f_bcBitCount = <u16 as FromIntoMemory>::from_bytes(&from[10..10 + 2]);
        Self {
            bcSize: f_bcSize,
            bcWidth: f_bcWidth,
            bcHeight: f_bcHeight,
            bcPlanes: f_bcPlanes,
            bcBitCount: f_bcBitCount,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.bcSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.bcWidth, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.bcHeight, &mut into[6..6 + 2]);
        FromIntoMemory::into_bytes(self.bcPlanes, &mut into[8..8 + 2]);
        FromIntoMemory::into_bytes(self.bcBitCount, &mut into[10..10 + 2]);
    }
    fn size() -> usize {
        12
    }
}
pub struct BITMAPCOREINFO {
    pub bmciHeader: BITMAPCOREHEADER,
    pub bmciColors: [RGBTRIPLE; 1],
}
impl ::core::marker::Copy for BITMAPCOREINFO {}
impl ::core::clone::Clone for BITMAPCOREINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BITMAPCOREINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BITMAPCOREINFO")
            .field("bmciHeader", &self.bmciHeader)
            .field("bmciColors", &self.bmciColors)
            .finish()
    }
}
impl ::core::cmp::PartialEq for BITMAPCOREINFO {
    fn eq(&self, other: &Self) -> bool {
        self.bmciHeader == other.bmciHeader && self.bmciColors == other.bmciColors
    }
}
impl ::core::cmp::Eq for BITMAPCOREINFO {}
impl FromIntoMemory for BITMAPCOREINFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_bmciHeader = <BITMAPCOREHEADER as FromIntoMemory>::from_bytes(&from[0..0 + 12]);
        let f_bmciColors = <[RGBTRIPLE; 1] as FromIntoMemory>::from_bytes(&from[12..12 + 3]);
        Self {
            bmciHeader: f_bmciHeader,
            bmciColors: f_bmciColors,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.bmciHeader, &mut into[0..0 + 12]);
        FromIntoMemory::into_bytes(self.bmciColors, &mut into[12..12 + 3]);
    }
    fn size() -> usize {
        16
    }
}
pub struct BITMAPINFO {
    pub bmiHeader: BITMAPINFOHEADER,
    pub bmiColors: [RGBQUAD; 1],
}
impl ::core::marker::Copy for BITMAPINFO {}
impl ::core::clone::Clone for BITMAPINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BITMAPINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BITMAPINFO")
            .field("bmiHeader", &self.bmiHeader)
            .field("bmiColors", &self.bmiColors)
            .finish()
    }
}
impl ::core::cmp::PartialEq for BITMAPINFO {
    fn eq(&self, other: &Self) -> bool {
        self.bmiHeader == other.bmiHeader && self.bmiColors == other.bmiColors
    }
}
impl ::core::cmp::Eq for BITMAPINFO {}
impl FromIntoMemory for BITMAPINFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 44);
        let f_bmiHeader = <BITMAPINFOHEADER as FromIntoMemory>::from_bytes(&from[0..0 + 40]);
        let f_bmiColors = <[RGBQUAD; 1] as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        Self {
            bmiHeader: f_bmiHeader,
            bmiColors: f_bmiColors,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 44);
        FromIntoMemory::into_bytes(self.bmiHeader, &mut into[0..0 + 40]);
        FromIntoMemory::into_bytes(self.bmiColors, &mut into[40..40 + 4]);
    }
    fn size() -> usize {
        44
    }
}
pub struct BITMAPINFOHEADER {
    pub biSize: u32,
    pub biWidth: i32,
    pub biHeight: i32,
    pub biPlanes: u16,
    pub biBitCount: u16,
    pub biCompression: u32,
    pub biSizeImage: u32,
    pub biXPelsPerMeter: i32,
    pub biYPelsPerMeter: i32,
    pub biClrUsed: u32,
    pub biClrImportant: u32,
}
impl ::core::marker::Copy for BITMAPINFOHEADER {}
impl ::core::clone::Clone for BITMAPINFOHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BITMAPINFOHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BITMAPINFOHEADER")
            .field("biSize", &self.biSize)
            .field("biWidth", &self.biWidth)
            .field("biHeight", &self.biHeight)
            .field("biPlanes", &self.biPlanes)
            .field("biBitCount", &self.biBitCount)
            .field("biCompression", &self.biCompression)
            .field("biSizeImage", &self.biSizeImage)
            .field("biXPelsPerMeter", &self.biXPelsPerMeter)
            .field("biYPelsPerMeter", &self.biYPelsPerMeter)
            .field("biClrUsed", &self.biClrUsed)
            .field("biClrImportant", &self.biClrImportant)
            .finish()
    }
}
impl ::core::cmp::PartialEq for BITMAPINFOHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.biSize == other.biSize
            && self.biWidth == other.biWidth
            && self.biHeight == other.biHeight
            && self.biPlanes == other.biPlanes
            && self.biBitCount == other.biBitCount
            && self.biCompression == other.biCompression
            && self.biSizeImage == other.biSizeImage
            && self.biXPelsPerMeter == other.biXPelsPerMeter
            && self.biYPelsPerMeter == other.biYPelsPerMeter
            && self.biClrUsed == other.biClrUsed
            && self.biClrImportant == other.biClrImportant
    }
}
impl ::core::cmp::Eq for BITMAPINFOHEADER {}
impl FromIntoMemory for BITMAPINFOHEADER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_biSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_biWidth = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_biHeight = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_biPlanes = <u16 as FromIntoMemory>::from_bytes(&from[12..12 + 2]);
        let f_biBitCount = <u16 as FromIntoMemory>::from_bytes(&from[14..14 + 2]);
        let f_biCompression = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_biSizeImage = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_biXPelsPerMeter = <i32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_biYPelsPerMeter = <i32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_biClrUsed = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_biClrImportant = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        Self {
            biSize: f_biSize,
            biWidth: f_biWidth,
            biHeight: f_biHeight,
            biPlanes: f_biPlanes,
            biBitCount: f_biBitCount,
            biCompression: f_biCompression,
            biSizeImage: f_biSizeImage,
            biXPelsPerMeter: f_biXPelsPerMeter,
            biYPelsPerMeter: f_biYPelsPerMeter,
            biClrUsed: f_biClrUsed,
            biClrImportant: f_biClrImportant,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.biSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.biWidth, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.biHeight, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.biPlanes, &mut into[12..12 + 2]);
        FromIntoMemory::into_bytes(self.biBitCount, &mut into[14..14 + 2]);
        FromIntoMemory::into_bytes(self.biCompression, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.biSizeImage, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.biXPelsPerMeter, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.biYPelsPerMeter, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.biClrUsed, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.biClrImportant, &mut into[36..36 + 4]);
    }
    fn size() -> usize {
        40
    }
}
pub struct BITMAPV4HEADER {
    pub bV4Size: u32,
    pub bV4Width: i32,
    pub bV4Height: i32,
    pub bV4Planes: u16,
    pub bV4BitCount: u16,
    pub bV4V4Compression: u32,
    pub bV4SizeImage: u32,
    pub bV4XPelsPerMeter: i32,
    pub bV4YPelsPerMeter: i32,
    pub bV4ClrUsed: u32,
    pub bV4ClrImportant: u32,
    pub bV4RedMask: u32,
    pub bV4GreenMask: u32,
    pub bV4BlueMask: u32,
    pub bV4AlphaMask: u32,
    pub bV4CSType: u32,
    pub bV4Endpoints: CIEXYZTRIPLE,
    pub bV4GammaRed: u32,
    pub bV4GammaGreen: u32,
    pub bV4GammaBlue: u32,
}
impl ::core::marker::Copy for BITMAPV4HEADER {}
impl ::core::clone::Clone for BITMAPV4HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BITMAPV4HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BITMAPV4HEADER")
            .field("bV4Size", &self.bV4Size)
            .field("bV4Width", &self.bV4Width)
            .field("bV4Height", &self.bV4Height)
            .field("bV4Planes", &self.bV4Planes)
            .field("bV4BitCount", &self.bV4BitCount)
            .field("bV4V4Compression", &self.bV4V4Compression)
            .field("bV4SizeImage", &self.bV4SizeImage)
            .field("bV4XPelsPerMeter", &self.bV4XPelsPerMeter)
            .field("bV4YPelsPerMeter", &self.bV4YPelsPerMeter)
            .field("bV4ClrUsed", &self.bV4ClrUsed)
            .field("bV4ClrImportant", &self.bV4ClrImportant)
            .field("bV4RedMask", &self.bV4RedMask)
            .field("bV4GreenMask", &self.bV4GreenMask)
            .field("bV4BlueMask", &self.bV4BlueMask)
            .field("bV4AlphaMask", &self.bV4AlphaMask)
            .field("bV4CSType", &self.bV4CSType)
            .field("bV4Endpoints", &self.bV4Endpoints)
            .field("bV4GammaRed", &self.bV4GammaRed)
            .field("bV4GammaGreen", &self.bV4GammaGreen)
            .field("bV4GammaBlue", &self.bV4GammaBlue)
            .finish()
    }
}
impl ::core::cmp::PartialEq for BITMAPV4HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.bV4Size == other.bV4Size
            && self.bV4Width == other.bV4Width
            && self.bV4Height == other.bV4Height
            && self.bV4Planes == other.bV4Planes
            && self.bV4BitCount == other.bV4BitCount
            && self.bV4V4Compression == other.bV4V4Compression
            && self.bV4SizeImage == other.bV4SizeImage
            && self.bV4XPelsPerMeter == other.bV4XPelsPerMeter
            && self.bV4YPelsPerMeter == other.bV4YPelsPerMeter
            && self.bV4ClrUsed == other.bV4ClrUsed
            && self.bV4ClrImportant == other.bV4ClrImportant
            && self.bV4RedMask == other.bV4RedMask
            && self.bV4GreenMask == other.bV4GreenMask
            && self.bV4BlueMask == other.bV4BlueMask
            && self.bV4AlphaMask == other.bV4AlphaMask
            && self.bV4CSType == other.bV4CSType
            && self.bV4Endpoints == other.bV4Endpoints
            && self.bV4GammaRed == other.bV4GammaRed
            && self.bV4GammaGreen == other.bV4GammaGreen
            && self.bV4GammaBlue == other.bV4GammaBlue
    }
}
impl ::core::cmp::Eq for BITMAPV4HEADER {}
impl FromIntoMemory for BITMAPV4HEADER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 108);
        let f_bV4Size = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_bV4Width = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_bV4Height = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_bV4Planes = <u16 as FromIntoMemory>::from_bytes(&from[12..12 + 2]);
        let f_bV4BitCount = <u16 as FromIntoMemory>::from_bytes(&from[14..14 + 2]);
        let f_bV4V4Compression = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_bV4SizeImage = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_bV4XPelsPerMeter = <i32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_bV4YPelsPerMeter = <i32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_bV4ClrUsed = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_bV4ClrImportant = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_bV4RedMask = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_bV4GreenMask = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_bV4BlueMask = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_bV4AlphaMask = <u32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_bV4CSType = <u32 as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_bV4Endpoints = <CIEXYZTRIPLE as FromIntoMemory>::from_bytes(&from[60..60 + 36]);
        let f_bV4GammaRed = <u32 as FromIntoMemory>::from_bytes(&from[96..96 + 4]);
        let f_bV4GammaGreen = <u32 as FromIntoMemory>::from_bytes(&from[100..100 + 4]);
        let f_bV4GammaBlue = <u32 as FromIntoMemory>::from_bytes(&from[104..104 + 4]);
        Self {
            bV4Size: f_bV4Size,
            bV4Width: f_bV4Width,
            bV4Height: f_bV4Height,
            bV4Planes: f_bV4Planes,
            bV4BitCount: f_bV4BitCount,
            bV4V4Compression: f_bV4V4Compression,
            bV4SizeImage: f_bV4SizeImage,
            bV4XPelsPerMeter: f_bV4XPelsPerMeter,
            bV4YPelsPerMeter: f_bV4YPelsPerMeter,
            bV4ClrUsed: f_bV4ClrUsed,
            bV4ClrImportant: f_bV4ClrImportant,
            bV4RedMask: f_bV4RedMask,
            bV4GreenMask: f_bV4GreenMask,
            bV4BlueMask: f_bV4BlueMask,
            bV4AlphaMask: f_bV4AlphaMask,
            bV4CSType: f_bV4CSType,
            bV4Endpoints: f_bV4Endpoints,
            bV4GammaRed: f_bV4GammaRed,
            bV4GammaGreen: f_bV4GammaGreen,
            bV4GammaBlue: f_bV4GammaBlue,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 108);
        FromIntoMemory::into_bytes(self.bV4Size, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.bV4Width, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.bV4Height, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.bV4Planes, &mut into[12..12 + 2]);
        FromIntoMemory::into_bytes(self.bV4BitCount, &mut into[14..14 + 2]);
        FromIntoMemory::into_bytes(self.bV4V4Compression, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.bV4SizeImage, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.bV4XPelsPerMeter, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.bV4YPelsPerMeter, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.bV4ClrUsed, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.bV4ClrImportant, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.bV4RedMask, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.bV4GreenMask, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.bV4BlueMask, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.bV4AlphaMask, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.bV4CSType, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.bV4Endpoints, &mut into[60..60 + 36]);
        FromIntoMemory::into_bytes(self.bV4GammaRed, &mut into[96..96 + 4]);
        FromIntoMemory::into_bytes(self.bV4GammaGreen, &mut into[100..100 + 4]);
        FromIntoMemory::into_bytes(self.bV4GammaBlue, &mut into[104..104 + 4]);
    }
    fn size() -> usize {
        108
    }
}
pub struct BITMAPV5HEADER {
    pub bV5Size: u32,
    pub bV5Width: i32,
    pub bV5Height: i32,
    pub bV5Planes: u16,
    pub bV5BitCount: u16,
    pub bV5Compression: u32,
    pub bV5SizeImage: u32,
    pub bV5XPelsPerMeter: i32,
    pub bV5YPelsPerMeter: i32,
    pub bV5ClrUsed: u32,
    pub bV5ClrImportant: u32,
    pub bV5RedMask: u32,
    pub bV5GreenMask: u32,
    pub bV5BlueMask: u32,
    pub bV5AlphaMask: u32,
    pub bV5CSType: u32,
    pub bV5Endpoints: CIEXYZTRIPLE,
    pub bV5GammaRed: u32,
    pub bV5GammaGreen: u32,
    pub bV5GammaBlue: u32,
    pub bV5Intent: u32,
    pub bV5ProfileData: u32,
    pub bV5ProfileSize: u32,
    pub bV5Reserved: u32,
}
impl ::core::marker::Copy for BITMAPV5HEADER {}
impl ::core::clone::Clone for BITMAPV5HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BITMAPV5HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BITMAPV5HEADER")
            .field("bV5Size", &self.bV5Size)
            .field("bV5Width", &self.bV5Width)
            .field("bV5Height", &self.bV5Height)
            .field("bV5Planes", &self.bV5Planes)
            .field("bV5BitCount", &self.bV5BitCount)
            .field("bV5Compression", &self.bV5Compression)
            .field("bV5SizeImage", &self.bV5SizeImage)
            .field("bV5XPelsPerMeter", &self.bV5XPelsPerMeter)
            .field("bV5YPelsPerMeter", &self.bV5YPelsPerMeter)
            .field("bV5ClrUsed", &self.bV5ClrUsed)
            .field("bV5ClrImportant", &self.bV5ClrImportant)
            .field("bV5RedMask", &self.bV5RedMask)
            .field("bV5GreenMask", &self.bV5GreenMask)
            .field("bV5BlueMask", &self.bV5BlueMask)
            .field("bV5AlphaMask", &self.bV5AlphaMask)
            .field("bV5CSType", &self.bV5CSType)
            .field("bV5Endpoints", &self.bV5Endpoints)
            .field("bV5GammaRed", &self.bV5GammaRed)
            .field("bV5GammaGreen", &self.bV5GammaGreen)
            .field("bV5GammaBlue", &self.bV5GammaBlue)
            .field("bV5Intent", &self.bV5Intent)
            .field("bV5ProfileData", &self.bV5ProfileData)
            .field("bV5ProfileSize", &self.bV5ProfileSize)
            .field("bV5Reserved", &self.bV5Reserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for BITMAPV5HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.bV5Size == other.bV5Size
            && self.bV5Width == other.bV5Width
            && self.bV5Height == other.bV5Height
            && self.bV5Planes == other.bV5Planes
            && self.bV5BitCount == other.bV5BitCount
            && self.bV5Compression == other.bV5Compression
            && self.bV5SizeImage == other.bV5SizeImage
            && self.bV5XPelsPerMeter == other.bV5XPelsPerMeter
            && self.bV5YPelsPerMeter == other.bV5YPelsPerMeter
            && self.bV5ClrUsed == other.bV5ClrUsed
            && self.bV5ClrImportant == other.bV5ClrImportant
            && self.bV5RedMask == other.bV5RedMask
            && self.bV5GreenMask == other.bV5GreenMask
            && self.bV5BlueMask == other.bV5BlueMask
            && self.bV5AlphaMask == other.bV5AlphaMask
            && self.bV5CSType == other.bV5CSType
            && self.bV5Endpoints == other.bV5Endpoints
            && self.bV5GammaRed == other.bV5GammaRed
            && self.bV5GammaGreen == other.bV5GammaGreen
            && self.bV5GammaBlue == other.bV5GammaBlue
            && self.bV5Intent == other.bV5Intent
            && self.bV5ProfileData == other.bV5ProfileData
            && self.bV5ProfileSize == other.bV5ProfileSize
            && self.bV5Reserved == other.bV5Reserved
    }
}
impl ::core::cmp::Eq for BITMAPV5HEADER {}
impl FromIntoMemory for BITMAPV5HEADER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 124);
        let f_bV5Size = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_bV5Width = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_bV5Height = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_bV5Planes = <u16 as FromIntoMemory>::from_bytes(&from[12..12 + 2]);
        let f_bV5BitCount = <u16 as FromIntoMemory>::from_bytes(&from[14..14 + 2]);
        let f_bV5Compression = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_bV5SizeImage = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_bV5XPelsPerMeter = <i32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_bV5YPelsPerMeter = <i32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_bV5ClrUsed = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_bV5ClrImportant = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_bV5RedMask = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_bV5GreenMask = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_bV5BlueMask = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_bV5AlphaMask = <u32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_bV5CSType = <u32 as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_bV5Endpoints = <CIEXYZTRIPLE as FromIntoMemory>::from_bytes(&from[60..60 + 36]);
        let f_bV5GammaRed = <u32 as FromIntoMemory>::from_bytes(&from[96..96 + 4]);
        let f_bV5GammaGreen = <u32 as FromIntoMemory>::from_bytes(&from[100..100 + 4]);
        let f_bV5GammaBlue = <u32 as FromIntoMemory>::from_bytes(&from[104..104 + 4]);
        let f_bV5Intent = <u32 as FromIntoMemory>::from_bytes(&from[108..108 + 4]);
        let f_bV5ProfileData = <u32 as FromIntoMemory>::from_bytes(&from[112..112 + 4]);
        let f_bV5ProfileSize = <u32 as FromIntoMemory>::from_bytes(&from[116..116 + 4]);
        let f_bV5Reserved = <u32 as FromIntoMemory>::from_bytes(&from[120..120 + 4]);
        Self {
            bV5Size: f_bV5Size,
            bV5Width: f_bV5Width,
            bV5Height: f_bV5Height,
            bV5Planes: f_bV5Planes,
            bV5BitCount: f_bV5BitCount,
            bV5Compression: f_bV5Compression,
            bV5SizeImage: f_bV5SizeImage,
            bV5XPelsPerMeter: f_bV5XPelsPerMeter,
            bV5YPelsPerMeter: f_bV5YPelsPerMeter,
            bV5ClrUsed: f_bV5ClrUsed,
            bV5ClrImportant: f_bV5ClrImportant,
            bV5RedMask: f_bV5RedMask,
            bV5GreenMask: f_bV5GreenMask,
            bV5BlueMask: f_bV5BlueMask,
            bV5AlphaMask: f_bV5AlphaMask,
            bV5CSType: f_bV5CSType,
            bV5Endpoints: f_bV5Endpoints,
            bV5GammaRed: f_bV5GammaRed,
            bV5GammaGreen: f_bV5GammaGreen,
            bV5GammaBlue: f_bV5GammaBlue,
            bV5Intent: f_bV5Intent,
            bV5ProfileData: f_bV5ProfileData,
            bV5ProfileSize: f_bV5ProfileSize,
            bV5Reserved: f_bV5Reserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 124);
        FromIntoMemory::into_bytes(self.bV5Size, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.bV5Width, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.bV5Height, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.bV5Planes, &mut into[12..12 + 2]);
        FromIntoMemory::into_bytes(self.bV5BitCount, &mut into[14..14 + 2]);
        FromIntoMemory::into_bytes(self.bV5Compression, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.bV5SizeImage, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.bV5XPelsPerMeter, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.bV5YPelsPerMeter, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.bV5ClrUsed, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.bV5ClrImportant, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.bV5RedMask, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.bV5GreenMask, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.bV5BlueMask, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.bV5AlphaMask, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.bV5CSType, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.bV5Endpoints, &mut into[60..60 + 36]);
        FromIntoMemory::into_bytes(self.bV5GammaRed, &mut into[96..96 + 4]);
        FromIntoMemory::into_bytes(self.bV5GammaGreen, &mut into[100..100 + 4]);
        FromIntoMemory::into_bytes(self.bV5GammaBlue, &mut into[104..104 + 4]);
        FromIntoMemory::into_bytes(self.bV5Intent, &mut into[108..108 + 4]);
        FromIntoMemory::into_bytes(self.bV5ProfileData, &mut into[112..112 + 4]);
        FromIntoMemory::into_bytes(self.bV5ProfileSize, &mut into[116..116 + 4]);
        FromIntoMemory::into_bytes(self.bV5Reserved, &mut into[120..120 + 4]);
    }
    fn size() -> usize {
        124
    }
}
pub const BI_BITFIELDS: i32 = 3i32;
pub const BI_JPEG: i32 = 4i32;
pub const BI_PNG: i32 = 5i32;
pub const BI_RGB: i32 = 0i32;
pub const BI_RLE4: i32 = 2i32;
pub const BI_RLE8: i32 = 1i32;
pub const BKMODE_LAST: u32 = 2u32;
pub struct BLENDFUNCTION {
    pub BlendOp: u8,
    pub BlendFlags: u8,
    pub SourceConstantAlpha: u8,
    pub AlphaFormat: u8,
}
impl ::core::marker::Copy for BLENDFUNCTION {}
impl ::core::clone::Clone for BLENDFUNCTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BLENDFUNCTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BLENDFUNCTION")
            .field("BlendOp", &self.BlendOp)
            .field("BlendFlags", &self.BlendFlags)
            .field("SourceConstantAlpha", &self.SourceConstantAlpha)
            .field("AlphaFormat", &self.AlphaFormat)
            .finish()
    }
}
impl ::core::cmp::PartialEq for BLENDFUNCTION {
    fn eq(&self, other: &Self) -> bool {
        self.BlendOp == other.BlendOp
            && self.BlendFlags == other.BlendFlags
            && self.SourceConstantAlpha == other.SourceConstantAlpha
            && self.AlphaFormat == other.AlphaFormat
    }
}
impl ::core::cmp::Eq for BLENDFUNCTION {}
impl FromIntoMemory for BLENDFUNCTION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_BlendOp = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_BlendFlags = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_SourceConstantAlpha = <u8 as FromIntoMemory>::from_bytes(&from[2..2 + 1]);
        let f_AlphaFormat = <u8 as FromIntoMemory>::from_bytes(&from[3..3 + 1]);
        Self {
            BlendOp: f_BlendOp,
            BlendFlags: f_BlendFlags,
            SourceConstantAlpha: f_SourceConstantAlpha,
            AlphaFormat: f_AlphaFormat,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.BlendOp, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.BlendFlags, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.SourceConstantAlpha, &mut into[2..2 + 1]);
        FromIntoMemory::into_bytes(self.AlphaFormat, &mut into[3..3 + 1]);
    }
    fn size() -> usize {
        4
    }
}
pub const BS_DIBPATTERN: u32 = 5u32;
pub const BS_DIBPATTERN8X8: u32 = 8u32;
pub const BS_DIBPATTERNPT: u32 = 6u32;
pub const BS_HATCHED: u32 = 2u32;
pub const BS_HOLLOW: u32 = 1u32;
pub const BS_INDEXED: u32 = 4u32;
pub const BS_MONOPATTERN: u32 = 9u32;
pub const BS_NULL: u32 = 1u32;
pub const BS_PATTERN: u32 = 3u32;
pub const BS_PATTERN8X8: u32 = 7u32;
pub const BS_SOLID: u32 = 0u32;
pub const CA_LOG_FILTER: u32 = 2u32;
pub const CA_NEGATIVE: u32 = 1u32;
pub const CBM_INIT: i32 = 4i32;
pub const CCHFORMNAME: u32 = 32u32;
pub const CC_CHORD: u32 = 4u32;
pub const CC_CIRCLES: u32 = 1u32;
pub const CC_ELLIPSES: u32 = 8u32;
pub const CC_INTERIORS: u32 = 128u32;
pub const CC_NONE: u32 = 0u32;
pub const CC_PIE: u32 = 2u32;
pub const CC_ROUNDRECT: u32 = 256u32;
pub const CC_STYLED: u32 = 32u32;
pub const CC_WIDE: u32 = 16u32;
pub const CC_WIDESTYLED: u32 = 64u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CDS_TYPE(pub u32);
pub const CDS_FULLSCREEN: CDS_TYPE = CDS_TYPE(4u32);
pub const CDS_GLOBAL: CDS_TYPE = CDS_TYPE(8u32);
pub const CDS_NORESET: CDS_TYPE = CDS_TYPE(268435456u32);
pub const CDS_RESET: CDS_TYPE = CDS_TYPE(1073741824u32);
pub const CDS_SET_PRIMARY: CDS_TYPE = CDS_TYPE(16u32);
pub const CDS_TEST: CDS_TYPE = CDS_TYPE(2u32);
pub const CDS_UPDATEREGISTRY: CDS_TYPE = CDS_TYPE(1u32);
pub const CDS_VIDEOPARAMETERS: CDS_TYPE = CDS_TYPE(32u32);
pub const CDS_ENABLE_UNSAFE_MODES: CDS_TYPE = CDS_TYPE(256u32);
pub const CDS_DISABLE_UNSAFE_MODES: CDS_TYPE = CDS_TYPE(512u32);
pub const CDS_RESET_EX: CDS_TYPE = CDS_TYPE(536870912u32);
impl ::core::marker::Copy for CDS_TYPE {}
impl ::core::clone::Clone for CDS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CDS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CDS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CDS_TYPE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CDS_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CDS_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CDS_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CDS_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CDS_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for CDS_TYPE {
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
pub type CFP_ALLOCPROC = StdCallFnPtr<(PtrRepr,), MutPtr<::core::ffi::c_void>>;
pub type CFP_FREEPROC = StdCallFnPtr<(MutPtr<::core::ffi::c_void>,), ()>;
pub type CFP_REALLOCPROC =
    StdCallFnPtr<(MutPtr<::core::ffi::c_void>, PtrRepr), MutPtr<::core::ffi::c_void>>;
pub const CHARSET_DEFAULT: u32 = 1u32;
pub const CHARSET_GLYPHIDX: u32 = 3u32;
pub const CHECKJPEGFORMAT: u32 = 4119u32;
pub const CHECKPNGFORMAT: u32 = 4120u32;
pub const CHINESEBIG5_CHARSET: u32 = 136u32;
pub struct CIEXYZ {
    pub ciexyzX: i32,
    pub ciexyzY: i32,
    pub ciexyzZ: i32,
}
impl ::core::marker::Copy for CIEXYZ {}
impl ::core::clone::Clone for CIEXYZ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CIEXYZ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CIEXYZ")
            .field("ciexyzX", &self.ciexyzX)
            .field("ciexyzY", &self.ciexyzY)
            .field("ciexyzZ", &self.ciexyzZ)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CIEXYZ {
    fn eq(&self, other: &Self) -> bool {
        self.ciexyzX == other.ciexyzX
            && self.ciexyzY == other.ciexyzY
            && self.ciexyzZ == other.ciexyzZ
    }
}
impl ::core::cmp::Eq for CIEXYZ {}
impl FromIntoMemory for CIEXYZ {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_ciexyzX = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ciexyzY = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_ciexyzZ = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            ciexyzX: f_ciexyzX,
            ciexyzY: f_ciexyzY,
            ciexyzZ: f_ciexyzZ,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.ciexyzX, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ciexyzY, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.ciexyzZ, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct CIEXYZTRIPLE {
    pub ciexyzRed: CIEXYZ,
    pub ciexyzGreen: CIEXYZ,
    pub ciexyzBlue: CIEXYZ,
}
impl ::core::marker::Copy for CIEXYZTRIPLE {}
impl ::core::clone::Clone for CIEXYZTRIPLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CIEXYZTRIPLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CIEXYZTRIPLE")
            .field("ciexyzRed", &self.ciexyzRed)
            .field("ciexyzGreen", &self.ciexyzGreen)
            .field("ciexyzBlue", &self.ciexyzBlue)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CIEXYZTRIPLE {
    fn eq(&self, other: &Self) -> bool {
        self.ciexyzRed == other.ciexyzRed
            && self.ciexyzGreen == other.ciexyzGreen
            && self.ciexyzBlue == other.ciexyzBlue
    }
}
impl ::core::cmp::Eq for CIEXYZTRIPLE {}
impl FromIntoMemory for CIEXYZTRIPLE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 36);
        let f_ciexyzRed = <CIEXYZ as FromIntoMemory>::from_bytes(&from[0..0 + 12]);
        let f_ciexyzGreen = <CIEXYZ as FromIntoMemory>::from_bytes(&from[12..12 + 12]);
        let f_ciexyzBlue = <CIEXYZ as FromIntoMemory>::from_bytes(&from[24..24 + 12]);
        Self {
            ciexyzRed: f_ciexyzRed,
            ciexyzGreen: f_ciexyzGreen,
            ciexyzBlue: f_ciexyzBlue,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 36);
        FromIntoMemory::into_bytes(self.ciexyzRed, &mut into[0..0 + 12]);
        FromIntoMemory::into_bytes(self.ciexyzGreen, &mut into[12..12 + 12]);
        FromIntoMemory::into_bytes(self.ciexyzBlue, &mut into[24..24 + 12]);
    }
    fn size() -> usize {
        36
    }
}
pub const CLEARTYPE_NATURAL_QUALITY: u32 = 6u32;
pub const CLIP_TO_PATH: u32 = 4097u32;
pub const CLOSECHANNEL: u32 = 4112u32;
pub const CLR_INVALID: u32 = 4294967295u32;
pub const CM_CMYK_COLOR: u32 = 4u32;
pub const CM_DEVICE_ICM: u32 = 1u32;
pub const CM_GAMMA_RAMP: u32 = 2u32;
pub const CM_IN_GAMUT: u32 = 0u32;
pub const CM_NONE: u32 = 0u32;
pub const CM_OUT_OF_GAMUT: u32 = 255u32;
pub struct COLORADJUSTMENT {
    pub caSize: u16,
    pub caFlags: u16,
    pub caIlluminantIndex: u16,
    pub caRedGamma: u16,
    pub caGreenGamma: u16,
    pub caBlueGamma: u16,
    pub caReferenceBlack: u16,
    pub caReferenceWhite: u16,
    pub caContrast: i16,
    pub caBrightness: i16,
    pub caColorfulness: i16,
    pub caRedGreenTint: i16,
}
impl ::core::marker::Copy for COLORADJUSTMENT {}
impl ::core::clone::Clone for COLORADJUSTMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COLORADJUSTMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COLORADJUSTMENT")
            .field("caSize", &self.caSize)
            .field("caFlags", &self.caFlags)
            .field("caIlluminantIndex", &self.caIlluminantIndex)
            .field("caRedGamma", &self.caRedGamma)
            .field("caGreenGamma", &self.caGreenGamma)
            .field("caBlueGamma", &self.caBlueGamma)
            .field("caReferenceBlack", &self.caReferenceBlack)
            .field("caReferenceWhite", &self.caReferenceWhite)
            .field("caContrast", &self.caContrast)
            .field("caBrightness", &self.caBrightness)
            .field("caColorfulness", &self.caColorfulness)
            .field("caRedGreenTint", &self.caRedGreenTint)
            .finish()
    }
}
impl ::core::cmp::PartialEq for COLORADJUSTMENT {
    fn eq(&self, other: &Self) -> bool {
        self.caSize == other.caSize
            && self.caFlags == other.caFlags
            && self.caIlluminantIndex == other.caIlluminantIndex
            && self.caRedGamma == other.caRedGamma
            && self.caGreenGamma == other.caGreenGamma
            && self.caBlueGamma == other.caBlueGamma
            && self.caReferenceBlack == other.caReferenceBlack
            && self.caReferenceWhite == other.caReferenceWhite
            && self.caContrast == other.caContrast
            && self.caBrightness == other.caBrightness
            && self.caColorfulness == other.caColorfulness
            && self.caRedGreenTint == other.caRedGreenTint
    }
}
impl ::core::cmp::Eq for COLORADJUSTMENT {}
impl FromIntoMemory for COLORADJUSTMENT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_caSize = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_caFlags = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_caIlluminantIndex = <u16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_caRedGamma = <u16 as FromIntoMemory>::from_bytes(&from[6..6 + 2]);
        let f_caGreenGamma = <u16 as FromIntoMemory>::from_bytes(&from[8..8 + 2]);
        let f_caBlueGamma = <u16 as FromIntoMemory>::from_bytes(&from[10..10 + 2]);
        let f_caReferenceBlack = <u16 as FromIntoMemory>::from_bytes(&from[12..12 + 2]);
        let f_caReferenceWhite = <u16 as FromIntoMemory>::from_bytes(&from[14..14 + 2]);
        let f_caContrast = <i16 as FromIntoMemory>::from_bytes(&from[16..16 + 2]);
        let f_caBrightness = <i16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        let f_caColorfulness = <i16 as FromIntoMemory>::from_bytes(&from[20..20 + 2]);
        let f_caRedGreenTint = <i16 as FromIntoMemory>::from_bytes(&from[22..22 + 2]);
        Self {
            caSize: f_caSize,
            caFlags: f_caFlags,
            caIlluminantIndex: f_caIlluminantIndex,
            caRedGamma: f_caRedGamma,
            caGreenGamma: f_caGreenGamma,
            caBlueGamma: f_caBlueGamma,
            caReferenceBlack: f_caReferenceBlack,
            caReferenceWhite: f_caReferenceWhite,
            caContrast: f_caContrast,
            caBrightness: f_caBrightness,
            caColorfulness: f_caColorfulness,
            caRedGreenTint: f_caRedGreenTint,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.caSize, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.caFlags, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.caIlluminantIndex, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.caRedGamma, &mut into[6..6 + 2]);
        FromIntoMemory::into_bytes(self.caGreenGamma, &mut into[8..8 + 2]);
        FromIntoMemory::into_bytes(self.caBlueGamma, &mut into[10..10 + 2]);
        FromIntoMemory::into_bytes(self.caReferenceBlack, &mut into[12..12 + 2]);
        FromIntoMemory::into_bytes(self.caReferenceWhite, &mut into[14..14 + 2]);
        FromIntoMemory::into_bytes(self.caContrast, &mut into[16..16 + 2]);
        FromIntoMemory::into_bytes(self.caBrightness, &mut into[18..18 + 2]);
        FromIntoMemory::into_bytes(self.caColorfulness, &mut into[20..20 + 2]);
        FromIntoMemory::into_bytes(self.caRedGreenTint, &mut into[22..22 + 2]);
    }
    fn size() -> usize {
        24
    }
}
pub struct COLORCORRECTPALETTE {
    pub emr: EMR,
    pub ihPalette: u32,
    pub nFirstEntry: u32,
    pub nPalEntries: u32,
    pub nReserved: u32,
}
impl ::core::marker::Copy for COLORCORRECTPALETTE {}
impl ::core::clone::Clone for COLORCORRECTPALETTE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COLORCORRECTPALETTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COLORCORRECTPALETTE")
            .field("emr", &self.emr)
            .field("ihPalette", &self.ihPalette)
            .field("nFirstEntry", &self.nFirstEntry)
            .field("nPalEntries", &self.nPalEntries)
            .field("nReserved", &self.nReserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for COLORCORRECTPALETTE {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr
            && self.ihPalette == other.ihPalette
            && self.nFirstEntry == other.nFirstEntry
            && self.nPalEntries == other.nPalEntries
            && self.nReserved == other.nReserved
    }
}
impl ::core::cmp::Eq for COLORCORRECTPALETTE {}
impl FromIntoMemory for COLORCORRECTPALETTE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_ihPalette = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_nFirstEntry = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_nPalEntries = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_nReserved = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            emr: f_emr,
            ihPalette: f_ihPalette,
            nFirstEntry: f_nFirstEntry,
            nPalEntries: f_nPalEntries,
            nReserved: f_nReserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.ihPalette, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.nFirstEntry, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.nPalEntries, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.nReserved, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct COLORMATCHTOTARGET {
    pub emr: EMR,
    pub dwAction: u32,
    pub dwFlags: u32,
    pub cbName: u32,
    pub cbData: u32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for COLORMATCHTOTARGET {}
impl ::core::clone::Clone for COLORMATCHTOTARGET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COLORMATCHTOTARGET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COLORMATCHTOTARGET")
            .field("emr", &self.emr)
            .field("dwAction", &self.dwAction)
            .field("dwFlags", &self.dwFlags)
            .field("cbName", &self.cbName)
            .field("cbData", &self.cbData)
            .field("Data", &self.Data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for COLORMATCHTOTARGET {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr
            && self.dwAction == other.dwAction
            && self.dwFlags == other.dwFlags
            && self.cbName == other.cbName
            && self.cbData == other.cbData
            && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for COLORMATCHTOTARGET {}
impl FromIntoMemory for COLORMATCHTOTARGET {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_dwAction = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_cbName = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_cbData = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_Data = <[u8; 1] as FromIntoMemory>::from_bytes(&from[24..24 + 1]);
        Self {
            emr: f_emr,
            dwAction: f_dwAction,
            dwFlags: f_dwFlags,
            cbName: f_cbName,
            cbData: f_cbData,
            Data: f_Data,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.dwAction, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.cbName, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.cbData, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.Data, &mut into[24..24 + 1]);
    }
    fn size() -> usize {
        28
    }
}
pub const COLORMATCHTOTARGET_EMBEDED: u32 = 1u32;
pub const COMPLEXREGION: u32 = 3u32;
pub const CP_NONE: u32 = 0u32;
pub const CP_RECTANGLE: u32 = 1u32;
pub const CP_REGION: u32 = 2u32;
pub const CREATECOLORSPACE_EMBEDED: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CREATE_FONT_PACKAGE_SUBSET_ENCODING(pub u32);
pub const TTFCFP_STD_MAC_CHAR_SET: CREATE_FONT_PACKAGE_SUBSET_ENCODING =
    CREATE_FONT_PACKAGE_SUBSET_ENCODING(0u32);
pub const TTFCFP_SYMBOL_CHAR_SET: CREATE_FONT_PACKAGE_SUBSET_ENCODING =
    CREATE_FONT_PACKAGE_SUBSET_ENCODING(0u32);
pub const TTFCFP_UNICODE_CHAR_SET: CREATE_FONT_PACKAGE_SUBSET_ENCODING =
    CREATE_FONT_PACKAGE_SUBSET_ENCODING(1u32);
impl ::core::marker::Copy for CREATE_FONT_PACKAGE_SUBSET_ENCODING {}
impl ::core::clone::Clone for CREATE_FONT_PACKAGE_SUBSET_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CREATE_FONT_PACKAGE_SUBSET_ENCODING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CREATE_FONT_PACKAGE_SUBSET_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREATE_FONT_PACKAGE_SUBSET_ENCODING")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for CREATE_FONT_PACKAGE_SUBSET_ENCODING {
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
pub struct CREATE_FONT_PACKAGE_SUBSET_PLATFORM(pub u32);
pub const TTFCFP_UNICODE_PLATFORMID: CREATE_FONT_PACKAGE_SUBSET_PLATFORM =
    CREATE_FONT_PACKAGE_SUBSET_PLATFORM(0u32);
pub const TTFCFP_ISO_PLATFORMID: CREATE_FONT_PACKAGE_SUBSET_PLATFORM =
    CREATE_FONT_PACKAGE_SUBSET_PLATFORM(2u32);
impl ::core::marker::Copy for CREATE_FONT_PACKAGE_SUBSET_PLATFORM {}
impl ::core::clone::Clone for CREATE_FONT_PACKAGE_SUBSET_PLATFORM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CREATE_FONT_PACKAGE_SUBSET_PLATFORM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CREATE_FONT_PACKAGE_SUBSET_PLATFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREATE_FONT_PACKAGE_SUBSET_PLATFORM")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for CREATE_FONT_PACKAGE_SUBSET_PLATFORM {
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
pub struct CREATE_POLYGON_RGN_MODE(pub u32);
pub const ALTERNATE: CREATE_POLYGON_RGN_MODE = CREATE_POLYGON_RGN_MODE(1u32);
pub const WINDING: CREATE_POLYGON_RGN_MODE = CREATE_POLYGON_RGN_MODE(2u32);
impl ::core::marker::Copy for CREATE_POLYGON_RGN_MODE {}
impl ::core::clone::Clone for CREATE_POLYGON_RGN_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CREATE_POLYGON_RGN_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CREATE_POLYGON_RGN_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREATE_POLYGON_RGN_MODE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for CREATE_POLYGON_RGN_MODE {
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
pub struct CreatedHDC(pub PtrDiffRepr);
impl CreatedHDC {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CreatedHDC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for CreatedHDC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for CreatedHDC {}
impl ::core::hash::Hash for CreatedHDC {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for CreatedHDC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreatedHDC").field(&self.0).finish()
    }
}
impl FromIntoMemory for CreatedHDC {
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
pub const DCBA_FACEDOWNCENTER: u32 = 257u32;
pub const DCBA_FACEDOWNLEFT: u32 = 258u32;
pub const DCBA_FACEDOWNNONE: u32 = 256u32;
pub const DCBA_FACEDOWNRIGHT: u32 = 259u32;
pub const DCBA_FACEUPCENTER: u32 = 1u32;
pub const DCBA_FACEUPLEFT: u32 = 2u32;
pub const DCBA_FACEUPNONE: u32 = 0u32;
pub const DCBA_FACEUPRIGHT: u32 = 3u32;
pub const DCTT_BITMAP: i32 = 1i32;
pub const DCTT_DOWNLOAD: i32 = 2i32;
pub const DCTT_DOWNLOAD_OUTLINE: i32 = 8i32;
pub const DCTT_SUBDEV: i32 = 4i32;
pub const DC_BINADJUST: u32 = 19u32;
pub const DC_DATATYPE_PRODUCED: u32 = 21u32;
pub const DC_EMF_COMPLIANT: u32 = 20u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DC_LAYOUT(pub u32);
pub const LAYOUT_BITMAPORIENTATIONPRESERVED: DC_LAYOUT = DC_LAYOUT(8u32);
pub const LAYOUT_RTL: DC_LAYOUT = DC_LAYOUT(1u32);
impl ::core::marker::Copy for DC_LAYOUT {}
impl ::core::clone::Clone for DC_LAYOUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DC_LAYOUT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DC_LAYOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DC_LAYOUT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DC_LAYOUT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DC_LAYOUT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DC_LAYOUT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DC_LAYOUT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DC_LAYOUT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for DC_LAYOUT {
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
pub const DC_MANUFACTURER: u32 = 23u32;
pub const DC_MODEL: u32 = 24u32;
pub const DEFAULT_CHARSET: u32 = 1u32;
pub const DEFAULT_PITCH: u32 = 0u32;
pub struct DESIGNVECTOR {
    pub dvReserved: u32,
    pub dvNumAxes: u32,
    pub dvValues: [i32; 16],
}
impl ::core::marker::Copy for DESIGNVECTOR {}
impl ::core::clone::Clone for DESIGNVECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DESIGNVECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DESIGNVECTOR")
            .field("dvReserved", &self.dvReserved)
            .field("dvNumAxes", &self.dvNumAxes)
            .field("dvValues", &self.dvValues)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DESIGNVECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.dvReserved == other.dvReserved
            && self.dvNumAxes == other.dvNumAxes
            && self.dvValues == other.dvValues
    }
}
impl ::core::cmp::Eq for DESIGNVECTOR {}
impl FromIntoMemory for DESIGNVECTOR {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 72);
        let f_dvReserved = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dvNumAxes = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dvValues = <[i32; 16] as FromIntoMemory>::from_bytes(&from[8..8 + 64]);
        Self {
            dvReserved: f_dvReserved,
            dvNumAxes: f_dvNumAxes,
            dvValues: f_dvValues,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 72);
        FromIntoMemory::into_bytes(self.dvReserved, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dvNumAxes, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dvValues, &mut into[8..8 + 64]);
    }
    fn size() -> usize {
        72
    }
}
pub const DEVICEDATA: u32 = 19u32;
pub const DEVICE_FONTTYPE: u32 = 2u32;
pub struct DEVMODEA {
    pub dmDeviceName: [u8; 32],
    pub dmSpecVersion: u16,
    pub dmDriverVersion: u16,
    pub dmSize: u16,
    pub dmDriverExtra: u16,
    pub dmFields: u32,
    pub Anonymous1: DEVMODEA_0,
    pub dmColor: i16,
    pub dmDuplex: i16,
    pub dmYResolution: i16,
    pub dmTTOption: i16,
    pub dmCollate: i16,
    pub dmFormName: [u8; 32],
    pub dmLogPixels: u16,
    pub dmBitsPerPel: u32,
    pub dmPelsWidth: u32,
    pub dmPelsHeight: u32,
    pub Anonymous2: DEVMODEA_1,
    pub dmDisplayFrequency: u32,
    pub dmICMMethod: u32,
    pub dmICMIntent: u32,
    pub dmMediaType: u32,
    pub dmDitherType: u32,
    pub dmReserved1: u32,
    pub dmReserved2: u32,
    pub dmPanningWidth: u32,
    pub dmPanningHeight: u32,
}
impl ::core::marker::Copy for DEVMODEA {}
impl ::core::clone::Clone for DEVMODEA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DEVMODEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVMODEA")
            .field("dmDeviceName", &self.dmDeviceName)
            .field("dmSpecVersion", &self.dmSpecVersion)
            .field("dmDriverVersion", &self.dmDriverVersion)
            .field("dmSize", &self.dmSize)
            .field("dmDriverExtra", &self.dmDriverExtra)
            .field("dmFields", &self.dmFields)
            .field("Anonymous1", &self.Anonymous1)
            .field("dmColor", &self.dmColor)
            .field("dmDuplex", &self.dmDuplex)
            .field("dmYResolution", &self.dmYResolution)
            .field("dmTTOption", &self.dmTTOption)
            .field("dmCollate", &self.dmCollate)
            .field("dmFormName", &self.dmFormName)
            .field("dmLogPixels", &self.dmLogPixels)
            .field("dmBitsPerPel", &self.dmBitsPerPel)
            .field("dmPelsWidth", &self.dmPelsWidth)
            .field("dmPelsHeight", &self.dmPelsHeight)
            .field("Anonymous2", &self.Anonymous2)
            .field("dmDisplayFrequency", &self.dmDisplayFrequency)
            .field("dmICMMethod", &self.dmICMMethod)
            .field("dmICMIntent", &self.dmICMIntent)
            .field("dmMediaType", &self.dmMediaType)
            .field("dmDitherType", &self.dmDitherType)
            .field("dmReserved1", &self.dmReserved1)
            .field("dmReserved2", &self.dmReserved2)
            .field("dmPanningWidth", &self.dmPanningWidth)
            .field("dmPanningHeight", &self.dmPanningHeight)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DEVMODEA {
    fn eq(&self, other: &Self) -> bool {
        self.dmDeviceName == other.dmDeviceName
            && self.dmSpecVersion == other.dmSpecVersion
            && self.dmDriverVersion == other.dmDriverVersion
            && self.dmSize == other.dmSize
            && self.dmDriverExtra == other.dmDriverExtra
            && self.dmFields == other.dmFields
            && self.Anonymous1 == other.Anonymous1
            && self.dmColor == other.dmColor
            && self.dmDuplex == other.dmDuplex
            && self.dmYResolution == other.dmYResolution
            && self.dmTTOption == other.dmTTOption
            && self.dmCollate == other.dmCollate
            && self.dmFormName == other.dmFormName
            && self.dmLogPixels == other.dmLogPixels
            && self.dmBitsPerPel == other.dmBitsPerPel
            && self.dmPelsWidth == other.dmPelsWidth
            && self.dmPelsHeight == other.dmPelsHeight
            && self.Anonymous2 == other.Anonymous2
            && self.dmDisplayFrequency == other.dmDisplayFrequency
            && self.dmICMMethod == other.dmICMMethod
            && self.dmICMIntent == other.dmICMIntent
            && self.dmMediaType == other.dmMediaType
            && self.dmDitherType == other.dmDitherType
            && self.dmReserved1 == other.dmReserved1
            && self.dmReserved2 == other.dmReserved2
            && self.dmPanningWidth == other.dmPanningWidth
            && self.dmPanningHeight == other.dmPanningHeight
    }
}
impl ::core::cmp::Eq for DEVMODEA {}
impl FromIntoMemory for DEVMODEA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 156);
        let f_dmDeviceName = <[u8; 32] as FromIntoMemory>::from_bytes(&from[0..0 + 32]);
        let f_dmSpecVersion = <u16 as FromIntoMemory>::from_bytes(&from[32..32 + 2]);
        let f_dmDriverVersion = <u16 as FromIntoMemory>::from_bytes(&from[34..34 + 2]);
        let f_dmSize = <u16 as FromIntoMemory>::from_bytes(&from[36..36 + 2]);
        let f_dmDriverExtra = <u16 as FromIntoMemory>::from_bytes(&from[38..38 + 2]);
        let f_dmFields = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_Anonymous1 = <DEVMODEA_0 as FromIntoMemory>::from_bytes(&from[44..44 + 16]);
        let f_dmColor = <i16 as FromIntoMemory>::from_bytes(&from[60..60 + 2]);
        let f_dmDuplex = <i16 as FromIntoMemory>::from_bytes(&from[62..62 + 2]);
        let f_dmYResolution = <i16 as FromIntoMemory>::from_bytes(&from[64..64 + 2]);
        let f_dmTTOption = <i16 as FromIntoMemory>::from_bytes(&from[66..66 + 2]);
        let f_dmCollate = <i16 as FromIntoMemory>::from_bytes(&from[68..68 + 2]);
        let f_dmFormName = <[u8; 32] as FromIntoMemory>::from_bytes(&from[70..70 + 32]);
        let f_dmLogPixels = <u16 as FromIntoMemory>::from_bytes(&from[102..102 + 2]);
        let f_dmBitsPerPel = <u32 as FromIntoMemory>::from_bytes(&from[104..104 + 4]);
        let f_dmPelsWidth = <u32 as FromIntoMemory>::from_bytes(&from[108..108 + 4]);
        let f_dmPelsHeight = <u32 as FromIntoMemory>::from_bytes(&from[112..112 + 4]);
        let f_Anonymous2 = <DEVMODEA_1 as FromIntoMemory>::from_bytes(&from[116..116 + 4]);
        let f_dmDisplayFrequency = <u32 as FromIntoMemory>::from_bytes(&from[120..120 + 4]);
        let f_dmICMMethod = <u32 as FromIntoMemory>::from_bytes(&from[124..124 + 4]);
        let f_dmICMIntent = <u32 as FromIntoMemory>::from_bytes(&from[128..128 + 4]);
        let f_dmMediaType = <u32 as FromIntoMemory>::from_bytes(&from[132..132 + 4]);
        let f_dmDitherType = <u32 as FromIntoMemory>::from_bytes(&from[136..136 + 4]);
        let f_dmReserved1 = <u32 as FromIntoMemory>::from_bytes(&from[140..140 + 4]);
        let f_dmReserved2 = <u32 as FromIntoMemory>::from_bytes(&from[144..144 + 4]);
        let f_dmPanningWidth = <u32 as FromIntoMemory>::from_bytes(&from[148..148 + 4]);
        let f_dmPanningHeight = <u32 as FromIntoMemory>::from_bytes(&from[152..152 + 4]);
        Self {
            dmDeviceName: f_dmDeviceName,
            dmSpecVersion: f_dmSpecVersion,
            dmDriverVersion: f_dmDriverVersion,
            dmSize: f_dmSize,
            dmDriverExtra: f_dmDriverExtra,
            dmFields: f_dmFields,
            Anonymous1: f_Anonymous1,
            dmColor: f_dmColor,
            dmDuplex: f_dmDuplex,
            dmYResolution: f_dmYResolution,
            dmTTOption: f_dmTTOption,
            dmCollate: f_dmCollate,
            dmFormName: f_dmFormName,
            dmLogPixels: f_dmLogPixels,
            dmBitsPerPel: f_dmBitsPerPel,
            dmPelsWidth: f_dmPelsWidth,
            dmPelsHeight: f_dmPelsHeight,
            Anonymous2: f_Anonymous2,
            dmDisplayFrequency: f_dmDisplayFrequency,
            dmICMMethod: f_dmICMMethod,
            dmICMIntent: f_dmICMIntent,
            dmMediaType: f_dmMediaType,
            dmDitherType: f_dmDitherType,
            dmReserved1: f_dmReserved1,
            dmReserved2: f_dmReserved2,
            dmPanningWidth: f_dmPanningWidth,
            dmPanningHeight: f_dmPanningHeight,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 156);
        FromIntoMemory::into_bytes(self.dmDeviceName, &mut into[0..0 + 32]);
        FromIntoMemory::into_bytes(self.dmSpecVersion, &mut into[32..32 + 2]);
        FromIntoMemory::into_bytes(self.dmDriverVersion, &mut into[34..34 + 2]);
        FromIntoMemory::into_bytes(self.dmSize, &mut into[36..36 + 2]);
        FromIntoMemory::into_bytes(self.dmDriverExtra, &mut into[38..38 + 2]);
        FromIntoMemory::into_bytes(self.dmFields, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.Anonymous1, &mut into[44..44 + 16]);
        FromIntoMemory::into_bytes(self.dmColor, &mut into[60..60 + 2]);
        FromIntoMemory::into_bytes(self.dmDuplex, &mut into[62..62 + 2]);
        FromIntoMemory::into_bytes(self.dmYResolution, &mut into[64..64 + 2]);
        FromIntoMemory::into_bytes(self.dmTTOption, &mut into[66..66 + 2]);
        FromIntoMemory::into_bytes(self.dmCollate, &mut into[68..68 + 2]);
        FromIntoMemory::into_bytes(self.dmFormName, &mut into[70..70 + 32]);
        FromIntoMemory::into_bytes(self.dmLogPixels, &mut into[102..102 + 2]);
        FromIntoMemory::into_bytes(self.dmBitsPerPel, &mut into[104..104 + 4]);
        FromIntoMemory::into_bytes(self.dmPelsWidth, &mut into[108..108 + 4]);
        FromIntoMemory::into_bytes(self.dmPelsHeight, &mut into[112..112 + 4]);
        FromIntoMemory::into_bytes(self.Anonymous2, &mut into[116..116 + 4]);
        FromIntoMemory::into_bytes(self.dmDisplayFrequency, &mut into[120..120 + 4]);
        FromIntoMemory::into_bytes(self.dmICMMethod, &mut into[124..124 + 4]);
        FromIntoMemory::into_bytes(self.dmICMIntent, &mut into[128..128 + 4]);
        FromIntoMemory::into_bytes(self.dmMediaType, &mut into[132..132 + 4]);
        FromIntoMemory::into_bytes(self.dmDitherType, &mut into[136..136 + 4]);
        FromIntoMemory::into_bytes(self.dmReserved1, &mut into[140..140 + 4]);
        FromIntoMemory::into_bytes(self.dmReserved2, &mut into[144..144 + 4]);
        FromIntoMemory::into_bytes(self.dmPanningWidth, &mut into[148..148 + 4]);
        FromIntoMemory::into_bytes(self.dmPanningHeight, &mut into[152..152 + 4]);
    }
    fn size() -> usize {
        156
    }
}
pub struct DEVMODEA_0 {
    data: [u8; 16],
}
impl ::core::default::Default for DEVMODEA_0 {
    fn default() -> Self {
        Self { data: [0u8; 16] }
    }
}
impl ::core::marker::Copy for DEVMODEA_0 {}
impl ::core::clone::Clone for DEVMODEA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DEVMODEA_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVMODEA_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DEVMODEA_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for DEVMODEA_0 {}
impl FromIntoMemory for DEVMODEA_0 {
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
pub struct DEVMODEA_0_0 {
    pub dmOrientation: i16,
    pub dmPaperSize: i16,
    pub dmPaperLength: i16,
    pub dmPaperWidth: i16,
    pub dmScale: i16,
    pub dmCopies: i16,
    pub dmDefaultSource: i16,
    pub dmPrintQuality: i16,
}
impl ::core::marker::Copy for DEVMODEA_0_0 {}
impl ::core::clone::Clone for DEVMODEA_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DEVMODEA_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVMODEA_0_0")
            .field("dmOrientation", &self.dmOrientation)
            .field("dmPaperSize", &self.dmPaperSize)
            .field("dmPaperLength", &self.dmPaperLength)
            .field("dmPaperWidth", &self.dmPaperWidth)
            .field("dmScale", &self.dmScale)
            .field("dmCopies", &self.dmCopies)
            .field("dmDefaultSource", &self.dmDefaultSource)
            .field("dmPrintQuality", &self.dmPrintQuality)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DEVMODEA_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dmOrientation == other.dmOrientation
            && self.dmPaperSize == other.dmPaperSize
            && self.dmPaperLength == other.dmPaperLength
            && self.dmPaperWidth == other.dmPaperWidth
            && self.dmScale == other.dmScale
            && self.dmCopies == other.dmCopies
            && self.dmDefaultSource == other.dmDefaultSource
            && self.dmPrintQuality == other.dmPrintQuality
    }
}
impl ::core::cmp::Eq for DEVMODEA_0_0 {}
impl FromIntoMemory for DEVMODEA_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dmOrientation = <i16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_dmPaperSize = <i16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_dmPaperLength = <i16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_dmPaperWidth = <i16 as FromIntoMemory>::from_bytes(&from[6..6 + 2]);
        let f_dmScale = <i16 as FromIntoMemory>::from_bytes(&from[8..8 + 2]);
        let f_dmCopies = <i16 as FromIntoMemory>::from_bytes(&from[10..10 + 2]);
        let f_dmDefaultSource = <i16 as FromIntoMemory>::from_bytes(&from[12..12 + 2]);
        let f_dmPrintQuality = <i16 as FromIntoMemory>::from_bytes(&from[14..14 + 2]);
        Self {
            dmOrientation: f_dmOrientation,
            dmPaperSize: f_dmPaperSize,
            dmPaperLength: f_dmPaperLength,
            dmPaperWidth: f_dmPaperWidth,
            dmScale: f_dmScale,
            dmCopies: f_dmCopies,
            dmDefaultSource: f_dmDefaultSource,
            dmPrintQuality: f_dmPrintQuality,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dmOrientation, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.dmPaperSize, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.dmPaperLength, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.dmPaperWidth, &mut into[6..6 + 2]);
        FromIntoMemory::into_bytes(self.dmScale, &mut into[8..8 + 2]);
        FromIntoMemory::into_bytes(self.dmCopies, &mut into[10..10 + 2]);
        FromIntoMemory::into_bytes(self.dmDefaultSource, &mut into[12..12 + 2]);
        FromIntoMemory::into_bytes(self.dmPrintQuality, &mut into[14..14 + 2]);
    }
    fn size() -> usize {
        16
    }
}
pub struct DEVMODEA_0_1 {
    pub dmPosition: super::super::Foundation::POINTL,
    pub dmDisplayOrientation: u32,
    pub dmDisplayFixedOutput: u32,
}
impl ::core::marker::Copy for DEVMODEA_0_1 {}
impl ::core::clone::Clone for DEVMODEA_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DEVMODEA_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVMODEA_0_1")
            .field("dmPosition", &self.dmPosition)
            .field("dmDisplayOrientation", &self.dmDisplayOrientation)
            .field("dmDisplayFixedOutput", &self.dmDisplayFixedOutput)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DEVMODEA_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.dmPosition == other.dmPosition
            && self.dmDisplayOrientation == other.dmDisplayOrientation
            && self.dmDisplayFixedOutput == other.dmDisplayFixedOutput
    }
}
impl ::core::cmp::Eq for DEVMODEA_0_1 {}
impl FromIntoMemory for DEVMODEA_0_1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dmPosition =
            <super::super::Foundation::POINTL as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_dmDisplayOrientation = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dmDisplayFixedOutput = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            dmPosition: f_dmPosition,
            dmDisplayOrientation: f_dmDisplayOrientation,
            dmDisplayFixedOutput: f_dmDisplayFixedOutput,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dmPosition, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.dmDisplayOrientation, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dmDisplayFixedOutput, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct DEVMODEA_1 {
    data: [u8; 4],
}
impl ::core::default::Default for DEVMODEA_1 {
    fn default() -> Self {
        Self { data: [0u8; 4] }
    }
}
impl ::core::marker::Copy for DEVMODEA_1 {}
impl ::core::clone::Clone for DEVMODEA_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DEVMODEA_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVMODEA_1")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DEVMODEA_1 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for DEVMODEA_1 {}
impl FromIntoMemory for DEVMODEA_1 {
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
pub struct DEVMODEW {
    pub dmDeviceName: [u16; 32],
    pub dmSpecVersion: u16,
    pub dmDriverVersion: u16,
    pub dmSize: u16,
    pub dmDriverExtra: u16,
    pub dmFields: u32,
    pub Anonymous1: DEVMODEW_0,
    pub dmColor: i16,
    pub dmDuplex: i16,
    pub dmYResolution: i16,
    pub dmTTOption: i16,
    pub dmCollate: i16,
    pub dmFormName: [u16; 32],
    pub dmLogPixels: u16,
    pub dmBitsPerPel: u32,
    pub dmPelsWidth: u32,
    pub dmPelsHeight: u32,
    pub Anonymous2: DEVMODEW_1,
    pub dmDisplayFrequency: u32,
    pub dmICMMethod: u32,
    pub dmICMIntent: u32,
    pub dmMediaType: u32,
    pub dmDitherType: u32,
    pub dmReserved1: u32,
    pub dmReserved2: u32,
    pub dmPanningWidth: u32,
    pub dmPanningHeight: u32,
}
impl ::core::marker::Copy for DEVMODEW {}
impl ::core::clone::Clone for DEVMODEW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DEVMODEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVMODEW")
            .field("dmDeviceName", &self.dmDeviceName)
            .field("dmSpecVersion", &self.dmSpecVersion)
            .field("dmDriverVersion", &self.dmDriverVersion)
            .field("dmSize", &self.dmSize)
            .field("dmDriverExtra", &self.dmDriverExtra)
            .field("dmFields", &self.dmFields)
            .field("Anonymous1", &self.Anonymous1)
            .field("dmColor", &self.dmColor)
            .field("dmDuplex", &self.dmDuplex)
            .field("dmYResolution", &self.dmYResolution)
            .field("dmTTOption", &self.dmTTOption)
            .field("dmCollate", &self.dmCollate)
            .field("dmFormName", &self.dmFormName)
            .field("dmLogPixels", &self.dmLogPixels)
            .field("dmBitsPerPel", &self.dmBitsPerPel)
            .field("dmPelsWidth", &self.dmPelsWidth)
            .field("dmPelsHeight", &self.dmPelsHeight)
            .field("Anonymous2", &self.Anonymous2)
            .field("dmDisplayFrequency", &self.dmDisplayFrequency)
            .field("dmICMMethod", &self.dmICMMethod)
            .field("dmICMIntent", &self.dmICMIntent)
            .field("dmMediaType", &self.dmMediaType)
            .field("dmDitherType", &self.dmDitherType)
            .field("dmReserved1", &self.dmReserved1)
            .field("dmReserved2", &self.dmReserved2)
            .field("dmPanningWidth", &self.dmPanningWidth)
            .field("dmPanningHeight", &self.dmPanningHeight)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DEVMODEW {
    fn eq(&self, other: &Self) -> bool {
        self.dmDeviceName == other.dmDeviceName
            && self.dmSpecVersion == other.dmSpecVersion
            && self.dmDriverVersion == other.dmDriverVersion
            && self.dmSize == other.dmSize
            && self.dmDriverExtra == other.dmDriverExtra
            && self.dmFields == other.dmFields
            && self.Anonymous1 == other.Anonymous1
            && self.dmColor == other.dmColor
            && self.dmDuplex == other.dmDuplex
            && self.dmYResolution == other.dmYResolution
            && self.dmTTOption == other.dmTTOption
            && self.dmCollate == other.dmCollate
            && self.dmFormName == other.dmFormName
            && self.dmLogPixels == other.dmLogPixels
            && self.dmBitsPerPel == other.dmBitsPerPel
            && self.dmPelsWidth == other.dmPelsWidth
            && self.dmPelsHeight == other.dmPelsHeight
            && self.Anonymous2 == other.Anonymous2
            && self.dmDisplayFrequency == other.dmDisplayFrequency
            && self.dmICMMethod == other.dmICMMethod
            && self.dmICMIntent == other.dmICMIntent
            && self.dmMediaType == other.dmMediaType
            && self.dmDitherType == other.dmDitherType
            && self.dmReserved1 == other.dmReserved1
            && self.dmReserved2 == other.dmReserved2
            && self.dmPanningWidth == other.dmPanningWidth
            && self.dmPanningHeight == other.dmPanningHeight
    }
}
impl ::core::cmp::Eq for DEVMODEW {}
impl FromIntoMemory for DEVMODEW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 156);
        let f_dmDeviceName = <[u16; 32] as FromIntoMemory>::from_bytes(&from[0..0 + 32]);
        let f_dmSpecVersion = <u16 as FromIntoMemory>::from_bytes(&from[32..32 + 2]);
        let f_dmDriverVersion = <u16 as FromIntoMemory>::from_bytes(&from[34..34 + 2]);
        let f_dmSize = <u16 as FromIntoMemory>::from_bytes(&from[36..36 + 2]);
        let f_dmDriverExtra = <u16 as FromIntoMemory>::from_bytes(&from[38..38 + 2]);
        let f_dmFields = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_Anonymous1 = <DEVMODEW_0 as FromIntoMemory>::from_bytes(&from[44..44 + 16]);
        let f_dmColor = <i16 as FromIntoMemory>::from_bytes(&from[60..60 + 2]);
        let f_dmDuplex = <i16 as FromIntoMemory>::from_bytes(&from[62..62 + 2]);
        let f_dmYResolution = <i16 as FromIntoMemory>::from_bytes(&from[64..64 + 2]);
        let f_dmTTOption = <i16 as FromIntoMemory>::from_bytes(&from[66..66 + 2]);
        let f_dmCollate = <i16 as FromIntoMemory>::from_bytes(&from[68..68 + 2]);
        let f_dmFormName = <[u16; 32] as FromIntoMemory>::from_bytes(&from[70..70 + 32]);
        let f_dmLogPixels = <u16 as FromIntoMemory>::from_bytes(&from[102..102 + 2]);
        let f_dmBitsPerPel = <u32 as FromIntoMemory>::from_bytes(&from[104..104 + 4]);
        let f_dmPelsWidth = <u32 as FromIntoMemory>::from_bytes(&from[108..108 + 4]);
        let f_dmPelsHeight = <u32 as FromIntoMemory>::from_bytes(&from[112..112 + 4]);
        let f_Anonymous2 = <DEVMODEW_1 as FromIntoMemory>::from_bytes(&from[116..116 + 4]);
        let f_dmDisplayFrequency = <u32 as FromIntoMemory>::from_bytes(&from[120..120 + 4]);
        let f_dmICMMethod = <u32 as FromIntoMemory>::from_bytes(&from[124..124 + 4]);
        let f_dmICMIntent = <u32 as FromIntoMemory>::from_bytes(&from[128..128 + 4]);
        let f_dmMediaType = <u32 as FromIntoMemory>::from_bytes(&from[132..132 + 4]);
        let f_dmDitherType = <u32 as FromIntoMemory>::from_bytes(&from[136..136 + 4]);
        let f_dmReserved1 = <u32 as FromIntoMemory>::from_bytes(&from[140..140 + 4]);
        let f_dmReserved2 = <u32 as FromIntoMemory>::from_bytes(&from[144..144 + 4]);
        let f_dmPanningWidth = <u32 as FromIntoMemory>::from_bytes(&from[148..148 + 4]);
        let f_dmPanningHeight = <u32 as FromIntoMemory>::from_bytes(&from[152..152 + 4]);
        Self {
            dmDeviceName: f_dmDeviceName,
            dmSpecVersion: f_dmSpecVersion,
            dmDriverVersion: f_dmDriverVersion,
            dmSize: f_dmSize,
            dmDriverExtra: f_dmDriverExtra,
            dmFields: f_dmFields,
            Anonymous1: f_Anonymous1,
            dmColor: f_dmColor,
            dmDuplex: f_dmDuplex,
            dmYResolution: f_dmYResolution,
            dmTTOption: f_dmTTOption,
            dmCollate: f_dmCollate,
            dmFormName: f_dmFormName,
            dmLogPixels: f_dmLogPixels,
            dmBitsPerPel: f_dmBitsPerPel,
            dmPelsWidth: f_dmPelsWidth,
            dmPelsHeight: f_dmPelsHeight,
            Anonymous2: f_Anonymous2,
            dmDisplayFrequency: f_dmDisplayFrequency,
            dmICMMethod: f_dmICMMethod,
            dmICMIntent: f_dmICMIntent,
            dmMediaType: f_dmMediaType,
            dmDitherType: f_dmDitherType,
            dmReserved1: f_dmReserved1,
            dmReserved2: f_dmReserved2,
            dmPanningWidth: f_dmPanningWidth,
            dmPanningHeight: f_dmPanningHeight,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 156);
        FromIntoMemory::into_bytes(self.dmDeviceName, &mut into[0..0 + 32]);
        FromIntoMemory::into_bytes(self.dmSpecVersion, &mut into[32..32 + 2]);
        FromIntoMemory::into_bytes(self.dmDriverVersion, &mut into[34..34 + 2]);
        FromIntoMemory::into_bytes(self.dmSize, &mut into[36..36 + 2]);
        FromIntoMemory::into_bytes(self.dmDriverExtra, &mut into[38..38 + 2]);
        FromIntoMemory::into_bytes(self.dmFields, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.Anonymous1, &mut into[44..44 + 16]);
        FromIntoMemory::into_bytes(self.dmColor, &mut into[60..60 + 2]);
        FromIntoMemory::into_bytes(self.dmDuplex, &mut into[62..62 + 2]);
        FromIntoMemory::into_bytes(self.dmYResolution, &mut into[64..64 + 2]);
        FromIntoMemory::into_bytes(self.dmTTOption, &mut into[66..66 + 2]);
        FromIntoMemory::into_bytes(self.dmCollate, &mut into[68..68 + 2]);
        FromIntoMemory::into_bytes(self.dmFormName, &mut into[70..70 + 32]);
        FromIntoMemory::into_bytes(self.dmLogPixels, &mut into[102..102 + 2]);
        FromIntoMemory::into_bytes(self.dmBitsPerPel, &mut into[104..104 + 4]);
        FromIntoMemory::into_bytes(self.dmPelsWidth, &mut into[108..108 + 4]);
        FromIntoMemory::into_bytes(self.dmPelsHeight, &mut into[112..112 + 4]);
        FromIntoMemory::into_bytes(self.Anonymous2, &mut into[116..116 + 4]);
        FromIntoMemory::into_bytes(self.dmDisplayFrequency, &mut into[120..120 + 4]);
        FromIntoMemory::into_bytes(self.dmICMMethod, &mut into[124..124 + 4]);
        FromIntoMemory::into_bytes(self.dmICMIntent, &mut into[128..128 + 4]);
        FromIntoMemory::into_bytes(self.dmMediaType, &mut into[132..132 + 4]);
        FromIntoMemory::into_bytes(self.dmDitherType, &mut into[136..136 + 4]);
        FromIntoMemory::into_bytes(self.dmReserved1, &mut into[140..140 + 4]);
        FromIntoMemory::into_bytes(self.dmReserved2, &mut into[144..144 + 4]);
        FromIntoMemory::into_bytes(self.dmPanningWidth, &mut into[148..148 + 4]);
        FromIntoMemory::into_bytes(self.dmPanningHeight, &mut into[152..152 + 4]);
    }
    fn size() -> usize {
        156
    }
}
pub struct DEVMODEW_0 {
    data: [u8; 16],
}
impl ::core::default::Default for DEVMODEW_0 {
    fn default() -> Self {
        Self { data: [0u8; 16] }
    }
}
impl ::core::marker::Copy for DEVMODEW_0 {}
impl ::core::clone::Clone for DEVMODEW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DEVMODEW_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVMODEW_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DEVMODEW_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for DEVMODEW_0 {}
impl FromIntoMemory for DEVMODEW_0 {
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
pub struct DEVMODEW_0_0 {
    pub dmOrientation: i16,
    pub dmPaperSize: i16,
    pub dmPaperLength: i16,
    pub dmPaperWidth: i16,
    pub dmScale: i16,
    pub dmCopies: i16,
    pub dmDefaultSource: i16,
    pub dmPrintQuality: i16,
}
impl ::core::marker::Copy for DEVMODEW_0_0 {}
impl ::core::clone::Clone for DEVMODEW_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DEVMODEW_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVMODEW_0_0")
            .field("dmOrientation", &self.dmOrientation)
            .field("dmPaperSize", &self.dmPaperSize)
            .field("dmPaperLength", &self.dmPaperLength)
            .field("dmPaperWidth", &self.dmPaperWidth)
            .field("dmScale", &self.dmScale)
            .field("dmCopies", &self.dmCopies)
            .field("dmDefaultSource", &self.dmDefaultSource)
            .field("dmPrintQuality", &self.dmPrintQuality)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DEVMODEW_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dmOrientation == other.dmOrientation
            && self.dmPaperSize == other.dmPaperSize
            && self.dmPaperLength == other.dmPaperLength
            && self.dmPaperWidth == other.dmPaperWidth
            && self.dmScale == other.dmScale
            && self.dmCopies == other.dmCopies
            && self.dmDefaultSource == other.dmDefaultSource
            && self.dmPrintQuality == other.dmPrintQuality
    }
}
impl ::core::cmp::Eq for DEVMODEW_0_0 {}
impl FromIntoMemory for DEVMODEW_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dmOrientation = <i16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_dmPaperSize = <i16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_dmPaperLength = <i16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_dmPaperWidth = <i16 as FromIntoMemory>::from_bytes(&from[6..6 + 2]);
        let f_dmScale = <i16 as FromIntoMemory>::from_bytes(&from[8..8 + 2]);
        let f_dmCopies = <i16 as FromIntoMemory>::from_bytes(&from[10..10 + 2]);
        let f_dmDefaultSource = <i16 as FromIntoMemory>::from_bytes(&from[12..12 + 2]);
        let f_dmPrintQuality = <i16 as FromIntoMemory>::from_bytes(&from[14..14 + 2]);
        Self {
            dmOrientation: f_dmOrientation,
            dmPaperSize: f_dmPaperSize,
            dmPaperLength: f_dmPaperLength,
            dmPaperWidth: f_dmPaperWidth,
            dmScale: f_dmScale,
            dmCopies: f_dmCopies,
            dmDefaultSource: f_dmDefaultSource,
            dmPrintQuality: f_dmPrintQuality,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dmOrientation, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.dmPaperSize, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.dmPaperLength, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.dmPaperWidth, &mut into[6..6 + 2]);
        FromIntoMemory::into_bytes(self.dmScale, &mut into[8..8 + 2]);
        FromIntoMemory::into_bytes(self.dmCopies, &mut into[10..10 + 2]);
        FromIntoMemory::into_bytes(self.dmDefaultSource, &mut into[12..12 + 2]);
        FromIntoMemory::into_bytes(self.dmPrintQuality, &mut into[14..14 + 2]);
    }
    fn size() -> usize {
        16
    }
}
pub struct DEVMODEW_0_1 {
    pub dmPosition: super::super::Foundation::POINTL,
    pub dmDisplayOrientation: u32,
    pub dmDisplayFixedOutput: u32,
}
impl ::core::marker::Copy for DEVMODEW_0_1 {}
impl ::core::clone::Clone for DEVMODEW_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DEVMODEW_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVMODEW_0_1")
            .field("dmPosition", &self.dmPosition)
            .field("dmDisplayOrientation", &self.dmDisplayOrientation)
            .field("dmDisplayFixedOutput", &self.dmDisplayFixedOutput)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DEVMODEW_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.dmPosition == other.dmPosition
            && self.dmDisplayOrientation == other.dmDisplayOrientation
            && self.dmDisplayFixedOutput == other.dmDisplayFixedOutput
    }
}
impl ::core::cmp::Eq for DEVMODEW_0_1 {}
impl FromIntoMemory for DEVMODEW_0_1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dmPosition =
            <super::super::Foundation::POINTL as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_dmDisplayOrientation = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dmDisplayFixedOutput = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            dmPosition: f_dmPosition,
            dmDisplayOrientation: f_dmDisplayOrientation,
            dmDisplayFixedOutput: f_dmDisplayFixedOutput,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dmPosition, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.dmDisplayOrientation, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dmDisplayFixedOutput, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct DEVMODEW_1 {
    data: [u8; 4],
}
impl ::core::default::Default for DEVMODEW_1 {
    fn default() -> Self {
        Self { data: [0u8; 4] }
    }
}
impl ::core::marker::Copy for DEVMODEW_1 {}
impl ::core::clone::Clone for DEVMODEW_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DEVMODEW_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVMODEW_1")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DEVMODEW_1 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for DEVMODEW_1 {}
impl FromIntoMemory for DEVMODEW_1 {
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
pub struct DFCS_STATE(pub u32);
pub const DFCS_CAPTIONCLOSE: DFCS_STATE = DFCS_STATE(0u32);
pub const DFCS_CAPTIONMIN: DFCS_STATE = DFCS_STATE(1u32);
pub const DFCS_CAPTIONMAX: DFCS_STATE = DFCS_STATE(2u32);
pub const DFCS_CAPTIONRESTORE: DFCS_STATE = DFCS_STATE(3u32);
pub const DFCS_CAPTIONHELP: DFCS_STATE = DFCS_STATE(4u32);
pub const DFCS_MENUARROW: DFCS_STATE = DFCS_STATE(0u32);
pub const DFCS_MENUCHECK: DFCS_STATE = DFCS_STATE(1u32);
pub const DFCS_MENUBULLET: DFCS_STATE = DFCS_STATE(2u32);
pub const DFCS_MENUARROWRIGHT: DFCS_STATE = DFCS_STATE(4u32);
pub const DFCS_SCROLLUP: DFCS_STATE = DFCS_STATE(0u32);
pub const DFCS_SCROLLDOWN: DFCS_STATE = DFCS_STATE(1u32);
pub const DFCS_SCROLLLEFT: DFCS_STATE = DFCS_STATE(2u32);
pub const DFCS_SCROLLRIGHT: DFCS_STATE = DFCS_STATE(3u32);
pub const DFCS_SCROLLCOMBOBOX: DFCS_STATE = DFCS_STATE(5u32);
pub const DFCS_SCROLLSIZEGRIP: DFCS_STATE = DFCS_STATE(8u32);
pub const DFCS_SCROLLSIZEGRIPRIGHT: DFCS_STATE = DFCS_STATE(16u32);
pub const DFCS_BUTTONCHECK: DFCS_STATE = DFCS_STATE(0u32);
pub const DFCS_BUTTONRADIOIMAGE: DFCS_STATE = DFCS_STATE(1u32);
pub const DFCS_BUTTONRADIOMASK: DFCS_STATE = DFCS_STATE(2u32);
pub const DFCS_BUTTONRADIO: DFCS_STATE = DFCS_STATE(4u32);
pub const DFCS_BUTTON3STATE: DFCS_STATE = DFCS_STATE(8u32);
pub const DFCS_BUTTONPUSH: DFCS_STATE = DFCS_STATE(16u32);
pub const DFCS_INACTIVE: DFCS_STATE = DFCS_STATE(256u32);
pub const DFCS_PUSHED: DFCS_STATE = DFCS_STATE(512u32);
pub const DFCS_CHECKED: DFCS_STATE = DFCS_STATE(1024u32);
pub const DFCS_TRANSPARENT: DFCS_STATE = DFCS_STATE(2048u32);
pub const DFCS_HOT: DFCS_STATE = DFCS_STATE(4096u32);
pub const DFCS_ADJUSTRECT: DFCS_STATE = DFCS_STATE(8192u32);
pub const DFCS_FLAT: DFCS_STATE = DFCS_STATE(16384u32);
pub const DFCS_MONO: DFCS_STATE = DFCS_STATE(32768u32);
impl ::core::marker::Copy for DFCS_STATE {}
impl ::core::clone::Clone for DFCS_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DFCS_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DFCS_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DFCS_STATE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DFCS_STATE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DFCS_STATE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DFCS_STATE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DFCS_STATE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DFCS_STATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for DFCS_STATE {
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
pub struct DFC_TYPE(pub u32);
pub const DFC_CAPTION: DFC_TYPE = DFC_TYPE(1u32);
pub const DFC_MENU: DFC_TYPE = DFC_TYPE(2u32);
pub const DFC_SCROLL: DFC_TYPE = DFC_TYPE(3u32);
pub const DFC_BUTTON: DFC_TYPE = DFC_TYPE(4u32);
pub const DFC_POPUPMENU: DFC_TYPE = DFC_TYPE(5u32);
impl ::core::marker::Copy for DFC_TYPE {}
impl ::core::clone::Clone for DFC_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DFC_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DFC_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DFC_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for DFC_TYPE {
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
pub struct DIBSECTION {
    pub dsBm: BITMAP,
    pub dsBmih: BITMAPINFOHEADER,
    pub dsBitfields: [u32; 3],
    pub dshSection: super::super::Foundation::HANDLE,
    pub dsOffset: u32,
}
impl ::core::marker::Copy for DIBSECTION {}
impl ::core::clone::Clone for DIBSECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIBSECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIBSECTION")
            .field("dsBm", &self.dsBm)
            .field("dsBmih", &self.dsBmih)
            .field("dsBitfields", &self.dsBitfields)
            .field("dshSection", &self.dshSection)
            .field("dsOffset", &self.dsOffset)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DIBSECTION {
    fn eq(&self, other: &Self) -> bool {
        self.dsBm == other.dsBm
            && self.dsBmih == other.dsBmih
            && self.dsBitfields == other.dsBitfields
            && self.dshSection == other.dshSection
            && self.dsOffset == other.dsOffset
    }
}
impl ::core::cmp::Eq for DIBSECTION {}
impl FromIntoMemory for DIBSECTION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 84);
        let f_dsBm = <BITMAP as FromIntoMemory>::from_bytes(&from[0..0 + 24]);
        let f_dsBmih = <BITMAPINFOHEADER as FromIntoMemory>::from_bytes(&from[24..24 + 40]);
        let f_dsBitfields = <[u32; 3] as FromIntoMemory>::from_bytes(&from[64..64 + 12]);
        let f_dshSection =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[76..76 + 4]);
        let f_dsOffset = <u32 as FromIntoMemory>::from_bytes(&from[80..80 + 4]);
        Self {
            dsBm: f_dsBm,
            dsBmih: f_dsBmih,
            dsBitfields: f_dsBitfields,
            dshSection: f_dshSection,
            dsOffset: f_dsOffset,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 84);
        FromIntoMemory::into_bytes(self.dsBm, &mut into[0..0 + 24]);
        FromIntoMemory::into_bytes(self.dsBmih, &mut into[24..24 + 40]);
        FromIntoMemory::into_bytes(self.dsBitfields, &mut into[64..64 + 12]);
        FromIntoMemory::into_bytes(self.dshSection, &mut into[76..76 + 4]);
        FromIntoMemory::into_bytes(self.dsOffset, &mut into[80..80 + 4]);
    }
    fn size() -> usize {
        84
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DIB_USAGE(pub u32);
pub const DIB_RGB_COLORS: DIB_USAGE = DIB_USAGE(0u32);
pub const DIB_PAL_COLORS: DIB_USAGE = DIB_USAGE(1u32);
impl ::core::marker::Copy for DIB_USAGE {}
impl ::core::clone::Clone for DIB_USAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIB_USAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DIB_USAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIB_USAGE").field(&self.0).finish()
    }
}
impl FromIntoMemory for DIB_USAGE {
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
pub struct DISPLAYCONFIG_COLOR_ENCODING(pub i32);
pub const DISPLAYCONFIG_COLOR_ENCODING_RGB: DISPLAYCONFIG_COLOR_ENCODING =
    DISPLAYCONFIG_COLOR_ENCODING(0i32);
pub const DISPLAYCONFIG_COLOR_ENCODING_YCBCR444: DISPLAYCONFIG_COLOR_ENCODING =
    DISPLAYCONFIG_COLOR_ENCODING(1i32);
pub const DISPLAYCONFIG_COLOR_ENCODING_YCBCR422: DISPLAYCONFIG_COLOR_ENCODING =
    DISPLAYCONFIG_COLOR_ENCODING(2i32);
pub const DISPLAYCONFIG_COLOR_ENCODING_YCBCR420: DISPLAYCONFIG_COLOR_ENCODING =
    DISPLAYCONFIG_COLOR_ENCODING(3i32);
pub const DISPLAYCONFIG_COLOR_ENCODING_INTENSITY: DISPLAYCONFIG_COLOR_ENCODING =
    DISPLAYCONFIG_COLOR_ENCODING(4i32);
pub const DISPLAYCONFIG_COLOR_ENCODING_FORCE_UINT32: DISPLAYCONFIG_COLOR_ENCODING =
    DISPLAYCONFIG_COLOR_ENCODING(-1i32);
impl ::core::marker::Copy for DISPLAYCONFIG_COLOR_ENCODING {}
impl ::core::clone::Clone for DISPLAYCONFIG_COLOR_ENCODING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPLAYCONFIG_COLOR_ENCODING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPLAYCONFIG_COLOR_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPLAYCONFIG_COLOR_ENCODING")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for DISPLAYCONFIG_COLOR_ENCODING {
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
pub const DISPLAYCONFIG_MAXPATH: u32 = 1024u32;
pub const DISPLAYCONFIG_PATH_ACTIVE: u32 = 1u32;
pub const DISPLAYCONFIG_PATH_CLONE_GROUP_INVALID: u32 = 65535u32;
pub const DISPLAYCONFIG_PATH_DESKTOP_IMAGE_IDX_INVALID: u32 = 65535u32;
pub const DISPLAYCONFIG_PATH_MODE_IDX_INVALID: u32 = 4294967295u32;
pub const DISPLAYCONFIG_PATH_PREFERRED_UNSCALED: u32 = 4u32;
pub const DISPLAYCONFIG_PATH_SOURCE_MODE_IDX_INVALID: u32 = 65535u32;
pub const DISPLAYCONFIG_PATH_SUPPORT_VIRTUAL_MODE: u32 = 8u32;
pub const DISPLAYCONFIG_PATH_TARGET_MODE_IDX_INVALID: u32 = 65535u32;
pub const DISPLAYCONFIG_PATH_VALID_FLAGS: u32 = 29u32;
pub const DISPLAYCONFIG_SOURCE_IN_USE: u32 = 1u32;
pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_BOOT: u32 = 4u32;
pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_PATH: u32 = 8u32;
pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_SYSTEM: u32 = 16u32;
pub const DISPLAYCONFIG_TARGET_FORCIBLE: u32 = 2u32;
pub const DISPLAYCONFIG_TARGET_IN_USE: u32 = 1u32;
pub const DISPLAYCONFIG_TARGET_IS_HMD: u32 = 32u32;
pub struct DISPLAY_DEVICEA {
    pub cb: u32,
    pub DeviceName: [super::super::Foundation::CHAR; 32],
    pub DeviceString: [super::super::Foundation::CHAR; 128],
    pub StateFlags: u32,
    pub DeviceID: [super::super::Foundation::CHAR; 128],
    pub DeviceKey: [super::super::Foundation::CHAR; 128],
}
impl ::core::marker::Copy for DISPLAY_DEVICEA {}
impl ::core::clone::Clone for DISPLAY_DEVICEA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DISPLAY_DEVICEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISPLAY_DEVICEA")
            .field("cb", &self.cb)
            .field("DeviceName", &self.DeviceName)
            .field("DeviceString", &self.DeviceString)
            .field("StateFlags", &self.StateFlags)
            .field("DeviceID", &self.DeviceID)
            .field("DeviceKey", &self.DeviceKey)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DISPLAY_DEVICEA {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb
            && self.DeviceName == other.DeviceName
            && self.DeviceString == other.DeviceString
            && self.StateFlags == other.StateFlags
            && self.DeviceID == other.DeviceID
            && self.DeviceKey == other.DeviceKey
    }
}
impl ::core::cmp::Eq for DISPLAY_DEVICEA {}
impl FromIntoMemory for DISPLAY_DEVICEA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 424);
        let f_cb = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_DeviceName =
            <[super::super::Foundation::CHAR; 32] as FromIntoMemory>::from_bytes(&from[4..4 + 32]);
        let f_DeviceString = <[super::super::Foundation::CHAR; 128] as FromIntoMemory>::from_bytes(
            &from[36..36 + 128],
        );
        let f_StateFlags = <u32 as FromIntoMemory>::from_bytes(&from[164..164 + 4]);
        let f_DeviceID = <[super::super::Foundation::CHAR; 128] as FromIntoMemory>::from_bytes(
            &from[168..168 + 128],
        );
        let f_DeviceKey = <[super::super::Foundation::CHAR; 128] as FromIntoMemory>::from_bytes(
            &from[296..296 + 128],
        );
        Self {
            cb: f_cb,
            DeviceName: f_DeviceName,
            DeviceString: f_DeviceString,
            StateFlags: f_StateFlags,
            DeviceID: f_DeviceID,
            DeviceKey: f_DeviceKey,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 424);
        FromIntoMemory::into_bytes(self.cb, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.DeviceName, &mut into[4..4 + 32]);
        FromIntoMemory::into_bytes(self.DeviceString, &mut into[36..36 + 128]);
        FromIntoMemory::into_bytes(self.StateFlags, &mut into[164..164 + 4]);
        FromIntoMemory::into_bytes(self.DeviceID, &mut into[168..168 + 128]);
        FromIntoMemory::into_bytes(self.DeviceKey, &mut into[296..296 + 128]);
    }
    fn size() -> usize {
        424
    }
}
pub struct DISPLAY_DEVICEW {
    pub cb: u32,
    pub DeviceName: [u16; 32],
    pub DeviceString: [u16; 128],
    pub StateFlags: u32,
    pub DeviceID: [u16; 128],
    pub DeviceKey: [u16; 128],
}
impl ::core::marker::Copy for DISPLAY_DEVICEW {}
impl ::core::clone::Clone for DISPLAY_DEVICEW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DISPLAY_DEVICEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISPLAY_DEVICEW")
            .field("cb", &self.cb)
            .field("DeviceName", &self.DeviceName)
            .field("DeviceString", &self.DeviceString)
            .field("StateFlags", &self.StateFlags)
            .field("DeviceID", &self.DeviceID)
            .field("DeviceKey", &self.DeviceKey)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DISPLAY_DEVICEW {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb
            && self.DeviceName == other.DeviceName
            && self.DeviceString == other.DeviceString
            && self.StateFlags == other.StateFlags
            && self.DeviceID == other.DeviceID
            && self.DeviceKey == other.DeviceKey
    }
}
impl ::core::cmp::Eq for DISPLAY_DEVICEW {}
impl FromIntoMemory for DISPLAY_DEVICEW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 424);
        let f_cb = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_DeviceName = <[u16; 32] as FromIntoMemory>::from_bytes(&from[4..4 + 32]);
        let f_DeviceString = <[u16; 128] as FromIntoMemory>::from_bytes(&from[36..36 + 128]);
        let f_StateFlags = <u32 as FromIntoMemory>::from_bytes(&from[164..164 + 4]);
        let f_DeviceID = <[u16; 128] as FromIntoMemory>::from_bytes(&from[168..168 + 128]);
        let f_DeviceKey = <[u16; 128] as FromIntoMemory>::from_bytes(&from[296..296 + 128]);
        Self {
            cb: f_cb,
            DeviceName: f_DeviceName,
            DeviceString: f_DeviceString,
            StateFlags: f_StateFlags,
            DeviceID: f_DeviceID,
            DeviceKey: f_DeviceKey,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 424);
        FromIntoMemory::into_bytes(self.cb, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.DeviceName, &mut into[4..4 + 32]);
        FromIntoMemory::into_bytes(self.DeviceString, &mut into[36..36 + 128]);
        FromIntoMemory::into_bytes(self.StateFlags, &mut into[164..164 + 4]);
        FromIntoMemory::into_bytes(self.DeviceID, &mut into[168..168 + 128]);
        FromIntoMemory::into_bytes(self.DeviceKey, &mut into[296..296 + 128]);
    }
    fn size() -> usize {
        424
    }
}
pub const DISPLAY_DEVICE_ACC_DRIVER: u32 = 64u32;
pub const DISPLAY_DEVICE_ACTIVE: u32 = 1u32;
pub const DISPLAY_DEVICE_ATTACHED: u32 = 2u32;
pub const DISPLAY_DEVICE_ATTACHED_TO_DESKTOP: u32 = 1u32;
pub const DISPLAY_DEVICE_DISCONNECT: u32 = 33554432u32;
pub const DISPLAY_DEVICE_MIRRORING_DRIVER: u32 = 8u32;
pub const DISPLAY_DEVICE_MODESPRUNED: u32 = 134217728u32;
pub const DISPLAY_DEVICE_MULTI_DRIVER: u32 = 2u32;
pub const DISPLAY_DEVICE_PRIMARY_DEVICE: u32 = 4u32;
pub const DISPLAY_DEVICE_RDPUDD: u32 = 16777216u32;
pub const DISPLAY_DEVICE_REMOTE: u32 = 67108864u32;
pub const DISPLAY_DEVICE_REMOVABLE: u32 = 32u32;
pub const DISPLAY_DEVICE_TS_COMPATIBLE: u32 = 2097152u32;
pub const DISPLAY_DEVICE_UNSAFE_MODES_ON: u32 = 524288u32;
pub const DISPLAY_DEVICE_VGA_COMPATIBLE: u32 = 16u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISP_CHANGE(pub i32);
pub const DISP_CHANGE_SUCCESSFUL: DISP_CHANGE = DISP_CHANGE(0i32);
pub const DISP_CHANGE_RESTART: DISP_CHANGE = DISP_CHANGE(1i32);
pub const DISP_CHANGE_FAILED: DISP_CHANGE = DISP_CHANGE(-1i32);
pub const DISP_CHANGE_BADMODE: DISP_CHANGE = DISP_CHANGE(-2i32);
pub const DISP_CHANGE_NOTUPDATED: DISP_CHANGE = DISP_CHANGE(-3i32);
pub const DISP_CHANGE_BADFLAGS: DISP_CHANGE = DISP_CHANGE(-4i32);
pub const DISP_CHANGE_BADPARAM: DISP_CHANGE = DISP_CHANGE(-5i32);
pub const DISP_CHANGE_BADDUALVIEW: DISP_CHANGE = DISP_CHANGE(-6i32);
impl ::core::marker::Copy for DISP_CHANGE {}
impl ::core::clone::Clone for DISP_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISP_CHANGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISP_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISP_CHANGE").field(&self.0).finish()
    }
}
impl FromIntoMemory for DISP_CHANGE {
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
pub const DI_APPBANDING: u32 = 1u32;
pub const DI_ROPS_READ_DESTINATION: u32 = 2u32;
pub const DMBIN_AUTO: u32 = 7u32;
pub const DMBIN_CASSETTE: u32 = 14u32;
pub const DMBIN_ENVELOPE: u32 = 5u32;
pub const DMBIN_ENVMANUAL: u32 = 6u32;
pub const DMBIN_FORMSOURCE: u32 = 15u32;
pub const DMBIN_LARGECAPACITY: u32 = 11u32;
pub const DMBIN_LARGEFMT: u32 = 10u32;
pub const DMBIN_LAST: u32 = 15u32;
pub const DMBIN_LOWER: u32 = 2u32;
pub const DMBIN_MANUAL: u32 = 4u32;
pub const DMBIN_MIDDLE: u32 = 3u32;
pub const DMBIN_ONLYONE: u32 = 1u32;
pub const DMBIN_SMALLFMT: u32 = 9u32;
pub const DMBIN_TRACTOR: u32 = 8u32;
pub const DMBIN_UPPER: u32 = 1u32;
pub const DMBIN_USER: u32 = 256u32;
pub const DMCOLLATE_FALSE: u32 = 0u32;
pub const DMCOLLATE_TRUE: u32 = 1u32;
pub const DMCOLOR_COLOR: u32 = 2u32;
pub const DMCOLOR_MONOCHROME: u32 = 1u32;
pub const DMDFO_CENTER: u32 = 2u32;
pub const DMDFO_DEFAULT: u32 = 0u32;
pub const DMDFO_STRETCH: u32 = 1u32;
pub const DMDISPLAYFLAGS_TEXTMODE: u32 = 4u32;
pub const DMDITHER_COARSE: u32 = 2u32;
pub const DMDITHER_ERRORDIFFUSION: u32 = 5u32;
pub const DMDITHER_FINE: u32 = 3u32;
pub const DMDITHER_GRAYSCALE: u32 = 10u32;
pub const DMDITHER_LINEART: u32 = 4u32;
pub const DMDITHER_NONE: u32 = 1u32;
pub const DMDITHER_RESERVED6: u32 = 6u32;
pub const DMDITHER_RESERVED7: u32 = 7u32;
pub const DMDITHER_RESERVED8: u32 = 8u32;
pub const DMDITHER_RESERVED9: u32 = 9u32;
pub const DMDITHER_USER: u32 = 256u32;
pub const DMDO_180: u32 = 2u32;
pub const DMDO_270: u32 = 3u32;
pub const DMDO_90: u32 = 1u32;
pub const DMDO_DEFAULT: u32 = 0u32;
pub const DMDUP_HORIZONTAL: u32 = 3u32;
pub const DMDUP_SIMPLEX: u32 = 1u32;
pub const DMDUP_VERTICAL: u32 = 2u32;
pub const DMICMMETHOD_DEVICE: u32 = 4u32;
pub const DMICMMETHOD_DRIVER: u32 = 3u32;
pub const DMICMMETHOD_NONE: u32 = 1u32;
pub const DMICMMETHOD_SYSTEM: u32 = 2u32;
pub const DMICMMETHOD_USER: u32 = 256u32;
pub const DMICM_ABS_COLORIMETRIC: u32 = 4u32;
pub const DMICM_COLORIMETRIC: u32 = 3u32;
pub const DMICM_CONTRAST: u32 = 2u32;
pub const DMICM_SATURATE: u32 = 1u32;
pub const DMICM_USER: u32 = 256u32;
pub const DMMEDIA_GLOSSY: u32 = 3u32;
pub const DMMEDIA_STANDARD: u32 = 1u32;
pub const DMMEDIA_TRANSPARENCY: u32 = 2u32;
pub const DMMEDIA_USER: u32 = 256u32;
pub const DMNUP_ONEUP: u32 = 2u32;
pub const DMNUP_SYSTEM: u32 = 1u32;
pub const DMORIENT_LANDSCAPE: u32 = 2u32;
pub const DMORIENT_PORTRAIT: u32 = 1u32;
pub const DMPAPER_10X11: u32 = 45u32;
pub const DMPAPER_10X14: u32 = 16u32;
pub const DMPAPER_11X17: u32 = 17u32;
pub const DMPAPER_12X11: u32 = 90u32;
pub const DMPAPER_15X11: u32 = 46u32;
pub const DMPAPER_9X11: u32 = 44u32;
pub const DMPAPER_A2: u32 = 66u32;
pub const DMPAPER_A3: u32 = 8u32;
pub const DMPAPER_A3_EXTRA: u32 = 63u32;
pub const DMPAPER_A3_EXTRA_TRANSVERSE: u32 = 68u32;
pub const DMPAPER_A3_ROTATED: u32 = 76u32;
pub const DMPAPER_A3_TRANSVERSE: u32 = 67u32;
pub const DMPAPER_A4: u32 = 9u32;
pub const DMPAPER_A4SMALL: u32 = 10u32;
pub const DMPAPER_A4_EXTRA: u32 = 53u32;
pub const DMPAPER_A4_PLUS: u32 = 60u32;
pub const DMPAPER_A4_ROTATED: u32 = 77u32;
pub const DMPAPER_A4_TRANSVERSE: u32 = 55u32;
pub const DMPAPER_A5: u32 = 11u32;
pub const DMPAPER_A5_EXTRA: u32 = 64u32;
pub const DMPAPER_A5_ROTATED: u32 = 78u32;
pub const DMPAPER_A5_TRANSVERSE: u32 = 61u32;
pub const DMPAPER_A6: u32 = 70u32;
pub const DMPAPER_A6_ROTATED: u32 = 83u32;
pub const DMPAPER_A_PLUS: u32 = 57u32;
pub const DMPAPER_B4: u32 = 12u32;
pub const DMPAPER_B4_JIS_ROTATED: u32 = 79u32;
pub const DMPAPER_B5: u32 = 13u32;
pub const DMPAPER_B5_EXTRA: u32 = 65u32;
pub const DMPAPER_B5_JIS_ROTATED: u32 = 80u32;
pub const DMPAPER_B5_TRANSVERSE: u32 = 62u32;
pub const DMPAPER_B6_JIS: u32 = 88u32;
pub const DMPAPER_B6_JIS_ROTATED: u32 = 89u32;
pub const DMPAPER_B_PLUS: u32 = 58u32;
pub const DMPAPER_CSHEET: u32 = 24u32;
pub const DMPAPER_DBL_JAPANESE_POSTCARD: u32 = 69u32;
pub const DMPAPER_DBL_JAPANESE_POSTCARD_ROTATED: u32 = 82u32;
pub const DMPAPER_DSHEET: u32 = 25u32;
pub const DMPAPER_ENV_10: u32 = 20u32;
pub const DMPAPER_ENV_11: u32 = 21u32;
pub const DMPAPER_ENV_12: u32 = 22u32;
pub const DMPAPER_ENV_14: u32 = 23u32;
pub const DMPAPER_ENV_9: u32 = 19u32;
pub const DMPAPER_ENV_B4: u32 = 33u32;
pub const DMPAPER_ENV_B5: u32 = 34u32;
pub const DMPAPER_ENV_B6: u32 = 35u32;
pub const DMPAPER_ENV_C3: u32 = 29u32;
pub const DMPAPER_ENV_C4: u32 = 30u32;
pub const DMPAPER_ENV_C5: u32 = 28u32;
pub const DMPAPER_ENV_C6: u32 = 31u32;
pub const DMPAPER_ENV_C65: u32 = 32u32;
pub const DMPAPER_ENV_DL: u32 = 27u32;
pub const DMPAPER_ENV_INVITE: u32 = 47u32;
pub const DMPAPER_ENV_ITALY: u32 = 36u32;
pub const DMPAPER_ENV_MONARCH: u32 = 37u32;
pub const DMPAPER_ENV_PERSONAL: u32 = 38u32;
pub const DMPAPER_ESHEET: u32 = 26u32;
pub const DMPAPER_EXECUTIVE: u32 = 7u32;
pub const DMPAPER_FANFOLD_LGL_GERMAN: u32 = 41u32;
pub const DMPAPER_FANFOLD_STD_GERMAN: u32 = 40u32;
pub const DMPAPER_FANFOLD_US: u32 = 39u32;
pub const DMPAPER_FOLIO: u32 = 14u32;
pub const DMPAPER_ISO_B4: u32 = 42u32;
pub const DMPAPER_JAPANESE_POSTCARD: u32 = 43u32;
pub const DMPAPER_JAPANESE_POSTCARD_ROTATED: u32 = 81u32;
pub const DMPAPER_JENV_CHOU3: u32 = 73u32;
pub const DMPAPER_JENV_CHOU3_ROTATED: u32 = 86u32;
pub const DMPAPER_JENV_CHOU4: u32 = 74u32;
pub const DMPAPER_JENV_CHOU4_ROTATED: u32 = 87u32;
pub const DMPAPER_JENV_KAKU2: u32 = 71u32;
pub const DMPAPER_JENV_KAKU2_ROTATED: u32 = 84u32;
pub const DMPAPER_JENV_KAKU3: u32 = 72u32;
pub const DMPAPER_JENV_KAKU3_ROTATED: u32 = 85u32;
pub const DMPAPER_JENV_YOU4: u32 = 91u32;
pub const DMPAPER_JENV_YOU4_ROTATED: u32 = 92u32;
pub const DMPAPER_LAST: u32 = 118u32;
pub const DMPAPER_LEDGER: u32 = 4u32;
pub const DMPAPER_LEGAL: u32 = 5u32;
pub const DMPAPER_LEGAL_EXTRA: u32 = 51u32;
pub const DMPAPER_LETTER: u32 = 1u32;
pub const DMPAPER_LETTERSMALL: u32 = 2u32;
pub const DMPAPER_LETTER_EXTRA: u32 = 50u32;
pub const DMPAPER_LETTER_EXTRA_TRANSVERSE: u32 = 56u32;
pub const DMPAPER_LETTER_PLUS: u32 = 59u32;
pub const DMPAPER_LETTER_ROTATED: u32 = 75u32;
pub const DMPAPER_LETTER_TRANSVERSE: u32 = 54u32;
pub const DMPAPER_NOTE: u32 = 18u32;
pub const DMPAPER_P16K: u32 = 93u32;
pub const DMPAPER_P16K_ROTATED: u32 = 106u32;
pub const DMPAPER_P32K: u32 = 94u32;
pub const DMPAPER_P32KBIG: u32 = 95u32;
pub const DMPAPER_P32KBIG_ROTATED: u32 = 108u32;
pub const DMPAPER_P32K_ROTATED: u32 = 107u32;
pub const DMPAPER_PENV_1: u32 = 96u32;
pub const DMPAPER_PENV_10: u32 = 105u32;
pub const DMPAPER_PENV_10_ROTATED: u32 = 118u32;
pub const DMPAPER_PENV_1_ROTATED: u32 = 109u32;
pub const DMPAPER_PENV_2: u32 = 97u32;
pub const DMPAPER_PENV_2_ROTATED: u32 = 110u32;
pub const DMPAPER_PENV_3: u32 = 98u32;
pub const DMPAPER_PENV_3_ROTATED: u32 = 111u32;
pub const DMPAPER_PENV_4: u32 = 99u32;
pub const DMPAPER_PENV_4_ROTATED: u32 = 112u32;
pub const DMPAPER_PENV_5: u32 = 100u32;
pub const DMPAPER_PENV_5_ROTATED: u32 = 113u32;
pub const DMPAPER_PENV_6: u32 = 101u32;
pub const DMPAPER_PENV_6_ROTATED: u32 = 114u32;
pub const DMPAPER_PENV_7: u32 = 102u32;
pub const DMPAPER_PENV_7_ROTATED: u32 = 115u32;
pub const DMPAPER_PENV_8: u32 = 103u32;
pub const DMPAPER_PENV_8_ROTATED: u32 = 116u32;
pub const DMPAPER_PENV_9: u32 = 104u32;
pub const DMPAPER_PENV_9_ROTATED: u32 = 117u32;
pub const DMPAPER_QUARTO: u32 = 15u32;
pub const DMPAPER_RESERVED_48: u32 = 48u32;
pub const DMPAPER_RESERVED_49: u32 = 49u32;
pub const DMPAPER_STATEMENT: u32 = 6u32;
pub const DMPAPER_TABLOID: u32 = 3u32;
pub const DMPAPER_TABLOID_EXTRA: u32 = 52u32;
pub const DMPAPER_USER: u32 = 256u32;
pub const DMRES_DRAFT: i32 = -1i32;
pub const DMRES_HIGH: i32 = -4i32;
pub const DMRES_LOW: i32 = -2i32;
pub const DMRES_MEDIUM: i32 = -3i32;
pub const DMTT_BITMAP: u32 = 1u32;
pub const DMTT_DOWNLOAD: u32 = 2u32;
pub const DMTT_DOWNLOAD_OUTLINE: u32 = 4u32;
pub const DMTT_SUBDEV: u32 = 3u32;
pub const DM_BITSPERPEL: i32 = 262144i32;
pub const DM_COLLATE: i32 = 32768i32;
pub const DM_COLOR: i32 = 2048i32;
pub const DM_COPIES: i32 = 256i32;
pub const DM_DEFAULTSOURCE: i32 = 512i32;
pub const DM_DISPLAYFIXEDOUTPUT: i32 = 536870912i32;
pub const DM_DISPLAYFLAGS: i32 = 2097152i32;
pub const DM_DISPLAYFREQUENCY: i32 = 4194304i32;
pub const DM_DISPLAYORIENTATION: i32 = 128i32;
pub const DM_DITHERTYPE: i32 = 67108864i32;
pub const DM_DUPLEX: i32 = 4096i32;
pub const DM_FORMNAME: i32 = 65536i32;
pub const DM_ICMINTENT: i32 = 16777216i32;
pub const DM_ICMMETHOD: i32 = 8388608i32;
pub const DM_INTERLACED: u32 = 2u32;
pub const DM_LOGPIXELS: i32 = 131072i32;
pub const DM_MEDIATYPE: i32 = 33554432i32;
pub const DM_NUP: i32 = 64i32;
pub const DM_ORIENTATION: i32 = 1i32;
pub const DM_PANNINGHEIGHT: i32 = 268435456i32;
pub const DM_PANNINGWIDTH: i32 = 134217728i32;
pub const DM_PAPERLENGTH: i32 = 4i32;
pub const DM_PAPERSIZE: i32 = 2i32;
pub const DM_PAPERWIDTH: i32 = 8i32;
pub const DM_PELSHEIGHT: i32 = 1048576i32;
pub const DM_PELSWIDTH: i32 = 524288i32;
pub const DM_POSITION: i32 = 32i32;
pub const DM_PRINTQUALITY: i32 = 1024i32;
pub const DM_SCALE: i32 = 16i32;
pub const DM_SPECVERSION: u32 = 1025u32;
pub const DM_TTOPTION: i32 = 16384i32;
pub const DM_YRESOLUTION: i32 = 8192i32;
pub const DOWNLOADFACE: u32 = 514u32;
pub const DOWNLOADHEADER: u32 = 4111u32;
pub const DRAFTMODE: u32 = 7u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DRAWEDGE_FLAGS(pub u32);
pub const BDR_RAISEDOUTER: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(1u32);
pub const BDR_SUNKENOUTER: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(2u32);
pub const BDR_RAISEDINNER: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(4u32);
pub const BDR_SUNKENINNER: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(8u32);
pub const BDR_OUTER: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(3u32);
pub const BDR_INNER: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(12u32);
pub const BDR_RAISED: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(5u32);
pub const BDR_SUNKEN: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(10u32);
pub const EDGE_RAISED: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(5u32);
pub const EDGE_SUNKEN: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(10u32);
pub const EDGE_ETCHED: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(6u32);
pub const EDGE_BUMP: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(9u32);
impl ::core::marker::Copy for DRAWEDGE_FLAGS {}
impl ::core::clone::Clone for DRAWEDGE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRAWEDGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRAWEDGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRAWEDGE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DRAWEDGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DRAWEDGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DRAWEDGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DRAWEDGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DRAWEDGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for DRAWEDGE_FLAGS {
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
pub const DRAWPATTERNRECT: u32 = 25u32;
pub type DRAWSTATEPROC = StdCallFnPtr<
    (
        HDC,
        super::super::Foundation::LPARAM,
        super::super::Foundation::WPARAM,
        i32,
        i32,
    ),
    super::super::Foundation::BOOL,
>;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DRAWSTATE_FLAGS(pub u32);
pub const DST_COMPLEX: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(0u32);
pub const DST_TEXT: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(1u32);
pub const DST_PREFIXTEXT: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(2u32);
pub const DST_ICON: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(3u32);
pub const DST_BITMAP: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(4u32);
pub const DSS_NORMAL: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(0u32);
pub const DSS_UNION: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(16u32);
pub const DSS_DISABLED: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(32u32);
pub const DSS_MONO: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(128u32);
pub const DSS_HIDEPREFIX: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(512u32);
pub const DSS_PREFIXONLY: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(1024u32);
pub const DSS_RIGHT: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(32768u32);
impl ::core::marker::Copy for DRAWSTATE_FLAGS {}
impl ::core::clone::Clone for DRAWSTATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRAWSTATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRAWSTATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRAWSTATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DRAWSTATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DRAWSTATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DRAWSTATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DRAWSTATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DRAWSTATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for DRAWSTATE_FLAGS {
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
pub struct DRAWTEXTPARAMS {
    pub cbSize: u32,
    pub iTabLength: i32,
    pub iLeftMargin: i32,
    pub iRightMargin: i32,
    pub uiLengthDrawn: u32,
}
impl ::core::marker::Copy for DRAWTEXTPARAMS {}
impl ::core::clone::Clone for DRAWTEXTPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRAWTEXTPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRAWTEXTPARAMS")
            .field("cbSize", &self.cbSize)
            .field("iTabLength", &self.iTabLength)
            .field("iLeftMargin", &self.iLeftMargin)
            .field("iRightMargin", &self.iRightMargin)
            .field("uiLengthDrawn", &self.uiLengthDrawn)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DRAWTEXTPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.iTabLength == other.iTabLength
            && self.iLeftMargin == other.iLeftMargin
            && self.iRightMargin == other.iRightMargin
            && self.uiLengthDrawn == other.uiLengthDrawn
    }
}
impl ::core::cmp::Eq for DRAWTEXTPARAMS {}
impl FromIntoMemory for DRAWTEXTPARAMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_cbSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_iTabLength = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_iLeftMargin = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_iRightMargin = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_uiLengthDrawn = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            cbSize: f_cbSize,
            iTabLength: f_iTabLength,
            iLeftMargin: f_iLeftMargin,
            iRightMargin: f_iRightMargin,
            uiLengthDrawn: f_uiLengthDrawn,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.cbSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.iTabLength, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.iLeftMargin, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.iRightMargin, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.uiLengthDrawn, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DRAW_CAPTION_FLAGS(pub u32);
pub const DC_ACTIVE: DRAW_CAPTION_FLAGS = DRAW_CAPTION_FLAGS(1u32);
pub const DC_BUTTONS: DRAW_CAPTION_FLAGS = DRAW_CAPTION_FLAGS(4096u32);
pub const DC_GRADIENT: DRAW_CAPTION_FLAGS = DRAW_CAPTION_FLAGS(32u32);
pub const DC_ICON: DRAW_CAPTION_FLAGS = DRAW_CAPTION_FLAGS(4u32);
pub const DC_INBUTTON: DRAW_CAPTION_FLAGS = DRAW_CAPTION_FLAGS(16u32);
pub const DC_SMALLCAP: DRAW_CAPTION_FLAGS = DRAW_CAPTION_FLAGS(2u32);
pub const DC_TEXT: DRAW_CAPTION_FLAGS = DRAW_CAPTION_FLAGS(8u32);
impl ::core::marker::Copy for DRAW_CAPTION_FLAGS {}
impl ::core::clone::Clone for DRAW_CAPTION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRAW_CAPTION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRAW_CAPTION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRAW_CAPTION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DRAW_CAPTION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DRAW_CAPTION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DRAW_CAPTION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DRAW_CAPTION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DRAW_CAPTION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for DRAW_CAPTION_FLAGS {
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
pub struct DRAW_EDGE_FLAGS(pub u32);
pub const BF_ADJUST: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(8192u32);
pub const BF_BOTTOM: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(8u32);
pub const BF_BOTTOMLEFT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(9u32);
pub const BF_BOTTOMRIGHT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(12u32);
pub const BF_DIAGONAL: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(16u32);
pub const BF_DIAGONAL_ENDBOTTOMLEFT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(25u32);
pub const BF_DIAGONAL_ENDBOTTOMRIGHT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(28u32);
pub const BF_DIAGONAL_ENDTOPLEFT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(19u32);
pub const BF_DIAGONAL_ENDTOPRIGHT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(22u32);
pub const BF_FLAT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(16384u32);
pub const BF_LEFT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(1u32);
pub const BF_MIDDLE: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(2048u32);
pub const BF_MONO: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(32768u32);
pub const BF_RECT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(15u32);
pub const BF_RIGHT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(4u32);
pub const BF_SOFT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(4096u32);
pub const BF_TOP: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(2u32);
pub const BF_TOPLEFT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(3u32);
pub const BF_TOPRIGHT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(6u32);
impl ::core::marker::Copy for DRAW_EDGE_FLAGS {}
impl ::core::clone::Clone for DRAW_EDGE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRAW_EDGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRAW_EDGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRAW_EDGE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DRAW_EDGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DRAW_EDGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DRAW_EDGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DRAW_EDGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DRAW_EDGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for DRAW_EDGE_FLAGS {
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
pub struct DRAW_TEXT_FORMAT(pub u32);
pub const DT_BOTTOM: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(8u32);
pub const DT_CALCRECT: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(1024u32);
pub const DT_CENTER: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(1u32);
pub const DT_EDITCONTROL: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(8192u32);
pub const DT_END_ELLIPSIS: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(32768u32);
pub const DT_EXPANDTABS: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(64u32);
pub const DT_EXTERNALLEADING: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(512u32);
pub const DT_HIDEPREFIX: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(1048576u32);
pub const DT_INTERNAL: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(4096u32);
pub const DT_LEFT: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(0u32);
pub const DT_MODIFYSTRING: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(65536u32);
pub const DT_NOCLIP: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(256u32);
pub const DT_NOFULLWIDTHCHARBREAK: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(524288u32);
pub const DT_NOPREFIX: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(2048u32);
pub const DT_PATH_ELLIPSIS: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(16384u32);
pub const DT_PREFIXONLY: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(2097152u32);
pub const DT_RIGHT: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(2u32);
pub const DT_RTLREADING: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(131072u32);
pub const DT_SINGLELINE: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(32u32);
pub const DT_TABSTOP: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(128u32);
pub const DT_TOP: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(0u32);
pub const DT_VCENTER: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(4u32);
pub const DT_WORDBREAK: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(16u32);
pub const DT_WORD_ELLIPSIS: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(262144u32);
impl ::core::marker::Copy for DRAW_TEXT_FORMAT {}
impl ::core::clone::Clone for DRAW_TEXT_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRAW_TEXT_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRAW_TEXT_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRAW_TEXT_FORMAT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DRAW_TEXT_FORMAT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DRAW_TEXT_FORMAT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DRAW_TEXT_FORMAT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DRAW_TEXT_FORMAT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DRAW_TEXT_FORMAT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for DRAW_TEXT_FORMAT {
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
pub const DT_CHARSTREAM: u32 = 4u32;
pub const DT_DISPFILE: u32 = 6u32;
pub const DT_METAFILE: u32 = 5u32;
pub const DT_PLOTTER: u32 = 0u32;
pub const DT_RASCAMERA: u32 = 3u32;
pub const DT_RASDISPLAY: u32 = 1u32;
pub const DT_RASPRINTER: u32 = 2u32;
pub const EASTEUROPE_CHARSET: u32 = 238u32;
pub const ELF_CULTURE_LATIN: u32 = 0u32;
pub const ELF_VENDOR_SIZE: u32 = 4u32;
pub const ELF_VERSION: u32 = 0u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EMBEDDED_FONT_PRIV_STATUS(pub u32);
pub const EMBED_PREVIEWPRINT: EMBEDDED_FONT_PRIV_STATUS = EMBEDDED_FONT_PRIV_STATUS(1u32);
pub const EMBED_EDITABLE: EMBEDDED_FONT_PRIV_STATUS = EMBEDDED_FONT_PRIV_STATUS(2u32);
pub const EMBED_INSTALLABLE: EMBEDDED_FONT_PRIV_STATUS = EMBEDDED_FONT_PRIV_STATUS(3u32);
pub const EMBED_NOEMBEDDING: EMBEDDED_FONT_PRIV_STATUS = EMBEDDED_FONT_PRIV_STATUS(4u32);
impl ::core::marker::Copy for EMBEDDED_FONT_PRIV_STATUS {}
impl ::core::clone::Clone for EMBEDDED_FONT_PRIV_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EMBEDDED_FONT_PRIV_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EMBEDDED_FONT_PRIV_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EMBEDDED_FONT_PRIV_STATUS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for EMBEDDED_FONT_PRIV_STATUS {
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
pub struct EMBED_FONT_CHARSET(pub u32);
pub const CHARSET_UNICODE: EMBED_FONT_CHARSET = EMBED_FONT_CHARSET(1u32);
pub const CHARSET_SYMBOL: EMBED_FONT_CHARSET = EMBED_FONT_CHARSET(2u32);
impl ::core::marker::Copy for EMBED_FONT_CHARSET {}
impl ::core::clone::Clone for EMBED_FONT_CHARSET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EMBED_FONT_CHARSET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EMBED_FONT_CHARSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EMBED_FONT_CHARSET").field(&self.0).finish()
    }
}
impl FromIntoMemory for EMBED_FONT_CHARSET {
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
pub struct EMR {
    pub iType: u32,
    pub nSize: u32,
}
impl ::core::marker::Copy for EMR {}
impl ::core::clone::Clone for EMR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMR")
            .field("iType", &self.iType)
            .field("nSize", &self.nSize)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMR {
    fn eq(&self, other: &Self) -> bool {
        self.iType == other.iType && self.nSize == other.nSize
    }
}
impl ::core::cmp::Eq for EMR {}
impl FromIntoMemory for EMR {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_iType = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_nSize = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            iType: f_iType,
            nSize: f_nSize,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.iType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.nSize, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct EMRARC {
    pub emr: EMR,
    pub rclBox: super::super::Foundation::RECTL,
    pub ptlStart: super::super::Foundation::POINTL,
    pub ptlEnd: super::super::Foundation::POINTL,
}
impl ::core::marker::Copy for EMRARC {}
impl ::core::clone::Clone for EMRARC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRARC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRARC")
            .field("emr", &self.emr)
            .field("rclBox", &self.rclBox)
            .field("ptlStart", &self.ptlStart)
            .field("ptlEnd", &self.ptlEnd)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRARC {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr
            && self.rclBox == other.rclBox
            && self.ptlStart == other.ptlStart
            && self.ptlEnd == other.ptlEnd
    }
}
impl ::core::cmp::Eq for EMRARC {}
impl FromIntoMemory for EMRARC {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_rclBox =
            <super::super::Foundation::RECTL as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        let f_ptlStart =
            <super::super::Foundation::POINTL as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        let f_ptlEnd =
            <super::super::Foundation::POINTL as FromIntoMemory>::from_bytes(&from[32..32 + 8]);
        Self {
            emr: f_emr,
            rclBox: f_rclBox,
            ptlStart: f_ptlStart,
            ptlEnd: f_ptlEnd,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.rclBox, &mut into[8..8 + 16]);
        FromIntoMemory::into_bytes(self.ptlStart, &mut into[24..24 + 8]);
        FromIntoMemory::into_bytes(self.ptlEnd, &mut into[32..32 + 8]);
    }
    fn size() -> usize {
        40
    }
}
pub struct EMRCREATEBRUSHINDIRECT {
    pub emr: EMR,
    pub ihBrush: u32,
    pub lb: LOGBRUSH32,
}
impl ::core::marker::Copy for EMRCREATEBRUSHINDIRECT {}
impl ::core::clone::Clone for EMRCREATEBRUSHINDIRECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRCREATEBRUSHINDIRECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRCREATEBRUSHINDIRECT")
            .field("emr", &self.emr)
            .field("ihBrush", &self.ihBrush)
            .field("lb", &self.lb)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRCREATEBRUSHINDIRECT {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ihBrush == other.ihBrush && self.lb == other.lb
    }
}
impl ::core::cmp::Eq for EMRCREATEBRUSHINDIRECT {}
impl FromIntoMemory for EMRCREATEBRUSHINDIRECT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_ihBrush = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lb = <LOGBRUSH32 as FromIntoMemory>::from_bytes(&from[12..12 + 12]);
        Self {
            emr: f_emr,
            ihBrush: f_ihBrush,
            lb: f_lb,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.ihBrush, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lb, &mut into[12..12 + 12]);
    }
    fn size() -> usize {
        24
    }
}
pub struct EMRCREATEDIBPATTERNBRUSHPT {
    pub emr: EMR,
    pub ihBrush: u32,
    pub iUsage: u32,
    pub offBmi: u32,
    pub cbBmi: u32,
    pub offBits: u32,
    pub cbBits: u32,
}
impl ::core::marker::Copy for EMRCREATEDIBPATTERNBRUSHPT {}
impl ::core::clone::Clone for EMRCREATEDIBPATTERNBRUSHPT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRCREATEDIBPATTERNBRUSHPT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRCREATEDIBPATTERNBRUSHPT")
            .field("emr", &self.emr)
            .field("ihBrush", &self.ihBrush)
            .field("iUsage", &self.iUsage)
            .field("offBmi", &self.offBmi)
            .field("cbBmi", &self.cbBmi)
            .field("offBits", &self.offBits)
            .field("cbBits", &self.cbBits)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRCREATEDIBPATTERNBRUSHPT {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr
            && self.ihBrush == other.ihBrush
            && self.iUsage == other.iUsage
            && self.offBmi == other.offBmi
            && self.cbBmi == other.cbBmi
            && self.offBits == other.offBits
            && self.cbBits == other.cbBits
    }
}
impl ::core::cmp::Eq for EMRCREATEDIBPATTERNBRUSHPT {}
impl FromIntoMemory for EMRCREATEDIBPATTERNBRUSHPT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_ihBrush = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_iUsage = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_offBmi = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_cbBmi = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_offBits = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_cbBits = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            emr: f_emr,
            ihBrush: f_ihBrush,
            iUsage: f_iUsage,
            offBmi: f_offBmi,
            cbBmi: f_cbBmi,
            offBits: f_offBits,
            cbBits: f_cbBits,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.ihBrush, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.iUsage, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.offBmi, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.cbBmi, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.offBits, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.cbBits, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
pub struct EMRCREATEMONOBRUSH {
    pub emr: EMR,
    pub ihBrush: u32,
    pub iUsage: u32,
    pub offBmi: u32,
    pub cbBmi: u32,
    pub offBits: u32,
    pub cbBits: u32,
}
impl ::core::marker::Copy for EMRCREATEMONOBRUSH {}
impl ::core::clone::Clone for EMRCREATEMONOBRUSH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRCREATEMONOBRUSH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRCREATEMONOBRUSH")
            .field("emr", &self.emr)
            .field("ihBrush", &self.ihBrush)
            .field("iUsage", &self.iUsage)
            .field("offBmi", &self.offBmi)
            .field("cbBmi", &self.cbBmi)
            .field("offBits", &self.offBits)
            .field("cbBits", &self.cbBits)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRCREATEMONOBRUSH {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr
            && self.ihBrush == other.ihBrush
            && self.iUsage == other.iUsage
            && self.offBmi == other.offBmi
            && self.cbBmi == other.cbBmi
            && self.offBits == other.offBits
            && self.cbBits == other.cbBits
    }
}
impl ::core::cmp::Eq for EMRCREATEMONOBRUSH {}
impl FromIntoMemory for EMRCREATEMONOBRUSH {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_ihBrush = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_iUsage = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_offBmi = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_cbBmi = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_offBits = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_cbBits = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            emr: f_emr,
            ihBrush: f_ihBrush,
            iUsage: f_iUsage,
            offBmi: f_offBmi,
            cbBmi: f_cbBmi,
            offBits: f_offBits,
            cbBits: f_cbBits,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.ihBrush, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.iUsage, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.offBmi, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.cbBmi, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.offBits, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.cbBits, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
pub struct EMRCREATEPALETTE {
    pub emr: EMR,
    pub ihPal: u32,
    pub lgpl: LOGPALETTE,
}
impl ::core::marker::Copy for EMRCREATEPALETTE {}
impl ::core::clone::Clone for EMRCREATEPALETTE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRCREATEPALETTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRCREATEPALETTE")
            .field("emr", &self.emr)
            .field("ihPal", &self.ihPal)
            .field("lgpl", &self.lgpl)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRCREATEPALETTE {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ihPal == other.ihPal && self.lgpl == other.lgpl
    }
}
impl ::core::cmp::Eq for EMRCREATEPALETTE {}
impl FromIntoMemory for EMRCREATEPALETTE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_ihPal = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lgpl = <LOGPALETTE as FromIntoMemory>::from_bytes(&from[12..12 + 8]);
        Self {
            emr: f_emr,
            ihPal: f_ihPal,
            lgpl: f_lgpl,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.ihPal, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lgpl, &mut into[12..12 + 8]);
    }
    fn size() -> usize {
        20
    }
}
pub struct EMRCREATEPEN {
    pub emr: EMR,
    pub ihPen: u32,
    pub lopn: LOGPEN,
}
impl ::core::marker::Copy for EMRCREATEPEN {}
impl ::core::clone::Clone for EMRCREATEPEN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRCREATEPEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRCREATEPEN")
            .field("emr", &self.emr)
            .field("ihPen", &self.ihPen)
            .field("lopn", &self.lopn)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRCREATEPEN {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ihPen == other.ihPen && self.lopn == other.lopn
    }
}
impl ::core::cmp::Eq for EMRCREATEPEN {}
impl FromIntoMemory for EMRCREATEPEN {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_ihPen = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lopn = <LOGPEN as FromIntoMemory>::from_bytes(&from[12..12 + 16]);
        Self {
            emr: f_emr,
            ihPen: f_ihPen,
            lopn: f_lopn,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.ihPen, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lopn, &mut into[12..12 + 16]);
    }
    fn size() -> usize {
        28
    }
}
pub struct EMRELLIPSE {
    pub emr: EMR,
    pub rclBox: super::super::Foundation::RECTL,
}
impl ::core::marker::Copy for EMRELLIPSE {}
impl ::core::clone::Clone for EMRELLIPSE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRELLIPSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRELLIPSE")
            .field("emr", &self.emr)
            .field("rclBox", &self.rclBox)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRELLIPSE {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclBox == other.rclBox
    }
}
impl ::core::cmp::Eq for EMRELLIPSE {}
impl FromIntoMemory for EMRELLIPSE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_rclBox =
            <super::super::Foundation::RECTL as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        Self {
            emr: f_emr,
            rclBox: f_rclBox,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.rclBox, &mut into[8..8 + 16]);
    }
    fn size() -> usize {
        24
    }
}
pub struct EMREOF {
    pub emr: EMR,
    pub nPalEntries: u32,
    pub offPalEntries: u32,
    pub nSizeLast: u32,
}
impl ::core::marker::Copy for EMREOF {}
impl ::core::clone::Clone for EMREOF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMREOF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMREOF")
            .field("emr", &self.emr)
            .field("nPalEntries", &self.nPalEntries)
            .field("offPalEntries", &self.offPalEntries)
            .field("nSizeLast", &self.nSizeLast)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMREOF {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr
            && self.nPalEntries == other.nPalEntries
            && self.offPalEntries == other.offPalEntries
            && self.nSizeLast == other.nSizeLast
    }
}
impl ::core::cmp::Eq for EMREOF {}
impl FromIntoMemory for EMREOF {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_nPalEntries = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_offPalEntries = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_nSizeLast = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            emr: f_emr,
            nPalEntries: f_nPalEntries,
            offPalEntries: f_offPalEntries,
            nSizeLast: f_nSizeLast,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.nPalEntries, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.offPalEntries, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.nSizeLast, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub struct EMREXCLUDECLIPRECT {
    pub emr: EMR,
    pub rclClip: super::super::Foundation::RECTL,
}
impl ::core::marker::Copy for EMREXCLUDECLIPRECT {}
impl ::core::clone::Clone for EMREXCLUDECLIPRECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMREXCLUDECLIPRECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMREXCLUDECLIPRECT")
            .field("emr", &self.emr)
            .field("rclClip", &self.rclClip)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMREXCLUDECLIPRECT {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclClip == other.rclClip
    }
}
impl ::core::cmp::Eq for EMREXCLUDECLIPRECT {}
impl FromIntoMemory for EMREXCLUDECLIPRECT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_rclClip =
            <super::super::Foundation::RECTL as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        Self {
            emr: f_emr,
            rclClip: f_rclClip,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.rclClip, &mut into[8..8 + 16]);
    }
    fn size() -> usize {
        24
    }
}
pub struct EMREXTCREATEFONTINDIRECTW {
    pub emr: EMR,
    pub ihFont: u32,
    pub elfw: EXTLOGFONTW,
}
impl ::core::marker::Copy for EMREXTCREATEFONTINDIRECTW {}
impl ::core::clone::Clone for EMREXTCREATEFONTINDIRECTW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMREXTCREATEFONTINDIRECTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMREXTCREATEFONTINDIRECTW")
            .field("emr", &self.emr)
            .field("ihFont", &self.ihFont)
            .field("elfw", &self.elfw)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMREXTCREATEFONTINDIRECTW {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ihFont == other.ihFont && self.elfw == other.elfw
    }
}
impl ::core::cmp::Eq for EMREXTCREATEFONTINDIRECTW {}
impl FromIntoMemory for EMREXTCREATEFONTINDIRECTW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 204);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_ihFont = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_elfw = <EXTLOGFONTW as FromIntoMemory>::from_bytes(&from[12..12 + 192]);
        Self {
            emr: f_emr,
            ihFont: f_ihFont,
            elfw: f_elfw,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 204);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.ihFont, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.elfw, &mut into[12..12 + 192]);
    }
    fn size() -> usize {
        204
    }
}
pub struct EMREXTCREATEPEN {
    pub emr: EMR,
    pub ihPen: u32,
    pub offBmi: u32,
    pub cbBmi: u32,
    pub offBits: u32,
    pub cbBits: u32,
    pub elp: EXTLOGPEN32,
}
impl ::core::marker::Copy for EMREXTCREATEPEN {}
impl ::core::clone::Clone for EMREXTCREATEPEN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMREXTCREATEPEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMREXTCREATEPEN")
            .field("emr", &self.emr)
            .field("ihPen", &self.ihPen)
            .field("offBmi", &self.offBmi)
            .field("cbBmi", &self.cbBmi)
            .field("offBits", &self.offBits)
            .field("cbBits", &self.cbBits)
            .field("elp", &self.elp)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMREXTCREATEPEN {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr
            && self.ihPen == other.ihPen
            && self.offBmi == other.offBmi
            && self.cbBmi == other.cbBmi
            && self.offBits == other.offBits
            && self.cbBits == other.cbBits
            && self.elp == other.elp
    }
}
impl ::core::cmp::Eq for EMREXTCREATEPEN {}
impl FromIntoMemory for EMREXTCREATEPEN {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 56);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_ihPen = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_offBmi = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_cbBmi = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_offBits = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_cbBits = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_elp = <EXTLOGPEN32 as FromIntoMemory>::from_bytes(&from[28..28 + 28]);
        Self {
            emr: f_emr,
            ihPen: f_ihPen,
            offBmi: f_offBmi,
            cbBmi: f_cbBmi,
            offBits: f_offBits,
            cbBits: f_cbBits,
            elp: f_elp,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 56);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.ihPen, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.offBmi, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.cbBmi, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.offBits, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.cbBits, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.elp, &mut into[28..28 + 28]);
    }
    fn size() -> usize {
        56
    }
}
pub struct EMREXTESCAPE {
    pub emr: EMR,
    pub iEscape: i32,
    pub cbEscData: i32,
    pub EscData: [u8; 1],
}
impl ::core::marker::Copy for EMREXTESCAPE {}
impl ::core::clone::Clone for EMREXTESCAPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMREXTESCAPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMREXTESCAPE")
            .field("emr", &self.emr)
            .field("iEscape", &self.iEscape)
            .field("cbEscData", &self.cbEscData)
            .field("EscData", &self.EscData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMREXTESCAPE {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr
            && self.iEscape == other.iEscape
            && self.cbEscData == other.cbEscData
            && self.EscData == other.EscData
    }
}
impl ::core::cmp::Eq for EMREXTESCAPE {}
impl FromIntoMemory for EMREXTESCAPE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_iEscape = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_cbEscData = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_EscData = <[u8; 1] as FromIntoMemory>::from_bytes(&from[16..16 + 1]);
        Self {
            emr: f_emr,
            iEscape: f_iEscape,
            cbEscData: f_cbEscData,
            EscData: f_EscData,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.iEscape, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.cbEscData, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.EscData, &mut into[16..16 + 1]);
    }
    fn size() -> usize {
        20
    }
}
pub struct EMREXTFLOODFILL {
    pub emr: EMR,
    pub ptlStart: super::super::Foundation::POINTL,
    pub crColor: u32,
    pub iMode: u32,
}
impl ::core::marker::Copy for EMREXTFLOODFILL {}
impl ::core::clone::Clone for EMREXTFLOODFILL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMREXTFLOODFILL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMREXTFLOODFILL")
            .field("emr", &self.emr)
            .field("ptlStart", &self.ptlStart)
            .field("crColor", &self.crColor)
            .field("iMode", &self.iMode)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMREXTFLOODFILL {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr
            && self.ptlStart == other.ptlStart
            && self.crColor == other.crColor
            && self.iMode == other.iMode
    }
}
impl ::core::cmp::Eq for EMREXTFLOODFILL {}
impl FromIntoMemory for EMREXTFLOODFILL {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_ptlStart =
            <super::super::Foundation::POINTL as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_crColor = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_iMode = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            emr: f_emr,
            ptlStart: f_ptlStart,
            crColor: f_crColor,
            iMode: f_iMode,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.ptlStart, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.crColor, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.iMode, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct EMREXTSELECTCLIPRGN {
    pub emr: EMR,
    pub cbRgnData: u32,
    pub iMode: u32,
    pub RgnData: [u8; 1],
}
impl ::core::marker::Copy for EMREXTSELECTCLIPRGN {}
impl ::core::clone::Clone for EMREXTSELECTCLIPRGN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMREXTSELECTCLIPRGN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMREXTSELECTCLIPRGN")
            .field("emr", &self.emr)
            .field("cbRgnData", &self.cbRgnData)
            .field("iMode", &self.iMode)
            .field("RgnData", &self.RgnData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMREXTSELECTCLIPRGN {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr
            && self.cbRgnData == other.cbRgnData
            && self.iMode == other.iMode
            && self.RgnData == other.RgnData
    }
}
impl ::core::cmp::Eq for EMREXTSELECTCLIPRGN {}
impl FromIntoMemory for EMREXTSELECTCLIPRGN {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_cbRgnData = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_iMode = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_RgnData = <[u8; 1] as FromIntoMemory>::from_bytes(&from[16..16 + 1]);
        Self {
            emr: f_emr,
            cbRgnData: f_cbRgnData,
            iMode: f_iMode,
            RgnData: f_RgnData,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.cbRgnData, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.iMode, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.RgnData, &mut into[16..16 + 1]);
    }
    fn size() -> usize {
        20
    }
}
pub struct EMRFILLPATH {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
}
impl ::core::marker::Copy for EMRFILLPATH {}
impl ::core::clone::Clone for EMRFILLPATH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRFILLPATH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRFILLPATH")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRFILLPATH {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclBounds == other.rclBounds
    }
}
impl ::core::cmp::Eq for EMRFILLPATH {}
impl FromIntoMemory for EMRFILLPATH {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_rclBounds =
            <super::super::Foundation::RECTL as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        Self {
            emr: f_emr,
            rclBounds: f_rclBounds,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.rclBounds, &mut into[8..8 + 16]);
    }
    fn size() -> usize {
        24
    }
}
pub struct EMRFILLRGN {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cbRgnData: u32,
    pub ihBrush: u32,
    pub RgnData: [u8; 1],
}
impl ::core::marker::Copy for EMRFILLRGN {}
impl ::core::clone::Clone for EMRFILLRGN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRFILLRGN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRFILLRGN")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .field("cbRgnData", &self.cbRgnData)
            .field("ihBrush", &self.ihBrush)
            .field("RgnData", &self.RgnData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRFILLRGN {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr
            && self.rclBounds == other.rclBounds
            && self.cbRgnData == other.cbRgnData
            && self.ihBrush == other.ihBrush
            && self.RgnData == other.RgnData
    }
}
impl ::core::cmp::Eq for EMRFILLRGN {}
impl FromIntoMemory for EMRFILLRGN {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 36);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_rclBounds =
            <super::super::Foundation::RECTL as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        let f_cbRgnData = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_ihBrush = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_RgnData = <[u8; 1] as FromIntoMemory>::from_bytes(&from[32..32 + 1]);
        Self {
            emr: f_emr,
            rclBounds: f_rclBounds,
            cbRgnData: f_cbRgnData,
            ihBrush: f_ihBrush,
            RgnData: f_RgnData,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 36);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.rclBounds, &mut into[8..8 + 16]);
        FromIntoMemory::into_bytes(self.cbRgnData, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.ihBrush, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.RgnData, &mut into[32..32 + 1]);
    }
    fn size() -> usize {
        36
    }
}
pub struct EMRFORMAT {
    pub dSignature: u32,
    pub nVersion: u32,
    pub cbData: u32,
    pub offData: u32,
}
impl ::core::marker::Copy for EMRFORMAT {}
impl ::core::clone::Clone for EMRFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRFORMAT")
            .field("dSignature", &self.dSignature)
            .field("nVersion", &self.nVersion)
            .field("cbData", &self.cbData)
            .field("offData", &self.offData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.dSignature == other.dSignature
            && self.nVersion == other.nVersion
            && self.cbData == other.cbData
            && self.offData == other.offData
    }
}
impl ::core::cmp::Eq for EMRFORMAT {}
impl FromIntoMemory for EMRFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dSignature = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_nVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_cbData = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_offData = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            dSignature: f_dSignature,
            nVersion: f_nVersion,
            cbData: f_cbData,
            offData: f_offData,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dSignature, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.nVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.cbData, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.offData, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct EMRFRAMERGN {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cbRgnData: u32,
    pub ihBrush: u32,
    pub szlStroke: super::super::Foundation::SIZE,
    pub RgnData: [u8; 1],
}
impl ::core::marker::Copy for EMRFRAMERGN {}
impl ::core::clone::Clone for EMRFRAMERGN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRFRAMERGN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRFRAMERGN")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .field("cbRgnData", &self.cbRgnData)
            .field("ihBrush", &self.ihBrush)
            .field("szlStroke", &self.szlStroke)
            .field("RgnData", &self.RgnData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRFRAMERGN {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr
            && self.rclBounds == other.rclBounds
            && self.cbRgnData == other.cbRgnData
            && self.ihBrush == other.ihBrush
            && self.szlStroke == other.szlStroke
            && self.RgnData == other.RgnData
    }
}
impl ::core::cmp::Eq for EMRFRAMERGN {}
impl FromIntoMemory for EMRFRAMERGN {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 44);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_rclBounds =
            <super::super::Foundation::RECTL as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        let f_cbRgnData = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_ihBrush = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_szlStroke =
            <super::super::Foundation::SIZE as FromIntoMemory>::from_bytes(&from[32..32 + 8]);
        let f_RgnData = <[u8; 1] as FromIntoMemory>::from_bytes(&from[40..40 + 1]);
        Self {
            emr: f_emr,
            rclBounds: f_rclBounds,
            cbRgnData: f_cbRgnData,
            ihBrush: f_ihBrush,
            szlStroke: f_szlStroke,
            RgnData: f_RgnData,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 44);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.rclBounds, &mut into[8..8 + 16]);
        FromIntoMemory::into_bytes(self.cbRgnData, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.ihBrush, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.szlStroke, &mut into[32..32 + 8]);
        FromIntoMemory::into_bytes(self.RgnData, &mut into[40..40 + 1]);
    }
    fn size() -> usize {
        44
    }
}
pub struct EMRGDICOMMENT {
    pub emr: EMR,
    pub cbData: u32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for EMRGDICOMMENT {}
impl ::core::clone::Clone for EMRGDICOMMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRGDICOMMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRGDICOMMENT")
            .field("emr", &self.emr)
            .field("cbData", &self.cbData)
            .field("Data", &self.Data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRGDICOMMENT {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.cbData == other.cbData && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for EMRGDICOMMENT {}
impl FromIntoMemory for EMRGDICOMMENT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_cbData = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_Data = <[u8; 1] as FromIntoMemory>::from_bytes(&from[12..12 + 1]);
        Self {
            emr: f_emr,
            cbData: f_cbData,
            Data: f_Data,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.cbData, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.Data, &mut into[12..12 + 1]);
    }
    fn size() -> usize {
        16
    }
}
pub struct EMRGLSBOUNDEDRECORD {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cbData: u32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for EMRGLSBOUNDEDRECORD {}
impl ::core::clone::Clone for EMRGLSBOUNDEDRECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRGLSBOUNDEDRECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRGLSBOUNDEDRECORD")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .field("cbData", &self.cbData)
            .field("Data", &self.Data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRGLSBOUNDEDRECORD {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr
            && self.rclBounds == other.rclBounds
            && self.cbData == other.cbData
            && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for EMRGLSBOUNDEDRECORD {}
impl FromIntoMemory for EMRGLSBOUNDEDRECORD {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_rclBounds =
            <super::super::Foundation::RECTL as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        let f_cbData = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_Data = <[u8; 1] as FromIntoMemory>::from_bytes(&from[28..28 + 1]);
        Self {
            emr: f_emr,
            rclBounds: f_rclBounds,
            cbData: f_cbData,
            Data: f_Data,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.rclBounds, &mut into[8..8 + 16]);
        FromIntoMemory::into_bytes(self.cbData, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.Data, &mut into[28..28 + 1]);
    }
    fn size() -> usize {
        32
    }
}
pub struct EMRGLSRECORD {
    pub emr: EMR,
    pub cbData: u32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for EMRGLSRECORD {}
impl ::core::clone::Clone for EMRGLSRECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRGLSRECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRGLSRECORD")
            .field("emr", &self.emr)
            .field("cbData", &self.cbData)
            .field("Data", &self.Data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRGLSRECORD {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.cbData == other.cbData && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for EMRGLSRECORD {}
impl FromIntoMemory for EMRGLSRECORD {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_cbData = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_Data = <[u8; 1] as FromIntoMemory>::from_bytes(&from[12..12 + 1]);
        Self {
            emr: f_emr,
            cbData: f_cbData,
            Data: f_Data,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.cbData, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.Data, &mut into[12..12 + 1]);
    }
    fn size() -> usize {
        16
    }
}
pub struct EMRGRADIENTFILL {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub nVer: u32,
    pub nTri: u32,
    pub ulMode: GRADIENT_FILL,
    pub Ver: [TRIVERTEX; 1],
}
impl ::core::marker::Copy for EMRGRADIENTFILL {}
impl ::core::clone::Clone for EMRGRADIENTFILL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRGRADIENTFILL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRGRADIENTFILL")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .field("nVer", &self.nVer)
            .field("nTri", &self.nTri)
            .field("ulMode", &self.ulMode)
            .field("Ver", &self.Ver)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRGRADIENTFILL {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr
            && self.rclBounds == other.rclBounds
            && self.nVer == other.nVer
            && self.nTri == other.nTri
            && self.ulMode == other.ulMode
            && self.Ver == other.Ver
    }
}
impl ::core::cmp::Eq for EMRGRADIENTFILL {}
impl FromIntoMemory for EMRGRADIENTFILL {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 52);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_rclBounds =
            <super::super::Foundation::RECTL as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        let f_nVer = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_nTri = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_ulMode = <GRADIENT_FILL as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_Ver = <[TRIVERTEX; 1] as FromIntoMemory>::from_bytes(&from[36..36 + 16]);
        Self {
            emr: f_emr,
            rclBounds: f_rclBounds,
            nVer: f_nVer,
            nTri: f_nTri,
            ulMode: f_ulMode,
            Ver: f_Ver,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 52);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.rclBounds, &mut into[8..8 + 16]);
        FromIntoMemory::into_bytes(self.nVer, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.nTri, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.ulMode, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.Ver, &mut into[36..36 + 16]);
    }
    fn size() -> usize {
        52
    }
}
pub struct EMRINVERTRGN {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cbRgnData: u32,
    pub RgnData: [u8; 1],
}
impl ::core::marker::Copy for EMRINVERTRGN {}
impl ::core::clone::Clone for EMRINVERTRGN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRINVERTRGN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRINVERTRGN")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .field("cbRgnData", &self.cbRgnData)
            .field("RgnData", &self.RgnData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRINVERTRGN {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr
            && self.rclBounds == other.rclBounds
            && self.cbRgnData == other.cbRgnData
            && self.RgnData == other.RgnData
    }
}
impl ::core::cmp::Eq for EMRINVERTRGN {}
impl FromIntoMemory for EMRINVERTRGN {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_rclBounds =
            <super::super::Foundation::RECTL as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        let f_cbRgnData = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_RgnData = <[u8; 1] as FromIntoMemory>::from_bytes(&from[28..28 + 1]);
        Self {
            emr: f_emr,
            rclBounds: f_rclBounds,
            cbRgnData: f_cbRgnData,
            RgnData: f_RgnData,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.rclBounds, &mut into[8..8 + 16]);
        FromIntoMemory::into_bytes(self.cbRgnData, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.RgnData, &mut into[28..28 + 1]);
    }
    fn size() -> usize {
        32
    }
}
pub struct EMRLINETO {
    pub emr: EMR,
    pub ptl: super::super::Foundation::POINTL,
}
impl ::core::marker::Copy for EMRLINETO {}
impl ::core::clone::Clone for EMRLINETO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRLINETO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRLINETO")
            .field("emr", &self.emr)
            .field("ptl", &self.ptl)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRLINETO {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ptl == other.ptl
    }
}
impl ::core::cmp::Eq for EMRLINETO {}
impl FromIntoMemory for EMRLINETO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_ptl =
            <super::super::Foundation::POINTL as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        Self {
            emr: f_emr,
            ptl: f_ptl,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.ptl, &mut into[8..8 + 8]);
    }
    fn size() -> usize {
        16
    }
}
pub struct EMRNAMEDESCAPE {
    pub emr: EMR,
    pub iEscape: i32,
    pub cbDriver: i32,
    pub cbEscData: i32,
    pub EscData: [u8; 1],
}
impl ::core::marker::Copy for EMRNAMEDESCAPE {}
impl ::core::clone::Clone for EMRNAMEDESCAPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRNAMEDESCAPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRNAMEDESCAPE")
            .field("emr", &self.emr)
            .field("iEscape", &self.iEscape)
            .field("cbDriver", &self.cbDriver)
            .field("cbEscData", &self.cbEscData)
            .field("EscData", &self.EscData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRNAMEDESCAPE {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr
            && self.iEscape == other.iEscape
            && self.cbDriver == other.cbDriver
            && self.cbEscData == other.cbEscData
            && self.EscData == other.EscData
    }
}
impl ::core::cmp::Eq for EMRNAMEDESCAPE {}
impl FromIntoMemory for EMRNAMEDESCAPE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_iEscape = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_cbDriver = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_cbEscData = <i32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_EscData = <[u8; 1] as FromIntoMemory>::from_bytes(&from[20..20 + 1]);
        Self {
            emr: f_emr,
            iEscape: f_iEscape,
            cbDriver: f_cbDriver,
            cbEscData: f_cbEscData,
            EscData: f_EscData,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.iEscape, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.cbDriver, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.cbEscData, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.EscData, &mut into[20..20 + 1]);
    }
    fn size() -> usize {
        24
    }
}
pub struct EMROFFSETCLIPRGN {
    pub emr: EMR,
    pub ptlOffset: super::super::Foundation::POINTL,
}
impl ::core::marker::Copy for EMROFFSETCLIPRGN {}
impl ::core::clone::Clone for EMROFFSETCLIPRGN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMROFFSETCLIPRGN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMROFFSETCLIPRGN")
            .field("emr", &self.emr)
            .field("ptlOffset", &self.ptlOffset)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMROFFSETCLIPRGN {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ptlOffset == other.ptlOffset
    }
}
impl ::core::cmp::Eq for EMROFFSETCLIPRGN {}
impl FromIntoMemory for EMROFFSETCLIPRGN {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_ptlOffset =
            <super::super::Foundation::POINTL as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        Self {
            emr: f_emr,
            ptlOffset: f_ptlOffset,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.ptlOffset, &mut into[8..8 + 8]);
    }
    fn size() -> usize {
        16
    }
}
pub struct EMRPOLYDRAW {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cptl: u32,
    pub aptl: [super::super::Foundation::POINTL; 1],
    pub abTypes: [u8; 1],
}
impl ::core::marker::Copy for EMRPOLYDRAW {}
impl ::core::clone::Clone for EMRPOLYDRAW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRPOLYDRAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRPOLYDRAW")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .field("cptl", &self.cptl)
            .field("aptl", &self.aptl)
            .field("abTypes", &self.abTypes)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRPOLYDRAW {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr
            && self.rclBounds == other.rclBounds
            && self.cptl == other.cptl
            && self.aptl == other.aptl
            && self.abTypes == other.abTypes
    }
}
impl ::core::cmp::Eq for EMRPOLYDRAW {}
impl FromIntoMemory for EMRPOLYDRAW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_rclBounds =
            <super::super::Foundation::RECTL as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        let f_cptl = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_aptl = <[super::super::Foundation::POINTL; 1] as FromIntoMemory>::from_bytes(
            &from[28..28 + 8],
        );
        let f_abTypes = <[u8; 1] as FromIntoMemory>::from_bytes(&from[36..36 + 1]);
        Self {
            emr: f_emr,
            rclBounds: f_rclBounds,
            cptl: f_cptl,
            aptl: f_aptl,
            abTypes: f_abTypes,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.rclBounds, &mut into[8..8 + 16]);
        FromIntoMemory::into_bytes(self.cptl, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.aptl, &mut into[28..28 + 8]);
        FromIntoMemory::into_bytes(self.abTypes, &mut into[36..36 + 1]);
    }
    fn size() -> usize {
        40
    }
}
pub struct EMRPOLYDRAW16 {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cpts: u32,
    pub apts: [super::super::Foundation::POINTS; 1],
    pub abTypes: [u8; 1],
}
impl ::core::marker::Copy for EMRPOLYDRAW16 {}
impl ::core::clone::Clone for EMRPOLYDRAW16 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRPOLYDRAW16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRPOLYDRAW16")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .field("cpts", &self.cpts)
            .field("apts", &self.apts)
            .field("abTypes", &self.abTypes)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRPOLYDRAW16 {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr
            && self.rclBounds == other.rclBounds
            && self.cpts == other.cpts
            && self.apts == other.apts
            && self.abTypes == other.abTypes
    }
}
impl ::core::cmp::Eq for EMRPOLYDRAW16 {}
impl FromIntoMemory for EMRPOLYDRAW16 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 36);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_rclBounds =
            <super::super::Foundation::RECTL as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        let f_cpts = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_apts = <[super::super::Foundation::POINTS; 1] as FromIntoMemory>::from_bytes(
            &from[28..28 + 4],
        );
        let f_abTypes = <[u8; 1] as FromIntoMemory>::from_bytes(&from[32..32 + 1]);
        Self {
            emr: f_emr,
            rclBounds: f_rclBounds,
            cpts: f_cpts,
            apts: f_apts,
            abTypes: f_abTypes,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 36);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.rclBounds, &mut into[8..8 + 16]);
        FromIntoMemory::into_bytes(self.cpts, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.apts, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.abTypes, &mut into[32..32 + 1]);
    }
    fn size() -> usize {
        36
    }
}
pub struct EMRPOLYLINE {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cptl: u32,
    pub aptl: [super::super::Foundation::POINTL; 1],
}
impl ::core::marker::Copy for EMRPOLYLINE {}
impl ::core::clone::Clone for EMRPOLYLINE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRPOLYLINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRPOLYLINE")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .field("cptl", &self.cptl)
            .field("aptl", &self.aptl)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRPOLYLINE {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr
            && self.rclBounds == other.rclBounds
            && self.cptl == other.cptl
            && self.aptl == other.aptl
    }
}
impl ::core::cmp::Eq for EMRPOLYLINE {}
impl FromIntoMemory for EMRPOLYLINE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 36);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_rclBounds =
            <super::super::Foundation::RECTL as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        let f_cptl = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_aptl = <[super::super::Foundation::POINTL; 1] as FromIntoMemory>::from_bytes(
            &from[28..28 + 8],
        );
        Self {
            emr: f_emr,
            rclBounds: f_rclBounds,
            cptl: f_cptl,
            aptl: f_aptl,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 36);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.rclBounds, &mut into[8..8 + 16]);
        FromIntoMemory::into_bytes(self.cptl, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.aptl, &mut into[28..28 + 8]);
    }
    fn size() -> usize {
        36
    }
}
pub struct EMRPOLYLINE16 {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cpts: u32,
    pub apts: [super::super::Foundation::POINTS; 1],
}
impl ::core::marker::Copy for EMRPOLYLINE16 {}
impl ::core::clone::Clone for EMRPOLYLINE16 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRPOLYLINE16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRPOLYLINE16")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .field("cpts", &self.cpts)
            .field("apts", &self.apts)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRPOLYLINE16 {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr
            && self.rclBounds == other.rclBounds
            && self.cpts == other.cpts
            && self.apts == other.apts
    }
}
impl ::core::cmp::Eq for EMRPOLYLINE16 {}
impl FromIntoMemory for EMRPOLYLINE16 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_rclBounds =
            <super::super::Foundation::RECTL as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        let f_cpts = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_apts = <[super::super::Foundation::POINTS; 1] as FromIntoMemory>::from_bytes(
            &from[28..28 + 4],
        );
        Self {
            emr: f_emr,
            rclBounds: f_rclBounds,
            cpts: f_cpts,
            apts: f_apts,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.rclBounds, &mut into[8..8 + 16]);
        FromIntoMemory::into_bytes(self.cpts, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.apts, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
pub struct EMRPOLYPOLYLINE {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub nPolys: u32,
    pub cptl: u32,
    pub aPolyCounts: [u32; 1],
    pub aptl: [super::super::Foundation::POINTL; 1],
}
impl ::core::marker::Copy for EMRPOLYPOLYLINE {}
impl ::core::clone::Clone for EMRPOLYPOLYLINE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRPOLYPOLYLINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRPOLYPOLYLINE")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .field("nPolys", &self.nPolys)
            .field("cptl", &self.cptl)
            .field("aPolyCounts", &self.aPolyCounts)
            .field("aptl", &self.aptl)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRPOLYPOLYLINE {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr
            && self.rclBounds == other.rclBounds
            && self.nPolys == other.nPolys
            && self.cptl == other.cptl
            && self.aPolyCounts == other.aPolyCounts
            && self.aptl == other.aptl
    }
}
impl ::core::cmp::Eq for EMRPOLYPOLYLINE {}
impl FromIntoMemory for EMRPOLYPOLYLINE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 44);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_rclBounds =
            <super::super::Foundation::RECTL as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        let f_nPolys = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_cptl = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_aPolyCounts = <[u32; 1] as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_aptl = <[super::super::Foundation::POINTL; 1] as FromIntoMemory>::from_bytes(
            &from[36..36 + 8],
        );
        Self {
            emr: f_emr,
            rclBounds: f_rclBounds,
            nPolys: f_nPolys,
            cptl: f_cptl,
            aPolyCounts: f_aPolyCounts,
            aptl: f_aptl,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 44);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.rclBounds, &mut into[8..8 + 16]);
        FromIntoMemory::into_bytes(self.nPolys, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.cptl, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.aPolyCounts, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.aptl, &mut into[36..36 + 8]);
    }
    fn size() -> usize {
        44
    }
}
pub struct EMRPOLYPOLYLINE16 {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub nPolys: u32,
    pub cpts: u32,
    pub aPolyCounts: [u32; 1],
    pub apts: [super::super::Foundation::POINTS; 1],
}
impl ::core::marker::Copy for EMRPOLYPOLYLINE16 {}
impl ::core::clone::Clone for EMRPOLYPOLYLINE16 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRPOLYPOLYLINE16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRPOLYPOLYLINE16")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .field("nPolys", &self.nPolys)
            .field("cpts", &self.cpts)
            .field("aPolyCounts", &self.aPolyCounts)
            .field("apts", &self.apts)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRPOLYPOLYLINE16 {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr
            && self.rclBounds == other.rclBounds
            && self.nPolys == other.nPolys
            && self.cpts == other.cpts
            && self.aPolyCounts == other.aPolyCounts
            && self.apts == other.apts
    }
}
impl ::core::cmp::Eq for EMRPOLYPOLYLINE16 {}
impl FromIntoMemory for EMRPOLYPOLYLINE16 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_rclBounds =
            <super::super::Foundation::RECTL as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        let f_nPolys = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_cpts = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_aPolyCounts = <[u32; 1] as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_apts = <[super::super::Foundation::POINTS; 1] as FromIntoMemory>::from_bytes(
            &from[36..36 + 4],
        );
        Self {
            emr: f_emr,
            rclBounds: f_rclBounds,
            nPolys: f_nPolys,
            cpts: f_cpts,
            aPolyCounts: f_aPolyCounts,
            apts: f_apts,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.rclBounds, &mut into[8..8 + 16]);
        FromIntoMemory::into_bytes(self.nPolys, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.cpts, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.aPolyCounts, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.apts, &mut into[36..36 + 4]);
    }
    fn size() -> usize {
        40
    }
}
pub struct EMRRESIZEPALETTE {
    pub emr: EMR,
    pub ihPal: u32,
    pub cEntries: u32,
}
impl ::core::marker::Copy for EMRRESIZEPALETTE {}
impl ::core::clone::Clone for EMRRESIZEPALETTE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRRESIZEPALETTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRRESIZEPALETTE")
            .field("emr", &self.emr)
            .field("ihPal", &self.ihPal)
            .field("cEntries", &self.cEntries)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRRESIZEPALETTE {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ihPal == other.ihPal && self.cEntries == other.cEntries
    }
}
impl ::core::cmp::Eq for EMRRESIZEPALETTE {}
impl FromIntoMemory for EMRRESIZEPALETTE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_ihPal = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_cEntries = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            emr: f_emr,
            ihPal: f_ihPal,
            cEntries: f_cEntries,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.ihPal, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.cEntries, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct EMRRESTOREDC {
    pub emr: EMR,
    pub iRelative: i32,
}
impl ::core::marker::Copy for EMRRESTOREDC {}
impl ::core::clone::Clone for EMRRESTOREDC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRRESTOREDC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRRESTOREDC")
            .field("emr", &self.emr)
            .field("iRelative", &self.iRelative)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRRESTOREDC {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.iRelative == other.iRelative
    }
}
impl ::core::cmp::Eq for EMRRESTOREDC {}
impl FromIntoMemory for EMRRESTOREDC {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_iRelative = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            emr: f_emr,
            iRelative: f_iRelative,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.iRelative, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct EMRROUNDRECT {
    pub emr: EMR,
    pub rclBox: super::super::Foundation::RECTL,
    pub szlCorner: super::super::Foundation::SIZE,
}
impl ::core::marker::Copy for EMRROUNDRECT {}
impl ::core::clone::Clone for EMRROUNDRECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRROUNDRECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRROUNDRECT")
            .field("emr", &self.emr)
            .field("rclBox", &self.rclBox)
            .field("szlCorner", &self.szlCorner)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRROUNDRECT {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclBox == other.rclBox && self.szlCorner == other.szlCorner
    }
}
impl ::core::cmp::Eq for EMRROUNDRECT {}
impl FromIntoMemory for EMRROUNDRECT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_rclBox =
            <super::super::Foundation::RECTL as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        let f_szlCorner =
            <super::super::Foundation::SIZE as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        Self {
            emr: f_emr,
            rclBox: f_rclBox,
            szlCorner: f_szlCorner,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.rclBox, &mut into[8..8 + 16]);
        FromIntoMemory::into_bytes(self.szlCorner, &mut into[24..24 + 8]);
    }
    fn size() -> usize {
        32
    }
}
pub struct EMRSCALEVIEWPORTEXTEX {
    pub emr: EMR,
    pub xNum: i32,
    pub xDenom: i32,
    pub yNum: i32,
    pub yDenom: i32,
}
impl ::core::marker::Copy for EMRSCALEVIEWPORTEXTEX {}
impl ::core::clone::Clone for EMRSCALEVIEWPORTEXTEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRSCALEVIEWPORTEXTEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSCALEVIEWPORTEXTEX")
            .field("emr", &self.emr)
            .field("xNum", &self.xNum)
            .field("xDenom", &self.xDenom)
            .field("yNum", &self.yNum)
            .field("yDenom", &self.yDenom)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRSCALEVIEWPORTEXTEX {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr
            && self.xNum == other.xNum
            && self.xDenom == other.xDenom
            && self.yNum == other.yNum
            && self.yDenom == other.yDenom
    }
}
impl ::core::cmp::Eq for EMRSCALEVIEWPORTEXTEX {}
impl FromIntoMemory for EMRSCALEVIEWPORTEXTEX {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_xNum = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_xDenom = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_yNum = <i32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_yDenom = <i32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            emr: f_emr,
            xNum: f_xNum,
            xDenom: f_xDenom,
            yNum: f_yNum,
            yDenom: f_yDenom,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.xNum, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.xDenom, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.yNum, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.yDenom, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct EMRSELECTCLIPPATH {
    pub emr: EMR,
    pub iMode: u32,
}
impl ::core::marker::Copy for EMRSELECTCLIPPATH {}
impl ::core::clone::Clone for EMRSELECTCLIPPATH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRSELECTCLIPPATH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSELECTCLIPPATH")
            .field("emr", &self.emr)
            .field("iMode", &self.iMode)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRSELECTCLIPPATH {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.iMode == other.iMode
    }
}
impl ::core::cmp::Eq for EMRSELECTCLIPPATH {}
impl FromIntoMemory for EMRSELECTCLIPPATH {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_iMode = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            emr: f_emr,
            iMode: f_iMode,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.iMode, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct EMRSELECTOBJECT {
    pub emr: EMR,
    pub ihObject: u32,
}
impl ::core::marker::Copy for EMRSELECTOBJECT {}
impl ::core::clone::Clone for EMRSELECTOBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRSELECTOBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSELECTOBJECT")
            .field("emr", &self.emr)
            .field("ihObject", &self.ihObject)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRSELECTOBJECT {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ihObject == other.ihObject
    }
}
impl ::core::cmp::Eq for EMRSELECTOBJECT {}
impl FromIntoMemory for EMRSELECTOBJECT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_ihObject = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            emr: f_emr,
            ihObject: f_ihObject,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.ihObject, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct EMRSELECTPALETTE {
    pub emr: EMR,
    pub ihPal: u32,
}
impl ::core::marker::Copy for EMRSELECTPALETTE {}
impl ::core::clone::Clone for EMRSELECTPALETTE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRSELECTPALETTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSELECTPALETTE")
            .field("emr", &self.emr)
            .field("ihPal", &self.ihPal)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRSELECTPALETTE {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ihPal == other.ihPal
    }
}
impl ::core::cmp::Eq for EMRSELECTPALETTE {}
impl FromIntoMemory for EMRSELECTPALETTE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_ihPal = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            emr: f_emr,
            ihPal: f_ihPal,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.ihPal, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct EMRSETARCDIRECTION {
    pub emr: EMR,
    pub iArcDirection: u32,
}
impl ::core::marker::Copy for EMRSETARCDIRECTION {}
impl ::core::clone::Clone for EMRSETARCDIRECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRSETARCDIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETARCDIRECTION")
            .field("emr", &self.emr)
            .field("iArcDirection", &self.iArcDirection)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRSETARCDIRECTION {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.iArcDirection == other.iArcDirection
    }
}
impl ::core::cmp::Eq for EMRSETARCDIRECTION {}
impl FromIntoMemory for EMRSETARCDIRECTION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_iArcDirection = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            emr: f_emr,
            iArcDirection: f_iArcDirection,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.iArcDirection, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct EMRSETCOLORADJUSTMENT {
    pub emr: EMR,
    pub ColorAdjustment: COLORADJUSTMENT,
}
impl ::core::marker::Copy for EMRSETCOLORADJUSTMENT {}
impl ::core::clone::Clone for EMRSETCOLORADJUSTMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRSETCOLORADJUSTMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETCOLORADJUSTMENT")
            .field("emr", &self.emr)
            .field("ColorAdjustment", &self.ColorAdjustment)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRSETCOLORADJUSTMENT {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ColorAdjustment == other.ColorAdjustment
    }
}
impl ::core::cmp::Eq for EMRSETCOLORADJUSTMENT {}
impl FromIntoMemory for EMRSETCOLORADJUSTMENT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_ColorAdjustment = <COLORADJUSTMENT as FromIntoMemory>::from_bytes(&from[8..8 + 24]);
        Self {
            emr: f_emr,
            ColorAdjustment: f_ColorAdjustment,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.ColorAdjustment, &mut into[8..8 + 24]);
    }
    fn size() -> usize {
        32
    }
}
pub struct EMRSETCOLORSPACE {
    pub emr: EMR,
    pub ihCS: u32,
}
impl ::core::marker::Copy for EMRSETCOLORSPACE {}
impl ::core::clone::Clone for EMRSETCOLORSPACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRSETCOLORSPACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETCOLORSPACE")
            .field("emr", &self.emr)
            .field("ihCS", &self.ihCS)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRSETCOLORSPACE {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ihCS == other.ihCS
    }
}
impl ::core::cmp::Eq for EMRSETCOLORSPACE {}
impl FromIntoMemory for EMRSETCOLORSPACE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_ihCS = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            emr: f_emr,
            ihCS: f_ihCS,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.ihCS, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct EMRSETDIBITSTODEVICE {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub cxSrc: i32,
    pub cySrc: i32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub iUsageSrc: u32,
    pub iStartScan: u32,
    pub cScans: u32,
}
impl ::core::marker::Copy for EMRSETDIBITSTODEVICE {}
impl ::core::clone::Clone for EMRSETDIBITSTODEVICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRSETDIBITSTODEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETDIBITSTODEVICE")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .field("xDest", &self.xDest)
            .field("yDest", &self.yDest)
            .field("xSrc", &self.xSrc)
            .field("ySrc", &self.ySrc)
            .field("cxSrc", &self.cxSrc)
            .field("cySrc", &self.cySrc)
            .field("offBmiSrc", &self.offBmiSrc)
            .field("cbBmiSrc", &self.cbBmiSrc)
            .field("offBitsSrc", &self.offBitsSrc)
            .field("cbBitsSrc", &self.cbBitsSrc)
            .field("iUsageSrc", &self.iUsageSrc)
            .field("iStartScan", &self.iStartScan)
            .field("cScans", &self.cScans)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRSETDIBITSTODEVICE {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr
            && self.rclBounds == other.rclBounds
            && self.xDest == other.xDest
            && self.yDest == other.yDest
            && self.xSrc == other.xSrc
            && self.ySrc == other.ySrc
            && self.cxSrc == other.cxSrc
            && self.cySrc == other.cySrc
            && self.offBmiSrc == other.offBmiSrc
            && self.cbBmiSrc == other.cbBmiSrc
            && self.offBitsSrc == other.offBitsSrc
            && self.cbBitsSrc == other.cbBitsSrc
            && self.iUsageSrc == other.iUsageSrc
            && self.iStartScan == other.iStartScan
            && self.cScans == other.cScans
    }
}
impl ::core::cmp::Eq for EMRSETDIBITSTODEVICE {}
impl FromIntoMemory for EMRSETDIBITSTODEVICE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 76);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_rclBounds =
            <super::super::Foundation::RECTL as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        let f_xDest = <i32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_yDest = <i32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_xSrc = <i32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_ySrc = <i32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_cxSrc = <i32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_cySrc = <i32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_offBmiSrc = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_cbBmiSrc = <u32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_offBitsSrc = <u32 as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_cbBitsSrc = <u32 as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_iUsageSrc = <u32 as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        let f_iStartScan = <u32 as FromIntoMemory>::from_bytes(&from[68..68 + 4]);
        let f_cScans = <u32 as FromIntoMemory>::from_bytes(&from[72..72 + 4]);
        Self {
            emr: f_emr,
            rclBounds: f_rclBounds,
            xDest: f_xDest,
            yDest: f_yDest,
            xSrc: f_xSrc,
            ySrc: f_ySrc,
            cxSrc: f_cxSrc,
            cySrc: f_cySrc,
            offBmiSrc: f_offBmiSrc,
            cbBmiSrc: f_cbBmiSrc,
            offBitsSrc: f_offBitsSrc,
            cbBitsSrc: f_cbBitsSrc,
            iUsageSrc: f_iUsageSrc,
            iStartScan: f_iStartScan,
            cScans: f_cScans,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 76);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.rclBounds, &mut into[8..8 + 16]);
        FromIntoMemory::into_bytes(self.xDest, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.yDest, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.xSrc, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.ySrc, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.cxSrc, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.cySrc, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.offBmiSrc, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.cbBmiSrc, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.offBitsSrc, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.cbBitsSrc, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.iUsageSrc, &mut into[64..64 + 4]);
        FromIntoMemory::into_bytes(self.iStartScan, &mut into[68..68 + 4]);
        FromIntoMemory::into_bytes(self.cScans, &mut into[72..72 + 4]);
    }
    fn size() -> usize {
        76
    }
}
pub struct EMRSETICMPROFILE {
    pub emr: EMR,
    pub dwFlags: u32,
    pub cbName: u32,
    pub cbData: u32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for EMRSETICMPROFILE {}
impl ::core::clone::Clone for EMRSETICMPROFILE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRSETICMPROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETICMPROFILE")
            .field("emr", &self.emr)
            .field("dwFlags", &self.dwFlags)
            .field("cbName", &self.cbName)
            .field("cbData", &self.cbData)
            .field("Data", &self.Data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRSETICMPROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr
            && self.dwFlags == other.dwFlags
            && self.cbName == other.cbName
            && self.cbData == other.cbData
            && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for EMRSETICMPROFILE {}
impl FromIntoMemory for EMRSETICMPROFILE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_cbName = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_cbData = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_Data = <[u8; 1] as FromIntoMemory>::from_bytes(&from[20..20 + 1]);
        Self {
            emr: f_emr,
            dwFlags: f_dwFlags,
            cbName: f_cbName,
            cbData: f_cbData,
            Data: f_Data,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.cbName, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.cbData, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.Data, &mut into[20..20 + 1]);
    }
    fn size() -> usize {
        24
    }
}
pub struct EMRSETMAPPERFLAGS {
    pub emr: EMR,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for EMRSETMAPPERFLAGS {}
impl ::core::clone::Clone for EMRSETMAPPERFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRSETMAPPERFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETMAPPERFLAGS")
            .field("emr", &self.emr)
            .field("dwFlags", &self.dwFlags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRSETMAPPERFLAGS {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for EMRSETMAPPERFLAGS {}
impl FromIntoMemory for EMRSETMAPPERFLAGS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            emr: f_emr,
            dwFlags: f_dwFlags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct EMRSETPALETTEENTRIES {
    pub emr: EMR,
    pub ihPal: u32,
    pub iStart: u32,
    pub cEntries: u32,
    pub aPalEntries: [PALETTEENTRY; 1],
}
impl ::core::marker::Copy for EMRSETPALETTEENTRIES {}
impl ::core::clone::Clone for EMRSETPALETTEENTRIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRSETPALETTEENTRIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETPALETTEENTRIES")
            .field("emr", &self.emr)
            .field("ihPal", &self.ihPal)
            .field("iStart", &self.iStart)
            .field("cEntries", &self.cEntries)
            .field("aPalEntries", &self.aPalEntries)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRSETPALETTEENTRIES {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr
            && self.ihPal == other.ihPal
            && self.iStart == other.iStart
            && self.cEntries == other.cEntries
            && self.aPalEntries == other.aPalEntries
    }
}
impl ::core::cmp::Eq for EMRSETPALETTEENTRIES {}
impl FromIntoMemory for EMRSETPALETTEENTRIES {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_ihPal = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_iStart = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_cEntries = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_aPalEntries = <[PALETTEENTRY; 1] as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            emr: f_emr,
            ihPal: f_ihPal,
            iStart: f_iStart,
            cEntries: f_cEntries,
            aPalEntries: f_aPalEntries,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.ihPal, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.iStart, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.cEntries, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.aPalEntries, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct EMRSETPIXELV {
    pub emr: EMR,
    pub ptlPixel: super::super::Foundation::POINTL,
    pub crColor: u32,
}
impl ::core::marker::Copy for EMRSETPIXELV {}
impl ::core::clone::Clone for EMRSETPIXELV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRSETPIXELV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETPIXELV")
            .field("emr", &self.emr)
            .field("ptlPixel", &self.ptlPixel)
            .field("crColor", &self.crColor)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRSETPIXELV {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ptlPixel == other.ptlPixel && self.crColor == other.crColor
    }
}
impl ::core::cmp::Eq for EMRSETPIXELV {}
impl FromIntoMemory for EMRSETPIXELV {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_ptlPixel =
            <super::super::Foundation::POINTL as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_crColor = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            emr: f_emr,
            ptlPixel: f_ptlPixel,
            crColor: f_crColor,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.ptlPixel, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.crColor, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub struct EMRSETTEXTCOLOR {
    pub emr: EMR,
    pub crColor: u32,
}
impl ::core::marker::Copy for EMRSETTEXTCOLOR {}
impl ::core::clone::Clone for EMRSETTEXTCOLOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRSETTEXTCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETTEXTCOLOR")
            .field("emr", &self.emr)
            .field("crColor", &self.crColor)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRSETTEXTCOLOR {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.crColor == other.crColor
    }
}
impl ::core::cmp::Eq for EMRSETTEXTCOLOR {}
impl FromIntoMemory for EMRSETTEXTCOLOR {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_crColor = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            emr: f_emr,
            crColor: f_crColor,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.crColor, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct EMRSETVIEWPORTEXTEX {
    pub emr: EMR,
    pub szlExtent: super::super::Foundation::SIZE,
}
impl ::core::marker::Copy for EMRSETVIEWPORTEXTEX {}
impl ::core::clone::Clone for EMRSETVIEWPORTEXTEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRSETVIEWPORTEXTEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETVIEWPORTEXTEX")
            .field("emr", &self.emr)
            .field("szlExtent", &self.szlExtent)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRSETVIEWPORTEXTEX {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.szlExtent == other.szlExtent
    }
}
impl ::core::cmp::Eq for EMRSETVIEWPORTEXTEX {}
impl FromIntoMemory for EMRSETVIEWPORTEXTEX {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_szlExtent =
            <super::super::Foundation::SIZE as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        Self {
            emr: f_emr,
            szlExtent: f_szlExtent,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.szlExtent, &mut into[8..8 + 8]);
    }
    fn size() -> usize {
        16
    }
}
pub struct EMRSETVIEWPORTORGEX {
    pub emr: EMR,
    pub ptlOrigin: super::super::Foundation::POINTL,
}
impl ::core::marker::Copy for EMRSETVIEWPORTORGEX {}
impl ::core::clone::Clone for EMRSETVIEWPORTORGEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRSETVIEWPORTORGEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETVIEWPORTORGEX")
            .field("emr", &self.emr)
            .field("ptlOrigin", &self.ptlOrigin)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRSETVIEWPORTORGEX {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ptlOrigin == other.ptlOrigin
    }
}
impl ::core::cmp::Eq for EMRSETVIEWPORTORGEX {}
impl FromIntoMemory for EMRSETVIEWPORTORGEX {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_ptlOrigin =
            <super::super::Foundation::POINTL as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        Self {
            emr: f_emr,
            ptlOrigin: f_ptlOrigin,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.ptlOrigin, &mut into[8..8 + 8]);
    }
    fn size() -> usize {
        16
    }
}
pub struct EMRSTRETCHDIBITS {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub cxSrc: i32,
    pub cySrc: i32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub iUsageSrc: u32,
    pub dwRop: u32,
    pub cxDest: i32,
    pub cyDest: i32,
}
impl ::core::marker::Copy for EMRSTRETCHDIBITS {}
impl ::core::clone::Clone for EMRSTRETCHDIBITS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRSTRETCHDIBITS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSTRETCHDIBITS")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .field("xDest", &self.xDest)
            .field("yDest", &self.yDest)
            .field("xSrc", &self.xSrc)
            .field("ySrc", &self.ySrc)
            .field("cxSrc", &self.cxSrc)
            .field("cySrc", &self.cySrc)
            .field("offBmiSrc", &self.offBmiSrc)
            .field("cbBmiSrc", &self.cbBmiSrc)
            .field("offBitsSrc", &self.offBitsSrc)
            .field("cbBitsSrc", &self.cbBitsSrc)
            .field("iUsageSrc", &self.iUsageSrc)
            .field("dwRop", &self.dwRop)
            .field("cxDest", &self.cxDest)
            .field("cyDest", &self.cyDest)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRSTRETCHDIBITS {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr
            && self.rclBounds == other.rclBounds
            && self.xDest == other.xDest
            && self.yDest == other.yDest
            && self.xSrc == other.xSrc
            && self.ySrc == other.ySrc
            && self.cxSrc == other.cxSrc
            && self.cySrc == other.cySrc
            && self.offBmiSrc == other.offBmiSrc
            && self.cbBmiSrc == other.cbBmiSrc
            && self.offBitsSrc == other.offBitsSrc
            && self.cbBitsSrc == other.cbBitsSrc
            && self.iUsageSrc == other.iUsageSrc
            && self.dwRop == other.dwRop
            && self.cxDest == other.cxDest
            && self.cyDest == other.cyDest
    }
}
impl ::core::cmp::Eq for EMRSTRETCHDIBITS {}
impl FromIntoMemory for EMRSTRETCHDIBITS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 80);
        let f_emr = <EMR as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_rclBounds =
            <super::super::Foundation::RECTL as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        let f_xDest = <i32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_yDest = <i32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_xSrc = <i32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_ySrc = <i32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_cxSrc = <i32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_cySrc = <i32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_offBmiSrc = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_cbBmiSrc = <u32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_offBitsSrc = <u32 as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_cbBitsSrc = <u32 as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_iUsageSrc = <u32 as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        let f_dwRop = <u32 as FromIntoMemory>::from_bytes(&from[68..68 + 4]);
        let f_cxDest = <i32 as FromIntoMemory>::from_bytes(&from[72..72 + 4]);
        let f_cyDest = <i32 as FromIntoMemory>::from_bytes(&from[76..76 + 4]);
        Self {
            emr: f_emr,
            rclBounds: f_rclBounds,
            xDest: f_xDest,
            yDest: f_yDest,
            xSrc: f_xSrc,
            ySrc: f_ySrc,
            cxSrc: f_cxSrc,
            cySrc: f_cySrc,
            offBmiSrc: f_offBmiSrc,
            cbBmiSrc: f_cbBmiSrc,
            offBitsSrc: f_offBitsSrc,
            cbBitsSrc: f_cbBitsSrc,
            iUsageSrc: f_iUsageSrc,
            dwRop: f_dwRop,
            cxDest: f_cxDest,
            cyDest: f_cyDest,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 80);
        FromIntoMemory::into_bytes(self.emr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.rclBounds, &mut into[8..8 + 16]);
        FromIntoMemory::into_bytes(self.xDest, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.yDest, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.xSrc, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.ySrc, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.cxSrc, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.cySrc, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.offBmiSrc, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.cbBmiSrc, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.offBitsSrc, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.cbBitsSrc, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.iUsageSrc, &mut into[64..64 + 4]);
        FromIntoMemory::into_bytes(self.dwRop, &mut into[68..68 + 4]);
        FromIntoMemory::into_bytes(self.cxDest, &mut into[72..72 + 4]);
        FromIntoMemory::into_bytes(self.cyDest, &mut into[76..76 + 4]);
    }
    fn size() -> usize {
        80
    }
}
pub struct EMRTEXT {
    pub ptlReference: super::super::Foundation::POINTL,
    pub nChars: u32,
    pub offString: u32,
    pub fOptions: u32,
    pub rcl: super::super::Foundation::RECTL,
    pub offDx: u32,
}
impl ::core::marker::Copy for EMRTEXT {}
impl ::core::clone::Clone for EMRTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EMRTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRTEXT")
            .field("ptlReference", &self.ptlReference)
            .field("nChars", &self.nChars)
            .field("offString", &self.offString)
            .field("fOptions", &self.fOptions)
            .field("rcl", &self.rcl)
            .field("offDx", &self.offDx)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EMRTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.ptlReference == other.ptlReference
            && self.nChars == other.nChars
            && self.offString == other.offString
            && self.fOptions == other.fOptions
            && self.rcl == other.rcl
            && self.offDx == other.offDx
    }
}
impl ::core::cmp::Eq for EMRTEXT {}
impl FromIntoMemory for EMRTEXT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_ptlReference =
            <super::super::Foundation::POINTL as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_nChars = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_offString = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_fOptions = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_rcl =
            <super::super::Foundation::RECTL as FromIntoMemory>::from_bytes(&from[20..20 + 16]);
        let f_offDx = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        Self {
            ptlReference: f_ptlReference,
            nChars: f_nChars,
            offString: f_offString,
            fOptions: f_fOptions,
            rcl: f_rcl,
            offDx: f_offDx,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.ptlReference, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.nChars, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.offString, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.fOptions, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.rcl, &mut into[20..20 + 16]);
        FromIntoMemory::into_bytes(self.offDx, &mut into[36..36 + 4]);
    }
    fn size() -> usize {
        40
    }
}
pub const EMR_ABORTPATH: u32 = 68u32;
pub const EMR_ALPHABLEND: u32 = 114u32;
pub const EMR_ANGLEARC: u32 = 41u32;
pub const EMR_ARC: u32 = 45u32;
pub const EMR_ARCTO: u32 = 55u32;
pub const EMR_BEGINPATH: u32 = 59u32;
pub const EMR_BITBLT: u32 = 76u32;
pub const EMR_CHORD: u32 = 46u32;
pub const EMR_CLOSEFIGURE: u32 = 61u32;
pub const EMR_COLORCORRECTPALETTE: u32 = 111u32;
pub const EMR_COLORMATCHTOTARGETW: u32 = 121u32;
pub const EMR_CREATEBRUSHINDIRECT: u32 = 39u32;
pub const EMR_CREATECOLORSPACE: u32 = 99u32;
pub const EMR_CREATECOLORSPACEW: u32 = 122u32;
pub const EMR_CREATEDIBPATTERNBRUSHPT: u32 = 94u32;
pub const EMR_CREATEMONOBRUSH: u32 = 93u32;
pub const EMR_CREATEPALETTE: u32 = 49u32;
pub const EMR_CREATEPEN: u32 = 38u32;
pub const EMR_DELETECOLORSPACE: u32 = 101u32;
pub const EMR_DELETEOBJECT: u32 = 40u32;
pub const EMR_ELLIPSE: u32 = 42u32;
pub const EMR_ENDPATH: u32 = 60u32;
pub const EMR_EOF: u32 = 14u32;
pub const EMR_EXCLUDECLIPRECT: u32 = 29u32;
pub const EMR_EXTCREATEFONTINDIRECTW: u32 = 82u32;
pub const EMR_EXTCREATEPEN: u32 = 95u32;
pub const EMR_EXTFLOODFILL: u32 = 53u32;
pub const EMR_EXTSELECTCLIPRGN: u32 = 75u32;
pub const EMR_EXTTEXTOUTA: u32 = 83u32;
pub const EMR_EXTTEXTOUTW: u32 = 84u32;
pub const EMR_FILLPATH: u32 = 62u32;
pub const EMR_FILLRGN: u32 = 71u32;
pub const EMR_FLATTENPATH: u32 = 65u32;
pub const EMR_FRAMERGN: u32 = 72u32;
pub const EMR_GDICOMMENT: u32 = 70u32;
pub const EMR_GLSBOUNDEDRECORD: u32 = 103u32;
pub const EMR_GLSRECORD: u32 = 102u32;
pub const EMR_GRADIENTFILL: u32 = 118u32;
pub const EMR_HEADER: u32 = 1u32;
pub const EMR_INTERSECTCLIPRECT: u32 = 30u32;
pub const EMR_INVERTRGN: u32 = 73u32;
pub const EMR_LINETO: u32 = 54u32;
pub const EMR_MASKBLT: u32 = 78u32;
pub const EMR_MAX: u32 = 122u32;
pub const EMR_MIN: u32 = 1u32;
pub const EMR_MODIFYWORLDTRANSFORM: u32 = 36u32;
pub const EMR_MOVETOEX: u32 = 27u32;
pub const EMR_OFFSETCLIPRGN: u32 = 26u32;
pub const EMR_PAINTRGN: u32 = 74u32;
pub const EMR_PIE: u32 = 47u32;
pub const EMR_PIXELFORMAT: u32 = 104u32;
pub const EMR_PLGBLT: u32 = 79u32;
pub const EMR_POLYBEZIER: u32 = 2u32;
pub const EMR_POLYBEZIER16: u32 = 85u32;
pub const EMR_POLYBEZIERTO: u32 = 5u32;
pub const EMR_POLYBEZIERTO16: u32 = 88u32;
pub const EMR_POLYDRAW: u32 = 56u32;
pub const EMR_POLYDRAW16: u32 = 92u32;
pub const EMR_POLYGON: u32 = 3u32;
pub const EMR_POLYGON16: u32 = 86u32;
pub const EMR_POLYLINE: u32 = 4u32;
pub const EMR_POLYLINE16: u32 = 87u32;
pub const EMR_POLYLINETO: u32 = 6u32;
pub const EMR_POLYLINETO16: u32 = 89u32;
pub const EMR_POLYPOLYGON: u32 = 8u32;
pub const EMR_POLYPOLYGON16: u32 = 91u32;
pub const EMR_POLYPOLYLINE: u32 = 7u32;
pub const EMR_POLYPOLYLINE16: u32 = 90u32;
pub const EMR_POLYTEXTOUTA: u32 = 96u32;
pub const EMR_POLYTEXTOUTW: u32 = 97u32;
pub const EMR_REALIZEPALETTE: u32 = 52u32;
pub const EMR_RECTANGLE: u32 = 43u32;
pub const EMR_RESERVED_105: u32 = 105u32;
pub const EMR_RESERVED_106: u32 = 106u32;
pub const EMR_RESERVED_107: u32 = 107u32;
pub const EMR_RESERVED_108: u32 = 108u32;
pub const EMR_RESERVED_109: u32 = 109u32;
pub const EMR_RESERVED_110: u32 = 110u32;
pub const EMR_RESERVED_117: u32 = 117u32;
pub const EMR_RESERVED_119: u32 = 119u32;
pub const EMR_RESERVED_120: u32 = 120u32;
pub const EMR_RESIZEPALETTE: u32 = 51u32;
pub const EMR_RESTOREDC: u32 = 34u32;
pub const EMR_ROUNDRECT: u32 = 44u32;
pub const EMR_SAVEDC: u32 = 33u32;
pub const EMR_SCALEVIEWPORTEXTEX: u32 = 31u32;
pub const EMR_SCALEWINDOWEXTEX: u32 = 32u32;
pub const EMR_SELECTCLIPPATH: u32 = 67u32;
pub const EMR_SELECTOBJECT: u32 = 37u32;
pub const EMR_SELECTPALETTE: u32 = 48u32;
pub const EMR_SETARCDIRECTION: u32 = 57u32;
pub const EMR_SETBKCOLOR: u32 = 25u32;
pub const EMR_SETBKMODE: u32 = 18u32;
pub const EMR_SETBRUSHORGEX: u32 = 13u32;
pub const EMR_SETCOLORADJUSTMENT: u32 = 23u32;
pub const EMR_SETCOLORSPACE: u32 = 100u32;
pub const EMR_SETDIBITSTODEVICE: u32 = 80u32;
pub const EMR_SETICMMODE: u32 = 98u32;
pub const EMR_SETICMPROFILEA: u32 = 112u32;
pub const EMR_SETICMPROFILEW: u32 = 113u32;
pub const EMR_SETLAYOUT: u32 = 115u32;
pub const EMR_SETMAPMODE: u32 = 17u32;
pub const EMR_SETMAPPERFLAGS: u32 = 16u32;
pub const EMR_SETMETARGN: u32 = 28u32;
pub const EMR_SETMITERLIMIT: u32 = 58u32;
pub const EMR_SETPALETTEENTRIES: u32 = 50u32;
pub const EMR_SETPIXELV: u32 = 15u32;
pub const EMR_SETPOLYFILLMODE: u32 = 19u32;
pub const EMR_SETROP2: u32 = 20u32;
pub const EMR_SETSTRETCHBLTMODE: u32 = 21u32;
pub const EMR_SETTEXTALIGN: u32 = 22u32;
pub const EMR_SETTEXTCOLOR: u32 = 24u32;
pub const EMR_SETVIEWPORTEXTEX: u32 = 11u32;
pub const EMR_SETVIEWPORTORGEX: u32 = 12u32;
pub const EMR_SETWINDOWEXTEX: u32 = 9u32;
pub const EMR_SETWINDOWORGEX: u32 = 10u32;
pub const EMR_SETWORLDTRANSFORM: u32 = 35u32;
pub const EMR_STRETCHBLT: u32 = 77u32;
pub const EMR_STRETCHDIBITS: u32 = 81u32;
pub const EMR_STROKEANDFILLPATH: u32 = 63u32;
pub const EMR_STROKEPATH: u32 = 64u32;
pub const EMR_TRANSPARENTBLT: u32 = 116u32;
pub const EMR_WIDENPATH: u32 = 66u32;
pub const ENABLEDUPLEX: u32 = 28u32;
pub const ENABLEPAIRKERNING: u32 = 769u32;
pub const ENABLERELATIVEWIDTHS: u32 = 768u32;
pub const ENCAPSULATED_POSTSCRIPT: u32 = 4116u32;
pub const ENDDOC: u32 = 11u32;
pub const END_PATH: u32 = 4098u32;
pub struct ENHMETAHEADER {
    pub iType: u32,
    pub nSize: u32,
    pub rclBounds: super::super::Foundation::RECTL,
    pub rclFrame: super::super::Foundation::RECTL,
    pub dSignature: u32,
    pub nVersion: u32,
    pub nBytes: u32,
    pub nRecords: u32,
    pub nHandles: u16,
    pub sReserved: u16,
    pub nDescription: u32,
    pub offDescription: u32,
    pub nPalEntries: u32,
    pub szlDevice: super::super::Foundation::SIZE,
    pub szlMillimeters: super::super::Foundation::SIZE,
    pub cbPixelFormat: u32,
    pub offPixelFormat: u32,
    pub bOpenGL: u32,
    pub szlMicrometers: super::super::Foundation::SIZE,
}
impl ::core::marker::Copy for ENHMETAHEADER {}
impl ::core::clone::Clone for ENHMETAHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENHMETAHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENHMETAHEADER")
            .field("iType", &self.iType)
            .field("nSize", &self.nSize)
            .field("rclBounds", &self.rclBounds)
            .field("rclFrame", &self.rclFrame)
            .field("dSignature", &self.dSignature)
            .field("nVersion", &self.nVersion)
            .field("nBytes", &self.nBytes)
            .field("nRecords", &self.nRecords)
            .field("nHandles", &self.nHandles)
            .field("sReserved", &self.sReserved)
            .field("nDescription", &self.nDescription)
            .field("offDescription", &self.offDescription)
            .field("nPalEntries", &self.nPalEntries)
            .field("szlDevice", &self.szlDevice)
            .field("szlMillimeters", &self.szlMillimeters)
            .field("cbPixelFormat", &self.cbPixelFormat)
            .field("offPixelFormat", &self.offPixelFormat)
            .field("bOpenGL", &self.bOpenGL)
            .field("szlMicrometers", &self.szlMicrometers)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ENHMETAHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.iType == other.iType
            && self.nSize == other.nSize
            && self.rclBounds == other.rclBounds
            && self.rclFrame == other.rclFrame
            && self.dSignature == other.dSignature
            && self.nVersion == other.nVersion
            && self.nBytes == other.nBytes
            && self.nRecords == other.nRecords
            && self.nHandles == other.nHandles
            && self.sReserved == other.sReserved
            && self.nDescription == other.nDescription
            && self.offDescription == other.offDescription
            && self.nPalEntries == other.nPalEntries
            && self.szlDevice == other.szlDevice
            && self.szlMillimeters == other.szlMillimeters
            && self.cbPixelFormat == other.cbPixelFormat
            && self.offPixelFormat == other.offPixelFormat
            && self.bOpenGL == other.bOpenGL
            && self.szlMicrometers == other.szlMicrometers
    }
}
impl ::core::cmp::Eq for ENHMETAHEADER {}
impl FromIntoMemory for ENHMETAHEADER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 108);
        let f_iType = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_nSize = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_rclBounds =
            <super::super::Foundation::RECTL as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        let f_rclFrame =
            <super::super::Foundation::RECTL as FromIntoMemory>::from_bytes(&from[24..24 + 16]);
        let f_dSignature = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_nVersion = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_nBytes = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_nRecords = <u32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_nHandles = <u16 as FromIntoMemory>::from_bytes(&from[56..56 + 2]);
        let f_sReserved = <u16 as FromIntoMemory>::from_bytes(&from[58..58 + 2]);
        let f_nDescription = <u32 as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_offDescription = <u32 as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        let f_nPalEntries = <u32 as FromIntoMemory>::from_bytes(&from[68..68 + 4]);
        let f_szlDevice =
            <super::super::Foundation::SIZE as FromIntoMemory>::from_bytes(&from[72..72 + 8]);
        let f_szlMillimeters =
            <super::super::Foundation::SIZE as FromIntoMemory>::from_bytes(&from[80..80 + 8]);
        let f_cbPixelFormat = <u32 as FromIntoMemory>::from_bytes(&from[88..88 + 4]);
        let f_offPixelFormat = <u32 as FromIntoMemory>::from_bytes(&from[92..92 + 4]);
        let f_bOpenGL = <u32 as FromIntoMemory>::from_bytes(&from[96..96 + 4]);
        let f_szlMicrometers =
            <super::super::Foundation::SIZE as FromIntoMemory>::from_bytes(&from[100..100 + 8]);
        Self {
            iType: f_iType,
            nSize: f_nSize,
            rclBounds: f_rclBounds,
            rclFrame: f_rclFrame,
            dSignature: f_dSignature,
            nVersion: f_nVersion,
            nBytes: f_nBytes,
            nRecords: f_nRecords,
            nHandles: f_nHandles,
            sReserved: f_sReserved,
            nDescription: f_nDescription,
            offDescription: f_offDescription,
            nPalEntries: f_nPalEntries,
            szlDevice: f_szlDevice,
            szlMillimeters: f_szlMillimeters,
            cbPixelFormat: f_cbPixelFormat,
            offPixelFormat: f_offPixelFormat,
            bOpenGL: f_bOpenGL,
            szlMicrometers: f_szlMicrometers,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 108);
        FromIntoMemory::into_bytes(self.iType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.nSize, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.rclBounds, &mut into[8..8 + 16]);
        FromIntoMemory::into_bytes(self.rclFrame, &mut into[24..24 + 16]);
        FromIntoMemory::into_bytes(self.dSignature, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.nVersion, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.nBytes, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.nRecords, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.nHandles, &mut into[56..56 + 2]);
        FromIntoMemory::into_bytes(self.sReserved, &mut into[58..58 + 2]);
        FromIntoMemory::into_bytes(self.nDescription, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.offDescription, &mut into[64..64 + 4]);
        FromIntoMemory::into_bytes(self.nPalEntries, &mut into[68..68 + 4]);
        FromIntoMemory::into_bytes(self.szlDevice, &mut into[72..72 + 8]);
        FromIntoMemory::into_bytes(self.szlMillimeters, &mut into[80..80 + 8]);
        FromIntoMemory::into_bytes(self.cbPixelFormat, &mut into[88..88 + 4]);
        FromIntoMemory::into_bytes(self.offPixelFormat, &mut into[92..92 + 4]);
        FromIntoMemory::into_bytes(self.bOpenGL, &mut into[96..96 + 4]);
        FromIntoMemory::into_bytes(self.szlMicrometers, &mut into[100..100 + 8]);
    }
    fn size() -> usize {
        108
    }
}
pub struct ENHMETARECORD {
    pub iType: u32,
    pub nSize: u32,
    pub dParm: [u32; 1],
}
impl ::core::marker::Copy for ENHMETARECORD {}
impl ::core::clone::Clone for ENHMETARECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENHMETARECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENHMETARECORD")
            .field("iType", &self.iType)
            .field("nSize", &self.nSize)
            .field("dParm", &self.dParm)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ENHMETARECORD {
    fn eq(&self, other: &Self) -> bool {
        self.iType == other.iType && self.nSize == other.nSize && self.dParm == other.dParm
    }
}
impl ::core::cmp::Eq for ENHMETARECORD {}
impl FromIntoMemory for ENHMETARECORD {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_iType = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_nSize = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dParm = <[u32; 1] as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            iType: f_iType,
            nSize: f_nSize,
            dParm: f_dParm,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.iType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.nSize, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dParm, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub const ENHMETA_SIGNATURE: u32 = 1179469088u32;
pub const ENHMETA_STOCK_OBJECT: u32 = 2147483648u32;
pub type ENHMFENUMPROC = StdCallFnPtr<
    (
        HDC,
        ConstPtr<HANDLETABLE>,
        ConstPtr<ENHMETARECORD>,
        i32,
        super::super::Foundation::LPARAM,
    ),
    i32,
>;
pub struct ENUMLOGFONTA {
    pub elfLogFont: LOGFONTA,
    pub elfFullName: [u8; 64],
    pub elfStyle: [u8; 32],
}
impl ::core::marker::Copy for ENUMLOGFONTA {}
impl ::core::clone::Clone for ENUMLOGFONTA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENUMLOGFONTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMLOGFONTA")
            .field("elfLogFont", &self.elfLogFont)
            .field("elfFullName", &self.elfFullName)
            .field("elfStyle", &self.elfStyle)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ENUMLOGFONTA {
    fn eq(&self, other: &Self) -> bool {
        self.elfLogFont == other.elfLogFont
            && self.elfFullName == other.elfFullName
            && self.elfStyle == other.elfStyle
    }
}
impl ::core::cmp::Eq for ENUMLOGFONTA {}
impl FromIntoMemory for ENUMLOGFONTA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 156);
        let f_elfLogFont = <LOGFONTA as FromIntoMemory>::from_bytes(&from[0..0 + 60]);
        let f_elfFullName = <[u8; 64] as FromIntoMemory>::from_bytes(&from[60..60 + 64]);
        let f_elfStyle = <[u8; 32] as FromIntoMemory>::from_bytes(&from[124..124 + 32]);
        Self {
            elfLogFont: f_elfLogFont,
            elfFullName: f_elfFullName,
            elfStyle: f_elfStyle,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 156);
        FromIntoMemory::into_bytes(self.elfLogFont, &mut into[0..0 + 60]);
        FromIntoMemory::into_bytes(self.elfFullName, &mut into[60..60 + 64]);
        FromIntoMemory::into_bytes(self.elfStyle, &mut into[124..124 + 32]);
    }
    fn size() -> usize {
        156
    }
}
pub struct ENUMLOGFONTEXA {
    pub elfLogFont: LOGFONTA,
    pub elfFullName: [u8; 64],
    pub elfStyle: [u8; 32],
    pub elfScript: [u8; 32],
}
impl ::core::marker::Copy for ENUMLOGFONTEXA {}
impl ::core::clone::Clone for ENUMLOGFONTEXA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENUMLOGFONTEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMLOGFONTEXA")
            .field("elfLogFont", &self.elfLogFont)
            .field("elfFullName", &self.elfFullName)
            .field("elfStyle", &self.elfStyle)
            .field("elfScript", &self.elfScript)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ENUMLOGFONTEXA {
    fn eq(&self, other: &Self) -> bool {
        self.elfLogFont == other.elfLogFont
            && self.elfFullName == other.elfFullName
            && self.elfStyle == other.elfStyle
            && self.elfScript == other.elfScript
    }
}
impl ::core::cmp::Eq for ENUMLOGFONTEXA {}
impl FromIntoMemory for ENUMLOGFONTEXA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 188);
        let f_elfLogFont = <LOGFONTA as FromIntoMemory>::from_bytes(&from[0..0 + 60]);
        let f_elfFullName = <[u8; 64] as FromIntoMemory>::from_bytes(&from[60..60 + 64]);
        let f_elfStyle = <[u8; 32] as FromIntoMemory>::from_bytes(&from[124..124 + 32]);
        let f_elfScript = <[u8; 32] as FromIntoMemory>::from_bytes(&from[156..156 + 32]);
        Self {
            elfLogFont: f_elfLogFont,
            elfFullName: f_elfFullName,
            elfStyle: f_elfStyle,
            elfScript: f_elfScript,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 188);
        FromIntoMemory::into_bytes(self.elfLogFont, &mut into[0..0 + 60]);
        FromIntoMemory::into_bytes(self.elfFullName, &mut into[60..60 + 64]);
        FromIntoMemory::into_bytes(self.elfStyle, &mut into[124..124 + 32]);
        FromIntoMemory::into_bytes(self.elfScript, &mut into[156..156 + 32]);
    }
    fn size() -> usize {
        188
    }
}
pub struct ENUMLOGFONTEXDVA {
    pub elfEnumLogfontEx: ENUMLOGFONTEXA,
    pub elfDesignVector: DESIGNVECTOR,
}
impl ::core::marker::Copy for ENUMLOGFONTEXDVA {}
impl ::core::clone::Clone for ENUMLOGFONTEXDVA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENUMLOGFONTEXDVA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMLOGFONTEXDVA")
            .field("elfEnumLogfontEx", &self.elfEnumLogfontEx)
            .field("elfDesignVector", &self.elfDesignVector)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ENUMLOGFONTEXDVA {
    fn eq(&self, other: &Self) -> bool {
        self.elfEnumLogfontEx == other.elfEnumLogfontEx
            && self.elfDesignVector == other.elfDesignVector
    }
}
impl ::core::cmp::Eq for ENUMLOGFONTEXDVA {}
impl FromIntoMemory for ENUMLOGFONTEXDVA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 260);
        let f_elfEnumLogfontEx = <ENUMLOGFONTEXA as FromIntoMemory>::from_bytes(&from[0..0 + 188]);
        let f_elfDesignVector = <DESIGNVECTOR as FromIntoMemory>::from_bytes(&from[188..188 + 72]);
        Self {
            elfEnumLogfontEx: f_elfEnumLogfontEx,
            elfDesignVector: f_elfDesignVector,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 260);
        FromIntoMemory::into_bytes(self.elfEnumLogfontEx, &mut into[0..0 + 188]);
        FromIntoMemory::into_bytes(self.elfDesignVector, &mut into[188..188 + 72]);
    }
    fn size() -> usize {
        260
    }
}
pub struct ENUMLOGFONTEXDVW {
    pub elfEnumLogfontEx: ENUMLOGFONTEXW,
    pub elfDesignVector: DESIGNVECTOR,
}
impl ::core::marker::Copy for ENUMLOGFONTEXDVW {}
impl ::core::clone::Clone for ENUMLOGFONTEXDVW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENUMLOGFONTEXDVW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMLOGFONTEXDVW")
            .field("elfEnumLogfontEx", &self.elfEnumLogfontEx)
            .field("elfDesignVector", &self.elfDesignVector)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ENUMLOGFONTEXDVW {
    fn eq(&self, other: &Self) -> bool {
        self.elfEnumLogfontEx == other.elfEnumLogfontEx
            && self.elfDesignVector == other.elfDesignVector
    }
}
impl ::core::cmp::Eq for ENUMLOGFONTEXDVW {}
impl FromIntoMemory for ENUMLOGFONTEXDVW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 260);
        let f_elfEnumLogfontEx = <ENUMLOGFONTEXW as FromIntoMemory>::from_bytes(&from[0..0 + 188]);
        let f_elfDesignVector = <DESIGNVECTOR as FromIntoMemory>::from_bytes(&from[188..188 + 72]);
        Self {
            elfEnumLogfontEx: f_elfEnumLogfontEx,
            elfDesignVector: f_elfDesignVector,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 260);
        FromIntoMemory::into_bytes(self.elfEnumLogfontEx, &mut into[0..0 + 188]);
        FromIntoMemory::into_bytes(self.elfDesignVector, &mut into[188..188 + 72]);
    }
    fn size() -> usize {
        260
    }
}
pub struct ENUMLOGFONTEXW {
    pub elfLogFont: LOGFONTW,
    pub elfFullName: [u16; 64],
    pub elfStyle: [u16; 32],
    pub elfScript: [u16; 32],
}
impl ::core::marker::Copy for ENUMLOGFONTEXW {}
impl ::core::clone::Clone for ENUMLOGFONTEXW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENUMLOGFONTEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMLOGFONTEXW")
            .field("elfLogFont", &self.elfLogFont)
            .field("elfFullName", &self.elfFullName)
            .field("elfStyle", &self.elfStyle)
            .field("elfScript", &self.elfScript)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ENUMLOGFONTEXW {
    fn eq(&self, other: &Self) -> bool {
        self.elfLogFont == other.elfLogFont
            && self.elfFullName == other.elfFullName
            && self.elfStyle == other.elfStyle
            && self.elfScript == other.elfScript
    }
}
impl ::core::cmp::Eq for ENUMLOGFONTEXW {}
impl FromIntoMemory for ENUMLOGFONTEXW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 188);
        let f_elfLogFont = <LOGFONTW as FromIntoMemory>::from_bytes(&from[0..0 + 60]);
        let f_elfFullName = <[u16; 64] as FromIntoMemory>::from_bytes(&from[60..60 + 64]);
        let f_elfStyle = <[u16; 32] as FromIntoMemory>::from_bytes(&from[124..124 + 32]);
        let f_elfScript = <[u16; 32] as FromIntoMemory>::from_bytes(&from[156..156 + 32]);
        Self {
            elfLogFont: f_elfLogFont,
            elfFullName: f_elfFullName,
            elfStyle: f_elfStyle,
            elfScript: f_elfScript,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 188);
        FromIntoMemory::into_bytes(self.elfLogFont, &mut into[0..0 + 60]);
        FromIntoMemory::into_bytes(self.elfFullName, &mut into[60..60 + 64]);
        FromIntoMemory::into_bytes(self.elfStyle, &mut into[124..124 + 32]);
        FromIntoMemory::into_bytes(self.elfScript, &mut into[156..156 + 32]);
    }
    fn size() -> usize {
        188
    }
}
pub struct ENUMLOGFONTW {
    pub elfLogFont: LOGFONTW,
    pub elfFullName: [u16; 64],
    pub elfStyle: [u16; 32],
}
impl ::core::marker::Copy for ENUMLOGFONTW {}
impl ::core::clone::Clone for ENUMLOGFONTW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENUMLOGFONTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMLOGFONTW")
            .field("elfLogFont", &self.elfLogFont)
            .field("elfFullName", &self.elfFullName)
            .field("elfStyle", &self.elfStyle)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ENUMLOGFONTW {
    fn eq(&self, other: &Self) -> bool {
        self.elfLogFont == other.elfLogFont
            && self.elfFullName == other.elfFullName
            && self.elfStyle == other.elfStyle
    }
}
impl ::core::cmp::Eq for ENUMLOGFONTW {}
impl FromIntoMemory for ENUMLOGFONTW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 156);
        let f_elfLogFont = <LOGFONTW as FromIntoMemory>::from_bytes(&from[0..0 + 60]);
        let f_elfFullName = <[u16; 64] as FromIntoMemory>::from_bytes(&from[60..60 + 64]);
        let f_elfStyle = <[u16; 32] as FromIntoMemory>::from_bytes(&from[124..124 + 32]);
        Self {
            elfLogFont: f_elfLogFont,
            elfFullName: f_elfFullName,
            elfStyle: f_elfStyle,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 156);
        FromIntoMemory::into_bytes(self.elfLogFont, &mut into[0..0 + 60]);
        FromIntoMemory::into_bytes(self.elfFullName, &mut into[60..60 + 64]);
        FromIntoMemory::into_bytes(self.elfStyle, &mut into[124..124 + 32]);
    }
    fn size() -> usize {
        156
    }
}
pub const ENUMPAPERBINS: u32 = 31u32;
pub const ENUMPAPERMETRICS: u32 = 34u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ENUM_DISPLAY_SETTINGS_MODE(pub u32);
pub const ENUM_CURRENT_SETTINGS: ENUM_DISPLAY_SETTINGS_MODE =
    ENUM_DISPLAY_SETTINGS_MODE(4294967295u32);
pub const ENUM_REGISTRY_SETTINGS: ENUM_DISPLAY_SETTINGS_MODE =
    ENUM_DISPLAY_SETTINGS_MODE(4294967294u32);
impl ::core::marker::Copy for ENUM_DISPLAY_SETTINGS_MODE {}
impl ::core::clone::Clone for ENUM_DISPLAY_SETTINGS_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ENUM_DISPLAY_SETTINGS_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ENUM_DISPLAY_SETTINGS_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENUM_DISPLAY_SETTINGS_MODE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ENUM_DISPLAY_SETTINGS_MODE {
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
pub const EPSPRINTING: u32 = 33u32;
pub const EPS_SIGNATURE: u32 = 1179865157u32;
pub const ERROR: u32 = 0u32;
pub const ERR_FORMAT: u32 = 1006u32;
pub const ERR_GENERIC: u32 = 1000u32;
pub const ERR_INVALID_BASE: u32 = 1085u32;
pub const ERR_INVALID_CMAP: u32 = 1060u32;
pub const ERR_INVALID_DELTA_FORMAT: u32 = 1013u32;
pub const ERR_INVALID_EBLC: u32 = 1086u32;
pub const ERR_INVALID_GDEF: u32 = 1083u32;
pub const ERR_INVALID_GLYF: u32 = 1061u32;
pub const ERR_INVALID_GPOS: u32 = 1082u32;
pub const ERR_INVALID_GSUB: u32 = 1081u32;
pub const ERR_INVALID_HDMX: u32 = 1089u32;
pub const ERR_INVALID_HEAD: u32 = 1062u32;
pub const ERR_INVALID_HHEA: u32 = 1063u32;
pub const ERR_INVALID_HHEA_OR_VHEA: u32 = 1072u32;
pub const ERR_INVALID_HMTX: u32 = 1064u32;
pub const ERR_INVALID_HMTX_OR_VMTX: u32 = 1073u32;
pub const ERR_INVALID_JSTF: u32 = 1084u32;
pub const ERR_INVALID_LOCA: u32 = 1065u32;
pub const ERR_INVALID_LTSH: u32 = 1087u32;
pub const ERR_INVALID_MAXP: u32 = 1066u32;
pub const ERR_INVALID_MERGE_CHECKSUMS: u32 = 1011u32;
pub const ERR_INVALID_MERGE_FORMATS: u32 = 1010u32;
pub const ERR_INVALID_MERGE_NUMGLYPHS: u32 = 1012u32;
pub const ERR_INVALID_NAME: u32 = 1067u32;
pub const ERR_INVALID_OS2: u32 = 1069u32;
pub const ERR_INVALID_POST: u32 = 1068u32;
pub const ERR_INVALID_TTC_INDEX: u32 = 1015u32;
pub const ERR_INVALID_TTO: u32 = 1080u32;
pub const ERR_INVALID_VDMX: u32 = 1088u32;
pub const ERR_INVALID_VHEA: u32 = 1070u32;
pub const ERR_INVALID_VMTX: u32 = 1071u32;
pub const ERR_MEM: u32 = 1005u32;
pub const ERR_MISSING_CMAP: u32 = 1030u32;
pub const ERR_MISSING_EBDT: u32 = 1044u32;
pub const ERR_MISSING_GLYF: u32 = 1031u32;
pub const ERR_MISSING_HEAD: u32 = 1032u32;
pub const ERR_MISSING_HHEA: u32 = 1033u32;
pub const ERR_MISSING_HHEA_OR_VHEA: u32 = 1042u32;
pub const ERR_MISSING_HMTX: u32 = 1034u32;
pub const ERR_MISSING_HMTX_OR_VMTX: u32 = 1043u32;
pub const ERR_MISSING_LOCA: u32 = 1035u32;
pub const ERR_MISSING_MAXP: u32 = 1036u32;
pub const ERR_MISSING_NAME: u32 = 1037u32;
pub const ERR_MISSING_OS2: u32 = 1039u32;
pub const ERR_MISSING_POST: u32 = 1038u32;
pub const ERR_MISSING_VHEA: u32 = 1040u32;
pub const ERR_MISSING_VMTX: u32 = 1041u32;
pub const ERR_NOT_TTC: u32 = 1014u32;
pub const ERR_NO_GLYPHS: u32 = 1009u32;
pub const ERR_PARAMETER0: u32 = 1100u32;
pub const ERR_PARAMETER1: u32 = 1101u32;
pub const ERR_PARAMETER10: u32 = 1110u32;
pub const ERR_PARAMETER11: u32 = 1111u32;
pub const ERR_PARAMETER12: u32 = 1112u32;
pub const ERR_PARAMETER13: u32 = 1113u32;
pub const ERR_PARAMETER14: u32 = 1114u32;
pub const ERR_PARAMETER15: u32 = 1115u32;
pub const ERR_PARAMETER16: u32 = 1116u32;
pub const ERR_PARAMETER2: u32 = 1102u32;
pub const ERR_PARAMETER3: u32 = 1103u32;
pub const ERR_PARAMETER4: u32 = 1104u32;
pub const ERR_PARAMETER5: u32 = 1105u32;
pub const ERR_PARAMETER6: u32 = 1106u32;
pub const ERR_PARAMETER7: u32 = 1107u32;
pub const ERR_PARAMETER8: u32 = 1108u32;
pub const ERR_PARAMETER9: u32 = 1109u32;
pub const ERR_READCONTROL: u32 = 1003u32;
pub const ERR_READOUTOFBOUNDS: u32 = 1001u32;
pub const ERR_VERSION: u32 = 1008u32;
pub const ERR_WOULD_GROW: u32 = 1007u32;
pub const ERR_WRITECONTROL: u32 = 1004u32;
pub const ERR_WRITEOUTOFBOUNDS: u32 = 1002u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ETO_OPTIONS(pub u32);
pub const ETO_OPAQUE: ETO_OPTIONS = ETO_OPTIONS(2u32);
pub const ETO_CLIPPED: ETO_OPTIONS = ETO_OPTIONS(4u32);
pub const ETO_GLYPH_INDEX: ETO_OPTIONS = ETO_OPTIONS(16u32);
pub const ETO_RTLREADING: ETO_OPTIONS = ETO_OPTIONS(128u32);
pub const ETO_NUMERICSLOCAL: ETO_OPTIONS = ETO_OPTIONS(1024u32);
pub const ETO_NUMERICSLATIN: ETO_OPTIONS = ETO_OPTIONS(2048u32);
pub const ETO_IGNORELANGUAGE: ETO_OPTIONS = ETO_OPTIONS(4096u32);
pub const ETO_PDY: ETO_OPTIONS = ETO_OPTIONS(8192u32);
pub const ETO_REVERSE_INDEX_MAP: ETO_OPTIONS = ETO_OPTIONS(65536u32);
impl ::core::marker::Copy for ETO_OPTIONS {}
impl ::core::clone::Clone for ETO_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ETO_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ETO_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ETO_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ETO_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ETO_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ETO_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ETO_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ETO_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for ETO_OPTIONS {
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
pub struct EXTLOGFONTA {
    pub elfLogFont: LOGFONTA,
    pub elfFullName: [u8; 64],
    pub elfStyle: [u8; 32],
    pub elfVersion: u32,
    pub elfStyleSize: u32,
    pub elfMatch: u32,
    pub elfReserved: u32,
    pub elfVendorId: [u8; 4],
    pub elfCulture: u32,
    pub elfPanose: PANOSE,
}
impl ::core::marker::Copy for EXTLOGFONTA {}
impl ::core::clone::Clone for EXTLOGFONTA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EXTLOGFONTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXTLOGFONTA")
            .field("elfLogFont", &self.elfLogFont)
            .field("elfFullName", &self.elfFullName)
            .field("elfStyle", &self.elfStyle)
            .field("elfVersion", &self.elfVersion)
            .field("elfStyleSize", &self.elfStyleSize)
            .field("elfMatch", &self.elfMatch)
            .field("elfReserved", &self.elfReserved)
            .field("elfVendorId", &self.elfVendorId)
            .field("elfCulture", &self.elfCulture)
            .field("elfPanose", &self.elfPanose)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EXTLOGFONTA {
    fn eq(&self, other: &Self) -> bool {
        self.elfLogFont == other.elfLogFont
            && self.elfFullName == other.elfFullName
            && self.elfStyle == other.elfStyle
            && self.elfVersion == other.elfVersion
            && self.elfStyleSize == other.elfStyleSize
            && self.elfMatch == other.elfMatch
            && self.elfReserved == other.elfReserved
            && self.elfVendorId == other.elfVendorId
            && self.elfCulture == other.elfCulture
            && self.elfPanose == other.elfPanose
    }
}
impl ::core::cmp::Eq for EXTLOGFONTA {}
impl FromIntoMemory for EXTLOGFONTA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 192);
        let f_elfLogFont = <LOGFONTA as FromIntoMemory>::from_bytes(&from[0..0 + 60]);
        let f_elfFullName = <[u8; 64] as FromIntoMemory>::from_bytes(&from[60..60 + 64]);
        let f_elfStyle = <[u8; 32] as FromIntoMemory>::from_bytes(&from[124..124 + 32]);
        let f_elfVersion = <u32 as FromIntoMemory>::from_bytes(&from[156..156 + 4]);
        let f_elfStyleSize = <u32 as FromIntoMemory>::from_bytes(&from[160..160 + 4]);
        let f_elfMatch = <u32 as FromIntoMemory>::from_bytes(&from[164..164 + 4]);
        let f_elfReserved = <u32 as FromIntoMemory>::from_bytes(&from[168..168 + 4]);
        let f_elfVendorId = <[u8; 4] as FromIntoMemory>::from_bytes(&from[172..172 + 4]);
        let f_elfCulture = <u32 as FromIntoMemory>::from_bytes(&from[176..176 + 4]);
        let f_elfPanose = <PANOSE as FromIntoMemory>::from_bytes(&from[180..180 + 10]);
        Self {
            elfLogFont: f_elfLogFont,
            elfFullName: f_elfFullName,
            elfStyle: f_elfStyle,
            elfVersion: f_elfVersion,
            elfStyleSize: f_elfStyleSize,
            elfMatch: f_elfMatch,
            elfReserved: f_elfReserved,
            elfVendorId: f_elfVendorId,
            elfCulture: f_elfCulture,
            elfPanose: f_elfPanose,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 192);
        FromIntoMemory::into_bytes(self.elfLogFont, &mut into[0..0 + 60]);
        FromIntoMemory::into_bytes(self.elfFullName, &mut into[60..60 + 64]);
        FromIntoMemory::into_bytes(self.elfStyle, &mut into[124..124 + 32]);
        FromIntoMemory::into_bytes(self.elfVersion, &mut into[156..156 + 4]);
        FromIntoMemory::into_bytes(self.elfStyleSize, &mut into[160..160 + 4]);
        FromIntoMemory::into_bytes(self.elfMatch, &mut into[164..164 + 4]);
        FromIntoMemory::into_bytes(self.elfReserved, &mut into[168..168 + 4]);
        FromIntoMemory::into_bytes(self.elfVendorId, &mut into[172..172 + 4]);
        FromIntoMemory::into_bytes(self.elfCulture, &mut into[176..176 + 4]);
        FromIntoMemory::into_bytes(self.elfPanose, &mut into[180..180 + 10]);
    }
    fn size() -> usize {
        192
    }
}
pub struct EXTLOGFONTW {
    pub elfLogFont: LOGFONTW,
    pub elfFullName: [u16; 64],
    pub elfStyle: [u16; 32],
    pub elfVersion: u32,
    pub elfStyleSize: u32,
    pub elfMatch: u32,
    pub elfReserved: u32,
    pub elfVendorId: [u8; 4],
    pub elfCulture: u32,
    pub elfPanose: PANOSE,
}
impl ::core::marker::Copy for EXTLOGFONTW {}
impl ::core::clone::Clone for EXTLOGFONTW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EXTLOGFONTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXTLOGFONTW")
            .field("elfLogFont", &self.elfLogFont)
            .field("elfFullName", &self.elfFullName)
            .field("elfStyle", &self.elfStyle)
            .field("elfVersion", &self.elfVersion)
            .field("elfStyleSize", &self.elfStyleSize)
            .field("elfMatch", &self.elfMatch)
            .field("elfReserved", &self.elfReserved)
            .field("elfVendorId", &self.elfVendorId)
            .field("elfCulture", &self.elfCulture)
            .field("elfPanose", &self.elfPanose)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EXTLOGFONTW {
    fn eq(&self, other: &Self) -> bool {
        self.elfLogFont == other.elfLogFont
            && self.elfFullName == other.elfFullName
            && self.elfStyle == other.elfStyle
            && self.elfVersion == other.elfVersion
            && self.elfStyleSize == other.elfStyleSize
            && self.elfMatch == other.elfMatch
            && self.elfReserved == other.elfReserved
            && self.elfVendorId == other.elfVendorId
            && self.elfCulture == other.elfCulture
            && self.elfPanose == other.elfPanose
    }
}
impl ::core::cmp::Eq for EXTLOGFONTW {}
impl FromIntoMemory for EXTLOGFONTW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 192);
        let f_elfLogFont = <LOGFONTW as FromIntoMemory>::from_bytes(&from[0..0 + 60]);
        let f_elfFullName = <[u16; 64] as FromIntoMemory>::from_bytes(&from[60..60 + 64]);
        let f_elfStyle = <[u16; 32] as FromIntoMemory>::from_bytes(&from[124..124 + 32]);
        let f_elfVersion = <u32 as FromIntoMemory>::from_bytes(&from[156..156 + 4]);
        let f_elfStyleSize = <u32 as FromIntoMemory>::from_bytes(&from[160..160 + 4]);
        let f_elfMatch = <u32 as FromIntoMemory>::from_bytes(&from[164..164 + 4]);
        let f_elfReserved = <u32 as FromIntoMemory>::from_bytes(&from[168..168 + 4]);
        let f_elfVendorId = <[u8; 4] as FromIntoMemory>::from_bytes(&from[172..172 + 4]);
        let f_elfCulture = <u32 as FromIntoMemory>::from_bytes(&from[176..176 + 4]);
        let f_elfPanose = <PANOSE as FromIntoMemory>::from_bytes(&from[180..180 + 10]);
        Self {
            elfLogFont: f_elfLogFont,
            elfFullName: f_elfFullName,
            elfStyle: f_elfStyle,
            elfVersion: f_elfVersion,
            elfStyleSize: f_elfStyleSize,
            elfMatch: f_elfMatch,
            elfReserved: f_elfReserved,
            elfVendorId: f_elfVendorId,
            elfCulture: f_elfCulture,
            elfPanose: f_elfPanose,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 192);
        FromIntoMemory::into_bytes(self.elfLogFont, &mut into[0..0 + 60]);
        FromIntoMemory::into_bytes(self.elfFullName, &mut into[60..60 + 64]);
        FromIntoMemory::into_bytes(self.elfStyle, &mut into[124..124 + 32]);
        FromIntoMemory::into_bytes(self.elfVersion, &mut into[156..156 + 4]);
        FromIntoMemory::into_bytes(self.elfStyleSize, &mut into[160..160 + 4]);
        FromIntoMemory::into_bytes(self.elfMatch, &mut into[164..164 + 4]);
        FromIntoMemory::into_bytes(self.elfReserved, &mut into[168..168 + 4]);
        FromIntoMemory::into_bytes(self.elfVendorId, &mut into[172..172 + 4]);
        FromIntoMemory::into_bytes(self.elfCulture, &mut into[176..176 + 4]);
        FromIntoMemory::into_bytes(self.elfPanose, &mut into[180..180 + 10]);
    }
    fn size() -> usize {
        192
    }
}
pub struct EXTLOGPEN {
    pub elpPenStyle: u32,
    pub elpWidth: u32,
    pub elpBrushStyle: u32,
    pub elpColor: u32,
    pub elpHatch: PtrRepr,
    pub elpNumEntries: u32,
    pub elpStyleEntry: [u32; 1],
}
impl ::core::marker::Copy for EXTLOGPEN {}
impl ::core::clone::Clone for EXTLOGPEN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EXTLOGPEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXTLOGPEN")
            .field("elpPenStyle", &self.elpPenStyle)
            .field("elpWidth", &self.elpWidth)
            .field("elpBrushStyle", &self.elpBrushStyle)
            .field("elpColor", &self.elpColor)
            .field("elpHatch", &self.elpHatch)
            .field("elpNumEntries", &self.elpNumEntries)
            .field("elpStyleEntry", &self.elpStyleEntry)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EXTLOGPEN {
    fn eq(&self, other: &Self) -> bool {
        self.elpPenStyle == other.elpPenStyle
            && self.elpWidth == other.elpWidth
            && self.elpBrushStyle == other.elpBrushStyle
            && self.elpColor == other.elpColor
            && self.elpHatch == other.elpHatch
            && self.elpNumEntries == other.elpNumEntries
            && self.elpStyleEntry == other.elpStyleEntry
    }
}
impl ::core::cmp::Eq for EXTLOGPEN {}
impl FromIntoMemory for EXTLOGPEN {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_elpPenStyle = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_elpWidth = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_elpBrushStyle = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_elpColor = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_elpHatch = <PtrRepr as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_elpNumEntries = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_elpStyleEntry = <[u32; 1] as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        Self {
            elpPenStyle: f_elpPenStyle,
            elpWidth: f_elpWidth,
            elpBrushStyle: f_elpBrushStyle,
            elpColor: f_elpColor,
            elpHatch: f_elpHatch,
            elpNumEntries: f_elpNumEntries,
            elpStyleEntry: f_elpStyleEntry,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.elpPenStyle, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.elpWidth, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.elpBrushStyle, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.elpColor, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.elpHatch, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.elpNumEntries, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.elpStyleEntry, &mut into[24..24 + 4]);
    }
    fn size() -> usize {
        28
    }
}
pub struct EXTLOGPEN32 {
    pub elpPenStyle: u32,
    pub elpWidth: u32,
    pub elpBrushStyle: u32,
    pub elpColor: u32,
    pub elpHatch: u32,
    pub elpNumEntries: u32,
    pub elpStyleEntry: [u32; 1],
}
impl ::core::marker::Copy for EXTLOGPEN32 {}
impl ::core::clone::Clone for EXTLOGPEN32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EXTLOGPEN32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXTLOGPEN32")
            .field("elpPenStyle", &self.elpPenStyle)
            .field("elpWidth", &self.elpWidth)
            .field("elpBrushStyle", &self.elpBrushStyle)
            .field("elpColor", &self.elpColor)
            .field("elpHatch", &self.elpHatch)
            .field("elpNumEntries", &self.elpNumEntries)
            .field("elpStyleEntry", &self.elpStyleEntry)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EXTLOGPEN32 {
    fn eq(&self, other: &Self) -> bool {
        self.elpPenStyle == other.elpPenStyle
            && self.elpWidth == other.elpWidth
            && self.elpBrushStyle == other.elpBrushStyle
            && self.elpColor == other.elpColor
            && self.elpHatch == other.elpHatch
            && self.elpNumEntries == other.elpNumEntries
            && self.elpStyleEntry == other.elpStyleEntry
    }
}
impl ::core::cmp::Eq for EXTLOGPEN32 {}
impl FromIntoMemory for EXTLOGPEN32 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_elpPenStyle = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_elpWidth = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_elpBrushStyle = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_elpColor = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_elpHatch = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_elpNumEntries = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_elpStyleEntry = <[u32; 1] as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        Self {
            elpPenStyle: f_elpPenStyle,
            elpWidth: f_elpWidth,
            elpBrushStyle: f_elpBrushStyle,
            elpColor: f_elpColor,
            elpHatch: f_elpHatch,
            elpNumEntries: f_elpNumEntries,
            elpStyleEntry: f_elpStyleEntry,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.elpPenStyle, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.elpWidth, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.elpBrushStyle, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.elpColor, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.elpHatch, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.elpNumEntries, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.elpStyleEntry, &mut into[24..24 + 4]);
    }
    fn size() -> usize {
        28
    }
}
pub const EXTTEXTOUT: u32 = 512u32;
pub const EXT_DEVICE_CAPS: u32 = 4099u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EXT_FLOOD_FILL_TYPE(pub u32);
pub const FLOODFILLBORDER: EXT_FLOOD_FILL_TYPE = EXT_FLOOD_FILL_TYPE(0u32);
pub const FLOODFILLSURFACE: EXT_FLOOD_FILL_TYPE = EXT_FLOOD_FILL_TYPE(1u32);
impl ::core::marker::Copy for EXT_FLOOD_FILL_TYPE {}
impl ::core::clone::Clone for EXT_FLOOD_FILL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EXT_FLOOD_FILL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EXT_FLOOD_FILL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EXT_FLOOD_FILL_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for EXT_FLOOD_FILL_TYPE {
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
pub const E_ADDFONTFAILED: i32 = 512i32;
pub const E_API_NOTIMPL: i32 = 1i32;
pub const E_CHARCODECOUNTINVALID: i32 = 2i32;
pub const E_CHARCODESETINVALID: i32 = 3i32;
pub const E_CHARSETINVALID: i32 = 21i32;
pub const E_COULDNTCREATETEMPFILE: i32 = 513i32;
pub const E_DEVICETRUETYPEFONT: i32 = 4i32;
pub const E_ERRORACCESSINGEXCLUDELIST: i32 = 274i32;
pub const E_ERRORACCESSINGFACENAME: i32 = 13i32;
pub const E_ERRORACCESSINGFONTDATA: i32 = 12i32;
pub const E_ERRORCOMPRESSINGFONTDATA: i32 = 256i32;
pub const E_ERRORCONVERTINGCHARS: i32 = 18i32;
pub const E_ERRORCREATINGFONTFILE: i32 = 269i32;
pub const E_ERRORDECOMPRESSINGFONTDATA: i32 = 273i32;
pub const E_ERROREXPANDINGFONTDATA: i32 = 519i32;
pub const E_ERRORGETTINGDC: i32 = 520i32;
pub const E_ERRORREADINGFONTDATA: i32 = 267i32;
pub const E_ERRORUNICODECONVERSION: i32 = 17i32;
pub const E_EXCEPTION: i32 = 19i32;
pub const E_EXCEPTIONINCOMPRESSION: i32 = 522i32;
pub const E_EXCEPTIONINDECOMPRESSION: i32 = 521i32;
pub const E_FACENAMEINVALID: i32 = 275i32;
pub const E_FILE_NOT_FOUND: i32 = 23i32;
pub const E_FLAGSINVALID: i32 = 268i32;
pub const E_FONTALREADYEXISTS: i32 = 270i32;
pub const E_FONTDATAINVALID: i32 = 258i32;
pub const E_FONTFAMILYNAMENOTINFULL: i32 = 285i32;
pub const E_FONTFILECREATEFAILED: i32 = 515i32;
pub const E_FONTFILENOTFOUND: i32 = 517i32;
pub const E_FONTINSTALLFAILED: i32 = 272i32;
pub const E_FONTNAMEALREADYEXISTS: i32 = 271i32;
pub const E_FONTNOTEMBEDDABLE: i32 = 260i32;
pub const E_FONTREFERENCEINVALID: i32 = 8i32;
pub const E_FONTVARIATIONSIMULATED: i32 = 283i32;
pub const E_HDCINVALID: i32 = 6i32;
pub const E_INPUTPARAMINVALID: i32 = 25i32;
pub const E_NAMECHANGEFAILED: i32 = 259i32;
pub const E_NOFREEMEMORY: i32 = 7i32;
pub const E_NONE: i32 = 0i32;
pub const E_NOOS2: i32 = 265i32;
pub const E_NOTATRUETYPEFONT: i32 = 10i32;
pub const E_PBENABLEDINVALID: i32 = 280i32;
pub const E_PERMISSIONSINVALID: i32 = 279i32;
pub const E_PRIVSINVALID: i32 = 261i32;
pub const E_PRIVSTATUSINVALID: i32 = 278i32;
pub const E_READFROMSTREAMFAILED: i32 = 263i32;
pub const E_RESERVEDPARAMNOTNULL: i32 = 20i32;
pub const E_RESOURCEFILECREATEFAILED: i32 = 518i32;
pub const E_SAVETOSTREAMFAILED: i32 = 264i32;
pub const E_STATUSINVALID: i32 = 277i32;
pub const E_STREAMINVALID: i32 = 276i32;
pub const E_SUBSETTINGEXCEPTION: i32 = 281i32;
pub const E_SUBSETTINGFAILED: i32 = 262i32;
pub const E_SUBSTRING_TEST_FAIL: i32 = 282i32;
pub const E_T2NOFREEMEMORY: i32 = 266i32;
pub const E_TTC_INDEX_OUT_OF_RANGE: i32 = 24i32;
pub const E_WINDOWSAPI: i32 = 516i32;
pub const FEATURESETTING_CUSTPAPER: u32 = 3u32;
pub const FEATURESETTING_MIRROR: u32 = 4u32;
pub const FEATURESETTING_NEGATIVE: u32 = 5u32;
pub const FEATURESETTING_NUP: u32 = 0u32;
pub const FEATURESETTING_OUTPUT: u32 = 1u32;
pub const FEATURESETTING_PRIVATE_BEGIN: u32 = 4096u32;
pub const FEATURESETTING_PRIVATE_END: u32 = 8191u32;
pub const FEATURESETTING_PROTOCOL: u32 = 6u32;
pub const FEATURESETTING_PSLEVEL: u32 = 2u32;
pub struct FIXED {
    pub fract: u16,
    pub value: i16,
}
impl ::core::marker::Copy for FIXED {}
impl ::core::clone::Clone for FIXED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FIXED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FIXED")
            .field("fract", &self.fract)
            .field("value", &self.value)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FIXED {
    fn eq(&self, other: &Self) -> bool {
        self.fract == other.fract && self.value == other.value
    }
}
impl ::core::cmp::Eq for FIXED {}
impl FromIntoMemory for FIXED {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_fract = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_value = <i16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        Self {
            fract: f_fract,
            value: f_value,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.fract, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.value, &mut into[2..2 + 2]);
    }
    fn size() -> usize {
        4
    }
}
pub const FIXED_PITCH: u32 = 1u32;
pub const FLI_GLYPHS: i32 = 262144i32;
pub const FLI_MASK: u32 = 4155u32;
pub const FLUSHOUTPUT: u32 = 6u32;
pub type FONTENUMPROCA = StdCallFnPtr<
    (
        ConstPtr<LOGFONTA>,
        ConstPtr<TEXTMETRICA>,
        u32,
        super::super::Foundation::LPARAM,
    ),
    i32,
>;
pub type FONTENUMPROCW = StdCallFnPtr<
    (
        ConstPtr<LOGFONTW>,
        ConstPtr<TEXTMETRICW>,
        u32,
        super::super::Foundation::LPARAM,
    ),
    i32,
>;
pub const FONTMAPPER_MAX: u32 = 10u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FONT_CLIP_PRECISION(pub u32);
pub const CLIP_CHARACTER_PRECIS: FONT_CLIP_PRECISION = FONT_CLIP_PRECISION(1u32);
pub const CLIP_DEFAULT_PRECIS: FONT_CLIP_PRECISION = FONT_CLIP_PRECISION(0u32);
pub const CLIP_DFA_DISABLE: FONT_CLIP_PRECISION = FONT_CLIP_PRECISION(64u32);
pub const CLIP_EMBEDDED: FONT_CLIP_PRECISION = FONT_CLIP_PRECISION(128u32);
pub const CLIP_LH_ANGLES: FONT_CLIP_PRECISION = FONT_CLIP_PRECISION(16u32);
pub const CLIP_MASK: FONT_CLIP_PRECISION = FONT_CLIP_PRECISION(15u32);
pub const CLIP_STROKE_PRECIS: FONT_CLIP_PRECISION = FONT_CLIP_PRECISION(2u32);
pub const CLIP_TT_ALWAYS: FONT_CLIP_PRECISION = FONT_CLIP_PRECISION(32u32);
impl ::core::marker::Copy for FONT_CLIP_PRECISION {}
impl ::core::clone::Clone for FONT_CLIP_PRECISION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FONT_CLIP_PRECISION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FONT_CLIP_PRECISION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FONT_CLIP_PRECISION").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FONT_CLIP_PRECISION {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FONT_CLIP_PRECISION {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FONT_CLIP_PRECISION {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FONT_CLIP_PRECISION {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FONT_CLIP_PRECISION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for FONT_CLIP_PRECISION {
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
pub struct FONT_LICENSE_PRIVS(pub u32);
pub const LICENSE_PREVIEWPRINT: FONT_LICENSE_PRIVS = FONT_LICENSE_PRIVS(4u32);
pub const LICENSE_EDITABLE: FONT_LICENSE_PRIVS = FONT_LICENSE_PRIVS(8u32);
pub const LICENSE_INSTALLABLE: FONT_LICENSE_PRIVS = FONT_LICENSE_PRIVS(0u32);
pub const LICENSE_NOEMBEDDING: FONT_LICENSE_PRIVS = FONT_LICENSE_PRIVS(2u32);
pub const LICENSE_DEFAULT: FONT_LICENSE_PRIVS = FONT_LICENSE_PRIVS(0u32);
impl ::core::marker::Copy for FONT_LICENSE_PRIVS {}
impl ::core::clone::Clone for FONT_LICENSE_PRIVS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FONT_LICENSE_PRIVS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FONT_LICENSE_PRIVS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FONT_LICENSE_PRIVS").field(&self.0).finish()
    }
}
impl FromIntoMemory for FONT_LICENSE_PRIVS {
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
pub struct FONT_OUTPUT_PRECISION(pub u32);
pub const OUT_CHARACTER_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(2u32);
pub const OUT_DEFAULT_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(0u32);
pub const OUT_DEVICE_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(5u32);
pub const OUT_OUTLINE_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(8u32);
pub const OUT_PS_ONLY_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(10u32);
pub const OUT_RASTER_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(6u32);
pub const OUT_STRING_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(1u32);
pub const OUT_STROKE_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(3u32);
pub const OUT_TT_ONLY_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(7u32);
pub const OUT_TT_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(4u32);
impl ::core::marker::Copy for FONT_OUTPUT_PRECISION {}
impl ::core::clone::Clone for FONT_OUTPUT_PRECISION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FONT_OUTPUT_PRECISION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FONT_OUTPUT_PRECISION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FONT_OUTPUT_PRECISION")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for FONT_OUTPUT_PRECISION {
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
pub struct FONT_PITCH_AND_FAMILY(pub u32);
pub const FF_DECORATIVE: FONT_PITCH_AND_FAMILY = FONT_PITCH_AND_FAMILY(80u32);
pub const FF_DONTCARE: FONT_PITCH_AND_FAMILY = FONT_PITCH_AND_FAMILY(0u32);
pub const FF_MODERN: FONT_PITCH_AND_FAMILY = FONT_PITCH_AND_FAMILY(48u32);
pub const FF_ROMAN: FONT_PITCH_AND_FAMILY = FONT_PITCH_AND_FAMILY(16u32);
pub const FF_SCRIPT: FONT_PITCH_AND_FAMILY = FONT_PITCH_AND_FAMILY(64u32);
pub const FF_SWISS: FONT_PITCH_AND_FAMILY = FONT_PITCH_AND_FAMILY(32u32);
impl ::core::marker::Copy for FONT_PITCH_AND_FAMILY {}
impl ::core::clone::Clone for FONT_PITCH_AND_FAMILY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FONT_PITCH_AND_FAMILY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FONT_PITCH_AND_FAMILY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FONT_PITCH_AND_FAMILY")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for FONT_PITCH_AND_FAMILY {
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
pub struct FONT_QUALITY(pub u32);
pub const ANTIALIASED_QUALITY: FONT_QUALITY = FONT_QUALITY(4u32);
pub const CLEARTYPE_QUALITY: FONT_QUALITY = FONT_QUALITY(5u32);
pub const DEFAULT_QUALITY: FONT_QUALITY = FONT_QUALITY(0u32);
pub const DRAFT_QUALITY: FONT_QUALITY = FONT_QUALITY(1u32);
pub const NONANTIALIASED_QUALITY: FONT_QUALITY = FONT_QUALITY(3u32);
pub const PROOF_QUALITY: FONT_QUALITY = FONT_QUALITY(2u32);
impl ::core::marker::Copy for FONT_QUALITY {}
impl ::core::clone::Clone for FONT_QUALITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FONT_QUALITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FONT_QUALITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FONT_QUALITY").field(&self.0).finish()
    }
}
impl FromIntoMemory for FONT_QUALITY {
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
pub struct FONT_RESOURCE_CHARACTERISTICS(pub u32);
pub const FR_PRIVATE: FONT_RESOURCE_CHARACTERISTICS = FONT_RESOURCE_CHARACTERISTICS(16u32);
pub const FR_NOT_ENUM: FONT_RESOURCE_CHARACTERISTICS = FONT_RESOURCE_CHARACTERISTICS(32u32);
impl ::core::marker::Copy for FONT_RESOURCE_CHARACTERISTICS {}
impl ::core::clone::Clone for FONT_RESOURCE_CHARACTERISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FONT_RESOURCE_CHARACTERISTICS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FONT_RESOURCE_CHARACTERISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FONT_RESOURCE_CHARACTERISTICS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for FONT_RESOURCE_CHARACTERISTICS {
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
pub const FS_ARABIC: i32 = 64i32;
pub const FS_BALTIC: i32 = 128i32;
pub const FS_CHINESESIMP: i32 = 262144i32;
pub const FS_CHINESETRAD: i32 = 1048576i32;
pub const FS_CYRILLIC: i32 = 4i32;
pub const FS_GREEK: i32 = 8i32;
pub const FS_HEBREW: i32 = 32i32;
pub const FS_JISJAPAN: i32 = 131072i32;
pub const FS_JOHAB: i32 = 2097152i32;
pub const FS_LATIN1: i32 = 1i32;
pub const FS_LATIN2: i32 = 2i32;
pub const FS_SYMBOL: i32 = -2147483648i32;
pub const FS_THAI: i32 = 65536i32;
pub const FS_TURKISH: i32 = 16i32;
pub const FS_VIETNAMESE: i32 = 256i32;
pub const FS_WANSUNG: i32 = 524288i32;
pub const FW_BLACK: u32 = 900u32;
pub const FW_BOLD: u32 = 700u32;
pub const FW_DEMIBOLD: u32 = 600u32;
pub const FW_DONTCARE: u32 = 0u32;
pub const FW_EXTRABOLD: u32 = 800u32;
pub const FW_EXTRALIGHT: u32 = 200u32;
pub const FW_HEAVY: u32 = 900u32;
pub const FW_LIGHT: u32 = 300u32;
pub const FW_MEDIUM: u32 = 500u32;
pub const FW_NORMAL: u32 = 400u32;
pub const FW_REGULAR: u32 = 400u32;
pub const FW_SEMIBOLD: u32 = 600u32;
pub const FW_THIN: u32 = 100u32;
pub const FW_ULTRABOLD: u32 = 800u32;
pub const FW_ULTRALIGHT: u32 = 200u32;
pub const GB2312_CHARSET: u32 = 134u32;
pub const GCPCLASS_ARABIC: u32 = 2u32;
pub const GCPCLASS_HEBREW: u32 = 2u32;
pub const GCPCLASS_LATIN: u32 = 1u32;
pub const GCPCLASS_LATINNUMBER: u32 = 5u32;
pub const GCPCLASS_LATINNUMERICSEPARATOR: u32 = 7u32;
pub const GCPCLASS_LATINNUMERICTERMINATOR: u32 = 6u32;
pub const GCPCLASS_LOCALNUMBER: u32 = 4u32;
pub const GCPCLASS_NEUTRAL: u32 = 3u32;
pub const GCPCLASS_NUMERICSEPARATOR: u32 = 8u32;
pub const GCPCLASS_POSTBOUNDLTR: u32 = 32u32;
pub const GCPCLASS_POSTBOUNDRTL: u32 = 16u32;
pub const GCPCLASS_PREBOUNDLTR: u32 = 128u32;
pub const GCPCLASS_PREBOUNDRTL: u32 = 64u32;
pub const GCPGLYPH_LINKAFTER: u32 = 16384u32;
pub const GCPGLYPH_LINKBEFORE: u32 = 32768u32;
pub const GCP_DBCS: u32 = 1u32;
pub const GCP_ERROR: u32 = 32768u32;
pub const GCP_JUSTIFYIN: i32 = 2097152i32;
pub struct GCP_RESULTSA {
    pub lStructSize: u32,
    pub lpOutString: PSTR,
    pub lpOrder: MutPtr<u32>,
    pub lpDx: MutPtr<i32>,
    pub lpCaretPos: MutPtr<i32>,
    pub lpClass: PSTR,
    pub lpGlyphs: PWSTR,
    pub nGlyphs: u32,
    pub nMaxFit: i32,
}
impl ::core::marker::Copy for GCP_RESULTSA {}
impl ::core::clone::Clone for GCP_RESULTSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GCP_RESULTSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GCP_RESULTSA")
            .field("lStructSize", &self.lStructSize)
            .field("lpOutString", &self.lpOutString)
            .field("lpOrder", &self.lpOrder)
            .field("lpDx", &self.lpDx)
            .field("lpCaretPos", &self.lpCaretPos)
            .field("lpClass", &self.lpClass)
            .field("lpGlyphs", &self.lpGlyphs)
            .field("nGlyphs", &self.nGlyphs)
            .field("nMaxFit", &self.nMaxFit)
            .finish()
    }
}
impl ::core::cmp::PartialEq for GCP_RESULTSA {
    fn eq(&self, other: &Self) -> bool {
        self.lStructSize == other.lStructSize
            && self.lpOutString == other.lpOutString
            && self.lpOrder == other.lpOrder
            && self.lpDx == other.lpDx
            && self.lpCaretPos == other.lpCaretPos
            && self.lpClass == other.lpClass
            && self.lpGlyphs == other.lpGlyphs
            && self.nGlyphs == other.nGlyphs
            && self.nMaxFit == other.nMaxFit
    }
}
impl ::core::cmp::Eq for GCP_RESULTSA {}
impl FromIntoMemory for GCP_RESULTSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 36);
        let f_lStructSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpOutString = <PSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpOrder = <MutPtr<u32> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpDx = <MutPtr<i32> as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpCaretPos = <MutPtr<i32> as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_lpClass = <PSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_lpGlyphs = <PWSTR as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_nGlyphs = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_nMaxFit = <i32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        Self {
            lStructSize: f_lStructSize,
            lpOutString: f_lpOutString,
            lpOrder: f_lpOrder,
            lpDx: f_lpDx,
            lpCaretPos: f_lpCaretPos,
            lpClass: f_lpClass,
            lpGlyphs: f_lpGlyphs,
            nGlyphs: f_nGlyphs,
            nMaxFit: f_nMaxFit,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 36);
        FromIntoMemory::into_bytes(self.lStructSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpOutString, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpOrder, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpDx, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpCaretPos, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.lpClass, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.lpGlyphs, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.nGlyphs, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.nMaxFit, &mut into[32..32 + 4]);
    }
    fn size() -> usize {
        36
    }
}
pub struct GCP_RESULTSW {
    pub lStructSize: u32,
    pub lpOutString: PWSTR,
    pub lpOrder: MutPtr<u32>,
    pub lpDx: MutPtr<i32>,
    pub lpCaretPos: MutPtr<i32>,
    pub lpClass: PSTR,
    pub lpGlyphs: PWSTR,
    pub nGlyphs: u32,
    pub nMaxFit: i32,
}
impl ::core::marker::Copy for GCP_RESULTSW {}
impl ::core::clone::Clone for GCP_RESULTSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GCP_RESULTSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GCP_RESULTSW")
            .field("lStructSize", &self.lStructSize)
            .field("lpOutString", &self.lpOutString)
            .field("lpOrder", &self.lpOrder)
            .field("lpDx", &self.lpDx)
            .field("lpCaretPos", &self.lpCaretPos)
            .field("lpClass", &self.lpClass)
            .field("lpGlyphs", &self.lpGlyphs)
            .field("nGlyphs", &self.nGlyphs)
            .field("nMaxFit", &self.nMaxFit)
            .finish()
    }
}
impl ::core::cmp::PartialEq for GCP_RESULTSW {
    fn eq(&self, other: &Self) -> bool {
        self.lStructSize == other.lStructSize
            && self.lpOutString == other.lpOutString
            && self.lpOrder == other.lpOrder
            && self.lpDx == other.lpDx
            && self.lpCaretPos == other.lpCaretPos
            && self.lpClass == other.lpClass
            && self.lpGlyphs == other.lpGlyphs
            && self.nGlyphs == other.nGlyphs
            && self.nMaxFit == other.nMaxFit
    }
}
impl ::core::cmp::Eq for GCP_RESULTSW {}
impl FromIntoMemory for GCP_RESULTSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 36);
        let f_lStructSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpOutString = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpOrder = <MutPtr<u32> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpDx = <MutPtr<i32> as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpCaretPos = <MutPtr<i32> as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_lpClass = <PSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_lpGlyphs = <PWSTR as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_nGlyphs = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_nMaxFit = <i32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        Self {
            lStructSize: f_lStructSize,
            lpOutString: f_lpOutString,
            lpOrder: f_lpOrder,
            lpDx: f_lpDx,
            lpCaretPos: f_lpCaretPos,
            lpClass: f_lpClass,
            lpGlyphs: f_lpGlyphs,
            nGlyphs: f_nGlyphs,
            nMaxFit: f_nMaxFit,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 36);
        FromIntoMemory::into_bytes(self.lStructSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpOutString, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpOrder, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpDx, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpCaretPos, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.lpClass, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.lpGlyphs, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.nGlyphs, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.nMaxFit, &mut into[32..32 + 4]);
    }
    fn size() -> usize {
        36
    }
}
pub const GDICOMMENT_BEGINGROUP: u32 = 2u32;
pub const GDICOMMENT_ENDGROUP: u32 = 3u32;
pub const GDICOMMENT_IDENTIFIER: u32 = 1128875079u32;
pub const GDICOMMENT_MULTIFORMATS: u32 = 1073741828u32;
pub const GDICOMMENT_UNICODE_END: u32 = 128u32;
pub const GDICOMMENT_UNICODE_STRING: u32 = 64u32;
pub const GDICOMMENT_WINDOWS_METAFILE: u32 = 2147483649u32;
pub const GDIPLUS_TS_QUERYVER: u32 = 4122u32;
pub const GDIPLUS_TS_RECORD: u32 = 4123u32;
pub const GDIREGISTERDDRAWPACKETVERSION: u32 = 1u32;
pub const GDI_ERROR: i32 = -1i32;
pub const GETCOLORTABLE: u32 = 5u32;
pub const GETDEVICEUNITS: u32 = 42u32;
pub const GETEXTENDEDTEXTMETRICS: u32 = 256u32;
pub const GETEXTENTTABLE: u32 = 257u32;
pub const GETFACENAME: u32 = 513u32;
pub const GETPAIRKERNTABLE: u32 = 258u32;
pub const GETPENWIDTH: u32 = 16u32;
pub const GETPHYSPAGESIZE: u32 = 12u32;
pub const GETPRINTINGOFFSET: u32 = 13u32;
pub const GETSCALINGFACTOR: u32 = 14u32;
pub const GETSETPAPERBINS: u32 = 29u32;
pub const GETSETPAPERMETRICS: u32 = 35u32;
pub const GETSETPRINTORIENT: u32 = 30u32;
pub const GETSETSCREENPARAMS: u32 = 3072u32;
pub const GETTECHNOLGY: u32 = 20u32;
pub const GETTECHNOLOGY: u32 = 20u32;
pub const GETTRACKKERNTABLE: u32 = 259u32;
pub const GETVECTORBRUSHSIZE: u32 = 27u32;
pub const GETVECTORPENSIZE: u32 = 26u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GET_CHARACTER_PLACEMENT_FLAGS(pub u32);
pub const GCP_CLASSIN: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(524288u32);
pub const GCP_DIACRITIC: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(256u32);
pub const GCP_DISPLAYZWG: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(4194304u32);
pub const GCP_GLYPHSHAPE: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(16u32);
pub const GCP_JUSTIFY: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(65536u32);
pub const GCP_KASHIDA: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(1024u32);
pub const GCP_LIGATE: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(32u32);
pub const GCP_MAXEXTENT: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(1048576u32);
pub const GCP_NEUTRALOVERRIDE: GET_CHARACTER_PLACEMENT_FLAGS =
    GET_CHARACTER_PLACEMENT_FLAGS(33554432u32);
pub const GCP_NUMERICOVERRIDE: GET_CHARACTER_PLACEMENT_FLAGS =
    GET_CHARACTER_PLACEMENT_FLAGS(16777216u32);
pub const GCP_NUMERICSLATIN: GET_CHARACTER_PLACEMENT_FLAGS =
    GET_CHARACTER_PLACEMENT_FLAGS(67108864u32);
pub const GCP_NUMERICSLOCAL: GET_CHARACTER_PLACEMENT_FLAGS =
    GET_CHARACTER_PLACEMENT_FLAGS(134217728u32);
pub const GCP_REORDER: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(2u32);
pub const GCP_SYMSWAPOFF: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(8388608u32);
pub const GCP_USEKERNING: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(8u32);
impl ::core::marker::Copy for GET_CHARACTER_PLACEMENT_FLAGS {}
impl ::core::clone::Clone for GET_CHARACTER_PLACEMENT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_CHARACTER_PLACEMENT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_CHARACTER_PLACEMENT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_CHARACTER_PLACEMENT_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for GET_CHARACTER_PLACEMENT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GET_CHARACTER_PLACEMENT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GET_CHARACTER_PLACEMENT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GET_CHARACTER_PLACEMENT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GET_CHARACTER_PLACEMENT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for GET_CHARACTER_PLACEMENT_FLAGS {
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
pub struct GET_DCX_FLAGS(pub u32);
pub const DCX_WINDOW: GET_DCX_FLAGS = GET_DCX_FLAGS(1u32);
pub const DCX_CACHE: GET_DCX_FLAGS = GET_DCX_FLAGS(2u32);
pub const DCX_PARENTCLIP: GET_DCX_FLAGS = GET_DCX_FLAGS(32u32);
pub const DCX_CLIPSIBLINGS: GET_DCX_FLAGS = GET_DCX_FLAGS(16u32);
pub const DCX_CLIPCHILDREN: GET_DCX_FLAGS = GET_DCX_FLAGS(8u32);
pub const DCX_NORESETATTRS: GET_DCX_FLAGS = GET_DCX_FLAGS(4u32);
pub const DCX_LOCKWINDOWUPDATE: GET_DCX_FLAGS = GET_DCX_FLAGS(1024u32);
pub const DCX_EXCLUDERGN: GET_DCX_FLAGS = GET_DCX_FLAGS(64u32);
pub const DCX_INTERSECTRGN: GET_DCX_FLAGS = GET_DCX_FLAGS(128u32);
pub const DCX_INTERSECTUPDATE: GET_DCX_FLAGS = GET_DCX_FLAGS(512u32);
pub const DCX_VALIDATE: GET_DCX_FLAGS = GET_DCX_FLAGS(2097152u32);
impl ::core::marker::Copy for GET_DCX_FLAGS {}
impl ::core::clone::Clone for GET_DCX_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_DCX_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_DCX_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_DCX_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GET_DCX_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GET_DCX_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GET_DCX_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GET_DCX_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GET_DCX_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for GET_DCX_FLAGS {
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
pub struct GET_DEVICE_CAPS_INDEX(pub u32);
pub const DRIVERVERSION: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(0u32);
pub const TECHNOLOGY: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(2u32);
pub const HORZSIZE: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(4u32);
pub const VERTSIZE: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(6u32);
pub const HORZRES: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(8u32);
pub const VERTRES: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(10u32);
pub const BITSPIXEL: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(12u32);
pub const PLANES: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(14u32);
pub const NUMBRUSHES: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(16u32);
pub const NUMPENS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(18u32);
pub const NUMMARKERS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(20u32);
pub const NUMFONTS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(22u32);
pub const NUMCOLORS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(24u32);
pub const PDEVICESIZE: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(26u32);
pub const CURVECAPS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(28u32);
pub const LINECAPS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(30u32);
pub const POLYGONALCAPS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(32u32);
pub const TEXTCAPS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(34u32);
pub const CLIPCAPS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(36u32);
pub const RASTERCAPS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(38u32);
pub const ASPECTX: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(40u32);
pub const ASPECTY: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(42u32);
pub const ASPECTXY: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(44u32);
pub const LOGPIXELSX: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(88u32);
pub const LOGPIXELSY: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(90u32);
pub const SIZEPALETTE: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(104u32);
pub const NUMRESERVED: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(106u32);
pub const COLORRES: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(108u32);
pub const PHYSICALWIDTH: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(110u32);
pub const PHYSICALHEIGHT: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(111u32);
pub const PHYSICALOFFSETX: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(112u32);
pub const PHYSICALOFFSETY: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(113u32);
pub const SCALINGFACTORX: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(114u32);
pub const SCALINGFACTORY: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(115u32);
pub const VREFRESH: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(116u32);
pub const DESKTOPVERTRES: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(117u32);
pub const DESKTOPHORZRES: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(118u32);
pub const BLTALIGNMENT: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(119u32);
pub const SHADEBLENDCAPS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(120u32);
pub const COLORMGMTCAPS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(121u32);
impl ::core::marker::Copy for GET_DEVICE_CAPS_INDEX {}
impl ::core::clone::Clone for GET_DEVICE_CAPS_INDEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_DEVICE_CAPS_INDEX {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_DEVICE_CAPS_INDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_DEVICE_CAPS_INDEX")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for GET_DEVICE_CAPS_INDEX {
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
pub struct GET_GLYPH_OUTLINE_FORMAT(pub u32);
pub const GGO_BEZIER: GET_GLYPH_OUTLINE_FORMAT = GET_GLYPH_OUTLINE_FORMAT(3u32);
pub const GGO_BITMAP: GET_GLYPH_OUTLINE_FORMAT = GET_GLYPH_OUTLINE_FORMAT(1u32);
pub const GGO_GLYPH_INDEX: GET_GLYPH_OUTLINE_FORMAT = GET_GLYPH_OUTLINE_FORMAT(128u32);
pub const GGO_GRAY2_BITMAP: GET_GLYPH_OUTLINE_FORMAT = GET_GLYPH_OUTLINE_FORMAT(4u32);
pub const GGO_GRAY4_BITMAP: GET_GLYPH_OUTLINE_FORMAT = GET_GLYPH_OUTLINE_FORMAT(5u32);
pub const GGO_GRAY8_BITMAP: GET_GLYPH_OUTLINE_FORMAT = GET_GLYPH_OUTLINE_FORMAT(6u32);
pub const GGO_METRICS: GET_GLYPH_OUTLINE_FORMAT = GET_GLYPH_OUTLINE_FORMAT(0u32);
pub const GGO_NATIVE: GET_GLYPH_OUTLINE_FORMAT = GET_GLYPH_OUTLINE_FORMAT(2u32);
pub const GGO_UNHINTED: GET_GLYPH_OUTLINE_FORMAT = GET_GLYPH_OUTLINE_FORMAT(256u32);
impl ::core::marker::Copy for GET_GLYPH_OUTLINE_FORMAT {}
impl ::core::clone::Clone for GET_GLYPH_OUTLINE_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_GLYPH_OUTLINE_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_GLYPH_OUTLINE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_GLYPH_OUTLINE_FORMAT")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for GET_GLYPH_OUTLINE_FORMAT {
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
pub const GET_PS_FEATURESETTING: u32 = 4121u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GET_STOCK_OBJECT_FLAGS(pub u32);
pub const BLACK_BRUSH: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(4u32);
pub const DKGRAY_BRUSH: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(3u32);
pub const DC_BRUSH: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(18u32);
pub const GRAY_BRUSH: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(2u32);
pub const HOLLOW_BRUSH: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(5u32);
pub const LTGRAY_BRUSH: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(1u32);
pub const NULL_BRUSH: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(5u32);
pub const WHITE_BRUSH: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(0u32);
pub const BLACK_PEN: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(7u32);
pub const DC_PEN: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(19u32);
pub const NULL_PEN: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(8u32);
pub const WHITE_PEN: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(6u32);
pub const ANSI_FIXED_FONT: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(11u32);
pub const ANSI_VAR_FONT: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(12u32);
pub const DEVICE_DEFAULT_FONT: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(14u32);
pub const DEFAULT_GUI_FONT: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(17u32);
pub const OEM_FIXED_FONT: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(10u32);
pub const SYSTEM_FONT: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(13u32);
pub const SYSTEM_FIXED_FONT: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(16u32);
pub const DEFAULT_PALETTE: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(15u32);
impl ::core::marker::Copy for GET_STOCK_OBJECT_FLAGS {}
impl ::core::clone::Clone for GET_STOCK_OBJECT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_STOCK_OBJECT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_STOCK_OBJECT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_STOCK_OBJECT_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for GET_STOCK_OBJECT_FLAGS {
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
pub const GGI_MARK_NONEXISTING_GLYPHS: u32 = 1u32;
pub struct GLYPHMETRICS {
    pub gmBlackBoxX: u32,
    pub gmBlackBoxY: u32,
    pub gmptGlyphOrigin: super::super::Foundation::POINT,
    pub gmCellIncX: i16,
    pub gmCellIncY: i16,
}
impl ::core::marker::Copy for GLYPHMETRICS {}
impl ::core::clone::Clone for GLYPHMETRICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GLYPHMETRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GLYPHMETRICS")
            .field("gmBlackBoxX", &self.gmBlackBoxX)
            .field("gmBlackBoxY", &self.gmBlackBoxY)
            .field("gmptGlyphOrigin", &self.gmptGlyphOrigin)
            .field("gmCellIncX", &self.gmCellIncX)
            .field("gmCellIncY", &self.gmCellIncY)
            .finish()
    }
}
impl ::core::cmp::PartialEq for GLYPHMETRICS {
    fn eq(&self, other: &Self) -> bool {
        self.gmBlackBoxX == other.gmBlackBoxX
            && self.gmBlackBoxY == other.gmBlackBoxY
            && self.gmptGlyphOrigin == other.gmptGlyphOrigin
            && self.gmCellIncX == other.gmCellIncX
            && self.gmCellIncY == other.gmCellIncY
    }
}
impl ::core::cmp::Eq for GLYPHMETRICS {}
impl FromIntoMemory for GLYPHMETRICS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_gmBlackBoxX = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_gmBlackBoxY = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_gmptGlyphOrigin =
            <super::super::Foundation::POINT as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_gmCellIncX = <i16 as FromIntoMemory>::from_bytes(&from[16..16 + 2]);
        let f_gmCellIncY = <i16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        Self {
            gmBlackBoxX: f_gmBlackBoxX,
            gmBlackBoxY: f_gmBlackBoxY,
            gmptGlyphOrigin: f_gmptGlyphOrigin,
            gmCellIncX: f_gmCellIncX,
            gmCellIncY: f_gmCellIncY,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.gmBlackBoxX, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.gmBlackBoxY, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.gmptGlyphOrigin, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.gmCellIncX, &mut into[16..16 + 2]);
        FromIntoMemory::into_bytes(self.gmCellIncY, &mut into[18..18 + 2]);
    }
    fn size() -> usize {
        20
    }
}
pub struct GLYPHSET {
    pub cbThis: u32,
    pub flAccel: u32,
    pub cGlyphsSupported: u32,
    pub cRanges: u32,
    pub ranges: [WCRANGE; 1],
}
impl ::core::marker::Copy for GLYPHSET {}
impl ::core::clone::Clone for GLYPHSET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GLYPHSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GLYPHSET")
            .field("cbThis", &self.cbThis)
            .field("flAccel", &self.flAccel)
            .field("cGlyphsSupported", &self.cGlyphsSupported)
            .field("cRanges", &self.cRanges)
            .field("ranges", &self.ranges)
            .finish()
    }
}
impl ::core::cmp::PartialEq for GLYPHSET {
    fn eq(&self, other: &Self) -> bool {
        self.cbThis == other.cbThis
            && self.flAccel == other.flAccel
            && self.cGlyphsSupported == other.cGlyphsSupported
            && self.cRanges == other.cRanges
            && self.ranges == other.ranges
    }
}
impl ::core::cmp::Eq for GLYPHSET {}
impl FromIntoMemory for GLYPHSET {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_cbThis = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_flAccel = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_cGlyphsSupported = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_cRanges = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_ranges = <[WCRANGE; 1] as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            cbThis: f_cbThis,
            flAccel: f_flAccel,
            cGlyphsSupported: f_cGlyphsSupported,
            cRanges: f_cRanges,
            ranges: f_ranges,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.cbThis, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.flAccel, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.cGlyphsSupported, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.cRanges, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.ranges, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub const GM_LAST: u32 = 2u32;
pub type GOBJENUMPROC = StdCallFnPtr<
    (
        MutPtr<::core::ffi::c_void>,
        super::super::Foundation::LPARAM,
    ),
    i32,
>;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GRADIENT_FILL(pub u32);
pub const GRADIENT_FILL_RECT_H: GRADIENT_FILL = GRADIENT_FILL(0u32);
pub const GRADIENT_FILL_RECT_V: GRADIENT_FILL = GRADIENT_FILL(1u32);
pub const GRADIENT_FILL_TRIANGLE: GRADIENT_FILL = GRADIENT_FILL(2u32);
impl ::core::marker::Copy for GRADIENT_FILL {}
impl ::core::clone::Clone for GRADIENT_FILL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GRADIENT_FILL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GRADIENT_FILL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GRADIENT_FILL").field(&self.0).finish()
    }
}
impl FromIntoMemory for GRADIENT_FILL {
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
pub const GRADIENT_FILL_OP_FLAG: u32 = 255u32;
pub struct GRADIENT_RECT {
    pub UpperLeft: u32,
    pub LowerRight: u32,
}
impl ::core::marker::Copy for GRADIENT_RECT {}
impl ::core::clone::Clone for GRADIENT_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GRADIENT_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GRADIENT_RECT")
            .field("UpperLeft", &self.UpperLeft)
            .field("LowerRight", &self.LowerRight)
            .finish()
    }
}
impl ::core::cmp::PartialEq for GRADIENT_RECT {
    fn eq(&self, other: &Self) -> bool {
        self.UpperLeft == other.UpperLeft && self.LowerRight == other.LowerRight
    }
}
impl ::core::cmp::Eq for GRADIENT_RECT {}
impl FromIntoMemory for GRADIENT_RECT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_UpperLeft = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_LowerRight = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            UpperLeft: f_UpperLeft,
            LowerRight: f_LowerRight,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.UpperLeft, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.LowerRight, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct GRADIENT_TRIANGLE {
    pub Vertex1: u32,
    pub Vertex2: u32,
    pub Vertex3: u32,
}
impl ::core::marker::Copy for GRADIENT_TRIANGLE {}
impl ::core::clone::Clone for GRADIENT_TRIANGLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GRADIENT_TRIANGLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GRADIENT_TRIANGLE")
            .field("Vertex1", &self.Vertex1)
            .field("Vertex2", &self.Vertex2)
            .field("Vertex3", &self.Vertex3)
            .finish()
    }
}
impl ::core::cmp::PartialEq for GRADIENT_TRIANGLE {
    fn eq(&self, other: &Self) -> bool {
        self.Vertex1 == other.Vertex1
            && self.Vertex2 == other.Vertex2
            && self.Vertex3 == other.Vertex3
    }
}
impl ::core::cmp::Eq for GRADIENT_TRIANGLE {}
impl FromIntoMemory for GRADIENT_TRIANGLE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Vertex1 = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Vertex2 = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Vertex3 = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            Vertex1: f_Vertex1,
            Vertex2: f_Vertex2,
            Vertex3: f_Vertex3,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Vertex1, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Vertex2, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Vertex3, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GRAPHICS_MODE(pub u32);
pub const GM_COMPATIBLE: GRAPHICS_MODE = GRAPHICS_MODE(1u32);
pub const GM_ADVANCED: GRAPHICS_MODE = GRAPHICS_MODE(2u32);
impl ::core::marker::Copy for GRAPHICS_MODE {}
impl ::core::clone::Clone for GRAPHICS_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GRAPHICS_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GRAPHICS_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GRAPHICS_MODE").field(&self.0).finish()
    }
}
impl FromIntoMemory for GRAPHICS_MODE {
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
pub type GRAYSTRINGPROC =
    StdCallFnPtr<(HDC, super::super::Foundation::LPARAM, i32), super::super::Foundation::BOOL>;
pub const GREEK_CHARSET: u32 = 161u32;
pub const GS_8BIT_INDICES: u32 = 1u32;
pub struct HANDLETABLE {
    pub objectHandle: [HGDIOBJ; 1],
}
impl ::core::marker::Copy for HANDLETABLE {}
impl ::core::clone::Clone for HANDLETABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HANDLETABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HANDLETABLE")
            .field("objectHandle", &self.objectHandle)
            .finish()
    }
}
impl ::core::cmp::PartialEq for HANDLETABLE {
    fn eq(&self, other: &Self) -> bool {
        self.objectHandle == other.objectHandle
    }
}
impl ::core::cmp::Eq for HANDLETABLE {}
impl FromIntoMemory for HANDLETABLE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_objectHandle = <[HGDIOBJ; 1] as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            objectHandle: f_objectHandle,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.objectHandle, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub const HANGEUL_CHARSET: u32 = 129u32;
pub const HANGUL_CHARSET: u32 = 129u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HATCH_BRUSH_STYLE(pub u32);
pub const HS_BDIAGONAL: HATCH_BRUSH_STYLE = HATCH_BRUSH_STYLE(3u32);
pub const HS_CROSS: HATCH_BRUSH_STYLE = HATCH_BRUSH_STYLE(4u32);
pub const HS_DIAGCROSS: HATCH_BRUSH_STYLE = HATCH_BRUSH_STYLE(5u32);
pub const HS_FDIAGONAL: HATCH_BRUSH_STYLE = HATCH_BRUSH_STYLE(2u32);
pub const HS_HORIZONTAL: HATCH_BRUSH_STYLE = HATCH_BRUSH_STYLE(0u32);
pub const HS_VERTICAL: HATCH_BRUSH_STYLE = HATCH_BRUSH_STYLE(1u32);
impl ::core::marker::Copy for HATCH_BRUSH_STYLE {}
impl ::core::clone::Clone for HATCH_BRUSH_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HATCH_BRUSH_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HATCH_BRUSH_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HATCH_BRUSH_STYLE").field(&self.0).finish()
    }
}
impl FromIntoMemory for HATCH_BRUSH_STYLE {
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
pub struct HBITMAP(pub PtrDiffRepr);
impl HBITMAP {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HBITMAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HBITMAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HBITMAP {}
impl ::core::hash::Hash for HBITMAP {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HBITMAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HBITMAP").field(&self.0).finish()
    }
}
impl FromIntoMemory for HBITMAP {
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
pub struct HBRUSH(pub PtrDiffRepr);
impl HBRUSH {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HBRUSH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HBRUSH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HBRUSH {}
impl ::core::hash::Hash for HBRUSH {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HBRUSH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HBRUSH").field(&self.0).finish()
    }
}
impl FromIntoMemory for HBRUSH {
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
pub struct HDC(pub PtrDiffRepr);
impl HDC {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HDC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HDC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HDC {}
impl ::core::hash::Hash for HDC {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HDC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDC").field(&self.0).finish()
    }
}
impl FromIntoMemory for HDC {
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
pub struct HDC_MAP_MODE(pub u32);
pub const MM_ANISOTROPIC: HDC_MAP_MODE = HDC_MAP_MODE(8u32);
pub const MM_HIENGLISH: HDC_MAP_MODE = HDC_MAP_MODE(5u32);
pub const MM_HIMETRIC: HDC_MAP_MODE = HDC_MAP_MODE(3u32);
pub const MM_ISOTROPIC: HDC_MAP_MODE = HDC_MAP_MODE(7u32);
pub const MM_LOENGLISH: HDC_MAP_MODE = HDC_MAP_MODE(4u32);
pub const MM_LOMETRIC: HDC_MAP_MODE = HDC_MAP_MODE(2u32);
pub const MM_TEXT: HDC_MAP_MODE = HDC_MAP_MODE(1u32);
pub const MM_TWIPS: HDC_MAP_MODE = HDC_MAP_MODE(6u32);
impl ::core::marker::Copy for HDC_MAP_MODE {}
impl ::core::clone::Clone for HDC_MAP_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HDC_MAP_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HDC_MAP_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDC_MAP_MODE").field(&self.0).finish()
    }
}
impl FromIntoMemory for HDC_MAP_MODE {
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
pub const HEBREW_CHARSET: u32 = 177u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HENHMETAFILE(pub PtrDiffRepr);
impl HENHMETAFILE {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HENHMETAFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HENHMETAFILE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HENHMETAFILE {}
impl ::core::hash::Hash for HENHMETAFILE {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HENHMETAFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HENHMETAFILE").field(&self.0).finish()
    }
}
impl FromIntoMemory for HENHMETAFILE {
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
pub struct HFONT(pub PtrDiffRepr);
impl HFONT {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HFONT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HFONT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HFONT {}
impl ::core::hash::Hash for HFONT {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HFONT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HFONT").field(&self.0).finish()
    }
}
impl FromIntoMemory for HFONT {
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
pub struct HGDIOBJ(pub PtrDiffRepr);
impl HGDIOBJ {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HGDIOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HGDIOBJ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HGDIOBJ {}
impl ::core::hash::Hash for HGDIOBJ {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HGDIOBJ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HGDIOBJ").field(&self.0).finish()
    }
}
impl FromIntoMemory for HGDIOBJ {
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
pub struct HMETAFILE(pub PtrDiffRepr);
impl HMETAFILE {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HMETAFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HMETAFILE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HMETAFILE {}
impl ::core::hash::Hash for HMETAFILE {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HMETAFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HMETAFILE").field(&self.0).finish()
    }
}
impl FromIntoMemory for HMETAFILE {
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
pub struct HMONITOR(pub PtrDiffRepr);
impl HMONITOR {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HMONITOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HMONITOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HMONITOR {}
impl ::core::hash::Hash for HMONITOR {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HMONITOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HMONITOR").field(&self.0).finish()
    }
}
impl FromIntoMemory for HMONITOR {
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
pub struct HPALETTE(pub PtrDiffRepr);
impl HPALETTE {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HPALETTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HPALETTE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HPALETTE {}
impl ::core::hash::Hash for HPALETTE {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HPALETTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HPALETTE").field(&self.0).finish()
    }
}
impl FromIntoMemory for HPALETTE {
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
pub struct HPEN(pub PtrDiffRepr);
impl HPEN {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HPEN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HPEN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HPEN {}
impl ::core::hash::Hash for HPEN {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HPEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HPEN").field(&self.0).finish()
    }
}
impl FromIntoMemory for HPEN {
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
pub struct HRGN(pub PtrDiffRepr);
impl HRGN {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HRGN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HRGN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HRGN {}
impl ::core::hash::Hash for HRGN {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HRGN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HRGN").field(&self.0).finish()
    }
}
impl FromIntoMemory for HRGN {
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
pub const HS_API_MAX: u32 = 12u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HdcMetdataEnhFileHandle(pub PtrDiffRepr);
impl HdcMetdataEnhFileHandle {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HdcMetdataEnhFileHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HdcMetdataEnhFileHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HdcMetdataEnhFileHandle {}
impl ::core::hash::Hash for HdcMetdataEnhFileHandle {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HdcMetdataEnhFileHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdcMetdataEnhFileHandle")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for HdcMetdataEnhFileHandle {
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
pub struct HdcMetdataFileHandle(pub PtrDiffRepr);
impl HdcMetdataFileHandle {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HdcMetdataFileHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HdcMetdataFileHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HdcMetdataFileHandle {}
impl ::core::hash::Hash for HdcMetdataFileHandle {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HdcMetdataFileHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdcMetdataFileHandle")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for HdcMetdataFileHandle {
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
pub const ICM_DONE_OUTSIDEDC: u32 = 4u32;
pub const ICM_OFF: u32 = 1u32;
pub const ICM_ON: u32 = 2u32;
pub const ICM_QUERY: u32 = 3u32;
pub const ILLUMINANT_A: u32 = 1u32;
pub const ILLUMINANT_B: u32 = 2u32;
pub const ILLUMINANT_C: u32 = 3u32;
pub const ILLUMINANT_D50: u32 = 4u32;
pub const ILLUMINANT_D55: u32 = 5u32;
pub const ILLUMINANT_D65: u32 = 6u32;
pub const ILLUMINANT_D75: u32 = 7u32;
pub const ILLUMINANT_DAYLIGHT: u32 = 3u32;
pub const ILLUMINANT_DEVICE_DEFAULT: u32 = 0u32;
pub const ILLUMINANT_F2: u32 = 8u32;
pub const ILLUMINANT_FLUORESCENT: u32 = 8u32;
pub const ILLUMINANT_MAX_INDEX: u32 = 8u32;
pub const ILLUMINANT_NTSC: u32 = 3u32;
pub const ILLUMINANT_TUNGSTEN: u32 = 1u32;
pub const JOHAB_CHARSET: u32 = 130u32;
pub struct KERNINGPAIR {
    pub wFirst: u16,
    pub wSecond: u16,
    pub iKernAmount: i32,
}
impl ::core::marker::Copy for KERNINGPAIR {}
impl ::core::clone::Clone for KERNINGPAIR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KERNINGPAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERNINGPAIR")
            .field("wFirst", &self.wFirst)
            .field("wSecond", &self.wSecond)
            .field("iKernAmount", &self.iKernAmount)
            .finish()
    }
}
impl ::core::cmp::PartialEq for KERNINGPAIR {
    fn eq(&self, other: &Self) -> bool {
        self.wFirst == other.wFirst
            && self.wSecond == other.wSecond
            && self.iKernAmount == other.iKernAmount
    }
}
impl ::core::cmp::Eq for KERNINGPAIR {}
impl FromIntoMemory for KERNINGPAIR {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_wFirst = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wSecond = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_iKernAmount = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            wFirst: f_wFirst,
            wSecond: f_wSecond,
            iKernAmount: f_iKernAmount,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.wFirst, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wSecond, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.iKernAmount, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub const LAYOUT_BTT: u32 = 2u32;
pub const LAYOUT_VBH: u32 = 4u32;
pub const LCS_CALIBRATED_RGB: i32 = 0i32;
pub const LCS_GM_ABS_COLORIMETRIC: i32 = 8i32;
pub const LCS_GM_BUSINESS: i32 = 1i32;
pub const LCS_GM_GRAPHICS: i32 = 2i32;
pub const LCS_GM_IMAGES: i32 = 4i32;
pub const LC_INTERIORS: u32 = 128u32;
pub const LC_MARKER: u32 = 4u32;
pub const LC_NONE: u32 = 0u32;
pub const LC_POLYLINE: u32 = 2u32;
pub const LC_POLYMARKER: u32 = 8u32;
pub const LC_STYLED: u32 = 32u32;
pub const LC_WIDE: u32 = 16u32;
pub const LC_WIDESTYLED: u32 = 64u32;
pub const LF_FACESIZE: u32 = 32u32;
pub const LF_FULLFACESIZE: u32 = 64u32;
pub type LINEDDAPROC = StdCallFnPtr<(i32, i32, super::super::Foundation::LPARAM), ()>;
pub struct LOGBRUSH {
    pub lbStyle: u32,
    pub lbColor: u32,
    pub lbHatch: PtrRepr,
}
impl ::core::marker::Copy for LOGBRUSH {}
impl ::core::clone::Clone for LOGBRUSH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LOGBRUSH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOGBRUSH")
            .field("lbStyle", &self.lbStyle)
            .field("lbColor", &self.lbColor)
            .field("lbHatch", &self.lbHatch)
            .finish()
    }
}
impl ::core::cmp::PartialEq for LOGBRUSH {
    fn eq(&self, other: &Self) -> bool {
        self.lbStyle == other.lbStyle
            && self.lbColor == other.lbColor
            && self.lbHatch == other.lbHatch
    }
}
impl ::core::cmp::Eq for LOGBRUSH {}
impl FromIntoMemory for LOGBRUSH {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_lbStyle = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lbColor = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lbHatch = <PtrRepr as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            lbStyle: f_lbStyle,
            lbColor: f_lbColor,
            lbHatch: f_lbHatch,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.lbStyle, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lbColor, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lbHatch, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct LOGBRUSH32 {
    pub lbStyle: u32,
    pub lbColor: u32,
    pub lbHatch: u32,
}
impl ::core::marker::Copy for LOGBRUSH32 {}
impl ::core::clone::Clone for LOGBRUSH32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LOGBRUSH32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOGBRUSH32")
            .field("lbStyle", &self.lbStyle)
            .field("lbColor", &self.lbColor)
            .field("lbHatch", &self.lbHatch)
            .finish()
    }
}
impl ::core::cmp::PartialEq for LOGBRUSH32 {
    fn eq(&self, other: &Self) -> bool {
        self.lbStyle == other.lbStyle
            && self.lbColor == other.lbColor
            && self.lbHatch == other.lbHatch
    }
}
impl ::core::cmp::Eq for LOGBRUSH32 {}
impl FromIntoMemory for LOGBRUSH32 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_lbStyle = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lbColor = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lbHatch = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            lbStyle: f_lbStyle,
            lbColor: f_lbColor,
            lbHatch: f_lbHatch,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.lbStyle, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lbColor, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lbHatch, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct LOGFONTA {
    pub lfHeight: i32,
    pub lfWidth: i32,
    pub lfEscapement: i32,
    pub lfOrientation: i32,
    pub lfWeight: i32,
    pub lfItalic: u8,
    pub lfUnderline: u8,
    pub lfStrikeOut: u8,
    pub lfCharSet: u8,
    pub lfOutPrecision: u8,
    pub lfClipPrecision: u8,
    pub lfQuality: u8,
    pub lfPitchAndFamily: u8,
    pub lfFaceName: [super::super::Foundation::CHAR; 32],
}
impl ::core::marker::Copy for LOGFONTA {}
impl ::core::clone::Clone for LOGFONTA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LOGFONTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOGFONTA")
            .field("lfHeight", &self.lfHeight)
            .field("lfWidth", &self.lfWidth)
            .field("lfEscapement", &self.lfEscapement)
            .field("lfOrientation", &self.lfOrientation)
            .field("lfWeight", &self.lfWeight)
            .field("lfItalic", &self.lfItalic)
            .field("lfUnderline", &self.lfUnderline)
            .field("lfStrikeOut", &self.lfStrikeOut)
            .field("lfCharSet", &self.lfCharSet)
            .field("lfOutPrecision", &self.lfOutPrecision)
            .field("lfClipPrecision", &self.lfClipPrecision)
            .field("lfQuality", &self.lfQuality)
            .field("lfPitchAndFamily", &self.lfPitchAndFamily)
            .field("lfFaceName", &self.lfFaceName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for LOGFONTA {
    fn eq(&self, other: &Self) -> bool {
        self.lfHeight == other.lfHeight
            && self.lfWidth == other.lfWidth
            && self.lfEscapement == other.lfEscapement
            && self.lfOrientation == other.lfOrientation
            && self.lfWeight == other.lfWeight
            && self.lfItalic == other.lfItalic
            && self.lfUnderline == other.lfUnderline
            && self.lfStrikeOut == other.lfStrikeOut
            && self.lfCharSet == other.lfCharSet
            && self.lfOutPrecision == other.lfOutPrecision
            && self.lfClipPrecision == other.lfClipPrecision
            && self.lfQuality == other.lfQuality
            && self.lfPitchAndFamily == other.lfPitchAndFamily
            && self.lfFaceName == other.lfFaceName
    }
}
impl ::core::cmp::Eq for LOGFONTA {}
impl FromIntoMemory for LOGFONTA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 60);
        let f_lfHeight = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lfWidth = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lfEscapement = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lfOrientation = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lfWeight = <i32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_lfItalic = <u8 as FromIntoMemory>::from_bytes(&from[20..20 + 1]);
        let f_lfUnderline = <u8 as FromIntoMemory>::from_bytes(&from[21..21 + 1]);
        let f_lfStrikeOut = <u8 as FromIntoMemory>::from_bytes(&from[22..22 + 1]);
        let f_lfCharSet = <u8 as FromIntoMemory>::from_bytes(&from[23..23 + 1]);
        let f_lfOutPrecision = <u8 as FromIntoMemory>::from_bytes(&from[24..24 + 1]);
        let f_lfClipPrecision = <u8 as FromIntoMemory>::from_bytes(&from[25..25 + 1]);
        let f_lfQuality = <u8 as FromIntoMemory>::from_bytes(&from[26..26 + 1]);
        let f_lfPitchAndFamily = <u8 as FromIntoMemory>::from_bytes(&from[27..27 + 1]);
        let f_lfFaceName = <[super::super::Foundation::CHAR; 32] as FromIntoMemory>::from_bytes(
            &from[28..28 + 32],
        );
        Self {
            lfHeight: f_lfHeight,
            lfWidth: f_lfWidth,
            lfEscapement: f_lfEscapement,
            lfOrientation: f_lfOrientation,
            lfWeight: f_lfWeight,
            lfItalic: f_lfItalic,
            lfUnderline: f_lfUnderline,
            lfStrikeOut: f_lfStrikeOut,
            lfCharSet: f_lfCharSet,
            lfOutPrecision: f_lfOutPrecision,
            lfClipPrecision: f_lfClipPrecision,
            lfQuality: f_lfQuality,
            lfPitchAndFamily: f_lfPitchAndFamily,
            lfFaceName: f_lfFaceName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 60);
        FromIntoMemory::into_bytes(self.lfHeight, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lfWidth, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lfEscapement, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lfOrientation, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lfWeight, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.lfItalic, &mut into[20..20 + 1]);
        FromIntoMemory::into_bytes(self.lfUnderline, &mut into[21..21 + 1]);
        FromIntoMemory::into_bytes(self.lfStrikeOut, &mut into[22..22 + 1]);
        FromIntoMemory::into_bytes(self.lfCharSet, &mut into[23..23 + 1]);
        FromIntoMemory::into_bytes(self.lfOutPrecision, &mut into[24..24 + 1]);
        FromIntoMemory::into_bytes(self.lfClipPrecision, &mut into[25..25 + 1]);
        FromIntoMemory::into_bytes(self.lfQuality, &mut into[26..26 + 1]);
        FromIntoMemory::into_bytes(self.lfPitchAndFamily, &mut into[27..27 + 1]);
        FromIntoMemory::into_bytes(self.lfFaceName, &mut into[28..28 + 32]);
    }
    fn size() -> usize {
        60
    }
}
pub struct LOGFONTW {
    pub lfHeight: i32,
    pub lfWidth: i32,
    pub lfEscapement: i32,
    pub lfOrientation: i32,
    pub lfWeight: i32,
    pub lfItalic: u8,
    pub lfUnderline: u8,
    pub lfStrikeOut: u8,
    pub lfCharSet: u8,
    pub lfOutPrecision: u8,
    pub lfClipPrecision: u8,
    pub lfQuality: u8,
    pub lfPitchAndFamily: u8,
    pub lfFaceName: [u16; 32],
}
impl ::core::marker::Copy for LOGFONTW {}
impl ::core::clone::Clone for LOGFONTW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LOGFONTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOGFONTW")
            .field("lfHeight", &self.lfHeight)
            .field("lfWidth", &self.lfWidth)
            .field("lfEscapement", &self.lfEscapement)
            .field("lfOrientation", &self.lfOrientation)
            .field("lfWeight", &self.lfWeight)
            .field("lfItalic", &self.lfItalic)
            .field("lfUnderline", &self.lfUnderline)
            .field("lfStrikeOut", &self.lfStrikeOut)
            .field("lfCharSet", &self.lfCharSet)
            .field("lfOutPrecision", &self.lfOutPrecision)
            .field("lfClipPrecision", &self.lfClipPrecision)
            .field("lfQuality", &self.lfQuality)
            .field("lfPitchAndFamily", &self.lfPitchAndFamily)
            .field("lfFaceName", &self.lfFaceName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for LOGFONTW {
    fn eq(&self, other: &Self) -> bool {
        self.lfHeight == other.lfHeight
            && self.lfWidth == other.lfWidth
            && self.lfEscapement == other.lfEscapement
            && self.lfOrientation == other.lfOrientation
            && self.lfWeight == other.lfWeight
            && self.lfItalic == other.lfItalic
            && self.lfUnderline == other.lfUnderline
            && self.lfStrikeOut == other.lfStrikeOut
            && self.lfCharSet == other.lfCharSet
            && self.lfOutPrecision == other.lfOutPrecision
            && self.lfClipPrecision == other.lfClipPrecision
            && self.lfQuality == other.lfQuality
            && self.lfPitchAndFamily == other.lfPitchAndFamily
            && self.lfFaceName == other.lfFaceName
    }
}
impl ::core::cmp::Eq for LOGFONTW {}
impl FromIntoMemory for LOGFONTW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 60);
        let f_lfHeight = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lfWidth = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lfEscapement = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lfOrientation = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lfWeight = <i32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_lfItalic = <u8 as FromIntoMemory>::from_bytes(&from[20..20 + 1]);
        let f_lfUnderline = <u8 as FromIntoMemory>::from_bytes(&from[21..21 + 1]);
        let f_lfStrikeOut = <u8 as FromIntoMemory>::from_bytes(&from[22..22 + 1]);
        let f_lfCharSet = <u8 as FromIntoMemory>::from_bytes(&from[23..23 + 1]);
        let f_lfOutPrecision = <u8 as FromIntoMemory>::from_bytes(&from[24..24 + 1]);
        let f_lfClipPrecision = <u8 as FromIntoMemory>::from_bytes(&from[25..25 + 1]);
        let f_lfQuality = <u8 as FromIntoMemory>::from_bytes(&from[26..26 + 1]);
        let f_lfPitchAndFamily = <u8 as FromIntoMemory>::from_bytes(&from[27..27 + 1]);
        let f_lfFaceName = <[u16; 32] as FromIntoMemory>::from_bytes(&from[28..28 + 32]);
        Self {
            lfHeight: f_lfHeight,
            lfWidth: f_lfWidth,
            lfEscapement: f_lfEscapement,
            lfOrientation: f_lfOrientation,
            lfWeight: f_lfWeight,
            lfItalic: f_lfItalic,
            lfUnderline: f_lfUnderline,
            lfStrikeOut: f_lfStrikeOut,
            lfCharSet: f_lfCharSet,
            lfOutPrecision: f_lfOutPrecision,
            lfClipPrecision: f_lfClipPrecision,
            lfQuality: f_lfQuality,
            lfPitchAndFamily: f_lfPitchAndFamily,
            lfFaceName: f_lfFaceName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 60);
        FromIntoMemory::into_bytes(self.lfHeight, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lfWidth, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lfEscapement, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lfOrientation, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lfWeight, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.lfItalic, &mut into[20..20 + 1]);
        FromIntoMemory::into_bytes(self.lfUnderline, &mut into[21..21 + 1]);
        FromIntoMemory::into_bytes(self.lfStrikeOut, &mut into[22..22 + 1]);
        FromIntoMemory::into_bytes(self.lfCharSet, &mut into[23..23 + 1]);
        FromIntoMemory::into_bytes(self.lfOutPrecision, &mut into[24..24 + 1]);
        FromIntoMemory::into_bytes(self.lfClipPrecision, &mut into[25..25 + 1]);
        FromIntoMemory::into_bytes(self.lfQuality, &mut into[26..26 + 1]);
        FromIntoMemory::into_bytes(self.lfPitchAndFamily, &mut into[27..27 + 1]);
        FromIntoMemory::into_bytes(self.lfFaceName, &mut into[28..28 + 32]);
    }
    fn size() -> usize {
        60
    }
}
pub struct LOGPALETTE {
    pub palVersion: u16,
    pub palNumEntries: u16,
    pub palPalEntry: [PALETTEENTRY; 1],
}
impl ::core::marker::Copy for LOGPALETTE {}
impl ::core::clone::Clone for LOGPALETTE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LOGPALETTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOGPALETTE")
            .field("palVersion", &self.palVersion)
            .field("palNumEntries", &self.palNumEntries)
            .field("palPalEntry", &self.palPalEntry)
            .finish()
    }
}
impl ::core::cmp::PartialEq for LOGPALETTE {
    fn eq(&self, other: &Self) -> bool {
        self.palVersion == other.palVersion
            && self.palNumEntries == other.palNumEntries
            && self.palPalEntry == other.palPalEntry
    }
}
impl ::core::cmp::Eq for LOGPALETTE {}
impl FromIntoMemory for LOGPALETTE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_palVersion = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_palNumEntries = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_palPalEntry = <[PALETTEENTRY; 1] as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            palVersion: f_palVersion,
            palNumEntries: f_palNumEntries,
            palPalEntry: f_palPalEntry,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.palVersion, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.palNumEntries, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.palPalEntry, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct LOGPEN {
    pub lopnStyle: u32,
    pub lopnWidth: super::super::Foundation::POINT,
    pub lopnColor: u32,
}
impl ::core::marker::Copy for LOGPEN {}
impl ::core::clone::Clone for LOGPEN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LOGPEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOGPEN")
            .field("lopnStyle", &self.lopnStyle)
            .field("lopnWidth", &self.lopnWidth)
            .field("lopnColor", &self.lopnColor)
            .finish()
    }
}
impl ::core::cmp::PartialEq for LOGPEN {
    fn eq(&self, other: &Self) -> bool {
        self.lopnStyle == other.lopnStyle
            && self.lopnWidth == other.lopnWidth
            && self.lopnColor == other.lopnColor
    }
}
impl ::core::cmp::Eq for LOGPEN {}
impl FromIntoMemory for LOGPEN {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_lopnStyle = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lopnWidth =
            <super::super::Foundation::POINT as FromIntoMemory>::from_bytes(&from[4..4 + 8]);
        let f_lopnColor = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            lopnStyle: f_lopnStyle,
            lopnWidth: f_lopnWidth,
            lopnColor: f_lopnColor,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.lopnStyle, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lopnWidth, &mut into[4..4 + 8]);
        FromIntoMemory::into_bytes(self.lopnColor, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub const LPD_DOUBLEBUFFER: u32 = 1u32;
pub const LPD_SHARE_ACCUM: u32 = 256u32;
pub const LPD_SHARE_DEPTH: u32 = 64u32;
pub const LPD_SHARE_STENCIL: u32 = 128u32;
pub const LPD_STEREO: u32 = 2u32;
pub const LPD_SUPPORT_GDI: u32 = 16u32;
pub const LPD_SUPPORT_OPENGL: u32 = 32u32;
pub const LPD_SWAP_COPY: u32 = 1024u32;
pub const LPD_SWAP_EXCHANGE: u32 = 512u32;
pub const LPD_TRANSPARENT: u32 = 4096u32;
pub const LPD_TYPE_COLORINDEX: u32 = 1u32;
pub const LPD_TYPE_RGBA: u32 = 0u32;
pub type LPFNDEVCAPS = StdCallFnPtr<(PCSTR, PCSTR, u32, PCSTR, MutPtr<DEVMODEA>), u32>;
pub type LPFNDEVMODE = StdCallFnPtr<
    (
        super::super::Foundation::HWND,
        super::super::Foundation::HINSTANCE,
        MutPtr<DEVMODEA>,
        PCSTR,
        PCSTR,
        MutPtr<DEVMODEA>,
        PCSTR,
        u32,
    ),
    u32,
>;
pub const MAC_CHARSET: u32 = 77u32;
pub struct MAT2 {
    pub eM11: FIXED,
    pub eM12: FIXED,
    pub eM21: FIXED,
    pub eM22: FIXED,
}
impl ::core::marker::Copy for MAT2 {}
impl ::core::clone::Clone for MAT2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MAT2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MAT2")
            .field("eM11", &self.eM11)
            .field("eM12", &self.eM12)
            .field("eM21", &self.eM21)
            .field("eM22", &self.eM22)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MAT2 {
    fn eq(&self, other: &Self) -> bool {
        self.eM11 == other.eM11
            && self.eM12 == other.eM12
            && self.eM21 == other.eM21
            && self.eM22 == other.eM22
    }
}
impl ::core::cmp::Eq for MAT2 {}
impl FromIntoMemory for MAT2 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_eM11 = <FIXED as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_eM12 = <FIXED as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_eM21 = <FIXED as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_eM22 = <FIXED as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            eM11: f_eM11,
            eM12: f_eM12,
            eM21: f_eM21,
            eM22: f_eM22,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.eM11, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.eM12, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.eM21, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.eM22, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub const MAXSTRETCHBLTMODE: u32 = 4u32;
pub const METAFILE_DRIVER: u32 = 2049u32;
pub struct METARECORD {
    pub rdSize: u32,
    pub rdFunction: u16,
    pub rdParm: [u16; 1],
}
impl ::core::marker::Copy for METARECORD {}
impl ::core::clone::Clone for METARECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for METARECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("METARECORD")
            .field("rdSize", &self.rdSize)
            .field("rdFunction", &self.rdFunction)
            .field("rdParm", &self.rdParm)
            .finish()
    }
}
impl ::core::cmp::PartialEq for METARECORD {
    fn eq(&self, other: &Self) -> bool {
        self.rdSize == other.rdSize
            && self.rdFunction == other.rdFunction
            && self.rdParm == other.rdParm
    }
}
impl ::core::cmp::Eq for METARECORD {}
impl FromIntoMemory for METARECORD {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_rdSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_rdFunction = <u16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_rdParm = <[u16; 1] as FromIntoMemory>::from_bytes(&from[6..6 + 2]);
        Self {
            rdSize: f_rdSize,
            rdFunction: f_rdFunction,
            rdParm: f_rdParm,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.rdSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.rdFunction, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.rdParm, &mut into[6..6 + 2]);
    }
    fn size() -> usize {
        8
    }
}
pub const META_ANIMATEPALETTE: u32 = 1078u32;
pub const META_ARC: u32 = 2071u32;
pub const META_BITBLT: u32 = 2338u32;
pub const META_CHORD: u32 = 2096u32;
pub const META_CREATEBRUSHINDIRECT: u32 = 764u32;
pub const META_CREATEFONTINDIRECT: u32 = 763u32;
pub const META_CREATEPALETTE: u32 = 247u32;
pub const META_CREATEPATTERNBRUSH: u32 = 505u32;
pub const META_CREATEPENINDIRECT: u32 = 762u32;
pub const META_CREATEREGION: u32 = 1791u32;
pub const META_DELETEOBJECT: u32 = 496u32;
pub const META_DIBBITBLT: u32 = 2368u32;
pub const META_DIBCREATEPATTERNBRUSH: u32 = 322u32;
pub const META_DIBSTRETCHBLT: u32 = 2881u32;
pub const META_ELLIPSE: u32 = 1048u32;
pub const META_ESCAPE: u32 = 1574u32;
pub const META_EXCLUDECLIPRECT: u32 = 1045u32;
pub const META_EXTFLOODFILL: u32 = 1352u32;
pub const META_EXTTEXTOUT: u32 = 2610u32;
pub const META_FILLREGION: u32 = 552u32;
pub const META_FLOODFILL: u32 = 1049u32;
pub const META_FRAMEREGION: u32 = 1065u32;
pub const META_INTERSECTCLIPRECT: u32 = 1046u32;
pub const META_INVERTREGION: u32 = 298u32;
pub const META_LINETO: u32 = 531u32;
pub const META_MOVETO: u32 = 532u32;
pub const META_OFFSETCLIPRGN: u32 = 544u32;
pub const META_OFFSETVIEWPORTORG: u32 = 529u32;
pub const META_OFFSETWINDOWORG: u32 = 527u32;
pub const META_PAINTREGION: u32 = 299u32;
pub const META_PATBLT: u32 = 1565u32;
pub const META_PIE: u32 = 2074u32;
pub const META_POLYGON: u32 = 804u32;
pub const META_POLYLINE: u32 = 805u32;
pub const META_POLYPOLYGON: u32 = 1336u32;
pub const META_REALIZEPALETTE: u32 = 53u32;
pub const META_RECTANGLE: u32 = 1051u32;
pub const META_RESIZEPALETTE: u32 = 313u32;
pub const META_RESTOREDC: u32 = 295u32;
pub const META_ROUNDRECT: u32 = 1564u32;
pub const META_SAVEDC: u32 = 30u32;
pub const META_SCALEVIEWPORTEXT: u32 = 1042u32;
pub const META_SCALEWINDOWEXT: u32 = 1040u32;
pub const META_SELECTCLIPREGION: u32 = 300u32;
pub const META_SELECTOBJECT: u32 = 301u32;
pub const META_SELECTPALETTE: u32 = 564u32;
pub const META_SETBKCOLOR: u32 = 513u32;
pub const META_SETBKMODE: u32 = 258u32;
pub const META_SETDIBTODEV: u32 = 3379u32;
pub const META_SETLAYOUT: u32 = 329u32;
pub const META_SETMAPMODE: u32 = 259u32;
pub const META_SETMAPPERFLAGS: u32 = 561u32;
pub const META_SETPALENTRIES: u32 = 55u32;
pub const META_SETPIXEL: u32 = 1055u32;
pub const META_SETPOLYFILLMODE: u32 = 262u32;
pub const META_SETRELABS: u32 = 261u32;
pub const META_SETROP2: u32 = 260u32;
pub const META_SETSTRETCHBLTMODE: u32 = 263u32;
pub const META_SETTEXTALIGN: u32 = 302u32;
pub const META_SETTEXTCHAREXTRA: u32 = 264u32;
pub const META_SETTEXTCOLOR: u32 = 521u32;
pub const META_SETTEXTJUSTIFICATION: u32 = 522u32;
pub const META_SETVIEWPORTEXT: u32 = 526u32;
pub const META_SETVIEWPORTORG: u32 = 525u32;
pub const META_SETWINDOWEXT: u32 = 524u32;
pub const META_SETWINDOWORG: u32 = 523u32;
pub const META_STRETCHBLT: u32 = 2851u32;
pub const META_STRETCHDIB: u32 = 3907u32;
pub const META_TEXTOUT: u32 = 1313u32;
pub const MFCOMMENT: u32 = 15u32;
pub type MFENUMPROC = StdCallFnPtr<
    (
        HDC,
        ConstPtr<HANDLETABLE>,
        ConstPtr<METARECORD>,
        i32,
        super::super::Foundation::LPARAM,
    ),
    i32,
>;
pub const MILCORE_TS_QUERYVER_RESULT_FALSE: u32 = 0u32;
pub const MILCORE_TS_QUERYVER_RESULT_TRUE: u32 = 2147483647u32;
pub const MM_MAX_AXES_NAMELEN: u32 = 16u32;
pub const MM_MAX_NUMAXES: u32 = 16u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MODIFY_WORLD_TRANSFORM_MODE(pub u32);
pub const MWT_IDENTITY: MODIFY_WORLD_TRANSFORM_MODE = MODIFY_WORLD_TRANSFORM_MODE(1u32);
pub const MWT_LEFTMULTIPLY: MODIFY_WORLD_TRANSFORM_MODE = MODIFY_WORLD_TRANSFORM_MODE(2u32);
pub const MWT_RIGHTMULTIPLY: MODIFY_WORLD_TRANSFORM_MODE = MODIFY_WORLD_TRANSFORM_MODE(3u32);
impl ::core::marker::Copy for MODIFY_WORLD_TRANSFORM_MODE {}
impl ::core::clone::Clone for MODIFY_WORLD_TRANSFORM_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MODIFY_WORLD_TRANSFORM_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MODIFY_WORLD_TRANSFORM_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MODIFY_WORLD_TRANSFORM_MODE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for MODIFY_WORLD_TRANSFORM_MODE {
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
pub type MONITORENUMPROC = StdCallFnPtr<
    (
        HMONITOR,
        HDC,
        MutPtr<super::super::Foundation::RECT>,
        super::super::Foundation::LPARAM,
    ),
    super::super::Foundation::BOOL,
>;
pub struct MONITORINFO {
    pub cbSize: u32,
    pub rcMonitor: super::super::Foundation::RECT,
    pub rcWork: super::super::Foundation::RECT,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for MONITORINFO {}
impl ::core::clone::Clone for MONITORINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MONITORINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONITORINFO")
            .field("cbSize", &self.cbSize)
            .field("rcMonitor", &self.rcMonitor)
            .field("rcWork", &self.rcWork)
            .field("dwFlags", &self.dwFlags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MONITORINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.rcMonitor == other.rcMonitor
            && self.rcWork == other.rcWork
            && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for MONITORINFO {}
impl FromIntoMemory for MONITORINFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_cbSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_rcMonitor =
            <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[4..4 + 16]);
        let f_rcWork =
            <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[20..20 + 16]);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        Self {
            cbSize: f_cbSize,
            rcMonitor: f_rcMonitor,
            rcWork: f_rcWork,
            dwFlags: f_dwFlags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.cbSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.rcMonitor, &mut into[4..4 + 16]);
        FromIntoMemory::into_bytes(self.rcWork, &mut into[20..20 + 16]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[36..36 + 4]);
    }
    fn size() -> usize {
        40
    }
}
pub struct MONITORINFOEXA {
    pub monitorInfo: MONITORINFO,
    pub szDevice: [super::super::Foundation::CHAR; 32],
}
impl ::core::marker::Copy for MONITORINFOEXA {}
impl ::core::clone::Clone for MONITORINFOEXA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MONITORINFOEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONITORINFOEXA")
            .field("monitorInfo", &self.monitorInfo)
            .field("szDevice", &self.szDevice)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MONITORINFOEXA {
    fn eq(&self, other: &Self) -> bool {
        self.monitorInfo == other.monitorInfo && self.szDevice == other.szDevice
    }
}
impl ::core::cmp::Eq for MONITORINFOEXA {}
impl FromIntoMemory for MONITORINFOEXA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 72);
        let f_monitorInfo = <MONITORINFO as FromIntoMemory>::from_bytes(&from[0..0 + 40]);
        let f_szDevice = <[super::super::Foundation::CHAR; 32] as FromIntoMemory>::from_bytes(
            &from[40..40 + 32],
        );
        Self {
            monitorInfo: f_monitorInfo,
            szDevice: f_szDevice,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 72);
        FromIntoMemory::into_bytes(self.monitorInfo, &mut into[0..0 + 40]);
        FromIntoMemory::into_bytes(self.szDevice, &mut into[40..40 + 32]);
    }
    fn size() -> usize {
        72
    }
}
pub struct MONITORINFOEXW {
    pub monitorInfo: MONITORINFO,
    pub szDevice: [u16; 32],
}
impl ::core::marker::Copy for MONITORINFOEXW {}
impl ::core::clone::Clone for MONITORINFOEXW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MONITORINFOEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONITORINFOEXW")
            .field("monitorInfo", &self.monitorInfo)
            .field("szDevice", &self.szDevice)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MONITORINFOEXW {
    fn eq(&self, other: &Self) -> bool {
        self.monitorInfo == other.monitorInfo && self.szDevice == other.szDevice
    }
}
impl ::core::cmp::Eq for MONITORINFOEXW {}
impl FromIntoMemory for MONITORINFOEXW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 72);
        let f_monitorInfo = <MONITORINFO as FromIntoMemory>::from_bytes(&from[0..0 + 40]);
        let f_szDevice = <[u16; 32] as FromIntoMemory>::from_bytes(&from[40..40 + 32]);
        Self {
            monitorInfo: f_monitorInfo,
            szDevice: f_szDevice,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 72);
        FromIntoMemory::into_bytes(self.monitorInfo, &mut into[0..0 + 40]);
        FromIntoMemory::into_bytes(self.szDevice, &mut into[40..40 + 32]);
    }
    fn size() -> usize {
        72
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MONITOR_FROM_FLAGS(pub u32);
pub const MONITOR_DEFAULTTONEAREST: MONITOR_FROM_FLAGS = MONITOR_FROM_FLAGS(2u32);
pub const MONITOR_DEFAULTTONULL: MONITOR_FROM_FLAGS = MONITOR_FROM_FLAGS(0u32);
pub const MONITOR_DEFAULTTOPRIMARY: MONITOR_FROM_FLAGS = MONITOR_FROM_FLAGS(1u32);
impl ::core::marker::Copy for MONITOR_FROM_FLAGS {}
impl ::core::clone::Clone for MONITOR_FROM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MONITOR_FROM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MONITOR_FROM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MONITOR_FROM_FLAGS").field(&self.0).finish()
    }
}
impl FromIntoMemory for MONITOR_FROM_FLAGS {
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
pub const MONO_FONT: u32 = 8u32;
pub const MOUSETRAILS: u32 = 39u32;
pub const NEWFRAME: u32 = 1u32;
pub struct NEWTEXTMETRICA {
    pub tmHeight: i32,
    pub tmAscent: i32,
    pub tmDescent: i32,
    pub tmInternalLeading: i32,
    pub tmExternalLeading: i32,
    pub tmAveCharWidth: i32,
    pub tmMaxCharWidth: i32,
    pub tmWeight: i32,
    pub tmOverhang: i32,
    pub tmDigitizedAspectX: i32,
    pub tmDigitizedAspectY: i32,
    pub tmFirstChar: u8,
    pub tmLastChar: u8,
    pub tmDefaultChar: u8,
    pub tmBreakChar: u8,
    pub tmItalic: u8,
    pub tmUnderlined: u8,
    pub tmStruckOut: u8,
    pub tmPitchAndFamily: u8,
    pub tmCharSet: u8,
    pub ntmFlags: u32,
    pub ntmSizeEM: u32,
    pub ntmCellHeight: u32,
    pub ntmAvgWidth: u32,
}
impl ::core::marker::Copy for NEWTEXTMETRICA {}
impl ::core::clone::Clone for NEWTEXTMETRICA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NEWTEXTMETRICA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NEWTEXTMETRICA")
            .field("tmHeight", &self.tmHeight)
            .field("tmAscent", &self.tmAscent)
            .field("tmDescent", &self.tmDescent)
            .field("tmInternalLeading", &self.tmInternalLeading)
            .field("tmExternalLeading", &self.tmExternalLeading)
            .field("tmAveCharWidth", &self.tmAveCharWidth)
            .field("tmMaxCharWidth", &self.tmMaxCharWidth)
            .field("tmWeight", &self.tmWeight)
            .field("tmOverhang", &self.tmOverhang)
            .field("tmDigitizedAspectX", &self.tmDigitizedAspectX)
            .field("tmDigitizedAspectY", &self.tmDigitizedAspectY)
            .field("tmFirstChar", &self.tmFirstChar)
            .field("tmLastChar", &self.tmLastChar)
            .field("tmDefaultChar", &self.tmDefaultChar)
            .field("tmBreakChar", &self.tmBreakChar)
            .field("tmItalic", &self.tmItalic)
            .field("tmUnderlined", &self.tmUnderlined)
            .field("tmStruckOut", &self.tmStruckOut)
            .field("tmPitchAndFamily", &self.tmPitchAndFamily)
            .field("tmCharSet", &self.tmCharSet)
            .field("ntmFlags", &self.ntmFlags)
            .field("ntmSizeEM", &self.ntmSizeEM)
            .field("ntmCellHeight", &self.ntmCellHeight)
            .field("ntmAvgWidth", &self.ntmAvgWidth)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NEWTEXTMETRICA {
    fn eq(&self, other: &Self) -> bool {
        self.tmHeight == other.tmHeight
            && self.tmAscent == other.tmAscent
            && self.tmDescent == other.tmDescent
            && self.tmInternalLeading == other.tmInternalLeading
            && self.tmExternalLeading == other.tmExternalLeading
            && self.tmAveCharWidth == other.tmAveCharWidth
            && self.tmMaxCharWidth == other.tmMaxCharWidth
            && self.tmWeight == other.tmWeight
            && self.tmOverhang == other.tmOverhang
            && self.tmDigitizedAspectX == other.tmDigitizedAspectX
            && self.tmDigitizedAspectY == other.tmDigitizedAspectY
            && self.tmFirstChar == other.tmFirstChar
            && self.tmLastChar == other.tmLastChar
            && self.tmDefaultChar == other.tmDefaultChar
            && self.tmBreakChar == other.tmBreakChar
            && self.tmItalic == other.tmItalic
            && self.tmUnderlined == other.tmUnderlined
            && self.tmStruckOut == other.tmStruckOut
            && self.tmPitchAndFamily == other.tmPitchAndFamily
            && self.tmCharSet == other.tmCharSet
            && self.ntmFlags == other.ntmFlags
            && self.ntmSizeEM == other.ntmSizeEM
            && self.ntmCellHeight == other.ntmCellHeight
            && self.ntmAvgWidth == other.ntmAvgWidth
    }
}
impl ::core::cmp::Eq for NEWTEXTMETRICA {}
impl FromIntoMemory for NEWTEXTMETRICA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 72);
        let f_tmHeight = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_tmAscent = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_tmDescent = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_tmInternalLeading = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_tmExternalLeading = <i32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_tmAveCharWidth = <i32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_tmMaxCharWidth = <i32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_tmWeight = <i32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_tmOverhang = <i32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_tmDigitizedAspectX = <i32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_tmDigitizedAspectY = <i32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_tmFirstChar = <u8 as FromIntoMemory>::from_bytes(&from[44..44 + 1]);
        let f_tmLastChar = <u8 as FromIntoMemory>::from_bytes(&from[45..45 + 1]);
        let f_tmDefaultChar = <u8 as FromIntoMemory>::from_bytes(&from[46..46 + 1]);
        let f_tmBreakChar = <u8 as FromIntoMemory>::from_bytes(&from[47..47 + 1]);
        let f_tmItalic = <u8 as FromIntoMemory>::from_bytes(&from[48..48 + 1]);
        let f_tmUnderlined = <u8 as FromIntoMemory>::from_bytes(&from[49..49 + 1]);
        let f_tmStruckOut = <u8 as FromIntoMemory>::from_bytes(&from[50..50 + 1]);
        let f_tmPitchAndFamily = <u8 as FromIntoMemory>::from_bytes(&from[51..51 + 1]);
        let f_tmCharSet = <u8 as FromIntoMemory>::from_bytes(&from[52..52 + 1]);
        let f_ntmFlags = <u32 as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_ntmSizeEM = <u32 as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_ntmCellHeight = <u32 as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        let f_ntmAvgWidth = <u32 as FromIntoMemory>::from_bytes(&from[68..68 + 4]);
        Self {
            tmHeight: f_tmHeight,
            tmAscent: f_tmAscent,
            tmDescent: f_tmDescent,
            tmInternalLeading: f_tmInternalLeading,
            tmExternalLeading: f_tmExternalLeading,
            tmAveCharWidth: f_tmAveCharWidth,
            tmMaxCharWidth: f_tmMaxCharWidth,
            tmWeight: f_tmWeight,
            tmOverhang: f_tmOverhang,
            tmDigitizedAspectX: f_tmDigitizedAspectX,
            tmDigitizedAspectY: f_tmDigitizedAspectY,
            tmFirstChar: f_tmFirstChar,
            tmLastChar: f_tmLastChar,
            tmDefaultChar: f_tmDefaultChar,
            tmBreakChar: f_tmBreakChar,
            tmItalic: f_tmItalic,
            tmUnderlined: f_tmUnderlined,
            tmStruckOut: f_tmStruckOut,
            tmPitchAndFamily: f_tmPitchAndFamily,
            tmCharSet: f_tmCharSet,
            ntmFlags: f_ntmFlags,
            ntmSizeEM: f_ntmSizeEM,
            ntmCellHeight: f_ntmCellHeight,
            ntmAvgWidth: f_ntmAvgWidth,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 72);
        FromIntoMemory::into_bytes(self.tmHeight, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.tmAscent, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.tmDescent, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.tmInternalLeading, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.tmExternalLeading, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.tmAveCharWidth, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.tmMaxCharWidth, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.tmWeight, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.tmOverhang, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.tmDigitizedAspectX, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.tmDigitizedAspectY, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.tmFirstChar, &mut into[44..44 + 1]);
        FromIntoMemory::into_bytes(self.tmLastChar, &mut into[45..45 + 1]);
        FromIntoMemory::into_bytes(self.tmDefaultChar, &mut into[46..46 + 1]);
        FromIntoMemory::into_bytes(self.tmBreakChar, &mut into[47..47 + 1]);
        FromIntoMemory::into_bytes(self.tmItalic, &mut into[48..48 + 1]);
        FromIntoMemory::into_bytes(self.tmUnderlined, &mut into[49..49 + 1]);
        FromIntoMemory::into_bytes(self.tmStruckOut, &mut into[50..50 + 1]);
        FromIntoMemory::into_bytes(self.tmPitchAndFamily, &mut into[51..51 + 1]);
        FromIntoMemory::into_bytes(self.tmCharSet, &mut into[52..52 + 1]);
        FromIntoMemory::into_bytes(self.ntmFlags, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.ntmSizeEM, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.ntmCellHeight, &mut into[64..64 + 4]);
        FromIntoMemory::into_bytes(self.ntmAvgWidth, &mut into[68..68 + 4]);
    }
    fn size() -> usize {
        72
    }
}
pub struct NEWTEXTMETRICW {
    pub tmHeight: i32,
    pub tmAscent: i32,
    pub tmDescent: i32,
    pub tmInternalLeading: i32,
    pub tmExternalLeading: i32,
    pub tmAveCharWidth: i32,
    pub tmMaxCharWidth: i32,
    pub tmWeight: i32,
    pub tmOverhang: i32,
    pub tmDigitizedAspectX: i32,
    pub tmDigitizedAspectY: i32,
    pub tmFirstChar: u16,
    pub tmLastChar: u16,
    pub tmDefaultChar: u16,
    pub tmBreakChar: u16,
    pub tmItalic: u8,
    pub tmUnderlined: u8,
    pub tmStruckOut: u8,
    pub tmPitchAndFamily: u8,
    pub tmCharSet: u8,
    pub ntmFlags: u32,
    pub ntmSizeEM: u32,
    pub ntmCellHeight: u32,
    pub ntmAvgWidth: u32,
}
impl ::core::marker::Copy for NEWTEXTMETRICW {}
impl ::core::clone::Clone for NEWTEXTMETRICW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NEWTEXTMETRICW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NEWTEXTMETRICW")
            .field("tmHeight", &self.tmHeight)
            .field("tmAscent", &self.tmAscent)
            .field("tmDescent", &self.tmDescent)
            .field("tmInternalLeading", &self.tmInternalLeading)
            .field("tmExternalLeading", &self.tmExternalLeading)
            .field("tmAveCharWidth", &self.tmAveCharWidth)
            .field("tmMaxCharWidth", &self.tmMaxCharWidth)
            .field("tmWeight", &self.tmWeight)
            .field("tmOverhang", &self.tmOverhang)
            .field("tmDigitizedAspectX", &self.tmDigitizedAspectX)
            .field("tmDigitizedAspectY", &self.tmDigitizedAspectY)
            .field("tmFirstChar", &self.tmFirstChar)
            .field("tmLastChar", &self.tmLastChar)
            .field("tmDefaultChar", &self.tmDefaultChar)
            .field("tmBreakChar", &self.tmBreakChar)
            .field("tmItalic", &self.tmItalic)
            .field("tmUnderlined", &self.tmUnderlined)
            .field("tmStruckOut", &self.tmStruckOut)
            .field("tmPitchAndFamily", &self.tmPitchAndFamily)
            .field("tmCharSet", &self.tmCharSet)
            .field("ntmFlags", &self.ntmFlags)
            .field("ntmSizeEM", &self.ntmSizeEM)
            .field("ntmCellHeight", &self.ntmCellHeight)
            .field("ntmAvgWidth", &self.ntmAvgWidth)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NEWTEXTMETRICW {
    fn eq(&self, other: &Self) -> bool {
        self.tmHeight == other.tmHeight
            && self.tmAscent == other.tmAscent
            && self.tmDescent == other.tmDescent
            && self.tmInternalLeading == other.tmInternalLeading
            && self.tmExternalLeading == other.tmExternalLeading
            && self.tmAveCharWidth == other.tmAveCharWidth
            && self.tmMaxCharWidth == other.tmMaxCharWidth
            && self.tmWeight == other.tmWeight
            && self.tmOverhang == other.tmOverhang
            && self.tmDigitizedAspectX == other.tmDigitizedAspectX
            && self.tmDigitizedAspectY == other.tmDigitizedAspectY
            && self.tmFirstChar == other.tmFirstChar
            && self.tmLastChar == other.tmLastChar
            && self.tmDefaultChar == other.tmDefaultChar
            && self.tmBreakChar == other.tmBreakChar
            && self.tmItalic == other.tmItalic
            && self.tmUnderlined == other.tmUnderlined
            && self.tmStruckOut == other.tmStruckOut
            && self.tmPitchAndFamily == other.tmPitchAndFamily
            && self.tmCharSet == other.tmCharSet
            && self.ntmFlags == other.ntmFlags
            && self.ntmSizeEM == other.ntmSizeEM
            && self.ntmCellHeight == other.ntmCellHeight
            && self.ntmAvgWidth == other.ntmAvgWidth
    }
}
impl ::core::cmp::Eq for NEWTEXTMETRICW {}
impl FromIntoMemory for NEWTEXTMETRICW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 72);
        let f_tmHeight = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_tmAscent = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_tmDescent = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_tmInternalLeading = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_tmExternalLeading = <i32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_tmAveCharWidth = <i32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_tmMaxCharWidth = <i32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_tmWeight = <i32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_tmOverhang = <i32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_tmDigitizedAspectX = <i32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_tmDigitizedAspectY = <i32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_tmFirstChar = <u16 as FromIntoMemory>::from_bytes(&from[44..44 + 1]);
        let f_tmLastChar = <u16 as FromIntoMemory>::from_bytes(&from[45..45 + 1]);
        let f_tmDefaultChar = <u16 as FromIntoMemory>::from_bytes(&from[46..46 + 1]);
        let f_tmBreakChar = <u16 as FromIntoMemory>::from_bytes(&from[47..47 + 1]);
        let f_tmItalic = <u8 as FromIntoMemory>::from_bytes(&from[48..48 + 1]);
        let f_tmUnderlined = <u8 as FromIntoMemory>::from_bytes(&from[49..49 + 1]);
        let f_tmStruckOut = <u8 as FromIntoMemory>::from_bytes(&from[50..50 + 1]);
        let f_tmPitchAndFamily = <u8 as FromIntoMemory>::from_bytes(&from[51..51 + 1]);
        let f_tmCharSet = <u8 as FromIntoMemory>::from_bytes(&from[52..52 + 1]);
        let f_ntmFlags = <u32 as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_ntmSizeEM = <u32 as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_ntmCellHeight = <u32 as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        let f_ntmAvgWidth = <u32 as FromIntoMemory>::from_bytes(&from[68..68 + 4]);
        Self {
            tmHeight: f_tmHeight,
            tmAscent: f_tmAscent,
            tmDescent: f_tmDescent,
            tmInternalLeading: f_tmInternalLeading,
            tmExternalLeading: f_tmExternalLeading,
            tmAveCharWidth: f_tmAveCharWidth,
            tmMaxCharWidth: f_tmMaxCharWidth,
            tmWeight: f_tmWeight,
            tmOverhang: f_tmOverhang,
            tmDigitizedAspectX: f_tmDigitizedAspectX,
            tmDigitizedAspectY: f_tmDigitizedAspectY,
            tmFirstChar: f_tmFirstChar,
            tmLastChar: f_tmLastChar,
            tmDefaultChar: f_tmDefaultChar,
            tmBreakChar: f_tmBreakChar,
            tmItalic: f_tmItalic,
            tmUnderlined: f_tmUnderlined,
            tmStruckOut: f_tmStruckOut,
            tmPitchAndFamily: f_tmPitchAndFamily,
            tmCharSet: f_tmCharSet,
            ntmFlags: f_ntmFlags,
            ntmSizeEM: f_ntmSizeEM,
            ntmCellHeight: f_ntmCellHeight,
            ntmAvgWidth: f_ntmAvgWidth,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 72);
        FromIntoMemory::into_bytes(self.tmHeight, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.tmAscent, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.tmDescent, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.tmInternalLeading, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.tmExternalLeading, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.tmAveCharWidth, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.tmMaxCharWidth, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.tmWeight, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.tmOverhang, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.tmDigitizedAspectX, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.tmDigitizedAspectY, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.tmFirstChar, &mut into[44..44 + 1]);
        FromIntoMemory::into_bytes(self.tmLastChar, &mut into[45..45 + 1]);
        FromIntoMemory::into_bytes(self.tmDefaultChar, &mut into[46..46 + 1]);
        FromIntoMemory::into_bytes(self.tmBreakChar, &mut into[47..47 + 1]);
        FromIntoMemory::into_bytes(self.tmItalic, &mut into[48..48 + 1]);
        FromIntoMemory::into_bytes(self.tmUnderlined, &mut into[49..49 + 1]);
        FromIntoMemory::into_bytes(self.tmStruckOut, &mut into[50..50 + 1]);
        FromIntoMemory::into_bytes(self.tmPitchAndFamily, &mut into[51..51 + 1]);
        FromIntoMemory::into_bytes(self.tmCharSet, &mut into[52..52 + 1]);
        FromIntoMemory::into_bytes(self.ntmFlags, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.ntmSizeEM, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.ntmCellHeight, &mut into[64..64 + 4]);
        FromIntoMemory::into_bytes(self.ntmAvgWidth, &mut into[68..68 + 4]);
    }
    fn size() -> usize {
        72
    }
}
pub const NEWTRANSPARENT: u32 = 3u32;
pub const NEXTBAND: u32 = 3u32;
pub const NTM_BOLD: i32 = 32i32;
pub const NTM_DSIG: u32 = 2097152u32;
pub const NTM_ITALIC: i32 = 1i32;
pub const NTM_MULTIPLEMASTER: u32 = 524288u32;
pub const NTM_NONNEGATIVE_AC: u32 = 65536u32;
pub const NTM_PS_OPENTYPE: u32 = 131072u32;
pub const NTM_REGULAR: i32 = 64i32;
pub const NTM_TT_OPENTYPE: u32 = 262144u32;
pub const NTM_TYPE1: u32 = 1048576u32;
pub const NULLREGION: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct OBJ_TYPE(pub i32);
pub const OBJ_PEN: OBJ_TYPE = OBJ_TYPE(1i32);
pub const OBJ_BRUSH: OBJ_TYPE = OBJ_TYPE(2i32);
pub const OBJ_DC: OBJ_TYPE = OBJ_TYPE(3i32);
pub const OBJ_METADC: OBJ_TYPE = OBJ_TYPE(4i32);
pub const OBJ_PAL: OBJ_TYPE = OBJ_TYPE(5i32);
pub const OBJ_FONT: OBJ_TYPE = OBJ_TYPE(6i32);
pub const OBJ_BITMAP: OBJ_TYPE = OBJ_TYPE(7i32);
pub const OBJ_REGION: OBJ_TYPE = OBJ_TYPE(8i32);
pub const OBJ_METAFILE: OBJ_TYPE = OBJ_TYPE(9i32);
pub const OBJ_MEMDC: OBJ_TYPE = OBJ_TYPE(10i32);
pub const OBJ_EXTPEN: OBJ_TYPE = OBJ_TYPE(11i32);
pub const OBJ_ENHMETADC: OBJ_TYPE = OBJ_TYPE(12i32);
pub const OBJ_ENHMETAFILE: OBJ_TYPE = OBJ_TYPE(13i32);
pub const OBJ_COLORSPACE: OBJ_TYPE = OBJ_TYPE(14i32);
impl ::core::marker::Copy for OBJ_TYPE {}
impl ::core::clone::Clone for OBJ_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OBJ_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OBJ_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OBJ_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for OBJ_TYPE {
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
pub const OEM_CHARSET: u32 = 255u32;
pub const OPENCHANNEL: u32 = 4110u32;
pub struct OUTLINETEXTMETRICA {
    pub otmSize: u32,
    pub otmTextMetrics: TEXTMETRICA,
    pub otmFiller: u8,
    pub otmPanoseNumber: PANOSE,
    pub otmfsSelection: u32,
    pub otmfsType: u32,
    pub otmsCharSlopeRise: i32,
    pub otmsCharSlopeRun: i32,
    pub otmItalicAngle: i32,
    pub otmEMSquare: u32,
    pub otmAscent: i32,
    pub otmDescent: i32,
    pub otmLineGap: u32,
    pub otmsCapEmHeight: u32,
    pub otmsXHeight: u32,
    pub otmrcFontBox: super::super::Foundation::RECT,
    pub otmMacAscent: i32,
    pub otmMacDescent: i32,
    pub otmMacLineGap: u32,
    pub otmusMinimumPPEM: u32,
    pub otmptSubscriptSize: super::super::Foundation::POINT,
    pub otmptSubscriptOffset: super::super::Foundation::POINT,
    pub otmptSuperscriptSize: super::super::Foundation::POINT,
    pub otmptSuperscriptOffset: super::super::Foundation::POINT,
    pub otmsStrikeoutSize: u32,
    pub otmsStrikeoutPosition: i32,
    pub otmsUnderscoreSize: i32,
    pub otmsUnderscorePosition: i32,
    pub otmpFamilyName: PSTR,
    pub otmpFaceName: PSTR,
    pub otmpStyleName: PSTR,
    pub otmpFullName: PSTR,
}
impl ::core::marker::Copy for OUTLINETEXTMETRICA {}
impl ::core::clone::Clone for OUTLINETEXTMETRICA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OUTLINETEXTMETRICA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OUTLINETEXTMETRICA")
            .field("otmSize", &self.otmSize)
            .field("otmTextMetrics", &self.otmTextMetrics)
            .field("otmFiller", &self.otmFiller)
            .field("otmPanoseNumber", &self.otmPanoseNumber)
            .field("otmfsSelection", &self.otmfsSelection)
            .field("otmfsType", &self.otmfsType)
            .field("otmsCharSlopeRise", &self.otmsCharSlopeRise)
            .field("otmsCharSlopeRun", &self.otmsCharSlopeRun)
            .field("otmItalicAngle", &self.otmItalicAngle)
            .field("otmEMSquare", &self.otmEMSquare)
            .field("otmAscent", &self.otmAscent)
            .field("otmDescent", &self.otmDescent)
            .field("otmLineGap", &self.otmLineGap)
            .field("otmsCapEmHeight", &self.otmsCapEmHeight)
            .field("otmsXHeight", &self.otmsXHeight)
            .field("otmrcFontBox", &self.otmrcFontBox)
            .field("otmMacAscent", &self.otmMacAscent)
            .field("otmMacDescent", &self.otmMacDescent)
            .field("otmMacLineGap", &self.otmMacLineGap)
            .field("otmusMinimumPPEM", &self.otmusMinimumPPEM)
            .field("otmptSubscriptSize", &self.otmptSubscriptSize)
            .field("otmptSubscriptOffset", &self.otmptSubscriptOffset)
            .field("otmptSuperscriptSize", &self.otmptSuperscriptSize)
            .field("otmptSuperscriptOffset", &self.otmptSuperscriptOffset)
            .field("otmsStrikeoutSize", &self.otmsStrikeoutSize)
            .field("otmsStrikeoutPosition", &self.otmsStrikeoutPosition)
            .field("otmsUnderscoreSize", &self.otmsUnderscoreSize)
            .field("otmsUnderscorePosition", &self.otmsUnderscorePosition)
            .field("otmpFamilyName", &self.otmpFamilyName)
            .field("otmpFaceName", &self.otmpFaceName)
            .field("otmpStyleName", &self.otmpStyleName)
            .field("otmpFullName", &self.otmpFullName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for OUTLINETEXTMETRICA {
    fn eq(&self, other: &Self) -> bool {
        self.otmSize == other.otmSize
            && self.otmTextMetrics == other.otmTextMetrics
            && self.otmFiller == other.otmFiller
            && self.otmPanoseNumber == other.otmPanoseNumber
            && self.otmfsSelection == other.otmfsSelection
            && self.otmfsType == other.otmfsType
            && self.otmsCharSlopeRise == other.otmsCharSlopeRise
            && self.otmsCharSlopeRun == other.otmsCharSlopeRun
            && self.otmItalicAngle == other.otmItalicAngle
            && self.otmEMSquare == other.otmEMSquare
            && self.otmAscent == other.otmAscent
            && self.otmDescent == other.otmDescent
            && self.otmLineGap == other.otmLineGap
            && self.otmsCapEmHeight == other.otmsCapEmHeight
            && self.otmsXHeight == other.otmsXHeight
            && self.otmrcFontBox == other.otmrcFontBox
            && self.otmMacAscent == other.otmMacAscent
            && self.otmMacDescent == other.otmMacDescent
            && self.otmMacLineGap == other.otmMacLineGap
            && self.otmusMinimumPPEM == other.otmusMinimumPPEM
            && self.otmptSubscriptSize == other.otmptSubscriptSize
            && self.otmptSubscriptOffset == other.otmptSubscriptOffset
            && self.otmptSuperscriptSize == other.otmptSuperscriptSize
            && self.otmptSuperscriptOffset == other.otmptSuperscriptOffset
            && self.otmsStrikeoutSize == other.otmsStrikeoutSize
            && self.otmsStrikeoutPosition == other.otmsStrikeoutPosition
            && self.otmsUnderscoreSize == other.otmsUnderscoreSize
            && self.otmsUnderscorePosition == other.otmsUnderscorePosition
            && self.otmpFamilyName == other.otmpFamilyName
            && self.otmpFaceName == other.otmpFaceName
            && self.otmpStyleName == other.otmpStyleName
            && self.otmpFullName == other.otmpFullName
    }
}
impl ::core::cmp::Eq for OUTLINETEXTMETRICA {}
impl FromIntoMemory for OUTLINETEXTMETRICA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 212);
        let f_otmSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_otmTextMetrics = <TEXTMETRICA as FromIntoMemory>::from_bytes(&from[4..4 + 56]);
        let f_otmFiller = <u8 as FromIntoMemory>::from_bytes(&from[60..60 + 1]);
        let f_otmPanoseNumber = <PANOSE as FromIntoMemory>::from_bytes(&from[61..61 + 10]);
        let f_otmfsSelection = <u32 as FromIntoMemory>::from_bytes(&from[72..72 + 4]);
        let f_otmfsType = <u32 as FromIntoMemory>::from_bytes(&from[76..76 + 4]);
        let f_otmsCharSlopeRise = <i32 as FromIntoMemory>::from_bytes(&from[80..80 + 4]);
        let f_otmsCharSlopeRun = <i32 as FromIntoMemory>::from_bytes(&from[84..84 + 4]);
        let f_otmItalicAngle = <i32 as FromIntoMemory>::from_bytes(&from[88..88 + 4]);
        let f_otmEMSquare = <u32 as FromIntoMemory>::from_bytes(&from[92..92 + 4]);
        let f_otmAscent = <i32 as FromIntoMemory>::from_bytes(&from[96..96 + 4]);
        let f_otmDescent = <i32 as FromIntoMemory>::from_bytes(&from[100..100 + 4]);
        let f_otmLineGap = <u32 as FromIntoMemory>::from_bytes(&from[104..104 + 4]);
        let f_otmsCapEmHeight = <u32 as FromIntoMemory>::from_bytes(&from[108..108 + 4]);
        let f_otmsXHeight = <u32 as FromIntoMemory>::from_bytes(&from[112..112 + 4]);
        let f_otmrcFontBox =
            <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[116..116 + 16]);
        let f_otmMacAscent = <i32 as FromIntoMemory>::from_bytes(&from[132..132 + 4]);
        let f_otmMacDescent = <i32 as FromIntoMemory>::from_bytes(&from[136..136 + 4]);
        let f_otmMacLineGap = <u32 as FromIntoMemory>::from_bytes(&from[140..140 + 4]);
        let f_otmusMinimumPPEM = <u32 as FromIntoMemory>::from_bytes(&from[144..144 + 4]);
        let f_otmptSubscriptSize =
            <super::super::Foundation::POINT as FromIntoMemory>::from_bytes(&from[148..148 + 8]);
        let f_otmptSubscriptOffset =
            <super::super::Foundation::POINT as FromIntoMemory>::from_bytes(&from[156..156 + 8]);
        let f_otmptSuperscriptSize =
            <super::super::Foundation::POINT as FromIntoMemory>::from_bytes(&from[164..164 + 8]);
        let f_otmptSuperscriptOffset =
            <super::super::Foundation::POINT as FromIntoMemory>::from_bytes(&from[172..172 + 8]);
        let f_otmsStrikeoutSize = <u32 as FromIntoMemory>::from_bytes(&from[180..180 + 4]);
        let f_otmsStrikeoutPosition = <i32 as FromIntoMemory>::from_bytes(&from[184..184 + 4]);
        let f_otmsUnderscoreSize = <i32 as FromIntoMemory>::from_bytes(&from[188..188 + 4]);
        let f_otmsUnderscorePosition = <i32 as FromIntoMemory>::from_bytes(&from[192..192 + 4]);
        let f_otmpFamilyName = <PSTR as FromIntoMemory>::from_bytes(&from[196..196 + 4]);
        let f_otmpFaceName = <PSTR as FromIntoMemory>::from_bytes(&from[200..200 + 4]);
        let f_otmpStyleName = <PSTR as FromIntoMemory>::from_bytes(&from[204..204 + 4]);
        let f_otmpFullName = <PSTR as FromIntoMemory>::from_bytes(&from[208..208 + 4]);
        Self {
            otmSize: f_otmSize,
            otmTextMetrics: f_otmTextMetrics,
            otmFiller: f_otmFiller,
            otmPanoseNumber: f_otmPanoseNumber,
            otmfsSelection: f_otmfsSelection,
            otmfsType: f_otmfsType,
            otmsCharSlopeRise: f_otmsCharSlopeRise,
            otmsCharSlopeRun: f_otmsCharSlopeRun,
            otmItalicAngle: f_otmItalicAngle,
            otmEMSquare: f_otmEMSquare,
            otmAscent: f_otmAscent,
            otmDescent: f_otmDescent,
            otmLineGap: f_otmLineGap,
            otmsCapEmHeight: f_otmsCapEmHeight,
            otmsXHeight: f_otmsXHeight,
            otmrcFontBox: f_otmrcFontBox,
            otmMacAscent: f_otmMacAscent,
            otmMacDescent: f_otmMacDescent,
            otmMacLineGap: f_otmMacLineGap,
            otmusMinimumPPEM: f_otmusMinimumPPEM,
            otmptSubscriptSize: f_otmptSubscriptSize,
            otmptSubscriptOffset: f_otmptSubscriptOffset,
            otmptSuperscriptSize: f_otmptSuperscriptSize,
            otmptSuperscriptOffset: f_otmptSuperscriptOffset,
            otmsStrikeoutSize: f_otmsStrikeoutSize,
            otmsStrikeoutPosition: f_otmsStrikeoutPosition,
            otmsUnderscoreSize: f_otmsUnderscoreSize,
            otmsUnderscorePosition: f_otmsUnderscorePosition,
            otmpFamilyName: f_otmpFamilyName,
            otmpFaceName: f_otmpFaceName,
            otmpStyleName: f_otmpStyleName,
            otmpFullName: f_otmpFullName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 212);
        FromIntoMemory::into_bytes(self.otmSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.otmTextMetrics, &mut into[4..4 + 56]);
        FromIntoMemory::into_bytes(self.otmFiller, &mut into[60..60 + 1]);
        FromIntoMemory::into_bytes(self.otmPanoseNumber, &mut into[61..61 + 10]);
        FromIntoMemory::into_bytes(self.otmfsSelection, &mut into[72..72 + 4]);
        FromIntoMemory::into_bytes(self.otmfsType, &mut into[76..76 + 4]);
        FromIntoMemory::into_bytes(self.otmsCharSlopeRise, &mut into[80..80 + 4]);
        FromIntoMemory::into_bytes(self.otmsCharSlopeRun, &mut into[84..84 + 4]);
        FromIntoMemory::into_bytes(self.otmItalicAngle, &mut into[88..88 + 4]);
        FromIntoMemory::into_bytes(self.otmEMSquare, &mut into[92..92 + 4]);
        FromIntoMemory::into_bytes(self.otmAscent, &mut into[96..96 + 4]);
        FromIntoMemory::into_bytes(self.otmDescent, &mut into[100..100 + 4]);
        FromIntoMemory::into_bytes(self.otmLineGap, &mut into[104..104 + 4]);
        FromIntoMemory::into_bytes(self.otmsCapEmHeight, &mut into[108..108 + 4]);
        FromIntoMemory::into_bytes(self.otmsXHeight, &mut into[112..112 + 4]);
        FromIntoMemory::into_bytes(self.otmrcFontBox, &mut into[116..116 + 16]);
        FromIntoMemory::into_bytes(self.otmMacAscent, &mut into[132..132 + 4]);
        FromIntoMemory::into_bytes(self.otmMacDescent, &mut into[136..136 + 4]);
        FromIntoMemory::into_bytes(self.otmMacLineGap, &mut into[140..140 + 4]);
        FromIntoMemory::into_bytes(self.otmusMinimumPPEM, &mut into[144..144 + 4]);
        FromIntoMemory::into_bytes(self.otmptSubscriptSize, &mut into[148..148 + 8]);
        FromIntoMemory::into_bytes(self.otmptSubscriptOffset, &mut into[156..156 + 8]);
        FromIntoMemory::into_bytes(self.otmptSuperscriptSize, &mut into[164..164 + 8]);
        FromIntoMemory::into_bytes(self.otmptSuperscriptOffset, &mut into[172..172 + 8]);
        FromIntoMemory::into_bytes(self.otmsStrikeoutSize, &mut into[180..180 + 4]);
        FromIntoMemory::into_bytes(self.otmsStrikeoutPosition, &mut into[184..184 + 4]);
        FromIntoMemory::into_bytes(self.otmsUnderscoreSize, &mut into[188..188 + 4]);
        FromIntoMemory::into_bytes(self.otmsUnderscorePosition, &mut into[192..192 + 4]);
        FromIntoMemory::into_bytes(self.otmpFamilyName, &mut into[196..196 + 4]);
        FromIntoMemory::into_bytes(self.otmpFaceName, &mut into[200..200 + 4]);
        FromIntoMemory::into_bytes(self.otmpStyleName, &mut into[204..204 + 4]);
        FromIntoMemory::into_bytes(self.otmpFullName, &mut into[208..208 + 4]);
    }
    fn size() -> usize {
        212
    }
}
pub struct OUTLINETEXTMETRICW {
    pub otmSize: u32,
    pub otmTextMetrics: TEXTMETRICW,
    pub otmFiller: u8,
    pub otmPanoseNumber: PANOSE,
    pub otmfsSelection: u32,
    pub otmfsType: u32,
    pub otmsCharSlopeRise: i32,
    pub otmsCharSlopeRun: i32,
    pub otmItalicAngle: i32,
    pub otmEMSquare: u32,
    pub otmAscent: i32,
    pub otmDescent: i32,
    pub otmLineGap: u32,
    pub otmsCapEmHeight: u32,
    pub otmsXHeight: u32,
    pub otmrcFontBox: super::super::Foundation::RECT,
    pub otmMacAscent: i32,
    pub otmMacDescent: i32,
    pub otmMacLineGap: u32,
    pub otmusMinimumPPEM: u32,
    pub otmptSubscriptSize: super::super::Foundation::POINT,
    pub otmptSubscriptOffset: super::super::Foundation::POINT,
    pub otmptSuperscriptSize: super::super::Foundation::POINT,
    pub otmptSuperscriptOffset: super::super::Foundation::POINT,
    pub otmsStrikeoutSize: u32,
    pub otmsStrikeoutPosition: i32,
    pub otmsUnderscoreSize: i32,
    pub otmsUnderscorePosition: i32,
    pub otmpFamilyName: PSTR,
    pub otmpFaceName: PSTR,
    pub otmpStyleName: PSTR,
    pub otmpFullName: PSTR,
}
impl ::core::marker::Copy for OUTLINETEXTMETRICW {}
impl ::core::clone::Clone for OUTLINETEXTMETRICW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OUTLINETEXTMETRICW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OUTLINETEXTMETRICW")
            .field("otmSize", &self.otmSize)
            .field("otmTextMetrics", &self.otmTextMetrics)
            .field("otmFiller", &self.otmFiller)
            .field("otmPanoseNumber", &self.otmPanoseNumber)
            .field("otmfsSelection", &self.otmfsSelection)
            .field("otmfsType", &self.otmfsType)
            .field("otmsCharSlopeRise", &self.otmsCharSlopeRise)
            .field("otmsCharSlopeRun", &self.otmsCharSlopeRun)
            .field("otmItalicAngle", &self.otmItalicAngle)
            .field("otmEMSquare", &self.otmEMSquare)
            .field("otmAscent", &self.otmAscent)
            .field("otmDescent", &self.otmDescent)
            .field("otmLineGap", &self.otmLineGap)
            .field("otmsCapEmHeight", &self.otmsCapEmHeight)
            .field("otmsXHeight", &self.otmsXHeight)
            .field("otmrcFontBox", &self.otmrcFontBox)
            .field("otmMacAscent", &self.otmMacAscent)
            .field("otmMacDescent", &self.otmMacDescent)
            .field("otmMacLineGap", &self.otmMacLineGap)
            .field("otmusMinimumPPEM", &self.otmusMinimumPPEM)
            .field("otmptSubscriptSize", &self.otmptSubscriptSize)
            .field("otmptSubscriptOffset", &self.otmptSubscriptOffset)
            .field("otmptSuperscriptSize", &self.otmptSuperscriptSize)
            .field("otmptSuperscriptOffset", &self.otmptSuperscriptOffset)
            .field("otmsStrikeoutSize", &self.otmsStrikeoutSize)
            .field("otmsStrikeoutPosition", &self.otmsStrikeoutPosition)
            .field("otmsUnderscoreSize", &self.otmsUnderscoreSize)
            .field("otmsUnderscorePosition", &self.otmsUnderscorePosition)
            .field("otmpFamilyName", &self.otmpFamilyName)
            .field("otmpFaceName", &self.otmpFaceName)
            .field("otmpStyleName", &self.otmpStyleName)
            .field("otmpFullName", &self.otmpFullName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for OUTLINETEXTMETRICW {
    fn eq(&self, other: &Self) -> bool {
        self.otmSize == other.otmSize
            && self.otmTextMetrics == other.otmTextMetrics
            && self.otmFiller == other.otmFiller
            && self.otmPanoseNumber == other.otmPanoseNumber
            && self.otmfsSelection == other.otmfsSelection
            && self.otmfsType == other.otmfsType
            && self.otmsCharSlopeRise == other.otmsCharSlopeRise
            && self.otmsCharSlopeRun == other.otmsCharSlopeRun
            && self.otmItalicAngle == other.otmItalicAngle
            && self.otmEMSquare == other.otmEMSquare
            && self.otmAscent == other.otmAscent
            && self.otmDescent == other.otmDescent
            && self.otmLineGap == other.otmLineGap
            && self.otmsCapEmHeight == other.otmsCapEmHeight
            && self.otmsXHeight == other.otmsXHeight
            && self.otmrcFontBox == other.otmrcFontBox
            && self.otmMacAscent == other.otmMacAscent
            && self.otmMacDescent == other.otmMacDescent
            && self.otmMacLineGap == other.otmMacLineGap
            && self.otmusMinimumPPEM == other.otmusMinimumPPEM
            && self.otmptSubscriptSize == other.otmptSubscriptSize
            && self.otmptSubscriptOffset == other.otmptSubscriptOffset
            && self.otmptSuperscriptSize == other.otmptSuperscriptSize
            && self.otmptSuperscriptOffset == other.otmptSuperscriptOffset
            && self.otmsStrikeoutSize == other.otmsStrikeoutSize
            && self.otmsStrikeoutPosition == other.otmsStrikeoutPosition
            && self.otmsUnderscoreSize == other.otmsUnderscoreSize
            && self.otmsUnderscorePosition == other.otmsUnderscorePosition
            && self.otmpFamilyName == other.otmpFamilyName
            && self.otmpFaceName == other.otmpFaceName
            && self.otmpStyleName == other.otmpStyleName
            && self.otmpFullName == other.otmpFullName
    }
}
impl ::core::cmp::Eq for OUTLINETEXTMETRICW {}
impl FromIntoMemory for OUTLINETEXTMETRICW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 212);
        let f_otmSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_otmTextMetrics = <TEXTMETRICW as FromIntoMemory>::from_bytes(&from[4..4 + 56]);
        let f_otmFiller = <u8 as FromIntoMemory>::from_bytes(&from[60..60 + 1]);
        let f_otmPanoseNumber = <PANOSE as FromIntoMemory>::from_bytes(&from[61..61 + 10]);
        let f_otmfsSelection = <u32 as FromIntoMemory>::from_bytes(&from[72..72 + 4]);
        let f_otmfsType = <u32 as FromIntoMemory>::from_bytes(&from[76..76 + 4]);
        let f_otmsCharSlopeRise = <i32 as FromIntoMemory>::from_bytes(&from[80..80 + 4]);
        let f_otmsCharSlopeRun = <i32 as FromIntoMemory>::from_bytes(&from[84..84 + 4]);
        let f_otmItalicAngle = <i32 as FromIntoMemory>::from_bytes(&from[88..88 + 4]);
        let f_otmEMSquare = <u32 as FromIntoMemory>::from_bytes(&from[92..92 + 4]);
        let f_otmAscent = <i32 as FromIntoMemory>::from_bytes(&from[96..96 + 4]);
        let f_otmDescent = <i32 as FromIntoMemory>::from_bytes(&from[100..100 + 4]);
        let f_otmLineGap = <u32 as FromIntoMemory>::from_bytes(&from[104..104 + 4]);
        let f_otmsCapEmHeight = <u32 as FromIntoMemory>::from_bytes(&from[108..108 + 4]);
        let f_otmsXHeight = <u32 as FromIntoMemory>::from_bytes(&from[112..112 + 4]);
        let f_otmrcFontBox =
            <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[116..116 + 16]);
        let f_otmMacAscent = <i32 as FromIntoMemory>::from_bytes(&from[132..132 + 4]);
        let f_otmMacDescent = <i32 as FromIntoMemory>::from_bytes(&from[136..136 + 4]);
        let f_otmMacLineGap = <u32 as FromIntoMemory>::from_bytes(&from[140..140 + 4]);
        let f_otmusMinimumPPEM = <u32 as FromIntoMemory>::from_bytes(&from[144..144 + 4]);
        let f_otmptSubscriptSize =
            <super::super::Foundation::POINT as FromIntoMemory>::from_bytes(&from[148..148 + 8]);
        let f_otmptSubscriptOffset =
            <super::super::Foundation::POINT as FromIntoMemory>::from_bytes(&from[156..156 + 8]);
        let f_otmptSuperscriptSize =
            <super::super::Foundation::POINT as FromIntoMemory>::from_bytes(&from[164..164 + 8]);
        let f_otmptSuperscriptOffset =
            <super::super::Foundation::POINT as FromIntoMemory>::from_bytes(&from[172..172 + 8]);
        let f_otmsStrikeoutSize = <u32 as FromIntoMemory>::from_bytes(&from[180..180 + 4]);
        let f_otmsStrikeoutPosition = <i32 as FromIntoMemory>::from_bytes(&from[184..184 + 4]);
        let f_otmsUnderscoreSize = <i32 as FromIntoMemory>::from_bytes(&from[188..188 + 4]);
        let f_otmsUnderscorePosition = <i32 as FromIntoMemory>::from_bytes(&from[192..192 + 4]);
        let f_otmpFamilyName = <PSTR as FromIntoMemory>::from_bytes(&from[196..196 + 4]);
        let f_otmpFaceName = <PSTR as FromIntoMemory>::from_bytes(&from[200..200 + 4]);
        let f_otmpStyleName = <PSTR as FromIntoMemory>::from_bytes(&from[204..204 + 4]);
        let f_otmpFullName = <PSTR as FromIntoMemory>::from_bytes(&from[208..208 + 4]);
        Self {
            otmSize: f_otmSize,
            otmTextMetrics: f_otmTextMetrics,
            otmFiller: f_otmFiller,
            otmPanoseNumber: f_otmPanoseNumber,
            otmfsSelection: f_otmfsSelection,
            otmfsType: f_otmfsType,
            otmsCharSlopeRise: f_otmsCharSlopeRise,
            otmsCharSlopeRun: f_otmsCharSlopeRun,
            otmItalicAngle: f_otmItalicAngle,
            otmEMSquare: f_otmEMSquare,
            otmAscent: f_otmAscent,
            otmDescent: f_otmDescent,
            otmLineGap: f_otmLineGap,
            otmsCapEmHeight: f_otmsCapEmHeight,
            otmsXHeight: f_otmsXHeight,
            otmrcFontBox: f_otmrcFontBox,
            otmMacAscent: f_otmMacAscent,
            otmMacDescent: f_otmMacDescent,
            otmMacLineGap: f_otmMacLineGap,
            otmusMinimumPPEM: f_otmusMinimumPPEM,
            otmptSubscriptSize: f_otmptSubscriptSize,
            otmptSubscriptOffset: f_otmptSubscriptOffset,
            otmptSuperscriptSize: f_otmptSuperscriptSize,
            otmptSuperscriptOffset: f_otmptSuperscriptOffset,
            otmsStrikeoutSize: f_otmsStrikeoutSize,
            otmsStrikeoutPosition: f_otmsStrikeoutPosition,
            otmsUnderscoreSize: f_otmsUnderscoreSize,
            otmsUnderscorePosition: f_otmsUnderscorePosition,
            otmpFamilyName: f_otmpFamilyName,
            otmpFaceName: f_otmpFaceName,
            otmpStyleName: f_otmpStyleName,
            otmpFullName: f_otmpFullName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 212);
        FromIntoMemory::into_bytes(self.otmSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.otmTextMetrics, &mut into[4..4 + 56]);
        FromIntoMemory::into_bytes(self.otmFiller, &mut into[60..60 + 1]);
        FromIntoMemory::into_bytes(self.otmPanoseNumber, &mut into[61..61 + 10]);
        FromIntoMemory::into_bytes(self.otmfsSelection, &mut into[72..72 + 4]);
        FromIntoMemory::into_bytes(self.otmfsType, &mut into[76..76 + 4]);
        FromIntoMemory::into_bytes(self.otmsCharSlopeRise, &mut into[80..80 + 4]);
        FromIntoMemory::into_bytes(self.otmsCharSlopeRun, &mut into[84..84 + 4]);
        FromIntoMemory::into_bytes(self.otmItalicAngle, &mut into[88..88 + 4]);
        FromIntoMemory::into_bytes(self.otmEMSquare, &mut into[92..92 + 4]);
        FromIntoMemory::into_bytes(self.otmAscent, &mut into[96..96 + 4]);
        FromIntoMemory::into_bytes(self.otmDescent, &mut into[100..100 + 4]);
        FromIntoMemory::into_bytes(self.otmLineGap, &mut into[104..104 + 4]);
        FromIntoMemory::into_bytes(self.otmsCapEmHeight, &mut into[108..108 + 4]);
        FromIntoMemory::into_bytes(self.otmsXHeight, &mut into[112..112 + 4]);
        FromIntoMemory::into_bytes(self.otmrcFontBox, &mut into[116..116 + 16]);
        FromIntoMemory::into_bytes(self.otmMacAscent, &mut into[132..132 + 4]);
        FromIntoMemory::into_bytes(self.otmMacDescent, &mut into[136..136 + 4]);
        FromIntoMemory::into_bytes(self.otmMacLineGap, &mut into[140..140 + 4]);
        FromIntoMemory::into_bytes(self.otmusMinimumPPEM, &mut into[144..144 + 4]);
        FromIntoMemory::into_bytes(self.otmptSubscriptSize, &mut into[148..148 + 8]);
        FromIntoMemory::into_bytes(self.otmptSubscriptOffset, &mut into[156..156 + 8]);
        FromIntoMemory::into_bytes(self.otmptSuperscriptSize, &mut into[164..164 + 8]);
        FromIntoMemory::into_bytes(self.otmptSuperscriptOffset, &mut into[172..172 + 8]);
        FromIntoMemory::into_bytes(self.otmsStrikeoutSize, &mut into[180..180 + 4]);
        FromIntoMemory::into_bytes(self.otmsStrikeoutPosition, &mut into[184..184 + 4]);
        FromIntoMemory::into_bytes(self.otmsUnderscoreSize, &mut into[188..188 + 4]);
        FromIntoMemory::into_bytes(self.otmsUnderscorePosition, &mut into[192..192 + 4]);
        FromIntoMemory::into_bytes(self.otmpFamilyName, &mut into[196..196 + 4]);
        FromIntoMemory::into_bytes(self.otmpFaceName, &mut into[200..200 + 4]);
        FromIntoMemory::into_bytes(self.otmpStyleName, &mut into[204..204 + 4]);
        FromIntoMemory::into_bytes(self.otmpFullName, &mut into[208..208 + 4]);
    }
    fn size() -> usize {
        212
    }
}
pub const OUT_SCREEN_OUTLINE_PRECIS: u32 = 9u32;
pub struct PAINTSTRUCT {
    pub hdc: HDC,
    pub fErase: super::super::Foundation::BOOL,
    pub rcPaint: super::super::Foundation::RECT,
    pub fRestore: super::super::Foundation::BOOL,
    pub fIncUpdate: super::super::Foundation::BOOL,
    pub rgbReserved: [u8; 32],
}
impl ::core::marker::Copy for PAINTSTRUCT {}
impl ::core::clone::Clone for PAINTSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PAINTSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PAINTSTRUCT")
            .field("hdc", &self.hdc)
            .field("fErase", &self.fErase)
            .field("rcPaint", &self.rcPaint)
            .field("fRestore", &self.fRestore)
            .field("fIncUpdate", &self.fIncUpdate)
            .field("rgbReserved", &self.rgbReserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PAINTSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.hdc == other.hdc
            && self.fErase == other.fErase
            && self.rcPaint == other.rcPaint
            && self.fRestore == other.fRestore
            && self.fIncUpdate == other.fIncUpdate
            && self.rgbReserved == other.rgbReserved
    }
}
impl ::core::cmp::Eq for PAINTSTRUCT {}
impl FromIntoMemory for PAINTSTRUCT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 64);
        let f_hdc = <HDC as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_fErase =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_rcPaint =
            <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        let f_fRestore =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_fIncUpdate =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_rgbReserved = <[u8; 32] as FromIntoMemory>::from_bytes(&from[32..32 + 32]);
        Self {
            hdc: f_hdc,
            fErase: f_fErase,
            rcPaint: f_rcPaint,
            fRestore: f_fRestore,
            fIncUpdate: f_fIncUpdate,
            rgbReserved: f_rgbReserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 64);
        FromIntoMemory::into_bytes(self.hdc, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.fErase, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.rcPaint, &mut into[8..8 + 16]);
        FromIntoMemory::into_bytes(self.fRestore, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.fIncUpdate, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.rgbReserved, &mut into[32..32 + 32]);
    }
    fn size() -> usize {
        64
    }
}
pub struct PALETTEENTRY {
    pub peRed: u8,
    pub peGreen: u8,
    pub peBlue: u8,
    pub peFlags: u8,
}
impl ::core::marker::Copy for PALETTEENTRY {}
impl ::core::clone::Clone for PALETTEENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PALETTEENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PALETTEENTRY")
            .field("peRed", &self.peRed)
            .field("peGreen", &self.peGreen)
            .field("peBlue", &self.peBlue)
            .field("peFlags", &self.peFlags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PALETTEENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.peRed == other.peRed
            && self.peGreen == other.peGreen
            && self.peBlue == other.peBlue
            && self.peFlags == other.peFlags
    }
}
impl ::core::cmp::Eq for PALETTEENTRY {}
impl FromIntoMemory for PALETTEENTRY {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_peRed = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_peGreen = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_peBlue = <u8 as FromIntoMemory>::from_bytes(&from[2..2 + 1]);
        let f_peFlags = <u8 as FromIntoMemory>::from_bytes(&from[3..3 + 1]);
        Self {
            peRed: f_peRed,
            peGreen: f_peGreen,
            peBlue: f_peBlue,
            peFlags: f_peFlags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.peRed, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.peGreen, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.peBlue, &mut into[2..2 + 1]);
        FromIntoMemory::into_bytes(self.peFlags, &mut into[3..3 + 1]);
    }
    fn size() -> usize {
        4
    }
}
pub struct PANOSE {
    pub bFamilyType: u8,
    pub bSerifStyle: u8,
    pub bWeight: u8,
    pub bProportion: u8,
    pub bContrast: u8,
    pub bStrokeVariation: u8,
    pub bArmStyle: u8,
    pub bLetterform: u8,
    pub bMidline: u8,
    pub bXHeight: u8,
}
impl ::core::marker::Copy for PANOSE {}
impl ::core::clone::Clone for PANOSE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PANOSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PANOSE")
            .field("bFamilyType", &self.bFamilyType)
            .field("bSerifStyle", &self.bSerifStyle)
            .field("bWeight", &self.bWeight)
            .field("bProportion", &self.bProportion)
            .field("bContrast", &self.bContrast)
            .field("bStrokeVariation", &self.bStrokeVariation)
            .field("bArmStyle", &self.bArmStyle)
            .field("bLetterform", &self.bLetterform)
            .field("bMidline", &self.bMidline)
            .field("bXHeight", &self.bXHeight)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PANOSE {
    fn eq(&self, other: &Self) -> bool {
        self.bFamilyType == other.bFamilyType
            && self.bSerifStyle == other.bSerifStyle
            && self.bWeight == other.bWeight
            && self.bProportion == other.bProportion
            && self.bContrast == other.bContrast
            && self.bStrokeVariation == other.bStrokeVariation
            && self.bArmStyle == other.bArmStyle
            && self.bLetterform == other.bLetterform
            && self.bMidline == other.bMidline
            && self.bXHeight == other.bXHeight
    }
}
impl ::core::cmp::Eq for PANOSE {}
impl FromIntoMemory for PANOSE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 10);
        let f_bFamilyType = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_bSerifStyle = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_bWeight = <u8 as FromIntoMemory>::from_bytes(&from[2..2 + 1]);
        let f_bProportion = <u8 as FromIntoMemory>::from_bytes(&from[3..3 + 1]);
        let f_bContrast = <u8 as FromIntoMemory>::from_bytes(&from[4..4 + 1]);
        let f_bStrokeVariation = <u8 as FromIntoMemory>::from_bytes(&from[5..5 + 1]);
        let f_bArmStyle = <u8 as FromIntoMemory>::from_bytes(&from[6..6 + 1]);
        let f_bLetterform = <u8 as FromIntoMemory>::from_bytes(&from[7..7 + 1]);
        let f_bMidline = <u8 as FromIntoMemory>::from_bytes(&from[8..8 + 1]);
        let f_bXHeight = <u8 as FromIntoMemory>::from_bytes(&from[9..9 + 1]);
        Self {
            bFamilyType: f_bFamilyType,
            bSerifStyle: f_bSerifStyle,
            bWeight: f_bWeight,
            bProportion: f_bProportion,
            bContrast: f_bContrast,
            bStrokeVariation: f_bStrokeVariation,
            bArmStyle: f_bArmStyle,
            bLetterform: f_bLetterform,
            bMidline: f_bMidline,
            bXHeight: f_bXHeight,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 10);
        FromIntoMemory::into_bytes(self.bFamilyType, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.bSerifStyle, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.bWeight, &mut into[2..2 + 1]);
        FromIntoMemory::into_bytes(self.bProportion, &mut into[3..3 + 1]);
        FromIntoMemory::into_bytes(self.bContrast, &mut into[4..4 + 1]);
        FromIntoMemory::into_bytes(self.bStrokeVariation, &mut into[5..5 + 1]);
        FromIntoMemory::into_bytes(self.bArmStyle, &mut into[6..6 + 1]);
        FromIntoMemory::into_bytes(self.bLetterform, &mut into[7..7 + 1]);
        FromIntoMemory::into_bytes(self.bMidline, &mut into[8..8 + 1]);
        FromIntoMemory::into_bytes(self.bXHeight, &mut into[9..9 + 1]);
    }
    fn size() -> usize {
        10
    }
}
pub const PANOSE_COUNT: u32 = 10u32;
pub const PAN_ANY: u32 = 0u32;
pub const PAN_ARMSTYLE_INDEX: u32 = 6u32;
pub const PAN_BENT_ARMS_DOUBLE_SERIF: u32 = 11u32;
pub const PAN_BENT_ARMS_HORZ: u32 = 7u32;
pub const PAN_BENT_ARMS_SINGLE_SERIF: u32 = 10u32;
pub const PAN_BENT_ARMS_VERT: u32 = 9u32;
pub const PAN_BENT_ARMS_WEDGE: u32 = 8u32;
pub const PAN_CONTRAST_HIGH: u32 = 8u32;
pub const PAN_CONTRAST_INDEX: u32 = 4u32;
pub const PAN_CONTRAST_LOW: u32 = 4u32;
pub const PAN_CONTRAST_MEDIUM: u32 = 6u32;
pub const PAN_CONTRAST_MEDIUM_HIGH: u32 = 7u32;
pub const PAN_CONTRAST_MEDIUM_LOW: u32 = 5u32;
pub const PAN_CONTRAST_NONE: u32 = 2u32;
pub const PAN_CONTRAST_VERY_HIGH: u32 = 9u32;
pub const PAN_CONTRAST_VERY_LOW: u32 = 3u32;
pub const PAN_CULTURE_LATIN: u32 = 0u32;
pub const PAN_FAMILYTYPE_INDEX: u32 = 0u32;
pub const PAN_FAMILY_DECORATIVE: u32 = 4u32;
pub const PAN_FAMILY_PICTORIAL: u32 = 5u32;
pub const PAN_FAMILY_SCRIPT: u32 = 3u32;
pub const PAN_FAMILY_TEXT_DISPLAY: u32 = 2u32;
pub const PAN_LETTERFORM_INDEX: u32 = 7u32;
pub const PAN_LETT_NORMAL_BOXED: u32 = 4u32;
pub const PAN_LETT_NORMAL_CONTACT: u32 = 2u32;
pub const PAN_LETT_NORMAL_FLATTENED: u32 = 5u32;
pub const PAN_LETT_NORMAL_OFF_CENTER: u32 = 7u32;
pub const PAN_LETT_NORMAL_ROUNDED: u32 = 6u32;
pub const PAN_LETT_NORMAL_SQUARE: u32 = 8u32;
pub const PAN_LETT_NORMAL_WEIGHTED: u32 = 3u32;
pub const PAN_LETT_OBLIQUE_BOXED: u32 = 11u32;
pub const PAN_LETT_OBLIQUE_CONTACT: u32 = 9u32;
pub const PAN_LETT_OBLIQUE_FLATTENED: u32 = 12u32;
pub const PAN_LETT_OBLIQUE_OFF_CENTER: u32 = 14u32;
pub const PAN_LETT_OBLIQUE_ROUNDED: u32 = 13u32;
pub const PAN_LETT_OBLIQUE_SQUARE: u32 = 15u32;
pub const PAN_LETT_OBLIQUE_WEIGHTED: u32 = 10u32;
pub const PAN_MIDLINE_CONSTANT_POINTED: u32 = 9u32;
pub const PAN_MIDLINE_CONSTANT_SERIFED: u32 = 10u32;
pub const PAN_MIDLINE_CONSTANT_TRIMMED: u32 = 8u32;
pub const PAN_MIDLINE_HIGH_POINTED: u32 = 6u32;
pub const PAN_MIDLINE_HIGH_SERIFED: u32 = 7u32;
pub const PAN_MIDLINE_HIGH_TRIMMED: u32 = 5u32;
pub const PAN_MIDLINE_INDEX: u32 = 8u32;
pub const PAN_MIDLINE_LOW_POINTED: u32 = 12u32;
pub const PAN_MIDLINE_LOW_SERIFED: u32 = 13u32;
pub const PAN_MIDLINE_LOW_TRIMMED: u32 = 11u32;
pub const PAN_MIDLINE_STANDARD_POINTED: u32 = 3u32;
pub const PAN_MIDLINE_STANDARD_SERIFED: u32 = 4u32;
pub const PAN_MIDLINE_STANDARD_TRIMMED: u32 = 2u32;
pub const PAN_NO_FIT: u32 = 1u32;
pub const PAN_PROPORTION_INDEX: u32 = 3u32;
pub const PAN_PROP_CONDENSED: u32 = 6u32;
pub const PAN_PROP_EVEN_WIDTH: u32 = 4u32;
pub const PAN_PROP_EXPANDED: u32 = 5u32;
pub const PAN_PROP_MODERN: u32 = 3u32;
pub const PAN_PROP_MONOSPACED: u32 = 9u32;
pub const PAN_PROP_OLD_STYLE: u32 = 2u32;
pub const PAN_PROP_VERY_CONDENSED: u32 = 8u32;
pub const PAN_PROP_VERY_EXPANDED: u32 = 7u32;
pub const PAN_SERIFSTYLE_INDEX: u32 = 1u32;
pub const PAN_SERIF_BONE: u32 = 8u32;
pub const PAN_SERIF_COVE: u32 = 2u32;
pub const PAN_SERIF_EXAGGERATED: u32 = 9u32;
pub const PAN_SERIF_FLARED: u32 = 14u32;
pub const PAN_SERIF_NORMAL_SANS: u32 = 11u32;
pub const PAN_SERIF_OBTUSE_COVE: u32 = 3u32;
pub const PAN_SERIF_OBTUSE_SANS: u32 = 12u32;
pub const PAN_SERIF_OBTUSE_SQUARE_COVE: u32 = 5u32;
pub const PAN_SERIF_PERP_SANS: u32 = 13u32;
pub const PAN_SERIF_ROUNDED: u32 = 15u32;
pub const PAN_SERIF_SQUARE: u32 = 6u32;
pub const PAN_SERIF_SQUARE_COVE: u32 = 4u32;
pub const PAN_SERIF_THIN: u32 = 7u32;
pub const PAN_SERIF_TRIANGLE: u32 = 10u32;
pub const PAN_STRAIGHT_ARMS_DOUBLE_SERIF: u32 = 6u32;
pub const PAN_STRAIGHT_ARMS_HORZ: u32 = 2u32;
pub const PAN_STRAIGHT_ARMS_SINGLE_SERIF: u32 = 5u32;
pub const PAN_STRAIGHT_ARMS_VERT: u32 = 4u32;
pub const PAN_STRAIGHT_ARMS_WEDGE: u32 = 3u32;
pub const PAN_STROKEVARIATION_INDEX: u32 = 5u32;
pub const PAN_STROKE_GRADUAL_DIAG: u32 = 2u32;
pub const PAN_STROKE_GRADUAL_HORZ: u32 = 5u32;
pub const PAN_STROKE_GRADUAL_TRAN: u32 = 3u32;
pub const PAN_STROKE_GRADUAL_VERT: u32 = 4u32;
pub const PAN_STROKE_INSTANT_VERT: u32 = 8u32;
pub const PAN_STROKE_RAPID_HORZ: u32 = 7u32;
pub const PAN_STROKE_RAPID_VERT: u32 = 6u32;
pub const PAN_WEIGHT_BLACK: u32 = 10u32;
pub const PAN_WEIGHT_BOLD: u32 = 8u32;
pub const PAN_WEIGHT_BOOK: u32 = 5u32;
pub const PAN_WEIGHT_DEMI: u32 = 7u32;
pub const PAN_WEIGHT_HEAVY: u32 = 9u32;
pub const PAN_WEIGHT_INDEX: u32 = 2u32;
pub const PAN_WEIGHT_LIGHT: u32 = 3u32;
pub const PAN_WEIGHT_MEDIUM: u32 = 6u32;
pub const PAN_WEIGHT_NORD: u32 = 11u32;
pub const PAN_WEIGHT_THIN: u32 = 4u32;
pub const PAN_WEIGHT_VERY_LIGHT: u32 = 2u32;
pub const PAN_XHEIGHT_CONSTANT_LARGE: u32 = 4u32;
pub const PAN_XHEIGHT_CONSTANT_SMALL: u32 = 2u32;
pub const PAN_XHEIGHT_CONSTANT_STD: u32 = 3u32;
pub const PAN_XHEIGHT_DUCKING_LARGE: u32 = 7u32;
pub const PAN_XHEIGHT_DUCKING_SMALL: u32 = 5u32;
pub const PAN_XHEIGHT_DUCKING_STD: u32 = 6u32;
pub const PAN_XHEIGHT_INDEX: u32 = 9u32;
pub const PASSTHROUGH: u32 = 19u32;
pub const PC_EXPLICIT: u32 = 2u32;
pub const PC_INTERIORS: u32 = 128u32;
pub const PC_NOCOLLAPSE: u32 = 4u32;
pub const PC_NONE: u32 = 0u32;
pub const PC_PATHS: u32 = 512u32;
pub const PC_POLYGON: u32 = 1u32;
pub const PC_POLYPOLYGON: u32 = 256u32;
pub const PC_RECTANGLE: u32 = 2u32;
pub const PC_RESERVED: u32 = 1u32;
pub const PC_SCANLINE: u32 = 8u32;
pub const PC_STYLED: u32 = 32u32;
pub const PC_TRAPEZOID: u32 = 4u32;
pub const PC_WIDE: u32 = 16u32;
pub const PC_WIDESTYLED: u32 = 64u32;
pub const PC_WINDPOLYGON: u32 = 4u32;
pub struct PELARRAY {
    pub paXCount: i32,
    pub paYCount: i32,
    pub paXExt: i32,
    pub paYExt: i32,
    pub paRGBs: u8,
}
impl ::core::marker::Copy for PELARRAY {}
impl ::core::clone::Clone for PELARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PELARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PELARRAY")
            .field("paXCount", &self.paXCount)
            .field("paYCount", &self.paYCount)
            .field("paXExt", &self.paXExt)
            .field("paYExt", &self.paYExt)
            .field("paRGBs", &self.paRGBs)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PELARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.paXCount == other.paXCount
            && self.paYCount == other.paYCount
            && self.paXExt == other.paXExt
            && self.paYExt == other.paYExt
            && self.paRGBs == other.paRGBs
    }
}
impl ::core::cmp::Eq for PELARRAY {}
impl FromIntoMemory for PELARRAY {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_paXCount = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_paYCount = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_paXExt = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_paYExt = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_paRGBs = <u8 as FromIntoMemory>::from_bytes(&from[16..16 + 1]);
        Self {
            paXCount: f_paXCount,
            paYCount: f_paYCount,
            paXExt: f_paXExt,
            paYExt: f_paYExt,
            paRGBs: f_paRGBs,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.paXCount, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.paYCount, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.paXExt, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.paYExt, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.paRGBs, &mut into[16..16 + 1]);
    }
    fn size() -> usize {
        20
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PEN_STYLE(pub u32);
pub const PS_GEOMETRIC: PEN_STYLE = PEN_STYLE(65536u32);
pub const PS_COSMETIC: PEN_STYLE = PEN_STYLE(0u32);
pub const PS_SOLID: PEN_STYLE = PEN_STYLE(0u32);
pub const PS_DASH: PEN_STYLE = PEN_STYLE(1u32);
pub const PS_DOT: PEN_STYLE = PEN_STYLE(2u32);
pub const PS_DASHDOT: PEN_STYLE = PEN_STYLE(3u32);
pub const PS_DASHDOTDOT: PEN_STYLE = PEN_STYLE(4u32);
pub const PS_NULL: PEN_STYLE = PEN_STYLE(5u32);
pub const PS_INSIDEFRAME: PEN_STYLE = PEN_STYLE(6u32);
pub const PS_USERSTYLE: PEN_STYLE = PEN_STYLE(7u32);
pub const PS_ALTERNATE: PEN_STYLE = PEN_STYLE(8u32);
pub const PS_STYLE_MASK: PEN_STYLE = PEN_STYLE(15u32);
pub const PS_ENDCAP_ROUND: PEN_STYLE = PEN_STYLE(0u32);
pub const PS_ENDCAP_SQUARE: PEN_STYLE = PEN_STYLE(256u32);
pub const PS_ENDCAP_FLAT: PEN_STYLE = PEN_STYLE(512u32);
pub const PS_ENDCAP_MASK: PEN_STYLE = PEN_STYLE(3840u32);
pub const PS_JOIN_ROUND: PEN_STYLE = PEN_STYLE(0u32);
pub const PS_JOIN_BEVEL: PEN_STYLE = PEN_STYLE(4096u32);
pub const PS_JOIN_MITER: PEN_STYLE = PEN_STYLE(8192u32);
pub const PS_JOIN_MASK: PEN_STYLE = PEN_STYLE(61440u32);
pub const PS_TYPE_MASK: PEN_STYLE = PEN_STYLE(983040u32);
impl ::core::marker::Copy for PEN_STYLE {}
impl ::core::clone::Clone for PEN_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEN_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEN_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEN_STYLE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PEN_STYLE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PEN_STYLE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PEN_STYLE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PEN_STYLE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PEN_STYLE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for PEN_STYLE {
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
pub const PFD_DEPTH_DONTCARE: u32 = 536870912u32;
pub const PFD_DIRECT3D_ACCELERATED: u32 = 16384u32;
pub const PFD_DOUBLEBUFFER: u32 = 1u32;
pub const PFD_DOUBLEBUFFER_DONTCARE: u32 = 1073741824u32;
pub const PFD_DRAW_TO_BITMAP: u32 = 8u32;
pub const PFD_DRAW_TO_WINDOW: u32 = 4u32;
pub const PFD_GENERIC_ACCELERATED: u32 = 4096u32;
pub const PFD_GENERIC_FORMAT: u32 = 64u32;
pub const PFD_MAIN_PLANE: u32 = 0u32;
pub const PFD_NEED_PALETTE: u32 = 128u32;
pub const PFD_NEED_SYSTEM_PALETTE: u32 = 256u32;
pub const PFD_OVERLAY_PLANE: u32 = 1u32;
pub const PFD_STEREO: u32 = 2u32;
pub const PFD_STEREO_DONTCARE: u32 = 2147483648u32;
pub const PFD_SUPPORT_COMPOSITION: u32 = 32768u32;
pub const PFD_SUPPORT_DIRECTDRAW: u32 = 8192u32;
pub const PFD_SUPPORT_GDI: u32 = 16u32;
pub const PFD_SUPPORT_OPENGL: u32 = 32u32;
pub const PFD_SWAP_COPY: u32 = 1024u32;
pub const PFD_SWAP_EXCHANGE: u32 = 512u32;
pub const PFD_SWAP_LAYER_BUFFERS: u32 = 2048u32;
pub const PFD_TYPE_COLORINDEX: u32 = 1u32;
pub const PFD_TYPE_RGBA: u32 = 0u32;
pub const PFD_UNDERLAY_PLANE: i32 = -1i32;
pub struct POINTFX {
    pub x: FIXED,
    pub y: FIXED,
}
impl ::core::marker::Copy for POINTFX {}
impl ::core::clone::Clone for POINTFX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for POINTFX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POINTFX")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}
impl ::core::cmp::PartialEq for POINTFX {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for POINTFX {}
impl FromIntoMemory for POINTFX {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_x = <FIXED as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_y = <FIXED as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self { x: f_x, y: f_y }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.x, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.y, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub const POLYFILL_LAST: u32 = 2u32;
pub struct POLYTEXTA {
    pub x: i32,
    pub y: i32,
    pub n: u32,
    pub lpstr: PCSTR,
    pub uiFlags: u32,
    pub rcl: super::super::Foundation::RECT,
    pub pdx: MutPtr<i32>,
}
impl ::core::marker::Copy for POLYTEXTA {}
impl ::core::clone::Clone for POLYTEXTA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for POLYTEXTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLYTEXTA")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("n", &self.n)
            .field("lpstr", &self.lpstr)
            .field("uiFlags", &self.uiFlags)
            .field("rcl", &self.rcl)
            .field("pdx", &self.pdx)
            .finish()
    }
}
impl ::core::cmp::PartialEq for POLYTEXTA {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
            && self.y == other.y
            && self.n == other.n
            && self.lpstr == other.lpstr
            && self.uiFlags == other.uiFlags
            && self.rcl == other.rcl
            && self.pdx == other.pdx
    }
}
impl ::core::cmp::Eq for POLYTEXTA {}
impl FromIntoMemory for POLYTEXTA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_x = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_y = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_n = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpstr = <PCSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_uiFlags = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_rcl =
            <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[20..20 + 16]);
        let f_pdx = <MutPtr<i32> as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        Self {
            x: f_x,
            y: f_y,
            n: f_n,
            lpstr: f_lpstr,
            uiFlags: f_uiFlags,
            rcl: f_rcl,
            pdx: f_pdx,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.x, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.y, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.n, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpstr, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.uiFlags, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.rcl, &mut into[20..20 + 16]);
        FromIntoMemory::into_bytes(self.pdx, &mut into[36..36 + 4]);
    }
    fn size() -> usize {
        40
    }
}
pub struct POLYTEXTW {
    pub x: i32,
    pub y: i32,
    pub n: u32,
    pub lpstr: PCWSTR,
    pub uiFlags: u32,
    pub rcl: super::super::Foundation::RECT,
    pub pdx: MutPtr<i32>,
}
impl ::core::marker::Copy for POLYTEXTW {}
impl ::core::clone::Clone for POLYTEXTW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for POLYTEXTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLYTEXTW")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("n", &self.n)
            .field("lpstr", &self.lpstr)
            .field("uiFlags", &self.uiFlags)
            .field("rcl", &self.rcl)
            .field("pdx", &self.pdx)
            .finish()
    }
}
impl ::core::cmp::PartialEq for POLYTEXTW {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
            && self.y == other.y
            && self.n == other.n
            && self.lpstr == other.lpstr
            && self.uiFlags == other.uiFlags
            && self.rcl == other.rcl
            && self.pdx == other.pdx
    }
}
impl ::core::cmp::Eq for POLYTEXTW {}
impl FromIntoMemory for POLYTEXTW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_x = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_y = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_n = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpstr = <PCWSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_uiFlags = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_rcl =
            <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[20..20 + 16]);
        let f_pdx = <MutPtr<i32> as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        Self {
            x: f_x,
            y: f_y,
            n: f_n,
            lpstr: f_lpstr,
            uiFlags: f_uiFlags,
            rcl: f_rcl,
            pdx: f_pdx,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.x, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.y, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.n, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpstr, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.uiFlags, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.rcl, &mut into[20..20 + 16]);
        FromIntoMemory::into_bytes(self.pdx, &mut into[36..36 + 4]);
    }
    fn size() -> usize {
        40
    }
}
pub const POSTSCRIPT_DATA: u32 = 37u32;
pub const POSTSCRIPT_IDENTIFY: u32 = 4117u32;
pub const POSTSCRIPT_IGNORE: u32 = 38u32;
pub const POSTSCRIPT_INJECTION: u32 = 4118u32;
pub const POSTSCRIPT_PASSTHROUGH: u32 = 4115u32;
pub const PRINTRATEUNIT_CPS: u32 = 2u32;
pub const PRINTRATEUNIT_IPM: u32 = 4u32;
pub const PRINTRATEUNIT_LPM: u32 = 3u32;
pub const PRINTRATEUNIT_PPM: u32 = 1u32;
pub const PR_JOBSTATUS: u32 = 0u32;
pub const PSIDENT_GDICENTRIC: u32 = 0u32;
pub const PSIDENT_PSCENTRIC: u32 = 1u32;
pub const PSINJECT_DLFONT: u32 = 3722304989u32;
pub const PSPROTOCOL_ASCII: u32 = 0u32;
pub const PSPROTOCOL_BCP: u32 = 1u32;
pub const PSPROTOCOL_BINARY: u32 = 3u32;
pub const PSPROTOCOL_TBCP: u32 = 2u32;
pub const PT_BEZIERTO: u32 = 4u32;
pub const PT_CLOSEFIGURE: u32 = 1u32;
pub const PT_LINETO: u32 = 2u32;
pub const PT_MOVETO: u32 = 6u32;
pub const QDC_ALL_PATHS: u32 = 1u32;
pub const QDC_DATABASE_CURRENT: u32 = 4u32;
pub const QDC_INCLUDE_HMD: u32 = 32u32;
pub const QDC_ONLY_ACTIVE_PATHS: u32 = 2u32;
pub const QDC_VIRTUAL_MODE_AWARE: u32 = 16u32;
pub const QDC_VIRTUAL_REFRESH_RATE_AWARE: u32 = 64u32;
pub const QDI_DIBTOSCREEN: u32 = 4u32;
pub const QDI_GETDIBITS: u32 = 2u32;
pub const QDI_SETDIBITS: u32 = 1u32;
pub const QDI_STRETCHDIB: u32 = 8u32;
pub const QUERYDIBSUPPORT: u32 = 3073u32;
pub const QUERYESCSUPPORT: u32 = 8u32;
pub const QUERYROPSUPPORT: u32 = 40u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct R2_MODE(pub i32);
pub const R2_BLACK: R2_MODE = R2_MODE(1i32);
pub const R2_NOTMERGEPEN: R2_MODE = R2_MODE(2i32);
pub const R2_MASKNOTPEN: R2_MODE = R2_MODE(3i32);
pub const R2_NOTCOPYPEN: R2_MODE = R2_MODE(4i32);
pub const R2_MASKPENNOT: R2_MODE = R2_MODE(5i32);
pub const R2_NOT: R2_MODE = R2_MODE(6i32);
pub const R2_XORPEN: R2_MODE = R2_MODE(7i32);
pub const R2_NOTMASKPEN: R2_MODE = R2_MODE(8i32);
pub const R2_MASKPEN: R2_MODE = R2_MODE(9i32);
pub const R2_NOTXORPEN: R2_MODE = R2_MODE(10i32);
pub const R2_NOP: R2_MODE = R2_MODE(11i32);
pub const R2_MERGENOTPEN: R2_MODE = R2_MODE(12i32);
pub const R2_COPYPEN: R2_MODE = R2_MODE(13i32);
pub const R2_MERGEPENNOT: R2_MODE = R2_MODE(14i32);
pub const R2_MERGEPEN: R2_MODE = R2_MODE(15i32);
pub const R2_WHITE: R2_MODE = R2_MODE(16i32);
pub const R2_LAST: R2_MODE = R2_MODE(16i32);
impl ::core::marker::Copy for R2_MODE {}
impl ::core::clone::Clone for R2_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for R2_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for R2_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("R2_MODE").field(&self.0).finish()
    }
}
impl FromIntoMemory for R2_MODE {
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
pub struct RASTERIZER_STATUS {
    pub nSize: i16,
    pub wFlags: i16,
    pub nLanguageID: i16,
}
impl ::core::marker::Copy for RASTERIZER_STATUS {}
impl ::core::clone::Clone for RASTERIZER_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RASTERIZER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASTERIZER_STATUS")
            .field("nSize", &self.nSize)
            .field("wFlags", &self.wFlags)
            .field("nLanguageID", &self.nLanguageID)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RASTERIZER_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.nSize == other.nSize
            && self.wFlags == other.wFlags
            && self.nLanguageID == other.nLanguageID
    }
}
impl ::core::cmp::Eq for RASTERIZER_STATUS {}
impl FromIntoMemory for RASTERIZER_STATUS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 6);
        let f_nSize = <i16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wFlags = <i16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_nLanguageID = <i16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        Self {
            nSize: f_nSize,
            wFlags: f_wFlags,
            nLanguageID: f_nLanguageID,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 6);
        FromIntoMemory::into_bytes(self.nSize, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wFlags, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.nLanguageID, &mut into[4..4 + 2]);
    }
    fn size() -> usize {
        6
    }
}
pub const RASTER_FONTTYPE: u32 = 1u32;
pub const RC_BANDING: u32 = 2u32;
pub const RC_BIGFONT: u32 = 1024u32;
pub const RC_BITBLT: u32 = 1u32;
pub const RC_BITMAP64: u32 = 8u32;
pub const RC_DEVBITS: u32 = 32768u32;
pub const RC_DIBTODEV: u32 = 512u32;
pub const RC_DI_BITMAP: u32 = 128u32;
pub const RC_FLOODFILL: u32 = 4096u32;
pub const RC_GDI20_OUTPUT: u32 = 16u32;
pub const RC_GDI20_STATE: u32 = 32u32;
pub const RC_OP_DX_OUTPUT: u32 = 16384u32;
pub const RC_PALETTE: u32 = 256u32;
pub const RC_SAVEBITMAP: u32 = 64u32;
pub const RC_SCALING: u32 = 4u32;
pub const RC_STRETCHBLT: u32 = 2048u32;
pub const RC_STRETCHDIB: u32 = 8192u32;
pub const RDH_RECTANGLES: u32 = 1u32;
pub type READEMBEDPROC = StdCallFnPtr<
    (
        MutPtr<::core::ffi::c_void>,
        MutPtr<::core::ffi::c_void>,
        u32,
    ),
    u32,
>;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct REDRAW_WINDOW_FLAGS(pub u32);
pub const RDW_INVALIDATE: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(1u32);
pub const RDW_INTERNALPAINT: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(2u32);
pub const RDW_ERASE: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(4u32);
pub const RDW_VALIDATE: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(8u32);
pub const RDW_NOINTERNALPAINT: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(16u32);
pub const RDW_NOERASE: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(32u32);
pub const RDW_NOCHILDREN: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(64u32);
pub const RDW_ALLCHILDREN: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(128u32);
pub const RDW_UPDATENOW: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(256u32);
pub const RDW_ERASENOW: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(512u32);
pub const RDW_FRAME: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(1024u32);
pub const RDW_NOFRAME: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(2048u32);
impl ::core::marker::Copy for REDRAW_WINDOW_FLAGS {}
impl ::core::clone::Clone for REDRAW_WINDOW_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REDRAW_WINDOW_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for REDRAW_WINDOW_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REDRAW_WINDOW_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for REDRAW_WINDOW_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for REDRAW_WINDOW_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for REDRAW_WINDOW_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for REDRAW_WINDOW_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for REDRAW_WINDOW_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for REDRAW_WINDOW_FLAGS {
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
pub const RELATIVE: u32 = 2u32;
pub const RESTORE_CTM: u32 = 4100u32;
pub struct RGBQUAD {
    pub rgbBlue: u8,
    pub rgbGreen: u8,
    pub rgbRed: u8,
    pub rgbReserved: u8,
}
impl ::core::marker::Copy for RGBQUAD {}
impl ::core::clone::Clone for RGBQUAD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RGBQUAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RGBQUAD")
            .field("rgbBlue", &self.rgbBlue)
            .field("rgbGreen", &self.rgbGreen)
            .field("rgbRed", &self.rgbRed)
            .field("rgbReserved", &self.rgbReserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RGBQUAD {
    fn eq(&self, other: &Self) -> bool {
        self.rgbBlue == other.rgbBlue
            && self.rgbGreen == other.rgbGreen
            && self.rgbRed == other.rgbRed
            && self.rgbReserved == other.rgbReserved
    }
}
impl ::core::cmp::Eq for RGBQUAD {}
impl FromIntoMemory for RGBQUAD {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_rgbBlue = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_rgbGreen = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_rgbRed = <u8 as FromIntoMemory>::from_bytes(&from[2..2 + 1]);
        let f_rgbReserved = <u8 as FromIntoMemory>::from_bytes(&from[3..3 + 1]);
        Self {
            rgbBlue: f_rgbBlue,
            rgbGreen: f_rgbGreen,
            rgbRed: f_rgbRed,
            rgbReserved: f_rgbReserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.rgbBlue, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.rgbGreen, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.rgbRed, &mut into[2..2 + 1]);
        FromIntoMemory::into_bytes(self.rgbReserved, &mut into[3..3 + 1]);
    }
    fn size() -> usize {
        4
    }
}
pub struct RGBTRIPLE {
    pub rgbtBlue: u8,
    pub rgbtGreen: u8,
    pub rgbtRed: u8,
}
impl ::core::marker::Copy for RGBTRIPLE {}
impl ::core::clone::Clone for RGBTRIPLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RGBTRIPLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RGBTRIPLE")
            .field("rgbtBlue", &self.rgbtBlue)
            .field("rgbtGreen", &self.rgbtGreen)
            .field("rgbtRed", &self.rgbtRed)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RGBTRIPLE {
    fn eq(&self, other: &Self) -> bool {
        self.rgbtBlue == other.rgbtBlue
            && self.rgbtGreen == other.rgbtGreen
            && self.rgbtRed == other.rgbtRed
    }
}
impl ::core::cmp::Eq for RGBTRIPLE {}
impl FromIntoMemory for RGBTRIPLE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 3);
        let f_rgbtBlue = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_rgbtGreen = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_rgbtRed = <u8 as FromIntoMemory>::from_bytes(&from[2..2 + 1]);
        Self {
            rgbtBlue: f_rgbtBlue,
            rgbtGreen: f_rgbtGreen,
            rgbtRed: f_rgbtRed,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 3);
        FromIntoMemory::into_bytes(self.rgbtBlue, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.rgbtGreen, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.rgbtRed, &mut into[2..2 + 1]);
    }
    fn size() -> usize {
        3
    }
}
pub struct RGNDATA {
    pub rdh: RGNDATAHEADER,
    pub Buffer: [super::super::Foundation::CHAR; 1],
}
impl ::core::marker::Copy for RGNDATA {}
impl ::core::clone::Clone for RGNDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RGNDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RGNDATA")
            .field("rdh", &self.rdh)
            .field("Buffer", &self.Buffer)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RGNDATA {
    fn eq(&self, other: &Self) -> bool {
        self.rdh == other.rdh && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for RGNDATA {}
impl FromIntoMemory for RGNDATA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 36);
        let f_rdh = <RGNDATAHEADER as FromIntoMemory>::from_bytes(&from[0..0 + 32]);
        let f_Buffer =
            <[super::super::Foundation::CHAR; 1] as FromIntoMemory>::from_bytes(&from[32..32 + 1]);
        Self {
            rdh: f_rdh,
            Buffer: f_Buffer,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 36);
        FromIntoMemory::into_bytes(self.rdh, &mut into[0..0 + 32]);
        FromIntoMemory::into_bytes(self.Buffer, &mut into[32..32 + 1]);
    }
    fn size() -> usize {
        36
    }
}
pub struct RGNDATAHEADER {
    pub dwSize: u32,
    pub iType: u32,
    pub nCount: u32,
    pub nRgnSize: u32,
    pub rcBound: super::super::Foundation::RECT,
}
impl ::core::marker::Copy for RGNDATAHEADER {}
impl ::core::clone::Clone for RGNDATAHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RGNDATAHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RGNDATAHEADER")
            .field("dwSize", &self.dwSize)
            .field("iType", &self.iType)
            .field("nCount", &self.nCount)
            .field("nRgnSize", &self.nRgnSize)
            .field("rcBound", &self.rcBound)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RGNDATAHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.iType == other.iType
            && self.nCount == other.nCount
            && self.nRgnSize == other.nRgnSize
            && self.rcBound == other.rcBound
    }
}
impl ::core::cmp::Eq for RGNDATAHEADER {}
impl FromIntoMemory for RGNDATAHEADER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_iType = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_nCount = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_nRgnSize = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_rcBound =
            <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[16..16 + 16]);
        Self {
            dwSize: f_dwSize,
            iType: f_iType,
            nCount: f_nCount,
            nRgnSize: f_nRgnSize,
            rcBound: f_rcBound,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.iType, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.nCount, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.nRgnSize, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.rcBound, &mut into[16..16 + 16]);
    }
    fn size() -> usize {
        32
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RGN_COMBINE_MODE(pub i32);
pub const RGN_AND: RGN_COMBINE_MODE = RGN_COMBINE_MODE(1i32);
pub const RGN_OR: RGN_COMBINE_MODE = RGN_COMBINE_MODE(2i32);
pub const RGN_XOR: RGN_COMBINE_MODE = RGN_COMBINE_MODE(3i32);
pub const RGN_DIFF: RGN_COMBINE_MODE = RGN_COMBINE_MODE(4i32);
pub const RGN_COPY: RGN_COMBINE_MODE = RGN_COMBINE_MODE(5i32);
pub const RGN_MIN: RGN_COMBINE_MODE = RGN_COMBINE_MODE(1i32);
pub const RGN_MAX: RGN_COMBINE_MODE = RGN_COMBINE_MODE(5i32);
impl ::core::marker::Copy for RGN_COMBINE_MODE {}
impl ::core::clone::Clone for RGN_COMBINE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RGN_COMBINE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RGN_COMBINE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RGN_COMBINE_MODE").field(&self.0).finish()
    }
}
impl FromIntoMemory for RGN_COMBINE_MODE {
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
pub const RGN_ERROR: u32 = 0u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ROP_CODE(pub u32);
pub const SRCCOPY: ROP_CODE = ROP_CODE(13369376u32);
pub const SRCPAINT: ROP_CODE = ROP_CODE(15597702u32);
pub const SRCAND: ROP_CODE = ROP_CODE(8913094u32);
pub const SRCINVERT: ROP_CODE = ROP_CODE(6684742u32);
pub const SRCERASE: ROP_CODE = ROP_CODE(4457256u32);
pub const NOTSRCCOPY: ROP_CODE = ROP_CODE(3342344u32);
pub const NOTSRCERASE: ROP_CODE = ROP_CODE(1114278u32);
pub const MERGECOPY: ROP_CODE = ROP_CODE(12583114u32);
pub const MERGEPAINT: ROP_CODE = ROP_CODE(12255782u32);
pub const PATCOPY: ROP_CODE = ROP_CODE(15728673u32);
pub const PATPAINT: ROP_CODE = ROP_CODE(16452105u32);
pub const PATINVERT: ROP_CODE = ROP_CODE(5898313u32);
pub const DSTINVERT: ROP_CODE = ROP_CODE(5570569u32);
pub const BLACKNESS: ROP_CODE = ROP_CODE(66u32);
pub const WHITENESS: ROP_CODE = ROP_CODE(16711778u32);
pub const NOMIRRORBITMAP: ROP_CODE = ROP_CODE(2147483648u32);
pub const CAPTUREBLT: ROP_CODE = ROP_CODE(1073741824u32);
impl ::core::marker::Copy for ROP_CODE {}
impl ::core::clone::Clone for ROP_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ROP_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ROP_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ROP_CODE").field(&self.0).finish()
    }
}
impl FromIntoMemory for ROP_CODE {
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
pub const RUSSIAN_CHARSET: u32 = 204u32;
pub const SAVE_CTM: u32 = 4101u32;
pub const SB_CONST_ALPHA: u32 = 1u32;
pub const SB_GRAD_RECT: u32 = 16u32;
pub const SB_GRAD_TRI: u32 = 32u32;
pub const SB_NONE: u32 = 0u32;
pub const SB_PIXEL_ALPHA: u32 = 2u32;
pub const SB_PREMULT_ALPHA: u32 = 4u32;
pub const SC_SCREENSAVE: u32 = 61760u32;
pub const SDC_ALLOW_CHANGES: u32 = 1024u32;
pub const SDC_ALLOW_PATH_ORDER_CHANGES: u32 = 8192u32;
pub const SDC_APPLY: u32 = 128u32;
pub const SDC_FORCE_MODE_ENUMERATION: u32 = 4096u32;
pub const SDC_NO_OPTIMIZATION: u32 = 256u32;
pub const SDC_PATH_PERSIST_IF_REQUIRED: u32 = 2048u32;
pub const SDC_SAVE_TO_DATABASE: u32 = 512u32;
pub const SDC_TOPOLOGY_CLONE: u32 = 2u32;
pub const SDC_TOPOLOGY_EXTEND: u32 = 4u32;
pub const SDC_TOPOLOGY_EXTERNAL: u32 = 8u32;
pub const SDC_TOPOLOGY_INTERNAL: u32 = 1u32;
pub const SDC_TOPOLOGY_SUPPLIED: u32 = 16u32;
pub const SDC_USE_SUPPLIED_DISPLAY_CONFIG: u32 = 32u32;
pub const SDC_VALIDATE: u32 = 64u32;
pub const SDC_VIRTUAL_MODE_AWARE: u32 = 32768u32;
pub const SDC_VIRTUAL_REFRESH_RATE_AWARE: u32 = 131072u32;
pub const SELECTDIB: u32 = 41u32;
pub const SELECTPAPERSOURCE: u32 = 18u32;
pub const SETABORTPROC: u32 = 9u32;
pub const SETALLJUSTVALUES: u32 = 771u32;
pub const SETCHARSET: u32 = 772u32;
pub const SETCOLORTABLE: u32 = 4u32;
pub const SETCOPYCOUNT: u32 = 17u32;
pub const SETDIBSCALING: u32 = 32u32;
pub const SETICMPROFILE_EMBEDED: u32 = 1u32;
pub const SETKERNTRACK: u32 = 770u32;
pub const SETLINECAP: u32 = 21u32;
pub const SETLINEJOIN: u32 = 22u32;
pub const SETMITERLIMIT: u32 = 23u32;
pub const SET_ARC_DIRECTION: u32 = 4102u32;
pub const SET_BACKGROUND_COLOR: u32 = 4103u32;
pub const SET_BOUNDS: u32 = 4109u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SET_BOUNDS_RECT_FLAGS(pub u32);
pub const DCB_ACCUMULATE: SET_BOUNDS_RECT_FLAGS = SET_BOUNDS_RECT_FLAGS(2u32);
pub const DCB_DISABLE: SET_BOUNDS_RECT_FLAGS = SET_BOUNDS_RECT_FLAGS(8u32);
pub const DCB_ENABLE: SET_BOUNDS_RECT_FLAGS = SET_BOUNDS_RECT_FLAGS(4u32);
pub const DCB_RESET: SET_BOUNDS_RECT_FLAGS = SET_BOUNDS_RECT_FLAGS(1u32);
impl ::core::marker::Copy for SET_BOUNDS_RECT_FLAGS {}
impl ::core::clone::Clone for SET_BOUNDS_RECT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SET_BOUNDS_RECT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SET_BOUNDS_RECT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SET_BOUNDS_RECT_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for SET_BOUNDS_RECT_FLAGS {
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
pub const SET_CLIP_BOX: u32 = 4108u32;
pub const SET_MIRROR_MODE: u32 = 4110u32;
pub const SET_POLY_MODE: u32 = 4104u32;
pub const SET_SCREEN_ANGLE: u32 = 4105u32;
pub const SET_SPREAD: u32 = 4106u32;
pub const SHIFTJIS_CHARSET: u32 = 128u32;
pub const SIMPLEREGION: u32 = 2u32;
pub const SPCLPASSTHROUGH2: u32 = 4568u32;
pub const SP_APPABORT: i32 = -2i32;
pub const SP_ERROR: i32 = -1i32;
pub const SP_NOTREPORTED: u32 = 16384u32;
pub const SP_OUTOFDISK: i32 = -4i32;
pub const SP_OUTOFMEMORY: i32 = -5i32;
pub const SP_USERABORT: i32 = -3i32;
pub const STARTDOC: u32 = 10u32;
pub const STOCK_LAST: u32 = 19u32;
pub const STRETCHBLT: u32 = 2048u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STRETCH_BLT_MODE(pub u32);
pub const BLACKONWHITE: STRETCH_BLT_MODE = STRETCH_BLT_MODE(1u32);
pub const COLORONCOLOR: STRETCH_BLT_MODE = STRETCH_BLT_MODE(3u32);
pub const HALFTONE: STRETCH_BLT_MODE = STRETCH_BLT_MODE(4u32);
pub const STRETCH_ANDSCANS: STRETCH_BLT_MODE = STRETCH_BLT_MODE(1u32);
pub const STRETCH_DELETESCANS: STRETCH_BLT_MODE = STRETCH_BLT_MODE(3u32);
pub const STRETCH_HALFTONE: STRETCH_BLT_MODE = STRETCH_BLT_MODE(4u32);
pub const STRETCH_ORSCANS: STRETCH_BLT_MODE = STRETCH_BLT_MODE(2u32);
pub const WHITEONBLACK: STRETCH_BLT_MODE = STRETCH_BLT_MODE(2u32);
impl ::core::marker::Copy for STRETCH_BLT_MODE {}
impl ::core::clone::Clone for STRETCH_BLT_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STRETCH_BLT_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STRETCH_BLT_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STRETCH_BLT_MODE").field(&self.0).finish()
    }
}
impl FromIntoMemory for STRETCH_BLT_MODE {
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
pub const SYMBOL_CHARSET: u32 = 2u32;
pub const SYSPAL_ERROR: u32 = 0u32;
pub const SYSRGN: u32 = 4u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SYSTEM_PALETTE_USE(pub u32);
pub const SYSPAL_NOSTATIC: SYSTEM_PALETTE_USE = SYSTEM_PALETTE_USE(2u32);
pub const SYSPAL_NOSTATIC256: SYSTEM_PALETTE_USE = SYSTEM_PALETTE_USE(3u32);
pub const SYSPAL_STATIC: SYSTEM_PALETTE_USE = SYSTEM_PALETTE_USE(1u32);
impl ::core::marker::Copy for SYSTEM_PALETTE_USE {}
impl ::core::clone::Clone for SYSTEM_PALETTE_USE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYSTEM_PALETTE_USE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYSTEM_PALETTE_USE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSTEM_PALETTE_USE").field(&self.0).finish()
    }
}
impl FromIntoMemory for SYSTEM_PALETTE_USE {
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
pub const TC_CP_STROKE: u32 = 4u32;
pub const TC_CR_90: u32 = 8u32;
pub const TC_CR_ANY: u32 = 16u32;
pub const TC_EA_DOUBLE: u32 = 512u32;
pub const TC_IA_ABLE: u32 = 1024u32;
pub const TC_OP_CHARACTER: u32 = 1u32;
pub const TC_OP_STROKE: u32 = 2u32;
pub const TC_RA_ABLE: u32 = 8192u32;
pub const TC_RESERVED: u32 = 32768u32;
pub const TC_SA_CONTIN: u32 = 256u32;
pub const TC_SA_DOUBLE: u32 = 64u32;
pub const TC_SA_INTEGER: u32 = 128u32;
pub const TC_SCROLLBLT: u32 = 65536u32;
pub const TC_SF_X_YINDEP: u32 = 32u32;
pub const TC_SO_ABLE: u32 = 4096u32;
pub const TC_UA_ABLE: u32 = 2048u32;
pub const TC_VA_ABLE: u32 = 16384u32;
pub struct TEXTMETRICA {
    pub tmHeight: i32,
    pub tmAscent: i32,
    pub tmDescent: i32,
    pub tmInternalLeading: i32,
    pub tmExternalLeading: i32,
    pub tmAveCharWidth: i32,
    pub tmMaxCharWidth: i32,
    pub tmWeight: i32,
    pub tmOverhang: i32,
    pub tmDigitizedAspectX: i32,
    pub tmDigitizedAspectY: i32,
    pub tmFirstChar: u8,
    pub tmLastChar: u8,
    pub tmDefaultChar: u8,
    pub tmBreakChar: u8,
    pub tmItalic: u8,
    pub tmUnderlined: u8,
    pub tmStruckOut: u8,
    pub tmPitchAndFamily: u8,
    pub tmCharSet: u8,
}
impl ::core::marker::Copy for TEXTMETRICA {}
impl ::core::clone::Clone for TEXTMETRICA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TEXTMETRICA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TEXTMETRICA")
            .field("tmHeight", &self.tmHeight)
            .field("tmAscent", &self.tmAscent)
            .field("tmDescent", &self.tmDescent)
            .field("tmInternalLeading", &self.tmInternalLeading)
            .field("tmExternalLeading", &self.tmExternalLeading)
            .field("tmAveCharWidth", &self.tmAveCharWidth)
            .field("tmMaxCharWidth", &self.tmMaxCharWidth)
            .field("tmWeight", &self.tmWeight)
            .field("tmOverhang", &self.tmOverhang)
            .field("tmDigitizedAspectX", &self.tmDigitizedAspectX)
            .field("tmDigitizedAspectY", &self.tmDigitizedAspectY)
            .field("tmFirstChar", &self.tmFirstChar)
            .field("tmLastChar", &self.tmLastChar)
            .field("tmDefaultChar", &self.tmDefaultChar)
            .field("tmBreakChar", &self.tmBreakChar)
            .field("tmItalic", &self.tmItalic)
            .field("tmUnderlined", &self.tmUnderlined)
            .field("tmStruckOut", &self.tmStruckOut)
            .field("tmPitchAndFamily", &self.tmPitchAndFamily)
            .field("tmCharSet", &self.tmCharSet)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TEXTMETRICA {
    fn eq(&self, other: &Self) -> bool {
        self.tmHeight == other.tmHeight
            && self.tmAscent == other.tmAscent
            && self.tmDescent == other.tmDescent
            && self.tmInternalLeading == other.tmInternalLeading
            && self.tmExternalLeading == other.tmExternalLeading
            && self.tmAveCharWidth == other.tmAveCharWidth
            && self.tmMaxCharWidth == other.tmMaxCharWidth
            && self.tmWeight == other.tmWeight
            && self.tmOverhang == other.tmOverhang
            && self.tmDigitizedAspectX == other.tmDigitizedAspectX
            && self.tmDigitizedAspectY == other.tmDigitizedAspectY
            && self.tmFirstChar == other.tmFirstChar
            && self.tmLastChar == other.tmLastChar
            && self.tmDefaultChar == other.tmDefaultChar
            && self.tmBreakChar == other.tmBreakChar
            && self.tmItalic == other.tmItalic
            && self.tmUnderlined == other.tmUnderlined
            && self.tmStruckOut == other.tmStruckOut
            && self.tmPitchAndFamily == other.tmPitchAndFamily
            && self.tmCharSet == other.tmCharSet
    }
}
impl ::core::cmp::Eq for TEXTMETRICA {}
impl FromIntoMemory for TEXTMETRICA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 56);
        let f_tmHeight = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_tmAscent = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_tmDescent = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_tmInternalLeading = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_tmExternalLeading = <i32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_tmAveCharWidth = <i32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_tmMaxCharWidth = <i32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_tmWeight = <i32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_tmOverhang = <i32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_tmDigitizedAspectX = <i32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_tmDigitizedAspectY = <i32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_tmFirstChar = <u8 as FromIntoMemory>::from_bytes(&from[44..44 + 1]);
        let f_tmLastChar = <u8 as FromIntoMemory>::from_bytes(&from[45..45 + 1]);
        let f_tmDefaultChar = <u8 as FromIntoMemory>::from_bytes(&from[46..46 + 1]);
        let f_tmBreakChar = <u8 as FromIntoMemory>::from_bytes(&from[47..47 + 1]);
        let f_tmItalic = <u8 as FromIntoMemory>::from_bytes(&from[48..48 + 1]);
        let f_tmUnderlined = <u8 as FromIntoMemory>::from_bytes(&from[49..49 + 1]);
        let f_tmStruckOut = <u8 as FromIntoMemory>::from_bytes(&from[50..50 + 1]);
        let f_tmPitchAndFamily = <u8 as FromIntoMemory>::from_bytes(&from[51..51 + 1]);
        let f_tmCharSet = <u8 as FromIntoMemory>::from_bytes(&from[52..52 + 1]);
        Self {
            tmHeight: f_tmHeight,
            tmAscent: f_tmAscent,
            tmDescent: f_tmDescent,
            tmInternalLeading: f_tmInternalLeading,
            tmExternalLeading: f_tmExternalLeading,
            tmAveCharWidth: f_tmAveCharWidth,
            tmMaxCharWidth: f_tmMaxCharWidth,
            tmWeight: f_tmWeight,
            tmOverhang: f_tmOverhang,
            tmDigitizedAspectX: f_tmDigitizedAspectX,
            tmDigitizedAspectY: f_tmDigitizedAspectY,
            tmFirstChar: f_tmFirstChar,
            tmLastChar: f_tmLastChar,
            tmDefaultChar: f_tmDefaultChar,
            tmBreakChar: f_tmBreakChar,
            tmItalic: f_tmItalic,
            tmUnderlined: f_tmUnderlined,
            tmStruckOut: f_tmStruckOut,
            tmPitchAndFamily: f_tmPitchAndFamily,
            tmCharSet: f_tmCharSet,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 56);
        FromIntoMemory::into_bytes(self.tmHeight, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.tmAscent, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.tmDescent, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.tmInternalLeading, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.tmExternalLeading, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.tmAveCharWidth, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.tmMaxCharWidth, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.tmWeight, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.tmOverhang, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.tmDigitizedAspectX, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.tmDigitizedAspectY, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.tmFirstChar, &mut into[44..44 + 1]);
        FromIntoMemory::into_bytes(self.tmLastChar, &mut into[45..45 + 1]);
        FromIntoMemory::into_bytes(self.tmDefaultChar, &mut into[46..46 + 1]);
        FromIntoMemory::into_bytes(self.tmBreakChar, &mut into[47..47 + 1]);
        FromIntoMemory::into_bytes(self.tmItalic, &mut into[48..48 + 1]);
        FromIntoMemory::into_bytes(self.tmUnderlined, &mut into[49..49 + 1]);
        FromIntoMemory::into_bytes(self.tmStruckOut, &mut into[50..50 + 1]);
        FromIntoMemory::into_bytes(self.tmPitchAndFamily, &mut into[51..51 + 1]);
        FromIntoMemory::into_bytes(self.tmCharSet, &mut into[52..52 + 1]);
    }
    fn size() -> usize {
        56
    }
}
pub struct TEXTMETRICW {
    pub tmHeight: i32,
    pub tmAscent: i32,
    pub tmDescent: i32,
    pub tmInternalLeading: i32,
    pub tmExternalLeading: i32,
    pub tmAveCharWidth: i32,
    pub tmMaxCharWidth: i32,
    pub tmWeight: i32,
    pub tmOverhang: i32,
    pub tmDigitizedAspectX: i32,
    pub tmDigitizedAspectY: i32,
    pub tmFirstChar: u16,
    pub tmLastChar: u16,
    pub tmDefaultChar: u16,
    pub tmBreakChar: u16,
    pub tmItalic: u8,
    pub tmUnderlined: u8,
    pub tmStruckOut: u8,
    pub tmPitchAndFamily: u8,
    pub tmCharSet: u8,
}
impl ::core::marker::Copy for TEXTMETRICW {}
impl ::core::clone::Clone for TEXTMETRICW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TEXTMETRICW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TEXTMETRICW")
            .field("tmHeight", &self.tmHeight)
            .field("tmAscent", &self.tmAscent)
            .field("tmDescent", &self.tmDescent)
            .field("tmInternalLeading", &self.tmInternalLeading)
            .field("tmExternalLeading", &self.tmExternalLeading)
            .field("tmAveCharWidth", &self.tmAveCharWidth)
            .field("tmMaxCharWidth", &self.tmMaxCharWidth)
            .field("tmWeight", &self.tmWeight)
            .field("tmOverhang", &self.tmOverhang)
            .field("tmDigitizedAspectX", &self.tmDigitizedAspectX)
            .field("tmDigitizedAspectY", &self.tmDigitizedAspectY)
            .field("tmFirstChar", &self.tmFirstChar)
            .field("tmLastChar", &self.tmLastChar)
            .field("tmDefaultChar", &self.tmDefaultChar)
            .field("tmBreakChar", &self.tmBreakChar)
            .field("tmItalic", &self.tmItalic)
            .field("tmUnderlined", &self.tmUnderlined)
            .field("tmStruckOut", &self.tmStruckOut)
            .field("tmPitchAndFamily", &self.tmPitchAndFamily)
            .field("tmCharSet", &self.tmCharSet)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TEXTMETRICW {
    fn eq(&self, other: &Self) -> bool {
        self.tmHeight == other.tmHeight
            && self.tmAscent == other.tmAscent
            && self.tmDescent == other.tmDescent
            && self.tmInternalLeading == other.tmInternalLeading
            && self.tmExternalLeading == other.tmExternalLeading
            && self.tmAveCharWidth == other.tmAveCharWidth
            && self.tmMaxCharWidth == other.tmMaxCharWidth
            && self.tmWeight == other.tmWeight
            && self.tmOverhang == other.tmOverhang
            && self.tmDigitizedAspectX == other.tmDigitizedAspectX
            && self.tmDigitizedAspectY == other.tmDigitizedAspectY
            && self.tmFirstChar == other.tmFirstChar
            && self.tmLastChar == other.tmLastChar
            && self.tmDefaultChar == other.tmDefaultChar
            && self.tmBreakChar == other.tmBreakChar
            && self.tmItalic == other.tmItalic
            && self.tmUnderlined == other.tmUnderlined
            && self.tmStruckOut == other.tmStruckOut
            && self.tmPitchAndFamily == other.tmPitchAndFamily
            && self.tmCharSet == other.tmCharSet
    }
}
impl ::core::cmp::Eq for TEXTMETRICW {}
impl FromIntoMemory for TEXTMETRICW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 56);
        let f_tmHeight = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_tmAscent = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_tmDescent = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_tmInternalLeading = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_tmExternalLeading = <i32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_tmAveCharWidth = <i32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_tmMaxCharWidth = <i32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_tmWeight = <i32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_tmOverhang = <i32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_tmDigitizedAspectX = <i32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_tmDigitizedAspectY = <i32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_tmFirstChar = <u16 as FromIntoMemory>::from_bytes(&from[44..44 + 1]);
        let f_tmLastChar = <u16 as FromIntoMemory>::from_bytes(&from[45..45 + 1]);
        let f_tmDefaultChar = <u16 as FromIntoMemory>::from_bytes(&from[46..46 + 1]);
        let f_tmBreakChar = <u16 as FromIntoMemory>::from_bytes(&from[47..47 + 1]);
        let f_tmItalic = <u8 as FromIntoMemory>::from_bytes(&from[48..48 + 1]);
        let f_tmUnderlined = <u8 as FromIntoMemory>::from_bytes(&from[49..49 + 1]);
        let f_tmStruckOut = <u8 as FromIntoMemory>::from_bytes(&from[50..50 + 1]);
        let f_tmPitchAndFamily = <u8 as FromIntoMemory>::from_bytes(&from[51..51 + 1]);
        let f_tmCharSet = <u8 as FromIntoMemory>::from_bytes(&from[52..52 + 1]);
        Self {
            tmHeight: f_tmHeight,
            tmAscent: f_tmAscent,
            tmDescent: f_tmDescent,
            tmInternalLeading: f_tmInternalLeading,
            tmExternalLeading: f_tmExternalLeading,
            tmAveCharWidth: f_tmAveCharWidth,
            tmMaxCharWidth: f_tmMaxCharWidth,
            tmWeight: f_tmWeight,
            tmOverhang: f_tmOverhang,
            tmDigitizedAspectX: f_tmDigitizedAspectX,
            tmDigitizedAspectY: f_tmDigitizedAspectY,
            tmFirstChar: f_tmFirstChar,
            tmLastChar: f_tmLastChar,
            tmDefaultChar: f_tmDefaultChar,
            tmBreakChar: f_tmBreakChar,
            tmItalic: f_tmItalic,
            tmUnderlined: f_tmUnderlined,
            tmStruckOut: f_tmStruckOut,
            tmPitchAndFamily: f_tmPitchAndFamily,
            tmCharSet: f_tmCharSet,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 56);
        FromIntoMemory::into_bytes(self.tmHeight, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.tmAscent, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.tmDescent, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.tmInternalLeading, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.tmExternalLeading, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.tmAveCharWidth, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.tmMaxCharWidth, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.tmWeight, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.tmOverhang, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.tmDigitizedAspectX, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.tmDigitizedAspectY, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.tmFirstChar, &mut into[44..44 + 1]);
        FromIntoMemory::into_bytes(self.tmLastChar, &mut into[45..45 + 1]);
        FromIntoMemory::into_bytes(self.tmDefaultChar, &mut into[46..46 + 1]);
        FromIntoMemory::into_bytes(self.tmBreakChar, &mut into[47..47 + 1]);
        FromIntoMemory::into_bytes(self.tmItalic, &mut into[48..48 + 1]);
        FromIntoMemory::into_bytes(self.tmUnderlined, &mut into[49..49 + 1]);
        FromIntoMemory::into_bytes(self.tmStruckOut, &mut into[50..50 + 1]);
        FromIntoMemory::into_bytes(self.tmPitchAndFamily, &mut into[51..51 + 1]);
        FromIntoMemory::into_bytes(self.tmCharSet, &mut into[52..52 + 1]);
    }
    fn size() -> usize {
        56
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TEXT_ALIGN_OPTIONS(pub u32);
pub const TA_NOUPDATECP: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(0u32);
pub const TA_UPDATECP: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(1u32);
pub const TA_LEFT: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(0u32);
pub const TA_RIGHT: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(2u32);
pub const TA_CENTER: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(6u32);
pub const TA_TOP: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(0u32);
pub const TA_BOTTOM: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(8u32);
pub const TA_BASELINE: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(24u32);
pub const TA_RTLREADING: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(256u32);
pub const TA_MASK: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(287u32);
pub const VTA_BASELINE: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(24u32);
pub const VTA_LEFT: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(8u32);
pub const VTA_RIGHT: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(0u32);
pub const VTA_CENTER: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(6u32);
pub const VTA_BOTTOM: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(2u32);
pub const VTA_TOP: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(0u32);
impl ::core::marker::Copy for TEXT_ALIGN_OPTIONS {}
impl ::core::clone::Clone for TEXT_ALIGN_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TEXT_ALIGN_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TEXT_ALIGN_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TEXT_ALIGN_OPTIONS").field(&self.0).finish()
    }
}
impl FromIntoMemory for TEXT_ALIGN_OPTIONS {
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
pub const THAI_CHARSET: u32 = 222u32;
pub const TMPF_DEVICE: u32 = 8u32;
pub const TMPF_FIXED_PITCH: u32 = 1u32;
pub const TMPF_TRUETYPE: u32 = 4u32;
pub const TMPF_VECTOR: u32 = 2u32;
pub const TRANSFORM_CTM: u32 = 4107u32;
pub struct TRIVERTEX {
    pub x: i32,
    pub y: i32,
    pub Red: u16,
    pub Green: u16,
    pub Blue: u16,
    pub Alpha: u16,
}
impl ::core::marker::Copy for TRIVERTEX {}
impl ::core::clone::Clone for TRIVERTEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRIVERTEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRIVERTEX")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("Red", &self.Red)
            .field("Green", &self.Green)
            .field("Blue", &self.Blue)
            .field("Alpha", &self.Alpha)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TRIVERTEX {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
            && self.y == other.y
            && self.Red == other.Red
            && self.Green == other.Green
            && self.Blue == other.Blue
            && self.Alpha == other.Alpha
    }
}
impl ::core::cmp::Eq for TRIVERTEX {}
impl FromIntoMemory for TRIVERTEX {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_x = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_y = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Red = <u16 as FromIntoMemory>::from_bytes(&from[8..8 + 2]);
        let f_Green = <u16 as FromIntoMemory>::from_bytes(&from[10..10 + 2]);
        let f_Blue = <u16 as FromIntoMemory>::from_bytes(&from[12..12 + 2]);
        let f_Alpha = <u16 as FromIntoMemory>::from_bytes(&from[14..14 + 2]);
        Self {
            x: f_x,
            y: f_y,
            Red: f_Red,
            Green: f_Green,
            Blue: f_Blue,
            Alpha: f_Alpha,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.x, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.y, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Red, &mut into[8..8 + 2]);
        FromIntoMemory::into_bytes(self.Green, &mut into[10..10 + 2]);
        FromIntoMemory::into_bytes(self.Blue, &mut into[12..12 + 2]);
        FromIntoMemory::into_bytes(self.Alpha, &mut into[14..14 + 2]);
    }
    fn size() -> usize {
        16
    }
}
pub const TRUETYPE_FONTTYPE: u32 = 4u32;
pub const TTDELETE_DONTREMOVEFONT: u32 = 1u32;
pub struct TTEMBEDINFO {
    pub usStructSize: u16,
    pub usRootStrSize: u16,
    pub pusRootStr: MutPtr<u16>,
}
impl ::core::marker::Copy for TTEMBEDINFO {}
impl ::core::clone::Clone for TTEMBEDINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TTEMBEDINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TTEMBEDINFO")
            .field("usStructSize", &self.usStructSize)
            .field("usRootStrSize", &self.usRootStrSize)
            .field("pusRootStr", &self.pusRootStr)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TTEMBEDINFO {
    fn eq(&self, other: &Self) -> bool {
        self.usStructSize == other.usStructSize
            && self.usRootStrSize == other.usRootStrSize
            && self.pusRootStr == other.pusRootStr
    }
}
impl ::core::cmp::Eq for TTEMBEDINFO {}
impl FromIntoMemory for TTEMBEDINFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_usStructSize = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_usRootStrSize = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_pusRootStr = <MutPtr<u16> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            usStructSize: f_usStructSize,
            usRootStrSize: f_usRootStrSize,
            pusRootStr: f_pusRootStr,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.usStructSize, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.usRootStrSize, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.pusRootStr, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub const TTEMBED_EUDCEMBEDDED: u32 = 2u32;
pub const TTEMBED_FAILIFVARIATIONSIMULATED: u32 = 16u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TTEMBED_FLAGS(pub u32);
pub const TTEMBED_EMBEDEUDC: TTEMBED_FLAGS = TTEMBED_FLAGS(32u32);
pub const TTEMBED_RAW: TTEMBED_FLAGS = TTEMBED_FLAGS(0u32);
pub const TTEMBED_SUBSET: TTEMBED_FLAGS = TTEMBED_FLAGS(1u32);
pub const TTEMBED_TTCOMPRESSED: TTEMBED_FLAGS = TTEMBED_FLAGS(4u32);
impl ::core::marker::Copy for TTEMBED_FLAGS {}
impl ::core::clone::Clone for TTEMBED_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TTEMBED_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TTEMBED_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TTEMBED_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TTEMBED_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TTEMBED_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TTEMBED_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TTEMBED_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TTEMBED_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for TTEMBED_FLAGS {
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
pub const TTEMBED_SUBSETCANCEL: u32 = 4u32;
pub const TTEMBED_VARIATIONSIMULATED: u32 = 1u32;
pub const TTEMBED_WEBOBJECT: u32 = 128u32;
pub const TTEMBED_XORENCRYPTDATA: u32 = 268435456u32;
pub const TTFCFP_APPLE_PLATFORMID: u32 = 1u32;
pub const TTFCFP_DELTA: u32 = 2u32;
pub const TTFCFP_DONT_CARE: u32 = 65535u32;
pub const TTFCFP_FLAGS_COMPRESS: u32 = 2u32;
pub const TTFCFP_FLAGS_GLYPHLIST: u32 = 8u32;
pub const TTFCFP_FLAGS_SUBSET: u32 = 1u32;
pub const TTFCFP_FLAGS_TTC: u32 = 4u32;
pub const TTFCFP_LANG_KEEP_ALL: u32 = 0u32;
pub const TTFCFP_MS_PLATFORMID: u32 = 3u32;
pub const TTFCFP_SUBSET: u32 = 0u32;
pub const TTFCFP_SUBSET1: u32 = 1u32;
pub const TTFMFP_DELTA: u32 = 2u32;
pub const TTFMFP_SUBSET: u32 = 0u32;
pub const TTFMFP_SUBSET1: u32 = 1u32;
pub struct TTLOADINFO {
    pub usStructSize: u16,
    pub usRefStrSize: u16,
    pub pusRefStr: MutPtr<u16>,
}
impl ::core::marker::Copy for TTLOADINFO {}
impl ::core::clone::Clone for TTLOADINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TTLOADINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TTLOADINFO")
            .field("usStructSize", &self.usStructSize)
            .field("usRefStrSize", &self.usRefStrSize)
            .field("pusRefStr", &self.pusRefStr)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TTLOADINFO {
    fn eq(&self, other: &Self) -> bool {
        self.usStructSize == other.usStructSize
            && self.usRefStrSize == other.usRefStrSize
            && self.pusRefStr == other.pusRefStr
    }
}
impl ::core::cmp::Eq for TTLOADINFO {}
impl FromIntoMemory for TTLOADINFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_usStructSize = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_usRefStrSize = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_pusRefStr = <MutPtr<u16> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            usStructSize: f_usStructSize,
            usRefStrSize: f_usRefStrSize,
            pusRefStr: f_pusRefStr,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.usStructSize, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.usRefStrSize, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.pusRefStr, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TTLOAD_EMBEDDED_FONT_STATUS(pub u32);
pub const TTLOAD_FONT_SUBSETTED: TTLOAD_EMBEDDED_FONT_STATUS = TTLOAD_EMBEDDED_FONT_STATUS(1u32);
pub const TTLOAD_FONT_IN_SYSSTARTUP: TTLOAD_EMBEDDED_FONT_STATUS =
    TTLOAD_EMBEDDED_FONT_STATUS(2u32);
impl ::core::marker::Copy for TTLOAD_EMBEDDED_FONT_STATUS {}
impl ::core::clone::Clone for TTLOAD_EMBEDDED_FONT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TTLOAD_EMBEDDED_FONT_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TTLOAD_EMBEDDED_FONT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TTLOAD_EMBEDDED_FONT_STATUS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for TTLOAD_EMBEDDED_FONT_STATUS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TTLOAD_EMBEDDED_FONT_STATUS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TTLOAD_EMBEDDED_FONT_STATUS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TTLOAD_EMBEDDED_FONT_STATUS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TTLOAD_EMBEDDED_FONT_STATUS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for TTLOAD_EMBEDDED_FONT_STATUS {
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
pub const TTLOAD_EUDC_OVERWRITE: u32 = 2u32;
pub const TTLOAD_EUDC_SET: u32 = 4u32;
pub const TTLOAD_PRIVATE: u32 = 1u32;
pub struct TTPOLYCURVE {
    pub wType: u16,
    pub cpfx: u16,
    pub apfx: [POINTFX; 1],
}
impl ::core::marker::Copy for TTPOLYCURVE {}
impl ::core::clone::Clone for TTPOLYCURVE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TTPOLYCURVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TTPOLYCURVE")
            .field("wType", &self.wType)
            .field("cpfx", &self.cpfx)
            .field("apfx", &self.apfx)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TTPOLYCURVE {
    fn eq(&self, other: &Self) -> bool {
        self.wType == other.wType && self.cpfx == other.cpfx && self.apfx == other.apfx
    }
}
impl ::core::cmp::Eq for TTPOLYCURVE {}
impl FromIntoMemory for TTPOLYCURVE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_wType = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_cpfx = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_apfx = <[POINTFX; 1] as FromIntoMemory>::from_bytes(&from[4..4 + 8]);
        Self {
            wType: f_wType,
            cpfx: f_cpfx,
            apfx: f_apfx,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.wType, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.cpfx, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.apfx, &mut into[4..4 + 8]);
    }
    fn size() -> usize {
        12
    }
}
pub struct TTPOLYGONHEADER {
    pub cb: u32,
    pub dwType: u32,
    pub pfxStart: POINTFX,
}
impl ::core::marker::Copy for TTPOLYGONHEADER {}
impl ::core::clone::Clone for TTPOLYGONHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TTPOLYGONHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TTPOLYGONHEADER")
            .field("cb", &self.cb)
            .field("dwType", &self.dwType)
            .field("pfxStart", &self.pfxStart)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TTPOLYGONHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.dwType == other.dwType && self.pfxStart == other.pfxStart
    }
}
impl ::core::cmp::Eq for TTPOLYGONHEADER {}
impl FromIntoMemory for TTPOLYGONHEADER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_cb = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwType = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_pfxStart = <POINTFX as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        Self {
            cb: f_cb,
            dwType: f_dwType,
            pfxStart: f_pfxStart,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.cb, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwType, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.pfxStart, &mut into[8..8 + 8]);
    }
    fn size() -> usize {
        16
    }
}
pub struct TTVALIDATIONTESTSPARAMS {
    pub ulStructSize: u32,
    pub lTestFromSize: i32,
    pub lTestToSize: i32,
    pub ulCharSet: u32,
    pub usReserved1: u16,
    pub usCharCodeCount: u16,
    pub pusCharCodeSet: MutPtr<u16>,
}
impl ::core::marker::Copy for TTVALIDATIONTESTSPARAMS {}
impl ::core::clone::Clone for TTVALIDATIONTESTSPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TTVALIDATIONTESTSPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TTVALIDATIONTESTSPARAMS")
            .field("ulStructSize", &self.ulStructSize)
            .field("lTestFromSize", &self.lTestFromSize)
            .field("lTestToSize", &self.lTestToSize)
            .field("ulCharSet", &self.ulCharSet)
            .field("usReserved1", &self.usReserved1)
            .field("usCharCodeCount", &self.usCharCodeCount)
            .field("pusCharCodeSet", &self.pusCharCodeSet)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TTVALIDATIONTESTSPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.ulStructSize == other.ulStructSize
            && self.lTestFromSize == other.lTestFromSize
            && self.lTestToSize == other.lTestToSize
            && self.ulCharSet == other.ulCharSet
            && self.usReserved1 == other.usReserved1
            && self.usCharCodeCount == other.usCharCodeCount
            && self.pusCharCodeSet == other.pusCharCodeSet
    }
}
impl ::core::cmp::Eq for TTVALIDATIONTESTSPARAMS {}
impl FromIntoMemory for TTVALIDATIONTESTSPARAMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_ulStructSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lTestFromSize = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lTestToSize = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ulCharSet = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_usReserved1 = <u16 as FromIntoMemory>::from_bytes(&from[16..16 + 2]);
        let f_usCharCodeCount = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        let f_pusCharCodeSet = <MutPtr<u16> as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            ulStructSize: f_ulStructSize,
            lTestFromSize: f_lTestFromSize,
            lTestToSize: f_lTestToSize,
            ulCharSet: f_ulCharSet,
            usReserved1: f_usReserved1,
            usCharCodeCount: f_usCharCodeCount,
            pusCharCodeSet: f_pusCharCodeSet,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.ulStructSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lTestFromSize, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lTestToSize, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ulCharSet, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.usReserved1, &mut into[16..16 + 2]);
        FromIntoMemory::into_bytes(self.usCharCodeCount, &mut into[18..18 + 2]);
        FromIntoMemory::into_bytes(self.pusCharCodeSet, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct TTVALIDATIONTESTSPARAMSEX {
    pub ulStructSize: u32,
    pub lTestFromSize: i32,
    pub lTestToSize: i32,
    pub ulCharSet: u32,
    pub usReserved1: u16,
    pub usCharCodeCount: u16,
    pub pulCharCodeSet: MutPtr<u32>,
}
impl ::core::marker::Copy for TTVALIDATIONTESTSPARAMSEX {}
impl ::core::clone::Clone for TTVALIDATIONTESTSPARAMSEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TTVALIDATIONTESTSPARAMSEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TTVALIDATIONTESTSPARAMSEX")
            .field("ulStructSize", &self.ulStructSize)
            .field("lTestFromSize", &self.lTestFromSize)
            .field("lTestToSize", &self.lTestToSize)
            .field("ulCharSet", &self.ulCharSet)
            .field("usReserved1", &self.usReserved1)
            .field("usCharCodeCount", &self.usCharCodeCount)
            .field("pulCharCodeSet", &self.pulCharCodeSet)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TTVALIDATIONTESTSPARAMSEX {
    fn eq(&self, other: &Self) -> bool {
        self.ulStructSize == other.ulStructSize
            && self.lTestFromSize == other.lTestFromSize
            && self.lTestToSize == other.lTestToSize
            && self.ulCharSet == other.ulCharSet
            && self.usReserved1 == other.usReserved1
            && self.usCharCodeCount == other.usCharCodeCount
            && self.pulCharCodeSet == other.pulCharCodeSet
    }
}
impl ::core::cmp::Eq for TTVALIDATIONTESTSPARAMSEX {}
impl FromIntoMemory for TTVALIDATIONTESTSPARAMSEX {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_ulStructSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lTestFromSize = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lTestToSize = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ulCharSet = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_usReserved1 = <u16 as FromIntoMemory>::from_bytes(&from[16..16 + 2]);
        let f_usCharCodeCount = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        let f_pulCharCodeSet = <MutPtr<u32> as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            ulStructSize: f_ulStructSize,
            lTestFromSize: f_lTestFromSize,
            lTestToSize: f_lTestToSize,
            ulCharSet: f_ulCharSet,
            usReserved1: f_usReserved1,
            usCharCodeCount: f_usCharCodeCount,
            pulCharCodeSet: f_pulCharCodeSet,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.ulStructSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lTestFromSize, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lTestToSize, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ulCharSet, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.usReserved1, &mut into[16..16 + 2]);
        FromIntoMemory::into_bytes(self.usCharCodeCount, &mut into[18..18 + 2]);
        FromIntoMemory::into_bytes(self.pulCharCodeSet, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub const TT_AVAILABLE: u32 = 1u32;
pub const TT_ENABLED: u32 = 2u32;
pub const TT_POLYGON_TYPE: u32 = 24u32;
pub const TT_PRIM_CSPLINE: u32 = 3u32;
pub const TT_PRIM_LINE: u32 = 1u32;
pub const TT_PRIM_QSPLINE: u32 = 2u32;
pub const TURKISH_CHARSET: u32 = 162u32;
pub const VARIABLE_PITCH: u32 = 2u32;
pub const VIETNAMESE_CHARSET: u32 = 163u32;
pub struct WCRANGE {
    pub wcLow: u16,
    pub cGlyphs: u16,
}
impl ::core::marker::Copy for WCRANGE {}
impl ::core::clone::Clone for WCRANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WCRANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCRANGE")
            .field("wcLow", &self.wcLow)
            .field("cGlyphs", &self.cGlyphs)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WCRANGE {
    fn eq(&self, other: &Self) -> bool {
        self.wcLow == other.wcLow && self.cGlyphs == other.cGlyphs
    }
}
impl ::core::cmp::Eq for WCRANGE {}
impl FromIntoMemory for WCRANGE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_wcLow = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_cGlyphs = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        Self {
            wcLow: f_wcLow,
            cGlyphs: f_cGlyphs,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.wcLow, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.cGlyphs, &mut into[2..2 + 2]);
    }
    fn size() -> usize {
        4
    }
}
pub struct WGLSWAP {
    pub hdc: HDC,
    pub uiFlags: u32,
}
impl ::core::marker::Copy for WGLSWAP {}
impl ::core::clone::Clone for WGLSWAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WGLSWAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WGLSWAP")
            .field("hdc", &self.hdc)
            .field("uiFlags", &self.uiFlags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WGLSWAP {
    fn eq(&self, other: &Self) -> bool {
        self.hdc == other.hdc && self.uiFlags == other.uiFlags
    }
}
impl ::core::cmp::Eq for WGLSWAP {}
impl FromIntoMemory for WGLSWAP {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_hdc = <HDC as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_uiFlags = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            hdc: f_hdc,
            uiFlags: f_uiFlags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.hdc, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.uiFlags, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub const WGL_FONT_LINES: u32 = 0u32;
pub const WGL_FONT_POLYGONS: u32 = 1u32;
pub const WGL_SWAPMULTIPLE_MAX: u32 = 16u32;
pub const WGL_SWAP_MAIN_PLANE: u32 = 1u32;
pub const WGL_SWAP_OVERLAY1: u32 = 2u32;
pub const WGL_SWAP_OVERLAY10: u32 = 1024u32;
pub const WGL_SWAP_OVERLAY11: u32 = 2048u32;
pub const WGL_SWAP_OVERLAY12: u32 = 4096u32;
pub const WGL_SWAP_OVERLAY13: u32 = 8192u32;
pub const WGL_SWAP_OVERLAY14: u32 = 16384u32;
pub const WGL_SWAP_OVERLAY15: u32 = 32768u32;
pub const WGL_SWAP_OVERLAY2: u32 = 4u32;
pub const WGL_SWAP_OVERLAY3: u32 = 8u32;
pub const WGL_SWAP_OVERLAY4: u32 = 16u32;
pub const WGL_SWAP_OVERLAY5: u32 = 32u32;
pub const WGL_SWAP_OVERLAY6: u32 = 64u32;
pub const WGL_SWAP_OVERLAY7: u32 = 128u32;
pub const WGL_SWAP_OVERLAY8: u32 = 256u32;
pub const WGL_SWAP_OVERLAY9: u32 = 512u32;
pub const WGL_SWAP_UNDERLAY1: u32 = 65536u32;
pub const WGL_SWAP_UNDERLAY10: u32 = 33554432u32;
pub const WGL_SWAP_UNDERLAY11: u32 = 67108864u32;
pub const WGL_SWAP_UNDERLAY12: u32 = 134217728u32;
pub const WGL_SWAP_UNDERLAY13: u32 = 268435456u32;
pub const WGL_SWAP_UNDERLAY14: u32 = 536870912u32;
pub const WGL_SWAP_UNDERLAY15: u32 = 1073741824u32;
pub const WGL_SWAP_UNDERLAY2: u32 = 131072u32;
pub const WGL_SWAP_UNDERLAY3: u32 = 262144u32;
pub const WGL_SWAP_UNDERLAY4: u32 = 524288u32;
pub const WGL_SWAP_UNDERLAY5: u32 = 1048576u32;
pub const WGL_SWAP_UNDERLAY6: u32 = 2097152u32;
pub const WGL_SWAP_UNDERLAY7: u32 = 4194304u32;
pub const WGL_SWAP_UNDERLAY8: u32 = 8388608u32;
pub const WGL_SWAP_UNDERLAY9: u32 = 16777216u32;
pub type WRITEEMBEDPROC = StdCallFnPtr<
    (
        MutPtr<::core::ffi::c_void>,
        ConstPtr<::core::ffi::c_void>,
        u32,
    ),
    u32,
>;
pub trait Api {
    #[doc = "AbortPath from GDI32"]
    fn AbortPath(&self, hdc: HDC) -> super::super::Foundation::BOOL {
        todo!("AbortPath")
    }
    #[doc = "AddFontMemResourceEx from GDI32"]
    fn AddFontMemResourceEx(
        &self,
        p_file_view: ConstPtr<::core::ffi::c_void>,
        cj_size: u32,
        pv_resrved: MutPtr<::core::ffi::c_void>,
        p_num_fonts: ConstPtr<u32>,
    ) -> super::super::Foundation::HANDLE {
        todo!("AddFontMemResourceEx")
    }
    #[doc = "AddFontResourceA from GDI32"]
    fn AddFontResourceA(&self, param_0: PCSTR) -> i32 {
        todo!("AddFontResourceA")
    }
    #[doc = "AddFontResourceExA from GDI32"]
    fn AddFontResourceExA(
        &self,
        name: PCSTR,
        fl: FONT_RESOURCE_CHARACTERISTICS,
        res: MutPtr<::core::ffi::c_void>,
    ) -> i32 {
        todo!("AddFontResourceExA")
    }
    #[doc = "AddFontResourceExW from GDI32"]
    fn AddFontResourceExW(
        &self,
        name: PCWSTR,
        fl: FONT_RESOURCE_CHARACTERISTICS,
        res: MutPtr<::core::ffi::c_void>,
    ) -> i32 {
        todo!("AddFontResourceExW")
    }
    #[doc = "AddFontResourceW from GDI32"]
    fn AddFontResourceW(&self, param_0: PCWSTR) -> i32 {
        todo!("AddFontResourceW")
    }
    #[doc = "AnimatePalette from GDI32"]
    fn AnimatePalette(
        &self,
        h_pal: HPALETTE,
        i_start_index: u32,
        c_entries: u32,
        ppe: ConstPtr<PALETTEENTRY>,
    ) -> super::super::Foundation::BOOL {
        todo!("AnimatePalette")
    }
    #[doc = "Arc from GDI32"]
    fn Arc(
        &self,
        hdc: HDC,
        x_1: i32,
        y_1: i32,
        x_2: i32,
        y_2: i32,
        x_3: i32,
        y_3: i32,
        x_4: i32,
        y_4: i32,
    ) -> super::super::Foundation::BOOL {
        todo!("Arc")
    }
    #[doc = "ArcTo from GDI32"]
    fn ArcTo(
        &self,
        hdc: HDC,
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
        xr_1: i32,
        yr_1: i32,
        xr_2: i32,
        yr_2: i32,
    ) -> super::super::Foundation::BOOL {
        todo!("ArcTo")
    }
    #[doc = "BeginPaint from USER32"]
    fn BeginPaint(
        &self,
        h_wnd: super::super::Foundation::HWND,
        lp_paint: MutPtr<PAINTSTRUCT>,
    ) -> HDC {
        todo!("BeginPaint")
    }
    #[doc = "BeginPath from GDI32"]
    fn BeginPath(&self, hdc: HDC) -> super::super::Foundation::BOOL {
        todo!("BeginPath")
    }
    #[doc = "BitBlt from GDI32"]
    fn BitBlt(
        &self,
        hdc: HDC,
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        hdc_src: HDC,
        x_1: i32,
        y_1: i32,
        rop: ROP_CODE,
    ) -> super::super::Foundation::BOOL {
        todo!("BitBlt")
    }
    #[doc = "CancelDC from GDI32"]
    fn CancelDC(&self, hdc: HDC) -> super::super::Foundation::BOOL {
        todo!("CancelDC")
    }
    #[doc = "ChangeDisplaySettingsA from USER32"]
    fn ChangeDisplaySettingsA(
        &self,
        lp_dev_mode: ConstPtr<DEVMODEA>,
        dw_flags: CDS_TYPE,
    ) -> DISP_CHANGE {
        todo!("ChangeDisplaySettingsA")
    }
    #[doc = "ChangeDisplaySettingsExA from USER32"]
    fn ChangeDisplaySettingsExA(
        &self,
        lpsz_device_name: PCSTR,
        lp_dev_mode: ConstPtr<DEVMODEA>,
        hwnd: super::super::Foundation::HWND,
        dwflags: CDS_TYPE,
        l_param: ConstPtr<::core::ffi::c_void>,
    ) -> DISP_CHANGE {
        todo!("ChangeDisplaySettingsExA")
    }
    #[doc = "ChangeDisplaySettingsExW from USER32"]
    fn ChangeDisplaySettingsExW(
        &self,
        lpsz_device_name: PCWSTR,
        lp_dev_mode: ConstPtr<DEVMODEW>,
        hwnd: super::super::Foundation::HWND,
        dwflags: CDS_TYPE,
        l_param: ConstPtr<::core::ffi::c_void>,
    ) -> DISP_CHANGE {
        todo!("ChangeDisplaySettingsExW")
    }
    #[doc = "ChangeDisplaySettingsW from USER32"]
    fn ChangeDisplaySettingsW(
        &self,
        lp_dev_mode: ConstPtr<DEVMODEW>,
        dw_flags: CDS_TYPE,
    ) -> DISP_CHANGE {
        todo!("ChangeDisplaySettingsW")
    }
    #[doc = "Chord from GDI32"]
    fn Chord(
        &self,
        hdc: HDC,
        x_1: i32,
        y_1: i32,
        x_2: i32,
        y_2: i32,
        x_3: i32,
        y_3: i32,
        x_4: i32,
        y_4: i32,
    ) -> super::super::Foundation::BOOL {
        todo!("Chord")
    }
    #[doc = "ClientToScreen from USER32"]
    fn ClientToScreen(
        &self,
        h_wnd: super::super::Foundation::HWND,
        lp_point: MutPtr<super::super::Foundation::POINT>,
    ) -> super::super::Foundation::BOOL {
        todo!("ClientToScreen")
    }
    #[doc = "CloseEnhMetaFile from GDI32"]
    fn CloseEnhMetaFile(&self, hdc: HDC) -> HENHMETAFILE {
        todo!("CloseEnhMetaFile")
    }
    #[doc = "CloseFigure from GDI32"]
    fn CloseFigure(&self, hdc: HDC) -> super::super::Foundation::BOOL {
        todo!("CloseFigure")
    }
    #[doc = "CloseMetaFile from GDI32"]
    fn CloseMetaFile(&self, hdc: HDC) -> HMETAFILE {
        todo!("CloseMetaFile")
    }
    #[doc = "CombineRgn from GDI32"]
    fn CombineRgn(
        &self,
        hrgn_dst: HRGN,
        hrgn_src_1: HRGN,
        hrgn_src_2: HRGN,
        i_mode: RGN_COMBINE_MODE,
    ) -> i32 {
        todo!("CombineRgn")
    }
    #[doc = "CopyEnhMetaFileA from GDI32"]
    fn CopyEnhMetaFileA(&self, h_enh: HENHMETAFILE, lp_file_name: PCSTR) -> HENHMETAFILE {
        todo!("CopyEnhMetaFileA")
    }
    #[doc = "CopyEnhMetaFileW from GDI32"]
    fn CopyEnhMetaFileW(&self, h_enh: HENHMETAFILE, lp_file_name: PCWSTR) -> HENHMETAFILE {
        todo!("CopyEnhMetaFileW")
    }
    #[doc = "CopyMetaFileA from GDI32"]
    fn CopyMetaFileA(&self, param_0: HMETAFILE, param_1: PCSTR) -> HMETAFILE {
        todo!("CopyMetaFileA")
    }
    #[doc = "CopyMetaFileW from GDI32"]
    fn CopyMetaFileW(&self, param_0: HMETAFILE, param_1: PCWSTR) -> HMETAFILE {
        todo!("CopyMetaFileW")
    }
    #[doc = "CopyRect from USER32"]
    fn CopyRect(
        &self,
        lprc_dst: MutPtr<super::super::Foundation::RECT>,
        lprc_src: ConstPtr<super::super::Foundation::RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!("CopyRect")
    }
    #[doc = "CreateBitmap from GDI32"]
    fn CreateBitmap(
        &self,
        n_width: i32,
        n_height: i32,
        n_planes: u32,
        n_bit_count: u32,
        lp_bits: ConstPtr<::core::ffi::c_void>,
    ) -> HBITMAP {
        todo!("CreateBitmap")
    }
    #[doc = "CreateBitmapIndirect from GDI32"]
    fn CreateBitmapIndirect(&self, pbm: ConstPtr<BITMAP>) -> HBITMAP {
        todo!("CreateBitmapIndirect")
    }
    #[doc = "CreateBrushIndirect from GDI32"]
    fn CreateBrushIndirect(&self, plbrush: ConstPtr<LOGBRUSH>) -> HBRUSH {
        todo!("CreateBrushIndirect")
    }
    #[doc = "CreateCompatibleBitmap from GDI32"]
    fn CreateCompatibleBitmap(&self, hdc: HDC, cx: i32, cy: i32) -> HBITMAP {
        todo!("CreateCompatibleBitmap")
    }
    #[doc = "CreateCompatibleDC from GDI32"]
    fn CreateCompatibleDC(&self, hdc: HDC) -> CreatedHDC {
        todo!("CreateCompatibleDC")
    }
    #[doc = "CreateDCA from GDI32"]
    fn CreateDCA(
        &self,
        pwsz_driver: PCSTR,
        pwsz_device: PCSTR,
        psz_port: PCSTR,
        pdm: ConstPtr<DEVMODEA>,
    ) -> CreatedHDC {
        todo!("CreateDCA")
    }
    #[doc = "CreateDCW from GDI32"]
    fn CreateDCW(
        &self,
        pwsz_driver: PCWSTR,
        pwsz_device: PCWSTR,
        psz_port: PCWSTR,
        pdm: ConstPtr<DEVMODEW>,
    ) -> CreatedHDC {
        todo!("CreateDCW")
    }
    #[doc = "CreateDIBPatternBrush from GDI32"]
    fn CreateDIBPatternBrush(&self, h: PtrDiffRepr, i_usage: DIB_USAGE) -> HBRUSH {
        todo!("CreateDIBPatternBrush")
    }
    #[doc = "CreateDIBPatternBrushPt from GDI32"]
    fn CreateDIBPatternBrushPt(
        &self,
        lp_packed_dib: ConstPtr<::core::ffi::c_void>,
        i_usage: DIB_USAGE,
    ) -> HBRUSH {
        todo!("CreateDIBPatternBrushPt")
    }
    #[doc = "CreateDIBSection from GDI32"]
    fn CreateDIBSection(
        &self,
        hdc: HDC,
        pbmi: ConstPtr<BITMAPINFO>,
        usage: DIB_USAGE,
        ppv_bits: MutPtr<ConstPtr<::core::ffi::c_void>>,
        h_section: super::super::Foundation::HANDLE,
        offset: u32,
    ) -> HBITMAP {
        todo!("CreateDIBSection")
    }
    #[doc = "CreateDIBitmap from GDI32"]
    fn CreateDIBitmap(
        &self,
        hdc: HDC,
        pbmih: ConstPtr<BITMAPINFOHEADER>,
        fl_init: u32,
        pj_bits: ConstPtr<::core::ffi::c_void>,
        pbmi: ConstPtr<BITMAPINFO>,
        i_usage: DIB_USAGE,
    ) -> HBITMAP {
        todo!("CreateDIBitmap")
    }
    #[doc = "CreateDiscardableBitmap from GDI32"]
    fn CreateDiscardableBitmap(&self, hdc: HDC, cx: i32, cy: i32) -> HBITMAP {
        todo!("CreateDiscardableBitmap")
    }
    #[doc = "CreateEllipticRgn from GDI32"]
    fn CreateEllipticRgn(&self, x_1: i32, y_1: i32, x_2: i32, y_2: i32) -> HRGN {
        todo!("CreateEllipticRgn")
    }
    #[doc = "CreateEllipticRgnIndirect from GDI32"]
    fn CreateEllipticRgnIndirect(&self, lprect: ConstPtr<super::super::Foundation::RECT>) -> HRGN {
        todo!("CreateEllipticRgnIndirect")
    }
    #[doc = "CreateEnhMetaFileA from GDI32"]
    fn CreateEnhMetaFileA(
        &self,
        hdc: HDC,
        lp_filename: PCSTR,
        lprc: ConstPtr<super::super::Foundation::RECT>,
        lp_desc: PCSTR,
    ) -> HdcMetdataEnhFileHandle {
        todo!("CreateEnhMetaFileA")
    }
    #[doc = "CreateEnhMetaFileW from GDI32"]
    fn CreateEnhMetaFileW(
        &self,
        hdc: HDC,
        lp_filename: PCWSTR,
        lprc: ConstPtr<super::super::Foundation::RECT>,
        lp_desc: PCWSTR,
    ) -> HdcMetdataEnhFileHandle {
        todo!("CreateEnhMetaFileW")
    }
    #[doc = "CreateFontA from GDI32"]
    fn CreateFontA(
        &self,
        c_height: i32,
        c_width: i32,
        c_escapement: i32,
        c_orientation: i32,
        c_weight: i32,
        b_italic: u32,
        b_underline: u32,
        b_strike_out: u32,
        i_char_set: u32,
        i_out_precision: FONT_OUTPUT_PRECISION,
        i_clip_precision: FONT_CLIP_PRECISION,
        i_quality: FONT_QUALITY,
        i_pitch_and_family: FONT_PITCH_AND_FAMILY,
        psz_face_name: PCSTR,
    ) -> HFONT {
        todo!("CreateFontA")
    }
    #[doc = "CreateFontIndirectA from GDI32"]
    fn CreateFontIndirectA(&self, lplf: ConstPtr<LOGFONTA>) -> HFONT {
        todo!("CreateFontIndirectA")
    }
    #[doc = "CreateFontIndirectExA from GDI32"]
    fn CreateFontIndirectExA(&self, param_0: ConstPtr<ENUMLOGFONTEXDVA>) -> HFONT {
        todo!("CreateFontIndirectExA")
    }
    #[doc = "CreateFontIndirectExW from GDI32"]
    fn CreateFontIndirectExW(&self, param_0: ConstPtr<ENUMLOGFONTEXDVW>) -> HFONT {
        todo!("CreateFontIndirectExW")
    }
    #[doc = "CreateFontIndirectW from GDI32"]
    fn CreateFontIndirectW(&self, lplf: ConstPtr<LOGFONTW>) -> HFONT {
        todo!("CreateFontIndirectW")
    }
    #[doc = "CreateFontW from GDI32"]
    fn CreateFontW(
        &self,
        c_height: i32,
        c_width: i32,
        c_escapement: i32,
        c_orientation: i32,
        c_weight: i32,
        b_italic: u32,
        b_underline: u32,
        b_strike_out: u32,
        i_char_set: u32,
        i_out_precision: FONT_OUTPUT_PRECISION,
        i_clip_precision: FONT_CLIP_PRECISION,
        i_quality: FONT_QUALITY,
        i_pitch_and_family: FONT_PITCH_AND_FAMILY,
        psz_face_name: PCWSTR,
    ) -> HFONT {
        todo!("CreateFontW")
    }
    #[doc = "CreateHalftonePalette from GDI32"]
    fn CreateHalftonePalette(&self, hdc: HDC) -> HPALETTE {
        todo!("CreateHalftonePalette")
    }
    #[doc = "CreateHatchBrush from GDI32"]
    fn CreateHatchBrush(&self, i_hatch: HATCH_BRUSH_STYLE, color: u32) -> HBRUSH {
        todo!("CreateHatchBrush")
    }
    #[doc = "CreateICA from GDI32"]
    fn CreateICA(
        &self,
        psz_driver: PCSTR,
        psz_device: PCSTR,
        psz_port: PCSTR,
        pdm: ConstPtr<DEVMODEA>,
    ) -> CreatedHDC {
        todo!("CreateICA")
    }
    #[doc = "CreateICW from GDI32"]
    fn CreateICW(
        &self,
        psz_driver: PCWSTR,
        psz_device: PCWSTR,
        psz_port: PCWSTR,
        pdm: ConstPtr<DEVMODEW>,
    ) -> CreatedHDC {
        todo!("CreateICW")
    }
    #[doc = "CreateMetaFileA from GDI32"]
    fn CreateMetaFileA(&self, psz_file: PCSTR) -> HdcMetdataFileHandle {
        todo!("CreateMetaFileA")
    }
    #[doc = "CreateMetaFileW from GDI32"]
    fn CreateMetaFileW(&self, psz_file: PCWSTR) -> HdcMetdataFileHandle {
        todo!("CreateMetaFileW")
    }
    #[doc = "CreatePalette from GDI32"]
    fn CreatePalette(&self, plpal: ConstPtr<LOGPALETTE>) -> HPALETTE {
        todo!("CreatePalette")
    }
    #[doc = "CreatePatternBrush from GDI32"]
    fn CreatePatternBrush(&self, hbm: HBITMAP) -> HBRUSH {
        todo!("CreatePatternBrush")
    }
    #[doc = "CreatePen from GDI32"]
    fn CreatePen(&self, i_style: PEN_STYLE, c_width: i32, color: u32) -> HPEN {
        todo!("CreatePen")
    }
    #[doc = "CreatePenIndirect from GDI32"]
    fn CreatePenIndirect(&self, plpen: ConstPtr<LOGPEN>) -> HPEN {
        todo!("CreatePenIndirect")
    }
    #[doc = "CreatePolyPolygonRgn from GDI32"]
    fn CreatePolyPolygonRgn(
        &self,
        pptl: ConstPtr<super::super::Foundation::POINT>,
        pc: ConstPtr<i32>,
        c_poly: i32,
        i_mode: CREATE_POLYGON_RGN_MODE,
    ) -> HRGN {
        todo!("CreatePolyPolygonRgn")
    }
    #[doc = "CreatePolygonRgn from GDI32"]
    fn CreatePolygonRgn(
        &self,
        pptl: ConstPtr<super::super::Foundation::POINT>,
        c_point: i32,
        i_mode: CREATE_POLYGON_RGN_MODE,
    ) -> HRGN {
        todo!("CreatePolygonRgn")
    }
    #[doc = "CreateRectRgn from GDI32"]
    fn CreateRectRgn(&self, x_1: i32, y_1: i32, x_2: i32, y_2: i32) -> HRGN {
        todo!("CreateRectRgn")
    }
    #[doc = "CreateRectRgnIndirect from GDI32"]
    fn CreateRectRgnIndirect(&self, lprect: ConstPtr<super::super::Foundation::RECT>) -> HRGN {
        todo!("CreateRectRgnIndirect")
    }
    #[doc = "CreateRoundRectRgn from GDI32"]
    fn CreateRoundRectRgn(&self, x_1: i32, y_1: i32, x_2: i32, y_2: i32, w: i32, h: i32) -> HRGN {
        todo!("CreateRoundRectRgn")
    }
    #[doc = "CreateScalableFontResourceA from GDI32"]
    fn CreateScalableFontResourceA(
        &self,
        fdw_hidden: u32,
        lpsz_font: PCSTR,
        lpsz_file: PCSTR,
        lpsz_path: PCSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("CreateScalableFontResourceA")
    }
    #[doc = "CreateScalableFontResourceW from GDI32"]
    fn CreateScalableFontResourceW(
        &self,
        fdw_hidden: u32,
        lpsz_font: PCWSTR,
        lpsz_file: PCWSTR,
        lpsz_path: PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("CreateScalableFontResourceW")
    }
    #[doc = "CreateSolidBrush from GDI32"]
    fn CreateSolidBrush(&self, color: u32) -> HBRUSH {
        todo!("CreateSolidBrush")
    }
    #[doc = "DPtoLP from GDI32"]
    fn DPtoLP(
        &self,
        hdc: HDC,
        lppt: MutPtr<super::super::Foundation::POINT>,
        c: i32,
    ) -> super::super::Foundation::BOOL {
        todo!("DPtoLP")
    }
    #[doc = "DeleteDC from GDI32"]
    fn DeleteDC(&self, hdc: CreatedHDC) -> super::super::Foundation::BOOL {
        todo!("DeleteDC")
    }
    #[doc = "DeleteEnhMetaFile from GDI32"]
    fn DeleteEnhMetaFile(&self, hmf: HENHMETAFILE) -> super::super::Foundation::BOOL {
        todo!("DeleteEnhMetaFile")
    }
    #[doc = "DeleteMetaFile from GDI32"]
    fn DeleteMetaFile(&self, hmf: HMETAFILE) -> super::super::Foundation::BOOL {
        todo!("DeleteMetaFile")
    }
    #[doc = "DeleteObject from GDI32"]
    fn DeleteObject(&self, ho: HGDIOBJ) -> super::super::Foundation::BOOL {
        todo!("DeleteObject")
    }
    #[doc = "DrawAnimatedRects from USER32"]
    fn DrawAnimatedRects(
        &self,
        hwnd: super::super::Foundation::HWND,
        id_ani: i32,
        lprc_from: ConstPtr<super::super::Foundation::RECT>,
        lprc_to: ConstPtr<super::super::Foundation::RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!("DrawAnimatedRects")
    }
    #[doc = "DrawCaption from USER32"]
    fn DrawCaption(
        &self,
        hwnd: super::super::Foundation::HWND,
        hdc: HDC,
        lprect: ConstPtr<super::super::Foundation::RECT>,
        flags: DRAW_CAPTION_FLAGS,
    ) -> super::super::Foundation::BOOL {
        todo!("DrawCaption")
    }
    #[doc = "DrawEdge from USER32"]
    fn DrawEdge(
        &self,
        hdc: HDC,
        qrc: MutPtr<super::super::Foundation::RECT>,
        edge: DRAWEDGE_FLAGS,
        grf_flags: DRAW_EDGE_FLAGS,
    ) -> super::super::Foundation::BOOL {
        todo!("DrawEdge")
    }
    #[doc = "DrawEscape from GDI32"]
    fn DrawEscape(&self, hdc: HDC, i_escape: i32, cj_in: i32, lp_in: PCSTR) -> i32 {
        todo!("DrawEscape")
    }
    #[doc = "DrawFocusRect from USER32"]
    fn DrawFocusRect(
        &self,
        h_dc: HDC,
        lprc: ConstPtr<super::super::Foundation::RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!("DrawFocusRect")
    }
    #[doc = "DrawFrameControl from USER32"]
    fn DrawFrameControl(
        &self,
        param_0: HDC,
        param_1: MutPtr<super::super::Foundation::RECT>,
        param_2: DFC_TYPE,
        param_3: DFCS_STATE,
    ) -> super::super::Foundation::BOOL {
        todo!("DrawFrameControl")
    }
    #[doc = "DrawStateA from USER32"]
    fn DrawStateA(
        &self,
        hdc: HDC,
        hbr_fore: HBRUSH,
        qfn_call_back: DRAWSTATEPROC,
        l_data: super::super::Foundation::LPARAM,
        w_data: super::super::Foundation::WPARAM,
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        u_flags: DRAWSTATE_FLAGS,
    ) -> super::super::Foundation::BOOL {
        todo!("DrawStateA")
    }
    #[doc = "DrawStateW from USER32"]
    fn DrawStateW(
        &self,
        hdc: HDC,
        hbr_fore: HBRUSH,
        qfn_call_back: DRAWSTATEPROC,
        l_data: super::super::Foundation::LPARAM,
        w_data: super::super::Foundation::WPARAM,
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        u_flags: DRAWSTATE_FLAGS,
    ) -> super::super::Foundation::BOOL {
        todo!("DrawStateW")
    }
    #[doc = "DrawTextA from USER32"]
    fn DrawTextA(
        &self,
        hdc: HDC,
        lpch_text: PCSTR,
        cch_text: i32,
        lprc: MutPtr<super::super::Foundation::RECT>,
        format: DRAW_TEXT_FORMAT,
    ) -> i32 {
        todo!("DrawTextA")
    }
    #[doc = "DrawTextExA from USER32"]
    fn DrawTextExA(
        &self,
        hdc: HDC,
        lpch_text: PSTR,
        cch_text: i32,
        lprc: MutPtr<super::super::Foundation::RECT>,
        format: DRAW_TEXT_FORMAT,
        lpdtp: ConstPtr<DRAWTEXTPARAMS>,
    ) -> i32 {
        todo!("DrawTextExA")
    }
    #[doc = "DrawTextExW from USER32"]
    fn DrawTextExW(
        &self,
        hdc: HDC,
        lpch_text: PWSTR,
        cch_text: i32,
        lprc: MutPtr<super::super::Foundation::RECT>,
        format: DRAW_TEXT_FORMAT,
        lpdtp: ConstPtr<DRAWTEXTPARAMS>,
    ) -> i32 {
        todo!("DrawTextExW")
    }
    #[doc = "DrawTextW from USER32"]
    fn DrawTextW(
        &self,
        hdc: HDC,
        lpch_text: PCWSTR,
        cch_text: i32,
        lprc: MutPtr<super::super::Foundation::RECT>,
        format: DRAW_TEXT_FORMAT,
    ) -> i32 {
        todo!("DrawTextW")
    }
    #[doc = "EndPaint from USER32"]
    fn EndPaint(
        &self,
        h_wnd: super::super::Foundation::HWND,
        lp_paint: ConstPtr<PAINTSTRUCT>,
    ) -> super::super::Foundation::BOOL {
        todo!("EndPaint")
    }
    #[doc = "EndPath from GDI32"]
    fn EndPath(&self, hdc: HDC) -> super::super::Foundation::BOOL {
        todo!("EndPath")
    }
    #[doc = "EnumDisplayDevicesA from USER32"]
    fn EnumDisplayDevicesA(
        &self,
        lp_device: PCSTR,
        i_dev_num: u32,
        lp_display_device: MutPtr<DISPLAY_DEVICEA>,
        dw_flags: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("EnumDisplayDevicesA")
    }
    #[doc = "EnumDisplayDevicesW from USER32"]
    fn EnumDisplayDevicesW(
        &self,
        lp_device: PCWSTR,
        i_dev_num: u32,
        lp_display_device: MutPtr<DISPLAY_DEVICEW>,
        dw_flags: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("EnumDisplayDevicesW")
    }
    #[doc = "EnumDisplayMonitors from USER32"]
    fn EnumDisplayMonitors(
        &self,
        hdc: HDC,
        lprc_clip: ConstPtr<super::super::Foundation::RECT>,
        lpfn_enum: MONITORENUMPROC,
        dw_data: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::BOOL {
        todo!("EnumDisplayMonitors")
    }
    #[doc = "EnumDisplaySettingsA from USER32"]
    fn EnumDisplaySettingsA(
        &self,
        lpsz_device_name: PCSTR,
        i_mode_num: ENUM_DISPLAY_SETTINGS_MODE,
        lp_dev_mode: MutPtr<DEVMODEA>,
    ) -> super::super::Foundation::BOOL {
        todo!("EnumDisplaySettingsA")
    }
    #[doc = "EnumDisplaySettingsExA from USER32"]
    fn EnumDisplaySettingsExA(
        &self,
        lpsz_device_name: PCSTR,
        i_mode_num: ENUM_DISPLAY_SETTINGS_MODE,
        lp_dev_mode: MutPtr<DEVMODEA>,
        dw_flags: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("EnumDisplaySettingsExA")
    }
    #[doc = "EnumDisplaySettingsExW from USER32"]
    fn EnumDisplaySettingsExW(
        &self,
        lpsz_device_name: PCWSTR,
        i_mode_num: ENUM_DISPLAY_SETTINGS_MODE,
        lp_dev_mode: MutPtr<DEVMODEW>,
        dw_flags: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("EnumDisplaySettingsExW")
    }
    #[doc = "EnumDisplaySettingsW from USER32"]
    fn EnumDisplaySettingsW(
        &self,
        lpsz_device_name: PCWSTR,
        i_mode_num: ENUM_DISPLAY_SETTINGS_MODE,
        lp_dev_mode: MutPtr<DEVMODEW>,
    ) -> super::super::Foundation::BOOL {
        todo!("EnumDisplaySettingsW")
    }
    #[doc = "EnumEnhMetaFile from GDI32"]
    fn EnumEnhMetaFile(
        &self,
        hdc: HDC,
        hmf: HENHMETAFILE,
        proc: ENHMFENUMPROC,
        param_3: ConstPtr<::core::ffi::c_void>,
        lp_rect: ConstPtr<super::super::Foundation::RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!("EnumEnhMetaFile")
    }
    #[doc = "EnumFontFamiliesA from GDI32"]
    fn EnumFontFamiliesA(
        &self,
        hdc: HDC,
        lp_logfont: PCSTR,
        lp_proc: FONTENUMPROCA,
        l_param: super::super::Foundation::LPARAM,
    ) -> i32 {
        todo!("EnumFontFamiliesA")
    }
    #[doc = "EnumFontFamiliesExA from GDI32"]
    fn EnumFontFamiliesExA(
        &self,
        hdc: HDC,
        lp_logfont: ConstPtr<LOGFONTA>,
        lp_proc: FONTENUMPROCA,
        l_param: super::super::Foundation::LPARAM,
        dw_flags: u32,
    ) -> i32 {
        todo!("EnumFontFamiliesExA")
    }
    #[doc = "EnumFontFamiliesExW from GDI32"]
    fn EnumFontFamiliesExW(
        &self,
        hdc: HDC,
        lp_logfont: ConstPtr<LOGFONTW>,
        lp_proc: FONTENUMPROCW,
        l_param: super::super::Foundation::LPARAM,
        dw_flags: u32,
    ) -> i32 {
        todo!("EnumFontFamiliesExW")
    }
    #[doc = "EnumFontFamiliesW from GDI32"]
    fn EnumFontFamiliesW(
        &self,
        hdc: HDC,
        lp_logfont: PCWSTR,
        lp_proc: FONTENUMPROCW,
        l_param: super::super::Foundation::LPARAM,
    ) -> i32 {
        todo!("EnumFontFamiliesW")
    }
    #[doc = "EnumFontsA from GDI32"]
    fn EnumFontsA(
        &self,
        hdc: HDC,
        lp_logfont: PCSTR,
        lp_proc: FONTENUMPROCA,
        l_param: super::super::Foundation::LPARAM,
    ) -> i32 {
        todo!("EnumFontsA")
    }
    #[doc = "EnumFontsW from GDI32"]
    fn EnumFontsW(
        &self,
        hdc: HDC,
        lp_logfont: PCWSTR,
        lp_proc: FONTENUMPROCW,
        l_param: super::super::Foundation::LPARAM,
    ) -> i32 {
        todo!("EnumFontsW")
    }
    #[doc = "EnumMetaFile from GDI32"]
    fn EnumMetaFile(
        &self,
        hdc: HDC,
        hmf: HMETAFILE,
        proc: MFENUMPROC,
        param_3: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::BOOL {
        todo!("EnumMetaFile")
    }
    #[doc = "EnumObjects from GDI32"]
    fn EnumObjects(
        &self,
        hdc: HDC,
        n_type: OBJ_TYPE,
        lp_func: GOBJENUMPROC,
        l_param: super::super::Foundation::LPARAM,
    ) -> i32 {
        todo!("EnumObjects")
    }
    #[doc = "EqualRect from USER32"]
    fn EqualRect(
        &self,
        lprc_1: ConstPtr<super::super::Foundation::RECT>,
        lprc_2: ConstPtr<super::super::Foundation::RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!("EqualRect")
    }
    #[doc = "EqualRgn from GDI32"]
    fn EqualRgn(&self, hrgn_1: HRGN, hrgn_2: HRGN) -> super::super::Foundation::BOOL {
        todo!("EqualRgn")
    }
    #[doc = "ExcludeClipRect from GDI32"]
    fn ExcludeClipRect(&self, hdc: HDC, left: i32, top: i32, right: i32, bottom: i32) -> i32 {
        todo!("ExcludeClipRect")
    }
    #[doc = "ExcludeUpdateRgn from USER32"]
    fn ExcludeUpdateRgn(&self, h_dc: HDC, h_wnd: super::super::Foundation::HWND) -> i32 {
        todo!("ExcludeUpdateRgn")
    }
    #[doc = "ExtCreatePen from GDI32"]
    fn ExtCreatePen(
        &self,
        i_pen_style: PEN_STYLE,
        c_width: u32,
        plbrush: ConstPtr<LOGBRUSH>,
        c_style: u32,
        pstyle: ConstPtr<u32>,
    ) -> HPEN {
        todo!("ExtCreatePen")
    }
    #[doc = "ExtFloodFill from GDI32"]
    fn ExtFloodFill(
        &self,
        hdc: HDC,
        x: i32,
        y: i32,
        color: u32,
        r#type: EXT_FLOOD_FILL_TYPE,
    ) -> super::super::Foundation::BOOL {
        todo!("ExtFloodFill")
    }
    #[doc = "ExtSelectClipRgn from GDI32"]
    fn ExtSelectClipRgn(&self, hdc: HDC, hrgn: HRGN, mode: RGN_COMBINE_MODE) -> i32 {
        todo!("ExtSelectClipRgn")
    }
    #[doc = "ExtTextOutA from GDI32"]
    fn ExtTextOutA(
        &self,
        hdc: HDC,
        x: i32,
        y: i32,
        options: ETO_OPTIONS,
        lprect: ConstPtr<super::super::Foundation::RECT>,
        lp_string: PCSTR,
        c: u32,
        lp_dx: ConstPtr<i32>,
    ) -> super::super::Foundation::BOOL {
        todo!("ExtTextOutA")
    }
    #[doc = "ExtTextOutW from GDI32"]
    fn ExtTextOutW(
        &self,
        hdc: HDC,
        x: i32,
        y: i32,
        options: ETO_OPTIONS,
        lprect: ConstPtr<super::super::Foundation::RECT>,
        lp_string: PCWSTR,
        c: u32,
        lp_dx: ConstPtr<i32>,
    ) -> super::super::Foundation::BOOL {
        todo!("ExtTextOutW")
    }
    #[doc = "FillPath from GDI32"]
    fn FillPath(&self, hdc: HDC) -> super::super::Foundation::BOOL {
        todo!("FillPath")
    }
    #[doc = "FillRect from USER32"]
    fn FillRect(
        &self,
        h_dc: HDC,
        lprc: ConstPtr<super::super::Foundation::RECT>,
        hbr: HBRUSH,
    ) -> i32 {
        todo!("FillRect")
    }
    #[doc = "FillRgn from GDI32"]
    fn FillRgn(&self, hdc: HDC, hrgn: HRGN, hbr: HBRUSH) -> super::super::Foundation::BOOL {
        todo!("FillRgn")
    }
    #[doc = "FixBrushOrgEx from GDI32"]
    fn FixBrushOrgEx(
        &self,
        hdc: HDC,
        x: i32,
        y: i32,
        ptl: ConstPtr<super::super::Foundation::POINT>,
    ) -> super::super::Foundation::BOOL {
        todo!("FixBrushOrgEx")
    }
    #[doc = "FlattenPath from GDI32"]
    fn FlattenPath(&self, hdc: HDC) -> super::super::Foundation::BOOL {
        todo!("FlattenPath")
    }
    #[doc = "FloodFill from GDI32"]
    fn FloodFill(&self, hdc: HDC, x: i32, y: i32, color: u32) -> super::super::Foundation::BOOL {
        todo!("FloodFill")
    }
    #[doc = "FrameRect from USER32"]
    fn FrameRect(
        &self,
        h_dc: HDC,
        lprc: ConstPtr<super::super::Foundation::RECT>,
        hbr: HBRUSH,
    ) -> i32 {
        todo!("FrameRect")
    }
    #[doc = "FrameRgn from GDI32"]
    fn FrameRgn(
        &self,
        hdc: HDC,
        hrgn: HRGN,
        hbr: HBRUSH,
        w: i32,
        h: i32,
    ) -> super::super::Foundation::BOOL {
        todo!("FrameRgn")
    }
    #[doc = "GdiAlphaBlend from GDI32"]
    fn GdiAlphaBlend(
        &self,
        hdc_dest: HDC,
        xorigin_dest: i32,
        yorigin_dest: i32,
        w_dest: i32,
        h_dest: i32,
        hdc_src: HDC,
        xorigin_src: i32,
        yorigin_src: i32,
        w_src: i32,
        h_src: i32,
        ftn: BLENDFUNCTION,
    ) -> super::super::Foundation::BOOL {
        todo!("GdiAlphaBlend")
    }
    #[doc = "GdiComment from GDI32"]
    fn GdiComment(
        &self,
        hdc: HDC,
        n_size: u32,
        lp_data: ConstPtr<u8>,
    ) -> super::super::Foundation::BOOL {
        todo!("GdiComment")
    }
    #[doc = "GdiFlush from GDI32"]
    fn GdiFlush(&self) -> super::super::Foundation::BOOL {
        todo!("GdiFlush")
    }
    #[doc = "GdiGetBatchLimit from GDI32"]
    fn GdiGetBatchLimit(&self) -> u32 {
        todo!("GdiGetBatchLimit")
    }
    #[doc = "GdiGradientFill from GDI32"]
    fn GdiGradientFill(
        &self,
        hdc: HDC,
        p_vertex: ConstPtr<TRIVERTEX>,
        n_vertex: u32,
        p_mesh: ConstPtr<::core::ffi::c_void>,
        n_count: u32,
        ul_mode: GRADIENT_FILL,
    ) -> super::super::Foundation::BOOL {
        todo!("GdiGradientFill")
    }
    #[doc = "GdiSetBatchLimit from GDI32"]
    fn GdiSetBatchLimit(&self, dw: u32) -> u32 {
        todo!("GdiSetBatchLimit")
    }
    #[doc = "GdiTransparentBlt from GDI32"]
    fn GdiTransparentBlt(
        &self,
        hdc_dest: HDC,
        xorigin_dest: i32,
        yorigin_dest: i32,
        w_dest: i32,
        h_dest: i32,
        hdc_src: HDC,
        xorigin_src: i32,
        yorigin_src: i32,
        w_src: i32,
        h_src: i32,
        cr_transparent: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("GdiTransparentBlt")
    }
    #[doc = "GetArcDirection from GDI32"]
    fn GetArcDirection(&self, hdc: HDC) -> i32 {
        todo!("GetArcDirection")
    }
    #[doc = "GetAspectRatioFilterEx from GDI32"]
    fn GetAspectRatioFilterEx(
        &self,
        hdc: HDC,
        lpsize: MutPtr<super::super::Foundation::SIZE>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetAspectRatioFilterEx")
    }
    #[doc = "GetBitmapBits from GDI32"]
    fn GetBitmapBits(&self, hbit: HBITMAP, cb: i32, lpv_bits: MutPtr<::core::ffi::c_void>) -> i32 {
        todo!("GetBitmapBits")
    }
    #[doc = "GetBitmapDimensionEx from GDI32"]
    fn GetBitmapDimensionEx(
        &self,
        hbit: HBITMAP,
        lpsize: MutPtr<super::super::Foundation::SIZE>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetBitmapDimensionEx")
    }
    #[doc = "GetBkColor from GDI32"]
    fn GetBkColor(&self, hdc: HDC) -> u32 {
        todo!("GetBkColor")
    }
    #[doc = "GetBkMode from GDI32"]
    fn GetBkMode(&self, hdc: HDC) -> i32 {
        todo!("GetBkMode")
    }
    #[doc = "GetBoundsRect from GDI32"]
    fn GetBoundsRect(
        &self,
        hdc: HDC,
        lprect: MutPtr<super::super::Foundation::RECT>,
        flags: u32,
    ) -> u32 {
        todo!("GetBoundsRect")
    }
    #[doc = "GetBrushOrgEx from GDI32"]
    fn GetBrushOrgEx(
        &self,
        hdc: HDC,
        lppt: MutPtr<super::super::Foundation::POINT>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetBrushOrgEx")
    }
    #[doc = "GetCharABCWidthsA from GDI32"]
    fn GetCharABCWidthsA(
        &self,
        hdc: HDC,
        w_first: u32,
        w_last: u32,
        lp_abc: MutPtr<ABC>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetCharABCWidthsA")
    }
    #[doc = "GetCharABCWidthsI from GDI32"]
    fn GetCharABCWidthsI(
        &self,
        hdc: HDC,
        gi_first: u32,
        cgi: u32,
        pgi: ConstPtr<u16>,
        pabc: MutPtr<ABC>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetCharABCWidthsI")
    }
    #[doc = "GetCharABCWidthsW from GDI32"]
    fn GetCharABCWidthsW(
        &self,
        hdc: HDC,
        w_first: u32,
        w_last: u32,
        lp_abc: MutPtr<ABC>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetCharABCWidthsW")
    }
    #[doc = "GetCharWidth32A from GDI32"]
    fn GetCharWidth32A(
        &self,
        hdc: HDC,
        i_first: u32,
        i_last: u32,
        lp_buffer: MutPtr<i32>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetCharWidth32A")
    }
    #[doc = "GetCharWidth32W from GDI32"]
    fn GetCharWidth32W(
        &self,
        hdc: HDC,
        i_first: u32,
        i_last: u32,
        lp_buffer: MutPtr<i32>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetCharWidth32W")
    }
    #[doc = "GetCharWidthA from GDI32"]
    fn GetCharWidthA(
        &self,
        hdc: HDC,
        i_first: u32,
        i_last: u32,
        lp_buffer: MutPtr<i32>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetCharWidthA")
    }
    #[doc = "GetCharWidthFloatA from GDI32"]
    fn GetCharWidthFloatA(
        &self,
        hdc: HDC,
        i_first: u32,
        i_last: u32,
        lp_buffer: MutPtr<f32>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetCharWidthFloatA")
    }
    #[doc = "GetCharWidthFloatW from GDI32"]
    fn GetCharWidthFloatW(
        &self,
        hdc: HDC,
        i_first: u32,
        i_last: u32,
        lp_buffer: MutPtr<f32>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetCharWidthFloatW")
    }
    #[doc = "GetCharWidthI from GDI32"]
    fn GetCharWidthI(
        &self,
        hdc: HDC,
        gi_first: u32,
        cgi: u32,
        pgi: ConstPtr<u16>,
        pi_widths: MutPtr<i32>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetCharWidthI")
    }
    #[doc = "GetCharWidthW from GDI32"]
    fn GetCharWidthW(
        &self,
        hdc: HDC,
        i_first: u32,
        i_last: u32,
        lp_buffer: MutPtr<i32>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetCharWidthW")
    }
    #[doc = "GetCharacterPlacementA from GDI32"]
    fn GetCharacterPlacementA(
        &self,
        hdc: HDC,
        lp_string: PCSTR,
        n_count: i32,
        n_mex_extent: i32,
        lp_results: MutPtr<GCP_RESULTSA>,
        dw_flags: GET_CHARACTER_PLACEMENT_FLAGS,
    ) -> u32 {
        todo!("GetCharacterPlacementA")
    }
    #[doc = "GetCharacterPlacementW from GDI32"]
    fn GetCharacterPlacementW(
        &self,
        hdc: HDC,
        lp_string: PCWSTR,
        n_count: i32,
        n_mex_extent: i32,
        lp_results: MutPtr<GCP_RESULTSW>,
        dw_flags: GET_CHARACTER_PLACEMENT_FLAGS,
    ) -> u32 {
        todo!("GetCharacterPlacementW")
    }
    #[doc = "GetClipBox from GDI32"]
    fn GetClipBox(&self, hdc: HDC, lprect: MutPtr<super::super::Foundation::RECT>) -> i32 {
        todo!("GetClipBox")
    }
    #[doc = "GetClipRgn from GDI32"]
    fn GetClipRgn(&self, hdc: HDC, hrgn: HRGN) -> i32 {
        todo!("GetClipRgn")
    }
    #[doc = "GetColorAdjustment from GDI32"]
    fn GetColorAdjustment(
        &self,
        hdc: HDC,
        lpca: MutPtr<COLORADJUSTMENT>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetColorAdjustment")
    }
    #[doc = "GetCurrentObject from GDI32"]
    fn GetCurrentObject(&self, hdc: HDC, r#type: OBJ_TYPE) -> HGDIOBJ {
        todo!("GetCurrentObject")
    }
    #[doc = "GetCurrentPositionEx from GDI32"]
    fn GetCurrentPositionEx(
        &self,
        hdc: HDC,
        lppt: MutPtr<super::super::Foundation::POINT>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetCurrentPositionEx")
    }
    #[doc = "GetDC from USER32"]
    fn GetDC(&self, h_wnd: super::super::Foundation::HWND) -> HDC {
        todo!("GetDC")
    }
    #[doc = "GetDCBrushColor from GDI32"]
    fn GetDCBrushColor(&self, hdc: HDC) -> u32 {
        todo!("GetDCBrushColor")
    }
    #[doc = "GetDCEx from USER32"]
    fn GetDCEx(
        &self,
        h_wnd: super::super::Foundation::HWND,
        hrgn_clip: HRGN,
        flags: GET_DCX_FLAGS,
    ) -> HDC {
        todo!("GetDCEx")
    }
    #[doc = "GetDCOrgEx from GDI32"]
    fn GetDCOrgEx(
        &self,
        hdc: HDC,
        lppt: MutPtr<super::super::Foundation::POINT>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetDCOrgEx")
    }
    #[doc = "GetDCPenColor from GDI32"]
    fn GetDCPenColor(&self, hdc: HDC) -> u32 {
        todo!("GetDCPenColor")
    }
    #[doc = "GetDIBColorTable from GDI32"]
    fn GetDIBColorTable(
        &self,
        hdc: HDC,
        i_start: u32,
        c_entries: u32,
        prgbq: MutPtr<RGBQUAD>,
    ) -> u32 {
        todo!("GetDIBColorTable")
    }
    #[doc = "GetDIBits from GDI32"]
    fn GetDIBits(
        &self,
        hdc: HDC,
        hbm: HBITMAP,
        start: u32,
        c_lines: u32,
        lpv_bits: MutPtr<::core::ffi::c_void>,
        lpbmi: MutPtr<BITMAPINFO>,
        usage: DIB_USAGE,
    ) -> i32 {
        todo!("GetDIBits")
    }
    #[doc = "GetDeviceCaps from GDI32"]
    fn GetDeviceCaps(&self, hdc: HDC, index: GET_DEVICE_CAPS_INDEX) -> i32 {
        todo!("GetDeviceCaps")
    }
    #[doc = "GetEnhMetaFileA from GDI32"]
    fn GetEnhMetaFileA(&self, lp_name: PCSTR) -> HENHMETAFILE {
        todo!("GetEnhMetaFileA")
    }
    #[doc = "GetEnhMetaFileBits from GDI32"]
    fn GetEnhMetaFileBits(&self, h_emf: HENHMETAFILE, n_size: u32, lp_data: MutPtr<u8>) -> u32 {
        todo!("GetEnhMetaFileBits")
    }
    #[doc = "GetEnhMetaFileDescriptionA from GDI32"]
    fn GetEnhMetaFileDescriptionA(
        &self,
        hemf: HENHMETAFILE,
        cch_buffer: u32,
        lp_description: PSTR,
    ) -> u32 {
        todo!("GetEnhMetaFileDescriptionA")
    }
    #[doc = "GetEnhMetaFileDescriptionW from GDI32"]
    fn GetEnhMetaFileDescriptionW(
        &self,
        hemf: HENHMETAFILE,
        cch_buffer: u32,
        lp_description: PWSTR,
    ) -> u32 {
        todo!("GetEnhMetaFileDescriptionW")
    }
    #[doc = "GetEnhMetaFileHeader from GDI32"]
    fn GetEnhMetaFileHeader(
        &self,
        hemf: HENHMETAFILE,
        n_size: u32,
        lp_enh_meta_header: MutPtr<ENHMETAHEADER>,
    ) -> u32 {
        todo!("GetEnhMetaFileHeader")
    }
    #[doc = "GetEnhMetaFilePaletteEntries from GDI32"]
    fn GetEnhMetaFilePaletteEntries(
        &self,
        hemf: HENHMETAFILE,
        n_num_entries: u32,
        lp_palette_entries: MutPtr<PALETTEENTRY>,
    ) -> u32 {
        todo!("GetEnhMetaFilePaletteEntries")
    }
    #[doc = "GetEnhMetaFileW from GDI32"]
    fn GetEnhMetaFileW(&self, lp_name: PCWSTR) -> HENHMETAFILE {
        todo!("GetEnhMetaFileW")
    }
    #[doc = "GetFontData from GDI32"]
    fn GetFontData(
        &self,
        hdc: HDC,
        dw_table: u32,
        dw_offset: u32,
        pv_buffer: MutPtr<::core::ffi::c_void>,
        cj_buffer: u32,
    ) -> u32 {
        todo!("GetFontData")
    }
    #[doc = "GetFontLanguageInfo from GDI32"]
    fn GetFontLanguageInfo(&self, hdc: HDC) -> u32 {
        todo!("GetFontLanguageInfo")
    }
    #[doc = "GetFontUnicodeRanges from GDI32"]
    fn GetFontUnicodeRanges(&self, hdc: HDC, lpgs: MutPtr<GLYPHSET>) -> u32 {
        todo!("GetFontUnicodeRanges")
    }
    #[doc = "GetGlyphIndicesA from GDI32"]
    fn GetGlyphIndicesA(&self, hdc: HDC, lpstr: PCSTR, c: i32, pgi: MutPtr<u16>, fl: u32) -> u32 {
        todo!("GetGlyphIndicesA")
    }
    #[doc = "GetGlyphIndicesW from GDI32"]
    fn GetGlyphIndicesW(&self, hdc: HDC, lpstr: PCWSTR, c: i32, pgi: MutPtr<u16>, fl: u32) -> u32 {
        todo!("GetGlyphIndicesW")
    }
    #[doc = "GetGlyphOutlineA from GDI32"]
    fn GetGlyphOutlineA(
        &self,
        hdc: HDC,
        u_char: u32,
        fu_format: GET_GLYPH_OUTLINE_FORMAT,
        lpgm: MutPtr<GLYPHMETRICS>,
        cj_buffer: u32,
        pv_buffer: MutPtr<::core::ffi::c_void>,
        lpmat_2: ConstPtr<MAT2>,
    ) -> u32 {
        todo!("GetGlyphOutlineA")
    }
    #[doc = "GetGlyphOutlineW from GDI32"]
    fn GetGlyphOutlineW(
        &self,
        hdc: HDC,
        u_char: u32,
        fu_format: GET_GLYPH_OUTLINE_FORMAT,
        lpgm: MutPtr<GLYPHMETRICS>,
        cj_buffer: u32,
        pv_buffer: MutPtr<::core::ffi::c_void>,
        lpmat_2: ConstPtr<MAT2>,
    ) -> u32 {
        todo!("GetGlyphOutlineW")
    }
    #[doc = "GetGraphicsMode from GDI32"]
    fn GetGraphicsMode(&self, hdc: HDC) -> i32 {
        todo!("GetGraphicsMode")
    }
    #[doc = "GetKerningPairsA from GDI32"]
    fn GetKerningPairsA(&self, hdc: HDC, n_pairs: u32, lp_kern_pair: MutPtr<KERNINGPAIR>) -> u32 {
        todo!("GetKerningPairsA")
    }
    #[doc = "GetKerningPairsW from GDI32"]
    fn GetKerningPairsW(&self, hdc: HDC, n_pairs: u32, lp_kern_pair: MutPtr<KERNINGPAIR>) -> u32 {
        todo!("GetKerningPairsW")
    }
    #[doc = "GetLayout from GDI32"]
    fn GetLayout(&self, hdc: HDC) -> u32 {
        todo!("GetLayout")
    }
    #[doc = "GetMapMode from GDI32"]
    fn GetMapMode(&self, hdc: HDC) -> i32 {
        todo!("GetMapMode")
    }
    #[doc = "GetMetaFileA from GDI32"]
    fn GetMetaFileA(&self, lp_name: PCSTR) -> HMETAFILE {
        todo!("GetMetaFileA")
    }
    #[doc = "GetMetaFileBitsEx from GDI32"]
    fn GetMetaFileBitsEx(
        &self,
        h_mf: HMETAFILE,
        cb_buffer: u32,
        lp_data: MutPtr<::core::ffi::c_void>,
    ) -> u32 {
        todo!("GetMetaFileBitsEx")
    }
    #[doc = "GetMetaFileW from GDI32"]
    fn GetMetaFileW(&self, lp_name: PCWSTR) -> HMETAFILE {
        todo!("GetMetaFileW")
    }
    #[doc = "GetMetaRgn from GDI32"]
    fn GetMetaRgn(&self, hdc: HDC, hrgn: HRGN) -> i32 {
        todo!("GetMetaRgn")
    }
    #[doc = "GetMiterLimit from GDI32"]
    fn GetMiterLimit(&self, hdc: HDC, plimit: MutPtr<f32>) -> super::super::Foundation::BOOL {
        todo!("GetMiterLimit")
    }
    #[doc = "GetMonitorInfoA from USER32"]
    fn GetMonitorInfoA(
        &self,
        h_monitor: HMONITOR,
        lpmi: MutPtr<MONITORINFO>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetMonitorInfoA")
    }
    #[doc = "GetMonitorInfoW from USER32"]
    fn GetMonitorInfoW(
        &self,
        h_monitor: HMONITOR,
        lpmi: MutPtr<MONITORINFO>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetMonitorInfoW")
    }
    #[doc = "GetNearestColor from GDI32"]
    fn GetNearestColor(&self, hdc: HDC, color: u32) -> u32 {
        todo!("GetNearestColor")
    }
    #[doc = "GetNearestPaletteIndex from GDI32"]
    fn GetNearestPaletteIndex(&self, h: HPALETTE, color: u32) -> u32 {
        todo!("GetNearestPaletteIndex")
    }
    #[doc = "GetObjectA from GDI32"]
    fn GetObjectA(&self, h: HGDIOBJ, c: i32, pv: MutPtr<::core::ffi::c_void>) -> i32 {
        todo!("GetObjectA")
    }
    #[doc = "GetObjectType from GDI32"]
    fn GetObjectType(&self, h: HGDIOBJ) -> u32 {
        todo!("GetObjectType")
    }
    #[doc = "GetObjectW from GDI32"]
    fn GetObjectW(&self, h: HGDIOBJ, c: i32, pv: MutPtr<::core::ffi::c_void>) -> i32 {
        todo!("GetObjectW")
    }
    #[doc = "GetOutlineTextMetricsA from GDI32"]
    fn GetOutlineTextMetricsA(
        &self,
        hdc: HDC,
        cj_copy: u32,
        potm: MutPtr<OUTLINETEXTMETRICA>,
    ) -> u32 {
        todo!("GetOutlineTextMetricsA")
    }
    #[doc = "GetOutlineTextMetricsW from GDI32"]
    fn GetOutlineTextMetricsW(
        &self,
        hdc: HDC,
        cj_copy: u32,
        potm: MutPtr<OUTLINETEXTMETRICW>,
    ) -> u32 {
        todo!("GetOutlineTextMetricsW")
    }
    #[doc = "GetPaletteEntries from GDI32"]
    fn GetPaletteEntries(
        &self,
        hpal: HPALETTE,
        i_start: u32,
        c_entries: u32,
        p_pal_entries: MutPtr<PALETTEENTRY>,
    ) -> u32 {
        todo!("GetPaletteEntries")
    }
    #[doc = "GetPath from GDI32"]
    fn GetPath(
        &self,
        hdc: HDC,
        apt: MutPtr<super::super::Foundation::POINT>,
        aj: MutPtr<u8>,
        cpt: i32,
    ) -> i32 {
        todo!("GetPath")
    }
    #[doc = "GetPixel from GDI32"]
    fn GetPixel(&self, hdc: HDC, x: i32, y: i32) -> u32 {
        todo!("GetPixel")
    }
    #[doc = "GetPolyFillMode from GDI32"]
    fn GetPolyFillMode(&self, hdc: HDC) -> i32 {
        todo!("GetPolyFillMode")
    }
    #[doc = "GetROP2 from GDI32"]
    fn GetROP2(&self, hdc: HDC) -> i32 {
        todo!("GetROP2")
    }
    #[doc = "GetRandomRgn from GDI32"]
    fn GetRandomRgn(&self, hdc: HDC, hrgn: HRGN, i: i32) -> i32 {
        todo!("GetRandomRgn")
    }
    #[doc = "GetRasterizerCaps from GDI32"]
    fn GetRasterizerCaps(
        &self,
        lpraststat: MutPtr<RASTERIZER_STATUS>,
        cj_bytes: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("GetRasterizerCaps")
    }
    #[doc = "GetRegionData from GDI32"]
    fn GetRegionData(&self, hrgn: HRGN, n_count: u32, lp_rgn_data: MutPtr<RGNDATA>) -> u32 {
        todo!("GetRegionData")
    }
    #[doc = "GetRgnBox from GDI32"]
    fn GetRgnBox(&self, hrgn: HRGN, lprc: MutPtr<super::super::Foundation::RECT>) -> i32 {
        todo!("GetRgnBox")
    }
    #[doc = "GetStockObject from GDI32"]
    fn GetStockObject(&self, i: GET_STOCK_OBJECT_FLAGS) -> HGDIOBJ {
        todo!("GetStockObject")
    }
    #[doc = "GetStretchBltMode from GDI32"]
    fn GetStretchBltMode(&self, hdc: HDC) -> i32 {
        todo!("GetStretchBltMode")
    }
    #[doc = "GetSysColorBrush from USER32"]
    fn GetSysColorBrush(&self, n_index: i32) -> HBRUSH {
        todo!("GetSysColorBrush")
    }
    #[doc = "GetSystemPaletteEntries from GDI32"]
    fn GetSystemPaletteEntries(
        &self,
        hdc: HDC,
        i_start: u32,
        c_entries: u32,
        p_pal_entries: MutPtr<PALETTEENTRY>,
    ) -> u32 {
        todo!("GetSystemPaletteEntries")
    }
    #[doc = "GetSystemPaletteUse from GDI32"]
    fn GetSystemPaletteUse(&self, hdc: HDC) -> u32 {
        todo!("GetSystemPaletteUse")
    }
    #[doc = "GetTabbedTextExtentA from USER32"]
    fn GetTabbedTextExtentA(
        &self,
        hdc: HDC,
        lp_string: PCSTR,
        ch_count: i32,
        n_tab_positions: i32,
        lpn_tab_stop_positions: ConstPtr<i32>,
    ) -> u32 {
        todo!("GetTabbedTextExtentA")
    }
    #[doc = "GetTabbedTextExtentW from USER32"]
    fn GetTabbedTextExtentW(
        &self,
        hdc: HDC,
        lp_string: PCWSTR,
        ch_count: i32,
        n_tab_positions: i32,
        lpn_tab_stop_positions: ConstPtr<i32>,
    ) -> u32 {
        todo!("GetTabbedTextExtentW")
    }
    #[doc = "GetTextAlign from GDI32"]
    fn GetTextAlign(&self, hdc: HDC) -> u32 {
        todo!("GetTextAlign")
    }
    #[doc = "GetTextCharacterExtra from GDI32"]
    fn GetTextCharacterExtra(&self, hdc: HDC) -> i32 {
        todo!("GetTextCharacterExtra")
    }
    #[doc = "GetTextColor from GDI32"]
    fn GetTextColor(&self, hdc: HDC) -> u32 {
        todo!("GetTextColor")
    }
    #[doc = "GetTextExtentExPointA from GDI32"]
    fn GetTextExtentExPointA(
        &self,
        hdc: HDC,
        lpsz_string: PCSTR,
        cch_string: i32,
        n_max_extent: i32,
        lpn_fit: MutPtr<i32>,
        lpn_dx: MutPtr<i32>,
        lp_size: MutPtr<super::super::Foundation::SIZE>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetTextExtentExPointA")
    }
    #[doc = "GetTextExtentExPointI from GDI32"]
    fn GetTextExtentExPointI(
        &self,
        hdc: HDC,
        lpwsz_string: ConstPtr<u16>,
        cwch_string: i32,
        n_max_extent: i32,
        lpn_fit: MutPtr<i32>,
        lpn_dx: MutPtr<i32>,
        lp_size: MutPtr<super::super::Foundation::SIZE>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetTextExtentExPointI")
    }
    #[doc = "GetTextExtentExPointW from GDI32"]
    fn GetTextExtentExPointW(
        &self,
        hdc: HDC,
        lpsz_string: PCWSTR,
        cch_string: i32,
        n_max_extent: i32,
        lpn_fit: MutPtr<i32>,
        lpn_dx: MutPtr<i32>,
        lp_size: MutPtr<super::super::Foundation::SIZE>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetTextExtentExPointW")
    }
    #[doc = "GetTextExtentPoint32A from GDI32"]
    fn GetTextExtentPoint32A(
        &self,
        hdc: HDC,
        lp_string: PCSTR,
        c: i32,
        psizl: MutPtr<super::super::Foundation::SIZE>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetTextExtentPoint32A")
    }
    #[doc = "GetTextExtentPoint32W from GDI32"]
    fn GetTextExtentPoint32W(
        &self,
        hdc: HDC,
        lp_string: PCWSTR,
        c: i32,
        psizl: MutPtr<super::super::Foundation::SIZE>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetTextExtentPoint32W")
    }
    #[doc = "GetTextExtentPointA from GDI32"]
    fn GetTextExtentPointA(
        &self,
        hdc: HDC,
        lp_string: PCSTR,
        c: i32,
        lpsz: MutPtr<super::super::Foundation::SIZE>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetTextExtentPointA")
    }
    #[doc = "GetTextExtentPointI from GDI32"]
    fn GetTextExtentPointI(
        &self,
        hdc: HDC,
        pgi_in: ConstPtr<u16>,
        cgi: i32,
        psize: MutPtr<super::super::Foundation::SIZE>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetTextExtentPointI")
    }
    #[doc = "GetTextExtentPointW from GDI32"]
    fn GetTextExtentPointW(
        &self,
        hdc: HDC,
        lp_string: PCWSTR,
        c: i32,
        lpsz: MutPtr<super::super::Foundation::SIZE>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetTextExtentPointW")
    }
    #[doc = "GetTextFaceA from GDI32"]
    fn GetTextFaceA(&self, hdc: HDC, c: i32, lp_name: PSTR) -> i32 {
        todo!("GetTextFaceA")
    }
    #[doc = "GetTextFaceW from GDI32"]
    fn GetTextFaceW(&self, hdc: HDC, c: i32, lp_name: PWSTR) -> i32 {
        todo!("GetTextFaceW")
    }
    #[doc = "GetTextMetricsA from GDI32"]
    fn GetTextMetricsA(
        &self,
        hdc: HDC,
        lptm: MutPtr<TEXTMETRICA>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetTextMetricsA")
    }
    #[doc = "GetTextMetricsW from GDI32"]
    fn GetTextMetricsW(
        &self,
        hdc: HDC,
        lptm: MutPtr<TEXTMETRICW>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetTextMetricsW")
    }
    #[doc = "GetUpdateRect from USER32"]
    fn GetUpdateRect(
        &self,
        h_wnd: super::super::Foundation::HWND,
        lp_rect: MutPtr<super::super::Foundation::RECT>,
        b_erase: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!("GetUpdateRect")
    }
    #[doc = "GetUpdateRgn from USER32"]
    fn GetUpdateRgn(
        &self,
        h_wnd: super::super::Foundation::HWND,
        h_rgn: HRGN,
        b_erase: super::super::Foundation::BOOL,
    ) -> i32 {
        todo!("GetUpdateRgn")
    }
    #[doc = "GetViewportExtEx from GDI32"]
    fn GetViewportExtEx(
        &self,
        hdc: HDC,
        lpsize: MutPtr<super::super::Foundation::SIZE>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetViewportExtEx")
    }
    #[doc = "GetViewportOrgEx from GDI32"]
    fn GetViewportOrgEx(
        &self,
        hdc: HDC,
        lppoint: MutPtr<super::super::Foundation::POINT>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetViewportOrgEx")
    }
    #[doc = "GetWinMetaFileBits from GDI32"]
    fn GetWinMetaFileBits(
        &self,
        hemf: HENHMETAFILE,
        cb_data_16: u32,
        p_data_16: MutPtr<u8>,
        i_map_mode: i32,
        hdc_ref: HDC,
    ) -> u32 {
        todo!("GetWinMetaFileBits")
    }
    #[doc = "GetWindowDC from USER32"]
    fn GetWindowDC(&self, h_wnd: super::super::Foundation::HWND) -> HDC {
        todo!("GetWindowDC")
    }
    #[doc = "GetWindowExtEx from GDI32"]
    fn GetWindowExtEx(
        &self,
        hdc: HDC,
        lpsize: MutPtr<super::super::Foundation::SIZE>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetWindowExtEx")
    }
    #[doc = "GetWindowOrgEx from GDI32"]
    fn GetWindowOrgEx(
        &self,
        hdc: HDC,
        lppoint: MutPtr<super::super::Foundation::POINT>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetWindowOrgEx")
    }
    #[doc = "GetWindowRgn from USER32"]
    fn GetWindowRgn(&self, h_wnd: super::super::Foundation::HWND, h_rgn: HRGN) -> i32 {
        todo!("GetWindowRgn")
    }
    #[doc = "GetWindowRgnBox from USER32"]
    fn GetWindowRgnBox(
        &self,
        h_wnd: super::super::Foundation::HWND,
        lprc: MutPtr<super::super::Foundation::RECT>,
    ) -> i32 {
        todo!("GetWindowRgnBox")
    }
    #[doc = "GrayStringA from USER32"]
    fn GrayStringA(
        &self,
        h_dc: HDC,
        h_brush: HBRUSH,
        lp_output_func: GRAYSTRINGPROC,
        lp_data: super::super::Foundation::LPARAM,
        n_count: i32,
        x: i32,
        y: i32,
        n_width: i32,
        n_height: i32,
    ) -> super::super::Foundation::BOOL {
        todo!("GrayStringA")
    }
    #[doc = "GrayStringW from USER32"]
    fn GrayStringW(
        &self,
        h_dc: HDC,
        h_brush: HBRUSH,
        lp_output_func: GRAYSTRINGPROC,
        lp_data: super::super::Foundation::LPARAM,
        n_count: i32,
        x: i32,
        y: i32,
        n_width: i32,
        n_height: i32,
    ) -> super::super::Foundation::BOOL {
        todo!("GrayStringW")
    }
    #[doc = "InflateRect from USER32"]
    fn InflateRect(
        &self,
        lprc: MutPtr<super::super::Foundation::RECT>,
        dx: i32,
        dy: i32,
    ) -> super::super::Foundation::BOOL {
        todo!("InflateRect")
    }
    #[doc = "IntersectClipRect from GDI32"]
    fn IntersectClipRect(&self, hdc: HDC, left: i32, top: i32, right: i32, bottom: i32) -> i32 {
        todo!("IntersectClipRect")
    }
    #[doc = "IntersectRect from USER32"]
    fn IntersectRect(
        &self,
        lprc_dst: MutPtr<super::super::Foundation::RECT>,
        lprc_src_1: ConstPtr<super::super::Foundation::RECT>,
        lprc_src_2: ConstPtr<super::super::Foundation::RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!("IntersectRect")
    }
    #[doc = "InvalidateRect from USER32"]
    fn InvalidateRect(
        &self,
        h_wnd: super::super::Foundation::HWND,
        lp_rect: ConstPtr<super::super::Foundation::RECT>,
        b_erase: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!("InvalidateRect")
    }
    #[doc = "InvalidateRgn from USER32"]
    fn InvalidateRgn(
        &self,
        h_wnd: super::super::Foundation::HWND,
        h_rgn: HRGN,
        b_erase: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!("InvalidateRgn")
    }
    #[doc = "InvertRect from USER32"]
    fn InvertRect(
        &self,
        h_dc: HDC,
        lprc: ConstPtr<super::super::Foundation::RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!("InvertRect")
    }
    #[doc = "InvertRgn from GDI32"]
    fn InvertRgn(&self, hdc: HDC, hrgn: HRGN) -> super::super::Foundation::BOOL {
        todo!("InvertRgn")
    }
    #[doc = "IsRectEmpty from USER32"]
    fn IsRectEmpty(
        &self,
        lprc: ConstPtr<super::super::Foundation::RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!("IsRectEmpty")
    }
    #[doc = "LPtoDP from GDI32"]
    fn LPtoDP(
        &self,
        hdc: HDC,
        lppt: MutPtr<super::super::Foundation::POINT>,
        c: i32,
    ) -> super::super::Foundation::BOOL {
        todo!("LPtoDP")
    }
    #[doc = "LineDDA from GDI32"]
    fn LineDDA(
        &self,
        x_start: i32,
        y_start: i32,
        x_end: i32,
        y_end: i32,
        lp_proc: LINEDDAPROC,
        data: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::BOOL {
        todo!("LineDDA")
    }
    #[doc = "LineTo from GDI32"]
    fn LineTo(&self, hdc: HDC, x: i32, y: i32) -> super::super::Foundation::BOOL {
        todo!("LineTo")
    }
    #[doc = "LoadBitmapA from USER32"]
    fn LoadBitmapA(
        &self,
        h_instance: super::super::Foundation::HINSTANCE,
        lp_bitmap_name: PCSTR,
    ) -> HBITMAP {
        todo!("LoadBitmapA")
    }
    #[doc = "LoadBitmapW from USER32"]
    fn LoadBitmapW(
        &self,
        h_instance: super::super::Foundation::HINSTANCE,
        lp_bitmap_name: PCWSTR,
    ) -> HBITMAP {
        todo!("LoadBitmapW")
    }
    #[doc = "LockWindowUpdate from USER32"]
    fn LockWindowUpdate(
        &self,
        h_wnd_lock: super::super::Foundation::HWND,
    ) -> super::super::Foundation::BOOL {
        todo!("LockWindowUpdate")
    }
    #[doc = "MapWindowPoints from USER32"]
    fn MapWindowPoints(
        &self,
        h_wnd_from: super::super::Foundation::HWND,
        h_wnd_to: super::super::Foundation::HWND,
        lp_points: MutPtr<super::super::Foundation::POINT>,
        c_points: u32,
    ) -> i32 {
        todo!("MapWindowPoints")
    }
    #[doc = "MaskBlt from GDI32"]
    fn MaskBlt(
        &self,
        hdc_dest: HDC,
        x_dest: i32,
        y_dest: i32,
        width: i32,
        height: i32,
        hdc_src: HDC,
        x_src: i32,
        y_src: i32,
        hbm_mask: HBITMAP,
        x_mask: i32,
        y_mask: i32,
        rop: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("MaskBlt")
    }
    #[doc = "MonitorFromPoint from USER32"]
    fn MonitorFromPoint(
        &self,
        pt: super::super::Foundation::POINT,
        dw_flags: MONITOR_FROM_FLAGS,
    ) -> HMONITOR {
        todo!("MonitorFromPoint")
    }
    #[doc = "MonitorFromRect from USER32"]
    fn MonitorFromRect(
        &self,
        lprc: ConstPtr<super::super::Foundation::RECT>,
        dw_flags: MONITOR_FROM_FLAGS,
    ) -> HMONITOR {
        todo!("MonitorFromRect")
    }
    #[doc = "MonitorFromWindow from USER32"]
    fn MonitorFromWindow(
        &self,
        hwnd: super::super::Foundation::HWND,
        dw_flags: MONITOR_FROM_FLAGS,
    ) -> HMONITOR {
        todo!("MonitorFromWindow")
    }
    #[doc = "MoveToEx from GDI32"]
    fn MoveToEx(
        &self,
        hdc: HDC,
        x: i32,
        y: i32,
        lppt: MutPtr<super::super::Foundation::POINT>,
    ) -> super::super::Foundation::BOOL {
        todo!("MoveToEx")
    }
    #[doc = "OffsetClipRgn from GDI32"]
    fn OffsetClipRgn(&self, hdc: HDC, x: i32, y: i32) -> i32 {
        todo!("OffsetClipRgn")
    }
    #[doc = "OffsetRect from USER32"]
    fn OffsetRect(
        &self,
        lprc: MutPtr<super::super::Foundation::RECT>,
        dx: i32,
        dy: i32,
    ) -> super::super::Foundation::BOOL {
        todo!("OffsetRect")
    }
    #[doc = "OffsetRgn from GDI32"]
    fn OffsetRgn(&self, hrgn: HRGN, x: i32, y: i32) -> i32 {
        todo!("OffsetRgn")
    }
    #[doc = "OffsetViewportOrgEx from GDI32"]
    fn OffsetViewportOrgEx(
        &self,
        hdc: HDC,
        x: i32,
        y: i32,
        lppt: MutPtr<super::super::Foundation::POINT>,
    ) -> super::super::Foundation::BOOL {
        todo!("OffsetViewportOrgEx")
    }
    #[doc = "OffsetWindowOrgEx from GDI32"]
    fn OffsetWindowOrgEx(
        &self,
        hdc: HDC,
        x: i32,
        y: i32,
        lppt: MutPtr<super::super::Foundation::POINT>,
    ) -> super::super::Foundation::BOOL {
        todo!("OffsetWindowOrgEx")
    }
    #[doc = "PaintDesktop from USER32"]
    fn PaintDesktop(&self, hdc: HDC) -> super::super::Foundation::BOOL {
        todo!("PaintDesktop")
    }
    #[doc = "PaintRgn from GDI32"]
    fn PaintRgn(&self, hdc: HDC, hrgn: HRGN) -> super::super::Foundation::BOOL {
        todo!("PaintRgn")
    }
    #[doc = "PatBlt from GDI32"]
    fn PatBlt(
        &self,
        hdc: HDC,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        rop: ROP_CODE,
    ) -> super::super::Foundation::BOOL {
        todo!("PatBlt")
    }
    #[doc = "PathToRegion from GDI32"]
    fn PathToRegion(&self, hdc: HDC) -> HRGN {
        todo!("PathToRegion")
    }
    #[doc = "Pie from GDI32"]
    fn Pie(
        &self,
        hdc: HDC,
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
        xr_1: i32,
        yr_1: i32,
        xr_2: i32,
        yr_2: i32,
    ) -> super::super::Foundation::BOOL {
        todo!("Pie")
    }
    #[doc = "PlayEnhMetaFile from GDI32"]
    fn PlayEnhMetaFile(
        &self,
        hdc: HDC,
        hmf: HENHMETAFILE,
        lprect: ConstPtr<super::super::Foundation::RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!("PlayEnhMetaFile")
    }
    #[doc = "PlayEnhMetaFileRecord from GDI32"]
    fn PlayEnhMetaFileRecord(
        &self,
        hdc: HDC,
        pht: ConstPtr<HANDLETABLE>,
        pmr: ConstPtr<ENHMETARECORD>,
        cht: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("PlayEnhMetaFileRecord")
    }
    #[doc = "PlayMetaFile from GDI32"]
    fn PlayMetaFile(&self, hdc: HDC, hmf: HMETAFILE) -> super::super::Foundation::BOOL {
        todo!("PlayMetaFile")
    }
    #[doc = "PlayMetaFileRecord from GDI32"]
    fn PlayMetaFileRecord(
        &self,
        hdc: HDC,
        lp_handle_table: ConstPtr<HANDLETABLE>,
        lp_mr: ConstPtr<METARECORD>,
        no_objs: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("PlayMetaFileRecord")
    }
    #[doc = "PlgBlt from GDI32"]
    fn PlgBlt(
        &self,
        hdc_dest: HDC,
        lp_point: ConstPtr<super::super::Foundation::POINT>,
        hdc_src: HDC,
        x_src: i32,
        y_src: i32,
        width: i32,
        height: i32,
        hbm_mask: HBITMAP,
        x_mask: i32,
        y_mask: i32,
    ) -> super::super::Foundation::BOOL {
        todo!("PlgBlt")
    }
    #[doc = "PolyBezier from GDI32"]
    fn PolyBezier(
        &self,
        hdc: HDC,
        apt: ConstPtr<super::super::Foundation::POINT>,
        cpt: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("PolyBezier")
    }
    #[doc = "PolyBezierTo from GDI32"]
    fn PolyBezierTo(
        &self,
        hdc: HDC,
        apt: ConstPtr<super::super::Foundation::POINT>,
        cpt: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("PolyBezierTo")
    }
    #[doc = "PolyDraw from GDI32"]
    fn PolyDraw(
        &self,
        hdc: HDC,
        apt: ConstPtr<super::super::Foundation::POINT>,
        aj: ConstPtr<u8>,
        cpt: i32,
    ) -> super::super::Foundation::BOOL {
        todo!("PolyDraw")
    }
    #[doc = "PolyPolygon from GDI32"]
    fn PolyPolygon(
        &self,
        hdc: HDC,
        apt: ConstPtr<super::super::Foundation::POINT>,
        asz: ConstPtr<i32>,
        csz: i32,
    ) -> super::super::Foundation::BOOL {
        todo!("PolyPolygon")
    }
    #[doc = "PolyPolyline from GDI32"]
    fn PolyPolyline(
        &self,
        hdc: HDC,
        apt: ConstPtr<super::super::Foundation::POINT>,
        asz: ConstPtr<u32>,
        csz: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("PolyPolyline")
    }
    #[doc = "PolyTextOutA from GDI32"]
    fn PolyTextOutA(
        &self,
        hdc: HDC,
        ppt: ConstPtr<POLYTEXTA>,
        nstrings: i32,
    ) -> super::super::Foundation::BOOL {
        todo!("PolyTextOutA")
    }
    #[doc = "PolyTextOutW from GDI32"]
    fn PolyTextOutW(
        &self,
        hdc: HDC,
        ppt: ConstPtr<POLYTEXTW>,
        nstrings: i32,
    ) -> super::super::Foundation::BOOL {
        todo!("PolyTextOutW")
    }
    #[doc = "Polygon from GDI32"]
    fn Polygon(
        &self,
        hdc: HDC,
        apt: ConstPtr<super::super::Foundation::POINT>,
        cpt: i32,
    ) -> super::super::Foundation::BOOL {
        todo!("Polygon")
    }
    #[doc = "Polyline from GDI32"]
    fn Polyline(
        &self,
        hdc: HDC,
        apt: ConstPtr<super::super::Foundation::POINT>,
        cpt: i32,
    ) -> super::super::Foundation::BOOL {
        todo!("Polyline")
    }
    #[doc = "PolylineTo from GDI32"]
    fn PolylineTo(
        &self,
        hdc: HDC,
        apt: ConstPtr<super::super::Foundation::POINT>,
        cpt: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("PolylineTo")
    }
    #[doc = "PtInRect from USER32"]
    fn PtInRect(
        &self,
        lprc: ConstPtr<super::super::Foundation::RECT>,
        pt: super::super::Foundation::POINT,
    ) -> super::super::Foundation::BOOL {
        todo!("PtInRect")
    }
    #[doc = "PtInRegion from GDI32"]
    fn PtInRegion(&self, hrgn: HRGN, x: i32, y: i32) -> super::super::Foundation::BOOL {
        todo!("PtInRegion")
    }
    #[doc = "PtVisible from GDI32"]
    fn PtVisible(&self, hdc: HDC, x: i32, y: i32) -> super::super::Foundation::BOOL {
        todo!("PtVisible")
    }
    #[doc = "RealizePalette from GDI32"]
    fn RealizePalette(&self, hdc: HDC) -> u32 {
        todo!("RealizePalette")
    }
    #[doc = "RectInRegion from GDI32"]
    fn RectInRegion(
        &self,
        hrgn: HRGN,
        lprect: ConstPtr<super::super::Foundation::RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!("RectInRegion")
    }
    #[doc = "RectVisible from GDI32"]
    fn RectVisible(
        &self,
        hdc: HDC,
        lprect: ConstPtr<super::super::Foundation::RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!("RectVisible")
    }
    #[doc = "Rectangle from GDI32"]
    fn Rectangle(
        &self,
        hdc: HDC,
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
    ) -> super::super::Foundation::BOOL {
        todo!("Rectangle")
    }
    #[doc = "RedrawWindow from USER32"]
    fn RedrawWindow(
        &self,
        h_wnd: super::super::Foundation::HWND,
        lprc_update: ConstPtr<super::super::Foundation::RECT>,
        hrgn_update: HRGN,
        flags: REDRAW_WINDOW_FLAGS,
    ) -> super::super::Foundation::BOOL {
        todo!("RedrawWindow")
    }
    #[doc = "ReleaseDC from USER32"]
    fn ReleaseDC(&self, h_wnd: super::super::Foundation::HWND, h_dc: HDC) -> i32 {
        todo!("ReleaseDC")
    }
    #[doc = "RemoveFontMemResourceEx from GDI32"]
    fn RemoveFontMemResourceEx(
        &self,
        h: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("RemoveFontMemResourceEx")
    }
    #[doc = "RemoveFontResourceA from GDI32"]
    fn RemoveFontResourceA(&self, lp_file_name: PCSTR) -> super::super::Foundation::BOOL {
        todo!("RemoveFontResourceA")
    }
    #[doc = "RemoveFontResourceExA from GDI32"]
    fn RemoveFontResourceExA(
        &self,
        name: PCSTR,
        fl: u32,
        pdv: MutPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!("RemoveFontResourceExA")
    }
    #[doc = "RemoveFontResourceExW from GDI32"]
    fn RemoveFontResourceExW(
        &self,
        name: PCWSTR,
        fl: u32,
        pdv: MutPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!("RemoveFontResourceExW")
    }
    #[doc = "RemoveFontResourceW from GDI32"]
    fn RemoveFontResourceW(&self, lp_file_name: PCWSTR) -> super::super::Foundation::BOOL {
        todo!("RemoveFontResourceW")
    }
    #[doc = "ResetDCA from GDI32"]
    fn ResetDCA(&self, hdc: HDC, lpdm: ConstPtr<DEVMODEA>) -> HDC {
        todo!("ResetDCA")
    }
    #[doc = "ResetDCW from GDI32"]
    fn ResetDCW(&self, hdc: HDC, lpdm: ConstPtr<DEVMODEW>) -> HDC {
        todo!("ResetDCW")
    }
    #[doc = "ResizePalette from GDI32"]
    fn ResizePalette(&self, hpal: HPALETTE, n: u32) -> super::super::Foundation::BOOL {
        todo!("ResizePalette")
    }
    #[doc = "RestoreDC from GDI32"]
    fn RestoreDC(&self, hdc: HDC, n_saved_dc: i32) -> super::super::Foundation::BOOL {
        todo!("RestoreDC")
    }
    #[doc = "RoundRect from GDI32"]
    fn RoundRect(
        &self,
        hdc: HDC,
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
        width: i32,
        height: i32,
    ) -> super::super::Foundation::BOOL {
        todo!("RoundRect")
    }
    #[doc = "SaveDC from GDI32"]
    fn SaveDC(&self, hdc: HDC) -> i32 {
        todo!("SaveDC")
    }
    #[doc = "ScaleViewportExtEx from GDI32"]
    fn ScaleViewportExtEx(
        &self,
        hdc: HDC,
        xn: i32,
        dx: i32,
        yn: i32,
        yd: i32,
        lpsz: MutPtr<super::super::Foundation::SIZE>,
    ) -> super::super::Foundation::BOOL {
        todo!("ScaleViewportExtEx")
    }
    #[doc = "ScaleWindowExtEx from GDI32"]
    fn ScaleWindowExtEx(
        &self,
        hdc: HDC,
        xn: i32,
        xd: i32,
        yn: i32,
        yd: i32,
        lpsz: MutPtr<super::super::Foundation::SIZE>,
    ) -> super::super::Foundation::BOOL {
        todo!("ScaleWindowExtEx")
    }
    #[doc = "ScreenToClient from USER32"]
    fn ScreenToClient(
        &self,
        h_wnd: super::super::Foundation::HWND,
        lp_point: MutPtr<super::super::Foundation::POINT>,
    ) -> super::super::Foundation::BOOL {
        todo!("ScreenToClient")
    }
    #[doc = "SelectClipPath from GDI32"]
    fn SelectClipPath(&self, hdc: HDC, mode: RGN_COMBINE_MODE) -> super::super::Foundation::BOOL {
        todo!("SelectClipPath")
    }
    #[doc = "SelectClipRgn from GDI32"]
    fn SelectClipRgn(&self, hdc: HDC, hrgn: HRGN) -> i32 {
        todo!("SelectClipRgn")
    }
    #[doc = "SelectObject from GDI32"]
    fn SelectObject(&self, hdc: HDC, h: HGDIOBJ) -> HGDIOBJ {
        todo!("SelectObject")
    }
    #[doc = "SelectPalette from GDI32"]
    fn SelectPalette(
        &self,
        hdc: HDC,
        h_pal: HPALETTE,
        b_force_bkgd: super::super::Foundation::BOOL,
    ) -> HPALETTE {
        todo!("SelectPalette")
    }
    #[doc = "SetArcDirection from GDI32"]
    fn SetArcDirection(&self, hdc: HDC, dir: ARC_DIRECTION) -> i32 {
        todo!("SetArcDirection")
    }
    #[doc = "SetBitmapBits from GDI32"]
    fn SetBitmapBits(&self, hbm: HBITMAP, cb: u32, pv_bits: ConstPtr<::core::ffi::c_void>) -> i32 {
        todo!("SetBitmapBits")
    }
    #[doc = "SetBitmapDimensionEx from GDI32"]
    fn SetBitmapDimensionEx(
        &self,
        hbm: HBITMAP,
        w: i32,
        h: i32,
        lpsz: MutPtr<super::super::Foundation::SIZE>,
    ) -> super::super::Foundation::BOOL {
        todo!("SetBitmapDimensionEx")
    }
    #[doc = "SetBkColor from GDI32"]
    fn SetBkColor(&self, hdc: HDC, color: u32) -> u32 {
        todo!("SetBkColor")
    }
    #[doc = "SetBkMode from GDI32"]
    fn SetBkMode(&self, hdc: HDC, mode: BACKGROUND_MODE) -> i32 {
        todo!("SetBkMode")
    }
    #[doc = "SetBoundsRect from GDI32"]
    fn SetBoundsRect(
        &self,
        hdc: HDC,
        lprect: ConstPtr<super::super::Foundation::RECT>,
        flags: SET_BOUNDS_RECT_FLAGS,
    ) -> u32 {
        todo!("SetBoundsRect")
    }
    #[doc = "SetBrushOrgEx from GDI32"]
    fn SetBrushOrgEx(
        &self,
        hdc: HDC,
        x: i32,
        y: i32,
        lppt: MutPtr<super::super::Foundation::POINT>,
    ) -> super::super::Foundation::BOOL {
        todo!("SetBrushOrgEx")
    }
    #[doc = "SetColorAdjustment from GDI32"]
    fn SetColorAdjustment(
        &self,
        hdc: HDC,
        lpca: ConstPtr<COLORADJUSTMENT>,
    ) -> super::super::Foundation::BOOL {
        todo!("SetColorAdjustment")
    }
    #[doc = "SetDCBrushColor from GDI32"]
    fn SetDCBrushColor(&self, hdc: HDC, color: u32) -> u32 {
        todo!("SetDCBrushColor")
    }
    #[doc = "SetDCPenColor from GDI32"]
    fn SetDCPenColor(&self, hdc: HDC, color: u32) -> u32 {
        todo!("SetDCPenColor")
    }
    #[doc = "SetDIBColorTable from GDI32"]
    fn SetDIBColorTable(
        &self,
        hdc: HDC,
        i_start: u32,
        c_entries: u32,
        prgbq: ConstPtr<RGBQUAD>,
    ) -> u32 {
        todo!("SetDIBColorTable")
    }
    #[doc = "SetDIBits from GDI32"]
    fn SetDIBits(
        &self,
        hdc: HDC,
        hbm: HBITMAP,
        start: u32,
        c_lines: u32,
        lp_bits: ConstPtr<::core::ffi::c_void>,
        lpbmi: ConstPtr<BITMAPINFO>,
        color_use: DIB_USAGE,
    ) -> i32 {
        todo!("SetDIBits")
    }
    #[doc = "SetDIBitsToDevice from GDI32"]
    fn SetDIBitsToDevice(
        &self,
        hdc: HDC,
        x_dest: i32,
        y_dest: i32,
        w: u32,
        h: u32,
        x_src: i32,
        y_src: i32,
        start_scan: u32,
        c_lines: u32,
        lpv_bits: ConstPtr<::core::ffi::c_void>,
        lpbmi: ConstPtr<BITMAPINFO>,
        color_use: DIB_USAGE,
    ) -> i32 {
        todo!("SetDIBitsToDevice")
    }
    #[doc = "SetEnhMetaFileBits from GDI32"]
    fn SetEnhMetaFileBits(&self, n_size: u32, pb: ConstPtr<u8>) -> HENHMETAFILE {
        todo!("SetEnhMetaFileBits")
    }
    #[doc = "SetGraphicsMode from GDI32"]
    fn SetGraphicsMode(&self, hdc: HDC, i_mode: GRAPHICS_MODE) -> i32 {
        todo!("SetGraphicsMode")
    }
    #[doc = "SetLayout from GDI32"]
    fn SetLayout(&self, hdc: HDC, l: DC_LAYOUT) -> u32 {
        todo!("SetLayout")
    }
    #[doc = "SetMapMode from GDI32"]
    fn SetMapMode(&self, hdc: HDC, i_mode: HDC_MAP_MODE) -> i32 {
        todo!("SetMapMode")
    }
    #[doc = "SetMapperFlags from GDI32"]
    fn SetMapperFlags(&self, hdc: HDC, flags: u32) -> u32 {
        todo!("SetMapperFlags")
    }
    #[doc = "SetMetaFileBitsEx from GDI32"]
    fn SetMetaFileBitsEx(&self, cb_buffer: u32, lp_data: ConstPtr<u8>) -> HMETAFILE {
        todo!("SetMetaFileBitsEx")
    }
    #[doc = "SetMetaRgn from GDI32"]
    fn SetMetaRgn(&self, hdc: HDC) -> i32 {
        todo!("SetMetaRgn")
    }
    #[doc = "SetPaletteEntries from GDI32"]
    fn SetPaletteEntries(
        &self,
        hpal: HPALETTE,
        i_start: u32,
        c_entries: u32,
        p_pal_entries: ConstPtr<PALETTEENTRY>,
    ) -> u32 {
        todo!("SetPaletteEntries")
    }
    #[doc = "SetPixel from GDI32"]
    fn SetPixel(&self, hdc: HDC, x: i32, y: i32, color: u32) -> u32 {
        todo!("SetPixel")
    }
    #[doc = "SetPixelV from GDI32"]
    fn SetPixelV(&self, hdc: HDC, x: i32, y: i32, color: u32) -> super::super::Foundation::BOOL {
        todo!("SetPixelV")
    }
    #[doc = "SetPolyFillMode from GDI32"]
    fn SetPolyFillMode(&self, hdc: HDC, mode: CREATE_POLYGON_RGN_MODE) -> i32 {
        todo!("SetPolyFillMode")
    }
    #[doc = "SetROP2 from GDI32"]
    fn SetROP2(&self, hdc: HDC, rop_2: R2_MODE) -> i32 {
        todo!("SetROP2")
    }
    #[doc = "SetRect from USER32"]
    fn SetRect(
        &self,
        lprc: MutPtr<super::super::Foundation::RECT>,
        x_left: i32,
        y_top: i32,
        x_right: i32,
        y_bottom: i32,
    ) -> super::super::Foundation::BOOL {
        todo!("SetRect")
    }
    #[doc = "SetRectEmpty from USER32"]
    fn SetRectEmpty(
        &self,
        lprc: MutPtr<super::super::Foundation::RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!("SetRectEmpty")
    }
    #[doc = "SetRectRgn from GDI32"]
    fn SetRectRgn(
        &self,
        hrgn: HRGN,
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
    ) -> super::super::Foundation::BOOL {
        todo!("SetRectRgn")
    }
    #[doc = "SetStretchBltMode from GDI32"]
    fn SetStretchBltMode(&self, hdc: HDC, mode: STRETCH_BLT_MODE) -> i32 {
        todo!("SetStretchBltMode")
    }
    #[doc = "SetSystemPaletteUse from GDI32"]
    fn SetSystemPaletteUse(&self, hdc: HDC, r#use: SYSTEM_PALETTE_USE) -> u32 {
        todo!("SetSystemPaletteUse")
    }
    #[doc = "SetTextAlign from GDI32"]
    fn SetTextAlign(&self, hdc: HDC, align: TEXT_ALIGN_OPTIONS) -> u32 {
        todo!("SetTextAlign")
    }
    #[doc = "SetTextCharacterExtra from GDI32"]
    fn SetTextCharacterExtra(&self, hdc: HDC, extra: i32) -> i32 {
        todo!("SetTextCharacterExtra")
    }
    #[doc = "SetTextColor from GDI32"]
    fn SetTextColor(&self, hdc: HDC, color: u32) -> u32 {
        todo!("SetTextColor")
    }
    #[doc = "SetTextJustification from GDI32"]
    fn SetTextJustification(
        &self,
        hdc: HDC,
        extra: i32,
        count: i32,
    ) -> super::super::Foundation::BOOL {
        todo!("SetTextJustification")
    }
    #[doc = "SetViewportExtEx from GDI32"]
    fn SetViewportExtEx(
        &self,
        hdc: HDC,
        x: i32,
        y: i32,
        lpsz: MutPtr<super::super::Foundation::SIZE>,
    ) -> super::super::Foundation::BOOL {
        todo!("SetViewportExtEx")
    }
    #[doc = "SetViewportOrgEx from GDI32"]
    fn SetViewportOrgEx(
        &self,
        hdc: HDC,
        x: i32,
        y: i32,
        lppt: MutPtr<super::super::Foundation::POINT>,
    ) -> super::super::Foundation::BOOL {
        todo!("SetViewportOrgEx")
    }
    #[doc = "SetWindowExtEx from GDI32"]
    fn SetWindowExtEx(
        &self,
        hdc: HDC,
        x: i32,
        y: i32,
        lpsz: MutPtr<super::super::Foundation::SIZE>,
    ) -> super::super::Foundation::BOOL {
        todo!("SetWindowExtEx")
    }
    #[doc = "SetWindowOrgEx from GDI32"]
    fn SetWindowOrgEx(
        &self,
        hdc: HDC,
        x: i32,
        y: i32,
        lppt: MutPtr<super::super::Foundation::POINT>,
    ) -> super::super::Foundation::BOOL {
        todo!("SetWindowOrgEx")
    }
    #[doc = "SetWindowRgn from USER32"]
    fn SetWindowRgn(
        &self,
        h_wnd: super::super::Foundation::HWND,
        h_rgn: HRGN,
        b_redraw: super::super::Foundation::BOOL,
    ) -> i32 {
        todo!("SetWindowRgn")
    }
    #[doc = "StretchBlt from GDI32"]
    fn StretchBlt(
        &self,
        hdc_dest: HDC,
        x_dest: i32,
        y_dest: i32,
        w_dest: i32,
        h_dest: i32,
        hdc_src: HDC,
        x_src: i32,
        y_src: i32,
        w_src: i32,
        h_src: i32,
        rop: ROP_CODE,
    ) -> super::super::Foundation::BOOL {
        todo!("StretchBlt")
    }
    #[doc = "StretchDIBits from GDI32"]
    fn StretchDIBits(
        &self,
        hdc: HDC,
        x_dest: i32,
        y_dest: i32,
        dest_width: i32,
        dest_height: i32,
        x_src: i32,
        y_src: i32,
        src_width: i32,
        src_height: i32,
        lp_bits: ConstPtr<::core::ffi::c_void>,
        lpbmi: ConstPtr<BITMAPINFO>,
        i_usage: DIB_USAGE,
        rop: ROP_CODE,
    ) -> i32 {
        todo!("StretchDIBits")
    }
    #[doc = "StrokeAndFillPath from GDI32"]
    fn StrokeAndFillPath(&self, hdc: HDC) -> super::super::Foundation::BOOL {
        todo!("StrokeAndFillPath")
    }
    #[doc = "StrokePath from GDI32"]
    fn StrokePath(&self, hdc: HDC) -> super::super::Foundation::BOOL {
        todo!("StrokePath")
    }
    #[doc = "SubtractRect from USER32"]
    fn SubtractRect(
        &self,
        lprc_dst: MutPtr<super::super::Foundation::RECT>,
        lprc_src_1: ConstPtr<super::super::Foundation::RECT>,
        lprc_src_2: ConstPtr<super::super::Foundation::RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!("SubtractRect")
    }
    #[doc = "TabbedTextOutA from USER32"]
    fn TabbedTextOutA(
        &self,
        hdc: HDC,
        x: i32,
        y: i32,
        lp_string: PCSTR,
        ch_count: i32,
        n_tab_positions: i32,
        lpn_tab_stop_positions: ConstPtr<i32>,
        n_tab_origin: i32,
    ) -> i32 {
        todo!("TabbedTextOutA")
    }
    #[doc = "TabbedTextOutW from USER32"]
    fn TabbedTextOutW(
        &self,
        hdc: HDC,
        x: i32,
        y: i32,
        lp_string: PCWSTR,
        ch_count: i32,
        n_tab_positions: i32,
        lpn_tab_stop_positions: ConstPtr<i32>,
        n_tab_origin: i32,
    ) -> i32 {
        todo!("TabbedTextOutW")
    }
    #[doc = "TextOutA from GDI32"]
    fn TextOutA(
        &self,
        hdc: HDC,
        x: i32,
        y: i32,
        lp_string: PCSTR,
        c: i32,
    ) -> super::super::Foundation::BOOL {
        todo!("TextOutA")
    }
    #[doc = "TextOutW from GDI32"]
    fn TextOutW(
        &self,
        hdc: HDC,
        x: i32,
        y: i32,
        lp_string: PCWSTR,
        c: i32,
    ) -> super::super::Foundation::BOOL {
        todo!("TextOutW")
    }
    #[doc = "UnionRect from USER32"]
    fn UnionRect(
        &self,
        lprc_dst: MutPtr<super::super::Foundation::RECT>,
        lprc_src_1: ConstPtr<super::super::Foundation::RECT>,
        lprc_src_2: ConstPtr<super::super::Foundation::RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!("UnionRect")
    }
    #[doc = "UnrealizeObject from GDI32"]
    fn UnrealizeObject(&self, h: HGDIOBJ) -> super::super::Foundation::BOOL {
        todo!("UnrealizeObject")
    }
    #[doc = "UpdateColors from GDI32"]
    fn UpdateColors(&self, hdc: HDC) -> super::super::Foundation::BOOL {
        todo!("UpdateColors")
    }
    #[doc = "UpdateWindow from USER32"]
    fn UpdateWindow(
        &self,
        h_wnd: super::super::Foundation::HWND,
    ) -> super::super::Foundation::BOOL {
        todo!("UpdateWindow")
    }
    #[doc = "ValidateRect from USER32"]
    fn ValidateRect(
        &self,
        h_wnd: super::super::Foundation::HWND,
        lp_rect: ConstPtr<super::super::Foundation::RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!("ValidateRect")
    }
    #[doc = "ValidateRgn from USER32"]
    fn ValidateRgn(
        &self,
        h_wnd: super::super::Foundation::HWND,
        h_rgn: HRGN,
    ) -> super::super::Foundation::BOOL {
        todo!("ValidateRgn")
    }
    #[doc = "WidenPath from GDI32"]
    fn WidenPath(&self, hdc: HDC) -> super::super::Foundation::BOOL {
        todo!("WidenPath")
    }
    #[doc = "WindowFromDC from USER32"]
    fn WindowFromDC(&self, h_dc: HDC) -> super::super::Foundation::HWND {
        todo!("WindowFromDC")
    }
}
pub fn get_api(ctx: &crate::core::Win32Context) -> std::sync::Arc<dyn Api> {
    ctx.get::<dyn Api>()
}
