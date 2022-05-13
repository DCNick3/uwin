use core_abi::callback_token::StdcallCallbackTokenTrait;
use core_abi::unwind_token::{UnwindReason, UnwindToken};
use core_heap::{Heap, HeapBox, RawHeapBox};
use core_mem::ctx::DefaultMemoryCtx;
use core_mem::ptr::{ConstPtr, MutPtr, PtrDiffRepr, PtrRepr};
use core_memmgr::MemoryManager;
use core_str::heap_helper::AnsiStringHeapBox;
use core_str::{AnsiString, PWSTR};
use encoding_rs::Encoding;
use std::ffi::c_void;
use std::sync::{Arc, Mutex};
use std::thread;
use tracing::{info, trace, warn};
use win32::core::prelude::{PCSTR, PSTR};
use win32::core::{IUnknown, GUID, HRESULT};
use win32::Win32::Foundation::{
    BOOL, HANDLE, HINSTANCE, HWND, INVALID_HANDLE_VALUE, LPARAM, LRESULT, WPARAM,
};
use win32::Win32::Globalization::CPINFO;
use win32::Win32::Graphics::DirectDraw::IDirectDraw;
use win32::Win32::System::Console::{
    STD_ERROR_HANDLE, STD_HANDLE, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE,
};
use win32::Win32::System::Memory::{
    HeapHandle, HEAP_FLAGS, HEAP_NO_SERIALIZE, PAGE_PROTECTION_FLAGS, VIRTUAL_ALLOCATION_TYPE,
};
use win32::Win32::System::Threading::STARTUPINFOA;
use win32::Win32::System::IO::OVERLAPPED;
use win32::Win32::UI::WindowsAndMessaging::{
    CREATESTRUCTA, HCURSOR, HICON, HMENU, MESSAGEBOX_RESULT, MESSAGEBOX_STYLE, MSG,
    SHOW_WINDOW_CMD, SM_CXSCREEN, SM_CYSCREEN, SYSTEM_METRICS_INDEX, WINDOW_EX_STYLE, WINDOW_STYLE,
    WM_CREATE, WM_MOUSEMOVE, WM_NCCREATE, WM_QUIT, WNDCLASSA,
};
use win32_heapmgr::HeapMgr;
use win32_io::IoDispatcher;
use win32_message_queue::MessageQueueRegistry;
use win32_module_table::ModuleTable;
use win32_virtmem::VirtualMemoryManager;
use win32_windows::{ClassRegistry, WindowClass, WindowsRegistry};
use win32_wobj::{WindowsHandleTable, WindowsObject};

#[derive(Clone)]
pub struct ProcessContext {
    pub memory_manager: Arc<Mutex<MemoryManager>>,
    pub process_heap: Arc<Mutex<Heap>>,
    pub memory_ctx: DefaultMemoryCtx,
    pub ansi_encoding: &'static Encoding,
}

pub struct WindowsAndMessaging {
    pub process_ctx: ProcessContext,
    pub windows_handle_table: Arc<Mutex<WindowsHandleTable>>,
    pub window_classes_registry: Mutex<ClassRegistry>,
    pub windows_registry: Mutex<WindowsRegistry>,
    pub message_queue_registry: Mutex<MessageQueueRegistry>,
}

