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
pub struct ACTIVATE_KEYBOARD_LAYOUT_FLAGS(pub u32);
pub const KLF_REORDER: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = ACTIVATE_KEYBOARD_LAYOUT_FLAGS(8u32);
pub const KLF_RESET: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = ACTIVATE_KEYBOARD_LAYOUT_FLAGS(1073741824u32);
pub const KLF_SETFORPROCESS: ACTIVATE_KEYBOARD_LAYOUT_FLAGS =
    ACTIVATE_KEYBOARD_LAYOUT_FLAGS(256u32);
pub const KLF_SHIFTLOCK: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = ACTIVATE_KEYBOARD_LAYOUT_FLAGS(65536u32);
pub const KLF_ACTIVATE: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = ACTIVATE_KEYBOARD_LAYOUT_FLAGS(1u32);
pub const KLF_NOTELLSHELL: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = ACTIVATE_KEYBOARD_LAYOUT_FLAGS(128u32);
pub const KLF_REPLACELANG: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = ACTIVATE_KEYBOARD_LAYOUT_FLAGS(16u32);
pub const KLF_SUBSTITUTE_OK: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = ACTIVATE_KEYBOARD_LAYOUT_FLAGS(2u32);
impl ::core::marker::Copy for ACTIVATE_KEYBOARD_LAYOUT_FLAGS {}
impl ::core::clone::Clone for ACTIVATE_KEYBOARD_LAYOUT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ACTIVATE_KEYBOARD_LAYOUT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ACTIVATE_KEYBOARD_LAYOUT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACTIVATE_KEYBOARD_LAYOUT_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ACTIVATE_KEYBOARD_LAYOUT_FLAGS {
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
pub const ACUTE: u32 = 769u32;
pub const AX_KBD_DESKTOP_TYPE: u32 = 1u32;
pub const BREVE: u32 = 774u32;
pub const CAPLOK: u32 = 1u32;
pub const CAPLOKALTGR: u32 = 4u32;
pub const CEDILLA: u32 = 807u32;
pub const CIRCUMFLEX: u32 = 770u32;
pub struct DEADKEY {
    pub dwBoth: u32,
    pub wchComposed: u16,
    pub uFlags: u16,
}
impl ::core::marker::Copy for DEADKEY {}
impl ::core::clone::Clone for DEADKEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DEADKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEADKEY")
            .field("dwBoth", &self.dwBoth)
            .field("wchComposed", &self.wchComposed)
            .field("uFlags", &self.uFlags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DEADKEY {
    fn eq(&self, other: &Self) -> bool {
        self.dwBoth == other.dwBoth
            && self.wchComposed == other.wchComposed
            && self.uFlags == other.uFlags
    }
}
impl ::core::cmp::Eq for DEADKEY {}
impl FromIntoMemory for DEADKEY {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_dwBoth = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_wchComposed = <u16 as FromIntoMemory>::from_bytes(&from[4..4 + 1]);
        let f_uFlags = <u16 as FromIntoMemory>::from_bytes(&from[6..6 + 2]);
        Self {
            dwBoth: f_dwBoth,
            wchComposed: f_wchComposed,
            uFlags: f_uFlags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.dwBoth, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.wchComposed, &mut into[4..4 + 1]);
        FromIntoMemory::into_bytes(self.uFlags, &mut into[6..6 + 2]);
    }
    fn size() -> usize {
        8
    }
}
pub const DEC_KBD_ANSI_LAYOUT_TYPE: u32 = 1u32;
pub const DEC_KBD_JIS_LAYOUT_TYPE: u32 = 2u32;
pub const DIARESIS: u32 = 776u32;
pub const DIARESIS_TONOS: u32 = 901u32;
pub const DKF_DEAD: u32 = 1u32;
pub const DONTCARE_BIT: u32 = 33554432u32;
pub const DOT_ABOVE: u32 = 775u32;
pub const DOUBLE_ACUTE: u32 = 779u32;
pub const EXTENDED_BIT: u32 = 16777216u32;
pub const FAKE_KEYSTROKE: u32 = 33554432u32;
pub const FMR_KBD_JIS_TYPE: u32 = 0u32;
pub const FMR_KBD_OASYS_TYPE: u32 = 1u32;
pub const FMV_KBD_OASYS_TYPE: u32 = 2u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GET_MOUSE_MOVE_POINTS_EX_RESOLUTION(pub u32);
pub const GMMP_USE_DISPLAY_POINTS: GET_MOUSE_MOVE_POINTS_EX_RESOLUTION =
    GET_MOUSE_MOVE_POINTS_EX_RESOLUTION(1u32);
pub const GMMP_USE_HIGH_RESOLUTION_POINTS: GET_MOUSE_MOVE_POINTS_EX_RESOLUTION =
    GET_MOUSE_MOVE_POINTS_EX_RESOLUTION(2u32);
impl ::core::marker::Copy for GET_MOUSE_MOVE_POINTS_EX_RESOLUTION {}
impl ::core::clone::Clone for GET_MOUSE_MOVE_POINTS_EX_RESOLUTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_MOUSE_MOVE_POINTS_EX_RESOLUTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_MOUSE_MOVE_POINTS_EX_RESOLUTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_MOUSE_MOVE_POINTS_EX_RESOLUTION")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for GET_MOUSE_MOVE_POINTS_EX_RESOLUTION {
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
pub const GRAVE: u32 = 768u32;
pub const GRPSELTAP: u32 = 128u32;
pub const HACEK: u32 = 780u32;
pub struct HARDWAREINPUT {
    pub uMsg: u32,
    pub wParamL: u16,
    pub wParamH: u16,
}
impl ::core::marker::Copy for HARDWAREINPUT {}
impl ::core::clone::Clone for HARDWAREINPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HARDWAREINPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HARDWAREINPUT")
            .field("uMsg", &self.uMsg)
            .field("wParamL", &self.wParamL)
            .field("wParamH", &self.wParamH)
            .finish()
    }
}
impl ::core::cmp::PartialEq for HARDWAREINPUT {
    fn eq(&self, other: &Self) -> bool {
        self.uMsg == other.uMsg && self.wParamL == other.wParamL && self.wParamH == other.wParamH
    }
}
impl ::core::cmp::Eq for HARDWAREINPUT {}
impl FromIntoMemory for HARDWAREINPUT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_uMsg = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_wParamL = <u16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_wParamH = <u16 as FromIntoMemory>::from_bytes(&from[6..6 + 2]);
        Self {
            uMsg: f_uMsg,
            wParamL: f_wParamL,
            wParamH: f_wParamH,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.uMsg, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.wParamL, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.wParamH, &mut into[6..6 + 2]);
    }
    fn size() -> usize {
        8
    }
}
pub const HOOK_ABOVE: u32 = 777u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HOT_KEY_MODIFIERS(pub u32);
pub const MOD_ALT: HOT_KEY_MODIFIERS = HOT_KEY_MODIFIERS(1u32);
pub const MOD_CONTROL: HOT_KEY_MODIFIERS = HOT_KEY_MODIFIERS(2u32);
pub const MOD_NOREPEAT: HOT_KEY_MODIFIERS = HOT_KEY_MODIFIERS(16384u32);
pub const MOD_SHIFT: HOT_KEY_MODIFIERS = HOT_KEY_MODIFIERS(4u32);
pub const MOD_WIN: HOT_KEY_MODIFIERS = HOT_KEY_MODIFIERS(8u32);
impl ::core::marker::Copy for HOT_KEY_MODIFIERS {}
impl ::core::clone::Clone for HOT_KEY_MODIFIERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HOT_KEY_MODIFIERS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HOT_KEY_MODIFIERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HOT_KEY_MODIFIERS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for HOT_KEY_MODIFIERS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for HOT_KEY_MODIFIERS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for HOT_KEY_MODIFIERS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for HOT_KEY_MODIFIERS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for HOT_KEY_MODIFIERS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for HOT_KEY_MODIFIERS {
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
pub struct INPUT {
    pub r#type: INPUT_TYPE,
    pub Anonymous: INPUT_0,
}
impl ::core::marker::Copy for INPUT {}
impl ::core::clone::Clone for INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INPUT")
            .field("type", &self.r#type)
            .field("Anonymous", &self.Anonymous)
            .finish()
    }
}
impl ::core::cmp::PartialEq for INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.Anonymous == other.Anonymous
    }
}
impl ::core::cmp::Eq for INPUT {}
impl FromIntoMemory for INPUT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_type = <INPUT_TYPE as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Anonymous = <INPUT_0 as FromIntoMemory>::from_bytes(&from[4..4 + 8]);
        Self {
            r#type: f_type,
            Anonymous: f_Anonymous,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.r#type, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[4..4 + 8]);
    }
    fn size() -> usize {
        12
    }
}
pub struct INPUT_0 {
    data: [u8; 8],
}
impl ::core::default::Default for INPUT_0 {
    fn default() -> Self {
        Self { data: [0u8; 8] }
    }
}
impl ::core::marker::Copy for INPUT_0 {}
impl ::core::clone::Clone for INPUT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INPUT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INPUT_0").field("data", &self.data).finish()
    }
}
impl ::core::cmp::PartialEq for INPUT_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for INPUT_0 {}
impl FromIntoMemory for INPUT_0 {
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
pub struct INPUT_TYPE(pub u32);
pub const INPUT_MOUSE: INPUT_TYPE = INPUT_TYPE(0u32);
pub const INPUT_KEYBOARD: INPUT_TYPE = INPUT_TYPE(1u32);
pub const INPUT_HARDWARE: INPUT_TYPE = INPUT_TYPE(2u32);
impl ::core::marker::Copy for INPUT_TYPE {}
impl ::core::clone::Clone for INPUT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INPUT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for INPUT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INPUT_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for INPUT_TYPE {
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
pub const KANALOK: u32 = 8u32;
pub const KBDALT: u32 = 4u32;
pub const KBDBASE: u32 = 0u32;
pub const KBDCTRL: u32 = 2u32;
pub const KBDGRPSELTAP: u32 = 128u32;
pub const KBDKANA: u32 = 8u32;
pub const KBDLOYA: u32 = 32u32;
pub const KBDNLS_ALPHANUM: u32 = 5u32;
pub const KBDNLS_CODEINPUT: u32 = 10u32;
pub const KBDNLS_CONV_OR_NONCONV: u32 = 15u32;
pub const KBDNLS_HELP_OR_END: u32 = 11u32;
pub const KBDNLS_HIRAGANA: u32 = 6u32;
pub const KBDNLS_HOME_OR_CLEAR: u32 = 12u32;
pub const KBDNLS_INDEX_ALT: u32 = 2u32;
pub const KBDNLS_INDEX_NORMAL: u32 = 1u32;
pub const KBDNLS_KANAEVENT: u32 = 14u32;
pub const KBDNLS_KANALOCK: u32 = 4u32;
pub const KBDNLS_KATAKANA: u32 = 7u32;
pub const KBDNLS_NOEVENT: u32 = 1u32;
pub const KBDNLS_NULL: u32 = 0u32;
pub const KBDNLS_NUMPAD: u32 = 13u32;
pub const KBDNLS_ROMAN: u32 = 9u32;
pub const KBDNLS_SBCSDBCS: u32 = 8u32;
pub const KBDNLS_SEND_BASE_VK: u32 = 2u32;
pub const KBDNLS_SEND_PARAM_VK: u32 = 3u32;
pub const KBDNLS_TYPE_NORMAL: u32 = 1u32;
pub const KBDNLS_TYPE_NULL: u32 = 0u32;
pub const KBDNLS_TYPE_TOGGLE: u32 = 2u32;
pub const KBDROYA: u32 = 16u32;
pub const KBDSHIFT: u32 = 1u32;
pub struct KBDTABLE_DESC {
    pub wszDllName: [u16; 32],
    pub dwType: u32,
    pub dwSubType: u32,
}
impl ::core::marker::Copy for KBDTABLE_DESC {}
impl ::core::clone::Clone for KBDTABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KBDTABLE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KBDTABLE_DESC")
            .field("wszDllName", &self.wszDllName)
            .field("dwType", &self.dwType)
            .field("dwSubType", &self.dwSubType)
            .finish()
    }
}
impl ::core::cmp::PartialEq for KBDTABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.wszDllName == other.wszDllName
            && self.dwType == other.dwType
            && self.dwSubType == other.dwSubType
    }
}
impl ::core::cmp::Eq for KBDTABLE_DESC {}
impl FromIntoMemory for KBDTABLE_DESC {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_wszDllName = <[u16; 32] as FromIntoMemory>::from_bytes(&from[0..0 + 32]);
        let f_dwType = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_dwSubType = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        Self {
            wszDllName: f_wszDllName,
            dwType: f_dwType,
            dwSubType: f_dwSubType,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.wszDllName, &mut into[0..0 + 32]);
        FromIntoMemory::into_bytes(self.dwType, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.dwSubType, &mut into[36..36 + 4]);
    }
    fn size() -> usize {
        40
    }
}
pub struct KBDTABLE_MULTI {
    pub nTables: u32,
    pub aKbdTables: [KBDTABLE_DESC; 8],
}
impl ::core::marker::Copy for KBDTABLE_MULTI {}
impl ::core::clone::Clone for KBDTABLE_MULTI {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KBDTABLE_MULTI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KBDTABLE_MULTI")
            .field("nTables", &self.nTables)
            .field("aKbdTables", &self.aKbdTables)
            .finish()
    }
}
impl ::core::cmp::PartialEq for KBDTABLE_MULTI {
    fn eq(&self, other: &Self) -> bool {
        self.nTables == other.nTables && self.aKbdTables == other.aKbdTables
    }
}
impl ::core::cmp::Eq for KBDTABLE_MULTI {}
impl FromIntoMemory for KBDTABLE_MULTI {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 324);
        let f_nTables = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_aKbdTables = <[KBDTABLE_DESC; 8] as FromIntoMemory>::from_bytes(&from[4..4 + 320]);
        Self {
            nTables: f_nTables,
            aKbdTables: f_aKbdTables,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 324);
        FromIntoMemory::into_bytes(self.nTables, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.aKbdTables, &mut into[4..4 + 320]);
    }
    fn size() -> usize {
        324
    }
}
pub const KBDTABLE_MULTI_MAX: u32 = 8u32;
pub const KBD_TYPE: u32 = 4u32;
pub struct KBD_TYPE_INFO {
    pub dwVersion: u32,
    pub dwType: u32,
    pub dwSubType: u32,
}
impl ::core::marker::Copy for KBD_TYPE_INFO {}
impl ::core::clone::Clone for KBD_TYPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KBD_TYPE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KBD_TYPE_INFO")
            .field("dwVersion", &self.dwVersion)
            .field("dwType", &self.dwType)
            .field("dwSubType", &self.dwSubType)
            .finish()
    }
}
impl ::core::cmp::PartialEq for KBD_TYPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion
            && self.dwType == other.dwType
            && self.dwSubType == other.dwSubType
    }
}
impl ::core::cmp::Eq for KBD_TYPE_INFO {}
impl FromIntoMemory for KBD_TYPE_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_dwVersion = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwType = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwSubType = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            dwVersion: f_dwVersion,
            dwType: f_dwType,
            dwSubType: f_dwSubType,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.dwVersion, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwType, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwSubType, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub const KBD_VERSION: u32 = 1u32;
