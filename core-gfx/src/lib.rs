use core_heap::{Heap, RawHeapBox};
use core_mem::ctx::MemoryCtx;
use core_mem::ptr::{PtrDiffRepr, PtrRepr};
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
    pixels: Pixels,
    size: (u32, u32),
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn from_point_and_size((x, y): (i32, i32), (width, height): (u32, u32)) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn fits_inside(&self, size: (u32, u32)) -> bool {
        self.x >= 0
            && self.y >= 0
            && (self.x as u32 + self.width) <= size.0
            && (self.y as u32 + self.height) <= size.1
    }
}

pub enum Surface {
    Onscreen(OnscreenSurface),
    Offscreen(OffscreenSurface),
}

impl Surface {
    pub fn size(&self) -> (u32, u32) {
        match self {
            Surface::Onscreen(surface) => surface.size,
            Surface::Offscreen(surface) => (surface.width, surface.height),
        }
    }

    // pitch is in bytes!
    pub fn pitch(&self) -> u32 {
        match self {
            Surface::Onscreen(_) => {
                unimplemented!() // we don't __really__ expose this
            }
            Surface::Offscreen(surface) => surface.pitch,
        }
    }

    pub fn bit_blit(
        &mut self,
        memory_ctx: impl MemoryCtx,
        dst_rect: Option<Rect>,
        src: &Surface,
        src_rect: Option<Rect>,
    ) {
        let dst_rect = dst_rect.unwrap_or_else(|| Rect::from_point_and_size((0, 0), self.size()));
        let src_rect = src_rect.unwrap_or_else(|| Rect::from_point_and_size((0, 0), src.size()));

        assert_eq!(
            dst_rect.size(),
            src_rect.size(),
            "Blit with unmatched sizes"
        );
        assert!(
            dst_rect.fits_inside(self.size()),
            "destination rect out of bounds"
        );
        assert!(
            src_rect.fits_inside(src.size()),
            "source rect out of bounds"
        );

        match (self, src) {
            (Surface::Onscreen(dst), Surface::Offscreen(src)) => {
                assert_eq!(src.format, SurfaceFormat::Rgb565);

                let src_frame = src.holder.ptr::<u16>();

                let (dst_width, _) = dst.size;

                let dst_frame = dst.pixels.get_frame();
                for (j, dst_row) in dst_frame
                    .chunks_exact_mut((dst_width * 4) as _ /* RGBA */)
                    .enumerate()
                    .skip((dst_rect.y as u32 * dst_width * 4) as _)
                    .take((dst_rect.height * dst_width * 4) as _)
                {
                    for (i, dst_ptr) in dst_row
                        .chunks_exact_mut(4 /* RGBA */)
                        .enumerate()
                        .skip((dst_rect.x as u32 * 4) as _)
                        .take((dst_rect.width * 4) as _)
                    {
                        let j: PtrDiffRepr = j.try_into().unwrap();
                        let i: PtrDiffRepr = i.try_into().unwrap();

                        let src_ptr = src_frame.offset_bytes(j * src.pitch as i32 + i * 2);

                        let src_val = src_ptr.read_with(memory_ctx);
                        // we got an RGB565 value
                        // now convert it to RGBA8888

                        let src_r = (src_val >> 11) & 0b11111; // 5 bits
                        let src_g = (src_val >> 5) & 0b111111; // 6 bits
                        let src_b = (src_val >> 0) & 0b11111; // 5 bits

                        // using values from https://stackoverflow.com/questions/2442576/how-does-one-convert-16-bit-rgb565-to-24-bit-rgb888

                        let dst_r = ((src_r as u32 * 527 + 23) >> 6) as u8;
                        let dst_g = ((src_g as u32 * 259 + 33) >> 6) as u8;
                        let dst_b = ((src_b as u32 * 527 + 23) >> 6) as u8;
                        let dst_val = [dst_r, dst_g, dst_b, 0xff];

                        dst_ptr.clone_from_slice(&dst_val);
                    }
                }

                dst.pixels
                    .render()
                    .expect("Rendering after a blit onto the onscreen surface");
            }
            (Surface::Offscreen(dst), Surface::Offscreen(src)) => {
                let src_frame = src.holder.ptr::<u16>();
                let dst_frame = dst.holder.ptr_mut::<u16>();

                for j in 0..dst_rect.height {
                    let src_row =
                        src_frame.offset_bytes(((src_rect.y as u32 + j) * src.pitch) as _);
                    let dst_row =
                        dst_frame.offset_bytes(((dst_rect.y as u32 + j) * dst.pitch) as _);
                    for i in 0..dst_rect.width {
                        let src_ptr = src_row.offset((src_rect.x as u32 + i) as _);

                        let val = src_ptr.read_with(memory_ctx);

                        let dst_ptr = dst_row.offset((dst_rect.x as u32 + i) as _);

                        dst_ptr.write_with(memory_ctx, val);
                    }
                }
            }
            _ => {
                todo!("Unsupported surface combination for blit")
            }
        }
    }
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
            pixels,
            size: (width as _, height as _),
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
        let pitch = width * 2; // it in bytes!

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
