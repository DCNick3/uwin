extern crate core;

use arcstr::ArcStr;
use core_abi::callback_token::StdcallCallbackToken;
use core_abi::stdcall_fn_ptr::StdCallFnPtr;
use core_abi::unwind_token::UnwindReason;
use core_fs::path::{Drive, Root};
use core_mem::ptr::{PtrDiffRepr, PtrRepr};
use core_memmgr::{AddressRange, MemoryManager};
use core_str::heap_helper::AnsiStringHeapBox;
use core_str::AnsiString;
use core_time::TimeProvider;
use itertools::Itertools;
use recompiler::memory_image::{MemoryImageItem, Protection};
use rusty_x86_runtime::{CpuContext, ExtendedContext, PROGRAM_IMAGE};
#[allow(unused)]
use rusty_x86_runtime::{InterpretedExecutor, RecompiledExecutor};
use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::panic::AssertUnwindSafe;
use std::sync::{Arc, Mutex};
use tracing::{debug, info};
use tracing_subscriber::fmt::format;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use win32::core::Win32Context;
use win32::Win32::Foundation::{BOOL, HINSTANCE};
use win32::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use win32_atoms::AtomTable;
use win32_fs::WindowsFsManager;
use win32_heapmgr::HeapMgr;
use win32_impl::{
    Console, DirectDrawApi, Environment, FileSystem, Foundation, Gdi, Globalization, LibraryLoader,
    Memory, ProcessContext, SystemInformation, Threading, Time, WindowsAndMessaging,
    WindowsProgramming,
};
use win32_io::IoDispatcher;
use win32_kobj::{KernelHandleTable, KernelObject};
use win32_message_queue::MessageQueueRegistry;
use win32_module_table::ModuleTable;
use win32_virtmem::VirtualMemoryManager;
use win32_windows::{ClassRegistry, WindowsRegistry};
use win32_wobj::WindowsHandleTable;