pub struct KEYBDINPUT {
    pub wVk: VIRTUAL_KEY,
    pub wScan: u16,
    pub dwFlags: KEYBD_EVENT_FLAGS,
    pub time: u32,
    pub dwExtraInfo: PtrRepr,
}
impl ::core::marker::Copy for KEYBDINPUT {}
impl ::core::clone::Clone for KEYBDINPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KEYBDINPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KEYBDINPUT")
            .field("wVk", &self.wVk)
            .field("wScan", &self.wScan)
            .field("dwFlags", &self.dwFlags)
            .field("time", &self.time)
            .field("dwExtraInfo", &self.dwExtraInfo)
            .finish()
    }
}
impl ::core::cmp::PartialEq for KEYBDINPUT {
    fn eq(&self, other: &Self) -> bool {
        self.wVk == other.wVk
            && self.wScan == other.wScan
            && self.dwFlags == other.dwFlags
            && self.time == other.time
            && self.dwExtraInfo == other.dwExtraInfo
    }
}
impl ::core::cmp::Eq for KEYBDINPUT {}
impl FromIntoMemory for KEYBDINPUT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_wVk = <VIRTUAL_KEY as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wScan = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_dwFlags = <KEYBD_EVENT_FLAGS as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_time = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwExtraInfo = <PtrRepr as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            wVk: f_wVk,
            wScan: f_wScan,
            dwFlags: f_dwFlags,
            time: f_time,
            dwExtraInfo: f_dwExtraInfo,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.wVk, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wScan, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.time, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwExtraInfo, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct KEYBD_EVENT_FLAGS(pub u32);
pub const KEYEVENTF_EXTENDEDKEY: KEYBD_EVENT_FLAGS = KEYBD_EVENT_FLAGS(1u32);
pub const KEYEVENTF_KEYUP: KEYBD_EVENT_FLAGS = KEYBD_EVENT_FLAGS(2u32);
pub const KEYEVENTF_SCANCODE: KEYBD_EVENT_FLAGS = KEYBD_EVENT_FLAGS(8u32);
pub const KEYEVENTF_UNICODE: KEYBD_EVENT_FLAGS = KEYBD_EVENT_FLAGS(4u32);
impl ::core::marker::Copy for KEYBD_EVENT_FLAGS {}
impl ::core::clone::Clone for KEYBD_EVENT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KEYBD_EVENT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KEYBD_EVENT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KEYBD_EVENT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for KEYBD_EVENT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for KEYBD_EVENT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for KEYBD_EVENT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for KEYBD_EVENT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for KEYBD_EVENT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for KEYBD_EVENT_FLAGS {
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
pub const KEYBOARD_TYPE_GENERIC_101: u32 = 4u32;
pub const KEYBOARD_TYPE_JAPAN: u32 = 7u32;
pub const KEYBOARD_TYPE_KOREA: u32 = 8u32;
pub const KEYBOARD_TYPE_UNKNOWN: u32 = 81u32;
pub const KLLF_ALTGR: u32 = 1u32;
pub const KLLF_GLOBAL_ATTRS: u32 = 2u32;
pub const KLLF_LRM_RLM: u32 = 4u32;
pub const KLLF_SHIFTLOCK: u32 = 2u32;
pub struct LASTINPUTINFO {
    pub cbSize: u32,
    pub dwTime: u32,
}
impl ::core::marker::Copy for LASTINPUTINFO {}
impl ::core::clone::Clone for LASTINPUTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LASTINPUTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LASTINPUTINFO")
            .field("cbSize", &self.cbSize)
            .field("dwTime", &self.dwTime)
            .finish()
    }
}
impl ::core::cmp::PartialEq for LASTINPUTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwTime == other.dwTime
    }
}
impl ::core::cmp::Eq for LASTINPUTINFO {}
impl FromIntoMemory for LASTINPUTINFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_cbSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwTime = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            cbSize: f_cbSize,
            dwTime: f_dwTime,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.cbSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwTime, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct LIGATURE1 {
    pub VirtualKey: u8,
    pub ModificationNumber: u16,
    pub wch: [u16; 1],
}
impl ::core::marker::Copy for LIGATURE1 {}
impl ::core::clone::Clone for LIGATURE1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LIGATURE1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LIGATURE1")
            .field("VirtualKey", &self.VirtualKey)
            .field("ModificationNumber", &self.ModificationNumber)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::core::cmp::PartialEq for LIGATURE1 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.ModificationNumber == other.ModificationNumber
            && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for LIGATURE1 {}
impl FromIntoMemory for LIGATURE1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 6);
        let f_VirtualKey = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_ModificationNumber = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_wch = <[u16; 1] as FromIntoMemory>::from_bytes(&from[4..4 + 1]);
        Self {
            VirtualKey: f_VirtualKey,
            ModificationNumber: f_ModificationNumber,
            wch: f_wch,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 6);
        FromIntoMemory::into_bytes(self.VirtualKey, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.ModificationNumber, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.wch, &mut into[4..4 + 1]);
    }
    fn size() -> usize {
        6
    }
}
pub struct LIGATURE2 {
    pub VirtualKey: u8,
    pub ModificationNumber: u16,
    pub wch: [u16; 2],
}
impl ::core::marker::Copy for LIGATURE2 {}
impl ::core::clone::Clone for LIGATURE2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LIGATURE2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LIGATURE2")
            .field("VirtualKey", &self.VirtualKey)
            .field("ModificationNumber", &self.ModificationNumber)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::core::cmp::PartialEq for LIGATURE2 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.ModificationNumber == other.ModificationNumber
            && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for LIGATURE2 {}
impl FromIntoMemory for LIGATURE2 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 6);
        let f_VirtualKey = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_ModificationNumber = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_wch = <[u16; 2] as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        Self {
            VirtualKey: f_VirtualKey,
            ModificationNumber: f_ModificationNumber,
            wch: f_wch,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 6);
        FromIntoMemory::into_bytes(self.VirtualKey, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.ModificationNumber, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.wch, &mut into[4..4 + 2]);
    }
    fn size() -> usize {
        6
    }
}
pub struct LIGATURE3 {
    pub VirtualKey: u8,
    pub ModificationNumber: u16,
    pub wch: [u16; 3],
}
impl ::core::marker::Copy for LIGATURE3 {}
impl ::core::clone::Clone for LIGATURE3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LIGATURE3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LIGATURE3")
            .field("VirtualKey", &self.VirtualKey)
            .field("ModificationNumber", &self.ModificationNumber)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::core::cmp::PartialEq for LIGATURE3 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.ModificationNumber == other.ModificationNumber
            && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for LIGATURE3 {}
impl FromIntoMemory for LIGATURE3 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_VirtualKey = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_ModificationNumber = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_wch = <[u16; 3] as FromIntoMemory>::from_bytes(&from[4..4 + 3]);
        Self {
            VirtualKey: f_VirtualKey,
            ModificationNumber: f_ModificationNumber,
            wch: f_wch,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.VirtualKey, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.ModificationNumber, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.wch, &mut into[4..4 + 3]);
    }
    fn size() -> usize {
        8
    }
}
pub struct LIGATURE4 {
    pub VirtualKey: u8,
    pub ModificationNumber: u16,
    pub wch: [u16; 4],
}
impl ::core::marker::Copy for LIGATURE4 {}
impl ::core::clone::Clone for LIGATURE4 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LIGATURE4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LIGATURE4")
            .field("VirtualKey", &self.VirtualKey)
            .field("ModificationNumber", &self.ModificationNumber)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::core::cmp::PartialEq for LIGATURE4 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.ModificationNumber == other.ModificationNumber
            && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for LIGATURE4 {}
impl FromIntoMemory for LIGATURE4 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_VirtualKey = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_ModificationNumber = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_wch = <[u16; 4] as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            VirtualKey: f_VirtualKey,
            ModificationNumber: f_ModificationNumber,
            wch: f_wch,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.VirtualKey, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.ModificationNumber, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.wch, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct LIGATURE5 {
    pub VirtualKey: u8,
    pub ModificationNumber: u16,
    pub wch: [u16; 5],
}
impl ::core::marker::Copy for LIGATURE5 {}
impl ::core::clone::Clone for LIGATURE5 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LIGATURE5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LIGATURE5")
            .field("VirtualKey", &self.VirtualKey)
            .field("ModificationNumber", &self.ModificationNumber)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::core::cmp::PartialEq for LIGATURE5 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.ModificationNumber == other.ModificationNumber
            && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for LIGATURE5 {}
