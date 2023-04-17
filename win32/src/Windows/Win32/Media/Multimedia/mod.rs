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
pub const ACMDM_BASE: u32 = 24576u32;
pub const ACM_MPEG_COPYRIGHT: u32 = 2u32;
pub const ACM_MPEG_DUALCHANNEL: u32 = 4u32;
pub const ACM_MPEG_ID_MPEG1: u32 = 16u32;
pub const ACM_MPEG_JOINTSTEREO: u32 = 2u32;
pub const ACM_MPEG_LAYER1: u32 = 1u32;
pub const ACM_MPEG_LAYER2: u32 = 2u32;
pub const ACM_MPEG_LAYER3: u32 = 4u32;
pub const ACM_MPEG_ORIGINALHOME: u32 = 4u32;
pub const ACM_MPEG_PRIVATEBIT: u32 = 1u32;
pub const ACM_MPEG_PROTECTIONBIT: u32 = 8u32;
pub const ACM_MPEG_SINGLECHANNEL: u32 = 8u32;
pub const ACM_MPEG_STEREO: u32 = 1u32;
pub struct ADPCMCOEFSET {
    pub iCoef1: i16,
    pub iCoef2: i16,
}
impl ::core::marker::Copy for ADPCMCOEFSET {}
impl ::core::clone::Clone for ADPCMCOEFSET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ADPCMCOEFSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADPCMCOEFSET")
            .field("iCoef1", &self.iCoef1)
            .field("iCoef2", &self.iCoef2)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ADPCMCOEFSET {
    fn eq(&self, other: &Self) -> bool {
        self.iCoef1 == other.iCoef1 && self.iCoef2 == other.iCoef2
    }
}
impl ::core::cmp::Eq for ADPCMCOEFSET {}
impl FromIntoMemory for ADPCMCOEFSET {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_iCoef1 = <i16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_iCoef2 = <i16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        Self {
            iCoef1: f_iCoef1,
            iCoef2: f_iCoef2,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.iCoef1, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.iCoef2, &mut into[2..2 + 2]);
    }
    fn size() -> usize {
        4
    }
}
pub struct ADPCMEWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
impl ::core::marker::Copy for ADPCMEWAVEFORMAT {}
impl ::core::clone::Clone for ADPCMEWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ADPCMEWAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADPCMEWAVEFORMAT")
            .field("wfx", &self.wfx)
            .field("wSamplesPerBlock", &self.wSamplesPerBlock)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ADPCMEWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx && self.wSamplesPerBlock == other.wSamplesPerBlock
    }
}
impl ::core::cmp::Eq for ADPCMEWAVEFORMAT {}
impl FromIntoMemory for ADPCMEWAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        let f_wSamplesPerBlock = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        Self {
            wfx: f_wfx,
            wSamplesPerBlock: f_wSamplesPerBlock,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
        FromIntoMemory::into_bytes(self.wSamplesPerBlock, &mut into[18..18 + 2]);
    }
    fn size() -> usize {
        20
    }
}
pub struct ADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
    pub wNumCoef: u16,
    pub aCoef: [ADPCMCOEFSET; 1],
}
impl ::core::marker::Copy for ADPCMWAVEFORMAT {}
impl ::core::clone::Clone for ADPCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ADPCMWAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADPCMWAVEFORMAT")
            .field("wfx", &self.wfx)
            .field("wSamplesPerBlock", &self.wSamplesPerBlock)
            .field("wNumCoef", &self.wNumCoef)
            .field("aCoef", &self.aCoef)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ADPCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx
            && self.wSamplesPerBlock == other.wSamplesPerBlock
            && self.wNumCoef == other.wNumCoef
            && self.aCoef == other.aCoef
    }
}
impl ::core::cmp::Eq for ADPCMWAVEFORMAT {}
impl FromIntoMemory for ADPCMWAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 26);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        let f_wSamplesPerBlock = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        let f_wNumCoef = <u16 as FromIntoMemory>::from_bytes(&from[20..20 + 2]);
        let f_aCoef = <[ADPCMCOEFSET; 1] as FromIntoMemory>::from_bytes(&from[22..22 + 4]);
        Self {
            wfx: f_wfx,
            wSamplesPerBlock: f_wSamplesPerBlock,
            wNumCoef: f_wNumCoef,
            aCoef: f_aCoef,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 26);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
        FromIntoMemory::into_bytes(self.wSamplesPerBlock, &mut into[18..18 + 2]);
        FromIntoMemory::into_bytes(self.wNumCoef, &mut into[20..20 + 2]);
        FromIntoMemory::into_bytes(self.aCoef, &mut into[22..22 + 4]);
    }
    fn size() -> usize {
        26
    }
}
pub struct APTXWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
impl ::core::marker::Copy for APTXWAVEFORMAT {}
impl ::core::clone::Clone for APTXWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for APTXWAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APTXWAVEFORMAT")
            .field("wfx", &self.wfx)
            .finish()
    }
}
impl ::core::cmp::PartialEq for APTXWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx
    }
}
impl ::core::cmp::Eq for APTXWAVEFORMAT {}
impl FromIntoMemory for APTXWAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 18);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        Self { wfx: f_wfx }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 18);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
    }
    fn size() -> usize {
        18
    }
}
pub struct AUDIOFILE_AF10WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
impl ::core::marker::Copy for AUDIOFILE_AF10WAVEFORMAT {}
impl ::core::clone::Clone for AUDIOFILE_AF10WAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUDIOFILE_AF10WAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIOFILE_AF10WAVEFORMAT")
            .field("wfx", &self.wfx)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AUDIOFILE_AF10WAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx
    }
}
impl ::core::cmp::Eq for AUDIOFILE_AF10WAVEFORMAT {}
impl FromIntoMemory for AUDIOFILE_AF10WAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 18);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        Self { wfx: f_wfx }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 18);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
    }
    fn size() -> usize {
        18
    }
}
pub struct AUDIOFILE_AF36WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
impl ::core::marker::Copy for AUDIOFILE_AF36WAVEFORMAT {}
impl ::core::clone::Clone for AUDIOFILE_AF36WAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUDIOFILE_AF36WAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIOFILE_AF36WAVEFORMAT")
            .field("wfx", &self.wfx)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AUDIOFILE_AF36WAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx
    }
}
impl ::core::cmp::Eq for AUDIOFILE_AF36WAVEFORMAT {}
impl FromIntoMemory for AUDIOFILE_AF36WAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 18);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        Self { wfx: f_wfx }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 18);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
    }
    fn size() -> usize {
        18
    }
}
pub const AUXDM_GETDEVCAPS: u32 = 4u32;
pub const AUXDM_GETNUMDEVS: u32 = 3u32;
pub const AUXDM_GETVOLUME: u32 = 5u32;
pub const AUXDM_SETVOLUME: u32 = 6u32;
pub const AUXM_INIT: u32 = 100u32;
pub const AUXM_INIT_EX: u32 = 104u32;
pub const AVICOMPRESSF_DATARATE: u32 = 2u32;
pub const AVICOMPRESSF_INTERLEAVE: u32 = 1u32;
pub const AVICOMPRESSF_KEYFRAMES: u32 = 4u32;
pub const AVICOMPRESSF_VALID: u32 = 8u32;
pub struct AVICOMPRESSOPTIONS {
    pub fccType: u32,
    pub fccHandler: u32,
    pub dwKeyFrameEvery: u32,
    pub dwQuality: u32,
    pub dwBytesPerSecond: u32,
    pub dwFlags: u32,
    pub lpFormat: MutPtr<::core::ffi::c_void>,
    pub cbFormat: u32,
    pub lpParms: MutPtr<::core::ffi::c_void>,
    pub cbParms: u32,
    pub dwInterleaveEvery: u32,
}
impl ::core::marker::Copy for AVICOMPRESSOPTIONS {}
impl ::core::clone::Clone for AVICOMPRESSOPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AVICOMPRESSOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AVICOMPRESSOPTIONS")
            .field("fccType", &self.fccType)
            .field("fccHandler", &self.fccHandler)
            .field("dwKeyFrameEvery", &self.dwKeyFrameEvery)
            .field("dwQuality", &self.dwQuality)
            .field("dwBytesPerSecond", &self.dwBytesPerSecond)
            .field("dwFlags", &self.dwFlags)
            .field("lpFormat", &self.lpFormat)
            .field("cbFormat", &self.cbFormat)
            .field("lpParms", &self.lpParms)
            .field("cbParms", &self.cbParms)
            .field("dwInterleaveEvery", &self.dwInterleaveEvery)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AVICOMPRESSOPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.fccType == other.fccType
            && self.fccHandler == other.fccHandler
            && self.dwKeyFrameEvery == other.dwKeyFrameEvery
            && self.dwQuality == other.dwQuality
            && self.dwBytesPerSecond == other.dwBytesPerSecond
            && self.dwFlags == other.dwFlags
            && self.lpFormat == other.lpFormat
            && self.cbFormat == other.cbFormat
            && self.lpParms == other.lpParms
            && self.cbParms == other.cbParms
            && self.dwInterleaveEvery == other.dwInterleaveEvery
    }
}
impl ::core::cmp::Eq for AVICOMPRESSOPTIONS {}
impl FromIntoMemory for AVICOMPRESSOPTIONS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 44);
        let f_fccType = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_fccHandler = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwKeyFrameEvery = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwQuality = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwBytesPerSecond = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_lpFormat =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_cbFormat = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_lpParms =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_cbParms = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_dwInterleaveEvery = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        Self {
            fccType: f_fccType,
            fccHandler: f_fccHandler,
            dwKeyFrameEvery: f_dwKeyFrameEvery,
            dwQuality: f_dwQuality,
            dwBytesPerSecond: f_dwBytesPerSecond,
            dwFlags: f_dwFlags,
            lpFormat: f_lpFormat,
            cbFormat: f_cbFormat,
            lpParms: f_lpParms,
            cbParms: f_cbParms,
            dwInterleaveEvery: f_dwInterleaveEvery,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 44);
        FromIntoMemory::into_bytes(self.fccType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.fccHandler, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwKeyFrameEvery, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwQuality, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwBytesPerSecond, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.lpFormat, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.cbFormat, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.lpParms, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.cbParms, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.dwInterleaveEvery, &mut into[40..40 + 4]);
    }
    fn size() -> usize {
        44
    }
}
pub const AVIERR_OK: i32 = 0i32;
pub const AVIFILECAPS_ALLKEYFRAMES: u32 = 16u32;
pub const AVIFILECAPS_CANREAD: u32 = 1u32;
pub const AVIFILECAPS_CANWRITE: u32 = 2u32;
pub const AVIFILECAPS_NOCOMPRESSION: u32 = 32u32;
pub const AVIFILEHANDLER_CANACCEPTNONRGB: u32 = 4u32;
pub const AVIFILEHANDLER_CANREAD: u32 = 1u32;
pub const AVIFILEHANDLER_CANWRITE: u32 = 2u32;
pub struct AVIFILEINFOA {
    pub dwMaxBytesPerSec: u32,
    pub dwFlags: u32,
    pub dwCaps: u32,
    pub dwStreams: u32,
    pub dwSuggestedBufferSize: u32,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwScale: u32,
    pub dwRate: u32,
    pub dwLength: u32,
    pub dwEditCount: u32,
    pub szFileType: [super::super::Foundation::CHAR; 64],
}
impl ::core::marker::Copy for AVIFILEINFOA {}
impl ::core::clone::Clone for AVIFILEINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AVIFILEINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AVIFILEINFOA")
            .field("dwMaxBytesPerSec", &self.dwMaxBytesPerSec)
            .field("dwFlags", &self.dwFlags)
            .field("dwCaps", &self.dwCaps)
            .field("dwStreams", &self.dwStreams)
            .field("dwSuggestedBufferSize", &self.dwSuggestedBufferSize)
            .field("dwWidth", &self.dwWidth)
            .field("dwHeight", &self.dwHeight)
            .field("dwScale", &self.dwScale)
            .field("dwRate", &self.dwRate)
            .field("dwLength", &self.dwLength)
            .field("dwEditCount", &self.dwEditCount)
            .field("szFileType", &self.szFileType)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AVIFILEINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwMaxBytesPerSec == other.dwMaxBytesPerSec
            && self.dwFlags == other.dwFlags
            && self.dwCaps == other.dwCaps
            && self.dwStreams == other.dwStreams
            && self.dwSuggestedBufferSize == other.dwSuggestedBufferSize
            && self.dwWidth == other.dwWidth
            && self.dwHeight == other.dwHeight
            && self.dwScale == other.dwScale
            && self.dwRate == other.dwRate
            && self.dwLength == other.dwLength
            && self.dwEditCount == other.dwEditCount
            && self.szFileType == other.szFileType
    }
}
impl ::core::cmp::Eq for AVIFILEINFOA {}
impl FromIntoMemory for AVIFILEINFOA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 108);
        let f_dwMaxBytesPerSec = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwCaps = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwStreams = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwSuggestedBufferSize = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwWidth = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwHeight = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_dwScale = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_dwRate = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_dwLength = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_dwEditCount = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_szFileType = <[super::super::Foundation::CHAR; 64] as FromIntoMemory>::from_bytes(
            &from[44..44 + 64],
        );
        Self {
            dwMaxBytesPerSec: f_dwMaxBytesPerSec,
            dwFlags: f_dwFlags,
            dwCaps: f_dwCaps,
            dwStreams: f_dwStreams,
            dwSuggestedBufferSize: f_dwSuggestedBufferSize,
            dwWidth: f_dwWidth,
            dwHeight: f_dwHeight,
            dwScale: f_dwScale,
            dwRate: f_dwRate,
            dwLength: f_dwLength,
            dwEditCount: f_dwEditCount,
            szFileType: f_szFileType,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 108);
        FromIntoMemory::into_bytes(self.dwMaxBytesPerSec, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwCaps, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwStreams, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwSuggestedBufferSize, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwWidth, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwHeight, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.dwScale, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.dwRate, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.dwLength, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.dwEditCount, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.szFileType, &mut into[44..44 + 64]);
    }
    fn size() -> usize {
        108
    }
}
pub struct AVIFILEINFOW {
    pub dwMaxBytesPerSec: u32,
    pub dwFlags: u32,
    pub dwCaps: u32,
    pub dwStreams: u32,
    pub dwSuggestedBufferSize: u32,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwScale: u32,
    pub dwRate: u32,
    pub dwLength: u32,
    pub dwEditCount: u32,
    pub szFileType: [u16; 64],
}
impl ::core::marker::Copy for AVIFILEINFOW {}
impl ::core::clone::Clone for AVIFILEINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AVIFILEINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AVIFILEINFOW")
            .field("dwMaxBytesPerSec", &self.dwMaxBytesPerSec)
            .field("dwFlags", &self.dwFlags)
            .field("dwCaps", &self.dwCaps)
            .field("dwStreams", &self.dwStreams)
            .field("dwSuggestedBufferSize", &self.dwSuggestedBufferSize)
            .field("dwWidth", &self.dwWidth)
            .field("dwHeight", &self.dwHeight)
            .field("dwScale", &self.dwScale)
            .field("dwRate", &self.dwRate)
            .field("dwLength", &self.dwLength)
            .field("dwEditCount", &self.dwEditCount)
            .field("szFileType", &self.szFileType)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AVIFILEINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwMaxBytesPerSec == other.dwMaxBytesPerSec
            && self.dwFlags == other.dwFlags
            && self.dwCaps == other.dwCaps
            && self.dwStreams == other.dwStreams
            && self.dwSuggestedBufferSize == other.dwSuggestedBufferSize
            && self.dwWidth == other.dwWidth
            && self.dwHeight == other.dwHeight
            && self.dwScale == other.dwScale
            && self.dwRate == other.dwRate
            && self.dwLength == other.dwLength
            && self.dwEditCount == other.dwEditCount
            && self.szFileType == other.szFileType
    }
}
impl ::core::cmp::Eq for AVIFILEINFOW {}
impl FromIntoMemory for AVIFILEINFOW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 108);
        let f_dwMaxBytesPerSec = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwCaps = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwStreams = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwSuggestedBufferSize = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwWidth = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwHeight = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_dwScale = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_dwRate = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_dwLength = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_dwEditCount = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_szFileType = <[u16; 64] as FromIntoMemory>::from_bytes(&from[44..44 + 64]);
        Self {
            dwMaxBytesPerSec: f_dwMaxBytesPerSec,
            dwFlags: f_dwFlags,
            dwCaps: f_dwCaps,
            dwStreams: f_dwStreams,
            dwSuggestedBufferSize: f_dwSuggestedBufferSize,
            dwWidth: f_dwWidth,
            dwHeight: f_dwHeight,
            dwScale: f_dwScale,
            dwRate: f_dwRate,
            dwLength: f_dwLength,
            dwEditCount: f_dwEditCount,
            szFileType: f_szFileType,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 108);
        FromIntoMemory::into_bytes(self.dwMaxBytesPerSec, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwCaps, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwStreams, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwSuggestedBufferSize, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwWidth, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwHeight, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.dwScale, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.dwRate, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.dwLength, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.dwEditCount, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.szFileType, &mut into[44..44 + 64]);
    }
    fn size() -> usize {
        108
    }
}
pub const AVIFILEINFO_COPYRIGHTED: u32 = 131072u32;
pub const AVIFILEINFO_HASINDEX: u32 = 16u32;
pub const AVIFILEINFO_ISINTERLEAVED: u32 = 256u32;
pub const AVIFILEINFO_MUSTUSEINDEX: u32 = 32u32;
pub const AVIFILEINFO_WASCAPTUREFILE: u32 = 65536u32;
pub const AVIGETFRAMEF_BESTDISPLAYFMT: u32 = 1u32;
pub const AVIIF_CONTROLFRAME: i32 = 512i32;
pub const AVIIF_TWOCC: i32 = 2i32;
pub type AVISAVECALLBACK = StdCallFnPtr<(i32,), super::super::Foundation::BOOL>;
pub struct AVISTREAMINFOA {
    pub fccType: u32,
    pub fccHandler: u32,
    pub dwFlags: u32,
    pub dwCaps: u32,
    pub wPriority: u16,
    pub wLanguage: u16,
    pub dwScale: u32,
    pub dwRate: u32,
    pub dwStart: u32,
    pub dwLength: u32,
    pub dwInitialFrames: u32,
    pub dwSuggestedBufferSize: u32,
    pub dwQuality: u32,
    pub dwSampleSize: u32,
    pub rcFrame: super::super::Foundation::RECT,
    pub dwEditCount: u32,
    pub dwFormatChangeCount: u32,
    pub szName: [super::super::Foundation::CHAR; 64],
}
impl ::core::marker::Copy for AVISTREAMINFOA {}
impl ::core::clone::Clone for AVISTREAMINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AVISTREAMINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AVISTREAMINFOA")
            .field("fccType", &self.fccType)
            .field("fccHandler", &self.fccHandler)
            .field("dwFlags", &self.dwFlags)
            .field("dwCaps", &self.dwCaps)
            .field("wPriority", &self.wPriority)
            .field("wLanguage", &self.wLanguage)
            .field("dwScale", &self.dwScale)
            .field("dwRate", &self.dwRate)
            .field("dwStart", &self.dwStart)
            .field("dwLength", &self.dwLength)
            .field("dwInitialFrames", &self.dwInitialFrames)
            .field("dwSuggestedBufferSize", &self.dwSuggestedBufferSize)
            .field("dwQuality", &self.dwQuality)
            .field("dwSampleSize", &self.dwSampleSize)
            .field("rcFrame", &self.rcFrame)
            .field("dwEditCount", &self.dwEditCount)
            .field("dwFormatChangeCount", &self.dwFormatChangeCount)
            .field("szName", &self.szName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AVISTREAMINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.fccType == other.fccType
            && self.fccHandler == other.fccHandler
            && self.dwFlags == other.dwFlags
            && self.dwCaps == other.dwCaps
            && self.wPriority == other.wPriority
            && self.wLanguage == other.wLanguage
            && self.dwScale == other.dwScale
            && self.dwRate == other.dwRate
            && self.dwStart == other.dwStart
            && self.dwLength == other.dwLength
            && self.dwInitialFrames == other.dwInitialFrames
            && self.dwSuggestedBufferSize == other.dwSuggestedBufferSize
            && self.dwQuality == other.dwQuality
            && self.dwSampleSize == other.dwSampleSize
            && self.rcFrame == other.rcFrame
            && self.dwEditCount == other.dwEditCount
            && self.dwFormatChangeCount == other.dwFormatChangeCount
            && self.szName == other.szName
    }
}
impl ::core::cmp::Eq for AVISTREAMINFOA {}
impl FromIntoMemory for AVISTREAMINFOA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 140);
        let f_fccType = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_fccHandler = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwCaps = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_wPriority = <u16 as FromIntoMemory>::from_bytes(&from[16..16 + 2]);
        let f_wLanguage = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        let f_dwScale = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwRate = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_dwStart = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_dwLength = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_dwInitialFrames = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_dwSuggestedBufferSize = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_dwQuality = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_dwSampleSize = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_rcFrame =
            <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[52..52 + 16]);
        let f_dwEditCount = <u32 as FromIntoMemory>::from_bytes(&from[68..68 + 4]);
        let f_dwFormatChangeCount = <u32 as FromIntoMemory>::from_bytes(&from[72..72 + 4]);
        let f_szName = <[super::super::Foundation::CHAR; 64] as FromIntoMemory>::from_bytes(
            &from[76..76 + 64],
        );
        Self {
            fccType: f_fccType,
            fccHandler: f_fccHandler,
            dwFlags: f_dwFlags,
            dwCaps: f_dwCaps,
            wPriority: f_wPriority,
            wLanguage: f_wLanguage,
            dwScale: f_dwScale,
            dwRate: f_dwRate,
            dwStart: f_dwStart,
            dwLength: f_dwLength,
            dwInitialFrames: f_dwInitialFrames,
            dwSuggestedBufferSize: f_dwSuggestedBufferSize,
            dwQuality: f_dwQuality,
            dwSampleSize: f_dwSampleSize,
            rcFrame: f_rcFrame,
            dwEditCount: f_dwEditCount,
            dwFormatChangeCount: f_dwFormatChangeCount,
            szName: f_szName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 140);
        FromIntoMemory::into_bytes(self.fccType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.fccHandler, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwCaps, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.wPriority, &mut into[16..16 + 2]);
        FromIntoMemory::into_bytes(self.wLanguage, &mut into[18..18 + 2]);
        FromIntoMemory::into_bytes(self.dwScale, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwRate, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.dwStart, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.dwLength, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.dwInitialFrames, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.dwSuggestedBufferSize, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.dwQuality, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.dwSampleSize, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.rcFrame, &mut into[52..52 + 16]);
        FromIntoMemory::into_bytes(self.dwEditCount, &mut into[68..68 + 4]);
        FromIntoMemory::into_bytes(self.dwFormatChangeCount, &mut into[72..72 + 4]);
        FromIntoMemory::into_bytes(self.szName, &mut into[76..76 + 64]);
    }
    fn size() -> usize {
        140
    }
}
pub struct AVISTREAMINFOW {
    pub fccType: u32,
    pub fccHandler: u32,
    pub dwFlags: u32,
    pub dwCaps: u32,
    pub wPriority: u16,
    pub wLanguage: u16,
    pub dwScale: u32,
    pub dwRate: u32,
    pub dwStart: u32,
    pub dwLength: u32,
    pub dwInitialFrames: u32,
    pub dwSuggestedBufferSize: u32,
    pub dwQuality: u32,
    pub dwSampleSize: u32,
    pub rcFrame: super::super::Foundation::RECT,
    pub dwEditCount: u32,
    pub dwFormatChangeCount: u32,
    pub szName: [u16; 64],
}
impl ::core::marker::Copy for AVISTREAMINFOW {}
impl ::core::clone::Clone for AVISTREAMINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AVISTREAMINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AVISTREAMINFOW")
            .field("fccType", &self.fccType)
            .field("fccHandler", &self.fccHandler)
            .field("dwFlags", &self.dwFlags)
            .field("dwCaps", &self.dwCaps)
            .field("wPriority", &self.wPriority)
            .field("wLanguage", &self.wLanguage)
            .field("dwScale", &self.dwScale)
            .field("dwRate", &self.dwRate)
            .field("dwStart", &self.dwStart)
            .field("dwLength", &self.dwLength)
            .field("dwInitialFrames", &self.dwInitialFrames)
            .field("dwSuggestedBufferSize", &self.dwSuggestedBufferSize)
            .field("dwQuality", &self.dwQuality)
            .field("dwSampleSize", &self.dwSampleSize)
            .field("rcFrame", &self.rcFrame)
            .field("dwEditCount", &self.dwEditCount)
            .field("dwFormatChangeCount", &self.dwFormatChangeCount)
            .field("szName", &self.szName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AVISTREAMINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.fccType == other.fccType
            && self.fccHandler == other.fccHandler
            && self.dwFlags == other.dwFlags
            && self.dwCaps == other.dwCaps
            && self.wPriority == other.wPriority
            && self.wLanguage == other.wLanguage
            && self.dwScale == other.dwScale
            && self.dwRate == other.dwRate
            && self.dwStart == other.dwStart
            && self.dwLength == other.dwLength
            && self.dwInitialFrames == other.dwInitialFrames
            && self.dwSuggestedBufferSize == other.dwSuggestedBufferSize
            && self.dwQuality == other.dwQuality
            && self.dwSampleSize == other.dwSampleSize
            && self.rcFrame == other.rcFrame
            && self.dwEditCount == other.dwEditCount
            && self.dwFormatChangeCount == other.dwFormatChangeCount
            && self.szName == other.szName
    }
}
impl ::core::cmp::Eq for AVISTREAMINFOW {}
impl FromIntoMemory for AVISTREAMINFOW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 140);
        let f_fccType = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_fccHandler = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwCaps = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_wPriority = <u16 as FromIntoMemory>::from_bytes(&from[16..16 + 2]);
        let f_wLanguage = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        let f_dwScale = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwRate = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_dwStart = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_dwLength = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_dwInitialFrames = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_dwSuggestedBufferSize = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_dwQuality = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_dwSampleSize = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_rcFrame =
            <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[52..52 + 16]);
        let f_dwEditCount = <u32 as FromIntoMemory>::from_bytes(&from[68..68 + 4]);
        let f_dwFormatChangeCount = <u32 as FromIntoMemory>::from_bytes(&from[72..72 + 4]);
        let f_szName = <[u16; 64] as FromIntoMemory>::from_bytes(&from[76..76 + 64]);
        Self {
            fccType: f_fccType,
            fccHandler: f_fccHandler,
            dwFlags: f_dwFlags,
            dwCaps: f_dwCaps,
            wPriority: f_wPriority,
            wLanguage: f_wLanguage,
            dwScale: f_dwScale,
            dwRate: f_dwRate,
            dwStart: f_dwStart,
            dwLength: f_dwLength,
            dwInitialFrames: f_dwInitialFrames,
            dwSuggestedBufferSize: f_dwSuggestedBufferSize,
            dwQuality: f_dwQuality,
            dwSampleSize: f_dwSampleSize,
            rcFrame: f_rcFrame,
            dwEditCount: f_dwEditCount,
            dwFormatChangeCount: f_dwFormatChangeCount,
            szName: f_szName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 140);
        FromIntoMemory::into_bytes(self.fccType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.fccHandler, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwCaps, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.wPriority, &mut into[16..16 + 2]);
        FromIntoMemory::into_bytes(self.wLanguage, &mut into[18..18 + 2]);
        FromIntoMemory::into_bytes(self.dwScale, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwRate, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.dwStart, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.dwLength, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.dwInitialFrames, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.dwSuggestedBufferSize, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.dwQuality, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.dwSampleSize, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.rcFrame, &mut into[52..52 + 16]);
        FromIntoMemory::into_bytes(self.dwEditCount, &mut into[68..68 + 4]);
        FromIntoMemory::into_bytes(self.dwFormatChangeCount, &mut into[72..72 + 4]);
        FromIntoMemory::into_bytes(self.szName, &mut into[76..76 + 64]);
    }
    fn size() -> usize {
        140
    }
}
pub const AVISTREAMINFO_DISABLED: u32 = 1u32;
pub const AVISTREAMINFO_FORMATCHANGES: u32 = 65536u32;
pub const AVISTREAMREAD_CONVENIENT: i32 = -1i32;
pub const AVSTREAMMASTER_AUDIO: u32 = 0u32;
pub const AVSTREAMMASTER_NONE: u32 = 1u32;
pub const BI_1632: u32 = 842217009u32;
pub type CAPCONTROLCALLBACK =
    StdCallFnPtr<(super::super::Foundation::HWND, i32), super::super::Foundation::LRESULT>;
pub struct CAPDRIVERCAPS {
    pub wDeviceIndex: u32,
    pub fHasOverlay: super::super::Foundation::BOOL,
    pub fHasDlgVideoSource: super::super::Foundation::BOOL,
    pub fHasDlgVideoFormat: super::super::Foundation::BOOL,
    pub fHasDlgVideoDisplay: super::super::Foundation::BOOL,
    pub fCaptureInitialized: super::super::Foundation::BOOL,
    pub fDriverSuppliesPalettes: super::super::Foundation::BOOL,
    pub hVideoIn: super::super::Foundation::HANDLE,
    pub hVideoOut: super::super::Foundation::HANDLE,
    pub hVideoExtIn: super::super::Foundation::HANDLE,
    pub hVideoExtOut: super::super::Foundation::HANDLE,
}
impl ::core::marker::Copy for CAPDRIVERCAPS {}
impl ::core::clone::Clone for CAPDRIVERCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CAPDRIVERCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAPDRIVERCAPS")
            .field("wDeviceIndex", &self.wDeviceIndex)
            .field("fHasOverlay", &self.fHasOverlay)
            .field("fHasDlgVideoSource", &self.fHasDlgVideoSource)
            .field("fHasDlgVideoFormat", &self.fHasDlgVideoFormat)
            .field("fHasDlgVideoDisplay", &self.fHasDlgVideoDisplay)
            .field("fCaptureInitialized", &self.fCaptureInitialized)
            .field("fDriverSuppliesPalettes", &self.fDriverSuppliesPalettes)
            .field("hVideoIn", &self.hVideoIn)
            .field("hVideoOut", &self.hVideoOut)
            .field("hVideoExtIn", &self.hVideoExtIn)
            .field("hVideoExtOut", &self.hVideoExtOut)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CAPDRIVERCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.wDeviceIndex == other.wDeviceIndex
            && self.fHasOverlay == other.fHasOverlay
            && self.fHasDlgVideoSource == other.fHasDlgVideoSource
            && self.fHasDlgVideoFormat == other.fHasDlgVideoFormat
            && self.fHasDlgVideoDisplay == other.fHasDlgVideoDisplay
            && self.fCaptureInitialized == other.fCaptureInitialized
            && self.fDriverSuppliesPalettes == other.fDriverSuppliesPalettes
            && self.hVideoIn == other.hVideoIn
            && self.hVideoOut == other.hVideoOut
            && self.hVideoExtIn == other.hVideoExtIn
            && self.hVideoExtOut == other.hVideoExtOut
    }
}
impl ::core::cmp::Eq for CAPDRIVERCAPS {}
impl FromIntoMemory for CAPDRIVERCAPS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 44);
        let f_wDeviceIndex = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_fHasOverlay =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_fHasDlgVideoSource =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_fHasDlgVideoFormat =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_fHasDlgVideoDisplay =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_fCaptureInitialized =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_fDriverSuppliesPalettes =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_hVideoIn =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_hVideoOut =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_hVideoExtIn =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_hVideoExtOut =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        Self {
            wDeviceIndex: f_wDeviceIndex,
            fHasOverlay: f_fHasOverlay,
            fHasDlgVideoSource: f_fHasDlgVideoSource,
            fHasDlgVideoFormat: f_fHasDlgVideoFormat,
            fHasDlgVideoDisplay: f_fHasDlgVideoDisplay,
            fCaptureInitialized: f_fCaptureInitialized,
            fDriverSuppliesPalettes: f_fDriverSuppliesPalettes,
            hVideoIn: f_hVideoIn,
            hVideoOut: f_hVideoOut,
            hVideoExtIn: f_hVideoExtIn,
            hVideoExtOut: f_hVideoExtOut,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 44);
        FromIntoMemory::into_bytes(self.wDeviceIndex, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.fHasOverlay, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.fHasDlgVideoSource, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.fHasDlgVideoFormat, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.fHasDlgVideoDisplay, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.fCaptureInitialized, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.fDriverSuppliesPalettes, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.hVideoIn, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.hVideoOut, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.hVideoExtIn, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.hVideoExtOut, &mut into[40..40 + 4]);
    }
    fn size() -> usize {
        44
    }
}
pub type CAPERRORCALLBACKA =
    StdCallFnPtr<(super::super::Foundation::HWND, i32, PCSTR), super::super::Foundation::LRESULT>;
pub type CAPERRORCALLBACKW =
    StdCallFnPtr<(super::super::Foundation::HWND, i32, PCWSTR), super::super::Foundation::LRESULT>;
pub struct CAPINFOCHUNK {
    pub fccInfoID: u32,
    pub lpData: MutPtr<::core::ffi::c_void>,
    pub cbData: i32,
}
impl ::core::marker::Copy for CAPINFOCHUNK {}
impl ::core::clone::Clone for CAPINFOCHUNK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CAPINFOCHUNK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAPINFOCHUNK")
            .field("fccInfoID", &self.fccInfoID)
            .field("lpData", &self.lpData)
            .field("cbData", &self.cbData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CAPINFOCHUNK {
    fn eq(&self, other: &Self) -> bool {
        self.fccInfoID == other.fccInfoID
            && self.lpData == other.lpData
            && self.cbData == other.cbData
    }
}
impl ::core::cmp::Eq for CAPINFOCHUNK {}
impl FromIntoMemory for CAPINFOCHUNK {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_fccInfoID = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpData = <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_cbData = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            fccInfoID: f_fccInfoID,
            lpData: f_lpData,
            cbData: f_cbData,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.fccInfoID, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpData, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.cbData, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct CAPSTATUS {
    pub uiImageWidth: u32,
    pub uiImageHeight: u32,
    pub fLiveWindow: super::super::Foundation::BOOL,
    pub fOverlayWindow: super::super::Foundation::BOOL,
    pub fScale: super::super::Foundation::BOOL,
    pub ptScroll: super::super::Foundation::POINT,
    pub fUsingDefaultPalette: super::super::Foundation::BOOL,
    pub fAudioHardware: super::super::Foundation::BOOL,
    pub fCapFileExists: super::super::Foundation::BOOL,
    pub dwCurrentVideoFrame: u32,
    pub dwCurrentVideoFramesDropped: u32,
    pub dwCurrentWaveSamples: u32,
    pub dwCurrentTimeElapsedMS: u32,
    pub hPalCurrent: super::super::Graphics::Gdi::HPALETTE,
    pub fCapturingNow: super::super::Foundation::BOOL,
    pub dwReturn: u32,
    pub wNumVideoAllocated: u32,
    pub wNumAudioAllocated: u32,
}
impl ::core::marker::Copy for CAPSTATUS {}
impl ::core::clone::Clone for CAPSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CAPSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAPSTATUS")
            .field("uiImageWidth", &self.uiImageWidth)
            .field("uiImageHeight", &self.uiImageHeight)
            .field("fLiveWindow", &self.fLiveWindow)
            .field("fOverlayWindow", &self.fOverlayWindow)
            .field("fScale", &self.fScale)
            .field("ptScroll", &self.ptScroll)
            .field("fUsingDefaultPalette", &self.fUsingDefaultPalette)
            .field("fAudioHardware", &self.fAudioHardware)
            .field("fCapFileExists", &self.fCapFileExists)
            .field("dwCurrentVideoFrame", &self.dwCurrentVideoFrame)
            .field(
                "dwCurrentVideoFramesDropped",
                &self.dwCurrentVideoFramesDropped,
            )
            .field("dwCurrentWaveSamples", &self.dwCurrentWaveSamples)
            .field("dwCurrentTimeElapsedMS", &self.dwCurrentTimeElapsedMS)
            .field("hPalCurrent", &self.hPalCurrent)
            .field("fCapturingNow", &self.fCapturingNow)
            .field("dwReturn", &self.dwReturn)
            .field("wNumVideoAllocated", &self.wNumVideoAllocated)
            .field("wNumAudioAllocated", &self.wNumAudioAllocated)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CAPSTATUS {
    fn eq(&self, other: &Self) -> bool {
        self.uiImageWidth == other.uiImageWidth
            && self.uiImageHeight == other.uiImageHeight
            && self.fLiveWindow == other.fLiveWindow
            && self.fOverlayWindow == other.fOverlayWindow
            && self.fScale == other.fScale
            && self.ptScroll == other.ptScroll
            && self.fUsingDefaultPalette == other.fUsingDefaultPalette
            && self.fAudioHardware == other.fAudioHardware
            && self.fCapFileExists == other.fCapFileExists
            && self.dwCurrentVideoFrame == other.dwCurrentVideoFrame
            && self.dwCurrentVideoFramesDropped == other.dwCurrentVideoFramesDropped
            && self.dwCurrentWaveSamples == other.dwCurrentWaveSamples
            && self.dwCurrentTimeElapsedMS == other.dwCurrentTimeElapsedMS
            && self.hPalCurrent == other.hPalCurrent
            && self.fCapturingNow == other.fCapturingNow
            && self.dwReturn == other.dwReturn
            && self.wNumVideoAllocated == other.wNumVideoAllocated
            && self.wNumAudioAllocated == other.wNumAudioAllocated
    }
}
impl ::core::cmp::Eq for CAPSTATUS {}
impl FromIntoMemory for CAPSTATUS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 76);
        let f_uiImageWidth = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_uiImageHeight = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_fLiveWindow =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_fOverlayWindow =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_fScale =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_ptScroll =
            <super::super::Foundation::POINT as FromIntoMemory>::from_bytes(&from[20..20 + 8]);
        let f_fUsingDefaultPalette =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_fAudioHardware =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_fCapFileExists =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_dwCurrentVideoFrame = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_dwCurrentVideoFramesDropped = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_dwCurrentWaveSamples = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_dwCurrentTimeElapsedMS = <u32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_hPalCurrent = <super::super::Graphics::Gdi::HPALETTE as FromIntoMemory>::from_bytes(
            &from[56..56 + 4],
        );
        let f_fCapturingNow =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_dwReturn = <u32 as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        let f_wNumVideoAllocated = <u32 as FromIntoMemory>::from_bytes(&from[68..68 + 4]);
        let f_wNumAudioAllocated = <u32 as FromIntoMemory>::from_bytes(&from[72..72 + 4]);
        Self {
            uiImageWidth: f_uiImageWidth,
            uiImageHeight: f_uiImageHeight,
            fLiveWindow: f_fLiveWindow,
            fOverlayWindow: f_fOverlayWindow,
            fScale: f_fScale,
            ptScroll: f_ptScroll,
            fUsingDefaultPalette: f_fUsingDefaultPalette,
            fAudioHardware: f_fAudioHardware,
            fCapFileExists: f_fCapFileExists,
            dwCurrentVideoFrame: f_dwCurrentVideoFrame,
            dwCurrentVideoFramesDropped: f_dwCurrentVideoFramesDropped,
            dwCurrentWaveSamples: f_dwCurrentWaveSamples,
            dwCurrentTimeElapsedMS: f_dwCurrentTimeElapsedMS,
            hPalCurrent: f_hPalCurrent,
            fCapturingNow: f_fCapturingNow,
            dwReturn: f_dwReturn,
            wNumVideoAllocated: f_wNumVideoAllocated,
            wNumAudioAllocated: f_wNumAudioAllocated,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 76);
        FromIntoMemory::into_bytes(self.uiImageWidth, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.uiImageHeight, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.fLiveWindow, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.fOverlayWindow, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.fScale, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.ptScroll, &mut into[20..20 + 8]);
        FromIntoMemory::into_bytes(self.fUsingDefaultPalette, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.fAudioHardware, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.fCapFileExists, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.dwCurrentVideoFrame, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.dwCurrentVideoFramesDropped, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.dwCurrentWaveSamples, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.dwCurrentTimeElapsedMS, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.hPalCurrent, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.fCapturingNow, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.dwReturn, &mut into[64..64 + 4]);
        FromIntoMemory::into_bytes(self.wNumVideoAllocated, &mut into[68..68 + 4]);
        FromIntoMemory::into_bytes(self.wNumAudioAllocated, &mut into[72..72 + 4]);
    }
    fn size() -> usize {
        76
    }
}
pub type CAPSTATUSCALLBACKA =
    StdCallFnPtr<(super::super::Foundation::HWND, i32, PCSTR), super::super::Foundation::LRESULT>;
pub type CAPSTATUSCALLBACKW =
    StdCallFnPtr<(super::super::Foundation::HWND, i32, PCWSTR), super::super::Foundation::LRESULT>;
pub struct CAPTUREPARMS {
    pub dwRequestMicroSecPerFrame: u32,
    pub fMakeUserHitOKToCapture: super::super::Foundation::BOOL,
    pub wPercentDropForError: u32,
    pub fYield: super::super::Foundation::BOOL,
    pub dwIndexSize: u32,
    pub wChunkGranularity: u32,
    pub fUsingDOSMemory: super::super::Foundation::BOOL,
    pub wNumVideoRequested: u32,
    pub fCaptureAudio: super::super::Foundation::BOOL,
    pub wNumAudioRequested: u32,
    pub vKeyAbort: u32,
    pub fAbortLeftMouse: super::super::Foundation::BOOL,
    pub fAbortRightMouse: super::super::Foundation::BOOL,
    pub fLimitEnabled: super::super::Foundation::BOOL,
    pub wTimeLimit: u32,
    pub fMCIControl: super::super::Foundation::BOOL,
    pub fStepMCIDevice: super::super::Foundation::BOOL,
    pub dwMCIStartTime: u32,
    pub dwMCIStopTime: u32,
    pub fStepCaptureAt2x: super::super::Foundation::BOOL,
    pub wStepCaptureAverageFrames: u32,
    pub dwAudioBufferSize: u32,
    pub fDisableWriteCache: super::super::Foundation::BOOL,
    pub AVStreamMaster: u32,
}
impl ::core::marker::Copy for CAPTUREPARMS {}
impl ::core::clone::Clone for CAPTUREPARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CAPTUREPARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAPTUREPARMS")
            .field("dwRequestMicroSecPerFrame", &self.dwRequestMicroSecPerFrame)
            .field("fMakeUserHitOKToCapture", &self.fMakeUserHitOKToCapture)
            .field("wPercentDropForError", &self.wPercentDropForError)
            .field("fYield", &self.fYield)
            .field("dwIndexSize", &self.dwIndexSize)
            .field("wChunkGranularity", &self.wChunkGranularity)
            .field("fUsingDOSMemory", &self.fUsingDOSMemory)
            .field("wNumVideoRequested", &self.wNumVideoRequested)
            .field("fCaptureAudio", &self.fCaptureAudio)
            .field("wNumAudioRequested", &self.wNumAudioRequested)
            .field("vKeyAbort", &self.vKeyAbort)
            .field("fAbortLeftMouse", &self.fAbortLeftMouse)
            .field("fAbortRightMouse", &self.fAbortRightMouse)
            .field("fLimitEnabled", &self.fLimitEnabled)
            .field("wTimeLimit", &self.wTimeLimit)
            .field("fMCIControl", &self.fMCIControl)
            .field("fStepMCIDevice", &self.fStepMCIDevice)
            .field("dwMCIStartTime", &self.dwMCIStartTime)
            .field("dwMCIStopTime", &self.dwMCIStopTime)
            .field("fStepCaptureAt2x", &self.fStepCaptureAt2x)
            .field("wStepCaptureAverageFrames", &self.wStepCaptureAverageFrames)
            .field("dwAudioBufferSize", &self.dwAudioBufferSize)
            .field("fDisableWriteCache", &self.fDisableWriteCache)
            .field("AVStreamMaster", &self.AVStreamMaster)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CAPTUREPARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwRequestMicroSecPerFrame == other.dwRequestMicroSecPerFrame
            && self.fMakeUserHitOKToCapture == other.fMakeUserHitOKToCapture
            && self.wPercentDropForError == other.wPercentDropForError
            && self.fYield == other.fYield
            && self.dwIndexSize == other.dwIndexSize
            && self.wChunkGranularity == other.wChunkGranularity
            && self.fUsingDOSMemory == other.fUsingDOSMemory
            && self.wNumVideoRequested == other.wNumVideoRequested
            && self.fCaptureAudio == other.fCaptureAudio
            && self.wNumAudioRequested == other.wNumAudioRequested
            && self.vKeyAbort == other.vKeyAbort
            && self.fAbortLeftMouse == other.fAbortLeftMouse
            && self.fAbortRightMouse == other.fAbortRightMouse
            && self.fLimitEnabled == other.fLimitEnabled
            && self.wTimeLimit == other.wTimeLimit
            && self.fMCIControl == other.fMCIControl
            && self.fStepMCIDevice == other.fStepMCIDevice
            && self.dwMCIStartTime == other.dwMCIStartTime
            && self.dwMCIStopTime == other.dwMCIStopTime
            && self.fStepCaptureAt2x == other.fStepCaptureAt2x
            && self.wStepCaptureAverageFrames == other.wStepCaptureAverageFrames
            && self.dwAudioBufferSize == other.dwAudioBufferSize
            && self.fDisableWriteCache == other.fDisableWriteCache
            && self.AVStreamMaster == other.AVStreamMaster
    }
}
impl ::core::cmp::Eq for CAPTUREPARMS {}
impl FromIntoMemory for CAPTUREPARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 96);
        let f_dwRequestMicroSecPerFrame = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_fMakeUserHitOKToCapture =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_wPercentDropForError = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_fYield =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwIndexSize = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_wChunkGranularity = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_fUsingDOSMemory =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_wNumVideoRequested = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_fCaptureAudio =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_wNumAudioRequested = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_vKeyAbort = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_fAbortLeftMouse =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_fAbortRightMouse =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_fLimitEnabled =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_wTimeLimit = <u32 as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_fMCIControl =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_fStepMCIDevice =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        let f_dwMCIStartTime = <u32 as FromIntoMemory>::from_bytes(&from[68..68 + 4]);
        let f_dwMCIStopTime = <u32 as FromIntoMemory>::from_bytes(&from[72..72 + 4]);
        let f_fStepCaptureAt2x =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[76..76 + 4]);
        let f_wStepCaptureAverageFrames = <u32 as FromIntoMemory>::from_bytes(&from[80..80 + 4]);
        let f_dwAudioBufferSize = <u32 as FromIntoMemory>::from_bytes(&from[84..84 + 4]);
        let f_fDisableWriteCache =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[88..88 + 4]);
        let f_AVStreamMaster = <u32 as FromIntoMemory>::from_bytes(&from[92..92 + 4]);
        Self {
            dwRequestMicroSecPerFrame: f_dwRequestMicroSecPerFrame,
            fMakeUserHitOKToCapture: f_fMakeUserHitOKToCapture,
            wPercentDropForError: f_wPercentDropForError,
            fYield: f_fYield,
            dwIndexSize: f_dwIndexSize,
            wChunkGranularity: f_wChunkGranularity,
            fUsingDOSMemory: f_fUsingDOSMemory,
            wNumVideoRequested: f_wNumVideoRequested,
            fCaptureAudio: f_fCaptureAudio,
            wNumAudioRequested: f_wNumAudioRequested,
            vKeyAbort: f_vKeyAbort,
            fAbortLeftMouse: f_fAbortLeftMouse,
            fAbortRightMouse: f_fAbortRightMouse,
            fLimitEnabled: f_fLimitEnabled,
            wTimeLimit: f_wTimeLimit,
            fMCIControl: f_fMCIControl,
            fStepMCIDevice: f_fStepMCIDevice,
            dwMCIStartTime: f_dwMCIStartTime,
            dwMCIStopTime: f_dwMCIStopTime,
            fStepCaptureAt2x: f_fStepCaptureAt2x,
            wStepCaptureAverageFrames: f_wStepCaptureAverageFrames,
            dwAudioBufferSize: f_dwAudioBufferSize,
            fDisableWriteCache: f_fDisableWriteCache,
            AVStreamMaster: f_AVStreamMaster,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 96);
        FromIntoMemory::into_bytes(self.dwRequestMicroSecPerFrame, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.fMakeUserHitOKToCapture, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.wPercentDropForError, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.fYield, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwIndexSize, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.wChunkGranularity, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.fUsingDOSMemory, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.wNumVideoRequested, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.fCaptureAudio, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.wNumAudioRequested, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.vKeyAbort, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.fAbortLeftMouse, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.fAbortRightMouse, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.fLimitEnabled, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.wTimeLimit, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.fMCIControl, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.fStepMCIDevice, &mut into[64..64 + 4]);
        FromIntoMemory::into_bytes(self.dwMCIStartTime, &mut into[68..68 + 4]);
        FromIntoMemory::into_bytes(self.dwMCIStopTime, &mut into[72..72 + 4]);
        FromIntoMemory::into_bytes(self.fStepCaptureAt2x, &mut into[76..76 + 4]);
        FromIntoMemory::into_bytes(self.wStepCaptureAverageFrames, &mut into[80..80 + 4]);
        FromIntoMemory::into_bytes(self.dwAudioBufferSize, &mut into[84..84 + 4]);
        FromIntoMemory::into_bytes(self.fDisableWriteCache, &mut into[88..88 + 4]);
        FromIntoMemory::into_bytes(self.AVStreamMaster, &mut into[92..92 + 4]);
    }
    fn size() -> usize {
        96
    }
}
pub type CAPVIDEOCALLBACK = StdCallFnPtr<
    (super::super::Foundation::HWND, ConstPtr<VIDEOHDR>),
    super::super::Foundation::LRESULT,
>;
pub type CAPWAVECALLBACK = StdCallFnPtr<
    (
        super::super::Foundation::HWND,
        ConstPtr<super::Audio::WAVEHDR>,
    ),
    super::super::Foundation::LRESULT,
>;
pub type CAPYIELDCALLBACK =
    StdCallFnPtr<(super::super::Foundation::HWND,), super::super::Foundation::LRESULT>;
pub struct CHANNEL_CAPS {
    pub dwFlags: u32,
    pub dwSrcRectXMod: u32,
    pub dwSrcRectYMod: u32,
    pub dwSrcRectWidthMod: u32,
    pub dwSrcRectHeightMod: u32,
    pub dwDstRectXMod: u32,
    pub dwDstRectYMod: u32,
    pub dwDstRectWidthMod: u32,
    pub dwDstRectHeightMod: u32,
}
impl ::core::marker::Copy for CHANNEL_CAPS {}
impl ::core::clone::Clone for CHANNEL_CAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CHANNEL_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANNEL_CAPS")
            .field("dwFlags", &self.dwFlags)
            .field("dwSrcRectXMod", &self.dwSrcRectXMod)
            .field("dwSrcRectYMod", &self.dwSrcRectYMod)
            .field("dwSrcRectWidthMod", &self.dwSrcRectWidthMod)
            .field("dwSrcRectHeightMod", &self.dwSrcRectHeightMod)
            .field("dwDstRectXMod", &self.dwDstRectXMod)
            .field("dwDstRectYMod", &self.dwDstRectYMod)
            .field("dwDstRectWidthMod", &self.dwDstRectWidthMod)
            .field("dwDstRectHeightMod", &self.dwDstRectHeightMod)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CHANNEL_CAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags
            && self.dwSrcRectXMod == other.dwSrcRectXMod
            && self.dwSrcRectYMod == other.dwSrcRectYMod
            && self.dwSrcRectWidthMod == other.dwSrcRectWidthMod
            && self.dwSrcRectHeightMod == other.dwSrcRectHeightMod
            && self.dwDstRectXMod == other.dwDstRectXMod
            && self.dwDstRectYMod == other.dwDstRectYMod
            && self.dwDstRectWidthMod == other.dwDstRectWidthMod
            && self.dwDstRectHeightMod == other.dwDstRectHeightMod
    }
}
impl ::core::cmp::Eq for CHANNEL_CAPS {}
impl FromIntoMemory for CHANNEL_CAPS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 36);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwSrcRectXMod = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwSrcRectYMod = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwSrcRectWidthMod = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwSrcRectHeightMod = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwDstRectXMod = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwDstRectYMod = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_dwDstRectWidthMod = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_dwDstRectHeightMod = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        Self {
            dwFlags: f_dwFlags,
            dwSrcRectXMod: f_dwSrcRectXMod,
            dwSrcRectYMod: f_dwSrcRectYMod,
            dwSrcRectWidthMod: f_dwSrcRectWidthMod,
            dwSrcRectHeightMod: f_dwSrcRectHeightMod,
            dwDstRectXMod: f_dwDstRectXMod,
            dwDstRectYMod: f_dwDstRectYMod,
            dwDstRectWidthMod: f_dwDstRectWidthMod,
            dwDstRectHeightMod: f_dwDstRectHeightMod,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 36);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwSrcRectXMod, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwSrcRectYMod, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwSrcRectWidthMod, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwSrcRectHeightMod, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwDstRectXMod, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwDstRectYMod, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.dwDstRectWidthMod, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.dwDstRectHeightMod, &mut into[32..32 + 4]);
    }
    fn size() -> usize {
        36
    }
}
pub const CLSID_AVIFile: crate::core::GUID =
    crate::core::GUID::from_u128(0x00020000_0000_0000_c000_000000000046);
pub const CLSID_AVISimpleUnMarshal: crate::core::GUID =
    crate::core::GUID::from_u128(0x00020009_0000_0000_c000_000000000046);
pub struct COMPVARS {
    pub cbSize: i32,
    pub dwFlags: u32,
    pub hic: HIC,
    pub fccType: u32,
    pub fccHandler: u32,
    pub lpbiIn: MutPtr<super::super::Graphics::Gdi::BITMAPINFO>,
    pub lpbiOut: MutPtr<super::super::Graphics::Gdi::BITMAPINFO>,
    pub lpBitsOut: MutPtr<::core::ffi::c_void>,
    pub lpBitsPrev: MutPtr<::core::ffi::c_void>,
    pub lFrame: i32,
    pub lKey: i32,
    pub lDataRate: i32,
    pub lQ: i32,
    pub lKeyCount: i32,
    pub lpState: MutPtr<::core::ffi::c_void>,
    pub cbState: i32,
}
impl ::core::marker::Copy for COMPVARS {}
impl ::core::clone::Clone for COMPVARS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMPVARS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPVARS")
            .field("cbSize", &self.cbSize)
            .field("dwFlags", &self.dwFlags)
            .field("hic", &self.hic)
            .field("fccType", &self.fccType)
            .field("fccHandler", &self.fccHandler)
            .field("lpbiIn", &self.lpbiIn)
            .field("lpbiOut", &self.lpbiOut)
            .field("lpBitsOut", &self.lpBitsOut)
            .field("lpBitsPrev", &self.lpBitsPrev)
            .field("lFrame", &self.lFrame)
            .field("lKey", &self.lKey)
            .field("lDataRate", &self.lDataRate)
            .field("lQ", &self.lQ)
            .field("lKeyCount", &self.lKeyCount)
            .field("lpState", &self.lpState)
            .field("cbState", &self.cbState)
            .finish()
    }
}
impl ::core::cmp::PartialEq for COMPVARS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.dwFlags == other.dwFlags
            && self.hic == other.hic
            && self.fccType == other.fccType
            && self.fccHandler == other.fccHandler
            && self.lpbiIn == other.lpbiIn
            && self.lpbiOut == other.lpbiOut
            && self.lpBitsOut == other.lpBitsOut
            && self.lpBitsPrev == other.lpBitsPrev
            && self.lFrame == other.lFrame
            && self.lKey == other.lKey
            && self.lDataRate == other.lDataRate
            && self.lQ == other.lQ
            && self.lKeyCount == other.lKeyCount
            && self.lpState == other.lpState
            && self.cbState == other.cbState
    }
}
impl ::core::cmp::Eq for COMPVARS {}
impl FromIntoMemory for COMPVARS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 64);
        let f_cbSize = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_hic = <HIC as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_fccType = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_fccHandler = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_lpbiIn =
            <MutPtr<super::super::Graphics::Gdi::BITMAPINFO> as FromIntoMemory>::from_bytes(
                &from[20..20 + 4],
            );
        let f_lpbiOut =
            <MutPtr<super::super::Graphics::Gdi::BITMAPINFO> as FromIntoMemory>::from_bytes(
                &from[24..24 + 4],
            );
        let f_lpBitsOut =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_lpBitsPrev =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_lFrame = <i32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_lKey = <i32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_lDataRate = <i32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_lQ = <i32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_lKeyCount = <i32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_lpState =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_cbState = <i32 as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        Self {
            cbSize: f_cbSize,
            dwFlags: f_dwFlags,
            hic: f_hic,
            fccType: f_fccType,
            fccHandler: f_fccHandler,
            lpbiIn: f_lpbiIn,
            lpbiOut: f_lpbiOut,
            lpBitsOut: f_lpBitsOut,
            lpBitsPrev: f_lpBitsPrev,
            lFrame: f_lFrame,
            lKey: f_lKey,
            lDataRate: f_lDataRate,
            lQ: f_lQ,
            lKeyCount: f_lKeyCount,
            lpState: f_lpState,
            cbState: f_cbState,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 64);
        FromIntoMemory::into_bytes(self.cbSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.hic, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.fccType, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.fccHandler, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.lpbiIn, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.lpbiOut, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.lpBitsOut, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.lpBitsPrev, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.lFrame, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.lKey, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.lDataRate, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.lQ, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.lKeyCount, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.lpState, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.cbState, &mut into[60..60 + 4]);
    }
    fn size() -> usize {
        64
    }
}
pub struct CONTRESCR10WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
impl ::core::marker::Copy for CONTRESCR10WAVEFORMAT {}
impl ::core::clone::Clone for CONTRESCR10WAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONTRESCR10WAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONTRESCR10WAVEFORMAT")
            .field("wfx", &self.wfx)
            .field("wSamplesPerBlock", &self.wSamplesPerBlock)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CONTRESCR10WAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx && self.wSamplesPerBlock == other.wSamplesPerBlock
    }
}
impl ::core::cmp::Eq for CONTRESCR10WAVEFORMAT {}
impl FromIntoMemory for CONTRESCR10WAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        let f_wSamplesPerBlock = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        Self {
            wfx: f_wfx,
            wSamplesPerBlock: f_wSamplesPerBlock,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
        FromIntoMemory::into_bytes(self.wSamplesPerBlock, &mut into[18..18 + 2]);
    }
    fn size() -> usize {
        20
    }
}
pub struct CONTRESVQLPCWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
impl ::core::marker::Copy for CONTRESVQLPCWAVEFORMAT {}
impl ::core::clone::Clone for CONTRESVQLPCWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONTRESVQLPCWAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONTRESVQLPCWAVEFORMAT")
            .field("wfx", &self.wfx)
            .field("wSamplesPerBlock", &self.wSamplesPerBlock)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CONTRESVQLPCWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx && self.wSamplesPerBlock == other.wSamplesPerBlock
    }
}
impl ::core::cmp::Eq for CONTRESVQLPCWAVEFORMAT {}
impl FromIntoMemory for CONTRESVQLPCWAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        let f_wSamplesPerBlock = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        Self {
            wfx: f_wfx,
            wSamplesPerBlock: f_wSamplesPerBlock,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
        FromIntoMemory::into_bytes(self.wSamplesPerBlock, &mut into[18..18 + 2]);
    }
    fn size() -> usize {
        20
    }
}
pub const CONTROLCALLBACK_CAPTURING: u32 = 2u32;
pub const CONTROLCALLBACK_PREROLL: u32 = 1u32;
pub struct CREATIVEADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wRevision: u16,
}
impl ::core::marker::Copy for CREATIVEADPCMWAVEFORMAT {}
impl ::core::clone::Clone for CREATIVEADPCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CREATIVEADPCMWAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREATIVEADPCMWAVEFORMAT")
            .field("wfx", &self.wfx)
            .field("wRevision", &self.wRevision)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CREATIVEADPCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx && self.wRevision == other.wRevision
    }
}
impl ::core::cmp::Eq for CREATIVEADPCMWAVEFORMAT {}
impl FromIntoMemory for CREATIVEADPCMWAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        let f_wRevision = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        Self {
            wfx: f_wfx,
            wRevision: f_wRevision,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
        FromIntoMemory::into_bytes(self.wRevision, &mut into[18..18 + 2]);
    }
    fn size() -> usize {
        20
    }
}
pub struct CREATIVEFASTSPEECH10WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wRevision: u16,
}
impl ::core::marker::Copy for CREATIVEFASTSPEECH10WAVEFORMAT {}
impl ::core::clone::Clone for CREATIVEFASTSPEECH10WAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CREATIVEFASTSPEECH10WAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREATIVEFASTSPEECH10WAVEFORMAT")
            .field("wfx", &self.wfx)
            .field("wRevision", &self.wRevision)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CREATIVEFASTSPEECH10WAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx && self.wRevision == other.wRevision
    }
}
impl ::core::cmp::Eq for CREATIVEFASTSPEECH10WAVEFORMAT {}
impl FromIntoMemory for CREATIVEFASTSPEECH10WAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        let f_wRevision = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        Self {
            wfx: f_wfx,
            wRevision: f_wRevision,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
        FromIntoMemory::into_bytes(self.wRevision, &mut into[18..18 + 2]);
    }
    fn size() -> usize {
        20
    }
}
pub struct CREATIVEFASTSPEECH8WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wRevision: u16,
}
impl ::core::marker::Copy for CREATIVEFASTSPEECH8WAVEFORMAT {}
impl ::core::clone::Clone for CREATIVEFASTSPEECH8WAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CREATIVEFASTSPEECH8WAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREATIVEFASTSPEECH8WAVEFORMAT")
            .field("wfx", &self.wfx)
            .field("wRevision", &self.wRevision)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CREATIVEFASTSPEECH8WAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx && self.wRevision == other.wRevision
    }
}
impl ::core::cmp::Eq for CREATIVEFASTSPEECH8WAVEFORMAT {}
impl FromIntoMemory for CREATIVEFASTSPEECH8WAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        let f_wRevision = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        Self {
            wfx: f_wfx,
            wRevision: f_wRevision,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
        FromIntoMemory::into_bytes(self.wRevision, &mut into[18..18 + 2]);
    }
    fn size() -> usize {
        20
    }
}
pub const CRYSTAL_NET_SFM_CODEC: u32 = 1u32;
pub struct CSIMAADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
impl ::core::marker::Copy for CSIMAADPCMWAVEFORMAT {}
impl ::core::clone::Clone for CSIMAADPCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CSIMAADPCMWAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSIMAADPCMWAVEFORMAT")
            .field("wfx", &self.wfx)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CSIMAADPCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx
    }
}
impl ::core::cmp::Eq for CSIMAADPCMWAVEFORMAT {}
impl FromIntoMemory for CSIMAADPCMWAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 18);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        Self { wfx: f_wfx }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 18);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
    }
    fn size() -> usize {
        18
    }
}
pub const DCB_EVENT: u32 = 5u32;
pub const DCB_FUNCTION: u32 = 3u32;
pub const DCB_NOSWITCH: u32 = 8u32;
pub const DCB_NULL: u32 = 0u32;
pub const DCB_TASK: u32 = 2u32;
pub const DCB_TYPEMASK: u32 = 7u32;
pub const DCB_WINDOW: u32 = 1u32;
pub const DDF_0001: u32 = 1u32;
pub const DDF_2000: u32 = 8192u32;
pub const DDF_ANIMATE: u32 = 32u32;
pub const DDF_BACKGROUNDPAL: u32 = 512u32;
pub const DDF_BUFFER: u32 = 64u32;
pub const DDF_DONTDRAW: u32 = 16u32;
pub const DDF_FULLSCREEN: u32 = 256u32;
pub const DDF_HALFTONE: u32 = 4096u32;
pub const DDF_HURRYUP: u32 = 2048u32;
pub const DDF_JUSTDRAWIT: u32 = 128u32;
pub const DDF_NOTKEYFRAME: u32 = 1024u32;
pub const DDF_PREROLL: u32 = 16u32;
pub const DDF_SAME_DIB: u32 = 8u32;
pub const DDF_SAME_DRAW: u32 = 8u32;
pub const DDF_SAME_HDC: u32 = 4u32;
pub const DDF_SAME_SIZE: u32 = 8u32;
pub const DDF_UPDATE: u32 = 2u32;
pub struct DIALOGICOKIADPCMWAVEFORMAT {
    pub ewf: super::Audio::WAVEFORMATEX,
}
impl ::core::marker::Copy for DIALOGICOKIADPCMWAVEFORMAT {}
impl ::core::clone::Clone for DIALOGICOKIADPCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIALOGICOKIADPCMWAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIALOGICOKIADPCMWAVEFORMAT")
            .field("ewf", &self.ewf)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DIALOGICOKIADPCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.ewf == other.ewf
    }
}
impl ::core::cmp::Eq for DIALOGICOKIADPCMWAVEFORMAT {}
impl FromIntoMemory for DIALOGICOKIADPCMWAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 18);
        let f_ewf = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        Self { ewf: f_ewf }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 18);
        FromIntoMemory::into_bytes(self.ewf, &mut into[0..0 + 18]);
    }
    fn size() -> usize {
        18
    }
}
pub struct DIGIADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
impl ::core::marker::Copy for DIGIADPCMWAVEFORMAT {}
impl ::core::clone::Clone for DIGIADPCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIGIADPCMWAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIGIADPCMWAVEFORMAT")
            .field("wfx", &self.wfx)
            .field("wSamplesPerBlock", &self.wSamplesPerBlock)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DIGIADPCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx && self.wSamplesPerBlock == other.wSamplesPerBlock
    }
}
impl ::core::cmp::Eq for DIGIADPCMWAVEFORMAT {}
impl FromIntoMemory for DIGIADPCMWAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        let f_wSamplesPerBlock = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        Self {
            wfx: f_wfx,
            wSamplesPerBlock: f_wSamplesPerBlock,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
        FromIntoMemory::into_bytes(self.wSamplesPerBlock, &mut into[18..18 + 2]);
    }
    fn size() -> usize {
        20
    }
}
pub struct DIGIFIXWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
impl ::core::marker::Copy for DIGIFIXWAVEFORMAT {}
impl ::core::clone::Clone for DIGIFIXWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIGIFIXWAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIGIFIXWAVEFORMAT")
            .field("wfx", &self.wfx)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DIGIFIXWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx
    }
}
impl ::core::cmp::Eq for DIGIFIXWAVEFORMAT {}
impl FromIntoMemory for DIGIFIXWAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 18);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        Self { wfx: f_wfx }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 18);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
    }
    fn size() -> usize {
        18
    }
}
pub struct DIGIREALWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
impl ::core::marker::Copy for DIGIREALWAVEFORMAT {}
impl ::core::clone::Clone for DIGIREALWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIGIREALWAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIGIREALWAVEFORMAT")
            .field("wfx", &self.wfx)
            .field("wSamplesPerBlock", &self.wSamplesPerBlock)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DIGIREALWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx && self.wSamplesPerBlock == other.wSamplesPerBlock
    }
}
impl ::core::cmp::Eq for DIGIREALWAVEFORMAT {}
impl FromIntoMemory for DIGIREALWAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        let f_wSamplesPerBlock = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        Self {
            wfx: f_wfx,
            wSamplesPerBlock: f_wSamplesPerBlock,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
        FromIntoMemory::into_bytes(self.wSamplesPerBlock, &mut into[18..18 + 2]);
    }
    fn size() -> usize {
        20
    }
}
pub struct DIGISTDWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
impl ::core::marker::Copy for DIGISTDWAVEFORMAT {}
impl ::core::clone::Clone for DIGISTDWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIGISTDWAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIGISTDWAVEFORMAT")
            .field("wfx", &self.wfx)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DIGISTDWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx
    }
}
impl ::core::cmp::Eq for DIGISTDWAVEFORMAT {}
impl FromIntoMemory for DIGISTDWAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 18);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        Self { wfx: f_wfx }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 18);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
    }
    fn size() -> usize {
        18
    }
}
pub const DLG_ACMFILTERCHOOSE_ID: u32 = 71u32;
pub const DLG_ACMFORMATCHOOSE_ID: u32 = 70u32;
pub struct DOLBYAC2WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub nAuxBitsCode: u16,
}
impl ::core::marker::Copy for DOLBYAC2WAVEFORMAT {}
impl ::core::clone::Clone for DOLBYAC2WAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOLBYAC2WAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOLBYAC2WAVEFORMAT")
            .field("wfx", &self.wfx)
            .field("nAuxBitsCode", &self.nAuxBitsCode)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DOLBYAC2WAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx && self.nAuxBitsCode == other.nAuxBitsCode
    }
}
impl ::core::cmp::Eq for DOLBYAC2WAVEFORMAT {}
impl FromIntoMemory for DOLBYAC2WAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        let f_nAuxBitsCode = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        Self {
            wfx: f_wfx,
            nAuxBitsCode: f_nAuxBitsCode,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
        FromIntoMemory::into_bytes(self.nAuxBitsCode, &mut into[18..18 + 2]);
    }
    fn size() -> usize {
        20
    }
}
pub struct DRAWDIBTIME {
    pub timeCount: i32,
    pub timeDraw: i32,
    pub timeDecompress: i32,
    pub timeDither: i32,
    pub timeStretch: i32,
    pub timeBlt: i32,
    pub timeSetDIBits: i32,
}
impl ::core::marker::Copy for DRAWDIBTIME {}
impl ::core::clone::Clone for DRAWDIBTIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRAWDIBTIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRAWDIBTIME")
            .field("timeCount", &self.timeCount)
            .field("timeDraw", &self.timeDraw)
            .field("timeDecompress", &self.timeDecompress)
            .field("timeDither", &self.timeDither)
            .field("timeStretch", &self.timeStretch)
            .field("timeBlt", &self.timeBlt)
            .field("timeSetDIBits", &self.timeSetDIBits)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DRAWDIBTIME {
    fn eq(&self, other: &Self) -> bool {
        self.timeCount == other.timeCount
            && self.timeDraw == other.timeDraw
            && self.timeDecompress == other.timeDecompress
            && self.timeDither == other.timeDither
            && self.timeStretch == other.timeStretch
            && self.timeBlt == other.timeBlt
            && self.timeSetDIBits == other.timeSetDIBits
    }
}
impl ::core::cmp::Eq for DRAWDIBTIME {}
impl FromIntoMemory for DRAWDIBTIME {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_timeCount = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_timeDraw = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_timeDecompress = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_timeDither = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_timeStretch = <i32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_timeBlt = <i32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_timeSetDIBits = <i32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        Self {
            timeCount: f_timeCount,
            timeDraw: f_timeDraw,
            timeDecompress: f_timeDecompress,
            timeDither: f_timeDither,
            timeStretch: f_timeStretch,
            timeBlt: f_timeBlt,
            timeSetDIBits: f_timeSetDIBits,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.timeCount, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.timeDraw, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.timeDecompress, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.timeDither, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.timeStretch, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.timeBlt, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.timeSetDIBits, &mut into[24..24 + 4]);
    }
    fn size() -> usize {
        28
    }
}
pub type DRIVERMSGPROC = StdCallFnPtr<(u32, u32, PtrRepr, PtrRepr, PtrRepr), u32>;
pub type DRIVERPROC = StdCallFnPtr<
    (
        PtrRepr,
        HDRVR,
        u32,
        super::super::Foundation::LPARAM,
        super::super::Foundation::LPARAM,
    ),
    super::super::Foundation::LRESULT,
>;
pub const DRIVERS_SECTION: &'static str = "DRIVERS32";
pub struct DRMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wReserved: u16,
    pub ulContentId: u32,
    pub wfxSecure: super::Audio::WAVEFORMATEX,
}
impl ::core::marker::Copy for DRMWAVEFORMAT {}
impl ::core::clone::Clone for DRMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRMWAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRMWAVEFORMAT")
            .field("wfx", &self.wfx)
            .field("wReserved", &self.wReserved)
            .field("ulContentId", &self.ulContentId)
            .field("wfxSecure", &self.wfxSecure)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DRMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx
            && self.wReserved == other.wReserved
            && self.ulContentId == other.ulContentId
            && self.wfxSecure == other.wfxSecure
    }
}
impl ::core::cmp::Eq for DRMWAVEFORMAT {}
impl FromIntoMemory for DRMWAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 42);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        let f_wReserved = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        let f_ulContentId = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_wfxSecure =
            <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[24..24 + 18]);
        Self {
            wfx: f_wfx,
            wReserved: f_wReserved,
            ulContentId: f_ulContentId,
            wfxSecure: f_wfxSecure,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 42);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
        FromIntoMemory::into_bytes(self.wReserved, &mut into[18..18 + 2]);
        FromIntoMemory::into_bytes(self.ulContentId, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.wfxSecure, &mut into[24..24 + 18]);
    }
    fn size() -> usize {
        42
    }
}
pub const DRVCNF_CANCEL: u32 = 0u32;
pub const DRVCNF_OK: u32 = 1u32;
pub const DRVCNF_RESTART: u32 = 2u32;
pub struct DRVCONFIGINFO {
    pub dwDCISize: u32,
    pub lpszDCISectionName: PCWSTR,
    pub lpszDCIAliasName: PCWSTR,
}
impl ::core::marker::Copy for DRVCONFIGINFO {}
impl ::core::clone::Clone for DRVCONFIGINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRVCONFIGINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRVCONFIGINFO")
            .field("dwDCISize", &self.dwDCISize)
            .field("lpszDCISectionName", &self.lpszDCISectionName)
            .field("lpszDCIAliasName", &self.lpszDCIAliasName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DRVCONFIGINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwDCISize == other.dwDCISize
            && self.lpszDCISectionName == other.lpszDCISectionName
            && self.lpszDCIAliasName == other.lpszDCIAliasName
    }
}
impl ::core::cmp::Eq for DRVCONFIGINFO {}
impl FromIntoMemory for DRVCONFIGINFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_dwDCISize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpszDCISectionName = <PCWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpszDCIAliasName = <PCWSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            dwDCISize: f_dwDCISize,
            lpszDCISectionName: f_lpszDCISectionName,
            lpszDCIAliasName: f_lpszDCIAliasName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.dwDCISize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpszDCISectionName, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpszDCIAliasName, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct DRVCONFIGINFOEX {
    pub dwDCISize: u32,
    pub lpszDCISectionName: PCWSTR,
    pub lpszDCIAliasName: PCWSTR,
    pub dnDevNode: u32,
}
impl ::core::marker::Copy for DRVCONFIGINFOEX {}
impl ::core::clone::Clone for DRVCONFIGINFOEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRVCONFIGINFOEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRVCONFIGINFOEX")
            .field("dwDCISize", &self.dwDCISize)
            .field("lpszDCISectionName", &self.lpszDCISectionName)
            .field("lpszDCIAliasName", &self.lpszDCIAliasName)
            .field("dnDevNode", &self.dnDevNode)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DRVCONFIGINFOEX {
    fn eq(&self, other: &Self) -> bool {
        self.dwDCISize == other.dwDCISize
            && self.lpszDCISectionName == other.lpszDCISectionName
            && self.lpszDCIAliasName == other.lpszDCIAliasName
            && self.dnDevNode == other.dnDevNode
    }
}
impl ::core::cmp::Eq for DRVCONFIGINFOEX {}
impl FromIntoMemory for DRVCONFIGINFOEX {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dwDCISize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpszDCISectionName = <PCWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpszDCIAliasName = <PCWSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dnDevNode = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            dwDCISize: f_dwDCISize,
            lpszDCISectionName: f_lpszDCISectionName,
            lpszDCIAliasName: f_lpszDCIAliasName,
            dnDevNode: f_dnDevNode,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dwDCISize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpszDCISectionName, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpszDCIAliasName, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dnDevNode, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub const DRVM_ADD_THRU: u32 = 257u32;
pub const DRVM_DISABLE: u32 = 102u32;
pub const DRVM_ENABLE: u32 = 103u32;
pub const DRVM_EXIT: u32 = 101u32;
pub const DRVM_INIT: u32 = 100u32;
pub const DRVM_INIT_EX: u32 = 104u32;
pub const DRVM_IOCTL: u32 = 256u32;
pub const DRVM_IOCTL_CMD_SYSTEM: i32 = -2147483648i32;
pub const DRVM_IOCTL_CMD_USER: i32 = 0i32;
pub struct DRVM_IOCTL_DATA {
    pub dwSize: u32,
    pub dwCmd: u32,
}
impl ::core::marker::Copy for DRVM_IOCTL_DATA {}
impl ::core::clone::Clone for DRVM_IOCTL_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRVM_IOCTL_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRVM_IOCTL_DATA")
            .field("dwSize", &self.dwSize)
            .field("dwCmd", &self.dwCmd)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DRVM_IOCTL_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwCmd == other.dwCmd
    }
}
impl ::core::cmp::Eq for DRVM_IOCTL_DATA {}
impl FromIntoMemory for DRVM_IOCTL_DATA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwCmd = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            dwSize: f_dwSize,
            dwCmd: f_dwCmd,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwCmd, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub const DRVM_IOCTL_LAST: u32 = 261u32;
pub const DRVM_MAPPER_CONSOLEVOICECOM_GET: u32 = 8215u32;
pub const DRVM_MAPPER_PREFERRED_FLAGS_PREFERREDONLY: u32 = 1u32;
pub const DRVM_MAPPER_PREFERRED_GET: u32 = 8213u32;
pub const DRVM_MAPPER_RECONFIGURE: u32 = 8193u32;
pub const DRVM_REMOVE_THRU: u32 = 258u32;
pub const DRVM_USER: u32 = 16384u32;
pub const DRV_CANCEL: u32 = 0u32;
pub const DRV_CLOSE: u32 = 4u32;
pub const DRV_CONFIGURE: u32 = 7u32;
pub const DRV_DISABLE: u32 = 5u32;
pub const DRV_ENABLE: u32 = 2u32;
pub const DRV_EXITSESSION: u32 = 11u32;
pub const DRV_FREE: u32 = 6u32;
pub const DRV_INSTALL: u32 = 9u32;
pub const DRV_LOAD: u32 = 1u32;
pub const DRV_MCI_FIRST: u32 = 2048u32;
pub const DRV_MCI_LAST: u32 = 6143u32;
pub const DRV_OK: u32 = 1u32;
pub const DRV_OPEN: u32 = 3u32;
pub const DRV_PNPINSTALL: u32 = 2059u32;
pub const DRV_POWER: u32 = 15u32;
pub const DRV_QUERYCONFIGURE: u32 = 8u32;
pub const DRV_QUERYDEVICEINTERFACE: u32 = 2060u32;
pub const DRV_QUERYDEVICEINTERFACESIZE: u32 = 2061u32;
pub const DRV_QUERYDEVNODE: u32 = 2050u32;
pub const DRV_QUERYFUNCTIONINSTANCEID: u32 = 2065u32;
pub const DRV_QUERYFUNCTIONINSTANCEIDSIZE: u32 = 2066u32;
pub const DRV_QUERYIDFROMSTRINGID: u32 = 2064u32;
pub const DRV_QUERYMAPPABLE: u32 = 2053u32;
pub const DRV_QUERYMODULE: u32 = 2057u32;
pub const DRV_QUERYSTRINGID: u32 = 2062u32;
pub const DRV_QUERYSTRINGIDSIZE: u32 = 2063u32;
pub const DRV_REMOVE: u32 = 10u32;
pub const DRV_RESERVED: u32 = 2048u32;
pub const DRV_RESTART: u32 = 2u32;
pub const DRV_USER: u32 = 16384u32;
pub struct DVIADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
impl ::core::marker::Copy for DVIADPCMWAVEFORMAT {}
impl ::core::clone::Clone for DVIADPCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DVIADPCMWAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DVIADPCMWAVEFORMAT")
            .field("wfx", &self.wfx)
            .field("wSamplesPerBlock", &self.wSamplesPerBlock)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DVIADPCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx && self.wSamplesPerBlock == other.wSamplesPerBlock
    }
}
impl ::core::cmp::Eq for DVIADPCMWAVEFORMAT {}
impl FromIntoMemory for DVIADPCMWAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        let f_wSamplesPerBlock = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        Self {
            wfx: f_wfx,
            wSamplesPerBlock: f_wSamplesPerBlock,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
        FromIntoMemory::into_bytes(self.wSamplesPerBlock, &mut into[18..18 + 2]);
    }
    fn size() -> usize {
        20
    }
}
pub const DVM_CONFIGURE_END: u32 = 8191u32;
pub const DVM_CONFIGURE_START: u32 = 4096u32;
pub const DVM_DST_RECT: u32 = 4101u32;
pub const DVM_FORMAT: u32 = 4098u32;
pub const DVM_PALETTE: u32 = 4097u32;
pub const DVM_PALETTERGB555: u32 = 4099u32;
pub const DVM_SRC_RECT: u32 = 4100u32;
pub const DVM_USER: u32 = 16384u32;
pub const DV_ERR_13: u32 = 16u32;
pub const DV_ERR_ALLOCATED: u32 = 19u32;
pub const DV_ERR_BADDEVICEID: u32 = 20u32;
pub const DV_ERR_BADERRNUM: u32 = 22u32;
pub const DV_ERR_BADFORMAT: u32 = 2u32;
pub const DV_ERR_BADINSTALL: u32 = 8u32;
pub const DV_ERR_BASE: u32 = 1u32;
pub const DV_ERR_CONFIG1: u32 = 13u32;
pub const DV_ERR_CONFIG2: u32 = 14u32;
pub const DV_ERR_CREATEPALETTE: u32 = 9u32;
pub const DV_ERR_DMA_CONFLICT: u32 = 26u32;
pub const DV_ERR_FLAGS: u32 = 15u32;
pub const DV_ERR_INT_CONFLICT: u32 = 27u32;
pub const DV_ERR_INVALHANDLE: u32 = 21u32;
pub const DV_ERR_IO_CONFLICT: u32 = 25u32;
pub const DV_ERR_LASTERROR: u32 = 28u32;
pub const DV_ERR_MEM_CONFLICT: u32 = 24u32;
pub const DV_ERR_NOMEM: u32 = 18u32;
pub const DV_ERR_NONSPECIFIC: u32 = 1u32;
pub const DV_ERR_NOTDETECTED: u32 = 7u32;
pub const DV_ERR_NOTSUPPORTED: u32 = 17u32;
pub const DV_ERR_NO_BUFFERS: u32 = 23u32;
pub const DV_ERR_OK: u32 = 0u32;
pub const DV_ERR_PARAM1: u32 = 11u32;
pub const DV_ERR_PARAM2: u32 = 12u32;
pub const DV_ERR_PROTECT_ONLY: u32 = 28u32;
pub const DV_ERR_SIZEFIELD: u32 = 10u32;
pub const DV_ERR_STILLPLAYING: u32 = 3u32;
pub const DV_ERR_SYNC: u32 = 5u32;
pub const DV_ERR_TOOMANYCHANNELS: u32 = 6u32;
pub const DV_ERR_UNPREPARED: u32 = 4u32;
pub const DV_ERR_USER_MSG: u32 = 1001u32;
pub const DV_VM_CLOSE: u32 = 977u32;
pub const DV_VM_DATA: u32 = 978u32;
pub const DV_VM_ERROR: u32 = 979u32;
pub const DV_VM_OPEN: u32 = 976u32;
pub struct ECHOSC1WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
impl ::core::marker::Copy for ECHOSC1WAVEFORMAT {}
impl ::core::clone::Clone for ECHOSC1WAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ECHOSC1WAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ECHOSC1WAVEFORMAT")
            .field("wfx", &self.wfx)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ECHOSC1WAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx
    }
}
impl ::core::cmp::Eq for ECHOSC1WAVEFORMAT {}
impl FromIntoMemory for ECHOSC1WAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 18);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        Self { wfx: f_wfx }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 18);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
    }
    fn size() -> usize {
        18
    }
}
pub struct EXBMINFOHEADER {
    pub bmi: super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub biExtDataOffset: u32,
}
impl ::core::marker::Copy for EXBMINFOHEADER {}
impl ::core::clone::Clone for EXBMINFOHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EXBMINFOHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXBMINFOHEADER")
            .field("bmi", &self.bmi)
            .field("biExtDataOffset", &self.biExtDataOffset)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EXBMINFOHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.bmi == other.bmi && self.biExtDataOffset == other.biExtDataOffset
    }
}
impl ::core::cmp::Eq for EXBMINFOHEADER {}
impl FromIntoMemory for EXBMINFOHEADER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 44);
        let f_bmi = <super::super::Graphics::Gdi::BITMAPINFOHEADER as FromIntoMemory>::from_bytes(
            &from[0..0 + 40],
        );
        let f_biExtDataOffset = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        Self {
            bmi: f_bmi,
            biExtDataOffset: f_biExtDataOffset,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 44);
        FromIntoMemory::into_bytes(self.bmi, &mut into[0..0 + 40]);
        FromIntoMemory::into_bytes(self.biExtDataOffset, &mut into[40..40 + 4]);
    }
    fn size() -> usize {
        44
    }
}
pub const FACILITY_NS: u32 = 13u32;
pub const FACILITY_NS_WIN32: u32 = 7u32;
pub const FIND_ANY: i32 = 32i32;
pub const FIND_DIR: i32 = 15i32;
pub const FIND_FORMAT: i32 = 64i32;
pub const FIND_FROM_START: i32 = 8i32;
pub const FIND_INDEX: i32 = 16384i32;
pub const FIND_KEY: i32 = 16i32;
pub const FIND_LENGTH: i32 = 4096i32;
pub const FIND_NEXT: i32 = 1i32;
pub const FIND_OFFSET: i32 = 8192i32;
pub const FIND_POS: i32 = 0i32;
pub const FIND_PREV: i32 = 4i32;
pub const FIND_RET: i32 = 61440i32;
pub const FIND_SIZE: i32 = 12288i32;
pub const FIND_TYPE: i32 = 240i32;
pub struct FMTOWNS_SND_WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wRevision: u16,
}
impl ::core::marker::Copy for FMTOWNS_SND_WAVEFORMAT {}
impl ::core::clone::Clone for FMTOWNS_SND_WAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FMTOWNS_SND_WAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FMTOWNS_SND_WAVEFORMAT")
            .field("wfx", &self.wfx)
            .field("wRevision", &self.wRevision)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FMTOWNS_SND_WAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx && self.wRevision == other.wRevision
    }
}
impl ::core::cmp::Eq for FMTOWNS_SND_WAVEFORMAT {}
impl FromIntoMemory for FMTOWNS_SND_WAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        let f_wRevision = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        Self {
            wfx: f_wfx,
            wRevision: f_wRevision,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
        FromIntoMemory::into_bytes(self.wRevision, &mut into[18..18 + 2]);
    }
    fn size() -> usize {
        20
    }
}
pub struct G721_ADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub nAuxBlockSize: u16,
}
impl ::core::marker::Copy for G721_ADPCMWAVEFORMAT {}
impl ::core::clone::Clone for G721_ADPCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for G721_ADPCMWAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("G721_ADPCMWAVEFORMAT")
            .field("wfx", &self.wfx)
            .field("nAuxBlockSize", &self.nAuxBlockSize)
            .finish()
    }
}
impl ::core::cmp::PartialEq for G721_ADPCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx && self.nAuxBlockSize == other.nAuxBlockSize
    }
}
impl ::core::cmp::Eq for G721_ADPCMWAVEFORMAT {}
impl FromIntoMemory for G721_ADPCMWAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        let f_nAuxBlockSize = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        Self {
            wfx: f_wfx,
            nAuxBlockSize: f_nAuxBlockSize,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
        FromIntoMemory::into_bytes(self.nAuxBlockSize, &mut into[18..18 + 2]);
    }
    fn size() -> usize {
        20
    }
}
pub struct G723_ADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub cbExtraSize: u16,
    pub nAuxBlockSize: u16,
}
impl ::core::marker::Copy for G723_ADPCMWAVEFORMAT {}
impl ::core::clone::Clone for G723_ADPCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for G723_ADPCMWAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("G723_ADPCMWAVEFORMAT")
            .field("wfx", &self.wfx)
            .field("cbExtraSize", &self.cbExtraSize)
            .field("nAuxBlockSize", &self.nAuxBlockSize)
            .finish()
    }
}
impl ::core::cmp::PartialEq for G723_ADPCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx
            && self.cbExtraSize == other.cbExtraSize
            && self.nAuxBlockSize == other.nAuxBlockSize
    }
}
impl ::core::cmp::Eq for G723_ADPCMWAVEFORMAT {}
impl FromIntoMemory for G723_ADPCMWAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 22);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        let f_cbExtraSize = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        let f_nAuxBlockSize = <u16 as FromIntoMemory>::from_bytes(&from[20..20 + 2]);
        Self {
            wfx: f_wfx,
            cbExtraSize: f_cbExtraSize,
            nAuxBlockSize: f_nAuxBlockSize,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 22);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
        FromIntoMemory::into_bytes(self.cbExtraSize, &mut into[18..18 + 2]);
        FromIntoMemory::into_bytes(self.nAuxBlockSize, &mut into[20..20 + 2]);
    }
    fn size() -> usize {
        22
    }
}
pub struct GSM610WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
impl ::core::marker::Copy for GSM610WAVEFORMAT {}
impl ::core::clone::Clone for GSM610WAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GSM610WAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GSM610WAVEFORMAT")
            .field("wfx", &self.wfx)
            .field("wSamplesPerBlock", &self.wSamplesPerBlock)
            .finish()
    }
}
impl ::core::cmp::PartialEq for GSM610WAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx && self.wSamplesPerBlock == other.wSamplesPerBlock
    }
}
impl ::core::cmp::Eq for GSM610WAVEFORMAT {}
impl FromIntoMemory for GSM610WAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        let f_wSamplesPerBlock = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        Self {
            wfx: f_wfx,
            wSamplesPerBlock: f_wSamplesPerBlock,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
        FromIntoMemory::into_bytes(self.wSamplesPerBlock, &mut into[18..18 + 2]);
    }
    fn size() -> usize {
        20
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HDRVR(pub PtrDiffRepr);
impl HDRVR {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HDRVR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HDRVR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HDRVR {}
impl ::core::hash::Hash for HDRVR {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HDRVR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDRVR").field(&self.0).finish()
    }
}
impl FromIntoMemory for HDRVR {
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
pub struct HIC(pub PtrDiffRepr);
impl HIC {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HIC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HIC {}
impl ::core::hash::Hash for HIC {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HIC").field(&self.0).finish()
    }
}
impl FromIntoMemory for HIC {
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
pub struct HMMIO(pub PtrDiffRepr);
impl HMMIO {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HMMIO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HMMIO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HMMIO {}
impl ::core::hash::Hash for HMMIO {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HMMIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HMMIO").field(&self.0).finish()
    }
}
impl FromIntoMemory for HMMIO {
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
pub struct HVIDEO(pub PtrDiffRepr);
impl HVIDEO {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HVIDEO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HVIDEO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HVIDEO {}
impl ::core::hash::Hash for HVIDEO {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HVIDEO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HVIDEO").field(&self.0).finish()
    }
}
impl FromIntoMemory for HVIDEO {
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
pub struct IAVIEditStream(pub crate::core::IUnknown);
pub trait IAVIEditStream_Trait: crate::core::IUnknown_Trait {
    fn Cut(
        &self,
        pl_start: MutPtr<i32>,
        pl_length: MutPtr<i32>,
        pp_result: MutPtr<IAVIStream>,
    ) -> crate::core::HRESULT {
        todo!("Cut")
    }
    fn Copy(
        &self,
        pl_start: MutPtr<i32>,
        pl_length: MutPtr<i32>,
        pp_result: MutPtr<IAVIStream>,
    ) -> crate::core::HRESULT {
        todo!("Copy")
    }
    fn Paste(
        &self,
        pl_pos: MutPtr<i32>,
        pl_length: MutPtr<i32>,
        pstream: IAVIStream,
        l_start: i32,
        l_end: i32,
    ) -> crate::core::HRESULT {
        todo!("Paste")
    }
    fn Clone(&self, pp_result: MutPtr<IAVIStream>) -> crate::core::HRESULT {
        todo!("Clone")
    }
    fn SetInfo(&self, lp_info: ConstPtr<AVISTREAMINFOW>, cb_info: i32) -> crate::core::HRESULT {
        todo!("SetInfo")
    }
}
impl ::core::clone::Clone for IAVIEditStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAVIEditStream {}
impl ::core::cmp::PartialEq for IAVIEditStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAVIEditStream {}
impl ::core::fmt::Debug for IAVIEditStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAVIEditStream").field(&self.0).finish()
    }
}
impl FromIntoMemory for IAVIEditStream {
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
impl crate::core::ComInterface for IAVIEditStream {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00020024_0000_0000_c000_000000000046);
}
pub struct IAVIFile(pub crate::core::IUnknown);
pub trait IAVIFile_Trait: crate::core::IUnknown_Trait {
    fn Info(&self, pfi: MutPtr<AVIFILEINFOW>, l_size: i32) -> crate::core::HRESULT {
        todo!("Info")
    }
    fn GetStream(
        &self,
        pp_stream: MutPtr<IAVIStream>,
        fcc_type: u32,
        l_param: i32,
    ) -> crate::core::HRESULT {
        todo!("GetStream")
    }
    fn CreateStream(
        &self,
        pp_stream: MutPtr<IAVIStream>,
        psi: ConstPtr<AVISTREAMINFOW>,
    ) -> crate::core::HRESULT {
        todo!("CreateStream")
    }
    fn WriteData(
        &self,
        ckid: u32,
        lp_data: ConstPtr<::core::ffi::c_void>,
        cb_data: i32,
    ) -> crate::core::HRESULT {
        todo!("WriteData")
    }
    fn ReadData(
        &self,
        ckid: u32,
        lp_data: MutPtr<::core::ffi::c_void>,
        lpcb_data: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("ReadData")
    }
    fn EndRecord(&self) -> crate::core::HRESULT {
        todo!("EndRecord")
    }
    fn DeleteStream(&self, fcc_type: u32, l_param: i32) -> crate::core::HRESULT {
        todo!("DeleteStream")
    }
}
impl ::core::clone::Clone for IAVIFile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAVIFile {}
impl ::core::cmp::PartialEq for IAVIFile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAVIFile {}
impl ::core::fmt::Debug for IAVIFile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAVIFile").field(&self.0).finish()
    }
}
impl FromIntoMemory for IAVIFile {
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
impl crate::core::ComInterface for IAVIFile {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00020020_0000_0000_c000_000000000046);
}
pub struct IAVIPersistFile(pub crate::core::IUnknown);
pub trait IAVIPersistFile_Trait: super::super::System::Com::IPersistFile_Trait {
    fn Reserved1(&self) -> crate::core::HRESULT {
        todo!("Reserved1")
    }
}
impl ::core::clone::Clone for IAVIPersistFile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAVIPersistFile {}
impl ::core::cmp::PartialEq for IAVIPersistFile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAVIPersistFile {}
impl ::core::fmt::Debug for IAVIPersistFile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAVIPersistFile").field(&self.0).finish()
    }
}
impl FromIntoMemory for IAVIPersistFile {
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
impl crate::core::ComInterface for IAVIPersistFile {
    type Super = super::super::System::Com::IPersistFile;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00020025_0000_0000_c000_000000000046);
}
pub struct IAVIStream(pub crate::core::IUnknown);
pub trait IAVIStream_Trait: crate::core::IUnknown_Trait {
    fn Create(
        &self,
        l_param_1: super::super::Foundation::LPARAM,
        l_param_2: super::super::Foundation::LPARAM,
    ) -> crate::core::HRESULT {
        todo!("Create")
    }
    fn Info(&self, psi: MutPtr<AVISTREAMINFOW>, l_size: i32) -> crate::core::HRESULT {
        todo!("Info")
    }
    fn FindSample(&self, l_pos: i32, l_flags: i32) -> i32 {
        todo!("FindSample")
    }
    fn ReadFormat(
        &self,
        l_pos: i32,
        lp_format: MutPtr<::core::ffi::c_void>,
        lpcb_format: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("ReadFormat")
    }
    fn SetFormat(
        &self,
        l_pos: i32,
        lp_format: ConstPtr<::core::ffi::c_void>,
        cb_format: i32,
    ) -> crate::core::HRESULT {
        todo!("SetFormat")
    }
    fn Read(
        &self,
        l_start: i32,
        l_samples: i32,
        lp_buffer: MutPtr<::core::ffi::c_void>,
        cb_buffer: i32,
        pl_bytes: MutPtr<i32>,
        pl_samples: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("Read")
    }
    fn Write(
        &self,
        l_start: i32,
        l_samples: i32,
        lp_buffer: ConstPtr<::core::ffi::c_void>,
        cb_buffer: i32,
        dw_flags: u32,
        pl_samp_written: MutPtr<i32>,
        pl_bytes_written: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("Write")
    }
    fn Delete(&self, l_start: i32, l_samples: i32) -> crate::core::HRESULT {
        todo!("Delete")
    }
    fn ReadData(
        &self,
        fcc: u32,
        lp: MutPtr<::core::ffi::c_void>,
        lpcb: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("ReadData")
    }
    fn WriteData(
        &self,
        fcc: u32,
        lp: ConstPtr<::core::ffi::c_void>,
        cb: i32,
    ) -> crate::core::HRESULT {
        todo!("WriteData")
    }
    fn SetInfo(&self, lp_info: ConstPtr<AVISTREAMINFOW>, cb_info: i32) -> crate::core::HRESULT {
        todo!("SetInfo")
    }
}
impl ::core::clone::Clone for IAVIStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAVIStream {}
impl ::core::cmp::PartialEq for IAVIStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAVIStream {}
impl ::core::fmt::Debug for IAVIStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAVIStream").field(&self.0).finish()
    }
}
impl FromIntoMemory for IAVIStream {
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
impl crate::core::ComInterface for IAVIStream {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00020021_0000_0000_c000_000000000046);
}
pub struct IAVIStreaming(pub crate::core::IUnknown);
pub trait IAVIStreaming_Trait: crate::core::IUnknown_Trait {
    fn Begin(&self, l_start: i32, l_end: i32, l_rate: i32) -> crate::core::HRESULT {
        todo!("Begin")
    }
    fn End(&self) -> crate::core::HRESULT {
        todo!("End")
    }
}
impl ::core::clone::Clone for IAVIStreaming {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAVIStreaming {}
impl ::core::cmp::PartialEq for IAVIStreaming {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAVIStreaming {}
impl ::core::fmt::Debug for IAVIStreaming {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAVIStreaming").field(&self.0).finish()
    }
}
impl FromIntoMemory for IAVIStreaming {
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
impl crate::core::ComInterface for IAVIStreaming {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00020022_0000_0000_c000_000000000046);
}
pub struct ICCOMPRESS {
    pub dwFlags: u32,
    pub lpbiOutput: MutPtr<super::super::Graphics::Gdi::BITMAPINFOHEADER>,
    pub lpOutput: MutPtr<::core::ffi::c_void>,
    pub lpbiInput: MutPtr<super::super::Graphics::Gdi::BITMAPINFOHEADER>,
    pub lpInput: MutPtr<::core::ffi::c_void>,
    pub lpckid: MutPtr<u32>,
    pub lpdwFlags: MutPtr<u32>,
    pub lFrameNum: i32,
    pub dwFrameSize: u32,
    pub dwQuality: u32,
    pub lpbiPrev: MutPtr<super::super::Graphics::Gdi::BITMAPINFOHEADER>,
    pub lpPrev: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for ICCOMPRESS {}
impl ::core::clone::Clone for ICCOMPRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ICCOMPRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICCOMPRESS")
            .field("dwFlags", &self.dwFlags)
            .field("lpbiOutput", &self.lpbiOutput)
            .field("lpOutput", &self.lpOutput)
            .field("lpbiInput", &self.lpbiInput)
            .field("lpInput", &self.lpInput)
            .field("lpckid", &self.lpckid)
            .field("lpdwFlags", &self.lpdwFlags)
            .field("lFrameNum", &self.lFrameNum)
            .field("dwFrameSize", &self.dwFrameSize)
            .field("dwQuality", &self.dwQuality)
            .field("lpbiPrev", &self.lpbiPrev)
            .field("lpPrev", &self.lpPrev)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ICCOMPRESS {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags
            && self.lpbiOutput == other.lpbiOutput
            && self.lpOutput == other.lpOutput
            && self.lpbiInput == other.lpbiInput
            && self.lpInput == other.lpInput
            && self.lpckid == other.lpckid
            && self.lpdwFlags == other.lpdwFlags
            && self.lFrameNum == other.lFrameNum
            && self.dwFrameSize == other.dwFrameSize
            && self.dwQuality == other.dwQuality
            && self.lpbiPrev == other.lpbiPrev
            && self.lpPrev == other.lpPrev
    }
}
impl ::core::cmp::Eq for ICCOMPRESS {}
impl FromIntoMemory for ICCOMPRESS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpbiOutput =
            <MutPtr<super::super::Graphics::Gdi::BITMAPINFOHEADER> as FromIntoMemory>::from_bytes(
                &from[4..4 + 4],
            );
        let f_lpOutput =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpbiInput =
            <MutPtr<super::super::Graphics::Gdi::BITMAPINFOHEADER> as FromIntoMemory>::from_bytes(
                &from[12..12 + 4],
            );
        let f_lpInput =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_lpckid = <MutPtr<u32> as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_lpdwFlags = <MutPtr<u32> as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_lFrameNum = <i32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_dwFrameSize = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_dwQuality = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_lpbiPrev =
            <MutPtr<super::super::Graphics::Gdi::BITMAPINFOHEADER> as FromIntoMemory>::from_bytes(
                &from[40..40 + 4],
            );
        let f_lpPrev =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        Self {
            dwFlags: f_dwFlags,
            lpbiOutput: f_lpbiOutput,
            lpOutput: f_lpOutput,
            lpbiInput: f_lpbiInput,
            lpInput: f_lpInput,
            lpckid: f_lpckid,
            lpdwFlags: f_lpdwFlags,
            lFrameNum: f_lFrameNum,
            dwFrameSize: f_dwFrameSize,
            dwQuality: f_dwQuality,
            lpbiPrev: f_lpbiPrev,
            lpPrev: f_lpPrev,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpbiOutput, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpOutput, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpbiInput, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpInput, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.lpckid, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.lpdwFlags, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.lFrameNum, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.dwFrameSize, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.dwQuality, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.lpbiPrev, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.lpPrev, &mut into[44..44 + 4]);
    }
    fn size() -> usize {
        48
    }
}
pub struct ICCOMPRESSFRAMES {
    pub dwFlags: u32,
    pub lpbiOutput: MutPtr<super::super::Graphics::Gdi::BITMAPINFOHEADER>,
    pub lOutput: super::super::Foundation::LPARAM,
    pub lpbiInput: MutPtr<super::super::Graphics::Gdi::BITMAPINFOHEADER>,
    pub lInput: super::super::Foundation::LPARAM,
    pub lStartFrame: i32,
    pub lFrameCount: i32,
    pub lQuality: i32,
    pub lDataRate: i32,
    pub lKeyRate: i32,
    pub dwRate: u32,
    pub dwScale: u32,
    pub dwOverheadPerFrame: u32,
    pub dwReserved2: u32,
    pub GetData: PtrDiffRepr,
    pub PutData: PtrDiffRepr,
}
impl ::core::marker::Copy for ICCOMPRESSFRAMES {}
impl ::core::clone::Clone for ICCOMPRESSFRAMES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ICCOMPRESSFRAMES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICCOMPRESSFRAMES")
            .field("dwFlags", &self.dwFlags)
            .field("lpbiOutput", &self.lpbiOutput)
            .field("lOutput", &self.lOutput)
            .field("lpbiInput", &self.lpbiInput)
            .field("lInput", &self.lInput)
            .field("lStartFrame", &self.lStartFrame)
            .field("lFrameCount", &self.lFrameCount)
            .field("lQuality", &self.lQuality)
            .field("lDataRate", &self.lDataRate)
            .field("lKeyRate", &self.lKeyRate)
            .field("dwRate", &self.dwRate)
            .field("dwScale", &self.dwScale)
            .field("dwOverheadPerFrame", &self.dwOverheadPerFrame)
            .field("dwReserved2", &self.dwReserved2)
            .field("GetData", &self.GetData)
            .field("PutData", &self.PutData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ICCOMPRESSFRAMES {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags
            && self.lpbiOutput == other.lpbiOutput
            && self.lOutput == other.lOutput
            && self.lpbiInput == other.lpbiInput
            && self.lInput == other.lInput
            && self.lStartFrame == other.lStartFrame
            && self.lFrameCount == other.lFrameCount
            && self.lQuality == other.lQuality
            && self.lDataRate == other.lDataRate
            && self.lKeyRate == other.lKeyRate
            && self.dwRate == other.dwRate
            && self.dwScale == other.dwScale
            && self.dwOverheadPerFrame == other.dwOverheadPerFrame
            && self.dwReserved2 == other.dwReserved2
            && self.GetData == other.GetData
            && self.PutData == other.PutData
    }
}
impl ::core::cmp::Eq for ICCOMPRESSFRAMES {}
impl FromIntoMemory for ICCOMPRESSFRAMES {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 64);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpbiOutput =
            <MutPtr<super::super::Graphics::Gdi::BITMAPINFOHEADER> as FromIntoMemory>::from_bytes(
                &from[4..4 + 4],
            );
        let f_lOutput =
            <super::super::Foundation::LPARAM as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpbiInput =
            <MutPtr<super::super::Graphics::Gdi::BITMAPINFOHEADER> as FromIntoMemory>::from_bytes(
                &from[12..12 + 4],
            );
        let f_lInput =
            <super::super::Foundation::LPARAM as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_lStartFrame = <i32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_lFrameCount = <i32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_lQuality = <i32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_lDataRate = <i32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_lKeyRate = <i32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_dwRate = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_dwScale = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_dwOverheadPerFrame = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_dwReserved2 = <u32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_GetData = <PtrDiffRepr as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_PutData = <PtrDiffRepr as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        Self {
            dwFlags: f_dwFlags,
            lpbiOutput: f_lpbiOutput,
            lOutput: f_lOutput,
            lpbiInput: f_lpbiInput,
            lInput: f_lInput,
            lStartFrame: f_lStartFrame,
            lFrameCount: f_lFrameCount,
            lQuality: f_lQuality,
            lDataRate: f_lDataRate,
            lKeyRate: f_lKeyRate,
            dwRate: f_dwRate,
            dwScale: f_dwScale,
            dwOverheadPerFrame: f_dwOverheadPerFrame,
            dwReserved2: f_dwReserved2,
            GetData: f_GetData,
            PutData: f_PutData,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 64);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpbiOutput, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lOutput, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpbiInput, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lInput, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.lStartFrame, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.lFrameCount, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.lQuality, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.lDataRate, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.lKeyRate, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.dwRate, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.dwScale, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.dwOverheadPerFrame, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.dwReserved2, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.GetData, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.PutData, &mut into[60..60 + 4]);
    }
    fn size() -> usize {
        64
    }
}
pub const ICCOMPRESSFRAMES_PADDING: u32 = 1u32;
pub const ICCOMPRESS_KEYFRAME: i32 = 1i32;
pub struct ICDECOMPRESS {
    pub dwFlags: u32,
    pub lpbiInput: MutPtr<super::super::Graphics::Gdi::BITMAPINFOHEADER>,
    pub lpInput: MutPtr<::core::ffi::c_void>,
    pub lpbiOutput: MutPtr<super::super::Graphics::Gdi::BITMAPINFOHEADER>,
    pub lpOutput: MutPtr<::core::ffi::c_void>,
    pub ckid: u32,
}
impl ::core::marker::Copy for ICDECOMPRESS {}
impl ::core::clone::Clone for ICDECOMPRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ICDECOMPRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICDECOMPRESS")
            .field("dwFlags", &self.dwFlags)
            .field("lpbiInput", &self.lpbiInput)
            .field("lpInput", &self.lpInput)
            .field("lpbiOutput", &self.lpbiOutput)
            .field("lpOutput", &self.lpOutput)
            .field("ckid", &self.ckid)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ICDECOMPRESS {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags
            && self.lpbiInput == other.lpbiInput
            && self.lpInput == other.lpInput
            && self.lpbiOutput == other.lpbiOutput
            && self.lpOutput == other.lpOutput
            && self.ckid == other.ckid
    }
}
impl ::core::cmp::Eq for ICDECOMPRESS {}
impl FromIntoMemory for ICDECOMPRESS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpbiInput =
            <MutPtr<super::super::Graphics::Gdi::BITMAPINFOHEADER> as FromIntoMemory>::from_bytes(
                &from[4..4 + 4],
            );
        let f_lpInput =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpbiOutput =
            <MutPtr<super::super::Graphics::Gdi::BITMAPINFOHEADER> as FromIntoMemory>::from_bytes(
                &from[12..12 + 4],
            );
        let f_lpOutput =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_ckid = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            dwFlags: f_dwFlags,
            lpbiInput: f_lpbiInput,
            lpInput: f_lpInput,
            lpbiOutput: f_lpbiOutput,
            lpOutput: f_lpOutput,
            ckid: f_ckid,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpbiInput, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpInput, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpbiOutput, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpOutput, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.ckid, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct ICDECOMPRESSEX {
    pub dwFlags: u32,
    pub lpbiSrc: MutPtr<super::super::Graphics::Gdi::BITMAPINFOHEADER>,
    pub lpSrc: MutPtr<::core::ffi::c_void>,
    pub lpbiDst: MutPtr<super::super::Graphics::Gdi::BITMAPINFOHEADER>,
    pub lpDst: MutPtr<::core::ffi::c_void>,
    pub xDst: i32,
    pub yDst: i32,
    pub dxDst: i32,
    pub dyDst: i32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub dxSrc: i32,
    pub dySrc: i32,
}
impl ::core::marker::Copy for ICDECOMPRESSEX {}
impl ::core::clone::Clone for ICDECOMPRESSEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ICDECOMPRESSEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICDECOMPRESSEX")
            .field("dwFlags", &self.dwFlags)
            .field("lpbiSrc", &self.lpbiSrc)
            .field("lpSrc", &self.lpSrc)
            .field("lpbiDst", &self.lpbiDst)
            .field("lpDst", &self.lpDst)
            .field("xDst", &self.xDst)
            .field("yDst", &self.yDst)
            .field("dxDst", &self.dxDst)
            .field("dyDst", &self.dyDst)
            .field("xSrc", &self.xSrc)
            .field("ySrc", &self.ySrc)
            .field("dxSrc", &self.dxSrc)
            .field("dySrc", &self.dySrc)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ICDECOMPRESSEX {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags
            && self.lpbiSrc == other.lpbiSrc
            && self.lpSrc == other.lpSrc
            && self.lpbiDst == other.lpbiDst
            && self.lpDst == other.lpDst
            && self.xDst == other.xDst
            && self.yDst == other.yDst
            && self.dxDst == other.dxDst
            && self.dyDst == other.dyDst
            && self.xSrc == other.xSrc
            && self.ySrc == other.ySrc
            && self.dxSrc == other.dxSrc
            && self.dySrc == other.dySrc
    }
}
impl ::core::cmp::Eq for ICDECOMPRESSEX {}
impl FromIntoMemory for ICDECOMPRESSEX {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 52);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpbiSrc =
            <MutPtr<super::super::Graphics::Gdi::BITMAPINFOHEADER> as FromIntoMemory>::from_bytes(
                &from[4..4 + 4],
            );
        let f_lpSrc = <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpbiDst =
            <MutPtr<super::super::Graphics::Gdi::BITMAPINFOHEADER> as FromIntoMemory>::from_bytes(
                &from[12..12 + 4],
            );
        let f_lpDst =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_xDst = <i32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_yDst = <i32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_dxDst = <i32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_dyDst = <i32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_xSrc = <i32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_ySrc = <i32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_dxSrc = <i32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_dySrc = <i32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        Self {
            dwFlags: f_dwFlags,
            lpbiSrc: f_lpbiSrc,
            lpSrc: f_lpSrc,
            lpbiDst: f_lpbiDst,
            lpDst: f_lpDst,
            xDst: f_xDst,
            yDst: f_yDst,
            dxDst: f_dxDst,
            dyDst: f_dyDst,
            xSrc: f_xSrc,
            ySrc: f_ySrc,
            dxSrc: f_dxSrc,
            dySrc: f_dySrc,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 52);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpbiSrc, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpSrc, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpbiDst, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpDst, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.xDst, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.yDst, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.dxDst, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.dyDst, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.xSrc, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.ySrc, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.dxSrc, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.dySrc, &mut into[48..48 + 4]);
    }
    fn size() -> usize {
        52
    }
}
pub const ICDECOMPRESS_HURRYUP: i32 = -2147483648i32;
pub const ICDECOMPRESS_NOTKEYFRAME: i32 = 134217728i32;
pub const ICDECOMPRESS_NULLFRAME: i32 = 268435456i32;
pub const ICDECOMPRESS_PREROLL: i32 = 536870912i32;
pub const ICDECOMPRESS_UPDATE: i32 = 1073741824i32;
pub struct ICDRAW {
    pub dwFlags: u32,
    pub lpFormat: MutPtr<::core::ffi::c_void>,
    pub lpData: MutPtr<::core::ffi::c_void>,
    pub cbData: u32,
    pub lTime: i32,
}
impl ::core::marker::Copy for ICDRAW {}
impl ::core::clone::Clone for ICDRAW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ICDRAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICDRAW")
            .field("dwFlags", &self.dwFlags)
            .field("lpFormat", &self.lpFormat)
            .field("lpData", &self.lpData)
            .field("cbData", &self.cbData)
            .field("lTime", &self.lTime)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ICDRAW {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags
            && self.lpFormat == other.lpFormat
            && self.lpData == other.lpData
            && self.cbData == other.cbData
            && self.lTime == other.lTime
    }
}
impl ::core::cmp::Eq for ICDRAW {}
impl FromIntoMemory for ICDRAW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpFormat =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpData = <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_cbData = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lTime = <i32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            dwFlags: f_dwFlags,
            lpFormat: f_lpFormat,
            lpData: f_lpData,
            cbData: f_cbData,
            lTime: f_lTime,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpFormat, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpData, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.cbData, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lTime, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub struct ICDRAWBEGIN {
    pub dwFlags: u32,
    pub hpal: super::super::Graphics::Gdi::HPALETTE,
    pub hwnd: super::super::Foundation::HWND,
    pub hdc: super::super::Graphics::Gdi::HDC,
    pub xDst: i32,
    pub yDst: i32,
    pub dxDst: i32,
    pub dyDst: i32,
    pub lpbi: MutPtr<super::super::Graphics::Gdi::BITMAPINFOHEADER>,
    pub xSrc: i32,
    pub ySrc: i32,
    pub dxSrc: i32,
    pub dySrc: i32,
    pub dwRate: u32,
    pub dwScale: u32,
}
impl ::core::marker::Copy for ICDRAWBEGIN {}
impl ::core::clone::Clone for ICDRAWBEGIN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ICDRAWBEGIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICDRAWBEGIN")
            .field("dwFlags", &self.dwFlags)
            .field("hpal", &self.hpal)
            .field("hwnd", &self.hwnd)
            .field("hdc", &self.hdc)
            .field("xDst", &self.xDst)
            .field("yDst", &self.yDst)
            .field("dxDst", &self.dxDst)
            .field("dyDst", &self.dyDst)
            .field("lpbi", &self.lpbi)
            .field("xSrc", &self.xSrc)
            .field("ySrc", &self.ySrc)
            .field("dxSrc", &self.dxSrc)
            .field("dySrc", &self.dySrc)
            .field("dwRate", &self.dwRate)
            .field("dwScale", &self.dwScale)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ICDRAWBEGIN {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags
            && self.hpal == other.hpal
            && self.hwnd == other.hwnd
            && self.hdc == other.hdc
            && self.xDst == other.xDst
            && self.yDst == other.yDst
            && self.dxDst == other.dxDst
            && self.dyDst == other.dyDst
            && self.lpbi == other.lpbi
            && self.xSrc == other.xSrc
            && self.ySrc == other.ySrc
            && self.dxSrc == other.dxSrc
            && self.dySrc == other.dySrc
            && self.dwRate == other.dwRate
            && self.dwScale == other.dwScale
    }
}
impl ::core::cmp::Eq for ICDRAWBEGIN {}
impl FromIntoMemory for ICDRAWBEGIN {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 60);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_hpal =
            <super::super::Graphics::Gdi::HPALETTE as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_hwnd =
            <super::super::Foundation::HWND as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_hdc =
            <super::super::Graphics::Gdi::HDC as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_xDst = <i32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_yDst = <i32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dxDst = <i32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_dyDst = <i32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_lpbi =
            <MutPtr<super::super::Graphics::Gdi::BITMAPINFOHEADER> as FromIntoMemory>::from_bytes(
                &from[32..32 + 4],
            );
        let f_xSrc = <i32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_ySrc = <i32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_dxSrc = <i32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_dySrc = <i32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_dwRate = <u32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_dwScale = <u32 as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        Self {
            dwFlags: f_dwFlags,
            hpal: f_hpal,
            hwnd: f_hwnd,
            hdc: f_hdc,
            xDst: f_xDst,
            yDst: f_yDst,
            dxDst: f_dxDst,
            dyDst: f_dyDst,
            lpbi: f_lpbi,
            xSrc: f_xSrc,
            ySrc: f_ySrc,
            dxSrc: f_dxSrc,
            dySrc: f_dySrc,
            dwRate: f_dwRate,
            dwScale: f_dwScale,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 60);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.hpal, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.hwnd, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.hdc, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.xDst, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.yDst, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dxDst, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.dyDst, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.lpbi, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.xSrc, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.ySrc, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.dxSrc, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.dySrc, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.dwRate, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.dwScale, &mut into[56..56 + 4]);
    }
    fn size() -> usize {
        60
    }
}
pub struct ICDRAWSUGGEST {
    pub lpbiIn: MutPtr<super::super::Graphics::Gdi::BITMAPINFOHEADER>,
    pub lpbiSuggest: MutPtr<super::super::Graphics::Gdi::BITMAPINFOHEADER>,
    pub dxSrc: i32,
    pub dySrc: i32,
    pub dxDst: i32,
    pub dyDst: i32,
    pub hicDecompressor: HIC,
}
impl ::core::marker::Copy for ICDRAWSUGGEST {}
impl ::core::clone::Clone for ICDRAWSUGGEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ICDRAWSUGGEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICDRAWSUGGEST")
            .field("lpbiIn", &self.lpbiIn)
            .field("lpbiSuggest", &self.lpbiSuggest)
            .field("dxSrc", &self.dxSrc)
            .field("dySrc", &self.dySrc)
            .field("dxDst", &self.dxDst)
            .field("dyDst", &self.dyDst)
            .field("hicDecompressor", &self.hicDecompressor)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ICDRAWSUGGEST {
    fn eq(&self, other: &Self) -> bool {
        self.lpbiIn == other.lpbiIn
            && self.lpbiSuggest == other.lpbiSuggest
            && self.dxSrc == other.dxSrc
            && self.dySrc == other.dySrc
            && self.dxDst == other.dxDst
            && self.dyDst == other.dyDst
            && self.hicDecompressor == other.hicDecompressor
    }
}
impl ::core::cmp::Eq for ICDRAWSUGGEST {}
impl FromIntoMemory for ICDRAWSUGGEST {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_lpbiIn =
            <MutPtr<super::super::Graphics::Gdi::BITMAPINFOHEADER> as FromIntoMemory>::from_bytes(
                &from[0..0 + 4],
            );
        let f_lpbiSuggest =
            <MutPtr<super::super::Graphics::Gdi::BITMAPINFOHEADER> as FromIntoMemory>::from_bytes(
                &from[4..4 + 4],
            );
        let f_dxSrc = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dySrc = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dxDst = <i32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dyDst = <i32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_hicDecompressor = <HIC as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        Self {
            lpbiIn: f_lpbiIn,
            lpbiSuggest: f_lpbiSuggest,
            dxSrc: f_dxSrc,
            dySrc: f_dySrc,
            dxDst: f_dxDst,
            dyDst: f_dyDst,
            hicDecompressor: f_hicDecompressor,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.lpbiIn, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpbiSuggest, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dxSrc, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dySrc, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dxDst, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dyDst, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.hicDecompressor, &mut into[24..24 + 4]);
    }
    fn size() -> usize {
        28
    }
}
pub const ICDRAW_ANIMATE: i32 = 8i32;
pub const ICDRAW_BUFFER: i32 = 256i32;
pub const ICDRAW_CONTINUE: i32 = 16i32;
pub const ICDRAW_FULLSCREEN: i32 = 2i32;
pub const ICDRAW_HDC: i32 = 4i32;
pub const ICDRAW_HURRYUP: i32 = -2147483648i32;
pub const ICDRAW_MEMORYDC: i32 = 32i32;
pub const ICDRAW_NOTKEYFRAME: i32 = 134217728i32;
pub const ICDRAW_NULLFRAME: i32 = 268435456i32;
pub const ICDRAW_PREROLL: i32 = 536870912i32;
pub const ICDRAW_QUERY: i32 = 1i32;
pub const ICDRAW_RENDER: i32 = 128i32;
pub const ICDRAW_UPDATE: i32 = 1073741824i32;
pub const ICDRAW_UPDATING: i32 = 64i32;
pub const ICERR_ABORT: i32 = -10i32;
pub const ICERR_BADBITDEPTH: i32 = -200i32;
pub const ICERR_BADFLAGS: i32 = -5i32;
pub const ICERR_BADFORMAT: i32 = -2i32;
pub const ICERR_BADHANDLE: i32 = -8i32;
pub const ICERR_BADIMAGESIZE: i32 = -201i32;
pub const ICERR_BADPARAM: i32 = -6i32;
pub const ICERR_BADSIZE: i32 = -7i32;
pub const ICERR_CANTUPDATE: i32 = -9i32;
pub const ICERR_CUSTOM: i32 = -400i32;
pub const ICERR_DONTDRAW: i32 = 1i32;
pub const ICERR_ERROR: i32 = -100i32;
pub const ICERR_GOTOKEYFRAME: i32 = 3i32;
pub const ICERR_INTERNAL: i32 = -4i32;
pub const ICERR_MEMORY: i32 = -3i32;
pub const ICERR_NEWPALETTE: i32 = 2i32;
pub const ICERR_OK: i32 = 0i32;
pub const ICERR_STOPDRAWING: i32 = 4i32;
pub const ICERR_UNSUPPORTED: i32 = -1i32;
pub struct ICINFO {
    pub dwSize: u32,
    pub fccType: u32,
    pub fccHandler: u32,
    pub dwFlags: u32,
    pub dwVersion: u32,
    pub dwVersionICM: u32,
    pub szName: [u16; 16],
    pub szDescription: [u16; 128],
    pub szDriver: [u16; 128],
}
impl ::core::marker::Copy for ICINFO {}
impl ::core::clone::Clone for ICINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ICINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICINFO")
            .field("dwSize", &self.dwSize)
            .field("fccType", &self.fccType)
            .field("fccHandler", &self.fccHandler)
            .field("dwFlags", &self.dwFlags)
            .field("dwVersion", &self.dwVersion)
            .field("dwVersionICM", &self.dwVersionICM)
            .field("szName", &self.szName)
            .field("szDescription", &self.szDescription)
            .field("szDriver", &self.szDriver)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ICINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.fccType == other.fccType
            && self.fccHandler == other.fccHandler
            && self.dwFlags == other.dwFlags
            && self.dwVersion == other.dwVersion
            && self.dwVersionICM == other.dwVersionICM
            && self.szName == other.szName
            && self.szDescription == other.szDescription
            && self.szDriver == other.szDriver
    }
}
impl ::core::cmp::Eq for ICINFO {}
impl FromIntoMemory for ICINFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 296);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_fccType = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_fccHandler = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwVersion = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwVersionICM = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_szName = <[u16; 16] as FromIntoMemory>::from_bytes(&from[24..24 + 16]);
        let f_szDescription = <[u16; 128] as FromIntoMemory>::from_bytes(&from[40..40 + 128]);
        let f_szDriver = <[u16; 128] as FromIntoMemory>::from_bytes(&from[168..168 + 128]);
        Self {
            dwSize: f_dwSize,
            fccType: f_fccType,
            fccHandler: f_fccHandler,
            dwFlags: f_dwFlags,
            dwVersion: f_dwVersion,
            dwVersionICM: f_dwVersionICM,
            szName: f_szName,
            szDescription: f_szDescription,
            szDriver: f_szDriver,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 296);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.fccType, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.fccHandler, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwVersion, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwVersionICM, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.szName, &mut into[24..24 + 16]);
        FromIntoMemory::into_bytes(self.szDescription, &mut into[40..40 + 128]);
        FromIntoMemory::into_bytes(self.szDriver, &mut into[168..168 + 128]);
    }
    fn size() -> usize {
        296
    }
}
pub const ICINSTALL_DRIVER: u32 = 2u32;
pub const ICINSTALL_DRIVERW: u32 = 32770u32;
pub const ICINSTALL_FUNCTION: u32 = 1u32;
pub const ICINSTALL_HDRV: u32 = 4u32;
pub const ICINSTALL_UNICODE: u32 = 32768u32;
pub const ICMF_ABOUT_QUERY: u32 = 1u32;
pub const ICMF_CHOOSE_ALLCOMPRESSORS: u32 = 8u32;
pub const ICMF_CHOOSE_DATARATE: u32 = 2u32;
pub const ICMF_CHOOSE_KEYFRAME: u32 = 1u32;
pub const ICMF_CHOOSE_PREVIEW: u32 = 4u32;
pub const ICMF_COMPVARS_VALID: u32 = 1u32;
pub const ICMF_CONFIGURE_QUERY: u32 = 1u32;
pub const ICMODE_COMPRESS: u32 = 1u32;
pub const ICMODE_DECOMPRESS: u32 = 2u32;
pub const ICMODE_DRAW: u32 = 8u32;
pub const ICMODE_FASTCOMPRESS: u32 = 5u32;
pub const ICMODE_FASTDECOMPRESS: u32 = 3u32;
pub const ICMODE_INTERNALF_FUNCTION32: u32 = 32768u32;
pub const ICMODE_INTERNALF_MASK: u32 = 32768u32;
pub const ICMODE_QUERY: u32 = 4u32;
pub const ICM_ABOUT: u32 = 20491u32;
pub const ICM_COMPRESS: u32 = 16392u32;
pub const ICM_COMPRESS_BEGIN: u32 = 16391u32;
pub const ICM_COMPRESS_END: u32 = 16393u32;
pub const ICM_COMPRESS_FRAMES: u32 = 16455u32;
pub const ICM_COMPRESS_FRAMES_INFO: u32 = 16454u32;
pub const ICM_COMPRESS_GET_FORMAT: u32 = 16388u32;
pub const ICM_COMPRESS_GET_SIZE: u32 = 16389u32;
pub const ICM_COMPRESS_QUERY: u32 = 16390u32;
pub const ICM_CONFIGURE: u32 = 20490u32;
pub const ICM_DECOMPRESS: u32 = 16397u32;
pub const ICM_DECOMPRESSEX: u32 = 16446u32;
pub const ICM_DECOMPRESSEX_BEGIN: u32 = 16444u32;
pub const ICM_DECOMPRESSEX_END: u32 = 16447u32;
pub const ICM_DECOMPRESSEX_QUERY: u32 = 16445u32;
pub const ICM_DECOMPRESS_BEGIN: u32 = 16396u32;
pub const ICM_DECOMPRESS_END: u32 = 16398u32;
pub const ICM_DECOMPRESS_GET_FORMAT: u32 = 16394u32;
pub const ICM_DECOMPRESS_GET_PALETTE: u32 = 16414u32;
pub const ICM_DECOMPRESS_QUERY: u32 = 16395u32;
pub const ICM_DECOMPRESS_SET_PALETTE: u32 = 16413u32;
pub const ICM_DRAW: u32 = 16417u32;
pub const ICM_DRAW_BEGIN: u32 = 16399u32;
pub const ICM_DRAW_BITS: u32 = 16404u32;
pub const ICM_DRAW_CHANGEPALETTE: u32 = 16435u32;
pub const ICM_DRAW_END: u32 = 16405u32;
pub const ICM_DRAW_FLUSH: u32 = 16421u32;
pub const ICM_DRAW_GETTIME: u32 = 16416u32;
pub const ICM_DRAW_GET_PALETTE: u32 = 16400u32;
pub const ICM_DRAW_IDLE: u32 = 16436u32;
pub const ICM_DRAW_QUERY: u32 = 16415u32;
pub const ICM_DRAW_REALIZE: u32 = 16420u32;
pub const ICM_DRAW_RENDERBUFFER: u32 = 16422u32;
pub const ICM_DRAW_SETTIME: u32 = 16419u32;
pub const ICM_DRAW_START: u32 = 16402u32;
pub const ICM_DRAW_START_PLAY: u32 = 16423u32;
pub const ICM_DRAW_STOP: u32 = 16403u32;
pub const ICM_DRAW_STOP_PLAY: u32 = 16424u32;
pub const ICM_DRAW_SUGGESTFORMAT: u32 = 16434u32;
pub const ICM_DRAW_UPDATE: u32 = 16401u32;
pub const ICM_DRAW_WINDOW: u32 = 16418u32;
pub const ICM_ENUMFORMATS: u32 = 20501u32;
pub const ICM_GET: u32 = 20521u32;
pub const ICM_GETBUFFERSWANTED: u32 = 16425u32;
pub const ICM_GETDEFAULTKEYFRAMERATE: u32 = 16426u32;
pub const ICM_GETDEFAULTQUALITY: u32 = 20510u32;
pub const ICM_GETERRORTEXT: u32 = 20492u32;
pub const ICM_GETFORMATNAME: u32 = 20500u32;
pub const ICM_GETINFO: u32 = 20482u32;
pub const ICM_GETQUALITY: u32 = 20511u32;
pub const ICM_GETSTATE: u32 = 20480u32;
pub const ICM_RESERVED: u32 = 20480u32;
pub const ICM_RESERVED_HIGH: u32 = 24576u32;
pub const ICM_RESERVED_LOW: u32 = 20480u32;
pub const ICM_SET: u32 = 20520u32;
pub const ICM_SETQUALITY: u32 = 20512u32;
pub const ICM_SETSTATE: u32 = 20481u32;
pub const ICM_SET_STATUS_PROC: u32 = 16456u32;
pub const ICM_USER: u32 = 16384u32;
pub struct ICOPEN {
    pub dwSize: u32,
    pub fccType: u32,
    pub fccHandler: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub dwError: super::super::Foundation::LRESULT,
    pub pV1Reserved: MutPtr<::core::ffi::c_void>,
    pub pV2Reserved: MutPtr<::core::ffi::c_void>,
    pub dnDevNode: u32,
}
impl ::core::marker::Copy for ICOPEN {}
impl ::core::clone::Clone for ICOPEN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ICOPEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICOPEN")
            .field("dwSize", &self.dwSize)
            .field("fccType", &self.fccType)
            .field("fccHandler", &self.fccHandler)
            .field("dwVersion", &self.dwVersion)
            .field("dwFlags", &self.dwFlags)
            .field("dwError", &self.dwError)
            .field("pV1Reserved", &self.pV1Reserved)
            .field("pV2Reserved", &self.pV2Reserved)
            .field("dnDevNode", &self.dnDevNode)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ICOPEN {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.fccType == other.fccType
            && self.fccHandler == other.fccHandler
            && self.dwVersion == other.dwVersion
            && self.dwFlags == other.dwFlags
            && self.dwError == other.dwError
            && self.pV1Reserved == other.pV1Reserved
            && self.pV2Reserved == other.pV2Reserved
            && self.dnDevNode == other.dnDevNode
    }
}
impl ::core::cmp::Eq for ICOPEN {}
impl FromIntoMemory for ICOPEN {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 36);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_fccType = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_fccHandler = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwVersion = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwError =
            <super::super::Foundation::LRESULT as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_pV1Reserved =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_pV2Reserved =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_dnDevNode = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        Self {
            dwSize: f_dwSize,
            fccType: f_fccType,
            fccHandler: f_fccHandler,
            dwVersion: f_dwVersion,
            dwFlags: f_dwFlags,
            dwError: f_dwError,
            pV1Reserved: f_pV1Reserved,
            pV2Reserved: f_pV2Reserved,
            dnDevNode: f_dnDevNode,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 36);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.fccType, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.fccHandler, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwVersion, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwError, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.pV1Reserved, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.pV2Reserved, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.dnDevNode, &mut into[32..32 + 4]);
    }
    fn size() -> usize {
        36
    }
}
pub struct ICPALETTE {
    pub dwFlags: u32,
    pub iStart: i32,
    pub iLen: i32,
    pub lppe: MutPtr<super::super::Graphics::Gdi::PALETTEENTRY>,
}
impl ::core::marker::Copy for ICPALETTE {}
impl ::core::clone::Clone for ICPALETTE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ICPALETTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICPALETTE")
            .field("dwFlags", &self.dwFlags)
            .field("iStart", &self.iStart)
            .field("iLen", &self.iLen)
            .field("lppe", &self.lppe)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ICPALETTE {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags
            && self.iStart == other.iStart
            && self.iLen == other.iLen
            && self.lppe == other.lppe
    }
}
impl ::core::cmp::Eq for ICPALETTE {}
impl FromIntoMemory for ICPALETTE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_iStart = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_iLen = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lppe =
            <MutPtr<super::super::Graphics::Gdi::PALETTEENTRY> as FromIntoMemory>::from_bytes(
                &from[12..12 + 4],
            );
        Self {
            dwFlags: f_dwFlags,
            iStart: f_iStart,
            iLen: f_iLen,
            lppe: f_lppe,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.iStart, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.iLen, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lppe, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub const ICQUALITY_DEFAULT: i32 = -1i32;
pub const ICQUALITY_HIGH: u32 = 10000u32;
pub const ICQUALITY_LOW: u32 = 0u32;
pub struct ICSETSTATUSPROC {
    pub dwFlags: u32,
    pub lParam: super::super::Foundation::LPARAM,
    pub Status: PtrDiffRepr,
}
impl ::core::marker::Copy for ICSETSTATUSPROC {}
impl ::core::clone::Clone for ICSETSTATUSPROC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ICSETSTATUSPROC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICSETSTATUSPROC")
            .field("dwFlags", &self.dwFlags)
            .field("lParam", &self.lParam)
            .field("Status", &self.Status)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ICSETSTATUSPROC {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.lParam == other.lParam && self.Status == other.Status
    }
}
impl ::core::cmp::Eq for ICSETSTATUSPROC {}
impl FromIntoMemory for ICSETSTATUSPROC {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lParam =
            <super::super::Foundation::LPARAM as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Status = <PtrDiffRepr as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            dwFlags: f_dwFlags,
            lParam: f_lParam,
            Status: f_Status,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lParam, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Status, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub const ICSTATUS_END: u32 = 2u32;
pub const ICSTATUS_ERROR: u32 = 3u32;
pub const ICSTATUS_START: u32 = 0u32;
pub const ICSTATUS_STATUS: u32 = 1u32;
pub const ICSTATUS_YIELD: u32 = 4u32;
pub const ICVERSION: u32 = 260u32;
pub const IDD_ACMFILTERCHOOSE_BTN_DELNAME: u32 = 104u32;
pub const IDD_ACMFILTERCHOOSE_BTN_HELP: u32 = 9u32;
pub const IDD_ACMFILTERCHOOSE_BTN_SETNAME: u32 = 103u32;
pub const IDD_ACMFILTERCHOOSE_CMB_CUSTOM: u32 = 100u32;
pub const IDD_ACMFILTERCHOOSE_CMB_FILTER: u32 = 102u32;
pub const IDD_ACMFILTERCHOOSE_CMB_FILTERTAG: u32 = 101u32;
pub const IDD_ACMFORMATCHOOSE_BTN_DELNAME: u32 = 104u32;
pub const IDD_ACMFORMATCHOOSE_BTN_HELP: u32 = 9u32;
pub const IDD_ACMFORMATCHOOSE_BTN_SETNAME: u32 = 103u32;
pub const IDD_ACMFORMATCHOOSE_CMB_CUSTOM: u32 = 100u32;
pub const IDD_ACMFORMATCHOOSE_CMB_FORMAT: u32 = 102u32;
pub const IDD_ACMFORMATCHOOSE_CMB_FORMATTAG: u32 = 101u32;
pub const IDS_CAP_AUDIO_DROP_COMPERROR: u32 = 442u32;
pub const IDS_CAP_AUDIO_DROP_ERROR: u32 = 441u32;
pub const IDS_CAP_AVI_DRAWDIB_ERROR: u32 = 439u32;
pub const IDS_CAP_AVI_INIT_ERROR: u32 = 433u32;
pub const IDS_CAP_BEGIN: u32 = 300u32;
pub const IDS_CAP_CANTOPEN: u32 = 409u32;
pub const IDS_CAP_COMPRESSOR_ERROR: u32 = 440u32;
pub const IDS_CAP_DEFAVIEXT: u32 = 407u32;
pub const IDS_CAP_DEFPALEXT: u32 = 408u32;
pub const IDS_CAP_DRIVER_ERROR: u32 = 418u32;
pub const IDS_CAP_END: u32 = 301u32;
pub const IDS_CAP_ERRORDIBSAVE: u32 = 406u32;
pub const IDS_CAP_ERRORPALOPEN: u32 = 404u32;
pub const IDS_CAP_ERRORPALSAVE: u32 = 405u32;
pub const IDS_CAP_FILEEXISTS: u32 = 403u32;
pub const IDS_CAP_FILE_OPEN_ERROR: u32 = 429u32;
pub const IDS_CAP_FILE_WRITE_ERROR: u32 = 430u32;
pub const IDS_CAP_INFO: u32 = 401u32;
pub const IDS_CAP_MCI_CANT_STEP_ERROR: u32 = 437u32;
pub const IDS_CAP_MCI_CONTROL_ERROR: u32 = 436u32;
pub const IDS_CAP_NODISKSPACE: u32 = 415u32;
pub const IDS_CAP_NO_AUDIO_CAP_ERROR: u32 = 438u32;
pub const IDS_CAP_NO_FRAME_CAP_ERROR: u32 = 434u32;
pub const IDS_CAP_NO_PALETTE_WARN: u32 = 435u32;
pub const IDS_CAP_OUTOFMEM: u32 = 402u32;
pub const IDS_CAP_READONLYFILE: u32 = 413u32;
pub const IDS_CAP_RECORDING_ERROR: u32 = 431u32;
pub const IDS_CAP_RECORDING_ERROR2: u32 = 432u32;
pub const IDS_CAP_SAVEASPERCENT: u32 = 417u32;
pub const IDS_CAP_SEQ_MSGSTART: u32 = 410u32;
pub const IDS_CAP_SEQ_MSGSTOP: u32 = 411u32;
pub const IDS_CAP_SETFILESIZE: u32 = 416u32;
pub const IDS_CAP_STAT_CAP_AUDIO: u32 = 509u32;
pub const IDS_CAP_STAT_CAP_FINI: u32 = 503u32;
pub const IDS_CAP_STAT_CAP_INIT: u32 = 502u32;
pub const IDS_CAP_STAT_CAP_L_FRAMES: u32 = 508u32;
pub const IDS_CAP_STAT_FRAMESDROPPED: u32 = 513u32;
pub const IDS_CAP_STAT_I_FRAMES: u32 = 506u32;
pub const IDS_CAP_STAT_LIVE_MODE: u32 = 500u32;
pub const IDS_CAP_STAT_L_FRAMES: u32 = 507u32;
pub const IDS_CAP_STAT_OPTPAL_BUILD: u32 = 505u32;
pub const IDS_CAP_STAT_OVERLAY_MODE: u32 = 501u32;
pub const IDS_CAP_STAT_PALETTE_BUILD: u32 = 504u32;
pub const IDS_CAP_STAT_VIDEOAUDIO: u32 = 511u32;
pub const IDS_CAP_STAT_VIDEOCURRENT: u32 = 510u32;
pub const IDS_CAP_STAT_VIDEOONLY: u32 = 512u32;
pub const IDS_CAP_VIDEDITERR: u32 = 412u32;
pub const IDS_CAP_VIDEO_ADD_ERROR: u32 = 427u32;
pub const IDS_CAP_VIDEO_ALLOC_ERROR: u32 = 425u32;
pub const IDS_CAP_VIDEO_OPEN_ERROR: u32 = 424u32;
pub const IDS_CAP_VIDEO_PREPARE_ERROR: u32 = 426u32;
pub const IDS_CAP_VIDEO_SIZE_ERROR: u32 = 428u32;
pub const IDS_CAP_WAVE_ADD_ERROR: u32 = 422u32;
pub const IDS_CAP_WAVE_ALLOC_ERROR: u32 = 420u32;
pub const IDS_CAP_WAVE_OPEN_ERROR: u32 = 419u32;
pub const IDS_CAP_WAVE_PREPARE_ERROR: u32 = 421u32;
pub const IDS_CAP_WAVE_SIZE_ERROR: u32 = 423u32;
pub const IDS_CAP_WRITEERROR: u32 = 414u32;
pub struct IGetFrame(pub crate::core::IUnknown);
pub trait IGetFrame_Trait: crate::core::IUnknown_Trait {
    fn GetFrame(&self, l_pos: i32) -> MutPtr<::core::ffi::c_void> {
        todo!("GetFrame")
    }
    fn Begin(&self, l_start: i32, l_end: i32, l_rate: i32) -> crate::core::HRESULT {
        todo!("Begin")
    }
    fn End(&self) -> crate::core::HRESULT {
        todo!("End")
    }
    fn SetFormat(
        &self,
        lpbi: ConstPtr<super::super::Graphics::Gdi::BITMAPINFOHEADER>,
        lp_bits: ConstPtr<::core::ffi::c_void>,
        x: i32,
        y: i32,
        dx: i32,
        dy: i32,
    ) -> crate::core::HRESULT {
        todo!("SetFormat")
    }
}
impl ::core::clone::Clone for IGetFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IGetFrame {}
impl ::core::cmp::PartialEq for IGetFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetFrame {}
impl ::core::fmt::Debug for IGetFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetFrame").field(&self.0).finish()
    }
}
impl FromIntoMemory for IGetFrame {
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
impl crate::core::ComInterface for IGetFrame {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00020023_0000_0000_c000_000000000046);
}
pub struct IMAADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
impl ::core::marker::Copy for IMAADPCMWAVEFORMAT {}
impl ::core::clone::Clone for IMAADPCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAADPCMWAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAADPCMWAVEFORMAT")
            .field("wfx", &self.wfx)
            .field("wSamplesPerBlock", &self.wSamplesPerBlock)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IMAADPCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx && self.wSamplesPerBlock == other.wSamplesPerBlock
    }
}
impl ::core::cmp::Eq for IMAADPCMWAVEFORMAT {}
impl FromIntoMemory for IMAADPCMWAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        let f_wSamplesPerBlock = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        Self {
            wfx: f_wfx,
            wSamplesPerBlock: f_wSamplesPerBlock,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
        FromIntoMemory::into_bytes(self.wSamplesPerBlock, &mut into[18..18 + 2]);
    }
    fn size() -> usize {
        20
    }
}
pub const JDD_CONFIGCHANGED: u32 = 2307u32;
pub const JDD_GETDEVCAPS: u32 = 2050u32;
pub const JDD_GETNUMDEVS: u32 = 2049u32;
pub const JDD_GETPOS: u32 = 2305u32;
pub const JDD_GETPOSEX: u32 = 2308u32;
pub const JDD_SETCALIBRATION: u32 = 2306u32;
pub const JIFMK_00: u32 = 65280u32;
pub const JIFMK_APP0: u32 = 65504u32;
pub const JIFMK_APP1: u32 = 65505u32;
pub const JIFMK_APP2: u32 = 65506u32;
pub const JIFMK_APP3: u32 = 65507u32;
pub const JIFMK_APP4: u32 = 65508u32;
pub const JIFMK_APP5: u32 = 65509u32;
pub const JIFMK_APP6: u32 = 65510u32;
pub const JIFMK_APP7: u32 = 65511u32;
pub const JIFMK_COM: u32 = 65534u32;
pub const JIFMK_DAC: u32 = 65484u32;
pub const JIFMK_DHP: u32 = 65502u32;
pub const JIFMK_DHT: u32 = 65476u32;
pub const JIFMK_DNL: u32 = 65500u32;
pub const JIFMK_DQT: u32 = 65499u32;
pub const JIFMK_DRI: u32 = 65501u32;
pub const JIFMK_EOI: u32 = 65497u32;
pub const JIFMK_EXP: u32 = 65503u32;
pub const JIFMK_FF: u32 = 65535u32;
pub const JIFMK_JPG: u32 = 65480u32;
pub const JIFMK_JPG0: u32 = 65520u32;
pub const JIFMK_JPG1: u32 = 65521u32;
pub const JIFMK_JPG10: u32 = 65530u32;
pub const JIFMK_JPG11: u32 = 65531u32;
pub const JIFMK_JPG12: u32 = 65532u32;
pub const JIFMK_JPG13: u32 = 65533u32;
pub const JIFMK_JPG2: u32 = 65522u32;
pub const JIFMK_JPG3: u32 = 65523u32;
pub const JIFMK_JPG4: u32 = 65524u32;
pub const JIFMK_JPG5: u32 = 65525u32;
pub const JIFMK_JPG6: u32 = 65526u32;
pub const JIFMK_JPG7: u32 = 65527u32;
pub const JIFMK_JPG8: u32 = 65528u32;
pub const JIFMK_JPG9: u32 = 65529u32;
pub const JIFMK_RES: u32 = 65282u32;
pub const JIFMK_RST0: u32 = 65488u32;
pub const JIFMK_RST1: u32 = 65489u32;
pub const JIFMK_RST2: u32 = 65490u32;
pub const JIFMK_RST3: u32 = 65491u32;
pub const JIFMK_RST4: u32 = 65492u32;
pub const JIFMK_RST5: u32 = 65493u32;
pub const JIFMK_RST6: u32 = 65494u32;
pub const JIFMK_RST7: u32 = 65495u32;
pub const JIFMK_SOF0: u32 = 65472u32;
pub const JIFMK_SOF1: u32 = 65473u32;
pub const JIFMK_SOF10: u32 = 65482u32;
pub const JIFMK_SOF11: u32 = 65483u32;
pub const JIFMK_SOF13: u32 = 65485u32;
pub const JIFMK_SOF14: u32 = 65486u32;
pub const JIFMK_SOF15: u32 = 65487u32;
pub const JIFMK_SOF2: u32 = 65474u32;
pub const JIFMK_SOF3: u32 = 65475u32;
pub const JIFMK_SOF5: u32 = 65477u32;
pub const JIFMK_SOF6: u32 = 65478u32;
pub const JIFMK_SOF7: u32 = 65479u32;
pub const JIFMK_SOF9: u32 = 65481u32;
pub const JIFMK_SOI: u32 = 65496u32;
pub const JIFMK_SOS: u32 = 65498u32;
pub const JIFMK_TEM: u32 = 65281u32;
pub struct JOYCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub wXmin: u32,
    pub wXmax: u32,
    pub wYmin: u32,
    pub wYmax: u32,
    pub wZmin: u32,
    pub wZmax: u32,
    pub wNumButtons: u32,
    pub wPeriodMin: u32,
    pub wPeriodMax: u32,
    pub wRmin: u32,
    pub wRmax: u32,
    pub wUmin: u32,
    pub wUmax: u32,
    pub wVmin: u32,
    pub wVmax: u32,
    pub wCaps: u32,
    pub wMaxAxes: u32,
    pub wNumAxes: u32,
    pub wMaxButtons: u32,
    pub szRegKey: [super::super::Foundation::CHAR; 32],
    pub szOEMVxD: [super::super::Foundation::CHAR; 260],
    pub ManufacturerGuid: crate::core::GUID,
    pub ProductGuid: crate::core::GUID,
    pub NameGuid: crate::core::GUID,
}
impl ::core::marker::Copy for JOYCAPS2A {}
impl ::core::clone::Clone for JOYCAPS2A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JOYCAPS2A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOYCAPS2A")
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("szPname", &self.szPname)
            .field("wXmin", &self.wXmin)
            .field("wXmax", &self.wXmax)
            .field("wYmin", &self.wYmin)
            .field("wYmax", &self.wYmax)
            .field("wZmin", &self.wZmin)
            .field("wZmax", &self.wZmax)
            .field("wNumButtons", &self.wNumButtons)
            .field("wPeriodMin", &self.wPeriodMin)
            .field("wPeriodMax", &self.wPeriodMax)
            .field("wRmin", &self.wRmin)
            .field("wRmax", &self.wRmax)
            .field("wUmin", &self.wUmin)
            .field("wUmax", &self.wUmax)
            .field("wVmin", &self.wVmin)
            .field("wVmax", &self.wVmax)
            .field("wCaps", &self.wCaps)
            .field("wMaxAxes", &self.wMaxAxes)
            .field("wNumAxes", &self.wNumAxes)
            .field("wMaxButtons", &self.wMaxButtons)
            .field("szRegKey", &self.szRegKey)
            .field("szOEMVxD", &self.szOEMVxD)
            .field("ManufacturerGuid", &self.ManufacturerGuid)
            .field("ProductGuid", &self.ProductGuid)
            .field("NameGuid", &self.NameGuid)
            .finish()
    }
}
impl ::core::cmp::PartialEq for JOYCAPS2A {
    fn eq(&self, other: &Self) -> bool {
        self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.szPname == other.szPname
            && self.wXmin == other.wXmin
            && self.wXmax == other.wXmax
            && self.wYmin == other.wYmin
            && self.wYmax == other.wYmax
            && self.wZmin == other.wZmin
            && self.wZmax == other.wZmax
            && self.wNumButtons == other.wNumButtons
            && self.wPeriodMin == other.wPeriodMin
            && self.wPeriodMax == other.wPeriodMax
            && self.wRmin == other.wRmin
            && self.wRmax == other.wRmax
            && self.wUmin == other.wUmin
            && self.wUmax == other.wUmax
            && self.wVmin == other.wVmin
            && self.wVmax == other.wVmax
            && self.wCaps == other.wCaps
            && self.wMaxAxes == other.wMaxAxes
            && self.wNumAxes == other.wNumAxes
            && self.wMaxButtons == other.wMaxButtons
            && self.szRegKey == other.szRegKey
            && self.szOEMVxD == other.szOEMVxD
            && self.ManufacturerGuid == other.ManufacturerGuid
            && self.ProductGuid == other.ProductGuid
            && self.NameGuid == other.NameGuid
    }
}
impl ::core::cmp::Eq for JOYCAPS2A {}
impl FromIntoMemory for JOYCAPS2A {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 452);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_szPname =
            <[super::super::Foundation::CHAR; 32] as FromIntoMemory>::from_bytes(&from[4..4 + 32]);
        let f_wXmin = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_wXmax = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_wYmin = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_wYmax = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_wZmin = <u32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_wZmax = <u32 as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_wNumButtons = <u32 as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_wPeriodMin = <u32 as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        let f_wPeriodMax = <u32 as FromIntoMemory>::from_bytes(&from[68..68 + 4]);
        let f_wRmin = <u32 as FromIntoMemory>::from_bytes(&from[72..72 + 4]);
        let f_wRmax = <u32 as FromIntoMemory>::from_bytes(&from[76..76 + 4]);
        let f_wUmin = <u32 as FromIntoMemory>::from_bytes(&from[80..80 + 4]);
        let f_wUmax = <u32 as FromIntoMemory>::from_bytes(&from[84..84 + 4]);
        let f_wVmin = <u32 as FromIntoMemory>::from_bytes(&from[88..88 + 4]);
        let f_wVmax = <u32 as FromIntoMemory>::from_bytes(&from[92..92 + 4]);
        let f_wCaps = <u32 as FromIntoMemory>::from_bytes(&from[96..96 + 4]);
        let f_wMaxAxes = <u32 as FromIntoMemory>::from_bytes(&from[100..100 + 4]);
        let f_wNumAxes = <u32 as FromIntoMemory>::from_bytes(&from[104..104 + 4]);
        let f_wMaxButtons = <u32 as FromIntoMemory>::from_bytes(&from[108..108 + 4]);
        let f_szRegKey = <[super::super::Foundation::CHAR; 32] as FromIntoMemory>::from_bytes(
            &from[112..112 + 32],
        );
        let f_szOEMVxD = <[super::super::Foundation::CHAR; 260] as FromIntoMemory>::from_bytes(
            &from[144..144 + 260],
        );
        let f_ManufacturerGuid =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[404..404 + 16]);
        let f_ProductGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[420..420 + 16]);
        let f_NameGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[436..436 + 16]);
        Self {
            wMid: f_wMid,
            wPid: f_wPid,
            szPname: f_szPname,
            wXmin: f_wXmin,
            wXmax: f_wXmax,
            wYmin: f_wYmin,
            wYmax: f_wYmax,
            wZmin: f_wZmin,
            wZmax: f_wZmax,
            wNumButtons: f_wNumButtons,
            wPeriodMin: f_wPeriodMin,
            wPeriodMax: f_wPeriodMax,
            wRmin: f_wRmin,
            wRmax: f_wRmax,
            wUmin: f_wUmin,
            wUmax: f_wUmax,
            wVmin: f_wVmin,
            wVmax: f_wVmax,
            wCaps: f_wCaps,
            wMaxAxes: f_wMaxAxes,
            wNumAxes: f_wNumAxes,
            wMaxButtons: f_wMaxButtons,
            szRegKey: f_szRegKey,
            szOEMVxD: f_szOEMVxD,
            ManufacturerGuid: f_ManufacturerGuid,
            ProductGuid: f_ProductGuid,
            NameGuid: f_NameGuid,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 452);
        FromIntoMemory::into_bytes(self.wMid, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[4..4 + 32]);
        FromIntoMemory::into_bytes(self.wXmin, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.wXmax, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.wYmin, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.wYmax, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.wZmin, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.wZmax, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.wNumButtons, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.wPeriodMin, &mut into[64..64 + 4]);
        FromIntoMemory::into_bytes(self.wPeriodMax, &mut into[68..68 + 4]);
        FromIntoMemory::into_bytes(self.wRmin, &mut into[72..72 + 4]);
        FromIntoMemory::into_bytes(self.wRmax, &mut into[76..76 + 4]);
        FromIntoMemory::into_bytes(self.wUmin, &mut into[80..80 + 4]);
        FromIntoMemory::into_bytes(self.wUmax, &mut into[84..84 + 4]);
        FromIntoMemory::into_bytes(self.wVmin, &mut into[88..88 + 4]);
        FromIntoMemory::into_bytes(self.wVmax, &mut into[92..92 + 4]);
        FromIntoMemory::into_bytes(self.wCaps, &mut into[96..96 + 4]);
        FromIntoMemory::into_bytes(self.wMaxAxes, &mut into[100..100 + 4]);
        FromIntoMemory::into_bytes(self.wNumAxes, &mut into[104..104 + 4]);
        FromIntoMemory::into_bytes(self.wMaxButtons, &mut into[108..108 + 4]);
        FromIntoMemory::into_bytes(self.szRegKey, &mut into[112..112 + 32]);
        FromIntoMemory::into_bytes(self.szOEMVxD, &mut into[144..144 + 260]);
        FromIntoMemory::into_bytes(self.ManufacturerGuid, &mut into[404..404 + 16]);
        FromIntoMemory::into_bytes(self.ProductGuid, &mut into[420..420 + 16]);
        FromIntoMemory::into_bytes(self.NameGuid, &mut into[436..436 + 16]);
    }
    fn size() -> usize {
        452
    }
}
pub struct JOYCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub szPname: [u16; 32],
    pub wXmin: u32,
    pub wXmax: u32,
    pub wYmin: u32,
    pub wYmax: u32,
    pub wZmin: u32,
    pub wZmax: u32,
    pub wNumButtons: u32,
    pub wPeriodMin: u32,
    pub wPeriodMax: u32,
    pub wRmin: u32,
    pub wRmax: u32,
    pub wUmin: u32,
    pub wUmax: u32,
    pub wVmin: u32,
    pub wVmax: u32,
    pub wCaps: u32,
    pub wMaxAxes: u32,
    pub wNumAxes: u32,
    pub wMaxButtons: u32,
    pub szRegKey: [u16; 32],
    pub szOEMVxD: [u16; 260],
    pub ManufacturerGuid: crate::core::GUID,
    pub ProductGuid: crate::core::GUID,
    pub NameGuid: crate::core::GUID,
}
impl ::core::marker::Copy for JOYCAPS2W {}
impl ::core::clone::Clone for JOYCAPS2W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JOYCAPS2W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOYCAPS2W")
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("szPname", &self.szPname)
            .field("wXmin", &self.wXmin)
            .field("wXmax", &self.wXmax)
            .field("wYmin", &self.wYmin)
            .field("wYmax", &self.wYmax)
            .field("wZmin", &self.wZmin)
            .field("wZmax", &self.wZmax)
            .field("wNumButtons", &self.wNumButtons)
            .field("wPeriodMin", &self.wPeriodMin)
            .field("wPeriodMax", &self.wPeriodMax)
            .field("wRmin", &self.wRmin)
            .field("wRmax", &self.wRmax)
            .field("wUmin", &self.wUmin)
            .field("wUmax", &self.wUmax)
            .field("wVmin", &self.wVmin)
            .field("wVmax", &self.wVmax)
            .field("wCaps", &self.wCaps)
            .field("wMaxAxes", &self.wMaxAxes)
            .field("wNumAxes", &self.wNumAxes)
            .field("wMaxButtons", &self.wMaxButtons)
            .field("szRegKey", &self.szRegKey)
            .field("szOEMVxD", &self.szOEMVxD)
            .field("ManufacturerGuid", &self.ManufacturerGuid)
            .field("ProductGuid", &self.ProductGuid)
            .field("NameGuid", &self.NameGuid)
            .finish()
    }
}
impl ::core::cmp::PartialEq for JOYCAPS2W {
    fn eq(&self, other: &Self) -> bool {
        self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.szPname == other.szPname
            && self.wXmin == other.wXmin
            && self.wXmax == other.wXmax
            && self.wYmin == other.wYmin
            && self.wYmax == other.wYmax
            && self.wZmin == other.wZmin
            && self.wZmax == other.wZmax
            && self.wNumButtons == other.wNumButtons
            && self.wPeriodMin == other.wPeriodMin
            && self.wPeriodMax == other.wPeriodMax
            && self.wRmin == other.wRmin
            && self.wRmax == other.wRmax
            && self.wUmin == other.wUmin
            && self.wUmax == other.wUmax
            && self.wVmin == other.wVmin
            && self.wVmax == other.wVmax
            && self.wCaps == other.wCaps
            && self.wMaxAxes == other.wMaxAxes
            && self.wNumAxes == other.wNumAxes
            && self.wMaxButtons == other.wMaxButtons
            && self.szRegKey == other.szRegKey
            && self.szOEMVxD == other.szOEMVxD
            && self.ManufacturerGuid == other.ManufacturerGuid
            && self.ProductGuid == other.ProductGuid
            && self.NameGuid == other.NameGuid
    }
}
impl ::core::cmp::Eq for JOYCAPS2W {}
impl FromIntoMemory for JOYCAPS2W {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 452);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_szPname = <[u16; 32] as FromIntoMemory>::from_bytes(&from[4..4 + 32]);
        let f_wXmin = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_wXmax = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_wYmin = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_wYmax = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_wZmin = <u32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_wZmax = <u32 as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_wNumButtons = <u32 as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_wPeriodMin = <u32 as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        let f_wPeriodMax = <u32 as FromIntoMemory>::from_bytes(&from[68..68 + 4]);
        let f_wRmin = <u32 as FromIntoMemory>::from_bytes(&from[72..72 + 4]);
        let f_wRmax = <u32 as FromIntoMemory>::from_bytes(&from[76..76 + 4]);
        let f_wUmin = <u32 as FromIntoMemory>::from_bytes(&from[80..80 + 4]);
        let f_wUmax = <u32 as FromIntoMemory>::from_bytes(&from[84..84 + 4]);
        let f_wVmin = <u32 as FromIntoMemory>::from_bytes(&from[88..88 + 4]);
        let f_wVmax = <u32 as FromIntoMemory>::from_bytes(&from[92..92 + 4]);
        let f_wCaps = <u32 as FromIntoMemory>::from_bytes(&from[96..96 + 4]);
        let f_wMaxAxes = <u32 as FromIntoMemory>::from_bytes(&from[100..100 + 4]);
        let f_wNumAxes = <u32 as FromIntoMemory>::from_bytes(&from[104..104 + 4]);
        let f_wMaxButtons = <u32 as FromIntoMemory>::from_bytes(&from[108..108 + 4]);
        let f_szRegKey = <[u16; 32] as FromIntoMemory>::from_bytes(&from[112..112 + 32]);
        let f_szOEMVxD = <[u16; 260] as FromIntoMemory>::from_bytes(&from[144..144 + 260]);
        let f_ManufacturerGuid =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[404..404 + 16]);
        let f_ProductGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[420..420 + 16]);
        let f_NameGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[436..436 + 16]);
        Self {
            wMid: f_wMid,
            wPid: f_wPid,
            szPname: f_szPname,
            wXmin: f_wXmin,
            wXmax: f_wXmax,
            wYmin: f_wYmin,
            wYmax: f_wYmax,
            wZmin: f_wZmin,
            wZmax: f_wZmax,
            wNumButtons: f_wNumButtons,
            wPeriodMin: f_wPeriodMin,
            wPeriodMax: f_wPeriodMax,
            wRmin: f_wRmin,
            wRmax: f_wRmax,
            wUmin: f_wUmin,
            wUmax: f_wUmax,
            wVmin: f_wVmin,
            wVmax: f_wVmax,
            wCaps: f_wCaps,
            wMaxAxes: f_wMaxAxes,
            wNumAxes: f_wNumAxes,
            wMaxButtons: f_wMaxButtons,
            szRegKey: f_szRegKey,
            szOEMVxD: f_szOEMVxD,
            ManufacturerGuid: f_ManufacturerGuid,
            ProductGuid: f_ProductGuid,
            NameGuid: f_NameGuid,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 452);
        FromIntoMemory::into_bytes(self.wMid, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[4..4 + 32]);
        FromIntoMemory::into_bytes(self.wXmin, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.wXmax, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.wYmin, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.wYmax, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.wZmin, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.wZmax, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.wNumButtons, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.wPeriodMin, &mut into[64..64 + 4]);
        FromIntoMemory::into_bytes(self.wPeriodMax, &mut into[68..68 + 4]);
        FromIntoMemory::into_bytes(self.wRmin, &mut into[72..72 + 4]);
        FromIntoMemory::into_bytes(self.wRmax, &mut into[76..76 + 4]);
        FromIntoMemory::into_bytes(self.wUmin, &mut into[80..80 + 4]);
        FromIntoMemory::into_bytes(self.wUmax, &mut into[84..84 + 4]);
        FromIntoMemory::into_bytes(self.wVmin, &mut into[88..88 + 4]);
        FromIntoMemory::into_bytes(self.wVmax, &mut into[92..92 + 4]);
        FromIntoMemory::into_bytes(self.wCaps, &mut into[96..96 + 4]);
        FromIntoMemory::into_bytes(self.wMaxAxes, &mut into[100..100 + 4]);
        FromIntoMemory::into_bytes(self.wNumAxes, &mut into[104..104 + 4]);
        FromIntoMemory::into_bytes(self.wMaxButtons, &mut into[108..108 + 4]);
        FromIntoMemory::into_bytes(self.szRegKey, &mut into[112..112 + 32]);
        FromIntoMemory::into_bytes(self.szOEMVxD, &mut into[144..144 + 260]);
        FromIntoMemory::into_bytes(self.ManufacturerGuid, &mut into[404..404 + 16]);
        FromIntoMemory::into_bytes(self.ProductGuid, &mut into[420..420 + 16]);
        FromIntoMemory::into_bytes(self.NameGuid, &mut into[436..436 + 16]);
    }
    fn size() -> usize {
        452
    }
}
pub struct JOYCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub wXmin: u32,
    pub wXmax: u32,
    pub wYmin: u32,
    pub wYmax: u32,
    pub wZmin: u32,
    pub wZmax: u32,
    pub wNumButtons: u32,
    pub wPeriodMin: u32,
    pub wPeriodMax: u32,
    pub wRmin: u32,
    pub wRmax: u32,
    pub wUmin: u32,
    pub wUmax: u32,
    pub wVmin: u32,
    pub wVmax: u32,
    pub wCaps: u32,
    pub wMaxAxes: u32,
    pub wNumAxes: u32,
    pub wMaxButtons: u32,
    pub szRegKey: [super::super::Foundation::CHAR; 32],
    pub szOEMVxD: [super::super::Foundation::CHAR; 260],
}
impl ::core::marker::Copy for JOYCAPSA {}
impl ::core::clone::Clone for JOYCAPSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JOYCAPSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOYCAPSA")
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("szPname", &self.szPname)
            .field("wXmin", &self.wXmin)
            .field("wXmax", &self.wXmax)
            .field("wYmin", &self.wYmin)
            .field("wYmax", &self.wYmax)
            .field("wZmin", &self.wZmin)
            .field("wZmax", &self.wZmax)
            .field("wNumButtons", &self.wNumButtons)
            .field("wPeriodMin", &self.wPeriodMin)
            .field("wPeriodMax", &self.wPeriodMax)
            .field("wRmin", &self.wRmin)
            .field("wRmax", &self.wRmax)
            .field("wUmin", &self.wUmin)
            .field("wUmax", &self.wUmax)
            .field("wVmin", &self.wVmin)
            .field("wVmax", &self.wVmax)
            .field("wCaps", &self.wCaps)
            .field("wMaxAxes", &self.wMaxAxes)
            .field("wNumAxes", &self.wNumAxes)
            .field("wMaxButtons", &self.wMaxButtons)
            .field("szRegKey", &self.szRegKey)
            .field("szOEMVxD", &self.szOEMVxD)
            .finish()
    }
}
impl ::core::cmp::PartialEq for JOYCAPSA {
    fn eq(&self, other: &Self) -> bool {
        self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.szPname == other.szPname
            && self.wXmin == other.wXmin
            && self.wXmax == other.wXmax
            && self.wYmin == other.wYmin
            && self.wYmax == other.wYmax
            && self.wZmin == other.wZmin
            && self.wZmax == other.wZmax
            && self.wNumButtons == other.wNumButtons
            && self.wPeriodMin == other.wPeriodMin
            && self.wPeriodMax == other.wPeriodMax
            && self.wRmin == other.wRmin
            && self.wRmax == other.wRmax
            && self.wUmin == other.wUmin
            && self.wUmax == other.wUmax
            && self.wVmin == other.wVmin
            && self.wVmax == other.wVmax
            && self.wCaps == other.wCaps
            && self.wMaxAxes == other.wMaxAxes
            && self.wNumAxes == other.wNumAxes
            && self.wMaxButtons == other.wMaxButtons
            && self.szRegKey == other.szRegKey
            && self.szOEMVxD == other.szOEMVxD
    }
}
impl ::core::cmp::Eq for JOYCAPSA {}
impl FromIntoMemory for JOYCAPSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 404);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_szPname =
            <[super::super::Foundation::CHAR; 32] as FromIntoMemory>::from_bytes(&from[4..4 + 32]);
        let f_wXmin = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_wXmax = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_wYmin = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_wYmax = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_wZmin = <u32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_wZmax = <u32 as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_wNumButtons = <u32 as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_wPeriodMin = <u32 as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        let f_wPeriodMax = <u32 as FromIntoMemory>::from_bytes(&from[68..68 + 4]);
        let f_wRmin = <u32 as FromIntoMemory>::from_bytes(&from[72..72 + 4]);
        let f_wRmax = <u32 as FromIntoMemory>::from_bytes(&from[76..76 + 4]);
        let f_wUmin = <u32 as FromIntoMemory>::from_bytes(&from[80..80 + 4]);
        let f_wUmax = <u32 as FromIntoMemory>::from_bytes(&from[84..84 + 4]);
        let f_wVmin = <u32 as FromIntoMemory>::from_bytes(&from[88..88 + 4]);
        let f_wVmax = <u32 as FromIntoMemory>::from_bytes(&from[92..92 + 4]);
        let f_wCaps = <u32 as FromIntoMemory>::from_bytes(&from[96..96 + 4]);
        let f_wMaxAxes = <u32 as FromIntoMemory>::from_bytes(&from[100..100 + 4]);
        let f_wNumAxes = <u32 as FromIntoMemory>::from_bytes(&from[104..104 + 4]);
        let f_wMaxButtons = <u32 as FromIntoMemory>::from_bytes(&from[108..108 + 4]);
        let f_szRegKey = <[super::super::Foundation::CHAR; 32] as FromIntoMemory>::from_bytes(
            &from[112..112 + 32],
        );
        let f_szOEMVxD = <[super::super::Foundation::CHAR; 260] as FromIntoMemory>::from_bytes(
            &from[144..144 + 260],
        );
        Self {
            wMid: f_wMid,
            wPid: f_wPid,
            szPname: f_szPname,
            wXmin: f_wXmin,
            wXmax: f_wXmax,
            wYmin: f_wYmin,
            wYmax: f_wYmax,
            wZmin: f_wZmin,
            wZmax: f_wZmax,
            wNumButtons: f_wNumButtons,
            wPeriodMin: f_wPeriodMin,
            wPeriodMax: f_wPeriodMax,
            wRmin: f_wRmin,
            wRmax: f_wRmax,
            wUmin: f_wUmin,
            wUmax: f_wUmax,
            wVmin: f_wVmin,
            wVmax: f_wVmax,
            wCaps: f_wCaps,
            wMaxAxes: f_wMaxAxes,
            wNumAxes: f_wNumAxes,
            wMaxButtons: f_wMaxButtons,
            szRegKey: f_szRegKey,
            szOEMVxD: f_szOEMVxD,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 404);
        FromIntoMemory::into_bytes(self.wMid, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[4..4 + 32]);
        FromIntoMemory::into_bytes(self.wXmin, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.wXmax, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.wYmin, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.wYmax, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.wZmin, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.wZmax, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.wNumButtons, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.wPeriodMin, &mut into[64..64 + 4]);
        FromIntoMemory::into_bytes(self.wPeriodMax, &mut into[68..68 + 4]);
        FromIntoMemory::into_bytes(self.wRmin, &mut into[72..72 + 4]);
        FromIntoMemory::into_bytes(self.wRmax, &mut into[76..76 + 4]);
        FromIntoMemory::into_bytes(self.wUmin, &mut into[80..80 + 4]);
        FromIntoMemory::into_bytes(self.wUmax, &mut into[84..84 + 4]);
        FromIntoMemory::into_bytes(self.wVmin, &mut into[88..88 + 4]);
        FromIntoMemory::into_bytes(self.wVmax, &mut into[92..92 + 4]);
        FromIntoMemory::into_bytes(self.wCaps, &mut into[96..96 + 4]);
        FromIntoMemory::into_bytes(self.wMaxAxes, &mut into[100..100 + 4]);
        FromIntoMemory::into_bytes(self.wNumAxes, &mut into[104..104 + 4]);
        FromIntoMemory::into_bytes(self.wMaxButtons, &mut into[108..108 + 4]);
        FromIntoMemory::into_bytes(self.szRegKey, &mut into[112..112 + 32]);
        FromIntoMemory::into_bytes(self.szOEMVxD, &mut into[144..144 + 260]);
    }
    fn size() -> usize {
        404
    }
}
pub struct JOYCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub szPname: [u16; 32],
    pub wXmin: u32,
    pub wXmax: u32,
    pub wYmin: u32,
    pub wYmax: u32,
    pub wZmin: u32,
    pub wZmax: u32,
    pub wNumButtons: u32,
    pub wPeriodMin: u32,
    pub wPeriodMax: u32,
    pub wRmin: u32,
    pub wRmax: u32,
    pub wUmin: u32,
    pub wUmax: u32,
    pub wVmin: u32,
    pub wVmax: u32,
    pub wCaps: u32,
    pub wMaxAxes: u32,
    pub wNumAxes: u32,
    pub wMaxButtons: u32,
    pub szRegKey: [u16; 32],
    pub szOEMVxD: [u16; 260],
}
impl ::core::marker::Copy for JOYCAPSW {}
impl ::core::clone::Clone for JOYCAPSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JOYCAPSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOYCAPSW")
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("szPname", &self.szPname)
            .field("wXmin", &self.wXmin)
            .field("wXmax", &self.wXmax)
            .field("wYmin", &self.wYmin)
            .field("wYmax", &self.wYmax)
            .field("wZmin", &self.wZmin)
            .field("wZmax", &self.wZmax)
            .field("wNumButtons", &self.wNumButtons)
            .field("wPeriodMin", &self.wPeriodMin)
            .field("wPeriodMax", &self.wPeriodMax)
            .field("wRmin", &self.wRmin)
            .field("wRmax", &self.wRmax)
            .field("wUmin", &self.wUmin)
            .field("wUmax", &self.wUmax)
            .field("wVmin", &self.wVmin)
            .field("wVmax", &self.wVmax)
            .field("wCaps", &self.wCaps)
            .field("wMaxAxes", &self.wMaxAxes)
            .field("wNumAxes", &self.wNumAxes)
            .field("wMaxButtons", &self.wMaxButtons)
            .field("szRegKey", &self.szRegKey)
            .field("szOEMVxD", &self.szOEMVxD)
            .finish()
    }
}
impl ::core::cmp::PartialEq for JOYCAPSW {
    fn eq(&self, other: &Self) -> bool {
        self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.szPname == other.szPname
            && self.wXmin == other.wXmin
            && self.wXmax == other.wXmax
            && self.wYmin == other.wYmin
            && self.wYmax == other.wYmax
            && self.wZmin == other.wZmin
            && self.wZmax == other.wZmax
            && self.wNumButtons == other.wNumButtons
            && self.wPeriodMin == other.wPeriodMin
            && self.wPeriodMax == other.wPeriodMax
            && self.wRmin == other.wRmin
            && self.wRmax == other.wRmax
            && self.wUmin == other.wUmin
            && self.wUmax == other.wUmax
            && self.wVmin == other.wVmin
            && self.wVmax == other.wVmax
            && self.wCaps == other.wCaps
            && self.wMaxAxes == other.wMaxAxes
            && self.wNumAxes == other.wNumAxes
            && self.wMaxButtons == other.wMaxButtons
            && self.szRegKey == other.szRegKey
            && self.szOEMVxD == other.szOEMVxD
    }
}
impl ::core::cmp::Eq for JOYCAPSW {}
impl FromIntoMemory for JOYCAPSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 404);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_szPname = <[u16; 32] as FromIntoMemory>::from_bytes(&from[4..4 + 32]);
        let f_wXmin = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_wXmax = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_wYmin = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_wYmax = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_wZmin = <u32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_wZmax = <u32 as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_wNumButtons = <u32 as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_wPeriodMin = <u32 as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        let f_wPeriodMax = <u32 as FromIntoMemory>::from_bytes(&from[68..68 + 4]);
        let f_wRmin = <u32 as FromIntoMemory>::from_bytes(&from[72..72 + 4]);
        let f_wRmax = <u32 as FromIntoMemory>::from_bytes(&from[76..76 + 4]);
        let f_wUmin = <u32 as FromIntoMemory>::from_bytes(&from[80..80 + 4]);
        let f_wUmax = <u32 as FromIntoMemory>::from_bytes(&from[84..84 + 4]);
        let f_wVmin = <u32 as FromIntoMemory>::from_bytes(&from[88..88 + 4]);
        let f_wVmax = <u32 as FromIntoMemory>::from_bytes(&from[92..92 + 4]);
        let f_wCaps = <u32 as FromIntoMemory>::from_bytes(&from[96..96 + 4]);
        let f_wMaxAxes = <u32 as FromIntoMemory>::from_bytes(&from[100..100 + 4]);
        let f_wNumAxes = <u32 as FromIntoMemory>::from_bytes(&from[104..104 + 4]);
        let f_wMaxButtons = <u32 as FromIntoMemory>::from_bytes(&from[108..108 + 4]);
        let f_szRegKey = <[u16; 32] as FromIntoMemory>::from_bytes(&from[112..112 + 32]);
        let f_szOEMVxD = <[u16; 260] as FromIntoMemory>::from_bytes(&from[144..144 + 260]);
        Self {
            wMid: f_wMid,
            wPid: f_wPid,
            szPname: f_szPname,
            wXmin: f_wXmin,
            wXmax: f_wXmax,
            wYmin: f_wYmin,
            wYmax: f_wYmax,
            wZmin: f_wZmin,
            wZmax: f_wZmax,
            wNumButtons: f_wNumButtons,
            wPeriodMin: f_wPeriodMin,
            wPeriodMax: f_wPeriodMax,
            wRmin: f_wRmin,
            wRmax: f_wRmax,
            wUmin: f_wUmin,
            wUmax: f_wUmax,
            wVmin: f_wVmin,
            wVmax: f_wVmax,
            wCaps: f_wCaps,
            wMaxAxes: f_wMaxAxes,
            wNumAxes: f_wNumAxes,
            wMaxButtons: f_wMaxButtons,
            szRegKey: f_szRegKey,
            szOEMVxD: f_szOEMVxD,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 404);
        FromIntoMemory::into_bytes(self.wMid, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[4..4 + 32]);
        FromIntoMemory::into_bytes(self.wXmin, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.wXmax, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.wYmin, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.wYmax, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.wZmin, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.wZmax, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.wNumButtons, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.wPeriodMin, &mut into[64..64 + 4]);
        FromIntoMemory::into_bytes(self.wPeriodMax, &mut into[68..68 + 4]);
        FromIntoMemory::into_bytes(self.wRmin, &mut into[72..72 + 4]);
        FromIntoMemory::into_bytes(self.wRmax, &mut into[76..76 + 4]);
        FromIntoMemory::into_bytes(self.wUmin, &mut into[80..80 + 4]);
        FromIntoMemory::into_bytes(self.wUmax, &mut into[84..84 + 4]);
        FromIntoMemory::into_bytes(self.wVmin, &mut into[88..88 + 4]);
        FromIntoMemory::into_bytes(self.wVmax, &mut into[92..92 + 4]);
        FromIntoMemory::into_bytes(self.wCaps, &mut into[96..96 + 4]);
        FromIntoMemory::into_bytes(self.wMaxAxes, &mut into[100..100 + 4]);
        FromIntoMemory::into_bytes(self.wNumAxes, &mut into[104..104 + 4]);
        FromIntoMemory::into_bytes(self.wMaxButtons, &mut into[108..108 + 4]);
        FromIntoMemory::into_bytes(self.szRegKey, &mut into[112..112 + 32]);
        FromIntoMemory::into_bytes(self.szOEMVxD, &mut into[144..144 + 260]);
    }
    fn size() -> usize {
        404
    }
}
pub const JOYCAPS_HASPOV: u32 = 16u32;
pub const JOYCAPS_HASR: u32 = 2u32;
pub const JOYCAPS_HASU: u32 = 4u32;
pub const JOYCAPS_HASV: u32 = 8u32;
pub const JOYCAPS_HASZ: u32 = 1u32;
pub const JOYCAPS_POV4DIR: u32 = 32u32;
pub const JOYCAPS_POVCTS: u32 = 64u32;
pub const JOYERR_NOCANDO: u32 = 166u32;
pub const JOYERR_NOERROR: u32 = 0u32;
pub const JOYERR_PARMS: u32 = 165u32;
pub const JOYERR_UNPLUGGED: u32 = 167u32;
pub struct JOYINFO {
    pub wXpos: u32,
    pub wYpos: u32,
    pub wZpos: u32,
    pub wButtons: u32,
}
impl ::core::marker::Copy for JOYINFO {}
impl ::core::clone::Clone for JOYINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JOYINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOYINFO")
            .field("wXpos", &self.wXpos)
            .field("wYpos", &self.wYpos)
            .field("wZpos", &self.wZpos)
            .field("wButtons", &self.wButtons)
            .finish()
    }
}
impl ::core::cmp::PartialEq for JOYINFO {
    fn eq(&self, other: &Self) -> bool {
        self.wXpos == other.wXpos
            && self.wYpos == other.wYpos
            && self.wZpos == other.wZpos
            && self.wButtons == other.wButtons
    }
}
impl ::core::cmp::Eq for JOYINFO {}
impl FromIntoMemory for JOYINFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_wXpos = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_wYpos = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_wZpos = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_wButtons = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            wXpos: f_wXpos,
            wYpos: f_wYpos,
            wZpos: f_wZpos,
            wButtons: f_wButtons,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.wXpos, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.wYpos, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.wZpos, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.wButtons, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct JOYINFOEX {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwXpos: u32,
    pub dwYpos: u32,
    pub dwZpos: u32,
    pub dwRpos: u32,
    pub dwUpos: u32,
    pub dwVpos: u32,
    pub dwButtons: u32,
    pub dwButtonNumber: u32,
    pub dwPOV: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
impl ::core::marker::Copy for JOYINFOEX {}
impl ::core::clone::Clone for JOYINFOEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JOYINFOEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOYINFOEX")
            .field("dwSize", &self.dwSize)
            .field("dwFlags", &self.dwFlags)
            .field("dwXpos", &self.dwXpos)
            .field("dwYpos", &self.dwYpos)
            .field("dwZpos", &self.dwZpos)
            .field("dwRpos", &self.dwRpos)
            .field("dwUpos", &self.dwUpos)
            .field("dwVpos", &self.dwVpos)
            .field("dwButtons", &self.dwButtons)
            .field("dwButtonNumber", &self.dwButtonNumber)
            .field("dwPOV", &self.dwPOV)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .finish()
    }
}
impl ::core::cmp::PartialEq for JOYINFOEX {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwFlags == other.dwFlags
            && self.dwXpos == other.dwXpos
            && self.dwYpos == other.dwYpos
            && self.dwZpos == other.dwZpos
            && self.dwRpos == other.dwRpos
            && self.dwUpos == other.dwUpos
            && self.dwVpos == other.dwVpos
            && self.dwButtons == other.dwButtons
            && self.dwButtonNumber == other.dwButtonNumber
            && self.dwPOV == other.dwPOV
            && self.dwReserved1 == other.dwReserved1
            && self.dwReserved2 == other.dwReserved2
    }
}
impl ::core::cmp::Eq for JOYINFOEX {}
impl FromIntoMemory for JOYINFOEX {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 52);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwXpos = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwYpos = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwZpos = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwRpos = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwUpos = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_dwVpos = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_dwButtons = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_dwButtonNumber = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_dwPOV = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_dwReserved1 = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_dwReserved2 = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        Self {
            dwSize: f_dwSize,
            dwFlags: f_dwFlags,
            dwXpos: f_dwXpos,
            dwYpos: f_dwYpos,
            dwZpos: f_dwZpos,
            dwRpos: f_dwRpos,
            dwUpos: f_dwUpos,
            dwVpos: f_dwVpos,
            dwButtons: f_dwButtons,
            dwButtonNumber: f_dwButtonNumber,
            dwPOV: f_dwPOV,
            dwReserved1: f_dwReserved1,
            dwReserved2: f_dwReserved2,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 52);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwXpos, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwYpos, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwZpos, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwRpos, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwUpos, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.dwVpos, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.dwButtons, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.dwButtonNumber, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.dwPOV, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.dwReserved1, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.dwReserved2, &mut into[48..48 + 4]);
    }
    fn size() -> usize {
        52
    }
}
pub const JOYSTICKID1: u32 = 0u32;
pub const JOYSTICKID2: u32 = 1u32;
pub const JOY_BUTTON1: u32 = 1u32;
pub const JOY_BUTTON10: i32 = 512i32;
pub const JOY_BUTTON11: i32 = 1024i32;
pub const JOY_BUTTON12: i32 = 2048i32;
pub const JOY_BUTTON13: i32 = 4096i32;
pub const JOY_BUTTON14: i32 = 8192i32;
pub const JOY_BUTTON15: i32 = 16384i32;
pub const JOY_BUTTON16: i32 = 32768i32;
pub const JOY_BUTTON17: i32 = 65536i32;
pub const JOY_BUTTON18: i32 = 131072i32;
pub const JOY_BUTTON19: i32 = 262144i32;
pub const JOY_BUTTON1CHG: u32 = 256u32;
pub const JOY_BUTTON2: u32 = 2u32;
pub const JOY_BUTTON20: i32 = 524288i32;
pub const JOY_BUTTON21: i32 = 1048576i32;
pub const JOY_BUTTON22: i32 = 2097152i32;
pub const JOY_BUTTON23: i32 = 4194304i32;
pub const JOY_BUTTON24: i32 = 8388608i32;
pub const JOY_BUTTON25: i32 = 16777216i32;
pub const JOY_BUTTON26: i32 = 33554432i32;
pub const JOY_BUTTON27: i32 = 67108864i32;
pub const JOY_BUTTON28: i32 = 134217728i32;
pub const JOY_BUTTON29: i32 = 268435456i32;
pub const JOY_BUTTON2CHG: u32 = 512u32;
pub const JOY_BUTTON3: u32 = 4u32;
pub const JOY_BUTTON30: i32 = 536870912i32;
pub const JOY_BUTTON31: i32 = 1073741824i32;
pub const JOY_BUTTON32: i32 = -2147483648i32;
pub const JOY_BUTTON3CHG: u32 = 1024u32;
pub const JOY_BUTTON4: u32 = 8u32;
pub const JOY_BUTTON4CHG: u32 = 2048u32;
pub const JOY_BUTTON5: i32 = 16i32;
pub const JOY_BUTTON6: i32 = 32i32;
pub const JOY_BUTTON7: i32 = 64i32;
pub const JOY_BUTTON8: i32 = 128i32;
pub const JOY_BUTTON9: i32 = 256i32;
pub const JOY_CAL_READ3: i32 = 262144i32;
pub const JOY_CAL_READ4: i32 = 524288i32;
pub const JOY_CAL_READ5: i32 = 4194304i32;
pub const JOY_CAL_READ6: i32 = 8388608i32;
pub const JOY_CAL_READALWAYS: i32 = 65536i32;
pub const JOY_CAL_READRONLY: i32 = 33554432i32;
pub const JOY_CAL_READUONLY: i32 = 67108864i32;
pub const JOY_CAL_READVONLY: i32 = 134217728i32;
pub const JOY_CAL_READXONLY: i32 = 1048576i32;
pub const JOY_CAL_READXYONLY: i32 = 131072i32;
pub const JOY_CAL_READYONLY: i32 = 2097152i32;
pub const JOY_CAL_READZONLY: i32 = 16777216i32;
pub const JOY_CONFIGCHANGED_MSGSTRING: &'static str = "MSJSTICK_VJOYD_MSGSTR";
pub const JOY_POVBACKWARD: u32 = 18000u32;
pub const JOY_POVFORWARD: u32 = 0u32;
pub const JOY_POVLEFT: u32 = 27000u32;
pub const JOY_POVRIGHT: u32 = 9000u32;
pub const JOY_RETURNBUTTONS: i32 = 128i32;
pub const JOY_RETURNCENTERED: i32 = 1024i32;
pub const JOY_RETURNPOV: i32 = 64i32;
pub const JOY_RETURNPOVCTS: i32 = 512i32;
pub const JOY_RETURNR: i32 = 8i32;
pub const JOY_RETURNRAWDATA: i32 = 256i32;
pub const JOY_RETURNU: i32 = 16i32;
pub const JOY_RETURNV: i32 = 32i32;
pub const JOY_RETURNX: i32 = 1i32;
pub const JOY_RETURNY: i32 = 2i32;
pub const JOY_RETURNZ: i32 = 4i32;
pub const JOY_USEDEADZONE: i32 = 2048i32;
pub struct JPEGINFOHEADER {
    pub JPEGSize: u32,
    pub JPEGProcess: u32,
    pub JPEGColorSpaceID: u32,
    pub JPEGBitsPerSample: u32,
    pub JPEGHSubSampling: u32,
    pub JPEGVSubSampling: u32,
}
impl ::core::marker::Copy for JPEGINFOHEADER {}
impl ::core::clone::Clone for JPEGINFOHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JPEGINFOHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JPEGINFOHEADER")
            .field("JPEGSize", &self.JPEGSize)
            .field("JPEGProcess", &self.JPEGProcess)
            .field("JPEGColorSpaceID", &self.JPEGColorSpaceID)
            .field("JPEGBitsPerSample", &self.JPEGBitsPerSample)
            .field("JPEGHSubSampling", &self.JPEGHSubSampling)
            .field("JPEGVSubSampling", &self.JPEGVSubSampling)
            .finish()
    }
}
impl ::core::cmp::PartialEq for JPEGINFOHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.JPEGSize == other.JPEGSize
            && self.JPEGProcess == other.JPEGProcess
            && self.JPEGColorSpaceID == other.JPEGColorSpaceID
            && self.JPEGBitsPerSample == other.JPEGBitsPerSample
            && self.JPEGHSubSampling == other.JPEGHSubSampling
            && self.JPEGVSubSampling == other.JPEGVSubSampling
    }
}
impl ::core::cmp::Eq for JPEGINFOHEADER {}
impl FromIntoMemory for JPEGINFOHEADER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_JPEGSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_JPEGProcess = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_JPEGColorSpaceID = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_JPEGBitsPerSample = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_JPEGHSubSampling = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_JPEGVSubSampling = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            JPEGSize: f_JPEGSize,
            JPEGProcess: f_JPEGProcess,
            JPEGColorSpaceID: f_JPEGColorSpaceID,
            JPEGBitsPerSample: f_JPEGBitsPerSample,
            JPEGHSubSampling: f_JPEGHSubSampling,
            JPEGVSubSampling: f_JPEGVSubSampling,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.JPEGSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.JPEGProcess, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.JPEGColorSpaceID, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.JPEGBitsPerSample, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.JPEGHSubSampling, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.JPEGVSubSampling, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub const JPEG_PROCESS_BASELINE: u32 = 0u32;
pub const JPEG_RGB: u32 = 3u32;
pub const JPEG_Y: u32 = 1u32;
pub const JPEG_YCbCr: u32 = 2u32;
pub const KSDATAFORMAT_SUBTYPE_IEEE_FLOAT: crate::core::GUID =
    crate::core::GUID::from_u128(0x00000003_0000_0010_8000_00aa00389b71);
pub type LPFNEXTDEVIO = StdCallFnPtr<
    (
        super::super::Foundation::LPARAM,
        u32,
        u32,
        MutPtr<::core::ffi::c_void>,
        u32,
        MutPtr<::core::ffi::c_void>,
        u32,
        MutPtr<u32>,
        MutPtr<super::super::System::IO::OVERLAPPED>,
    ),
    super::super::Foundation::BOOL,
>;
pub type LPMMIOPROC = StdCallFnPtr<
    (
        PCSTR,
        u32,
        super::super::Foundation::LPARAM,
        super::super::Foundation::LPARAM,
    ),
    super::super::Foundation::LRESULT,
>;
pub type LPTASKCALLBACK = StdCallFnPtr<(PtrRepr,), ()>;
pub const MCIERR_AVI_AUDIOERROR: u32 = 619u32;
pub const MCIERR_AVI_BADPALETTE: u32 = 620u32;
pub const MCIERR_AVI_CANTPLAYFULLSCREEN: u32 = 615u32;
pub const MCIERR_AVI_DISPLAYERROR: u32 = 618u32;
pub const MCIERR_AVI_NOCOMPRESSOR: u32 = 617u32;
pub const MCIERR_AVI_NODISPDIB: u32 = 614u32;
pub const MCIERR_AVI_NOTINTERLEAVED: u32 = 613u32;
pub const MCIERR_AVI_OLDAVIFORMAT: u32 = 612u32;
pub const MCIERR_AVI_TOOBIGFORVGA: u32 = 616u32;
pub const MCIERR_BAD_CONSTANT: u32 = 290u32;
pub const MCIERR_BAD_INTEGER: u32 = 270u32;
pub const MCIERR_BAD_TIME_FORMAT: u32 = 293u32;
pub const MCIERR_CANNOT_LOAD_DRIVER: u32 = 266u32;
pub const MCIERR_CANNOT_USE_ALL: u32 = 279u32;
pub const MCIERR_CREATEWINDOW: u32 = 347u32;
pub const MCIERR_CUSTOM_DRIVER_BASE: u32 = 512u32;
pub const MCIERR_DEVICE_LENGTH: u32 = 310u32;
pub const MCIERR_DEVICE_LOCKED: u32 = 288u32;
pub const MCIERR_DEVICE_NOT_INSTALLED: u32 = 306u32;
pub const MCIERR_DEVICE_NOT_READY: u32 = 276u32;
pub const MCIERR_DEVICE_OPEN: u32 = 265u32;
pub const MCIERR_DEVICE_ORD_LENGTH: u32 = 311u32;
pub const MCIERR_DEVICE_TYPE_REQUIRED: u32 = 287u32;
pub const MCIERR_DGV_BAD_CLIPBOARD_RANGE: u32 = 517u32;
pub const MCIERR_DGV_DEVICE_LIMIT: u32 = 512u32;
pub const MCIERR_DGV_DEVICE_MEMORY_FULL: u32 = 516u32;
pub const MCIERR_DGV_DISK_FULL: u32 = 515u32;
pub const MCIERR_DGV_IOERR: u32 = 513u32;
pub const MCIERR_DGV_WORKSPACE_EMPTY: u32 = 514u32;
pub const MCIERR_DRIVER: u32 = 278u32;
pub const MCIERR_DRIVER_INTERNAL: u32 = 272u32;
pub const MCIERR_DUPLICATE_ALIAS: u32 = 289u32;
pub const MCIERR_DUPLICATE_FLAGS: u32 = 295u32;
pub const MCIERR_EXTENSION_NOT_FOUND: u32 = 281u32;
pub const MCIERR_EXTRA_CHARACTERS: u32 = 305u32;
pub const MCIERR_FILENAME_REQUIRED: u32 = 304u32;
pub const MCIERR_FILE_NOT_FOUND: u32 = 275u32;
pub const MCIERR_FILE_NOT_SAVED: u32 = 286u32;
pub const MCIERR_FILE_READ: u32 = 348u32;
pub const MCIERR_FILE_WRITE: u32 = 349u32;
pub const MCIERR_FLAGS_NOT_COMPATIBLE: u32 = 284u32;
pub const MCIERR_GET_CD: u32 = 307u32;
pub const MCIERR_HARDWARE: u32 = 262u32;
pub const MCIERR_ILLEGAL_FOR_AUTO_OPEN: u32 = 303u32;
pub const MCIERR_INTERNAL: u32 = 277u32;
pub const MCIERR_INVALID_DEVICE_ID: u32 = 257u32;
pub const MCIERR_INVALID_DEVICE_NAME: u32 = 263u32;
pub const MCIERR_INVALID_FILE: u32 = 296u32;
pub const MCIERR_MISSING_COMMAND_STRING: u32 = 267u32;
pub const MCIERR_MISSING_DEVICE_NAME: u32 = 292u32;
pub const MCIERR_MISSING_PARAMETER: u32 = 273u32;
pub const MCIERR_MISSING_STRING_ARGUMENT: u32 = 269u32;
pub const MCIERR_MULTIPLE: u32 = 280u32;
pub const MCIERR_MUST_USE_SHAREABLE: u32 = 291u32;
pub const MCIERR_NEW_REQUIRES_ALIAS: u32 = 299u32;
pub const MCIERR_NONAPPLICABLE_FUNCTION: u32 = 302u32;
pub const MCIERR_NOTIFY_ON_AUTO_OPEN: u32 = 300u32;
pub const MCIERR_NO_CLOSING_QUOTE: u32 = 294u32;
pub const MCIERR_NO_ELEMENT_ALLOWED: u32 = 301u32;
pub const MCIERR_NO_IDENTITY: u32 = 350u32;
pub const MCIERR_NO_INTEGER: u32 = 312u32;
pub const MCIERR_NO_WINDOW: u32 = 346u32;
pub const MCIERR_NULL_PARAMETER_BLOCK: u32 = 297u32;
pub const MCIERR_OUTOFRANGE: u32 = 282u32;
pub const MCIERR_OUT_OF_MEMORY: u32 = 264u32;
pub const MCIERR_PARAM_OVERFLOW: u32 = 268u32;
pub const MCIERR_PARSER_INTERNAL: u32 = 271u32;
pub const MCIERR_SEQ_DIV_INCOMPATIBLE: u32 = 336u32;
pub const MCIERR_SEQ_NOMIDIPRESENT: u32 = 343u32;
pub const MCIERR_SEQ_PORTUNSPECIFIED: u32 = 342u32;
pub const MCIERR_SEQ_PORT_INUSE: u32 = 337u32;
pub const MCIERR_SEQ_PORT_MAPNODEVICE: u32 = 339u32;
pub const MCIERR_SEQ_PORT_MISCERROR: u32 = 340u32;
pub const MCIERR_SEQ_PORT_NONEXISTENT: u32 = 338u32;
pub const MCIERR_SEQ_TIMER: u32 = 341u32;
pub const MCIERR_SET_CD: u32 = 308u32;
pub const MCIERR_SET_DRIVE: u32 = 309u32;
pub const MCIERR_UNNAMED_RESOURCE: u32 = 298u32;
pub const MCIERR_UNRECOGNIZED_COMMAND: u32 = 261u32;
pub const MCIERR_UNRECOGNIZED_KEYWORD: u32 = 259u32;
pub const MCIERR_UNSUPPORTED_FUNCTION: u32 = 274u32;
pub const MCIERR_WAVE_INPUTSINUSE: u32 = 322u32;
pub const MCIERR_WAVE_INPUTSUNSUITABLE: u32 = 328u32;
pub const MCIERR_WAVE_INPUTUNSPECIFIED: u32 = 325u32;
pub const MCIERR_WAVE_OUTPUTSINUSE: u32 = 320u32;
pub const MCIERR_WAVE_OUTPUTSUNSUITABLE: u32 = 326u32;
pub const MCIERR_WAVE_OUTPUTUNSPECIFIED: u32 = 324u32;
pub const MCIERR_WAVE_SETINPUTINUSE: u32 = 323u32;
pub const MCIERR_WAVE_SETINPUTUNSUITABLE: u32 = 329u32;
pub const MCIERR_WAVE_SETOUTPUTINUSE: u32 = 321u32;
pub const MCIERR_WAVE_SETOUTPUTUNSUITABLE: u32 = 327u32;
pub const MCIWNDF_NOAUTOSIZEMOVIE: u32 = 4u32;
pub const MCIWNDF_NOAUTOSIZEWINDOW: u32 = 1u32;
pub const MCIWNDF_NOERRORDLG: u32 = 16384u32;
pub const MCIWNDF_NOMENU: u32 = 8u32;
pub const MCIWNDF_NOOPEN: u32 = 32768u32;
pub const MCIWNDF_NOPLAYBAR: u32 = 2u32;
pub const MCIWNDF_NOTIFYALL: u32 = 7936u32;
pub const MCIWNDF_NOTIFYANSI: u32 = 128u32;
pub const MCIWNDF_NOTIFYERROR: u32 = 4096u32;
pub const MCIWNDF_NOTIFYMEDIA: u32 = 2048u32;
pub const MCIWNDF_NOTIFYMEDIAA: u32 = 2176u32;
pub const MCIWNDF_NOTIFYMEDIAW: u32 = 2048u32;
pub const MCIWNDF_NOTIFYMODE: u32 = 256u32;
pub const MCIWNDF_NOTIFYPOS: u32 = 512u32;
pub const MCIWNDF_NOTIFYSIZE: u32 = 1024u32;
pub const MCIWNDF_RECORD: u32 = 8192u32;
pub const MCIWNDF_SHOWALL: u32 = 112u32;
pub const MCIWNDF_SHOWMODE: u32 = 64u32;
pub const MCIWNDF_SHOWNAME: u32 = 16u32;
pub const MCIWNDF_SHOWPOS: u32 = 32u32;
pub const MCIWNDM_CAN_CONFIG: u32 = 1173u32;
pub const MCIWNDM_CAN_EJECT: u32 = 1172u32;
pub const MCIWNDM_CAN_PLAY: u32 = 1168u32;
pub const MCIWNDM_CAN_RECORD: u32 = 1170u32;
pub const MCIWNDM_CAN_SAVE: u32 = 1171u32;
pub const MCIWNDM_CAN_WINDOW: u32 = 1169u32;
pub const MCIWNDM_CHANGESTYLES: u32 = 1159u32;
pub const MCIWNDM_EJECT: u32 = 1131u32;
pub const MCIWNDM_GETACTIVETIMER: u32 = 1156u32;
pub const MCIWNDM_GETALIAS: u32 = 1161u32;
pub const MCIWNDM_GETDEVICE: u32 = 1249u32;
pub const MCIWNDM_GETDEVICEA: u32 = 1149u32;
pub const MCIWNDM_GETDEVICEID: u32 = 1124u32;
pub const MCIWNDM_GETDEVICEW: u32 = 1249u32;
pub const MCIWNDM_GETEND: u32 = 1129u32;
pub const MCIWNDM_GETERROR: u32 = 1252u32;
pub const MCIWNDM_GETERRORA: u32 = 1152u32;
pub const MCIWNDM_GETERRORW: u32 = 1252u32;
pub const MCIWNDM_GETFILENAME: u32 = 1248u32;
pub const MCIWNDM_GETFILENAMEA: u32 = 1148u32;
pub const MCIWNDM_GETFILENAMEW: u32 = 1248u32;
pub const MCIWNDM_GETINACTIVETIMER: u32 = 1157u32;
pub const MCIWNDM_GETLENGTH: u32 = 1128u32;
pub const MCIWNDM_GETMODE: u32 = 1230u32;
pub const MCIWNDM_GETMODEA: u32 = 1130u32;
pub const MCIWNDM_GETMODEW: u32 = 1230u32;
pub const MCIWNDM_GETPALETTE: u32 = 1150u32;
pub const MCIWNDM_GETPOSITION: u32 = 1226u32;
pub const MCIWNDM_GETPOSITIONA: u32 = 1126u32;
pub const MCIWNDM_GETPOSITIONW: u32 = 1226u32;
pub const MCIWNDM_GETREPEAT: u32 = 1139u32;
pub const MCIWNDM_GETSPEED: u32 = 1137u32;
pub const MCIWNDM_GETSTART: u32 = 1127u32;
pub const MCIWNDM_GETSTYLES: u32 = 1160u32;
pub const MCIWNDM_GETTIMEFORMAT: u32 = 1244u32;
pub const MCIWNDM_GETTIMEFORMATA: u32 = 1144u32;
pub const MCIWNDM_GETTIMEFORMATW: u32 = 1244u32;
pub const MCIWNDM_GETVOLUME: u32 = 1135u32;
pub const MCIWNDM_GETZOOM: u32 = 1133u32;
pub const MCIWNDM_GET_DEST: u32 = 1166u32;
pub const MCIWNDM_GET_SOURCE: u32 = 1164u32;
pub const MCIWNDM_NEW: u32 = 1258u32;
pub const MCIWNDM_NEWA: u32 = 1158u32;
pub const MCIWNDM_NEWW: u32 = 1258u32;
pub const MCIWNDM_NOTIFYERROR: u32 = 1229u32;
pub const MCIWNDM_NOTIFYMEDIA: u32 = 1227u32;
pub const MCIWNDM_NOTIFYMODE: u32 = 1224u32;
pub const MCIWNDM_NOTIFYPOS: u32 = 1225u32;
pub const MCIWNDM_NOTIFYSIZE: u32 = 1226u32;
pub const MCIWNDM_OPEN: u32 = 1276u32;
pub const MCIWNDM_OPENA: u32 = 1177u32;
pub const MCIWNDM_OPENINTERFACE: u32 = 1175u32;
pub const MCIWNDM_OPENW: u32 = 1276u32;
pub const MCIWNDM_PALETTEKICK: u32 = 1174u32;
pub const MCIWNDM_PLAYFROM: u32 = 1146u32;
pub const MCIWNDM_PLAYREVERSE: u32 = 1163u32;
pub const MCIWNDM_PLAYTO: u32 = 1147u32;
pub const MCIWNDM_PUT_DEST: u32 = 1167u32;
pub const MCIWNDM_PUT_SOURCE: u32 = 1165u32;
pub const MCIWNDM_REALIZE: u32 = 1142u32;
pub const MCIWNDM_RETURNSTRING: u32 = 1262u32;
pub const MCIWNDM_RETURNSTRINGA: u32 = 1162u32;
pub const MCIWNDM_RETURNSTRINGW: u32 = 1262u32;
pub const MCIWNDM_SENDSTRING: u32 = 1225u32;
pub const MCIWNDM_SENDSTRINGA: u32 = 1125u32;
pub const MCIWNDM_SENDSTRINGW: u32 = 1225u32;
pub const MCIWNDM_SETACTIVETIMER: u32 = 1154u32;
pub const MCIWNDM_SETINACTIVETIMER: u32 = 1155u32;
pub const MCIWNDM_SETOWNER: u32 = 1176u32;
pub const MCIWNDM_SETPALETTE: u32 = 1151u32;
pub const MCIWNDM_SETREPEAT: u32 = 1138u32;
pub const MCIWNDM_SETSPEED: u32 = 1136u32;
pub const MCIWNDM_SETTIMEFORMAT: u32 = 1243u32;
pub const MCIWNDM_SETTIMEFORMATA: u32 = 1143u32;
pub const MCIWNDM_SETTIMEFORMATW: u32 = 1243u32;
pub const MCIWNDM_SETTIMERS: u32 = 1153u32;
pub const MCIWNDM_SETVOLUME: u32 = 1134u32;
pub const MCIWNDM_SETZOOM: u32 = 1132u32;
pub const MCIWNDM_VALIDATEMEDIA: u32 = 1145u32;
pub const MCIWNDOPENF_NEW: u32 = 1u32;
pub const MCIWND_END: i32 = -2i32;
pub const MCIWND_START: i32 = -1i32;
pub const MCIWND_WINDOW_CLASS: &'static str = "MCIWndClass";
pub const MCI_ANIM_GETDEVCAPS_CAN_REVERSE: i32 = 16385i32;
pub const MCI_ANIM_GETDEVCAPS_CAN_STRETCH: i32 = 16391i32;
pub const MCI_ANIM_GETDEVCAPS_FAST_RATE: i32 = 16386i32;
pub const MCI_ANIM_GETDEVCAPS_MAX_WINDOWS: i32 = 16392i32;
pub const MCI_ANIM_GETDEVCAPS_NORMAL_RATE: i32 = 16388i32;
pub const MCI_ANIM_GETDEVCAPS_PALETTES: i32 = 16390i32;
pub const MCI_ANIM_GETDEVCAPS_SLOW_RATE: i32 = 16387i32;
pub const MCI_ANIM_INFO_TEXT: i32 = 65536i32;
pub const MCI_ANIM_OPEN_NOSTATIC: i32 = 262144i32;
pub const MCI_ANIM_OPEN_PARENT: i32 = 131072i32;
pub struct MCI_ANIM_OPEN_PARMSA {
    pub dwCallback: PtrRepr,
    pub wDeviceID: u32,
    pub lpstrDeviceType: PCSTR,
    pub lpstrElementName: PCSTR,
    pub lpstrAlias: PCSTR,
    pub dwStyle: u32,
    pub hWndParent: super::super::Foundation::HWND,
}
impl ::core::marker::Copy for MCI_ANIM_OPEN_PARMSA {}
impl ::core::clone::Clone for MCI_ANIM_OPEN_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_ANIM_OPEN_PARMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_ANIM_OPEN_PARMSA")
            .field("dwCallback", &self.dwCallback)
            .field("wDeviceID", &self.wDeviceID)
            .field("lpstrDeviceType", &self.lpstrDeviceType)
            .field("lpstrElementName", &self.lpstrElementName)
            .field("lpstrAlias", &self.lpstrAlias)
            .field("dwStyle", &self.dwStyle)
            .field("hWndParent", &self.hWndParent)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_ANIM_OPEN_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.wDeviceID == other.wDeviceID
            && self.lpstrDeviceType == other.lpstrDeviceType
            && self.lpstrElementName == other.lpstrElementName
            && self.lpstrAlias == other.lpstrAlias
            && self.dwStyle == other.dwStyle
            && self.hWndParent == other.hWndParent
    }
}
impl ::core::cmp::Eq for MCI_ANIM_OPEN_PARMSA {}
impl FromIntoMemory for MCI_ANIM_OPEN_PARMSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_wDeviceID = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpstrDeviceType = <PCSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpstrElementName = <PCSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpstrAlias = <PCSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwStyle = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_hWndParent =
            <super::super::Foundation::HWND as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        Self {
            dwCallback: f_dwCallback,
            wDeviceID: f_wDeviceID,
            lpstrDeviceType: f_lpstrDeviceType,
            lpstrElementName: f_lpstrElementName,
            lpstrAlias: f_lpstrAlias,
            dwStyle: f_dwStyle,
            hWndParent: f_hWndParent,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.wDeviceID, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpstrDeviceType, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpstrElementName, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpstrAlias, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwStyle, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.hWndParent, &mut into[24..24 + 4]);
    }
    fn size() -> usize {
        28
    }
}
pub struct MCI_ANIM_OPEN_PARMSW {
    pub dwCallback: PtrRepr,
    pub wDeviceID: u32,
    pub lpstrDeviceType: PCWSTR,
    pub lpstrElementName: PCWSTR,
    pub lpstrAlias: PCWSTR,
    pub dwStyle: u32,
    pub hWndParent: super::super::Foundation::HWND,
}
impl ::core::marker::Copy for MCI_ANIM_OPEN_PARMSW {}
impl ::core::clone::Clone for MCI_ANIM_OPEN_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_ANIM_OPEN_PARMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_ANIM_OPEN_PARMSW")
            .field("dwCallback", &self.dwCallback)
            .field("wDeviceID", &self.wDeviceID)
            .field("lpstrDeviceType", &self.lpstrDeviceType)
            .field("lpstrElementName", &self.lpstrElementName)
            .field("lpstrAlias", &self.lpstrAlias)
            .field("dwStyle", &self.dwStyle)
            .field("hWndParent", &self.hWndParent)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_ANIM_OPEN_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.wDeviceID == other.wDeviceID
            && self.lpstrDeviceType == other.lpstrDeviceType
            && self.lpstrElementName == other.lpstrElementName
            && self.lpstrAlias == other.lpstrAlias
            && self.dwStyle == other.dwStyle
            && self.hWndParent == other.hWndParent
    }
}
impl ::core::cmp::Eq for MCI_ANIM_OPEN_PARMSW {}
impl FromIntoMemory for MCI_ANIM_OPEN_PARMSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_wDeviceID = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpstrDeviceType = <PCWSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpstrElementName = <PCWSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpstrAlias = <PCWSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwStyle = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_hWndParent =
            <super::super::Foundation::HWND as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        Self {
            dwCallback: f_dwCallback,
            wDeviceID: f_wDeviceID,
            lpstrDeviceType: f_lpstrDeviceType,
            lpstrElementName: f_lpstrElementName,
            lpstrAlias: f_lpstrAlias,
            dwStyle: f_dwStyle,
            hWndParent: f_hWndParent,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.wDeviceID, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpstrDeviceType, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpstrElementName, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpstrAlias, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwStyle, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.hWndParent, &mut into[24..24 + 4]);
    }
    fn size() -> usize {
        28
    }
}
pub const MCI_ANIM_OPEN_WS: i32 = 65536i32;
pub const MCI_ANIM_PLAY_FAST: i32 = 262144i32;
pub struct MCI_ANIM_PLAY_PARMS {
    pub dwCallback: PtrRepr,
    pub dwFrom: u32,
    pub dwTo: u32,
    pub dwSpeed: u32,
}
impl ::core::marker::Copy for MCI_ANIM_PLAY_PARMS {}
impl ::core::clone::Clone for MCI_ANIM_PLAY_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_ANIM_PLAY_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_ANIM_PLAY_PARMS")
            .field("dwCallback", &self.dwCallback)
            .field("dwFrom", &self.dwFrom)
            .field("dwTo", &self.dwTo)
            .field("dwSpeed", &self.dwSpeed)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_ANIM_PLAY_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.dwFrom == other.dwFrom
            && self.dwTo == other.dwTo
            && self.dwSpeed == other.dwSpeed
    }
}
impl ::core::cmp::Eq for MCI_ANIM_PLAY_PARMS {}
impl FromIntoMemory for MCI_ANIM_PLAY_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFrom = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwTo = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwSpeed = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwFrom: f_dwFrom,
            dwTo: f_dwTo,
            dwSpeed: f_dwSpeed,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFrom, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwTo, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwSpeed, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub const MCI_ANIM_PLAY_REVERSE: i32 = 131072i32;
pub const MCI_ANIM_PLAY_SCAN: i32 = 1048576i32;
pub const MCI_ANIM_PLAY_SLOW: i32 = 524288i32;
pub const MCI_ANIM_PLAY_SPEED: i32 = 65536i32;
pub const MCI_ANIM_PUT_DESTINATION: i32 = 262144i32;
pub const MCI_ANIM_PUT_SOURCE: i32 = 131072i32;
pub const MCI_ANIM_REALIZE_BKGD: i32 = 131072i32;
pub const MCI_ANIM_REALIZE_NORM: i32 = 65536i32;
pub const MCI_ANIM_RECT: i32 = 65536i32;
pub struct MCI_ANIM_RECT_PARMS {
    pub dwCallback: PtrRepr,
    pub rc: super::super::Foundation::RECT,
}
impl ::core::marker::Copy for MCI_ANIM_RECT_PARMS {}
impl ::core::clone::Clone for MCI_ANIM_RECT_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_ANIM_RECT_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_ANIM_RECT_PARMS")
            .field("dwCallback", &self.dwCallback)
            .field("rc", &self.rc)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_ANIM_RECT_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback && self.rc == other.rc
    }
}
impl ::core::cmp::Eq for MCI_ANIM_RECT_PARMS {}
impl FromIntoMemory for MCI_ANIM_RECT_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_rc = <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[4..4 + 16]);
        Self {
            dwCallback: f_dwCallback,
            rc: f_rc,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.rc, &mut into[4..4 + 16]);
    }
    fn size() -> usize {
        20
    }
}
pub const MCI_ANIM_STATUS_FORWARD: i32 = 16386i32;
pub const MCI_ANIM_STATUS_HPAL: i32 = 16388i32;
pub const MCI_ANIM_STATUS_HWND: i32 = 16387i32;
pub const MCI_ANIM_STATUS_SPEED: i32 = 16385i32;
pub const MCI_ANIM_STATUS_STRETCH: i32 = 16389i32;
pub const MCI_ANIM_STEP_FRAMES: i32 = 131072i32;
pub struct MCI_ANIM_STEP_PARMS {
    pub dwCallback: PtrRepr,
    pub dwFrames: u32,
}
impl ::core::marker::Copy for MCI_ANIM_STEP_PARMS {}
impl ::core::clone::Clone for MCI_ANIM_STEP_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_ANIM_STEP_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_ANIM_STEP_PARMS")
            .field("dwCallback", &self.dwCallback)
            .field("dwFrames", &self.dwFrames)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_ANIM_STEP_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback && self.dwFrames == other.dwFrames
    }
}
impl ::core::cmp::Eq for MCI_ANIM_STEP_PARMS {}
impl FromIntoMemory for MCI_ANIM_STEP_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFrames = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwFrames: f_dwFrames,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFrames, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub const MCI_ANIM_STEP_REVERSE: i32 = 65536i32;
pub const MCI_ANIM_UPDATE_HDC: i32 = 131072i32;
pub struct MCI_ANIM_UPDATE_PARMS {
    pub dwCallback: PtrRepr,
    pub rc: super::super::Foundation::RECT,
    pub hDC: super::super::Graphics::Gdi::HDC,
}
impl ::core::marker::Copy for MCI_ANIM_UPDATE_PARMS {}
impl ::core::clone::Clone for MCI_ANIM_UPDATE_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_ANIM_UPDATE_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_ANIM_UPDATE_PARMS")
            .field("dwCallback", &self.dwCallback)
            .field("rc", &self.rc)
            .field("hDC", &self.hDC)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_ANIM_UPDATE_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback && self.rc == other.rc && self.hDC == other.hDC
    }
}
impl ::core::cmp::Eq for MCI_ANIM_UPDATE_PARMS {}
impl FromIntoMemory for MCI_ANIM_UPDATE_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_rc = <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[4..4 + 16]);
        let f_hDC =
            <super::super::Graphics::Gdi::HDC as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            dwCallback: f_dwCallback,
            rc: f_rc,
            hDC: f_hDC,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.rc, &mut into[4..4 + 16]);
        FromIntoMemory::into_bytes(self.hDC, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub const MCI_ANIM_WHERE_DESTINATION: i32 = 262144i32;
pub const MCI_ANIM_WHERE_SOURCE: i32 = 131072i32;
pub const MCI_ANIM_WINDOW_DEFAULT: i32 = 0i32;
pub const MCI_ANIM_WINDOW_DISABLE_STRETCH: i32 = 2097152i32;
pub const MCI_ANIM_WINDOW_ENABLE_STRETCH: i32 = 1048576i32;
pub const MCI_ANIM_WINDOW_HWND: i32 = 65536i32;
pub struct MCI_ANIM_WINDOW_PARMSA {
    pub dwCallback: PtrRepr,
    pub hWnd: super::super::Foundation::HWND,
    pub nCmdShow: u32,
    pub lpstrText: PCSTR,
}
impl ::core::marker::Copy for MCI_ANIM_WINDOW_PARMSA {}
impl ::core::clone::Clone for MCI_ANIM_WINDOW_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_ANIM_WINDOW_PARMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_ANIM_WINDOW_PARMSA")
            .field("dwCallback", &self.dwCallback)
            .field("hWnd", &self.hWnd)
            .field("nCmdShow", &self.nCmdShow)
            .field("lpstrText", &self.lpstrText)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_ANIM_WINDOW_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.hWnd == other.hWnd
            && self.nCmdShow == other.nCmdShow
            && self.lpstrText == other.lpstrText
    }
}
impl ::core::cmp::Eq for MCI_ANIM_WINDOW_PARMSA {}
impl FromIntoMemory for MCI_ANIM_WINDOW_PARMSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_hWnd =
            <super::super::Foundation::HWND as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_nCmdShow = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpstrText = <PCSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            dwCallback: f_dwCallback,
            hWnd: f_hWnd,
            nCmdShow: f_nCmdShow,
            lpstrText: f_lpstrText,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.hWnd, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.nCmdShow, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpstrText, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct MCI_ANIM_WINDOW_PARMSW {
    pub dwCallback: PtrRepr,
    pub hWnd: super::super::Foundation::HWND,
    pub nCmdShow: u32,
    pub lpstrText: PCWSTR,
}
impl ::core::marker::Copy for MCI_ANIM_WINDOW_PARMSW {}
impl ::core::clone::Clone for MCI_ANIM_WINDOW_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_ANIM_WINDOW_PARMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_ANIM_WINDOW_PARMSW")
            .field("dwCallback", &self.dwCallback)
            .field("hWnd", &self.hWnd)
            .field("nCmdShow", &self.nCmdShow)
            .field("lpstrText", &self.lpstrText)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_ANIM_WINDOW_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.hWnd == other.hWnd
            && self.nCmdShow == other.nCmdShow
            && self.lpstrText == other.lpstrText
    }
}
impl ::core::cmp::Eq for MCI_ANIM_WINDOW_PARMSW {}
impl FromIntoMemory for MCI_ANIM_WINDOW_PARMSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_hWnd =
            <super::super::Foundation::HWND as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_nCmdShow = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpstrText = <PCWSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            dwCallback: f_dwCallback,
            hWnd: f_hWnd,
            nCmdShow: f_nCmdShow,
            lpstrText: f_lpstrText,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.hWnd, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.nCmdShow, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpstrText, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub const MCI_ANIM_WINDOW_STATE: i32 = 262144i32;
pub const MCI_ANIM_WINDOW_TEXT: i32 = 524288i32;
pub const MCI_AVI_SETVIDEO_DRAW_PROCEDURE: i32 = 32768i32;
pub const MCI_AVI_SETVIDEO_PALETTE_COLOR: i32 = 33024i32;
pub const MCI_AVI_SETVIDEO_PALETTE_HALFTONE: i32 = 65535i32;
pub const MCI_AVI_STATUS_AUDIO_BREAKS: i32 = 32771i32;
pub const MCI_AVI_STATUS_FRAMES_SKIPPED: i32 = 32769i32;
pub const MCI_AVI_STATUS_LAST_PLAY_SPEED: i32 = 32770i32;
pub const MCI_BREAK: u32 = 2065u32;
pub const MCI_BREAK_HWND: i32 = 512i32;
pub const MCI_BREAK_KEY: i32 = 256i32;
pub const MCI_BREAK_OFF: i32 = 1024i32;
pub struct MCI_BREAK_PARMS {
    pub dwCallback: PtrRepr,
    pub nVirtKey: i32,
    pub hwndBreak: super::super::Foundation::HWND,
}
impl ::core::marker::Copy for MCI_BREAK_PARMS {}
impl ::core::clone::Clone for MCI_BREAK_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_BREAK_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_BREAK_PARMS")
            .field("dwCallback", &self.dwCallback)
            .field("nVirtKey", &self.nVirtKey)
            .field("hwndBreak", &self.hwndBreak)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_BREAK_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.nVirtKey == other.nVirtKey
            && self.hwndBreak == other.hwndBreak
    }
}
impl ::core::cmp::Eq for MCI_BREAK_PARMS {}
impl FromIntoMemory for MCI_BREAK_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_nVirtKey = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_hwndBreak =
            <super::super::Foundation::HWND as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            dwCallback: f_dwCallback,
            nVirtKey: f_nVirtKey,
            hwndBreak: f_hwndBreak,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.nVirtKey, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.hwndBreak, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub const MCI_CAPTURE: u32 = 2160u32;
pub const MCI_CDA_STATUS_TYPE_TRACK: i32 = 16385i32;
pub const MCI_CDA_TRACK_AUDIO: u32 = 1088u32;
pub const MCI_CDA_TRACK_OTHER: u32 = 1089u32;
pub const MCI_CLOSE: u32 = 2052u32;
pub const MCI_CLOSE_DRIVER: u32 = 2050u32;
pub const MCI_COLONIZED3_RETURN: u32 = 131072u32;
pub const MCI_COLONIZED4_RETURN: u32 = 262144u32;
pub const MCI_COMMAND_HEAD: u32 = 0u32;
pub const MCI_CONFIGURE: u32 = 2170u32;
pub const MCI_CONSTANT: u32 = 8u32;
pub const MCI_COPY: u32 = 2130u32;
pub const MCI_CUE: u32 = 2096u32;
pub const MCI_CUT: u32 = 2129u32;
pub const MCI_DELETE: u32 = 2134u32;
pub const MCI_DEVTYPE_ANIMATION: u32 = 519u32;
pub const MCI_DEVTYPE_CD_AUDIO: u32 = 516u32;
pub const MCI_DEVTYPE_DAT: u32 = 517u32;
pub const MCI_DEVTYPE_DIGITAL_VIDEO: u32 = 520u32;
pub const MCI_DEVTYPE_FIRST: u32 = 513u32;
pub const MCI_DEVTYPE_FIRST_USER: u32 = 4096u32;
pub const MCI_DEVTYPE_LAST: u32 = 523u32;
pub const MCI_DEVTYPE_OTHER: u32 = 521u32;
pub const MCI_DEVTYPE_OVERLAY: u32 = 515u32;
pub const MCI_DEVTYPE_SCANNER: u32 = 518u32;
pub const MCI_DEVTYPE_SEQUENCER: u32 = 523u32;
pub const MCI_DEVTYPE_VCR: u32 = 513u32;
pub const MCI_DEVTYPE_VIDEODISC: u32 = 514u32;
pub const MCI_DEVTYPE_WAVEFORM_AUDIO: u32 = 522u32;
pub const MCI_DGV_CAPTURE_AS: i32 = 65536i32;
pub const MCI_DGV_CAPTURE_AT: i32 = 131072i32;
pub struct MCI_DGV_CAPTURE_PARMSA {
    pub dwCallback: PtrRepr,
    pub lpstrFileName: PSTR,
    pub rc: super::super::Foundation::RECT,
}
impl ::core::marker::Copy for MCI_DGV_CAPTURE_PARMSA {}
impl ::core::clone::Clone for MCI_DGV_CAPTURE_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_CAPTURE_PARMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_CAPTURE_PARMSA")
            .field("dwCallback", &self.dwCallback)
            .field("lpstrFileName", &self.lpstrFileName)
            .field("rc", &self.rc)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_CAPTURE_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.lpstrFileName == other.lpstrFileName
            && self.rc == other.rc
    }
}
impl ::core::cmp::Eq for MCI_DGV_CAPTURE_PARMSA {}
impl FromIntoMemory for MCI_DGV_CAPTURE_PARMSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpstrFileName = <PSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_rc = <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        Self {
            dwCallback: f_dwCallback,
            lpstrFileName: f_lpstrFileName,
            rc: f_rc,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpstrFileName, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.rc, &mut into[8..8 + 16]);
    }
    fn size() -> usize {
        24
    }
}
pub struct MCI_DGV_CAPTURE_PARMSW {
    pub dwCallback: PtrRepr,
    pub lpstrFileName: PWSTR,
    pub rc: super::super::Foundation::RECT,
}
impl ::core::marker::Copy for MCI_DGV_CAPTURE_PARMSW {}
impl ::core::clone::Clone for MCI_DGV_CAPTURE_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_CAPTURE_PARMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_CAPTURE_PARMSW")
            .field("dwCallback", &self.dwCallback)
            .field("lpstrFileName", &self.lpstrFileName)
            .field("rc", &self.rc)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_CAPTURE_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.lpstrFileName == other.lpstrFileName
            && self.rc == other.rc
    }
}
impl ::core::cmp::Eq for MCI_DGV_CAPTURE_PARMSW {}
impl FromIntoMemory for MCI_DGV_CAPTURE_PARMSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpstrFileName = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_rc = <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        Self {
            dwCallback: f_dwCallback,
            lpstrFileName: f_lpstrFileName,
            rc: f_rc,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpstrFileName, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.rc, &mut into[8..8 + 16]);
    }
    fn size() -> usize {
        24
    }
}
pub const MCI_DGV_COPY_AT: i32 = 65536i32;
pub const MCI_DGV_COPY_AUDIO_STREAM: i32 = 131072i32;
pub struct MCI_DGV_COPY_PARMS {
    pub dwCallback: PtrRepr,
    pub dwFrom: u32,
    pub dwTo: u32,
    pub rc: super::super::Foundation::RECT,
    pub dwAudioStream: u32,
    pub dwVideoStream: u32,
}
impl ::core::marker::Copy for MCI_DGV_COPY_PARMS {}
impl ::core::clone::Clone for MCI_DGV_COPY_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_COPY_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_COPY_PARMS")
            .field("dwCallback", &self.dwCallback)
            .field("dwFrom", &self.dwFrom)
            .field("dwTo", &self.dwTo)
            .field("rc", &self.rc)
            .field("dwAudioStream", &self.dwAudioStream)
            .field("dwVideoStream", &self.dwVideoStream)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_COPY_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.dwFrom == other.dwFrom
            && self.dwTo == other.dwTo
            && self.rc == other.rc
            && self.dwAudioStream == other.dwAudioStream
            && self.dwVideoStream == other.dwVideoStream
    }
}
impl ::core::cmp::Eq for MCI_DGV_COPY_PARMS {}
impl FromIntoMemory for MCI_DGV_COPY_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 36);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFrom = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwTo = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_rc =
            <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[12..12 + 16]);
        let f_dwAudioStream = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_dwVideoStream = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwFrom: f_dwFrom,
            dwTo: f_dwTo,
            rc: f_rc,
            dwAudioStream: f_dwAudioStream,
            dwVideoStream: f_dwVideoStream,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 36);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFrom, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwTo, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.rc, &mut into[12..12 + 16]);
        FromIntoMemory::into_bytes(self.dwAudioStream, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.dwVideoStream, &mut into[32..32 + 4]);
    }
    fn size() -> usize {
        36
    }
}
pub const MCI_DGV_COPY_VIDEO_STREAM: i32 = 262144i32;
pub const MCI_DGV_CUE_INPUT: i32 = 65536i32;
pub const MCI_DGV_CUE_NOSHOW: i32 = 262144i32;
pub const MCI_DGV_CUE_OUTPUT: i32 = 131072i32;
pub struct MCI_DGV_CUE_PARMS {
    pub dwCallback: PtrRepr,
    pub dwTo: u32,
}
impl ::core::marker::Copy for MCI_DGV_CUE_PARMS {}
impl ::core::clone::Clone for MCI_DGV_CUE_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_CUE_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_CUE_PARMS")
            .field("dwCallback", &self.dwCallback)
            .field("dwTo", &self.dwTo)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_CUE_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback && self.dwTo == other.dwTo
    }
}
impl ::core::cmp::Eq for MCI_DGV_CUE_PARMS {}
impl FromIntoMemory for MCI_DGV_CUE_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwTo = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwTo: f_dwTo,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwTo, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub const MCI_DGV_CUT_AT: i32 = 65536i32;
pub const MCI_DGV_CUT_AUDIO_STREAM: i32 = 131072i32;
pub struct MCI_DGV_CUT_PARMS {
    pub dwCallback: PtrRepr,
    pub dwFrom: u32,
    pub dwTo: u32,
    pub rc: super::super::Foundation::RECT,
    pub dwAudioStream: u32,
    pub dwVideoStream: u32,
}
impl ::core::marker::Copy for MCI_DGV_CUT_PARMS {}
impl ::core::clone::Clone for MCI_DGV_CUT_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_CUT_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_CUT_PARMS")
            .field("dwCallback", &self.dwCallback)
            .field("dwFrom", &self.dwFrom)
            .field("dwTo", &self.dwTo)
            .field("rc", &self.rc)
            .field("dwAudioStream", &self.dwAudioStream)
            .field("dwVideoStream", &self.dwVideoStream)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_CUT_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.dwFrom == other.dwFrom
            && self.dwTo == other.dwTo
            && self.rc == other.rc
            && self.dwAudioStream == other.dwAudioStream
            && self.dwVideoStream == other.dwVideoStream
    }
}
impl ::core::cmp::Eq for MCI_DGV_CUT_PARMS {}
impl FromIntoMemory for MCI_DGV_CUT_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 36);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFrom = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwTo = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_rc =
            <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[12..12 + 16]);
        let f_dwAudioStream = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_dwVideoStream = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwFrom: f_dwFrom,
            dwTo: f_dwTo,
            rc: f_rc,
            dwAudioStream: f_dwAudioStream,
            dwVideoStream: f_dwVideoStream,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 36);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFrom, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwTo, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.rc, &mut into[12..12 + 16]);
        FromIntoMemory::into_bytes(self.dwAudioStream, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.dwVideoStream, &mut into[32..32 + 4]);
    }
    fn size() -> usize {
        36
    }
}
pub const MCI_DGV_CUT_VIDEO_STREAM: i32 = 262144i32;
pub const MCI_DGV_DELETE_AT: i32 = 65536i32;
pub const MCI_DGV_DELETE_AUDIO_STREAM: i32 = 131072i32;
pub struct MCI_DGV_DELETE_PARMS {
    pub dwCallback: PtrRepr,
    pub dwFrom: u32,
    pub dwTo: u32,
    pub rc: super::super::Foundation::RECT,
    pub dwAudioStream: u32,
    pub dwVideoStream: u32,
}
impl ::core::marker::Copy for MCI_DGV_DELETE_PARMS {}
impl ::core::clone::Clone for MCI_DGV_DELETE_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_DELETE_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_DELETE_PARMS")
            .field("dwCallback", &self.dwCallback)
            .field("dwFrom", &self.dwFrom)
            .field("dwTo", &self.dwTo)
            .field("rc", &self.rc)
            .field("dwAudioStream", &self.dwAudioStream)
            .field("dwVideoStream", &self.dwVideoStream)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_DELETE_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.dwFrom == other.dwFrom
            && self.dwTo == other.dwTo
            && self.rc == other.rc
            && self.dwAudioStream == other.dwAudioStream
            && self.dwVideoStream == other.dwVideoStream
    }
}
impl ::core::cmp::Eq for MCI_DGV_DELETE_PARMS {}
impl FromIntoMemory for MCI_DGV_DELETE_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 36);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFrom = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwTo = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_rc =
            <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[12..12 + 16]);
        let f_dwAudioStream = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_dwVideoStream = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwFrom: f_dwFrom,
            dwTo: f_dwTo,
            rc: f_rc,
            dwAudioStream: f_dwAudioStream,
            dwVideoStream: f_dwVideoStream,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 36);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFrom, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwTo, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.rc, &mut into[12..12 + 16]);
        FromIntoMemory::into_bytes(self.dwAudioStream, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.dwVideoStream, &mut into[32..32 + 4]);
    }
    fn size() -> usize {
        36
    }
}
pub const MCI_DGV_DELETE_VIDEO_STREAM: i32 = 262144i32;
pub const MCI_DGV_FF_AVI: i32 = 16385i32;
pub const MCI_DGV_FF_AVSS: i32 = 16384i32;
pub const MCI_DGV_FF_DIB: i32 = 16386i32;
pub const MCI_DGV_FF_JFIF: i32 = 16390i32;
pub const MCI_DGV_FF_JPEG: i32 = 16388i32;
pub const MCI_DGV_FF_MPEG: i32 = 16391i32;
pub const MCI_DGV_FF_RDIB: i32 = 16387i32;
pub const MCI_DGV_FF_RJPEG: i32 = 16389i32;
pub const MCI_DGV_FILE_MODE_EDITING: u32 = 3u32;
pub const MCI_DGV_FILE_MODE_EDITING_S: i32 = 32774i32;
pub const MCI_DGV_FILE_MODE_IDLE: u32 = 4u32;
pub const MCI_DGV_FILE_MODE_IDLE_S: i32 = 32775i32;
pub const MCI_DGV_FILE_MODE_LOADING: u32 = 2u32;
pub const MCI_DGV_FILE_MODE_LOADING_S: i32 = 32773i32;
pub const MCI_DGV_FILE_MODE_SAVING: u32 = 1u32;
pub const MCI_DGV_FILE_MODE_SAVING_S: i32 = 32772i32;
pub const MCI_DGV_FILE_S: i32 = 32770i32;
pub const MCI_DGV_FREEZE_AT: i32 = 65536i32;
pub const MCI_DGV_FREEZE_OUTSIDE: i32 = 131072i32;
pub const MCI_DGV_GETDEVCAPS_CAN_FREEZE: i32 = 16386i32;
pub const MCI_DGV_GETDEVCAPS_CAN_LOCK: i32 = 16384i32;
pub const MCI_DGV_GETDEVCAPS_CAN_REVERSE: i32 = 16388i32;
pub const MCI_DGV_GETDEVCAPS_CAN_STRETCH: i32 = 16385i32;
pub const MCI_DGV_GETDEVCAPS_CAN_STR_IN: i32 = 16392i32;
pub const MCI_DGV_GETDEVCAPS_CAN_TEST: i32 = 16393i32;
pub const MCI_DGV_GETDEVCAPS_HAS_STILL: i32 = 16389i32;
pub const MCI_DGV_GETDEVCAPS_MAXIMUM_RATE: i32 = 16394i32;
pub const MCI_DGV_GETDEVCAPS_MAX_WINDOWS: i32 = 16387i32;
pub const MCI_DGV_GETDEVCAPS_MINIMUM_RATE: i32 = 16395i32;
pub const MCI_DGV_GETDEVCAPS_PALETTES: i32 = 16390i32;
pub const MCI_DGV_INFO_AUDIO_ALG: i32 = 16388i32;
pub const MCI_DGV_INFO_AUDIO_QUALITY: i32 = 16385i32;
pub const MCI_DGV_INFO_ITEM: i32 = 131072i32;
pub struct MCI_DGV_INFO_PARMSA {
    pub dwCallback: PtrRepr,
    pub lpstrReturn: PSTR,
    pub dwRetSize: u32,
    pub dwItem: u32,
}
impl ::core::marker::Copy for MCI_DGV_INFO_PARMSA {}
impl ::core::clone::Clone for MCI_DGV_INFO_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_INFO_PARMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_INFO_PARMSA")
            .field("dwCallback", &self.dwCallback)
            .field("lpstrReturn", &self.lpstrReturn)
            .field("dwRetSize", &self.dwRetSize)
            .field("dwItem", &self.dwItem)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_INFO_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.lpstrReturn == other.lpstrReturn
            && self.dwRetSize == other.dwRetSize
            && self.dwItem == other.dwItem
    }
}
impl ::core::cmp::Eq for MCI_DGV_INFO_PARMSA {}
impl FromIntoMemory for MCI_DGV_INFO_PARMSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpstrReturn = <PSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwRetSize = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwItem = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            dwCallback: f_dwCallback,
            lpstrReturn: f_lpstrReturn,
            dwRetSize: f_dwRetSize,
            dwItem: f_dwItem,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpstrReturn, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwRetSize, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwItem, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct MCI_DGV_INFO_PARMSW {
    pub dwCallback: PtrRepr,
    pub lpstrReturn: PWSTR,
    pub dwRetSize: u32,
    pub dwItem: u32,
}
impl ::core::marker::Copy for MCI_DGV_INFO_PARMSW {}
impl ::core::clone::Clone for MCI_DGV_INFO_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_INFO_PARMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_INFO_PARMSW")
            .field("dwCallback", &self.dwCallback)
            .field("lpstrReturn", &self.lpstrReturn)
            .field("dwRetSize", &self.dwRetSize)
            .field("dwItem", &self.dwItem)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_INFO_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.lpstrReturn == other.lpstrReturn
            && self.dwRetSize == other.dwRetSize
            && self.dwItem == other.dwItem
    }
}
impl ::core::cmp::Eq for MCI_DGV_INFO_PARMSW {}
impl FromIntoMemory for MCI_DGV_INFO_PARMSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpstrReturn = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwRetSize = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwItem = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            dwCallback: f_dwCallback,
            lpstrReturn: f_lpstrReturn,
            dwRetSize: f_dwRetSize,
            dwItem: f_dwItem,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpstrReturn, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwRetSize, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwItem, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub const MCI_DGV_INFO_STILL_ALG: i32 = 16389i32;
pub const MCI_DGV_INFO_STILL_QUALITY: i32 = 16386i32;
pub const MCI_DGV_INFO_TEXT: i32 = 65536i32;
pub const MCI_DGV_INFO_USAGE: i32 = 16384i32;
pub const MCI_DGV_INFO_VIDEO_ALG: i32 = 16390i32;
pub const MCI_DGV_INFO_VIDEO_QUALITY: i32 = 16387i32;
pub const MCI_DGV_INPUT_S: i32 = 32771i32;
pub const MCI_DGV_LIST_ALG: i32 = 524288i32;
pub const MCI_DGV_LIST_AUDIO_ALG: i32 = 16384i32;
pub const MCI_DGV_LIST_AUDIO_QUALITY: i32 = 16385i32;
pub const MCI_DGV_LIST_AUDIO_STREAM: i32 = 16386i32;
pub const MCI_DGV_LIST_COUNT: i32 = 131072i32;
pub const MCI_DGV_LIST_ITEM: i32 = 65536i32;
pub const MCI_DGV_LIST_NUMBER: i32 = 262144i32;
pub struct MCI_DGV_LIST_PARMSA {
    pub dwCallback: PtrRepr,
    pub lpstrReturn: PSTR,
    pub dwLength: u32,
    pub dwNumber: u32,
    pub dwItem: u32,
    pub lpstrAlgorithm: PSTR,
}
impl ::core::marker::Copy for MCI_DGV_LIST_PARMSA {}
impl ::core::clone::Clone for MCI_DGV_LIST_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_LIST_PARMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_LIST_PARMSA")
            .field("dwCallback", &self.dwCallback)
            .field("lpstrReturn", &self.lpstrReturn)
            .field("dwLength", &self.dwLength)
            .field("dwNumber", &self.dwNumber)
            .field("dwItem", &self.dwItem)
            .field("lpstrAlgorithm", &self.lpstrAlgorithm)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_LIST_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.lpstrReturn == other.lpstrReturn
            && self.dwLength == other.dwLength
            && self.dwNumber == other.dwNumber
            && self.dwItem == other.dwItem
            && self.lpstrAlgorithm == other.lpstrAlgorithm
    }
}
impl ::core::cmp::Eq for MCI_DGV_LIST_PARMSA {}
impl FromIntoMemory for MCI_DGV_LIST_PARMSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpstrReturn = <PSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwLength = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwNumber = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwItem = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_lpstrAlgorithm = <PSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            dwCallback: f_dwCallback,
            lpstrReturn: f_lpstrReturn,
            dwLength: f_dwLength,
            dwNumber: f_dwNumber,
            dwItem: f_dwItem,
            lpstrAlgorithm: f_lpstrAlgorithm,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpstrReturn, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwLength, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwNumber, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwItem, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.lpstrAlgorithm, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct MCI_DGV_LIST_PARMSW {
    pub dwCallback: PtrRepr,
    pub lpstrReturn: PWSTR,
    pub dwLength: u32,
    pub dwNumber: u32,
    pub dwItem: u32,
    pub lpstrAlgorithm: PWSTR,
}
impl ::core::marker::Copy for MCI_DGV_LIST_PARMSW {}
impl ::core::clone::Clone for MCI_DGV_LIST_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_LIST_PARMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_LIST_PARMSW")
            .field("dwCallback", &self.dwCallback)
            .field("lpstrReturn", &self.lpstrReturn)
            .field("dwLength", &self.dwLength)
            .field("dwNumber", &self.dwNumber)
            .field("dwItem", &self.dwItem)
            .field("lpstrAlgorithm", &self.lpstrAlgorithm)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_LIST_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.lpstrReturn == other.lpstrReturn
            && self.dwLength == other.dwLength
            && self.dwNumber == other.dwNumber
            && self.dwItem == other.dwItem
            && self.lpstrAlgorithm == other.lpstrAlgorithm
    }
}
impl ::core::cmp::Eq for MCI_DGV_LIST_PARMSW {}
impl FromIntoMemory for MCI_DGV_LIST_PARMSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpstrReturn = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwLength = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwNumber = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwItem = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_lpstrAlgorithm = <PWSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            dwCallback: f_dwCallback,
            lpstrReturn: f_lpstrReturn,
            dwLength: f_dwLength,
            dwNumber: f_dwNumber,
            dwItem: f_dwItem,
            lpstrAlgorithm: f_lpstrAlgorithm,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpstrReturn, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwLength, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwNumber, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwItem, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.lpstrAlgorithm, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub const MCI_DGV_LIST_STILL_ALG: i32 = 16387i32;
pub const MCI_DGV_LIST_STILL_QUALITY: i32 = 16388i32;
pub const MCI_DGV_LIST_VIDEO_ALG: i32 = 16389i32;
pub const MCI_DGV_LIST_VIDEO_QUALITY: i32 = 16390i32;
pub const MCI_DGV_LIST_VIDEO_SOURCE: i32 = 16392i32;
pub const MCI_DGV_LIST_VIDEO_STREAM: i32 = 16391i32;
pub const MCI_DGV_METHOD_DIRECT: i32 = 40962i32;
pub const MCI_DGV_METHOD_POST: i32 = 40961i32;
pub const MCI_DGV_METHOD_PRE: i32 = 40960i32;
pub const MCI_DGV_MONITOR_FILE: i32 = 16385i32;
pub const MCI_DGV_MONITOR_INPUT: i32 = 16384i32;
pub const MCI_DGV_MONITOR_METHOD: i32 = 65536i32;
pub struct MCI_DGV_MONITOR_PARMS {
    pub dwCallback: PtrRepr,
    pub dwSource: u32,
    pub dwMethod: u32,
}
impl ::core::marker::Copy for MCI_DGV_MONITOR_PARMS {}
impl ::core::clone::Clone for MCI_DGV_MONITOR_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_MONITOR_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_MONITOR_PARMS")
            .field("dwCallback", &self.dwCallback)
            .field("dwSource", &self.dwSource)
            .field("dwMethod", &self.dwMethod)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_MONITOR_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.dwSource == other.dwSource
            && self.dwMethod == other.dwMethod
    }
}
impl ::core::cmp::Eq for MCI_DGV_MONITOR_PARMS {}
impl FromIntoMemory for MCI_DGV_MONITOR_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwSource = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwMethod = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwSource: f_dwSource,
            dwMethod: f_dwMethod,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwSource, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwMethod, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub const MCI_DGV_MONITOR_SOURCE: i32 = 131072i32;
pub const MCI_DGV_OPEN_16BIT: i32 = 524288i32;
pub const MCI_DGV_OPEN_32BIT: i32 = 1048576i32;
pub const MCI_DGV_OPEN_NOSTATIC: i32 = 262144i32;
pub const MCI_DGV_OPEN_PARENT: i32 = 131072i32;
pub struct MCI_DGV_OPEN_PARMSA {
    pub dwCallback: PtrRepr,
    pub wDeviceID: u32,
    pub lpstrDeviceType: PSTR,
    pub lpstrElementName: PSTR,
    pub lpstrAlias: PSTR,
    pub dwStyle: u32,
    pub hWndParent: super::super::Foundation::HWND,
}
impl ::core::marker::Copy for MCI_DGV_OPEN_PARMSA {}
impl ::core::clone::Clone for MCI_DGV_OPEN_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_OPEN_PARMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_OPEN_PARMSA")
            .field("dwCallback", &self.dwCallback)
            .field("wDeviceID", &self.wDeviceID)
            .field("lpstrDeviceType", &self.lpstrDeviceType)
            .field("lpstrElementName", &self.lpstrElementName)
            .field("lpstrAlias", &self.lpstrAlias)
            .field("dwStyle", &self.dwStyle)
            .field("hWndParent", &self.hWndParent)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_OPEN_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.wDeviceID == other.wDeviceID
            && self.lpstrDeviceType == other.lpstrDeviceType
            && self.lpstrElementName == other.lpstrElementName
            && self.lpstrAlias == other.lpstrAlias
            && self.dwStyle == other.dwStyle
            && self.hWndParent == other.hWndParent
    }
}
impl ::core::cmp::Eq for MCI_DGV_OPEN_PARMSA {}
impl FromIntoMemory for MCI_DGV_OPEN_PARMSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_wDeviceID = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpstrDeviceType = <PSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpstrElementName = <PSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpstrAlias = <PSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwStyle = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_hWndParent =
            <super::super::Foundation::HWND as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        Self {
            dwCallback: f_dwCallback,
            wDeviceID: f_wDeviceID,
            lpstrDeviceType: f_lpstrDeviceType,
            lpstrElementName: f_lpstrElementName,
            lpstrAlias: f_lpstrAlias,
            dwStyle: f_dwStyle,
            hWndParent: f_hWndParent,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.wDeviceID, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpstrDeviceType, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpstrElementName, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpstrAlias, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwStyle, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.hWndParent, &mut into[24..24 + 4]);
    }
    fn size() -> usize {
        28
    }
}
pub struct MCI_DGV_OPEN_PARMSW {
    pub dwCallback: PtrRepr,
    pub wDeviceID: u32,
    pub lpstrDeviceType: PWSTR,
    pub lpstrElementName: PWSTR,
    pub lpstrAlias: PWSTR,
    pub dwStyle: u32,
    pub hWndParent: super::super::Foundation::HWND,
}
impl ::core::marker::Copy for MCI_DGV_OPEN_PARMSW {}
impl ::core::clone::Clone for MCI_DGV_OPEN_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_OPEN_PARMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_OPEN_PARMSW")
            .field("dwCallback", &self.dwCallback)
            .field("wDeviceID", &self.wDeviceID)
            .field("lpstrDeviceType", &self.lpstrDeviceType)
            .field("lpstrElementName", &self.lpstrElementName)
            .field("lpstrAlias", &self.lpstrAlias)
            .field("dwStyle", &self.dwStyle)
            .field("hWndParent", &self.hWndParent)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_OPEN_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.wDeviceID == other.wDeviceID
            && self.lpstrDeviceType == other.lpstrDeviceType
            && self.lpstrElementName == other.lpstrElementName
            && self.lpstrAlias == other.lpstrAlias
            && self.dwStyle == other.dwStyle
            && self.hWndParent == other.hWndParent
    }
}
impl ::core::cmp::Eq for MCI_DGV_OPEN_PARMSW {}
impl FromIntoMemory for MCI_DGV_OPEN_PARMSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_wDeviceID = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpstrDeviceType = <PWSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpstrElementName = <PWSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpstrAlias = <PWSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwStyle = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_hWndParent =
            <super::super::Foundation::HWND as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        Self {
            dwCallback: f_dwCallback,
            wDeviceID: f_wDeviceID,
            lpstrDeviceType: f_lpstrDeviceType,
            lpstrElementName: f_lpstrElementName,
            lpstrAlias: f_lpstrAlias,
            dwStyle: f_dwStyle,
            hWndParent: f_hWndParent,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.wDeviceID, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpstrDeviceType, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpstrElementName, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpstrAlias, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwStyle, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.hWndParent, &mut into[24..24 + 4]);
    }
    fn size() -> usize {
        28
    }
}
pub const MCI_DGV_OPEN_WS: i32 = 65536i32;
pub const MCI_DGV_PASTE_AT: i32 = 65536i32;
pub const MCI_DGV_PASTE_AUDIO_STREAM: i32 = 131072i32;
pub const MCI_DGV_PASTE_INSERT: i32 = 524288i32;
pub const MCI_DGV_PASTE_OVERWRITE: i32 = 1048576i32;
pub struct MCI_DGV_PASTE_PARMS {
    pub dwCallback: PtrRepr,
    pub dwTo: u32,
    pub rc: super::super::Foundation::RECT,
    pub dwAudioStream: u32,
    pub dwVideoStream: u32,
}
impl ::core::marker::Copy for MCI_DGV_PASTE_PARMS {}
impl ::core::clone::Clone for MCI_DGV_PASTE_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_PASTE_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_PASTE_PARMS")
            .field("dwCallback", &self.dwCallback)
            .field("dwTo", &self.dwTo)
            .field("rc", &self.rc)
            .field("dwAudioStream", &self.dwAudioStream)
            .field("dwVideoStream", &self.dwVideoStream)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_PASTE_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.dwTo == other.dwTo
            && self.rc == other.rc
            && self.dwAudioStream == other.dwAudioStream
            && self.dwVideoStream == other.dwVideoStream
    }
}
impl ::core::cmp::Eq for MCI_DGV_PASTE_PARMS {}
impl FromIntoMemory for MCI_DGV_PASTE_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwTo = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_rc = <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        let f_dwAudioStream = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_dwVideoStream = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwTo: f_dwTo,
            rc: f_rc,
            dwAudioStream: f_dwAudioStream,
            dwVideoStream: f_dwVideoStream,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwTo, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.rc, &mut into[8..8 + 16]);
        FromIntoMemory::into_bytes(self.dwAudioStream, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.dwVideoStream, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
pub const MCI_DGV_PASTE_VIDEO_STREAM: i32 = 262144i32;
pub const MCI_DGV_PLAY_REPEAT: i32 = 65536i32;
pub const MCI_DGV_PLAY_REVERSE: i32 = 131072i32;
pub const MCI_DGV_PUT_CLIENT: i32 = 4194304i32;
pub const MCI_DGV_PUT_DESTINATION: i32 = 262144i32;
pub const MCI_DGV_PUT_FRAME: i32 = 524288i32;
pub const MCI_DGV_PUT_SOURCE: i32 = 131072i32;
pub const MCI_DGV_PUT_VIDEO: i32 = 1048576i32;
pub const MCI_DGV_PUT_WINDOW: i32 = 2097152i32;
pub struct MCI_DGV_QUALITY_PARMSA {
    pub dwCallback: PtrRepr,
    pub dwItem: u32,
    pub lpstrName: PSTR,
    pub lpstrAlgorithm: u32,
    pub dwHandle: u32,
}
impl ::core::marker::Copy for MCI_DGV_QUALITY_PARMSA {}
impl ::core::clone::Clone for MCI_DGV_QUALITY_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_QUALITY_PARMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_QUALITY_PARMSA")
            .field("dwCallback", &self.dwCallback)
            .field("dwItem", &self.dwItem)
            .field("lpstrName", &self.lpstrName)
            .field("lpstrAlgorithm", &self.lpstrAlgorithm)
            .field("dwHandle", &self.dwHandle)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_QUALITY_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.dwItem == other.dwItem
            && self.lpstrName == other.lpstrName
            && self.lpstrAlgorithm == other.lpstrAlgorithm
            && self.dwHandle == other.dwHandle
    }
}
impl ::core::cmp::Eq for MCI_DGV_QUALITY_PARMSA {}
impl FromIntoMemory for MCI_DGV_QUALITY_PARMSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwItem = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpstrName = <PSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpstrAlgorithm = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwHandle = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwItem: f_dwItem,
            lpstrName: f_lpstrName,
            lpstrAlgorithm: f_lpstrAlgorithm,
            dwHandle: f_dwHandle,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwItem, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpstrName, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpstrAlgorithm, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwHandle, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub struct MCI_DGV_QUALITY_PARMSW {
    pub dwCallback: PtrRepr,
    pub dwItem: u32,
    pub lpstrName: PWSTR,
    pub lpstrAlgorithm: u32,
    pub dwHandle: u32,
}
impl ::core::marker::Copy for MCI_DGV_QUALITY_PARMSW {}
impl ::core::clone::Clone for MCI_DGV_QUALITY_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_QUALITY_PARMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_QUALITY_PARMSW")
            .field("dwCallback", &self.dwCallback)
            .field("dwItem", &self.dwItem)
            .field("lpstrName", &self.lpstrName)
            .field("lpstrAlgorithm", &self.lpstrAlgorithm)
            .field("dwHandle", &self.dwHandle)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_QUALITY_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.dwItem == other.dwItem
            && self.lpstrName == other.lpstrName
            && self.lpstrAlgorithm == other.lpstrAlgorithm
            && self.dwHandle == other.dwHandle
    }
}
impl ::core::cmp::Eq for MCI_DGV_QUALITY_PARMSW {}
impl FromIntoMemory for MCI_DGV_QUALITY_PARMSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwItem = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpstrName = <PWSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpstrAlgorithm = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwHandle = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwItem: f_dwItem,
            lpstrName: f_lpstrName,
            lpstrAlgorithm: f_lpstrAlgorithm,
            dwHandle: f_dwHandle,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwItem, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpstrName, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpstrAlgorithm, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwHandle, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub const MCI_DGV_REALIZE_BKGD: i32 = 131072i32;
pub const MCI_DGV_REALIZE_NORM: i32 = 65536i32;
pub const MCI_DGV_RECORD_AUDIO_STREAM: i32 = 262144i32;
pub const MCI_DGV_RECORD_HOLD: i32 = 131072i32;
pub struct MCI_DGV_RECORD_PARMS {
    pub dwCallback: PtrRepr,
    pub dwFrom: u32,
    pub dwTo: u32,
    pub rc: super::super::Foundation::RECT,
    pub dwAudioStream: u32,
    pub dwVideoStream: u32,
}
impl ::core::marker::Copy for MCI_DGV_RECORD_PARMS {}
impl ::core::clone::Clone for MCI_DGV_RECORD_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_RECORD_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_RECORD_PARMS")
            .field("dwCallback", &self.dwCallback)
            .field("dwFrom", &self.dwFrom)
            .field("dwTo", &self.dwTo)
            .field("rc", &self.rc)
            .field("dwAudioStream", &self.dwAudioStream)
            .field("dwVideoStream", &self.dwVideoStream)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_RECORD_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.dwFrom == other.dwFrom
            && self.dwTo == other.dwTo
            && self.rc == other.rc
            && self.dwAudioStream == other.dwAudioStream
            && self.dwVideoStream == other.dwVideoStream
    }
}
impl ::core::cmp::Eq for MCI_DGV_RECORD_PARMS {}
impl FromIntoMemory for MCI_DGV_RECORD_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 36);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFrom = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwTo = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_rc =
            <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[12..12 + 16]);
        let f_dwAudioStream = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_dwVideoStream = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwFrom: f_dwFrom,
            dwTo: f_dwTo,
            rc: f_rc,
            dwAudioStream: f_dwAudioStream,
            dwVideoStream: f_dwVideoStream,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 36);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFrom, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwTo, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.rc, &mut into[12..12 + 16]);
        FromIntoMemory::into_bytes(self.dwAudioStream, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.dwVideoStream, &mut into[32..32 + 4]);
    }
    fn size() -> usize {
        36
    }
}
pub const MCI_DGV_RECORD_VIDEO_STREAM: i32 = 524288i32;
pub const MCI_DGV_RECT: i32 = 65536i32;
pub struct MCI_DGV_RECT_PARMS {
    pub dwCallback: PtrRepr,
    pub rc: super::super::Foundation::RECT,
}
impl ::core::marker::Copy for MCI_DGV_RECT_PARMS {}
impl ::core::clone::Clone for MCI_DGV_RECT_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_RECT_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_RECT_PARMS")
            .field("dwCallback", &self.dwCallback)
            .field("rc", &self.rc)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_RECT_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback && self.rc == other.rc
    }
}
impl ::core::cmp::Eq for MCI_DGV_RECT_PARMS {}
impl FromIntoMemory for MCI_DGV_RECT_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_rc = <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[4..4 + 16]);
        Self {
            dwCallback: f_dwCallback,
            rc: f_rc,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.rc, &mut into[4..4 + 16]);
    }
    fn size() -> usize {
        20
    }
}
pub const MCI_DGV_RESERVE_IN: i32 = 65536i32;
pub struct MCI_DGV_RESERVE_PARMSA {
    pub dwCallback: PtrRepr,
    pub lpstrPath: PSTR,
    pub dwSize: u32,
}
impl ::core::marker::Copy for MCI_DGV_RESERVE_PARMSA {}
impl ::core::clone::Clone for MCI_DGV_RESERVE_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_RESERVE_PARMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_RESERVE_PARMSA")
            .field("dwCallback", &self.dwCallback)
            .field("lpstrPath", &self.lpstrPath)
            .field("dwSize", &self.dwSize)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_RESERVE_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.lpstrPath == other.lpstrPath
            && self.dwSize == other.dwSize
    }
}
impl ::core::cmp::Eq for MCI_DGV_RESERVE_PARMSA {}
impl FromIntoMemory for MCI_DGV_RESERVE_PARMSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpstrPath = <PSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            dwCallback: f_dwCallback,
            lpstrPath: f_lpstrPath,
            dwSize: f_dwSize,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpstrPath, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct MCI_DGV_RESERVE_PARMSW {
    pub dwCallback: PtrRepr,
    pub lpstrPath: PWSTR,
    pub dwSize: u32,
}
impl ::core::marker::Copy for MCI_DGV_RESERVE_PARMSW {}
impl ::core::clone::Clone for MCI_DGV_RESERVE_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_RESERVE_PARMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_RESERVE_PARMSW")
            .field("dwCallback", &self.dwCallback)
            .field("lpstrPath", &self.lpstrPath)
            .field("dwSize", &self.dwSize)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_RESERVE_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.lpstrPath == other.lpstrPath
            && self.dwSize == other.dwSize
    }
}
impl ::core::cmp::Eq for MCI_DGV_RESERVE_PARMSW {}
impl FromIntoMemory for MCI_DGV_RESERVE_PARMSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpstrPath = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            dwCallback: f_dwCallback,
            lpstrPath: f_lpstrPath,
            dwSize: f_dwSize,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpstrPath, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub const MCI_DGV_RESERVE_SIZE: i32 = 131072i32;
pub const MCI_DGV_RESTORE_AT: i32 = 131072i32;
pub const MCI_DGV_RESTORE_FROM: i32 = 65536i32;
pub struct MCI_DGV_RESTORE_PARMSA {
    pub dwCallback: PtrRepr,
    pub lpstrFileName: PSTR,
    pub rc: super::super::Foundation::RECT,
}
impl ::core::marker::Copy for MCI_DGV_RESTORE_PARMSA {}
impl ::core::clone::Clone for MCI_DGV_RESTORE_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_RESTORE_PARMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_RESTORE_PARMSA")
            .field("dwCallback", &self.dwCallback)
            .field("lpstrFileName", &self.lpstrFileName)
            .field("rc", &self.rc)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_RESTORE_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.lpstrFileName == other.lpstrFileName
            && self.rc == other.rc
    }
}
impl ::core::cmp::Eq for MCI_DGV_RESTORE_PARMSA {}
impl FromIntoMemory for MCI_DGV_RESTORE_PARMSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpstrFileName = <PSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_rc = <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        Self {
            dwCallback: f_dwCallback,
            lpstrFileName: f_lpstrFileName,
            rc: f_rc,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpstrFileName, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.rc, &mut into[8..8 + 16]);
    }
    fn size() -> usize {
        24
    }
}
pub struct MCI_DGV_RESTORE_PARMSW {
    pub dwCallback: PtrRepr,
    pub lpstrFileName: PWSTR,
    pub rc: super::super::Foundation::RECT,
}
impl ::core::marker::Copy for MCI_DGV_RESTORE_PARMSW {}
impl ::core::clone::Clone for MCI_DGV_RESTORE_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_RESTORE_PARMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_RESTORE_PARMSW")
            .field("dwCallback", &self.dwCallback)
            .field("lpstrFileName", &self.lpstrFileName)
            .field("rc", &self.rc)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_RESTORE_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.lpstrFileName == other.lpstrFileName
            && self.rc == other.rc
    }
}
impl ::core::cmp::Eq for MCI_DGV_RESTORE_PARMSW {}
impl FromIntoMemory for MCI_DGV_RESTORE_PARMSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpstrFileName = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_rc = <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        Self {
            dwCallback: f_dwCallback,
            lpstrFileName: f_lpstrFileName,
            rc: f_rc,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpstrFileName, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.rc, &mut into[8..8 + 16]);
    }
    fn size() -> usize {
        24
    }
}
pub const MCI_DGV_SAVE_ABORT: i32 = 131072i32;
pub const MCI_DGV_SAVE_KEEPRESERVE: i32 = 262144i32;
pub struct MCI_DGV_SAVE_PARMSA {
    pub dwCallback: PtrRepr,
    pub lpstrFileName: PSTR,
    pub rc: super::super::Foundation::RECT,
}
impl ::core::marker::Copy for MCI_DGV_SAVE_PARMSA {}
impl ::core::clone::Clone for MCI_DGV_SAVE_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_SAVE_PARMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_SAVE_PARMSA")
            .field("dwCallback", &self.dwCallback)
            .field("lpstrFileName", &self.lpstrFileName)
            .field("rc", &self.rc)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_SAVE_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.lpstrFileName == other.lpstrFileName
            && self.rc == other.rc
    }
}
impl ::core::cmp::Eq for MCI_DGV_SAVE_PARMSA {}
impl FromIntoMemory for MCI_DGV_SAVE_PARMSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpstrFileName = <PSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_rc = <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        Self {
            dwCallback: f_dwCallback,
            lpstrFileName: f_lpstrFileName,
            rc: f_rc,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpstrFileName, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.rc, &mut into[8..8 + 16]);
    }
    fn size() -> usize {
        24
    }
}
pub struct MCI_DGV_SAVE_PARMSW {
    pub dwCallback: PtrRepr,
    pub lpstrFileName: PWSTR,
    pub rc: super::super::Foundation::RECT,
}
impl ::core::marker::Copy for MCI_DGV_SAVE_PARMSW {}
impl ::core::clone::Clone for MCI_DGV_SAVE_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_SAVE_PARMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_SAVE_PARMSW")
            .field("dwCallback", &self.dwCallback)
            .field("lpstrFileName", &self.lpstrFileName)
            .field("rc", &self.rc)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_SAVE_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.lpstrFileName == other.lpstrFileName
            && self.rc == other.rc
    }
}
impl ::core::cmp::Eq for MCI_DGV_SAVE_PARMSW {}
impl FromIntoMemory for MCI_DGV_SAVE_PARMSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpstrFileName = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_rc = <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        Self {
            dwCallback: f_dwCallback,
            lpstrFileName: f_lpstrFileName,
            rc: f_rc,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpstrFileName, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.rc, &mut into[8..8 + 16]);
    }
    fn size() -> usize {
        24
    }
}
pub const MCI_DGV_SETAUDIO_ALG: i32 = 262144i32;
pub const MCI_DGV_SETAUDIO_AVGBYTESPERSEC: i32 = 16390i32;
pub const MCI_DGV_SETAUDIO_BASS: i32 = 16385i32;
pub const MCI_DGV_SETAUDIO_BITSPERSAMPLE: i32 = 16392i32;
pub const MCI_DGV_SETAUDIO_BLOCKALIGN: i32 = 16391i32;
pub const MCI_DGV_SETAUDIO_CLOCKTIME: i32 = 131072i32;
pub const MCI_DGV_SETAUDIO_INPUT: i32 = 33554432i32;
pub const MCI_DGV_SETAUDIO_ITEM: i32 = 8388608i32;
pub const MCI_DGV_SETAUDIO_LEFT: i32 = 2097152i32;
pub const MCI_DGV_SETAUDIO_OUTPUT: i32 = 67108864i32;
pub const MCI_DGV_SETAUDIO_OVER: i32 = 65536i32;
pub struct MCI_DGV_SETAUDIO_PARMSA {
    pub dwCallback: PtrRepr,
    pub dwItem: u32,
    pub dwValue: u32,
    pub dwOver: u32,
    pub lpstrAlgorithm: PSTR,
    pub lpstrQuality: PSTR,
}
impl ::core::marker::Copy for MCI_DGV_SETAUDIO_PARMSA {}
impl ::core::clone::Clone for MCI_DGV_SETAUDIO_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_SETAUDIO_PARMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_SETAUDIO_PARMSA")
            .field("dwCallback", &self.dwCallback)
            .field("dwItem", &self.dwItem)
            .field("dwValue", &self.dwValue)
            .field("dwOver", &self.dwOver)
            .field("lpstrAlgorithm", &self.lpstrAlgorithm)
            .field("lpstrQuality", &self.lpstrQuality)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_SETAUDIO_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.dwItem == other.dwItem
            && self.dwValue == other.dwValue
            && self.dwOver == other.dwOver
            && self.lpstrAlgorithm == other.lpstrAlgorithm
            && self.lpstrQuality == other.lpstrQuality
    }
}
impl ::core::cmp::Eq for MCI_DGV_SETAUDIO_PARMSA {}
impl FromIntoMemory for MCI_DGV_SETAUDIO_PARMSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwItem = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwValue = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwOver = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpstrAlgorithm = <PSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_lpstrQuality = <PSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwItem: f_dwItem,
            dwValue: f_dwValue,
            dwOver: f_dwOver,
            lpstrAlgorithm: f_lpstrAlgorithm,
            lpstrQuality: f_lpstrQuality,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwItem, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwValue, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwOver, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpstrAlgorithm, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.lpstrQuality, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct MCI_DGV_SETAUDIO_PARMSW {
    pub dwCallback: PtrRepr,
    pub dwItem: u32,
    pub dwValue: u32,
    pub dwOver: u32,
    pub lpstrAlgorithm: PWSTR,
    pub lpstrQuality: PWSTR,
}
impl ::core::marker::Copy for MCI_DGV_SETAUDIO_PARMSW {}
impl ::core::clone::Clone for MCI_DGV_SETAUDIO_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_SETAUDIO_PARMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_SETAUDIO_PARMSW")
            .field("dwCallback", &self.dwCallback)
            .field("dwItem", &self.dwItem)
            .field("dwValue", &self.dwValue)
            .field("dwOver", &self.dwOver)
            .field("lpstrAlgorithm", &self.lpstrAlgorithm)
            .field("lpstrQuality", &self.lpstrQuality)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_SETAUDIO_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.dwItem == other.dwItem
            && self.dwValue == other.dwValue
            && self.dwOver == other.dwOver
            && self.lpstrAlgorithm == other.lpstrAlgorithm
            && self.lpstrQuality == other.lpstrQuality
    }
}
impl ::core::cmp::Eq for MCI_DGV_SETAUDIO_PARMSW {}
impl FromIntoMemory for MCI_DGV_SETAUDIO_PARMSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwItem = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwValue = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwOver = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpstrAlgorithm = <PWSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_lpstrQuality = <PWSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwItem: f_dwItem,
            dwValue: f_dwValue,
            dwOver: f_dwOver,
            lpstrAlgorithm: f_lpstrAlgorithm,
            lpstrQuality: f_lpstrQuality,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwItem, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwValue, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwOver, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpstrAlgorithm, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.lpstrQuality, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub const MCI_DGV_SETAUDIO_QUALITY: i32 = 524288i32;
pub const MCI_DGV_SETAUDIO_RECORD: i32 = 1048576i32;
pub const MCI_DGV_SETAUDIO_RIGHT: i32 = 4194304i32;
pub const MCI_DGV_SETAUDIO_SAMPLESPERSEC: i32 = 16389i32;
pub const MCI_DGV_SETAUDIO_SOURCE: i32 = 16388i32;
pub const MCI_DGV_SETAUDIO_SOURCE_AVERAGE: i32 = 16384i32;
pub const MCI_DGV_SETAUDIO_SOURCE_LEFT: i32 = 1i32;
pub const MCI_DGV_SETAUDIO_SOURCE_RIGHT: i32 = 2i32;
pub const MCI_DGV_SETAUDIO_SOURCE_STEREO: i32 = 0i32;
pub const MCI_DGV_SETAUDIO_SRC_AVERAGE_S: i32 = 32802i32;
pub const MCI_DGV_SETAUDIO_SRC_LEFT_S: i32 = 32800i32;
pub const MCI_DGV_SETAUDIO_SRC_RIGHT_S: i32 = 32801i32;
pub const MCI_DGV_SETAUDIO_SRC_STEREO_S: i32 = 32803i32;
pub const MCI_DGV_SETAUDIO_STREAM: i32 = 16387i32;
pub const MCI_DGV_SETAUDIO_TREBLE: i32 = 16384i32;
pub const MCI_DGV_SETAUDIO_VALUE: i32 = 16777216i32;
pub const MCI_DGV_SETAUDIO_VOLUME: i32 = 16386i32;
pub const MCI_DGV_SETVIDEO_ALG: i32 = 131072i32;
pub const MCI_DGV_SETVIDEO_BITSPERPEL: i32 = 16396i32;
pub const MCI_DGV_SETVIDEO_BRIGHTNESS: i32 = 16384i32;
pub const MCI_DGV_SETVIDEO_CLOCKTIME: i32 = 262144i32;
pub const MCI_DGV_SETVIDEO_COLOR: i32 = 16385i32;
pub const MCI_DGV_SETVIDEO_CONTRAST: i32 = 16386i32;
pub const MCI_DGV_SETVIDEO_FRAME_RATE: i32 = 16392i32;
pub const MCI_DGV_SETVIDEO_GAMMA: i32 = 16389i32;
pub const MCI_DGV_SETVIDEO_INPUT: i32 = 33554432i32;
pub const MCI_DGV_SETVIDEO_ITEM: i32 = 1048576i32;
pub const MCI_DGV_SETVIDEO_KEY_COLOR: i32 = 16395i32;
pub const MCI_DGV_SETVIDEO_KEY_INDEX: i32 = 16394i32;
pub const MCI_DGV_SETVIDEO_OUTPUT: i32 = 67108864i32;
pub const MCI_DGV_SETVIDEO_OVER: i32 = 2097152i32;
pub const MCI_DGV_SETVIDEO_PALHANDLE: i32 = 16391i32;
pub struct MCI_DGV_SETVIDEO_PARMSA {
    pub dwCallback: PtrRepr,
    pub dwItem: u32,
    pub dwValue: u32,
    pub dwOver: u32,
    pub lpstrAlgorithm: PSTR,
    pub lpstrQuality: PSTR,
    pub dwSourceNumber: u32,
}
impl ::core::marker::Copy for MCI_DGV_SETVIDEO_PARMSA {}
impl ::core::clone::Clone for MCI_DGV_SETVIDEO_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_SETVIDEO_PARMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_SETVIDEO_PARMSA")
            .field("dwCallback", &self.dwCallback)
            .field("dwItem", &self.dwItem)
            .field("dwValue", &self.dwValue)
            .field("dwOver", &self.dwOver)
            .field("lpstrAlgorithm", &self.lpstrAlgorithm)
            .field("lpstrQuality", &self.lpstrQuality)
            .field("dwSourceNumber", &self.dwSourceNumber)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_SETVIDEO_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.dwItem == other.dwItem
            && self.dwValue == other.dwValue
            && self.dwOver == other.dwOver
            && self.lpstrAlgorithm == other.lpstrAlgorithm
            && self.lpstrQuality == other.lpstrQuality
            && self.dwSourceNumber == other.dwSourceNumber
    }
}
impl ::core::cmp::Eq for MCI_DGV_SETVIDEO_PARMSA {}
impl FromIntoMemory for MCI_DGV_SETVIDEO_PARMSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwItem = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwValue = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwOver = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpstrAlgorithm = <PSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_lpstrQuality = <PSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwSourceNumber = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwItem: f_dwItem,
            dwValue: f_dwValue,
            dwOver: f_dwOver,
            lpstrAlgorithm: f_lpstrAlgorithm,
            lpstrQuality: f_lpstrQuality,
            dwSourceNumber: f_dwSourceNumber,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwItem, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwValue, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwOver, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpstrAlgorithm, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.lpstrQuality, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwSourceNumber, &mut into[24..24 + 4]);
    }
    fn size() -> usize {
        28
    }
}
pub struct MCI_DGV_SETVIDEO_PARMSW {
    pub dwCallback: PtrRepr,
    pub dwItem: u32,
    pub dwValue: u32,
    pub dwOver: u32,
    pub lpstrAlgorithm: PWSTR,
    pub lpstrQuality: PWSTR,
    pub dwSourceNumber: u32,
}
impl ::core::marker::Copy for MCI_DGV_SETVIDEO_PARMSW {}
impl ::core::clone::Clone for MCI_DGV_SETVIDEO_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_SETVIDEO_PARMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_SETVIDEO_PARMSW")
            .field("dwCallback", &self.dwCallback)
            .field("dwItem", &self.dwItem)
            .field("dwValue", &self.dwValue)
            .field("dwOver", &self.dwOver)
            .field("lpstrAlgorithm", &self.lpstrAlgorithm)
            .field("lpstrQuality", &self.lpstrQuality)
            .field("dwSourceNumber", &self.dwSourceNumber)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_SETVIDEO_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.dwItem == other.dwItem
            && self.dwValue == other.dwValue
            && self.dwOver == other.dwOver
            && self.lpstrAlgorithm == other.lpstrAlgorithm
            && self.lpstrQuality == other.lpstrQuality
            && self.dwSourceNumber == other.dwSourceNumber
    }
}
impl ::core::cmp::Eq for MCI_DGV_SETVIDEO_PARMSW {}
impl FromIntoMemory for MCI_DGV_SETVIDEO_PARMSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwItem = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwValue = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwOver = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpstrAlgorithm = <PWSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_lpstrQuality = <PWSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwSourceNumber = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwItem: f_dwItem,
            dwValue: f_dwValue,
            dwOver: f_dwOver,
            lpstrAlgorithm: f_lpstrAlgorithm,
            lpstrQuality: f_lpstrQuality,
            dwSourceNumber: f_dwSourceNumber,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwItem, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwValue, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwOver, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpstrAlgorithm, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.lpstrQuality, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwSourceNumber, &mut into[24..24 + 4]);
    }
    fn size() -> usize {
        28
    }
}
pub const MCI_DGV_SETVIDEO_QUALITY: i32 = 65536i32;
pub const MCI_DGV_SETVIDEO_RECORD: i32 = 4194304i32;
pub const MCI_DGV_SETVIDEO_SHARPNESS: i32 = 16388i32;
pub const MCI_DGV_SETVIDEO_SOURCE: i32 = 16393i32;
pub const MCI_DGV_SETVIDEO_SRC_GENERIC: i32 = 16389i32;
pub const MCI_DGV_SETVIDEO_SRC_GENERIC_S: i32 = 32789i32;
pub const MCI_DGV_SETVIDEO_SRC_NTSC: i32 = 16384i32;
pub const MCI_DGV_SETVIDEO_SRC_NTSC_S: i32 = 32784i32;
pub const MCI_DGV_SETVIDEO_SRC_NUMBER: i32 = 524288i32;
pub const MCI_DGV_SETVIDEO_SRC_PAL: i32 = 16387i32;
pub const MCI_DGV_SETVIDEO_SRC_PAL_S: i32 = 32787i32;
pub const MCI_DGV_SETVIDEO_SRC_RGB: i32 = 16385i32;
pub const MCI_DGV_SETVIDEO_SRC_RGB_S: i32 = 32785i32;
pub const MCI_DGV_SETVIDEO_SRC_SECAM: i32 = 16388i32;
pub const MCI_DGV_SETVIDEO_SRC_SECAM_S: i32 = 32788i32;
pub const MCI_DGV_SETVIDEO_SRC_SVIDEO: i32 = 16386i32;
pub const MCI_DGV_SETVIDEO_SRC_SVIDEO_S: i32 = 32786i32;
pub const MCI_DGV_SETVIDEO_STILL: i32 = 8388608i32;
pub const MCI_DGV_SETVIDEO_STREAM: i32 = 16390i32;
pub const MCI_DGV_SETVIDEO_TINT: i32 = 16387i32;
pub const MCI_DGV_SETVIDEO_VALUE: i32 = 16777216i32;
pub const MCI_DGV_SET_FILEFORMAT: i32 = 524288i32;
pub struct MCI_DGV_SET_PARMS {
    pub dwCallback: PtrRepr,
    pub dwTimeFormat: u32,
    pub dwAudio: u32,
    pub dwFileFormat: u32,
    pub dwSpeed: u32,
}
impl ::core::marker::Copy for MCI_DGV_SET_PARMS {}
impl ::core::clone::Clone for MCI_DGV_SET_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_SET_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_SET_PARMS")
            .field("dwCallback", &self.dwCallback)
            .field("dwTimeFormat", &self.dwTimeFormat)
            .field("dwAudio", &self.dwAudio)
            .field("dwFileFormat", &self.dwFileFormat)
            .field("dwSpeed", &self.dwSpeed)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_SET_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.dwTimeFormat == other.dwTimeFormat
            && self.dwAudio == other.dwAudio
            && self.dwFileFormat == other.dwFileFormat
            && self.dwSpeed == other.dwSpeed
    }
}
impl ::core::cmp::Eq for MCI_DGV_SET_PARMS {}
impl FromIntoMemory for MCI_DGV_SET_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwTimeFormat = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwAudio = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwFileFormat = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwSpeed = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwTimeFormat: f_dwTimeFormat,
            dwAudio: f_dwAudio,
            dwFileFormat: f_dwFileFormat,
            dwSpeed: f_dwSpeed,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwTimeFormat, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwAudio, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwFileFormat, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwSpeed, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub const MCI_DGV_SET_SEEK_EXACTLY: i32 = 65536i32;
pub const MCI_DGV_SET_SPEED: i32 = 131072i32;
pub const MCI_DGV_SET_STILL: i32 = 262144i32;
pub const MCI_DGV_SIGNAL_AT: i32 = 65536i32;
pub const MCI_DGV_SIGNAL_CANCEL: i32 = 524288i32;
pub const MCI_DGV_SIGNAL_EVERY: i32 = 131072i32;
pub struct MCI_DGV_SIGNAL_PARMS {
    pub dwCallback: PtrRepr,
    pub dwPosition: u32,
    pub dwPeriod: u32,
    pub dwUserParm: u32,
}
impl ::core::marker::Copy for MCI_DGV_SIGNAL_PARMS {}
impl ::core::clone::Clone for MCI_DGV_SIGNAL_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_SIGNAL_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_SIGNAL_PARMS")
            .field("dwCallback", &self.dwCallback)
            .field("dwPosition", &self.dwPosition)
            .field("dwPeriod", &self.dwPeriod)
            .field("dwUserParm", &self.dwUserParm)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_SIGNAL_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.dwPosition == other.dwPosition
            && self.dwPeriod == other.dwPeriod
            && self.dwUserParm == other.dwUserParm
    }
}
impl ::core::cmp::Eq for MCI_DGV_SIGNAL_PARMS {}
impl FromIntoMemory for MCI_DGV_SIGNAL_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwPosition = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwPeriod = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwUserParm = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwPosition: f_dwPosition,
            dwPeriod: f_dwPeriod,
            dwUserParm: f_dwUserParm,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwPosition, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwPeriod, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwUserParm, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub const MCI_DGV_SIGNAL_POSITION: i32 = 1048576i32;
pub const MCI_DGV_SIGNAL_USERVAL: i32 = 262144i32;
pub const MCI_DGV_STATUS_AUDIO: i32 = 16404i32;
pub const MCI_DGV_STATUS_AUDIO_INPUT: i32 = 16384i32;
pub const MCI_DGV_STATUS_AUDIO_RECORD: i32 = 16410i32;
pub const MCI_DGV_STATUS_AUDIO_SOURCE: i32 = 16393i32;
pub const MCI_DGV_STATUS_AUDIO_STREAM: i32 = 16429i32;
pub const MCI_DGV_STATUS_AVGBYTESPERSEC: i32 = 16424i32;
pub const MCI_DGV_STATUS_BASS: i32 = 16399i32;
pub const MCI_DGV_STATUS_BITSPERPEL: i32 = 16427i32;
pub const MCI_DGV_STATUS_BITSPERSAMPLE: i32 = 16426i32;
pub const MCI_DGV_STATUS_BLOCKALIGN: i32 = 16425i32;
pub const MCI_DGV_STATUS_BRIGHTNESS: i32 = 16389i32;
pub const MCI_DGV_STATUS_COLOR: i32 = 16390i32;
pub const MCI_DGV_STATUS_CONTRAST: i32 = 16391i32;
pub const MCI_DGV_STATUS_DISKSPACE: i32 = 2097152i32;
pub const MCI_DGV_STATUS_FILEFORMAT: i32 = 16392i32;
pub const MCI_DGV_STATUS_FILE_COMPLETION: i32 = 16416i32;
pub const MCI_DGV_STATUS_FILE_MODE: i32 = 16415i32;
pub const MCI_DGV_STATUS_FORWARD: i32 = 16428i32;
pub const MCI_DGV_STATUS_FRAME_RATE: i32 = 16398i32;
pub const MCI_DGV_STATUS_GAMMA: i32 = 16394i32;
pub const MCI_DGV_STATUS_HPAL: i32 = 16388i32;
pub const MCI_DGV_STATUS_HWND: i32 = 16385i32;
pub const MCI_DGV_STATUS_INPUT: i32 = 4194304i32;
pub const MCI_DGV_STATUS_KEY_COLOR: i32 = 16421i32;
pub const MCI_DGV_STATUS_KEY_INDEX: i32 = 16420i32;
pub const MCI_DGV_STATUS_LEFT: i32 = 524288i32;
pub const MCI_DGV_STATUS_MONITOR: i32 = 16395i32;
pub const MCI_DGV_STATUS_MONITOR_METHOD: i32 = 16396i32;
pub const MCI_DGV_STATUS_NOMINAL: i32 = 131072i32;
pub const MCI_DGV_STATUS_OUTPUT: i32 = 8388608i32;
pub struct MCI_DGV_STATUS_PARMSA {
    pub dwCallback: PtrRepr,
    pub dwReturn: PtrRepr,
    pub dwItem: u32,
    pub dwTrack: u32,
    pub lpstrDrive: PSTR,
    pub dwReference: u32,
}
impl ::core::marker::Copy for MCI_DGV_STATUS_PARMSA {}
impl ::core::clone::Clone for MCI_DGV_STATUS_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_STATUS_PARMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_STATUS_PARMSA")
            .field("dwCallback", &self.dwCallback)
            .field("dwReturn", &self.dwReturn)
            .field("dwItem", &self.dwItem)
            .field("dwTrack", &self.dwTrack)
            .field("lpstrDrive", &self.lpstrDrive)
            .field("dwReference", &self.dwReference)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_STATUS_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.dwReturn == other.dwReturn
            && self.dwItem == other.dwItem
            && self.dwTrack == other.dwTrack
            && self.lpstrDrive == other.lpstrDrive
            && self.dwReference == other.dwReference
    }
}
impl ::core::cmp::Eq for MCI_DGV_STATUS_PARMSA {}
impl FromIntoMemory for MCI_DGV_STATUS_PARMSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwReturn = <PtrRepr as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwItem = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwTrack = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpstrDrive = <PSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwReference = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwReturn: f_dwReturn,
            dwItem: f_dwItem,
            dwTrack: f_dwTrack,
            lpstrDrive: f_lpstrDrive,
            dwReference: f_dwReference,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwReturn, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwItem, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwTrack, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpstrDrive, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwReference, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct MCI_DGV_STATUS_PARMSW {
    pub dwCallback: PtrRepr,
    pub dwReturn: PtrRepr,
    pub dwItem: u32,
    pub dwTrack: u32,
    pub lpstrDrive: PWSTR,
    pub dwReference: u32,
}
impl ::core::marker::Copy for MCI_DGV_STATUS_PARMSW {}
impl ::core::clone::Clone for MCI_DGV_STATUS_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_STATUS_PARMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_STATUS_PARMSW")
            .field("dwCallback", &self.dwCallback)
            .field("dwReturn", &self.dwReturn)
            .field("dwItem", &self.dwItem)
            .field("dwTrack", &self.dwTrack)
            .field("lpstrDrive", &self.lpstrDrive)
            .field("dwReference", &self.dwReference)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_STATUS_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.dwReturn == other.dwReturn
            && self.dwItem == other.dwItem
            && self.dwTrack == other.dwTrack
            && self.lpstrDrive == other.lpstrDrive
            && self.dwReference == other.dwReference
    }
}
impl ::core::cmp::Eq for MCI_DGV_STATUS_PARMSW {}
impl FromIntoMemory for MCI_DGV_STATUS_PARMSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwReturn = <PtrRepr as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwItem = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwTrack = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpstrDrive = <PWSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwReference = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwReturn: f_dwReturn,
            dwItem: f_dwItem,
            dwTrack: f_dwTrack,
            lpstrDrive: f_lpstrDrive,
            dwReference: f_dwReference,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwReturn, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwItem, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwTrack, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpstrDrive, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwReference, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub const MCI_DGV_STATUS_PAUSE_MODE: i32 = 16422i32;
pub const MCI_DGV_STATUS_RECORD: i32 = 16777216i32;
pub const MCI_DGV_STATUS_REFERENCE: i32 = 262144i32;
pub const MCI_DGV_STATUS_RIGHT: i32 = 1048576i32;
pub const MCI_DGV_STATUS_SAMPLESPERSEC: i32 = 16423i32;
pub const MCI_DGV_STATUS_SEEK_EXACTLY: i32 = 16401i32;
pub const MCI_DGV_STATUS_SHARPNESS: i32 = 16402i32;
pub const MCI_DGV_STATUS_SIZE: i32 = 16400i32;
pub const MCI_DGV_STATUS_SMPTE: i32 = 16403i32;
pub const MCI_DGV_STATUS_SPEED: i32 = 16387i32;
pub const MCI_DGV_STATUS_STILL_FILEFORMAT: i32 = 16413i32;
pub const MCI_DGV_STATUS_TINT: i32 = 16405i32;
pub const MCI_DGV_STATUS_TREBLE: i32 = 16406i32;
pub const MCI_DGV_STATUS_UNSAVED: i32 = 16407i32;
pub const MCI_DGV_STATUS_VIDEO: i32 = 16408i32;
pub const MCI_DGV_STATUS_VIDEO_RECORD: i32 = 16412i32;
pub const MCI_DGV_STATUS_VIDEO_SOURCE: i32 = 16411i32;
pub const MCI_DGV_STATUS_VIDEO_SRC_NUM: i32 = 16414i32;
pub const MCI_DGV_STATUS_VIDEO_STREAM: i32 = 16430i32;
pub const MCI_DGV_STATUS_VOLUME: i32 = 16409i32;
pub const MCI_DGV_STATUS_WINDOW_MAXIMIZED: i32 = 16419i32;
pub const MCI_DGV_STATUS_WINDOW_MINIMIZED: i32 = 16418i32;
pub const MCI_DGV_STATUS_WINDOW_VISIBLE: i32 = 16417i32;
pub const MCI_DGV_STEP_FRAMES: i32 = 131072i32;
pub struct MCI_DGV_STEP_PARMS {
    pub dwCallback: PtrRepr,
    pub dwFrames: u32,
}
impl ::core::marker::Copy for MCI_DGV_STEP_PARMS {}
impl ::core::clone::Clone for MCI_DGV_STEP_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_STEP_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_STEP_PARMS")
            .field("dwCallback", &self.dwCallback)
            .field("dwFrames", &self.dwFrames)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_STEP_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback && self.dwFrames == other.dwFrames
    }
}
impl ::core::cmp::Eq for MCI_DGV_STEP_PARMS {}
impl FromIntoMemory for MCI_DGV_STEP_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFrames = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwFrames: f_dwFrames,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFrames, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub const MCI_DGV_STEP_REVERSE: i32 = 65536i32;
pub const MCI_DGV_STOP_HOLD: i32 = 65536i32;
pub const MCI_DGV_UPDATE_HDC: i32 = 131072i32;
pub const MCI_DGV_UPDATE_PAINT: i32 = 262144i32;
pub struct MCI_DGV_UPDATE_PARMS {
    pub dwCallback: PtrRepr,
    pub rc: super::super::Foundation::RECT,
    pub hDC: super::super::Graphics::Gdi::HDC,
}
impl ::core::marker::Copy for MCI_DGV_UPDATE_PARMS {}
impl ::core::clone::Clone for MCI_DGV_UPDATE_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_UPDATE_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_UPDATE_PARMS")
            .field("dwCallback", &self.dwCallback)
            .field("rc", &self.rc)
            .field("hDC", &self.hDC)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_UPDATE_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback && self.rc == other.rc && self.hDC == other.hDC
    }
}
impl ::core::cmp::Eq for MCI_DGV_UPDATE_PARMS {}
impl FromIntoMemory for MCI_DGV_UPDATE_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_rc = <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[4..4 + 16]);
        let f_hDC =
            <super::super::Graphics::Gdi::HDC as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            dwCallback: f_dwCallback,
            rc: f_rc,
            hDC: f_hDC,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.rc, &mut into[4..4 + 16]);
        FromIntoMemory::into_bytes(self.hDC, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub const MCI_DGV_WHERE_DESTINATION: i32 = 262144i32;
pub const MCI_DGV_WHERE_FRAME: i32 = 524288i32;
pub const MCI_DGV_WHERE_MAX: i32 = 4194304i32;
pub const MCI_DGV_WHERE_SOURCE: i32 = 131072i32;
pub const MCI_DGV_WHERE_VIDEO: i32 = 1048576i32;
pub const MCI_DGV_WHERE_WINDOW: i32 = 2097152i32;
pub const MCI_DGV_WINDOW_DEFAULT: i32 = 0i32;
pub const MCI_DGV_WINDOW_HWND: i32 = 65536i32;
pub struct MCI_DGV_WINDOW_PARMSA {
    pub dwCallback: PtrRepr,
    pub hWnd: super::super::Foundation::HWND,
    pub nCmdShow: u32,
    pub lpstrText: PSTR,
}
impl ::core::marker::Copy for MCI_DGV_WINDOW_PARMSA {}
impl ::core::clone::Clone for MCI_DGV_WINDOW_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_WINDOW_PARMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_WINDOW_PARMSA")
            .field("dwCallback", &self.dwCallback)
            .field("hWnd", &self.hWnd)
            .field("nCmdShow", &self.nCmdShow)
            .field("lpstrText", &self.lpstrText)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_WINDOW_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.hWnd == other.hWnd
            && self.nCmdShow == other.nCmdShow
            && self.lpstrText == other.lpstrText
    }
}
impl ::core::cmp::Eq for MCI_DGV_WINDOW_PARMSA {}
impl FromIntoMemory for MCI_DGV_WINDOW_PARMSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_hWnd =
            <super::super::Foundation::HWND as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_nCmdShow = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpstrText = <PSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            dwCallback: f_dwCallback,
            hWnd: f_hWnd,
            nCmdShow: f_nCmdShow,
            lpstrText: f_lpstrText,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.hWnd, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.nCmdShow, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpstrText, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct MCI_DGV_WINDOW_PARMSW {
    pub dwCallback: PtrRepr,
    pub hWnd: super::super::Foundation::HWND,
    pub nCmdShow: u32,
    pub lpstrText: PWSTR,
}
impl ::core::marker::Copy for MCI_DGV_WINDOW_PARMSW {}
impl ::core::clone::Clone for MCI_DGV_WINDOW_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_DGV_WINDOW_PARMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_DGV_WINDOW_PARMSW")
            .field("dwCallback", &self.dwCallback)
            .field("hWnd", &self.hWnd)
            .field("nCmdShow", &self.nCmdShow)
            .field("lpstrText", &self.lpstrText)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_DGV_WINDOW_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.hWnd == other.hWnd
            && self.nCmdShow == other.nCmdShow
            && self.lpstrText == other.lpstrText
    }
}
impl ::core::cmp::Eq for MCI_DGV_WINDOW_PARMSW {}
impl FromIntoMemory for MCI_DGV_WINDOW_PARMSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_hWnd =
            <super::super::Foundation::HWND as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_nCmdShow = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpstrText = <PWSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            dwCallback: f_dwCallback,
            hWnd: f_hWnd,
            nCmdShow: f_nCmdShow,
            lpstrText: f_lpstrText,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.hWnd, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.nCmdShow, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpstrText, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub const MCI_DGV_WINDOW_STATE: i32 = 262144i32;
pub const MCI_DGV_WINDOW_TEXT: i32 = 524288i32;
pub const MCI_END_COMMAND: u32 = 3u32;
pub const MCI_END_COMMAND_LIST: u32 = 6u32;
pub const MCI_END_CONSTANT: u32 = 9u32;
pub const MCI_ESCAPE: u32 = 2053u32;
pub const MCI_FALSE: u32 = 531u32;
pub const MCI_FIRST: u32 = 2048u32;
pub const MCI_FLAG: u32 = 5u32;
pub const MCI_FORMAT_BYTES: u32 = 8u32;
pub const MCI_FORMAT_BYTES_S: u32 = 541u32;
pub const MCI_FORMAT_FRAMES: u32 = 3u32;
pub const MCI_FORMAT_FRAMES_S: u32 = 536u32;
pub const MCI_FORMAT_HMS: u32 = 1u32;
pub const MCI_FORMAT_HMS_S: u32 = 534u32;
pub const MCI_FORMAT_MILLISECONDS: u32 = 0u32;
pub const MCI_FORMAT_MILLISECONDS_S: u32 = 533u32;
pub const MCI_FORMAT_MSF: u32 = 2u32;
pub const MCI_FORMAT_MSF_S: u32 = 535u32;
pub const MCI_FORMAT_SAMPLES: u32 = 9u32;
pub const MCI_FORMAT_SAMPLES_S: u32 = 542u32;
pub const MCI_FORMAT_SMPTE_24: u32 = 4u32;
pub const MCI_FORMAT_SMPTE_24_S: u32 = 537u32;
pub const MCI_FORMAT_SMPTE_25: u32 = 5u32;
pub const MCI_FORMAT_SMPTE_25_S: u32 = 538u32;
pub const MCI_FORMAT_SMPTE_30: u32 = 6u32;
pub const MCI_FORMAT_SMPTE_30DROP: u32 = 7u32;
pub const MCI_FORMAT_SMPTE_30DROP_S: u32 = 540u32;
pub const MCI_FORMAT_SMPTE_30_S: u32 = 539u32;
pub const MCI_FORMAT_TMSF: u32 = 10u32;
pub const MCI_FORMAT_TMSF_S: u32 = 543u32;
pub const MCI_FREEZE: u32 = 2116u32;
pub const MCI_FROM: i32 = 4i32;
pub struct MCI_GENERIC_PARMS {
    pub dwCallback: PtrRepr,
}
impl ::core::marker::Copy for MCI_GENERIC_PARMS {}
impl ::core::clone::Clone for MCI_GENERIC_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_GENERIC_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_GENERIC_PARMS")
            .field("dwCallback", &self.dwCallback)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_GENERIC_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
    }
}
impl ::core::cmp::Eq for MCI_GENERIC_PARMS {}
impl FromIntoMemory for MCI_GENERIC_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            dwCallback: f_dwCallback,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub const MCI_GETDEVCAPS: u32 = 2059u32;
pub const MCI_GETDEVCAPS_CAN_EJECT: i32 = 7i32;
pub const MCI_GETDEVCAPS_CAN_PLAY: i32 = 8i32;
pub const MCI_GETDEVCAPS_CAN_RECORD: i32 = 1i32;
pub const MCI_GETDEVCAPS_CAN_SAVE: i32 = 9i32;
pub const MCI_GETDEVCAPS_COMPOUND_DEVICE: i32 = 6i32;
pub const MCI_GETDEVCAPS_DEVICE_TYPE: i32 = 4i32;
pub const MCI_GETDEVCAPS_HAS_AUDIO: i32 = 2i32;
pub const MCI_GETDEVCAPS_HAS_VIDEO: i32 = 3i32;
pub const MCI_GETDEVCAPS_ITEM: i32 = 256i32;
pub struct MCI_GETDEVCAPS_PARMS {
    pub dwCallback: PtrRepr,
    pub dwReturn: u32,
    pub dwItem: u32,
}
impl ::core::marker::Copy for MCI_GETDEVCAPS_PARMS {}
impl ::core::clone::Clone for MCI_GETDEVCAPS_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_GETDEVCAPS_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_GETDEVCAPS_PARMS")
            .field("dwCallback", &self.dwCallback)
            .field("dwReturn", &self.dwReturn)
            .field("dwItem", &self.dwItem)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_GETDEVCAPS_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.dwReturn == other.dwReturn
            && self.dwItem == other.dwItem
    }
}
impl ::core::cmp::Eq for MCI_GETDEVCAPS_PARMS {}
impl FromIntoMemory for MCI_GETDEVCAPS_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwReturn = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwItem = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwReturn: f_dwReturn,
            dwItem: f_dwItem,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwReturn, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwItem, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub const MCI_GETDEVCAPS_USES_FILES: i32 = 5i32;
pub const MCI_HDC: u32 = 12u32;
pub const MCI_HPAL: u32 = 11u32;
pub const MCI_HWND: u32 = 10u32;
pub const MCI_INFO: u32 = 2058u32;
pub const MCI_INFO_COPYRIGHT: i32 = 8192i32;
pub const MCI_INFO_FILE: i32 = 512i32;
pub const MCI_INFO_MEDIA_IDENTITY: i32 = 2048i32;
pub const MCI_INFO_MEDIA_UPC: i32 = 1024i32;
pub const MCI_INFO_NAME: i32 = 4096i32;
pub struct MCI_INFO_PARMSA {
    pub dwCallback: PtrRepr,
    pub lpstrReturn: PSTR,
    pub dwRetSize: u32,
}
impl ::core::marker::Copy for MCI_INFO_PARMSA {}
impl ::core::clone::Clone for MCI_INFO_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_INFO_PARMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_INFO_PARMSA")
            .field("dwCallback", &self.dwCallback)
            .field("lpstrReturn", &self.lpstrReturn)
            .field("dwRetSize", &self.dwRetSize)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_INFO_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.lpstrReturn == other.lpstrReturn
            && self.dwRetSize == other.dwRetSize
    }
}
impl ::core::cmp::Eq for MCI_INFO_PARMSA {}
impl FromIntoMemory for MCI_INFO_PARMSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpstrReturn = <PSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwRetSize = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            dwCallback: f_dwCallback,
            lpstrReturn: f_lpstrReturn,
            dwRetSize: f_dwRetSize,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpstrReturn, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwRetSize, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct MCI_INFO_PARMSW {
    pub dwCallback: PtrRepr,
    pub lpstrReturn: PWSTR,
    pub dwRetSize: u32,
}
impl ::core::marker::Copy for MCI_INFO_PARMSW {}
impl ::core::clone::Clone for MCI_INFO_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_INFO_PARMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_INFO_PARMSW")
            .field("dwCallback", &self.dwCallback)
            .field("lpstrReturn", &self.lpstrReturn)
            .field("dwRetSize", &self.dwRetSize)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_INFO_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.lpstrReturn == other.lpstrReturn
            && self.dwRetSize == other.dwRetSize
    }
}
impl ::core::cmp::Eq for MCI_INFO_PARMSW {}
impl FromIntoMemory for MCI_INFO_PARMSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpstrReturn = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwRetSize = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            dwCallback: f_dwCallback,
            lpstrReturn: f_lpstrReturn,
            dwRetSize: f_dwRetSize,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpstrReturn, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwRetSize, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub const MCI_INFO_PRODUCT: i32 = 256i32;
pub const MCI_INFO_VERSION: i32 = 1024i32;
pub const MCI_INTEGER: u32 = 2u32;
pub const MCI_INTEGER64: u32 = 13u32;
pub const MCI_INTEGER_RETURNED: u32 = 524288u32;
pub const MCI_LAST: u32 = 4095u32;
pub const MCI_LIST: u32 = 2168u32;
pub const MCI_LOAD: u32 = 2128u32;
pub const MCI_LOAD_FILE: i32 = 256i32;
pub struct MCI_LOAD_PARMSA {
    pub dwCallback: PtrRepr,
    pub lpfilename: PCSTR,
}
impl ::core::marker::Copy for MCI_LOAD_PARMSA {}
impl ::core::clone::Clone for MCI_LOAD_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_LOAD_PARMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_LOAD_PARMSA")
            .field("dwCallback", &self.dwCallback)
            .field("lpfilename", &self.lpfilename)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_LOAD_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback && self.lpfilename == other.lpfilename
    }
}
impl ::core::cmp::Eq for MCI_LOAD_PARMSA {}
impl FromIntoMemory for MCI_LOAD_PARMSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpfilename = <PCSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            dwCallback: f_dwCallback,
            lpfilename: f_lpfilename,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpfilename, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct MCI_LOAD_PARMSW {
    pub dwCallback: PtrRepr,
    pub lpfilename: PCWSTR,
}
impl ::core::marker::Copy for MCI_LOAD_PARMSW {}
impl ::core::clone::Clone for MCI_LOAD_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_LOAD_PARMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_LOAD_PARMSW")
            .field("dwCallback", &self.dwCallback)
            .field("lpfilename", &self.lpfilename)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_LOAD_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback && self.lpfilename == other.lpfilename
    }
}
impl ::core::cmp::Eq for MCI_LOAD_PARMSW {}
impl FromIntoMemory for MCI_LOAD_PARMSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpfilename = <PCWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            dwCallback: f_dwCallback,
            lpfilename: f_lpfilename,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpfilename, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub const MCI_MAX_DEVICE_TYPE_LENGTH: u32 = 80u32;
pub const MCI_MCIAVI_PLAY_FULLBY2: i32 = 67108864i32;
pub const MCI_MCIAVI_PLAY_FULLSCREEN: i32 = 33554432i32;
pub const MCI_MCIAVI_PLAY_WINDOW: i32 = 16777216i32;
pub const MCI_MODE_NOT_READY: u32 = 524u32;
pub const MCI_MODE_OPEN: u32 = 530u32;
pub const MCI_MODE_PAUSE: u32 = 529u32;
pub const MCI_MODE_PLAY: u32 = 526u32;
pub const MCI_MODE_RECORD: u32 = 527u32;
pub const MCI_MODE_SEEK: u32 = 528u32;
pub const MCI_MODE_STOP: u32 = 525u32;
pub const MCI_MONITOR: u32 = 2161u32;
pub const MCI_NOTIFY: i32 = 1i32;
pub const MCI_NOTIFY_ABORTED: u32 = 4u32;
pub const MCI_NOTIFY_FAILURE: u32 = 8u32;
pub const MCI_NOTIFY_SUCCESSFUL: u32 = 1u32;
pub const MCI_NOTIFY_SUPERSEDED: u32 = 2u32;
pub const MCI_OFF: u32 = 0u32;
pub const MCI_OFF_S: i32 = 32769i32;
pub const MCI_ON: u32 = 1u32;
pub const MCI_ON_S: i32 = 32768i32;
pub const MCI_OPEN: u32 = 2051u32;
pub const MCI_OPEN_ALIAS: i32 = 1024i32;
pub const MCI_OPEN_DRIVER: u32 = 2049u32;
pub struct MCI_OPEN_DRIVER_PARMS {
    pub wDeviceID: u32,
    pub lpstrParams: PCWSTR,
    pub wCustomCommandTable: u32,
    pub wType: u32,
}
impl ::core::marker::Copy for MCI_OPEN_DRIVER_PARMS {}
impl ::core::clone::Clone for MCI_OPEN_DRIVER_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_OPEN_DRIVER_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_OPEN_DRIVER_PARMS")
            .field("wDeviceID", &self.wDeviceID)
            .field("lpstrParams", &self.lpstrParams)
            .field("wCustomCommandTable", &self.wCustomCommandTable)
            .field("wType", &self.wType)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_OPEN_DRIVER_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.wDeviceID == other.wDeviceID
            && self.lpstrParams == other.lpstrParams
            && self.wCustomCommandTable == other.wCustomCommandTable
            && self.wType == other.wType
    }
}
impl ::core::cmp::Eq for MCI_OPEN_DRIVER_PARMS {}
impl FromIntoMemory for MCI_OPEN_DRIVER_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_wDeviceID = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpstrParams = <PCWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_wCustomCommandTable = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_wType = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            wDeviceID: f_wDeviceID,
            lpstrParams: f_lpstrParams,
            wCustomCommandTable: f_wCustomCommandTable,
            wType: f_wType,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.wDeviceID, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpstrParams, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.wCustomCommandTable, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.wType, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub const MCI_OPEN_ELEMENT: i32 = 512i32;
pub const MCI_OPEN_ELEMENT_ID: i32 = 2048i32;
pub struct MCI_OPEN_PARMSA {
    pub dwCallback: PtrRepr,
    pub wDeviceID: u32,
    pub lpstrDeviceType: PCSTR,
    pub lpstrElementName: PCSTR,
    pub lpstrAlias: PCSTR,
}
impl ::core::marker::Copy for MCI_OPEN_PARMSA {}
impl ::core::clone::Clone for MCI_OPEN_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_OPEN_PARMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_OPEN_PARMSA")
            .field("dwCallback", &self.dwCallback)
            .field("wDeviceID", &self.wDeviceID)
            .field("lpstrDeviceType", &self.lpstrDeviceType)
            .field("lpstrElementName", &self.lpstrElementName)
            .field("lpstrAlias", &self.lpstrAlias)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_OPEN_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.wDeviceID == other.wDeviceID
            && self.lpstrDeviceType == other.lpstrDeviceType
            && self.lpstrElementName == other.lpstrElementName
            && self.lpstrAlias == other.lpstrAlias
    }
}
impl ::core::cmp::Eq for MCI_OPEN_PARMSA {}
impl FromIntoMemory for MCI_OPEN_PARMSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_wDeviceID = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpstrDeviceType = <PCSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpstrElementName = <PCSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpstrAlias = <PCSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            dwCallback: f_dwCallback,
            wDeviceID: f_wDeviceID,
            lpstrDeviceType: f_lpstrDeviceType,
            lpstrElementName: f_lpstrElementName,
            lpstrAlias: f_lpstrAlias,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.wDeviceID, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpstrDeviceType, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpstrElementName, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpstrAlias, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub struct MCI_OPEN_PARMSW {
    pub dwCallback: PtrRepr,
    pub wDeviceID: u32,
    pub lpstrDeviceType: PCWSTR,
    pub lpstrElementName: PCWSTR,
    pub lpstrAlias: PCWSTR,
}
impl ::core::marker::Copy for MCI_OPEN_PARMSW {}
impl ::core::clone::Clone for MCI_OPEN_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_OPEN_PARMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_OPEN_PARMSW")
            .field("dwCallback", &self.dwCallback)
            .field("wDeviceID", &self.wDeviceID)
            .field("lpstrDeviceType", &self.lpstrDeviceType)
            .field("lpstrElementName", &self.lpstrElementName)
            .field("lpstrAlias", &self.lpstrAlias)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_OPEN_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.wDeviceID == other.wDeviceID
            && self.lpstrDeviceType == other.lpstrDeviceType
            && self.lpstrElementName == other.lpstrElementName
            && self.lpstrAlias == other.lpstrAlias
    }
}
impl ::core::cmp::Eq for MCI_OPEN_PARMSW {}
impl FromIntoMemory for MCI_OPEN_PARMSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_wDeviceID = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpstrDeviceType = <PCWSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpstrElementName = <PCWSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpstrAlias = <PCWSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            dwCallback: f_dwCallback,
            wDeviceID: f_wDeviceID,
            lpstrDeviceType: f_lpstrDeviceType,
            lpstrElementName: f_lpstrElementName,
            lpstrAlias: f_lpstrAlias,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.wDeviceID, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpstrDeviceType, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpstrElementName, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpstrAlias, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub const MCI_OPEN_SHAREABLE: i32 = 256i32;
pub const MCI_OPEN_TYPE: i32 = 8192i32;
pub const MCI_OPEN_TYPE_ID: i32 = 4096i32;
pub const MCI_OVLY_GETDEVCAPS_CAN_FREEZE: i32 = 16386i32;
pub const MCI_OVLY_GETDEVCAPS_CAN_STRETCH: i32 = 16385i32;
pub const MCI_OVLY_GETDEVCAPS_MAX_WINDOWS: i32 = 16387i32;
pub const MCI_OVLY_INFO_TEXT: i32 = 65536i32;
pub struct MCI_OVLY_LOAD_PARMSA {
    pub dwCallback: PtrRepr,
    pub lpfilename: PCSTR,
    pub rc: super::super::Foundation::RECT,
}
impl ::core::marker::Copy for MCI_OVLY_LOAD_PARMSA {}
impl ::core::clone::Clone for MCI_OVLY_LOAD_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_OVLY_LOAD_PARMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_OVLY_LOAD_PARMSA")
            .field("dwCallback", &self.dwCallback)
            .field("lpfilename", &self.lpfilename)
            .field("rc", &self.rc)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_OVLY_LOAD_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.lpfilename == other.lpfilename
            && self.rc == other.rc
    }
}
impl ::core::cmp::Eq for MCI_OVLY_LOAD_PARMSA {}
impl FromIntoMemory for MCI_OVLY_LOAD_PARMSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpfilename = <PCSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_rc = <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        Self {
            dwCallback: f_dwCallback,
            lpfilename: f_lpfilename,
            rc: f_rc,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpfilename, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.rc, &mut into[8..8 + 16]);
    }
    fn size() -> usize {
        24
    }
}
pub struct MCI_OVLY_LOAD_PARMSW {
    pub dwCallback: PtrRepr,
    pub lpfilename: PCWSTR,
    pub rc: super::super::Foundation::RECT,
}
impl ::core::marker::Copy for MCI_OVLY_LOAD_PARMSW {}
impl ::core::clone::Clone for MCI_OVLY_LOAD_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_OVLY_LOAD_PARMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_OVLY_LOAD_PARMSW")
            .field("dwCallback", &self.dwCallback)
            .field("lpfilename", &self.lpfilename)
            .field("rc", &self.rc)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_OVLY_LOAD_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.lpfilename == other.lpfilename
            && self.rc == other.rc
    }
}
impl ::core::cmp::Eq for MCI_OVLY_LOAD_PARMSW {}
impl FromIntoMemory for MCI_OVLY_LOAD_PARMSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpfilename = <PCWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_rc = <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        Self {
            dwCallback: f_dwCallback,
            lpfilename: f_lpfilename,
            rc: f_rc,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpfilename, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.rc, &mut into[8..8 + 16]);
    }
    fn size() -> usize {
        24
    }
}
pub const MCI_OVLY_OPEN_PARENT: i32 = 131072i32;
pub struct MCI_OVLY_OPEN_PARMSA {
    pub dwCallback: PtrRepr,
    pub wDeviceID: u32,
    pub lpstrDeviceType: PCSTR,
    pub lpstrElementName: PCSTR,
    pub lpstrAlias: PCSTR,
    pub dwStyle: u32,
    pub hWndParent: super::super::Foundation::HWND,
}
impl ::core::marker::Copy for MCI_OVLY_OPEN_PARMSA {}
impl ::core::clone::Clone for MCI_OVLY_OPEN_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_OVLY_OPEN_PARMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_OVLY_OPEN_PARMSA")
            .field("dwCallback", &self.dwCallback)
            .field("wDeviceID", &self.wDeviceID)
            .field("lpstrDeviceType", &self.lpstrDeviceType)
            .field("lpstrElementName", &self.lpstrElementName)
            .field("lpstrAlias", &self.lpstrAlias)
            .field("dwStyle", &self.dwStyle)
            .field("hWndParent", &self.hWndParent)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_OVLY_OPEN_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.wDeviceID == other.wDeviceID
            && self.lpstrDeviceType == other.lpstrDeviceType
            && self.lpstrElementName == other.lpstrElementName
            && self.lpstrAlias == other.lpstrAlias
            && self.dwStyle == other.dwStyle
            && self.hWndParent == other.hWndParent
    }
}
impl ::core::cmp::Eq for MCI_OVLY_OPEN_PARMSA {}
impl FromIntoMemory for MCI_OVLY_OPEN_PARMSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_wDeviceID = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpstrDeviceType = <PCSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpstrElementName = <PCSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpstrAlias = <PCSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwStyle = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_hWndParent =
            <super::super::Foundation::HWND as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        Self {
            dwCallback: f_dwCallback,
            wDeviceID: f_wDeviceID,
            lpstrDeviceType: f_lpstrDeviceType,
            lpstrElementName: f_lpstrElementName,
            lpstrAlias: f_lpstrAlias,
            dwStyle: f_dwStyle,
            hWndParent: f_hWndParent,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.wDeviceID, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpstrDeviceType, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpstrElementName, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpstrAlias, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwStyle, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.hWndParent, &mut into[24..24 + 4]);
    }
    fn size() -> usize {
        28
    }
}
pub struct MCI_OVLY_OPEN_PARMSW {
    pub dwCallback: PtrRepr,
    pub wDeviceID: u32,
    pub lpstrDeviceType: PCWSTR,
    pub lpstrElementName: PCWSTR,
    pub lpstrAlias: PCWSTR,
    pub dwStyle: u32,
    pub hWndParent: super::super::Foundation::HWND,
}
impl ::core::marker::Copy for MCI_OVLY_OPEN_PARMSW {}
impl ::core::clone::Clone for MCI_OVLY_OPEN_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_OVLY_OPEN_PARMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_OVLY_OPEN_PARMSW")
            .field("dwCallback", &self.dwCallback)
            .field("wDeviceID", &self.wDeviceID)
            .field("lpstrDeviceType", &self.lpstrDeviceType)
            .field("lpstrElementName", &self.lpstrElementName)
            .field("lpstrAlias", &self.lpstrAlias)
            .field("dwStyle", &self.dwStyle)
            .field("hWndParent", &self.hWndParent)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_OVLY_OPEN_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.wDeviceID == other.wDeviceID
            && self.lpstrDeviceType == other.lpstrDeviceType
            && self.lpstrElementName == other.lpstrElementName
            && self.lpstrAlias == other.lpstrAlias
            && self.dwStyle == other.dwStyle
            && self.hWndParent == other.hWndParent
    }
}
impl ::core::cmp::Eq for MCI_OVLY_OPEN_PARMSW {}
impl FromIntoMemory for MCI_OVLY_OPEN_PARMSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_wDeviceID = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpstrDeviceType = <PCWSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpstrElementName = <PCWSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpstrAlias = <PCWSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwStyle = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_hWndParent =
            <super::super::Foundation::HWND as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        Self {
            dwCallback: f_dwCallback,
            wDeviceID: f_wDeviceID,
            lpstrDeviceType: f_lpstrDeviceType,
            lpstrElementName: f_lpstrElementName,
            lpstrAlias: f_lpstrAlias,
            dwStyle: f_dwStyle,
            hWndParent: f_hWndParent,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.wDeviceID, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpstrDeviceType, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpstrElementName, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpstrAlias, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwStyle, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.hWndParent, &mut into[24..24 + 4]);
    }
    fn size() -> usize {
        28
    }
}
pub const MCI_OVLY_OPEN_WS: i32 = 65536i32;
pub const MCI_OVLY_PUT_DESTINATION: i32 = 262144i32;
pub const MCI_OVLY_PUT_FRAME: i32 = 524288i32;
pub const MCI_OVLY_PUT_SOURCE: i32 = 131072i32;
pub const MCI_OVLY_PUT_VIDEO: i32 = 1048576i32;
pub const MCI_OVLY_RECT: i32 = 65536i32;
pub struct MCI_OVLY_RECT_PARMS {
    pub dwCallback: PtrRepr,
    pub rc: super::super::Foundation::RECT,
}
impl ::core::marker::Copy for MCI_OVLY_RECT_PARMS {}
impl ::core::clone::Clone for MCI_OVLY_RECT_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_OVLY_RECT_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_OVLY_RECT_PARMS")
            .field("dwCallback", &self.dwCallback)
            .field("rc", &self.rc)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_OVLY_RECT_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback && self.rc == other.rc
    }
}
impl ::core::cmp::Eq for MCI_OVLY_RECT_PARMS {}
impl FromIntoMemory for MCI_OVLY_RECT_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_rc = <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[4..4 + 16]);
        Self {
            dwCallback: f_dwCallback,
            rc: f_rc,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.rc, &mut into[4..4 + 16]);
    }
    fn size() -> usize {
        20
    }
}
pub struct MCI_OVLY_SAVE_PARMSA {
    pub dwCallback: PtrRepr,
    pub lpfilename: PCSTR,
    pub rc: super::super::Foundation::RECT,
}
impl ::core::marker::Copy for MCI_OVLY_SAVE_PARMSA {}
impl ::core::clone::Clone for MCI_OVLY_SAVE_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_OVLY_SAVE_PARMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_OVLY_SAVE_PARMSA")
            .field("dwCallback", &self.dwCallback)
            .field("lpfilename", &self.lpfilename)
            .field("rc", &self.rc)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_OVLY_SAVE_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.lpfilename == other.lpfilename
            && self.rc == other.rc
    }
}
impl ::core::cmp::Eq for MCI_OVLY_SAVE_PARMSA {}
impl FromIntoMemory for MCI_OVLY_SAVE_PARMSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpfilename = <PCSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_rc = <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        Self {
            dwCallback: f_dwCallback,
            lpfilename: f_lpfilename,
            rc: f_rc,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpfilename, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.rc, &mut into[8..8 + 16]);
    }
    fn size() -> usize {
        24
    }
}
pub struct MCI_OVLY_SAVE_PARMSW {
    pub dwCallback: PtrRepr,
    pub lpfilename: PCWSTR,
    pub rc: super::super::Foundation::RECT,
}
impl ::core::marker::Copy for MCI_OVLY_SAVE_PARMSW {}
impl ::core::clone::Clone for MCI_OVLY_SAVE_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_OVLY_SAVE_PARMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_OVLY_SAVE_PARMSW")
            .field("dwCallback", &self.dwCallback)
            .field("lpfilename", &self.lpfilename)
            .field("rc", &self.rc)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_OVLY_SAVE_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.lpfilename == other.lpfilename
            && self.rc == other.rc
    }
}
impl ::core::cmp::Eq for MCI_OVLY_SAVE_PARMSW {}
impl FromIntoMemory for MCI_OVLY_SAVE_PARMSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpfilename = <PCWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_rc = <super::super::Foundation::RECT as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        Self {
            dwCallback: f_dwCallback,
            lpfilename: f_lpfilename,
            rc: f_rc,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpfilename, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.rc, &mut into[8..8 + 16]);
    }
    fn size() -> usize {
        24
    }
}
pub const MCI_OVLY_STATUS_HWND: i32 = 16385i32;
pub const MCI_OVLY_STATUS_STRETCH: i32 = 16386i32;
pub const MCI_OVLY_WHERE_DESTINATION: i32 = 262144i32;
pub const MCI_OVLY_WHERE_FRAME: i32 = 524288i32;
pub const MCI_OVLY_WHERE_SOURCE: i32 = 131072i32;
pub const MCI_OVLY_WHERE_VIDEO: i32 = 1048576i32;
pub const MCI_OVLY_WINDOW_DEFAULT: i32 = 0i32;
pub const MCI_OVLY_WINDOW_DISABLE_STRETCH: i32 = 2097152i32;
pub const MCI_OVLY_WINDOW_ENABLE_STRETCH: i32 = 1048576i32;
pub const MCI_OVLY_WINDOW_HWND: i32 = 65536i32;
pub struct MCI_OVLY_WINDOW_PARMSA {
    pub dwCallback: PtrRepr,
    pub hWnd: super::super::Foundation::HWND,
    pub nCmdShow: u32,
    pub lpstrText: PCSTR,
}
impl ::core::marker::Copy for MCI_OVLY_WINDOW_PARMSA {}
impl ::core::clone::Clone for MCI_OVLY_WINDOW_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_OVLY_WINDOW_PARMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_OVLY_WINDOW_PARMSA")
            .field("dwCallback", &self.dwCallback)
            .field("hWnd", &self.hWnd)
            .field("nCmdShow", &self.nCmdShow)
            .field("lpstrText", &self.lpstrText)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_OVLY_WINDOW_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.hWnd == other.hWnd
            && self.nCmdShow == other.nCmdShow
            && self.lpstrText == other.lpstrText
    }
}
impl ::core::cmp::Eq for MCI_OVLY_WINDOW_PARMSA {}
impl FromIntoMemory for MCI_OVLY_WINDOW_PARMSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_hWnd =
            <super::super::Foundation::HWND as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_nCmdShow = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpstrText = <PCSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            dwCallback: f_dwCallback,
            hWnd: f_hWnd,
            nCmdShow: f_nCmdShow,
            lpstrText: f_lpstrText,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.hWnd, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.nCmdShow, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpstrText, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct MCI_OVLY_WINDOW_PARMSW {
    pub dwCallback: PtrRepr,
    pub hWnd: super::super::Foundation::HWND,
    pub nCmdShow: u32,
    pub lpstrText: PCWSTR,
}
impl ::core::marker::Copy for MCI_OVLY_WINDOW_PARMSW {}
impl ::core::clone::Clone for MCI_OVLY_WINDOW_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_OVLY_WINDOW_PARMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_OVLY_WINDOW_PARMSW")
            .field("dwCallback", &self.dwCallback)
            .field("hWnd", &self.hWnd)
            .field("nCmdShow", &self.nCmdShow)
            .field("lpstrText", &self.lpstrText)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_OVLY_WINDOW_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.hWnd == other.hWnd
            && self.nCmdShow == other.nCmdShow
            && self.lpstrText == other.lpstrText
    }
}
impl ::core::cmp::Eq for MCI_OVLY_WINDOW_PARMSW {}
impl FromIntoMemory for MCI_OVLY_WINDOW_PARMSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_hWnd =
            <super::super::Foundation::HWND as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_nCmdShow = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpstrText = <PCWSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            dwCallback: f_dwCallback,
            hWnd: f_hWnd,
            nCmdShow: f_nCmdShow,
            lpstrText: f_lpstrText,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.hWnd, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.nCmdShow, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpstrText, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub const MCI_OVLY_WINDOW_STATE: i32 = 262144i32;
pub const MCI_OVLY_WINDOW_TEXT: i32 = 524288i32;
pub const MCI_PASTE: u32 = 2131u32;
pub const MCI_PAUSE: u32 = 2057u32;
pub const MCI_PLAY: u32 = 2054u32;
pub struct MCI_PLAY_PARMS {
    pub dwCallback: PtrRepr,
    pub dwFrom: u32,
    pub dwTo: u32,
}
impl ::core::marker::Copy for MCI_PLAY_PARMS {}
impl ::core::clone::Clone for MCI_PLAY_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_PLAY_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_PLAY_PARMS")
            .field("dwCallback", &self.dwCallback)
            .field("dwFrom", &self.dwFrom)
            .field("dwTo", &self.dwTo)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_PLAY_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.dwFrom == other.dwFrom
            && self.dwTo == other.dwTo
    }
}
impl ::core::cmp::Eq for MCI_PLAY_PARMS {}
impl FromIntoMemory for MCI_PLAY_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFrom = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwTo = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwFrom: f_dwFrom,
            dwTo: f_dwTo,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFrom, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwTo, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub const MCI_PUT: u32 = 2114u32;
pub const MCI_QUALITY: u32 = 2167u32;
pub const MCI_QUALITY_ALG: i32 = 262144i32;
pub const MCI_QUALITY_DIALOG: i32 = 524288i32;
pub const MCI_QUALITY_HANDLE: i32 = 1048576i32;
pub const MCI_QUALITY_ITEM: i32 = 65536i32;
pub const MCI_QUALITY_ITEM_AUDIO: i32 = 16384i32;
pub const MCI_QUALITY_ITEM_STILL: i32 = 16385i32;
pub const MCI_QUALITY_ITEM_VIDEO: i32 = 16386i32;
pub const MCI_QUALITY_NAME: i32 = 131072i32;
pub const MCI_REALIZE: u32 = 2112u32;
pub const MCI_RECORD: u32 = 2063u32;
pub const MCI_RECORD_INSERT: i32 = 256i32;
pub const MCI_RECORD_OVERWRITE: i32 = 512i32;
pub struct MCI_RECORD_PARMS {
    pub dwCallback: PtrRepr,
    pub dwFrom: u32,
    pub dwTo: u32,
}
impl ::core::marker::Copy for MCI_RECORD_PARMS {}
impl ::core::clone::Clone for MCI_RECORD_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_RECORD_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_RECORD_PARMS")
            .field("dwCallback", &self.dwCallback)
            .field("dwFrom", &self.dwFrom)
            .field("dwTo", &self.dwTo)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_RECORD_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.dwFrom == other.dwFrom
            && self.dwTo == other.dwTo
    }
}
impl ::core::cmp::Eq for MCI_RECORD_PARMS {}
impl FromIntoMemory for MCI_RECORD_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFrom = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwTo = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwFrom: f_dwFrom,
            dwTo: f_dwTo,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFrom, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwTo, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub const MCI_RECT: u32 = 7u32;
pub const MCI_RESERVE: u32 = 2162u32;
pub const MCI_RESOURCE_DRIVER: u32 = 1048576u32;
pub const MCI_RESOURCE_RETURNED: u32 = 65536u32;
pub const MCI_RESTORE: u32 = 2171u32;
pub const MCI_RESUME: u32 = 2133u32;
pub const MCI_RETURN: u32 = 4u32;
pub const MCI_SAVE: u32 = 2067u32;
pub const MCI_SAVE_FILE: i32 = 256i32;
pub struct MCI_SAVE_PARMSA {
    pub dwCallback: PtrRepr,
    pub lpfilename: PCSTR,
}
impl ::core::marker::Copy for MCI_SAVE_PARMSA {}
impl ::core::clone::Clone for MCI_SAVE_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_SAVE_PARMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_SAVE_PARMSA")
            .field("dwCallback", &self.dwCallback)
            .field("lpfilename", &self.lpfilename)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_SAVE_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback && self.lpfilename == other.lpfilename
    }
}
impl ::core::cmp::Eq for MCI_SAVE_PARMSA {}
impl FromIntoMemory for MCI_SAVE_PARMSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpfilename = <PCSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            dwCallback: f_dwCallback,
            lpfilename: f_lpfilename,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpfilename, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct MCI_SAVE_PARMSW {
    pub dwCallback: PtrRepr,
    pub lpfilename: PCWSTR,
}
impl ::core::marker::Copy for MCI_SAVE_PARMSW {}
impl ::core::clone::Clone for MCI_SAVE_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_SAVE_PARMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_SAVE_PARMSW")
            .field("dwCallback", &self.dwCallback)
            .field("lpfilename", &self.lpfilename)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_SAVE_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback && self.lpfilename == other.lpfilename
    }
}
impl ::core::cmp::Eq for MCI_SAVE_PARMSW {}
impl FromIntoMemory for MCI_SAVE_PARMSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpfilename = <PCWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            dwCallback: f_dwCallback,
            lpfilename: f_lpfilename,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpfilename, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub const MCI_SECTION: &'static str = "MCI32";
pub const MCI_SEEK: u32 = 2055u32;
pub struct MCI_SEEK_PARMS {
    pub dwCallback: PtrRepr,
    pub dwTo: u32,
}
impl ::core::marker::Copy for MCI_SEEK_PARMS {}
impl ::core::clone::Clone for MCI_SEEK_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_SEEK_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_SEEK_PARMS")
            .field("dwCallback", &self.dwCallback)
            .field("dwTo", &self.dwTo)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_SEEK_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback && self.dwTo == other.dwTo
    }
}
impl ::core::cmp::Eq for MCI_SEEK_PARMS {}
impl FromIntoMemory for MCI_SEEK_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwTo = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwTo: f_dwTo,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwTo, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub const MCI_SEEK_TO_END: i32 = 512i32;
pub const MCI_SEEK_TO_START: i32 = 256i32;
pub const MCI_SEQ_FILE: u32 = 16386u32;
pub const MCI_SEQ_FILE_S: u32 = 1222u32;
pub const MCI_SEQ_FORMAT_SONGPTR: u32 = 16385u32;
pub const MCI_SEQ_FORMAT_SONGPTR_S: u32 = 1225u32;
pub const MCI_SEQ_MAPPER: u32 = 65535u32;
pub const MCI_SEQ_MAPPER_S: u32 = 1221u32;
pub const MCI_SEQ_MIDI: u32 = 16387u32;
pub const MCI_SEQ_MIDI_S: u32 = 1223u32;
pub const MCI_SEQ_NONE: u32 = 65533u32;
pub const MCI_SEQ_NONE_S: u32 = 1226u32;
pub const MCI_SEQ_SET_MASTER: i32 = 524288i32;
pub const MCI_SEQ_SET_OFFSET: i32 = 16777216i32;
pub struct MCI_SEQ_SET_PARMS {
    pub dwCallback: PtrRepr,
    pub dwTimeFormat: u32,
    pub dwAudio: u32,
    pub dwTempo: u32,
    pub dwPort: u32,
    pub dwSlave: u32,
    pub dwMaster: u32,
    pub dwOffset: u32,
}
impl ::core::marker::Copy for MCI_SEQ_SET_PARMS {}
impl ::core::clone::Clone for MCI_SEQ_SET_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_SEQ_SET_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_SEQ_SET_PARMS")
            .field("dwCallback", &self.dwCallback)
            .field("dwTimeFormat", &self.dwTimeFormat)
            .field("dwAudio", &self.dwAudio)
            .field("dwTempo", &self.dwTempo)
            .field("dwPort", &self.dwPort)
            .field("dwSlave", &self.dwSlave)
            .field("dwMaster", &self.dwMaster)
            .field("dwOffset", &self.dwOffset)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_SEQ_SET_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.dwTimeFormat == other.dwTimeFormat
            && self.dwAudio == other.dwAudio
            && self.dwTempo == other.dwTempo
            && self.dwPort == other.dwPort
            && self.dwSlave == other.dwSlave
            && self.dwMaster == other.dwMaster
            && self.dwOffset == other.dwOffset
    }
}
impl ::core::cmp::Eq for MCI_SEQ_SET_PARMS {}
impl FromIntoMemory for MCI_SEQ_SET_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwTimeFormat = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwAudio = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwTempo = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwPort = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwSlave = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwMaster = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_dwOffset = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwTimeFormat: f_dwTimeFormat,
            dwAudio: f_dwAudio,
            dwTempo: f_dwTempo,
            dwPort: f_dwPort,
            dwSlave: f_dwSlave,
            dwMaster: f_dwMaster,
            dwOffset: f_dwOffset,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwTimeFormat, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwAudio, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwTempo, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwPort, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwSlave, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwMaster, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.dwOffset, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
pub const MCI_SEQ_SET_PORT: i32 = 131072i32;
pub const MCI_SEQ_SET_SLAVE: i32 = 262144i32;
pub const MCI_SEQ_SET_TEMPO: i32 = 65536i32;
pub const MCI_SEQ_SMPTE: u32 = 16388u32;
pub const MCI_SEQ_SMPTE_S: u32 = 1224u32;
pub const MCI_SEQ_STATUS_COPYRIGHT: i32 = 16396i32;
pub const MCI_SEQ_STATUS_DIVTYPE: i32 = 16394i32;
pub const MCI_SEQ_STATUS_MASTER: i32 = 16392i32;
pub const MCI_SEQ_STATUS_NAME: i32 = 16395i32;
pub const MCI_SEQ_STATUS_OFFSET: i32 = 16393i32;
pub const MCI_SEQ_STATUS_PORT: i32 = 16387i32;
pub const MCI_SEQ_STATUS_SLAVE: i32 = 16391i32;
pub const MCI_SEQ_STATUS_TEMPO: i32 = 16386i32;
pub const MCI_SET: u32 = 2061u32;
pub const MCI_SETAUDIO: u32 = 2163u32;
pub const MCI_SETVIDEO: u32 = 2166u32;
pub const MCI_SET_AUDIO: i32 = 2048i32;
pub const MCI_SET_AUDIO_ALL: i32 = 0i32;
pub const MCI_SET_AUDIO_LEFT: i32 = 1i32;
pub const MCI_SET_AUDIO_RIGHT: i32 = 2i32;
pub const MCI_SET_DOOR_CLOSED: i32 = 512i32;
pub const MCI_SET_DOOR_OPEN: i32 = 256i32;
pub const MCI_SET_OFF: i32 = 16384i32;
pub const MCI_SET_ON: i32 = 8192i32;
pub struct MCI_SET_PARMS {
    pub dwCallback: PtrRepr,
    pub dwTimeFormat: u32,
    pub dwAudio: u32,
}
impl ::core::marker::Copy for MCI_SET_PARMS {}
impl ::core::clone::Clone for MCI_SET_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_SET_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_SET_PARMS")
            .field("dwCallback", &self.dwCallback)
            .field("dwTimeFormat", &self.dwTimeFormat)
            .field("dwAudio", &self.dwAudio)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_SET_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.dwTimeFormat == other.dwTimeFormat
            && self.dwAudio == other.dwAudio
    }
}
impl ::core::cmp::Eq for MCI_SET_PARMS {}
impl FromIntoMemory for MCI_SET_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwTimeFormat = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwAudio = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwTimeFormat: f_dwTimeFormat,
            dwAudio: f_dwAudio,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwTimeFormat, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwAudio, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub const MCI_SET_TIME_FORMAT: i32 = 1024i32;
pub const MCI_SET_VIDEO: i32 = 4096i32;
pub const MCI_SIGNAL: u32 = 2165u32;
pub const MCI_SPIN: u32 = 2060u32;
pub const MCI_STATUS: u32 = 2068u32;
pub const MCI_STATUS_CURRENT_TRACK: i32 = 8i32;
pub const MCI_STATUS_ITEM: i32 = 256i32;
pub const MCI_STATUS_LENGTH: i32 = 1i32;
pub const MCI_STATUS_MEDIA_PRESENT: i32 = 5i32;
pub const MCI_STATUS_MODE: i32 = 4i32;
pub const MCI_STATUS_NUMBER_OF_TRACKS: i32 = 3i32;
pub struct MCI_STATUS_PARMS {
    pub dwCallback: PtrRepr,
    pub dwReturn: PtrRepr,
    pub dwItem: u32,
    pub dwTrack: u32,
}
impl ::core::marker::Copy for MCI_STATUS_PARMS {}
impl ::core::clone::Clone for MCI_STATUS_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_STATUS_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_STATUS_PARMS")
            .field("dwCallback", &self.dwCallback)
            .field("dwReturn", &self.dwReturn)
            .field("dwItem", &self.dwItem)
            .field("dwTrack", &self.dwTrack)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_STATUS_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.dwReturn == other.dwReturn
            && self.dwItem == other.dwItem
            && self.dwTrack == other.dwTrack
    }
}
impl ::core::cmp::Eq for MCI_STATUS_PARMS {}
impl FromIntoMemory for MCI_STATUS_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwReturn = <PtrRepr as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwItem = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwTrack = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwReturn: f_dwReturn,
            dwItem: f_dwItem,
            dwTrack: f_dwTrack,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwReturn, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwItem, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwTrack, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub const MCI_STATUS_POSITION: i32 = 2i32;
pub const MCI_STATUS_READY: i32 = 7i32;
pub const MCI_STATUS_START: i32 = 512i32;
pub const MCI_STATUS_TIME_FORMAT: i32 = 6i32;
pub const MCI_STEP: u32 = 2062u32;
pub const MCI_STOP: u32 = 2056u32;
pub const MCI_STRING: u32 = 1u32;
pub const MCI_SYSINFO: u32 = 2064u32;
pub const MCI_SYSINFO_INSTALLNAME: i32 = 2048i32;
pub const MCI_SYSINFO_NAME: i32 = 1024i32;
pub const MCI_SYSINFO_OPEN: i32 = 512i32;
pub struct MCI_SYSINFO_PARMSA {
    pub dwCallback: PtrRepr,
    pub lpstrReturn: PSTR,
    pub dwRetSize: u32,
    pub dwNumber: u32,
    pub wDeviceType: u32,
}
impl ::core::marker::Copy for MCI_SYSINFO_PARMSA {}
impl ::core::clone::Clone for MCI_SYSINFO_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_SYSINFO_PARMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_SYSINFO_PARMSA")
            .field("dwCallback", &self.dwCallback)
            .field("lpstrReturn", &self.lpstrReturn)
            .field("dwRetSize", &self.dwRetSize)
            .field("dwNumber", &self.dwNumber)
            .field("wDeviceType", &self.wDeviceType)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_SYSINFO_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.lpstrReturn == other.lpstrReturn
            && self.dwRetSize == other.dwRetSize
            && self.dwNumber == other.dwNumber
            && self.wDeviceType == other.wDeviceType
    }
}
impl ::core::cmp::Eq for MCI_SYSINFO_PARMSA {}
impl FromIntoMemory for MCI_SYSINFO_PARMSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpstrReturn = <PSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwRetSize = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwNumber = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_wDeviceType = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            dwCallback: f_dwCallback,
            lpstrReturn: f_lpstrReturn,
            dwRetSize: f_dwRetSize,
            dwNumber: f_dwNumber,
            wDeviceType: f_wDeviceType,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpstrReturn, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwRetSize, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwNumber, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.wDeviceType, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub struct MCI_SYSINFO_PARMSW {
    pub dwCallback: PtrRepr,
    pub lpstrReturn: PWSTR,
    pub dwRetSize: u32,
    pub dwNumber: u32,
    pub wDeviceType: u32,
}
impl ::core::marker::Copy for MCI_SYSINFO_PARMSW {}
impl ::core::clone::Clone for MCI_SYSINFO_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_SYSINFO_PARMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_SYSINFO_PARMSW")
            .field("dwCallback", &self.dwCallback)
            .field("lpstrReturn", &self.lpstrReturn)
            .field("dwRetSize", &self.dwRetSize)
            .field("dwNumber", &self.dwNumber)
            .field("wDeviceType", &self.wDeviceType)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_SYSINFO_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.lpstrReturn == other.lpstrReturn
            && self.dwRetSize == other.dwRetSize
            && self.dwNumber == other.dwNumber
            && self.wDeviceType == other.wDeviceType
    }
}
impl ::core::cmp::Eq for MCI_SYSINFO_PARMSW {}
impl FromIntoMemory for MCI_SYSINFO_PARMSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpstrReturn = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwRetSize = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwNumber = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_wDeviceType = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            dwCallback: f_dwCallback,
            lpstrReturn: f_lpstrReturn,
            dwRetSize: f_dwRetSize,
            dwNumber: f_dwNumber,
            wDeviceType: f_wDeviceType,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpstrReturn, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwRetSize, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwNumber, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.wDeviceType, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub const MCI_SYSINFO_QUANTITY: i32 = 256i32;
pub const MCI_TEST: i32 = 32i32;
pub const MCI_TO: i32 = 8i32;
pub const MCI_TRACK: i32 = 16i32;
pub const MCI_TRUE: u32 = 532u32;
pub const MCI_UNDO: u32 = 2169u32;
pub const MCI_UNFREEZE: u32 = 2117u32;
pub const MCI_UPDATE: u32 = 2132u32;
pub const MCI_USER_MESSAGES: u32 = 3072u32;
pub struct MCI_VD_ESCAPE_PARMSA {
    pub dwCallback: PtrRepr,
    pub lpstrCommand: PCSTR,
}
impl ::core::marker::Copy for MCI_VD_ESCAPE_PARMSA {}
impl ::core::clone::Clone for MCI_VD_ESCAPE_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_VD_ESCAPE_PARMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_VD_ESCAPE_PARMSA")
            .field("dwCallback", &self.dwCallback)
            .field("lpstrCommand", &self.lpstrCommand)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_VD_ESCAPE_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback && self.lpstrCommand == other.lpstrCommand
    }
}
impl ::core::cmp::Eq for MCI_VD_ESCAPE_PARMSA {}
impl FromIntoMemory for MCI_VD_ESCAPE_PARMSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpstrCommand = <PCSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            dwCallback: f_dwCallback,
            lpstrCommand: f_lpstrCommand,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpstrCommand, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct MCI_VD_ESCAPE_PARMSW {
    pub dwCallback: PtrRepr,
    pub lpstrCommand: PCWSTR,
}
impl ::core::marker::Copy for MCI_VD_ESCAPE_PARMSW {}
impl ::core::clone::Clone for MCI_VD_ESCAPE_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_VD_ESCAPE_PARMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_VD_ESCAPE_PARMSW")
            .field("dwCallback", &self.dwCallback)
            .field("lpstrCommand", &self.lpstrCommand)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_VD_ESCAPE_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback && self.lpstrCommand == other.lpstrCommand
    }
}
impl ::core::cmp::Eq for MCI_VD_ESCAPE_PARMSW {}
impl FromIntoMemory for MCI_VD_ESCAPE_PARMSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpstrCommand = <PCWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            dwCallback: f_dwCallback,
            lpstrCommand: f_lpstrCommand,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpstrCommand, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub const MCI_VD_ESCAPE_STRING: i32 = 256i32;
pub const MCI_VD_FORMAT_TRACK: u32 = 16385u32;
pub const MCI_VD_FORMAT_TRACK_S: u32 = 1029u32;
pub const MCI_VD_GETDEVCAPS_CAN_REVERSE: i32 = 16386i32;
pub const MCI_VD_GETDEVCAPS_CAV: i32 = 131072i32;
pub const MCI_VD_GETDEVCAPS_CLV: i32 = 65536i32;
pub const MCI_VD_GETDEVCAPS_FAST_RATE: i32 = 16387i32;
pub const MCI_VD_GETDEVCAPS_NORMAL_RATE: i32 = 16389i32;
pub const MCI_VD_GETDEVCAPS_SLOW_RATE: i32 = 16388i32;
pub const MCI_VD_MEDIA_CAV: u32 = 1027u32;
pub const MCI_VD_MEDIA_CLV: u32 = 1026u32;
pub const MCI_VD_MEDIA_OTHER: u32 = 1028u32;
pub const MCI_VD_MODE_PARK: u32 = 1025u32;
pub const MCI_VD_PLAY_FAST: i32 = 131072i32;
pub struct MCI_VD_PLAY_PARMS {
    pub dwCallback: PtrRepr,
    pub dwFrom: u32,
    pub dwTo: u32,
    pub dwSpeed: u32,
}
impl ::core::marker::Copy for MCI_VD_PLAY_PARMS {}
impl ::core::clone::Clone for MCI_VD_PLAY_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_VD_PLAY_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_VD_PLAY_PARMS")
            .field("dwCallback", &self.dwCallback)
            .field("dwFrom", &self.dwFrom)
            .field("dwTo", &self.dwTo)
            .field("dwSpeed", &self.dwSpeed)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_VD_PLAY_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.dwFrom == other.dwFrom
            && self.dwTo == other.dwTo
            && self.dwSpeed == other.dwSpeed
    }
}
impl ::core::cmp::Eq for MCI_VD_PLAY_PARMS {}
impl FromIntoMemory for MCI_VD_PLAY_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFrom = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwTo = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwSpeed = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwFrom: f_dwFrom,
            dwTo: f_dwTo,
            dwSpeed: f_dwSpeed,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFrom, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwTo, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwSpeed, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub const MCI_VD_PLAY_REVERSE: i32 = 65536i32;
pub const MCI_VD_PLAY_SCAN: i32 = 524288i32;
pub const MCI_VD_PLAY_SLOW: i32 = 1048576i32;
pub const MCI_VD_PLAY_SPEED: i32 = 262144i32;
pub const MCI_VD_SEEK_REVERSE: i32 = 65536i32;
pub const MCI_VD_SPIN_DOWN: i32 = 131072i32;
pub const MCI_VD_SPIN_UP: i32 = 65536i32;
pub const MCI_VD_STATUS_DISC_SIZE: i32 = 16390i32;
pub const MCI_VD_STATUS_FORWARD: i32 = 16387i32;
pub const MCI_VD_STATUS_MEDIA_TYPE: i32 = 16388i32;
pub const MCI_VD_STATUS_SIDE: i32 = 16389i32;
pub const MCI_VD_STATUS_SPEED: i32 = 16386i32;
pub const MCI_VD_STEP_FRAMES: i32 = 65536i32;
pub struct MCI_VD_STEP_PARMS {
    pub dwCallback: PtrRepr,
    pub dwFrames: u32,
}
impl ::core::marker::Copy for MCI_VD_STEP_PARMS {}
impl ::core::clone::Clone for MCI_VD_STEP_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_VD_STEP_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_VD_STEP_PARMS")
            .field("dwCallback", &self.dwCallback)
            .field("dwFrames", &self.dwFrames)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_VD_STEP_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback && self.dwFrames == other.dwFrames
    }
}
impl ::core::cmp::Eq for MCI_VD_STEP_PARMS {}
impl FromIntoMemory for MCI_VD_STEP_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFrames = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwFrames: f_dwFrames,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFrames, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub const MCI_VD_STEP_REVERSE: i32 = 131072i32;
pub const MCI_WAIT: i32 = 2i32;
pub struct MCI_WAVE_DELETE_PARMS {
    pub dwCallback: PtrRepr,
    pub dwFrom: u32,
    pub dwTo: u32,
}
impl ::core::marker::Copy for MCI_WAVE_DELETE_PARMS {}
impl ::core::clone::Clone for MCI_WAVE_DELETE_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_WAVE_DELETE_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_WAVE_DELETE_PARMS")
            .field("dwCallback", &self.dwCallback)
            .field("dwFrom", &self.dwFrom)
            .field("dwTo", &self.dwTo)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_WAVE_DELETE_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.dwFrom == other.dwFrom
            && self.dwTo == other.dwTo
    }
}
impl ::core::cmp::Eq for MCI_WAVE_DELETE_PARMS {}
impl FromIntoMemory for MCI_WAVE_DELETE_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFrom = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwTo = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            dwCallback: f_dwCallback,
            dwFrom: f_dwFrom,
            dwTo: f_dwTo,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFrom, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwTo, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub const MCI_WAVE_GETDEVCAPS_INPUTS: i32 = 16385i32;
pub const MCI_WAVE_GETDEVCAPS_OUTPUTS: i32 = 16386i32;
pub const MCI_WAVE_INPUT: i32 = 4194304i32;
pub const MCI_WAVE_MAPPER: u32 = 1153u32;
pub const MCI_WAVE_OPEN_BUFFER: i32 = 65536i32;
pub struct MCI_WAVE_OPEN_PARMSA {
    pub dwCallback: PtrRepr,
    pub wDeviceID: u32,
    pub lpstrDeviceType: PCSTR,
    pub lpstrElementName: PCSTR,
    pub lpstrAlias: PCSTR,
    pub dwBufferSeconds: u32,
}
impl ::core::marker::Copy for MCI_WAVE_OPEN_PARMSA {}
impl ::core::clone::Clone for MCI_WAVE_OPEN_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_WAVE_OPEN_PARMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_WAVE_OPEN_PARMSA")
            .field("dwCallback", &self.dwCallback)
            .field("wDeviceID", &self.wDeviceID)
            .field("lpstrDeviceType", &self.lpstrDeviceType)
            .field("lpstrElementName", &self.lpstrElementName)
            .field("lpstrAlias", &self.lpstrAlias)
            .field("dwBufferSeconds", &self.dwBufferSeconds)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_WAVE_OPEN_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.wDeviceID == other.wDeviceID
            && self.lpstrDeviceType == other.lpstrDeviceType
            && self.lpstrElementName == other.lpstrElementName
            && self.lpstrAlias == other.lpstrAlias
            && self.dwBufferSeconds == other.dwBufferSeconds
    }
}
impl ::core::cmp::Eq for MCI_WAVE_OPEN_PARMSA {}
impl FromIntoMemory for MCI_WAVE_OPEN_PARMSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_wDeviceID = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpstrDeviceType = <PCSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpstrElementName = <PCSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpstrAlias = <PCSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwBufferSeconds = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            dwCallback: f_dwCallback,
            wDeviceID: f_wDeviceID,
            lpstrDeviceType: f_lpstrDeviceType,
            lpstrElementName: f_lpstrElementName,
            lpstrAlias: f_lpstrAlias,
            dwBufferSeconds: f_dwBufferSeconds,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.wDeviceID, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpstrDeviceType, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpstrElementName, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpstrAlias, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwBufferSeconds, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct MCI_WAVE_OPEN_PARMSW {
    pub dwCallback: PtrRepr,
    pub wDeviceID: u32,
    pub lpstrDeviceType: PCWSTR,
    pub lpstrElementName: PCWSTR,
    pub lpstrAlias: PCWSTR,
    pub dwBufferSeconds: u32,
}
impl ::core::marker::Copy for MCI_WAVE_OPEN_PARMSW {}
impl ::core::clone::Clone for MCI_WAVE_OPEN_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_WAVE_OPEN_PARMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_WAVE_OPEN_PARMSW")
            .field("dwCallback", &self.dwCallback)
            .field("wDeviceID", &self.wDeviceID)
            .field("lpstrDeviceType", &self.lpstrDeviceType)
            .field("lpstrElementName", &self.lpstrElementName)
            .field("lpstrAlias", &self.lpstrAlias)
            .field("dwBufferSeconds", &self.dwBufferSeconds)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_WAVE_OPEN_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.wDeviceID == other.wDeviceID
            && self.lpstrDeviceType == other.lpstrDeviceType
            && self.lpstrElementName == other.lpstrElementName
            && self.lpstrAlias == other.lpstrAlias
            && self.dwBufferSeconds == other.dwBufferSeconds
    }
}
impl ::core::cmp::Eq for MCI_WAVE_OPEN_PARMSW {}
impl FromIntoMemory for MCI_WAVE_OPEN_PARMSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_wDeviceID = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpstrDeviceType = <PCWSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpstrElementName = <PCWSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpstrAlias = <PCWSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwBufferSeconds = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            dwCallback: f_dwCallback,
            wDeviceID: f_wDeviceID,
            lpstrDeviceType: f_lpstrDeviceType,
            lpstrElementName: f_lpstrElementName,
            lpstrAlias: f_lpstrAlias,
            dwBufferSeconds: f_dwBufferSeconds,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.wDeviceID, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpstrDeviceType, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpstrElementName, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpstrAlias, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwBufferSeconds, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub const MCI_WAVE_OUTPUT: i32 = 8388608i32;
pub const MCI_WAVE_PCM: u32 = 1152u32;
pub const MCI_WAVE_SET_ANYINPUT: i32 = 67108864i32;
pub const MCI_WAVE_SET_ANYOUTPUT: i32 = 134217728i32;
pub const MCI_WAVE_SET_AVGBYTESPERSEC: i32 = 524288i32;
pub const MCI_WAVE_SET_BITSPERSAMPLE: i32 = 2097152i32;
pub const MCI_WAVE_SET_BLOCKALIGN: i32 = 1048576i32;
pub const MCI_WAVE_SET_CHANNELS: i32 = 131072i32;
pub const MCI_WAVE_SET_FORMATTAG: i32 = 65536i32;
pub struct MCI_WAVE_SET_PARMS {
    pub dwCallback: PtrRepr,
    pub dwTimeFormat: u32,
    pub dwAudio: u32,
    pub wInput: u32,
    pub wOutput: u32,
    pub wFormatTag: u16,
    pub wReserved2: u16,
    pub nChannels: u16,
    pub wReserved3: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
    pub wReserved4: u16,
    pub wBitsPerSample: u16,
    pub wReserved5: u16,
}
impl ::core::marker::Copy for MCI_WAVE_SET_PARMS {}
impl ::core::clone::Clone for MCI_WAVE_SET_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCI_WAVE_SET_PARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCI_WAVE_SET_PARMS")
            .field("dwCallback", &self.dwCallback)
            .field("dwTimeFormat", &self.dwTimeFormat)
            .field("dwAudio", &self.dwAudio)
            .field("wInput", &self.wInput)
            .field("wOutput", &self.wOutput)
            .field("wFormatTag", &self.wFormatTag)
            .field("wReserved2", &self.wReserved2)
            .field("nChannels", &self.nChannels)
            .field("wReserved3", &self.wReserved3)
            .field("nSamplesPerSec", &self.nSamplesPerSec)
            .field("nAvgBytesPerSec", &self.nAvgBytesPerSec)
            .field("nBlockAlign", &self.nBlockAlign)
            .field("wReserved4", &self.wReserved4)
            .field("wBitsPerSample", &self.wBitsPerSample)
            .field("wReserved5", &self.wReserved5)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MCI_WAVE_SET_PARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallback == other.dwCallback
            && self.dwTimeFormat == other.dwTimeFormat
            && self.dwAudio == other.dwAudio
            && self.wInput == other.wInput
            && self.wOutput == other.wOutput
            && self.wFormatTag == other.wFormatTag
            && self.wReserved2 == other.wReserved2
            && self.nChannels == other.nChannels
            && self.wReserved3 == other.wReserved3
            && self.nSamplesPerSec == other.nSamplesPerSec
            && self.nAvgBytesPerSec == other.nAvgBytesPerSec
            && self.nBlockAlign == other.nBlockAlign
            && self.wReserved4 == other.wReserved4
            && self.wBitsPerSample == other.wBitsPerSample
            && self.wReserved5 == other.wReserved5
    }
}
impl ::core::cmp::Eq for MCI_WAVE_SET_PARMS {}
impl FromIntoMemory for MCI_WAVE_SET_PARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 44);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwTimeFormat = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwAudio = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_wInput = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_wOutput = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_wFormatTag = <u16 as FromIntoMemory>::from_bytes(&from[20..20 + 2]);
        let f_wReserved2 = <u16 as FromIntoMemory>::from_bytes(&from[22..22 + 2]);
        let f_nChannels = <u16 as FromIntoMemory>::from_bytes(&from[24..24 + 2]);
        let f_wReserved3 = <u16 as FromIntoMemory>::from_bytes(&from[26..26 + 2]);
        let f_nSamplesPerSec = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_nAvgBytesPerSec = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_nBlockAlign = <u16 as FromIntoMemory>::from_bytes(&from[36..36 + 2]);
        let f_wReserved4 = <u16 as FromIntoMemory>::from_bytes(&from[38..38 + 2]);
        let f_wBitsPerSample = <u16 as FromIntoMemory>::from_bytes(&from[40..40 + 2]);
        let f_wReserved5 = <u16 as FromIntoMemory>::from_bytes(&from[42..42 + 2]);
        Self {
            dwCallback: f_dwCallback,
            dwTimeFormat: f_dwTimeFormat,
            dwAudio: f_dwAudio,
            wInput: f_wInput,
            wOutput: f_wOutput,
            wFormatTag: f_wFormatTag,
            wReserved2: f_wReserved2,
            nChannels: f_nChannels,
            wReserved3: f_wReserved3,
            nSamplesPerSec: f_nSamplesPerSec,
            nAvgBytesPerSec: f_nAvgBytesPerSec,
            nBlockAlign: f_nBlockAlign,
            wReserved4: f_wReserved4,
            wBitsPerSample: f_wBitsPerSample,
            wReserved5: f_wReserved5,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 44);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwTimeFormat, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwAudio, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.wInput, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.wOutput, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.wFormatTag, &mut into[20..20 + 2]);
        FromIntoMemory::into_bytes(self.wReserved2, &mut into[22..22 + 2]);
        FromIntoMemory::into_bytes(self.nChannels, &mut into[24..24 + 2]);
        FromIntoMemory::into_bytes(self.wReserved3, &mut into[26..26 + 2]);
        FromIntoMemory::into_bytes(self.nSamplesPerSec, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.nAvgBytesPerSec, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.nBlockAlign, &mut into[36..36 + 2]);
        FromIntoMemory::into_bytes(self.wReserved4, &mut into[38..38 + 2]);
        FromIntoMemory::into_bytes(self.wBitsPerSample, &mut into[40..40 + 2]);
        FromIntoMemory::into_bytes(self.wReserved5, &mut into[42..42 + 2]);
    }
    fn size() -> usize {
        44
    }
}
pub const MCI_WAVE_SET_SAMPLESPERSEC: i32 = 262144i32;
pub const MCI_WAVE_STATUS_AVGBYTESPERSEC: i32 = 16388i32;
pub const MCI_WAVE_STATUS_BITSPERSAMPLE: i32 = 16390i32;
pub const MCI_WAVE_STATUS_BLOCKALIGN: i32 = 16389i32;
pub const MCI_WAVE_STATUS_CHANNELS: i32 = 16386i32;
pub const MCI_WAVE_STATUS_FORMATTAG: i32 = 16385i32;
pub const MCI_WAVE_STATUS_LEVEL: i32 = 16391i32;
pub const MCI_WAVE_STATUS_SAMPLESPERSEC: i32 = 16387i32;
pub const MCI_WHERE: u32 = 2115u32;
pub const MCI_WINDOW: u32 = 2113u32;
pub const MCMADM_E_REGKEY_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-1072889750i32);
pub const MCMADM_I_NO_EVENTS: crate::core::HRESULT = crate::core::HRESULT(1074593897i32);
pub struct MEDIASPACEADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wRevision: u16,
}
impl ::core::marker::Copy for MEDIASPACEADPCMWAVEFORMAT {}
impl ::core::clone::Clone for MEDIASPACEADPCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MEDIASPACEADPCMWAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEDIASPACEADPCMWAVEFORMAT")
            .field("wfx", &self.wfx)
            .field("wRevision", &self.wRevision)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MEDIASPACEADPCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx && self.wRevision == other.wRevision
    }
}
impl ::core::cmp::Eq for MEDIASPACEADPCMWAVEFORMAT {}
impl FromIntoMemory for MEDIASPACEADPCMWAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        let f_wRevision = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        Self {
            wfx: f_wfx,
            wRevision: f_wRevision,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
        FromIntoMemory::into_bytes(self.wRevision, &mut into[18..18 + 2]);
    }
    fn size() -> usize {
        20
    }
}
pub const MIDIMAPPER_S: u32 = 1227u32;
pub struct MIDIOPENSTRMID {
    pub dwStreamID: u32,
    pub uDeviceID: u32,
}
impl ::core::marker::Copy for MIDIOPENSTRMID {}
impl ::core::clone::Clone for MIDIOPENSTRMID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIDIOPENSTRMID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDIOPENSTRMID")
            .field("dwStreamID", &self.dwStreamID)
            .field("uDeviceID", &self.uDeviceID)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIDIOPENSTRMID {
    fn eq(&self, other: &Self) -> bool {
        self.dwStreamID == other.dwStreamID && self.uDeviceID == other.uDeviceID
    }
}
impl ::core::cmp::Eq for MIDIOPENSTRMID {}
impl FromIntoMemory for MIDIOPENSTRMID {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_dwStreamID = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_uDeviceID = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            dwStreamID: f_dwStreamID,
            uDeviceID: f_uDeviceID,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.dwStreamID, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.uDeviceID, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub const MIDI_IO_COOKED: i32 = 2i32;
pub const MIDI_IO_PACKED: i32 = 0i32;
pub const MIDM_ADDBUFFER: u32 = 59u32;
pub const MIDM_CLOSE: u32 = 56u32;
pub const MIDM_GETDEVCAPS: u32 = 54u32;
pub const MIDM_GETNUMDEVS: u32 = 53u32;
pub const MIDM_INIT: u32 = 100u32;
pub const MIDM_INIT_EX: u32 = 104u32;
pub const MIDM_MAPPER: u32 = 8192u32;
pub const MIDM_OPEN: u32 = 55u32;
pub const MIDM_PREPARE: u32 = 57u32;
pub const MIDM_RESET: u32 = 62u32;
pub const MIDM_START: u32 = 60u32;
pub const MIDM_STOP: u32 = 61u32;
pub const MIDM_UNPREPARE: u32 = 58u32;
pub const MIDM_USER: u32 = 16384u32;
pub const MIXERCONTROL_CONTROLTYPE_SRS_MTS: u32 = 536936454u32;
pub const MIXERCONTROL_CONTROLTYPE_SRS_ONOFF: u32 = 536936455u32;
pub const MIXERCONTROL_CONTROLTYPE_SRS_SYNTHSELECT: u32 = 536936456u32;
pub struct MIXEROPENDESC {
    pub hmx: super::Audio::HMIXER,
    pub pReserved0: MutPtr<::core::ffi::c_void>,
    pub dwCallback: PtrRepr,
    pub dwInstance: PtrRepr,
    pub dnDevNode: PtrRepr,
}
impl ::core::marker::Copy for MIXEROPENDESC {}
impl ::core::clone::Clone for MIXEROPENDESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXEROPENDESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXEROPENDESC")
            .field("hmx", &self.hmx)
            .field("pReserved0", &self.pReserved0)
            .field("dwCallback", &self.dwCallback)
            .field("dwInstance", &self.dwInstance)
            .field("dnDevNode", &self.dnDevNode)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXEROPENDESC {
    fn eq(&self, other: &Self) -> bool {
        self.hmx == other.hmx
            && self.pReserved0 == other.pReserved0
            && self.dwCallback == other.dwCallback
            && self.dwInstance == other.dwInstance
            && self.dnDevNode == other.dnDevNode
    }
}
impl ::core::cmp::Eq for MIXEROPENDESC {}
impl FromIntoMemory for MIXEROPENDESC {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_hmx = <super::Audio::HMIXER as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_pReserved0 =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwInstance = <PtrRepr as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dnDevNode = <PtrRepr as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            hmx: f_hmx,
            pReserved0: f_pReserved0,
            dwCallback: f_dwCallback,
            dwInstance: f_dwInstance,
            dnDevNode: f_dnDevNode,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.hmx, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.pReserved0, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwInstance, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dnDevNode, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub struct MMCKINFO {
    pub ckid: u32,
    pub cksize: u32,
    pub fccType: u32,
    pub dwDataOffset: u32,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for MMCKINFO {}
impl ::core::clone::Clone for MMCKINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MMCKINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMCKINFO")
            .field("ckid", &self.ckid)
            .field("cksize", &self.cksize)
            .field("fccType", &self.fccType)
            .field("dwDataOffset", &self.dwDataOffset)
            .field("dwFlags", &self.dwFlags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MMCKINFO {
    fn eq(&self, other: &Self) -> bool {
        self.ckid == other.ckid
            && self.cksize == other.cksize
            && self.fccType == other.fccType
            && self.dwDataOffset == other.dwDataOffset
            && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for MMCKINFO {}
impl FromIntoMemory for MMCKINFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_ckid = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_cksize = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_fccType = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwDataOffset = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            ckid: f_ckid,
            cksize: f_cksize,
            fccType: f_fccType,
            dwDataOffset: f_dwDataOffset,
            dwFlags: f_dwFlags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.ckid, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.cksize, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.fccType, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwDataOffset, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub const MMIOERR_ACCESSDENIED: u32 = 268u32;
pub const MMIOERR_BASE: u32 = 256u32;
pub const MMIOERR_CANNOTCLOSE: u32 = 260u32;
pub const MMIOERR_CANNOTEXPAND: u32 = 264u32;
pub const MMIOERR_CANNOTOPEN: u32 = 259u32;
pub const MMIOERR_CANNOTREAD: u32 = 261u32;
pub const MMIOERR_CANNOTSEEK: u32 = 263u32;
pub const MMIOERR_CANNOTWRITE: u32 = 262u32;
pub const MMIOERR_CHUNKNOTFOUND: u32 = 265u32;
pub const MMIOERR_FILENOTFOUND: u32 = 257u32;
pub const MMIOERR_INVALIDFILE: u32 = 272u32;
pub const MMIOERR_NETWORKERROR: u32 = 270u32;
pub const MMIOERR_OUTOFMEMORY: u32 = 258u32;
pub const MMIOERR_PATHNOTFOUND: u32 = 267u32;
pub const MMIOERR_SHARINGVIOLATION: u32 = 269u32;
pub const MMIOERR_TOOMANYOPENFILES: u32 = 271u32;
pub const MMIOERR_UNBUFFERED: u32 = 266u32;
pub struct MMIOINFO {
    pub dwFlags: u32,
    pub fccIOProc: u32,
    pub pIOProc: LPMMIOPROC,
    pub wErrorRet: u32,
    pub htask: super::HTASK,
    pub cchBuffer: i32,
    pub pchBuffer: MutPtr<i8>,
    pub pchNext: MutPtr<i8>,
    pub pchEndRead: MutPtr<i8>,
    pub pchEndWrite: MutPtr<i8>,
    pub lBufOffset: i32,
    pub lDiskOffset: i32,
    pub adwInfo: [u32; 3],
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub hmmio: HMMIO,
}
impl ::core::marker::Copy for MMIOINFO {}
impl ::core::clone::Clone for MMIOINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MMIOINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMIOINFO")
            .field("dwFlags", &self.dwFlags)
            .field("fccIOProc", &self.fccIOProc)
            .field("pIOProc", &self.pIOProc)
            .field("wErrorRet", &self.wErrorRet)
            .field("htask", &self.htask)
            .field("cchBuffer", &self.cchBuffer)
            .field("pchBuffer", &self.pchBuffer)
            .field("pchNext", &self.pchNext)
            .field("pchEndRead", &self.pchEndRead)
            .field("pchEndWrite", &self.pchEndWrite)
            .field("lBufOffset", &self.lBufOffset)
            .field("lDiskOffset", &self.lDiskOffset)
            .field("adwInfo", &self.adwInfo)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("hmmio", &self.hmmio)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MMIOINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags
            && self.fccIOProc == other.fccIOProc
            && self.pIOProc == other.pIOProc
            && self.wErrorRet == other.wErrorRet
            && self.htask == other.htask
            && self.cchBuffer == other.cchBuffer
            && self.pchBuffer == other.pchBuffer
            && self.pchNext == other.pchNext
            && self.pchEndRead == other.pchEndRead
            && self.pchEndWrite == other.pchEndWrite
            && self.lBufOffset == other.lBufOffset
            && self.lDiskOffset == other.lDiskOffset
            && self.adwInfo == other.adwInfo
            && self.dwReserved1 == other.dwReserved1
            && self.dwReserved2 == other.dwReserved2
            && self.hmmio == other.hmmio
    }
}
impl ::core::cmp::Eq for MMIOINFO {}
impl FromIntoMemory for MMIOINFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 72);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_fccIOProc = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_pIOProc = <LPMMIOPROC as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_wErrorRet = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_htask = <super::HTASK as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_cchBuffer = <i32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_pchBuffer = <MutPtr<i8> as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_pchNext = <MutPtr<i8> as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_pchEndRead = <MutPtr<i8> as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_pchEndWrite = <MutPtr<i8> as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_lBufOffset = <i32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_lDiskOffset = <i32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_adwInfo = <[u32; 3] as FromIntoMemory>::from_bytes(&from[48..48 + 12]);
        let f_dwReserved1 = <u32 as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_dwReserved2 = <u32 as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        let f_hmmio = <HMMIO as FromIntoMemory>::from_bytes(&from[68..68 + 4]);
        Self {
            dwFlags: f_dwFlags,
            fccIOProc: f_fccIOProc,
            pIOProc: f_pIOProc,
            wErrorRet: f_wErrorRet,
            htask: f_htask,
            cchBuffer: f_cchBuffer,
            pchBuffer: f_pchBuffer,
            pchNext: f_pchNext,
            pchEndRead: f_pchEndRead,
            pchEndWrite: f_pchEndWrite,
            lBufOffset: f_lBufOffset,
            lDiskOffset: f_lDiskOffset,
            adwInfo: f_adwInfo,
            dwReserved1: f_dwReserved1,
            dwReserved2: f_dwReserved2,
            hmmio: f_hmmio,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 72);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.fccIOProc, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.pIOProc, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.wErrorRet, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.htask, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.cchBuffer, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.pchBuffer, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.pchNext, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.pchEndRead, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.pchEndWrite, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.lBufOffset, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.lDiskOffset, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.adwInfo, &mut into[48..48 + 12]);
        FromIntoMemory::into_bytes(self.dwReserved1, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.dwReserved2, &mut into[64..64 + 4]);
        FromIntoMemory::into_bytes(self.hmmio, &mut into[68..68 + 4]);
    }
    fn size() -> usize {
        72
    }
}
pub const MMIOM_CLOSE: u32 = 4u32;
pub const MMIOM_OPEN: u32 = 3u32;
pub const MMIOM_READ: u32 = 0u32;
pub const MMIOM_RENAME: u32 = 6u32;
pub const MMIOM_SEEK: u32 = 2u32;
pub const MMIOM_USER: u32 = 32768u32;
pub const MMIOM_WRITE: u32 = 1u32;
pub const MMIOM_WRITEFLUSH: u32 = 5u32;
pub const MMIO_ALLOCBUF: u32 = 65536u32;
pub const MMIO_COMPAT: u32 = 0u32;
pub const MMIO_CREATE: u32 = 4096u32;
pub const MMIO_CREATELIST: u32 = 64u32;
pub const MMIO_CREATERIFF: u32 = 32u32;
pub const MMIO_DEFAULTBUFFER: u32 = 8192u32;
pub const MMIO_DELETE: u32 = 512u32;
pub const MMIO_DENYNONE: u32 = 64u32;
pub const MMIO_DENYREAD: u32 = 48u32;
pub const MMIO_DENYWRITE: u32 = 32u32;
pub const MMIO_DIRTY: u32 = 268435456u32;
pub const MMIO_EMPTYBUF: u32 = 16u32;
pub const MMIO_EXCLUSIVE: u32 = 16u32;
pub const MMIO_EXIST: u32 = 16384u32;
pub const MMIO_FHOPEN: u32 = 16u32;
pub const MMIO_FINDCHUNK: u32 = 16u32;
pub const MMIO_FINDLIST: u32 = 64u32;
pub const MMIO_FINDPROC: u32 = 262144u32;
pub const MMIO_FINDRIFF: u32 = 32u32;
pub const MMIO_GETTEMP: u32 = 131072u32;
pub const MMIO_GLOBALPROC: u32 = 268435456u32;
pub const MMIO_INSTALLPROC: u32 = 65536u32;
pub const MMIO_PARSE: u32 = 256u32;
pub const MMIO_READ: u32 = 0u32;
pub const MMIO_READWRITE: u32 = 2u32;
pub const MMIO_REMOVEPROC: u32 = 131072u32;
pub const MMIO_RWMODE: u32 = 3u32;
pub const MMIO_SHAREMODE: u32 = 112u32;
pub const MMIO_TOUPPER: u32 = 16u32;
pub const MMIO_UNICODEPROC: u32 = 16777216u32;
pub const MMIO_WRITE: u32 = 1u32;
pub const MM_3COM: u32 = 260u32;
pub const MM_3COM_CB_MIXER: u32 = 1u32;
pub const MM_3COM_CB_WAVEIN: u32 = 2u32;
pub const MM_3COM_CB_WAVEOUT: u32 = 3u32;
pub const MM_3DFX: u32 = 262u32;
pub const MM_AARDVARK: u32 = 11u32;
pub const MM_AARDVARK_STUDIO12_WAVEIN: u32 = 2u32;
pub const MM_AARDVARK_STUDIO12_WAVEOUT: u32 = 1u32;
pub const MM_AARDVARK_STUDIO88_WAVEIN: u32 = 4u32;
pub const MM_AARDVARK_STUDIO88_WAVEOUT: u32 = 3u32;
pub const MM_ACTIVEVOICE: u32 = 225u32;
pub const MM_ACTIVEVOICE_ACM_VOXADPCM: u32 = 1u32;
pub const MM_ACULAB: u32 = 14u32;
pub const MM_ADDX: u32 = 118u32;
pub const MM_ADDX_PCTV_AUX_CD: u32 = 5u32;
pub const MM_ADDX_PCTV_AUX_LINE: u32 = 6u32;
pub const MM_ADDX_PCTV_DIGITALMIX: u32 = 1u32;
pub const MM_ADDX_PCTV_MIXER: u32 = 4u32;
pub const MM_ADDX_PCTV_WAVEIN: u32 = 2u32;
pub const MM_ADDX_PCTV_WAVEOUT: u32 = 3u32;
pub const MM_ADLACC: u32 = 91u32;
pub const MM_ADMOS: u32 = 235u32;
pub const MM_ADMOS_FM_SYNTH: u32 = 1u32;
pub const MM_ADMOS_QS3AMIDIIN: u32 = 3u32;
pub const MM_ADMOS_QS3AMIDIOUT: u32 = 2u32;
pub const MM_ADMOS_QS3AWAVEIN: u32 = 5u32;
pub const MM_ADMOS_QS3AWAVEOUT: u32 = 4u32;
pub const MM_AHEAD: u32 = 77u32;
pub const MM_AHEAD_GENERIC: u32 = 4u32;
pub const MM_AHEAD_MULTISOUND: u32 = 1u32;
pub const MM_AHEAD_PROAUDIO: u32 = 3u32;
pub const MM_AHEAD_SOUNDBLASTER: u32 = 2u32;
pub const MM_ALARIS: u32 = 174u32;
pub const MM_ALDIGITAL: u32 = 143u32;
pub const MM_ALESIS: u32 = 243u32;
pub const MM_ALGOVISION: u32 = 266u32;
pub const MM_ALGOVISION_VB80AUX: u32 = 4u32;
pub const MM_ALGOVISION_VB80AUX2: u32 = 5u32;
pub const MM_ALGOVISION_VB80MIXER: u32 = 3u32;
pub const MM_ALGOVISION_VB80WAVEIN: u32 = 2u32;
pub const MM_ALGOVISION_VB80WAVEOUT: u32 = 1u32;
pub const MM_AMD: u32 = 146u32;
pub const MM_AMD_INTERWAVE_AUX1: u32 = 10u32;
pub const MM_AMD_INTERWAVE_AUX2: u32 = 11u32;
pub const MM_AMD_INTERWAVE_AUX_CD: u32 = 13u32;
pub const MM_AMD_INTERWAVE_AUX_MIC: u32 = 12u32;
pub const MM_AMD_INTERWAVE_EX_CD: u32 = 7u32;
pub const MM_AMD_INTERWAVE_EX_TELEPHONY: u32 = 16u32;
pub const MM_AMD_INTERWAVE_JOYSTICK: u32 = 6u32;
pub const MM_AMD_INTERWAVE_MIDIIN: u32 = 8u32;
pub const MM_AMD_INTERWAVE_MIDIOUT: u32 = 9u32;
pub const MM_AMD_INTERWAVE_MIXER1: u32 = 4u32;
pub const MM_AMD_INTERWAVE_MIXER2: u32 = 5u32;
pub const MM_AMD_INTERWAVE_MONO_IN: u32 = 14u32;
pub const MM_AMD_INTERWAVE_MONO_OUT: u32 = 15u32;
pub const MM_AMD_INTERWAVE_STEREO_ENHANCED: u32 = 19u32;
pub const MM_AMD_INTERWAVE_SYNTH: u32 = 3u32;
pub const MM_AMD_INTERWAVE_WAVEIN: u32 = 1u32;
pub const MM_AMD_INTERWAVE_WAVEOUT: u32 = 2u32;
pub const MM_AMD_INTERWAVE_WAVEOUT_BASE: u32 = 17u32;
pub const MM_AMD_INTERWAVE_WAVEOUT_TREBLE: u32 = 18u32;
pub const MM_ANALOGDEVICES: u32 = 252u32;
pub const MM_ANTEX: u32 = 31u32;
pub const MM_ANTEX_AUDIOPORT22_FEEDTHRU: u32 = 9u32;
pub const MM_ANTEX_AUDIOPORT22_WAVEIN: u32 = 7u32;
pub const MM_ANTEX_AUDIOPORT22_WAVEOUT: u32 = 8u32;
pub const MM_ANTEX_SX12_WAVEIN: u32 = 1u32;
pub const MM_ANTEX_SX12_WAVEOUT: u32 = 2u32;
pub const MM_ANTEX_SX15_WAVEIN: u32 = 3u32;
pub const MM_ANTEX_SX15_WAVEOUT: u32 = 4u32;
pub const MM_ANTEX_VP625_WAVEIN: u32 = 5u32;
pub const MM_ANTEX_VP625_WAVEOUT: u32 = 6u32;
pub const MM_APICOM: u32 = 116u32;
pub const MM_APPLE: u32 = 99u32;
pub const MM_APPS: u32 = 42u32;
pub const MM_APT: u32 = 56u32;
pub const MM_APT_ACE100CD: u32 = 1u32;
pub const MM_ARRAY: u32 = 231u32;
pub const MM_ARTISOFT: u32 = 20u32;
pub const MM_ARTISOFT_SBWAVEIN: u32 = 1u32;
pub const MM_ARTISOFT_SBWAVEOUT: u32 = 2u32;
pub const MM_AST: u32 = 64u32;
pub const MM_AST_MODEMWAVE_WAVEIN: u32 = 13u32;
pub const MM_AST_MODEMWAVE_WAVEOUT: u32 = 14u32;
pub const MM_ATI: u32 = 27u32;
pub const MM_ATT: u32 = 185u32;
pub const MM_ATT_G729A: u32 = 1u32;
pub const MM_ATT_MICROELECTRONICS: u32 = 139u32;
pub const MM_AU8820_AUX: u32 = 21u32;
pub const MM_AU8820_MIDIIN: u32 = 23u32;
pub const MM_AU8820_MIDIOUT: u32 = 22u32;
pub const MM_AU8820_MIXER: u32 = 20u32;
pub const MM_AU8820_SYNTH: u32 = 17u32;
pub const MM_AU8820_WAVEIN: u32 = 19u32;
pub const MM_AU8820_WAVEOUT: u32 = 18u32;
pub const MM_AU8830_AUX: u32 = 37u32;
pub const MM_AU8830_MIDIIN: u32 = 39u32;
pub const MM_AU8830_MIDIOUT: u32 = 38u32;
pub const MM_AU8830_MIXER: u32 = 36u32;
pub const MM_AU8830_SYNTH: u32 = 33u32;
pub const MM_AU8830_WAVEIN: u32 = 35u32;
pub const MM_AU8830_WAVEOUT: u32 = 34u32;
pub const MM_AUDIOFILE: u32 = 47u32;
pub const MM_AUDIOPT: u32 = 74u32;
pub const MM_AUDIOSCIENCE: u32 = 217u32;
pub const MM_AURAVISION: u32 = 80u32;
pub const MM_AUREAL: u32 = 181u32;
pub const MM_AUREAL_AU8820: u32 = 16u32;
pub const MM_AUREAL_AU8830: u32 = 32u32;
pub const MM_AZTECH: u32 = 52u32;
pub const MM_AZTECH_AUX: u32 = 404u32;
pub const MM_AZTECH_AUX_CD: u32 = 401u32;
pub const MM_AZTECH_AUX_LINE: u32 = 402u32;
pub const MM_AZTECH_AUX_MIC: u32 = 403u32;
pub const MM_AZTECH_DSP16_FMSYNTH: u32 = 68u32;
pub const MM_AZTECH_DSP16_WAVEIN: u32 = 65u32;
pub const MM_AZTECH_DSP16_WAVEOUT: u32 = 66u32;
pub const MM_AZTECH_DSP16_WAVESYNTH: u32 = 70u32;
pub const MM_AZTECH_FMSYNTH: u32 = 20u32;
pub const MM_AZTECH_MIDIIN: u32 = 4u32;
pub const MM_AZTECH_MIDIOUT: u32 = 3u32;
pub const MM_AZTECH_MIXER: u32 = 21u32;
pub const MM_AZTECH_NOVA16_MIXER: u32 = 73u32;
pub const MM_AZTECH_NOVA16_WAVEIN: u32 = 71u32;
pub const MM_AZTECH_NOVA16_WAVEOUT: u32 = 72u32;
pub const MM_AZTECH_PRO16_FMSYNTH: u32 = 38u32;
pub const MM_AZTECH_PRO16_WAVEIN: u32 = 33u32;
pub const MM_AZTECH_PRO16_WAVEOUT: u32 = 34u32;
pub const MM_AZTECH_WASH16_MIXER: u32 = 76u32;
pub const MM_AZTECH_WASH16_WAVEIN: u32 = 74u32;
pub const MM_AZTECH_WASH16_WAVEOUT: u32 = 75u32;
pub const MM_AZTECH_WAVEIN: u32 = 17u32;
pub const MM_AZTECH_WAVEOUT: u32 = 18u32;
pub const MM_BCB: u32 = 192u32;
pub const MM_BCB_NETBOARD_10: u32 = 1u32;
pub const MM_BCB_TT75_10: u32 = 2u32;
pub const MM_BECUBED: u32 = 10u32;
pub const MM_BERCOS: u32 = 199u32;
pub const MM_BERCOS_MIXER: u32 = 2u32;
pub const MM_BERCOS_WAVEIN: u32 = 1u32;
pub const MM_BERCOS_WAVEOUT: u32 = 3u32;
pub const MM_BERKOM: u32 = 189u32;
pub const MM_BINTEC: u32 = 12u32;
pub const MM_BINTEC_TAPI_WAVE: u32 = 1u32;
pub const MM_BROOKTREE: u32 = 121u32;
pub const MM_BTV_AUX_CD: u32 = 8u32;
pub const MM_BTV_AUX_LINE: u32 = 6u32;
pub const MM_BTV_AUX_MIC: u32 = 7u32;
pub const MM_BTV_DIGITALIN: u32 = 9u32;
pub const MM_BTV_DIGITALOUT: u32 = 10u32;
pub const MM_BTV_MIDIIN: u32 = 3u32;
pub const MM_BTV_MIDIOUT: u32 = 4u32;
pub const MM_BTV_MIDISYNTH: u32 = 5u32;
pub const MM_BTV_MIDIWAVESTREAM: u32 = 11u32;
pub const MM_BTV_MIXER: u32 = 12u32;
pub const MM_BTV_WAVEIN: u32 = 1u32;
pub const MM_BTV_WAVEOUT: u32 = 2u32;
pub const MM_CANAM: u32 = 148u32;
pub const MM_CANAM_CBXWAVEIN: u32 = 2u32;
pub const MM_CANAM_CBXWAVEOUT: u32 = 1u32;
pub const MM_CANOPUS: u32 = 49u32;
pub const MM_CANOPUS_ACM_DVREX: u32 = 1u32;
pub const MM_CASIO: u32 = 162u32;
pub const MM_CASIO_LSG_MIDIOUT: u32 = 3u32;
pub const MM_CASIO_WP150_MIDIIN: u32 = 2u32;
pub const MM_CASIO_WP150_MIDIOUT: u32 = 1u32;
pub const MM_CAT: u32 = 41u32;
pub const MM_CAT_WAVEOUT: u32 = 1u32;
pub const MM_CDPC_AUX: u32 = 119u32;
pub const MM_CDPC_MIDIIN: u32 = 114u32;
pub const MM_CDPC_MIDIOUT: u32 = 113u32;
pub const MM_CDPC_MIXER: u32 = 118u32;
pub const MM_CDPC_SYNTH: u32 = 115u32;
pub const MM_CDPC_WAVEIN: u32 = 117u32;
pub const MM_CDPC_WAVEOUT: u32 = 116u32;
pub const MM_CHROMATIC: u32 = 155u32;
pub const MM_CHROMATIC_M1: u32 = 1u32;
pub const MM_CHROMATIC_M1_AUX: u32 = 6u32;
pub const MM_CHROMATIC_M1_AUX_CD: u32 = 7u32;
pub const MM_CHROMATIC_M1_FMSYNTH: u32 = 4u32;
pub const MM_CHROMATIC_M1_MIDIIN: u32 = 8u32;
pub const MM_CHROMATIC_M1_MIDIOUT: u32 = 9u32;
pub const MM_CHROMATIC_M1_MIXER: u32 = 5u32;
pub const MM_CHROMATIC_M1_MPEGWAVEIN: u32 = 17u32;
pub const MM_CHROMATIC_M1_MPEGWAVEOUT: u32 = 18u32;
pub const MM_CHROMATIC_M1_WAVEIN: u32 = 2u32;
pub const MM_CHROMATIC_M1_WAVEOUT: u32 = 3u32;
pub const MM_CHROMATIC_M1_WTSYNTH: u32 = 16u32;
pub const MM_CHROMATIC_M2: u32 = 19u32;
pub const MM_CHROMATIC_M2_AUX: u32 = 24u32;
pub const MM_CHROMATIC_M2_AUX_CD: u32 = 25u32;
pub const MM_CHROMATIC_M2_FMSYNTH: u32 = 22u32;
pub const MM_CHROMATIC_M2_MIDIIN: u32 = 32u32;
pub const MM_CHROMATIC_M2_MIDIOUT: u32 = 33u32;
pub const MM_CHROMATIC_M2_MIXER: u32 = 23u32;
pub const MM_CHROMATIC_M2_MPEGWAVEIN: u32 = 35u32;
pub const MM_CHROMATIC_M2_MPEGWAVEOUT: u32 = 36u32;
pub const MM_CHROMATIC_M2_WAVEIN: u32 = 20u32;
pub const MM_CHROMATIC_M2_WAVEOUT: u32 = 21u32;
pub const MM_CHROMATIC_M2_WTSYNTH: u32 = 34u32;
pub const MM_CIRRUSLOGIC: u32 = 105u32;
pub const MM_COLORGRAPH: u32 = 179u32;
pub const MM_COMPAQ: u32 = 92u32;
pub const MM_COMPAQ_BB_WAVEAUX: u32 = 3u32;
pub const MM_COMPAQ_BB_WAVEIN: u32 = 1u32;
pub const MM_COMPAQ_BB_WAVEOUT: u32 = 2u32;
pub const MM_COMPUSIC: u32 = 89u32;
pub const MM_COMPUTER_FRIENDS: u32 = 45u32;
pub const MM_CONCEPTS: u32 = 108u32;
pub const MM_CONNECTIX: u32 = 158u32;
pub const MM_CONNECTIX_VIDEC_CODEC: u32 = 1u32;
pub const MM_CONTROLRES: u32 = 84u32;
pub const MM_COREDYNAMICS: u32 = 147u32;
pub const MM_COREDYNAMICS_DYNAGRAFX_VGA: u32 = 9u32;
pub const MM_COREDYNAMICS_DYNAGRAFX_WAVE_IN: u32 = 10u32;
pub const MM_COREDYNAMICS_DYNAGRAFX_WAVE_OUT: u32 = 11u32;
pub const MM_COREDYNAMICS_DYNAMIXHR: u32 = 1u32;
pub const MM_COREDYNAMICS_DYNASONIX_AUDIO_IN: u32 = 7u32;
pub const MM_COREDYNAMICS_DYNASONIX_AUDIO_OUT: u32 = 8u32;
pub const MM_COREDYNAMICS_DYNASONIX_MIDI_IN: u32 = 3u32;
pub const MM_COREDYNAMICS_DYNASONIX_MIDI_OUT: u32 = 4u32;
pub const MM_COREDYNAMICS_DYNASONIX_SYNTH: u32 = 2u32;
pub const MM_COREDYNAMICS_DYNASONIX_WAVE_IN: u32 = 5u32;
pub const MM_COREDYNAMICS_DYNASONIX_WAVE_OUT: u32 = 6u32;
pub const MM_CREATIVE: u32 = 2u32;
pub const MM_CREATIVE_AUX_CD: u32 = 401u32;
pub const MM_CREATIVE_AUX_LINE: u32 = 402u32;
pub const MM_CREATIVE_AUX_MASTER: u32 = 404u32;
pub const MM_CREATIVE_AUX_MIC: u32 = 403u32;
pub const MM_CREATIVE_AUX_MIDI: u32 = 407u32;
pub const MM_CREATIVE_AUX_PCSPK: u32 = 405u32;
pub const MM_CREATIVE_AUX_WAVE: u32 = 406u32;
pub const MM_CREATIVE_FMSYNTH_MONO: u32 = 301u32;
pub const MM_CREATIVE_FMSYNTH_STEREO: u32 = 302u32;
pub const MM_CREATIVE_MIDIIN: u32 = 202u32;
pub const MM_CREATIVE_MIDIOUT: u32 = 201u32;
pub const MM_CREATIVE_MIDI_AWE32: u32 = 303u32;
pub const MM_CREATIVE_PHNBLST_WAVEIN: u32 = 5u32;
pub const MM_CREATIVE_PHNBLST_WAVEOUT: u32 = 105u32;
pub const MM_CREATIVE_SB15_WAVEIN: u32 = 1u32;
pub const MM_CREATIVE_SB15_WAVEOUT: u32 = 101u32;
pub const MM_CREATIVE_SB16_MIXER: u32 = 409u32;
pub const MM_CREATIVE_SB20_WAVEIN: u32 = 2u32;
pub const MM_CREATIVE_SB20_WAVEOUT: u32 = 102u32;
pub const MM_CREATIVE_SBP16_WAVEIN: u32 = 4u32;
pub const MM_CREATIVE_SBP16_WAVEOUT: u32 = 104u32;
pub const MM_CREATIVE_SBPRO_MIXER: u32 = 408u32;
pub const MM_CREATIVE_SBPRO_WAVEIN: u32 = 3u32;
pub const MM_CREATIVE_SBPRO_WAVEOUT: u32 = 103u32;
pub const MM_CRYSTAL: u32 = 132u32;
pub const MM_CRYSTAL_CS4232_INPUTGAIN_AUX1: u32 = 13u32;
pub const MM_CRYSTAL_CS4232_INPUTGAIN_LOOP: u32 = 14u32;
pub const MM_CRYSTAL_CS4232_MIDIIN: u32 = 9u32;
pub const MM_CRYSTAL_CS4232_MIDIOUT: u32 = 10u32;
pub const MM_CRYSTAL_CS4232_WAVEAUX_AUX1: u32 = 4u32;
pub const MM_CRYSTAL_CS4232_WAVEAUX_AUX2: u32 = 5u32;
pub const MM_CRYSTAL_CS4232_WAVEAUX_LINE: u32 = 6u32;
pub const MM_CRYSTAL_CS4232_WAVEAUX_MASTER: u32 = 8u32;
pub const MM_CRYSTAL_CS4232_WAVEAUX_MONO: u32 = 7u32;
pub const MM_CRYSTAL_CS4232_WAVEIN: u32 = 1u32;
pub const MM_CRYSTAL_CS4232_WAVEMIXER: u32 = 3u32;
pub const MM_CRYSTAL_CS4232_WAVEOUT: u32 = 2u32;
pub const MM_CRYSTAL_NET: u32 = 154u32;
pub const MM_CRYSTAL_SOUND_FUSION_JOYSTICK: u32 = 26u32;
pub const MM_CRYSTAL_SOUND_FUSION_MIDIIN: u32 = 24u32;
pub const MM_CRYSTAL_SOUND_FUSION_MIDIOUT: u32 = 25u32;
pub const MM_CRYSTAL_SOUND_FUSION_MIXER: u32 = 23u32;
pub const MM_CRYSTAL_SOUND_FUSION_WAVEIN: u32 = 21u32;
pub const MM_CRYSTAL_SOUND_FUSION_WAVEOUT: u32 = 22u32;
pub const MM_CS: u32 = 242u32;
pub const MM_CYRIX: u32 = 6u32;
pub const MM_CYRIX_XAAUX: u32 = 6u32;
pub const MM_CYRIX_XAMIDIIN: u32 = 2u32;
pub const MM_CYRIX_XAMIDIOUT: u32 = 3u32;
pub const MM_CYRIX_XAMIXER: u32 = 7u32;
pub const MM_CYRIX_XASYNTH: u32 = 1u32;
pub const MM_CYRIX_XAWAVEIN: u32 = 4u32;
pub const MM_CYRIX_XAWAVEOUT: u32 = 5u32;
pub const MM_DATAFUSION: u32 = 196u32;
pub const MM_DATARAN: u32 = 232u32;
pub const MM_DDD: u32 = 151u32;
pub const MM_DDD_MIDILINK_MIDIIN: u32 = 1u32;
pub const MM_DDD_MIDILINK_MIDIOUT: u32 = 2u32;
pub const MM_DF_ACM_G726: u32 = 1u32;
pub const MM_DF_ACM_GSM610: u32 = 2u32;
pub const MM_DIACOUSTICS: u32 = 129u32;
pub const MM_DIACOUSTICS_DRUM_ACTION: u32 = 1u32;
pub const MM_DIALOGIC: u32 = 93u32;
pub const MM_DIAMONDMM: u32 = 163u32;
pub const MM_DICTAPHONE: u32 = 214u32;
pub const MM_DICTAPHONE_G726: u32 = 1u32;
pub const MM_DIGIGRAM: u32 = 227u32;
pub const MM_DIGITAL: u32 = 100u32;
pub const MM_DIGITAL_ACM_G723: u32 = 3u32;
pub const MM_DIGITAL_AUDIO_LABS: u32 = 136u32;
pub const MM_DIGITAL_AUDIO_LABS_CDLX: u32 = 19u32;
pub const MM_DIGITAL_AUDIO_LABS_CPRO: u32 = 17u32;
pub const MM_DIGITAL_AUDIO_LABS_CTDIF: u32 = 20u32;
pub const MM_DIGITAL_AUDIO_LABS_DOC: u32 = 2u32;
pub const MM_DIGITAL_AUDIO_LABS_TC: u32 = 1u32;
pub const MM_DIGITAL_AUDIO_LABS_V8: u32 = 16u32;
pub const MM_DIGITAL_AUDIO_LABS_VP: u32 = 18u32;
pub const MM_DIGITAL_AV320_WAVEIN: u32 = 1u32;
pub const MM_DIGITAL_AV320_WAVEOUT: u32 = 2u32;
pub const MM_DIGITAL_ICM_H261: u32 = 5u32;
pub const MM_DIGITAL_ICM_H263: u32 = 4u32;
pub const MM_DIMD_AUX_LINE: u32 = 9u32;
pub const MM_DIMD_DIRSOUND: u32 = 1u32;
pub const MM_DIMD_MIDIIN: u32 = 7u32;
pub const MM_DIMD_MIDIOUT: u32 = 8u32;
pub const MM_DIMD_MIXER: u32 = 10u32;
pub const MM_DIMD_PLATFORM: u32 = 0u32;
pub const MM_DIMD_VIRTJOY: u32 = 4u32;
pub const MM_DIMD_VIRTMPU: u32 = 2u32;
pub const MM_DIMD_VIRTSB: u32 = 3u32;
pub const MM_DIMD_WAVEIN: u32 = 5u32;
pub const MM_DIMD_WAVEOUT: u32 = 6u32;
pub const MM_DIMD_WSS_AUX: u32 = 21u32;
pub const MM_DIMD_WSS_MIXER: u32 = 17u32;
pub const MM_DIMD_WSS_SYNTH: u32 = 76u32;
pub const MM_DIMD_WSS_WAVEIN: u32 = 14u32;
pub const MM_DIMD_WSS_WAVEOUT: u32 = 15u32;
pub const MM_DOLBY: u32 = 78u32;
pub const MM_DPSINC: u32 = 191u32;
pub const MM_DSP_GROUP: u32 = 43u32;
pub const MM_DSP_GROUP_TRUESPEECH: u32 = 1u32;
pub const MM_DSP_SOLUTIONS: u32 = 25u32;
pub const MM_DSP_SOLUTIONS_AUX: u32 = 4u32;
pub const MM_DSP_SOLUTIONS_SYNTH: u32 = 3u32;
pub const MM_DSP_SOLUTIONS_WAVEIN: u32 = 2u32;
pub const MM_DSP_SOLUTIONS_WAVEOUT: u32 = 1u32;
pub const MM_DTS: u32 = 226u32;
pub const MM_DTS_DS: u32 = 1u32;
pub const MM_DUCK: u32 = 197u32;
pub const MM_DVISION: u32 = 165u32;
pub const MM_ECHO: u32 = 39u32;
pub const MM_ECHO_AUX: u32 = 6u32;
pub const MM_ECHO_MIDIIN: u32 = 5u32;
pub const MM_ECHO_MIDIOUT: u32 = 4u32;
pub const MM_ECHO_SYNTH: u32 = 1u32;
pub const MM_ECHO_WAVEIN: u32 = 3u32;
pub const MM_ECHO_WAVEOUT: u32 = 2u32;
pub const MM_ECS: u32 = 145u32;
pub const MM_ECS_AADF_MIDI_IN: u32 = 10u32;
pub const MM_ECS_AADF_MIDI_OUT: u32 = 11u32;
pub const MM_ECS_AADF_WAVE2MIDI_IN: u32 = 12u32;
pub const MM_EES: u32 = 219u32;
pub const MM_EES_PCMIDI14: u32 = 1u32;
pub const MM_EES_PCMIDI14_IN: u32 = 2u32;
pub const MM_EES_PCMIDI14_OUT1: u32 = 3u32;
pub const MM_EES_PCMIDI14_OUT2: u32 = 4u32;
pub const MM_EES_PCMIDI14_OUT3: u32 = 5u32;
pub const MM_EES_PCMIDI14_OUT4: u32 = 6u32;
pub const MM_EMAGIC: u32 = 208u32;
pub const MM_EMAGIC_UNITOR8: u32 = 1u32;
pub const MM_EMU: u32 = 19u32;
pub const MM_EMU_APSMIDIIN: u32 = 2u32;
pub const MM_EMU_APSMIDIOUT: u32 = 3u32;
pub const MM_EMU_APSSYNTH: u32 = 1u32;
pub const MM_EMU_APSWAVEIN: u32 = 4u32;
pub const MM_EMU_APSWAVEOUT: u32 = 5u32;
pub const MM_ENET: u32 = 206u32;
pub const MM_ENET_T2000_HANDSETIN: u32 = 3u32;
pub const MM_ENET_T2000_HANDSETOUT: u32 = 4u32;
pub const MM_ENET_T2000_LINEIN: u32 = 1u32;
pub const MM_ENET_T2000_LINEOUT: u32 = 2u32;
pub const MM_ENSONIQ: u32 = 125u32;
pub const MM_ENSONIQ_SOUNDSCAPE: u32 = 16u32;
pub const MM_EPSON: u32 = 50u32;
pub const MM_EPS_FMSND: u32 = 1u32;
pub const MM_ESS: u32 = 46u32;
pub const MM_ESS_AMAUX: u32 = 3u32;
pub const MM_ESS_AMMIDIIN: u32 = 6u32;
pub const MM_ESS_AMMIDIOUT: u32 = 5u32;
pub const MM_ESS_AMSYNTH: u32 = 4u32;
pub const MM_ESS_AMWAVEIN: u32 = 2u32;
pub const MM_ESS_AMWAVEOUT: u32 = 1u32;
pub const MM_ESS_AUX_CD: u32 = 8u32;
pub const MM_ESS_ES1488_MIXER: u32 = 24u32;
pub const MM_ESS_ES1488_WAVEIN: u32 = 23u32;
pub const MM_ESS_ES1488_WAVEOUT: u32 = 22u32;
pub const MM_ESS_ES1688_MIXER: u32 = 27u32;
pub const MM_ESS_ES1688_WAVEIN: u32 = 26u32;
pub const MM_ESS_ES1688_WAVEOUT: u32 = 25u32;
pub const MM_ESS_ES1788_MIXER: u32 = 30u32;
pub const MM_ESS_ES1788_WAVEIN: u32 = 29u32;
pub const MM_ESS_ES1788_WAVEOUT: u32 = 28u32;
pub const MM_ESS_ES1868_MIXER: u32 = 36u32;
pub const MM_ESS_ES1868_WAVEIN: u32 = 35u32;
pub const MM_ESS_ES1868_WAVEOUT: u32 = 34u32;
pub const MM_ESS_ES1878_MIXER: u32 = 39u32;
pub const MM_ESS_ES1878_WAVEIN: u32 = 38u32;
pub const MM_ESS_ES1878_WAVEOUT: u32 = 37u32;
pub const MM_ESS_ES1888_MIXER: u32 = 33u32;
pub const MM_ESS_ES1888_WAVEIN: u32 = 32u32;
pub const MM_ESS_ES1888_WAVEOUT: u32 = 31u32;
pub const MM_ESS_ES488_MIXER: u32 = 18u32;
pub const MM_ESS_ES488_WAVEIN: u32 = 17u32;
pub const MM_ESS_ES488_WAVEOUT: u32 = 16u32;
pub const MM_ESS_ES688_MIXER: u32 = 21u32;
pub const MM_ESS_ES688_WAVEIN: u32 = 20u32;
pub const MM_ESS_ES688_WAVEOUT: u32 = 19u32;
pub const MM_ESS_MIXER: u32 = 7u32;
pub const MM_ESS_MPU401_MIDIIN: u32 = 10u32;
pub const MM_ESS_MPU401_MIDIOUT: u32 = 9u32;
pub const MM_ETEK: u32 = 241u32;
pub const MM_ETEK_KWIKMIDI_MIDIIN: u32 = 1u32;
pub const MM_ETEK_KWIKMIDI_MIDIOUT: u32 = 2u32;
pub const MM_EUPHONICS: u32 = 152u32;
pub const MM_EUPHONICS_AUX_CD: u32 = 1u32;
pub const MM_EUPHONICS_AUX_LINE: u32 = 2u32;
pub const MM_EUPHONICS_AUX_MASTER: u32 = 3u32;
pub const MM_EUPHONICS_AUX_MIC: u32 = 4u32;
pub const MM_EUPHONICS_AUX_MIDI: u32 = 5u32;
pub const MM_EUPHONICS_AUX_WAVE: u32 = 6u32;
pub const MM_EUPHONICS_EUSYNTH: u32 = 14u32;
pub const MM_EUPHONICS_FMSYNTH_MONO: u32 = 7u32;
pub const MM_EUPHONICS_FMSYNTH_STEREO: u32 = 8u32;
pub const MM_EUPHONICS_MIDIIN: u32 = 9u32;
pub const MM_EUPHONICS_MIDIOUT: u32 = 10u32;
pub const MM_EUPHONICS_MIXER: u32 = 11u32;
pub const MM_EUPHONICS_WAVEIN: u32 = 12u32;
pub const MM_EUPHONICS_WAVEOUT: u32 = 13u32;
pub const MM_EVEREX: u32 = 38u32;
pub const MM_EVEREX_CARRIER: u32 = 1u32;
pub const MM_EXAN: u32 = 63u32;
pub const MM_FAITH: u32 = 15u32;
pub const MM_FAST: u32 = 126u32;
pub const MM_FHGIIS_MPEGLAYER3: u32 = 10u32;
pub const MM_FHGIIS_MPEGLAYER3_ADVANCED: u32 = 12u32;
pub const MM_FHGIIS_MPEGLAYER3_ADVANCEDPLUS: u32 = 14u32;
pub const MM_FHGIIS_MPEGLAYER3_BASIC: u32 = 11u32;
pub const MM_FHGIIS_MPEGLAYER3_DECODE: u32 = 9u32;
pub const MM_FHGIIS_MPEGLAYER3_LITE: u32 = 10u32;
pub const MM_FHGIIS_MPEGLAYER3_PROFESSIONAL: u32 = 13u32;
pub const MM_FLEXION: u32 = 249u32;
pub const MM_FLEXION_X300_WAVEIN: u32 = 1u32;
pub const MM_FLEXION_X300_WAVEOUT: u32 = 2u32;
pub const MM_FORTEMEDIA: u32 = 229u32;
pub const MM_FORTEMEDIA_AUX: u32 = 5u32;
pub const MM_FORTEMEDIA_FMSYNC: u32 = 3u32;
pub const MM_FORTEMEDIA_MIXER: u32 = 4u32;
pub const MM_FORTEMEDIA_WAVEIN: u32 = 1u32;
pub const MM_FORTEMEDIA_WAVEOUT: u32 = 2u32;
pub const MM_FRAUNHOFER_IIS: u32 = 172u32;
pub const MM_FRONTIER: u32 = 160u32;
pub const MM_FRONTIER_WAVECENTER_MIDIIN: u32 = 1u32;
pub const MM_FRONTIER_WAVECENTER_MIDIOUT: u32 = 2u32;
pub const MM_FRONTIER_WAVECENTER_WAVEIN: u32 = 3u32;
pub const MM_FRONTIER_WAVECENTER_WAVEOUT: u32 = 4u32;
pub const MM_FTR: u32 = 198u32;
pub const MM_FTR_ACM: u32 = 2u32;
pub const MM_FTR_ENCODER_WAVEIN: u32 = 1u32;
pub const MM_FUJITSU: u32 = 4u32;
pub const MM_GADGETLABS: u32 = 159u32;
pub const MM_GADGETLABS_WAVE42_WAVEIN: u32 = 3u32;
pub const MM_GADGETLABS_WAVE42_WAVEOUT: u32 = 4u32;
pub const MM_GADGETLABS_WAVE44_WAVEIN: u32 = 1u32;
pub const MM_GADGETLABS_WAVE44_WAVEOUT: u32 = 2u32;
pub const MM_GADGETLABS_WAVE4_MIDIIN: u32 = 5u32;
pub const MM_GADGETLABS_WAVE4_MIDIOUT: u32 = 6u32;
pub const MM_GRANDE: u32 = 117u32;
pub const MM_GRAVIS: u32 = 34u32;
pub const MM_GUILLEMOT: u32 = 207u32;
pub const MM_GULBRANSEN: u32 = 130u32;
pub const MM_HAFTMANN: u32 = 220u32;
pub const MM_HAFTMANN_LPTDAC2: u32 = 1u32;
pub const MM_HEADSPACE: u32 = 222u32;
pub const MM_HEADSPACE_HAEMIXER: u32 = 4u32;
pub const MM_HEADSPACE_HAESYNTH: u32 = 1u32;
pub const MM_HEADSPACE_HAEWAVEIN: u32 = 3u32;
pub const MM_HEADSPACE_HAEWAVEOUT: u32 = 2u32;
pub const MM_HEWLETT_PACKARD: u32 = 13u32;
pub const MM_HEWLETT_PACKARD_CU_CODEC: u32 = 1u32;
pub const MM_HORIZONS: u32 = 107u32;
pub const MM_HP: u32 = 253u32;
pub const MM_HP_WAVEIN: u32 = 2u32;
pub const MM_HP_WAVEOUT: u32 = 1u32;
pub const MM_HYPERACTIVE: u32 = 246u32;
pub const MM_IBM: u32 = 22u32;
pub const MM_IBM_MWAVE_AUX: u32 = 23u32;
pub const MM_IBM_MWAVE_MIDIIN: u32 = 21u32;
pub const MM_IBM_MWAVE_MIDIOUT: u32 = 22u32;
pub const MM_IBM_MWAVE_MIXER: u32 = 20u32;
pub const MM_IBM_MWAVE_WAVEIN: u32 = 18u32;
pub const MM_IBM_MWAVE_WAVEOUT: u32 = 19u32;
pub const MM_IBM_PCMCIA_AUX: u32 = 16u32;
pub const MM_IBM_PCMCIA_MIDIIN: u32 = 14u32;
pub const MM_IBM_PCMCIA_MIDIOUT: u32 = 15u32;
pub const MM_IBM_PCMCIA_SYNTH: u32 = 13u32;
pub const MM_IBM_PCMCIA_WAVEIN: u32 = 11u32;
pub const MM_IBM_PCMCIA_WAVEOUT: u32 = 12u32;
pub const MM_IBM_THINKPAD200: u32 = 17u32;
pub const MM_IBM_WC_MIDIOUT: u32 = 30u32;
pub const MM_IBM_WC_MIXEROUT: u32 = 33u32;
pub const MM_IBM_WC_WAVEOUT: u32 = 31u32;
pub const MM_ICCC: u32 = 259u32;
pub const MM_ICCC_UNA3_AUX: u32 = 3u32;
pub const MM_ICCC_UNA3_MIXER: u32 = 4u32;
pub const MM_ICCC_UNA3_WAVEIN: u32 = 1u32;
pub const MM_ICCC_UNA3_WAVEOUT: u32 = 2u32;
pub const MM_ICE: u32 = 239u32;
pub const MM_ICE_AUX: u32 = 11u32;
pub const MM_ICE_MIDIIN1: u32 = 6u32;
pub const MM_ICE_MIDIIN2: u32 = 8u32;
pub const MM_ICE_MIDIOUT1: u32 = 5u32;
pub const MM_ICE_MIDIOUT2: u32 = 7u32;
pub const MM_ICE_MIXER: u32 = 10u32;
pub const MM_ICE_MTWAVEIN: u32 = 4u32;
pub const MM_ICE_MTWAVEOUT: u32 = 3u32;
pub const MM_ICE_SYNTH: u32 = 9u32;
pub const MM_ICE_WAVEIN: u32 = 2u32;
pub const MM_ICE_WAVEOUT: u32 = 1u32;
pub const MM_ICL_PS: u32 = 32u32;
pub const MM_ICOM_AUX: u32 = 6u32;
pub const MM_ICOM_LINE: u32 = 7u32;
pub const MM_ICOM_MIXER: u32 = 5u32;
pub const MM_ICOM_WAVEIN: u32 = 3u32;
pub const MM_ICOM_WAVEOUT: u32 = 4u32;
pub const MM_ICS: u32 = 57u32;
pub const MM_ICS_2115_LITE_MIDIOUT: u32 = 13u32;
pub const MM_ICS_2120_LITE_MIDIOUT: u32 = 14u32;
pub const MM_ICS_WAVEDECK_AUX: u32 = 4u32;
pub const MM_ICS_WAVEDECK_MIXER: u32 = 3u32;
pub const MM_ICS_WAVEDECK_SYNTH: u32 = 5u32;
pub const MM_ICS_WAVEDECK_WAVEIN: u32 = 2u32;
pub const MM_ICS_WAVEDECK_WAVEOUT: u32 = 1u32;
pub const MM_ICS_WAVEDEC_SB_AUX: u32 = 12u32;
pub const MM_ICS_WAVEDEC_SB_FM_MIDIOUT: u32 = 8u32;
pub const MM_ICS_WAVEDEC_SB_MIXER: u32 = 11u32;
pub const MM_ICS_WAVEDEC_SB_MPU401_MIDIIN: u32 = 10u32;
pub const MM_ICS_WAVEDEC_SB_MPU401_MIDIOUT: u32 = 9u32;
pub const MM_ICS_WAVEDEC_SB_WAVEIN: u32 = 7u32;
pub const MM_ICS_WAVEDEC_SB_WAVEOUT: u32 = 6u32;
pub const MM_INSOFT: u32 = 94u32;
pub const MM_INTEL: u32 = 33u32;
pub const MM_INTELOPD_AUX: u32 = 401u32;
pub const MM_INTELOPD_WAVEIN: u32 = 1u32;
pub const MM_INTELOPD_WAVEOUT: u32 = 101u32;
pub const MM_INTEL_NSPMODEMLINEIN: u32 = 501u32;
pub const MM_INTEL_NSPMODEMLINEOUT: u32 = 502u32;
pub const MM_INTERACTIVE: u32 = 36u32;
pub const MM_INTERACTIVE_WAVEIN: u32 = 69u32;
pub const MM_INTERACTIVE_WAVEOUT: u32 = 69u32;
pub const MM_INTERNET: u32 = 244u32;
pub const MM_INTERNET_SSW_MIDIIN: u32 = 11u32;
pub const MM_INTERNET_SSW_MIDIOUT: u32 = 10u32;
pub const MM_INTERNET_SSW_WAVEIN: u32 = 13u32;
pub const MM_INTERNET_SSW_WAVEOUT: u32 = 12u32;
pub const MM_INVISION: u32 = 188u32;
pub const MM_IODD: u32 = 258u32;
pub const MM_IOMAGIC: u32 = 82u32;
pub const MM_IOMAGIC_TEMPO_AUXOUT: u32 = 6u32;
pub const MM_IOMAGIC_TEMPO_MIDIOUT: u32 = 4u32;
pub const MM_IOMAGIC_TEMPO_MXDOUT: u32 = 5u32;
pub const MM_IOMAGIC_TEMPO_SYNTH: u32 = 3u32;
pub const MM_IOMAGIC_TEMPO_WAVEIN: u32 = 2u32;
pub const MM_IOMAGIC_TEMPO_WAVEOUT: u32 = 1u32;
pub const MM_IPI: u32 = 238u32;
pub const MM_IPI_ACM_HSX: u32 = 1u32;
pub const MM_IPI_ACM_RPELP: u32 = 2u32;
pub const MM_IPI_AT_MIXER: u32 = 6u32;
pub const MM_IPI_AT_WAVEIN: u32 = 5u32;
pub const MM_IPI_AT_WAVEOUT: u32 = 4u32;
pub const MM_IPI_WF_ASSS: u32 = 3u32;
pub const MM_ISOLUTION: u32 = 106u32;
pub const MM_ISOLUTION_PASCAL: u32 = 1u32;
pub const MM_ITERATEDSYS: u32 = 58u32;
pub const MM_ITERATEDSYS_FUFCODEC: u32 = 1u32;
pub const MM_I_LINK: u32 = 233u32;
pub const MM_I_LINK_VOICE_CODER: u32 = 1u32;
pub const MM_KAY_ELEMETRICS: u32 = 131u32;
pub const MM_KAY_ELEMETRICS_CSL: u32 = 17152u32;
pub const MM_KAY_ELEMETRICS_CSL_4CHANNEL: u32 = 17161u32;
pub const MM_KAY_ELEMETRICS_CSL_DAT: u32 = 17160u32;
pub const MM_KORG: u32 = 55u32;
pub const MM_KORG_1212IO_MSWAVEIN: u32 = 3u32;
pub const MM_KORG_1212IO_MSWAVEOUT: u32 = 4u32;
pub const MM_KORG_PCIF_MIDIIN: u32 = 2u32;
pub const MM_KORG_PCIF_MIDIOUT: u32 = 1u32;
pub const MM_LERNOUT_ANDHAUSPIE_LHCODECACM: u32 = 1u32;
pub const MM_LERNOUT_AND_HAUSPIE: u32 = 97u32;
pub const MM_LEXICON: u32 = 236u32;
pub const MM_LEXICON_STUDIO_WAVE_IN: u32 = 2u32;
pub const MM_LEXICON_STUDIO_WAVE_OUT: u32 = 1u32;
pub const MM_LOGITECH: u32 = 60u32;
pub const MM_LUCENT: u32 = 184u32;
pub const MM_LUCENT_ACM_G723: u32 = 0u32;
pub const MM_LUCID: u32 = 221u32;
pub const MM_LUCID_PCI24WAVEIN: u32 = 1u32;
pub const MM_LUCID_PCI24WAVEOUT: u32 = 2u32;
pub const MM_LUMINOSITI: u32 = 224u32;
pub const MM_LUMINOSITI_SCWAVEIN: u32 = 1u32;
pub const MM_LUMINOSITI_SCWAVEMIX: u32 = 3u32;
pub const MM_LUMINOSITI_SCWAVEOUT: u32 = 2u32;
pub const MM_LYNX: u32 = 212u32;
pub const MM_LYRRUS: u32 = 88u32;
pub const MM_LYRRUS_BRIDGE_GUITAR: u32 = 1u32;
pub const MM_MALDEN: u32 = 261u32;
pub const MM_MARIAN: u32 = 190u32;
pub const MM_MARIAN_ARC44WAVEIN: u32 = 1u32;
pub const MM_MARIAN_ARC44WAVEOUT: u32 = 2u32;
pub const MM_MARIAN_ARC88WAVEIN: u32 = 5u32;
pub const MM_MARIAN_ARC88WAVEOUT: u32 = 6u32;
pub const MM_MARIAN_PRODIF24WAVEIN: u32 = 3u32;
pub const MM_MARIAN_PRODIF24WAVEOUT: u32 = 4u32;
pub const MM_MATROX_DIV: u32 = 254u32;
pub const MM_MATSUSHITA: u32 = 83u32;
pub const MM_MATSUSHITA_AUX: u32 = 5u32;
pub const MM_MATSUSHITA_FMSYNTH_STEREO: u32 = 3u32;
pub const MM_MATSUSHITA_MIXER: u32 = 4u32;
pub const MM_MATSUSHITA_WAVEIN: u32 = 1u32;
pub const MM_MATSUSHITA_WAVEOUT: u32 = 2u32;
pub const MM_MEDIASONIC: u32 = 71u32;
pub const MM_MEDIASONIC_ACM_G723: u32 = 1u32;
pub const MM_MEDIASONIC_ICOM: u32 = 2u32;
pub const MM_MEDIATRIX: u32 = 141u32;
pub const MM_MEDIAVISION: u32 = 3u32;
pub const MM_MEDIAVISION_CDPC: u32 = 112u32;
pub const MM_MEDIAVISION_OPUS1208: u32 = 128u32;
pub const MM_MEDIAVISION_OPUS1216: u32 = 144u32;
pub const MM_MEDIAVISION_PROAUDIO: u32 = 16u32;
pub const MM_MEDIAVISION_PROAUDIO_16: u32 = 96u32;
pub const MM_MEDIAVISION_PROAUDIO_PLUS: u32 = 80u32;
pub const MM_MEDIAVISION_PROSTUDIO_16: u32 = 96u32;
pub const MM_MEDIAVISION_THUNDER: u32 = 32u32;
pub const MM_MEDIAVISION_TPORT: u32 = 64u32;
pub const MM_MELABS: u32 = 44u32;
pub const MM_MELABS_MIDI2GO: u32 = 1u32;
pub const MM_MERGING_MPEGL3: u32 = 1u32;
pub const MM_MERGING_TECHNOLOGIES: u32 = 177u32;
pub const MM_METHEUS: u32 = 59u32;
pub const MM_METHEUS_ZIPPER: u32 = 1u32;
pub const MM_MICRONAS: u32 = 251u32;
pub const MM_MICRONAS_CLP833: u32 = 2u32;
pub const MM_MICRONAS_SC4: u32 = 1u32;
pub const MM_MINDMAKER: u32 = 263u32;
pub const MM_MINDMAKER_GC_MIXER: u32 = 3u32;
pub const MM_MINDMAKER_GC_WAVEIN: u32 = 1u32;
pub const MM_MINDMAKER_GC_WAVEOUT: u32 = 2u32;
pub const MM_MIRO: u32 = 104u32;
pub const MM_MIRO_DC30_MIX: u32 = 7u32;
pub const MM_MIRO_DC30_WAVEIN: u32 = 6u32;
pub const MM_MIRO_DC30_WAVEOUT: u32 = 5u32;
pub const MM_MIRO_MOVIEPRO: u32 = 1u32;
pub const MM_MIRO_VIDEOD1: u32 = 2u32;
pub const MM_MIRO_VIDEODC1TV: u32 = 3u32;
pub const MM_MIRO_VIDEOTD: u32 = 4u32;
pub const MM_MITEL: u32 = 16u32;
pub const MM_MITEL_MEDIAPATH_WAVEIN: u32 = 301u32;
pub const MM_MITEL_MEDIAPATH_WAVEOUT: u32 = 300u32;
pub const MM_MITEL_MPA_HANDSET_WAVEIN: u32 = 201u32;
pub const MM_MITEL_MPA_HANDSET_WAVEOUT: u32 = 200u32;
pub const MM_MITEL_MPA_HANDSFREE_WAVEIN: u32 = 203u32;
pub const MM_MITEL_MPA_HANDSFREE_WAVEOUT: u32 = 202u32;
pub const MM_MITEL_MPA_LINE1_WAVEIN: u32 = 205u32;
pub const MM_MITEL_MPA_LINE1_WAVEOUT: u32 = 204u32;
pub const MM_MITEL_MPA_LINE2_WAVEIN: u32 = 207u32;
pub const MM_MITEL_MPA_LINE2_WAVEOUT: u32 = 206u32;
pub const MM_MITEL_TALKTO_BRIDGED_WAVEIN: u32 = 105u32;
pub const MM_MITEL_TALKTO_BRIDGED_WAVEOUT: u32 = 104u32;
pub const MM_MITEL_TALKTO_HANDSET_WAVEIN: u32 = 103u32;
pub const MM_MITEL_TALKTO_HANDSET_WAVEOUT: u32 = 102u32;
pub const MM_MITEL_TALKTO_LINE_WAVEIN: u32 = 101u32;
pub const MM_MITEL_TALKTO_LINE_WAVEOUT: u32 = 100u32;
pub const MM_MMOTION_WAVEAUX: u32 = 1u32;
pub const MM_MMOTION_WAVEIN: u32 = 3u32;
pub const MM_MMOTION_WAVEOUT: u32 = 2u32;
pub const MM_MOSCOM: u32 = 68u32;
pub const MM_MOSCOM_VPC2400_IN: u32 = 1u32;
pub const MM_MOSCOM_VPC2400_OUT: u32 = 2u32;
pub const MM_MOTIONPIXELS: u32 = 193u32;
pub const MM_MOTIONPIXELS_MVI2: u32 = 1u32;
pub const MM_MOTOROLA: u32 = 48u32;
pub const MM_MOTU: u32 = 101u32;
pub const MM_MOTU_DTX_MIDI_IN_A: u32 = 801u32;
pub const MM_MOTU_DTX_MIDI_IN_B: u32 = 802u32;
pub const MM_MOTU_DTX_MIDI_IN_SYNC: u32 = 800u32;
pub const MM_MOTU_DTX_MIDI_OUT_A: u32 = 801u32;
pub const MM_MOTU_DTX_MIDI_OUT_B: u32 = 802u32;
pub const MM_MOTU_FLYER_MIDI_IN_A: u32 = 601u32;
pub const MM_MOTU_FLYER_MIDI_IN_B: u32 = 602u32;
pub const MM_MOTU_FLYER_MIDI_IN_SYNC: u32 = 600u32;
pub const MM_MOTU_FLYER_MIDI_OUT_A: u32 = 601u32;
pub const MM_MOTU_FLYER_MIDI_OUT_B: u32 = 602u32;
pub const MM_MOTU_MTPAV_MIDIIN_1: u32 = 901u32;
pub const MM_MOTU_MTPAV_MIDIIN_2: u32 = 902u32;
pub const MM_MOTU_MTPAV_MIDIIN_3: u32 = 903u32;
pub const MM_MOTU_MTPAV_MIDIIN_4: u32 = 904u32;
pub const MM_MOTU_MTPAV_MIDIIN_5: u32 = 905u32;
pub const MM_MOTU_MTPAV_MIDIIN_6: u32 = 906u32;
pub const MM_MOTU_MTPAV_MIDIIN_7: u32 = 907u32;
pub const MM_MOTU_MTPAV_MIDIIN_8: u32 = 908u32;
pub const MM_MOTU_MTPAV_MIDIIN_ADAT: u32 = 917u32;
pub const MM_MOTU_MTPAV_MIDIIN_SYNC: u32 = 900u32;
pub const MM_MOTU_MTPAV_MIDIOUT_1: u32 = 901u32;
pub const MM_MOTU_MTPAV_MIDIOUT_2: u32 = 902u32;
pub const MM_MOTU_MTPAV_MIDIOUT_3: u32 = 903u32;
pub const MM_MOTU_MTPAV_MIDIOUT_4: u32 = 904u32;
pub const MM_MOTU_MTPAV_MIDIOUT_5: u32 = 905u32;
pub const MM_MOTU_MTPAV_MIDIOUT_6: u32 = 906u32;
pub const MM_MOTU_MTPAV_MIDIOUT_7: u32 = 907u32;
pub const MM_MOTU_MTPAV_MIDIOUT_8: u32 = 908u32;
pub const MM_MOTU_MTPAV_MIDIOUT_ADAT: u32 = 917u32;
pub const MM_MOTU_MTPAV_MIDIOUT_ALL: u32 = 900u32;
pub const MM_MOTU_MTPAV_NET_MIDIIN_1: u32 = 909u32;
pub const MM_MOTU_MTPAV_NET_MIDIIN_2: u32 = 910u32;
pub const MM_MOTU_MTPAV_NET_MIDIIN_3: u32 = 911u32;
pub const MM_MOTU_MTPAV_NET_MIDIIN_4: u32 = 912u32;
pub const MM_MOTU_MTPAV_NET_MIDIIN_5: u32 = 913u32;
pub const MM_MOTU_MTPAV_NET_MIDIIN_6: u32 = 914u32;
pub const MM_MOTU_MTPAV_NET_MIDIIN_7: u32 = 915u32;
pub const MM_MOTU_MTPAV_NET_MIDIIN_8: u32 = 916u32;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_1: u32 = 909u32;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_2: u32 = 910u32;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_3: u32 = 911u32;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_4: u32 = 912u32;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_5: u32 = 913u32;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_6: u32 = 914u32;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_7: u32 = 915u32;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_8: u32 = 916u32;
pub const MM_MOTU_MTPII_MIDIIN_1: u32 = 201u32;
pub const MM_MOTU_MTPII_MIDIIN_2: u32 = 202u32;
pub const MM_MOTU_MTPII_MIDIIN_3: u32 = 203u32;
pub const MM_MOTU_MTPII_MIDIIN_4: u32 = 204u32;
pub const MM_MOTU_MTPII_MIDIIN_5: u32 = 205u32;
pub const MM_MOTU_MTPII_MIDIIN_6: u32 = 206u32;
pub const MM_MOTU_MTPII_MIDIIN_7: u32 = 207u32;
pub const MM_MOTU_MTPII_MIDIIN_8: u32 = 208u32;
pub const MM_MOTU_MTPII_MIDIIN_SYNC: u32 = 200u32;
pub const MM_MOTU_MTPII_MIDIOUT_1: u32 = 201u32;
pub const MM_MOTU_MTPII_MIDIOUT_2: u32 = 202u32;
pub const MM_MOTU_MTPII_MIDIOUT_3: u32 = 203u32;
pub const MM_MOTU_MTPII_MIDIOUT_4: u32 = 204u32;
pub const MM_MOTU_MTPII_MIDIOUT_5: u32 = 205u32;
pub const MM_MOTU_MTPII_MIDIOUT_6: u32 = 206u32;
pub const MM_MOTU_MTPII_MIDIOUT_7: u32 = 207u32;
pub const MM_MOTU_MTPII_MIDIOUT_8: u32 = 208u32;
pub const MM_MOTU_MTPII_MIDIOUT_ALL: u32 = 200u32;
pub const MM_MOTU_MTPII_NET_MIDIIN_1: u32 = 209u32;
pub const MM_MOTU_MTPII_NET_MIDIIN_2: u32 = 210u32;
pub const MM_MOTU_MTPII_NET_MIDIIN_3: u32 = 211u32;
pub const MM_MOTU_MTPII_NET_MIDIIN_4: u32 = 212u32;
pub const MM_MOTU_MTPII_NET_MIDIIN_5: u32 = 213u32;
pub const MM_MOTU_MTPII_NET_MIDIIN_6: u32 = 214u32;
pub const MM_MOTU_MTPII_NET_MIDIIN_7: u32 = 215u32;
pub const MM_MOTU_MTPII_NET_MIDIIN_8: u32 = 216u32;
pub const MM_MOTU_MTPII_NET_MIDIOUT_1: u32 = 209u32;
pub const MM_MOTU_MTPII_NET_MIDIOUT_2: u32 = 210u32;
pub const MM_MOTU_MTPII_NET_MIDIOUT_3: u32 = 211u32;
pub const MM_MOTU_MTPII_NET_MIDIOUT_4: u32 = 212u32;
pub const MM_MOTU_MTPII_NET_MIDIOUT_5: u32 = 213u32;
pub const MM_MOTU_MTPII_NET_MIDIOUT_6: u32 = 214u32;
pub const MM_MOTU_MTPII_NET_MIDIOUT_7: u32 = 215u32;
pub const MM_MOTU_MTPII_NET_MIDIOUT_8: u32 = 216u32;
pub const MM_MOTU_MTP_MIDIIN_1: u32 = 101u32;
pub const MM_MOTU_MTP_MIDIIN_2: u32 = 102u32;
pub const MM_MOTU_MTP_MIDIIN_3: u32 = 103u32;
pub const MM_MOTU_MTP_MIDIIN_4: u32 = 104u32;
pub const MM_MOTU_MTP_MIDIIN_5: u32 = 105u32;
pub const MM_MOTU_MTP_MIDIIN_6: u32 = 106u32;
pub const MM_MOTU_MTP_MIDIIN_7: u32 = 107u32;
pub const MM_MOTU_MTP_MIDIIN_8: u32 = 108u32;
pub const MM_MOTU_MTP_MIDIOUT_1: u32 = 101u32;
pub const MM_MOTU_MTP_MIDIOUT_2: u32 = 102u32;
pub const MM_MOTU_MTP_MIDIOUT_3: u32 = 103u32;
pub const MM_MOTU_MTP_MIDIOUT_4: u32 = 104u32;
pub const MM_MOTU_MTP_MIDIOUT_5: u32 = 105u32;
pub const MM_MOTU_MTP_MIDIOUT_6: u32 = 106u32;
pub const MM_MOTU_MTP_MIDIOUT_7: u32 = 107u32;
pub const MM_MOTU_MTP_MIDIOUT_8: u32 = 108u32;
pub const MM_MOTU_MTP_MIDIOUT_ALL: u32 = 100u32;
pub const MM_MOTU_MXN_MIDIIN_1: u32 = 501u32;
pub const MM_MOTU_MXN_MIDIIN_2: u32 = 502u32;
pub const MM_MOTU_MXN_MIDIIN_3: u32 = 503u32;
pub const MM_MOTU_MXN_MIDIIN_4: u32 = 504u32;
pub const MM_MOTU_MXN_MIDIIN_SYNC: u32 = 500u32;
pub const MM_MOTU_MXN_MIDIOUT_1: u32 = 501u32;
pub const MM_MOTU_MXN_MIDIOUT_2: u32 = 502u32;
pub const MM_MOTU_MXN_MIDIOUT_3: u32 = 503u32;
pub const MM_MOTU_MXN_MIDIOUT_4: u32 = 504u32;
pub const MM_MOTU_MXN_MIDIOUT_ALL: u32 = 500u32;
pub const MM_MOTU_MXPMPU_MIDIIN_1: u32 = 401u32;
pub const MM_MOTU_MXPMPU_MIDIIN_2: u32 = 402u32;
pub const MM_MOTU_MXPMPU_MIDIIN_3: u32 = 403u32;
pub const MM_MOTU_MXPMPU_MIDIIN_4: u32 = 404u32;
pub const MM_MOTU_MXPMPU_MIDIIN_5: u32 = 405u32;
pub const MM_MOTU_MXPMPU_MIDIIN_6: u32 = 406u32;
pub const MM_MOTU_MXPMPU_MIDIIN_SYNC: u32 = 400u32;
pub const MM_MOTU_MXPMPU_MIDIOUT_1: u32 = 401u32;
pub const MM_MOTU_MXPMPU_MIDIOUT_2: u32 = 402u32;
pub const MM_MOTU_MXPMPU_MIDIOUT_3: u32 = 403u32;
pub const MM_MOTU_MXPMPU_MIDIOUT_4: u32 = 404u32;
pub const MM_MOTU_MXPMPU_MIDIOUT_5: u32 = 405u32;
pub const MM_MOTU_MXPMPU_MIDIOUT_6: u32 = 406u32;
pub const MM_MOTU_MXPMPU_MIDIOUT_ALL: u32 = 400u32;
pub const MM_MOTU_MXPXT_MIDIIN_1: u32 = 1001u32;
pub const MM_MOTU_MXPXT_MIDIIN_2: u32 = 1002u32;
pub const MM_MOTU_MXPXT_MIDIIN_3: u32 = 1003u32;
pub const MM_MOTU_MXPXT_MIDIIN_4: u32 = 1004u32;
pub const MM_MOTU_MXPXT_MIDIIN_5: u32 = 1005u32;
pub const MM_MOTU_MXPXT_MIDIIN_6: u32 = 1006u32;
pub const MM_MOTU_MXPXT_MIDIIN_7: u32 = 1007u32;
pub const MM_MOTU_MXPXT_MIDIIN_8: u32 = 1008u32;
pub const MM_MOTU_MXPXT_MIDIIN_SYNC: u32 = 1000u32;
pub const MM_MOTU_MXPXT_MIDIOUT_1: u32 = 1001u32;
pub const MM_MOTU_MXPXT_MIDIOUT_2: u32 = 1002u32;
pub const MM_MOTU_MXPXT_MIDIOUT_3: u32 = 1003u32;
pub const MM_MOTU_MXPXT_MIDIOUT_4: u32 = 1004u32;
pub const MM_MOTU_MXPXT_MIDIOUT_5: u32 = 1005u32;
pub const MM_MOTU_MXPXT_MIDIOUT_6: u32 = 1006u32;
pub const MM_MOTU_MXPXT_MIDIOUT_7: u32 = 1007u32;
pub const MM_MOTU_MXPXT_MIDIOUT_8: u32 = 1008u32;
pub const MM_MOTU_MXPXT_MIDIOUT_ALL: u32 = 1000u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_1: u32 = 301u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_2: u32 = 302u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_3: u32 = 303u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_4: u32 = 304u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_5: u32 = 305u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_6: u32 = 306u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_1: u32 = 301u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_2: u32 = 302u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_3: u32 = 303u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_4: u32 = 304u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_5: u32 = 305u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_6: u32 = 306u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_ALL: u32 = 300u32;
pub const MM_MOTU_MXP_MIDIIN_SYNC: u32 = 300u32;
pub const MM_MOTU_PKX_MIDI_IN_A: u32 = 701u32;
pub const MM_MOTU_PKX_MIDI_IN_B: u32 = 702u32;
pub const MM_MOTU_PKX_MIDI_IN_SYNC: u32 = 700u32;
pub const MM_MOTU_PKX_MIDI_OUT_A: u32 = 701u32;
pub const MM_MOTU_PKX_MIDI_OUT_B: u32 = 702u32;
pub const MM_MPTUS: u32 = 95u32;
pub const MM_MPTUS_SPWAVEOUT: u32 = 1u32;
pub const MM_MSFT_ACM_G711: u32 = 37u32;
pub const MM_MSFT_ACM_GSM610: u32 = 36u32;
pub const MM_MSFT_ACM_IMAADPCM: u32 = 34u32;
pub const MM_MSFT_ACM_MSADPCM: u32 = 33u32;
pub const MM_MSFT_ACM_MSAUDIO1: u32 = 39u32;
pub const MM_MSFT_ACM_MSFILTER: u32 = 35u32;
pub const MM_MSFT_ACM_MSG723: u32 = 92u32;
pub const MM_MSFT_ACM_MSNAUDIO: u32 = 91u32;
pub const MM_MSFT_ACM_MSRT24: u32 = 93u32;
pub const MM_MSFT_ACM_PCM: u32 = 38u32;
pub const MM_MSFT_ACM_WMAUDIO: u32 = 39u32;
pub const MM_MSFT_ACM_WMAUDIO2: u32 = 101u32;
pub const MM_MSFT_GENERIC_AUX_CD: u32 = 30u32;
pub const MM_MSFT_GENERIC_AUX_LINE: u32 = 28u32;
pub const MM_MSFT_GENERIC_AUX_MIC: u32 = 29u32;
pub const MM_MSFT_GENERIC_MIDIIN: u32 = 25u32;
pub const MM_MSFT_GENERIC_MIDIOUT: u32 = 26u32;
pub const MM_MSFT_GENERIC_MIDISYNTH: u32 = 27u32;
pub const MM_MSFT_GENERIC_WAVEIN: u32 = 23u32;
pub const MM_MSFT_GENERIC_WAVEOUT: u32 = 24u32;
pub const MM_MSFT_MSACM: u32 = 32u32;
pub const MM_MSFT_MSOPL_SYNTH: u32 = 76u32;
pub const MM_MSFT_SB16_AUX_CD: u32 = 66u32;
pub const MM_MSFT_SB16_AUX_LINE: u32 = 65u32;
pub const MM_MSFT_SB16_MIDIIN: u32 = 62u32;
pub const MM_MSFT_SB16_MIDIOUT: u32 = 63u32;
pub const MM_MSFT_SB16_MIXER: u32 = 67u32;
pub const MM_MSFT_SB16_SYNTH: u32 = 64u32;
pub const MM_MSFT_SB16_WAVEIN: u32 = 60u32;
pub const MM_MSFT_SB16_WAVEOUT: u32 = 61u32;
pub const MM_MSFT_SBPRO_AUX_CD: u32 = 74u32;
pub const MM_MSFT_SBPRO_AUX_LINE: u32 = 73u32;
pub const MM_MSFT_SBPRO_MIDIIN: u32 = 70u32;
pub const MM_MSFT_SBPRO_MIDIOUT: u32 = 71u32;
pub const MM_MSFT_SBPRO_MIXER: u32 = 75u32;
pub const MM_MSFT_SBPRO_SYNTH: u32 = 72u32;
pub const MM_MSFT_SBPRO_WAVEIN: u32 = 68u32;
pub const MM_MSFT_SBPRO_WAVEOUT: u32 = 69u32;
pub const MM_MSFT_VMDMS_HANDSET_WAVEIN: u32 = 82u32;
pub const MM_MSFT_VMDMS_HANDSET_WAVEOUT: u32 = 83u32;
pub const MM_MSFT_VMDMS_LINE_WAVEIN: u32 = 80u32;
pub const MM_MSFT_VMDMS_LINE_WAVEOUT: u32 = 81u32;
pub const MM_MSFT_VMDMW_HANDSET_WAVEIN: u32 = 86u32;
pub const MM_MSFT_VMDMW_HANDSET_WAVEOUT: u32 = 87u32;
pub const MM_MSFT_VMDMW_LINE_WAVEIN: u32 = 84u32;
pub const MM_MSFT_VMDMW_LINE_WAVEOUT: u32 = 85u32;
pub const MM_MSFT_VMDMW_MIXER: u32 = 88u32;
pub const MM_MSFT_VMDM_GAME_WAVEIN: u32 = 90u32;
pub const MM_MSFT_VMDM_GAME_WAVEOUT: u32 = 89u32;
pub const MM_MSFT_WDMAUDIO_AUX: u32 = 105u32;
pub const MM_MSFT_WDMAUDIO_MIDIIN: u32 = 103u32;
pub const MM_MSFT_WDMAUDIO_MIDIOUT: u32 = 102u32;
pub const MM_MSFT_WDMAUDIO_MIXER: u32 = 104u32;
pub const MM_MSFT_WDMAUDIO_WAVEIN: u32 = 101u32;
pub const MM_MSFT_WDMAUDIO_WAVEOUT: u32 = 100u32;
pub const MM_MSFT_WSS_AUX: u32 = 21u32;
pub const MM_MSFT_WSS_FMSYNTH_STEREO: u32 = 16u32;
pub const MM_MSFT_WSS_MIXER: u32 = 17u32;
pub const MM_MSFT_WSS_NT_AUX: u32 = 59u32;
pub const MM_MSFT_WSS_NT_FMSYNTH_STEREO: u32 = 57u32;
pub const MM_MSFT_WSS_NT_MIXER: u32 = 58u32;
pub const MM_MSFT_WSS_NT_WAVEIN: u32 = 55u32;
pub const MM_MSFT_WSS_NT_WAVEOUT: u32 = 56u32;
pub const MM_MSFT_WSS_OEM_AUX: u32 = 22u32;
pub const MM_MSFT_WSS_OEM_FMSYNTH_STEREO: u32 = 20u32;
pub const MM_MSFT_WSS_OEM_MIXER: u32 = 31u32;
pub const MM_MSFT_WSS_OEM_WAVEIN: u32 = 18u32;
pub const MM_MSFT_WSS_OEM_WAVEOUT: u32 = 19u32;
pub const MM_MSFT_WSS_WAVEIN: u32 = 14u32;
pub const MM_MSFT_WSS_WAVEOUT: u32 = 15u32;
pub const MM_MWM: u32 = 209u32;
pub const MM_NCR: u32 = 62u32;
pub const MM_NCR_BA_AUX: u32 = 4u32;
pub const MM_NCR_BA_MIXER: u32 = 5u32;
pub const MM_NCR_BA_SYNTH: u32 = 3u32;
pub const MM_NCR_BA_WAVEIN: u32 = 1u32;
pub const MM_NCR_BA_WAVEOUT: u32 = 2u32;
pub const MM_NEC: u32 = 26u32;
pub const MM_NEC_26_SYNTH: u32 = 9u32;
pub const MM_NEC_73_86_SYNTH: u32 = 5u32;
pub const MM_NEC_73_86_WAVEIN: u32 = 7u32;
pub const MM_NEC_73_86_WAVEOUT: u32 = 6u32;
pub const MM_NEC_JOYSTICK: u32 = 12u32;
pub const MM_NEC_MPU401_MIDIIN: u32 = 11u32;
pub const MM_NEC_MPU401_MIDIOUT: u32 = 10u32;
pub const MM_NEOMAGIC: u32 = 176u32;
pub const MM_NEOMAGIC_AUX: u32 = 6u32;
pub const MM_NEOMAGIC_MIDIIN: u32 = 5u32;
pub const MM_NEOMAGIC_MIDIOUT: u32 = 4u32;
pub const MM_NEOMAGIC_MW3DX_AUX: u32 = 17u32;
pub const MM_NEOMAGIC_MW3DX_FMSYNTH: u32 = 14u32;
pub const MM_NEOMAGIC_MW3DX_GMSYNTH: u32 = 15u32;
pub const MM_NEOMAGIC_MW3DX_MIDIIN: u32 = 13u32;
pub const MM_NEOMAGIC_MW3DX_MIDIOUT: u32 = 12u32;
pub const MM_NEOMAGIC_MW3DX_MIXER: u32 = 16u32;
pub const MM_NEOMAGIC_MW3DX_WAVEIN: u32 = 11u32;
pub const MM_NEOMAGIC_MW3DX_WAVEOUT: u32 = 10u32;
pub const MM_NEOMAGIC_MWAVE_AUX: u32 = 25u32;
pub const MM_NEOMAGIC_MWAVE_MIDIIN: u32 = 23u32;
pub const MM_NEOMAGIC_MWAVE_MIDIOUT: u32 = 22u32;
pub const MM_NEOMAGIC_MWAVE_MIXER: u32 = 24u32;
pub const MM_NEOMAGIC_MWAVE_WAVEIN: u32 = 21u32;
pub const MM_NEOMAGIC_MWAVE_WAVEOUT: u32 = 20u32;
pub const MM_NEOMAGIC_SYNTH: u32 = 1u32;
pub const MM_NEOMAGIC_WAVEIN: u32 = 3u32;
pub const MM_NEOMAGIC_WAVEOUT: u32 = 2u32;
pub const MM_NETSCAPE: u32 = 166u32;
pub const MM_NETXL: u32 = 8u32;
pub const MM_NETXL_XLVIDEO: u32 = 1u32;
pub const MM_NEWMEDIA: u32 = 86u32;
pub const MM_NEWMEDIA_WAVJAMMER: u32 = 1u32;
pub const MM_NMP: u32 = 195u32;
pub const MM_NMP_ACM_AMR: u32 = 10u32;
pub const MM_NMP_CCP_WAVEIN: u32 = 1u32;
pub const MM_NMP_CCP_WAVEOUT: u32 = 2u32;
pub const MM_NMS: u32 = 87u32;
pub const MM_NOGATECH: u32 = 75u32;
pub const MM_NORRIS: u32 = 150u32;
pub const MM_NORRIS_VOICELINK: u32 = 1u32;
pub const MM_NORTEL_MPXAC_WAVEIN: u32 = 1u32;
pub const MM_NORTEL_MPXAC_WAVEOUT: u32 = 2u32;
pub const MM_NORTHERN_TELECOM: u32 = 115u32;
pub const MM_NVIDIA: u32 = 127u32;
pub const MM_NVIDIA_AUX: u32 = 7u32;
pub const MM_NVIDIA_GAMEPORT: u32 = 5u32;
pub const MM_NVIDIA_MIDIIN: u32 = 4u32;
pub const MM_NVIDIA_MIDIOUT: u32 = 3u32;
pub const MM_NVIDIA_MIXER: u32 = 6u32;
pub const MM_NVIDIA_WAVEIN: u32 = 2u32;
pub const MM_NVIDIA_WAVEOUT: u32 = 1u32;
pub const MM_OKI: u32 = 79u32;
pub const MM_OKSORI: u32 = 128u32;
pub const MM_OKSORI_BASE: u32 = 0u32;
pub const MM_OKSORI_EXT_MIC1: u32 = 15u32;
pub const MM_OKSORI_EXT_MIC2: u32 = 16u32;
pub const MM_OKSORI_FM_OPL4: u32 = 5u32;
pub const MM_OKSORI_MIDIIN: u32 = 18u32;
pub const MM_OKSORI_MIDIOUT: u32 = 17u32;
pub const MM_OKSORI_MIX_AUX1: u32 = 13u32;
pub const MM_OKSORI_MIX_CD: u32 = 10u32;
pub const MM_OKSORI_MIX_ECHO: u32 = 12u32;
pub const MM_OKSORI_MIX_FM: u32 = 8u32;
pub const MM_OKSORI_MIX_LINE: u32 = 9u32;
pub const MM_OKSORI_MIX_LINE1: u32 = 14u32;
pub const MM_OKSORI_MIX_MASTER: u32 = 6u32;
pub const MM_OKSORI_MIX_MIC: u32 = 11u32;
pub const MM_OKSORI_MIX_WAVE: u32 = 7u32;
pub const MM_OKSORI_MPEG_CDVISION: u32 = 19u32;
pub const MM_OKSORI_OSR16_WAVEIN: u32 = 4u32;
pub const MM_OKSORI_OSR16_WAVEOUT: u32 = 3u32;
pub const MM_OKSORI_OSR8_WAVEIN: u32 = 2u32;
pub const MM_OKSORI_OSR8_WAVEOUT: u32 = 1u32;
pub const MM_OLIVETTI: u32 = 81u32;
pub const MM_OLIVETTI_ACM_ADPCM: u32 = 10u32;
pub const MM_OLIVETTI_ACM_CELP: u32 = 11u32;
pub const MM_OLIVETTI_ACM_GSM: u32 = 9u32;
pub const MM_OLIVETTI_ACM_OPR: u32 = 13u32;
pub const MM_OLIVETTI_ACM_SBC: u32 = 12u32;
pub const MM_OLIVETTI_AUX: u32 = 4u32;
pub const MM_OLIVETTI_JOYSTICK: u32 = 8u32;
pub const MM_OLIVETTI_MIDIIN: u32 = 5u32;
pub const MM_OLIVETTI_MIDIOUT: u32 = 6u32;
pub const MM_OLIVETTI_MIXER: u32 = 3u32;
pub const MM_OLIVETTI_SYNTH: u32 = 7u32;
pub const MM_OLIVETTI_WAVEIN: u32 = 1u32;
pub const MM_OLIVETTI_WAVEOUT: u32 = 2u32;
pub const MM_ONLIVE: u32 = 200u32;
pub const MM_ONLIVE_MPCODEC: u32 = 1u32;
pub const MM_OPCODE: u32 = 113u32;
pub const MM_OPTI: u32 = 90u32;
pub const MM_OPTI_M16_AUX: u32 = 7u32;
pub const MM_OPTI_M16_FMSYNTH_STEREO: u32 = 1u32;
pub const MM_OPTI_M16_MIDIIN: u32 = 2u32;
pub const MM_OPTI_M16_MIDIOUT: u32 = 3u32;
pub const MM_OPTI_M16_MIXER: u32 = 6u32;
pub const MM_OPTI_M16_WAVEIN: u32 = 4u32;
pub const MM_OPTI_M16_WAVEOUT: u32 = 5u32;
pub const MM_OPTI_M32_AUX: u32 = 38u32;
pub const MM_OPTI_M32_MIDIIN: u32 = 34u32;
pub const MM_OPTI_M32_MIDIOUT: u32 = 35u32;
pub const MM_OPTI_M32_MIXER: u32 = 37u32;
pub const MM_OPTI_M32_SYNTH_STEREO: u32 = 36u32;
pub const MM_OPTI_M32_WAVEIN: u32 = 32u32;
pub const MM_OPTI_M32_WAVEOUT: u32 = 33u32;
pub const MM_OPTI_P16_AUX: u32 = 22u32;
pub const MM_OPTI_P16_FMSYNTH_STEREO: u32 = 16u32;
pub const MM_OPTI_P16_MIDIIN: u32 = 17u32;
pub const MM_OPTI_P16_MIDIOUT: u32 = 18u32;
pub const MM_OPTI_P16_MIXER: u32 = 21u32;
pub const MM_OPTI_P16_WAVEIN: u32 = 19u32;
pub const MM_OPTI_P16_WAVEOUT: u32 = 20u32;
pub const MM_OPUS1208_AUX: u32 = 135u32;
pub const MM_OPUS1208_MIXER: u32 = 134u32;
pub const MM_OPUS1208_SYNTH: u32 = 131u32;
pub const MM_OPUS1208_WAVEIN: u32 = 133u32;
pub const MM_OPUS1208_WAVEOUT: u32 = 132u32;
pub const MM_OPUS1216_AUX: u32 = 151u32;
pub const MM_OPUS1216_MIDIIN: u32 = 146u32;
pub const MM_OPUS1216_MIDIOUT: u32 = 145u32;
pub const MM_OPUS1216_MIXER: u32 = 150u32;
pub const MM_OPUS1216_SYNTH: u32 = 147u32;
pub const MM_OPUS1216_WAVEIN: u32 = 149u32;
pub const MM_OPUS1216_WAVEOUT: u32 = 148u32;
pub const MM_OPUS401_MIDIIN: u32 = 130u32;
pub const MM_OPUS401_MIDIOUT: u32 = 129u32;
pub const MM_OSITECH: u32 = 103u32;
pub const MM_OSITECH_TRUMPCARD: u32 = 1u32;
pub const MM_OSPREY: u32 = 140u32;
pub const MM_OSPREY_1000WAVEIN: u32 = 1u32;
pub const MM_OSPREY_1000WAVEOUT: u32 = 2u32;
pub const MM_OTI: u32 = 180u32;
pub const MM_OTI_611MIDIN: u32 = 18u32;
pub const MM_OTI_611MIDIOUT: u32 = 19u32;
pub const MM_OTI_611MIXER: u32 = 7u32;
pub const MM_OTI_611WAVEIN: u32 = 5u32;
pub const MM_OTI_611WAVEOUT: u32 = 6u32;
pub const MM_PACIFICRESEARCH: u32 = 210u32;
pub const MM_PCSPEAKER_WAVEOUT: u32 = 13u32;
pub const MM_PHILIPS_ACM_LPCBB: u32 = 1u32;
pub const MM_PHILIPS_SPEECH_PROCESSING: u32 = 7u32;
pub const MM_PHONET: u32 = 203u32;
pub const MM_PHONET_PP_MIXER: u32 = 3u32;
pub const MM_PHONET_PP_WAVEIN: u32 = 2u32;
pub const MM_PHONET_PP_WAVEOUT: u32 = 1u32;
pub const MM_PICTURETEL: u32 = 138u32;
pub const MM_PID_UNMAPPED: u32 = 65535u32;
pub const MM_PINNACLE: u32 = 218u32;
pub const MM_PRAGMATRAX: u32 = 5u32;
pub const MM_PRECEPT: u32 = 153u32;
pub const MM_PROAUD_16_AUX: u32 = 103u32;
pub const MM_PROAUD_16_MIDIIN: u32 = 98u32;
pub const MM_PROAUD_16_MIDIOUT: u32 = 97u32;
pub const MM_PROAUD_16_MIXER: u32 = 102u32;
pub const MM_PROAUD_16_SYNTH: u32 = 99u32;
pub const MM_PROAUD_16_WAVEIN: u32 = 101u32;
pub const MM_PROAUD_16_WAVEOUT: u32 = 100u32;
pub const MM_PROAUD_AUX: u32 = 23u32;
pub const MM_PROAUD_MIDIIN: u32 = 18u32;
pub const MM_PROAUD_MIDIOUT: u32 = 17u32;
pub const MM_PROAUD_MIXER: u32 = 22u32;
pub const MM_PROAUD_PLUS_AUX: u32 = 87u32;
pub const MM_PROAUD_PLUS_MIDIIN: u32 = 82u32;
pub const MM_PROAUD_PLUS_MIDIOUT: u32 = 81u32;
pub const MM_PROAUD_PLUS_MIXER: u32 = 86u32;
pub const MM_PROAUD_PLUS_SYNTH: u32 = 83u32;
pub const MM_PROAUD_PLUS_WAVEIN: u32 = 85u32;
pub const MM_PROAUD_PLUS_WAVEOUT: u32 = 84u32;
pub const MM_PROAUD_SYNTH: u32 = 19u32;
pub const MM_PROAUD_WAVEIN: u32 = 21u32;
pub const MM_PROAUD_WAVEOUT: u32 = 20u32;
pub const MM_QCIAR: u32 = 98u32;
pub const MM_QDESIGN: u32 = 194u32;
pub const MM_QDESIGN_ACM_MPEG: u32 = 1u32;
pub const MM_QDESIGN_ACM_QDESIGN_MUSIC: u32 = 2u32;
pub const MM_QTEAM: u32 = 169u32;
pub const MM_QUALCOMM: u32 = 215u32;
pub const MM_QUANTUM3D: u32 = 17u32;
pub const MM_QUARTERDECK: u32 = 134u32;
pub const MM_QUARTERDECK_LHWAVEIN: u32 = 0u32;
pub const MM_QUARTERDECK_LHWAVEOUT: u32 = 1u32;
pub const MM_QUICKAUDIO: u32 = 255u32;
pub const MM_QUICKAUDIO_MAXIMIDI: u32 = 2u32;
pub const MM_QUICKAUDIO_MINIMIDI: u32 = 1u32;
pub const MM_QUICKNET: u32 = 173u32;
pub const MM_QUICKNET_PJWAVEIN: u32 = 1u32;
pub const MM_QUICKNET_PJWAVEOUT: u32 = 2u32;
pub const MM_RADIUS: u32 = 110u32;
pub const MM_RHETOREX: u32 = 120u32;
pub const MM_RHETOREX_WAVEIN: u32 = 1u32;
pub const MM_RHETOREX_WAVEOUT: u32 = 2u32;
pub const MM_RICHMOND: u32 = 257u32;
pub const MM_ROCKWELL: u32 = 111u32;
pub const MM_ROLAND: u32 = 24u32;
pub const MM_ROLAND_MPU401_MIDIIN: u32 = 16u32;
pub const MM_ROLAND_MPU401_MIDIOUT: u32 = 15u32;
pub const MM_ROLAND_RAP10_MIDIIN: u32 = 11u32;
pub const MM_ROLAND_RAP10_MIDIOUT: u32 = 10u32;
pub const MM_ROLAND_RAP10_SYNTH: u32 = 12u32;
pub const MM_ROLAND_RAP10_WAVEIN: u32 = 14u32;
pub const MM_ROLAND_RAP10_WAVEOUT: u32 = 13u32;
pub const MM_ROLAND_SC7_MIDIIN: u32 = 22u32;
pub const MM_ROLAND_SC7_MIDIOUT: u32 = 21u32;
pub const MM_ROLAND_SCP_AUX: u32 = 48u32;
pub const MM_ROLAND_SCP_MIDIIN: u32 = 39u32;
pub const MM_ROLAND_SCP_MIDIOUT: u32 = 38u32;
pub const MM_ROLAND_SCP_MIXER: u32 = 42u32;
pub const MM_ROLAND_SCP_WAVEIN: u32 = 41u32;
pub const MM_ROLAND_SCP_WAVEOUT: u32 = 40u32;
pub const MM_ROLAND_SERIAL_MIDIIN: u32 = 24u32;
pub const MM_ROLAND_SERIAL_MIDIOUT: u32 = 23u32;
pub const MM_ROLAND_SMPU_MIDIINA: u32 = 19u32;
pub const MM_ROLAND_SMPU_MIDIINB: u32 = 20u32;
pub const MM_ROLAND_SMPU_MIDIOUTA: u32 = 17u32;
pub const MM_ROLAND_SMPU_MIDIOUTB: u32 = 18u32;
pub const MM_RZS: u32 = 216u32;
pub const MM_RZS_ACM_TUBGSM: u32 = 1u32;
pub const MM_S3: u32 = 164u32;
pub const MM_S3_AUX: u32 = 7u32;
pub const MM_S3_FMSYNTH: u32 = 5u32;
pub const MM_S3_MIDIIN: u32 = 4u32;
pub const MM_S3_MIDIOUT: u32 = 3u32;
pub const MM_S3_MIXER: u32 = 6u32;
pub const MM_S3_WAVEIN: u32 = 2u32;
pub const MM_S3_WAVEOUT: u32 = 1u32;
pub const MM_SANYO: u32 = 72u32;
pub const MM_SANYO_ACM_LD_ADPCM: u32 = 1u32;
pub const MM_SCALACS: u32 = 54u32;
pub const MM_SEERSYS: u32 = 137u32;
pub const MM_SEERSYS_REALITY: u32 = 6u32;
pub const MM_SEERSYS_SEERMIX: u32 = 3u32;
pub const MM_SEERSYS_SEERSYNTH: u32 = 1u32;
pub const MM_SEERSYS_SEERWAVE: u32 = 2u32;
pub const MM_SEERSYS_WAVESYNTH: u32 = 4u32;
pub const MM_SEERSYS_WAVESYNTH_WG: u32 = 5u32;
pub const MM_SELSIUS_SYSTEMS: u32 = 234u32;
pub const MM_SELSIUS_SYSTEMS_RTPWAVEIN: u32 = 2u32;
pub const MM_SELSIUS_SYSTEMS_RTPWAVEOUT: u32 = 1u32;
pub const MM_SGI: u32 = 237u32;
pub const MM_SGI_320_MIXER: u32 = 3u32;
pub const MM_SGI_320_WAVEIN: u32 = 1u32;
pub const MM_SGI_320_WAVEOUT: u32 = 2u32;
pub const MM_SGI_540_MIXER: u32 = 6u32;
pub const MM_SGI_540_WAVEIN: u32 = 4u32;
pub const MM_SGI_540_WAVEOUT: u32 = 5u32;
pub const MM_SGI_RAD_ADAT8CHAN_WAVEIN: u32 = 19u32;
pub const MM_SGI_RAD_ADAT8CHAN_WAVEOUT: u32 = 32u32;
pub const MM_SGI_RAD_ADATMONO1_WAVEIN: u32 = 7u32;
pub const MM_SGI_RAD_ADATMONO1_WAVEOUT: u32 = 20u32;
pub const MM_SGI_RAD_ADATMONO2_WAVEIN: u32 = 8u32;
pub const MM_SGI_RAD_ADATMONO2_WAVEOUT: u32 = 21u32;
pub const MM_SGI_RAD_ADATMONO3_WAVEIN: u32 = 9u32;
pub const MM_SGI_RAD_ADATMONO3_WAVEOUT: u32 = 22u32;
pub const MM_SGI_RAD_ADATMONO4_WAVEIN: u32 = 10u32;
pub const MM_SGI_RAD_ADATMONO4_WAVEOUT: u32 = 23u32;
pub const MM_SGI_RAD_ADATMONO5_WAVEIN: u32 = 11u32;
pub const MM_SGI_RAD_ADATMONO5_WAVEOUT: u32 = 24u32;
pub const MM_SGI_RAD_ADATMONO6_WAVEIN: u32 = 12u32;
pub const MM_SGI_RAD_ADATMONO6_WAVEOUT: u32 = 25u32;
pub const MM_SGI_RAD_ADATMONO7_WAVEIN: u32 = 13u32;
pub const MM_SGI_RAD_ADATMONO7_WAVEOUT: u32 = 26u32;
pub const MM_SGI_RAD_ADATMONO8_WAVEIN: u32 = 14u32;
pub const MM_SGI_RAD_ADATMONO8_WAVEOUT: u32 = 27u32;
pub const MM_SGI_RAD_ADATSTEREO12_WAVEIN: u32 = 15u32;
pub const MM_SGI_RAD_ADATSTEREO12_WAVEOUT: u32 = 28u32;
pub const MM_SGI_RAD_ADATSTEREO32_WAVEOUT: u32 = 29u32;
pub const MM_SGI_RAD_ADATSTEREO34_WAVEIN: u32 = 16u32;
pub const MM_SGI_RAD_ADATSTEREO56_WAVEIN: u32 = 17u32;
pub const MM_SGI_RAD_ADATSTEREO56_WAVEOUT: u32 = 30u32;
pub const MM_SGI_RAD_ADATSTEREO78_WAVEIN: u32 = 18u32;
pub const MM_SGI_RAD_ADATSTEREO78_WAVEOUT: u32 = 31u32;
pub const MM_SGI_RAD_AESMONO1_WAVEIN: u32 = 33u32;
pub const MM_SGI_RAD_AESMONO1_WAVEOUT: u32 = 36u32;
pub const MM_SGI_RAD_AESMONO2_WAVEIN: u32 = 34u32;
pub const MM_SGI_RAD_AESMONO2_WAVEOUT: u32 = 37u32;
pub const MM_SGI_RAD_AESSTEREO_WAVEIN: u32 = 35u32;
pub const MM_SGI_RAD_AESSTEREO_WAVEOUT: u32 = 38u32;
pub const MM_SHARP: u32 = 183u32;
pub const MM_SHARP_MDC_AUX: u32 = 6u32;
pub const MM_SHARP_MDC_AUX_BASS: u32 = 101u32;
pub const MM_SHARP_MDC_AUX_CHR: u32 = 109u32;
pub const MM_SHARP_MDC_AUX_MASTER: u32 = 100u32;
pub const MM_SHARP_MDC_AUX_MIDI_VOL: u32 = 103u32;
pub const MM_SHARP_MDC_AUX_RVB: u32 = 108u32;
pub const MM_SHARP_MDC_AUX_TREBLE: u32 = 102u32;
pub const MM_SHARP_MDC_AUX_VOL: u32 = 107u32;
pub const MM_SHARP_MDC_AUX_WAVE_CHR: u32 = 106u32;
pub const MM_SHARP_MDC_AUX_WAVE_RVB: u32 = 105u32;
pub const MM_SHARP_MDC_AUX_WAVE_VOL: u32 = 104u32;
pub const MM_SHARP_MDC_MIDI_IN: u32 = 2u32;
pub const MM_SHARP_MDC_MIDI_OUT: u32 = 3u32;
pub const MM_SHARP_MDC_MIDI_SYNTH: u32 = 1u32;
pub const MM_SHARP_MDC_MIXER: u32 = 10u32;
pub const MM_SHARP_MDC_WAVE_IN: u32 = 4u32;
pub const MM_SHARP_MDC_WAVE_OUT: u32 = 5u32;
pub const MM_SICRESOURCE: u32 = 175u32;
pub const MM_SICRESOURCE_SSO3D: u32 = 2u32;
pub const MM_SICRESOURCE_SSOW3DI: u32 = 3u32;
pub const MM_SIEMENS_SBC: u32 = 201u32;
pub const MM_SIERRA: u32 = 40u32;
pub const MM_SIERRA_ARIA_AUX: u32 = 25u32;
pub const MM_SIERRA_ARIA_AUX2: u32 = 32u32;
pub const MM_SIERRA_ARIA_MIDIIN: u32 = 21u32;
pub const MM_SIERRA_ARIA_MIDIOUT: u32 = 20u32;
pub const MM_SIERRA_ARIA_SYNTH: u32 = 22u32;
pub const MM_SIERRA_ARIA_WAVEIN: u32 = 24u32;
pub const MM_SIERRA_ARIA_WAVEOUT: u32 = 23u32;
pub const MM_SIERRA_QUARTET_AUX_CD: u32 = 85u32;
pub const MM_SIERRA_QUARTET_AUX_LINE: u32 = 86u32;
pub const MM_SIERRA_QUARTET_AUX_MODEM: u32 = 87u32;
pub const MM_SIERRA_QUARTET_MIDIIN: u32 = 82u32;
pub const MM_SIERRA_QUARTET_MIDIOUT: u32 = 83u32;
pub const MM_SIERRA_QUARTET_MIXER: u32 = 88u32;
pub const MM_SIERRA_QUARTET_SYNTH: u32 = 84u32;
pub const MM_SIERRA_QUARTET_WAVEIN: u32 = 80u32;
pub const MM_SIERRA_QUARTET_WAVEOUT: u32 = 81u32;
pub const MM_SILICONSOFT: u32 = 69u32;
pub const MM_SILICONSOFT_SC1_WAVEIN: u32 = 1u32;
pub const MM_SILICONSOFT_SC1_WAVEOUT: u32 = 2u32;
pub const MM_SILICONSOFT_SC2_WAVEIN: u32 = 3u32;
pub const MM_SILICONSOFT_SC2_WAVEOUT: u32 = 4u32;
pub const MM_SILICONSOFT_SOUNDJR2PR_WAVEIN: u32 = 6u32;
pub const MM_SILICONSOFT_SOUNDJR2PR_WAVEOUT: u32 = 7u32;
pub const MM_SILICONSOFT_SOUNDJR2_WAVEOUT: u32 = 5u32;
pub const MM_SILICONSOFT_SOUNDJR3_WAVEOUT: u32 = 8u32;
pub const MM_SIPROLAB: u32 = 211u32;
pub const MM_SIPROLAB_ACELPNET: u32 = 1u32;
pub const MM_SNI: u32 = 18u32;
pub const MM_SNI_ACM_G721: u32 = 1u32;
pub const MM_SOFTLAB_NSK: u32 = 228u32;
pub const MM_SOFTLAB_NSK_FRW_AUX: u32 = 4u32;
pub const MM_SOFTLAB_NSK_FRW_MIXER: u32 = 3u32;
pub const MM_SOFTLAB_NSK_FRW_WAVEIN: u32 = 1u32;
pub const MM_SOFTLAB_NSK_FRW_WAVEOUT: u32 = 2u32;
pub const MM_SOFTSOUND: u32 = 149u32;
pub const MM_SOFTSOUND_CODEC: u32 = 1u32;
pub const MM_SONICFOUNDRY: u32 = 66u32;
pub const MM_SONORUS: u32 = 230u32;
pub const MM_SONORUS_STUDIO: u32 = 1u32;
pub const MM_SONY: u32 = 245u32;
pub const MM_SONY_ACM_SCX: u32 = 1u32;
pub const MM_SORVIS: u32 = 187u32;
pub const MM_SOUNDESIGNS: u32 = 142u32;
pub const MM_SOUNDESIGNS_WAVEIN: u32 = 1u32;
pub const MM_SOUNDESIGNS_WAVEOUT: u32 = 2u32;
pub const MM_SOUNDSCAPE_AUX: u32 = 24u32;
pub const MM_SOUNDSCAPE_MIDIIN: u32 = 21u32;
pub const MM_SOUNDSCAPE_MIDIOUT: u32 = 20u32;
pub const MM_SOUNDSCAPE_MIXER: u32 = 23u32;
pub const MM_SOUNDSCAPE_SYNTH: u32 = 22u32;
pub const MM_SOUNDSCAPE_WAVEIN: u32 = 19u32;
pub const MM_SOUNDSCAPE_WAVEOUT: u32 = 17u32;
pub const MM_SOUNDSCAPE_WAVEOUT_AUX: u32 = 18u32;
pub const MM_SOUNDSPACE: u32 = 167u32;
pub const MM_SPECTRUM_PRODUCTIONS: u32 = 213u32;
pub const MM_SPECTRUM_SIGNAL_PROCESSING: u32 = 144u32;
pub const MM_SPEECHCOMP: u32 = 76u32;
pub const MM_SPLASH_STUDIOS: u32 = 133u32;
pub const MM_SSP_SNDFESAUX: u32 = 7u32;
pub const MM_SSP_SNDFESMIDIIN: u32 = 3u32;
pub const MM_SSP_SNDFESMIDIOUT: u32 = 4u32;
pub const MM_SSP_SNDFESMIX: u32 = 6u32;
pub const MM_SSP_SNDFESSYNTH: u32 = 5u32;
pub const MM_SSP_SNDFESWAVEIN: u32 = 1u32;
pub const MM_SSP_SNDFESWAVEOUT: u32 = 2u32;
pub const MM_STUDER: u32 = 171u32;
pub const MM_STUDIO_16_AUX: u32 = 103u32;
pub const MM_STUDIO_16_MIDIIN: u32 = 98u32;
pub const MM_STUDIO_16_MIDIOUT: u32 = 97u32;
pub const MM_STUDIO_16_MIXER: u32 = 102u32;
pub const MM_STUDIO_16_SYNTH: u32 = 99u32;
pub const MM_STUDIO_16_WAVEIN: u32 = 101u32;
pub const MM_STUDIO_16_WAVEOUT: u32 = 100u32;
pub const MM_ST_MICROELECTRONICS: u32 = 265u32;
pub const MM_SUNCOM: u32 = 186u32;
pub const MM_SUPERMAC: u32 = 73u32;
pub const MM_SYDEC_NV: u32 = 248u32;
pub const MM_SYDEC_NV_WAVEIN: u32 = 1u32;
pub const MM_SYDEC_NV_WAVEOUT: u32 = 2u32;
pub const MM_TANDY: u32 = 29u32;
pub const MM_TANDY_PSSJWAVEIN: u32 = 9u32;
pub const MM_TANDY_PSSJWAVEOUT: u32 = 10u32;
pub const MM_TANDY_SENS_MMAMIDIIN: u32 = 6u32;
pub const MM_TANDY_SENS_MMAMIDIOUT: u32 = 7u32;
pub const MM_TANDY_SENS_MMAWAVEIN: u32 = 4u32;
pub const MM_TANDY_SENS_MMAWAVEOUT: u32 = 5u32;
pub const MM_TANDY_SENS_VISWAVEOUT: u32 = 8u32;
pub const MM_TANDY_VISBIOSSYNTH: u32 = 3u32;
pub const MM_TANDY_VISWAVEIN: u32 = 1u32;
pub const MM_TANDY_VISWAVEOUT: u32 = 2u32;
pub const MM_TBS_TROPEZ_AUX1: u32 = 39u32;
pub const MM_TBS_TROPEZ_AUX2: u32 = 40u32;
pub const MM_TBS_TROPEZ_LINE: u32 = 41u32;
pub const MM_TBS_TROPEZ_WAVEIN: u32 = 37u32;
pub const MM_TBS_TROPEZ_WAVEOUT: u32 = 38u32;
pub const MM_TDK: u32 = 135u32;
pub const MM_TDK_MW_AUX: u32 = 6u32;
pub const MM_TDK_MW_AUX_BASS: u32 = 101u32;
pub const MM_TDK_MW_AUX_CHR: u32 = 109u32;
pub const MM_TDK_MW_AUX_MASTER: u32 = 100u32;
pub const MM_TDK_MW_AUX_MIDI_VOL: u32 = 103u32;
pub const MM_TDK_MW_AUX_RVB: u32 = 108u32;
pub const MM_TDK_MW_AUX_TREBLE: u32 = 102u32;
pub const MM_TDK_MW_AUX_VOL: u32 = 107u32;
pub const MM_TDK_MW_AUX_WAVE_CHR: u32 = 106u32;
pub const MM_TDK_MW_AUX_WAVE_RVB: u32 = 105u32;
pub const MM_TDK_MW_AUX_WAVE_VOL: u32 = 104u32;
pub const MM_TDK_MW_MIDI_IN: u32 = 2u32;
pub const MM_TDK_MW_MIDI_OUT: u32 = 3u32;
pub const MM_TDK_MW_MIDI_SYNTH: u32 = 1u32;
pub const MM_TDK_MW_MIXER: u32 = 10u32;
pub const MM_TDK_MW_WAVE_IN: u32 = 4u32;
pub const MM_TDK_MW_WAVE_OUT: u32 = 5u32;
pub const MM_TELEKOL: u32 = 264u32;
pub const MM_TELEKOL_WAVEIN: u32 = 2u32;
pub const MM_TELEKOL_WAVEOUT: u32 = 1u32;
pub const MM_TERALOGIC: u32 = 202u32;
pub const MM_TERRATEC: u32 = 70u32;
pub const MM_THUNDER_AUX: u32 = 39u32;
pub const MM_THUNDER_SYNTH: u32 = 35u32;
pub const MM_THUNDER_WAVEIN: u32 = 37u32;
pub const MM_THUNDER_WAVEOUT: u32 = 36u32;
pub const MM_TPORT_SYNTH: u32 = 67u32;
pub const MM_TPORT_WAVEIN: u32 = 66u32;
pub const MM_TPORT_WAVEOUT: u32 = 65u32;
pub const MM_TRUEVISION: u32 = 51u32;
pub const MM_TRUEVISION_WAVEIN1: u32 = 1u32;
pub const MM_TRUEVISION_WAVEOUT1: u32 = 2u32;
pub const MM_TTEWS_AUX: u32 = 9u32;
pub const MM_TTEWS_MIDIIN: u32 = 3u32;
pub const MM_TTEWS_MIDIMONITOR: u32 = 6u32;
pub const MM_TTEWS_MIDIOUT: u32 = 4u32;
pub const MM_TTEWS_MIDISYNTH: u32 = 5u32;
pub const MM_TTEWS_MIXER: u32 = 10u32;
pub const MM_TTEWS_VMIDIIN: u32 = 7u32;
pub const MM_TTEWS_VMIDIOUT: u32 = 8u32;
pub const MM_TTEWS_WAVEIN: u32 = 1u32;
pub const MM_TTEWS_WAVEOUT: u32 = 2u32;
pub const MM_TURTLE_BEACH: u32 = 21u32;
pub const MM_UHER_INFORMATIC: u32 = 247u32;
pub const MM_UH_ACM_ADPCM: u32 = 1u32;
pub const MM_UNISYS: u32 = 223u32;
pub const MM_UNISYS_ACM_NAP: u32 = 1u32;
pub const MM_UNMAPPED: u32 = 65535u32;
pub const MM_VAL: u32 = 35u32;
pub const MM_VAL_MICROKEY_AP_WAVEIN: u32 = 1u32;
pub const MM_VAL_MICROKEY_AP_WAVEOUT: u32 = 2u32;
pub const MM_VANKOEVERING: u32 = 168u32;
pub const MM_VIA: u32 = 250u32;
pub const MM_VIA_AUX: u32 = 4u32;
pub const MM_VIA_MIXER: u32 = 3u32;
pub const MM_VIA_MPU401_MIDIIN: u32 = 6u32;
pub const MM_VIA_MPU401_MIDIOUT: u32 = 5u32;
pub const MM_VIA_SWFM_SYNTH: u32 = 7u32;
pub const MM_VIA_WAVEIN: u32 = 2u32;
pub const MM_VIA_WAVEOUT: u32 = 1u32;
pub const MM_VIA_WDM_MIXER: u32 = 10u32;
pub const MM_VIA_WDM_MPU401_MIDIIN: u32 = 12u32;
pub const MM_VIA_WDM_MPU401_MIDIOUT: u32 = 11u32;
pub const MM_VIA_WDM_WAVEIN: u32 = 9u32;
pub const MM_VIA_WDM_WAVEOUT: u32 = 8u32;
pub const MM_VIDEOLOGIC: u32 = 53u32;
pub const MM_VIDEOLOGIC_MSWAVEIN: u32 = 1u32;
pub const MM_VIDEOLOGIC_MSWAVEOUT: u32 = 2u32;
pub const MM_VIENNASYS: u32 = 157u32;
pub const MM_VIENNASYS_TSP_WAVE_DRIVER: u32 = 1u32;
pub const MM_VIONA: u32 = 161u32;
pub const MM_VIONAQVINPCI_WAVEOUT: u32 = 3u32;
pub const MM_VIONA_BUSTER_MIXER: u32 = 4u32;
pub const MM_VIONA_CINEMASTER_MIXER: u32 = 5u32;
pub const MM_VIONA_CONCERTO_MIXER: u32 = 6u32;
pub const MM_VIONA_QVINPCI_MIXER: u32 = 1u32;
pub const MM_VIONA_QVINPCI_WAVEIN: u32 = 2u32;
pub const MM_VIRTUALMUSIC: u32 = 205u32;
pub const MM_VITEC: u32 = 67u32;
pub const MM_VITEC_VMAKER: u32 = 1u32;
pub const MM_VITEC_VMPRO: u32 = 2u32;
pub const MM_VIVO: u32 = 182u32;
pub const MM_VIVO_AUDIO_CODEC: u32 = 1u32;
pub const MM_VKC_MPU401_MIDIIN: u32 = 256u32;
pub const MM_VKC_MPU401_MIDIOUT: u32 = 512u32;
pub const MM_VKC_SERIAL_MIDIIN: u32 = 257u32;
pub const MM_VKC_SERIAL_MIDIOUT: u32 = 513u32;
pub const MM_VOCALTEC: u32 = 23u32;
pub const MM_VOCALTEC_WAVEIN: u32 = 2u32;
pub const MM_VOCALTEC_WAVEOUT: u32 = 1u32;
pub const MM_VOICEINFO: u32 = 156u32;
pub const MM_VOICEMIXER: u32 = 1u32;
pub const MM_VOXWARE: u32 = 114u32;
pub const MM_VOXWARE_CODEC: u32 = 1u32;
pub const MM_VOYETRA: u32 = 30u32;
pub const MM_VQST: u32 = 240u32;
pub const MM_VQST_VQC1: u32 = 1u32;
pub const MM_VQST_VQC2: u32 = 2u32;
pub const MM_VTG: u32 = 109u32;
pub const MM_WANGLABS: u32 = 28u32;
pub const MM_WANGLABS_WAVEIN1: u32 = 1u32;
pub const MM_WANGLABS_WAVEOUT1: u32 = 2u32;
pub const MM_WEITEK: u32 = 96u32;
pub const MM_WILDCAT: u32 = 119u32;
pub const MM_WILDCAT_AUTOSCOREMIDIIN: u32 = 1u32;
pub const MM_WILLOPOND_SNDCOMM_WAVEIN: u32 = 108u32;
pub const MM_WILLOWPOND: u32 = 65u32;
pub const MM_WILLOWPOND_FMSYNTH_STEREO: u32 = 20u32;
pub const MM_WILLOWPOND_GENERIC_AUX: u32 = 115u32;
pub const MM_WILLOWPOND_GENERIC_MIXER: u32 = 114u32;
pub const MM_WILLOWPOND_GENERIC_WAVEIN: u32 = 112u32;
pub const MM_WILLOWPOND_GENERIC_WAVEOUT: u32 = 113u32;
pub const MM_WILLOWPOND_MPU401: u32 = 21u32;
pub const MM_WILLOWPOND_PH_AUX: u32 = 107u32;
pub const MM_WILLOWPOND_PH_MIXER: u32 = 106u32;
pub const MM_WILLOWPOND_PH_WAVEIN: u32 = 104u32;
pub const MM_WILLOWPOND_PH_WAVEOUT: u32 = 105u32;
pub const MM_WILLOWPOND_SNDCOMM_AUX: u32 = 111u32;
pub const MM_WILLOWPOND_SNDCOMM_MIXER: u32 = 110u32;
pub const MM_WILLOWPOND_SNDCOMM_WAVEOUT: u32 = 109u32;
pub const MM_WILLOWPOND_SNDPORT_AUX: u32 = 103u32;
pub const MM_WILLOWPOND_SNDPORT_MIXER: u32 = 102u32;
pub const MM_WILLOWPOND_SNDPORT_WAVEIN: u32 = 100u32;
pub const MM_WILLOWPOND_SNDPORT_WAVEOUT: u32 = 101u32;
pub const MM_WINBOND: u32 = 204u32;
pub const MM_WINNOV: u32 = 61u32;
pub const MM_WINNOV_CAVIAR_CHAMPAGNE: u32 = 4u32;
pub const MM_WINNOV_CAVIAR_VIDC: u32 = 3u32;
pub const MM_WINNOV_CAVIAR_WAVEIN: u32 = 1u32;
pub const MM_WINNOV_CAVIAR_WAVEOUT: u32 = 2u32;
pub const MM_WINNOV_CAVIAR_YUV8: u32 = 5u32;
pub const MM_WORKBIT: u32 = 102u32;
pub const MM_WORKBIT_AUX: u32 = 7u32;
pub const MM_WORKBIT_FMSYNTH: u32 = 6u32;
pub const MM_WORKBIT_JOYSTICK: u32 = 8u32;
pub const MM_WORKBIT_MIDIIN: u32 = 4u32;
pub const MM_WORKBIT_MIDIOUT: u32 = 5u32;
pub const MM_WORKBIT_MIXER: u32 = 1u32;
pub const MM_WORKBIT_WAVEIN: u32 = 3u32;
pub const MM_WORKBIT_WAVEOUT: u32 = 2u32;
pub const MM_WSS_SB16_AUX_CD: u32 = 45u32;
pub const MM_WSS_SB16_AUX_LINE: u32 = 44u32;
pub const MM_WSS_SB16_MIDIIN: u32 = 41u32;
pub const MM_WSS_SB16_MIDIOUT: u32 = 42u32;
pub const MM_WSS_SB16_MIXER: u32 = 46u32;
pub const MM_WSS_SB16_SYNTH: u32 = 43u32;
pub const MM_WSS_SB16_WAVEIN: u32 = 39u32;
pub const MM_WSS_SB16_WAVEOUT: u32 = 40u32;
pub const MM_WSS_SBPRO_AUX_CD: u32 = 53u32;
pub const MM_WSS_SBPRO_AUX_LINE: u32 = 52u32;
pub const MM_WSS_SBPRO_MIDIIN: u32 = 49u32;
pub const MM_WSS_SBPRO_MIDIOUT: u32 = 50u32;
pub const MM_WSS_SBPRO_MIXER: u32 = 54u32;
pub const MM_WSS_SBPRO_SYNTH: u32 = 51u32;
pub const MM_WSS_SBPRO_WAVEIN: u32 = 47u32;
pub const MM_WSS_SBPRO_WAVEOUT: u32 = 48u32;
pub const MM_XEBEC: u32 = 85u32;
pub const MM_XIRLINK: u32 = 178u32;
pub const MM_XIRLINK_VISIONLINK: u32 = 1u32;
pub const MM_XYZ: u32 = 112u32;
pub const MM_YAMAHA: u32 = 37u32;
pub const MM_YAMAHA_ACXG_AUX: u32 = 41u32;
pub const MM_YAMAHA_ACXG_MIDIOUT: u32 = 39u32;
pub const MM_YAMAHA_ACXG_MIXER: u32 = 40u32;
pub const MM_YAMAHA_ACXG_WAVEIN: u32 = 37u32;
pub const MM_YAMAHA_ACXG_WAVEOUT: u32 = 38u32;
pub const MM_YAMAHA_GSS_AUX: u32 = 6u32;
pub const MM_YAMAHA_GSS_MIDIIN: u32 = 5u32;
pub const MM_YAMAHA_GSS_MIDIOUT: u32 = 4u32;
pub const MM_YAMAHA_GSS_SYNTH: u32 = 1u32;
pub const MM_YAMAHA_GSS_WAVEIN: u32 = 3u32;
pub const MM_YAMAHA_GSS_WAVEOUT: u32 = 2u32;
pub const MM_YAMAHA_OPL3SA_FMSYNTH: u32 = 18u32;
pub const MM_YAMAHA_OPL3SA_JOYSTICK: u32 = 24u32;
pub const MM_YAMAHA_OPL3SA_MIDIIN: u32 = 21u32;
pub const MM_YAMAHA_OPL3SA_MIDIOUT: u32 = 20u32;
pub const MM_YAMAHA_OPL3SA_MIXER: u32 = 23u32;
pub const MM_YAMAHA_OPL3SA_WAVEIN: u32 = 17u32;
pub const MM_YAMAHA_OPL3SA_WAVEOUT: u32 = 16u32;
pub const MM_YAMAHA_OPL3SA_YSYNTH: u32 = 19u32;
pub const MM_YAMAHA_SERIAL_MIDIIN: u32 = 8u32;
pub const MM_YAMAHA_SERIAL_MIDIOUT: u32 = 7u32;
pub const MM_YAMAHA_SXG_MIDIOUT: u32 = 34u32;
pub const MM_YAMAHA_SXG_MIXER: u32 = 36u32;
pub const MM_YAMAHA_SXG_WAVEOUT: u32 = 35u32;
pub const MM_YAMAHA_YMF724LEG_FMSYNTH: u32 = 32u32;
pub const MM_YAMAHA_YMF724LEG_MIDIIN: u32 = 26u32;
pub const MM_YAMAHA_YMF724LEG_MIDIOUT: u32 = 25u32;
pub const MM_YAMAHA_YMF724LEG_MIXER: u32 = 33u32;
pub const MM_YAMAHA_YMF724_AUX: u32 = 30u32;
pub const MM_YAMAHA_YMF724_MIDIOUT: u32 = 29u32;
pub const MM_YAMAHA_YMF724_MIXER: u32 = 31u32;
pub const MM_YAMAHA_YMF724_WAVEIN: u32 = 28u32;
pub const MM_YAMAHA_YMF724_WAVEOUT: u32 = 27u32;
pub const MM_YOUCOM: u32 = 256u32;
pub const MM_ZEFIRO: u32 = 170u32;
pub const MM_ZEFIRO_ZA2: u32 = 2u32;
pub const MM_ZYXEL: u32 = 9u32;
pub const MM_ZYXEL_ACM_ADPCM: u32 = 1u32;
pub const MODM_CACHEDRUMPATCHES: u32 = 13u32;
pub const MODM_CACHEPATCHES: u32 = 12u32;
pub const MODM_CLOSE: u32 = 4u32;
pub const MODM_DATA: u32 = 7u32;
pub const MODM_GETDEVCAPS: u32 = 2u32;
pub const MODM_GETNUMDEVS: u32 = 1u32;
pub const MODM_GETPOS: u32 = 17u32;
pub const MODM_GETVOLUME: u32 = 10u32;
pub const MODM_INIT: u32 = 100u32;
pub const MODM_INIT_EX: u32 = 104u32;
pub const MODM_LONGDATA: u32 = 8u32;
pub const MODM_MAPPER: u32 = 8192u32;
pub const MODM_OPEN: u32 = 3u32;
pub const MODM_PAUSE: u32 = 18u32;
pub const MODM_PREFERRED: u32 = 22u32;
pub const MODM_PREPARE: u32 = 5u32;
pub const MODM_PROPERTIES: u32 = 21u32;
pub const MODM_RECONFIGURE: u32 = 18280u32;
pub const MODM_RESET: u32 = 9u32;
pub const MODM_RESTART: u32 = 19u32;
pub const MODM_SETVOLUME: u32 = 11u32;
pub const MODM_STOP: u32 = 20u32;
pub const MODM_STRMDATA: u32 = 14u32;
pub const MODM_UNPREPARE: u32 = 6u32;
pub const MODM_USER: u32 = 16384u32;
pub const MPEGLAYER3_ID_CONSTANTFRAMESIZE: u32 = 2u32;
pub const MPEGLAYER3_ID_MPEG: u32 = 1u32;
pub const MPEGLAYER3_ID_UNKNOWN: u32 = 0u32;
pub const MPEGLAYER3_WFX_EXTRA_BYTES: u32 = 12u32;
pub struct MSAUDIO1WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
    pub wEncodeOptions: u16,
}
impl ::core::marker::Copy for MSAUDIO1WAVEFORMAT {}
impl ::core::clone::Clone for MSAUDIO1WAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MSAUDIO1WAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSAUDIO1WAVEFORMAT")
            .field("wfx", &self.wfx)
            .field("wSamplesPerBlock", &self.wSamplesPerBlock)
            .field("wEncodeOptions", &self.wEncodeOptions)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MSAUDIO1WAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx
            && self.wSamplesPerBlock == other.wSamplesPerBlock
            && self.wEncodeOptions == other.wEncodeOptions
    }
}
impl ::core::cmp::Eq for MSAUDIO1WAVEFORMAT {}
impl FromIntoMemory for MSAUDIO1WAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 22);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        let f_wSamplesPerBlock = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        let f_wEncodeOptions = <u16 as FromIntoMemory>::from_bytes(&from[20..20 + 2]);
        Self {
            wfx: f_wfx,
            wSamplesPerBlock: f_wSamplesPerBlock,
            wEncodeOptions: f_wEncodeOptions,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 22);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
        FromIntoMemory::into_bytes(self.wSamplesPerBlock, &mut into[18..18 + 2]);
        FromIntoMemory::into_bytes(self.wEncodeOptions, &mut into[20..20 + 2]);
    }
    fn size() -> usize {
        22
    }
}
pub const MSAUDIO1_BITS_PER_SAMPLE: u32 = 16u32;
pub const MSAUDIO1_MAX_CHANNELS: u32 = 2u32;
pub const MXDM_BASE: u32 = 1u32;
pub const MXDM_CLOSE: u32 = 4u32;
pub const MXDM_GETCONTROLDETAILS: u32 = 7u32;
pub const MXDM_GETDEVCAPS: u32 = 2u32;
pub const MXDM_GETLINECONTROLS: u32 = 6u32;
pub const MXDM_GETLINEINFO: u32 = 5u32;
pub const MXDM_GETNUMDEVS: u32 = 1u32;
pub const MXDM_INIT: u32 = 100u32;
pub const MXDM_INIT_EX: u32 = 104u32;
pub const MXDM_OPEN: u32 = 3u32;
pub const MXDM_SETCONTROLDETAILS: u32 = 8u32;
pub const MXDM_USER: u32 = 16384u32;
pub struct NMS_VBXADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
impl ::core::marker::Copy for NMS_VBXADPCMWAVEFORMAT {}
impl ::core::clone::Clone for NMS_VBXADPCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMS_VBXADPCMWAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMS_VBXADPCMWAVEFORMAT")
            .field("wfx", &self.wfx)
            .field("wSamplesPerBlock", &self.wSamplesPerBlock)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NMS_VBXADPCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx && self.wSamplesPerBlock == other.wSamplesPerBlock
    }
}
impl ::core::cmp::Eq for NMS_VBXADPCMWAVEFORMAT {}
impl FromIntoMemory for NMS_VBXADPCMWAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        let f_wSamplesPerBlock = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        Self {
            wfx: f_wfx,
            wSamplesPerBlock: f_wSamplesPerBlock,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
        FromIntoMemory::into_bytes(self.wSamplesPerBlock, &mut into[18..18 + 2]);
    }
    fn size() -> usize {
        20
    }
}
pub const NS_DRM_E_MIGRATION_IMAGE_ALREADY_EXISTS: crate::core::HRESULT =
    crate::core::HRESULT(-1072879730i32);
pub const NS_DRM_E_MIGRATION_SOURCE_MACHINE_IN_USE: crate::core::HRESULT =
    crate::core::HRESULT(-1072879732i32);
pub const NS_DRM_E_MIGRATION_TARGET_MACHINE_LESS_THAN_LH: crate::core::HRESULT =
    crate::core::HRESULT(-1072879731i32);
pub const NS_DRM_E_MIGRATION_UPGRADE_WITH_DIFF_SID: crate::core::HRESULT =
    crate::core::HRESULT(-1072879733i32);
pub const NS_E_8BIT_WAVE_UNSUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-1072886834i32);
pub const NS_E_ACTIVE_SG_DEVICE_CONTROL_DISCONNECTED: crate::core::HRESULT =
    crate::core::HRESULT(-1072882778i32);
pub const NS_E_ACTIVE_SG_DEVICE_DISCONNECTED: crate::core::HRESULT =
    crate::core::HRESULT(-1072882779i32);
pub const NS_E_ADVANCEDEDIT_TOO_MANY_PICTURES: crate::core::HRESULT =
    crate::core::HRESULT(-1072884886i32);
pub const NS_E_ALLOCATE_FILE_FAIL: crate::core::HRESULT = crate::core::HRESULT(-1072889759i32);
pub const NS_E_ALL_PROTOCOLS_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-1072877845i32);
pub const NS_E_ALREADY_CONNECTED: crate::core::HRESULT = crate::core::HRESULT(-1072889840i32);
pub const NS_E_ANALOG_VIDEO_PROTECTION_LEVEL_UNSUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1072879353i32);
pub const NS_E_ARCHIVE_ABORT_DUE_TO_BCAST: crate::core::HRESULT =
    crate::core::HRESULT(-1072884338i32);
pub const NS_E_ARCHIVE_FILENAME_NOTSET: crate::core::HRESULT = crate::core::HRESULT(-1072882823i32);
pub const NS_E_ARCHIVE_GAP_DETECTED: crate::core::HRESULT = crate::core::HRESULT(-1072884337i32);
pub const NS_E_ARCHIVE_REACH_QUOTA: crate::core::HRESULT = crate::core::HRESULT(-1072884339i32);
pub const NS_E_ARCHIVE_SAME_AS_INPUT: crate::core::HRESULT = crate::core::HRESULT(-1072882812i32);
pub const NS_E_ASSERT: crate::core::HRESULT = crate::core::HRESULT(-1072889653i32);
pub const NS_E_ASX_INVALIDFORMAT: crate::core::HRESULT = crate::core::HRESULT(-1072885655i32);
pub const NS_E_ASX_INVALIDVERSION: crate::core::HRESULT = crate::core::HRESULT(-1072885654i32);
pub const NS_E_ASX_INVALID_REPEAT_BLOCK: crate::core::HRESULT =
    crate::core::HRESULT(-1072885653i32);
pub const NS_E_ASX_NOTHING_TO_WRITE: crate::core::HRESULT = crate::core::HRESULT(-1072885652i32);
pub const NS_E_ATTRIBUTE_NOT_ALLOWED: crate::core::HRESULT = crate::core::HRESULT(-1072886825i32);
pub const NS_E_ATTRIBUTE_READ_ONLY: crate::core::HRESULT = crate::core::HRESULT(-1072886826i32);
pub const NS_E_AUDIENCE_CONTENTTYPE_MISMATCH: crate::core::HRESULT =
    crate::core::HRESULT(-1072882791i32);
pub const NS_E_AUDIENCE__LANGUAGE_CONTENTTYPE_MISMATCH: crate::core::HRESULT =
    crate::core::HRESULT(-1072882717i32);
pub const NS_E_AUDIODEVICE_BADFORMAT: crate::core::HRESULT = crate::core::HRESULT(-1072882845i32);
pub const NS_E_AUDIODEVICE_BUSY: crate::core::HRESULT = crate::core::HRESULT(-1072882847i32);
pub const NS_E_AUDIODEVICE_UNEXPECTED: crate::core::HRESULT = crate::core::HRESULT(-1072882846i32);
pub const NS_E_AUDIO_BITRATE_STEPDOWN: crate::core::HRESULT = crate::core::HRESULT(-1072882759i32);
pub const NS_E_AUDIO_CODEC_ERROR: crate::core::HRESULT = crate::core::HRESULT(-1072886845i32);
pub const NS_E_AUDIO_CODEC_NOT_INSTALLED: crate::core::HRESULT =
    crate::core::HRESULT(-1072886846i32);
pub const NS_E_AUTHORIZATION_FILE_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-1072884336i32);
pub const NS_E_BACKUP_RESTORE_BAD_DATA: crate::core::HRESULT = crate::core::HRESULT(-1072879803i32);
pub const NS_E_BACKUP_RESTORE_BAD_REQUEST_ID: crate::core::HRESULT =
    crate::core::HRESULT(-1072879826i32);
pub const NS_E_BACKUP_RESTORE_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-1072879827i32);
pub const NS_E_BACKUP_RESTORE_TOO_MANY_RESETS: crate::core::HRESULT =
    crate::core::HRESULT(-1072879770i32);
pub const NS_E_BAD_ADAPTER_ADDRESS: crate::core::HRESULT = crate::core::HRESULT(-1072889799i32);
pub const NS_E_BAD_ADAPTER_NAME: crate::core::HRESULT = crate::core::HRESULT(-1072889652i32);
pub const NS_E_BAD_BLOCK0_VERSION: crate::core::HRESULT = crate::core::HRESULT(-1072889757i32);
pub const NS_E_BAD_CONTENTEDL: crate::core::HRESULT = crate::core::HRESULT(-1072882774i32);
pub const NS_E_BAD_CONTROL_DATA: crate::core::HRESULT = crate::core::HRESULT(-1072889806i32);
pub const NS_E_BAD_CUB_UID: crate::core::HRESULT = crate::core::HRESULT(-1072889454i32);
pub const NS_E_BAD_DELIVERY_MODE: crate::core::HRESULT = crate::core::HRESULT(-1072889798i32);
pub const NS_E_BAD_DISK_UID: crate::core::HRESULT = crate::core::HRESULT(-1072889756i32);
pub const NS_E_BAD_FSMAJOR_VERSION: crate::core::HRESULT = crate::core::HRESULT(-1072889755i32);
pub const NS_E_BAD_MARKIN: crate::core::HRESULT = crate::core::HRESULT(-1072882856i32);
pub const NS_E_BAD_MARKOUT: crate::core::HRESULT = crate::core::HRESULT(-1072882855i32);
pub const NS_E_BAD_MULTICAST_ADDRESS: crate::core::HRESULT = crate::core::HRESULT(-1072889800i32);
pub const NS_E_BAD_REQUEST: crate::core::HRESULT = crate::core::HRESULT(-1072877853i32);
pub const NS_E_BAD_STAMPNUMBER: crate::core::HRESULT = crate::core::HRESULT(-1072889754i32);
pub const NS_E_BAD_SYNTAX_IN_SERVER_RESPONSE: crate::core::HRESULT =
    crate::core::HRESULT(-1072877826i32);
pub const NS_E_BKGDOWNLOAD_CALLFUNCENDED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885145i32);
pub const NS_E_BKGDOWNLOAD_CALLFUNCFAILED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885147i32);
pub const NS_E_BKGDOWNLOAD_CALLFUNCTIMEOUT: crate::core::HRESULT =
    crate::core::HRESULT(-1072885146i32);
pub const NS_E_BKGDOWNLOAD_CANCELCOMPLETEDJOB: crate::core::HRESULT =
    crate::core::HRESULT(-1072885153i32);
pub const NS_E_BKGDOWNLOAD_COMPLETECANCELLEDJOB: crate::core::HRESULT =
    crate::core::HRESULT(-1072885154i32);
pub const NS_E_BKGDOWNLOAD_FAILEDINITIALIZE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885143i32);
pub const NS_E_BKGDOWNLOAD_FAILED_TO_CREATE_TEMPFILE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885150i32);
pub const NS_E_BKGDOWNLOAD_INVALIDJOBSIGNATURE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885151i32);
pub const NS_E_BKGDOWNLOAD_INVALID_FILE_NAME: crate::core::HRESULT =
    crate::core::HRESULT(-1072885141i32);
pub const NS_E_BKGDOWNLOAD_NOJOBPOINTER: crate::core::HRESULT =
    crate::core::HRESULT(-1072885152i32);
pub const NS_E_BKGDOWNLOAD_PLUGIN_FAILEDINITIALIZE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885149i32);
pub const NS_E_BKGDOWNLOAD_PLUGIN_FAILEDTOMOVEFILE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885148i32);
pub const NS_E_BKGDOWNLOAD_WMDUNPACKFAILED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885144i32);
pub const NS_E_BKGDOWNLOAD_WRONG_NO_FILES: crate::core::HRESULT =
    crate::core::HRESULT(-1072885155i32);
pub const NS_E_BUSY: crate::core::HRESULT = crate::core::HRESULT(-1072889819i32);
pub const NS_E_CACHE_ARCHIVE_CONFLICT: crate::core::HRESULT = crate::core::HRESULT(-1072884756i32);
pub const NS_E_CACHE_CANNOT_BE_CACHED: crate::core::HRESULT = crate::core::HRESULT(-1072884752i32);
pub const NS_E_CACHE_NOT_BROADCAST: crate::core::HRESULT = crate::core::HRESULT(-1072884753i32);
pub const NS_E_CACHE_NOT_MODIFIED: crate::core::HRESULT = crate::core::HRESULT(-1072884751i32);
pub const NS_E_CACHE_ORIGIN_SERVER_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-1072884755i32);
pub const NS_E_CACHE_ORIGIN_SERVER_TIMEOUT: crate::core::HRESULT =
    crate::core::HRESULT(-1072884754i32);
pub const NS_E_CANNOTCONNECT: crate::core::HRESULT = crate::core::HRESULT(-1072889850i32);
pub const NS_E_CANNOTCONNECTEVENTS: crate::core::HRESULT = crate::core::HRESULT(-1072889745i32);
pub const NS_E_CANNOTDESTROYTITLE: crate::core::HRESULT = crate::core::HRESULT(-1072889849i32);
pub const NS_E_CANNOTOFFLINEDISK: crate::core::HRESULT = crate::core::HRESULT(-1072889847i32);
pub const NS_E_CANNOTONLINEDISK: crate::core::HRESULT = crate::core::HRESULT(-1072889846i32);
pub const NS_E_CANNOTRENAMETITLE: crate::core::HRESULT = crate::core::HRESULT(-1072889848i32);
pub const NS_E_CANNOT_BUY_OR_DOWNLOAD_CONTENT: crate::core::HRESULT =
    crate::core::HRESULT(-1072884904i32);
pub const NS_E_CANNOT_BUY_OR_DOWNLOAD_FROM_MULTIPLE_SERVICES: crate::core::HRESULT =
    crate::core::HRESULT(-1072884905i32);
pub const NS_E_CANNOT_CONNECT_TO_PROXY: crate::core::HRESULT = crate::core::HRESULT(-1072877842i32);
pub const NS_E_CANNOT_DELETE_ACTIVE_SOURCEGROUP: crate::core::HRESULT =
    crate::core::HRESULT(-1072882848i32);
pub const NS_E_CANNOT_GENERATE_BROADCAST_INFO_FOR_QUALITYVBR: crate::core::HRESULT =
    crate::core::HRESULT(-1072882721i32);
pub const NS_E_CANNOT_PAUSE_LIVEBROADCAST: crate::core::HRESULT =
    crate::core::HRESULT(-1072882802i32);
pub const NS_E_CANNOT_READ_PLAYLIST_FROM_MEDIASERVER: crate::core::HRESULT =
    crate::core::HRESULT(-1072877838i32);
pub const NS_E_CANNOT_REMOVE_PLUGIN: crate::core::HRESULT = crate::core::HRESULT(-1072884655i32);
pub const NS_E_CANNOT_REMOVE_PUBLISHING_POINT: crate::core::HRESULT =
    crate::core::HRESULT(-1072884656i32);
pub const NS_E_CANNOT_SYNC_DRM_TO_NON_JANUS_DEVICE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885178i32);
pub const NS_E_CANNOT_SYNC_PREVIOUS_SYNC_RUNNING: crate::core::HRESULT =
    crate::core::HRESULT(-1072885177i32);
pub const NS_E_CANT_READ_DIGITAL: crate::core::HRESULT = crate::core::HRESULT(-1072885855i32);
pub const NS_E_CCLINK_DOWN: crate::core::HRESULT = crate::core::HRESULT(-1072889821i32);
pub const NS_E_CD_COPYTO_CD: crate::core::HRESULT = crate::core::HRESULT(-1072885842i32);
pub const NS_E_CD_DRIVER_PROBLEM: crate::core::HRESULT = crate::core::HRESULT(-1072885838i32);
pub const NS_E_CD_EMPTY_TRACK_QUEUE: crate::core::HRESULT = crate::core::HRESULT(-1072885255i32);
pub const NS_E_CD_ISRC_INVALID: crate::core::HRESULT = crate::core::HRESULT(-1072885253i32);
pub const NS_E_CD_MEDIA_CATALOG_NUMBER_INVALID: crate::core::HRESULT =
    crate::core::HRESULT(-1072885252i32);
pub const NS_E_CD_NO_BUFFERS_READ: crate::core::HRESULT = crate::core::HRESULT(-1072885256i32);
pub const NS_E_CD_NO_READER: crate::core::HRESULT = crate::core::HRESULT(-1072885254i32);
pub const NS_E_CD_QUEUEING_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-1072885249i32);
pub const NS_E_CD_READ_ERROR: crate::core::HRESULT = crate::core::HRESULT(-1072885844i32);
pub const NS_E_CD_READ_ERROR_NO_CORRECTION: crate::core::HRESULT =
    crate::core::HRESULT(-1072885845i32);
pub const NS_E_CD_REFRESH: crate::core::HRESULT = crate::core::HRESULT(-1072885839i32);
pub const NS_E_CD_SLOW_COPY: crate::core::HRESULT = crate::core::HRESULT(-1072885843i32);
pub const NS_E_CD_SPEEDDETECT_NOT_ENOUGH_READS: crate::core::HRESULT =
    crate::core::HRESULT(-1072885250i32);
pub const NS_E_CHANGING_PROXYBYPASS: crate::core::HRESULT = crate::core::HRESULT(-1072885565i32);
pub const NS_E_CHANGING_PROXY_EXCEPTIONLIST: crate::core::HRESULT =
    crate::core::HRESULT(-1072885566i32);
pub const NS_E_CHANGING_PROXY_NAME: crate::core::HRESULT = crate::core::HRESULT(-1072885568i32);
pub const NS_E_CHANGING_PROXY_PORT: crate::core::HRESULT = crate::core::HRESULT(-1072885567i32);
pub const NS_E_CHANGING_PROXY_PROTOCOL_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-1072885564i32);
pub const NS_E_CLOSED_ON_SUSPEND: crate::core::HRESULT = crate::core::HRESULT(-1072877839i32);
pub const NS_E_CODEC_DMO_ERROR: crate::core::HRESULT = crate::core::HRESULT(-1072886822i32);
pub const NS_E_CODEC_UNAVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-1072882813i32);
pub const NS_E_COMPRESSED_DIGITAL_AUDIO_PROTECTION_LEVEL_UNSUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1072879352i32);
pub const NS_E_COMPRESSED_DIGITAL_VIDEO_PROTECTION_LEVEL_UNSUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1072879355i32);
pub const NS_E_CONNECTION_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-1072889815i32);
pub const NS_E_CONNECT_TIMEOUT: crate::core::HRESULT = crate::core::HRESULT(-1072877818i32);
pub const NS_E_CONTENT_PARTNER_STILL_INITIALIZING: crate::core::HRESULT =
    crate::core::HRESULT(-1072884894i32);
pub const NS_E_CORECD_NOTAMEDIACD: crate::core::HRESULT = crate::core::HRESULT(-1072885561i32);
pub const NS_E_CRITICAL_ERROR: crate::core::HRESULT = crate::core::HRESULT(-1072884452i32);
pub const NS_E_CUB_FAIL: crate::core::HRESULT = crate::core::HRESULT(-1072889773i32);
pub const NS_E_CUB_FAIL_LINK: crate::core::HRESULT = crate::core::HRESULT(-1072889456i32);
pub const NS_E_CURLHELPER_NOTADIRECTORY: crate::core::HRESULT =
    crate::core::HRESULT(-1072884947i32);
pub const NS_E_CURLHELPER_NOTAFILE: crate::core::HRESULT = crate::core::HRESULT(-1072884946i32);
pub const NS_E_CURLHELPER_NOTRELATIVE: crate::core::HRESULT = crate::core::HRESULT(-1072884944i32);
pub const NS_E_CURL_CANTDECODE: crate::core::HRESULT = crate::core::HRESULT(-1072884945i32);
pub const NS_E_CURL_CANTWALK: crate::core::HRESULT = crate::core::HRESULT(-1072884949i32);
pub const NS_E_CURL_INVALIDBUFFERSIZE: crate::core::HRESULT = crate::core::HRESULT(-1072884943i32);
pub const NS_E_CURL_INVALIDCHAR: crate::core::HRESULT = crate::core::HRESULT(-1072884955i32);
pub const NS_E_CURL_INVALIDHOSTNAME: crate::core::HRESULT = crate::core::HRESULT(-1072884954i32);
pub const NS_E_CURL_INVALIDPATH: crate::core::HRESULT = crate::core::HRESULT(-1072884953i32);
pub const NS_E_CURL_INVALIDPORT: crate::core::HRESULT = crate::core::HRESULT(-1072884948i32);
pub const NS_E_CURL_INVALIDSCHEME: crate::core::HRESULT = crate::core::HRESULT(-1072884952i32);
pub const NS_E_CURL_INVALIDURL: crate::core::HRESULT = crate::core::HRESULT(-1072884951i32);
pub const NS_E_CURL_NOTSAFE: crate::core::HRESULT = crate::core::HRESULT(-1072884956i32);
pub const NS_E_DAMAGED_FILE: crate::core::HRESULT = crate::core::HRESULT(-1072885813i32);
pub const NS_E_DATAPATH_NO_SINK: crate::core::HRESULT = crate::core::HRESULT(-1072884456i32);
pub const NS_E_DATA_SOURCE_ENUMERATION_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1072884352i32);
pub const NS_E_DATA_UNIT_EXTENSION_TOO_LARGE: crate::core::HRESULT =
    crate::core::HRESULT(-1072886823i32);
pub const NS_E_DDRAW_GENERIC: crate::core::HRESULT = crate::core::HRESULT(-1072885571i32);
pub const NS_E_DEVCONTROL_FAILED_SEEK: crate::core::HRESULT = crate::core::HRESULT(-1072882796i32);
pub const NS_E_DEVICECONTROL_UNSTABLE: crate::core::HRESULT = crate::core::HRESULT(-1072882719i32);
pub const NS_E_DEVICE_DISCONNECTED: crate::core::HRESULT = crate::core::HRESULT(-1072885854i32);
pub const NS_E_DEVICE_IS_NOT_READY: crate::core::HRESULT = crate::core::HRESULT(-1072885385i32);
pub const NS_E_DEVICE_NOT_READY: crate::core::HRESULT = crate::core::HRESULT(-1072885814i32);
pub const NS_E_DEVICE_NOT_SUPPORT_FORMAT: crate::core::HRESULT =
    crate::core::HRESULT(-1072885853i32);
pub const NS_E_DEVICE_NOT_WMDRM_DEVICE: crate::core::HRESULT = crate::core::HRESULT(-1072879749i32);
pub const NS_E_DISK_FAIL: crate::core::HRESULT = crate::core::HRESULT(-1072889771i32);
pub const NS_E_DISK_READ: crate::core::HRESULT = crate::core::HRESULT(-1072889833i32);
pub const NS_E_DISK_WRITE: crate::core::HRESULT = crate::core::HRESULT(-1072889834i32);
pub const NS_E_DISPLAY_MODE_CHANGE_FAILED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885570i32);
pub const NS_E_DRMPROFILE_NOTFOUND: crate::core::HRESULT = crate::core::HRESULT(-1072882731i32);
pub const NS_E_DRM_ACQUIRING_LICENSE: crate::core::HRESULT = crate::core::HRESULT(-1072879829i32);
pub const NS_E_DRM_ACTION_NOT_QUERIED: crate::core::HRESULT = crate::core::HRESULT(-1072879830i32);
pub const NS_E_DRM_ALREADY_INDIVIDUALIZED: crate::core::HRESULT =
    crate::core::HRESULT(-1072879831i32);
pub const NS_E_DRM_APPCERT_REVOKED: crate::core::HRESULT = crate::core::HRESULT(-1072879790i32);
pub const NS_E_DRM_ATTRIBUTE_TOO_LONG: crate::core::HRESULT = crate::core::HRESULT(-1072879438i32);
pub const NS_E_DRM_BACKUPRESTORE_BUSY: crate::core::HRESULT = crate::core::HRESULT(-1072879804i32);
pub const NS_E_DRM_BACKUP_CORRUPT: crate::core::HRESULT = crate::core::HRESULT(-1072879805i32);
pub const NS_E_DRM_BACKUP_EXISTS: crate::core::HRESULT = crate::core::HRESULT(-1072879806i32);
pub const NS_E_DRM_BAD_REQUEST: crate::core::HRESULT = crate::core::HRESULT(-1072879440i32);
pub const NS_E_DRM_BB_UNABLE_TO_INITIALIZE: crate::core::HRESULT =
    crate::core::HRESULT(-1072879744i32);
pub const NS_E_DRM_BUFFER_TOO_SMALL: crate::core::HRESULT = crate::core::HRESULT(-1072879780i32);
pub const NS_E_DRM_BUSY: crate::core::HRESULT = crate::core::HRESULT(-1072879551i32);
pub const NS_E_DRM_CACHED_CONTENT_ERROR: crate::core::HRESULT =
    crate::core::HRESULT(-1072879797i32);
pub const NS_E_DRM_CERTIFICATE_REVOKED: crate::core::HRESULT = crate::core::HRESULT(-1072879455i32);
pub const NS_E_DRM_CERTIFICATE_SECURITY_LEVEL_INADEQUATE: crate::core::HRESULT =
    crate::core::HRESULT(-1072879442i32);
pub const NS_E_DRM_CHAIN_TOO_LONG: crate::core::HRESULT = crate::core::HRESULT(-1072879540i32);
pub const NS_E_DRM_CHECKPOINT_CORRUPT: crate::core::HRESULT = crate::core::HRESULT(-1072879721i32);
pub const NS_E_DRM_CHECKPOINT_FAILED: crate::core::HRESULT = crate::core::HRESULT(-1072879745i32);
pub const NS_E_DRM_CHECKPOINT_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-1072879722i32);
pub const NS_E_DRM_CLIENT_CODE_EXPIRED: crate::core::HRESULT = crate::core::HRESULT(-1072879545i32);
pub const NS_E_DRM_DATASTORE_CORRUPT: crate::core::HRESULT = crate::core::HRESULT(-1072879741i32);
pub const NS_E_DRM_DEBUGGING_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-1072879769i32);
pub const NS_E_DRM_DECRYPT_ERROR: crate::core::HRESULT = crate::core::HRESULT(-1072879837i32);
pub const NS_E_DRM_DEVICE_ACTIVATION_CANCELED: crate::core::HRESULT =
    crate::core::HRESULT(-1072879771i32);
pub const NS_E_DRM_DEVICE_ALREADY_REGISTERED: crate::core::HRESULT =
    crate::core::HRESULT(-1072879445i32);
pub const NS_E_DRM_DEVICE_LIMIT_REACHED: crate::core::HRESULT =
    crate::core::HRESULT(-1072879453i32);
pub const NS_E_DRM_DEVICE_NOT_OPEN: crate::core::HRESULT = crate::core::HRESULT(-1072879446i32);
pub const NS_E_DRM_DEVICE_NOT_REGISTERED: crate::core::HRESULT =
    crate::core::HRESULT(-1072879646i32);
pub const NS_E_DRM_DRIVER_AUTH_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-1072879795i32);
pub const NS_E_DRM_DRIVER_DIGIOUT_FAILURE: crate::core::HRESULT =
    crate::core::HRESULT(-1072879792i32);
pub const NS_E_DRM_DRMV2CLT_REVOKED: crate::core::HRESULT = crate::core::HRESULT(-1072879434i32);
pub const NS_E_DRM_ENCRYPT_ERROR: crate::core::HRESULT = crate::core::HRESULT(-1072879838i32);
pub const NS_E_DRM_ENUM_LICENSE_FAILED: crate::core::HRESULT = crate::core::HRESULT(-1072879845i32);
pub const NS_E_DRM_ERROR_BAD_NET_RESP: crate::core::HRESULT = crate::core::HRESULT(-1072879778i32);
pub const NS_E_DRM_EXPIRED_LICENSEBLOB: crate::core::HRESULT = crate::core::HRESULT(-1072879437i32);
pub const NS_E_DRM_GET_CONTENTSTRING_ERROR: crate::core::HRESULT =
    crate::core::HRESULT(-1072879811i32);
pub const NS_E_DRM_GET_LICENSESTRING_ERROR: crate::core::HRESULT =
    crate::core::HRESULT(-1072879812i32);
pub const NS_E_DRM_GET_LICENSE_ERROR: crate::core::HRESULT = crate::core::HRESULT(-1072879815i32);
pub const NS_E_DRM_HARDWAREID_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-1072879729i32);
pub const NS_E_DRM_HARDWARE_INCONSISTENT: crate::core::HRESULT =
    crate::core::HRESULT(-1072879788i32);
pub const NS_E_DRM_INCLUSION_LIST_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-1072879435i32);
pub const NS_E_DRM_INDIVIDUALIZATION_INCOMPLETE: crate::core::HRESULT =
    crate::core::HRESULT(-1072879796i32);
pub const NS_E_DRM_INDIVIDUALIZE_ERROR: crate::core::HRESULT = crate::core::HRESULT(-1072879818i32);
pub const NS_E_DRM_INDIVIDUALIZING: crate::core::HRESULT = crate::core::HRESULT(-1072879828i32);
pub const NS_E_DRM_INDIV_FRAUD: crate::core::HRESULT = crate::core::HRESULT(-1072879549i32);
pub const NS_E_DRM_INDIV_NO_CABS: crate::core::HRESULT = crate::core::HRESULT(-1072879548i32);
pub const NS_E_DRM_INDIV_SERVICE_UNAVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-1072879547i32);
pub const NS_E_DRM_INVALID_APPCERT: crate::core::HRESULT = crate::core::HRESULT(-1072879748i32);
pub const NS_E_DRM_INVALID_APPDATA: crate::core::HRESULT = crate::core::HRESULT(-1072879808i32);
pub const NS_E_DRM_INVALID_APPDATA_VERSION: crate::core::HRESULT =
    crate::core::HRESULT(-1072879807i32);
pub const NS_E_DRM_INVALID_APPLICATION: crate::core::HRESULT = crate::core::HRESULT(-1072879855i32);
pub const NS_E_DRM_INVALID_CERTIFICATE: crate::core::HRESULT = crate::core::HRESULT(-1072879456i32);
pub const NS_E_DRM_INVALID_CONTENT: crate::core::HRESULT = crate::core::HRESULT(-1072879850i32);
pub const NS_E_DRM_INVALID_CRL: crate::core::HRESULT = crate::core::HRESULT(-1072879439i32);
pub const NS_E_DRM_INVALID_DATA: crate::core::HRESULT = crate::core::HRESULT(-1072879775i32);
pub const NS_E_DRM_INVALID_KID: crate::core::HRESULT = crate::core::HRESULT(-1072879543i32);
pub const NS_E_DRM_INVALID_LICENSE: crate::core::HRESULT = crate::core::HRESULT(-1072879848i32);
pub const NS_E_DRM_INVALID_LICENSEBLOB: crate::core::HRESULT = crate::core::HRESULT(-1072879436i32);
pub const NS_E_DRM_INVALID_LICENSE_ACQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-1072879841i32);
pub const NS_E_DRM_INVALID_LICENSE_REQUEST: crate::core::HRESULT =
    crate::core::HRESULT(-1072879844i32);
pub const NS_E_DRM_INVALID_MACHINE: crate::core::HRESULT = crate::core::HRESULT(-1072879847i32);
pub const NS_E_DRM_INVALID_MIGRATION_IMAGE: crate::core::HRESULT =
    crate::core::HRESULT(-1072879736i32);
pub const NS_E_DRM_INVALID_PROPERTY: crate::core::HRESULT = crate::core::HRESULT(-1072879799i32);
pub const NS_E_DRM_INVALID_PROXIMITY_RESPONSE: crate::core::HRESULT =
    crate::core::HRESULT(-1072879448i32);
pub const NS_E_DRM_INVALID_SECURESTORE_PASSWORD: crate::core::HRESULT =
    crate::core::HRESULT(-1072879791i32);
pub const NS_E_DRM_INVALID_SESSION: crate::core::HRESULT = crate::core::HRESULT(-1072879447i32);
pub const NS_E_DRM_KEY_ERROR: crate::core::HRESULT = crate::core::HRESULT(-1072879839i32);
pub const NS_E_DRM_LICENSE_APPSECLOW: crate::core::HRESULT = crate::core::HRESULT(-1072879654i32);
pub const NS_E_DRM_LICENSE_APP_NOTALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-1072879651i32);
pub const NS_E_DRM_LICENSE_CERT_EXPIRED: crate::core::HRESULT =
    crate::core::HRESULT(-1072879649i32);
pub const NS_E_DRM_LICENSE_CLOSE_ERROR: crate::core::HRESULT = crate::core::HRESULT(-1072879816i32);
pub const NS_E_DRM_LICENSE_CONTENT_REVOKED: crate::core::HRESULT =
    crate::core::HRESULT(-1072879647i32);
pub const NS_E_DRM_LICENSE_DELETION_ERROR: crate::core::HRESULT =
    crate::core::HRESULT(-1072879538i32);
pub const NS_E_DRM_LICENSE_EXPIRED: crate::core::HRESULT = crate::core::HRESULT(-1072879656i32);
pub const NS_E_DRM_LICENSE_INITIALIZATION_ERROR: crate::core::HRESULT =
    crate::core::HRESULT(-1072879542i32);
pub const NS_E_DRM_LICENSE_INVALID_XML: crate::core::HRESULT = crate::core::HRESULT(-1072879835i32);
pub const NS_E_DRM_LICENSE_NOSAP: crate::core::HRESULT = crate::core::HRESULT(-1072879606i32);
pub const NS_E_DRM_LICENSE_NOSVP: crate::core::HRESULT = crate::core::HRESULT(-1072879605i32);
pub const NS_E_DRM_LICENSE_NOTACQUIRED: crate::core::HRESULT = crate::core::HRESULT(-1072879783i32);
pub const NS_E_DRM_LICENSE_NOTENABLED: crate::core::HRESULT = crate::core::HRESULT(-1072879655i32);
pub const NS_E_DRM_LICENSE_NOTRUSTEDCODEC: crate::core::HRESULT =
    crate::core::HRESULT(-1072879603i32);
pub const NS_E_DRM_LICENSE_NOWDM: crate::core::HRESULT = crate::core::HRESULT(-1072879604i32);
pub const NS_E_DRM_LICENSE_OPEN_ERROR: crate::core::HRESULT = crate::core::HRESULT(-1072879817i32);
pub const NS_E_DRM_LICENSE_SECLOW: crate::core::HRESULT = crate::core::HRESULT(-1072879648i32);
pub const NS_E_DRM_LICENSE_SERVER_INFO_MISSING: crate::core::HRESULT =
    crate::core::HRESULT(-1072879552i32);
pub const NS_E_DRM_LICENSE_STORE_ERROR: crate::core::HRESULT = crate::core::HRESULT(-1072879854i32);
pub const NS_E_DRM_LICENSE_STORE_SAVE_ERROR: crate::core::HRESULT =
    crate::core::HRESULT(-1072879852i32);
pub const NS_E_DRM_LICENSE_UNAVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-1072879454i32);
pub const NS_E_DRM_LICENSE_UNUSABLE: crate::core::HRESULT = crate::core::HRESULT(-1072879800i32);
pub const NS_E_DRM_LIC_NEEDS_DEVICE_CLOCK_SET: crate::core::HRESULT =
    crate::core::HRESULT(-1072879751i32);
pub const NS_E_DRM_MALFORMED_CONTENT_HEADER: crate::core::HRESULT =
    crate::core::HRESULT(-1072879716i32);
pub const NS_E_DRM_MIGRATION_IMPORTER_NOT_AVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-1072879734i32);
pub const NS_E_DRM_MIGRATION_INVALID_LEGACYV2_DATA: crate::core::HRESULT =
    crate::core::HRESULT(-1072879727i32);
pub const NS_E_DRM_MIGRATION_INVALID_LEGACYV2_SST_PASSWORD: crate::core::HRESULT =
    crate::core::HRESULT(-1072879725i32);
pub const NS_E_DRM_MIGRATION_LICENSE_ALREADY_EXISTS: crate::core::HRESULT =
    crate::core::HRESULT(-1072879726i32);
pub const NS_E_DRM_MIGRATION_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1072879724i32);
pub const NS_E_DRM_MIGRATION_OBJECT_IN_USE: crate::core::HRESULT =
    crate::core::HRESULT(-1072879717i32);
pub const NS_E_DRM_MIGRATION_OPERATION_CANCELLED: crate::core::HRESULT =
    crate::core::HRESULT(-1072879718i32);
pub const NS_E_DRM_MIGRATION_TARGET_NOT_ONLINE: crate::core::HRESULT =
    crate::core::HRESULT(-1072879737i32);
pub const NS_E_DRM_MIGRATION_TARGET_STATES_CORRUPTED: crate::core::HRESULT =
    crate::core::HRESULT(-1072879735i32);
pub const NS_E_DRM_MONITOR_ERROR: crate::core::HRESULT = crate::core::HRESULT(-1072879810i32);
pub const NS_E_DRM_MUST_APPROVE: crate::core::HRESULT = crate::core::HRESULT(-1072879450i32);
pub const NS_E_DRM_MUST_REGISTER: crate::core::HRESULT = crate::core::HRESULT(-1072879451i32);
pub const NS_E_DRM_MUST_REVALIDATE: crate::core::HRESULT = crate::core::HRESULT(-1072879449i32);
pub const NS_E_DRM_NEEDS_INDIVIDUALIZATION: crate::core::HRESULT =
    crate::core::HRESULT(-1072879832i32);
pub const NS_E_DRM_NEEDS_UPGRADE_TEMPFILE: crate::core::HRESULT =
    crate::core::HRESULT(-1072879555i32);
pub const NS_E_DRM_NEED_UPGRADE_MSSAP: crate::core::HRESULT = crate::core::HRESULT(-1072879794i32);
pub const NS_E_DRM_NEED_UPGRADE_PD: crate::core::HRESULT = crate::core::HRESULT(-1072879554i32);
pub const NS_E_DRM_NOT_CONFIGURED: crate::core::HRESULT = crate::core::HRESULT(-1072879772i32);
pub const NS_E_DRM_NO_RIGHTS: crate::core::HRESULT = crate::core::HRESULT(-1072879840i32);
pub const NS_E_DRM_NO_UPLINK_LICENSE: crate::core::HRESULT = crate::core::HRESULT(-1072879544i32);
pub const NS_E_DRM_OPERATION_CANCELED: crate::core::HRESULT = crate::core::HRESULT(-1072879768i32);
pub const NS_E_DRM_PARAMETERS_MISMATCHED: crate::core::HRESULT =
    crate::core::HRESULT(-1072879825i32);
pub const NS_E_DRM_PASSWORD_TOO_LONG: crate::core::HRESULT = crate::core::HRESULT(-1072882797i32);
pub const NS_E_DRM_PD_TOO_MANY_DEVICES: crate::core::HRESULT = crate::core::HRESULT(-1072879550i32);
pub const NS_E_DRM_POLICY_DISABLE_ONLINE: crate::core::HRESULT =
    crate::core::HRESULT(-1072879774i32);
pub const NS_E_DRM_POLICY_METERING_DISABLED: crate::core::HRESULT =
    crate::core::HRESULT(-1072879754i32);
pub const NS_E_DRM_PROFILE_NOT_SET: crate::core::HRESULT = crate::core::HRESULT(-1072882801i32);
pub const NS_E_DRM_PROTOCOL_FORCEFUL_TERMINATION_ON_CHALLENGE: crate::core::HRESULT =
    crate::core::HRESULT(-1072879746i32);
pub const NS_E_DRM_PROTOCOL_FORCEFUL_TERMINATION_ON_PETITION: crate::core::HRESULT =
    crate::core::HRESULT(-1072879747i32);
pub const NS_E_DRM_QUERY_ERROR: crate::core::HRESULT = crate::core::HRESULT(-1072879814i32);
pub const NS_E_DRM_REOPEN_CONTENT: crate::core::HRESULT = crate::core::HRESULT(-1072879793i32);
pub const NS_E_DRM_REPORT_ERROR: crate::core::HRESULT = crate::core::HRESULT(-1072879813i32);
pub const NS_E_DRM_RESTORE_FRAUD: crate::core::HRESULT = crate::core::HRESULT(-1072879789i32);
pub const NS_E_DRM_RESTORE_SERVICE_UNAVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-1072879546i32);
pub const NS_E_DRM_RESTRICTIONS_NOT_RETRIEVED: crate::core::HRESULT =
    crate::core::HRESULT(-1072879767i32);
pub const NS_E_DRM_RIV_TOO_SMALL: crate::core::HRESULT = crate::core::HRESULT(-1072879433i32);
pub const NS_E_DRM_SDK_VERSIONMISMATCH: crate::core::HRESULT = crate::core::HRESULT(-1072879752i32);
pub const NS_E_DRM_SDMI_NOMORECOPIES: crate::core::HRESULT = crate::core::HRESULT(-1072879786i32);
pub const NS_E_DRM_SDMI_TRIGGER: crate::core::HRESULT = crate::core::HRESULT(-1072879787i32);
pub const NS_E_DRM_SECURE_STORE_ERROR: crate::core::HRESULT = crate::core::HRESULT(-1072879853i32);
pub const NS_E_DRM_SECURE_STORE_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-1072879798i32);
pub const NS_E_DRM_SECURE_STORE_UNLOCK_ERROR: crate::core::HRESULT =
    crate::core::HRESULT(-1072879851i32);
pub const NS_E_DRM_SECURITY_COMPONENT_SIGNATURE_INVALID: crate::core::HRESULT =
    crate::core::HRESULT(-1072879776i32);
pub const NS_E_DRM_SIGNATURE_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-1072879553i32);
pub const NS_E_DRM_SOURCEID_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1072879602i32);
pub const NS_E_DRM_STORE_NEEDINDI: crate::core::HRESULT = crate::core::HRESULT(-1072879653i32);
pub const NS_E_DRM_STORE_NOTALLOWED: crate::core::HRESULT = crate::core::HRESULT(-1072879652i32);
pub const NS_E_DRM_STORE_NOTALLSTORED: crate::core::HRESULT = crate::core::HRESULT(-1072879777i32);
pub const NS_E_DRM_STUBLIB_REQUIRED: crate::core::HRESULT = crate::core::HRESULT(-1072879739i32);
pub const NS_E_DRM_TRACK_EXCEEDED_PLAYLIST_RESTICTION: crate::core::HRESULT =
    crate::core::HRESULT(-1072879760i32);
pub const NS_E_DRM_TRACK_EXCEEDED_TRACKBURN_RESTRICTION: crate::core::HRESULT =
    crate::core::HRESULT(-1072879759i32);
pub const NS_E_DRM_TRANSFER_CHAINED_LICENSES_UNSUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1072879753i32);
pub const NS_E_DRM_UNABLE_TO_ACQUIRE_LICENSE: crate::core::HRESULT =
    crate::core::HRESULT(-1072879842i32);
pub const NS_E_DRM_UNABLE_TO_CREATE_AUTHENTICATION_OBJECT: crate::core::HRESULT =
    crate::core::HRESULT(-1072879773i32);
pub const NS_E_DRM_UNABLE_TO_CREATE_BACKUP_OBJECT: crate::core::HRESULT =
    crate::core::HRESULT(-1072879819i32);
pub const NS_E_DRM_UNABLE_TO_CREATE_CERTIFICATE_OBJECT: crate::core::HRESULT =
    crate::core::HRESULT(-1072879738i32);
pub const NS_E_DRM_UNABLE_TO_CREATE_CODING_OBJECT: crate::core::HRESULT =
    crate::core::HRESULT(-1072879782i32);
pub const NS_E_DRM_UNABLE_TO_CREATE_DECRYPT_OBJECT: crate::core::HRESULT =
    crate::core::HRESULT(-1072879821i32);
pub const NS_E_DRM_UNABLE_TO_CREATE_DEVICE_REGISTRATION_OBJECT: crate::core::HRESULT =
    crate::core::HRESULT(-1072879764i32);
pub const NS_E_DRM_UNABLE_TO_CREATE_ENCRYPT_OBJECT: crate::core::HRESULT =
    crate::core::HRESULT(-1072879822i32);
pub const NS_E_DRM_UNABLE_TO_CREATE_HEADER_OBJECT: crate::core::HRESULT =
    crate::core::HRESULT(-1072879785i32);
pub const NS_E_DRM_UNABLE_TO_CREATE_INDI_OBJECT: crate::core::HRESULT =
    crate::core::HRESULT(-1072879823i32);
pub const NS_E_DRM_UNABLE_TO_CREATE_INMEMORYSTORE_OBJECT: crate::core::HRESULT =
    crate::core::HRESULT(-1072879740i32);
pub const NS_E_DRM_UNABLE_TO_CREATE_KEYS_OBJECT: crate::core::HRESULT =
    crate::core::HRESULT(-1072879784i32);
pub const NS_E_DRM_UNABLE_TO_CREATE_LICENSE_OBJECT: crate::core::HRESULT =
    crate::core::HRESULT(-1072879824i32);
pub const NS_E_DRM_UNABLE_TO_CREATE_METERING_OBJECT: crate::core::HRESULT =
    crate::core::HRESULT(-1072879763i32);
pub const NS_E_DRM_UNABLE_TO_CREATE_MIGRATION_IMPORTER_OBJECT: crate::core::HRESULT =
    crate::core::HRESULT(-1072879723i32);
pub const NS_E_DRM_UNABLE_TO_CREATE_PLAYLIST_BURN_OBJECT: crate::core::HRESULT =
    crate::core::HRESULT(-1072879765i32);
pub const NS_E_DRM_UNABLE_TO_CREATE_PLAYLIST_OBJECT: crate::core::HRESULT =
    crate::core::HRESULT(-1072879766i32);
pub const NS_E_DRM_UNABLE_TO_CREATE_PROPERTIES_OBJECT: crate::core::HRESULT =
    crate::core::HRESULT(-1072879820i32);
pub const NS_E_DRM_UNABLE_TO_CREATE_STATE_DATA_OBJECT: crate::core::HRESULT =
    crate::core::HRESULT(-1072879781i32);
pub const NS_E_DRM_UNABLE_TO_GET_DEVICE_CERT: crate::core::HRESULT =
    crate::core::HRESULT(-1072879758i32);
pub const NS_E_DRM_UNABLE_TO_GET_SECURE_CLOCK: crate::core::HRESULT =
    crate::core::HRESULT(-1072879757i32);
pub const NS_E_DRM_UNABLE_TO_GET_SECURE_CLOCK_FROM_SERVER: crate::core::HRESULT =
    crate::core::HRESULT(-1072879755i32);
pub const NS_E_DRM_UNABLE_TO_INITIALIZE: crate::core::HRESULT =
    crate::core::HRESULT(-1072879843i32);
pub const NS_E_DRM_UNABLE_TO_LOAD_HARDWARE_ID: crate::core::HRESULT =
    crate::core::HRESULT(-1072879743i32);
pub const NS_E_DRM_UNABLE_TO_OPEN_DATA_STORE: crate::core::HRESULT =
    crate::core::HRESULT(-1072879742i32);
pub const NS_E_DRM_UNABLE_TO_OPEN_LICENSE: crate::core::HRESULT =
    crate::core::HRESULT(-1072879849i32);
pub const NS_E_DRM_UNABLE_TO_OPEN_PORT: crate::core::HRESULT = crate::core::HRESULT(-1072879441i32);
pub const NS_E_DRM_UNABLE_TO_SET_PARAMETER: crate::core::HRESULT =
    crate::core::HRESULT(-1072879809i32);
pub const NS_E_DRM_UNABLE_TO_SET_SECURE_CLOCK: crate::core::HRESULT =
    crate::core::HRESULT(-1072879756i32);
pub const NS_E_DRM_UNABLE_TO_VERIFY_PROXIMITY: crate::core::HRESULT =
    crate::core::HRESULT(-1072879452i32);
pub const NS_E_DRM_UNSUPPORTED_ACTION: crate::core::HRESULT = crate::core::HRESULT(-1072879443i32);
pub const NS_E_DRM_UNSUPPORTED_ALGORITHM: crate::core::HRESULT =
    crate::core::HRESULT(-1072879539i32);
pub const NS_E_DRM_UNSUPPORTED_PROPERTY: crate::core::HRESULT =
    crate::core::HRESULT(-1072879779i32);
pub const NS_E_DRM_UNSUPPORTED_PROTOCOL_VERSION: crate::core::HRESULT =
    crate::core::HRESULT(-1072879444i32);
pub const NS_E_DUPLICATE_ADDRESS: crate::core::HRESULT = crate::core::HRESULT(-1072889801i32);
pub const NS_E_DUPLICATE_DRMPROFILE: crate::core::HRESULT = crate::core::HRESULT(-1072882800i32);
pub const NS_E_DUPLICATE_NAME: crate::core::HRESULT = crate::core::HRESULT(-1072889802i32);
pub const NS_E_DUPLICATE_PACKET: crate::core::HRESULT = crate::core::HRESULT(-1072886829i32);
pub const NS_E_DVD_AUTHORING_PROBLEM: crate::core::HRESULT = crate::core::HRESULT(-1072885404i32);
pub const NS_E_DVD_CANNOT_COPY_PROTECTED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885390i32);
pub const NS_E_DVD_CANNOT_JUMP: crate::core::HRESULT = crate::core::HRESULT(-1072885393i32);
pub const NS_E_DVD_COMPATIBLE_VIDEO_CARD: crate::core::HRESULT =
    crate::core::HRESULT(-1072885402i32);
pub const NS_E_DVD_COPY_PROTECT: crate::core::HRESULT = crate::core::HRESULT(-1072885405i32);
pub const NS_E_DVD_DEVICE_CONTENTION: crate::core::HRESULT = crate::core::HRESULT(-1072885392i32);
pub const NS_E_DVD_DISC_COPY_PROTECT_OUTPUT_FAILED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885407i32);
pub const NS_E_DVD_DISC_COPY_PROTECT_OUTPUT_NS: crate::core::HRESULT =
    crate::core::HRESULT(-1072885408i32);
pub const NS_E_DVD_DISC_DECODER_REGION: crate::core::HRESULT = crate::core::HRESULT(-1072885399i32);
pub const NS_E_DVD_GRAPH_BUILDING: crate::core::HRESULT = crate::core::HRESULT(-1072885396i32);
pub const NS_E_DVD_INVALID_DISC_REGION: crate::core::HRESULT = crate::core::HRESULT(-1072885403i32);
pub const NS_E_DVD_INVALID_TITLE_CHAPTER: crate::core::HRESULT =
    crate::core::HRESULT(-1072885388i32);
pub const NS_E_DVD_MACROVISION: crate::core::HRESULT = crate::core::HRESULT(-1072885401i32);
pub const NS_E_DVD_NO_AUDIO_STREAM: crate::core::HRESULT = crate::core::HRESULT(-1072885397i32);
pub const NS_E_DVD_NO_DECODER: crate::core::HRESULT = crate::core::HRESULT(-1072885395i32);
pub const NS_E_DVD_NO_SUBPICTURE_STREAM: crate::core::HRESULT =
    crate::core::HRESULT(-1072885406i32);
pub const NS_E_DVD_NO_VIDEO_MEMORY: crate::core::HRESULT = crate::core::HRESULT(-1072885391i32);
pub const NS_E_DVD_NO_VIDEO_STREAM: crate::core::HRESULT = crate::core::HRESULT(-1072885398i32);
pub const NS_E_DVD_PARENTAL: crate::core::HRESULT = crate::core::HRESULT(-1072885394i32);
pub const NS_E_DVD_REQUIRED_PROPERTY_NOT_SET: crate::core::HRESULT =
    crate::core::HRESULT(-1072885389i32);
pub const NS_E_DVD_SYSTEM_DECODER_REGION: crate::core::HRESULT =
    crate::core::HRESULT(-1072885400i32);
pub const NS_E_EDL_REQUIRED_FOR_DEVICE_MULTIPASS: crate::core::HRESULT =
    crate::core::HRESULT(-1072882713i32);
pub const NS_E_EMPTY_PLAYLIST: crate::core::HRESULT = crate::core::HRESULT(-1072884555i32);
pub const NS_E_EMPTY_PROGRAM_NAME: crate::core::HRESULT = crate::core::HRESULT(-1072889642i32);
pub const NS_E_ENACTPLAN_GIVEUP: crate::core::HRESULT = crate::core::HRESULT(-1072889752i32);
pub const NS_E_END_OF_PLAYLIST: crate::core::HRESULT = crate::core::HRESULT(-1072876856i32);
pub const NS_E_END_OF_TAPE: crate::core::HRESULT = crate::core::HRESULT(-1072882770i32);
pub const NS_E_ERROR_FROM_PROXY: crate::core::HRESULT = crate::core::HRESULT(-1072877852i32);
pub const NS_E_EXCEED_MAX_DRM_PROFILE_LIMIT: crate::core::HRESULT =
    crate::core::HRESULT(-1072882720i32);
pub const NS_E_EXPECT_MONO_WAV_INPUT: crate::core::HRESULT = crate::core::HRESULT(-1072882783i32);
pub const NS_E_FAILED_DOWNLOAD_ABORT_BURN: crate::core::HRESULT =
    crate::core::HRESULT(-1072885540i32);
pub const NS_E_FAIL_LAUNCH_ROXIO_PLUGIN: crate::core::HRESULT =
    crate::core::HRESULT(-1072885376i32);
pub const NS_E_FEATURE_DISABLED_BY_GROUP_POLICY: crate::core::HRESULT =
    crate::core::HRESULT(-1072886820i32);
pub const NS_E_FEATURE_DISABLED_IN_SKU: crate::core::HRESULT = crate::core::HRESULT(-1072886819i32);
pub const NS_E_FEATURE_REQUIRES_ENTERPRISE_SERVER: crate::core::HRESULT =
    crate::core::HRESULT(-1072884349i32);
pub const NS_E_FILE_ALLOCATION_FAILED: crate::core::HRESULT = crate::core::HRESULT(-1072889826i32);
pub const NS_E_FILE_BANDWIDTH_LIMIT: crate::core::HRESULT = crate::core::HRESULT(-1072889808i32);
pub const NS_E_FILE_EXISTS: crate::core::HRESULT = crate::core::HRESULT(-1072889829i32);
pub const NS_E_FILE_FAILED_CHECKS: crate::core::HRESULT = crate::core::HRESULT(-1072885811i32);
pub const NS_E_FILE_INIT_FAILED: crate::core::HRESULT = crate::core::HRESULT(-1072889825i32);
pub const NS_E_FILE_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-1072889830i32);
pub const NS_E_FILE_OPEN_FAILED: crate::core::HRESULT = crate::core::HRESULT(-1072889827i32);
pub const NS_E_FILE_PLAY_FAILED: crate::core::HRESULT = crate::core::HRESULT(-1072889824i32);
pub const NS_E_FILE_READ: crate::core::HRESULT = crate::core::HRESULT(-1072889831i32);
pub const NS_E_FILE_WRITE: crate::core::HRESULT = crate::core::HRESULT(-1072889832i32);
pub const NS_E_FIREWALL: crate::core::HRESULT = crate::core::HRESULT(-1072877831i32);
pub const NS_E_FLASH_PLAYBACK_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885553i32);
pub const NS_E_GLITCH_MODE: crate::core::HRESULT = crate::core::HRESULT(-1072889451i32);
pub const NS_E_GRAPH_NOAUDIOLANGUAGE: crate::core::HRESULT = crate::core::HRESULT(-1072885563i32);
pub const NS_E_GRAPH_NOAUDIOLANGUAGESELECTED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885562i32);
pub const NS_E_HDS_KEY_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-1072879719i32);
pub const NS_E_HEADER_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-1072884449i32);
pub const NS_E_HTTP_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-1072889645i32);
pub const NS_E_HTTP_TEXT_DATACONTAINER_INVALID_SERVER_RESPONSE: crate::core::HRESULT =
    crate::core::HRESULT(-1072884340i32);
pub const NS_E_HTTP_TEXT_DATACONTAINER_SIZE_LIMIT_EXCEEDED: crate::core::HRESULT =
    crate::core::HRESULT(-1072884343i32);
pub const NS_E_ICMQUERYFORMAT: crate::core::HRESULT = crate::core::HRESULT(-1072882836i32);
pub const NS_E_IE_DISALLOWS_ACTIVEX_CONTROLS: crate::core::HRESULT =
    crate::core::HRESULT(-1072885554i32);
pub const NS_E_IMAGE_DOWNLOAD_FAILED: crate::core::HRESULT = crate::core::HRESULT(-1072885106i32);
pub const NS_E_IMAPI_LOSSOFSTREAMING: crate::core::HRESULT = crate::core::HRESULT(-1072885378i32);
pub const NS_E_IMAPI_MEDIUM_INVALIDTYPE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885374i32);
pub const NS_E_INCOMPATIBLE_FORMAT: crate::core::HRESULT = crate::core::HRESULT(-1072889791i32);
pub const NS_E_INCOMPATIBLE_PUSH_SERVER: crate::core::HRESULT =
    crate::core::HRESULT(-1072877812i32);
pub const NS_E_INCOMPATIBLE_SERVER: crate::core::HRESULT = crate::core::HRESULT(-1072877848i32);
pub const NS_E_INCOMPATIBLE_VERSION: crate::core::HRESULT = crate::core::HRESULT(-1072886841i32);
pub const NS_E_INCOMPLETE_PLAYLIST: crate::core::HRESULT = crate::core::HRESULT(-1072885182i32);
pub const NS_E_INCORRECTCLIPSETTINGS: crate::core::HRESULT = crate::core::HRESULT(-1072882820i32);
pub const NS_E_INDUCED: crate::core::HRESULT = crate::core::HRESULT(-1072889822i32);
pub const NS_E_INPUTSOURCE_PROBLEM: crate::core::HRESULT = crate::core::HRESULT(-1072882806i32);
pub const NS_E_INPUT_DOESNOT_SUPPORT_SMPTE: crate::core::HRESULT =
    crate::core::HRESULT(-1072882776i32);
pub const NS_E_INPUT_WAVFORMAT_MISMATCH: crate::core::HRESULT =
    crate::core::HRESULT(-1072882782i32);
pub const NS_E_INSUFFICIENT_BANDWIDTH: crate::core::HRESULT = crate::core::HRESULT(-1072889812i32);
pub const NS_E_INSUFFICIENT_DATA: crate::core::HRESULT = crate::core::HRESULT(-1072889654i32);
pub const NS_E_INTERFACE_NOT_REGISTERED_IN_GIT: crate::core::HRESULT =
    crate::core::HRESULT(-1072885142i32);
pub const NS_E_INTERLACEMODE_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-1072882773i32);
pub const NS_E_INTERLACE_REQUIRE_SAMESIZE: crate::core::HRESULT =
    crate::core::HRESULT(-1072882795i32);
pub const NS_E_INTERNAL: crate::core::HRESULT = crate::core::HRESULT(-1072889820i32);
pub const NS_E_INTERNAL_SERVER_ERROR: crate::core::HRESULT = crate::core::HRESULT(-1072877854i32);
pub const NS_E_INVALIDCALL_WHILE_ARCHIVAL_RUNNING: crate::core::HRESULT =
    crate::core::HRESULT(-1072882828i32);
pub const NS_E_INVALIDCALL_WHILE_ENCODER_RUNNING: crate::core::HRESULT =
    crate::core::HRESULT(-1072882842i32);
pub const NS_E_INVALIDCALL_WHILE_ENCODER_STOPPED: crate::core::HRESULT =
    crate::core::HRESULT(-1072882817i32);
pub const NS_E_INVALIDINPUTFPS: crate::core::HRESULT = crate::core::HRESULT(-1072882815i32);
pub const NS_E_INVALIDPACKETSIZE: crate::core::HRESULT = crate::core::HRESULT(-1072882827i32);
pub const NS_E_INVALIDPROFILE: crate::core::HRESULT = crate::core::HRESULT(-1072886842i32);
pub const NS_E_INVALID_ARCHIVE: crate::core::HRESULT = crate::core::HRESULT(-1072889795i32);
pub const NS_E_INVALID_AUDIO_BUFFERMAX: crate::core::HRESULT = crate::core::HRESULT(-1072882756i32);
pub const NS_E_INVALID_AUDIO_PEAKRATE: crate::core::HRESULT = crate::core::HRESULT(-1072882758i32);
pub const NS_E_INVALID_AUDIO_PEAKRATE_2: crate::core::HRESULT =
    crate::core::HRESULT(-1072882757i32);
pub const NS_E_INVALID_BLACKHOLE_ADDRESS: crate::core::HRESULT =
    crate::core::HRESULT(-1072889792i32);
pub const NS_E_INVALID_CHANNEL: crate::core::HRESULT = crate::core::HRESULT(-1072889797i32);
pub const NS_E_INVALID_CLIENT: crate::core::HRESULT = crate::core::HRESULT(-1072889793i32);
pub const NS_E_INVALID_DATA: crate::core::HRESULT = crate::core::HRESULT(-1072889809i32);
pub const NS_E_INVALID_DEVICE: crate::core::HRESULT = crate::core::HRESULT(-1072882799i32);
pub const NS_E_INVALID_DRMV2CLT_STUBLIB: crate::core::HRESULT =
    crate::core::HRESULT(-1072879728i32);
pub const NS_E_INVALID_EDL: crate::core::HRESULT = crate::core::HRESULT(-1072886824i32);
pub const NS_E_INVALID_FILE_BITRATE: crate::core::HRESULT = crate::core::HRESULT(-1072882735i32);
pub const NS_E_INVALID_FOLDDOWN_COEFFICIENTS: crate::core::HRESULT =
    crate::core::HRESULT(-1072882732i32);
pub const NS_E_INVALID_INDEX: crate::core::HRESULT = crate::core::HRESULT(-1072889839i32);
pub const NS_E_INVALID_INDEX2: crate::core::HRESULT = crate::core::HRESULT(-1072889639i32);
pub const NS_E_INVALID_INPUT_AUDIENCE_INDEX: crate::core::HRESULT =
    crate::core::HRESULT(-1072882786i32);
pub const NS_E_INVALID_INPUT_FORMAT: crate::core::HRESULT = crate::core::HRESULT(-1072886856i32);
pub const NS_E_INVALID_INPUT_LANGUAGE: crate::core::HRESULT = crate::core::HRESULT(-1072882785i32);
pub const NS_E_INVALID_INPUT_STREAM: crate::core::HRESULT = crate::core::HRESULT(-1072882784i32);
pub const NS_E_INVALID_INTERLACEMODE: crate::core::HRESULT = crate::core::HRESULT(-1072882725i32);
pub const NS_E_INVALID_INTERLACE_COMPAT: crate::core::HRESULT =
    crate::core::HRESULT(-1072882724i32);
pub const NS_E_INVALID_KEY: crate::core::HRESULT = crate::core::HRESULT(-1072889790i32);
pub const NS_E_INVALID_LOG_URL: crate::core::HRESULT = crate::core::HRESULT(-1072884347i32);
pub const NS_E_INVALID_MTU_RANGE: crate::core::HRESULT = crate::core::HRESULT(-1072884346i32);
pub const NS_E_INVALID_NAME: crate::core::HRESULT = crate::core::HRESULT(-1072889828i32);
pub const NS_E_INVALID_NONSQUAREPIXEL_COMPAT: crate::core::HRESULT =
    crate::core::HRESULT(-1072882723i32);
pub const NS_E_INVALID_NUM_PASSES: crate::core::HRESULT = crate::core::HRESULT(-1072886827i32);
pub const NS_E_INVALID_OPERATING_SYSTEM_VERSION: crate::core::HRESULT =
    crate::core::HRESULT(-1072884647i32);
pub const NS_E_INVALID_OUTPUT_FORMAT: crate::core::HRESULT = crate::core::HRESULT(-1072886853i32);
pub const NS_E_INVALID_PIXEL_ASPECT_RATIO: crate::core::HRESULT =
    crate::core::HRESULT(-1072882718i32);
pub const NS_E_INVALID_PLAY_STATISTICS: crate::core::HRESULT = crate::core::HRESULT(-1072884345i32);
pub const NS_E_INVALID_PLUGIN_LOAD_TYPE_CONFIGURATION: crate::core::HRESULT =
    crate::core::HRESULT(-1072884652i32);
pub const NS_E_INVALID_PORT: crate::core::HRESULT = crate::core::HRESULT(-1072889789i32);
pub const NS_E_INVALID_PROFILE_CONTENTTYPE: crate::core::HRESULT =
    crate::core::HRESULT(-1072882716i32);
pub const NS_E_INVALID_PUBLISHING_POINT_NAME: crate::core::HRESULT =
    crate::core::HRESULT(-1072884651i32);
pub const NS_E_INVALID_PUSH_PUBLISHING_POINT: crate::core::HRESULT =
    crate::core::HRESULT(-1072884453i32);
pub const NS_E_INVALID_PUSH_PUBLISHING_POINT_START_REQUEST: crate::core::HRESULT =
    crate::core::HRESULT(-1072884645i32);
pub const NS_E_INVALID_PUSH_TEMPLATE: crate::core::HRESULT = crate::core::HRESULT(-1072884454i32);
pub const NS_E_INVALID_QUERY_OPERATOR: crate::core::HRESULT = crate::core::HRESULT(-1072876849i32);
pub const NS_E_INVALID_QUERY_PROPERTY: crate::core::HRESULT = crate::core::HRESULT(-1072876848i32);
pub const NS_E_INVALID_REDIRECT: crate::core::HRESULT = crate::core::HRESULT(-1072877846i32);
pub const NS_E_INVALID_REQUEST: crate::core::HRESULT = crate::core::HRESULT(-1072889813i32);
pub const NS_E_INVALID_SAMPLING_RATE: crate::core::HRESULT = crate::core::HRESULT(-1072886832i32);
pub const NS_E_INVALID_SCRIPT_BITRATE: crate::core::HRESULT = crate::core::HRESULT(-1072882737i32);
pub const NS_E_INVALID_SOURCE_WITH_DEVICE_CONTROL: crate::core::HRESULT =
    crate::core::HRESULT(-1072882722i32);
pub const NS_E_INVALID_STREAM: crate::core::HRESULT = crate::core::HRESULT(-1072889796i32);
pub const NS_E_INVALID_TIMECODE: crate::core::HRESULT = crate::core::HRESULT(-1072882730i32);
pub const NS_E_INVALID_TTL: crate::core::HRESULT = crate::core::HRESULT(-1072889788i32);
pub const NS_E_INVALID_VBR_COMPAT: crate::core::HRESULT = crate::core::HRESULT(-1072882766i32);
pub const NS_E_INVALID_VBR_WITH_UNCOMP: crate::core::HRESULT = crate::core::HRESULT(-1072882764i32);
pub const NS_E_INVALID_VIDEO_BITRATE: crate::core::HRESULT = crate::core::HRESULT(-1072882753i32);
pub const NS_E_INVALID_VIDEO_BUFFER: crate::core::HRESULT = crate::core::HRESULT(-1072882743i32);
pub const NS_E_INVALID_VIDEO_BUFFERMAX: crate::core::HRESULT = crate::core::HRESULT(-1072882742i32);
pub const NS_E_INVALID_VIDEO_BUFFERMAX_2: crate::core::HRESULT =
    crate::core::HRESULT(-1072882741i32);
pub const NS_E_INVALID_VIDEO_CQUALITY: crate::core::HRESULT = crate::core::HRESULT(-1072882744i32);
pub const NS_E_INVALID_VIDEO_FPS: crate::core::HRESULT = crate::core::HRESULT(-1072882747i32);
pub const NS_E_INVALID_VIDEO_HEIGHT: crate::core::HRESULT = crate::core::HRESULT(-1072882748i32);
pub const NS_E_INVALID_VIDEO_HEIGHT_ALIGN: crate::core::HRESULT =
    crate::core::HRESULT(-1072882739i32);
pub const NS_E_INVALID_VIDEO_IQUALITY: crate::core::HRESULT = crate::core::HRESULT(-1072882745i32);
pub const NS_E_INVALID_VIDEO_KEYFRAME: crate::core::HRESULT = crate::core::HRESULT(-1072882746i32);
pub const NS_E_INVALID_VIDEO_PEAKRATE: crate::core::HRESULT = crate::core::HRESULT(-1072882751i32);
pub const NS_E_INVALID_VIDEO_PEAKRATE_2: crate::core::HRESULT =
    crate::core::HRESULT(-1072882750i32);
pub const NS_E_INVALID_VIDEO_WIDTH: crate::core::HRESULT = crate::core::HRESULT(-1072882749i32);
pub const NS_E_INVALID_VIDEO_WIDTH_ALIGN: crate::core::HRESULT =
    crate::core::HRESULT(-1072882740i32);
pub const NS_E_INVALID_VIDEO_WIDTH_FOR_INTERLACED_ENCODING: crate::core::HRESULT =
    crate::core::HRESULT(-1072882712i32);
pub const NS_E_LANGUAGE_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-1072882788i32);
pub const NS_E_LATE_OPERATION: crate::core::HRESULT = crate::core::HRESULT(-1072889810i32);
pub const NS_E_LATE_PACKET: crate::core::HRESULT = crate::core::HRESULT(-1072886830i32);
pub const NS_E_LICENSE_EXPIRED: crate::core::HRESULT = crate::core::HRESULT(-1072889644i32);
pub const NS_E_LICENSE_HEADER_MISSING_URL: crate::core::HRESULT =
    crate::core::HRESULT(-1072879750i32);
pub const NS_E_LICENSE_INCORRECT_RIGHTS: crate::core::HRESULT =
    crate::core::HRESULT(-1072886847i32);
pub const NS_E_LICENSE_OUTOFDATE: crate::core::HRESULT = crate::core::HRESULT(-1072886848i32);
pub const NS_E_LICENSE_REQUIRED: crate::core::HRESULT = crate::core::HRESULT(-1072886850i32);
pub const NS_E_LOGFILEPERIOD: crate::core::HRESULT = crate::core::HRESULT(-1072889784i32);
pub const NS_E_LOG_FILE_SIZE: crate::core::HRESULT = crate::core::HRESULT(-1072889782i32);
pub const NS_E_LOG_NEED_TO_BE_SKIPPED: crate::core::HRESULT = crate::core::HRESULT(-1072884344i32);
pub const NS_E_MARKIN_UNSUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-1072882711i32);
pub const NS_E_MAX_BITRATE: crate::core::HRESULT = crate::core::HRESULT(-1072889785i32);
pub const NS_E_MAX_CLIENTS: crate::core::HRESULT = crate::core::HRESULT(-1072889783i32);
pub const NS_E_MAX_FILERATE: crate::core::HRESULT = crate::core::HRESULT(-1072889781i32);
pub const NS_E_MAX_FUNNELS_ALERT: crate::core::HRESULT = crate::core::HRESULT(-1072889760i32);
pub const NS_E_MAX_PACKET_SIZE_TOO_SMALL: crate::core::HRESULT =
    crate::core::HRESULT(-1072886831i32);
pub const NS_E_MEDIACD_READ_ERROR: crate::core::HRESULT = crate::core::HRESULT(-1072885555i32);
pub const NS_E_MEDIA_LIBRARY_FAILED: crate::core::HRESULT = crate::core::HRESULT(-1072885810i32);
pub const NS_E_MEDIA_PARSER_INVALID_FORMAT: crate::core::HRESULT =
    crate::core::HRESULT(-1072884351i32);
pub const NS_E_MEMSTORAGE_BAD_DATA: crate::core::HRESULT = crate::core::HRESULT(-1072885381i32);
pub const NS_E_METADATA_CACHE_DATA_NOT_AVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-1072876837i32);
pub const NS_E_METADATA_CANNOT_RETRIEVE_FROM_OFFLINE_CACHE: crate::core::HRESULT =
    crate::core::HRESULT(-1072876834i32);
pub const NS_E_METADATA_CANNOT_SET_LOCALE: crate::core::HRESULT =
    crate::core::HRESULT(-1072876841i32);
pub const NS_E_METADATA_FORMAT_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1072876843i32);
pub const NS_E_METADATA_IDENTIFIER_NOT_AVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-1072876835i32);
pub const NS_E_METADATA_INVALID_DOCUMENT_TYPE: crate::core::HRESULT =
    crate::core::HRESULT(-1072876836i32);
pub const NS_E_METADATA_LANGUAGE_NOT_SUPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1072876840i32);
pub const NS_E_METADATA_NOT_AVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-1072876838i32);
pub const NS_E_METADATA_NO_EDITING_CAPABILITY: crate::core::HRESULT =
    crate::core::HRESULT(-1072876842i32);
pub const NS_E_METADATA_NO_RFC1766_NAME_FOR_LOCALE: crate::core::HRESULT =
    crate::core::HRESULT(-1072876839i32);
pub const NS_E_MISMATCHED_MEDIACONTENT: crate::core::HRESULT = crate::core::HRESULT(-1072882849i32);
pub const NS_E_MISSING_AUDIENCE: crate::core::HRESULT = crate::core::HRESULT(-1072882792i32);
pub const NS_E_MISSING_CHANNEL: crate::core::HRESULT = crate::core::HRESULT(-1072889641i32);
pub const NS_E_MISSING_SOURCE_INDEX: crate::core::HRESULT = crate::core::HRESULT(-1072882790i32);
pub const NS_E_MIXER_INVALID_CONTROL: crate::core::HRESULT = crate::core::HRESULT(-1072885850i32);
pub const NS_E_MIXER_INVALID_LINE: crate::core::HRESULT = crate::core::HRESULT(-1072885851i32);
pub const NS_E_MIXER_INVALID_VALUE: crate::core::HRESULT = crate::core::HRESULT(-1072885849i32);
pub const NS_E_MIXER_NODRIVER: crate::core::HRESULT = crate::core::HRESULT(-1072885841i32);
pub const NS_E_MIXER_UNKNOWN_MMRESULT: crate::core::HRESULT = crate::core::HRESULT(-1072885848i32);
pub const NS_E_MLS_SMARTPLAYLIST_FILTER_NOT_REGISTERED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885643i32);
pub const NS_E_MMSAUTOSERVER_CANTFINDWALKER: crate::core::HRESULT =
    crate::core::HRESULT(-1072889786i32);
pub const NS_E_MMS_NOT_SUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-1072877830i32);
pub const NS_E_MONITOR_GIVEUP: crate::core::HRESULT = crate::core::HRESULT(-1072889656i32);
pub const NS_E_MP3_FORMAT_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-1072885846i32);
pub const NS_E_MPDB_GENERIC: crate::core::HRESULT = crate::core::HRESULT(-1072885812i32);
pub const NS_E_MSAUDIO_NOT_INSTALLED: crate::core::HRESULT = crate::core::HRESULT(-1072886855i32);
pub const NS_E_MSBD_NO_LONGER_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1072877844i32);
pub const NS_E_MULTICAST_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-1072877847i32);
pub const NS_E_MULTICAST_PLUGIN_NOT_ENABLED: crate::core::HRESULT =
    crate::core::HRESULT(-1072884648i32);
pub const NS_E_MULTIPLE_AUDIO_CODECS: crate::core::HRESULT = crate::core::HRESULT(-1072882761i32);
pub const NS_E_MULTIPLE_AUDIO_FORMATS: crate::core::HRESULT = crate::core::HRESULT(-1072882760i32);
pub const NS_E_MULTIPLE_FILE_BITRATES: crate::core::HRESULT = crate::core::HRESULT(-1072882736i32);
pub const NS_E_MULTIPLE_SCRIPT_BITRATES: crate::core::HRESULT =
    crate::core::HRESULT(-1072882738i32);
pub const NS_E_MULTIPLE_VBR_AUDIENCES: crate::core::HRESULT = crate::core::HRESULT(-1072882763i32);
pub const NS_E_MULTIPLE_VIDEO_CODECS: crate::core::HRESULT = crate::core::HRESULT(-1072882755i32);
pub const NS_E_MULTIPLE_VIDEO_SIZES: crate::core::HRESULT = crate::core::HRESULT(-1072882754i32);
pub const NS_E_NAMESPACE_BAD_NAME: crate::core::HRESULT = crate::core::HRESULT(-1072884842i32);
pub const NS_E_NAMESPACE_BUFFER_TOO_SMALL: crate::core::HRESULT =
    crate::core::HRESULT(-1072884850i32);
pub const NS_E_NAMESPACE_CALLBACK_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-1072884847i32);
pub const NS_E_NAMESPACE_DUPLICATE_CALLBACK: crate::core::HRESULT =
    crate::core::HRESULT(-1072884848i32);
pub const NS_E_NAMESPACE_DUPLICATE_NAME: crate::core::HRESULT =
    crate::core::HRESULT(-1072884845i32);
pub const NS_E_NAMESPACE_EMPTY_NAME: crate::core::HRESULT = crate::core::HRESULT(-1072884844i32);
pub const NS_E_NAMESPACE_INDEX_TOO_LARGE: crate::core::HRESULT =
    crate::core::HRESULT(-1072884843i32);
pub const NS_E_NAMESPACE_NAME_TOO_LONG: crate::core::HRESULT = crate::core::HRESULT(-1072884846i32);
pub const NS_E_NAMESPACE_NODE_CONFLICT: crate::core::HRESULT = crate::core::HRESULT(-1072884852i32);
pub const NS_E_NAMESPACE_NODE_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-1072884851i32);
pub const NS_E_NAMESPACE_TOO_MANY_CALLBACKS: crate::core::HRESULT =
    crate::core::HRESULT(-1072884849i32);
pub const NS_E_NAMESPACE_WRONG_PERSIST: crate::core::HRESULT = crate::core::HRESULT(-1072884854i32);
pub const NS_E_NAMESPACE_WRONG_SECURITY: crate::core::HRESULT =
    crate::core::HRESULT(-1072884841i32);
pub const NS_E_NAMESPACE_WRONG_TYPE: crate::core::HRESULT = crate::core::HRESULT(-1072884853i32);
pub const NS_E_NEED_CORE_REFERENCE: crate::core::HRESULT = crate::core::HRESULT(-1072885556i32);
pub const NS_E_NEED_TO_ASK_USER: crate::core::HRESULT = crate::core::HRESULT(-1072885798i32);
pub const NS_E_NETWORK_BUSY: crate::core::HRESULT = crate::core::HRESULT(-1072889842i32);
pub const NS_E_NETWORK_RESOURCE_FAILURE: crate::core::HRESULT =
    crate::core::HRESULT(-1072889816i32);
pub const NS_E_NETWORK_SERVICE_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-1072889817i32);
pub const NS_E_NETWORK_SINK_WRITE: crate::core::HRESULT = crate::core::HRESULT(-1072877832i32);
pub const NS_E_NET_READ: crate::core::HRESULT = crate::core::HRESULT(-1072889835i32);
pub const NS_E_NET_WRITE: crate::core::HRESULT = crate::core::HRESULT(-1072889836i32);
pub const NS_E_NOCONNECTION: crate::core::HRESULT = crate::core::HRESULT(-1072889851i32);
pub const NS_E_NOFUNNEL: crate::core::HRESULT = crate::core::HRESULT(-1072889844i32);
pub const NS_E_NOMATCHING_ELEMENT: crate::core::HRESULT = crate::core::HRESULT(-1072882850i32);
pub const NS_E_NOMATCHING_MEDIASOURCE: crate::core::HRESULT = crate::core::HRESULT(-1072882854i32);
pub const NS_E_NONSQUAREPIXELMODE_MISMATCH: crate::core::HRESULT =
    crate::core::HRESULT(-1072882772i32);
pub const NS_E_NOREGISTEREDWALKER: crate::core::HRESULT = crate::core::HRESULT(-1072889845i32);
pub const NS_E_NOSOURCEGROUPS: crate::core::HRESULT = crate::core::HRESULT(-1072882816i32);
pub const NS_E_NOSTATSAVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-1072882819i32);
pub const NS_E_NOTARCHIVING: crate::core::HRESULT = crate::core::HRESULT(-1072882818i32);
pub const NS_E_NOTHING_TO_DO: crate::core::HRESULT = crate::core::HRESULT(-1072887823i32);
pub const NS_E_NOTITLES: crate::core::HRESULT = crate::core::HRESULT(-1072889794i32);
pub const NS_E_NOT_CONFIGURED: crate::core::HRESULT = crate::core::HRESULT(-1072886852i32);
pub const NS_E_NOT_CONNECTED: crate::core::HRESULT = crate::core::HRESULT(-1072886837i32);
pub const NS_E_NOT_CONTENT_PARTNER_TRACK: crate::core::HRESULT =
    crate::core::HRESULT(-1072884902i32);
pub const NS_E_NOT_LICENSED: crate::core::HRESULT = crate::core::HRESULT(-1072889651i32);
pub const NS_E_NOT_REBUILDING: crate::core::HRESULT = crate::core::HRESULT(-1072889811i32);
pub const NS_E_NO_ACTIVE_SOURCEGROUP: crate::core::HRESULT = crate::core::HRESULT(-1072882830i32);
pub const NS_E_NO_AUDIENCES: crate::core::HRESULT = crate::core::HRESULT(-1072882768i32);
pub const NS_E_NO_AUDIODATA: crate::core::HRESULT = crate::core::HRESULT(-1072882807i32);
pub const NS_E_NO_AUDIO_COMPAT: crate::core::HRESULT = crate::core::HRESULT(-1072882767i32);
pub const NS_E_NO_AUDIO_TIMECOMPRESSION: crate::core::HRESULT =
    crate::core::HRESULT(-1072882729i32);
pub const NS_E_NO_CD: crate::core::HRESULT = crate::core::HRESULT(-1072885856i32);
pub const NS_E_NO_CD_BURNER: crate::core::HRESULT = crate::core::HRESULT(-1072885386i32);
pub const NS_E_NO_CHANNELS: crate::core::HRESULT = crate::core::HRESULT(-1072889640i32);
pub const NS_E_NO_DATAVIEW_SUPPORT: crate::core::HRESULT = crate::core::HRESULT(-1072882814i32);
pub const NS_E_NO_DEVICE: crate::core::HRESULT = crate::core::HRESULT(-1072889743i32);
pub const NS_E_NO_ERROR_STRING_FOUND: crate::core::HRESULT = crate::core::HRESULT(-1072885808i32);
pub const NS_E_NO_EXISTING_PACKETIZER: crate::core::HRESULT = crate::core::HRESULT(-1072877827i32);
pub const NS_E_NO_FORMATS: crate::core::HRESULT = crate::core::HRESULT(-1072889749i32);
pub const NS_E_NO_FRAMES_SUBMITTED_TO_ANALYZER: crate::core::HRESULT =
    crate::core::HRESULT(-1072882777i32);
pub const NS_E_NO_LOCALPLAY: crate::core::HRESULT = crate::core::HRESULT(-1072889843i32);
pub const NS_E_NO_MBR_WITH_TIMECODE: crate::core::HRESULT = crate::core::HRESULT(-1072882726i32);
pub const NS_E_NO_MEDIAFORMAT_IN_SOURCE: crate::core::HRESULT =
    crate::core::HRESULT(-1072882833i32);
pub const NS_E_NO_MEDIA_IN_AUDIENCE: crate::core::HRESULT = crate::core::HRESULT(-1072882769i32);
pub const NS_E_NO_MEDIA_PROTOCOL: crate::core::HRESULT = crate::core::HRESULT(-1072889445i32);
pub const NS_E_NO_MORE_SAMPLES: crate::core::HRESULT = crate::core::HRESULT(-1072886833i32);
pub const NS_E_NO_MULTICAST: crate::core::HRESULT = crate::core::HRESULT(-1072887822i32);
pub const NS_E_NO_MULTIPASS_FOR_LIVEDEVICE: crate::core::HRESULT =
    crate::core::HRESULT(-1072882793i32);
pub const NS_E_NO_NEW_CONNECTIONS: crate::core::HRESULT = crate::core::HRESULT(-1072884451i32);
pub const NS_E_NO_PAL_INVERSE_TELECINE: crate::core::HRESULT = crate::core::HRESULT(-1072882780i32);
pub const NS_E_NO_PDA: crate::core::HRESULT = crate::core::HRESULT(-1072885383i32);
pub const NS_E_NO_PROFILE_IN_SOURCEGROUP: crate::core::HRESULT =
    crate::core::HRESULT(-1072882841i32);
pub const NS_E_NO_PROFILE_NAME: crate::core::HRESULT = crate::core::HRESULT(-1072882765i32);
pub const NS_E_NO_REALTIME_PREPROCESS: crate::core::HRESULT = crate::core::HRESULT(-1072882804i32);
pub const NS_E_NO_REALTIME_TIMECOMPRESSION: crate::core::HRESULT =
    crate::core::HRESULT(-1072882810i32);
pub const NS_E_NO_REFERENCES: crate::core::HRESULT = crate::core::HRESULT(-1072889748i32);
pub const NS_E_NO_REPEAT_PREPROCESS: crate::core::HRESULT = crate::core::HRESULT(-1072882803i32);
pub const NS_E_NO_SCRIPT_ENGINE: crate::core::HRESULT = crate::core::HRESULT(-1072884356i32);
pub const NS_E_NO_SCRIPT_STREAM: crate::core::HRESULT = crate::core::HRESULT(-1072882829i32);
pub const NS_E_NO_SERVER_CONTACT: crate::core::HRESULT = crate::core::HRESULT(-1072889650i32);
pub const NS_E_NO_SMPTE_WITH_MULTIPLE_SOURCEGROUPS: crate::core::HRESULT =
    crate::core::HRESULT(-1072882775i32);
pub const NS_E_NO_SPECIFIED_DEVICE: crate::core::HRESULT = crate::core::HRESULT(-1072889742i32);
pub const NS_E_NO_STREAM: crate::core::HRESULT = crate::core::HRESULT(-1072889805i32);
pub const NS_E_NO_TWOPASS_TIMECOMPRESSION: crate::core::HRESULT =
    crate::core::HRESULT(-1072882728i32);
pub const NS_E_NO_VALID_OUTPUT_STREAM: crate::core::HRESULT = crate::core::HRESULT(-1072882832i32);
pub const NS_E_NO_VALID_SOURCE_PLUGIN: crate::core::HRESULT = crate::core::HRESULT(-1072882831i32);
pub const NS_E_NUM_LANGUAGE_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-1072882789i32);
pub const NS_E_OFFLINE_MODE: crate::core::HRESULT = crate::core::HRESULT(-1072886838i32);
pub const NS_E_OPEN_CONTAINING_FOLDER_FAILED: crate::core::HRESULT =
    crate::core::HRESULT(-1072884893i32);
pub const NS_E_OPEN_FILE_LIMIT: crate::core::HRESULT = crate::core::HRESULT(-1072889807i32);
pub const NS_E_OUTPUT_PROTECTION_LEVEL_UNSUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1072879356i32);
pub const NS_E_OUTPUT_PROTECTION_SCHEME_UNSUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1072879350i32);
pub const NS_E_PACKETSINK_UNKNOWN_FEC_STREAM: crate::core::HRESULT =
    crate::core::HRESULT(-1072877814i32);
pub const NS_E_PAGING_ERROR: crate::core::HRESULT = crate::core::HRESULT(-1072889758i32);
pub const NS_E_PARTIALLY_REBUILT_DISK: crate::core::HRESULT = crate::core::HRESULT(-1072889753i32);
pub const NS_E_PDA_CANNOT_CREATE_ADDITIONAL_SYNC_RELATIONSHIP: crate::core::HRESULT =
    crate::core::HRESULT(-1072885371i32);
pub const NS_E_PDA_CANNOT_SYNC_FROM_INTERNET: crate::core::HRESULT =
    crate::core::HRESULT(-1072885196i32);
pub const NS_E_PDA_CANNOT_SYNC_FROM_LOCATION: crate::core::HRESULT =
    crate::core::HRESULT(-1072885357i32);
pub const NS_E_PDA_CANNOT_SYNC_INVALID_PLAYLIST: crate::core::HRESULT =
    crate::core::HRESULT(-1072885195i32);
pub const NS_E_PDA_CANNOT_TRANSCODE: crate::core::HRESULT = crate::core::HRESULT(-1072885367i32);
pub const NS_E_PDA_CANNOT_TRANSCODE_TO_AUDIO: crate::core::HRESULT =
    crate::core::HRESULT(-1072885187i32);
pub const NS_E_PDA_CANNOT_TRANSCODE_TO_IMAGE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885185i32);
pub const NS_E_PDA_CANNOT_TRANSCODE_TO_VIDEO: crate::core::HRESULT =
    crate::core::HRESULT(-1072885186i32);
pub const NS_E_PDA_CEWMDM_DRM_ERROR: crate::core::HRESULT = crate::core::HRESULT(-1072885183i32);
pub const NS_E_PDA_DELETE_FAILED: crate::core::HRESULT = crate::core::HRESULT(-1072885192i32);
pub const NS_E_PDA_DEVICESUPPORTDISABLED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885360i32);
pub const NS_E_PDA_DEVICE_FULL: crate::core::HRESULT = crate::core::HRESULT(-1072885377i32);
pub const NS_E_PDA_DEVICE_FULL_IN_SESSION: crate::core::HRESULT =
    crate::core::HRESULT(-1072885375i32);
pub const NS_E_PDA_DEVICE_NOT_RESPONDING: crate::core::HRESULT =
    crate::core::HRESULT(-1072885190i32);
pub const NS_E_PDA_ENCODER_NOT_RESPONDING: crate::core::HRESULT =
    crate::core::HRESULT(-1072885358i32);
pub const NS_E_PDA_FAILED_TO_BURN: crate::core::HRESULT = crate::core::HRESULT(-1072885542i32);
pub const NS_E_PDA_FAILED_TO_ENCRYPT_TRANSCODED_FILE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885188i32);
pub const NS_E_PDA_FAILED_TO_RETRIEVE_FILE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885191i32);
pub const NS_E_PDA_FAILED_TO_SYNCHRONIZE_FILE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885194i32);
pub const NS_E_PDA_FAILED_TO_TRANSCODE_PHOTO: crate::core::HRESULT =
    crate::core::HRESULT(-1072885189i32);
pub const NS_E_PDA_FAIL_READ_WAVE_FILE: crate::core::HRESULT = crate::core::HRESULT(-1072885379i32);
pub const NS_E_PDA_FAIL_SELECT_DEVICE: crate::core::HRESULT = crate::core::HRESULT(-1072885380i32);
pub const NS_E_PDA_INITIALIZINGDEVICES: crate::core::HRESULT = crate::core::HRESULT(-1072885363i32);
pub const NS_E_PDA_MANUALDEVICE: crate::core::HRESULT = crate::core::HRESULT(-1072885373i32);
pub const NS_E_PDA_NO_LONGER_AVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-1072885359i32);
pub const NS_E_PDA_NO_TRANSCODE_OF_DRM: crate::core::HRESULT = crate::core::HRESULT(-1072885370i32);
pub const NS_E_PDA_OBSOLETE_SP: crate::core::HRESULT = crate::core::HRESULT(-1072885362i32);
pub const NS_E_PDA_PARTNERSHIPNOTEXIST: crate::core::HRESULT = crate::core::HRESULT(-1072885372i32);
pub const NS_E_PDA_RETRIEVED_FILE_FILENAME_TOO_LONG: crate::core::HRESULT =
    crate::core::HRESULT(-1072885184i32);
pub const NS_E_PDA_SYNC_FAILED: crate::core::HRESULT = crate::core::HRESULT(-1072885193i32);
pub const NS_E_PDA_SYNC_LOGIN_ERROR: crate::core::HRESULT = crate::core::HRESULT(-1072885180i32);
pub const NS_E_PDA_SYNC_RUNNING: crate::core::HRESULT = crate::core::HRESULT(-1072885181i32);
pub const NS_E_PDA_TITLE_COLLISION: crate::core::HRESULT = crate::core::HRESULT(-1072885361i32);
pub const NS_E_PDA_TOO_MANY_FILES_IN_DIRECTORY: crate::core::HRESULT =
    crate::core::HRESULT(-1072885366i32);
pub const NS_E_PDA_TOO_MANY_FILE_COLLISIONS: crate::core::HRESULT =
    crate::core::HRESULT(-1072885368i32);
pub const NS_E_PDA_TRANSCODECACHEFULL: crate::core::HRESULT = crate::core::HRESULT(-1072885369i32);
pub const NS_E_PDA_TRANSCODE_CODEC_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-1072885179i32);
pub const NS_E_PDA_TRANSCODE_NOT_PERMITTED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885364i32);
pub const NS_E_PDA_UNSPECIFIED_ERROR: crate::core::HRESULT = crate::core::HRESULT(-1072885382i32);
pub const NS_E_PDA_UNSUPPORTED_FORMAT: crate::core::HRESULT = crate::core::HRESULT(-1072885384i32);
pub const NS_E_PLAYLIST_CONTAINS_ERRORS: crate::core::HRESULT =
    crate::core::HRESULT(-1072885569i32);
pub const NS_E_PLAYLIST_END_RECEDING: crate::core::HRESULT = crate::core::HRESULT(-1072884547i32);
pub const NS_E_PLAYLIST_ENTRY_ALREADY_PLAYING: crate::core::HRESULT =
    crate::core::HRESULT(-1072884556i32);
pub const NS_E_PLAYLIST_ENTRY_HAS_CHANGED: crate::core::HRESULT =
    crate::core::HRESULT(-1072877835i32);
pub const NS_E_PLAYLIST_ENTRY_NOT_IN_PLAYLIST: crate::core::HRESULT =
    crate::core::HRESULT(-1072884552i32);
pub const NS_E_PLAYLIST_ENTRY_SEEK: crate::core::HRESULT = crate::core::HRESULT(-1072884551i32);
pub const NS_E_PLAYLIST_PARSE_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-1072884554i32);
pub const NS_E_PLAYLIST_PLUGIN_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-1072884353i32);
pub const NS_E_PLAYLIST_RECURSIVE_PLAYLISTS: crate::core::HRESULT =
    crate::core::HRESULT(-1072884550i32);
pub const NS_E_PLAYLIST_SHUTDOWN: crate::core::HRESULT = crate::core::HRESULT(-1072884548i32);
pub const NS_E_PLAYLIST_TOO_MANY_NESTED_PLAYLISTS: crate::core::HRESULT =
    crate::core::HRESULT(-1072884549i32);
pub const NS_E_PLAYLIST_UNSUPPORTED_ENTRY: crate::core::HRESULT =
    crate::core::HRESULT(-1072884553i32);
pub const NS_E_PLUGIN_CLSID_INVALID: crate::core::HRESULT = crate::core::HRESULT(-1072882826i32);
pub const NS_E_PLUGIN_ERROR_REPORTED: crate::core::HRESULT = crate::core::HRESULT(-1072884355i32);
pub const NS_E_PLUGIN_NOTSHUTDOWN: crate::core::HRESULT = crate::core::HRESULT(-1072885802i32);
pub const NS_E_PORT_IN_USE: crate::core::HRESULT = crate::core::HRESULT(-1072884342i32);
pub const NS_E_PORT_IN_USE_HTTP: crate::core::HRESULT = crate::core::HRESULT(-1072884341i32);
pub const NS_E_PROCESSINGSHOWSYNCWIZARD: crate::core::HRESULT =
    crate::core::HRESULT(-1072885365i32);
pub const NS_E_PROFILE_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-1072882821i32);
pub const NS_E_PROPERTY_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-1072876854i32);
pub const NS_E_PROPERTY_NOT_SUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-1072876846i32);
pub const NS_E_PROPERTY_READ_ONLY: crate::core::HRESULT = crate::core::HRESULT(-1072876852i32);
pub const NS_E_PROTECTED_CONTENT: crate::core::HRESULT = crate::core::HRESULT(-1072886851i32);
pub const NS_E_PROTOCOL_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-1072889838i32);
pub const NS_E_PROXY_ACCESSDENIED: crate::core::HRESULT = crate::core::HRESULT(-1072877834i32);
pub const NS_E_PROXY_CONNECT_TIMEOUT: crate::core::HRESULT = crate::core::HRESULT(-1072877817i32);
pub const NS_E_PROXY_DNS_TIMEOUT: crate::core::HRESULT = crate::core::HRESULT(-1072877840i32);
pub const NS_E_PROXY_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-1072877843i32);
pub const NS_E_PROXY_SOURCE_ACCESSDENIED: crate::core::HRESULT =
    crate::core::HRESULT(-1072877833i32);
pub const NS_E_PROXY_TIMEOUT: crate::core::HRESULT = crate::core::HRESULT(-1072877851i32);
pub const NS_E_PUBLISHING_POINT_INVALID_REQUEST_WHILE_STARTED: crate::core::HRESULT =
    crate::core::HRESULT(-1072884649i32);
pub const NS_E_PUBLISHING_POINT_REMOVED: crate::core::HRESULT =
    crate::core::HRESULT(-1072884646i32);
pub const NS_E_PUBLISHING_POINT_STOPPED: crate::core::HRESULT =
    crate::core::HRESULT(-1072884642i32);
pub const NS_E_PUSH_CANNOTCONNECT: crate::core::HRESULT = crate::core::HRESULT(-1072877813i32);
pub const NS_E_PUSH_DUPLICATE_PUBLISHING_POINT_NAME: crate::core::HRESULT =
    crate::core::HRESULT(-1072884448i32);
pub const NS_E_REBOOT_RECOMMENDED: crate::core::HRESULT = crate::core::HRESULT(-1072878854i32);
pub const NS_E_REBOOT_REQUIRED: crate::core::HRESULT = crate::core::HRESULT(-1072878853i32);
pub const NS_E_RECORDQ_DISK_FULL: crate::core::HRESULT = crate::core::HRESULT(-1072882781i32);
pub const NS_E_REDBOOK_ENABLED_WHILE_COPYING: crate::core::HRESULT =
    crate::core::HRESULT(-1072885840i32);
pub const NS_E_REDIRECT: crate::core::HRESULT = crate::core::HRESULT(-1072884856i32);
pub const NS_E_REDIRECT_TO_PROXY: crate::core::HRESULT = crate::core::HRESULT(-1072877855i32);
pub const NS_E_REFUSED_BY_SERVER: crate::core::HRESULT = crate::core::HRESULT(-1072877849i32);
pub const NS_E_REG_FLUSH_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-1072879720i32);
pub const NS_E_REMIRRORED_DISK: crate::core::HRESULT = crate::core::HRESULT(-1072889655i32);
pub const NS_E_REQUIRE_STREAMING_CLIENT: crate::core::HRESULT =
    crate::core::HRESULT(-1072877836i32);
pub const NS_E_RESET_SOCKET_CONNECTION: crate::core::HRESULT = crate::core::HRESULT(-1072877824i32);
pub const NS_E_RESOURCE_GONE: crate::core::HRESULT = crate::core::HRESULT(-1072877828i32);
pub const NS_E_SAME_AS_INPUT_COMBINATION: crate::core::HRESULT =
    crate::core::HRESULT(-1072882734i32);
pub const NS_E_SCHEMA_CLASSIFY_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-1072876844i32);
pub const NS_E_SCRIPT_DEBUGGER_NOT_INSTALLED: crate::core::HRESULT =
    crate::core::HRESULT(-1072884350i32);
pub const NS_E_SDK_BUFFERTOOSMALL: crate::core::HRESULT = crate::core::HRESULT(-1072886828i32);
pub const NS_E_SERVER_ACCESSDENIED: crate::core::HRESULT = crate::core::HRESULT(-1072877829i32);
pub const NS_E_SERVER_DNS_TIMEOUT: crate::core::HRESULT = crate::core::HRESULT(-1072877841i32);
pub const NS_E_SERVER_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-1072889803i32);
pub const NS_E_SERVER_UNAVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-1072877850i32);
pub const NS_E_SESSION_INVALID: crate::core::HRESULT = crate::core::HRESULT(-1072877816i32);
pub const NS_E_SESSION_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-1072877837i32);
pub const NS_E_SETUP_BLOCKED: crate::core::HRESULT = crate::core::HRESULT(-1072878848i32);
pub const NS_E_SETUP_DRM_MIGRATION_FAILED: crate::core::HRESULT =
    crate::core::HRESULT(-1072878851i32);
pub const NS_E_SETUP_DRM_MIGRATION_FAILED_AND_IGNORABLE_FAILURE: crate::core::HRESULT =
    crate::core::HRESULT(-1072878849i32);
pub const NS_E_SETUP_IGNORABLE_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-1072878850i32);
pub const NS_E_SETUP_INCOMPLETE: crate::core::HRESULT = crate::core::HRESULT(-1072878852i32);
pub const NS_E_SET_DISK_UID_FAILED: crate::core::HRESULT = crate::core::HRESULT(-1072889823i32);
pub const NS_E_SHARING_STATE_OUT_OF_SYNC: crate::core::HRESULT =
    crate::core::HRESULT(-1072885772i32);
pub const NS_E_SHARING_VIOLATION: crate::core::HRESULT = crate::core::HRESULT(-1072885809i32);
pub const NS_E_SHUTDOWN: crate::core::HRESULT = crate::core::HRESULT(-1072889814i32);
pub const NS_E_SLOW_READ_DIGITAL: crate::core::HRESULT = crate::core::HRESULT(-1072885852i32);
pub const NS_E_SLOW_READ_DIGITAL_WITH_ERRORCORRECTION: crate::core::HRESULT =
    crate::core::HRESULT(-1072885251i32);
pub const NS_E_SMPTEMODE_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-1072882771i32);
pub const NS_E_SOURCEGROUP_NOTPREPARED: crate::core::HRESULT = crate::core::HRESULT(-1072882822i32);
pub const NS_E_SOURCE_CANNOT_LOOP: crate::core::HRESULT = crate::core::HRESULT(-1072882733i32);
pub const NS_E_SOURCE_NOTSPECIFIED: crate::core::HRESULT = crate::core::HRESULT(-1072882811i32);
pub const NS_E_SOURCE_PLUGIN_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-1072884354i32);
pub const NS_E_SPEECHEDL_ON_NON_MIXEDMODE: crate::core::HRESULT =
    crate::core::HRESULT(-1072882798i32);
pub const NS_E_STALE_PRESENTATION: crate::core::HRESULT = crate::core::HRESULT(-1072884855i32);
pub const NS_E_STREAM_END: crate::core::HRESULT = crate::core::HRESULT(-1072889804i32);
pub const NS_E_STRIDE_REFUSED: crate::core::HRESULT = crate::core::HRESULT(-1072889787i32);
pub const NS_E_SUBSCRIPTIONSERVICE_DOWNLOAD_TIMEOUT: crate::core::HRESULT =
    crate::core::HRESULT(-1072884896i32);
pub const NS_E_SUBSCRIPTIONSERVICE_LOGIN_FAILED: crate::core::HRESULT =
    crate::core::HRESULT(-1072884897i32);
pub const NS_E_SUBSCRIPTIONSERVICE_PLAYBACK_DISALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-1072884906i32);
pub const NS_E_SYNCWIZ_CANNOT_CHANGE_SETTINGS: crate::core::HRESULT =
    crate::core::HRESULT(-1072885265i32);
pub const NS_E_SYNCWIZ_DEVICE_FULL: crate::core::HRESULT = crate::core::HRESULT(-1072885266i32);
pub const NS_E_TABLE_KEY_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-1072876851i32);
pub const NS_E_TAMPERED_CONTENT: crate::core::HRESULT = crate::core::HRESULT(-1072886849i32);
pub const NS_E_TCP_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-1072889646i32);
pub const NS_E_TIGER_FAIL: crate::core::HRESULT = crate::core::HRESULT(-1072889776i32);
pub const NS_E_TIMECODE_REQUIRES_VIDEOSTREAM: crate::core::HRESULT =
    crate::core::HRESULT(-1072882727i32);
pub const NS_E_TIMEOUT: crate::core::HRESULT = crate::core::HRESULT(-1072889837i32);
pub const NS_E_TITLE_BITRATE: crate::core::HRESULT = crate::core::HRESULT(-1072889643i32);
pub const NS_E_TITLE_SIZE_EXCEEDED: crate::core::HRESULT = crate::core::HRESULT(-1072889648i32);
pub const NS_E_TOO_MANY_AUDIO: crate::core::HRESULT = crate::core::HRESULT(-1072882852i32);
pub const NS_E_TOO_MANY_DEVICECONTROL: crate::core::HRESULT = crate::core::HRESULT(-1072882794i32);
pub const NS_E_TOO_MANY_HOPS: crate::core::HRESULT = crate::core::HRESULT(-1072877822i32);
pub const NS_E_TOO_MANY_MULTICAST_SINKS: crate::core::HRESULT =
    crate::core::HRESULT(-1072884650i32);
pub const NS_E_TOO_MANY_SESS: crate::core::HRESULT = crate::core::HRESULT(-1072889841i32);
pub const NS_E_TOO_MANY_TITLES: crate::core::HRESULT = crate::core::HRESULT(-1072889649i32);
pub const NS_E_TOO_MANY_VIDEO: crate::core::HRESULT = crate::core::HRESULT(-1072882851i32);
pub const NS_E_TOO_MUCH_DATA: crate::core::HRESULT = crate::core::HRESULT(-1072886836i32);
pub const NS_E_TOO_MUCH_DATA_FROM_SERVER: crate::core::HRESULT =
    crate::core::HRESULT(-1072877819i32);
pub const NS_E_TRACK_DOWNLOAD_REQUIRES_ALBUM_PURCHASE: crate::core::HRESULT =
    crate::core::HRESULT(-1072884901i32);
pub const NS_E_TRACK_DOWNLOAD_REQUIRES_PURCHASE: crate::core::HRESULT =
    crate::core::HRESULT(-1072884900i32);
pub const NS_E_TRACK_PURCHASE_MAXIMUM_EXCEEDED: crate::core::HRESULT =
    crate::core::HRESULT(-1072884899i32);
pub const NS_E_TRANSCODE_DELETECACHEERROR: crate::core::HRESULT =
    crate::core::HRESULT(-1072885264i32);
pub const NS_E_TRANSFORM_PLUGIN_INVALID: crate::core::HRESULT =
    crate::core::HRESULT(-1072882714i32);
pub const NS_E_TRANSFORM_PLUGIN_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-1072882715i32);
pub const NS_E_UDP_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-1072889647i32);
pub const NS_E_UNABLE_TO_CREATE_RIP_LOCATION: crate::core::HRESULT =
    crate::core::HRESULT(-1072885552i32);
pub const NS_E_UNCOMPRESSED_DIGITAL_AUDIO_PROTECTION_LEVEL_UNSUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1072879351i32);
pub const NS_E_UNCOMPRESSED_DIGITAL_VIDEO_PROTECTION_LEVEL_UNSUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1072879354i32);
pub const NS_E_UNCOMP_COMP_COMBINATION: crate::core::HRESULT = crate::core::HRESULT(-1072882762i32);
pub const NS_E_UNEXPECTED_DISPLAY_SETTINGS: crate::core::HRESULT =
    crate::core::HRESULT(-1072882808i32);
pub const NS_E_UNEXPECTED_MSAUDIO_ERROR: crate::core::HRESULT =
    crate::core::HRESULT(-1072886854i32);
pub const NS_E_UNKNOWN_PROTOCOL: crate::core::HRESULT = crate::core::HRESULT(-1072877856i32);
pub const NS_E_UNRECOGNIZED_STREAM_TYPE: crate::core::HRESULT =
    crate::core::HRESULT(-1072889818i32);
pub const NS_E_UNSUPPORTED_ARCHIVEOPERATION: crate::core::HRESULT =
    crate::core::HRESULT(-1072882824i32);
pub const NS_E_UNSUPPORTED_ARCHIVETYPE: crate::core::HRESULT = crate::core::HRESULT(-1072882825i32);
pub const NS_E_UNSUPPORTED_ENCODER_DEVICE: crate::core::HRESULT =
    crate::core::HRESULT(-1072882809i32);
pub const NS_E_UNSUPPORTED_LANGUAGE: crate::core::HRESULT = crate::core::HRESULT(-1072884644i32);
pub const NS_E_UNSUPPORTED_LOAD_TYPE: crate::core::HRESULT = crate::core::HRESULT(-1072884653i32);
pub const NS_E_UNSUPPORTED_PROPERTY: crate::core::HRESULT = crate::core::HRESULT(-1072886835i32);
pub const NS_E_UNSUPPORTED_SOURCETYPE: crate::core::HRESULT = crate::core::HRESULT(-1072882853i32);
pub const NS_E_URLLIST_INVALIDFORMAT: crate::core::HRESULT = crate::core::HRESULT(-1072885651i32);
pub const NS_E_USER_STOP: crate::core::HRESULT = crate::core::HRESULT(-1072885847i32);
pub const NS_E_USE_FILE_SOURCE: crate::core::HRESULT = crate::core::HRESULT(-1072876855i32);
pub const NS_E_VBRMODE_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-1072882787i32);
pub const NS_E_VIDCAPCREATEWINDOW: crate::core::HRESULT = crate::core::HRESULT(-1072882835i32);
pub const NS_E_VIDCAPDRVINUSE: crate::core::HRESULT = crate::core::HRESULT(-1072882834i32);
pub const NS_E_VIDCAPSTARTFAILED: crate::core::HRESULT = crate::core::HRESULT(-1072882839i32);
pub const NS_E_VIDEODEVICE_BUSY: crate::core::HRESULT = crate::core::HRESULT(-1072882844i32);
pub const NS_E_VIDEODEVICE_UNEXPECTED: crate::core::HRESULT = crate::core::HRESULT(-1072882843i32);
pub const NS_E_VIDEODRIVER_UNSTABLE: crate::core::HRESULT = crate::core::HRESULT(-1072882840i32);
pub const NS_E_VIDEO_BITRATE_STEPDOWN: crate::core::HRESULT = crate::core::HRESULT(-1072882752i32);
pub const NS_E_VIDEO_CODEC_ERROR: crate::core::HRESULT = crate::core::HRESULT(-1072886843i32);
pub const NS_E_VIDEO_CODEC_NOT_INSTALLED: crate::core::HRESULT =
    crate::core::HRESULT(-1072886844i32);
pub const NS_E_VIDSOURCECOMPRESSION: crate::core::HRESULT = crate::core::HRESULT(-1072882838i32);
pub const NS_E_VIDSOURCESIZE: crate::core::HRESULT = crate::core::HRESULT(-1072882837i32);
pub const NS_E_WALKER_SERVER: crate::core::HRESULT = crate::core::HRESULT(-1072889779i32);
pub const NS_E_WALKER_UNKNOWN: crate::core::HRESULT = crate::core::HRESULT(-1072889780i32);
pub const NS_E_WALKER_USAGE: crate::core::HRESULT = crate::core::HRESULT(-1072889778i32);
pub const NS_E_WAVE_OPEN: crate::core::HRESULT = crate::core::HRESULT(-1072889747i32);
pub const NS_E_WINSOCK_ERROR_STRING: crate::core::HRESULT = crate::core::HRESULT(-1072885463i32);
pub const NS_E_WIZARD_RUNNING: crate::core::HRESULT = crate::core::HRESULT(-1072884348i32);
pub const NS_E_WMDM_REVOKED: crate::core::HRESULT = crate::core::HRESULT(-1072885572i32);
pub const NS_E_WMDRM_DEPRECATED: crate::core::HRESULT = crate::core::HRESULT(-1072886818i32);
pub const NS_E_WME_VERSION_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-1072882805i32);
pub const NS_E_WMG_CANNOTQUEUE: crate::core::HRESULT = crate::core::HRESULT(-1072885684i32);
pub const NS_E_WMG_COPP_SECURITY_INVALID: crate::core::HRESULT =
    crate::core::HRESULT(-1072885678i32);
pub const NS_E_WMG_COPP_UNSUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-1072885677i32);
pub const NS_E_WMG_FILETRANSFERNOTALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885672i32);
pub const NS_E_WMG_INVALIDSTATE: crate::core::HRESULT = crate::core::HRESULT(-1072885676i32);
pub const NS_E_WMG_INVALID_COPP_CERTIFICATE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885679i32);
pub const NS_E_WMG_LICENSE_TAMPERED: crate::core::HRESULT = crate::core::HRESULT(-1072885660i32);
pub const NS_E_WMG_NOSDKINTERFACE: crate::core::HRESULT = crate::core::HRESULT(-1072885674i32);
pub const NS_E_WMG_NOTALLOUTPUTSRENDERED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885673i32);
pub const NS_E_WMG_PLUGINUNAVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-1072885685i32);
pub const NS_E_WMG_PREROLLLICENSEACQUISITIONNOTALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885683i32);
pub const NS_E_WMG_RATEUNAVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-1072885686i32);
pub const NS_E_WMG_SINKALREADYEXISTS: crate::core::HRESULT = crate::core::HRESULT(-1072885675i32);
pub const NS_E_WMG_UNEXPECTEDPREROLLSTATUS: crate::core::HRESULT =
    crate::core::HRESULT(-1072885682i32);
pub const NS_E_WMPBR_BACKUPCANCEL: crate::core::HRESULT = crate::core::HRESULT(-1072885455i32);
pub const NS_E_WMPBR_BACKUPRESTOREFAILED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885448i32);
pub const NS_E_WMPBR_DRIVE_INVALID: crate::core::HRESULT = crate::core::HRESULT(-1072885449i32);
pub const NS_E_WMPBR_ERRORWITHURL: crate::core::HRESULT = crate::core::HRESULT(-1072885453i32);
pub const NS_E_WMPBR_NAMECOLLISION: crate::core::HRESULT = crate::core::HRESULT(-1072885452i32);
pub const NS_E_WMPBR_NOLISTENER: crate::core::HRESULT = crate::core::HRESULT(-1072885456i32);
pub const NS_E_WMPBR_RESTORECANCEL: crate::core::HRESULT = crate::core::HRESULT(-1072885454i32);
pub const NS_E_WMPCORE_BUFFERTOOSMALL: crate::core::HRESULT = crate::core::HRESULT(-1072885633i32);
pub const NS_E_WMPCORE_BUSY: crate::core::HRESULT = crate::core::HRESULT(-1072885577i32);
pub const NS_E_WMPCORE_COCREATEFAILEDFORGITOBJECT: crate::core::HRESULT =
    crate::core::HRESULT(-1072885635i32);
pub const NS_E_WMPCORE_CODEC_DOWNLOAD_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885604i32);
pub const NS_E_WMPCORE_CODEC_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-1072885605i32);
pub const NS_E_WMPCORE_CODEC_NOT_TRUSTED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885606i32);
pub const NS_E_WMPCORE_CURRENT_MEDIA_NOT_ACTIVE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885591i32);
pub const NS_E_WMPCORE_DEVICE_DRIVERS_MISSING: crate::core::HRESULT =
    crate::core::HRESULT(-1072885539i32);
pub const NS_E_WMPCORE_ERRORMANAGERNOTAVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885619i32);
pub const NS_E_WMPCORE_ERRORSINKNOTREGISTERED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885620i32);
pub const NS_E_WMPCORE_ERROR_DOWNLOADING_PLAYLIST: crate::core::HRESULT =
    crate::core::HRESULT(-1072885603i32);
pub const NS_E_WMPCORE_FAILEDTOGETMARSHALLEDEVENTHANDLERINTERFACE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885634i32);
pub const NS_E_WMPCORE_FAILED_TO_BUILD_PLAYLIST: crate::core::HRESULT =
    crate::core::HRESULT(-1072885602i32);
pub const NS_E_WMPCORE_FILE_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-1072885574i32);
pub const NS_E_WMPCORE_GRAPH_NOT_IN_LIST: crate::core::HRESULT =
    crate::core::HRESULT(-1072885622i32);
pub const NS_E_WMPCORE_INVALIDPLAYLISTMODE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885631i32);
pub const NS_E_WMPCORE_INVALID_PLAYLIST_URL: crate::core::HRESULT =
    crate::core::HRESULT(-1072885585i32);
pub const NS_E_WMPCORE_ITEMNOTINPLAYLIST: crate::core::HRESULT =
    crate::core::HRESULT(-1072885626i32);
pub const NS_E_WMPCORE_LIST_ENTRY_NO_REF: crate::core::HRESULT =
    crate::core::HRESULT(-1072885608i32);
pub const NS_E_WMPCORE_MEDIA_ALTERNATE_REF_EMPTY: crate::core::HRESULT =
    crate::core::HRESULT(-1072885596i32);
pub const NS_E_WMPCORE_MEDIA_CHILD_PLAYLIST_UNAVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885576i32);
pub const NS_E_WMPCORE_MEDIA_ERROR_RESUME_FAILED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885617i32);
pub const NS_E_WMPCORE_MEDIA_NO_CHILD_PLAYLIST: crate::core::HRESULT =
    crate::core::HRESULT(-1072885575i32);
pub const NS_E_WMPCORE_MEDIA_UNAVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885581i32);
pub const NS_E_WMPCORE_MEDIA_URL_TOO_LONG: crate::core::HRESULT =
    crate::core::HRESULT(-1072885560i32);
pub const NS_E_WMPCORE_MISMATCHED_RUNTIME: crate::core::HRESULT =
    crate::core::HRESULT(-1072885584i32);
pub const NS_E_WMPCORE_MISNAMED_FILE: crate::core::HRESULT = crate::core::HRESULT(-1072885607i32);
pub const NS_E_WMPCORE_NOBROWSER: crate::core::HRESULT = crate::core::HRESULT(-1072885624i32);
pub const NS_E_WMPCORE_NOSOURCEURLSTRING: crate::core::HRESULT =
    crate::core::HRESULT(-1072885636i32);
pub const NS_E_WMPCORE_NO_PLAYABLE_MEDIA_IN_PLAYLIST: crate::core::HRESULT =
    crate::core::HRESULT(-1072885579i32);
pub const NS_E_WMPCORE_NO_REF_IN_ENTRY: crate::core::HRESULT = crate::core::HRESULT(-1072885616i32);
pub const NS_E_WMPCORE_PLAYLISTEMPTY: crate::core::HRESULT = crate::core::HRESULT(-1072885625i32);
pub const NS_E_WMPCORE_PLAYLIST_EMPTY_NESTED_PLAYLIST_SKIPPED_ITEMS: crate::core::HRESULT =
    crate::core::HRESULT(-1072885578i32);
pub const NS_E_WMPCORE_PLAYLIST_EMPTY_OR_SINGLE_MEDIA: crate::core::HRESULT =
    crate::core::HRESULT(-1072885621i32);
pub const NS_E_WMPCORE_PLAYLIST_EVENT_ATTRIBUTE_ABSENT: crate::core::HRESULT =
    crate::core::HRESULT(-1072885594i32);
pub const NS_E_WMPCORE_PLAYLIST_EVENT_EMPTY: crate::core::HRESULT =
    crate::core::HRESULT(-1072885593i32);
pub const NS_E_WMPCORE_PLAYLIST_IMPORT_FAILED_NO_ITEMS: crate::core::HRESULT =
    crate::core::HRESULT(-1072885583i32);
pub const NS_E_WMPCORE_PLAYLIST_ITEM_ALTERNATE_EXHAUSTED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885600i32);
pub const NS_E_WMPCORE_PLAYLIST_ITEM_ALTERNATE_INIT_FAILED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885597i32);
pub const NS_E_WMPCORE_PLAYLIST_ITEM_ALTERNATE_MORPH_FAILED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885598i32);
pub const NS_E_WMPCORE_PLAYLIST_ITEM_ALTERNATE_NAME_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-1072885599i32);
pub const NS_E_WMPCORE_PLAYLIST_ITEM_ALTERNATE_NONE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885601i32);
pub const NS_E_WMPCORE_PLAYLIST_NO_EVENT_NAME: crate::core::HRESULT =
    crate::core::HRESULT(-1072885595i32);
pub const NS_E_WMPCORE_PLAYLIST_REPEAT_EMPTY: crate::core::HRESULT =
    crate::core::HRESULT(-1072885588i32);
pub const NS_E_WMPCORE_PLAYLIST_REPEAT_END_MEDIA_NONE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885586i32);
pub const NS_E_WMPCORE_PLAYLIST_REPEAT_START_MEDIA_NONE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885587i32);
pub const NS_E_WMPCORE_PLAYLIST_STACK_EMPTY: crate::core::HRESULT =
    crate::core::HRESULT(-1072885592i32);
pub const NS_E_WMPCORE_SOME_CODECS_MISSING: crate::core::HRESULT =
    crate::core::HRESULT(-1072885551i32);
pub const NS_E_WMPCORE_TEMP_FILE_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-1072885573i32);
pub const NS_E_WMPCORE_UNAVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-1072885632i32);
pub const NS_E_WMPCORE_UNRECOGNIZED_MEDIA_URL: crate::core::HRESULT =
    crate::core::HRESULT(-1072885623i32);
pub const NS_E_WMPCORE_USER_CANCEL: crate::core::HRESULT = crate::core::HRESULT(-1072885589i32);
pub const NS_E_WMPCORE_VIDEO_TRANSFORM_FILTER_INSERTION: crate::core::HRESULT =
    crate::core::HRESULT(-1072885582i32);
pub const NS_E_WMPCORE_WEBHELPFAILED: crate::core::HRESULT = crate::core::HRESULT(-1072885618i32);
pub const NS_E_WMPCORE_WMX_ENTRYREF_NO_REF: crate::core::HRESULT =
    crate::core::HRESULT(-1072885580i32);
pub const NS_E_WMPCORE_WMX_LIST_ATTRIBUTE_NAME_EMPTY: crate::core::HRESULT =
    crate::core::HRESULT(-1072885615i32);
pub const NS_E_WMPCORE_WMX_LIST_ATTRIBUTE_NAME_ILLEGAL: crate::core::HRESULT =
    crate::core::HRESULT(-1072885614i32);
pub const NS_E_WMPCORE_WMX_LIST_ATTRIBUTE_VALUE_EMPTY: crate::core::HRESULT =
    crate::core::HRESULT(-1072885613i32);
pub const NS_E_WMPCORE_WMX_LIST_ATTRIBUTE_VALUE_ILLEGAL: crate::core::HRESULT =
    crate::core::HRESULT(-1072885612i32);
pub const NS_E_WMPCORE_WMX_LIST_ITEM_ATTRIBUTE_NAME_EMPTY: crate::core::HRESULT =
    crate::core::HRESULT(-1072885611i32);
pub const NS_E_WMPCORE_WMX_LIST_ITEM_ATTRIBUTE_NAME_ILLEGAL: crate::core::HRESULT =
    crate::core::HRESULT(-1072885610i32);
pub const NS_E_WMPCORE_WMX_LIST_ITEM_ATTRIBUTE_VALUE_EMPTY: crate::core::HRESULT =
    crate::core::HRESULT(-1072885609i32);
pub const NS_E_WMPFLASH_CANT_FIND_COM_SERVER: crate::core::HRESULT =
    crate::core::HRESULT(-1072885559i32);
pub const NS_E_WMPFLASH_INCOMPATIBLEVERSION: crate::core::HRESULT =
    crate::core::HRESULT(-1072885558i32);
pub const NS_E_WMPIM_DIALUPFAILED: crate::core::HRESULT = crate::core::HRESULT(-1072885464i32);
pub const NS_E_WMPIM_USERCANCELED: crate::core::HRESULT = crate::core::HRESULT(-1072885465i32);
pub const NS_E_WMPIM_USEROFFLINE: crate::core::HRESULT = crate::core::HRESULT(-1072885466i32);
pub const NS_E_WMPOCXGRAPH_IE_DISALLOWS_ACTIVEX_CONTROLS: crate::core::HRESULT =
    crate::core::HRESULT(-1072885557i32);
pub const NS_E_WMPOCX_ERRORMANAGERNOTAVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885803i32);
pub const NS_E_WMPOCX_NOT_RUNNING_REMOTELY: crate::core::HRESULT =
    crate::core::HRESULT(-1072885805i32);
pub const NS_E_WMPOCX_NO_ACTIVE_CORE: crate::core::HRESULT = crate::core::HRESULT(-1072885806i32);
pub const NS_E_WMPOCX_NO_REMOTE_CORE: crate::core::HRESULT = crate::core::HRESULT(-1072885807i32);
pub const NS_E_WMPOCX_NO_REMOTE_WINDOW: crate::core::HRESULT = crate::core::HRESULT(-1072885804i32);
pub const NS_E_WMPOCX_PLAYER_NOT_DOCKED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885797i32);
pub const NS_E_WMPOCX_REMOTE_PLAYER_ALREADY_RUNNING: crate::core::HRESULT =
    crate::core::HRESULT(-1072885766i32);
pub const NS_E_WMPOCX_UNABLE_TO_LOAD_SKIN: crate::core::HRESULT =
    crate::core::HRESULT(-1072885781i32);
pub const NS_E_WMPXML_ATTRIBUTENOTFOUND: crate::core::HRESULT =
    crate::core::HRESULT(-1072885833i32);
pub const NS_E_WMPXML_EMPTYDOC: crate::core::HRESULT = crate::core::HRESULT(-1072885831i32);
pub const NS_E_WMPXML_ENDOFDATA: crate::core::HRESULT = crate::core::HRESULT(-1072885835i32);
pub const NS_E_WMPXML_NOERROR: crate::core::HRESULT = crate::core::HRESULT(-1072885836i32);
pub const NS_E_WMPXML_PARSEERROR: crate::core::HRESULT = crate::core::HRESULT(-1072885834i32);
pub const NS_E_WMPXML_PINOTFOUND: crate::core::HRESULT = crate::core::HRESULT(-1072885832i32);
pub const NS_E_WMPZIP_CORRUPT: crate::core::HRESULT = crate::core::HRESULT(-1072885735i32);
pub const NS_E_WMPZIP_FILENOTFOUND: crate::core::HRESULT = crate::core::HRESULT(-1072885734i32);
pub const NS_E_WMPZIP_NOTAZIPFILE: crate::core::HRESULT = crate::core::HRESULT(-1072885736i32);
pub const NS_E_WMP_ACCESS_DENIED: crate::core::HRESULT = crate::core::HRESULT(-1072885294i32);
pub const NS_E_WMP_ADDTOLIBRARY_FAILED: crate::core::HRESULT = crate::core::HRESULT(-1072885817i32);
pub const NS_E_WMP_ALREADY_IN_USE: crate::core::HRESULT = crate::core::HRESULT(-1072885346i32);
pub const NS_E_WMP_AUDIO_CODEC_NOT_INSTALLED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885305i32);
pub const NS_E_WMP_AUDIO_DEVICE_LOST: crate::core::HRESULT = crate::core::HRESULT(-1072885275i32);
pub const NS_E_WMP_AUDIO_HW_PROBLEM: crate::core::HRESULT = crate::core::HRESULT(-1072885318i32);
pub const NS_E_WMP_AUTOPLAY_INVALID_STATE: crate::core::HRESULT =
    crate::core::HRESULT(-1072884996i32);
pub const NS_E_WMP_BAD_DRIVER: crate::core::HRESULT = crate::core::HRESULT(-1072885295i32);
pub const NS_E_WMP_BMP_BITMAP_NOT_CREATED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885712i32);
pub const NS_E_WMP_BMP_COMPRESSION_UNSUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885711i32);
pub const NS_E_WMP_BMP_INVALID_BITMASK: crate::core::HRESULT = crate::core::HRESULT(-1072885714i32);
pub const NS_E_WMP_BMP_INVALID_FORMAT: crate::core::HRESULT = crate::core::HRESULT(-1072885710i32);
pub const NS_E_WMP_BMP_TOPDOWN_DIB_UNSUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885713i32);
pub const NS_E_WMP_BSTR_TOO_LONG: crate::core::HRESULT = crate::core::HRESULT(-1072885006i32);
pub const NS_E_WMP_BURN_DISC_OVERFLOW: crate::core::HRESULT = crate::core::HRESULT(-1072885287i32);
pub const NS_E_WMP_CANNOT_BURN_NON_LOCAL_FILE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885546i32);
pub const NS_E_WMP_CANNOT_FIND_FILE: crate::core::HRESULT = crate::core::HRESULT(-1072885353i32);
pub const NS_E_WMP_CANNOT_FIND_FOLDER: crate::core::HRESULT = crate::core::HRESULT(-1072885801i32);
pub const NS_E_WMP_CANT_PLAY_PROTECTED: crate::core::HRESULT = crate::core::HRESULT(-1072885773i32);
pub const NS_E_WMP_CD_ANOTHER_USER: crate::core::HRESULT = crate::core::HRESULT(-1072885297i32);
pub const NS_E_WMP_CD_STASH_NO_SPACE: crate::core::HRESULT = crate::core::HRESULT(-1072885291i32);
pub const NS_E_WMP_CODEC_NEEDED_WITH_4CC: crate::core::HRESULT =
    crate::core::HRESULT(-1072885343i32);
pub const NS_E_WMP_CODEC_NEEDED_WITH_FORMATTAG: crate::core::HRESULT =
    crate::core::HRESULT(-1072885342i32);
pub const NS_E_WMP_COMPONENT_REVOKED: crate::core::HRESULT = crate::core::HRESULT(-1072884986i32);
pub const NS_E_WMP_CONNECT_TIMEOUT: crate::core::HRESULT = crate::core::HRESULT(-1072885311i32);
pub const NS_E_WMP_CONVERT_FILE_CORRUPT: crate::core::HRESULT =
    crate::core::HRESULT(-1072885413i32);
pub const NS_E_WMP_CONVERT_FILE_FAILED: crate::core::HRESULT = crate::core::HRESULT(-1072885416i32);
pub const NS_E_WMP_CONVERT_NO_RIGHTS_ERRORURL: crate::core::HRESULT =
    crate::core::HRESULT(-1072885415i32);
pub const NS_E_WMP_CONVERT_NO_RIGHTS_NOERRORURL: crate::core::HRESULT =
    crate::core::HRESULT(-1072885414i32);
pub const NS_E_WMP_CONVERT_PLUGIN_UNAVAILABLE_ERRORURL: crate::core::HRESULT =
    crate::core::HRESULT(-1072885412i32);
pub const NS_E_WMP_CONVERT_PLUGIN_UNAVAILABLE_NOERRORURL: crate::core::HRESULT =
    crate::core::HRESULT(-1072885411i32);
pub const NS_E_WMP_CONVERT_PLUGIN_UNKNOWN_FILE_OWNER: crate::core::HRESULT =
    crate::core::HRESULT(-1072885410i32);
pub const NS_E_WMP_CS_JPGPOSITIONIMAGE: crate::core::HRESULT = crate::core::HRESULT(-1072885746i32);
pub const NS_E_WMP_CS_NOTEVENLYDIVISIBLE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885745i32);
pub const NS_E_WMP_DAI_SONGTOOSHORT: crate::core::HRESULT = crate::core::HRESULT(-1072885687i32);
pub const NS_E_WMP_DRM_ACQUIRING_LICENSE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885246i32);
pub const NS_E_WMP_DRM_CANNOT_RESTORE: crate::core::HRESULT = crate::core::HRESULT(-1072885288i32);
pub const NS_E_WMP_DRM_COMPONENT_FAILURE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885278i32);
pub const NS_E_WMP_DRM_CORRUPT_BACKUP: crate::core::HRESULT = crate::core::HRESULT(-1072885324i32);
pub const NS_E_WMP_DRM_DRIVER_AUTH_FAILURE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885302i32);
pub const NS_E_WMP_DRM_GENERIC_LICENSE_FAILURE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885286i32);
pub const NS_E_WMP_DRM_INDIV_FAILED: crate::core::HRESULT = crate::core::HRESULT(-1072885283i32);
pub const NS_E_WMP_DRM_INVALID_SIG: crate::core::HRESULT = crate::core::HRESULT(-1072885289i32);
pub const NS_E_WMP_DRM_LICENSE_CONTENT_REVOKED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885241i32);
pub const NS_E_WMP_DRM_LICENSE_EXPIRED: crate::core::HRESULT = crate::core::HRESULT(-1072885245i32);
pub const NS_E_WMP_DRM_LICENSE_NOSAP: crate::core::HRESULT = crate::core::HRESULT(-1072885240i32);
pub const NS_E_WMP_DRM_LICENSE_NOTACQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885244i32);
pub const NS_E_WMP_DRM_LICENSE_NOTENABLED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885243i32);
pub const NS_E_WMP_DRM_LICENSE_SERVER_UNAVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885323i32);
pub const NS_E_WMP_DRM_LICENSE_UNUSABLE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885242i32);
pub const NS_E_WMP_DRM_NEEDS_AUTHORIZATION: crate::core::HRESULT =
    crate::core::HRESULT(-1072885296i32);
pub const NS_E_WMP_DRM_NEW_HARDWARE: crate::core::HRESULT = crate::core::HRESULT(-1072885290i32);
pub const NS_E_WMP_DRM_NOT_ACQUIRING: crate::core::HRESULT = crate::core::HRESULT(-1072885055i32);
pub const NS_E_WMP_DRM_NO_DEVICE_CERT: crate::core::HRESULT = crate::core::HRESULT(-1072885277i32);
pub const NS_E_WMP_DRM_NO_RIGHTS: crate::core::HRESULT = crate::core::HRESULT(-1072885284i32);
pub const NS_E_WMP_DRM_NO_SECURE_CLOCK: crate::core::HRESULT = crate::core::HRESULT(-1072885285i32);
pub const NS_E_WMP_DRM_UNABLE_TO_ACQUIRE_LICENSE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885239i32);
pub const NS_E_WMP_DSHOW_UNSUPPORTED_FORMAT: crate::core::HRESULT =
    crate::core::HRESULT(-1072885350i32);
pub const NS_E_WMP_ERASE_FAILED: crate::core::HRESULT = crate::core::HRESULT(-1072885548i32);
pub const NS_E_WMP_EXTERNAL_NOTREADY: crate::core::HRESULT = crate::core::HRESULT(-1072885796i32);
pub const NS_E_WMP_FAILED_TO_OPEN_IMAGE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885692i32);
pub const NS_E_WMP_FAILED_TO_OPEN_WMD: crate::core::HRESULT = crate::core::HRESULT(-1072885774i32);
pub const NS_E_WMP_FAILED_TO_RIP_TRACK: crate::core::HRESULT = crate::core::HRESULT(-1072885549i32);
pub const NS_E_WMP_FAILED_TO_SAVE_FILE: crate::core::HRESULT = crate::core::HRESULT(-1072885777i32);
pub const NS_E_WMP_FAILED_TO_SAVE_PLAYLIST: crate::core::HRESULT =
    crate::core::HRESULT(-1072885775i32);
pub const NS_E_WMP_FILESCANALREADYSTARTED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885826i32);
pub const NS_E_WMP_FILE_DOES_NOT_FIT_ON_CD: crate::core::HRESULT =
    crate::core::HRESULT(-1072885544i32);
pub const NS_E_WMP_FILE_NO_DURATION: crate::core::HRESULT = crate::core::HRESULT(-1072885543i32);
pub const NS_E_WMP_FILE_OPEN_FAILED: crate::core::HRESULT = crate::core::HRESULT(-1072885327i32);
pub const NS_E_WMP_FILE_TYPE_CANNOT_BURN_TO_AUDIO_CD: crate::core::HRESULT =
    crate::core::HRESULT(-1072885545i32);
pub const NS_E_WMP_FORMAT_FAILED: crate::core::HRESULT = crate::core::HRESULT(-1072885547i32);
pub const NS_E_WMP_GIF_BAD_VERSION_NUMBER: crate::core::HRESULT =
    crate::core::HRESULT(-1072885722i32);
pub const NS_E_WMP_GIF_INVALID_FORMAT: crate::core::HRESULT = crate::core::HRESULT(-1072885723i32);
pub const NS_E_WMP_GIF_NO_IMAGE_IN_FILE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885721i32);
pub const NS_E_WMP_GIF_UNEXPECTED_ENDOFFILE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885724i32);
pub const NS_E_WMP_GOFULLSCREEN_FAILED: crate::core::HRESULT = crate::core::HRESULT(-1072885313i32);
pub const NS_E_WMP_HME_INVALIDOBJECTID: crate::core::HRESULT = crate::core::HRESULT(-1072885825i32);
pub const NS_E_WMP_HME_NOTSEARCHABLEFORITEMS: crate::core::HRESULT =
    crate::core::HRESULT(-1072885823i32);
pub const NS_E_WMP_HME_STALEREQUEST: crate::core::HRESULT = crate::core::HRESULT(-1072885822i32);
pub const NS_E_WMP_HWND_NOTFOUND: crate::core::HRESULT = crate::core::HRESULT(-1072885156i32);
pub const NS_E_WMP_IMAGE_FILETYPE_UNSUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885726i32);
pub const NS_E_WMP_IMAGE_INVALID_FORMAT: crate::core::HRESULT =
    crate::core::HRESULT(-1072885725i32);
pub const NS_E_WMP_IMAPI2_ERASE_DEVICE_BUSY: crate::core::HRESULT =
    crate::core::HRESULT(-1072885279i32);
pub const NS_E_WMP_IMAPI2_ERASE_FAIL: crate::core::HRESULT = crate::core::HRESULT(-1072885280i32);
pub const NS_E_WMP_IMAPI_DEVICE_BUSY: crate::core::HRESULT = crate::core::HRESULT(-1072885330i32);
pub const NS_E_WMP_IMAPI_DEVICE_INVALIDTYPE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885303i32);
pub const NS_E_WMP_IMAPI_DEVICE_NOTPRESENT: crate::core::HRESULT =
    crate::core::HRESULT(-1072885331i32);
pub const NS_E_WMP_IMAPI_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-1072885345i32);
pub const NS_E_WMP_IMAPI_GENERIC: crate::core::HRESULT = crate::core::HRESULT(-1072885333i32);
pub const NS_E_WMP_IMAPI_LOSS_OF_STREAMING: crate::core::HRESULT =
    crate::core::HRESULT(-1072885329i32);
pub const NS_E_WMP_IMAPI_MEDIA_INCOMPATIBLE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885274i32);
pub const NS_E_WMP_INVALID_ASX: crate::core::HRESULT = crate::core::HRESULT(-1072885347i32);
pub const NS_E_WMP_INVALID_KEY: crate::core::HRESULT = crate::core::HRESULT(-1072885298i32);
pub const NS_E_WMP_INVALID_LIBRARY_ADD: crate::core::HRESULT = crate::core::HRESULT(-1072885316i32);
pub const NS_E_WMP_INVALID_MAX_VAL: crate::core::HRESULT = crate::core::HRESULT(-1072885751i32);
pub const NS_E_WMP_INVALID_MIN_VAL: crate::core::HRESULT = crate::core::HRESULT(-1072885750i32);
pub const NS_E_WMP_INVALID_PROTOCOL: crate::core::HRESULT = crate::core::HRESULT(-1072885317i32);
pub const NS_E_WMP_INVALID_REQUEST: crate::core::HRESULT = crate::core::HRESULT(-1072885292i32);
pub const NS_E_WMP_INVALID_SKIN: crate::core::HRESULT = crate::core::HRESULT(-1072885780i32);
pub const NS_E_WMP_JPGTRANSPARENCY: crate::core::HRESULT = crate::core::HRESULT(-1072885755i32);
pub const NS_E_WMP_JPG_BAD_DCTSIZE: crate::core::HRESULT = crate::core::HRESULT(-1072885707i32);
pub const NS_E_WMP_JPG_BAD_PRECISION: crate::core::HRESULT = crate::core::HRESULT(-1072885705i32);
pub const NS_E_WMP_JPG_BAD_VERSION_NUMBER: crate::core::HRESULT =
    crate::core::HRESULT(-1072885706i32);
pub const NS_E_WMP_JPG_CCIR601_NOTIMPL: crate::core::HRESULT = crate::core::HRESULT(-1072885704i32);
pub const NS_E_WMP_JPG_FRACT_SAMPLE_NOTIMPL: crate::core::HRESULT =
    crate::core::HRESULT(-1072885701i32);
pub const NS_E_WMP_JPG_IMAGE_TOO_BIG: crate::core::HRESULT = crate::core::HRESULT(-1072885700i32);
pub const NS_E_WMP_JPG_INVALID_FORMAT: crate::core::HRESULT = crate::core::HRESULT(-1072885708i32);
pub const NS_E_WMP_JPG_JERR_ARITHCODING_NOTIMPL: crate::core::HRESULT =
    crate::core::HRESULT(-1072885709i32);
pub const NS_E_WMP_JPG_NO_IMAGE_IN_FILE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885703i32);
pub const NS_E_WMP_JPG_READ_ERROR: crate::core::HRESULT = crate::core::HRESULT(-1072885702i32);
pub const NS_E_WMP_JPG_SOF_UNSUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-1072885698i32);
pub const NS_E_WMP_JPG_UNEXPECTED_ENDOFFILE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885699i32);
pub const NS_E_WMP_JPG_UNKNOWN_MARKER: crate::core::HRESULT = crate::core::HRESULT(-1072885697i32);
pub const NS_E_WMP_LICENSE_REQUIRED: crate::core::HRESULT = crate::core::HRESULT(-1072885238i32);
pub const NS_E_WMP_LICENSE_RESTRICTS: crate::core::HRESULT = crate::core::HRESULT(-1072885293i32);
pub const NS_E_WMP_LOCKEDINSKINMODE: crate::core::HRESULT = crate::core::HRESULT(-1072885778i32);
pub const NS_E_WMP_LOGON_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-1072885354i32);
pub const NS_E_WMP_MF_CODE_EXPIRED: crate::core::HRESULT = crate::core::HRESULT(-1072885824i32);
pub const NS_E_WMP_MLS_STALE_DATA: crate::core::HRESULT = crate::core::HRESULT(-1072885795i32);
pub const NS_E_WMP_MMS_NOT_SUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-1072885315i32);
pub const NS_E_WMP_MSSAP_NOT_AVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-1072885341i32);
pub const NS_E_WMP_MULTICAST_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-1072885310i32);
pub const NS_E_WMP_MULTIPLE_ERROR_IN_PLAYLIST: crate::core::HRESULT =
    crate::core::HRESULT(-1072885281i32);
pub const NS_E_WMP_NEED_UPGRADE: crate::core::HRESULT = crate::core::HRESULT(-1072885319i32);
pub const NS_E_WMP_NETWORK_ERROR: crate::core::HRESULT = crate::core::HRESULT(-1072885312i32);
pub const NS_E_WMP_NETWORK_FIREWALL: crate::core::HRESULT = crate::core::HRESULT(-1072885322i32);
pub const NS_E_WMP_NETWORK_RESOURCE_FAILURE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885301i32);
pub const NS_E_WMP_NONMEDIA_FILES: crate::core::HRESULT = crate::core::HRESULT(-1072885348i32);
pub const NS_E_WMP_NO_DISK_SPACE: crate::core::HRESULT = crate::core::HRESULT(-1072885355i32);
pub const NS_E_WMP_NO_PROTOCOLS_SELECTED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885314i32);
pub const NS_E_WMP_NO_REMOVABLE_MEDIA: crate::core::HRESULT = crate::core::HRESULT(-1072885321i32);
pub const NS_E_WMP_OUTOFMEMORY: crate::core::HRESULT = crate::core::HRESULT(-1072885306i32);
pub const NS_E_WMP_PATH_ALREADY_IN_LIBRARY: crate::core::HRESULT =
    crate::core::HRESULT(-1072885830i32);
pub const NS_E_WMP_PLAYLIST_EXISTS: crate::core::HRESULT = crate::core::HRESULT(-1072885349i32);
pub const NS_E_WMP_PLUGINDLL_NOTFOUND: crate::core::HRESULT = crate::core::HRESULT(-1072885799i32);
pub const NS_E_WMP_PNG_INVALIDFORMAT: crate::core::HRESULT = crate::core::HRESULT(-1072885720i32);
pub const NS_E_WMP_PNG_UNSUPPORTED_BAD_CRC: crate::core::HRESULT =
    crate::core::HRESULT(-1072885715i32);
pub const NS_E_WMP_PNG_UNSUPPORTED_BITDEPTH: crate::core::HRESULT =
    crate::core::HRESULT(-1072885719i32);
pub const NS_E_WMP_PNG_UNSUPPORTED_COMPRESSION: crate::core::HRESULT =
    crate::core::HRESULT(-1072885718i32);
pub const NS_E_WMP_PNG_UNSUPPORTED_FILTER: crate::core::HRESULT =
    crate::core::HRESULT(-1072885717i32);
pub const NS_E_WMP_PNG_UNSUPPORTED_INTERLACE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885716i32);
pub const NS_E_WMP_POLICY_VALUE_NOT_CONFIGURED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885206i32);
pub const NS_E_WMP_PROTECTED_CONTENT: crate::core::HRESULT = crate::core::HRESULT(-1072885237i32);
pub const NS_E_WMP_PROTOCOL_PROBLEM: crate::core::HRESULT = crate::core::HRESULT(-1072885356i32);
pub const NS_E_WMP_PROXY_CONNECT_TIMEOUT: crate::core::HRESULT =
    crate::core::HRESULT(-1072885320i32);
pub const NS_E_WMP_PROXY_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-1072885308i32);
pub const NS_E_WMP_RBC_JPGMAPPINGIMAGE: crate::core::HRESULT = crate::core::HRESULT(-1072885756i32);
pub const NS_E_WMP_RECORDING_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885815i32);
pub const NS_E_WMP_RIP_FAILED: crate::core::HRESULT = crate::core::HRESULT(-1072885550i32);
pub const NS_E_WMP_SAVEAS_READONLY: crate::core::HRESULT = crate::core::HRESULT(-1072885776i32);
pub const NS_E_WMP_SENDMAILFAILED: crate::core::HRESULT = crate::core::HRESULT(-1072885779i32);
pub const NS_E_WMP_SERVER_DNS_TIMEOUT: crate::core::HRESULT = crate::core::HRESULT(-1072885309i32);
pub const NS_E_WMP_SERVER_INACCESSIBLE: crate::core::HRESULT = crate::core::HRESULT(-1072885352i32);
pub const NS_E_WMP_SERVER_NONEWCONNECTIONS: crate::core::HRESULT =
    crate::core::HRESULT(-1072885282i32);
pub const NS_E_WMP_SERVER_NOT_RESPONDING: crate::core::HRESULT =
    crate::core::HRESULT(-1072885325i32);
pub const NS_E_WMP_SERVER_SECURITY_ERROR: crate::core::HRESULT =
    crate::core::HRESULT(-1072885276i32);
pub const NS_E_WMP_SERVER_UNAVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-1072885328i32);
pub const NS_E_WMP_STREAMING_RECORDING_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885800i32);
pub const NS_E_WMP_TAMPERED_CONTENT: crate::core::HRESULT = crate::core::HRESULT(-1072885307i32);
pub const NS_E_WMP_UDRM_NOUSERLIST: crate::core::HRESULT = crate::core::HRESULT(-1072885056i32);
pub const NS_E_WMP_UI_NOSKININZIP: crate::core::HRESULT = crate::core::HRESULT(-1072885785i32);
pub const NS_E_WMP_UI_NOTATHEMEFILE: crate::core::HRESULT = crate::core::HRESULT(-1072885792i32);
pub const NS_E_WMP_UI_OBJECTNOTFOUND: crate::core::HRESULT = crate::core::HRESULT(-1072885787i32);
pub const NS_E_WMP_UI_PASSTHROUGH: crate::core::HRESULT = crate::core::HRESULT(-1072885788i32);
pub const NS_E_WMP_UI_SECONDHANDLER: crate::core::HRESULT = crate::core::HRESULT(-1072885786i32);
pub const NS_E_WMP_UI_SUBCONTROLSNOTSUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885794i32);
pub const NS_E_WMP_UI_SUBELEMENTNOTFOUND: crate::core::HRESULT =
    crate::core::HRESULT(-1072885791i32);
pub const NS_E_WMP_UI_VERSIONMISMATCH: crate::core::HRESULT = crate::core::HRESULT(-1072885793i32);
pub const NS_E_WMP_UI_VERSIONPARSE: crate::core::HRESULT = crate::core::HRESULT(-1072885790i32);
pub const NS_E_WMP_UI_VIEWIDNOTFOUND: crate::core::HRESULT = crate::core::HRESULT(-1072885789i32);
pub const NS_E_WMP_UNKNOWN_ERROR: crate::core::HRESULT = crate::core::HRESULT(-1072885299i32);
pub const NS_E_WMP_UNSUPPORTED_FORMAT: crate::core::HRESULT = crate::core::HRESULT(-1072885351i32);
pub const NS_E_WMP_UPGRADE_APPLICATION: crate::core::HRESULT = crate::core::HRESULT(-1072885300i32);
pub const NS_E_WMP_URLDOWNLOADFAILED: crate::core::HRESULT = crate::core::HRESULT(-1072885782i32);
pub const NS_E_WMP_VERIFY_ONLINE: crate::core::HRESULT = crate::core::HRESULT(-1072885326i32);
pub const NS_E_WMP_VIDEO_CODEC_NOT_INSTALLED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885304i32);
pub const NS_E_WMP_WINDOWSAPIFAILURE: crate::core::HRESULT = crate::core::HRESULT(-1072885816i32);
pub const NS_E_WMP_WMDM_BUSY: crate::core::HRESULT = crate::core::HRESULT(-1072885336i32);
pub const NS_E_WMP_WMDM_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-1072885344i32);
pub const NS_E_WMP_WMDM_INCORRECT_RIGHTS: crate::core::HRESULT =
    crate::core::HRESULT(-1072885334i32);
pub const NS_E_WMP_WMDM_INTERFACEDEAD: crate::core::HRESULT = crate::core::HRESULT(-1072885340i32);
pub const NS_E_WMP_WMDM_LICENSE_EXPIRED: crate::core::HRESULT =
    crate::core::HRESULT(-1072885337i32);
pub const NS_E_WMP_WMDM_LICENSE_NOTEXIST: crate::core::HRESULT =
    crate::core::HRESULT(-1072885338i32);
pub const NS_E_WMP_WMDM_NORIGHTS: crate::core::HRESULT = crate::core::HRESULT(-1072885335i32);
pub const NS_E_WMP_WMDM_NOTCERTIFIED: crate::core::HRESULT = crate::core::HRESULT(-1072885339i32);
pub const NS_E_WMR_CANNOT_RENDER_BINARY_STREAM: crate::core::HRESULT =
    crate::core::HRESULT(-1072885661i32);
pub const NS_E_WMR_NOCALLBACKAVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-1072885666i32);
pub const NS_E_WMR_NOSOURCEFILTER: crate::core::HRESULT = crate::core::HRESULT(-1072885668i32);
pub const NS_E_WMR_PINNOTFOUND: crate::core::HRESULT = crate::core::HRESULT(-1072885670i32);
pub const NS_E_WMR_PINTYPENOMATCH: crate::core::HRESULT = crate::core::HRESULT(-1072885667i32);
pub const NS_E_WMR_SAMPLEPROPERTYNOTSET: crate::core::HRESULT =
    crate::core::HRESULT(-1072885662i32);
pub const NS_E_WMR_UNSUPPORTEDSTREAM: crate::core::HRESULT = crate::core::HRESULT(-1072885671i32);
pub const NS_E_WMR_WAITINGONFORMATSWITCH: crate::core::HRESULT =
    crate::core::HRESULT(-1072885669i32);
pub const NS_E_WMR_WILLNOT_RENDER_BINARY_STREAM: crate::core::HRESULT =
    crate::core::HRESULT(-1072885659i32);
pub const NS_E_WMX_ATTRIBUTE_ALREADY_EXISTS: crate::core::HRESULT =
    crate::core::HRESULT(-1072885649i32);
pub const NS_E_WMX_ATTRIBUTE_DOES_NOT_EXIST: crate::core::HRESULT =
    crate::core::HRESULT(-1072885650i32);
pub const NS_E_WMX_ATTRIBUTE_UNRETRIEVABLE: crate::core::HRESULT =
    crate::core::HRESULT(-1072885648i32);
pub const NS_E_WMX_INVALID_FORMAT_OVER_NESTING: crate::core::HRESULT =
    crate::core::HRESULT(-1072885642i32);
pub const NS_E_WMX_ITEM_DOES_NOT_EXIST: crate::core::HRESULT = crate::core::HRESULT(-1072885647i32);
pub const NS_E_WMX_ITEM_TYPE_ILLEGAL: crate::core::HRESULT = crate::core::HRESULT(-1072885646i32);
pub const NS_E_WMX_ITEM_UNSETTABLE: crate::core::HRESULT = crate::core::HRESULT(-1072885645i32);
pub const NS_E_WMX_PLAYLIST_EMPTY: crate::core::HRESULT = crate::core::HRESULT(-1072885644i32);
pub const NS_E_WMX_UNRECOGNIZED_PLAYLIST_FORMAT: crate::core::HRESULT =
    crate::core::HRESULT(-1072885656i32);
pub const NS_E_WONT_DO_DIGITAL: crate::core::HRESULT = crate::core::HRESULT(-1072885837i32);
pub const NS_E_WRONG_OS_VERSION: crate::core::HRESULT = crate::core::HRESULT(-1072884643i32);
pub const NS_E_WRONG_PUBLISHING_POINT_TYPE: crate::core::HRESULT =
    crate::core::HRESULT(-1072884654i32);
pub const NS_E_WSX_INVALID_VERSION: crate::core::HRESULT = crate::core::HRESULT(-1072884450i32);
pub const NS_I_CATATONIC_AUTO_UNFAIL: crate::core::HRESULT = crate::core::HRESULT(-2146631270i32);
pub const NS_I_CATATONIC_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2146631271i32);
pub const NS_I_CUB_RUNNING: crate::core::HRESULT = crate::core::HRESULT(1074593874i32);
pub const NS_I_CUB_START: crate::core::HRESULT = crate::core::HRESULT(1074593873i32);
pub const NS_I_CUB_UNFAIL_LINK: crate::core::HRESULT = crate::core::HRESULT(1074594193i32);
pub const NS_I_DISK_REBUILD_ABORTED: crate::core::HRESULT = crate::core::HRESULT(1074593880i32);
pub const NS_I_DISK_REBUILD_FINISHED: crate::core::HRESULT = crate::core::HRESULT(1074593879i32);
pub const NS_I_DISK_REBUILD_STARTED: crate::core::HRESULT = crate::core::HRESULT(1074593878i32);
pub const NS_I_DISK_START: crate::core::HRESULT = crate::core::HRESULT(1074593876i32);
pub const NS_I_DISK_STOP: crate::core::HRESULT = crate::core::HRESULT(1074594200i32);
pub const NS_I_EXISTING_PACKETIZER: crate::core::HRESULT = crate::core::HRESULT(1074605827i32);
pub const NS_I_KILL_CONNECTION: crate::core::HRESULT = crate::core::HRESULT(1074593886i32);
pub const NS_I_KILL_USERSESSION: crate::core::HRESULT = crate::core::HRESULT(1074593885i32);
pub const NS_I_LIMIT_BANDWIDTH: crate::core::HRESULT = crate::core::HRESULT(1074593904i32);
pub const NS_I_LIMIT_FUNNELS: crate::core::HRESULT = crate::core::HRESULT(1074593881i32);
pub const NS_I_LOGGING_FAILED: crate::core::HRESULT = crate::core::HRESULT(1074593902i32);
pub const NS_I_MANUAL_PROXY: crate::core::HRESULT = crate::core::HRESULT(1074605828i32);
pub const NS_I_NOLOG_STOP: crate::core::HRESULT = crate::core::HRESULT(1074605825i32);
pub const NS_I_PLAYLIST_CHANGE_RECEDING: crate::core::HRESULT = crate::core::HRESULT(1074599102i32);
pub const NS_I_REBUILD_DISK: crate::core::HRESULT = crate::core::HRESULT(1074593887i32);
pub const NS_I_RECONNECTED: crate::core::HRESULT = crate::core::HRESULT(1074605823i32);
pub const NS_I_RESTRIPE_CUB_OUT: crate::core::HRESULT = crate::core::HRESULT(1074594199i32);
pub const NS_I_RESTRIPE_DISK_OUT: crate::core::HRESULT = crate::core::HRESULT(1074594198i32);
pub const NS_I_RESTRIPE_DONE: crate::core::HRESULT = crate::core::HRESULT(1074594196i32);
pub const NS_I_RESTRIPE_START: crate::core::HRESULT = crate::core::HRESULT(1074594195i32);
pub const NS_I_START_DISK: crate::core::HRESULT = crate::core::HRESULT(1074593882i32);
pub const NS_I_STOP_CUB: crate::core::HRESULT = crate::core::HRESULT(1074593884i32);
pub const NS_I_STOP_DISK: crate::core::HRESULT = crate::core::HRESULT(1074593883i32);
pub const NS_I_TIGER_START: crate::core::HRESULT = crate::core::HRESULT(1074593871i32);
pub const NS_S_CALLABORTED: crate::core::HRESULT = crate::core::HRESULT(851969i32);
pub const NS_S_CALLPENDING: crate::core::HRESULT = crate::core::HRESULT(851968i32);
pub const NS_S_CHANGENOTICE: crate::core::HRESULT = crate::core::HRESULT(864013i32);
pub const NS_S_DEGRADING_QUALITY: crate::core::HRESULT = crate::core::HRESULT(854985i32);
pub const NS_S_DRM_ACQUIRE_CANCELLED: crate::core::HRESULT = crate::core::HRESULT(862023i32);
pub const NS_S_DRM_BURNABLE_TRACK: crate::core::HRESULT = crate::core::HRESULT(862062i32);
pub const NS_S_DRM_BURNABLE_TRACK_WITH_PLAYLIST_RESTRICTION: crate::core::HRESULT =
    crate::core::HRESULT(862063i32);
pub const NS_S_DRM_INDIVIDUALIZED: crate::core::HRESULT = crate::core::HRESULT(861991i32);
pub const NS_S_DRM_LICENSE_ACQUIRED: crate::core::HRESULT = crate::core::HRESULT(861990i32);
pub const NS_S_DRM_MONITOR_CANCELLED: crate::core::HRESULT = crate::core::HRESULT(862022i32);
pub const NS_S_DRM_NEEDS_INDIVIDUALIZATION: crate::core::HRESULT = crate::core::HRESULT(862174i32);
pub const NS_S_EOSRECEDING: crate::core::HRESULT = crate::core::HRESULT(864009i32);
pub const NS_S_NAVIGATION_COMPLETE_WITH_ERRORS: crate::core::HRESULT =
    crate::core::HRESULT(856926i32);
pub const NS_S_NEED_TO_BUY_BURN_RIGHTS: crate::core::HRESULT = crate::core::HRESULT(856283i32);
pub const NS_S_OPERATION_PENDING: crate::core::HRESULT = crate::core::HRESULT(856398i32);
pub const NS_S_PUBLISHING_POINT_STARTED_WITH_FAILED_SINKS: crate::core::HRESULT =
    crate::core::HRESULT(857369i32);
pub const NS_S_REBOOT_RECOMMENDED: crate::core::HRESULT = crate::core::HRESULT(862968i32);
pub const NS_S_REBOOT_REQUIRED: crate::core::HRESULT = crate::core::HRESULT(862969i32);
pub const NS_S_REBUFFERING: crate::core::HRESULT = crate::core::HRESULT(854984i32);
pub const NS_S_STREAM_TRUNCATED: crate::core::HRESULT = crate::core::HRESULT(851970i32);
pub const NS_S_TRACK_ALREADY_DOWNLOADED: crate::core::HRESULT = crate::core::HRESULT(856929i32);
pub const NS_S_TRACK_BUY_REQUIRES_ALBUM_PURCHASE: crate::core::HRESULT =
    crate::core::HRESULT(856921i32);
pub const NS_S_TRANSCRYPTOR_EOF: crate::core::HRESULT = crate::core::HRESULT(855003i32);
pub const NS_S_WMG_ADVISE_DROP_FRAME: crate::core::HRESULT = crate::core::HRESULT(856166i32);
pub const NS_S_WMG_ADVISE_DROP_TO_KEYFRAME: crate::core::HRESULT = crate::core::HRESULT(856167i32);
pub const NS_S_WMG_FORCE_DROP_FRAME: crate::core::HRESULT = crate::core::HRESULT(856143i32);
pub const NS_S_WMPBR_PARTIALSUCCESS: crate::core::HRESULT = crate::core::HRESULT(856374i32);
pub const NS_S_WMPBR_SUCCESS: crate::core::HRESULT = crate::core::HRESULT(856373i32);
pub const NS_S_WMPCORE_COMMAND_NOT_AVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(856325i32);
pub const NS_S_WMPCORE_MEDIA_CHILD_PLAYLIST_OPEN_PENDING: crate::core::HRESULT =
    crate::core::HRESULT(856329i32);
pub const NS_S_WMPCORE_MEDIA_VALIDATION_PENDING: crate::core::HRESULT =
    crate::core::HRESULT(856323i32);
pub const NS_S_WMPCORE_MORE_NODES_AVAIABLE: crate::core::HRESULT = crate::core::HRESULT(856330i32);
pub const NS_S_WMPCORE_PLAYLISTCLEARABORT: crate::core::HRESULT = crate::core::HRESULT(856318i32);
pub const NS_S_WMPCORE_PLAYLISTREMOVEITEMABORT: crate::core::HRESULT =
    crate::core::HRESULT(856319i32);
pub const NS_S_WMPCORE_PLAYLIST_COLLAPSED_TO_SINGLE_MEDIA: crate::core::HRESULT =
    crate::core::HRESULT(856328i32);
pub const NS_S_WMPCORE_PLAYLIST_CREATION_PENDING: crate::core::HRESULT =
    crate::core::HRESULT(856322i32);
pub const NS_S_WMPCORE_PLAYLIST_IMPORT_MISSING_ITEMS: crate::core::HRESULT =
    crate::core::HRESULT(856327i32);
pub const NS_S_WMPCORE_PLAYLIST_NAME_AUTO_GENERATED: crate::core::HRESULT =
    crate::core::HRESULT(856326i32);
pub const NS_S_WMPCORE_PLAYLIST_REPEAT_SECONDARY_SEGMENTS_IGNORED: crate::core::HRESULT =
    crate::core::HRESULT(856324i32);
pub const NS_S_WMPEFFECT_OPAQUE: crate::core::HRESULT = crate::core::HRESULT(856389i32);
pub const NS_S_WMPEFFECT_TRANSPARENT: crate::core::HRESULT = crate::core::HRESULT(856388i32);
pub const NS_S_WMP_EXCEPTION: crate::core::HRESULT = crate::core::HRESULT(856041i32);
pub const NS_S_WMP_LOADED_BMP_IMAGE: crate::core::HRESULT = crate::core::HRESULT(856130i32);
pub const NS_S_WMP_LOADED_GIF_IMAGE: crate::core::HRESULT = crate::core::HRESULT(856128i32);
pub const NS_S_WMP_LOADED_JPG_IMAGE: crate::core::HRESULT = crate::core::HRESULT(856131i32);
pub const NS_S_WMP_LOADED_PNG_IMAGE: crate::core::HRESULT = crate::core::HRESULT(856129i32);
pub const NS_S_WMP_UI_VERSIONMISMATCH: crate::core::HRESULT = crate::core::HRESULT(856040i32);
pub const NS_S_WMR_ALREADYRENDERED: crate::core::HRESULT = crate::core::HRESULT(856159i32);
pub const NS_S_WMR_PINTYPEFULLMATCH: crate::core::HRESULT = crate::core::HRESULT(856161i32);
pub const NS_S_WMR_PINTYPEPARTIALMATCH: crate::core::HRESULT = crate::core::HRESULT(856160i32);
pub const NS_W_FILE_BANDWIDTH_LIMIT: crate::core::HRESULT = crate::core::HRESULT(-2146631676i32);
pub const NS_W_SERVER_BANDWIDTH_LIMIT: crate::core::HRESULT = crate::core::HRESULT(-2146631677i32);
pub const NS_W_UNKNOWN_EVENT: crate::core::HRESULT = crate::core::HRESULT(-2146631584i32);
pub struct OLIADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
impl ::core::marker::Copy for OLIADPCMWAVEFORMAT {}
impl ::core::clone::Clone for OLIADPCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OLIADPCMWAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLIADPCMWAVEFORMAT")
            .field("wfx", &self.wfx)
            .finish()
    }
}
impl ::core::cmp::PartialEq for OLIADPCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx
    }
}
impl ::core::cmp::Eq for OLIADPCMWAVEFORMAT {}
impl FromIntoMemory for OLIADPCMWAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 18);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        Self { wfx: f_wfx }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 18);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
    }
    fn size() -> usize {
        18
    }
}
pub struct OLICELPWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
impl ::core::marker::Copy for OLICELPWAVEFORMAT {}
impl ::core::clone::Clone for OLICELPWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OLICELPWAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLICELPWAVEFORMAT")
            .field("wfx", &self.wfx)
            .finish()
    }
}
impl ::core::cmp::PartialEq for OLICELPWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx
    }
}
impl ::core::cmp::Eq for OLICELPWAVEFORMAT {}
impl FromIntoMemory for OLICELPWAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 18);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        Self { wfx: f_wfx }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 18);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
    }
    fn size() -> usize {
        18
    }
}
pub struct OLIGSMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
impl ::core::marker::Copy for OLIGSMWAVEFORMAT {}
impl ::core::clone::Clone for OLIGSMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OLIGSMWAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLIGSMWAVEFORMAT")
            .field("wfx", &self.wfx)
            .finish()
    }
}
impl ::core::cmp::PartialEq for OLIGSMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx
    }
}
impl ::core::cmp::Eq for OLIGSMWAVEFORMAT {}
impl FromIntoMemory for OLIGSMWAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 18);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        Self { wfx: f_wfx }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 18);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
    }
    fn size() -> usize {
        18
    }
}
pub struct OLIOPRWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
impl ::core::marker::Copy for OLIOPRWAVEFORMAT {}
impl ::core::clone::Clone for OLIOPRWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OLIOPRWAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLIOPRWAVEFORMAT")
            .field("wfx", &self.wfx)
            .finish()
    }
}
impl ::core::cmp::PartialEq for OLIOPRWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx
    }
}
impl ::core::cmp::Eq for OLIOPRWAVEFORMAT {}
impl FromIntoMemory for OLIOPRWAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 18);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        Self { wfx: f_wfx }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 18);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
    }
    fn size() -> usize {
        18
    }
}
pub struct OLISBCWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
impl ::core::marker::Copy for OLISBCWAVEFORMAT {}
impl ::core::clone::Clone for OLISBCWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OLISBCWAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLISBCWAVEFORMAT")
            .field("wfx", &self.wfx)
            .finish()
    }
}
impl ::core::cmp::PartialEq for OLISBCWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx
    }
}
impl ::core::cmp::Eq for OLISBCWAVEFORMAT {}
impl FromIntoMemory for OLISBCWAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 18);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        Self { wfx: f_wfx }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 18);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
    }
    fn size() -> usize {
        18
    }
}
pub const PD_CAN_DRAW_DIB: u32 = 1u32;
pub const PD_CAN_STRETCHDIB: u32 = 2u32;
pub const PD_STRETCHDIB_1_1_OK: u32 = 4u32;
pub const PD_STRETCHDIB_1_2_OK: u32 = 8u32;
pub const PD_STRETCHDIB_1_N_OK: u32 = 16u32;
pub const ROCKWELL_WA1_MIXER: u32 = 103u32;
pub const ROCKWELL_WA1_MPU401_IN: u32 = 104u32;
pub const ROCKWELL_WA1_MPU401_OUT: u32 = 105u32;
pub const ROCKWELL_WA1_SYNTH: u32 = 102u32;
pub const ROCKWELL_WA1_WAVEIN: u32 = 100u32;
pub const ROCKWELL_WA1_WAVEOUT: u32 = 101u32;
pub const ROCKWELL_WA2_MIXER: u32 = 203u32;
pub const ROCKWELL_WA2_MPU401_IN: u32 = 204u32;
pub const ROCKWELL_WA2_MPU401_OUT: u32 = 205u32;
pub const ROCKWELL_WA2_SYNTH: u32 = 202u32;
pub const ROCKWELL_WA2_WAVEIN: u32 = 200u32;
pub const ROCKWELL_WA2_WAVEOUT: u32 = 201u32;
pub const SEARCH_ANY: i32 = 32i32;
pub const SEARCH_BACKWARD: i32 = 4i32;
pub const SEARCH_FORWARD: i32 = 1i32;
pub const SEARCH_KEY: i32 = 16i32;
pub const SEARCH_NEAREST: i32 = 4i32;
pub const SEEK_CUR: u32 = 1u32;
pub const SEEK_END: u32 = 2u32;
pub const SEEK_SET: u32 = 0u32;
pub struct SIERRAADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wRevision: u16,
}
impl ::core::marker::Copy for SIERRAADPCMWAVEFORMAT {}
impl ::core::clone::Clone for SIERRAADPCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SIERRAADPCMWAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SIERRAADPCMWAVEFORMAT")
            .field("wfx", &self.wfx)
            .field("wRevision", &self.wRevision)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SIERRAADPCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx && self.wRevision == other.wRevision
    }
}
impl ::core::cmp::Eq for SIERRAADPCMWAVEFORMAT {}
impl FromIntoMemory for SIERRAADPCMWAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        let f_wRevision = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        Self {
            wfx: f_wfx,
            wRevision: f_wRevision,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
        FromIntoMemory::into_bytes(self.wRevision, &mut into[18..18 + 2]);
    }
    fn size() -> usize {
        20
    }
}
pub struct SONARCWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wCompType: u16,
}
impl ::core::marker::Copy for SONARCWAVEFORMAT {}
impl ::core::clone::Clone for SONARCWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SONARCWAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SONARCWAVEFORMAT")
            .field("wfx", &self.wfx)
            .field("wCompType", &self.wCompType)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SONARCWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx && self.wCompType == other.wCompType
    }
}
impl ::core::cmp::Eq for SONARCWAVEFORMAT {}
impl FromIntoMemory for SONARCWAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        let f_wCompType = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        Self {
            wfx: f_wfx,
            wCompType: f_wCompType,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
        FromIntoMemory::into_bytes(self.wCompType, &mut into[18..18 + 2]);
    }
    fn size() -> usize {
        20
    }
}
pub const TARGET_DEVICE_FRIENDLY_NAME: &'static str = "TargetDeviceFriendlyName";
pub const TARGET_DEVICE_OPEN_EXCLUSIVELY: &'static str = "TargetDeviceOpenExclusively";
pub const TASKERR_NOTASKSUPPORT: u32 = 1u32;
pub const TASKERR_OUTOFMEMORY: u32 = 2u32;
pub const TDD_BEGINMINPERIOD: u32 = 2064u32;
pub const TDD_ENDMINPERIOD: u32 = 2068u32;
pub const TDD_GETDEVCAPS: u32 = 2060u32;
pub const TDD_GETSYSTEMTIME: u32 = 2056u32;
pub const TDD_KILLTIMEREVENT: u32 = 2048u32;
pub const TDD_SETTIMEREVENT: u32 = 2052u32;
pub struct TIMEREVENT {
    pub wDelay: u16,
    pub wResolution: u16,
    pub lpFunction: super::LPTIMECALLBACK,
    pub dwUser: u32,
    pub wFlags: u16,
    pub wReserved1: u16,
}
impl ::core::marker::Copy for TIMEREVENT {}
impl ::core::clone::Clone for TIMEREVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TIMEREVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TIMEREVENT")
            .field("wDelay", &self.wDelay)
            .field("wResolution", &self.wResolution)
            .field("lpFunction", &self.lpFunction)
            .field("dwUser", &self.dwUser)
            .field("wFlags", &self.wFlags)
            .field("wReserved1", &self.wReserved1)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TIMEREVENT {
    fn eq(&self, other: &Self) -> bool {
        self.wDelay == other.wDelay
            && self.wResolution == other.wResolution
            && self.lpFunction == other.lpFunction
            && self.dwUser == other.dwUser
            && self.wFlags == other.wFlags
            && self.wReserved1 == other.wReserved1
    }
}
impl ::core::cmp::Eq for TIMEREVENT {}
impl FromIntoMemory for TIMEREVENT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_wDelay = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wResolution = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_lpFunction = <super::LPTIMECALLBACK as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwUser = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_wFlags = <u16 as FromIntoMemory>::from_bytes(&from[12..12 + 2]);
        let f_wReserved1 = <u16 as FromIntoMemory>::from_bytes(&from[14..14 + 2]);
        Self {
            wDelay: f_wDelay,
            wResolution: f_wResolution,
            lpFunction: f_lpFunction,
            dwUser: f_dwUser,
            wFlags: f_wFlags,
            wReserved1: f_wReserved1,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.wDelay, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wResolution, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.lpFunction, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwUser, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.wFlags, &mut into[12..12 + 2]);
        FromIntoMemory::into_bytes(self.wReserved1, &mut into[14..14 + 2]);
    }
    fn size() -> usize {
        16
    }
}
pub struct TRUESPEECHWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wRevision: u16,
    pub nSamplesPerBlock: u16,
    pub abReserved: [u8; 28],
}
impl ::core::marker::Copy for TRUESPEECHWAVEFORMAT {}
impl ::core::clone::Clone for TRUESPEECHWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRUESPEECHWAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRUESPEECHWAVEFORMAT")
            .field("wfx", &self.wfx)
            .field("wRevision", &self.wRevision)
            .field("nSamplesPerBlock", &self.nSamplesPerBlock)
            .field("abReserved", &self.abReserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TRUESPEECHWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx
            && self.wRevision == other.wRevision
            && self.nSamplesPerBlock == other.nSamplesPerBlock
            && self.abReserved == other.abReserved
    }
}
impl ::core::cmp::Eq for TRUESPEECHWAVEFORMAT {}
impl FromIntoMemory for TRUESPEECHWAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 50);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        let f_wRevision = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        let f_nSamplesPerBlock = <u16 as FromIntoMemory>::from_bytes(&from[20..20 + 2]);
        let f_abReserved = <[u8; 28] as FromIntoMemory>::from_bytes(&from[22..22 + 28]);
        Self {
            wfx: f_wfx,
            wRevision: f_wRevision,
            nSamplesPerBlock: f_nSamplesPerBlock,
            abReserved: f_abReserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 50);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
        FromIntoMemory::into_bytes(self.wRevision, &mut into[18..18 + 2]);
        FromIntoMemory::into_bytes(self.nSamplesPerBlock, &mut into[20..20 + 2]);
        FromIntoMemory::into_bytes(self.abReserved, &mut into[22..22 + 28]);
    }
    fn size() -> usize {
        50
    }
}
pub const VADMAD_Device_ID: u32 = 1092u32;
pub const VCAPS_CAN_SCALE: u32 = 8u32;
pub const VCAPS_DST_CAN_CLIP: u32 = 4u32;
pub const VCAPS_OVERLAY: u32 = 1u32;
pub const VCAPS_SRC_CAN_CLIP: u32 = 2u32;
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.UI.Controls'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub type VFWWDMExtensionProc = StdCallFnPtr<
    (
        MutPtr<::core::ffi::c_void>,
        super::super::UI::Controls::LPFNSVADDPROPSHEETPAGE,
        super::super::Foundation::LPARAM,
    ),
    u32,
>;
pub const VFW_HIDE_CAMERACONTROL_PAGE: u32 = 4u32;
pub const VFW_HIDE_SETTINGS_PAGE: u32 = 1u32;
pub const VFW_HIDE_VIDEOSRC_PAGE: u32 = 2u32;
pub const VFW_OEM_ADD_PAGE: u32 = 2147483648u32;
pub const VFW_QUERY_DEV_CHANGED: u32 = 256u32;
pub const VFW_USE_DEVICE_HANDLE: u32 = 1u32;
pub const VFW_USE_STREAM_HANDLE: u32 = 2u32;
pub const VHDR_DONE: u32 = 1u32;
pub const VHDR_INQUEUE: u32 = 4u32;
pub const VHDR_KEYFRAME: u32 = 8u32;
pub const VHDR_PREPARED: u32 = 2u32;
pub const VHDR_VALID: u32 = 15u32;
pub const VIDCF_COMPRESSFRAMES: u32 = 8u32;
pub const VIDCF_CRUNCH: u32 = 2u32;
pub const VIDCF_DRAW: u32 = 16u32;
pub const VIDCF_FASTTEMPORALC: u32 = 32u32;
pub const VIDCF_FASTTEMPORALD: u32 = 128u32;
pub const VIDCF_QUALITY: u32 = 1u32;
pub const VIDCF_TEMPORAL: u32 = 4u32;
pub struct VIDEOHDR {
    pub lpData: MutPtr<u8>,
    pub dwBufferLength: u32,
    pub dwBytesUsed: u32,
    pub dwTimeCaptured: u32,
    pub dwUser: PtrRepr,
    pub dwFlags: u32,
    pub dwReserved: [PtrRepr; 4],
}
impl ::core::marker::Copy for VIDEOHDR {}
impl ::core::clone::Clone for VIDEOHDR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VIDEOHDR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEOHDR")
            .field("lpData", &self.lpData)
            .field("dwBufferLength", &self.dwBufferLength)
            .field("dwBytesUsed", &self.dwBytesUsed)
            .field("dwTimeCaptured", &self.dwTimeCaptured)
            .field("dwUser", &self.dwUser)
            .field("dwFlags", &self.dwFlags)
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VIDEOHDR {
    fn eq(&self, other: &Self) -> bool {
        self.lpData == other.lpData
            && self.dwBufferLength == other.dwBufferLength
            && self.dwBytesUsed == other.dwBytesUsed
            && self.dwTimeCaptured == other.dwTimeCaptured
            && self.dwUser == other.dwUser
            && self.dwFlags == other.dwFlags
            && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for VIDEOHDR {}
impl FromIntoMemory for VIDEOHDR {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_lpData = <MutPtr<u8> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwBufferLength = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwBytesUsed = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwTimeCaptured = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwUser = <PtrRepr as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwReserved = <[PtrRepr; 4] as FromIntoMemory>::from_bytes(&from[24..24 + 16]);
        Self {
            lpData: f_lpData,
            dwBufferLength: f_dwBufferLength,
            dwBytesUsed: f_dwBytesUsed,
            dwTimeCaptured: f_dwTimeCaptured,
            dwUser: f_dwUser,
            dwFlags: f_dwFlags,
            dwReserved: f_dwReserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.lpData, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwBufferLength, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwBytesUsed, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwTimeCaptured, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwUser, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwReserved, &mut into[24..24 + 16]);
    }
    fn size() -> usize {
        40
    }
}
pub const VIDEO_CONFIGURE_CURRENT: u32 = 16u32;
pub const VIDEO_CONFIGURE_GET: u32 = 8192u32;
pub const VIDEO_CONFIGURE_MAX: u32 = 128u32;
pub const VIDEO_CONFIGURE_MIN: u32 = 64u32;
pub const VIDEO_CONFIGURE_NOMINAL: u32 = 32u32;
pub const VIDEO_CONFIGURE_QUERY: u32 = 32768u32;
pub const VIDEO_CONFIGURE_QUERYSIZE: u32 = 1u32;
pub const VIDEO_CONFIGURE_SET: u32 = 4096u32;
pub const VIDEO_DLG_QUERY: u32 = 16u32;
pub const VIDEO_EXTERNALIN: u32 = 1u32;
pub const VIDEO_EXTERNALOUT: u32 = 2u32;
pub const VIDEO_IN: u32 = 4u32;
pub const VIDEO_OUT: u32 = 8u32;
pub const VP_COMMAND_GET: u32 = 1u32;
pub const VP_COMMAND_SET: u32 = 2u32;
pub const VP_CP_CMD_ACTIVATE: u32 = 1u32;
pub const VP_CP_CMD_CHANGE: u32 = 4u32;
pub const VP_CP_CMD_DEACTIVATE: u32 = 2u32;
pub const VP_CP_TYPE_APS_TRIGGER: u32 = 1u32;
pub const VP_CP_TYPE_MACROVISION: u32 = 2u32;
pub const VP_FLAGS_BRIGHTNESS: u32 = 64u32;
pub const VP_FLAGS_CONTRAST: u32 = 128u32;
pub const VP_FLAGS_COPYPROTECT: u32 = 256u32;
pub const VP_FLAGS_FLICKER: u32 = 4u32;
pub const VP_FLAGS_MAX_UNSCALED: u32 = 16u32;
pub const VP_FLAGS_OVERSCAN: u32 = 8u32;
pub const VP_FLAGS_POSITION: u32 = 32u32;
pub const VP_FLAGS_TV_MODE: u32 = 1u32;
pub const VP_FLAGS_TV_STANDARD: u32 = 2u32;
pub const VP_MODE_TV_PLAYBACK: u32 = 2u32;
pub const VP_MODE_WIN_GRAPHICS: u32 = 1u32;
pub const VP_TV_STANDARD_NTSC_433: u32 = 65536u32;
pub const VP_TV_STANDARD_NTSC_M: u32 = 1u32;
pub const VP_TV_STANDARD_NTSC_M_J: u32 = 2u32;
pub const VP_TV_STANDARD_PAL_60: u32 = 262144u32;
pub const VP_TV_STANDARD_PAL_B: u32 = 4u32;
pub const VP_TV_STANDARD_PAL_D: u32 = 8u32;
pub const VP_TV_STANDARD_PAL_G: u32 = 131072u32;
pub const VP_TV_STANDARD_PAL_H: u32 = 16u32;
pub const VP_TV_STANDARD_PAL_I: u32 = 32u32;
pub const VP_TV_STANDARD_PAL_M: u32 = 64u32;
pub const VP_TV_STANDARD_PAL_N: u32 = 128u32;
pub const VP_TV_STANDARD_SECAM_B: u32 = 256u32;
pub const VP_TV_STANDARD_SECAM_D: u32 = 512u32;
pub const VP_TV_STANDARD_SECAM_G: u32 = 1024u32;
pub const VP_TV_STANDARD_SECAM_H: u32 = 2048u32;
pub const VP_TV_STANDARD_SECAM_K: u32 = 4096u32;
pub const VP_TV_STANDARD_SECAM_K1: u32 = 8192u32;
pub const VP_TV_STANDARD_SECAM_L: u32 = 16384u32;
pub const VP_TV_STANDARD_SECAM_L1: u32 = 524288u32;
pub const VP_TV_STANDARD_WIN_VGA: u32 = 32768u32;
pub struct WAVEOPENDESC {
    pub hWave: super::Audio::HWAVE,
    pub lpFormat: MutPtr<super::Audio::WAVEFORMAT>,
    pub dwCallback: PtrRepr,
    pub dwInstance: PtrRepr,
    pub uMappedDeviceID: u32,
    pub dnDevNode: PtrRepr,
}
impl ::core::marker::Copy for WAVEOPENDESC {}
impl ::core::clone::Clone for WAVEOPENDESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WAVEOPENDESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WAVEOPENDESC")
            .field("hWave", &self.hWave)
            .field("lpFormat", &self.lpFormat)
            .field("dwCallback", &self.dwCallback)
            .field("dwInstance", &self.dwInstance)
            .field("uMappedDeviceID", &self.uMappedDeviceID)
            .field("dnDevNode", &self.dnDevNode)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WAVEOPENDESC {
    fn eq(&self, other: &Self) -> bool {
        self.hWave == other.hWave
            && self.lpFormat == other.lpFormat
            && self.dwCallback == other.dwCallback
            && self.dwInstance == other.dwInstance
            && self.uMappedDeviceID == other.uMappedDeviceID
            && self.dnDevNode == other.dnDevNode
    }
}
impl ::core::cmp::Eq for WAVEOPENDESC {}
impl FromIntoMemory for WAVEOPENDESC {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_hWave = <super::Audio::HWAVE as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpFormat =
            <MutPtr<super::Audio::WAVEFORMAT> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwInstance = <PtrRepr as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_uMappedDeviceID = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dnDevNode = <PtrRepr as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            hWave: f_hWave,
            lpFormat: f_lpFormat,
            dwCallback: f_dwCallback,
            dwInstance: f_dwInstance,
            uMappedDeviceID: f_uMappedDeviceID,
            dnDevNode: f_dnDevNode,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.hWave, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpFormat, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwInstance, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.uMappedDeviceID, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dnDevNode, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub const WAVE_FILTER_DEVELOPMENT: u32 = 65535u32;
pub const WAVE_FILTER_ECHO: u32 = 2u32;
pub const WAVE_FILTER_UNKNOWN: u32 = 0u32;
pub const WAVE_FILTER_VOLUME: u32 = 1u32;
pub const WAVE_FORMAT_3COM_NBX: u32 = 28672u32;
pub const WAVE_FORMAT_ADPCM: u32 = 2u32;
pub const WAVE_FORMAT_ALAC: u32 = 27745u32;
pub const WAVE_FORMAT_ALAW: u32 = 6u32;
pub const WAVE_FORMAT_AMR_NB: u32 = 29537u32;
pub const WAVE_FORMAT_AMR_WB: u32 = 29538u32;
pub const WAVE_FORMAT_AMR_WP: u32 = 29539u32;
pub const WAVE_FORMAT_ANTEX_ADPCME: u32 = 51u32;
pub const WAVE_FORMAT_APTX: u32 = 37u32;
pub const WAVE_FORMAT_AUDIOFILE_AF10: u32 = 38u32;
pub const WAVE_FORMAT_AUDIOFILE_AF36: u32 = 36u32;
pub const WAVE_FORMAT_BTV_DIGITAL: u32 = 1024u32;
pub const WAVE_FORMAT_CANOPUS_ATRAC: u32 = 99u32;
pub const WAVE_FORMAT_CIRRUS: u32 = 96u32;
pub const WAVE_FORMAT_CODIAN: u32 = 41252u32;
pub const WAVE_FORMAT_COMVERSE_INFOSYS_AVQSBC: u32 = 41217u32;
pub const WAVE_FORMAT_COMVERSE_INFOSYS_G723_1: u32 = 41216u32;
pub const WAVE_FORMAT_COMVERSE_INFOSYS_SBC: u32 = 41218u32;
pub const WAVE_FORMAT_CONGRUENCY: u32 = 141u32;
pub const WAVE_FORMAT_CONTROL_RES_CR10: u32 = 55u32;
pub const WAVE_FORMAT_CONTROL_RES_VQLPC: u32 = 52u32;
pub const WAVE_FORMAT_CONVEDIA_G729: u32 = 140u32;
pub const WAVE_FORMAT_CREATIVE_ADPCM: u32 = 512u32;
pub const WAVE_FORMAT_CREATIVE_FASTSPEECH10: u32 = 515u32;
pub const WAVE_FORMAT_CREATIVE_FASTSPEECH8: u32 = 514u32;
pub const WAVE_FORMAT_CS2: u32 = 608u32;
pub const WAVE_FORMAT_CS_IMAADPCM: u32 = 57u32;
pub const WAVE_FORMAT_CUSEEME: u32 = 7939u32;
pub const WAVE_FORMAT_CU_CODEC: u32 = 25u32;
pub const WAVE_FORMAT_DEVELOPMENT: u32 = 65535u32;
pub const WAVE_FORMAT_DF_G726: u32 = 133u32;
pub const WAVE_FORMAT_DF_GSM610: u32 = 134u32;
pub const WAVE_FORMAT_DIALOGIC_OKI_ADPCM: u32 = 23u32;
pub const WAVE_FORMAT_DICTAPHONE_CELP54: u32 = 322u32;
pub const WAVE_FORMAT_DICTAPHONE_CELP68: u32 = 321u32;
pub const WAVE_FORMAT_DIGIADPCM: u32 = 54u32;
pub const WAVE_FORMAT_DIGIFIX: u32 = 22u32;
pub const WAVE_FORMAT_DIGIREAL: u32 = 53u32;
pub const WAVE_FORMAT_DIGISTD: u32 = 21u32;
pub const WAVE_FORMAT_DIGITAL_G723: u32 = 291u32;
pub const WAVE_FORMAT_DIVIO_G726: u32 = 16963u32;
pub const WAVE_FORMAT_DIVIO_MPEG4_AAC: u32 = 16707u32;
pub const WAVE_FORMAT_DOLBY_AC2: u32 = 48u32;
pub const WAVE_FORMAT_DOLBY_AC3_SPDIF: u32 = 146u32;
pub const WAVE_FORMAT_DOLBY_AC4: u32 = 44096u32;
pub const WAVE_FORMAT_DRM: u32 = 9u32;
pub const WAVE_FORMAT_DSAT: u32 = 102u32;
pub const WAVE_FORMAT_DSAT_DISPLAY: u32 = 103u32;
pub const WAVE_FORMAT_DSPGROUP_TRUESPEECH: u32 = 34u32;
pub const WAVE_FORMAT_DTS: u32 = 8u32;
pub const WAVE_FORMAT_DTS2: u32 = 8193u32;
pub const WAVE_FORMAT_DTS_DS: u32 = 400u32;
pub const WAVE_FORMAT_DVI_ADPCM: u32 = 17u32;
pub const WAVE_FORMAT_DVM: u32 = 8192u32;
pub const WAVE_FORMAT_ECHOSC1: u32 = 35u32;
pub const WAVE_FORMAT_ECHOSC3: u32 = 58u32;
pub const WAVE_FORMAT_ENCORE_G726: u32 = 41223u32;
pub const WAVE_FORMAT_ESPCM: u32 = 97u32;
pub const WAVE_FORMAT_ESST_AC3: u32 = 577u32;
pub const WAVE_FORMAT_FAAD_AAC: u32 = 28781u32;
pub const WAVE_FORMAT_FLAC: u32 = 61868u32;
pub const WAVE_FORMAT_FM_TOWNS_SND: u32 = 768u32;
pub const WAVE_FORMAT_FRACE_TELECOM_G729: u32 = 41251u32;
pub const WAVE_FORMAT_FRAUNHOFER_IIS_MPEG2_AAC: u32 = 384u32;
pub const WAVE_FORMAT_G721_ADPCM: u32 = 64u32;
pub const WAVE_FORMAT_G722_ADPCM: u32 = 101u32;
pub const WAVE_FORMAT_G723_ADPCM: u32 = 20u32;
pub const WAVE_FORMAT_G726ADPCM: u32 = 320u32;
pub const WAVE_FORMAT_G726_ADPCM: u32 = 100u32;
pub const WAVE_FORMAT_G728_CELP: u32 = 65u32;
pub const WAVE_FORMAT_G729A: u32 = 131u32;
pub const WAVE_FORMAT_GENERIC_PASSTHRU: u32 = 585u32;
pub const WAVE_FORMAT_GLOBAL_IP_ILBC: u32 = 41238u32;
pub const WAVE_FORMAT_GSM610: u32 = 49u32;
pub const WAVE_FORMAT_GSM_610: u32 = 41229u32;
pub const WAVE_FORMAT_GSM_620: u32 = 41230u32;
pub const WAVE_FORMAT_GSM_660: u32 = 41231u32;
pub const WAVE_FORMAT_GSM_690: u32 = 41232u32;
pub const WAVE_FORMAT_GSM_ADAPTIVE_MULTIRATE_WB: u32 = 41233u32;
pub const WAVE_FORMAT_GSM_AMR_CBR: u32 = 31265u32;
pub const WAVE_FORMAT_GSM_AMR_VBR_SID: u32 = 31266u32;
pub const WAVE_FORMAT_HP_DYN_VOICE: u32 = 26u32;
pub const WAVE_FORMAT_IBM_CVSD: u32 = 5u32;
pub const WAVE_FORMAT_IEEE_FLOAT: u32 = 3u32;
pub const WAVE_FORMAT_ILINK_VC: u32 = 560u32;
pub const WAVE_FORMAT_IMA_ADPCM: u32 = 17u32;
pub const WAVE_FORMAT_INDEO_AUDIO: u32 = 1026u32;
pub const WAVE_FORMAT_INFOCOM_ITS_G721_ADPCM: u32 = 139u32;
pub const WAVE_FORMAT_INGENIENT_G726: u32 = 41221u32;
pub const WAVE_FORMAT_INNINGS_TELECOM_ADPCM: u32 = 6521u32;
pub const WAVE_FORMAT_INTEL_G723_1: u32 = 67u32;
pub const WAVE_FORMAT_INTEL_G729: u32 = 68u32;
pub const WAVE_FORMAT_INTEL_MUSIC_CODER: u32 = 1025u32;
pub const WAVE_FORMAT_IPI_HSX: u32 = 592u32;
pub const WAVE_FORMAT_IPI_RPELP: u32 = 593u32;
pub const WAVE_FORMAT_IRAT: u32 = 257u32;
pub const WAVE_FORMAT_ISIAUDIO: u32 = 136u32;
pub const WAVE_FORMAT_ISIAUDIO_2: u32 = 5121u32;
pub const WAVE_FORMAT_KNOWLEDGE_ADVENTURE_ADPCM: u32 = 376u32;
pub const WAVE_FORMAT_LEAD_SPEECH: u32 = 17228u32;
pub const WAVE_FORMAT_LEAD_VORBIS: u32 = 22092u32;
pub const WAVE_FORMAT_LH_CODEC: u32 = 4352u32;
pub const WAVE_FORMAT_LH_CODEC_CELP: u32 = 4353u32;
pub const WAVE_FORMAT_LH_CODEC_SBC12: u32 = 4355u32;
pub const WAVE_FORMAT_LH_CODEC_SBC16: u32 = 4356u32;
pub const WAVE_FORMAT_LH_CODEC_SBC8: u32 = 4354u32;
pub const WAVE_FORMAT_LIGHTWAVE_LOSSLESS: u32 = 2222u32;
pub const WAVE_FORMAT_LRC: u32 = 40u32;
pub const WAVE_FORMAT_LUCENT_G723: u32 = 89u32;
pub const WAVE_FORMAT_LUCENT_SX5363S: u32 = 7180u32;
pub const WAVE_FORMAT_LUCENT_SX8300P: u32 = 7175u32;
pub const WAVE_FORMAT_MAKEAVIS: u32 = 13075u32;
pub const WAVE_FORMAT_MALDEN_PHONYTALK: u32 = 160u32;
pub const WAVE_FORMAT_MEDIASONIC_G723: u32 = 147u32;
pub const WAVE_FORMAT_MEDIASPACE_ADPCM: u32 = 18u32;
pub const WAVE_FORMAT_MEDIAVISION_ADPCM: u32 = 24u32;
pub const WAVE_FORMAT_MICRONAS: u32 = 848u32;
pub const WAVE_FORMAT_MICRONAS_CELP833: u32 = 849u32;
pub const WAVE_FORMAT_MPEG: u32 = 80u32;
pub const WAVE_FORMAT_MPEG4_AAC: u32 = 41222u32;
pub const WAVE_FORMAT_MPEGLAYER3: u32 = 85u32;
pub const WAVE_FORMAT_MPEG_ADTS_AAC: u32 = 5632u32;
pub const WAVE_FORMAT_MPEG_HEAAC: u32 = 5648u32;
pub const WAVE_FORMAT_MPEG_LOAS: u32 = 5634u32;
pub const WAVE_FORMAT_MPEG_RAW_AAC: u32 = 5633u32;
pub const WAVE_FORMAT_MSAUDIO1: u32 = 352u32;
pub const WAVE_FORMAT_MSG723: u32 = 66u32;
pub const WAVE_FORMAT_MSNAUDIO: u32 = 50u32;
pub const WAVE_FORMAT_MSRT24: u32 = 130u32;
pub const WAVE_FORMAT_MULAW: u32 = 7u32;
pub const WAVE_FORMAT_MULTITUDE_FT_SX20: u32 = 138u32;
pub const WAVE_FORMAT_MVI_MVI2: u32 = 132u32;
pub const WAVE_FORMAT_NEC_AAC: u32 = 176u32;
pub const WAVE_FORMAT_NICE_ACA: u32 = 41240u32;
pub const WAVE_FORMAT_NICE_ADPCM: u32 = 41241u32;
pub const WAVE_FORMAT_NICE_G728: u32 = 41250u32;
pub const WAVE_FORMAT_NMS_VBXADPCM: u32 = 56u32;
pub const WAVE_FORMAT_NOKIA_ADAPTIVE_MULTIRATE: u32 = 16897u32;
pub const WAVE_FORMAT_NOKIA_MPEG_ADTS_AAC: u32 = 5640u32;
pub const WAVE_FORMAT_NOKIA_MPEG_RAW_AAC: u32 = 5641u32;
pub const WAVE_FORMAT_NORCOM_VOICE_SYSTEMS_ADPCM: u32 = 645u32;
pub const WAVE_FORMAT_NORRIS: u32 = 5120u32;
pub const WAVE_FORMAT_NTCSOFT_ALF2CM_ACM: u32 = 8132u32;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_1: u32 = 26447u32;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_1_PLUS: u32 = 26479u32;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_2: u32 = 26448u32;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_2_PLUS: u32 = 26480u32;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_3: u32 = 26449u32;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_3_PLUS: u32 = 26481u32;
pub const WAVE_FORMAT_OKI_ADPCM: u32 = 16u32;
pub const WAVE_FORMAT_OLIADPCM: u32 = 4097u32;
pub const WAVE_FORMAT_OLICELP: u32 = 4098u32;
pub const WAVE_FORMAT_OLIGSM: u32 = 4096u32;
pub const WAVE_FORMAT_OLIOPR: u32 = 4100u32;
pub const WAVE_FORMAT_OLISBC: u32 = 4099u32;
pub const WAVE_FORMAT_ON2_VP6_AUDIO: u32 = 1281u32;
pub const WAVE_FORMAT_ON2_VP7_AUDIO: u32 = 1280u32;
pub const WAVE_FORMAT_ONLIVE: u32 = 137u32;
pub const WAVE_FORMAT_OPUS: u32 = 28751u32;
pub const WAVE_FORMAT_PAC: u32 = 83u32;
pub const WAVE_FORMAT_PACKED: u32 = 153u32;
pub const WAVE_FORMAT_PCM_S: u32 = 1152u32;
pub const WAVE_FORMAT_PHILIPS_CELP: u32 = 288u32;
pub const WAVE_FORMAT_PHILIPS_GRUNDIG: u32 = 289u32;
pub const WAVE_FORMAT_PHILIPS_LPCBB: u32 = 152u32;
pub const WAVE_FORMAT_POLYCOM_G722: u32 = 41234u32;
pub const WAVE_FORMAT_POLYCOM_G728: u32 = 41235u32;
pub const WAVE_FORMAT_POLYCOM_G729_A: u32 = 41236u32;
pub const WAVE_FORMAT_POLYCOM_SIREN: u32 = 41237u32;
pub const WAVE_FORMAT_PROSODY_1612: u32 = 39u32;
pub const WAVE_FORMAT_PROSODY_8KBPS: u32 = 148u32;
pub const WAVE_FORMAT_QDESIGN_MUSIC: u32 = 1104u32;
pub const WAVE_FORMAT_QUALCOMM_HALFRATE: u32 = 337u32;
pub const WAVE_FORMAT_QUALCOMM_PUREVOICE: u32 = 336u32;
pub const WAVE_FORMAT_QUARTERDECK: u32 = 544u32;
pub const WAVE_FORMAT_RACAL_RECORDER_G720_A: u32 = 162u32;
pub const WAVE_FORMAT_RACAL_RECORDER_G723_1: u32 = 163u32;
pub const WAVE_FORMAT_RACAL_RECORDER_GSM: u32 = 161u32;
pub const WAVE_FORMAT_RACAL_RECORDER_TETRA_ACELP: u32 = 164u32;
pub const WAVE_FORMAT_RADIOTIME_TIME_SHIFT_RADIO: u32 = 41239u32;
pub const WAVE_FORMAT_RAW_AAC1: u32 = 255u32;
pub const WAVE_FORMAT_RAW_SPORT: u32 = 576u32;
pub const WAVE_FORMAT_RHETOREX_ADPCM: u32 = 256u32;
pub const WAVE_FORMAT_ROCKWELL_ADPCM: u32 = 59u32;
pub const WAVE_FORMAT_ROCKWELL_DIGITALK: u32 = 60u32;
pub const WAVE_FORMAT_RT24: u32 = 82u32;
pub const WAVE_FORMAT_SANYO_LD_ADPCM: u32 = 293u32;
pub const WAVE_FORMAT_SBC24: u32 = 145u32;
pub const WAVE_FORMAT_SHARP_G726: u32 = 69u32;
pub const WAVE_FORMAT_SIERRA_ADPCM: u32 = 19u32;
pub const WAVE_FORMAT_SIPROLAB_ACELP4800: u32 = 305u32;
pub const WAVE_FORMAT_SIPROLAB_ACELP8V3: u32 = 306u32;
pub const WAVE_FORMAT_SIPROLAB_ACEPLNET: u32 = 304u32;
pub const WAVE_FORMAT_SIPROLAB_G729: u32 = 307u32;
pub const WAVE_FORMAT_SIPROLAB_G729A: u32 = 308u32;
pub const WAVE_FORMAT_SIPROLAB_KELVIN: u32 = 309u32;
pub const WAVE_FORMAT_SOFTSOUND: u32 = 128u32;
pub const WAVE_FORMAT_SONARC: u32 = 33u32;
pub const WAVE_FORMAT_SONICFOUNDRY_LOSSLESS: u32 = 6513u32;
pub const WAVE_FORMAT_SONY_ATRAC3: u32 = 626u32;
pub const WAVE_FORMAT_SONY_SCX: u32 = 624u32;
pub const WAVE_FORMAT_SONY_SCY: u32 = 625u32;
pub const WAVE_FORMAT_SONY_SPC: u32 = 627u32;
pub const WAVE_FORMAT_SOUNDSPACE_MUSICOMPRESS: u32 = 5376u32;
pub const WAVE_FORMAT_SPEEX_VOICE: u32 = 41225u32;
pub const WAVE_FORMAT_SYCOM_ACM_SYC008: u32 = 372u32;
pub const WAVE_FORMAT_SYCOM_ACM_SYC701_CELP54: u32 = 374u32;
pub const WAVE_FORMAT_SYCOM_ACM_SYC701_CELP68: u32 = 375u32;
pub const WAVE_FORMAT_SYCOM_ACM_SYC701_G726L: u32 = 373u32;
pub const WAVE_FORMAT_SYMBOL_G729_A: u32 = 41219u32;
pub const WAVE_FORMAT_TELUM_AUDIO: u32 = 640u32;
pub const WAVE_FORMAT_TELUM_IA_AUDIO: u32 = 641u32;
pub const WAVE_FORMAT_TPC: u32 = 1665u32;
pub const WAVE_FORMAT_TUBGSM: u32 = 341u32;
pub const WAVE_FORMAT_UHER_ADPCM: u32 = 528u32;
pub const WAVE_FORMAT_ULEAD_DV_AUDIO: u32 = 533u32;
pub const WAVE_FORMAT_ULEAD_DV_AUDIO_1: u32 = 534u32;
pub const WAVE_FORMAT_UNISYS_NAP_16K: u32 = 371u32;
pub const WAVE_FORMAT_UNISYS_NAP_ADPCM: u32 = 368u32;
pub const WAVE_FORMAT_UNISYS_NAP_ALAW: u32 = 370u32;
pub const WAVE_FORMAT_UNISYS_NAP_ULAW: u32 = 369u32;
pub const WAVE_FORMAT_UNKNOWN: u32 = 0u32;
pub const WAVE_FORMAT_VIANIX_MASC: u32 = 41226u32;
pub const WAVE_FORMAT_VIVO_G723: u32 = 273u32;
pub const WAVE_FORMAT_VIVO_SIREN: u32 = 274u32;
pub const WAVE_FORMAT_VME_VMPCM: u32 = 1664u32;
pub const WAVE_FORMAT_VOCORD_G721: u32 = 41242u32;
pub const WAVE_FORMAT_VOCORD_G722_1: u32 = 41244u32;
pub const WAVE_FORMAT_VOCORD_G723_1: u32 = 41248u32;
pub const WAVE_FORMAT_VOCORD_G726: u32 = 41243u32;
pub const WAVE_FORMAT_VOCORD_G728: u32 = 41245u32;
pub const WAVE_FORMAT_VOCORD_G729: u32 = 41246u32;
pub const WAVE_FORMAT_VOCORD_G729_A: u32 = 41247u32;
pub const WAVE_FORMAT_VOCORD_LBC: u32 = 41249u32;
pub const WAVE_FORMAT_VODAFONE_MPEG_ADTS_AAC: u32 = 5642u32;
pub const WAVE_FORMAT_VODAFONE_MPEG_RAW_AAC: u32 = 5643u32;
pub const WAVE_FORMAT_VOICEAGE_AMR: u32 = 310u32;
pub const WAVE_FORMAT_VOICEAGE_AMR_WB: u32 = 41220u32;
pub const WAVE_FORMAT_VOXWARE: u32 = 98u32;
pub const WAVE_FORMAT_VOXWARE_AC10: u32 = 113u32;
pub const WAVE_FORMAT_VOXWARE_AC16: u32 = 114u32;
pub const WAVE_FORMAT_VOXWARE_AC20: u32 = 115u32;
pub const WAVE_FORMAT_VOXWARE_AC8: u32 = 112u32;
pub const WAVE_FORMAT_VOXWARE_BYTE_ALIGNED: u32 = 105u32;
pub const WAVE_FORMAT_VOXWARE_RT24: u32 = 116u32;
pub const WAVE_FORMAT_VOXWARE_RT24_SPEECH: u32 = 6172u32;
pub const WAVE_FORMAT_VOXWARE_RT29: u32 = 117u32;
pub const WAVE_FORMAT_VOXWARE_RT29HW: u32 = 118u32;
pub const WAVE_FORMAT_VOXWARE_SC3: u32 = 122u32;
pub const WAVE_FORMAT_VOXWARE_SC3_1: u32 = 123u32;
pub const WAVE_FORMAT_VOXWARE_TQ40: u32 = 121u32;
pub const WAVE_FORMAT_VOXWARE_TQ60: u32 = 129u32;
pub const WAVE_FORMAT_VOXWARE_VR12: u32 = 119u32;
pub const WAVE_FORMAT_VOXWARE_VR18: u32 = 120u32;
pub const WAVE_FORMAT_VSELP: u32 = 4u32;
pub const WAVE_FORMAT_WAVPACK_AUDIO: u32 = 22358u32;
pub const WAVE_FORMAT_WM9_SPECTRUM_ANALYZER: u32 = 41227u32;
pub const WAVE_FORMAT_WMASPDIF: u32 = 356u32;
pub const WAVE_FORMAT_WMAUDIO2: u32 = 353u32;
pub const WAVE_FORMAT_WMAUDIO3: u32 = 354u32;
pub const WAVE_FORMAT_WMAUDIO_LOSSLESS: u32 = 355u32;
pub const WAVE_FORMAT_WMAVOICE10: u32 = 11u32;
pub const WAVE_FORMAT_WMAVOICE9: u32 = 10u32;
pub const WAVE_FORMAT_WMF_SPECTRUM_ANAYZER: u32 = 41228u32;
pub const WAVE_FORMAT_XEBEC: u32 = 61u32;
pub const WAVE_FORMAT_YAMAHA_ADPCM: u32 = 32u32;
pub const WAVE_FORMAT_ZOLL_ASAO: u32 = 41224u32;
pub const WAVE_FORMAT_ZYXEL_ADPCM: u32 = 151u32;
pub const WAVE_MAPPER_S: u32 = 1153u32;
pub const WIDM_ADDBUFFER: u32 = 56u32;
pub const WIDM_CLOSE: u32 = 53u32;
pub const WIDM_GETDEVCAPS: u32 = 51u32;
pub const WIDM_GETNUMDEVS: u32 = 50u32;
pub const WIDM_GETPOS: u32 = 60u32;
pub const WIDM_INIT: u32 = 100u32;
pub const WIDM_INIT_EX: u32 = 104u32;
pub const WIDM_OPEN: u32 = 52u32;
pub const WIDM_PREFERRED: u32 = 61u32;
pub const WIDM_PREPARE: u32 = 54u32;
pub const WIDM_RESET: u32 = 59u32;
pub const WIDM_START: u32 = 57u32;
pub const WIDM_STOP: u32 = 58u32;
pub const WIDM_UNPREPARE: u32 = 55u32;
pub struct WMAUDIO2WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub dwSamplesPerBlock: u32,
    pub wEncodeOptions: u16,
    pub dwSuperBlockAlign: u32,
}
impl ::core::marker::Copy for WMAUDIO2WAVEFORMAT {}
impl ::core::clone::Clone for WMAUDIO2WAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WMAUDIO2WAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMAUDIO2WAVEFORMAT")
            .field("wfx", &self.wfx)
            .field("dwSamplesPerBlock", &self.dwSamplesPerBlock)
            .field("wEncodeOptions", &self.wEncodeOptions)
            .field("dwSuperBlockAlign", &self.dwSuperBlockAlign)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WMAUDIO2WAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx
            && self.dwSamplesPerBlock == other.dwSamplesPerBlock
            && self.wEncodeOptions == other.wEncodeOptions
            && self.dwSuperBlockAlign == other.dwSuperBlockAlign
    }
}
impl ::core::cmp::Eq for WMAUDIO2WAVEFORMAT {}
impl FromIntoMemory for WMAUDIO2WAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        let f_dwSamplesPerBlock = <u32 as FromIntoMemory>::from_bytes(&from[18..18 + 4]);
        let f_wEncodeOptions = <u16 as FromIntoMemory>::from_bytes(&from[22..22 + 2]);
        let f_dwSuperBlockAlign = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        Self {
            wfx: f_wfx,
            dwSamplesPerBlock: f_dwSamplesPerBlock,
            wEncodeOptions: f_wEncodeOptions,
            dwSuperBlockAlign: f_dwSuperBlockAlign,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
        FromIntoMemory::into_bytes(self.dwSamplesPerBlock, &mut into[18..18 + 4]);
        FromIntoMemory::into_bytes(self.wEncodeOptions, &mut into[22..22 + 2]);
        FromIntoMemory::into_bytes(self.dwSuperBlockAlign, &mut into[24..24 + 4]);
    }
    fn size() -> usize {
        28
    }
}
pub const WMAUDIO2_BITS_PER_SAMPLE: u32 = 16u32;
pub const WMAUDIO2_MAX_CHANNELS: u32 = 2u32;
pub struct WMAUDIO3WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wValidBitsPerSample: u16,
    pub dwChannelMask: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub wEncodeOptions: u16,
    pub wReserved3: u16,
}
impl ::core::marker::Copy for WMAUDIO3WAVEFORMAT {}
impl ::core::clone::Clone for WMAUDIO3WAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WMAUDIO3WAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMAUDIO3WAVEFORMAT")
            .field("wfx", &self.wfx)
            .field("wValidBitsPerSample", &self.wValidBitsPerSample)
            .field("dwChannelMask", &self.dwChannelMask)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("wEncodeOptions", &self.wEncodeOptions)
            .field("wReserved3", &self.wReserved3)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WMAUDIO3WAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx
            && self.wValidBitsPerSample == other.wValidBitsPerSample
            && self.dwChannelMask == other.dwChannelMask
            && self.dwReserved1 == other.dwReserved1
            && self.dwReserved2 == other.dwReserved2
            && self.wEncodeOptions == other.wEncodeOptions
            && self.wReserved3 == other.wReserved3
    }
}
impl ::core::cmp::Eq for WMAUDIO3WAVEFORMAT {}
impl FromIntoMemory for WMAUDIO3WAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 36);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        let f_wValidBitsPerSample = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        let f_dwChannelMask = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwReserved1 = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_dwReserved2 = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_wEncodeOptions = <u16 as FromIntoMemory>::from_bytes(&from[32..32 + 2]);
        let f_wReserved3 = <u16 as FromIntoMemory>::from_bytes(&from[34..34 + 2]);
        Self {
            wfx: f_wfx,
            wValidBitsPerSample: f_wValidBitsPerSample,
            dwChannelMask: f_dwChannelMask,
            dwReserved1: f_dwReserved1,
            dwReserved2: f_dwReserved2,
            wEncodeOptions: f_wEncodeOptions,
            wReserved3: f_wReserved3,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 36);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
        FromIntoMemory::into_bytes(self.wValidBitsPerSample, &mut into[18..18 + 2]);
        FromIntoMemory::into_bytes(self.dwChannelMask, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwReserved1, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.dwReserved2, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.wEncodeOptions, &mut into[32..32 + 2]);
        FromIntoMemory::into_bytes(self.wReserved3, &mut into[34..34 + 2]);
    }
    fn size() -> usize {
        36
    }
}
pub const WMAUDIO_BITS_PER_SAMPLE: u32 = 16u32;
pub const WMAUDIO_MAX_CHANNELS: u32 = 2u32;
pub const WM_CAP_ABORT: u32 = 1093u32;
pub const WM_CAP_DLG_VIDEOCOMPRESSION: u32 = 1070u32;
pub const WM_CAP_DLG_VIDEODISPLAY: u32 = 1067u32;
pub const WM_CAP_DLG_VIDEOFORMAT: u32 = 1065u32;
pub const WM_CAP_DLG_VIDEOSOURCE: u32 = 1066u32;
pub const WM_CAP_DRIVER_CONNECT: u32 = 1034u32;
pub const WM_CAP_DRIVER_DISCONNECT: u32 = 1035u32;
pub const WM_CAP_DRIVER_GET_CAPS: u32 = 1038u32;
pub const WM_CAP_DRIVER_GET_NAME: u32 = 1136u32;
pub const WM_CAP_DRIVER_GET_NAMEA: u32 = 1036u32;
pub const WM_CAP_DRIVER_GET_NAMEW: u32 = 1136u32;
pub const WM_CAP_DRIVER_GET_VERSION: u32 = 1137u32;
pub const WM_CAP_DRIVER_GET_VERSIONA: u32 = 1037u32;
pub const WM_CAP_DRIVER_GET_VERSIONW: u32 = 1137u32;
pub const WM_CAP_EDIT_COPY: u32 = 1054u32;
pub const WM_CAP_END: u32 = 1205u32;
pub const WM_CAP_FILE_ALLOCATE: u32 = 1046u32;
pub const WM_CAP_FILE_GET_CAPTURE_FILE: u32 = 1145u32;
pub const WM_CAP_FILE_GET_CAPTURE_FILEA: u32 = 1045u32;
pub const WM_CAP_FILE_GET_CAPTURE_FILEW: u32 = 1145u32;
pub const WM_CAP_FILE_SAVEAS: u32 = 1147u32;
pub const WM_CAP_FILE_SAVEASA: u32 = 1047u32;
pub const WM_CAP_FILE_SAVEASW: u32 = 1147u32;
pub const WM_CAP_FILE_SAVEDIB: u32 = 1149u32;
pub const WM_CAP_FILE_SAVEDIBA: u32 = 1049u32;
pub const WM_CAP_FILE_SAVEDIBW: u32 = 1149u32;
pub const WM_CAP_FILE_SET_CAPTURE_FILE: u32 = 1144u32;
pub const WM_CAP_FILE_SET_CAPTURE_FILEA: u32 = 1044u32;
pub const WM_CAP_FILE_SET_CAPTURE_FILEW: u32 = 1144u32;
pub const WM_CAP_FILE_SET_INFOCHUNK: u32 = 1048u32;
pub const WM_CAP_GET_AUDIOFORMAT: u32 = 1060u32;
pub const WM_CAP_GET_CAPSTREAMPTR: u32 = 1025u32;
pub const WM_CAP_GET_MCI_DEVICE: u32 = 1191u32;
pub const WM_CAP_GET_MCI_DEVICEA: u32 = 1091u32;
pub const WM_CAP_GET_MCI_DEVICEW: u32 = 1191u32;
pub const WM_CAP_GET_SEQUENCE_SETUP: u32 = 1089u32;
pub const WM_CAP_GET_STATUS: u32 = 1078u32;
pub const WM_CAP_GET_USER_DATA: u32 = 1032u32;
pub const WM_CAP_GET_VIDEOFORMAT: u32 = 1068u32;
pub const WM_CAP_GRAB_FRAME: u32 = 1084u32;
pub const WM_CAP_GRAB_FRAME_NOSTOP: u32 = 1085u32;
pub const WM_CAP_PAL_AUTOCREATE: u32 = 1107u32;
pub const WM_CAP_PAL_MANUALCREATE: u32 = 1108u32;
pub const WM_CAP_PAL_OPEN: u32 = 1204u32;
pub const WM_CAP_PAL_OPENA: u32 = 1104u32;
pub const WM_CAP_PAL_OPENW: u32 = 1204u32;
pub const WM_CAP_PAL_PASTE: u32 = 1106u32;
pub const WM_CAP_PAL_SAVE: u32 = 1205u32;
pub const WM_CAP_PAL_SAVEA: u32 = 1105u32;
pub const WM_CAP_PAL_SAVEW: u32 = 1205u32;
pub const WM_CAP_SEQUENCE: u32 = 1086u32;
pub const WM_CAP_SEQUENCE_NOFILE: u32 = 1087u32;
pub const WM_CAP_SET_AUDIOFORMAT: u32 = 1059u32;
pub const WM_CAP_SET_CALLBACK_CAPCONTROL: u32 = 1109u32;
pub const WM_CAP_SET_CALLBACK_ERROR: u32 = 1126u32;
pub const WM_CAP_SET_CALLBACK_ERRORA: u32 = 1026u32;
pub const WM_CAP_SET_CALLBACK_ERRORW: u32 = 1126u32;
pub const WM_CAP_SET_CALLBACK_FRAME: u32 = 1029u32;
pub const WM_CAP_SET_CALLBACK_STATUS: u32 = 1127u32;
pub const WM_CAP_SET_CALLBACK_STATUSA: u32 = 1027u32;
pub const WM_CAP_SET_CALLBACK_STATUSW: u32 = 1127u32;
pub const WM_CAP_SET_CALLBACK_VIDEOSTREAM: u32 = 1030u32;
pub const WM_CAP_SET_CALLBACK_WAVESTREAM: u32 = 1031u32;
pub const WM_CAP_SET_CALLBACK_YIELD: u32 = 1028u32;
pub const WM_CAP_SET_MCI_DEVICE: u32 = 1190u32;
pub const WM_CAP_SET_MCI_DEVICEA: u32 = 1090u32;
pub const WM_CAP_SET_MCI_DEVICEW: u32 = 1190u32;
pub const WM_CAP_SET_OVERLAY: u32 = 1075u32;
pub const WM_CAP_SET_PREVIEW: u32 = 1074u32;
pub const WM_CAP_SET_PREVIEWRATE: u32 = 1076u32;
pub const WM_CAP_SET_SCALE: u32 = 1077u32;
pub const WM_CAP_SET_SCROLL: u32 = 1079u32;
pub const WM_CAP_SET_SEQUENCE_SETUP: u32 = 1088u32;
pub const WM_CAP_SET_USER_DATA: u32 = 1033u32;
pub const WM_CAP_SET_VIDEOFORMAT: u32 = 1069u32;
pub const WM_CAP_SINGLE_FRAME: u32 = 1096u32;
pub const WM_CAP_SINGLE_FRAME_CLOSE: u32 = 1095u32;
pub const WM_CAP_SINGLE_FRAME_OPEN: u32 = 1094u32;
pub const WM_CAP_START: u32 = 1024u32;
pub const WM_CAP_STOP: u32 = 1092u32;
pub const WM_CAP_UNICODE_END: u32 = 1205u32;
pub const WM_CAP_UNICODE_START: u32 = 1124u32;
pub const WODM_BREAKLOOP: u32 = 20u32;
pub const WODM_BUSY: u32 = 21u32;
pub const WODM_CLOSE: u32 = 6u32;
pub const WODM_GETDEVCAPS: u32 = 4u32;
pub const WODM_GETNUMDEVS: u32 = 3u32;
pub const WODM_GETPITCH: u32 = 14u32;
pub const WODM_GETPLAYBACKRATE: u32 = 18u32;
pub const WODM_GETPOS: u32 = 13u32;
pub const WODM_GETVOLUME: u32 = 16u32;
pub const WODM_INIT: u32 = 100u32;
pub const WODM_INIT_EX: u32 = 104u32;
pub const WODM_OPEN: u32 = 5u32;
pub const WODM_PAUSE: u32 = 10u32;
pub const WODM_PREFERRED: u32 = 21u32;
pub const WODM_PREPARE: u32 = 7u32;
pub const WODM_RESET: u32 = 12u32;
pub const WODM_RESTART: u32 = 11u32;
pub const WODM_SETPITCH: u32 = 15u32;
pub const WODM_SETPLAYBACKRATE: u32 = 19u32;
pub const WODM_SETVOLUME: u32 = 17u32;
pub const WODM_UNPREPARE: u32 = 8u32;
pub const WODM_WRITE: u32 = 9u32;
pub struct YAMAHA_ADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
impl ::core::marker::Copy for YAMAHA_ADPCMWAVEFORMAT {}
impl ::core::clone::Clone for YAMAHA_ADPCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for YAMAHA_ADPCMWAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("YAMAHA_ADPCMWAVEFORMAT")
            .field("wfx", &self.wfx)
            .finish()
    }
}
impl ::core::cmp::PartialEq for YAMAHA_ADPCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wfx == other.wfx
    }
}
impl ::core::cmp::Eq for YAMAHA_ADPCMWAVEFORMAT {}
impl FromIntoMemory for YAMAHA_ADPCMWAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 18);
        let f_wfx = <super::Audio::WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        Self { wfx: f_wfx }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 18);
        FromIntoMemory::into_bytes(self.wfx, &mut into[0..0 + 18]);
    }
    fn size() -> usize {
        18
    }
}
pub type YIELDPROC = StdCallFnPtr<(u32, u32), u32>;
pub struct s_RIFFWAVE_inst {
    pub bUnshiftedNote: u8,
    pub chFineTune: super::super::Foundation::CHAR,
    pub chGain: super::super::Foundation::CHAR,
    pub bLowNote: u8,
    pub bHighNote: u8,
    pub bLowVelocity: u8,
    pub bHighVelocity: u8,
}
impl ::core::marker::Copy for s_RIFFWAVE_inst {}
impl ::core::clone::Clone for s_RIFFWAVE_inst {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for s_RIFFWAVE_inst {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("s_RIFFWAVE_inst")
            .field("bUnshiftedNote", &self.bUnshiftedNote)
            .field("chFineTune", &self.chFineTune)
            .field("chGain", &self.chGain)
            .field("bLowNote", &self.bLowNote)
            .field("bHighNote", &self.bHighNote)
            .field("bLowVelocity", &self.bLowVelocity)
            .field("bHighVelocity", &self.bHighVelocity)
            .finish()
    }
}
impl ::core::cmp::PartialEq for s_RIFFWAVE_inst {
    fn eq(&self, other: &Self) -> bool {
        self.bUnshiftedNote == other.bUnshiftedNote
            && self.chFineTune == other.chFineTune
            && self.chGain == other.chGain
            && self.bLowNote == other.bLowNote
            && self.bHighNote == other.bHighNote
            && self.bLowVelocity == other.bLowVelocity
            && self.bHighVelocity == other.bHighVelocity
    }
}
impl ::core::cmp::Eq for s_RIFFWAVE_inst {}
impl FromIntoMemory for s_RIFFWAVE_inst {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 7);
        let f_bUnshiftedNote = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_chFineTune =
            <super::super::Foundation::CHAR as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_chGain =
            <super::super::Foundation::CHAR as FromIntoMemory>::from_bytes(&from[2..2 + 1]);
        let f_bLowNote = <u8 as FromIntoMemory>::from_bytes(&from[3..3 + 1]);
        let f_bHighNote = <u8 as FromIntoMemory>::from_bytes(&from[4..4 + 1]);
        let f_bLowVelocity = <u8 as FromIntoMemory>::from_bytes(&from[5..5 + 1]);
        let f_bHighVelocity = <u8 as FromIntoMemory>::from_bytes(&from[6..6 + 1]);
        Self {
            bUnshiftedNote: f_bUnshiftedNote,
            chFineTune: f_chFineTune,
            chGain: f_chGain,
            bLowNote: f_bLowNote,
            bHighNote: f_bHighNote,
            bLowVelocity: f_bLowVelocity,
            bHighVelocity: f_bHighVelocity,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 7);
        FromIntoMemory::into_bytes(self.bUnshiftedNote, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.chFineTune, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.chGain, &mut into[2..2 + 1]);
        FromIntoMemory::into_bytes(self.bLowNote, &mut into[3..3 + 1]);
        FromIntoMemory::into_bytes(self.bHighNote, &mut into[4..4 + 1]);
        FromIntoMemory::into_bytes(self.bLowVelocity, &mut into[5..5 + 1]);
        FromIntoMemory::into_bytes(self.bHighVelocity, &mut into[6..6 + 1]);
    }
    fn size() -> usize {
        7
    }
}
pub trait Api {
    #[doc = "CloseDriver from WINMM"]
    fn CloseDriver(
        &self,
        h_driver: HDRVR,
        l_param_1: super::super::Foundation::LPARAM,
        l_param_2: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::LRESULT {
        todo!("CloseDriver")
    }
    #[doc = "DefDriverProc from WINMM"]
    fn DefDriverProc(
        &self,
        dw_driver_identifier: PtrRepr,
        hdrvr: HDRVR,
        u_msg: u32,
        l_param_1: super::super::Foundation::LPARAM,
        l_param_2: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::LRESULT {
        todo!("DefDriverProc")
    }
    #[doc = "DriverCallback from WINMM"]
    fn DriverCallback(
        &self,
        dw_callback: PtrRepr,
        dw_flags: u32,
        h_device: HDRVR,
        dw_msg: u32,
        dw_user: PtrRepr,
        dw_param_1: PtrRepr,
        dw_param_2: PtrRepr,
    ) -> super::super::Foundation::BOOL {
        todo!("DriverCallback")
    }
    #[doc = "DrvGetModuleHandle from WINMM"]
    fn DrvGetModuleHandle(&self, h_driver: HDRVR) -> super::super::Foundation::HINSTANCE {
        todo!("DrvGetModuleHandle")
    }
    #[doc = "GetDriverModuleHandle from WINMM"]
    fn GetDriverModuleHandle(&self, h_driver: HDRVR) -> super::super::Foundation::HINSTANCE {
        todo!("GetDriverModuleHandle")
    }
    #[doc = "OpenDriver from WINMM"]
    fn OpenDriver(
        &self,
        sz_driver_name: PCWSTR,
        sz_section_name: PCWSTR,
        l_param_2: super::super::Foundation::LPARAM,
    ) -> HDRVR {
        todo!("OpenDriver")
    }
    #[doc = "SendDriverMessage from WINMM"]
    fn SendDriverMessage(
        &self,
        h_driver: HDRVR,
        message: u32,
        l_param_1: super::super::Foundation::LPARAM,
        l_param_2: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::LRESULT {
        todo!("SendDriverMessage")
    }
    #[doc = "joyGetDevCapsA from WINMM"]
    fn joyGetDevCapsA(&self, u_joy_id: PtrRepr, pjc: MutPtr<JOYCAPSA>, cbjc: u32) -> u32 {
        todo!("joyGetDevCapsA")
    }
    #[doc = "joyGetDevCapsW from WINMM"]
    fn joyGetDevCapsW(&self, u_joy_id: PtrRepr, pjc: MutPtr<JOYCAPSW>, cbjc: u32) -> u32 {
        todo!("joyGetDevCapsW")
    }
    #[doc = "joyGetNumDevs from WINMM"]
    fn joyGetNumDevs(&self) -> u32 {
        todo!("joyGetNumDevs")
    }
    #[doc = "joyGetPos from WINMM"]
    fn joyGetPos(&self, u_joy_id: u32, pji: MutPtr<JOYINFO>) -> u32 {
        todo!("joyGetPos")
    }
    #[doc = "joyGetPosEx from WINMM"]
    fn joyGetPosEx(&self, u_joy_id: u32, pji: MutPtr<JOYINFOEX>) -> u32 {
        todo!("joyGetPosEx")
    }
    #[doc = "joyGetThreshold from WINMM"]
    fn joyGetThreshold(&self, u_joy_id: u32, pu_threshold: MutPtr<u32>) -> u32 {
        todo!("joyGetThreshold")
    }
    #[doc = "joyReleaseCapture from WINMM"]
    fn joyReleaseCapture(&self, u_joy_id: u32) -> u32 {
        todo!("joyReleaseCapture")
    }
    #[doc = "joySetCapture from WINMM"]
    fn joySetCapture(
        &self,
        hwnd: super::super::Foundation::HWND,
        u_joy_id: u32,
        u_period: u32,
        f_changed: super::super::Foundation::BOOL,
    ) -> u32 {
        todo!("joySetCapture")
    }
    #[doc = "joySetThreshold from WINMM"]
    fn joySetThreshold(&self, u_joy_id: u32, u_threshold: u32) -> u32 {
        todo!("joySetThreshold")
    }
    #[doc = "mciDriverNotify from WINMM"]
    fn mciDriverNotify(
        &self,
        hwnd_callback: super::super::Foundation::HANDLE,
        w_device_id: u32,
        u_status: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("mciDriverNotify")
    }
    #[doc = "mciDriverYield from WINMM"]
    fn mciDriverYield(&self, w_device_id: u32) -> u32 {
        todo!("mciDriverYield")
    }
    #[doc = "mciFreeCommandResource from WINMM"]
    fn mciFreeCommandResource(&self, w_table: u32) -> super::super::Foundation::BOOL {
        todo!("mciFreeCommandResource")
    }
    #[doc = "mciGetCreatorTask from WINMM"]
    fn mciGetCreatorTask(&self, mci_id: u32) -> super::HTASK {
        todo!("mciGetCreatorTask")
    }
    #[doc = "mciGetDeviceIDA from WINMM"]
    fn mciGetDeviceIDA(&self, psz_device: PCSTR) -> u32 {
        todo!("mciGetDeviceIDA")
    }
    #[doc = "mciGetDeviceIDFromElementIDA from WINMM"]
    fn mciGetDeviceIDFromElementIDA(&self, dw_element_id: u32, lpstr_type: PCSTR) -> u32 {
        todo!("mciGetDeviceIDFromElementIDA")
    }
    #[doc = "mciGetDeviceIDFromElementIDW from WINMM"]
    fn mciGetDeviceIDFromElementIDW(&self, dw_element_id: u32, lpstr_type: PCWSTR) -> u32 {
        todo!("mciGetDeviceIDFromElementIDW")
    }
    #[doc = "mciGetDeviceIDW from WINMM"]
    fn mciGetDeviceIDW(&self, psz_device: PCWSTR) -> u32 {
        todo!("mciGetDeviceIDW")
    }
    #[doc = "mciGetDriverData from WINMM"]
    fn mciGetDriverData(&self, w_device_id: u32) -> PtrRepr {
        todo!("mciGetDriverData")
    }
    #[doc = "mciGetErrorStringA from WINMM"]
    fn mciGetErrorStringA(
        &self,
        mcierr: u32,
        psz_text: PSTR,
        cch_text: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("mciGetErrorStringA")
    }
    #[doc = "mciGetErrorStringW from WINMM"]
    fn mciGetErrorStringW(
        &self,
        mcierr: u32,
        psz_text: PWSTR,
        cch_text: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("mciGetErrorStringW")
    }
    #[doc = "mciGetYieldProc from WINMM"]
    fn mciGetYieldProc(&self, mci_id: u32, pdw_yield_data: ConstPtr<u32>) -> YIELDPROC {
        todo!("mciGetYieldProc")
    }
    #[doc = "mciLoadCommandResource from WINMM"]
    fn mciLoadCommandResource(
        &self,
        h_instance: super::super::Foundation::HANDLE,
        lp_res_name: PCWSTR,
        w_type: u32,
    ) -> u32 {
        todo!("mciLoadCommandResource")
    }
    #[doc = "mciSendCommandA from WINMM"]
    fn mciSendCommandA(
        &self,
        mci_id: u32,
        u_msg: u32,
        dw_param_1: PtrRepr,
        dw_param_2: PtrRepr,
    ) -> u32 {
        todo!("mciSendCommandA")
    }
    #[doc = "mciSendCommandW from WINMM"]
    fn mciSendCommandW(
        &self,
        mci_id: u32,
        u_msg: u32,
        dw_param_1: PtrRepr,
        dw_param_2: PtrRepr,
    ) -> u32 {
        todo!("mciSendCommandW")
    }
    #[doc = "mciSendStringA from WINMM"]
    fn mciSendStringA(
        &self,
        lpstr_command: PCSTR,
        lpstr_return_string: PSTR,
        u_return_length: u32,
        hwnd_callback: super::super::Foundation::HWND,
    ) -> u32 {
        todo!("mciSendStringA")
    }
    #[doc = "mciSendStringW from WINMM"]
    fn mciSendStringW(
        &self,
        lpstr_command: PCWSTR,
        lpstr_return_string: PWSTR,
        u_return_length: u32,
        hwnd_callback: super::super::Foundation::HWND,
    ) -> u32 {
        todo!("mciSendStringW")
    }
    #[doc = "mciSetDriverData from WINMM"]
    fn mciSetDriverData(
        &self,
        w_device_id: u32,
        dw_data: PtrRepr,
    ) -> super::super::Foundation::BOOL {
        todo!("mciSetDriverData")
    }
    #[doc = "mciSetYieldProc from WINMM"]
    fn mciSetYieldProc(
        &self,
        mci_id: u32,
        fp_yield_proc: YIELDPROC,
        dw_yield_data: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("mciSetYieldProc")
    }
    #[doc = "mmDrvInstall from WINMM"]
    fn mmDrvInstall(
        &self,
        h_driver: HDRVR,
        wsz_drv_entry: PCWSTR,
        drv_message: DRIVERMSGPROC,
        w_flags: u32,
    ) -> u32 {
        todo!("mmDrvInstall")
    }
    #[doc = "mmGetCurrentTask from WINMM"]
    fn mmGetCurrentTask(&self) -> u32 {
        todo!("mmGetCurrentTask")
    }
    #[doc = "mmTaskBlock from WINMM"]
    fn mmTaskBlock(&self, h: u32) {
        todo!("mmTaskBlock")
    }
    #[doc = "mmTaskCreate from WINMM"]
    fn mmTaskCreate(
        &self,
        lpfn: LPTASKCALLBACK,
        lph: MutPtr<super::super::Foundation::HANDLE>,
        dw_inst: PtrRepr,
    ) -> u32 {
        todo!("mmTaskCreate")
    }
    #[doc = "mmTaskSignal from WINMM"]
    fn mmTaskSignal(&self, h: u32) -> super::super::Foundation::BOOL {
        todo!("mmTaskSignal")
    }
    #[doc = "mmTaskYield from WINMM"]
    fn mmTaskYield(&self) {
        todo!("mmTaskYield")
    }
    #[doc = "mmioAdvance from WINMM"]
    fn mmioAdvance(&self, hmmio: HMMIO, pmmioinfo: ConstPtr<MMIOINFO>, fu_advance: u32) -> u32 {
        todo!("mmioAdvance")
    }
    #[doc = "mmioAscend from WINMM"]
    fn mmioAscend(&self, hmmio: HMMIO, pmmcki: ConstPtr<MMCKINFO>, fu_ascend: u32) -> u32 {
        todo!("mmioAscend")
    }
    #[doc = "mmioClose from WINMM"]
    fn mmioClose(&self, hmmio: HMMIO, fu_close: u32) -> u32 {
        todo!("mmioClose")
    }
    #[doc = "mmioCreateChunk from WINMM"]
    fn mmioCreateChunk(&self, hmmio: HMMIO, pmmcki: ConstPtr<MMCKINFO>, fu_create: u32) -> u32 {
        todo!("mmioCreateChunk")
    }
    #[doc = "mmioDescend from WINMM"]
    fn mmioDescend(
        &self,
        hmmio: HMMIO,
        pmmcki: MutPtr<MMCKINFO>,
        pmmcki_parent: ConstPtr<MMCKINFO>,
        fu_descend: u32,
    ) -> u32 {
        todo!("mmioDescend")
    }
    #[doc = "mmioFlush from WINMM"]
    fn mmioFlush(&self, hmmio: HMMIO, fu_flush: u32) -> u32 {
        todo!("mmioFlush")
    }
    #[doc = "mmioGetInfo from WINMM"]
    fn mmioGetInfo(&self, hmmio: HMMIO, pmmioinfo: MutPtr<MMIOINFO>, fu_info: u32) -> u32 {
        todo!("mmioGetInfo")
    }
    #[doc = "mmioInstallIOProcA from WINMM"]
    fn mmioInstallIOProcA(
        &self,
        fcc_io_proc: u32,
        p_io_proc: LPMMIOPROC,
        dw_flags: u32,
    ) -> LPMMIOPROC {
        todo!("mmioInstallIOProcA")
    }
    #[doc = "mmioInstallIOProcW from WINMM"]
    fn mmioInstallIOProcW(
        &self,
        fcc_io_proc: u32,
        p_io_proc: LPMMIOPROC,
        dw_flags: u32,
    ) -> LPMMIOPROC {
        todo!("mmioInstallIOProcW")
    }
    #[doc = "mmioOpenA from WINMM"]
    fn mmioOpenA(&self, psz_file_name: PSTR, pmmioinfo: MutPtr<MMIOINFO>, fdw_open: u32) -> HMMIO {
        todo!("mmioOpenA")
    }
    #[doc = "mmioOpenW from WINMM"]
    fn mmioOpenW(&self, psz_file_name: PWSTR, pmmioinfo: MutPtr<MMIOINFO>, fdw_open: u32) -> HMMIO {
        todo!("mmioOpenW")
    }
    #[doc = "mmioRead from WINMM"]
    fn mmioRead(&self, hmmio: HMMIO, pch: MutPtr<i8>, cch: i32) -> i32 {
        todo!("mmioRead")
    }
    #[doc = "mmioRenameA from WINMM"]
    fn mmioRenameA(
        &self,
        psz_file_name: PCSTR,
        psz_new_file_name: PCSTR,
        pmmioinfo: ConstPtr<MMIOINFO>,
        fdw_rename: u32,
    ) -> u32 {
        todo!("mmioRenameA")
    }
    #[doc = "mmioRenameW from WINMM"]
    fn mmioRenameW(
        &self,
        psz_file_name: PCWSTR,
        psz_new_file_name: PCWSTR,
        pmmioinfo: ConstPtr<MMIOINFO>,
        fdw_rename: u32,
    ) -> u32 {
        todo!("mmioRenameW")
    }
    #[doc = "mmioSeek from WINMM"]
    fn mmioSeek(&self, hmmio: HMMIO, l_offset: i32, i_origin: i32) -> i32 {
        todo!("mmioSeek")
    }
    #[doc = "mmioSendMessage from WINMM"]
    fn mmioSendMessage(
        &self,
        hmmio: HMMIO,
        u_msg: u32,
        l_param_1: super::super::Foundation::LPARAM,
        l_param_2: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::LRESULT {
        todo!("mmioSendMessage")
    }
    #[doc = "mmioSetBuffer from WINMM"]
    fn mmioSetBuffer(
        &self,
        hmmio: HMMIO,
        pch_buffer: PSTR,
        cch_buffer: i32,
        fu_buffer: u32,
    ) -> u32 {
        todo!("mmioSetBuffer")
    }
    #[doc = "mmioSetInfo from WINMM"]
    fn mmioSetInfo(&self, hmmio: HMMIO, pmmioinfo: ConstPtr<MMIOINFO>, fu_info: u32) -> u32 {
        todo!("mmioSetInfo")
    }
    #[doc = "mmioStringToFOURCCA from WINMM"]
    fn mmioStringToFOURCCA(&self, sz: PCSTR, u_flags: u32) -> u32 {
        todo!("mmioStringToFOURCCA")
    }
    #[doc = "mmioStringToFOURCCW from WINMM"]
    fn mmioStringToFOURCCW(&self, sz: PCWSTR, u_flags: u32) -> u32 {
        todo!("mmioStringToFOURCCW")
    }
    #[doc = "mmioWrite from WINMM"]
    fn mmioWrite(&self, hmmio: HMMIO, pch: PCSTR, cch: i32) -> i32 {
        todo!("mmioWrite")
    }
}
pub fn get_api(ctx: &crate::core::Win32Context) -> std::sync::Arc<dyn Api> {
    ctx.get::<dyn Api>()
}
