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
pub const ALTNUMPAD_BIT: u32 = 67108864u32;
pub const ATTACH_PARENT_PROCESS: u32 = 4294967295u32;
pub const BACKGROUND_BLUE: u32 = 16u32;
pub const BACKGROUND_GREEN: u32 = 32u32;
pub const BACKGROUND_INTENSITY: u32 = 128u32;
pub const BACKGROUND_RED: u32 = 64u32;
pub const CAPSLOCK_ON: u32 = 128u32;
pub struct CHAR_INFO {
    pub Char: CHAR_INFO_0,
    pub Attributes: u16,
}
impl ::core::marker::Copy for CHAR_INFO {}
impl ::core::clone::Clone for CHAR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for CHAR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Char == other.Char && self.Attributes == other.Attributes
    }
}
impl ::core::cmp::Eq for CHAR_INFO {}
impl FromIntoMemory for CHAR_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_Char = <CHAR_INFO_0 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_Attributes = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        Self {
            Char: f_Char,
            Attributes: f_Attributes,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.Char, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.Attributes, &mut into[2..2 + 2]);
    }
    fn size() -> usize {
        4
    }
}
pub struct CHAR_INFO_0 {
    data: [u8; 1],
}
impl ::core::default::Default for CHAR_INFO_0 {
    fn default() -> Self {
        Self { data: [0u8; 1] }
    }
}
impl ::core::marker::Copy for CHAR_INFO_0 {}
impl ::core::clone::Clone for CHAR_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for CHAR_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for CHAR_INFO_0 {}
impl FromIntoMemory for CHAR_INFO_0 {
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
pub const COMMON_LVB_GRID_HORIZONTAL: u32 = 1024u32;
pub const COMMON_LVB_GRID_LVERTICAL: u32 = 2048u32;
pub const COMMON_LVB_GRID_RVERTICAL: u32 = 4096u32;
pub const COMMON_LVB_LEADING_BYTE: u32 = 256u32;
pub const COMMON_LVB_REVERSE_VIDEO: u32 = 16384u32;
pub const COMMON_LVB_SBCSDBCS: u32 = 768u32;
pub const COMMON_LVB_TRAILING_BYTE: u32 = 512u32;
pub const COMMON_LVB_UNDERSCORE: u32 = 32768u32;
pub struct CONSOLE_CURSOR_INFO {
    pub dwSize: u32,
    pub bVisible: super::super::Foundation::BOOL,
}
impl ::core::marker::Copy for CONSOLE_CURSOR_INFO {}
impl ::core::clone::Clone for CONSOLE_CURSOR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONSOLE_CURSOR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_CURSOR_INFO")
            .field("dwSize", &self.dwSize)
            .field("bVisible", &self.bVisible)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CONSOLE_CURSOR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.bVisible == other.bVisible
    }
}
impl ::core::cmp::Eq for CONSOLE_CURSOR_INFO {}
impl FromIntoMemory for CONSOLE_CURSOR_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_bVisible =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            dwSize: f_dwSize,
            bVisible: f_bVisible,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.bVisible, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct CONSOLE_FONT_INFO {
    pub nFont: u32,
    pub dwFontSize: COORD,
}
impl ::core::marker::Copy for CONSOLE_FONT_INFO {}
impl ::core::clone::Clone for CONSOLE_FONT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONSOLE_FONT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_FONT_INFO")
            .field("nFont", &self.nFont)
            .field("dwFontSize", &self.dwFontSize)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CONSOLE_FONT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.nFont == other.nFont && self.dwFontSize == other.dwFontSize
    }
}
impl ::core::cmp::Eq for CONSOLE_FONT_INFO {}
impl FromIntoMemory for CONSOLE_FONT_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_nFont = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFontSize = <COORD as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            nFont: f_nFont,
            dwFontSize: f_dwFontSize,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.nFont, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFontSize, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct CONSOLE_FONT_INFOEX {
    pub cbSize: u32,
    pub nFont: u32,
    pub dwFontSize: COORD,
    pub FontFamily: u32,
    pub FontWeight: u32,
    pub FaceName: [u16; 32],
}
impl ::core::marker::Copy for CONSOLE_FONT_INFOEX {}
impl ::core::clone::Clone for CONSOLE_FONT_INFOEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONSOLE_FONT_INFOEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_FONT_INFOEX")
            .field("cbSize", &self.cbSize)
            .field("nFont", &self.nFont)
            .field("dwFontSize", &self.dwFontSize)
            .field("FontFamily", &self.FontFamily)
            .field("FontWeight", &self.FontWeight)
            .field("FaceName", &self.FaceName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CONSOLE_FONT_INFOEX {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.nFont == other.nFont
            && self.dwFontSize == other.dwFontSize
            && self.FontFamily == other.FontFamily
            && self.FontWeight == other.FontWeight
            && self.FaceName == other.FaceName
    }
}
impl ::core::cmp::Eq for CONSOLE_FONT_INFOEX {}
impl FromIntoMemory for CONSOLE_FONT_INFOEX {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 52);
        let f_cbSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_nFont = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwFontSize = <COORD as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_FontFamily = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_FontWeight = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_FaceName = <[u16; 32] as FromIntoMemory>::from_bytes(&from[20..20 + 32]);
        Self {
            cbSize: f_cbSize,
            nFont: f_nFont,
            dwFontSize: f_dwFontSize,
            FontFamily: f_FontFamily,
            FontWeight: f_FontWeight,
            FaceName: f_FaceName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 52);
        FromIntoMemory::into_bytes(self.cbSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.nFont, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwFontSize, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.FontFamily, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.FontWeight, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.FaceName, &mut into[20..20 + 32]);
    }
    fn size() -> usize {
        52
    }
}
pub const CONSOLE_FULLSCREEN: u32 = 1u32;
pub const CONSOLE_FULLSCREEN_HARDWARE: u32 = 2u32;
pub const CONSOLE_FULLSCREEN_MODE: u32 = 1u32;
pub struct CONSOLE_HISTORY_INFO {
    pub cbSize: u32,
    pub HistoryBufferSize: u32,
    pub NumberOfHistoryBuffers: u32,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for CONSOLE_HISTORY_INFO {}
impl ::core::clone::Clone for CONSOLE_HISTORY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONSOLE_HISTORY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_HISTORY_INFO")
            .field("cbSize", &self.cbSize)
            .field("HistoryBufferSize", &self.HistoryBufferSize)
            .field("NumberOfHistoryBuffers", &self.NumberOfHistoryBuffers)
            .field("dwFlags", &self.dwFlags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CONSOLE_HISTORY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.HistoryBufferSize == other.HistoryBufferSize
            && self.NumberOfHistoryBuffers == other.NumberOfHistoryBuffers
            && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for CONSOLE_HISTORY_INFO {}
impl FromIntoMemory for CONSOLE_HISTORY_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_cbSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_HistoryBufferSize = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_NumberOfHistoryBuffers = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            cbSize: f_cbSize,
            HistoryBufferSize: f_HistoryBufferSize,
            NumberOfHistoryBuffers: f_NumberOfHistoryBuffers,
            dwFlags: f_dwFlags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.cbSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.HistoryBufferSize, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.NumberOfHistoryBuffers, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CONSOLE_MODE(pub u32);
pub const ENABLE_PROCESSED_INPUT: CONSOLE_MODE = CONSOLE_MODE(1u32);
pub const ENABLE_LINE_INPUT: CONSOLE_MODE = CONSOLE_MODE(2u32);
pub const ENABLE_ECHO_INPUT: CONSOLE_MODE = CONSOLE_MODE(4u32);
pub const ENABLE_WINDOW_INPUT: CONSOLE_MODE = CONSOLE_MODE(8u32);
pub const ENABLE_MOUSE_INPUT: CONSOLE_MODE = CONSOLE_MODE(16u32);
pub const ENABLE_INSERT_MODE: CONSOLE_MODE = CONSOLE_MODE(32u32);
pub const ENABLE_QUICK_EDIT_MODE: CONSOLE_MODE = CONSOLE_MODE(64u32);
pub const ENABLE_EXTENDED_FLAGS: CONSOLE_MODE = CONSOLE_MODE(128u32);
pub const ENABLE_AUTO_POSITION: CONSOLE_MODE = CONSOLE_MODE(256u32);
pub const ENABLE_VIRTUAL_TERMINAL_INPUT: CONSOLE_MODE = CONSOLE_MODE(512u32);
pub const ENABLE_PROCESSED_OUTPUT: CONSOLE_MODE = CONSOLE_MODE(1u32);
pub const ENABLE_WRAP_AT_EOL_OUTPUT: CONSOLE_MODE = CONSOLE_MODE(2u32);
pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: CONSOLE_MODE = CONSOLE_MODE(4u32);
pub const DISABLE_NEWLINE_AUTO_RETURN: CONSOLE_MODE = CONSOLE_MODE(8u32);
pub const ENABLE_LVB_GRID_WORLDWIDE: CONSOLE_MODE = CONSOLE_MODE(16u32);
impl ::core::marker::Copy for CONSOLE_MODE {}
impl ::core::clone::Clone for CONSOLE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CONSOLE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CONSOLE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONSOLE_MODE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CONSOLE_MODE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CONSOLE_MODE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CONSOLE_MODE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CONSOLE_MODE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CONSOLE_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for CONSOLE_MODE {
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
pub const CONSOLE_MOUSE_DOWN: u32 = 8u32;
pub const CONSOLE_MOUSE_SELECTION: u32 = 4u32;
pub const CONSOLE_NO_SELECTION: u32 = 0u32;
pub struct CONSOLE_READCONSOLE_CONTROL {
    pub nLength: u32,
    pub nInitialChars: u32,
    pub dwCtrlWakeupMask: u32,
    pub dwControlKeyState: u32,
}
impl ::core::marker::Copy for CONSOLE_READCONSOLE_CONTROL {}
impl ::core::clone::Clone for CONSOLE_READCONSOLE_CONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONSOLE_READCONSOLE_CONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_READCONSOLE_CONTROL")
            .field("nLength", &self.nLength)
            .field("nInitialChars", &self.nInitialChars)
            .field("dwCtrlWakeupMask", &self.dwCtrlWakeupMask)
            .field("dwControlKeyState", &self.dwControlKeyState)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CONSOLE_READCONSOLE_CONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.nLength == other.nLength
            && self.nInitialChars == other.nInitialChars
            && self.dwCtrlWakeupMask == other.dwCtrlWakeupMask
            && self.dwControlKeyState == other.dwControlKeyState
    }
}
impl ::core::cmp::Eq for CONSOLE_READCONSOLE_CONTROL {}
impl FromIntoMemory for CONSOLE_READCONSOLE_CONTROL {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_nLength = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_nInitialChars = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwCtrlWakeupMask = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwControlKeyState = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            nLength: f_nLength,
            nInitialChars: f_nInitialChars,
            dwCtrlWakeupMask: f_dwCtrlWakeupMask,
            dwControlKeyState: f_dwControlKeyState,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.nLength, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.nInitialChars, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwCtrlWakeupMask, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwControlKeyState, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct CONSOLE_SCREEN_BUFFER_INFO {
    pub dwSize: COORD,
    pub dwCursorPosition: COORD,
    pub wAttributes: u16,
    pub srWindow: SMALL_RECT,
    pub dwMaximumWindowSize: COORD,
}
impl ::core::marker::Copy for CONSOLE_SCREEN_BUFFER_INFO {}
impl ::core::clone::Clone for CONSOLE_SCREEN_BUFFER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONSOLE_SCREEN_BUFFER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_SCREEN_BUFFER_INFO")
            .field("dwSize", &self.dwSize)
            .field("dwCursorPosition", &self.dwCursorPosition)
            .field("wAttributes", &self.wAttributes)
            .field("srWindow", &self.srWindow)
            .field("dwMaximumWindowSize", &self.dwMaximumWindowSize)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CONSOLE_SCREEN_BUFFER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwCursorPosition == other.dwCursorPosition
            && self.wAttributes == other.wAttributes
            && self.srWindow == other.srWindow
            && self.dwMaximumWindowSize == other.dwMaximumWindowSize
    }
}
impl ::core::cmp::Eq for CONSOLE_SCREEN_BUFFER_INFO {}
impl FromIntoMemory for CONSOLE_SCREEN_BUFFER_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 22);
        let f_dwSize = <COORD as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwCursorPosition = <COORD as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_wAttributes = <u16 as FromIntoMemory>::from_bytes(&from[8..8 + 2]);
        let f_srWindow = <SMALL_RECT as FromIntoMemory>::from_bytes(&from[10..10 + 8]);
        let f_dwMaximumWindowSize = <COORD as FromIntoMemory>::from_bytes(&from[18..18 + 4]);
        Self {
            dwSize: f_dwSize,
            dwCursorPosition: f_dwCursorPosition,
            wAttributes: f_wAttributes,
            srWindow: f_srWindow,
            dwMaximumWindowSize: f_dwMaximumWindowSize,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 22);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwCursorPosition, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.wAttributes, &mut into[8..8 + 2]);
        FromIntoMemory::into_bytes(self.srWindow, &mut into[10..10 + 8]);
        FromIntoMemory::into_bytes(self.dwMaximumWindowSize, &mut into[18..18 + 4]);
    }
    fn size() -> usize {
        22
    }
}
pub struct CONSOLE_SCREEN_BUFFER_INFOEX {
    pub cbSize: u32,
    pub dwSize: COORD,
    pub dwCursorPosition: COORD,
    pub wAttributes: u16,
    pub srWindow: SMALL_RECT,
    pub dwMaximumWindowSize: COORD,
    pub wPopupAttributes: u16,
    pub bFullscreenSupported: super::super::Foundation::BOOL,
    pub ColorTable: [u32; 16],
}
impl ::core::marker::Copy for CONSOLE_SCREEN_BUFFER_INFOEX {}
impl ::core::clone::Clone for CONSOLE_SCREEN_BUFFER_INFOEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONSOLE_SCREEN_BUFFER_INFOEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_SCREEN_BUFFER_INFOEX")
            .field("cbSize", &self.cbSize)
            .field("dwSize", &self.dwSize)
            .field("dwCursorPosition", &self.dwCursorPosition)
            .field("wAttributes", &self.wAttributes)
            .field("srWindow", &self.srWindow)
            .field("dwMaximumWindowSize", &self.dwMaximumWindowSize)
            .field("wPopupAttributes", &self.wPopupAttributes)
            .field("bFullscreenSupported", &self.bFullscreenSupported)
            .field("ColorTable", &self.ColorTable)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CONSOLE_SCREEN_BUFFER_INFOEX {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.dwSize == other.dwSize
            && self.dwCursorPosition == other.dwCursorPosition
            && self.wAttributes == other.wAttributes
            && self.srWindow == other.srWindow
            && self.dwMaximumWindowSize == other.dwMaximumWindowSize
            && self.wPopupAttributes == other.wPopupAttributes
            && self.bFullscreenSupported == other.bFullscreenSupported
            && self.ColorTable == other.ColorTable
    }
}
impl ::core::cmp::Eq for CONSOLE_SCREEN_BUFFER_INFOEX {}
impl FromIntoMemory for CONSOLE_SCREEN_BUFFER_INFOEX {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 96);
        let f_cbSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwSize = <COORD as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwCursorPosition = <COORD as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_wAttributes = <u16 as FromIntoMemory>::from_bytes(&from[12..12 + 2]);
        let f_srWindow = <SMALL_RECT as FromIntoMemory>::from_bytes(&from[14..14 + 8]);
        let f_dwMaximumWindowSize = <COORD as FromIntoMemory>::from_bytes(&from[22..22 + 4]);
        let f_wPopupAttributes = <u16 as FromIntoMemory>::from_bytes(&from[26..26 + 2]);
        let f_bFullscreenSupported =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_ColorTable = <[u32; 16] as FromIntoMemory>::from_bytes(&from[32..32 + 64]);
        Self {
            cbSize: f_cbSize,
            dwSize: f_dwSize,
            dwCursorPosition: f_dwCursorPosition,
            wAttributes: f_wAttributes,
            srWindow: f_srWindow,
            dwMaximumWindowSize: f_dwMaximumWindowSize,
            wPopupAttributes: f_wPopupAttributes,
            bFullscreenSupported: f_bFullscreenSupported,
            ColorTable: f_ColorTable,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 96);
        FromIntoMemory::into_bytes(self.cbSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwCursorPosition, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.wAttributes, &mut into[12..12 + 2]);
        FromIntoMemory::into_bytes(self.srWindow, &mut into[14..14 + 8]);
        FromIntoMemory::into_bytes(self.dwMaximumWindowSize, &mut into[22..22 + 4]);
        FromIntoMemory::into_bytes(self.wPopupAttributes, &mut into[26..26 + 2]);
        FromIntoMemory::into_bytes(self.bFullscreenSupported, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.ColorTable, &mut into[32..32 + 64]);
    }
    fn size() -> usize {
        96
    }
}
pub struct CONSOLE_SELECTION_INFO {
    pub dwFlags: u32,
    pub dwSelectionAnchor: COORD,
    pub srSelection: SMALL_RECT,
}
impl ::core::marker::Copy for CONSOLE_SELECTION_INFO {}
impl ::core::clone::Clone for CONSOLE_SELECTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONSOLE_SELECTION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_SELECTION_INFO")
            .field("dwFlags", &self.dwFlags)
            .field("dwSelectionAnchor", &self.dwSelectionAnchor)
            .field("srSelection", &self.srSelection)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CONSOLE_SELECTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags
            && self.dwSelectionAnchor == other.dwSelectionAnchor
            && self.srSelection == other.srSelection
    }
}
impl ::core::cmp::Eq for CONSOLE_SELECTION_INFO {}
impl FromIntoMemory for CONSOLE_SELECTION_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwSelectionAnchor = <COORD as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_srSelection = <SMALL_RECT as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        Self {
            dwFlags: f_dwFlags,
            dwSelectionAnchor: f_dwSelectionAnchor,
            srSelection: f_srSelection,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwSelectionAnchor, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.srSelection, &mut into[8..8 + 8]);
    }
    fn size() -> usize {
        16
    }
}
pub const CONSOLE_SELECTION_IN_PROGRESS: u32 = 1u32;
pub const CONSOLE_SELECTION_NOT_EMPTY: u32 = 2u32;
pub const CONSOLE_TEXTMODE_BUFFER: u32 = 1u32;
pub const CONSOLE_WINDOWED_MODE: u32 = 2u32;
pub struct COORD {
    pub X: i16,
    pub Y: i16,
}
impl ::core::marker::Copy for COORD {}
impl ::core::clone::Clone for COORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COORD")
            .field("X", &self.X)
            .field("Y", &self.Y)
            .finish()
    }
}
impl ::core::cmp::PartialEq for COORD {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y
    }
}
impl ::core::cmp::Eq for COORD {}
impl FromIntoMemory for COORD {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_X = <i16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_Y = <i16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        Self { X: f_X, Y: f_Y }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.X, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.Y, &mut into[2..2 + 2]);
    }
    fn size() -> usize {
        4
    }
}
pub const CTRL_BREAK_EVENT: u32 = 1u32;
pub const CTRL_CLOSE_EVENT: u32 = 2u32;
pub const CTRL_C_EVENT: u32 = 0u32;
pub const CTRL_LOGOFF_EVENT: u32 = 5u32;
pub const CTRL_SHUTDOWN_EVENT: u32 = 6u32;
pub const DOUBLE_CLICK: u32 = 2u32;
pub const ENHANCED_KEY: u32 = 256u32;
pub const FOCUS_EVENT: u32 = 16u32;
pub struct FOCUS_EVENT_RECORD {
    pub bSetFocus: super::super::Foundation::BOOL,
}
impl ::core::marker::Copy for FOCUS_EVENT_RECORD {}
impl ::core::clone::Clone for FOCUS_EVENT_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FOCUS_EVENT_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FOCUS_EVENT_RECORD")
            .field("bSetFocus", &self.bSetFocus)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FOCUS_EVENT_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.bSetFocus == other.bSetFocus
    }
}
impl ::core::cmp::Eq for FOCUS_EVENT_RECORD {}
impl FromIntoMemory for FOCUS_EVENT_RECORD {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_bSetFocus =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            bSetFocus: f_bSetFocus,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.bSetFocus, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub const FOREGROUND_BLUE: u32 = 1u32;
pub const FOREGROUND_GREEN: u32 = 2u32;
pub const FOREGROUND_INTENSITY: u32 = 8u32;
pub const FOREGROUND_RED: u32 = 4u32;
pub const FROM_LEFT_1ST_BUTTON_PRESSED: u32 = 1u32;
pub const FROM_LEFT_2ND_BUTTON_PRESSED: u32 = 4u32;
pub const FROM_LEFT_3RD_BUTTON_PRESSED: u32 = 8u32;
pub const FROM_LEFT_4TH_BUTTON_PRESSED: u32 = 16u32;
pub const HISTORY_NO_DUP_FLAG: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HPCON(pub PtrDiffRepr);
impl HPCON {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HPCON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HPCON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HPCON {}
impl ::core::hash::Hash for HPCON {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HPCON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HPCON").field(&self.0).finish()
    }
}
impl FromIntoMemory for HPCON {
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
pub struct INPUT_RECORD {
    pub EventType: u16,
    pub Event: INPUT_RECORD_0,
}
impl ::core::marker::Copy for INPUT_RECORD {}
impl ::core::clone::Clone for INPUT_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for INPUT_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.EventType == other.EventType && self.Event == other.Event
    }
}
impl ::core::cmp::Eq for INPUT_RECORD {}
impl FromIntoMemory for INPUT_RECORD {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_EventType = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_Event = <INPUT_RECORD_0 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            EventType: f_EventType,
            Event: f_Event,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.EventType, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.Event, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct INPUT_RECORD_0 {
    data: [u8; 4],
}
impl ::core::default::Default for INPUT_RECORD_0 {
    fn default() -> Self {
        Self { data: [0u8; 4] }
    }
}
impl ::core::marker::Copy for INPUT_RECORD_0 {}
impl ::core::clone::Clone for INPUT_RECORD_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for INPUT_RECORD_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for INPUT_RECORD_0 {}
impl FromIntoMemory for INPUT_RECORD_0 {
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
pub const KEY_EVENT: u32 = 1u32;
pub struct KEY_EVENT_RECORD {
    pub bKeyDown: super::super::Foundation::BOOL,
    pub wRepeatCount: u16,
    pub wVirtualKeyCode: u16,
    pub wVirtualScanCode: u16,
    pub uChar: KEY_EVENT_RECORD_0,
    pub dwControlKeyState: u32,
}
impl ::core::marker::Copy for KEY_EVENT_RECORD {}
impl ::core::clone::Clone for KEY_EVENT_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for KEY_EVENT_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.bKeyDown == other.bKeyDown
            && self.wRepeatCount == other.wRepeatCount
            && self.wVirtualKeyCode == other.wVirtualKeyCode
            && self.wVirtualScanCode == other.wVirtualScanCode
            && self.uChar == other.uChar
            && self.dwControlKeyState == other.dwControlKeyState
    }
}
impl ::core::cmp::Eq for KEY_EVENT_RECORD {}
impl FromIntoMemory for KEY_EVENT_RECORD {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_bKeyDown =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_wRepeatCount = <u16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_wVirtualKeyCode = <u16 as FromIntoMemory>::from_bytes(&from[6..6 + 2]);
        let f_wVirtualScanCode = <u16 as FromIntoMemory>::from_bytes(&from[8..8 + 2]);
        let f_uChar = <KEY_EVENT_RECORD_0 as FromIntoMemory>::from_bytes(&from[10..10 + 1]);
        let f_dwControlKeyState = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            bKeyDown: f_bKeyDown,
            wRepeatCount: f_wRepeatCount,
            wVirtualKeyCode: f_wVirtualKeyCode,
            wVirtualScanCode: f_wVirtualScanCode,
            uChar: f_uChar,
            dwControlKeyState: f_dwControlKeyState,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.bKeyDown, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.wRepeatCount, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.wVirtualKeyCode, &mut into[6..6 + 2]);
        FromIntoMemory::into_bytes(self.wVirtualScanCode, &mut into[8..8 + 2]);
        FromIntoMemory::into_bytes(self.uChar, &mut into[10..10 + 1]);
        FromIntoMemory::into_bytes(self.dwControlKeyState, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct KEY_EVENT_RECORD_0 {
    data: [u8; 1],
}
impl ::core::default::Default for KEY_EVENT_RECORD_0 {
    fn default() -> Self {
        Self { data: [0u8; 1] }
    }
}
impl ::core::marker::Copy for KEY_EVENT_RECORD_0 {}
impl ::core::clone::Clone for KEY_EVENT_RECORD_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for KEY_EVENT_RECORD_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for KEY_EVENT_RECORD_0 {}
impl FromIntoMemory for KEY_EVENT_RECORD_0 {
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
pub const LEFT_ALT_PRESSED: u32 = 2u32;
pub const LEFT_CTRL_PRESSED: u32 = 8u32;
pub const MENU_EVENT: u32 = 8u32;
pub struct MENU_EVENT_RECORD {
    pub dwCommandId: u32,
}
impl ::core::marker::Copy for MENU_EVENT_RECORD {}
impl ::core::clone::Clone for MENU_EVENT_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MENU_EVENT_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MENU_EVENT_RECORD")
            .field("dwCommandId", &self.dwCommandId)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MENU_EVENT_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.dwCommandId == other.dwCommandId
    }
}
impl ::core::cmp::Eq for MENU_EVENT_RECORD {}
impl FromIntoMemory for MENU_EVENT_RECORD {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_dwCommandId = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            dwCommandId: f_dwCommandId,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.dwCommandId, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub const MOUSE_EVENT: u32 = 2u32;
pub struct MOUSE_EVENT_RECORD {
    pub dwMousePosition: COORD,
    pub dwButtonState: u32,
    pub dwControlKeyState: u32,
    pub dwEventFlags: u32,
}
impl ::core::marker::Copy for MOUSE_EVENT_RECORD {}
impl ::core::clone::Clone for MOUSE_EVENT_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MOUSE_EVENT_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MOUSE_EVENT_RECORD")
            .field("dwMousePosition", &self.dwMousePosition)
            .field("dwButtonState", &self.dwButtonState)
            .field("dwControlKeyState", &self.dwControlKeyState)
            .field("dwEventFlags", &self.dwEventFlags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MOUSE_EVENT_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.dwMousePosition == other.dwMousePosition
            && self.dwButtonState == other.dwButtonState
            && self.dwControlKeyState == other.dwControlKeyState
            && self.dwEventFlags == other.dwEventFlags
    }
}
impl ::core::cmp::Eq for MOUSE_EVENT_RECORD {}
impl FromIntoMemory for MOUSE_EVENT_RECORD {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dwMousePosition = <COORD as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwButtonState = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwControlKeyState = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwEventFlags = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            dwMousePosition: f_dwMousePosition,
            dwButtonState: f_dwButtonState,
            dwControlKeyState: f_dwControlKeyState,
            dwEventFlags: f_dwEventFlags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dwMousePosition, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwButtonState, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwControlKeyState, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwEventFlags, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub const MOUSE_HWHEELED: u32 = 8u32;
pub const MOUSE_MOVED: u32 = 1u32;
pub const MOUSE_WHEELED: u32 = 4u32;
pub const NLS_ALPHANUMERIC: u32 = 0u32;
pub const NLS_DBCSCHAR: u32 = 65536u32;
pub const NLS_HIRAGANA: u32 = 262144u32;
pub const NLS_IME_CONVERSION: u32 = 8388608u32;
pub const NLS_IME_DISABLE: u32 = 536870912u32;
pub const NLS_KATAKANA: u32 = 131072u32;
pub const NLS_ROMAN: u32 = 4194304u32;
pub const NUMLOCK_ON: u32 = 32u32;
pub type PHANDLER_ROUTINE = StdCallFnPtr<(u32,), super::super::Foundation::BOOL>;
pub const PSEUDOCONSOLE_INHERIT_CURSOR: u32 = 1u32;
pub const RIGHTMOST_BUTTON_PRESSED: u32 = 2u32;
pub const RIGHT_ALT_PRESSED: u32 = 1u32;
pub const RIGHT_CTRL_PRESSED: u32 = 4u32;
pub const SCROLLLOCK_ON: u32 = 64u32;
pub const SHIFT_PRESSED: u32 = 16u32;
pub struct SMALL_RECT {
    pub Left: i16,
    pub Top: i16,
    pub Right: i16,
    pub Bottom: i16,
}
impl ::core::marker::Copy for SMALL_RECT {}
impl ::core::clone::Clone for SMALL_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SMALL_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SMALL_RECT")
            .field("Left", &self.Left)
            .field("Top", &self.Top)
            .field("Right", &self.Right)
            .field("Bottom", &self.Bottom)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SMALL_RECT {
    fn eq(&self, other: &Self) -> bool {
        self.Left == other.Left
            && self.Top == other.Top
            && self.Right == other.Right
            && self.Bottom == other.Bottom
    }
}
impl ::core::cmp::Eq for SMALL_RECT {}
impl FromIntoMemory for SMALL_RECT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Left = <i16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_Top = <i16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_Right = <i16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_Bottom = <i16 as FromIntoMemory>::from_bytes(&from[6..6 + 2]);
        Self {
            Left: f_Left,
            Top: f_Top,
            Right: f_Right,
            Bottom: f_Bottom,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Left, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.Top, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.Right, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.Bottom, &mut into[6..6 + 2]);
    }
    fn size() -> usize {
        8
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STD_HANDLE(pub u32);
pub const STD_INPUT_HANDLE: STD_HANDLE = STD_HANDLE(4294967286u32);
pub const STD_OUTPUT_HANDLE: STD_HANDLE = STD_HANDLE(4294967285u32);
pub const STD_ERROR_HANDLE: STD_HANDLE = STD_HANDLE(4294967284u32);
impl ::core::marker::Copy for STD_HANDLE {}
impl ::core::clone::Clone for STD_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STD_HANDLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STD_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STD_HANDLE").field(&self.0).finish()
    }
}
impl FromIntoMemory for STD_HANDLE {
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
pub const WINDOW_BUFFER_SIZE_EVENT: u32 = 4u32;
pub struct WINDOW_BUFFER_SIZE_RECORD {
    pub dwSize: COORD,
}
impl ::core::marker::Copy for WINDOW_BUFFER_SIZE_RECORD {}
impl ::core::clone::Clone for WINDOW_BUFFER_SIZE_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINDOW_BUFFER_SIZE_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINDOW_BUFFER_SIZE_RECORD")
            .field("dwSize", &self.dwSize)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WINDOW_BUFFER_SIZE_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
    }
}
impl ::core::cmp::Eq for WINDOW_BUFFER_SIZE_RECORD {}
impl FromIntoMemory for WINDOW_BUFFER_SIZE_RECORD {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_dwSize = <COORD as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self { dwSize: f_dwSize }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub trait Api {
    fn AddConsoleAliasA(
        &self,
        source: PCSTR,
        target: PCSTR,
        exe_name: PCSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("AddConsoleAliasA")
    }
    fn AddConsoleAliasW(
        &self,
        source: PCWSTR,
        target: PCWSTR,
        exe_name: PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("AddConsoleAliasW")
    }
    fn AllocConsole(&self) -> super::super::Foundation::BOOL {
        todo!("AllocConsole")
    }
    fn AttachConsole(&self, dw_process_id: u32) -> super::super::Foundation::BOOL {
        todo!("AttachConsole")
    }
    fn ClosePseudoConsole(&self, h_pc: HPCON) {
        todo!("ClosePseudoConsole")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Security'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateConsoleScreenBuffer(
        &self,
        dw_desired_access: u32,
        dw_share_mode: u32,
        lp_security_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        dw_flags: u32,
        lp_screen_buffer_data: MutPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateConsoleScreenBuffer")
    }
    fn CreatePseudoConsole(
        &self,
        size: COORD,
        h_input: super::super::Foundation::HANDLE,
        h_output: super::super::Foundation::HANDLE,
        dw_flags: u32,
        ph_pc: MutPtr<HPCON>,
    ) -> crate::core::HRESULT {
        todo!("CreatePseudoConsole")
    }
    fn ExpungeConsoleCommandHistoryA(&self, exe_name: PCSTR) {
        todo!("ExpungeConsoleCommandHistoryA")
    }
    fn ExpungeConsoleCommandHistoryW(&self, exe_name: PCWSTR) {
        todo!("ExpungeConsoleCommandHistoryW")
    }
    fn FillConsoleOutputAttribute(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        w_attribute: u16,
        n_length: u32,
        dw_write_coord: COORD,
        lp_number_of_attrs_written: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("FillConsoleOutputAttribute")
    }
    fn FillConsoleOutputCharacterA(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        c_character: super::super::Foundation::CHAR,
        n_length: u32,
        dw_write_coord: COORD,
        lp_number_of_chars_written: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("FillConsoleOutputCharacterA")
    }
    fn FillConsoleOutputCharacterW(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        c_character: u16,
        n_length: u32,
        dw_write_coord: COORD,
        lp_number_of_chars_written: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("FillConsoleOutputCharacterW")
    }
    fn FlushConsoleInputBuffer(
        &self,
        h_console_input: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("FlushConsoleInputBuffer")
    }
    fn FreeConsole(&self) -> super::super::Foundation::BOOL {
        todo!("FreeConsole")
    }
    fn GenerateConsoleCtrlEvent(
        &self,
        dw_ctrl_event: u32,
        dw_process_group_id: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("GenerateConsoleCtrlEvent")
    }
    fn GetConsoleAliasA(
        &self,
        source: PCSTR,
        target_buffer: PSTR,
        target_buffer_length: u32,
        exe_name: PCSTR,
    ) -> u32 {
        todo!("GetConsoleAliasA")
    }
    fn GetConsoleAliasExesA(&self, exe_name_buffer: PSTR, exe_name_buffer_length: u32) -> u32 {
        todo!("GetConsoleAliasExesA")
    }
    fn GetConsoleAliasExesLengthA(&self) -> u32 {
        todo!("GetConsoleAliasExesLengthA")
    }
    fn GetConsoleAliasExesLengthW(&self) -> u32 {
        todo!("GetConsoleAliasExesLengthW")
    }
    fn GetConsoleAliasExesW(&self, exe_name_buffer: PWSTR, exe_name_buffer_length: u32) -> u32 {
        todo!("GetConsoleAliasExesW")
    }
    fn GetConsoleAliasW(
        &self,
        source: PCWSTR,
        target_buffer: PWSTR,
        target_buffer_length: u32,
        exe_name: PCWSTR,
    ) -> u32 {
        todo!("GetConsoleAliasW")
    }
    fn GetConsoleAliasesA(
        &self,
        alias_buffer: PSTR,
        alias_buffer_length: u32,
        exe_name: PCSTR,
    ) -> u32 {
        todo!("GetConsoleAliasesA")
    }
    fn GetConsoleAliasesLengthA(&self, exe_name: PCSTR) -> u32 {
        todo!("GetConsoleAliasesLengthA")
    }
    fn GetConsoleAliasesLengthW(&self, exe_name: PCWSTR) -> u32 {
        todo!("GetConsoleAliasesLengthW")
    }
    fn GetConsoleAliasesW(
        &self,
        alias_buffer: PWSTR,
        alias_buffer_length: u32,
        exe_name: PCWSTR,
    ) -> u32 {
        todo!("GetConsoleAliasesW")
    }
    fn GetConsoleCP(&self) -> u32 {
        todo!("GetConsoleCP")
    }
    fn GetConsoleCommandHistoryA(
        &self,
        commands: PSTR,
        command_buffer_length: u32,
        exe_name: PCSTR,
    ) -> u32 {
        todo!("GetConsoleCommandHistoryA")
    }
    fn GetConsoleCommandHistoryLengthA(&self, exe_name: PCSTR) -> u32 {
        todo!("GetConsoleCommandHistoryLengthA")
    }
    fn GetConsoleCommandHistoryLengthW(&self, exe_name: PCWSTR) -> u32 {
        todo!("GetConsoleCommandHistoryLengthW")
    }
    fn GetConsoleCommandHistoryW(
        &self,
        commands: PWSTR,
        command_buffer_length: u32,
        exe_name: PCWSTR,
    ) -> u32 {
        todo!("GetConsoleCommandHistoryW")
    }
    fn GetConsoleCursorInfo(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        lp_console_cursor_info: MutPtr<CONSOLE_CURSOR_INFO>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetConsoleCursorInfo")
    }
    fn GetConsoleDisplayMode(&self, lp_mode_flags: MutPtr<u32>) -> super::super::Foundation::BOOL {
        todo!("GetConsoleDisplayMode")
    }
    fn GetConsoleHistoryInfo(
        &self,
        lp_console_history_info: MutPtr<CONSOLE_HISTORY_INFO>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetConsoleHistoryInfo")
    }
    fn GetConsoleMode(
        &self,
        h_console_handle: super::super::Foundation::HANDLE,
        lp_mode: MutPtr<CONSOLE_MODE>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetConsoleMode")
    }
    fn GetConsoleOriginalTitleA(&self, lp_console_title: PSTR, n_size: u32) -> u32 {
        todo!("GetConsoleOriginalTitleA")
    }
    fn GetConsoleOriginalTitleW(&self, lp_console_title: PWSTR, n_size: u32) -> u32 {
        todo!("GetConsoleOriginalTitleW")
    }
    fn GetConsoleOutputCP(&self) -> u32 {
        todo!("GetConsoleOutputCP")
    }
    fn GetConsoleProcessList(&self, lpdw_process_list: MutPtr<u32>, dw_process_count: u32) -> u32 {
        todo!("GetConsoleProcessList")
    }
    fn GetConsoleScreenBufferInfo(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        lp_console_screen_buffer_info: MutPtr<CONSOLE_SCREEN_BUFFER_INFO>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetConsoleScreenBufferInfo")
    }
    fn GetConsoleScreenBufferInfoEx(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        lp_console_screen_buffer_info_ex: MutPtr<CONSOLE_SCREEN_BUFFER_INFOEX>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetConsoleScreenBufferInfoEx")
    }
    fn GetConsoleSelectionInfo(
        &self,
        lp_console_selection_info: MutPtr<CONSOLE_SELECTION_INFO>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetConsoleSelectionInfo")
    }
    fn GetConsoleTitleA(&self, lp_console_title: PSTR, n_size: u32) -> u32 {
        todo!("GetConsoleTitleA")
    }
    fn GetConsoleTitleW(&self, lp_console_title: PWSTR, n_size: u32) -> u32 {
        todo!("GetConsoleTitleW")
    }
    fn GetConsoleWindow(&self) -> super::super::Foundation::HWND {
        todo!("GetConsoleWindow")
    }
    fn GetCurrentConsoleFont(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        b_maximum_window: super::super::Foundation::BOOL,
        lp_console_current_font: MutPtr<CONSOLE_FONT_INFO>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetCurrentConsoleFont")
    }
    fn GetCurrentConsoleFontEx(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        b_maximum_window: super::super::Foundation::BOOL,
        lp_console_current_font_ex: MutPtr<CONSOLE_FONT_INFOEX>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetCurrentConsoleFontEx")
    }
    fn GetNumberOfConsoleInputEvents(
        &self,
        h_console_input: super::super::Foundation::HANDLE,
        lp_number_of_events: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetNumberOfConsoleInputEvents")
    }
    fn GetNumberOfConsoleMouseButtons(
        &self,
        lp_number_of_mouse_buttons: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetNumberOfConsoleMouseButtons")
    }
    fn GetStdHandle(&self, n_std_handle: STD_HANDLE) -> super::super::Foundation::HANDLE {
        todo!("GetStdHandle")
    }
    fn PeekConsoleInputA(
        &self,
        h_console_input: super::super::Foundation::HANDLE,
        lp_buffer: MutPtr<INPUT_RECORD>,
        n_length: u32,
        lp_number_of_events_read: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("PeekConsoleInputA")
    }
    fn PeekConsoleInputW(
        &self,
        h_console_input: super::super::Foundation::HANDLE,
        lp_buffer: MutPtr<INPUT_RECORD>,
        n_length: u32,
        lp_number_of_events_read: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("PeekConsoleInputW")
    }
    fn ReadConsoleA(
        &self,
        h_console_input: super::super::Foundation::HANDLE,
        lp_buffer: MutPtr<::core::ffi::c_void>,
        n_number_of_chars_to_read: u32,
        lp_number_of_chars_read: MutPtr<u32>,
        p_input_control: ConstPtr<CONSOLE_READCONSOLE_CONTROL>,
    ) -> super::super::Foundation::BOOL {
        todo!("ReadConsoleA")
    }
    fn ReadConsoleInputA(
        &self,
        h_console_input: super::super::Foundation::HANDLE,
        lp_buffer: MutPtr<INPUT_RECORD>,
        n_length: u32,
        lp_number_of_events_read: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("ReadConsoleInputA")
    }
    fn ReadConsoleInputW(
        &self,
        h_console_input: super::super::Foundation::HANDLE,
        lp_buffer: MutPtr<INPUT_RECORD>,
        n_length: u32,
        lp_number_of_events_read: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("ReadConsoleInputW")
    }
    fn ReadConsoleOutputA(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        lp_buffer: MutPtr<CHAR_INFO>,
        dw_buffer_size: COORD,
        dw_buffer_coord: COORD,
        lp_read_region: MutPtr<SMALL_RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!("ReadConsoleOutputA")
    }
    fn ReadConsoleOutputAttribute(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        lp_attribute: MutPtr<u16>,
        n_length: u32,
        dw_read_coord: COORD,
        lp_number_of_attrs_read: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("ReadConsoleOutputAttribute")
    }
    fn ReadConsoleOutputCharacterA(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        lp_character: PSTR,
        n_length: u32,
        dw_read_coord: COORD,
        lp_number_of_chars_read: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("ReadConsoleOutputCharacterA")
    }
    fn ReadConsoleOutputCharacterW(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        lp_character: PWSTR,
        n_length: u32,
        dw_read_coord: COORD,
        lp_number_of_chars_read: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("ReadConsoleOutputCharacterW")
    }
    fn ReadConsoleOutputW(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        lp_buffer: MutPtr<CHAR_INFO>,
        dw_buffer_size: COORD,
        dw_buffer_coord: COORD,
        lp_read_region: MutPtr<SMALL_RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!("ReadConsoleOutputW")
    }
    fn ReadConsoleW(
        &self,
        h_console_input: super::super::Foundation::HANDLE,
        lp_buffer: MutPtr<::core::ffi::c_void>,
        n_number_of_chars_to_read: u32,
        lp_number_of_chars_read: MutPtr<u32>,
        p_input_control: ConstPtr<CONSOLE_READCONSOLE_CONTROL>,
    ) -> super::super::Foundation::BOOL {
        todo!("ReadConsoleW")
    }
    fn ResizePseudoConsole(&self, h_pc: HPCON, size: COORD) -> crate::core::HRESULT {
        todo!("ResizePseudoConsole")
    }
    fn ScrollConsoleScreenBufferA(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        lp_scroll_rectangle: ConstPtr<SMALL_RECT>,
        lp_clip_rectangle: ConstPtr<SMALL_RECT>,
        dw_destination_origin: COORD,
        lp_fill: ConstPtr<CHAR_INFO>,
    ) -> super::super::Foundation::BOOL {
        todo!("ScrollConsoleScreenBufferA")
    }
    fn ScrollConsoleScreenBufferW(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        lp_scroll_rectangle: ConstPtr<SMALL_RECT>,
        lp_clip_rectangle: ConstPtr<SMALL_RECT>,
        dw_destination_origin: COORD,
        lp_fill: ConstPtr<CHAR_INFO>,
    ) -> super::super::Foundation::BOOL {
        todo!("ScrollConsoleScreenBufferW")
    }
    fn SetConsoleActiveScreenBuffer(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("SetConsoleActiveScreenBuffer")
    }
    fn SetConsoleCP(&self, w_code_page_id: u32) -> super::super::Foundation::BOOL {
        todo!("SetConsoleCP")
    }
    fn SetConsoleCtrlHandler(
        &self,
        handler_routine: PHANDLER_ROUTINE,
        add: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!("SetConsoleCtrlHandler")
    }
    fn SetConsoleCursorInfo(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        lp_console_cursor_info: ConstPtr<CONSOLE_CURSOR_INFO>,
    ) -> super::super::Foundation::BOOL {
        todo!("SetConsoleCursorInfo")
    }
    fn SetConsoleCursorPosition(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        dw_cursor_position: COORD,
    ) -> super::super::Foundation::BOOL {
        todo!("SetConsoleCursorPosition")
    }
    fn SetConsoleDisplayMode(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        dw_flags: u32,
        lp_new_screen_buffer_dimensions: MutPtr<COORD>,
    ) -> super::super::Foundation::BOOL {
        todo!("SetConsoleDisplayMode")
    }
    fn SetConsoleHistoryInfo(
        &self,
        lp_console_history_info: ConstPtr<CONSOLE_HISTORY_INFO>,
    ) -> super::super::Foundation::BOOL {
        todo!("SetConsoleHistoryInfo")
    }
    fn SetConsoleMode(
        &self,
        h_console_handle: super::super::Foundation::HANDLE,
        dw_mode: CONSOLE_MODE,
    ) -> super::super::Foundation::BOOL {
        todo!("SetConsoleMode")
    }
    fn SetConsoleNumberOfCommandsA(
        &self,
        number: u32,
        exe_name: PCSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("SetConsoleNumberOfCommandsA")
    }
    fn SetConsoleNumberOfCommandsW(
        &self,
        number: u32,
        exe_name: PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("SetConsoleNumberOfCommandsW")
    }
    fn SetConsoleOutputCP(&self, w_code_page_id: u32) -> super::super::Foundation::BOOL {
        todo!("SetConsoleOutputCP")
    }
    fn SetConsoleScreenBufferInfoEx(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        lp_console_screen_buffer_info_ex: ConstPtr<CONSOLE_SCREEN_BUFFER_INFOEX>,
    ) -> super::super::Foundation::BOOL {
        todo!("SetConsoleScreenBufferInfoEx")
    }
    fn SetConsoleScreenBufferSize(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        dw_size: COORD,
    ) -> super::super::Foundation::BOOL {
        todo!("SetConsoleScreenBufferSize")
    }
    fn SetConsoleTextAttribute(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        w_attributes: u16,
    ) -> super::super::Foundation::BOOL {
        todo!("SetConsoleTextAttribute")
    }
    fn SetConsoleTitleA(&self, lp_console_title: PCSTR) -> super::super::Foundation::BOOL {
        todo!("SetConsoleTitleA")
    }
    fn SetConsoleTitleW(&self, lp_console_title: PCWSTR) -> super::super::Foundation::BOOL {
        todo!("SetConsoleTitleW")
    }
    fn SetConsoleWindowInfo(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        b_absolute: super::super::Foundation::BOOL,
        lp_console_window: ConstPtr<SMALL_RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!("SetConsoleWindowInfo")
    }
    fn SetCurrentConsoleFontEx(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        b_maximum_window: super::super::Foundation::BOOL,
        lp_console_current_font_ex: ConstPtr<CONSOLE_FONT_INFOEX>,
    ) -> super::super::Foundation::BOOL {
        todo!("SetCurrentConsoleFontEx")
    }
    fn SetStdHandle(
        &self,
        n_std_handle: STD_HANDLE,
        h_handle: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("SetStdHandle")
    }
    fn SetStdHandleEx(
        &self,
        n_std_handle: STD_HANDLE,
        h_handle: super::super::Foundation::HANDLE,
        ph_prev_value: MutPtr<super::super::Foundation::HANDLE>,
    ) -> super::super::Foundation::BOOL {
        todo!("SetStdHandleEx")
    }
    fn WriteConsoleA(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        lp_buffer: ConstPtr<::core::ffi::c_void>,
        n_number_of_chars_to_write: u32,
        lp_number_of_chars_written: MutPtr<u32>,
        lp_reserved: MutPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!("WriteConsoleA")
    }
    fn WriteConsoleInputA(
        &self,
        h_console_input: super::super::Foundation::HANDLE,
        lp_buffer: ConstPtr<INPUT_RECORD>,
        n_length: u32,
        lp_number_of_events_written: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("WriteConsoleInputA")
    }
    fn WriteConsoleInputW(
        &self,
        h_console_input: super::super::Foundation::HANDLE,
        lp_buffer: ConstPtr<INPUT_RECORD>,
        n_length: u32,
        lp_number_of_events_written: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("WriteConsoleInputW")
    }
    fn WriteConsoleOutputA(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        lp_buffer: ConstPtr<CHAR_INFO>,
        dw_buffer_size: COORD,
        dw_buffer_coord: COORD,
        lp_write_region: MutPtr<SMALL_RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!("WriteConsoleOutputA")
    }
    fn WriteConsoleOutputAttribute(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        lp_attribute: ConstPtr<u16>,
        n_length: u32,
        dw_write_coord: COORD,
        lp_number_of_attrs_written: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("WriteConsoleOutputAttribute")
    }
    fn WriteConsoleOutputCharacterA(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        lp_character: PCSTR,
        n_length: u32,
        dw_write_coord: COORD,
        lp_number_of_chars_written: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("WriteConsoleOutputCharacterA")
    }
    fn WriteConsoleOutputCharacterW(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        lp_character: PCWSTR,
        n_length: u32,
        dw_write_coord: COORD,
        lp_number_of_chars_written: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("WriteConsoleOutputCharacterW")
    }
    fn WriteConsoleOutputW(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        lp_buffer: ConstPtr<CHAR_INFO>,
        dw_buffer_size: COORD,
        dw_buffer_coord: COORD,
        lp_write_region: MutPtr<SMALL_RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!("WriteConsoleOutputW")
    }
    fn WriteConsoleW(
        &self,
        h_console_output: super::super::Foundation::HANDLE,
        lp_buffer: ConstPtr<::core::ffi::c_void>,
        n_number_of_chars_to_write: u32,
        lp_number_of_chars_written: MutPtr<u32>,
        lp_reserved: MutPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!("WriteConsoleW")
    }
}
pub fn get_api(ctx: &crate::core::Win32Context) -> std::sync::Arc<dyn Api> {
    ctx.get::<dyn Api>()
}
