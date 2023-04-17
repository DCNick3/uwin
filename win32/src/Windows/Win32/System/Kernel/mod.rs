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
pub struct COMPARTMENT_ID(pub i32);
pub const UNSPECIFIED_COMPARTMENT_ID: COMPARTMENT_ID = COMPARTMENT_ID(0i32);
pub const DEFAULT_COMPARTMENT_ID: COMPARTMENT_ID = COMPARTMENT_ID(1i32);
impl ::core::marker::Copy for COMPARTMENT_ID {}
impl ::core::clone::Clone for COMPARTMENT_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMPARTMENT_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COMPARTMENT_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMPARTMENT_ID").field(&self.0).finish()
    }
}
impl FromIntoMemory for COMPARTMENT_ID {
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
pub struct CSTRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: PCSTR,
}
impl ::core::marker::Copy for CSTRING {}
impl ::core::clone::Clone for CSTRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CSTRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSTRING")
            .field("Length", &self.Length)
            .field("MaximumLength", &self.MaximumLength)
            .field("Buffer", &self.Buffer)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CSTRING {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
            && self.MaximumLength == other.MaximumLength
            && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for CSTRING {}
impl FromIntoMemory for CSTRING {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Length = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_MaximumLength = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_Buffer = <PCSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            Length: f_Length,
            MaximumLength: f_MaximumLength,
            Buffer: f_Buffer,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Length, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.MaximumLength, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.Buffer, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EVENT_TYPE(pub i32);
pub const NotificationEvent: EVENT_TYPE = EVENT_TYPE(0i32);
pub const SynchronizationEvent: EVENT_TYPE = EVENT_TYPE(1i32);
impl ::core::marker::Copy for EVENT_TYPE {}
impl ::core::clone::Clone for EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVENT_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for EVENT_TYPE {
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
pub struct EXCEPTION_DISPOSITION(pub i32);
pub const ExceptionContinueExecution: EXCEPTION_DISPOSITION = EXCEPTION_DISPOSITION(0i32);
pub const ExceptionContinueSearch: EXCEPTION_DISPOSITION = EXCEPTION_DISPOSITION(1i32);
pub const ExceptionNestedException: EXCEPTION_DISPOSITION = EXCEPTION_DISPOSITION(2i32);
pub const ExceptionCollidedUnwind: EXCEPTION_DISPOSITION = EXCEPTION_DISPOSITION(3i32);
impl ::core::marker::Copy for EXCEPTION_DISPOSITION {}
impl ::core::clone::Clone for EXCEPTION_DISPOSITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EXCEPTION_DISPOSITION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EXCEPTION_DISPOSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EXCEPTION_DISPOSITION")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for EXCEPTION_DISPOSITION {
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
pub struct EXCEPTION_REGISTRATION_RECORD {
    pub Next: MutPtr<EXCEPTION_REGISTRATION_RECORD>,
    pub Handler: EXCEPTION_ROUTINE,
}
impl ::core::marker::Copy for EXCEPTION_REGISTRATION_RECORD {}
impl ::core::clone::Clone for EXCEPTION_REGISTRATION_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EXCEPTION_REGISTRATION_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXCEPTION_REGISTRATION_RECORD")
            .field("Next", &self.Next)
            .field("Handler", &self.Handler)
            .finish()
    }
}
impl ::core::cmp::PartialEq for EXCEPTION_REGISTRATION_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Handler == other.Handler
    }
}
impl ::core::cmp::Eq for EXCEPTION_REGISTRATION_RECORD {}
impl FromIntoMemory for EXCEPTION_REGISTRATION_RECORD {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Next =
            <MutPtr<EXCEPTION_REGISTRATION_RECORD> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Handler = <EXCEPTION_ROUTINE as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            Next: f_Next,
            Handler: f_Handler,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Next, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Handler, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub type EXCEPTION_ROUTINE = StdCallFnPtr<
    (
        MutPtr<super::Diagnostics::Debug::EXCEPTION_RECORD>,
        ConstPtr<::core::ffi::c_void>,
        MutPtr<super::Diagnostics::Debug::CONTEXT>,
        ConstPtr<::core::ffi::c_void>,
    ),
    EXCEPTION_DISPOSITION,
>;
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct FLOATING_SAVE_AREA {
    pub ControlWord: u32,
    pub StatusWord: u32,
    pub TagWord: u32,
    pub ErrorOffset: u32,
    pub ErrorSelector: u32,
    pub DataOffset: u32,
    pub DataSelector: u32,
    pub RegisterArea: [u8; 80],
    pub Cr0NpxState: u32,
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for FLOATING_SAVE_AREA {}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for FLOATING_SAVE_AREA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for FLOATING_SAVE_AREA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLOATING_SAVE_AREA")
            .field("ControlWord", &self.ControlWord)
            .field("StatusWord", &self.StatusWord)
            .field("TagWord", &self.TagWord)
            .field("ErrorOffset", &self.ErrorOffset)
            .field("ErrorSelector", &self.ErrorSelector)
            .field("DataOffset", &self.DataOffset)
            .field("DataSelector", &self.DataSelector)
            .field("RegisterArea", &self.RegisterArea)
            .field("Cr0NpxState", &self.Cr0NpxState)
            .finish()
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for FLOATING_SAVE_AREA {
    fn eq(&self, other: &Self) -> bool {
        self.ControlWord == other.ControlWord
            && self.StatusWord == other.StatusWord
            && self.TagWord == other.TagWord
            && self.ErrorOffset == other.ErrorOffset
            && self.ErrorSelector == other.ErrorSelector
            && self.DataOffset == other.DataOffset
            && self.DataSelector == other.DataSelector
            && self.RegisterArea == other.RegisterArea
            && self.Cr0NpxState == other.Cr0NpxState
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for FLOATING_SAVE_AREA {}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for FLOATING_SAVE_AREA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 112);
        let f_ControlWord = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_StatusWord = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_TagWord = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ErrorOffset = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_ErrorSelector = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_DataOffset = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_DataSelector = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_RegisterArea = <[u8; 80] as FromIntoMemory>::from_bytes(&from[28..28 + 80]);
        let f_Cr0NpxState = <u32 as FromIntoMemory>::from_bytes(&from[108..108 + 4]);
        Self {
            ControlWord: f_ControlWord,
            StatusWord: f_StatusWord,
            TagWord: f_TagWord,
            ErrorOffset: f_ErrorOffset,
            ErrorSelector: f_ErrorSelector,
            DataOffset: f_DataOffset,
            DataSelector: f_DataSelector,
            RegisterArea: f_RegisterArea,
            Cr0NpxState: f_Cr0NpxState,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 112);
        FromIntoMemory::into_bytes(self.ControlWord, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.StatusWord, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.TagWord, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ErrorOffset, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.ErrorSelector, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.DataOffset, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.DataSelector, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.RegisterArea, &mut into[28..28 + 80]);
        FromIntoMemory::into_bytes(self.Cr0NpxState, &mut into[108..108 + 4]);
    }
    fn size() -> usize {
        112
    }
}
pub struct FLOATING_SAVE_AREA {
    pub ControlWord: u32,
    pub StatusWord: u32,
    pub TagWord: u32,
    pub ErrorOffset: u32,
    pub ErrorSelector: u32,
    pub DataOffset: u32,
    pub DataSelector: u32,
    pub RegisterArea: [u8; 80],
    pub Spare0: u32,
}
impl ::core::marker::Copy for FLOATING_SAVE_AREA {}
impl ::core::clone::Clone for FLOATING_SAVE_AREA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FLOATING_SAVE_AREA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLOATING_SAVE_AREA")
            .field("ControlWord", &self.ControlWord)
            .field("StatusWord", &self.StatusWord)
            .field("TagWord", &self.TagWord)
            .field("ErrorOffset", &self.ErrorOffset)
            .field("ErrorSelector", &self.ErrorSelector)
            .field("DataOffset", &self.DataOffset)
            .field("DataSelector", &self.DataSelector)
            .field("RegisterArea", &self.RegisterArea)
            .field("Spare0", &self.Spare0)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FLOATING_SAVE_AREA {
    fn eq(&self, other: &Self) -> bool {
        self.ControlWord == other.ControlWord
            && self.StatusWord == other.StatusWord
            && self.TagWord == other.TagWord
            && self.ErrorOffset == other.ErrorOffset
            && self.ErrorSelector == other.ErrorSelector
            && self.DataOffset == other.DataOffset
            && self.DataSelector == other.DataSelector
            && self.RegisterArea == other.RegisterArea
            && self.Spare0 == other.Spare0
    }
}
impl ::core::cmp::Eq for FLOATING_SAVE_AREA {}
impl FromIntoMemory for FLOATING_SAVE_AREA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 112);
        let f_ControlWord = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_StatusWord = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_TagWord = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ErrorOffset = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_ErrorSelector = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_DataOffset = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_DataSelector = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_RegisterArea = <[u8; 80] as FromIntoMemory>::from_bytes(&from[28..28 + 80]);
        let f_Spare0 = <u32 as FromIntoMemory>::from_bytes(&from[108..108 + 4]);
        Self {
            ControlWord: f_ControlWord,
            StatusWord: f_StatusWord,
            TagWord: f_TagWord,
            ErrorOffset: f_ErrorOffset,
            ErrorSelector: f_ErrorSelector,
            DataOffset: f_DataOffset,
            DataSelector: f_DataSelector,
            RegisterArea: f_RegisterArea,
            Spare0: f_Spare0,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 112);
        FromIntoMemory::into_bytes(self.ControlWord, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.StatusWord, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.TagWord, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ErrorOffset, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.ErrorSelector, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.DataOffset, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.DataSelector, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.RegisterArea, &mut into[28..28 + 80]);
        FromIntoMemory::into_bytes(self.Spare0, &mut into[108..108 + 4]);
    }
    fn size() -> usize {
        112
    }
}
pub struct LIST_ENTRY {
    pub Flink: MutPtr<LIST_ENTRY>,
    pub Blink: MutPtr<LIST_ENTRY>,
}
impl ::core::marker::Copy for LIST_ENTRY {}
impl ::core::clone::Clone for LIST_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LIST_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LIST_ENTRY")
            .field("Flink", &self.Flink)
            .field("Blink", &self.Blink)
            .finish()
    }
}
impl ::core::cmp::PartialEq for LIST_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Flink == other.Flink && self.Blink == other.Blink
    }
}
impl ::core::cmp::Eq for LIST_ENTRY {}
impl FromIntoMemory for LIST_ENTRY {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Flink = <MutPtr<LIST_ENTRY> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Blink = <MutPtr<LIST_ENTRY> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            Flink: f_Flink,
            Blink: f_Blink,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Flink, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Blink, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct LIST_ENTRY32 {
    pub Flink: u32,
    pub Blink: u32,
}
impl ::core::marker::Copy for LIST_ENTRY32 {}
impl ::core::clone::Clone for LIST_ENTRY32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LIST_ENTRY32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LIST_ENTRY32")
            .field("Flink", &self.Flink)
            .field("Blink", &self.Blink)
            .finish()
    }
}
impl ::core::cmp::PartialEq for LIST_ENTRY32 {
    fn eq(&self, other: &Self) -> bool {
        self.Flink == other.Flink && self.Blink == other.Blink
    }
}
impl ::core::cmp::Eq for LIST_ENTRY32 {}
impl FromIntoMemory for LIST_ENTRY32 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Flink = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Blink = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            Flink: f_Flink,
            Blink: f_Blink,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Flink, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Blink, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct LIST_ENTRY64 {
    pub Flink: u64,
    pub Blink: u64,
}
impl ::core::marker::Copy for LIST_ENTRY64 {}
impl ::core::clone::Clone for LIST_ENTRY64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LIST_ENTRY64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LIST_ENTRY64")
            .field("Flink", &self.Flink)
            .field("Blink", &self.Blink)
            .finish()
    }
}
impl ::core::cmp::PartialEq for LIST_ENTRY64 {
    fn eq(&self, other: &Self) -> bool {
        self.Flink == other.Flink && self.Blink == other.Blink
    }
}
impl ::core::cmp::Eq for LIST_ENTRY64 {}
impl FromIntoMemory for LIST_ENTRY64 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_Flink = <u64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_Blink = <u64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        Self {
            Flink: f_Flink,
            Blink: f_Blink,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.Flink, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.Blink, &mut into[8..8 + 8]);
    }
    fn size() -> usize {
        16
    }
}
pub const MAXUCHAR: u32 = 255u32;
pub const MAXULONG: u32 = 4294967295u32;
pub const MAXUSHORT: u32 = 65535u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NT_PRODUCT_TYPE(pub i32);
pub const NtProductWinNt: NT_PRODUCT_TYPE = NT_PRODUCT_TYPE(1i32);
pub const NtProductLanManNt: NT_PRODUCT_TYPE = NT_PRODUCT_TYPE(2i32);
pub const NtProductServer: NT_PRODUCT_TYPE = NT_PRODUCT_TYPE(3i32);
impl ::core::marker::Copy for NT_PRODUCT_TYPE {}
impl ::core::clone::Clone for NT_PRODUCT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NT_PRODUCT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NT_PRODUCT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NT_PRODUCT_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for NT_PRODUCT_TYPE {
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
pub struct NT_TIB {
    pub ExceptionList: MutPtr<EXCEPTION_REGISTRATION_RECORD>,
    pub StackBase: MutPtr<::core::ffi::c_void>,
    pub StackLimit: MutPtr<::core::ffi::c_void>,
    pub SubSystemTib: MutPtr<::core::ffi::c_void>,
    pub Anonymous: NT_TIB_0,
    pub ArbitraryUserPointer: MutPtr<::core::ffi::c_void>,
    pub Self_: MutPtr<NT_TIB>,
}
impl ::core::marker::Copy for NT_TIB {}
impl ::core::clone::Clone for NT_TIB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NT_TIB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NT_TIB")
            .field("ExceptionList", &self.ExceptionList)
            .field("StackBase", &self.StackBase)
            .field("StackLimit", &self.StackLimit)
            .field("SubSystemTib", &self.SubSystemTib)
            .field("Anonymous", &self.Anonymous)
            .field("ArbitraryUserPointer", &self.ArbitraryUserPointer)
            .field("Self", &self.Self_)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NT_TIB {
    fn eq(&self, other: &Self) -> bool {
        self.ExceptionList == other.ExceptionList
            && self.StackBase == other.StackBase
            && self.StackLimit == other.StackLimit
            && self.SubSystemTib == other.SubSystemTib
            && self.Anonymous == other.Anonymous
            && self.ArbitraryUserPointer == other.ArbitraryUserPointer
            && self.Self_ == other.Self_
    }
}
impl ::core::cmp::Eq for NT_TIB {}
impl FromIntoMemory for NT_TIB {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_ExceptionList =
            <MutPtr<EXCEPTION_REGISTRATION_RECORD> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_StackBase =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_StackLimit =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_SubSystemTib =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_Anonymous = <NT_TIB_0 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_ArbitraryUserPointer =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_Self = <MutPtr<NT_TIB> as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        Self {
            ExceptionList: f_ExceptionList,
            StackBase: f_StackBase,
            StackLimit: f_StackLimit,
            SubSystemTib: f_SubSystemTib,
            Anonymous: f_Anonymous,
            ArbitraryUserPointer: f_ArbitraryUserPointer,
            Self_: f_Self,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.ExceptionList, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.StackBase, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.StackLimit, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.SubSystemTib, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.ArbitraryUserPointer, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.Self_, &mut into[24..24 + 4]);
    }
    fn size() -> usize {
        28
    }
}
pub struct NT_TIB_0 {
    data: [u8; 4],
}
impl ::core::default::Default for NT_TIB_0 {
    fn default() -> Self {
        Self { data: [0u8; 4] }
    }
}
impl ::core::marker::Copy for NT_TIB_0 {}
impl ::core::clone::Clone for NT_TIB_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NT_TIB_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NT_TIB_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NT_TIB_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for NT_TIB_0 {}
impl FromIntoMemory for NT_TIB_0 {
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
pub const NULL64: u32 = 0u32;
pub struct OBJECTID {
    pub Lineage: crate::core::GUID,
    pub Uniquifier: u32,
}
impl ::core::marker::Copy for OBJECTID {}
impl ::core::clone::Clone for OBJECTID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OBJECTID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OBJECTID")
            .field("Lineage", &self.Lineage)
            .field("Uniquifier", &self.Uniquifier)
            .finish()
    }
}
impl ::core::cmp::PartialEq for OBJECTID {
    fn eq(&self, other: &Self) -> bool {
        self.Lineage == other.Lineage && self.Uniquifier == other.Uniquifier
    }
}
impl ::core::cmp::Eq for OBJECTID {}
impl FromIntoMemory for OBJECTID {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_Lineage = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_Uniquifier = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            Lineage: f_Lineage,
            Uniquifier: f_Uniquifier,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.Lineage, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.Uniquifier, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub struct OBJECT_ATTRIBUTES32 {
    pub Length: u32,
    pub RootDirectory: u32,
    pub ObjectName: u32,
    pub Attributes: u32,
    pub SecurityDescriptor: u32,
    pub SecurityQualityOfService: u32,
}
impl ::core::marker::Copy for OBJECT_ATTRIBUTES32 {}
impl ::core::clone::Clone for OBJECT_ATTRIBUTES32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OBJECT_ATTRIBUTES32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OBJECT_ATTRIBUTES32")
            .field("Length", &self.Length)
            .field("RootDirectory", &self.RootDirectory)
            .field("ObjectName", &self.ObjectName)
            .field("Attributes", &self.Attributes)
            .field("SecurityDescriptor", &self.SecurityDescriptor)
            .field("SecurityQualityOfService", &self.SecurityQualityOfService)
            .finish()
    }
}
impl ::core::cmp::PartialEq for OBJECT_ATTRIBUTES32 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
            && self.RootDirectory == other.RootDirectory
            && self.ObjectName == other.ObjectName
            && self.Attributes == other.Attributes
            && self.SecurityDescriptor == other.SecurityDescriptor
            && self.SecurityQualityOfService == other.SecurityQualityOfService
    }
}
impl ::core::cmp::Eq for OBJECT_ATTRIBUTES32 {}
impl FromIntoMemory for OBJECT_ATTRIBUTES32 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_Length = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_RootDirectory = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_ObjectName = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_Attributes = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_SecurityDescriptor = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_SecurityQualityOfService = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            Length: f_Length,
            RootDirectory: f_RootDirectory,
            ObjectName: f_ObjectName,
            Attributes: f_Attributes,
            SecurityDescriptor: f_SecurityDescriptor,
            SecurityQualityOfService: f_SecurityQualityOfService,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.Length, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.RootDirectory, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.ObjectName, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.Attributes, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.SecurityDescriptor, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.SecurityQualityOfService, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct OBJECT_ATTRIBUTES64 {
    pub Length: u32,
    pub RootDirectory: u64,
    pub ObjectName: u64,
    pub Attributes: u32,
    pub SecurityDescriptor: u64,
    pub SecurityQualityOfService: u64,
}
impl ::core::marker::Copy for OBJECT_ATTRIBUTES64 {}
impl ::core::clone::Clone for OBJECT_ATTRIBUTES64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OBJECT_ATTRIBUTES64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OBJECT_ATTRIBUTES64")
            .field("Length", &self.Length)
            .field("RootDirectory", &self.RootDirectory)
            .field("ObjectName", &self.ObjectName)
            .field("Attributes", &self.Attributes)
            .field("SecurityDescriptor", &self.SecurityDescriptor)
            .field("SecurityQualityOfService", &self.SecurityQualityOfService)
            .finish()
    }
}
impl ::core::cmp::PartialEq for OBJECT_ATTRIBUTES64 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
            && self.RootDirectory == other.RootDirectory
            && self.ObjectName == other.ObjectName
            && self.Attributes == other.Attributes
            && self.SecurityDescriptor == other.SecurityDescriptor
            && self.SecurityQualityOfService == other.SecurityQualityOfService
    }
}
impl ::core::cmp::Eq for OBJECT_ATTRIBUTES64 {}
impl FromIntoMemory for OBJECT_ATTRIBUTES64 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_Length = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_RootDirectory = <u64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_ObjectName = <u64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_Attributes = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_SecurityDescriptor = <u64 as FromIntoMemory>::from_bytes(&from[32..32 + 8]);
        let f_SecurityQualityOfService = <u64 as FromIntoMemory>::from_bytes(&from[40..40 + 8]);
        Self {
            Length: f_Length,
            RootDirectory: f_RootDirectory,
            ObjectName: f_ObjectName,
            Attributes: f_Attributes,
            SecurityDescriptor: f_SecurityDescriptor,
            SecurityQualityOfService: f_SecurityQualityOfService,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.Length, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.RootDirectory, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.ObjectName, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.Attributes, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.SecurityDescriptor, &mut into[32..32 + 8]);
        FromIntoMemory::into_bytes(self.SecurityQualityOfService, &mut into[40..40 + 8]);
    }
    fn size() -> usize {
        48
    }
}
pub const OBJ_CASE_INSENSITIVE: i32 = 64i32;
pub const OBJ_DONT_REPARSE: i32 = 4096i32;
pub const OBJ_EXCLUSIVE: i32 = 32i32;
pub const OBJ_FORCE_ACCESS_CHECK: i32 = 1024i32;
pub const OBJ_HANDLE_TAGBITS: i32 = 3i32;
pub const OBJ_IGNORE_IMPERSONATED_DEVICEMAP: i32 = 2048i32;
pub const OBJ_INHERIT: i32 = 2i32;
pub const OBJ_KERNEL_HANDLE: i32 = 512i32;
pub const OBJ_OPENIF: i32 = 128i32;
pub const OBJ_OPENLINK: i32 = 256i32;
pub const OBJ_PERMANENT: i32 = 16i32;
pub const OBJ_VALID_ATTRIBUTES: i32 = 8178i32;
pub struct PROCESSOR_NUMBER {
    pub Group: u16,
    pub Number: u8,
    pub Reserved: u8,
}
impl ::core::marker::Copy for PROCESSOR_NUMBER {}
impl ::core::clone::Clone for PROCESSOR_NUMBER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESSOR_NUMBER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESSOR_NUMBER")
            .field("Group", &self.Group)
            .field("Number", &self.Number)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PROCESSOR_NUMBER {
    fn eq(&self, other: &Self) -> bool {
        self.Group == other.Group && self.Number == other.Number && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for PROCESSOR_NUMBER {}
impl FromIntoMemory for PROCESSOR_NUMBER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_Group = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_Number = <u8 as FromIntoMemory>::from_bytes(&from[2..2 + 1]);
        let f_Reserved = <u8 as FromIntoMemory>::from_bytes(&from[3..3 + 1]);
        Self {
            Group: f_Group,
            Number: f_Number,
            Reserved: f_Reserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.Group, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.Number, &mut into[2..2 + 1]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[3..3 + 1]);
    }
    fn size() -> usize {
        4
    }
}
pub struct QUAD {
    pub Anonymous: QUAD_0,
}
impl ::core::marker::Copy for QUAD {}
impl ::core::clone::Clone for QUAD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUAD")
            .field("Anonymous", &self.Anonymous)
            .finish()
    }
}
impl ::core::cmp::PartialEq for QUAD {
    fn eq(&self, other: &Self) -> bool {
        self.Anonymous == other.Anonymous
    }
}
impl ::core::cmp::Eq for QUAD {}
impl FromIntoMemory for QUAD {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Anonymous = <QUAD_0 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        Self {
            Anonymous: f_Anonymous,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[0..0 + 8]);
    }
    fn size() -> usize {
        8
    }
}
pub struct QUAD_0 {
    data: [u8; 8],
}
impl ::core::default::Default for QUAD_0 {
    fn default() -> Self {
        Self { data: [0u8; 8] }
    }
}
impl ::core::marker::Copy for QUAD_0 {}
impl ::core::clone::Clone for QUAD_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUAD_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUAD_0").field("data", &self.data).finish()
    }
}
impl ::core::cmp::PartialEq for QUAD_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for QUAD_0 {}
impl FromIntoMemory for QUAD_0 {
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
pub struct RTL_BALANCED_NODE {
    pub Anonymous1: RTL_BALANCED_NODE_0,
    pub Anonymous2: RTL_BALANCED_NODE_1,
}
impl ::core::marker::Copy for RTL_BALANCED_NODE {}
impl ::core::clone::Clone for RTL_BALANCED_NODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RTL_BALANCED_NODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTL_BALANCED_NODE")
            .field("Anonymous1", &self.Anonymous1)
            .field("Anonymous2", &self.Anonymous2)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RTL_BALANCED_NODE {
    fn eq(&self, other: &Self) -> bool {
        self.Anonymous1 == other.Anonymous1 && self.Anonymous2 == other.Anonymous2
    }
}
impl ::core::cmp::Eq for RTL_BALANCED_NODE {}
impl FromIntoMemory for RTL_BALANCED_NODE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Anonymous1 = <RTL_BALANCED_NODE_0 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_Anonymous2 = <RTL_BALANCED_NODE_1 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            Anonymous1: f_Anonymous1,
            Anonymous2: f_Anonymous2,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Anonymous1, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.Anonymous2, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct RTL_BALANCED_NODE_0 {
    data: [u8; 8],
}
impl ::core::default::Default for RTL_BALANCED_NODE_0 {
    fn default() -> Self {
        Self { data: [0u8; 8] }
    }
}
impl ::core::marker::Copy for RTL_BALANCED_NODE_0 {}
impl ::core::clone::Clone for RTL_BALANCED_NODE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RTL_BALANCED_NODE_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTL_BALANCED_NODE_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RTL_BALANCED_NODE_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for RTL_BALANCED_NODE_0 {}
impl FromIntoMemory for RTL_BALANCED_NODE_0 {
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
pub struct RTL_BALANCED_NODE_0_0 {
    pub Left: MutPtr<RTL_BALANCED_NODE>,
    pub Right: MutPtr<RTL_BALANCED_NODE>,
}
impl ::core::marker::Copy for RTL_BALANCED_NODE_0_0 {}
impl ::core::clone::Clone for RTL_BALANCED_NODE_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RTL_BALANCED_NODE_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTL_BALANCED_NODE_0_0")
            .field("Left", &self.Left)
            .field("Right", &self.Right)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RTL_BALANCED_NODE_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Left == other.Left && self.Right == other.Right
    }
}
impl ::core::cmp::Eq for RTL_BALANCED_NODE_0_0 {}
impl FromIntoMemory for RTL_BALANCED_NODE_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Left = <MutPtr<RTL_BALANCED_NODE> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Right = <MutPtr<RTL_BALANCED_NODE> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            Left: f_Left,
            Right: f_Right,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Left, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Right, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct RTL_BALANCED_NODE_1 {
    data: [u8; 4],
}
impl ::core::default::Default for RTL_BALANCED_NODE_1 {
    fn default() -> Self {
        Self { data: [0u8; 4] }
    }
}
impl ::core::marker::Copy for RTL_BALANCED_NODE_1 {}
impl ::core::clone::Clone for RTL_BALANCED_NODE_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RTL_BALANCED_NODE_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTL_BALANCED_NODE_1")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RTL_BALANCED_NODE_1 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for RTL_BALANCED_NODE_1 {}
impl FromIntoMemory for RTL_BALANCED_NODE_1 {
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
pub const RTL_BALANCED_NODE_RESERVED_PARENT_MASK: u32 = 3u32;
pub struct SINGLE_LIST_ENTRY {
    pub Next: MutPtr<SINGLE_LIST_ENTRY>,
}
impl ::core::marker::Copy for SINGLE_LIST_ENTRY {}
impl ::core::clone::Clone for SINGLE_LIST_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SINGLE_LIST_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SINGLE_LIST_ENTRY")
            .field("Next", &self.Next)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SINGLE_LIST_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next
    }
}
impl ::core::cmp::Eq for SINGLE_LIST_ENTRY {}
impl FromIntoMemory for SINGLE_LIST_ENTRY {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_Next = <MutPtr<SINGLE_LIST_ENTRY> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self { Next: f_Next }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.Next, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct SINGLE_LIST_ENTRY32 {
    pub Next: u32,
}
impl ::core::marker::Copy for SINGLE_LIST_ENTRY32 {}
impl ::core::clone::Clone for SINGLE_LIST_ENTRY32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SINGLE_LIST_ENTRY32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SINGLE_LIST_ENTRY32")
            .field("Next", &self.Next)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SINGLE_LIST_ENTRY32 {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next
    }
}
impl ::core::cmp::Eq for SINGLE_LIST_ENTRY32 {}
impl FromIntoMemory for SINGLE_LIST_ENTRY32 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_Next = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self { Next: f_Next }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.Next, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct SLIST_ENTRY {
    pub Next: MutPtr<SLIST_ENTRY>,
}
impl ::core::marker::Copy for SLIST_ENTRY {}
impl ::core::clone::Clone for SLIST_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SLIST_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SLIST_ENTRY")
            .field("Next", &self.Next)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SLIST_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next
    }
}
impl ::core::cmp::Eq for SLIST_ENTRY {}
impl FromIntoMemory for SLIST_ENTRY {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_Next = <MutPtr<SLIST_ENTRY> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self { Next: f_Next }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.Next, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct SLIST_HEADER {
    data: [u8; 16],
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::default::Default for SLIST_HEADER {
    fn default() -> Self {
        Self { data: [0u8; 16] }
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for SLIST_HEADER {}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for SLIST_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for SLIST_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SLIST_HEADER")
            .field("data", &self.data)
            .finish()
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for SLIST_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for SLIST_HEADER {}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for SLIST_HEADER {
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
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct SLIST_HEADER_0 {
    pub Alignment: u64,
    pub Region: u64,
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for SLIST_HEADER_0 {}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for SLIST_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for SLIST_HEADER_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SLIST_HEADER_0")
            .field("Alignment", &self.Alignment)
            .field("Region", &self.Region)
            .finish()
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for SLIST_HEADER_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Alignment == other.Alignment && self.Region == other.Region
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for SLIST_HEADER_0 {}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for SLIST_HEADER_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_Alignment = <u64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_Region = <u64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        Self {
            Alignment: f_Alignment,
            Region: f_Region,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.Alignment, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.Region, &mut into[8..8 + 8]);
    }
    fn size() -> usize {
        16
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct SLIST_HEADER_1 {
    pub _bitfield1: u64,
    pub _bitfield2: u64,
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for SLIST_HEADER_1 {}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for SLIST_HEADER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for SLIST_HEADER_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SLIST_HEADER_1")
            .field("_bitfield1", &self._bitfield1)
            .field("_bitfield2", &self._bitfield2)
            .finish()
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for SLIST_HEADER_1 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield1 == other._bitfield1 && self._bitfield2 == other._bitfield2
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for SLIST_HEADER_1 {}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for SLIST_HEADER_1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f__bitfield1 = <u64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f__bitfield2 = <u64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        Self {
            _bitfield1: f__bitfield1,
            _bitfield2: f__bitfield2,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self._bitfield1, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self._bitfield2, &mut into[8..8 + 8]);
    }
    fn size() -> usize {
        16
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct SLIST_HEADER {
    data: [u8; 16],
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::default::Default for SLIST_HEADER {
    fn default() -> Self {
        Self { data: [0u8; 16] }
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for SLIST_HEADER {}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for SLIST_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for SLIST_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SLIST_HEADER")
            .field("data", &self.data)
            .finish()
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for SLIST_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for SLIST_HEADER {}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for SLIST_HEADER {
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
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct SLIST_HEADER_0 {
    pub Alignment: u64,
    pub Region: u64,
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for SLIST_HEADER_0 {}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for SLIST_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for SLIST_HEADER_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SLIST_HEADER_0")
            .field("Alignment", &self.Alignment)
            .field("Region", &self.Region)
            .finish()
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for SLIST_HEADER_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Alignment == other.Alignment && self.Region == other.Region
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for SLIST_HEADER_0 {}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for SLIST_HEADER_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_Alignment = <u64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_Region = <u64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        Self {
            Alignment: f_Alignment,
            Region: f_Region,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.Alignment, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.Region, &mut into[8..8 + 8]);
    }
    fn size() -> usize {
        16
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct SLIST_HEADER_1 {
    pub _bitfield1: u64,
    pub _bitfield2: u64,
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for SLIST_HEADER_1 {}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for SLIST_HEADER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for SLIST_HEADER_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SLIST_HEADER_1")
            .field("_bitfield1", &self._bitfield1)
            .field("_bitfield2", &self._bitfield2)
            .finish()
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for SLIST_HEADER_1 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield1 == other._bitfield1 && self._bitfield2 == other._bitfield2
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for SLIST_HEADER_1 {}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for SLIST_HEADER_1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f__bitfield1 = <u64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f__bitfield2 = <u64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        Self {
            _bitfield1: f__bitfield1,
            _bitfield2: f__bitfield2,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self._bitfield1, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self._bitfield2, &mut into[8..8 + 8]);
    }
    fn size() -> usize {
        16
    }
}
pub struct SLIST_HEADER {
    data: [u8; 8],
}
impl ::core::default::Default for SLIST_HEADER {
    fn default() -> Self {
        Self { data: [0u8; 8] }
    }
}
impl ::core::marker::Copy for SLIST_HEADER {}
impl ::core::clone::Clone for SLIST_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SLIST_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SLIST_HEADER")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SLIST_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for SLIST_HEADER {}
impl FromIntoMemory for SLIST_HEADER {
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
pub struct SLIST_HEADER_0 {
    pub Next: SINGLE_LIST_ENTRY,
    pub Depth: u16,
    pub CpuId: u16,
}
impl ::core::marker::Copy for SLIST_HEADER_0 {}
impl ::core::clone::Clone for SLIST_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SLIST_HEADER_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SLIST_HEADER_0")
            .field("Next", &self.Next)
            .field("Depth", &self.Depth)
            .field("CpuId", &self.CpuId)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SLIST_HEADER_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Depth == other.Depth && self.CpuId == other.CpuId
    }
}
impl ::core::cmp::Eq for SLIST_HEADER_0 {}
impl FromIntoMemory for SLIST_HEADER_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Next = <SINGLE_LIST_ENTRY as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Depth = <u16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_CpuId = <u16 as FromIntoMemory>::from_bytes(&from[6..6 + 2]);
        Self {
            Next: f_Next,
            Depth: f_Depth,
            CpuId: f_CpuId,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Next, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Depth, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.CpuId, &mut into[6..6 + 2]);
    }
    fn size() -> usize {
        8
    }
}
pub struct STRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: PSTR,
}
impl ::core::marker::Copy for STRING {}
impl ::core::clone::Clone for STRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STRING")
            .field("Length", &self.Length)
            .field("MaximumLength", &self.MaximumLength)
            .field("Buffer", &self.Buffer)
            .finish()
    }
}
impl ::core::cmp::PartialEq for STRING {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
            && self.MaximumLength == other.MaximumLength
            && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for STRING {}
impl FromIntoMemory for STRING {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Length = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_MaximumLength = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_Buffer = <PSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            Length: f_Length,
            MaximumLength: f_MaximumLength,
            Buffer: f_Buffer,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Length, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.MaximumLength, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.Buffer, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct STRING32 {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: u32,
}
impl ::core::marker::Copy for STRING32 {}
impl ::core::clone::Clone for STRING32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STRING32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STRING32")
            .field("Length", &self.Length)
            .field("MaximumLength", &self.MaximumLength)
            .field("Buffer", &self.Buffer)
            .finish()
    }
}
impl ::core::cmp::PartialEq for STRING32 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
            && self.MaximumLength == other.MaximumLength
            && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for STRING32 {}
impl FromIntoMemory for STRING32 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Length = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_MaximumLength = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_Buffer = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            Length: f_Length,
            MaximumLength: f_MaximumLength,
            Buffer: f_Buffer,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Length, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.MaximumLength, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.Buffer, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct STRING64 {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: u64,
}
impl ::core::marker::Copy for STRING64 {}
impl ::core::clone::Clone for STRING64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STRING64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STRING64")
            .field("Length", &self.Length)
            .field("MaximumLength", &self.MaximumLength)
            .field("Buffer", &self.Buffer)
            .finish()
    }
}
impl ::core::cmp::PartialEq for STRING64 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
            && self.MaximumLength == other.MaximumLength
            && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for STRING64 {}
impl FromIntoMemory for STRING64 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_Length = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_MaximumLength = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_Buffer = <u64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        Self {
            Length: f_Length,
            MaximumLength: f_MaximumLength,
            Buffer: f_Buffer,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.Length, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.MaximumLength, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.Buffer, &mut into[8..8 + 8]);
    }
    fn size() -> usize {
        16
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SUITE_TYPE(pub i32);
pub const SmallBusiness: SUITE_TYPE = SUITE_TYPE(0i32);
pub const Enterprise: SUITE_TYPE = SUITE_TYPE(1i32);
pub const BackOffice: SUITE_TYPE = SUITE_TYPE(2i32);
pub const CommunicationServer: SUITE_TYPE = SUITE_TYPE(3i32);
pub const TerminalServer: SUITE_TYPE = SUITE_TYPE(4i32);
pub const SmallBusinessRestricted: SUITE_TYPE = SUITE_TYPE(5i32);
pub const EmbeddedNT: SUITE_TYPE = SUITE_TYPE(6i32);
pub const DataCenter: SUITE_TYPE = SUITE_TYPE(7i32);
pub const SingleUserTS: SUITE_TYPE = SUITE_TYPE(8i32);
pub const Personal: SUITE_TYPE = SUITE_TYPE(9i32);
pub const Blade: SUITE_TYPE = SUITE_TYPE(10i32);
pub const EmbeddedRestricted: SUITE_TYPE = SUITE_TYPE(11i32);
pub const SecurityAppliance: SUITE_TYPE = SUITE_TYPE(12i32);
pub const StorageServer: SUITE_TYPE = SUITE_TYPE(13i32);
pub const ComputeServer: SUITE_TYPE = SUITE_TYPE(14i32);
pub const WHServer: SUITE_TYPE = SUITE_TYPE(15i32);
pub const PhoneNT: SUITE_TYPE = SUITE_TYPE(16i32);
pub const MultiUserTS: SUITE_TYPE = SUITE_TYPE(17i32);
pub const MaxSuiteType: SUITE_TYPE = SUITE_TYPE(18i32);
impl ::core::marker::Copy for SUITE_TYPE {}
impl ::core::clone::Clone for SUITE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SUITE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SUITE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SUITE_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for SUITE_TYPE {
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
pub struct TIMER_TYPE(pub i32);
pub const NotificationTimer: TIMER_TYPE = TIMER_TYPE(0i32);
pub const SynchronizationTimer: TIMER_TYPE = TIMER_TYPE(1i32);
impl ::core::marker::Copy for TIMER_TYPE {}
impl ::core::clone::Clone for TIMER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TIMER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TIMER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TIMER_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for TIMER_TYPE {
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
pub struct WAIT_TYPE(pub i32);
pub const WaitAll: WAIT_TYPE = WAIT_TYPE(0i32);
pub const WaitAny: WAIT_TYPE = WAIT_TYPE(1i32);
pub const WaitNotification: WAIT_TYPE = WAIT_TYPE(2i32);
pub const WaitDequeue: WAIT_TYPE = WAIT_TYPE(3i32);
pub const WaitDpc: WAIT_TYPE = WAIT_TYPE(4i32);
impl ::core::marker::Copy for WAIT_TYPE {}
impl ::core::clone::Clone for WAIT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WAIT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WAIT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WAIT_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for WAIT_TYPE {
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
pub struct WNF_STATE_NAME {
    pub Data: [u32; 2],
}
impl ::core::marker::Copy for WNF_STATE_NAME {}
impl ::core::clone::Clone for WNF_STATE_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WNF_STATE_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WNF_STATE_NAME")
            .field("Data", &self.Data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WNF_STATE_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.Data == other.Data
    }
}
impl ::core::cmp::Eq for WNF_STATE_NAME {}
impl FromIntoMemory for WNF_STATE_NAME {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Data = <[u32; 2] as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        Self { Data: f_Data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Data, &mut into[0..0 + 8]);
    }
    fn size() -> usize {
        8
    }
}
