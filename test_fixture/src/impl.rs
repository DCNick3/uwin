use core_abi::unwind_token::{UnwindReason, UnwindToken};
use core_mem::ptr::{MutPtr, PtrDiffRepr, PtrRepr};
use core_mem::thread_ctx::get_thread_ctx;
use core_memmgr::MemoryManager;
use std::ffi::{c_void, CStr};
use std::os::raw::c_char;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::info;
use win32::core::{PCSTR, PSTR};
use win32::Win32::Foundation::{HINSTANCE, HWND};
use win32::Win32::System::Memory::{HeapHandle, HEAP_FLAGS, HEAP_NO_SERIALIZE, HEAP_ZERO_MEMORY};
use win32::Win32::UI::WindowsAndMessaging::{MESSAGEBOX_RESULT, MESSAGEBOX_STYLE};
use win32_heapmgr::HeapMgr;

pub struct WindowsAndMessaging {}

#[allow(non_snake_case)]
impl win32::Win32::UI::WindowsAndMessaging::Api for WindowsAndMessaging {
    fn MessageBoxA(
        &self,
        h_wnd: HWND,
        lp_text: ::win32::core::PCSTR,
        lp_caption: ::win32::core::PCSTR,
        u_type: MESSAGEBOX_STYLE,
    ) -> MESSAGEBOX_RESULT {
        let memory = get_thread_ctx();

        let text =
            unsafe { CStr::from_ptr(memory.to_native_ptr(lp_text.0.repr()) as *const c_char) }
                .to_str()
                .unwrap();
        let caption =
            unsafe { CStr::from_ptr(memory.to_native_ptr(lp_caption.0.repr()) as *const c_char) }
                .to_str()
                .unwrap();

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
    pub(crate) memory_mgr: Arc<Mutex<MemoryManager>>,
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
}
