use core_abi::unwind_token::{UnwindReason, UnwindToken};
use core_heap::{Heap, RawHeapBox};
use core_mem::ctx::DefaultMemoryCtx;
use core_mem::ptr::{ConstPtr, MutPtr, PtrDiffRepr, PtrRepr};
use core_memmgr::MemoryManager;
use core_str::heap_helper::AnsiStringHeapBox;
use core_str::{AnsiString, PWSTR};
use encoding_rs::Encoding;
use std::ffi::c_void;
use std::sync::{Arc, Mutex};
use tracing::{info, warn};
use win32::core::prelude::{PCSTR, PSTR};
use win32::Win32::Foundation::{BOOL, HANDLE, HINSTANCE, HWND, INVALID_HANDLE_VALUE};
use win32::Win32::Globalization::CPINFO;
use win32::Win32::System::Console::{
    STD_ERROR_HANDLE, STD_HANDLE, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE,
};
use win32::Win32::System::Memory::{
    HeapHandle, HEAP_FLAGS, HEAP_NO_SERIALIZE, PAGE_PROTECTION_FLAGS, VIRTUAL_ALLOCATION_TYPE,
};
use win32::Win32::System::Threading::STARTUPINFOA;
use win32::Win32::System::IO::OVERLAPPED;
use win32::Win32::UI::WindowsAndMessaging::{MESSAGEBOX_RESULT, MESSAGEBOX_STYLE};
use win32_heapmgr::HeapMgr;
use win32_io::IoDispatcher;
use win32_module_table::ModuleTable;
use win32_virtmem::VirtualMemoryManager;

#[derive(Clone)]
pub struct ProcessContext {
    pub memory_manager: Arc<Mutex<MemoryManager>>,
    pub process_heap: Arc<Mutex<Heap>>,
    pub memory_ctx: DefaultMemoryCtx,
    pub ansi_encoding: &'static Encoding,
}

pub struct WindowsAndMessaging {
    pub process_ctx: ProcessContext,
}

#[allow(non_snake_case)]
impl win32::Win32::UI::WindowsAndMessaging::Api for WindowsAndMessaging {
    fn MessageBoxA(
        &self,
        h_wnd: HWND,
        lp_text: PCSTR,
        lp_caption: PCSTR,
        u_type: MESSAGEBOX_STYLE,
    ) -> MESSAGEBOX_RESULT {
        let memory = self.process_ctx.memory_ctx;

        let text = lp_text.read_with(memory);
        let text = text.as_utf8(self.process_ctx.ansi_encoding);
        let caption = lp_caption.read_with(memory);
        let caption = caption.as_utf8(self.process_ctx.ansi_encoding);

        info!(
            "MessageBoxA({:#0x}, {:?}, {:?}, {:#0x})",
            h_wnd.0, text, caption, u_type.0
        );

        MESSAGEBOX_RESULT(0)
    }
}

pub struct SystemInformation {
    pub process_ctx: ProcessContext,
}

#[allow(non_snake_case)]
impl win32::Win32::System::SystemInformation::Api for SystemInformation {
    fn GetVersion(&self) -> u32 {
        0x0ece0205 // (I think?) corresponds to windows 98
    }
}

pub struct Memory {
    pub process_ctx: ProcessContext,
    pub virtmem_mgr: VirtualMemoryManager,
    pub heap_mgr: Mutex<HeapMgr>,
    pub process_heap_handle: PtrRepr,
}

#[allow(non_snake_case)]
impl win32::Win32::System::Memory::Api for Memory {
    fn GetProcessHeap(&self) -> HeapHandle {
        HeapHandle(self.process_heap_handle as PtrDiffRepr)
    }

    fn HeapAlloc(
        &self,
        h_heap: HeapHandle,
        dw_flags: HEAP_FLAGS,
        dw_bytes: PtrRepr,
    ) -> MutPtr<c_void> {
        let heap_mgr = self.heap_mgr.lock().unwrap();

        let res = heap_mgr.alloc(h_heap.0 as PtrRepr, dw_bytes, dw_flags);

        MutPtr::new(res)
    }