impl FromIntoMemory for LIGATURE5 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 10);
        let f_VirtualKey = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_ModificationNumber = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_wch = <[u16; 5] as FromIntoMemory>::from_bytes(&from[4..4 + 5]);
        Self {
            VirtualKey: f_VirtualKey,
            ModificationNumber: f_ModificationNumber,
            wch: f_wch,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 10);
        FromIntoMemory::into_bytes(self.VirtualKey, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.ModificationNumber, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.wch, &mut into[4..4 + 5]);
    }
    fn size() -> usize {
        10
    }
}
pub const MACRON: u32 = 772u32;
pub const MICROSOFT_KBD_001_TYPE: u32 = 4u32;
pub const MICROSOFT_KBD_002_TYPE: u32 = 3u32;
pub const MICROSOFT_KBD_101A_TYPE: u32 = 0u32;
pub const MICROSOFT_KBD_101B_TYPE: u32 = 4u32;
pub const MICROSOFT_KBD_101C_TYPE: u32 = 5u32;
pub const MICROSOFT_KBD_101_TYPE: u32 = 0u32;
pub const MICROSOFT_KBD_103_TYPE: u32 = 6u32;
pub const MICROSOFT_KBD_106_TYPE: u32 = 2u32;
pub const MICROSOFT_KBD_AX_TYPE: u32 = 1u32;
pub const MICROSOFT_KBD_FUNC: u32 = 12u32;
pub struct MODIFIERS {
    pub pVkToBit: MutPtr<VK_TO_BIT>,
    pub wMaxModBits: u16,
    pub ModNumber: [u8; 1],
}
impl ::core::marker::Copy for MODIFIERS {}
impl ::core::clone::Clone for MODIFIERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MODIFIERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MODIFIERS")
            .field("pVkToBit", &self.pVkToBit)
            .field("wMaxModBits", &self.wMaxModBits)
            .field("ModNumber", &self.ModNumber)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MODIFIERS {
    fn eq(&self, other: &Self) -> bool {
        self.pVkToBit == other.pVkToBit
            && self.wMaxModBits == other.wMaxModBits
            && self.ModNumber == other.ModNumber
    }
}
impl ::core::cmp::Eq for MODIFIERS {}
impl FromIntoMemory for MODIFIERS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_pVkToBit = <MutPtr<VK_TO_BIT> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_wMaxModBits = <u16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_ModNumber = <[u8; 1] as FromIntoMemory>::from_bytes(&from[6..6 + 1]);
        Self {
            pVkToBit: f_pVkToBit,
            wMaxModBits: f_wMaxModBits,
            ModNumber: f_ModNumber,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.pVkToBit, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.wMaxModBits, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.ModNumber, &mut into[6..6 + 1]);
    }
    fn size() -> usize {
        8
    }
}
pub struct MOUSEINPUT {
    pub dx: i32,
    pub dy: i32,
    pub mouseData: u32,
    pub dwFlags: MOUSE_EVENT_FLAGS,
    pub time: u32,
    pub dwExtraInfo: PtrRepr,
}
impl ::core::marker::Copy for MOUSEINPUT {}
impl ::core::clone::Clone for MOUSEINPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MOUSEINPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MOUSEINPUT")
            .field("dx", &self.dx)
            .field("dy", &self.dy)
            .field("mouseData", &self.mouseData)
            .field("dwFlags", &self.dwFlags)
            .field("time", &self.time)
            .field("dwExtraInfo", &self.dwExtraInfo)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MOUSEINPUT {
    fn eq(&self, other: &Self) -> bool {
        self.dx == other.dx
            && self.dy == other.dy
            && self.mouseData == other.mouseData
            && self.dwFlags == other.dwFlags
            && self.time == other.time
            && self.dwExtraInfo == other.dwExtraInfo
    }
}
impl ::core::cmp::Eq for MOUSEINPUT {}
impl FromIntoMemory for MOUSEINPUT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_dx = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dy = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_mouseData = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwFlags = <MOUSE_EVENT_FLAGS as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_time = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwExtraInfo = <PtrRepr as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            dx: f_dx,
            dy: f_dy,
            mouseData: f_mouseData,
            dwFlags: f_dwFlags,
            time: f_time,
            dwExtraInfo: f_dwExtraInfo,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.dx, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dy, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.mouseData, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.time, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwExtraInfo, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct MOUSEMOVEPOINT {
    pub x: i32,
    pub y: i32,
    pub time: u32,
    pub dwExtraInfo: PtrRepr,
}
impl ::core::marker::Copy for MOUSEMOVEPOINT {}
impl ::core::clone::Clone for MOUSEMOVEPOINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MOUSEMOVEPOINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MOUSEMOVEPOINT")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("time", &self.time)
            .field("dwExtraInfo", &self.dwExtraInfo)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MOUSEMOVEPOINT {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
            && self.y == other.y
            && self.time == other.time
            && self.dwExtraInfo == other.dwExtraInfo
    }
}
impl ::core::cmp::Eq for MOUSEMOVEPOINT {}
impl FromIntoMemory for MOUSEMOVEPOINT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_x = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_y = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_time = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwExtraInfo = <PtrRepr as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            x: f_x,
            y: f_y,
            time: f_time,
            dwExtraInfo: f_dwExtraInfo,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.x, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.y, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.time, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwExtraInfo, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MOUSE_EVENT_FLAGS(pub u32);
pub const MOUSEEVENTF_ABSOLUTE: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(32768u32);
pub const MOUSEEVENTF_LEFTDOWN: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(2u32);
pub const MOUSEEVENTF_LEFTUP: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(4u32);
pub const MOUSEEVENTF_MIDDLEDOWN: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(32u32);
pub const MOUSEEVENTF_MIDDLEUP: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(64u32);
pub const MOUSEEVENTF_MOVE: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(1u32);
pub const MOUSEEVENTF_RIGHTDOWN: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(8u32);
pub const MOUSEEVENTF_RIGHTUP: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(16u32);
pub const MOUSEEVENTF_WHEEL: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(2048u32);
pub const MOUSEEVENTF_XDOWN: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(128u32);
pub const MOUSEEVENTF_XUP: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(256u32);
pub const MOUSEEVENTF_HWHEEL: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(4096u32);
pub const MOUSEEVENTF_MOVE_NOCOALESCE: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(8192u32);
pub const MOUSEEVENTF_VIRTUALDESK: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(16384u32);
impl ::core::marker::Copy for MOUSE_EVENT_FLAGS {}
impl ::core::clone::Clone for MOUSE_EVENT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MOUSE_EVENT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MOUSE_EVENT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MOUSE_EVENT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MOUSE_EVENT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MOUSE_EVENT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MOUSE_EVENT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MOUSE_EVENT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MOUSE_EVENT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for MOUSE_EVENT_FLAGS {
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
pub const NEC_KBD_106_TYPE: u32 = 5u32;
pub const NEC_KBD_H_MODE_TYPE: u32 = 3u32;
pub const NEC_KBD_LAPTOP_TYPE: u32 = 4u32;
pub const NEC_KBD_NORMAL_TYPE: u32 = 1u32;
pub const NEC_KBD_N_MODE_TYPE: u32 = 2u32;
pub const NLSKBD_INFO_ACCESSIBILITY_KEYMAP: u32 = 2u32;
pub const NLSKBD_INFO_EMURATE_101_KEYBOARD: u32 = 16u32;
pub const NLSKBD_INFO_EMURATE_106_KEYBOARD: u32 = 32u32;
pub const NLSKBD_INFO_SEND_IME_NOTIFICATION: u32 = 1u32;
pub const NLSKBD_OEM_AX: u32 = 1u32;
pub const NLSKBD_OEM_DEC: u32 = 24u32;
pub const NLSKBD_OEM_EPSON: u32 = 4u32;
pub const NLSKBD_OEM_FUJITSU: u32 = 5u32;
pub const NLSKBD_OEM_IBM: u32 = 7u32;
pub const NLSKBD_OEM_MATSUSHITA: u32 = 10u32;
pub const NLSKBD_OEM_MICROSOFT: u32 = 0u32;
pub const NLSKBD_OEM_NEC: u32 = 13u32;
pub const NLSKBD_OEM_TOSHIBA: u32 = 18u32;
pub const OGONEK: u32 = 808u32;
pub const OVERSCORE: u32 = 773u32;
pub const RING: u32 = 778u32;
pub const SCANCODE_ALT: u32 = 56u32;
pub const SCANCODE_CTRL: u32 = 29u32;
pub const SCANCODE_LSHIFT: u32 = 42u32;
pub const SCANCODE_LWIN: u32 = 91u32;
pub const SCANCODE_NUMPAD_FIRST: u32 = 71u32;
pub const SCANCODE_NUMPAD_LAST: u32 = 82u32;
pub const SCANCODE_RSHIFT: u32 = 54u32;
pub const SCANCODE_RWIN: u32 = 92u32;
pub const SCANCODE_THAI_LAYOUT_TOGGLE: u32 = 41u32;
pub const SGCAPS: u32 = 2u32;
pub const SHFT_INVALID: u32 = 15u32;
pub const TILDE: u32 = 771u32;
pub const TONOS: u32 = 900u32;
pub const TOSHIBA_KBD_DESKTOP_TYPE: u32 = 13u32;
pub const TOSHIBA_KBD_LAPTOP_TYPE: u32 = 15u32;
pub struct TRACKMOUSEEVENT {
    pub cbSize: u32,
    pub dwFlags: TRACKMOUSEEVENT_FLAGS,
    pub hwndTrack: super::super::super::Foundation::HWND,
    pub dwHoverTime: u32,
}
impl ::core::marker::Copy for TRACKMOUSEEVENT {}
impl ::core::clone::Clone for TRACKMOUSEEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRACKMOUSEEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRACKMOUSEEVENT")
            .field("cbSize", &self.cbSize)
            .field("dwFlags", &self.dwFlags)
            .field("hwndTrack", &self.hwndTrack)
            .field("dwHoverTime", &self.dwHoverTime)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TRACKMOUSEEVENT {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.dwFlags == other.dwFlags
            && self.hwndTrack == other.hwndTrack
            && self.dwHoverTime == other.dwHoverTime
    }
}
impl ::core::cmp::Eq for TRACKMOUSEEVENT {}
impl FromIntoMemory for TRACKMOUSEEVENT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_cbSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFlags = <TRACKMOUSEEVENT_FLAGS as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_hwndTrack =
            <super::super::super::Foundation::HWND as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwHoverTime = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            cbSize: f_cbSize,
            dwFlags: f_dwFlags,
            hwndTrack: f_hwndTrack,
            dwHoverTime: f_dwHoverTime,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.cbSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.hwndTrack, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwHoverTime, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TRACKMOUSEEVENT_FLAGS(pub u32);
