use crate::ProcessContext;
use core_mem::ptr::{MutPtr, PtrRepr};
use std::sync::{Arc, Mutex};
use tracing::trace;
use win32::core::{IUnknown, IUnknown_Trait, GUID, HRESULT};
use win32::Win32::Foundation::{HWND, S_OK};
use win32::Win32::Graphics::DirectDraw::{
    DirectDraw_Repr, IDirectDraw, IDirectDraw_Trait, DDSCL_ALLOWREBOOT, DDSCL_EXCLUSIVE,
    DDSCL_FULLSCREEN,
};
use win32_windows::{Window, WindowsRegistry};

pub struct DirectDrawApi {
    pub process_ctx: ProcessContext,
    pub direct_draw_vtable: PtrRepr, // Would __really__ want to have some generalized API for handling those
    pub windows_registry: Arc<Mutex<WindowsRegistry>>,
}

#[allow(non_snake_case)]
impl win32::Win32::Graphics::DirectDraw::Api for DirectDrawApi {
    fn DirectDrawCreate(
        &self,
        lp_guid: MutPtr<GUID>,
        lplp_dd: MutPtr<IDirectDraw>,
        p_unk_outer: IUnknown,
    ) -> HRESULT {
        let ctx = self.process_ctx.memory_ctx;

        assert_eq!(lp_guid, MutPtr::null());
        assert_eq!(p_unk_outer.raw_ptr(), 0);

        let mut heap = self.process_ctx.process_heap.lock().unwrap();

        let cls = Arc::new(DirectDrawCls {
            inner: Mutex::new(DirectDrawInner { window: None }),
            windows_registry: self.windows_registry.clone(),
        });
        let cls = cls as Arc<dyn IDirectDraw_Trait>;

        let res = heap
            .alloc_typed(
                ctx,
                DirectDraw_Repr {
                    vtable_IDirectDraw: self.direct_draw_vtable,
                    implementation: Arc::into_raw(cls),
                },
            )
            .expect("Allocating memory for the DirectDraw object");

        trace!("Allocated a DirectDraw object at {:#010x}", res.repr());

        // TODO: use some API to cast the class repr to an interface ptr
        // would probably be useful for QueryInterface implementations

        lplp_dd.write_with(ctx, IDirectDraw(IUnknown::from_raw_ptr(res.repr())));

        S_OK
    }
}

struct DirectDrawInner {
    window: Option<Arc<Window>>,
}

struct DirectDrawCls {
    inner: Mutex<DirectDrawInner>,
    windows_registry: Arc<Mutex<WindowsRegistry>>,
}

impl IUnknown_Trait for DirectDrawCls {}

impl IDirectDraw_Trait for DirectDrawCls {
    fn SetCooperativeLevel(&self, hWnd: HWND, dwFlags: u32) -> HRESULT {
        // ignore DDSCL_ALLOWREBOOT, otherwise allow only DDSCL_FULLSCREEN | DDSCL_EXCLUSIVE
        assert_eq!(
            dwFlags as i32 & !DDSCL_ALLOWREBOOT,
            DDSCL_FULLSCREEN | DDSCL_EXCLUSIVE
        );

        let window = {
            let registry = self.windows_registry.lock().unwrap();
            registry
                .find(hWnd)
                .expect("Could not find the window specified in SetCooperativeLevel")
        };

        let mut inner = self.inner.lock().unwrap();

        assert!(inner.window.is_none());
        inner.window = Some(window);

        S_OK
    }

    fn SetDisplayMode(&self, dwWidth: u32, dwHeight: u32, dwBPP: u32) -> HRESULT {
        let inner = self.inner.lock().unwrap();
        let window = inner
            .window
            .as_ref()
            .expect("Call to SetDisplayMode without an associated window")
            .as_ref();
        assert_eq!(
            window.size,
            (dwWidth, dwHeight),
            "Crated window size and the requested display mode do not match ({:?} vs {:?})",
            window.size,
            (dwWidth, dwHeight)
        );

        assert_eq!(dwBPP, 16, "Currently only 16-bit color is supported");

        S_OK
    }
}
