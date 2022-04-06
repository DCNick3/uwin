use lazy_static::lazy_static;
use recompiler::LoadedProcessImage;
pub use rusty_x86::types::CpuContext;
use std::ffi::c_void;
pub use win32_mem::ctx::FlatMemoryCtx;

extern "C" {
    static uwin_serialized_process_image: c_void;
    static uwin_serialized_process_image_size: u32;
    fn uwin_indirect_bb_call(context: &mut CpuContext, memory: FlatMemoryCtx, eip: u32);
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

pub fn execute_recompiled_code(context: &mut CpuContext, memory: FlatMemoryCtx, eip: u32) {
    // SAFETY: TODO??
    unsafe { uwin_indirect_bb_call(context, memory, eip) }
}