pub const TME_CANCEL: TRACKMOUSEEVENT_FLAGS = TRACKMOUSEEVENT_FLAGS(2147483648u32);
pub const TME_HOVER: TRACKMOUSEEVENT_FLAGS = TRACKMOUSEEVENT_FLAGS(1u32);
pub const TME_LEAVE: TRACKMOUSEEVENT_FLAGS = TRACKMOUSEEVENT_FLAGS(2u32);
pub const TME_NONCLIENT: TRACKMOUSEEVENT_FLAGS = TRACKMOUSEEVENT_FLAGS(16u32);
pub const TME_QUERY: TRACKMOUSEEVENT_FLAGS = TRACKMOUSEEVENT_FLAGS(1073741824u32);
impl ::core::marker::Copy for TRACKMOUSEEVENT_FLAGS {}
impl ::core::clone::Clone for TRACKMOUSEEVENT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TRACKMOUSEEVENT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TRACKMOUSEEVENT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRACKMOUSEEVENT_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for TRACKMOUSEEVENT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TRACKMOUSEEVENT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TRACKMOUSEEVENT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TRACKMOUSEEVENT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TRACKMOUSEEVENT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for TRACKMOUSEEVENT_FLAGS {
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
pub const UMLAUT: u32 = 776u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VIRTUAL_KEY(pub u16);
pub const VK_0: VIRTUAL_KEY = VIRTUAL_KEY(48u16);
pub const VK_1: VIRTUAL_KEY = VIRTUAL_KEY(49u16);
pub const VK_2: VIRTUAL_KEY = VIRTUAL_KEY(50u16);
pub const VK_3: VIRTUAL_KEY = VIRTUAL_KEY(51u16);
pub const VK_4: VIRTUAL_KEY = VIRTUAL_KEY(52u16);
pub const VK_5: VIRTUAL_KEY = VIRTUAL_KEY(53u16);
pub const VK_6: VIRTUAL_KEY = VIRTUAL_KEY(54u16);
pub const VK_7: VIRTUAL_KEY = VIRTUAL_KEY(55u16);
pub const VK_8: VIRTUAL_KEY = VIRTUAL_KEY(56u16);
pub const VK_9: VIRTUAL_KEY = VIRTUAL_KEY(57u16);
pub const VK_A: VIRTUAL_KEY = VIRTUAL_KEY(65u16);
pub const VK_B: VIRTUAL_KEY = VIRTUAL_KEY(66u16);
pub const VK_C: VIRTUAL_KEY = VIRTUAL_KEY(67u16);
pub const VK_D: VIRTUAL_KEY = VIRTUAL_KEY(68u16);
pub const VK_E: VIRTUAL_KEY = VIRTUAL_KEY(69u16);
pub const VK_F: VIRTUAL_KEY = VIRTUAL_KEY(70u16);
pub const VK_G: VIRTUAL_KEY = VIRTUAL_KEY(71u16);
pub const VK_H: VIRTUAL_KEY = VIRTUAL_KEY(72u16);
pub const VK_I: VIRTUAL_KEY = VIRTUAL_KEY(73u16);
pub const VK_J: VIRTUAL_KEY = VIRTUAL_KEY(74u16);
pub const VK_K: VIRTUAL_KEY = VIRTUAL_KEY(75u16);
pub const VK_L: VIRTUAL_KEY = VIRTUAL_KEY(76u16);
pub const VK_M: VIRTUAL_KEY = VIRTUAL_KEY(77u16);
pub const VK_N: VIRTUAL_KEY = VIRTUAL_KEY(78u16);
pub const VK_O: VIRTUAL_KEY = VIRTUAL_KEY(79u16);
pub const VK_P: VIRTUAL_KEY = VIRTUAL_KEY(80u16);
pub const VK_Q: VIRTUAL_KEY = VIRTUAL_KEY(81u16);
pub const VK_R: VIRTUAL_KEY = VIRTUAL_KEY(82u16);
pub const VK_S: VIRTUAL_KEY = VIRTUAL_KEY(83u16);
pub const VK_T: VIRTUAL_KEY = VIRTUAL_KEY(84u16);
pub const VK_U: VIRTUAL_KEY = VIRTUAL_KEY(85u16);
pub const VK_V: VIRTUAL_KEY = VIRTUAL_KEY(86u16);
pub const VK_W: VIRTUAL_KEY = VIRTUAL_KEY(87u16);
pub const VK_X: VIRTUAL_KEY = VIRTUAL_KEY(88u16);
pub const VK_Y: VIRTUAL_KEY = VIRTUAL_KEY(89u16);
pub const VK_Z: VIRTUAL_KEY = VIRTUAL_KEY(90u16);
pub const VK_LBUTTON: VIRTUAL_KEY = VIRTUAL_KEY(1u16);
pub const VK_RBUTTON: VIRTUAL_KEY = VIRTUAL_KEY(2u16);
pub const VK_CANCEL: VIRTUAL_KEY = VIRTUAL_KEY(3u16);
pub const VK_MBUTTON: VIRTUAL_KEY = VIRTUAL_KEY(4u16);
pub const VK_XBUTTON1: VIRTUAL_KEY = VIRTUAL_KEY(5u16);
pub const VK_XBUTTON2: VIRTUAL_KEY = VIRTUAL_KEY(6u16);
pub const VK_BACK: VIRTUAL_KEY = VIRTUAL_KEY(8u16);
pub const VK_TAB: VIRTUAL_KEY = VIRTUAL_KEY(9u16);
pub const VK_CLEAR: VIRTUAL_KEY = VIRTUAL_KEY(12u16);
pub const VK_RETURN: VIRTUAL_KEY = VIRTUAL_KEY(13u16);
pub const VK_SHIFT: VIRTUAL_KEY = VIRTUAL_KEY(16u16);
pub const VK_CONTROL: VIRTUAL_KEY = VIRTUAL_KEY(17u16);
pub const VK_MENU: VIRTUAL_KEY = VIRTUAL_KEY(18u16);
pub const VK_PAUSE: VIRTUAL_KEY = VIRTUAL_KEY(19u16);
pub const VK_CAPITAL: VIRTUAL_KEY = VIRTUAL_KEY(20u16);
pub const VK_KANA: VIRTUAL_KEY = VIRTUAL_KEY(21u16);
pub const VK_HANGEUL: VIRTUAL_KEY = VIRTUAL_KEY(21u16);
pub const VK_HANGUL: VIRTUAL_KEY = VIRTUAL_KEY(21u16);
pub const VK_IME_ON: VIRTUAL_KEY = VIRTUAL_KEY(22u16);
pub const VK_JUNJA: VIRTUAL_KEY = VIRTUAL_KEY(23u16);
pub const VK_FINAL: VIRTUAL_KEY = VIRTUAL_KEY(24u16);
pub const VK_HANJA: VIRTUAL_KEY = VIRTUAL_KEY(25u16);
pub const VK_KANJI: VIRTUAL_KEY = VIRTUAL_KEY(25u16);
pub const VK_IME_OFF: VIRTUAL_KEY = VIRTUAL_KEY(26u16);
pub const VK_ESCAPE: VIRTUAL_KEY = VIRTUAL_KEY(27u16);
pub const VK_CONVERT: VIRTUAL_KEY = VIRTUAL_KEY(28u16);
pub const VK_NONCONVERT: VIRTUAL_KEY = VIRTUAL_KEY(29u16);
pub const VK_ACCEPT: VIRTUAL_KEY = VIRTUAL_KEY(30u16);
pub const VK_MODECHANGE: VIRTUAL_KEY = VIRTUAL_KEY(31u16);
pub const VK_SPACE: VIRTUAL_KEY = VIRTUAL_KEY(32u16);
pub const VK_PRIOR: VIRTUAL_KEY = VIRTUAL_KEY(33u16);
pub const VK_NEXT: VIRTUAL_KEY = VIRTUAL_KEY(34u16);
pub const VK_END: VIRTUAL_KEY = VIRTUAL_KEY(35u16);
pub const VK_HOME: VIRTUAL_KEY = VIRTUAL_KEY(36u16);
pub const VK_LEFT: VIRTUAL_KEY = VIRTUAL_KEY(37u16);
pub const VK_UP: VIRTUAL_KEY = VIRTUAL_KEY(38u16);
pub const VK_RIGHT: VIRTUAL_KEY = VIRTUAL_KEY(39u16);
pub const VK_DOWN: VIRTUAL_KEY = VIRTUAL_KEY(40u16);
pub const VK_SELECT: VIRTUAL_KEY = VIRTUAL_KEY(41u16);
pub const VK_PRINT: VIRTUAL_KEY = VIRTUAL_KEY(42u16);
pub const VK_EXECUTE: VIRTUAL_KEY = VIRTUAL_KEY(43u16);
pub const VK_SNAPSHOT: VIRTUAL_KEY = VIRTUAL_KEY(44u16);
pub const VK_INSERT: VIRTUAL_KEY = VIRTUAL_KEY(45u16);
pub const VK_DELETE: VIRTUAL_KEY = VIRTUAL_KEY(46u16);
pub const VK_HELP: VIRTUAL_KEY = VIRTUAL_KEY(47u16);
pub const VK_LWIN: VIRTUAL_KEY = VIRTUAL_KEY(91u16);
pub const VK_RWIN: VIRTUAL_KEY = VIRTUAL_KEY(92u16);
pub const VK_APPS: VIRTUAL_KEY = VIRTUAL_KEY(93u16);
pub const VK_SLEEP: VIRTUAL_KEY = VIRTUAL_KEY(95u16);
pub const VK_NUMPAD0: VIRTUAL_KEY = VIRTUAL_KEY(96u16);
pub const VK_NUMPAD1: VIRTUAL_KEY = VIRTUAL_KEY(97u16);
pub const VK_NUMPAD2: VIRTUAL_KEY = VIRTUAL_KEY(98u16);
pub const VK_NUMPAD3: VIRTUAL_KEY = VIRTUAL_KEY(99u16);
pub const VK_NUMPAD4: VIRTUAL_KEY = VIRTUAL_KEY(100u16);
pub const VK_NUMPAD5: VIRTUAL_KEY = VIRTUAL_KEY(101u16);
pub const VK_NUMPAD6: VIRTUAL_KEY = VIRTUAL_KEY(102u16);
pub const VK_NUMPAD7: VIRTUAL_KEY = VIRTUAL_KEY(103u16);
pub const VK_NUMPAD8: VIRTUAL_KEY = VIRTUAL_KEY(104u16);
pub const VK_NUMPAD9: VIRTUAL_KEY = VIRTUAL_KEY(105u16);
pub const VK_MULTIPLY: VIRTUAL_KEY = VIRTUAL_KEY(106u16);
pub const VK_ADD: VIRTUAL_KEY = VIRTUAL_KEY(107u16);
pub const VK_SEPARATOR: VIRTUAL_KEY = VIRTUAL_KEY(108u16);
pub const VK_SUBTRACT: VIRTUAL_KEY = VIRTUAL_KEY(109u16);
pub const VK_DECIMAL: VIRTUAL_KEY = VIRTUAL_KEY(110u16);
pub const VK_DIVIDE: VIRTUAL_KEY = VIRTUAL_KEY(111u16);
pub const VK_F1: VIRTUAL_KEY = VIRTUAL_KEY(112u16);
pub const VK_F2: VIRTUAL_KEY = VIRTUAL_KEY(113u16);
pub const VK_F3: VIRTUAL_KEY = VIRTUAL_KEY(114u16);
pub const VK_F4: VIRTUAL_KEY = VIRTUAL_KEY(115u16);
pub const VK_F5: VIRTUAL_KEY = VIRTUAL_KEY(116u16);
pub const VK_F6: VIRTUAL_KEY = VIRTUAL_KEY(117u16);
pub const VK_F7: VIRTUAL_KEY = VIRTUAL_KEY(118u16);
pub const VK_F8: VIRTUAL_KEY = VIRTUAL_KEY(119u16);
pub const VK_F9: VIRTUAL_KEY = VIRTUAL_KEY(120u16);
pub const VK_F10: VIRTUAL_KEY = VIRTUAL_KEY(121u16);
pub const VK_F11: VIRTUAL_KEY = VIRTUAL_KEY(122u16);
pub const VK_F12: VIRTUAL_KEY = VIRTUAL_KEY(123u16);
pub const VK_F13: VIRTUAL_KEY = VIRTUAL_KEY(124u16);
pub const VK_F14: VIRTUAL_KEY = VIRTUAL_KEY(125u16);
pub const VK_F15: VIRTUAL_KEY = VIRTUAL_KEY(126u16);
pub const VK_F16: VIRTUAL_KEY = VIRTUAL_KEY(127u16);
pub const VK_F17: VIRTUAL_KEY = VIRTUAL_KEY(128u16);
pub const VK_F18: VIRTUAL_KEY = VIRTUAL_KEY(129u16);
pub const VK_F19: VIRTUAL_KEY = VIRTUAL_KEY(130u16);
pub const VK_F20: VIRTUAL_KEY = VIRTUAL_KEY(131u16);
pub const VK_F21: VIRTUAL_KEY = VIRTUAL_KEY(132u16);
pub const VK_F22: VIRTUAL_KEY = VIRTUAL_KEY(133u16);
pub const VK_F23: VIRTUAL_KEY = VIRTUAL_KEY(134u16);
pub const VK_F24: VIRTUAL_KEY = VIRTUAL_KEY(135u16);
pub const VK_NAVIGATION_VIEW: VIRTUAL_KEY = VIRTUAL_KEY(136u16);
pub const VK_NAVIGATION_MENU: VIRTUAL_KEY = VIRTUAL_KEY(137u16);
pub const VK_NAVIGATION_UP: VIRTUAL_KEY = VIRTUAL_KEY(138u16);
pub const VK_NAVIGATION_DOWN: VIRTUAL_KEY = VIRTUAL_KEY(139u16);
pub const VK_NAVIGATION_LEFT: VIRTUAL_KEY = VIRTUAL_KEY(140u16);
pub const VK_NAVIGATION_RIGHT: VIRTUAL_KEY = VIRTUAL_KEY(141u16);
pub const VK_NAVIGATION_ACCEPT: VIRTUAL_KEY = VIRTUAL_KEY(142u16);
pub const VK_NAVIGATION_CANCEL: VIRTUAL_KEY = VIRTUAL_KEY(143u16);
pub const VK_NUMLOCK: VIRTUAL_KEY = VIRTUAL_KEY(144u16);
pub const VK_SCROLL: VIRTUAL_KEY = VIRTUAL_KEY(145u16);
pub const VK_OEM_NEC_EQUAL: VIRTUAL_KEY = VIRTUAL_KEY(146u16);
pub const VK_OEM_FJ_JISHO: VIRTUAL_KEY = VIRTUAL_KEY(146u16);
pub const VK_OEM_FJ_MASSHOU: VIRTUAL_KEY = VIRTUAL_KEY(147u16);
pub const VK_OEM_FJ_TOUROKU: VIRTUAL_KEY = VIRTUAL_KEY(148u16);
pub const VK_OEM_FJ_LOYA: VIRTUAL_KEY = VIRTUAL_KEY(149u16);
pub const VK_OEM_FJ_ROYA: VIRTUAL_KEY = VIRTUAL_KEY(150u16);
pub const VK_LSHIFT: VIRTUAL_KEY = VIRTUAL_KEY(160u16);
pub const VK_RSHIFT: VIRTUAL_KEY = VIRTUAL_KEY(161u16);
pub const VK_LCONTROL: VIRTUAL_KEY = VIRTUAL_KEY(162u16);
pub const VK_RCONTROL: VIRTUAL_KEY = VIRTUAL_KEY(163u16);
pub const VK_LMENU: VIRTUAL_KEY = VIRTUAL_KEY(164u16);
pub const VK_RMENU: VIRTUAL_KEY = VIRTUAL_KEY(165u16);
pub const VK_BROWSER_BACK: VIRTUAL_KEY = VIRTUAL_KEY(166u16);
pub const VK_BROWSER_FORWARD: VIRTUAL_KEY = VIRTUAL_KEY(167u16);
pub const VK_BROWSER_REFRESH: VIRTUAL_KEY = VIRTUAL_KEY(168u16);
pub const VK_BROWSER_STOP: VIRTUAL_KEY = VIRTUAL_KEY(169u16);
pub const VK_BROWSER_SEARCH: VIRTUAL_KEY = VIRTUAL_KEY(170u16);
pub const VK_BROWSER_FAVORITES: VIRTUAL_KEY = VIRTUAL_KEY(171u16);
pub const VK_BROWSER_HOME: VIRTUAL_KEY = VIRTUAL_KEY(172u16);
pub const VK_VOLUME_MUTE: VIRTUAL_KEY = VIRTUAL_KEY(173u16);
pub const VK_VOLUME_DOWN: VIRTUAL_KEY = VIRTUAL_KEY(174u16);
pub const VK_VOLUME_UP: VIRTUAL_KEY = VIRTUAL_KEY(175u16);
pub const VK_MEDIA_NEXT_TRACK: VIRTUAL_KEY = VIRTUAL_KEY(176u16);
pub const VK_MEDIA_PREV_TRACK: VIRTUAL_KEY = VIRTUAL_KEY(177u16);
pub const VK_MEDIA_STOP: VIRTUAL_KEY = VIRTUAL_KEY(178u16);
pub const VK_MEDIA_PLAY_PAUSE: VIRTUAL_KEY = VIRTUAL_KEY(179u16);
pub const VK_LAUNCH_MAIL: VIRTUAL_KEY = VIRTUAL_KEY(180u16);
pub const VK_LAUNCH_MEDIA_SELECT: VIRTUAL_KEY = VIRTUAL_KEY(181u16);
pub const VK_LAUNCH_APP1: VIRTUAL_KEY = VIRTUAL_KEY(182u16);
pub const VK_LAUNCH_APP2: VIRTUAL_KEY = VIRTUAL_KEY(183u16);
pub const VK_OEM_1: VIRTUAL_KEY = VIRTUAL_KEY(186u16);
pub const VK_OEM_PLUS: VIRTUAL_KEY = VIRTUAL_KEY(187u16);
pub const VK_OEM_COMMA: VIRTUAL_KEY = VIRTUAL_KEY(188u16);
pub const VK_OEM_MINUS: VIRTUAL_KEY = VIRTUAL_KEY(189u16);
pub const VK_OEM_PERIOD: VIRTUAL_KEY = VIRTUAL_KEY(190u16);
pub const VK_OEM_2: VIRTUAL_KEY = VIRTUAL_KEY(191u16);
pub const VK_OEM_3: VIRTUAL_KEY = VIRTUAL_KEY(192u16);
pub const VK_GAMEPAD_A: VIRTUAL_KEY = VIRTUAL_KEY(195u16);
pub const VK_GAMEPAD_B: VIRTUAL_KEY = VIRTUAL_KEY(196u16);
pub const VK_GAMEPAD_X: VIRTUAL_KEY = VIRTUAL_KEY(197u16);
pub const VK_GAMEPAD_Y: VIRTUAL_KEY = VIRTUAL_KEY(198u16);
pub const VK_GAMEPAD_RIGHT_SHOULDER: VIRTUAL_KEY = VIRTUAL_KEY(199u16);
pub const VK_GAMEPAD_LEFT_SHOULDER: VIRTUAL_KEY = VIRTUAL_KEY(200u16);
pub const VK_GAMEPAD_LEFT_TRIGGER: VIRTUAL_KEY = VIRTUAL_KEY(201u16);
pub const VK_GAMEPAD_RIGHT_TRIGGER: VIRTUAL_KEY = VIRTUAL_KEY(202u16);
pub const VK_GAMEPAD_DPAD_UP: VIRTUAL_KEY = VIRTUAL_KEY(203u16);
pub const VK_GAMEPAD_DPAD_DOWN: VIRTUAL_KEY = VIRTUAL_KEY(204u16);
pub const VK_GAMEPAD_DPAD_LEFT: VIRTUAL_KEY = VIRTUAL_KEY(205u16);
pub const VK_GAMEPAD_DPAD_RIGHT: VIRTUAL_KEY = VIRTUAL_KEY(206u16);
pub const VK_GAMEPAD_MENU: VIRTUAL_KEY = VIRTUAL_KEY(207u16);
pub const VK_GAMEPAD_VIEW: VIRTUAL_KEY = VIRTUAL_KEY(208u16);
pub const VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON: VIRTUAL_KEY = VIRTUAL_KEY(209u16);
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON: VIRTUAL_KEY = VIRTUAL_KEY(210u16);
pub const VK_GAMEPAD_LEFT_THUMBSTICK_UP: VIRTUAL_KEY = VIRTUAL_KEY(211u16);
pub const VK_GAMEPAD_LEFT_THUMBSTICK_DOWN: VIRTUAL_KEY = VIRTUAL_KEY(212u16);
pub const VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT: VIRTUAL_KEY = VIRTUAL_KEY(213u16);
pub const VK_GAMEPAD_LEFT_THUMBSTICK_LEFT: VIRTUAL_KEY = VIRTUAL_KEY(214u16);
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_UP: VIRTUAL_KEY = VIRTUAL_KEY(215u16);
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN: VIRTUAL_KEY = VIRTUAL_KEY(216u16);
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT: VIRTUAL_KEY = VIRTUAL_KEY(217u16);
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT: VIRTUAL_KEY = VIRTUAL_KEY(218u16);
pub const VK_OEM_4: VIRTUAL_KEY = VIRTUAL_KEY(219u16);
pub const VK_OEM_5: VIRTUAL_KEY = VIRTUAL_KEY(220u16);
pub const VK_OEM_6: VIRTUAL_KEY = VIRTUAL_KEY(221u16);
pub const VK_OEM_7: VIRTUAL_KEY = VIRTUAL_KEY(222u16);
pub const VK_OEM_8: VIRTUAL_KEY = VIRTUAL_KEY(223u16);
pub const VK_OEM_AX: VIRTUAL_KEY = VIRTUAL_KEY(225u16);
pub const VK_OEM_102: VIRTUAL_KEY = VIRTUAL_KEY(226u16);
pub const VK_ICO_HELP: VIRTUAL_KEY = VIRTUAL_KEY(227u16);
pub const VK_ICO_00: VIRTUAL_KEY = VIRTUAL_KEY(228u16);
pub const VK_PROCESSKEY: VIRTUAL_KEY = VIRTUAL_KEY(229u16);
pub const VK_ICO_CLEAR: VIRTUAL_KEY = VIRTUAL_KEY(230u16);
pub const VK_PACKET: VIRTUAL_KEY = VIRTUAL_KEY(231u16);
pub const VK_OEM_RESET: VIRTUAL_KEY = VIRTUAL_KEY(233u16);
pub const VK_OEM_JUMP: VIRTUAL_KEY = VIRTUAL_KEY(234u16);
pub const VK_OEM_PA1: VIRTUAL_KEY = VIRTUAL_KEY(235u16);
pub const VK_OEM_PA2: VIRTUAL_KEY = VIRTUAL_KEY(236u16);
pub const VK_OEM_PA3: VIRTUAL_KEY = VIRTUAL_KEY(237u16);
pub const VK_OEM_WSCTRL: VIRTUAL_KEY = VIRTUAL_KEY(238u16);
pub const VK_OEM_CUSEL: VIRTUAL_KEY = VIRTUAL_KEY(239u16);
pub const VK_OEM_ATTN: VIRTUAL_KEY = VIRTUAL_KEY(240u16);
pub const VK_OEM_FINISH: VIRTUAL_KEY = VIRTUAL_KEY(241u16);
pub const VK_OEM_COPY: VIRTUAL_KEY = VIRTUAL_KEY(242u16);
pub const VK_OEM_AUTO: VIRTUAL_KEY = VIRTUAL_KEY(243u16);
pub const VK_OEM_ENLW: VIRTUAL_KEY = VIRTUAL_KEY(244u16);
pub const VK_OEM_BACKTAB: VIRTUAL_KEY = VIRTUAL_KEY(245u16);
pub const VK_ATTN: VIRTUAL_KEY = VIRTUAL_KEY(246u16);
pub const VK_CRSEL: VIRTUAL_KEY = VIRTUAL_KEY(247u16);
pub const VK_EXSEL: VIRTUAL_KEY = VIRTUAL_KEY(248u16);
pub const VK_EREOF: VIRTUAL_KEY = VIRTUAL_KEY(249u16);
pub const VK_PLAY: VIRTUAL_KEY = VIRTUAL_KEY(250u16);
pub const VK_ZOOM: VIRTUAL_KEY = VIRTUAL_KEY(251u16);
pub const VK_NONAME: VIRTUAL_KEY = VIRTUAL_KEY(252u16);
pub const VK_PA1: VIRTUAL_KEY = VIRTUAL_KEY(253u16);
pub const VK_OEM_CLEAR: VIRTUAL_KEY = VIRTUAL_KEY(254u16);
impl ::core::marker::Copy for VIRTUAL_KEY {}
impl ::core::clone::Clone for VIRTUAL_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VIRTUAL_KEY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VIRTUAL_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VIRTUAL_KEY").field(&self.0).finish()
    }
}
impl FromIntoMemory for VIRTUAL_KEY {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u16 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        2
    }
}
pub const VK_ABNT_C1: u32 = 193u32;
pub const VK_ABNT_C2: u32 = 194u32;
pub const VK_DBE_ALPHANUMERIC: u32 = 240u32;
pub const VK_DBE_CODEINPUT: u32 = 250u32;
pub const VK_DBE_DBCSCHAR: u32 = 244u32;
pub const VK_DBE_DETERMINESTRING: u32 = 252u32;
pub const VK_DBE_ENTERDLGCONVERSIONMODE: u32 = 253u32;
pub const VK_DBE_ENTERIMECONFIGMODE: u32 = 248u32;
pub const VK_DBE_ENTERWORDREGISTERMODE: u32 = 247u32;
pub const VK_DBE_FLUSHSTRING: u32 = 249u32;
pub const VK_DBE_HIRAGANA: u32 = 242u32;
pub const VK_DBE_KATAKANA: u32 = 241u32;
pub const VK_DBE_NOCODEINPUT: u32 = 251u32;
pub const VK_DBE_NOROMAN: u32 = 246u32;
pub const VK_DBE_ROMAN: u32 = 245u32;
pub const VK_DBE_SBCSCHAR: u32 = 243u32;
pub struct VK_TO_BIT {
    pub Vk: u8,
    pub ModBits: u8,
}
impl ::core::marker::Copy for VK_TO_BIT {}
impl ::core::clone::Clone for VK_TO_BIT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VK_TO_BIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VK_TO_BIT")
            .field("Vk", &self.Vk)
            .field("ModBits", &self.ModBits)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VK_TO_BIT {
    fn eq(&self, other: &Self) -> bool {
        self.Vk == other.Vk && self.ModBits == other.ModBits
    }
}
impl ::core::cmp::Eq for VK_TO_BIT {}
impl FromIntoMemory for VK_TO_BIT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 2);
        let f_Vk = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_ModBits = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        Self {
            Vk: f_Vk,
            ModBits: f_ModBits,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 2);
        FromIntoMemory::into_bytes(self.Vk, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.ModBits, &mut into[1..1 + 1]);
    }
    fn size() -> usize {
        2
    }
}
pub struct VK_TO_WCHARS1 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 1],
}
impl ::core::marker::Copy for VK_TO_WCHARS1 {}
impl ::core::clone::Clone for VK_TO_WCHARS1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VK_TO_WCHARS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VK_TO_WCHARS1")
            .field("VirtualKey", &self.VirtualKey)
            .field("Attributes", &self.Attributes)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VK_TO_WCHARS1 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.Attributes == other.Attributes
            && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for VK_TO_WCHARS1 {}
