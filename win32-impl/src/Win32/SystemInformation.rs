use crate::ProcessContext;
use core_mem::ptr::MutPtr;
use std::sync::Arc;
use win32::Win32::Foundation::SYSTEMTIME;
use win32_time::Win32TimeProvider;

pub struct SystemInformation {
    pub process_ctx: ProcessContext,
    pub time: Arc<Win32TimeProvider>,
}

#[allow(non_snake_case)]
impl win32::Win32::System::SystemInformation::Api for SystemInformation {
    fn GetLocalTime(&self, lp_system_time: MutPtr<SYSTEMTIME>) {
        let ctx = self.process_ctx.memory_ctx;

        let time = self.time.get_local_time();

        lp_system_time.write_with(ctx, time);
    }

    fn GetSystemTime(&self, lp_system_time: MutPtr<SYSTEMTIME>) {
        let ctx = self.process_ctx.memory_ctx;

        let time = self.time.get_system_time();

        lp_system_time.write_with(ctx, time);
    }

    fn GetVersion(&self) -> u32 {
        0x0ece0205 // (I think?) corresponds to windows 98
    }
}
