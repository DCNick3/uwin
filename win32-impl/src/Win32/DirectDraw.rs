use crate::ProcessContext;
use core_gfx::{GfxContext, Surface, SurfaceFormat};
use core_heap::{Heap, HeapOptions};
use core_mem::conv::FromIntoMemory;
use core_mem::ptr::{MutPtr, PtrRepr};
use std::sync::{Arc, Mutex};
use tracing::trace;
use win32::core::{IUnknown, IUnknown_Trait, GUID, HRESULT};
use win32::Win32::Foundation::{HWND, S_OK};
use win32::Win32::Graphics::DirectDraw::{
    DirectDrawSurface_Repr, DirectDraw_Repr, IDirectDraw, IDirectDrawSurface,
    IDirectDrawSurface_Trait, IDirectDraw_Trait, DDSCAPS_OFFSCREENPLAIN, DDSCAPS_PRIMARYSURFACE,
    DDSCAPS_SYSTEMMEMORY, DDSCL_ALLOWREBOOT, DDSCL_EXCLUSIVE, DDSCL_FULLSCREEN, DDSD_CAPS,
    DDSD_HEIGHT, DDSD_WIDTH, DDSURFACEDESC,
};
use win32_windows::{Window, WindowsRegistry};

pub struct DirectDrawApi {
    pub process_ctx: ProcessContext,
    pub direct_draw_vtable: PtrRepr, // Would __really__ want to have some generalized API for handling those
    pub direct_draw_surface_vtable: PtrRepr,
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

        let mut process_heap = self.process_ctx.process_heap.lock().unwrap();

        let dd_heap = Heap::new(
            self.process_ctx.memory_manager.clone(),
            HeapOptions::default(),
        )
        .expect("Creating heap for DirectDraw object");
        let gfx_context = GfxContext::new(Arc::new(Mutex::new(dd_heap)));

        let cls = Arc::new(DirectDrawCls {
            process_ctx: self.process_ctx.clone(),
            inner: Mutex::new(DirectDrawInner { window: None }),
            windows_registry: self.windows_registry.clone(),
            gfx_context,
            direct_draw_surface_vtable: self.direct_draw_surface_vtable,
        });
        let cls = cls as Arc<dyn IDirectDraw_Trait>;

        let res = process_heap
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
    process_ctx: ProcessContext,
    inner: Mutex<DirectDrawInner>,
    windows_registry: Arc<Mutex<WindowsRegistry>>,
    gfx_context: GfxContext,
    direct_draw_surface_vtable: PtrRepr,
}

impl IUnknown_Trait for DirectDrawCls {}

impl IDirectDraw_Trait for DirectDrawCls {
    fn CreateSurface(
        &self,
        lpDDSurfaceDesc: MutPtr<DDSURFACEDESC>,
        lplpDDSurface: MutPtr<IDirectDrawSurface>,
        pUnkOther: IUnknown,
    ) -> HRESULT {
        assert_eq!(pUnkOther.raw_ptr(), 0);

        let ctx = self.process_ctx.memory_ctx;
        let inner = self.inner.lock().unwrap();

        let desc = lpDDSurfaceDesc.read_with(ctx);

        assert_eq!(desc.dwSize, DDSURFACEDESC::size().try_into().unwrap());

        assert_ne!(desc.dwFlags as i32 & DDSD_CAPS, 0);

        let caps = desc.ddsCaps;

        let surface = if caps.dwCaps as i32 == DDSCAPS_PRIMARYSURFACE {
            let window = inner
                .window
                .as_ref()
                .expect("Creating a primary surface without associated window")
                .clone();

            let (width, height) = window.size;

            let registry = self.windows_registry.lock().unwrap();
            let window = registry
                .core_windows_context()
                .get_window(window.window_id)
                .unwrap();

            self.gfx_context
                .create_onscreen(width, height, window.as_ref())
        } else if caps.dwCaps as i32 == DDSCAPS_SYSTEMMEMORY | DDSCAPS_OFFSCREENPLAIN {
            assert_eq!(
                desc.dwFlags as i32 & !(DDSD_CAPS | DDSD_HEIGHT | DDSD_WIDTH),
                0
            );
            assert_ne!(desc.dwFlags as i32 & DDSD_HEIGHT, 0);
            assert_ne!(desc.dwFlags as i32 & DDSD_WIDTH, 0);
            let width = desc.dwWidth.try_into().unwrap();
            let height = desc.dwHeight.try_into().unwrap();
            let format = SurfaceFormat::Rgb565;

            self.gfx_context.create_offscreen(width, height, format)
        } else {
            unimplemented!("Unsupported caps: {}", caps.dwCaps)
        };

        let surface = Arc::new(DirectDrawSurface { surface });

        let mut process_heap = self.process_ctx.process_heap.lock().unwrap();

        let res = process_heap
            .alloc_typed(
                ctx,
                DirectDrawSurface_Repr {
                    vtable_IDirectDrawSurface: self.direct_draw_surface_vtable,
                    implementation: Arc::into_raw(surface),
                },
            )
            .expect("Allocating memory for the DirectDraw object");

        lplpDDSurface.write_with(ctx, IDirectDrawSurface(IUnknown::from_raw_ptr(res.repr())));

        S_OK
    }

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

struct DirectDrawSurface {
    surface: Surface,
}

impl IUnknown_Trait for DirectDrawSurface {}

impl IDirectDrawSurface_Trait for DirectDrawSurface {}
