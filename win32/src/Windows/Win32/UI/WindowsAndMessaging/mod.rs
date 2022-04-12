#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    clippy::all
)]
#[allow(unused)]
use win32::core::prelude::*;
pub struct ACCEL {
    pub fVirt: u8,
    pub key: u16,
    pub cmd: u16,
}
impl ::core::marker::Copy for ACCEL {}
impl ::core::clone::Clone for ACCEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACCEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCEL")
            .field("fVirt", &self.fVirt)
            .field("key", &self.key)
            .field("cmd", &self.cmd)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACCEL {
    fn eq(&self, other: &Self) -> bool {
        self.fVirt == other.fVirt && self.key == other.key && self.cmd == other.cmd
    }
}
impl ::core::cmp::Eq for ACCEL {}
pub struct ALTTABINFO {
    pub cbSize: u32,
    pub cItems: i32,
    pub cColumns: i32,
    pub cRows: i32,
    pub iColFocus: i32,
    pub iRowFocus: i32,
    pub cxItem: i32,
    pub cyItem: i32,
    pub ptStart: super::super::Foundation::POINT,
}
impl ::core::marker::Copy for ALTTABINFO {}
impl ::core::clone::Clone for ALTTABINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ALTTABINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ALTTABINFO")
            .field("cbSize", &self.cbSize)
            .field("cItems", &self.cItems)
            .field("cColumns", &self.cColumns)
            .field("cRows", &self.cRows)
            .field("iColFocus", &self.iColFocus)
            .field("iRowFocus", &self.iRowFocus)
            .field("cxItem", &self.cxItem)
            .field("cyItem", &self.cyItem)
            .field("ptStart", &self.ptStart)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ALTTABINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.cItems == other.cItems
            && self.cColumns == other.cColumns
            && self.cRows == other.cRows
            && self.iColFocus == other.iColFocus
            && self.iRowFocus == other.iRowFocus
            && self.cxItem == other.cxItem
            && self.cyItem == other.cyItem
            && self.ptStart == other.ptStart
    }
}
impl ::core::cmp::Eq for ALTTABINFO {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ANIMATE_WINDOW_FLAGS(pub u32);
pub const AW_ACTIVATE: ANIMATE_WINDOW_FLAGS = ANIMATE_WINDOW_FLAGS(131072u32);
pub const AW_BLEND: ANIMATE_WINDOW_FLAGS = ANIMATE_WINDOW_FLAGS(524288u32);
pub const AW_CENTER: ANIMATE_WINDOW_FLAGS = ANIMATE_WINDOW_FLAGS(16u32);
pub const AW_HIDE: ANIMATE_WINDOW_FLAGS = ANIMATE_WINDOW_FLAGS(65536u32);
pub const AW_HOR_POSITIVE: ANIMATE_WINDOW_FLAGS = ANIMATE_WINDOW_FLAGS(1u32);
pub const AW_HOR_NEGATIVE: ANIMATE_WINDOW_FLAGS = ANIMATE_WINDOW_FLAGS(2u32);
pub const AW_SLIDE: ANIMATE_WINDOW_FLAGS = ANIMATE_WINDOW_FLAGS(262144u32);
pub const AW_VER_POSITIVE: ANIMATE_WINDOW_FLAGS = ANIMATE_WINDOW_FLAGS(4u32);
pub const AW_VER_NEGATIVE: ANIMATE_WINDOW_FLAGS = ANIMATE_WINDOW_FLAGS(8u32);
impl ::core::marker::Copy for ANIMATE_WINDOW_FLAGS {}
impl ::core::clone::Clone for ANIMATE_WINDOW_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ANIMATE_WINDOW_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ANIMATE_WINDOW_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ANIMATE_WINDOW_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for ANIMATE_WINDOW_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ANIMATE_WINDOW_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ANIMATE_WINDOW_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ANIMATE_WINDOW_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ANIMATE_WINDOW_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub struct ANIMATIONINFO {
    pub cbSize: u32,
    pub iMinAnimate: i32,
}
impl ::core::marker::Copy for ANIMATIONINFO {}
impl ::core::clone::Clone for ANIMATIONINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ANIMATIONINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ANIMATIONINFO")
            .field("cbSize", &self.cbSize)
            .field("iMinAnimate", &self.iMinAnimate)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ANIMATIONINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.iMinAnimate == other.iMinAnimate
    }
}
impl ::core::cmp::Eq for ANIMATIONINFO {}
pub const ARW_DOWN: i32 = 4i32;
pub const ARW_HIDE: i32 = 8i32;
pub const ARW_LEFT: i32 = 0i32;
pub const ARW_RIGHT: i32 = 0i32;
pub const ARW_STARTMASK: i32 = 3i32;
pub const ARW_STARTRIGHT: i32 = 1i32;
pub const ARW_STARTTOP: i32 = 2i32;
pub const ARW_UP: i32 = 4i32;
pub const ASFW_ANY: u32 = 4294967295u32;
pub struct AUDIODESCRIPTION {
    pub cbSize: u32,
    pub Enabled: super::super::Foundation::BOOL,
    pub Locale: u32,
}
impl ::core::marker::Copy for AUDIODESCRIPTION {}
impl ::core::clone::Clone for AUDIODESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUDIODESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIODESCRIPTION")
            .field("cbSize", &self.cbSize)
            .field("Enabled", &self.Enabled)
            .field("Locale", &self.Locale)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AUDIODESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.Enabled == other.Enabled && self.Locale == other.Locale
    }
}
impl ::core::cmp::Eq for AUDIODESCRIPTION {}
pub const BM_CLICK: u32 = 245u32;
pub const BM_GETCHECK: u32 = 240u32;
pub const BM_GETIMAGE: u32 = 246u32;
pub const BM_GETSTATE: u32 = 242u32;
pub const BM_SETCHECK: u32 = 241u32;
pub const BM_SETDONTCLICK: u32 = 248u32;
pub const BM_SETIMAGE: u32 = 247u32;
pub const BM_SETSTATE: u32 = 243u32;
pub const BM_SETSTYLE: u32 = 244u32;
pub const BN_CLICKED: u32 = 0u32;
pub const BN_DBLCLK: u32 = 5u32;
pub const BN_DISABLE: u32 = 4u32;
pub const BN_DOUBLECLICKED: u32 = 5u32;
pub const BN_HILITE: u32 = 2u32;
pub const BN_KILLFOCUS: u32 = 7u32;
pub const BN_PAINT: u32 = 1u32;
pub const BN_PUSHED: u32 = 2u32;
pub const BN_SETFOCUS: u32 = 6u32;
pub const BN_UNHILITE: u32 = 3u32;
pub const BN_UNPUSHED: u32 = 3u32;
pub const BROADCAST_QUERY_DENY: u32 = 1112363332u32;
pub const BSM_INSTALLABLEDRIVERS: u32 = 4u32;
pub const BSM_NETDRIVER: u32 = 2u32;
pub const BSM_VXDS: u32 = 1u32;
pub const BST_FOCUS: u32 = 8u32;
pub const BST_PUSHED: u32 = 4u32;
pub const BS_3STATE: i32 = 5i32;
pub const BS_AUTO3STATE: i32 = 6i32;
pub const BS_AUTOCHECKBOX: i32 = 3i32;
pub const BS_AUTORADIOBUTTON: i32 = 9i32;
pub const BS_BITMAP: i32 = 128i32;
pub const BS_BOTTOM: i32 = 2048i32;
pub const BS_CENTER: i32 = 768i32;
pub const BS_CHECKBOX: i32 = 2i32;
pub const BS_DEFPUSHBUTTON: i32 = 1i32;
pub const BS_FLAT: i32 = 32768i32;
pub const BS_GROUPBOX: i32 = 7i32;
pub const BS_ICON: i32 = 64i32;
pub const BS_LEFT: i32 = 256i32;
pub const BS_LEFTTEXT: i32 = 32i32;
pub const BS_MULTILINE: i32 = 8192i32;
pub const BS_NOTIFY: i32 = 16384i32;
pub const BS_OWNERDRAW: i32 = 11i32;
pub const BS_PUSHBOX: i32 = 10i32;
pub const BS_PUSHBUTTON: i32 = 0i32;
pub const BS_PUSHLIKE: i32 = 4096i32;
pub const BS_RADIOBUTTON: i32 = 4i32;
pub const BS_RIGHT: i32 = 512i32;
pub const BS_RIGHTBUTTON: i32 = 32i32;
pub const BS_TEXT: i32 = 0i32;
pub const BS_TOP: i32 = 1024i32;
pub const BS_TYPEMASK: i32 = 15i32;
pub const BS_USERBUTTON: i32 = 8i32;
pub const BS_VCENTER: i32 = 3072i32;
pub const CALERT_SYSTEM: u32 = 6u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CASCADE_WINDOWS_HOW(pub u32);
pub const MDITILE_SKIPDISABLED: CASCADE_WINDOWS_HOW = CASCADE_WINDOWS_HOW(2u32);
pub const MDITILE_ZORDER: CASCADE_WINDOWS_HOW = CASCADE_WINDOWS_HOW(4u32);
impl ::core::marker::Copy for CASCADE_WINDOWS_HOW {}
impl ::core::clone::Clone for CASCADE_WINDOWS_HOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CASCADE_WINDOWS_HOW {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CASCADE_WINDOWS_HOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CASCADE_WINDOWS_HOW").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CASCADE_WINDOWS_HOW {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CASCADE_WINDOWS_HOW {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CASCADE_WINDOWS_HOW {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CASCADE_WINDOWS_HOW {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CASCADE_WINDOWS_HOW {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CBN_CLOSEUP: u32 = 8u32;
pub const CBN_DBLCLK: u32 = 2u32;
pub const CBN_DROPDOWN: u32 = 7u32;
pub const CBN_EDITCHANGE: u32 = 5u32;
pub const CBN_EDITUPDATE: u32 = 6u32;
pub const CBN_ERRSPACE: i32 = -1i32;
pub const CBN_KILLFOCUS: u32 = 4u32;
pub const CBN_SELCHANGE: u32 = 1u32;
pub const CBN_SELENDCANCEL: u32 = 10u32;
pub const CBN_SELENDOK: u32 = 9u32;
pub const CBN_SETFOCUS: u32 = 3u32;
pub const CBS_AUTOHSCROLL: i32 = 64i32;
pub const CBS_DISABLENOSCROLL: i32 = 2048i32;
pub const CBS_DROPDOWN: i32 = 2i32;
pub const CBS_DROPDOWNLIST: i32 = 3i32;
pub const CBS_HASSTRINGS: i32 = 512i32;
pub const CBS_LOWERCASE: i32 = 16384i32;
pub const CBS_NOINTEGRALHEIGHT: i32 = 1024i32;
pub const CBS_OEMCONVERT: i32 = 128i32;
pub const CBS_OWNERDRAWFIXED: i32 = 16i32;
pub const CBS_OWNERDRAWVARIABLE: i32 = 32i32;
pub const CBS_SIMPLE: i32 = 1i32;
pub const CBS_SORT: i32 = 256i32;
pub const CBS_UPPERCASE: i32 = 8192i32;
pub struct CBTACTIVATESTRUCT {
    pub fMouse: super::super::Foundation::BOOL,
    pub hWndActive: super::super::Foundation::HWND,
}
impl ::core::marker::Copy for CBTACTIVATESTRUCT {}
impl ::core::clone::Clone for CBTACTIVATESTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CBTACTIVATESTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CBTACTIVATESTRUCT")
            .field("fMouse", &self.fMouse)
            .field("hWndActive", &self.hWndActive)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CBTACTIVATESTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.fMouse == other.fMouse && self.hWndActive == other.hWndActive
    }
}
impl ::core::cmp::Eq for CBTACTIVATESTRUCT {}
pub struct CBT_CREATEWNDA {
    pub lpcs: MutPtr<CREATESTRUCTA>,
    pub hwndInsertAfter: super::super::Foundation::HWND,
}
impl ::core::marker::Copy for CBT_CREATEWNDA {}
impl ::core::clone::Clone for CBT_CREATEWNDA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CBT_CREATEWNDA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CBT_CREATEWNDA")
            .field("lpcs", &self.lpcs)
            .field("hwndInsertAfter", &self.hwndInsertAfter)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CBT_CREATEWNDA {
    fn eq(&self, other: &Self) -> bool {
        self.lpcs == other.lpcs && self.hwndInsertAfter == other.hwndInsertAfter
    }
}
impl ::core::cmp::Eq for CBT_CREATEWNDA {}
pub struct CBT_CREATEWNDW {
    pub lpcs: MutPtr<CREATESTRUCTW>,
    pub hwndInsertAfter: super::super::Foundation::HWND,
}
impl ::core::marker::Copy for CBT_CREATEWNDW {}
impl ::core::clone::Clone for CBT_CREATEWNDW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CBT_CREATEWNDW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CBT_CREATEWNDW")
            .field("lpcs", &self.lpcs)
            .field("hwndInsertAfter", &self.hwndInsertAfter)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CBT_CREATEWNDW {
    fn eq(&self, other: &Self) -> bool {
        self.lpcs == other.lpcs && self.hwndInsertAfter == other.hwndInsertAfter
    }
}
impl ::core::cmp::Eq for CBT_CREATEWNDW {}
pub const CB_ADDSTRING: u32 = 323u32;
pub const CB_DELETESTRING: u32 = 324u32;
pub const CB_DIR: u32 = 325u32;
pub const CB_ERR: i32 = -1i32;
pub const CB_ERRSPACE: i32 = -2i32;
pub const CB_FINDSTRING: u32 = 332u32;
pub const CB_FINDSTRINGEXACT: u32 = 344u32;
pub const CB_GETCOMBOBOXINFO: u32 = 356u32;
pub const CB_GETCOUNT: u32 = 326u32;
pub const CB_GETCURSEL: u32 = 327u32;
pub const CB_GETDROPPEDCONTROLRECT: u32 = 338u32;
pub const CB_GETDROPPEDSTATE: u32 = 343u32;
pub const CB_GETDROPPEDWIDTH: u32 = 351u32;
pub const CB_GETEDITSEL: u32 = 320u32;
pub const CB_GETEXTENDEDUI: u32 = 342u32;
pub const CB_GETHORIZONTALEXTENT: u32 = 349u32;
pub const CB_GETITEMDATA: u32 = 336u32;
pub const CB_GETITEMHEIGHT: u32 = 340u32;
pub const CB_GETLBTEXT: u32 = 328u32;
pub const CB_GETLBTEXTLEN: u32 = 329u32;
pub const CB_GETLOCALE: u32 = 346u32;
pub const CB_GETTOPINDEX: u32 = 347u32;
pub const CB_INITSTORAGE: u32 = 353u32;
pub const CB_INSERTSTRING: u32 = 330u32;
pub const CB_LIMITTEXT: u32 = 321u32;
pub const CB_MSGMAX: u32 = 357u32;
pub const CB_MULTIPLEADDSTRING: u32 = 355u32;
pub const CB_OKAY: u32 = 0u32;
pub const CB_RESETCONTENT: u32 = 331u32;
pub const CB_SELECTSTRING: u32 = 333u32;
pub const CB_SETCURSEL: u32 = 334u32;
pub const CB_SETDROPPEDWIDTH: u32 = 352u32;
pub const CB_SETEDITSEL: u32 = 322u32;
pub const CB_SETEXTENDEDUI: u32 = 341u32;
pub const CB_SETHORIZONTALEXTENT: u32 = 350u32;
pub const CB_SETITEMDATA: u32 = 337u32;
pub const CB_SETITEMHEIGHT: u32 = 339u32;
pub const CB_SETLOCALE: u32 = 345u32;
pub const CB_SETTOPINDEX: u32 = 348u32;
pub const CB_SHOWDROPDOWN: u32 = 335u32;
pub const CCHILDREN_SCROLLBAR: u32 = 5u32;
pub const CCHILDREN_TITLEBAR: u32 = 5u32;
pub struct CHANGEFILTERSTRUCT {
    pub cbSize: u32,
    pub ExtStatus: MSGFLTINFO_STATUS,
}
impl ::core::marker::Copy for CHANGEFILTERSTRUCT {}
impl ::core::clone::Clone for CHANGEFILTERSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CHANGEFILTERSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGEFILTERSTRUCT")
            .field("cbSize", &self.cbSize)
            .field("ExtStatus", &self.ExtStatus)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CHANGEFILTERSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.ExtStatus == other.ExtStatus
    }
}
impl ::core::cmp::Eq for CHANGEFILTERSTRUCT {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CHANGE_WINDOW_MESSAGE_FILTER_FLAGS(pub u32);
pub const MSGFLT_ADD: CHANGE_WINDOW_MESSAGE_FILTER_FLAGS = CHANGE_WINDOW_MESSAGE_FILTER_FLAGS(1u32);
pub const MSGFLT_REMOVE: CHANGE_WINDOW_MESSAGE_FILTER_FLAGS =
    CHANGE_WINDOW_MESSAGE_FILTER_FLAGS(2u32);
impl ::core::marker::Copy for CHANGE_WINDOW_MESSAGE_FILTER_FLAGS {}
impl ::core::clone::Clone for CHANGE_WINDOW_MESSAGE_FILTER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CHANGE_WINDOW_MESSAGE_FILTER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CHANGE_WINDOW_MESSAGE_FILTER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHANGE_WINDOW_MESSAGE_FILTER_FLAGS")
            .field(&self.0)
            .finish()
    }
}
pub const CHILDID_SELF: u32 = 0u32;
pub struct CLIENTCREATESTRUCT {
    pub hWindowMenu: super::super::Foundation::HANDLE,
    pub idFirstChild: u32,
}
impl ::core::marker::Copy for CLIENTCREATESTRUCT {}
impl ::core::clone::Clone for CLIENTCREATESTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLIENTCREATESTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLIENTCREATESTRUCT")
            .field("hWindowMenu", &self.hWindowMenu)
            .field("idFirstChild", &self.idFirstChild)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLIENTCREATESTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.hWindowMenu == other.hWindowMenu && self.idFirstChild == other.idFirstChild
    }
}
impl ::core::cmp::Eq for CLIENTCREATESTRUCT {}
pub const COLOR_BTNHIGHLIGHT: u32 = 20u32;
pub const COLOR_BTNHILIGHT: u32 = 20u32;
pub const CONSOLE_APPLICATION_16BIT: u32 = 0u32;
pub const CONSOLE_CARET_SELECTION: u32 = 1u32;
pub const CONSOLE_CARET_VISIBLE: u32 = 2u32;
pub const CONTACTVISUALIZATION_OFF: u32 = 0u32;
pub const CONTACTVISUALIZATION_ON: u32 = 1u32;
pub const CONTACTVISUALIZATION_PRESENTATIONMODE: u32 = 2u32;
pub const CREATEPROCESS_MANIFEST_RESOURCE_ID: u32 = 1u32;
pub struct CREATESTRUCTA {
    pub lpCreateParams: MutPtr<::core::ffi::c_void>,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub hMenu: HMENU,
    pub hwndParent: super::super::Foundation::HWND,
    pub cy: i32,
    pub cx: i32,
    pub y: i32,
    pub x: i32,
    pub style: i32,
    pub lpszName: ::win32::core::PCSTR,
    pub lpszClass: ::win32::core::PCSTR,
    pub dwExStyle: u32,
}
impl ::core::marker::Copy for CREATESTRUCTA {}
impl ::core::clone::Clone for CREATESTRUCTA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CREATESTRUCTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREATESTRUCTA")
            .field("lpCreateParams", &self.lpCreateParams)
            .field("hInstance", &self.hInstance)
            .field("hMenu", &self.hMenu)
            .field("hwndParent", &self.hwndParent)
            .field("cy", &self.cy)
            .field("cx", &self.cx)
            .field("y", &self.y)
            .field("x", &self.x)
            .field("style", &self.style)
            .field("lpszName", &self.lpszName)
            .field("lpszClass", &self.lpszClass)
            .field("dwExStyle", &self.dwExStyle)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CREATESTRUCTA {
    fn eq(&self, other: &Self) -> bool {
        self.lpCreateParams == other.lpCreateParams
            && self.hInstance == other.hInstance
            && self.hMenu == other.hMenu
            && self.hwndParent == other.hwndParent
            && self.cy == other.cy
            && self.cx == other.cx
            && self.y == other.y
            && self.x == other.x
            && self.style == other.style
            && self.lpszName == other.lpszName
            && self.lpszClass == other.lpszClass
            && self.dwExStyle == other.dwExStyle
    }
}
impl ::core::cmp::Eq for CREATESTRUCTA {}
pub struct CREATESTRUCTW {
    pub lpCreateParams: MutPtr<::core::ffi::c_void>,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub hMenu: HMENU,
    pub hwndParent: super::super::Foundation::HWND,
    pub cy: i32,
    pub cx: i32,
    pub y: i32,
    pub x: i32,
    pub style: i32,
    pub lpszName: ::win32::core::PCWSTR,
    pub lpszClass: ::win32::core::PCWSTR,
    pub dwExStyle: u32,
}
impl ::core::marker::Copy for CREATESTRUCTW {}
impl ::core::clone::Clone for CREATESTRUCTW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CREATESTRUCTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREATESTRUCTW")
            .field("lpCreateParams", &self.lpCreateParams)
            .field("hInstance", &self.hInstance)
            .field("hMenu", &self.hMenu)
            .field("hwndParent", &self.hwndParent)
            .field("cy", &self.cy)
            .field("cx", &self.cx)
            .field("y", &self.y)
            .field("x", &self.x)
            .field("style", &self.style)
            .field("lpszName", &self.lpszName)
            .field("lpszClass", &self.lpszClass)
            .field("dwExStyle", &self.dwExStyle)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CREATESTRUCTW {
    fn eq(&self, other: &Self) -> bool {
        self.lpCreateParams == other.lpCreateParams
            && self.hInstance == other.hInstance
            && self.hMenu == other.hMenu
            && self.hwndParent == other.hwndParent
            && self.cy == other.cy
            && self.cx == other.cx
            && self.y == other.y
            && self.x == other.x
            && self.style == other.style
            && self.lpszName == other.lpszName
            && self.lpszClass == other.lpszClass
            && self.dwExStyle == other.dwExStyle
    }
}
impl ::core::cmp::Eq for CREATESTRUCTW {}
pub const CSOUND_SYSTEM: u32 = 16u32;
pub const CTLCOLOR_BTN: u32 = 3u32;
pub const CTLCOLOR_DLG: u32 = 4u32;
pub const CTLCOLOR_EDIT: u32 = 1u32;
pub const CTLCOLOR_LISTBOX: u32 = 2u32;
pub const CTLCOLOR_MAX: u32 = 7u32;
pub const CTLCOLOR_MSGBOX: u32 = 0u32;
pub const CTLCOLOR_SCROLLBAR: u32 = 5u32;
pub const CTLCOLOR_STATIC: u32 = 6u32;
pub struct CURSORINFO {
    pub cbSize: u32,
    pub flags: CURSORINFO_FLAGS,
    pub hCursor: HCURSOR,
    pub ptScreenPos: super::super::Foundation::POINT,
}
impl ::core::marker::Copy for CURSORINFO {}
impl ::core::clone::Clone for CURSORINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CURSORINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CURSORINFO")
            .field("cbSize", &self.cbSize)
            .field("flags", &self.flags)
            .field("hCursor", &self.hCursor)
            .field("ptScreenPos", &self.ptScreenPos)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CURSORINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.flags == other.flags
            && self.hCursor == other.hCursor
            && self.ptScreenPos == other.ptScreenPos
    }
}
impl ::core::cmp::Eq for CURSORINFO {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CURSORINFO_FLAGS(pub u32);
pub const CURSOR_SHOWING: CURSORINFO_FLAGS = CURSORINFO_FLAGS(1u32);
pub const CURSOR_SUPPRESSED: CURSORINFO_FLAGS = CURSORINFO_FLAGS(2u32);
impl ::core::marker::Copy for CURSORINFO_FLAGS {}
impl ::core::clone::Clone for CURSORINFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CURSORINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CURSORINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CURSORINFO_FLAGS").field(&self.0).finish()
    }
}
pub struct CURSORSHAPE {
    pub xHotSpot: i32,
    pub yHotSpot: i32,
    pub cx: i32,
    pub cy: i32,
    pub cbWidth: i32,
    pub Planes: u8,
    pub BitsPixel: u8,
}
impl ::core::marker::Copy for CURSORSHAPE {}
impl ::core::clone::Clone for CURSORSHAPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CURSORSHAPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CURSORSHAPE")
            .field("xHotSpot", &self.xHotSpot)
            .field("yHotSpot", &self.yHotSpot)
            .field("cx", &self.cx)
            .field("cy", &self.cy)
            .field("cbWidth", &self.cbWidth)
            .field("Planes", &self.Planes)
            .field("BitsPixel", &self.BitsPixel)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CURSORSHAPE {
    fn eq(&self, other: &Self) -> bool {
        self.xHotSpot == other.xHotSpot
            && self.yHotSpot == other.yHotSpot
            && self.cx == other.cx
            && self.cy == other.cy
            && self.cbWidth == other.cbWidth
            && self.Planes == other.Planes
            && self.BitsPixel == other.BitsPixel
    }
}
impl ::core::cmp::Eq for CURSORSHAPE {}
pub const CURSOR_CREATION_SCALING_DEFAULT: u32 = 2u32;
pub const CURSOR_CREATION_SCALING_NONE: u32 = 1u32;
pub const CWF_CREATE_ONLY: u32 = 1u32;
pub struct CWPRETSTRUCT {
    pub lResult: super::super::Foundation::LRESULT,
    pub lParam: super::super::Foundation::LPARAM,
    pub wParam: super::super::Foundation::WPARAM,
    pub message: u32,
    pub hwnd: super::super::Foundation::HWND,
}
impl ::core::marker::Copy for CWPRETSTRUCT {}
impl ::core::clone::Clone for CWPRETSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CWPRETSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CWPRETSTRUCT")
            .field("lResult", &self.lResult)
            .field("lParam", &self.lParam)
            .field("wParam", &self.wParam)
            .field("message", &self.message)
            .field("hwnd", &self.hwnd)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CWPRETSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.lResult == other.lResult
            && self.lParam == other.lParam
            && self.wParam == other.wParam
            && self.message == other.message
            && self.hwnd == other.hwnd
    }
}
impl ::core::cmp::Eq for CWPRETSTRUCT {}
pub struct CWPSTRUCT {
    pub lParam: super::super::Foundation::LPARAM,
    pub wParam: super::super::Foundation::WPARAM,
    pub message: u32,
    pub hwnd: super::super::Foundation::HWND,
}
impl ::core::marker::Copy for CWPSTRUCT {}
impl ::core::clone::Clone for CWPSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CWPSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CWPSTRUCT")
            .field("lParam", &self.lParam)
            .field("wParam", &self.wParam)
            .field("message", &self.message)
            .field("hwnd", &self.hwnd)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CWPSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.lParam == other.lParam
            && self.wParam == other.wParam
            && self.message == other.message
            && self.hwnd == other.hwnd
    }
}
impl ::core::cmp::Eq for CWPSTRUCT {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CWP_FLAGS(pub u32);
pub const CWP_ALL: CWP_FLAGS = CWP_FLAGS(0u32);
pub const CWP_SKIPINVISIBLE: CWP_FLAGS = CWP_FLAGS(1u32);
pub const CWP_SKIPDISABLED: CWP_FLAGS = CWP_FLAGS(2u32);
pub const CWP_SKIPTRANSPARENT: CWP_FLAGS = CWP_FLAGS(4u32);
impl ::core::marker::Copy for CWP_FLAGS {}
impl ::core::clone::Clone for CWP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CWP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CWP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CWP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CWP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CWP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CWP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CWP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CWP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CW_USEDEFAULT: i32 = -2147483648i32;
pub const DCX_EXCLUDEUPDATE: i32 = 256i32;
pub const DC_HASDEFID: u32 = 21323u32;
pub struct DEBUGHOOKINFO {
    pub idThread: u32,
    pub idThreadInstaller: u32,
    pub lParam: super::super::Foundation::LPARAM,
    pub wParam: super::super::Foundation::WPARAM,
    pub code: i32,
}
impl ::core::marker::Copy for DEBUGHOOKINFO {}
impl ::core::clone::Clone for DEBUGHOOKINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DEBUGHOOKINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUGHOOKINFO")
            .field("idThread", &self.idThread)
            .field("idThreadInstaller", &self.idThreadInstaller)
            .field("lParam", &self.lParam)
            .field("wParam", &self.wParam)
            .field("code", &self.code)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DEBUGHOOKINFO {
    fn eq(&self, other: &Self) -> bool {
        self.idThread == other.idThread
            && self.idThreadInstaller == other.idThreadInstaller
            && self.lParam == other.lParam
            && self.wParam == other.wParam
            && self.code == other.code
    }
}
impl ::core::cmp::Eq for DEBUGHOOKINFO {}
pub const DESKTOP_CREATEMENU: i32 = 4i32;
pub const DESKTOP_CREATEWINDOW: i32 = 2i32;
pub const DESKTOP_ENUMERATE: i32 = 64i32;
pub const DESKTOP_HOOKCONTROL: i32 = 8i32;
pub const DESKTOP_JOURNALPLAYBACK: i32 = 32i32;
pub const DESKTOP_JOURNALRECORD: i32 = 16i32;
pub const DESKTOP_READOBJECTS: i32 = 1i32;
pub const DESKTOP_SWITCHDESKTOP: i32 = 256i32;
pub const DESKTOP_WRITEOBJECTS: i32 = 128i32;
pub const DEVICE_NOTIFY_ALL_INTERFACE_CLASSES: u32 = 4u32;
pub const DF_ALLOWOTHERACCOUNTHOOK: i32 = 1i32;
pub const DIFFERENCE: u32 = 11u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DI_FLAGS(pub u32);
pub const DI_MASK: DI_FLAGS = DI_FLAGS(1u32);
pub const DI_IMAGE: DI_FLAGS = DI_FLAGS(2u32);
pub const DI_NORMAL: DI_FLAGS = DI_FLAGS(3u32);
pub const DI_COMPAT: DI_FLAGS = DI_FLAGS(4u32);
pub const DI_DEFAULTSIZE: DI_FLAGS = DI_FLAGS(8u32);
pub const DI_NOMIRROR: DI_FLAGS = DI_FLAGS(16u32);
impl ::core::marker::Copy for DI_FLAGS {}
impl ::core::clone::Clone for DI_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DI_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DI_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DI_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DI_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DI_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DI_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DI_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DI_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DLGC_BUTTON: u32 = 8192u32;
pub const DLGC_DEFPUSHBUTTON: u32 = 16u32;
pub const DLGC_HASSETSEL: u32 = 8u32;
pub const DLGC_RADIOBUTTON: u32 = 64u32;
pub const DLGC_STATIC: u32 = 256u32;
pub const DLGC_UNDEFPUSHBUTTON: u32 = 32u32;
pub const DLGC_WANTALLKEYS: u32 = 4u32;
pub const DLGC_WANTARROWS: u32 = 1u32;
pub const DLGC_WANTCHARS: u32 = 128u32;
pub const DLGC_WANTMESSAGE: u32 = 4u32;
pub const DLGC_WANTTAB: u32 = 2u32;
pub struct DLGITEMTEMPLATE {
    pub style: u32,
    pub dwExtendedStyle: u32,
    pub x: i16,
    pub y: i16,
    pub cx: i16,
    pub cy: i16,
    pub id: u16,
}
impl ::core::marker::Copy for DLGITEMTEMPLATE {}
impl ::core::clone::Clone for DLGITEMTEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for DLGITEMTEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.style == other.style
            && self.dwExtendedStyle == other.dwExtendedStyle
            && self.x == other.x
            && self.y == other.y
            && self.cx == other.cx
            && self.cy == other.cy
            && self.id == other.id
    }
}
impl ::core::cmp::Eq for DLGITEMTEMPLATE {}
pub type DLGPROC = ::core::option::Option<
    unsafe extern "system" fn(
        param0: super::super::Foundation::HWND,
        param1: u32,
        param2: super::super::Foundation::WPARAM,
        param3: super::super::Foundation::LPARAM,
    ) -> isize,
>;
pub struct DLGTEMPLATE {
    pub style: u32,
    pub dwExtendedStyle: u32,
    pub cdit: u16,
    pub x: i16,
    pub y: i16,
    pub cx: i16,
    pub cy: i16,
}
impl ::core::marker::Copy for DLGTEMPLATE {}
impl ::core::clone::Clone for DLGTEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for DLGTEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.style == other.style
            && self.dwExtendedStyle == other.dwExtendedStyle
            && self.cdit == other.cdit
            && self.x == other.x
            && self.y == other.y
            && self.cx == other.cx
            && self.cy == other.cy
    }
}
impl ::core::cmp::Eq for DLGTEMPLATE {}
pub const DLGWINDOWEXTRA: u32 = 30u32;
pub const DM_GETDEFID: u32 = 1024u32;
pub const DM_POINTERHITTEST: u32 = 592u32;
pub const DM_REPOSITION: u32 = 1026u32;
pub const DM_SETDEFID: u32 = 1025u32;
pub const DOF_DIRECTORY: u32 = 32771u32;
pub const DOF_DOCUMENT: u32 = 32770u32;
pub const DOF_EXECUTABLE: u32 = 32769u32;
pub const DOF_MULTIPLE: u32 = 32772u32;
pub const DOF_PROGMAN: u32 = 1u32;
pub const DOF_SHELLDATA: u32 = 2u32;
pub const DO_DROPFILE: i32 = 1162627398i32;
pub const DO_PRINTFILE: i32 = 1414419024i32;
pub struct DROPSTRUCT {
    pub hwndSource: super::super::Foundation::HWND,
    pub hwndSink: super::super::Foundation::HWND,
    pub wFmt: u32,
    pub dwData: usize,
    pub ptDrop: super::super::Foundation::POINT,
    pub dwControlData: u32,
}
impl ::core::marker::Copy for DROPSTRUCT {}
impl ::core::clone::Clone for DROPSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DROPSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DROPSTRUCT")
            .field("hwndSource", &self.hwndSource)
            .field("hwndSink", &self.hwndSink)
            .field("wFmt", &self.wFmt)
            .field("dwData", &self.dwData)
            .field("ptDrop", &self.ptDrop)
            .field("dwControlData", &self.dwControlData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DROPSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.hwndSource == other.hwndSource
            && self.hwndSink == other.hwndSink
            && self.wFmt == other.wFmt
            && self.dwData == other.dwData
            && self.ptDrop == other.ptDrop
            && self.dwControlData == other.dwControlData
    }
}
impl ::core::cmp::Eq for DROPSTRUCT {}
pub const DS_3DLOOK: i32 = 4i32;
pub const DS_ABSALIGN: i32 = 1i32;
pub const DS_CENTER: i32 = 2048i32;
pub const DS_CENTERMOUSE: i32 = 4096i32;
pub const DS_CONTEXTHELP: i32 = 8192i32;
pub const DS_CONTROL: i32 = 1024i32;
pub const DS_FIXEDSYS: i32 = 8i32;
pub const DS_LOCALEDIT: i32 = 32i32;
pub const DS_MODALFRAME: i32 = 128i32;
pub const DS_NOFAILCREATE: i32 = 16i32;
pub const DS_NOIDLEMSG: i32 = 256i32;
pub const DS_SETFONT: i32 = 64i32;
pub const DS_SETFOREGROUND: i32 = 512i32;
pub const DS_SYSMODAL: i32 = 2i32;
pub const DS_USEPIXELS: i32 = 32768i32;
pub const DWLP_MSGRESULT: u32 = 0u32;
pub const DWL_DLGPROC: u32 = 4u32;
pub const DWL_MSGRESULT: u32 = 0u32;
pub const DWL_USER: u32 = 8u32;
pub const EC_LEFTMARGIN: u32 = 1u32;
pub const EC_RIGHTMARGIN: u32 = 2u32;
pub const EC_USEFONTINFO: u32 = 65535u32;
pub const EDD_GET_DEVICE_INTERFACE_NAME: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EDIT_CONTROL_FEATURE(pub i32);
pub const EDIT_CONTROL_FEATURE_ENTERPRISE_DATA_PROTECTION_PASTE_SUPPORT: EDIT_CONTROL_FEATURE =
    EDIT_CONTROL_FEATURE(0i32);
pub const EDIT_CONTROL_FEATURE_PASTE_NOTIFICATIONS: EDIT_CONTROL_FEATURE =
    EDIT_CONTROL_FEATURE(1i32);
impl ::core::marker::Copy for EDIT_CONTROL_FEATURE {}
impl ::core::clone::Clone for EDIT_CONTROL_FEATURE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EDIT_CONTROL_FEATURE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EDIT_CONTROL_FEATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EDIT_CONTROL_FEATURE")
            .field(&self.0)
            .finish()
    }
}
pub const EDS_RAWMODE: u32 = 2u32;
pub const EDS_ROTATEDMODE: u32 = 4u32;
pub const EIMES_CANCELCOMPSTRINFOCUS: u32 = 2u32;
pub const EIMES_COMPLETECOMPSTRKILLFOCUS: u32 = 4u32;
pub const EIMES_GETCOMPSTRATONCE: u32 = 1u32;
pub const EMSIS_COMPOSITIONSTRING: u32 = 1u32;
pub const ENDSESSION_CLOSEAPP: u32 = 1u32;
pub const ENDSESSION_CRITICAL: u32 = 1073741824u32;
pub const ENDSESSION_LOGOFF: u32 = 2147483648u32;
pub const EN_AFTER_PASTE: u32 = 2049u32;
pub const EN_ALIGN_LTR_EC: u32 = 1792u32;
pub const EN_ALIGN_RTL_EC: u32 = 1793u32;
pub const EN_BEFORE_PASTE: u32 = 2048u32;
pub const EN_CHANGE: u32 = 768u32;
pub const EN_ERRSPACE: u32 = 1280u32;
pub const EN_HSCROLL: u32 = 1537u32;
pub const EN_KILLFOCUS: u32 = 512u32;
pub const EN_MAXTEXT: u32 = 1281u32;
pub const EN_SETFOCUS: u32 = 256u32;
pub const EN_UPDATE: u32 = 1024u32;
pub const EN_VSCROLL: u32 = 1538u32;
pub const ES_AUTOHSCROLL: i32 = 128i32;
pub const ES_AUTOVSCROLL: i32 = 64i32;
pub const ES_CENTER: i32 = 1i32;
pub const ES_LEFT: i32 = 0i32;
pub const ES_LOWERCASE: i32 = 16i32;
pub const ES_MULTILINE: i32 = 4i32;
pub const ES_NOHIDESEL: i32 = 256i32;
pub const ES_NUMBER: i32 = 8192i32;
pub const ES_OEMCONVERT: i32 = 1024i32;
pub const ES_PASSWORD: i32 = 32i32;
pub const ES_READONLY: i32 = 2048i32;
pub const ES_RIGHT: i32 = 2i32;
pub const ES_UPPERCASE: i32 = 8i32;
pub const ES_WANTRETURN: i32 = 4096i32;
pub struct EVENTMSG {
    pub message: u32,
    pub paramL: u32,
    pub paramH: u32,
    pub time: u32,
    pub hwnd: super::super::Foundation::HWND,
}
impl ::core::marker::Copy for EVENTMSG {}
impl ::core::clone::Clone for EVENTMSG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EVENTMSG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENTMSG")
            .field("message", &self.message)
            .field("paramL", &self.paramL)
            .field("paramH", &self.paramH)
            .field("time", &self.time)
            .field("hwnd", &self.hwnd)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EVENTMSG {
    fn eq(&self, other: &Self) -> bool {
        self.message == other.message
            && self.paramL == other.paramL
            && self.paramH == other.paramH
            && self.time == other.time
            && self.hwnd == other.hwnd
    }
}
impl ::core::cmp::Eq for EVENTMSG {}
pub const EVENT_AIA_END: u32 = 45055u32;
pub const EVENT_AIA_START: u32 = 40960u32;
pub const EVENT_CONSOLE_CARET: u32 = 16385u32;
pub const EVENT_CONSOLE_END: u32 = 16639u32;
pub const EVENT_CONSOLE_END_APPLICATION: u32 = 16391u32;
pub const EVENT_CONSOLE_LAYOUT: u32 = 16389u32;
pub const EVENT_CONSOLE_START_APPLICATION: u32 = 16390u32;
pub const EVENT_CONSOLE_UPDATE_REGION: u32 = 16386u32;
pub const EVENT_CONSOLE_UPDATE_SCROLL: u32 = 16388u32;
pub const EVENT_CONSOLE_UPDATE_SIMPLE: u32 = 16387u32;
pub const EVENT_MAX: u32 = 2147483647u32;
pub const EVENT_MIN: u32 = 1u32;
pub const EVENT_OBJECT_ACCELERATORCHANGE: u32 = 32786u32;
pub const EVENT_OBJECT_CLOAKED: u32 = 32791u32;
pub const EVENT_OBJECT_CONTENTSCROLLED: u32 = 32789u32;
pub const EVENT_OBJECT_CREATE: u32 = 32768u32;
pub const EVENT_OBJECT_DEFACTIONCHANGE: u32 = 32785u32;
pub const EVENT_OBJECT_DESCRIPTIONCHANGE: u32 = 32781u32;
pub const EVENT_OBJECT_DESTROY: u32 = 32769u32;
pub const EVENT_OBJECT_DRAGCANCEL: u32 = 32802u32;
pub const EVENT_OBJECT_DRAGCOMPLETE: u32 = 32803u32;
pub const EVENT_OBJECT_DRAGDROPPED: u32 = 32806u32;
pub const EVENT_OBJECT_DRAGENTER: u32 = 32804u32;
pub const EVENT_OBJECT_DRAGLEAVE: u32 = 32805u32;
pub const EVENT_OBJECT_DRAGSTART: u32 = 32801u32;
pub const EVENT_OBJECT_END: u32 = 33023u32;
pub const EVENT_OBJECT_FOCUS: u32 = 32773u32;
pub const EVENT_OBJECT_HELPCHANGE: u32 = 32784u32;
pub const EVENT_OBJECT_HIDE: u32 = 32771u32;
pub const EVENT_OBJECT_HOSTEDOBJECTSINVALIDATED: u32 = 32800u32;
pub const EVENT_OBJECT_IME_CHANGE: u32 = 32809u32;
pub const EVENT_OBJECT_IME_HIDE: u32 = 32808u32;
pub const EVENT_OBJECT_IME_SHOW: u32 = 32807u32;
pub const EVENT_OBJECT_INVOKED: u32 = 32787u32;
pub const EVENT_OBJECT_LIVEREGIONCHANGED: u32 = 32793u32;
pub const EVENT_OBJECT_LOCATIONCHANGE: u32 = 32779u32;
pub const EVENT_OBJECT_NAMECHANGE: u32 = 32780u32;
pub const EVENT_OBJECT_PARENTCHANGE: u32 = 32783u32;
pub const EVENT_OBJECT_REORDER: u32 = 32772u32;
pub const EVENT_OBJECT_SELECTION: u32 = 32774u32;
pub const EVENT_OBJECT_SELECTIONADD: u32 = 32775u32;
pub const EVENT_OBJECT_SELECTIONREMOVE: u32 = 32776u32;
pub const EVENT_OBJECT_SELECTIONWITHIN: u32 = 32777u32;
pub const EVENT_OBJECT_SHOW: u32 = 32770u32;
pub const EVENT_OBJECT_STATECHANGE: u32 = 32778u32;
pub const EVENT_OBJECT_TEXTEDIT_CONVERSIONTARGETCHANGED: u32 = 32816u32;
pub const EVENT_OBJECT_TEXTSELECTIONCHANGED: u32 = 32788u32;
pub const EVENT_OBJECT_UNCLOAKED: u32 = 32792u32;
pub const EVENT_OBJECT_VALUECHANGE: u32 = 32782u32;
pub const EVENT_OEM_DEFINED_END: u32 = 511u32;
pub const EVENT_OEM_DEFINED_START: u32 = 257u32;
pub const EVENT_SYSTEM_ALERT: u32 = 2u32;
pub const EVENT_SYSTEM_ARRANGMENTPREVIEW: u32 = 32790u32;
pub const EVENT_SYSTEM_CAPTUREEND: u32 = 9u32;
pub const EVENT_SYSTEM_CAPTURESTART: u32 = 8u32;
pub const EVENT_SYSTEM_CONTEXTHELPEND: u32 = 13u32;
pub const EVENT_SYSTEM_CONTEXTHELPSTART: u32 = 12u32;
pub const EVENT_SYSTEM_DESKTOPSWITCH: u32 = 32u32;
pub const EVENT_SYSTEM_DIALOGEND: u32 = 17u32;
pub const EVENT_SYSTEM_DIALOGSTART: u32 = 16u32;
pub const EVENT_SYSTEM_DRAGDROPEND: u32 = 15u32;
pub const EVENT_SYSTEM_DRAGDROPSTART: u32 = 14u32;
pub const EVENT_SYSTEM_END: u32 = 255u32;
pub const EVENT_SYSTEM_FOREGROUND: u32 = 3u32;
pub const EVENT_SYSTEM_IME_KEY_NOTIFICATION: u32 = 41u32;
pub const EVENT_SYSTEM_MENUEND: u32 = 5u32;
pub const EVENT_SYSTEM_MENUPOPUPEND: u32 = 7u32;
pub const EVENT_SYSTEM_MENUPOPUPSTART: u32 = 6u32;
pub const EVENT_SYSTEM_MENUSTART: u32 = 4u32;
pub const EVENT_SYSTEM_MINIMIZEEND: u32 = 23u32;
pub const EVENT_SYSTEM_MINIMIZESTART: u32 = 22u32;
pub const EVENT_SYSTEM_MOVESIZEEND: u32 = 11u32;
pub const EVENT_SYSTEM_MOVESIZESTART: u32 = 10u32;
pub const EVENT_SYSTEM_SCROLLINGEND: u32 = 19u32;
pub const EVENT_SYSTEM_SCROLLINGSTART: u32 = 18u32;
pub const EVENT_SYSTEM_SOUND: u32 = 1u32;
pub const EVENT_SYSTEM_SWITCHEND: u32 = 21u32;
pub const EVENT_SYSTEM_SWITCHER_APPDROPPED: u32 = 38u32;
pub const EVENT_SYSTEM_SWITCHER_APPGRABBED: u32 = 36u32;
pub const EVENT_SYSTEM_SWITCHER_APPOVERTARGET: u32 = 37u32;
pub const EVENT_SYSTEM_SWITCHER_CANCELLED: u32 = 39u32;
pub const EVENT_SYSTEM_SWITCHSTART: u32 = 20u32;
pub const EVENT_UIA_EVENTID_END: u32 = 20223u32;
pub const EVENT_UIA_EVENTID_START: u32 = 19968u32;
pub const EVENT_UIA_PROPID_END: u32 = 30207u32;
pub const EVENT_UIA_PROPID_START: u32 = 29952u32;
pub const EWX_ARSO: u32 = 67108864u32;
pub const EWX_BOOTOPTIONS: u32 = 16777216u32;
pub const EWX_CHECK_SAFE_FOR_SERVER: u32 = 134217728u32;
pub const EWX_FORCE: u32 = 4u32;
pub const EWX_FORCEIFHUNG: u32 = 16u32;
pub const EWX_QUICKRESOLVE: u32 = 32u32;
pub const EWX_SYSTEM_INITIATED: u32 = 268435456u32;
pub const FALT: u32 = 16u32;
pub const FAPPCOMMAND_KEY: u32 = 0u32;
pub const FAPPCOMMAND_MASK: u32 = 61440u32;
pub const FAPPCOMMAND_MOUSE: u32 = 32768u32;
pub const FAPPCOMMAND_OEM: u32 = 4096u32;
pub const FCONTROL: u32 = 8u32;
pub const FE_FONTSMOOTHINGCLEARTYPE: u32 = 2u32;
pub const FE_FONTSMOOTHINGORIENTATIONBGR: u32 = 0u32;
pub const FE_FONTSMOOTHINGORIENTATIONRGB: u32 = 1u32;
pub const FE_FONTSMOOTHINGSTANDARD: u32 = 1u32;
pub const FKF_AVAILABLE: u32 = 2u32;
pub const FKF_CLICKON: u32 = 64u32;
pub const FKF_CONFIRMHOTKEY: u32 = 8u32;
pub const FKF_FILTERKEYSON: u32 = 1u32;
pub const FKF_HOTKEYACTIVE: u32 = 4u32;
pub const FKF_HOTKEYSOUND: u32 = 16u32;
pub const FKF_INDICATOR: u32 = 32u32;
pub struct FLASHWINFO {
    pub cbSize: u32,
    pub hwnd: super::super::Foundation::HWND,
    pub dwFlags: FLASHWINFO_FLAGS,
    pub uCount: u32,
    pub dwTimeout: u32,
}
impl ::core::marker::Copy for FLASHWINFO {}
impl ::core::clone::Clone for FLASHWINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FLASHWINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLASHWINFO")
            .field("cbSize", &self.cbSize)
            .field("hwnd", &self.hwnd)
            .field("dwFlags", &self.dwFlags)
            .field("uCount", &self.uCount)
            .field("dwTimeout", &self.dwTimeout)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FLASHWINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.hwnd == other.hwnd
            && self.dwFlags == other.dwFlags
            && self.uCount == other.uCount
            && self.dwTimeout == other.dwTimeout
    }
}
impl ::core::cmp::Eq for FLASHWINFO {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FLASHWINFO_FLAGS(pub u32);
pub const FLASHW_ALL: FLASHWINFO_FLAGS = FLASHWINFO_FLAGS(3u32);
pub const FLASHW_CAPTION: FLASHWINFO_FLAGS = FLASHWINFO_FLAGS(1u32);
pub const FLASHW_STOP: FLASHWINFO_FLAGS = FLASHWINFO_FLAGS(0u32);
pub const FLASHW_TIMER: FLASHWINFO_FLAGS = FLASHWINFO_FLAGS(4u32);
pub const FLASHW_TIMERNOFG: FLASHWINFO_FLAGS = FLASHWINFO_FLAGS(12u32);
pub const FLASHW_TRAY: FLASHWINFO_FLAGS = FLASHWINFO_FLAGS(2u32);
impl ::core::marker::Copy for FLASHWINFO_FLAGS {}
impl ::core::clone::Clone for FLASHWINFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FLASHWINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FLASHWINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FLASHWINFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FLASHWINFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FLASHWINFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FLASHWINFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FLASHWINFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FLASHWINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const FNOINVERT: u32 = 2u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FOREGROUND_WINDOW_LOCK_CODE(pub u32);
pub const LSFW_LOCK: FOREGROUND_WINDOW_LOCK_CODE = FOREGROUND_WINDOW_LOCK_CODE(1u32);
pub const LSFW_UNLOCK: FOREGROUND_WINDOW_LOCK_CODE = FOREGROUND_WINDOW_LOCK_CODE(2u32);
impl ::core::marker::Copy for FOREGROUND_WINDOW_LOCK_CODE {}
impl ::core::clone::Clone for FOREGROUND_WINDOW_LOCK_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FOREGROUND_WINDOW_LOCK_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FOREGROUND_WINDOW_LOCK_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FOREGROUND_WINDOW_LOCK_CODE")
            .field(&self.0)
            .finish()
    }
}
pub const FSHIFT: u32 = 4u32;
pub const FVIRTKEY: u32 = 1u32;
pub const GCF_INCLUDE_ANCESTORS: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GDI_IMAGE_TYPE(pub u32);
pub const IMAGE_BITMAP: GDI_IMAGE_TYPE = GDI_IMAGE_TYPE(0u32);
pub const IMAGE_CURSOR: GDI_IMAGE_TYPE = GDI_IMAGE_TYPE(2u32);
pub const IMAGE_ICON: GDI_IMAGE_TYPE = GDI_IMAGE_TYPE(1u32);
impl ::core::marker::Copy for GDI_IMAGE_TYPE {}
impl ::core::clone::Clone for GDI_IMAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GDI_IMAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GDI_IMAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GDI_IMAGE_TYPE").field(&self.0).finish()
    }
}
pub const GESTURECONFIGMAXCOUNT: u32 = 256u32;
pub const GESTUREVISUALIZATION_DOUBLETAP: u32 = 2u32;
pub const GESTUREVISUALIZATION_OFF: u32 = 0u32;
pub const GESTUREVISUALIZATION_ON: u32 = 31u32;
pub const GESTUREVISUALIZATION_PRESSANDHOLD: u32 = 8u32;
pub const GESTUREVISUALIZATION_PRESSANDTAP: u32 = 4u32;
pub const GESTUREVISUALIZATION_RIGHTTAP: u32 = 16u32;
pub const GESTUREVISUALIZATION_TAP: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GET_ANCESTOR_FLAGS(pub u32);
pub const GA_PARENT: GET_ANCESTOR_FLAGS = GET_ANCESTOR_FLAGS(1u32);
pub const GA_ROOT: GET_ANCESTOR_FLAGS = GET_ANCESTOR_FLAGS(2u32);
pub const GA_ROOTOWNER: GET_ANCESTOR_FLAGS = GET_ANCESTOR_FLAGS(3u32);
impl ::core::marker::Copy for GET_ANCESTOR_FLAGS {}
impl ::core::clone::Clone for GET_ANCESTOR_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_ANCESTOR_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_ANCESTOR_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_ANCESTOR_FLAGS").field(&self.0).finish()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GET_CLASS_LONG_INDEX(pub i32);
pub const GCW_ATOM: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-32i32);
pub const GCL_CBCLSEXTRA: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-20i32);
pub const GCL_CBWNDEXTRA: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-18i32);
pub const GCL_HBRBACKGROUND: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-10i32);
pub const GCL_HCURSOR: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-12i32);
pub const GCL_HICON: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-14i32);
pub const GCL_HICONSM: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-34i32);
pub const GCL_HMODULE: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-16i32);
pub const GCL_MENUNAME: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-8i32);
pub const GCL_STYLE: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-26i32);
pub const GCL_WNDPROC: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-24i32);
pub const GCLP_HBRBACKGROUND: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-10i32);
pub const GCLP_HCURSOR: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-12i32);
pub const GCLP_HICON: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-14i32);
pub const GCLP_HICONSM: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-34i32);
pub const GCLP_HMODULE: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-16i32);
pub const GCLP_MENUNAME: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-8i32);
pub const GCLP_WNDPROC: GET_CLASS_LONG_INDEX = GET_CLASS_LONG_INDEX(-24i32);
impl ::core::marker::Copy for GET_CLASS_LONG_INDEX {}
impl ::core::clone::Clone for GET_CLASS_LONG_INDEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_CLASS_LONG_INDEX {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_CLASS_LONG_INDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_CLASS_LONG_INDEX")
            .field(&self.0)
            .finish()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GET_MENU_DEFAULT_ITEM_FLAGS(pub u32);
pub const GMDI_GOINTOPOPUPS: GET_MENU_DEFAULT_ITEM_FLAGS = GET_MENU_DEFAULT_ITEM_FLAGS(2u32);
pub const GMDI_USEDISABLED: GET_MENU_DEFAULT_ITEM_FLAGS = GET_MENU_DEFAULT_ITEM_FLAGS(1u32);
impl ::core::marker::Copy for GET_MENU_DEFAULT_ITEM_FLAGS {}
impl ::core::clone::Clone for GET_MENU_DEFAULT_ITEM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_MENU_DEFAULT_ITEM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_MENU_DEFAULT_ITEM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_MENU_DEFAULT_ITEM_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for GET_MENU_DEFAULT_ITEM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GET_MENU_DEFAULT_ITEM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GET_MENU_DEFAULT_ITEM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GET_MENU_DEFAULT_ITEM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GET_MENU_DEFAULT_ITEM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GET_WINDOW_CMD(pub u32);
pub const GW_CHILD: GET_WINDOW_CMD = GET_WINDOW_CMD(5u32);
pub const GW_ENABLEDPOPUP: GET_WINDOW_CMD = GET_WINDOW_CMD(6u32);
pub const GW_HWNDFIRST: GET_WINDOW_CMD = GET_WINDOW_CMD(0u32);
pub const GW_HWNDLAST: GET_WINDOW_CMD = GET_WINDOW_CMD(1u32);
pub const GW_HWNDNEXT: GET_WINDOW_CMD = GET_WINDOW_CMD(2u32);
pub const GW_HWNDPREV: GET_WINDOW_CMD = GET_WINDOW_CMD(3u32);
pub const GW_OWNER: GET_WINDOW_CMD = GET_WINDOW_CMD(4u32);
impl ::core::marker::Copy for GET_WINDOW_CMD {}
impl ::core::clone::Clone for GET_WINDOW_CMD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_WINDOW_CMD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_WINDOW_CMD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_WINDOW_CMD").field(&self.0).finish()
    }
}
pub const GF_BEGIN: u32 = 1u32;
pub const GF_END: u32 = 4u32;
pub const GF_INERTIA: u32 = 2u32;
pub const GIDC_ARRIVAL: u32 = 1u32;
pub const GIDC_REMOVAL: u32 = 2u32;
pub struct GUITHREADINFO {
    pub cbSize: u32,
    pub flags: GUITHREADINFO_FLAGS,
    pub hwndActive: super::super::Foundation::HWND,
    pub hwndFocus: super::super::Foundation::HWND,
    pub hwndCapture: super::super::Foundation::HWND,
    pub hwndMenuOwner: super::super::Foundation::HWND,
    pub hwndMoveSize: super::super::Foundation::HWND,
    pub hwndCaret: super::super::Foundation::HWND,
    pub rcCaret: super::super::Foundation::RECT,
}
impl ::core::marker::Copy for GUITHREADINFO {}
impl ::core::clone::Clone for GUITHREADINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GUITHREADINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GUITHREADINFO")
            .field("cbSize", &self.cbSize)
            .field("flags", &self.flags)
            .field("hwndActive", &self.hwndActive)
            .field("hwndFocus", &self.hwndFocus)
            .field("hwndCapture", &self.hwndCapture)
            .field("hwndMenuOwner", &self.hwndMenuOwner)
            .field("hwndMoveSize", &self.hwndMoveSize)
            .field("hwndCaret", &self.hwndCaret)
            .field("rcCaret", &self.rcCaret)
            .finish()
    }
}
impl ::core::cmp::PartialEq for GUITHREADINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.flags == other.flags
            && self.hwndActive == other.hwndActive
            && self.hwndFocus == other.hwndFocus
            && self.hwndCapture == other.hwndCapture
            && self.hwndMenuOwner == other.hwndMenuOwner
            && self.hwndMoveSize == other.hwndMoveSize
            && self.hwndCaret == other.hwndCaret
            && self.rcCaret == other.rcCaret
    }
}
impl ::core::cmp::Eq for GUITHREADINFO {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GUITHREADINFO_FLAGS(pub u32);
pub const GUI_CARETBLINKING: GUITHREADINFO_FLAGS = GUITHREADINFO_FLAGS(1u32);
pub const GUI_INMENUMODE: GUITHREADINFO_FLAGS = GUITHREADINFO_FLAGS(4u32);
pub const GUI_INMOVESIZE: GUITHREADINFO_FLAGS = GUITHREADINFO_FLAGS(2u32);
pub const GUI_POPUPMENUMODE: GUITHREADINFO_FLAGS = GUITHREADINFO_FLAGS(16u32);
pub const GUI_SYSTEMMENUMODE: GUITHREADINFO_FLAGS = GUITHREADINFO_FLAGS(8u32);
impl ::core::marker::Copy for GUITHREADINFO_FLAGS {}
impl ::core::clone::Clone for GUITHREADINFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GUITHREADINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GUITHREADINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GUITHREADINFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GUITHREADINFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GUITHREADINFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GUITHREADINFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GUITHREADINFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GUITHREADINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const GUI_16BITTASK: u32 = 0u32;
pub const GWFS_INCLUDE_ANCESTORS: u32 = 1u32;
pub const GW_MAX: u32 = 5u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HACCEL(pub isize);
impl HACCEL {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HACCEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HACCEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HACCEL {}
impl ::core::fmt::Debug for HACCEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HACCEL").field(&self.0).finish()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HANDEDNESS(pub i32);
pub const HANDEDNESS_LEFT: HANDEDNESS = HANDEDNESS(0i32);
pub const HANDEDNESS_RIGHT: HANDEDNESS = HANDEDNESS(1i32);
impl ::core::marker::Copy for HANDEDNESS {}
impl ::core::clone::Clone for HANDEDNESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HANDEDNESS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HANDEDNESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HANDEDNESS").field(&self.0).finish()
    }
}
pub struct HARDWAREHOOKSTRUCT {
    pub hwnd: super::super::Foundation::HWND,
    pub message: u32,
    pub wParam: super::super::Foundation::WPARAM,
    pub lParam: super::super::Foundation::LPARAM,
}
impl ::core::marker::Copy for HARDWAREHOOKSTRUCT {}
impl ::core::clone::Clone for HARDWAREHOOKSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HARDWAREHOOKSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HARDWAREHOOKSTRUCT")
            .field("hwnd", &self.hwnd)
            .field("message", &self.message)
            .field("wParam", &self.wParam)
            .field("lParam", &self.lParam)
            .finish()
    }
}
impl ::core::cmp::PartialEq for HARDWAREHOOKSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.hwnd == other.hwnd
            && self.message == other.message
            && self.wParam == other.wParam
            && self.lParam == other.lParam
    }
}
impl ::core::cmp::Eq for HARDWAREHOOKSTRUCT {}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub const HBMMENU_CALLBACK: super::super::Graphics::Gdi::HBITMAP =
    super::super::Graphics::Gdi::HBITMAP(-1i32 as _);
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub const HBMMENU_MBAR_CLOSE: super::super::Graphics::Gdi::HBITMAP =
    super::super::Graphics::Gdi::HBITMAP(5i32 as _);
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub const HBMMENU_MBAR_CLOSE_D: super::super::Graphics::Gdi::HBITMAP =
    super::super::Graphics::Gdi::HBITMAP(6i32 as _);
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub const HBMMENU_MBAR_MINIMIZE: super::super::Graphics::Gdi::HBITMAP =
    super::super::Graphics::Gdi::HBITMAP(3i32 as _);
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub const HBMMENU_MBAR_MINIMIZE_D: super::super::Graphics::Gdi::HBITMAP =
    super::super::Graphics::Gdi::HBITMAP(7i32 as _);
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub const HBMMENU_MBAR_RESTORE: super::super::Graphics::Gdi::HBITMAP =
    super::super::Graphics::Gdi::HBITMAP(2i32 as _);
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub const HBMMENU_POPUP_CLOSE: super::super::Graphics::Gdi::HBITMAP =
    super::super::Graphics::Gdi::HBITMAP(8i32 as _);
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub const HBMMENU_POPUP_MAXIMIZE: super::super::Graphics::Gdi::HBITMAP =
    super::super::Graphics::Gdi::HBITMAP(10i32 as _);
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub const HBMMENU_POPUP_MINIMIZE: super::super::Graphics::Gdi::HBITMAP =
    super::super::Graphics::Gdi::HBITMAP(11i32 as _);
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub const HBMMENU_POPUP_RESTORE: super::super::Graphics::Gdi::HBITMAP =
    super::super::Graphics::Gdi::HBITMAP(9i32 as _);
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub const HBMMENU_SYSTEM: super::super::Graphics::Gdi::HBITMAP =
    super::super::Graphics::Gdi::HBITMAP(1i32 as _);
pub const HCBT_ACTIVATE: u32 = 5u32;
pub const HCBT_CLICKSKIPPED: u32 = 6u32;
pub const HCBT_CREATEWND: u32 = 3u32;
pub const HCBT_DESTROYWND: u32 = 4u32;
pub const HCBT_KEYSKIPPED: u32 = 7u32;
pub const HCBT_MINMAX: u32 = 1u32;
pub const HCBT_MOVESIZE: u32 = 0u32;
pub const HCBT_QS: u32 = 2u32;
pub const HCBT_SETFOCUS: u32 = 9u32;
pub const HCBT_SYSCOMMAND: u32 = 8u32;
pub const HCF_DEFAULTDESKTOP: u32 = 512u32;
pub const HCF_LOGONDESKTOP: u32 = 256u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HCURSOR(pub isize);
impl HCURSOR {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HCURSOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HCURSOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HCURSOR {}
impl ::core::fmt::Debug for HCURSOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCURSOR").field(&self.0).finish()
    }
}
pub const HC_ACTION: u32 = 0u32;
pub const HC_GETNEXT: u32 = 1u32;
pub const HC_NOREM: u32 = 3u32;
pub const HC_NOREMOVE: u32 = 3u32;
pub const HC_SKIP: u32 = 2u32;
pub const HC_SYSMODALOFF: u32 = 5u32;
pub const HC_SYSMODALON: u32 = 4u32;
pub const HELPINFO_MENUITEM: u32 = 2u32;
pub const HELPINFO_WINDOW: u32 = 1u32;
pub const HELP_COMMAND: i32 = 258i32;
pub const HELP_CONTENTS: i32 = 3i32;
pub const HELP_CONTEXT: i32 = 1i32;
pub const HELP_CONTEXTMENU: u32 = 10u32;
pub const HELP_CONTEXTPOPUP: i32 = 8i32;
pub const HELP_FINDER: u32 = 11u32;
pub const HELP_FORCEFILE: i32 = 9i32;
pub const HELP_HELPONHELP: i32 = 4i32;
pub const HELP_INDEX: i32 = 3i32;
pub const HELP_KEY: i32 = 257i32;
pub const HELP_MULTIKEY: i32 = 513i32;
pub const HELP_PARTIALKEY: i32 = 261i32;
pub const HELP_QUIT: i32 = 2i32;
pub const HELP_SETCONTENTS: i32 = 5i32;
pub const HELP_SETINDEX: i32 = 5i32;
pub const HELP_SETPOPUP_POS: u32 = 13u32;
pub const HELP_SETWINPOS: i32 = 515i32;
pub const HELP_TCARD: u32 = 32768u32;
pub const HELP_TCARD_DATA: u32 = 16u32;
pub const HELP_TCARD_OTHER_CALLER: u32 = 17u32;
pub const HELP_WM_HELP: u32 = 12u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HHOOK(pub isize);
impl HHOOK {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HHOOK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HHOOK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HHOOK {}
impl ::core::fmt::Debug for HHOOK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HHOOK").field(&self.0).finish()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HICON(pub isize);
impl HICON {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HICON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HICON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HICON {}
impl ::core::fmt::Debug for HICON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HICON").field(&self.0).finish()
    }
}
pub const HIDE_WINDOW: u32 = 0u32;
pub const HKL_NEXT: u32 = 1u32;
pub const HKL_PREV: u32 = 0u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HMENU(pub isize);
impl HMENU {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HMENU {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HMENU {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HMENU {}
impl ::core::fmt::Debug for HMENU {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HMENU").field(&self.0).finish()
    }
}
pub type HOOKPROC = ::core::option::Option<
    unsafe extern "system" fn(
        code: i32,
        wParam: super::super::Foundation::WPARAM,
        lParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::LRESULT,
>;
pub const HSHELL_ACCESSIBILITYSTATE: u32 = 11u32;
pub const HSHELL_ACTIVATESHELLWINDOW: u32 = 3u32;
pub const HSHELL_APPCOMMAND: u32 = 12u32;
pub const HSHELL_ENDTASK: u32 = 10u32;
pub const HSHELL_GETMINRECT: u32 = 5u32;
pub const HSHELL_HIGHBIT: u32 = 32768u32;
pub const HSHELL_LANGUAGE: u32 = 8u32;
pub const HSHELL_MONITORCHANGED: u32 = 16u32;
pub const HSHELL_REDRAW: u32 = 6u32;
pub const HSHELL_SYSMENU: u32 = 9u32;
pub const HSHELL_TASKMAN: u32 = 7u32;
pub const HSHELL_WINDOWACTIVATED: u32 = 4u32;
pub const HSHELL_WINDOWCREATED: u32 = 1u32;
pub const HSHELL_WINDOWDESTROYED: u32 = 2u32;
pub const HSHELL_WINDOWREPLACED: u32 = 13u32;
pub const HSHELL_WINDOWREPLACING: u32 = 14u32;
pub const HTBORDER: u32 = 18u32;
pub const HTBOTTOM: u32 = 15u32;
pub const HTBOTTOMLEFT: u32 = 16u32;
pub const HTBOTTOMRIGHT: u32 = 17u32;
pub const HTCAPTION: u32 = 2u32;
pub const HTCLIENT: u32 = 1u32;
pub const HTCLOSE: u32 = 20u32;
pub const HTERROR: i32 = -2i32;
pub const HTGROWBOX: u32 = 4u32;
pub const HTHELP: u32 = 21u32;
pub const HTHSCROLL: u32 = 6u32;
pub const HTLEFT: u32 = 10u32;
pub const HTMAXBUTTON: u32 = 9u32;
pub const HTMENU: u32 = 5u32;
pub const HTMINBUTTON: u32 = 8u32;
pub const HTNOWHERE: u32 = 0u32;
pub const HTOBJECT: u32 = 19u32;
pub const HTREDUCE: u32 = 8u32;
pub const HTRIGHT: u32 = 11u32;
pub const HTSIZE: u32 = 4u32;
pub const HTSIZEFIRST: u32 = 10u32;
pub const HTSIZELAST: u32 = 17u32;
pub const HTSYSMENU: u32 = 3u32;
pub const HTTOP: u32 = 12u32;
pub const HTTOPLEFT: u32 = 13u32;
pub const HTTOPRIGHT: u32 = 14u32;
pub const HTTRANSPARENT: i32 = -1i32;
pub const HTVSCROLL: u32 = 7u32;
pub const HTZOOM: u32 = 9u32;
pub const HWND_BOTTOM: super::super::Foundation::HWND = super::super::Foundation::HWND(1i32 as _);
pub const HWND_DESKTOP: super::super::Foundation::HWND = super::super::Foundation::HWND(0i32 as _);
pub const HWND_MESSAGE: super::super::Foundation::HWND = super::super::Foundation::HWND(-3i32 as _);
pub const HWND_NOTOPMOST: super::super::Foundation::HWND =
    super::super::Foundation::HWND(-2i32 as _);
pub const HWND_TOP: super::super::Foundation::HWND = super::super::Foundation::HWND(0i32 as _);
pub const HWND_TOPMOST: super::super::Foundation::HWND = super::super::Foundation::HWND(-1i32 as _);
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ICONINFO {
    pub fIcon: super::super::Foundation::BOOL,
    pub xHotspot: u32,
    pub yHotspot: u32,
    pub hbmMask: super::super::Graphics::Gdi::HBITMAP,
    pub hbmColor: super::super::Graphics::Gdi::HBITMAP,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ICONINFO {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ICONINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ICONINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICONINFO")
            .field("fIcon", &self.fIcon)
            .field("xHotspot", &self.xHotspot)
            .field("yHotspot", &self.yHotspot)
            .field("hbmMask", &self.hbmMask)
            .field("hbmColor", &self.hbmColor)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ICONINFO {
    fn eq(&self, other: &Self) -> bool {
        self.fIcon == other.fIcon
            && self.xHotspot == other.xHotspot
            && self.yHotspot == other.yHotspot
            && self.hbmMask == other.hbmMask
            && self.hbmColor == other.hbmColor
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ICONINFO {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ICONINFOEXA {
    pub cbSize: u32,
    pub fIcon: super::super::Foundation::BOOL,
    pub xHotspot: u32,
    pub yHotspot: u32,
    pub hbmMask: super::super::Graphics::Gdi::HBITMAP,
    pub hbmColor: super::super::Graphics::Gdi::HBITMAP,
    pub wResID: u16,
    pub szModName: [super::super::Foundation::CHAR; 260],
    pub szResName: [super::super::Foundation::CHAR; 260],
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ICONINFOEXA {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ICONINFOEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ICONINFOEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICONINFOEXA")
            .field("cbSize", &self.cbSize)
            .field("fIcon", &self.fIcon)
            .field("xHotspot", &self.xHotspot)
            .field("yHotspot", &self.yHotspot)
            .field("hbmMask", &self.hbmMask)
            .field("hbmColor", &self.hbmColor)
            .field("wResID", &self.wResID)
            .field("szModName", &self.szModName)
            .field("szResName", &self.szResName)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ICONINFOEXA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.fIcon == other.fIcon
            && self.xHotspot == other.xHotspot
            && self.yHotspot == other.yHotspot
            && self.hbmMask == other.hbmMask
            && self.hbmColor == other.hbmColor
            && self.wResID == other.wResID
            && self.szModName == other.szModName
            && self.szResName == other.szResName
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ICONINFOEXA {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ICONINFOEXW {
    pub cbSize: u32,
    pub fIcon: super::super::Foundation::BOOL,
    pub xHotspot: u32,
    pub yHotspot: u32,
    pub hbmMask: super::super::Graphics::Gdi::HBITMAP,
    pub hbmColor: super::super::Graphics::Gdi::HBITMAP,
    pub wResID: u16,
    pub szModName: [u16; 260],
    pub szResName: [u16; 260],
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ICONINFOEXW {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ICONINFOEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ICONINFOEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICONINFOEXW")
            .field("cbSize", &self.cbSize)
            .field("fIcon", &self.fIcon)
            .field("xHotspot", &self.xHotspot)
            .field("yHotspot", &self.yHotspot)
            .field("hbmMask", &self.hbmMask)
            .field("hbmColor", &self.hbmColor)
            .field("wResID", &self.wResID)
            .field("szModName", &self.szModName)
            .field("szResName", &self.szResName)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ICONINFOEXW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.fIcon == other.fIcon
            && self.xHotspot == other.xHotspot
            && self.yHotspot == other.yHotspot
            && self.hbmMask == other.hbmMask
            && self.hbmColor == other.hbmColor
            && self.wResID == other.wResID
            && self.szModName == other.szModName
            && self.szResName == other.szResName
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ICONINFOEXW {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ICONMETRICSA {
    pub cbSize: u32,
    pub iHorzSpacing: i32,
    pub iVertSpacing: i32,
    pub iTitleWrap: i32,
    pub lfFont: super::super::Graphics::Gdi::LOGFONTA,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ICONMETRICSA {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ICONMETRICSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ICONMETRICSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICONMETRICSA")
            .field("cbSize", &self.cbSize)
            .field("iHorzSpacing", &self.iHorzSpacing)
            .field("iVertSpacing", &self.iVertSpacing)
            .field("iTitleWrap", &self.iTitleWrap)
            .field("lfFont", &self.lfFont)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ICONMETRICSA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.iHorzSpacing == other.iHorzSpacing
            && self.iVertSpacing == other.iVertSpacing
            && self.iTitleWrap == other.iTitleWrap
            && self.lfFont == other.lfFont
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ICONMETRICSA {}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ICONMETRICSW {
    pub cbSize: u32,
    pub iHorzSpacing: i32,
    pub iVertSpacing: i32,
    pub iTitleWrap: i32,
    pub lfFont: super::super::Graphics::Gdi::LOGFONTW,
}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ICONMETRICSW {}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ICONMETRICSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ICONMETRICSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICONMETRICSW")
            .field("cbSize", &self.cbSize)
            .field("iHorzSpacing", &self.iHorzSpacing)
            .field("iVertSpacing", &self.iVertSpacing)
            .field("iTitleWrap", &self.iTitleWrap)
            .field("lfFont", &self.lfFont)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ICONMETRICSW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.iHorzSpacing == other.iHorzSpacing
            && self.iVertSpacing == other.iVertSpacing
            && self.iTitleWrap == other.iTitleWrap
            && self.lfFont == other.lfFont
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ICONMETRICSW {}
pub const ICON_BIG: u32 = 1u32;
pub const ICON_SMALL: u32 = 0u32;
pub const ICON_SMALL2: u32 = 2u32;
pub const IDANI_CAPTION: u32 = 3u32;
pub const IDANI_OPEN: u32 = 1u32;
pub const IDC_APPSTARTING: ::win32::core::PCWSTR = ::win32::core::PCWSTR(32650i32 as _);
pub const IDC_ARROW: ::win32::core::PCWSTR = ::win32::core::PCWSTR(32512i32 as _);
pub const IDC_CROSS: ::win32::core::PCWSTR = ::win32::core::PCWSTR(32515i32 as _);
pub const IDC_HAND: ::win32::core::PCWSTR = ::win32::core::PCWSTR(32649i32 as _);
pub const IDC_HELP: ::win32::core::PCWSTR = ::win32::core::PCWSTR(32651i32 as _);
pub const IDC_IBEAM: ::win32::core::PCWSTR = ::win32::core::PCWSTR(32513i32 as _);
pub const IDC_ICON: ::win32::core::PCWSTR = ::win32::core::PCWSTR(32641i32 as _);
pub const IDC_NO: ::win32::core::PCWSTR = ::win32::core::PCWSTR(32648i32 as _);
pub const IDC_PERSON: ::win32::core::PCWSTR = ::win32::core::PCWSTR(32672i32 as _);
pub const IDC_PIN: ::win32::core::PCWSTR = ::win32::core::PCWSTR(32671i32 as _);
pub const IDC_SIZE: ::win32::core::PCWSTR = ::win32::core::PCWSTR(32640i32 as _);
pub const IDC_SIZEALL: ::win32::core::PCWSTR = ::win32::core::PCWSTR(32646i32 as _);
pub const IDC_SIZENESW: ::win32::core::PCWSTR = ::win32::core::PCWSTR(32643i32 as _);
pub const IDC_SIZENS: ::win32::core::PCWSTR = ::win32::core::PCWSTR(32645i32 as _);
pub const IDC_SIZENWSE: ::win32::core::PCWSTR = ::win32::core::PCWSTR(32642i32 as _);
pub const IDC_SIZEWE: ::win32::core::PCWSTR = ::win32::core::PCWSTR(32644i32 as _);
pub const IDC_UPARROW: ::win32::core::PCWSTR = ::win32::core::PCWSTR(32516i32 as _);
pub const IDC_WAIT: ::win32::core::PCWSTR = ::win32::core::PCWSTR(32514i32 as _);
pub const IDHOT_SNAPDESKTOP: i32 = -2i32;
pub const IDHOT_SNAPWINDOW: i32 = -1i32;
pub const IDH_CANCEL: u32 = 28444u32;
pub const IDH_GENERIC_HELP_BUTTON: u32 = 28442u32;
pub const IDH_HELP: u32 = 28445u32;
pub const IDH_MISSING_CONTEXT: u32 = 28441u32;
pub const IDH_NO_HELP: u32 = 28440u32;
pub const IDH_OK: u32 = 28443u32;
pub const IDI_APPLICATION: ::win32::core::PCWSTR = ::win32::core::PCWSTR(32512u32 as _);
pub const IDI_ASTERISK: ::win32::core::PCWSTR = ::win32::core::PCWSTR(32516u32 as _);
pub const IDI_ERROR: u32 = 32513u32;
pub const IDI_EXCLAMATION: ::win32::core::PCWSTR = ::win32::core::PCWSTR(32515u32 as _);
pub const IDI_HAND: ::win32::core::PCWSTR = ::win32::core::PCWSTR(32513u32 as _);
pub const IDI_INFORMATION: u32 = 32516u32;
pub const IDI_QUESTION: ::win32::core::PCWSTR = ::win32::core::PCWSTR(32514u32 as _);
pub const IDI_SHIELD: ::win32::core::PCWSTR = ::win32::core::PCWSTR(32518u32 as _);
pub const IDI_WARNING: u32 = 32515u32;
pub const IDI_WINLOGO: ::win32::core::PCWSTR = ::win32::core::PCWSTR(32517u32 as _);
pub const IMAGE_ENHMETAFILE: u32 = 3u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IMAGE_FLAGS(pub u32);
pub const LR_CREATEDIBSECTION: IMAGE_FLAGS = IMAGE_FLAGS(8192u32);
pub const LR_DEFAULTCOLOR: IMAGE_FLAGS = IMAGE_FLAGS(0u32);
pub const LR_DEFAULTSIZE: IMAGE_FLAGS = IMAGE_FLAGS(64u32);
pub const LR_LOADFROMFILE: IMAGE_FLAGS = IMAGE_FLAGS(16u32);
pub const LR_LOADMAP3DCOLORS: IMAGE_FLAGS = IMAGE_FLAGS(4096u32);
pub const LR_LOADTRANSPARENT: IMAGE_FLAGS = IMAGE_FLAGS(32u32);
pub const LR_MONOCHROME: IMAGE_FLAGS = IMAGE_FLAGS(1u32);
pub const LR_SHARED: IMAGE_FLAGS = IMAGE_FLAGS(32768u32);
pub const LR_VGACOLOR: IMAGE_FLAGS = IMAGE_FLAGS(128u32);
pub const LR_COPYDELETEORG: IMAGE_FLAGS = IMAGE_FLAGS(8u32);
pub const LR_COPYFROMRESOURCE: IMAGE_FLAGS = IMAGE_FLAGS(16384u32);
pub const LR_COPYRETURNORG: IMAGE_FLAGS = IMAGE_FLAGS(4u32);
impl ::core::marker::Copy for IMAGE_FLAGS {}
impl ::core::clone::Clone for IMAGE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMAGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IMAGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IMAGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IMAGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IMAGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IMAGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const INDEXID_CONTAINER: u32 = 0u32;
pub const INDEXID_OBJECT: u32 = 0u32;
pub const INPUTLANGCHANGE_BACKWARD: u32 = 4u32;
pub const INPUTLANGCHANGE_FORWARD: u32 = 2u32;
pub const INPUTLANGCHANGE_SYSCHARSET: u32 = 1u32;
pub const ISMEX_CALLBACK: u32 = 4u32;
pub const ISMEX_NOSEND: u32 = 0u32;
pub const ISMEX_NOTIFY: u32 = 2u32;
pub const ISMEX_REPLIED: u32 = 8u32;
pub const ISMEX_SEND: u32 = 1u32;
pub const ISOLATIONAWARE_MANIFEST_RESOURCE_ID: u32 = 2u32;
pub const ISOLATIONAWARE_NOSTATICIMPORT_MANIFEST_RESOURCE_ID: u32 = 3u32;
pub const ISOLATIONPOLICY_BROWSER_MANIFEST_RESOURCE_ID: u32 = 5u32;
pub const ISOLATIONPOLICY_MANIFEST_RESOURCE_ID: u32 = 4u32;
pub struct IndexedResourceQualifier {
    pub name: ::win32::core::PWSTR,
    pub value: ::win32::core::PWSTR,
}
impl ::core::marker::Copy for IndexedResourceQualifier {}
impl ::core::clone::Clone for IndexedResourceQualifier {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IndexedResourceQualifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IndexedResourceQualifier")
            .field("name", &self.name)
            .field("value", &self.value)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IndexedResourceQualifier {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value == other.value
    }
}
impl ::core::cmp::Eq for IndexedResourceQualifier {}
pub struct KBDLLHOOKSTRUCT {
    pub vkCode: u32,
    pub scanCode: u32,
    pub flags: KBDLLHOOKSTRUCT_FLAGS,
    pub time: u32,
    pub dwExtraInfo: usize,
}
impl ::core::marker::Copy for KBDLLHOOKSTRUCT {}
impl ::core::clone::Clone for KBDLLHOOKSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KBDLLHOOKSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KBDLLHOOKSTRUCT")
            .field("vkCode", &self.vkCode)
            .field("scanCode", &self.scanCode)
            .field("flags", &self.flags)
            .field("time", &self.time)
            .field("dwExtraInfo", &self.dwExtraInfo)
            .finish()
    }
}
impl ::core::cmp::PartialEq for KBDLLHOOKSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.vkCode == other.vkCode
            && self.scanCode == other.scanCode
            && self.flags == other.flags
            && self.time == other.time
            && self.dwExtraInfo == other.dwExtraInfo
    }
}
impl ::core::cmp::Eq for KBDLLHOOKSTRUCT {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct KBDLLHOOKSTRUCT_FLAGS(pub u32);
pub const LLKHF_EXTENDED: KBDLLHOOKSTRUCT_FLAGS = KBDLLHOOKSTRUCT_FLAGS(1u32);
pub const LLKHF_ALTDOWN: KBDLLHOOKSTRUCT_FLAGS = KBDLLHOOKSTRUCT_FLAGS(32u32);
pub const LLKHF_UP: KBDLLHOOKSTRUCT_FLAGS = KBDLLHOOKSTRUCT_FLAGS(128u32);
pub const LLKHF_INJECTED: KBDLLHOOKSTRUCT_FLAGS = KBDLLHOOKSTRUCT_FLAGS(16u32);
pub const LLKHF_LOWER_IL_INJECTED: KBDLLHOOKSTRUCT_FLAGS = KBDLLHOOKSTRUCT_FLAGS(2u32);
impl ::core::marker::Copy for KBDLLHOOKSTRUCT_FLAGS {}
impl ::core::clone::Clone for KBDLLHOOKSTRUCT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KBDLLHOOKSTRUCT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KBDLLHOOKSTRUCT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KBDLLHOOKSTRUCT_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for KBDLLHOOKSTRUCT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for KBDLLHOOKSTRUCT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for KBDLLHOOKSTRUCT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for KBDLLHOOKSTRUCT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for KBDLLHOOKSTRUCT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const KF_ALTDOWN: u32 = 8192u32;
pub const KF_DLGMODE: u32 = 2048u32;
pub const KF_EXTENDED: u32 = 256u32;
pub const KF_MENUMODE: u32 = 4096u32;
pub const KF_REPEAT: u32 = 16384u32;
pub const KF_UP: u32 = 32768u32;
pub const KL_NAMELENGTH: u32 = 9u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LAYERED_WINDOW_ATTRIBUTES_FLAGS(pub u32);
pub const LWA_ALPHA: LAYERED_WINDOW_ATTRIBUTES_FLAGS = LAYERED_WINDOW_ATTRIBUTES_FLAGS(2u32);
pub const LWA_COLORKEY: LAYERED_WINDOW_ATTRIBUTES_FLAGS = LAYERED_WINDOW_ATTRIBUTES_FLAGS(1u32);
impl ::core::marker::Copy for LAYERED_WINDOW_ATTRIBUTES_FLAGS {}
impl ::core::clone::Clone for LAYERED_WINDOW_ATTRIBUTES_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LAYERED_WINDOW_ATTRIBUTES_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LAYERED_WINDOW_ATTRIBUTES_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LAYERED_WINDOW_ATTRIBUTES_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for LAYERED_WINDOW_ATTRIBUTES_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LAYERED_WINDOW_ATTRIBUTES_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LAYERED_WINDOW_ATTRIBUTES_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LAYERED_WINDOW_ATTRIBUTES_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LAYERED_WINDOW_ATTRIBUTES_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const LBN_DBLCLK: u32 = 2u32;
pub const LBN_ERRSPACE: i32 = -2i32;
pub const LBN_KILLFOCUS: u32 = 5u32;
pub const LBN_SELCANCEL: u32 = 3u32;
pub const LBN_SELCHANGE: u32 = 1u32;
pub const LBN_SETFOCUS: u32 = 4u32;
pub const LBS_COMBOBOX: i32 = 32768i32;
pub const LBS_DISABLENOSCROLL: i32 = 4096i32;
pub const LBS_EXTENDEDSEL: i32 = 2048i32;
pub const LBS_HASSTRINGS: i32 = 64i32;
pub const LBS_MULTICOLUMN: i32 = 512i32;
pub const LBS_MULTIPLESEL: i32 = 8i32;
pub const LBS_NODATA: i32 = 8192i32;
pub const LBS_NOINTEGRALHEIGHT: i32 = 256i32;
pub const LBS_NOREDRAW: i32 = 4i32;
pub const LBS_NOSEL: i32 = 16384i32;
pub const LBS_NOTIFY: i32 = 1i32;
pub const LBS_OWNERDRAWFIXED: i32 = 16i32;
pub const LBS_OWNERDRAWVARIABLE: i32 = 32i32;
pub const LBS_SORT: i32 = 2i32;
pub const LBS_STANDARD: i32 = 10485763i32;
pub const LBS_USETABSTOPS: i32 = 128i32;
pub const LBS_WANTKEYBOARDINPUT: i32 = 1024i32;
pub const LB_ADDFILE: u32 = 406u32;
pub const LB_ADDSTRING: u32 = 384u32;
pub const LB_CTLCODE: i32 = 0i32;
pub const LB_DELETESTRING: u32 = 386u32;
pub const LB_DIR: u32 = 397u32;
pub const LB_ERR: i32 = -1i32;
pub const LB_ERRSPACE: i32 = -2i32;
pub const LB_FINDSTRING: u32 = 399u32;
pub const LB_FINDSTRINGEXACT: u32 = 418u32;
pub const LB_GETANCHORINDEX: u32 = 413u32;
pub const LB_GETCARETINDEX: u32 = 415u32;
pub const LB_GETCOUNT: u32 = 395u32;
pub const LB_GETCURSEL: u32 = 392u32;
pub const LB_GETHORIZONTALEXTENT: u32 = 403u32;
pub const LB_GETITEMDATA: u32 = 409u32;
pub const LB_GETITEMHEIGHT: u32 = 417u32;
pub const LB_GETITEMRECT: u32 = 408u32;
pub const LB_GETLISTBOXINFO: u32 = 434u32;
pub const LB_GETLOCALE: u32 = 422u32;
pub const LB_GETSEL: u32 = 391u32;
pub const LB_GETSELCOUNT: u32 = 400u32;
pub const LB_GETSELITEMS: u32 = 401u32;
pub const LB_GETTEXT: u32 = 393u32;
pub const LB_GETTEXTLEN: u32 = 394u32;
pub const LB_GETTOPINDEX: u32 = 398u32;
pub const LB_INITSTORAGE: u32 = 424u32;
pub const LB_INSERTSTRING: u32 = 385u32;
pub const LB_ITEMFROMPOINT: u32 = 425u32;
pub const LB_MSGMAX: u32 = 435u32;
pub const LB_MULTIPLEADDSTRING: u32 = 433u32;
pub const LB_OKAY: u32 = 0u32;
pub const LB_RESETCONTENT: u32 = 388u32;
pub const LB_SELECTSTRING: u32 = 396u32;
pub const LB_SELITEMRANGE: u32 = 411u32;
pub const LB_SELITEMRANGEEX: u32 = 387u32;
pub const LB_SETANCHORINDEX: u32 = 412u32;
pub const LB_SETCARETINDEX: u32 = 414u32;
pub const LB_SETCOLUMNWIDTH: u32 = 405u32;
pub const LB_SETCOUNT: u32 = 423u32;
pub const LB_SETCURSEL: u32 = 390u32;
pub const LB_SETHORIZONTALEXTENT: u32 = 404u32;
pub const LB_SETITEMDATA: u32 = 410u32;
pub const LB_SETITEMHEIGHT: u32 = 416u32;
pub const LB_SETLOCALE: u32 = 421u32;
pub const LB_SETSEL: u32 = 389u32;
pub const LB_SETTABSTOPS: u32 = 402u32;
pub const LB_SETTOPINDEX: u32 = 407u32;
pub const LLMHF_INJECTED: u32 = 1u32;
pub const LLMHF_LOWER_IL_INJECTED: u32 = 2u32;
pub const LR_COLOR: u32 = 2u32;
pub const MAPVK_VK_TO_CHAR: u32 = 2u32;
pub const MAPVK_VK_TO_VSC: u32 = 0u32;
pub const MAPVK_VK_TO_VSC_EX: u32 = 4u32;
pub const MAPVK_VSC_TO_VK: u32 = 1u32;
pub const MAPVK_VSC_TO_VK_EX: u32 = 3u32;
pub const MAXIMUM_RESERVED_MANIFEST_RESOURCE_ID: u32 = 16u32;
pub const MAX_LOGICALDPIOVERRIDE: u32 = 2u32;
pub const MAX_STR_BLOCKREASON: u32 = 256u32;
pub const MAX_TOUCH_COUNT: u32 = 256u32;
pub const MAX_TOUCH_PREDICTION_FILTER_TAPS: u32 = 3u32;
pub const MA_ACTIVATE: u32 = 1u32;
pub const MA_ACTIVATEANDEAT: u32 = 2u32;
pub const MA_NOACTIVATE: u32 = 3u32;
pub const MA_NOACTIVATEANDEAT: u32 = 4u32;
pub struct MDICREATESTRUCTA {
    pub szClass: ::win32::core::PCSTR,
    pub szTitle: ::win32::core::PCSTR,
    pub hOwner: super::super::Foundation::HANDLE,
    pub x: i32,
    pub y: i32,
    pub cx: i32,
    pub cy: i32,
    pub style: WINDOW_STYLE,
    pub lParam: super::super::Foundation::LPARAM,
}
impl ::core::marker::Copy for MDICREATESTRUCTA {}
impl ::core::clone::Clone for MDICREATESTRUCTA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MDICREATESTRUCTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MDICREATESTRUCTA")
            .field("szClass", &self.szClass)
            .field("szTitle", &self.szTitle)
            .field("hOwner", &self.hOwner)
            .field("x", &self.x)
            .field("y", &self.y)
            .field("cx", &self.cx)
            .field("cy", &self.cy)
            .field("style", &self.style)
            .field("lParam", &self.lParam)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MDICREATESTRUCTA {
    fn eq(&self, other: &Self) -> bool {
        self.szClass == other.szClass
            && self.szTitle == other.szTitle
            && self.hOwner == other.hOwner
            && self.x == other.x
            && self.y == other.y
            && self.cx == other.cx
            && self.cy == other.cy
            && self.style == other.style
            && self.lParam == other.lParam
    }
}
impl ::core::cmp::Eq for MDICREATESTRUCTA {}
pub struct MDICREATESTRUCTW {
    pub szClass: ::win32::core::PCWSTR,
    pub szTitle: ::win32::core::PCWSTR,
    pub hOwner: super::super::Foundation::HANDLE,
    pub x: i32,
    pub y: i32,
    pub cx: i32,
    pub cy: i32,
    pub style: WINDOW_STYLE,
    pub lParam: super::super::Foundation::LPARAM,
}
impl ::core::marker::Copy for MDICREATESTRUCTW {}
impl ::core::clone::Clone for MDICREATESTRUCTW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MDICREATESTRUCTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MDICREATESTRUCTW")
            .field("szClass", &self.szClass)
            .field("szTitle", &self.szTitle)
            .field("hOwner", &self.hOwner)
            .field("x", &self.x)
            .field("y", &self.y)
            .field("cx", &self.cx)
            .field("cy", &self.cy)
            .field("style", &self.style)
            .field("lParam", &self.lParam)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MDICREATESTRUCTW {
    fn eq(&self, other: &Self) -> bool {
        self.szClass == other.szClass
            && self.szTitle == other.szTitle
            && self.hOwner == other.hOwner
            && self.x == other.x
            && self.y == other.y
            && self.cx == other.cx
            && self.cy == other.cy
            && self.style == other.style
            && self.lParam == other.lParam
    }
}
impl ::core::cmp::Eq for MDICREATESTRUCTW {}
pub struct MDINEXTMENU {
    pub hmenuIn: HMENU,
    pub hmenuNext: HMENU,
    pub hwndNext: super::super::Foundation::HWND,
}
impl ::core::marker::Copy for MDINEXTMENU {}
impl ::core::clone::Clone for MDINEXTMENU {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MDINEXTMENU {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MDINEXTMENU")
            .field("hmenuIn", &self.hmenuIn)
            .field("hmenuNext", &self.hmenuNext)
            .field("hwndNext", &self.hwndNext)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MDINEXTMENU {
    fn eq(&self, other: &Self) -> bool {
        self.hmenuIn == other.hmenuIn
            && self.hmenuNext == other.hmenuNext
            && self.hwndNext == other.hwndNext
    }
}
impl ::core::cmp::Eq for MDINEXTMENU {}
pub const MDIS_ALLCHILDSTYLES: u32 = 1u32;
pub struct MENUBARINFO {
    pub cbSize: u32,
    pub rcBar: super::super::Foundation::RECT,
    pub hMenu: HMENU,
    pub hwndMenu: super::super::Foundation::HWND,
    pub _bitfield: i32,
}
impl ::core::marker::Copy for MENUBARINFO {}
impl ::core::clone::Clone for MENUBARINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MENUBARINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MENUBARINFO")
            .field("cbSize", &self.cbSize)
            .field("rcBar", &self.rcBar)
            .field("hMenu", &self.hMenu)
            .field("hwndMenu", &self.hwndMenu)
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MENUBARINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.rcBar == other.rcBar
            && self.hMenu == other.hMenu
            && self.hwndMenu == other.hwndMenu
            && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for MENUBARINFO {}
pub struct MENUGETOBJECTINFO {
    pub dwFlags: MENUGETOBJECTINFO_FLAGS,
    pub uPos: u32,
    pub hmenu: HMENU,
    pub riid: MutPtr<::core::ffi::c_void>,
    pub pvObj: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for MENUGETOBJECTINFO {}
impl ::core::clone::Clone for MENUGETOBJECTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MENUGETOBJECTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MENUGETOBJECTINFO")
            .field("dwFlags", &self.dwFlags)
            .field("uPos", &self.uPos)
            .field("hmenu", &self.hmenu)
            .field("riid", &self.riid)
            .field("pvObj", &self.pvObj)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MENUGETOBJECTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags
            && self.uPos == other.uPos
            && self.hmenu == other.hmenu
            && self.riid == other.riid
            && self.pvObj == other.pvObj
    }
}
impl ::core::cmp::Eq for MENUGETOBJECTINFO {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MENUGETOBJECTINFO_FLAGS(pub u32);
pub const MNGOF_BOTTOMGAP: MENUGETOBJECTINFO_FLAGS = MENUGETOBJECTINFO_FLAGS(2u32);
pub const MNGOF_TOPGAP: MENUGETOBJECTINFO_FLAGS = MENUGETOBJECTINFO_FLAGS(1u32);
impl ::core::marker::Copy for MENUGETOBJECTINFO_FLAGS {}
impl ::core::clone::Clone for MENUGETOBJECTINFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MENUGETOBJECTINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MENUGETOBJECTINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MENUGETOBJECTINFO_FLAGS")
            .field(&self.0)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct MENUINFO {
    pub cbSize: u32,
    pub fMask: MENUINFO_MASK,
    pub dwStyle: MENUINFO_STYLE,
    pub cyMax: u32,
    pub hbrBack: super::super::Graphics::Gdi::HBRUSH,
    pub dwContextHelpID: u32,
    pub dwMenuData: usize,
}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for MENUINFO {}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for MENUINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for MENUINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MENUINFO")
            .field("cbSize", &self.cbSize)
            .field("fMask", &self.fMask)
            .field("dwStyle", &self.dwStyle)
            .field("cyMax", &self.cyMax)
            .field("hbrBack", &self.hbrBack)
            .field("dwContextHelpID", &self.dwContextHelpID)
            .field("dwMenuData", &self.dwMenuData)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for MENUINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.fMask == other.fMask
            && self.dwStyle == other.dwStyle
            && self.cyMax == other.cyMax
            && self.hbrBack == other.hbrBack
            && self.dwContextHelpID == other.dwContextHelpID
            && self.dwMenuData == other.dwMenuData
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for MENUINFO {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MENUINFO_MASK(pub u32);
pub const MIM_APPLYTOSUBMENUS: MENUINFO_MASK = MENUINFO_MASK(2147483648u32);
pub const MIM_BACKGROUND: MENUINFO_MASK = MENUINFO_MASK(2u32);
pub const MIM_HELPID: MENUINFO_MASK = MENUINFO_MASK(4u32);
pub const MIM_MAXHEIGHT: MENUINFO_MASK = MENUINFO_MASK(1u32);
pub const MIM_MENUDATA: MENUINFO_MASK = MENUINFO_MASK(8u32);
pub const MIM_STYLE: MENUINFO_MASK = MENUINFO_MASK(16u32);
impl ::core::marker::Copy for MENUINFO_MASK {}
impl ::core::clone::Clone for MENUINFO_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MENUINFO_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MENUINFO_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MENUINFO_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MENUINFO_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MENUINFO_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MENUINFO_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MENUINFO_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MENUINFO_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MENUINFO_STYLE(pub u32);
pub const MNS_AUTODISMISS: MENUINFO_STYLE = MENUINFO_STYLE(268435456u32);
pub const MNS_CHECKORBMP: MENUINFO_STYLE = MENUINFO_STYLE(67108864u32);
pub const MNS_DRAGDROP: MENUINFO_STYLE = MENUINFO_STYLE(536870912u32);
pub const MNS_MODELESS: MENUINFO_STYLE = MENUINFO_STYLE(1073741824u32);
pub const MNS_NOCHECK: MENUINFO_STYLE = MENUINFO_STYLE(2147483648u32);
pub const MNS_NOTIFYBYPOS: MENUINFO_STYLE = MENUINFO_STYLE(134217728u32);
impl ::core::marker::Copy for MENUINFO_STYLE {}
impl ::core::clone::Clone for MENUINFO_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MENUINFO_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MENUINFO_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MENUINFO_STYLE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MENUINFO_STYLE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MENUINFO_STYLE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MENUINFO_STYLE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MENUINFO_STYLE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MENUINFO_STYLE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct MENUITEMINFOA {
    pub cbSize: u32,
    pub fMask: MENU_ITEM_MASK,
    pub fType: MENU_ITEM_TYPE,
    pub fState: MENU_ITEM_STATE,
    pub wID: u32,
    pub hSubMenu: HMENU,
    pub hbmpChecked: super::super::Graphics::Gdi::HBITMAP,
    pub hbmpUnchecked: super::super::Graphics::Gdi::HBITMAP,
    pub dwItemData: usize,
    pub dwTypeData: ::win32::core::PSTR,
    pub cch: u32,
    pub hbmpItem: super::super::Graphics::Gdi::HBITMAP,
}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for MENUITEMINFOA {}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for MENUITEMINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for MENUITEMINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MENUITEMINFOA")
            .field("cbSize", &self.cbSize)
            .field("fMask", &self.fMask)
            .field("fType", &self.fType)
            .field("fState", &self.fState)
            .field("wID", &self.wID)
            .field("hSubMenu", &self.hSubMenu)
            .field("hbmpChecked", &self.hbmpChecked)
            .field("hbmpUnchecked", &self.hbmpUnchecked)
            .field("dwItemData", &self.dwItemData)
            .field("dwTypeData", &self.dwTypeData)
            .field("cch", &self.cch)
            .field("hbmpItem", &self.hbmpItem)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for MENUITEMINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.fMask == other.fMask
            && self.fType == other.fType
            && self.fState == other.fState
            && self.wID == other.wID
            && self.hSubMenu == other.hSubMenu
            && self.hbmpChecked == other.hbmpChecked
            && self.hbmpUnchecked == other.hbmpUnchecked
            && self.dwItemData == other.dwItemData
            && self.dwTypeData == other.dwTypeData
            && self.cch == other.cch
            && self.hbmpItem == other.hbmpItem
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for MENUITEMINFOA {}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct MENUITEMINFOW {
    pub cbSize: u32,
    pub fMask: MENU_ITEM_MASK,
    pub fType: MENU_ITEM_TYPE,
    pub fState: MENU_ITEM_STATE,
    pub wID: u32,
    pub hSubMenu: HMENU,
    pub hbmpChecked: super::super::Graphics::Gdi::HBITMAP,
    pub hbmpUnchecked: super::super::Graphics::Gdi::HBITMAP,
    pub dwItemData: usize,
    pub dwTypeData: ::win32::core::PWSTR,
    pub cch: u32,
    pub hbmpItem: super::super::Graphics::Gdi::HBITMAP,
}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for MENUITEMINFOW {}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for MENUITEMINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for MENUITEMINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MENUITEMINFOW")
            .field("cbSize", &self.cbSize)
            .field("fMask", &self.fMask)
            .field("fType", &self.fType)
            .field("fState", &self.fState)
            .field("wID", &self.wID)
            .field("hSubMenu", &self.hSubMenu)
            .field("hbmpChecked", &self.hbmpChecked)
            .field("hbmpUnchecked", &self.hbmpUnchecked)
            .field("dwItemData", &self.dwItemData)
            .field("dwTypeData", &self.dwTypeData)
            .field("cch", &self.cch)
            .field("hbmpItem", &self.hbmpItem)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for MENUITEMINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.fMask == other.fMask
            && self.fType == other.fType
            && self.fState == other.fState
            && self.wID == other.wID
            && self.hSubMenu == other.hSubMenu
            && self.hbmpChecked == other.hbmpChecked
            && self.hbmpUnchecked == other.hbmpUnchecked
            && self.dwItemData == other.dwItemData
            && self.dwTypeData == other.dwTypeData
            && self.cch == other.cch
            && self.hbmpItem == other.hbmpItem
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for MENUITEMINFOW {}
pub struct MENUITEMTEMPLATE {
    pub mtOption: u16,
    pub mtID: u16,
    pub mtString: [u16; 1],
}
impl ::core::marker::Copy for MENUITEMTEMPLATE {}
impl ::core::clone::Clone for MENUITEMTEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MENUITEMTEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MENUITEMTEMPLATE")
            .field("mtOption", &self.mtOption)
            .field("mtID", &self.mtID)
            .field("mtString", &self.mtString)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MENUITEMTEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.mtOption == other.mtOption
            && self.mtID == other.mtID
            && self.mtString == other.mtString
    }
}
impl ::core::cmp::Eq for MENUITEMTEMPLATE {}
pub struct MENUITEMTEMPLATEHEADER {
    pub versionNumber: u16,
    pub offset: u16,
}
impl ::core::marker::Copy for MENUITEMTEMPLATEHEADER {}
impl ::core::clone::Clone for MENUITEMTEMPLATEHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MENUITEMTEMPLATEHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MENUITEMTEMPLATEHEADER")
            .field("versionNumber", &self.versionNumber)
            .field("offset", &self.offset)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MENUITEMTEMPLATEHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.versionNumber == other.versionNumber && self.offset == other.offset
    }
}
impl ::core::cmp::Eq for MENUITEMTEMPLATEHEADER {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MENU_ITEM_FLAGS(pub u32);
pub const MF_BYCOMMAND: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(0u32);
pub const MF_BYPOSITION: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(1024u32);
pub const MF_BITMAP: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(4u32);
pub const MF_CHECKED: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(8u32);
pub const MF_DISABLED: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(2u32);
pub const MF_ENABLED: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(0u32);
pub const MF_GRAYED: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(1u32);
pub const MF_MENUBARBREAK: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(32u32);
pub const MF_MENUBREAK: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(64u32);
pub const MF_OWNERDRAW: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(256u32);
pub const MF_POPUP: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(16u32);
pub const MF_SEPARATOR: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(2048u32);
pub const MF_STRING: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(0u32);
pub const MF_UNCHECKED: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(0u32);
pub const MF_INSERT: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(0u32);
pub const MF_CHANGE: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(128u32);
pub const MF_APPEND: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(256u32);
pub const MF_DELETE: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(512u32);
pub const MF_REMOVE: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(4096u32);
pub const MF_USECHECKBITMAPS: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(512u32);
pub const MF_UNHILITE: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(0u32);
pub const MF_HILITE: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(128u32);
pub const MF_DEFAULT: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(4096u32);
pub const MF_SYSMENU: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(8192u32);
pub const MF_HELP: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(16384u32);
pub const MF_RIGHTJUSTIFY: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(16384u32);
pub const MF_MOUSESELECT: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(32768u32);
pub const MF_END: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(128u32);
impl ::core::marker::Copy for MENU_ITEM_FLAGS {}
impl ::core::clone::Clone for MENU_ITEM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MENU_ITEM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MENU_ITEM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MENU_ITEM_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MENU_ITEM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MENU_ITEM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MENU_ITEM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MENU_ITEM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MENU_ITEM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MENU_ITEM_MASK(pub u32);
pub const MIIM_BITMAP: MENU_ITEM_MASK = MENU_ITEM_MASK(128u32);
pub const MIIM_CHECKMARKS: MENU_ITEM_MASK = MENU_ITEM_MASK(8u32);
pub const MIIM_DATA: MENU_ITEM_MASK = MENU_ITEM_MASK(32u32);
pub const MIIM_FTYPE: MENU_ITEM_MASK = MENU_ITEM_MASK(256u32);
pub const MIIM_ID: MENU_ITEM_MASK = MENU_ITEM_MASK(2u32);
pub const MIIM_STATE: MENU_ITEM_MASK = MENU_ITEM_MASK(1u32);
pub const MIIM_STRING: MENU_ITEM_MASK = MENU_ITEM_MASK(64u32);
pub const MIIM_SUBMENU: MENU_ITEM_MASK = MENU_ITEM_MASK(4u32);
pub const MIIM_TYPE: MENU_ITEM_MASK = MENU_ITEM_MASK(16u32);
impl ::core::marker::Copy for MENU_ITEM_MASK {}
impl ::core::clone::Clone for MENU_ITEM_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MENU_ITEM_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MENU_ITEM_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MENU_ITEM_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MENU_ITEM_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MENU_ITEM_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MENU_ITEM_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MENU_ITEM_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MENU_ITEM_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MENU_ITEM_STATE(pub u32);
pub const MFS_GRAYED: MENU_ITEM_STATE = MENU_ITEM_STATE(3u32);
pub const MFS_DISABLED: MENU_ITEM_STATE = MENU_ITEM_STATE(3u32);
pub const MFS_CHECKED: MENU_ITEM_STATE = MENU_ITEM_STATE(8u32);
pub const MFS_HILITE: MENU_ITEM_STATE = MENU_ITEM_STATE(128u32);
pub const MFS_ENABLED: MENU_ITEM_STATE = MENU_ITEM_STATE(0u32);
pub const MFS_UNCHECKED: MENU_ITEM_STATE = MENU_ITEM_STATE(0u32);
pub const MFS_UNHILITE: MENU_ITEM_STATE = MENU_ITEM_STATE(0u32);
pub const MFS_DEFAULT: MENU_ITEM_STATE = MENU_ITEM_STATE(4096u32);
impl ::core::marker::Copy for MENU_ITEM_STATE {}
impl ::core::clone::Clone for MENU_ITEM_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MENU_ITEM_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MENU_ITEM_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MENU_ITEM_STATE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MENU_ITEM_STATE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MENU_ITEM_STATE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MENU_ITEM_STATE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MENU_ITEM_STATE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MENU_ITEM_STATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MENU_ITEM_TYPE(pub u32);
pub const MFT_BITMAP: MENU_ITEM_TYPE = MENU_ITEM_TYPE(4u32);
pub const MFT_MENUBARBREAK: MENU_ITEM_TYPE = MENU_ITEM_TYPE(32u32);
pub const MFT_MENUBREAK: MENU_ITEM_TYPE = MENU_ITEM_TYPE(64u32);
pub const MFT_OWNERDRAW: MENU_ITEM_TYPE = MENU_ITEM_TYPE(256u32);
pub const MFT_RADIOCHECK: MENU_ITEM_TYPE = MENU_ITEM_TYPE(512u32);
pub const MFT_RIGHTJUSTIFY: MENU_ITEM_TYPE = MENU_ITEM_TYPE(16384u32);
pub const MFT_RIGHTORDER: MENU_ITEM_TYPE = MENU_ITEM_TYPE(8192u32);
pub const MFT_SEPARATOR: MENU_ITEM_TYPE = MENU_ITEM_TYPE(2048u32);
pub const MFT_STRING: MENU_ITEM_TYPE = MENU_ITEM_TYPE(0u32);
impl ::core::marker::Copy for MENU_ITEM_TYPE {}
impl ::core::clone::Clone for MENU_ITEM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MENU_ITEM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MENU_ITEM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MENU_ITEM_TYPE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MENU_ITEM_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MENU_ITEM_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MENU_ITEM_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MENU_ITEM_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MENU_ITEM_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MESSAGEBOX_RESULT(pub i32);
pub const IDOK: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(1i32);
pub const IDCANCEL: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(2i32);
pub const IDABORT: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(3i32);
pub const IDRETRY: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(4i32);
pub const IDIGNORE: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(5i32);
pub const IDYES: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(6i32);
pub const IDNO: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(7i32);
pub const IDCLOSE: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(8i32);
pub const IDHELP: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(9i32);
pub const IDTRYAGAIN: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(10i32);
pub const IDCONTINUE: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(11i32);
pub const IDASYNC: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(32001i32);
pub const IDTIMEOUT: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(32000i32);
impl ::core::marker::Copy for MESSAGEBOX_RESULT {}
impl ::core::clone::Clone for MESSAGEBOX_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MESSAGEBOX_RESULT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MESSAGEBOX_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MESSAGEBOX_RESULT").field(&self.0).finish()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MESSAGEBOX_STYLE(pub u32);
pub const MB_ABORTRETRYIGNORE: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(2u32);
pub const MB_CANCELTRYCONTINUE: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(6u32);
pub const MB_HELP: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(16384u32);
pub const MB_OK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(0u32);
pub const MB_OKCANCEL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(1u32);
pub const MB_RETRYCANCEL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(5u32);
pub const MB_YESNO: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(4u32);
pub const MB_YESNOCANCEL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(3u32);
pub const MB_ICONHAND: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(16u32);
pub const MB_ICONQUESTION: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(32u32);
pub const MB_ICONEXCLAMATION: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(48u32);
pub const MB_ICONASTERISK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(64u32);
pub const MB_USERICON: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(128u32);
pub const MB_ICONWARNING: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(48u32);
pub const MB_ICONERROR: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(16u32);
pub const MB_ICONINFORMATION: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(64u32);
pub const MB_ICONSTOP: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(16u32);
pub const MB_DEFBUTTON1: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(0u32);
pub const MB_DEFBUTTON2: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(256u32);
pub const MB_DEFBUTTON3: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(512u32);
pub const MB_DEFBUTTON4: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(768u32);
pub const MB_APPLMODAL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(0u32);
pub const MB_SYSTEMMODAL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(4096u32);
pub const MB_TASKMODAL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(8192u32);
pub const MB_NOFOCUS: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(32768u32);
pub const MB_SETFOREGROUND: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(65536u32);
pub const MB_DEFAULT_DESKTOP_ONLY: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(131072u32);
pub const MB_TOPMOST: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(262144u32);
pub const MB_RIGHT: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(524288u32);
pub const MB_RTLREADING: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(1048576u32);
pub const MB_SERVICE_NOTIFICATION: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(2097152u32);
pub const MB_SERVICE_NOTIFICATION_NT3X: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(262144u32);
pub const MB_TYPEMASK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(15u32);
pub const MB_ICONMASK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(240u32);
pub const MB_DEFMASK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(3840u32);
pub const MB_MODEMASK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(12288u32);
pub const MB_MISCMASK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(49152u32);
impl ::core::marker::Copy for MESSAGEBOX_STYLE {}
impl ::core::clone::Clone for MESSAGEBOX_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MESSAGEBOX_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MESSAGEBOX_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MESSAGEBOX_STYLE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MESSAGEBOX_STYLE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MESSAGEBOX_STYLE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MESSAGEBOX_STYLE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MESSAGEBOX_STYLE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MESSAGEBOX_STYLE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub struct MESSAGE_RESOURCE_BLOCK {
    pub LowId: u32,
    pub HighId: u32,
    pub OffsetToEntries: u32,
}
impl ::core::marker::Copy for MESSAGE_RESOURCE_BLOCK {}
impl ::core::clone::Clone for MESSAGE_RESOURCE_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MESSAGE_RESOURCE_BLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MESSAGE_RESOURCE_BLOCK")
            .field("LowId", &self.LowId)
            .field("HighId", &self.HighId)
            .field("OffsetToEntries", &self.OffsetToEntries)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MESSAGE_RESOURCE_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.LowId == other.LowId
            && self.HighId == other.HighId
            && self.OffsetToEntries == other.OffsetToEntries
    }
}
impl ::core::cmp::Eq for MESSAGE_RESOURCE_BLOCK {}
pub struct MESSAGE_RESOURCE_DATA {
    pub NumberOfBlocks: u32,
    pub Blocks: [MESSAGE_RESOURCE_BLOCK; 1],
}
impl ::core::marker::Copy for MESSAGE_RESOURCE_DATA {}
impl ::core::clone::Clone for MESSAGE_RESOURCE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MESSAGE_RESOURCE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MESSAGE_RESOURCE_DATA")
            .field("NumberOfBlocks", &self.NumberOfBlocks)
            .field("Blocks", &self.Blocks)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MESSAGE_RESOURCE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfBlocks == other.NumberOfBlocks && self.Blocks == other.Blocks
    }
}
impl ::core::cmp::Eq for MESSAGE_RESOURCE_DATA {}
pub struct MESSAGE_RESOURCE_ENTRY {
    pub Length: u16,
    pub Flags: u16,
    pub Text: [u8; 1],
}
impl ::core::marker::Copy for MESSAGE_RESOURCE_ENTRY {}
impl ::core::clone::Clone for MESSAGE_RESOURCE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MESSAGE_RESOURCE_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MESSAGE_RESOURCE_ENTRY")
            .field("Length", &self.Length)
            .field("Flags", &self.Flags)
            .field("Text", &self.Text)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MESSAGE_RESOURCE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.Flags == other.Flags && self.Text == other.Text
    }
}
impl ::core::cmp::Eq for MESSAGE_RESOURCE_ENTRY {}
pub const METRICS_USEDEFAULT: i32 = -1i32;
pub struct MINIMIZEDMETRICS {
    pub cbSize: u32,
    pub iWidth: i32,
    pub iHorzGap: i32,
    pub iVertGap: i32,
    pub iArrange: MINIMIZEDMETRICS_ARRANGE,
}
impl ::core::marker::Copy for MINIMIZEDMETRICS {}
impl ::core::clone::Clone for MINIMIZEDMETRICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MINIMIZEDMETRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MINIMIZEDMETRICS")
            .field("cbSize", &self.cbSize)
            .field("iWidth", &self.iWidth)
            .field("iHorzGap", &self.iHorzGap)
            .field("iVertGap", &self.iVertGap)
            .field("iArrange", &self.iArrange)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MINIMIZEDMETRICS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.iWidth == other.iWidth
            && self.iHorzGap == other.iHorzGap
            && self.iVertGap == other.iVertGap
            && self.iArrange == other.iArrange
    }
}
impl ::core::cmp::Eq for MINIMIZEDMETRICS {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MINIMIZEDMETRICS_ARRANGE(pub i32);
pub const ARW_BOTTOMLEFT: MINIMIZEDMETRICS_ARRANGE = MINIMIZEDMETRICS_ARRANGE(0i32);
pub const ARW_BOTTOMRIGHT: MINIMIZEDMETRICS_ARRANGE = MINIMIZEDMETRICS_ARRANGE(1i32);
pub const ARW_TOPLEFT: MINIMIZEDMETRICS_ARRANGE = MINIMIZEDMETRICS_ARRANGE(2i32);
pub const ARW_TOPRIGHT: MINIMIZEDMETRICS_ARRANGE = MINIMIZEDMETRICS_ARRANGE(3i32);
impl ::core::marker::Copy for MINIMIZEDMETRICS_ARRANGE {}
impl ::core::clone::Clone for MINIMIZEDMETRICS_ARRANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MINIMIZEDMETRICS_ARRANGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MINIMIZEDMETRICS_ARRANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MINIMIZEDMETRICS_ARRANGE")
            .field(&self.0)
            .finish()
    }
}
pub const MINIMUM_RESERVED_MANIFEST_RESOURCE_ID: u32 = 1u32;
pub struct MINMAXINFO {
    pub ptReserved: super::super::Foundation::POINT,
    pub ptMaxSize: super::super::Foundation::POINT,
    pub ptMaxPosition: super::super::Foundation::POINT,
    pub ptMinTrackSize: super::super::Foundation::POINT,
    pub ptMaxTrackSize: super::super::Foundation::POINT,
}
impl ::core::marker::Copy for MINMAXINFO {}
impl ::core::clone::Clone for MINMAXINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MINMAXINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MINMAXINFO")
            .field("ptReserved", &self.ptReserved)
            .field("ptMaxSize", &self.ptMaxSize)
            .field("ptMaxPosition", &self.ptMaxPosition)
            .field("ptMinTrackSize", &self.ptMinTrackSize)
            .field("ptMaxTrackSize", &self.ptMaxTrackSize)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MINMAXINFO {
    fn eq(&self, other: &Self) -> bool {
        self.ptReserved == other.ptReserved
            && self.ptMaxSize == other.ptMaxSize
            && self.ptMaxPosition == other.ptMaxPosition
            && self.ptMinTrackSize == other.ptMinTrackSize
            && self.ptMaxTrackSize == other.ptMaxTrackSize
    }
}
impl ::core::cmp::Eq for MINMAXINFO {}
pub const MIN_LOGICALDPIOVERRIDE: i32 = -2i32;
pub const MKF_AVAILABLE: u32 = 2u32;
pub const MKF_CONFIRMHOTKEY: u32 = 8u32;
pub const MKF_HOTKEYACTIVE: u32 = 4u32;
pub const MKF_HOTKEYSOUND: u32 = 16u32;
pub const MKF_INDICATOR: u32 = 32u32;
pub const MKF_LEFTBUTTONDOWN: u32 = 16777216u32;
pub const MKF_LEFTBUTTONSEL: u32 = 268435456u32;
pub const MKF_MODIFIERS: u32 = 64u32;
pub const MKF_MOUSEKEYSON: u32 = 1u32;
pub const MKF_MOUSEMODE: u32 = 2147483648u32;
pub const MKF_REPLACENUMBERS: u32 = 128u32;
pub const MKF_RIGHTBUTTONDOWN: u32 = 33554432u32;
pub const MKF_RIGHTBUTTONSEL: u32 = 536870912u32;
pub const MK_CONTROL: u32 = 8u32;
pub const MK_LBUTTON: u32 = 1u32;
pub const MK_MBUTTON: u32 = 16u32;
pub const MK_RBUTTON: u32 = 2u32;
pub const MK_SHIFT: u32 = 4u32;
pub const MK_XBUTTON1: u32 = 32u32;
pub const MK_XBUTTON2: u32 = 64u32;
pub const MNC_CLOSE: u32 = 1u32;
pub const MNC_EXECUTE: u32 = 2u32;
pub const MNC_IGNORE: u32 = 0u32;
pub const MNC_SELECT: u32 = 3u32;
pub const MND_CONTINUE: u32 = 0u32;
pub const MND_ENDMENU: u32 = 1u32;
pub const MNGO_NOERROR: u32 = 1u32;
pub const MNGO_NOINTERFACE: u32 = 0u32;
pub const MN_GETHMENU: u32 = 481u32;
pub const MONITORINFOF_PRIMARY: u32 = 1u32;
pub struct MOUSEHOOKSTRUCT {
    pub pt: super::super::Foundation::POINT,
    pub hwnd: super::super::Foundation::HWND,
    pub wHitTestCode: u32,
    pub dwExtraInfo: usize,
}
impl ::core::marker::Copy for MOUSEHOOKSTRUCT {}
impl ::core::clone::Clone for MOUSEHOOKSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MOUSEHOOKSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MOUSEHOOKSTRUCT")
            .field("pt", &self.pt)
            .field("hwnd", &self.hwnd)
            .field("wHitTestCode", &self.wHitTestCode)
            .field("dwExtraInfo", &self.dwExtraInfo)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MOUSEHOOKSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.pt == other.pt
            && self.hwnd == other.hwnd
            && self.wHitTestCode == other.wHitTestCode
            && self.dwExtraInfo == other.dwExtraInfo
    }
}
impl ::core::cmp::Eq for MOUSEHOOKSTRUCT {}
pub struct MOUSEHOOKSTRUCTEX {
    pub __AnonymousBase_winuser_L1166_C46: MOUSEHOOKSTRUCT,
    pub mouseData: MOUSEHOOKSTRUCTEX_MOUSE_DATA,
}
impl ::core::marker::Copy for MOUSEHOOKSTRUCTEX {}
impl ::core::clone::Clone for MOUSEHOOKSTRUCTEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MOUSEHOOKSTRUCTEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MOUSEHOOKSTRUCTEX")
            .field(
                "__AnonymousBase_winuser_L1166_C46",
                &self.__AnonymousBase_winuser_L1166_C46,
            )
            .field("mouseData", &self.mouseData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MOUSEHOOKSTRUCTEX {
    fn eq(&self, other: &Self) -> bool {
        self.__AnonymousBase_winuser_L1166_C46 == other.__AnonymousBase_winuser_L1166_C46
            && self.mouseData == other.mouseData
    }
}
impl ::core::cmp::Eq for MOUSEHOOKSTRUCTEX {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MOUSEHOOKSTRUCTEX_MOUSE_DATA(pub u32);
pub const XBUTTON1: MOUSEHOOKSTRUCTEX_MOUSE_DATA = MOUSEHOOKSTRUCTEX_MOUSE_DATA(1u32);
pub const XBUTTON2: MOUSEHOOKSTRUCTEX_MOUSE_DATA = MOUSEHOOKSTRUCTEX_MOUSE_DATA(2u32);
impl ::core::marker::Copy for MOUSEHOOKSTRUCTEX_MOUSE_DATA {}
impl ::core::clone::Clone for MOUSEHOOKSTRUCTEX_MOUSE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MOUSEHOOKSTRUCTEX_MOUSE_DATA {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MOUSEHOOKSTRUCTEX_MOUSE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MOUSEHOOKSTRUCTEX_MOUSE_DATA")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for MOUSEHOOKSTRUCTEX_MOUSE_DATA {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MOUSEHOOKSTRUCTEX_MOUSE_DATA {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MOUSEHOOKSTRUCTEX_MOUSE_DATA {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MOUSEHOOKSTRUCTEX_MOUSE_DATA {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MOUSEHOOKSTRUCTEX_MOUSE_DATA {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const MOUSEWHEEL_ROUTING_FOCUS: u32 = 0u32;
pub const MOUSEWHEEL_ROUTING_HYBRID: u32 = 1u32;
pub const MOUSEWHEEL_ROUTING_MOUSE_POS: u32 = 2u32;
pub struct MSG {
    pub hwnd: super::super::Foundation::HWND,
    pub message: u32,
    pub wParam: super::super::Foundation::WPARAM,
    pub lParam: super::super::Foundation::LPARAM,
    pub time: u32,
    pub pt: super::super::Foundation::POINT,
}
impl ::core::marker::Copy for MSG {}
impl ::core::clone::Clone for MSG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MSG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSG")
            .field("hwnd", &self.hwnd)
            .field("message", &self.message)
            .field("wParam", &self.wParam)
            .field("lParam", &self.lParam)
            .field("time", &self.time)
            .field("pt", &self.pt)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MSG {
    fn eq(&self, other: &Self) -> bool {
        self.hwnd == other.hwnd
            && self.message == other.message
            && self.wParam == other.wParam
            && self.lParam == other.lParam
            && self.time == other.time
            && self.pt == other.pt
    }
}
impl ::core::cmp::Eq for MSG {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.UI.Shell'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub type MSGBOXCALLBACK =
    ::core::option::Option<unsafe extern "system" fn(lpHelpInfo: MutPtr<super::Shell::HELPINFO>)>;
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.UI.Shell'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct MSGBOXPARAMSA {
    pub cbSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszText: ::win32::core::PCSTR,
    pub lpszCaption: ::win32::core::PCSTR,
    pub dwStyle: MESSAGEBOX_STYLE,
    pub lpszIcon: ::win32::core::PCSTR,
    pub dwContextHelpId: usize,
    pub lpfnMsgBoxCallback: MSGBOXCALLBACK,
    pub dwLanguageId: u32,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.UI.Shell'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for MSGBOXPARAMSA {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.UI.Shell'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for MSGBOXPARAMSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.UI.Shell'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for MSGBOXPARAMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSGBOXPARAMSA")
            .field("cbSize", &self.cbSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hInstance", &self.hInstance)
            .field("lpszText", &self.lpszText)
            .field("lpszCaption", &self.lpszCaption)
            .field("dwStyle", &self.dwStyle)
            .field("lpszIcon", &self.lpszIcon)
            .field("dwContextHelpId", &self.dwContextHelpId)
            .field(
                "lpfnMsgBoxCallback",
                &self.lpfnMsgBoxCallback.map(|f| f as usize),
            )
            .field("dwLanguageId", &self.dwLanguageId)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.UI.Shell'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for MSGBOXPARAMSA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.hwndOwner == other.hwndOwner
            && self.hInstance == other.hInstance
            && self.lpszText == other.lpszText
            && self.lpszCaption == other.lpszCaption
            && self.dwStyle == other.dwStyle
            && self.lpszIcon == other.lpszIcon
            && self.dwContextHelpId == other.dwContextHelpId
            && self.lpfnMsgBoxCallback.map(|f| f as usize)
                == other.lpfnMsgBoxCallback.map(|f| f as usize)
            && self.dwLanguageId == other.dwLanguageId
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.UI.Shell'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for MSGBOXPARAMSA {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.UI.Shell'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct MSGBOXPARAMSW {
    pub cbSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszText: ::win32::core::PCWSTR,
    pub lpszCaption: ::win32::core::PCWSTR,
    pub dwStyle: MESSAGEBOX_STYLE,
    pub lpszIcon: ::win32::core::PCWSTR,
    pub dwContextHelpId: usize,
    pub lpfnMsgBoxCallback: MSGBOXCALLBACK,
    pub dwLanguageId: u32,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.UI.Shell'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for MSGBOXPARAMSW {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.UI.Shell'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for MSGBOXPARAMSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.UI.Shell'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for MSGBOXPARAMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSGBOXPARAMSW")
            .field("cbSize", &self.cbSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hInstance", &self.hInstance)
            .field("lpszText", &self.lpszText)
            .field("lpszCaption", &self.lpszCaption)
            .field("dwStyle", &self.dwStyle)
            .field("lpszIcon", &self.lpszIcon)
            .field("dwContextHelpId", &self.dwContextHelpId)
            .field(
                "lpfnMsgBoxCallback",
                &self.lpfnMsgBoxCallback.map(|f| f as usize),
            )
            .field("dwLanguageId", &self.dwLanguageId)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.UI.Shell'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for MSGBOXPARAMSW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.hwndOwner == other.hwndOwner
            && self.hInstance == other.hInstance
            && self.lpszText == other.lpszText
            && self.lpszCaption == other.lpszCaption
            && self.dwStyle == other.dwStyle
            && self.lpszIcon == other.lpszIcon
            && self.dwContextHelpId == other.dwContextHelpId
            && self.lpfnMsgBoxCallback.map(|f| f as usize)
                == other.lpfnMsgBoxCallback.map(|f| f as usize)
            && self.dwLanguageId == other.dwLanguageId
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.UI.Shell'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for MSGBOXPARAMSW {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MSGFLTINFO_STATUS(pub u32);
pub const MSGFLTINFO_NONE: MSGFLTINFO_STATUS = MSGFLTINFO_STATUS(0u32);
pub const MSGFLTINFO_ALLOWED_HIGHER: MSGFLTINFO_STATUS = MSGFLTINFO_STATUS(3u32);
pub const MSGFLTINFO_ALREADYALLOWED_FORWND: MSGFLTINFO_STATUS = MSGFLTINFO_STATUS(1u32);
pub const MSGFLTINFO_ALREADYDISALLOWED_FORWND: MSGFLTINFO_STATUS = MSGFLTINFO_STATUS(2u32);
impl ::core::marker::Copy for MSGFLTINFO_STATUS {}
impl ::core::clone::Clone for MSGFLTINFO_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSGFLTINFO_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MSGFLTINFO_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSGFLTINFO_STATUS").field(&self.0).finish()
    }
}
pub const MSGF_DIALOGBOX: u32 = 0u32;
pub const MSGF_MAX: u32 = 8u32;
pub const MSGF_MENU: u32 = 2u32;
pub const MSGF_MESSAGEBOX: u32 = 1u32;
pub const MSGF_NEXTWINDOW: u32 = 6u32;
pub const MSGF_SCROLLBAR: u32 = 5u32;
pub const MSGF_USER: u32 = 4096u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS(pub u32);
pub const MWMO_NONE: MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS =
    MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS(0u32);
pub const MWMO_ALERTABLE: MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS =
    MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS(2u32);
pub const MWMO_INPUTAVAILABLE: MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS =
    MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS(4u32);
pub const MWMO_WAITALL: MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS =
    MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS(1u32);
impl ::core::marker::Copy for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {}
impl ::core::clone::Clone for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub struct MSLLHOOKSTRUCT {
    pub pt: super::super::Foundation::POINT,
    pub mouseData: MOUSEHOOKSTRUCTEX_MOUSE_DATA,
    pub flags: u32,
    pub time: u32,
    pub dwExtraInfo: usize,
}
impl ::core::marker::Copy for MSLLHOOKSTRUCT {}
impl ::core::clone::Clone for MSLLHOOKSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MSLLHOOKSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSLLHOOKSTRUCT")
            .field("pt", &self.pt)
            .field("mouseData", &self.mouseData)
            .field("flags", &self.flags)
            .field("time", &self.time)
            .field("dwExtraInfo", &self.dwExtraInfo)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MSLLHOOKSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.pt == other.pt
            && self.mouseData == other.mouseData
            && self.flags == other.flags
            && self.time == other.time
            && self.dwExtraInfo == other.dwExtraInfo
    }
}
impl ::core::cmp::Eq for MSLLHOOKSTRUCT {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MrmDumpType(pub i32);
pub const MrmDumpType_Basic: MrmDumpType = MrmDumpType(0i32);
pub const MrmDumpType_Detailed: MrmDumpType = MrmDumpType(1i32);
pub const MrmDumpType_Schema: MrmDumpType = MrmDumpType(2i32);
impl ::core::marker::Copy for MrmDumpType {}
impl ::core::clone::Clone for MrmDumpType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MrmDumpType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MrmDumpType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MrmDumpType").field(&self.0).finish()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MrmIndexerFlags(pub i32);
pub const MrmIndexerFlagsNone: MrmIndexerFlags = MrmIndexerFlags(0i32);
pub const MrmIndexerFlagsAutoMerge: MrmIndexerFlags = MrmIndexerFlags(1i32);
pub const MrmIndexerFlagsCreateContentChecksum: MrmIndexerFlags = MrmIndexerFlags(2i32);
impl ::core::marker::Copy for MrmIndexerFlags {}
impl ::core::clone::Clone for MrmIndexerFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MrmIndexerFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MrmIndexerFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MrmIndexerFlags").field(&self.0).finish()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MrmPackagingMode(pub i32);
pub const MrmPackagingModeStandaloneFile: MrmPackagingMode = MrmPackagingMode(0i32);
pub const MrmPackagingModeAutoSplit: MrmPackagingMode = MrmPackagingMode(1i32);
pub const MrmPackagingModeResourcePack: MrmPackagingMode = MrmPackagingMode(2i32);
impl ::core::marker::Copy for MrmPackagingMode {}
impl ::core::clone::Clone for MrmPackagingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MrmPackagingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MrmPackagingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MrmPackagingMode").field(&self.0).finish()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MrmPackagingOptions(pub i32);
pub const MrmPackagingOptionsNone: MrmPackagingOptions = MrmPackagingOptions(0i32);
pub const MrmPackagingOptionsOmitSchemaFromResourcePacks: MrmPackagingOptions =
    MrmPackagingOptions(1i32);
pub const MrmPackagingOptionsSplitLanguageVariants: MrmPackagingOptions = MrmPackagingOptions(2i32);
impl ::core::marker::Copy for MrmPackagingOptions {}
impl ::core::clone::Clone for MrmPackagingOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MrmPackagingOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MrmPackagingOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MrmPackagingOptions").field(&self.0).finish()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MrmPlatformVersion(pub i32);
pub const MrmPlatformVersion_Default: MrmPlatformVersion = MrmPlatformVersion(0i32);
pub const MrmPlatformVersion_Windows10_0_0_0: MrmPlatformVersion = MrmPlatformVersion(17432576i32);
pub const MrmPlatformVersion_Windows10_0_0_5: MrmPlatformVersion = MrmPlatformVersion(17432581i32);
impl ::core::marker::Copy for MrmPlatformVersion {}
impl ::core::clone::Clone for MrmPlatformVersion {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MrmPlatformVersion {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MrmPlatformVersion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MrmPlatformVersion").field(&self.0).finish()
    }
}
pub struct MrmResourceIndexerHandle {
    pub handle: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for MrmResourceIndexerHandle {}
impl ::core::clone::Clone for MrmResourceIndexerHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MrmResourceIndexerHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MrmResourceIndexerHandle")
            .field("handle", &self.handle)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MrmResourceIndexerHandle {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
impl ::core::cmp::Eq for MrmResourceIndexerHandle {}
pub struct MrmResourceIndexerMessage {
    pub severity: MrmResourceIndexerMessageSeverity,
    pub id: u32,
    pub text: ::win32::core::PCWSTR,
}
impl ::core::marker::Copy for MrmResourceIndexerMessage {}
impl ::core::clone::Clone for MrmResourceIndexerMessage {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MrmResourceIndexerMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MrmResourceIndexerMessage")
            .field("severity", &self.severity)
            .field("id", &self.id)
            .field("text", &self.text)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MrmResourceIndexerMessage {
    fn eq(&self, other: &Self) -> bool {
        self.severity == other.severity && self.id == other.id && self.text == other.text
    }
}
impl ::core::cmp::Eq for MrmResourceIndexerMessage {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MrmResourceIndexerMessageSeverity(pub i32);
pub const MrmResourceIndexerMessageSeverityVerbose: MrmResourceIndexerMessageSeverity =
    MrmResourceIndexerMessageSeverity(0i32);
pub const MrmResourceIndexerMessageSeverityInfo: MrmResourceIndexerMessageSeverity =
    MrmResourceIndexerMessageSeverity(1i32);
pub const MrmResourceIndexerMessageSeverityWarning: MrmResourceIndexerMessageSeverity =
    MrmResourceIndexerMessageSeverity(2i32);
pub const MrmResourceIndexerMessageSeverityError: MrmResourceIndexerMessageSeverity =
    MrmResourceIndexerMessageSeverity(3i32);
impl ::core::marker::Copy for MrmResourceIndexerMessageSeverity {}
impl ::core::clone::Clone for MrmResourceIndexerMessageSeverity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MrmResourceIndexerMessageSeverity {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MrmResourceIndexerMessageSeverity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MrmResourceIndexerMessageSeverity")
            .field(&self.0)
            .finish()
    }
}
pub type NAMEENUMPROCA = ::core::option::Option<
    unsafe extern "system" fn(
        param0: ::win32::core::PCSTR,
        param1: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::BOOL,
>;
pub type NAMEENUMPROCW = ::core::option::Option<
    unsafe extern "system" fn(
        param0: ::win32::core::PCWSTR,
        param1: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::BOOL,
>;
pub struct NCCALCSIZE_PARAMS {
    pub rgrc: [super::super::Foundation::RECT; 3],
    pub lppos: MutPtr<WINDOWPOS>,
}
impl ::core::marker::Copy for NCCALCSIZE_PARAMS {}
impl ::core::clone::Clone for NCCALCSIZE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NCCALCSIZE_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NCCALCSIZE_PARAMS")
            .field("rgrc", &self.rgrc)
            .field("lppos", &self.lppos)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NCCALCSIZE_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.rgrc == other.rgrc && self.lppos == other.lppos
    }
}
impl ::core::cmp::Eq for NCCALCSIZE_PARAMS {}
pub const NFR_ANSI: u32 = 1u32;
pub const NFR_UNICODE: u32 = 2u32;
pub const NF_QUERY: u32 = 3u32;
pub const NF_REQUERY: u32 = 4u32;
pub const NID_EXTERNAL_PEN: u32 = 8u32;
pub const NID_EXTERNAL_TOUCH: u32 = 2u32;
pub const NID_INTEGRATED_PEN: u32 = 4u32;
pub const NID_INTEGRATED_TOUCH: u32 = 1u32;
pub const NID_MULTI_INPUT: u32 = 64u32;
pub const NID_READY: u32 = 128u32;
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct NONCLIENTMETRICSA {
    pub cbSize: u32,
    pub iBorderWidth: i32,
    pub iScrollWidth: i32,
    pub iScrollHeight: i32,
    pub iCaptionWidth: i32,
    pub iCaptionHeight: i32,
    pub lfCaptionFont: super::super::Graphics::Gdi::LOGFONTA,
    pub iSmCaptionWidth: i32,
    pub iSmCaptionHeight: i32,
    pub lfSmCaptionFont: super::super::Graphics::Gdi::LOGFONTA,
    pub iMenuWidth: i32,
    pub iMenuHeight: i32,
    pub lfMenuFont: super::super::Graphics::Gdi::LOGFONTA,
    pub lfStatusFont: super::super::Graphics::Gdi::LOGFONTA,
    pub lfMessageFont: super::super::Graphics::Gdi::LOGFONTA,
    pub iPaddedBorderWidth: i32,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for NONCLIENTMETRICSA {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for NONCLIENTMETRICSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for NONCLIENTMETRICSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NONCLIENTMETRICSA")
            .field("cbSize", &self.cbSize)
            .field("iBorderWidth", &self.iBorderWidth)
            .field("iScrollWidth", &self.iScrollWidth)
            .field("iScrollHeight", &self.iScrollHeight)
            .field("iCaptionWidth", &self.iCaptionWidth)
            .field("iCaptionHeight", &self.iCaptionHeight)
            .field("lfCaptionFont", &self.lfCaptionFont)
            .field("iSmCaptionWidth", &self.iSmCaptionWidth)
            .field("iSmCaptionHeight", &self.iSmCaptionHeight)
            .field("lfSmCaptionFont", &self.lfSmCaptionFont)
            .field("iMenuWidth", &self.iMenuWidth)
            .field("iMenuHeight", &self.iMenuHeight)
            .field("lfMenuFont", &self.lfMenuFont)
            .field("lfStatusFont", &self.lfStatusFont)
            .field("lfMessageFont", &self.lfMessageFont)
            .field("iPaddedBorderWidth", &self.iPaddedBorderWidth)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for NONCLIENTMETRICSA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.iBorderWidth == other.iBorderWidth
            && self.iScrollWidth == other.iScrollWidth
            && self.iScrollHeight == other.iScrollHeight
            && self.iCaptionWidth == other.iCaptionWidth
            && self.iCaptionHeight == other.iCaptionHeight
            && self.lfCaptionFont == other.lfCaptionFont
            && self.iSmCaptionWidth == other.iSmCaptionWidth
            && self.iSmCaptionHeight == other.iSmCaptionHeight
            && self.lfSmCaptionFont == other.lfSmCaptionFont
            && self.iMenuWidth == other.iMenuWidth
            && self.iMenuHeight == other.iMenuHeight
            && self.lfMenuFont == other.lfMenuFont
            && self.lfStatusFont == other.lfStatusFont
            && self.lfMessageFont == other.lfMessageFont
            && self.iPaddedBorderWidth == other.iPaddedBorderWidth
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for NONCLIENTMETRICSA {}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct NONCLIENTMETRICSW {
    pub cbSize: u32,
    pub iBorderWidth: i32,
    pub iScrollWidth: i32,
    pub iScrollHeight: i32,
    pub iCaptionWidth: i32,
    pub iCaptionHeight: i32,
    pub lfCaptionFont: super::super::Graphics::Gdi::LOGFONTW,
    pub iSmCaptionWidth: i32,
    pub iSmCaptionHeight: i32,
    pub lfSmCaptionFont: super::super::Graphics::Gdi::LOGFONTW,
    pub iMenuWidth: i32,
    pub iMenuHeight: i32,
    pub lfMenuFont: super::super::Graphics::Gdi::LOGFONTW,
    pub lfStatusFont: super::super::Graphics::Gdi::LOGFONTW,
    pub lfMessageFont: super::super::Graphics::Gdi::LOGFONTW,
    pub iPaddedBorderWidth: i32,
}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for NONCLIENTMETRICSW {}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for NONCLIENTMETRICSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for NONCLIENTMETRICSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NONCLIENTMETRICSW")
            .field("cbSize", &self.cbSize)
            .field("iBorderWidth", &self.iBorderWidth)
            .field("iScrollWidth", &self.iScrollWidth)
            .field("iScrollHeight", &self.iScrollHeight)
            .field("iCaptionWidth", &self.iCaptionWidth)
            .field("iCaptionHeight", &self.iCaptionHeight)
            .field("lfCaptionFont", &self.lfCaptionFont)
            .field("iSmCaptionWidth", &self.iSmCaptionWidth)
            .field("iSmCaptionHeight", &self.iSmCaptionHeight)
            .field("lfSmCaptionFont", &self.lfSmCaptionFont)
            .field("iMenuWidth", &self.iMenuWidth)
            .field("iMenuHeight", &self.iMenuHeight)
            .field("lfMenuFont", &self.lfMenuFont)
            .field("lfStatusFont", &self.lfStatusFont)
            .field("lfMessageFont", &self.lfMessageFont)
            .field("iPaddedBorderWidth", &self.iPaddedBorderWidth)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for NONCLIENTMETRICSW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.iBorderWidth == other.iBorderWidth
            && self.iScrollWidth == other.iScrollWidth
            && self.iScrollHeight == other.iScrollHeight
            && self.iCaptionWidth == other.iCaptionWidth
            && self.iCaptionHeight == other.iCaptionHeight
            && self.lfCaptionFont == other.lfCaptionFont
            && self.iSmCaptionWidth == other.iSmCaptionWidth
            && self.iSmCaptionHeight == other.iSmCaptionHeight
            && self.lfSmCaptionFont == other.lfSmCaptionFont
            && self.iMenuWidth == other.iMenuWidth
            && self.iMenuHeight == other.iMenuHeight
            && self.lfMenuFont == other.lfMenuFont
            && self.lfStatusFont == other.lfStatusFont
            && self.lfMessageFont == other.lfMessageFont
            && self.iPaddedBorderWidth == other.iPaddedBorderWidth
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for NONCLIENTMETRICSW {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct OBJECT_IDENTIFIER(pub i32);
pub const OBJID_WINDOW: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(0i32);
pub const OBJID_SYSMENU: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-1i32);
pub const OBJID_TITLEBAR: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-2i32);
pub const OBJID_MENU: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-3i32);
pub const OBJID_CLIENT: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-4i32);
pub const OBJID_VSCROLL: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-5i32);
pub const OBJID_HSCROLL: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-6i32);
pub const OBJID_SIZEGRIP: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-7i32);
pub const OBJID_CARET: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-8i32);
pub const OBJID_CURSOR: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-9i32);
pub const OBJID_ALERT: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-10i32);
pub const OBJID_SOUND: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-11i32);
pub const OBJID_QUERYCLASSNAMEIDX: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-12i32);
pub const OBJID_NATIVEOM: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER(-16i32);
impl ::core::marker::Copy for OBJECT_IDENTIFIER {}
impl ::core::clone::Clone for OBJECT_IDENTIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OBJECT_IDENTIFIER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OBJECT_IDENTIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OBJECT_IDENTIFIER").field(&self.0).finish()
    }
}
pub const OBM_BTNCORNERS: u32 = 32758u32;
pub const OBM_BTSIZE: u32 = 32761u32;
pub const OBM_CHECK: u32 = 32760u32;
pub const OBM_CHECKBOXES: u32 = 32759u32;
pub const OBM_CLOSE: u32 = 32754u32;
pub const OBM_COMBO: u32 = 32738u32;
pub const OBM_DNARROW: u32 = 32752u32;
pub const OBM_DNARROWD: u32 = 32742u32;
pub const OBM_DNARROWI: u32 = 32736u32;
pub const OBM_LFARROW: u32 = 32750u32;
pub const OBM_LFARROWD: u32 = 32740u32;
pub const OBM_LFARROWI: u32 = 32734u32;
pub const OBM_MNARROW: u32 = 32739u32;
pub const OBM_OLD_CLOSE: u32 = 32767u32;
pub const OBM_OLD_DNARROW: u32 = 32764u32;
pub const OBM_OLD_LFARROW: u32 = 32762u32;
pub const OBM_OLD_REDUCE: u32 = 32757u32;
pub const OBM_OLD_RESTORE: u32 = 32755u32;
pub const OBM_OLD_RGARROW: u32 = 32763u32;
pub const OBM_OLD_UPARROW: u32 = 32765u32;
pub const OBM_OLD_ZOOM: u32 = 32756u32;
pub const OBM_REDUCE: u32 = 32749u32;
pub const OBM_REDUCED: u32 = 32746u32;
pub const OBM_RESTORE: u32 = 32747u32;
pub const OBM_RESTORED: u32 = 32744u32;
pub const OBM_RGARROW: u32 = 32751u32;
pub const OBM_RGARROWD: u32 = 32741u32;
pub const OBM_RGARROWI: u32 = 32735u32;
pub const OBM_SIZE: u32 = 32766u32;
pub const OBM_UPARROW: u32 = 32753u32;
pub const OBM_UPARROWD: u32 = 32743u32;
pub const OBM_UPARROWI: u32 = 32737u32;
pub const OBM_ZOOM: u32 = 32748u32;
pub const OBM_ZOOMD: u32 = 32745u32;
pub const OCR_ICOCUR: u32 = 32647u32;
pub const OCR_ICON: u32 = 32641u32;
pub const OCR_SIZE: u32 = 32640u32;
pub const ODA_DRAWENTIRE: u32 = 1u32;
pub const ODA_FOCUS: u32 = 4u32;
pub const ODA_SELECT: u32 = 2u32;
pub const ODS_CHECKED: u32 = 8u32;
pub const ODS_COMBOBOXEDIT: u32 = 4096u32;
pub const ODS_DEFAULT: u32 = 32u32;
pub const ODS_DISABLED: u32 = 4u32;
pub const ODS_FOCUS: u32 = 16u32;
pub const ODS_GRAYED: u32 = 2u32;
pub const ODS_HOTLIGHT: u32 = 64u32;
pub const ODS_INACTIVE: u32 = 128u32;
pub const ODS_NOACCEL: u32 = 256u32;
pub const ODS_NOFOCUSRECT: u32 = 512u32;
pub const ODS_SELECTED: u32 = 1u32;
pub const OIC_BANG: u32 = 32515u32;
pub const OIC_ERROR: u32 = 32513u32;
pub const OIC_HAND: u32 = 32513u32;
pub const OIC_INFORMATION: u32 = 32516u32;
pub const OIC_NOTE: u32 = 32516u32;
pub const OIC_QUES: u32 = 32514u32;
pub const OIC_SAMPLE: u32 = 32512u32;
pub const OIC_SHIELD: u32 = 32518u32;
pub const OIC_WARNING: u32 = 32515u32;
pub const OIC_WINLOGO: u32 = 32517u32;
pub const ORD_LANGDRIVER: u32 = 1u32;
pub const PA_ACTIVATE: u32 = 1u32;
pub const PA_NOACTIVATE: u32 = 3u32;
pub const PBTF_APMRESUMEFROMFAILURE: u32 = 1u32;
pub const PBT_APMBATTERYLOW: u32 = 9u32;
pub const PBT_APMOEMEVENT: u32 = 11u32;
pub const PBT_APMPOWERSTATUSCHANGE: u32 = 10u32;
pub const PBT_APMQUERYSTANDBY: u32 = 1u32;
pub const PBT_APMQUERYSTANDBYFAILED: u32 = 3u32;
pub const PBT_APMQUERYSUSPEND: u32 = 0u32;
pub const PBT_APMQUERYSUSPENDFAILED: u32 = 2u32;
pub const PBT_APMRESUMEAUTOMATIC: u32 = 18u32;
pub const PBT_APMRESUMECRITICAL: u32 = 6u32;
pub const PBT_APMRESUMESTANDBY: u32 = 8u32;
pub const PBT_APMRESUMESUSPEND: u32 = 7u32;
pub const PBT_APMSTANDBY: u32 = 5u32;
pub const PBT_APMSUSPEND: u32 = 4u32;
pub const PBT_POWERSETTINGCHANGE: u32 = 32787u32;
pub const PDC_ARRIVAL: u32 = 1u32;
pub const PDC_MAPPING_CHANGE: u32 = 256u32;
pub const PDC_MODE_ASPECTRATIOPRESERVED: u32 = 2048u32;
pub const PDC_MODE_CENTERED: u32 = 128u32;
pub const PDC_MODE_DEFAULT: u32 = 64u32;
pub const PDC_ORIENTATION_0: u32 = 4u32;
pub const PDC_ORIENTATION_180: u32 = 16u32;
pub const PDC_ORIENTATION_270: u32 = 32u32;
pub const PDC_ORIENTATION_90: u32 = 8u32;
pub const PDC_ORIGIN: u32 = 1024u32;
pub const PDC_REMOVAL: u32 = 2u32;
pub const PDC_RESOLUTION: u32 = 512u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PEEK_MESSAGE_REMOVE_TYPE(pub u32);
pub const PM_NOREMOVE: PEEK_MESSAGE_REMOVE_TYPE = PEEK_MESSAGE_REMOVE_TYPE(0u32);
pub const PM_REMOVE: PEEK_MESSAGE_REMOVE_TYPE = PEEK_MESSAGE_REMOVE_TYPE(1u32);
pub const PM_NOYIELD: PEEK_MESSAGE_REMOVE_TYPE = PEEK_MESSAGE_REMOVE_TYPE(2u32);
pub const PM_QS_INPUT: PEEK_MESSAGE_REMOVE_TYPE = PEEK_MESSAGE_REMOVE_TYPE(67567616u32);
pub const PM_QS_POSTMESSAGE: PEEK_MESSAGE_REMOVE_TYPE = PEEK_MESSAGE_REMOVE_TYPE(9961472u32);
pub const PM_QS_PAINT: PEEK_MESSAGE_REMOVE_TYPE = PEEK_MESSAGE_REMOVE_TYPE(2097152u32);
pub const PM_QS_SENDMESSAGE: PEEK_MESSAGE_REMOVE_TYPE = PEEK_MESSAGE_REMOVE_TYPE(4194304u32);
impl ::core::marker::Copy for PEEK_MESSAGE_REMOVE_TYPE {}
impl ::core::clone::Clone for PEEK_MESSAGE_REMOVE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEEK_MESSAGE_REMOVE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEEK_MESSAGE_REMOVE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEEK_MESSAGE_REMOVE_TYPE")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for PEEK_MESSAGE_REMOVE_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PEEK_MESSAGE_REMOVE_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PEEK_MESSAGE_REMOVE_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PEEK_MESSAGE_REMOVE_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PEEK_MESSAGE_REMOVE_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PENARBITRATIONTYPE_FIS: u32 = 2u32;
pub const PENARBITRATIONTYPE_MAX: u32 = 4u32;
pub const PENARBITRATIONTYPE_NONE: u32 = 0u32;
pub const PENARBITRATIONTYPE_SPT: u32 = 3u32;
pub const PENARBITRATIONTYPE_WIN8: u32 = 1u32;
pub const PENVISUALIZATION_CURSOR: u32 = 32u32;
pub const PENVISUALIZATION_DOUBLETAP: u32 = 2u32;
pub const PENVISUALIZATION_OFF: u32 = 0u32;
pub const PENVISUALIZATION_ON: u32 = 35u32;
pub const PENVISUALIZATION_TAP: u32 = 1u32;
pub const PEN_FLAG_BARREL: u32 = 1u32;
pub const PEN_FLAG_ERASER: u32 = 4u32;
pub const PEN_FLAG_INVERTED: u32 = 2u32;
pub const PEN_FLAG_NONE: u32 = 0u32;
pub const PEN_MASK_NONE: u32 = 0u32;
pub const PEN_MASK_PRESSURE: u32 = 1u32;
pub const PEN_MASK_ROTATION: u32 = 2u32;
pub const PEN_MASK_TILT_X: u32 = 4u32;
pub const PEN_MASK_TILT_Y: u32 = 8u32;
pub const PMB_ACTIVE: u32 = 1u32;
pub const POINTER_DEVICE_PRODUCT_STRING_MAX: u32 = 520u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct POINTER_INPUT_TYPE(pub i32);
pub const PT_POINTER: POINTER_INPUT_TYPE = POINTER_INPUT_TYPE(1i32);
pub const PT_TOUCH: POINTER_INPUT_TYPE = POINTER_INPUT_TYPE(2i32);
pub const PT_PEN: POINTER_INPUT_TYPE = POINTER_INPUT_TYPE(3i32);
pub const PT_MOUSE: POINTER_INPUT_TYPE = POINTER_INPUT_TYPE(4i32);
pub const PT_TOUCHPAD: POINTER_INPUT_TYPE = POINTER_INPUT_TYPE(5i32);
impl ::core::marker::Copy for POINTER_INPUT_TYPE {}
impl ::core::clone::Clone for POINTER_INPUT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for POINTER_INPUT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POINTER_INPUT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POINTER_INPUT_TYPE").field(&self.0).finish()
    }
}
pub const POINTER_MESSAGE_FLAG_CANCELED: u32 = 32768u32;
pub const POINTER_MESSAGE_FLAG_CONFIDENCE: u32 = 16384u32;
pub const POINTER_MESSAGE_FLAG_FIFTHBUTTON: u32 = 256u32;
pub const POINTER_MESSAGE_FLAG_FIRSTBUTTON: u32 = 16u32;
pub const POINTER_MESSAGE_FLAG_FOURTHBUTTON: u32 = 128u32;
pub const POINTER_MESSAGE_FLAG_INCONTACT: u32 = 4u32;
pub const POINTER_MESSAGE_FLAG_INRANGE: u32 = 2u32;
pub const POINTER_MESSAGE_FLAG_NEW: u32 = 1u32;
pub const POINTER_MESSAGE_FLAG_PRIMARY: u32 = 8192u32;
pub const POINTER_MESSAGE_FLAG_SECONDBUTTON: u32 = 32u32;
pub const POINTER_MESSAGE_FLAG_THIRDBUTTON: u32 = 64u32;
pub const POINTER_MOD_CTRL: u32 = 8u32;
pub const POINTER_MOD_SHIFT: u32 = 4u32;
pub type PREGISTERCLASSNAMEW = ::core::option::Option<
    unsafe extern "system" fn(param0: ::win32::core::PCWSTR) -> super::super::Foundation::BOOLEAN,
>;
pub const PRF_CHECKVISIBLE: i32 = 1i32;
pub const PRF_CHILDREN: i32 = 16i32;
pub const PRF_CLIENT: i32 = 4i32;
pub const PRF_ERASEBKGND: i32 = 8i32;
pub const PRF_NONCLIENT: i32 = 2i32;
pub const PRF_OWNED: i32 = 32i32;
pub type PROPENUMPROCA = ::core::option::Option<
    unsafe extern "system" fn(
        param0: super::super::Foundation::HWND,
        param1: ::win32::core::PCSTR,
        param2: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL,
>;
pub type PROPENUMPROCEXA = ::core::option::Option<
    unsafe extern "system" fn(
        param0: super::super::Foundation::HWND,
        param1: ::win32::core::PCSTR,
        param2: super::super::Foundation::HANDLE,
        param3: usize,
    ) -> super::super::Foundation::BOOL,
>;
pub type PROPENUMPROCEXW = ::core::option::Option<
    unsafe extern "system" fn(
        param0: super::super::Foundation::HWND,
        param1: ::win32::core::PCWSTR,
        param2: super::super::Foundation::HANDLE,
        param3: usize,
    ) -> super::super::Foundation::BOOL,
>;
pub type PROPENUMPROCW = ::core::option::Option<
    unsafe extern "system" fn(
        param0: super::super::Foundation::HWND,
        param1: ::win32::core::PCWSTR,
        param2: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL,
>;
pub const PWR_CRITICALRESUME: u32 = 3u32;
pub const PWR_FAIL: i32 = -1i32;
pub const PWR_OK: u32 = 1u32;
pub const PWR_SUSPENDREQUEST: u32 = 1u32;
pub const PWR_SUSPENDRESUME: u32 = 2u32;
pub const PW_RENDERFULLCONTENT: u32 = 2u32;
pub const QS_POINTER: u32 = 4096u32;
pub const QS_TOUCH: u32 = 2048u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct QUEUE_STATUS_FLAGS(pub u32);
pub const QS_ALLEVENTS: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(1215u32);
pub const QS_ALLINPUT: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(1279u32);
pub const QS_ALLPOSTMESSAGE: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(256u32);
pub const QS_HOTKEY: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(128u32);
pub const QS_INPUT: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(1031u32);
pub const QS_KEY: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(1u32);
pub const QS_MOUSE: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(6u32);
pub const QS_MOUSEBUTTON: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(4u32);
pub const QS_MOUSEMOVE: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(2u32);
pub const QS_PAINT: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(32u32);
pub const QS_POSTMESSAGE: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(8u32);
pub const QS_RAWINPUT: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(1024u32);
pub const QS_SENDMESSAGE: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(64u32);
pub const QS_TIMER: QUEUE_STATUS_FLAGS = QUEUE_STATUS_FLAGS(16u32);
impl ::core::marker::Copy for QUEUE_STATUS_FLAGS {}
impl ::core::clone::Clone for QUEUE_STATUS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for QUEUE_STATUS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for QUEUE_STATUS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUEUE_STATUS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for QUEUE_STATUS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for QUEUE_STATUS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for QUEUE_STATUS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for QUEUE_STATUS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for QUEUE_STATUS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const RES_CURSOR: u32 = 2u32;
pub const RES_ICON: u32 = 1u32;
pub const RIDEV_EXMODEMASK: u32 = 240u32;
pub const RIM_INPUT: u32 = 0u32;
pub const RIM_INPUTSINK: u32 = 1u32;
pub const RIM_TYPEMAX: u32 = 2u32;
pub const RI_KEY_BREAK: u32 = 1u32;
pub const RI_KEY_E0: u32 = 2u32;
pub const RI_KEY_E1: u32 = 4u32;
pub const RI_KEY_MAKE: u32 = 0u32;
pub const RI_KEY_TERMSRV_SET_LED: u32 = 8u32;
pub const RI_KEY_TERMSRV_SHADOW: u32 = 16u32;
pub const RI_MOUSE_BUTTON_1_DOWN: u32 = 1u32;
pub const RI_MOUSE_BUTTON_1_UP: u32 = 2u32;
pub const RI_MOUSE_BUTTON_2_DOWN: u32 = 4u32;
pub const RI_MOUSE_BUTTON_2_UP: u32 = 8u32;
pub const RI_MOUSE_BUTTON_3_DOWN: u32 = 16u32;
pub const RI_MOUSE_BUTTON_3_UP: u32 = 32u32;
pub const RI_MOUSE_BUTTON_4_DOWN: u32 = 64u32;
pub const RI_MOUSE_BUTTON_4_UP: u32 = 128u32;
pub const RI_MOUSE_BUTTON_5_DOWN: u32 = 256u32;
pub const RI_MOUSE_BUTTON_5_UP: u32 = 512u32;
pub const RI_MOUSE_HWHEEL: u32 = 2048u32;
pub const RI_MOUSE_LEFT_BUTTON_DOWN: u32 = 1u32;
pub const RI_MOUSE_LEFT_BUTTON_UP: u32 = 2u32;
pub const RI_MOUSE_MIDDLE_BUTTON_DOWN: u32 = 16u32;
pub const RI_MOUSE_MIDDLE_BUTTON_UP: u32 = 32u32;
pub const RI_MOUSE_RIGHT_BUTTON_DOWN: u32 = 4u32;
pub const RI_MOUSE_RIGHT_BUTTON_UP: u32 = 8u32;
pub const RI_MOUSE_WHEEL: u32 = 1024u32;
pub const RT_ACCELERATOR: ::win32::core::PCWSTR = ::win32::core::PCWSTR(9i32 as _);
pub const RT_ANICURSOR: ::win32::core::PCWSTR = ::win32::core::PCWSTR(21i32 as _);
pub const RT_ANIICON: ::win32::core::PCWSTR = ::win32::core::PCWSTR(22i32 as _);
pub const RT_BITMAP: ::win32::core::PCWSTR = ::win32::core::PCWSTR(2i32 as _);
pub const RT_CURSOR: ::win32::core::PCWSTR = ::win32::core::PCWSTR(1i32 as _);
pub const RT_DIALOG: ::win32::core::PCWSTR = ::win32::core::PCWSTR(5i32 as _);
pub const RT_DLGINCLUDE: ::win32::core::PCWSTR = ::win32::core::PCWSTR(17i32 as _);
pub const RT_FONT: ::win32::core::PCWSTR = ::win32::core::PCWSTR(8i32 as _);
pub const RT_FONTDIR: ::win32::core::PCWSTR = ::win32::core::PCWSTR(7i32 as _);
pub const RT_HTML: ::win32::core::PCWSTR = ::win32::core::PCWSTR(23i32 as _);
pub const RT_ICON: ::win32::core::PCWSTR = ::win32::core::PCWSTR(3i32 as _);
pub const RT_MANIFEST: u32 = 24u32;
pub const RT_MENU: ::win32::core::PCWSTR = ::win32::core::PCWSTR(4i32 as _);
pub const RT_MESSAGETABLE: ::win32::core::PCWSTR = ::win32::core::PCWSTR(11i32 as _);
pub const RT_PLUGPLAY: ::win32::core::PCWSTR = ::win32::core::PCWSTR(19i32 as _);
pub const RT_VERSION: ::win32::core::PCWSTR = ::win32::core::PCWSTR(16i32 as _);
pub const RT_VXD: ::win32::core::PCWSTR = ::win32::core::PCWSTR(20i32 as _);
pub const SBM_ENABLE_ARROWS: u32 = 228u32;
pub const SBM_GETPOS: u32 = 225u32;
pub const SBM_GETRANGE: u32 = 227u32;
pub const SBM_GETSCROLLBARINFO: u32 = 235u32;
pub const SBM_GETSCROLLINFO: u32 = 234u32;
pub const SBM_SETPOS: u32 = 224u32;
pub const SBM_SETRANGE: u32 = 226u32;
pub const SBM_SETRANGEREDRAW: u32 = 230u32;
pub const SBM_SETSCROLLINFO: u32 = 233u32;
pub const SBS_BOTTOMALIGN: i32 = 4i32;
pub const SBS_HORZ: i32 = 0i32;
pub const SBS_LEFTALIGN: i32 = 2i32;
pub const SBS_RIGHTALIGN: i32 = 4i32;
pub const SBS_SIZEBOX: i32 = 8i32;
pub const SBS_SIZEBOXBOTTOMRIGHTALIGN: i32 = 4i32;
pub const SBS_SIZEBOXTOPLEFTALIGN: i32 = 2i32;
pub const SBS_SIZEGRIP: i32 = 16i32;
pub const SBS_TOPALIGN: i32 = 2i32;
pub const SBS_VERT: i32 = 1i32;
pub const SB_BOTTOM: u32 = 7u32;
pub const SB_ENDSCROLL: u32 = 8u32;
pub const SB_LEFT: u32 = 6u32;
pub const SB_LINEDOWN: u32 = 1u32;
pub const SB_LINELEFT: u32 = 0u32;
pub const SB_LINERIGHT: u32 = 1u32;
pub const SB_LINEUP: u32 = 0u32;
pub const SB_PAGEDOWN: u32 = 3u32;
pub const SB_PAGELEFT: u32 = 2u32;
pub const SB_PAGERIGHT: u32 = 3u32;
pub const SB_PAGEUP: u32 = 2u32;
pub const SB_RIGHT: u32 = 7u32;
pub const SB_THUMBPOSITION: u32 = 4u32;
pub const SB_THUMBTRACK: u32 = 5u32;
pub const SB_TOP: u32 = 6u32;
pub const SCF_ISSECURE: u32 = 1u32;
pub struct SCROLLBARINFO {
    pub cbSize: u32,
    pub rcScrollBar: super::super::Foundation::RECT,
    pub dxyLineButton: i32,
    pub xyThumbTop: i32,
    pub xyThumbBottom: i32,
    pub reserved: i32,
    pub rgstate: [u32; 6],
}
impl ::core::marker::Copy for SCROLLBARINFO {}
impl ::core::clone::Clone for SCROLLBARINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCROLLBARINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCROLLBARINFO")
            .field("cbSize", &self.cbSize)
            .field("rcScrollBar", &self.rcScrollBar)
            .field("dxyLineButton", &self.dxyLineButton)
            .field("xyThumbTop", &self.xyThumbTop)
            .field("xyThumbBottom", &self.xyThumbBottom)
            .field("reserved", &self.reserved)
            .field("rgstate", &self.rgstate)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SCROLLBARINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.rcScrollBar == other.rcScrollBar
            && self.dxyLineButton == other.dxyLineButton
            && self.xyThumbTop == other.xyThumbTop
            && self.xyThumbBottom == other.xyThumbBottom
            && self.reserved == other.reserved
            && self.rgstate == other.rgstate
    }
}
impl ::core::cmp::Eq for SCROLLBARINFO {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SCROLLBAR_CONSTANTS(pub u32);
pub const SB_CTL: SCROLLBAR_CONSTANTS = SCROLLBAR_CONSTANTS(2u32);
pub const SB_HORZ: SCROLLBAR_CONSTANTS = SCROLLBAR_CONSTANTS(0u32);
pub const SB_VERT: SCROLLBAR_CONSTANTS = SCROLLBAR_CONSTANTS(1u32);
pub const SB_BOTH: SCROLLBAR_CONSTANTS = SCROLLBAR_CONSTANTS(3u32);
impl ::core::marker::Copy for SCROLLBAR_CONSTANTS {}
impl ::core::clone::Clone for SCROLLBAR_CONSTANTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCROLLBAR_CONSTANTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCROLLBAR_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCROLLBAR_CONSTANTS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SCROLLBAR_CONSTANTS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SCROLLBAR_CONSTANTS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SCROLLBAR_CONSTANTS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SCROLLBAR_CONSTANTS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SCROLLBAR_CONSTANTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub struct SCROLLINFO {
    pub cbSize: u32,
    pub fMask: SCROLLINFO_MASK,
    pub nMin: i32,
    pub nMax: i32,
    pub nPage: u32,
    pub nPos: i32,
    pub nTrackPos: i32,
}
impl ::core::marker::Copy for SCROLLINFO {}
impl ::core::clone::Clone for SCROLLINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCROLLINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCROLLINFO")
            .field("cbSize", &self.cbSize)
            .field("fMask", &self.fMask)
            .field("nMin", &self.nMin)
            .field("nMax", &self.nMax)
            .field("nPage", &self.nPage)
            .field("nPos", &self.nPos)
            .field("nTrackPos", &self.nTrackPos)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SCROLLINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.fMask == other.fMask
            && self.nMin == other.nMin
            && self.nMax == other.nMax
            && self.nPage == other.nPage
            && self.nPos == other.nPos
            && self.nTrackPos == other.nTrackPos
    }
}
impl ::core::cmp::Eq for SCROLLINFO {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SCROLLINFO_MASK(pub u32);
pub const SIF_ALL: SCROLLINFO_MASK = SCROLLINFO_MASK(23u32);
pub const SIF_DISABLENOSCROLL: SCROLLINFO_MASK = SCROLLINFO_MASK(8u32);
pub const SIF_PAGE: SCROLLINFO_MASK = SCROLLINFO_MASK(2u32);
pub const SIF_POS: SCROLLINFO_MASK = SCROLLINFO_MASK(4u32);
pub const SIF_RANGE: SCROLLINFO_MASK = SCROLLINFO_MASK(1u32);
pub const SIF_TRACKPOS: SCROLLINFO_MASK = SCROLLINFO_MASK(16u32);
impl ::core::marker::Copy for SCROLLINFO_MASK {}
impl ::core::clone::Clone for SCROLLINFO_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCROLLINFO_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCROLLINFO_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCROLLINFO_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SCROLLINFO_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SCROLLINFO_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SCROLLINFO_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SCROLLINFO_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SCROLLINFO_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const SC_ARRANGE: u32 = 61712u32;
pub const SC_CLOSE: u32 = 61536u32;
pub const SC_CONTEXTHELP: u32 = 61824u32;
pub const SC_DEFAULT: u32 = 61792u32;
pub const SC_HOTKEY: u32 = 61776u32;
pub const SC_HSCROLL: u32 = 61568u32;
pub const SC_ICON: u32 = 61472u32;
pub const SC_KEYMENU: u32 = 61696u32;
pub const SC_MAXIMIZE: u32 = 61488u32;
pub const SC_MINIMIZE: u32 = 61472u32;
pub const SC_MONITORPOWER: u32 = 61808u32;
pub const SC_MOUSEMENU: u32 = 61584u32;
pub const SC_MOVE: u32 = 61456u32;
pub const SC_NEXTWINDOW: u32 = 61504u32;
pub const SC_PREVWINDOW: u32 = 61520u32;
pub const SC_RESTORE: u32 = 61728u32;
pub const SC_SEPARATOR: u32 = 61455u32;
pub const SC_SIZE: u32 = 61440u32;
pub const SC_TASKLIST: u32 = 61744u32;
pub const SC_VSCROLL: u32 = 61552u32;
pub const SC_ZOOM: u32 = 61488u32;
pub type SENDASYNCPROC = ::core::option::Option<
    unsafe extern "system" fn(
        param0: super::super::Foundation::HWND,
        param1: u32,
        param2: usize,
        param3: super::super::Foundation::LRESULT,
    ),
>;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SEND_MESSAGE_TIMEOUT_FLAGS(pub u32);
pub const SMTO_ABORTIFHUNG: SEND_MESSAGE_TIMEOUT_FLAGS = SEND_MESSAGE_TIMEOUT_FLAGS(2u32);
pub const SMTO_BLOCK: SEND_MESSAGE_TIMEOUT_FLAGS = SEND_MESSAGE_TIMEOUT_FLAGS(1u32);
pub const SMTO_NORMAL: SEND_MESSAGE_TIMEOUT_FLAGS = SEND_MESSAGE_TIMEOUT_FLAGS(0u32);
pub const SMTO_NOTIMEOUTIFNOTHUNG: SEND_MESSAGE_TIMEOUT_FLAGS = SEND_MESSAGE_TIMEOUT_FLAGS(8u32);
pub const SMTO_ERRORONEXIT: SEND_MESSAGE_TIMEOUT_FLAGS = SEND_MESSAGE_TIMEOUT_FLAGS(32u32);
impl ::core::marker::Copy for SEND_MESSAGE_TIMEOUT_FLAGS {}
impl ::core::clone::Clone for SEND_MESSAGE_TIMEOUT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SEND_MESSAGE_TIMEOUT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SEND_MESSAGE_TIMEOUT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SEND_MESSAGE_TIMEOUT_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for SEND_MESSAGE_TIMEOUT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SEND_MESSAGE_TIMEOUT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SEND_MESSAGE_TIMEOUT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SEND_MESSAGE_TIMEOUT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SEND_MESSAGE_TIMEOUT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SET_WINDOW_POS_FLAGS(pub u32);
pub const SWP_ASYNCWINDOWPOS: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(16384u32);
pub const SWP_DEFERERASE: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(8192u32);
pub const SWP_DRAWFRAME: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(32u32);
pub const SWP_FRAMECHANGED: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(32u32);
pub const SWP_HIDEWINDOW: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(128u32);
pub const SWP_NOACTIVATE: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(16u32);
pub const SWP_NOCOPYBITS: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(256u32);
pub const SWP_NOMOVE: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(2u32);
pub const SWP_NOOWNERZORDER: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(512u32);
pub const SWP_NOREDRAW: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(8u32);
pub const SWP_NOREPOSITION: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(512u32);
pub const SWP_NOSENDCHANGING: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(1024u32);
pub const SWP_NOSIZE: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(1u32);
pub const SWP_NOZORDER: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(4u32);
pub const SWP_SHOWWINDOW: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(64u32);
pub const SWP__NOOWNERZORDER: SET_WINDOW_POS_FLAGS = SET_WINDOW_POS_FLAGS(512u32);
impl ::core::marker::Copy for SET_WINDOW_POS_FLAGS {}
impl ::core::clone::Clone for SET_WINDOW_POS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SET_WINDOW_POS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SET_WINDOW_POS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SET_WINDOW_POS_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for SET_WINDOW_POS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SET_WINDOW_POS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SET_WINDOW_POS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SET_WINDOW_POS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SET_WINDOW_POS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub struct SHELLHOOKINFO {
    pub hwnd: super::super::Foundation::HWND,
    pub rc: super::super::Foundation::RECT,
}
impl ::core::marker::Copy for SHELLHOOKINFO {}
impl ::core::clone::Clone for SHELLHOOKINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SHELLHOOKINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHELLHOOKINFO")
            .field("hwnd", &self.hwnd)
            .field("rc", &self.rc)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SHELLHOOKINFO {
    fn eq(&self, other: &Self) -> bool {
        self.hwnd == other.hwnd && self.rc == other.rc
    }
}
impl ::core::cmp::Eq for SHELLHOOKINFO {}
pub const SHOW_FULLSCREEN: u32 = 3u32;
pub const SHOW_ICONWINDOW: u32 = 2u32;
pub const SHOW_OPENNOACTIVATE: u32 = 4u32;
pub const SHOW_OPENWINDOW: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SHOW_WINDOW_CMD(pub u32);
pub const SW_FORCEMINIMIZE: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(11u32);
pub const SW_HIDE: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(0u32);
pub const SW_MAXIMIZE: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(3u32);
pub const SW_MINIMIZE: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(6u32);
pub const SW_RESTORE: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(9u32);
pub const SW_SHOW: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(5u32);
pub const SW_SHOWDEFAULT: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(10u32);
pub const SW_SHOWMAXIMIZED: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(3u32);
pub const SW_SHOWMINIMIZED: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(2u32);
pub const SW_SHOWMINNOACTIVE: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(7u32);
pub const SW_SHOWNA: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(8u32);
pub const SW_SHOWNOACTIVATE: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(4u32);
pub const SW_SHOWNORMAL: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(1u32);
pub const SW_NORMAL: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(1u32);
pub const SW_MAX: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(11u32);
pub const SW_PARENTCLOSING: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(1u32);
pub const SW_OTHERZOOM: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(2u32);
pub const SW_PARENTOPENING: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(3u32);
pub const SW_OTHERUNZOOM: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(4u32);
pub const SW_SCROLLCHILDREN: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(1u32);
pub const SW_INVALIDATE: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(2u32);
pub const SW_ERASE: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(4u32);
pub const SW_SMOOTHSCROLL: SHOW_WINDOW_CMD = SHOW_WINDOW_CMD(16u32);
impl ::core::marker::Copy for SHOW_WINDOW_CMD {}
impl ::core::clone::Clone for SHOW_WINDOW_CMD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SHOW_WINDOW_CMD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHOW_WINDOW_CMD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHOW_WINDOW_CMD").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SHOW_WINDOW_CMD {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SHOW_WINDOW_CMD {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SHOW_WINDOW_CMD {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SHOW_WINDOW_CMD {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SHOW_WINDOW_CMD {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const SIZEFULLSCREEN: u32 = 2u32;
pub const SIZEICONIC: u32 = 1u32;
pub const SIZENORMAL: u32 = 0u32;
pub const SIZEZOOMHIDE: u32 = 4u32;
pub const SIZEZOOMSHOW: u32 = 3u32;
pub const SIZE_MAXHIDE: u32 = 4u32;
pub const SIZE_MAXIMIZED: u32 = 2u32;
pub const SIZE_MAXSHOW: u32 = 3u32;
pub const SIZE_MINIMIZED: u32 = 1u32;
pub const SIZE_RESTORED: u32 = 0u32;
pub const SM_CARETBLINKINGENABLED: u32 = 8194u32;
pub const SM_CMETRICS: u32 = 76u32;
pub const SM_RESERVED1: u32 = 24u32;
pub const SM_RESERVED2: u32 = 25u32;
pub const SM_RESERVED3: u32 = 26u32;
pub const SM_RESERVED4: u32 = 27u32;
pub const SM_SYSTEMDOCKED: u32 = 8196u32;
pub const SOUND_SYSTEM_APPEND: u32 = 14u32;
pub const SOUND_SYSTEM_APPSTART: u32 = 12u32;
pub const SOUND_SYSTEM_BEEP: u32 = 3u32;
pub const SOUND_SYSTEM_ERROR: u32 = 4u32;
pub const SOUND_SYSTEM_FAULT: u32 = 13u32;
pub const SOUND_SYSTEM_INFORMATION: u32 = 7u32;
pub const SOUND_SYSTEM_MAXIMIZE: u32 = 8u32;
pub const SOUND_SYSTEM_MENUCOMMAND: u32 = 15u32;
pub const SOUND_SYSTEM_MENUPOPUP: u32 = 16u32;
pub const SOUND_SYSTEM_MINIMIZE: u32 = 9u32;
pub const SOUND_SYSTEM_QUESTION: u32 = 5u32;
pub const SOUND_SYSTEM_RESTOREDOWN: u32 = 11u32;
pub const SOUND_SYSTEM_RESTOREUP: u32 = 10u32;
pub const SOUND_SYSTEM_SHUTDOWN: u32 = 2u32;
pub const SOUND_SYSTEM_STARTUP: u32 = 1u32;
pub const SOUND_SYSTEM_WARNING: u32 = 6u32;
pub const SS_BITMAP: i32 = 14i32;
pub const SS_BLACKFRAME: i32 = 7i32;
pub const SS_BLACKRECT: i32 = 4i32;
pub const SS_CENTER: i32 = 1i32;
pub const SS_CENTERIMAGE: i32 = 512i32;
pub const SS_EDITCONTROL: i32 = 8192i32;
pub const SS_ELLIPSISMASK: i32 = 49152i32;
pub const SS_ENDELLIPSIS: i32 = 16384i32;
pub const SS_ENHMETAFILE: i32 = 15i32;
pub const SS_ETCHEDFRAME: i32 = 18i32;
pub const SS_ETCHEDHORZ: i32 = 16i32;
pub const SS_ETCHEDVERT: i32 = 17i32;
pub const SS_GRAYFRAME: i32 = 8i32;
pub const SS_GRAYRECT: i32 = 5i32;
pub const SS_ICON: i32 = 3i32;
pub const SS_LEFT: i32 = 0i32;
pub const SS_LEFTNOWORDWRAP: i32 = 12i32;
pub const SS_NOPREFIX: i32 = 128i32;
pub const SS_NOTIFY: i32 = 256i32;
pub const SS_OWNERDRAW: i32 = 13i32;
pub const SS_PATHELLIPSIS: i32 = 32768i32;
pub const SS_REALSIZECONTROL: i32 = 64i32;
pub const SS_REALSIZEIMAGE: i32 = 2048i32;
pub const SS_RIGHT: i32 = 2i32;
pub const SS_RIGHTJUST: i32 = 1024i32;
pub const SS_SIMPLE: i32 = 11i32;
pub const SS_SUNKEN: i32 = 4096i32;
pub const SS_TYPEMASK: i32 = 31i32;
pub const SS_USERITEM: i32 = 10i32;
pub const SS_WHITEFRAME: i32 = 9i32;
pub const SS_WHITERECT: i32 = 6i32;
pub const SS_WORDELLIPSIS: i32 = 49152i32;
pub const STATE_SYSTEM_ALERT_HIGH: u32 = 268435456u32;
pub const STATE_SYSTEM_ALERT_LOW: u32 = 67108864u32;
pub const STATE_SYSTEM_ALERT_MEDIUM: u32 = 134217728u32;
pub const STATE_SYSTEM_ANIMATED: u32 = 16384u32;
pub const STATE_SYSTEM_BUSY: u32 = 2048u32;
pub const STATE_SYSTEM_CHECKED: u32 = 16u32;
pub const STATE_SYSTEM_COLLAPSED: u32 = 1024u32;
pub const STATE_SYSTEM_DEFAULT: u32 = 256u32;
pub const STATE_SYSTEM_EXPANDED: u32 = 512u32;
pub const STATE_SYSTEM_EXTSELECTABLE: u32 = 33554432u32;
pub const STATE_SYSTEM_FLOATING: u32 = 4096u32;
pub const STATE_SYSTEM_FOCUSED: u32 = 4u32;
pub const STATE_SYSTEM_HOTTRACKED: u32 = 128u32;
pub const STATE_SYSTEM_INDETERMINATE: u32 = 32u32;
pub const STATE_SYSTEM_LINKED: u32 = 4194304u32;
pub const STATE_SYSTEM_MARQUEED: u32 = 8192u32;
pub const STATE_SYSTEM_MIXED: u32 = 32u32;
pub const STATE_SYSTEM_MOVEABLE: u32 = 262144u32;
pub const STATE_SYSTEM_MULTISELECTABLE: u32 = 16777216u32;
pub const STATE_SYSTEM_PROTECTED: u32 = 536870912u32;
pub const STATE_SYSTEM_READONLY: u32 = 64u32;
pub const STATE_SYSTEM_SELECTABLE: u32 = 2097152u32;
pub const STATE_SYSTEM_SELECTED: u32 = 2u32;
pub const STATE_SYSTEM_SELFVOICING: u32 = 524288u32;
pub const STATE_SYSTEM_SIZEABLE: u32 = 131072u32;
pub const STATE_SYSTEM_TRAVERSED: u32 = 8388608u32;
pub const STATE_SYSTEM_VALID: u32 = 1073741823u32;
pub const STM_GETICON: u32 = 369u32;
pub const STM_GETIMAGE: u32 = 371u32;
pub const STM_MSGMAX: u32 = 372u32;
pub const STM_SETICON: u32 = 368u32;
pub const STM_SETIMAGE: u32 = 370u32;
pub const STN_CLICKED: u32 = 0u32;
pub const STN_DBLCLK: u32 = 1u32;
pub const STN_DISABLE: u32 = 3u32;
pub const STN_ENABLE: u32 = 2u32;
pub const STRSAFE_E_END_OF_FILE: ::win32::core::HRESULT = ::win32::core::HRESULT(-2147024858i32);
pub const STRSAFE_E_INSUFFICIENT_BUFFER: ::win32::core::HRESULT =
    ::win32::core::HRESULT(-2147024774i32);
pub const STRSAFE_E_INVALID_PARAMETER: ::win32::core::HRESULT =
    ::win32::core::HRESULT(-2147024809i32);
pub const STRSAFE_FILL_BEHIND_NULL: u32 = 512u32;
pub const STRSAFE_FILL_ON_FAILURE: u32 = 1024u32;
pub const STRSAFE_IGNORE_NULLS: u32 = 256u32;
pub const STRSAFE_MAX_CCH: u32 = 2147483647u32;
pub const STRSAFE_NO_TRUNCATION: u32 = 4096u32;
pub const STRSAFE_NULL_ON_FAILURE: u32 = 2048u32;
pub const STRSAFE_USE_SECURE_CRT: u32 = 0u32;
pub struct STYLESTRUCT {
    pub styleOld: u32,
    pub styleNew: u32,
}
impl ::core::marker::Copy for STYLESTRUCT {}
impl ::core::clone::Clone for STYLESTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STYLESTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STYLESTRUCT")
            .field("styleOld", &self.styleOld)
            .field("styleNew", &self.styleNew)
            .finish()
    }
}
impl ::core::cmp::PartialEq for STYLESTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.styleOld == other.styleOld && self.styleNew == other.styleNew
    }
}
impl ::core::cmp::Eq for STYLESTRUCT {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SYSTEM_CURSOR_ID(pub u32);
pub const OCR_APPSTARTING: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32650u32);
pub const OCR_NORMAL: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32512u32);
pub const OCR_CROSS: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32515u32);
pub const OCR_HAND: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32649u32);
pub const OCR_HELP: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32651u32);
pub const OCR_IBEAM: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32513u32);
pub const OCR_NO: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32648u32);
pub const OCR_SIZEALL: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32646u32);
pub const OCR_SIZENESW: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32643u32);
pub const OCR_SIZENS: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32645u32);
pub const OCR_SIZENWSE: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32642u32);
pub const OCR_SIZEWE: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32644u32);
pub const OCR_UP: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32516u32);
pub const OCR_WAIT: SYSTEM_CURSOR_ID = SYSTEM_CURSOR_ID(32514u32);
impl ::core::marker::Copy for SYSTEM_CURSOR_ID {}
impl ::core::clone::Clone for SYSTEM_CURSOR_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYSTEM_CURSOR_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYSTEM_CURSOR_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSTEM_CURSOR_ID").field(&self.0).finish()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SYSTEM_METRICS_INDEX(pub u32);
pub const SM_ARRANGE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(56u32);
pub const SM_CLEANBOOT: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(67u32);
pub const SM_CMONITORS: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(80u32);
pub const SM_CMOUSEBUTTONS: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(43u32);
pub const SM_CONVERTIBLESLATEMODE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(8195u32);
pub const SM_CXBORDER: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(5u32);
pub const SM_CXCURSOR: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(13u32);
pub const SM_CXDLGFRAME: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(7u32);
pub const SM_CXDOUBLECLK: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(36u32);
pub const SM_CXDRAG: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(68u32);
pub const SM_CXEDGE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(45u32);
pub const SM_CXFIXEDFRAME: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(7u32);
pub const SM_CXFOCUSBORDER: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(83u32);
pub const SM_CXFRAME: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(32u32);
pub const SM_CXFULLSCREEN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(16u32);
pub const SM_CXHSCROLL: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(21u32);
pub const SM_CXHTHUMB: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(10u32);
pub const SM_CXICON: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(11u32);
pub const SM_CXICONSPACING: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(38u32);
pub const SM_CXMAXIMIZED: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(61u32);
pub const SM_CXMAXTRACK: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(59u32);
pub const SM_CXMENUCHECK: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(71u32);
pub const SM_CXMENUSIZE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(54u32);
pub const SM_CXMIN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(28u32);
pub const SM_CXMINIMIZED: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(57u32);
pub const SM_CXMINSPACING: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(47u32);
pub const SM_CXMINTRACK: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(34u32);
pub const SM_CXPADDEDBORDER: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(92u32);
pub const SM_CXSCREEN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(0u32);
pub const SM_CXSIZE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(30u32);
pub const SM_CXSIZEFRAME: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(32u32);
pub const SM_CXSMICON: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(49u32);
pub const SM_CXSMSIZE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(52u32);
pub const SM_CXVIRTUALSCREEN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(78u32);
pub const SM_CXVSCROLL: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(2u32);
pub const SM_CYBORDER: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(6u32);
pub const SM_CYCAPTION: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(4u32);
pub const SM_CYCURSOR: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(14u32);
pub const SM_CYDLGFRAME: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(8u32);
pub const SM_CYDOUBLECLK: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(37u32);
pub const SM_CYDRAG: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(69u32);
pub const SM_CYEDGE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(46u32);
pub const SM_CYFIXEDFRAME: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(8u32);
pub const SM_CYFOCUSBORDER: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(84u32);
pub const SM_CYFRAME: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(33u32);
pub const SM_CYFULLSCREEN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(17u32);
pub const SM_CYHSCROLL: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(3u32);
pub const SM_CYICON: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(12u32);
pub const SM_CYICONSPACING: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(39u32);
pub const SM_CYKANJIWINDOW: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(18u32);
pub const SM_CYMAXIMIZED: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(62u32);
pub const SM_CYMAXTRACK: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(60u32);
pub const SM_CYMENU: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(15u32);
pub const SM_CYMENUCHECK: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(72u32);
pub const SM_CYMENUSIZE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(55u32);
pub const SM_CYMIN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(29u32);
pub const SM_CYMINIMIZED: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(58u32);
pub const SM_CYMINSPACING: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(48u32);
pub const SM_CYMINTRACK: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(35u32);
pub const SM_CYSCREEN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(1u32);
pub const SM_CYSIZE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(31u32);
pub const SM_CYSIZEFRAME: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(33u32);
pub const SM_CYSMCAPTION: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(51u32);
pub const SM_CYSMICON: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(50u32);
pub const SM_CYSMSIZE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(53u32);
pub const SM_CYVIRTUALSCREEN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(79u32);
pub const SM_CYVSCROLL: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(20u32);
pub const SM_CYVTHUMB: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(9u32);
pub const SM_DBCSENABLED: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(42u32);
pub const SM_DEBUG: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(22u32);
pub const SM_DIGITIZER: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(94u32);
pub const SM_IMMENABLED: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(82u32);
pub const SM_MAXIMUMTOUCHES: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(95u32);
pub const SM_MEDIACENTER: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(87u32);
pub const SM_MENUDROPALIGNMENT: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(40u32);
pub const SM_MIDEASTENABLED: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(74u32);
pub const SM_MOUSEPRESENT: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(19u32);
pub const SM_MOUSEHORIZONTALWHEELPRESENT: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(91u32);
pub const SM_MOUSEWHEELPRESENT: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(75u32);
pub const SM_NETWORK: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(63u32);
pub const SM_PENWINDOWS: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(41u32);
pub const SM_REMOTECONTROL: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(8193u32);
pub const SM_REMOTESESSION: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(4096u32);
pub const SM_SAMEDISPLAYFORMAT: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(81u32);
pub const SM_SECURE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(44u32);
pub const SM_SERVERR2: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(89u32);
pub const SM_SHOWSOUNDS: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(70u32);
pub const SM_SHUTTINGDOWN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(8192u32);
pub const SM_SLOWMACHINE: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(73u32);
pub const SM_STARTER: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(88u32);
pub const SM_SWAPBUTTON: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(23u32);
pub const SM_SYSTEMDOCKED_: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(8196u32);
pub const SM_TABLETPC: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(86u32);
pub const SM_XVIRTUALSCREEN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(76u32);
pub const SM_YVIRTUALSCREEN: SYSTEM_METRICS_INDEX = SYSTEM_METRICS_INDEX(77u32);
impl ::core::marker::Copy for SYSTEM_METRICS_INDEX {}
impl ::core::clone::Clone for SYSTEM_METRICS_INDEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYSTEM_METRICS_INDEX {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYSTEM_METRICS_INDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSTEM_METRICS_INDEX")
            .field(&self.0)
            .finish()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SYSTEM_PARAMETERS_INFO_ACTION(pub u32);
pub const SPI_GETBEEP: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(1u32);
pub const SPI_SETBEEP: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(2u32);
pub const SPI_GETMOUSE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(3u32);
pub const SPI_SETMOUSE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4u32);
pub const SPI_GETBORDER: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(5u32);
pub const SPI_SETBORDER: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(6u32);
pub const SPI_GETKEYBOARDSPEED: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(10u32);
pub const SPI_SETKEYBOARDSPEED: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(11u32);
pub const SPI_LANGDRIVER: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(12u32);
pub const SPI_ICONHORIZONTALSPACING: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(13u32);
pub const SPI_GETSCREENSAVETIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(14u32);
pub const SPI_SETSCREENSAVETIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(15u32);
pub const SPI_GETSCREENSAVEACTIVE: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(16u32);
pub const SPI_SETSCREENSAVEACTIVE: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(17u32);
pub const SPI_GETGRIDGRANULARITY: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(18u32);
pub const SPI_SETGRIDGRANULARITY: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(19u32);
pub const SPI_SETDESKWALLPAPER: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(20u32);
pub const SPI_SETDESKPATTERN: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(21u32);
pub const SPI_GETKEYBOARDDELAY: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(22u32);
pub const SPI_SETKEYBOARDDELAY: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(23u32);
pub const SPI_ICONVERTICALSPACING: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(24u32);
pub const SPI_GETICONTITLEWRAP: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(25u32);
pub const SPI_SETICONTITLEWRAP: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(26u32);
pub const SPI_GETMENUDROPALIGNMENT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(27u32);
pub const SPI_SETMENUDROPALIGNMENT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(28u32);
pub const SPI_SETDOUBLECLKWIDTH: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(29u32);
pub const SPI_SETDOUBLECLKHEIGHT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(30u32);
pub const SPI_GETICONTITLELOGFONT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(31u32);
pub const SPI_SETDOUBLECLICKTIME: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(32u32);
pub const SPI_SETMOUSEBUTTONSWAP: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(33u32);
pub const SPI_SETICONTITLELOGFONT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(34u32);
pub const SPI_GETFASTTASKSWITCH: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(35u32);
pub const SPI_SETFASTTASKSWITCH: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(36u32);
pub const SPI_SETDRAGFULLWINDOWS: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(37u32);
pub const SPI_GETDRAGFULLWINDOWS: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(38u32);
pub const SPI_GETNONCLIENTMETRICS: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(41u32);
pub const SPI_SETNONCLIENTMETRICS: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(42u32);
pub const SPI_GETMINIMIZEDMETRICS: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(43u32);
pub const SPI_SETMINIMIZEDMETRICS: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(44u32);
pub const SPI_GETICONMETRICS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(45u32);
pub const SPI_SETICONMETRICS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(46u32);
pub const SPI_SETWORKAREA: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(47u32);
pub const SPI_GETWORKAREA: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(48u32);
pub const SPI_SETPENWINDOWS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(49u32);
pub const SPI_GETHIGHCONTRAST: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(66u32);
pub const SPI_SETHIGHCONTRAST: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(67u32);
pub const SPI_GETKEYBOARDPREF: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(68u32);
pub const SPI_SETKEYBOARDPREF: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(69u32);
pub const SPI_GETSCREENREADER: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(70u32);
pub const SPI_SETSCREENREADER: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(71u32);
pub const SPI_GETANIMATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(72u32);
pub const SPI_SETANIMATION: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(73u32);
pub const SPI_GETFONTSMOOTHING: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(74u32);
pub const SPI_SETFONTSMOOTHING: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(75u32);
pub const SPI_SETDRAGWIDTH: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(76u32);
pub const SPI_SETDRAGHEIGHT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(77u32);
pub const SPI_SETHANDHELD: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(78u32);
pub const SPI_GETLOWPOWERTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(79u32);
pub const SPI_GETPOWEROFFTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(80u32);
pub const SPI_SETLOWPOWERTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(81u32);
pub const SPI_SETPOWEROFFTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(82u32);
pub const SPI_GETLOWPOWERACTIVE: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(83u32);
pub const SPI_GETPOWEROFFACTIVE: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(84u32);
pub const SPI_SETLOWPOWERACTIVE: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(85u32);
pub const SPI_SETPOWEROFFACTIVE: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(86u32);
pub const SPI_SETCURSORS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(87u32);
pub const SPI_SETICONS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(88u32);
pub const SPI_GETDEFAULTINPUTLANG: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(89u32);
pub const SPI_SETDEFAULTINPUTLANG: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(90u32);
pub const SPI_SETLANGTOGGLE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(91u32);
pub const SPI_GETWINDOWSEXTENSION: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(92u32);
pub const SPI_SETMOUSETRAILS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(93u32);
pub const SPI_GETMOUSETRAILS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(94u32);
pub const SPI_SETSCREENSAVERRUNNING: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(97u32);
pub const SPI_SCREENSAVERRUNNING: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(97u32);
pub const SPI_GETFILTERKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(50u32);
pub const SPI_SETFILTERKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(51u32);
pub const SPI_GETTOGGLEKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(52u32);
pub const SPI_SETTOGGLEKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(53u32);
pub const SPI_GETMOUSEKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(54u32);
pub const SPI_SETMOUSEKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(55u32);
pub const SPI_GETSHOWSOUNDS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(56u32);
pub const SPI_SETSHOWSOUNDS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(57u32);
pub const SPI_GETSTICKYKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(58u32);
pub const SPI_SETSTICKYKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(59u32);
pub const SPI_GETACCESSTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(60u32);
pub const SPI_SETACCESSTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(61u32);
pub const SPI_GETSERIALKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(62u32);
pub const SPI_SETSERIALKEYS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(63u32);
pub const SPI_GETSOUNDSENTRY: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(64u32);
pub const SPI_SETSOUNDSENTRY: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(65u32);
pub const SPI_GETSNAPTODEFBUTTON: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(95u32);
pub const SPI_SETSNAPTODEFBUTTON: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(96u32);
pub const SPI_GETMOUSEHOVERWIDTH: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(98u32);
pub const SPI_SETMOUSEHOVERWIDTH: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(99u32);
pub const SPI_GETMOUSEHOVERHEIGHT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(100u32);
pub const SPI_SETMOUSEHOVERHEIGHT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(101u32);
pub const SPI_GETMOUSEHOVERTIME: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(102u32);
pub const SPI_SETMOUSEHOVERTIME: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(103u32);
pub const SPI_GETWHEELSCROLLLINES: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(104u32);
pub const SPI_SETWHEELSCROLLLINES: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(105u32);
pub const SPI_GETMENUSHOWDELAY: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(106u32);
pub const SPI_SETMENUSHOWDELAY: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(107u32);
pub const SPI_GETWHEELSCROLLCHARS: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(108u32);
pub const SPI_SETWHEELSCROLLCHARS: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(109u32);
pub const SPI_GETSHOWIMEUI: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(110u32);
pub const SPI_SETSHOWIMEUI: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(111u32);
pub const SPI_GETMOUSESPEED: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(112u32);
pub const SPI_SETMOUSESPEED: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(113u32);
pub const SPI_GETSCREENSAVERRUNNING: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(114u32);
pub const SPI_GETDESKWALLPAPER: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(115u32);
pub const SPI_GETAUDIODESCRIPTION: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(116u32);
pub const SPI_SETAUDIODESCRIPTION: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(117u32);
pub const SPI_GETSCREENSAVESECURE: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(118u32);
pub const SPI_SETSCREENSAVESECURE: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(119u32);
pub const SPI_GETHUNGAPPTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(120u32);
pub const SPI_SETHUNGAPPTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(121u32);
pub const SPI_GETWAITTOKILLTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(122u32);
pub const SPI_SETWAITTOKILLTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(123u32);
pub const SPI_GETWAITTOKILLSERVICETIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(124u32);
pub const SPI_SETWAITTOKILLSERVICETIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(125u32);
pub const SPI_GETMOUSEDOCKTHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(126u32);
pub const SPI_SETMOUSEDOCKTHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(127u32);
pub const SPI_GETPENDOCKTHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(128u32);
pub const SPI_SETPENDOCKTHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(129u32);
pub const SPI_GETWINARRANGING: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(130u32);
pub const SPI_SETWINARRANGING: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(131u32);
pub const SPI_GETMOUSEDRAGOUTTHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(132u32);
pub const SPI_SETMOUSEDRAGOUTTHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(133u32);
pub const SPI_GETPENDRAGOUTTHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(134u32);
pub const SPI_SETPENDRAGOUTTHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(135u32);
pub const SPI_GETMOUSESIDEMOVETHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(136u32);
pub const SPI_SETMOUSESIDEMOVETHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(137u32);
pub const SPI_GETPENSIDEMOVETHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(138u32);
pub const SPI_SETPENSIDEMOVETHRESHOLD: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(139u32);
pub const SPI_GETDRAGFROMMAXIMIZE: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(140u32);
pub const SPI_SETDRAGFROMMAXIMIZE: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(141u32);
pub const SPI_GETSNAPSIZING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(142u32);
pub const SPI_SETSNAPSIZING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(143u32);
pub const SPI_GETDOCKMOVING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(144u32);
pub const SPI_SETDOCKMOVING: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(145u32);
pub const SPI_GETTOUCHPREDICTIONPARAMETERS: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(156u32);
pub const SPI_SETTOUCHPREDICTIONPARAMETERS: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(157u32);
pub const SPI_GETLOGICALDPIOVERRIDE: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(158u32);
pub const SPI_SETLOGICALDPIOVERRIDE: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(159u32);
pub const SPI_GETMENURECT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(162u32);
pub const SPI_SETMENURECT: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(163u32);
pub const SPI_GETACTIVEWINDOWTRACKING: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4096u32);
pub const SPI_SETACTIVEWINDOWTRACKING: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4097u32);
pub const SPI_GETMENUANIMATION: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4098u32);
pub const SPI_SETMENUANIMATION: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4099u32);
pub const SPI_GETCOMBOBOXANIMATION: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4100u32);
pub const SPI_SETCOMBOBOXANIMATION: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4101u32);
pub const SPI_GETLISTBOXSMOOTHSCROLLING: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4102u32);
pub const SPI_SETLISTBOXSMOOTHSCROLLING: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4103u32);
pub const SPI_GETGRADIENTCAPTIONS: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4104u32);
pub const SPI_SETGRADIENTCAPTIONS: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4105u32);
pub const SPI_GETKEYBOARDCUES: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4106u32);
pub const SPI_SETKEYBOARDCUES: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4107u32);
pub const SPI_GETMENUUNDERLINES: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4106u32);
pub const SPI_SETMENUUNDERLINES: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4107u32);
pub const SPI_GETACTIVEWNDTRKZORDER: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4108u32);
pub const SPI_SETACTIVEWNDTRKZORDER: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4109u32);
pub const SPI_GETHOTTRACKING: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4110u32);
pub const SPI_SETHOTTRACKING: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4111u32);
pub const SPI_GETMENUFADE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4114u32);
pub const SPI_SETMENUFADE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4115u32);
pub const SPI_GETSELECTIONFADE: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4116u32);
pub const SPI_SETSELECTIONFADE: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4117u32);
pub const SPI_GETTOOLTIPANIMATION: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4118u32);
pub const SPI_SETTOOLTIPANIMATION: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4119u32);
pub const SPI_GETTOOLTIPFADE: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4120u32);
pub const SPI_SETTOOLTIPFADE: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4121u32);
pub const SPI_GETCURSORSHADOW: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4122u32);
pub const SPI_SETCURSORSHADOW: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4123u32);
pub const SPI_GETMOUSESONAR: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4124u32);
pub const SPI_SETMOUSESONAR: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4125u32);
pub const SPI_GETMOUSECLICKLOCK: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4126u32);
pub const SPI_SETMOUSECLICKLOCK: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4127u32);
pub const SPI_GETMOUSEVANISH: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4128u32);
pub const SPI_SETMOUSEVANISH: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4129u32);
pub const SPI_GETFLATMENU: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4130u32);
pub const SPI_SETFLATMENU: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4131u32);
pub const SPI_GETDROPSHADOW: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4132u32);
pub const SPI_SETDROPSHADOW: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4133u32);
pub const SPI_GETBLOCKSENDINPUTRESETS: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4134u32);
pub const SPI_SETBLOCKSENDINPUTRESETS: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4135u32);
pub const SPI_GETUIEFFECTS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4158u32);
pub const SPI_SETUIEFFECTS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4159u32);
pub const SPI_GETDISABLEOVERLAPPEDCONTENT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4160u32);
pub const SPI_SETDISABLEOVERLAPPEDCONTENT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4161u32);
pub const SPI_GETCLIENTAREAANIMATION: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4162u32);
pub const SPI_SETCLIENTAREAANIMATION: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4163u32);
pub const SPI_GETCLEARTYPE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4168u32);
pub const SPI_SETCLEARTYPE: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(4169u32);
pub const SPI_GETSPEECHRECOGNITION: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4170u32);
pub const SPI_SETSPEECHRECOGNITION: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4171u32);
pub const SPI_GETCARETBROWSING: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4172u32);
pub const SPI_SETCARETBROWSING: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4173u32);
pub const SPI_GETTHREADLOCALINPUTSETTINGS: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4174u32);
pub const SPI_SETTHREADLOCALINPUTSETTINGS: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4175u32);
pub const SPI_GETSYSTEMLANGUAGEBAR: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4176u32);
pub const SPI_SETSYSTEMLANGUAGEBAR: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(4177u32);
pub const SPI_GETFOREGROUNDLOCKTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8192u32);
pub const SPI_SETFOREGROUNDLOCKTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8193u32);
pub const SPI_GETACTIVEWNDTRKTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8194u32);
pub const SPI_SETACTIVEWNDTRKTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8195u32);
pub const SPI_GETFOREGROUNDFLASHCOUNT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8196u32);
pub const SPI_SETFOREGROUNDFLASHCOUNT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8197u32);
pub const SPI_GETCARETWIDTH: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8198u32);
pub const SPI_SETCARETWIDTH: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8199u32);
pub const SPI_GETMOUSECLICKLOCKTIME: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8200u32);
pub const SPI_SETMOUSECLICKLOCKTIME: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8201u32);
pub const SPI_GETFONTSMOOTHINGTYPE: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8202u32);
pub const SPI_SETFONTSMOOTHINGTYPE: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8203u32);
pub const SPI_GETFONTSMOOTHINGCONTRAST: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8204u32);
pub const SPI_SETFONTSMOOTHINGCONTRAST: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8205u32);
pub const SPI_GETFOCUSBORDERWIDTH: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8206u32);
pub const SPI_SETFOCUSBORDERWIDTH: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8207u32);
pub const SPI_GETFOCUSBORDERHEIGHT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8208u32);
pub const SPI_SETFOCUSBORDERHEIGHT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8209u32);
pub const SPI_GETFONTSMOOTHINGORIENTATION: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8210u32);
pub const SPI_SETFONTSMOOTHINGORIENTATION: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8211u32);
pub const SPI_GETMINIMUMHITRADIUS: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8212u32);
pub const SPI_SETMINIMUMHITRADIUS: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8213u32);
pub const SPI_GETMESSAGEDURATION: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8214u32);
pub const SPI_SETMESSAGEDURATION: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8215u32);
pub const SPI_GETCONTACTVISUALIZATION: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8216u32);
pub const SPI_SETCONTACTVISUALIZATION: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8217u32);
pub const SPI_GETGESTUREVISUALIZATION: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8218u32);
pub const SPI_SETGESTUREVISUALIZATION: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8219u32);
pub const SPI_GETMOUSEWHEELROUTING: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8220u32);
pub const SPI_SETMOUSEWHEELROUTING: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8221u32);
pub const SPI_GETPENVISUALIZATION: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8222u32);
pub const SPI_SETPENVISUALIZATION: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8223u32);
pub const SPI_GETPENARBITRATIONTYPE: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8224u32);
pub const SPI_SETPENARBITRATIONTYPE: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8225u32);
pub const SPI_GETCARETTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8226u32);
pub const SPI_SETCARETTIMEOUT: SYSTEM_PARAMETERS_INFO_ACTION =
    SYSTEM_PARAMETERS_INFO_ACTION(8227u32);
pub const SPI_GETHANDEDNESS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8228u32);
pub const SPI_SETHANDEDNESS: SYSTEM_PARAMETERS_INFO_ACTION = SYSTEM_PARAMETERS_INFO_ACTION(8229u32);
impl ::core::marker::Copy for SYSTEM_PARAMETERS_INFO_ACTION {}
impl ::core::clone::Clone for SYSTEM_PARAMETERS_INFO_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYSTEM_PARAMETERS_INFO_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYSTEM_PARAMETERS_INFO_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSTEM_PARAMETERS_INFO_ACTION")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for SYSTEM_PARAMETERS_INFO_ACTION {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SYSTEM_PARAMETERS_INFO_ACTION {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SYSTEM_PARAMETERS_INFO_ACTION {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SYSTEM_PARAMETERS_INFO_ACTION {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SYSTEM_PARAMETERS_INFO_ACTION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(pub u32);
pub const SPIF_UPDATEINIFILE: SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS =
    SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(1u32);
pub const SPIF_SENDCHANGE: SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS =
    SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(2u32);
pub const SPIF_SENDWININICHANGE: SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS =
    SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(2u32);
impl ::core::marker::Copy for SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS {}
impl ::core::clone::Clone for SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SYS_COLOR_INDEX(pub u32);
pub const COLOR_3DDKSHADOW: SYS_COLOR_INDEX = SYS_COLOR_INDEX(21u32);
pub const COLOR_3DFACE: SYS_COLOR_INDEX = SYS_COLOR_INDEX(15u32);
pub const COLOR_3DHIGHLIGHT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(20u32);
pub const COLOR_3DHILIGHT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(20u32);
pub const COLOR_3DLIGHT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(22u32);
pub const COLOR_3DSHADOW: SYS_COLOR_INDEX = SYS_COLOR_INDEX(16u32);
pub const COLOR_ACTIVEBORDER: SYS_COLOR_INDEX = SYS_COLOR_INDEX(10u32);
pub const COLOR_ACTIVECAPTION: SYS_COLOR_INDEX = SYS_COLOR_INDEX(2u32);
pub const COLOR_APPWORKSPACE: SYS_COLOR_INDEX = SYS_COLOR_INDEX(12u32);
pub const COLOR_BACKGROUND: SYS_COLOR_INDEX = SYS_COLOR_INDEX(1u32);
pub const COLOR_BTNFACE: SYS_COLOR_INDEX = SYS_COLOR_INDEX(15u32);
pub const _COLOR_BTNHIGHLIGHT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(20u32);
pub const _COLOR_BTNHILIGHT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(20u32);
pub const COLOR_BTNSHADOW: SYS_COLOR_INDEX = SYS_COLOR_INDEX(16u32);
pub const COLOR_BTNTEXT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(18u32);
pub const COLOR_CAPTIONTEXT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(9u32);
pub const COLOR_DESKTOP: SYS_COLOR_INDEX = SYS_COLOR_INDEX(1u32);
pub const COLOR_GRADIENTACTIVECAPTION: SYS_COLOR_INDEX = SYS_COLOR_INDEX(27u32);
pub const COLOR_GRADIENTINACTIVECAPTION: SYS_COLOR_INDEX = SYS_COLOR_INDEX(28u32);
pub const COLOR_GRAYTEXT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(17u32);
pub const COLOR_HIGHLIGHT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(13u32);
pub const COLOR_HIGHLIGHTTEXT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(14u32);
pub const COLOR_HOTLIGHT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(26u32);
pub const COLOR_INACTIVEBORDER: SYS_COLOR_INDEX = SYS_COLOR_INDEX(11u32);
pub const COLOR_INACTIVECAPTION: SYS_COLOR_INDEX = SYS_COLOR_INDEX(3u32);
pub const COLOR_INACTIVECAPTIONTEXT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(19u32);
pub const COLOR_INFOBK: SYS_COLOR_INDEX = SYS_COLOR_INDEX(24u32);
pub const COLOR_INFOTEXT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(23u32);
pub const COLOR_MENU: SYS_COLOR_INDEX = SYS_COLOR_INDEX(4u32);
pub const COLOR_MENUHILIGHT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(29u32);
pub const COLOR_MENUBAR: SYS_COLOR_INDEX = SYS_COLOR_INDEX(30u32);
pub const COLOR_MENUTEXT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(7u32);
pub const COLOR_SCROLLBAR: SYS_COLOR_INDEX = SYS_COLOR_INDEX(0u32);
pub const COLOR_WINDOW: SYS_COLOR_INDEX = SYS_COLOR_INDEX(5u32);
pub const COLOR_WINDOWFRAME: SYS_COLOR_INDEX = SYS_COLOR_INDEX(6u32);
pub const COLOR_WINDOWTEXT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(8u32);
impl ::core::marker::Copy for SYS_COLOR_INDEX {}
impl ::core::clone::Clone for SYS_COLOR_INDEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYS_COLOR_INDEX {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYS_COLOR_INDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYS_COLOR_INDEX").field(&self.0).finish()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TILE_WINDOWS_HOW(pub u32);
pub const MDITILE_HORIZONTAL: TILE_WINDOWS_HOW = TILE_WINDOWS_HOW(1u32);
pub const MDITILE_VERTICAL: TILE_WINDOWS_HOW = TILE_WINDOWS_HOW(0u32);
impl ::core::marker::Copy for TILE_WINDOWS_HOW {}
impl ::core::clone::Clone for TILE_WINDOWS_HOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TILE_WINDOWS_HOW {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TILE_WINDOWS_HOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TILE_WINDOWS_HOW").field(&self.0).finish()
    }
}
pub type TIMERPROC = ::core::option::Option<
    unsafe extern "system" fn(
        param0: super::super::Foundation::HWND,
        param1: u32,
        param2: usize,
        param3: u32,
    ),
>;
pub const TIMERV_COALESCING_MAX: u32 = 2147483637u32;
pub const TIMERV_COALESCING_MIN: u32 = 1u32;
pub const TIMERV_DEFAULT_COALESCING: u32 = 0u32;
pub const TIMERV_NO_COALESCING: u32 = 4294967295u32;
pub struct TITLEBARINFO {
    pub cbSize: u32,
    pub rcTitleBar: super::super::Foundation::RECT,
    pub rgstate: [u32; 6],
}
impl ::core::marker::Copy for TITLEBARINFO {}
impl ::core::clone::Clone for TITLEBARINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TITLEBARINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TITLEBARINFO")
            .field("cbSize", &self.cbSize)
            .field("rcTitleBar", &self.rcTitleBar)
            .field("rgstate", &self.rgstate)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TITLEBARINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.rcTitleBar == other.rcTitleBar
            && self.rgstate == other.rgstate
    }
}
impl ::core::cmp::Eq for TITLEBARINFO {}
pub struct TITLEBARINFOEX {
    pub cbSize: u32,
    pub rcTitleBar: super::super::Foundation::RECT,
    pub rgstate: [u32; 6],
    pub rgrect: [super::super::Foundation::RECT; 6],
}
impl ::core::marker::Copy for TITLEBARINFOEX {}
impl ::core::clone::Clone for TITLEBARINFOEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TITLEBARINFOEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TITLEBARINFOEX")
            .field("cbSize", &self.cbSize)
            .field("rcTitleBar", &self.rcTitleBar)
            .field("rgstate", &self.rgstate)
            .field("rgrect", &self.rgrect)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TITLEBARINFOEX {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.rcTitleBar == other.rcTitleBar
            && self.rgstate == other.rgstate
            && self.rgrect == other.rgrect
    }
}
impl ::core::cmp::Eq for TITLEBARINFOEX {}
pub const TKF_AVAILABLE: u32 = 2u32;
pub const TKF_CONFIRMHOTKEY: u32 = 8u32;
pub const TKF_HOTKEYACTIVE: u32 = 4u32;
pub const TKF_HOTKEYSOUND: u32 = 16u32;
pub const TKF_INDICATOR: u32 = 32u32;
pub const TKF_TOGGLEKEYSON: u32 = 1u32;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_LATENCY: u32 = 8u32;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_RLS_DELTA: f32 = 0.001f32;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_RLS_EXPO_SMOOTH_ALPHA: f32 = 0.99f32;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_RLS_LAMBDA_LEARNING_RATE: f32 = 0.001f32;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_RLS_LAMBDA_MAX: f32 = 0.999f32;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_RLS_LAMBDA_MIN: f32 = 0.9f32;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_SAMPLETIME: u32 = 8u32;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_USE_HW_TIMESTAMP: u32 = 1u32;
pub const TOUCH_FLAG_NONE: u32 = 0u32;
pub const TOUCH_HIT_TESTING_CLIENT: u32 = 1u32;
pub const TOUCH_HIT_TESTING_DEFAULT: u32 = 0u32;
pub const TOUCH_HIT_TESTING_NONE: u32 = 2u32;
pub const TOUCH_HIT_TESTING_PROXIMITY_CLOSEST: u32 = 0u32;
pub const TOUCH_HIT_TESTING_PROXIMITY_FARTHEST: u32 = 4095u32;
pub const TOUCH_MASK_CONTACTAREA: u32 = 1u32;
pub const TOUCH_MASK_NONE: u32 = 0u32;
pub const TOUCH_MASK_ORIENTATION: u32 = 2u32;
pub const TOUCH_MASK_PRESSURE: u32 = 4u32;
pub struct TPMPARAMS {
    pub cbSize: u32,
    pub rcExclude: super::super::Foundation::RECT,
}
impl ::core::marker::Copy for TPMPARAMS {}
impl ::core::clone::Clone for TPMPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TPMPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TPMPARAMS")
            .field("cbSize", &self.cbSize)
            .field("rcExclude", &self.rcExclude)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TPMPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.rcExclude == other.rcExclude
    }
}
impl ::core::cmp::Eq for TPMPARAMS {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TRACK_POPUP_MENU_FLAGS(pub u32);
pub const TPM_LEFTBUTTON: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(0u32);
pub const TPM_RIGHTBUTTON: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(2u32);
pub const TPM_LEFTALIGN: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(0u32);
pub const TPM_CENTERALIGN: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(4u32);
pub const TPM_RIGHTALIGN: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(8u32);
pub const TPM_TOPALIGN: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(0u32);
pub const TPM_VCENTERALIGN: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(16u32);
pub const TPM_BOTTOMALIGN: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(32u32);
pub const TPM_HORIZONTAL: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(0u32);
pub const TPM_VERTICAL: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(64u32);
pub const TPM_NONOTIFY: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(128u32);
pub const TPM_RETURNCMD: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(256u32);
pub const TPM_RECURSE: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(1u32);
pub const TPM_HORPOSANIMATION: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(1024u32);
pub const TPM_HORNEGANIMATION: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(2048u32);
pub const TPM_VERPOSANIMATION: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(4096u32);
pub const TPM_VERNEGANIMATION: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(8192u32);
pub const TPM_NOANIMATION: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(16384u32);
pub const TPM_LAYOUTRTL: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(32768u32);
pub const TPM_WORKAREA: TRACK_POPUP_MENU_FLAGS = TRACK_POPUP_MENU_FLAGS(65536u32);
impl ::core::marker::Copy for TRACK_POPUP_MENU_FLAGS {}
impl ::core::clone::Clone for TRACK_POPUP_MENU_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TRACK_POPUP_MENU_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TRACK_POPUP_MENU_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRACK_POPUP_MENU_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for TRACK_POPUP_MENU_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TRACK_POPUP_MENU_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TRACK_POPUP_MENU_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TRACK_POPUP_MENU_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TRACK_POPUP_MENU_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub struct TouchPredictionParameters {
    pub cbSize: u32,
    pub dwLatency: u32,
    pub dwSampleTime: u32,
    pub bUseHWTimeStamp: u32,
}
impl ::core::marker::Copy for TouchPredictionParameters {}
impl ::core::clone::Clone for TouchPredictionParameters {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TouchPredictionParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TouchPredictionParameters")
            .field("cbSize", &self.cbSize)
            .field("dwLatency", &self.dwLatency)
            .field("dwSampleTime", &self.dwSampleTime)
            .field("bUseHWTimeStamp", &self.bUseHWTimeStamp)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TouchPredictionParameters {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.dwLatency == other.dwLatency
            && self.dwSampleTime == other.dwSampleTime
            && self.bUseHWTimeStamp == other.bUseHWTimeStamp
    }
}
impl ::core::cmp::Eq for TouchPredictionParameters {}
pub const UISF_ACTIVE: u32 = 4u32;
pub const UISF_HIDEACCEL: u32 = 2u32;
pub const UISF_HIDEFOCUS: u32 = 1u32;
pub const UIS_CLEAR: u32 = 2u32;
pub const UIS_INITIALIZE: u32 = 3u32;
pub const UIS_SET: u32 = 1u32;
pub const UNICODE_NOCHAR: u32 = 65535u32;
pub const UOI_TIMERPROC_EXCEPTION_SUPPRESSION: u32 = 7u32;
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct UPDATELAYEREDWINDOWINFO {
    pub cbSize: u32,
    pub hdcDst: super::super::Graphics::Gdi::HDC,
    pub pptDst: ConstPtr<super::super::Foundation::POINT>,
    pub psize: ConstPtr<super::super::Foundation::SIZE>,
    pub hdcSrc: super::super::Graphics::Gdi::HDC,
    pub pptSrc: ConstPtr<super::super::Foundation::POINT>,
    pub crKey: u32,
    pub pblend: ConstPtr<super::super::Graphics::Gdi::BLENDFUNCTION>,
    pub dwFlags: UPDATE_LAYERED_WINDOW_FLAGS,
    pub prcDirty: ConstPtr<super::super::Foundation::RECT>,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for UPDATELAYEREDWINDOWINFO {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for UPDATELAYEREDWINDOWINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for UPDATELAYEREDWINDOWINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UPDATELAYEREDWINDOWINFO")
            .field("cbSize", &self.cbSize)
            .field("hdcDst", &self.hdcDst)
            .field("pptDst", &self.pptDst)
            .field("psize", &self.psize)
            .field("hdcSrc", &self.hdcSrc)
            .field("pptSrc", &self.pptSrc)
            .field("crKey", &self.crKey)
            .field("pblend", &self.pblend)
            .field("dwFlags", &self.dwFlags)
            .field("prcDirty", &self.prcDirty)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for UPDATELAYEREDWINDOWINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.hdcDst == other.hdcDst
            && self.pptDst == other.pptDst
            && self.psize == other.psize
            && self.hdcSrc == other.hdcSrc
            && self.pptSrc == other.pptSrc
            && self.crKey == other.crKey
            && self.pblend == other.pblend
            && self.dwFlags == other.dwFlags
            && self.prcDirty == other.prcDirty
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for UPDATELAYEREDWINDOWINFO {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UPDATE_LAYERED_WINDOW_FLAGS(pub u32);
pub const ULW_ALPHA: UPDATE_LAYERED_WINDOW_FLAGS = UPDATE_LAYERED_WINDOW_FLAGS(2u32);
pub const ULW_COLORKEY: UPDATE_LAYERED_WINDOW_FLAGS = UPDATE_LAYERED_WINDOW_FLAGS(1u32);
pub const ULW_OPAQUE: UPDATE_LAYERED_WINDOW_FLAGS = UPDATE_LAYERED_WINDOW_FLAGS(4u32);
pub const ULW_EX_NORESIZE: UPDATE_LAYERED_WINDOW_FLAGS = UPDATE_LAYERED_WINDOW_FLAGS(8u32);
impl ::core::marker::Copy for UPDATE_LAYERED_WINDOW_FLAGS {}
impl ::core::clone::Clone for UPDATE_LAYERED_WINDOW_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UPDATE_LAYERED_WINDOW_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UPDATE_LAYERED_WINDOW_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UPDATE_LAYERED_WINDOW_FLAGS")
            .field(&self.0)
            .finish()
    }
}
pub const USER_DEFAULT_SCREEN_DPI: u32 = 96u32;
pub const USER_TIMER_MAXIMUM: u32 = 2147483647u32;
pub const USER_TIMER_MINIMUM: u32 = 10u32;
pub const WA_ACTIVE: u32 = 1u32;
pub const WA_CLICKACTIVE: u32 = 2u32;
pub const WA_INACTIVE: u32 = 0u32;
pub const WHEEL_DELTA: u32 = 120u32;
pub const WH_HARDWARE: u32 = 8u32;
pub const WH_MAX: u32 = 14u32;
pub const WH_MAXHOOK: u32 = 14u32;
pub const WH_MIN: i32 = -1i32;
pub const WH_MINHOOK: i32 = -1i32;
pub struct WINDOWINFO {
    pub cbSize: u32,
    pub rcWindow: super::super::Foundation::RECT,
    pub rcClient: super::super::Foundation::RECT,
    pub dwStyle: u32,
    pub dwExStyle: u32,
    pub dwWindowStatus: u32,
    pub cxWindowBorders: u32,
    pub cyWindowBorders: u32,
    pub atomWindowType: u16,
    pub wCreatorVersion: u16,
}
impl ::core::marker::Copy for WINDOWINFO {}
impl ::core::clone::Clone for WINDOWINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINDOWINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINDOWINFO")
            .field("cbSize", &self.cbSize)
            .field("rcWindow", &self.rcWindow)
            .field("rcClient", &self.rcClient)
            .field("dwStyle", &self.dwStyle)
            .field("dwExStyle", &self.dwExStyle)
            .field("dwWindowStatus", &self.dwWindowStatus)
            .field("cxWindowBorders", &self.cxWindowBorders)
            .field("cyWindowBorders", &self.cyWindowBorders)
            .field("atomWindowType", &self.atomWindowType)
            .field("wCreatorVersion", &self.wCreatorVersion)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WINDOWINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.rcWindow == other.rcWindow
            && self.rcClient == other.rcClient
            && self.dwStyle == other.dwStyle
            && self.dwExStyle == other.dwExStyle
            && self.dwWindowStatus == other.dwWindowStatus
            && self.cxWindowBorders == other.cxWindowBorders
            && self.cyWindowBorders == other.cyWindowBorders
            && self.atomWindowType == other.atomWindowType
            && self.wCreatorVersion == other.wCreatorVersion
    }
}
impl ::core::cmp::Eq for WINDOWINFO {}
pub struct WINDOWPLACEMENT {
    pub length: u32,
    pub flags: WINDOWPLACEMENT_FLAGS,
    pub showCmd: SHOW_WINDOW_CMD,
    pub ptMinPosition: super::super::Foundation::POINT,
    pub ptMaxPosition: super::super::Foundation::POINT,
    pub rcNormalPosition: super::super::Foundation::RECT,
}
impl ::core::marker::Copy for WINDOWPLACEMENT {}
impl ::core::clone::Clone for WINDOWPLACEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINDOWPLACEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINDOWPLACEMENT")
            .field("length", &self.length)
            .field("flags", &self.flags)
            .field("showCmd", &self.showCmd)
            .field("ptMinPosition", &self.ptMinPosition)
            .field("ptMaxPosition", &self.ptMaxPosition)
            .field("rcNormalPosition", &self.rcNormalPosition)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WINDOWPLACEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length
            && self.flags == other.flags
            && self.showCmd == other.showCmd
            && self.ptMinPosition == other.ptMinPosition
            && self.ptMaxPosition == other.ptMaxPosition
            && self.rcNormalPosition == other.rcNormalPosition
    }
}
impl ::core::cmp::Eq for WINDOWPLACEMENT {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WINDOWPLACEMENT_FLAGS(pub u32);
pub const WPF_ASYNCWINDOWPLACEMENT: WINDOWPLACEMENT_FLAGS = WINDOWPLACEMENT_FLAGS(4u32);
pub const WPF_RESTORETOMAXIMIZED: WINDOWPLACEMENT_FLAGS = WINDOWPLACEMENT_FLAGS(2u32);
pub const WPF_SETMINPOSITION: WINDOWPLACEMENT_FLAGS = WINDOWPLACEMENT_FLAGS(1u32);
impl ::core::marker::Copy for WINDOWPLACEMENT_FLAGS {}
impl ::core::clone::Clone for WINDOWPLACEMENT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINDOWPLACEMENT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINDOWPLACEMENT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINDOWPLACEMENT_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for WINDOWPLACEMENT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WINDOWPLACEMENT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WINDOWPLACEMENT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WINDOWPLACEMENT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WINDOWPLACEMENT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub struct WINDOWPOS {
    pub hwnd: super::super::Foundation::HWND,
    pub hwndInsertAfter: super::super::Foundation::HWND,
    pub x: i32,
    pub y: i32,
    pub cx: i32,
    pub cy: i32,
    pub flags: SET_WINDOW_POS_FLAGS,
}
impl ::core::marker::Copy for WINDOWPOS {}
impl ::core::clone::Clone for WINDOWPOS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINDOWPOS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINDOWPOS")
            .field("hwnd", &self.hwnd)
            .field("hwndInsertAfter", &self.hwndInsertAfter)
            .field("x", &self.x)
            .field("y", &self.y)
            .field("cx", &self.cx)
            .field("cy", &self.cy)
            .field("flags", &self.flags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WINDOWPOS {
    fn eq(&self, other: &Self) -> bool {
        self.hwnd == other.hwnd
            && self.hwndInsertAfter == other.hwndInsertAfter
            && self.x == other.x
            && self.y == other.y
            && self.cx == other.cx
            && self.cy == other.cy
            && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for WINDOWPOS {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WINDOWS_HOOK_ID(pub i32);
pub const WH_CALLWNDPROC: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(4i32);
pub const WH_CALLWNDPROCRET: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(12i32);
pub const WH_CBT: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(5i32);
pub const WH_DEBUG: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(9i32);
pub const WH_FOREGROUNDIDLE: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(11i32);
pub const WH_GETMESSAGE: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(3i32);
pub const WH_JOURNALPLAYBACK: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(1i32);
pub const WH_JOURNALRECORD: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(0i32);
pub const WH_KEYBOARD: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(2i32);
pub const WH_KEYBOARD_LL: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(13i32);
pub const WH_MOUSE: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(7i32);
pub const WH_MOUSE_LL: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(14i32);
pub const WH_MSGFILTER: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(-1i32);
pub const WH_SHELL: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(10i32);
pub const WH_SYSMSGFILTER: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(6i32);
impl ::core::marker::Copy for WINDOWS_HOOK_ID {}
impl ::core::clone::Clone for WINDOWS_HOOK_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINDOWS_HOOK_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINDOWS_HOOK_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINDOWS_HOOK_ID").field(&self.0).finish()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WINDOW_DISPLAY_AFFINITY(pub u32);
pub const WDA_NONE: WINDOW_DISPLAY_AFFINITY = WINDOW_DISPLAY_AFFINITY(0u32);
pub const WDA_MONITOR: WINDOW_DISPLAY_AFFINITY = WINDOW_DISPLAY_AFFINITY(1u32);
pub const WDA_EXCLUDEFROMCAPTURE: WINDOW_DISPLAY_AFFINITY = WINDOW_DISPLAY_AFFINITY(17u32);
impl ::core::marker::Copy for WINDOW_DISPLAY_AFFINITY {}
impl ::core::clone::Clone for WINDOW_DISPLAY_AFFINITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINDOW_DISPLAY_AFFINITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINDOW_DISPLAY_AFFINITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINDOW_DISPLAY_AFFINITY")
            .field(&self.0)
            .finish()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WINDOW_EX_STYLE(pub u32);
pub const WS_EX_DLGMODALFRAME: WINDOW_EX_STYLE = WINDOW_EX_STYLE(1u32);
pub const WS_EX_NOPARENTNOTIFY: WINDOW_EX_STYLE = WINDOW_EX_STYLE(4u32);
pub const WS_EX_TOPMOST: WINDOW_EX_STYLE = WINDOW_EX_STYLE(8u32);
pub const WS_EX_ACCEPTFILES: WINDOW_EX_STYLE = WINDOW_EX_STYLE(16u32);
pub const WS_EX_TRANSPARENT: WINDOW_EX_STYLE = WINDOW_EX_STYLE(32u32);
pub const WS_EX_MDICHILD: WINDOW_EX_STYLE = WINDOW_EX_STYLE(64u32);
pub const WS_EX_TOOLWINDOW: WINDOW_EX_STYLE = WINDOW_EX_STYLE(128u32);
pub const WS_EX_WINDOWEDGE: WINDOW_EX_STYLE = WINDOW_EX_STYLE(256u32);
pub const WS_EX_CLIENTEDGE: WINDOW_EX_STYLE = WINDOW_EX_STYLE(512u32);
pub const WS_EX_CONTEXTHELP: WINDOW_EX_STYLE = WINDOW_EX_STYLE(1024u32);
pub const WS_EX_RIGHT: WINDOW_EX_STYLE = WINDOW_EX_STYLE(4096u32);
pub const WS_EX_LEFT: WINDOW_EX_STYLE = WINDOW_EX_STYLE(0u32);
pub const WS_EX_RTLREADING: WINDOW_EX_STYLE = WINDOW_EX_STYLE(8192u32);
pub const WS_EX_LTRREADING: WINDOW_EX_STYLE = WINDOW_EX_STYLE(0u32);
pub const WS_EX_LEFTSCROLLBAR: WINDOW_EX_STYLE = WINDOW_EX_STYLE(16384u32);
pub const WS_EX_RIGHTSCROLLBAR: WINDOW_EX_STYLE = WINDOW_EX_STYLE(0u32);
pub const WS_EX_CONTROLPARENT: WINDOW_EX_STYLE = WINDOW_EX_STYLE(65536u32);
pub const WS_EX_STATICEDGE: WINDOW_EX_STYLE = WINDOW_EX_STYLE(131072u32);
pub const WS_EX_APPWINDOW: WINDOW_EX_STYLE = WINDOW_EX_STYLE(262144u32);
pub const WS_EX_OVERLAPPEDWINDOW: WINDOW_EX_STYLE = WINDOW_EX_STYLE(768u32);
pub const WS_EX_PALETTEWINDOW: WINDOW_EX_STYLE = WINDOW_EX_STYLE(392u32);
pub const WS_EX_LAYERED: WINDOW_EX_STYLE = WINDOW_EX_STYLE(524288u32);
pub const WS_EX_NOINHERITLAYOUT: WINDOW_EX_STYLE = WINDOW_EX_STYLE(1048576u32);
pub const WS_EX_NOREDIRECTIONBITMAP: WINDOW_EX_STYLE = WINDOW_EX_STYLE(2097152u32);
pub const WS_EX_LAYOUTRTL: WINDOW_EX_STYLE = WINDOW_EX_STYLE(4194304u32);
pub const WS_EX_COMPOSITED: WINDOW_EX_STYLE = WINDOW_EX_STYLE(33554432u32);
pub const WS_EX_NOACTIVATE: WINDOW_EX_STYLE = WINDOW_EX_STYLE(134217728u32);
impl ::core::marker::Copy for WINDOW_EX_STYLE {}
impl ::core::clone::Clone for WINDOW_EX_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINDOW_EX_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINDOW_EX_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINDOW_EX_STYLE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WINDOW_EX_STYLE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WINDOW_EX_STYLE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WINDOW_EX_STYLE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WINDOW_EX_STYLE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WINDOW_EX_STYLE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WINDOW_LONG_PTR_INDEX(pub i32);
pub const GWL_EXSTYLE: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-20i32);
pub const GWLP_HINSTANCE: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-6i32);
pub const GWLP_HWNDPARENT: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-8i32);
pub const GWLP_ID: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-12i32);
pub const GWL_STYLE: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-16i32);
pub const GWLP_USERDATA: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-21i32);
pub const GWLP_WNDPROC: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-4i32);
pub const GWL_HINSTANCE: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-6i32);
pub const GWL_ID: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-12i32);
pub const GWL_USERDATA: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-21i32);
pub const GWL_WNDPROC: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-4i32);
pub const GWL_HWNDPARENT: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-8i32);
impl ::core::marker::Copy for WINDOW_LONG_PTR_INDEX {}
impl ::core::clone::Clone for WINDOW_LONG_PTR_INDEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINDOW_LONG_PTR_INDEX {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINDOW_LONG_PTR_INDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINDOW_LONG_PTR_INDEX")
            .field(&self.0)
            .finish()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WINDOW_MESSAGE_FILTER_ACTION(pub u32);
pub const MSGFLT_ALLOW: WINDOW_MESSAGE_FILTER_ACTION = WINDOW_MESSAGE_FILTER_ACTION(1u32);
pub const MSGFLT_DISALLOW: WINDOW_MESSAGE_FILTER_ACTION = WINDOW_MESSAGE_FILTER_ACTION(2u32);
pub const MSGFLT_RESET: WINDOW_MESSAGE_FILTER_ACTION = WINDOW_MESSAGE_FILTER_ACTION(0u32);
impl ::core::marker::Copy for WINDOW_MESSAGE_FILTER_ACTION {}
impl ::core::clone::Clone for WINDOW_MESSAGE_FILTER_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINDOW_MESSAGE_FILTER_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINDOW_MESSAGE_FILTER_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINDOW_MESSAGE_FILTER_ACTION")
            .field(&self.0)
            .finish()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WINDOW_STYLE(pub u32);
pub const WS_OVERLAPPED: WINDOW_STYLE = WINDOW_STYLE(0u32);
pub const WS_POPUP: WINDOW_STYLE = WINDOW_STYLE(2147483648u32);
pub const WS_CHILD: WINDOW_STYLE = WINDOW_STYLE(1073741824u32);
pub const WS_MINIMIZE: WINDOW_STYLE = WINDOW_STYLE(536870912u32);
pub const WS_VISIBLE: WINDOW_STYLE = WINDOW_STYLE(268435456u32);
pub const WS_DISABLED: WINDOW_STYLE = WINDOW_STYLE(134217728u32);
pub const WS_CLIPSIBLINGS: WINDOW_STYLE = WINDOW_STYLE(67108864u32);
pub const WS_CLIPCHILDREN: WINDOW_STYLE = WINDOW_STYLE(33554432u32);
pub const WS_MAXIMIZE: WINDOW_STYLE = WINDOW_STYLE(16777216u32);
pub const WS_CAPTION: WINDOW_STYLE = WINDOW_STYLE(12582912u32);
pub const WS_BORDER: WINDOW_STYLE = WINDOW_STYLE(8388608u32);
pub const WS_DLGFRAME: WINDOW_STYLE = WINDOW_STYLE(4194304u32);
pub const WS_VSCROLL: WINDOW_STYLE = WINDOW_STYLE(2097152u32);
pub const WS_HSCROLL: WINDOW_STYLE = WINDOW_STYLE(1048576u32);
pub const WS_SYSMENU: WINDOW_STYLE = WINDOW_STYLE(524288u32);
pub const WS_THICKFRAME: WINDOW_STYLE = WINDOW_STYLE(262144u32);
pub const WS_GROUP: WINDOW_STYLE = WINDOW_STYLE(131072u32);
pub const WS_TABSTOP: WINDOW_STYLE = WINDOW_STYLE(65536u32);
pub const WS_MINIMIZEBOX: WINDOW_STYLE = WINDOW_STYLE(131072u32);
pub const WS_MAXIMIZEBOX: WINDOW_STYLE = WINDOW_STYLE(65536u32);
pub const WS_TILED: WINDOW_STYLE = WINDOW_STYLE(0u32);
pub const WS_ICONIC: WINDOW_STYLE = WINDOW_STYLE(536870912u32);
pub const WS_SIZEBOX: WINDOW_STYLE = WINDOW_STYLE(262144u32);
pub const WS_TILEDWINDOW: WINDOW_STYLE = WINDOW_STYLE(13565952u32);
pub const WS_OVERLAPPEDWINDOW: WINDOW_STYLE = WINDOW_STYLE(13565952u32);
pub const WS_POPUPWINDOW: WINDOW_STYLE = WINDOW_STYLE(2156396544u32);
pub const WS_CHILDWINDOW: WINDOW_STYLE = WINDOW_STYLE(1073741824u32);
pub const WS_ACTIVECAPTION: WINDOW_STYLE = WINDOW_STYLE(1u32);
impl ::core::marker::Copy for WINDOW_STYLE {}
impl ::core::clone::Clone for WINDOW_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINDOW_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINDOW_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINDOW_STYLE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WINDOW_STYLE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WINDOW_STYLE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WINDOW_STYLE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WINDOW_STYLE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WINDOW_STYLE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const WINEVENT_INCONTEXT: u32 = 4u32;
pub const WINEVENT_OUTOFCONTEXT: u32 = 0u32;
pub const WINEVENT_SKIPOWNPROCESS: u32 = 2u32;
pub const WINEVENT_SKIPOWNTHREAD: u32 = 1u32;
pub const WINSTA_ACCESSCLIPBOARD: i32 = 4i32;
pub const WINSTA_ACCESSGLOBALATOMS: i32 = 32i32;
pub const WINSTA_CREATEDESKTOP: i32 = 8i32;
pub const WINSTA_ENUMDESKTOPS: i32 = 1i32;
pub const WINSTA_ENUMERATE: i32 = 256i32;
pub const WINSTA_EXITWINDOWS: i32 = 64i32;
pub const WINSTA_READATTRIBUTES: i32 = 2i32;
pub const WINSTA_READSCREEN: i32 = 512i32;
pub const WINSTA_WRITEATTRIBUTES: i32 = 16i32;
pub const WMSZ_BOTTOM: u32 = 6u32;
pub const WMSZ_BOTTOMLEFT: u32 = 7u32;
pub const WMSZ_BOTTOMRIGHT: u32 = 8u32;
pub const WMSZ_LEFT: u32 = 1u32;
pub const WMSZ_RIGHT: u32 = 2u32;
pub const WMSZ_TOP: u32 = 3u32;
pub const WMSZ_TOPLEFT: u32 = 4u32;
pub const WMSZ_TOPRIGHT: u32 = 5u32;
pub const WM_ACTIVATE: u32 = 6u32;
pub const WM_ACTIVATEAPP: u32 = 28u32;
pub const WM_AFXFIRST: u32 = 864u32;
pub const WM_AFXLAST: u32 = 895u32;
pub const WM_APP: u32 = 32768u32;
pub const WM_APPCOMMAND: u32 = 793u32;
pub const WM_ASKCBFORMATNAME: u32 = 780u32;
pub const WM_CANCELJOURNAL: u32 = 75u32;
pub const WM_CANCELMODE: u32 = 31u32;
pub const WM_CAPTURECHANGED: u32 = 533u32;
pub const WM_CHANGECBCHAIN: u32 = 781u32;
pub const WM_CHANGEUISTATE: u32 = 295u32;
pub const WM_CHAR: u32 = 258u32;
pub const WM_CHARTOITEM: u32 = 47u32;
pub const WM_CHILDACTIVATE: u32 = 34u32;
pub const WM_CLEAR: u32 = 771u32;
pub const WM_CLIPBOARDUPDATE: u32 = 797u32;
pub const WM_CLOSE: u32 = 16u32;
pub const WM_COMMAND: u32 = 273u32;
pub const WM_COMMNOTIFY: u32 = 68u32;
pub const WM_COMPACTING: u32 = 65u32;
pub const WM_COMPAREITEM: u32 = 57u32;
pub const WM_COPY: u32 = 769u32;
pub const WM_COPYDATA: u32 = 74u32;
pub const WM_CREATE: u32 = 1u32;
pub const WM_CTLCOLORBTN: u32 = 309u32;
pub const WM_CTLCOLORDLG: u32 = 310u32;
pub const WM_CTLCOLOREDIT: u32 = 307u32;
pub const WM_CTLCOLORLISTBOX: u32 = 308u32;
pub const WM_CTLCOLORMSGBOX: u32 = 306u32;
pub const WM_CTLCOLORSCROLLBAR: u32 = 311u32;
pub const WM_CTLCOLORSTATIC: u32 = 312u32;
pub const WM_CUT: u32 = 768u32;
pub const WM_DEADCHAR: u32 = 259u32;
pub const WM_DELETEITEM: u32 = 45u32;
pub const WM_DESTROY: u32 = 2u32;
pub const WM_DESTROYCLIPBOARD: u32 = 775u32;
pub const WM_DEVICECHANGE: u32 = 537u32;
pub const WM_DEVMODECHANGE: u32 = 27u32;
pub const WM_DISPLAYCHANGE: u32 = 126u32;
pub const WM_DPICHANGED: u32 = 736u32;
pub const WM_DPICHANGED_AFTERPARENT: u32 = 739u32;
pub const WM_DPICHANGED_BEFOREPARENT: u32 = 738u32;
pub const WM_DRAWCLIPBOARD: u32 = 776u32;
pub const WM_DRAWITEM: u32 = 43u32;
pub const WM_DROPFILES: u32 = 563u32;
pub const WM_DWMCOLORIZATIONCOLORCHANGED: u32 = 800u32;
pub const WM_DWMCOMPOSITIONCHANGED: u32 = 798u32;
pub const WM_DWMNCRENDERINGCHANGED: u32 = 799u32;
pub const WM_DWMSENDICONICLIVEPREVIEWBITMAP: u32 = 806u32;
pub const WM_DWMSENDICONICTHUMBNAIL: u32 = 803u32;
pub const WM_DWMWINDOWMAXIMIZEDCHANGE: u32 = 801u32;
pub const WM_ENABLE: u32 = 10u32;
pub const WM_ENDSESSION: u32 = 22u32;
pub const WM_ENTERIDLE: u32 = 289u32;
pub const WM_ENTERMENULOOP: u32 = 529u32;
pub const WM_ENTERSIZEMOVE: u32 = 561u32;
pub const WM_ERASEBKGND: u32 = 20u32;
pub const WM_EXITMENULOOP: u32 = 530u32;
pub const WM_EXITSIZEMOVE: u32 = 562u32;
pub const WM_FONTCHANGE: u32 = 29u32;
pub const WM_GESTURE: u32 = 281u32;
pub const WM_GESTURENOTIFY: u32 = 282u32;
pub const WM_GETDLGCODE: u32 = 135u32;
pub const WM_GETDPISCALEDSIZE: u32 = 740u32;
pub const WM_GETFONT: u32 = 49u32;
pub const WM_GETHOTKEY: u32 = 51u32;
pub const WM_GETICON: u32 = 127u32;
pub const WM_GETMINMAXINFO: u32 = 36u32;
pub const WM_GETOBJECT: u32 = 61u32;
pub const WM_GETTEXT: u32 = 13u32;
pub const WM_GETTEXTLENGTH: u32 = 14u32;
pub const WM_GETTITLEBARINFOEX: u32 = 831u32;
pub const WM_HANDHELDFIRST: u32 = 856u32;
pub const WM_HANDHELDLAST: u32 = 863u32;
pub const WM_HELP: u32 = 83u32;
pub const WM_HOTKEY: u32 = 786u32;
pub const WM_HSCROLL: u32 = 276u32;
pub const WM_HSCROLLCLIPBOARD: u32 = 782u32;
pub const WM_ICONERASEBKGND: u32 = 39u32;
pub const WM_IME_CHAR: u32 = 646u32;
pub const WM_IME_COMPOSITION: u32 = 271u32;
pub const WM_IME_COMPOSITIONFULL: u32 = 644u32;
pub const WM_IME_CONTROL: u32 = 643u32;
pub const WM_IME_ENDCOMPOSITION: u32 = 270u32;
pub const WM_IME_KEYDOWN: u32 = 656u32;
pub const WM_IME_KEYLAST: u32 = 271u32;
pub const WM_IME_KEYUP: u32 = 657u32;
pub const WM_IME_NOTIFY: u32 = 642u32;
pub const WM_IME_REQUEST: u32 = 648u32;
pub const WM_IME_SELECT: u32 = 645u32;
pub const WM_IME_SETCONTEXT: u32 = 641u32;
pub const WM_IME_STARTCOMPOSITION: u32 = 269u32;
pub const WM_INITDIALOG: u32 = 272u32;
pub const WM_INITMENU: u32 = 278u32;
pub const WM_INITMENUPOPUP: u32 = 279u32;
pub const WM_INPUT: u32 = 255u32;
pub const WM_INPUTLANGCHANGE: u32 = 81u32;
pub const WM_INPUTLANGCHANGEREQUEST: u32 = 80u32;
pub const WM_INPUT_DEVICE_CHANGE: u32 = 254u32;
pub const WM_KEYDOWN: u32 = 256u32;
pub const WM_KEYFIRST: u32 = 256u32;
pub const WM_KEYLAST: u32 = 265u32;
pub const WM_KEYUP: u32 = 257u32;
pub const WM_KILLFOCUS: u32 = 8u32;
pub const WM_LBUTTONDBLCLK: u32 = 515u32;
pub const WM_LBUTTONDOWN: u32 = 513u32;
pub const WM_LBUTTONUP: u32 = 514u32;
pub const WM_MBUTTONDBLCLK: u32 = 521u32;
pub const WM_MBUTTONDOWN: u32 = 519u32;
pub const WM_MBUTTONUP: u32 = 520u32;
pub const WM_MDIACTIVATE: u32 = 546u32;
pub const WM_MDICASCADE: u32 = 551u32;
pub const WM_MDICREATE: u32 = 544u32;
pub const WM_MDIDESTROY: u32 = 545u32;
pub const WM_MDIGETACTIVE: u32 = 553u32;
pub const WM_MDIICONARRANGE: u32 = 552u32;
pub const WM_MDIMAXIMIZE: u32 = 549u32;
pub const WM_MDINEXT: u32 = 548u32;
pub const WM_MDIREFRESHMENU: u32 = 564u32;
pub const WM_MDIRESTORE: u32 = 547u32;
pub const WM_MDISETMENU: u32 = 560u32;
pub const WM_MDITILE: u32 = 550u32;
pub const WM_MEASUREITEM: u32 = 44u32;
pub const WM_MENUCHAR: u32 = 288u32;
pub const WM_MENUCOMMAND: u32 = 294u32;
pub const WM_MENUDRAG: u32 = 291u32;
pub const WM_MENUGETOBJECT: u32 = 292u32;
pub const WM_MENURBUTTONUP: u32 = 290u32;
pub const WM_MENUSELECT: u32 = 287u32;
pub const WM_MOUSEACTIVATE: u32 = 33u32;
pub const WM_MOUSEFIRST: u32 = 512u32;
pub const WM_MOUSEHWHEEL: u32 = 526u32;
pub const WM_MOUSELAST: u32 = 526u32;
pub const WM_MOUSEMOVE: u32 = 512u32;
pub const WM_MOUSEWHEEL: u32 = 522u32;
pub const WM_MOVE: u32 = 3u32;
pub const WM_MOVING: u32 = 534u32;
pub const WM_NCACTIVATE: u32 = 134u32;
pub const WM_NCCALCSIZE: u32 = 131u32;
pub const WM_NCCREATE: u32 = 129u32;
pub const WM_NCDESTROY: u32 = 130u32;
pub const WM_NCHITTEST: u32 = 132u32;
pub const WM_NCLBUTTONDBLCLK: u32 = 163u32;
pub const WM_NCLBUTTONDOWN: u32 = 161u32;
pub const WM_NCLBUTTONUP: u32 = 162u32;
pub const WM_NCMBUTTONDBLCLK: u32 = 169u32;
pub const WM_NCMBUTTONDOWN: u32 = 167u32;
pub const WM_NCMBUTTONUP: u32 = 168u32;
pub const WM_NCMOUSEHOVER: u32 = 672u32;
pub const WM_NCMOUSELEAVE: u32 = 674u32;
pub const WM_NCMOUSEMOVE: u32 = 160u32;
pub const WM_NCPAINT: u32 = 133u32;
pub const WM_NCPOINTERDOWN: u32 = 578u32;
pub const WM_NCPOINTERUP: u32 = 579u32;
pub const WM_NCPOINTERUPDATE: u32 = 577u32;
pub const WM_NCRBUTTONDBLCLK: u32 = 166u32;
pub const WM_NCRBUTTONDOWN: u32 = 164u32;
pub const WM_NCRBUTTONUP: u32 = 165u32;
pub const WM_NCXBUTTONDBLCLK: u32 = 173u32;
pub const WM_NCXBUTTONDOWN: u32 = 171u32;
pub const WM_NCXBUTTONUP: u32 = 172u32;
pub const WM_NEXTDLGCTL: u32 = 40u32;
pub const WM_NEXTMENU: u32 = 531u32;
pub const WM_NOTIFYFORMAT: u32 = 85u32;
pub const WM_NULL: u32 = 0u32;
pub const WM_PAINT: u32 = 15u32;
pub const WM_PAINTCLIPBOARD: u32 = 777u32;
pub const WM_PAINTICON: u32 = 38u32;
pub const WM_PALETTECHANGED: u32 = 785u32;
pub const WM_PALETTEISCHANGING: u32 = 784u32;
pub const WM_PARENTNOTIFY: u32 = 528u32;
pub const WM_PASTE: u32 = 770u32;
pub const WM_PENWINFIRST: u32 = 896u32;
pub const WM_PENWINLAST: u32 = 911u32;
pub const WM_POINTERACTIVATE: u32 = 587u32;
pub const WM_POINTERCAPTURECHANGED: u32 = 588u32;
pub const WM_POINTERDEVICECHANGE: u32 = 568u32;
pub const WM_POINTERDEVICEINRANGE: u32 = 569u32;
pub const WM_POINTERDEVICEOUTOFRANGE: u32 = 570u32;
pub const WM_POINTERDOWN: u32 = 582u32;
pub const WM_POINTERENTER: u32 = 585u32;
pub const WM_POINTERHWHEEL: u32 = 591u32;
pub const WM_POINTERLEAVE: u32 = 586u32;
pub const WM_POINTERROUTEDAWAY: u32 = 594u32;
pub const WM_POINTERROUTEDRELEASED: u32 = 595u32;
pub const WM_POINTERROUTEDTO: u32 = 593u32;
pub const WM_POINTERUP: u32 = 583u32;
pub const WM_POINTERUPDATE: u32 = 581u32;
pub const WM_POINTERWHEEL: u32 = 590u32;
pub const WM_POWER: u32 = 72u32;
pub const WM_POWERBROADCAST: u32 = 536u32;
pub const WM_PRINT: u32 = 791u32;
pub const WM_QUERYDRAGICON: u32 = 55u32;
pub const WM_QUERYENDSESSION: u32 = 17u32;
pub const WM_QUERYNEWPALETTE: u32 = 783u32;
pub const WM_QUERYOPEN: u32 = 19u32;
pub const WM_QUERYUISTATE: u32 = 297u32;
pub const WM_QUEUESYNC: u32 = 35u32;
pub const WM_QUIT: u32 = 18u32;
pub const WM_RBUTTONDBLCLK: u32 = 518u32;
pub const WM_RBUTTONDOWN: u32 = 516u32;
pub const WM_RBUTTONUP: u32 = 517u32;
pub const WM_RENDERALLFORMATS: u32 = 774u32;
pub const WM_RENDERFORMAT: u32 = 773u32;
pub const WM_SETCURSOR: u32 = 32u32;
pub const WM_SETFOCUS: u32 = 7u32;
pub const WM_SETFONT: u32 = 48u32;
pub const WM_SETHOTKEY: u32 = 50u32;
pub const WM_SETICON: u32 = 128u32;
pub const WM_SETREDRAW: u32 = 11u32;
pub const WM_SETTEXT: u32 = 12u32;
pub const WM_SETTINGCHANGE: u32 = 26u32;
pub const WM_SHOWWINDOW: u32 = 24u32;
pub const WM_SIZE: u32 = 5u32;
pub const WM_SIZECLIPBOARD: u32 = 779u32;
pub const WM_SIZING: u32 = 532u32;
pub const WM_SPOOLERSTATUS: u32 = 42u32;
pub const WM_STYLECHANGED: u32 = 125u32;
pub const WM_STYLECHANGING: u32 = 124u32;
pub const WM_SYNCPAINT: u32 = 136u32;
pub const WM_SYSCHAR: u32 = 262u32;
pub const WM_SYSCOLORCHANGE: u32 = 21u32;
pub const WM_SYSCOMMAND: u32 = 274u32;
pub const WM_SYSDEADCHAR: u32 = 263u32;
pub const WM_SYSKEYDOWN: u32 = 260u32;
pub const WM_SYSKEYUP: u32 = 261u32;
pub const WM_TABLET_FIRST: u32 = 704u32;
pub const WM_TABLET_LAST: u32 = 735u32;
pub const WM_TCARD: u32 = 82u32;
pub const WM_THEMECHANGED: u32 = 794u32;
pub const WM_TIMECHANGE: u32 = 30u32;
pub const WM_TIMER: u32 = 275u32;
pub const WM_TOUCH: u32 = 576u32;
pub const WM_TOUCHHITTESTING: u32 = 589u32;
pub const WM_UNDO: u32 = 772u32;
pub const WM_UNINITMENUPOPUP: u32 = 293u32;
pub const WM_UPDATEUISTATE: u32 = 296u32;
pub const WM_USER: u32 = 1024u32;
pub const WM_USERCHANGED: u32 = 84u32;
pub const WM_VKEYTOITEM: u32 = 46u32;
pub const WM_VSCROLL: u32 = 277u32;
pub const WM_VSCROLLCLIPBOARD: u32 = 778u32;
pub const WM_WINDOWPOSCHANGED: u32 = 71u32;
pub const WM_WINDOWPOSCHANGING: u32 = 70u32;
pub const WM_WININICHANGE: u32 = 26u32;
pub const WM_WTSSESSION_CHANGE: u32 = 689u32;
pub const WM_XBUTTONDBLCLK: u32 = 525u32;
pub const WM_XBUTTONDOWN: u32 = 523u32;
pub const WM_XBUTTONUP: u32 = 524u32;
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct WNDCLASSA {
    pub style: WNDCLASS_STYLES,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: super::super::Graphics::Gdi::HBRUSH,
    pub lpszMenuName: ::win32::core::PCSTR,
    pub lpszClassName: ::win32::core::PCSTR,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for WNDCLASSA {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for WNDCLASSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for WNDCLASSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WNDCLASSA")
            .field("style", &self.style)
            .field("lpfnWndProc", &self.lpfnWndProc.map(|f| f as usize))
            .field("cbClsExtra", &self.cbClsExtra)
            .field("cbWndExtra", &self.cbWndExtra)
            .field("hInstance", &self.hInstance)
            .field("hIcon", &self.hIcon)
            .field("hCursor", &self.hCursor)
            .field("hbrBackground", &self.hbrBackground)
            .field("lpszMenuName", &self.lpszMenuName)
            .field("lpszClassName", &self.lpszClassName)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for WNDCLASSA {
    fn eq(&self, other: &Self) -> bool {
        self.style == other.style
            && self.lpfnWndProc.map(|f| f as usize) == other.lpfnWndProc.map(|f| f as usize)
            && self.cbClsExtra == other.cbClsExtra
            && self.cbWndExtra == other.cbWndExtra
            && self.hInstance == other.hInstance
            && self.hIcon == other.hIcon
            && self.hCursor == other.hCursor
            && self.hbrBackground == other.hbrBackground
            && self.lpszMenuName == other.lpszMenuName
            && self.lpszClassName == other.lpszClassName
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for WNDCLASSA {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct WNDCLASSEXA {
    pub cbSize: u32,
    pub style: WNDCLASS_STYLES,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: super::super::Graphics::Gdi::HBRUSH,
    pub lpszMenuName: ::win32::core::PCSTR,
    pub lpszClassName: ::win32::core::PCSTR,
    pub hIconSm: HICON,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for WNDCLASSEXA {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for WNDCLASSEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for WNDCLASSEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WNDCLASSEXA")
            .field("cbSize", &self.cbSize)
            .field("style", &self.style)
            .field("lpfnWndProc", &self.lpfnWndProc.map(|f| f as usize))
            .field("cbClsExtra", &self.cbClsExtra)
            .field("cbWndExtra", &self.cbWndExtra)
            .field("hInstance", &self.hInstance)
            .field("hIcon", &self.hIcon)
            .field("hCursor", &self.hCursor)
            .field("hbrBackground", &self.hbrBackground)
            .field("lpszMenuName", &self.lpszMenuName)
            .field("lpszClassName", &self.lpszClassName)
            .field("hIconSm", &self.hIconSm)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for WNDCLASSEXA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.style == other.style
            && self.lpfnWndProc.map(|f| f as usize) == other.lpfnWndProc.map(|f| f as usize)
            && self.cbClsExtra == other.cbClsExtra
            && self.cbWndExtra == other.cbWndExtra
            && self.hInstance == other.hInstance
            && self.hIcon == other.hIcon
            && self.hCursor == other.hCursor
            && self.hbrBackground == other.hbrBackground
            && self.lpszMenuName == other.lpszMenuName
            && self.lpszClassName == other.lpszClassName
            && self.hIconSm == other.hIconSm
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for WNDCLASSEXA {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct WNDCLASSEXW {
    pub cbSize: u32,
    pub style: WNDCLASS_STYLES,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: super::super::Graphics::Gdi::HBRUSH,
    pub lpszMenuName: ::win32::core::PCWSTR,
    pub lpszClassName: ::win32::core::PCWSTR,
    pub hIconSm: HICON,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for WNDCLASSEXW {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for WNDCLASSEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for WNDCLASSEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WNDCLASSEXW")
            .field("cbSize", &self.cbSize)
            .field("style", &self.style)
            .field("lpfnWndProc", &self.lpfnWndProc.map(|f| f as usize))
            .field("cbClsExtra", &self.cbClsExtra)
            .field("cbWndExtra", &self.cbWndExtra)
            .field("hInstance", &self.hInstance)
            .field("hIcon", &self.hIcon)
            .field("hCursor", &self.hCursor)
            .field("hbrBackground", &self.hbrBackground)
            .field("lpszMenuName", &self.lpszMenuName)
            .field("lpszClassName", &self.lpszClassName)
            .field("hIconSm", &self.hIconSm)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for WNDCLASSEXW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.style == other.style
            && self.lpfnWndProc.map(|f| f as usize) == other.lpfnWndProc.map(|f| f as usize)
            && self.cbClsExtra == other.cbClsExtra
            && self.cbWndExtra == other.cbWndExtra
            && self.hInstance == other.hInstance
            && self.hIcon == other.hIcon
            && self.hCursor == other.hCursor
            && self.hbrBackground == other.hbrBackground
            && self.lpszMenuName == other.lpszMenuName
            && self.lpszClassName == other.lpszClassName
            && self.hIconSm == other.hIconSm
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for WNDCLASSEXW {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct WNDCLASSW {
    pub style: WNDCLASS_STYLES,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: super::super::Graphics::Gdi::HBRUSH,
    pub lpszMenuName: ::win32::core::PCWSTR,
    pub lpszClassName: ::win32::core::PCWSTR,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for WNDCLASSW {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for WNDCLASSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for WNDCLASSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WNDCLASSW")
            .field("style", &self.style)
            .field("lpfnWndProc", &self.lpfnWndProc.map(|f| f as usize))
            .field("cbClsExtra", &self.cbClsExtra)
            .field("cbWndExtra", &self.cbWndExtra)
            .field("hInstance", &self.hInstance)
            .field("hIcon", &self.hIcon)
            .field("hCursor", &self.hCursor)
            .field("hbrBackground", &self.hbrBackground)
            .field("lpszMenuName", &self.lpszMenuName)
            .field("lpszClassName", &self.lpszClassName)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for WNDCLASSW {
    fn eq(&self, other: &Self) -> bool {
        self.style == other.style
            && self.lpfnWndProc.map(|f| f as usize) == other.lpfnWndProc.map(|f| f as usize)
            && self.cbClsExtra == other.cbClsExtra
            && self.cbWndExtra == other.cbWndExtra
            && self.hInstance == other.hInstance
            && self.hIcon == other.hIcon
            && self.hCursor == other.hCursor
            && self.hbrBackground == other.hbrBackground
            && self.lpszMenuName == other.lpszMenuName
            && self.lpszClassName == other.lpszClassName
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for WNDCLASSW {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WNDCLASS_STYLES(pub u32);
pub const CS_VREDRAW: WNDCLASS_STYLES = WNDCLASS_STYLES(1u32);
pub const CS_HREDRAW: WNDCLASS_STYLES = WNDCLASS_STYLES(2u32);
pub const CS_DBLCLKS: WNDCLASS_STYLES = WNDCLASS_STYLES(8u32);
pub const CS_OWNDC: WNDCLASS_STYLES = WNDCLASS_STYLES(32u32);
pub const CS_CLASSDC: WNDCLASS_STYLES = WNDCLASS_STYLES(64u32);
pub const CS_PARENTDC: WNDCLASS_STYLES = WNDCLASS_STYLES(128u32);
pub const CS_NOCLOSE: WNDCLASS_STYLES = WNDCLASS_STYLES(512u32);
pub const CS_SAVEBITS: WNDCLASS_STYLES = WNDCLASS_STYLES(2048u32);
pub const CS_BYTEALIGNCLIENT: WNDCLASS_STYLES = WNDCLASS_STYLES(4096u32);
pub const CS_BYTEALIGNWINDOW: WNDCLASS_STYLES = WNDCLASS_STYLES(8192u32);
pub const CS_GLOBALCLASS: WNDCLASS_STYLES = WNDCLASS_STYLES(16384u32);
pub const CS_IME: WNDCLASS_STYLES = WNDCLASS_STYLES(65536u32);
pub const CS_DROPSHADOW: WNDCLASS_STYLES = WNDCLASS_STYLES(131072u32);
impl ::core::marker::Copy for WNDCLASS_STYLES {}
impl ::core::clone::Clone for WNDCLASS_STYLES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WNDCLASS_STYLES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WNDCLASS_STYLES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WNDCLASS_STYLES").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WNDCLASS_STYLES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WNDCLASS_STYLES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WNDCLASS_STYLES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WNDCLASS_STYLES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WNDCLASS_STYLES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub type WNDENUMPROC = ::core::option::Option<
    unsafe extern "system" fn(
        param0: super::super::Foundation::HWND,
        param1: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::BOOL,
>;
pub type WNDPROC = ::core::option::Option<
    unsafe extern "system" fn(
        param0: super::super::Foundation::HWND,
        param1: u32,
        param2: super::super::Foundation::WPARAM,
        param3: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::LRESULT,
>;
pub const WSF_VISIBLE: i32 = 1i32;
pub const WTS_CONSOLE_CONNECT: u32 = 1u32;
pub const WTS_CONSOLE_DISCONNECT: u32 = 2u32;
pub const WTS_REMOTE_CONNECT: u32 = 3u32;
pub const WTS_REMOTE_DISCONNECT: u32 = 4u32;
pub const WTS_SESSION_CREATE: u32 = 10u32;
pub const WTS_SESSION_LOCK: u32 = 7u32;
pub const WTS_SESSION_LOGOFF: u32 = 6u32;
pub const WTS_SESSION_LOGON: u32 = 5u32;
pub const WTS_SESSION_REMOTE_CONTROL: u32 = 9u32;
pub const WTS_SESSION_TERMINATE: u32 = 11u32;
pub const WTS_SESSION_UNLOCK: u32 = 8u32;
pub const WVR_ALIGNBOTTOM: u32 = 64u32;
pub const WVR_ALIGNLEFT: u32 = 32u32;
pub const WVR_ALIGNRIGHT: u32 = 128u32;
pub const WVR_ALIGNTOP: u32 = 16u32;
pub const WVR_HREDRAW: u32 = 256u32;
pub const WVR_VALIDRECTS: u32 = 1024u32;
pub const WVR_VREDRAW: u32 = 512u32;
pub const __WARNING_BANNED_API_USAGE: u32 = 28719u32;
pub const __WARNING_CYCLOMATIC_COMPLEXITY: u32 = 28734u32;
pub const __WARNING_DEREF_NULL_PTR: u32 = 6011u32;
pub const __WARNING_HIGH_PRIORITY_OVERFLOW_POSTCONDITION: u32 = 26045u32;
pub const __WARNING_INCORRECT_ANNOTATION: u32 = 26007u32;
pub const __WARNING_INVALID_PARAM_VALUE_1: u32 = 6387u32;
pub const __WARNING_INVALID_PARAM_VALUE_3: u32 = 28183u32;
pub const __WARNING_MISSING_ZERO_TERMINATION2: u32 = 6054u32;
pub const __WARNING_POSTCONDITION_NULLTERMINATION_VIOLATION: u32 = 26036u32;
pub const __WARNING_POST_EXPECTED: u32 = 28210u32;
pub const __WARNING_POTENTIAL_BUFFER_OVERFLOW_HIGH_PRIORITY: u32 = 26015u32;
pub const __WARNING_POTENTIAL_RANGE_POSTCONDITION_VIOLATION: u32 = 26071u32;
pub const __WARNING_PRECONDITION_NULLTERMINATION_VIOLATION: u32 = 26035u32;
pub const __WARNING_RANGE_POSTCONDITION_VIOLATION: u32 = 26061u32;
pub const __WARNING_RETURNING_BAD_RESULT: u32 = 28196u32;
pub const __WARNING_RETURN_UNINIT_VAR: u32 = 6101u32;
pub const __WARNING_USING_UNINIT_VAR: u32 = 6001u32;
pub trait Api {
    fn AdjustWindowRect(
        lpRect: MutPtr<super::super::Foundation::RECT>,
        dwStyle: WINDOW_STYLE,
        bMenu: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn AdjustWindowRectEx(
        lpRect: MutPtr<super::super::Foundation::RECT>,
        dwStyle: WINDOW_STYLE,
        bMenu: super::super::Foundation::BOOL,
        dwExStyle: WINDOW_EX_STYLE,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn AllowSetForegroundWindow(dwProcessId: u32) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn AnimateWindow(
        hWnd: super::super::Foundation::HWND,
        dwTime: u32,
        dwFlags: ANIMATE_WINDOW_FLAGS,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn AnyPopup() -> super::super::Foundation::BOOL {
        todo!()
    }
    fn AppendMenuA(
        hMenu: HMENU,
        uFlags: MENU_ITEM_FLAGS,
        uIDNewItem: usize,
        lpNewItem: ::win32::core::PCSTR,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn AppendMenuW(
        hMenu: HMENU,
        uFlags: MENU_ITEM_FLAGS,
        uIDNewItem: usize,
        lpNewItem: ::win32::core::PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn ArrangeIconicWindows(hWnd: super::super::Foundation::HWND) -> u32 {
        todo!()
    }
    fn BeginDeferWindowPos(nNumWindows: i32) -> isize {
        todo!()
    }
    fn BringWindowToTop(hWnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn CalculatePopupWindowPosition(
        anchorPoint: ConstPtr<super::super::Foundation::POINT>,
        windowSize: ConstPtr<super::super::Foundation::SIZE>,
        flags: u32,
        excludeRect: ConstPtr<super::super::Foundation::RECT>,
        popupWindowPosition: MutPtr<super::super::Foundation::RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn CallMsgFilterA(lpMsg: ConstPtr<MSG>, nCode: i32) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn CallMsgFilterW(lpMsg: ConstPtr<MSG>, nCode: i32) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn CallNextHookEx(
        hhk: HHOOK,
        nCode: i32,
        wParam: super::super::Foundation::WPARAM,
        lParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::LRESULT {
        todo!()
    }
    fn CallWindowProcA(
        lpPrevWndFunc: WNDPROC,
        hWnd: super::super::Foundation::HWND,
        Msg: u32,
        wParam: super::super::Foundation::WPARAM,
        lParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::LRESULT {
        todo!()
    }
    fn CallWindowProcW(
        lpPrevWndFunc: WNDPROC,
        hWnd: super::super::Foundation::HWND,
        Msg: u32,
        wParam: super::super::Foundation::WPARAM,
        lParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::LRESULT {
        todo!()
    }
    fn CancelShutdown() -> super::super::Foundation::BOOL {
        todo!()
    }
    fn CascadeWindows(
        hwndParent: super::super::Foundation::HWND,
        wHow: CASCADE_WINDOWS_HOW,
        lpRect: ConstPtr<super::super::Foundation::RECT>,
        cKids: u32,
        lpKids: ConstPtr<super::super::Foundation::HWND>,
    ) -> u16 {
        todo!()
    }
    fn ChangeMenuA(
        hMenu: HMENU,
        cmd: u32,
        lpszNewItem: ::win32::core::PCSTR,
        cmdInsert: u32,
        flags: u32,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn ChangeMenuW(
        hMenu: HMENU,
        cmd: u32,
        lpszNewItem: ::win32::core::PCWSTR,
        cmdInsert: u32,
        flags: u32,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn ChangeWindowMessageFilter(
        message: u32,
        dwFlag: CHANGE_WINDOW_MESSAGE_FILTER_FLAGS,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn ChangeWindowMessageFilterEx(
        hwnd: super::super::Foundation::HWND,
        message: u32,
        action: WINDOW_MESSAGE_FILTER_ACTION,
        pChangeFilterStruct: MutPtr<CHANGEFILTERSTRUCT>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn CharLowerA(lpsz: ::win32::core::PSTR) -> ::win32::core::PSTR {
        todo!()
    }
    fn CharLowerBuffA(lpsz: ::win32::core::PSTR, cchLength: u32) -> u32 {
        todo!()
    }
    fn CharLowerBuffW(lpsz: ::win32::core::PWSTR, cchLength: u32) -> u32 {
        todo!()
    }
    fn CharLowerW(lpsz: ::win32::core::PWSTR) -> ::win32::core::PWSTR {
        todo!()
    }
    fn CharNextA(lpsz: ::win32::core::PCSTR) -> ::win32::core::PSTR {
        todo!()
    }
    fn CharNextExA(
        CodePage: u16,
        lpCurrentChar: ::win32::core::PCSTR,
        dwFlags: u32,
    ) -> ::win32::core::PSTR {
        todo!()
    }
    fn CharNextW(lpsz: ::win32::core::PCWSTR) -> ::win32::core::PWSTR {
        todo!()
    }
    fn CharPrevA(
        lpszStart: ::win32::core::PCSTR,
        lpszCurrent: ::win32::core::PCSTR,
    ) -> ::win32::core::PSTR {
        todo!()
    }
    fn CharPrevExA(
        CodePage: u16,
        lpStart: ::win32::core::PCSTR,
        lpCurrentChar: ::win32::core::PCSTR,
        dwFlags: u32,
    ) -> ::win32::core::PSTR {
        todo!()
    }
    fn CharPrevW(
        lpszStart: ::win32::core::PCWSTR,
        lpszCurrent: ::win32::core::PCWSTR,
    ) -> ::win32::core::PWSTR {
        todo!()
    }
    fn CharToOemA(
        pSrc: ::win32::core::PCSTR,
        pDst: ::win32::core::PSTR,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn CharToOemBuffA(
        lpszSrc: ::win32::core::PCSTR,
        lpszDst: ::win32::core::PSTR,
        cchDstLength: u32,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn CharToOemBuffW(
        lpszSrc: ::win32::core::PCWSTR,
        lpszDst: ::win32::core::PSTR,
        cchDstLength: u32,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn CharToOemW(
        pSrc: ::win32::core::PCWSTR,
        pDst: ::win32::core::PSTR,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn CharUpperA(lpsz: ::win32::core::PSTR) -> ::win32::core::PSTR {
        todo!()
    }
    fn CharUpperBuffA(lpsz: ::win32::core::PSTR, cchLength: u32) -> u32 {
        todo!()
    }
    fn CharUpperBuffW(lpsz: ::win32::core::PWSTR, cchLength: u32) -> u32 {
        todo!()
    }
    fn CharUpperW(lpsz: ::win32::core::PWSTR) -> ::win32::core::PWSTR {
        todo!()
    }
    fn CheckMenuItem(hMenu: HMENU, uIDCheckItem: u32, uCheck: u32) -> u32 {
        todo!()
    }
    fn CheckMenuRadioItem(
        hmenu: HMENU,
        first: u32,
        last: u32,
        check: u32,
        flags: u32,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn ChildWindowFromPoint(
        hWndParent: super::super::Foundation::HWND,
        Point: super::super::Foundation::POINT,
    ) -> super::super::Foundation::HWND {
        todo!()
    }
    fn ChildWindowFromPointEx(
        hwnd: super::super::Foundation::HWND,
        pt: super::super::Foundation::POINT,
        flags: CWP_FLAGS,
    ) -> super::super::Foundation::HWND {
        todo!()
    }
    fn ClipCursor(
        lpRect: ConstPtr<super::super::Foundation::RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn CloseWindow(hWnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn CopyAcceleratorTableA(
        hAccelSrc: HACCEL,
        lpAccelDst: MutPtr<ACCEL>,
        cAccelEntries: i32,
    ) -> i32 {
        todo!()
    }
    fn CopyAcceleratorTableW(
        hAccelSrc: HACCEL,
        lpAccelDst: MutPtr<ACCEL>,
        cAccelEntries: i32,
    ) -> i32 {
        todo!()
    }
    fn CopyIcon(hIcon: HICON) -> HICON {
        todo!()
    }
    fn CopyImage(
        h: super::super::Foundation::HANDLE,
        r#type: GDI_IMAGE_TYPE,
        cx: i32,
        cy: i32,
        flags: IMAGE_FLAGS,
    ) -> super::super::Foundation::HANDLE {
        todo!()
    }
    fn CreateAcceleratorTableA(paccel: ConstPtr<ACCEL>, cAccel: i32) -> HACCEL {
        todo!()
    }
    fn CreateAcceleratorTableW(paccel: ConstPtr<ACCEL>, cAccel: i32) -> HACCEL {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateCaret(
        hWnd: super::super::Foundation::HWND,
        hBitmap: super::super::Graphics::Gdi::HBITMAP,
        nWidth: i32,
        nHeight: i32,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn CreateCursor(
        hInst: super::super::Foundation::HINSTANCE,
        xHotSpot: i32,
        yHotSpot: i32,
        nWidth: i32,
        nHeight: i32,
        pvANDPlane: ConstPtr<::core::ffi::c_void>,
        pvXORPlane: ConstPtr<::core::ffi::c_void>,
    ) -> HCURSOR {
        todo!()
    }
    fn CreateDialogIndirectParamA(
        hInstance: super::super::Foundation::HINSTANCE,
        lpTemplate: ConstPtr<DLGTEMPLATE>,
        hWndParent: super::super::Foundation::HWND,
        lpDialogFunc: DLGPROC,
        dwInitParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::HWND {
        todo!()
    }
    fn CreateDialogIndirectParamW(
        hInstance: super::super::Foundation::HINSTANCE,
        lpTemplate: ConstPtr<DLGTEMPLATE>,
        hWndParent: super::super::Foundation::HWND,
        lpDialogFunc: DLGPROC,
        dwInitParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::HWND {
        todo!()
    }
    fn CreateDialogParamA(
        hInstance: super::super::Foundation::HINSTANCE,
        lpTemplateName: ::win32::core::PCSTR,
        hWndParent: super::super::Foundation::HWND,
        lpDialogFunc: DLGPROC,
        dwInitParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::HWND {
        todo!()
    }
    fn CreateDialogParamW(
        hInstance: super::super::Foundation::HINSTANCE,
        lpTemplateName: ::win32::core::PCWSTR,
        hWndParent: super::super::Foundation::HWND,
        lpDialogFunc: DLGPROC,
        dwInitParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::HWND {
        todo!()
    }
    fn CreateIcon(
        hInstance: super::super::Foundation::HINSTANCE,
        nWidth: i32,
        nHeight: i32,
        cPlanes: u8,
        cBitsPixel: u8,
        lpbANDbits: ConstPtr<u8>,
        lpbXORbits: ConstPtr<u8>,
    ) -> HICON {
        todo!()
    }
    fn CreateIconFromResource(
        presbits: ConstPtr<u8>,
        dwResSize: u32,
        fIcon: super::super::Foundation::BOOL,
        dwVer: u32,
    ) -> HICON {
        todo!()
    }
    fn CreateIconFromResourceEx(
        presbits: ConstPtr<u8>,
        dwResSize: u32,
        fIcon: super::super::Foundation::BOOL,
        dwVer: u32,
        cxDesired: i32,
        cyDesired: i32,
        Flags: IMAGE_FLAGS,
    ) -> HICON {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn CreateIconIndirect(piconinfo: ConstPtr<ICONINFO>) -> HICON {
        todo!()
    }
    fn CreateMDIWindowA(
        lpClassName: ::win32::core::PCSTR,
        lpWindowName: ::win32::core::PCSTR,
        dwStyle: WINDOW_STYLE,
        X: i32,
        Y: i32,
        nWidth: i32,
        nHeight: i32,
        hWndParent: super::super::Foundation::HWND,
        hInstance: super::super::Foundation::HINSTANCE,
        lParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::HWND {
        todo!()
    }
    fn CreateMDIWindowW(
        lpClassName: ::win32::core::PCWSTR,
        lpWindowName: ::win32::core::PCWSTR,
        dwStyle: WINDOW_STYLE,
        X: i32,
        Y: i32,
        nWidth: i32,
        nHeight: i32,
        hWndParent: super::super::Foundation::HWND,
        hInstance: super::super::Foundation::HINSTANCE,
        lParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::HWND {
        todo!()
    }
    fn CreateMenu() -> HMENU {
        todo!()
    }
    fn CreatePopupMenu() -> HMENU {
        todo!()
    }
    fn CreateResourceIndexer(
        projectRoot: ::win32::core::PCWSTR,
        extensionDllPath: ::win32::core::PCWSTR,
        ppResourceIndexer: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> ::win32::core::HRESULT {
        todo!()
    }
    fn CreateWindowExA(
        dwExStyle: WINDOW_EX_STYLE,
        lpClassName: ::win32::core::PCSTR,
        lpWindowName: ::win32::core::PCSTR,
        dwStyle: WINDOW_STYLE,
        X: i32,
        Y: i32,
        nWidth: i32,
        nHeight: i32,
        hWndParent: super::super::Foundation::HWND,
        hMenu: HMENU,
        hInstance: super::super::Foundation::HINSTANCE,
        lpParam: ConstPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::HWND {
        todo!()
    }
    fn CreateWindowExW(
        dwExStyle: WINDOW_EX_STYLE,
        lpClassName: ::win32::core::PCWSTR,
        lpWindowName: ::win32::core::PCWSTR,
        dwStyle: WINDOW_STYLE,
        X: i32,
        Y: i32,
        nWidth: i32,
        nHeight: i32,
        hWndParent: super::super::Foundation::HWND,
        hMenu: HMENU,
        hInstance: super::super::Foundation::HINSTANCE,
        lpParam: ConstPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::HWND {
        todo!()
    }
    fn DefDlgProcA(
        hDlg: super::super::Foundation::HWND,
        Msg: u32,
        wParam: super::super::Foundation::WPARAM,
        lParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::LRESULT {
        todo!()
    }
    fn DefDlgProcW(
        hDlg: super::super::Foundation::HWND,
        Msg: u32,
        wParam: super::super::Foundation::WPARAM,
        lParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::LRESULT {
        todo!()
    }
    fn DefFrameProcA(
        hWnd: super::super::Foundation::HWND,
        hWndMDIClient: super::super::Foundation::HWND,
        uMsg: u32,
        wParam: super::super::Foundation::WPARAM,
        lParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::LRESULT {
        todo!()
    }
    fn DefFrameProcW(
        hWnd: super::super::Foundation::HWND,
        hWndMDIClient: super::super::Foundation::HWND,
        uMsg: u32,
        wParam: super::super::Foundation::WPARAM,
        lParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::LRESULT {
        todo!()
    }
    fn DefMDIChildProcA(
        hWnd: super::super::Foundation::HWND,
        uMsg: u32,
        wParam: super::super::Foundation::WPARAM,
        lParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::LRESULT {
        todo!()
    }
    fn DefMDIChildProcW(
        hWnd: super::super::Foundation::HWND,
        uMsg: u32,
        wParam: super::super::Foundation::WPARAM,
        lParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::LRESULT {
        todo!()
    }
    fn DefWindowProcA(
        hWnd: super::super::Foundation::HWND,
        Msg: u32,
        wParam: super::super::Foundation::WPARAM,
        lParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::LRESULT {
        todo!()
    }
    fn DefWindowProcW(
        hWnd: super::super::Foundation::HWND,
        Msg: u32,
        wParam: super::super::Foundation::WPARAM,
        lParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::LRESULT {
        todo!()
    }
    fn DeferWindowPos(
        hWinPosInfo: isize,
        hWnd: super::super::Foundation::HWND,
        hWndInsertAfter: super::super::Foundation::HWND,
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        uFlags: SET_WINDOW_POS_FLAGS,
    ) -> isize {
        todo!()
    }
    fn DeleteMenu(
        hMenu: HMENU,
        uPosition: u32,
        uFlags: MENU_ITEM_FLAGS,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn DeregisterShellHookWindow(
        hwnd: super::super::Foundation::HWND,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn DestroyAcceleratorTable(hAccel: HACCEL) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn DestroyCaret() -> super::super::Foundation::BOOL {
        todo!()
    }
    fn DestroyCursor(hCursor: HCURSOR) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn DestroyIcon(hIcon: HICON) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn DestroyIndexedResults(
        resourceUri: ::win32::core::PCWSTR,
        qualifierCount: u32,
        qualifiers: ConstPtr<IndexedResourceQualifier>,
    ) {
        todo!()
    }
    fn DestroyMenu(hMenu: HMENU) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn DestroyResourceIndexer(resourceIndexer: ConstPtr<::core::ffi::c_void>) {
        todo!()
    }
    fn DestroyWindow(hWnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn DialogBoxIndirectParamA(
        hInstance: super::super::Foundation::HINSTANCE,
        hDialogTemplate: ConstPtr<DLGTEMPLATE>,
        hWndParent: super::super::Foundation::HWND,
        lpDialogFunc: DLGPROC,
        dwInitParam: super::super::Foundation::LPARAM,
    ) -> isize {
        todo!()
    }
    fn DialogBoxIndirectParamW(
        hInstance: super::super::Foundation::HINSTANCE,
        hDialogTemplate: ConstPtr<DLGTEMPLATE>,
        hWndParent: super::super::Foundation::HWND,
        lpDialogFunc: DLGPROC,
        dwInitParam: super::super::Foundation::LPARAM,
    ) -> isize {
        todo!()
    }
    fn DialogBoxParamA(
        hInstance: super::super::Foundation::HINSTANCE,
        lpTemplateName: ::win32::core::PCSTR,
        hWndParent: super::super::Foundation::HWND,
        lpDialogFunc: DLGPROC,
        dwInitParam: super::super::Foundation::LPARAM,
    ) -> isize {
        todo!()
    }
    fn DialogBoxParamW(
        hInstance: super::super::Foundation::HINSTANCE,
        lpTemplateName: ::win32::core::PCWSTR,
        hWndParent: super::super::Foundation::HWND,
        lpDialogFunc: DLGPROC,
        dwInitParam: super::super::Foundation::LPARAM,
    ) -> isize {
        todo!()
    }
    fn DisableProcessWindowsGhosting() {
        todo!()
    }
    fn DispatchMessageA(lpMsg: ConstPtr<MSG>) -> super::super::Foundation::LRESULT {
        todo!()
    }
    fn DispatchMessageW(lpMsg: ConstPtr<MSG>) -> super::super::Foundation::LRESULT {
        todo!()
    }
    fn DragObject(
        hwndParent: super::super::Foundation::HWND,
        hwndFrom: super::super::Foundation::HWND,
        fmt: u32,
        data: usize,
        hcur: HCURSOR,
    ) -> u32 {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn DrawIcon(
        hDC: super::super::Graphics::Gdi::HDC,
        X: i32,
        Y: i32,
        hIcon: HICON,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn DrawIconEx(
        hdc: super::super::Graphics::Gdi::HDC,
        xLeft: i32,
        yTop: i32,
        hIcon: HICON,
        cxWidth: i32,
        cyWidth: i32,
        istepIfAniCur: u32,
        hbrFlickerFreeDraw: super::super::Graphics::Gdi::HBRUSH,
        diFlags: DI_FLAGS,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn DrawMenuBar(hWnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn EnableMenuItem(
        hMenu: HMENU,
        uIDEnableItem: u32,
        uEnable: MENU_ITEM_FLAGS,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn EndDeferWindowPos(hWinPosInfo: isize) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn EndDialog(
        hDlg: super::super::Foundation::HWND,
        nResult: isize,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn EndMenu() -> super::super::Foundation::BOOL {
        todo!()
    }
    fn EnumChildWindows(
        hWndParent: super::super::Foundation::HWND,
        lpEnumFunc: WNDENUMPROC,
        lParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn EnumPropsA(hWnd: super::super::Foundation::HWND, lpEnumFunc: PROPENUMPROCA) -> i32 {
        todo!()
    }
    fn EnumPropsExA(
        hWnd: super::super::Foundation::HWND,
        lpEnumFunc: PROPENUMPROCEXA,
        lParam: super::super::Foundation::LPARAM,
    ) -> i32 {
        todo!()
    }
    fn EnumPropsExW(
        hWnd: super::super::Foundation::HWND,
        lpEnumFunc: PROPENUMPROCEXW,
        lParam: super::super::Foundation::LPARAM,
    ) -> i32 {
        todo!()
    }
    fn EnumPropsW(hWnd: super::super::Foundation::HWND, lpEnumFunc: PROPENUMPROCW) -> i32 {
        todo!()
    }
    fn EnumThreadWindows(
        dwThreadId: u32,
        lpfn: WNDENUMPROC,
        lParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn EnumWindows(
        lpEnumFunc: WNDENUMPROC,
        lParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn FindWindowA(
        lpClassName: ::win32::core::PCSTR,
        lpWindowName: ::win32::core::PCSTR,
    ) -> super::super::Foundation::HWND {
        todo!()
    }
    fn FindWindowExA(
        hWndParent: super::super::Foundation::HWND,
        hWndChildAfter: super::super::Foundation::HWND,
        lpszClass: ::win32::core::PCSTR,
        lpszWindow: ::win32::core::PCSTR,
    ) -> super::super::Foundation::HWND {
        todo!()
    }
    fn FindWindowExW(
        hWndParent: super::super::Foundation::HWND,
        hWndChildAfter: super::super::Foundation::HWND,
        lpszClass: ::win32::core::PCWSTR,
        lpszWindow: ::win32::core::PCWSTR,
    ) -> super::super::Foundation::HWND {
        todo!()
    }
    fn FindWindowW(
        lpClassName: ::win32::core::PCWSTR,
        lpWindowName: ::win32::core::PCWSTR,
    ) -> super::super::Foundation::HWND {
        todo!()
    }
    fn FlashWindow(
        hWnd: super::super::Foundation::HWND,
        bInvert: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn FlashWindowEx(pfwi: ConstPtr<FLASHWINFO>) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetAltTabInfoA(
        hwnd: super::super::Foundation::HWND,
        iItem: i32,
        pati: MutPtr<ALTTABINFO>,
        pszItemText: ::win32::core::PSTR,
        cchItemText: u32,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetAltTabInfoW(
        hwnd: super::super::Foundation::HWND,
        iItem: i32,
        pati: MutPtr<ALTTABINFO>,
        pszItemText: ::win32::core::PWSTR,
        cchItemText: u32,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetAncestor(
        hwnd: super::super::Foundation::HWND,
        gaFlags: GET_ANCESTOR_FLAGS,
    ) -> super::super::Foundation::HWND {
        todo!()
    }
    fn GetCaretBlinkTime() -> u32 {
        todo!()
    }
    fn GetCaretPos(
        lpPoint: MutPtr<super::super::Foundation::POINT>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetClassInfoA(
        hInstance: super::super::Foundation::HINSTANCE,
        lpClassName: ::win32::core::PCSTR,
        lpWndClass: MutPtr<WNDCLASSA>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetClassInfoExA(
        hInstance: super::super::Foundation::HINSTANCE,
        lpszClass: ::win32::core::PCSTR,
        lpwcx: MutPtr<WNDCLASSEXA>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetClassInfoExW(
        hInstance: super::super::Foundation::HINSTANCE,
        lpszClass: ::win32::core::PCWSTR,
        lpwcx: MutPtr<WNDCLASSEXW>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetClassInfoW(
        hInstance: super::super::Foundation::HINSTANCE,
        lpClassName: ::win32::core::PCWSTR,
        lpWndClass: MutPtr<WNDCLASSW>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetClassLongA(hWnd: super::super::Foundation::HWND, nIndex: GET_CLASS_LONG_INDEX) -> u32 {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetClassLongPtrA(
        hWnd: super::super::Foundation::HWND,
        nIndex: GET_CLASS_LONG_INDEX,
    ) -> usize {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetClassLongPtrW(
        hWnd: super::super::Foundation::HWND,
        nIndex: GET_CLASS_LONG_INDEX,
    ) -> usize {
        todo!()
    }
    fn GetClassLongW(hWnd: super::super::Foundation::HWND, nIndex: GET_CLASS_LONG_INDEX) -> u32 {
        todo!()
    }
    fn GetClassNameA(
        hWnd: super::super::Foundation::HWND,
        lpClassName: ::win32::core::PSTR,
        nMaxCount: i32,
    ) -> i32 {
        todo!()
    }
    fn GetClassNameW(
        hWnd: super::super::Foundation::HWND,
        lpClassName: ::win32::core::PWSTR,
        nMaxCount: i32,
    ) -> i32 {
        todo!()
    }
    fn GetClassWord(hWnd: super::super::Foundation::HWND, nIndex: i32) -> u16 {
        todo!()
    }
    fn GetClientRect(
        hWnd: super::super::Foundation::HWND,
        lpRect: MutPtr<super::super::Foundation::RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetClipCursor(
        lpRect: MutPtr<super::super::Foundation::RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetCursor() -> HCURSOR {
        todo!()
    }
    fn GetCursorInfo(pci: MutPtr<CURSORINFO>) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetCursorPos(
        lpPoint: MutPtr<super::super::Foundation::POINT>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetDesktopWindow() -> super::super::Foundation::HWND {
        todo!()
    }
    fn GetDialogBaseUnits() -> i32 {
        todo!()
    }
    fn GetDlgCtrlID(hWnd: super::super::Foundation::HWND) -> i32 {
        todo!()
    }
    fn GetDlgItem(
        hDlg: super::super::Foundation::HWND,
        nIDDlgItem: i32,
    ) -> super::super::Foundation::HWND {
        todo!()
    }
    fn GetDlgItemInt(
        hDlg: super::super::Foundation::HWND,
        nIDDlgItem: i32,
        lpTranslated: MutPtr<super::super::Foundation::BOOL>,
        bSigned: super::super::Foundation::BOOL,
    ) -> u32 {
        todo!()
    }
    fn GetDlgItemTextA(
        hDlg: super::super::Foundation::HWND,
        nIDDlgItem: i32,
        lpString: ::win32::core::PSTR,
        cchMax: i32,
    ) -> u32 {
        todo!()
    }
    fn GetDlgItemTextW(
        hDlg: super::super::Foundation::HWND,
        nIDDlgItem: i32,
        lpString: ::win32::core::PWSTR,
        cchMax: i32,
    ) -> u32 {
        todo!()
    }
    fn GetForegroundWindow() -> super::super::Foundation::HWND {
        todo!()
    }
    fn GetGUIThreadInfo(
        idThread: u32,
        pgui: MutPtr<GUITHREADINFO>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetIconInfo(hIcon: HICON, piconinfo: MutPtr<ICONINFO>) -> super::super::Foundation::BOOL {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetIconInfoExA(
        hicon: HICON,
        piconinfo: MutPtr<ICONINFOEXA>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetIconInfoExW(
        hicon: HICON,
        piconinfo: MutPtr<ICONINFOEXW>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetInputState() -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetLastActivePopup(hWnd: super::super::Foundation::HWND) -> super::super::Foundation::HWND {
        todo!()
    }
    fn GetLayeredWindowAttributes(
        hwnd: super::super::Foundation::HWND,
        pcrKey: MutPtr<u32>,
        pbAlpha: MutPtr<u8>,
        pdwFlags: MutPtr<LAYERED_WINDOW_ATTRIBUTES_FLAGS>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetMenu(hWnd: super::super::Foundation::HWND) -> HMENU {
        todo!()
    }
    fn GetMenuBarInfo(
        hwnd: super::super::Foundation::HWND,
        idObject: OBJECT_IDENTIFIER,
        idItem: i32,
        pmbi: MutPtr<MENUBARINFO>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetMenuCheckMarkDimensions() -> i32 {
        todo!()
    }
    fn GetMenuDefaultItem(
        hMenu: HMENU,
        fByPos: u32,
        gmdiFlags: GET_MENU_DEFAULT_ITEM_FLAGS,
    ) -> u32 {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetMenuInfo(param0: HMENU, param1: MutPtr<MENUINFO>) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetMenuItemCount(hMenu: HMENU) -> i32 {
        todo!()
    }
    fn GetMenuItemID(hMenu: HMENU, nPos: i32) -> u32 {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetMenuItemInfoA(
        hmenu: HMENU,
        item: u32,
        fByPosition: super::super::Foundation::BOOL,
        lpmii: MutPtr<MENUITEMINFOA>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetMenuItemInfoW(
        hmenu: HMENU,
        item: u32,
        fByPosition: super::super::Foundation::BOOL,
        lpmii: MutPtr<MENUITEMINFOW>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetMenuItemRect(
        hWnd: super::super::Foundation::HWND,
        hMenu: HMENU,
        uItem: u32,
        lprcItem: MutPtr<super::super::Foundation::RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetMenuState(hMenu: HMENU, uId: u32, uFlags: MENU_ITEM_FLAGS) -> u32 {
        todo!()
    }
    fn GetMenuStringA(
        hMenu: HMENU,
        uIDItem: u32,
        lpString: ::win32::core::PSTR,
        cchMax: i32,
        flags: MENU_ITEM_FLAGS,
    ) -> i32 {
        todo!()
    }
    fn GetMenuStringW(
        hMenu: HMENU,
        uIDItem: u32,
        lpString: ::win32::core::PWSTR,
        cchMax: i32,
        flags: MENU_ITEM_FLAGS,
    ) -> i32 {
        todo!()
    }
    fn GetMessageA(
        lpMsg: MutPtr<MSG>,
        hWnd: super::super::Foundation::HWND,
        wMsgFilterMin: u32,
        wMsgFilterMax: u32,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetMessageExtraInfo() -> super::super::Foundation::LPARAM {
        todo!()
    }
    fn GetMessagePos() -> u32 {
        todo!()
    }
    fn GetMessageTime() -> i32 {
        todo!()
    }
    fn GetMessageW(
        lpMsg: MutPtr<MSG>,
        hWnd: super::super::Foundation::HWND,
        wMsgFilterMin: u32,
        wMsgFilterMax: u32,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetNextDlgGroupItem(
        hDlg: super::super::Foundation::HWND,
        hCtl: super::super::Foundation::HWND,
        bPrevious: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::HWND {
        todo!()
    }
    fn GetNextDlgTabItem(
        hDlg: super::super::Foundation::HWND,
        hCtl: super::super::Foundation::HWND,
        bPrevious: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::HWND {
        todo!()
    }
    fn GetParent(hWnd: super::super::Foundation::HWND) -> super::super::Foundation::HWND {
        todo!()
    }
    fn GetPhysicalCursorPos(
        lpPoint: MutPtr<super::super::Foundation::POINT>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetProcessDefaultLayout(pdwDefaultLayout: MutPtr<u32>) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetPropA(
        hWnd: super::super::Foundation::HWND,
        lpString: ::win32::core::PCSTR,
    ) -> super::super::Foundation::HANDLE {
        todo!()
    }
    fn GetPropW(
        hWnd: super::super::Foundation::HWND,
        lpString: ::win32::core::PCWSTR,
    ) -> super::super::Foundation::HANDLE {
        todo!()
    }
    fn GetQueueStatus(flags: QUEUE_STATUS_FLAGS) -> u32 {
        todo!()
    }
    fn GetScrollBarInfo(
        hwnd: super::super::Foundation::HWND,
        idObject: OBJECT_IDENTIFIER,
        psbi: MutPtr<SCROLLBARINFO>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetScrollInfo(
        hwnd: super::super::Foundation::HWND,
        nBar: SCROLLBAR_CONSTANTS,
        lpsi: MutPtr<SCROLLINFO>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetScrollPos(hWnd: super::super::Foundation::HWND, nBar: SCROLLBAR_CONSTANTS) -> i32 {
        todo!()
    }
    fn GetScrollRange(
        hWnd: super::super::Foundation::HWND,
        nBar: SCROLLBAR_CONSTANTS,
        lpMinPos: MutPtr<i32>,
        lpMaxPos: MutPtr<i32>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetShellWindow() -> super::super::Foundation::HWND {
        todo!()
    }
    fn GetSubMenu(hMenu: HMENU, nPos: i32) -> HMENU {
        todo!()
    }
    fn GetSysColor(nIndex: SYS_COLOR_INDEX) -> u32 {
        todo!()
    }
    fn GetSystemMenu(
        hWnd: super::super::Foundation::HWND,
        bRevert: super::super::Foundation::BOOL,
    ) -> HMENU {
        todo!()
    }
    fn GetSystemMetrics(nIndex: SYSTEM_METRICS_INDEX) -> i32 {
        todo!()
    }
    fn GetTitleBarInfo(
        hwnd: super::super::Foundation::HWND,
        pti: MutPtr<TITLEBARINFO>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetTopWindow(hWnd: super::super::Foundation::HWND) -> super::super::Foundation::HWND {
        todo!()
    }
    fn GetWindow(
        hWnd: super::super::Foundation::HWND,
        uCmd: GET_WINDOW_CMD,
    ) -> super::super::Foundation::HWND {
        todo!()
    }
    fn GetWindowDisplayAffinity(
        hWnd: super::super::Foundation::HWND,
        pdwAffinity: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetWindowInfo(
        hwnd: super::super::Foundation::HWND,
        pwi: MutPtr<WINDOWINFO>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetWindowLongA(hWnd: super::super::Foundation::HWND, nIndex: WINDOW_LONG_PTR_INDEX) -> i32 {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetWindowLongPtrA(
        hWnd: super::super::Foundation::HWND,
        nIndex: WINDOW_LONG_PTR_INDEX,
    ) -> isize {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn GetWindowLongPtrW(
        hWnd: super::super::Foundation::HWND,
        nIndex: WINDOW_LONG_PTR_INDEX,
    ) -> isize {
        todo!()
    }
    fn GetWindowLongW(hWnd: super::super::Foundation::HWND, nIndex: WINDOW_LONG_PTR_INDEX) -> i32 {
        todo!()
    }
    fn GetWindowModuleFileNameA(
        hwnd: super::super::Foundation::HWND,
        pszFileName: ::win32::core::PSTR,
        cchFileNameMax: u32,
    ) -> u32 {
        todo!()
    }
    fn GetWindowModuleFileNameW(
        hwnd: super::super::Foundation::HWND,
        pszFileName: ::win32::core::PWSTR,
        cchFileNameMax: u32,
    ) -> u32 {
        todo!()
    }
    fn GetWindowPlacement(
        hWnd: super::super::Foundation::HWND,
        lpwndpl: MutPtr<WINDOWPLACEMENT>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetWindowRect(
        hWnd: super::super::Foundation::HWND,
        lpRect: MutPtr<super::super::Foundation::RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetWindowTextA(
        hWnd: super::super::Foundation::HWND,
        lpString: ::win32::core::PSTR,
        nMaxCount: i32,
    ) -> i32 {
        todo!()
    }
    fn GetWindowTextLengthA(hWnd: super::super::Foundation::HWND) -> i32 {
        todo!()
    }
    fn GetWindowTextLengthW(hWnd: super::super::Foundation::HWND) -> i32 {
        todo!()
    }
    fn GetWindowTextW(
        hWnd: super::super::Foundation::HWND,
        lpString: ::win32::core::PWSTR,
        nMaxCount: i32,
    ) -> i32 {
        todo!()
    }
    fn GetWindowThreadProcessId(
        hWnd: super::super::Foundation::HWND,
        lpdwProcessId: MutPtr<u32>,
    ) -> u32 {
        todo!()
    }
    fn GetWindowWord(hWnd: super::super::Foundation::HWND, nIndex: i32) -> u16 {
        todo!()
    }
    fn HideCaret(hWnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn HiliteMenuItem(
        hWnd: super::super::Foundation::HWND,
        hMenu: HMENU,
        uIDHiliteItem: u32,
        uHilite: u32,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn InSendMessage() -> super::super::Foundation::BOOL {
        todo!()
    }
    fn InSendMessageEx(lpReserved: MutPtr<::core::ffi::c_void>) -> u32 {
        todo!()
    }
    fn IndexFilePath(
        resourceIndexer: ConstPtr<::core::ffi::c_void>,
        filePath: ::win32::core::PCWSTR,
        ppResourceUri: MutPtr<::win32::core::PWSTR>,
        pQualifierCount: MutPtr<u32>,
        ppQualifiers: MutPtr<ConstPtr<IndexedResourceQualifier>>,
    ) -> ::win32::core::HRESULT {
        todo!()
    }
    fn InheritWindowMonitor(
        hwnd: super::super::Foundation::HWND,
        hwndInherit: super::super::Foundation::HWND,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn InsertMenuA(
        hMenu: HMENU,
        uPosition: u32,
        uFlags: MENU_ITEM_FLAGS,
        uIDNewItem: usize,
        lpNewItem: ::win32::core::PCSTR,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn InsertMenuItemA(
        hmenu: HMENU,
        item: u32,
        fByPosition: super::super::Foundation::BOOL,
        lpmi: ConstPtr<MENUITEMINFOA>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn InsertMenuItemW(
        hmenu: HMENU,
        item: u32,
        fByPosition: super::super::Foundation::BOOL,
        lpmi: ConstPtr<MENUITEMINFOW>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn InsertMenuW(
        hMenu: HMENU,
        uPosition: u32,
        uFlags: MENU_ITEM_FLAGS,
        uIDNewItem: usize,
        lpNewItem: ::win32::core::PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn InternalGetWindowText(
        hWnd: super::super::Foundation::HWND,
        pString: ::win32::core::PWSTR,
        cchMaxCount: i32,
    ) -> i32 {
        todo!()
    }
    fn IsCharAlphaA(ch: super::super::Foundation::CHAR) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsCharAlphaNumericA(ch: super::super::Foundation::CHAR) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsCharAlphaNumericW(ch: u16) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsCharAlphaW(ch: u16) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsCharLowerA(ch: super::super::Foundation::CHAR) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsCharUpperA(ch: super::super::Foundation::CHAR) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsCharUpperW(ch: u16) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsChild(
        hWndParent: super::super::Foundation::HWND,
        hWnd: super::super::Foundation::HWND,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsDialogMessageA(
        hDlg: super::super::Foundation::HWND,
        lpMsg: ConstPtr<MSG>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsDialogMessageW(
        hDlg: super::super::Foundation::HWND,
        lpMsg: ConstPtr<MSG>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsGUIThread(bConvert: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsHungAppWindow(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsIconic(hWnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsMenu(hMenu: HMENU) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsProcessDPIAware() -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsWindow(hWnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsWindowUnicode(hWnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsWindowVisible(hWnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsWow64Message() -> super::super::Foundation::BOOL {
        todo!()
    }
    fn IsZoomed(hWnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn KillTimer(
        hWnd: super::super::Foundation::HWND,
        uIDEvent: usize,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn LoadAcceleratorsA(
        hInstance: super::super::Foundation::HINSTANCE,
        lpTableName: ::win32::core::PCSTR,
    ) -> HACCEL {
        todo!()
    }
    fn LoadAcceleratorsW(
        hInstance: super::super::Foundation::HINSTANCE,
        lpTableName: ::win32::core::PCWSTR,
    ) -> HACCEL {
        todo!()
    }
    fn LoadCursorA(
        hInstance: super::super::Foundation::HINSTANCE,
        lpCursorName: ::win32::core::PCSTR,
    ) -> HCURSOR {
        todo!()
    }
    fn LoadCursorFromFileA(lpFileName: ::win32::core::PCSTR) -> HCURSOR {
        todo!()
    }
    fn LoadCursorFromFileW(lpFileName: ::win32::core::PCWSTR) -> HCURSOR {
        todo!()
    }
    fn LoadCursorW(
        hInstance: super::super::Foundation::HINSTANCE,
        lpCursorName: ::win32::core::PCWSTR,
    ) -> HCURSOR {
        todo!()
    }
    fn LoadIconA(
        hInstance: super::super::Foundation::HINSTANCE,
        lpIconName: ::win32::core::PCSTR,
    ) -> HICON {
        todo!()
    }
    fn LoadIconW(
        hInstance: super::super::Foundation::HINSTANCE,
        lpIconName: ::win32::core::PCWSTR,
    ) -> HICON {
        todo!()
    }
    fn LoadImageA(
        hInst: super::super::Foundation::HINSTANCE,
        name: ::win32::core::PCSTR,
        r#type: GDI_IMAGE_TYPE,
        cx: i32,
        cy: i32,
        fuLoad: IMAGE_FLAGS,
    ) -> super::super::Foundation::HANDLE {
        todo!()
    }
    fn LoadImageW(
        hInst: super::super::Foundation::HINSTANCE,
        name: ::win32::core::PCWSTR,
        r#type: GDI_IMAGE_TYPE,
        cx: i32,
        cy: i32,
        fuLoad: IMAGE_FLAGS,
    ) -> super::super::Foundation::HANDLE {
        todo!()
    }
    fn LoadMenuA(
        hInstance: super::super::Foundation::HINSTANCE,
        lpMenuName: ::win32::core::PCSTR,
    ) -> HMENU {
        todo!()
    }
    fn LoadMenuIndirectA(lpMenuTemplate: ConstPtr<::core::ffi::c_void>) -> HMENU {
        todo!()
    }
    fn LoadMenuIndirectW(lpMenuTemplate: ConstPtr<::core::ffi::c_void>) -> HMENU {
        todo!()
    }
    fn LoadMenuW(
        hInstance: super::super::Foundation::HINSTANCE,
        lpMenuName: ::win32::core::PCWSTR,
    ) -> HMENU {
        todo!()
    }
    fn LoadStringA(
        hInstance: super::super::Foundation::HINSTANCE,
        uID: u32,
        lpBuffer: ::win32::core::PSTR,
        cchBufferMax: i32,
    ) -> i32 {
        todo!()
    }
    fn LoadStringW(
        hInstance: super::super::Foundation::HINSTANCE,
        uID: u32,
        lpBuffer: ::win32::core::PWSTR,
        cchBufferMax: i32,
    ) -> i32 {
        todo!()
    }
    fn LockSetForegroundWindow(
        uLockCode: FOREGROUND_WINDOW_LOCK_CODE,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn LogicalToPhysicalPoint(
        hWnd: super::super::Foundation::HWND,
        lpPoint: MutPtr<super::super::Foundation::POINT>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn LookupIconIdFromDirectory(
        presbits: ConstPtr<u8>,
        fIcon: super::super::Foundation::BOOL,
    ) -> i32 {
        todo!()
    }
    fn LookupIconIdFromDirectoryEx(
        presbits: ConstPtr<u8>,
        fIcon: super::super::Foundation::BOOL,
        cxDesired: i32,
        cyDesired: i32,
        Flags: IMAGE_FLAGS,
    ) -> i32 {
        todo!()
    }
    fn MapDialogRect(
        hDlg: super::super::Foundation::HWND,
        lpRect: MutPtr<super::super::Foundation::RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn MenuItemFromPoint(
        hWnd: super::super::Foundation::HWND,
        hMenu: HMENU,
        ptScreen: super::super::Foundation::POINT,
    ) -> i32 {
        todo!()
    }
    fn MessageBoxA(
        hWnd: super::super::Foundation::HWND,
        lpText: ::win32::core::PCSTR,
        lpCaption: ::win32::core::PCSTR,
        uType: MESSAGEBOX_STYLE,
    ) -> MESSAGEBOX_RESULT {
        todo!()
    }
    fn MessageBoxExA(
        hWnd: super::super::Foundation::HWND,
        lpText: ::win32::core::PCSTR,
        lpCaption: ::win32::core::PCSTR,
        uType: MESSAGEBOX_STYLE,
        wLanguageId: u16,
    ) -> MESSAGEBOX_RESULT {
        todo!()
    }
    fn MessageBoxExW(
        hWnd: super::super::Foundation::HWND,
        lpText: ::win32::core::PCWSTR,
        lpCaption: ::win32::core::PCWSTR,
        uType: MESSAGEBOX_STYLE,
        wLanguageId: u16,
    ) -> MESSAGEBOX_RESULT {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.UI.Shell'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn MessageBoxIndirectA(lpmbp: ConstPtr<MSGBOXPARAMSA>) -> MESSAGEBOX_RESULT {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.UI.Shell'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn MessageBoxIndirectW(lpmbp: ConstPtr<MSGBOXPARAMSW>) -> MESSAGEBOX_RESULT {
        todo!()
    }
    fn MessageBoxW(
        hWnd: super::super::Foundation::HWND,
        lpText: ::win32::core::PCWSTR,
        lpCaption: ::win32::core::PCWSTR,
        uType: MESSAGEBOX_STYLE,
    ) -> MESSAGEBOX_RESULT {
        todo!()
    }
    fn ModifyMenuA(
        hMnu: HMENU,
        uPosition: u32,
        uFlags: MENU_ITEM_FLAGS,
        uIDNewItem: usize,
        lpNewItem: ::win32::core::PCSTR,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn ModifyMenuW(
        hMnu: HMENU,
        uPosition: u32,
        uFlags: MENU_ITEM_FLAGS,
        uIDNewItem: usize,
        lpNewItem: ::win32::core::PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn MoveWindow(
        hWnd: super::super::Foundation::HWND,
        X: i32,
        Y: i32,
        nWidth: i32,
        nHeight: i32,
        bRepaint: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn MrmCreateConfig(
        platformVersion: MrmPlatformVersion,
        defaultQualifiers: ::win32::core::PCWSTR,
        outputXmlFile: ::win32::core::PCWSTR,
    ) -> ::win32::core::HRESULT {
        todo!()
    }
    fn MrmCreateConfigInMemory(
        platformVersion: MrmPlatformVersion,
        defaultQualifiers: ::win32::core::PCWSTR,
        outputXmlData: MutPtr<ConstPtr<u8>>,
        outputXmlSize: MutPtr<u32>,
    ) -> ::win32::core::HRESULT {
        todo!()
    }
    fn MrmCreateResourceFile(
        indexer: MrmResourceIndexerHandle,
        packagingMode: MrmPackagingMode,
        packagingOptions: MrmPackagingOptions,
        outputDirectory: ::win32::core::PCWSTR,
    ) -> ::win32::core::HRESULT {
        todo!()
    }
    fn MrmCreateResourceFileInMemory(
        indexer: MrmResourceIndexerHandle,
        packagingMode: MrmPackagingMode,
        packagingOptions: MrmPackagingOptions,
        outputPriData: MutPtr<ConstPtr<u8>>,
        outputPriSize: MutPtr<u32>,
    ) -> ::win32::core::HRESULT {
        todo!()
    }
    fn MrmCreateResourceFileWithChecksum(
        indexer: MrmResourceIndexerHandle,
        packagingMode: MrmPackagingMode,
        packagingOptions: MrmPackagingOptions,
        checksum: u32,
        outputDirectory: ::win32::core::PCWSTR,
    ) -> ::win32::core::HRESULT {
        todo!()
    }
    fn MrmCreateResourceIndexer(
        packageFamilyName: ::win32::core::PCWSTR,
        projectRoot: ::win32::core::PCWSTR,
        platformVersion: MrmPlatformVersion,
        defaultQualifiers: ::win32::core::PCWSTR,
        indexer: MutPtr<MrmResourceIndexerHandle>,
    ) -> ::win32::core::HRESULT {
        todo!()
    }
    fn MrmCreateResourceIndexerFromPreviousPriData(
        projectRoot: ::win32::core::PCWSTR,
        platformVersion: MrmPlatformVersion,
        defaultQualifiers: ::win32::core::PCWSTR,
        priData: ConstPtr<u8>,
        priSize: u32,
        indexer: MutPtr<MrmResourceIndexerHandle>,
    ) -> ::win32::core::HRESULT {
        todo!()
    }
    fn MrmCreateResourceIndexerFromPreviousPriFile(
        projectRoot: ::win32::core::PCWSTR,
        platformVersion: MrmPlatformVersion,
        defaultQualifiers: ::win32::core::PCWSTR,
        priFile: ::win32::core::PCWSTR,
        indexer: MutPtr<MrmResourceIndexerHandle>,
    ) -> ::win32::core::HRESULT {
        todo!()
    }
    fn MrmCreateResourceIndexerFromPreviousSchemaData(
        projectRoot: ::win32::core::PCWSTR,
        platformVersion: MrmPlatformVersion,
        defaultQualifiers: ::win32::core::PCWSTR,
        schemaXmlData: ConstPtr<u8>,
        schemaXmlSize: u32,
        indexer: MutPtr<MrmResourceIndexerHandle>,
    ) -> ::win32::core::HRESULT {
        todo!()
    }
    fn MrmCreateResourceIndexerFromPreviousSchemaFile(
        projectRoot: ::win32::core::PCWSTR,
        platformVersion: MrmPlatformVersion,
        defaultQualifiers: ::win32::core::PCWSTR,
        schemaFile: ::win32::core::PCWSTR,
        indexer: MutPtr<MrmResourceIndexerHandle>,
    ) -> ::win32::core::HRESULT {
        todo!()
    }
    fn MrmCreateResourceIndexerWithFlags(
        packageFamilyName: ::win32::core::PCWSTR,
        projectRoot: ::win32::core::PCWSTR,
        platformVersion: MrmPlatformVersion,
        defaultQualifiers: ::win32::core::PCWSTR,
        flags: MrmIndexerFlags,
        indexer: MutPtr<MrmResourceIndexerHandle>,
    ) -> ::win32::core::HRESULT {
        todo!()
    }
    fn MrmDestroyIndexerAndMessages(indexer: MrmResourceIndexerHandle) -> ::win32::core::HRESULT {
        todo!()
    }
    fn MrmDumpPriDataInMemory(
        inputPriData: ConstPtr<u8>,
        inputPriSize: u32,
        schemaPriData: ConstPtr<u8>,
        schemaPriSize: u32,
        dumpType: MrmDumpType,
        outputXmlData: MutPtr<ConstPtr<u8>>,
        outputXmlSize: MutPtr<u32>,
    ) -> ::win32::core::HRESULT {
        todo!()
    }
    fn MrmDumpPriFile(
        indexFileName: ::win32::core::PCWSTR,
        schemaPriFile: ::win32::core::PCWSTR,
        dumpType: MrmDumpType,
        outputXmlFile: ::win32::core::PCWSTR,
    ) -> ::win32::core::HRESULT {
        todo!()
    }
    fn MrmDumpPriFileInMemory(
        indexFileName: ::win32::core::PCWSTR,
        schemaPriFile: ::win32::core::PCWSTR,
        dumpType: MrmDumpType,
        outputXmlData: MutPtr<ConstPtr<u8>>,
        outputXmlSize: MutPtr<u32>,
    ) -> ::win32::core::HRESULT {
        todo!()
    }
    fn MrmFreeMemory(data: ConstPtr<u8>) -> ::win32::core::HRESULT {
        todo!()
    }
    fn MrmGetPriFileContentChecksum(
        priFile: ::win32::core::PCWSTR,
        checksum: MutPtr<u32>,
    ) -> ::win32::core::HRESULT {
        todo!()
    }
    fn MrmIndexEmbeddedData(
        indexer: MrmResourceIndexerHandle,
        resourceUri: ::win32::core::PCWSTR,
        embeddedData: ConstPtr<u8>,
        embeddedDataSize: u32,
        qualifiers: ::win32::core::PCWSTR,
    ) -> ::win32::core::HRESULT {
        todo!()
    }
    fn MrmIndexFile(
        indexer: MrmResourceIndexerHandle,
        resourceUri: ::win32::core::PCWSTR,
        filePath: ::win32::core::PCWSTR,
        qualifiers: ::win32::core::PCWSTR,
    ) -> ::win32::core::HRESULT {
        todo!()
    }
    fn MrmIndexFileAutoQualifiers(
        indexer: MrmResourceIndexerHandle,
        filePath: ::win32::core::PCWSTR,
    ) -> ::win32::core::HRESULT {
        todo!()
    }
    fn MrmIndexResourceContainerAutoQualifiers(
        indexer: MrmResourceIndexerHandle,
        containerPath: ::win32::core::PCWSTR,
    ) -> ::win32::core::HRESULT {
        todo!()
    }
    fn MrmIndexString(
        indexer: MrmResourceIndexerHandle,
        resourceUri: ::win32::core::PCWSTR,
        resourceString: ::win32::core::PCWSTR,
        qualifiers: ::win32::core::PCWSTR,
    ) -> ::win32::core::HRESULT {
        todo!()
    }
    fn MrmPeekResourceIndexerMessages(
        handle: MrmResourceIndexerHandle,
        messages: MutPtr<ConstPtr<MrmResourceIndexerMessage>>,
        numMsgs: MutPtr<u32>,
    ) -> ::win32::core::HRESULT {
        todo!()
    }
    fn MsgWaitForMultipleObjects(
        nCount: u32,
        pHandles: ConstPtr<super::super::Foundation::HANDLE>,
        fWaitAll: super::super::Foundation::BOOL,
        dwMilliseconds: u32,
        dwWakeMask: QUEUE_STATUS_FLAGS,
    ) -> u32 {
        todo!()
    }
    fn MsgWaitForMultipleObjectsEx(
        nCount: u32,
        pHandles: ConstPtr<super::super::Foundation::HANDLE>,
        dwMilliseconds: u32,
        dwWakeMask: QUEUE_STATUS_FLAGS,
        dwFlags: MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS,
    ) -> u32 {
        todo!()
    }
    fn OemToCharA(
        pSrc: ::win32::core::PCSTR,
        pDst: ::win32::core::PSTR,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn OemToCharBuffA(
        lpszSrc: ::win32::core::PCSTR,
        lpszDst: ::win32::core::PSTR,
        cchDstLength: u32,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn OemToCharBuffW(
        lpszSrc: ::win32::core::PCSTR,
        lpszDst: ::win32::core::PWSTR,
        cchDstLength: u32,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn OemToCharW(
        pSrc: ::win32::core::PCSTR,
        pDst: ::win32::core::PWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn OpenIcon(hWnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn PeekMessageA(
        lpMsg: MutPtr<MSG>,
        hWnd: super::super::Foundation::HWND,
        wMsgFilterMin: u32,
        wMsgFilterMax: u32,
        wRemoveMsg: PEEK_MESSAGE_REMOVE_TYPE,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn PeekMessageW(
        lpMsg: MutPtr<MSG>,
        hWnd: super::super::Foundation::HWND,
        wMsgFilterMin: u32,
        wMsgFilterMax: u32,
        wRemoveMsg: PEEK_MESSAGE_REMOVE_TYPE,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn PhysicalToLogicalPoint(
        hWnd: super::super::Foundation::HWND,
        lpPoint: MutPtr<super::super::Foundation::POINT>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn PostMessageA(
        hWnd: super::super::Foundation::HWND,
        Msg: u32,
        wParam: super::super::Foundation::WPARAM,
        lParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn PostMessageW(
        hWnd: super::super::Foundation::HWND,
        Msg: u32,
        wParam: super::super::Foundation::WPARAM,
        lParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn PostQuitMessage(nExitCode: i32) {
        todo!()
    }
    fn PostThreadMessageA(
        idThread: u32,
        Msg: u32,
        wParam: super::super::Foundation::WPARAM,
        lParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn PostThreadMessageW(
        idThread: u32,
        Msg: u32,
        wParam: super::super::Foundation::WPARAM,
        lParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn PrivateExtractIconsA(
        szFileName: ::win32::core::PCSTR,
        nIconIndex: i32,
        cxIcon: i32,
        cyIcon: i32,
        phicon: MutPtr<HICON>,
        piconid: MutPtr<u32>,
        nIcons: u32,
        flags: u32,
    ) -> u32 {
        todo!()
    }
    fn PrivateExtractIconsW(
        szFileName: ::win32::core::PCWSTR,
        nIconIndex: i32,
        cxIcon: i32,
        cyIcon: i32,
        phicon: MutPtr<HICON>,
        piconid: MutPtr<u32>,
        nIcons: u32,
        flags: u32,
    ) -> u32 {
        todo!()
    }
    fn RealChildWindowFromPoint(
        hwndParent: super::super::Foundation::HWND,
        ptParentClientCoords: super::super::Foundation::POINT,
    ) -> super::super::Foundation::HWND {
        todo!()
    }
    fn RealGetWindowClassA(
        hwnd: super::super::Foundation::HWND,
        ptszClassName: ::win32::core::PSTR,
        cchClassNameMax: u32,
    ) -> u32 {
        todo!()
    }
    fn RealGetWindowClassW(
        hwnd: super::super::Foundation::HWND,
        ptszClassName: ::win32::core::PWSTR,
        cchClassNameMax: u32,
    ) -> u32 {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn RegisterClassA(lpWndClass: ConstPtr<WNDCLASSA>) -> u16 {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn RegisterClassExA(param0: ConstPtr<WNDCLASSEXA>) -> u16 {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn RegisterClassExW(param0: ConstPtr<WNDCLASSEXW>) -> u16 {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn RegisterClassW(lpWndClass: ConstPtr<WNDCLASSW>) -> u16 {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Power'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn RegisterDeviceNotificationA(
        hRecipient: super::super::Foundation::HANDLE,
        NotificationFilter: ConstPtr<::core::ffi::c_void>,
        Flags: super::super::System::Power::POWER_SETTING_REGISTER_NOTIFICATION_FLAGS,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Power'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn RegisterDeviceNotificationW(
        hRecipient: super::super::Foundation::HANDLE,
        NotificationFilter: ConstPtr<::core::ffi::c_void>,
        Flags: super::super::System::Power::POWER_SETTING_REGISTER_NOTIFICATION_FLAGS,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!()
    }
    fn RegisterShellHookWindow(
        hwnd: super::super::Foundation::HWND,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn RegisterWindowMessageA(lpString: ::win32::core::PCSTR) -> u32 {
        todo!()
    }
    fn RegisterWindowMessageW(lpString: ::win32::core::PCWSTR) -> u32 {
        todo!()
    }
    fn RemoveMenu(
        hMenu: HMENU,
        uPosition: u32,
        uFlags: MENU_ITEM_FLAGS,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn RemovePropA(
        hWnd: super::super::Foundation::HWND,
        lpString: ::win32::core::PCSTR,
    ) -> super::super::Foundation::HANDLE {
        todo!()
    }
    fn RemovePropW(
        hWnd: super::super::Foundation::HWND,
        lpString: ::win32::core::PCWSTR,
    ) -> super::super::Foundation::HANDLE {
        todo!()
    }
    fn ReplyMessage(lResult: super::super::Foundation::LRESULT) -> super::super::Foundation::BOOL {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn ScrollDC(
        hDC: super::super::Graphics::Gdi::HDC,
        dx: i32,
        dy: i32,
        lprcScroll: ConstPtr<super::super::Foundation::RECT>,
        lprcClip: ConstPtr<super::super::Foundation::RECT>,
        hrgnUpdate: super::super::Graphics::Gdi::HRGN,
        lprcUpdate: MutPtr<super::super::Foundation::RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn ScrollWindow(
        hWnd: super::super::Foundation::HWND,
        XAmount: i32,
        YAmount: i32,
        lpRect: ConstPtr<super::super::Foundation::RECT>,
        lpClipRect: ConstPtr<super::super::Foundation::RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn ScrollWindowEx(
        hWnd: super::super::Foundation::HWND,
        dx: i32,
        dy: i32,
        prcScroll: ConstPtr<super::super::Foundation::RECT>,
        prcClip: ConstPtr<super::super::Foundation::RECT>,
        hrgnUpdate: super::super::Graphics::Gdi::HRGN,
        prcUpdate: MutPtr<super::super::Foundation::RECT>,
        flags: SHOW_WINDOW_CMD,
    ) -> i32 {
        todo!()
    }
    fn SendDlgItemMessageA(
        hDlg: super::super::Foundation::HWND,
        nIDDlgItem: i32,
        Msg: u32,
        wParam: super::super::Foundation::WPARAM,
        lParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::LRESULT {
        todo!()
    }
    fn SendDlgItemMessageW(
        hDlg: super::super::Foundation::HWND,
        nIDDlgItem: i32,
        Msg: u32,
        wParam: super::super::Foundation::WPARAM,
        lParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::LRESULT {
        todo!()
    }
    fn SendMessageA(
        hWnd: super::super::Foundation::HWND,
        Msg: u32,
        wParam: super::super::Foundation::WPARAM,
        lParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::LRESULT {
        todo!()
    }
    fn SendMessageCallbackA(
        hWnd: super::super::Foundation::HWND,
        Msg: u32,
        wParam: super::super::Foundation::WPARAM,
        lParam: super::super::Foundation::LPARAM,
        lpResultCallBack: SENDASYNCPROC,
        dwData: usize,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SendMessageCallbackW(
        hWnd: super::super::Foundation::HWND,
        Msg: u32,
        wParam: super::super::Foundation::WPARAM,
        lParam: super::super::Foundation::LPARAM,
        lpResultCallBack: SENDASYNCPROC,
        dwData: usize,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SendMessageTimeoutA(
        hWnd: super::super::Foundation::HWND,
        Msg: u32,
        wParam: super::super::Foundation::WPARAM,
        lParam: super::super::Foundation::LPARAM,
        fuFlags: SEND_MESSAGE_TIMEOUT_FLAGS,
        uTimeout: u32,
        lpdwResult: MutPtr<usize>,
    ) -> super::super::Foundation::LRESULT {
        todo!()
    }
    fn SendMessageTimeoutW(
        hWnd: super::super::Foundation::HWND,
        Msg: u32,
        wParam: super::super::Foundation::WPARAM,
        lParam: super::super::Foundation::LPARAM,
        fuFlags: SEND_MESSAGE_TIMEOUT_FLAGS,
        uTimeout: u32,
        lpdwResult: MutPtr<usize>,
    ) -> super::super::Foundation::LRESULT {
        todo!()
    }
    fn SendMessageW(
        hWnd: super::super::Foundation::HWND,
        Msg: u32,
        wParam: super::super::Foundation::WPARAM,
        lParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::LRESULT {
        todo!()
    }
    fn SendNotifyMessageA(
        hWnd: super::super::Foundation::HWND,
        Msg: u32,
        wParam: super::super::Foundation::WPARAM,
        lParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SendNotifyMessageW(
        hWnd: super::super::Foundation::HWND,
        Msg: u32,
        wParam: super::super::Foundation::WPARAM,
        lParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetCaretBlinkTime(uMSeconds: u32) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetCaretPos(X: i32, Y: i32) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetClassLongA(
        hWnd: super::super::Foundation::HWND,
        nIndex: GET_CLASS_LONG_INDEX,
        dwNewLong: i32,
    ) -> u32 {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn SetClassLongPtrA(
        hWnd: super::super::Foundation::HWND,
        nIndex: GET_CLASS_LONG_INDEX,
        dwNewLong: isize,
    ) -> usize {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn SetClassLongPtrW(
        hWnd: super::super::Foundation::HWND,
        nIndex: GET_CLASS_LONG_INDEX,
        dwNewLong: isize,
    ) -> usize {
        todo!()
    }
    fn SetClassLongW(
        hWnd: super::super::Foundation::HWND,
        nIndex: GET_CLASS_LONG_INDEX,
        dwNewLong: i32,
    ) -> u32 {
        todo!()
    }
    fn SetClassWord(hWnd: super::super::Foundation::HWND, nIndex: i32, wNewWord: u16) -> u16 {
        todo!()
    }
    fn SetCoalescableTimer(
        hWnd: super::super::Foundation::HWND,
        nIDEvent: usize,
        uElapse: u32,
        lpTimerFunc: TIMERPROC,
        uToleranceDelay: u32,
    ) -> usize {
        todo!()
    }
    fn SetCursor(hCursor: HCURSOR) -> HCURSOR {
        todo!()
    }
    fn SetCursorPos(X: i32, Y: i32) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetDebugErrorLevel(dwLevel: u32) {
        todo!()
    }
    fn SetDlgItemInt(
        hDlg: super::super::Foundation::HWND,
        nIDDlgItem: i32,
        uValue: u32,
        bSigned: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetDlgItemTextA(
        hDlg: super::super::Foundation::HWND,
        nIDDlgItem: i32,
        lpString: ::win32::core::PCSTR,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetDlgItemTextW(
        hDlg: super::super::Foundation::HWND,
        nIDDlgItem: i32,
        lpString: ::win32::core::PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetForegroundWindow(hWnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetLayeredWindowAttributes(
        hwnd: super::super::Foundation::HWND,
        crKey: u32,
        bAlpha: u8,
        dwFlags: LAYERED_WINDOW_ATTRIBUTES_FLAGS,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetMenu(
        hWnd: super::super::Foundation::HWND,
        hMenu: HMENU,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetMenuDefaultItem(hMenu: HMENU, uItem: u32, fByPos: u32) -> super::super::Foundation::BOOL {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn SetMenuInfo(param0: HMENU, param1: ConstPtr<MENUINFO>) -> super::super::Foundation::BOOL {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn SetMenuItemBitmaps(
        hMenu: HMENU,
        uPosition: u32,
        uFlags: MENU_ITEM_FLAGS,
        hBitmapUnchecked: super::super::Graphics::Gdi::HBITMAP,
        hBitmapChecked: super::super::Graphics::Gdi::HBITMAP,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn SetMenuItemInfoA(
        hmenu: HMENU,
        item: u32,
        fByPositon: super::super::Foundation::BOOL,
        lpmii: ConstPtr<MENUITEMINFOA>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn SetMenuItemInfoW(
        hmenu: HMENU,
        item: u32,
        fByPositon: super::super::Foundation::BOOL,
        lpmii: ConstPtr<MENUITEMINFOW>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetMessageExtraInfo(
        lParam: super::super::Foundation::LPARAM,
    ) -> super::super::Foundation::LPARAM {
        todo!()
    }
    fn SetMessageQueue(cMessagesMax: i32) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetParent(
        hWndChild: super::super::Foundation::HWND,
        hWndNewParent: super::super::Foundation::HWND,
    ) -> super::super::Foundation::HWND {
        todo!()
    }
    fn SetPhysicalCursorPos(X: i32, Y: i32) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetProcessDPIAware() -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetProcessDefaultLayout(dwDefaultLayout: u32) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetPropA(
        hWnd: super::super::Foundation::HWND,
        lpString: ::win32::core::PCSTR,
        hData: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetPropW(
        hWnd: super::super::Foundation::HWND,
        lpString: ::win32::core::PCWSTR,
        hData: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetSysColors(
        cElements: i32,
        lpaElements: ConstPtr<i32>,
        lpaRgbValues: ConstPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetSystemCursor(hcur: HCURSOR, id: SYSTEM_CURSOR_ID) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetTimer(
        hWnd: super::super::Foundation::HWND,
        nIDEvent: usize,
        uElapse: u32,
        lpTimerFunc: TIMERPROC,
    ) -> usize {
        todo!()
    }
    fn SetWindowDisplayAffinity(
        hWnd: super::super::Foundation::HWND,
        dwAffinity: WINDOW_DISPLAY_AFFINITY,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetWindowLongA(
        hWnd: super::super::Foundation::HWND,
        nIndex: WINDOW_LONG_PTR_INDEX,
        dwNewLong: i32,
    ) -> i32 {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn SetWindowLongPtrA(
        hWnd: super::super::Foundation::HWND,
        nIndex: WINDOW_LONG_PTR_INDEX,
        dwNewLong: isize,
    ) -> isize {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn SetWindowLongPtrW(
        hWnd: super::super::Foundation::HWND,
        nIndex: WINDOW_LONG_PTR_INDEX,
        dwNewLong: isize,
    ) -> isize {
        todo!()
    }
    fn SetWindowLongW(
        hWnd: super::super::Foundation::HWND,
        nIndex: WINDOW_LONG_PTR_INDEX,
        dwNewLong: i32,
    ) -> i32 {
        todo!()
    }
    fn SetWindowPlacement(
        hWnd: super::super::Foundation::HWND,
        lpwndpl: ConstPtr<WINDOWPLACEMENT>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetWindowPos(
        hWnd: super::super::Foundation::HWND,
        hWndInsertAfter: super::super::Foundation::HWND,
        X: i32,
        Y: i32,
        cx: i32,
        cy: i32,
        uFlags: SET_WINDOW_POS_FLAGS,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetWindowTextA(
        hWnd: super::super::Foundation::HWND,
        lpString: ::win32::core::PCSTR,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetWindowTextW(
        hWnd: super::super::Foundation::HWND,
        lpString: ::win32::core::PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SetWindowWord(hWnd: super::super::Foundation::HWND, nIndex: i32, wNewWord: u16) -> u16 {
        todo!()
    }
    fn SetWindowsHookA(nFilterType: i32, pfnFilterProc: HOOKPROC) -> HHOOK {
        todo!()
    }
    fn SetWindowsHookExA(
        idHook: WINDOWS_HOOK_ID,
        lpfn: HOOKPROC,
        hmod: super::super::Foundation::HINSTANCE,
        dwThreadId: u32,
    ) -> HHOOK {
        todo!()
    }
    fn SetWindowsHookExW(
        idHook: WINDOWS_HOOK_ID,
        lpfn: HOOKPROC,
        hmod: super::super::Foundation::HINSTANCE,
        dwThreadId: u32,
    ) -> HHOOK {
        todo!()
    }
    fn SetWindowsHookW(nFilterType: i32, pfnFilterProc: HOOKPROC) -> HHOOK {
        todo!()
    }
    fn ShowCaret(hWnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn ShowCursor(bShow: super::super::Foundation::BOOL) -> i32 {
        todo!()
    }
    fn ShowOwnedPopups(
        hWnd: super::super::Foundation::HWND,
        fShow: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn ShowWindow(
        hWnd: super::super::Foundation::HWND,
        nCmdShow: SHOW_WINDOW_CMD,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn ShowWindowAsync(
        hWnd: super::super::Foundation::HWND,
        nCmdShow: SHOW_WINDOW_CMD,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SoundSentry() -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SwitchToThisWindow(
        hwnd: super::super::Foundation::HWND,
        fUnknown: super::super::Foundation::BOOL,
    ) {
        todo!()
    }
    fn SystemParametersInfoA(
        uiAction: SYSTEM_PARAMETERS_INFO_ACTION,
        uiParam: u32,
        pvParam: MutPtr<::core::ffi::c_void>,
        fWinIni: SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn SystemParametersInfoW(
        uiAction: SYSTEM_PARAMETERS_INFO_ACTION,
        uiParam: u32,
        pvParam: MutPtr<::core::ffi::c_void>,
        fWinIni: SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn TileWindows(
        hwndParent: super::super::Foundation::HWND,
        wHow: TILE_WINDOWS_HOW,
        lpRect: ConstPtr<super::super::Foundation::RECT>,
        cKids: u32,
        lpKids: ConstPtr<super::super::Foundation::HWND>,
    ) -> u16 {
        todo!()
    }
    fn TrackPopupMenu(
        hMenu: HMENU,
        uFlags: TRACK_POPUP_MENU_FLAGS,
        x: i32,
        y: i32,
        nReserved: i32,
        hWnd: super::super::Foundation::HWND,
        prcRect: ConstPtr<super::super::Foundation::RECT>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn TrackPopupMenuEx(
        hMenu: HMENU,
        uFlags: u32,
        x: i32,
        y: i32,
        hwnd: super::super::Foundation::HWND,
        lptpm: ConstPtr<TPMPARAMS>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn TranslateAcceleratorA(
        hWnd: super::super::Foundation::HWND,
        hAccTable: HACCEL,
        lpMsg: ConstPtr<MSG>,
    ) -> i32 {
        todo!()
    }
    fn TranslateAcceleratorW(
        hWnd: super::super::Foundation::HWND,
        hAccTable: HACCEL,
        lpMsg: ConstPtr<MSG>,
    ) -> i32 {
        todo!()
    }
    fn TranslateMDISysAccel(
        hWndClient: super::super::Foundation::HWND,
        lpMsg: ConstPtr<MSG>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn TranslateMessage(lpMsg: ConstPtr<MSG>) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn UnhookWindowsHook(nCode: i32, pfnFilterProc: HOOKPROC) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn UnhookWindowsHookEx(hhk: HHOOK) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn UnregisterClassA(
        lpClassName: ::win32::core::PCSTR,
        hInstance: super::super::Foundation::HINSTANCE,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn UnregisterClassW(
        lpClassName: ::win32::core::PCWSTR,
        hInstance: super::super::Foundation::HINSTANCE,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn UpdateLayeredWindow(
        hWnd: super::super::Foundation::HWND,
        hdcDst: super::super::Graphics::Gdi::HDC,
        pptDst: ConstPtr<super::super::Foundation::POINT>,
        psize: ConstPtr<super::super::Foundation::SIZE>,
        hdcSrc: super::super::Graphics::Gdi::HDC,
        pptSrc: ConstPtr<super::super::Foundation::POINT>,
        crKey: u32,
        pblend: ConstPtr<super::super::Graphics::Gdi::BLENDFUNCTION>,
        dwFlags: UPDATE_LAYERED_WINDOW_FLAGS,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.Graphics.Gdi'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    fn UpdateLayeredWindowIndirect(
        hWnd: super::super::Foundation::HWND,
        pULWInfo: ConstPtr<UPDATELAYEREDWINDOWINFO>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn WaitMessage() -> super::super::Foundation::BOOL {
        todo!()
    }
    fn WindowFromPhysicalPoint(
        Point: super::super::Foundation::POINT,
    ) -> super::super::Foundation::HWND {
        todo!()
    }
    fn WindowFromPoint(Point: super::super::Foundation::POINT) -> super::super::Foundation::HWND {
        todo!()
    }
    fn wsprintfA(param0: ::win32::core::PSTR, param1: ::win32::core::PCSTR) -> i32 {
        todo!()
    }
    fn wsprintfW(param0: ::win32::core::PWSTR, param1: ::win32::core::PCWSTR) -> i32 {
        todo!()
    }
    fn wvsprintfA(
        param0: ::win32::core::PSTR,
        param1: ::win32::core::PCSTR,
        arglist: ConstPtr<i8>,
    ) -> i32 {
        todo!()
    }
    fn wvsprintfW(
        param0: ::win32::core::PWSTR,
        param1: ::win32::core::PCWSTR,
        arglist: ConstPtr<i8>,
    ) -> i32 {
        todo!()
    }
}
