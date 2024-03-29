extern crate core;

mod com_stubs;
mod com_stubs_params;
mod dll_exports;
mod error;
mod loader;
mod pe_file;
#[cfg(feature = "recompilation")]
mod recompile;
mod stubs;
mod stubs_list;
mod thunk_id_allocator;

pub use loader::{load_process_image, LoadedProcessImage, OwnedImport};
pub use memory_image;
pub use pe_file::{LoadedPeInfo, ModuleExports, PeFile};
#[cfg(feature = "recompilation")]
pub use recompile::{find_basic_blocks, recompile_image};
#[cfg(feature = "recompilation")]
pub use rusty_x86::inkwell;
pub use stubs::make_dll_stub;
pub use thunk_id_allocator::Thunk;