impl FromIntoMemory for VK_TO_WCHARS1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 3);
        let f_VirtualKey = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_Attributes = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_wch = <[u16; 1] as FromIntoMemory>::from_bytes(&from[2..2 + 1]);
        Self {
            VirtualKey: f_VirtualKey,
            Attributes: f_Attributes,
            wch: f_wch,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 3);
        FromIntoMemory::into_bytes(self.VirtualKey, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.Attributes, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.wch, &mut into[2..2 + 1]);
    }
    fn size() -> usize {
        3
    }
}
pub struct VK_TO_WCHARS10 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 10],
}
impl ::core::marker::Copy for VK_TO_WCHARS10 {}
impl ::core::clone::Clone for VK_TO_WCHARS10 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VK_TO_WCHARS10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VK_TO_WCHARS10")
            .field("VirtualKey", &self.VirtualKey)
            .field("Attributes", &self.Attributes)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VK_TO_WCHARS10 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.Attributes == other.Attributes
            && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for VK_TO_WCHARS10 {}
impl FromIntoMemory for VK_TO_WCHARS10 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_VirtualKey = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_Attributes = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_wch = <[u16; 10] as FromIntoMemory>::from_bytes(&from[2..2 + 10]);
        Self {
            VirtualKey: f_VirtualKey,
            Attributes: f_Attributes,
            wch: f_wch,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.VirtualKey, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.Attributes, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.wch, &mut into[2..2 + 10]);
    }
    fn size() -> usize {
        12
    }
}
pub struct VK_TO_WCHARS2 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 2],
}
impl ::core::marker::Copy for VK_TO_WCHARS2 {}
impl ::core::clone::Clone for VK_TO_WCHARS2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VK_TO_WCHARS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VK_TO_WCHARS2")
            .field("VirtualKey", &self.VirtualKey)
            .field("Attributes", &self.Attributes)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VK_TO_WCHARS2 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.Attributes == other.Attributes
            && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for VK_TO_WCHARS2 {}
