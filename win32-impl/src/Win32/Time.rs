use crate::ProcessContext;
use core_mem::ptr::MutPtr;
use core_time::TimeProvider;
use std::sync::Arc;
use tracing::warn;
use win32::Win32::System::Time::TIME_ZONE_INFORMATION;

pub struct Time {
    pub process_ctx: ProcessContext,
    pub time: Arc<TimeProvider>,
}

impl win32::Win32::System::Time::Api for Time {
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
