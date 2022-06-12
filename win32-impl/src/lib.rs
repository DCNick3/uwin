#![allow(non_snake_case)]

mod Win32;

use core_heap::Heap;
use core_mem::ctx::DefaultMemoryCtx;
use core_memmgr::MemoryManager;
use encoding_rs::Encoding;
use std::sync::{Arc, Mutex};

pub use Win32::Console::*;
pub use Win32::DirectDraw::*;
pub use Win32::Environment::*;
pub use Win32::FileSystem::*;
pub use Win32::Foundation::*;
pub use Win32::Gdi::*;
pub use Win32::Globalization::*;
pub use Win32::LibraryLoader::*;
pub use Win32::Memory::*;
pub use Win32::SystemInformation::*;
pub use Win32::Threading::*;
pub use Win32::Time::*;
pub use Win32::WindowsAndMessaging::*;
pub use Win32::WindowsProgramming::*;

#[derive(Clone)]
pub struct ProcessContext {
    pub memory_manager: Arc<Mutex<MemoryManager>>,
    pub process_heap: Arc<Mutex<Heap>>,
    pub memory_ctx: DefaultMemoryCtx,
    pub ansi_encoding: &'static Encoding,
}