    fn HeapCreate(
        &self,
        fl_options: HEAP_FLAGS,
        dw_initial_size: PtrRepr,
        dw_maximum_size: PtrRepr,
    ) -> HeapHandle {
        // we ignore HEAP_NO_SERIALIZE (it's hard in Rust, lol)
        assert_eq!(fl_options & !HEAP_NO_SERIALIZE, HEAP_FLAGS(0));

        let mut heap_mgr = self.heap_mgr.lock().unwrap();

        let heap_handle = heap_mgr.create(dw_initial_size, dw_maximum_size);

        HeapHandle(heap_handle as PtrDiffRepr)
    }

    fn VirtualAlloc(
        &self,
        lp_address: ConstPtr<c_void>,
        dw_size: PtrRepr,
        fl_allocation_type: VIRTUAL_ALLOCATION_TYPE,
        fl_protect: PAGE_PROTECTION_FLAGS,
    ) -> MutPtr<c_void> {
        MutPtr::new(self.virtmem_mgr.alloc(
            lp_address.repr(),
            dw_size,
            fl_allocation_type,
            fl_protect,
        ))
    }
}

pub struct LibraryLoader {
    pub process_ctx: ProcessContext,
    pub module_table: ModuleTable,
}

#[allow(non_snake_case)]
impl win32::Win32::System::LibraryLoader::Api for LibraryLoader {
    fn GetModuleFileNameA(&self, mut h_module: HINSTANCE, lp_filename: PSTR, n_size: u32) -> u32 {
        if h_module == HINSTANCE(0) {
            h_module = self.module_table.get_main_program_handle();
        }
        let name = self.module_table.get_handle_file_name(h_module);
        let name = match name {
            None => return 0,
            Some(name) => name,
        };

        let ansi_str = AnsiString::from_utf8(name, self.process_ctx.ansi_encoding);
        // TODO: handle overflows gracefully
        lp_filename.write_with(self.process_ctx.memory_ctx, n_size, &ansi_str);

        ansi_str.len() as u32
    }

    fn GetModuleHandleA(&self, lp_module_name: PCSTR) -> HINSTANCE {
        if lp_module_name.0.repr() == 0 {
            return self.module_table.get_main_program_handle();
        }
        todo!("GetModuleHandleA")
    }

    fn LoadLibraryA(&self, _lp_lib_file_name: PCSTR) -> HINSTANCE {
        HINSTANCE(0)
    }
}

pub struct Threading {
    pub process_ctx: ProcessContext,
}

#[allow(non_snake_case)]
impl win32::Win32::System::Threading::Api for Threading {
    fn ExitProcess(&self, unwind_token: &mut UnwindToken, u_exit_code: u32) {
        unwind_token.unwind(UnwindReason::ExitProcess(u_exit_code));
    }

    fn GetStartupInfoA(&self, lp_startup_info: MutPtr<STARTUPINFOA>) {
        lp_startup_info.write_with(
            self.process_ctx.memory_ctx,
            STARTUPINFOA {
                cb: 68,
                lpReserved: PSTR(MutPtr::new(0)),
                lpDesktop: PSTR(MutPtr::new(0)),
                lpTitle: PSTR(MutPtr::new(0)),
                dwX: 0,
                dwY: 0,
                dwXSize: 0,
                dwYSize: 0,
                dwXCountChars: 0,
                dwYCountChars: 0,
                dwFillAttribute: 0,
                dwFlags: Default::default(),
                wShowWindow: 0,
                cbReserved2: 0,
                lpReserved2: MutPtr::new(0),
                hStdInput: Default::default(),
                hStdOutput: Default::default(),
                hStdError: Default::default(),
            },
        )
    }
}

