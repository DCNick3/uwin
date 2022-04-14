pub use core_abi::stdcall::StdCallHelper;
pub use core_mem::ctx::FlatMemoryCtx;
use lazy_static::lazy_static;
use recompiler::LoadedProcessImage;
pub use rusty_x86::types::CpuContext;
use std::ffi::c_void;
use win32::core::Win32Context;

// here we do ABI some trickery
// rusty-x86-generated code will use only the contents of CpuContext (the first field)
// but runtime will store some of its information after it
// this should be safe thanks to repr(C)
#[repr(C)]
pub struct ExtendedContext {
    pub cpu: CpuContext,
    pub win32: Win32Context,
}

extern "C" {
    static uwin_serialized_process_image: c_void;
    static uwin_serialized_process_image_size: u32;

    #[allow(improper_ctypes)] // I know I am doing dark magic, that's ok
    fn uwin_indirect_bb_call(context: &mut ExtendedContext, memory: FlatMemoryCtx, eip: u32);
}

fn get_process_image_data() -> &'static [u8] {
    // SAFETY: hopefully the codegen will follow the assumptions
    unsafe {
        std::slice::from_raw_parts(
            std::mem::transmute::<_, *const u8>(&uwin_serialized_process_image),
            uwin_serialized_process_image_size as usize,
        )
    }
}

fn get_process_image() -> LoadedProcessImage {
    rmp_serde::from_slice(get_process_image_data())
        .expect("Failed to deserialize embedded process image")
}

lazy_static! {
    pub static ref PROGRAM_IMAGE: LoadedProcessImage = get_process_image();
}

pub fn execute_recompiled_code(context: &mut ExtendedContext, memory: FlatMemoryCtx, eip: u32) {
    // SAFETY: TODO??
    // It seems that safety guarantees are pushed onto the LLVM-generated code
    unsafe { uwin_indirect_bb_call(context, memory, eip) }
}
