use crate::ProcessContext;
use core_abi::callback_token::StdcallCallbackTokenTrait;
use core_heap::HeapBox;
use core_mem::ptr::{ConstPtr, MutPtr, PtrDiffRepr};
use core_str::PCSTR;
use std::ffi::c_void;
use std::sync::{Arc, Mutex};
use std::thread;
use tracing::{info, trace, warn};
use win32::Win32::Foundation::{BOOL, HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use win32::Win32::UI::WindowsAndMessaging::{
    CREATESTRUCTA, HCURSOR, HICON, HMENU, MESSAGEBOX_RESULT, MESSAGEBOX_STYLE, MSG,
    SHOW_WINDOW_CMD, SM_CXSCREEN, SM_CYSCREEN, SYSTEM_METRICS_INDEX, WINDOW_EX_STYLE, WINDOW_STYLE,
    WM_CREATE, WM_MOUSEMOVE, WM_NCCREATE, WM_QUIT, WNDCLASSA,
};
use win32_message_queue::MessageQueueRegistry;
use win32_windows::{ClassRegistry, WindowClass, WindowsRegistry};
use win32_wobj::{WindowsHandleTable, WindowsObject};

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
