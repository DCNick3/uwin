use core_abi::unwind_token::{UnwindReason, UnwindToken};
use core_mem::ptr::{ConstPtr, MutPtr, PtrDiffRepr, PtrRepr};
use core_mem::thread_ctx::get_thread_ctx;
use encoding_rs::Encoding;
use std::ffi::c_void;
use std::sync::Mutex;
use tracing::{info, warn};
use win32::core::prelude::{PCSTR, PSTR};
use win32::Win32::Foundation::{HANDLE, HINSTANCE, HWND, INVALID_HANDLE_VALUE};
use win32::Win32::System::Console::STD_HANDLE;
use win32::Win32::System::Memory::{
    HeapHandle, HEAP_FLAGS, HEAP_NO_SERIALIZE, PAGE_PROTECTION_FLAGS, VIRTUAL_ALLOCATION_TYPE,
};
use win32::Win32::System::Threading::STARTUPINFOA;
use win32::Win32::UI::WindowsAndMessaging::{MESSAGEBOX_RESULT, MESSAGEBOX_STYLE};
use win32_heapmgr::HeapMgr;
use win32_virtmem::VirtualMemoryManager;

pub struct WindowsAndMessaging {
    pub local_encoding: &'static Encoding,
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
        let memory = get_thread_ctx();

        let text = lp_text.read_with(memory);
        let text = text.as_utf8(self.local_encoding);
        let caption = lp_caption.read_with(memory);
        let caption = caption.as_utf8(self.local_encoding);

        info!(
            "MessageBoxA({:#0x}, {:?}, {:?}, {:#0x})",
            h_wnd.0, text, caption, u_type.0
        );

        MESSAGEBOX_RESULT(0)
    }
}

pub struct SystemInformation {}

#[allow(non_snake_case)]
impl win32::Win32::System::SystemInformation::Api for SystemInformation {
    fn GetVersion(&self) -> u32 {
        0x0ece0205 // (I think?) corresponds to windows 98
    }
}

pub struct Memory {
    pub(crate) virtmem_mgr: VirtualMemoryManager,
    pub(crate) heap_mgr: Mutex<HeapMgr>,
}

#[allow(non_snake_case)]
impl win32::Win32::System::Memory::Api for Memory {
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

pub struct LibraryLoader {}

#[allow(non_snake_case)]
impl win32::Win32::System::LibraryLoader::Api for LibraryLoader {
    fn GetModuleFileNameA(&self, _h_module: HINSTANCE, _lp_filename: PSTR, _n_size: u32) -> u32 {
        0
    }

    fn LoadLibraryA(&self, _lp_lib_file_name: PCSTR) -> HINSTANCE {
        HINSTANCE(0)
    }
}

pub struct Threading {}

#[allow(non_snake_case)]
impl win32::Win32::System::Threading::Api for Threading {
    fn ExitProcess(&self, unwind_token: &mut UnwindToken, u_exit_code: u32) {
        unwind_token.unwind(UnwindReason::ExitProcess(u_exit_code));
    }

    fn GetStartupInfoA(&self, lp_startup_info: MutPtr<STARTUPINFOA>) {
        lp_startup_info.write(STARTUPINFOA {
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
        })
    }
}

pub struct Console {}
#[allow(non_snake_case)]
impl win32::Win32::System::Console::Api for Console {
    fn GetStdHandle(&self, n_std_handle: STD_HANDLE) -> HANDLE {
        warn!("Returning invalid handle for std handle {:?}", n_std_handle);

        INVALID_HANDLE_VALUE
    }
}

pub struct WindowsProgramming {}
#[allow(non_snake_case)]
impl win32::Win32::System::WindowsProgramming::Api for WindowsProgramming {
    fn SetHandleCount(&self, u_number: u32) -> u32 {
        // actually a legacy function & impl doesn't really matter
        u_number
    }
}

pub struct Environment {}
#[allow(non_snake_case)]
impl win32::Win32::System::Environment::Api for Environment {
    fn GetCommandLineA(&self) -> PSTR {
        // well, this actually needs some setup
        todo!()
    }
}