#[allow(non_snake_case)]
impl win32::Win32::UI::WindowsAndMessaging::Api for WindowsAndMessaging {
    fn CreateWindowExA(
        &self,
        callback_token: &mut dyn StdcallCallbackTokenTrait,
        dw_ex_style: WINDOW_EX_STYLE,
        lp_class_name: PCSTR,
        lp_window_name: PCSTR,
        dw_style: WINDOW_STYLE,
        x: i32,
        y: i32,
        n_width: i32,
        n_height: i32,
        h_wnd_parent: HWND,
        h_menu: HMENU,
        h_instance: HINSTANCE,
        lp_param: ConstPtr<c_void>,
    ) -> HWND {
        let ctx = self.process_ctx.memory_ctx;
        let class_name = lp_class_name.read_with(ctx);
        let class_name = class_name.as_utf8(self.process_ctx.ansi_encoding);
        let class = self
            .window_classes_registry
            .lock()
            .unwrap()
            .find(&class_name)
            .expect("Creation of window with a non-existing class");

        let mut window_registry = self.windows_registry.lock().unwrap();

        // preventively create the message queue (well, it will be needed to handle the window messages
        let mut registry = self.message_queue_registry.lock().unwrap();

        let thread_id = thread::current().id();

        let queue = registry.insert(thread_id);
        let sender = queue.get_sender();

        let wndproc = class.wndproc;

        let hwnd = window_registry.create(
            class,
            lp_param.repr(),
            (n_width as u32, n_height as u32),
            sender,
        );

        let create_struct_box = HeapBox::new(
            ctx,
            self.process_ctx.process_heap.clone(),
            CREATESTRUCTA {
                lpCreateParams: MutPtr::new(lp_param.repr()),
                hInstance: h_instance,
                hMenu: h_menu,
                hwndParent: h_wnd_parent,
                cy: n_height,
                cx: n_width,
                y,
                x,
                style: dw_style.0 as i32,
                lpszName: lp_window_name,
                lpszClass: lp_class_name,
                dwExStyle: dw_ex_style.0,
            },
        )
        .unwrap();

        trace!("Calling wndproc with WM_NCCREATE message");
        let res = wndproc.call(
            callback_token,
            hwnd,
            WM_NCCREATE,
            WPARAM::default(),
            LPARAM(create_struct_box.ptr().repr() as PtrDiffRepr),
        );

        assert_ne!(
            res.0, 0,
            "WM_NCCREATE handler asked us not to create the window"
        );

        trace!("Calling wndproc with WM_CREATE message");
        let res = wndproc.call(
            callback_token,
            hwnd,
            WM_CREATE,
            WPARAM::default(),
            LPARAM(create_struct_box.ptr().repr() as PtrDiffRepr),
        );

        assert_eq!(
            res.0, 0,
            "WM_CREATE handler returned something different than 0"
        );

        hwnd
    }

    fn DefWindowProcA(
        &self,
        _h_wnd: HWND,
        msg: u32,
        _w_param: WPARAM,
        _l_param: LPARAM,
    ) -> LRESULT {
        match msg {
            WM_NCCREATE => LRESULT(1),
            WM_CREATE => LRESULT(0),
            WM_MOUSEMOVE => LRESULT(1),
            _ => todo!("Window message {:#010x}", msg),
        }
    }

    fn DispatchMessageA(
        &self,
        callback_token: &mut dyn StdcallCallbackTokenTrait,
        lp_msg: ConstPtr<MSG>,
    ) -> LRESULT {
        let ctx = self.process_ctx.memory_ctx;
        let msg = lp_msg.read_with(ctx);
        let hwnd = msg.hwnd;

        let wndproc = {
            let windows_registry = self.windows_registry.lock().unwrap();
            let window = windows_registry.find(hwnd).unwrap();
            window.wndproc()
        };

        wndproc.call(callback_token, hwnd, msg.message, msg.wParam, msg.lParam)
    }

    fn GetMessageA(
        &self,
        lp_msg: MutPtr<MSG>,
        h_wnd: HWND,
        w_msg_filter_min: u32,
        w_msg_filter_max: u32,
    ) -> BOOL {
        // we don't support any filtering
        assert_eq!(h_wnd, HWND::default());
        assert_eq!(w_msg_filter_max, 0);
        assert_eq!(w_msg_filter_min, 0);

        let registry = self.message_queue_registry.lock().unwrap();
        let thread_id = thread::current().id();

        let queue = registry.get_queue(thread_id).unwrap();

        let msg = queue.get_message();

        let memory = self.process_ctx.memory_ctx;

        lp_msg.write_with(memory, msg);

        (msg.message != WM_QUIT).into()
    }

    fn GetSystemMetrics(&self, n_index: SYSTEM_METRICS_INDEX) -> i32 {
        // TODO: these are clearly stubs
        if n_index == SM_CXSCREEN {
            800
        } else if n_index == SM_CYSCREEN {
            600
        } else {
            todo!("GetSystemMetrics({:?})", n_index)
        }
    }

    fn LoadCursorA(&self, _h_instance: HINSTANCE, _lp_cursor_name: PCSTR) -> HCURSOR {
        let mut ht = self.windows_handle_table.lock().unwrap();
        let res = ht.put(WindowsObject::Cursor());

        HCURSOR(res as PtrDiffRepr)
    }

