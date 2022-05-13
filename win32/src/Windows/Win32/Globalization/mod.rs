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
pub const ALL_SERVICES: u32 = 0u32;
pub const ALL_SERVICE_TYPES: u32 = 0u32;
pub const C1_ALPHA: u32 = 256u32;
pub const C1_BLANK: u32 = 64u32;
pub const C1_CNTRL: u32 = 32u32;
pub const C1_DEFINED: u32 = 512u32;
pub const C1_DIGIT: u32 = 4u32;
pub const C1_LOWER: u32 = 2u32;
pub const C1_PUNCT: u32 = 16u32;
pub const C1_SPACE: u32 = 8u32;
pub const C1_UPPER: u32 = 1u32;
pub const C1_XDIGIT: u32 = 128u32;
pub const C2_ARABICNUMBER: u32 = 6u32;
pub const C2_BLOCKSEPARATOR: u32 = 8u32;
pub const C2_COMMONSEPARATOR: u32 = 7u32;
pub const C2_EUROPENUMBER: u32 = 3u32;
pub const C2_EUROPESEPARATOR: u32 = 4u32;
pub const C2_EUROPETERMINATOR: u32 = 5u32;
pub const C2_LEFTTORIGHT: u32 = 1u32;
pub const C2_NOTAPPLICABLE: u32 = 0u32;
pub const C2_OTHERNEUTRAL: u32 = 11u32;
pub const C2_RIGHTTOLEFT: u32 = 2u32;
pub const C2_SEGMENTSEPARATOR: u32 = 9u32;
pub const C2_WHITESPACE: u32 = 10u32;
pub const C3_ALPHA: u32 = 32768u32;
pub const C3_DIACRITIC: u32 = 2u32;
pub const C3_FULLWIDTH: u32 = 128u32;
pub const C3_HALFWIDTH: u32 = 64u32;
pub const C3_HIGHSURROGATE: u32 = 2048u32;
pub const C3_HIRAGANA: u32 = 32u32;
pub const C3_IDEOGRAPH: u32 = 256u32;
pub const C3_KASHIDA: u32 = 512u32;
pub const C3_KATAKANA: u32 = 16u32;
pub const C3_LEXICAL: u32 = 1024u32;
pub const C3_LOWSURROGATE: u32 = 4096u32;
pub const C3_NONSPACING: u32 = 1u32;
pub const C3_NOTAPPLICABLE: u32 = 0u32;
pub const C3_SYMBOL: u32 = 8u32;
pub const C3_VOWELMARK: u32 = 4u32;
pub type CALINFO_ENUMPROCA = StdCallFnPtr<(PCSTR,), super::Foundation::BOOL>;
pub type CALINFO_ENUMPROCEXA = StdCallFnPtr<(PCSTR, u32), super::Foundation::BOOL>;
pub type CALINFO_ENUMPROCEXEX =
    StdCallFnPtr<(PCWSTR, u32, PCWSTR, super::Foundation::LPARAM), super::Foundation::BOOL>;
pub type CALINFO_ENUMPROCEXW = StdCallFnPtr<(PCWSTR, u32), super::Foundation::BOOL>;
pub type CALINFO_ENUMPROCW = StdCallFnPtr<(PCWSTR,), super::Foundation::BOOL>;
pub const CAL_GREGORIAN: u32 = 1u32;
pub const CAL_GREGORIAN_ARABIC: u32 = 10u32;
pub const CAL_GREGORIAN_ME_FRENCH: u32 = 9u32;
pub const CAL_GREGORIAN_US: u32 = 2u32;
pub const CAL_GREGORIAN_XLIT_ENGLISH: u32 = 11u32;
pub const CAL_GREGORIAN_XLIT_FRENCH: u32 = 12u32;
pub const CAL_HEBREW: u32 = 8u32;
pub const CAL_HIJRI: u32 = 6u32;
pub const CAL_ICALINTVALUE: u32 = 1u32;
pub const CAL_ITWODIGITYEARMAX: u32 = 48u32;
pub const CAL_IYEAROFFSETRANGE: u32 = 3u32;
pub const CAL_JAPAN: u32 = 3u32;
pub const CAL_KOREA: u32 = 5u32;
pub const CAL_NOUSEROVERRIDE: u32 = 2147483648u32;
pub const CAL_PERSIAN: u32 = 22u32;
pub const CAL_RETURN_GENITIVE_NAMES: u32 = 268435456u32;
pub const CAL_RETURN_NUMBER: u32 = 536870912u32;
pub const CAL_SABBREVDAYNAME1: u32 = 14u32;
pub const CAL_SABBREVDAYNAME2: u32 = 15u32;
pub const CAL_SABBREVDAYNAME3: u32 = 16u32;
pub const CAL_SABBREVDAYNAME4: u32 = 17u32;
pub const CAL_SABBREVDAYNAME5: u32 = 18u32;
pub const CAL_SABBREVDAYNAME6: u32 = 19u32;
pub const CAL_SABBREVDAYNAME7: u32 = 20u32;
pub const CAL_SABBREVERASTRING: u32 = 57u32;
pub const CAL_SABBREVMONTHNAME1: u32 = 34u32;
pub const CAL_SABBREVMONTHNAME10: u32 = 43u32;
pub const CAL_SABBREVMONTHNAME11: u32 = 44u32;
pub const CAL_SABBREVMONTHNAME12: u32 = 45u32;
pub const CAL_SABBREVMONTHNAME13: u32 = 46u32;
pub const CAL_SABBREVMONTHNAME2: u32 = 35u32;
pub const CAL_SABBREVMONTHNAME3: u32 = 36u32;
pub const CAL_SABBREVMONTHNAME4: u32 = 37u32;
pub const CAL_SABBREVMONTHNAME5: u32 = 38u32;
pub const CAL_SABBREVMONTHNAME6: u32 = 39u32;
pub const CAL_SABBREVMONTHNAME7: u32 = 40u32;
pub const CAL_SABBREVMONTHNAME8: u32 = 41u32;
pub const CAL_SABBREVMONTHNAME9: u32 = 42u32;
pub const CAL_SCALNAME: u32 = 2u32;
pub const CAL_SDAYNAME1: u32 = 7u32;
pub const CAL_SDAYNAME2: u32 = 8u32;
pub const CAL_SDAYNAME3: u32 = 9u32;
pub const CAL_SDAYNAME4: u32 = 10u32;
pub const CAL_SDAYNAME5: u32 = 11u32;
pub const CAL_SDAYNAME6: u32 = 12u32;
pub const CAL_SDAYNAME7: u32 = 13u32;
pub const CAL_SENGLISHABBREVERANAME: u32 = 60u32;
pub const CAL_SENGLISHERANAME: u32 = 59u32;
pub const CAL_SERASTRING: u32 = 4u32;
pub const CAL_SJAPANESEERAFIRSTYEAR: u32 = 61u32;
pub const CAL_SLONGDATE: u32 = 6u32;
pub const CAL_SMONTHDAY: u32 = 56u32;
pub const CAL_SMONTHNAME1: u32 = 21u32;
pub const CAL_SMONTHNAME10: u32 = 30u32;
pub const CAL_SMONTHNAME11: u32 = 31u32;
pub const CAL_SMONTHNAME12: u32 = 32u32;
pub const CAL_SMONTHNAME13: u32 = 33u32;
pub const CAL_SMONTHNAME2: u32 = 22u32;
pub const CAL_SMONTHNAME3: u32 = 23u32;
pub const CAL_SMONTHNAME4: u32 = 24u32;
pub const CAL_SMONTHNAME5: u32 = 25u32;
pub const CAL_SMONTHNAME6: u32 = 26u32;
pub const CAL_SMONTHNAME7: u32 = 27u32;
pub const CAL_SMONTHNAME8: u32 = 28u32;
pub const CAL_SMONTHNAME9: u32 = 29u32;
pub const CAL_SRELATIVELONGDATE: u32 = 58u32;
pub const CAL_SSHORTDATE: u32 = 5u32;
pub const CAL_SSHORTESTDAYNAME1: u32 = 49u32;
pub const CAL_SSHORTESTDAYNAME2: u32 = 50u32;
pub const CAL_SSHORTESTDAYNAME3: u32 = 51u32;
pub const CAL_SSHORTESTDAYNAME4: u32 = 52u32;
pub const CAL_SSHORTESTDAYNAME5: u32 = 53u32;
pub const CAL_SSHORTESTDAYNAME6: u32 = 54u32;
pub const CAL_SSHORTESTDAYNAME7: u32 = 55u32;
pub const CAL_SYEARMONTH: u32 = 47u32;
pub const CAL_TAIWAN: u32 = 4u32;
pub const CAL_THAI: u32 = 7u32;
pub const CAL_UMALQURA: u32 = 23u32;
pub const CAL_USE_CP_ACP: u32 = 1073741824u32;
pub const CANITER_SKIP_ZEROES: u32 = 1u32;
pub struct CHARSETINFO {
    pub ciCharset: u32,
    pub ciACP: u32,
    pub fs: FONTSIGNATURE,
}
impl ::core::marker::Copy for CHARSETINFO {}
impl ::core::clone::Clone for CHARSETINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CHARSETINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHARSETINFO")
            .field("ciCharset", &self.ciCharset)
            .field("ciACP", &self.ciACP)
            .field("fs", &self.fs)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CHARSETINFO {
    fn eq(&self, other: &Self) -> bool {
        self.ciCharset == other.ciCharset && self.ciACP == other.ciACP && self.fs == other.fs
    }
}
impl ::core::cmp::Eq for CHARSETINFO {}
impl FromIntoMemory for CHARSETINFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32u32 as usize);
        let f_ciCharset = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ciACP = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_fs = <FONTSIGNATURE as FromIntoMemory>::from_bytes(&from[8..8 + 24]);
        Self {
            ciCharset: f_ciCharset,
            ciACP: f_ciACP,
            fs: f_fs,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32u32 as usize);
        FromIntoMemory::into_bytes(self.ciCharset, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ciACP, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.fs, &mut into[8..8 + 24]);
    }
    fn size() -> usize {
        32u32 as usize
    }
}
pub const CMLangConvertCharset: crate::core::GUID =
    crate::core::GUID::from_u128(0xd66d6f99_cdaa_11d0_b822_00c04fc9b31f);
pub const CMLangString: crate::core::GUID =
    crate::core::GUID::from_u128(0xc04d65cf_b70d_11d0_b188_00aa0038c969);
pub const CMultiLanguage: crate::core::GUID =
    crate::core::GUID::from_u128(0x275c23e2_3747_11d0_9fea_00aa003f8646);
pub type CODEPAGE_ENUMPROCA = StdCallFnPtr<(PCSTR,), super::Foundation::BOOL>;
pub type CODEPAGE_ENUMPROCW = StdCallFnPtr<(PCWSTR,), super::Foundation::BOOL>;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMPARE_STRING_FLAGS(pub u32);
pub const LINGUISTIC_IGNORECASE: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(16u32);
pub const LINGUISTIC_IGNOREDIACRITIC: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(32u32);
pub const NORM_IGNORECASE: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(1u32);
pub const NORM_IGNOREKANATYPE: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(65536u32);
pub const NORM_IGNORENONSPACE: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(2u32);
pub const NORM_IGNORESYMBOLS: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(4u32);
pub const NORM_IGNOREWIDTH: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(131072u32);
pub const NORM_LINGUISTIC_CASING: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(134217728u32);
pub const SORT_DIGITSASNUMBERS: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(8u32);
pub const SORT_STRINGSORT: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(4096u32);
impl ::core::marker::Copy for COMPARE_STRING_FLAGS {}
impl ::core::clone::Clone for COMPARE_STRING_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMPARE_STRING_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COMPARE_STRING_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMPARE_STRING_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for COMPARE_STRING_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for COMPARE_STRING_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for COMPARE_STRING_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for COMPARE_STRING_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for COMPARE_STRING_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for COMPARE_STRING_FLAGS {
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
pub struct CORRECTIVE_ACTION(pub i32);
pub const CORRECTIVE_ACTION_NONE: CORRECTIVE_ACTION = CORRECTIVE_ACTION(0i32);
pub const CORRECTIVE_ACTION_GET_SUGGESTIONS: CORRECTIVE_ACTION = CORRECTIVE_ACTION(1i32);
pub const CORRECTIVE_ACTION_REPLACE: CORRECTIVE_ACTION = CORRECTIVE_ACTION(2i32);
pub const CORRECTIVE_ACTION_DELETE: CORRECTIVE_ACTION = CORRECTIVE_ACTION(3i32);
impl ::core::marker::Copy for CORRECTIVE_ACTION {}
impl ::core::clone::Clone for CORRECTIVE_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CORRECTIVE_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CORRECTIVE_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CORRECTIVE_ACTION").field(&self.0).finish()
    }
}
impl FromIntoMemory for CORRECTIVE_ACTION {
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
pub struct CPINFO {
    pub MaxCharSize: u32,
    pub DefaultChar: [u8; 2],
    pub LeadByte: [u8; 12],
}
impl ::core::marker::Copy for CPINFO {}
impl ::core::clone::Clone for CPINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CPINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CPINFO")
            .field("MaxCharSize", &self.MaxCharSize)
            .field("DefaultChar", &self.DefaultChar)
            .field("LeadByte", &self.LeadByte)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CPINFO {
    fn eq(&self, other: &Self) -> bool {
        self.MaxCharSize == other.MaxCharSize
            && self.DefaultChar == other.DefaultChar
            && self.LeadByte == other.LeadByte
    }
}
impl ::core::cmp::Eq for CPINFO {}
impl FromIntoMemory for CPINFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20u32 as usize);
        let f_MaxCharSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_DefaultChar = <[u8; 2] as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_LeadByte = <[u8; 12] as FromIntoMemory>::from_bytes(&from[6..6 + 12]);
        Self {
            MaxCharSize: f_MaxCharSize,
            DefaultChar: f_DefaultChar,
            LeadByte: f_LeadByte,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20u32 as usize);
        FromIntoMemory::into_bytes(self.MaxCharSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.DefaultChar, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.LeadByte, &mut into[6..6 + 12]);
    }
    fn size() -> usize {
        20u32 as usize
    }
}
pub struct CPINFOEXA {
    pub MaxCharSize: u32,
    pub DefaultChar: [u8; 2],
    pub LeadByte: [u8; 12],
    pub UnicodeDefaultChar: u16,
    pub CodePage: u32,
    pub CodePageName: [super::Foundation::CHAR; 260],
}
impl ::core::marker::Copy for CPINFOEXA {}
impl ::core::clone::Clone for CPINFOEXA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CPINFOEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CPINFOEXA")
            .field("MaxCharSize", &self.MaxCharSize)
            .field("DefaultChar", &self.DefaultChar)
            .field("LeadByte", &self.LeadByte)
            .field("UnicodeDefaultChar", &self.UnicodeDefaultChar)
            .field("CodePage", &self.CodePage)
            .field("CodePageName", &self.CodePageName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CPINFOEXA {
    fn eq(&self, other: &Self) -> bool {
        self.MaxCharSize == other.MaxCharSize
            && self.DefaultChar == other.DefaultChar
            && self.LeadByte == other.LeadByte
            && self.UnicodeDefaultChar == other.UnicodeDefaultChar
            && self.CodePage == other.CodePage
            && self.CodePageName == other.CodePageName
    }
}
impl ::core::cmp::Eq for CPINFOEXA {}
impl FromIntoMemory for CPINFOEXA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 284u32 as usize);
        let f_MaxCharSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_DefaultChar = <[u8; 2] as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_LeadByte = <[u8; 12] as FromIntoMemory>::from_bytes(&from[6..6 + 12]);
        let f_UnicodeDefaultChar = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 1]);
        let f_CodePage = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_CodePageName =
            <[super::Foundation::CHAR; 260] as FromIntoMemory>::from_bytes(&from[24..24 + 260]);
        Self {
            MaxCharSize: f_MaxCharSize,
            DefaultChar: f_DefaultChar,
            LeadByte: f_LeadByte,
            UnicodeDefaultChar: f_UnicodeDefaultChar,
            CodePage: f_CodePage,
            CodePageName: f_CodePageName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 284u32 as usize);
        FromIntoMemory::into_bytes(self.MaxCharSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.DefaultChar, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.LeadByte, &mut into[6..6 + 12]);
        FromIntoMemory::into_bytes(self.UnicodeDefaultChar, &mut into[18..18 + 1]);
        FromIntoMemory::into_bytes(self.CodePage, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.CodePageName, &mut into[24..24 + 260]);
    }
    fn size() -> usize {
        284u32 as usize
    }
}
pub struct CPINFOEXW {
    pub MaxCharSize: u32,
    pub DefaultChar: [u8; 2],
    pub LeadByte: [u8; 12],
    pub UnicodeDefaultChar: u16,
    pub CodePage: u32,
    pub CodePageName: [u16; 260],
}
impl ::core::marker::Copy for CPINFOEXW {}
impl ::core::clone::Clone for CPINFOEXW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CPINFOEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CPINFOEXW")
            .field("MaxCharSize", &self.MaxCharSize)
            .field("DefaultChar", &self.DefaultChar)
            .field("LeadByte", &self.LeadByte)
            .field("UnicodeDefaultChar", &self.UnicodeDefaultChar)
            .field("CodePage", &self.CodePage)
            .field("CodePageName", &self.CodePageName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CPINFOEXW {
    fn eq(&self, other: &Self) -> bool {
        self.MaxCharSize == other.MaxCharSize
            && self.DefaultChar == other.DefaultChar
            && self.LeadByte == other.LeadByte
            && self.UnicodeDefaultChar == other.UnicodeDefaultChar
            && self.CodePage == other.CodePage
            && self.CodePageName == other.CodePageName
    }
}
impl ::core::cmp::Eq for CPINFOEXW {}
impl FromIntoMemory for CPINFOEXW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 284u32 as usize);
        let f_MaxCharSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_DefaultChar = <[u8; 2] as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_LeadByte = <[u8; 12] as FromIntoMemory>::from_bytes(&from[6..6 + 12]);
        let f_UnicodeDefaultChar = <u16 as FromIntoMemory>::from_bytes(&from[18..18 + 1]);
        let f_CodePage = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_CodePageName = <[u16; 260] as FromIntoMemory>::from_bytes(&from[24..24 + 260]);
        Self {
            MaxCharSize: f_MaxCharSize,
            DefaultChar: f_DefaultChar,
            LeadByte: f_LeadByte,
            UnicodeDefaultChar: f_UnicodeDefaultChar,
            CodePage: f_CodePage,
            CodePageName: f_CodePageName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 284u32 as usize);
        FromIntoMemory::into_bytes(self.MaxCharSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.DefaultChar, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.LeadByte, &mut into[6..6 + 12]);
        FromIntoMemory::into_bytes(self.UnicodeDefaultChar, &mut into[18..18 + 1]);
        FromIntoMemory::into_bytes(self.CodePage, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.CodePageName, &mut into[24..24 + 260]);
    }
    fn size() -> usize {
        284u32 as usize
    }
}
pub const CPIOD_FORCE_PROMPT: i32 = -2147483648i32;
pub const CPIOD_PEEK: i32 = 1073741824i32;
pub const CP_ACP: u32 = 0u32;
pub const CP_MACCP: u32 = 2u32;
pub const CP_OEMCP: u32 = 1u32;
pub const CP_SYMBOL: u32 = 42u32;
pub const CP_THREAD_ACP: u32 = 3u32;
pub const CP_UTF7: u32 = 65000u32;
pub const CP_UTF8: u32 = 65001u32;
pub const CSTR_EQUAL: u32 = 2u32;
pub const CSTR_GREATER_THAN: u32 = 3u32;
pub const CSTR_LESS_THAN: u32 = 1u32;
pub const CTRY_ALBANIA: u32 = 355u32;
pub const CTRY_ALGERIA: u32 = 213u32;
pub const CTRY_ARGENTINA: u32 = 54u32;
pub const CTRY_ARMENIA: u32 = 374u32;
pub const CTRY_AUSTRALIA: u32 = 61u32;
pub const CTRY_AUSTRIA: u32 = 43u32;
pub const CTRY_AZERBAIJAN: u32 = 994u32;
pub const CTRY_BAHRAIN: u32 = 973u32;
pub const CTRY_BELARUS: u32 = 375u32;
pub const CTRY_BELGIUM: u32 = 32u32;
pub const CTRY_BELIZE: u32 = 501u32;
pub const CTRY_BOLIVIA: u32 = 591u32;
pub const CTRY_BRAZIL: u32 = 55u32;
pub const CTRY_BRUNEI_DARUSSALAM: u32 = 673u32;
pub const CTRY_BULGARIA: u32 = 359u32;
pub const CTRY_CANADA: u32 = 2u32;
pub const CTRY_CARIBBEAN: u32 = 1u32;
pub const CTRY_CHILE: u32 = 56u32;
pub const CTRY_COLOMBIA: u32 = 57u32;
pub const CTRY_COSTA_RICA: u32 = 506u32;
pub const CTRY_CROATIA: u32 = 385u32;
pub const CTRY_CZECH: u32 = 420u32;
pub const CTRY_DEFAULT: u32 = 0u32;
pub const CTRY_DENMARK: u32 = 45u32;
pub const CTRY_DOMINICAN_REPUBLIC: u32 = 1u32;
pub const CTRY_ECUADOR: u32 = 593u32;
pub const CTRY_EGYPT: u32 = 20u32;
pub const CTRY_EL_SALVADOR: u32 = 503u32;
pub const CTRY_ESTONIA: u32 = 372u32;
pub const CTRY_FAEROE_ISLANDS: u32 = 298u32;
pub const CTRY_FINLAND: u32 = 358u32;
pub const CTRY_FRANCE: u32 = 33u32;
pub const CTRY_GEORGIA: u32 = 995u32;
pub const CTRY_GERMANY: u32 = 49u32;
pub const CTRY_GREECE: u32 = 30u32;
pub const CTRY_GUATEMALA: u32 = 502u32;
pub const CTRY_HONDURAS: u32 = 504u32;
pub const CTRY_HONG_KONG: u32 = 852u32;
pub const CTRY_HUNGARY: u32 = 36u32;
pub const CTRY_ICELAND: u32 = 354u32;
pub const CTRY_INDIA: u32 = 91u32;
pub const CTRY_INDONESIA: u32 = 62u32;
pub const CTRY_IRAN: u32 = 981u32;
pub const CTRY_IRAQ: u32 = 964u32;
pub const CTRY_IRELAND: u32 = 353u32;
pub const CTRY_ISRAEL: u32 = 972u32;
pub const CTRY_ITALY: u32 = 39u32;
pub const CTRY_JAMAICA: u32 = 1u32;
pub const CTRY_JAPAN: u32 = 81u32;
pub const CTRY_JORDAN: u32 = 962u32;
pub const CTRY_KAZAKSTAN: u32 = 7u32;
pub const CTRY_KENYA: u32 = 254u32;
pub const CTRY_KUWAIT: u32 = 965u32;
pub const CTRY_KYRGYZSTAN: u32 = 996u32;
pub const CTRY_LATVIA: u32 = 371u32;
pub const CTRY_LEBANON: u32 = 961u32;
pub const CTRY_LIBYA: u32 = 218u32;
pub const CTRY_LIECHTENSTEIN: u32 = 41u32;
pub const CTRY_LITHUANIA: u32 = 370u32;
pub const CTRY_LUXEMBOURG: u32 = 352u32;
pub const CTRY_MACAU: u32 = 853u32;
pub const CTRY_MACEDONIA: u32 = 389u32;
pub const CTRY_MALAYSIA: u32 = 60u32;
pub const CTRY_MALDIVES: u32 = 960u32;
pub const CTRY_MEXICO: u32 = 52u32;
pub const CTRY_MONACO: u32 = 33u32;
pub const CTRY_MONGOLIA: u32 = 976u32;
pub const CTRY_MOROCCO: u32 = 212u32;
pub const CTRY_NETHERLANDS: u32 = 31u32;
pub const CTRY_NEW_ZEALAND: u32 = 64u32;
pub const CTRY_NICARAGUA: u32 = 505u32;
pub const CTRY_NORWAY: u32 = 47u32;
pub const CTRY_OMAN: u32 = 968u32;
pub const CTRY_PAKISTAN: u32 = 92u32;
pub const CTRY_PANAMA: u32 = 507u32;
pub const CTRY_PARAGUAY: u32 = 595u32;
pub const CTRY_PERU: u32 = 51u32;
pub const CTRY_PHILIPPINES: u32 = 63u32;
pub const CTRY_POLAND: u32 = 48u32;
pub const CTRY_PORTUGAL: u32 = 351u32;
pub const CTRY_PRCHINA: u32 = 86u32;
pub const CTRY_PUERTO_RICO: u32 = 1u32;
pub const CTRY_QATAR: u32 = 974u32;
pub const CTRY_ROMANIA: u32 = 40u32;
pub const CTRY_RUSSIA: u32 = 7u32;
pub const CTRY_SAUDI_ARABIA: u32 = 966u32;
pub const CTRY_SERBIA: u32 = 381u32;
pub const CTRY_SINGAPORE: u32 = 65u32;
pub const CTRY_SLOVAK: u32 = 421u32;
pub const CTRY_SLOVENIA: u32 = 386u32;
pub const CTRY_SOUTH_AFRICA: u32 = 27u32;
pub const CTRY_SOUTH_KOREA: u32 = 82u32;
pub const CTRY_SPAIN: u32 = 34u32;
pub const CTRY_SWEDEN: u32 = 46u32;
pub const CTRY_SWITZERLAND: u32 = 41u32;
pub const CTRY_SYRIA: u32 = 963u32;
pub const CTRY_TAIWAN: u32 = 886u32;
pub const CTRY_TATARSTAN: u32 = 7u32;
pub const CTRY_THAILAND: u32 = 66u32;
pub const CTRY_TRINIDAD_Y_TOBAGO: u32 = 1u32;
pub const CTRY_TUNISIA: u32 = 216u32;
pub const CTRY_TURKEY: u32 = 90u32;
pub const CTRY_UAE: u32 = 971u32;
pub const CTRY_UKRAINE: u32 = 380u32;
pub const CTRY_UNITED_KINGDOM: u32 = 44u32;
pub const CTRY_UNITED_STATES: u32 = 1u32;
pub const CTRY_URUGUAY: u32 = 598u32;
pub const CTRY_UZBEKISTAN: u32 = 7u32;
pub const CTRY_VENEZUELA: u32 = 58u32;
pub const CTRY_VIET_NAM: u32 = 84u32;
pub const CTRY_YEMEN: u32 = 967u32;
pub const CTRY_ZIMBABWE: u32 = 263u32;
pub const CT_CTYPE1: u32 = 1u32;
pub const CT_CTYPE2: u32 = 2u32;
pub const CT_CTYPE3: u32 = 4u32;
pub struct CURRENCYFMTA {
    pub NumDigits: u32,
    pub LeadingZero: u32,
    pub Grouping: u32,
    pub lpDecimalSep: PSTR,
    pub lpThousandSep: PSTR,
    pub NegativeOrder: u32,
    pub PositiveOrder: u32,
    pub lpCurrencySymbol: PSTR,
}
impl ::core::marker::Copy for CURRENCYFMTA {}
impl ::core::clone::Clone for CURRENCYFMTA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CURRENCYFMTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CURRENCYFMTA")
            .field("NumDigits", &self.NumDigits)
            .field("LeadingZero", &self.LeadingZero)
            .field("Grouping", &self.Grouping)
            .field("lpDecimalSep", &self.lpDecimalSep)
            .field("lpThousandSep", &self.lpThousandSep)
            .field("NegativeOrder", &self.NegativeOrder)
            .field("PositiveOrder", &self.PositiveOrder)
            .field("lpCurrencySymbol", &self.lpCurrencySymbol)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CURRENCYFMTA {
    fn eq(&self, other: &Self) -> bool {
        self.NumDigits == other.NumDigits
            && self.LeadingZero == other.LeadingZero
            && self.Grouping == other.Grouping
            && self.lpDecimalSep == other.lpDecimalSep
            && self.lpThousandSep == other.lpThousandSep
            && self.NegativeOrder == other.NegativeOrder
            && self.PositiveOrder == other.PositiveOrder
            && self.lpCurrencySymbol == other.lpCurrencySymbol
    }
}
impl ::core::cmp::Eq for CURRENCYFMTA {}
impl FromIntoMemory for CURRENCYFMTA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32u32 as usize);
        let f_NumDigits = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_LeadingZero = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Grouping = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpDecimalSep = <PSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpThousandSep = <PSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_NegativeOrder = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_PositiveOrder = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_lpCurrencySymbol = <PSTR as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            NumDigits: f_NumDigits,
            LeadingZero: f_LeadingZero,
            Grouping: f_Grouping,
            lpDecimalSep: f_lpDecimalSep,
            lpThousandSep: f_lpThousandSep,
            NegativeOrder: f_NegativeOrder,
            PositiveOrder: f_PositiveOrder,
            lpCurrencySymbol: f_lpCurrencySymbol,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32u32 as usize);
        FromIntoMemory::into_bytes(self.NumDigits, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.LeadingZero, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Grouping, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpDecimalSep, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpThousandSep, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.NegativeOrder, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.PositiveOrder, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.lpCurrencySymbol, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32u32 as usize
    }
}
pub struct CURRENCYFMTW {
    pub NumDigits: u32,
    pub LeadingZero: u32,
    pub Grouping: u32,
    pub lpDecimalSep: PWSTR,
    pub lpThousandSep: PWSTR,
    pub NegativeOrder: u32,
    pub PositiveOrder: u32,
    pub lpCurrencySymbol: PWSTR,
}
impl ::core::marker::Copy for CURRENCYFMTW {}
impl ::core::clone::Clone for CURRENCYFMTW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CURRENCYFMTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CURRENCYFMTW")
            .field("NumDigits", &self.NumDigits)
            .field("LeadingZero", &self.LeadingZero)
            .field("Grouping", &self.Grouping)
            .field("lpDecimalSep", &self.lpDecimalSep)
            .field("lpThousandSep", &self.lpThousandSep)
            .field("NegativeOrder", &self.NegativeOrder)
            .field("PositiveOrder", &self.PositiveOrder)
            .field("lpCurrencySymbol", &self.lpCurrencySymbol)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CURRENCYFMTW {
    fn eq(&self, other: &Self) -> bool {
        self.NumDigits == other.NumDigits
            && self.LeadingZero == other.LeadingZero
            && self.Grouping == other.Grouping
            && self.lpDecimalSep == other.lpDecimalSep
            && self.lpThousandSep == other.lpThousandSep
            && self.NegativeOrder == other.NegativeOrder
            && self.PositiveOrder == other.PositiveOrder
            && self.lpCurrencySymbol == other.lpCurrencySymbol
    }
}
impl ::core::cmp::Eq for CURRENCYFMTW {}
impl FromIntoMemory for CURRENCYFMTW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32u32 as usize);
        let f_NumDigits = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_LeadingZero = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Grouping = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpDecimalSep = <PWSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpThousandSep = <PWSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_NegativeOrder = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_PositiveOrder = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_lpCurrencySymbol = <PWSTR as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            NumDigits: f_NumDigits,
            LeadingZero: f_LeadingZero,
            Grouping: f_Grouping,
            lpDecimalSep: f_lpDecimalSep,
            lpThousandSep: f_lpThousandSep,
            NegativeOrder: f_NegativeOrder,
            PositiveOrder: f_PositiveOrder,
            lpCurrencySymbol: f_lpCurrencySymbol,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32u32 as usize);
        FromIntoMemory::into_bytes(self.NumDigits, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.LeadingZero, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Grouping, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpDecimalSep, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpThousandSep, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.NegativeOrder, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.PositiveOrder, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.lpCurrencySymbol, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32u32 as usize
    }
}
pub type DATEFMT_ENUMPROCA = StdCallFnPtr<(PCSTR,), super::Foundation::BOOL>;
pub type DATEFMT_ENUMPROCEXA = StdCallFnPtr<(PCSTR, u32), super::Foundation::BOOL>;
pub type DATEFMT_ENUMPROCEXEX =
    StdCallFnPtr<(PCWSTR, u32, super::Foundation::LPARAM), super::Foundation::BOOL>;
pub type DATEFMT_ENUMPROCEXW = StdCallFnPtr<(PCWSTR, u32), super::Foundation::BOOL>;
pub type DATEFMT_ENUMPROCW = StdCallFnPtr<(PCWSTR,), super::Foundation::BOOL>;
pub struct DetectEncodingInfo {
    pub nLangID: u32,
    pub nCodePage: u32,
    pub nDocPercent: i32,
    pub nConfidence: i32,
}
impl ::core::marker::Copy for DetectEncodingInfo {}
impl ::core::clone::Clone for DetectEncodingInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DetectEncodingInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DetectEncodingInfo")
            .field("nLangID", &self.nLangID)
            .field("nCodePage", &self.nCodePage)
            .field("nDocPercent", &self.nDocPercent)
            .field("nConfidence", &self.nConfidence)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DetectEncodingInfo {
    fn eq(&self, other: &Self) -> bool {
        self.nLangID == other.nLangID
            && self.nCodePage == other.nCodePage
            && self.nDocPercent == other.nDocPercent
            && self.nConfidence == other.nConfidence
    }
}
impl ::core::cmp::Eq for DetectEncodingInfo {}
impl FromIntoMemory for DetectEncodingInfo {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16u32 as usize);
        let f_nLangID = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_nCodePage = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_nDocPercent = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_nConfidence = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            nLangID: f_nLangID,
            nCodePage: f_nCodePage,
            nDocPercent: f_nDocPercent,
            nConfidence: f_nConfidence,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16u32 as usize);
        FromIntoMemory::into_bytes(self.nLangID, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.nCodePage, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.nDocPercent, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.nConfidence, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16u32 as usize
    }
}
pub const ELS_GUID_LANGUAGE_DETECTION: crate::core::GUID =
    crate::core::GUID::from_u128(0xcf7e00b1_909b_4d95_a8f4_611f7c377702);
pub const ELS_GUID_SCRIPT_DETECTION: crate::core::GUID =
    crate::core::GUID::from_u128(0x2d64b439_6caf_4f6b_b688_e5d0f4faa7d7);
pub const ELS_GUID_TRANSLITERATION_BENGALI_TO_LATIN: crate::core::GUID =
    crate::core::GUID::from_u128(0xf4dfd825_91a4_489f_855e_9ad9bee55727);
pub const ELS_GUID_TRANSLITERATION_CYRILLIC_TO_LATIN: crate::core::GUID =
    crate::core::GUID::from_u128(0x3dd12a98_5afd_4903_a13f_e17e6c0bfe01);
pub const ELS_GUID_TRANSLITERATION_DEVANAGARI_TO_LATIN: crate::core::GUID =
    crate::core::GUID::from_u128(0xc4a4dcfe_2661_4d02_9835_f48187109803);
pub const ELS_GUID_TRANSLITERATION_HANGUL_DECOMPOSITION: crate::core::GUID =
    crate::core::GUID::from_u128(0x4ba2a721_e43d_41b7_b330_536ae1e48863);
pub const ELS_GUID_TRANSLITERATION_HANS_TO_HANT: crate::core::GUID =
    crate::core::GUID::from_u128(0x3caccdc8_5590_42dc_9a7b_b5a6b5b3b63b);
pub const ELS_GUID_TRANSLITERATION_HANT_TO_HANS: crate::core::GUID =
    crate::core::GUID::from_u128(0xa3a8333b_f4fc_42f6_a0c4_0462fe7317cb);
pub const ELS_GUID_TRANSLITERATION_MALAYALAM_TO_LATIN: crate::core::GUID =
    crate::core::GUID::from_u128(0xd8b983b1_f8bf_4a2b_bcd5_5b5ea20613e1);
pub struct ENUMTEXTMETRICA {
    pub etmNewTextMetricEx: NEWTEXTMETRICEXA,
    pub etmAxesList: super::Graphics::Gdi::AXESLISTA,
}
impl ::core::marker::Copy for ENUMTEXTMETRICA {}
impl ::core::clone::Clone for ENUMTEXTMETRICA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENUMTEXTMETRICA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMTEXTMETRICA")
            .field("etmNewTextMetricEx", &self.etmNewTextMetricEx)
            .field("etmAxesList", &self.etmAxesList)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ENUMTEXTMETRICA {
    fn eq(&self, other: &Self) -> bool {
        self.etmNewTextMetricEx == other.etmNewTextMetricEx && self.etmAxesList == other.etmAxesList
    }
}
impl ::core::cmp::Eq for ENUMTEXTMETRICA {}
impl FromIntoMemory for ENUMTEXTMETRICA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 488u32 as usize);
        let f_etmNewTextMetricEx =
            <NEWTEXTMETRICEXA as FromIntoMemory>::from_bytes(&from[0..0 + 96]);
        let f_etmAxesList =
            <super::Graphics::Gdi::AXESLISTA as FromIntoMemory>::from_bytes(&from[96..96 + 392]);
        Self {
            etmNewTextMetricEx: f_etmNewTextMetricEx,
            etmAxesList: f_etmAxesList,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 488u32 as usize);
        FromIntoMemory::into_bytes(self.etmNewTextMetricEx, &mut into[0..0 + 96]);
        FromIntoMemory::into_bytes(self.etmAxesList, &mut into[96..96 + 392]);
    }
    fn size() -> usize {
        488u32 as usize
    }
}
pub struct ENUMTEXTMETRICW {
    pub etmNewTextMetricEx: NEWTEXTMETRICEXW,
    pub etmAxesList: super::Graphics::Gdi::AXESLISTW,
}
impl ::core::marker::Copy for ENUMTEXTMETRICW {}
impl ::core::clone::Clone for ENUMTEXTMETRICW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENUMTEXTMETRICW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMTEXTMETRICW")
            .field("etmNewTextMetricEx", &self.etmNewTextMetricEx)
            .field("etmAxesList", &self.etmAxesList)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ENUMTEXTMETRICW {
    fn eq(&self, other: &Self) -> bool {
        self.etmNewTextMetricEx == other.etmNewTextMetricEx && self.etmAxesList == other.etmAxesList
    }
}
impl ::core::cmp::Eq for ENUMTEXTMETRICW {}
impl FromIntoMemory for ENUMTEXTMETRICW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 488u32 as usize);
        let f_etmNewTextMetricEx =
            <NEWTEXTMETRICEXW as FromIntoMemory>::from_bytes(&from[0..0 + 96]);
        let f_etmAxesList =
            <super::Graphics::Gdi::AXESLISTW as FromIntoMemory>::from_bytes(&from[96..96 + 392]);
        Self {
            etmNewTextMetricEx: f_etmNewTextMetricEx,
            etmAxesList: f_etmAxesList,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 488u32 as usize);
        FromIntoMemory::into_bytes(self.etmNewTextMetricEx, &mut into[0..0 + 96]);
        FromIntoMemory::into_bytes(self.etmAxesList, &mut into[96..96 + 392]);
    }
    fn size() -> usize {
        488u32 as usize
    }
}
pub const ENUM_ALL_CALENDARS: u32 = 4294967295u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ENUM_DATE_FORMATS_FLAGS(pub u32);
pub const DATE_SHORTDATE: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(1u32);
pub const DATE_LONGDATE: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(2u32);
pub const DATE_YEARMONTH: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(8u32);
pub const DATE_MONTHDAY: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(128u32);
pub const DATE_AUTOLAYOUT: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(64u32);
pub const DATE_LTRREADING: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(16u32);
pub const DATE_RTLREADING: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(32u32);
pub const DATE_USE_ALT_CALENDAR: ENUM_DATE_FORMATS_FLAGS = ENUM_DATE_FORMATS_FLAGS(4u32);
impl ::core::marker::Copy for ENUM_DATE_FORMATS_FLAGS {}
impl ::core::clone::Clone for ENUM_DATE_FORMATS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ENUM_DATE_FORMATS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ENUM_DATE_FORMATS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENUM_DATE_FORMATS_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ENUM_DATE_FORMATS_FLAGS {
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
pub struct ENUM_SYSTEM_CODE_PAGES_FLAGS(pub u32);
pub const CP_INSTALLED: ENUM_SYSTEM_CODE_PAGES_FLAGS = ENUM_SYSTEM_CODE_PAGES_FLAGS(1u32);
pub const CP_SUPPORTED: ENUM_SYSTEM_CODE_PAGES_FLAGS = ENUM_SYSTEM_CODE_PAGES_FLAGS(2u32);
impl ::core::marker::Copy for ENUM_SYSTEM_CODE_PAGES_FLAGS {}
impl ::core::clone::Clone for ENUM_SYSTEM_CODE_PAGES_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ENUM_SYSTEM_CODE_PAGES_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ENUM_SYSTEM_CODE_PAGES_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENUM_SYSTEM_CODE_PAGES_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ENUM_SYSTEM_CODE_PAGES_FLAGS {
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
pub struct ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS(pub u32);
pub const LGRPID_INSTALLED: ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS =
    ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS(1u32);
pub const LGRPID_SUPPORTED: ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS =
    ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS(2u32);
impl ::core::marker::Copy for ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS {}
impl ::core::clone::Clone for ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS {
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
pub struct FILEMUIINFO {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFileType: u32,
    pub pChecksum: [u8; 16],
    pub pServiceChecksum: [u8; 16],
    pub dwLanguageNameOffset: u32,
    pub dwTypeIDMainSize: u32,
    pub dwTypeIDMainOffset: u32,
    pub dwTypeNameMainOffset: u32,
    pub dwTypeIDMUISize: u32,
    pub dwTypeIDMUIOffset: u32,
    pub dwTypeNameMUIOffset: u32,
    pub abBuffer: [u8; 8],
}
impl ::core::marker::Copy for FILEMUIINFO {}
impl ::core::clone::Clone for FILEMUIINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILEMUIINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILEMUIINFO")
            .field("dwSize", &self.dwSize)
            .field("dwVersion", &self.dwVersion)
            .field("dwFileType", &self.dwFileType)
            .field("pChecksum", &self.pChecksum)
            .field("pServiceChecksum", &self.pServiceChecksum)
            .field("dwLanguageNameOffset", &self.dwLanguageNameOffset)
            .field("dwTypeIDMainSize", &self.dwTypeIDMainSize)
            .field("dwTypeIDMainOffset", &self.dwTypeIDMainOffset)
            .field("dwTypeNameMainOffset", &self.dwTypeNameMainOffset)
            .field("dwTypeIDMUISize", &self.dwTypeIDMUISize)
            .field("dwTypeIDMUIOffset", &self.dwTypeIDMUIOffset)
            .field("dwTypeNameMUIOffset", &self.dwTypeNameMUIOffset)
            .field("abBuffer", &self.abBuffer)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILEMUIINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwVersion == other.dwVersion
            && self.dwFileType == other.dwFileType
            && self.pChecksum == other.pChecksum
            && self.pServiceChecksum == other.pServiceChecksum
            && self.dwLanguageNameOffset == other.dwLanguageNameOffset
            && self.dwTypeIDMainSize == other.dwTypeIDMainSize
            && self.dwTypeIDMainOffset == other.dwTypeIDMainOffset
            && self.dwTypeNameMainOffset == other.dwTypeNameMainOffset
            && self.dwTypeIDMUISize == other.dwTypeIDMUISize
            && self.dwTypeIDMUIOffset == other.dwTypeIDMUIOffset
            && self.dwTypeNameMUIOffset == other.dwTypeNameMUIOffset
            && self.abBuffer == other.abBuffer
    }
}
impl ::core::cmp::Eq for FILEMUIINFO {}
impl FromIntoMemory for FILEMUIINFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 80u32 as usize);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwFileType = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_pChecksum = <[u8; 16] as FromIntoMemory>::from_bytes(&from[12..12 + 16]);
        let f_pServiceChecksum = <[u8; 16] as FromIntoMemory>::from_bytes(&from[28..28 + 16]);
        let f_dwLanguageNameOffset = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_dwTypeIDMainSize = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_dwTypeIDMainOffset = <u32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_dwTypeNameMainOffset = <u32 as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_dwTypeIDMUISize = <u32 as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_dwTypeIDMUIOffset = <u32 as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        let f_dwTypeNameMUIOffset = <u32 as FromIntoMemory>::from_bytes(&from[68..68 + 4]);
        let f_abBuffer = <[u8; 8] as FromIntoMemory>::from_bytes(&from[72..72 + 8]);
        Self {
            dwSize: f_dwSize,
            dwVersion: f_dwVersion,
            dwFileType: f_dwFileType,
            pChecksum: f_pChecksum,
            pServiceChecksum: f_pServiceChecksum,
            dwLanguageNameOffset: f_dwLanguageNameOffset,
            dwTypeIDMainSize: f_dwTypeIDMainSize,
            dwTypeIDMainOffset: f_dwTypeIDMainOffset,
            dwTypeNameMainOffset: f_dwTypeNameMainOffset,
            dwTypeIDMUISize: f_dwTypeIDMUISize,
            dwTypeIDMUIOffset: f_dwTypeIDMUIOffset,
            dwTypeNameMUIOffset: f_dwTypeNameMUIOffset,
            abBuffer: f_abBuffer,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 80u32 as usize);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwFileType, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.pChecksum, &mut into[12..12 + 16]);
        FromIntoMemory::into_bytes(self.pServiceChecksum, &mut into[28..28 + 16]);
        FromIntoMemory::into_bytes(self.dwLanguageNameOffset, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.dwTypeIDMainSize, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.dwTypeIDMainOffset, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.dwTypeNameMainOffset, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.dwTypeIDMUISize, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.dwTypeIDMUIOffset, &mut into[64..64 + 4]);
        FromIntoMemory::into_bytes(self.dwTypeNameMUIOffset, &mut into[68..68 + 4]);
        FromIntoMemory::into_bytes(self.abBuffer, &mut into[72..72 + 8]);
    }
    fn size() -> usize {
        80u32 as usize
    }
}
pub const FIND_ENDSWITH: u32 = 2097152u32;
pub const FIND_FROMEND: u32 = 8388608u32;
pub const FIND_FROMSTART: u32 = 4194304u32;
pub const FIND_STARTSWITH: u32 = 1048576u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FOLD_STRING_MAP_FLAGS(pub u32);
pub const MAP_COMPOSITE: FOLD_STRING_MAP_FLAGS = FOLD_STRING_MAP_FLAGS(64u32);
pub const MAP_EXPAND_LIGATURES: FOLD_STRING_MAP_FLAGS = FOLD_STRING_MAP_FLAGS(8192u32);
pub const MAP_FOLDCZONE: FOLD_STRING_MAP_FLAGS = FOLD_STRING_MAP_FLAGS(16u32);
pub const MAP_FOLDDIGITS: FOLD_STRING_MAP_FLAGS = FOLD_STRING_MAP_FLAGS(128u32);
pub const MAP_PRECOMPOSED: FOLD_STRING_MAP_FLAGS = FOLD_STRING_MAP_FLAGS(32u32);
impl ::core::marker::Copy for FOLD_STRING_MAP_FLAGS {}
impl ::core::clone::Clone for FOLD_STRING_MAP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FOLD_STRING_MAP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FOLD_STRING_MAP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FOLD_STRING_MAP_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for FOLD_STRING_MAP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FOLD_STRING_MAP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FOLD_STRING_MAP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FOLD_STRING_MAP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FOLD_STRING_MAP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for FOLD_STRING_MAP_FLAGS {
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
pub struct FONTSIGNATURE {
    pub fsUsb: [u32; 4],
    pub fsCsb: [u32; 2],
}
impl ::core::marker::Copy for FONTSIGNATURE {}
impl ::core::clone::Clone for FONTSIGNATURE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FONTSIGNATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FONTSIGNATURE")
            .field("fsUsb", &self.fsUsb)
            .field("fsCsb", &self.fsCsb)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FONTSIGNATURE {
    fn eq(&self, other: &Self) -> bool {
        self.fsUsb == other.fsUsb && self.fsCsb == other.fsCsb
    }
}
impl ::core::cmp::Eq for FONTSIGNATURE {}
impl FromIntoMemory for FONTSIGNATURE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24u32 as usize);
        let f_fsUsb = <[u32; 4] as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_fsCsb = <[u32; 2] as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        Self {
            fsUsb: f_fsUsb,
            fsCsb: f_fsCsb,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24u32 as usize);
        FromIntoMemory::into_bytes(self.fsUsb, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.fsCsb, &mut into[16..16 + 8]);
    }
    fn size() -> usize {
        24u32 as usize
    }
}
pub const GEOID_NOT_AVAILABLE: i32 = -1i32;
pub type GEO_ENUMNAMEPROC =
    StdCallFnPtr<(PCWSTR, super::Foundation::LPARAM), super::Foundation::BOOL>;
pub type GEO_ENUMPROC = StdCallFnPtr<(i32,), super::Foundation::BOOL>;
pub struct GOFFSET {
    pub du: i32,
    pub dv: i32,
}
impl ::core::marker::Copy for GOFFSET {}
impl ::core::clone::Clone for GOFFSET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GOFFSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GOFFSET")
            .field("du", &self.du)
            .field("dv", &self.dv)
            .finish()
    }
}
impl ::core::cmp::PartialEq for GOFFSET {
    fn eq(&self, other: &Self) -> bool {
        self.du == other.du && self.dv == other.dv
    }
}
impl ::core::cmp::Eq for GOFFSET {}
impl FromIntoMemory for GOFFSET {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8u32 as usize);
        let f_du = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dv = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self { du: f_du, dv: f_dv }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8u32 as usize);
        FromIntoMemory::into_bytes(self.du, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dv, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8u32 as usize
    }
}
pub const GSS_ALLOW_INHERITED_COMMON: u32 = 1u32;
pub const HIGHLEVEL_SERVICE_TYPES: u32 = 1u32;
pub const HIGH_SURROGATE_END: u32 = 56319u32;
pub const HIGH_SURROGATE_START: u32 = 55296u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HIMC(pub PtrDiffRepr);
impl HIMC {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HIMC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HIMC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HIMC {}
impl ::core::hash::Hash for HIMC {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HIMC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HIMC").field(&self.0).finish()
    }
}
impl FromIntoMemory for HIMC {
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
pub struct HIMCC(pub PtrDiffRepr);
impl HIMCC {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HIMCC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HIMCC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HIMCC {}
impl ::core::hash::Hash for HIMCC {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HIMCC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HIMCC").field(&self.0).finish()
    }
}
impl FromIntoMemory for HIMCC {
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
pub struct HSAVEDUILANGUAGES(pub PtrDiffRepr);
impl HSAVEDUILANGUAGES {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HSAVEDUILANGUAGES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HSAVEDUILANGUAGES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HSAVEDUILANGUAGES {}
impl ::core::hash::Hash for HSAVEDUILANGUAGES {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HSAVEDUILANGUAGES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HSAVEDUILANGUAGES").field(&self.0).finish()
    }
}
impl FromIntoMemory for HSAVEDUILANGUAGES {
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
pub struct IComprehensiveSpellCheckProvider(crate::core::IUnknown);
pub trait IComprehensiveSpellCheckProvider_Trait: crate::core::IUnknown_Trait {
    fn ComprehensiveCheck(
        &self,
        text: PCWSTR,
        value: MutPtr<IEnumSpellingError>,
    ) -> crate::core::HRESULT {
        todo!("ComprehensiveCheck")
    }
}
impl ::core::clone::Clone for IComprehensiveSpellCheckProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IComprehensiveSpellCheckProvider {}
impl ::core::cmp::PartialEq for IComprehensiveSpellCheckProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComprehensiveSpellCheckProvider {}
impl ::core::fmt::Debug for IComprehensiveSpellCheckProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComprehensiveSpellCheckProvider")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IComprehensiveSpellCheckProvider {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<crate::core::IUnknown>()
    }
}
impl crate::core::ComInterface for IComprehensiveSpellCheckProvider {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x0c58f8de_8e94_479e_9717_70c42c4ad2c3);
}
pub const IDN_ALLOW_UNASSIGNED: u32 = 1u32;
pub const IDN_EMAIL_ADDRESS: u32 = 4u32;
pub const IDN_RAW_PUNYCODE: u32 = 8u32;
pub const IDN_USE_STD3_ASCII_RULES: u32 = 2u32;
pub struct IEnumCodePage(crate::core::IUnknown);
pub trait IEnumCodePage_Trait: crate::core::IUnknown_Trait {
    fn Clone(&self, pp_enum: MutPtr<IEnumCodePage>) -> crate::core::HRESULT {
        todo!("Clone")
    }
    fn Next(
        &self,
        celt: u32,
        rgelt: MutPtr<MIMECPINFO>,
        pcelt_fetched: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("Next")
    }
    fn Reset(&self) -> crate::core::HRESULT {
        todo!("Reset")
    }
    fn Skip(&self, celt: u32) -> crate::core::HRESULT {
        todo!("Skip")
    }
}
impl ::core::clone::Clone for IEnumCodePage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IEnumCodePage {}
impl ::core::cmp::PartialEq for IEnumCodePage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumCodePage {}
impl ::core::fmt::Debug for IEnumCodePage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumCodePage").field(&self.0).finish()
    }
}
impl FromIntoMemory for IEnumCodePage {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<crate::core::IUnknown>()
    }
}
impl crate::core::ComInterface for IEnumCodePage {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x275c23e3_3747_11d0_9fea_00aa003f8646);
}
pub struct IEnumRfc1766(crate::core::IUnknown);
pub trait IEnumRfc1766_Trait: crate::core::IUnknown_Trait {
    fn Clone(&self, pp_enum: MutPtr<IEnumRfc1766>) -> crate::core::HRESULT {
        todo!("Clone")
    }
    fn Next(
        &self,
        celt: u32,
        rgelt: MutPtr<RFC1766INFO>,
        pcelt_fetched: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("Next")
    }
    fn Reset(&self) -> crate::core::HRESULT {
        todo!("Reset")
    }
    fn Skip(&self, celt: u32) -> crate::core::HRESULT {
        todo!("Skip")
    }
}
impl ::core::clone::Clone for IEnumRfc1766 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IEnumRfc1766 {}
impl ::core::cmp::PartialEq for IEnumRfc1766 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumRfc1766 {}
impl ::core::fmt::Debug for IEnumRfc1766 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumRfc1766").field(&self.0).finish()
    }
}
impl FromIntoMemory for IEnumRfc1766 {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<crate::core::IUnknown>()
    }
}
impl crate::core::ComInterface for IEnumRfc1766 {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x3dc39d1d_c030_11d0_b81b_00c04fc9b31f);
}
pub struct IEnumScript(crate::core::IUnknown);
pub trait IEnumScript_Trait: crate::core::IUnknown_Trait {
    fn Clone(&self, pp_enum: MutPtr<IEnumScript>) -> crate::core::HRESULT {
        todo!("Clone")
    }
    fn Next(
        &self,
        celt: u32,
        rgelt: MutPtr<SCRIPTINFO>,
        pcelt_fetched: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("Next")
    }
    fn Reset(&self) -> crate::core::HRESULT {
        todo!("Reset")
    }
    fn Skip(&self, celt: u32) -> crate::core::HRESULT {
        todo!("Skip")
    }
}
impl ::core::clone::Clone for IEnumScript {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IEnumScript {}
impl ::core::cmp::PartialEq for IEnumScript {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumScript {}
impl ::core::fmt::Debug for IEnumScript {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumScript").field(&self.0).finish()
    }
}
impl FromIntoMemory for IEnumScript {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<crate::core::IUnknown>()
    }
}
impl crate::core::ComInterface for IEnumScript {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xae5f1430_388b_11d2_8380_00c04f8f5da1);
}
pub struct IEnumSpellingError(crate::core::IUnknown);
pub trait IEnumSpellingError_Trait: crate::core::IUnknown_Trait {
    fn Next(&self, value: MutPtr<ISpellingError>) -> crate::core::HRESULT {
        todo!("Next")
    }
}
impl ::core::clone::Clone for IEnumSpellingError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IEnumSpellingError {}
impl ::core::cmp::PartialEq for IEnumSpellingError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSpellingError {}
impl ::core::fmt::Debug for IEnumSpellingError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSpellingError").field(&self.0).finish()
    }
}
impl FromIntoMemory for IEnumSpellingError {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<crate::core::IUnknown>()
    }
}
impl crate::core::ComInterface for IEnumSpellingError {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x803e3bd4_2828_4410_8290_418d1d73c762);
}
pub const IME_CMODE_ALPHANUMERIC: u32 = 0u32;
pub const IME_CMODE_CHARCODE: u32 = 32u32;
pub const IME_CMODE_CHINESE: u32 = 1u32;
pub const IME_CMODE_FULLSHAPE: u32 = 8u32;
pub const IME_CMODE_HANGUL: u32 = 1u32;
pub const IME_CMODE_HANJACONVERT: u32 = 64u32;
pub const IME_CMODE_JAPANESE: u32 = 1u32;
pub const IME_CMODE_KATAKANA: u32 = 2u32;
pub const IME_CMODE_LANGUAGE: u32 = 3u32;
pub const IME_CMODE_NATIVE: u32 = 1u32;
pub const IME_CMODE_NATIVESYMBOL: u32 = 128u32;
pub const IME_CMODE_ROMAN: u32 = 16u32;
pub struct IMLangCodePages(crate::core::IUnknown);
pub trait IMLangCodePages_Trait: crate::core::IUnknown_Trait {
    fn GetCharCodePages(&self, ch_src: u16, pdw_code_pages: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetCharCodePages")
    }
    fn GetStrCodePages(
        &self,
        psz_src: PCWSTR,
        cch_src: i32,
        dw_priority_code_pages: u32,
        pdw_code_pages: MutPtr<u32>,
        pcch_code_pages: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("GetStrCodePages")
    }
    fn CodePageToCodePages(
        &self,
        u_code_page: u32,
        pdw_code_pages: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("CodePageToCodePages")
    }
    fn CodePagesToCodePage(
        &self,
        dw_code_pages: u32,
        u_default_code_page: u32,
        pu_code_page: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("CodePagesToCodePage")
    }
}
impl ::core::clone::Clone for IMLangCodePages {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IMLangCodePages {}
impl ::core::cmp::PartialEq for IMLangCodePages {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLangCodePages {}
impl ::core::fmt::Debug for IMLangCodePages {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLangCodePages").field(&self.0).finish()
    }
}
impl FromIntoMemory for IMLangCodePages {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<crate::core::IUnknown>()
    }
}
impl crate::core::ComInterface for IMLangCodePages {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x359f3443_bd4a_11d0_b188_00aa0038c969);
}
pub struct IMLangConvertCharset(crate::core::IUnknown);
pub trait IMLangConvertCharset_Trait: crate::core::IUnknown_Trait {
    fn Initialize(
        &self,
        ui_src_code_page: u32,
        ui_dst_code_page: u32,
        dw_property: u32,
    ) -> crate::core::HRESULT {
        todo!("Initialize")
    }
    fn GetSourceCodePage(&self, pui_src_code_page: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetSourceCodePage")
    }
    fn GetDestinationCodePage(&self, pui_dst_code_page: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetDestinationCodePage")
    }
    fn GetProperty(&self, pdw_property: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetProperty")
    }
    fn DoConversion(
        &self,
        p_src_str: ConstPtr<u8>,
        pc_src_size: MutPtr<u32>,
        p_dst_str: MutPtr<u8>,
        pc_dst_size: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("DoConversion")
    }
    fn DoConversionToUnicode(
        &self,
        p_src_str: PCSTR,
        pc_src_size: MutPtr<u32>,
        p_dst_str: PWSTR,
        pc_dst_size: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("DoConversionToUnicode")
    }
    fn DoConversionFromUnicode(
        &self,
        p_src_str: PCWSTR,
        pc_src_size: MutPtr<u32>,
        p_dst_str: PSTR,
        pc_dst_size: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("DoConversionFromUnicode")
    }
}
impl ::core::clone::Clone for IMLangConvertCharset {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IMLangConvertCharset {}
impl ::core::cmp::PartialEq for IMLangConvertCharset {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLangConvertCharset {}
impl ::core::fmt::Debug for IMLangConvertCharset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLangConvertCharset")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IMLangConvertCharset {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<crate::core::IUnknown>()
    }
}
impl crate::core::ComInterface for IMLangConvertCharset {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xd66d6f98_cdaa_11d0_b822_00c04fc9b31f);
}
pub struct IMLangFontLink(crate::core::IUnknown);
pub trait IMLangFontLink_Trait: IMLangCodePages_Trait {
    fn GetFontCodePages(
        &self,
        h_dc: super::Graphics::Gdi::HDC,
        h_font: super::Graphics::Gdi::HFONT,
        pdw_code_pages: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("GetFontCodePages")
    }
    fn MapFont(
        &self,
        h_dc: super::Graphics::Gdi::HDC,
        dw_code_pages: u32,
        h_src_font: super::Graphics::Gdi::HFONT,
        ph_dest_font: MutPtr<super::Graphics::Gdi::HFONT>,
    ) -> crate::core::HRESULT {
        todo!("MapFont")
    }
    fn ReleaseFont(&self, h_font: super::Graphics::Gdi::HFONT) -> crate::core::HRESULT {
        todo!("ReleaseFont")
    }
    fn ResetFontMapping(&self) -> crate::core::HRESULT {
        todo!("ResetFontMapping")
    }
}
impl ::core::clone::Clone for IMLangFontLink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IMLangFontLink {}
impl ::core::cmp::PartialEq for IMLangFontLink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLangFontLink {}
impl ::core::fmt::Debug for IMLangFontLink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLangFontLink").field(&self.0).finish()
    }
}
impl FromIntoMemory for IMLangFontLink {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<crate::core::IUnknown>()
    }
}
impl crate::core::ComInterface for IMLangFontLink {
    type Super = IMLangCodePages;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x359f3441_bd4a_11d0_b188_00aa0038c969);
}
pub struct IMLangFontLink2(crate::core::IUnknown);
pub trait IMLangFontLink2_Trait: IMLangCodePages_Trait {
    fn GetFontCodePages(
        &self,
        h_dc: super::Graphics::Gdi::HDC,
        h_font: super::Graphics::Gdi::HFONT,
        pdw_code_pages: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("GetFontCodePages")
    }
    fn ReleaseFont(&self, h_font: super::Graphics::Gdi::HFONT) -> crate::core::HRESULT {
        todo!("ReleaseFont")
    }
    fn ResetFontMapping(&self) -> crate::core::HRESULT {
        todo!("ResetFontMapping")
    }
    fn MapFont(
        &self,
        h_dc: super::Graphics::Gdi::HDC,
        dw_code_pages: u32,
        ch_src: u16,
        p_font: MutPtr<super::Graphics::Gdi::HFONT>,
    ) -> crate::core::HRESULT {
        todo!("MapFont")
    }
    fn GetFontUnicodeRanges(
        &self,
        h_dc: super::Graphics::Gdi::HDC,
        pui_ranges: ConstPtr<u32>,
        p_uranges: MutPtr<UNICODERANGE>,
    ) -> crate::core::HRESULT {
        todo!("GetFontUnicodeRanges")
    }
    fn GetScriptFontInfo(
        &self,
        sid: u8,
        dw_flags: u32,
        pui_fonts: MutPtr<u32>,
        p_script_font: MutPtr<tagSCRIPFONTINFO>,
    ) -> crate::core::HRESULT {
        todo!("GetScriptFontInfo")
    }
    fn CodePageToScriptID(&self, ui_code_page: u32, p_sid: MutPtr<u8>) -> crate::core::HRESULT {
        todo!("CodePageToScriptID")
    }
}
impl ::core::clone::Clone for IMLangFontLink2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IMLangFontLink2 {}
impl ::core::cmp::PartialEq for IMLangFontLink2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLangFontLink2 {}
impl ::core::fmt::Debug for IMLangFontLink2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLangFontLink2").field(&self.0).finish()
    }
}
impl FromIntoMemory for IMLangFontLink2 {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<crate::core::IUnknown>()
    }
}
impl crate::core::ComInterface for IMLangFontLink2 {
    type Super = IMLangCodePages;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xdccfc162_2b38_11d2_b7ec_00c04f8f5d9a);
}
pub struct IMLangLineBreakConsole(crate::core::IUnknown);
pub trait IMLangLineBreakConsole_Trait: crate::core::IUnknown_Trait {
    fn BreakLineML(
        &self,
        p_src_ml_str: IMLangString,
        l_src_pos: i32,
        l_src_len: i32,
        c_min_columns: i32,
        c_max_columns: i32,
        pl_line_len: MutPtr<i32>,
        pl_skip_len: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("BreakLineML")
    }
    fn BreakLineW(
        &self,
        locale: u32,
        psz_src: PCWSTR,
        cch_src: i32,
        c_max_columns: i32,
        pcch_line: MutPtr<i32>,
        pcch_skip: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("BreakLineW")
    }
    fn BreakLineA(
        &self,
        locale: u32,
        u_code_page: u32,
        psz_src: PCSTR,
        cch_src: i32,
        c_max_columns: i32,
        pcch_line: MutPtr<i32>,
        pcch_skip: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("BreakLineA")
    }
}
impl ::core::clone::Clone for IMLangLineBreakConsole {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IMLangLineBreakConsole {}
impl ::core::cmp::PartialEq for IMLangLineBreakConsole {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLangLineBreakConsole {}
impl ::core::fmt::Debug for IMLangLineBreakConsole {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLangLineBreakConsole")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IMLangLineBreakConsole {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<crate::core::IUnknown>()
    }
}
impl crate::core::ComInterface for IMLangLineBreakConsole {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xf5be2ee1_bfd7_11d0_b188_00aa0038c969);
}
pub struct IMLangString(crate::core::IUnknown);
pub trait IMLangString_Trait: crate::core::IUnknown_Trait {
    fn Sync(&self, f_no_access: super::Foundation::BOOL) -> crate::core::HRESULT {
        todo!("Sync")
    }
    fn GetLength(&self, pl_len: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("GetLength")
    }
    fn SetMLStr(
        &self,
        l_dest_pos: i32,
        l_dest_len: i32,
        p_src_ml_str: crate::core::IUnknown,
        l_src_pos: i32,
        l_src_len: i32,
    ) -> crate::core::HRESULT {
        todo!("SetMLStr")
    }
    fn GetMLStr(
        &self,
        l_src_pos: i32,
        l_src_len: i32,
        p_unk_outer: crate::core::IUnknown,
        dw_cls_context: u32,
        piid: ConstPtr<crate::core::GUID>,
        pp_dest_ml_str: MutPtr<crate::core::IUnknown>,
        pl_dest_pos: MutPtr<i32>,
        pl_dest_len: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("GetMLStr")
    }
}
impl ::core::clone::Clone for IMLangString {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IMLangString {}
impl ::core::cmp::PartialEq for IMLangString {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLangString {}
impl ::core::fmt::Debug for IMLangString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLangString").field(&self.0).finish()
    }
}
impl FromIntoMemory for IMLangString {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<crate::core::IUnknown>()
    }
}
impl crate::core::ComInterface for IMLangString {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xc04d65ce_b70d_11d0_b188_00aa0038c969);
}
pub struct IMLangStringAStr(crate::core::IUnknown);
pub trait IMLangStringAStr_Trait: IMLangString_Trait {
    fn SetAStr(
        &self,
        l_dest_pos: i32,
        l_dest_len: i32,
        u_code_page: u32,
        psz_src: PCSTR,
        cch_src: i32,
        pcch_actual: MutPtr<i32>,
        pl_actual_len: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("SetAStr")
    }
    fn SetStrBufA(
        &self,
        l_dest_pos: i32,
        l_dest_len: i32,
        u_code_page: u32,
        p_src_buf: IMLangStringBufA,
        pcch_actual: MutPtr<i32>,
        pl_actual_len: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("SetStrBufA")
    }
    fn GetAStr(
        &self,
        l_src_pos: i32,
        l_src_len: i32,
        u_code_page_in: u32,
        pu_code_page_out: MutPtr<u32>,
        psz_dest: PSTR,
        cch_dest: i32,
        pcch_actual: MutPtr<i32>,
        pl_actual_len: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("GetAStr")
    }
    fn GetStrBufA(
        &self,
        l_src_pos: i32,
        l_src_max_len: i32,
        pu_dest_code_page: MutPtr<u32>,
        pp_dest_buf: MutPtr<IMLangStringBufA>,
        pl_dest_len: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("GetStrBufA")
    }
    fn LockAStr(
        &self,
        l_src_pos: i32,
        l_src_len: i32,
        l_flags: i32,
        u_code_page_in: u32,
        cch_request: i32,
        pu_code_page_out: MutPtr<u32>,
        ppsz_dest: MutPtr<PSTR>,
        pcch_dest: MutPtr<i32>,
        pl_dest_len: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("LockAStr")
    }
    fn UnlockAStr(
        &self,
        psz_src: PCSTR,
        cch_src: i32,
        pcch_actual: MutPtr<i32>,
        pl_actual_len: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("UnlockAStr")
    }
    fn SetLocale(&self, l_dest_pos: i32, l_dest_len: i32, locale: u32) -> crate::core::HRESULT {
        todo!("SetLocale")
    }
    fn GetLocale(
        &self,
        l_src_pos: i32,
        l_src_max_len: i32,
        plocale: MutPtr<u32>,
        pl_locale_pos: MutPtr<i32>,
        pl_locale_len: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("GetLocale")
    }
}
impl ::core::clone::Clone for IMLangStringAStr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IMLangStringAStr {}
impl ::core::cmp::PartialEq for IMLangStringAStr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLangStringAStr {}
impl ::core::fmt::Debug for IMLangStringAStr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLangStringAStr").field(&self.0).finish()
    }
}
impl FromIntoMemory for IMLangStringAStr {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<crate::core::IUnknown>()
    }
}
impl crate::core::ComInterface for IMLangStringAStr {
    type Super = IMLangString;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xc04d65d2_b70d_11d0_b188_00aa0038c969);
}
pub struct IMLangStringBufA(crate::core::IUnknown);
pub trait IMLangStringBufA_Trait: crate::core::IUnknown_Trait {
    fn GetStatus(&self, pl_flags: MutPtr<i32>, pcch_buf: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("GetStatus")
    }
    fn LockBuf(
        &self,
        cch_offset: i32,
        cch_max_lock: i32,
        ppsz_buf: MutPtr<ConstPtr<super::Foundation::CHAR>>,
        pcch_buf: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("LockBuf")
    }
    fn UnlockBuf(&self, psz_buf: PCSTR, cch_offset: i32, cch_write: i32) -> crate::core::HRESULT {
        todo!("UnlockBuf")
    }
    fn Insert(
        &self,
        cch_offset: i32,
        cch_max_insert: i32,
        pcch_actual: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("Insert")
    }
    fn Delete(&self, cch_offset: i32, cch_delete: i32) -> crate::core::HRESULT {
        todo!("Delete")
    }
}
impl ::core::clone::Clone for IMLangStringBufA {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IMLangStringBufA {}
impl ::core::cmp::PartialEq for IMLangStringBufA {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLangStringBufA {}
impl ::core::fmt::Debug for IMLangStringBufA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLangStringBufA").field(&self.0).finish()
    }
}
impl FromIntoMemory for IMLangStringBufA {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<crate::core::IUnknown>()
    }
}
impl crate::core::ComInterface for IMLangStringBufA {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xd24acd23_ba72_11d0_b188_00aa0038c969);
}
pub struct IMLangStringBufW(crate::core::IUnknown);
pub trait IMLangStringBufW_Trait: crate::core::IUnknown_Trait {
    fn GetStatus(&self, pl_flags: MutPtr<i32>, pcch_buf: MutPtr<i32>) -> crate::core::HRESULT {
        todo!("GetStatus")
    }
    fn LockBuf(
        &self,
        cch_offset: i32,
        cch_max_lock: i32,
        ppsz_buf: MutPtr<ConstPtr<u16>>,
        pcch_buf: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("LockBuf")
    }
    fn UnlockBuf(&self, psz_buf: PCWSTR, cch_offset: i32, cch_write: i32) -> crate::core::HRESULT {
        todo!("UnlockBuf")
    }
    fn Insert(
        &self,
        cch_offset: i32,
        cch_max_insert: i32,
        pcch_actual: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("Insert")
    }
    fn Delete(&self, cch_offset: i32, cch_delete: i32) -> crate::core::HRESULT {
        todo!("Delete")
    }
}
impl ::core::clone::Clone for IMLangStringBufW {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IMLangStringBufW {}
impl ::core::cmp::PartialEq for IMLangStringBufW {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLangStringBufW {}
impl ::core::fmt::Debug for IMLangStringBufW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLangStringBufW").field(&self.0).finish()
    }
}
impl FromIntoMemory for IMLangStringBufW {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<crate::core::IUnknown>()
    }
}
impl crate::core::ComInterface for IMLangStringBufW {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xd24acd21_ba72_11d0_b188_00aa0038c969);
}
pub struct IMLangStringWStr(crate::core::IUnknown);
pub trait IMLangStringWStr_Trait: IMLangString_Trait {
    fn SetWStr(
        &self,
        l_dest_pos: i32,
        l_dest_len: i32,
        psz_src: PCWSTR,
        cch_src: i32,
        pcch_actual: MutPtr<i32>,
        pl_actual_len: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("SetWStr")
    }
    fn SetStrBufW(
        &self,
        l_dest_pos: i32,
        l_dest_len: i32,
        p_src_buf: IMLangStringBufW,
        pcch_actual: MutPtr<i32>,
        pl_actual_len: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("SetStrBufW")
    }
    fn GetWStr(
        &self,
        l_src_pos: i32,
        l_src_len: i32,
        psz_dest: PWSTR,
        cch_dest: i32,
        pcch_actual: MutPtr<i32>,
        pl_actual_len: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("GetWStr")
    }
    fn GetStrBufW(
        &self,
        l_src_pos: i32,
        l_src_max_len: i32,
        pp_dest_buf: MutPtr<IMLangStringBufW>,
        pl_dest_len: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("GetStrBufW")
    }
    fn LockWStr(
        &self,
        l_src_pos: i32,
        l_src_len: i32,
        l_flags: i32,
        cch_request: i32,
        ppsz_dest: MutPtr<PWSTR>,
        pcch_dest: MutPtr<i32>,
        pl_dest_len: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("LockWStr")
    }
    fn UnlockWStr(
        &self,
        psz_src: PCWSTR,
        cch_src: i32,
        pcch_actual: MutPtr<i32>,
        pl_actual_len: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("UnlockWStr")
    }
    fn SetLocale(&self, l_dest_pos: i32, l_dest_len: i32, locale: u32) -> crate::core::HRESULT {
        todo!("SetLocale")
    }
    fn GetLocale(
        &self,
        l_src_pos: i32,
        l_src_max_len: i32,
        plocale: MutPtr<u32>,
        pl_locale_pos: MutPtr<i32>,
        pl_locale_len: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("GetLocale")
    }
}
impl ::core::clone::Clone for IMLangStringWStr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IMLangStringWStr {}
impl ::core::cmp::PartialEq for IMLangStringWStr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLangStringWStr {}
impl ::core::fmt::Debug for IMLangStringWStr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLangStringWStr").field(&self.0).finish()
    }
}
impl FromIntoMemory for IMLangStringWStr {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<crate::core::IUnknown>()
    }
}
impl crate::core::ComInterface for IMLangStringWStr {
    type Super = IMLangString;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xc04d65d0_b70d_11d0_b188_00aa0038c969);
}
pub struct IMultiLanguage(crate::core::IUnknown);
pub trait IMultiLanguage_Trait: crate::core::IUnknown_Trait {
    fn GetNumberOfCodePageInfo(&self, pc_code_page: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetNumberOfCodePageInfo")
    }
    fn GetCodePageInfo(
        &self,
        ui_code_page: u32,
        p_code_page_info: MutPtr<MIMECPINFO>,
    ) -> crate::core::HRESULT {
        todo!("GetCodePageInfo")
    }
    fn GetFamilyCodePage(
        &self,
        ui_code_page: u32,
        pui_family_code_page: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("GetFamilyCodePage")
    }
    fn EnumCodePages(
        &self,
        grf_flags: u32,
        pp_enum_code_page: MutPtr<IEnumCodePage>,
    ) -> crate::core::HRESULT {
        todo!("EnumCodePages")
    }
    fn GetCharsetInfo(
        &self,
        charset: super::Foundation::BSTR,
        p_charset_info: MutPtr<MIMECSETINFO>,
    ) -> crate::core::HRESULT {
        todo!("GetCharsetInfo")
    }
    fn IsConvertible(&self, dw_src_encoding: u32, dw_dst_encoding: u32) -> crate::core::HRESULT {
        todo!("IsConvertible")
    }
    fn ConvertString(
        &self,
        pdw_mode: MutPtr<u32>,
        dw_src_encoding: u32,
        dw_dst_encoding: u32,
        p_src_str: ConstPtr<u8>,
        pc_src_size: MutPtr<u32>,
        p_dst_str: MutPtr<u8>,
        pc_dst_size: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("ConvertString")
    }
    fn ConvertStringToUnicode(
        &self,
        pdw_mode: MutPtr<u32>,
        dw_encoding: u32,
        p_src_str: PCSTR,
        pc_src_size: MutPtr<u32>,
        p_dst_str: PWSTR,
        pc_dst_size: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("ConvertStringToUnicode")
    }
    fn ConvertStringFromUnicode(
        &self,
        pdw_mode: MutPtr<u32>,
        dw_encoding: u32,
        p_src_str: PCWSTR,
        pc_src_size: MutPtr<u32>,
        p_dst_str: PSTR,
        pc_dst_size: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("ConvertStringFromUnicode")
    }
    fn ConvertStringReset(&self) -> crate::core::HRESULT {
        todo!("ConvertStringReset")
    }
    fn GetRfc1766FromLcid(
        &self,
        locale: u32,
        pbstr_rfc_1766: MutPtr<super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetRfc1766FromLcid")
    }
    fn GetLcidFromRfc1766(
        &self,
        p_locale: MutPtr<u32>,
        bstr_rfc_1766: super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("GetLcidFromRfc1766")
    }
    fn EnumRfc1766(&self, pp_enum_rfc_1766: MutPtr<IEnumRfc1766>) -> crate::core::HRESULT {
        todo!("EnumRfc1766")
    }
    fn GetRfc1766Info(
        &self,
        locale: u32,
        p_rfc_1766_info: MutPtr<RFC1766INFO>,
    ) -> crate::core::HRESULT {
        todo!("GetRfc1766Info")
    }
    fn CreateConvertCharset(
        &self,
        ui_src_code_page: u32,
        ui_dst_code_page: u32,
        dw_property: u32,
        pp_m_lang_convert_charset: MutPtr<IMLangConvertCharset>,
    ) -> crate::core::HRESULT {
        todo!("CreateConvertCharset")
    }
}
impl ::core::clone::Clone for IMultiLanguage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IMultiLanguage {}
impl ::core::cmp::PartialEq for IMultiLanguage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMultiLanguage {}
impl ::core::fmt::Debug for IMultiLanguage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMultiLanguage").field(&self.0).finish()
    }
}
impl FromIntoMemory for IMultiLanguage {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<crate::core::IUnknown>()
    }
}
impl crate::core::ComInterface for IMultiLanguage {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x275c23e1_3747_11d0_9fea_00aa003f8646);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IMultiLanguage2(crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IMultiLanguage2_Trait: crate::core::IUnknown_Trait {
    fn GetNumberOfCodePageInfo(&self, pc_code_page: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetNumberOfCodePageInfo")
    }
    fn GetCodePageInfo(
        &self,
        ui_code_page: u32,
        lang_id: u16,
        p_code_page_info: MutPtr<MIMECPINFO>,
    ) -> crate::core::HRESULT {
        todo!("GetCodePageInfo")
    }
    fn GetFamilyCodePage(
        &self,
        ui_code_page: u32,
        pui_family_code_page: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("GetFamilyCodePage")
    }
    fn EnumCodePages(
        &self,
        grf_flags: u32,
        lang_id: u16,
        pp_enum_code_page: MutPtr<IEnumCodePage>,
    ) -> crate::core::HRESULT {
        todo!("EnumCodePages")
    }
    fn GetCharsetInfo(
        &self,
        charset: super::Foundation::BSTR,
        p_charset_info: MutPtr<MIMECSETINFO>,
    ) -> crate::core::HRESULT {
        todo!("GetCharsetInfo")
    }
    fn IsConvertible(&self, dw_src_encoding: u32, dw_dst_encoding: u32) -> crate::core::HRESULT {
        todo!("IsConvertible")
    }
    fn ConvertString(
        &self,
        pdw_mode: MutPtr<u32>,
        dw_src_encoding: u32,
        dw_dst_encoding: u32,
        p_src_str: ConstPtr<u8>,
        pc_src_size: MutPtr<u32>,
        p_dst_str: MutPtr<u8>,
        pc_dst_size: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("ConvertString")
    }
    fn ConvertStringToUnicode(
        &self,
        pdw_mode: MutPtr<u32>,
        dw_encoding: u32,
        p_src_str: PCSTR,
        pc_src_size: MutPtr<u32>,
        p_dst_str: PWSTR,
        pc_dst_size: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("ConvertStringToUnicode")
    }
    fn ConvertStringFromUnicode(
        &self,
        pdw_mode: MutPtr<u32>,
        dw_encoding: u32,
        p_src_str: PCWSTR,
        pc_src_size: MutPtr<u32>,
        p_dst_str: PSTR,
        pc_dst_size: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("ConvertStringFromUnicode")
    }
    fn ConvertStringReset(&self) -> crate::core::HRESULT {
        todo!("ConvertStringReset")
    }
    fn GetRfc1766FromLcid(
        &self,
        locale: u32,
        pbstr_rfc_1766: MutPtr<super::Foundation::BSTR>,
    ) -> crate::core::HRESULT {
        todo!("GetRfc1766FromLcid")
    }
    fn GetLcidFromRfc1766(
        &self,
        p_locale: MutPtr<u32>,
        bstr_rfc_1766: super::Foundation::BSTR,
    ) -> crate::core::HRESULT {
        todo!("GetLcidFromRfc1766")
    }
    fn EnumRfc1766(
        &self,
        lang_id: u16,
        pp_enum_rfc_1766: MutPtr<IEnumRfc1766>,
    ) -> crate::core::HRESULT {
        todo!("EnumRfc1766")
    }
    fn GetRfc1766Info(
        &self,
        locale: u32,
        lang_id: u16,
        p_rfc_1766_info: MutPtr<RFC1766INFO>,
    ) -> crate::core::HRESULT {
        todo!("GetRfc1766Info")
    }
    fn CreateConvertCharset(
        &self,
        ui_src_code_page: u32,
        ui_dst_code_page: u32,
        dw_property: u32,
        pp_m_lang_convert_charset: MutPtr<IMLangConvertCharset>,
    ) -> crate::core::HRESULT {
        todo!("CreateConvertCharset")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn ConvertStringInIStream(
        &self,
        pdw_mode: MutPtr<u32>,
        dw_flag: u32,
        lp_fall_back: PCWSTR,
        dw_src_encoding: u32,
        dw_dst_encoding: u32,
        pstm_in: super::System::Com::IStream,
        pstm_out: super::System::Com::IStream,
    ) -> crate::core::HRESULT {
        todo!("ConvertStringInIStream")
    }
    fn ConvertStringToUnicodeEx(
        &self,
        pdw_mode: MutPtr<u32>,
        dw_encoding: u32,
        p_src_str: PCSTR,
        pc_src_size: MutPtr<u32>,
        p_dst_str: PWSTR,
        pc_dst_size: MutPtr<u32>,
        dw_flag: u32,
        lp_fall_back: PCWSTR,
    ) -> crate::core::HRESULT {
        todo!("ConvertStringToUnicodeEx")
    }
    fn ConvertStringFromUnicodeEx(
        &self,
        pdw_mode: MutPtr<u32>,
        dw_encoding: u32,
        p_src_str: PCWSTR,
        pc_src_size: MutPtr<u32>,
        p_dst_str: PSTR,
        pc_dst_size: MutPtr<u32>,
        dw_flag: u32,
        lp_fall_back: PCWSTR,
    ) -> crate::core::HRESULT {
        todo!("ConvertStringFromUnicodeEx")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn DetectCodepageInIStream(
        &self,
        dw_flag: u32,
        dw_pref_win_code_page: u32,
        pstm_in: super::System::Com::IStream,
        lp_encoding: MutPtr<DetectEncodingInfo>,
        pn_scores: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("DetectCodepageInIStream")
    }
    fn DetectInputCodepage(
        &self,
        dw_flag: u32,
        dw_pref_win_code_page: u32,
        p_src_str: PCSTR,
        pc_src_size: MutPtr<i32>,
        lp_encoding: MutPtr<DetectEncodingInfo>,
        pn_scores: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("DetectInputCodepage")
    }
    fn ValidateCodePage(
        &self,
        ui_code_page: u32,
        hwnd: super::Foundation::HWND,
    ) -> crate::core::HRESULT {
        todo!("ValidateCodePage")
    }
    fn GetCodePageDescription(
        &self,
        ui_code_page: u32,
        lcid: u32,
        lp_wide_char_str: PWSTR,
        cch_wide_char: i32,
    ) -> crate::core::HRESULT {
        todo!("GetCodePageDescription")
    }
    fn IsCodePageInstallable(&self, ui_code_page: u32) -> crate::core::HRESULT {
        todo!("IsCodePageInstallable")
    }
    fn SetMimeDBSource(&self, dw_source: MIMECONTF) -> crate::core::HRESULT {
        todo!("SetMimeDBSource")
    }
    fn GetNumberOfScripts(&self, pn_scripts: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("GetNumberOfScripts")
    }
    fn EnumScripts(
        &self,
        dw_flags: u32,
        lang_id: u16,
        pp_enum_script: MutPtr<IEnumScript>,
    ) -> crate::core::HRESULT {
        todo!("EnumScripts")
    }
    fn ValidateCodePageEx(
        &self,
        ui_code_page: u32,
        hwnd: super::Foundation::HWND,
        dwf_iod_control: u32,
    ) -> crate::core::HRESULT {
        todo!("ValidateCodePageEx")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IMultiLanguage2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IMultiLanguage2 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IMultiLanguage2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IMultiLanguage2 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IMultiLanguage2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMultiLanguage2").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IMultiLanguage2 {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<crate::core::IUnknown>()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IMultiLanguage2 {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xdccfc164_2b38_11d2_b7ec_00c04f8f5d9a);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IMultiLanguage3(crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IMultiLanguage3_Trait: IMultiLanguage2_Trait {
    fn DetectOutboundCodePage(
        &self,
        dw_flags: u32,
        lp_wide_char_str: PCWSTR,
        cch_wide_char: u32,
        pui_preferred_code_pages: ConstPtr<u32>,
        n_preferred_code_pages: u32,
        pui_detected_code_pages: MutPtr<u32>,
        pn_detected_code_pages: MutPtr<u32>,
        lp_special_char: PCWSTR,
    ) -> crate::core::HRESULT {
        todo!("DetectOutboundCodePage")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn DetectOutboundCodePageInIStream(
        &self,
        dw_flags: u32,
        p_str_in: super::System::Com::IStream,
        pui_preferred_code_pages: ConstPtr<u32>,
        n_preferred_code_pages: u32,
        pui_detected_code_pages: MutPtr<u32>,
        pn_detected_code_pages: MutPtr<u32>,
        lp_special_char: PCWSTR,
    ) -> crate::core::HRESULT {
        todo!("DetectOutboundCodePageInIStream")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IMultiLanguage3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IMultiLanguage3 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IMultiLanguage3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IMultiLanguage3 {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IMultiLanguage3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMultiLanguage3").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IMultiLanguage3 {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<crate::core::IUnknown>()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com.StructuredStorage'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IMultiLanguage3 {
    type Super = IMultiLanguage2;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x4e5868ab_b157_4623_9acc_6a1d9caebe04);
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct IOptionDescription(crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait IOptionDescription_Trait: crate::core::IUnknown_Trait {
    fn get_Id(&self, value: MutPtr<PWSTR>) -> crate::core::HRESULT {
        todo!("get_Id")
    }
    fn get_Heading(&self, value: MutPtr<PWSTR>) -> crate::core::HRESULT {
        todo!("get_Heading")
    }
    fn get_Description(&self, value: MutPtr<PWSTR>) -> crate::core::HRESULT {
        todo!("get_Description")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_Labels(&self, value: MutPtr<super::System::Com::IEnumString>) -> crate::core::HRESULT {
        todo!("get_Labels")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for IOptionDescription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for IOptionDescription {}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for IOptionDescription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for IOptionDescription {}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for IOptionDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOptionDescription").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for IOptionDescription {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<crate::core::IUnknown>()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for IOptionDescription {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x432e5f85_35cf_4606_a801_6f70277e1d7a);
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IS_TEXT_UNICODE_RESULT(pub u32);
pub const IS_TEXT_UNICODE_ASCII16: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(1u32);
pub const IS_TEXT_UNICODE_REVERSE_ASCII16: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(16u32);
pub const IS_TEXT_UNICODE_STATISTICS: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(2u32);
pub const IS_TEXT_UNICODE_REVERSE_STATISTICS: IS_TEXT_UNICODE_RESULT =
    IS_TEXT_UNICODE_RESULT(32u32);
pub const IS_TEXT_UNICODE_CONTROLS: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(4u32);
pub const IS_TEXT_UNICODE_REVERSE_CONTROLS: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(64u32);
pub const IS_TEXT_UNICODE_SIGNATURE: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(8u32);
pub const IS_TEXT_UNICODE_REVERSE_SIGNATURE: IS_TEXT_UNICODE_RESULT =
    IS_TEXT_UNICODE_RESULT(128u32);
pub const IS_TEXT_UNICODE_ILLEGAL_CHARS: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(256u32);
pub const IS_TEXT_UNICODE_ODD_LENGTH: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(512u32);
pub const IS_TEXT_UNICODE_NULL_BYTES: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(4096u32);
pub const IS_TEXT_UNICODE_UNICODE_MASK: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(15u32);
pub const IS_TEXT_UNICODE_REVERSE_MASK: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(240u32);
pub const IS_TEXT_UNICODE_NOT_UNICODE_MASK: IS_TEXT_UNICODE_RESULT =
    IS_TEXT_UNICODE_RESULT(3840u32);
pub const IS_TEXT_UNICODE_NOT_ASCII_MASK: IS_TEXT_UNICODE_RESULT = IS_TEXT_UNICODE_RESULT(61440u32);
impl ::core::marker::Copy for IS_TEXT_UNICODE_RESULT {}
impl ::core::clone::Clone for IS_TEXT_UNICODE_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IS_TEXT_UNICODE_RESULT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IS_TEXT_UNICODE_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IS_TEXT_UNICODE_RESULT")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for IS_TEXT_UNICODE_RESULT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IS_TEXT_UNICODE_RESULT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IS_TEXT_UNICODE_RESULT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IS_TEXT_UNICODE_RESULT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IS_TEXT_UNICODE_RESULT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for IS_TEXT_UNICODE_RESULT {
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
pub struct IS_VALID_LOCALE_FLAGS(pub u32);
pub const LCID_INSTALLED: IS_VALID_LOCALE_FLAGS = IS_VALID_LOCALE_FLAGS(1u32);
pub const LCID_SUPPORTED: IS_VALID_LOCALE_FLAGS = IS_VALID_LOCALE_FLAGS(2u32);
impl ::core::marker::Copy for IS_VALID_LOCALE_FLAGS {}
impl ::core::clone::Clone for IS_VALID_LOCALE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IS_VALID_LOCALE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IS_VALID_LOCALE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IS_VALID_LOCALE_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IS_VALID_LOCALE_FLAGS {
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
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ISpellCheckProvider(crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait ISpellCheckProvider_Trait: crate::core::IUnknown_Trait {
    fn get_LanguageTag(&self, value: MutPtr<PWSTR>) -> crate::core::HRESULT {
        todo!("get_LanguageTag")
    }
    fn Check(&self, text: PCWSTR, value: MutPtr<IEnumSpellingError>) -> crate::core::HRESULT {
        todo!("Check")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Suggest(
        &self,
        word: PCWSTR,
        value: MutPtr<super::System::Com::IEnumString>,
    ) -> crate::core::HRESULT {
        todo!("Suggest")
    }
    fn GetOptionValue(&self, option_id: PCWSTR, value: MutPtr<u8>) -> crate::core::HRESULT {
        todo!("GetOptionValue")
    }
    fn SetOptionValue(&self, option_id: PCWSTR, value: u8) -> crate::core::HRESULT {
        todo!("SetOptionValue")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_OptionIds(
        &self,
        value: MutPtr<super::System::Com::IEnumString>,
    ) -> crate::core::HRESULT {
        todo!("get_OptionIds")
    }
    fn get_Id(&self, value: MutPtr<PWSTR>) -> crate::core::HRESULT {
        todo!("get_Id")
    }
    fn get_LocalizedName(&self, value: MutPtr<PWSTR>) -> crate::core::HRESULT {
        todo!("get_LocalizedName")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetOptionDescription(
        &self,
        option_id: PCWSTR,
        value: MutPtr<IOptionDescription>,
    ) -> crate::core::HRESULT {
        todo!("GetOptionDescription")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn InitializeWordlist(
        &self,
        wordlist_type: WORDLIST_TYPE,
        words: super::System::Com::IEnumString,
    ) -> crate::core::HRESULT {
        todo!("InitializeWordlist")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ISpellCheckProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ISpellCheckProvider {}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ISpellCheckProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ISpellCheckProvider {}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ISpellCheckProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpellCheckProvider").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ISpellCheckProvider {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<crate::core::IUnknown>()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for ISpellCheckProvider {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x73e976e0_8ed4_4eb1_80d7_1be0a16b0c38);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ISpellCheckProviderFactory(crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait ISpellCheckProviderFactory_Trait: crate::core::IUnknown_Trait {
    #[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_SupportedLanguages(
        &self,
        value: MutPtr<super::System::Com::IEnumString>,
    ) -> crate::core::HRESULT {
        todo!("get_SupportedLanguages")
    }
    fn IsSupported(
        &self,
        language_tag: PCWSTR,
        value: MutPtr<super::Foundation::BOOL>,
    ) -> crate::core::HRESULT {
        todo!("IsSupported")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateSpellCheckProvider(
        &self,
        language_tag: PCWSTR,
        value: MutPtr<ISpellCheckProvider>,
    ) -> crate::core::HRESULT {
        todo!("CreateSpellCheckProvider")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ISpellCheckProviderFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ISpellCheckProviderFactory {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ISpellCheckProviderFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ISpellCheckProviderFactory {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ISpellCheckProviderFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpellCheckProviderFactory")
            .field(&self.0)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ISpellCheckProviderFactory {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<crate::core::IUnknown>()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for ISpellCheckProviderFactory {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x9f671e11_77d6_4c92_aefb_615215e3a4be);
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ISpellChecker(crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait ISpellChecker_Trait: crate::core::IUnknown_Trait {
    fn get_LanguageTag(&self, value: MutPtr<PWSTR>) -> crate::core::HRESULT {
        todo!("get_LanguageTag")
    }
    fn Check(&self, text: PCWSTR, value: MutPtr<IEnumSpellingError>) -> crate::core::HRESULT {
        todo!("Check")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Suggest(
        &self,
        word: PCWSTR,
        value: MutPtr<super::System::Com::IEnumString>,
    ) -> crate::core::HRESULT {
        todo!("Suggest")
    }
    fn Add(&self, word: PCWSTR) -> crate::core::HRESULT {
        todo!("Add")
    }
    fn Ignore(&self, word: PCWSTR) -> crate::core::HRESULT {
        todo!("Ignore")
    }
    fn AutoCorrect(&self, from: PCWSTR, to: PCWSTR) -> crate::core::HRESULT {
        todo!("AutoCorrect")
    }
    fn GetOptionValue(&self, option_id: PCWSTR, value: MutPtr<u8>) -> crate::core::HRESULT {
        todo!("GetOptionValue")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_OptionIds(
        &self,
        value: MutPtr<super::System::Com::IEnumString>,
    ) -> crate::core::HRESULT {
        todo!("get_OptionIds")
    }
    fn get_Id(&self, value: MutPtr<PWSTR>) -> crate::core::HRESULT {
        todo!("get_Id")
    }
    fn get_LocalizedName(&self, value: MutPtr<PWSTR>) -> crate::core::HRESULT {
        todo!("get_LocalizedName")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn add_SpellCheckerChanged(
        &self,
        handler: ISpellCheckerChangedEventHandler,
        event_cookie: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("add_SpellCheckerChanged")
    }
    fn remove_SpellCheckerChanged(&self, event_cookie: u32) -> crate::core::HRESULT {
        todo!("remove_SpellCheckerChanged")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetOptionDescription(
        &self,
        option_id: PCWSTR,
        value: MutPtr<IOptionDescription>,
    ) -> crate::core::HRESULT {
        todo!("GetOptionDescription")
    }
    fn ComprehensiveCheck(
        &self,
        text: PCWSTR,
        value: MutPtr<IEnumSpellingError>,
    ) -> crate::core::HRESULT {
        todo!("ComprehensiveCheck")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ISpellChecker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ISpellChecker {}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ISpellChecker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ISpellChecker {}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ISpellChecker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpellChecker").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ISpellChecker {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<crate::core::IUnknown>()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for ISpellChecker {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xb6fd0b71_e2bc_4653_8d05_f197e412770b);
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ISpellChecker2(crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait ISpellChecker2_Trait: ISpellChecker_Trait {
    fn Remove(&self, word: PCWSTR) -> crate::core::HRESULT {
        todo!("Remove")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ISpellChecker2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ISpellChecker2 {}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ISpellChecker2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ISpellChecker2 {}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ISpellChecker2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpellChecker2").field(&self.0).finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ISpellChecker2 {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<crate::core::IUnknown>()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for ISpellChecker2 {
    type Super = ISpellChecker;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xe7ed1c71_87f7_4378_a840_c9200dacee47);
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ISpellCheckerChangedEventHandler(crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait ISpellCheckerChangedEventHandler_Trait: crate::core::IUnknown_Trait {
    #[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn Invoke(&self, sender: ISpellChecker) -> crate::core::HRESULT {
        todo!("Invoke")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ISpellCheckerChangedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ISpellCheckerChangedEventHandler {}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ISpellCheckerChangedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ISpellCheckerChangedEventHandler {}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ISpellCheckerChangedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpellCheckerChangedEventHandler")
            .field(&self.0)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ISpellCheckerChangedEventHandler {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<crate::core::IUnknown>()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for ISpellCheckerChangedEventHandler {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x0b83a5b0_792f_4eab_9799_acf52c5ed08a);
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ISpellCheckerFactory(crate::core::IUnknown);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub trait ISpellCheckerFactory_Trait: crate::core::IUnknown_Trait {
    #[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn get_SupportedLanguages(
        &self,
        value: MutPtr<super::System::Com::IEnumString>,
    ) -> crate::core::HRESULT {
        todo!("get_SupportedLanguages")
    }
    fn IsSupported(
        &self,
        language_tag: PCWSTR,
        value: MutPtr<super::Foundation::BOOL>,
    ) -> crate::core::HRESULT {
        todo!("IsSupported")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateSpellChecker(
        &self,
        language_tag: PCWSTR,
        value: MutPtr<ISpellChecker>,
    ) -> crate::core::HRESULT {
        todo!("CreateSpellChecker")
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ISpellCheckerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ISpellCheckerFactory {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ISpellCheckerFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ISpellCheckerFactory {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ISpellCheckerFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpellCheckerFactory")
            .field(&self.0)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ISpellCheckerFactory {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<crate::core::IUnknown>()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl crate::core::ComInterface for ISpellCheckerFactory {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0x8e018a9d_2415_4677_bf08_794ea61f94bb);
}
pub struct ISpellingError(crate::core::IUnknown);
pub trait ISpellingError_Trait: crate::core::IUnknown_Trait {
    fn get_StartIndex(&self, value: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_StartIndex")
    }
    fn get_Length(&self, value: MutPtr<u32>) -> crate::core::HRESULT {
        todo!("get_Length")
    }
    fn get_CorrectiveAction(&self, value: MutPtr<CORRECTIVE_ACTION>) -> crate::core::HRESULT {
        todo!("get_CorrectiveAction")
    }
    fn get_Replacement(&self, value: MutPtr<PWSTR>) -> crate::core::HRESULT {
        todo!("get_Replacement")
    }
}
impl ::core::clone::Clone for ISpellingError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for ISpellingError {}
impl ::core::cmp::PartialEq for ISpellingError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpellingError {}
impl ::core::fmt::Debug for ISpellingError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpellingError").field(&self.0).finish()
    }
}
impl FromIntoMemory for ISpellingError {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<crate::core::IUnknown>()
    }
}
impl crate::core::ComInterface for ISpellingError {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xb7c82d61_fbe8_4b47_9b27_6c0d2e0de0a3);
}
pub struct IUserDictionariesRegistrar(crate::core::IUnknown);
pub trait IUserDictionariesRegistrar_Trait: crate::core::IUnknown_Trait {
    fn RegisterUserDictionary(
        &self,
        dictionary_path: PCWSTR,
        language_tag: PCWSTR,
    ) -> crate::core::HRESULT {
        todo!("RegisterUserDictionary")
    }
    fn UnregisterUserDictionary(
        &self,
        dictionary_path: PCWSTR,
        language_tag: PCWSTR,
    ) -> crate::core::HRESULT {
        todo!("UnregisterUserDictionary")
    }
}
impl ::core::clone::Clone for IUserDictionariesRegistrar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::marker::Copy for IUserDictionariesRegistrar {}
impl ::core::cmp::PartialEq for IUserDictionariesRegistrar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUserDictionariesRegistrar {}
impl ::core::fmt::Debug for IUserDictionariesRegistrar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUserDictionariesRegistrar")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for IUserDictionariesRegistrar {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<crate::core::IUnknown>()
    }
}
impl crate::core::ComInterface for IUserDictionariesRegistrar {
    type Super = crate::core::IUnknown;
    const IID: crate::core::GUID =
        crate::core::GUID::from_u128(0xaa176b85_0e12_4844_8e1a_eef1da77f586);
}
pub type LANGGROUPLOCALE_ENUMPROCA =
    StdCallFnPtr<(u32, u32, PCSTR, PtrDiffRepr), super::Foundation::BOOL>;
pub type LANGGROUPLOCALE_ENUMPROCW =
    StdCallFnPtr<(u32, u32, PCWSTR, PtrDiffRepr), super::Foundation::BOOL>;
pub type LANGUAGEGROUP_ENUMPROCA =
    StdCallFnPtr<(u32, PCSTR, PCSTR, u32, PtrDiffRepr), super::Foundation::BOOL>;
pub type LANGUAGEGROUP_ENUMPROCW =
    StdCallFnPtr<(u32, PCWSTR, PCWSTR, u32, PtrDiffRepr), super::Foundation::BOOL>;
pub const LCID_ALTERNATE_SORTS: u32 = 4u32;
pub const LCMAP_BYTEREV: u32 = 2048u32;
pub const LCMAP_FULLWIDTH: u32 = 8388608u32;
pub const LCMAP_HALFWIDTH: u32 = 4194304u32;
pub const LCMAP_HASH: u32 = 262144u32;
pub const LCMAP_HIRAGANA: u32 = 1048576u32;
pub const LCMAP_KATAKANA: u32 = 2097152u32;
pub const LCMAP_LINGUISTIC_CASING: u32 = 16777216u32;
pub const LCMAP_LOWERCASE: u32 = 256u32;
pub const LCMAP_SIMPLIFIED_CHINESE: u32 = 33554432u32;
pub const LCMAP_SORTHANDLE: u32 = 536870912u32;
pub const LCMAP_SORTKEY: u32 = 1024u32;
pub const LCMAP_TITLECASE: u32 = 768u32;
pub const LCMAP_TRADITIONAL_CHINESE: u32 = 67108864u32;
pub const LCMAP_UPPERCASE: u32 = 512u32;
pub const LGRPID_ARABIC: u32 = 13u32;
pub const LGRPID_ARMENIAN: u32 = 17u32;
pub const LGRPID_BALTIC: u32 = 3u32;
pub const LGRPID_CENTRAL_EUROPE: u32 = 2u32;
pub const LGRPID_CYRILLIC: u32 = 5u32;
pub const LGRPID_GEORGIAN: u32 = 16u32;
pub const LGRPID_GREEK: u32 = 4u32;
pub const LGRPID_HEBREW: u32 = 12u32;
pub const LGRPID_INDIC: u32 = 15u32;
pub const LGRPID_JAPANESE: u32 = 7u32;
pub const LGRPID_KOREAN: u32 = 8u32;
pub const LGRPID_SIMPLIFIED_CHINESE: u32 = 10u32;
pub const LGRPID_THAI: u32 = 11u32;
pub const LGRPID_TRADITIONAL_CHINESE: u32 = 9u32;
pub const LGRPID_TURKIC: u32 = 6u32;
pub const LGRPID_TURKISH: u32 = 6u32;
pub const LGRPID_VIETNAMESE: u32 = 14u32;
pub const LGRPID_WESTERN_EUROPE: u32 = 1u32;
pub struct LOCALESIGNATURE {
    pub lsUsb: [u32; 4],
    pub lsCsbDefault: [u32; 2],
    pub lsCsbSupported: [u32; 2],
}
impl ::core::marker::Copy for LOCALESIGNATURE {}
impl ::core::clone::Clone for LOCALESIGNATURE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LOCALESIGNATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOCALESIGNATURE")
            .field("lsUsb", &self.lsUsb)
            .field("lsCsbDefault", &self.lsCsbDefault)
            .field("lsCsbSupported", &self.lsCsbSupported)
            .finish()
    }
}
impl ::core::cmp::PartialEq for LOCALESIGNATURE {
    fn eq(&self, other: &Self) -> bool {
        self.lsUsb == other.lsUsb
            && self.lsCsbDefault == other.lsCsbDefault
            && self.lsCsbSupported == other.lsCsbSupported
    }
}
impl ::core::cmp::Eq for LOCALESIGNATURE {}
impl FromIntoMemory for LOCALESIGNATURE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32u32 as usize);
        let f_lsUsb = <[u32; 4] as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_lsCsbDefault = <[u32; 2] as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_lsCsbSupported = <[u32; 2] as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        Self {
            lsUsb: f_lsUsb,
            lsCsbDefault: f_lsCsbDefault,
            lsCsbSupported: f_lsCsbSupported,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32u32 as usize);
        FromIntoMemory::into_bytes(self.lsUsb, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.lsCsbDefault, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.lsCsbSupported, &mut into[24..24 + 8]);
    }
    fn size() -> usize {
        32u32 as usize
    }
}
pub const LOCALE_ALL: u32 = 0u32;
pub const LOCALE_ALLOW_NEUTRAL_NAMES: u32 = 134217728u32;
pub const LOCALE_ALTERNATE_SORTS: u32 = 4u32;
pub type LOCALE_ENUMPROCA = StdCallFnPtr<(PCSTR,), super::Foundation::BOOL>;
pub type LOCALE_ENUMPROCEX =
    StdCallFnPtr<(PCWSTR, u32, super::Foundation::LPARAM), super::Foundation::BOOL>;
pub type LOCALE_ENUMPROCW = StdCallFnPtr<(PCWSTR,), super::Foundation::BOOL>;
pub const LOCALE_FONTSIGNATURE: u32 = 88u32;
pub const LOCALE_ICALENDARTYPE: u32 = 4105u32;
pub const LOCALE_ICENTURY: u32 = 36u32;
pub const LOCALE_ICONSTRUCTEDLOCALE: u32 = 125u32;
pub const LOCALE_ICOUNTRY: u32 = 5u32;
pub const LOCALE_ICURRDIGITS: u32 = 25u32;
pub const LOCALE_ICURRENCY: u32 = 27u32;
pub const LOCALE_IDATE: u32 = 33u32;
pub const LOCALE_IDAYLZERO: u32 = 38u32;
pub const LOCALE_IDEFAULTANSICODEPAGE: u32 = 4100u32;
pub const LOCALE_IDEFAULTCODEPAGE: u32 = 11u32;
pub const LOCALE_IDEFAULTCOUNTRY: u32 = 10u32;
pub const LOCALE_IDEFAULTEBCDICCODEPAGE: u32 = 4114u32;
pub const LOCALE_IDEFAULTLANGUAGE: u32 = 9u32;
pub const LOCALE_IDEFAULTMACCODEPAGE: u32 = 4113u32;
pub const LOCALE_IDIALINGCODE: u32 = 5u32;
pub const LOCALE_IDIGITS: u32 = 17u32;
pub const LOCALE_IDIGITSUBSTITUTION: u32 = 4116u32;
pub const LOCALE_IFIRSTDAYOFWEEK: u32 = 4108u32;
pub const LOCALE_IFIRSTWEEKOFYEAR: u32 = 4109u32;
pub const LOCALE_IGEOID: u32 = 91u32;
pub const LOCALE_IINTLCURRDIGITS: u32 = 26u32;
pub const LOCALE_ILANGUAGE: u32 = 1u32;
pub const LOCALE_ILDATE: u32 = 34u32;
pub const LOCALE_ILZERO: u32 = 18u32;
pub const LOCALE_IMEASURE: u32 = 13u32;
pub const LOCALE_IMONLZERO: u32 = 39u32;
pub const LOCALE_INEGATIVEPERCENT: u32 = 116u32;
pub const LOCALE_INEGCURR: u32 = 28u32;
pub const LOCALE_INEGNUMBER: u32 = 4112u32;
pub const LOCALE_INEGSEPBYSPACE: u32 = 87u32;
pub const LOCALE_INEGSIGNPOSN: u32 = 83u32;
pub const LOCALE_INEGSYMPRECEDES: u32 = 86u32;
pub const LOCALE_INEUTRAL: u32 = 113u32;
pub const LOCALE_IOPTIONALCALENDAR: u32 = 4107u32;
pub const LOCALE_IPAPERSIZE: u32 = 4106u32;
pub const LOCALE_IPOSITIVEPERCENT: u32 = 117u32;
pub const LOCALE_IPOSSEPBYSPACE: u32 = 85u32;
pub const LOCALE_IPOSSIGNPOSN: u32 = 82u32;
pub const LOCALE_IPOSSYMPRECEDES: u32 = 84u32;
pub const LOCALE_IREADINGLAYOUT: u32 = 112u32;
pub const LOCALE_ITIME: u32 = 35u32;
pub const LOCALE_ITIMEMARKPOSN: u32 = 4101u32;
pub const LOCALE_ITLZERO: u32 = 37u32;
pub const LOCALE_IUSEUTF8LEGACYACP: u32 = 1638u32;
pub const LOCALE_IUSEUTF8LEGACYOEMCP: u32 = 2457u32;
pub const LOCALE_NAME_INVARIANT: &'static str = "";
pub const LOCALE_NAME_SYSTEM_DEFAULT: &'static str = "!x-sys-default-locale";
pub const LOCALE_NEUTRALDATA: u32 = 16u32;
pub const LOCALE_NOUSEROVERRIDE: u32 = 2147483648u32;
pub const LOCALE_REPLACEMENT: u32 = 8u32;
pub const LOCALE_RETURN_GENITIVE_NAMES: u32 = 268435456u32;
pub const LOCALE_RETURN_NUMBER: u32 = 536870912u32;
pub const LOCALE_S1159: u32 = 40u32;
pub const LOCALE_S2359: u32 = 41u32;
pub const LOCALE_SABBREVCTRYNAME: u32 = 7u32;
pub const LOCALE_SABBREVDAYNAME1: u32 = 49u32;
pub const LOCALE_SABBREVDAYNAME2: u32 = 50u32;
pub const LOCALE_SABBREVDAYNAME3: u32 = 51u32;
pub const LOCALE_SABBREVDAYNAME4: u32 = 52u32;
pub const LOCALE_SABBREVDAYNAME5: u32 = 53u32;
pub const LOCALE_SABBREVDAYNAME6: u32 = 54u32;
pub const LOCALE_SABBREVDAYNAME7: u32 = 55u32;
pub const LOCALE_SABBREVLANGNAME: u32 = 3u32;
pub const LOCALE_SABBREVMONTHNAME1: u32 = 68u32;
pub const LOCALE_SABBREVMONTHNAME10: u32 = 77u32;
pub const LOCALE_SABBREVMONTHNAME11: u32 = 78u32;
pub const LOCALE_SABBREVMONTHNAME12: u32 = 79u32;
pub const LOCALE_SABBREVMONTHNAME13: u32 = 4111u32;
pub const LOCALE_SABBREVMONTHNAME2: u32 = 69u32;
pub const LOCALE_SABBREVMONTHNAME3: u32 = 70u32;
pub const LOCALE_SABBREVMONTHNAME4: u32 = 71u32;
pub const LOCALE_SABBREVMONTHNAME5: u32 = 72u32;
pub const LOCALE_SABBREVMONTHNAME6: u32 = 73u32;
pub const LOCALE_SABBREVMONTHNAME7: u32 = 74u32;
pub const LOCALE_SABBREVMONTHNAME8: u32 = 75u32;
pub const LOCALE_SABBREVMONTHNAME9: u32 = 76u32;
pub const LOCALE_SAM: u32 = 40u32;
pub const LOCALE_SCONSOLEFALLBACKNAME: u32 = 110u32;
pub const LOCALE_SCOUNTRY: u32 = 6u32;
pub const LOCALE_SCURRENCY: u32 = 20u32;
pub const LOCALE_SDATE: u32 = 29u32;
pub const LOCALE_SDAYNAME1: u32 = 42u32;
pub const LOCALE_SDAYNAME2: u32 = 43u32;
pub const LOCALE_SDAYNAME3: u32 = 44u32;
pub const LOCALE_SDAYNAME4: u32 = 45u32;
pub const LOCALE_SDAYNAME5: u32 = 46u32;
pub const LOCALE_SDAYNAME6: u32 = 47u32;
pub const LOCALE_SDAYNAME7: u32 = 48u32;
pub const LOCALE_SDECIMAL: u32 = 14u32;
pub const LOCALE_SDURATION: u32 = 93u32;
pub const LOCALE_SENGCOUNTRY: u32 = 4098u32;
pub const LOCALE_SENGCURRNAME: u32 = 4103u32;
pub const LOCALE_SENGLANGUAGE: u32 = 4097u32;
pub const LOCALE_SENGLISHCOUNTRYNAME: u32 = 4098u32;
pub const LOCALE_SENGLISHDISPLAYNAME: u32 = 114u32;
pub const LOCALE_SENGLISHLANGUAGENAME: u32 = 4097u32;
pub const LOCALE_SGROUPING: u32 = 16u32;
pub const LOCALE_SINTLSYMBOL: u32 = 21u32;
pub const LOCALE_SISO3166CTRYNAME: u32 = 90u32;
pub const LOCALE_SISO3166CTRYNAME2: u32 = 104u32;
pub const LOCALE_SISO639LANGNAME: u32 = 89u32;
pub const LOCALE_SISO639LANGNAME2: u32 = 103u32;
pub const LOCALE_SKEYBOARDSTOINSTALL: u32 = 94u32;
pub const LOCALE_SLANGDISPLAYNAME: u32 = 111u32;
pub const LOCALE_SLANGUAGE: u32 = 2u32;
pub const LOCALE_SLIST: u32 = 12u32;
pub const LOCALE_SLOCALIZEDCOUNTRYNAME: u32 = 6u32;
pub const LOCALE_SLOCALIZEDDISPLAYNAME: u32 = 2u32;
pub const LOCALE_SLOCALIZEDLANGUAGENAME: u32 = 111u32;
pub const LOCALE_SLONGDATE: u32 = 32u32;
pub const LOCALE_SMONDECIMALSEP: u32 = 22u32;
pub const LOCALE_SMONGROUPING: u32 = 24u32;
pub const LOCALE_SMONTHDAY: u32 = 120u32;
pub const LOCALE_SMONTHNAME1: u32 = 56u32;
pub const LOCALE_SMONTHNAME10: u32 = 65u32;
pub const LOCALE_SMONTHNAME11: u32 = 66u32;
pub const LOCALE_SMONTHNAME12: u32 = 67u32;
pub const LOCALE_SMONTHNAME13: u32 = 4110u32;
pub const LOCALE_SMONTHNAME2: u32 = 57u32;
pub const LOCALE_SMONTHNAME3: u32 = 58u32;
pub const LOCALE_SMONTHNAME4: u32 = 59u32;
pub const LOCALE_SMONTHNAME5: u32 = 60u32;
pub const LOCALE_SMONTHNAME6: u32 = 61u32;
pub const LOCALE_SMONTHNAME7: u32 = 62u32;
pub const LOCALE_SMONTHNAME8: u32 = 63u32;
pub const LOCALE_SMONTHNAME9: u32 = 64u32;
pub const LOCALE_SMONTHOUSANDSEP: u32 = 23u32;
pub const LOCALE_SNAME: u32 = 92u32;
pub const LOCALE_SNAN: u32 = 105u32;
pub const LOCALE_SNATIVECOUNTRYNAME: u32 = 8u32;
pub const LOCALE_SNATIVECTRYNAME: u32 = 8u32;
pub const LOCALE_SNATIVECURRNAME: u32 = 4104u32;
pub const LOCALE_SNATIVEDIGITS: u32 = 19u32;
pub const LOCALE_SNATIVEDISPLAYNAME: u32 = 115u32;
pub const LOCALE_SNATIVELANGNAME: u32 = 4u32;
pub const LOCALE_SNATIVELANGUAGENAME: u32 = 4u32;
pub const LOCALE_SNEGATIVESIGN: u32 = 81u32;
pub const LOCALE_SNEGINFINITY: u32 = 107u32;
pub const LOCALE_SOPENTYPELANGUAGETAG: u32 = 122u32;
pub const LOCALE_SPARENT: u32 = 109u32;
pub const LOCALE_SPECIFICDATA: u32 = 32u32;
pub const LOCALE_SPERCENT: u32 = 118u32;
pub const LOCALE_SPERMILLE: u32 = 119u32;
pub const LOCALE_SPM: u32 = 41u32;
pub const LOCALE_SPOSINFINITY: u32 = 106u32;
pub const LOCALE_SPOSITIVESIGN: u32 = 80u32;
pub const LOCALE_SRELATIVELONGDATE: u32 = 124u32;
pub const LOCALE_SSCRIPTS: u32 = 108u32;
pub const LOCALE_SSHORTDATE: u32 = 31u32;
pub const LOCALE_SSHORTESTAM: u32 = 126u32;
pub const LOCALE_SSHORTESTDAYNAME1: u32 = 96u32;
pub const LOCALE_SSHORTESTDAYNAME2: u32 = 97u32;
pub const LOCALE_SSHORTESTDAYNAME3: u32 = 98u32;
pub const LOCALE_SSHORTESTDAYNAME4: u32 = 99u32;
pub const LOCALE_SSHORTESTDAYNAME5: u32 = 100u32;
pub const LOCALE_SSHORTESTDAYNAME6: u32 = 101u32;
pub const LOCALE_SSHORTESTDAYNAME7: u32 = 102u32;
pub const LOCALE_SSHORTESTPM: u32 = 127u32;
pub const LOCALE_SSHORTTIME: u32 = 121u32;
pub const LOCALE_SSORTLOCALE: u32 = 123u32;
pub const LOCALE_SSORTNAME: u32 = 4115u32;
pub const LOCALE_STHOUSAND: u32 = 15u32;
pub const LOCALE_STIME: u32 = 30u32;
pub const LOCALE_STIMEFORMAT: u32 = 4099u32;
pub const LOCALE_SUPPLEMENTAL: u32 = 2u32;
pub const LOCALE_SYEARMONTH: u32 = 4102u32;
pub const LOCALE_USE_CP_ACP: u32 = 1073741824u32;
pub const LOCALE_WINDOWS: u32 = 1u32;
pub const LOWLEVEL_SERVICE_TYPES: u32 = 2u32;
pub const LOW_SURROGATE_END: u32 = 57343u32;
pub const LOW_SURROGATE_START: u32 = 56320u32;
pub struct MAPPING_DATA_RANGE {
    pub dwStartIndex: u32,
    pub dwEndIndex: u32,
    pub pszDescription: PWSTR,
    pub dwDescriptionLength: u32,
    pub pData: MutPtr<::core::ffi::c_void>,
    pub dwDataSize: u32,
    pub pszContentType: PWSTR,
    pub prgActionIds: MutPtr<PWSTR>,
    pub dwActionsCount: u32,
    pub prgActionDisplayNames: MutPtr<PWSTR>,
}
impl ::core::marker::Copy for MAPPING_DATA_RANGE {}
impl ::core::clone::Clone for MAPPING_DATA_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MAPPING_DATA_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MAPPING_DATA_RANGE")
            .field("dwStartIndex", &self.dwStartIndex)
            .field("dwEndIndex", &self.dwEndIndex)
            .field("pszDescription", &self.pszDescription)
            .field("dwDescriptionLength", &self.dwDescriptionLength)
            .field("pData", &self.pData)
            .field("dwDataSize", &self.dwDataSize)
            .field("pszContentType", &self.pszContentType)
            .field("prgActionIds", &self.prgActionIds)
            .field("dwActionsCount", &self.dwActionsCount)
            .field("prgActionDisplayNames", &self.prgActionDisplayNames)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MAPPING_DATA_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.dwStartIndex == other.dwStartIndex
            && self.dwEndIndex == other.dwEndIndex
            && self.pszDescription == other.pszDescription
            && self.dwDescriptionLength == other.dwDescriptionLength
            && self.pData == other.pData
            && self.dwDataSize == other.dwDataSize
            && self.pszContentType == other.pszContentType
            && self.prgActionIds == other.prgActionIds
            && self.dwActionsCount == other.dwActionsCount
            && self.prgActionDisplayNames == other.prgActionDisplayNames
    }
}
impl ::core::cmp::Eq for MAPPING_DATA_RANGE {}
impl FromIntoMemory for MAPPING_DATA_RANGE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40u32 as usize);
        let f_dwStartIndex = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwEndIndex = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_pszDescription = <PWSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwDescriptionLength = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_pData =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwDataSize = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_pszContentType = <PWSTR as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_prgActionIds = <MutPtr<PWSTR> as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_dwActionsCount = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_prgActionDisplayNames =
            <MutPtr<PWSTR> as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        Self {
            dwStartIndex: f_dwStartIndex,
            dwEndIndex: f_dwEndIndex,
            pszDescription: f_pszDescription,
            dwDescriptionLength: f_dwDescriptionLength,
            pData: f_pData,
            dwDataSize: f_dwDataSize,
            pszContentType: f_pszContentType,
            prgActionIds: f_prgActionIds,
            dwActionsCount: f_dwActionsCount,
            prgActionDisplayNames: f_prgActionDisplayNames,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40u32 as usize);
        FromIntoMemory::into_bytes(self.dwStartIndex, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwEndIndex, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.pszDescription, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwDescriptionLength, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.pData, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwDataSize, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.pszContentType, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.prgActionIds, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.dwActionsCount, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.prgActionDisplayNames, &mut into[36..36 + 4]);
    }
    fn size() -> usize {
        40u32 as usize
    }
}
pub struct MAPPING_ENUM_OPTIONS {
    pub Size: PtrRepr,
    pub pszCategory: PWSTR,
    pub pszInputLanguage: PWSTR,
    pub pszOutputLanguage: PWSTR,
    pub pszInputScript: PWSTR,
    pub pszOutputScript: PWSTR,
    pub pszInputContentType: PWSTR,
    pub pszOutputContentType: PWSTR,
    pub pGuid: MutPtr<crate::core::GUID>,
    pub _bitfield: u32,
}
impl ::core::marker::Copy for MAPPING_ENUM_OPTIONS {}
impl ::core::clone::Clone for MAPPING_ENUM_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MAPPING_ENUM_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MAPPING_ENUM_OPTIONS")
            .field("Size", &self.Size)
            .field("pszCategory", &self.pszCategory)
            .field("pszInputLanguage", &self.pszInputLanguage)
            .field("pszOutputLanguage", &self.pszOutputLanguage)
            .field("pszInputScript", &self.pszInputScript)
            .field("pszOutputScript", &self.pszOutputScript)
            .field("pszInputContentType", &self.pszInputContentType)
            .field("pszOutputContentType", &self.pszOutputContentType)
            .field("pGuid", &self.pGuid)
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MAPPING_ENUM_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.pszCategory == other.pszCategory
            && self.pszInputLanguage == other.pszInputLanguage
            && self.pszOutputLanguage == other.pszOutputLanguage
            && self.pszInputScript == other.pszInputScript
            && self.pszOutputScript == other.pszOutputScript
            && self.pszInputContentType == other.pszInputContentType
            && self.pszOutputContentType == other.pszOutputContentType
            && self.pGuid == other.pGuid
            && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for MAPPING_ENUM_OPTIONS {}
impl FromIntoMemory for MAPPING_ENUM_OPTIONS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40u32 as usize);
        let f_Size = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_pszCategory = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_pszInputLanguage = <PWSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_pszOutputLanguage = <PWSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_pszInputScript = <PWSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_pszOutputScript = <PWSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_pszInputContentType = <PWSTR as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_pszOutputContentType = <PWSTR as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_pGuid = <MutPtr<crate::core::GUID> as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f__bitfield = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        Self {
            Size: f_Size,
            pszCategory: f_pszCategory,
            pszInputLanguage: f_pszInputLanguage,
            pszOutputLanguage: f_pszOutputLanguage,
            pszInputScript: f_pszInputScript,
            pszOutputScript: f_pszOutputScript,
            pszInputContentType: f_pszInputContentType,
            pszOutputContentType: f_pszOutputContentType,
            pGuid: f_pGuid,
            _bitfield: f__bitfield,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40u32 as usize);
        FromIntoMemory::into_bytes(self.Size, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.pszCategory, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.pszInputLanguage, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.pszOutputLanguage, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.pszInputScript, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.pszOutputScript, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.pszInputContentType, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.pszOutputContentType, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.pGuid, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self._bitfield, &mut into[36..36 + 4]);
    }
    fn size() -> usize {
        40u32 as usize
    }
}
pub struct MAPPING_OPTIONS {
    pub Size: PtrRepr,
    pub pszInputLanguage: PWSTR,
    pub pszOutputLanguage: PWSTR,
    pub pszInputScript: PWSTR,
    pub pszOutputScript: PWSTR,
    pub pszInputContentType: PWSTR,
    pub pszOutputContentType: PWSTR,
    pub pszUILanguage: PWSTR,
    pub pfnRecognizeCallback: PFN_MAPPINGCALLBACKPROC,
    pub pRecognizeCallerData: MutPtr<::core::ffi::c_void>,
    pub dwRecognizeCallerDataSize: u32,
    pub pfnActionCallback: PFN_MAPPINGCALLBACKPROC,
    pub pActionCallerData: MutPtr<::core::ffi::c_void>,
    pub dwActionCallerDataSize: u32,
    pub dwServiceFlag: u32,
    pub _bitfield: u32,
}
impl ::core::marker::Copy for MAPPING_OPTIONS {}
impl ::core::clone::Clone for MAPPING_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MAPPING_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MAPPING_OPTIONS")
            .field("Size", &self.Size)
            .field("pszInputLanguage", &self.pszInputLanguage)
            .field("pszOutputLanguage", &self.pszOutputLanguage)
            .field("pszInputScript", &self.pszInputScript)
            .field("pszOutputScript", &self.pszOutputScript)
            .field("pszInputContentType", &self.pszInputContentType)
            .field("pszOutputContentType", &self.pszOutputContentType)
            .field("pszUILanguage", &self.pszUILanguage)
            .field("pfnRecognizeCallback", &self.pfnRecognizeCallback)
            .field("pRecognizeCallerData", &self.pRecognizeCallerData)
            .field("dwRecognizeCallerDataSize", &self.dwRecognizeCallerDataSize)
            .field("pfnActionCallback", &self.pfnActionCallback)
            .field("pActionCallerData", &self.pActionCallerData)
            .field("dwActionCallerDataSize", &self.dwActionCallerDataSize)
            .field("dwServiceFlag", &self.dwServiceFlag)
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MAPPING_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.pszInputLanguage == other.pszInputLanguage
            && self.pszOutputLanguage == other.pszOutputLanguage
            && self.pszInputScript == other.pszInputScript
            && self.pszOutputScript == other.pszOutputScript
            && self.pszInputContentType == other.pszInputContentType
            && self.pszOutputContentType == other.pszOutputContentType
            && self.pszUILanguage == other.pszUILanguage
            && self.pfnRecognizeCallback == other.pfnRecognizeCallback
            && self.pRecognizeCallerData == other.pRecognizeCallerData
            && self.dwRecognizeCallerDataSize == other.dwRecognizeCallerDataSize
            && self.pfnActionCallback == other.pfnActionCallback
            && self.pActionCallerData == other.pActionCallerData
            && self.dwActionCallerDataSize == other.dwActionCallerDataSize
            && self.dwServiceFlag == other.dwServiceFlag
            && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for MAPPING_OPTIONS {}
impl FromIntoMemory for MAPPING_OPTIONS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 64u32 as usize);
        let f_Size = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_pszInputLanguage = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_pszOutputLanguage = <PWSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_pszInputScript = <PWSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_pszOutputScript = <PWSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_pszInputContentType = <PWSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_pszOutputContentType = <PWSTR as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_pszUILanguage = <PWSTR as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_pfnRecognizeCallback =
            <PFN_MAPPINGCALLBACKPROC as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_pRecognizeCallerData =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_dwRecognizeCallerDataSize = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_pfnActionCallback =
            <PFN_MAPPINGCALLBACKPROC as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_pActionCallerData =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_dwActionCallerDataSize = <u32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_dwServiceFlag = <u32 as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f__bitfield = <u32 as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        Self {
            Size: f_Size,
            pszInputLanguage: f_pszInputLanguage,
            pszOutputLanguage: f_pszOutputLanguage,
            pszInputScript: f_pszInputScript,
            pszOutputScript: f_pszOutputScript,
            pszInputContentType: f_pszInputContentType,
            pszOutputContentType: f_pszOutputContentType,
            pszUILanguage: f_pszUILanguage,
            pfnRecognizeCallback: f_pfnRecognizeCallback,
            pRecognizeCallerData: f_pRecognizeCallerData,
            dwRecognizeCallerDataSize: f_dwRecognizeCallerDataSize,
            pfnActionCallback: f_pfnActionCallback,
            pActionCallerData: f_pActionCallerData,
            dwActionCallerDataSize: f_dwActionCallerDataSize,
            dwServiceFlag: f_dwServiceFlag,
            _bitfield: f__bitfield,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 64u32 as usize);
        FromIntoMemory::into_bytes(self.Size, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.pszInputLanguage, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.pszOutputLanguage, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.pszInputScript, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.pszOutputScript, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.pszInputContentType, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.pszOutputContentType, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.pszUILanguage, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.pfnRecognizeCallback, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.pRecognizeCallerData, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.dwRecognizeCallerDataSize, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.pfnActionCallback, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.pActionCallerData, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.dwActionCallerDataSize, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.dwServiceFlag, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self._bitfield, &mut into[60..60 + 4]);
    }
    fn size() -> usize {
        64u32 as usize
    }
}
pub struct MAPPING_PROPERTY_BAG {
    pub Size: PtrRepr,
    pub prgResultRanges: MutPtr<MAPPING_DATA_RANGE>,
    pub dwRangesCount: u32,
    pub pServiceData: MutPtr<::core::ffi::c_void>,
    pub dwServiceDataSize: u32,
    pub pCallerData: MutPtr<::core::ffi::c_void>,
    pub dwCallerDataSize: u32,
    pub pContext: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for MAPPING_PROPERTY_BAG {}
impl ::core::clone::Clone for MAPPING_PROPERTY_BAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MAPPING_PROPERTY_BAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MAPPING_PROPERTY_BAG")
            .field("Size", &self.Size)
            .field("prgResultRanges", &self.prgResultRanges)
            .field("dwRangesCount", &self.dwRangesCount)
            .field("pServiceData", &self.pServiceData)
            .field("dwServiceDataSize", &self.dwServiceDataSize)
            .field("pCallerData", &self.pCallerData)
            .field("dwCallerDataSize", &self.dwCallerDataSize)
            .field("pContext", &self.pContext)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MAPPING_PROPERTY_BAG {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.prgResultRanges == other.prgResultRanges
            && self.dwRangesCount == other.dwRangesCount
            && self.pServiceData == other.pServiceData
            && self.dwServiceDataSize == other.dwServiceDataSize
            && self.pCallerData == other.pCallerData
            && self.dwCallerDataSize == other.dwCallerDataSize
            && self.pContext == other.pContext
    }
}
impl ::core::cmp::Eq for MAPPING_PROPERTY_BAG {}
impl FromIntoMemory for MAPPING_PROPERTY_BAG {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32u32 as usize);
        let f_Size = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_prgResultRanges =
            <MutPtr<MAPPING_DATA_RANGE> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwRangesCount = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_pServiceData =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwServiceDataSize = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_pCallerData =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwCallerDataSize = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_pContext =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            Size: f_Size,
            prgResultRanges: f_prgResultRanges,
            dwRangesCount: f_dwRangesCount,
            pServiceData: f_pServiceData,
            dwServiceDataSize: f_dwServiceDataSize,
            pCallerData: f_pCallerData,
            dwCallerDataSize: f_dwCallerDataSize,
            pContext: f_pContext,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32u32 as usize);
        FromIntoMemory::into_bytes(self.Size, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.prgResultRanges, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwRangesCount, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.pServiceData, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwServiceDataSize, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.pCallerData, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwCallerDataSize, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.pContext, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32u32 as usize
    }
}
pub struct MAPPING_SERVICE_INFO {
    pub Size: PtrRepr,
    pub pszCopyright: PWSTR,
    pub wMajorVersion: u16,
    pub wMinorVersion: u16,
    pub wBuildVersion: u16,
    pub wStepVersion: u16,
    pub dwInputContentTypesCount: u32,
    pub prgInputContentTypes: MutPtr<PWSTR>,
    pub dwOutputContentTypesCount: u32,
    pub prgOutputContentTypes: MutPtr<PWSTR>,
    pub dwInputLanguagesCount: u32,
    pub prgInputLanguages: MutPtr<PWSTR>,
    pub dwOutputLanguagesCount: u32,
    pub prgOutputLanguages: MutPtr<PWSTR>,
    pub dwInputScriptsCount: u32,
    pub prgInputScripts: MutPtr<PWSTR>,
    pub dwOutputScriptsCount: u32,
    pub prgOutputScripts: MutPtr<PWSTR>,
    pub guid: crate::core::GUID,
    pub pszCategory: PWSTR,
    pub pszDescription: PWSTR,
    pub dwPrivateDataSize: u32,
    pub pPrivateData: MutPtr<::core::ffi::c_void>,
    pub pContext: MutPtr<::core::ffi::c_void>,
    pub _bitfield: u32,
}
impl ::core::marker::Copy for MAPPING_SERVICE_INFO {}
impl ::core::clone::Clone for MAPPING_SERVICE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MAPPING_SERVICE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MAPPING_SERVICE_INFO")
            .field("Size", &self.Size)
            .field("pszCopyright", &self.pszCopyright)
            .field("wMajorVersion", &self.wMajorVersion)
            .field("wMinorVersion", &self.wMinorVersion)
            .field("wBuildVersion", &self.wBuildVersion)
            .field("wStepVersion", &self.wStepVersion)
            .field("dwInputContentTypesCount", &self.dwInputContentTypesCount)
            .field("prgInputContentTypes", &self.prgInputContentTypes)
            .field("dwOutputContentTypesCount", &self.dwOutputContentTypesCount)
            .field("prgOutputContentTypes", &self.prgOutputContentTypes)
            .field("dwInputLanguagesCount", &self.dwInputLanguagesCount)
            .field("prgInputLanguages", &self.prgInputLanguages)
            .field("dwOutputLanguagesCount", &self.dwOutputLanguagesCount)
            .field("prgOutputLanguages", &self.prgOutputLanguages)
            .field("dwInputScriptsCount", &self.dwInputScriptsCount)
            .field("prgInputScripts", &self.prgInputScripts)
            .field("dwOutputScriptsCount", &self.dwOutputScriptsCount)
            .field("prgOutputScripts", &self.prgOutputScripts)
            .field("guid", &self.guid)
            .field("pszCategory", &self.pszCategory)
            .field("pszDescription", &self.pszDescription)
            .field("dwPrivateDataSize", &self.dwPrivateDataSize)
            .field("pPrivateData", &self.pPrivateData)
            .field("pContext", &self.pContext)
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MAPPING_SERVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.pszCopyright == other.pszCopyright
            && self.wMajorVersion == other.wMajorVersion
            && self.wMinorVersion == other.wMinorVersion
            && self.wBuildVersion == other.wBuildVersion
            && self.wStepVersion == other.wStepVersion
            && self.dwInputContentTypesCount == other.dwInputContentTypesCount
            && self.prgInputContentTypes == other.prgInputContentTypes
            && self.dwOutputContentTypesCount == other.dwOutputContentTypesCount
            && self.prgOutputContentTypes == other.prgOutputContentTypes
            && self.dwInputLanguagesCount == other.dwInputLanguagesCount
            && self.prgInputLanguages == other.prgInputLanguages
            && self.dwOutputLanguagesCount == other.dwOutputLanguagesCount
            && self.prgOutputLanguages == other.prgOutputLanguages
            && self.dwInputScriptsCount == other.dwInputScriptsCount
            && self.prgInputScripts == other.prgInputScripts
            && self.dwOutputScriptsCount == other.dwOutputScriptsCount
            && self.prgOutputScripts == other.prgOutputScripts
            && self.guid == other.guid
            && self.pszCategory == other.pszCategory
            && self.pszDescription == other.pszDescription
            && self.dwPrivateDataSize == other.dwPrivateDataSize
            && self.pPrivateData == other.pPrivateData
            && self.pContext == other.pContext
            && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for MAPPING_SERVICE_INFO {}
impl FromIntoMemory for MAPPING_SERVICE_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 104u32 as usize);
        let f_Size = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_pszCopyright = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_wMajorVersion = <u16 as FromIntoMemory>::from_bytes(&from[8..8 + 2]);
        let f_wMinorVersion = <u16 as FromIntoMemory>::from_bytes(&from[10..10 + 2]);
        let f_wBuildVersion = <u16 as FromIntoMemory>::from_bytes(&from[12..12 + 2]);
        let f_wStepVersion = <u16 as FromIntoMemory>::from_bytes(&from[14..14 + 2]);
        let f_dwInputContentTypesCount = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_prgInputContentTypes =
            <MutPtr<PWSTR> as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwOutputContentTypesCount = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_prgOutputContentTypes =
            <MutPtr<PWSTR> as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_dwInputLanguagesCount = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_prgInputLanguages = <MutPtr<PWSTR> as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_dwOutputLanguagesCount = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_prgOutputLanguages = <MutPtr<PWSTR> as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_dwInputScriptsCount = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_prgInputScripts = <MutPtr<PWSTR> as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_dwOutputScriptsCount = <u32 as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_prgOutputScripts = <MutPtr<PWSTR> as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_guid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[64..64 + 16]);
        let f_pszCategory = <PWSTR as FromIntoMemory>::from_bytes(&from[80..80 + 4]);
        let f_pszDescription = <PWSTR as FromIntoMemory>::from_bytes(&from[84..84 + 4]);
        let f_dwPrivateDataSize = <u32 as FromIntoMemory>::from_bytes(&from[88..88 + 4]);
        let f_pPrivateData =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[92..92 + 4]);
        let f_pContext =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[96..96 + 4]);
        let f__bitfield = <u32 as FromIntoMemory>::from_bytes(&from[100..100 + 4]);
        Self {
            Size: f_Size,
            pszCopyright: f_pszCopyright,
            wMajorVersion: f_wMajorVersion,
            wMinorVersion: f_wMinorVersion,
            wBuildVersion: f_wBuildVersion,
            wStepVersion: f_wStepVersion,
            dwInputContentTypesCount: f_dwInputContentTypesCount,
            prgInputContentTypes: f_prgInputContentTypes,
            dwOutputContentTypesCount: f_dwOutputContentTypesCount,
            prgOutputContentTypes: f_prgOutputContentTypes,
            dwInputLanguagesCount: f_dwInputLanguagesCount,
            prgInputLanguages: f_prgInputLanguages,
            dwOutputLanguagesCount: f_dwOutputLanguagesCount,
            prgOutputLanguages: f_prgOutputLanguages,
            dwInputScriptsCount: f_dwInputScriptsCount,
            prgInputScripts: f_prgInputScripts,
            dwOutputScriptsCount: f_dwOutputScriptsCount,
            prgOutputScripts: f_prgOutputScripts,
            guid: f_guid,
            pszCategory: f_pszCategory,
            pszDescription: f_pszDescription,
            dwPrivateDataSize: f_dwPrivateDataSize,
            pPrivateData: f_pPrivateData,
            pContext: f_pContext,
            _bitfield: f__bitfield,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 104u32 as usize);
        FromIntoMemory::into_bytes(self.Size, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.pszCopyright, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.wMajorVersion, &mut into[8..8 + 2]);
        FromIntoMemory::into_bytes(self.wMinorVersion, &mut into[10..10 + 2]);
        FromIntoMemory::into_bytes(self.wBuildVersion, &mut into[12..12 + 2]);
        FromIntoMemory::into_bytes(self.wStepVersion, &mut into[14..14 + 2]);
        FromIntoMemory::into_bytes(self.dwInputContentTypesCount, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.prgInputContentTypes, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwOutputContentTypesCount, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.prgOutputContentTypes, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.dwInputLanguagesCount, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.prgInputLanguages, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.dwOutputLanguagesCount, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.prgOutputLanguages, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.dwInputScriptsCount, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.prgInputScripts, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.dwOutputScriptsCount, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.prgOutputScripts, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.guid, &mut into[64..64 + 16]);
        FromIntoMemory::into_bytes(self.pszCategory, &mut into[80..80 + 4]);
        FromIntoMemory::into_bytes(self.pszDescription, &mut into[84..84 + 4]);
        FromIntoMemory::into_bytes(self.dwPrivateDataSize, &mut into[88..88 + 4]);
        FromIntoMemory::into_bytes(self.pPrivateData, &mut into[92..92 + 4]);
        FromIntoMemory::into_bytes(self.pContext, &mut into[96..96 + 4]);
        FromIntoMemory::into_bytes(self._bitfield, &mut into[100..100 + 4]);
    }
    fn size() -> usize {
        104u32 as usize
    }
}
pub const MAX_DEFAULTCHAR: u32 = 2u32;
pub const MAX_LEADBYTES: u32 = 12u32;
pub const MAX_LOCALE_NAME: u32 = 32u32;
pub const MAX_MIMECP_NAME: u32 = 64u32;
pub const MAX_MIMECSET_NAME: u32 = 50u32;
pub const MAX_MIMEFACE_NAME: u32 = 32u32;
pub const MAX_RFC1766_NAME: u32 = 6u32;
pub const MAX_SCRIPT_NAME: u32 = 48u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MIMECONTF(pub i32);
pub const MIMECONTF_MAILNEWS: MIMECONTF = MIMECONTF(1i32);
pub const MIMECONTF_BROWSER: MIMECONTF = MIMECONTF(2i32);
pub const MIMECONTF_MINIMAL: MIMECONTF = MIMECONTF(4i32);
pub const MIMECONTF_IMPORT: MIMECONTF = MIMECONTF(8i32);
pub const MIMECONTF_SAVABLE_MAILNEWS: MIMECONTF = MIMECONTF(256i32);
pub const MIMECONTF_SAVABLE_BROWSER: MIMECONTF = MIMECONTF(512i32);
pub const MIMECONTF_EXPORT: MIMECONTF = MIMECONTF(1024i32);
pub const MIMECONTF_PRIVCONVERTER: MIMECONTF = MIMECONTF(65536i32);
pub const MIMECONTF_VALID: MIMECONTF = MIMECONTF(131072i32);
pub const MIMECONTF_VALID_NLS: MIMECONTF = MIMECONTF(262144i32);
pub const MIMECONTF_MIME_IE4: MIMECONTF = MIMECONTF(268435456i32);
pub const MIMECONTF_MIME_LATEST: MIMECONTF = MIMECONTF(536870912i32);
pub const MIMECONTF_MIME_REGISTRY: MIMECONTF = MIMECONTF(1073741824i32);
impl ::core::marker::Copy for MIMECONTF {}
impl ::core::clone::Clone for MIMECONTF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MIMECONTF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MIMECONTF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIMECONTF").field(&self.0).finish()
    }
}
impl FromIntoMemory for MIMECONTF {
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
pub struct MIMECPINFO {
    pub dwFlags: u32,
    pub uiCodePage: u32,
    pub uiFamilyCodePage: u32,
    pub wszDescription: [u16; 64],
    pub wszWebCharset: [u16; 50],
    pub wszHeaderCharset: [u16; 50],
    pub wszBodyCharset: [u16; 50],
    pub wszFixedWidthFont: [u16; 32],
    pub wszProportionalFont: [u16; 32],
    pub bGDICharset: u8,
}
impl ::core::marker::Copy for MIMECPINFO {}
impl ::core::clone::Clone for MIMECPINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIMECPINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIMECPINFO")
            .field("dwFlags", &self.dwFlags)
            .field("uiCodePage", &self.uiCodePage)
            .field("uiFamilyCodePage", &self.uiFamilyCodePage)
            .field("wszDescription", &self.wszDescription)
            .field("wszWebCharset", &self.wszWebCharset)
            .field("wszHeaderCharset", &self.wszHeaderCharset)
            .field("wszBodyCharset", &self.wszBodyCharset)
            .field("wszFixedWidthFont", &self.wszFixedWidthFont)
            .field("wszProportionalFont", &self.wszProportionalFont)
            .field("bGDICharset", &self.bGDICharset)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIMECPINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags
            && self.uiCodePage == other.uiCodePage
            && self.uiFamilyCodePage == other.uiFamilyCodePage
            && self.wszDescription == other.wszDescription
            && self.wszWebCharset == other.wszWebCharset
            && self.wszHeaderCharset == other.wszHeaderCharset
            && self.wszBodyCharset == other.wszBodyCharset
            && self.wszFixedWidthFont == other.wszFixedWidthFont
            && self.wszProportionalFont == other.wszProportionalFont
            && self.bGDICharset == other.bGDICharset
    }
}
impl ::core::cmp::Eq for MIMECPINFO {}
impl FromIntoMemory for MIMECPINFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 292u32 as usize);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_uiCodePage = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_uiFamilyCodePage = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_wszDescription = <[u16; 64] as FromIntoMemory>::from_bytes(&from[12..12 + 64]);
        let f_wszWebCharset = <[u16; 50] as FromIntoMemory>::from_bytes(&from[76..76 + 50]);
        let f_wszHeaderCharset = <[u16; 50] as FromIntoMemory>::from_bytes(&from[126..126 + 50]);
        let f_wszBodyCharset = <[u16; 50] as FromIntoMemory>::from_bytes(&from[176..176 + 50]);
        let f_wszFixedWidthFont = <[u16; 32] as FromIntoMemory>::from_bytes(&from[226..226 + 32]);
        let f_wszProportionalFont = <[u16; 32] as FromIntoMemory>::from_bytes(&from[258..258 + 32]);
        let f_bGDICharset = <u8 as FromIntoMemory>::from_bytes(&from[290..290 + 1]);
        Self {
            dwFlags: f_dwFlags,
            uiCodePage: f_uiCodePage,
            uiFamilyCodePage: f_uiFamilyCodePage,
            wszDescription: f_wszDescription,
            wszWebCharset: f_wszWebCharset,
            wszHeaderCharset: f_wszHeaderCharset,
            wszBodyCharset: f_wszBodyCharset,
            wszFixedWidthFont: f_wszFixedWidthFont,
            wszProportionalFont: f_wszProportionalFont,
            bGDICharset: f_bGDICharset,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 292u32 as usize);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.uiCodePage, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.uiFamilyCodePage, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.wszDescription, &mut into[12..12 + 64]);
        FromIntoMemory::into_bytes(self.wszWebCharset, &mut into[76..76 + 50]);
        FromIntoMemory::into_bytes(self.wszHeaderCharset, &mut into[126..126 + 50]);
        FromIntoMemory::into_bytes(self.wszBodyCharset, &mut into[176..176 + 50]);
        FromIntoMemory::into_bytes(self.wszFixedWidthFont, &mut into[226..226 + 32]);
        FromIntoMemory::into_bytes(self.wszProportionalFont, &mut into[258..258 + 32]);
        FromIntoMemory::into_bytes(self.bGDICharset, &mut into[290..290 + 1]);
    }
    fn size() -> usize {
        292u32 as usize
    }
}
pub struct MIMECSETINFO {
    pub uiCodePage: u32,
    pub uiInternetEncoding: u32,
    pub wszCharset: [u16; 50],
}
impl ::core::marker::Copy for MIMECSETINFO {}
impl ::core::clone::Clone for MIMECSETINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIMECSETINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIMECSETINFO")
            .field("uiCodePage", &self.uiCodePage)
            .field("uiInternetEncoding", &self.uiInternetEncoding)
            .field("wszCharset", &self.wszCharset)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MIMECSETINFO {
    fn eq(&self, other: &Self) -> bool {
        self.uiCodePage == other.uiCodePage
            && self.uiInternetEncoding == other.uiInternetEncoding
            && self.wszCharset == other.wszCharset
    }
}
impl ::core::cmp::Eq for MIMECSETINFO {}
impl FromIntoMemory for MIMECSETINFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 60u32 as usize);
        let f_uiCodePage = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_uiInternetEncoding = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_wszCharset = <[u16; 50] as FromIntoMemory>::from_bytes(&from[8..8 + 50]);
        Self {
            uiCodePage: f_uiCodePage,
            uiInternetEncoding: f_uiInternetEncoding,
            wszCharset: f_wszCharset,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 60u32 as usize);
        FromIntoMemory::into_bytes(self.uiCodePage, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.uiInternetEncoding, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.wszCharset, &mut into[8..8 + 50]);
    }
    fn size() -> usize {
        60u32 as usize
    }
}
pub const MIN_SPELLING_NTDDI: u32 = 100794368u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MLDETECTCP(pub i32);
pub const MLDETECTCP_NONE: MLDETECTCP = MLDETECTCP(0i32);
pub const MLDETECTCP_7BIT: MLDETECTCP = MLDETECTCP(1i32);
pub const MLDETECTCP_8BIT: MLDETECTCP = MLDETECTCP(2i32);
pub const MLDETECTCP_DBCS: MLDETECTCP = MLDETECTCP(4i32);
pub const MLDETECTCP_HTML: MLDETECTCP = MLDETECTCP(8i32);
pub const MLDETECTCP_NUMBER: MLDETECTCP = MLDETECTCP(16i32);
impl ::core::marker::Copy for MLDETECTCP {}
impl ::core::clone::Clone for MLDETECTCP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MLDETECTCP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MLDETECTCP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MLDETECTCP").field(&self.0).finish()
    }
}
impl FromIntoMemory for MLDETECTCP {
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
pub struct MLSTR_FLAGS(pub i32);
pub const MLSTR_READ: MLSTR_FLAGS = MLSTR_FLAGS(1i32);
pub const MLSTR_WRITE: MLSTR_FLAGS = MLSTR_FLAGS(2i32);
impl ::core::marker::Copy for MLSTR_FLAGS {}
impl ::core::clone::Clone for MLSTR_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MLSTR_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MLSTR_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MLSTR_FLAGS").field(&self.0).finish()
    }
}
impl FromIntoMemory for MLSTR_FLAGS {
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
pub const MUI_COMPLEX_SCRIPT_FILTER: u32 = 512u32;
pub const MUI_CONSOLE_FILTER: u32 = 256u32;
pub const MUI_FILEINFO_VERSION: u32 = 1u32;
pub const MUI_FILETYPE_LANGUAGE_NEUTRAL_MAIN: u32 = 2u32;
pub const MUI_FILETYPE_LANGUAGE_NEUTRAL_MUI: u32 = 4u32;
pub const MUI_FILETYPE_NOT_LANGUAGE_NEUTRAL: u32 = 1u32;
pub const MUI_FORMAT_INF_COMPAT: u32 = 2u32;
pub const MUI_FORMAT_REG_COMPAT: u32 = 1u32;
pub const MUI_FULL_LANGUAGE: u32 = 1u32;
pub const MUI_IMMUTABLE_LOOKUP: u32 = 16u32;
pub const MUI_LANGUAGE_EXACT: u32 = 16u32;
pub const MUI_LANGUAGE_ID: u32 = 4u32;
pub const MUI_LANGUAGE_INSTALLED: u32 = 32u32;
pub const MUI_LANGUAGE_LICENSED: u32 = 64u32;
pub const MUI_LANGUAGE_NAME: u32 = 8u32;
pub const MUI_LANG_NEUTRAL_PE_FILE: u32 = 256u32;
pub const MUI_LIP_LANGUAGE: u32 = 4u32;
pub const MUI_MACHINE_LANGUAGE_SETTINGS: u32 = 1024u32;
pub const MUI_MERGE_SYSTEM_FALLBACK: u32 = 16u32;
pub const MUI_MERGE_USER_FALLBACK: u32 = 32u32;
pub const MUI_NON_LANG_NEUTRAL_FILE: u32 = 512u32;
pub const MUI_PARTIAL_LANGUAGE: u32 = 2u32;
pub const MUI_QUERY_CHECKSUM: u32 = 2u32;
pub const MUI_QUERY_LANGUAGE_NAME: u32 = 4u32;
pub const MUI_QUERY_RESOURCE_TYPES: u32 = 8u32;
pub const MUI_QUERY_TYPE: u32 = 1u32;
pub const MUI_RESET_FILTERS: u32 = 1u32;
pub const MUI_SKIP_STRING_CACHE: u32 = 8u32;
pub const MUI_THREAD_LANGUAGES: u32 = 64u32;
pub const MUI_USER_PREFERRED_UI_LANGUAGES: u32 = 16u32;
pub const MUI_USE_INSTALLED_LANGUAGES: u32 = 32u32;
pub const MUI_USE_SEARCH_ALL_LANGUAGES: u32 = 64u32;
pub const MUI_VERIFY_FILE_EXISTS: u32 = 4u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MULTI_BYTE_TO_WIDE_CHAR_FLAGS(pub u32);
pub const MB_COMPOSITE: MULTI_BYTE_TO_WIDE_CHAR_FLAGS = MULTI_BYTE_TO_WIDE_CHAR_FLAGS(2u32);
pub const MB_ERR_INVALID_CHARS: MULTI_BYTE_TO_WIDE_CHAR_FLAGS = MULTI_BYTE_TO_WIDE_CHAR_FLAGS(8u32);
pub const MB_PRECOMPOSED: MULTI_BYTE_TO_WIDE_CHAR_FLAGS = MULTI_BYTE_TO_WIDE_CHAR_FLAGS(1u32);
pub const MB_USEGLYPHCHARS: MULTI_BYTE_TO_WIDE_CHAR_FLAGS = MULTI_BYTE_TO_WIDE_CHAR_FLAGS(4u32);
impl ::core::marker::Copy for MULTI_BYTE_TO_WIDE_CHAR_FLAGS {}
impl ::core::clone::Clone for MULTI_BYTE_TO_WIDE_CHAR_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MULTI_BYTE_TO_WIDE_CHAR_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MULTI_BYTE_TO_WIDE_CHAR_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MULTI_BYTE_TO_WIDE_CHAR_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for MULTI_BYTE_TO_WIDE_CHAR_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MULTI_BYTE_TO_WIDE_CHAR_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MULTI_BYTE_TO_WIDE_CHAR_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MULTI_BYTE_TO_WIDE_CHAR_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MULTI_BYTE_TO_WIDE_CHAR_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for MULTI_BYTE_TO_WIDE_CHAR_FLAGS {
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
pub struct NEWTEXTMETRICEXA {
    pub ntmTm: super::Graphics::Gdi::NEWTEXTMETRICA,
    pub ntmFontSig: FONTSIGNATURE,
}
impl ::core::marker::Copy for NEWTEXTMETRICEXA {}
impl ::core::clone::Clone for NEWTEXTMETRICEXA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NEWTEXTMETRICEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NEWTEXTMETRICEXA")
            .field("ntmTm", &self.ntmTm)
            .field("ntmFontSig", &self.ntmFontSig)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NEWTEXTMETRICEXA {
    fn eq(&self, other: &Self) -> bool {
        self.ntmTm == other.ntmTm && self.ntmFontSig == other.ntmFontSig
    }
}
impl ::core::cmp::Eq for NEWTEXTMETRICEXA {}
impl FromIntoMemory for NEWTEXTMETRICEXA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 96u32 as usize);
        let f_ntmTm =
            <super::Graphics::Gdi::NEWTEXTMETRICA as FromIntoMemory>::from_bytes(&from[0..0 + 72]);
        let f_ntmFontSig = <FONTSIGNATURE as FromIntoMemory>::from_bytes(&from[72..72 + 24]);
        Self {
            ntmTm: f_ntmTm,
            ntmFontSig: f_ntmFontSig,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 96u32 as usize);
        FromIntoMemory::into_bytes(self.ntmTm, &mut into[0..0 + 72]);
        FromIntoMemory::into_bytes(self.ntmFontSig, &mut into[72..72 + 24]);
    }
    fn size() -> usize {
        96u32 as usize
    }
}
pub struct NEWTEXTMETRICEXW {
    pub ntmTm: super::Graphics::Gdi::NEWTEXTMETRICW,
    pub ntmFontSig: FONTSIGNATURE,
}
impl ::core::marker::Copy for NEWTEXTMETRICEXW {}
impl ::core::clone::Clone for NEWTEXTMETRICEXW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NEWTEXTMETRICEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NEWTEXTMETRICEXW")
            .field("ntmTm", &self.ntmTm)
            .field("ntmFontSig", &self.ntmFontSig)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NEWTEXTMETRICEXW {
    fn eq(&self, other: &Self) -> bool {
        self.ntmTm == other.ntmTm && self.ntmFontSig == other.ntmFontSig
    }
}
impl ::core::cmp::Eq for NEWTEXTMETRICEXW {}
impl FromIntoMemory for NEWTEXTMETRICEXW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 96u32 as usize);
        let f_ntmTm =
            <super::Graphics::Gdi::NEWTEXTMETRICW as FromIntoMemory>::from_bytes(&from[0..0 + 72]);
        let f_ntmFontSig = <FONTSIGNATURE as FromIntoMemory>::from_bytes(&from[72..72 + 24]);
        Self {
            ntmTm: f_ntmTm,
            ntmFontSig: f_ntmFontSig,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 96u32 as usize);
        FromIntoMemory::into_bytes(self.ntmTm, &mut into[0..0 + 72]);
        FromIntoMemory::into_bytes(self.ntmFontSig, &mut into[72..72 + 24]);
    }
    fn size() -> usize {
        96u32 as usize
    }
}
pub struct NLSVERSIONINFO {
    pub dwNLSVersionInfoSize: u32,
    pub dwNLSVersion: u32,
    pub dwDefinedVersion: u32,
    pub dwEffectiveId: u32,
    pub guidCustomVersion: crate::core::GUID,
}
impl ::core::marker::Copy for NLSVERSIONINFO {}
impl ::core::clone::Clone for NLSVERSIONINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NLSVERSIONINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NLSVERSIONINFO")
            .field("dwNLSVersionInfoSize", &self.dwNLSVersionInfoSize)
            .field("dwNLSVersion", &self.dwNLSVersion)
            .field("dwDefinedVersion", &self.dwDefinedVersion)
            .field("dwEffectiveId", &self.dwEffectiveId)
            .field("guidCustomVersion", &self.guidCustomVersion)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NLSVERSIONINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwNLSVersionInfoSize == other.dwNLSVersionInfoSize
            && self.dwNLSVersion == other.dwNLSVersion
            && self.dwDefinedVersion == other.dwDefinedVersion
            && self.dwEffectiveId == other.dwEffectiveId
            && self.guidCustomVersion == other.guidCustomVersion
    }
}
impl ::core::cmp::Eq for NLSVERSIONINFO {}
impl FromIntoMemory for NLSVERSIONINFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32u32 as usize);
        let f_dwNLSVersionInfoSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwNLSVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwDefinedVersion = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwEffectiveId = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_guidCustomVersion =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[16..16 + 16]);
        Self {
            dwNLSVersionInfoSize: f_dwNLSVersionInfoSize,
            dwNLSVersion: f_dwNLSVersion,
            dwDefinedVersion: f_dwDefinedVersion,
            dwEffectiveId: f_dwEffectiveId,
            guidCustomVersion: f_guidCustomVersion,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32u32 as usize);
        FromIntoMemory::into_bytes(self.dwNLSVersionInfoSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwNLSVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwDefinedVersion, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwEffectiveId, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.guidCustomVersion, &mut into[16..16 + 16]);
    }
    fn size() -> usize {
        32u32 as usize
    }
}
pub struct NLSVERSIONINFOEX {
    pub dwNLSVersionInfoSize: u32,
    pub dwNLSVersion: u32,
    pub dwDefinedVersion: u32,
    pub dwEffectiveId: u32,
    pub guidCustomVersion: crate::core::GUID,
}
impl ::core::marker::Copy for NLSVERSIONINFOEX {}
impl ::core::clone::Clone for NLSVERSIONINFOEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NLSVERSIONINFOEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NLSVERSIONINFOEX")
            .field("dwNLSVersionInfoSize", &self.dwNLSVersionInfoSize)
            .field("dwNLSVersion", &self.dwNLSVersion)
            .field("dwDefinedVersion", &self.dwDefinedVersion)
            .field("dwEffectiveId", &self.dwEffectiveId)
            .field("guidCustomVersion", &self.guidCustomVersion)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NLSVERSIONINFOEX {
    fn eq(&self, other: &Self) -> bool {
        self.dwNLSVersionInfoSize == other.dwNLSVersionInfoSize
            && self.dwNLSVersion == other.dwNLSVersion
            && self.dwDefinedVersion == other.dwDefinedVersion
            && self.dwEffectiveId == other.dwEffectiveId
            && self.guidCustomVersion == other.guidCustomVersion
    }
}
impl ::core::cmp::Eq for NLSVERSIONINFOEX {}
impl FromIntoMemory for NLSVERSIONINFOEX {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32u32 as usize);
        let f_dwNLSVersionInfoSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwNLSVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwDefinedVersion = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwEffectiveId = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_guidCustomVersion =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[16..16 + 16]);
        Self {
            dwNLSVersionInfoSize: f_dwNLSVersionInfoSize,
            dwNLSVersion: f_dwNLSVersion,
            dwDefinedVersion: f_dwDefinedVersion,
            dwEffectiveId: f_dwEffectiveId,
            guidCustomVersion: f_guidCustomVersion,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32u32 as usize);
        FromIntoMemory::into_bytes(self.dwNLSVersionInfoSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwNLSVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwDefinedVersion, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwEffectiveId, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.guidCustomVersion, &mut into[16..16 + 16]);
    }
    fn size() -> usize {
        32u32 as usize
    }
}
pub const NLS_CP_CPINFO: u32 = 268435456u32;
pub const NLS_CP_MBTOWC: u32 = 1073741824u32;
pub const NLS_CP_WCTOMB: u32 = 2147483648u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NORM_FORM(pub i32);
pub const NormalizationOther: NORM_FORM = NORM_FORM(0i32);
pub const NormalizationC: NORM_FORM = NORM_FORM(1i32);
pub const NormalizationD: NORM_FORM = NORM_FORM(2i32);
pub const NormalizationKC: NORM_FORM = NORM_FORM(5i32);
pub const NormalizationKD: NORM_FORM = NORM_FORM(6i32);
impl ::core::marker::Copy for NORM_FORM {}
impl ::core::clone::Clone for NORM_FORM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NORM_FORM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NORM_FORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NORM_FORM").field(&self.0).finish()
    }
}
impl FromIntoMemory for NORM_FORM {
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
pub struct NUMBERFMTA {
    pub NumDigits: u32,
    pub LeadingZero: u32,
    pub Grouping: u32,
    pub lpDecimalSep: PSTR,
    pub lpThousandSep: PSTR,
    pub NegativeOrder: u32,
}
impl ::core::marker::Copy for NUMBERFMTA {}
impl ::core::clone::Clone for NUMBERFMTA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NUMBERFMTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NUMBERFMTA")
            .field("NumDigits", &self.NumDigits)
            .field("LeadingZero", &self.LeadingZero)
            .field("Grouping", &self.Grouping)
            .field("lpDecimalSep", &self.lpDecimalSep)
            .field("lpThousandSep", &self.lpThousandSep)
            .field("NegativeOrder", &self.NegativeOrder)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NUMBERFMTA {
    fn eq(&self, other: &Self) -> bool {
        self.NumDigits == other.NumDigits
            && self.LeadingZero == other.LeadingZero
            && self.Grouping == other.Grouping
            && self.lpDecimalSep == other.lpDecimalSep
            && self.lpThousandSep == other.lpThousandSep
            && self.NegativeOrder == other.NegativeOrder
    }
}
impl ::core::cmp::Eq for NUMBERFMTA {}
impl FromIntoMemory for NUMBERFMTA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24u32 as usize);
        let f_NumDigits = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_LeadingZero = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Grouping = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpDecimalSep = <PSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpThousandSep = <PSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_NegativeOrder = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            NumDigits: f_NumDigits,
            LeadingZero: f_LeadingZero,
            Grouping: f_Grouping,
            lpDecimalSep: f_lpDecimalSep,
            lpThousandSep: f_lpThousandSep,
            NegativeOrder: f_NegativeOrder,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24u32 as usize);
        FromIntoMemory::into_bytes(self.NumDigits, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.LeadingZero, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Grouping, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpDecimalSep, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpThousandSep, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.NegativeOrder, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24u32 as usize
    }
}
pub struct NUMBERFMTW {
    pub NumDigits: u32,
    pub LeadingZero: u32,
    pub Grouping: u32,
    pub lpDecimalSep: PWSTR,
    pub lpThousandSep: PWSTR,
    pub NegativeOrder: u32,
}
impl ::core::marker::Copy for NUMBERFMTW {}
impl ::core::clone::Clone for NUMBERFMTW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NUMBERFMTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NUMBERFMTW")
            .field("NumDigits", &self.NumDigits)
            .field("LeadingZero", &self.LeadingZero)
            .field("Grouping", &self.Grouping)
            .field("lpDecimalSep", &self.lpDecimalSep)
            .field("lpThousandSep", &self.lpThousandSep)
            .field("NegativeOrder", &self.NegativeOrder)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NUMBERFMTW {
    fn eq(&self, other: &Self) -> bool {
        self.NumDigits == other.NumDigits
            && self.LeadingZero == other.LeadingZero
            && self.Grouping == other.Grouping
            && self.lpDecimalSep == other.lpDecimalSep
            && self.lpThousandSep == other.lpThousandSep
            && self.NegativeOrder == other.NegativeOrder
    }
}
impl ::core::cmp::Eq for NUMBERFMTW {}
impl FromIntoMemory for NUMBERFMTW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24u32 as usize);
        let f_NumDigits = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_LeadingZero = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Grouping = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpDecimalSep = <PWSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpThousandSep = <PWSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_NegativeOrder = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            NumDigits: f_NumDigits,
            LeadingZero: f_LeadingZero,
            Grouping: f_Grouping,
            lpDecimalSep: f_lpDecimalSep,
            lpThousandSep: f_lpThousandSep,
            NegativeOrder: f_NegativeOrder,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24u32 as usize);
        FromIntoMemory::into_bytes(self.NumDigits, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.LeadingZero, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Grouping, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpDecimalSep, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpThousandSep, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.NegativeOrder, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24u32 as usize
    }
}
pub const NUMSYS_NAME_CAPACITY: u32 = 8u32;
pub const OFFLINE_SERVICES: u32 = 2u32;
pub const ONLINE_SERVICES: u32 = 1u32;
pub type PFN_MAPPINGCALLBACKPROC = StdCallFnPtr<
    (
        MutPtr<MAPPING_PROPERTY_BAG>,
        MutPtr<::core::ffi::c_void>,
        u32,
        crate::core::HRESULT,
    ),
    (),
>;
pub struct RFC1766INFO {
    pub lcid: u32,
    pub wszRfc1766: [u16; 6],
    pub wszLocaleName: [u16; 32],
}
impl ::core::marker::Copy for RFC1766INFO {}
impl ::core::clone::Clone for RFC1766INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RFC1766INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RFC1766INFO")
            .field("lcid", &self.lcid)
            .field("wszRfc1766", &self.wszRfc1766)
            .field("wszLocaleName", &self.wszLocaleName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RFC1766INFO {
    fn eq(&self, other: &Self) -> bool {
        self.lcid == other.lcid
            && self.wszRfc1766 == other.wszRfc1766
            && self.wszLocaleName == other.wszLocaleName
    }
}
impl ::core::cmp::Eq for RFC1766INFO {}
impl FromIntoMemory for RFC1766INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 44u32 as usize);
        let f_lcid = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_wszRfc1766 = <[u16; 6] as FromIntoMemory>::from_bytes(&from[4..4 + 6]);
        let f_wszLocaleName = <[u16; 32] as FromIntoMemory>::from_bytes(&from[10..10 + 32]);
        Self {
            lcid: f_lcid,
            wszRfc1766: f_wszRfc1766,
            wszLocaleName: f_wszLocaleName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 44u32 as usize);
        FromIntoMemory::into_bytes(self.lcid, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.wszRfc1766, &mut into[4..4 + 6]);
        FromIntoMemory::into_bytes(self.wszLocaleName, &mut into[10..10 + 32]);
    }
    fn size() -> usize {
        44u32 as usize
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SCRIPTCONTF(pub i32);
pub const sidDefault: SCRIPTCONTF = SCRIPTCONTF(0i32);
pub const sidMerge: SCRIPTCONTF = SCRIPTCONTF(1i32);
pub const sidAsciiSym: SCRIPTCONTF = SCRIPTCONTF(2i32);
pub const sidAsciiLatin: SCRIPTCONTF = SCRIPTCONTF(3i32);
pub const sidLatin: SCRIPTCONTF = SCRIPTCONTF(4i32);
pub const sidGreek: SCRIPTCONTF = SCRIPTCONTF(5i32);
pub const sidCyrillic: SCRIPTCONTF = SCRIPTCONTF(6i32);
pub const sidArmenian: SCRIPTCONTF = SCRIPTCONTF(7i32);
pub const sidHebrew: SCRIPTCONTF = SCRIPTCONTF(8i32);
pub const sidArabic: SCRIPTCONTF = SCRIPTCONTF(9i32);
pub const sidDevanagari: SCRIPTCONTF = SCRIPTCONTF(10i32);
pub const sidBengali: SCRIPTCONTF = SCRIPTCONTF(11i32);
pub const sidGurmukhi: SCRIPTCONTF = SCRIPTCONTF(12i32);
pub const sidGujarati: SCRIPTCONTF = SCRIPTCONTF(13i32);
pub const sidOriya: SCRIPTCONTF = SCRIPTCONTF(14i32);
pub const sidTamil: SCRIPTCONTF = SCRIPTCONTF(15i32);
pub const sidTelugu: SCRIPTCONTF = SCRIPTCONTF(16i32);
pub const sidKannada: SCRIPTCONTF = SCRIPTCONTF(17i32);
pub const sidMalayalam: SCRIPTCONTF = SCRIPTCONTF(18i32);
pub const sidThai: SCRIPTCONTF = SCRIPTCONTF(19i32);
pub const sidLao: SCRIPTCONTF = SCRIPTCONTF(20i32);
pub const sidTibetan: SCRIPTCONTF = SCRIPTCONTF(21i32);
pub const sidGeorgian: SCRIPTCONTF = SCRIPTCONTF(22i32);
pub const sidHangul: SCRIPTCONTF = SCRIPTCONTF(23i32);
pub const sidKana: SCRIPTCONTF = SCRIPTCONTF(24i32);
pub const sidBopomofo: SCRIPTCONTF = SCRIPTCONTF(25i32);
pub const sidHan: SCRIPTCONTF = SCRIPTCONTF(26i32);
pub const sidEthiopic: SCRIPTCONTF = SCRIPTCONTF(27i32);
pub const sidCanSyllabic: SCRIPTCONTF = SCRIPTCONTF(28i32);
pub const sidCherokee: SCRIPTCONTF = SCRIPTCONTF(29i32);
pub const sidYi: SCRIPTCONTF = SCRIPTCONTF(30i32);
pub const sidBraille: SCRIPTCONTF = SCRIPTCONTF(31i32);
pub const sidRunic: SCRIPTCONTF = SCRIPTCONTF(32i32);
pub const sidOgham: SCRIPTCONTF = SCRIPTCONTF(33i32);
pub const sidSinhala: SCRIPTCONTF = SCRIPTCONTF(34i32);
pub const sidSyriac: SCRIPTCONTF = SCRIPTCONTF(35i32);
pub const sidBurmese: SCRIPTCONTF = SCRIPTCONTF(36i32);
pub const sidKhmer: SCRIPTCONTF = SCRIPTCONTF(37i32);
pub const sidThaana: SCRIPTCONTF = SCRIPTCONTF(38i32);
pub const sidMongolian: SCRIPTCONTF = SCRIPTCONTF(39i32);
pub const sidUserDefined: SCRIPTCONTF = SCRIPTCONTF(40i32);
pub const sidLim: SCRIPTCONTF = SCRIPTCONTF(41i32);
pub const sidFEFirst: SCRIPTCONTF = SCRIPTCONTF(23i32);
pub const sidFELast: SCRIPTCONTF = SCRIPTCONTF(26i32);
impl ::core::marker::Copy for SCRIPTCONTF {}
impl ::core::clone::Clone for SCRIPTCONTF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCRIPTCONTF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCRIPTCONTF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPTCONTF").field(&self.0).finish()
    }
}
impl FromIntoMemory for SCRIPTCONTF {
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
pub struct SCRIPTFONTCONTF(pub i32);
pub const SCRIPTCONTF_FIXED_FONT: SCRIPTFONTCONTF = SCRIPTFONTCONTF(1i32);
pub const SCRIPTCONTF_PROPORTIONAL_FONT: SCRIPTFONTCONTF = SCRIPTFONTCONTF(2i32);
pub const SCRIPTCONTF_SCRIPT_USER: SCRIPTFONTCONTF = SCRIPTFONTCONTF(65536i32);
pub const SCRIPTCONTF_SCRIPT_HIDE: SCRIPTFONTCONTF = SCRIPTFONTCONTF(131072i32);
pub const SCRIPTCONTF_SCRIPT_SYSTEM: SCRIPTFONTCONTF = SCRIPTFONTCONTF(262144i32);
impl ::core::marker::Copy for SCRIPTFONTCONTF {}
impl ::core::clone::Clone for SCRIPTFONTCONTF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCRIPTFONTCONTF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCRIPTFONTCONTF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPTFONTCONTF").field(&self.0).finish()
    }
}
impl FromIntoMemory for SCRIPTFONTCONTF {
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
pub struct SCRIPTINFO {
    pub ScriptId: u8,
    pub uiCodePage: u32,
    pub wszDescription: [u16; 48],
    pub wszFixedWidthFont: [u16; 32],
    pub wszProportionalFont: [u16; 32],
}
impl ::core::marker::Copy for SCRIPTINFO {}
impl ::core::clone::Clone for SCRIPTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCRIPTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCRIPTINFO")
            .field("ScriptId", &self.ScriptId)
            .field("uiCodePage", &self.uiCodePage)
            .field("wszDescription", &self.wszDescription)
            .field("wszFixedWidthFont", &self.wszFixedWidthFont)
            .field("wszProportionalFont", &self.wszProportionalFont)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SCRIPTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.ScriptId == other.ScriptId
            && self.uiCodePage == other.uiCodePage
            && self.wszDescription == other.wszDescription
            && self.wszFixedWidthFont == other.wszFixedWidthFont
            && self.wszProportionalFont == other.wszProportionalFont
    }
}
impl ::core::cmp::Eq for SCRIPTINFO {}
impl FromIntoMemory for SCRIPTINFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 120u32 as usize);
        let f_ScriptId = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_uiCodePage = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_wszDescription = <[u16; 48] as FromIntoMemory>::from_bytes(&from[8..8 + 48]);
        let f_wszFixedWidthFont = <[u16; 32] as FromIntoMemory>::from_bytes(&from[56..56 + 32]);
        let f_wszProportionalFont = <[u16; 32] as FromIntoMemory>::from_bytes(&from[88..88 + 32]);
        Self {
            ScriptId: f_ScriptId,
            uiCodePage: f_uiCodePage,
            wszDescription: f_wszDescription,
            wszFixedWidthFont: f_wszFixedWidthFont,
            wszProportionalFont: f_wszProportionalFont,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 120u32 as usize);
        FromIntoMemory::into_bytes(self.ScriptId, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.uiCodePage, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.wszDescription, &mut into[8..8 + 48]);
        FromIntoMemory::into_bytes(self.wszFixedWidthFont, &mut into[56..56 + 32]);
        FromIntoMemory::into_bytes(self.wszProportionalFont, &mut into[88..88 + 32]);
    }
    fn size() -> usize {
        120u32 as usize
    }
}
pub struct SCRIPT_ANALYSIS {
    pub _bitfield: u16,
    pub s: SCRIPT_STATE,
}
impl ::core::marker::Copy for SCRIPT_ANALYSIS {}
impl ::core::clone::Clone for SCRIPT_ANALYSIS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCRIPT_ANALYSIS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCRIPT_ANALYSIS")
            .field("_bitfield", &self._bitfield)
            .field("s", &self.s)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SCRIPT_ANALYSIS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield && self.s == other.s
    }
}
impl ::core::cmp::Eq for SCRIPT_ANALYSIS {}
impl FromIntoMemory for SCRIPT_ANALYSIS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4u32 as usize);
        let f__bitfield = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_s = <SCRIPT_STATE as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        Self {
            _bitfield: f__bitfield,
            s: f_s,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4u32 as usize);
        FromIntoMemory::into_bytes(self._bitfield, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.s, &mut into[2..2 + 2]);
    }
    fn size() -> usize {
        4u32 as usize
    }
}
pub struct SCRIPT_CONTROL {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for SCRIPT_CONTROL {}
impl ::core::clone::Clone for SCRIPT_CONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCRIPT_CONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCRIPT_CONTROL")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SCRIPT_CONTROL {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for SCRIPT_CONTROL {}
impl FromIntoMemory for SCRIPT_CONTROL {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4u32 as usize);
        let f__bitfield = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            _bitfield: f__bitfield,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4u32 as usize);
        FromIntoMemory::into_bytes(self._bitfield, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4u32 as usize
    }
}
pub struct SCRIPT_DIGITSUBSTITUTE {
    pub _bitfield1: u32,
    pub _bitfield2: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for SCRIPT_DIGITSUBSTITUTE {}
impl ::core::clone::Clone for SCRIPT_DIGITSUBSTITUTE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCRIPT_DIGITSUBSTITUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCRIPT_DIGITSUBSTITUTE")
            .field("_bitfield1", &self._bitfield1)
            .field("_bitfield2", &self._bitfield2)
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SCRIPT_DIGITSUBSTITUTE {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield1 == other._bitfield1
            && self._bitfield2 == other._bitfield2
            && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for SCRIPT_DIGITSUBSTITUTE {}
impl FromIntoMemory for SCRIPT_DIGITSUBSTITUTE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12u32 as usize);
        let f__bitfield1 = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f__bitfield2 = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwReserved = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            _bitfield1: f__bitfield1,
            _bitfield2: f__bitfield2,
            dwReserved: f_dwReserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12u32 as usize);
        FromIntoMemory::into_bytes(self._bitfield1, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self._bitfield2, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwReserved, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12u32 as usize
    }
}
pub const SCRIPT_DIGITSUBSTITUTE_CONTEXT: u32 = 0u32;
pub const SCRIPT_DIGITSUBSTITUTE_NATIONAL: u32 = 2u32;
pub const SCRIPT_DIGITSUBSTITUTE_NONE: u32 = 1u32;
pub const SCRIPT_DIGITSUBSTITUTE_TRADITIONAL: u32 = 3u32;
pub struct SCRIPT_FONTPROPERTIES {
    pub cBytes: i32,
    pub wgBlank: u16,
    pub wgDefault: u16,
    pub wgInvalid: u16,
    pub wgKashida: u16,
    pub iKashidaWidth: i32,
}
impl ::core::marker::Copy for SCRIPT_FONTPROPERTIES {}
impl ::core::clone::Clone for SCRIPT_FONTPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCRIPT_FONTPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCRIPT_FONTPROPERTIES")
            .field("cBytes", &self.cBytes)
            .field("wgBlank", &self.wgBlank)
            .field("wgDefault", &self.wgDefault)
            .field("wgInvalid", &self.wgInvalid)
            .field("wgKashida", &self.wgKashida)
            .field("iKashidaWidth", &self.iKashidaWidth)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SCRIPT_FONTPROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.cBytes == other.cBytes
            && self.wgBlank == other.wgBlank
            && self.wgDefault == other.wgDefault
            && self.wgInvalid == other.wgInvalid
            && self.wgKashida == other.wgKashida
            && self.iKashidaWidth == other.iKashidaWidth
    }
}
impl ::core::cmp::Eq for SCRIPT_FONTPROPERTIES {}
impl FromIntoMemory for SCRIPT_FONTPROPERTIES {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16u32 as usize);
        let f_cBytes = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_wgBlank = <u16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_wgDefault = <u16 as FromIntoMemory>::from_bytes(&from[6..6 + 2]);
        let f_wgInvalid = <u16 as FromIntoMemory>::from_bytes(&from[8..8 + 2]);
        let f_wgKashida = <u16 as FromIntoMemory>::from_bytes(&from[10..10 + 2]);
        let f_iKashidaWidth = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            cBytes: f_cBytes,
            wgBlank: f_wgBlank,
            wgDefault: f_wgDefault,
            wgInvalid: f_wgInvalid,
            wgKashida: f_wgKashida,
            iKashidaWidth: f_iKashidaWidth,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16u32 as usize);
        FromIntoMemory::into_bytes(self.cBytes, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.wgBlank, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.wgDefault, &mut into[6..6 + 2]);
        FromIntoMemory::into_bytes(self.wgInvalid, &mut into[8..8 + 2]);
        FromIntoMemory::into_bytes(self.wgKashida, &mut into[10..10 + 2]);
        FromIntoMemory::into_bytes(self.iKashidaWidth, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16u32 as usize
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SCRIPT_IS_COMPLEX_FLAGS(pub u32);
pub const SIC_ASCIIDIGIT: SCRIPT_IS_COMPLEX_FLAGS = SCRIPT_IS_COMPLEX_FLAGS(2u32);
pub const SIC_COMPLEX: SCRIPT_IS_COMPLEX_FLAGS = SCRIPT_IS_COMPLEX_FLAGS(1u32);
pub const SIC_NEUTRAL: SCRIPT_IS_COMPLEX_FLAGS = SCRIPT_IS_COMPLEX_FLAGS(4u32);
impl ::core::marker::Copy for SCRIPT_IS_COMPLEX_FLAGS {}
impl ::core::clone::Clone for SCRIPT_IS_COMPLEX_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCRIPT_IS_COMPLEX_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCRIPT_IS_COMPLEX_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPT_IS_COMPLEX_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for SCRIPT_IS_COMPLEX_FLAGS {
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
pub struct SCRIPT_ITEM {
    pub iCharPos: i32,
    pub a: SCRIPT_ANALYSIS,
}
impl ::core::marker::Copy for SCRIPT_ITEM {}
impl ::core::clone::Clone for SCRIPT_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCRIPT_ITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCRIPT_ITEM")
            .field("iCharPos", &self.iCharPos)
            .field("a", &self.a)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SCRIPT_ITEM {
    fn eq(&self, other: &Self) -> bool {
        self.iCharPos == other.iCharPos && self.a == other.a
    }
}
impl ::core::cmp::Eq for SCRIPT_ITEM {}
impl FromIntoMemory for SCRIPT_ITEM {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8u32 as usize);
        let f_iCharPos = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_a = <SCRIPT_ANALYSIS as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            iCharPos: f_iCharPos,
            a: f_a,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8u32 as usize);
        FromIntoMemory::into_bytes(self.iCharPos, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.a, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8u32 as usize
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SCRIPT_JUSTIFY(pub i32);
pub const SCRIPT_JUSTIFY_NONE: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(0i32);
pub const SCRIPT_JUSTIFY_ARABIC_BLANK: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(1i32);
pub const SCRIPT_JUSTIFY_CHARACTER: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(2i32);
pub const SCRIPT_JUSTIFY_RESERVED1: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(3i32);
pub const SCRIPT_JUSTIFY_BLANK: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(4i32);
pub const SCRIPT_JUSTIFY_RESERVED2: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(5i32);
pub const SCRIPT_JUSTIFY_RESERVED3: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(6i32);
pub const SCRIPT_JUSTIFY_ARABIC_NORMAL: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(7i32);
pub const SCRIPT_JUSTIFY_ARABIC_KASHIDA: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(8i32);
pub const SCRIPT_JUSTIFY_ARABIC_ALEF: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(9i32);
pub const SCRIPT_JUSTIFY_ARABIC_HA: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(10i32);
pub const SCRIPT_JUSTIFY_ARABIC_RA: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(11i32);
pub const SCRIPT_JUSTIFY_ARABIC_BA: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(12i32);
pub const SCRIPT_JUSTIFY_ARABIC_BARA: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(13i32);
pub const SCRIPT_JUSTIFY_ARABIC_SEEN: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(14i32);
pub const SCRIPT_JUSTIFY_ARABIC_SEEN_M: SCRIPT_JUSTIFY = SCRIPT_JUSTIFY(15i32);
impl ::core::marker::Copy for SCRIPT_JUSTIFY {}
impl ::core::clone::Clone for SCRIPT_JUSTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCRIPT_JUSTIFY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCRIPT_JUSTIFY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPT_JUSTIFY").field(&self.0).finish()
    }
}
impl FromIntoMemory for SCRIPT_JUSTIFY {
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
pub struct SCRIPT_LOGATTR {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for SCRIPT_LOGATTR {}
impl ::core::clone::Clone for SCRIPT_LOGATTR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCRIPT_LOGATTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCRIPT_LOGATTR")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SCRIPT_LOGATTR {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for SCRIPT_LOGATTR {}
impl FromIntoMemory for SCRIPT_LOGATTR {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 1u32 as usize);
        let f__bitfield = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        Self {
            _bitfield: f__bitfield,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 1u32 as usize);
        FromIntoMemory::into_bytes(self._bitfield, &mut into[0..0 + 1]);
    }
    fn size() -> usize {
        1u32 as usize
    }
}
pub struct SCRIPT_PROPERTIES {
    pub _bitfield1: u32,
    pub _bitfield2: u32,
}
impl ::core::marker::Copy for SCRIPT_PROPERTIES {}
impl ::core::clone::Clone for SCRIPT_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCRIPT_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCRIPT_PROPERTIES")
            .field("_bitfield1", &self._bitfield1)
            .field("_bitfield2", &self._bitfield2)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SCRIPT_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield1 == other._bitfield1 && self._bitfield2 == other._bitfield2
    }
}
impl ::core::cmp::Eq for SCRIPT_PROPERTIES {}
impl FromIntoMemory for SCRIPT_PROPERTIES {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8u32 as usize);
        let f__bitfield1 = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f__bitfield2 = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            _bitfield1: f__bitfield1,
            _bitfield2: f__bitfield2,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8u32 as usize);
        FromIntoMemory::into_bytes(self._bitfield1, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self._bitfield2, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8u32 as usize
    }
}
pub struct SCRIPT_STATE {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for SCRIPT_STATE {}
impl ::core::clone::Clone for SCRIPT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCRIPT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCRIPT_STATE")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SCRIPT_STATE {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for SCRIPT_STATE {}
impl FromIntoMemory for SCRIPT_STATE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 2u32 as usize);
        let f__bitfield = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        Self {
            _bitfield: f__bitfield,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 2u32 as usize);
        FromIntoMemory::into_bytes(self._bitfield, &mut into[0..0 + 2]);
    }
    fn size() -> usize {
        2u32 as usize
    }
}
pub struct SCRIPT_TABDEF {
    pub cTabStops: i32,
    pub iScale: i32,
    pub pTabStops: MutPtr<i32>,
    pub iTabOrigin: i32,
}
impl ::core::marker::Copy for SCRIPT_TABDEF {}
impl ::core::clone::Clone for SCRIPT_TABDEF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCRIPT_TABDEF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCRIPT_TABDEF")
            .field("cTabStops", &self.cTabStops)
            .field("iScale", &self.iScale)
            .field("pTabStops", &self.pTabStops)
            .field("iTabOrigin", &self.iTabOrigin)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SCRIPT_TABDEF {
    fn eq(&self, other: &Self) -> bool {
        self.cTabStops == other.cTabStops
            && self.iScale == other.iScale
            && self.pTabStops == other.pTabStops
            && self.iTabOrigin == other.iTabOrigin
    }
}
impl ::core::cmp::Eq for SCRIPT_TABDEF {}
impl FromIntoMemory for SCRIPT_TABDEF {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16u32 as usize);
        let f_cTabStops = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_iScale = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_pTabStops = <MutPtr<i32> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_iTabOrigin = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            cTabStops: f_cTabStops,
            iScale: f_iScale,
            pTabStops: f_pTabStops,
            iTabOrigin: f_iTabOrigin,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16u32 as usize);
        FromIntoMemory::into_bytes(self.cTabStops, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.iScale, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.pTabStops, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.iTabOrigin, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16u32 as usize
    }
}
pub const SCRIPT_TAG_UNKNOWN: u32 = 0u32;
pub const SCRIPT_UNDEFINED: u32 = 0u32;
pub struct SCRIPT_VISATTR {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for SCRIPT_VISATTR {}
impl ::core::clone::Clone for SCRIPT_VISATTR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCRIPT_VISATTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCRIPT_VISATTR")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SCRIPT_VISATTR {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for SCRIPT_VISATTR {}
impl FromIntoMemory for SCRIPT_VISATTR {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 2u32 as usize);
        let f__bitfield = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        Self {
            _bitfield: f__bitfield,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 2u32 as usize);
        FromIntoMemory::into_bytes(self._bitfield, &mut into[0..0 + 2]);
    }
    fn size() -> usize {
        2u32 as usize
    }
}
pub const SGCM_RTL: u32 = 1u32;
pub const SORTING_PARADIGM_ICU: u32 = 16777216u32;
pub const SORTING_PARADIGM_NLS: u32 = 0u32;
pub const SSA_BREAK: u32 = 64u32;
pub const SSA_CLIP: u32 = 4u32;
pub const SSA_DONTGLYPH: u32 = 1073741824u32;
pub const SSA_DZWG: u32 = 16u32;
pub const SSA_FALLBACK: u32 = 32u32;
pub const SSA_FIT: u32 = 8u32;
pub const SSA_FULLMEASURE: u32 = 67108864u32;
pub const SSA_GCP: u32 = 512u32;
pub const SSA_GLYPHS: u32 = 128u32;
pub const SSA_HIDEHOTKEY: u32 = 8192u32;
pub const SSA_HOTKEY: u32 = 1024u32;
pub const SSA_HOTKEYONLY: u32 = 9216u32;
pub const SSA_LAYOUTRTL: u32 = 536870912u32;
pub const SSA_LINK: u32 = 4096u32;
pub const SSA_LPKANSIFALLBACK: u32 = 134217728u32;
pub const SSA_METAFILE: u32 = 2048u32;
pub const SSA_NOKASHIDA: u32 = 2147483648u32;
pub const SSA_PASSWORD: u32 = 1u32;
pub const SSA_PIDX: u32 = 268435456u32;
pub const SSA_RTL: u32 = 256u32;
pub const SSA_TAB: u32 = 2u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SYSGEOCLASS(pub i32);
pub const GEOCLASS_NATION: SYSGEOCLASS = SYSGEOCLASS(16i32);
pub const GEOCLASS_REGION: SYSGEOCLASS = SYSGEOCLASS(14i32);
pub const GEOCLASS_ALL: SYSGEOCLASS = SYSGEOCLASS(0i32);
impl ::core::marker::Copy for SYSGEOCLASS {}
impl ::core::clone::Clone for SYSGEOCLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYSGEOCLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYSGEOCLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSGEOCLASS").field(&self.0).finish()
    }
}
impl FromIntoMemory for SYSGEOCLASS {
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
pub struct SYSGEOTYPE(pub i32);
pub const GEO_NATION: SYSGEOTYPE = SYSGEOTYPE(1i32);
pub const GEO_LATITUDE: SYSGEOTYPE = SYSGEOTYPE(2i32);
pub const GEO_LONGITUDE: SYSGEOTYPE = SYSGEOTYPE(3i32);
pub const GEO_ISO2: SYSGEOTYPE = SYSGEOTYPE(4i32);
pub const GEO_ISO3: SYSGEOTYPE = SYSGEOTYPE(5i32);
pub const GEO_RFC1766: SYSGEOTYPE = SYSGEOTYPE(6i32);
pub const GEO_LCID: SYSGEOTYPE = SYSGEOTYPE(7i32);
pub const GEO_FRIENDLYNAME: SYSGEOTYPE = SYSGEOTYPE(8i32);
pub const GEO_OFFICIALNAME: SYSGEOTYPE = SYSGEOTYPE(9i32);
pub const GEO_TIMEZONES: SYSGEOTYPE = SYSGEOTYPE(10i32);
pub const GEO_OFFICIALLANGUAGES: SYSGEOTYPE = SYSGEOTYPE(11i32);
pub const GEO_ISO_UN_NUMBER: SYSGEOTYPE = SYSGEOTYPE(12i32);
pub const GEO_PARENT: SYSGEOTYPE = SYSGEOTYPE(13i32);
pub const GEO_DIALINGCODE: SYSGEOTYPE = SYSGEOTYPE(14i32);
pub const GEO_CURRENCYCODE: SYSGEOTYPE = SYSGEOTYPE(15i32);
pub const GEO_CURRENCYSYMBOL: SYSGEOTYPE = SYSGEOTYPE(16i32);
pub const GEO_NAME: SYSGEOTYPE = SYSGEOTYPE(17i32);
pub const GEO_ID: SYSGEOTYPE = SYSGEOTYPE(18i32);
impl ::core::marker::Copy for SYSGEOTYPE {}
impl ::core::clone::Clone for SYSGEOTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYSGEOTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYSGEOTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSGEOTYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for SYSGEOTYPE {
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
pub struct SYSNLS_FUNCTION(pub i32);
pub const COMPARE_STRING: SYSNLS_FUNCTION = SYSNLS_FUNCTION(1i32);
impl ::core::marker::Copy for SYSNLS_FUNCTION {}
impl ::core::clone::Clone for SYSNLS_FUNCTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYSNLS_FUNCTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYSNLS_FUNCTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSNLS_FUNCTION").field(&self.0).finish()
    }
}
impl FromIntoMemory for SYSNLS_FUNCTION {
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
pub const SpellCheckerFactory: crate::core::GUID =
    crate::core::GUID::from_u128(0x7ab36653_1796_484b_bdfa_e74f1db7c1dc);
pub type TIMEFMT_ENUMPROCA = StdCallFnPtr<(PCSTR,), super::Foundation::BOOL>;
pub type TIMEFMT_ENUMPROCEX =
    StdCallFnPtr<(PCWSTR, super::Foundation::LPARAM), super::Foundation::BOOL>;
pub type TIMEFMT_ENUMPROCW = StdCallFnPtr<(PCWSTR,), super::Foundation::BOOL>;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TIME_FORMAT_FLAGS(pub u32);
pub const TIME_NOMINUTESORSECONDS: TIME_FORMAT_FLAGS = TIME_FORMAT_FLAGS(1u32);
pub const TIME_NOSECONDS: TIME_FORMAT_FLAGS = TIME_FORMAT_FLAGS(2u32);
pub const TIME_NOTIMEMARKER: TIME_FORMAT_FLAGS = TIME_FORMAT_FLAGS(4u32);
pub const TIME_FORCE24HOURFORMAT: TIME_FORMAT_FLAGS = TIME_FORMAT_FLAGS(8u32);
impl ::core::marker::Copy for TIME_FORMAT_FLAGS {}
impl ::core::clone::Clone for TIME_FORMAT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TIME_FORMAT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TIME_FORMAT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TIME_FORMAT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TIME_FORMAT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TIME_FORMAT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TIME_FORMAT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TIME_FORMAT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TIME_FORMAT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for TIME_FORMAT_FLAGS {
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
pub struct TRANSLATE_CHARSET_INFO_FLAGS(pub u32);
pub const TCI_SRCCHARSET: TRANSLATE_CHARSET_INFO_FLAGS = TRANSLATE_CHARSET_INFO_FLAGS(1u32);
pub const TCI_SRCCODEPAGE: TRANSLATE_CHARSET_INFO_FLAGS = TRANSLATE_CHARSET_INFO_FLAGS(2u32);
pub const TCI_SRCFONTSIG: TRANSLATE_CHARSET_INFO_FLAGS = TRANSLATE_CHARSET_INFO_FLAGS(3u32);
pub const TCI_SRCLOCALE: TRANSLATE_CHARSET_INFO_FLAGS = TRANSLATE_CHARSET_INFO_FLAGS(4096u32);
impl ::core::marker::Copy for TRANSLATE_CHARSET_INFO_FLAGS {}
impl ::core::clone::Clone for TRANSLATE_CHARSET_INFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TRANSLATE_CHARSET_INFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TRANSLATE_CHARSET_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRANSLATE_CHARSET_INFO_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for TRANSLATE_CHARSET_INFO_FLAGS {
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
pub const U16_MAX_LENGTH: u32 = 2u32;
pub const U8_LEAD3_T1_BITS: &'static str = " 000000000000\u{10}00";
pub const U8_LEAD4_T1_BITS: &'static str =
    "\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{1e}\u{f}\u{f}\u{f}\u{0}\u{0}\u{0}\u{0}";
pub const U8_MAX_LENGTH: u32 = 4u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UAcceptResult(pub i32);
pub const ULOC_ACCEPT_FAILED: UAcceptResult = UAcceptResult(0i32);
pub const ULOC_ACCEPT_VALID: UAcceptResult = UAcceptResult(1i32);
pub const ULOC_ACCEPT_FALLBACK: UAcceptResult = UAcceptResult(2i32);
impl ::core::marker::Copy for UAcceptResult {}
impl ::core::clone::Clone for UAcceptResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UAcceptResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UAcceptResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UAcceptResult").field(&self.0).finish()
    }
}
impl FromIntoMemory for UAcceptResult {
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
pub struct UAlphabeticIndexLabelType(pub i32);
pub const U_ALPHAINDEX_NORMAL: UAlphabeticIndexLabelType = UAlphabeticIndexLabelType(0i32);
pub const U_ALPHAINDEX_UNDERFLOW: UAlphabeticIndexLabelType = UAlphabeticIndexLabelType(1i32);
pub const U_ALPHAINDEX_INFLOW: UAlphabeticIndexLabelType = UAlphabeticIndexLabelType(2i32);
pub const U_ALPHAINDEX_OVERFLOW: UAlphabeticIndexLabelType = UAlphabeticIndexLabelType(3i32);
impl ::core::marker::Copy for UAlphabeticIndexLabelType {}
impl ::core::clone::Clone for UAlphabeticIndexLabelType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UAlphabeticIndexLabelType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UAlphabeticIndexLabelType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UAlphabeticIndexLabelType")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UAlphabeticIndexLabelType {
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
pub const UBIDI_DEFAULT_LTR: u32 = 254u32;
pub const UBIDI_DEFAULT_RTL: u32 = 255u32;
pub const UBIDI_DO_MIRRORING: u32 = 2u32;
pub const UBIDI_INSERT_LRM_FOR_NUMERIC: u32 = 4u32;
pub const UBIDI_KEEP_BASE_COMBINING: u32 = 1u32;
pub const UBIDI_LEVEL_OVERRIDE: u32 = 128u32;
pub const UBIDI_MAP_NOWHERE: i32 = -1i32;
pub const UBIDI_MAX_EXPLICIT_LEVEL: u32 = 125u32;
pub const UBIDI_OUTPUT_REVERSE: u32 = 16u32;
pub const UBIDI_REMOVE_BIDI_CONTROLS: u32 = 8u32;
pub struct UBiDi(pub u8);
pub type UBiDiClassCallback = StdCallFnPtr<(ConstPtr<::core::ffi::c_void>, i32), UCharDirection>;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UBiDiDirection(pub i32);
pub const UBIDI_LTR: UBiDiDirection = UBiDiDirection(0i32);
pub const UBIDI_RTL: UBiDiDirection = UBiDiDirection(1i32);
pub const UBIDI_MIXED: UBiDiDirection = UBiDiDirection(2i32);
pub const UBIDI_NEUTRAL: UBiDiDirection = UBiDiDirection(3i32);
impl ::core::marker::Copy for UBiDiDirection {}
impl ::core::clone::Clone for UBiDiDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UBiDiDirection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UBiDiDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UBiDiDirection").field(&self.0).finish()
    }
}
impl FromIntoMemory for UBiDiDirection {
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
pub struct UBiDiMirroring(pub i32);
pub const UBIDI_MIRRORING_OFF: UBiDiMirroring = UBiDiMirroring(0i32);
pub const UBIDI_MIRRORING_ON: UBiDiMirroring = UBiDiMirroring(1i32);
impl ::core::marker::Copy for UBiDiMirroring {}
impl ::core::clone::Clone for UBiDiMirroring {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UBiDiMirroring {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UBiDiMirroring {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UBiDiMirroring").field(&self.0).finish()
    }
}
impl FromIntoMemory for UBiDiMirroring {
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
pub struct UBiDiOrder(pub i32);
pub const UBIDI_LOGICAL: UBiDiOrder = UBiDiOrder(0i32);
pub const UBIDI_VISUAL: UBiDiOrder = UBiDiOrder(1i32);
impl ::core::marker::Copy for UBiDiOrder {}
impl ::core::clone::Clone for UBiDiOrder {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UBiDiOrder {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UBiDiOrder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UBiDiOrder").field(&self.0).finish()
    }
}
impl FromIntoMemory for UBiDiOrder {
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
pub struct UBiDiReorderingMode(pub i32);
pub const UBIDI_REORDER_DEFAULT: UBiDiReorderingMode = UBiDiReorderingMode(0i32);
pub const UBIDI_REORDER_NUMBERS_SPECIAL: UBiDiReorderingMode = UBiDiReorderingMode(1i32);
pub const UBIDI_REORDER_GROUP_NUMBERS_WITH_R: UBiDiReorderingMode = UBiDiReorderingMode(2i32);
pub const UBIDI_REORDER_RUNS_ONLY: UBiDiReorderingMode = UBiDiReorderingMode(3i32);
pub const UBIDI_REORDER_INVERSE_NUMBERS_AS_L: UBiDiReorderingMode = UBiDiReorderingMode(4i32);
pub const UBIDI_REORDER_INVERSE_LIKE_DIRECT: UBiDiReorderingMode = UBiDiReorderingMode(5i32);
pub const UBIDI_REORDER_INVERSE_FOR_NUMBERS_SPECIAL: UBiDiReorderingMode =
    UBiDiReorderingMode(6i32);
impl ::core::marker::Copy for UBiDiReorderingMode {}
impl ::core::clone::Clone for UBiDiReorderingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UBiDiReorderingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UBiDiReorderingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UBiDiReorderingMode").field(&self.0).finish()
    }
}
impl FromIntoMemory for UBiDiReorderingMode {
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
pub struct UBiDiReorderingOption(pub i32);
pub const UBIDI_OPTION_DEFAULT: UBiDiReorderingOption = UBiDiReorderingOption(0i32);
pub const UBIDI_OPTION_INSERT_MARKS: UBiDiReorderingOption = UBiDiReorderingOption(1i32);
pub const UBIDI_OPTION_REMOVE_CONTROLS: UBiDiReorderingOption = UBiDiReorderingOption(2i32);
pub const UBIDI_OPTION_STREAMING: UBiDiReorderingOption = UBiDiReorderingOption(4i32);
impl ::core::marker::Copy for UBiDiReorderingOption {}
impl ::core::clone::Clone for UBiDiReorderingOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UBiDiReorderingOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UBiDiReorderingOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UBiDiReorderingOption")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UBiDiReorderingOption {
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
pub struct UBiDiTransform(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UBidiPairedBracketType(pub i32);
pub const U_BPT_NONE: UBidiPairedBracketType = UBidiPairedBracketType(0i32);
pub const U_BPT_OPEN: UBidiPairedBracketType = UBidiPairedBracketType(1i32);
pub const U_BPT_CLOSE: UBidiPairedBracketType = UBidiPairedBracketType(2i32);
impl ::core::marker::Copy for UBidiPairedBracketType {}
impl ::core::clone::Clone for UBidiPairedBracketType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UBidiPairedBracketType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UBidiPairedBracketType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UBidiPairedBracketType")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UBidiPairedBracketType {
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
pub struct UBlockCode(pub i32);
pub const UBLOCK_NO_BLOCK: UBlockCode = UBlockCode(0i32);
pub const UBLOCK_BASIC_LATIN: UBlockCode = UBlockCode(1i32);
pub const UBLOCK_LATIN_1_SUPPLEMENT: UBlockCode = UBlockCode(2i32);
pub const UBLOCK_LATIN_EXTENDED_A: UBlockCode = UBlockCode(3i32);
pub const UBLOCK_LATIN_EXTENDED_B: UBlockCode = UBlockCode(4i32);
pub const UBLOCK_IPA_EXTENSIONS: UBlockCode = UBlockCode(5i32);
pub const UBLOCK_SPACING_MODIFIER_LETTERS: UBlockCode = UBlockCode(6i32);
pub const UBLOCK_COMBINING_DIACRITICAL_MARKS: UBlockCode = UBlockCode(7i32);
pub const UBLOCK_GREEK: UBlockCode = UBlockCode(8i32);
pub const UBLOCK_CYRILLIC: UBlockCode = UBlockCode(9i32);
pub const UBLOCK_ARMENIAN: UBlockCode = UBlockCode(10i32);
pub const UBLOCK_HEBREW: UBlockCode = UBlockCode(11i32);
pub const UBLOCK_ARABIC: UBlockCode = UBlockCode(12i32);
pub const UBLOCK_SYRIAC: UBlockCode = UBlockCode(13i32);
pub const UBLOCK_THAANA: UBlockCode = UBlockCode(14i32);
pub const UBLOCK_DEVANAGARI: UBlockCode = UBlockCode(15i32);
pub const UBLOCK_BENGALI: UBlockCode = UBlockCode(16i32);
pub const UBLOCK_GURMUKHI: UBlockCode = UBlockCode(17i32);
pub const UBLOCK_GUJARATI: UBlockCode = UBlockCode(18i32);
pub const UBLOCK_ORIYA: UBlockCode = UBlockCode(19i32);
pub const UBLOCK_TAMIL: UBlockCode = UBlockCode(20i32);
pub const UBLOCK_TELUGU: UBlockCode = UBlockCode(21i32);
pub const UBLOCK_KANNADA: UBlockCode = UBlockCode(22i32);
pub const UBLOCK_MALAYALAM: UBlockCode = UBlockCode(23i32);
pub const UBLOCK_SINHALA: UBlockCode = UBlockCode(24i32);
pub const UBLOCK_THAI: UBlockCode = UBlockCode(25i32);
pub const UBLOCK_LAO: UBlockCode = UBlockCode(26i32);
pub const UBLOCK_TIBETAN: UBlockCode = UBlockCode(27i32);
pub const UBLOCK_MYANMAR: UBlockCode = UBlockCode(28i32);
pub const UBLOCK_GEORGIAN: UBlockCode = UBlockCode(29i32);
pub const UBLOCK_HANGUL_JAMO: UBlockCode = UBlockCode(30i32);
pub const UBLOCK_ETHIOPIC: UBlockCode = UBlockCode(31i32);
pub const UBLOCK_CHEROKEE: UBlockCode = UBlockCode(32i32);
pub const UBLOCK_UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS: UBlockCode = UBlockCode(33i32);
pub const UBLOCK_OGHAM: UBlockCode = UBlockCode(34i32);
pub const UBLOCK_RUNIC: UBlockCode = UBlockCode(35i32);
pub const UBLOCK_KHMER: UBlockCode = UBlockCode(36i32);
pub const UBLOCK_MONGOLIAN: UBlockCode = UBlockCode(37i32);
pub const UBLOCK_LATIN_EXTENDED_ADDITIONAL: UBlockCode = UBlockCode(38i32);
pub const UBLOCK_GREEK_EXTENDED: UBlockCode = UBlockCode(39i32);
pub const UBLOCK_GENERAL_PUNCTUATION: UBlockCode = UBlockCode(40i32);
pub const UBLOCK_SUPERSCRIPTS_AND_SUBSCRIPTS: UBlockCode = UBlockCode(41i32);
pub const UBLOCK_CURRENCY_SYMBOLS: UBlockCode = UBlockCode(42i32);
pub const UBLOCK_COMBINING_MARKS_FOR_SYMBOLS: UBlockCode = UBlockCode(43i32);
pub const UBLOCK_LETTERLIKE_SYMBOLS: UBlockCode = UBlockCode(44i32);
pub const UBLOCK_NUMBER_FORMS: UBlockCode = UBlockCode(45i32);
pub const UBLOCK_ARROWS: UBlockCode = UBlockCode(46i32);
pub const UBLOCK_MATHEMATICAL_OPERATORS: UBlockCode = UBlockCode(47i32);
pub const UBLOCK_MISCELLANEOUS_TECHNICAL: UBlockCode = UBlockCode(48i32);
pub const UBLOCK_CONTROL_PICTURES: UBlockCode = UBlockCode(49i32);
pub const UBLOCK_OPTICAL_CHARACTER_RECOGNITION: UBlockCode = UBlockCode(50i32);
pub const UBLOCK_ENCLOSED_ALPHANUMERICS: UBlockCode = UBlockCode(51i32);
pub const UBLOCK_BOX_DRAWING: UBlockCode = UBlockCode(52i32);
pub const UBLOCK_BLOCK_ELEMENTS: UBlockCode = UBlockCode(53i32);
pub const UBLOCK_GEOMETRIC_SHAPES: UBlockCode = UBlockCode(54i32);
pub const UBLOCK_MISCELLANEOUS_SYMBOLS: UBlockCode = UBlockCode(55i32);
pub const UBLOCK_DINGBATS: UBlockCode = UBlockCode(56i32);
pub const UBLOCK_BRAILLE_PATTERNS: UBlockCode = UBlockCode(57i32);
pub const UBLOCK_CJK_RADICALS_SUPPLEMENT: UBlockCode = UBlockCode(58i32);
pub const UBLOCK_KANGXI_RADICALS: UBlockCode = UBlockCode(59i32);
pub const UBLOCK_IDEOGRAPHIC_DESCRIPTION_CHARACTERS: UBlockCode = UBlockCode(60i32);
pub const UBLOCK_CJK_SYMBOLS_AND_PUNCTUATION: UBlockCode = UBlockCode(61i32);
pub const UBLOCK_HIRAGANA: UBlockCode = UBlockCode(62i32);
pub const UBLOCK_KATAKANA: UBlockCode = UBlockCode(63i32);
pub const UBLOCK_BOPOMOFO: UBlockCode = UBlockCode(64i32);
pub const UBLOCK_HANGUL_COMPATIBILITY_JAMO: UBlockCode = UBlockCode(65i32);
pub const UBLOCK_KANBUN: UBlockCode = UBlockCode(66i32);
pub const UBLOCK_BOPOMOFO_EXTENDED: UBlockCode = UBlockCode(67i32);
pub const UBLOCK_ENCLOSED_CJK_LETTERS_AND_MONTHS: UBlockCode = UBlockCode(68i32);
pub const UBLOCK_CJK_COMPATIBILITY: UBlockCode = UBlockCode(69i32);
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_A: UBlockCode = UBlockCode(70i32);
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS: UBlockCode = UBlockCode(71i32);
pub const UBLOCK_YI_SYLLABLES: UBlockCode = UBlockCode(72i32);
pub const UBLOCK_YI_RADICALS: UBlockCode = UBlockCode(73i32);
pub const UBLOCK_HANGUL_SYLLABLES: UBlockCode = UBlockCode(74i32);
pub const UBLOCK_HIGH_SURROGATES: UBlockCode = UBlockCode(75i32);
pub const UBLOCK_HIGH_PRIVATE_USE_SURROGATES: UBlockCode = UBlockCode(76i32);
pub const UBLOCK_LOW_SURROGATES: UBlockCode = UBlockCode(77i32);
pub const UBLOCK_PRIVATE_USE_AREA: UBlockCode = UBlockCode(78i32);
pub const UBLOCK_PRIVATE_USE: UBlockCode = UBlockCode(78i32);
pub const UBLOCK_CJK_COMPATIBILITY_IDEOGRAPHS: UBlockCode = UBlockCode(79i32);
pub const UBLOCK_ALPHABETIC_PRESENTATION_FORMS: UBlockCode = UBlockCode(80i32);
pub const UBLOCK_ARABIC_PRESENTATION_FORMS_A: UBlockCode = UBlockCode(81i32);
pub const UBLOCK_COMBINING_HALF_MARKS: UBlockCode = UBlockCode(82i32);
pub const UBLOCK_CJK_COMPATIBILITY_FORMS: UBlockCode = UBlockCode(83i32);
pub const UBLOCK_SMALL_FORM_VARIANTS: UBlockCode = UBlockCode(84i32);
pub const UBLOCK_ARABIC_PRESENTATION_FORMS_B: UBlockCode = UBlockCode(85i32);
pub const UBLOCK_SPECIALS: UBlockCode = UBlockCode(86i32);
pub const UBLOCK_HALFWIDTH_AND_FULLWIDTH_FORMS: UBlockCode = UBlockCode(87i32);
pub const UBLOCK_OLD_ITALIC: UBlockCode = UBlockCode(88i32);
pub const UBLOCK_GOTHIC: UBlockCode = UBlockCode(89i32);
pub const UBLOCK_DESERET: UBlockCode = UBlockCode(90i32);
pub const UBLOCK_BYZANTINE_MUSICAL_SYMBOLS: UBlockCode = UBlockCode(91i32);
pub const UBLOCK_MUSICAL_SYMBOLS: UBlockCode = UBlockCode(92i32);
pub const UBLOCK_MATHEMATICAL_ALPHANUMERIC_SYMBOLS: UBlockCode = UBlockCode(93i32);
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_B: UBlockCode = UBlockCode(94i32);
pub const UBLOCK_CJK_COMPATIBILITY_IDEOGRAPHS_SUPPLEMENT: UBlockCode = UBlockCode(95i32);
pub const UBLOCK_TAGS: UBlockCode = UBlockCode(96i32);
pub const UBLOCK_CYRILLIC_SUPPLEMENT: UBlockCode = UBlockCode(97i32);
pub const UBLOCK_CYRILLIC_SUPPLEMENTARY: UBlockCode = UBlockCode(97i32);
pub const UBLOCK_TAGALOG: UBlockCode = UBlockCode(98i32);
pub const UBLOCK_HANUNOO: UBlockCode = UBlockCode(99i32);
pub const UBLOCK_BUHID: UBlockCode = UBlockCode(100i32);
pub const UBLOCK_TAGBANWA: UBlockCode = UBlockCode(101i32);
pub const UBLOCK_MISCELLANEOUS_MATHEMATICAL_SYMBOLS_A: UBlockCode = UBlockCode(102i32);
pub const UBLOCK_SUPPLEMENTAL_ARROWS_A: UBlockCode = UBlockCode(103i32);
pub const UBLOCK_SUPPLEMENTAL_ARROWS_B: UBlockCode = UBlockCode(104i32);
pub const UBLOCK_MISCELLANEOUS_MATHEMATICAL_SYMBOLS_B: UBlockCode = UBlockCode(105i32);
pub const UBLOCK_SUPPLEMENTAL_MATHEMATICAL_OPERATORS: UBlockCode = UBlockCode(106i32);
pub const UBLOCK_KATAKANA_PHONETIC_EXTENSIONS: UBlockCode = UBlockCode(107i32);
pub const UBLOCK_VARIATION_SELECTORS: UBlockCode = UBlockCode(108i32);
pub const UBLOCK_SUPPLEMENTARY_PRIVATE_USE_AREA_A: UBlockCode = UBlockCode(109i32);
pub const UBLOCK_SUPPLEMENTARY_PRIVATE_USE_AREA_B: UBlockCode = UBlockCode(110i32);
pub const UBLOCK_LIMBU: UBlockCode = UBlockCode(111i32);
pub const UBLOCK_TAI_LE: UBlockCode = UBlockCode(112i32);
pub const UBLOCK_KHMER_SYMBOLS: UBlockCode = UBlockCode(113i32);
pub const UBLOCK_PHONETIC_EXTENSIONS: UBlockCode = UBlockCode(114i32);
pub const UBLOCK_MISCELLANEOUS_SYMBOLS_AND_ARROWS: UBlockCode = UBlockCode(115i32);
pub const UBLOCK_YIJING_HEXAGRAM_SYMBOLS: UBlockCode = UBlockCode(116i32);
pub const UBLOCK_LINEAR_B_SYLLABARY: UBlockCode = UBlockCode(117i32);
pub const UBLOCK_LINEAR_B_IDEOGRAMS: UBlockCode = UBlockCode(118i32);
pub const UBLOCK_AEGEAN_NUMBERS: UBlockCode = UBlockCode(119i32);
pub const UBLOCK_UGARITIC: UBlockCode = UBlockCode(120i32);
pub const UBLOCK_SHAVIAN: UBlockCode = UBlockCode(121i32);
pub const UBLOCK_OSMANYA: UBlockCode = UBlockCode(122i32);
pub const UBLOCK_CYPRIOT_SYLLABARY: UBlockCode = UBlockCode(123i32);
pub const UBLOCK_TAI_XUAN_JING_SYMBOLS: UBlockCode = UBlockCode(124i32);
pub const UBLOCK_VARIATION_SELECTORS_SUPPLEMENT: UBlockCode = UBlockCode(125i32);
pub const UBLOCK_ANCIENT_GREEK_MUSICAL_NOTATION: UBlockCode = UBlockCode(126i32);
pub const UBLOCK_ANCIENT_GREEK_NUMBERS: UBlockCode = UBlockCode(127i32);
pub const UBLOCK_ARABIC_SUPPLEMENT: UBlockCode = UBlockCode(128i32);
pub const UBLOCK_BUGINESE: UBlockCode = UBlockCode(129i32);
pub const UBLOCK_CJK_STROKES: UBlockCode = UBlockCode(130i32);
pub const UBLOCK_COMBINING_DIACRITICAL_MARKS_SUPPLEMENT: UBlockCode = UBlockCode(131i32);
pub const UBLOCK_COPTIC: UBlockCode = UBlockCode(132i32);
pub const UBLOCK_ETHIOPIC_EXTENDED: UBlockCode = UBlockCode(133i32);
pub const UBLOCK_ETHIOPIC_SUPPLEMENT: UBlockCode = UBlockCode(134i32);
pub const UBLOCK_GEORGIAN_SUPPLEMENT: UBlockCode = UBlockCode(135i32);
pub const UBLOCK_GLAGOLITIC: UBlockCode = UBlockCode(136i32);
pub const UBLOCK_KHAROSHTHI: UBlockCode = UBlockCode(137i32);
pub const UBLOCK_MODIFIER_TONE_LETTERS: UBlockCode = UBlockCode(138i32);
pub const UBLOCK_NEW_TAI_LUE: UBlockCode = UBlockCode(139i32);
pub const UBLOCK_OLD_PERSIAN: UBlockCode = UBlockCode(140i32);
pub const UBLOCK_PHONETIC_EXTENSIONS_SUPPLEMENT: UBlockCode = UBlockCode(141i32);
pub const UBLOCK_SUPPLEMENTAL_PUNCTUATION: UBlockCode = UBlockCode(142i32);
pub const UBLOCK_SYLOTI_NAGRI: UBlockCode = UBlockCode(143i32);
pub const UBLOCK_TIFINAGH: UBlockCode = UBlockCode(144i32);
pub const UBLOCK_VERTICAL_FORMS: UBlockCode = UBlockCode(145i32);
pub const UBLOCK_NKO: UBlockCode = UBlockCode(146i32);
pub const UBLOCK_BALINESE: UBlockCode = UBlockCode(147i32);
pub const UBLOCK_LATIN_EXTENDED_C: UBlockCode = UBlockCode(148i32);
pub const UBLOCK_LATIN_EXTENDED_D: UBlockCode = UBlockCode(149i32);
pub const UBLOCK_PHAGS_PA: UBlockCode = UBlockCode(150i32);
pub const UBLOCK_PHOENICIAN: UBlockCode = UBlockCode(151i32);
pub const UBLOCK_CUNEIFORM: UBlockCode = UBlockCode(152i32);
pub const UBLOCK_CUNEIFORM_NUMBERS_AND_PUNCTUATION: UBlockCode = UBlockCode(153i32);
pub const UBLOCK_COUNTING_ROD_NUMERALS: UBlockCode = UBlockCode(154i32);
pub const UBLOCK_SUNDANESE: UBlockCode = UBlockCode(155i32);
pub const UBLOCK_LEPCHA: UBlockCode = UBlockCode(156i32);
pub const UBLOCK_OL_CHIKI: UBlockCode = UBlockCode(157i32);
pub const UBLOCK_CYRILLIC_EXTENDED_A: UBlockCode = UBlockCode(158i32);
pub const UBLOCK_VAI: UBlockCode = UBlockCode(159i32);
pub const UBLOCK_CYRILLIC_EXTENDED_B: UBlockCode = UBlockCode(160i32);
pub const UBLOCK_SAURASHTRA: UBlockCode = UBlockCode(161i32);
pub const UBLOCK_KAYAH_LI: UBlockCode = UBlockCode(162i32);
pub const UBLOCK_REJANG: UBlockCode = UBlockCode(163i32);
pub const UBLOCK_CHAM: UBlockCode = UBlockCode(164i32);
pub const UBLOCK_ANCIENT_SYMBOLS: UBlockCode = UBlockCode(165i32);
pub const UBLOCK_PHAISTOS_DISC: UBlockCode = UBlockCode(166i32);
pub const UBLOCK_LYCIAN: UBlockCode = UBlockCode(167i32);
pub const UBLOCK_CARIAN: UBlockCode = UBlockCode(168i32);
pub const UBLOCK_LYDIAN: UBlockCode = UBlockCode(169i32);
pub const UBLOCK_MAHJONG_TILES: UBlockCode = UBlockCode(170i32);
pub const UBLOCK_DOMINO_TILES: UBlockCode = UBlockCode(171i32);
pub const UBLOCK_SAMARITAN: UBlockCode = UBlockCode(172i32);
pub const UBLOCK_UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED: UBlockCode = UBlockCode(173i32);
pub const UBLOCK_TAI_THAM: UBlockCode = UBlockCode(174i32);
pub const UBLOCK_VEDIC_EXTENSIONS: UBlockCode = UBlockCode(175i32);
pub const UBLOCK_LISU: UBlockCode = UBlockCode(176i32);
pub const UBLOCK_BAMUM: UBlockCode = UBlockCode(177i32);
pub const UBLOCK_COMMON_INDIC_NUMBER_FORMS: UBlockCode = UBlockCode(178i32);
pub const UBLOCK_DEVANAGARI_EXTENDED: UBlockCode = UBlockCode(179i32);
pub const UBLOCK_HANGUL_JAMO_EXTENDED_A: UBlockCode = UBlockCode(180i32);
pub const UBLOCK_JAVANESE: UBlockCode = UBlockCode(181i32);
pub const UBLOCK_MYANMAR_EXTENDED_A: UBlockCode = UBlockCode(182i32);
pub const UBLOCK_TAI_VIET: UBlockCode = UBlockCode(183i32);
pub const UBLOCK_MEETEI_MAYEK: UBlockCode = UBlockCode(184i32);
pub const UBLOCK_HANGUL_JAMO_EXTENDED_B: UBlockCode = UBlockCode(185i32);
pub const UBLOCK_IMPERIAL_ARAMAIC: UBlockCode = UBlockCode(186i32);
pub const UBLOCK_OLD_SOUTH_ARABIAN: UBlockCode = UBlockCode(187i32);
pub const UBLOCK_AVESTAN: UBlockCode = UBlockCode(188i32);
pub const UBLOCK_INSCRIPTIONAL_PARTHIAN: UBlockCode = UBlockCode(189i32);
pub const UBLOCK_INSCRIPTIONAL_PAHLAVI: UBlockCode = UBlockCode(190i32);
pub const UBLOCK_OLD_TURKIC: UBlockCode = UBlockCode(191i32);
pub const UBLOCK_RUMI_NUMERAL_SYMBOLS: UBlockCode = UBlockCode(192i32);
pub const UBLOCK_KAITHI: UBlockCode = UBlockCode(193i32);
pub const UBLOCK_EGYPTIAN_HIEROGLYPHS: UBlockCode = UBlockCode(194i32);
pub const UBLOCK_ENCLOSED_ALPHANUMERIC_SUPPLEMENT: UBlockCode = UBlockCode(195i32);
pub const UBLOCK_ENCLOSED_IDEOGRAPHIC_SUPPLEMENT: UBlockCode = UBlockCode(196i32);
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_C: UBlockCode = UBlockCode(197i32);
pub const UBLOCK_MANDAIC: UBlockCode = UBlockCode(198i32);
pub const UBLOCK_BATAK: UBlockCode = UBlockCode(199i32);
pub const UBLOCK_ETHIOPIC_EXTENDED_A: UBlockCode = UBlockCode(200i32);
pub const UBLOCK_BRAHMI: UBlockCode = UBlockCode(201i32);
pub const UBLOCK_BAMUM_SUPPLEMENT: UBlockCode = UBlockCode(202i32);
pub const UBLOCK_KANA_SUPPLEMENT: UBlockCode = UBlockCode(203i32);
pub const UBLOCK_PLAYING_CARDS: UBlockCode = UBlockCode(204i32);
pub const UBLOCK_MISCELLANEOUS_SYMBOLS_AND_PICTOGRAPHS: UBlockCode = UBlockCode(205i32);
pub const UBLOCK_EMOTICONS: UBlockCode = UBlockCode(206i32);
pub const UBLOCK_TRANSPORT_AND_MAP_SYMBOLS: UBlockCode = UBlockCode(207i32);
pub const UBLOCK_ALCHEMICAL_SYMBOLS: UBlockCode = UBlockCode(208i32);
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_D: UBlockCode = UBlockCode(209i32);
pub const UBLOCK_ARABIC_EXTENDED_A: UBlockCode = UBlockCode(210i32);
pub const UBLOCK_ARABIC_MATHEMATICAL_ALPHABETIC_SYMBOLS: UBlockCode = UBlockCode(211i32);
pub const UBLOCK_CHAKMA: UBlockCode = UBlockCode(212i32);
pub const UBLOCK_MEETEI_MAYEK_EXTENSIONS: UBlockCode = UBlockCode(213i32);
pub const UBLOCK_MEROITIC_CURSIVE: UBlockCode = UBlockCode(214i32);
pub const UBLOCK_MEROITIC_HIEROGLYPHS: UBlockCode = UBlockCode(215i32);
pub const UBLOCK_MIAO: UBlockCode = UBlockCode(216i32);
pub const UBLOCK_SHARADA: UBlockCode = UBlockCode(217i32);
pub const UBLOCK_SORA_SOMPENG: UBlockCode = UBlockCode(218i32);
pub const UBLOCK_SUNDANESE_SUPPLEMENT: UBlockCode = UBlockCode(219i32);
pub const UBLOCK_TAKRI: UBlockCode = UBlockCode(220i32);
pub const UBLOCK_BASSA_VAH: UBlockCode = UBlockCode(221i32);
pub const UBLOCK_CAUCASIAN_ALBANIAN: UBlockCode = UBlockCode(222i32);
pub const UBLOCK_COPTIC_EPACT_NUMBERS: UBlockCode = UBlockCode(223i32);
pub const UBLOCK_COMBINING_DIACRITICAL_MARKS_EXTENDED: UBlockCode = UBlockCode(224i32);
pub const UBLOCK_DUPLOYAN: UBlockCode = UBlockCode(225i32);
pub const UBLOCK_ELBASAN: UBlockCode = UBlockCode(226i32);
pub const UBLOCK_GEOMETRIC_SHAPES_EXTENDED: UBlockCode = UBlockCode(227i32);
pub const UBLOCK_GRANTHA: UBlockCode = UBlockCode(228i32);
pub const UBLOCK_KHOJKI: UBlockCode = UBlockCode(229i32);
pub const UBLOCK_KHUDAWADI: UBlockCode = UBlockCode(230i32);
pub const UBLOCK_LATIN_EXTENDED_E: UBlockCode = UBlockCode(231i32);
pub const UBLOCK_LINEAR_A: UBlockCode = UBlockCode(232i32);
pub const UBLOCK_MAHAJANI: UBlockCode = UBlockCode(233i32);
pub const UBLOCK_MANICHAEAN: UBlockCode = UBlockCode(234i32);
pub const UBLOCK_MENDE_KIKAKUI: UBlockCode = UBlockCode(235i32);
pub const UBLOCK_MODI: UBlockCode = UBlockCode(236i32);
pub const UBLOCK_MRO: UBlockCode = UBlockCode(237i32);
pub const UBLOCK_MYANMAR_EXTENDED_B: UBlockCode = UBlockCode(238i32);
pub const UBLOCK_NABATAEAN: UBlockCode = UBlockCode(239i32);
pub const UBLOCK_OLD_NORTH_ARABIAN: UBlockCode = UBlockCode(240i32);
pub const UBLOCK_OLD_PERMIC: UBlockCode = UBlockCode(241i32);
pub const UBLOCK_ORNAMENTAL_DINGBATS: UBlockCode = UBlockCode(242i32);
pub const UBLOCK_PAHAWH_HMONG: UBlockCode = UBlockCode(243i32);
pub const UBLOCK_PALMYRENE: UBlockCode = UBlockCode(244i32);
pub const UBLOCK_PAU_CIN_HAU: UBlockCode = UBlockCode(245i32);
pub const UBLOCK_PSALTER_PAHLAVI: UBlockCode = UBlockCode(246i32);
pub const UBLOCK_SHORTHAND_FORMAT_CONTROLS: UBlockCode = UBlockCode(247i32);
pub const UBLOCK_SIDDHAM: UBlockCode = UBlockCode(248i32);
pub const UBLOCK_SINHALA_ARCHAIC_NUMBERS: UBlockCode = UBlockCode(249i32);
pub const UBLOCK_SUPPLEMENTAL_ARROWS_C: UBlockCode = UBlockCode(250i32);
pub const UBLOCK_TIRHUTA: UBlockCode = UBlockCode(251i32);
pub const UBLOCK_WARANG_CITI: UBlockCode = UBlockCode(252i32);
pub const UBLOCK_AHOM: UBlockCode = UBlockCode(253i32);
pub const UBLOCK_ANATOLIAN_HIEROGLYPHS: UBlockCode = UBlockCode(254i32);
pub const UBLOCK_CHEROKEE_SUPPLEMENT: UBlockCode = UBlockCode(255i32);
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_E: UBlockCode = UBlockCode(256i32);
pub const UBLOCK_EARLY_DYNASTIC_CUNEIFORM: UBlockCode = UBlockCode(257i32);
pub const UBLOCK_HATRAN: UBlockCode = UBlockCode(258i32);
pub const UBLOCK_MULTANI: UBlockCode = UBlockCode(259i32);
pub const UBLOCK_OLD_HUNGARIAN: UBlockCode = UBlockCode(260i32);
pub const UBLOCK_SUPPLEMENTAL_SYMBOLS_AND_PICTOGRAPHS: UBlockCode = UBlockCode(261i32);
pub const UBLOCK_SUTTON_SIGNWRITING: UBlockCode = UBlockCode(262i32);
pub const UBLOCK_ADLAM: UBlockCode = UBlockCode(263i32);
pub const UBLOCK_BHAIKSUKI: UBlockCode = UBlockCode(264i32);
pub const UBLOCK_CYRILLIC_EXTENDED_C: UBlockCode = UBlockCode(265i32);
pub const UBLOCK_GLAGOLITIC_SUPPLEMENT: UBlockCode = UBlockCode(266i32);
pub const UBLOCK_IDEOGRAPHIC_SYMBOLS_AND_PUNCTUATION: UBlockCode = UBlockCode(267i32);
pub const UBLOCK_MARCHEN: UBlockCode = UBlockCode(268i32);
pub const UBLOCK_MONGOLIAN_SUPPLEMENT: UBlockCode = UBlockCode(269i32);
pub const UBLOCK_NEWA: UBlockCode = UBlockCode(270i32);
pub const UBLOCK_OSAGE: UBlockCode = UBlockCode(271i32);
pub const UBLOCK_TANGUT: UBlockCode = UBlockCode(272i32);
pub const UBLOCK_TANGUT_COMPONENTS: UBlockCode = UBlockCode(273i32);
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_F: UBlockCode = UBlockCode(274i32);
pub const UBLOCK_KANA_EXTENDED_A: UBlockCode = UBlockCode(275i32);
pub const UBLOCK_MASARAM_GONDI: UBlockCode = UBlockCode(276i32);
pub const UBLOCK_NUSHU: UBlockCode = UBlockCode(277i32);
pub const UBLOCK_SOYOMBO: UBlockCode = UBlockCode(278i32);
pub const UBLOCK_SYRIAC_SUPPLEMENT: UBlockCode = UBlockCode(279i32);
pub const UBLOCK_ZANABAZAR_SQUARE: UBlockCode = UBlockCode(280i32);
pub const UBLOCK_CHESS_SYMBOLS: UBlockCode = UBlockCode(281i32);
pub const UBLOCK_DOGRA: UBlockCode = UBlockCode(282i32);
pub const UBLOCK_GEORGIAN_EXTENDED: UBlockCode = UBlockCode(283i32);
pub const UBLOCK_GUNJALA_GONDI: UBlockCode = UBlockCode(284i32);
pub const UBLOCK_HANIFI_ROHINGYA: UBlockCode = UBlockCode(285i32);
pub const UBLOCK_INDIC_SIYAQ_NUMBERS: UBlockCode = UBlockCode(286i32);
pub const UBLOCK_MAKASAR: UBlockCode = UBlockCode(287i32);
pub const UBLOCK_MAYAN_NUMERALS: UBlockCode = UBlockCode(288i32);
pub const UBLOCK_MEDEFAIDRIN: UBlockCode = UBlockCode(289i32);
pub const UBLOCK_OLD_SOGDIAN: UBlockCode = UBlockCode(290i32);
pub const UBLOCK_SOGDIAN: UBlockCode = UBlockCode(291i32);
pub const UBLOCK_EGYPTIAN_HIEROGLYPH_FORMAT_CONTROLS: UBlockCode = UBlockCode(292i32);
pub const UBLOCK_ELYMAIC: UBlockCode = UBlockCode(293i32);
pub const UBLOCK_NANDINAGARI: UBlockCode = UBlockCode(294i32);
pub const UBLOCK_NYIAKENG_PUACHUE_HMONG: UBlockCode = UBlockCode(295i32);
pub const UBLOCK_OTTOMAN_SIYAQ_NUMBERS: UBlockCode = UBlockCode(296i32);
pub const UBLOCK_SMALL_KANA_EXTENSION: UBlockCode = UBlockCode(297i32);
pub const UBLOCK_SYMBOLS_AND_PICTOGRAPHS_EXTENDED_A: UBlockCode = UBlockCode(298i32);
pub const UBLOCK_TAMIL_SUPPLEMENT: UBlockCode = UBlockCode(299i32);
pub const UBLOCK_WANCHO: UBlockCode = UBlockCode(300i32);
pub const UBLOCK_CHORASMIAN: UBlockCode = UBlockCode(301i32);
pub const UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_G: UBlockCode = UBlockCode(302i32);
pub const UBLOCK_DIVES_AKURU: UBlockCode = UBlockCode(303i32);
pub const UBLOCK_KHITAN_SMALL_SCRIPT: UBlockCode = UBlockCode(304i32);
pub const UBLOCK_LISU_SUPPLEMENT: UBlockCode = UBlockCode(305i32);
pub const UBLOCK_SYMBOLS_FOR_LEGACY_COMPUTING: UBlockCode = UBlockCode(306i32);
pub const UBLOCK_TANGUT_SUPPLEMENT: UBlockCode = UBlockCode(307i32);
pub const UBLOCK_YEZIDI: UBlockCode = UBlockCode(308i32);
pub const UBLOCK_INVALID_CODE: UBlockCode = UBlockCode(-1i32);
impl ::core::marker::Copy for UBlockCode {}
impl ::core::clone::Clone for UBlockCode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UBlockCode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UBlockCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UBlockCode").field(&self.0).finish()
    }
}
impl FromIntoMemory for UBlockCode {
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
pub struct UBreakIterator(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UBreakIteratorType(pub i32);
pub const UBRK_CHARACTER: UBreakIteratorType = UBreakIteratorType(0i32);
pub const UBRK_WORD: UBreakIteratorType = UBreakIteratorType(1i32);
pub const UBRK_LINE: UBreakIteratorType = UBreakIteratorType(2i32);
pub const UBRK_SENTENCE: UBreakIteratorType = UBreakIteratorType(3i32);
impl ::core::marker::Copy for UBreakIteratorType {}
impl ::core::clone::Clone for UBreakIteratorType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UBreakIteratorType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UBreakIteratorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UBreakIteratorType").field(&self.0).finish()
    }
}
impl FromIntoMemory for UBreakIteratorType {
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
pub const UCAL_UNKNOWN_ZONE_ID: &'static str = "Etc/Unknown";
pub const UCHAR_MAX_VALUE: u32 = 1114111u32;
pub const UCHAR_MIN_VALUE: u32 = 0u32;
pub const UCLN_NO_AUTO_CLEANUP: u32 = 1u32;
pub const UCNV_ESCAPE_C: &'static str = "C";
pub const UCNV_ESCAPE_CSS2: &'static str = "S";
pub const UCNV_ESCAPE_JAVA: &'static str = "J";
pub const UCNV_ESCAPE_UNICODE: &'static str = "U";
pub const UCNV_ESCAPE_XML_DEC: &'static str = "D";
pub const UCNV_ESCAPE_XML_HEX: &'static str = "X";
pub const UCNV_LOCALE_OPTION_STRING: &'static str = ",locale=";
pub const UCNV_MAX_CONVERTER_NAME_LENGTH: u32 = 60u32;
pub const UCNV_OPTION_SEP_STRING: &'static str = ",";
pub const UCNV_SI: u32 = 15u32;
pub const UCNV_SKIP_STOP_ON_ILLEGAL: &'static str = "i";
pub const UCNV_SO: u32 = 14u32;
pub const UCNV_SUB_STOP_ON_ILLEGAL: &'static str = "i";
pub const UCNV_SWAP_LFNL_OPTION_STRING: &'static str = ",swaplfnl";
pub const UCNV_VALUE_SEP_STRING: &'static str = "=";
pub const UCNV_VERSION_OPTION_STRING: &'static str = ",version=";
pub const UCONFIG_ENABLE_PLUGINS: u32 = 0u32;
pub const UCONFIG_FORMAT_FASTPATHS_49: u32 = 1u32;
pub const UCONFIG_HAVE_PARSEALLINPUT: u32 = 1u32;
pub const UCONFIG_NO_BREAK_ITERATION: u32 = 1u32;
pub const UCONFIG_NO_COLLATION: u32 = 1u32;
pub const UCONFIG_NO_CONVERSION: u32 = 0u32;
pub const UCONFIG_NO_FILE_IO: u32 = 0u32;
pub const UCONFIG_NO_FILTERED_BREAK_ITERATION: u32 = 0u32;
pub const UCONFIG_NO_FORMATTING: u32 = 1u32;
pub const UCONFIG_NO_IDNA: u32 = 1u32;
pub const UCONFIG_NO_LEGACY_CONVERSION: u32 = 1u32;
pub const UCONFIG_NO_NORMALIZATION: u32 = 0u32;
pub const UCONFIG_NO_REGULAR_EXPRESSIONS: u32 = 1u32;
pub const UCONFIG_NO_SERVICE: u32 = 0u32;
pub const UCONFIG_NO_TRANSLITERATION: u32 = 1u32;
pub const UCONFIG_ONLY_COLLATION: u32 = 0u32;
pub const UCONFIG_ONLY_HTML_CONVERSION: u32 = 0u32;
pub struct UCPMap(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UCPMapRangeOption(pub i32);
pub const UCPMAP_RANGE_NORMAL: UCPMapRangeOption = UCPMapRangeOption(0i32);
pub const UCPMAP_RANGE_FIXED_LEAD_SURROGATES: UCPMapRangeOption = UCPMapRangeOption(1i32);
pub const UCPMAP_RANGE_FIXED_ALL_SURROGATES: UCPMapRangeOption = UCPMapRangeOption(2i32);
impl ::core::marker::Copy for UCPMapRangeOption {}
impl ::core::clone::Clone for UCPMapRangeOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UCPMapRangeOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCPMapRangeOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCPMapRangeOption").field(&self.0).finish()
    }
}
impl FromIntoMemory for UCPMapRangeOption {
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
pub type UCPMapValueFilter = StdCallFnPtr<(ConstPtr<::core::ffi::c_void>, u32), u32>;
pub const UCPTRIE_ERROR_VALUE_NEG_DATA_OFFSET: i32 = 1i32;
pub const UCPTRIE_FAST_DATA_BLOCK_LENGTH: i32 = 64i32;
pub const UCPTRIE_FAST_DATA_MASK: i32 = 63i32;
pub const UCPTRIE_FAST_SHIFT: i32 = 6i32;
pub const UCPTRIE_HIGH_VALUE_NEG_DATA_OFFSET: i32 = 2i32;
pub const UCPTRIE_SMALL_MAX: i32 = 4095i32;
pub struct UCPTrie {
    pub index: ConstPtr<u16>,
    pub data: UCPTrieData,
    pub indexLength: i32,
    pub dataLength: i32,
    pub highStart: i32,
    pub shifted12HighStart: u16,
    pub r#type: i8,
    pub valueWidth: i8,
    pub reserved32: u32,
    pub reserved16: u16,
    pub index3NullOffset: u16,
    pub dataNullOffset: i32,
    pub nullValue: u32,
}
impl ::core::marker::Copy for UCPTrie {}
impl ::core::clone::Clone for UCPTrie {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for UCPTrie {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
            && self.data == other.data
            && self.indexLength == other.indexLength
            && self.dataLength == other.dataLength
            && self.highStart == other.highStart
            && self.shifted12HighStart == other.shifted12HighStart
            && self.r#type == other.r#type
            && self.valueWidth == other.valueWidth
            && self.reserved32 == other.reserved32
            && self.reserved16 == other.reserved16
            && self.index3NullOffset == other.index3NullOffset
            && self.dataNullOffset == other.dataNullOffset
            && self.nullValue == other.nullValue
    }
}
impl ::core::cmp::Eq for UCPTrie {}
impl FromIntoMemory for UCPTrie {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 52u32 as usize);
        let f_index = <ConstPtr<u16> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_data = <UCPTrieData as FromIntoMemory>::from_bytes(&from[4..4 + 16]);
        let f_indexLength = <i32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dataLength = <i32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_highStart = <i32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_shifted12HighStart = <u16 as FromIntoMemory>::from_bytes(&from[32..32 + 2]);
        let f_type = <i8 as FromIntoMemory>::from_bytes(&from[34..34 + 1]);
        let f_valueWidth = <i8 as FromIntoMemory>::from_bytes(&from[35..35 + 1]);
        let f_reserved32 = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_reserved16 = <u16 as FromIntoMemory>::from_bytes(&from[40..40 + 2]);
        let f_index3NullOffset = <u16 as FromIntoMemory>::from_bytes(&from[42..42 + 2]);
        let f_dataNullOffset = <i32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_nullValue = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        Self {
            index: f_index,
            data: f_data,
            indexLength: f_indexLength,
            dataLength: f_dataLength,
            highStart: f_highStart,
            shifted12HighStart: f_shifted12HighStart,
            r#type: f_type,
            valueWidth: f_valueWidth,
            reserved32: f_reserved32,
            reserved16: f_reserved16,
            index3NullOffset: f_index3NullOffset,
            dataNullOffset: f_dataNullOffset,
            nullValue: f_nullValue,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 52u32 as usize);
        FromIntoMemory::into_bytes(self.index, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.data, &mut into[4..4 + 16]);
        FromIntoMemory::into_bytes(self.indexLength, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dataLength, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.highStart, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.shifted12HighStart, &mut into[32..32 + 2]);
        FromIntoMemory::into_bytes(self.r#type, &mut into[34..34 + 1]);
        FromIntoMemory::into_bytes(self.valueWidth, &mut into[35..35 + 1]);
        FromIntoMemory::into_bytes(self.reserved32, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.reserved16, &mut into[40..40 + 2]);
        FromIntoMemory::into_bytes(self.index3NullOffset, &mut into[42..42 + 2]);
        FromIntoMemory::into_bytes(self.dataNullOffset, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.nullValue, &mut into[48..48 + 4]);
    }
    fn size() -> usize {
        52u32 as usize
    }
}
pub struct UCPTrieData {
    pub ptr0: ConstPtr<::core::ffi::c_void>,
    pub ptr16: ConstPtr<u16>,
    pub ptr32: ConstPtr<u32>,
    pub ptr8: ConstPtr<u8>,
}
impl ::core::marker::Copy for UCPTrieData {}
impl ::core::clone::Clone for UCPTrieData {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for UCPTrieData {
    fn eq(&self, other: &Self) -> bool {
        self.ptr0 == other.ptr0
            && self.ptr16 == other.ptr16
            && self.ptr32 == other.ptr32
            && self.ptr8 == other.ptr8
    }
}
impl ::core::cmp::Eq for UCPTrieData {}
impl FromIntoMemory for UCPTrieData {
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
pub struct UCPTrieType(pub i32);
pub const UCPTRIE_TYPE_ANY: UCPTrieType = UCPTrieType(-1i32);
pub const UCPTRIE_TYPE_FAST: UCPTrieType = UCPTrieType(0i32);
pub const UCPTRIE_TYPE_SMALL: UCPTrieType = UCPTrieType(1i32);
impl ::core::marker::Copy for UCPTrieType {}
impl ::core::clone::Clone for UCPTrieType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UCPTrieType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCPTrieType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCPTrieType").field(&self.0).finish()
    }
}
impl FromIntoMemory for UCPTrieType {
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
pub struct UCPTrieValueWidth(pub i32);
pub const UCPTRIE_VALUE_BITS_ANY: UCPTrieValueWidth = UCPTrieValueWidth(-1i32);
pub const UCPTRIE_VALUE_BITS_16: UCPTrieValueWidth = UCPTrieValueWidth(0i32);
pub const UCPTRIE_VALUE_BITS_32: UCPTrieValueWidth = UCPTrieValueWidth(1i32);
pub const UCPTRIE_VALUE_BITS_8: UCPTrieValueWidth = UCPTrieValueWidth(2i32);
impl ::core::marker::Copy for UCPTrieValueWidth {}
impl ::core::clone::Clone for UCPTrieValueWidth {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UCPTrieValueWidth {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCPTrieValueWidth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCPTrieValueWidth").field(&self.0).finish()
    }
}
impl FromIntoMemory for UCPTrieValueWidth {
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
pub struct UCalendarAMPMs(pub i32);
pub const UCAL_AM: UCalendarAMPMs = UCalendarAMPMs(0i32);
pub const UCAL_PM: UCalendarAMPMs = UCalendarAMPMs(1i32);
impl ::core::marker::Copy for UCalendarAMPMs {}
impl ::core::clone::Clone for UCalendarAMPMs {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UCalendarAMPMs {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCalendarAMPMs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCalendarAMPMs").field(&self.0).finish()
    }
}
impl FromIntoMemory for UCalendarAMPMs {
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
pub struct UCalendarAttribute(pub i32);
pub const UCAL_LENIENT: UCalendarAttribute = UCalendarAttribute(0i32);
pub const UCAL_FIRST_DAY_OF_WEEK: UCalendarAttribute = UCalendarAttribute(1i32);
pub const UCAL_MINIMAL_DAYS_IN_FIRST_WEEK: UCalendarAttribute = UCalendarAttribute(2i32);
pub const UCAL_REPEATED_WALL_TIME: UCalendarAttribute = UCalendarAttribute(3i32);
pub const UCAL_SKIPPED_WALL_TIME: UCalendarAttribute = UCalendarAttribute(4i32);
impl ::core::marker::Copy for UCalendarAttribute {}
impl ::core::clone::Clone for UCalendarAttribute {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UCalendarAttribute {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCalendarAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCalendarAttribute").field(&self.0).finish()
    }
}
impl FromIntoMemory for UCalendarAttribute {
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
pub struct UCalendarDateFields(pub i32);
pub const UCAL_ERA: UCalendarDateFields = UCalendarDateFields(0i32);
pub const UCAL_YEAR: UCalendarDateFields = UCalendarDateFields(1i32);
pub const UCAL_MONTH: UCalendarDateFields = UCalendarDateFields(2i32);
pub const UCAL_WEEK_OF_YEAR: UCalendarDateFields = UCalendarDateFields(3i32);
pub const UCAL_WEEK_OF_MONTH: UCalendarDateFields = UCalendarDateFields(4i32);
pub const UCAL_DATE: UCalendarDateFields = UCalendarDateFields(5i32);
pub const UCAL_DAY_OF_YEAR: UCalendarDateFields = UCalendarDateFields(6i32);
pub const UCAL_DAY_OF_WEEK: UCalendarDateFields = UCalendarDateFields(7i32);
pub const UCAL_DAY_OF_WEEK_IN_MONTH: UCalendarDateFields = UCalendarDateFields(8i32);
pub const UCAL_AM_PM: UCalendarDateFields = UCalendarDateFields(9i32);
pub const UCAL_HOUR: UCalendarDateFields = UCalendarDateFields(10i32);
pub const UCAL_HOUR_OF_DAY: UCalendarDateFields = UCalendarDateFields(11i32);
pub const UCAL_MINUTE: UCalendarDateFields = UCalendarDateFields(12i32);
pub const UCAL_SECOND: UCalendarDateFields = UCalendarDateFields(13i32);
pub const UCAL_MILLISECOND: UCalendarDateFields = UCalendarDateFields(14i32);
pub const UCAL_ZONE_OFFSET: UCalendarDateFields = UCalendarDateFields(15i32);
pub const UCAL_DST_OFFSET: UCalendarDateFields = UCalendarDateFields(16i32);
pub const UCAL_YEAR_WOY: UCalendarDateFields = UCalendarDateFields(17i32);
pub const UCAL_DOW_LOCAL: UCalendarDateFields = UCalendarDateFields(18i32);
pub const UCAL_EXTENDED_YEAR: UCalendarDateFields = UCalendarDateFields(19i32);
pub const UCAL_JULIAN_DAY: UCalendarDateFields = UCalendarDateFields(20i32);
pub const UCAL_MILLISECONDS_IN_DAY: UCalendarDateFields = UCalendarDateFields(21i32);
pub const UCAL_IS_LEAP_MONTH: UCalendarDateFields = UCalendarDateFields(22i32);
pub const UCAL_FIELD_COUNT: UCalendarDateFields = UCalendarDateFields(23i32);
pub const UCAL_DAY_OF_MONTH: UCalendarDateFields = UCalendarDateFields(5i32);
impl ::core::marker::Copy for UCalendarDateFields {}
impl ::core::clone::Clone for UCalendarDateFields {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UCalendarDateFields {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCalendarDateFields {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCalendarDateFields").field(&self.0).finish()
    }
}
impl FromIntoMemory for UCalendarDateFields {
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
pub struct UCalendarDaysOfWeek(pub i32);
pub const UCAL_SUNDAY: UCalendarDaysOfWeek = UCalendarDaysOfWeek(1i32);
pub const UCAL_MONDAY: UCalendarDaysOfWeek = UCalendarDaysOfWeek(2i32);
pub const UCAL_TUESDAY: UCalendarDaysOfWeek = UCalendarDaysOfWeek(3i32);
pub const UCAL_WEDNESDAY: UCalendarDaysOfWeek = UCalendarDaysOfWeek(4i32);
pub const UCAL_THURSDAY: UCalendarDaysOfWeek = UCalendarDaysOfWeek(5i32);
pub const UCAL_FRIDAY: UCalendarDaysOfWeek = UCalendarDaysOfWeek(6i32);
pub const UCAL_SATURDAY: UCalendarDaysOfWeek = UCalendarDaysOfWeek(7i32);
impl ::core::marker::Copy for UCalendarDaysOfWeek {}
impl ::core::clone::Clone for UCalendarDaysOfWeek {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UCalendarDaysOfWeek {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCalendarDaysOfWeek {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCalendarDaysOfWeek").field(&self.0).finish()
    }
}
impl FromIntoMemory for UCalendarDaysOfWeek {
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
pub struct UCalendarDisplayNameType(pub i32);
pub const UCAL_STANDARD: UCalendarDisplayNameType = UCalendarDisplayNameType(0i32);
pub const UCAL_SHORT_STANDARD: UCalendarDisplayNameType = UCalendarDisplayNameType(1i32);
pub const UCAL_DST: UCalendarDisplayNameType = UCalendarDisplayNameType(2i32);
pub const UCAL_SHORT_DST: UCalendarDisplayNameType = UCalendarDisplayNameType(3i32);
impl ::core::marker::Copy for UCalendarDisplayNameType {}
impl ::core::clone::Clone for UCalendarDisplayNameType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UCalendarDisplayNameType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCalendarDisplayNameType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCalendarDisplayNameType")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UCalendarDisplayNameType {
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
pub struct UCalendarLimitType(pub i32);
pub const UCAL_MINIMUM: UCalendarLimitType = UCalendarLimitType(0i32);
pub const UCAL_MAXIMUM: UCalendarLimitType = UCalendarLimitType(1i32);
pub const UCAL_GREATEST_MINIMUM: UCalendarLimitType = UCalendarLimitType(2i32);
pub const UCAL_LEAST_MAXIMUM: UCalendarLimitType = UCalendarLimitType(3i32);
pub const UCAL_ACTUAL_MINIMUM: UCalendarLimitType = UCalendarLimitType(4i32);
pub const UCAL_ACTUAL_MAXIMUM: UCalendarLimitType = UCalendarLimitType(5i32);
impl ::core::marker::Copy for UCalendarLimitType {}
impl ::core::clone::Clone for UCalendarLimitType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UCalendarLimitType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCalendarLimitType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCalendarLimitType").field(&self.0).finish()
    }
}
impl FromIntoMemory for UCalendarLimitType {
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
pub struct UCalendarMonths(pub i32);
pub const UCAL_JANUARY: UCalendarMonths = UCalendarMonths(0i32);
pub const UCAL_FEBRUARY: UCalendarMonths = UCalendarMonths(1i32);
pub const UCAL_MARCH: UCalendarMonths = UCalendarMonths(2i32);
pub const UCAL_APRIL: UCalendarMonths = UCalendarMonths(3i32);
pub const UCAL_MAY: UCalendarMonths = UCalendarMonths(4i32);
pub const UCAL_JUNE: UCalendarMonths = UCalendarMonths(5i32);
pub const UCAL_JULY: UCalendarMonths = UCalendarMonths(6i32);
pub const UCAL_AUGUST: UCalendarMonths = UCalendarMonths(7i32);
pub const UCAL_SEPTEMBER: UCalendarMonths = UCalendarMonths(8i32);
pub const UCAL_OCTOBER: UCalendarMonths = UCalendarMonths(9i32);
pub const UCAL_NOVEMBER: UCalendarMonths = UCalendarMonths(10i32);
pub const UCAL_DECEMBER: UCalendarMonths = UCalendarMonths(11i32);
pub const UCAL_UNDECIMBER: UCalendarMonths = UCalendarMonths(12i32);
impl ::core::marker::Copy for UCalendarMonths {}
impl ::core::clone::Clone for UCalendarMonths {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UCalendarMonths {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCalendarMonths {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCalendarMonths").field(&self.0).finish()
    }
}
impl FromIntoMemory for UCalendarMonths {
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
pub struct UCalendarType(pub i32);
pub const UCAL_TRADITIONAL: UCalendarType = UCalendarType(0i32);
pub const UCAL_DEFAULT: UCalendarType = UCalendarType(0i32);
pub const UCAL_GREGORIAN: UCalendarType = UCalendarType(1i32);
impl ::core::marker::Copy for UCalendarType {}
impl ::core::clone::Clone for UCalendarType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UCalendarType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCalendarType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCalendarType").field(&self.0).finish()
    }
}
impl FromIntoMemory for UCalendarType {
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
pub struct UCalendarWallTimeOption(pub i32);
pub const UCAL_WALLTIME_LAST: UCalendarWallTimeOption = UCalendarWallTimeOption(0i32);
pub const UCAL_WALLTIME_FIRST: UCalendarWallTimeOption = UCalendarWallTimeOption(1i32);
pub const UCAL_WALLTIME_NEXT_VALID: UCalendarWallTimeOption = UCalendarWallTimeOption(2i32);
impl ::core::marker::Copy for UCalendarWallTimeOption {}
impl ::core::clone::Clone for UCalendarWallTimeOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UCalendarWallTimeOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCalendarWallTimeOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCalendarWallTimeOption")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UCalendarWallTimeOption {
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
pub struct UCalendarWeekdayType(pub i32);
pub const UCAL_WEEKDAY: UCalendarWeekdayType = UCalendarWeekdayType(0i32);
pub const UCAL_WEEKEND: UCalendarWeekdayType = UCalendarWeekdayType(1i32);
pub const UCAL_WEEKEND_ONSET: UCalendarWeekdayType = UCalendarWeekdayType(2i32);
pub const UCAL_WEEKEND_CEASE: UCalendarWeekdayType = UCalendarWeekdayType(3i32);
impl ::core::marker::Copy for UCalendarWeekdayType {}
impl ::core::clone::Clone for UCalendarWeekdayType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UCalendarWeekdayType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCalendarWeekdayType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCalendarWeekdayType")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UCalendarWeekdayType {
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
pub struct UCaseMap(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UCharCategory(pub i32);
pub const U_UNASSIGNED: UCharCategory = UCharCategory(0i32);
pub const U_GENERAL_OTHER_TYPES: UCharCategory = UCharCategory(0i32);
pub const U_UPPERCASE_LETTER: UCharCategory = UCharCategory(1i32);
pub const U_LOWERCASE_LETTER: UCharCategory = UCharCategory(2i32);
pub const U_TITLECASE_LETTER: UCharCategory = UCharCategory(3i32);
pub const U_MODIFIER_LETTER: UCharCategory = UCharCategory(4i32);
pub const U_OTHER_LETTER: UCharCategory = UCharCategory(5i32);
pub const U_NON_SPACING_MARK: UCharCategory = UCharCategory(6i32);
pub const U_ENCLOSING_MARK: UCharCategory = UCharCategory(7i32);
pub const U_COMBINING_SPACING_MARK: UCharCategory = UCharCategory(8i32);
pub const U_DECIMAL_DIGIT_NUMBER: UCharCategory = UCharCategory(9i32);
pub const U_LETTER_NUMBER: UCharCategory = UCharCategory(10i32);
pub const U_OTHER_NUMBER: UCharCategory = UCharCategory(11i32);
pub const U_SPACE_SEPARATOR: UCharCategory = UCharCategory(12i32);
pub const U_LINE_SEPARATOR: UCharCategory = UCharCategory(13i32);
pub const U_PARAGRAPH_SEPARATOR: UCharCategory = UCharCategory(14i32);
pub const U_CONTROL_CHAR: UCharCategory = UCharCategory(15i32);
pub const U_FORMAT_CHAR: UCharCategory = UCharCategory(16i32);
pub const U_PRIVATE_USE_CHAR: UCharCategory = UCharCategory(17i32);
pub const U_SURROGATE: UCharCategory = UCharCategory(18i32);
pub const U_DASH_PUNCTUATION: UCharCategory = UCharCategory(19i32);
pub const U_START_PUNCTUATION: UCharCategory = UCharCategory(20i32);
pub const U_END_PUNCTUATION: UCharCategory = UCharCategory(21i32);
pub const U_CONNECTOR_PUNCTUATION: UCharCategory = UCharCategory(22i32);
pub const U_OTHER_PUNCTUATION: UCharCategory = UCharCategory(23i32);
pub const U_MATH_SYMBOL: UCharCategory = UCharCategory(24i32);
pub const U_CURRENCY_SYMBOL: UCharCategory = UCharCategory(25i32);
pub const U_MODIFIER_SYMBOL: UCharCategory = UCharCategory(26i32);
pub const U_OTHER_SYMBOL: UCharCategory = UCharCategory(27i32);
pub const U_INITIAL_PUNCTUATION: UCharCategory = UCharCategory(28i32);
pub const U_FINAL_PUNCTUATION: UCharCategory = UCharCategory(29i32);
pub const U_CHAR_CATEGORY_COUNT: UCharCategory = UCharCategory(30i32);
impl ::core::marker::Copy for UCharCategory {}
impl ::core::clone::Clone for UCharCategory {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UCharCategory {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCharCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCharCategory").field(&self.0).finish()
    }
}
impl FromIntoMemory for UCharCategory {
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
pub struct UCharDirection(pub i32);
pub const U_LEFT_TO_RIGHT: UCharDirection = UCharDirection(0i32);
pub const U_RIGHT_TO_LEFT: UCharDirection = UCharDirection(1i32);
pub const U_EUROPEAN_NUMBER: UCharDirection = UCharDirection(2i32);
pub const U_EUROPEAN_NUMBER_SEPARATOR: UCharDirection = UCharDirection(3i32);
pub const U_EUROPEAN_NUMBER_TERMINATOR: UCharDirection = UCharDirection(4i32);
pub const U_ARABIC_NUMBER: UCharDirection = UCharDirection(5i32);
pub const U_COMMON_NUMBER_SEPARATOR: UCharDirection = UCharDirection(6i32);
pub const U_BLOCK_SEPARATOR: UCharDirection = UCharDirection(7i32);
pub const U_SEGMENT_SEPARATOR: UCharDirection = UCharDirection(8i32);
pub const U_WHITE_SPACE_NEUTRAL: UCharDirection = UCharDirection(9i32);
pub const U_OTHER_NEUTRAL: UCharDirection = UCharDirection(10i32);
pub const U_LEFT_TO_RIGHT_EMBEDDING: UCharDirection = UCharDirection(11i32);
pub const U_LEFT_TO_RIGHT_OVERRIDE: UCharDirection = UCharDirection(12i32);
pub const U_RIGHT_TO_LEFT_ARABIC: UCharDirection = UCharDirection(13i32);
pub const U_RIGHT_TO_LEFT_EMBEDDING: UCharDirection = UCharDirection(14i32);
pub const U_RIGHT_TO_LEFT_OVERRIDE: UCharDirection = UCharDirection(15i32);
pub const U_POP_DIRECTIONAL_FORMAT: UCharDirection = UCharDirection(16i32);
pub const U_DIR_NON_SPACING_MARK: UCharDirection = UCharDirection(17i32);
pub const U_BOUNDARY_NEUTRAL: UCharDirection = UCharDirection(18i32);
pub const U_FIRST_STRONG_ISOLATE: UCharDirection = UCharDirection(19i32);
pub const U_LEFT_TO_RIGHT_ISOLATE: UCharDirection = UCharDirection(20i32);
pub const U_RIGHT_TO_LEFT_ISOLATE: UCharDirection = UCharDirection(21i32);
pub const U_POP_DIRECTIONAL_ISOLATE: UCharDirection = UCharDirection(22i32);
impl ::core::marker::Copy for UCharDirection {}
impl ::core::clone::Clone for UCharDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UCharDirection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCharDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCharDirection").field(&self.0).finish()
    }
}
impl FromIntoMemory for UCharDirection {
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
pub type UCharEnumTypeRange =
    StdCallFnPtr<(ConstPtr<::core::ffi::c_void>, i32, i32, UCharCategory), i8>;
pub struct UCharIterator {
    pub context: ConstPtr<::core::ffi::c_void>,
    pub length: i32,
    pub start: i32,
    pub index: i32,
    pub limit: i32,
    pub reservedField: i32,
    pub getIndex: UCharIteratorGetIndex,
    pub r#move: UCharIteratorMove,
    pub hasNext: UCharIteratorHasNext,
    pub hasPrevious: UCharIteratorHasPrevious,
    pub current: UCharIteratorCurrent,
    pub next: UCharIteratorNext,
    pub previous: UCharIteratorPrevious,
    pub reservedFn: UCharIteratorReserved,
    pub getState: UCharIteratorGetState,
    pub setState: UCharIteratorSetState,
}
impl ::core::marker::Copy for UCharIterator {}
impl ::core::clone::Clone for UCharIterator {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UCharIterator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UCharIterator")
            .field("context", &self.context)
            .field("length", &self.length)
            .field("start", &self.start)
            .field("index", &self.index)
            .field("limit", &self.limit)
            .field("reservedField", &self.reservedField)
            .field("getIndex", &self.getIndex)
            .field("move", &self.r#move)
            .field("hasNext", &self.hasNext)
            .field("hasPrevious", &self.hasPrevious)
            .field("current", &self.current)
            .field("next", &self.next)
            .field("previous", &self.previous)
            .field("reservedFn", &self.reservedFn)
            .field("getState", &self.getState)
            .field("setState", &self.setState)
            .finish()
    }
}
impl ::core::cmp::PartialEq for UCharIterator {
    fn eq(&self, other: &Self) -> bool {
        self.context == other.context
            && self.length == other.length
            && self.start == other.start
            && self.index == other.index
            && self.limit == other.limit
            && self.reservedField == other.reservedField
            && self.getIndex == other.getIndex
            && self.r#move == other.r#move
            && self.hasNext == other.hasNext
            && self.hasPrevious == other.hasPrevious
            && self.current == other.current
            && self.next == other.next
            && self.previous == other.previous
            && self.reservedFn == other.reservedFn
            && self.getState == other.getState
            && self.setState == other.setState
    }
}
impl ::core::cmp::Eq for UCharIterator {}
impl FromIntoMemory for UCharIterator {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 64u32 as usize);
        let f_context =
            <ConstPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_length = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_start = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_index = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_limit = <i32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_reservedField = <i32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_getIndex = <UCharIteratorGetIndex as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_move = <UCharIteratorMove as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_hasNext = <UCharIteratorHasNext as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_hasPrevious =
            <UCharIteratorHasPrevious as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_current = <UCharIteratorCurrent as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_next = <UCharIteratorNext as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_previous = <UCharIteratorPrevious as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_reservedFn = <UCharIteratorReserved as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_getState = <UCharIteratorGetState as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_setState = <UCharIteratorSetState as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        Self {
            context: f_context,
            length: f_length,
            start: f_start,
            index: f_index,
            limit: f_limit,
            reservedField: f_reservedField,
            getIndex: f_getIndex,
            r#move: f_move,
            hasNext: f_hasNext,
            hasPrevious: f_hasPrevious,
            current: f_current,
            next: f_next,
            previous: f_previous,
            reservedFn: f_reservedFn,
            getState: f_getState,
            setState: f_setState,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 64u32 as usize);
        FromIntoMemory::into_bytes(self.context, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.length, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.start, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.index, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.limit, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.reservedField, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.getIndex, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.r#move, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.hasNext, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.hasPrevious, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.current, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.next, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.previous, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.reservedFn, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.getState, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.setState, &mut into[60..60 + 4]);
    }
    fn size() -> usize {
        64u32 as usize
    }
}
pub type UCharIteratorCurrent = StdCallFnPtr<(MutPtr<UCharIterator>,), i32>;
pub type UCharIteratorGetIndex = StdCallFnPtr<(MutPtr<UCharIterator>, UCharIteratorOrigin), i32>;
pub type UCharIteratorGetState = StdCallFnPtr<(ConstPtr<UCharIterator>,), u32>;
pub type UCharIteratorHasNext = StdCallFnPtr<(MutPtr<UCharIterator>,), i8>;
pub type UCharIteratorHasPrevious = StdCallFnPtr<(MutPtr<UCharIterator>,), i8>;
pub type UCharIteratorMove = StdCallFnPtr<(MutPtr<UCharIterator>, i32, UCharIteratorOrigin), i32>;
pub type UCharIteratorNext = StdCallFnPtr<(MutPtr<UCharIterator>,), i32>;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UCharIteratorOrigin(pub i32);
pub const UITER_START: UCharIteratorOrigin = UCharIteratorOrigin(0i32);
pub const UITER_CURRENT: UCharIteratorOrigin = UCharIteratorOrigin(1i32);
pub const UITER_LIMIT: UCharIteratorOrigin = UCharIteratorOrigin(2i32);
pub const UITER_ZERO: UCharIteratorOrigin = UCharIteratorOrigin(3i32);
pub const UITER_LENGTH: UCharIteratorOrigin = UCharIteratorOrigin(4i32);
impl ::core::marker::Copy for UCharIteratorOrigin {}
impl ::core::clone::Clone for UCharIteratorOrigin {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UCharIteratorOrigin {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCharIteratorOrigin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCharIteratorOrigin").field(&self.0).finish()
    }
}
impl FromIntoMemory for UCharIteratorOrigin {
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
pub type UCharIteratorPrevious = StdCallFnPtr<(MutPtr<UCharIterator>,), i32>;
pub type UCharIteratorReserved = StdCallFnPtr<(MutPtr<UCharIterator>, i32), i32>;
pub type UCharIteratorSetState = StdCallFnPtr<(MutPtr<UCharIterator>, u32, MutPtr<UErrorCode>), ()>;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UCharNameChoice(pub i32);
pub const U_UNICODE_CHAR_NAME: UCharNameChoice = UCharNameChoice(0i32);
pub const U_EXTENDED_CHAR_NAME: UCharNameChoice = UCharNameChoice(2i32);
pub const U_CHAR_NAME_ALIAS: UCharNameChoice = UCharNameChoice(3i32);
impl ::core::marker::Copy for UCharNameChoice {}
impl ::core::clone::Clone for UCharNameChoice {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UCharNameChoice {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCharNameChoice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCharNameChoice").field(&self.0).finish()
    }
}
impl FromIntoMemory for UCharNameChoice {
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
pub struct UCharsetDetector(pub u8);
pub struct UCharsetMatch(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UColAttribute(pub i32);
pub const UCOL_FRENCH_COLLATION: UColAttribute = UColAttribute(0i32);
pub const UCOL_ALTERNATE_HANDLING: UColAttribute = UColAttribute(1i32);
pub const UCOL_CASE_FIRST: UColAttribute = UColAttribute(2i32);
pub const UCOL_CASE_LEVEL: UColAttribute = UColAttribute(3i32);
pub const UCOL_NORMALIZATION_MODE: UColAttribute = UColAttribute(4i32);
pub const UCOL_DECOMPOSITION_MODE: UColAttribute = UColAttribute(4i32);
pub const UCOL_STRENGTH: UColAttribute = UColAttribute(5i32);
pub const UCOL_NUMERIC_COLLATION: UColAttribute = UColAttribute(7i32);
pub const UCOL_ATTRIBUTE_COUNT: UColAttribute = UColAttribute(8i32);
impl ::core::marker::Copy for UColAttribute {}
impl ::core::clone::Clone for UColAttribute {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UColAttribute {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UColAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UColAttribute").field(&self.0).finish()
    }
}
impl FromIntoMemory for UColAttribute {
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
pub struct UColAttributeValue(pub i32);
pub const UCOL_DEFAULT: UColAttributeValue = UColAttributeValue(-1i32);
pub const UCOL_PRIMARY: UColAttributeValue = UColAttributeValue(0i32);
pub const UCOL_SECONDARY: UColAttributeValue = UColAttributeValue(1i32);
pub const UCOL_TERTIARY: UColAttributeValue = UColAttributeValue(2i32);
pub const UCOL_DEFAULT_STRENGTH: UColAttributeValue = UColAttributeValue(2i32);
pub const UCOL_CE_STRENGTH_LIMIT: UColAttributeValue = UColAttributeValue(3i32);
pub const UCOL_QUATERNARY: UColAttributeValue = UColAttributeValue(3i32);
pub const UCOL_IDENTICAL: UColAttributeValue = UColAttributeValue(15i32);
pub const UCOL_STRENGTH_LIMIT: UColAttributeValue = UColAttributeValue(16i32);
pub const UCOL_OFF: UColAttributeValue = UColAttributeValue(16i32);
pub const UCOL_ON: UColAttributeValue = UColAttributeValue(17i32);
pub const UCOL_SHIFTED: UColAttributeValue = UColAttributeValue(20i32);
pub const UCOL_NON_IGNORABLE: UColAttributeValue = UColAttributeValue(21i32);
pub const UCOL_LOWER_FIRST: UColAttributeValue = UColAttributeValue(24i32);
pub const UCOL_UPPER_FIRST: UColAttributeValue = UColAttributeValue(25i32);
impl ::core::marker::Copy for UColAttributeValue {}
impl ::core::clone::Clone for UColAttributeValue {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UColAttributeValue {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UColAttributeValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UColAttributeValue").field(&self.0).finish()
    }
}
impl FromIntoMemory for UColAttributeValue {
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
pub struct UColBoundMode(pub i32);
pub const UCOL_BOUND_LOWER: UColBoundMode = UColBoundMode(0i32);
pub const UCOL_BOUND_UPPER: UColBoundMode = UColBoundMode(1i32);
pub const UCOL_BOUND_UPPER_LONG: UColBoundMode = UColBoundMode(2i32);
impl ::core::marker::Copy for UColBoundMode {}
impl ::core::clone::Clone for UColBoundMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UColBoundMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UColBoundMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UColBoundMode").field(&self.0).finish()
    }
}
impl FromIntoMemory for UColBoundMode {
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
pub struct UColReorderCode(pub i32);
pub const UCOL_REORDER_CODE_DEFAULT: UColReorderCode = UColReorderCode(-1i32);
pub const UCOL_REORDER_CODE_NONE: UColReorderCode = UColReorderCode(103i32);
pub const UCOL_REORDER_CODE_OTHERS: UColReorderCode = UColReorderCode(103i32);
pub const UCOL_REORDER_CODE_SPACE: UColReorderCode = UColReorderCode(4096i32);
pub const UCOL_REORDER_CODE_FIRST: UColReorderCode = UColReorderCode(4096i32);
pub const UCOL_REORDER_CODE_PUNCTUATION: UColReorderCode = UColReorderCode(4097i32);
pub const UCOL_REORDER_CODE_SYMBOL: UColReorderCode = UColReorderCode(4098i32);
pub const UCOL_REORDER_CODE_CURRENCY: UColReorderCode = UColReorderCode(4099i32);
pub const UCOL_REORDER_CODE_DIGIT: UColReorderCode = UColReorderCode(4100i32);
impl ::core::marker::Copy for UColReorderCode {}
impl ::core::clone::Clone for UColReorderCode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UColReorderCode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UColReorderCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UColReorderCode").field(&self.0).finish()
    }
}
impl FromIntoMemory for UColReorderCode {
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
pub struct UColRuleOption(pub i32);
pub const UCOL_TAILORING_ONLY: UColRuleOption = UColRuleOption(0i32);
pub const UCOL_FULL_RULES: UColRuleOption = UColRuleOption(1i32);
impl ::core::marker::Copy for UColRuleOption {}
impl ::core::clone::Clone for UColRuleOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UColRuleOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UColRuleOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UColRuleOption").field(&self.0).finish()
    }
}
impl FromIntoMemory for UColRuleOption {
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
pub struct UCollationElements(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UCollationResult(pub i32);
pub const UCOL_EQUAL: UCollationResult = UCollationResult(0i32);
pub const UCOL_GREATER: UCollationResult = UCollationResult(1i32);
pub const UCOL_LESS: UCollationResult = UCollationResult(-1i32);
impl ::core::marker::Copy for UCollationResult {}
impl ::core::clone::Clone for UCollationResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UCollationResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCollationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCollationResult").field(&self.0).finish()
    }
}
impl FromIntoMemory for UCollationResult {
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
pub struct UCollator(pub u8);
pub struct UConstrainedFieldPosition(pub u8);
pub struct UConverter(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UConverterCallbackReason(pub i32);
pub const UCNV_UNASSIGNED: UConverterCallbackReason = UConverterCallbackReason(0i32);
pub const UCNV_ILLEGAL: UConverterCallbackReason = UConverterCallbackReason(1i32);
pub const UCNV_IRREGULAR: UConverterCallbackReason = UConverterCallbackReason(2i32);
pub const UCNV_RESET: UConverterCallbackReason = UConverterCallbackReason(3i32);
pub const UCNV_CLOSE: UConverterCallbackReason = UConverterCallbackReason(4i32);
pub const UCNV_CLONE: UConverterCallbackReason = UConverterCallbackReason(5i32);
impl ::core::marker::Copy for UConverterCallbackReason {}
impl ::core::clone::Clone for UConverterCallbackReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UConverterCallbackReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UConverterCallbackReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UConverterCallbackReason")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UConverterCallbackReason {
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
pub type UConverterFromUCallback = StdCallFnPtr<
    (
        ConstPtr<::core::ffi::c_void>,
        MutPtr<UConverterFromUnicodeArgs>,
        ConstPtr<u16>,
        i32,
        i32,
        UConverterCallbackReason,
        MutPtr<UErrorCode>,
    ),
    (),
>;
pub struct UConverterFromUnicodeArgs {
    pub size: u16,
    pub flush: i8,
    pub converter: MutPtr<UConverter>,
    pub source: ConstPtr<u16>,
    pub sourceLimit: ConstPtr<u16>,
    pub target: PSTR,
    pub targetLimit: PCSTR,
    pub offsets: MutPtr<i32>,
}
impl ::core::marker::Copy for UConverterFromUnicodeArgs {}
impl ::core::clone::Clone for UConverterFromUnicodeArgs {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UConverterFromUnicodeArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UConverterFromUnicodeArgs")
            .field("size", &self.size)
            .field("flush", &self.flush)
            .field("converter", &self.converter)
            .field("source", &self.source)
            .field("sourceLimit", &self.sourceLimit)
            .field("target", &self.target)
            .field("targetLimit", &self.targetLimit)
            .field("offsets", &self.offsets)
            .finish()
    }
}
impl ::core::cmp::PartialEq for UConverterFromUnicodeArgs {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
            && self.flush == other.flush
            && self.converter == other.converter
            && self.source == other.source
            && self.sourceLimit == other.sourceLimit
            && self.target == other.target
            && self.targetLimit == other.targetLimit
            && self.offsets == other.offsets
    }
}
impl ::core::cmp::Eq for UConverterFromUnicodeArgs {}
impl FromIntoMemory for UConverterFromUnicodeArgs {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28u32 as usize);
        let f_size = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_flush = <i8 as FromIntoMemory>::from_bytes(&from[2..2 + 1]);
        let f_converter = <MutPtr<UConverter> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_source = <ConstPtr<u16> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_sourceLimit = <ConstPtr<u16> as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_target = <PSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_targetLimit = <PCSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_offsets = <MutPtr<i32> as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        Self {
            size: f_size,
            flush: f_flush,
            converter: f_converter,
            source: f_source,
            sourceLimit: f_sourceLimit,
            target: f_target,
            targetLimit: f_targetLimit,
            offsets: f_offsets,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28u32 as usize);
        FromIntoMemory::into_bytes(self.size, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.flush, &mut into[2..2 + 1]);
        FromIntoMemory::into_bytes(self.converter, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.source, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.sourceLimit, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.target, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.targetLimit, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.offsets, &mut into[24..24 + 4]);
    }
    fn size() -> usize {
        28u32 as usize
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UConverterPlatform(pub i32);
pub const UCNV_UNKNOWN: UConverterPlatform = UConverterPlatform(-1i32);
pub const UCNV_IBM: UConverterPlatform = UConverterPlatform(0i32);
impl ::core::marker::Copy for UConverterPlatform {}
impl ::core::clone::Clone for UConverterPlatform {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UConverterPlatform {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UConverterPlatform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UConverterPlatform").field(&self.0).finish()
    }
}
impl FromIntoMemory for UConverterPlatform {
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
pub struct UConverterSelector(pub u8);
pub type UConverterToUCallback = StdCallFnPtr<
    (
        ConstPtr<::core::ffi::c_void>,
        MutPtr<UConverterToUnicodeArgs>,
        PCSTR,
        i32,
        UConverterCallbackReason,
        MutPtr<UErrorCode>,
    ),
    (),
>;
pub struct UConverterToUnicodeArgs {
    pub size: u16,
    pub flush: i8,
    pub converter: MutPtr<UConverter>,
    pub source: PCSTR,
    pub sourceLimit: PCSTR,
    pub target: MutPtr<u16>,
    pub targetLimit: ConstPtr<u16>,
    pub offsets: MutPtr<i32>,
}
impl ::core::marker::Copy for UConverterToUnicodeArgs {}
impl ::core::clone::Clone for UConverterToUnicodeArgs {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UConverterToUnicodeArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UConverterToUnicodeArgs")
            .field("size", &self.size)
            .field("flush", &self.flush)
            .field("converter", &self.converter)
            .field("source", &self.source)
            .field("sourceLimit", &self.sourceLimit)
            .field("target", &self.target)
            .field("targetLimit", &self.targetLimit)
            .field("offsets", &self.offsets)
            .finish()
    }
}
impl ::core::cmp::PartialEq for UConverterToUnicodeArgs {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
            && self.flush == other.flush
            && self.converter == other.converter
            && self.source == other.source
            && self.sourceLimit == other.sourceLimit
            && self.target == other.target
            && self.targetLimit == other.targetLimit
            && self.offsets == other.offsets
    }
}
impl ::core::cmp::Eq for UConverterToUnicodeArgs {}
impl FromIntoMemory for UConverterToUnicodeArgs {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28u32 as usize);
        let f_size = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_flush = <i8 as FromIntoMemory>::from_bytes(&from[2..2 + 1]);
        let f_converter = <MutPtr<UConverter> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_source = <PCSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_sourceLimit = <PCSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_target = <MutPtr<u16> as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_targetLimit = <ConstPtr<u16> as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_offsets = <MutPtr<i32> as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        Self {
            size: f_size,
            flush: f_flush,
            converter: f_converter,
            source: f_source,
            sourceLimit: f_sourceLimit,
            target: f_target,
            targetLimit: f_targetLimit,
            offsets: f_offsets,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28u32 as usize);
        FromIntoMemory::into_bytes(self.size, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.flush, &mut into[2..2 + 1]);
        FromIntoMemory::into_bytes(self.converter, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.source, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.sourceLimit, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.target, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.targetLimit, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.offsets, &mut into[24..24 + 4]);
    }
    fn size() -> usize {
        28u32 as usize
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UConverterType(pub i32);
pub const UCNV_UNSUPPORTED_CONVERTER: UConverterType = UConverterType(-1i32);
pub const UCNV_SBCS: UConverterType = UConverterType(0i32);
pub const UCNV_DBCS: UConverterType = UConverterType(1i32);
pub const UCNV_MBCS: UConverterType = UConverterType(2i32);
pub const UCNV_LATIN_1: UConverterType = UConverterType(3i32);
pub const UCNV_UTF8: UConverterType = UConverterType(4i32);
pub const UCNV_UTF16_BigEndian: UConverterType = UConverterType(5i32);
pub const UCNV_UTF16_LittleEndian: UConverterType = UConverterType(6i32);
pub const UCNV_UTF32_BigEndian: UConverterType = UConverterType(7i32);
pub const UCNV_UTF32_LittleEndian: UConverterType = UConverterType(8i32);
pub const UCNV_EBCDIC_STATEFUL: UConverterType = UConverterType(9i32);
pub const UCNV_ISO_2022: UConverterType = UConverterType(10i32);
pub const UCNV_LMBCS_1: UConverterType = UConverterType(11i32);
pub const UCNV_LMBCS_2: UConverterType = UConverterType(12i32);
pub const UCNV_LMBCS_3: UConverterType = UConverterType(13i32);
pub const UCNV_LMBCS_4: UConverterType = UConverterType(14i32);
pub const UCNV_LMBCS_5: UConverterType = UConverterType(15i32);
pub const UCNV_LMBCS_6: UConverterType = UConverterType(16i32);
pub const UCNV_LMBCS_8: UConverterType = UConverterType(17i32);
pub const UCNV_LMBCS_11: UConverterType = UConverterType(18i32);
pub const UCNV_LMBCS_16: UConverterType = UConverterType(19i32);
pub const UCNV_LMBCS_17: UConverterType = UConverterType(20i32);
pub const UCNV_LMBCS_18: UConverterType = UConverterType(21i32);
pub const UCNV_LMBCS_19: UConverterType = UConverterType(22i32);
pub const UCNV_LMBCS_LAST: UConverterType = UConverterType(22i32);
pub const UCNV_HZ: UConverterType = UConverterType(23i32);
pub const UCNV_SCSU: UConverterType = UConverterType(24i32);
pub const UCNV_ISCII: UConverterType = UConverterType(25i32);
pub const UCNV_US_ASCII: UConverterType = UConverterType(26i32);
pub const UCNV_UTF7: UConverterType = UConverterType(27i32);
pub const UCNV_BOCU1: UConverterType = UConverterType(28i32);
pub const UCNV_UTF16: UConverterType = UConverterType(29i32);
pub const UCNV_UTF32: UConverterType = UConverterType(30i32);
pub const UCNV_CESU8: UConverterType = UConverterType(31i32);
pub const UCNV_IMAP_MAILBOX: UConverterType = UConverterType(32i32);
pub const UCNV_COMPOUND_TEXT: UConverterType = UConverterType(33i32);
pub const UCNV_NUMBER_OF_SUPPORTED_CONVERTER_TYPES: UConverterType = UConverterType(34i32);
impl ::core::marker::Copy for UConverterType {}
impl ::core::clone::Clone for UConverterType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UConverterType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UConverterType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UConverterType").field(&self.0).finish()
    }
}
impl FromIntoMemory for UConverterType {
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
pub struct UConverterUnicodeSet(pub i32);
pub const UCNV_ROUNDTRIP_SET: UConverterUnicodeSet = UConverterUnicodeSet(0i32);
pub const UCNV_ROUNDTRIP_AND_FALLBACK_SET: UConverterUnicodeSet = UConverterUnicodeSet(1i32);
impl ::core::marker::Copy for UConverterUnicodeSet {}
impl ::core::clone::Clone for UConverterUnicodeSet {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UConverterUnicodeSet {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UConverterUnicodeSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UConverterUnicodeSet")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UConverterUnicodeSet {
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
pub struct UCurrCurrencyType(pub i32);
pub const UCURR_ALL: UCurrCurrencyType = UCurrCurrencyType(2147483647i32);
pub const UCURR_COMMON: UCurrCurrencyType = UCurrCurrencyType(1i32);
pub const UCURR_UNCOMMON: UCurrCurrencyType = UCurrCurrencyType(2i32);
pub const UCURR_DEPRECATED: UCurrCurrencyType = UCurrCurrencyType(4i32);
pub const UCURR_NON_DEPRECATED: UCurrCurrencyType = UCurrCurrencyType(8i32);
impl ::core::marker::Copy for UCurrCurrencyType {}
impl ::core::clone::Clone for UCurrCurrencyType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UCurrCurrencyType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCurrCurrencyType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCurrCurrencyType").field(&self.0).finish()
    }
}
impl FromIntoMemory for UCurrCurrencyType {
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
pub struct UCurrNameStyle(pub i32);
pub const UCURR_SYMBOL_NAME: UCurrNameStyle = UCurrNameStyle(0i32);
pub const UCURR_LONG_NAME: UCurrNameStyle = UCurrNameStyle(1i32);
pub const UCURR_NARROW_SYMBOL_NAME: UCurrNameStyle = UCurrNameStyle(2i32);
impl ::core::marker::Copy for UCurrNameStyle {}
impl ::core::clone::Clone for UCurrNameStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UCurrNameStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCurrNameStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCurrNameStyle").field(&self.0).finish()
    }
}
impl FromIntoMemory for UCurrNameStyle {
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
pub struct UCurrencySpacing(pub i32);
pub const UNUM_CURRENCY_MATCH: UCurrencySpacing = UCurrencySpacing(0i32);
pub const UNUM_CURRENCY_SURROUNDING_MATCH: UCurrencySpacing = UCurrencySpacing(1i32);
pub const UNUM_CURRENCY_INSERT: UCurrencySpacing = UCurrencySpacing(2i32);
pub const UNUM_CURRENCY_SPACING_COUNT: UCurrencySpacing = UCurrencySpacing(3i32);
impl ::core::marker::Copy for UCurrencySpacing {}
impl ::core::clone::Clone for UCurrencySpacing {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UCurrencySpacing {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCurrencySpacing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCurrencySpacing").field(&self.0).finish()
    }
}
impl FromIntoMemory for UCurrencySpacing {
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
pub struct UCurrencyUsage(pub i32);
pub const UCURR_USAGE_STANDARD: UCurrencyUsage = UCurrencyUsage(0i32);
pub const UCURR_USAGE_CASH: UCurrencyUsage = UCurrencyUsage(1i32);
impl ::core::marker::Copy for UCurrencyUsage {}
impl ::core::clone::Clone for UCurrencyUsage {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UCurrencyUsage {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UCurrencyUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UCurrencyUsage").field(&self.0).finish()
    }
}
impl FromIntoMemory for UCurrencyUsage {
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
pub const UDAT_ABBR_GENERIC_TZ: &'static str = "v";
pub const UDAT_ABBR_MONTH: &'static str = "MMM";
pub const UDAT_ABBR_MONTH_DAY: &'static str = "MMMd";
pub const UDAT_ABBR_MONTH_WEEKDAY_DAY: &'static str = "MMMEd";
pub const UDAT_ABBR_QUARTER: &'static str = "QQQ";
pub const UDAT_ABBR_SPECIFIC_TZ: &'static str = "z";
pub const UDAT_ABBR_UTC_TZ: &'static str = "ZZZZ";
pub const UDAT_ABBR_WEEKDAY: &'static str = "E";
pub const UDAT_DAY: &'static str = "d";
pub const UDAT_GENERIC_TZ: &'static str = "vvvv";
pub const UDAT_HOUR: &'static str = "j";
pub const UDAT_HOUR24: &'static str = "H";
pub const UDAT_HOUR24_MINUTE: &'static str = "Hm";
pub const UDAT_HOUR24_MINUTE_SECOND: &'static str = "Hms";
pub const UDAT_HOUR_MINUTE: &'static str = "jm";
pub const UDAT_HOUR_MINUTE_SECOND: &'static str = "jms";
pub const UDAT_LOCATION_TZ: &'static str = "VVVV";
pub const UDAT_MINUTE: &'static str = "m";
pub const UDAT_MINUTE_SECOND: &'static str = "ms";
pub const UDAT_MONTH: &'static str = "MMMM";
pub const UDAT_MONTH_DAY: &'static str = "MMMMd";
pub const UDAT_MONTH_WEEKDAY_DAY: &'static str = "MMMMEEEEd";
pub const UDAT_NUM_MONTH: &'static str = "M";
pub const UDAT_NUM_MONTH_DAY: &'static str = "Md";
pub const UDAT_NUM_MONTH_WEEKDAY_DAY: &'static str = "MEd";
pub const UDAT_QUARTER: &'static str = "QQQQ";
pub const UDAT_SECOND: &'static str = "s";
pub const UDAT_SPECIFIC_TZ: &'static str = "zzzz";
pub const UDAT_WEEKDAY: &'static str = "EEEE";
pub const UDAT_YEAR: &'static str = "y";
pub const UDAT_YEAR_ABBR_MONTH: &'static str = "yMMM";
pub const UDAT_YEAR_ABBR_MONTH_DAY: &'static str = "yMMMd";
pub const UDAT_YEAR_ABBR_MONTH_WEEKDAY_DAY: &'static str = "yMMMEd";
pub const UDAT_YEAR_ABBR_QUARTER: &'static str = "yQQQ";
pub const UDAT_YEAR_MONTH: &'static str = "yMMMM";
pub const UDAT_YEAR_MONTH_DAY: &'static str = "yMMMMd";
pub const UDAT_YEAR_MONTH_WEEKDAY_DAY: &'static str = "yMMMMEEEEd";
pub const UDAT_YEAR_NUM_MONTH: &'static str = "yM";
pub const UDAT_YEAR_NUM_MONTH_DAY: &'static str = "yMd";
pub const UDAT_YEAR_NUM_MONTH_WEEKDAY_DAY: &'static str = "yMEd";
pub const UDAT_YEAR_QUARTER: &'static str = "yQQQQ";
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UDateAbsoluteUnit(pub i32);
pub const UDAT_ABSOLUTE_SUNDAY: UDateAbsoluteUnit = UDateAbsoluteUnit(0i32);
pub const UDAT_ABSOLUTE_MONDAY: UDateAbsoluteUnit = UDateAbsoluteUnit(1i32);
pub const UDAT_ABSOLUTE_TUESDAY: UDateAbsoluteUnit = UDateAbsoluteUnit(2i32);
pub const UDAT_ABSOLUTE_WEDNESDAY: UDateAbsoluteUnit = UDateAbsoluteUnit(3i32);
pub const UDAT_ABSOLUTE_THURSDAY: UDateAbsoluteUnit = UDateAbsoluteUnit(4i32);
pub const UDAT_ABSOLUTE_FRIDAY: UDateAbsoluteUnit = UDateAbsoluteUnit(5i32);
pub const UDAT_ABSOLUTE_SATURDAY: UDateAbsoluteUnit = UDateAbsoluteUnit(6i32);
pub const UDAT_ABSOLUTE_DAY: UDateAbsoluteUnit = UDateAbsoluteUnit(7i32);
pub const UDAT_ABSOLUTE_WEEK: UDateAbsoluteUnit = UDateAbsoluteUnit(8i32);
pub const UDAT_ABSOLUTE_MONTH: UDateAbsoluteUnit = UDateAbsoluteUnit(9i32);
pub const UDAT_ABSOLUTE_YEAR: UDateAbsoluteUnit = UDateAbsoluteUnit(10i32);
pub const UDAT_ABSOLUTE_NOW: UDateAbsoluteUnit = UDateAbsoluteUnit(11i32);
pub const UDAT_ABSOLUTE_UNIT_COUNT: UDateAbsoluteUnit = UDateAbsoluteUnit(12i32);
impl ::core::marker::Copy for UDateAbsoluteUnit {}
impl ::core::clone::Clone for UDateAbsoluteUnit {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UDateAbsoluteUnit {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDateAbsoluteUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDateAbsoluteUnit").field(&self.0).finish()
    }
}
impl FromIntoMemory for UDateAbsoluteUnit {
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
pub struct UDateDirection(pub i32);
pub const UDAT_DIRECTION_LAST_2: UDateDirection = UDateDirection(0i32);
pub const UDAT_DIRECTION_LAST: UDateDirection = UDateDirection(1i32);
pub const UDAT_DIRECTION_THIS: UDateDirection = UDateDirection(2i32);
pub const UDAT_DIRECTION_NEXT: UDateDirection = UDateDirection(3i32);
pub const UDAT_DIRECTION_NEXT_2: UDateDirection = UDateDirection(4i32);
pub const UDAT_DIRECTION_PLAIN: UDateDirection = UDateDirection(5i32);
pub const UDAT_DIRECTION_COUNT: UDateDirection = UDateDirection(6i32);
impl ::core::marker::Copy for UDateDirection {}
impl ::core::clone::Clone for UDateDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UDateDirection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDateDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDateDirection").field(&self.0).finish()
    }
}
impl FromIntoMemory for UDateDirection {
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
pub struct UDateFormatBooleanAttribute(pub i32);
pub const UDAT_PARSE_ALLOW_WHITESPACE: UDateFormatBooleanAttribute =
    UDateFormatBooleanAttribute(0i32);
pub const UDAT_PARSE_ALLOW_NUMERIC: UDateFormatBooleanAttribute = UDateFormatBooleanAttribute(1i32);
pub const UDAT_PARSE_PARTIAL_LITERAL_MATCH: UDateFormatBooleanAttribute =
    UDateFormatBooleanAttribute(2i32);
pub const UDAT_PARSE_MULTIPLE_PATTERNS_FOR_MATCH: UDateFormatBooleanAttribute =
    UDateFormatBooleanAttribute(3i32);
pub const UDAT_BOOLEAN_ATTRIBUTE_COUNT: UDateFormatBooleanAttribute =
    UDateFormatBooleanAttribute(4i32);
impl ::core::marker::Copy for UDateFormatBooleanAttribute {}
impl ::core::clone::Clone for UDateFormatBooleanAttribute {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UDateFormatBooleanAttribute {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDateFormatBooleanAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDateFormatBooleanAttribute")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UDateFormatBooleanAttribute {
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
pub struct UDateFormatField(pub i32);
pub const UDAT_ERA_FIELD: UDateFormatField = UDateFormatField(0i32);
pub const UDAT_YEAR_FIELD: UDateFormatField = UDateFormatField(1i32);
pub const UDAT_MONTH_FIELD: UDateFormatField = UDateFormatField(2i32);
pub const UDAT_DATE_FIELD: UDateFormatField = UDateFormatField(3i32);
pub const UDAT_HOUR_OF_DAY1_FIELD: UDateFormatField = UDateFormatField(4i32);
pub const UDAT_HOUR_OF_DAY0_FIELD: UDateFormatField = UDateFormatField(5i32);
pub const UDAT_MINUTE_FIELD: UDateFormatField = UDateFormatField(6i32);
pub const UDAT_SECOND_FIELD: UDateFormatField = UDateFormatField(7i32);
pub const UDAT_FRACTIONAL_SECOND_FIELD: UDateFormatField = UDateFormatField(8i32);
pub const UDAT_DAY_OF_WEEK_FIELD: UDateFormatField = UDateFormatField(9i32);
pub const UDAT_DAY_OF_YEAR_FIELD: UDateFormatField = UDateFormatField(10i32);
pub const UDAT_DAY_OF_WEEK_IN_MONTH_FIELD: UDateFormatField = UDateFormatField(11i32);
pub const UDAT_WEEK_OF_YEAR_FIELD: UDateFormatField = UDateFormatField(12i32);
pub const UDAT_WEEK_OF_MONTH_FIELD: UDateFormatField = UDateFormatField(13i32);
pub const UDAT_AM_PM_FIELD: UDateFormatField = UDateFormatField(14i32);
pub const UDAT_HOUR1_FIELD: UDateFormatField = UDateFormatField(15i32);
pub const UDAT_HOUR0_FIELD: UDateFormatField = UDateFormatField(16i32);
pub const UDAT_TIMEZONE_FIELD: UDateFormatField = UDateFormatField(17i32);
pub const UDAT_YEAR_WOY_FIELD: UDateFormatField = UDateFormatField(18i32);
pub const UDAT_DOW_LOCAL_FIELD: UDateFormatField = UDateFormatField(19i32);
pub const UDAT_EXTENDED_YEAR_FIELD: UDateFormatField = UDateFormatField(20i32);
pub const UDAT_JULIAN_DAY_FIELD: UDateFormatField = UDateFormatField(21i32);
pub const UDAT_MILLISECONDS_IN_DAY_FIELD: UDateFormatField = UDateFormatField(22i32);
pub const UDAT_TIMEZONE_RFC_FIELD: UDateFormatField = UDateFormatField(23i32);
pub const UDAT_TIMEZONE_GENERIC_FIELD: UDateFormatField = UDateFormatField(24i32);
pub const UDAT_STANDALONE_DAY_FIELD: UDateFormatField = UDateFormatField(25i32);
pub const UDAT_STANDALONE_MONTH_FIELD: UDateFormatField = UDateFormatField(26i32);
pub const UDAT_QUARTER_FIELD: UDateFormatField = UDateFormatField(27i32);
pub const UDAT_STANDALONE_QUARTER_FIELD: UDateFormatField = UDateFormatField(28i32);
pub const UDAT_TIMEZONE_SPECIAL_FIELD: UDateFormatField = UDateFormatField(29i32);
pub const UDAT_YEAR_NAME_FIELD: UDateFormatField = UDateFormatField(30i32);
pub const UDAT_TIMEZONE_LOCALIZED_GMT_OFFSET_FIELD: UDateFormatField = UDateFormatField(31i32);
pub const UDAT_TIMEZONE_ISO_FIELD: UDateFormatField = UDateFormatField(32i32);
pub const UDAT_TIMEZONE_ISO_LOCAL_FIELD: UDateFormatField = UDateFormatField(33i32);
pub const UDAT_AM_PM_MIDNIGHT_NOON_FIELD: UDateFormatField = UDateFormatField(35i32);
pub const UDAT_FLEXIBLE_DAY_PERIOD_FIELD: UDateFormatField = UDateFormatField(36i32);
impl ::core::marker::Copy for UDateFormatField {}
impl ::core::clone::Clone for UDateFormatField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UDateFormatField {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDateFormatField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDateFormatField").field(&self.0).finish()
    }
}
impl FromIntoMemory for UDateFormatField {
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
pub struct UDateFormatStyle(pub i32);
pub const UDAT_FULL: UDateFormatStyle = UDateFormatStyle(0i32);
pub const UDAT_LONG: UDateFormatStyle = UDateFormatStyle(1i32);
pub const UDAT_MEDIUM: UDateFormatStyle = UDateFormatStyle(2i32);
pub const UDAT_SHORT: UDateFormatStyle = UDateFormatStyle(3i32);
pub const UDAT_DEFAULT: UDateFormatStyle = UDateFormatStyle(2i32);
pub const UDAT_RELATIVE: UDateFormatStyle = UDateFormatStyle(128i32);
pub const UDAT_FULL_RELATIVE: UDateFormatStyle = UDateFormatStyle(128i32);
pub const UDAT_LONG_RELATIVE: UDateFormatStyle = UDateFormatStyle(129i32);
pub const UDAT_MEDIUM_RELATIVE: UDateFormatStyle = UDateFormatStyle(130i32);
pub const UDAT_SHORT_RELATIVE: UDateFormatStyle = UDateFormatStyle(131i32);
pub const UDAT_NONE: UDateFormatStyle = UDateFormatStyle(-1i32);
pub const UDAT_PATTERN: UDateFormatStyle = UDateFormatStyle(-2i32);
impl ::core::marker::Copy for UDateFormatStyle {}
impl ::core::clone::Clone for UDateFormatStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UDateFormatStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDateFormatStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDateFormatStyle").field(&self.0).finish()
    }
}
impl FromIntoMemory for UDateFormatStyle {
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
pub struct UDateFormatSymbolType(pub i32);
pub const UDAT_ERAS: UDateFormatSymbolType = UDateFormatSymbolType(0i32);
pub const UDAT_MONTHS: UDateFormatSymbolType = UDateFormatSymbolType(1i32);
pub const UDAT_SHORT_MONTHS: UDateFormatSymbolType = UDateFormatSymbolType(2i32);
pub const UDAT_WEEKDAYS: UDateFormatSymbolType = UDateFormatSymbolType(3i32);
pub const UDAT_SHORT_WEEKDAYS: UDateFormatSymbolType = UDateFormatSymbolType(4i32);
pub const UDAT_AM_PMS: UDateFormatSymbolType = UDateFormatSymbolType(5i32);
pub const UDAT_LOCALIZED_CHARS: UDateFormatSymbolType = UDateFormatSymbolType(6i32);
pub const UDAT_ERA_NAMES: UDateFormatSymbolType = UDateFormatSymbolType(7i32);
pub const UDAT_NARROW_MONTHS: UDateFormatSymbolType = UDateFormatSymbolType(8i32);
pub const UDAT_NARROW_WEEKDAYS: UDateFormatSymbolType = UDateFormatSymbolType(9i32);
pub const UDAT_STANDALONE_MONTHS: UDateFormatSymbolType = UDateFormatSymbolType(10i32);
pub const UDAT_STANDALONE_SHORT_MONTHS: UDateFormatSymbolType = UDateFormatSymbolType(11i32);
pub const UDAT_STANDALONE_NARROW_MONTHS: UDateFormatSymbolType = UDateFormatSymbolType(12i32);
pub const UDAT_STANDALONE_WEEKDAYS: UDateFormatSymbolType = UDateFormatSymbolType(13i32);
pub const UDAT_STANDALONE_SHORT_WEEKDAYS: UDateFormatSymbolType = UDateFormatSymbolType(14i32);
pub const UDAT_STANDALONE_NARROW_WEEKDAYS: UDateFormatSymbolType = UDateFormatSymbolType(15i32);
pub const UDAT_QUARTERS: UDateFormatSymbolType = UDateFormatSymbolType(16i32);
pub const UDAT_SHORT_QUARTERS: UDateFormatSymbolType = UDateFormatSymbolType(17i32);
pub const UDAT_STANDALONE_QUARTERS: UDateFormatSymbolType = UDateFormatSymbolType(18i32);
pub const UDAT_STANDALONE_SHORT_QUARTERS: UDateFormatSymbolType = UDateFormatSymbolType(19i32);
pub const UDAT_SHORTER_WEEKDAYS: UDateFormatSymbolType = UDateFormatSymbolType(20i32);
pub const UDAT_STANDALONE_SHORTER_WEEKDAYS: UDateFormatSymbolType = UDateFormatSymbolType(21i32);
pub const UDAT_CYCLIC_YEARS_WIDE: UDateFormatSymbolType = UDateFormatSymbolType(22i32);
pub const UDAT_CYCLIC_YEARS_ABBREVIATED: UDateFormatSymbolType = UDateFormatSymbolType(23i32);
pub const UDAT_CYCLIC_YEARS_NARROW: UDateFormatSymbolType = UDateFormatSymbolType(24i32);
pub const UDAT_ZODIAC_NAMES_WIDE: UDateFormatSymbolType = UDateFormatSymbolType(25i32);
pub const UDAT_ZODIAC_NAMES_ABBREVIATED: UDateFormatSymbolType = UDateFormatSymbolType(26i32);
pub const UDAT_ZODIAC_NAMES_NARROW: UDateFormatSymbolType = UDateFormatSymbolType(27i32);
impl ::core::marker::Copy for UDateFormatSymbolType {}
impl ::core::clone::Clone for UDateFormatSymbolType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UDateFormatSymbolType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDateFormatSymbolType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDateFormatSymbolType")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UDateFormatSymbolType {
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
pub struct UDateFormatSymbols(pub u8);
pub struct UDateIntervalFormat(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UDateRelativeDateTimeFormatterStyle(pub i32);
pub const UDAT_STYLE_LONG: UDateRelativeDateTimeFormatterStyle =
    UDateRelativeDateTimeFormatterStyle(0i32);
pub const UDAT_STYLE_SHORT: UDateRelativeDateTimeFormatterStyle =
    UDateRelativeDateTimeFormatterStyle(1i32);
pub const UDAT_STYLE_NARROW: UDateRelativeDateTimeFormatterStyle =
    UDateRelativeDateTimeFormatterStyle(2i32);
impl ::core::marker::Copy for UDateRelativeDateTimeFormatterStyle {}
impl ::core::clone::Clone for UDateRelativeDateTimeFormatterStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UDateRelativeDateTimeFormatterStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDateRelativeDateTimeFormatterStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDateRelativeDateTimeFormatterStyle")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UDateRelativeDateTimeFormatterStyle {
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
pub struct UDateRelativeUnit(pub i32);
pub const UDAT_RELATIVE_SECONDS: UDateRelativeUnit = UDateRelativeUnit(0i32);
pub const UDAT_RELATIVE_MINUTES: UDateRelativeUnit = UDateRelativeUnit(1i32);
pub const UDAT_RELATIVE_HOURS: UDateRelativeUnit = UDateRelativeUnit(2i32);
pub const UDAT_RELATIVE_DAYS: UDateRelativeUnit = UDateRelativeUnit(3i32);
pub const UDAT_RELATIVE_WEEKS: UDateRelativeUnit = UDateRelativeUnit(4i32);
pub const UDAT_RELATIVE_MONTHS: UDateRelativeUnit = UDateRelativeUnit(5i32);
pub const UDAT_RELATIVE_YEARS: UDateRelativeUnit = UDateRelativeUnit(6i32);
pub const UDAT_RELATIVE_UNIT_COUNT: UDateRelativeUnit = UDateRelativeUnit(7i32);
impl ::core::marker::Copy for UDateRelativeUnit {}
impl ::core::clone::Clone for UDateRelativeUnit {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UDateRelativeUnit {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDateRelativeUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDateRelativeUnit").field(&self.0).finish()
    }
}
impl FromIntoMemory for UDateRelativeUnit {
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
pub struct UDateTimePGDisplayWidth(pub i32);
pub const UDATPG_WIDE: UDateTimePGDisplayWidth = UDateTimePGDisplayWidth(0i32);
pub const UDATPG_ABBREVIATED: UDateTimePGDisplayWidth = UDateTimePGDisplayWidth(1i32);
pub const UDATPG_NARROW: UDateTimePGDisplayWidth = UDateTimePGDisplayWidth(2i32);
impl ::core::marker::Copy for UDateTimePGDisplayWidth {}
impl ::core::clone::Clone for UDateTimePGDisplayWidth {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UDateTimePGDisplayWidth {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDateTimePGDisplayWidth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDateTimePGDisplayWidth")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UDateTimePGDisplayWidth {
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
pub struct UDateTimePatternConflict(pub i32);
pub const UDATPG_NO_CONFLICT: UDateTimePatternConflict = UDateTimePatternConflict(0i32);
pub const UDATPG_BASE_CONFLICT: UDateTimePatternConflict = UDateTimePatternConflict(1i32);
pub const UDATPG_CONFLICT: UDateTimePatternConflict = UDateTimePatternConflict(2i32);
impl ::core::marker::Copy for UDateTimePatternConflict {}
impl ::core::clone::Clone for UDateTimePatternConflict {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UDateTimePatternConflict {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDateTimePatternConflict {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDateTimePatternConflict")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UDateTimePatternConflict {
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
pub struct UDateTimePatternField(pub i32);
pub const UDATPG_ERA_FIELD: UDateTimePatternField = UDateTimePatternField(0i32);
pub const UDATPG_YEAR_FIELD: UDateTimePatternField = UDateTimePatternField(1i32);
pub const UDATPG_QUARTER_FIELD: UDateTimePatternField = UDateTimePatternField(2i32);
pub const UDATPG_MONTH_FIELD: UDateTimePatternField = UDateTimePatternField(3i32);
pub const UDATPG_WEEK_OF_YEAR_FIELD: UDateTimePatternField = UDateTimePatternField(4i32);
pub const UDATPG_WEEK_OF_MONTH_FIELD: UDateTimePatternField = UDateTimePatternField(5i32);
pub const UDATPG_WEEKDAY_FIELD: UDateTimePatternField = UDateTimePatternField(6i32);
pub const UDATPG_DAY_OF_YEAR_FIELD: UDateTimePatternField = UDateTimePatternField(7i32);
pub const UDATPG_DAY_OF_WEEK_IN_MONTH_FIELD: UDateTimePatternField = UDateTimePatternField(8i32);
pub const UDATPG_DAY_FIELD: UDateTimePatternField = UDateTimePatternField(9i32);
pub const UDATPG_DAYPERIOD_FIELD: UDateTimePatternField = UDateTimePatternField(10i32);
pub const UDATPG_HOUR_FIELD: UDateTimePatternField = UDateTimePatternField(11i32);
pub const UDATPG_MINUTE_FIELD: UDateTimePatternField = UDateTimePatternField(12i32);
pub const UDATPG_SECOND_FIELD: UDateTimePatternField = UDateTimePatternField(13i32);
pub const UDATPG_FRACTIONAL_SECOND_FIELD: UDateTimePatternField = UDateTimePatternField(14i32);
pub const UDATPG_ZONE_FIELD: UDateTimePatternField = UDateTimePatternField(15i32);
pub const UDATPG_FIELD_COUNT: UDateTimePatternField = UDateTimePatternField(16i32);
impl ::core::marker::Copy for UDateTimePatternField {}
impl ::core::clone::Clone for UDateTimePatternField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UDateTimePatternField {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDateTimePatternField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDateTimePatternField")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UDateTimePatternField {
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
pub struct UDateTimePatternMatchOptions(pub i32);
pub const UDATPG_MATCH_NO_OPTIONS: UDateTimePatternMatchOptions =
    UDateTimePatternMatchOptions(0i32);
pub const UDATPG_MATCH_HOUR_FIELD_LENGTH: UDateTimePatternMatchOptions =
    UDateTimePatternMatchOptions(2048i32);
pub const UDATPG_MATCH_ALL_FIELDS_LENGTH: UDateTimePatternMatchOptions =
    UDateTimePatternMatchOptions(65535i32);
impl ::core::marker::Copy for UDateTimePatternMatchOptions {}
impl ::core::clone::Clone for UDateTimePatternMatchOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UDateTimePatternMatchOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDateTimePatternMatchOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDateTimePatternMatchOptions")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UDateTimePatternMatchOptions {
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
pub struct UDateTimeScale(pub i32);
pub const UDTS_JAVA_TIME: UDateTimeScale = UDateTimeScale(0i32);
pub const UDTS_UNIX_TIME: UDateTimeScale = UDateTimeScale(1i32);
pub const UDTS_ICU4C_TIME: UDateTimeScale = UDateTimeScale(2i32);
pub const UDTS_WINDOWS_FILE_TIME: UDateTimeScale = UDateTimeScale(3i32);
pub const UDTS_DOTNET_DATE_TIME: UDateTimeScale = UDateTimeScale(4i32);
pub const UDTS_MAC_OLD_TIME: UDateTimeScale = UDateTimeScale(5i32);
pub const UDTS_MAC_TIME: UDateTimeScale = UDateTimeScale(6i32);
pub const UDTS_EXCEL_TIME: UDateTimeScale = UDateTimeScale(7i32);
pub const UDTS_DB2_TIME: UDateTimeScale = UDateTimeScale(8i32);
pub const UDTS_UNIX_MICROSECONDS_TIME: UDateTimeScale = UDateTimeScale(9i32);
impl ::core::marker::Copy for UDateTimeScale {}
impl ::core::clone::Clone for UDateTimeScale {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UDateTimeScale {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDateTimeScale {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDateTimeScale").field(&self.0).finish()
    }
}
impl FromIntoMemory for UDateTimeScale {
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
pub struct UDecompositionType(pub i32);
pub const U_DT_NONE: UDecompositionType = UDecompositionType(0i32);
pub const U_DT_CANONICAL: UDecompositionType = UDecompositionType(1i32);
pub const U_DT_COMPAT: UDecompositionType = UDecompositionType(2i32);
pub const U_DT_CIRCLE: UDecompositionType = UDecompositionType(3i32);
pub const U_DT_FINAL: UDecompositionType = UDecompositionType(4i32);
pub const U_DT_FONT: UDecompositionType = UDecompositionType(5i32);
pub const U_DT_FRACTION: UDecompositionType = UDecompositionType(6i32);
pub const U_DT_INITIAL: UDecompositionType = UDecompositionType(7i32);
pub const U_DT_ISOLATED: UDecompositionType = UDecompositionType(8i32);
pub const U_DT_MEDIAL: UDecompositionType = UDecompositionType(9i32);
pub const U_DT_NARROW: UDecompositionType = UDecompositionType(10i32);
pub const U_DT_NOBREAK: UDecompositionType = UDecompositionType(11i32);
pub const U_DT_SMALL: UDecompositionType = UDecompositionType(12i32);
pub const U_DT_SQUARE: UDecompositionType = UDecompositionType(13i32);
pub const U_DT_SUB: UDecompositionType = UDecompositionType(14i32);
pub const U_DT_SUPER: UDecompositionType = UDecompositionType(15i32);
pub const U_DT_VERTICAL: UDecompositionType = UDecompositionType(16i32);
pub const U_DT_WIDE: UDecompositionType = UDecompositionType(17i32);
impl ::core::marker::Copy for UDecompositionType {}
impl ::core::clone::Clone for UDecompositionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UDecompositionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDecompositionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDecompositionType").field(&self.0).finish()
    }
}
impl FromIntoMemory for UDecompositionType {
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
pub struct UDialectHandling(pub i32);
pub const ULDN_STANDARD_NAMES: UDialectHandling = UDialectHandling(0i32);
pub const ULDN_DIALECT_NAMES: UDialectHandling = UDialectHandling(1i32);
impl ::core::marker::Copy for UDialectHandling {}
impl ::core::clone::Clone for UDialectHandling {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UDialectHandling {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDialectHandling {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDialectHandling").field(&self.0).finish()
    }
}
impl FromIntoMemory for UDialectHandling {
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
pub struct UDisplayContext(pub i32);
pub const UDISPCTX_STANDARD_NAMES: UDisplayContext = UDisplayContext(0i32);
pub const UDISPCTX_DIALECT_NAMES: UDisplayContext = UDisplayContext(1i32);
pub const UDISPCTX_CAPITALIZATION_NONE: UDisplayContext = UDisplayContext(256i32);
pub const UDISPCTX_CAPITALIZATION_FOR_MIDDLE_OF_SENTENCE: UDisplayContext = UDisplayContext(257i32);
pub const UDISPCTX_CAPITALIZATION_FOR_BEGINNING_OF_SENTENCE: UDisplayContext =
    UDisplayContext(258i32);
pub const UDISPCTX_CAPITALIZATION_FOR_UI_LIST_OR_MENU: UDisplayContext = UDisplayContext(259i32);
pub const UDISPCTX_CAPITALIZATION_FOR_STANDALONE: UDisplayContext = UDisplayContext(260i32);
pub const UDISPCTX_LENGTH_FULL: UDisplayContext = UDisplayContext(512i32);
pub const UDISPCTX_LENGTH_SHORT: UDisplayContext = UDisplayContext(513i32);
pub const UDISPCTX_SUBSTITUTE: UDisplayContext = UDisplayContext(768i32);
pub const UDISPCTX_NO_SUBSTITUTE: UDisplayContext = UDisplayContext(769i32);
impl ::core::marker::Copy for UDisplayContext {}
impl ::core::clone::Clone for UDisplayContext {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UDisplayContext {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDisplayContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDisplayContext").field(&self.0).finish()
    }
}
impl FromIntoMemory for UDisplayContext {
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
pub struct UDisplayContextType(pub i32);
pub const UDISPCTX_TYPE_DIALECT_HANDLING: UDisplayContextType = UDisplayContextType(0i32);
pub const UDISPCTX_TYPE_CAPITALIZATION: UDisplayContextType = UDisplayContextType(1i32);
pub const UDISPCTX_TYPE_DISPLAY_LENGTH: UDisplayContextType = UDisplayContextType(2i32);
pub const UDISPCTX_TYPE_SUBSTITUTE_HANDLING: UDisplayContextType = UDisplayContextType(3i32);
impl ::core::marker::Copy for UDisplayContextType {}
impl ::core::clone::Clone for UDisplayContextType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UDisplayContextType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDisplayContextType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDisplayContextType").field(&self.0).finish()
    }
}
impl FromIntoMemory for UDisplayContextType {
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
pub struct UEastAsianWidth(pub i32);
pub const U_EA_NEUTRAL: UEastAsianWidth = UEastAsianWidth(0i32);
pub const U_EA_AMBIGUOUS: UEastAsianWidth = UEastAsianWidth(1i32);
pub const U_EA_HALFWIDTH: UEastAsianWidth = UEastAsianWidth(2i32);
pub const U_EA_FULLWIDTH: UEastAsianWidth = UEastAsianWidth(3i32);
pub const U_EA_NARROW: UEastAsianWidth = UEastAsianWidth(4i32);
pub const U_EA_WIDE: UEastAsianWidth = UEastAsianWidth(5i32);
impl ::core::marker::Copy for UEastAsianWidth {}
impl ::core::clone::Clone for UEastAsianWidth {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UEastAsianWidth {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UEastAsianWidth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UEastAsianWidth").field(&self.0).finish()
    }
}
impl FromIntoMemory for UEastAsianWidth {
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
pub type UEnumCharNamesFn = StdCallFnPtr<
    (
        MutPtr<::core::ffi::c_void>,
        i32,
        UCharNameChoice,
        PCSTR,
        i32,
    ),
    i8,
>;
pub struct UEnumeration(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UErrorCode(pub i32);
pub const U_USING_FALLBACK_WARNING: UErrorCode = UErrorCode(-128i32);
pub const U_ERROR_WARNING_START: UErrorCode = UErrorCode(-128i32);
pub const U_USING_DEFAULT_WARNING: UErrorCode = UErrorCode(-127i32);
pub const U_SAFECLONE_ALLOCATED_WARNING: UErrorCode = UErrorCode(-126i32);
pub const U_STATE_OLD_WARNING: UErrorCode = UErrorCode(-125i32);
pub const U_STRING_NOT_TERMINATED_WARNING: UErrorCode = UErrorCode(-124i32);
pub const U_SORT_KEY_TOO_SHORT_WARNING: UErrorCode = UErrorCode(-123i32);
pub const U_AMBIGUOUS_ALIAS_WARNING: UErrorCode = UErrorCode(-122i32);
pub const U_DIFFERENT_UCA_VERSION: UErrorCode = UErrorCode(-121i32);
pub const U_PLUGIN_CHANGED_LEVEL_WARNING: UErrorCode = UErrorCode(-120i32);
pub const U_ZERO_ERROR: UErrorCode = UErrorCode(0i32);
pub const U_ILLEGAL_ARGUMENT_ERROR: UErrorCode = UErrorCode(1i32);
pub const U_MISSING_RESOURCE_ERROR: UErrorCode = UErrorCode(2i32);
pub const U_INVALID_FORMAT_ERROR: UErrorCode = UErrorCode(3i32);
pub const U_FILE_ACCESS_ERROR: UErrorCode = UErrorCode(4i32);
pub const U_INTERNAL_PROGRAM_ERROR: UErrorCode = UErrorCode(5i32);
pub const U_MESSAGE_PARSE_ERROR: UErrorCode = UErrorCode(6i32);
pub const U_MEMORY_ALLOCATION_ERROR: UErrorCode = UErrorCode(7i32);
pub const U_INDEX_OUTOFBOUNDS_ERROR: UErrorCode = UErrorCode(8i32);
pub const U_PARSE_ERROR: UErrorCode = UErrorCode(9i32);
pub const U_INVALID_CHAR_FOUND: UErrorCode = UErrorCode(10i32);
pub const U_TRUNCATED_CHAR_FOUND: UErrorCode = UErrorCode(11i32);
pub const U_ILLEGAL_CHAR_FOUND: UErrorCode = UErrorCode(12i32);
pub const U_INVALID_TABLE_FORMAT: UErrorCode = UErrorCode(13i32);
pub const U_INVALID_TABLE_FILE: UErrorCode = UErrorCode(14i32);
pub const U_BUFFER_OVERFLOW_ERROR: UErrorCode = UErrorCode(15i32);
pub const U_UNSUPPORTED_ERROR: UErrorCode = UErrorCode(16i32);
pub const U_RESOURCE_TYPE_MISMATCH: UErrorCode = UErrorCode(17i32);
pub const U_ILLEGAL_ESCAPE_SEQUENCE: UErrorCode = UErrorCode(18i32);
pub const U_UNSUPPORTED_ESCAPE_SEQUENCE: UErrorCode = UErrorCode(19i32);
pub const U_NO_SPACE_AVAILABLE: UErrorCode = UErrorCode(20i32);
pub const U_CE_NOT_FOUND_ERROR: UErrorCode = UErrorCode(21i32);
pub const U_PRIMARY_TOO_LONG_ERROR: UErrorCode = UErrorCode(22i32);
pub const U_STATE_TOO_OLD_ERROR: UErrorCode = UErrorCode(23i32);
pub const U_TOO_MANY_ALIASES_ERROR: UErrorCode = UErrorCode(24i32);
pub const U_ENUM_OUT_OF_SYNC_ERROR: UErrorCode = UErrorCode(25i32);
pub const U_INVARIANT_CONVERSION_ERROR: UErrorCode = UErrorCode(26i32);
pub const U_INVALID_STATE_ERROR: UErrorCode = UErrorCode(27i32);
pub const U_COLLATOR_VERSION_MISMATCH: UErrorCode = UErrorCode(28i32);
pub const U_USELESS_COLLATOR_ERROR: UErrorCode = UErrorCode(29i32);
pub const U_NO_WRITE_PERMISSION: UErrorCode = UErrorCode(30i32);
pub const U_BAD_VARIABLE_DEFINITION: UErrorCode = UErrorCode(65536i32);
pub const U_PARSE_ERROR_START: UErrorCode = UErrorCode(65536i32);
pub const U_MALFORMED_RULE: UErrorCode = UErrorCode(65537i32);
pub const U_MALFORMED_SET: UErrorCode = UErrorCode(65538i32);
pub const U_MALFORMED_SYMBOL_REFERENCE: UErrorCode = UErrorCode(65539i32);
pub const U_MALFORMED_UNICODE_ESCAPE: UErrorCode = UErrorCode(65540i32);
pub const U_MALFORMED_VARIABLE_DEFINITION: UErrorCode = UErrorCode(65541i32);
pub const U_MALFORMED_VARIABLE_REFERENCE: UErrorCode = UErrorCode(65542i32);
pub const U_MISMATCHED_SEGMENT_DELIMITERS: UErrorCode = UErrorCode(65543i32);
pub const U_MISPLACED_ANCHOR_START: UErrorCode = UErrorCode(65544i32);
pub const U_MISPLACED_CURSOR_OFFSET: UErrorCode = UErrorCode(65545i32);
pub const U_MISPLACED_QUANTIFIER: UErrorCode = UErrorCode(65546i32);
pub const U_MISSING_OPERATOR: UErrorCode = UErrorCode(65547i32);
pub const U_MISSING_SEGMENT_CLOSE: UErrorCode = UErrorCode(65548i32);
pub const U_MULTIPLE_ANTE_CONTEXTS: UErrorCode = UErrorCode(65549i32);
pub const U_MULTIPLE_CURSORS: UErrorCode = UErrorCode(65550i32);
pub const U_MULTIPLE_POST_CONTEXTS: UErrorCode = UErrorCode(65551i32);
pub const U_TRAILING_BACKSLASH: UErrorCode = UErrorCode(65552i32);
pub const U_UNDEFINED_SEGMENT_REFERENCE: UErrorCode = UErrorCode(65553i32);
pub const U_UNDEFINED_VARIABLE: UErrorCode = UErrorCode(65554i32);
pub const U_UNQUOTED_SPECIAL: UErrorCode = UErrorCode(65555i32);
pub const U_UNTERMINATED_QUOTE: UErrorCode = UErrorCode(65556i32);
pub const U_RULE_MASK_ERROR: UErrorCode = UErrorCode(65557i32);
pub const U_MISPLACED_COMPOUND_FILTER: UErrorCode = UErrorCode(65558i32);
pub const U_MULTIPLE_COMPOUND_FILTERS: UErrorCode = UErrorCode(65559i32);
pub const U_INVALID_RBT_SYNTAX: UErrorCode = UErrorCode(65560i32);
pub const U_INVALID_PROPERTY_PATTERN: UErrorCode = UErrorCode(65561i32);
pub const U_MALFORMED_PRAGMA: UErrorCode = UErrorCode(65562i32);
pub const U_UNCLOSED_SEGMENT: UErrorCode = UErrorCode(65563i32);
pub const U_ILLEGAL_CHAR_IN_SEGMENT: UErrorCode = UErrorCode(65564i32);
pub const U_VARIABLE_RANGE_EXHAUSTED: UErrorCode = UErrorCode(65565i32);
pub const U_VARIABLE_RANGE_OVERLAP: UErrorCode = UErrorCode(65566i32);
pub const U_ILLEGAL_CHARACTER: UErrorCode = UErrorCode(65567i32);
pub const U_INTERNAL_TRANSLITERATOR_ERROR: UErrorCode = UErrorCode(65568i32);
pub const U_INVALID_ID: UErrorCode = UErrorCode(65569i32);
pub const U_INVALID_FUNCTION: UErrorCode = UErrorCode(65570i32);
pub const U_UNEXPECTED_TOKEN: UErrorCode = UErrorCode(65792i32);
pub const U_FMT_PARSE_ERROR_START: UErrorCode = UErrorCode(65792i32);
pub const U_MULTIPLE_DECIMAL_SEPARATORS: UErrorCode = UErrorCode(65793i32);
pub const U_MULTIPLE_DECIMAL_SEPERATORS: UErrorCode = UErrorCode(65793i32);
pub const U_MULTIPLE_EXPONENTIAL_SYMBOLS: UErrorCode = UErrorCode(65794i32);
pub const U_MALFORMED_EXPONENTIAL_PATTERN: UErrorCode = UErrorCode(65795i32);
pub const U_MULTIPLE_PERCENT_SYMBOLS: UErrorCode = UErrorCode(65796i32);
pub const U_MULTIPLE_PERMILL_SYMBOLS: UErrorCode = UErrorCode(65797i32);
pub const U_MULTIPLE_PAD_SPECIFIERS: UErrorCode = UErrorCode(65798i32);
pub const U_PATTERN_SYNTAX_ERROR: UErrorCode = UErrorCode(65799i32);
pub const U_ILLEGAL_PAD_POSITION: UErrorCode = UErrorCode(65800i32);
pub const U_UNMATCHED_BRACES: UErrorCode = UErrorCode(65801i32);
pub const U_UNSUPPORTED_PROPERTY: UErrorCode = UErrorCode(65802i32);
pub const U_UNSUPPORTED_ATTRIBUTE: UErrorCode = UErrorCode(65803i32);
pub const U_ARGUMENT_TYPE_MISMATCH: UErrorCode = UErrorCode(65804i32);
pub const U_DUPLICATE_KEYWORD: UErrorCode = UErrorCode(65805i32);
pub const U_UNDEFINED_KEYWORD: UErrorCode = UErrorCode(65806i32);
pub const U_DEFAULT_KEYWORD_MISSING: UErrorCode = UErrorCode(65807i32);
pub const U_DECIMAL_NUMBER_SYNTAX_ERROR: UErrorCode = UErrorCode(65808i32);
pub const U_FORMAT_INEXACT_ERROR: UErrorCode = UErrorCode(65809i32);
pub const U_NUMBER_ARG_OUTOFBOUNDS_ERROR: UErrorCode = UErrorCode(65810i32);
pub const U_NUMBER_SKELETON_SYNTAX_ERROR: UErrorCode = UErrorCode(65811i32);
pub const U_BRK_INTERNAL_ERROR: UErrorCode = UErrorCode(66048i32);
pub const U_BRK_ERROR_START: UErrorCode = UErrorCode(66048i32);
pub const U_BRK_HEX_DIGITS_EXPECTED: UErrorCode = UErrorCode(66049i32);
pub const U_BRK_SEMICOLON_EXPECTED: UErrorCode = UErrorCode(66050i32);
pub const U_BRK_RULE_SYNTAX: UErrorCode = UErrorCode(66051i32);
pub const U_BRK_UNCLOSED_SET: UErrorCode = UErrorCode(66052i32);
pub const U_BRK_ASSIGN_ERROR: UErrorCode = UErrorCode(66053i32);
pub const U_BRK_VARIABLE_REDFINITION: UErrorCode = UErrorCode(66054i32);
pub const U_BRK_MISMATCHED_PAREN: UErrorCode = UErrorCode(66055i32);
pub const U_BRK_NEW_LINE_IN_QUOTED_STRING: UErrorCode = UErrorCode(66056i32);
pub const U_BRK_UNDEFINED_VARIABLE: UErrorCode = UErrorCode(66057i32);
pub const U_BRK_INIT_ERROR: UErrorCode = UErrorCode(66058i32);
pub const U_BRK_RULE_EMPTY_SET: UErrorCode = UErrorCode(66059i32);
pub const U_BRK_UNRECOGNIZED_OPTION: UErrorCode = UErrorCode(66060i32);
pub const U_BRK_MALFORMED_RULE_TAG: UErrorCode = UErrorCode(66061i32);
pub const U_REGEX_INTERNAL_ERROR: UErrorCode = UErrorCode(66304i32);
pub const U_REGEX_ERROR_START: UErrorCode = UErrorCode(66304i32);
pub const U_REGEX_RULE_SYNTAX: UErrorCode = UErrorCode(66305i32);
pub const U_REGEX_INVALID_STATE: UErrorCode = UErrorCode(66306i32);
pub const U_REGEX_BAD_ESCAPE_SEQUENCE: UErrorCode = UErrorCode(66307i32);
pub const U_REGEX_PROPERTY_SYNTAX: UErrorCode = UErrorCode(66308i32);
pub const U_REGEX_UNIMPLEMENTED: UErrorCode = UErrorCode(66309i32);
pub const U_REGEX_MISMATCHED_PAREN: UErrorCode = UErrorCode(66310i32);
pub const U_REGEX_NUMBER_TOO_BIG: UErrorCode = UErrorCode(66311i32);
pub const U_REGEX_BAD_INTERVAL: UErrorCode = UErrorCode(66312i32);
pub const U_REGEX_MAX_LT_MIN: UErrorCode = UErrorCode(66313i32);
pub const U_REGEX_INVALID_BACK_REF: UErrorCode = UErrorCode(66314i32);
pub const U_REGEX_INVALID_FLAG: UErrorCode = UErrorCode(66315i32);
pub const U_REGEX_LOOK_BEHIND_LIMIT: UErrorCode = UErrorCode(66316i32);
pub const U_REGEX_SET_CONTAINS_STRING: UErrorCode = UErrorCode(66317i32);
pub const U_REGEX_MISSING_CLOSE_BRACKET: UErrorCode = UErrorCode(66319i32);
pub const U_REGEX_INVALID_RANGE: UErrorCode = UErrorCode(66320i32);
pub const U_REGEX_STACK_OVERFLOW: UErrorCode = UErrorCode(66321i32);
pub const U_REGEX_TIME_OUT: UErrorCode = UErrorCode(66322i32);
pub const U_REGEX_STOPPED_BY_CALLER: UErrorCode = UErrorCode(66323i32);
pub const U_REGEX_PATTERN_TOO_BIG: UErrorCode = UErrorCode(66324i32);
pub const U_REGEX_INVALID_CAPTURE_GROUP_NAME: UErrorCode = UErrorCode(66325i32);
pub const U_IDNA_PROHIBITED_ERROR: UErrorCode = UErrorCode(66560i32);
pub const U_IDNA_ERROR_START: UErrorCode = UErrorCode(66560i32);
pub const U_IDNA_UNASSIGNED_ERROR: UErrorCode = UErrorCode(66561i32);
pub const U_IDNA_CHECK_BIDI_ERROR: UErrorCode = UErrorCode(66562i32);
pub const U_IDNA_STD3_ASCII_RULES_ERROR: UErrorCode = UErrorCode(66563i32);
pub const U_IDNA_ACE_PREFIX_ERROR: UErrorCode = UErrorCode(66564i32);
pub const U_IDNA_VERIFICATION_ERROR: UErrorCode = UErrorCode(66565i32);
pub const U_IDNA_LABEL_TOO_LONG_ERROR: UErrorCode = UErrorCode(66566i32);
pub const U_IDNA_ZERO_LENGTH_LABEL_ERROR: UErrorCode = UErrorCode(66567i32);
pub const U_IDNA_DOMAIN_NAME_TOO_LONG_ERROR: UErrorCode = UErrorCode(66568i32);
pub const U_STRINGPREP_PROHIBITED_ERROR: UErrorCode = UErrorCode(66560i32);
pub const U_STRINGPREP_UNASSIGNED_ERROR: UErrorCode = UErrorCode(66561i32);
pub const U_STRINGPREP_CHECK_BIDI_ERROR: UErrorCode = UErrorCode(66562i32);
pub const U_PLUGIN_ERROR_START: UErrorCode = UErrorCode(66816i32);
pub const U_PLUGIN_TOO_HIGH: UErrorCode = UErrorCode(66816i32);
pub const U_PLUGIN_DIDNT_SET_LEVEL: UErrorCode = UErrorCode(66817i32);
impl ::core::marker::Copy for UErrorCode {}
impl ::core::clone::Clone for UErrorCode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UErrorCode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UErrorCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UErrorCode").field(&self.0).finish()
    }
}
impl FromIntoMemory for UErrorCode {
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
pub struct UFieldCategory(pub i32);
pub const UFIELD_CATEGORY_UNDEFINED: UFieldCategory = UFieldCategory(0i32);
pub const UFIELD_CATEGORY_DATE: UFieldCategory = UFieldCategory(1i32);
pub const UFIELD_CATEGORY_NUMBER: UFieldCategory = UFieldCategory(2i32);
pub const UFIELD_CATEGORY_LIST: UFieldCategory = UFieldCategory(3i32);
pub const UFIELD_CATEGORY_RELATIVE_DATETIME: UFieldCategory = UFieldCategory(4i32);
pub const UFIELD_CATEGORY_DATE_INTERVAL: UFieldCategory = UFieldCategory(5i32);
pub const UFIELD_CATEGORY_LIST_SPAN: UFieldCategory = UFieldCategory(4099i32);
pub const UFIELD_CATEGORY_DATE_INTERVAL_SPAN: UFieldCategory = UFieldCategory(4101i32);
impl ::core::marker::Copy for UFieldCategory {}
impl ::core::clone::Clone for UFieldCategory {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UFieldCategory {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UFieldCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UFieldCategory").field(&self.0).finish()
    }
}
impl FromIntoMemory for UFieldCategory {
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
pub struct UFieldPosition {
    pub field: i32,
    pub beginIndex: i32,
    pub endIndex: i32,
}
impl ::core::marker::Copy for UFieldPosition {}
impl ::core::clone::Clone for UFieldPosition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UFieldPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UFieldPosition")
            .field("field", &self.field)
            .field("beginIndex", &self.beginIndex)
            .field("endIndex", &self.endIndex)
            .finish()
    }
}
impl ::core::cmp::PartialEq for UFieldPosition {
    fn eq(&self, other: &Self) -> bool {
        self.field == other.field
            && self.beginIndex == other.beginIndex
            && self.endIndex == other.endIndex
    }
}
impl ::core::cmp::Eq for UFieldPosition {}
impl FromIntoMemory for UFieldPosition {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12u32 as usize);
        let f_field = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_beginIndex = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_endIndex = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            field: f_field,
            beginIndex: f_beginIndex,
            endIndex: f_endIndex,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12u32 as usize);
        FromIntoMemory::into_bytes(self.field, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.beginIndex, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.endIndex, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12u32 as usize
    }
}
pub struct UFieldPositionIterator(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UFormattableType(pub i32);
pub const UFMT_DATE: UFormattableType = UFormattableType(0i32);
pub const UFMT_DOUBLE: UFormattableType = UFormattableType(1i32);
pub const UFMT_LONG: UFormattableType = UFormattableType(2i32);
pub const UFMT_STRING: UFormattableType = UFormattableType(3i32);
pub const UFMT_ARRAY: UFormattableType = UFormattableType(4i32);
pub const UFMT_INT64: UFormattableType = UFormattableType(5i32);
pub const UFMT_OBJECT: UFormattableType = UFormattableType(6i32);
impl ::core::marker::Copy for UFormattableType {}
impl ::core::clone::Clone for UFormattableType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UFormattableType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UFormattableType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UFormattableType").field(&self.0).finish()
    }
}
impl FromIntoMemory for UFormattableType {
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
pub struct UFormattedDateInterval(pub u8);
pub struct UFormattedList(pub u8);
pub struct UFormattedNumber(pub u8);
pub struct UFormattedNumberRange(pub u8);
pub struct UFormattedRelativeDateTime(pub u8);
pub struct UFormattedValue(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UGender(pub i32);
pub const UGENDER_MALE: UGender = UGender(0i32);
pub const UGENDER_FEMALE: UGender = UGender(1i32);
pub const UGENDER_OTHER: UGender = UGender(2i32);
impl ::core::marker::Copy for UGender {}
impl ::core::clone::Clone for UGender {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UGender {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UGender {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UGender").field(&self.0).finish()
    }
}
impl FromIntoMemory for UGender {
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
pub struct UGenderInfo(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UGraphemeClusterBreak(pub i32);
pub const U_GCB_OTHER: UGraphemeClusterBreak = UGraphemeClusterBreak(0i32);
pub const U_GCB_CONTROL: UGraphemeClusterBreak = UGraphemeClusterBreak(1i32);
pub const U_GCB_CR: UGraphemeClusterBreak = UGraphemeClusterBreak(2i32);
pub const U_GCB_EXTEND: UGraphemeClusterBreak = UGraphemeClusterBreak(3i32);
pub const U_GCB_L: UGraphemeClusterBreak = UGraphemeClusterBreak(4i32);
pub const U_GCB_LF: UGraphemeClusterBreak = UGraphemeClusterBreak(5i32);
pub const U_GCB_LV: UGraphemeClusterBreak = UGraphemeClusterBreak(6i32);
pub const U_GCB_LVT: UGraphemeClusterBreak = UGraphemeClusterBreak(7i32);
pub const U_GCB_T: UGraphemeClusterBreak = UGraphemeClusterBreak(8i32);
pub const U_GCB_V: UGraphemeClusterBreak = UGraphemeClusterBreak(9i32);
pub const U_GCB_SPACING_MARK: UGraphemeClusterBreak = UGraphemeClusterBreak(10i32);
pub const U_GCB_PREPEND: UGraphemeClusterBreak = UGraphemeClusterBreak(11i32);
pub const U_GCB_REGIONAL_INDICATOR: UGraphemeClusterBreak = UGraphemeClusterBreak(12i32);
pub const U_GCB_E_BASE: UGraphemeClusterBreak = UGraphemeClusterBreak(13i32);
pub const U_GCB_E_BASE_GAZ: UGraphemeClusterBreak = UGraphemeClusterBreak(14i32);
pub const U_GCB_E_MODIFIER: UGraphemeClusterBreak = UGraphemeClusterBreak(15i32);
pub const U_GCB_GLUE_AFTER_ZWJ: UGraphemeClusterBreak = UGraphemeClusterBreak(16i32);
pub const U_GCB_ZWJ: UGraphemeClusterBreak = UGraphemeClusterBreak(17i32);
impl ::core::marker::Copy for UGraphemeClusterBreak {}
impl ::core::clone::Clone for UGraphemeClusterBreak {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UGraphemeClusterBreak {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UGraphemeClusterBreak {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UGraphemeClusterBreak")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UGraphemeClusterBreak {
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
pub struct UHangulSyllableType(pub i32);
pub const U_HST_NOT_APPLICABLE: UHangulSyllableType = UHangulSyllableType(0i32);
pub const U_HST_LEADING_JAMO: UHangulSyllableType = UHangulSyllableType(1i32);
pub const U_HST_VOWEL_JAMO: UHangulSyllableType = UHangulSyllableType(2i32);
pub const U_HST_TRAILING_JAMO: UHangulSyllableType = UHangulSyllableType(3i32);
pub const U_HST_LV_SYLLABLE: UHangulSyllableType = UHangulSyllableType(4i32);
pub const U_HST_LVT_SYLLABLE: UHangulSyllableType = UHangulSyllableType(5i32);
impl ::core::marker::Copy for UHangulSyllableType {}
impl ::core::clone::Clone for UHangulSyllableType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UHangulSyllableType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UHangulSyllableType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UHangulSyllableType").field(&self.0).finish()
    }
}
impl FromIntoMemory for UHangulSyllableType {
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
pub struct UHashtable(pub u8);
pub struct UIDNA(pub u8);
pub struct UIDNAInfo {
    pub size: i16,
    pub isTransitionalDifferent: i8,
    pub reservedB3: i8,
    pub errors: u32,
    pub reservedI2: i32,
    pub reservedI3: i32,
}
impl ::core::marker::Copy for UIDNAInfo {}
impl ::core::clone::Clone for UIDNAInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UIDNAInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UIDNAInfo")
            .field("size", &self.size)
            .field("isTransitionalDifferent", &self.isTransitionalDifferent)
            .field("reservedB3", &self.reservedB3)
            .field("errors", &self.errors)
            .field("reservedI2", &self.reservedI2)
            .field("reservedI3", &self.reservedI3)
            .finish()
    }
}
impl ::core::cmp::PartialEq for UIDNAInfo {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
            && self.isTransitionalDifferent == other.isTransitionalDifferent
            && self.reservedB3 == other.reservedB3
            && self.errors == other.errors
            && self.reservedI2 == other.reservedI2
            && self.reservedI3 == other.reservedI3
    }
}
impl ::core::cmp::Eq for UIDNAInfo {}
impl FromIntoMemory for UIDNAInfo {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16u32 as usize);
        let f_size = <i16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_isTransitionalDifferent = <i8 as FromIntoMemory>::from_bytes(&from[2..2 + 1]);
        let f_reservedB3 = <i8 as FromIntoMemory>::from_bytes(&from[3..3 + 1]);
        let f_errors = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_reservedI2 = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_reservedI3 = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            size: f_size,
            isTransitionalDifferent: f_isTransitionalDifferent,
            reservedB3: f_reservedB3,
            errors: f_errors,
            reservedI2: f_reservedI2,
            reservedI3: f_reservedI3,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16u32 as usize);
        FromIntoMemory::into_bytes(self.size, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.isTransitionalDifferent, &mut into[2..2 + 1]);
        FromIntoMemory::into_bytes(self.reservedB3, &mut into[3..3 + 1]);
        FromIntoMemory::into_bytes(self.errors, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.reservedI2, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.reservedI3, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16u32 as usize
    }
}
pub const UIDNA_CHECK_BIDI: i32 = 4i32;
pub const UIDNA_CHECK_CONTEXTJ: i32 = 8i32;
pub const UIDNA_CHECK_CONTEXTO: i32 = 64i32;
pub const UIDNA_DEFAULT: i32 = 0i32;
pub const UIDNA_ERROR_BIDI: i32 = 2048i32;
pub const UIDNA_ERROR_CONTEXTJ: i32 = 4096i32;
pub const UIDNA_ERROR_CONTEXTO_DIGITS: i32 = 16384i32;
pub const UIDNA_ERROR_CONTEXTO_PUNCTUATION: i32 = 8192i32;
pub const UIDNA_ERROR_DISALLOWED: i32 = 128i32;
pub const UIDNA_ERROR_DOMAIN_NAME_TOO_LONG: i32 = 4i32;
pub const UIDNA_ERROR_EMPTY_LABEL: i32 = 1i32;
pub const UIDNA_ERROR_HYPHEN_3_4: i32 = 32i32;
pub const UIDNA_ERROR_INVALID_ACE_LABEL: i32 = 1024i32;
pub const UIDNA_ERROR_LABEL_HAS_DOT: i32 = 512i32;
pub const UIDNA_ERROR_LABEL_TOO_LONG: i32 = 2i32;
pub const UIDNA_ERROR_LEADING_COMBINING_MARK: i32 = 64i32;
pub const UIDNA_ERROR_LEADING_HYPHEN: i32 = 8i32;
pub const UIDNA_ERROR_PUNYCODE: i32 = 256i32;
pub const UIDNA_ERROR_TRAILING_HYPHEN: i32 = 16i32;
pub const UIDNA_NONTRANSITIONAL_TO_ASCII: i32 = 16i32;
pub const UIDNA_NONTRANSITIONAL_TO_UNICODE: i32 = 32i32;
pub const UIDNA_USE_STD3_RULES: i32 = 2i32;
pub type UILANGUAGE_ENUMPROCA = StdCallFnPtr<(PCSTR, PtrDiffRepr), super::Foundation::BOOL>;
pub type UILANGUAGE_ENUMPROCW = StdCallFnPtr<(PCWSTR, PtrDiffRepr), super::Foundation::BOOL>;
pub const UITER_UNKNOWN_INDEX: i32 = -2i32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UIndicPositionalCategory(pub i32);
pub const U_INPC_NA: UIndicPositionalCategory = UIndicPositionalCategory(0i32);
pub const U_INPC_BOTTOM: UIndicPositionalCategory = UIndicPositionalCategory(1i32);
pub const U_INPC_BOTTOM_AND_LEFT: UIndicPositionalCategory = UIndicPositionalCategory(2i32);
pub const U_INPC_BOTTOM_AND_RIGHT: UIndicPositionalCategory = UIndicPositionalCategory(3i32);
pub const U_INPC_LEFT: UIndicPositionalCategory = UIndicPositionalCategory(4i32);
pub const U_INPC_LEFT_AND_RIGHT: UIndicPositionalCategory = UIndicPositionalCategory(5i32);
pub const U_INPC_OVERSTRUCK: UIndicPositionalCategory = UIndicPositionalCategory(6i32);
pub const U_INPC_RIGHT: UIndicPositionalCategory = UIndicPositionalCategory(7i32);
pub const U_INPC_TOP: UIndicPositionalCategory = UIndicPositionalCategory(8i32);
pub const U_INPC_TOP_AND_BOTTOM: UIndicPositionalCategory = UIndicPositionalCategory(9i32);
pub const U_INPC_TOP_AND_BOTTOM_AND_RIGHT: UIndicPositionalCategory =
    UIndicPositionalCategory(10i32);
pub const U_INPC_TOP_AND_LEFT: UIndicPositionalCategory = UIndicPositionalCategory(11i32);
pub const U_INPC_TOP_AND_LEFT_AND_RIGHT: UIndicPositionalCategory = UIndicPositionalCategory(12i32);
pub const U_INPC_TOP_AND_RIGHT: UIndicPositionalCategory = UIndicPositionalCategory(13i32);
pub const U_INPC_VISUAL_ORDER_LEFT: UIndicPositionalCategory = UIndicPositionalCategory(14i32);
pub const U_INPC_TOP_AND_BOTTOM_AND_LEFT: UIndicPositionalCategory =
    UIndicPositionalCategory(15i32);
impl ::core::marker::Copy for UIndicPositionalCategory {}
impl ::core::clone::Clone for UIndicPositionalCategory {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UIndicPositionalCategory {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UIndicPositionalCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UIndicPositionalCategory")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UIndicPositionalCategory {
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
pub struct UIndicSyllabicCategory(pub i32);
pub const U_INSC_OTHER: UIndicSyllabicCategory = UIndicSyllabicCategory(0i32);
pub const U_INSC_AVAGRAHA: UIndicSyllabicCategory = UIndicSyllabicCategory(1i32);
pub const U_INSC_BINDU: UIndicSyllabicCategory = UIndicSyllabicCategory(2i32);
pub const U_INSC_BRAHMI_JOINING_NUMBER: UIndicSyllabicCategory = UIndicSyllabicCategory(3i32);
pub const U_INSC_CANTILLATION_MARK: UIndicSyllabicCategory = UIndicSyllabicCategory(4i32);
pub const U_INSC_CONSONANT: UIndicSyllabicCategory = UIndicSyllabicCategory(5i32);
pub const U_INSC_CONSONANT_DEAD: UIndicSyllabicCategory = UIndicSyllabicCategory(6i32);
pub const U_INSC_CONSONANT_FINAL: UIndicSyllabicCategory = UIndicSyllabicCategory(7i32);
pub const U_INSC_CONSONANT_HEAD_LETTER: UIndicSyllabicCategory = UIndicSyllabicCategory(8i32);
pub const U_INSC_CONSONANT_INITIAL_POSTFIXED: UIndicSyllabicCategory = UIndicSyllabicCategory(9i32);
pub const U_INSC_CONSONANT_KILLER: UIndicSyllabicCategory = UIndicSyllabicCategory(10i32);
pub const U_INSC_CONSONANT_MEDIAL: UIndicSyllabicCategory = UIndicSyllabicCategory(11i32);
pub const U_INSC_CONSONANT_PLACEHOLDER: UIndicSyllabicCategory = UIndicSyllabicCategory(12i32);
pub const U_INSC_CONSONANT_PRECEDING_REPHA: UIndicSyllabicCategory = UIndicSyllabicCategory(13i32);
pub const U_INSC_CONSONANT_PREFIXED: UIndicSyllabicCategory = UIndicSyllabicCategory(14i32);
pub const U_INSC_CONSONANT_SUBJOINED: UIndicSyllabicCategory = UIndicSyllabicCategory(15i32);
pub const U_INSC_CONSONANT_SUCCEEDING_REPHA: UIndicSyllabicCategory = UIndicSyllabicCategory(16i32);
pub const U_INSC_CONSONANT_WITH_STACKER: UIndicSyllabicCategory = UIndicSyllabicCategory(17i32);
pub const U_INSC_GEMINATION_MARK: UIndicSyllabicCategory = UIndicSyllabicCategory(18i32);
pub const U_INSC_INVISIBLE_STACKER: UIndicSyllabicCategory = UIndicSyllabicCategory(19i32);
pub const U_INSC_JOINER: UIndicSyllabicCategory = UIndicSyllabicCategory(20i32);
pub const U_INSC_MODIFYING_LETTER: UIndicSyllabicCategory = UIndicSyllabicCategory(21i32);
pub const U_INSC_NON_JOINER: UIndicSyllabicCategory = UIndicSyllabicCategory(22i32);
pub const U_INSC_NUKTA: UIndicSyllabicCategory = UIndicSyllabicCategory(23i32);
pub const U_INSC_NUMBER: UIndicSyllabicCategory = UIndicSyllabicCategory(24i32);
pub const U_INSC_NUMBER_JOINER: UIndicSyllabicCategory = UIndicSyllabicCategory(25i32);
pub const U_INSC_PURE_KILLER: UIndicSyllabicCategory = UIndicSyllabicCategory(26i32);
pub const U_INSC_REGISTER_SHIFTER: UIndicSyllabicCategory = UIndicSyllabicCategory(27i32);
pub const U_INSC_SYLLABLE_MODIFIER: UIndicSyllabicCategory = UIndicSyllabicCategory(28i32);
pub const U_INSC_TONE_LETTER: UIndicSyllabicCategory = UIndicSyllabicCategory(29i32);
pub const U_INSC_TONE_MARK: UIndicSyllabicCategory = UIndicSyllabicCategory(30i32);
pub const U_INSC_VIRAMA: UIndicSyllabicCategory = UIndicSyllabicCategory(31i32);
pub const U_INSC_VISARGA: UIndicSyllabicCategory = UIndicSyllabicCategory(32i32);
pub const U_INSC_VOWEL: UIndicSyllabicCategory = UIndicSyllabicCategory(33i32);
pub const U_INSC_VOWEL_DEPENDENT: UIndicSyllabicCategory = UIndicSyllabicCategory(34i32);
pub const U_INSC_VOWEL_INDEPENDENT: UIndicSyllabicCategory = UIndicSyllabicCategory(35i32);
impl ::core::marker::Copy for UIndicSyllabicCategory {}
impl ::core::clone::Clone for UIndicSyllabicCategory {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UIndicSyllabicCategory {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UIndicSyllabicCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UIndicSyllabicCategory")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UIndicSyllabicCategory {
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
pub struct UJoiningGroup(pub i32);
pub const U_JG_NO_JOINING_GROUP: UJoiningGroup = UJoiningGroup(0i32);
pub const U_JG_AIN: UJoiningGroup = UJoiningGroup(1i32);
pub const U_JG_ALAPH: UJoiningGroup = UJoiningGroup(2i32);
pub const U_JG_ALEF: UJoiningGroup = UJoiningGroup(3i32);
pub const U_JG_BEH: UJoiningGroup = UJoiningGroup(4i32);
pub const U_JG_BETH: UJoiningGroup = UJoiningGroup(5i32);
pub const U_JG_DAL: UJoiningGroup = UJoiningGroup(6i32);
pub const U_JG_DALATH_RISH: UJoiningGroup = UJoiningGroup(7i32);
pub const U_JG_E: UJoiningGroup = UJoiningGroup(8i32);
pub const U_JG_FEH: UJoiningGroup = UJoiningGroup(9i32);
pub const U_JG_FINAL_SEMKATH: UJoiningGroup = UJoiningGroup(10i32);
pub const U_JG_GAF: UJoiningGroup = UJoiningGroup(11i32);
pub const U_JG_GAMAL: UJoiningGroup = UJoiningGroup(12i32);
pub const U_JG_HAH: UJoiningGroup = UJoiningGroup(13i32);
pub const U_JG_TEH_MARBUTA_GOAL: UJoiningGroup = UJoiningGroup(14i32);
pub const U_JG_HAMZA_ON_HEH_GOAL: UJoiningGroup = UJoiningGroup(14i32);
pub const U_JG_HE: UJoiningGroup = UJoiningGroup(15i32);
pub const U_JG_HEH: UJoiningGroup = UJoiningGroup(16i32);
pub const U_JG_HEH_GOAL: UJoiningGroup = UJoiningGroup(17i32);
pub const U_JG_HETH: UJoiningGroup = UJoiningGroup(18i32);
pub const U_JG_KAF: UJoiningGroup = UJoiningGroup(19i32);
pub const U_JG_KAPH: UJoiningGroup = UJoiningGroup(20i32);
pub const U_JG_KNOTTED_HEH: UJoiningGroup = UJoiningGroup(21i32);
pub const U_JG_LAM: UJoiningGroup = UJoiningGroup(22i32);
pub const U_JG_LAMADH: UJoiningGroup = UJoiningGroup(23i32);
pub const U_JG_MEEM: UJoiningGroup = UJoiningGroup(24i32);
pub const U_JG_MIM: UJoiningGroup = UJoiningGroup(25i32);
pub const U_JG_NOON: UJoiningGroup = UJoiningGroup(26i32);
pub const U_JG_NUN: UJoiningGroup = UJoiningGroup(27i32);
pub const U_JG_PE: UJoiningGroup = UJoiningGroup(28i32);
pub const U_JG_QAF: UJoiningGroup = UJoiningGroup(29i32);
pub const U_JG_QAPH: UJoiningGroup = UJoiningGroup(30i32);
pub const U_JG_REH: UJoiningGroup = UJoiningGroup(31i32);
pub const U_JG_REVERSED_PE: UJoiningGroup = UJoiningGroup(32i32);
pub const U_JG_SAD: UJoiningGroup = UJoiningGroup(33i32);
pub const U_JG_SADHE: UJoiningGroup = UJoiningGroup(34i32);
pub const U_JG_SEEN: UJoiningGroup = UJoiningGroup(35i32);
pub const U_JG_SEMKATH: UJoiningGroup = UJoiningGroup(36i32);
pub const U_JG_SHIN: UJoiningGroup = UJoiningGroup(37i32);
pub const U_JG_SWASH_KAF: UJoiningGroup = UJoiningGroup(38i32);
pub const U_JG_SYRIAC_WAW: UJoiningGroup = UJoiningGroup(39i32);
pub const U_JG_TAH: UJoiningGroup = UJoiningGroup(40i32);
pub const U_JG_TAW: UJoiningGroup = UJoiningGroup(41i32);
pub const U_JG_TEH_MARBUTA: UJoiningGroup = UJoiningGroup(42i32);
pub const U_JG_TETH: UJoiningGroup = UJoiningGroup(43i32);
pub const U_JG_WAW: UJoiningGroup = UJoiningGroup(44i32);
pub const U_JG_YEH: UJoiningGroup = UJoiningGroup(45i32);
pub const U_JG_YEH_BARREE: UJoiningGroup = UJoiningGroup(46i32);
pub const U_JG_YEH_WITH_TAIL: UJoiningGroup = UJoiningGroup(47i32);
pub const U_JG_YUDH: UJoiningGroup = UJoiningGroup(48i32);
pub const U_JG_YUDH_HE: UJoiningGroup = UJoiningGroup(49i32);
pub const U_JG_ZAIN: UJoiningGroup = UJoiningGroup(50i32);
pub const U_JG_FE: UJoiningGroup = UJoiningGroup(51i32);
pub const U_JG_KHAPH: UJoiningGroup = UJoiningGroup(52i32);
pub const U_JG_ZHAIN: UJoiningGroup = UJoiningGroup(53i32);
pub const U_JG_BURUSHASKI_YEH_BARREE: UJoiningGroup = UJoiningGroup(54i32);
pub const U_JG_FARSI_YEH: UJoiningGroup = UJoiningGroup(55i32);
pub const U_JG_NYA: UJoiningGroup = UJoiningGroup(56i32);
pub const U_JG_ROHINGYA_YEH: UJoiningGroup = UJoiningGroup(57i32);
pub const U_JG_MANICHAEAN_ALEPH: UJoiningGroup = UJoiningGroup(58i32);
pub const U_JG_MANICHAEAN_AYIN: UJoiningGroup = UJoiningGroup(59i32);
pub const U_JG_MANICHAEAN_BETH: UJoiningGroup = UJoiningGroup(60i32);
pub const U_JG_MANICHAEAN_DALETH: UJoiningGroup = UJoiningGroup(61i32);
pub const U_JG_MANICHAEAN_DHAMEDH: UJoiningGroup = UJoiningGroup(62i32);
pub const U_JG_MANICHAEAN_FIVE: UJoiningGroup = UJoiningGroup(63i32);
pub const U_JG_MANICHAEAN_GIMEL: UJoiningGroup = UJoiningGroup(64i32);
pub const U_JG_MANICHAEAN_HETH: UJoiningGroup = UJoiningGroup(65i32);
pub const U_JG_MANICHAEAN_HUNDRED: UJoiningGroup = UJoiningGroup(66i32);
pub const U_JG_MANICHAEAN_KAPH: UJoiningGroup = UJoiningGroup(67i32);
pub const U_JG_MANICHAEAN_LAMEDH: UJoiningGroup = UJoiningGroup(68i32);
pub const U_JG_MANICHAEAN_MEM: UJoiningGroup = UJoiningGroup(69i32);
pub const U_JG_MANICHAEAN_NUN: UJoiningGroup = UJoiningGroup(70i32);
pub const U_JG_MANICHAEAN_ONE: UJoiningGroup = UJoiningGroup(71i32);
pub const U_JG_MANICHAEAN_PE: UJoiningGroup = UJoiningGroup(72i32);
pub const U_JG_MANICHAEAN_QOPH: UJoiningGroup = UJoiningGroup(73i32);
pub const U_JG_MANICHAEAN_RESH: UJoiningGroup = UJoiningGroup(74i32);
pub const U_JG_MANICHAEAN_SADHE: UJoiningGroup = UJoiningGroup(75i32);
pub const U_JG_MANICHAEAN_SAMEKH: UJoiningGroup = UJoiningGroup(76i32);
pub const U_JG_MANICHAEAN_TAW: UJoiningGroup = UJoiningGroup(77i32);
pub const U_JG_MANICHAEAN_TEN: UJoiningGroup = UJoiningGroup(78i32);
pub const U_JG_MANICHAEAN_TETH: UJoiningGroup = UJoiningGroup(79i32);
pub const U_JG_MANICHAEAN_THAMEDH: UJoiningGroup = UJoiningGroup(80i32);
pub const U_JG_MANICHAEAN_TWENTY: UJoiningGroup = UJoiningGroup(81i32);
pub const U_JG_MANICHAEAN_WAW: UJoiningGroup = UJoiningGroup(82i32);
pub const U_JG_MANICHAEAN_YODH: UJoiningGroup = UJoiningGroup(83i32);
pub const U_JG_MANICHAEAN_ZAYIN: UJoiningGroup = UJoiningGroup(84i32);
pub const U_JG_STRAIGHT_WAW: UJoiningGroup = UJoiningGroup(85i32);
pub const U_JG_AFRICAN_FEH: UJoiningGroup = UJoiningGroup(86i32);
pub const U_JG_AFRICAN_NOON: UJoiningGroup = UJoiningGroup(87i32);
pub const U_JG_AFRICAN_QAF: UJoiningGroup = UJoiningGroup(88i32);
pub const U_JG_MALAYALAM_BHA: UJoiningGroup = UJoiningGroup(89i32);
pub const U_JG_MALAYALAM_JA: UJoiningGroup = UJoiningGroup(90i32);
pub const U_JG_MALAYALAM_LLA: UJoiningGroup = UJoiningGroup(91i32);
pub const U_JG_MALAYALAM_LLLA: UJoiningGroup = UJoiningGroup(92i32);
pub const U_JG_MALAYALAM_NGA: UJoiningGroup = UJoiningGroup(93i32);
pub const U_JG_MALAYALAM_NNA: UJoiningGroup = UJoiningGroup(94i32);
pub const U_JG_MALAYALAM_NNNA: UJoiningGroup = UJoiningGroup(95i32);
pub const U_JG_MALAYALAM_NYA: UJoiningGroup = UJoiningGroup(96i32);
pub const U_JG_MALAYALAM_RA: UJoiningGroup = UJoiningGroup(97i32);
pub const U_JG_MALAYALAM_SSA: UJoiningGroup = UJoiningGroup(98i32);
pub const U_JG_MALAYALAM_TTA: UJoiningGroup = UJoiningGroup(99i32);
pub const U_JG_HANIFI_ROHINGYA_KINNA_YA: UJoiningGroup = UJoiningGroup(100i32);
pub const U_JG_HANIFI_ROHINGYA_PA: UJoiningGroup = UJoiningGroup(101i32);
impl ::core::marker::Copy for UJoiningGroup {}
impl ::core::clone::Clone for UJoiningGroup {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UJoiningGroup {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UJoiningGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UJoiningGroup").field(&self.0).finish()
    }
}
impl FromIntoMemory for UJoiningGroup {
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
pub struct UJoiningType(pub i32);
pub const U_JT_NON_JOINING: UJoiningType = UJoiningType(0i32);
pub const U_JT_JOIN_CAUSING: UJoiningType = UJoiningType(1i32);
pub const U_JT_DUAL_JOINING: UJoiningType = UJoiningType(2i32);
pub const U_JT_LEFT_JOINING: UJoiningType = UJoiningType(3i32);
pub const U_JT_RIGHT_JOINING: UJoiningType = UJoiningType(4i32);
pub const U_JT_TRANSPARENT: UJoiningType = UJoiningType(5i32);
impl ::core::marker::Copy for UJoiningType {}
impl ::core::clone::Clone for UJoiningType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UJoiningType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UJoiningType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UJoiningType").field(&self.0).finish()
    }
}
impl FromIntoMemory for UJoiningType {
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
pub const ULOC_CANADA: &'static str = "en_CA";
pub const ULOC_CANADA_FRENCH: &'static str = "fr_CA";
pub const ULOC_CHINA: &'static str = "zh_CN";
pub const ULOC_CHINESE: &'static str = "zh";
pub const ULOC_COUNTRY_CAPACITY: u32 = 4u32;
pub const ULOC_ENGLISH: &'static str = "en";
pub const ULOC_FRANCE: &'static str = "fr_FR";
pub const ULOC_FRENCH: &'static str = "fr";
pub const ULOC_FULLNAME_CAPACITY: u32 = 157u32;
pub const ULOC_GERMAN: &'static str = "de";
pub const ULOC_GERMANY: &'static str = "de_DE";
pub const ULOC_ITALIAN: &'static str = "it";
pub const ULOC_ITALY: &'static str = "it_IT";
pub const ULOC_JAPAN: &'static str = "ja_JP";
pub const ULOC_JAPANESE: &'static str = "ja";
pub const ULOC_KEYWORDS_CAPACITY: u32 = 96u32;
pub const ULOC_KEYWORD_AND_VALUES_CAPACITY: u32 = 100u32;
pub const ULOC_KEYWORD_ASSIGN_UNICODE: u32 = 61u32;
pub const ULOC_KEYWORD_ITEM_SEPARATOR_UNICODE: u32 = 59u32;
pub const ULOC_KEYWORD_SEPARATOR_UNICODE: u32 = 64u32;
pub const ULOC_KOREA: &'static str = "ko_KR";
pub const ULOC_KOREAN: &'static str = "ko";
pub const ULOC_LANG_CAPACITY: u32 = 12u32;
pub const ULOC_PRC: &'static str = "zh_CN";
pub const ULOC_SCRIPT_CAPACITY: u32 = 6u32;
pub const ULOC_SIMPLIFIED_CHINESE: &'static str = "zh_CN";
pub const ULOC_TAIWAN: &'static str = "zh_TW";
pub const ULOC_TRADITIONAL_CHINESE: &'static str = "zh_TW";
pub const ULOC_UK: &'static str = "en_GB";
pub const ULOC_US: &'static str = "en_US";
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ULayoutType(pub i32);
pub const ULOC_LAYOUT_LTR: ULayoutType = ULayoutType(0i32);
pub const ULOC_LAYOUT_RTL: ULayoutType = ULayoutType(1i32);
pub const ULOC_LAYOUT_TTB: ULayoutType = ULayoutType(2i32);
pub const ULOC_LAYOUT_BTT: ULayoutType = ULayoutType(3i32);
pub const ULOC_LAYOUT_UNKNOWN: ULayoutType = ULayoutType(4i32);
impl ::core::marker::Copy for ULayoutType {}
impl ::core::clone::Clone for ULayoutType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ULayoutType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ULayoutType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ULayoutType").field(&self.0).finish()
    }
}
impl FromIntoMemory for ULayoutType {
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
pub struct ULineBreak(pub i32);
pub const U_LB_UNKNOWN: ULineBreak = ULineBreak(0i32);
pub const U_LB_AMBIGUOUS: ULineBreak = ULineBreak(1i32);
pub const U_LB_ALPHABETIC: ULineBreak = ULineBreak(2i32);
pub const U_LB_BREAK_BOTH: ULineBreak = ULineBreak(3i32);
pub const U_LB_BREAK_AFTER: ULineBreak = ULineBreak(4i32);
pub const U_LB_BREAK_BEFORE: ULineBreak = ULineBreak(5i32);
pub const U_LB_MANDATORY_BREAK: ULineBreak = ULineBreak(6i32);
pub const U_LB_CONTINGENT_BREAK: ULineBreak = ULineBreak(7i32);
pub const U_LB_CLOSE_PUNCTUATION: ULineBreak = ULineBreak(8i32);
pub const U_LB_COMBINING_MARK: ULineBreak = ULineBreak(9i32);
pub const U_LB_CARRIAGE_RETURN: ULineBreak = ULineBreak(10i32);
pub const U_LB_EXCLAMATION: ULineBreak = ULineBreak(11i32);
pub const U_LB_GLUE: ULineBreak = ULineBreak(12i32);
pub const U_LB_HYPHEN: ULineBreak = ULineBreak(13i32);
pub const U_LB_IDEOGRAPHIC: ULineBreak = ULineBreak(14i32);
pub const U_LB_INSEPARABLE: ULineBreak = ULineBreak(15i32);
pub const U_LB_INSEPERABLE: ULineBreak = ULineBreak(15i32);
pub const U_LB_INFIX_NUMERIC: ULineBreak = ULineBreak(16i32);
pub const U_LB_LINE_FEED: ULineBreak = ULineBreak(17i32);
pub const U_LB_NONSTARTER: ULineBreak = ULineBreak(18i32);
pub const U_LB_NUMERIC: ULineBreak = ULineBreak(19i32);
pub const U_LB_OPEN_PUNCTUATION: ULineBreak = ULineBreak(20i32);
pub const U_LB_POSTFIX_NUMERIC: ULineBreak = ULineBreak(21i32);
pub const U_LB_PREFIX_NUMERIC: ULineBreak = ULineBreak(22i32);
pub const U_LB_QUOTATION: ULineBreak = ULineBreak(23i32);
pub const U_LB_COMPLEX_CONTEXT: ULineBreak = ULineBreak(24i32);
pub const U_LB_SURROGATE: ULineBreak = ULineBreak(25i32);
pub const U_LB_SPACE: ULineBreak = ULineBreak(26i32);
pub const U_LB_BREAK_SYMBOLS: ULineBreak = ULineBreak(27i32);
pub const U_LB_ZWSPACE: ULineBreak = ULineBreak(28i32);
pub const U_LB_NEXT_LINE: ULineBreak = ULineBreak(29i32);
pub const U_LB_WORD_JOINER: ULineBreak = ULineBreak(30i32);
pub const U_LB_H2: ULineBreak = ULineBreak(31i32);
pub const U_LB_H3: ULineBreak = ULineBreak(32i32);
pub const U_LB_JL: ULineBreak = ULineBreak(33i32);
pub const U_LB_JT: ULineBreak = ULineBreak(34i32);
pub const U_LB_JV: ULineBreak = ULineBreak(35i32);
pub const U_LB_CLOSE_PARENTHESIS: ULineBreak = ULineBreak(36i32);
pub const U_LB_CONDITIONAL_JAPANESE_STARTER: ULineBreak = ULineBreak(37i32);
pub const U_LB_HEBREW_LETTER: ULineBreak = ULineBreak(38i32);
pub const U_LB_REGIONAL_INDICATOR: ULineBreak = ULineBreak(39i32);
pub const U_LB_E_BASE: ULineBreak = ULineBreak(40i32);
pub const U_LB_E_MODIFIER: ULineBreak = ULineBreak(41i32);
pub const U_LB_ZWJ: ULineBreak = ULineBreak(42i32);
impl ::core::marker::Copy for ULineBreak {}
impl ::core::clone::Clone for ULineBreak {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ULineBreak {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ULineBreak {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ULineBreak").field(&self.0).finish()
    }
}
impl FromIntoMemory for ULineBreak {
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
pub struct ULineBreakTag(pub i32);
pub const UBRK_LINE_SOFT: ULineBreakTag = ULineBreakTag(0i32);
pub const UBRK_LINE_SOFT_LIMIT: ULineBreakTag = ULineBreakTag(100i32);
pub const UBRK_LINE_HARD: ULineBreakTag = ULineBreakTag(100i32);
pub const UBRK_LINE_HARD_LIMIT: ULineBreakTag = ULineBreakTag(200i32);
impl ::core::marker::Copy for ULineBreakTag {}
impl ::core::clone::Clone for ULineBreakTag {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ULineBreakTag {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ULineBreakTag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ULineBreakTag").field(&self.0).finish()
    }
}
impl FromIntoMemory for ULineBreakTag {
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
pub struct UListFormatter(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UListFormatterField(pub i32);
pub const ULISTFMT_LITERAL_FIELD: UListFormatterField = UListFormatterField(0i32);
pub const ULISTFMT_ELEMENT_FIELD: UListFormatterField = UListFormatterField(1i32);
impl ::core::marker::Copy for UListFormatterField {}
impl ::core::clone::Clone for UListFormatterField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UListFormatterField {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UListFormatterField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UListFormatterField").field(&self.0).finish()
    }
}
impl FromIntoMemory for UListFormatterField {
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
pub struct UListFormatterType(pub i32);
pub const ULISTFMT_TYPE_AND: UListFormatterType = UListFormatterType(0i32);
pub const ULISTFMT_TYPE_OR: UListFormatterType = UListFormatterType(1i32);
pub const ULISTFMT_TYPE_UNITS: UListFormatterType = UListFormatterType(2i32);
impl ::core::marker::Copy for UListFormatterType {}
impl ::core::clone::Clone for UListFormatterType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UListFormatterType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UListFormatterType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UListFormatterType").field(&self.0).finish()
    }
}
impl FromIntoMemory for UListFormatterType {
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
pub struct UListFormatterWidth(pub i32);
pub const ULISTFMT_WIDTH_WIDE: UListFormatterWidth = UListFormatterWidth(0i32);
pub const ULISTFMT_WIDTH_SHORT: UListFormatterWidth = UListFormatterWidth(1i32);
pub const ULISTFMT_WIDTH_NARROW: UListFormatterWidth = UListFormatterWidth(2i32);
impl ::core::marker::Copy for UListFormatterWidth {}
impl ::core::clone::Clone for UListFormatterWidth {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UListFormatterWidth {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UListFormatterWidth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UListFormatterWidth").field(&self.0).finish()
    }
}
impl FromIntoMemory for UListFormatterWidth {
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
pub struct ULocAvailableType(pub i32);
pub const ULOC_AVAILABLE_DEFAULT: ULocAvailableType = ULocAvailableType(0i32);
pub const ULOC_AVAILABLE_ONLY_LEGACY_ALIASES: ULocAvailableType = ULocAvailableType(1i32);
pub const ULOC_AVAILABLE_WITH_LEGACY_ALIASES: ULocAvailableType = ULocAvailableType(2i32);
impl ::core::marker::Copy for ULocAvailableType {}
impl ::core::clone::Clone for ULocAvailableType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ULocAvailableType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ULocAvailableType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ULocAvailableType").field(&self.0).finish()
    }
}
impl FromIntoMemory for ULocAvailableType {
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
pub struct ULocDataLocaleType(pub i32);
pub const ULOC_ACTUAL_LOCALE: ULocDataLocaleType = ULocDataLocaleType(0i32);
pub const ULOC_VALID_LOCALE: ULocDataLocaleType = ULocDataLocaleType(1i32);
impl ::core::marker::Copy for ULocDataLocaleType {}
impl ::core::clone::Clone for ULocDataLocaleType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ULocDataLocaleType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ULocDataLocaleType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ULocDataLocaleType").field(&self.0).finish()
    }
}
impl FromIntoMemory for ULocDataLocaleType {
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
pub struct ULocaleData(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ULocaleDataDelimiterType(pub i32);
pub const ULOCDATA_QUOTATION_START: ULocaleDataDelimiterType = ULocaleDataDelimiterType(0i32);
pub const ULOCDATA_QUOTATION_END: ULocaleDataDelimiterType = ULocaleDataDelimiterType(1i32);
pub const ULOCDATA_ALT_QUOTATION_START: ULocaleDataDelimiterType = ULocaleDataDelimiterType(2i32);
pub const ULOCDATA_ALT_QUOTATION_END: ULocaleDataDelimiterType = ULocaleDataDelimiterType(3i32);
impl ::core::marker::Copy for ULocaleDataDelimiterType {}
impl ::core::clone::Clone for ULocaleDataDelimiterType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ULocaleDataDelimiterType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ULocaleDataDelimiterType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ULocaleDataDelimiterType")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ULocaleDataDelimiterType {
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
pub struct ULocaleDataExemplarSetType(pub i32);
pub const ULOCDATA_ES_STANDARD: ULocaleDataExemplarSetType = ULocaleDataExemplarSetType(0i32);
pub const ULOCDATA_ES_AUXILIARY: ULocaleDataExemplarSetType = ULocaleDataExemplarSetType(1i32);
pub const ULOCDATA_ES_INDEX: ULocaleDataExemplarSetType = ULocaleDataExemplarSetType(2i32);
pub const ULOCDATA_ES_PUNCTUATION: ULocaleDataExemplarSetType = ULocaleDataExemplarSetType(3i32);
impl ::core::marker::Copy for ULocaleDataExemplarSetType {}
impl ::core::clone::Clone for ULocaleDataExemplarSetType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ULocaleDataExemplarSetType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ULocaleDataExemplarSetType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ULocaleDataExemplarSetType")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ULocaleDataExemplarSetType {
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
pub struct ULocaleDisplayNames(pub u8);
pub const UMSGPAT_ARG_NAME_NOT_NUMBER: i32 = -1i32;
pub const UMSGPAT_ARG_NAME_NOT_VALID: i32 = -2i32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UMeasureFormatWidth(pub i32);
pub const UMEASFMT_WIDTH_WIDE: UMeasureFormatWidth = UMeasureFormatWidth(0i32);
pub const UMEASFMT_WIDTH_SHORT: UMeasureFormatWidth = UMeasureFormatWidth(1i32);
pub const UMEASFMT_WIDTH_NARROW: UMeasureFormatWidth = UMeasureFormatWidth(2i32);
pub const UMEASFMT_WIDTH_NUMERIC: UMeasureFormatWidth = UMeasureFormatWidth(3i32);
pub const UMEASFMT_WIDTH_COUNT: UMeasureFormatWidth = UMeasureFormatWidth(4i32);
impl ::core::marker::Copy for UMeasureFormatWidth {}
impl ::core::clone::Clone for UMeasureFormatWidth {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UMeasureFormatWidth {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UMeasureFormatWidth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UMeasureFormatWidth").field(&self.0).finish()
    }
}
impl FromIntoMemory for UMeasureFormatWidth {
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
pub struct UMeasurementSystem(pub i32);
pub const UMS_SI: UMeasurementSystem = UMeasurementSystem(0i32);
pub const UMS_US: UMeasurementSystem = UMeasurementSystem(1i32);
pub const UMS_UK: UMeasurementSystem = UMeasurementSystem(2i32);
impl ::core::marker::Copy for UMeasurementSystem {}
impl ::core::clone::Clone for UMeasurementSystem {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UMeasurementSystem {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UMeasurementSystem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UMeasurementSystem").field(&self.0).finish()
    }
}
impl FromIntoMemory for UMeasurementSystem {
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
pub type UMemAllocFn =
    StdCallFnPtr<(ConstPtr<::core::ffi::c_void>, PtrRepr), MutPtr<::core::ffi::c_void>>;
pub type UMemFreeFn =
    StdCallFnPtr<(ConstPtr<::core::ffi::c_void>, MutPtr<::core::ffi::c_void>), ()>;
pub type UMemReallocFn = StdCallFnPtr<
    (
        ConstPtr<::core::ffi::c_void>,
        MutPtr<::core::ffi::c_void>,
        PtrRepr,
    ),
    MutPtr<::core::ffi::c_void>,
>;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UMessagePatternApostropheMode(pub i32);
pub const UMSGPAT_APOS_DOUBLE_OPTIONAL: UMessagePatternApostropheMode =
    UMessagePatternApostropheMode(0i32);
pub const UMSGPAT_APOS_DOUBLE_REQUIRED: UMessagePatternApostropheMode =
    UMessagePatternApostropheMode(1i32);
impl ::core::marker::Copy for UMessagePatternApostropheMode {}
impl ::core::clone::Clone for UMessagePatternApostropheMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UMessagePatternApostropheMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UMessagePatternApostropheMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UMessagePatternApostropheMode")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UMessagePatternApostropheMode {
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
pub struct UMessagePatternArgType(pub i32);
pub const UMSGPAT_ARG_TYPE_NONE: UMessagePatternArgType = UMessagePatternArgType(0i32);
pub const UMSGPAT_ARG_TYPE_SIMPLE: UMessagePatternArgType = UMessagePatternArgType(1i32);
pub const UMSGPAT_ARG_TYPE_CHOICE: UMessagePatternArgType = UMessagePatternArgType(2i32);
pub const UMSGPAT_ARG_TYPE_PLURAL: UMessagePatternArgType = UMessagePatternArgType(3i32);
pub const UMSGPAT_ARG_TYPE_SELECT: UMessagePatternArgType = UMessagePatternArgType(4i32);
pub const UMSGPAT_ARG_TYPE_SELECTORDINAL: UMessagePatternArgType = UMessagePatternArgType(5i32);
impl ::core::marker::Copy for UMessagePatternArgType {}
impl ::core::clone::Clone for UMessagePatternArgType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UMessagePatternArgType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UMessagePatternArgType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UMessagePatternArgType")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UMessagePatternArgType {
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
pub struct UMessagePatternPartType(pub i32);
pub const UMSGPAT_PART_TYPE_MSG_START: UMessagePatternPartType = UMessagePatternPartType(0i32);
pub const UMSGPAT_PART_TYPE_MSG_LIMIT: UMessagePatternPartType = UMessagePatternPartType(1i32);
pub const UMSGPAT_PART_TYPE_SKIP_SYNTAX: UMessagePatternPartType = UMessagePatternPartType(2i32);
pub const UMSGPAT_PART_TYPE_INSERT_CHAR: UMessagePatternPartType = UMessagePatternPartType(3i32);
pub const UMSGPAT_PART_TYPE_REPLACE_NUMBER: UMessagePatternPartType = UMessagePatternPartType(4i32);
pub const UMSGPAT_PART_TYPE_ARG_START: UMessagePatternPartType = UMessagePatternPartType(5i32);
pub const UMSGPAT_PART_TYPE_ARG_LIMIT: UMessagePatternPartType = UMessagePatternPartType(6i32);
pub const UMSGPAT_PART_TYPE_ARG_NUMBER: UMessagePatternPartType = UMessagePatternPartType(7i32);
pub const UMSGPAT_PART_TYPE_ARG_NAME: UMessagePatternPartType = UMessagePatternPartType(8i32);
pub const UMSGPAT_PART_TYPE_ARG_TYPE: UMessagePatternPartType = UMessagePatternPartType(9i32);
pub const UMSGPAT_PART_TYPE_ARG_STYLE: UMessagePatternPartType = UMessagePatternPartType(10i32);
pub const UMSGPAT_PART_TYPE_ARG_SELECTOR: UMessagePatternPartType = UMessagePatternPartType(11i32);
pub const UMSGPAT_PART_TYPE_ARG_INT: UMessagePatternPartType = UMessagePatternPartType(12i32);
pub const UMSGPAT_PART_TYPE_ARG_DOUBLE: UMessagePatternPartType = UMessagePatternPartType(13i32);
impl ::core::marker::Copy for UMessagePatternPartType {}
impl ::core::clone::Clone for UMessagePatternPartType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UMessagePatternPartType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UMessagePatternPartType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UMessagePatternPartType")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UMessagePatternPartType {
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
pub struct UMutableCPTrie(pub u8);
pub type UNESCAPE_CHAR_AT = StdCallFnPtr<(i32, MutPtr<::core::ffi::c_void>), u16>;
pub struct UNICODERANGE {
    pub wcFrom: u16,
    pub wcTo: u16,
}
impl ::core::marker::Copy for UNICODERANGE {}
impl ::core::clone::Clone for UNICODERANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UNICODERANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UNICODERANGE")
            .field("wcFrom", &self.wcFrom)
            .field("wcTo", &self.wcTo)
            .finish()
    }
}
impl ::core::cmp::PartialEq for UNICODERANGE {
    fn eq(&self, other: &Self) -> bool {
        self.wcFrom == other.wcFrom && self.wcTo == other.wcTo
    }
}
impl ::core::cmp::Eq for UNICODERANGE {}
impl FromIntoMemory for UNICODERANGE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 2u32 as usize);
        let f_wcFrom = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_wcTo = <u16 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        Self {
            wcFrom: f_wcFrom,
            wcTo: f_wcTo,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 2u32 as usize);
        FromIntoMemory::into_bytes(self.wcFrom, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.wcTo, &mut into[1..1 + 1]);
    }
    fn size() -> usize {
        2u32 as usize
    }
}
pub const UNISCRIBE_OPENTYPE: u32 = 256u32;
pub const UNORM_INPUT_IS_FCD: u32 = 131072u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UNormalization2Mode(pub i32);
pub const UNORM2_COMPOSE: UNormalization2Mode = UNormalization2Mode(0i32);
pub const UNORM2_DECOMPOSE: UNormalization2Mode = UNormalization2Mode(1i32);
pub const UNORM2_FCD: UNormalization2Mode = UNormalization2Mode(2i32);
pub const UNORM2_COMPOSE_CONTIGUOUS: UNormalization2Mode = UNormalization2Mode(3i32);
impl ::core::marker::Copy for UNormalization2Mode {}
impl ::core::clone::Clone for UNormalization2Mode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UNormalization2Mode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNormalization2Mode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNormalization2Mode").field(&self.0).finish()
    }
}
impl FromIntoMemory for UNormalization2Mode {
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
pub struct UNormalizationCheckResult(pub i32);
pub const UNORM_NO: UNormalizationCheckResult = UNormalizationCheckResult(0i32);
pub const UNORM_YES: UNormalizationCheckResult = UNormalizationCheckResult(1i32);
pub const UNORM_MAYBE: UNormalizationCheckResult = UNormalizationCheckResult(2i32);
impl ::core::marker::Copy for UNormalizationCheckResult {}
impl ::core::clone::Clone for UNormalizationCheckResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UNormalizationCheckResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNormalizationCheckResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNormalizationCheckResult")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UNormalizationCheckResult {
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
pub struct UNormalizationMode(pub i32);
pub const UNORM_NONE: UNormalizationMode = UNormalizationMode(1i32);
pub const UNORM_NFD: UNormalizationMode = UNormalizationMode(2i32);
pub const UNORM_NFKD: UNormalizationMode = UNormalizationMode(3i32);
pub const UNORM_NFC: UNormalizationMode = UNormalizationMode(4i32);
pub const UNORM_DEFAULT: UNormalizationMode = UNormalizationMode(4i32);
pub const UNORM_NFKC: UNormalizationMode = UNormalizationMode(5i32);
pub const UNORM_FCD: UNormalizationMode = UNormalizationMode(6i32);
pub const UNORM_MODE_COUNT: UNormalizationMode = UNormalizationMode(7i32);
impl ::core::marker::Copy for UNormalizationMode {}
impl ::core::clone::Clone for UNormalizationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UNormalizationMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNormalizationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNormalizationMode").field(&self.0).finish()
    }
}
impl FromIntoMemory for UNormalizationMode {
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
pub struct UNormalizer2(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UNumberCompactStyle(pub i32);
pub const UNUM_SHORT: UNumberCompactStyle = UNumberCompactStyle(0i32);
pub const UNUM_LONG: UNumberCompactStyle = UNumberCompactStyle(1i32);
impl ::core::marker::Copy for UNumberCompactStyle {}
impl ::core::clone::Clone for UNumberCompactStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UNumberCompactStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberCompactStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberCompactStyle").field(&self.0).finish()
    }
}
impl FromIntoMemory for UNumberCompactStyle {
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
pub struct UNumberDecimalSeparatorDisplay(pub i32);
pub const UNUM_DECIMAL_SEPARATOR_AUTO: UNumberDecimalSeparatorDisplay =
    UNumberDecimalSeparatorDisplay(0i32);
pub const UNUM_DECIMAL_SEPARATOR_ALWAYS: UNumberDecimalSeparatorDisplay =
    UNumberDecimalSeparatorDisplay(1i32);
pub const UNUM_DECIMAL_SEPARATOR_COUNT: UNumberDecimalSeparatorDisplay =
    UNumberDecimalSeparatorDisplay(2i32);
impl ::core::marker::Copy for UNumberDecimalSeparatorDisplay {}
impl ::core::clone::Clone for UNumberDecimalSeparatorDisplay {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UNumberDecimalSeparatorDisplay {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberDecimalSeparatorDisplay {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberDecimalSeparatorDisplay")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UNumberDecimalSeparatorDisplay {
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
pub struct UNumberFormatAttribute(pub i32);
pub const UNUM_PARSE_INT_ONLY: UNumberFormatAttribute = UNumberFormatAttribute(0i32);
pub const UNUM_GROUPING_USED: UNumberFormatAttribute = UNumberFormatAttribute(1i32);
pub const UNUM_DECIMAL_ALWAYS_SHOWN: UNumberFormatAttribute = UNumberFormatAttribute(2i32);
pub const UNUM_MAX_INTEGER_DIGITS: UNumberFormatAttribute = UNumberFormatAttribute(3i32);
pub const UNUM_MIN_INTEGER_DIGITS: UNumberFormatAttribute = UNumberFormatAttribute(4i32);
pub const UNUM_INTEGER_DIGITS: UNumberFormatAttribute = UNumberFormatAttribute(5i32);
pub const UNUM_MAX_FRACTION_DIGITS: UNumberFormatAttribute = UNumberFormatAttribute(6i32);
pub const UNUM_MIN_FRACTION_DIGITS: UNumberFormatAttribute = UNumberFormatAttribute(7i32);
pub const UNUM_FRACTION_DIGITS: UNumberFormatAttribute = UNumberFormatAttribute(8i32);
pub const UNUM_MULTIPLIER: UNumberFormatAttribute = UNumberFormatAttribute(9i32);
pub const UNUM_GROUPING_SIZE: UNumberFormatAttribute = UNumberFormatAttribute(10i32);
pub const UNUM_ROUNDING_MODE: UNumberFormatAttribute = UNumberFormatAttribute(11i32);
pub const UNUM_ROUNDING_INCREMENT: UNumberFormatAttribute = UNumberFormatAttribute(12i32);
pub const UNUM_FORMAT_WIDTH: UNumberFormatAttribute = UNumberFormatAttribute(13i32);
pub const UNUM_PADDING_POSITION: UNumberFormatAttribute = UNumberFormatAttribute(14i32);
pub const UNUM_SECONDARY_GROUPING_SIZE: UNumberFormatAttribute = UNumberFormatAttribute(15i32);
pub const UNUM_SIGNIFICANT_DIGITS_USED: UNumberFormatAttribute = UNumberFormatAttribute(16i32);
pub const UNUM_MIN_SIGNIFICANT_DIGITS: UNumberFormatAttribute = UNumberFormatAttribute(17i32);
pub const UNUM_MAX_SIGNIFICANT_DIGITS: UNumberFormatAttribute = UNumberFormatAttribute(18i32);
pub const UNUM_LENIENT_PARSE: UNumberFormatAttribute = UNumberFormatAttribute(19i32);
pub const UNUM_PARSE_ALL_INPUT: UNumberFormatAttribute = UNumberFormatAttribute(20i32);
pub const UNUM_SCALE: UNumberFormatAttribute = UNumberFormatAttribute(21i32);
pub const UNUM_MINIMUM_GROUPING_DIGITS: UNumberFormatAttribute = UNumberFormatAttribute(22i32);
pub const UNUM_CURRENCY_USAGE: UNumberFormatAttribute = UNumberFormatAttribute(23i32);
pub const UNUM_FORMAT_FAIL_IF_MORE_THAN_MAX_DIGITS: UNumberFormatAttribute =
    UNumberFormatAttribute(4096i32);
pub const UNUM_PARSE_NO_EXPONENT: UNumberFormatAttribute = UNumberFormatAttribute(4097i32);
pub const UNUM_PARSE_DECIMAL_MARK_REQUIRED: UNumberFormatAttribute =
    UNumberFormatAttribute(4098i32);
pub const UNUM_PARSE_CASE_SENSITIVE: UNumberFormatAttribute = UNumberFormatAttribute(4099i32);
pub const UNUM_SIGN_ALWAYS_SHOWN: UNumberFormatAttribute = UNumberFormatAttribute(4100i32);
impl ::core::marker::Copy for UNumberFormatAttribute {}
impl ::core::clone::Clone for UNumberFormatAttribute {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UNumberFormatAttribute {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberFormatAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberFormatAttribute")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UNumberFormatAttribute {
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
pub struct UNumberFormatAttributeValue(pub i32);
pub const UNUM_FORMAT_ATTRIBUTE_VALUE_HIDDEN: UNumberFormatAttributeValue =
    UNumberFormatAttributeValue(0i32);
impl ::core::marker::Copy for UNumberFormatAttributeValue {}
impl ::core::clone::Clone for UNumberFormatAttributeValue {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UNumberFormatAttributeValue {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberFormatAttributeValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberFormatAttributeValue")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UNumberFormatAttributeValue {
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
pub struct UNumberFormatFields(pub i32);
pub const UNUM_INTEGER_FIELD: UNumberFormatFields = UNumberFormatFields(0i32);
pub const UNUM_FRACTION_FIELD: UNumberFormatFields = UNumberFormatFields(1i32);
pub const UNUM_DECIMAL_SEPARATOR_FIELD: UNumberFormatFields = UNumberFormatFields(2i32);
pub const UNUM_EXPONENT_SYMBOL_FIELD: UNumberFormatFields = UNumberFormatFields(3i32);
pub const UNUM_EXPONENT_SIGN_FIELD: UNumberFormatFields = UNumberFormatFields(4i32);
pub const UNUM_EXPONENT_FIELD: UNumberFormatFields = UNumberFormatFields(5i32);
pub const UNUM_GROUPING_SEPARATOR_FIELD: UNumberFormatFields = UNumberFormatFields(6i32);
pub const UNUM_CURRENCY_FIELD: UNumberFormatFields = UNumberFormatFields(7i32);
pub const UNUM_PERCENT_FIELD: UNumberFormatFields = UNumberFormatFields(8i32);
pub const UNUM_PERMILL_FIELD: UNumberFormatFields = UNumberFormatFields(9i32);
pub const UNUM_SIGN_FIELD: UNumberFormatFields = UNumberFormatFields(10i32);
pub const UNUM_MEASURE_UNIT_FIELD: UNumberFormatFields = UNumberFormatFields(11i32);
pub const UNUM_COMPACT_FIELD: UNumberFormatFields = UNumberFormatFields(12i32);
impl ::core::marker::Copy for UNumberFormatFields {}
impl ::core::clone::Clone for UNumberFormatFields {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UNumberFormatFields {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberFormatFields {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberFormatFields").field(&self.0).finish()
    }
}
impl FromIntoMemory for UNumberFormatFields {
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
pub struct UNumberFormatPadPosition(pub i32);
pub const UNUM_PAD_BEFORE_PREFIX: UNumberFormatPadPosition = UNumberFormatPadPosition(0i32);
pub const UNUM_PAD_AFTER_PREFIX: UNumberFormatPadPosition = UNumberFormatPadPosition(1i32);
pub const UNUM_PAD_BEFORE_SUFFIX: UNumberFormatPadPosition = UNumberFormatPadPosition(2i32);
pub const UNUM_PAD_AFTER_SUFFIX: UNumberFormatPadPosition = UNumberFormatPadPosition(3i32);
impl ::core::marker::Copy for UNumberFormatPadPosition {}
impl ::core::clone::Clone for UNumberFormatPadPosition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UNumberFormatPadPosition {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberFormatPadPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberFormatPadPosition")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UNumberFormatPadPosition {
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
pub struct UNumberFormatRoundingMode(pub i32);
pub const UNUM_ROUND_CEILING: UNumberFormatRoundingMode = UNumberFormatRoundingMode(0i32);
pub const UNUM_ROUND_FLOOR: UNumberFormatRoundingMode = UNumberFormatRoundingMode(1i32);
pub const UNUM_ROUND_DOWN: UNumberFormatRoundingMode = UNumberFormatRoundingMode(2i32);
pub const UNUM_ROUND_UP: UNumberFormatRoundingMode = UNumberFormatRoundingMode(3i32);
pub const UNUM_ROUND_HALFEVEN: UNumberFormatRoundingMode = UNumberFormatRoundingMode(4i32);
pub const UNUM_ROUND_HALFDOWN: UNumberFormatRoundingMode = UNumberFormatRoundingMode(5i32);
pub const UNUM_ROUND_HALFUP: UNumberFormatRoundingMode = UNumberFormatRoundingMode(6i32);
pub const UNUM_ROUND_UNNECESSARY: UNumberFormatRoundingMode = UNumberFormatRoundingMode(7i32);
impl ::core::marker::Copy for UNumberFormatRoundingMode {}
impl ::core::clone::Clone for UNumberFormatRoundingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UNumberFormatRoundingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberFormatRoundingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberFormatRoundingMode")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UNumberFormatRoundingMode {
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
pub struct UNumberFormatStyle(pub i32);
pub const UNUM_PATTERN_DECIMAL: UNumberFormatStyle = UNumberFormatStyle(0i32);
pub const UNUM_DECIMAL: UNumberFormatStyle = UNumberFormatStyle(1i32);
pub const UNUM_CURRENCY: UNumberFormatStyle = UNumberFormatStyle(2i32);
pub const UNUM_PERCENT: UNumberFormatStyle = UNumberFormatStyle(3i32);
pub const UNUM_SCIENTIFIC: UNumberFormatStyle = UNumberFormatStyle(4i32);
pub const UNUM_SPELLOUT: UNumberFormatStyle = UNumberFormatStyle(5i32);
pub const UNUM_ORDINAL: UNumberFormatStyle = UNumberFormatStyle(6i32);
pub const UNUM_DURATION: UNumberFormatStyle = UNumberFormatStyle(7i32);
pub const UNUM_NUMBERING_SYSTEM: UNumberFormatStyle = UNumberFormatStyle(8i32);
pub const UNUM_PATTERN_RULEBASED: UNumberFormatStyle = UNumberFormatStyle(9i32);
pub const UNUM_CURRENCY_ISO: UNumberFormatStyle = UNumberFormatStyle(10i32);
pub const UNUM_CURRENCY_PLURAL: UNumberFormatStyle = UNumberFormatStyle(11i32);
pub const UNUM_CURRENCY_ACCOUNTING: UNumberFormatStyle = UNumberFormatStyle(12i32);
pub const UNUM_CASH_CURRENCY: UNumberFormatStyle = UNumberFormatStyle(13i32);
pub const UNUM_DECIMAL_COMPACT_SHORT: UNumberFormatStyle = UNumberFormatStyle(14i32);
pub const UNUM_DECIMAL_COMPACT_LONG: UNumberFormatStyle = UNumberFormatStyle(15i32);
pub const UNUM_CURRENCY_STANDARD: UNumberFormatStyle = UNumberFormatStyle(16i32);
pub const UNUM_DEFAULT: UNumberFormatStyle = UNumberFormatStyle(1i32);
pub const UNUM_IGNORE: UNumberFormatStyle = UNumberFormatStyle(0i32);
impl ::core::marker::Copy for UNumberFormatStyle {}
impl ::core::clone::Clone for UNumberFormatStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UNumberFormatStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberFormatStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberFormatStyle").field(&self.0).finish()
    }
}
impl FromIntoMemory for UNumberFormatStyle {
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
pub struct UNumberFormatSymbol(pub i32);
pub const UNUM_DECIMAL_SEPARATOR_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(0i32);
pub const UNUM_GROUPING_SEPARATOR_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(1i32);
pub const UNUM_PATTERN_SEPARATOR_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(2i32);
pub const UNUM_PERCENT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(3i32);
pub const UNUM_ZERO_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(4i32);
pub const UNUM_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(5i32);
pub const UNUM_MINUS_SIGN_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(6i32);
pub const UNUM_PLUS_SIGN_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(7i32);
pub const UNUM_CURRENCY_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(8i32);
pub const UNUM_INTL_CURRENCY_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(9i32);
pub const UNUM_MONETARY_SEPARATOR_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(10i32);
pub const UNUM_EXPONENTIAL_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(11i32);
pub const UNUM_PERMILL_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(12i32);
pub const UNUM_PAD_ESCAPE_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(13i32);
pub const UNUM_INFINITY_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(14i32);
pub const UNUM_NAN_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(15i32);
pub const UNUM_SIGNIFICANT_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(16i32);
pub const UNUM_MONETARY_GROUPING_SEPARATOR_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(17i32);
pub const UNUM_ONE_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(18i32);
pub const UNUM_TWO_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(19i32);
pub const UNUM_THREE_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(20i32);
pub const UNUM_FOUR_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(21i32);
pub const UNUM_FIVE_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(22i32);
pub const UNUM_SIX_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(23i32);
pub const UNUM_SEVEN_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(24i32);
pub const UNUM_EIGHT_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(25i32);
pub const UNUM_NINE_DIGIT_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(26i32);
pub const UNUM_EXPONENT_MULTIPLICATION_SYMBOL: UNumberFormatSymbol = UNumberFormatSymbol(27i32);
impl ::core::marker::Copy for UNumberFormatSymbol {}
impl ::core::clone::Clone for UNumberFormatSymbol {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UNumberFormatSymbol {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberFormatSymbol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberFormatSymbol").field(&self.0).finish()
    }
}
impl FromIntoMemory for UNumberFormatSymbol {
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
pub struct UNumberFormatTextAttribute(pub i32);
pub const UNUM_POSITIVE_PREFIX: UNumberFormatTextAttribute = UNumberFormatTextAttribute(0i32);
pub const UNUM_POSITIVE_SUFFIX: UNumberFormatTextAttribute = UNumberFormatTextAttribute(1i32);
pub const UNUM_NEGATIVE_PREFIX: UNumberFormatTextAttribute = UNumberFormatTextAttribute(2i32);
pub const UNUM_NEGATIVE_SUFFIX: UNumberFormatTextAttribute = UNumberFormatTextAttribute(3i32);
pub const UNUM_PADDING_CHARACTER: UNumberFormatTextAttribute = UNumberFormatTextAttribute(4i32);
pub const UNUM_CURRENCY_CODE: UNumberFormatTextAttribute = UNumberFormatTextAttribute(5i32);
pub const UNUM_DEFAULT_RULESET: UNumberFormatTextAttribute = UNumberFormatTextAttribute(6i32);
pub const UNUM_PUBLIC_RULESETS: UNumberFormatTextAttribute = UNumberFormatTextAttribute(7i32);
impl ::core::marker::Copy for UNumberFormatTextAttribute {}
impl ::core::clone::Clone for UNumberFormatTextAttribute {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UNumberFormatTextAttribute {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberFormatTextAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberFormatTextAttribute")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UNumberFormatTextAttribute {
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
pub struct UNumberFormatter(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UNumberGroupingStrategy(pub i32);
pub const UNUM_GROUPING_OFF: UNumberGroupingStrategy = UNumberGroupingStrategy(0i32);
pub const UNUM_GROUPING_MIN2: UNumberGroupingStrategy = UNumberGroupingStrategy(1i32);
pub const UNUM_GROUPING_AUTO: UNumberGroupingStrategy = UNumberGroupingStrategy(2i32);
pub const UNUM_GROUPING_ON_ALIGNED: UNumberGroupingStrategy = UNumberGroupingStrategy(3i32);
pub const UNUM_GROUPING_THOUSANDS: UNumberGroupingStrategy = UNumberGroupingStrategy(4i32);
impl ::core::marker::Copy for UNumberGroupingStrategy {}
impl ::core::clone::Clone for UNumberGroupingStrategy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UNumberGroupingStrategy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberGroupingStrategy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberGroupingStrategy")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UNumberGroupingStrategy {
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
pub struct UNumberRangeCollapse(pub i32);
pub const UNUM_RANGE_COLLAPSE_AUTO: UNumberRangeCollapse = UNumberRangeCollapse(0i32);
pub const UNUM_RANGE_COLLAPSE_NONE: UNumberRangeCollapse = UNumberRangeCollapse(1i32);
pub const UNUM_RANGE_COLLAPSE_UNIT: UNumberRangeCollapse = UNumberRangeCollapse(2i32);
pub const UNUM_RANGE_COLLAPSE_ALL: UNumberRangeCollapse = UNumberRangeCollapse(3i32);
impl ::core::marker::Copy for UNumberRangeCollapse {}
impl ::core::clone::Clone for UNumberRangeCollapse {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UNumberRangeCollapse {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberRangeCollapse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberRangeCollapse")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UNumberRangeCollapse {
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
pub struct UNumberRangeIdentityFallback(pub i32);
pub const UNUM_IDENTITY_FALLBACK_SINGLE_VALUE: UNumberRangeIdentityFallback =
    UNumberRangeIdentityFallback(0i32);
pub const UNUM_IDENTITY_FALLBACK_APPROXIMATELY_OR_SINGLE_VALUE: UNumberRangeIdentityFallback =
    UNumberRangeIdentityFallback(1i32);
pub const UNUM_IDENTITY_FALLBACK_APPROXIMATELY: UNumberRangeIdentityFallback =
    UNumberRangeIdentityFallback(2i32);
pub const UNUM_IDENTITY_FALLBACK_RANGE: UNumberRangeIdentityFallback =
    UNumberRangeIdentityFallback(3i32);
impl ::core::marker::Copy for UNumberRangeIdentityFallback {}
impl ::core::clone::Clone for UNumberRangeIdentityFallback {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UNumberRangeIdentityFallback {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberRangeIdentityFallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberRangeIdentityFallback")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UNumberRangeIdentityFallback {
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
pub struct UNumberRangeIdentityResult(pub i32);
pub const UNUM_IDENTITY_RESULT_EQUAL_BEFORE_ROUNDING: UNumberRangeIdentityResult =
    UNumberRangeIdentityResult(0i32);
pub const UNUM_IDENTITY_RESULT_EQUAL_AFTER_ROUNDING: UNumberRangeIdentityResult =
    UNumberRangeIdentityResult(1i32);
pub const UNUM_IDENTITY_RESULT_NOT_EQUAL: UNumberRangeIdentityResult =
    UNumberRangeIdentityResult(2i32);
impl ::core::marker::Copy for UNumberRangeIdentityResult {}
impl ::core::clone::Clone for UNumberRangeIdentityResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UNumberRangeIdentityResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberRangeIdentityResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberRangeIdentityResult")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UNumberRangeIdentityResult {
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
pub struct UNumberSignDisplay(pub i32);
pub const UNUM_SIGN_AUTO: UNumberSignDisplay = UNumberSignDisplay(0i32);
pub const UNUM_SIGN_ALWAYS: UNumberSignDisplay = UNumberSignDisplay(1i32);
pub const UNUM_SIGN_NEVER: UNumberSignDisplay = UNumberSignDisplay(2i32);
pub const UNUM_SIGN_ACCOUNTING: UNumberSignDisplay = UNumberSignDisplay(3i32);
pub const UNUM_SIGN_ACCOUNTING_ALWAYS: UNumberSignDisplay = UNumberSignDisplay(4i32);
pub const UNUM_SIGN_EXCEPT_ZERO: UNumberSignDisplay = UNumberSignDisplay(5i32);
pub const UNUM_SIGN_ACCOUNTING_EXCEPT_ZERO: UNumberSignDisplay = UNumberSignDisplay(6i32);
pub const UNUM_SIGN_COUNT: UNumberSignDisplay = UNumberSignDisplay(7i32);
impl ::core::marker::Copy for UNumberSignDisplay {}
impl ::core::clone::Clone for UNumberSignDisplay {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UNumberSignDisplay {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberSignDisplay {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberSignDisplay").field(&self.0).finish()
    }
}
impl FromIntoMemory for UNumberSignDisplay {
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
pub struct UNumberUnitWidth(pub i32);
pub const UNUM_UNIT_WIDTH_NARROW: UNumberUnitWidth = UNumberUnitWidth(0i32);
pub const UNUM_UNIT_WIDTH_SHORT: UNumberUnitWidth = UNumberUnitWidth(1i32);
pub const UNUM_UNIT_WIDTH_FULL_NAME: UNumberUnitWidth = UNumberUnitWidth(2i32);
pub const UNUM_UNIT_WIDTH_ISO_CODE: UNumberUnitWidth = UNumberUnitWidth(3i32);
pub const UNUM_UNIT_WIDTH_HIDDEN: UNumberUnitWidth = UNumberUnitWidth(4i32);
pub const UNUM_UNIT_WIDTH_COUNT: UNumberUnitWidth = UNumberUnitWidth(5i32);
impl ::core::marker::Copy for UNumberUnitWidth {}
impl ::core::clone::Clone for UNumberUnitWidth {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UNumberUnitWidth {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumberUnitWidth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumberUnitWidth").field(&self.0).finish()
    }
}
impl FromIntoMemory for UNumberUnitWidth {
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
pub struct UNumberingSystem(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UNumericType(pub i32);
pub const U_NT_NONE: UNumericType = UNumericType(0i32);
pub const U_NT_DECIMAL: UNumericType = UNumericType(1i32);
pub const U_NT_DIGIT: UNumericType = UNumericType(2i32);
pub const U_NT_NUMERIC: UNumericType = UNumericType(3i32);
impl ::core::marker::Copy for UNumericType {}
impl ::core::clone::Clone for UNumericType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UNumericType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNumericType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNumericType").field(&self.0).finish()
    }
}
impl FromIntoMemory for UNumericType {
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
pub struct UParseError {
    pub line: i32,
    pub offset: i32,
    pub preContext: [u16; 16],
    pub postContext: [u16; 16],
}
impl ::core::marker::Copy for UParseError {}
impl ::core::clone::Clone for UParseError {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UParseError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UParseError")
            .field("line", &self.line)
            .field("offset", &self.offset)
            .field("preContext", &self.preContext)
            .field("postContext", &self.postContext)
            .finish()
    }
}
impl ::core::cmp::PartialEq for UParseError {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line
            && self.offset == other.offset
            && self.preContext == other.preContext
            && self.postContext == other.postContext
    }
}
impl ::core::cmp::Eq for UParseError {}
impl FromIntoMemory for UParseError {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 72u32 as usize);
        let f_line = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_offset = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_preContext = <[u16; 16] as FromIntoMemory>::from_bytes(&from[8..8 + 32]);
        let f_postContext = <[u16; 16] as FromIntoMemory>::from_bytes(&from[40..40 + 32]);
        Self {
            line: f_line,
            offset: f_offset,
            preContext: f_preContext,
            postContext: f_postContext,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 72u32 as usize);
        FromIntoMemory::into_bytes(self.line, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.offset, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.preContext, &mut into[8..8 + 32]);
        FromIntoMemory::into_bytes(self.postContext, &mut into[40..40 + 32]);
    }
    fn size() -> usize {
        72u32 as usize
    }
}
pub struct UPluralRules(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UPluralType(pub i32);
pub const UPLURAL_TYPE_CARDINAL: UPluralType = UPluralType(0i32);
pub const UPLURAL_TYPE_ORDINAL: UPluralType = UPluralType(1i32);
impl ::core::marker::Copy for UPluralType {}
impl ::core::clone::Clone for UPluralType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UPluralType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UPluralType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UPluralType").field(&self.0).finish()
    }
}
impl FromIntoMemory for UPluralType {
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
pub struct UProperty(pub i32);
pub const UCHAR_ALPHABETIC: UProperty = UProperty(0i32);
pub const UCHAR_BINARY_START: UProperty = UProperty(0i32);
pub const UCHAR_ASCII_HEX_DIGIT: UProperty = UProperty(1i32);
pub const UCHAR_BIDI_CONTROL: UProperty = UProperty(2i32);
pub const UCHAR_BIDI_MIRRORED: UProperty = UProperty(3i32);
pub const UCHAR_DASH: UProperty = UProperty(4i32);
pub const UCHAR_DEFAULT_IGNORABLE_CODE_POINT: UProperty = UProperty(5i32);
pub const UCHAR_DEPRECATED: UProperty = UProperty(6i32);
pub const UCHAR_DIACRITIC: UProperty = UProperty(7i32);
pub const UCHAR_EXTENDER: UProperty = UProperty(8i32);
pub const UCHAR_FULL_COMPOSITION_EXCLUSION: UProperty = UProperty(9i32);
pub const UCHAR_GRAPHEME_BASE: UProperty = UProperty(10i32);
pub const UCHAR_GRAPHEME_EXTEND: UProperty = UProperty(11i32);
pub const UCHAR_GRAPHEME_LINK: UProperty = UProperty(12i32);
pub const UCHAR_HEX_DIGIT: UProperty = UProperty(13i32);
pub const UCHAR_HYPHEN: UProperty = UProperty(14i32);
pub const UCHAR_ID_CONTINUE: UProperty = UProperty(15i32);
pub const UCHAR_ID_START: UProperty = UProperty(16i32);
pub const UCHAR_IDEOGRAPHIC: UProperty = UProperty(17i32);
pub const UCHAR_IDS_BINARY_OPERATOR: UProperty = UProperty(18i32);
pub const UCHAR_IDS_TRINARY_OPERATOR: UProperty = UProperty(19i32);
pub const UCHAR_JOIN_CONTROL: UProperty = UProperty(20i32);
pub const UCHAR_LOGICAL_ORDER_EXCEPTION: UProperty = UProperty(21i32);
pub const UCHAR_LOWERCASE: UProperty = UProperty(22i32);
pub const UCHAR_MATH: UProperty = UProperty(23i32);
pub const UCHAR_NONCHARACTER_CODE_POINT: UProperty = UProperty(24i32);
pub const UCHAR_QUOTATION_MARK: UProperty = UProperty(25i32);
pub const UCHAR_RADICAL: UProperty = UProperty(26i32);
pub const UCHAR_SOFT_DOTTED: UProperty = UProperty(27i32);
pub const UCHAR_TERMINAL_PUNCTUATION: UProperty = UProperty(28i32);
pub const UCHAR_UNIFIED_IDEOGRAPH: UProperty = UProperty(29i32);
pub const UCHAR_UPPERCASE: UProperty = UProperty(30i32);
pub const UCHAR_WHITE_SPACE: UProperty = UProperty(31i32);
pub const UCHAR_XID_CONTINUE: UProperty = UProperty(32i32);
pub const UCHAR_XID_START: UProperty = UProperty(33i32);
pub const UCHAR_CASE_SENSITIVE: UProperty = UProperty(34i32);
pub const UCHAR_S_TERM: UProperty = UProperty(35i32);
pub const UCHAR_VARIATION_SELECTOR: UProperty = UProperty(36i32);
pub const UCHAR_NFD_INERT: UProperty = UProperty(37i32);
pub const UCHAR_NFKD_INERT: UProperty = UProperty(38i32);
pub const UCHAR_NFC_INERT: UProperty = UProperty(39i32);
pub const UCHAR_NFKC_INERT: UProperty = UProperty(40i32);
pub const UCHAR_SEGMENT_STARTER: UProperty = UProperty(41i32);
pub const UCHAR_PATTERN_SYNTAX: UProperty = UProperty(42i32);
pub const UCHAR_PATTERN_WHITE_SPACE: UProperty = UProperty(43i32);
pub const UCHAR_POSIX_ALNUM: UProperty = UProperty(44i32);
pub const UCHAR_POSIX_BLANK: UProperty = UProperty(45i32);
pub const UCHAR_POSIX_GRAPH: UProperty = UProperty(46i32);
pub const UCHAR_POSIX_PRINT: UProperty = UProperty(47i32);
pub const UCHAR_POSIX_XDIGIT: UProperty = UProperty(48i32);
pub const UCHAR_CASED: UProperty = UProperty(49i32);
pub const UCHAR_CASE_IGNORABLE: UProperty = UProperty(50i32);
pub const UCHAR_CHANGES_WHEN_LOWERCASED: UProperty = UProperty(51i32);
pub const UCHAR_CHANGES_WHEN_UPPERCASED: UProperty = UProperty(52i32);
pub const UCHAR_CHANGES_WHEN_TITLECASED: UProperty = UProperty(53i32);
pub const UCHAR_CHANGES_WHEN_CASEFOLDED: UProperty = UProperty(54i32);
pub const UCHAR_CHANGES_WHEN_CASEMAPPED: UProperty = UProperty(55i32);
pub const UCHAR_CHANGES_WHEN_NFKC_CASEFOLDED: UProperty = UProperty(56i32);
pub const UCHAR_EMOJI: UProperty = UProperty(57i32);
pub const UCHAR_EMOJI_PRESENTATION: UProperty = UProperty(58i32);
pub const UCHAR_EMOJI_MODIFIER: UProperty = UProperty(59i32);
pub const UCHAR_EMOJI_MODIFIER_BASE: UProperty = UProperty(60i32);
pub const UCHAR_EMOJI_COMPONENT: UProperty = UProperty(61i32);
pub const UCHAR_REGIONAL_INDICATOR: UProperty = UProperty(62i32);
pub const UCHAR_PREPENDED_CONCATENATION_MARK: UProperty = UProperty(63i32);
pub const UCHAR_EXTENDED_PICTOGRAPHIC: UProperty = UProperty(64i32);
pub const UCHAR_BIDI_CLASS: UProperty = UProperty(4096i32);
pub const UCHAR_INT_START: UProperty = UProperty(4096i32);
pub const UCHAR_BLOCK: UProperty = UProperty(4097i32);
pub const UCHAR_CANONICAL_COMBINING_CLASS: UProperty = UProperty(4098i32);
pub const UCHAR_DECOMPOSITION_TYPE: UProperty = UProperty(4099i32);
pub const UCHAR_EAST_ASIAN_WIDTH: UProperty = UProperty(4100i32);
pub const UCHAR_GENERAL_CATEGORY: UProperty = UProperty(4101i32);
pub const UCHAR_JOINING_GROUP: UProperty = UProperty(4102i32);
pub const UCHAR_JOINING_TYPE: UProperty = UProperty(4103i32);
pub const UCHAR_LINE_BREAK: UProperty = UProperty(4104i32);
pub const UCHAR_NUMERIC_TYPE: UProperty = UProperty(4105i32);
pub const UCHAR_SCRIPT: UProperty = UProperty(4106i32);
pub const UCHAR_HANGUL_SYLLABLE_TYPE: UProperty = UProperty(4107i32);
pub const UCHAR_NFD_QUICK_CHECK: UProperty = UProperty(4108i32);
pub const UCHAR_NFKD_QUICK_CHECK: UProperty = UProperty(4109i32);
pub const UCHAR_NFC_QUICK_CHECK: UProperty = UProperty(4110i32);
pub const UCHAR_NFKC_QUICK_CHECK: UProperty = UProperty(4111i32);
pub const UCHAR_LEAD_CANONICAL_COMBINING_CLASS: UProperty = UProperty(4112i32);
pub const UCHAR_TRAIL_CANONICAL_COMBINING_CLASS: UProperty = UProperty(4113i32);
pub const UCHAR_GRAPHEME_CLUSTER_BREAK: UProperty = UProperty(4114i32);
pub const UCHAR_SENTENCE_BREAK: UProperty = UProperty(4115i32);
pub const UCHAR_WORD_BREAK: UProperty = UProperty(4116i32);
pub const UCHAR_BIDI_PAIRED_BRACKET_TYPE: UProperty = UProperty(4117i32);
pub const UCHAR_INDIC_POSITIONAL_CATEGORY: UProperty = UProperty(4118i32);
pub const UCHAR_INDIC_SYLLABIC_CATEGORY: UProperty = UProperty(4119i32);
pub const UCHAR_VERTICAL_ORIENTATION: UProperty = UProperty(4120i32);
pub const UCHAR_GENERAL_CATEGORY_MASK: UProperty = UProperty(8192i32);
pub const UCHAR_MASK_START: UProperty = UProperty(8192i32);
pub const UCHAR_NUMERIC_VALUE: UProperty = UProperty(12288i32);
pub const UCHAR_DOUBLE_START: UProperty = UProperty(12288i32);
pub const UCHAR_AGE: UProperty = UProperty(16384i32);
pub const UCHAR_STRING_START: UProperty = UProperty(16384i32);
pub const UCHAR_BIDI_MIRRORING_GLYPH: UProperty = UProperty(16385i32);
pub const UCHAR_CASE_FOLDING: UProperty = UProperty(16386i32);
pub const UCHAR_LOWERCASE_MAPPING: UProperty = UProperty(16388i32);
pub const UCHAR_NAME: UProperty = UProperty(16389i32);
pub const UCHAR_SIMPLE_CASE_FOLDING: UProperty = UProperty(16390i32);
pub const UCHAR_SIMPLE_LOWERCASE_MAPPING: UProperty = UProperty(16391i32);
pub const UCHAR_SIMPLE_TITLECASE_MAPPING: UProperty = UProperty(16392i32);
pub const UCHAR_SIMPLE_UPPERCASE_MAPPING: UProperty = UProperty(16393i32);
pub const UCHAR_TITLECASE_MAPPING: UProperty = UProperty(16394i32);
pub const UCHAR_UPPERCASE_MAPPING: UProperty = UProperty(16396i32);
pub const UCHAR_BIDI_PAIRED_BRACKET: UProperty = UProperty(16397i32);
pub const UCHAR_SCRIPT_EXTENSIONS: UProperty = UProperty(28672i32);
pub const UCHAR_OTHER_PROPERTY_START: UProperty = UProperty(28672i32);
pub const UCHAR_INVALID_CODE: UProperty = UProperty(-1i32);
impl ::core::marker::Copy for UProperty {}
impl ::core::clone::Clone for UProperty {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UProperty {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UProperty").field(&self.0).finish()
    }
}
impl FromIntoMemory for UProperty {
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
pub struct UPropertyNameChoice(pub i32);
pub const U_SHORT_PROPERTY_NAME: UPropertyNameChoice = UPropertyNameChoice(0i32);
pub const U_LONG_PROPERTY_NAME: UPropertyNameChoice = UPropertyNameChoice(1i32);
impl ::core::marker::Copy for UPropertyNameChoice {}
impl ::core::clone::Clone for UPropertyNameChoice {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UPropertyNameChoice {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UPropertyNameChoice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UPropertyNameChoice").field(&self.0).finish()
    }
}
impl FromIntoMemory for UPropertyNameChoice {
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
pub type URegexFindProgressCallback = StdCallFnPtr<(ConstPtr<::core::ffi::c_void>, i64), i8>;
pub type URegexMatchCallback = StdCallFnPtr<(ConstPtr<::core::ffi::c_void>, i32), i8>;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct URegexpFlag(pub i32);
pub const UREGEX_CASE_INSENSITIVE: URegexpFlag = URegexpFlag(2i32);
pub const UREGEX_COMMENTS: URegexpFlag = URegexpFlag(4i32);
pub const UREGEX_DOTALL: URegexpFlag = URegexpFlag(32i32);
pub const UREGEX_LITERAL: URegexpFlag = URegexpFlag(16i32);
pub const UREGEX_MULTILINE: URegexpFlag = URegexpFlag(8i32);
pub const UREGEX_UNIX_LINES: URegexpFlag = URegexpFlag(1i32);
pub const UREGEX_UWORD: URegexpFlag = URegexpFlag(256i32);
pub const UREGEX_ERROR_ON_UNKNOWN_ESCAPES: URegexpFlag = URegexpFlag(512i32);
impl ::core::marker::Copy for URegexpFlag {}
impl ::core::clone::Clone for URegexpFlag {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for URegexpFlag {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for URegexpFlag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("URegexpFlag").field(&self.0).finish()
    }
}
impl FromIntoMemory for URegexpFlag {
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
pub struct URegion(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct URegionType(pub i32);
pub const URGN_UNKNOWN: URegionType = URegionType(0i32);
pub const URGN_TERRITORY: URegionType = URegionType(1i32);
pub const URGN_WORLD: URegionType = URegionType(2i32);
pub const URGN_CONTINENT: URegionType = URegionType(3i32);
pub const URGN_SUBCONTINENT: URegionType = URegionType(4i32);
pub const URGN_GROUPING: URegionType = URegionType(5i32);
pub const URGN_DEPRECATED: URegionType = URegionType(6i32);
impl ::core::marker::Copy for URegionType {}
impl ::core::clone::Clone for URegionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for URegionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for URegionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("URegionType").field(&self.0).finish()
    }
}
impl FromIntoMemory for URegionType {
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
pub struct URegularExpression(pub u8);
pub struct URelativeDateTimeFormatter(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct URelativeDateTimeFormatterField(pub i32);
pub const UDAT_REL_LITERAL_FIELD: URelativeDateTimeFormatterField =
    URelativeDateTimeFormatterField(0i32);
pub const UDAT_REL_NUMERIC_FIELD: URelativeDateTimeFormatterField =
    URelativeDateTimeFormatterField(1i32);
impl ::core::marker::Copy for URelativeDateTimeFormatterField {}
impl ::core::clone::Clone for URelativeDateTimeFormatterField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for URelativeDateTimeFormatterField {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for URelativeDateTimeFormatterField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("URelativeDateTimeFormatterField")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for URelativeDateTimeFormatterField {
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
pub struct URelativeDateTimeUnit(pub i32);
pub const UDAT_REL_UNIT_YEAR: URelativeDateTimeUnit = URelativeDateTimeUnit(0i32);
pub const UDAT_REL_UNIT_QUARTER: URelativeDateTimeUnit = URelativeDateTimeUnit(1i32);
pub const UDAT_REL_UNIT_MONTH: URelativeDateTimeUnit = URelativeDateTimeUnit(2i32);
pub const UDAT_REL_UNIT_WEEK: URelativeDateTimeUnit = URelativeDateTimeUnit(3i32);
pub const UDAT_REL_UNIT_DAY: URelativeDateTimeUnit = URelativeDateTimeUnit(4i32);
pub const UDAT_REL_UNIT_HOUR: URelativeDateTimeUnit = URelativeDateTimeUnit(5i32);
pub const UDAT_REL_UNIT_MINUTE: URelativeDateTimeUnit = URelativeDateTimeUnit(6i32);
pub const UDAT_REL_UNIT_SECOND: URelativeDateTimeUnit = URelativeDateTimeUnit(7i32);
pub const UDAT_REL_UNIT_SUNDAY: URelativeDateTimeUnit = URelativeDateTimeUnit(8i32);
pub const UDAT_REL_UNIT_MONDAY: URelativeDateTimeUnit = URelativeDateTimeUnit(9i32);
pub const UDAT_REL_UNIT_TUESDAY: URelativeDateTimeUnit = URelativeDateTimeUnit(10i32);
pub const UDAT_REL_UNIT_WEDNESDAY: URelativeDateTimeUnit = URelativeDateTimeUnit(11i32);
pub const UDAT_REL_UNIT_THURSDAY: URelativeDateTimeUnit = URelativeDateTimeUnit(12i32);
pub const UDAT_REL_UNIT_FRIDAY: URelativeDateTimeUnit = URelativeDateTimeUnit(13i32);
pub const UDAT_REL_UNIT_SATURDAY: URelativeDateTimeUnit = URelativeDateTimeUnit(14i32);
impl ::core::marker::Copy for URelativeDateTimeUnit {}
impl ::core::clone::Clone for URelativeDateTimeUnit {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for URelativeDateTimeUnit {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for URelativeDateTimeUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("URelativeDateTimeUnit")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for URelativeDateTimeUnit {
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
pub struct UReplaceableCallbacks {
    pub length: PtrDiffRepr,
    pub charAt: PtrDiffRepr,
    pub char32At: PtrDiffRepr,
    pub replace: PtrDiffRepr,
    pub extract: PtrDiffRepr,
    pub copy: PtrDiffRepr,
}
impl ::core::marker::Copy for UReplaceableCallbacks {}
impl ::core::clone::Clone for UReplaceableCallbacks {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UReplaceableCallbacks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UReplaceableCallbacks")
            .field("length", &self.length)
            .field("charAt", &self.charAt)
            .field("char32At", &self.char32At)
            .field("replace", &self.replace)
            .field("extract", &self.extract)
            .field("copy", &self.copy)
            .finish()
    }
}
impl ::core::cmp::PartialEq for UReplaceableCallbacks {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length
            && self.charAt == other.charAt
            && self.char32At == other.char32At
            && self.replace == other.replace
            && self.extract == other.extract
            && self.copy == other.copy
    }
}
impl ::core::cmp::Eq for UReplaceableCallbacks {}
impl FromIntoMemory for UReplaceableCallbacks {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24u32 as usize);
        let f_length = <PtrDiffRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_charAt = <PtrDiffRepr as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_char32At = <PtrDiffRepr as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_replace = <PtrDiffRepr as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_extract = <PtrDiffRepr as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_copy = <PtrDiffRepr as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            length: f_length,
            charAt: f_charAt,
            char32At: f_char32At,
            replace: f_replace,
            extract: f_extract,
            copy: f_copy,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24u32 as usize);
        FromIntoMemory::into_bytes(self.length, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.charAt, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.char32At, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.replace, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.extract, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.copy, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24u32 as usize
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UResType(pub i32);
pub const URES_NONE: UResType = UResType(-1i32);
pub const URES_STRING: UResType = UResType(0i32);
pub const URES_BINARY: UResType = UResType(1i32);
pub const URES_TABLE: UResType = UResType(2i32);
pub const URES_ALIAS: UResType = UResType(3i32);
pub const URES_INT: UResType = UResType(7i32);
pub const URES_ARRAY: UResType = UResType(8i32);
pub const URES_INT_VECTOR: UResType = UResType(14i32);
impl ::core::marker::Copy for UResType {}
impl ::core::clone::Clone for UResType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UResType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UResType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UResType").field(&self.0).finish()
    }
}
impl FromIntoMemory for UResType {
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
pub struct UResourceBundle(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct URestrictionLevel(pub i32);
pub const USPOOF_ASCII: URestrictionLevel = URestrictionLevel(268435456i32);
pub const USPOOF_SINGLE_SCRIPT_RESTRICTIVE: URestrictionLevel = URestrictionLevel(536870912i32);
pub const USPOOF_HIGHLY_RESTRICTIVE: URestrictionLevel = URestrictionLevel(805306368i32);
pub const USPOOF_MODERATELY_RESTRICTIVE: URestrictionLevel = URestrictionLevel(1073741824i32);
pub const USPOOF_MINIMALLY_RESTRICTIVE: URestrictionLevel = URestrictionLevel(1342177280i32);
pub const USPOOF_UNRESTRICTIVE: URestrictionLevel = URestrictionLevel(1610612736i32);
pub const USPOOF_RESTRICTION_LEVEL_MASK: URestrictionLevel = URestrictionLevel(2130706432i32);
impl ::core::marker::Copy for URestrictionLevel {}
impl ::core::clone::Clone for URestrictionLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for URestrictionLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for URestrictionLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("URestrictionLevel").field(&self.0).finish()
    }
}
impl FromIntoMemory for URestrictionLevel {
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
pub const USEARCH_DONE: i32 = -1i32;
pub const USET_ADD_CASE_MAPPINGS: i32 = 4i32;
pub const USET_CASE_INSENSITIVE: i32 = 2i32;
pub const USET_IGNORE_SPACE: i32 = 1i32;
pub const USET_SERIALIZED_STATIC_ARRAY_CAPACITY: i32 = 8i32;
pub const USPREP_ALLOW_UNASSIGNED: u32 = 1u32;
pub const USPREP_DEFAULT: u32 = 0u32;
pub const USP_E_SCRIPT_NOT_IN_FONT: crate::core::HRESULT = crate::core::HRESULT(-2147220992i32);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UScriptCode(pub i32);
pub const USCRIPT_INVALID_CODE: UScriptCode = UScriptCode(-1i32);
pub const USCRIPT_COMMON: UScriptCode = UScriptCode(0i32);
pub const USCRIPT_INHERITED: UScriptCode = UScriptCode(1i32);
pub const USCRIPT_ARABIC: UScriptCode = UScriptCode(2i32);
pub const USCRIPT_ARMENIAN: UScriptCode = UScriptCode(3i32);
pub const USCRIPT_BENGALI: UScriptCode = UScriptCode(4i32);
pub const USCRIPT_BOPOMOFO: UScriptCode = UScriptCode(5i32);
pub const USCRIPT_CHEROKEE: UScriptCode = UScriptCode(6i32);
pub const USCRIPT_COPTIC: UScriptCode = UScriptCode(7i32);
pub const USCRIPT_CYRILLIC: UScriptCode = UScriptCode(8i32);
pub const USCRIPT_DESERET: UScriptCode = UScriptCode(9i32);
pub const USCRIPT_DEVANAGARI: UScriptCode = UScriptCode(10i32);
pub const USCRIPT_ETHIOPIC: UScriptCode = UScriptCode(11i32);
pub const USCRIPT_GEORGIAN: UScriptCode = UScriptCode(12i32);
pub const USCRIPT_GOTHIC: UScriptCode = UScriptCode(13i32);
pub const USCRIPT_GREEK: UScriptCode = UScriptCode(14i32);
pub const USCRIPT_GUJARATI: UScriptCode = UScriptCode(15i32);
pub const USCRIPT_GURMUKHI: UScriptCode = UScriptCode(16i32);
pub const USCRIPT_HAN: UScriptCode = UScriptCode(17i32);
pub const USCRIPT_HANGUL: UScriptCode = UScriptCode(18i32);
pub const USCRIPT_HEBREW: UScriptCode = UScriptCode(19i32);
pub const USCRIPT_HIRAGANA: UScriptCode = UScriptCode(20i32);
pub const USCRIPT_KANNADA: UScriptCode = UScriptCode(21i32);
pub const USCRIPT_KATAKANA: UScriptCode = UScriptCode(22i32);
pub const USCRIPT_KHMER: UScriptCode = UScriptCode(23i32);
pub const USCRIPT_LAO: UScriptCode = UScriptCode(24i32);
pub const USCRIPT_LATIN: UScriptCode = UScriptCode(25i32);
pub const USCRIPT_MALAYALAM: UScriptCode = UScriptCode(26i32);
pub const USCRIPT_MONGOLIAN: UScriptCode = UScriptCode(27i32);
pub const USCRIPT_MYANMAR: UScriptCode = UScriptCode(28i32);
pub const USCRIPT_OGHAM: UScriptCode = UScriptCode(29i32);
pub const USCRIPT_OLD_ITALIC: UScriptCode = UScriptCode(30i32);
pub const USCRIPT_ORIYA: UScriptCode = UScriptCode(31i32);
pub const USCRIPT_RUNIC: UScriptCode = UScriptCode(32i32);
pub const USCRIPT_SINHALA: UScriptCode = UScriptCode(33i32);
pub const USCRIPT_SYRIAC: UScriptCode = UScriptCode(34i32);
pub const USCRIPT_TAMIL: UScriptCode = UScriptCode(35i32);
pub const USCRIPT_TELUGU: UScriptCode = UScriptCode(36i32);
pub const USCRIPT_THAANA: UScriptCode = UScriptCode(37i32);
pub const USCRIPT_THAI: UScriptCode = UScriptCode(38i32);
pub const USCRIPT_TIBETAN: UScriptCode = UScriptCode(39i32);
pub const USCRIPT_CANADIAN_ABORIGINAL: UScriptCode = UScriptCode(40i32);
pub const USCRIPT_UCAS: UScriptCode = UScriptCode(40i32);
pub const USCRIPT_YI: UScriptCode = UScriptCode(41i32);
pub const USCRIPT_TAGALOG: UScriptCode = UScriptCode(42i32);
pub const USCRIPT_HANUNOO: UScriptCode = UScriptCode(43i32);
pub const USCRIPT_BUHID: UScriptCode = UScriptCode(44i32);
pub const USCRIPT_TAGBANWA: UScriptCode = UScriptCode(45i32);
pub const USCRIPT_BRAILLE: UScriptCode = UScriptCode(46i32);
pub const USCRIPT_CYPRIOT: UScriptCode = UScriptCode(47i32);
pub const USCRIPT_LIMBU: UScriptCode = UScriptCode(48i32);
pub const USCRIPT_LINEAR_B: UScriptCode = UScriptCode(49i32);
pub const USCRIPT_OSMANYA: UScriptCode = UScriptCode(50i32);
pub const USCRIPT_SHAVIAN: UScriptCode = UScriptCode(51i32);
pub const USCRIPT_TAI_LE: UScriptCode = UScriptCode(52i32);
pub const USCRIPT_UGARITIC: UScriptCode = UScriptCode(53i32);
pub const USCRIPT_KATAKANA_OR_HIRAGANA: UScriptCode = UScriptCode(54i32);
pub const USCRIPT_BUGINESE: UScriptCode = UScriptCode(55i32);
pub const USCRIPT_GLAGOLITIC: UScriptCode = UScriptCode(56i32);
pub const USCRIPT_KHAROSHTHI: UScriptCode = UScriptCode(57i32);
pub const USCRIPT_SYLOTI_NAGRI: UScriptCode = UScriptCode(58i32);
pub const USCRIPT_NEW_TAI_LUE: UScriptCode = UScriptCode(59i32);
pub const USCRIPT_TIFINAGH: UScriptCode = UScriptCode(60i32);
pub const USCRIPT_OLD_PERSIAN: UScriptCode = UScriptCode(61i32);
pub const USCRIPT_BALINESE: UScriptCode = UScriptCode(62i32);
pub const USCRIPT_BATAK: UScriptCode = UScriptCode(63i32);
pub const USCRIPT_BLISSYMBOLS: UScriptCode = UScriptCode(64i32);
pub const USCRIPT_BRAHMI: UScriptCode = UScriptCode(65i32);
pub const USCRIPT_CHAM: UScriptCode = UScriptCode(66i32);
pub const USCRIPT_CIRTH: UScriptCode = UScriptCode(67i32);
pub const USCRIPT_OLD_CHURCH_SLAVONIC_CYRILLIC: UScriptCode = UScriptCode(68i32);
pub const USCRIPT_DEMOTIC_EGYPTIAN: UScriptCode = UScriptCode(69i32);
pub const USCRIPT_HIERATIC_EGYPTIAN: UScriptCode = UScriptCode(70i32);
pub const USCRIPT_EGYPTIAN_HIEROGLYPHS: UScriptCode = UScriptCode(71i32);
pub const USCRIPT_KHUTSURI: UScriptCode = UScriptCode(72i32);
pub const USCRIPT_SIMPLIFIED_HAN: UScriptCode = UScriptCode(73i32);
pub const USCRIPT_TRADITIONAL_HAN: UScriptCode = UScriptCode(74i32);
pub const USCRIPT_PAHAWH_HMONG: UScriptCode = UScriptCode(75i32);
pub const USCRIPT_OLD_HUNGARIAN: UScriptCode = UScriptCode(76i32);
pub const USCRIPT_HARAPPAN_INDUS: UScriptCode = UScriptCode(77i32);
pub const USCRIPT_JAVANESE: UScriptCode = UScriptCode(78i32);
pub const USCRIPT_KAYAH_LI: UScriptCode = UScriptCode(79i32);
pub const USCRIPT_LATIN_FRAKTUR: UScriptCode = UScriptCode(80i32);
pub const USCRIPT_LATIN_GAELIC: UScriptCode = UScriptCode(81i32);
pub const USCRIPT_LEPCHA: UScriptCode = UScriptCode(82i32);
pub const USCRIPT_LINEAR_A: UScriptCode = UScriptCode(83i32);
pub const USCRIPT_MANDAIC: UScriptCode = UScriptCode(84i32);
pub const USCRIPT_MANDAEAN: UScriptCode = UScriptCode(84i32);
pub const USCRIPT_MAYAN_HIEROGLYPHS: UScriptCode = UScriptCode(85i32);
pub const USCRIPT_MEROITIC_HIEROGLYPHS: UScriptCode = UScriptCode(86i32);
pub const USCRIPT_MEROITIC: UScriptCode = UScriptCode(86i32);
pub const USCRIPT_NKO: UScriptCode = UScriptCode(87i32);
pub const USCRIPT_ORKHON: UScriptCode = UScriptCode(88i32);
pub const USCRIPT_OLD_PERMIC: UScriptCode = UScriptCode(89i32);
pub const USCRIPT_PHAGS_PA: UScriptCode = UScriptCode(90i32);
pub const USCRIPT_PHOENICIAN: UScriptCode = UScriptCode(91i32);
pub const USCRIPT_MIAO: UScriptCode = UScriptCode(92i32);
pub const USCRIPT_PHONETIC_POLLARD: UScriptCode = UScriptCode(92i32);
pub const USCRIPT_RONGORONGO: UScriptCode = UScriptCode(93i32);
pub const USCRIPT_SARATI: UScriptCode = UScriptCode(94i32);
pub const USCRIPT_ESTRANGELO_SYRIAC: UScriptCode = UScriptCode(95i32);
pub const USCRIPT_WESTERN_SYRIAC: UScriptCode = UScriptCode(96i32);
pub const USCRIPT_EASTERN_SYRIAC: UScriptCode = UScriptCode(97i32);
pub const USCRIPT_TENGWAR: UScriptCode = UScriptCode(98i32);
pub const USCRIPT_VAI: UScriptCode = UScriptCode(99i32);
pub const USCRIPT_VISIBLE_SPEECH: UScriptCode = UScriptCode(100i32);
pub const USCRIPT_CUNEIFORM: UScriptCode = UScriptCode(101i32);
pub const USCRIPT_UNWRITTEN_LANGUAGES: UScriptCode = UScriptCode(102i32);
pub const USCRIPT_UNKNOWN: UScriptCode = UScriptCode(103i32);
pub const USCRIPT_CARIAN: UScriptCode = UScriptCode(104i32);
pub const USCRIPT_JAPANESE: UScriptCode = UScriptCode(105i32);
pub const USCRIPT_LANNA: UScriptCode = UScriptCode(106i32);
pub const USCRIPT_LYCIAN: UScriptCode = UScriptCode(107i32);
pub const USCRIPT_LYDIAN: UScriptCode = UScriptCode(108i32);
pub const USCRIPT_OL_CHIKI: UScriptCode = UScriptCode(109i32);
pub const USCRIPT_REJANG: UScriptCode = UScriptCode(110i32);
pub const USCRIPT_SAURASHTRA: UScriptCode = UScriptCode(111i32);
pub const USCRIPT_SIGN_WRITING: UScriptCode = UScriptCode(112i32);
pub const USCRIPT_SUNDANESE: UScriptCode = UScriptCode(113i32);
pub const USCRIPT_MOON: UScriptCode = UScriptCode(114i32);
pub const USCRIPT_MEITEI_MAYEK: UScriptCode = UScriptCode(115i32);
pub const USCRIPT_IMPERIAL_ARAMAIC: UScriptCode = UScriptCode(116i32);
pub const USCRIPT_AVESTAN: UScriptCode = UScriptCode(117i32);
pub const USCRIPT_CHAKMA: UScriptCode = UScriptCode(118i32);
pub const USCRIPT_KOREAN: UScriptCode = UScriptCode(119i32);
pub const USCRIPT_KAITHI: UScriptCode = UScriptCode(120i32);
pub const USCRIPT_MANICHAEAN: UScriptCode = UScriptCode(121i32);
pub const USCRIPT_INSCRIPTIONAL_PAHLAVI: UScriptCode = UScriptCode(122i32);
pub const USCRIPT_PSALTER_PAHLAVI: UScriptCode = UScriptCode(123i32);
pub const USCRIPT_BOOK_PAHLAVI: UScriptCode = UScriptCode(124i32);
pub const USCRIPT_INSCRIPTIONAL_PARTHIAN: UScriptCode = UScriptCode(125i32);
pub const USCRIPT_SAMARITAN: UScriptCode = UScriptCode(126i32);
pub const USCRIPT_TAI_VIET: UScriptCode = UScriptCode(127i32);
pub const USCRIPT_MATHEMATICAL_NOTATION: UScriptCode = UScriptCode(128i32);
pub const USCRIPT_SYMBOLS: UScriptCode = UScriptCode(129i32);
pub const USCRIPT_BAMUM: UScriptCode = UScriptCode(130i32);
pub const USCRIPT_LISU: UScriptCode = UScriptCode(131i32);
pub const USCRIPT_NAKHI_GEBA: UScriptCode = UScriptCode(132i32);
pub const USCRIPT_OLD_SOUTH_ARABIAN: UScriptCode = UScriptCode(133i32);
pub const USCRIPT_BASSA_VAH: UScriptCode = UScriptCode(134i32);
pub const USCRIPT_DUPLOYAN: UScriptCode = UScriptCode(135i32);
pub const USCRIPT_ELBASAN: UScriptCode = UScriptCode(136i32);
pub const USCRIPT_GRANTHA: UScriptCode = UScriptCode(137i32);
pub const USCRIPT_KPELLE: UScriptCode = UScriptCode(138i32);
pub const USCRIPT_LOMA: UScriptCode = UScriptCode(139i32);
pub const USCRIPT_MENDE: UScriptCode = UScriptCode(140i32);
pub const USCRIPT_MEROITIC_CURSIVE: UScriptCode = UScriptCode(141i32);
pub const USCRIPT_OLD_NORTH_ARABIAN: UScriptCode = UScriptCode(142i32);
pub const USCRIPT_NABATAEAN: UScriptCode = UScriptCode(143i32);
pub const USCRIPT_PALMYRENE: UScriptCode = UScriptCode(144i32);
pub const USCRIPT_KHUDAWADI: UScriptCode = UScriptCode(145i32);
pub const USCRIPT_SINDHI: UScriptCode = UScriptCode(145i32);
pub const USCRIPT_WARANG_CITI: UScriptCode = UScriptCode(146i32);
pub const USCRIPT_AFAKA: UScriptCode = UScriptCode(147i32);
pub const USCRIPT_JURCHEN: UScriptCode = UScriptCode(148i32);
pub const USCRIPT_MRO: UScriptCode = UScriptCode(149i32);
pub const USCRIPT_NUSHU: UScriptCode = UScriptCode(150i32);
pub const USCRIPT_SHARADA: UScriptCode = UScriptCode(151i32);
pub const USCRIPT_SORA_SOMPENG: UScriptCode = UScriptCode(152i32);
pub const USCRIPT_TAKRI: UScriptCode = UScriptCode(153i32);
pub const USCRIPT_TANGUT: UScriptCode = UScriptCode(154i32);
pub const USCRIPT_WOLEAI: UScriptCode = UScriptCode(155i32);
pub const USCRIPT_ANATOLIAN_HIEROGLYPHS: UScriptCode = UScriptCode(156i32);
pub const USCRIPT_KHOJKI: UScriptCode = UScriptCode(157i32);
pub const USCRIPT_TIRHUTA: UScriptCode = UScriptCode(158i32);
pub const USCRIPT_CAUCASIAN_ALBANIAN: UScriptCode = UScriptCode(159i32);
pub const USCRIPT_MAHAJANI: UScriptCode = UScriptCode(160i32);
pub const USCRIPT_AHOM: UScriptCode = UScriptCode(161i32);
pub const USCRIPT_HATRAN: UScriptCode = UScriptCode(162i32);
pub const USCRIPT_MODI: UScriptCode = UScriptCode(163i32);
pub const USCRIPT_MULTANI: UScriptCode = UScriptCode(164i32);
pub const USCRIPT_PAU_CIN_HAU: UScriptCode = UScriptCode(165i32);
pub const USCRIPT_SIDDHAM: UScriptCode = UScriptCode(166i32);
pub const USCRIPT_ADLAM: UScriptCode = UScriptCode(167i32);
pub const USCRIPT_BHAIKSUKI: UScriptCode = UScriptCode(168i32);
pub const USCRIPT_MARCHEN: UScriptCode = UScriptCode(169i32);
pub const USCRIPT_NEWA: UScriptCode = UScriptCode(170i32);
pub const USCRIPT_OSAGE: UScriptCode = UScriptCode(171i32);
pub const USCRIPT_HAN_WITH_BOPOMOFO: UScriptCode = UScriptCode(172i32);
pub const USCRIPT_JAMO: UScriptCode = UScriptCode(173i32);
pub const USCRIPT_SYMBOLS_EMOJI: UScriptCode = UScriptCode(174i32);
pub const USCRIPT_MASARAM_GONDI: UScriptCode = UScriptCode(175i32);
pub const USCRIPT_SOYOMBO: UScriptCode = UScriptCode(176i32);
pub const USCRIPT_ZANABAZAR_SQUARE: UScriptCode = UScriptCode(177i32);
pub const USCRIPT_DOGRA: UScriptCode = UScriptCode(178i32);
pub const USCRIPT_GUNJALA_GONDI: UScriptCode = UScriptCode(179i32);
pub const USCRIPT_MAKASAR: UScriptCode = UScriptCode(180i32);
pub const USCRIPT_MEDEFAIDRIN: UScriptCode = UScriptCode(181i32);
pub const USCRIPT_HANIFI_ROHINGYA: UScriptCode = UScriptCode(182i32);
pub const USCRIPT_SOGDIAN: UScriptCode = UScriptCode(183i32);
pub const USCRIPT_OLD_SOGDIAN: UScriptCode = UScriptCode(184i32);
pub const USCRIPT_ELYMAIC: UScriptCode = UScriptCode(185i32);
pub const USCRIPT_NYIAKENG_PUACHUE_HMONG: UScriptCode = UScriptCode(186i32);
pub const USCRIPT_NANDINAGARI: UScriptCode = UScriptCode(187i32);
pub const USCRIPT_WANCHO: UScriptCode = UScriptCode(188i32);
pub const USCRIPT_CHORASMIAN: UScriptCode = UScriptCode(189i32);
pub const USCRIPT_DIVES_AKURU: UScriptCode = UScriptCode(190i32);
pub const USCRIPT_KHITAN_SMALL_SCRIPT: UScriptCode = UScriptCode(191i32);
pub const USCRIPT_YEZIDI: UScriptCode = UScriptCode(192i32);
impl ::core::marker::Copy for UScriptCode {}
impl ::core::clone::Clone for UScriptCode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UScriptCode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UScriptCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UScriptCode").field(&self.0).finish()
    }
}
impl FromIntoMemory for UScriptCode {
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
pub struct UScriptUsage(pub i32);
pub const USCRIPT_USAGE_NOT_ENCODED: UScriptUsage = UScriptUsage(0i32);
pub const USCRIPT_USAGE_UNKNOWN: UScriptUsage = UScriptUsage(1i32);
pub const USCRIPT_USAGE_EXCLUDED: UScriptUsage = UScriptUsage(2i32);
pub const USCRIPT_USAGE_LIMITED_USE: UScriptUsage = UScriptUsage(3i32);
pub const USCRIPT_USAGE_ASPIRATIONAL: UScriptUsage = UScriptUsage(4i32);
pub const USCRIPT_USAGE_RECOMMENDED: UScriptUsage = UScriptUsage(5i32);
impl ::core::marker::Copy for UScriptUsage {}
impl ::core::clone::Clone for UScriptUsage {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UScriptUsage {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UScriptUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UScriptUsage").field(&self.0).finish()
    }
}
impl FromIntoMemory for UScriptUsage {
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
pub struct USearch(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct USearchAttribute(pub i32);
pub const USEARCH_OVERLAP: USearchAttribute = USearchAttribute(0i32);
pub const USEARCH_ELEMENT_COMPARISON: USearchAttribute = USearchAttribute(2i32);
impl ::core::marker::Copy for USearchAttribute {}
impl ::core::clone::Clone for USearchAttribute {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for USearchAttribute {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USearchAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USearchAttribute").field(&self.0).finish()
    }
}
impl FromIntoMemory for USearchAttribute {
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
pub struct USearchAttributeValue(pub i32);
pub const USEARCH_DEFAULT: USearchAttributeValue = USearchAttributeValue(-1i32);
pub const USEARCH_OFF: USearchAttributeValue = USearchAttributeValue(0i32);
pub const USEARCH_ON: USearchAttributeValue = USearchAttributeValue(1i32);
pub const USEARCH_STANDARD_ELEMENT_COMPARISON: USearchAttributeValue = USearchAttributeValue(2i32);
pub const USEARCH_PATTERN_BASE_WEIGHT_IS_WILDCARD: USearchAttributeValue =
    USearchAttributeValue(3i32);
pub const USEARCH_ANY_BASE_WEIGHT_IS_WILDCARD: USearchAttributeValue = USearchAttributeValue(4i32);
impl ::core::marker::Copy for USearchAttributeValue {}
impl ::core::clone::Clone for USearchAttributeValue {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for USearchAttributeValue {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USearchAttributeValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USearchAttributeValue")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for USearchAttributeValue {
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
pub struct USentenceBreak(pub i32);
pub const U_SB_OTHER: USentenceBreak = USentenceBreak(0i32);
pub const U_SB_ATERM: USentenceBreak = USentenceBreak(1i32);
pub const U_SB_CLOSE: USentenceBreak = USentenceBreak(2i32);
pub const U_SB_FORMAT: USentenceBreak = USentenceBreak(3i32);
pub const U_SB_LOWER: USentenceBreak = USentenceBreak(4i32);
pub const U_SB_NUMERIC: USentenceBreak = USentenceBreak(5i32);
pub const U_SB_OLETTER: USentenceBreak = USentenceBreak(6i32);
pub const U_SB_SEP: USentenceBreak = USentenceBreak(7i32);
pub const U_SB_SP: USentenceBreak = USentenceBreak(8i32);
pub const U_SB_STERM: USentenceBreak = USentenceBreak(9i32);
pub const U_SB_UPPER: USentenceBreak = USentenceBreak(10i32);
pub const U_SB_CR: USentenceBreak = USentenceBreak(11i32);
pub const U_SB_EXTEND: USentenceBreak = USentenceBreak(12i32);
pub const U_SB_LF: USentenceBreak = USentenceBreak(13i32);
pub const U_SB_SCONTINUE: USentenceBreak = USentenceBreak(14i32);
impl ::core::marker::Copy for USentenceBreak {}
impl ::core::clone::Clone for USentenceBreak {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for USentenceBreak {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USentenceBreak {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USentenceBreak").field(&self.0).finish()
    }
}
impl FromIntoMemory for USentenceBreak {
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
pub struct USentenceBreakTag(pub i32);
pub const UBRK_SENTENCE_TERM: USentenceBreakTag = USentenceBreakTag(0i32);
pub const UBRK_SENTENCE_TERM_LIMIT: USentenceBreakTag = USentenceBreakTag(100i32);
pub const UBRK_SENTENCE_SEP: USentenceBreakTag = USentenceBreakTag(100i32);
pub const UBRK_SENTENCE_SEP_LIMIT: USentenceBreakTag = USentenceBreakTag(200i32);
impl ::core::marker::Copy for USentenceBreakTag {}
impl ::core::clone::Clone for USentenceBreakTag {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for USentenceBreakTag {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USentenceBreakTag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USentenceBreakTag").field(&self.0).finish()
    }
}
impl FromIntoMemory for USentenceBreakTag {
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
pub struct USerializedSet {
    pub array: ConstPtr<u16>,
    pub bmpLength: i32,
    pub length: i32,
    pub staticArray: [u16; 8],
}
impl ::core::marker::Copy for USerializedSet {}
impl ::core::clone::Clone for USerializedSet {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USerializedSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USerializedSet")
            .field("array", &self.array)
            .field("bmpLength", &self.bmpLength)
            .field("length", &self.length)
            .field("staticArray", &self.staticArray)
            .finish()
    }
}
impl ::core::cmp::PartialEq for USerializedSet {
    fn eq(&self, other: &Self) -> bool {
        self.array == other.array
            && self.bmpLength == other.bmpLength
            && self.length == other.length
            && self.staticArray == other.staticArray
    }
}
impl ::core::cmp::Eq for USerializedSet {}
impl FromIntoMemory for USerializedSet {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28u32 as usize);
        let f_array = <ConstPtr<u16> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_bmpLength = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_length = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_staticArray = <[u16; 8] as FromIntoMemory>::from_bytes(&from[12..12 + 16]);
        Self {
            array: f_array,
            bmpLength: f_bmpLength,
            length: f_length,
            staticArray: f_staticArray,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28u32 as usize);
        FromIntoMemory::into_bytes(self.array, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.bmpLength, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.length, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.staticArray, &mut into[12..12 + 16]);
    }
    fn size() -> usize {
        28u32 as usize
    }
}
pub struct USet(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct USetSpanCondition(pub i32);
pub const USET_SPAN_NOT_CONTAINED: USetSpanCondition = USetSpanCondition(0i32);
pub const USET_SPAN_CONTAINED: USetSpanCondition = USetSpanCondition(1i32);
pub const USET_SPAN_SIMPLE: USetSpanCondition = USetSpanCondition(2i32);
impl ::core::marker::Copy for USetSpanCondition {}
impl ::core::clone::Clone for USetSpanCondition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for USetSpanCondition {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USetSpanCondition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USetSpanCondition").field(&self.0).finish()
    }
}
impl FromIntoMemory for USetSpanCondition {
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
pub struct USpoofCheckResult(pub u8);
pub struct USpoofChecker(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct USpoofChecks(pub i32);
pub const USPOOF_SINGLE_SCRIPT_CONFUSABLE: USpoofChecks = USpoofChecks(1i32);
pub const USPOOF_MIXED_SCRIPT_CONFUSABLE: USpoofChecks = USpoofChecks(2i32);
pub const USPOOF_WHOLE_SCRIPT_CONFUSABLE: USpoofChecks = USpoofChecks(4i32);
pub const USPOOF_CONFUSABLE: USpoofChecks = USpoofChecks(7i32);
pub const USPOOF_RESTRICTION_LEVEL: USpoofChecks = USpoofChecks(16i32);
pub const USPOOF_INVISIBLE: USpoofChecks = USpoofChecks(32i32);
pub const USPOOF_CHAR_LIMIT: USpoofChecks = USpoofChecks(64i32);
pub const USPOOF_MIXED_NUMBERS: USpoofChecks = USpoofChecks(128i32);
pub const USPOOF_HIDDEN_OVERLAY: USpoofChecks = USpoofChecks(256i32);
pub const USPOOF_ALL_CHECKS: USpoofChecks = USpoofChecks(65535i32);
pub const USPOOF_AUX_INFO: USpoofChecks = USpoofChecks(1073741824i32);
impl ::core::marker::Copy for USpoofChecks {}
impl ::core::clone::Clone for USpoofChecks {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for USpoofChecks {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USpoofChecks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USpoofChecks").field(&self.0).finish()
    }
}
impl FromIntoMemory for USpoofChecks {
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
pub type UStringCaseMapper = StdCallFnPtr<
    (
        ConstPtr<UCaseMap>,
        MutPtr<u16>,
        i32,
        ConstPtr<u16>,
        i32,
        MutPtr<UErrorCode>,
    ),
    i32,
>;
pub struct UStringPrepProfile(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UStringPrepProfileType(pub i32);
pub const USPREP_RFC3491_NAMEPREP: UStringPrepProfileType = UStringPrepProfileType(0i32);
pub const USPREP_RFC3530_NFS4_CS_PREP: UStringPrepProfileType = UStringPrepProfileType(1i32);
pub const USPREP_RFC3530_NFS4_CS_PREP_CI: UStringPrepProfileType = UStringPrepProfileType(2i32);
pub const USPREP_RFC3530_NFS4_CIS_PREP: UStringPrepProfileType = UStringPrepProfileType(3i32);
pub const USPREP_RFC3530_NFS4_MIXED_PREP_PREFIX: UStringPrepProfileType =
    UStringPrepProfileType(4i32);
pub const USPREP_RFC3530_NFS4_MIXED_PREP_SUFFIX: UStringPrepProfileType =
    UStringPrepProfileType(5i32);
pub const USPREP_RFC3722_ISCSI: UStringPrepProfileType = UStringPrepProfileType(6i32);
pub const USPREP_RFC3920_NODEPREP: UStringPrepProfileType = UStringPrepProfileType(7i32);
pub const USPREP_RFC3920_RESOURCEPREP: UStringPrepProfileType = UStringPrepProfileType(8i32);
pub const USPREP_RFC4011_MIB: UStringPrepProfileType = UStringPrepProfileType(9i32);
pub const USPREP_RFC4013_SASLPREP: UStringPrepProfileType = UStringPrepProfileType(10i32);
pub const USPREP_RFC4505_TRACE: UStringPrepProfileType = UStringPrepProfileType(11i32);
pub const USPREP_RFC4518_LDAP: UStringPrepProfileType = UStringPrepProfileType(12i32);
pub const USPREP_RFC4518_LDAP_CI: UStringPrepProfileType = UStringPrepProfileType(13i32);
impl ::core::marker::Copy for UStringPrepProfileType {}
impl ::core::clone::Clone for UStringPrepProfileType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UStringPrepProfileType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UStringPrepProfileType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UStringPrepProfileType")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UStringPrepProfileType {
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
pub struct UStringSearch(pub u8);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UStringTrieBuildOption(pub i32);
pub const USTRINGTRIE_BUILD_FAST: UStringTrieBuildOption = UStringTrieBuildOption(0i32);
pub const USTRINGTRIE_BUILD_SMALL: UStringTrieBuildOption = UStringTrieBuildOption(1i32);
impl ::core::marker::Copy for UStringTrieBuildOption {}
impl ::core::clone::Clone for UStringTrieBuildOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UStringTrieBuildOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UStringTrieBuildOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UStringTrieBuildOption")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UStringTrieBuildOption {
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
pub struct UStringTrieResult(pub i32);
pub const USTRINGTRIE_NO_MATCH: UStringTrieResult = UStringTrieResult(0i32);
pub const USTRINGTRIE_NO_VALUE: UStringTrieResult = UStringTrieResult(1i32);
pub const USTRINGTRIE_FINAL_VALUE: UStringTrieResult = UStringTrieResult(2i32);
pub const USTRINGTRIE_INTERMEDIATE_VALUE: UStringTrieResult = UStringTrieResult(3i32);
impl ::core::marker::Copy for UStringTrieResult {}
impl ::core::clone::Clone for UStringTrieResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UStringTrieResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UStringTrieResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UStringTrieResult").field(&self.0).finish()
    }
}
impl FromIntoMemory for UStringTrieResult {
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
pub struct USystemTimeZoneType(pub i32);
pub const UCAL_ZONE_TYPE_ANY: USystemTimeZoneType = USystemTimeZoneType(0i32);
pub const UCAL_ZONE_TYPE_CANONICAL: USystemTimeZoneType = USystemTimeZoneType(1i32);
pub const UCAL_ZONE_TYPE_CANONICAL_LOCATION: USystemTimeZoneType = USystemTimeZoneType(2i32);
impl ::core::marker::Copy for USystemTimeZoneType {}
impl ::core::clone::Clone for USystemTimeZoneType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for USystemTimeZoneType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USystemTimeZoneType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USystemTimeZoneType").field(&self.0).finish()
    }
}
impl FromIntoMemory for USystemTimeZoneType {
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
pub const UTEXT_MAGIC: i32 = 878368812i32;
pub const UTEXT_PROVIDER_HAS_META_DATA: i32 = 4i32;
pub const UTEXT_PROVIDER_LENGTH_IS_EXPENSIVE: i32 = 1i32;
pub const UTEXT_PROVIDER_OWNS_TEXT: i32 = 5i32;
pub const UTEXT_PROVIDER_STABLE_CHUNKS: i32 = 2i32;
pub const UTEXT_PROVIDER_WRITABLE: i32 = 3i32;
pub const UTF16_MAX_CHAR_LENGTH: u32 = 2u32;
pub const UTF32_MAX_CHAR_LENGTH: u32 = 1u32;
pub const UTF8_ERROR_VALUE_1: u32 = 21u32;
pub const UTF8_ERROR_VALUE_2: u32 = 159u32;
pub const UTF8_MAX_CHAR_LENGTH: u32 = 4u32;
pub const UTF_ERROR_VALUE: u32 = 65535u32;
pub const UTF_MAX_CHAR_LENGTH: u32 = 2u32;
pub const UTF_SIZE: u32 = 16u32;
pub struct UText {
    pub magic: u32,
    pub flags: i32,
    pub providerProperties: i32,
    pub sizeOfStruct: i32,
    pub chunkNativeLimit: i64,
    pub extraSize: i32,
    pub nativeIndexingLimit: i32,
    pub chunkNativeStart: i64,
    pub chunkOffset: i32,
    pub chunkLength: i32,
    pub chunkContents: ConstPtr<u16>,
    pub pFuncs: ConstPtr<UTextFuncs>,
    pub pExtra: MutPtr<::core::ffi::c_void>,
    pub context: ConstPtr<::core::ffi::c_void>,
    pub p: ConstPtr<::core::ffi::c_void>,
    pub q: ConstPtr<::core::ffi::c_void>,
    pub r: ConstPtr<::core::ffi::c_void>,
    pub privP: MutPtr<::core::ffi::c_void>,
    pub a: i64,
    pub b: i32,
    pub c: i32,
    pub privA: i64,
    pub privB: i32,
    pub privC: i32,
}
impl ::core::marker::Copy for UText {}
impl ::core::clone::Clone for UText {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UText")
            .field("magic", &self.magic)
            .field("flags", &self.flags)
            .field("providerProperties", &self.providerProperties)
            .field("sizeOfStruct", &self.sizeOfStruct)
            .field("chunkNativeLimit", &self.chunkNativeLimit)
            .field("extraSize", &self.extraSize)
            .field("nativeIndexingLimit", &self.nativeIndexingLimit)
            .field("chunkNativeStart", &self.chunkNativeStart)
            .field("chunkOffset", &self.chunkOffset)
            .field("chunkLength", &self.chunkLength)
            .field("chunkContents", &self.chunkContents)
            .field("pFuncs", &self.pFuncs)
            .field("pExtra", &self.pExtra)
            .field("context", &self.context)
            .field("p", &self.p)
            .field("q", &self.q)
            .field("r", &self.r)
            .field("privP", &self.privP)
            .field("a", &self.a)
            .field("b", &self.b)
            .field("c", &self.c)
            .field("privA", &self.privA)
            .field("privB", &self.privB)
            .field("privC", &self.privC)
            .finish()
    }
}
impl ::core::cmp::PartialEq for UText {
    fn eq(&self, other: &Self) -> bool {
        self.magic == other.magic
            && self.flags == other.flags
            && self.providerProperties == other.providerProperties
            && self.sizeOfStruct == other.sizeOfStruct
            && self.chunkNativeLimit == other.chunkNativeLimit
            && self.extraSize == other.extraSize
            && self.nativeIndexingLimit == other.nativeIndexingLimit
            && self.chunkNativeStart == other.chunkNativeStart
            && self.chunkOffset == other.chunkOffset
            && self.chunkLength == other.chunkLength
            && self.chunkContents == other.chunkContents
            && self.pFuncs == other.pFuncs
            && self.pExtra == other.pExtra
            && self.context == other.context
            && self.p == other.p
            && self.q == other.q
            && self.r == other.r
            && self.privP == other.privP
            && self.a == other.a
            && self.b == other.b
            && self.c == other.c
            && self.privA == other.privA
            && self.privB == other.privB
            && self.privC == other.privC
    }
}
impl ::core::cmp::Eq for UText {}
impl FromIntoMemory for UText {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 112u32 as usize);
        let f_magic = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_flags = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_providerProperties = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_sizeOfStruct = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_chunkNativeLimit = <i64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_extraSize = <i32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_nativeIndexingLimit = <i32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_chunkNativeStart = <i64 as FromIntoMemory>::from_bytes(&from[32..32 + 8]);
        let f_chunkOffset = <i32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_chunkLength = <i32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_chunkContents = <ConstPtr<u16> as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_pFuncs = <ConstPtr<UTextFuncs> as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_pExtra =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_context =
            <ConstPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_p = <ConstPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        let f_q = <ConstPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[68..68 + 4]);
        let f_r = <ConstPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[72..72 + 4]);
        let f_privP =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[76..76 + 4]);
        let f_a = <i64 as FromIntoMemory>::from_bytes(&from[80..80 + 8]);
        let f_b = <i32 as FromIntoMemory>::from_bytes(&from[88..88 + 4]);
        let f_c = <i32 as FromIntoMemory>::from_bytes(&from[92..92 + 4]);
        let f_privA = <i64 as FromIntoMemory>::from_bytes(&from[96..96 + 8]);
        let f_privB = <i32 as FromIntoMemory>::from_bytes(&from[104..104 + 4]);
        let f_privC = <i32 as FromIntoMemory>::from_bytes(&from[108..108 + 4]);
        Self {
            magic: f_magic,
            flags: f_flags,
            providerProperties: f_providerProperties,
            sizeOfStruct: f_sizeOfStruct,
            chunkNativeLimit: f_chunkNativeLimit,
            extraSize: f_extraSize,
            nativeIndexingLimit: f_nativeIndexingLimit,
            chunkNativeStart: f_chunkNativeStart,
            chunkOffset: f_chunkOffset,
            chunkLength: f_chunkLength,
            chunkContents: f_chunkContents,
            pFuncs: f_pFuncs,
            pExtra: f_pExtra,
            context: f_context,
            p: f_p,
            q: f_q,
            r: f_r,
            privP: f_privP,
            a: f_a,
            b: f_b,
            c: f_c,
            privA: f_privA,
            privB: f_privB,
            privC: f_privC,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 112u32 as usize);
        FromIntoMemory::into_bytes(self.magic, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.flags, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.providerProperties, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.sizeOfStruct, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.chunkNativeLimit, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.extraSize, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.nativeIndexingLimit, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.chunkNativeStart, &mut into[32..32 + 8]);
        FromIntoMemory::into_bytes(self.chunkOffset, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.chunkLength, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.chunkContents, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.pFuncs, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.pExtra, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.context, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.p, &mut into[64..64 + 4]);
        FromIntoMemory::into_bytes(self.q, &mut into[68..68 + 4]);
        FromIntoMemory::into_bytes(self.r, &mut into[72..72 + 4]);
        FromIntoMemory::into_bytes(self.privP, &mut into[76..76 + 4]);
        FromIntoMemory::into_bytes(self.a, &mut into[80..80 + 8]);
        FromIntoMemory::into_bytes(self.b, &mut into[88..88 + 4]);
        FromIntoMemory::into_bytes(self.c, &mut into[92..92 + 4]);
        FromIntoMemory::into_bytes(self.privA, &mut into[96..96 + 8]);
        FromIntoMemory::into_bytes(self.privB, &mut into[104..104 + 4]);
        FromIntoMemory::into_bytes(self.privC, &mut into[108..108 + 4]);
    }
    fn size() -> usize {
        112u32 as usize
    }
}
pub type UTextAccess = StdCallFnPtr<(MutPtr<UText>, i64, i8), i8>;
pub type UTextClone =
    StdCallFnPtr<(MutPtr<UText>, ConstPtr<UText>, i8, MutPtr<UErrorCode>), MutPtr<UText>>;
pub type UTextClose = StdCallFnPtr<(MutPtr<UText>,), ()>;
pub type UTextCopy = StdCallFnPtr<(MutPtr<UText>, i64, i64, i64, i8, MutPtr<UErrorCode>), ()>;
pub type UTextExtract = StdCallFnPtr<
    (
        MutPtr<UText>,
        i64,
        i64,
        MutPtr<u16>,
        i32,
        MutPtr<UErrorCode>,
    ),
    i32,
>;
pub struct UTextFuncs {
    pub tableSize: i32,
    pub reserved1: i32,
    pub reserved2: i32,
    pub reserved3: i32,
    pub clone: UTextClone,
    pub nativeLength: UTextNativeLength,
    pub access: UTextAccess,
    pub extract: UTextExtract,
    pub replace: UTextReplace,
    pub copy: UTextCopy,
    pub mapOffsetToNative: UTextMapOffsetToNative,
    pub mapNativeIndexToUTF16: UTextMapNativeIndexToUTF16,
    pub close: UTextClose,
    pub spare1: UTextClose,
    pub spare2: UTextClose,
    pub spare3: UTextClose,
}
impl ::core::marker::Copy for UTextFuncs {}
impl ::core::clone::Clone for UTextFuncs {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UTextFuncs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UTextFuncs")
            .field("tableSize", &self.tableSize)
            .field("reserved1", &self.reserved1)
            .field("reserved2", &self.reserved2)
            .field("reserved3", &self.reserved3)
            .field("clone", &self.clone)
            .field("nativeLength", &self.nativeLength)
            .field("access", &self.access)
            .field("extract", &self.extract)
            .field("replace", &self.replace)
            .field("copy", &self.copy)
            .field("mapOffsetToNative", &self.mapOffsetToNative)
            .field("mapNativeIndexToUTF16", &self.mapNativeIndexToUTF16)
            .field("close", &self.close)
            .field("spare1", &self.spare1)
            .field("spare2", &self.spare2)
            .field("spare3", &self.spare3)
            .finish()
    }
}
impl ::core::cmp::PartialEq for UTextFuncs {
    fn eq(&self, other: &Self) -> bool {
        self.tableSize == other.tableSize
            && self.reserved1 == other.reserved1
            && self.reserved2 == other.reserved2
            && self.reserved3 == other.reserved3
            && self.clone == other.clone
            && self.nativeLength == other.nativeLength
            && self.access == other.access
            && self.extract == other.extract
            && self.replace == other.replace
            && self.copy == other.copy
            && self.mapOffsetToNative == other.mapOffsetToNative
            && self.mapNativeIndexToUTF16 == other.mapNativeIndexToUTF16
            && self.close == other.close
            && self.spare1 == other.spare1
            && self.spare2 == other.spare2
            && self.spare3 == other.spare3
    }
}
impl ::core::cmp::Eq for UTextFuncs {}
impl FromIntoMemory for UTextFuncs {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 64u32 as usize);
        let f_tableSize = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_reserved1 = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_reserved2 = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_reserved3 = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_clone = <UTextClone as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_nativeLength = <UTextNativeLength as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_access = <UTextAccess as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_extract = <UTextExtract as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_replace = <UTextReplace as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_copy = <UTextCopy as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_mapOffsetToNative =
            <UTextMapOffsetToNative as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_mapNativeIndexToUTF16 =
            <UTextMapNativeIndexToUTF16 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_close = <UTextClose as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_spare1 = <UTextClose as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_spare2 = <UTextClose as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_spare3 = <UTextClose as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        Self {
            tableSize: f_tableSize,
            reserved1: f_reserved1,
            reserved2: f_reserved2,
            reserved3: f_reserved3,
            clone: f_clone,
            nativeLength: f_nativeLength,
            access: f_access,
            extract: f_extract,
            replace: f_replace,
            copy: f_copy,
            mapOffsetToNative: f_mapOffsetToNative,
            mapNativeIndexToUTF16: f_mapNativeIndexToUTF16,
            close: f_close,
            spare1: f_spare1,
            spare2: f_spare2,
            spare3: f_spare3,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 64u32 as usize);
        FromIntoMemory::into_bytes(self.tableSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.reserved1, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.reserved2, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.reserved3, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.clone, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.nativeLength, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.access, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.extract, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.replace, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.copy, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.mapOffsetToNative, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.mapNativeIndexToUTF16, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.close, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.spare1, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.spare2, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.spare3, &mut into[60..60 + 4]);
    }
    fn size() -> usize {
        64u32 as usize
    }
}
pub type UTextMapNativeIndexToUTF16 = StdCallFnPtr<(ConstPtr<UText>, i64), i32>;
pub type UTextMapOffsetToNative = StdCallFnPtr<(ConstPtr<UText>,), i64>;
pub type UTextNativeLength = StdCallFnPtr<(MutPtr<UText>,), i64>;
pub type UTextReplace = StdCallFnPtr<
    (
        MutPtr<UText>,
        i64,
        i64,
        ConstPtr<u16>,
        i32,
        MutPtr<UErrorCode>,
    ),
    i32,
>;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UTimeScaleValue(pub i32);
pub const UTSV_UNITS_VALUE: UTimeScaleValue = UTimeScaleValue(0i32);
pub const UTSV_EPOCH_OFFSET_VALUE: UTimeScaleValue = UTimeScaleValue(1i32);
pub const UTSV_FROM_MIN_VALUE: UTimeScaleValue = UTimeScaleValue(2i32);
pub const UTSV_FROM_MAX_VALUE: UTimeScaleValue = UTimeScaleValue(3i32);
pub const UTSV_TO_MIN_VALUE: UTimeScaleValue = UTimeScaleValue(4i32);
pub const UTSV_TO_MAX_VALUE: UTimeScaleValue = UTimeScaleValue(5i32);
impl ::core::marker::Copy for UTimeScaleValue {}
impl ::core::clone::Clone for UTimeScaleValue {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UTimeScaleValue {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UTimeScaleValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UTimeScaleValue").field(&self.0).finish()
    }
}
impl FromIntoMemory for UTimeScaleValue {
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
pub struct UTimeZoneFormatGMTOffsetPatternType(pub i32);
pub const UTZFMT_PAT_POSITIVE_HM: UTimeZoneFormatGMTOffsetPatternType =
    UTimeZoneFormatGMTOffsetPatternType(0i32);
pub const UTZFMT_PAT_POSITIVE_HMS: UTimeZoneFormatGMTOffsetPatternType =
    UTimeZoneFormatGMTOffsetPatternType(1i32);
pub const UTZFMT_PAT_NEGATIVE_HM: UTimeZoneFormatGMTOffsetPatternType =
    UTimeZoneFormatGMTOffsetPatternType(2i32);
pub const UTZFMT_PAT_NEGATIVE_HMS: UTimeZoneFormatGMTOffsetPatternType =
    UTimeZoneFormatGMTOffsetPatternType(3i32);
pub const UTZFMT_PAT_POSITIVE_H: UTimeZoneFormatGMTOffsetPatternType =
    UTimeZoneFormatGMTOffsetPatternType(4i32);
pub const UTZFMT_PAT_NEGATIVE_H: UTimeZoneFormatGMTOffsetPatternType =
    UTimeZoneFormatGMTOffsetPatternType(5i32);
pub const UTZFMT_PAT_COUNT: UTimeZoneFormatGMTOffsetPatternType =
    UTimeZoneFormatGMTOffsetPatternType(6i32);
impl ::core::marker::Copy for UTimeZoneFormatGMTOffsetPatternType {}
impl ::core::clone::Clone for UTimeZoneFormatGMTOffsetPatternType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UTimeZoneFormatGMTOffsetPatternType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UTimeZoneFormatGMTOffsetPatternType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UTimeZoneFormatGMTOffsetPatternType")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UTimeZoneFormatGMTOffsetPatternType {
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
pub struct UTimeZoneFormatParseOption(pub i32);
pub const UTZFMT_PARSE_OPTION_NONE: UTimeZoneFormatParseOption = UTimeZoneFormatParseOption(0i32);
pub const UTZFMT_PARSE_OPTION_ALL_STYLES: UTimeZoneFormatParseOption =
    UTimeZoneFormatParseOption(1i32);
pub const UTZFMT_PARSE_OPTION_TZ_DATABASE_ABBREVIATIONS: UTimeZoneFormatParseOption =
    UTimeZoneFormatParseOption(2i32);
impl ::core::marker::Copy for UTimeZoneFormatParseOption {}
impl ::core::clone::Clone for UTimeZoneFormatParseOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UTimeZoneFormatParseOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UTimeZoneFormatParseOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UTimeZoneFormatParseOption")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UTimeZoneFormatParseOption {
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
pub struct UTimeZoneFormatStyle(pub i32);
pub const UTZFMT_STYLE_GENERIC_LOCATION: UTimeZoneFormatStyle = UTimeZoneFormatStyle(0i32);
pub const UTZFMT_STYLE_GENERIC_LONG: UTimeZoneFormatStyle = UTimeZoneFormatStyle(1i32);
pub const UTZFMT_STYLE_GENERIC_SHORT: UTimeZoneFormatStyle = UTimeZoneFormatStyle(2i32);
pub const UTZFMT_STYLE_SPECIFIC_LONG: UTimeZoneFormatStyle = UTimeZoneFormatStyle(3i32);
pub const UTZFMT_STYLE_SPECIFIC_SHORT: UTimeZoneFormatStyle = UTimeZoneFormatStyle(4i32);
pub const UTZFMT_STYLE_LOCALIZED_GMT: UTimeZoneFormatStyle = UTimeZoneFormatStyle(5i32);
pub const UTZFMT_STYLE_LOCALIZED_GMT_SHORT: UTimeZoneFormatStyle = UTimeZoneFormatStyle(6i32);
pub const UTZFMT_STYLE_ISO_BASIC_SHORT: UTimeZoneFormatStyle = UTimeZoneFormatStyle(7i32);
pub const UTZFMT_STYLE_ISO_BASIC_LOCAL_SHORT: UTimeZoneFormatStyle = UTimeZoneFormatStyle(8i32);
pub const UTZFMT_STYLE_ISO_BASIC_FIXED: UTimeZoneFormatStyle = UTimeZoneFormatStyle(9i32);
pub const UTZFMT_STYLE_ISO_BASIC_LOCAL_FIXED: UTimeZoneFormatStyle = UTimeZoneFormatStyle(10i32);
pub const UTZFMT_STYLE_ISO_BASIC_FULL: UTimeZoneFormatStyle = UTimeZoneFormatStyle(11i32);
pub const UTZFMT_STYLE_ISO_BASIC_LOCAL_FULL: UTimeZoneFormatStyle = UTimeZoneFormatStyle(12i32);
pub const UTZFMT_STYLE_ISO_EXTENDED_FIXED: UTimeZoneFormatStyle = UTimeZoneFormatStyle(13i32);
pub const UTZFMT_STYLE_ISO_EXTENDED_LOCAL_FIXED: UTimeZoneFormatStyle = UTimeZoneFormatStyle(14i32);
pub const UTZFMT_STYLE_ISO_EXTENDED_FULL: UTimeZoneFormatStyle = UTimeZoneFormatStyle(15i32);
pub const UTZFMT_STYLE_ISO_EXTENDED_LOCAL_FULL: UTimeZoneFormatStyle = UTimeZoneFormatStyle(16i32);
pub const UTZFMT_STYLE_ZONE_ID: UTimeZoneFormatStyle = UTimeZoneFormatStyle(17i32);
pub const UTZFMT_STYLE_ZONE_ID_SHORT: UTimeZoneFormatStyle = UTimeZoneFormatStyle(18i32);
pub const UTZFMT_STYLE_EXEMPLAR_LOCATION: UTimeZoneFormatStyle = UTimeZoneFormatStyle(19i32);
impl ::core::marker::Copy for UTimeZoneFormatStyle {}
impl ::core::clone::Clone for UTimeZoneFormatStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UTimeZoneFormatStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UTimeZoneFormatStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UTimeZoneFormatStyle")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UTimeZoneFormatStyle {
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
pub struct UTimeZoneFormatTimeType(pub i32);
pub const UTZFMT_TIME_TYPE_UNKNOWN: UTimeZoneFormatTimeType = UTimeZoneFormatTimeType(0i32);
pub const UTZFMT_TIME_TYPE_STANDARD: UTimeZoneFormatTimeType = UTimeZoneFormatTimeType(1i32);
pub const UTZFMT_TIME_TYPE_DAYLIGHT: UTimeZoneFormatTimeType = UTimeZoneFormatTimeType(2i32);
impl ::core::marker::Copy for UTimeZoneFormatTimeType {}
impl ::core::clone::Clone for UTimeZoneFormatTimeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UTimeZoneFormatTimeType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UTimeZoneFormatTimeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UTimeZoneFormatTimeType")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UTimeZoneFormatTimeType {
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
pub struct UTimeZoneNameType(pub i32);
pub const UTZNM_UNKNOWN: UTimeZoneNameType = UTimeZoneNameType(0i32);
pub const UTZNM_LONG_GENERIC: UTimeZoneNameType = UTimeZoneNameType(1i32);
pub const UTZNM_LONG_STANDARD: UTimeZoneNameType = UTimeZoneNameType(2i32);
pub const UTZNM_LONG_DAYLIGHT: UTimeZoneNameType = UTimeZoneNameType(4i32);
pub const UTZNM_SHORT_GENERIC: UTimeZoneNameType = UTimeZoneNameType(8i32);
pub const UTZNM_SHORT_STANDARD: UTimeZoneNameType = UTimeZoneNameType(16i32);
pub const UTZNM_SHORT_DAYLIGHT: UTimeZoneNameType = UTimeZoneNameType(32i32);
pub const UTZNM_EXEMPLAR_LOCATION: UTimeZoneNameType = UTimeZoneNameType(64i32);
impl ::core::marker::Copy for UTimeZoneNameType {}
impl ::core::clone::Clone for UTimeZoneNameType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UTimeZoneNameType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UTimeZoneNameType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UTimeZoneNameType").field(&self.0).finish()
    }
}
impl FromIntoMemory for UTimeZoneNameType {
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
pub struct UTimeZoneTransitionType(pub i32);
pub const UCAL_TZ_TRANSITION_NEXT: UTimeZoneTransitionType = UTimeZoneTransitionType(0i32);
pub const UCAL_TZ_TRANSITION_NEXT_INCLUSIVE: UTimeZoneTransitionType =
    UTimeZoneTransitionType(1i32);
pub const UCAL_TZ_TRANSITION_PREVIOUS: UTimeZoneTransitionType = UTimeZoneTransitionType(2i32);
pub const UCAL_TZ_TRANSITION_PREVIOUS_INCLUSIVE: UTimeZoneTransitionType =
    UTimeZoneTransitionType(3i32);
impl ::core::marker::Copy for UTimeZoneTransitionType {}
impl ::core::clone::Clone for UTimeZoneTransitionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UTimeZoneTransitionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UTimeZoneTransitionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UTimeZoneTransitionType")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UTimeZoneTransitionType {
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
pub type UTraceData =
    StdCallFnPtr<(ConstPtr<::core::ffi::c_void>, i32, i32, PCSTR, MutPtr<i8>), ()>;
pub type UTraceEntry = StdCallFnPtr<(ConstPtr<::core::ffi::c_void>, i32), ()>;
pub type UTraceExit = StdCallFnPtr<(ConstPtr<::core::ffi::c_void>, i32, PCSTR, MutPtr<i8>), ()>;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UTraceFunctionNumber(pub i32);
pub const UTRACE_FUNCTION_START: UTraceFunctionNumber = UTraceFunctionNumber(0i32);
pub const UTRACE_U_INIT: UTraceFunctionNumber = UTraceFunctionNumber(0i32);
pub const UTRACE_U_CLEANUP: UTraceFunctionNumber = UTraceFunctionNumber(1i32);
pub const UTRACE_CONVERSION_START: UTraceFunctionNumber = UTraceFunctionNumber(4096i32);
pub const UTRACE_UCNV_OPEN: UTraceFunctionNumber = UTraceFunctionNumber(4096i32);
pub const UTRACE_UCNV_OPEN_PACKAGE: UTraceFunctionNumber = UTraceFunctionNumber(4097i32);
pub const UTRACE_UCNV_OPEN_ALGORITHMIC: UTraceFunctionNumber = UTraceFunctionNumber(4098i32);
pub const UTRACE_UCNV_CLONE: UTraceFunctionNumber = UTraceFunctionNumber(4099i32);
pub const UTRACE_UCNV_CLOSE: UTraceFunctionNumber = UTraceFunctionNumber(4100i32);
pub const UTRACE_UCNV_FLUSH_CACHE: UTraceFunctionNumber = UTraceFunctionNumber(4101i32);
pub const UTRACE_UCNV_LOAD: UTraceFunctionNumber = UTraceFunctionNumber(4102i32);
pub const UTRACE_UCNV_UNLOAD: UTraceFunctionNumber = UTraceFunctionNumber(4103i32);
pub const UTRACE_COLLATION_START: UTraceFunctionNumber = UTraceFunctionNumber(8192i32);
pub const UTRACE_UCOL_OPEN: UTraceFunctionNumber = UTraceFunctionNumber(8192i32);
pub const UTRACE_UCOL_CLOSE: UTraceFunctionNumber = UTraceFunctionNumber(8193i32);
pub const UTRACE_UCOL_STRCOLL: UTraceFunctionNumber = UTraceFunctionNumber(8194i32);
pub const UTRACE_UCOL_GET_SORTKEY: UTraceFunctionNumber = UTraceFunctionNumber(8195i32);
pub const UTRACE_UCOL_GETLOCALE: UTraceFunctionNumber = UTraceFunctionNumber(8196i32);
pub const UTRACE_UCOL_NEXTSORTKEYPART: UTraceFunctionNumber = UTraceFunctionNumber(8197i32);
pub const UTRACE_UCOL_STRCOLLITER: UTraceFunctionNumber = UTraceFunctionNumber(8198i32);
pub const UTRACE_UCOL_OPEN_FROM_SHORT_STRING: UTraceFunctionNumber = UTraceFunctionNumber(8199i32);
pub const UTRACE_UCOL_STRCOLLUTF8: UTraceFunctionNumber = UTraceFunctionNumber(8200i32);
pub const UTRACE_UDATA_START: UTraceFunctionNumber = UTraceFunctionNumber(12288i32);
pub const UTRACE_UDATA_RESOURCE: UTraceFunctionNumber = UTraceFunctionNumber(12288i32);
pub const UTRACE_UDATA_BUNDLE: UTraceFunctionNumber = UTraceFunctionNumber(12289i32);
pub const UTRACE_UDATA_DATA_FILE: UTraceFunctionNumber = UTraceFunctionNumber(12290i32);
pub const UTRACE_UDATA_RES_FILE: UTraceFunctionNumber = UTraceFunctionNumber(12291i32);
impl ::core::marker::Copy for UTraceFunctionNumber {}
impl ::core::clone::Clone for UTraceFunctionNumber {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UTraceFunctionNumber {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UTraceFunctionNumber {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UTraceFunctionNumber")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UTraceFunctionNumber {
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
pub struct UTraceLevel(pub i32);
pub const UTRACE_OFF: UTraceLevel = UTraceLevel(-1i32);
pub const UTRACE_ERROR: UTraceLevel = UTraceLevel(0i32);
pub const UTRACE_WARNING: UTraceLevel = UTraceLevel(3i32);
pub const UTRACE_OPEN_CLOSE: UTraceLevel = UTraceLevel(5i32);
pub const UTRACE_INFO: UTraceLevel = UTraceLevel(7i32);
pub const UTRACE_VERBOSE: UTraceLevel = UTraceLevel(9i32);
impl ::core::marker::Copy for UTraceLevel {}
impl ::core::clone::Clone for UTraceLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UTraceLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UTraceLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UTraceLevel").field(&self.0).finish()
    }
}
impl FromIntoMemory for UTraceLevel {
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
pub struct UTransDirection(pub i32);
pub const UTRANS_FORWARD: UTransDirection = UTransDirection(0i32);
pub const UTRANS_REVERSE: UTransDirection = UTransDirection(1i32);
impl ::core::marker::Copy for UTransDirection {}
impl ::core::clone::Clone for UTransDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UTransDirection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UTransDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UTransDirection").field(&self.0).finish()
    }
}
impl FromIntoMemory for UTransDirection {
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
pub struct UTransPosition {
    pub contextStart: i32,
    pub contextLimit: i32,
    pub start: i32,
    pub limit: i32,
}
impl ::core::marker::Copy for UTransPosition {}
impl ::core::clone::Clone for UTransPosition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UTransPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UTransPosition")
            .field("contextStart", &self.contextStart)
            .field("contextLimit", &self.contextLimit)
            .field("start", &self.start)
            .field("limit", &self.limit)
            .finish()
    }
}
impl ::core::cmp::PartialEq for UTransPosition {
    fn eq(&self, other: &Self) -> bool {
        self.contextStart == other.contextStart
            && self.contextLimit == other.contextLimit
            && self.start == other.start
            && self.limit == other.limit
    }
}
impl ::core::cmp::Eq for UTransPosition {}
impl FromIntoMemory for UTransPosition {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16u32 as usize);
        let f_contextStart = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_contextLimit = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_start = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_limit = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            contextStart: f_contextStart,
            contextLimit: f_contextLimit,
            start: f_start,
            limit: f_limit,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16u32 as usize);
        FromIntoMemory::into_bytes(self.contextStart, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.contextLimit, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.start, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.limit, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16u32 as usize
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UVerticalOrientation(pub i32);
pub const U_VO_ROTATED: UVerticalOrientation = UVerticalOrientation(0i32);
pub const U_VO_TRANSFORMED_ROTATED: UVerticalOrientation = UVerticalOrientation(1i32);
pub const U_VO_TRANSFORMED_UPRIGHT: UVerticalOrientation = UVerticalOrientation(2i32);
pub const U_VO_UPRIGHT: UVerticalOrientation = UVerticalOrientation(3i32);
impl ::core::marker::Copy for UVerticalOrientation {}
impl ::core::clone::Clone for UVerticalOrientation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UVerticalOrientation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UVerticalOrientation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UVerticalOrientation")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for UVerticalOrientation {
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
pub struct UWordBreak(pub i32);
pub const UBRK_WORD_NONE: UWordBreak = UWordBreak(0i32);
pub const UBRK_WORD_NONE_LIMIT: UWordBreak = UWordBreak(100i32);
pub const UBRK_WORD_NUMBER: UWordBreak = UWordBreak(100i32);
pub const UBRK_WORD_NUMBER_LIMIT: UWordBreak = UWordBreak(200i32);
pub const UBRK_WORD_LETTER: UWordBreak = UWordBreak(200i32);
pub const UBRK_WORD_LETTER_LIMIT: UWordBreak = UWordBreak(300i32);
pub const UBRK_WORD_KANA: UWordBreak = UWordBreak(300i32);
pub const UBRK_WORD_KANA_LIMIT: UWordBreak = UWordBreak(400i32);
pub const UBRK_WORD_IDEO: UWordBreak = UWordBreak(400i32);
pub const UBRK_WORD_IDEO_LIMIT: UWordBreak = UWordBreak(500i32);
impl ::core::marker::Copy for UWordBreak {}
impl ::core::clone::Clone for UWordBreak {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UWordBreak {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UWordBreak {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UWordBreak").field(&self.0).finish()
    }
}
impl FromIntoMemory for UWordBreak {
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
pub struct UWordBreakValues(pub i32);
pub const U_WB_OTHER: UWordBreakValues = UWordBreakValues(0i32);
pub const U_WB_ALETTER: UWordBreakValues = UWordBreakValues(1i32);
pub const U_WB_FORMAT: UWordBreakValues = UWordBreakValues(2i32);
pub const U_WB_KATAKANA: UWordBreakValues = UWordBreakValues(3i32);
pub const U_WB_MIDLETTER: UWordBreakValues = UWordBreakValues(4i32);
pub const U_WB_MIDNUM: UWordBreakValues = UWordBreakValues(5i32);
pub const U_WB_NUMERIC: UWordBreakValues = UWordBreakValues(6i32);
pub const U_WB_EXTENDNUMLET: UWordBreakValues = UWordBreakValues(7i32);
pub const U_WB_CR: UWordBreakValues = UWordBreakValues(8i32);
pub const U_WB_EXTEND: UWordBreakValues = UWordBreakValues(9i32);
pub const U_WB_LF: UWordBreakValues = UWordBreakValues(10i32);
pub const U_WB_MIDNUMLET: UWordBreakValues = UWordBreakValues(11i32);
pub const U_WB_NEWLINE: UWordBreakValues = UWordBreakValues(12i32);
pub const U_WB_REGIONAL_INDICATOR: UWordBreakValues = UWordBreakValues(13i32);
pub const U_WB_HEBREW_LETTER: UWordBreakValues = UWordBreakValues(14i32);
pub const U_WB_SINGLE_QUOTE: UWordBreakValues = UWordBreakValues(15i32);
pub const U_WB_DOUBLE_QUOTE: UWordBreakValues = UWordBreakValues(16i32);
pub const U_WB_E_BASE: UWordBreakValues = UWordBreakValues(17i32);
pub const U_WB_E_BASE_GAZ: UWordBreakValues = UWordBreakValues(18i32);
pub const U_WB_E_MODIFIER: UWordBreakValues = UWordBreakValues(19i32);
pub const U_WB_GLUE_AFTER_ZWJ: UWordBreakValues = UWordBreakValues(20i32);
pub const U_WB_ZWJ: UWordBreakValues = UWordBreakValues(21i32);
pub const U_WB_WSEGSPACE: UWordBreakValues = UWordBreakValues(22i32);
impl ::core::marker::Copy for UWordBreakValues {}
impl ::core::clone::Clone for UWordBreakValues {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UWordBreakValues {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UWordBreakValues {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UWordBreakValues").field(&self.0).finish()
    }
}
impl FromIntoMemory for UWordBreakValues {
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
pub const U_ASCII_FAMILY: u32 = 0u32;
pub const U_CHAR16_IS_TYPEDEF: u32 = 1u32;
pub const U_CHARSET_FAMILY: u32 = 1u32;
pub const U_CHARSET_IS_UTF8: u32 = 1u32;
pub const U_CHECK_DYLOAD: u32 = 1u32;
pub const U_COMBINED_IMPLEMENTATION: u32 = 1u32;
pub const U_COMPARE_CODE_POINT_ORDER: u32 = 32768u32;
pub const U_COMPARE_IGNORE_CASE: u32 = 65536u32;
pub const U_COPYRIGHT_STRING_LENGTH: u32 = 128u32;
pub const U_CPLUSPLUS_VERSION: u32 = 0u32;
pub const U_DEBUG: u32 = 1u32;
pub const U_DEFAULT_SHOW_DRAFT: u32 = 0u32;
pub const U_DEFINE_FALSE_AND_TRUE: u32 = 1u32;
pub const U_DISABLE_RENAMING: u32 = 1u32;
pub const U_EBCDIC_FAMILY: u32 = 1u32;
pub const U_EDITS_NO_RESET: u32 = 8192u32;
pub const U_ENABLE_DYLOAD: u32 = 1u32;
pub const U_ENABLE_TRACING: u32 = 0u32;
pub const U_FOLD_CASE_DEFAULT: u32 = 0u32;
pub const U_FOLD_CASE_EXCLUDE_SPECIAL_I: u32 = 1u32;
pub const U_GCC_MAJOR_MINOR: u32 = 0u32;
pub const U_HAVE_CHAR16_T: u32 = 1u32;
pub const U_HAVE_DEBUG_LOCATION_NEW: u32 = 1u32;
pub const U_HAVE_INTTYPES_H: u32 = 1u32;
pub const U_HAVE_LIB_SUFFIX: u32 = 1u32;
pub const U_HAVE_PLACEMENT_NEW: u32 = 0u32;
pub const U_HAVE_RBNF: u32 = 0u32;
pub const U_HAVE_RVALUE_REFERENCES: u32 = 1u32;
pub const U_HAVE_STDINT_H: u32 = 1u32;
pub const U_HAVE_STD_STRING: u32 = 0u32;
pub const U_HAVE_WCHAR_H: u32 = 0u32;
pub const U_HAVE_WCSCPY: u32 = 0u32;
pub const U_HIDE_DEPRECATED_API: u32 = 1u32;
pub const U_HIDE_DRAFT_API: u32 = 1u32;
pub const U_HIDE_INTERNAL_API: u32 = 1u32;
pub const U_HIDE_OBSOLETE_API: u32 = 1u32;
pub const U_HIDE_OBSOLETE_UTF_OLD_H: u32 = 0u32;
pub const U_ICUDATA_TYPE_LETTER: &'static str = "e";
pub const U_ICU_DATA_KEY: &'static str = "DataVersion";
pub const U_ICU_VERSION_BUNDLE: &'static str = "icuver";
pub const U_IOSTREAM_SOURCE: u32 = 199711u32;
pub const U_IS_BIG_ENDIAN: u32 = 0u32;
pub const U_LIB_SUFFIX_C_NAME_STRING: &'static str = "";
pub const U_MAX_VERSION_LENGTH: u32 = 4u32;
pub const U_MAX_VERSION_STRING_LENGTH: u32 = 20u32;
pub const U_MILLIS_PER_DAY: u32 = 86400000u32;
pub const U_MILLIS_PER_HOUR: u32 = 3600000u32;
pub const U_MILLIS_PER_MINUTE: u32 = 60000u32;
pub const U_MILLIS_PER_SECOND: u32 = 1000u32;
pub const U_NO_DEFAULT_INCLUDE_UTF_HEADERS: u32 = 1u32;
pub const U_OMIT_UNCHANGED_TEXT: u32 = 16384u32;
pub const U_OVERRIDE_CXX_ALLOCATION: u32 = 1u32;
pub const U_PARSE_CONTEXT_LEN: i32 = 16i32;
pub const U_PF_AIX: u32 = 3100u32;
pub const U_PF_ANDROID: u32 = 4050u32;
pub const U_PF_BROWSER_NATIVE_CLIENT: u32 = 4020u32;
pub const U_PF_BSD: u32 = 3000u32;
pub const U_PF_CYGWIN: u32 = 1900u32;
pub const U_PF_DARWIN: u32 = 3500u32;
pub const U_PF_EMSCRIPTEN: u32 = 5010u32;
pub const U_PF_FUCHSIA: u32 = 4100u32;
pub const U_PF_HPUX: u32 = 2100u32;
pub const U_PF_IPHONE: u32 = 3550u32;
pub const U_PF_IRIX: u32 = 3200u32;
pub const U_PF_LINUX: u32 = 4000u32;
pub const U_PF_MINGW: u32 = 1800u32;
pub const U_PF_OS390: u32 = 9000u32;
pub const U_PF_OS400: u32 = 9400u32;
pub const U_PF_QNX: u32 = 3700u32;
pub const U_PF_SOLARIS: u32 = 2600u32;
pub const U_PF_UNKNOWN: u32 = 0u32;
pub const U_PF_WINDOWS: u32 = 1000u32;
pub const U_PLATFORM: u32 = 1800u32;
pub const U_PLATFORM_HAS_WIN32_API: u32 = 1u32;
pub const U_PLATFORM_HAS_WINUWP_API: u32 = 0u32;
pub const U_PLATFORM_IMPLEMENTS_POSIX: u32 = 0u32;
pub const U_PLATFORM_IS_DARWIN_BASED: u32 = 1u32;
pub const U_PLATFORM_IS_LINUX_BASED: u32 = 1u32;
pub const U_PLATFORM_USES_ONLY_WIN32_API: u32 = 1u32;
pub const U_SENTINEL: i32 = -1i32;
pub const U_SHAPE_AGGREGATE_TASHKEEL: u32 = 16384u32;
pub const U_SHAPE_AGGREGATE_TASHKEEL_MASK: u32 = 16384u32;
pub const U_SHAPE_AGGREGATE_TASHKEEL_NOOP: u32 = 0u32;
pub const U_SHAPE_DIGITS_ALEN2AN_INIT_AL: u32 = 128u32;
pub const U_SHAPE_DIGITS_ALEN2AN_INIT_LR: u32 = 96u32;
pub const U_SHAPE_DIGITS_AN2EN: u32 = 64u32;
pub const U_SHAPE_DIGITS_EN2AN: u32 = 32u32;
pub const U_SHAPE_DIGITS_MASK: u32 = 224u32;
pub const U_SHAPE_DIGITS_NOOP: u32 = 0u32;
pub const U_SHAPE_DIGITS_RESERVED: u32 = 160u32;
pub const U_SHAPE_DIGIT_TYPE_AN: u32 = 0u32;
pub const U_SHAPE_DIGIT_TYPE_AN_EXTENDED: u32 = 256u32;
pub const U_SHAPE_DIGIT_TYPE_MASK: u32 = 768u32;
pub const U_SHAPE_DIGIT_TYPE_RESERVED: u32 = 512u32;
pub const U_SHAPE_LAMALEF_AUTO: u32 = 65536u32;
pub const U_SHAPE_LAMALEF_BEGIN: u32 = 3u32;
pub const U_SHAPE_LAMALEF_END: u32 = 2u32;
pub const U_SHAPE_LAMALEF_MASK: u32 = 65539u32;
pub const U_SHAPE_LAMALEF_NEAR: u32 = 1u32;
pub const U_SHAPE_LAMALEF_RESIZE: u32 = 0u32;
pub const U_SHAPE_LENGTH_FIXED_SPACES_AT_BEGINNING: u32 = 3u32;
pub const U_SHAPE_LENGTH_FIXED_SPACES_AT_END: u32 = 2u32;
pub const U_SHAPE_LENGTH_FIXED_SPACES_NEAR: u32 = 1u32;
pub const U_SHAPE_LENGTH_GROW_SHRINK: u32 = 0u32;
pub const U_SHAPE_LENGTH_MASK: u32 = 65539u32;
pub const U_SHAPE_LETTERS_MASK: u32 = 24u32;
pub const U_SHAPE_LETTERS_NOOP: u32 = 0u32;
pub const U_SHAPE_LETTERS_SHAPE: u32 = 8u32;
pub const U_SHAPE_LETTERS_SHAPE_TASHKEEL_ISOLATED: u32 = 24u32;
pub const U_SHAPE_LETTERS_UNSHAPE: u32 = 16u32;
pub const U_SHAPE_PRESERVE_PRESENTATION: u32 = 32768u32;
pub const U_SHAPE_PRESERVE_PRESENTATION_MASK: u32 = 32768u32;
pub const U_SHAPE_PRESERVE_PRESENTATION_NOOP: u32 = 0u32;
pub const U_SHAPE_SEEN_MASK: u32 = 7340032u32;
pub const U_SHAPE_SEEN_TWOCELL_NEAR: u32 = 2097152u32;
pub const U_SHAPE_SPACES_RELATIVE_TO_TEXT_BEGIN_END: u32 = 67108864u32;
pub const U_SHAPE_SPACES_RELATIVE_TO_TEXT_MASK: u32 = 67108864u32;
pub const U_SHAPE_TAIL_NEW_UNICODE: u32 = 134217728u32;
pub const U_SHAPE_TAIL_TYPE_MASK: u32 = 134217728u32;
pub const U_SHAPE_TASHKEEL_BEGIN: u32 = 262144u32;
pub const U_SHAPE_TASHKEEL_END: u32 = 393216u32;
pub const U_SHAPE_TASHKEEL_MASK: u32 = 917504u32;
pub const U_SHAPE_TASHKEEL_REPLACE_BY_TATWEEL: u32 = 786432u32;
pub const U_SHAPE_TASHKEEL_RESIZE: u32 = 524288u32;
pub const U_SHAPE_TEXT_DIRECTION_LOGICAL: u32 = 0u32;
pub const U_SHAPE_TEXT_DIRECTION_MASK: u32 = 4u32;
pub const U_SHAPE_TEXT_DIRECTION_VISUAL_LTR: u32 = 4u32;
pub const U_SHAPE_TEXT_DIRECTION_VISUAL_RTL: u32 = 0u32;
pub const U_SHAPE_YEHHAMZA_MASK: u32 = 58720256u32;
pub const U_SHAPE_YEHHAMZA_TWOCELL_NEAR: u32 = 16777216u32;
pub const U_SHOW_CPLUSPLUS_API: u32 = 0u32;
pub const U_SIZEOF_UCHAR: u32 = 2u32;
pub const U_SIZEOF_WCHAR_T: u32 = 1u32;
pub const U_TITLECASE_ADJUST_TO_CASED: u32 = 1024u32;
pub const U_TITLECASE_NO_BREAK_ADJUSTMENT: u32 = 512u32;
pub const U_TITLECASE_NO_LOWERCASE: u32 = 256u32;
pub const U_TITLECASE_SENTENCES: u32 = 64u32;
pub const U_TITLECASE_WHOLE_STRING: u32 = 32u32;
pub const U_UNICODE_VERSION: &'static str = "8.0";
pub const U_USING_ICU_NAMESPACE: u32 = 1u32;
pub const VS_ALLOW_LATIN: u32 = 1u32;
pub const WC_COMPOSITECHECK: u32 = 512u32;
pub const WC_DEFAULTCHAR: u32 = 64u32;
pub const WC_DISCARDNS: u32 = 16u32;
pub const WC_ERR_INVALID_CHARS: u32 = 128u32;
pub const WC_NO_BEST_FIT_CHARS: u32 = 1024u32;
pub const WC_SEPCHARS: u32 = 32u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WORDLIST_TYPE(pub i32);
pub const WORDLIST_TYPE_IGNORE: WORDLIST_TYPE = WORDLIST_TYPE(0i32);
pub const WORDLIST_TYPE_ADD: WORDLIST_TYPE = WORDLIST_TYPE(1i32);
pub const WORDLIST_TYPE_EXCLUDE: WORDLIST_TYPE = WORDLIST_TYPE(2i32);
pub const WORDLIST_TYPE_AUTOCORRECT: WORDLIST_TYPE = WORDLIST_TYPE(3i32);
impl ::core::marker::Copy for WORDLIST_TYPE {}
impl ::core::clone::Clone for WORDLIST_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WORDLIST_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WORDLIST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WORDLIST_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for WORDLIST_TYPE {
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
pub struct opentype_feature_record {
    pub tagFeature: u32,
    pub lParameter: i32,
}
impl ::core::marker::Copy for opentype_feature_record {}
impl ::core::clone::Clone for opentype_feature_record {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for opentype_feature_record {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("opentype_feature_record")
            .field("tagFeature", &self.tagFeature)
            .field("lParameter", &self.lParameter)
            .finish()
    }
}
impl ::core::cmp::PartialEq for opentype_feature_record {
    fn eq(&self, other: &Self) -> bool {
        self.tagFeature == other.tagFeature && self.lParameter == other.lParameter
    }
}
impl ::core::cmp::Eq for opentype_feature_record {}
impl FromIntoMemory for opentype_feature_record {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8u32 as usize);
        let f_tagFeature = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lParameter = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            tagFeature: f_tagFeature,
            lParameter: f_lParameter,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8u32 as usize);
        FromIntoMemory::into_bytes(self.tagFeature, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lParameter, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8u32 as usize
    }
}
pub struct script_charprop {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for script_charprop {}
impl ::core::clone::Clone for script_charprop {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for script_charprop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("script_charprop")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::core::cmp::PartialEq for script_charprop {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for script_charprop {}
impl FromIntoMemory for script_charprop {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 2u32 as usize);
        let f__bitfield = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        Self {
            _bitfield: f__bitfield,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 2u32 as usize);
        FromIntoMemory::into_bytes(self._bitfield, &mut into[0..0 + 2]);
    }
    fn size() -> usize {
        2u32 as usize
    }
}
pub struct script_glyphprop {
    pub sva: SCRIPT_VISATTR,
    pub reserved: u16,
}
impl ::core::marker::Copy for script_glyphprop {}
impl ::core::clone::Clone for script_glyphprop {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for script_glyphprop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("script_glyphprop")
            .field("sva", &self.sva)
            .field("reserved", &self.reserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for script_glyphprop {
    fn eq(&self, other: &Self) -> bool {
        self.sva == other.sva && self.reserved == other.reserved
    }
}
impl ::core::cmp::Eq for script_glyphprop {}
impl FromIntoMemory for script_glyphprop {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4u32 as usize);
        let f_sva = <SCRIPT_VISATTR as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_reserved = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        Self {
            sva: f_sva,
            reserved: f_reserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4u32 as usize);
        FromIntoMemory::into_bytes(self.sva, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.reserved, &mut into[2..2 + 2]);
    }
    fn size() -> usize {
        4u32 as usize
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct tagMLCONVCHARF(pub i32);
pub const MLCONVCHARF_AUTODETECT: tagMLCONVCHARF = tagMLCONVCHARF(1i32);
pub const MLCONVCHARF_ENTITIZE: tagMLCONVCHARF = tagMLCONVCHARF(2i32);
pub const MLCONVCHARF_NCR_ENTITIZE: tagMLCONVCHARF = tagMLCONVCHARF(2i32);
pub const MLCONVCHARF_NAME_ENTITIZE: tagMLCONVCHARF = tagMLCONVCHARF(4i32);
pub const MLCONVCHARF_USEDEFCHAR: tagMLCONVCHARF = tagMLCONVCHARF(8i32);
pub const MLCONVCHARF_NOBESTFITCHARS: tagMLCONVCHARF = tagMLCONVCHARF(16i32);
pub const MLCONVCHARF_DETECTJPN: tagMLCONVCHARF = tagMLCONVCHARF(32i32);
impl ::core::marker::Copy for tagMLCONVCHARF {}
impl ::core::clone::Clone for tagMLCONVCHARF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for tagMLCONVCHARF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for tagMLCONVCHARF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("tagMLCONVCHARF").field(&self.0).finish()
    }
}
impl FromIntoMemory for tagMLCONVCHARF {
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
pub struct tagMLCPF(pub i32);
pub const MLDETECTF_MAILNEWS: tagMLCPF = tagMLCPF(1i32);
pub const MLDETECTF_BROWSER: tagMLCPF = tagMLCPF(2i32);
pub const MLDETECTF_VALID: tagMLCPF = tagMLCPF(4i32);
pub const MLDETECTF_VALID_NLS: tagMLCPF = tagMLCPF(8i32);
pub const MLDETECTF_PRESERVE_ORDER: tagMLCPF = tagMLCPF(16i32);
pub const MLDETECTF_PREFERRED_ONLY: tagMLCPF = tagMLCPF(32i32);
pub const MLDETECTF_FILTER_SPECIALCHAR: tagMLCPF = tagMLCPF(64i32);
pub const MLDETECTF_EURO_UTF8: tagMLCPF = tagMLCPF(128i32);
impl ::core::marker::Copy for tagMLCPF {}
impl ::core::clone::Clone for tagMLCPF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for tagMLCPF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for tagMLCPF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("tagMLCPF").field(&self.0).finish()
    }
}
impl FromIntoMemory for tagMLCPF {
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
pub struct tagSCRIPFONTINFO {
    pub scripts: i64,
    pub wszFont: [u16; 32],
}
impl ::core::marker::Copy for tagSCRIPFONTINFO {}
impl ::core::clone::Clone for tagSCRIPFONTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for tagSCRIPFONTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("tagSCRIPFONTINFO")
            .field("scripts", &self.scripts)
            .field("wszFont", &self.wszFont)
            .finish()
    }
}
impl ::core::cmp::PartialEq for tagSCRIPFONTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.scripts == other.scripts && self.wszFont == other.wszFont
    }
}
impl ::core::cmp::Eq for tagSCRIPFONTINFO {}
impl FromIntoMemory for tagSCRIPFONTINFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40u32 as usize);
        let f_scripts = <i64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_wszFont = <[u16; 32] as FromIntoMemory>::from_bytes(&from[8..8 + 32]);
        Self {
            scripts: f_scripts,
            wszFont: f_wszFont,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40u32 as usize);
        FromIntoMemory::into_bytes(self.scripts, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.wszFont, &mut into[8..8 + 32]);
    }
    fn size() -> usize {
        40u32 as usize
    }
}
pub struct textrange_properties {
    pub potfRecords: MutPtr<opentype_feature_record>,
    pub cotfRecords: i32,
}
impl ::core::marker::Copy for textrange_properties {}
impl ::core::clone::Clone for textrange_properties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for textrange_properties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("textrange_properties")
            .field("potfRecords", &self.potfRecords)
            .field("cotfRecords", &self.cotfRecords)
            .finish()
    }
}
impl ::core::cmp::PartialEq for textrange_properties {
    fn eq(&self, other: &Self) -> bool {
        self.potfRecords == other.potfRecords && self.cotfRecords == other.cotfRecords
    }
}
impl ::core::cmp::Eq for textrange_properties {}
impl FromIntoMemory for textrange_properties {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8u32 as usize);
        let f_potfRecords =
            <MutPtr<opentype_feature_record> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_cotfRecords = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            potfRecords: f_potfRecords,
            cotfRecords: f_cotfRecords,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8u32 as usize);
        FromIntoMemory::into_bytes(self.potfRecords, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.cotfRecords, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8u32 as usize
    }
}
pub trait Api {
    fn CompareStringA(
        &self,
        locale: u32,
        dw_cmp_flags: u32,
        lp_string_1: ConstPtr<i8>,
        cch_count_1: i32,
        lp_string_2: ConstPtr<i8>,
        cch_count_2: i32,
    ) -> i32 {
        todo!("CompareStringA")
    }
    fn CompareStringEx(
        &self,
        lp_locale_name: PCWSTR,
        dw_cmp_flags: COMPARE_STRING_FLAGS,
        lp_string_1: PCWSTR,
        cch_count_1: i32,
        lp_string_2: PCWSTR,
        cch_count_2: i32,
        lp_version_information: MutPtr<NLSVERSIONINFO>,
        lp_reserved: MutPtr<::core::ffi::c_void>,
        l_param: super::Foundation::LPARAM,
    ) -> i32 {
        todo!("CompareStringEx")
    }
    fn CompareStringOrdinal(
        &self,
        lp_string_1: PCWSTR,
        cch_count_1: i32,
        lp_string_2: PCWSTR,
        cch_count_2: i32,
        b_ignore_case: super::Foundation::BOOL,
    ) -> i32 {
        todo!("CompareStringOrdinal")
    }
    fn CompareStringW(
        &self,
        locale: u32,
        dw_cmp_flags: u32,
        lp_string_1: PCWSTR,
        cch_count_1: i32,
        lp_string_2: PCWSTR,
        cch_count_2: i32,
    ) -> i32 {
        todo!("CompareStringW")
    }
    fn ConvertDefaultLocale(&self, locale: u32) -> u32 {
        todo!("ConvertDefaultLocale")
    }
    fn EnumCalendarInfoA(
        &self,
        lp_cal_info_enum_proc: CALINFO_ENUMPROCA,
        locale: u32,
        calendar: u32,
        cal_type: u32,
    ) -> super::Foundation::BOOL {
        todo!("EnumCalendarInfoA")
    }
    fn EnumCalendarInfoExA(
        &self,
        lp_cal_info_enum_proc_ex: CALINFO_ENUMPROCEXA,
        locale: u32,
        calendar: u32,
        cal_type: u32,
    ) -> super::Foundation::BOOL {
        todo!("EnumCalendarInfoExA")
    }
    fn EnumCalendarInfoExEx(
        &self,
        p_cal_info_enum_proc_ex_ex: CALINFO_ENUMPROCEXEX,
        lp_locale_name: PCWSTR,
        calendar: u32,
        lp_reserved: PCWSTR,
        cal_type: u32,
        l_param: super::Foundation::LPARAM,
    ) -> super::Foundation::BOOL {
        todo!("EnumCalendarInfoExEx")
    }
    fn EnumCalendarInfoExW(
        &self,
        lp_cal_info_enum_proc_ex: CALINFO_ENUMPROCEXW,
        locale: u32,
        calendar: u32,
        cal_type: u32,
    ) -> super::Foundation::BOOL {
        todo!("EnumCalendarInfoExW")
    }
    fn EnumCalendarInfoW(
        &self,
        lp_cal_info_enum_proc: CALINFO_ENUMPROCW,
        locale: u32,
        calendar: u32,
        cal_type: u32,
    ) -> super::Foundation::BOOL {
        todo!("EnumCalendarInfoW")
    }
    fn EnumDateFormatsA(
        &self,
        lp_date_fmt_enum_proc: DATEFMT_ENUMPROCA,
        locale: u32,
        dw_flags: u32,
    ) -> super::Foundation::BOOL {
        todo!("EnumDateFormatsA")
    }
    fn EnumDateFormatsExA(
        &self,
        lp_date_fmt_enum_proc_ex: DATEFMT_ENUMPROCEXA,
        locale: u32,
        dw_flags: u32,
    ) -> super::Foundation::BOOL {
        todo!("EnumDateFormatsExA")
    }
    fn EnumDateFormatsExEx(
        &self,
        lp_date_fmt_enum_proc_ex_ex: DATEFMT_ENUMPROCEXEX,
        lp_locale_name: PCWSTR,
        dw_flags: ENUM_DATE_FORMATS_FLAGS,
        l_param: super::Foundation::LPARAM,
    ) -> super::Foundation::BOOL {
        todo!("EnumDateFormatsExEx")
    }
    fn EnumDateFormatsExW(
        &self,
        lp_date_fmt_enum_proc_ex: DATEFMT_ENUMPROCEXW,
        locale: u32,
        dw_flags: u32,
    ) -> super::Foundation::BOOL {
        todo!("EnumDateFormatsExW")
    }
    fn EnumDateFormatsW(
        &self,
        lp_date_fmt_enum_proc: DATEFMT_ENUMPROCW,
        locale: u32,
        dw_flags: u32,
    ) -> super::Foundation::BOOL {
        todo!("EnumDateFormatsW")
    }
    fn EnumLanguageGroupLocalesA(
        &self,
        lp_lang_group_locale_enum_proc: LANGGROUPLOCALE_ENUMPROCA,
        language_group: u32,
        dw_flags: u32,
        l_param: PtrDiffRepr,
    ) -> super::Foundation::BOOL {
        todo!("EnumLanguageGroupLocalesA")
    }
    fn EnumLanguageGroupLocalesW(
        &self,
        lp_lang_group_locale_enum_proc: LANGGROUPLOCALE_ENUMPROCW,
        language_group: u32,
        dw_flags: u32,
        l_param: PtrDiffRepr,
    ) -> super::Foundation::BOOL {
        todo!("EnumLanguageGroupLocalesW")
    }
    fn EnumSystemCodePagesA(
        &self,
        lp_code_page_enum_proc: CODEPAGE_ENUMPROCA,
        dw_flags: ENUM_SYSTEM_CODE_PAGES_FLAGS,
    ) -> super::Foundation::BOOL {
        todo!("EnumSystemCodePagesA")
    }
    fn EnumSystemCodePagesW(
        &self,
        lp_code_page_enum_proc: CODEPAGE_ENUMPROCW,
        dw_flags: ENUM_SYSTEM_CODE_PAGES_FLAGS,
    ) -> super::Foundation::BOOL {
        todo!("EnumSystemCodePagesW")
    }
    fn EnumSystemGeoID(
        &self,
        geo_class: u32,
        parent_geo_id: i32,
        lp_geo_enum_proc: GEO_ENUMPROC,
    ) -> super::Foundation::BOOL {
        todo!("EnumSystemGeoID")
    }
    fn EnumSystemGeoNames(
        &self,
        geo_class: u32,
        geo_enum_proc: GEO_ENUMNAMEPROC,
        data: super::Foundation::LPARAM,
    ) -> super::Foundation::BOOL {
        todo!("EnumSystemGeoNames")
    }
    fn EnumSystemLanguageGroupsA(
        &self,
        lp_language_group_enum_proc: LANGUAGEGROUP_ENUMPROCA,
        dw_flags: ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS,
        l_param: PtrDiffRepr,
    ) -> super::Foundation::BOOL {
        todo!("EnumSystemLanguageGroupsA")
    }
    fn EnumSystemLanguageGroupsW(
        &self,
        lp_language_group_enum_proc: LANGUAGEGROUP_ENUMPROCW,
        dw_flags: ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS,
        l_param: PtrDiffRepr,
    ) -> super::Foundation::BOOL {
        todo!("EnumSystemLanguageGroupsW")
    }
    fn EnumSystemLocalesA(
        &self,
        lp_locale_enum_proc: LOCALE_ENUMPROCA,
        dw_flags: u32,
    ) -> super::Foundation::BOOL {
        todo!("EnumSystemLocalesA")
    }
    fn EnumSystemLocalesEx(
        &self,
        lp_locale_enum_proc_ex: LOCALE_ENUMPROCEX,
        dw_flags: u32,
        l_param: super::Foundation::LPARAM,
        lp_reserved: ConstPtr<::core::ffi::c_void>,
    ) -> super::Foundation::BOOL {
        todo!("EnumSystemLocalesEx")
    }
    fn EnumSystemLocalesW(
        &self,
        lp_locale_enum_proc: LOCALE_ENUMPROCW,
        dw_flags: u32,
    ) -> super::Foundation::BOOL {
        todo!("EnumSystemLocalesW")
    }
    fn EnumTimeFormatsA(
        &self,
        lp_time_fmt_enum_proc: TIMEFMT_ENUMPROCA,
        locale: u32,
        dw_flags: TIME_FORMAT_FLAGS,
    ) -> super::Foundation::BOOL {
        todo!("EnumTimeFormatsA")
    }
    fn EnumTimeFormatsEx(
        &self,
        lp_time_fmt_enum_proc_ex: TIMEFMT_ENUMPROCEX,
        lp_locale_name: PCWSTR,
        dw_flags: u32,
        l_param: super::Foundation::LPARAM,
    ) -> super::Foundation::BOOL {
        todo!("EnumTimeFormatsEx")
    }
    fn EnumTimeFormatsW(
        &self,
        lp_time_fmt_enum_proc: TIMEFMT_ENUMPROCW,
        locale: u32,
        dw_flags: TIME_FORMAT_FLAGS,
    ) -> super::Foundation::BOOL {
        todo!("EnumTimeFormatsW")
    }
    fn EnumUILanguagesA(
        &self,
        lp_ui_language_enum_proc: UILANGUAGE_ENUMPROCA,
        dw_flags: u32,
        l_param: PtrDiffRepr,
    ) -> super::Foundation::BOOL {
        todo!("EnumUILanguagesA")
    }
    fn EnumUILanguagesW(
        &self,
        lp_ui_language_enum_proc: UILANGUAGE_ENUMPROCW,
        dw_flags: u32,
        l_param: PtrDiffRepr,
    ) -> super::Foundation::BOOL {
        todo!("EnumUILanguagesW")
    }
    fn FindNLSString(
        &self,
        locale: u32,
        dw_find_nls_string_flags: u32,
        lp_string_source: PCWSTR,
        cch_source: i32,
        lp_string_value: PCWSTR,
        cch_value: i32,
        pcch_found: MutPtr<i32>,
    ) -> i32 {
        todo!("FindNLSString")
    }
    fn FindNLSStringEx(
        &self,
        lp_locale_name: PCWSTR,
        dw_find_nls_string_flags: u32,
        lp_string_source: PCWSTR,
        cch_source: i32,
        lp_string_value: PCWSTR,
        cch_value: i32,
        pcch_found: MutPtr<i32>,
        lp_version_information: ConstPtr<NLSVERSIONINFO>,
        lp_reserved: ConstPtr<::core::ffi::c_void>,
        sort_handle: super::Foundation::LPARAM,
    ) -> i32 {
        todo!("FindNLSStringEx")
    }
    fn FindStringOrdinal(
        &self,
        dw_find_string_ordinal_flags: u32,
        lp_string_source: PCWSTR,
        cch_source: i32,
        lp_string_value: PCWSTR,
        cch_value: i32,
        b_ignore_case: super::Foundation::BOOL,
    ) -> i32 {
        todo!("FindStringOrdinal")
    }
    fn FoldStringA(
        &self,
        dw_map_flags: FOLD_STRING_MAP_FLAGS,
        lp_src_str: PCSTR,
        cch_src: i32,
        lp_dest_str: PSTR,
        cch_dest: i32,
    ) -> i32 {
        todo!("FoldStringA")
    }
    fn FoldStringW(
        &self,
        dw_map_flags: FOLD_STRING_MAP_FLAGS,
        lp_src_str: PCWSTR,
        cch_src: i32,
        lp_dest_str: PWSTR,
        cch_dest: i32,
    ) -> i32 {
        todo!("FoldStringW")
    }
    fn GetACP(&self) -> u32 {
        todo!("GetACP")
    }
    fn GetCPInfo(&self, code_page: u32, lp_cp_info: MutPtr<CPINFO>) -> super::Foundation::BOOL {
        todo!("GetCPInfo")
    }
    fn GetCPInfoExA(
        &self,
        code_page: u32,
        dw_flags: u32,
        lp_cp_info_ex: MutPtr<CPINFOEXA>,
    ) -> super::Foundation::BOOL {
        todo!("GetCPInfoExA")
    }
    fn GetCPInfoExW(
        &self,
        code_page: u32,
        dw_flags: u32,
        lp_cp_info_ex: MutPtr<CPINFOEXW>,
    ) -> super::Foundation::BOOL {
        todo!("GetCPInfoExW")
    }
    fn GetCalendarInfoA(
        &self,
        locale: u32,
        calendar: u32,
        cal_type: u32,
        lp_cal_data: PSTR,
        cch_data: i32,
        lp_value: MutPtr<u32>,
    ) -> i32 {
        todo!("GetCalendarInfoA")
    }
    fn GetCalendarInfoEx(
        &self,
        lp_locale_name: PCWSTR,
        calendar: u32,
        lp_reserved: PCWSTR,
        cal_type: u32,
        lp_cal_data: PWSTR,
        cch_data: i32,
        lp_value: MutPtr<u32>,
    ) -> i32 {
        todo!("GetCalendarInfoEx")
    }
    fn GetCalendarInfoW(
        &self,
        locale: u32,
        calendar: u32,
        cal_type: u32,
        lp_cal_data: PWSTR,
        cch_data: i32,
        lp_value: MutPtr<u32>,
    ) -> i32 {
        todo!("GetCalendarInfoW")
    }
    fn GetCurrencyFormatA(
        &self,
        locale: u32,
        dw_flags: u32,
        lp_value: PCSTR,
        lp_format: ConstPtr<CURRENCYFMTA>,
        lp_currency_str: PSTR,
        cch_currency: i32,
    ) -> i32 {
        todo!("GetCurrencyFormatA")
    }
    fn GetCurrencyFormatEx(
        &self,
        lp_locale_name: PCWSTR,
        dw_flags: u32,
        lp_value: PCWSTR,
        lp_format: ConstPtr<CURRENCYFMTW>,
        lp_currency_str: PWSTR,
        cch_currency: i32,
    ) -> i32 {
        todo!("GetCurrencyFormatEx")
    }
    fn GetCurrencyFormatW(
        &self,
        locale: u32,
        dw_flags: u32,
        lp_value: PCWSTR,
        lp_format: ConstPtr<CURRENCYFMTW>,
        lp_currency_str: PWSTR,
        cch_currency: i32,
    ) -> i32 {
        todo!("GetCurrencyFormatW")
    }
    fn GetDateFormatA(
        &self,
        locale: u32,
        dw_flags: u32,
        lp_date: ConstPtr<super::Foundation::SYSTEMTIME>,
        lp_format: PCSTR,
        lp_date_str: PSTR,
        cch_date: i32,
    ) -> i32 {
        todo!("GetDateFormatA")
    }
    fn GetDateFormatEx(
        &self,
        lp_locale_name: PCWSTR,
        dw_flags: ENUM_DATE_FORMATS_FLAGS,
        lp_date: ConstPtr<super::Foundation::SYSTEMTIME>,
        lp_format: PCWSTR,
        lp_date_str: PWSTR,
        cch_date: i32,
        lp_calendar: PCWSTR,
    ) -> i32 {
        todo!("GetDateFormatEx")
    }
    fn GetDateFormatW(
        &self,
        locale: u32,
        dw_flags: u32,
        lp_date: ConstPtr<super::Foundation::SYSTEMTIME>,
        lp_format: PCWSTR,
        lp_date_str: PWSTR,
        cch_date: i32,
    ) -> i32 {
        todo!("GetDateFormatW")
    }
    fn GetDistanceOfClosestLanguageInList(
        &self,
        psz_language: PCWSTR,
        psz_languages_list: PCWSTR,
        wch_list_delimiter: u16,
        p_closest_distance: MutPtr<f64>,
    ) -> crate::core::HRESULT {
        todo!("GetDistanceOfClosestLanguageInList")
    }
    fn GetDurationFormat(
        &self,
        locale: u32,
        dw_flags: u32,
        lp_duration: ConstPtr<super::Foundation::SYSTEMTIME>,
        ull_duration: u64,
        lp_format: PCWSTR,
        lp_duration_str: PWSTR,
        cch_duration: i32,
    ) -> i32 {
        todo!("GetDurationFormat")
    }
    fn GetDurationFormatEx(
        &self,
        lp_locale_name: PCWSTR,
        dw_flags: u32,
        lp_duration: ConstPtr<super::Foundation::SYSTEMTIME>,
        ull_duration: u64,
        lp_format: PCWSTR,
        lp_duration_str: PWSTR,
        cch_duration: i32,
    ) -> i32 {
        todo!("GetDurationFormatEx")
    }
    fn GetFileMUIInfo(
        &self,
        dw_flags: u32,
        pcwsz_file_path: PCWSTR,
        p_file_mui_info: MutPtr<FILEMUIINFO>,
        pcb_file_mui_info: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("GetFileMUIInfo")
    }
    fn GetFileMUIPath(
        &self,
        dw_flags: u32,
        pcwsz_file_path: PCWSTR,
        pwsz_language: PWSTR,
        pcch_language: MutPtr<u32>,
        pwsz_file_mui_path: PWSTR,
        pcch_file_mui_path: MutPtr<u32>,
        pulul_enumerator: MutPtr<u64>,
    ) -> super::Foundation::BOOL {
        todo!("GetFileMUIPath")
    }
    fn GetGeoInfoA(
        &self,
        location: i32,
        geo_type: u32,
        lp_geo_data: PSTR,
        cch_data: i32,
        lang_id: u16,
    ) -> i32 {
        todo!("GetGeoInfoA")
    }
    fn GetGeoInfoEx(
        &self,
        location: PCWSTR,
        geo_type: u32,
        geo_data: PWSTR,
        geo_data_count: i32,
    ) -> i32 {
        todo!("GetGeoInfoEx")
    }
    fn GetGeoInfoW(
        &self,
        location: i32,
        geo_type: u32,
        lp_geo_data: PWSTR,
        cch_data: i32,
        lang_id: u16,
    ) -> i32 {
        todo!("GetGeoInfoW")
    }
    fn GetLocaleInfoA(&self, locale: u32, lc_type: u32, lp_lc_data: PSTR, cch_data: i32) -> i32 {
        todo!("GetLocaleInfoA")
    }
    fn GetLocaleInfoEx(
        &self,
        lp_locale_name: PCWSTR,
        lc_type: u32,
        lp_lc_data: PWSTR,
        cch_data: i32,
    ) -> i32 {
        todo!("GetLocaleInfoEx")
    }
    fn GetLocaleInfoW(&self, locale: u32, lc_type: u32, lp_lc_data: PWSTR, cch_data: i32) -> i32 {
        todo!("GetLocaleInfoW")
    }
    fn GetNLSVersion(
        &self,
        function: u32,
        locale: u32,
        lp_version_information: MutPtr<NLSVERSIONINFO>,
    ) -> super::Foundation::BOOL {
        todo!("GetNLSVersion")
    }
    fn GetNLSVersionEx(
        &self,
        function: u32,
        lp_locale_name: PCWSTR,
        lp_version_information: MutPtr<NLSVERSIONINFOEX>,
    ) -> super::Foundation::BOOL {
        todo!("GetNLSVersionEx")
    }
    fn GetNumberFormatA(
        &self,
        locale: u32,
        dw_flags: u32,
        lp_value: PCSTR,
        lp_format: ConstPtr<NUMBERFMTA>,
        lp_number_str: PSTR,
        cch_number: i32,
    ) -> i32 {
        todo!("GetNumberFormatA")
    }
    fn GetNumberFormatEx(
        &self,
        lp_locale_name: PCWSTR,
        dw_flags: u32,
        lp_value: PCWSTR,
        lp_format: ConstPtr<NUMBERFMTW>,
        lp_number_str: PWSTR,
        cch_number: i32,
    ) -> i32 {
        todo!("GetNumberFormatEx")
    }
    fn GetNumberFormatW(
        &self,
        locale: u32,
        dw_flags: u32,
        lp_value: PCWSTR,
        lp_format: ConstPtr<NUMBERFMTW>,
        lp_number_str: PWSTR,
        cch_number: i32,
    ) -> i32 {
        todo!("GetNumberFormatW")
    }
    fn GetOEMCP(&self) -> u32 {
        todo!("GetOEMCP")
    }
    fn GetProcessPreferredUILanguages(
        &self,
        dw_flags: u32,
        pul_num_languages: MutPtr<u32>,
        pwsz_languages_buffer: PWSTR,
        pcch_languages_buffer: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("GetProcessPreferredUILanguages")
    }
    fn GetStringScripts(
        &self,
        dw_flags: u32,
        lp_string: PCWSTR,
        cch_string: i32,
        lp_scripts: PWSTR,
        cch_scripts: i32,
    ) -> i32 {
        todo!("GetStringScripts")
    }
    fn GetStringTypeA(
        &self,
        locale: u32,
        dw_info_type: u32,
        lp_src_str: PCSTR,
        cch_src: i32,
        lp_char_type: MutPtr<u16>,
    ) -> super::Foundation::BOOL {
        todo!("GetStringTypeA")
    }
    fn GetStringTypeExA(
        &self,
        locale: u32,
        dw_info_type: u32,
        lp_src_str: PCSTR,
        cch_src: i32,
        lp_char_type: MutPtr<u16>,
    ) -> super::Foundation::BOOL {
        todo!("GetStringTypeExA")
    }
    fn GetStringTypeExW(
        &self,
        locale: u32,
        dw_info_type: u32,
        lp_src_str: PCWSTR,
        cch_src: i32,
        lp_char_type: MutPtr<u16>,
    ) -> super::Foundation::BOOL {
        todo!("GetStringTypeExW")
    }
    fn GetStringTypeW(
        &self,
        dw_info_type: u32,
        lp_src_str: PCWSTR,
        cch_src: i32,
        lp_char_type: MutPtr<u16>,
    ) -> super::Foundation::BOOL {
        todo!("GetStringTypeW")
    }
    fn GetSystemDefaultLCID(&self) -> u32 {
        todo!("GetSystemDefaultLCID")
    }
    fn GetSystemDefaultLangID(&self) -> u16 {
        todo!("GetSystemDefaultLangID")
    }
    fn GetSystemDefaultLocaleName(&self, lp_locale_name: PWSTR, cch_locale_name: i32) -> i32 {
        todo!("GetSystemDefaultLocaleName")
    }
    fn GetSystemDefaultUILanguage(&self) -> u16 {
        todo!("GetSystemDefaultUILanguage")
    }
    fn GetSystemPreferredUILanguages(
        &self,
        dw_flags: u32,
        pul_num_languages: MutPtr<u32>,
        pwsz_languages_buffer: PWSTR,
        pcch_languages_buffer: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("GetSystemPreferredUILanguages")
    }
    fn GetTextCharset(&self, hdc: super::Graphics::Gdi::HDC) -> i32 {
        todo!("GetTextCharset")
    }
    fn GetTextCharsetInfo(
        &self,
        hdc: super::Graphics::Gdi::HDC,
        lp_sig: MutPtr<FONTSIGNATURE>,
        dw_flags: u32,
    ) -> i32 {
        todo!("GetTextCharsetInfo")
    }
    fn GetThreadLocale(&self) -> u32 {
        todo!("GetThreadLocale")
    }
    fn GetThreadPreferredUILanguages(
        &self,
        dw_flags: u32,
        pul_num_languages: MutPtr<u32>,
        pwsz_languages_buffer: PWSTR,
        pcch_languages_buffer: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("GetThreadPreferredUILanguages")
    }
    fn GetThreadUILanguage(&self) -> u16 {
        todo!("GetThreadUILanguage")
    }
    fn GetTimeFormatA(
        &self,
        locale: u32,
        dw_flags: u32,
        lp_time: ConstPtr<super::Foundation::SYSTEMTIME>,
        lp_format: PCSTR,
        lp_time_str: PSTR,
        cch_time: i32,
    ) -> i32 {
        todo!("GetTimeFormatA")
    }
    fn GetTimeFormatEx(
        &self,
        lp_locale_name: PCWSTR,
        dw_flags: TIME_FORMAT_FLAGS,
        lp_time: ConstPtr<super::Foundation::SYSTEMTIME>,
        lp_format: PCWSTR,
        lp_time_str: PWSTR,
        cch_time: i32,
    ) -> i32 {
        todo!("GetTimeFormatEx")
    }
    fn GetTimeFormatW(
        &self,
        locale: u32,
        dw_flags: u32,
        lp_time: ConstPtr<super::Foundation::SYSTEMTIME>,
        lp_format: PCWSTR,
        lp_time_str: PWSTR,
        cch_time: i32,
    ) -> i32 {
        todo!("GetTimeFormatW")
    }
    fn GetUILanguageInfo(
        &self,
        dw_flags: u32,
        pwmsz_language: PCWSTR,
        pwsz_fallback_languages: PWSTR,
        pcch_fallback_languages: MutPtr<u32>,
        p_attributes: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("GetUILanguageInfo")
    }
    fn GetUserDefaultGeoName(&self, geo_name: PWSTR, geo_name_count: i32) -> i32 {
        todo!("GetUserDefaultGeoName")
    }
    fn GetUserDefaultLCID(&self) -> u32 {
        todo!("GetUserDefaultLCID")
    }
    fn GetUserDefaultLangID(&self) -> u16 {
        todo!("GetUserDefaultLangID")
    }
    fn GetUserDefaultLocaleName(&self, lp_locale_name: PWSTR, cch_locale_name: i32) -> i32 {
        todo!("GetUserDefaultLocaleName")
    }
    fn GetUserDefaultUILanguage(&self) -> u16 {
        todo!("GetUserDefaultUILanguage")
    }
    fn GetUserGeoID(&self, geo_class: u32) -> i32 {
        todo!("GetUserGeoID")
    }
    fn GetUserPreferredUILanguages(
        &self,
        dw_flags: u32,
        pul_num_languages: MutPtr<u32>,
        pwsz_languages_buffer: PWSTR,
        pcch_languages_buffer: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("GetUserPreferredUILanguages")
    }
    fn IdnToAscii(
        &self,
        dw_flags: u32,
        lp_unicode_char_str: PCWSTR,
        cch_unicode_char: i32,
        lp_ascii_char_str: PWSTR,
        cch_ascii_char: i32,
    ) -> i32 {
        todo!("IdnToAscii")
    }
    fn IdnToNameprepUnicode(
        &self,
        dw_flags: u32,
        lp_unicode_char_str: PCWSTR,
        cch_unicode_char: i32,
        lp_nameprep_char_str: PWSTR,
        cch_nameprep_char: i32,
    ) -> i32 {
        todo!("IdnToNameprepUnicode")
    }
    fn IdnToUnicode(
        &self,
        dw_flags: u32,
        lp_ascii_char_str: PCWSTR,
        cch_ascii_char: i32,
        lp_unicode_char_str: PWSTR,
        cch_unicode_char: i32,
    ) -> i32 {
        todo!("IdnToUnicode")
    }
    fn IsDBCSLeadByte(&self, test_char: u8) -> super::Foundation::BOOL {
        todo!("IsDBCSLeadByte")
    }
    fn IsDBCSLeadByteEx(&self, code_page: u32, test_char: u8) -> super::Foundation::BOOL {
        todo!("IsDBCSLeadByteEx")
    }
    fn IsNLSDefinedString(
        &self,
        function: u32,
        dw_flags: u32,
        lp_version_information: ConstPtr<NLSVERSIONINFO>,
        lp_string: PCWSTR,
        cch_str: i32,
    ) -> super::Foundation::BOOL {
        todo!("IsNLSDefinedString")
    }
    fn IsNormalizedString(
        &self,
        norm_form: NORM_FORM,
        lp_string: PCWSTR,
        cw_length: i32,
    ) -> super::Foundation::BOOL {
        todo!("IsNormalizedString")
    }
    fn IsTextUnicode(
        &self,
        lpv: ConstPtr<::core::ffi::c_void>,
        i_size: i32,
        lpi_result: MutPtr<IS_TEXT_UNICODE_RESULT>,
    ) -> super::Foundation::BOOL {
        todo!("IsTextUnicode")
    }
    fn IsValidCodePage(&self, code_page: u32) -> super::Foundation::BOOL {
        todo!("IsValidCodePage")
    }
    fn IsValidLanguageGroup(
        &self,
        language_group: u32,
        dw_flags: ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS,
    ) -> super::Foundation::BOOL {
        todo!("IsValidLanguageGroup")
    }
    fn IsValidLocale(
        &self,
        locale: u32,
        dw_flags: IS_VALID_LOCALE_FLAGS,
    ) -> super::Foundation::BOOL {
        todo!("IsValidLocale")
    }
    fn IsValidLocaleName(&self, lp_locale_name: PCWSTR) -> super::Foundation::BOOL {
        todo!("IsValidLocaleName")
    }
    fn IsValidNLSVersion(
        &self,
        function: u32,
        lp_locale_name: PCWSTR,
        lp_version_information: ConstPtr<NLSVERSIONINFOEX>,
    ) -> u32 {
        todo!("IsValidNLSVersion")
    }
    fn IsWellFormedTag(&self, psz_tag: PCWSTR) -> u8 {
        todo!("IsWellFormedTag")
    }
    fn LCIDToLocaleName(&self, locale: u32, lp_name: PWSTR, cch_name: i32, dw_flags: u32) -> i32 {
        todo!("LCIDToLocaleName")
    }
    fn LCMapStringA(
        &self,
        locale: u32,
        dw_map_flags: u32,
        lp_src_str: PCSTR,
        cch_src: i32,
        lp_dest_str: PSTR,
        cch_dest: i32,
    ) -> i32 {
        todo!("LCMapStringA")
    }
    fn LCMapStringEx(
        &self,
        lp_locale_name: PCWSTR,
        dw_map_flags: u32,
        lp_src_str: PCWSTR,
        cch_src: i32,
        lp_dest_str: PWSTR,
        cch_dest: i32,
        lp_version_information: ConstPtr<NLSVERSIONINFO>,
        lp_reserved: ConstPtr<::core::ffi::c_void>,
        sort_handle: super::Foundation::LPARAM,
    ) -> i32 {
        todo!("LCMapStringEx")
    }
    fn LCMapStringW(
        &self,
        locale: u32,
        dw_map_flags: u32,
        lp_src_str: PCWSTR,
        cch_src: i32,
        lp_dest_str: PWSTR,
        cch_dest: i32,
    ) -> i32 {
        todo!("LCMapStringW")
    }
    fn LocaleNameToLCID(&self, lp_name: PCWSTR, dw_flags: u32) -> u32 {
        todo!("LocaleNameToLCID")
    }
    fn MappingDoAction(
        &self,
        p_bag: MutPtr<MAPPING_PROPERTY_BAG>,
        dw_range_index: u32,
        psz_action_id: PCWSTR,
    ) -> crate::core::HRESULT {
        todo!("MappingDoAction")
    }
    fn MappingFreePropertyBag(
        &self,
        p_bag: ConstPtr<MAPPING_PROPERTY_BAG>,
    ) -> crate::core::HRESULT {
        todo!("MappingFreePropertyBag")
    }
    fn MappingFreeServices(
        &self,
        p_service_info: ConstPtr<MAPPING_SERVICE_INFO>,
    ) -> crate::core::HRESULT {
        todo!("MappingFreeServices")
    }
    fn MappingGetServices(
        &self,
        p_options: ConstPtr<MAPPING_ENUM_OPTIONS>,
        prg_services: MutPtr<ConstPtr<MAPPING_SERVICE_INFO>>,
        pdw_services_count: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("MappingGetServices")
    }
    fn MappingRecognizeText(
        &self,
        p_service_info: ConstPtr<MAPPING_SERVICE_INFO>,
        psz_text: PCWSTR,
        dw_length: u32,
        dw_index: u32,
        p_options: ConstPtr<MAPPING_OPTIONS>,
        pbag: MutPtr<MAPPING_PROPERTY_BAG>,
    ) -> crate::core::HRESULT {
        todo!("MappingRecognizeText")
    }
    fn MultiByteToWideChar(
        &self,
        code_page: u32,
        dw_flags: MULTI_BYTE_TO_WIDE_CHAR_FLAGS,
        lp_multi_byte_str: PCSTR,
        cb_multi_byte: i32,
        lp_wide_char_str: PWSTR,
        cch_wide_char: i32,
    ) -> i32 {
        todo!("MultiByteToWideChar")
    }
    fn NormalizeString(
        &self,
        norm_form: NORM_FORM,
        lp_src_string: PCWSTR,
        cw_src_length: i32,
        lp_dst_string: PWSTR,
        cw_dst_length: i32,
    ) -> i32 {
        todo!("NormalizeString")
    }
    fn NotifyUILanguageChange(
        &self,
        dw_flags: u32,
        pcwstr_new_language: PCWSTR,
        pcwstr_previous_language: PCWSTR,
        dw_reserved: u32,
        pdw_status_rtrn: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("NotifyUILanguageChange")
    }
    fn ResolveLocaleName(
        &self,
        lp_name_to_resolve: PCWSTR,
        lp_locale_name: PWSTR,
        cch_locale_name: i32,
    ) -> i32 {
        todo!("ResolveLocaleName")
    }
    fn RestoreThreadPreferredUILanguages(&self, snapshot: HSAVEDUILANGUAGES) {
        todo!("RestoreThreadPreferredUILanguages")
    }
    fn ScriptApplyDigitSubstitution(
        &self,
        psds: ConstPtr<SCRIPT_DIGITSUBSTITUTE>,
        psc: MutPtr<SCRIPT_CONTROL>,
        pss: MutPtr<SCRIPT_STATE>,
    ) -> crate::core::HRESULT {
        todo!("ScriptApplyDigitSubstitution")
    }
    fn ScriptApplyLogicalWidth(
        &self,
        pi_dx: ConstPtr<i32>,
        c_chars: i32,
        c_glyphs: i32,
        pw_log_clust: ConstPtr<u16>,
        psva: ConstPtr<SCRIPT_VISATTR>,
        pi_advance: ConstPtr<i32>,
        psa: ConstPtr<SCRIPT_ANALYSIS>,
        p_abc: MutPtr<super::Graphics::Gdi::ABC>,
        pi_justify: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("ScriptApplyLogicalWidth")
    }
    fn ScriptBreak(
        &self,
        pwc_chars: PCWSTR,
        c_chars: i32,
        psa: ConstPtr<SCRIPT_ANALYSIS>,
        psla: MutPtr<SCRIPT_LOGATTR>,
    ) -> crate::core::HRESULT {
        todo!("ScriptBreak")
    }
    fn ScriptCPtoX(
        &self,
        i_cp: i32,
        f_trailing: super::Foundation::BOOL,
        c_chars: i32,
        c_glyphs: i32,
        pw_log_clust: ConstPtr<u16>,
        psva: ConstPtr<SCRIPT_VISATTR>,
        pi_advance: ConstPtr<i32>,
        psa: ConstPtr<SCRIPT_ANALYSIS>,
        pi_x: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("ScriptCPtoX")
    }
    fn ScriptCacheGetHeight(
        &self,
        hdc: super::Graphics::Gdi::HDC,
        psc: MutPtr<ConstPtr<::core::ffi::c_void>>,
        tm_height: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("ScriptCacheGetHeight")
    }
    fn ScriptFreeCache(&self, psc: MutPtr<ConstPtr<::core::ffi::c_void>>) -> crate::core::HRESULT {
        todo!("ScriptFreeCache")
    }
    fn ScriptGetCMap(
        &self,
        hdc: super::Graphics::Gdi::HDC,
        psc: MutPtr<ConstPtr<::core::ffi::c_void>>,
        pwc_in_chars: PCWSTR,
        c_chars: i32,
        dw_flags: u32,
        pw_out_glyphs: MutPtr<u16>,
    ) -> crate::core::HRESULT {
        todo!("ScriptGetCMap")
    }
    fn ScriptGetFontAlternateGlyphs(
        &self,
        hdc: super::Graphics::Gdi::HDC,
        psc: MutPtr<ConstPtr<::core::ffi::c_void>>,
        psa: ConstPtr<SCRIPT_ANALYSIS>,
        tag_script: u32,
        tag_lang_sys: u32,
        tag_feature: u32,
        w_glyph_id: u16,
        c_max_alternates: i32,
        p_alternate_glyphs: MutPtr<u16>,
        pc_alternates: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("ScriptGetFontAlternateGlyphs")
    }
    fn ScriptGetFontFeatureTags(
        &self,
        hdc: super::Graphics::Gdi::HDC,
        psc: MutPtr<ConstPtr<::core::ffi::c_void>>,
        psa: ConstPtr<SCRIPT_ANALYSIS>,
        tag_script: u32,
        tag_lang_sys: u32,
        c_max_tags: i32,
        p_feature_tags: MutPtr<u32>,
        pc_tags: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("ScriptGetFontFeatureTags")
    }
    fn ScriptGetFontLanguageTags(
        &self,
        hdc: super::Graphics::Gdi::HDC,
        psc: MutPtr<ConstPtr<::core::ffi::c_void>>,
        psa: ConstPtr<SCRIPT_ANALYSIS>,
        tag_script: u32,
        c_max_tags: i32,
        p_langsys_tags: MutPtr<u32>,
        pc_tags: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("ScriptGetFontLanguageTags")
    }
    fn ScriptGetFontProperties(
        &self,
        hdc: super::Graphics::Gdi::HDC,
        psc: MutPtr<ConstPtr<::core::ffi::c_void>>,
        sfp: MutPtr<SCRIPT_FONTPROPERTIES>,
    ) -> crate::core::HRESULT {
        todo!("ScriptGetFontProperties")
    }
    fn ScriptGetFontScriptTags(
        &self,
        hdc: super::Graphics::Gdi::HDC,
        psc: MutPtr<ConstPtr<::core::ffi::c_void>>,
        psa: ConstPtr<SCRIPT_ANALYSIS>,
        c_max_tags: i32,
        p_script_tags: MutPtr<u32>,
        pc_tags: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("ScriptGetFontScriptTags")
    }
    fn ScriptGetGlyphABCWidth(
        &self,
        hdc: super::Graphics::Gdi::HDC,
        psc: MutPtr<ConstPtr<::core::ffi::c_void>>,
        w_glyph: u16,
        p_abc: MutPtr<super::Graphics::Gdi::ABC>,
    ) -> crate::core::HRESULT {
        todo!("ScriptGetGlyphABCWidth")
    }
    fn ScriptGetLogicalWidths(
        &self,
        psa: ConstPtr<SCRIPT_ANALYSIS>,
        c_chars: i32,
        c_glyphs: i32,
        pi_glyph_width: ConstPtr<i32>,
        pw_log_clust: ConstPtr<u16>,
        psva: ConstPtr<SCRIPT_VISATTR>,
        pi_dx: ConstPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("ScriptGetLogicalWidths")
    }
    fn ScriptGetProperties(
        &self,
        pp_sp: MutPtr<ConstPtr<ConstPtr<SCRIPT_PROPERTIES>>>,
        pi_num_scripts: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("ScriptGetProperties")
    }
    fn ScriptIsComplex(
        &self,
        pwc_in_chars: PCWSTR,
        c_in_chars: i32,
        dw_flags: SCRIPT_IS_COMPLEX_FLAGS,
    ) -> crate::core::HRESULT {
        todo!("ScriptIsComplex")
    }
    fn ScriptItemize(
        &self,
        pwc_in_chars: PCWSTR,
        c_in_chars: i32,
        c_max_items: i32,
        ps_control: ConstPtr<SCRIPT_CONTROL>,
        ps_state: ConstPtr<SCRIPT_STATE>,
        p_items: MutPtr<SCRIPT_ITEM>,
        pc_items: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("ScriptItemize")
    }
    fn ScriptItemizeOpenType(
        &self,
        pwc_in_chars: PCWSTR,
        c_in_chars: i32,
        c_max_items: i32,
        ps_control: ConstPtr<SCRIPT_CONTROL>,
        ps_state: ConstPtr<SCRIPT_STATE>,
        p_items: MutPtr<SCRIPT_ITEM>,
        p_script_tags: MutPtr<u32>,
        pc_items: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("ScriptItemizeOpenType")
    }
    fn ScriptJustify(
        &self,
        psva: ConstPtr<SCRIPT_VISATTR>,
        pi_advance: ConstPtr<i32>,
        c_glyphs: i32,
        i_dx: i32,
        i_min_kashida: i32,
        pi_justify: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("ScriptJustify")
    }
    fn ScriptLayout(
        &self,
        c_runs: i32,
        pb_level: ConstPtr<u8>,
        pi_visual_to_logical: MutPtr<i32>,
        pi_logical_to_visual: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("ScriptLayout")
    }
    fn ScriptPlace(
        &self,
        hdc: super::Graphics::Gdi::HDC,
        psc: MutPtr<ConstPtr<::core::ffi::c_void>>,
        pw_glyphs: ConstPtr<u16>,
        c_glyphs: i32,
        psva: ConstPtr<SCRIPT_VISATTR>,
        psa: MutPtr<SCRIPT_ANALYSIS>,
        pi_advance: MutPtr<i32>,
        p_goffset: MutPtr<GOFFSET>,
        p_abc: MutPtr<super::Graphics::Gdi::ABC>,
    ) -> crate::core::HRESULT {
        todo!("ScriptPlace")
    }
    fn ScriptPlaceOpenType(
        &self,
        hdc: super::Graphics::Gdi::HDC,
        psc: MutPtr<ConstPtr<::core::ffi::c_void>>,
        psa: MutPtr<SCRIPT_ANALYSIS>,
        tag_script: u32,
        tag_lang_sys: u32,
        rc_range_chars: ConstPtr<i32>,
        rp_range_properties: ConstPtr<ConstPtr<textrange_properties>>,
        c_ranges: i32,
        pwc_chars: PCWSTR,
        pw_log_clust: ConstPtr<u16>,
        p_char_props: ConstPtr<script_charprop>,
        c_chars: i32,
        pw_glyphs: ConstPtr<u16>,
        p_glyph_props: ConstPtr<script_glyphprop>,
        c_glyphs: i32,
        pi_advance: MutPtr<i32>,
        p_goffset: MutPtr<GOFFSET>,
        p_abc: MutPtr<super::Graphics::Gdi::ABC>,
    ) -> crate::core::HRESULT {
        todo!("ScriptPlaceOpenType")
    }
    fn ScriptPositionSingleGlyph(
        &self,
        hdc: super::Graphics::Gdi::HDC,
        psc: MutPtr<ConstPtr<::core::ffi::c_void>>,
        psa: ConstPtr<SCRIPT_ANALYSIS>,
        tag_script: u32,
        tag_lang_sys: u32,
        tag_feature: u32,
        l_parameter: i32,
        w_glyph_id: u16,
        i_advance: i32,
        g_offset: GOFFSET,
        pi_out_advance: MutPtr<i32>,
        p_out_goffset: MutPtr<GOFFSET>,
    ) -> crate::core::HRESULT {
        todo!("ScriptPositionSingleGlyph")
    }
    fn ScriptRecordDigitSubstitution(
        &self,
        locale: u32,
        psds: MutPtr<SCRIPT_DIGITSUBSTITUTE>,
    ) -> crate::core::HRESULT {
        todo!("ScriptRecordDigitSubstitution")
    }
    fn ScriptShape(
        &self,
        hdc: super::Graphics::Gdi::HDC,
        psc: MutPtr<ConstPtr<::core::ffi::c_void>>,
        pwc_chars: PCWSTR,
        c_chars: i32,
        c_max_glyphs: i32,
        psa: MutPtr<SCRIPT_ANALYSIS>,
        pw_out_glyphs: MutPtr<u16>,
        pw_log_clust: MutPtr<u16>,
        psva: MutPtr<SCRIPT_VISATTR>,
        pc_glyphs: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("ScriptShape")
    }
    fn ScriptShapeOpenType(
        &self,
        hdc: super::Graphics::Gdi::HDC,
        psc: MutPtr<ConstPtr<::core::ffi::c_void>>,
        psa: MutPtr<SCRIPT_ANALYSIS>,
        tag_script: u32,
        tag_lang_sys: u32,
        rc_range_chars: ConstPtr<i32>,
        rp_range_properties: ConstPtr<ConstPtr<textrange_properties>>,
        c_ranges: i32,
        pwc_chars: PCWSTR,
        c_chars: i32,
        c_max_glyphs: i32,
        pw_log_clust: MutPtr<u16>,
        p_char_props: MutPtr<script_charprop>,
        pw_out_glyphs: MutPtr<u16>,
        p_out_glyph_props: MutPtr<script_glyphprop>,
        pc_glyphs: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("ScriptShapeOpenType")
    }
    fn ScriptStringAnalyse(
        &self,
        hdc: super::Graphics::Gdi::HDC,
        p_string: ConstPtr<::core::ffi::c_void>,
        c_string: i32,
        c_glyphs: i32,
        i_charset: i32,
        dw_flags: u32,
        i_req_width: i32,
        ps_control: ConstPtr<SCRIPT_CONTROL>,
        ps_state: ConstPtr<SCRIPT_STATE>,
        pi_dx: ConstPtr<i32>,
        p_tabdef: ConstPtr<SCRIPT_TABDEF>,
        pb_in_class: ConstPtr<u8>,
        pssa: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("ScriptStringAnalyse")
    }
    fn ScriptStringCPtoX(
        &self,
        ssa: ConstPtr<::core::ffi::c_void>,
        icp: i32,
        f_trailing: super::Foundation::BOOL,
        p_x: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("ScriptStringCPtoX")
    }
    fn ScriptStringFree(
        &self,
        pssa: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> crate::core::HRESULT {
        todo!("ScriptStringFree")
    }
    fn ScriptStringGetLogicalWidths(
        &self,
        ssa: ConstPtr<::core::ffi::c_void>,
        pi_dx: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("ScriptStringGetLogicalWidths")
    }
    fn ScriptStringGetOrder(
        &self,
        ssa: ConstPtr<::core::ffi::c_void>,
        pu_order: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("ScriptStringGetOrder")
    }
    fn ScriptStringOut(
        &self,
        ssa: ConstPtr<::core::ffi::c_void>,
        i_x: i32,
        i_y: i32,
        u_options: super::Graphics::Gdi::ETO_OPTIONS,
        prc: ConstPtr<super::Foundation::RECT>,
        i_min_sel: i32,
        i_max_sel: i32,
        f_disabled: super::Foundation::BOOL,
    ) -> crate::core::HRESULT {
        todo!("ScriptStringOut")
    }
    fn ScriptStringValidate(&self, ssa: ConstPtr<::core::ffi::c_void>) -> crate::core::HRESULT {
        todo!("ScriptStringValidate")
    }
    fn ScriptStringXtoCP(
        &self,
        ssa: ConstPtr<::core::ffi::c_void>,
        i_x: i32,
        pi_ch: MutPtr<i32>,
        pi_trailing: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("ScriptStringXtoCP")
    }
    fn ScriptString_pLogAttr(&self, ssa: ConstPtr<::core::ffi::c_void>) -> MutPtr<SCRIPT_LOGATTR> {
        todo!("ScriptString_pLogAttr")
    }
    fn ScriptString_pSize(
        &self,
        ssa: ConstPtr<::core::ffi::c_void>,
    ) -> MutPtr<super::Foundation::SIZE> {
        todo!("ScriptString_pSize")
    }
    fn ScriptString_pcOutChars(&self, ssa: ConstPtr<::core::ffi::c_void>) -> MutPtr<i32> {
        todo!("ScriptString_pcOutChars")
    }
    fn ScriptSubstituteSingleGlyph(
        &self,
        hdc: super::Graphics::Gdi::HDC,
        psc: MutPtr<ConstPtr<::core::ffi::c_void>>,
        psa: ConstPtr<SCRIPT_ANALYSIS>,
        tag_script: u32,
        tag_lang_sys: u32,
        tag_feature: u32,
        l_parameter: i32,
        w_glyph_id: u16,
        pw_out_glyph_id: MutPtr<u16>,
    ) -> crate::core::HRESULT {
        todo!("ScriptSubstituteSingleGlyph")
    }
    fn ScriptTextOut(
        &self,
        hdc: super::Graphics::Gdi::HDC,
        psc: MutPtr<ConstPtr<::core::ffi::c_void>>,
        x: i32,
        y: i32,
        fu_options: u32,
        lprc: ConstPtr<super::Foundation::RECT>,
        psa: ConstPtr<SCRIPT_ANALYSIS>,
        pwc_reserved: PCWSTR,
        i_reserved: i32,
        pw_glyphs: ConstPtr<u16>,
        c_glyphs: i32,
        pi_advance: ConstPtr<i32>,
        pi_justify: ConstPtr<i32>,
        p_goffset: ConstPtr<GOFFSET>,
    ) -> crate::core::HRESULT {
        todo!("ScriptTextOut")
    }
    fn ScriptXtoCP(
        &self,
        i_x: i32,
        c_chars: i32,
        c_glyphs: i32,
        pw_log_clust: ConstPtr<u16>,
        psva: ConstPtr<SCRIPT_VISATTR>,
        pi_advance: ConstPtr<i32>,
        psa: ConstPtr<SCRIPT_ANALYSIS>,
        pi_cp: MutPtr<i32>,
        pi_trailing: MutPtr<i32>,
    ) -> crate::core::HRESULT {
        todo!("ScriptXtoCP")
    }
    fn SetCalendarInfoA(
        &self,
        locale: u32,
        calendar: u32,
        cal_type: u32,
        lp_cal_data: PCSTR,
    ) -> super::Foundation::BOOL {
        todo!("SetCalendarInfoA")
    }
    fn SetCalendarInfoW(
        &self,
        locale: u32,
        calendar: u32,
        cal_type: u32,
        lp_cal_data: PCWSTR,
    ) -> super::Foundation::BOOL {
        todo!("SetCalendarInfoW")
    }
    fn SetLocaleInfoA(
        &self,
        locale: u32,
        lc_type: u32,
        lp_lc_data: PCSTR,
    ) -> super::Foundation::BOOL {
        todo!("SetLocaleInfoA")
    }
    fn SetLocaleInfoW(
        &self,
        locale: u32,
        lc_type: u32,
        lp_lc_data: PCWSTR,
    ) -> super::Foundation::BOOL {
        todo!("SetLocaleInfoW")
    }
    fn SetProcessPreferredUILanguages(
        &self,
        dw_flags: u32,
        pwsz_languages_buffer: PCWSTR,
        pul_num_languages: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("SetProcessPreferredUILanguages")
    }
    fn SetThreadLocale(&self, locale: u32) -> super::Foundation::BOOL {
        todo!("SetThreadLocale")
    }
    fn SetThreadPreferredUILanguages(
        &self,
        dw_flags: u32,
        pwsz_languages_buffer: PCWSTR,
        pul_num_languages: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("SetThreadPreferredUILanguages")
    }
    fn SetThreadPreferredUILanguages2(
        &self,
        flags: u32,
        languages: PCWSTR,
        num_languages_set: MutPtr<u32>,
        snapshot: MutPtr<HSAVEDUILANGUAGES>,
    ) -> super::Foundation::BOOL {
        todo!("SetThreadPreferredUILanguages2")
    }
    fn SetThreadUILanguage(&self, lang_id: u16) -> u16 {
        todo!("SetThreadUILanguage")
    }
    fn SetUserGeoID(&self, geo_id: i32) -> super::Foundation::BOOL {
        todo!("SetUserGeoID")
    }
    fn SetUserGeoName(&self, geo_name: PCWSTR) -> super::Foundation::BOOL {
        todo!("SetUserGeoName")
    }
    fn TranslateCharsetInfo(
        &self,
        lp_src: MutPtr<u32>,
        lp_cs: MutPtr<CHARSETINFO>,
        dw_flags: TRANSLATE_CHARSET_INFO_FLAGS,
    ) -> super::Foundation::BOOL {
        todo!("TranslateCharsetInfo")
    }
    fn VerifyScripts(
        &self,
        dw_flags: u32,
        lp_locale_scripts: PCWSTR,
        cch_locale_scripts: i32,
        lp_test_scripts: PCWSTR,
        cch_test_scripts: i32,
    ) -> super::Foundation::BOOL {
        todo!("VerifyScripts")
    }
    fn WideCharToMultiByte(
        &self,
        code_page: u32,
        dw_flags: u32,
        lp_wide_char_str: PCWSTR,
        cch_wide_char: i32,
        lp_multi_byte_str: PSTR,
        cb_multi_byte: i32,
        lp_default_char: PCSTR,
        lp_used_default_char: MutPtr<i32>,
    ) -> i32 {
        todo!("WideCharToMultiByte")
    }
    fn lstrcatA(&self, lp_string_1: PSTR, lp_string_2: PCSTR) -> PSTR {
        todo!("lstrcatA")
    }
    fn lstrcatW(&self, lp_string_1: PWSTR, lp_string_2: PCWSTR) -> PWSTR {
        todo!("lstrcatW")
    }
    fn lstrcmpA(&self, lp_string_1: PCSTR, lp_string_2: PCSTR) -> i32 {
        todo!("lstrcmpA")
    }
    fn lstrcmpW(&self, lp_string_1: PCWSTR, lp_string_2: PCWSTR) -> i32 {
        todo!("lstrcmpW")
    }
    fn lstrcmpiA(&self, lp_string_1: PCSTR, lp_string_2: PCSTR) -> i32 {
        todo!("lstrcmpiA")
    }
    fn lstrcmpiW(&self, lp_string_1: PCWSTR, lp_string_2: PCWSTR) -> i32 {
        todo!("lstrcmpiW")
    }
    fn lstrcpyA(&self, lp_string_1: PSTR, lp_string_2: PCSTR) -> PSTR {
        todo!("lstrcpyA")
    }
    fn lstrcpyW(&self, lp_string_1: PWSTR, lp_string_2: PCWSTR) -> PWSTR {
        todo!("lstrcpyW")
    }
    fn lstrcpynA(&self, lp_string_1: PSTR, lp_string_2: PCSTR, i_max_length: i32) -> PSTR {
        todo!("lstrcpynA")
    }
    fn lstrcpynW(&self, lp_string_1: PWSTR, lp_string_2: PCWSTR, i_max_length: i32) -> PWSTR {
        todo!("lstrcpynW")
    }
    fn lstrlenA(&self, lp_string: PCSTR) -> i32 {
        todo!("lstrlenA")
    }
    fn lstrlenW(&self, lp_string: PCWSTR) -> i32 {
        todo!("lstrlenW")
    }
}
pub fn get_api(ctx: &crate::core::Win32Context) -> std::sync::Arc<dyn Api> {
    ctx.get::<dyn Api>()
}
