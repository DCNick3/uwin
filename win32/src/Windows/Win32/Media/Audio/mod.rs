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
pub const ACMDM_DRIVER_ABOUT: u32 = 24587u32;
pub const ACMDM_DRIVER_DETAILS: u32 = 24586u32;
pub const ACMDM_DRIVER_NOTIFY: u32 = 24577u32;
pub const ACMDM_FILTERTAG_DETAILS: u32 = 24626u32;
pub const ACMDM_FILTER_DETAILS: u32 = 24627u32;
pub const ACMDM_FORMATTAG_DETAILS: u32 = 24601u32;
pub const ACMDM_FORMAT_DETAILS: u32 = 24602u32;
pub const ACMDM_FORMAT_SUGGEST: u32 = 24603u32;
pub const ACMDM_HARDWARE_WAVE_CAPS_INPUT: u32 = 24596u32;
pub const ACMDM_HARDWARE_WAVE_CAPS_OUTPUT: u32 = 24597u32;
pub const ACMDM_RESERVED_HIGH: u32 = 28671u32;
pub const ACMDM_RESERVED_LOW: u32 = 24576u32;
pub const ACMDM_STREAM_CLOSE: u32 = 24653u32;
pub const ACMDM_STREAM_CONVERT: u32 = 24655u32;
pub const ACMDM_STREAM_OPEN: u32 = 24652u32;
pub const ACMDM_STREAM_PREPARE: u32 = 24657u32;
pub const ACMDM_STREAM_RESET: u32 = 24656u32;
pub const ACMDM_STREAM_SIZE: u32 = 24654u32;
pub const ACMDM_STREAM_UNPREPARE: u32 = 24658u32;
pub const ACMDM_STREAM_UPDATE: u32 = 24659u32;
pub const ACMDM_USER: u32 = 16384u32;
pub struct ACMDRIVERDETAILSA {
    pub cbStruct: u32,
    pub fccType: u32,
    pub fccComp: u32,
    pub wMid: u16,
    pub wPid: u16,
    pub vdwACM: u32,
    pub vdwDriver: u32,
    pub fdwSupport: u32,
    pub cFormatTags: u32,
    pub cFilterTags: u32,
    pub hicon: super::super::UI::WindowsAndMessaging::HICON,
    pub szShortName: [super::super::Foundation::CHAR; 32],
    pub szLongName: [super::super::Foundation::CHAR; 128],
    pub szCopyright: [super::super::Foundation::CHAR; 80],
    pub szLicensing: [super::super::Foundation::CHAR; 128],
    pub szFeatures: [super::super::Foundation::CHAR; 512],
}
impl ::core::marker::Copy for ACMDRIVERDETAILSA {}
impl ::core::clone::Clone for ACMDRIVERDETAILSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACMDRIVERDETAILSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACMDRIVERDETAILSA")
            .field("cbStruct", &self.cbStruct)
            .field("fccType", &self.fccType)
            .field("fccComp", &self.fccComp)
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("vdwACM", &self.vdwACM)
            .field("vdwDriver", &self.vdwDriver)
            .field("fdwSupport", &self.fdwSupport)
            .field("cFormatTags", &self.cFormatTags)
            .field("cFilterTags", &self.cFilterTags)
            .field("hicon", &self.hicon)
            .field("szShortName", &self.szShortName)
            .field("szLongName", &self.szLongName)
            .field("szCopyright", &self.szCopyright)
            .field("szLicensing", &self.szLicensing)
            .field("szFeatures", &self.szFeatures)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACMDRIVERDETAILSA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.fccType == other.fccType
            && self.fccComp == other.fccComp
            && self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.vdwACM == other.vdwACM
            && self.vdwDriver == other.vdwDriver
            && self.fdwSupport == other.fdwSupport
            && self.cFormatTags == other.cFormatTags
            && self.cFilterTags == other.cFilterTags
            && self.hicon == other.hicon
            && self.szShortName == other.szShortName
            && self.szLongName == other.szLongName
            && self.szCopyright == other.szCopyright
            && self.szLicensing == other.szLicensing
            && self.szFeatures == other.szFeatures
    }
}
impl ::core::cmp::Eq for ACMDRIVERDETAILSA {}
impl FromIntoMemory for ACMDRIVERDETAILSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 920);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_fccType = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_fccComp = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[12..12 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[14..14 + 2]);
        let f_vdwACM = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_vdwDriver = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_fdwSupport = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_cFormatTags = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_cFilterTags = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_hicon = <super::super::UI::WindowsAndMessaging::HICON as FromIntoMemory>::from_bytes(
            &from[36..36 + 4],
        );
        let f_szShortName = <[super::super::Foundation::CHAR; 32] as FromIntoMemory>::from_bytes(
            &from[40..40 + 32],
        );
        let f_szLongName = <[super::super::Foundation::CHAR; 128] as FromIntoMemory>::from_bytes(
            &from[72..72 + 128],
        );
        let f_szCopyright = <[super::super::Foundation::CHAR; 80] as FromIntoMemory>::from_bytes(
            &from[200..200 + 80],
        );
        let f_szLicensing = <[super::super::Foundation::CHAR; 128] as FromIntoMemory>::from_bytes(
            &from[280..280 + 128],
        );
        let f_szFeatures = <[super::super::Foundation::CHAR; 512] as FromIntoMemory>::from_bytes(
            &from[408..408 + 512],
        );
        Self {
            cbStruct: f_cbStruct,
            fccType: f_fccType,
            fccComp: f_fccComp,
            wMid: f_wMid,
            wPid: f_wPid,
            vdwACM: f_vdwACM,
            vdwDriver: f_vdwDriver,
            fdwSupport: f_fdwSupport,
            cFormatTags: f_cFormatTags,
            cFilterTags: f_cFilterTags,
            hicon: f_hicon,
            szShortName: f_szShortName,
            szLongName: f_szLongName,
            szCopyright: f_szCopyright,
            szLicensing: f_szLicensing,
            szFeatures: f_szFeatures,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 920);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.fccType, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.fccComp, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.wMid, &mut into[12..12 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[14..14 + 2]);
        FromIntoMemory::into_bytes(self.vdwACM, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.vdwDriver, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.fdwSupport, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.cFormatTags, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.cFilterTags, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.hicon, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.szShortName, &mut into[40..40 + 32]);
        FromIntoMemory::into_bytes(self.szLongName, &mut into[72..72 + 128]);
        FromIntoMemory::into_bytes(self.szCopyright, &mut into[200..200 + 80]);
        FromIntoMemory::into_bytes(self.szLicensing, &mut into[280..280 + 128]);
        FromIntoMemory::into_bytes(self.szFeatures, &mut into[408..408 + 512]);
    }
    fn size() -> usize {
        920
    }
}
pub struct ACMDRIVERDETAILSW {
    pub cbStruct: u32,
    pub fccType: u32,
    pub fccComp: u32,
    pub wMid: u16,
    pub wPid: u16,
    pub vdwACM: u32,
    pub vdwDriver: u32,
    pub fdwSupport: u32,
    pub cFormatTags: u32,
    pub cFilterTags: u32,
    pub hicon: super::super::UI::WindowsAndMessaging::HICON,
    pub szShortName: [u16; 32],
    pub szLongName: [u16; 128],
    pub szCopyright: [u16; 80],
    pub szLicensing: [u16; 128],
    pub szFeatures: [u16; 512],
}
impl ::core::marker::Copy for ACMDRIVERDETAILSW {}
impl ::core::clone::Clone for ACMDRIVERDETAILSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACMDRIVERDETAILSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACMDRIVERDETAILSW")
            .field("cbStruct", &self.cbStruct)
            .field("fccType", &self.fccType)
            .field("fccComp", &self.fccComp)
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("vdwACM", &self.vdwACM)
            .field("vdwDriver", &self.vdwDriver)
            .field("fdwSupport", &self.fdwSupport)
            .field("cFormatTags", &self.cFormatTags)
            .field("cFilterTags", &self.cFilterTags)
            .field("hicon", &self.hicon)
            .field("szShortName", &self.szShortName)
            .field("szLongName", &self.szLongName)
            .field("szCopyright", &self.szCopyright)
            .field("szLicensing", &self.szLicensing)
            .field("szFeatures", &self.szFeatures)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACMDRIVERDETAILSW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.fccType == other.fccType
            && self.fccComp == other.fccComp
            && self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.vdwACM == other.vdwACM
            && self.vdwDriver == other.vdwDriver
            && self.fdwSupport == other.fdwSupport
            && self.cFormatTags == other.cFormatTags
            && self.cFilterTags == other.cFilterTags
            && self.hicon == other.hicon
            && self.szShortName == other.szShortName
            && self.szLongName == other.szLongName
            && self.szCopyright == other.szCopyright
            && self.szLicensing == other.szLicensing
            && self.szFeatures == other.szFeatures
    }
}
impl ::core::cmp::Eq for ACMDRIVERDETAILSW {}
impl FromIntoMemory for ACMDRIVERDETAILSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 920);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_fccType = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_fccComp = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[12..12 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[14..14 + 2]);
        let f_vdwACM = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_vdwDriver = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_fdwSupport = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_cFormatTags = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_cFilterTags = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_hicon = <super::super::UI::WindowsAndMessaging::HICON as FromIntoMemory>::from_bytes(
            &from[36..36 + 4],
        );
        let f_szShortName = <[u16; 32] as FromIntoMemory>::from_bytes(&from[40..40 + 32]);
        let f_szLongName = <[u16; 128] as FromIntoMemory>::from_bytes(&from[72..72 + 128]);
        let f_szCopyright = <[u16; 80] as FromIntoMemory>::from_bytes(&from[200..200 + 80]);
        let f_szLicensing = <[u16; 128] as FromIntoMemory>::from_bytes(&from[280..280 + 128]);
        let f_szFeatures = <[u16; 512] as FromIntoMemory>::from_bytes(&from[408..408 + 512]);
        Self {
            cbStruct: f_cbStruct,
            fccType: f_fccType,
            fccComp: f_fccComp,
            wMid: f_wMid,
            wPid: f_wPid,
            vdwACM: f_vdwACM,
            vdwDriver: f_vdwDriver,
            fdwSupport: f_fdwSupport,
            cFormatTags: f_cFormatTags,
            cFilterTags: f_cFilterTags,
            hicon: f_hicon,
            szShortName: f_szShortName,
            szLongName: f_szLongName,
            szCopyright: f_szCopyright,
            szLicensing: f_szLicensing,
            szFeatures: f_szFeatures,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 920);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.fccType, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.fccComp, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.wMid, &mut into[12..12 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[14..14 + 2]);
        FromIntoMemory::into_bytes(self.vdwACM, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.vdwDriver, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.fdwSupport, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.cFormatTags, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.cFilterTags, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.hicon, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.szShortName, &mut into[40..40 + 32]);
        FromIntoMemory::into_bytes(self.szLongName, &mut into[72..72 + 128]);
        FromIntoMemory::into_bytes(self.szCopyright, &mut into[200..200 + 80]);
        FromIntoMemory::into_bytes(self.szLicensing, &mut into[280..280 + 128]);
        FromIntoMemory::into_bytes(self.szFeatures, &mut into[408..408 + 512]);
    }
    fn size() -> usize {
        920
    }
}
pub const ACMDRIVERDETAILS_COPYRIGHT_CHARS: u32 = 80u32;
pub const ACMDRIVERDETAILS_FEATURES_CHARS: u32 = 512u32;
pub const ACMDRIVERDETAILS_LICENSING_CHARS: u32 = 128u32;
pub const ACMDRIVERDETAILS_LONGNAME_CHARS: u32 = 128u32;
pub const ACMDRIVERDETAILS_SHORTNAME_CHARS: u32 = 32u32;
pub const ACMDRIVERDETAILS_SUPPORTF_ASYNC: i32 = 16i32;
pub const ACMDRIVERDETAILS_SUPPORTF_CODEC: i32 = 1i32;
pub const ACMDRIVERDETAILS_SUPPORTF_CONVERTER: i32 = 2i32;
pub const ACMDRIVERDETAILS_SUPPORTF_DISABLED: i32 = -2147483648i32;
pub const ACMDRIVERDETAILS_SUPPORTF_FILTER: i32 = 4i32;
pub const ACMDRIVERDETAILS_SUPPORTF_HARDWARE: i32 = 8i32;
pub const ACMDRIVERDETAILS_SUPPORTF_LOCAL: i32 = 1073741824i32;
pub type ACMDRIVERENUMCB =
    StdCallFnPtr<(HACMDRIVERID, PtrRepr, u32), super::super::Foundation::BOOL>;
pub struct ACMDRVFORMATSUGGEST {
    pub cbStruct: u32,
    pub fdwSuggest: u32,
    pub pwfxSrc: MutPtr<WAVEFORMATEX>,
    pub cbwfxSrc: u32,
    pub pwfxDst: MutPtr<WAVEFORMATEX>,
    pub cbwfxDst: u32,
}
impl ::core::marker::Copy for ACMDRVFORMATSUGGEST {}
impl ::core::clone::Clone for ACMDRVFORMATSUGGEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACMDRVFORMATSUGGEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACMDRVFORMATSUGGEST")
            .field("cbStruct", &self.cbStruct)
            .field("fdwSuggest", &self.fdwSuggest)
            .field("pwfxSrc", &self.pwfxSrc)
            .field("cbwfxSrc", &self.cbwfxSrc)
            .field("pwfxDst", &self.pwfxDst)
            .field("cbwfxDst", &self.cbwfxDst)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACMDRVFORMATSUGGEST {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.fdwSuggest == other.fdwSuggest
            && self.pwfxSrc == other.pwfxSrc
            && self.cbwfxSrc == other.cbwfxSrc
            && self.pwfxDst == other.pwfxDst
            && self.cbwfxDst == other.cbwfxDst
    }
}
impl ::core::cmp::Eq for ACMDRVFORMATSUGGEST {}
impl FromIntoMemory for ACMDRVFORMATSUGGEST {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_fdwSuggest = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_pwfxSrc = <MutPtr<WAVEFORMATEX> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_cbwfxSrc = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_pwfxDst = <MutPtr<WAVEFORMATEX> as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_cbwfxDst = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            cbStruct: f_cbStruct,
            fdwSuggest: f_fdwSuggest,
            pwfxSrc: f_pwfxSrc,
            cbwfxSrc: f_cbwfxSrc,
            pwfxDst: f_pwfxDst,
            cbwfxDst: f_cbwfxDst,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.fdwSuggest, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.pwfxSrc, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.cbwfxSrc, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.pwfxDst, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.cbwfxDst, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct ACMDRVSTREAMHEADER {
    pub cbStruct: u32,
    pub fdwStatus: u32,
    pub dwUser: PtrRepr,
    pub pbSrc: MutPtr<u8>,
    pub cbSrcLength: u32,
    pub cbSrcLengthUsed: u32,
    pub dwSrcUser: PtrRepr,
    pub pbDst: MutPtr<u8>,
    pub cbDstLength: u32,
    pub cbDstLengthUsed: u32,
    pub dwDstUser: PtrRepr,
    pub fdwConvert: u32,
    pub padshNext: MutPtr<ACMDRVSTREAMHEADER>,
    pub fdwDriver: u32,
    pub dwDriver: PtrRepr,
    pub fdwPrepared: u32,
    pub dwPrepared: PtrRepr,
    pub pbPreparedSrc: MutPtr<u8>,
    pub cbPreparedSrcLength: u32,
    pub pbPreparedDst: MutPtr<u8>,
    pub cbPreparedDstLength: u32,
}
impl ::core::marker::Copy for ACMDRVSTREAMHEADER {}
impl ::core::clone::Clone for ACMDRVSTREAMHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACMDRVSTREAMHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACMDRVSTREAMHEADER")
            .field("cbStruct", &self.cbStruct)
            .field("fdwStatus", &self.fdwStatus)
            .field("dwUser", &self.dwUser)
            .field("pbSrc", &self.pbSrc)
            .field("cbSrcLength", &self.cbSrcLength)
            .field("cbSrcLengthUsed", &self.cbSrcLengthUsed)
            .field("dwSrcUser", &self.dwSrcUser)
            .field("pbDst", &self.pbDst)
            .field("cbDstLength", &self.cbDstLength)
            .field("cbDstLengthUsed", &self.cbDstLengthUsed)
            .field("dwDstUser", &self.dwDstUser)
            .field("fdwConvert", &self.fdwConvert)
            .field("padshNext", &self.padshNext)
            .field("fdwDriver", &self.fdwDriver)
            .field("dwDriver", &self.dwDriver)
            .field("fdwPrepared", &self.fdwPrepared)
            .field("dwPrepared", &self.dwPrepared)
            .field("pbPreparedSrc", &self.pbPreparedSrc)
            .field("cbPreparedSrcLength", &self.cbPreparedSrcLength)
            .field("pbPreparedDst", &self.pbPreparedDst)
            .field("cbPreparedDstLength", &self.cbPreparedDstLength)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACMDRVSTREAMHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.fdwStatus == other.fdwStatus
            && self.dwUser == other.dwUser
            && self.pbSrc == other.pbSrc
            && self.cbSrcLength == other.cbSrcLength
            && self.cbSrcLengthUsed == other.cbSrcLengthUsed
            && self.dwSrcUser == other.dwSrcUser
            && self.pbDst == other.pbDst
            && self.cbDstLength == other.cbDstLength
            && self.cbDstLengthUsed == other.cbDstLengthUsed
            && self.dwDstUser == other.dwDstUser
            && self.fdwConvert == other.fdwConvert
            && self.padshNext == other.padshNext
            && self.fdwDriver == other.fdwDriver
            && self.dwDriver == other.dwDriver
            && self.fdwPrepared == other.fdwPrepared
            && self.dwPrepared == other.dwPrepared
            && self.pbPreparedSrc == other.pbPreparedSrc
            && self.cbPreparedSrcLength == other.cbPreparedSrcLength
            && self.pbPreparedDst == other.pbPreparedDst
            && self.cbPreparedDstLength == other.cbPreparedDstLength
    }
}
impl ::core::cmp::Eq for ACMDRVSTREAMHEADER {}
impl FromIntoMemory for ACMDRVSTREAMHEADER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 84);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_fdwStatus = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwUser = <PtrRepr as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_pbSrc = <MutPtr<u8> as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_cbSrcLength = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_cbSrcLengthUsed = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwSrcUser = <PtrRepr as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_pbDst = <MutPtr<u8> as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_cbDstLength = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_cbDstLengthUsed = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_dwDstUser = <PtrRepr as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_fdwConvert = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_padshNext =
            <MutPtr<ACMDRVSTREAMHEADER> as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_fdwDriver = <u32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_dwDriver = <PtrRepr as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_fdwPrepared = <u32 as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_dwPrepared = <PtrRepr as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        let f_pbPreparedSrc = <MutPtr<u8> as FromIntoMemory>::from_bytes(&from[68..68 + 4]);
        let f_cbPreparedSrcLength = <u32 as FromIntoMemory>::from_bytes(&from[72..72 + 4]);
        let f_pbPreparedDst = <MutPtr<u8> as FromIntoMemory>::from_bytes(&from[76..76 + 4]);
        let f_cbPreparedDstLength = <u32 as FromIntoMemory>::from_bytes(&from[80..80 + 4]);
        Self {
            cbStruct: f_cbStruct,
            fdwStatus: f_fdwStatus,
            dwUser: f_dwUser,
            pbSrc: f_pbSrc,
            cbSrcLength: f_cbSrcLength,
            cbSrcLengthUsed: f_cbSrcLengthUsed,
            dwSrcUser: f_dwSrcUser,
            pbDst: f_pbDst,
            cbDstLength: f_cbDstLength,
            cbDstLengthUsed: f_cbDstLengthUsed,
            dwDstUser: f_dwDstUser,
            fdwConvert: f_fdwConvert,
            padshNext: f_padshNext,
            fdwDriver: f_fdwDriver,
            dwDriver: f_dwDriver,
            fdwPrepared: f_fdwPrepared,
            dwPrepared: f_dwPrepared,
            pbPreparedSrc: f_pbPreparedSrc,
            cbPreparedSrcLength: f_cbPreparedSrcLength,
            pbPreparedDst: f_pbPreparedDst,
            cbPreparedDstLength: f_cbPreparedDstLength,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 84);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.fdwStatus, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwUser, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.pbSrc, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.cbSrcLength, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.cbSrcLengthUsed, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwSrcUser, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.pbDst, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.cbDstLength, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.cbDstLengthUsed, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.dwDstUser, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.fdwConvert, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.padshNext, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.fdwDriver, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.dwDriver, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.fdwPrepared, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.dwPrepared, &mut into[64..64 + 4]);
        FromIntoMemory::into_bytes(self.pbPreparedSrc, &mut into[68..68 + 4]);
        FromIntoMemory::into_bytes(self.cbPreparedSrcLength, &mut into[72..72 + 4]);
        FromIntoMemory::into_bytes(self.pbPreparedDst, &mut into[76..76 + 4]);
        FromIntoMemory::into_bytes(self.cbPreparedDstLength, &mut into[80..80 + 4]);
    }
    fn size() -> usize {
        84
    }
}
pub struct ACMDRVSTREAMINSTANCE {
    pub cbStruct: u32,
    pub pwfxSrc: MutPtr<WAVEFORMATEX>,
    pub pwfxDst: MutPtr<WAVEFORMATEX>,
    pub pwfltr: MutPtr<WAVEFILTER>,
    pub dwCallback: PtrRepr,
    pub dwInstance: PtrRepr,
    pub fdwOpen: u32,
    pub fdwDriver: u32,
    pub dwDriver: PtrRepr,
    pub has: HACMSTREAM,
}
impl ::core::marker::Copy for ACMDRVSTREAMINSTANCE {}
impl ::core::clone::Clone for ACMDRVSTREAMINSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACMDRVSTREAMINSTANCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACMDRVSTREAMINSTANCE")
            .field("cbStruct", &self.cbStruct)
            .field("pwfxSrc", &self.pwfxSrc)
            .field("pwfxDst", &self.pwfxDst)
            .field("pwfltr", &self.pwfltr)
            .field("dwCallback", &self.dwCallback)
            .field("dwInstance", &self.dwInstance)
            .field("fdwOpen", &self.fdwOpen)
            .field("fdwDriver", &self.fdwDriver)
            .field("dwDriver", &self.dwDriver)
            .field("has", &self.has)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACMDRVSTREAMINSTANCE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.pwfxSrc == other.pwfxSrc
            && self.pwfxDst == other.pwfxDst
            && self.pwfltr == other.pwfltr
            && self.dwCallback == other.dwCallback
            && self.dwInstance == other.dwInstance
            && self.fdwOpen == other.fdwOpen
            && self.fdwDriver == other.fdwDriver
            && self.dwDriver == other.dwDriver
            && self.has == other.has
    }
}
impl ::core::cmp::Eq for ACMDRVSTREAMINSTANCE {}
impl FromIntoMemory for ACMDRVSTREAMINSTANCE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_pwfxSrc = <MutPtr<WAVEFORMATEX> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_pwfxDst = <MutPtr<WAVEFORMATEX> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_pwfltr = <MutPtr<WAVEFILTER> as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwCallback = <PtrRepr as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwInstance = <PtrRepr as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_fdwOpen = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_fdwDriver = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_dwDriver = <PtrRepr as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_has = <HACMSTREAM as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        Self {
            cbStruct: f_cbStruct,
            pwfxSrc: f_pwfxSrc,
            pwfxDst: f_pwfxDst,
            pwfltr: f_pwfltr,
            dwCallback: f_dwCallback,
            dwInstance: f_dwInstance,
            fdwOpen: f_fdwOpen,
            fdwDriver: f_fdwDriver,
            dwDriver: f_dwDriver,
            has: f_has,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.pwfxSrc, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.pwfxDst, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.pwfltr, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwCallback, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwInstance, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.fdwOpen, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.fdwDriver, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.dwDriver, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.has, &mut into[36..36 + 4]);
    }
    fn size() -> usize {
        40
    }
}
pub struct ACMDRVSTREAMSIZE {
    pub cbStruct: u32,
    pub fdwSize: u32,
    pub cbSrcLength: u32,
    pub cbDstLength: u32,
}
impl ::core::marker::Copy for ACMDRVSTREAMSIZE {}
impl ::core::clone::Clone for ACMDRVSTREAMSIZE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACMDRVSTREAMSIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACMDRVSTREAMSIZE")
            .field("cbStruct", &self.cbStruct)
            .field("fdwSize", &self.fdwSize)
            .field("cbSrcLength", &self.cbSrcLength)
            .field("cbDstLength", &self.cbDstLength)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACMDRVSTREAMSIZE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.fdwSize == other.fdwSize
            && self.cbSrcLength == other.cbSrcLength
            && self.cbDstLength == other.cbDstLength
    }
}
impl ::core::cmp::Eq for ACMDRVSTREAMSIZE {}
impl FromIntoMemory for ACMDRVSTREAMSIZE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_fdwSize = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_cbSrcLength = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_cbDstLength = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            cbStruct: f_cbStruct,
            fdwSize: f_fdwSize,
            cbSrcLength: f_cbSrcLength,
            cbDstLength: f_cbDstLength,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.fdwSize, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.cbSrcLength, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.cbDstLength, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub const ACMERR_BASE: u32 = 512u32;
pub const ACMERR_BUSY: u32 = 513u32;
pub const ACMERR_CANCELED: u32 = 515u32;
pub const ACMERR_NOTPOSSIBLE: u32 = 512u32;
pub const ACMERR_UNPREPARED: u32 = 514u32;
pub struct ACMFILTERCHOOSEA {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pwfltr: MutPtr<WAVEFILTER>,
    pub cbwfltr: u32,
    pub pszTitle: PCSTR,
    pub szFilterTag: [super::super::Foundation::CHAR; 48],
    pub szFilter: [super::super::Foundation::CHAR; 128],
    pub pszName: PSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfltrEnum: MutPtr<WAVEFILTER>,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub pszTemplateName: PCSTR,
    pub lCustData: super::super::Foundation::LPARAM,
    pub pfnHook: ACMFILTERCHOOSEHOOKPROCA,
}
impl ::core::marker::Copy for ACMFILTERCHOOSEA {}
impl ::core::clone::Clone for ACMFILTERCHOOSEA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACMFILTERCHOOSEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACMFILTERCHOOSEA")
            .field("cbStruct", &self.cbStruct)
            .field("fdwStyle", &self.fdwStyle)
            .field("hwndOwner", &self.hwndOwner)
            .field("pwfltr", &self.pwfltr)
            .field("cbwfltr", &self.cbwfltr)
            .field("pszTitle", &self.pszTitle)
            .field("szFilterTag", &self.szFilterTag)
            .field("szFilter", &self.szFilter)
            .field("pszName", &self.pszName)
            .field("cchName", &self.cchName)
            .field("fdwEnum", &self.fdwEnum)
            .field("pwfltrEnum", &self.pwfltrEnum)
            .field("hInstance", &self.hInstance)
            .field("pszTemplateName", &self.pszTemplateName)
            .field("lCustData", &self.lCustData)
            .field("pfnHook", &self.pfnHook)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACMFILTERCHOOSEA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.fdwStyle == other.fdwStyle
            && self.hwndOwner == other.hwndOwner
            && self.pwfltr == other.pwfltr
            && self.cbwfltr == other.cbwfltr
            && self.pszTitle == other.pszTitle
            && self.szFilterTag == other.szFilterTag
            && self.szFilter == other.szFilter
            && self.pszName == other.pszName
            && self.cchName == other.cchName
            && self.fdwEnum == other.fdwEnum
            && self.pwfltrEnum == other.pwfltrEnum
            && self.hInstance == other.hInstance
            && self.pszTemplateName == other.pszTemplateName
            && self.lCustData == other.lCustData
            && self.pfnHook == other.pfnHook
    }
}
impl ::core::cmp::Eq for ACMFILTERCHOOSEA {}
impl FromIntoMemory for ACMFILTERCHOOSEA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 232);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_fdwStyle = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_hwndOwner =
            <super::super::Foundation::HWND as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_pwfltr = <MutPtr<WAVEFILTER> as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_cbwfltr = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_pszTitle = <PCSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_szFilterTag = <[super::super::Foundation::CHAR; 48] as FromIntoMemory>::from_bytes(
            &from[24..24 + 48],
        );
        let f_szFilter = <[super::super::Foundation::CHAR; 128] as FromIntoMemory>::from_bytes(
            &from[72..72 + 128],
        );
        let f_pszName = <PSTR as FromIntoMemory>::from_bytes(&from[200..200 + 4]);
        let f_cchName = <u32 as FromIntoMemory>::from_bytes(&from[204..204 + 4]);
        let f_fdwEnum = <u32 as FromIntoMemory>::from_bytes(&from[208..208 + 4]);
        let f_pwfltrEnum = <MutPtr<WAVEFILTER> as FromIntoMemory>::from_bytes(&from[212..212 + 4]);
        let f_hInstance = <super::super::Foundation::HINSTANCE as FromIntoMemory>::from_bytes(
            &from[216..216 + 4],
        );
        let f_pszTemplateName = <PCSTR as FromIntoMemory>::from_bytes(&from[220..220 + 4]);
        let f_lCustData =
            <super::super::Foundation::LPARAM as FromIntoMemory>::from_bytes(&from[224..224 + 4]);
        let f_pfnHook =
            <ACMFILTERCHOOSEHOOKPROCA as FromIntoMemory>::from_bytes(&from[228..228 + 4]);
        Self {
            cbStruct: f_cbStruct,
            fdwStyle: f_fdwStyle,
            hwndOwner: f_hwndOwner,
            pwfltr: f_pwfltr,
            cbwfltr: f_cbwfltr,
            pszTitle: f_pszTitle,
            szFilterTag: f_szFilterTag,
            szFilter: f_szFilter,
            pszName: f_pszName,
            cchName: f_cchName,
            fdwEnum: f_fdwEnum,
            pwfltrEnum: f_pwfltrEnum,
            hInstance: f_hInstance,
            pszTemplateName: f_pszTemplateName,
            lCustData: f_lCustData,
            pfnHook: f_pfnHook,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 232);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.fdwStyle, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.hwndOwner, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.pwfltr, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.cbwfltr, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.pszTitle, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.szFilterTag, &mut into[24..24 + 48]);
        FromIntoMemory::into_bytes(self.szFilter, &mut into[72..72 + 128]);
        FromIntoMemory::into_bytes(self.pszName, &mut into[200..200 + 4]);
        FromIntoMemory::into_bytes(self.cchName, &mut into[204..204 + 4]);
        FromIntoMemory::into_bytes(self.fdwEnum, &mut into[208..208 + 4]);
        FromIntoMemory::into_bytes(self.pwfltrEnum, &mut into[212..212 + 4]);
        FromIntoMemory::into_bytes(self.hInstance, &mut into[216..216 + 4]);
        FromIntoMemory::into_bytes(self.pszTemplateName, &mut into[220..220 + 4]);
        FromIntoMemory::into_bytes(self.lCustData, &mut into[224..224 + 4]);
        FromIntoMemory::into_bytes(self.pfnHook, &mut into[228..228 + 4]);
    }
    fn size() -> usize {
        232
    }
}
pub type ACMFILTERCHOOSEHOOKPROCA = StdCallFnPtr<
    (
        super::super::Foundation::HWND,
        u32,
        super::super::Foundation::WPARAM,
        super::super::Foundation::LPARAM,
    ),
    u32,
>;
pub type ACMFILTERCHOOSEHOOKPROCW = StdCallFnPtr<
    (
        super::super::Foundation::HWND,
        u32,
        super::super::Foundation::WPARAM,
        super::super::Foundation::LPARAM,
    ),
    u32,
>;
pub struct ACMFILTERCHOOSEW {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pwfltr: MutPtr<WAVEFILTER>,
    pub cbwfltr: u32,
    pub pszTitle: PCWSTR,
    pub szFilterTag: [u16; 48],
    pub szFilter: [u16; 128],
    pub pszName: PWSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfltrEnum: MutPtr<WAVEFILTER>,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub pszTemplateName: PCWSTR,
    pub lCustData: super::super::Foundation::LPARAM,
    pub pfnHook: ACMFILTERCHOOSEHOOKPROCW,
}
impl ::core::marker::Copy for ACMFILTERCHOOSEW {}
impl ::core::clone::Clone for ACMFILTERCHOOSEW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACMFILTERCHOOSEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACMFILTERCHOOSEW")
            .field("cbStruct", &self.cbStruct)
            .field("fdwStyle", &self.fdwStyle)
            .field("hwndOwner", &self.hwndOwner)
            .field("pwfltr", &self.pwfltr)
            .field("cbwfltr", &self.cbwfltr)
            .field("pszTitle", &self.pszTitle)
            .field("szFilterTag", &self.szFilterTag)
            .field("szFilter", &self.szFilter)
            .field("pszName", &self.pszName)
            .field("cchName", &self.cchName)
            .field("fdwEnum", &self.fdwEnum)
            .field("pwfltrEnum", &self.pwfltrEnum)
            .field("hInstance", &self.hInstance)
            .field("pszTemplateName", &self.pszTemplateName)
            .field("lCustData", &self.lCustData)
            .field("pfnHook", &self.pfnHook)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACMFILTERCHOOSEW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.fdwStyle == other.fdwStyle
            && self.hwndOwner == other.hwndOwner
            && self.pwfltr == other.pwfltr
            && self.cbwfltr == other.cbwfltr
            && self.pszTitle == other.pszTitle
            && self.szFilterTag == other.szFilterTag
            && self.szFilter == other.szFilter
            && self.pszName == other.pszName
            && self.cchName == other.cchName
            && self.fdwEnum == other.fdwEnum
            && self.pwfltrEnum == other.pwfltrEnum
            && self.hInstance == other.hInstance
            && self.pszTemplateName == other.pszTemplateName
            && self.lCustData == other.lCustData
            && self.pfnHook == other.pfnHook
    }
}
impl ::core::cmp::Eq for ACMFILTERCHOOSEW {}
impl FromIntoMemory for ACMFILTERCHOOSEW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 232);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_fdwStyle = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_hwndOwner =
            <super::super::Foundation::HWND as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_pwfltr = <MutPtr<WAVEFILTER> as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_cbwfltr = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_pszTitle = <PCWSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_szFilterTag = <[u16; 48] as FromIntoMemory>::from_bytes(&from[24..24 + 48]);
        let f_szFilter = <[u16; 128] as FromIntoMemory>::from_bytes(&from[72..72 + 128]);
        let f_pszName = <PWSTR as FromIntoMemory>::from_bytes(&from[200..200 + 4]);
        let f_cchName = <u32 as FromIntoMemory>::from_bytes(&from[204..204 + 4]);
        let f_fdwEnum = <u32 as FromIntoMemory>::from_bytes(&from[208..208 + 4]);
        let f_pwfltrEnum = <MutPtr<WAVEFILTER> as FromIntoMemory>::from_bytes(&from[212..212 + 4]);
        let f_hInstance = <super::super::Foundation::HINSTANCE as FromIntoMemory>::from_bytes(
            &from[216..216 + 4],
        );
        let f_pszTemplateName = <PCWSTR as FromIntoMemory>::from_bytes(&from[220..220 + 4]);
        let f_lCustData =
            <super::super::Foundation::LPARAM as FromIntoMemory>::from_bytes(&from[224..224 + 4]);
        let f_pfnHook =
            <ACMFILTERCHOOSEHOOKPROCW as FromIntoMemory>::from_bytes(&from[228..228 + 4]);
        Self {
            cbStruct: f_cbStruct,
            fdwStyle: f_fdwStyle,
            hwndOwner: f_hwndOwner,
            pwfltr: f_pwfltr,
            cbwfltr: f_cbwfltr,
            pszTitle: f_pszTitle,
            szFilterTag: f_szFilterTag,
            szFilter: f_szFilter,
            pszName: f_pszName,
            cchName: f_cchName,
            fdwEnum: f_fdwEnum,
            pwfltrEnum: f_pwfltrEnum,
            hInstance: f_hInstance,
            pszTemplateName: f_pszTemplateName,
            lCustData: f_lCustData,
            pfnHook: f_pfnHook,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 232);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.fdwStyle, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.hwndOwner, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.pwfltr, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.cbwfltr, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.pszTitle, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.szFilterTag, &mut into[24..24 + 48]);
        FromIntoMemory::into_bytes(self.szFilter, &mut into[72..72 + 128]);
        FromIntoMemory::into_bytes(self.pszName, &mut into[200..200 + 4]);
        FromIntoMemory::into_bytes(self.cchName, &mut into[204..204 + 4]);
        FromIntoMemory::into_bytes(self.fdwEnum, &mut into[208..208 + 4]);
        FromIntoMemory::into_bytes(self.pwfltrEnum, &mut into[212..212 + 4]);
        FromIntoMemory::into_bytes(self.hInstance, &mut into[216..216 + 4]);
        FromIntoMemory::into_bytes(self.pszTemplateName, &mut into[220..220 + 4]);
        FromIntoMemory::into_bytes(self.lCustData, &mut into[224..224 + 4]);
        FromIntoMemory::into_bytes(self.pfnHook, &mut into[228..228 + 4]);
    }
    fn size() -> usize {
        232
    }
}
pub const ACMFILTERCHOOSE_STYLEF_CONTEXTHELP: i32 = 128i32;
pub const ACMFILTERCHOOSE_STYLEF_ENABLEHOOK: i32 = 8i32;
pub const ACMFILTERCHOOSE_STYLEF_ENABLETEMPLATE: i32 = 16i32;
pub const ACMFILTERCHOOSE_STYLEF_ENABLETEMPLATEHANDLE: i32 = 32i32;
pub const ACMFILTERCHOOSE_STYLEF_INITTOFILTERSTRUCT: i32 = 64i32;
pub const ACMFILTERCHOOSE_STYLEF_SHOWHELP: i32 = 4i32;
pub struct ACMFILTERDETAILSA {
    pub cbStruct: u32,
    pub dwFilterIndex: u32,
    pub dwFilterTag: u32,
    pub fdwSupport: u32,
    pub pwfltr: MutPtr<WAVEFILTER>,
    pub cbwfltr: u32,
    pub szFilter: [super::super::Foundation::CHAR; 128],
}
impl ::core::marker::Copy for ACMFILTERDETAILSA {}
impl ::core::clone::Clone for ACMFILTERDETAILSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACMFILTERDETAILSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACMFILTERDETAILSA")
            .field("cbStruct", &self.cbStruct)
            .field("dwFilterIndex", &self.dwFilterIndex)
            .field("dwFilterTag", &self.dwFilterTag)
            .field("fdwSupport", &self.fdwSupport)
            .field("pwfltr", &self.pwfltr)
            .field("cbwfltr", &self.cbwfltr)
            .field("szFilter", &self.szFilter)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACMFILTERDETAILSA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwFilterIndex == other.dwFilterIndex
            && self.dwFilterTag == other.dwFilterTag
            && self.fdwSupport == other.fdwSupport
            && self.pwfltr == other.pwfltr
            && self.cbwfltr == other.cbwfltr
            && self.szFilter == other.szFilter
    }
}
impl ::core::cmp::Eq for ACMFILTERDETAILSA {}
impl FromIntoMemory for ACMFILTERDETAILSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 152);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFilterIndex = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwFilterTag = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_fdwSupport = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_pwfltr = <MutPtr<WAVEFILTER> as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_cbwfltr = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_szFilter = <[super::super::Foundation::CHAR; 128] as FromIntoMemory>::from_bytes(
            &from[24..24 + 128],
        );
        Self {
            cbStruct: f_cbStruct,
            dwFilterIndex: f_dwFilterIndex,
            dwFilterTag: f_dwFilterTag,
            fdwSupport: f_fdwSupport,
            pwfltr: f_pwfltr,
            cbwfltr: f_cbwfltr,
            szFilter: f_szFilter,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 152);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFilterIndex, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwFilterTag, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.fdwSupport, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.pwfltr, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.cbwfltr, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.szFilter, &mut into[24..24 + 128]);
    }
    fn size() -> usize {
        152
    }
}
pub struct ACMFILTERDETAILSW {
    pub cbStruct: u32,
    pub dwFilterIndex: u32,
    pub dwFilterTag: u32,
    pub fdwSupport: u32,
    pub pwfltr: MutPtr<WAVEFILTER>,
    pub cbwfltr: u32,
    pub szFilter: [u16; 128],
}
impl ::core::marker::Copy for ACMFILTERDETAILSW {}
impl ::core::clone::Clone for ACMFILTERDETAILSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACMFILTERDETAILSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACMFILTERDETAILSW")
            .field("cbStruct", &self.cbStruct)
            .field("dwFilterIndex", &self.dwFilterIndex)
            .field("dwFilterTag", &self.dwFilterTag)
            .field("fdwSupport", &self.fdwSupport)
            .field("pwfltr", &self.pwfltr)
            .field("cbwfltr", &self.cbwfltr)
            .field("szFilter", &self.szFilter)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACMFILTERDETAILSW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwFilterIndex == other.dwFilterIndex
            && self.dwFilterTag == other.dwFilterTag
            && self.fdwSupport == other.fdwSupport
            && self.pwfltr == other.pwfltr
            && self.cbwfltr == other.cbwfltr
            && self.szFilter == other.szFilter
    }
}
impl ::core::cmp::Eq for ACMFILTERDETAILSW {}
impl FromIntoMemory for ACMFILTERDETAILSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 152);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFilterIndex = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwFilterTag = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_fdwSupport = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_pwfltr = <MutPtr<WAVEFILTER> as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_cbwfltr = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_szFilter = <[u16; 128] as FromIntoMemory>::from_bytes(&from[24..24 + 128]);
        Self {
            cbStruct: f_cbStruct,
            dwFilterIndex: f_dwFilterIndex,
            dwFilterTag: f_dwFilterTag,
            fdwSupport: f_fdwSupport,
            pwfltr: f_pwfltr,
            cbwfltr: f_cbwfltr,
            szFilter: f_szFilter,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 152);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFilterIndex, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwFilterTag, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.fdwSupport, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.pwfltr, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.cbwfltr, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.szFilter, &mut into[24..24 + 128]);
    }
    fn size() -> usize {
        152
    }
}
pub const ACMFILTERDETAILS_FILTER_CHARS: u32 = 128u32;
pub type ACMFILTERENUMCBA = StdCallFnPtr<
    (HACMDRIVERID, MutPtr<ACMFILTERDETAILSA>, PtrRepr, u32),
    super::super::Foundation::BOOL,
>;
pub type ACMFILTERENUMCBW = StdCallFnPtr<
    (HACMDRIVERID, MutPtr<ACMFILTERDETAILSW>, PtrRepr, u32),
    super::super::Foundation::BOOL,
>;
pub struct ACMFILTERTAGDETAILSA {
    pub cbStruct: u32,
    pub dwFilterTagIndex: u32,
    pub dwFilterTag: u32,
    pub cbFilterSize: u32,
    pub fdwSupport: u32,
    pub cStandardFilters: u32,
    pub szFilterTag: [super::super::Foundation::CHAR; 48],
}
impl ::core::marker::Copy for ACMFILTERTAGDETAILSA {}
impl ::core::clone::Clone for ACMFILTERTAGDETAILSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACMFILTERTAGDETAILSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACMFILTERTAGDETAILSA")
            .field("cbStruct", &self.cbStruct)
            .field("dwFilterTagIndex", &self.dwFilterTagIndex)
            .field("dwFilterTag", &self.dwFilterTag)
            .field("cbFilterSize", &self.cbFilterSize)
            .field("fdwSupport", &self.fdwSupport)
            .field("cStandardFilters", &self.cStandardFilters)
            .field("szFilterTag", &self.szFilterTag)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACMFILTERTAGDETAILSA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwFilterTagIndex == other.dwFilterTagIndex
            && self.dwFilterTag == other.dwFilterTag
            && self.cbFilterSize == other.cbFilterSize
            && self.fdwSupport == other.fdwSupport
            && self.cStandardFilters == other.cStandardFilters
            && self.szFilterTag == other.szFilterTag
    }
}
impl ::core::cmp::Eq for ACMFILTERTAGDETAILSA {}
impl FromIntoMemory for ACMFILTERTAGDETAILSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 72);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFilterTagIndex = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwFilterTag = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_cbFilterSize = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_fdwSupport = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_cStandardFilters = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_szFilterTag = <[super::super::Foundation::CHAR; 48] as FromIntoMemory>::from_bytes(
            &from[24..24 + 48],
        );
        Self {
            cbStruct: f_cbStruct,
            dwFilterTagIndex: f_dwFilterTagIndex,
            dwFilterTag: f_dwFilterTag,
            cbFilterSize: f_cbFilterSize,
            fdwSupport: f_fdwSupport,
            cStandardFilters: f_cStandardFilters,
            szFilterTag: f_szFilterTag,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 72);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFilterTagIndex, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwFilterTag, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.cbFilterSize, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.fdwSupport, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.cStandardFilters, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.szFilterTag, &mut into[24..24 + 48]);
    }
    fn size() -> usize {
        72
    }
}
pub struct ACMFILTERTAGDETAILSW {
    pub cbStruct: u32,
    pub dwFilterTagIndex: u32,
    pub dwFilterTag: u32,
    pub cbFilterSize: u32,
    pub fdwSupport: u32,
    pub cStandardFilters: u32,
    pub szFilterTag: [u16; 48],
}
impl ::core::marker::Copy for ACMFILTERTAGDETAILSW {}
impl ::core::clone::Clone for ACMFILTERTAGDETAILSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACMFILTERTAGDETAILSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACMFILTERTAGDETAILSW")
            .field("cbStruct", &self.cbStruct)
            .field("dwFilterTagIndex", &self.dwFilterTagIndex)
            .field("dwFilterTag", &self.dwFilterTag)
            .field("cbFilterSize", &self.cbFilterSize)
            .field("fdwSupport", &self.fdwSupport)
            .field("cStandardFilters", &self.cStandardFilters)
            .field("szFilterTag", &self.szFilterTag)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACMFILTERTAGDETAILSW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwFilterTagIndex == other.dwFilterTagIndex
            && self.dwFilterTag == other.dwFilterTag
            && self.cbFilterSize == other.cbFilterSize
            && self.fdwSupport == other.fdwSupport
            && self.cStandardFilters == other.cStandardFilters
            && self.szFilterTag == other.szFilterTag
    }
}
impl ::core::cmp::Eq for ACMFILTERTAGDETAILSW {}
impl FromIntoMemory for ACMFILTERTAGDETAILSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 72);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFilterTagIndex = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwFilterTag = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_cbFilterSize = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_fdwSupport = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_cStandardFilters = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_szFilterTag = <[u16; 48] as FromIntoMemory>::from_bytes(&from[24..24 + 48]);
        Self {
            cbStruct: f_cbStruct,
            dwFilterTagIndex: f_dwFilterTagIndex,
            dwFilterTag: f_dwFilterTag,
            cbFilterSize: f_cbFilterSize,
            fdwSupport: f_fdwSupport,
            cStandardFilters: f_cStandardFilters,
            szFilterTag: f_szFilterTag,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 72);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFilterTagIndex, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwFilterTag, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.cbFilterSize, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.fdwSupport, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.cStandardFilters, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.szFilterTag, &mut into[24..24 + 48]);
    }
    fn size() -> usize {
        72
    }
}
pub const ACMFILTERTAGDETAILS_FILTERTAG_CHARS: u32 = 48u32;
pub type ACMFILTERTAGENUMCBA = StdCallFnPtr<
    (HACMDRIVERID, MutPtr<ACMFILTERTAGDETAILSA>, PtrRepr, u32),
    super::super::Foundation::BOOL,
>;
pub type ACMFILTERTAGENUMCBW = StdCallFnPtr<
    (HACMDRIVERID, MutPtr<ACMFILTERTAGDETAILSW>, PtrRepr, u32),
    super::super::Foundation::BOOL,
>;
pub struct ACMFORMATCHOOSEA {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pwfx: MutPtr<WAVEFORMATEX>,
    pub cbwfx: u32,
    pub pszTitle: PCSTR,
    pub szFormatTag: [super::super::Foundation::CHAR; 48],
    pub szFormat: [super::super::Foundation::CHAR; 128],
    pub pszName: PSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfxEnum: MutPtr<WAVEFORMATEX>,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub pszTemplateName: PCSTR,
    pub lCustData: super::super::Foundation::LPARAM,
    pub pfnHook: ACMFORMATCHOOSEHOOKPROCA,
}
impl ::core::marker::Copy for ACMFORMATCHOOSEA {}
impl ::core::clone::Clone for ACMFORMATCHOOSEA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACMFORMATCHOOSEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACMFORMATCHOOSEA")
            .field("cbStruct", &self.cbStruct)
            .field("fdwStyle", &self.fdwStyle)
            .field("hwndOwner", &self.hwndOwner)
            .field("pwfx", &self.pwfx)
            .field("cbwfx", &self.cbwfx)
            .field("pszTitle", &self.pszTitle)
            .field("szFormatTag", &self.szFormatTag)
            .field("szFormat", &self.szFormat)
            .field("pszName", &self.pszName)
            .field("cchName", &self.cchName)
            .field("fdwEnum", &self.fdwEnum)
            .field("pwfxEnum", &self.pwfxEnum)
            .field("hInstance", &self.hInstance)
            .field("pszTemplateName", &self.pszTemplateName)
            .field("lCustData", &self.lCustData)
            .field("pfnHook", &self.pfnHook)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACMFORMATCHOOSEA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.fdwStyle == other.fdwStyle
            && self.hwndOwner == other.hwndOwner
            && self.pwfx == other.pwfx
            && self.cbwfx == other.cbwfx
            && self.pszTitle == other.pszTitle
            && self.szFormatTag == other.szFormatTag
            && self.szFormat == other.szFormat
            && self.pszName == other.pszName
            && self.cchName == other.cchName
            && self.fdwEnum == other.fdwEnum
            && self.pwfxEnum == other.pwfxEnum
            && self.hInstance == other.hInstance
            && self.pszTemplateName == other.pszTemplateName
            && self.lCustData == other.lCustData
            && self.pfnHook == other.pfnHook
    }
}
impl ::core::cmp::Eq for ACMFORMATCHOOSEA {}
impl FromIntoMemory for ACMFORMATCHOOSEA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 232);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_fdwStyle = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_hwndOwner =
            <super::super::Foundation::HWND as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_pwfx = <MutPtr<WAVEFORMATEX> as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_cbwfx = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_pszTitle = <PCSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_szFormatTag = <[super::super::Foundation::CHAR; 48] as FromIntoMemory>::from_bytes(
            &from[24..24 + 48],
        );
        let f_szFormat = <[super::super::Foundation::CHAR; 128] as FromIntoMemory>::from_bytes(
            &from[72..72 + 128],
        );
        let f_pszName = <PSTR as FromIntoMemory>::from_bytes(&from[200..200 + 4]);
        let f_cchName = <u32 as FromIntoMemory>::from_bytes(&from[204..204 + 4]);
        let f_fdwEnum = <u32 as FromIntoMemory>::from_bytes(&from[208..208 + 4]);
        let f_pwfxEnum = <MutPtr<WAVEFORMATEX> as FromIntoMemory>::from_bytes(&from[212..212 + 4]);
        let f_hInstance = <super::super::Foundation::HINSTANCE as FromIntoMemory>::from_bytes(
            &from[216..216 + 4],
        );
        let f_pszTemplateName = <PCSTR as FromIntoMemory>::from_bytes(&from[220..220 + 4]);
        let f_lCustData =
            <super::super::Foundation::LPARAM as FromIntoMemory>::from_bytes(&from[224..224 + 4]);
        let f_pfnHook =
            <ACMFORMATCHOOSEHOOKPROCA as FromIntoMemory>::from_bytes(&from[228..228 + 4]);
        Self {
            cbStruct: f_cbStruct,
            fdwStyle: f_fdwStyle,
            hwndOwner: f_hwndOwner,
            pwfx: f_pwfx,
            cbwfx: f_cbwfx,
            pszTitle: f_pszTitle,
            szFormatTag: f_szFormatTag,
            szFormat: f_szFormat,
            pszName: f_pszName,
            cchName: f_cchName,
            fdwEnum: f_fdwEnum,
            pwfxEnum: f_pwfxEnum,
            hInstance: f_hInstance,
            pszTemplateName: f_pszTemplateName,
            lCustData: f_lCustData,
            pfnHook: f_pfnHook,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 232);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.fdwStyle, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.hwndOwner, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.pwfx, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.cbwfx, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.pszTitle, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.szFormatTag, &mut into[24..24 + 48]);
        FromIntoMemory::into_bytes(self.szFormat, &mut into[72..72 + 128]);
        FromIntoMemory::into_bytes(self.pszName, &mut into[200..200 + 4]);
        FromIntoMemory::into_bytes(self.cchName, &mut into[204..204 + 4]);
        FromIntoMemory::into_bytes(self.fdwEnum, &mut into[208..208 + 4]);
        FromIntoMemory::into_bytes(self.pwfxEnum, &mut into[212..212 + 4]);
        FromIntoMemory::into_bytes(self.hInstance, &mut into[216..216 + 4]);
        FromIntoMemory::into_bytes(self.pszTemplateName, &mut into[220..220 + 4]);
        FromIntoMemory::into_bytes(self.lCustData, &mut into[224..224 + 4]);
        FromIntoMemory::into_bytes(self.pfnHook, &mut into[228..228 + 4]);
    }
    fn size() -> usize {
        232
    }
}
pub type ACMFORMATCHOOSEHOOKPROCA = StdCallFnPtr<
    (
        super::super::Foundation::HWND,
        u32,
        super::super::Foundation::WPARAM,
        super::super::Foundation::LPARAM,
    ),
    u32,
>;
pub type ACMFORMATCHOOSEHOOKPROCW = StdCallFnPtr<
    (
        super::super::Foundation::HWND,
        u32,
        super::super::Foundation::WPARAM,
        super::super::Foundation::LPARAM,
    ),
    u32,
>;
pub struct ACMFORMATCHOOSEW {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pwfx: MutPtr<WAVEFORMATEX>,
    pub cbwfx: u32,
    pub pszTitle: PCWSTR,
    pub szFormatTag: [u16; 48],
    pub szFormat: [u16; 128],
    pub pszName: PWSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfxEnum: MutPtr<WAVEFORMATEX>,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub pszTemplateName: PCWSTR,
    pub lCustData: super::super::Foundation::LPARAM,
    pub pfnHook: ACMFORMATCHOOSEHOOKPROCW,
}
impl ::core::marker::Copy for ACMFORMATCHOOSEW {}
impl ::core::clone::Clone for ACMFORMATCHOOSEW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACMFORMATCHOOSEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACMFORMATCHOOSEW")
            .field("cbStruct", &self.cbStruct)
            .field("fdwStyle", &self.fdwStyle)
            .field("hwndOwner", &self.hwndOwner)
            .field("pwfx", &self.pwfx)
            .field("cbwfx", &self.cbwfx)
            .field("pszTitle", &self.pszTitle)
            .field("szFormatTag", &self.szFormatTag)
            .field("szFormat", &self.szFormat)
            .field("pszName", &self.pszName)
            .field("cchName", &self.cchName)
            .field("fdwEnum", &self.fdwEnum)
            .field("pwfxEnum", &self.pwfxEnum)
            .field("hInstance", &self.hInstance)
            .field("pszTemplateName", &self.pszTemplateName)
            .field("lCustData", &self.lCustData)
            .field("pfnHook", &self.pfnHook)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACMFORMATCHOOSEW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.fdwStyle == other.fdwStyle
            && self.hwndOwner == other.hwndOwner
            && self.pwfx == other.pwfx
            && self.cbwfx == other.cbwfx
            && self.pszTitle == other.pszTitle
            && self.szFormatTag == other.szFormatTag
            && self.szFormat == other.szFormat
            && self.pszName == other.pszName
            && self.cchName == other.cchName
            && self.fdwEnum == other.fdwEnum
            && self.pwfxEnum == other.pwfxEnum
            && self.hInstance == other.hInstance
            && self.pszTemplateName == other.pszTemplateName
            && self.lCustData == other.lCustData
            && self.pfnHook == other.pfnHook
    }
}
impl ::core::cmp::Eq for ACMFORMATCHOOSEW {}
impl FromIntoMemory for ACMFORMATCHOOSEW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 232);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_fdwStyle = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_hwndOwner =
            <super::super::Foundation::HWND as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_pwfx = <MutPtr<WAVEFORMATEX> as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_cbwfx = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_pszTitle = <PCWSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_szFormatTag = <[u16; 48] as FromIntoMemory>::from_bytes(&from[24..24 + 48]);
        let f_szFormat = <[u16; 128] as FromIntoMemory>::from_bytes(&from[72..72 + 128]);
        let f_pszName = <PWSTR as FromIntoMemory>::from_bytes(&from[200..200 + 4]);
        let f_cchName = <u32 as FromIntoMemory>::from_bytes(&from[204..204 + 4]);
        let f_fdwEnum = <u32 as FromIntoMemory>::from_bytes(&from[208..208 + 4]);
        let f_pwfxEnum = <MutPtr<WAVEFORMATEX> as FromIntoMemory>::from_bytes(&from[212..212 + 4]);
        let f_hInstance = <super::super::Foundation::HINSTANCE as FromIntoMemory>::from_bytes(
            &from[216..216 + 4],
        );
        let f_pszTemplateName = <PCWSTR as FromIntoMemory>::from_bytes(&from[220..220 + 4]);
        let f_lCustData =
            <super::super::Foundation::LPARAM as FromIntoMemory>::from_bytes(&from[224..224 + 4]);
        let f_pfnHook =
            <ACMFORMATCHOOSEHOOKPROCW as FromIntoMemory>::from_bytes(&from[228..228 + 4]);
        Self {
            cbStruct: f_cbStruct,
            fdwStyle: f_fdwStyle,
            hwndOwner: f_hwndOwner,
            pwfx: f_pwfx,
            cbwfx: f_cbwfx,
            pszTitle: f_pszTitle,
            szFormatTag: f_szFormatTag,
            szFormat: f_szFormat,
            pszName: f_pszName,
            cchName: f_cchName,
            fdwEnum: f_fdwEnum,
            pwfxEnum: f_pwfxEnum,
            hInstance: f_hInstance,
            pszTemplateName: f_pszTemplateName,
            lCustData: f_lCustData,
            pfnHook: f_pfnHook,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 232);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.fdwStyle, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.hwndOwner, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.pwfx, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.cbwfx, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.pszTitle, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.szFormatTag, &mut into[24..24 + 48]);
        FromIntoMemory::into_bytes(self.szFormat, &mut into[72..72 + 128]);
        FromIntoMemory::into_bytes(self.pszName, &mut into[200..200 + 4]);
        FromIntoMemory::into_bytes(self.cchName, &mut into[204..204 + 4]);
        FromIntoMemory::into_bytes(self.fdwEnum, &mut into[208..208 + 4]);
        FromIntoMemory::into_bytes(self.pwfxEnum, &mut into[212..212 + 4]);
        FromIntoMemory::into_bytes(self.hInstance, &mut into[216..216 + 4]);
        FromIntoMemory::into_bytes(self.pszTemplateName, &mut into[220..220 + 4]);
        FromIntoMemory::into_bytes(self.lCustData, &mut into[224..224 + 4]);
        FromIntoMemory::into_bytes(self.pfnHook, &mut into[228..228 + 4]);
    }
    fn size() -> usize {
        232
    }
}
pub const ACMFORMATCHOOSE_STYLEF_CONTEXTHELP: i32 = 128i32;
pub const ACMFORMATCHOOSE_STYLEF_ENABLEHOOK: i32 = 8i32;
pub const ACMFORMATCHOOSE_STYLEF_ENABLETEMPLATE: i32 = 16i32;
pub const ACMFORMATCHOOSE_STYLEF_ENABLETEMPLATEHANDLE: i32 = 32i32;
pub const ACMFORMATCHOOSE_STYLEF_INITTOWFXSTRUCT: i32 = 64i32;
pub const ACMFORMATCHOOSE_STYLEF_SHOWHELP: i32 = 4i32;
pub struct ACMFORMATDETAILSA {
    pub cbStruct: u32,
    pub dwFormatIndex: u32,
    pub dwFormatTag: u32,
    pub fdwSupport: u32,
    pub pwfx: MutPtr<WAVEFORMATEX>,
    pub cbwfx: u32,
    pub szFormat: [super::super::Foundation::CHAR; 128],
}
impl ::core::marker::Copy for ACMFORMATDETAILSA {}
impl ::core::clone::Clone for ACMFORMATDETAILSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACMFORMATDETAILSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACMFORMATDETAILSA")
            .field("cbStruct", &self.cbStruct)
            .field("dwFormatIndex", &self.dwFormatIndex)
            .field("dwFormatTag", &self.dwFormatTag)
            .field("fdwSupport", &self.fdwSupport)
            .field("pwfx", &self.pwfx)
            .field("cbwfx", &self.cbwfx)
            .field("szFormat", &self.szFormat)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACMFORMATDETAILSA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwFormatIndex == other.dwFormatIndex
            && self.dwFormatTag == other.dwFormatTag
            && self.fdwSupport == other.fdwSupport
            && self.pwfx == other.pwfx
            && self.cbwfx == other.cbwfx
            && self.szFormat == other.szFormat
    }
}
impl ::core::cmp::Eq for ACMFORMATDETAILSA {}
impl FromIntoMemory for ACMFORMATDETAILSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 152);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFormatIndex = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwFormatTag = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_fdwSupport = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_pwfx = <MutPtr<WAVEFORMATEX> as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_cbwfx = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_szFormat = <[super::super::Foundation::CHAR; 128] as FromIntoMemory>::from_bytes(
            &from[24..24 + 128],
        );
        Self {
            cbStruct: f_cbStruct,
            dwFormatIndex: f_dwFormatIndex,
            dwFormatTag: f_dwFormatTag,
            fdwSupport: f_fdwSupport,
            pwfx: f_pwfx,
            cbwfx: f_cbwfx,
            szFormat: f_szFormat,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 152);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFormatIndex, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwFormatTag, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.fdwSupport, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.pwfx, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.cbwfx, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.szFormat, &mut into[24..24 + 128]);
    }
    fn size() -> usize {
        152
    }
}
pub const ACMFORMATDETAILS_FORMAT_CHARS: u32 = 128u32;
pub type ACMFORMATENUMCBA = StdCallFnPtr<
    (HACMDRIVERID, MutPtr<ACMFORMATDETAILSA>, PtrRepr, u32),
    super::super::Foundation::BOOL,
>;
pub type ACMFORMATENUMCBW = StdCallFnPtr<
    (HACMDRIVERID, MutPtr<tACMFORMATDETAILSW>, PtrRepr, u32),
    super::super::Foundation::BOOL,
>;
pub struct ACMFORMATTAGDETAILSA {
    pub cbStruct: u32,
    pub dwFormatTagIndex: u32,
    pub dwFormatTag: u32,
    pub cbFormatSize: u32,
    pub fdwSupport: u32,
    pub cStandardFormats: u32,
    pub szFormatTag: [super::super::Foundation::CHAR; 48],
}
impl ::core::marker::Copy for ACMFORMATTAGDETAILSA {}
impl ::core::clone::Clone for ACMFORMATTAGDETAILSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACMFORMATTAGDETAILSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACMFORMATTAGDETAILSA")
            .field("cbStruct", &self.cbStruct)
            .field("dwFormatTagIndex", &self.dwFormatTagIndex)
            .field("dwFormatTag", &self.dwFormatTag)
            .field("cbFormatSize", &self.cbFormatSize)
            .field("fdwSupport", &self.fdwSupport)
            .field("cStandardFormats", &self.cStandardFormats)
            .field("szFormatTag", &self.szFormatTag)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACMFORMATTAGDETAILSA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwFormatTagIndex == other.dwFormatTagIndex
            && self.dwFormatTag == other.dwFormatTag
            && self.cbFormatSize == other.cbFormatSize
            && self.fdwSupport == other.fdwSupport
            && self.cStandardFormats == other.cStandardFormats
            && self.szFormatTag == other.szFormatTag
    }
}
impl ::core::cmp::Eq for ACMFORMATTAGDETAILSA {}
impl FromIntoMemory for ACMFORMATTAGDETAILSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 72);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFormatTagIndex = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwFormatTag = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_cbFormatSize = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_fdwSupport = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_cStandardFormats = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_szFormatTag = <[super::super::Foundation::CHAR; 48] as FromIntoMemory>::from_bytes(
            &from[24..24 + 48],
        );
        Self {
            cbStruct: f_cbStruct,
            dwFormatTagIndex: f_dwFormatTagIndex,
            dwFormatTag: f_dwFormatTag,
            cbFormatSize: f_cbFormatSize,
            fdwSupport: f_fdwSupport,
            cStandardFormats: f_cStandardFormats,
            szFormatTag: f_szFormatTag,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 72);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFormatTagIndex, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwFormatTag, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.cbFormatSize, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.fdwSupport, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.cStandardFormats, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.szFormatTag, &mut into[24..24 + 48]);
    }
    fn size() -> usize {
        72
    }
}
pub struct ACMFORMATTAGDETAILSW {
    pub cbStruct: u32,
    pub dwFormatTagIndex: u32,
    pub dwFormatTag: u32,
    pub cbFormatSize: u32,
    pub fdwSupport: u32,
    pub cStandardFormats: u32,
    pub szFormatTag: [u16; 48],
}
impl ::core::marker::Copy for ACMFORMATTAGDETAILSW {}
impl ::core::clone::Clone for ACMFORMATTAGDETAILSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACMFORMATTAGDETAILSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACMFORMATTAGDETAILSW")
            .field("cbStruct", &self.cbStruct)
            .field("dwFormatTagIndex", &self.dwFormatTagIndex)
            .field("dwFormatTag", &self.dwFormatTag)
            .field("cbFormatSize", &self.cbFormatSize)
            .field("fdwSupport", &self.fdwSupport)
            .field("cStandardFormats", &self.cStandardFormats)
            .field("szFormatTag", &self.szFormatTag)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACMFORMATTAGDETAILSW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwFormatTagIndex == other.dwFormatTagIndex
            && self.dwFormatTag == other.dwFormatTag
            && self.cbFormatSize == other.cbFormatSize
            && self.fdwSupport == other.fdwSupport
            && self.cStandardFormats == other.cStandardFormats
            && self.szFormatTag == other.szFormatTag
    }
}
impl ::core::cmp::Eq for ACMFORMATTAGDETAILSW {}
impl FromIntoMemory for ACMFORMATTAGDETAILSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 72);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFormatTagIndex = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwFormatTag = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_cbFormatSize = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_fdwSupport = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_cStandardFormats = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_szFormatTag = <[u16; 48] as FromIntoMemory>::from_bytes(&from[24..24 + 48]);
        Self {
            cbStruct: f_cbStruct,
            dwFormatTagIndex: f_dwFormatTagIndex,
            dwFormatTag: f_dwFormatTag,
            cbFormatSize: f_cbFormatSize,
            fdwSupport: f_fdwSupport,
            cStandardFormats: f_cStandardFormats,
            szFormatTag: f_szFormatTag,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 72);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFormatTagIndex, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwFormatTag, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.cbFormatSize, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.fdwSupport, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.cStandardFormats, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.szFormatTag, &mut into[24..24 + 48]);
    }
    fn size() -> usize {
        72
    }
}
pub const ACMFORMATTAGDETAILS_FORMATTAG_CHARS: u32 = 48u32;
pub type ACMFORMATTAGENUMCBA = StdCallFnPtr<
    (HACMDRIVERID, MutPtr<ACMFORMATTAGDETAILSA>, PtrRepr, u32),
    super::super::Foundation::BOOL,
>;
pub type ACMFORMATTAGENUMCBW = StdCallFnPtr<
    (HACMDRIVERID, MutPtr<ACMFORMATTAGDETAILSW>, PtrRepr, u32),
    super::super::Foundation::BOOL,
>;
pub const ACMHELPMSGCONTEXTHELP: &'static str = "acmchoose_contexthelp";
pub const ACMHELPMSGCONTEXTHELPA: &'static str = "acmchoose_contexthelp";
pub const ACMHELPMSGCONTEXTHELPW: &'static str = "acmchoose_contexthelp";
pub const ACMHELPMSGCONTEXTMENU: &'static str = "acmchoose_contextmenu";
pub const ACMHELPMSGCONTEXTMENUA: &'static str = "acmchoose_contextmenu";
pub const ACMHELPMSGCONTEXTMENUW: &'static str = "acmchoose_contextmenu";
pub const ACMHELPMSGSTRING: &'static str = "acmchoose_help";
pub const ACMHELPMSGSTRINGA: &'static str = "acmchoose_help";
pub const ACMHELPMSGSTRINGW: &'static str = "acmchoose_help";
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ACMSTREAMHEADER {
    pub cbStruct: u32,
    pub fdwStatus: u32,
    pub dwUser: PtrRepr,
    pub pbSrc: MutPtr<u8>,
    pub cbSrcLength: u32,
    pub cbSrcLengthUsed: u32,
    pub dwSrcUser: PtrRepr,
    pub pbDst: MutPtr<u8>,
    pub cbDstLength: u32,
    pub cbDstLengthUsed: u32,
    pub dwDstUser: PtrRepr,
    pub dwReservedDriver: [u32; 15],
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ACMSTREAMHEADER {}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ACMSTREAMHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ACMSTREAMHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACMSTREAMHEADER")
            .field("cbStruct", &self.cbStruct)
            .field("fdwStatus", &self.fdwStatus)
            .field("dwUser", &self.dwUser)
            .field("pbSrc", &self.pbSrc)
            .field("cbSrcLength", &self.cbSrcLength)
            .field("cbSrcLengthUsed", &self.cbSrcLengthUsed)
            .field("dwSrcUser", &self.dwSrcUser)
            .field("pbDst", &self.pbDst)
            .field("cbDstLength", &self.cbDstLength)
            .field("cbDstLengthUsed", &self.cbDstLengthUsed)
            .field("dwDstUser", &self.dwDstUser)
            .field("dwReservedDriver", &self.dwReservedDriver)
            .finish()
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ACMSTREAMHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.fdwStatus == other.fdwStatus
            && self.dwUser == other.dwUser
            && self.pbSrc == other.pbSrc
            && self.cbSrcLength == other.cbSrcLength
            && self.cbSrcLengthUsed == other.cbSrcLengthUsed
            && self.dwSrcUser == other.dwSrcUser
            && self.pbDst == other.pbDst
            && self.cbDstLength == other.cbDstLength
            && self.cbDstLengthUsed == other.cbDstLengthUsed
            && self.dwDstUser == other.dwDstUser
            && self.dwReservedDriver == other.dwReservedDriver
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ACMSTREAMHEADER {}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ACMSTREAMHEADER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 104);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_fdwStatus = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwUser = <PtrRepr as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_pbSrc = <MutPtr<u8> as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_cbSrcLength = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_cbSrcLengthUsed = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwSrcUser = <PtrRepr as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_pbDst = <MutPtr<u8> as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_cbDstLength = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_cbDstLengthUsed = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_dwDstUser = <PtrRepr as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_dwReservedDriver = <[u32; 15] as FromIntoMemory>::from_bytes(&from[44..44 + 60]);
        Self {
            cbStruct: f_cbStruct,
            fdwStatus: f_fdwStatus,
            dwUser: f_dwUser,
            pbSrc: f_pbSrc,
            cbSrcLength: f_cbSrcLength,
            cbSrcLengthUsed: f_cbSrcLengthUsed,
            dwSrcUser: f_dwSrcUser,
            pbDst: f_pbDst,
            cbDstLength: f_cbDstLength,
            cbDstLengthUsed: f_cbDstLengthUsed,
            dwDstUser: f_dwDstUser,
            dwReservedDriver: f_dwReservedDriver,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 104);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.fdwStatus, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwUser, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.pbSrc, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.cbSrcLength, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.cbSrcLengthUsed, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwSrcUser, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.pbDst, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.cbDstLength, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.cbDstLengthUsed, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.dwDstUser, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.dwReservedDriver, &mut into[44..44 + 60]);
    }
    fn size() -> usize {
        104
    }
}
pub struct ACMSTREAMHEADER {
    pub cbStruct: u32,
    pub fdwStatus: u32,
    pub dwUser: PtrRepr,
    pub pbSrc: MutPtr<u8>,
    pub cbSrcLength: u32,
    pub cbSrcLengthUsed: u32,
    pub dwSrcUser: PtrRepr,
    pub pbDst: MutPtr<u8>,
    pub cbDstLength: u32,
    pub cbDstLengthUsed: u32,
    pub dwDstUser: PtrRepr,
    pub dwReservedDriver: [u32; 10],
}
impl ::core::marker::Copy for ACMSTREAMHEADER {}
impl ::core::clone::Clone for ACMSTREAMHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACMSTREAMHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACMSTREAMHEADER")
            .field("cbStruct", &self.cbStruct)
            .field("fdwStatus", &self.fdwStatus)
            .field("dwUser", &self.dwUser)
            .field("pbSrc", &self.pbSrc)
            .field("cbSrcLength", &self.cbSrcLength)
            .field("cbSrcLengthUsed", &self.cbSrcLengthUsed)
            .field("dwSrcUser", &self.dwSrcUser)
            .field("pbDst", &self.pbDst)
            .field("cbDstLength", &self.cbDstLength)
            .field("cbDstLengthUsed", &self.cbDstLengthUsed)
            .field("dwDstUser", &self.dwDstUser)
            .field("dwReservedDriver", &self.dwReservedDriver)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACMSTREAMHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.fdwStatus == other.fdwStatus
            && self.dwUser == other.dwUser
            && self.pbSrc == other.pbSrc
            && self.cbSrcLength == other.cbSrcLength
            && self.cbSrcLengthUsed == other.cbSrcLengthUsed
            && self.dwSrcUser == other.dwSrcUser
            && self.pbDst == other.pbDst
            && self.cbDstLength == other.cbDstLength
            && self.cbDstLengthUsed == other.cbDstLengthUsed
            && self.dwDstUser == other.dwDstUser
            && self.dwReservedDriver == other.dwReservedDriver
    }
}
impl ::core::cmp::Eq for ACMSTREAMHEADER {}
impl FromIntoMemory for ACMSTREAMHEADER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 84);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_fdwStatus = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwUser = <PtrRepr as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_pbSrc = <MutPtr<u8> as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_cbSrcLength = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_cbSrcLengthUsed = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwSrcUser = <PtrRepr as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_pbDst = <MutPtr<u8> as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_cbDstLength = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_cbDstLengthUsed = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_dwDstUser = <PtrRepr as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_dwReservedDriver = <[u32; 10] as FromIntoMemory>::from_bytes(&from[44..44 + 40]);
        Self {
            cbStruct: f_cbStruct,
            fdwStatus: f_fdwStatus,
            dwUser: f_dwUser,
            pbSrc: f_pbSrc,
            cbSrcLength: f_cbSrcLength,
            cbSrcLengthUsed: f_cbSrcLengthUsed,
            dwSrcUser: f_dwSrcUser,
            pbDst: f_pbDst,
            cbDstLength: f_cbDstLength,
            cbDstLengthUsed: f_cbDstLengthUsed,
            dwDstUser: f_dwDstUser,
            dwReservedDriver: f_dwReservedDriver,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 84);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.fdwStatus, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwUser, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.pbSrc, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.cbSrcLength, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.cbSrcLengthUsed, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwSrcUser, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.pbDst, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.cbDstLength, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.cbDstLengthUsed, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.dwDstUser, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.dwReservedDriver, &mut into[44..44 + 40]);
    }
    fn size() -> usize {
        84
    }
}
pub const ACMSTREAMHEADER_STATUSF_DONE: i32 = 65536i32;
pub const ACMSTREAMHEADER_STATUSF_INQUEUE: i32 = 1048576i32;
pub const ACMSTREAMHEADER_STATUSF_PREPARED: i32 = 131072i32;
pub const ACM_DRIVERADDF_FUNCTION: i32 = 3i32;
pub const ACM_DRIVERADDF_GLOBAL: i32 = 8i32;
pub const ACM_DRIVERADDF_LOCAL: i32 = 0i32;
pub const ACM_DRIVERADDF_NAME: i32 = 1i32;
pub const ACM_DRIVERADDF_NOTIFYHWND: i32 = 4i32;
pub const ACM_DRIVERADDF_TYPEMASK: i32 = 7i32;
pub const ACM_DRIVERENUMF_DISABLED: i32 = -2147483648i32;
pub const ACM_DRIVERENUMF_NOLOCAL: i32 = 1073741824i32;
pub const ACM_DRIVERPRIORITYF_ABLEMASK: i32 = 3i32;
pub const ACM_DRIVERPRIORITYF_BEGIN: i32 = 65536i32;
pub const ACM_DRIVERPRIORITYF_DEFERMASK: i32 = 196608i32;
pub const ACM_DRIVERPRIORITYF_DISABLE: i32 = 2i32;
pub const ACM_DRIVERPRIORITYF_ENABLE: i32 = 1i32;
pub const ACM_DRIVERPRIORITYF_END: i32 = 131072i32;
pub const ACM_FILTERDETAILSF_FILTER: i32 = 1i32;
pub const ACM_FILTERDETAILSF_INDEX: i32 = 0i32;
pub const ACM_FILTERDETAILSF_QUERYMASK: i32 = 15i32;
pub const ACM_FILTERENUMF_DWFILTERTAG: i32 = 65536i32;
pub const ACM_FILTERTAGDETAILSF_FILTERTAG: i32 = 1i32;
pub const ACM_FILTERTAGDETAILSF_INDEX: i32 = 0i32;
pub const ACM_FILTERTAGDETAILSF_LARGESTSIZE: i32 = 2i32;
pub const ACM_FILTERTAGDETAILSF_QUERYMASK: i32 = 15i32;
pub const ACM_FORMATDETAILSF_FORMAT: i32 = 1i32;
pub const ACM_FORMATDETAILSF_INDEX: i32 = 0i32;
pub const ACM_FORMATDETAILSF_QUERYMASK: i32 = 15i32;
pub const ACM_FORMATENUMF_CONVERT: i32 = 1048576i32;
pub const ACM_FORMATENUMF_HARDWARE: i32 = 4194304i32;
pub const ACM_FORMATENUMF_INPUT: i32 = 8388608i32;
pub const ACM_FORMATENUMF_NCHANNELS: i32 = 131072i32;
pub const ACM_FORMATENUMF_NSAMPLESPERSEC: i32 = 262144i32;
pub const ACM_FORMATENUMF_OUTPUT: i32 = 16777216i32;
pub const ACM_FORMATENUMF_SUGGEST: i32 = 2097152i32;
pub const ACM_FORMATENUMF_WBITSPERSAMPLE: i32 = 524288i32;
pub const ACM_FORMATENUMF_WFORMATTAG: i32 = 65536i32;
pub const ACM_FORMATSUGGESTF_NCHANNELS: i32 = 131072i32;
pub const ACM_FORMATSUGGESTF_NSAMPLESPERSEC: i32 = 262144i32;
pub const ACM_FORMATSUGGESTF_TYPEMASK: i32 = 16711680i32;
pub const ACM_FORMATSUGGESTF_WBITSPERSAMPLE: i32 = 524288i32;
pub const ACM_FORMATSUGGESTF_WFORMATTAG: i32 = 65536i32;
pub const ACM_FORMATTAGDETAILSF_FORMATTAG: i32 = 1i32;
pub const ACM_FORMATTAGDETAILSF_INDEX: i32 = 0i32;
pub const ACM_FORMATTAGDETAILSF_LARGESTSIZE: i32 = 2i32;
pub const ACM_FORMATTAGDETAILSF_QUERYMASK: i32 = 15i32;
pub const ACM_METRIC_COUNT_CODECS: u32 = 2u32;
pub const ACM_METRIC_COUNT_CONVERTERS: u32 = 3u32;
pub const ACM_METRIC_COUNT_DISABLED: u32 = 5u32;
pub const ACM_METRIC_COUNT_DRIVERS: u32 = 1u32;
pub const ACM_METRIC_COUNT_FILTERS: u32 = 4u32;
pub const ACM_METRIC_COUNT_HARDWARE: u32 = 6u32;
pub const ACM_METRIC_COUNT_LOCAL_CODECS: u32 = 21u32;
pub const ACM_METRIC_COUNT_LOCAL_CONVERTERS: u32 = 22u32;
pub const ACM_METRIC_COUNT_LOCAL_DISABLED: u32 = 24u32;
pub const ACM_METRIC_COUNT_LOCAL_DRIVERS: u32 = 20u32;
pub const ACM_METRIC_COUNT_LOCAL_FILTERS: u32 = 23u32;
pub const ACM_METRIC_DRIVER_PRIORITY: u32 = 101u32;
pub const ACM_METRIC_DRIVER_SUPPORT: u32 = 100u32;
pub const ACM_METRIC_HARDWARE_WAVE_INPUT: u32 = 30u32;
pub const ACM_METRIC_HARDWARE_WAVE_OUTPUT: u32 = 31u32;
pub const ACM_METRIC_MAX_SIZE_FILTER: u32 = 51u32;
pub const ACM_METRIC_MAX_SIZE_FORMAT: u32 = 50u32;
pub const ACM_STREAMCONVERTF_BLOCKALIGN: u32 = 4u32;
pub const ACM_STREAMCONVERTF_END: u32 = 32u32;
pub const ACM_STREAMCONVERTF_START: u32 = 16u32;
pub const ACM_STREAMOPENF_ASYNC: u32 = 2u32;
pub const ACM_STREAMOPENF_NONREALTIME: u32 = 4u32;
pub const ACM_STREAMOPENF_QUERY: u32 = 1u32;
pub const ACM_STREAMSIZEF_DESTINATION: i32 = 1i32;
pub const ACM_STREAMSIZEF_QUERYMASK: i32 = 15i32;
pub const ACM_STREAMSIZEF_SOURCE: i32 = 0i32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AMBISONICS_CHANNEL_ORDERING(pub i32);
pub const AMBISONICS_CHANNEL_ORDERING_ACN: AMBISONICS_CHANNEL_ORDERING =
    AMBISONICS_CHANNEL_ORDERING(0i32);
impl ::core::marker::Copy for AMBISONICS_CHANNEL_ORDERING {}
impl ::core::clone::Clone for AMBISONICS_CHANNEL_ORDERING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AMBISONICS_CHANNEL_ORDERING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AMBISONICS_CHANNEL_ORDERING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AMBISONICS_CHANNEL_ORDERING")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for AMBISONICS_CHANNEL_ORDERING {
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
pub struct AMBISONICS_NORMALIZATION(pub i32);
pub const AMBISONICS_NORMALIZATION_SN3D: AMBISONICS_NORMALIZATION = AMBISONICS_NORMALIZATION(0i32);
pub const AMBISONICS_NORMALIZATION_N3D: AMBISONICS_NORMALIZATION = AMBISONICS_NORMALIZATION(1i32);
impl ::core::marker::Copy for AMBISONICS_NORMALIZATION {}
impl ::core::clone::Clone for AMBISONICS_NORMALIZATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AMBISONICS_NORMALIZATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AMBISONICS_NORMALIZATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AMBISONICS_NORMALIZATION")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for AMBISONICS_NORMALIZATION {
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
pub struct AMBISONICS_PARAMS {
    pub u32Size: u32,
    pub u32Version: u32,
    pub u32Type: AMBISONICS_TYPE,
    pub u32ChannelOrdering: AMBISONICS_CHANNEL_ORDERING,
    pub u32Normalization: AMBISONICS_NORMALIZATION,
    pub u32Order: u32,
    pub u32NumChannels: u32,
    pub pu32ChannelMap: MutPtr<u32>,
}
impl ::core::marker::Copy for AMBISONICS_PARAMS {}
impl ::core::clone::Clone for AMBISONICS_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AMBISONICS_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AMBISONICS_PARAMS")
            .field("u32Size", &self.u32Size)
            .field("u32Version", &self.u32Version)
            .field("u32Type", &self.u32Type)
            .field("u32ChannelOrdering", &self.u32ChannelOrdering)
            .field("u32Normalization", &self.u32Normalization)
            .field("u32Order", &self.u32Order)
            .field("u32NumChannels", &self.u32NumChannels)
            .field("pu32ChannelMap", &self.pu32ChannelMap)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AMBISONICS_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.u32Size == other.u32Size
            && self.u32Version == other.u32Version
            && self.u32Type == other.u32Type
            && self.u32ChannelOrdering == other.u32ChannelOrdering
            && self.u32Normalization == other.u32Normalization
            && self.u32Order == other.u32Order
            && self.u32NumChannels == other.u32NumChannels
            && self.pu32ChannelMap == other.pu32ChannelMap
    }
}
impl ::core::cmp::Eq for AMBISONICS_PARAMS {}
impl FromIntoMemory for AMBISONICS_PARAMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_u32Size = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_u32Version = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_u32Type = <AMBISONICS_TYPE as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_u32ChannelOrdering =
            <AMBISONICS_CHANNEL_ORDERING as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_u32Normalization =
            <AMBISONICS_NORMALIZATION as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_u32Order = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_u32NumChannels = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_pu32ChannelMap = <MutPtr<u32> as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            u32Size: f_u32Size,
            u32Version: f_u32Version,
            u32Type: f_u32Type,
            u32ChannelOrdering: f_u32ChannelOrdering,
            u32Normalization: f_u32Normalization,
            u32Order: f_u32Order,
            u32NumChannels: f_u32NumChannels,
            pu32ChannelMap: f_pu32ChannelMap,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.u32Size, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.u32Version, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.u32Type, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.u32ChannelOrdering, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.u32Normalization, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.u32Order, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.u32NumChannels, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.pu32ChannelMap, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
pub const AMBISONICS_PARAM_VERSION_1: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AMBISONICS_TYPE(pub i32);
pub const AMBISONICS_TYPE_FULL3D: AMBISONICS_TYPE = AMBISONICS_TYPE(0i32);
impl ::core::marker::Copy for AMBISONICS_TYPE {}
impl ::core::clone::Clone for AMBISONICS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AMBISONICS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AMBISONICS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AMBISONICS_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for AMBISONICS_TYPE {
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
pub const AUDCLNT_E_ALREADY_INITIALIZED: crate::core::HRESULT =
    crate::core::HRESULT(-2004287486i32);
pub const AUDCLNT_E_BUFDURATION_PERIOD_NOT_EQUAL: crate::core::HRESULT =
    crate::core::HRESULT(-2004287469i32);
pub const AUDCLNT_E_BUFFER_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2004287464i32);
pub const AUDCLNT_E_BUFFER_OPERATION_PENDING: crate::core::HRESULT =
    crate::core::HRESULT(-2004287477i32);
pub const AUDCLNT_E_BUFFER_SIZE_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2004287466i32);
pub const AUDCLNT_E_BUFFER_SIZE_NOT_ALIGNED: crate::core::HRESULT =
    crate::core::HRESULT(-2004287463i32);
pub const AUDCLNT_E_BUFFER_TOO_LARGE: crate::core::HRESULT = crate::core::HRESULT(-2004287482i32);
pub const AUDCLNT_E_CPUUSAGE_EXCEEDED: crate::core::HRESULT = crate::core::HRESULT(-2004287465i32);
pub const AUDCLNT_E_DEVICE_INVALIDATED: crate::core::HRESULT = crate::core::HRESULT(-2004287484i32);
pub const AUDCLNT_E_DEVICE_IN_USE: crate::core::HRESULT = crate::core::HRESULT(-2004287478i32);
pub const AUDCLNT_E_EFFECT_NOT_AVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2004287423i32);
pub const AUDCLNT_E_EFFECT_STATE_READ_ONLY: crate::core::HRESULT =
    crate::core::HRESULT(-2004287422i32);
pub const AUDCLNT_E_ENDPOINT_CREATE_FAILED: crate::core::HRESULT =
    crate::core::HRESULT(-2004287473i32);
pub const AUDCLNT_E_ENDPOINT_OFFLOAD_NOT_CAPABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2004287454i32);
pub const AUDCLNT_E_ENGINE_FORMAT_LOCKED: crate::core::HRESULT =
    crate::core::HRESULT(-2004287447i32);
pub const AUDCLNT_E_ENGINE_PERIODICITY_LOCKED: crate::core::HRESULT =
    crate::core::HRESULT(-2004287448i32);
pub const AUDCLNT_E_EVENTHANDLE_NOT_EXPECTED: crate::core::HRESULT =
    crate::core::HRESULT(-2004287471i32);
pub const AUDCLNT_E_EVENTHANDLE_NOT_SET: crate::core::HRESULT =
    crate::core::HRESULT(-2004287468i32);
pub const AUDCLNT_E_EXCLUSIVE_MODE_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-2004287474i32);
pub const AUDCLNT_E_EXCLUSIVE_MODE_ONLY: crate::core::HRESULT =
    crate::core::HRESULT(-2004287470i32);
pub const AUDCLNT_E_HEADTRACKING_ENABLED: crate::core::HRESULT =
    crate::core::HRESULT(-2004287440i32);
pub const AUDCLNT_E_HEADTRACKING_UNSUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2004287424i32);
pub const AUDCLNT_E_INCORRECT_BUFFER_SIZE: crate::core::HRESULT =
    crate::core::HRESULT(-2004287467i32);
pub const AUDCLNT_E_INVALID_DEVICE_PERIOD: crate::core::HRESULT =
    crate::core::HRESULT(-2004287456i32);
pub const AUDCLNT_E_INVALID_SIZE: crate::core::HRESULT = crate::core::HRESULT(-2004287479i32);
pub const AUDCLNT_E_INVALID_STREAM_FLAG: crate::core::HRESULT =
    crate::core::HRESULT(-2004287455i32);
pub const AUDCLNT_E_NONOFFLOAD_MODE_ONLY: crate::core::HRESULT =
    crate::core::HRESULT(-2004287451i32);
pub const AUDCLNT_E_NOT_INITIALIZED: crate::core::HRESULT = crate::core::HRESULT(-2004287487i32);
pub const AUDCLNT_E_NOT_STOPPED: crate::core::HRESULT = crate::core::HRESULT(-2004287483i32);
pub const AUDCLNT_E_OFFLOAD_MODE_ONLY: crate::core::HRESULT = crate::core::HRESULT(-2004287452i32);
pub const AUDCLNT_E_OUT_OF_OFFLOAD_RESOURCES: crate::core::HRESULT =
    crate::core::HRESULT(-2004287453i32);
pub const AUDCLNT_E_OUT_OF_ORDER: crate::core::HRESULT = crate::core::HRESULT(-2004287481i32);
pub const AUDCLNT_E_RAW_MODE_UNSUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2004287449i32);
pub const AUDCLNT_E_RESOURCES_INVALIDATED: crate::core::HRESULT =
    crate::core::HRESULT(-2004287450i32);
pub const AUDCLNT_E_SERVICE_NOT_RUNNING: crate::core::HRESULT =
    crate::core::HRESULT(-2004287472i32);
pub const AUDCLNT_E_THREAD_NOT_REGISTERED: crate::core::HRESULT =
    crate::core::HRESULT(-2004287476i32);
pub const AUDCLNT_E_UNSUPPORTED_FORMAT: crate::core::HRESULT = crate::core::HRESULT(-2004287480i32);
pub const AUDCLNT_E_WRONG_ENDPOINT_TYPE: crate::core::HRESULT =
    crate::core::HRESULT(-2004287485i32);
pub const AUDCLNT_SESSIONFLAGS_DISPLAY_HIDE: u32 = 536870912u32;
pub const AUDCLNT_SESSIONFLAGS_DISPLAY_HIDEWHENEXPIRED: u32 = 1073741824u32;
pub const AUDCLNT_SESSIONFLAGS_EXPIREWHENUNOWNED: u32 = 268435456u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUDCLNT_SHAREMODE(pub i32);
pub const AUDCLNT_SHAREMODE_SHARED: AUDCLNT_SHAREMODE = AUDCLNT_SHAREMODE(0i32);
pub const AUDCLNT_SHAREMODE_EXCLUSIVE: AUDCLNT_SHAREMODE = AUDCLNT_SHAREMODE(1i32);
impl ::core::marker::Copy for AUDCLNT_SHAREMODE {}
impl ::core::clone::Clone for AUDCLNT_SHAREMODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDCLNT_SHAREMODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AUDCLNT_SHAREMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDCLNT_SHAREMODE").field(&self.0).finish()
    }
}
impl FromIntoMemory for AUDCLNT_SHAREMODE {
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
pub const AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM: u32 = 2147483648u32;
pub const AUDCLNT_STREAMFLAGS_CROSSPROCESS: u32 = 65536u32;
pub const AUDCLNT_STREAMFLAGS_EVENTCALLBACK: u32 = 262144u32;
pub const AUDCLNT_STREAMFLAGS_LOOPBACK: u32 = 131072u32;
pub const AUDCLNT_STREAMFLAGS_NOPERSIST: u32 = 524288u32;
pub const AUDCLNT_STREAMFLAGS_RATEADJUST: u32 = 1048576u32;
pub const AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY: u32 = 134217728u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUDCLNT_STREAMOPTIONS(pub u32);
pub const AUDCLNT_STREAMOPTIONS_NONE: AUDCLNT_STREAMOPTIONS = AUDCLNT_STREAMOPTIONS(0u32);
pub const AUDCLNT_STREAMOPTIONS_RAW: AUDCLNT_STREAMOPTIONS = AUDCLNT_STREAMOPTIONS(1u32);
pub const AUDCLNT_STREAMOPTIONS_MATCH_FORMAT: AUDCLNT_STREAMOPTIONS = AUDCLNT_STREAMOPTIONS(2u32);
pub const AUDCLNT_STREAMOPTIONS_AMBISONICS: AUDCLNT_STREAMOPTIONS = AUDCLNT_STREAMOPTIONS(4u32);
impl ::core::marker::Copy for AUDCLNT_STREAMOPTIONS {}
impl ::core::clone::Clone for AUDCLNT_STREAMOPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDCLNT_STREAMOPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AUDCLNT_STREAMOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDCLNT_STREAMOPTIONS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for AUDCLNT_STREAMOPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AUDCLNT_STREAMOPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AUDCLNT_STREAMOPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AUDCLNT_STREAMOPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AUDCLNT_STREAMOPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for AUDCLNT_STREAMOPTIONS {
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
pub const AUDCLNT_S_BUFFER_EMPTY: crate::core::HRESULT = crate::core::HRESULT(143196161i32);
pub const AUDCLNT_S_POSITION_STALLED: crate::core::HRESULT = crate::core::HRESULT(143196163i32);
pub const AUDCLNT_S_THREAD_ALREADY_REGISTERED: crate::core::HRESULT =
    crate::core::HRESULT(143196162i32);
pub struct AUDIOCLIENT_ACTIVATION_PARAMS {
    pub ActivationType: AUDIOCLIENT_ACTIVATION_TYPE,
    pub Anonymous: AUDIOCLIENT_ACTIVATION_PARAMS_0,
}
impl ::core::marker::Copy for AUDIOCLIENT_ACTIVATION_PARAMS {}
impl ::core::clone::Clone for AUDIOCLIENT_ACTIVATION_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUDIOCLIENT_ACTIVATION_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIOCLIENT_ACTIVATION_PARAMS")
            .field("ActivationType", &self.ActivationType)
            .field("Anonymous", &self.Anonymous)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AUDIOCLIENT_ACTIVATION_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.ActivationType == other.ActivationType && self.Anonymous == other.Anonymous
    }
}
impl ::core::cmp::Eq for AUDIOCLIENT_ACTIVATION_PARAMS {}
impl FromIntoMemory for AUDIOCLIENT_ACTIVATION_PARAMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_ActivationType =
            <AUDIOCLIENT_ACTIVATION_TYPE as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Anonymous =
            <AUDIOCLIENT_ACTIVATION_PARAMS_0 as FromIntoMemory>::from_bytes(&from[4..4 + 8]);
        Self {
            ActivationType: f_ActivationType,
            Anonymous: f_Anonymous,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.ActivationType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[4..4 + 8]);
    }
    fn size() -> usize {
        12
    }
}
pub struct AUDIOCLIENT_ACTIVATION_PARAMS_0 {
    data: [u8; 8],
}
impl ::core::default::Default for AUDIOCLIENT_ACTIVATION_PARAMS_0 {
    fn default() -> Self {
        Self { data: [0u8; 8] }
    }
}
impl ::core::marker::Copy for AUDIOCLIENT_ACTIVATION_PARAMS_0 {}
impl ::core::clone::Clone for AUDIOCLIENT_ACTIVATION_PARAMS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUDIOCLIENT_ACTIVATION_PARAMS_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIOCLIENT_ACTIVATION_PARAMS_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AUDIOCLIENT_ACTIVATION_PARAMS_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for AUDIOCLIENT_ACTIVATION_PARAMS_0 {}
impl FromIntoMemory for AUDIOCLIENT_ACTIVATION_PARAMS_0 {
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
pub struct AUDIOCLIENT_ACTIVATION_TYPE(pub i32);
pub const AUDIOCLIENT_ACTIVATION_TYPE_DEFAULT: AUDIOCLIENT_ACTIVATION_TYPE =
    AUDIOCLIENT_ACTIVATION_TYPE(0i32);
pub const AUDIOCLIENT_ACTIVATION_TYPE_PROCESS_LOOPBACK: AUDIOCLIENT_ACTIVATION_TYPE =
    AUDIOCLIENT_ACTIVATION_TYPE(1i32);
impl ::core::marker::Copy for AUDIOCLIENT_ACTIVATION_TYPE {}
impl ::core::clone::Clone for AUDIOCLIENT_ACTIVATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDIOCLIENT_ACTIVATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AUDIOCLIENT_ACTIVATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIOCLIENT_ACTIVATION_TYPE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for AUDIOCLIENT_ACTIVATION_TYPE {
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
pub struct AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    pub TargetProcessId: u32,
    pub ProcessLoopbackMode: PROCESS_LOOPBACK_MODE,
}
impl ::core::marker::Copy for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {}
impl ::core::clone::Clone for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS")
            .field("TargetProcessId", &self.TargetProcessId)
            .field("ProcessLoopbackMode", &self.ProcessLoopbackMode)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.TargetProcessId == other.TargetProcessId
            && self.ProcessLoopbackMode == other.ProcessLoopbackMode
    }
}
impl ::core::cmp::Eq for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {}
impl FromIntoMemory for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_TargetProcessId = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ProcessLoopbackMode =
            <PROCESS_LOOPBACK_MODE as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            TargetProcessId: f_TargetProcessId,
            ProcessLoopbackMode: f_ProcessLoopbackMode,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.TargetProcessId, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ProcessLoopbackMode, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub const AUDIOCLOCK_CHARACTERISTIC_FIXED_FREQ: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUDIO_DUCKING_OPTIONS(pub u32);
pub const AUDIO_DUCKING_OPTIONS_DEFAULT: AUDIO_DUCKING_OPTIONS = AUDIO_DUCKING_OPTIONS(0u32);
pub const AUDIO_DUCKING_OPTIONS_DO_NOT_DUCK_OTHER_STREAMS: AUDIO_DUCKING_OPTIONS =
    AUDIO_DUCKING_OPTIONS(1u32);
impl ::core::marker::Copy for AUDIO_DUCKING_OPTIONS {}
impl ::core::clone::Clone for AUDIO_DUCKING_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDIO_DUCKING_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AUDIO_DUCKING_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIO_DUCKING_OPTIONS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for AUDIO_DUCKING_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AUDIO_DUCKING_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AUDIO_DUCKING_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AUDIO_DUCKING_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AUDIO_DUCKING_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for AUDIO_DUCKING_OPTIONS {
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
pub struct AUDIO_EFFECT {
    pub id: crate::core::GUID,
    pub canSetState: super::super::Foundation::BOOL,
    pub state: AUDIO_EFFECT_STATE,
}
impl ::core::marker::Copy for AUDIO_EFFECT {}
impl ::core::clone::Clone for AUDIO_EFFECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUDIO_EFFECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_EFFECT")
            .field("id", &self.id)
            .field("canSetState", &self.canSetState)
            .field("state", &self.state)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AUDIO_EFFECT {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.canSetState == other.canSetState && self.state == other.state
    }
}
impl ::core::cmp::Eq for AUDIO_EFFECT {}
impl FromIntoMemory for AUDIO_EFFECT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_id = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_canSetState =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_state = <AUDIO_EFFECT_STATE as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            id: f_id,
            canSetState: f_canSetState,
            state: f_state,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.id, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.canSetState, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.state, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUDIO_EFFECT_STATE(pub i32);
pub const AUDIO_EFFECT_STATE_OFF: AUDIO_EFFECT_STATE = AUDIO_EFFECT_STATE(0i32);
pub const AUDIO_EFFECT_STATE_ON: AUDIO_EFFECT_STATE = AUDIO_EFFECT_STATE(1i32);
impl ::core::marker::Copy for AUDIO_EFFECT_STATE {}
impl ::core::clone::Clone for AUDIO_EFFECT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDIO_EFFECT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AUDIO_EFFECT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIO_EFFECT_STATE").field(&self.0).finish()
    }
}
impl FromIntoMemory for AUDIO_EFFECT_STATE {
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
pub struct AUDIO_STREAM_CATEGORY(pub i32);
pub const AudioCategory_Other: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(0i32);
pub const AudioCategory_ForegroundOnlyMedia: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(1i32);
pub const AudioCategory_Communications: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(3i32);
pub const AudioCategory_Alerts: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(4i32);
pub const AudioCategory_SoundEffects: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(5i32);
pub const AudioCategory_GameEffects: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(6i32);
pub const AudioCategory_GameMedia: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(7i32);
pub const AudioCategory_GameChat: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(8i32);
pub const AudioCategory_Speech: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(9i32);
pub const AudioCategory_Movie: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(10i32);
pub const AudioCategory_Media: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(11i32);
pub const AudioCategory_FarFieldSpeech: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(12i32);
pub const AudioCategory_UniformSpeech: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(13i32);
pub const AudioCategory_VoiceTyping: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(14i32);
impl ::core::marker::Copy for AUDIO_STREAM_CATEGORY {}
impl ::core::clone::Clone for AUDIO_STREAM_CATEGORY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDIO_STREAM_CATEGORY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AUDIO_STREAM_CATEGORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIO_STREAM_CATEGORY")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for AUDIO_STREAM_CATEGORY {
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
pub struct AUDIO_VOLUME_NOTIFICATION_DATA {
    pub guidEventContext: crate::core::GUID,
    pub bMuted: super::super::Foundation::BOOL,
    pub fMasterVolume: f32,
    pub nChannels: u32,
    pub afChannelVolumes: [f32; 1],
}
impl ::core::marker::Copy for AUDIO_VOLUME_NOTIFICATION_DATA {}
impl ::core::clone::Clone for AUDIO_VOLUME_NOTIFICATION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUDIO_VOLUME_NOTIFICATION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_VOLUME_NOTIFICATION_DATA")
            .field("guidEventContext", &self.guidEventContext)
            .field("bMuted", &self.bMuted)
            .field("fMasterVolume", &self.fMasterVolume)
            .field("nChannels", &self.nChannels)
            .field("afChannelVolumes", &self.afChannelVolumes)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AUDIO_VOLUME_NOTIFICATION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.guidEventContext == other.guidEventContext
            && self.bMuted == other.bMuted
            && self.fMasterVolume == other.fMasterVolume
            && self.nChannels == other.nChannels
            && self.afChannelVolumes == other.afChannelVolumes
    }
}
impl ::core::cmp::Eq for AUDIO_VOLUME_NOTIFICATION_DATA {}
impl FromIntoMemory for AUDIO_VOLUME_NOTIFICATION_DATA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_guidEventContext =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_bMuted =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_fMasterVolume = <f32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_nChannels = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_afChannelVolumes = <[f32; 1] as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            guidEventContext: f_guidEventContext,
            bMuted: f_bMuted,
            fMasterVolume: f_fMasterVolume,
            nChannels: f_nChannels,
            afChannelVolumes: f_afChannelVolumes,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.guidEventContext, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.bMuted, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.fMasterVolume, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.nChannels, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.afChannelVolumes, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
pub struct AUXCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub wTechnology: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: crate::core::GUID,
    pub ProductGuid: crate::core::GUID,
    pub NameGuid: crate::core::GUID,
}
impl ::core::marker::Copy for AUXCAPS2A {}
impl ::core::clone::Clone for AUXCAPS2A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUXCAPS2A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUXCAPS2A")
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("vDriverVersion", &self.vDriverVersion)
            .field("szPname", &self.szPname)
            .field("wTechnology", &self.wTechnology)
            .field("wReserved1", &self.wReserved1)
            .field("dwSupport", &self.dwSupport)
            .field("ManufacturerGuid", &self.ManufacturerGuid)
            .field("ProductGuid", &self.ProductGuid)
            .field("NameGuid", &self.NameGuid)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AUXCAPS2A {
    fn eq(&self, other: &Self) -> bool {
        self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.vDriverVersion == other.vDriverVersion
            && self.szPname == other.szPname
            && self.wTechnology == other.wTechnology
            && self.wReserved1 == other.wReserved1
            && self.dwSupport == other.dwSupport
            && self.ManufacturerGuid == other.ManufacturerGuid
            && self.ProductGuid == other.ProductGuid
            && self.NameGuid == other.NameGuid
    }
}
impl ::core::cmp::Eq for AUXCAPS2A {}
impl FromIntoMemory for AUXCAPS2A {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 96);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_vDriverVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szPname =
            <[super::super::Foundation::CHAR; 32] as FromIntoMemory>::from_bytes(&from[8..8 + 32]);
        let f_wTechnology = <u16 as FromIntoMemory>::from_bytes(&from[40..40 + 2]);
        let f_wReserved1 = <u16 as FromIntoMemory>::from_bytes(&from[42..42 + 2]);
        let f_dwSupport = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_ManufacturerGuid =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[48..48 + 16]);
        let f_ProductGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[64..64 + 16]);
        let f_NameGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[80..80 + 16]);
        Self {
            wMid: f_wMid,
            wPid: f_wPid,
            vDriverVersion: f_vDriverVersion,
            szPname: f_szPname,
            wTechnology: f_wTechnology,
            wReserved1: f_wReserved1,
            dwSupport: f_dwSupport,
            ManufacturerGuid: f_ManufacturerGuid,
            ProductGuid: f_ProductGuid,
            NameGuid: f_NameGuid,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 96);
        FromIntoMemory::into_bytes(self.wMid, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.vDriverVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[8..8 + 32]);
        FromIntoMemory::into_bytes(self.wTechnology, &mut into[40..40 + 2]);
        FromIntoMemory::into_bytes(self.wReserved1, &mut into[42..42 + 2]);
        FromIntoMemory::into_bytes(self.dwSupport, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.ManufacturerGuid, &mut into[48..48 + 16]);
        FromIntoMemory::into_bytes(self.ProductGuid, &mut into[64..64 + 16]);
        FromIntoMemory::into_bytes(self.NameGuid, &mut into[80..80 + 16]);
    }
    fn size() -> usize {
        96
    }
}
pub struct AUXCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub wTechnology: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: crate::core::GUID,
    pub ProductGuid: crate::core::GUID,
    pub NameGuid: crate::core::GUID,
}
impl ::core::marker::Copy for AUXCAPS2W {}
impl ::core::clone::Clone for AUXCAPS2W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUXCAPS2W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUXCAPS2W")
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("vDriverVersion", &self.vDriverVersion)
            .field("szPname", &self.szPname)
            .field("wTechnology", &self.wTechnology)
            .field("wReserved1", &self.wReserved1)
            .field("dwSupport", &self.dwSupport)
            .field("ManufacturerGuid", &self.ManufacturerGuid)
            .field("ProductGuid", &self.ProductGuid)
            .field("NameGuid", &self.NameGuid)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AUXCAPS2W {
    fn eq(&self, other: &Self) -> bool {
        self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.vDriverVersion == other.vDriverVersion
            && self.szPname == other.szPname
            && self.wTechnology == other.wTechnology
            && self.wReserved1 == other.wReserved1
            && self.dwSupport == other.dwSupport
            && self.ManufacturerGuid == other.ManufacturerGuid
            && self.ProductGuid == other.ProductGuid
            && self.NameGuid == other.NameGuid
    }
}
impl ::core::cmp::Eq for AUXCAPS2W {}
impl FromIntoMemory for AUXCAPS2W {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 96);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_vDriverVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szPname = <[u16; 32] as FromIntoMemory>::from_bytes(&from[8..8 + 32]);
        let f_wTechnology = <u16 as FromIntoMemory>::from_bytes(&from[40..40 + 2]);
        let f_wReserved1 = <u16 as FromIntoMemory>::from_bytes(&from[42..42 + 2]);
        let f_dwSupport = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_ManufacturerGuid =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[48..48 + 16]);
        let f_ProductGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[64..64 + 16]);
        let f_NameGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[80..80 + 16]);
        Self {
            wMid: f_wMid,
            wPid: f_wPid,
            vDriverVersion: f_vDriverVersion,
            szPname: f_szPname,
            wTechnology: f_wTechnology,
            wReserved1: f_wReserved1,
            dwSupport: f_dwSupport,
            ManufacturerGuid: f_ManufacturerGuid,
            ProductGuid: f_ProductGuid,
            NameGuid: f_NameGuid,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 96);
        FromIntoMemory::into_bytes(self.wMid, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.vDriverVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[8..8 + 32]);
        FromIntoMemory::into_bytes(self.wTechnology, &mut into[40..40 + 2]);
        FromIntoMemory::into_bytes(self.wReserved1, &mut into[42..42 + 2]);
        FromIntoMemory::into_bytes(self.dwSupport, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.ManufacturerGuid, &mut into[48..48 + 16]);
        FromIntoMemory::into_bytes(self.ProductGuid, &mut into[64..64 + 16]);
        FromIntoMemory::into_bytes(self.NameGuid, &mut into[80..80 + 16]);
    }
    fn size() -> usize {
        96
    }
}
pub struct AUXCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub wTechnology: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
}
impl ::core::marker::Copy for AUXCAPSA {}
impl ::core::clone::Clone for AUXCAPSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUXCAPSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUXCAPSA")
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("vDriverVersion", &self.vDriverVersion)
            .field("szPname", &self.szPname)
            .field("wTechnology", &self.wTechnology)
            .field("wReserved1", &self.wReserved1)
            .field("dwSupport", &self.dwSupport)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AUXCAPSA {
    fn eq(&self, other: &Self) -> bool {
        self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.vDriverVersion == other.vDriverVersion
            && self.szPname == other.szPname
            && self.wTechnology == other.wTechnology
            && self.wReserved1 == other.wReserved1
            && self.dwSupport == other.dwSupport
    }
}
impl ::core::cmp::Eq for AUXCAPSA {}
impl FromIntoMemory for AUXCAPSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_vDriverVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szPname =
            <[super::super::Foundation::CHAR; 32] as FromIntoMemory>::from_bytes(&from[8..8 + 32]);
        let f_wTechnology = <u16 as FromIntoMemory>::from_bytes(&from[40..40 + 2]);
        let f_wReserved1 = <u16 as FromIntoMemory>::from_bytes(&from[42..42 + 2]);
        let f_dwSupport = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        Self {
            wMid: f_wMid,
            wPid: f_wPid,
            vDriverVersion: f_vDriverVersion,
            szPname: f_szPname,
            wTechnology: f_wTechnology,
            wReserved1: f_wReserved1,
            dwSupport: f_dwSupport,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.wMid, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.vDriverVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[8..8 + 32]);
        FromIntoMemory::into_bytes(self.wTechnology, &mut into[40..40 + 2]);
        FromIntoMemory::into_bytes(self.wReserved1, &mut into[42..42 + 2]);
        FromIntoMemory::into_bytes(self.dwSupport, &mut into[44..44 + 4]);
    }
    fn size() -> usize {
        48
    }
}
pub struct AUXCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub wTechnology: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
}
impl ::core::marker::Copy for AUXCAPSW {}
impl ::core::clone::Clone for AUXCAPSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUXCAPSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUXCAPSW")
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("vDriverVersion", &self.vDriverVersion)
            .field("szPname", &self.szPname)
            .field("wTechnology", &self.wTechnology)
            .field("wReserved1", &self.wReserved1)
            .field("dwSupport", &self.dwSupport)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AUXCAPSW {
    fn eq(&self, other: &Self) -> bool {
        self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.vDriverVersion == other.vDriverVersion
            && self.szPname == other.szPname
            && self.wTechnology == other.wTechnology
            && self.wReserved1 == other.wReserved1
            && self.dwSupport == other.dwSupport
    }
}
impl ::core::cmp::Eq for AUXCAPSW {}
impl FromIntoMemory for AUXCAPSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_vDriverVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szPname = <[u16; 32] as FromIntoMemory>::from_bytes(&from[8..8 + 32]);
        let f_wTechnology = <u16 as FromIntoMemory>::from_bytes(&from[40..40 + 2]);
        let f_wReserved1 = <u16 as FromIntoMemory>::from_bytes(&from[42..42 + 2]);
        let f_dwSupport = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        Self {
            wMid: f_wMid,
            wPid: f_wPid,
            vDriverVersion: f_vDriverVersion,
            szPname: f_szPname,
            wTechnology: f_wTechnology,
            wReserved1: f_wReserved1,
            dwSupport: f_dwSupport,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.wMid, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.vDriverVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[8..8 + 32]);
        FromIntoMemory::into_bytes(self.wTechnology, &mut into[40..40 + 2]);
        FromIntoMemory::into_bytes(self.wReserved1, &mut into[42..42 + 2]);
        FromIntoMemory::into_bytes(self.dwSupport, &mut into[44..44 + 4]);
    }
    fn size() -> usize {
        48
    }
}
pub const AUXCAPS_AUXIN: u32 = 2u32;
pub const AUXCAPS_CDAUDIO: u32 = 1u32;
pub const AUXCAPS_LRVOLUME: u32 = 2u32;
pub const AUXCAPS_VOLUME: u32 = 1u32;
pub struct AudioClient3ActivationParams {
    pub tracingContextId: crate::core::GUID,
}
impl ::core::marker::Copy for AudioClient3ActivationParams {}
impl ::core::clone::Clone for AudioClient3ActivationParams {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AudioClient3ActivationParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AudioClient3ActivationParams")
            .field("tracingContextId", &self.tracingContextId)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AudioClient3ActivationParams {
    fn eq(&self, other: &Self) -> bool {
        self.tracingContextId == other.tracingContextId
    }
}
impl ::core::cmp::Eq for AudioClient3ActivationParams {}
impl FromIntoMemory for AudioClient3ActivationParams {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_tracingContextId =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        Self {
            tracingContextId: f_tracingContextId,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.tracingContextId, &mut into[0..0 + 16]);
    }
    fn size() -> usize {
        16
    }
}
pub struct AudioClientProperties {
    pub cbSize: u32,
    pub bIsOffload: super::super::Foundation::BOOL,
    pub eCategory: AUDIO_STREAM_CATEGORY,
    pub Options: AUDCLNT_STREAMOPTIONS,
}
impl ::core::marker::Copy for AudioClientProperties {}
impl ::core::clone::Clone for AudioClientProperties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AudioClientProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AudioClientProperties")
            .field("cbSize", &self.cbSize)
            .field("bIsOffload", &self.bIsOffload)
            .field("eCategory", &self.eCategory)
            .field("Options", &self.Options)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AudioClientProperties {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.bIsOffload == other.bIsOffload
            && self.eCategory == other.eCategory
            && self.Options == other.Options
    }
}
impl ::core::cmp::Eq for AudioClientProperties {}
impl FromIntoMemory for AudioClientProperties {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_cbSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_bIsOffload =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_eCategory = <AUDIO_STREAM_CATEGORY as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_Options = <AUDCLNT_STREAMOPTIONS as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            cbSize: f_cbSize,
            bIsOffload: f_bIsOffload,
            eCategory: f_eCategory,
            Options: f_Options,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.cbSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.bIsOffload, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.eCategory, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.Options, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct AudioExtensionParams {
    pub AddPageParam: super::super::Foundation::LPARAM,
    pub pEndpoint: IMMDevice,
    pub pPnpInterface: IMMDevice,
    pub pPnpDevnode: IMMDevice,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for AudioExtensionParams {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for AudioExtensionParams {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for AudioExtensionParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AudioExtensionParams")
            .field("AddPageParam", &self.AddPageParam)
            .field("pEndpoint", &self.pEndpoint)
            .field("pPnpInterface", &self.pPnpInterface)
            .field("pPnpDevnode", &self.pPnpDevnode)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for AudioExtensionParams {
    fn eq(&self, other: &Self) -> bool {
        self.AddPageParam == other.AddPageParam
            && self.pEndpoint == other.pEndpoint
            && self.pPnpInterface == other.pPnpInterface
            && self.pPnpDevnode == other.pPnpDevnode
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for AudioExtensionParams {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for AudioExtensionParams {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_AddPageParam =
            <super::super::Foundation::LPARAM as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_pEndpoint = <IMMDevice as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_pPnpInterface = <IMMDevice as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_pPnpDevnode = <IMMDevice as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            AddPageParam: f_AddPageParam,
            pEndpoint: f_pEndpoint,
            pPnpInterface: f_pPnpInterface,
            pPnpDevnode: f_pPnpDevnode,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.AddPageParam, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.pEndpoint, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.pPnpInterface, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.pPnpDevnode, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioObjectType(pub u32);
pub const AudioObjectType_None: AudioObjectType = AudioObjectType(0u32);
pub const AudioObjectType_Dynamic: AudioObjectType = AudioObjectType(1u32);
pub const AudioObjectType_FrontLeft: AudioObjectType = AudioObjectType(2u32);
pub const AudioObjectType_FrontRight: AudioObjectType = AudioObjectType(4u32);
pub const AudioObjectType_FrontCenter: AudioObjectType = AudioObjectType(8u32);
pub const AudioObjectType_LowFrequency: AudioObjectType = AudioObjectType(16u32);
pub const AudioObjectType_SideLeft: AudioObjectType = AudioObjectType(32u32);
pub const AudioObjectType_SideRight: AudioObjectType = AudioObjectType(64u32);
pub const AudioObjectType_BackLeft: AudioObjectType = AudioObjectType(128u32);
pub const AudioObjectType_BackRight: AudioObjectType = AudioObjectType(256u32);
pub const AudioObjectType_TopFrontLeft: AudioObjectType = AudioObjectType(512u32);
pub const AudioObjectType_TopFrontRight: AudioObjectType = AudioObjectType(1024u32);
pub const AudioObjectType_TopBackLeft: AudioObjectType = AudioObjectType(2048u32);
pub const AudioObjectType_TopBackRight: AudioObjectType = AudioObjectType(4096u32);
pub const AudioObjectType_BottomFrontLeft: AudioObjectType = AudioObjectType(8192u32);
pub const AudioObjectType_BottomFrontRight: AudioObjectType = AudioObjectType(16384u32);
pub const AudioObjectType_BottomBackLeft: AudioObjectType = AudioObjectType(32768u32);
pub const AudioObjectType_BottomBackRight: AudioObjectType = AudioObjectType(65536u32);
pub const AudioObjectType_BackCenter: AudioObjectType = AudioObjectType(131072u32);
impl ::core::marker::Copy for AudioObjectType {}
impl ::core::clone::Clone for AudioObjectType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioObjectType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AudioObjectType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioObjectType").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for AudioObjectType {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AudioObjectType {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AudioObjectType {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AudioObjectType {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AudioObjectType {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for AudioObjectType {
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
pub struct AudioSessionDisconnectReason(pub i32);
pub const DisconnectReasonDeviceRemoval: AudioSessionDisconnectReason =
    AudioSessionDisconnectReason(0i32);
pub const DisconnectReasonServerShutdown: AudioSessionDisconnectReason =
    AudioSessionDisconnectReason(1i32);
pub const DisconnectReasonFormatChanged: AudioSessionDisconnectReason =
    AudioSessionDisconnectReason(2i32);
pub const DisconnectReasonSessionLogoff: AudioSessionDisconnectReason =
    AudioSessionDisconnectReason(3i32);
pub const DisconnectReasonSessionDisconnected: AudioSessionDisconnectReason =
    AudioSessionDisconnectReason(4i32);
pub const DisconnectReasonExclusiveModeOverride: AudioSessionDisconnectReason =
    AudioSessionDisconnectReason(5i32);
impl ::core::marker::Copy for AudioSessionDisconnectReason {}
impl ::core::clone::Clone for AudioSessionDisconnectReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioSessionDisconnectReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AudioSessionDisconnectReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioSessionDisconnectReason")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for AudioSessionDisconnectReason {
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
pub struct AudioSessionState(pub i32);
pub const AudioSessionStateInactive: AudioSessionState = AudioSessionState(0i32);
pub const AudioSessionStateActive: AudioSessionState = AudioSessionState(1i32);
pub const AudioSessionStateExpired: AudioSessionState = AudioSessionState(2i32);
impl ::core::marker::Copy for AudioSessionState {}
impl ::core::clone::Clone for AudioSessionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioSessionState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AudioSessionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioSessionState").field(&self.0).finish()
    }
}
impl FromIntoMemory for AudioSessionState {
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
pub struct AudioStateMonitorSoundLevel(pub i32);
pub const Muted: AudioStateMonitorSoundLevel = AudioStateMonitorSoundLevel(0i32);
pub const Low: AudioStateMonitorSoundLevel = AudioStateMonitorSoundLevel(1i32);
pub const Full: AudioStateMonitorSoundLevel = AudioStateMonitorSoundLevel(2i32);
impl ::core::marker::Copy for AudioStateMonitorSoundLevel {}
impl ::core::clone::Clone for AudioStateMonitorSoundLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioStateMonitorSoundLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AudioStateMonitorSoundLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioStateMonitorSoundLevel")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for AudioStateMonitorSoundLevel {
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
pub struct ConnectorType(pub i32);
impl ConnectorType {
    pub const Unknown_Connector: Self = Self(0i32);
    pub const Physical_Internal: Self = Self(1i32);
    pub const Physical_External: Self = Self(2i32);
    pub const Software_IO: Self = Self(3i32);
    pub const Software_Fixed: Self = Self(4i32);
    pub const Network: Self = Self(5i32);
}
impl ::core::marker::Copy for ConnectorType {}
impl ::core::clone::Clone for ConnectorType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ConnectorType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ConnectorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConnectorType").field(&self.0).finish()
    }
}
impl FromIntoMemory for ConnectorType {
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
pub const DEVICE_STATEMASK_ALL: u32 = 15u32;
pub const DEVICE_STATE_ACTIVE: u32 = 1u32;
pub const DEVICE_STATE_DISABLED: u32 = 2u32;
pub const DEVICE_STATE_NOTPRESENT: u32 = 4u32;
pub const DEVICE_STATE_UNPLUGGED: u32 = 8u32;
pub const DEVINTERFACE_AUDIO_CAPTURE: crate::core::GUID =
    crate::core::GUID::from_u128(0x2eef81be_33fa_4800_9670_1cd474972c3f);
pub const DEVINTERFACE_AUDIO_RENDER: crate::core::GUID =
    crate::core::GUID::from_u128(0xe6327cad_dcec_4949_ae8a_991e976a79d2);
pub const DEVINTERFACE_MIDI_INPUT: crate::core::GUID =
    crate::core::GUID::from_u128(0x504be32c_ccf6_4d2c_b73f_6f8b3747e22b);
pub const DEVINTERFACE_MIDI_OUTPUT: crate::core::GUID =
    crate::core::GUID::from_u128(0x6dc23320_ab33_4ce4_80d4_bbb3ebbf2814);
pub struct DIRECTX_AUDIO_ACTIVATION_PARAMS {
    pub cbDirectXAudioActivationParams: u32,
    pub guidAudioSession: crate::core::GUID,
    pub dwAudioStreamFlags: u32,
}
impl ::core::marker::Copy for DIRECTX_AUDIO_ACTIVATION_PARAMS {}
impl ::core::clone::Clone for DIRECTX_AUDIO_ACTIVATION_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIRECTX_AUDIO_ACTIVATION_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIRECTX_AUDIO_ACTIVATION_PARAMS")
            .field(
                "cbDirectXAudioActivationParams",
                &self.cbDirectXAudioActivationParams,
            )
            .field("guidAudioSession", &self.guidAudioSession)
            .field("dwAudioStreamFlags", &self.dwAudioStreamFlags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DIRECTX_AUDIO_ACTIVATION_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.cbDirectXAudioActivationParams == other.cbDirectXAudioActivationParams
            && self.guidAudioSession == other.guidAudioSession
            && self.dwAudioStreamFlags == other.dwAudioStreamFlags
    }
}
impl ::core::cmp::Eq for DIRECTX_AUDIO_ACTIVATION_PARAMS {}
impl FromIntoMemory for DIRECTX_AUDIO_ACTIVATION_PARAMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_cbDirectXAudioActivationParams = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_guidAudioSession =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[4..4 + 16]);
        let f_dwAudioStreamFlags = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            cbDirectXAudioActivationParams: f_cbDirectXAudioActivationParams,
            guidAudioSession: f_guidAudioSession,
            dwAudioStreamFlags: f_dwAudioStreamFlags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.cbDirectXAudioActivationParams, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.guidAudioSession, &mut into[4..4 + 16]);
        FromIntoMemory::into_bytes(self.dwAudioStreamFlags, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub const DRVM_MAPPER: u32 = 8192u32;
pub const DRVM_MAPPER_STATUS: u32 = 8192u32;
pub const DRV_MAPPER_PREFERRED_INPUT_GET: u32 = 16384u32;
pub const DRV_MAPPER_PREFERRED_OUTPUT_GET: u32 = 16386u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DataFlow(pub i32);
pub const In: DataFlow = DataFlow(0i32);
pub const Out: DataFlow = DataFlow(1i32);
impl ::core::marker::Copy for DataFlow {}
impl ::core::clone::Clone for DataFlow {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DataFlow {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DataFlow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataFlow").field(&self.0).finish()
    }
}
impl FromIntoMemory for DataFlow {
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
pub const DeviceTopology: crate::core::GUID =
    crate::core::GUID::from_u128(0x1df639d0_5ec1_47aa_9379_828dc1aa8c59);
pub struct ECHOWAVEFILTER {
    pub wfltr: WAVEFILTER,
    pub dwVolume: u32,
    pub dwDelay: u32,
}
impl ::core::marker::Copy for ECHOWAVEFILTER {}
impl ::core::clone::Clone for ECHOWAVEFILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ECHOWAVEFILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ECHOWAVEFILTER")
            .field("wfltr", &self.wfltr)
            .field("dwVolume", &self.dwVolume)
            .field("dwDelay", &self.dwDelay)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ECHOWAVEFILTER {
    fn eq(&self, other: &Self) -> bool {
        self.wfltr == other.wfltr
            && self.dwVolume == other.dwVolume
            && self.dwDelay == other.dwDelay
    }
}
impl ::core::cmp::Eq for ECHOWAVEFILTER {}
impl FromIntoMemory for ECHOWAVEFILTER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_wfltr = <WAVEFILTER as FromIntoMemory>::from_bytes(&from[0..0 + 32]);
        let f_dwVolume = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_dwDelay = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        Self {
            wfltr: f_wfltr,
            dwVolume: f_dwVolume,
            dwDelay: f_dwDelay,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.wfltr, &mut into[0..0 + 32]);
        FromIntoMemory::into_bytes(self.dwVolume, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.dwDelay, &mut into[36..36 + 4]);
    }
    fn size() -> usize {
        40
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EDataFlow(pub i32);
pub const eRender: EDataFlow = EDataFlow(0i32);
pub const eCapture: EDataFlow = EDataFlow(1i32);
pub const eAll: EDataFlow = EDataFlow(2i32);
pub const EDataFlow_enum_count: EDataFlow = EDataFlow(3i32);
impl ::core::marker::Copy for EDataFlow {}
impl ::core::clone::Clone for EDataFlow {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EDataFlow {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EDataFlow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EDataFlow").field(&self.0).finish()
    }
}
impl FromIntoMemory for EDataFlow {
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
pub const ENDPOINT_FORMAT_RESET_MIX_ONLY: u32 = 1u32;
pub const ENDPOINT_HARDWARE_SUPPORT_METER: u32 = 4u32;
pub const ENDPOINT_HARDWARE_SUPPORT_MUTE: u32 = 2u32;
pub const ENDPOINT_HARDWARE_SUPPORT_VOLUME: u32 = 1u32;
pub const ENDPOINT_SYSFX_DISABLED: u32 = 1u32;
pub const ENDPOINT_SYSFX_ENABLED: u32 = 0u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ERole(pub i32);
pub const eConsole: ERole = ERole(0i32);
pub const eMultimedia: ERole = ERole(1i32);
pub const eCommunications: ERole = ERole(2i32);
pub const ERole_enum_count: ERole = ERole(3i32);
impl ::core::marker::Copy for ERole {}
impl ::core::clone::Clone for ERole {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ERole {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ERole {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ERole").field(&self.0).finish()
    }
}
impl FromIntoMemory for ERole {
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
pub const EVENTCONTEXT_VOLUMESLIDER: crate::core::GUID =
    crate::core::GUID::from_u128(0xe2c2e9de_09b1_4b04_84e5_07931225ee04);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EndpointFormFactor(pub i32);
pub const RemoteNetworkDevice: EndpointFormFactor = EndpointFormFactor(0i32);
pub const Speakers: EndpointFormFactor = EndpointFormFactor(1i32);
pub const LineLevel: EndpointFormFactor = EndpointFormFactor(2i32);
pub const Headphones: EndpointFormFactor = EndpointFormFactor(3i32);
pub const Microphone: EndpointFormFactor = EndpointFormFactor(4i32);
pub const Headset: EndpointFormFactor = EndpointFormFactor(5i32);
pub const Handset: EndpointFormFactor = EndpointFormFactor(6i32);
pub const UnknownDigitalPassthrough: EndpointFormFactor = EndpointFormFactor(7i32);
pub const SPDIF: EndpointFormFactor = EndpointFormFactor(8i32);
pub const DigitalAudioDisplayDevice: EndpointFormFactor = EndpointFormFactor(9i32);
pub const UnknownFormFactor: EndpointFormFactor = EndpointFormFactor(10i32);
pub const EndpointFormFactor_enum_count: EndpointFormFactor = EndpointFormFactor(11i32);
impl ::core::marker::Copy for EndpointFormFactor {}
impl ::core::clone::Clone for EndpointFormFactor {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EndpointFormFactor {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EndpointFormFactor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EndpointFormFactor").field(&self.0).finish()
    }
}
impl FromIntoMemory for EndpointFormFactor {
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
pub const FILTERCHOOSE_CUSTOM_VERIFY: u32 = 2u32;
pub const FILTERCHOOSE_FILTERTAG_VERIFY: u32 = 0u32;
pub const FILTERCHOOSE_FILTER_VERIFY: u32 = 1u32;
pub const FILTERCHOOSE_MESSAGE: u32 = 0u32;
pub const FORMATCHOOSE_CUSTOM_VERIFY: u32 = 2u32;
pub const FORMATCHOOSE_FORMATTAG_VERIFY: u32 = 0u32;
pub const FORMATCHOOSE_FORMAT_VERIFY: u32 = 1u32;
pub const FORMATCHOOSE_MESSAGE: u32 = 0u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HACMDRIVER(pub PtrDiffRepr);
impl HACMDRIVER {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HACMDRIVER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HACMDRIVER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HACMDRIVER {}
impl ::core::hash::Hash for HACMDRIVER {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HACMDRIVER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HACMDRIVER").field(&self.0).finish()
    }
}
impl FromIntoMemory for HACMDRIVER {
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
pub struct HACMDRIVERID(pub PtrDiffRepr);
impl HACMDRIVERID {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HACMDRIVERID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HACMDRIVERID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HACMDRIVERID {}
impl ::core::hash::Hash for HACMDRIVERID {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HACMDRIVERID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HACMDRIVERID").field(&self.0).finish()
    }
}
impl FromIntoMemory for HACMDRIVERID {
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
pub struct HACMOBJ(pub PtrDiffRepr);
impl HACMOBJ {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HACMOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HACMOBJ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HACMOBJ {}
impl ::core::hash::Hash for HACMOBJ {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HACMOBJ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HACMOBJ").field(&self.0).finish()
    }
}
impl FromIntoMemory for HACMOBJ {
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
pub struct HACMSTREAM(pub PtrDiffRepr);
impl HACMSTREAM {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HACMSTREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HACMSTREAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HACMSTREAM {}
impl ::core::hash::Hash for HACMSTREAM {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HACMSTREAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HACMSTREAM").field(&self.0).finish()
    }
}
impl FromIntoMemory for HACMSTREAM {
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
pub struct HMIDI(pub PtrDiffRepr);
impl HMIDI {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HMIDI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HMIDI {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HMIDI {}
impl ::core::hash::Hash for HMIDI {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HMIDI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HMIDI").field(&self.0).finish()
    }
}
impl FromIntoMemory for HMIDI {
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
pub struct HMIDIIN(pub PtrDiffRepr);
impl HMIDIIN {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HMIDIIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HMIDIIN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HMIDIIN {}
impl ::core::hash::Hash for HMIDIIN {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HMIDIIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HMIDIIN").field(&self.0).finish()
    }
}
impl FromIntoMemory for HMIDIIN {
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
pub struct HMIDIOUT(pub PtrDiffRepr);
impl HMIDIOUT {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HMIDIOUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HMIDIOUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HMIDIOUT {}
impl ::core::hash::Hash for HMIDIOUT {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HMIDIOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HMIDIOUT").field(&self.0).finish()
    }
}
impl FromIntoMemory for HMIDIOUT {
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
pub struct HMIDISTRM(pub PtrDiffRepr);
impl HMIDISTRM {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HMIDISTRM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HMIDISTRM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HMIDISTRM {}
impl ::core::hash::Hash for HMIDISTRM {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HMIDISTRM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HMIDISTRM").field(&self.0).finish()
    }
}
impl FromIntoMemory for HMIDISTRM {
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
pub struct HMIXER(pub PtrDiffRepr);
impl HMIXER {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HMIXER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HMIXER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HMIXER {}
impl ::core::hash::Hash for HMIXER {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HMIXER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HMIXER").field(&self.0).finish()
    }
}
impl FromIntoMemory for HMIXER {
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
pub struct HMIXEROBJ(pub PtrDiffRepr);
impl HMIXEROBJ {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HMIXEROBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HMIXEROBJ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HMIXEROBJ {}
impl ::core::hash::Hash for HMIXEROBJ {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HMIXEROBJ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HMIXEROBJ").field(&self.0).finish()
    }
}
impl FromIntoMemory for HMIXEROBJ {
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
pub struct HWAVE(pub PtrDiffRepr);
impl HWAVE {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HWAVE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HWAVE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HWAVE {}
impl ::core::hash::Hash for HWAVE {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HWAVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HWAVE").field(&self.0).finish()
    }
}
impl FromIntoMemory for HWAVE {
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
pub struct HWAVEIN(pub PtrDiffRepr);
impl HWAVEIN {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HWAVEIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HWAVEIN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HWAVEIN {}
impl ::core::hash::Hash for HWAVEIN {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HWAVEIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HWAVEIN").field(&self.0).finish()
    }
}
impl FromIntoMemory for HWAVEIN {
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
pub struct HWAVEOUT(pub PtrDiffRepr);
impl HWAVEOUT {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HWAVEOUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HWAVEOUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HWAVEOUT {}
impl ::core::hash::Hash for HWAVEOUT {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HWAVEOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HWAVEOUT").field(&self.0).finish()
    }
}
impl FromIntoMemory for HWAVEOUT {
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
pub struct IActivateAudioInterfaceAsyncOperation(pub crate::core::IUnknown);
pub trait IActivateAudioInterfaceAsyncOperation_Trait: crate::core::IUnknown_Trait {
    fn GetActivateResult(
        &self,
        activate_result: MutPtr<crate::core::HRESULT>,
        activated_interface: MutPtr<crate::core::IUnknown>,
    ) -> crate::core::HRESULT {
        todo!("GetActivateResult")
    }
}
impl ::core::clone::Clone for IActivateAudioInterfaceAsyncOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IActivateAudioInterfaceAsyncOperation {}
impl ::core::cmp::PartialEq for IActivateAudioInterfaceAsyncOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActivateAudioInterfaceAsyncOperation {}
impl ::core::fmt::Debug for IActivateAudioInterfaceAsyncOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActivateAudioInterfaceAsyncOperation")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IActivateAudioInterfaceAsyncOperation {
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
impl crate::core::ComInterface for IActivateAudioInterfaceAsyncOperation {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x72a22d78_cde4_431d_b8cc_843a71199b6d);
}
pub struct IActivateAudioInterfaceCompletionHandler(pub crate::core::IUnknown);
pub trait IActivateAudioInterfaceCompletionHandler_Trait: crate::core::IUnknown_Trait {
    fn ActivateCompleted(
        &self,
        activate_operation: IActivateAudioInterfaceAsyncOperation,
    ) -> crate::core::HRESULT {
        todo!("ActivateCompleted")
    }
}
impl ::core::clone::Clone for IActivateAudioInterfaceCompletionHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IActivateAudioInterfaceCompletionHandler {}
impl ::core::cmp::PartialEq for IActivateAudioInterfaceCompletionHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActivateAudioInterfaceCompletionHandler {}
impl ::core::fmt::Debug for IActivateAudioInterfaceCompletionHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActivateAudioInterfaceCompletionHandler")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IActivateAudioInterfaceCompletionHandler {
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
impl crate::core::ComInterface for IActivateAudioInterfaceCompletionHandler {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x41d949ab_9862_444a_80f6_c261334da5eb);
}
pub struct IAudioAmbisonicsControl(pub crate::core::IUnknown);
pub trait IAudioAmbisonicsControl_Trait: crate::core::IUnknown_Trait {
    fn SetData(
        &self,
        p_ambisonics_params: ConstPtr<AMBISONICS_PARAMS>,
        cb_ambisonics_params: u32,
    ) -> crate::core::HRESULT {
        todo!("SetData")
    }
    fn SetHeadTracking(
        &self,
        b_enable_head_tracking: super::super::Foundation::BOOL,
    ) -> crate::core::HRESULT {
        todo!("SetHeadTracking")
    }
    fn GetHeadTracking(
        &self,
        pb_enable_head_tracking: MutPtr<super::super::Foundation::BOOL>,
    ) -> crate::core::HRESULT {
        todo!("GetHeadTracking")
    }
    fn SetRotation(&self, x: f32, y: f32, z: f32, w: f32) -> crate::core::HRESULT {
        todo!("SetRotation")
    }
}
impl ::core::clone::Clone for IAudioAmbisonicsControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioAmbisonicsControl {}
impl ::core::cmp::PartialEq for IAudioAmbisonicsControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioAmbisonicsControl {}
impl ::core::fmt::Debug for IAudioAmbisonicsControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioAmbisonicsControl")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IAudioAmbisonicsControl {
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
impl crate::core::ComInterface for IAudioAmbisonicsControl {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x28724c91_df35_4856_9f76_d6a26413f3df);
}
pub struct IAudioAutoGainControl(pub crate::core::IUnknown);
pub trait IAudioAutoGainControl_Trait: crate::core::IUnknown_Trait {
    fn GetEnabled(
        &self,
        pb_enabled: MutPtr<super::super::Foundation::BOOL>,
    ) -> crate::core::HRESULT {
        todo!("GetEnabled")
    }
    fn SetEnabled(
        &self,
        b_enable: super::super::Foundation::BOOL,
        pguid_event_context: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("SetEnabled")
    }
}
impl ::core::clone::Clone for IAudioAutoGainControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioAutoGainControl {}
impl ::core::cmp::PartialEq for IAudioAutoGainControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioAutoGainControl {}
impl ::core::fmt::Debug for IAudioAutoGainControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioAutoGainControl")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IAudioAutoGainControl {
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
impl crate::core::ComInterface for IAudioAutoGainControl {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x85401fd4_6de4_4b9d_9869_2d6753a82f3c);
}
pub struct IAudioBass(pub crate::core::IUnknown);
pub trait IAudioBass_Trait: IPerChannelDbLevel_Trait {}
impl ::core::clone::Clone for IAudioBass {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioBass {}
impl ::core::cmp::PartialEq for IAudioBass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioBass {}
impl ::core::fmt::Debug for IAudioBass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioBass").field(&self.0).finish()
    }
}
impl FromIntoMemory for IAudioBass {
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
impl crate::core::ComInterface for IAudioBass {
    type Super = IPerChannelDbLevel;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xa2b1a1d9_4db3_425d_a2b2_bd335cb3e2e5);
}
pub struct IAudioCaptureClient(pub crate::core::IUnknown);
pub trait IAudioCaptureClient_Trait: crate::core::IUnknown_Trait {
    fn GetBuffer(
        &self,
        pp_data: MutPtr<ConstPtr<u8>>,
        p_num_frames_to_read: MutPtr<u32>,
        pdw_flags: MutPtr<u32>,
        pu_64_device_position: MutPtr<u64>,
        pu_64_qpc_position: MutPtr<u64>,
    ) -> crate::core::HRESULT {
        todo!("GetBuffer")
    }
    fn ReleaseBuffer(&self, num_frames_read: u32) -> crate::core::HRESULT {
        todo!("ReleaseBuffer")
    }
    fn GetNextPacketSize(&self, p_num_frames_in_next_packet: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetNextPacketSize")
    }
}
impl ::core::clone::Clone for IAudioCaptureClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioCaptureClient {}
impl ::core::cmp::PartialEq for IAudioCaptureClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioCaptureClient {}
impl ::core::fmt::Debug for IAudioCaptureClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioCaptureClient").field(&self.0).finish()
    }
}
impl FromIntoMemory for IAudioCaptureClient {
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
impl crate::core::ComInterface for IAudioCaptureClient {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xc8adbd64_e71e_48a0_a4de_185c395cd317);
}
pub struct IAudioChannelConfig(pub crate::core::IUnknown);
pub trait IAudioChannelConfig_Trait: crate::core::IUnknown_Trait {
    fn SetChannelConfig(
        &self,
        dw_config: u32,
        pguid_event_context: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("SetChannelConfig")
    }
    fn GetChannelConfig(&self, pdw_config: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetChannelConfig")
    }
}
impl ::core::clone::Clone for IAudioChannelConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioChannelConfig {}
impl ::core::cmp::PartialEq for IAudioChannelConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioChannelConfig {}
impl ::core::fmt::Debug for IAudioChannelConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioChannelConfig").field(&self.0).finish()
    }
}
impl FromIntoMemory for IAudioChannelConfig {
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
impl crate::core::ComInterface for IAudioChannelConfig {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xbb11c46f_ec28_493c_b88a_5db88062ce98);
}
pub struct IAudioClient(pub crate::core::IUnknown);
pub trait IAudioClient_Trait: crate::core::IUnknown_Trait {
    fn Initialize(
        &self,
        share_mode: AUDCLNT_SHAREMODE,
        stream_flags: u32,
        hns_buffer_duration: i64,
        hns_periodicity: i64,
        p_format: ConstPtr<WAVEFORMATEX>,
        audio_session_guid: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("Initialize")
    }
    fn GetBufferSize(&self, p_num_buffer_frames: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetBufferSize")
    }
    fn GetStreamLatency(&self, phns_latency: MutPtr<i64>) -> crate::core::HRESULT {
        todo!("GetStreamLatency")
    }
    fn GetCurrentPadding(&self, p_num_padding_frames: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetCurrentPadding")
    }
    fn IsFormatSupported(
        &self,
        share_mode: AUDCLNT_SHAREMODE,
        p_format: ConstPtr<WAVEFORMATEX>,
        pp_closest_match: MutPtr<ConstPtr<WAVEFORMATEX>>,
    ) -> crate::core::HRESULT {
        todo!("IsFormatSupported")
    }
    fn GetMixFormat(
        &self,
        pp_device_format: MutPtr<ConstPtr<WAVEFORMATEX>>,
    ) -> crate::core::HRESULT {
        todo!("GetMixFormat")
    }
    fn GetDevicePeriod(
        &self,
        phns_default_device_period: MutPtr<i64>,
        phns_minimum_device_period: MutPtr<i64>,
    ) -> crate::core::HRESULT {
        todo!("GetDevicePeriod")
    }
    fn Start(&self) -> crate::core::HRESULT {
        todo!("Start")
    }
    fn Stop(&self) -> crate::core::HRESULT {
        todo!("Stop")
    }
    fn Reset(&self) -> crate::core::HRESULT {
        todo!("Reset")
    }
    fn SetEventHandle(
        &self,
        event_handle: super::super::Foundation::HANDLE,
    ) -> crate::core::HRESULT {
        todo!("SetEventHandle")
    }
    fn GetService(
        &self,
        riid: ConstPtr<crate::core::GUID>,
        ppv: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("GetService")
    }
}
impl ::core::clone::Clone for IAudioClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioClient {}
impl ::core::cmp::PartialEq for IAudioClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioClient {}
impl ::core::fmt::Debug for IAudioClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioClient").field(&self.0).finish()
    }
}
impl FromIntoMemory for IAudioClient {
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
impl crate::core::ComInterface for IAudioClient {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x1cb9ad4c_dbfa_4c32_b178_c2f568a703b2);
}
pub struct IAudioClient2(pub crate::core::IUnknown);
pub trait IAudioClient2_Trait: IAudioClient_Trait {
    fn IsOffloadCapable(
        &self,
        category: AUDIO_STREAM_CATEGORY,
        pb_offload_capable: MutPtr<super::super::Foundation::BOOL>,
    ) -> crate::core::HRESULT {
        todo!("IsOffloadCapable")
    }
    fn SetClientProperties(
        &self,
        p_properties: ConstPtr<AudioClientProperties>,
    ) -> crate::core::HRESULT {
        todo!("SetClientProperties")
    }
    fn GetBufferSizeLimits(
        &self,
        p_format: ConstPtr<WAVEFORMATEX>,
        b_event_driven: super::super::Foundation::BOOL,
        phns_min_buffer_duration: MutPtr<i64>,
        phns_max_buffer_duration: MutPtr<i64>,
    ) -> crate::core::HRESULT {
        todo!("GetBufferSizeLimits")
    }
}
impl ::core::clone::Clone for IAudioClient2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioClient2 {}
impl ::core::cmp::PartialEq for IAudioClient2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioClient2 {}
impl ::core::fmt::Debug for IAudioClient2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioClient2").field(&self.0).finish()
    }
}
impl FromIntoMemory for IAudioClient2 {
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
impl crate::core::ComInterface for IAudioClient2 {
    type Super = IAudioClient;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x726778cd_f60a_4eda_82de_e47610cd78aa);
}
pub struct IAudioClient3(pub crate::core::IUnknown);
pub trait IAudioClient3_Trait: IAudioClient2_Trait {
    fn GetSharedModeEnginePeriod(
        &self,
        p_format: ConstPtr<WAVEFORMATEX>,
        p_default_period_in_frames: MutPtr<u32>,
        p_fundamental_period_in_frames: MutPtr<u32>,
        p_min_period_in_frames: MutPtr<u32>,
        p_max_period_in_frames: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("GetSharedModeEnginePeriod")
    }
    fn GetCurrentSharedModeEnginePeriod(
        &self,
        pp_format: MutPtr<ConstPtr<WAVEFORMATEX>>,
        p_current_period_in_frames: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("GetCurrentSharedModeEnginePeriod")
    }
    fn InitializeSharedAudioStream(
        &self,
        stream_flags: u32,
        period_in_frames: u32,
        p_format: ConstPtr<WAVEFORMATEX>,
        audio_session_guid: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("InitializeSharedAudioStream")
    }
}
impl ::core::clone::Clone for IAudioClient3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioClient3 {}
impl ::core::cmp::PartialEq for IAudioClient3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioClient3 {}
impl ::core::fmt::Debug for IAudioClient3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioClient3").field(&self.0).finish()
    }
}
impl FromIntoMemory for IAudioClient3 {
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
impl crate::core::ComInterface for IAudioClient3 {
    type Super = IAudioClient2;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x7ed4ee07_8e67_4cd4_8c1a_2b7a5987ad42);
}
pub struct IAudioClientDuckingControl(pub crate::core::IUnknown);
pub trait IAudioClientDuckingControl_Trait: crate::core::IUnknown_Trait {
    fn SetDuckingOptionsForCurrentStream(
        &self,
        options: AUDIO_DUCKING_OPTIONS,
    ) -> crate::core::HRESULT {
        todo!("SetDuckingOptionsForCurrentStream")
    }
}
impl ::core::clone::Clone for IAudioClientDuckingControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioClientDuckingControl {}
impl ::core::cmp::PartialEq for IAudioClientDuckingControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioClientDuckingControl {}
impl ::core::fmt::Debug for IAudioClientDuckingControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioClientDuckingControl")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IAudioClientDuckingControl {
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
impl crate::core::ComInterface for IAudioClientDuckingControl {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xc789d381_a28c_4168_b28f_d3a837924dc3);
}
pub struct IAudioClock(pub crate::core::IUnknown);
pub trait IAudioClock_Trait: crate::core::IUnknown_Trait {
    fn GetFrequency(&self, pu_64_frequency: MutPtr<u64>) -> crate::core::HRESULT {
        todo!("GetFrequency")
    }
    fn GetPosition(
        &self,
        pu_64_position: MutPtr<u64>,
        pu_64_qpc_position: MutPtr<u64>,
    ) -> crate::core::HRESULT {
        todo!("GetPosition")
    }
    fn GetCharacteristics(&self, pdw_characteristics: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetCharacteristics")
    }
}
impl ::core::clone::Clone for IAudioClock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioClock {}
impl ::core::cmp::PartialEq for IAudioClock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioClock {}
impl ::core::fmt::Debug for IAudioClock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioClock").field(&self.0).finish()
    }
}
impl FromIntoMemory for IAudioClock {
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
impl crate::core::ComInterface for IAudioClock {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xcd63314f_3fba_4a1b_812c_ef96358728e7);
}
pub struct IAudioClock2(pub crate::core::IUnknown);
pub trait IAudioClock2_Trait: crate::core::IUnknown_Trait {
    fn GetDevicePosition(
        &self,
        device_position: MutPtr<u64>,
        qpc_position: MutPtr<u64>,
    ) -> crate::core::HRESULT {
        todo!("GetDevicePosition")
    }
}
impl ::core::clone::Clone for IAudioClock2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioClock2 {}
impl ::core::cmp::PartialEq for IAudioClock2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioClock2 {}
impl ::core::fmt::Debug for IAudioClock2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioClock2").field(&self.0).finish()
    }
}
impl FromIntoMemory for IAudioClock2 {
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
impl crate::core::ComInterface for IAudioClock2 {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x6f49ff73_6727_49ac_a008_d98cf5e70048);
}
pub struct IAudioClockAdjustment(pub crate::core::IUnknown);
pub trait IAudioClockAdjustment_Trait: crate::core::IUnknown_Trait {
    fn SetSampleRate(&self, fl_sample_rate: f32) -> crate::core::HRESULT {
        todo!("SetSampleRate")
    }
}
impl ::core::clone::Clone for IAudioClockAdjustment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioClockAdjustment {}
impl ::core::cmp::PartialEq for IAudioClockAdjustment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioClockAdjustment {}
impl ::core::fmt::Debug for IAudioClockAdjustment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioClockAdjustment")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IAudioClockAdjustment {
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
impl crate::core::ComInterface for IAudioClockAdjustment {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xf6e4c0a0_46d9_4fb8_be21_57a3ef2b626c);
}
pub struct IAudioEffectsChangedNotificationClient(pub crate::core::IUnknown);
pub trait IAudioEffectsChangedNotificationClient_Trait: crate::core::IUnknown_Trait {
    fn OnAudioEffectsChanged(&self) -> crate::core::HRESULT {
        todo!("OnAudioEffectsChanged")
    }
}
impl ::core::clone::Clone for IAudioEffectsChangedNotificationClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioEffectsChangedNotificationClient {}
impl ::core::cmp::PartialEq for IAudioEffectsChangedNotificationClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEffectsChangedNotificationClient {}
impl ::core::fmt::Debug for IAudioEffectsChangedNotificationClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEffectsChangedNotificationClient")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IAudioEffectsChangedNotificationClient {
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
impl crate::core::ComInterface for IAudioEffectsChangedNotificationClient {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xa5ded44f_3c5d_4b2b_bd1e_5dc1ee20bbf6);
}
pub struct IAudioEffectsManager(pub crate::core::IUnknown);
pub trait IAudioEffectsManager_Trait: crate::core::IUnknown_Trait {
    fn RegisterAudioEffectsChangedNotificationCallback(
        &self,
        client: IAudioEffectsChangedNotificationClient,
    ) -> crate::core::HRESULT {
        todo!("RegisterAudioEffectsChangedNotificationCallback")
    }
    fn UnregisterAudioEffectsChangedNotificationCallback(
        &self,
        client: IAudioEffectsChangedNotificationClient,
    ) -> crate::core::HRESULT {
        todo!("UnregisterAudioEffectsChangedNotificationCallback")
    }
    fn GetAudioEffects(
        &self,
        effects: MutPtr<ConstPtr<AUDIO_EFFECT>>,
        num_effects: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("GetAudioEffects")
    }
    fn SetAudioEffectState(
        &self,
        effect_id: crate::core::GUID,
        state: AUDIO_EFFECT_STATE,
    ) -> crate::core::HRESULT {
        todo!("SetAudioEffectState")
    }
}
impl ::core::clone::Clone for IAudioEffectsManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioEffectsManager {}
impl ::core::cmp::PartialEq for IAudioEffectsManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEffectsManager {}
impl ::core::fmt::Debug for IAudioEffectsManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEffectsManager")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IAudioEffectsManager {
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
impl crate::core::ComInterface for IAudioEffectsManager {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x4460b3ae_4b44_4527_8676_7548a8acd260);
}
pub struct IAudioFormatEnumerator(pub crate::core::IUnknown);
pub trait IAudioFormatEnumerator_Trait: crate::core::IUnknown_Trait {
    fn GetCount(&self, count: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetCount")
    }
    fn GetFormat(
        &self,
        index: u32,
        format: MutPtr<ConstPtr<WAVEFORMATEX>>,
    ) -> crate::core::HRESULT {
        todo!("GetFormat")
    }
}
impl ::core::clone::Clone for IAudioFormatEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioFormatEnumerator {}
impl ::core::cmp::PartialEq for IAudioFormatEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioFormatEnumerator {}
impl ::core::fmt::Debug for IAudioFormatEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioFormatEnumerator")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IAudioFormatEnumerator {
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
impl crate::core::ComInterface for IAudioFormatEnumerator {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xdcdaa858_895a_4a22_a5eb_67bda506096d);
}
pub struct IAudioInputSelector(pub crate::core::IUnknown);
pub trait IAudioInputSelector_Trait: crate::core::IUnknown_Trait {
    fn GetSelection(&self, pn_id_selected: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetSelection")
    }
    fn SetSelection(
        &self,
        n_id_select: u32,
        pguid_event_context: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("SetSelection")
    }
}
impl ::core::clone::Clone for IAudioInputSelector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioInputSelector {}
impl ::core::cmp::PartialEq for IAudioInputSelector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioInputSelector {}
impl ::core::fmt::Debug for IAudioInputSelector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioInputSelector").field(&self.0).finish()
    }
}
impl FromIntoMemory for IAudioInputSelector {
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
impl crate::core::ComInterface for IAudioInputSelector {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x4f03dc02_5e6e_4653_8f72_a030c123d598);
}
pub struct IAudioLoudness(pub crate::core::IUnknown);
pub trait IAudioLoudness_Trait: crate::core::IUnknown_Trait {
    fn GetEnabled(
        &self,
        pb_enabled: MutPtr<super::super::Foundation::BOOL>,
    ) -> crate::core::HRESULT {
        todo!("GetEnabled")
    }
    fn SetEnabled(
        &self,
        b_enable: super::super::Foundation::BOOL,
        pguid_event_context: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("SetEnabled")
    }
}
impl ::core::clone::Clone for IAudioLoudness {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioLoudness {}
impl ::core::cmp::PartialEq for IAudioLoudness {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioLoudness {}
impl ::core::fmt::Debug for IAudioLoudness {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioLoudness").field(&self.0).finish()
    }
}
impl FromIntoMemory for IAudioLoudness {
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
impl crate::core::ComInterface for IAudioLoudness {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x7d8b1437_dd53_4350_9c1b_1ee2890bd938);
}
pub struct IAudioMidrange(pub crate::core::IUnknown);
pub trait IAudioMidrange_Trait: IPerChannelDbLevel_Trait {}
impl ::core::clone::Clone for IAudioMidrange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioMidrange {}
impl ::core::cmp::PartialEq for IAudioMidrange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioMidrange {}
impl ::core::fmt::Debug for IAudioMidrange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioMidrange").field(&self.0).finish()
    }
}
impl FromIntoMemory for IAudioMidrange {
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
impl crate::core::ComInterface for IAudioMidrange {
    type Super = IPerChannelDbLevel;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x5e54b6d7_b44b_40d9_9a9e_e691d9ce6edf);
}
pub struct IAudioMute(pub crate::core::IUnknown);
pub trait IAudioMute_Trait: crate::core::IUnknown_Trait {
    fn SetMute(
        &self,
        b_muted: super::super::Foundation::BOOL,
        pguid_event_context: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("SetMute")
    }
    fn GetMute(&self, pb_muted: MutPtr<super::super::Foundation::BOOL>) -> crate::core::HRESULT {
        todo!("GetMute")
    }
}
impl ::core::clone::Clone for IAudioMute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioMute {}
impl ::core::cmp::PartialEq for IAudioMute {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioMute {}
impl ::core::fmt::Debug for IAudioMute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioMute").field(&self.0).finish()
    }
}
impl FromIntoMemory for IAudioMute {
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
impl crate::core::ComInterface for IAudioMute {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xdf45aeea_b74a_4b6b_afad_2366b6aa012e);
}
pub struct IAudioOutputSelector(pub crate::core::IUnknown);
pub trait IAudioOutputSelector_Trait: crate::core::IUnknown_Trait {
    fn GetSelection(&self, pn_id_selected: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetSelection")
    }
    fn SetSelection(
        &self,
        n_id_select: u32,
        pguid_event_context: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("SetSelection")
    }
}
impl ::core::clone::Clone for IAudioOutputSelector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioOutputSelector {}
impl ::core::cmp::PartialEq for IAudioOutputSelector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioOutputSelector {}
impl ::core::fmt::Debug for IAudioOutputSelector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioOutputSelector")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IAudioOutputSelector {
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
impl crate::core::ComInterface for IAudioOutputSelector {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xbb515f69_94a7_429e_8b9c_271b3f11a3ab);
}
pub struct IAudioPeakMeter(pub crate::core::IUnknown);
pub trait IAudioPeakMeter_Trait: crate::core::IUnknown_Trait {
    fn GetChannelCount(&self, pc_channels: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetChannelCount")
    }
    fn GetLevel(&self, n_channel: u32, pf_level: MutPtr<f32>) -> crate::core::HRESULT {
        todo!("GetLevel")
    }
}
impl ::core::clone::Clone for IAudioPeakMeter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioPeakMeter {}
impl ::core::cmp::PartialEq for IAudioPeakMeter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioPeakMeter {}
impl ::core::fmt::Debug for IAudioPeakMeter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioPeakMeter").field(&self.0).finish()
    }
}
impl FromIntoMemory for IAudioPeakMeter {
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
impl crate::core::ComInterface for IAudioPeakMeter {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xdd79923c_0599_45e0_b8b6_c8df7db6e796);
}
pub struct IAudioRenderClient(pub crate::core::IUnknown);
pub trait IAudioRenderClient_Trait: crate::core::IUnknown_Trait {
    fn GetBuffer(
        &self,
        num_frames_requested: u32,
        pp_data: MutPtr<ConstPtr<u8>>,
    ) -> crate::core::HRESULT {
        todo!("GetBuffer")
    }
    fn ReleaseBuffer(&self, num_frames_written: u32, dw_flags: u32) -> crate::core::HRESULT {
        todo!("ReleaseBuffer")
    }
}
impl ::core::clone::Clone for IAudioRenderClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioRenderClient {}
impl ::core::cmp::PartialEq for IAudioRenderClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioRenderClient {}
impl ::core::fmt::Debug for IAudioRenderClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioRenderClient").field(&self.0).finish()
    }
}
impl FromIntoMemory for IAudioRenderClient {
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
impl crate::core::ComInterface for IAudioRenderClient {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xf294acfc_3146_4483_a7bf_addca7c260e2);
}
pub struct IAudioSessionControl(pub crate::core::IUnknown);
pub trait IAudioSessionControl_Trait: crate::core::IUnknown_Trait {
    fn GetState(&self, p_ret_val: MutPtr<AudioSessionState>) -> crate::core::HRESULT {
        todo!("GetState")
    }
    fn GetDisplayName(&self, p_ret_val: MutPtr<PWSTR>) -> crate::core::HRESULT {
        todo!("GetDisplayName")
    }
    fn SetDisplayName(
        &self,
        value: PCWSTR,
        event_context: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("SetDisplayName")
    }
    fn GetIconPath(&self, p_ret_val: MutPtr<PWSTR>) -> crate::core::HRESULT {
        todo!("GetIconPath")
    }
    fn SetIconPath(
        &self,
        value: PCWSTR,
        event_context: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("SetIconPath")
    }
    fn GetGroupingParam(&self, p_ret_val: MutPtr<crate::core::GUID>) -> crate::core::HRESULT {
        todo!("GetGroupingParam")
    }
    fn SetGroupingParam(
        &self,
        r#override: ConstPtr<crate::core::GUID>,
        event_context: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("SetGroupingParam")
    }
    fn RegisterAudioSessionNotification(
        &self,
        new_notifications: IAudioSessionEvents,
    ) -> crate::core::HRESULT {
        todo!("RegisterAudioSessionNotification")
    }
    fn UnregisterAudioSessionNotification(
        &self,
        new_notifications: IAudioSessionEvents,
    ) -> crate::core::HRESULT {
        todo!("UnregisterAudioSessionNotification")
    }
}
impl ::core::clone::Clone for IAudioSessionControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioSessionControl {}
impl ::core::cmp::PartialEq for IAudioSessionControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSessionControl {}
impl ::core::fmt::Debug for IAudioSessionControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSessionControl")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IAudioSessionControl {
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
impl crate::core::ComInterface for IAudioSessionControl {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xf4b1a599_7266_4319_a8ca_e70acb11e8cd);
}
pub struct IAudioSessionControl2(pub crate::core::IUnknown);
pub trait IAudioSessionControl2_Trait: IAudioSessionControl_Trait {
    fn GetSessionIdentifier(&self, p_ret_val: MutPtr<PWSTR>) -> crate::core::HRESULT {
        todo!("GetSessionIdentifier")
    }
    fn GetSessionInstanceIdentifier(&self, p_ret_val: MutPtr<PWSTR>) -> crate::core::HRESULT {
        todo!("GetSessionInstanceIdentifier")
    }
    fn GetProcessId(&self, p_ret_val: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetProcessId")
    }
    fn IsSystemSoundsSession(&self) -> crate::core::HRESULT {
        todo!("IsSystemSoundsSession")
    }
    fn SetDuckingPreference(
        &self,
        opt_out: super::super::Foundation::BOOL,
    ) -> crate::core::HRESULT {
        todo!("SetDuckingPreference")
    }
}
impl ::core::clone::Clone for IAudioSessionControl2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioSessionControl2 {}
impl ::core::cmp::PartialEq for IAudioSessionControl2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSessionControl2 {}
impl ::core::fmt::Debug for IAudioSessionControl2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSessionControl2")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IAudioSessionControl2 {
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
impl crate::core::ComInterface for IAudioSessionControl2 {
    type Super = IAudioSessionControl;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xbfb7ff88_7239_4fc9_8fa2_07c950be9c6d);
}
pub struct IAudioSessionEnumerator(pub crate::core::IUnknown);
pub trait IAudioSessionEnumerator_Trait: crate::core::IUnknown_Trait {
    fn GetCount(&self, session_count: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("GetCount")
    }
    fn GetSession(
        &self,
        session_count: i32,
        session: MutPtr<IAudioSessionControl>,
    ) -> crate::core::HRESULT {
        todo!("GetSession")
    }
}
impl ::core::clone::Clone for IAudioSessionEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioSessionEnumerator {}
impl ::core::cmp::PartialEq for IAudioSessionEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSessionEnumerator {}
impl ::core::fmt::Debug for IAudioSessionEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSessionEnumerator")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IAudioSessionEnumerator {
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
impl crate::core::ComInterface for IAudioSessionEnumerator {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xe2f5bb11_0570_40ca_acdd_3aa01277dee8);
}
pub struct IAudioSessionEvents(pub crate::core::IUnknown);
pub trait IAudioSessionEvents_Trait: crate::core::IUnknown_Trait {
    fn OnDisplayNameChanged(
        &self,
        new_display_name: PCWSTR,
        event_context: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("OnDisplayNameChanged")
    }
    fn OnIconPathChanged(
        &self,
        new_icon_path: PCWSTR,
        event_context: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("OnIconPathChanged")
    }
    fn OnSimpleVolumeChanged(
        &self,
        new_volume: f32,
        new_mute: super::super::Foundation::BOOL,
        event_context: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("OnSimpleVolumeChanged")
    }
    fn OnChannelVolumeChanged(
        &self,
        channel_count: u32,
        new_channel_volume_array: ConstPtr<f32>,
        changed_channel: u32,
        event_context: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("OnChannelVolumeChanged")
    }
    fn OnGroupingParamChanged(
        &self,
        new_grouping_param: ConstPtr<crate::core::GUID>,
        event_context: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("OnGroupingParamChanged")
    }
    fn OnStateChanged(&self, new_state: AudioSessionState) -> crate::core::HRESULT {
        todo!("OnStateChanged")
    }
    fn OnSessionDisconnected(
        &self,
        disconnect_reason: AudioSessionDisconnectReason,
    ) -> crate::core::HRESULT {
        todo!("OnSessionDisconnected")
    }
}
impl ::core::clone::Clone for IAudioSessionEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioSessionEvents {}
impl ::core::cmp::PartialEq for IAudioSessionEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSessionEvents {}
impl ::core::fmt::Debug for IAudioSessionEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSessionEvents").field(&self.0).finish()
    }
}
impl FromIntoMemory for IAudioSessionEvents {
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
impl crate::core::ComInterface for IAudioSessionEvents {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x24918acc_64b3_37c1_8ca9_74a66e9957a8);
}
pub struct IAudioSessionManager(pub crate::core::IUnknown);
pub trait IAudioSessionManager_Trait: crate::core::IUnknown_Trait {
    fn GetAudioSessionControl(
        &self,
        audio_session_guid: ConstPtr<crate::core::GUID>,
        stream_flags: u32,
        session_control: MutPtr<IAudioSessionControl>,
    ) -> crate::core::HRESULT {
        todo!("GetAudioSessionControl")
    }
    fn GetSimpleAudioVolume(
        &self,
        audio_session_guid: ConstPtr<crate::core::GUID>,
        stream_flags: u32,
        audio_volume: MutPtr<ISimpleAudioVolume>,
    ) -> crate::core::HRESULT {
        todo!("GetSimpleAudioVolume")
    }
}
impl ::core::clone::Clone for IAudioSessionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioSessionManager {}
impl ::core::cmp::PartialEq for IAudioSessionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSessionManager {}
impl ::core::fmt::Debug for IAudioSessionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSessionManager")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IAudioSessionManager {
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
impl crate::core::ComInterface for IAudioSessionManager {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xbfa971f1_4d5e_40bb_935e_967039bfbee4);
}
pub struct IAudioSessionManager2(pub crate::core::IUnknown);
pub trait IAudioSessionManager2_Trait: IAudioSessionManager_Trait {
    fn GetSessionEnumerator(
        &self,
        session_enum: MutPtr<IAudioSessionEnumerator>,
    ) -> crate::core::HRESULT {
        todo!("GetSessionEnumerator")
    }
    fn RegisterSessionNotification(
        &self,
        session_notification: IAudioSessionNotification,
    ) -> crate::core::HRESULT {
        todo!("RegisterSessionNotification")
    }
    fn UnregisterSessionNotification(
        &self,
        session_notification: IAudioSessionNotification,
    ) -> crate::core::HRESULT {
        todo!("UnregisterSessionNotification")
    }
    fn RegisterDuckNotification(
        &self,
        session_id: PCWSTR,
        duck_notification: IAudioVolumeDuckNotification,
    ) -> crate::core::HRESULT {
        todo!("RegisterDuckNotification")
    }
    fn UnregisterDuckNotification(
        &self,
        duck_notification: IAudioVolumeDuckNotification,
    ) -> crate::core::HRESULT {
        todo!("UnregisterDuckNotification")
    }
}
impl ::core::clone::Clone for IAudioSessionManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioSessionManager2 {}
impl ::core::cmp::PartialEq for IAudioSessionManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSessionManager2 {}
impl ::core::fmt::Debug for IAudioSessionManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSessionManager2")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IAudioSessionManager2 {
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
impl crate::core::ComInterface for IAudioSessionManager2 {
    type Super = IAudioSessionManager;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x77aa99a0_1bd6_484f_8bc7_2c654c9a9b6f);
}
pub struct IAudioSessionNotification(pub crate::core::IUnknown);
pub trait IAudioSessionNotification_Trait: crate::core::IUnknown_Trait {
    fn OnSessionCreated(&self, new_session: IAudioSessionControl) -> crate::core::HRESULT {
        todo!("OnSessionCreated")
    }
}
impl ::core::clone::Clone for IAudioSessionNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioSessionNotification {}
impl ::core::cmp::PartialEq for IAudioSessionNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSessionNotification {}
impl ::core::fmt::Debug for IAudioSessionNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSessionNotification")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IAudioSessionNotification {
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
impl crate::core::ComInterface for IAudioSessionNotification {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x641dd20b_4d41_49cc_aba3_174b9477bb08);
}
pub struct IAudioStateMonitor(pub crate::core::IUnknown);
pub trait IAudioStateMonitor_Trait: crate::core::IUnknown_Trait {
    fn RegisterCallback(
        &self,
        callback: PAudioStateMonitorCallback,
        context: ConstPtr<::core::ffi::c_void>,
        registration: MutPtr<i64>,
    ) -> crate::core::HRESULT {
        todo!("RegisterCallback")
    }
    fn UnregisterCallback(&self, registration: i64) {
        todo!("UnregisterCallback")
    }
    fn GetSoundLevel(&self) -> AudioStateMonitorSoundLevel {
        todo!("GetSoundLevel")
    }
}
impl ::core::clone::Clone for IAudioStateMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioStateMonitor {}
impl ::core::cmp::PartialEq for IAudioStateMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioStateMonitor {}
impl ::core::fmt::Debug for IAudioStateMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioStateMonitor").field(&self.0).finish()
    }
}
impl FromIntoMemory for IAudioStateMonitor {
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
impl crate::core::ComInterface for IAudioStateMonitor {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x63bd8738_e30d_4c77_bf5c_834e87c657e2);
}
pub struct IAudioStreamVolume(pub crate::core::IUnknown);
pub trait IAudioStreamVolume_Trait: crate::core::IUnknown_Trait {
    fn GetChannelCount(&self, pdw_count: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetChannelCount")
    }
    fn SetChannelVolume(&self, dw_index: u32, f_level: f32) -> crate::core::HRESULT {
        todo!("SetChannelVolume")
    }
    fn GetChannelVolume(&self, dw_index: u32, pf_level: MutPtr<f32>) -> crate::core::HRESULT {
        todo!("GetChannelVolume")
    }
    fn SetAllVolumes(&self, dw_count: u32, pf_volumes: ConstPtr<f32>) -> crate::core::HRESULT {
        todo!("SetAllVolumes")
    }
    fn GetAllVolumes(&self, dw_count: u32, pf_volumes: MutPtr<f32>) -> crate::core::HRESULT {
        todo!("GetAllVolumes")
    }
}
impl ::core::clone::Clone for IAudioStreamVolume {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioStreamVolume {}
impl ::core::cmp::PartialEq for IAudioStreamVolume {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioStreamVolume {}
impl ::core::fmt::Debug for IAudioStreamVolume {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioStreamVolume").field(&self.0).finish()
    }
}
impl FromIntoMemory for IAudioStreamVolume {
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
impl crate::core::ComInterface for IAudioStreamVolume {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x93014887_242d_4068_8a15_cf5e93b90fe3);
}
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IAudioSystemEffectsPropertyChangeNotificationClient(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IAudioSystemEffectsPropertyChangeNotificationClient_Trait:
    crate::core::IUnknown_Trait
{
    #[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn OnPropertyChanged(
        &self,
        r#type: __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002,
        key: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY,
    ) -> crate::core::HRESULT {
        todo!("OnPropertyChanged")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IAudioSystemEffectsPropertyChangeNotificationClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IAudioSystemEffectsPropertyChangeNotificationClient {}
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IAudioSystemEffectsPropertyChangeNotificationClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IAudioSystemEffectsPropertyChangeNotificationClient {}
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IAudioSystemEffectsPropertyChangeNotificationClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSystemEffectsPropertyChangeNotificationClient")
            .field(&self.0)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IAudioSystemEffectsPropertyChangeNotificationClient {
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
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IAudioSystemEffectsPropertyChangeNotificationClient {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x20049d40_56d5_400e_a2ef_385599feed49);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IAudioSystemEffectsPropertyStore(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IAudioSystemEffectsPropertyStore_Trait: crate::core::IUnknown_Trait {
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn OpenDefaultPropertyStore(
        &self,
        stgm_access: u32,
        prop_store: MutPtr<super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    ) -> crate::core::HRESULT {
        todo!("OpenDefaultPropertyStore")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn OpenUserPropertyStore(
        &self,
        stgm_access: u32,
        prop_store: MutPtr<super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    ) -> crate::core::HRESULT {
        todo!("OpenUserPropertyStore")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn OpenVolatilePropertyStore(
        &self,
        stgm_access: u32,
        prop_store: MutPtr<super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    ) -> crate::core::HRESULT {
        todo!("OpenVolatilePropertyStore")
    }
    fn ResetUserPropertyStore(&self) -> crate::core::HRESULT {
        todo!("ResetUserPropertyStore")
    }
    fn ResetVolatilePropertyStore(&self) -> crate::core::HRESULT {
        todo!("ResetVolatilePropertyStore")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn RegisterPropertyChangeNotification(
        &self,
        callback: IAudioSystemEffectsPropertyChangeNotificationClient,
    ) -> crate::core::HRESULT {
        todo!("RegisterPropertyChangeNotification")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn UnregisterPropertyChangeNotification(
        &self,
        callback: IAudioSystemEffectsPropertyChangeNotificationClient,
    ) -> crate::core::HRESULT {
        todo!("UnregisterPropertyChangeNotification")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IAudioSystemEffectsPropertyStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IAudioSystemEffectsPropertyStore {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IAudioSystemEffectsPropertyStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IAudioSystemEffectsPropertyStore {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IAudioSystemEffectsPropertyStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSystemEffectsPropertyStore")
            .field(&self.0)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IAudioSystemEffectsPropertyStore {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IAudioSystemEffectsPropertyStore {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x302ae7f9_d7e0_43e4_971b_1f8293613d2a);
}
pub struct IAudioTreble(pub crate::core::IUnknown);
pub trait IAudioTreble_Trait: IPerChannelDbLevel_Trait {}
impl ::core::clone::Clone for IAudioTreble {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioTreble {}
impl ::core::cmp::PartialEq for IAudioTreble {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioTreble {}
impl ::core::fmt::Debug for IAudioTreble {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioTreble").field(&self.0).finish()
    }
}
impl FromIntoMemory for IAudioTreble {
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
impl crate::core::ComInterface for IAudioTreble {
    type Super = IPerChannelDbLevel;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x0a717812_694e_4907_b74b_bafa5cfdca7b);
}
pub struct IAudioVolumeDuckNotification(pub crate::core::IUnknown);
pub trait IAudioVolumeDuckNotification_Trait: crate::core::IUnknown_Trait {
    fn OnVolumeDuckNotification(
        &self,
        session_id: PCWSTR,
        count_communication_sessions: u32,
    ) -> crate::core::HRESULT {
        todo!("OnVolumeDuckNotification")
    }
    fn OnVolumeUnduckNotification(&self, session_id: PCWSTR) -> crate::core::HRESULT {
        todo!("OnVolumeUnduckNotification")
    }
}
impl ::core::clone::Clone for IAudioVolumeDuckNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioVolumeDuckNotification {}
impl ::core::cmp::PartialEq for IAudioVolumeDuckNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioVolumeDuckNotification {}
impl ::core::fmt::Debug for IAudioVolumeDuckNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioVolumeDuckNotification")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IAudioVolumeDuckNotification {
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
impl crate::core::ComInterface for IAudioVolumeDuckNotification {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xc3b284d4_6d39_4359_b3cf_b56ddb3bb39c);
}
pub struct IAudioVolumeLevel(pub crate::core::IUnknown);
pub trait IAudioVolumeLevel_Trait: IPerChannelDbLevel_Trait {}
impl ::core::clone::Clone for IAudioVolumeLevel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IAudioVolumeLevel {}
impl ::core::cmp::PartialEq for IAudioVolumeLevel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioVolumeLevel {}
impl ::core::fmt::Debug for IAudioVolumeLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioVolumeLevel").field(&self.0).finish()
    }
}
impl FromIntoMemory for IAudioVolumeLevel {
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
impl crate::core::ComInterface for IAudioVolumeLevel {
    type Super = IPerChannelDbLevel;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x7fb7b48f_531d_44a2_bcb3_5ad5a134b3dc);
}
pub struct IChannelAudioVolume(pub crate::core::IUnknown);
pub trait IChannelAudioVolume_Trait: crate::core::IUnknown_Trait {
    fn GetChannelCount(&self, pdw_count: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetChannelCount")
    }
    fn SetChannelVolume(
        &self,
        dw_index: u32,
        f_level: f32,
        event_context: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("SetChannelVolume")
    }
    fn GetChannelVolume(&self, dw_index: u32, pf_level: MutPtr<f32>) -> crate::core::HRESULT {
        todo!("GetChannelVolume")
    }
    fn SetAllVolumes(
        &self,
        dw_count: u32,
        pf_volumes: ConstPtr<f32>,
        event_context: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("SetAllVolumes")
    }
    fn GetAllVolumes(&self, dw_count: u32, pf_volumes: MutPtr<f32>) -> crate::core::HRESULT {
        todo!("GetAllVolumes")
    }
}
impl ::core::clone::Clone for IChannelAudioVolume {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IChannelAudioVolume {}
impl ::core::cmp::PartialEq for IChannelAudioVolume {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IChannelAudioVolume {}
impl ::core::fmt::Debug for IChannelAudioVolume {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IChannelAudioVolume").field(&self.0).finish()
    }
}
impl FromIntoMemory for IChannelAudioVolume {
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
impl crate::core::ComInterface for IChannelAudioVolume {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x1c158861_b533_4b30_b1cf_e853e51c59b8);
}
pub struct IConnector(pub crate::core::IUnknown);
pub trait IConnector_Trait: crate::core::IUnknown_Trait {
    fn GetType(&self, p_type: MutPtr<ConnectorType>) -> crate::core::HRESULT {
        todo!("GetType")
    }
    fn GetDataFlow(&self, p_flow: MutPtr<DataFlow>) -> crate::core::HRESULT {
        todo!("GetDataFlow")
    }
    fn ConnectTo(&self, p_connect_to: IConnector) -> crate::core::HRESULT {
        todo!("ConnectTo")
    }
    fn Disconnect(&self) -> crate::core::HRESULT {
        todo!("Disconnect")
    }
    fn IsConnected(
        &self,
        pb_connected: MutPtr<super::super::Foundation::BOOL>,
    ) -> crate::core::HRESULT {
        todo!("IsConnected")
    }
    fn GetConnectedTo(&self, pp_con_to: MutPtr<IConnector>) -> crate::core::HRESULT {
        todo!("GetConnectedTo")
    }
    fn GetConnectorIdConnectedTo(
        &self,
        ppwstr_connector_id: MutPtr<PWSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetConnectorIdConnectedTo")
    }
    fn GetDeviceIdConnectedTo(&self, ppwstr_device_id: MutPtr<PWSTR>) -> crate::core::HRESULT {
        todo!("GetDeviceIdConnectedTo")
    }
}
impl ::core::clone::Clone for IConnector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IConnector {}
impl ::core::cmp::PartialEq for IConnector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConnector {}
impl ::core::fmt::Debug for IConnector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConnector").field(&self.0).finish()
    }
}
impl FromIntoMemory for IConnector {
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
impl crate::core::ComInterface for IConnector {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x9c2c4058_23f5_41de_877a_df3af236a09e);
}
pub struct IControlChangeNotify(pub crate::core::IUnknown);
pub trait IControlChangeNotify_Trait: crate::core::IUnknown_Trait {
    fn OnNotify(
        &self,
        dw_sender_process_id: u32,
        pguid_event_context: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("OnNotify")
    }
}
impl ::core::clone::Clone for IControlChangeNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IControlChangeNotify {}
impl ::core::cmp::PartialEq for IControlChangeNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IControlChangeNotify {}
impl ::core::fmt::Debug for IControlChangeNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IControlChangeNotify")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IControlChangeNotify {
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
impl crate::core::ComInterface for IControlChangeNotify {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xa09513ed_c709_4d21_bd7b_5f34c47f3947);
}
pub struct IControlInterface(pub crate::core::IUnknown);
pub trait IControlInterface_Trait: crate::core::IUnknown_Trait {
    fn GetName(&self, ppwstr_name: MutPtr<PWSTR>) -> crate::core::HRESULT {
        todo!("GetName")
    }
    fn GetIID(&self, p_iid: MutPtr<crate::core::GUID>) -> crate::core::HRESULT {
        todo!("GetIID")
    }
}
impl ::core::clone::Clone for IControlInterface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IControlInterface {}
impl ::core::cmp::PartialEq for IControlInterface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IControlInterface {}
impl ::core::fmt::Debug for IControlInterface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IControlInterface").field(&self.0).finish()
    }
}
impl FromIntoMemory for IControlInterface {
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
impl crate::core::ComInterface for IControlInterface {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x45d37c3f_5140_444a_ae24_400789f3cbf3);
}
pub struct IDeviceSpecificProperty(pub crate::core::IUnknown);
pub trait IDeviceSpecificProperty_Trait: crate::core::IUnknown_Trait {
    fn GetType(&self, p_v_type: MutPtr<u16>) -> crate::core::HRESULT {
        todo!("GetType")
    }
    fn GetValue(
        &self,
        pv_value: MutPtr<::core::ffi::c_void>,
        pcb_value: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("GetValue")
    }
    fn SetValue(
        &self,
        pv_value: ConstPtr<::core::ffi::c_void>,
        cb_value: u32,
        pguid_event_context: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("SetValue")
    }
    fn Get4BRange(
        &self,
        pl_min: MutPtr<i32>,
        pl_max: MutPtr<i32>,
        pl_stepping: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("Get4BRange")
    }
}
impl ::core::clone::Clone for IDeviceSpecificProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IDeviceSpecificProperty {}
impl ::core::cmp::PartialEq for IDeviceSpecificProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDeviceSpecificProperty {}
impl ::core::fmt::Debug for IDeviceSpecificProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDeviceSpecificProperty")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IDeviceSpecificProperty {
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
impl crate::core::ComInterface for IDeviceSpecificProperty {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x3b22bcbf_2586_4af0_8583_205d391b807c);
}
pub struct IDeviceTopology(pub crate::core::IUnknown);
pub trait IDeviceTopology_Trait: crate::core::IUnknown_Trait {
    fn GetConnectorCount(&self, p_count: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetConnectorCount")
    }
    fn GetConnector(&self, n_index: u32, pp_connector: MutPtr<IConnector>) -> crate::core::HRESULT {
        todo!("GetConnector")
    }
    fn GetSubunitCount(&self, p_count: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetSubunitCount")
    }
    fn GetSubunit(&self, n_index: u32, pp_subunit: MutPtr<ISubunit>) -> crate::core::HRESULT {
        todo!("GetSubunit")
    }
    fn GetPartById(&self, n_id: u32, pp_part: MutPtr<IPart>) -> crate::core::HRESULT {
        todo!("GetPartById")
    }
    fn GetDeviceId(&self, ppwstr_device_id: MutPtr<PWSTR>) -> crate::core::HRESULT {
        todo!("GetDeviceId")
    }
    fn GetSignalPath(
        &self,
        p_i_part_from: IPart,
        p_i_part_to: IPart,
        b_reject_mixed_paths: super::super::Foundation::BOOL,
        pp_parts: MutPtr<IPartsList>,
    ) -> crate::core::HRESULT {
        todo!("GetSignalPath")
    }
}
impl ::core::clone::Clone for IDeviceTopology {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IDeviceTopology {}
impl ::core::cmp::PartialEq for IDeviceTopology {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDeviceTopology {}
impl ::core::fmt::Debug for IDeviceTopology {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDeviceTopology").field(&self.0).finish()
    }
}
impl FromIntoMemory for IDeviceTopology {
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
impl crate::core::ComInterface for IDeviceTopology {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x2a07407e_6497_4a18_9787_32f79bd0d98f);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IMMDevice(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IMMDevice_Trait: crate::core::IUnknown_Trait {
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Activate(
        &self,
        iid: ConstPtr<crate::core::GUID>,
        dw_cls_ctx: super::super::System::Com::CLSCTX,
        p_activation_params: ConstPtr<super::super::System::Com::StructuredStorage::PROPVARIANT>,
        pp_interface: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("Activate")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn OpenPropertyStore(
        &self,
        stgm_access: super::super::System::Com::StructuredStorage::STGM,
        pp_properties: MutPtr<super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    ) -> crate::core::HRESULT {
        todo!("OpenPropertyStore")
    }
    fn GetId(&self, ppstr_id: MutPtr<PWSTR>) -> crate::core::HRESULT {
        todo!("GetId")
    }
    fn GetState(&self, pdw_state: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetState")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IMMDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IMMDevice {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IMMDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IMMDevice {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IMMDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMDevice").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IMMDevice {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IMMDevice {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xd666063f_1587_4e43_81f1_b948e807363f);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IMMDeviceActivator(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IMMDeviceActivator_Trait: crate::core::IUnknown_Trait {
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Activate(
        &self,
        iid: ConstPtr<crate::core::GUID>,
        p_device: IMMDevice,
        p_activation_params: ConstPtr<super::super::System::Com::StructuredStorage::PROPVARIANT>,
        pp_interface: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("Activate")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IMMDeviceActivator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IMMDeviceActivator {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IMMDeviceActivator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IMMDeviceActivator {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IMMDeviceActivator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMDeviceActivator").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IMMDeviceActivator {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IMMDeviceActivator {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x3b0d0ea4_d0a9_4b0e_935b_09516746fac0);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IMMDeviceCollection(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IMMDeviceCollection_Trait: crate::core::IUnknown_Trait {
    fn GetCount(&self, pc_devices: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetCount")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Item(&self, n_device: u32, pp_device: MutPtr<IMMDevice>) -> crate::core::HRESULT {
        todo!("Item")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IMMDeviceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IMMDeviceCollection {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IMMDeviceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IMMDeviceCollection {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IMMDeviceCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMDeviceCollection").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IMMDeviceCollection {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IMMDeviceCollection {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x0bd7a1be_7a1a_44db_8397_cc5392387b5e);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IMMDeviceEnumerator(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IMMDeviceEnumerator_Trait: crate::core::IUnknown_Trait {
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn EnumAudioEndpoints(
        &self,
        data_flow: EDataFlow,
        dw_state_mask: u32,
        pp_devices: MutPtr<IMMDeviceCollection>,
    ) -> crate::core::HRESULT {
        todo!("EnumAudioEndpoints")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetDefaultAudioEndpoint(
        &self,
        data_flow: EDataFlow,
        role: ERole,
        pp_endpoint: MutPtr<IMMDevice>,
    ) -> crate::core::HRESULT {
        todo!("GetDefaultAudioEndpoint")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetDevice(&self, pwstr_id: PCWSTR, pp_device: MutPtr<IMMDevice>) -> crate::core::HRESULT {
        todo!("GetDevice")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn RegisterEndpointNotificationCallback(
        &self,
        p_client: IMMNotificationClient,
    ) -> crate::core::HRESULT {
        todo!("RegisterEndpointNotificationCallback")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn UnregisterEndpointNotificationCallback(
        &self,
        p_client: IMMNotificationClient,
    ) -> crate::core::HRESULT {
        todo!("UnregisterEndpointNotificationCallback")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IMMDeviceEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IMMDeviceEnumerator {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IMMDeviceEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IMMDeviceEnumerator {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IMMDeviceEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMDeviceEnumerator").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IMMDeviceEnumerator {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole', 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IMMDeviceEnumerator {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xa95664d2_9614_4f35_a746_de8db63617e6);
}
pub struct IMMEndpoint(pub crate::core::IUnknown);
pub trait IMMEndpoint_Trait: crate::core::IUnknown_Trait {
    fn GetDataFlow(&self, p_data_flow: MutPtr<EDataFlow>) -> crate::core::HRESULT {
        todo!("GetDataFlow")
    }
}
impl ::core::clone::Clone for IMMEndpoint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IMMEndpoint {}
impl ::core::cmp::PartialEq for IMMEndpoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMMEndpoint {}
impl ::core::fmt::Debug for IMMEndpoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMEndpoint").field(&self.0).finish()
    }
}
impl FromIntoMemory for IMMEndpoint {
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
impl crate::core::ComInterface for IMMEndpoint {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x1be09788_6894_4089_8586_9a2a6c265ac5);
}
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IMMNotificationClient(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IMMNotificationClient_Trait: crate::core::IUnknown_Trait {
    fn OnDeviceStateChanged(
        &self,
        pwstr_device_id: PCWSTR,
        dw_new_state: u32,
    ) -> crate::core::HRESULT {
        todo!("OnDeviceStateChanged")
    }
    fn OnDeviceAdded(&self, pwstr_device_id: PCWSTR) -> crate::core::HRESULT {
        todo!("OnDeviceAdded")
    }
    fn OnDeviceRemoved(&self, pwstr_device_id: PCWSTR) -> crate::core::HRESULT {
        todo!("OnDeviceRemoved")
    }
    fn OnDefaultDeviceChanged(
        &self,
        flow: EDataFlow,
        role: ERole,
        pwstr_default_device_id: PCWSTR,
    ) -> crate::core::HRESULT {
        todo!("OnDefaultDeviceChanged")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn OnPropertyValueChanged(
        &self,
        pwstr_device_id: PCWSTR,
        key: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY,
    ) -> crate::core::HRESULT {
        todo!("OnPropertyValueChanged")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IMMNotificationClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IMMNotificationClient {}
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IMMNotificationClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IMMNotificationClient {}
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IMMNotificationClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMNotificationClient")
            .field(&self.0)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IMMNotificationClient {
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
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IMMNotificationClient {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x7991eec9_7e89_4d85_8390_6c703cec60c0);
}
pub struct IMessageFilter(pub crate::core::IUnknown);
pub trait IMessageFilter_Trait: crate::core::IUnknown_Trait {
    fn HandleInComingCall(
        &self,
        dw_call_type: u32,
        htask_caller: super::HTASK,
        dw_tick_count: u32,
        lp_interface_info: ConstPtr<super::super::System::Com::INTERFACEINFO>,
    ) -> u32 {
        todo!("HandleInComingCall")
    }
    fn RetryRejectedCall(
        &self,
        htask_callee: super::HTASK,
        dw_tick_count: u32,
        dw_reject_type: u32,
    ) -> u32 {
        todo!("RetryRejectedCall")
    }
    fn MessagePending(
        &self,
        htask_callee: super::HTASK,
        dw_tick_count: u32,
        dw_pending_type: u32,
    ) -> u32 {
        todo!("MessagePending")
    }
}
impl ::core::clone::Clone for IMessageFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IMessageFilter {}
impl ::core::cmp::PartialEq for IMessageFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMessageFilter {}
impl ::core::fmt::Debug for IMessageFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMessageFilter").field(&self.0).finish()
    }
}
impl FromIntoMemory for IMessageFilter {
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
impl crate::core::ComInterface for IMessageFilter {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x00000016_0000_0000_c000_000000000046);
}
pub struct IPart(pub crate::core::IUnknown);
pub trait IPart_Trait: crate::core::IUnknown_Trait {
    fn GetName(&self, ppwstr_name: MutPtr<PWSTR>) -> crate::core::HRESULT {
        todo!("GetName")
    }
    fn GetLocalId(&self, pn_id: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetLocalId")
    }
    fn GetGlobalId(&self, ppwstr_global_id: MutPtr<PWSTR>) -> crate::core::HRESULT {
        todo!("GetGlobalId")
    }
    fn GetPartType(&self, p_part_type: MutPtr<PartType>) -> crate::core::HRESULT {
        todo!("GetPartType")
    }
    fn GetSubType(&self, p_sub_type: MutPtr<crate::core::GUID>) -> crate::core::HRESULT {
        todo!("GetSubType")
    }
    fn GetControlInterfaceCount(&self, p_count: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetControlInterfaceCount")
    }
    fn GetControlInterface(
        &self,
        n_index: u32,
        pp_interface_desc: MutPtr<IControlInterface>,
    ) -> crate::core::HRESULT {
        todo!("GetControlInterface")
    }
    fn EnumPartsIncoming(&self, pp_parts: MutPtr<IPartsList>) -> crate::core::HRESULT {
        todo!("EnumPartsIncoming")
    }
    fn EnumPartsOutgoing(&self, pp_parts: MutPtr<IPartsList>) -> crate::core::HRESULT {
        todo!("EnumPartsOutgoing")
    }
    fn GetTopologyObject(&self, pp_topology: MutPtr<IDeviceTopology>) -> crate::core::HRESULT {
        todo!("GetTopologyObject")
    }
    fn Activate(
        &self,
        dw_cls_context: u32,
        refiid: ConstPtr<crate::core::GUID>,
        ppv_object: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("Activate")
    }
    fn RegisterControlChangeCallback(
        &self,
        riid: ConstPtr<crate::core::GUID>,
        p_notify: IControlChangeNotify,
    ) -> crate::core::HRESULT {
        todo!("RegisterControlChangeCallback")
    }
    fn UnregisterControlChangeCallback(
        &self,
        p_notify: IControlChangeNotify,
    ) -> crate::core::HRESULT {
        todo!("UnregisterControlChangeCallback")
    }
}
impl ::core::clone::Clone for IPart {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IPart {}
impl ::core::cmp::PartialEq for IPart {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPart {}
impl ::core::fmt::Debug for IPart {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPart").field(&self.0).finish()
    }
}
impl FromIntoMemory for IPart {
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
impl crate::core::ComInterface for IPart {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xae2de0e4_5bca_4f2d_aa46_5d13f8fdb3a9);
}
pub struct IPartsList(pub crate::core::IUnknown);
pub trait IPartsList_Trait: crate::core::IUnknown_Trait {
    fn GetCount(&self, p_count: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetCount")
    }
    fn GetPart(&self, n_index: u32, pp_part: MutPtr<IPart>) -> crate::core::HRESULT {
        todo!("GetPart")
    }
}
impl ::core::clone::Clone for IPartsList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IPartsList {}
impl ::core::cmp::PartialEq for IPartsList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPartsList {}
impl ::core::fmt::Debug for IPartsList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPartsList").field(&self.0).finish()
    }
}
impl FromIntoMemory for IPartsList {
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
impl crate::core::ComInterface for IPartsList {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x6daa848c_5eb0_45cc_aea5_998a2cda1ffb);
}
pub struct IPerChannelDbLevel(pub crate::core::IUnknown);
pub trait IPerChannelDbLevel_Trait: crate::core::IUnknown_Trait {
    fn GetChannelCount(&self, pc_channels: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetChannelCount")
    }
    fn GetLevelRange(
        &self,
        n_channel: u32,
        pf_min_level_db: MutPtr<f32>,
        pf_max_level_db: MutPtr<f32>,
        pf_stepping: MutPtr<f32>,
    ) -> crate::core::HRESULT {
        todo!("GetLevelRange")
    }
    fn GetLevel(&self, n_channel: u32, pf_level_db: MutPtr<f32>) -> crate::core::HRESULT {
        todo!("GetLevel")
    }
    fn SetLevel(
        &self,
        n_channel: u32,
        f_level_db: f32,
        pguid_event_context: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("SetLevel")
    }
    fn SetLevelUniform(
        &self,
        f_level_db: f32,
        pguid_event_context: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("SetLevelUniform")
    }
    fn SetLevelAllChannels(
        &self,
        a_levels_db: ConstPtr<f32>,
        c_channels: u32,
        pguid_event_context: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("SetLevelAllChannels")
    }
}
impl ::core::clone::Clone for IPerChannelDbLevel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IPerChannelDbLevel {}
impl ::core::cmp::PartialEq for IPerChannelDbLevel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPerChannelDbLevel {}
impl ::core::fmt::Debug for IPerChannelDbLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPerChannelDbLevel").field(&self.0).finish()
    }
}
impl FromIntoMemory for IPerChannelDbLevel {
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
impl crate::core::ComInterface for IPerChannelDbLevel {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xc2f8e001_f205_4bc9_99bc_c13b1e048ccb);
}
pub struct ISimpleAudioVolume(pub crate::core::IUnknown);
pub trait ISimpleAudioVolume_Trait: crate::core::IUnknown_Trait {
    fn SetMasterVolume(
        &self,
        f_level: f32,
        event_context: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("SetMasterVolume")
    }
    fn GetMasterVolume(&self, pf_level: MutPtr<f32>) -> crate::core::HRESULT {
        todo!("GetMasterVolume")
    }
    fn SetMute(
        &self,
        b_mute: super::super::Foundation::BOOL,
        event_context: ConstPtr<crate::core::GUID>,
    ) -> crate::core::HRESULT {
        todo!("SetMute")
    }
    fn GetMute(&self, pb_mute: MutPtr<super::super::Foundation::BOOL>) -> crate::core::HRESULT {
        todo!("GetMute")
    }
}
impl ::core::clone::Clone for ISimpleAudioVolume {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ISimpleAudioVolume {}
impl ::core::cmp::PartialEq for ISimpleAudioVolume {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimpleAudioVolume {}
impl ::core::fmt::Debug for ISimpleAudioVolume {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISimpleAudioVolume").field(&self.0).finish()
    }
}
impl FromIntoMemory for ISimpleAudioVolume {
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
impl crate::core::ComInterface for ISimpleAudioVolume {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x87ce5498_68d6_44e5_9215_6da47ef883d8);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ISpatialAudioClient(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait ISpatialAudioClient_Trait: crate::core::IUnknown_Trait {
    fn GetStaticObjectPosition(
        &self,
        r#type: AudioObjectType,
        x: MutPtr<f32>,
        y: MutPtr<f32>,
        z: MutPtr<f32>,
    ) -> crate::core::HRESULT {
        todo!("GetStaticObjectPosition")
    }
    fn GetNativeStaticObjectTypeMask(&self, mask: MutPtr<AudioObjectType>) -> crate::core::HRESULT {
        todo!("GetNativeStaticObjectTypeMask")
    }
    fn GetMaxDynamicObjectCount(&self, value: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetMaxDynamicObjectCount")
    }
    fn GetSupportedAudioObjectFormatEnumerator(
        &self,
        enumerator: MutPtr<IAudioFormatEnumerator>,
    ) -> crate::core::HRESULT {
        todo!("GetSupportedAudioObjectFormatEnumerator")
    }
    fn GetMaxFrameCount(
        &self,
        object_format: ConstPtr<WAVEFORMATEX>,
        frame_count_per_buffer: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("GetMaxFrameCount")
    }
    fn IsAudioObjectFormatSupported(
        &self,
        object_format: ConstPtr<WAVEFORMATEX>,
    ) -> crate::core::HRESULT {
        todo!("IsAudioObjectFormatSupported")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn IsSpatialAudioStreamAvailable(
        &self,
        stream_uuid: ConstPtr<crate::core::GUID>,
        auxiliary_info: ConstPtr<super::super::System::Com::StructuredStorage::PROPVARIANT>,
    ) -> crate::core::HRESULT {
        todo!("IsSpatialAudioStreamAvailable")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn ActivateSpatialAudioStream(
        &self,
        activation_params: ConstPtr<super::super::System::Com::StructuredStorage::PROPVARIANT>,
        riid: ConstPtr<crate::core::GUID>,
        stream: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("ActivateSpatialAudioStream")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ISpatialAudioClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ISpatialAudioClient {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ISpatialAudioClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ISpatialAudioClient {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ISpatialAudioClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioClient").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ISpatialAudioClient {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for ISpatialAudioClient {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xbbf8e066_aaaa_49be_9a4d_fd2a858ea27f);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ISpatialAudioClient2(pub crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait ISpatialAudioClient2_Trait: ISpatialAudioClient_Trait {
    fn IsOffloadCapable(
        &self,
        category: AUDIO_STREAM_CATEGORY,
        is_offload_capable: MutPtr<super::super::Foundation::BOOL>,
    ) -> crate::core::HRESULT {
        todo!("IsOffloadCapable")
    }
    fn GetMaxFrameCountForCategory(
        &self,
        category: AUDIO_STREAM_CATEGORY,
        offload_enabled: super::super::Foundation::BOOL,
        object_format: ConstPtr<WAVEFORMATEX>,
        frame_count_per_buffer: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("GetMaxFrameCountForCategory")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ISpatialAudioClient2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ISpatialAudioClient2 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ISpatialAudioClient2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ISpatialAudioClient2 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ISpatialAudioClient2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioClient2")
            .field(&self.0)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ISpatialAudioClient2 {
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
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for ISpatialAudioClient2 {
    type Super = ISpatialAudioClient;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xcaabe452_a66a_4bee_a93e_e320463f6a53);
}
pub struct ISpatialAudioMetadataClient(pub crate::core::IUnknown);
pub trait ISpatialAudioMetadataClient_Trait: crate::core::IUnknown_Trait {
    fn ActivateSpatialAudioMetadataItems(
        &self,
        max_item_count: u16,
        frame_count: u16,
        metadata_items_buffer: MutPtr<ISpatialAudioMetadataItemsBuffer>,
        metadata_items: MutPtr<ISpatialAudioMetadataItems>,
    ) -> crate::core::HRESULT {
        todo!("ActivateSpatialAudioMetadataItems")
    }
    fn GetSpatialAudioMetadataItemsBufferLength(
        &self,
        max_item_count: u16,
        buffer_length: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("GetSpatialAudioMetadataItemsBufferLength")
    }
    fn ActivateSpatialAudioMetadataWriter(
        &self,
        overflow_mode: SpatialAudioMetadataWriterOverflowMode,
        metadata_writer: MutPtr<ISpatialAudioMetadataWriter>,
    ) -> crate::core::HRESULT {
        todo!("ActivateSpatialAudioMetadataWriter")
    }
    fn ActivateSpatialAudioMetadataCopier(
        &self,
        metadata_copier: MutPtr<ISpatialAudioMetadataCopier>,
    ) -> crate::core::HRESULT {
        todo!("ActivateSpatialAudioMetadataCopier")
    }
    fn ActivateSpatialAudioMetadataReader(
        &self,
        metadata_reader: MutPtr<ISpatialAudioMetadataReader>,
    ) -> crate::core::HRESULT {
        todo!("ActivateSpatialAudioMetadataReader")
    }
}
impl ::core::clone::Clone for ISpatialAudioMetadataClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ISpatialAudioMetadataClient {}
impl ::core::cmp::PartialEq for ISpatialAudioMetadataClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioMetadataClient {}
impl ::core::fmt::Debug for ISpatialAudioMetadataClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioMetadataClient")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ISpatialAudioMetadataClient {
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
impl crate::core::ComInterface for ISpatialAudioMetadataClient {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x777d4a3b_f6ff_4a26_85dc_68d7cdeda1d4);
}
pub struct ISpatialAudioMetadataCopier(pub crate::core::IUnknown);
pub trait ISpatialAudioMetadataCopier_Trait: crate::core::IUnknown_Trait {
    fn Open(&self, metadata_items: ISpatialAudioMetadataItems) -> crate::core::HRESULT {
        todo!("Open")
    }
    fn CopyMetadataForFrames(
        &self,
        copy_frame_count: u16,
        copy_mode: SpatialAudioMetadataCopyMode,
        dst_metadata_items: ISpatialAudioMetadataItems,
        items_copied: MutPtr<u16>,
    ) -> crate::core::HRESULT {
        todo!("CopyMetadataForFrames")
    }
    fn Close(&self) -> crate::core::HRESULT {
        todo!("Close")
    }
}
impl ::core::clone::Clone for ISpatialAudioMetadataCopier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ISpatialAudioMetadataCopier {}
impl ::core::cmp::PartialEq for ISpatialAudioMetadataCopier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioMetadataCopier {}
impl ::core::fmt::Debug for ISpatialAudioMetadataCopier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioMetadataCopier")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ISpatialAudioMetadataCopier {
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
impl crate::core::ComInterface for ISpatialAudioMetadataCopier {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xd224b233_e251_4fd0_9ca2_d5ecf9a68404);
}
pub struct ISpatialAudioMetadataItems(pub crate::core::IUnknown);
pub trait ISpatialAudioMetadataItems_Trait: crate::core::IUnknown_Trait {
    fn GetFrameCount(&self, frame_count: MutPtr<u16>) -> crate::core::HRESULT {
        todo!("GetFrameCount")
    }
    fn GetItemCount(&self, item_count: MutPtr<u16>) -> crate::core::HRESULT {
        todo!("GetItemCount")
    }
    fn GetMaxItemCount(&self, max_item_count: MutPtr<u16>) -> crate::core::HRESULT {
        todo!("GetMaxItemCount")
    }
    fn GetMaxValueBufferLength(
        &self,
        max_value_buffer_length: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("GetMaxValueBufferLength")
    }
    fn GetInfo(&self, info: MutPtr<SpatialAudioMetadataItemsInfo>) -> crate::core::HRESULT {
        todo!("GetInfo")
    }
}
impl ::core::clone::Clone for ISpatialAudioMetadataItems {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ISpatialAudioMetadataItems {}
impl ::core::cmp::PartialEq for ISpatialAudioMetadataItems {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioMetadataItems {}
impl ::core::fmt::Debug for ISpatialAudioMetadataItems {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioMetadataItems")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ISpatialAudioMetadataItems {
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
impl crate::core::ComInterface for ISpatialAudioMetadataItems {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xbcd7c78f_3098_4f22_b547_a2f25a381269);
}
pub struct ISpatialAudioMetadataItemsBuffer(pub crate::core::IUnknown);
pub trait ISpatialAudioMetadataItemsBuffer_Trait: crate::core::IUnknown_Trait {
    fn AttachToBuffer(&self, buffer: MutPtr<u8>, buffer_length: u32) -> crate::core::HRESULT {
        todo!("AttachToBuffer")
    }
    fn AttachToPopulatedBuffer(
        &self,
        buffer: MutPtr<u8>,
        buffer_length: u32,
    ) -> crate::core::HRESULT {
        todo!("AttachToPopulatedBuffer")
    }
    fn DetachBuffer(&self) -> crate::core::HRESULT {
        todo!("DetachBuffer")
    }
}
impl ::core::clone::Clone for ISpatialAudioMetadataItemsBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ISpatialAudioMetadataItemsBuffer {}
impl ::core::cmp::PartialEq for ISpatialAudioMetadataItemsBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioMetadataItemsBuffer {}
impl ::core::fmt::Debug for ISpatialAudioMetadataItemsBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioMetadataItemsBuffer")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ISpatialAudioMetadataItemsBuffer {
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
impl crate::core::ComInterface for ISpatialAudioMetadataItemsBuffer {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x42640a16_e1bd_42d9_9ff6_031ab71a2dba);
}
pub struct ISpatialAudioMetadataReader(pub crate::core::IUnknown);
pub trait ISpatialAudioMetadataReader_Trait: crate::core::IUnknown_Trait {
    fn Open(&self, metadata_items: ISpatialAudioMetadataItems) -> crate::core::HRESULT {
        todo!("Open")
    }
    fn ReadNextItem(
        &self,
        command_count: MutPtr<u8>,
        frame_offset: MutPtr<u16>,
    ) -> crate::core::HRESULT {
        todo!("ReadNextItem")
    }
    fn ReadNextItemCommand(
        &self,
        command_id: MutPtr<u8>,
        value_buffer: MutPtr<::core::ffi::c_void>,
        max_value_buffer_length: u32,
        value_buffer_length: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("ReadNextItemCommand")
    }
    fn Close(&self) -> crate::core::HRESULT {
        todo!("Close")
    }
}
impl ::core::clone::Clone for ISpatialAudioMetadataReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ISpatialAudioMetadataReader {}
impl ::core::cmp::PartialEq for ISpatialAudioMetadataReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioMetadataReader {}
impl ::core::fmt::Debug for ISpatialAudioMetadataReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioMetadataReader")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ISpatialAudioMetadataReader {
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
impl crate::core::ComInterface for ISpatialAudioMetadataReader {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xb78e86a2_31d9_4c32_94d2_7df40fc7ebec);
}
pub struct ISpatialAudioMetadataWriter(pub crate::core::IUnknown);
pub trait ISpatialAudioMetadataWriter_Trait: crate::core::IUnknown_Trait {
    fn Open(&self, metadata_items: ISpatialAudioMetadataItems) -> crate::core::HRESULT {
        todo!("Open")
    }
    fn WriteNextItem(&self, frame_offset: u16) -> crate::core::HRESULT {
        todo!("WriteNextItem")
    }
    fn WriteNextItemCommand(
        &self,
        command_id: u8,
        value_buffer: ConstPtr<::core::ffi::c_void>,
        value_buffer_length: u32,
    ) -> crate::core::HRESULT {
        todo!("WriteNextItemCommand")
    }
    fn Close(&self) -> crate::core::HRESULT {
        todo!("Close")
    }
}
impl ::core::clone::Clone for ISpatialAudioMetadataWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ISpatialAudioMetadataWriter {}
impl ::core::cmp::PartialEq for ISpatialAudioMetadataWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioMetadataWriter {}
impl ::core::fmt::Debug for ISpatialAudioMetadataWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioMetadataWriter")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ISpatialAudioMetadataWriter {
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
impl crate::core::ComInterface for ISpatialAudioMetadataWriter {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x1b17ca01_2955_444d_a430_537dc589a844);
}
pub struct ISpatialAudioObject(pub crate::core::IUnknown);
pub trait ISpatialAudioObject_Trait: ISpatialAudioObjectBase_Trait {
    fn SetPosition(&self, x: f32, y: f32, z: f32) -> crate::core::HRESULT {
        todo!("SetPosition")
    }
    fn SetVolume(&self, volume: f32) -> crate::core::HRESULT {
        todo!("SetVolume")
    }
}
impl ::core::clone::Clone for ISpatialAudioObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ISpatialAudioObject {}
impl ::core::cmp::PartialEq for ISpatialAudioObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObject {}
impl ::core::fmt::Debug for ISpatialAudioObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObject").field(&self.0).finish()
    }
}
impl FromIntoMemory for ISpatialAudioObject {
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
impl crate::core::ComInterface for ISpatialAudioObject {
    type Super = ISpatialAudioObjectBase;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xdde28967_521b_46e5_8f00_bd6f2bc8ab1d);
}
pub struct ISpatialAudioObjectBase(pub crate::core::IUnknown);
pub trait ISpatialAudioObjectBase_Trait: crate::core::IUnknown_Trait {
    fn GetBuffer(
        &self,
        buffer: MutPtr<ConstPtr<u8>>,
        buffer_length: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("GetBuffer")
    }
    fn SetEndOfStream(&self, frame_count: u32) -> crate::core::HRESULT {
        todo!("SetEndOfStream")
    }
    fn IsActive(&self, is_active: MutPtr<super::super::Foundation::BOOL>) -> crate::core::HRESULT {
        todo!("IsActive")
    }
    fn GetAudioObjectType(
        &self,
        audio_object_type: MutPtr<AudioObjectType>,
    ) -> crate::core::HRESULT {
        todo!("GetAudioObjectType")
    }
}
impl ::core::clone::Clone for ISpatialAudioObjectBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ISpatialAudioObjectBase {}
impl ::core::cmp::PartialEq for ISpatialAudioObjectBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectBase {}
impl ::core::fmt::Debug for ISpatialAudioObjectBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectBase")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ISpatialAudioObjectBase {
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
impl crate::core::ComInterface for ISpatialAudioObjectBase {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xcce0b8f2_8d4d_4efb_a8cf_3d6ecf1c30e0);
}
pub struct ISpatialAudioObjectForHrtf(pub crate::core::IUnknown);
pub trait ISpatialAudioObjectForHrtf_Trait: ISpatialAudioObjectBase_Trait {
    fn SetPosition(&self, x: f32, y: f32, z: f32) -> crate::core::HRESULT {
        todo!("SetPosition")
    }
    fn SetGain(&self, gain: f32) -> crate::core::HRESULT {
        todo!("SetGain")
    }
    fn SetOrientation(&self, orientation: ConstPtr<ConstPtr<f32>>) -> crate::core::HRESULT {
        todo!("SetOrientation")
    }
    fn SetEnvironment(&self, environment: SpatialAudioHrtfEnvironmentType) -> crate::core::HRESULT {
        todo!("SetEnvironment")
    }
    fn SetDistanceDecay(
        &self,
        distance_decay: ConstPtr<SpatialAudioHrtfDistanceDecay>,
    ) -> crate::core::HRESULT {
        todo!("SetDistanceDecay")
    }
    fn SetDirectivity(
        &self,
        directivity: ConstPtr<SpatialAudioHrtfDirectivityUnion>,
    ) -> crate::core::HRESULT {
        todo!("SetDirectivity")
    }
}
impl ::core::clone::Clone for ISpatialAudioObjectForHrtf {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ISpatialAudioObjectForHrtf {}
impl ::core::cmp::PartialEq for ISpatialAudioObjectForHrtf {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectForHrtf {}
impl ::core::fmt::Debug for ISpatialAudioObjectForHrtf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectForHrtf")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ISpatialAudioObjectForHrtf {
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
impl crate::core::ComInterface for ISpatialAudioObjectForHrtf {
    type Super = ISpatialAudioObjectBase;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xd7436ade_1978_4e14_aba0_555bd8eb83b4);
}
pub struct ISpatialAudioObjectForMetadataCommands(pub crate::core::IUnknown);
pub trait ISpatialAudioObjectForMetadataCommands_Trait: ISpatialAudioObjectBase_Trait {
    fn WriteNextMetadataCommand(
        &self,
        command_id: u8,
        value_buffer: ConstPtr<::core::ffi::c_void>,
        value_buffer_length: u32,
    ) -> crate::core::HRESULT {
        todo!("WriteNextMetadataCommand")
    }
}
impl ::core::clone::Clone for ISpatialAudioObjectForMetadataCommands {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ISpatialAudioObjectForMetadataCommands {}
impl ::core::cmp::PartialEq for ISpatialAudioObjectForMetadataCommands {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectForMetadataCommands {}
impl ::core::fmt::Debug for ISpatialAudioObjectForMetadataCommands {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectForMetadataCommands")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ISpatialAudioObjectForMetadataCommands {
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
impl crate::core::ComInterface for ISpatialAudioObjectForMetadataCommands {
    type Super = ISpatialAudioObjectBase;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x0df2c94b_f5f9_472d_af6b_c46e0ac9cd05);
}
pub struct ISpatialAudioObjectForMetadataItems(pub crate::core::IUnknown);
pub trait ISpatialAudioObjectForMetadataItems_Trait: ISpatialAudioObjectBase_Trait {
    fn GetSpatialAudioMetadataItems(
        &self,
        metadata_items: MutPtr<ISpatialAudioMetadataItems>,
    ) -> crate::core::HRESULT {
        todo!("GetSpatialAudioMetadataItems")
    }
}
impl ::core::clone::Clone for ISpatialAudioObjectForMetadataItems {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ISpatialAudioObjectForMetadataItems {}
impl ::core::cmp::PartialEq for ISpatialAudioObjectForMetadataItems {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectForMetadataItems {}
impl ::core::fmt::Debug for ISpatialAudioObjectForMetadataItems {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectForMetadataItems")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ISpatialAudioObjectForMetadataItems {
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
impl crate::core::ComInterface for ISpatialAudioObjectForMetadataItems {
    type Super = ISpatialAudioObjectBase;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xddea49ff_3bc0_4377_8aad_9fbcfd808566);
}
pub struct ISpatialAudioObjectRenderStream(pub crate::core::IUnknown);
pub trait ISpatialAudioObjectRenderStream_Trait: ISpatialAudioObjectRenderStreamBase_Trait {
    fn ActivateSpatialAudioObject(
        &self,
        r#type: AudioObjectType,
        audio_object: MutPtr<ISpatialAudioObject>,
    ) -> crate::core::HRESULT {
        todo!("ActivateSpatialAudioObject")
    }
}
impl ::core::clone::Clone for ISpatialAudioObjectRenderStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ISpatialAudioObjectRenderStream {}
impl ::core::cmp::PartialEq for ISpatialAudioObjectRenderStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectRenderStream {}
impl ::core::fmt::Debug for ISpatialAudioObjectRenderStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectRenderStream")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ISpatialAudioObjectRenderStream {
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
impl crate::core::ComInterface for ISpatialAudioObjectRenderStream {
    type Super = ISpatialAudioObjectRenderStreamBase;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xbab5f473_b423_477b_85f5_b5a332a04153);
}
pub struct ISpatialAudioObjectRenderStreamBase(pub crate::core::IUnknown);
pub trait ISpatialAudioObjectRenderStreamBase_Trait: crate::core::IUnknown_Trait {
    fn GetAvailableDynamicObjectCount(&self, value: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetAvailableDynamicObjectCount")
    }
    fn GetService(
        &self,
        riid: ConstPtr<crate::core::GUID>,
        service: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("GetService")
    }
    fn Start(&self) -> crate::core::HRESULT {
        todo!("Start")
    }
    fn Stop(&self) -> crate::core::HRESULT {
        todo!("Stop")
    }
    fn Reset(&self) -> crate::core::HRESULT {
        todo!("Reset")
    }
    fn BeginUpdatingAudioObjects(
        &self,
        available_dynamic_object_count: MutPtr<u32>,
        frame_count_per_buffer: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("BeginUpdatingAudioObjects")
    }
    fn EndUpdatingAudioObjects(&self) -> crate::core::HRESULT {
        todo!("EndUpdatingAudioObjects")
    }
}
impl ::core::clone::Clone for ISpatialAudioObjectRenderStreamBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ISpatialAudioObjectRenderStreamBase {}
impl ::core::cmp::PartialEq for ISpatialAudioObjectRenderStreamBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectRenderStreamBase {}
impl ::core::fmt::Debug for ISpatialAudioObjectRenderStreamBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectRenderStreamBase")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ISpatialAudioObjectRenderStreamBase {
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
impl crate::core::ComInterface for ISpatialAudioObjectRenderStreamBase {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xfeaaf403_c1d8_450d_aa05_e0ccee7502a8);
}
pub struct ISpatialAudioObjectRenderStreamForHrtf(pub crate::core::IUnknown);
pub trait ISpatialAudioObjectRenderStreamForHrtf_Trait:
    ISpatialAudioObjectRenderStreamBase_Trait
{
    fn ActivateSpatialAudioObjectForHrtf(
        &self,
        r#type: AudioObjectType,
        audio_object: MutPtr<ISpatialAudioObjectForHrtf>,
    ) -> crate::core::HRESULT {
        todo!("ActivateSpatialAudioObjectForHrtf")
    }
}
impl ::core::clone::Clone for ISpatialAudioObjectRenderStreamForHrtf {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ISpatialAudioObjectRenderStreamForHrtf {}
impl ::core::cmp::PartialEq for ISpatialAudioObjectRenderStreamForHrtf {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectRenderStreamForHrtf {}
impl ::core::fmt::Debug for ISpatialAudioObjectRenderStreamForHrtf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectRenderStreamForHrtf")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ISpatialAudioObjectRenderStreamForHrtf {
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
impl crate::core::ComInterface for ISpatialAudioObjectRenderStreamForHrtf {
    type Super = ISpatialAudioObjectRenderStreamBase;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xe08deef9_5363_406e_9fdc_080ee247bbe0);
}
pub struct ISpatialAudioObjectRenderStreamForMetadata(pub crate::core::IUnknown);
pub trait ISpatialAudioObjectRenderStreamForMetadata_Trait:
    ISpatialAudioObjectRenderStreamBase_Trait
{
    fn ActivateSpatialAudioObjectForMetadataCommands(
        &self,
        r#type: AudioObjectType,
        audio_object: MutPtr<ISpatialAudioObjectForMetadataCommands>,
    ) -> crate::core::HRESULT {
        todo!("ActivateSpatialAudioObjectForMetadataCommands")
    }
    fn ActivateSpatialAudioObjectForMetadataItems(
        &self,
        r#type: AudioObjectType,
        audio_object: MutPtr<ISpatialAudioObjectForMetadataItems>,
    ) -> crate::core::HRESULT {
        todo!("ActivateSpatialAudioObjectForMetadataItems")
    }
}
impl ::core::clone::Clone for ISpatialAudioObjectRenderStreamForMetadata {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ISpatialAudioObjectRenderStreamForMetadata {}
impl ::core::cmp::PartialEq for ISpatialAudioObjectRenderStreamForMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectRenderStreamForMetadata {}
impl ::core::fmt::Debug for ISpatialAudioObjectRenderStreamForMetadata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectRenderStreamForMetadata")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ISpatialAudioObjectRenderStreamForMetadata {
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
impl crate::core::ComInterface for ISpatialAudioObjectRenderStreamForMetadata {
    type Super = ISpatialAudioObjectRenderStreamBase;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xbbc9c907_48d5_4a2e_a0c7_f7f0d67c1fb1);
}
pub struct ISpatialAudioObjectRenderStreamNotify(pub crate::core::IUnknown);
pub trait ISpatialAudioObjectRenderStreamNotify_Trait: crate::core::IUnknown_Trait {
    fn OnAvailableDynamicObjectCountChange(
        &self,
        sender: ISpatialAudioObjectRenderStreamBase,
        hns_compliance_deadline_time: i64,
        available_dynamic_object_count_change: u32,
    ) -> crate::core::HRESULT {
        todo!("OnAvailableDynamicObjectCountChange")
    }
}
impl ::core::clone::Clone for ISpatialAudioObjectRenderStreamNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ISpatialAudioObjectRenderStreamNotify {}
impl ::core::cmp::PartialEq for ISpatialAudioObjectRenderStreamNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectRenderStreamNotify {}
impl ::core::fmt::Debug for ISpatialAudioObjectRenderStreamNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectRenderStreamNotify")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ISpatialAudioObjectRenderStreamNotify {
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
impl crate::core::ComInterface for ISpatialAudioObjectRenderStreamNotify {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xdddf83e6_68d7_4c70_883f_a1836afb4a50);
}
pub struct ISubunit(pub crate::core::IUnknown);
pub trait ISubunit_Trait: crate::core::IUnknown_Trait {}
impl ::core::clone::Clone for ISubunit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ISubunit {}
impl ::core::cmp::PartialEq for ISubunit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISubunit {}
impl ::core::fmt::Debug for ISubunit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISubunit").field(&self.0).finish()
    }
}
impl FromIntoMemory for ISubunit {
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
impl crate::core::ComInterface for ISubunit {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x82149a85_dba6_4487_86bb_ea8f7fefcc71);
}
pub type LPACMDRIVERPROC = StdCallFnPtr<
    (
        PtrRepr,
        HACMDRIVERID,
        u32,
        super::super::Foundation::LPARAM,
        super::super::Foundation::LPARAM,
    ),
    super::super::Foundation::LRESULT,
>;
pub type LPMIDICALLBACK =
    StdCallFnPtr<(super::Multimedia::HDRVR, u32, PtrRepr, PtrRepr, PtrRepr), ()>;
pub type LPWAVECALLBACK =
    StdCallFnPtr<(super::Multimedia::HDRVR, u32, PtrRepr, PtrRepr, PtrRepr), ()>;
pub const MEVT_F_CALLBACK: i32 = 1073741824i32;
pub const MEVT_F_LONG: i32 = -2147483648i32;
pub const MEVT_F_SHORT: i32 = 0i32;
pub const MHDR_DONE: u32 = 1u32;
pub const MHDR_INQUEUE: u32 = 4u32;
pub const MHDR_ISSTRM: u32 = 8u32;
pub const MHDR_PREPARED: u32 = 2u32;
pub const MIDICAPS_CACHE: u32 = 4u32;
pub const MIDICAPS_LRVOLUME: u32 = 2u32;
pub const MIDICAPS_STREAM: u32 = 8u32;
pub const MIDICAPS_VOLUME: u32 = 1u32;
pub const MIDIERR_BADOPENMODE: u32 = 70u32;
pub const MIDIERR_DONT_CONTINUE: u32 = 71u32;
pub const MIDIERR_INVALIDSETUP: u32 = 69u32;
pub const MIDIERR_LASTERROR: u32 = 71u32;
pub const MIDIERR_NODEVICE: u32 = 68u32;
pub const MIDIERR_NOMAP: u32 = 66u32;
pub const MIDIERR_NOTREADY: u32 = 67u32;
pub const MIDIERR_STILLPLAYING: u32 = 65u32;
pub const MIDIERR_UNPREPARED: u32 = 64u32;
pub struct MIDIEVENT {
    pub dwDeltaTime: u32,
    pub dwStreamID: u32,
    pub dwEvent: u32,
    pub dwParms: [u32; 1],
}
impl ::core::marker::Copy for MIDIEVENT {}
impl ::core::clone::Clone for MIDIEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIDIEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDIEVENT")
            .field("dwDeltaTime", &self.dwDeltaTime)
            .field("dwStreamID", &self.dwStreamID)
            .field("dwEvent", &self.dwEvent)
            .field("dwParms", &self.dwParms)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIDIEVENT {
    fn eq(&self, other: &Self) -> bool {
        self.dwDeltaTime == other.dwDeltaTime
            && self.dwStreamID == other.dwStreamID
            && self.dwEvent == other.dwEvent
            && self.dwParms == other.dwParms
    }
}
impl ::core::cmp::Eq for MIDIEVENT {}
impl FromIntoMemory for MIDIEVENT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dwDeltaTime = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwStreamID = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwEvent = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwParms = <[u32; 1] as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            dwDeltaTime: f_dwDeltaTime,
            dwStreamID: f_dwStreamID,
            dwEvent: f_dwEvent,
            dwParms: f_dwParms,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dwDeltaTime, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwStreamID, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwEvent, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwParms, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct MIDIHDR {
    pub lpData: PSTR,
    pub dwBufferLength: u32,
    pub dwBytesRecorded: u32,
    pub dwUser: PtrRepr,
    pub dwFlags: u32,
    pub lpNext: MutPtr<MIDIHDR>,
    pub reserved: PtrRepr,
    pub dwOffset: u32,
    pub dwReserved: [PtrRepr; 8],
}
impl ::core::marker::Copy for MIDIHDR {}
impl ::core::clone::Clone for MIDIHDR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIDIHDR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDIHDR")
            .field("lpData", &self.lpData)
            .field("dwBufferLength", &self.dwBufferLength)
            .field("dwBytesRecorded", &self.dwBytesRecorded)
            .field("dwUser", &self.dwUser)
            .field("dwFlags", &self.dwFlags)
            .field("lpNext", &self.lpNext)
            .field("reserved", &self.reserved)
            .field("dwOffset", &self.dwOffset)
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIDIHDR {
    fn eq(&self, other: &Self) -> bool {
        self.lpData == other.lpData
            && self.dwBufferLength == other.dwBufferLength
            && self.dwBytesRecorded == other.dwBytesRecorded
            && self.dwUser == other.dwUser
            && self.dwFlags == other.dwFlags
            && self.lpNext == other.lpNext
            && self.reserved == other.reserved
            && self.dwOffset == other.dwOffset
            && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for MIDIHDR {}
impl FromIntoMemory for MIDIHDR {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 64);
        let f_lpData = <PSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwBufferLength = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwBytesRecorded = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwUser = <PtrRepr as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_lpNext = <MutPtr<MIDIHDR> as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_reserved = <PtrRepr as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_dwOffset = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_dwReserved = <[PtrRepr; 8] as FromIntoMemory>::from_bytes(&from[32..32 + 32]);
        Self {
            lpData: f_lpData,
            dwBufferLength: f_dwBufferLength,
            dwBytesRecorded: f_dwBytesRecorded,
            dwUser: f_dwUser,
            dwFlags: f_dwFlags,
            lpNext: f_lpNext,
            reserved: f_reserved,
            dwOffset: f_dwOffset,
            dwReserved: f_dwReserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 64);
        FromIntoMemory::into_bytes(self.lpData, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwBufferLength, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwBytesRecorded, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwUser, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.lpNext, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.reserved, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.dwOffset, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.dwReserved, &mut into[32..32 + 32]);
    }
    fn size() -> usize {
        64
    }
}
pub struct MIDIINCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub dwSupport: u32,
    pub ManufacturerGuid: crate::core::GUID,
    pub ProductGuid: crate::core::GUID,
    pub NameGuid: crate::core::GUID,
}
impl ::core::marker::Copy for MIDIINCAPS2A {}
impl ::core::clone::Clone for MIDIINCAPS2A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIDIINCAPS2A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDIINCAPS2A")
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("vDriverVersion", &self.vDriverVersion)
            .field("szPname", &self.szPname)
            .field("dwSupport", &self.dwSupport)
            .field("ManufacturerGuid", &self.ManufacturerGuid)
            .field("ProductGuid", &self.ProductGuid)
            .field("NameGuid", &self.NameGuid)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIDIINCAPS2A {
    fn eq(&self, other: &Self) -> bool {
        self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.vDriverVersion == other.vDriverVersion
            && self.szPname == other.szPname
            && self.dwSupport == other.dwSupport
            && self.ManufacturerGuid == other.ManufacturerGuid
            && self.ProductGuid == other.ProductGuid
            && self.NameGuid == other.NameGuid
    }
}
impl ::core::cmp::Eq for MIDIINCAPS2A {}
impl FromIntoMemory for MIDIINCAPS2A {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 92);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_vDriverVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szPname =
            <[super::super::Foundation::CHAR; 32] as FromIntoMemory>::from_bytes(&from[8..8 + 32]);
        let f_dwSupport = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_ManufacturerGuid =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[44..44 + 16]);
        let f_ProductGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[60..60 + 16]);
        let f_NameGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[76..76 + 16]);
        Self {
            wMid: f_wMid,
            wPid: f_wPid,
            vDriverVersion: f_vDriverVersion,
            szPname: f_szPname,
            dwSupport: f_dwSupport,
            ManufacturerGuid: f_ManufacturerGuid,
            ProductGuid: f_ProductGuid,
            NameGuid: f_NameGuid,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 92);
        FromIntoMemory::into_bytes(self.wMid, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.vDriverVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[8..8 + 32]);
        FromIntoMemory::into_bytes(self.dwSupport, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.ManufacturerGuid, &mut into[44..44 + 16]);
        FromIntoMemory::into_bytes(self.ProductGuid, &mut into[60..60 + 16]);
        FromIntoMemory::into_bytes(self.NameGuid, &mut into[76..76 + 16]);
    }
    fn size() -> usize {
        92
    }
}
pub struct MIDIINCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwSupport: u32,
    pub ManufacturerGuid: crate::core::GUID,
    pub ProductGuid: crate::core::GUID,
    pub NameGuid: crate::core::GUID,
}
impl ::core::marker::Copy for MIDIINCAPS2W {}
impl ::core::clone::Clone for MIDIINCAPS2W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIDIINCAPS2W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDIINCAPS2W")
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("vDriverVersion", &self.vDriverVersion)
            .field("szPname", &self.szPname)
            .field("dwSupport", &self.dwSupport)
            .field("ManufacturerGuid", &self.ManufacturerGuid)
            .field("ProductGuid", &self.ProductGuid)
            .field("NameGuid", &self.NameGuid)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIDIINCAPS2W {
    fn eq(&self, other: &Self) -> bool {
        self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.vDriverVersion == other.vDriverVersion
            && self.szPname == other.szPname
            && self.dwSupport == other.dwSupport
            && self.ManufacturerGuid == other.ManufacturerGuid
            && self.ProductGuid == other.ProductGuid
            && self.NameGuid == other.NameGuid
    }
}
impl ::core::cmp::Eq for MIDIINCAPS2W {}
impl FromIntoMemory for MIDIINCAPS2W {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 92);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_vDriverVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szPname = <[u16; 32] as FromIntoMemory>::from_bytes(&from[8..8 + 32]);
        let f_dwSupport = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_ManufacturerGuid =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[44..44 + 16]);
        let f_ProductGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[60..60 + 16]);
        let f_NameGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[76..76 + 16]);
        Self {
            wMid: f_wMid,
            wPid: f_wPid,
            vDriverVersion: f_vDriverVersion,
            szPname: f_szPname,
            dwSupport: f_dwSupport,
            ManufacturerGuid: f_ManufacturerGuid,
            ProductGuid: f_ProductGuid,
            NameGuid: f_NameGuid,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 92);
        FromIntoMemory::into_bytes(self.wMid, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.vDriverVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[8..8 + 32]);
        FromIntoMemory::into_bytes(self.dwSupport, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.ManufacturerGuid, &mut into[44..44 + 16]);
        FromIntoMemory::into_bytes(self.ProductGuid, &mut into[60..60 + 16]);
        FromIntoMemory::into_bytes(self.NameGuid, &mut into[76..76 + 16]);
    }
    fn size() -> usize {
        92
    }
}
pub struct MIDIINCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub dwSupport: u32,
}
impl ::core::marker::Copy for MIDIINCAPSA {}
impl ::core::clone::Clone for MIDIINCAPSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIDIINCAPSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDIINCAPSA")
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("vDriverVersion", &self.vDriverVersion)
            .field("szPname", &self.szPname)
            .field("dwSupport", &self.dwSupport)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIDIINCAPSA {
    fn eq(&self, other: &Self) -> bool {
        self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.vDriverVersion == other.vDriverVersion
            && self.szPname == other.szPname
            && self.dwSupport == other.dwSupport
    }
}
impl ::core::cmp::Eq for MIDIINCAPSA {}
impl FromIntoMemory for MIDIINCAPSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 44);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_vDriverVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szPname =
            <[super::super::Foundation::CHAR; 32] as FromIntoMemory>::from_bytes(&from[8..8 + 32]);
        let f_dwSupport = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        Self {
            wMid: f_wMid,
            wPid: f_wPid,
            vDriverVersion: f_vDriverVersion,
            szPname: f_szPname,
            dwSupport: f_dwSupport,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 44);
        FromIntoMemory::into_bytes(self.wMid, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.vDriverVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[8..8 + 32]);
        FromIntoMemory::into_bytes(self.dwSupport, &mut into[40..40 + 4]);
    }
    fn size() -> usize {
        44
    }
}
pub struct MIDIINCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwSupport: u32,
}
impl ::core::marker::Copy for MIDIINCAPSW {}
impl ::core::clone::Clone for MIDIINCAPSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIDIINCAPSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDIINCAPSW")
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("vDriverVersion", &self.vDriverVersion)
            .field("szPname", &self.szPname)
            .field("dwSupport", &self.dwSupport)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIDIINCAPSW {
    fn eq(&self, other: &Self) -> bool {
        self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.vDriverVersion == other.vDriverVersion
            && self.szPname == other.szPname
            && self.dwSupport == other.dwSupport
    }
}
impl ::core::cmp::Eq for MIDIINCAPSW {}
impl FromIntoMemory for MIDIINCAPSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 44);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_vDriverVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szPname = <[u16; 32] as FromIntoMemory>::from_bytes(&from[8..8 + 32]);
        let f_dwSupport = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        Self {
            wMid: f_wMid,
            wPid: f_wPid,
            vDriverVersion: f_vDriverVersion,
            szPname: f_szPname,
            dwSupport: f_dwSupport,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 44);
        FromIntoMemory::into_bytes(self.wMid, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.vDriverVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[8..8 + 32]);
        FromIntoMemory::into_bytes(self.dwSupport, &mut into[40..40 + 4]);
    }
    fn size() -> usize {
        44
    }
}
pub struct MIDIOUTCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub wTechnology: u16,
    pub wVoices: u16,
    pub wNotes: u16,
    pub wChannelMask: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: crate::core::GUID,
    pub ProductGuid: crate::core::GUID,
    pub NameGuid: crate::core::GUID,
}
impl ::core::marker::Copy for MIDIOUTCAPS2A {}
impl ::core::clone::Clone for MIDIOUTCAPS2A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIDIOUTCAPS2A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDIOUTCAPS2A")
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("vDriverVersion", &self.vDriverVersion)
            .field("szPname", &self.szPname)
            .field("wTechnology", &self.wTechnology)
            .field("wVoices", &self.wVoices)
            .field("wNotes", &self.wNotes)
            .field("wChannelMask", &self.wChannelMask)
            .field("dwSupport", &self.dwSupport)
            .field("ManufacturerGuid", &self.ManufacturerGuid)
            .field("ProductGuid", &self.ProductGuid)
            .field("NameGuid", &self.NameGuid)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIDIOUTCAPS2A {
    fn eq(&self, other: &Self) -> bool {
        self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.vDriverVersion == other.vDriverVersion
            && self.szPname == other.szPname
            && self.wTechnology == other.wTechnology
            && self.wVoices == other.wVoices
            && self.wNotes == other.wNotes
            && self.wChannelMask == other.wChannelMask
            && self.dwSupport == other.dwSupport
            && self.ManufacturerGuid == other.ManufacturerGuid
            && self.ProductGuid == other.ProductGuid
            && self.NameGuid == other.NameGuid
    }
}
impl ::core::cmp::Eq for MIDIOUTCAPS2A {}
impl FromIntoMemory for MIDIOUTCAPS2A {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 100);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_vDriverVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szPname =
            <[super::super::Foundation::CHAR; 32] as FromIntoMemory>::from_bytes(&from[8..8 + 32]);
        let f_wTechnology = <u16 as FromIntoMemory>::from_bytes(&from[40..40 + 2]);
        let f_wVoices = <u16 as FromIntoMemory>::from_bytes(&from[42..42 + 2]);
        let f_wNotes = <u16 as FromIntoMemory>::from_bytes(&from[44..44 + 2]);
        let f_wChannelMask = <u16 as FromIntoMemory>::from_bytes(&from[46..46 + 2]);
        let f_dwSupport = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_ManufacturerGuid =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[52..52 + 16]);
        let f_ProductGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[68..68 + 16]);
        let f_NameGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[84..84 + 16]);
        Self {
            wMid: f_wMid,
            wPid: f_wPid,
            vDriverVersion: f_vDriverVersion,
            szPname: f_szPname,
            wTechnology: f_wTechnology,
            wVoices: f_wVoices,
            wNotes: f_wNotes,
            wChannelMask: f_wChannelMask,
            dwSupport: f_dwSupport,
            ManufacturerGuid: f_ManufacturerGuid,
            ProductGuid: f_ProductGuid,
            NameGuid: f_NameGuid,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 100);
        FromIntoMemory::into_bytes(self.wMid, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.vDriverVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[8..8 + 32]);
        FromIntoMemory::into_bytes(self.wTechnology, &mut into[40..40 + 2]);
        FromIntoMemory::into_bytes(self.wVoices, &mut into[42..42 + 2]);
        FromIntoMemory::into_bytes(self.wNotes, &mut into[44..44 + 2]);
        FromIntoMemory::into_bytes(self.wChannelMask, &mut into[46..46 + 2]);
        FromIntoMemory::into_bytes(self.dwSupport, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.ManufacturerGuid, &mut into[52..52 + 16]);
        FromIntoMemory::into_bytes(self.ProductGuid, &mut into[68..68 + 16]);
        FromIntoMemory::into_bytes(self.NameGuid, &mut into[84..84 + 16]);
    }
    fn size() -> usize {
        100
    }
}
pub struct MIDIOUTCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub wTechnology: u16,
    pub wVoices: u16,
    pub wNotes: u16,
    pub wChannelMask: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: crate::core::GUID,
    pub ProductGuid: crate::core::GUID,
    pub NameGuid: crate::core::GUID,
}
impl ::core::marker::Copy for MIDIOUTCAPS2W {}
impl ::core::clone::Clone for MIDIOUTCAPS2W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIDIOUTCAPS2W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDIOUTCAPS2W")
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("vDriverVersion", &self.vDriverVersion)
            .field("szPname", &self.szPname)
            .field("wTechnology", &self.wTechnology)
            .field("wVoices", &self.wVoices)
            .field("wNotes", &self.wNotes)
            .field("wChannelMask", &self.wChannelMask)
            .field("dwSupport", &self.dwSupport)
            .field("ManufacturerGuid", &self.ManufacturerGuid)
            .field("ProductGuid", &self.ProductGuid)
            .field("NameGuid", &self.NameGuid)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIDIOUTCAPS2W {
    fn eq(&self, other: &Self) -> bool {
        self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.vDriverVersion == other.vDriverVersion
            && self.szPname == other.szPname
            && self.wTechnology == other.wTechnology
            && self.wVoices == other.wVoices
            && self.wNotes == other.wNotes
            && self.wChannelMask == other.wChannelMask
            && self.dwSupport == other.dwSupport
            && self.ManufacturerGuid == other.ManufacturerGuid
            && self.ProductGuid == other.ProductGuid
            && self.NameGuid == other.NameGuid
    }
}
impl ::core::cmp::Eq for MIDIOUTCAPS2W {}
impl FromIntoMemory for MIDIOUTCAPS2W {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 100);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_vDriverVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szPname = <[u16; 32] as FromIntoMemory>::from_bytes(&from[8..8 + 32]);
        let f_wTechnology = <u16 as FromIntoMemory>::from_bytes(&from[40..40 + 2]);
        let f_wVoices = <u16 as FromIntoMemory>::from_bytes(&from[42..42 + 2]);
        let f_wNotes = <u16 as FromIntoMemory>::from_bytes(&from[44..44 + 2]);
        let f_wChannelMask = <u16 as FromIntoMemory>::from_bytes(&from[46..46 + 2]);
        let f_dwSupport = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_ManufacturerGuid =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[52..52 + 16]);
        let f_ProductGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[68..68 + 16]);
        let f_NameGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[84..84 + 16]);
        Self {
            wMid: f_wMid,
            wPid: f_wPid,
            vDriverVersion: f_vDriverVersion,
            szPname: f_szPname,
            wTechnology: f_wTechnology,
            wVoices: f_wVoices,
            wNotes: f_wNotes,
            wChannelMask: f_wChannelMask,
            dwSupport: f_dwSupport,
            ManufacturerGuid: f_ManufacturerGuid,
            ProductGuid: f_ProductGuid,
            NameGuid: f_NameGuid,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 100);
        FromIntoMemory::into_bytes(self.wMid, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.vDriverVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[8..8 + 32]);
        FromIntoMemory::into_bytes(self.wTechnology, &mut into[40..40 + 2]);
        FromIntoMemory::into_bytes(self.wVoices, &mut into[42..42 + 2]);
        FromIntoMemory::into_bytes(self.wNotes, &mut into[44..44 + 2]);
        FromIntoMemory::into_bytes(self.wChannelMask, &mut into[46..46 + 2]);
        FromIntoMemory::into_bytes(self.dwSupport, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.ManufacturerGuid, &mut into[52..52 + 16]);
        FromIntoMemory::into_bytes(self.ProductGuid, &mut into[68..68 + 16]);
        FromIntoMemory::into_bytes(self.NameGuid, &mut into[84..84 + 16]);
    }
    fn size() -> usize {
        100
    }
}
pub struct MIDIOUTCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub wTechnology: u16,
    pub wVoices: u16,
    pub wNotes: u16,
    pub wChannelMask: u16,
    pub dwSupport: u32,
}
impl ::core::marker::Copy for MIDIOUTCAPSA {}
impl ::core::clone::Clone for MIDIOUTCAPSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIDIOUTCAPSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDIOUTCAPSA")
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("vDriverVersion", &self.vDriverVersion)
            .field("szPname", &self.szPname)
            .field("wTechnology", &self.wTechnology)
            .field("wVoices", &self.wVoices)
            .field("wNotes", &self.wNotes)
            .field("wChannelMask", &self.wChannelMask)
            .field("dwSupport", &self.dwSupport)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIDIOUTCAPSA {
    fn eq(&self, other: &Self) -> bool {
        self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.vDriverVersion == other.vDriverVersion
            && self.szPname == other.szPname
            && self.wTechnology == other.wTechnology
            && self.wVoices == other.wVoices
            && self.wNotes == other.wNotes
            && self.wChannelMask == other.wChannelMask
            && self.dwSupport == other.dwSupport
    }
}
impl ::core::cmp::Eq for MIDIOUTCAPSA {}
impl FromIntoMemory for MIDIOUTCAPSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 52);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_vDriverVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szPname =
            <[super::super::Foundation::CHAR; 32] as FromIntoMemory>::from_bytes(&from[8..8 + 32]);
        let f_wTechnology = <u16 as FromIntoMemory>::from_bytes(&from[40..40 + 2]);
        let f_wVoices = <u16 as FromIntoMemory>::from_bytes(&from[42..42 + 2]);
        let f_wNotes = <u16 as FromIntoMemory>::from_bytes(&from[44..44 + 2]);
        let f_wChannelMask = <u16 as FromIntoMemory>::from_bytes(&from[46..46 + 2]);
        let f_dwSupport = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        Self {
            wMid: f_wMid,
            wPid: f_wPid,
            vDriverVersion: f_vDriverVersion,
            szPname: f_szPname,
            wTechnology: f_wTechnology,
            wVoices: f_wVoices,
            wNotes: f_wNotes,
            wChannelMask: f_wChannelMask,
            dwSupport: f_dwSupport,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 52);
        FromIntoMemory::into_bytes(self.wMid, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.vDriverVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[8..8 + 32]);
        FromIntoMemory::into_bytes(self.wTechnology, &mut into[40..40 + 2]);
        FromIntoMemory::into_bytes(self.wVoices, &mut into[42..42 + 2]);
        FromIntoMemory::into_bytes(self.wNotes, &mut into[44..44 + 2]);
        FromIntoMemory::into_bytes(self.wChannelMask, &mut into[46..46 + 2]);
        FromIntoMemory::into_bytes(self.dwSupport, &mut into[48..48 + 4]);
    }
    fn size() -> usize {
        52
    }
}
pub struct MIDIOUTCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub wTechnology: u16,
    pub wVoices: u16,
    pub wNotes: u16,
    pub wChannelMask: u16,
    pub dwSupport: u32,
}
impl ::core::marker::Copy for MIDIOUTCAPSW {}
impl ::core::clone::Clone for MIDIOUTCAPSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIDIOUTCAPSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDIOUTCAPSW")
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("vDriverVersion", &self.vDriverVersion)
            .field("szPname", &self.szPname)
            .field("wTechnology", &self.wTechnology)
            .field("wVoices", &self.wVoices)
            .field("wNotes", &self.wNotes)
            .field("wChannelMask", &self.wChannelMask)
            .field("dwSupport", &self.dwSupport)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIDIOUTCAPSW {
    fn eq(&self, other: &Self) -> bool {
        self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.vDriverVersion == other.vDriverVersion
            && self.szPname == other.szPname
            && self.wTechnology == other.wTechnology
            && self.wVoices == other.wVoices
            && self.wNotes == other.wNotes
            && self.wChannelMask == other.wChannelMask
            && self.dwSupport == other.dwSupport
    }
}
impl ::core::cmp::Eq for MIDIOUTCAPSW {}
impl FromIntoMemory for MIDIOUTCAPSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 52);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_vDriverVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szPname = <[u16; 32] as FromIntoMemory>::from_bytes(&from[8..8 + 32]);
        let f_wTechnology = <u16 as FromIntoMemory>::from_bytes(&from[40..40 + 2]);
        let f_wVoices = <u16 as FromIntoMemory>::from_bytes(&from[42..42 + 2]);
        let f_wNotes = <u16 as FromIntoMemory>::from_bytes(&from[44..44 + 2]);
        let f_wChannelMask = <u16 as FromIntoMemory>::from_bytes(&from[46..46 + 2]);
        let f_dwSupport = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        Self {
            wMid: f_wMid,
            wPid: f_wPid,
            vDriverVersion: f_vDriverVersion,
            szPname: f_szPname,
            wTechnology: f_wTechnology,
            wVoices: f_wVoices,
            wNotes: f_wNotes,
            wChannelMask: f_wChannelMask,
            dwSupport: f_dwSupport,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 52);
        FromIntoMemory::into_bytes(self.wMid, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.vDriverVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[8..8 + 32]);
        FromIntoMemory::into_bytes(self.wTechnology, &mut into[40..40 + 2]);
        FromIntoMemory::into_bytes(self.wVoices, &mut into[42..42 + 2]);
        FromIntoMemory::into_bytes(self.wNotes, &mut into[44..44 + 2]);
        FromIntoMemory::into_bytes(self.wChannelMask, &mut into[46..46 + 2]);
        FromIntoMemory::into_bytes(self.dwSupport, &mut into[48..48 + 4]);
    }
    fn size() -> usize {
        52
    }
}
pub const MIDIPATCHSIZE: u32 = 128u32;
pub struct MIDIPROPTEMPO {
    pub cbStruct: u32,
    pub dwTempo: u32,
}
impl ::core::marker::Copy for MIDIPROPTEMPO {}
impl ::core::clone::Clone for MIDIPROPTEMPO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIDIPROPTEMPO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDIPROPTEMPO")
            .field("cbStruct", &self.cbStruct)
            .field("dwTempo", &self.dwTempo)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIDIPROPTEMPO {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwTempo == other.dwTempo
    }
}
impl ::core::cmp::Eq for MIDIPROPTEMPO {}
impl FromIntoMemory for MIDIPROPTEMPO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwTempo = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            cbStruct: f_cbStruct,
            dwTempo: f_dwTempo,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwTempo, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct MIDIPROPTIMEDIV {
    pub cbStruct: u32,
    pub dwTimeDiv: u32,
}
impl ::core::marker::Copy for MIDIPROPTIMEDIV {}
impl ::core::clone::Clone for MIDIPROPTIMEDIV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIDIPROPTIMEDIV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDIPROPTIMEDIV")
            .field("cbStruct", &self.cbStruct)
            .field("dwTimeDiv", &self.dwTimeDiv)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIDIPROPTIMEDIV {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwTimeDiv == other.dwTimeDiv
    }
}
impl ::core::cmp::Eq for MIDIPROPTIMEDIV {}
impl FromIntoMemory for MIDIPROPTIMEDIV {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwTimeDiv = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            cbStruct: f_cbStruct,
            dwTimeDiv: f_dwTimeDiv,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwTimeDiv, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub const MIDIPROP_GET: i32 = 1073741824i32;
pub const MIDIPROP_SET: i32 = -2147483648i32;
pub const MIDIPROP_TEMPO: i32 = 2i32;
pub const MIDIPROP_TIMEDIV: i32 = 1i32;
pub struct MIDISTRMBUFFVER {
    pub dwVersion: u32,
    pub dwMid: u32,
    pub dwOEMVersion: u32,
}
impl ::core::marker::Copy for MIDISTRMBUFFVER {}
impl ::core::clone::Clone for MIDISTRMBUFFVER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIDISTRMBUFFVER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDISTRMBUFFVER")
            .field("dwVersion", &self.dwVersion)
            .field("dwMid", &self.dwMid)
            .field("dwOEMVersion", &self.dwOEMVersion)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIDISTRMBUFFVER {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion
            && self.dwMid == other.dwMid
            && self.dwOEMVersion == other.dwOEMVersion
    }
}
impl ::core::cmp::Eq for MIDISTRMBUFFVER {}
impl FromIntoMemory for MIDISTRMBUFFVER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_dwVersion = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwMid = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwOEMVersion = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            dwVersion: f_dwVersion,
            dwMid: f_dwMid,
            dwOEMVersion: f_dwOEMVersion,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.dwVersion, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwMid, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwOEMVersion, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub const MIDISTRM_ERROR: i32 = -2i32;
pub const MIDI_CACHE_ALL: u32 = 1u32;
pub const MIDI_CACHE_BESTFIT: u32 = 2u32;
pub const MIDI_CACHE_QUERY: u32 = 3u32;
pub const MIDI_UNCACHE: u32 = 4u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MIDI_WAVE_OPEN_TYPE(pub u32);
pub const CALLBACK_TYPEMASK: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(458752u32);
pub const CALLBACK_NULL: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(0u32);
pub const CALLBACK_WINDOW: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(65536u32);
pub const CALLBACK_TASK: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(131072u32);
pub const CALLBACK_FUNCTION: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(196608u32);
pub const CALLBACK_THREAD: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(131072u32);
pub const CALLBACK_EVENT: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(327680u32);
pub const WAVE_FORMAT_QUERY: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(1u32);
pub const WAVE_ALLOWSYNC: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(2u32);
pub const WAVE_MAPPED: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(4u32);
pub const WAVE_FORMAT_DIRECT: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(8u32);
pub const WAVE_FORMAT_DIRECT_QUERY: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(9u32);
pub const WAVE_MAPPED_DEFAULT_COMMUNICATION_DEVICE: MIDI_WAVE_OPEN_TYPE =
    MIDI_WAVE_OPEN_TYPE(16u32);
pub const MIDI_IO_STATUS: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(32u32);
impl ::core::marker::Copy for MIDI_WAVE_OPEN_TYPE {}
impl ::core::clone::Clone for MIDI_WAVE_OPEN_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MIDI_WAVE_OPEN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MIDI_WAVE_OPEN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIDI_WAVE_OPEN_TYPE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MIDI_WAVE_OPEN_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MIDI_WAVE_OPEN_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MIDI_WAVE_OPEN_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MIDI_WAVE_OPEN_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MIDI_WAVE_OPEN_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for MIDI_WAVE_OPEN_TYPE {
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
pub struct MIXERCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub fdwSupport: u32,
    pub cDestinations: u32,
    pub ManufacturerGuid: crate::core::GUID,
    pub ProductGuid: crate::core::GUID,
    pub NameGuid: crate::core::GUID,
}
impl ::core::marker::Copy for MIXERCAPS2A {}
impl ::core::clone::Clone for MIXERCAPS2A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXERCAPS2A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXERCAPS2A")
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("vDriverVersion", &self.vDriverVersion)
            .field("szPname", &self.szPname)
            .field("fdwSupport", &self.fdwSupport)
            .field("cDestinations", &self.cDestinations)
            .field("ManufacturerGuid", &self.ManufacturerGuid)
            .field("ProductGuid", &self.ProductGuid)
            .field("NameGuid", &self.NameGuid)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXERCAPS2A {
    fn eq(&self, other: &Self) -> bool {
        self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.vDriverVersion == other.vDriverVersion
            && self.szPname == other.szPname
            && self.fdwSupport == other.fdwSupport
            && self.cDestinations == other.cDestinations
            && self.ManufacturerGuid == other.ManufacturerGuid
            && self.ProductGuid == other.ProductGuid
            && self.NameGuid == other.NameGuid
    }
}
impl ::core::cmp::Eq for MIXERCAPS2A {}
impl FromIntoMemory for MIXERCAPS2A {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 96);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_vDriverVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szPname =
            <[super::super::Foundation::CHAR; 32] as FromIntoMemory>::from_bytes(&from[8..8 + 32]);
        let f_fdwSupport = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_cDestinations = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_ManufacturerGuid =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[48..48 + 16]);
        let f_ProductGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[64..64 + 16]);
        let f_NameGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[80..80 + 16]);
        Self {
            wMid: f_wMid,
            wPid: f_wPid,
            vDriverVersion: f_vDriverVersion,
            szPname: f_szPname,
            fdwSupport: f_fdwSupport,
            cDestinations: f_cDestinations,
            ManufacturerGuid: f_ManufacturerGuid,
            ProductGuid: f_ProductGuid,
            NameGuid: f_NameGuid,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 96);
        FromIntoMemory::into_bytes(self.wMid, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.vDriverVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[8..8 + 32]);
        FromIntoMemory::into_bytes(self.fdwSupport, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.cDestinations, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.ManufacturerGuid, &mut into[48..48 + 16]);
        FromIntoMemory::into_bytes(self.ProductGuid, &mut into[64..64 + 16]);
        FromIntoMemory::into_bytes(self.NameGuid, &mut into[80..80 + 16]);
    }
    fn size() -> usize {
        96
    }
}
pub struct MIXERCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub fdwSupport: u32,
    pub cDestinations: u32,
    pub ManufacturerGuid: crate::core::GUID,
    pub ProductGuid: crate::core::GUID,
    pub NameGuid: crate::core::GUID,
}
impl ::core::marker::Copy for MIXERCAPS2W {}
impl ::core::clone::Clone for MIXERCAPS2W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXERCAPS2W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXERCAPS2W")
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("vDriverVersion", &self.vDriverVersion)
            .field("szPname", &self.szPname)
            .field("fdwSupport", &self.fdwSupport)
            .field("cDestinations", &self.cDestinations)
            .field("ManufacturerGuid", &self.ManufacturerGuid)
            .field("ProductGuid", &self.ProductGuid)
            .field("NameGuid", &self.NameGuid)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXERCAPS2W {
    fn eq(&self, other: &Self) -> bool {
        self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.vDriverVersion == other.vDriverVersion
            && self.szPname == other.szPname
            && self.fdwSupport == other.fdwSupport
            && self.cDestinations == other.cDestinations
            && self.ManufacturerGuid == other.ManufacturerGuid
            && self.ProductGuid == other.ProductGuid
            && self.NameGuid == other.NameGuid
    }
}
impl ::core::cmp::Eq for MIXERCAPS2W {}
impl FromIntoMemory for MIXERCAPS2W {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 96);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_vDriverVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szPname = <[u16; 32] as FromIntoMemory>::from_bytes(&from[8..8 + 32]);
        let f_fdwSupport = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_cDestinations = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_ManufacturerGuid =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[48..48 + 16]);
        let f_ProductGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[64..64 + 16]);
        let f_NameGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[80..80 + 16]);
        Self {
            wMid: f_wMid,
            wPid: f_wPid,
            vDriverVersion: f_vDriverVersion,
            szPname: f_szPname,
            fdwSupport: f_fdwSupport,
            cDestinations: f_cDestinations,
            ManufacturerGuid: f_ManufacturerGuid,
            ProductGuid: f_ProductGuid,
            NameGuid: f_NameGuid,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 96);
        FromIntoMemory::into_bytes(self.wMid, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.vDriverVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[8..8 + 32]);
        FromIntoMemory::into_bytes(self.fdwSupport, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.cDestinations, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.ManufacturerGuid, &mut into[48..48 + 16]);
        FromIntoMemory::into_bytes(self.ProductGuid, &mut into[64..64 + 16]);
        FromIntoMemory::into_bytes(self.NameGuid, &mut into[80..80 + 16]);
    }
    fn size() -> usize {
        96
    }
}
pub struct MIXERCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub fdwSupport: u32,
    pub cDestinations: u32,
}
impl ::core::marker::Copy for MIXERCAPSA {}
impl ::core::clone::Clone for MIXERCAPSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXERCAPSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXERCAPSA")
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("vDriverVersion", &self.vDriverVersion)
            .field("szPname", &self.szPname)
            .field("fdwSupport", &self.fdwSupport)
            .field("cDestinations", &self.cDestinations)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXERCAPSA {
    fn eq(&self, other: &Self) -> bool {
        self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.vDriverVersion == other.vDriverVersion
            && self.szPname == other.szPname
            && self.fdwSupport == other.fdwSupport
            && self.cDestinations == other.cDestinations
    }
}
impl ::core::cmp::Eq for MIXERCAPSA {}
impl FromIntoMemory for MIXERCAPSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_vDriverVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szPname =
            <[super::super::Foundation::CHAR; 32] as FromIntoMemory>::from_bytes(&from[8..8 + 32]);
        let f_fdwSupport = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_cDestinations = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        Self {
            wMid: f_wMid,
            wPid: f_wPid,
            vDriverVersion: f_vDriverVersion,
            szPname: f_szPname,
            fdwSupport: f_fdwSupport,
            cDestinations: f_cDestinations,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.wMid, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.vDriverVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[8..8 + 32]);
        FromIntoMemory::into_bytes(self.fdwSupport, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.cDestinations, &mut into[44..44 + 4]);
    }
    fn size() -> usize {
        48
    }
}
pub struct MIXERCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub fdwSupport: u32,
    pub cDestinations: u32,
}
impl ::core::marker::Copy for MIXERCAPSW {}
impl ::core::clone::Clone for MIXERCAPSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXERCAPSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXERCAPSW")
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("vDriverVersion", &self.vDriverVersion)
            .field("szPname", &self.szPname)
            .field("fdwSupport", &self.fdwSupport)
            .field("cDestinations", &self.cDestinations)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXERCAPSW {
    fn eq(&self, other: &Self) -> bool {
        self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.vDriverVersion == other.vDriverVersion
            && self.szPname == other.szPname
            && self.fdwSupport == other.fdwSupport
            && self.cDestinations == other.cDestinations
    }
}
impl ::core::cmp::Eq for MIXERCAPSW {}
impl FromIntoMemory for MIXERCAPSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_vDriverVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szPname = <[u16; 32] as FromIntoMemory>::from_bytes(&from[8..8 + 32]);
        let f_fdwSupport = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_cDestinations = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        Self {
            wMid: f_wMid,
            wPid: f_wPid,
            vDriverVersion: f_vDriverVersion,
            szPname: f_szPname,
            fdwSupport: f_fdwSupport,
            cDestinations: f_cDestinations,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.wMid, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.vDriverVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[8..8 + 32]);
        FromIntoMemory::into_bytes(self.fdwSupport, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.cDestinations, &mut into[44..44 + 4]);
    }
    fn size() -> usize {
        48
    }
}
pub struct MIXERCONTROLA {
    pub cbStruct: u32,
    pub dwControlID: u32,
    pub dwControlType: u32,
    pub fdwControl: u32,
    pub cMultipleItems: u32,
    pub szShortName: [super::super::Foundation::CHAR; 16],
    pub szName: [super::super::Foundation::CHAR; 64],
    pub Bounds: MIXERCONTROLA_0,
    pub Metrics: MIXERCONTROLA_1,
}
impl ::core::marker::Copy for MIXERCONTROLA {}
impl ::core::clone::Clone for MIXERCONTROLA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXERCONTROLA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXERCONTROLA")
            .field("cbStruct", &self.cbStruct)
            .field("dwControlID", &self.dwControlID)
            .field("dwControlType", &self.dwControlType)
            .field("fdwControl", &self.fdwControl)
            .field("cMultipleItems", &self.cMultipleItems)
            .field("szShortName", &self.szShortName)
            .field("szName", &self.szName)
            .field("Bounds", &self.Bounds)
            .field("Metrics", &self.Metrics)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXERCONTROLA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwControlID == other.dwControlID
            && self.dwControlType == other.dwControlType
            && self.fdwControl == other.fdwControl
            && self.cMultipleItems == other.cMultipleItems
            && self.szShortName == other.szShortName
            && self.szName == other.szName
            && self.Bounds == other.Bounds
            && self.Metrics == other.Metrics
    }
}
impl ::core::cmp::Eq for MIXERCONTROLA {}
impl FromIntoMemory for MIXERCONTROLA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 148);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwControlID = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwControlType = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_fdwControl = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_cMultipleItems = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_szShortName = <[super::super::Foundation::CHAR; 16] as FromIntoMemory>::from_bytes(
            &from[20..20 + 16],
        );
        let f_szName = <[super::super::Foundation::CHAR; 64] as FromIntoMemory>::from_bytes(
            &from[36..36 + 64],
        );
        let f_Bounds = <MIXERCONTROLA_0 as FromIntoMemory>::from_bytes(&from[100..100 + 24]);
        let f_Metrics = <MIXERCONTROLA_1 as FromIntoMemory>::from_bytes(&from[124..124 + 24]);
        Self {
            cbStruct: f_cbStruct,
            dwControlID: f_dwControlID,
            dwControlType: f_dwControlType,
            fdwControl: f_fdwControl,
            cMultipleItems: f_cMultipleItems,
            szShortName: f_szShortName,
            szName: f_szName,
            Bounds: f_Bounds,
            Metrics: f_Metrics,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 148);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwControlID, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwControlType, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.fdwControl, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.cMultipleItems, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.szShortName, &mut into[20..20 + 16]);
        FromIntoMemory::into_bytes(self.szName, &mut into[36..36 + 64]);
        FromIntoMemory::into_bytes(self.Bounds, &mut into[100..100 + 24]);
        FromIntoMemory::into_bytes(self.Metrics, &mut into[124..124 + 24]);
    }
    fn size() -> usize {
        148
    }
}
pub struct MIXERCONTROLA_0 {
    data: [u8; 24],
}
impl ::core::default::Default for MIXERCONTROLA_0 {
    fn default() -> Self {
        Self { data: [0u8; 24] }
    }
}
impl ::core::marker::Copy for MIXERCONTROLA_0 {}
impl ::core::clone::Clone for MIXERCONTROLA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXERCONTROLA_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXERCONTROLA_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXERCONTROLA_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for MIXERCONTROLA_0 {}
impl FromIntoMemory for MIXERCONTROLA_0 {
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
pub struct MIXERCONTROLA_0_0 {
    pub lMinimum: i32,
    pub lMaximum: i32,
}
impl ::core::marker::Copy for MIXERCONTROLA_0_0 {}
impl ::core::clone::Clone for MIXERCONTROLA_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXERCONTROLA_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXERCONTROLA_0_0")
            .field("lMinimum", &self.lMinimum)
            .field("lMaximum", &self.lMaximum)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXERCONTROLA_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.lMinimum == other.lMinimum && self.lMaximum == other.lMaximum
    }
}
impl ::core::cmp::Eq for MIXERCONTROLA_0_0 {}
impl FromIntoMemory for MIXERCONTROLA_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_lMinimum = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lMaximum = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            lMinimum: f_lMinimum,
            lMaximum: f_lMaximum,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.lMinimum, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lMaximum, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct MIXERCONTROLA_0_1 {
    pub dwMinimum: u32,
    pub dwMaximum: u32,
}
impl ::core::marker::Copy for MIXERCONTROLA_0_1 {}
impl ::core::clone::Clone for MIXERCONTROLA_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXERCONTROLA_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXERCONTROLA_0_1")
            .field("dwMinimum", &self.dwMinimum)
            .field("dwMaximum", &self.dwMaximum)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXERCONTROLA_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwMinimum == other.dwMinimum && self.dwMaximum == other.dwMaximum
    }
}
impl ::core::cmp::Eq for MIXERCONTROLA_0_1 {}
impl FromIntoMemory for MIXERCONTROLA_0_1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_dwMinimum = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwMaximum = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            dwMinimum: f_dwMinimum,
            dwMaximum: f_dwMaximum,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.dwMinimum, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwMaximum, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct MIXERCONTROLA_1 {
    data: [u8; 24],
}
impl ::core::default::Default for MIXERCONTROLA_1 {
    fn default() -> Self {
        Self { data: [0u8; 24] }
    }
}
impl ::core::marker::Copy for MIXERCONTROLA_1 {}
impl ::core::clone::Clone for MIXERCONTROLA_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXERCONTROLA_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXERCONTROLA_1")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXERCONTROLA_1 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for MIXERCONTROLA_1 {}
impl FromIntoMemory for MIXERCONTROLA_1 {
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
pub struct MIXERCONTROLDETAILS {
    pub cbStruct: u32,
    pub dwControlID: u32,
    pub cChannels: u32,
    pub Anonymous: MIXERCONTROLDETAILS_0,
    pub cbDetails: u32,
    pub paDetails: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for MIXERCONTROLDETAILS {}
impl ::core::clone::Clone for MIXERCONTROLDETAILS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXERCONTROLDETAILS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXERCONTROLDETAILS")
            .field("cbStruct", &self.cbStruct)
            .field("dwControlID", &self.dwControlID)
            .field("cChannels", &self.cChannels)
            .field("Anonymous", &self.Anonymous)
            .field("cbDetails", &self.cbDetails)
            .field("paDetails", &self.paDetails)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXERCONTROLDETAILS {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwControlID == other.dwControlID
            && self.cChannels == other.cChannels
            && self.Anonymous == other.Anonymous
            && self.cbDetails == other.cbDetails
            && self.paDetails == other.paDetails
    }
}
impl ::core::cmp::Eq for MIXERCONTROLDETAILS {}
impl FromIntoMemory for MIXERCONTROLDETAILS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwControlID = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_cChannels = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_Anonymous = <MIXERCONTROLDETAILS_0 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_cbDetails = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_paDetails =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            cbStruct: f_cbStruct,
            dwControlID: f_dwControlID,
            cChannels: f_cChannels,
            Anonymous: f_Anonymous,
            cbDetails: f_cbDetails,
            paDetails: f_paDetails,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwControlID, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.cChannels, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.cbDetails, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.paDetails, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct MIXERCONTROLDETAILS_0 {
    data: [u8; 4],
}
impl ::core::default::Default for MIXERCONTROLDETAILS_0 {
    fn default() -> Self {
        Self { data: [0u8; 4] }
    }
}
impl ::core::marker::Copy for MIXERCONTROLDETAILS_0 {}
impl ::core::clone::Clone for MIXERCONTROLDETAILS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXERCONTROLDETAILS_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXERCONTROLDETAILS_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXERCONTROLDETAILS_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for MIXERCONTROLDETAILS_0 {}
impl FromIntoMemory for MIXERCONTROLDETAILS_0 {
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
pub struct MIXERCONTROLDETAILS_BOOLEAN {
    pub fValue: i32,
}
impl ::core::marker::Copy for MIXERCONTROLDETAILS_BOOLEAN {}
impl ::core::clone::Clone for MIXERCONTROLDETAILS_BOOLEAN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXERCONTROLDETAILS_BOOLEAN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXERCONTROLDETAILS_BOOLEAN")
            .field("fValue", &self.fValue)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXERCONTROLDETAILS_BOOLEAN {
    fn eq(&self, other: &Self) -> bool {
        self.fValue == other.fValue
    }
}
impl ::core::cmp::Eq for MIXERCONTROLDETAILS_BOOLEAN {}
impl FromIntoMemory for MIXERCONTROLDETAILS_BOOLEAN {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_fValue = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self { fValue: f_fValue }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.fValue, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct MIXERCONTROLDETAILS_LISTTEXTA {
    pub dwParam1: u32,
    pub dwParam2: u32,
    pub szName: [super::super::Foundation::CHAR; 64],
}
impl ::core::marker::Copy for MIXERCONTROLDETAILS_LISTTEXTA {}
impl ::core::clone::Clone for MIXERCONTROLDETAILS_LISTTEXTA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXERCONTROLDETAILS_LISTTEXTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXERCONTROLDETAILS_LISTTEXTA")
            .field("dwParam1", &self.dwParam1)
            .field("dwParam2", &self.dwParam2)
            .field("szName", &self.szName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXERCONTROLDETAILS_LISTTEXTA {
    fn eq(&self, other: &Self) -> bool {
        self.dwParam1 == other.dwParam1
            && self.dwParam2 == other.dwParam2
            && self.szName == other.szName
    }
}
impl ::core::cmp::Eq for MIXERCONTROLDETAILS_LISTTEXTA {}
impl FromIntoMemory for MIXERCONTROLDETAILS_LISTTEXTA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 72);
        let f_dwParam1 = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwParam2 = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szName =
            <[super::super::Foundation::CHAR; 64] as FromIntoMemory>::from_bytes(&from[8..8 + 64]);
        Self {
            dwParam1: f_dwParam1,
            dwParam2: f_dwParam2,
            szName: f_szName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 72);
        FromIntoMemory::into_bytes(self.dwParam1, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwParam2, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szName, &mut into[8..8 + 64]);
    }
    fn size() -> usize {
        72
    }
}
pub struct MIXERCONTROLDETAILS_LISTTEXTW {
    pub dwParam1: u32,
    pub dwParam2: u32,
    pub szName: [u16; 64],
}
impl ::core::marker::Copy for MIXERCONTROLDETAILS_LISTTEXTW {}
impl ::core::clone::Clone for MIXERCONTROLDETAILS_LISTTEXTW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXERCONTROLDETAILS_LISTTEXTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXERCONTROLDETAILS_LISTTEXTW")
            .field("dwParam1", &self.dwParam1)
            .field("dwParam2", &self.dwParam2)
            .field("szName", &self.szName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXERCONTROLDETAILS_LISTTEXTW {
    fn eq(&self, other: &Self) -> bool {
        self.dwParam1 == other.dwParam1
            && self.dwParam2 == other.dwParam2
            && self.szName == other.szName
    }
}
impl ::core::cmp::Eq for MIXERCONTROLDETAILS_LISTTEXTW {}
impl FromIntoMemory for MIXERCONTROLDETAILS_LISTTEXTW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 72);
        let f_dwParam1 = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwParam2 = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szName = <[u16; 64] as FromIntoMemory>::from_bytes(&from[8..8 + 64]);
        Self {
            dwParam1: f_dwParam1,
            dwParam2: f_dwParam2,
            szName: f_szName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 72);
        FromIntoMemory::into_bytes(self.dwParam1, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwParam2, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szName, &mut into[8..8 + 64]);
    }
    fn size() -> usize {
        72
    }
}
pub struct MIXERCONTROLDETAILS_SIGNED {
    pub lValue: i32,
}
impl ::core::marker::Copy for MIXERCONTROLDETAILS_SIGNED {}
impl ::core::clone::Clone for MIXERCONTROLDETAILS_SIGNED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXERCONTROLDETAILS_SIGNED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXERCONTROLDETAILS_SIGNED")
            .field("lValue", &self.lValue)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXERCONTROLDETAILS_SIGNED {
    fn eq(&self, other: &Self) -> bool {
        self.lValue == other.lValue
    }
}
impl ::core::cmp::Eq for MIXERCONTROLDETAILS_SIGNED {}
impl FromIntoMemory for MIXERCONTROLDETAILS_SIGNED {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_lValue = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self { lValue: f_lValue }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.lValue, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct MIXERCONTROLDETAILS_UNSIGNED {
    pub dwValue: u32,
}
impl ::core::marker::Copy for MIXERCONTROLDETAILS_UNSIGNED {}
impl ::core::clone::Clone for MIXERCONTROLDETAILS_UNSIGNED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXERCONTROLDETAILS_UNSIGNED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXERCONTROLDETAILS_UNSIGNED")
            .field("dwValue", &self.dwValue)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXERCONTROLDETAILS_UNSIGNED {
    fn eq(&self, other: &Self) -> bool {
        self.dwValue == other.dwValue
    }
}
impl ::core::cmp::Eq for MIXERCONTROLDETAILS_UNSIGNED {}
impl FromIntoMemory for MIXERCONTROLDETAILS_UNSIGNED {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_dwValue = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self { dwValue: f_dwValue }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.dwValue, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct MIXERCONTROLW {
    pub cbStruct: u32,
    pub dwControlID: u32,
    pub dwControlType: u32,
    pub fdwControl: u32,
    pub cMultipleItems: u32,
    pub szShortName: [u16; 16],
    pub szName: [u16; 64],
    pub Bounds: MIXERCONTROLW_0,
    pub Metrics: MIXERCONTROLW_1,
}
impl ::core::marker::Copy for MIXERCONTROLW {}
impl ::core::clone::Clone for MIXERCONTROLW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXERCONTROLW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXERCONTROLW")
            .field("cbStruct", &self.cbStruct)
            .field("dwControlID", &self.dwControlID)
            .field("dwControlType", &self.dwControlType)
            .field("fdwControl", &self.fdwControl)
            .field("cMultipleItems", &self.cMultipleItems)
            .field("szShortName", &self.szShortName)
            .field("szName", &self.szName)
            .field("Bounds", &self.Bounds)
            .field("Metrics", &self.Metrics)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXERCONTROLW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwControlID == other.dwControlID
            && self.dwControlType == other.dwControlType
            && self.fdwControl == other.fdwControl
            && self.cMultipleItems == other.cMultipleItems
            && self.szShortName == other.szShortName
            && self.szName == other.szName
            && self.Bounds == other.Bounds
            && self.Metrics == other.Metrics
    }
}
impl ::core::cmp::Eq for MIXERCONTROLW {}
impl FromIntoMemory for MIXERCONTROLW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 148);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwControlID = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwControlType = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_fdwControl = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_cMultipleItems = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_szShortName = <[u16; 16] as FromIntoMemory>::from_bytes(&from[20..20 + 16]);
        let f_szName = <[u16; 64] as FromIntoMemory>::from_bytes(&from[36..36 + 64]);
        let f_Bounds = <MIXERCONTROLW_0 as FromIntoMemory>::from_bytes(&from[100..100 + 24]);
        let f_Metrics = <MIXERCONTROLW_1 as FromIntoMemory>::from_bytes(&from[124..124 + 24]);
        Self {
            cbStruct: f_cbStruct,
            dwControlID: f_dwControlID,
            dwControlType: f_dwControlType,
            fdwControl: f_fdwControl,
            cMultipleItems: f_cMultipleItems,
            szShortName: f_szShortName,
            szName: f_szName,
            Bounds: f_Bounds,
            Metrics: f_Metrics,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 148);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwControlID, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwControlType, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.fdwControl, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.cMultipleItems, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.szShortName, &mut into[20..20 + 16]);
        FromIntoMemory::into_bytes(self.szName, &mut into[36..36 + 64]);
        FromIntoMemory::into_bytes(self.Bounds, &mut into[100..100 + 24]);
        FromIntoMemory::into_bytes(self.Metrics, &mut into[124..124 + 24]);
    }
    fn size() -> usize {
        148
    }
}
pub struct MIXERCONTROLW_0 {
    data: [u8; 24],
}
impl ::core::default::Default for MIXERCONTROLW_0 {
    fn default() -> Self {
        Self { data: [0u8; 24] }
    }
}
impl ::core::marker::Copy for MIXERCONTROLW_0 {}
impl ::core::clone::Clone for MIXERCONTROLW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXERCONTROLW_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXERCONTROLW_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXERCONTROLW_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for MIXERCONTROLW_0 {}
impl FromIntoMemory for MIXERCONTROLW_0 {
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
pub struct MIXERCONTROLW_0_0 {
    pub lMinimum: i32,
    pub lMaximum: i32,
}
impl ::core::marker::Copy for MIXERCONTROLW_0_0 {}
impl ::core::clone::Clone for MIXERCONTROLW_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXERCONTROLW_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXERCONTROLW_0_0")
            .field("lMinimum", &self.lMinimum)
            .field("lMaximum", &self.lMaximum)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXERCONTROLW_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.lMinimum == other.lMinimum && self.lMaximum == other.lMaximum
    }
}
impl ::core::cmp::Eq for MIXERCONTROLW_0_0 {}
impl FromIntoMemory for MIXERCONTROLW_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_lMinimum = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lMaximum = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            lMinimum: f_lMinimum,
            lMaximum: f_lMaximum,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.lMinimum, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lMaximum, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct MIXERCONTROLW_0_1 {
    pub dwMinimum: u32,
    pub dwMaximum: u32,
}
impl ::core::marker::Copy for MIXERCONTROLW_0_1 {}
impl ::core::clone::Clone for MIXERCONTROLW_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXERCONTROLW_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXERCONTROLW_0_1")
            .field("dwMinimum", &self.dwMinimum)
            .field("dwMaximum", &self.dwMaximum)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXERCONTROLW_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwMinimum == other.dwMinimum && self.dwMaximum == other.dwMaximum
    }
}
impl ::core::cmp::Eq for MIXERCONTROLW_0_1 {}
impl FromIntoMemory for MIXERCONTROLW_0_1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_dwMinimum = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwMaximum = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            dwMinimum: f_dwMinimum,
            dwMaximum: f_dwMaximum,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.dwMinimum, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwMaximum, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct MIXERCONTROLW_1 {
    data: [u8; 24],
}
impl ::core::default::Default for MIXERCONTROLW_1 {
    fn default() -> Self {
        Self { data: [0u8; 24] }
    }
}
impl ::core::marker::Copy for MIXERCONTROLW_1 {}
impl ::core::clone::Clone for MIXERCONTROLW_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXERCONTROLW_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXERCONTROLW_1")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXERCONTROLW_1 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for MIXERCONTROLW_1 {}
impl FromIntoMemory for MIXERCONTROLW_1 {
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
pub const MIXERCONTROL_CONTROLF_DISABLED: i32 = -2147483648i32;
pub const MIXERCONTROL_CONTROLF_MULTIPLE: i32 = 2i32;
pub const MIXERCONTROL_CONTROLF_UNIFORM: i32 = 1i32;
pub const MIXERCONTROL_CONTROLTYPE_BASS: u32 = 1342373890u32;
pub const MIXERCONTROL_CONTROLTYPE_BASS_BOOST: u32 = 536945271u32;
pub const MIXERCONTROL_CONTROLTYPE_BOOLEAN: u32 = 536936448u32;
pub const MIXERCONTROL_CONTROLTYPE_BOOLEANMETER: u32 = 268500992u32;
pub const MIXERCONTROL_CONTROLTYPE_BUTTON: u32 = 553713664u32;
pub const MIXERCONTROL_CONTROLTYPE_CUSTOM: u32 = 0u32;
pub const MIXERCONTROL_CONTROLTYPE_DECIBELS: u32 = 805568512u32;
pub const MIXERCONTROL_CONTROLTYPE_EQUALIZER: u32 = 1342373892u32;
pub const MIXERCONTROL_CONTROLTYPE_FADER: u32 = 1342373888u32;
pub const MIXERCONTROL_CONTROLTYPE_LOUDNESS: u32 = 536936452u32;
pub const MIXERCONTROL_CONTROLTYPE_MICROTIME: u32 = 1610809344u32;
pub const MIXERCONTROL_CONTROLTYPE_MILLITIME: u32 = 1627586560u32;
pub const MIXERCONTROL_CONTROLTYPE_MIXER: u32 = 1895890945u32;
pub const MIXERCONTROL_CONTROLTYPE_MONO: u32 = 536936451u32;
pub const MIXERCONTROL_CONTROLTYPE_MULTIPLESELECT: u32 = 1895890944u32;
pub const MIXERCONTROL_CONTROLTYPE_MUTE: u32 = 536936450u32;
pub const MIXERCONTROL_CONTROLTYPE_MUX: u32 = 1879113729u32;
pub const MIXERCONTROL_CONTROLTYPE_ONOFF: u32 = 536936449u32;
pub const MIXERCONTROL_CONTROLTYPE_PAN: u32 = 1073872897u32;
pub const MIXERCONTROL_CONTROLTYPE_PEAKMETER: u32 = 268566529u32;
pub const MIXERCONTROL_CONTROLTYPE_PERCENT: u32 = 805634048u32;
pub const MIXERCONTROL_CONTROLTYPE_QSOUNDPAN: u32 = 1073872898u32;
pub const MIXERCONTROL_CONTROLTYPE_SIGNED: u32 = 805437440u32;
pub const MIXERCONTROL_CONTROLTYPE_SIGNEDMETER: u32 = 268566528u32;
pub const MIXERCONTROL_CONTROLTYPE_SINGLESELECT: u32 = 1879113728u32;
pub const MIXERCONTROL_CONTROLTYPE_SLIDER: u32 = 1073872896u32;
pub const MIXERCONTROL_CONTROLTYPE_STEREOENH: u32 = 536936453u32;
pub const MIXERCONTROL_CONTROLTYPE_TREBLE: u32 = 1342373891u32;
pub const MIXERCONTROL_CONTROLTYPE_UNSIGNED: u32 = 805502976u32;
pub const MIXERCONTROL_CONTROLTYPE_UNSIGNEDMETER: u32 = 268632064u32;
pub const MIXERCONTROL_CONTROLTYPE_VOLUME: u32 = 1342373889u32;
pub const MIXERCONTROL_CT_CLASS_CUSTOM: i32 = 0i32;
pub const MIXERCONTROL_CT_CLASS_FADER: i32 = 1342177280i32;
pub const MIXERCONTROL_CT_CLASS_LIST: i32 = 1879048192i32;
pub const MIXERCONTROL_CT_CLASS_MASK: i32 = -268435456i32;
pub const MIXERCONTROL_CT_CLASS_METER: i32 = 268435456i32;
pub const MIXERCONTROL_CT_CLASS_NUMBER: i32 = 805306368i32;
pub const MIXERCONTROL_CT_CLASS_SLIDER: i32 = 1073741824i32;
pub const MIXERCONTROL_CT_CLASS_SWITCH: i32 = 536870912i32;
pub const MIXERCONTROL_CT_CLASS_TIME: i32 = 1610612736i32;
pub const MIXERCONTROL_CT_SC_LIST_MULTIPLE: i32 = 16777216i32;
pub const MIXERCONTROL_CT_SC_LIST_SINGLE: i32 = 0i32;
pub const MIXERCONTROL_CT_SC_METER_POLLED: i32 = 0i32;
pub const MIXERCONTROL_CT_SC_SWITCH_BOOLEAN: i32 = 0i32;
pub const MIXERCONTROL_CT_SC_SWITCH_BUTTON: i32 = 16777216i32;
pub const MIXERCONTROL_CT_SC_TIME_MICROSECS: i32 = 0i32;
pub const MIXERCONTROL_CT_SC_TIME_MILLISECS: i32 = 16777216i32;
pub const MIXERCONTROL_CT_SUBCLASS_MASK: i32 = 251658240i32;
pub const MIXERCONTROL_CT_UNITS_BOOLEAN: i32 = 65536i32;
pub const MIXERCONTROL_CT_UNITS_CUSTOM: i32 = 0i32;
pub const MIXERCONTROL_CT_UNITS_DECIBELS: i32 = 262144i32;
pub const MIXERCONTROL_CT_UNITS_MASK: i32 = 16711680i32;
pub const MIXERCONTROL_CT_UNITS_PERCENT: i32 = 327680i32;
pub const MIXERCONTROL_CT_UNITS_SIGNED: i32 = 131072i32;
pub const MIXERCONTROL_CT_UNITS_UNSIGNED: i32 = 196608i32;
pub struct MIXERLINEA {
    pub cbStruct: u32,
    pub dwDestination: u32,
    pub dwSource: u32,
    pub dwLineID: u32,
    pub fdwLine: u32,
    pub dwUser: PtrRepr,
    pub dwComponentType: MIXERLINE_COMPONENTTYPE,
    pub cChannels: u32,
    pub cConnections: u32,
    pub cControls: u32,
    pub szShortName: [super::super::Foundation::CHAR; 16],
    pub szName: [super::super::Foundation::CHAR; 64],
    pub Target: MIXERLINEA_0,
}
impl ::core::marker::Copy for MIXERLINEA {}
impl ::core::clone::Clone for MIXERLINEA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXERLINEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXERLINEA")
            .field("cbStruct", &self.cbStruct)
            .field("dwDestination", &self.dwDestination)
            .field("dwSource", &self.dwSource)
            .field("dwLineID", &self.dwLineID)
            .field("fdwLine", &self.fdwLine)
            .field("dwUser", &self.dwUser)
            .field("dwComponentType", &self.dwComponentType)
            .field("cChannels", &self.cChannels)
            .field("cConnections", &self.cConnections)
            .field("cControls", &self.cControls)
            .field("szShortName", &self.szShortName)
            .field("szName", &self.szName)
            .field("Target", &self.Target)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXERLINEA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwDestination == other.dwDestination
            && self.dwSource == other.dwSource
            && self.dwLineID == other.dwLineID
            && self.fdwLine == other.fdwLine
            && self.dwUser == other.dwUser
            && self.dwComponentType == other.dwComponentType
            && self.cChannels == other.cChannels
            && self.cConnections == other.cConnections
            && self.cControls == other.cControls
            && self.szShortName == other.szShortName
            && self.szName == other.szName
            && self.Target == other.Target
    }
}
impl ::core::cmp::Eq for MIXERLINEA {}
impl FromIntoMemory for MIXERLINEA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 168);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwDestination = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwSource = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwLineID = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_fdwLine = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwUser = <PtrRepr as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwComponentType =
            <MIXERLINE_COMPONENTTYPE as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_cChannels = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_cConnections = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_cControls = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_szShortName = <[super::super::Foundation::CHAR; 16] as FromIntoMemory>::from_bytes(
            &from[40..40 + 16],
        );
        let f_szName = <[super::super::Foundation::CHAR; 64] as FromIntoMemory>::from_bytes(
            &from[56..56 + 64],
        );
        let f_Target = <MIXERLINEA_0 as FromIntoMemory>::from_bytes(&from[120..120 + 48]);
        Self {
            cbStruct: f_cbStruct,
            dwDestination: f_dwDestination,
            dwSource: f_dwSource,
            dwLineID: f_dwLineID,
            fdwLine: f_fdwLine,
            dwUser: f_dwUser,
            dwComponentType: f_dwComponentType,
            cChannels: f_cChannels,
            cConnections: f_cConnections,
            cControls: f_cControls,
            szShortName: f_szShortName,
            szName: f_szName,
            Target: f_Target,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 168);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwDestination, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwSource, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwLineID, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.fdwLine, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwUser, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwComponentType, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.cChannels, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.cConnections, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.cControls, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.szShortName, &mut into[40..40 + 16]);
        FromIntoMemory::into_bytes(self.szName, &mut into[56..56 + 64]);
        FromIntoMemory::into_bytes(self.Target, &mut into[120..120 + 48]);
    }
    fn size() -> usize {
        168
    }
}
pub struct MIXERLINEA_0 {
    pub dwType: u32,
    pub dwDeviceID: u32,
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
}
impl ::core::marker::Copy for MIXERLINEA_0 {}
impl ::core::clone::Clone for MIXERLINEA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXERLINEA_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXERLINEA_0")
            .field("dwType", &self.dwType)
            .field("dwDeviceID", &self.dwDeviceID)
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("vDriverVersion", &self.vDriverVersion)
            .field("szPname", &self.szPname)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXERLINEA_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType
            && self.dwDeviceID == other.dwDeviceID
            && self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.vDriverVersion == other.vDriverVersion
            && self.szPname == other.szPname
    }
}
impl ::core::cmp::Eq for MIXERLINEA_0 {}
impl FromIntoMemory for MIXERLINEA_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_dwType = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwDeviceID = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[8..8 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[10..10 + 2]);
        let f_vDriverVersion = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_szPname = <[super::super::Foundation::CHAR; 32] as FromIntoMemory>::from_bytes(
            &from[16..16 + 32],
        );
        Self {
            dwType: f_dwType,
            dwDeviceID: f_dwDeviceID,
            wMid: f_wMid,
            wPid: f_wPid,
            vDriverVersion: f_vDriverVersion,
            szPname: f_szPname,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.dwType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwDeviceID, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.wMid, &mut into[8..8 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[10..10 + 2]);
        FromIntoMemory::into_bytes(self.vDriverVersion, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[16..16 + 32]);
    }
    fn size() -> usize {
        48
    }
}
pub struct MIXERLINECONTROLSA {
    pub cbStruct: u32,
    pub dwLineID: u32,
    pub Anonymous: MIXERLINECONTROLSA_0,
    pub cControls: u32,
    pub cbmxctrl: u32,
    pub pamxctrl: MutPtr<MIXERCONTROLA>,
}
impl ::core::marker::Copy for MIXERLINECONTROLSA {}
impl ::core::clone::Clone for MIXERLINECONTROLSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXERLINECONTROLSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXERLINECONTROLSA")
            .field("cbStruct", &self.cbStruct)
            .field("dwLineID", &self.dwLineID)
            .field("Anonymous", &self.Anonymous)
            .field("cControls", &self.cControls)
            .field("cbmxctrl", &self.cbmxctrl)
            .field("pamxctrl", &self.pamxctrl)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXERLINECONTROLSA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwLineID == other.dwLineID
            && self.Anonymous == other.Anonymous
            && self.cControls == other.cControls
            && self.cbmxctrl == other.cbmxctrl
            && self.pamxctrl == other.pamxctrl
    }
}
impl ::core::cmp::Eq for MIXERLINECONTROLSA {}
impl FromIntoMemory for MIXERLINECONTROLSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwLineID = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Anonymous = <MIXERLINECONTROLSA_0 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_cControls = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_cbmxctrl = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_pamxctrl = <MutPtr<MIXERCONTROLA> as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            cbStruct: f_cbStruct,
            dwLineID: f_dwLineID,
            Anonymous: f_Anonymous,
            cControls: f_cControls,
            cbmxctrl: f_cbmxctrl,
            pamxctrl: f_pamxctrl,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwLineID, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.cControls, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.cbmxctrl, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.pamxctrl, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct MIXERLINECONTROLSA_0 {
    data: [u8; 4],
}
impl ::core::default::Default for MIXERLINECONTROLSA_0 {
    fn default() -> Self {
        Self { data: [0u8; 4] }
    }
}
impl ::core::marker::Copy for MIXERLINECONTROLSA_0 {}
impl ::core::clone::Clone for MIXERLINECONTROLSA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXERLINECONTROLSA_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXERLINECONTROLSA_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXERLINECONTROLSA_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for MIXERLINECONTROLSA_0 {}
impl FromIntoMemory for MIXERLINECONTROLSA_0 {
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
pub struct MIXERLINECONTROLSW {
    pub cbStruct: u32,
    pub dwLineID: u32,
    pub Anonymous: MIXERLINECONTROLSW_0,
    pub cControls: u32,
    pub cbmxctrl: u32,
    pub pamxctrl: MutPtr<MIXERCONTROLW>,
}
impl ::core::marker::Copy for MIXERLINECONTROLSW {}
impl ::core::clone::Clone for MIXERLINECONTROLSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXERLINECONTROLSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXERLINECONTROLSW")
            .field("cbStruct", &self.cbStruct)
            .field("dwLineID", &self.dwLineID)
            .field("Anonymous", &self.Anonymous)
            .field("cControls", &self.cControls)
            .field("cbmxctrl", &self.cbmxctrl)
            .field("pamxctrl", &self.pamxctrl)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXERLINECONTROLSW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwLineID == other.dwLineID
            && self.Anonymous == other.Anonymous
            && self.cControls == other.cControls
            && self.cbmxctrl == other.cbmxctrl
            && self.pamxctrl == other.pamxctrl
    }
}
impl ::core::cmp::Eq for MIXERLINECONTROLSW {}
impl FromIntoMemory for MIXERLINECONTROLSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwLineID = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Anonymous = <MIXERLINECONTROLSW_0 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_cControls = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_cbmxctrl = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_pamxctrl = <MutPtr<MIXERCONTROLW> as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            cbStruct: f_cbStruct,
            dwLineID: f_dwLineID,
            Anonymous: f_Anonymous,
            cControls: f_cControls,
            cbmxctrl: f_cbmxctrl,
            pamxctrl: f_pamxctrl,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwLineID, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.cControls, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.cbmxctrl, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.pamxctrl, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct MIXERLINECONTROLSW_0 {
    data: [u8; 4],
}
impl ::core::default::Default for MIXERLINECONTROLSW_0 {
    fn default() -> Self {
        Self { data: [0u8; 4] }
    }
}
impl ::core::marker::Copy for MIXERLINECONTROLSW_0 {}
impl ::core::clone::Clone for MIXERLINECONTROLSW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXERLINECONTROLSW_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXERLINECONTROLSW_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXERLINECONTROLSW_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for MIXERLINECONTROLSW_0 {}
impl FromIntoMemory for MIXERLINECONTROLSW_0 {
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
pub struct MIXERLINEW {
    pub cbStruct: u32,
    pub dwDestination: u32,
    pub dwSource: u32,
    pub dwLineID: u32,
    pub fdwLine: u32,
    pub dwUser: PtrRepr,
    pub dwComponentType: MIXERLINE_COMPONENTTYPE,
    pub cChannels: u32,
    pub cConnections: u32,
    pub cControls: u32,
    pub szShortName: [u16; 16],
    pub szName: [u16; 64],
    pub Target: MIXERLINEW_0,
}
impl ::core::marker::Copy for MIXERLINEW {}
impl ::core::clone::Clone for MIXERLINEW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXERLINEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXERLINEW")
            .field("cbStruct", &self.cbStruct)
            .field("dwDestination", &self.dwDestination)
            .field("dwSource", &self.dwSource)
            .field("dwLineID", &self.dwLineID)
            .field("fdwLine", &self.fdwLine)
            .field("dwUser", &self.dwUser)
            .field("dwComponentType", &self.dwComponentType)
            .field("cChannels", &self.cChannels)
            .field("cConnections", &self.cConnections)
            .field("cControls", &self.cControls)
            .field("szShortName", &self.szShortName)
            .field("szName", &self.szName)
            .field("Target", &self.Target)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXERLINEW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwDestination == other.dwDestination
            && self.dwSource == other.dwSource
            && self.dwLineID == other.dwLineID
            && self.fdwLine == other.fdwLine
            && self.dwUser == other.dwUser
            && self.dwComponentType == other.dwComponentType
            && self.cChannels == other.cChannels
            && self.cConnections == other.cConnections
            && self.cControls == other.cControls
            && self.szShortName == other.szShortName
            && self.szName == other.szName
            && self.Target == other.Target
    }
}
impl ::core::cmp::Eq for MIXERLINEW {}
impl FromIntoMemory for MIXERLINEW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 168);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwDestination = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwSource = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwLineID = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_fdwLine = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwUser = <PtrRepr as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwComponentType =
            <MIXERLINE_COMPONENTTYPE as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_cChannels = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_cConnections = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_cControls = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_szShortName = <[u16; 16] as FromIntoMemory>::from_bytes(&from[40..40 + 16]);
        let f_szName = <[u16; 64] as FromIntoMemory>::from_bytes(&from[56..56 + 64]);
        let f_Target = <MIXERLINEW_0 as FromIntoMemory>::from_bytes(&from[120..120 + 48]);
        Self {
            cbStruct: f_cbStruct,
            dwDestination: f_dwDestination,
            dwSource: f_dwSource,
            dwLineID: f_dwLineID,
            fdwLine: f_fdwLine,
            dwUser: f_dwUser,
            dwComponentType: f_dwComponentType,
            cChannels: f_cChannels,
            cConnections: f_cConnections,
            cControls: f_cControls,
            szShortName: f_szShortName,
            szName: f_szName,
            Target: f_Target,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 168);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwDestination, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwSource, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwLineID, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.fdwLine, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwUser, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwComponentType, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.cChannels, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.cConnections, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.cControls, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.szShortName, &mut into[40..40 + 16]);
        FromIntoMemory::into_bytes(self.szName, &mut into[56..56 + 64]);
        FromIntoMemory::into_bytes(self.Target, &mut into[120..120 + 48]);
    }
    fn size() -> usize {
        168
    }
}
pub struct MIXERLINEW_0 {
    pub dwType: u32,
    pub dwDeviceID: u32,
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
}
impl ::core::marker::Copy for MIXERLINEW_0 {}
impl ::core::clone::Clone for MIXERLINEW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIXERLINEW_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIXERLINEW_0")
            .field("dwType", &self.dwType)
            .field("dwDeviceID", &self.dwDeviceID)
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("vDriverVersion", &self.vDriverVersion)
            .field("szPname", &self.szPname)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIXERLINEW_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType
            && self.dwDeviceID == other.dwDeviceID
            && self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.vDriverVersion == other.vDriverVersion
            && self.szPname == other.szPname
    }
}
impl ::core::cmp::Eq for MIXERLINEW_0 {}
impl FromIntoMemory for MIXERLINEW_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_dwType = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwDeviceID = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[8..8 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[10..10 + 2]);
        let f_vDriverVersion = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_szPname = <[u16; 32] as FromIntoMemory>::from_bytes(&from[16..16 + 32]);
        Self {
            dwType: f_dwType,
            dwDeviceID: f_dwDeviceID,
            wMid: f_wMid,
            wPid: f_wPid,
            vDriverVersion: f_vDriverVersion,
            szPname: f_szPname,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.dwType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwDeviceID, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.wMid, &mut into[8..8 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[10..10 + 2]);
        FromIntoMemory::into_bytes(self.vDriverVersion, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[16..16 + 32]);
    }
    fn size() -> usize {
        48
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MIXERLINE_COMPONENTTYPE(pub u32);
pub const MIXERLINE_COMPONENTTYPE_DST_DIGITAL: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(1u32);
pub const MIXERLINE_COMPONENTTYPE_DST_HEADPHONES: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(5u32);
pub const MIXERLINE_COMPONENTTYPE_DST_LINE: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(2u32);
pub const MIXERLINE_COMPONENTTYPE_DST_MONITOR: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(3u32);
pub const MIXERLINE_COMPONENTTYPE_DST_SPEAKERS: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(4u32);
pub const MIXERLINE_COMPONENTTYPE_DST_TELEPHONE: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(6u32);
pub const MIXERLINE_COMPONENTTYPE_DST_UNDEFINED: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(0u32);
pub const MIXERLINE_COMPONENTTYPE_DST_VOICEIN: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(8u32);
pub const MIXERLINE_COMPONENTTYPE_DST_WAVEIN: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(7u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_ANALOG: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(4106u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_AUXILIARY: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(4105u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_COMPACTDISC: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(4101u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_DIGITAL: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(4097u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_LINE: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(4098u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_MICROPHONE: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(4099u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_PCSPEAKER: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(4103u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_SYNTHESIZER: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(4100u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_TELEPHONE: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(4102u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_UNDEFINED: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(4096u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_WAVEOUT: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(4104u32);
impl ::core::marker::Copy for MIXERLINE_COMPONENTTYPE {}
impl ::core::clone::Clone for MIXERLINE_COMPONENTTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MIXERLINE_COMPONENTTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MIXERLINE_COMPONENTTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIXERLINE_COMPONENTTYPE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for MIXERLINE_COMPONENTTYPE {
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
pub const MIXERLINE_COMPONENTTYPE_DST_FIRST: i32 = 0i32;
pub const MIXERLINE_COMPONENTTYPE_DST_LAST: u32 = 8u32;
pub const MIXERLINE_COMPONENTTYPE_SRC_FIRST: i32 = 4096i32;
pub const MIXERLINE_COMPONENTTYPE_SRC_LAST: u32 = 4106u32;
pub const MIXERLINE_LINEF_ACTIVE: i32 = 1i32;
pub const MIXERLINE_LINEF_DISCONNECTED: i32 = 32768i32;
pub const MIXERLINE_LINEF_SOURCE: i32 = -2147483648i32;
pub const MIXERLINE_TARGETTYPE_AUX: u32 = 5u32;
pub const MIXERLINE_TARGETTYPE_MIDIIN: u32 = 4u32;
pub const MIXERLINE_TARGETTYPE_MIDIOUT: u32 = 3u32;
pub const MIXERLINE_TARGETTYPE_UNDEFINED: u32 = 0u32;
pub const MIXERLINE_TARGETTYPE_WAVEIN: u32 = 2u32;
pub const MIXERLINE_TARGETTYPE_WAVEOUT: u32 = 1u32;
pub const MIXERR_INVALCONTROL: u32 = 1025u32;
pub const MIXERR_INVALLINE: u32 = 1024u32;
pub const MIXERR_INVALVALUE: u32 = 1026u32;
pub const MIXERR_LASTERROR: u32 = 1026u32;
pub const MIXER_GETCONTROLDETAILSF_LISTTEXT: i32 = 1i32;
pub const MIXER_GETCONTROLDETAILSF_QUERYMASK: i32 = 15i32;
pub const MIXER_GETCONTROLDETAILSF_VALUE: i32 = 0i32;
pub const MIXER_GETLINECONTROLSF_ALL: i32 = 0i32;
pub const MIXER_GETLINECONTROLSF_ONEBYID: i32 = 1i32;
pub const MIXER_GETLINECONTROLSF_ONEBYTYPE: i32 = 2i32;
pub const MIXER_GETLINECONTROLSF_QUERYMASK: i32 = 15i32;
pub const MIXER_GETLINEINFOF_COMPONENTTYPE: i32 = 3i32;
pub const MIXER_GETLINEINFOF_DESTINATION: i32 = 0i32;
pub const MIXER_GETLINEINFOF_LINEID: i32 = 2i32;
pub const MIXER_GETLINEINFOF_QUERYMASK: i32 = 15i32;
pub const MIXER_GETLINEINFOF_SOURCE: i32 = 1i32;
pub const MIXER_GETLINEINFOF_TARGETTYPE: i32 = 4i32;
pub const MIXER_LONG_NAME_CHARS: u32 = 64u32;
pub const MIXER_OBJECTF_AUX: i32 = 1342177280i32;
pub const MIXER_OBJECTF_HANDLE: i32 = -2147483648i32;
pub const MIXER_OBJECTF_MIDIIN: i32 = 1073741824i32;
pub const MIXER_OBJECTF_MIDIOUT: i32 = 805306368i32;
pub const MIXER_OBJECTF_MIXER: i32 = 0i32;
pub const MIXER_OBJECTF_WAVEIN: i32 = 536870912i32;
pub const MIXER_OBJECTF_WAVEOUT: i32 = 268435456i32;
pub const MIXER_SETCONTROLDETAILSF_CUSTOM: i32 = 1i32;
pub const MIXER_SETCONTROLDETAILSF_QUERYMASK: i32 = 15i32;
pub const MIXER_SETCONTROLDETAILSF_VALUE: i32 = 0i32;
pub const MIXER_SHORT_NAME_CHARS: u32 = 16u32;
pub const MMDeviceEnumerator: crate::core::GUID =
    crate::core::GUID::from_u128(0xbcde0395_e52f_467c_8e3d_c4579291692e);
pub const MM_ACM_FILTERCHOOSE: u32 = 32768u32;
pub const MM_ACM_FORMATCHOOSE: u32 = 32768u32;
pub const MOD_FMSYNTH: u32 = 4u32;
pub const MOD_MAPPER: u32 = 5u32;
pub const MOD_MIDIPORT: u32 = 1u32;
pub const MOD_SQSYNTH: u32 = 3u32;
pub const MOD_SWSYNTH: u32 = 7u32;
pub const MOD_SYNTH: u32 = 2u32;
pub const MOD_WAVETABLE: u32 = 6u32;
pub type PAudioStateMonitorCallback =
    StdCallFnPtr<(IAudioStateMonitor, ConstPtr<::core::ffi::c_void>), ()>;
pub struct PCMWAVEFORMAT {
    pub wf: WAVEFORMAT,
    pub wBitsPerSample: u16,
}
impl ::core::marker::Copy for PCMWAVEFORMAT {}
impl ::core::clone::Clone for PCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PCMWAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PCMWAVEFORMAT")
            .field("wf", &self.wf)
            .field("wBitsPerSample", &self.wBitsPerSample)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wf == other.wf && self.wBitsPerSample == other.wBitsPerSample
    }
}
impl ::core::cmp::Eq for PCMWAVEFORMAT {}
impl FromIntoMemory for PCMWAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_wf = <WAVEFORMAT as FromIntoMemory>::from_bytes(&from[0..0 + 14]);
        let f_wBitsPerSample = <u16 as FromIntoMemory>::from_bytes(&from[14..14 + 2]);
        Self {
            wf: f_wf,
            wBitsPerSample: f_wBitsPerSample,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.wf, &mut into[0..0 + 14]);
        FromIntoMemory::into_bytes(self.wBitsPerSample, &mut into[14..14 + 2]);
    }
    fn size() -> usize {
        16
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub const PKEY_AudioEndpointLogo_IconEffects:
    super::super::UI::Shell::PropertiesSystem::PROPERTYKEY =
    super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
        fmtid: crate::core::GUID::from_u128(0xf1ab780d_2010_4ed3_a3a6_8b87f0f0c476),
        pid: 0u32,
    };
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub const PKEY_AudioEndpointLogo_IconPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY =
    super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
        fmtid: crate::core::GUID::from_u128(0xf1ab780d_2010_4ed3_a3a6_8b87f0f0c476),
        pid: 1u32,
    };
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub const PKEY_AudioEndpointSettings_LaunchContract:
    super::super::UI::Shell::PropertiesSystem::PROPERTYKEY =
    super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
        fmtid: crate::core::GUID::from_u128(0x14242002_0320_4de4_9555_a7d82b73c286),
        pid: 1u32,
    };
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub const PKEY_AudioEndpointSettings_MenuText:
    super::super::UI::Shell::PropertiesSystem::PROPERTYKEY =
    super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
        fmtid: crate::core::GUID::from_u128(0x14242002_0320_4de4_9555_a7d82b73c286),
        pid: 0u32,
    };
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub const PKEY_AudioEndpoint_Association: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY =
    super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
        fmtid: crate::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e),
        pid: 2u32,
    };
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub const PKEY_AudioEndpoint_ControlPanelPageProvider:
    super::super::UI::Shell::PropertiesSystem::PROPERTYKEY =
    super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
        fmtid: crate::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e),
        pid: 1u32,
    };
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub const PKEY_AudioEndpoint_Default_VolumeInDb:
    super::super::UI::Shell::PropertiesSystem::PROPERTYKEY =
    super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
        fmtid: crate::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e),
        pid: 9u32,
    };
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub const PKEY_AudioEndpoint_Disable_SysFx: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY =
    super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
        fmtid: crate::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e),
        pid: 5u32,
    };
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub const PKEY_AudioEndpoint_FormFactor: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY =
    super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
        fmtid: crate::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e),
        pid: 0u32,
    };
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub const PKEY_AudioEndpoint_FullRangeSpeakers:
    super::super::UI::Shell::PropertiesSystem::PROPERTYKEY =
    super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
        fmtid: crate::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e),
        pid: 6u32,
    };
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub const PKEY_AudioEndpoint_GUID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY =
    super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
        fmtid: crate::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e),
        pid: 4u32,
    };
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub const PKEY_AudioEndpoint_JackSubType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY =
    super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
        fmtid: crate::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e),
        pid: 8u32,
    };
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub const PKEY_AudioEndpoint_PhysicalSpeakers:
    super::super::UI::Shell::PropertiesSystem::PROPERTYKEY =
    super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
        fmtid: crate::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e),
        pid: 3u32,
    };
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub const PKEY_AudioEndpoint_Supports_EventDriven_Mode:
    super::super::UI::Shell::PropertiesSystem::PROPERTYKEY =
    super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
        fmtid: crate::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e),
        pid: 7u32,
    };
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub const PKEY_AudioEngine_DeviceFormat: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY =
    super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
        fmtid: crate::core::GUID::from_u128(0xf19f064d_082c_4e27_bc73_6882a1bb8e4c),
        pid: 0u32,
    };
#[doc = "*Required namespaces: 'Windows.Win32.UI.Shell.PropertiesSystem'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub const PKEY_AudioEngine_OEMFormat: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY =
    super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
        fmtid: crate::core::GUID::from_u128(0xe4870e26_3cc5_4cd2_ba46_ca0a9a70ed04),
        pid: 3u32,
    };
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROCESS_LOOPBACK_MODE(pub i32);
pub const PROCESS_LOOPBACK_MODE_INCLUDE_TARGET_PROCESS_TREE: PROCESS_LOOPBACK_MODE =
    PROCESS_LOOPBACK_MODE(0i32);
pub const PROCESS_LOOPBACK_MODE_EXCLUDE_TARGET_PROCESS_TREE: PROCESS_LOOPBACK_MODE =
    PROCESS_LOOPBACK_MODE(1i32);
impl ::core::marker::Copy for PROCESS_LOOPBACK_MODE {}
impl ::core::clone::Clone for PROCESS_LOOPBACK_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_LOOPBACK_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROCESS_LOOPBACK_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_LOOPBACK_MODE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for PROCESS_LOOPBACK_MODE {
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
pub struct PartType(pub i32);
pub const Connector: PartType = PartType(0i32);
pub const Subunit: PartType = PartType(1i32);
impl ::core::marker::Copy for PartType {}
impl ::core::clone::Clone for PartType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PartType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PartType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PartType").field(&self.0).finish()
    }
}
impl FromIntoMemory for PartType {
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
pub const SND_ALIAS: i32 = 65536i32;
pub const SND_ALIAS_ID: i32 = 1114112i32;
pub const SND_ALIAS_START: u32 = 0u32;
pub const SND_APPLICATION: u32 = 128u32;
pub const SND_ASYNC: u32 = 1u32;
pub const SND_FILENAME: i32 = 131072i32;
pub const SND_LOOP: u32 = 8u32;
pub const SND_MEMORY: u32 = 4u32;
pub const SND_NODEFAULT: u32 = 2u32;
pub const SND_NOSTOP: u32 = 16u32;
pub const SND_NOWAIT: i32 = 8192i32;
pub const SND_PURGE: u32 = 64u32;
pub const SND_RESOURCE: i32 = 262148i32;
pub const SND_RING: i32 = 1048576i32;
pub const SND_SENTRY: i32 = 524288i32;
pub const SND_SYNC: u32 = 0u32;
pub const SND_SYSTEM: i32 = 2097152i32;
pub const SPATIAL_AUDIO_POSITION: u32 = 200u32;
pub const SPATIAL_AUDIO_STANDARD_COMMANDS_START: u32 = 200u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SPATIAL_AUDIO_STREAM_OPTIONS(pub u32);
pub const SPATIAL_AUDIO_STREAM_OPTIONS_NONE: SPATIAL_AUDIO_STREAM_OPTIONS =
    SPATIAL_AUDIO_STREAM_OPTIONS(0u32);
pub const SPATIAL_AUDIO_STREAM_OPTIONS_OFFLOAD: SPATIAL_AUDIO_STREAM_OPTIONS =
    SPATIAL_AUDIO_STREAM_OPTIONS(1u32);
impl ::core::marker::Copy for SPATIAL_AUDIO_STREAM_OPTIONS {}
impl ::core::clone::Clone for SPATIAL_AUDIO_STREAM_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPATIAL_AUDIO_STREAM_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SPATIAL_AUDIO_STREAM_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPATIAL_AUDIO_STREAM_OPTIONS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for SPATIAL_AUDIO_STREAM_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SPATIAL_AUDIO_STREAM_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SPATIAL_AUDIO_STREAM_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SPATIAL_AUDIO_STREAM_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SPATIAL_AUDIO_STREAM_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for SPATIAL_AUDIO_STREAM_OPTIONS {
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
pub const SPTLAUDCLNT_E_DESTROYED: crate::core::HRESULT = crate::core::HRESULT(-2004287232i32);
pub const SPTLAUDCLNT_E_ERRORS_IN_OBJECT_CALLS: crate::core::HRESULT =
    crate::core::HRESULT(-2004287227i32);
pub const SPTLAUDCLNT_E_INTERNAL: crate::core::HRESULT = crate::core::HRESULT(-2004287219i32);
pub const SPTLAUDCLNT_E_INVALID_LICENSE: crate::core::HRESULT =
    crate::core::HRESULT(-2004287224i32);
pub const SPTLAUDCLNT_E_METADATA_FORMAT_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2004287226i32);
pub const SPTLAUDCLNT_E_NO_MORE_OBJECTS: crate::core::HRESULT =
    crate::core::HRESULT(-2004287229i32);
pub const SPTLAUDCLNT_E_OBJECT_ALREADY_ACTIVE: crate::core::HRESULT =
    crate::core::HRESULT(-2004287220i32);
pub const SPTLAUDCLNT_E_OUT_OF_ORDER: crate::core::HRESULT = crate::core::HRESULT(-2004287231i32);
pub const SPTLAUDCLNT_E_PROPERTY_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2004287228i32);
pub const SPTLAUDCLNT_E_RESOURCES_INVALIDATED: crate::core::HRESULT =
    crate::core::HRESULT(-2004287230i32);
pub const SPTLAUDCLNT_E_STATIC_OBJECT_NOT_AVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2004287221i32);
pub const SPTLAUDCLNT_E_STREAM_NOT_AVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2004287225i32);
pub const SPTLAUDCLNT_E_STREAM_NOT_STOPPED: crate::core::HRESULT =
    crate::core::HRESULT(-2004287222i32);
pub const SPTLAUD_MD_CLNT_E_ATTACH_FAILED_INTERNAL_BUFFER: crate::core::HRESULT =
    crate::core::HRESULT(-2004286956i32);
pub const SPTLAUD_MD_CLNT_E_BUFFER_ALREADY_ATTACHED: crate::core::HRESULT =
    crate::core::HRESULT(-2004286969i32);
pub const SPTLAUD_MD_CLNT_E_BUFFER_NOT_ATTACHED: crate::core::HRESULT =
    crate::core::HRESULT(-2004286968i32);
pub const SPTLAUD_MD_CLNT_E_BUFFER_STILL_ATTACHED: crate::core::HRESULT =
    crate::core::HRESULT(-2004286940i32);
pub const SPTLAUD_MD_CLNT_E_COMMAND_ALREADY_WRITTEN: crate::core::HRESULT =
    crate::core::HRESULT(-2004286942i32);
pub const SPTLAUD_MD_CLNT_E_COMMAND_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-2004286976i32);
pub const SPTLAUD_MD_CLNT_E_DETACH_FAILED_INTERNAL_BUFFER: crate::core::HRESULT =
    crate::core::HRESULT(-2004286955i32);
pub const SPTLAUD_MD_CLNT_E_FORMAT_MISMATCH: crate::core::HRESULT =
    crate::core::HRESULT(-2004286941i32);
pub const SPTLAUD_MD_CLNT_E_FRAMECOUNT_OUT_OF_RANGE: crate::core::HRESULT =
    crate::core::HRESULT(-2004286967i32);
pub const SPTLAUD_MD_CLNT_E_FRAMEOFFSET_OUT_OF_RANGE: crate::core::HRESULT =
    crate::core::HRESULT(-2004286952i32);
pub const SPTLAUD_MD_CLNT_E_INVALID_ARGS: crate::core::HRESULT =
    crate::core::HRESULT(-2004286974i32);
pub const SPTLAUD_MD_CLNT_E_ITEMS_ALREADY_OPEN: crate::core::HRESULT =
    crate::core::HRESULT(-2004286957i32);
pub const SPTLAUD_MD_CLNT_E_ITEMS_LOCKED_FOR_WRITING: crate::core::HRESULT =
    crate::core::HRESULT(-2004286939i32);
pub const SPTLAUD_MD_CLNT_E_ITEM_COPY_OVERFLOW: crate::core::HRESULT =
    crate::core::HRESULT(-2004286959i32);
pub const SPTLAUD_MD_CLNT_E_ITEM_MUST_HAVE_COMMANDS: crate::core::HRESULT =
    crate::core::HRESULT(-2004286951i32);
pub const SPTLAUD_MD_CLNT_E_MEMORY_BOUNDS: crate::core::HRESULT =
    crate::core::HRESULT(-2004286971i32);
pub const SPTLAUD_MD_CLNT_E_METADATA_FORMAT_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-2004286973i32);
pub const SPTLAUD_MD_CLNT_E_NO_BUFFER_ATTACHED: crate::core::HRESULT =
    crate::core::HRESULT(-2004286954i32);
pub const SPTLAUD_MD_CLNT_E_NO_ITEMOFFSET_WRITTEN: crate::core::HRESULT =
    crate::core::HRESULT(-2004286944i32);
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-2004286960i32);
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_OPEN: crate::core::HRESULT =
    crate::core::HRESULT(-2004286958i32);
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_WRITTEN: crate::core::HRESULT =
    crate::core::HRESULT(-2004286943i32);
pub const SPTLAUD_MD_CLNT_E_NO_MORE_COMMANDS: crate::core::HRESULT =
    crate::core::HRESULT(-2004286970i32);
pub const SPTLAUD_MD_CLNT_E_NO_MORE_ITEMS: crate::core::HRESULT =
    crate::core::HRESULT(-2004286953i32);
pub const SPTLAUD_MD_CLNT_E_OBJECT_NOT_INITIALIZED: crate::core::HRESULT =
    crate::core::HRESULT(-2004286975i32);
pub const SPTLAUD_MD_CLNT_E_VALUE_BUFFER_INCORRECT_SIZE: crate::core::HRESULT =
    crate::core::HRESULT(-2004286972i32);
pub struct SpatialAudioClientActivationParams {
    pub tracingContextId: crate::core::GUID,
    pub appId: crate::core::GUID,
    pub majorVersion: i32,
    pub minorVersion1: i32,
    pub minorVersion2: i32,
    pub minorVersion3: i32,
}
impl ::core::marker::Copy for SpatialAudioClientActivationParams {}
impl ::core::clone::Clone for SpatialAudioClientActivationParams {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SpatialAudioClientActivationParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialAudioClientActivationParams")
            .field("tracingContextId", &self.tracingContextId)
            .field("appId", &self.appId)
            .field("majorVersion", &self.majorVersion)
            .field("minorVersion1", &self.minorVersion1)
            .field("minorVersion2", &self.minorVersion2)
            .field("minorVersion3", &self.minorVersion3)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SpatialAudioClientActivationParams {
    fn eq(&self, other: &Self) -> bool {
        self.tracingContextId == other.tracingContextId
            && self.appId == other.appId
            && self.majorVersion == other.majorVersion
            && self.minorVersion1 == other.minorVersion1
            && self.minorVersion2 == other.minorVersion2
            && self.minorVersion3 == other.minorVersion3
    }
}
impl ::core::cmp::Eq for SpatialAudioClientActivationParams {}
impl FromIntoMemory for SpatialAudioClientActivationParams {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_tracingContextId =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_appId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[16..16 + 16]);
        let f_majorVersion = <i32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_minorVersion1 = <i32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_minorVersion2 = <i32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_minorVersion3 = <i32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        Self {
            tracingContextId: f_tracingContextId,
            appId: f_appId,
            majorVersion: f_majorVersion,
            minorVersion1: f_minorVersion1,
            minorVersion2: f_minorVersion2,
            minorVersion3: f_minorVersion3,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.tracingContextId, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.appId, &mut into[16..16 + 16]);
        FromIntoMemory::into_bytes(self.majorVersion, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.minorVersion1, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.minorVersion2, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.minorVersion3, &mut into[44..44 + 4]);
    }
    fn size() -> usize {
        48
    }
}
pub struct SpatialAudioHrtfActivationParams {
    pub ObjectFormat: ConstPtr<WAVEFORMATEX>,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub NotifyObject: ISpatialAudioObjectRenderStreamNotify,
    pub DistanceDecay: MutPtr<SpatialAudioHrtfDistanceDecay>,
    pub Directivity: MutPtr<SpatialAudioHrtfDirectivityUnion>,
    pub Environment: MutPtr<SpatialAudioHrtfEnvironmentType>,
    pub Orientation: MutPtr<f32>,
}
impl ::core::marker::Copy for SpatialAudioHrtfActivationParams {}
impl ::core::clone::Clone for SpatialAudioHrtfActivationParams {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SpatialAudioHrtfActivationParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialAudioHrtfActivationParams")
            .field("ObjectFormat", &self.ObjectFormat)
            .field("StaticObjectTypeMask", &self.StaticObjectTypeMask)
            .field("MinDynamicObjectCount", &self.MinDynamicObjectCount)
            .field("MaxDynamicObjectCount", &self.MaxDynamicObjectCount)
            .field("Category", &self.Category)
            .field("EventHandle", &self.EventHandle)
            .field("NotifyObject", &self.NotifyObject)
            .field("DistanceDecay", &self.DistanceDecay)
            .field("Directivity", &self.Directivity)
            .field("Environment", &self.Environment)
            .field("Orientation", &self.Orientation)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SpatialAudioHrtfActivationParams {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectFormat == other.ObjectFormat
            && self.StaticObjectTypeMask == other.StaticObjectTypeMask
            && self.MinDynamicObjectCount == other.MinDynamicObjectCount
            && self.MaxDynamicObjectCount == other.MaxDynamicObjectCount
            && self.Category == other.Category
            && self.EventHandle == other.EventHandle
            && self.NotifyObject == other.NotifyObject
            && self.DistanceDecay == other.DistanceDecay
            && self.Directivity == other.Directivity
            && self.Environment == other.Environment
            && self.Orientation == other.Orientation
    }
}
impl ::core::cmp::Eq for SpatialAudioHrtfActivationParams {}
impl FromIntoMemory for SpatialAudioHrtfActivationParams {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 44);
        let f_ObjectFormat =
            <ConstPtr<WAVEFORMATEX> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_StaticObjectTypeMask =
            <AudioObjectType as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_MinDynamicObjectCount = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_MaxDynamicObjectCount = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_Category = <AUDIO_STREAM_CATEGORY as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_EventHandle =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_NotifyObject = <ISpatialAudioObjectRenderStreamNotify as FromIntoMemory>::from_bytes(
            &from[24..24 + 4],
        );
        let f_DistanceDecay = <MutPtr<SpatialAudioHrtfDistanceDecay> as FromIntoMemory>::from_bytes(
            &from[28..28 + 4],
        );
        let f_Directivity =
            <MutPtr<SpatialAudioHrtfDirectivityUnion> as FromIntoMemory>::from_bytes(
                &from[32..32 + 4],
            );
        let f_Environment = <MutPtr<SpatialAudioHrtfEnvironmentType> as FromIntoMemory>::from_bytes(
            &from[36..36 + 4],
        );
        let f_Orientation = <MutPtr<f32> as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        Self {
            ObjectFormat: f_ObjectFormat,
            StaticObjectTypeMask: f_StaticObjectTypeMask,
            MinDynamicObjectCount: f_MinDynamicObjectCount,
            MaxDynamicObjectCount: f_MaxDynamicObjectCount,
            Category: f_Category,
            EventHandle: f_EventHandle,
            NotifyObject: f_NotifyObject,
            DistanceDecay: f_DistanceDecay,
            Directivity: f_Directivity,
            Environment: f_Environment,
            Orientation: f_Orientation,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 44);
        FromIntoMemory::into_bytes(self.ObjectFormat, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.StaticObjectTypeMask, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.MinDynamicObjectCount, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.MaxDynamicObjectCount, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.Category, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.EventHandle, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.NotifyObject, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.DistanceDecay, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.Directivity, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.Environment, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.Orientation, &mut into[40..40 + 4]);
    }
    fn size() -> usize {
        44
    }
}
pub struct SpatialAudioHrtfActivationParams2 {
    pub ObjectFormat: ConstPtr<WAVEFORMATEX>,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub NotifyObject: ISpatialAudioObjectRenderStreamNotify,
    pub DistanceDecay: MutPtr<SpatialAudioHrtfDistanceDecay>,
    pub Directivity: MutPtr<SpatialAudioHrtfDirectivityUnion>,
    pub Environment: MutPtr<SpatialAudioHrtfEnvironmentType>,
    pub Orientation: MutPtr<f32>,
    pub Options: SPATIAL_AUDIO_STREAM_OPTIONS,
}
impl ::core::marker::Copy for SpatialAudioHrtfActivationParams2 {}
impl ::core::clone::Clone for SpatialAudioHrtfActivationParams2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SpatialAudioHrtfActivationParams2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialAudioHrtfActivationParams2")
            .field("ObjectFormat", &self.ObjectFormat)
            .field("StaticObjectTypeMask", &self.StaticObjectTypeMask)
            .field("MinDynamicObjectCount", &self.MinDynamicObjectCount)
            .field("MaxDynamicObjectCount", &self.MaxDynamicObjectCount)
            .field("Category", &self.Category)
            .field("EventHandle", &self.EventHandle)
            .field("NotifyObject", &self.NotifyObject)
            .field("DistanceDecay", &self.DistanceDecay)
            .field("Directivity", &self.Directivity)
            .field("Environment", &self.Environment)
            .field("Orientation", &self.Orientation)
            .field("Options", &self.Options)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SpatialAudioHrtfActivationParams2 {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectFormat == other.ObjectFormat
            && self.StaticObjectTypeMask == other.StaticObjectTypeMask
            && self.MinDynamicObjectCount == other.MinDynamicObjectCount
            && self.MaxDynamicObjectCount == other.MaxDynamicObjectCount
            && self.Category == other.Category
            && self.EventHandle == other.EventHandle
            && self.NotifyObject == other.NotifyObject
            && self.DistanceDecay == other.DistanceDecay
            && self.Directivity == other.Directivity
            && self.Environment == other.Environment
            && self.Orientation == other.Orientation
            && self.Options == other.Options
    }
}
impl ::core::cmp::Eq for SpatialAudioHrtfActivationParams2 {}
impl FromIntoMemory for SpatialAudioHrtfActivationParams2 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_ObjectFormat =
            <ConstPtr<WAVEFORMATEX> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_StaticObjectTypeMask =
            <AudioObjectType as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_MinDynamicObjectCount = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_MaxDynamicObjectCount = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_Category = <AUDIO_STREAM_CATEGORY as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_EventHandle =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_NotifyObject = <ISpatialAudioObjectRenderStreamNotify as FromIntoMemory>::from_bytes(
            &from[24..24 + 4],
        );
        let f_DistanceDecay = <MutPtr<SpatialAudioHrtfDistanceDecay> as FromIntoMemory>::from_bytes(
            &from[28..28 + 4],
        );
        let f_Directivity =
            <MutPtr<SpatialAudioHrtfDirectivityUnion> as FromIntoMemory>::from_bytes(
                &from[32..32 + 4],
            );
        let f_Environment = <MutPtr<SpatialAudioHrtfEnvironmentType> as FromIntoMemory>::from_bytes(
            &from[36..36 + 4],
        );
        let f_Orientation = <MutPtr<f32> as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_Options =
            <SPATIAL_AUDIO_STREAM_OPTIONS as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        Self {
            ObjectFormat: f_ObjectFormat,
            StaticObjectTypeMask: f_StaticObjectTypeMask,
            MinDynamicObjectCount: f_MinDynamicObjectCount,
            MaxDynamicObjectCount: f_MaxDynamicObjectCount,
            Category: f_Category,
            EventHandle: f_EventHandle,
            NotifyObject: f_NotifyObject,
            DistanceDecay: f_DistanceDecay,
            Directivity: f_Directivity,
            Environment: f_Environment,
            Orientation: f_Orientation,
            Options: f_Options,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.ObjectFormat, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.StaticObjectTypeMask, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.MinDynamicObjectCount, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.MaxDynamicObjectCount, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.Category, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.EventHandle, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.NotifyObject, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.DistanceDecay, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.Directivity, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.Environment, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.Orientation, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.Options, &mut into[44..44 + 4]);
    }
    fn size() -> usize {
        48
    }
}
pub struct SpatialAudioHrtfDirectivity {
    pub Type: SpatialAudioHrtfDirectivityType,
    pub Scaling: f32,
}
impl ::core::marker::Copy for SpatialAudioHrtfDirectivity {}
impl ::core::clone::Clone for SpatialAudioHrtfDirectivity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SpatialAudioHrtfDirectivity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialAudioHrtfDirectivity")
            .field("Type", &self.Type)
            .field("Scaling", &self.Scaling)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SpatialAudioHrtfDirectivity {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Scaling == other.Scaling
    }
}
impl ::core::cmp::Eq for SpatialAudioHrtfDirectivity {}
impl FromIntoMemory for SpatialAudioHrtfDirectivity {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Type =
            <SpatialAudioHrtfDirectivityType as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Scaling = <f32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            Type: f_Type,
            Scaling: f_Scaling,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Type, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Scaling, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct SpatialAudioHrtfDirectivityCardioid {
    pub directivity: SpatialAudioHrtfDirectivity,
    pub Order: f32,
}
impl ::core::marker::Copy for SpatialAudioHrtfDirectivityCardioid {}
impl ::core::clone::Clone for SpatialAudioHrtfDirectivityCardioid {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SpatialAudioHrtfDirectivityCardioid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialAudioHrtfDirectivityCardioid")
            .field("directivity", &self.directivity)
            .field("Order", &self.Order)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SpatialAudioHrtfDirectivityCardioid {
    fn eq(&self, other: &Self) -> bool {
        self.directivity == other.directivity && self.Order == other.Order
    }
}
impl ::core::cmp::Eq for SpatialAudioHrtfDirectivityCardioid {}
impl FromIntoMemory for SpatialAudioHrtfDirectivityCardioid {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_directivity =
            <SpatialAudioHrtfDirectivity as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_Order = <f32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            directivity: f_directivity,
            Order: f_Order,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.directivity, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.Order, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct SpatialAudioHrtfDirectivityCone {
    pub directivity: SpatialAudioHrtfDirectivity,
    pub InnerAngle: f32,
    pub OuterAngle: f32,
}
impl ::core::marker::Copy for SpatialAudioHrtfDirectivityCone {}
impl ::core::clone::Clone for SpatialAudioHrtfDirectivityCone {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SpatialAudioHrtfDirectivityCone {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialAudioHrtfDirectivityCone")
            .field("directivity", &self.directivity)
            .field("InnerAngle", &self.InnerAngle)
            .field("OuterAngle", &self.OuterAngle)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SpatialAudioHrtfDirectivityCone {
    fn eq(&self, other: &Self) -> bool {
        self.directivity == other.directivity
            && self.InnerAngle == other.InnerAngle
            && self.OuterAngle == other.OuterAngle
    }
}
impl ::core::cmp::Eq for SpatialAudioHrtfDirectivityCone {}
impl FromIntoMemory for SpatialAudioHrtfDirectivityCone {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_directivity =
            <SpatialAudioHrtfDirectivity as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_InnerAngle = <f32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_OuterAngle = <f32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            directivity: f_directivity,
            InnerAngle: f_InnerAngle,
            OuterAngle: f_OuterAngle,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.directivity, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.InnerAngle, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.OuterAngle, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SpatialAudioHrtfDirectivityType(pub i32);
pub const SpatialAudioHrtfDirectivity_OmniDirectional: SpatialAudioHrtfDirectivityType =
    SpatialAudioHrtfDirectivityType(0i32);
pub const SpatialAudioHrtfDirectivity_Cardioid: SpatialAudioHrtfDirectivityType =
    SpatialAudioHrtfDirectivityType(1i32);
pub const SpatialAudioHrtfDirectivity_Cone: SpatialAudioHrtfDirectivityType =
    SpatialAudioHrtfDirectivityType(2i32);
impl ::core::marker::Copy for SpatialAudioHrtfDirectivityType {}
impl ::core::clone::Clone for SpatialAudioHrtfDirectivityType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpatialAudioHrtfDirectivityType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SpatialAudioHrtfDirectivityType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAudioHrtfDirectivityType")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for SpatialAudioHrtfDirectivityType {
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
pub struct SpatialAudioHrtfDirectivityUnion {
    data: [u8; 8],
}
impl ::core::default::Default for SpatialAudioHrtfDirectivityUnion {
    fn default() -> Self {
        Self { data: [0u8; 8] }
    }
}
impl ::core::marker::Copy for SpatialAudioHrtfDirectivityUnion {}
impl ::core::clone::Clone for SpatialAudioHrtfDirectivityUnion {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SpatialAudioHrtfDirectivityUnion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialAudioHrtfDirectivityUnion")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SpatialAudioHrtfDirectivityUnion {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for SpatialAudioHrtfDirectivityUnion {}
impl FromIntoMemory for SpatialAudioHrtfDirectivityUnion {
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
pub struct SpatialAudioHrtfDistanceDecay {
    pub Type: SpatialAudioHrtfDistanceDecayType,
    pub MaxGain: f32,
    pub MinGain: f32,
    pub UnityGainDistance: f32,
    pub CutoffDistance: f32,
}
impl ::core::marker::Copy for SpatialAudioHrtfDistanceDecay {}
impl ::core::clone::Clone for SpatialAudioHrtfDistanceDecay {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SpatialAudioHrtfDistanceDecay {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialAudioHrtfDistanceDecay")
            .field("Type", &self.Type)
            .field("MaxGain", &self.MaxGain)
            .field("MinGain", &self.MinGain)
            .field("UnityGainDistance", &self.UnityGainDistance)
            .field("CutoffDistance", &self.CutoffDistance)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SpatialAudioHrtfDistanceDecay {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type
            && self.MaxGain == other.MaxGain
            && self.MinGain == other.MinGain
            && self.UnityGainDistance == other.UnityGainDistance
            && self.CutoffDistance == other.CutoffDistance
    }
}
impl ::core::cmp::Eq for SpatialAudioHrtfDistanceDecay {}
impl FromIntoMemory for SpatialAudioHrtfDistanceDecay {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_Type =
            <SpatialAudioHrtfDistanceDecayType as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_MaxGain = <f32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_MinGain = <f32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_UnityGainDistance = <f32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_CutoffDistance = <f32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            Type: f_Type,
            MaxGain: f_MaxGain,
            MinGain: f_MinGain,
            UnityGainDistance: f_UnityGainDistance,
            CutoffDistance: f_CutoffDistance,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.Type, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.MaxGain, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.MinGain, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.UnityGainDistance, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.CutoffDistance, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SpatialAudioHrtfDistanceDecayType(pub i32);
pub const SpatialAudioHrtfDistanceDecay_NaturalDecay: SpatialAudioHrtfDistanceDecayType =
    SpatialAudioHrtfDistanceDecayType(0i32);
pub const SpatialAudioHrtfDistanceDecay_CustomDecay: SpatialAudioHrtfDistanceDecayType =
    SpatialAudioHrtfDistanceDecayType(1i32);
impl ::core::marker::Copy for SpatialAudioHrtfDistanceDecayType {}
impl ::core::clone::Clone for SpatialAudioHrtfDistanceDecayType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpatialAudioHrtfDistanceDecayType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SpatialAudioHrtfDistanceDecayType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAudioHrtfDistanceDecayType")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for SpatialAudioHrtfDistanceDecayType {
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
pub struct SpatialAudioHrtfEnvironmentType(pub i32);
pub const SpatialAudioHrtfEnvironment_Small: SpatialAudioHrtfEnvironmentType =
    SpatialAudioHrtfEnvironmentType(0i32);
pub const SpatialAudioHrtfEnvironment_Medium: SpatialAudioHrtfEnvironmentType =
    SpatialAudioHrtfEnvironmentType(1i32);
pub const SpatialAudioHrtfEnvironment_Large: SpatialAudioHrtfEnvironmentType =
    SpatialAudioHrtfEnvironmentType(2i32);
pub const SpatialAudioHrtfEnvironment_Outdoors: SpatialAudioHrtfEnvironmentType =
    SpatialAudioHrtfEnvironmentType(3i32);
pub const SpatialAudioHrtfEnvironment_Average: SpatialAudioHrtfEnvironmentType =
    SpatialAudioHrtfEnvironmentType(4i32);
impl ::core::marker::Copy for SpatialAudioHrtfEnvironmentType {}
impl ::core::clone::Clone for SpatialAudioHrtfEnvironmentType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpatialAudioHrtfEnvironmentType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SpatialAudioHrtfEnvironmentType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAudioHrtfEnvironmentType")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for SpatialAudioHrtfEnvironmentType {
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
pub struct SpatialAudioMetadataCopyMode(pub i32);
pub const SpatialAudioMetadataCopy_Overwrite: SpatialAudioMetadataCopyMode =
    SpatialAudioMetadataCopyMode(0i32);
pub const SpatialAudioMetadataCopy_Append: SpatialAudioMetadataCopyMode =
    SpatialAudioMetadataCopyMode(1i32);
pub const SpatialAudioMetadataCopy_AppendMergeWithLast: SpatialAudioMetadataCopyMode =
    SpatialAudioMetadataCopyMode(2i32);
pub const SpatialAudioMetadataCopy_AppendMergeWithFirst: SpatialAudioMetadataCopyMode =
    SpatialAudioMetadataCopyMode(3i32);
impl ::core::marker::Copy for SpatialAudioMetadataCopyMode {}
impl ::core::clone::Clone for SpatialAudioMetadataCopyMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpatialAudioMetadataCopyMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SpatialAudioMetadataCopyMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAudioMetadataCopyMode")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for SpatialAudioMetadataCopyMode {
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
pub struct SpatialAudioMetadataItemsInfo {
    pub FrameCount: u16,
    pub ItemCount: u16,
    pub MaxItemCount: u16,
    pub MaxValueBufferLength: u32,
}
impl ::core::marker::Copy for SpatialAudioMetadataItemsInfo {}
impl ::core::clone::Clone for SpatialAudioMetadataItemsInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SpatialAudioMetadataItemsInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialAudioMetadataItemsInfo")
            .field("FrameCount", &self.FrameCount)
            .field("ItemCount", &self.ItemCount)
            .field("MaxItemCount", &self.MaxItemCount)
            .field("MaxValueBufferLength", &self.MaxValueBufferLength)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SpatialAudioMetadataItemsInfo {
    fn eq(&self, other: &Self) -> bool {
        self.FrameCount == other.FrameCount
            && self.ItemCount == other.ItemCount
            && self.MaxItemCount == other.MaxItemCount
            && self.MaxValueBufferLength == other.MaxValueBufferLength
    }
}
impl ::core::cmp::Eq for SpatialAudioMetadataItemsInfo {}
impl FromIntoMemory for SpatialAudioMetadataItemsInfo {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 10);
        let f_FrameCount = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_ItemCount = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_MaxItemCount = <u16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_MaxValueBufferLength = <u32 as FromIntoMemory>::from_bytes(&from[6..6 + 4]);
        Self {
            FrameCount: f_FrameCount,
            ItemCount: f_ItemCount,
            MaxItemCount: f_MaxItemCount,
            MaxValueBufferLength: f_MaxValueBufferLength,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 10);
        FromIntoMemory::into_bytes(self.FrameCount, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.ItemCount, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.MaxItemCount, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.MaxValueBufferLength, &mut into[6..6 + 4]);
    }
    fn size() -> usize {
        10
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SpatialAudioMetadataWriterOverflowMode(pub i32);
pub const SpatialAudioMetadataWriterOverflow_Fail: SpatialAudioMetadataWriterOverflowMode =
    SpatialAudioMetadataWriterOverflowMode(0i32);
pub const SpatialAudioMetadataWriterOverflow_MergeWithNew: SpatialAudioMetadataWriterOverflowMode =
    SpatialAudioMetadataWriterOverflowMode(1i32);
pub const SpatialAudioMetadataWriterOverflow_MergeWithLast: SpatialAudioMetadataWriterOverflowMode =
    SpatialAudioMetadataWriterOverflowMode(2i32);
impl ::core::marker::Copy for SpatialAudioMetadataWriterOverflowMode {}
impl ::core::clone::Clone for SpatialAudioMetadataWriterOverflowMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpatialAudioMetadataWriterOverflowMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SpatialAudioMetadataWriterOverflowMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAudioMetadataWriterOverflowMode")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for SpatialAudioMetadataWriterOverflowMode {
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
pub struct SpatialAudioObjectRenderStreamActivationParams {
    pub ObjectFormat: ConstPtr<WAVEFORMATEX>,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub NotifyObject: ISpatialAudioObjectRenderStreamNotify,
}
impl ::core::marker::Copy for SpatialAudioObjectRenderStreamActivationParams {}
impl ::core::clone::Clone for SpatialAudioObjectRenderStreamActivationParams {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SpatialAudioObjectRenderStreamActivationParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialAudioObjectRenderStreamActivationParams")
            .field("ObjectFormat", &self.ObjectFormat)
            .field("StaticObjectTypeMask", &self.StaticObjectTypeMask)
            .field("MinDynamicObjectCount", &self.MinDynamicObjectCount)
            .field("MaxDynamicObjectCount", &self.MaxDynamicObjectCount)
            .field("Category", &self.Category)
            .field("EventHandle", &self.EventHandle)
            .field("NotifyObject", &self.NotifyObject)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SpatialAudioObjectRenderStreamActivationParams {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectFormat == other.ObjectFormat
            && self.StaticObjectTypeMask == other.StaticObjectTypeMask
            && self.MinDynamicObjectCount == other.MinDynamicObjectCount
            && self.MaxDynamicObjectCount == other.MaxDynamicObjectCount
            && self.Category == other.Category
            && self.EventHandle == other.EventHandle
            && self.NotifyObject == other.NotifyObject
    }
}
impl ::core::cmp::Eq for SpatialAudioObjectRenderStreamActivationParams {}
impl FromIntoMemory for SpatialAudioObjectRenderStreamActivationParams {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_ObjectFormat =
            <ConstPtr<WAVEFORMATEX> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_StaticObjectTypeMask =
            <AudioObjectType as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_MinDynamicObjectCount = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_MaxDynamicObjectCount = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_Category = <AUDIO_STREAM_CATEGORY as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_EventHandle =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_NotifyObject = <ISpatialAudioObjectRenderStreamNotify as FromIntoMemory>::from_bytes(
            &from[24..24 + 4],
        );
        Self {
            ObjectFormat: f_ObjectFormat,
            StaticObjectTypeMask: f_StaticObjectTypeMask,
            MinDynamicObjectCount: f_MinDynamicObjectCount,
            MaxDynamicObjectCount: f_MaxDynamicObjectCount,
            Category: f_Category,
            EventHandle: f_EventHandle,
            NotifyObject: f_NotifyObject,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.ObjectFormat, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.StaticObjectTypeMask, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.MinDynamicObjectCount, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.MaxDynamicObjectCount, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.Category, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.EventHandle, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.NotifyObject, &mut into[24..24 + 4]);
    }
    fn size() -> usize {
        28
    }
}
pub struct SpatialAudioObjectRenderStreamActivationParams2 {
    pub ObjectFormat: ConstPtr<WAVEFORMATEX>,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub NotifyObject: ISpatialAudioObjectRenderStreamNotify,
    pub Options: SPATIAL_AUDIO_STREAM_OPTIONS,
}
impl ::core::marker::Copy for SpatialAudioObjectRenderStreamActivationParams2 {}
impl ::core::clone::Clone for SpatialAudioObjectRenderStreamActivationParams2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SpatialAudioObjectRenderStreamActivationParams2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialAudioObjectRenderStreamActivationParams2")
            .field("ObjectFormat", &self.ObjectFormat)
            .field("StaticObjectTypeMask", &self.StaticObjectTypeMask)
            .field("MinDynamicObjectCount", &self.MinDynamicObjectCount)
            .field("MaxDynamicObjectCount", &self.MaxDynamicObjectCount)
            .field("Category", &self.Category)
            .field("EventHandle", &self.EventHandle)
            .field("NotifyObject", &self.NotifyObject)
            .field("Options", &self.Options)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SpatialAudioObjectRenderStreamActivationParams2 {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectFormat == other.ObjectFormat
            && self.StaticObjectTypeMask == other.StaticObjectTypeMask
            && self.MinDynamicObjectCount == other.MinDynamicObjectCount
            && self.MaxDynamicObjectCount == other.MaxDynamicObjectCount
            && self.Category == other.Category
            && self.EventHandle == other.EventHandle
            && self.NotifyObject == other.NotifyObject
            && self.Options == other.Options
    }
}
impl ::core::cmp::Eq for SpatialAudioObjectRenderStreamActivationParams2 {}
impl FromIntoMemory for SpatialAudioObjectRenderStreamActivationParams2 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_ObjectFormat =
            <ConstPtr<WAVEFORMATEX> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_StaticObjectTypeMask =
            <AudioObjectType as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_MinDynamicObjectCount = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_MaxDynamicObjectCount = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_Category = <AUDIO_STREAM_CATEGORY as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_EventHandle =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_NotifyObject = <ISpatialAudioObjectRenderStreamNotify as FromIntoMemory>::from_bytes(
            &from[24..24 + 4],
        );
        let f_Options =
            <SPATIAL_AUDIO_STREAM_OPTIONS as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            ObjectFormat: f_ObjectFormat,
            StaticObjectTypeMask: f_StaticObjectTypeMask,
            MinDynamicObjectCount: f_MinDynamicObjectCount,
            MaxDynamicObjectCount: f_MaxDynamicObjectCount,
            Category: f_Category,
            EventHandle: f_EventHandle,
            NotifyObject: f_NotifyObject,
            Options: f_Options,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.ObjectFormat, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.StaticObjectTypeMask, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.MinDynamicObjectCount, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.MaxDynamicObjectCount, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.Category, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.EventHandle, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.NotifyObject, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.Options, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct SpatialAudioObjectRenderStreamForMetadataActivationParams {
    pub ObjectFormat: ConstPtr<WAVEFORMATEX>,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub MetadataFormatId: crate::core::GUID,
    pub MaxMetadataItemCount: u16,
    pub MetadataActivationParams:
        ConstPtr<super::super::System::Com::StructuredStorage::PROPVARIANT>,
    pub NotifyObject: ISpatialAudioObjectRenderStreamNotify,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for SpatialAudioObjectRenderStreamForMetadataActivationParams {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for SpatialAudioObjectRenderStreamForMetadataActivationParams {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for SpatialAudioObjectRenderStreamForMetadataActivationParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialAudioObjectRenderStreamForMetadataActivationParams")
            .field("ObjectFormat", &self.ObjectFormat)
            .field("StaticObjectTypeMask", &self.StaticObjectTypeMask)
            .field("MinDynamicObjectCount", &self.MinDynamicObjectCount)
            .field("MaxDynamicObjectCount", &self.MaxDynamicObjectCount)
            .field("Category", &self.Category)
            .field("EventHandle", &self.EventHandle)
            .field("MetadataFormatId", &self.MetadataFormatId)
            .field("MaxMetadataItemCount", &self.MaxMetadataItemCount)
            .field("MetadataActivationParams", &self.MetadataActivationParams)
            .field("NotifyObject", &self.NotifyObject)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for SpatialAudioObjectRenderStreamForMetadataActivationParams {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectFormat == other.ObjectFormat
            && self.StaticObjectTypeMask == other.StaticObjectTypeMask
            && self.MinDynamicObjectCount == other.MinDynamicObjectCount
            && self.MaxDynamicObjectCount == other.MaxDynamicObjectCount
            && self.Category == other.Category
            && self.EventHandle == other.EventHandle
            && self.MetadataFormatId == other.MetadataFormatId
            && self.MaxMetadataItemCount == other.MaxMetadataItemCount
            && self.MetadataActivationParams == other.MetadataActivationParams
            && self.NotifyObject == other.NotifyObject
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for SpatialAudioObjectRenderStreamForMetadataActivationParams {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for SpatialAudioObjectRenderStreamForMetadataActivationParams {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 50);
        let f_ObjectFormat =
            <ConstPtr<WAVEFORMATEX> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_StaticObjectTypeMask =
            <AudioObjectType as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_MinDynamicObjectCount = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_MaxDynamicObjectCount = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_Category = <AUDIO_STREAM_CATEGORY as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_EventHandle =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_MetadataFormatId =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[24..24 + 16]);
        let f_MaxMetadataItemCount = <u16 as FromIntoMemory>::from_bytes(&from[40..40 + 2]);
        let f_MetadataActivationParams = <ConstPtr<
            super::super::System::Com::StructuredStorage::PROPVARIANT,
        > as FromIntoMemory>::from_bytes(&from[42..42 + 4]);
        let f_NotifyObject = <ISpatialAudioObjectRenderStreamNotify as FromIntoMemory>::from_bytes(
            &from[46..46 + 4],
        );
        Self {
            ObjectFormat: f_ObjectFormat,
            StaticObjectTypeMask: f_StaticObjectTypeMask,
            MinDynamicObjectCount: f_MinDynamicObjectCount,
            MaxDynamicObjectCount: f_MaxDynamicObjectCount,
            Category: f_Category,
            EventHandle: f_EventHandle,
            MetadataFormatId: f_MetadataFormatId,
            MaxMetadataItemCount: f_MaxMetadataItemCount,
            MetadataActivationParams: f_MetadataActivationParams,
            NotifyObject: f_NotifyObject,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 50);
        FromIntoMemory::into_bytes(self.ObjectFormat, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.StaticObjectTypeMask, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.MinDynamicObjectCount, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.MaxDynamicObjectCount, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.Category, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.EventHandle, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.MetadataFormatId, &mut into[24..24 + 16]);
        FromIntoMemory::into_bytes(self.MaxMetadataItemCount, &mut into[40..40 + 2]);
        FromIntoMemory::into_bytes(self.MetadataActivationParams, &mut into[42..42 + 4]);
        FromIntoMemory::into_bytes(self.NotifyObject, &mut into[46..46 + 4]);
    }
    fn size() -> usize {
        50
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    pub ObjectFormat: ConstPtr<WAVEFORMATEX>,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub MetadataFormatId: crate::core::GUID,
    pub MaxMetadataItemCount: u32,
    pub MetadataActivationParams:
        ConstPtr<super::super::System::Com::StructuredStorage::PROPVARIANT>,
    pub NotifyObject: ISpatialAudioObjectRenderStreamNotify,
    pub Options: SPATIAL_AUDIO_STREAM_OPTIONS,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for SpatialAudioObjectRenderStreamForMetadataActivationParams2 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialAudioObjectRenderStreamForMetadataActivationParams2")
            .field("ObjectFormat", &self.ObjectFormat)
            .field("StaticObjectTypeMask", &self.StaticObjectTypeMask)
            .field("MinDynamicObjectCount", &self.MinDynamicObjectCount)
            .field("MaxDynamicObjectCount", &self.MaxDynamicObjectCount)
            .field("Category", &self.Category)
            .field("EventHandle", &self.EventHandle)
            .field("MetadataFormatId", &self.MetadataFormatId)
            .field("MaxMetadataItemCount", &self.MaxMetadataItemCount)
            .field("MetadataActivationParams", &self.MetadataActivationParams)
            .field("NotifyObject", &self.NotifyObject)
            .field("Options", &self.Options)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectFormat == other.ObjectFormat
            && self.StaticObjectTypeMask == other.StaticObjectTypeMask
            && self.MinDynamicObjectCount == other.MinDynamicObjectCount
            && self.MaxDynamicObjectCount == other.MaxDynamicObjectCount
            && self.Category == other.Category
            && self.EventHandle == other.EventHandle
            && self.MetadataFormatId == other.MetadataFormatId
            && self.MaxMetadataItemCount == other.MaxMetadataItemCount
            && self.MetadataActivationParams == other.MetadataActivationParams
            && self.NotifyObject == other.NotifyObject
            && self.Options == other.Options
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for SpatialAudioObjectRenderStreamForMetadataActivationParams2 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 56);
        let f_ObjectFormat =
            <ConstPtr<WAVEFORMATEX> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_StaticObjectTypeMask =
            <AudioObjectType as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_MinDynamicObjectCount = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_MaxDynamicObjectCount = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_Category = <AUDIO_STREAM_CATEGORY as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_EventHandle =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_MetadataFormatId =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[24..24 + 16]);
        let f_MaxMetadataItemCount = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_MetadataActivationParams = <ConstPtr<
            super::super::System::Com::StructuredStorage::PROPVARIANT,
        > as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_NotifyObject = <ISpatialAudioObjectRenderStreamNotify as FromIntoMemory>::from_bytes(
            &from[48..48 + 4],
        );
        let f_Options =
            <SPATIAL_AUDIO_STREAM_OPTIONS as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        Self {
            ObjectFormat: f_ObjectFormat,
            StaticObjectTypeMask: f_StaticObjectTypeMask,
            MinDynamicObjectCount: f_MinDynamicObjectCount,
            MaxDynamicObjectCount: f_MaxDynamicObjectCount,
            Category: f_Category,
            EventHandle: f_EventHandle,
            MetadataFormatId: f_MetadataFormatId,
            MaxMetadataItemCount: f_MaxMetadataItemCount,
            MetadataActivationParams: f_MetadataActivationParams,
            NotifyObject: f_NotifyObject,
            Options: f_Options,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 56);
        FromIntoMemory::into_bytes(self.ObjectFormat, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.StaticObjectTypeMask, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.MinDynamicObjectCount, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.MaxDynamicObjectCount, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.Category, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.EventHandle, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.MetadataFormatId, &mut into[24..24 + 16]);
        FromIntoMemory::into_bytes(self.MaxMetadataItemCount, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.MetadataActivationParams, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.NotifyObject, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.Options, &mut into[52..52 + 4]);
    }
    fn size() -> usize {
        56
    }
}
pub const VIRTUAL_AUDIO_DEVICE_PROCESS_LOOPBACK: &'static str = "VAD\\Process_Loopback";
pub struct VOLUMEWAVEFILTER {
    pub wfltr: WAVEFILTER,
    pub dwVolume: u32,
}
impl ::core::marker::Copy for VOLUMEWAVEFILTER {}
impl ::core::clone::Clone for VOLUMEWAVEFILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VOLUMEWAVEFILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUMEWAVEFILTER")
            .field("wfltr", &self.wfltr)
            .field("dwVolume", &self.dwVolume)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VOLUMEWAVEFILTER {
    fn eq(&self, other: &Self) -> bool {
        self.wfltr == other.wfltr && self.dwVolume == other.dwVolume
    }
}
impl ::core::cmp::Eq for VOLUMEWAVEFILTER {}
impl FromIntoMemory for VOLUMEWAVEFILTER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 36);
        let f_wfltr = <WAVEFILTER as FromIntoMemory>::from_bytes(&from[0..0 + 32]);
        let f_dwVolume = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        Self {
            wfltr: f_wfltr,
            dwVolume: f_dwVolume,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 36);
        FromIntoMemory::into_bytes(self.wfltr, &mut into[0..0 + 32]);
        FromIntoMemory::into_bytes(self.dwVolume, &mut into[32..32 + 4]);
    }
    fn size() -> usize {
        36
    }
}
pub const WAVECAPS_LRVOLUME: u32 = 8u32;
pub const WAVECAPS_PITCH: u32 = 1u32;
pub const WAVECAPS_PLAYBACKRATE: u32 = 2u32;
pub const WAVECAPS_SAMPLEACCURATE: u32 = 32u32;
pub const WAVECAPS_SYNC: u32 = 16u32;
pub const WAVECAPS_VOLUME: u32 = 4u32;
pub struct WAVEFILTER {
    pub cbStruct: u32,
    pub dwFilterTag: u32,
    pub fdwFilter: u32,
    pub dwReserved: [u32; 5],
}
impl ::core::marker::Copy for WAVEFILTER {}
impl ::core::clone::Clone for WAVEFILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WAVEFILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WAVEFILTER")
            .field("cbStruct", &self.cbStruct)
            .field("dwFilterTag", &self.dwFilterTag)
            .field("fdwFilter", &self.fdwFilter)
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WAVEFILTER {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwFilterTag == other.dwFilterTag
            && self.fdwFilter == other.fdwFilter
            && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for WAVEFILTER {}
impl FromIntoMemory for WAVEFILTER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFilterTag = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_fdwFilter = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwReserved = <[u32; 5] as FromIntoMemory>::from_bytes(&from[12..12 + 20]);
        Self {
            cbStruct: f_cbStruct,
            dwFilterTag: f_dwFilterTag,
            fdwFilter: f_fdwFilter,
            dwReserved: f_dwReserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFilterTag, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.fdwFilter, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwReserved, &mut into[12..12 + 20]);
    }
    fn size() -> usize {
        32
    }
}
pub struct WAVEFORMAT {
    pub wFormatTag: u16,
    pub nChannels: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
}
impl ::core::marker::Copy for WAVEFORMAT {}
impl ::core::clone::Clone for WAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WAVEFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WAVEFORMAT")
            .field("wFormatTag", &self.wFormatTag)
            .field("nChannels", &self.nChannels)
            .field("nSamplesPerSec", &self.nSamplesPerSec)
            .field("nAvgBytesPerSec", &self.nAvgBytesPerSec)
            .field("nBlockAlign", &self.nBlockAlign)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.wFormatTag == other.wFormatTag
            && self.nChannels == other.nChannels
            && self.nSamplesPerSec == other.nSamplesPerSec
            && self.nAvgBytesPerSec == other.nAvgBytesPerSec
            && self.nBlockAlign == other.nBlockAlign
    }
}
impl ::core::cmp::Eq for WAVEFORMAT {}
impl FromIntoMemory for WAVEFORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 14);
        let f_wFormatTag = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_nChannels = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_nSamplesPerSec = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_nAvgBytesPerSec = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_nBlockAlign = <u16 as FromIntoMemory>::from_bytes(&from[12..12 + 2]);
        Self {
            wFormatTag: f_wFormatTag,
            nChannels: f_nChannels,
            nSamplesPerSec: f_nSamplesPerSec,
            nAvgBytesPerSec: f_nAvgBytesPerSec,
            nBlockAlign: f_nBlockAlign,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 14);
        FromIntoMemory::into_bytes(self.wFormatTag, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.nChannels, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.nSamplesPerSec, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.nAvgBytesPerSec, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.nBlockAlign, &mut into[12..12 + 2]);
    }
    fn size() -> usize {
        14
    }
}
pub struct WAVEFORMATEX {
    pub wFormatTag: u16,
    pub nChannels: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
    pub wBitsPerSample: u16,
    pub cbSize: u16,
}
impl ::core::marker::Copy for WAVEFORMATEX {}
impl ::core::clone::Clone for WAVEFORMATEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WAVEFORMATEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WAVEFORMATEX")
            .field("wFormatTag", &self.wFormatTag)
            .field("nChannels", &self.nChannels)
            .field("nSamplesPerSec", &self.nSamplesPerSec)
            .field("nAvgBytesPerSec", &self.nAvgBytesPerSec)
            .field("nBlockAlign", &self.nBlockAlign)
            .field("wBitsPerSample", &self.wBitsPerSample)
            .field("cbSize", &self.cbSize)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WAVEFORMATEX {
    fn eq(&self, other: &Self) -> bool {
        self.wFormatTag == other.wFormatTag
            && self.nChannels == other.nChannels
            && self.nSamplesPerSec == other.nSamplesPerSec
            && self.nAvgBytesPerSec == other.nAvgBytesPerSec
            && self.nBlockAlign == other.nBlockAlign
            && self.wBitsPerSample == other.wBitsPerSample
            && self.cbSize == other.cbSize
    }
}
impl ::core::cmp::Eq for WAVEFORMATEX {}
impl FromIntoMemory for WAVEFORMATEX {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 18);
        let f_wFormatTag = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_nChannels = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_nSamplesPerSec = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_nAvgBytesPerSec = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_nBlockAlign = <u16 as FromIntoMemory>::from_bytes(&from[12..12 + 2]);
        let f_wBitsPerSample = <u16 as FromIntoMemory>::from_bytes(&from[14..14 + 2]);
        let f_cbSize = <u16 as FromIntoMemory>::from_bytes(&from[16..16 + 2]);
        Self {
            wFormatTag: f_wFormatTag,
            nChannels: f_nChannels,
            nSamplesPerSec: f_nSamplesPerSec,
            nAvgBytesPerSec: f_nAvgBytesPerSec,
            nBlockAlign: f_nBlockAlign,
            wBitsPerSample: f_wBitsPerSample,
            cbSize: f_cbSize,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 18);
        FromIntoMemory::into_bytes(self.wFormatTag, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.nChannels, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.nSamplesPerSec, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.nAvgBytesPerSec, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.nBlockAlign, &mut into[12..12 + 2]);
        FromIntoMemory::into_bytes(self.wBitsPerSample, &mut into[14..14 + 2]);
        FromIntoMemory::into_bytes(self.cbSize, &mut into[16..16 + 2]);
    }
    fn size() -> usize {
        18
    }
}
pub struct WAVEFORMATEXTENSIBLE {
    pub Format: WAVEFORMATEX,
    pub Samples: WAVEFORMATEXTENSIBLE_0,
    pub dwChannelMask: u32,
    pub SubFormat: crate::core::GUID,
}
impl ::core::marker::Copy for WAVEFORMATEXTENSIBLE {}
impl ::core::clone::Clone for WAVEFORMATEXTENSIBLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WAVEFORMATEXTENSIBLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WAVEFORMATEXTENSIBLE")
            .field("Format", &self.Format)
            .field("Samples", &self.Samples)
            .field("dwChannelMask", &self.dwChannelMask)
            .field("SubFormat", &self.SubFormat)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WAVEFORMATEXTENSIBLE {
    fn eq(&self, other: &Self) -> bool {
        self.Format == other.Format
            && self.Samples == other.Samples
            && self.dwChannelMask == other.dwChannelMask
            && self.SubFormat == other.SubFormat
    }
}
impl ::core::cmp::Eq for WAVEFORMATEXTENSIBLE {}
impl FromIntoMemory for WAVEFORMATEXTENSIBLE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_Format = <WAVEFORMATEX as FromIntoMemory>::from_bytes(&from[0..0 + 18]);
        let f_Samples = <WAVEFORMATEXTENSIBLE_0 as FromIntoMemory>::from_bytes(&from[18..18 + 2]);
        let f_dwChannelMask = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_SubFormat = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[24..24 + 16]);
        Self {
            Format: f_Format,
            Samples: f_Samples,
            dwChannelMask: f_dwChannelMask,
            SubFormat: f_SubFormat,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.Format, &mut into[0..0 + 18]);
        FromIntoMemory::into_bytes(self.Samples, &mut into[18..18 + 2]);
        FromIntoMemory::into_bytes(self.dwChannelMask, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.SubFormat, &mut into[24..24 + 16]);
    }
    fn size() -> usize {
        40
    }
}
pub struct WAVEFORMATEXTENSIBLE_0 {
    data: [u8; 2],
}
impl ::core::default::Default for WAVEFORMATEXTENSIBLE_0 {
    fn default() -> Self {
        Self { data: [0u8; 2] }
    }
}
impl ::core::marker::Copy for WAVEFORMATEXTENSIBLE_0 {}
impl ::core::clone::Clone for WAVEFORMATEXTENSIBLE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WAVEFORMATEXTENSIBLE_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WAVEFORMATEXTENSIBLE_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WAVEFORMATEXTENSIBLE_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for WAVEFORMATEXTENSIBLE_0 {}
impl FromIntoMemory for WAVEFORMATEXTENSIBLE_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 2];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        2
    }
}
pub struct WAVEHDR {
    pub lpData: PSTR,
    pub dwBufferLength: u32,
    pub dwBytesRecorded: u32,
    pub dwUser: PtrRepr,
    pub dwFlags: u32,
    pub dwLoops: u32,
    pub lpNext: MutPtr<WAVEHDR>,
    pub reserved: PtrRepr,
}
impl ::core::marker::Copy for WAVEHDR {}
impl ::core::clone::Clone for WAVEHDR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WAVEHDR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WAVEHDR")
            .field("lpData", &self.lpData)
            .field("dwBufferLength", &self.dwBufferLength)
            .field("dwBytesRecorded", &self.dwBytesRecorded)
            .field("dwUser", &self.dwUser)
            .field("dwFlags", &self.dwFlags)
            .field("dwLoops", &self.dwLoops)
            .field("lpNext", &self.lpNext)
            .field("reserved", &self.reserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WAVEHDR {
    fn eq(&self, other: &Self) -> bool {
        self.lpData == other.lpData
            && self.dwBufferLength == other.dwBufferLength
            && self.dwBytesRecorded == other.dwBytesRecorded
            && self.dwUser == other.dwUser
            && self.dwFlags == other.dwFlags
            && self.dwLoops == other.dwLoops
            && self.lpNext == other.lpNext
            && self.reserved == other.reserved
    }
}
impl ::core::cmp::Eq for WAVEHDR {}
impl FromIntoMemory for WAVEHDR {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_lpData = <PSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwBufferLength = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwBytesRecorded = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwUser = <PtrRepr as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwLoops = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_lpNext = <MutPtr<WAVEHDR> as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_reserved = <PtrRepr as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            lpData: f_lpData,
            dwBufferLength: f_dwBufferLength,
            dwBytesRecorded: f_dwBytesRecorded,
            dwUser: f_dwUser,
            dwFlags: f_dwFlags,
            dwLoops: f_dwLoops,
            lpNext: f_lpNext,
            reserved: f_reserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.lpData, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwBufferLength, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwBytesRecorded, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwUser, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwLoops, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.lpNext, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.reserved, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
pub struct WAVEINCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub ManufacturerGuid: crate::core::GUID,
    pub ProductGuid: crate::core::GUID,
    pub NameGuid: crate::core::GUID,
}
impl ::core::marker::Copy for WAVEINCAPS2A {}
impl ::core::clone::Clone for WAVEINCAPS2A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WAVEINCAPS2A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WAVEINCAPS2A")
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("vDriverVersion", &self.vDriverVersion)
            .field("szPname", &self.szPname)
            .field("dwFormats", &self.dwFormats)
            .field("wChannels", &self.wChannels)
            .field("wReserved1", &self.wReserved1)
            .field("ManufacturerGuid", &self.ManufacturerGuid)
            .field("ProductGuid", &self.ProductGuid)
            .field("NameGuid", &self.NameGuid)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WAVEINCAPS2A {
    fn eq(&self, other: &Self) -> bool {
        self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.vDriverVersion == other.vDriverVersion
            && self.szPname == other.szPname
            && self.dwFormats == other.dwFormats
            && self.wChannels == other.wChannels
            && self.wReserved1 == other.wReserved1
            && self.ManufacturerGuid == other.ManufacturerGuid
            && self.ProductGuid == other.ProductGuid
            && self.NameGuid == other.NameGuid
    }
}
impl ::core::cmp::Eq for WAVEINCAPS2A {}
impl FromIntoMemory for WAVEINCAPS2A {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 96);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_vDriverVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szPname =
            <[super::super::Foundation::CHAR; 32] as FromIntoMemory>::from_bytes(&from[8..8 + 32]);
        let f_dwFormats = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_wChannels = <u16 as FromIntoMemory>::from_bytes(&from[44..44 + 2]);
        let f_wReserved1 = <u16 as FromIntoMemory>::from_bytes(&from[46..46 + 2]);
        let f_ManufacturerGuid =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[48..48 + 16]);
        let f_ProductGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[64..64 + 16]);
        let f_NameGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[80..80 + 16]);
        Self {
            wMid: f_wMid,
            wPid: f_wPid,
            vDriverVersion: f_vDriverVersion,
            szPname: f_szPname,
            dwFormats: f_dwFormats,
            wChannels: f_wChannels,
            wReserved1: f_wReserved1,
            ManufacturerGuid: f_ManufacturerGuid,
            ProductGuid: f_ProductGuid,
            NameGuid: f_NameGuid,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 96);
        FromIntoMemory::into_bytes(self.wMid, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.vDriverVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[8..8 + 32]);
        FromIntoMemory::into_bytes(self.dwFormats, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.wChannels, &mut into[44..44 + 2]);
        FromIntoMemory::into_bytes(self.wReserved1, &mut into[46..46 + 2]);
        FromIntoMemory::into_bytes(self.ManufacturerGuid, &mut into[48..48 + 16]);
        FromIntoMemory::into_bytes(self.ProductGuid, &mut into[64..64 + 16]);
        FromIntoMemory::into_bytes(self.NameGuid, &mut into[80..80 + 16]);
    }
    fn size() -> usize {
        96
    }
}
pub struct WAVEINCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub ManufacturerGuid: crate::core::GUID,
    pub ProductGuid: crate::core::GUID,
    pub NameGuid: crate::core::GUID,
}
impl ::core::marker::Copy for WAVEINCAPS2W {}
impl ::core::clone::Clone for WAVEINCAPS2W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WAVEINCAPS2W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WAVEINCAPS2W")
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("vDriverVersion", &self.vDriverVersion)
            .field("szPname", &self.szPname)
            .field("dwFormats", &self.dwFormats)
            .field("wChannels", &self.wChannels)
            .field("wReserved1", &self.wReserved1)
            .field("ManufacturerGuid", &self.ManufacturerGuid)
            .field("ProductGuid", &self.ProductGuid)
            .field("NameGuid", &self.NameGuid)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WAVEINCAPS2W {
    fn eq(&self, other: &Self) -> bool {
        self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.vDriverVersion == other.vDriverVersion
            && self.szPname == other.szPname
            && self.dwFormats == other.dwFormats
            && self.wChannels == other.wChannels
            && self.wReserved1 == other.wReserved1
            && self.ManufacturerGuid == other.ManufacturerGuid
            && self.ProductGuid == other.ProductGuid
            && self.NameGuid == other.NameGuid
    }
}
impl ::core::cmp::Eq for WAVEINCAPS2W {}
impl FromIntoMemory for WAVEINCAPS2W {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 96);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_vDriverVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szPname = <[u16; 32] as FromIntoMemory>::from_bytes(&from[8..8 + 32]);
        let f_dwFormats = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_wChannels = <u16 as FromIntoMemory>::from_bytes(&from[44..44 + 2]);
        let f_wReserved1 = <u16 as FromIntoMemory>::from_bytes(&from[46..46 + 2]);
        let f_ManufacturerGuid =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[48..48 + 16]);
        let f_ProductGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[64..64 + 16]);
        let f_NameGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[80..80 + 16]);
        Self {
            wMid: f_wMid,
            wPid: f_wPid,
            vDriverVersion: f_vDriverVersion,
            szPname: f_szPname,
            dwFormats: f_dwFormats,
            wChannels: f_wChannels,
            wReserved1: f_wReserved1,
            ManufacturerGuid: f_ManufacturerGuid,
            ProductGuid: f_ProductGuid,
            NameGuid: f_NameGuid,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 96);
        FromIntoMemory::into_bytes(self.wMid, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.vDriverVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[8..8 + 32]);
        FromIntoMemory::into_bytes(self.dwFormats, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.wChannels, &mut into[44..44 + 2]);
        FromIntoMemory::into_bytes(self.wReserved1, &mut into[46..46 + 2]);
        FromIntoMemory::into_bytes(self.ManufacturerGuid, &mut into[48..48 + 16]);
        FromIntoMemory::into_bytes(self.ProductGuid, &mut into[64..64 + 16]);
        FromIntoMemory::into_bytes(self.NameGuid, &mut into[80..80 + 16]);
    }
    fn size() -> usize {
        96
    }
}
pub struct WAVEINCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
}
impl ::core::marker::Copy for WAVEINCAPSA {}
impl ::core::clone::Clone for WAVEINCAPSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WAVEINCAPSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WAVEINCAPSA")
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("vDriverVersion", &self.vDriverVersion)
            .field("szPname", &self.szPname)
            .field("dwFormats", &self.dwFormats)
            .field("wChannels", &self.wChannels)
            .field("wReserved1", &self.wReserved1)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WAVEINCAPSA {
    fn eq(&self, other: &Self) -> bool {
        self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.vDriverVersion == other.vDriverVersion
            && self.szPname == other.szPname
            && self.dwFormats == other.dwFormats
            && self.wChannels == other.wChannels
            && self.wReserved1 == other.wReserved1
    }
}
impl ::core::cmp::Eq for WAVEINCAPSA {}
impl FromIntoMemory for WAVEINCAPSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_vDriverVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szPname =
            <[super::super::Foundation::CHAR; 32] as FromIntoMemory>::from_bytes(&from[8..8 + 32]);
        let f_dwFormats = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_wChannels = <u16 as FromIntoMemory>::from_bytes(&from[44..44 + 2]);
        let f_wReserved1 = <u16 as FromIntoMemory>::from_bytes(&from[46..46 + 2]);
        Self {
            wMid: f_wMid,
            wPid: f_wPid,
            vDriverVersion: f_vDriverVersion,
            szPname: f_szPname,
            dwFormats: f_dwFormats,
            wChannels: f_wChannels,
            wReserved1: f_wReserved1,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.wMid, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.vDriverVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[8..8 + 32]);
        FromIntoMemory::into_bytes(self.dwFormats, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.wChannels, &mut into[44..44 + 2]);
        FromIntoMemory::into_bytes(self.wReserved1, &mut into[46..46 + 2]);
    }
    fn size() -> usize {
        48
    }
}
pub struct WAVEINCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
}
impl ::core::marker::Copy for WAVEINCAPSW {}
impl ::core::clone::Clone for WAVEINCAPSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WAVEINCAPSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WAVEINCAPSW")
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("vDriverVersion", &self.vDriverVersion)
            .field("szPname", &self.szPname)
            .field("dwFormats", &self.dwFormats)
            .field("wChannels", &self.wChannels)
            .field("wReserved1", &self.wReserved1)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WAVEINCAPSW {
    fn eq(&self, other: &Self) -> bool {
        self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.vDriverVersion == other.vDriverVersion
            && self.szPname == other.szPname
            && self.dwFormats == other.dwFormats
            && self.wChannels == other.wChannels
            && self.wReserved1 == other.wReserved1
    }
}
impl ::core::cmp::Eq for WAVEINCAPSW {}
impl FromIntoMemory for WAVEINCAPSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_vDriverVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szPname = <[u16; 32] as FromIntoMemory>::from_bytes(&from[8..8 + 32]);
        let f_dwFormats = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_wChannels = <u16 as FromIntoMemory>::from_bytes(&from[44..44 + 2]);
        let f_wReserved1 = <u16 as FromIntoMemory>::from_bytes(&from[46..46 + 2]);
        Self {
            wMid: f_wMid,
            wPid: f_wPid,
            vDriverVersion: f_vDriverVersion,
            szPname: f_szPname,
            dwFormats: f_dwFormats,
            wChannels: f_wChannels,
            wReserved1: f_wReserved1,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.wMid, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.vDriverVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[8..8 + 32]);
        FromIntoMemory::into_bytes(self.dwFormats, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.wChannels, &mut into[44..44 + 2]);
        FromIntoMemory::into_bytes(self.wReserved1, &mut into[46..46 + 2]);
    }
    fn size() -> usize {
        48
    }
}
pub const WAVEIN_MAPPER_STATUS_DEVICE: u32 = 0u32;
pub const WAVEIN_MAPPER_STATUS_FORMAT: u32 = 2u32;
pub const WAVEIN_MAPPER_STATUS_MAPPED: u32 = 1u32;
pub struct WAVEOUTCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: crate::core::GUID,
    pub ProductGuid: crate::core::GUID,
    pub NameGuid: crate::core::GUID,
}
impl ::core::marker::Copy for WAVEOUTCAPS2A {}
impl ::core::clone::Clone for WAVEOUTCAPS2A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WAVEOUTCAPS2A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WAVEOUTCAPS2A")
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("vDriverVersion", &self.vDriverVersion)
            .field("szPname", &self.szPname)
            .field("dwFormats", &self.dwFormats)
            .field("wChannels", &self.wChannels)
            .field("wReserved1", &self.wReserved1)
            .field("dwSupport", &self.dwSupport)
            .field("ManufacturerGuid", &self.ManufacturerGuid)
            .field("ProductGuid", &self.ProductGuid)
            .field("NameGuid", &self.NameGuid)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WAVEOUTCAPS2A {
    fn eq(&self, other: &Self) -> bool {
        self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.vDriverVersion == other.vDriverVersion
            && self.szPname == other.szPname
            && self.dwFormats == other.dwFormats
            && self.wChannels == other.wChannels
            && self.wReserved1 == other.wReserved1
            && self.dwSupport == other.dwSupport
            && self.ManufacturerGuid == other.ManufacturerGuid
            && self.ProductGuid == other.ProductGuid
            && self.NameGuid == other.NameGuid
    }
}
impl ::core::cmp::Eq for WAVEOUTCAPS2A {}
impl FromIntoMemory for WAVEOUTCAPS2A {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 100);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_vDriverVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szPname =
            <[super::super::Foundation::CHAR; 32] as FromIntoMemory>::from_bytes(&from[8..8 + 32]);
        let f_dwFormats = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_wChannels = <u16 as FromIntoMemory>::from_bytes(&from[44..44 + 2]);
        let f_wReserved1 = <u16 as FromIntoMemory>::from_bytes(&from[46..46 + 2]);
        let f_dwSupport = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_ManufacturerGuid =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[52..52 + 16]);
        let f_ProductGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[68..68 + 16]);
        let f_NameGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[84..84 + 16]);
        Self {
            wMid: f_wMid,
            wPid: f_wPid,
            vDriverVersion: f_vDriverVersion,
            szPname: f_szPname,
            dwFormats: f_dwFormats,
            wChannels: f_wChannels,
            wReserved1: f_wReserved1,
            dwSupport: f_dwSupport,
            ManufacturerGuid: f_ManufacturerGuid,
            ProductGuid: f_ProductGuid,
            NameGuid: f_NameGuid,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 100);
        FromIntoMemory::into_bytes(self.wMid, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.vDriverVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[8..8 + 32]);
        FromIntoMemory::into_bytes(self.dwFormats, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.wChannels, &mut into[44..44 + 2]);
        FromIntoMemory::into_bytes(self.wReserved1, &mut into[46..46 + 2]);
        FromIntoMemory::into_bytes(self.dwSupport, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.ManufacturerGuid, &mut into[52..52 + 16]);
        FromIntoMemory::into_bytes(self.ProductGuid, &mut into[68..68 + 16]);
        FromIntoMemory::into_bytes(self.NameGuid, &mut into[84..84 + 16]);
    }
    fn size() -> usize {
        100
    }
}
pub struct WAVEOUTCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: crate::core::GUID,
    pub ProductGuid: crate::core::GUID,
    pub NameGuid: crate::core::GUID,
}
impl ::core::marker::Copy for WAVEOUTCAPS2W {}
impl ::core::clone::Clone for WAVEOUTCAPS2W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WAVEOUTCAPS2W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WAVEOUTCAPS2W")
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("vDriverVersion", &self.vDriverVersion)
            .field("szPname", &self.szPname)
            .field("dwFormats", &self.dwFormats)
            .field("wChannels", &self.wChannels)
            .field("wReserved1", &self.wReserved1)
            .field("dwSupport", &self.dwSupport)
            .field("ManufacturerGuid", &self.ManufacturerGuid)
            .field("ProductGuid", &self.ProductGuid)
            .field("NameGuid", &self.NameGuid)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WAVEOUTCAPS2W {
    fn eq(&self, other: &Self) -> bool {
        self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.vDriverVersion == other.vDriverVersion
            && self.szPname == other.szPname
            && self.dwFormats == other.dwFormats
            && self.wChannels == other.wChannels
            && self.wReserved1 == other.wReserved1
            && self.dwSupport == other.dwSupport
            && self.ManufacturerGuid == other.ManufacturerGuid
            && self.ProductGuid == other.ProductGuid
            && self.NameGuid == other.NameGuid
    }
}
impl ::core::cmp::Eq for WAVEOUTCAPS2W {}
impl FromIntoMemory for WAVEOUTCAPS2W {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 100);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_vDriverVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szPname = <[u16; 32] as FromIntoMemory>::from_bytes(&from[8..8 + 32]);
        let f_dwFormats = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_wChannels = <u16 as FromIntoMemory>::from_bytes(&from[44..44 + 2]);
        let f_wReserved1 = <u16 as FromIntoMemory>::from_bytes(&from[46..46 + 2]);
        let f_dwSupport = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_ManufacturerGuid =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[52..52 + 16]);
        let f_ProductGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[68..68 + 16]);
        let f_NameGuid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[84..84 + 16]);
        Self {
            wMid: f_wMid,
            wPid: f_wPid,
            vDriverVersion: f_vDriverVersion,
            szPname: f_szPname,
            dwFormats: f_dwFormats,
            wChannels: f_wChannels,
            wReserved1: f_wReserved1,
            dwSupport: f_dwSupport,
            ManufacturerGuid: f_ManufacturerGuid,
            ProductGuid: f_ProductGuid,
            NameGuid: f_NameGuid,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 100);
        FromIntoMemory::into_bytes(self.wMid, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.vDriverVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[8..8 + 32]);
        FromIntoMemory::into_bytes(self.dwFormats, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.wChannels, &mut into[44..44 + 2]);
        FromIntoMemory::into_bytes(self.wReserved1, &mut into[46..46 + 2]);
        FromIntoMemory::into_bytes(self.dwSupport, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.ManufacturerGuid, &mut into[52..52 + 16]);
        FromIntoMemory::into_bytes(self.ProductGuid, &mut into[68..68 + 16]);
        FromIntoMemory::into_bytes(self.NameGuid, &mut into[84..84 + 16]);
    }
    fn size() -> usize {
        100
    }
}
pub struct WAVEOUTCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
}
impl ::core::marker::Copy for WAVEOUTCAPSA {}
impl ::core::clone::Clone for WAVEOUTCAPSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WAVEOUTCAPSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WAVEOUTCAPSA")
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("vDriverVersion", &self.vDriverVersion)
            .field("szPname", &self.szPname)
            .field("dwFormats", &self.dwFormats)
            .field("wChannels", &self.wChannels)
            .field("wReserved1", &self.wReserved1)
            .field("dwSupport", &self.dwSupport)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WAVEOUTCAPSA {
    fn eq(&self, other: &Self) -> bool {
        self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.vDriverVersion == other.vDriverVersion
            && self.szPname == other.szPname
            && self.dwFormats == other.dwFormats
            && self.wChannels == other.wChannels
            && self.wReserved1 == other.wReserved1
            && self.dwSupport == other.dwSupport
    }
}
impl ::core::cmp::Eq for WAVEOUTCAPSA {}
impl FromIntoMemory for WAVEOUTCAPSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 52);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_vDriverVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szPname =
            <[super::super::Foundation::CHAR; 32] as FromIntoMemory>::from_bytes(&from[8..8 + 32]);
        let f_dwFormats = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_wChannels = <u16 as FromIntoMemory>::from_bytes(&from[44..44 + 2]);
        let f_wReserved1 = <u16 as FromIntoMemory>::from_bytes(&from[46..46 + 2]);
        let f_dwSupport = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        Self {
            wMid: f_wMid,
            wPid: f_wPid,
            vDriverVersion: f_vDriverVersion,
            szPname: f_szPname,
            dwFormats: f_dwFormats,
            wChannels: f_wChannels,
            wReserved1: f_wReserved1,
            dwSupport: f_dwSupport,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 52);
        FromIntoMemory::into_bytes(self.wMid, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.vDriverVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[8..8 + 32]);
        FromIntoMemory::into_bytes(self.dwFormats, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.wChannels, &mut into[44..44 + 2]);
        FromIntoMemory::into_bytes(self.wReserved1, &mut into[46..46 + 2]);
        FromIntoMemory::into_bytes(self.dwSupport, &mut into[48..48 + 4]);
    }
    fn size() -> usize {
        52
    }
}
pub struct WAVEOUTCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
}
impl ::core::marker::Copy for WAVEOUTCAPSW {}
impl ::core::clone::Clone for WAVEOUTCAPSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WAVEOUTCAPSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WAVEOUTCAPSW")
            .field("wMid", &self.wMid)
            .field("wPid", &self.wPid)
            .field("vDriverVersion", &self.vDriverVersion)
            .field("szPname", &self.szPname)
            .field("dwFormats", &self.dwFormats)
            .field("wChannels", &self.wChannels)
            .field("wReserved1", &self.wReserved1)
            .field("dwSupport", &self.dwSupport)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WAVEOUTCAPSW {
    fn eq(&self, other: &Self) -> bool {
        self.wMid == other.wMid
            && self.wPid == other.wPid
            && self.vDriverVersion == other.vDriverVersion
            && self.szPname == other.szPname
            && self.dwFormats == other.dwFormats
            && self.wChannels == other.wChannels
            && self.wReserved1 == other.wReserved1
            && self.dwSupport == other.dwSupport
    }
}
impl ::core::cmp::Eq for WAVEOUTCAPSW {}
impl FromIntoMemory for WAVEOUTCAPSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 52);
        let f_wMid = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wPid = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_vDriverVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_szPname = <[u16; 32] as FromIntoMemory>::from_bytes(&from[8..8 + 32]);
        let f_dwFormats = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_wChannels = <u16 as FromIntoMemory>::from_bytes(&from[44..44 + 2]);
        let f_wReserved1 = <u16 as FromIntoMemory>::from_bytes(&from[46..46 + 2]);
        let f_dwSupport = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        Self {
            wMid: f_wMid,
            wPid: f_wPid,
            vDriverVersion: f_vDriverVersion,
            szPname: f_szPname,
            dwFormats: f_dwFormats,
            wChannels: f_wChannels,
            wReserved1: f_wReserved1,
            dwSupport: f_dwSupport,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 52);
        FromIntoMemory::into_bytes(self.wMid, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wPid, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.vDriverVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.szPname, &mut into[8..8 + 32]);
        FromIntoMemory::into_bytes(self.dwFormats, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.wChannels, &mut into[44..44 + 2]);
        FromIntoMemory::into_bytes(self.wReserved1, &mut into[46..46 + 2]);
        FromIntoMemory::into_bytes(self.dwSupport, &mut into[48..48 + 4]);
    }
    fn size() -> usize {
        52
    }
}
pub const WAVEOUT_MAPPER_STATUS_DEVICE: u32 = 0u32;
pub const WAVEOUT_MAPPER_STATUS_FORMAT: u32 = 2u32;
pub const WAVEOUT_MAPPER_STATUS_MAPPED: u32 = 1u32;
pub const WAVERR_BADFORMAT: u32 = 32u32;
pub const WAVERR_LASTERROR: u32 = 35u32;
pub const WAVERR_STILLPLAYING: u32 = 33u32;
pub const WAVERR_SYNC: u32 = 35u32;
pub const WAVERR_UNPREPARED: u32 = 34u32;
pub const WAVE_FORMAT_1M08: u32 = 1u32;
pub const WAVE_FORMAT_1M16: u32 = 4u32;
pub const WAVE_FORMAT_1S08: u32 = 2u32;
pub const WAVE_FORMAT_1S16: u32 = 8u32;
pub const WAVE_FORMAT_2M08: u32 = 16u32;
pub const WAVE_FORMAT_2M16: u32 = 64u32;
pub const WAVE_FORMAT_2S08: u32 = 32u32;
pub const WAVE_FORMAT_2S16: u32 = 128u32;
pub const WAVE_FORMAT_44M08: u32 = 256u32;
pub const WAVE_FORMAT_44M16: u32 = 1024u32;
pub const WAVE_FORMAT_44S08: u32 = 512u32;
pub const WAVE_FORMAT_44S16: u32 = 2048u32;
pub const WAVE_FORMAT_48M08: u32 = 4096u32;
pub const WAVE_FORMAT_48M16: u32 = 16384u32;
pub const WAVE_FORMAT_48S08: u32 = 8192u32;
pub const WAVE_FORMAT_48S16: u32 = 32768u32;
pub const WAVE_FORMAT_4M08: u32 = 256u32;
pub const WAVE_FORMAT_4M16: u32 = 1024u32;
pub const WAVE_FORMAT_4S08: u32 = 512u32;
pub const WAVE_FORMAT_4S16: u32 = 2048u32;
pub const WAVE_FORMAT_96M08: u32 = 65536u32;
pub const WAVE_FORMAT_96M16: u32 = 262144u32;
pub const WAVE_FORMAT_96S08: u32 = 131072u32;
pub const WAVE_FORMAT_96S16: u32 = 524288u32;
pub const WAVE_FORMAT_PCM: u32 = 1u32;
pub const WAVE_INVALIDFORMAT: u32 = 0u32;
pub const WAVE_MAPPER: u32 = 4294967295u32;
pub const WHDR_BEGINLOOP: u32 = 4u32;
pub const WHDR_DONE: u32 = 1u32;
pub const WHDR_ENDLOOP: u32 = 8u32;
pub const WHDR_INQUEUE: u32 = 16u32;
pub const WHDR_PREPARED: u32 = 2u32;
pub const WIDM_MAPPER_STATUS: u32 = 8192u32;
pub const WODM_MAPPER_STATUS: u32 = 8192u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct _AUDCLNT_BUFFERFLAGS(pub i32);
pub const AUDCLNT_BUFFERFLAGS_DATA_DISCONTINUITY: _AUDCLNT_BUFFERFLAGS = _AUDCLNT_BUFFERFLAGS(1i32);
pub const AUDCLNT_BUFFERFLAGS_SILENT: _AUDCLNT_BUFFERFLAGS = _AUDCLNT_BUFFERFLAGS(2i32);
pub const AUDCLNT_BUFFERFLAGS_TIMESTAMP_ERROR: _AUDCLNT_BUFFERFLAGS = _AUDCLNT_BUFFERFLAGS(4i32);
impl ::core::marker::Copy for _AUDCLNT_BUFFERFLAGS {}
impl ::core::clone::Clone for _AUDCLNT_BUFFERFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _AUDCLNT_BUFFERFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _AUDCLNT_BUFFERFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_AUDCLNT_BUFFERFLAGS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for _AUDCLNT_BUFFERFLAGS {
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
pub struct __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002(pub i32);
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_DEFAULT:
    __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 =
    __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002(0i32);
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_USER:
    __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 =
    __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002(1i32);
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_VOLATILE:
    __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 =
    __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002(2i32);
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_ENUM_COUNT:
    __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 =
    __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002(3i32);
impl ::core::marker::Copy for __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 {}
impl ::core::clone::Clone for __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("__MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 {
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
pub struct tACMDRVOPENDESCA {
    pub cbStruct: u32,
    pub fccType: u32,
    pub fccComp: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub dwError: u32,
    pub pszSectionName: PCSTR,
    pub pszAliasName: PCSTR,
    pub dnDevNode: u32,
}
impl ::core::marker::Copy for tACMDRVOPENDESCA {}
impl ::core::clone::Clone for tACMDRVOPENDESCA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for tACMDRVOPENDESCA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("tACMDRVOPENDESCA")
            .field("cbStruct", &self.cbStruct)
            .field("fccType", &self.fccType)
            .field("fccComp", &self.fccComp)
            .field("dwVersion", &self.dwVersion)
            .field("dwFlags", &self.dwFlags)
            .field("dwError", &self.dwError)
            .field("pszSectionName", &self.pszSectionName)
            .field("pszAliasName", &self.pszAliasName)
            .field("dnDevNode", &self.dnDevNode)
            .finish()
    }
}
impl ::core::cmp::PartialEq for tACMDRVOPENDESCA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.fccType == other.fccType
            && self.fccComp == other.fccComp
            && self.dwVersion == other.dwVersion
            && self.dwFlags == other.dwFlags
            && self.dwError == other.dwError
            && self.pszSectionName == other.pszSectionName
            && self.pszAliasName == other.pszAliasName
            && self.dnDevNode == other.dnDevNode
    }
}
impl ::core::cmp::Eq for tACMDRVOPENDESCA {}
impl FromIntoMemory for tACMDRVOPENDESCA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 36);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_fccType = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_fccComp = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwVersion = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwError = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_pszSectionName = <PCSTR as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_pszAliasName = <PCSTR as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_dnDevNode = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        Self {
            cbStruct: f_cbStruct,
            fccType: f_fccType,
            fccComp: f_fccComp,
            dwVersion: f_dwVersion,
            dwFlags: f_dwFlags,
            dwError: f_dwError,
            pszSectionName: f_pszSectionName,
            pszAliasName: f_pszAliasName,
            dnDevNode: f_dnDevNode,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 36);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.fccType, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.fccComp, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwVersion, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwError, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.pszSectionName, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.pszAliasName, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.dnDevNode, &mut into[32..32 + 4]);
    }
    fn size() -> usize {
        36
    }
}
pub struct tACMDRVOPENDESCW {
    pub cbStruct: u32,
    pub fccType: u32,
    pub fccComp: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub dwError: u32,
    pub pszSectionName: PCWSTR,
    pub pszAliasName: PCWSTR,
    pub dnDevNode: u32,
}
impl ::core::marker::Copy for tACMDRVOPENDESCW {}
impl ::core::clone::Clone for tACMDRVOPENDESCW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for tACMDRVOPENDESCW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("tACMDRVOPENDESCW")
            .field("cbStruct", &self.cbStruct)
            .field("fccType", &self.fccType)
            .field("fccComp", &self.fccComp)
            .field("dwVersion", &self.dwVersion)
            .field("dwFlags", &self.dwFlags)
            .field("dwError", &self.dwError)
            .field("pszSectionName", &self.pszSectionName)
            .field("pszAliasName", &self.pszAliasName)
            .field("dnDevNode", &self.dnDevNode)
            .finish()
    }
}
impl ::core::cmp::PartialEq for tACMDRVOPENDESCW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.fccType == other.fccType
            && self.fccComp == other.fccComp
            && self.dwVersion == other.dwVersion
            && self.dwFlags == other.dwFlags
            && self.dwError == other.dwError
            && self.pszSectionName == other.pszSectionName
            && self.pszAliasName == other.pszAliasName
            && self.dnDevNode == other.dnDevNode
    }
}
impl ::core::cmp::Eq for tACMDRVOPENDESCW {}
impl FromIntoMemory for tACMDRVOPENDESCW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 36);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_fccType = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_fccComp = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwVersion = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwError = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_pszSectionName = <PCWSTR as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_pszAliasName = <PCWSTR as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_dnDevNode = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        Self {
            cbStruct: f_cbStruct,
            fccType: f_fccType,
            fccComp: f_fccComp,
            dwVersion: f_dwVersion,
            dwFlags: f_dwFlags,
            dwError: f_dwError,
            pszSectionName: f_pszSectionName,
            pszAliasName: f_pszAliasName,
            dnDevNode: f_dnDevNode,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 36);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.fccType, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.fccComp, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwVersion, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwError, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.pszSectionName, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.pszAliasName, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.dnDevNode, &mut into[32..32 + 4]);
    }
    fn size() -> usize {
        36
    }
}
pub struct tACMFORMATDETAILSW {
    pub cbStruct: u32,
    pub dwFormatIndex: u32,
    pub dwFormatTag: u32,
    pub fdwSupport: u32,
    pub pwfx: MutPtr<WAVEFORMATEX>,
    pub cbwfx: u32,
    pub szFormat: [u16; 128],
}
impl ::core::marker::Copy for tACMFORMATDETAILSW {}
impl ::core::clone::Clone for tACMFORMATDETAILSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for tACMFORMATDETAILSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("tACMFORMATDETAILSW")
            .field("cbStruct", &self.cbStruct)
            .field("dwFormatIndex", &self.dwFormatIndex)
            .field("dwFormatTag", &self.dwFormatTag)
            .field("fdwSupport", &self.fdwSupport)
            .field("pwfx", &self.pwfx)
            .field("cbwfx", &self.cbwfx)
            .field("szFormat", &self.szFormat)
            .finish()
    }
}
impl ::core::cmp::PartialEq for tACMFORMATDETAILSW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwFormatIndex == other.dwFormatIndex
            && self.dwFormatTag == other.dwFormatTag
            && self.fdwSupport == other.fdwSupport
            && self.pwfx == other.pwfx
            && self.cbwfx == other.cbwfx
            && self.szFormat == other.szFormat
    }
}
impl ::core::cmp::Eq for tACMFORMATDETAILSW {}
impl FromIntoMemory for tACMFORMATDETAILSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 152);
        let f_cbStruct = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFormatIndex = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwFormatTag = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_fdwSupport = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_pwfx = <MutPtr<WAVEFORMATEX> as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_cbwfx = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_szFormat = <[u16; 128] as FromIntoMemory>::from_bytes(&from[24..24 + 128]);
        Self {
            cbStruct: f_cbStruct,
            dwFormatIndex: f_dwFormatIndex,
            dwFormatTag: f_dwFormatTag,
            fdwSupport: f_fdwSupport,
            pwfx: f_pwfx,
            cbwfx: f_cbwfx,
            szFormat: f_szFormat,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 152);
        FromIntoMemory::into_bytes(self.cbStruct, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFormatIndex, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwFormatTag, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.fdwSupport, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.pwfx, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.cbwfx, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.szFormat, &mut into[24..24 + 128]);
    }
    fn size() -> usize {
        152
    }
}
pub trait Api {
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage', 'Windows.Win32.System.Ole'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn ActivateAudioInterfaceAsync(
        &self,
        device_interface_path: PCWSTR,
        riid: ConstPtr<crate::core::GUID>,
        activation_params: ConstPtr<super::super::System::Com::StructuredStorage::PROPVARIANT>,
        completion_handler: IActivateAudioInterfaceCompletionHandler,
        activation_operation: MutPtr<IActivateAudioInterfaceAsyncOperation>,
    ) -> crate::core::HRESULT {
        todo!("ActivateAudioInterfaceAsync")
    }
    fn CoRegisterMessageFilter(
        &self,
        lp_message_filter: IMessageFilter,
        lplp_message_filter: MutPtr<IMessageFilter>,
    ) -> crate::core::HRESULT {
        todo!("CoRegisterMessageFilter")
    }
    fn CreateCaptureAudioStateMonitor(
        &self,
        audio_state_monitor: MutPtr<IAudioStateMonitor>,
    ) -> crate::core::HRESULT {
        todo!("CreateCaptureAudioStateMonitor")
    }
    fn CreateCaptureAudioStateMonitorForCategory(
        &self,
        category: AUDIO_STREAM_CATEGORY,
        audio_state_monitor: MutPtr<IAudioStateMonitor>,
    ) -> crate::core::HRESULT {
        todo!("CreateCaptureAudioStateMonitorForCategory")
    }
    fn CreateCaptureAudioStateMonitorForCategoryAndDeviceId(
        &self,
        category: AUDIO_STREAM_CATEGORY,
        device_id: PCWSTR,
        audio_state_monitor: MutPtr<IAudioStateMonitor>,
    ) -> crate::core::HRESULT {
        todo!("CreateCaptureAudioStateMonitorForCategoryAndDeviceId")
    }
    fn CreateCaptureAudioStateMonitorForCategoryAndDeviceRole(
        &self,
        category: AUDIO_STREAM_CATEGORY,
        role: ERole,
        audio_state_monitor: MutPtr<IAudioStateMonitor>,
    ) -> crate::core::HRESULT {
        todo!("CreateCaptureAudioStateMonitorForCategoryAndDeviceRole")
    }
    fn CreateRenderAudioStateMonitor(
        &self,
        audio_state_monitor: MutPtr<IAudioStateMonitor>,
    ) -> crate::core::HRESULT {
        todo!("CreateRenderAudioStateMonitor")
    }
    fn CreateRenderAudioStateMonitorForCategory(
        &self,
        category: AUDIO_STREAM_CATEGORY,
        audio_state_monitor: MutPtr<IAudioStateMonitor>,
    ) -> crate::core::HRESULT {
        todo!("CreateRenderAudioStateMonitorForCategory")
    }
    fn CreateRenderAudioStateMonitorForCategoryAndDeviceId(
        &self,
        category: AUDIO_STREAM_CATEGORY,
        device_id: PCWSTR,
        audio_state_monitor: MutPtr<IAudioStateMonitor>,
    ) -> crate::core::HRESULT {
        todo!("CreateRenderAudioStateMonitorForCategoryAndDeviceId")
    }
    fn CreateRenderAudioStateMonitorForCategoryAndDeviceRole(
        &self,
        category: AUDIO_STREAM_CATEGORY,
        role: ERole,
        audio_state_monitor: MutPtr<IAudioStateMonitor>,
    ) -> crate::core::HRESULT {
        todo!("CreateRenderAudioStateMonitorForCategoryAndDeviceRole")
    }
    fn PlaySoundA(
        &self,
        psz_sound: PCSTR,
        hmod: super::super::Foundation::HINSTANCE,
        fdw_sound: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("PlaySoundA")
    }
    fn PlaySoundW(
        &self,
        psz_sound: PCWSTR,
        hmod: super::super::Foundation::HINSTANCE,
        fdw_sound: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("PlaySoundW")
    }
    fn acmDriverAddA(
        &self,
        phadid: MutPtr<PtrDiffRepr>,
        hinst_module: super::super::Foundation::HINSTANCE,
        l_param: super::super::Foundation::LPARAM,
        dw_priority: u32,
        fdw_add: u32,
    ) -> u32 {
        todo!("acmDriverAddA")
    }
    fn acmDriverAddW(
        &self,
        phadid: MutPtr<PtrDiffRepr>,
        hinst_module: super::super::Foundation::HINSTANCE,
        l_param: super::super::Foundation::LPARAM,
        dw_priority: u32,
        fdw_add: u32,
    ) -> u32 {
        todo!("acmDriverAddW")
    }
    fn acmDriverClose(&self, had: HACMDRIVER, fdw_close: u32) -> u32 {
        todo!("acmDriverClose")
    }
    fn acmDriverDetailsA(
        &self,
        hadid: HACMDRIVERID,
        padd: MutPtr<ACMDRIVERDETAILSA>,
        fdw_details: u32,
    ) -> u32 {
        todo!("acmDriverDetailsA")
    }
    fn acmDriverDetailsW(
        &self,
        hadid: HACMDRIVERID,
        padd: MutPtr<ACMDRIVERDETAILSW>,
        fdw_details: u32,
    ) -> u32 {
        todo!("acmDriverDetailsW")
    }
    fn acmDriverEnum(
        &self,
        fn_callback: ACMDRIVERENUMCB,
        dw_instance: PtrRepr,
        fdw_enum: u32,
    ) -> u32 {
        todo!("acmDriverEnum")
    }
    fn acmDriverID(&self, hao: HACMOBJ, phadid: MutPtr<PtrDiffRepr>, fdw_driver_id: u32) -> u32 {
        todo!("acmDriverID")
    }
    fn acmDriverMessage(
        &self,
        had: HACMDRIVER,
        u_msg: u32,
        l_param_1: super::super::Foundation::LPARAM,
        l_param_2: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::LRESULT {
        todo!("acmDriverMessage")
    }
    fn acmDriverOpen(&self, phad: MutPtr<PtrDiffRepr>, hadid: HACMDRIVERID, fdw_open: u32) -> u32 {
        todo!("acmDriverOpen")
    }
    fn acmDriverPriority(&self, hadid: HACMDRIVERID, dw_priority: u32, fdw_priority: u32) -> u32 {
        todo!("acmDriverPriority")
    }
    fn acmDriverRemove(&self, hadid: HACMDRIVERID, fdw_remove: u32) -> u32 {
        todo!("acmDriverRemove")
    }
    fn acmFilterChooseA(&self, pafltrc: MutPtr<ACMFILTERCHOOSEA>) -> u32 {
        todo!("acmFilterChooseA")
    }
    fn acmFilterChooseW(&self, pafltrc: MutPtr<ACMFILTERCHOOSEW>) -> u32 {
        todo!("acmFilterChooseW")
    }
    fn acmFilterDetailsA(
        &self,
        had: HACMDRIVER,
        pafd: MutPtr<ACMFILTERDETAILSA>,
        fdw_details: u32,
    ) -> u32 {
        todo!("acmFilterDetailsA")
    }
    fn acmFilterDetailsW(
        &self,
        had: HACMDRIVER,
        pafd: MutPtr<ACMFILTERDETAILSW>,
        fdw_details: u32,
    ) -> u32 {
        todo!("acmFilterDetailsW")
    }
    fn acmFilterEnumA(
        &self,
        had: HACMDRIVER,
        pafd: MutPtr<ACMFILTERDETAILSA>,
        fn_callback: ACMFILTERENUMCBA,
        dw_instance: PtrRepr,
        fdw_enum: u32,
    ) -> u32 {
        todo!("acmFilterEnumA")
    }
    fn acmFilterEnumW(
        &self,
        had: HACMDRIVER,
        pafd: MutPtr<ACMFILTERDETAILSW>,
        fn_callback: ACMFILTERENUMCBW,
        dw_instance: PtrRepr,
        fdw_enum: u32,
    ) -> u32 {
        todo!("acmFilterEnumW")
    }
    fn acmFilterTagDetailsA(
        &self,
        had: HACMDRIVER,
        paftd: MutPtr<ACMFILTERTAGDETAILSA>,
        fdw_details: u32,
    ) -> u32 {
        todo!("acmFilterTagDetailsA")
    }
    fn acmFilterTagDetailsW(
        &self,
        had: HACMDRIVER,
        paftd: MutPtr<ACMFILTERTAGDETAILSW>,
        fdw_details: u32,
    ) -> u32 {
        todo!("acmFilterTagDetailsW")
    }
    fn acmFilterTagEnumA(
        &self,
        had: HACMDRIVER,
        paftd: MutPtr<ACMFILTERTAGDETAILSA>,
        fn_callback: ACMFILTERTAGENUMCBA,
        dw_instance: PtrRepr,
        fdw_enum: u32,
    ) -> u32 {
        todo!("acmFilterTagEnumA")
    }
    fn acmFilterTagEnumW(
        &self,
        had: HACMDRIVER,
        paftd: MutPtr<ACMFILTERTAGDETAILSW>,
        fn_callback: ACMFILTERTAGENUMCBW,
        dw_instance: PtrRepr,
        fdw_enum: u32,
    ) -> u32 {
        todo!("acmFilterTagEnumW")
    }
    fn acmFormatChooseA(&self, pafmtc: MutPtr<ACMFORMATCHOOSEA>) -> u32 {
        todo!("acmFormatChooseA")
    }
    fn acmFormatChooseW(&self, pafmtc: MutPtr<ACMFORMATCHOOSEW>) -> u32 {
        todo!("acmFormatChooseW")
    }
    fn acmFormatDetailsA(
        &self,
        had: HACMDRIVER,
        pafd: MutPtr<ACMFORMATDETAILSA>,
        fdw_details: u32,
    ) -> u32 {
        todo!("acmFormatDetailsA")
    }
    fn acmFormatDetailsW(
        &self,
        had: HACMDRIVER,
        pafd: MutPtr<tACMFORMATDETAILSW>,
        fdw_details: u32,
    ) -> u32 {
        todo!("acmFormatDetailsW")
    }
    fn acmFormatEnumA(
        &self,
        had: HACMDRIVER,
        pafd: MutPtr<ACMFORMATDETAILSA>,
        fn_callback: ACMFORMATENUMCBA,
        dw_instance: PtrRepr,
        fdw_enum: u32,
    ) -> u32 {
        todo!("acmFormatEnumA")
    }
    fn acmFormatEnumW(
        &self,
        had: HACMDRIVER,
        pafd: MutPtr<tACMFORMATDETAILSW>,
        fn_callback: ACMFORMATENUMCBW,
        dw_instance: PtrRepr,
        fdw_enum: u32,
    ) -> u32 {
        todo!("acmFormatEnumW")
    }
    fn acmFormatSuggest(
        &self,
        had: HACMDRIVER,
        pwfx_src: MutPtr<WAVEFORMATEX>,
        pwfx_dst: MutPtr<WAVEFORMATEX>,
        cbwfx_dst: u32,
        fdw_suggest: u32,
    ) -> u32 {
        todo!("acmFormatSuggest")
    }
    fn acmFormatTagDetailsA(
        &self,
        had: HACMDRIVER,
        paftd: MutPtr<ACMFORMATTAGDETAILSA>,
        fdw_details: u32,
    ) -> u32 {
        todo!("acmFormatTagDetailsA")
    }
    fn acmFormatTagDetailsW(
        &self,
        had: HACMDRIVER,
        paftd: MutPtr<ACMFORMATTAGDETAILSW>,
        fdw_details: u32,
    ) -> u32 {
        todo!("acmFormatTagDetailsW")
    }
    fn acmFormatTagEnumA(
        &self,
        had: HACMDRIVER,
        paftd: MutPtr<ACMFORMATTAGDETAILSA>,
        fn_callback: ACMFORMATTAGENUMCBA,
        dw_instance: PtrRepr,
        fdw_enum: u32,
    ) -> u32 {
        todo!("acmFormatTagEnumA")
    }
    fn acmFormatTagEnumW(
        &self,
        had: HACMDRIVER,
        paftd: MutPtr<ACMFORMATTAGDETAILSW>,
        fn_callback: ACMFORMATTAGENUMCBW,
        dw_instance: PtrRepr,
        fdw_enum: u32,
    ) -> u32 {
        todo!("acmFormatTagEnumW")
    }
    fn acmGetVersion(&self) -> u32 {
        todo!("acmGetVersion")
    }
    fn acmMetrics(
        &self,
        hao: HACMOBJ,
        u_metric: u32,
        p_metric: MutPtr<::core::ffi::c_void>,
    ) -> u32 {
        todo!("acmMetrics")
    }
    fn acmStreamClose(&self, has: HACMSTREAM, fdw_close: u32) -> u32 {
        todo!("acmStreamClose")
    }
    fn acmStreamConvert(
        &self,
        has: HACMSTREAM,
        pash: MutPtr<ACMSTREAMHEADER>,
        fdw_convert: u32,
    ) -> u32 {
        todo!("acmStreamConvert")
    }
    fn acmStreamMessage(
        &self,
        has: HACMSTREAM,
        u_msg: u32,
        l_param_1: super::super::Foundation::LPARAM,
        l_param_2: super::super::Foundation::LPARAM,
    ) -> u32 {
        todo!("acmStreamMessage")
    }
    fn acmStreamOpen(
        &self,
        phas: MutPtr<PtrDiffRepr>,
        had: HACMDRIVER,
        pwfx_src: MutPtr<WAVEFORMATEX>,
        pwfx_dst: MutPtr<WAVEFORMATEX>,
        pwfltr: MutPtr<WAVEFILTER>,
        dw_callback: PtrRepr,
        dw_instance: PtrRepr,
        fdw_open: u32,
    ) -> u32 {
        todo!("acmStreamOpen")
    }
    fn acmStreamPrepareHeader(
        &self,
        has: HACMSTREAM,
        pash: MutPtr<ACMSTREAMHEADER>,
        fdw_prepare: u32,
    ) -> u32 {
        todo!("acmStreamPrepareHeader")
    }
    fn acmStreamReset(&self, has: HACMSTREAM, fdw_reset: u32) -> u32 {
        todo!("acmStreamReset")
    }
    fn acmStreamSize(
        &self,
        has: HACMSTREAM,
        cb_input: u32,
        pdw_output_bytes: MutPtr<u32>,
        fdw_size: u32,
    ) -> u32 {
        todo!("acmStreamSize")
    }
    fn acmStreamUnprepareHeader(
        &self,
        has: HACMSTREAM,
        pash: MutPtr<ACMSTREAMHEADER>,
        fdw_unprepare: u32,
    ) -> u32 {
        todo!("acmStreamUnprepareHeader")
    }
    fn auxGetDevCapsA(&self, u_device_id: PtrRepr, pac: MutPtr<AUXCAPSA>, cbac: u32) -> u32 {
        todo!("auxGetDevCapsA")
    }
    fn auxGetDevCapsW(&self, u_device_id: PtrRepr, pac: MutPtr<AUXCAPSW>, cbac: u32) -> u32 {
        todo!("auxGetDevCapsW")
    }
    fn auxGetNumDevs(&self) -> u32 {
        todo!("auxGetNumDevs")
    }
    fn auxGetVolume(&self, u_device_id: u32, pdw_volume: MutPtr<u32>) -> u32 {
        todo!("auxGetVolume")
    }
    fn auxOutMessage(&self, u_device_id: u32, u_msg: u32, dw_1: PtrRepr, dw_2: PtrRepr) -> u32 {
        todo!("auxOutMessage")
    }
    fn auxSetVolume(&self, u_device_id: u32, dw_volume: u32) -> u32 {
        todo!("auxSetVolume")
    }
    fn midiConnect(
        &self,
        hmi: HMIDI,
        hmo: HMIDIOUT,
        p_reserved: ConstPtr<::core::ffi::c_void>,
    ) -> u32 {
        todo!("midiConnect")
    }
    fn midiDisconnect(
        &self,
        hmi: HMIDI,
        hmo: HMIDIOUT,
        p_reserved: ConstPtr<::core::ffi::c_void>,
    ) -> u32 {
        todo!("midiDisconnect")
    }
    fn midiInAddBuffer(&self, hmi: HMIDIIN, pmh: MutPtr<MIDIHDR>, cbmh: u32) -> u32 {
        todo!("midiInAddBuffer")
    }
    fn midiInClose(&self, hmi: HMIDIIN) -> u32 {
        todo!("midiInClose")
    }
    fn midiInGetDevCapsA(
        &self,
        u_device_id: PtrRepr,
        pmic: MutPtr<MIDIINCAPSA>,
        cbmic: u32,
    ) -> u32 {
        todo!("midiInGetDevCapsA")
    }
    fn midiInGetDevCapsW(
        &self,
        u_device_id: PtrRepr,
        pmic: MutPtr<MIDIINCAPSW>,
        cbmic: u32,
    ) -> u32 {
        todo!("midiInGetDevCapsW")
    }
    fn midiInGetErrorTextA(&self, mmr_error: u32, psz_text: PSTR, cch_text: u32) -> u32 {
        todo!("midiInGetErrorTextA")
    }
    fn midiInGetErrorTextW(&self, mmr_error: u32, psz_text: PWSTR, cch_text: u32) -> u32 {
        todo!("midiInGetErrorTextW")
    }
    fn midiInGetID(&self, hmi: HMIDIIN, pu_device_id: MutPtr<u32>) -> u32 {
        todo!("midiInGetID")
    }
    fn midiInGetNumDevs(&self) -> u32 {
        todo!("midiInGetNumDevs")
    }
    fn midiInMessage(&self, hmi: HMIDIIN, u_msg: u32, dw_1: PtrRepr, dw_2: PtrRepr) -> u32 {
        todo!("midiInMessage")
    }
    fn midiInOpen(
        &self,
        phmi: MutPtr<HMIDIIN>,
        u_device_id: u32,
        dw_callback: PtrRepr,
        dw_instance: PtrRepr,
        fdw_open: MIDI_WAVE_OPEN_TYPE,
    ) -> u32 {
        todo!("midiInOpen")
    }
    fn midiInPrepareHeader(&self, hmi: HMIDIIN, pmh: MutPtr<MIDIHDR>, cbmh: u32) -> u32 {
        todo!("midiInPrepareHeader")
    }
    fn midiInReset(&self, hmi: HMIDIIN) -> u32 {
        todo!("midiInReset")
    }
    fn midiInStart(&self, hmi: HMIDIIN) -> u32 {
        todo!("midiInStart")
    }
    fn midiInStop(&self, hmi: HMIDIIN) -> u32 {
        todo!("midiInStop")
    }
    fn midiInUnprepareHeader(&self, hmi: HMIDIIN, pmh: MutPtr<MIDIHDR>, cbmh: u32) -> u32 {
        todo!("midiInUnprepareHeader")
    }
    fn midiOutCacheDrumPatches(
        &self,
        hmo: HMIDIOUT,
        u_patch: u32,
        pwkya: ConstPtr<u16>,
        fu_cache: u32,
    ) -> u32 {
        todo!("midiOutCacheDrumPatches")
    }
    fn midiOutCachePatches(
        &self,
        hmo: HMIDIOUT,
        u_bank: u32,
        pwpa: ConstPtr<u16>,
        fu_cache: u32,
    ) -> u32 {
        todo!("midiOutCachePatches")
    }
    fn midiOutClose(&self, hmo: HMIDIOUT) -> u32 {
        todo!("midiOutClose")
    }
    fn midiOutGetDevCapsA(
        &self,
        u_device_id: PtrRepr,
        pmoc: MutPtr<MIDIOUTCAPSA>,
        cbmoc: u32,
    ) -> u32 {
        todo!("midiOutGetDevCapsA")
    }
    fn midiOutGetDevCapsW(
        &self,
        u_device_id: PtrRepr,
        pmoc: MutPtr<MIDIOUTCAPSW>,
        cbmoc: u32,
    ) -> u32 {
        todo!("midiOutGetDevCapsW")
    }
    fn midiOutGetErrorTextA(&self, mmr_error: u32, psz_text: PSTR, cch_text: u32) -> u32 {
        todo!("midiOutGetErrorTextA")
    }
    fn midiOutGetErrorTextW(&self, mmr_error: u32, psz_text: PWSTR, cch_text: u32) -> u32 {
        todo!("midiOutGetErrorTextW")
    }
    fn midiOutGetID(&self, hmo: HMIDIOUT, pu_device_id: MutPtr<u32>) -> u32 {
        todo!("midiOutGetID")
    }
    fn midiOutGetNumDevs(&self) -> u32 {
        todo!("midiOutGetNumDevs")
    }
    fn midiOutGetVolume(&self, hmo: HMIDIOUT, pdw_volume: MutPtr<u32>) -> u32 {
        todo!("midiOutGetVolume")
    }
    fn midiOutLongMsg(&self, hmo: HMIDIOUT, pmh: ConstPtr<MIDIHDR>, cbmh: u32) -> u32 {
        todo!("midiOutLongMsg")
    }
    fn midiOutMessage(&self, hmo: HMIDIOUT, u_msg: u32, dw_1: PtrRepr, dw_2: PtrRepr) -> u32 {
        todo!("midiOutMessage")
    }
    fn midiOutOpen(
        &self,
        phmo: MutPtr<HMIDIOUT>,
        u_device_id: u32,
        dw_callback: PtrRepr,
        dw_instance: PtrRepr,
        fdw_open: MIDI_WAVE_OPEN_TYPE,
    ) -> u32 {
        todo!("midiOutOpen")
    }
    fn midiOutPrepareHeader(&self, hmo: HMIDIOUT, pmh: MutPtr<MIDIHDR>, cbmh: u32) -> u32 {
        todo!("midiOutPrepareHeader")
    }
    fn midiOutReset(&self, hmo: HMIDIOUT) -> u32 {
        todo!("midiOutReset")
    }
    fn midiOutSetVolume(&self, hmo: HMIDIOUT, dw_volume: u32) -> u32 {
        todo!("midiOutSetVolume")
    }
    fn midiOutShortMsg(&self, hmo: HMIDIOUT, dw_msg: u32) -> u32 {
        todo!("midiOutShortMsg")
    }
    fn midiOutUnprepareHeader(&self, hmo: HMIDIOUT, pmh: MutPtr<MIDIHDR>, cbmh: u32) -> u32 {
        todo!("midiOutUnprepareHeader")
    }
    fn midiStreamClose(&self, hms: HMIDISTRM) -> u32 {
        todo!("midiStreamClose")
    }
    fn midiStreamOpen(
        &self,
        phms: MutPtr<HMIDISTRM>,
        pu_device_id: MutPtr<u32>,
        c_midi: u32,
        dw_callback: PtrRepr,
        dw_instance: PtrRepr,
        fdw_open: u32,
    ) -> u32 {
        todo!("midiStreamOpen")
    }
    fn midiStreamOut(&self, hms: HMIDISTRM, pmh: MutPtr<MIDIHDR>, cbmh: u32) -> u32 {
        todo!("midiStreamOut")
    }
    fn midiStreamPause(&self, hms: HMIDISTRM) -> u32 {
        todo!("midiStreamPause")
    }
    fn midiStreamPosition(&self, hms: HMIDISTRM, lpmmt: MutPtr<super::MMTIME>, cbmmt: u32) -> u32 {
        todo!("midiStreamPosition")
    }
    fn midiStreamProperty(&self, hms: HMIDISTRM, lppropdata: MutPtr<u8>, dw_property: u32) -> u32 {
        todo!("midiStreamProperty")
    }
    fn midiStreamRestart(&self, hms: HMIDISTRM) -> u32 {
        todo!("midiStreamRestart")
    }
    fn midiStreamStop(&self, hms: HMIDISTRM) -> u32 {
        todo!("midiStreamStop")
    }
    fn mixerClose(&self, hmx: HMIXER) -> u32 {
        todo!("mixerClose")
    }
    fn mixerGetControlDetailsA(
        &self,
        hmxobj: HMIXEROBJ,
        pmxcd: MutPtr<MIXERCONTROLDETAILS>,
        fdw_details: u32,
    ) -> u32 {
        todo!("mixerGetControlDetailsA")
    }
    fn mixerGetControlDetailsW(
        &self,
        hmxobj: HMIXEROBJ,
        pmxcd: MutPtr<MIXERCONTROLDETAILS>,
        fdw_details: u32,
    ) -> u32 {
        todo!("mixerGetControlDetailsW")
    }
    fn mixerGetDevCapsA(
        &self,
        u_mx_id: PtrRepr,
        pmxcaps: MutPtr<MIXERCAPSA>,
        cbmxcaps: u32,
    ) -> u32 {
        todo!("mixerGetDevCapsA")
    }
    fn mixerGetDevCapsW(
        &self,
        u_mx_id: PtrRepr,
        pmxcaps: MutPtr<MIXERCAPSW>,
        cbmxcaps: u32,
    ) -> u32 {
        todo!("mixerGetDevCapsW")
    }
    fn mixerGetID(&self, hmxobj: HMIXEROBJ, pu_mx_id: MutPtr<u32>, fdw_id: u32) -> u32 {
        todo!("mixerGetID")
    }
    fn mixerGetLineControlsA(
        &self,
        hmxobj: HMIXEROBJ,
        pmxlc: MutPtr<MIXERLINECONTROLSA>,
        fdw_controls: u32,
    ) -> u32 {
        todo!("mixerGetLineControlsA")
    }
    fn mixerGetLineControlsW(
        &self,
        hmxobj: HMIXEROBJ,
        pmxlc: MutPtr<MIXERLINECONTROLSW>,
        fdw_controls: u32,
    ) -> u32 {
        todo!("mixerGetLineControlsW")
    }
    fn mixerGetLineInfoA(&self, hmxobj: HMIXEROBJ, pmxl: MutPtr<MIXERLINEA>, fdw_info: u32) -> u32 {
        todo!("mixerGetLineInfoA")
    }
    fn mixerGetLineInfoW(&self, hmxobj: HMIXEROBJ, pmxl: MutPtr<MIXERLINEW>, fdw_info: u32) -> u32 {
        todo!("mixerGetLineInfoW")
    }
    fn mixerGetNumDevs(&self) -> u32 {
        todo!("mixerGetNumDevs")
    }
    fn mixerMessage(
        &self,
        hmx: HMIXER,
        u_msg: u32,
        dw_param_1: PtrRepr,
        dw_param_2: PtrRepr,
    ) -> u32 {
        todo!("mixerMessage")
    }
    fn mixerOpen(
        &self,
        phmx: MutPtr<PtrDiffRepr>,
        u_mx_id: u32,
        dw_callback: PtrRepr,
        dw_instance: PtrRepr,
        fdw_open: u32,
    ) -> u32 {
        todo!("mixerOpen")
    }
    fn mixerSetControlDetails(
        &self,
        hmxobj: HMIXEROBJ,
        pmxcd: ConstPtr<MIXERCONTROLDETAILS>,
        fdw_details: u32,
    ) -> u32 {
        todo!("mixerSetControlDetails")
    }
    fn sndPlaySoundA(&self, psz_sound: PCSTR, fu_sound: u32) -> super::super::Foundation::BOOL {
        todo!("sndPlaySoundA")
    }
    fn sndPlaySoundW(&self, psz_sound: PCWSTR, fu_sound: u32) -> super::super::Foundation::BOOL {
        todo!("sndPlaySoundW")
    }
    fn waveInAddBuffer(&self, hwi: HWAVEIN, pwh: MutPtr<WAVEHDR>, cbwh: u32) -> u32 {
        todo!("waveInAddBuffer")
    }
    fn waveInClose(&self, hwi: HWAVEIN) -> u32 {
        todo!("waveInClose")
    }
    fn waveInGetDevCapsA(
        &self,
        u_device_id: PtrRepr,
        pwic: MutPtr<WAVEINCAPSA>,
        cbwic: u32,
    ) -> u32 {
        todo!("waveInGetDevCapsA")
    }
    fn waveInGetDevCapsW(
        &self,
        u_device_id: PtrRepr,
        pwic: MutPtr<WAVEINCAPSW>,
        cbwic: u32,
    ) -> u32 {
        todo!("waveInGetDevCapsW")
    }
    fn waveInGetErrorTextA(&self, mmr_error: u32, psz_text: PSTR, cch_text: u32) -> u32 {
        todo!("waveInGetErrorTextA")
    }
    fn waveInGetErrorTextW(&self, mmr_error: u32, psz_text: PWSTR, cch_text: u32) -> u32 {
        todo!("waveInGetErrorTextW")
    }
    fn waveInGetID(&self, hwi: HWAVEIN, pu_device_id: ConstPtr<u32>) -> u32 {
        todo!("waveInGetID")
    }
    fn waveInGetNumDevs(&self) -> u32 {
        todo!("waveInGetNumDevs")
    }
    fn waveInGetPosition(&self, hwi: HWAVEIN, pmmt: MutPtr<super::MMTIME>, cbmmt: u32) -> u32 {
        todo!("waveInGetPosition")
    }
    fn waveInMessage(&self, hwi: HWAVEIN, u_msg: u32, dw_1: PtrRepr, dw_2: PtrRepr) -> u32 {
        todo!("waveInMessage")
    }
    fn waveInOpen(
        &self,
        phwi: MutPtr<HWAVEIN>,
        u_device_id: u32,
        pwfx: ConstPtr<WAVEFORMATEX>,
        dw_callback: PtrRepr,
        dw_instance: PtrRepr,
        fdw_open: MIDI_WAVE_OPEN_TYPE,
    ) -> u32 {
        todo!("waveInOpen")
    }
    fn waveInPrepareHeader(&self, hwi: HWAVEIN, pwh: MutPtr<WAVEHDR>, cbwh: u32) -> u32 {
        todo!("waveInPrepareHeader")
    }
    fn waveInReset(&self, hwi: HWAVEIN) -> u32 {
        todo!("waveInReset")
    }
    fn waveInStart(&self, hwi: HWAVEIN) -> u32 {
        todo!("waveInStart")
    }
    fn waveInStop(&self, hwi: HWAVEIN) -> u32 {
        todo!("waveInStop")
    }
    fn waveInUnprepareHeader(&self, hwi: HWAVEIN, pwh: MutPtr<WAVEHDR>, cbwh: u32) -> u32 {
        todo!("waveInUnprepareHeader")
    }
    fn waveOutBreakLoop(&self, hwo: HWAVEOUT) -> u32 {
        todo!("waveOutBreakLoop")
    }
    fn waveOutClose(&self, hwo: HWAVEOUT) -> u32 {
        todo!("waveOutClose")
    }
    fn waveOutGetDevCapsA(
        &self,
        u_device_id: PtrRepr,
        pwoc: MutPtr<WAVEOUTCAPSA>,
        cbwoc: u32,
    ) -> u32 {
        todo!("waveOutGetDevCapsA")
    }
    fn waveOutGetDevCapsW(
        &self,
        u_device_id: PtrRepr,
        pwoc: MutPtr<WAVEOUTCAPSW>,
        cbwoc: u32,
    ) -> u32 {
        todo!("waveOutGetDevCapsW")
    }
    fn waveOutGetErrorTextA(&self, mmr_error: u32, psz_text: PSTR, cch_text: u32) -> u32 {
        todo!("waveOutGetErrorTextA")
    }
    fn waveOutGetErrorTextW(&self, mmr_error: u32, psz_text: PWSTR, cch_text: u32) -> u32 {
        todo!("waveOutGetErrorTextW")
    }
    fn waveOutGetID(&self, hwo: HWAVEOUT, pu_device_id: MutPtr<u32>) -> u32 {
        todo!("waveOutGetID")
    }
    fn waveOutGetNumDevs(&self) -> u32 {
        todo!("waveOutGetNumDevs")
    }
    fn waveOutGetPitch(&self, hwo: HWAVEOUT, pdw_pitch: MutPtr<u32>) -> u32 {
        todo!("waveOutGetPitch")
    }
    fn waveOutGetPlaybackRate(&self, hwo: HWAVEOUT, pdw_rate: MutPtr<u32>) -> u32 {
        todo!("waveOutGetPlaybackRate")
    }
    fn waveOutGetPosition(&self, hwo: HWAVEOUT, pmmt: MutPtr<super::MMTIME>, cbmmt: u32) -> u32 {
        todo!("waveOutGetPosition")
    }
    fn waveOutGetVolume(&self, hwo: HWAVEOUT, pdw_volume: MutPtr<u32>) -> u32 {
        todo!("waveOutGetVolume")
    }
    fn waveOutMessage(&self, hwo: HWAVEOUT, u_msg: u32, dw_1: PtrRepr, dw_2: PtrRepr) -> u32 {
        todo!("waveOutMessage")
    }
    fn waveOutOpen(
        &self,
        phwo: MutPtr<HWAVEOUT>,
        u_device_id: u32,
        pwfx: ConstPtr<WAVEFORMATEX>,
        dw_callback: PtrRepr,
        dw_instance: PtrRepr,
        fdw_open: MIDI_WAVE_OPEN_TYPE,
    ) -> u32 {
        todo!("waveOutOpen")
    }
    fn waveOutPause(&self, hwo: HWAVEOUT) -> u32 {
        todo!("waveOutPause")
    }
    fn waveOutPrepareHeader(&self, hwo: HWAVEOUT, pwh: MutPtr<WAVEHDR>, cbwh: u32) -> u32 {
        todo!("waveOutPrepareHeader")
    }
    fn waveOutReset(&self, hwo: HWAVEOUT) -> u32 {
        todo!("waveOutReset")
    }
    fn waveOutRestart(&self, hwo: HWAVEOUT) -> u32 {
        todo!("waveOutRestart")
    }
    fn waveOutSetPitch(&self, hwo: HWAVEOUT, dw_pitch: u32) -> u32 {
        todo!("waveOutSetPitch")
    }
    fn waveOutSetPlaybackRate(&self, hwo: HWAVEOUT, dw_rate: u32) -> u32 {
        todo!("waveOutSetPlaybackRate")
    }
    fn waveOutSetVolume(&self, hwo: HWAVEOUT, dw_volume: u32) -> u32 {
        todo!("waveOutSetVolume")
    }
    fn waveOutUnprepareHeader(&self, hwo: HWAVEOUT, pwh: MutPtr<WAVEHDR>, cbwh: u32) -> u32 {
        todo!("waveOutUnprepareHeader")
    }
    fn waveOutWrite(&self, hwo: HWAVEOUT, pwh: MutPtr<WAVEHDR>, cbwh: u32) -> u32 {
        todo!("waveOutWrite")
    }
}
pub fn get_api(ctx: &crate::core::Win32Context) -> std::sync::Arc<dyn Api> {
    ctx.get::<dyn Api>()
}
