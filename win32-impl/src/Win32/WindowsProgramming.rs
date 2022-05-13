use crate::ProcessContext;

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
