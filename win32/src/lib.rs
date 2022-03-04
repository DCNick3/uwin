#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    unused_variables
)]

extern crate self as win32;
mod Windows;
pub mod core;
pub use Windows::*;
