extern crate core;

mod error;
mod loader;
mod pe_file;
#[cfg(feature = "recompilation")]
mod recompile;
mod stubs;

pub use loader::{load_process_image, LoadedProcessImage};
pub use pe_file::{LoadedPeInfo, PeFile};
#[cfg(feature = "recompilation")]
pub use recompile::{find_basic_blocks, recompile_image};
#[cfg(feature = "recompilation")]
pub use rusty_x86::inkwell;
pub use stubs::make_dll_stub;
