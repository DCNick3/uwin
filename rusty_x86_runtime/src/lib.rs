mod callback_token;
mod interp;
mod thunk_helper;
mod thunks;

pub use crate::callback_token::RustyX86CallbackToken;
pub use core_abi::stdcall::StdCalleeHelper;
pub use core_mem::ctx::FlatMemoryCtx;
pub use rusty_x86::types::CpuContext;
pub(crate) use thunk_helper::{thunk_helper, MyCallsite};

use crate::interp::run_interp;
use core_abi::context::{Context, Executor};
use core_abi::unwind_token::UnwindReason;
use core_mem::ctx::DefaultMemoryCtx;
use core_mem::ptr::PtrRepr;
use lazy_static::lazy_static;
use recompiler::LoadedProcessImage;
use std::collections::HashSet;
use std::ffi::c_void;
use std::panic::AssertUnwindSafe;
use tracing::warn;
use win32::core::Win32Context;

// here we do ABI some trickery
// rusty-x86-generated code will use only the contents of CpuContext (the first field)
// but runtime will store some of its information after it
// this should be safe thanks to repr(C)
#[repr(C)]
pub struct ExtendedContext {
    // !!! Should be the first struct element
    pub cpu: CpuContext,
    pub win32: Win32Context,
    pub unwind_reason: Option<UnwindReason>,
    pub interpreted_blocks: HashSet<PtrRepr>,
}

impl Context for ExtendedContext {
    type CpuContext = CpuContext;

    fn cpu_context(&self) -> &Self::CpuContext {
        &self.cpu
    }

    fn cpu_context_mut(&mut self) -> &mut Self::CpuContext {
        &mut self.cpu
    }

    fn get_unwind_reason(&self) -> Option<UnwindReason> {
        self.unwind_reason.clone()
    }
}

extern "C" {
    static uwin_serialized_process_image: c_void;
    static uwin_serialized_process_image_size: u32;

    #[allow(improper_ctypes)] // I know I am doing dark magic, that's ok
    fn uwin_indirect_bb_call(context: &mut ExtendedContext, memory: FlatMemoryCtx, eip: u32)
        -> u32;

    #[allow(improper_ctypes)]
    fn uwin_find_thunk(
        thunk_index: u32,
    ) -> Option<unsafe extern "C" fn(&mut ExtendedContext, memory: FlatMemoryCtx) -> u32>;
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

#[no_mangle]
unsafe extern "C" fn uwin_missing_bb(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
    eip: u32,
) -> u32 {
    warn!(
        "Called a missing bb @ ip = {:#010x}; switching to an interpreter",
        eip
    );

    let res = std::panic::catch_unwind(AssertUnwindSafe(|| run_interp(context, memory, eip)));
    match res {
        Ok(r) => r,
        Err(_) => {
            eprintln!("Interpretation panicked");
            std::process::abort();
        }
    }
}

fn execute_recompiled_code(context: &mut ExtendedContext, memory: FlatMemoryCtx, eip: u32) -> u32 {
    // SAFETY: TODO??
    // It seems that safety guarantees are pushed onto the LLVM-generated code

    // don't use recompiled code, just run it in interp
    // TODO: make this configurable (compile time?)
    // unsafe { run_interp(context, memory, eip) }

    unsafe { uwin_indirect_bb_call(context, memory, eip) }
}

type DynRustyExecutor = dyn Executor<
    Context = ExtendedContext,
    CpuContext = CpuContext,
    MemoryContext = DefaultMemoryCtx,
>;

// TODO: implement more executors (like pure interpreter ones and instrumented interpreters)
pub struct RecompiledExecutor {}

impl Executor for RecompiledExecutor {
    type Context = ExtendedContext;
    type CpuContext = CpuContext;
    type MemoryContext = DefaultMemoryCtx;

    fn execute_recompiled_code(
        &self,
        context: &mut Self::Context,
        memory: Self::MemoryContext,
        eip: u32,
    ) -> u32 {
        execute_recompiled_code(context, memory, eip)
    }
}
