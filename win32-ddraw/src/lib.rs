use core_gfx::{GfxContext, Surface, SurfaceFormat};
use core_heap::Heap;
use core_mem::conv::FromIntoMemory;
use core_mem::ctx::DefaultMemoryCtx;
use core_mem::ptr::{MutPtr, PtrRepr};
use std::ffi::c_void;
use std::sync::{Arc, Mutex};
use win32::core::{IUnknown, IUnknown_Trait, HRESULT};
use win32::Win32::Foundation::{HANDLE, HWND, RECT, S_OK};
use win32::Win32::Graphics::DirectDraw::{
    DirectDrawSurface_Repr, IDirectDrawSurface, IDirectDrawSurface_Trait, IDirectDraw_Trait,
    DDLOCK_WAIT, DDSCAPS_OFFSCREENPLAIN, DDSCAPS_PRIMARYSURFACE, DDSCAPS_SYSTEMMEMORY,
    DDSCL_ALLOWREBOOT, DDSCL_EXCLUSIVE, DDSCL_FULLSCREEN, DDSD_CAPS, DDSD_HEIGHT, DDSD_WIDTH,
    DDSURFACEDESC,
};
use win32_windows::{Window, WindowsRegistry};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

struct DirectDrawInner {
    window: Option<Arc<Window>>,
}

pub struct DirectDraw {
    memory_ctx: DefaultMemoryCtx,
    heap: Arc<Mutex<Heap>>,
    inner: Mutex<DirectDrawInner>,
    windows_registry: Arc<Mutex<WindowsRegistry>>,
    gfx_context: GfxContext,
    direct_draw_surface_vtable: PtrRepr,
}

impl DirectDraw {
    pub fn new(
        memory_ctx: DefaultMemoryCtx,
        heap: Arc<Mutex<Heap>>,
        windows_registry: Arc<Mutex<WindowsRegistry>>,
        gfx_context: GfxContext,
        direct_draw_surface_vtable: PtrRepr,
    ) -> Self {
        Self {
            memory_ctx,
            heap,
            inner: Mutex::new(DirectDrawInner { window: None }),
            windows_registry,
            gfx_context,
            direct_draw_surface_vtable,
        }
    }
}

impl IUnknown_Trait for DirectDraw {}

#[allow(non_snake_case)]
impl IDirectDraw_Trait for DirectDraw {
    fn CreateSurface(
        &self,
        lpDDSurfaceDesc: MutPtr<DDSURFACEDESC>,
        lplpDDSurface: MutPtr<IDirectDrawSurface>,
        pUnkOther: IUnknown,
    ) -> HRESULT {
        assert_eq!(pUnkOther.raw_ptr(), 0);

        let ctx = self.memory_ctx;
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

        let mut process_heap = self.heap.lock().unwrap();

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

pub struct DirectDrawSurface {
    surface: Surface,
}

impl IUnknown_Trait for DirectDrawSurface {}

#[allow(non_snake_case)]
impl IDirectDrawSurface_Trait for DirectDrawSurface {
    fn Lock(
        &self,
        lpDestRect: MutPtr<RECT>,
        _lpDDSurfaceDesc: MutPtr<DDSURFACEDESC>,
        dwFlags: u32,
        hEvent: HANDLE,
    ) -> HRESULT {
        assert_eq!(lpDestRect, MutPtr::null());
        assert_eq!(hEvent, HANDLE(0));
        assert_eq!(dwFlags as i32, DDLOCK_WAIT);

        match &self.surface {
            Surface::Onscreen(_) => {
                panic!("Attempt to lock an onscreen surface (not supported, at least yet)")
            }
            Surface::Offscreen(_surface) => {
                todo!()
            }
        }
    }

    fn Unlock(&self, _lpSurfaceData: MutPtr<c_void>) -> HRESULT {
        todo!()
    }
}