    fn LoadIconA(&self, _h_instance: HINSTANCE, _lp_icon_name: PCSTR) -> HICON {
        let mut ht = self.windows_handle_table.lock().unwrap();
        let res = ht.put(WindowsObject::Icon());

        HICON(res as PtrDiffRepr)
    }

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

    fn RegisterClassA(&self, lp_wnd_class: ConstPtr<WNDCLASSA>) -> u16 {
        let ctx = self.process_ctx.memory_ctx;

        let wnd_class = lp_wnd_class.read_with(ctx);

        let name = wnd_class
            .lpszClassName
            .read_with(ctx)
            .as_utf8(self.process_ctx.ansi_encoding)
            .to_string();

        let class = WindowClass {
            name,
            wndproc: wnd_class.lpfnWndProc,
        };

        let mut registry = self.window_classes_registry.lock().unwrap();

        registry.register(class)
    }

    fn ShowWindow(&self, h_wnd: HWND, n_cmd_show: SHOW_WINDOW_CMD) -> BOOL {
        warn!(
            "No-op stub for ShowWindow({:?}, {:?}) called",
            h_wnd, n_cmd_show
        );
        BOOL(1)
    }

    fn TranslateMessage(&self, _lp_msg: ConstPtr<MSG>) -> BOOL {
        // nothing to do (at least yet)
        BOOL(0)
    }
}

pub struct Gdi {
    pub process_ctx: ProcessContext,
}

#[allow(non_snake_case)]
impl win32::Win32::Graphics::Gdi::Api for Gdi {
    fn UpdateWindow(&self, h_wnd: HWND) -> BOOL {
        warn!("No-op stub for UpdateWindow({:?}) called", h_wnd);
        BOOL(1)
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
    pub process_heap_handle: HeapHandle,
}

#[allow(non_snake_case)]
impl win32::Win32::System::Memory::Api for Memory {
    fn GetProcessHeap(&self) -> HeapHandle {
        self.process_heap_handle
    }

    fn HeapAlloc(
        &self,
        h_heap: HeapHandle,
        dw_flags: HEAP_FLAGS,
        dw_bytes: PtrRepr,
    ) -> MutPtr<c_void> {
        let heap_mgr = self.heap_mgr.lock().unwrap();

        let res = heap_mgr.alloc(h_heap, dw_bytes, dw_flags);

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

        heap_mgr.create(dw_initial_size, dw_maximum_size)
    }

    fn HeapFree(&self, h_heap: HeapHandle, dw_flags: HEAP_FLAGS, lp_mem: ConstPtr<c_void>) -> BOOL {
        let heap_mgr = self.heap_mgr.lock().unwrap();

        heap_mgr.free(h_heap, lp_mem.repr(), dw_flags)
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
    fn FreeEnvironmentStringsA(&self, penv: PCSTR) -> BOOL {
        let mut heap = self.process_ctx.process_heap.lock().unwrap();

        heap.free(penv.0.repr()).unwrap();

        BOOL(1)
    }

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

    fn ReadFile(
        &self,
        h_file: HANDLE,
        lp_buffer: MutPtr<c_void>,
        n_number_of_bytes_to_read: u32,
        lp_number_of_bytes_read: MutPtr<u32>,
        lp_overlapped: MutPtr<OVERLAPPED>,
    ) -> BOOL {
        assert_eq!(lp_overlapped, MutPtr::null());
        let ctx = self.process_ctx.memory_ctx;

        let mut buffer = vec![0u8; n_number_of_bytes_to_read as usize];

        let (ok, read) = self.io_dispatcher.read_file(h_file, &mut buffer);

        lp_buffer
            .pun::<u8>()
            .write_bytes(ctx, &buffer[..read as usize]);

        assert_ne!(lp_number_of_bytes_read, MutPtr::null());
        lp_number_of_bytes_read.write_with(ctx, read);

        ok.into()
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

pub struct DirectDraw {
    pub process_ctx: ProcessContext,
}

#[allow(non_snake_case)]
impl win32::Win32::Graphics::DirectDraw::Api for DirectDraw {
    fn DirectDrawCreate(
        &self,
        lp_guid: MutPtr<GUID>,
        _lplp_dd: MutPtr<IDirectDraw>,
        p_unk_outer: IUnknown,
    ) -> HRESULT {
        let _ctx = self.process_ctx.memory_ctx;

        assert_eq!(lp_guid, MutPtr::null());
        assert_eq!(p_unk_outer.raw_ptr(), 0);

        todo!()
    }
}