pub struct Console {
    pub process_ctx: ProcessContext,
    pub stdin_handle: HANDLE,
    pub stdout_handle: HANDLE,
    pub stderr_handle: HANDLE,
}
#[allow(non_snake_case)]
impl win32::Win32::System::Console::Api for Console {
    fn GetStdHandle(&self, n_std_handle: STD_HANDLE) -> HANDLE {
        if n_std_handle == STD_INPUT_HANDLE {
            self.stdin_handle
        } else if n_std_handle == STD_OUTPUT_HANDLE {
            self.stdout_handle
        } else if n_std_handle == STD_ERROR_HANDLE {
            self.stderr_handle
        } else {
            INVALID_HANDLE_VALUE
        }
    }
}

pub struct WindowsProgramming {
    pub process_ctx: ProcessContext,
}
#[allow(non_snake_case)]
impl win32::Win32::System::WindowsProgramming::Api for WindowsProgramming {
    fn SetHandleCount(&self, u_number: u32) -> u32 {
        // actually a legacy function & impl doesn't really matter
        u_number
    }
}

pub struct Environment {
    pub process_ctx: ProcessContext,
    pub command_line_ansi: AnsiStringHeapBox,
    pub environment_strings_oem: Vec<u8>,
}
#[allow(non_snake_case)]
impl win32::Win32::System::Environment::Api for Environment {
    fn GetCommandLineA(&self) -> PSTR {
        self.command_line_ansi.ptr_mut()
    }

    fn GetEnvironmentStrings(&self) -> PSTR {
        let res = RawHeapBox::new_init(
            self.process_ctx.memory_ctx,
            self.process_ctx.process_heap.clone(),
            &self.environment_strings_oem,
        )
        .unwrap();

        PSTR::new(res.leak())
    }

    fn FreeEnvironmentStringsA(&self, penv: PCSTR) -> BOOL {
        let mut heap = self.process_ctx.process_heap.lock().unwrap();

        heap.free(penv.0.repr()).unwrap();

        BOOL(1)
    }

    // report no unicode support, like on Windows 9x
    fn GetEnvironmentStringsW(&self) -> PWSTR {
        PWSTR::new(0)
    }
}

pub struct Globalization {
    pub process_ctx: ProcessContext,
}

#[allow(non_snake_case)]
impl win32::Win32::Globalization::Api for Globalization {
    fn GetACP(&self) -> u32 {
        1251 // report CP1251... Just because
    }

    fn GetCPInfo(&self, _code_page: u32, _lp_cp_info: MutPtr<CPINFO>) -> BOOL {
        warn!("Returning an error from GetCPInfo (not implemented yet :shrug:)");
        BOOL(0)
    }
}

pub struct FileSystem {
    pub process_ctx: ProcessContext,
    pub io_dispatcher: IoDispatcher,
}

#[allow(non_snake_case)]
impl win32::Win32::Storage::FileSystem::Api for FileSystem {
    fn GetFileType(&self, h_file: HANDLE) -> u32 {
        self.io_dispatcher.get_file_type(h_file)
    }

    fn WriteFile(
        &self,
        h_file: HANDLE,
        lp_buffer: ConstPtr<c_void>,
        n_number_of_bytes_to_write: u32,
        lp_number_of_bytes_written: MutPtr<u32>,
        lp_overlapped: MutPtr<OVERLAPPED>,
    ) -> BOOL {
        assert_eq!(lp_overlapped, MutPtr::null());
        let ctx = self.process_ctx.memory_ctx;

        let bytes = lp_buffer
            .pun::<u8>()
            .read_bytes(ctx, n_number_of_bytes_to_write);

        let (ok, written) = self.io_dispatcher.write_file(h_file, &bytes);

        assert_ne!(lp_number_of_bytes_written, MutPtr::null());
        lp_number_of_bytes_written.write_with(ctx, written);

        ok.into()
    }
}
