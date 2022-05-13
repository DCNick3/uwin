use crate::ProcessContext;
use core_mem::ptr::MutPtr;
use tracing::warn;
use win32::Win32::Foundation::BOOL;
use win32::Win32::Globalization::CPINFO;

pub struct Globalization {
    pub process_ctx: ProcessContext,
}

#[allow(non_snake_case)]
impl win32::Win32::Globalization::Api for Globalization {
    fn GetACP(&self) -> u32 {
        1251 // report CP1251... Just because
    }

    fn GetCPInfo(&self, _code_page: u32, _lp_cp_info: MutPtr<CPINFO>) -> BOOL {
        warn!("Returning an error from GetCPInfo (not implemented yet :shrug:)");
        BOOL(0)
    }
}
