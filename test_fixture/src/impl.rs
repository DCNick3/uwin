use core_abi::unwind_token::{UnwindReason, UnwindToken};
use core_mem::ptr::PtrRepr;
use core_mem::thread_ctx::get_thread_ctx;
use log::trace;
use std::ffi::CStr;
use std::os::raw::c_char;
use win32::core::{PCSTR, PSTR};
use win32::Win32::Foundation::{HINSTANCE, HWND};
use win32::Win32::System::Memory::{HeapHandle, HEAP_FLAGS};
use win32::Win32::UI::WindowsAndMessaging::{MESSAGEBOX_RESULT, MESSAGEBOX_STYLE};

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

        trace!(
            "MessageBoxA({:?}, {:?}, {:?}, {:?})",
            h_wnd,
            text,
            caption,
            u_type
        );

        MESSAGEBOX_RESULT(0)
    }
}

pub struct SystemInformation {}

#[allow(non_snake_case)]
impl win32::Win32::System::SystemInformation::Api for SystemInformation {
    fn GetVersion(&self) -> u32 {
        trace!("GetVersion()");
        0x0ece0205 // (I think?) corresponds to windows 98
    }
}

pub struct Memory {}

#[allow(non_snake_case)]
impl win32::Win32::System::Memory::Api for Memory {
    fn HeapCreate(
        &self,
        fl_options: HEAP_FLAGS,
        dw_initial_size: PtrRepr,
        dw_maximum_size: PtrRepr,
    ) -> HeapHandle {
        trace!(
            "HeapCreate({:?}, {:?}, {:?})",
            fl_options,
            dw_initial_size,
            dw_maximum_size
        );
        HeapHandle(0)
    }
}

pub struct LibraryLoader {}

#[allow(non_snake_case)]
impl win32::Win32::System::LibraryLoader::Api for LibraryLoader {
    fn GetModuleFileNameA(&self, h_module: HINSTANCE, lp_filename: PSTR, n_size: u32) -> u32 {
        trace!(
            "GetModuleFileNameA({:?}, {:?}, {:?})",
            h_module,
            lp_filename,
            n_size
        );
        0
    }

    fn LoadLibraryA(&self, lp_lib_file_name: PCSTR) -> HINSTANCE {
        trace!("LoadLibraryA({:?})", lp_lib_file_name);
        HINSTANCE(0)
    }
}

pub struct Threading {}

#[allow(non_snake_case)]
impl win32::Win32::System::Threading::Api for Threading {
    fn ExitProcess(&self, unwind_token: &mut UnwindToken, u_exit_code: u32) {
        trace!("ExitProcess({:?})", u_exit_code);

        unwind_token.unwind(UnwindReason::ExitProcess(u_exit_code));
    }
}
