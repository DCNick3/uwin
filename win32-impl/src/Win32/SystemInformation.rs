use crate::ProcessContext;
use core_mem::ptr::MutPtr;
use core_time::{SystemTime, TimeProvider};
use std::sync::Arc;
use win32::Win32::Foundation::SYSTEMTIME;
pub struct SystemInformation {
    pub process_ctx: ProcessContext,
    pub time: Arc<TimeProvider>,
}

// instead of defining a win32-time crate, we do the type conversions right in the win32-impl
// When should this be okay?
fn convert_system_time(time: SystemTime) -> SYSTEMTIME {
    SYSTEMTIME {
        wYear: time.year,
        wMonth: time.month,
        wDayOfWeek: time.day_of_week,
        wDay: time.day,
        wHour: time.hour,
        wMinute: time.minute,
        wSecond: time.second,
        wMilliseconds: time.millisecond,
    }
}

#[allow(non_snake_case)]
impl win32::Win32::System::SystemInformation::Api for SystemInformation {
    fn GetLocalTime(&self, lp_system_time: MutPtr<SYSTEMTIME>) {
        let ctx = self.process_ctx.memory_ctx;

        let time = self.time.get_local_time();

        lp_system_time.write_with(ctx, convert_system_time(time));
    }

    fn GetSystemTime(&self, lp_system_time: MutPtr<SYSTEMTIME>) {
        let ctx = self.process_ctx.memory_ctx;

        let time = self.time.get_system_time();

        lp_system_time.write_with(ctx, convert_system_time(time));
    }

    fn GetVersion(&self) -> u32 {
        0x0ece0205 // (I think?) corresponds to windows 98
    }
}
