mod thunks;

pub use core_abi::stdcall::StdCallHelper;
use core_abi::unwind_token::UnwindReason;
pub use core_mem::ctx::FlatMemoryCtx;
use core_mem::ptr::PtrRepr;
use lazy_static::lazy_static;
use recompiler::LoadedProcessImage;
pub use rusty_x86::types::CpuContext;
use std::ffi::c_void;
use tracing::subscriber::Interest;
use tracing::{Callsite, Metadata};
use win32::core::Win32Context;

// here we do ABI some trickery
// rusty-x86-generated code will use only the contents of CpuContext (the first field)
// but runtime will store some of its information after it
// this should be safe thanks to repr(C)
#[repr(C)]
pub struct ExtendedContext {
    pub cpu: CpuContext,
    pub win32: Win32Context,
    pub unwind_reason: Option<UnwindReason>,
}

extern "C" {
    static uwin_serialized_process_image: c_void;
    static uwin_serialized_process_image_size: u32;

    #[allow(improper_ctypes)] // I know I am doing dark magic, that's ok
    fn uwin_indirect_bb_call(context: &mut ExtendedContext, memory: FlatMemoryCtx, eip: u32)
        -> u32;
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
extern "C" fn uwin_missing_bb(
    _context: &mut ExtendedContext,
    _memory: FlatMemoryCtx,
    eip: u32,
) -> u32 {
    eprintln!("Called a missing bb @ ip = {:#010x}; aborting", eip);
    std::process::abort();
}

pub fn execute_recompiled_code(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
    eip: u32,
) -> u32 {
    // SAFETY: TODO??
    // It seems that safety guarantees are pushed onto the LLVM-generated code
    unsafe { uwin_indirect_bb_call(context, memory, eip) }
}

struct MyCallsite {
    metadata: Metadata<'static>,
}

impl MyCallsite {
    const fn new(identifier: tracing::callsite::Identifier) -> Self {
        Self {
            metadata: Metadata::new(
                "",
                "thunk",
                tracing::Level::TRACE,
                None,
                None,
                None,
                tracing::field::FieldSet::new(&["message"], identifier),
                tracing::metadata::Kind::EVENT,
            ),
        }
    }

    const fn new_span(identifier: tracing::callsite::Identifier, span_name: &'static str) -> Self {
        Self {
            metadata: Metadata::new(
                span_name,
                "thunk",
                tracing::Level::TRACE,
                None,
                None,
                None,
                tracing::field::FieldSet::new(&[], identifier),
                tracing::metadata::Kind::EVENT,
            ),
        }
    }
}

impl Callsite for MyCallsite {
    fn set_interest(&self, _interest: Interest) {
        // поебать
    }

    fn metadata(&self) -> &Metadata<'_> {
        &self.metadata
    }
}

pub(crate) fn thunk_helper(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
    span_callsite: &'static MyCallsite,
    body: fn(
        StdCallHelper<FlatMemoryCtx, rusty_x86::types::CpuContext>,
        &win32::core::Win32Context,
        bool, // trace_enabled
        &'static MyCallsite,
    ) -> PtrRepr,
) -> PtrRepr {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let call = StdCallHelper::new(memory, &mut context.cpu, &mut context.unwind_reason);

        let span = tracing::Span::new(
            span_callsite.metadata(),
            &span_callsite.metadata().fields().value_set(&[]),
        );

        // let span = tracing::trace_span!("name"); // TODO
        let _enter = span.enter();
        // tracing::trace!("ret_addr = {:#010x}", call.return_address());

        let trace_event_enabled = tracing::event_enabled!(tracing::Level::TRACE);

        static CALLSITE: MyCallsite = MyCallsite::new(tracing::callsite::Identifier(&CALLSITE));

        let fields = CALLSITE.metadata().fields();

        tracing::event::Event::dispatch(
            CALLSITE.metadata(),
            &fields.value_set(&[(
                &unsafe { fields.iter().next().unwrap_unchecked() },
                Some(&format_args!("ret_addr = {:#010x}", call.return_address())
                    as &dyn tracing::Value),
            )]),
        );

        body(call, &context.win32, trace_event_enabled, &CALLSITE)
    }));
    match result {
        Ok(ret) => ret,
        Err(_) => {
            eprintln!("Caught a panic in native code. Whoops, aborting..");
            std::process::abort();
        }
    }
}
