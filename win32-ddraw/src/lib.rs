use core_gfx::{GfxContext, Surface, SurfaceFormat};
use core_heap::Heap;
use core_mem::conv::FromIntoMemory;
use core_mem::ctx::DefaultMemoryCtx;
use core_mem::ptr::{ConstPtr, MutPtr, PtrRepr};
use std::ffi::c_void;
use std::sync::{Arc, Mutex};
use win32::core::{IUnknown, IUnknown_Trait, HRESULT};
use win32::Win32::Foundation::{HANDLE, HWND, RECT, S_OK};
use win32::Win32::Graphics::DirectDraw::{
    DirectDrawSurface_Repr, IDirectDrawSurface, IDirectDrawSurface_Trait, IDirectDraw_Trait,
    DDBLTFX, DDCOLORKEY, DDLOCK_WAIT, DDPF_RGB, DDPIXELFORMAT, DDSCAPS, DDSCAPS_OFFSCREENPLAIN,
    DDSCAPS_PRIMARYSURFACE, DDSCAPS_SYSTEMMEMORY, DDSCL_ALLOWREBOOT, DDSCL_EXCLUSIVE,
    DDSCL_FULLSCREEN, DDSD_CAPS, DDSD_HEIGHT, DDSD_PITCH, DDSD_PIXELFORMAT, DDSD_WIDTH,
    DDSURFACEDESC,
};
use win32_windows::{Window, WindowsRegistry};

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

        let surface = Arc::new(DirectDrawSurface {
            surface,
            memory_ctx: ctx,
            direct_draw_surface_vtable: self.direct_draw_surface_vtable,
        });

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
            DDSCL_FULLSCREEN | DDSCL_EXCLUSIVE,
            "Unsupported SetCooperativeLevel flags"
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
    memory_ctx: DefaultMemoryCtx,
    surface: Surface,
    direct_draw_surface_vtable: PtrRepr,
}

impl DirectDrawSurface {
    fn get_pixel_format(&self) -> DDPIXELFORMAT {
        // TODO: when unions are more working return the RGB pixel layout
        DDPIXELFORMAT {
            dwSize: DDPIXELFORMAT::size().try_into().unwrap(),
            dwFlags: DDPF_RGB as _,
            dwFourCC: 0,
            Anonymous1: Default::default(), //todo!(),
            Anonymous2: Default::default(), //todo!(),
            Anonymous3: Default::default(), //todo!(),
            Anonymous4: Default::default(), //todo!(),
            Anonymous5: Default::default(), //todo!(),
        }
    }

    fn get_surface_desc(&self) -> DDSURFACEDESC {
        let zero_colorkey = DDCOLORKEY {
            dwColorSpaceLowValue: 0,
            dwColorSpaceHighValue: 0,
        };

        match &self.surface {
            Surface::Onscreen(_) => {
                todo!("get_surface_desc for onscreen surfaces")
            }
            Surface::Offscreen(surface) => DDSURFACEDESC {
                dwSize: DDSURFACEDESC::size().try_into().unwrap(),
                dwFlags: (DDSD_PIXELFORMAT | DDSD_PITCH | DDSD_WIDTH | DDSD_HEIGHT | DDSD_CAPS)
                    as u32,
                dwHeight: surface.height as _,
                dwWidth: surface.width as _,
                Anonymous1: Default::default(),
                dwBackBufferCount: 0,
                Anonymous2: Default::default(),
                dwAlphaBitDepth: 0,
                dwReserved: 0,
                lpSurface: MutPtr::null(),
                ddckCKDestOverlay: zero_colorkey,
                ddckCKDestBlt: zero_colorkey,
                ddckCKSrcOverlay: zero_colorkey,
                ddckCKSrcBlt: zero_colorkey,
                ddpfPixelFormat: self.get_pixel_format(),
                ddsCaps: DDSCAPS {
                    dwCaps: (DDSCAPS_SYSTEMMEMORY | DDSCAPS_OFFSCREENPLAIN) as u32,
                },
            },
        }
    }
}

impl IUnknown_Trait for DirectDrawSurface {}

#[allow(non_snake_case)]
impl IDirectDrawSurface_Trait for DirectDrawSurface {
    fn Blt(
        &self,
        lpDestRect: MutPtr<RECT>,
        lpDDSrcSurface: IDirectDrawSurface,
        lpSrcRect: MutPtr<RECT>,
        dwFlags: u32,
        lpDDBltFx: MutPtr<DDBLTFX>,
    ) -> HRESULT {
        let ctx = self.memory_ctx;

        assert_eq!(dwFlags, 0, "Unsupported flags in Blt");

        let desc_rect = lpDestRect.read_with(ctx);
        let src_rect = lpSrcRect.read_with(ctx);
        let blt_fx = lpDDBltFx.read_with(ctx);

        assert_eq!(
            blt_fx.dwSize,
            DDBLTFX::size().try_into().unwrap(),
            "Size mismatch"
        );

        assert_eq!(blt_fx.dwDDFX, 0, "Unsupported Blt FX");

        let vtable_ptr = ConstPtr::<PtrRepr>::new(lpDDSrcSurface.0.raw_ptr());
        assert_eq!(
            vtable_ptr.read_with(ctx),
            self.direct_draw_surface_vtable,
            "Can't do blt on surfaces with different impls"
        );
        let repr_ptr = vtable_ptr.pun::<DirectDrawSurface_Repr>();
        let trait_reference = unsafe { &*repr_ptr.read_with(ctx).implementation };

        // let dst_surface: Option<&DirectDrawSurface> = trait_reference;

        todo!()
    }

    fn Lock(
        &self,
        lpDestRect: MutPtr<RECT>,
        lpDDSurfaceDesc: MutPtr<DDSURFACEDESC>,
        dwFlags: u32,
        hEvent: HANDLE,
    ) -> HRESULT {
        assert_eq!(
            lpDestRect,
            MutPtr::null(),
            "Locking rectangles not supported"
        );
        assert_eq!(hEvent, HANDLE(0), "Using Lock hEvent is not supported");
        assert_eq!(dwFlags as i32, DDLOCK_WAIT, "Unsupported Lock flags");

        // we don't do any "actual" locking here
        // this is because the locking can't __really__ be observed here
        let desc = match &self.surface {
            Surface::Onscreen(_) => {
                panic!("Attempt to lock an onscreen surface (not supported, at least yet)")
            }
            Surface::Offscreen(surface) => {
                let ptr = surface.holder.repr();

                let mut desc = self.get_surface_desc();
                desc.lpSurface = MutPtr::new(ptr);

                desc
            }
        };

        lpDDSurfaceDesc.write_with(self.memory_ctx, desc);

        S_OK
    }

    fn Unlock(&self, _lpSurfaceData: MutPtr<c_void>) -> HRESULT {
        // nothing to do here, actually
        S_OK
    }
}
