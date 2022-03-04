pub mod prelude;

#[allow(unused)]
use prelude::*;

/// An error code value returned by most COM functions.
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
#[must_use]
#[allow(non_camel_case_types)]
pub struct HRESULT(pub i32);

// TODO: stubs for pointers
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub struct PSTR(pub u32);
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub struct PWSTR(pub u32);
