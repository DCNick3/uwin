use core_mem::from_into_mem_impl_for_wrapper;
use core_mem::ptr::{ConstPtr, MutPtr, PtrRepr};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct AnsiChar(u8);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct WideChar(u16);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct AnsiString {
    slice: Vec<AnsiChar>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct WideString {
    slice: Vec<WideChar>,
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
}
impl PCSTR {
    pub const fn new(raw_ptr: PtrRepr) -> Self {
        Self(ConstPtr::new(raw_ptr))
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