impl FromIntoMemory for VK_TO_WCHARS2 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_VirtualKey = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_Attributes = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_wch = <[u16; 2] as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        Self {
            VirtualKey: f_VirtualKey,
            Attributes: f_Attributes,
            wch: f_wch,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.VirtualKey, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.Attributes, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.wch, &mut into[2..2 + 2]);
    }
    fn size() -> usize {
        4
    }
}
pub struct VK_TO_WCHARS3 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 3],
}
impl ::core::marker::Copy for VK_TO_WCHARS3 {}
impl ::core::clone::Clone for VK_TO_WCHARS3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VK_TO_WCHARS3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VK_TO_WCHARS3")
            .field("VirtualKey", &self.VirtualKey)
            .field("Attributes", &self.Attributes)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VK_TO_WCHARS3 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.Attributes == other.Attributes
            && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for VK_TO_WCHARS3 {}
impl FromIntoMemory for VK_TO_WCHARS3 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 5);
        let f_VirtualKey = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_Attributes = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_wch = <[u16; 3] as FromIntoMemory>::from_bytes(&from[2..2 + 3]);
        Self {
            VirtualKey: f_VirtualKey,
            Attributes: f_Attributes,
            wch: f_wch,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 5);
        FromIntoMemory::into_bytes(self.VirtualKey, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.Attributes, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.wch, &mut into[2..2 + 3]);
    }
    fn size() -> usize {
        5
    }
}
pub struct VK_TO_WCHARS4 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 4],
}
impl ::core::marker::Copy for VK_TO_WCHARS4 {}
impl ::core::clone::Clone for VK_TO_WCHARS4 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VK_TO_WCHARS4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VK_TO_WCHARS4")
            .field("VirtualKey", &self.VirtualKey)
            .field("Attributes", &self.Attributes)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VK_TO_WCHARS4 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.Attributes == other.Attributes
            && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for VK_TO_WCHARS4 {}
impl FromIntoMemory for VK_TO_WCHARS4 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 6);
        let f_VirtualKey = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_Attributes = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_wch = <[u16; 4] as FromIntoMemory>::from_bytes(&from[2..2 + 4]);
        Self {
            VirtualKey: f_VirtualKey,
            Attributes: f_Attributes,
            wch: f_wch,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 6);
        FromIntoMemory::into_bytes(self.VirtualKey, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.Attributes, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.wch, &mut into[2..2 + 4]);
    }
    fn size() -> usize {
        6
    }
}
pub struct VK_TO_WCHARS5 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 5],
}
impl ::core::marker::Copy for VK_TO_WCHARS5 {}
impl ::core::clone::Clone for VK_TO_WCHARS5 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VK_TO_WCHARS5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VK_TO_WCHARS5")
            .field("VirtualKey", &self.VirtualKey)
            .field("Attributes", &self.Attributes)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VK_TO_WCHARS5 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.Attributes == other.Attributes
            && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for VK_TO_WCHARS5 {}
impl FromIntoMemory for VK_TO_WCHARS5 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 7);
        let f_VirtualKey = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_Attributes = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_wch = <[u16; 5] as FromIntoMemory>::from_bytes(&from[2..2 + 5]);
        Self {
            VirtualKey: f_VirtualKey,
            Attributes: f_Attributes,
            wch: f_wch,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 7);
        FromIntoMemory::into_bytes(self.VirtualKey, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.Attributes, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.wch, &mut into[2..2 + 5]);
    }
    fn size() -> usize {
        7
    }
}
pub struct VK_TO_WCHARS6 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 6],
}
impl ::core::marker::Copy for VK_TO_WCHARS6 {}
impl ::core::clone::Clone for VK_TO_WCHARS6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VK_TO_WCHARS6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VK_TO_WCHARS6")
            .field("VirtualKey", &self.VirtualKey)
            .field("Attributes", &self.Attributes)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VK_TO_WCHARS6 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.Attributes == other.Attributes
            && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for VK_TO_WCHARS6 {}
impl FromIntoMemory for VK_TO_WCHARS6 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_VirtualKey = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_Attributes = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_wch = <[u16; 6] as FromIntoMemory>::from_bytes(&from[2..2 + 6]);
        Self {
            VirtualKey: f_VirtualKey,
            Attributes: f_Attributes,
            wch: f_wch,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.VirtualKey, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.Attributes, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.wch, &mut into[2..2 + 6]);
    }
    fn size() -> usize {
        8
    }
}
pub struct VK_TO_WCHARS7 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 7],
}
impl ::core::marker::Copy for VK_TO_WCHARS7 {}
impl ::core::clone::Clone for VK_TO_WCHARS7 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VK_TO_WCHARS7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VK_TO_WCHARS7")
            .field("VirtualKey", &self.VirtualKey)
            .field("Attributes", &self.Attributes)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VK_TO_WCHARS7 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.Attributes == other.Attributes
            && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for VK_TO_WCHARS7 {}
impl FromIntoMemory for VK_TO_WCHARS7 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 9);
        let f_VirtualKey = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_Attributes = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_wch = <[u16; 7] as FromIntoMemory>::from_bytes(&from[2..2 + 7]);
        Self {
            VirtualKey: f_VirtualKey,
            Attributes: f_Attributes,
            wch: f_wch,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 9);
        FromIntoMemory::into_bytes(self.VirtualKey, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.Attributes, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.wch, &mut into[2..2 + 7]);
    }
    fn size() -> usize {
        9
    }
}
pub struct VK_TO_WCHARS8 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 8],
}
impl ::core::marker::Copy for VK_TO_WCHARS8 {}
impl ::core::clone::Clone for VK_TO_WCHARS8 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VK_TO_WCHARS8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VK_TO_WCHARS8")
            .field("VirtualKey", &self.VirtualKey)
            .field("Attributes", &self.Attributes)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VK_TO_WCHARS8 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.Attributes == other.Attributes
            && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for VK_TO_WCHARS8 {}
impl FromIntoMemory for VK_TO_WCHARS8 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 10);
        let f_VirtualKey = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_Attributes = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_wch = <[u16; 8] as FromIntoMemory>::from_bytes(&from[2..2 + 8]);
        Self {
            VirtualKey: f_VirtualKey,
            Attributes: f_Attributes,
            wch: f_wch,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 10);
        FromIntoMemory::into_bytes(self.VirtualKey, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.Attributes, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.wch, &mut into[2..2 + 8]);
    }
    fn size() -> usize {
        10
    }
}
pub struct VK_TO_WCHARS9 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 9],
}
impl ::core::marker::Copy for VK_TO_WCHARS9 {}
impl ::core::clone::Clone for VK_TO_WCHARS9 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VK_TO_WCHARS9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VK_TO_WCHARS9")
            .field("VirtualKey", &self.VirtualKey)
            .field("Attributes", &self.Attributes)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VK_TO_WCHARS9 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.Attributes == other.Attributes
            && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for VK_TO_WCHARS9 {}
