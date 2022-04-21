extern crate core;

mod r#impl;

use crate::r#impl::*;
use core_mem::ptr::PtrRepr;
use core_mem::thread_ctx::set_thread_ctx;
use core_memmgr::{AddressRange, MemoryManager};
use recompiler::memory_image::{MemoryImageItem, Protection};
use rusty_x86_runtime::{CpuContext, ExtendedContext, PROGRAM_IMAGE};
use std::sync::Arc;
use tracing::trace;
use tracing_subscriber::fmt::format;
use tracing_subscriber::layer::SubscriberExt;
use win32::core::Win32Context;

fn map_item(mgr: &mut MemoryManager, item: &MemoryImageItem) -> core_memmgr::Result<()> {
    let range = AddressRange::new(item.addr, item.data.len() as PtrRepr);

    assert_eq!(range.start, item.addr);
    assert_eq!(range.size, item.data.len() as PtrRepr);

    mgr.commit(range, Protection::READ_WRITE)?;

    unsafe {
        let ptr = mgr.memory_context().to_native_ptr(range.start);
        std::ptr::copy_nonoverlapping(item.data.as_ptr(), ptr, item.data.len())
    };

    mgr.protect(range, item.protection)?;

    Ok(())
}

fn main_impl() {
    let mut mgr = MemoryManager::new().expect("Mapping the base region");

    let memory_ctx = unsafe { mgr.memory_context() };
    set_thread_ctx(memory_ctx);

    for (_, (_, info)) in PROGRAM_IMAGE.modules.iter() {
        let res = mgr
            .reserve_static(AddressRange::new(info.base_addr, info.image_size))
            .expect("Reserving address space for a module");
        assert_eq!(res.start, info.base_addr);
        assert!(res.size >= info.image_size);
    }

    for item in PROGRAM_IMAGE.memory.iter() {
        map_item(&mut mgr, item).expect("Mapping program memory")
    }

    let entry = PROGRAM_IMAGE.exe_entrypoint;

    let mut context = ExtendedContext {
        cpu: CpuContext::default(),
        win32: Win32Context::new(),
        unwind_reason: None,
    };

    context.win32.insert(
        Arc::new(WindowsAndMessaging {}) as Arc<dyn win32::Win32::UI::WindowsAndMessaging::Api>
    );
    context.win32.insert(
        Arc::new(SystemInformation {}) as Arc<dyn win32::Win32::System::SystemInformation::Api>
    );
    context
        .win32
        .insert(Arc::new(Memory {}) as Arc<dyn win32::Win32::System::Memory::Api>);
    context
        .win32
        .insert(Arc::new(LibraryLoader {}) as Arc<dyn win32::Win32::System::LibraryLoader::Api>);
    context
        .win32
        .insert(Arc::new(Threading {}) as Arc<dyn win32::Win32::System::Threading::Api>);

    let tlb = mgr
        .reserve_and_commit_dynamic(0x10000, Protection::READ_WRITE)
        .expect("Mapping TLB");

    let stack_size = 0x400000; // 4 MiB
    let stack = mgr
        .reserve_and_commit_dynamic(stack_size, Protection::READ_WRITE)
        .expect("Mapping stack");
    // set ESP to the bottom of the stack
    // TODO: put some return value to the stack when calling
    // probably could be done through a generalized way of callbacks implementation
    // for now the zero is left there (hopefully it will be zero...)
    context.cpu.gp_regs[4] = stack.end() - 4;
    context.cpu.fs_base = tlb.start;

    let res = rusty_x86_runtime::execute_recompiled_code(&mut context, memory_ctx, entry);
    trace!("execute_recompiled_code returned 0x{:08x}", res);

    if res == 0 {
        if let Some(reason) = context.unwind_reason {
            trace!("The recompiled stack was unwound, reason = {reason:?}")
        } else {
            panic!(
                "Zero continuation address without an unwind reason set. Sounds like an ABI crime"
            )
        }
    } else {
        assert!(
            context.unwind_reason.is_none(),
            "Non-zero address return with some unwind reason... Sounds like an ABI crime"
        );
        todo!("Re-entering the code execution after yielding") // Not sure of all the consequences of just doing it
    }
}

fn main() {
    tracing::subscriber::set_global_default(
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::Layer::new().event_format(format().compact()))
            .with(tracing_tracy::TracyLayer::new()),
    )
    .expect("Setting up logging");

    main_impl();
}
