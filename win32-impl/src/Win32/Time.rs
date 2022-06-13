use crate::ProcessContext;
use core_mem::ptr::{ConstPtr, MutPtr};
use std::sync::Arc;
use tracing::warn;
use win32::Win32::Foundation::{BOOL, FILETIME, SYSTEMTIME};
use win32::Win32::System::Time::TIME_ZONE_INFORMATION;
use win32_time::Win32TimeProvider;

pub struct Time {
    pub process_ctx: ProcessContext,
    pub time: Arc<Win32TimeProvider>,
}

impl win32::Win32::System::Time::Api for Time {
    fn FileTimeToSystemTime(
        &self,
        lp_file_time: ConstPtr<FILETIME>,
        lp_system_time: MutPtr<SYSTEMTIME>,
    ) -> BOOL {
        let ctx = self.process_ctx.memory_ctx;

        lp_system_time.write_with(
            ctx,
            self.time
                .file_time_to_system_time(lp_file_time.read_with(ctx)),
        );

        BOOL(1)
    }

    fn GetTimeZoneInformation(
        &self,
        _lp_time_zone_information: MutPtr<TIME_ZONE_INFORMATION>,
    ) -> u32 {
        // let's ignore all the timezone stuff for now, as it's hard...
        warn!("Err-stub for GetTimeZoneInformation");

        // I don't know!!!!
        // TODO: SetLastError
        0xffffffff
    }
}
