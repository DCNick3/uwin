use crate::ProcessContext;
use tracing::warn;
use win32::Win32::Foundation::{BOOL, HWND};

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