impl FromIntoMemory for VK_TO_WCHARS9 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 11);
        let f_VirtualKey = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_Attributes = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_wch = <[u16; 9] as FromIntoMemory>::from_bytes(&from[2..2 + 9]);
        Self {
            VirtualKey: f_VirtualKey,
            Attributes: f_Attributes,
            wch: f_wch,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 11);
        FromIntoMemory::into_bytes(self.VirtualKey, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.Attributes, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.wch, &mut into[2..2 + 9]);
    }
    fn size() -> usize {
        11
    }
}
pub struct VK_TO_WCHAR_TABLE {
    pub pVkToWchars: MutPtr<VK_TO_WCHARS1>,
    pub nModifications: u8,
    pub cbSize: u8,
}
impl ::core::marker::Copy for VK_TO_WCHAR_TABLE {}
impl ::core::clone::Clone for VK_TO_WCHAR_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VK_TO_WCHAR_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VK_TO_WCHAR_TABLE")
            .field("pVkToWchars", &self.pVkToWchars)
            .field("nModifications", &self.nModifications)
            .field("cbSize", &self.cbSize)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VK_TO_WCHAR_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.pVkToWchars == other.pVkToWchars
            && self.nModifications == other.nModifications
            && self.cbSize == other.cbSize
    }
}
impl ::core::cmp::Eq for VK_TO_WCHAR_TABLE {}
impl FromIntoMemory for VK_TO_WCHAR_TABLE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_pVkToWchars = <MutPtr<VK_TO_WCHARS1> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_nModifications = <u8 as FromIntoMemory>::from_bytes(&from[4..4 + 1]);
        let f_cbSize = <u8 as FromIntoMemory>::from_bytes(&from[5..5 + 1]);
        Self {
            pVkToWchars: f_pVkToWchars,
            nModifications: f_nModifications,
            cbSize: f_cbSize,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.pVkToWchars, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.nModifications, &mut into[4..4 + 1]);
        FromIntoMemory::into_bytes(self.cbSize, &mut into[5..5 + 1]);
    }
    fn size() -> usize {
        8
    }
}
pub struct VK_VSC {
    pub Vk: u8,
    pub Vsc: u8,
}
impl ::core::marker::Copy for VK_VSC {}
impl ::core::clone::Clone for VK_VSC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VK_VSC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VK_VSC")
            .field("Vk", &self.Vk)
            .field("Vsc", &self.Vsc)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VK_VSC {
    fn eq(&self, other: &Self) -> bool {
        self.Vk == other.Vk && self.Vsc == other.Vsc
    }
}
impl ::core::cmp::Eq for VK_VSC {}
impl FromIntoMemory for VK_VSC {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 2);
        let f_Vk = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_Vsc = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        Self {
            Vk: f_Vk,
            Vsc: f_Vsc,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 2);
        FromIntoMemory::into_bytes(self.Vk, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.Vsc, &mut into[1..1 + 1]);
    }
    fn size() -> usize {
        2
    }
}
pub const VK__none_: u32 = 255u32;
pub struct VSC_LPWSTR {
    pub vsc: u8,
    pub pwsz: PWSTR,
}
impl ::core::marker::Copy for VSC_LPWSTR {}
impl ::core::clone::Clone for VSC_LPWSTR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VSC_LPWSTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VSC_LPWSTR")
            .field("vsc", &self.vsc)
            .field("pwsz", &self.pwsz)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VSC_LPWSTR {
    fn eq(&self, other: &Self) -> bool {
        self.vsc == other.vsc && self.pwsz == other.pwsz
    }
}
impl ::core::cmp::Eq for VSC_LPWSTR {}
impl FromIntoMemory for VSC_LPWSTR {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_vsc = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_pwsz = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            vsc: f_vsc,
            pwsz: f_pwsz,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.vsc, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.pwsz, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct VSC_VK {
    pub Vsc: u8,
    pub Vk: u16,
}
impl ::core::marker::Copy for VSC_VK {}
impl ::core::clone::Clone for VSC_VK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VSC_VK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VSC_VK")
            .field("Vsc", &self.Vsc)
            .field("Vk", &self.Vk)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VSC_VK {
    fn eq(&self, other: &Self) -> bool {
        self.Vsc == other.Vsc && self.Vk == other.Vk
    }
}
impl ::core::cmp::Eq for VSC_VK {}
impl FromIntoMemory for VSC_VK {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_Vsc = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_Vk = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        Self {
            Vsc: f_Vsc,
            Vk: f_Vk,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.Vsc, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.Vk, &mut into[2..2 + 2]);
    }
    fn size() -> usize {
        4
    }
}
pub const WCH_DEAD: u32 = 61441u32;
pub const WCH_LGTR: u32 = 61442u32;
pub const WCH_NONE: u32 = 61440u32;
pub struct _VK_FUNCTION_PARAM {
    pub NLSFEProcIndex: u8,
    pub NLSFEProcParam: u32,
}
impl ::core::marker::Copy for _VK_FUNCTION_PARAM {}
impl ::core::clone::Clone for _VK_FUNCTION_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for _VK_FUNCTION_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_VK_FUNCTION_PARAM")
            .field("NLSFEProcIndex", &self.NLSFEProcIndex)
            .field("NLSFEProcParam", &self.NLSFEProcParam)
            .finish()
    }
}
impl ::core::cmp::PartialEq for _VK_FUNCTION_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.NLSFEProcIndex == other.NLSFEProcIndex && self.NLSFEProcParam == other.NLSFEProcParam
    }
}
impl ::core::cmp::Eq for _VK_FUNCTION_PARAM {}
impl FromIntoMemory for _VK_FUNCTION_PARAM {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_NLSFEProcIndex = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_NLSFEProcParam = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            NLSFEProcIndex: f_NLSFEProcIndex,
            NLSFEProcParam: f_NLSFEProcParam,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.NLSFEProcIndex, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.NLSFEProcParam, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct _VK_TO_FUNCTION_TABLE {
    pub Vk: u8,
    pub NLSFEProcType: u8,
    pub NLSFEProcCurrent: u8,
    pub NLSFEProcSwitch: u8,
    pub NLSFEProc: [_VK_FUNCTION_PARAM; 8],
    pub NLSFEProcAlt: [_VK_FUNCTION_PARAM; 8],
}
impl ::core::marker::Copy for _VK_TO_FUNCTION_TABLE {}
impl ::core::clone::Clone for _VK_TO_FUNCTION_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for _VK_TO_FUNCTION_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_VK_TO_FUNCTION_TABLE")
            .field("Vk", &self.Vk)
            .field("NLSFEProcType", &self.NLSFEProcType)
            .field("NLSFEProcCurrent", &self.NLSFEProcCurrent)
            .field("NLSFEProcSwitch", &self.NLSFEProcSwitch)
            .field("NLSFEProc", &self.NLSFEProc)
            .field("NLSFEProcAlt", &self.NLSFEProcAlt)
            .finish()
    }
}
impl ::core::cmp::PartialEq for _VK_TO_FUNCTION_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.Vk == other.Vk
            && self.NLSFEProcType == other.NLSFEProcType
            && self.NLSFEProcCurrent == other.NLSFEProcCurrent
            && self.NLSFEProcSwitch == other.NLSFEProcSwitch
            && self.NLSFEProc == other.NLSFEProc
            && self.NLSFEProcAlt == other.NLSFEProcAlt
    }
}
impl ::core::cmp::Eq for _VK_TO_FUNCTION_TABLE {}
impl FromIntoMemory for _VK_TO_FUNCTION_TABLE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 132);
        let f_Vk = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_NLSFEProcType = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_NLSFEProcCurrent = <u8 as FromIntoMemory>::from_bytes(&from[2..2 + 1]);
        let f_NLSFEProcSwitch = <u8 as FromIntoMemory>::from_bytes(&from[3..3 + 1]);
        let f_NLSFEProc = <[_VK_FUNCTION_PARAM; 8] as FromIntoMemory>::from_bytes(&from[4..4 + 64]);
        let f_NLSFEProcAlt =
            <[_VK_FUNCTION_PARAM; 8] as FromIntoMemory>::from_bytes(&from[68..68 + 64]);
        Self {
            Vk: f_Vk,
            NLSFEProcType: f_NLSFEProcType,
            NLSFEProcCurrent: f_NLSFEProcCurrent,
            NLSFEProcSwitch: f_NLSFEProcSwitch,
            NLSFEProc: f_NLSFEProc,
            NLSFEProcAlt: f_NLSFEProcAlt,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 132);
        FromIntoMemory::into_bytes(self.Vk, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.NLSFEProcType, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.NLSFEProcCurrent, &mut into[2..2 + 1]);
        FromIntoMemory::into_bytes(self.NLSFEProcSwitch, &mut into[3..3 + 1]);
        FromIntoMemory::into_bytes(self.NLSFEProc, &mut into[4..4 + 64]);
        FromIntoMemory::into_bytes(self.NLSFEProcAlt, &mut into[68..68 + 64]);
    }
    fn size() -> usize {
        132
    }
}
pub struct tagKbdLayer {
    pub pCharModifiers: MutPtr<MODIFIERS>,
    pub pVkToWcharTable: MutPtr<VK_TO_WCHAR_TABLE>,
    pub pDeadKey: MutPtr<DEADKEY>,
    pub pKeyNames: MutPtr<VSC_LPWSTR>,
    pub pKeyNamesExt: MutPtr<VSC_LPWSTR>,
    pub pKeyNamesDead: MutPtr<ConstPtr<u16>>,
    pub pusVSCtoVK: MutPtr<u16>,
    pub bMaxVSCtoVK: u8,
    pub pVSCtoVK_E0: MutPtr<VSC_VK>,
    pub pVSCtoVK_E1: MutPtr<VSC_VK>,
    pub fLocaleFlags: u32,
    pub nLgMax: u8,
    pub cbLgEntry: u8,
    pub pLigature: MutPtr<LIGATURE1>,
    pub dwType: u32,
    pub dwSubType: u32,
}
impl ::core::marker::Copy for tagKbdLayer {}
impl ::core::clone::Clone for tagKbdLayer {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for tagKbdLayer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("tagKbdLayer")
            .field("pCharModifiers", &self.pCharModifiers)
            .field("pVkToWcharTable", &self.pVkToWcharTable)
            .field("pDeadKey", &self.pDeadKey)
            .field("pKeyNames", &self.pKeyNames)
            .field("pKeyNamesExt", &self.pKeyNamesExt)
            .field("pKeyNamesDead", &self.pKeyNamesDead)
            .field("pusVSCtoVK", &self.pusVSCtoVK)
            .field("bMaxVSCtoVK", &self.bMaxVSCtoVK)
            .field("pVSCtoVK_E0", &self.pVSCtoVK_E0)
            .field("pVSCtoVK_E1", &self.pVSCtoVK_E1)
            .field("fLocaleFlags", &self.fLocaleFlags)
            .field("nLgMax", &self.nLgMax)
            .field("cbLgEntry", &self.cbLgEntry)
            .field("pLigature", &self.pLigature)
            .field("dwType", &self.dwType)
            .field("dwSubType", &self.dwSubType)
            .finish()
    }
}
impl ::core::cmp::PartialEq for tagKbdLayer {
    fn eq(&self, other: &Self) -> bool {
        self.pCharModifiers == other.pCharModifiers
            && self.pVkToWcharTable == other.pVkToWcharTable
            && self.pDeadKey == other.pDeadKey
            && self.pKeyNames == other.pKeyNames
            && self.pKeyNamesExt == other.pKeyNamesExt
            && self.pKeyNamesDead == other.pKeyNamesDead
            && self.pusVSCtoVK == other.pusVSCtoVK
            && self.bMaxVSCtoVK == other.bMaxVSCtoVK
            && self.pVSCtoVK_E0 == other.pVSCtoVK_E0
            && self.pVSCtoVK_E1 == other.pVSCtoVK_E1
            && self.fLocaleFlags == other.fLocaleFlags
            && self.nLgMax == other.nLgMax
            && self.cbLgEntry == other.cbLgEntry
            && self.pLigature == other.pLigature
            && self.dwType == other.dwType
            && self.dwSubType == other.dwSubType
    }
}
impl ::core::cmp::Eq for tagKbdLayer {}
impl FromIntoMemory for tagKbdLayer {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 60);
        let f_pCharModifiers = <MutPtr<MODIFIERS> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_pVkToWcharTable =
            <MutPtr<VK_TO_WCHAR_TABLE> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_pDeadKey = <MutPtr<DEADKEY> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_pKeyNames = <MutPtr<VSC_LPWSTR> as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_pKeyNamesExt = <MutPtr<VSC_LPWSTR> as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_pKeyNamesDead =
            <MutPtr<ConstPtr<u16>> as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_pusVSCtoVK = <MutPtr<u16> as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_bMaxVSCtoVK = <u8 as FromIntoMemory>::from_bytes(&from[28..28 + 1]);
        let f_pVSCtoVK_E0 = <MutPtr<VSC_VK> as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_pVSCtoVK_E1 = <MutPtr<VSC_VK> as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_fLocaleFlags = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_nLgMax = <u8 as FromIntoMemory>::from_bytes(&from[44..44 + 1]);
        let f_cbLgEntry = <u8 as FromIntoMemory>::from_bytes(&from[45..45 + 1]);
        let f_pLigature = <MutPtr<LIGATURE1> as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_dwType = <u32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_dwSubType = <u32 as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        Self {
            pCharModifiers: f_pCharModifiers,
            pVkToWcharTable: f_pVkToWcharTable,
            pDeadKey: f_pDeadKey,
            pKeyNames: f_pKeyNames,
            pKeyNamesExt: f_pKeyNamesExt,
            pKeyNamesDead: f_pKeyNamesDead,
            pusVSCtoVK: f_pusVSCtoVK,
            bMaxVSCtoVK: f_bMaxVSCtoVK,
            pVSCtoVK_E0: f_pVSCtoVK_E0,
            pVSCtoVK_E1: f_pVSCtoVK_E1,
            fLocaleFlags: f_fLocaleFlags,
            nLgMax: f_nLgMax,
            cbLgEntry: f_cbLgEntry,
            pLigature: f_pLigature,
            dwType: f_dwType,
            dwSubType: f_dwSubType,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 60);
        FromIntoMemory::into_bytes(self.pCharModifiers, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.pVkToWcharTable, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.pDeadKey, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.pKeyNames, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.pKeyNamesExt, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.pKeyNamesDead, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.pusVSCtoVK, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.bMaxVSCtoVK, &mut into[28..28 + 1]);
        FromIntoMemory::into_bytes(self.pVSCtoVK_E0, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.pVSCtoVK_E1, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.fLocaleFlags, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.nLgMax, &mut into[44..44 + 1]);
        FromIntoMemory::into_bytes(self.cbLgEntry, &mut into[45..45 + 1]);
        FromIntoMemory::into_bytes(self.pLigature, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.dwType, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.dwSubType, &mut into[56..56 + 4]);
    }
    fn size() -> usize {
        60
    }
}
pub struct tagKbdNlsLayer {
    pub OEMIdentifier: u16,
    pub LayoutInformation: u16,
    pub NumOfVkToF: u32,
    pub pVkToF: MutPtr<_VK_TO_FUNCTION_TABLE>,
    pub NumOfMouseVKey: i32,
    pub pusMouseVKey: MutPtr<u16>,
}
impl ::core::marker::Copy for tagKbdNlsLayer {}
impl ::core::clone::Clone for tagKbdNlsLayer {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for tagKbdNlsLayer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("tagKbdNlsLayer")
            .field("OEMIdentifier", &self.OEMIdentifier)
            .field("LayoutInformation", &self.LayoutInformation)
            .field("NumOfVkToF", &self.NumOfVkToF)
            .field("pVkToF", &self.pVkToF)
            .field("NumOfMouseVKey", &self.NumOfMouseVKey)
            .field("pusMouseVKey", &self.pusMouseVKey)
            .finish()
    }
}
impl ::core::cmp::PartialEq for tagKbdNlsLayer {
    fn eq(&self, other: &Self) -> bool {
        self.OEMIdentifier == other.OEMIdentifier
            && self.LayoutInformation == other.LayoutInformation
            && self.NumOfVkToF == other.NumOfVkToF
            && self.pVkToF == other.pVkToF
            && self.NumOfMouseVKey == other.NumOfMouseVKey
            && self.pusMouseVKey == other.pusMouseVKey
    }
}
impl ::core::cmp::Eq for tagKbdNlsLayer {}
impl FromIntoMemory for tagKbdNlsLayer {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_OEMIdentifier = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_LayoutInformation = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_NumOfVkToF = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_pVkToF =
            <MutPtr<_VK_TO_FUNCTION_TABLE> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_NumOfMouseVKey = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_pusMouseVKey = <MutPtr<u16> as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            OEMIdentifier: f_OEMIdentifier,
            LayoutInformation: f_LayoutInformation,
            NumOfVkToF: f_NumOfVkToF,
            pVkToF: f_pVkToF,
            NumOfMouseVKey: f_NumOfMouseVKey,
            pusMouseVKey: f_pusMouseVKey,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.OEMIdentifier, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.LayoutInformation, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.NumOfVkToF, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.pVkToF, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.NumOfMouseVKey, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.pusMouseVKey, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub const wszACUTE: &'static str = "\u{301}";
pub const wszBREVE: &'static str = "\u{306}";
pub const wszCEDILLA: &'static str = "\u{327}";
pub const wszCIRCUMFLEX: &'static str = "\u{302}";
pub const wszDIARESIS_TONOS: &'static str = "\u{385}";
pub const wszDOT_ABOVE: &'static str = "\u{307}";
pub const wszDOUBLE_ACUTE: &'static str = "\u{30b}";
pub const wszGRAVE: &'static str = "\u{300}";
pub const wszHACEK: &'static str = "\u{30c}";
pub const wszHOOK_ABOVE: &'static str = "\u{309}";
pub const wszMACRON: &'static str = "\u{304}";
pub const wszOGONEK: &'static str = "\u{328}";
pub const wszOVERSCORE: &'static str = "\u{305}";
pub const wszRING: &'static str = "\u{30a}";
pub const wszTILDE: &'static str = "\u{303}";
pub const wszTONOS: &'static str = "\u{384}";
pub const wszUMLAUT: &'static str = "\u{308}";
pub trait Api {
    #[doc = "*Required namespaces: 'Windows.Win32.UI.TextServices'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn ActivateKeyboardLayout(
        &self,
        hkl: super::super::TextServices::HKL,
        flags: ACTIVATE_KEYBOARD_LAYOUT_FLAGS,
    ) -> super::super::TextServices::HKL {
        todo!("ActivateKeyboardLayout")
    }
    fn BlockInput(
        &self,
        f_block_it: super::super::super::Foundation::BOOL,
    ) -> super::super::super::Foundation::BOOL {
        todo!("BlockInput")
    }
    fn DragDetect(
        &self,
        hwnd: super::super::super::Foundation::HWND,
        pt: super::super::super::Foundation::POINT,
    ) -> super::super::super::Foundation::BOOL {
        todo!("DragDetect")
    }
    fn EnableWindow(
        &self,
        h_wnd: super::super::super::Foundation::HWND,
        b_enable: super::super::super::Foundation::BOOL,
    ) -> super::super::super::Foundation::BOOL {
        todo!("EnableWindow")
    }
    fn GetActiveWindow(&self) -> super::super::super::Foundation::HWND {
        todo!("GetActiveWindow")
    }
    fn GetAsyncKeyState(&self, v_key: i32) -> i16 {
        todo!("GetAsyncKeyState")
    }
    fn GetCapture(&self) -> super::super::super::Foundation::HWND {
        todo!("GetCapture")
    }
    fn GetDoubleClickTime(&self) -> u32 {
        todo!("GetDoubleClickTime")
    }
    fn GetFocus(&self) -> super::super::super::Foundation::HWND {
        todo!("GetFocus")
    }
    fn GetKBCodePage(&self) -> u32 {
        todo!("GetKBCodePage")
    }
    fn GetKeyNameTextA(&self, l_param: i32, lp_string: PSTR, cch_size: i32) -> i32 {
        todo!("GetKeyNameTextA")
    }
    fn GetKeyNameTextW(&self, l_param: i32, lp_string: PWSTR, cch_size: i32) -> i32 {
        todo!("GetKeyNameTextW")
    }
    fn GetKeyState(&self, n_virt_key: i32) -> i16 {
        todo!("GetKeyState")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.UI.TextServices'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetKeyboardLayout(&self, id_thread: u32) -> super::super::TextServices::HKL {
        todo!("GetKeyboardLayout")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.UI.TextServices'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetKeyboardLayoutList(
        &self,
        n_buff: i32,
        lp_list: MutPtr<super::super::TextServices::HKL>,
    ) -> i32 {
        todo!("GetKeyboardLayoutList")
    }
    fn GetKeyboardLayoutNameA(&self, pwsz_klid: PSTR) -> super::super::super::Foundation::BOOL {
        todo!("GetKeyboardLayoutNameA")
    }
    fn GetKeyboardLayoutNameW(&self, pwsz_klid: PWSTR) -> super::super::super::Foundation::BOOL {
        todo!("GetKeyboardLayoutNameW")
    }
    fn GetKeyboardState(&self, lp_key_state: MutPtr<u8>) -> super::super::super::Foundation::BOOL {
        todo!("GetKeyboardState")
    }
    fn GetKeyboardType(&self, n_type_flag: i32) -> i32 {
        todo!("GetKeyboardType")
    }
    fn GetLastInputInfo(
        &self,
        plii: MutPtr<LASTINPUTINFO>,
    ) -> super::super::super::Foundation::BOOL {
        todo!("GetLastInputInfo")
    }
    fn GetMouseMovePointsEx(
        &self,
        cb_size: u32,
        lppt: ConstPtr<MOUSEMOVEPOINT>,
        lppt_buf: MutPtr<MOUSEMOVEPOINT>,
        n_buf_points: i32,
        resolution: GET_MOUSE_MOVE_POINTS_EX_RESOLUTION,
    ) -> i32 {
        todo!("GetMouseMovePointsEx")
    }
    fn IsWindowEnabled(
        &self,
        h_wnd: super::super::super::Foundation::HWND,
    ) -> super::super::super::Foundation::BOOL {
        todo!("IsWindowEnabled")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.UI.TextServices'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn LoadKeyboardLayoutA(
        &self,
        pwsz_klid: PCSTR,
        flags: ACTIVATE_KEYBOARD_LAYOUT_FLAGS,
    ) -> super::super::TextServices::HKL {
        todo!("LoadKeyboardLayoutA")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.UI.TextServices'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn LoadKeyboardLayoutW(
        &self,
        pwsz_klid: PCWSTR,
        flags: ACTIVATE_KEYBOARD_LAYOUT_FLAGS,
    ) -> super::super::TextServices::HKL {
        todo!("LoadKeyboardLayoutW")
    }
    fn MapVirtualKeyA(&self, u_code: u32, u_map_type: u32) -> u32 {
        todo!("MapVirtualKeyA")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.UI.TextServices'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn MapVirtualKeyExA(
        &self,
        u_code: u32,
        u_map_type: u32,
        dwhkl: super::super::TextServices::HKL,
    ) -> u32 {
        todo!("MapVirtualKeyExA")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.UI.TextServices'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn MapVirtualKeyExW(
        &self,
        u_code: u32,
        u_map_type: u32,
        dwhkl: super::super::TextServices::HKL,
    ) -> u32 {
        todo!("MapVirtualKeyExW")
    }
    fn MapVirtualKeyW(&self, u_code: u32, u_map_type: u32) -> u32 {
        todo!("MapVirtualKeyW")
    }
    fn OemKeyScan(&self, w_oem_char: u16) -> u32 {
        todo!("OemKeyScan")
    }
    fn RegisterHotKey(
        &self,
        h_wnd: super::super::super::Foundation::HWND,
        id: i32,
        fs_modifiers: HOT_KEY_MODIFIERS,
        vk: u32,
    ) -> super::super::super::Foundation::BOOL {
        todo!("RegisterHotKey")
    }
    fn ReleaseCapture(&self) -> super::super::super::Foundation::BOOL {
        todo!("ReleaseCapture")
    }
    fn SendInput(&self, c_inputs: u32, p_inputs: ConstPtr<INPUT>, cb_size: i32) -> u32 {
        todo!("SendInput")
    }
    fn SetActiveWindow(
        &self,
        h_wnd: super::super::super::Foundation::HWND,
    ) -> super::super::super::Foundation::HWND {
        todo!("SetActiveWindow")
    }
    fn SetCapture(
        &self,
        h_wnd: super::super::super::Foundation::HWND,
    ) -> super::super::super::Foundation::HWND {
        todo!("SetCapture")
    }
    fn SetDoubleClickTime(&self, param_0: u32) -> super::super::super::Foundation::BOOL {
        todo!("SetDoubleClickTime")
    }
    fn SetFocus(
        &self,
        h_wnd: super::super::super::Foundation::HWND,
    ) -> super::super::super::Foundation::HWND {
        todo!("SetFocus")
    }
    fn SetKeyboardState(
        &self,
        lp_key_state: ConstPtr<u8>,
    ) -> super::super::super::Foundation::BOOL {
        todo!("SetKeyboardState")
    }
    fn SwapMouseButton(
        &self,
        f_swap: super::super::super::Foundation::BOOL,
    ) -> super::super::super::Foundation::BOOL {
        todo!("SwapMouseButton")
    }
    fn ToAscii(
        &self,
        u_virt_key: u32,
        u_scan_code: u32,
        lp_key_state: ConstPtr<u8>,
        lp_char: MutPtr<u16>,
        u_flags: u32,
    ) -> i32 {
        todo!("ToAscii")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.UI.TextServices'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn ToAsciiEx(
        &self,
        u_virt_key: u32,
        u_scan_code: u32,
        lp_key_state: ConstPtr<u8>,
        lp_char: MutPtr<u16>,
        u_flags: u32,
        dwhkl: super::super::TextServices::HKL,
    ) -> i32 {
        todo!("ToAsciiEx")
    }
    fn ToUnicode(
        &self,
        w_virt_key: u32,
        w_scan_code: u32,
        lp_key_state: ConstPtr<u8>,
        pwsz_buff: PWSTR,
        cch_buff: i32,
        w_flags: u32,
    ) -> i32 {
        todo!("ToUnicode")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.UI.TextServices'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn ToUnicodeEx(
        &self,
        w_virt_key: u32,
        w_scan_code: u32,
        lp_key_state: ConstPtr<u8>,
        pwsz_buff: PWSTR,
        cch_buff: i32,
        w_flags: u32,
        dwhkl: super::super::TextServices::HKL,
    ) -> i32 {
        todo!("ToUnicodeEx")
    }
    fn TrackMouseEvent(
        &self,
        lp_event_track: MutPtr<TRACKMOUSEEVENT>,
    ) -> super::super::super::Foundation::BOOL {
        todo!("TrackMouseEvent")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.UI.TextServices'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn UnloadKeyboardLayout(
        &self,
        hkl: super::super::TextServices::HKL,
    ) -> super::super::super::Foundation::BOOL {
        todo!("UnloadKeyboardLayout")
    }
    fn UnregisterHotKey(
        &self,
        h_wnd: super::super::super::Foundation::HWND,
        id: i32,
    ) -> super::super::super::Foundation::BOOL {
        todo!("UnregisterHotKey")
    }
    fn VkKeyScanA(&self, ch: super::super::super::Foundation::CHAR) -> i16 {
        todo!("VkKeyScanA")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.UI.TextServices'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn VkKeyScanExA(
        &self,
        ch: super::super::super::Foundation::CHAR,
        dwhkl: super::super::TextServices::HKL,
    ) -> i16 {
        todo!("VkKeyScanExA")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.UI.TextServices'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn VkKeyScanExW(&self, ch: u16, dwhkl: super::super::TextServices::HKL) -> i16 {
        todo!("VkKeyScanExW")
    }
    fn VkKeyScanW(&self, ch: u16) -> i16 {
        todo!("VkKeyScanW")
    }
    fn _TrackMouseEvent(
        &self,
        lp_event_track: MutPtr<TRACKMOUSEEVENT>,
    ) -> super::super::super::Foundation::BOOL {
        todo!("_TrackMouseEvent")
    }
    fn keybd_event(
        &self,
        b_vk: u8,
        b_scan: u8,
        dw_flags: KEYBD_EVENT_FLAGS,
        dw_extra_info: PtrRepr,
    ) {
        todo!("keybd_event")
    }
    fn mouse_event(
        &self,
        dw_flags: MOUSE_EVENT_FLAGS,
        dx: i32,
        dy: i32,
        dw_data: u32,
        dw_extra_info: PtrRepr,
    ) {
        todo!("mouse_event")
    }
}
pub fn get_api(ctx: &crate::core::Win32Context) -> std::sync::Arc<dyn Api> {
    ctx.get::<dyn Api>()
}
