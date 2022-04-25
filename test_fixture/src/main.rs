extern crate core;

mod r#impl;

use crate::r#impl::*;
use core_mem::ptr::PtrRepr;
use core_memmgr::{AddressRange, MemoryManager};
use core_str::heap_helper::AnsiStringHeapBox;
use core_str::AnsiString;
use recompiler::memory_image::{MemoryImageItem, Protection};
use rusty_x86_runtime::{CpuContext, ExtendedContext, PROGRAM_IMAGE};
use std::sync::{Arc, Mutex};
use tracing::trace;
use tracing_subscriber::fmt::format;
use tracing_subscriber::layer::SubscriberExt;
use win32::core::Win32Context;
use win32_heapmgr::HeapMgr;
use win32_virtmem::VirtualMemoryManager;

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
    let mut memory_mgr = MemoryManager::new().expect("Mapping the base region");

    let memory_ctx = unsafe { memory_mgr.memory_context() };

    for (_, (_, info)) in PROGRAM_IMAGE.modules.iter() {
        let res = memory_mgr
            .reserve_static(AddressRange::new(info.base_addr, info.image_size))
            .expect("Reserving address space for a module");
        assert_eq!(res.start, info.base_addr);
        assert!(res.size >= info.image_size);
    }

    for item in PROGRAM_IMAGE.memory.iter() {
        map_item(&mut memory_mgr, item).expect("Mapping program memory")
    }

    let entry = PROGRAM_IMAGE.exe_entrypoint;

    let mut context = ExtendedContext {
        cpu: CpuContext::default(),
        win32: Win32Context::new(),
        unwind_reason: None,
    };

    let tlb = memory_mgr
        .reserve_and_commit_dynamic(0x10000, Protection::READ_WRITE)
        .expect("Mapping TLB");

    let stack_size = 0x400000; // 4 MiB
    let stack = memory_mgr
        .reserve_and_commit_dynamic(stack_size, Protection::READ_WRITE)
        .expect("Mapping stack");
    // set ESP to the bottom of the stack
    // TODO: put some return value to the stack when calling
    // probably could be done through a generalized way of callbacks implementation
    // for now the zero is left there (hopefully it will be zero...)
    context.cpu.gp_regs[4] = stack.end() - 4;
    context.cpu.fs_base = tlb.start;

    let memory_mgr = Arc::new(Mutex::new(memory_mgr));

    let mut heap_mgr = HeapMgr::new(memory_mgr.clone());

    let process_heap_handle = heap_mgr.create(0, 0);

    let process_ctx = ProcessContext {
        memory_manager: memory_mgr.clone(),
        process_heap: heap_mgr.get_heap(process_heap_handle),
        memory_ctx,
        ansi_encoding: encoding_rs::WINDOWS_1251,
    };

    let command_line_ansi = AnsiStringHeapBox::new(
        process_ctx.memory_ctx,
        process_ctx.process_heap.clone(),
        &AnsiString::from_ascii("C:\\MAIN.EXE"),
    )
    .expect("Allocating command_line_ansi");

    // NOTE: this is in OEM encoding, not ANSI
    // for now, I don't care, but this... Might be problematic
    let environment_strings_oem = vec![0u8; 2];

    // ===

    context.win32.insert(Arc::new(WindowsAndMessaging {
        process_ctx: process_ctx.clone(),
    })
        as Arc<dyn win32::Win32::UI::WindowsAndMessaging::Api>);

    context.win32.insert(Arc::new(SystemInformation {
        process_ctx: process_ctx.clone(),
    })
        as Arc<dyn win32::Win32::System::SystemInformation::Api>);

    context.win32.insert(Arc::new(Memory {
        process_ctx: process_ctx.clone(),
        virtmem_mgr: VirtualMemoryManager::new(memory_mgr.clone()),
        heap_mgr: Mutex::new(heap_mgr),
        process_heap_handle,
    }) as Arc<dyn win32::Win32::System::Memory::Api>);

    context.win32.insert(Arc::new(LibraryLoader {
        process_ctx: process_ctx.clone(),
    }) as Arc<dyn win32::Win32::System::LibraryLoader::Api>);

    context.win32.insert(Arc::new(Threading {
        process_ctx: process_ctx.clone(),
    }) as Arc<dyn win32::Win32::System::Threading::Api>);

    context.win32.insert(Arc::new(Console {
        process_ctx: process_ctx.clone(),
    }) as Arc<dyn win32::Win32::System::Console::Api>);

    context.win32.insert(Arc::new(WindowsProgramming {
        process_ctx: process_ctx.clone(),
    })
        as Arc<dyn win32::Win32::System::WindowsProgramming::Api>);

    context.win32.insert(Arc::new(Environment {
        process_ctx: process_ctx.clone(),
        command_line_ansi,
        environment_strings_oem,
    }) as Arc<dyn win32::Win32::System::Environment::Api>);

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
