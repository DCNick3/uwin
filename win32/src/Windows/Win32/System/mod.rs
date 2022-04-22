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
pub mod Console;
pub mod Diagnostics;
pub mod Environment;
pub mod IO;
pub mod Kernel;
pub mod LibraryLoader;
pub mod Memory;
pub mod SystemInformation;
pub mod Threading;
pub mod WindowsProgramming;
