pub mod prelude;

use anymap::any::{Any, IntoBox};
use anymap::AnyMap;
#[allow(unused)]
use prelude::*;
use std::sync::Arc;

/// An error code value returned by most COM functions.
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
#[must_use]
#[allow(non_camel_case_types)]
pub struct HRESULT(pub i32);

// TODO: stubs for pointers
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PSTR(pub MutPtr<u8>);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PCSTR(pub ConstPtr<u8>);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PWSTR(pub MutPtr<u16>);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PCWSTR(pub ConstPtr<u8>);

impl PSTR {
    pub const fn new(value: PtrRepr) -> Self {
        Self(MutPtr::new(value))
    }
}
impl PCSTR {
    pub const fn new(value: PtrRepr) -> Self {
        Self(ConstPtr::new(value))
    }
}

impl PWSTR {
    pub const fn new(value: PtrRepr) -> Self {
        Self(MutPtr::new(value))
    }
}
impl PCWSTR {
    pub const fn new(value: PtrRepr) -> Self {
        Self(ConstPtr::new(value))
    }
}

impl FromIntoMemory for PCSTR {
    fn try_from_bytes(from: &[u8]) -> Self {
        Self(ConstPtr::try_from_bytes(from))
    }

    fn try_into_bytes(self, into: &mut [u8]) {
        self.0.try_into_bytes(into)
    }

    fn size() -> usize {
        ConstPtr::<u8>::size()
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl GUID {
    /// Creates a `GUID` represented by the all-zero byte-pattern.
    pub const fn zeroed() -> Self {
        Self {
            data1: 0,
            data2: 0,
            data3: 0,
            data4: [0, 0, 0, 0, 0, 0, 0, 0],
        }
    }

    /// Creates a `GUID` with the given constant values.
    pub const fn from_values(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Self {
        Self {
            data1,
            data2,
            data3,
            data4,
        }
    }

    /// Creates a `GUID` from a `u128` value.
    pub const fn from_u128(uuid: u128) -> Self {
        Self {
            data1: (uuid >> 96) as u32,
            data2: (uuid >> 80 & 0xffff) as u16,
            data3: (uuid >> 64 & 0xffff) as u16,
            data4: (uuid as u64).to_be_bytes(),
        }
    }
}

impl core::fmt::Debug for GUID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:08X?}-{:04X?}-{:04X?}-{:02X?}{:02X?}-{:02X?}{:02X?}{:02X?}{:02X?}{:02X?}{:02X?}",
            self.data1,
            self.data2,
            self.data3,
            self.data4[0],
            self.data4[1],
            self.data4[2],
            self.data4[3],
            self.data4[4],
            self.data4[5],
            self.data4[6],
            self.data4[7]
        )
    }
}

pub struct Win32Context(anymap::AnyMap);

impl Win32Context {
    pub fn new() -> Self {
        Self(AnyMap::new())
    }

    pub fn get<T: ?Sized>(&self) -> &T
    where
        Arc<T>: IntoBox<dyn Any>,
    {
        self.0
            .get::<Arc<T>>()
            .unwrap_or_else(|| {
                panic!(
                    "Could not find a registered implementation for {}",
                    std::any::type_name::<T>()
                )
            })
            .as_ref()
    }

    pub fn insert<T: ?Sized>(&mut self, value: Arc<T>)
    where
        Arc<T>: IntoBox<dyn Any>,
    {
        if !self.0.insert(value).is_none() {
            panic!(
                "Multiple Api implementations registered for {}",
                std::any::type_name::<T>()
            )
        };
    }
}