fn map_item(mgr: &mut MemoryManager, item: &MemoryImageItem) -> core_memmgr::Result<()> {
    if item.data.is_empty() {
        // no need to do anything in this case
        return Ok(());
    }

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

type DllEntrypointFnPtr = StdCallFnPtr<(HINSTANCE, u32, PtrRepr), BOOL>;
type EntrypointFnPtr = StdCallFnPtr<(), ()>;

fn main_impl() {
    let mut memory_mgr = MemoryManager::new().expect("Mapping the base region");

    let memory_ctx = unsafe { memory_mgr.memory_context() };

    let mut module_map = String::new();

    for (_, (pe, info)) in PROGRAM_IMAGE
        .modules
        .iter()
        .sorted_by_key(|(_, (_, info))| info.base_addr)
    {
        let res = memory_mgr
            .reserve_static(AddressRange::new(info.base_addr, info.image_size))
            .expect("Reserving address space for a module");
        assert_eq!(res.start, info.base_addr);
        assert!(res.size >= info.image_size);

        writeln!(
            module_map,
            "{:#010x}-{:#010x} ({:#010x}): {}",
            res.start,
            res.end(),
            res.size,
            pe.name()
        )
        .unwrap();
    }

    debug!("Program module map:\n{}", module_map);
    debug!("Program memory map:\n{}", PROGRAM_IMAGE.memory.map());

    for item in PROGRAM_IMAGE.memory.iter() {
        map_item(&mut memory_mgr, item).expect("Mapping program memory")
    }

    // InterpretedExecutor suffers from lacking tail call optimization in
    let executor = RecompiledExecutor {};

    let mut context = ExtendedContext {
        cpu: CpuContext::default(),
        win32: Win32Context::new(),
        unwind_reason: None,
        interpreted_blocks: HashSet::new(),
        executor: Arc::new(executor.clone()),
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
    context.cpu.gp_regs[4] = stack.end();
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
        &AnsiString::from_ascii(&PROGRAM_IMAGE.main_module_name),
    )
    .expect("Allocating command_line_ansi");

    // NOTE: this is in OEM encoding, not ANSI
    // for now, I don't care, but this... Might be problematic
    let environment_strings_oem = vec![0u8; 2];

    let console = Arc::new(core_console::StdioConsole {});

    let mut handle_table = KernelHandleTable::new();
    let stdin_handle = handle_table.put(Arc::new(KernelObject::Console(console.clone())));
    let stdout_handle = handle_table.put(Arc::new(KernelObject::Console(console.clone())));
    let stderr_handle = handle_table.put(Arc::new(KernelObject::Console(console.clone())));

    let handle_table = Arc::new(Mutex::new(handle_table));

    let windows_handle_table = WindowsHandleTable::new(memory_mgr.clone(), 0x10000);
    let windows_handle_table = Arc::new(Mutex::new(windows_handle_table));

    let user_atom_table = Arc::new(Mutex::new(AtomTable::new()));
    let window_classes_registry = Mutex::new(ClassRegistry::new(user_atom_table));
    let windows_registry = Arc::new(Mutex::new(WindowsRegistry::new()));

    let message_queue_registry = Mutex::new(MessageQueueRegistry::new(windows_registry.clone()));

    let c_fs_tree = {
        let c_fs_dir = std::env::current_dir()
            .expect("getting current_dir")
            .join("drive_c");
        if c_fs_dir.exists() {
            std::fs::remove_dir_all(&c_fs_dir).expect("Cleaning drive_c");
        }
        std::fs::create_dir(&c_fs_dir).expect("Creating drive_c dir");
        core_fs::Tree::FsDir(ArcStr::from(c_fs_dir.to_str().unwrap()))
    };

    let time = Arc::new(TimeProvider {});

    let fs_manager = Arc::new(WindowsFsManager::new(
        handle_table.clone(),
        HashMap::from([(Root::Drive(Drive::C), c_fs_tree)]),
    ));

    // ===

    context.win32.insert(Arc::new(WindowsAndMessaging {
        process_ctx: process_ctx.clone(),
        windows_handle_table,
        window_classes_registry,
        windows_registry: windows_registry.clone(),
        message_queue_registry,
    })
        as Arc<dyn win32::Win32::UI::WindowsAndMessaging::Api>);

    context.win32.insert(Arc::new(Gdi {
        process_ctx: process_ctx.clone(),
    }) as Arc<dyn win32::Win32::Graphics::Gdi::Api>);

    context.win32.insert(Arc::new(SystemInformation {
        process_ctx: process_ctx.clone(),
        time: time.clone(),
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
        module_table: ModuleTable::new(
            PROGRAM_IMAGE
                .modules
                .iter()
                .map(|(name, (_, info))| (name.clone(), HINSTANCE(info.base_addr as PtrDiffRepr))),
            HINSTANCE(PROGRAM_IMAGE.main_module_base as PtrDiffRepr),
        ),
    }) as Arc<dyn win32::Win32::System::LibraryLoader::Api>);

    context.win32.insert(Arc::new(Threading {
        process_ctx: process_ctx.clone(),
    }) as Arc<dyn win32::Win32::System::Threading::Api>);

    context.win32.insert(Arc::new(Console {
        process_ctx: process_ctx.clone(),
        stdin_handle,
        stdout_handle,
        stderr_handle,
    }) as Arc<dyn win32::Win32::System::Console::Api>);

    context.win32.insert(Arc::new(WindowsProgramming {
        process_ctx: process_ctx.clone(),
    })
        as Arc<dyn win32::Win32::System::WindowsProgramming::Api>);

    context.win32.insert(Arc::new(Environment {
        process_ctx: process_ctx.clone(),
        command_line_ansi,
        environment_strings_oem,
        fs_manager: fs_manager.clone(),
    }) as Arc<dyn win32::Win32::System::Environment::Api>);

    context.win32.insert(Arc::new(Globalization {
        process_ctx: process_ctx.clone(),
    }) as Arc<dyn win32::Win32::Globalization::Api>);

    context.win32.insert(Arc::new(FileSystem {
        process_ctx: process_ctx.clone(),
        io_dispatcher: IoDispatcher::new(handle_table.clone()),
        fs_manager: fs_manager.clone(),
    }) as Arc<dyn win32::Win32::Storage::FileSystem::Api>);

    context.win32.insert(Arc::new(Foundation {
        process_ctx: process_ctx.clone(),
        handle_table: handle_table.clone(),
    }) as Arc<dyn win32::Win32::Foundation::Api>);

    context.win32.insert(Arc::new(DirectDrawApi {
        process_ctx: process_ctx.clone(),
        direct_draw_vtable: *PROGRAM_IMAGE
            .com_thunks_info
            .get("Windows.Win32.Graphics.DirectDraw.DirectDraw")
            .expect("Getting DirectDraw vtable")
            .first()
            .unwrap(),
        direct_draw_surface_vtable: *PROGRAM_IMAGE
            .com_thunks_info
            .get("Windows.Win32.Graphics.DirectDraw.DirectDrawSurface")
            .expect("Getting DirectDrawSurface vtable")
            .first()
            .unwrap(),
        windows_registry: windows_registry.clone(),
    }) as Arc<dyn win32::Win32::Graphics::DirectDraw::Api>);

    context.win32.insert(Arc::new(Time {
        process_ctx: process_ctx.clone(),
        time,
    }) as Arc<dyn win32::Win32::System::Time::Api>);

    // =======

    panic_control::chain_hook_ignoring::<UnwindReason>();

    match std::panic::catch_unwind(AssertUnwindSafe(|| {
        for (dll, info) in PROGRAM_IMAGE.modules.values() {
            if info.base_addr != PROGRAM_IMAGE.main_module_base {
                let entry = dll.entry();
                if entry == 0 {
                    continue;
                }
                let entry = DllEntrypointFnPtr::new(info.base_addr + entry);

                let instance = HINSTANCE(info.base_addr as PtrDiffRepr);

                let mut callback_token = StdcallCallbackToken::new(&mut context, memory_ctx);
                // call process attachment callbacks
                // no thread callbacks (at least for now, when we don't have any threads =))
                let res = entry
                    .call(&mut callback_token, instance, DLL_PROCESS_ATTACH, 1)
                    .as_bool();
                if !res {
                    panic!("DllMain for {} failed", dll.name());
                }
            }
        }

        let entry = EntrypointFnPtr::new(PROGRAM_IMAGE.exe_entrypoint);

        let mut callback_token = StdcallCallbackToken::new(&mut context, memory_ctx);
        entry.call(&mut callback_token);
    })) {
        Ok(_) => {
            debug!("The entrypoint returned");
        }
        Err(unwind) => match unwind.downcast::<UnwindReason>() {
            Ok(reason) => {
                debug!("The recompiled stack was unwound, reason = {:?}", reason);
            }
            Err(_) => {
                eprintln!("Target code panicked, and it was not a controlled panic...");
                std::process::abort();
            }
        },
    }

    info!(
        "Interpreted (due to missing recompiled functions) these basic blocks:\n{}",
        context
            .interpreted_blocks
            .iter()
            .cloned()
            .sorted()
            .map(|f| format!("  {:#010x}", f))
            .join("\n")
    )
}

fn main() {
    tracing_subscriber::registry()
        .with(tracing_tracy::TracyLayer::new())
        .with(
            tracing_subscriber::fmt::Layer::new()
                .event_format(format().compact())
                .with_writer(std::io::stderr),
        )
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    main_impl();
}
