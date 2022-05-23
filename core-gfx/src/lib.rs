use core_heap::{Heap, RawHeapBox};
use core_mem::ptr::PtrRepr;
use pixels::raw_window_handle::HasRawWindowHandle;
use pixels::wgpu::util::power_preference_from_env;
use pixels::wgpu::{PowerPreference, PresentMode, RequestAdapterOptions};
use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use std::sync::{Arc, Mutex};

#[derive(PartialEq, Eq, Debug)]
pub enum SurfaceFormat {
    Rgb565,
}

impl SurfaceFormat {
    pub fn bytes_per_pixel(&self) -> PtrRepr {
        match self {
            SurfaceFormat::Rgb565 => 2,
        }
    }
}

pub struct OffscreenSurface {
    pub holder: RawHeapBox,
    pub format: SurfaceFormat,
    pub width: PtrRepr,
    pub pitch: PtrRepr,
    pub height: PtrRepr,
}

pub struct OnscreenSurface {
    pixels: Mutex<Pixels>,
}

pub enum Surface {
    Onscreen(OnscreenSurface),
    Offscreen(OffscreenSurface),
}

pub struct GfxContext {
    heap: Arc<Mutex<Heap>>,
}

impl GfxContext {
    pub fn new(heap: Arc<Mutex<Heap>>) -> Self {
        GfxContext { heap }
    }

    pub fn create_onscreen(
        &self,
        width: PtrRepr,
        height: PtrRepr,
        screen: &impl HasRawWindowHandle,
    ) -> Surface {
        let surface = SurfaceTexture::new(width, height, screen);

        let mut pixels = PixelsBuilder::new(width, height, surface)
            .present_mode(PresentMode::Mailbox)
            .request_adapter_options(RequestAdapterOptions {
                // request low power adapter by default, but allow overriding
                power_preference: power_preference_from_env().unwrap_or(PowerPreference::LowPower),
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .build()
            .expect("Build pixels");

        pixels.get_frame().fill(0);
        pixels
            .render()
            .expect("Rendering the first frame (all zeros)");

        let surface = OnscreenSurface {
            pixels: Mutex::new(pixels),
        };

        Surface::Onscreen(surface)
    }

    pub fn create_offscreen(
        &self,
        width: PtrRepr,
        height: PtrRepr,
        format: SurfaceFormat,
    ) -> Surface {
        assert_eq!(format, SurfaceFormat::Rgb565);
        let pitch = width;

        let size = pitch * height * format.bytes_per_pixel();

        let holder = RawHeapBox::new_zeroed(self.heap.clone(), size)
            .expect("Allocating memory for the surface");

        Surface::Offscreen(OffscreenSurface {
            holder,
            format,
            width,
            pitch,
            height,
        })
    }
}
