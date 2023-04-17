pub mod heap_helper;

use core_mem::ctx::MemoryCtx;
use core_mem::from_into_mem_impl_for_wrapper;
use core_mem::ptr::{ConstPtr, MutPtr, PtrRepr};
use encoding_rs::Encoding;
use std::borrow::Cow;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct AnsiChar(u8);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct WideChar(u16);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct AnsiString {
    vec: Vec<AnsiChar>,
}

impl AnsiString {
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }

    pub fn from_ascii(ascii_str: &str) -> Self {
        assert!(ascii_str.chars().all(|c| c.is_ascii()));

        Self {
            vec: ascii_str.as_bytes().iter().map(|&b| AnsiChar(b)).collect(),
        }
    }

    #[allow(clippy::unsound_collection_transmute)] // eh? u8 and AnsiChar are the same thing!
    pub fn from_utf8(utf8_str: &str, ansi_encoding: &'static Encoding) -> Self {
        let (res, _, _) = ansi_encoding.encode(utf8_str);
        let vec = res.into_owned();
        let vec = unsafe {
            std::mem::transmute(vec) /* this works, right? */
        };
        Self { vec }
    }

    pub fn as_utf8(&self, ansi_encoding: &'static Encoding) -> Cow<str> {
        let vec = unsafe { std::mem::transmute(self.vec.as_slice()) };
        let (res, _, _) = ansi_encoding.decode(vec);
        res
    }

    pub fn push(&mut self, v: AnsiChar) {
        self.vec.push(v)
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<AnsiChar> {
        self.vec.iter()
    }
}

impl Default for AnsiString {
    fn default() -> Self {
        AnsiString::new()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct WideString {
    vec: Vec<WideChar>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PSTR(pub MutPtr<u8>);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PCSTR(pub ConstPtr<u8>);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PWSTR(pub MutPtr<u16>);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PCWSTR(pub ConstPtr<u16>);

impl PSTR {
    pub const fn new(raw_ptr: PtrRepr) -> Self {
        Self(MutPtr::new(raw_ptr))
    }

    pub fn read_with(&self, ctx: impl MemoryCtx) -> AnsiString {
        PCSTR::new(self.0.repr()).read_with(ctx)
    }

    pub fn write_with(&self, ctx: impl MemoryCtx, buffer_size: PtrRepr, value: &AnsiString) {
        let mut ptr = self.0;

        // we don't want buffer overflows, right? (NUL character is accounted for)
        assert!(buffer_size as usize >= value.len() + 1);

        for c in value.iter() {
            ptr.write_with(ctx, c.0);

            ptr = ptr.offset(1);
        }

        ptr.write_with(ctx, 0u8);
    }
}
impl PCSTR {
    pub const fn new(raw_ptr: PtrRepr) -> Self {
        Self(ConstPtr::new(raw_ptr))
    }

    pub fn read_with(&self, ctx: impl MemoryCtx) -> AnsiString {
        let mut res = AnsiString::new();
        let mut ptr = self.0;

        loop {
            let c = ptr.read_with(ctx);
            ptr = ptr.offset(1);

            if c == 0 {
                break;
            }

            res.push(AnsiChar(c));
        }

        res
    }

    pub fn to_utf8(&self, ctx: impl MemoryCtx, ansi_encoding: &'static Encoding) -> String {
        let mut decoder = ansi_encoding.new_decoder();
        let mut res = String::new();
        let mut ptr = self.0;

        res.reserve(16);

        loop {
            if res.len() == res.capacity() {
                res.reserve(res.len());
            }

            let c = ptr.read_with(ctx);
            ptr = ptr.offset(1);

            if c == 0 {
                break;
            }

            let (result, _) = decoder.decode_to_string_without_replacement(&[c], &mut res, false);
            assert_eq!(result, encoding_rs::DecoderResult::InputEmpty);
        }

        let (result, _) = decoder.decode_to_string_without_replacement(&[], &mut res, true);
        assert_eq!(result, encoding_rs::DecoderResult::InputEmpty);

        res
    }
}
impl PWSTR {
    pub const fn new(raw_ptr: PtrRepr) -> Self {
        Self(MutPtr::new(raw_ptr))
    }
}
impl PCWSTR {
    pub const fn new(raw_ptr: PtrRepr) -> Self {
        Self(ConstPtr::new(raw_ptr))
    }
}

from_into_mem_impl_for_wrapper!(PSTR, MutPtr<u8>);
from_into_mem_impl_for_wrapper!(PCSTR, ConstPtr<u8>);
from_into_mem_impl_for_wrapper!(PWSTR, MutPtr<u16>);
from_into_mem_impl_for_wrapper!(PCWSTR, ConstPtr<u16>);
