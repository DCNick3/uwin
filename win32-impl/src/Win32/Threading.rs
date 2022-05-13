use crate::ProcessContext;
use core_abi::unwind_token::{UnwindReason, UnwindToken};
use core_mem::ptr::MutPtr;
use core_str::PSTR;
use win32::Win32::System::Threading::STARTUPINFOA;

pub struct Threading {
    pub process_ctx: ProcessContext,
}

#[allow(non_snake_case)]
impl win32::Win32::System::Threading::Api for Threading {
    fn ExitProcess(&self, unwind_token: &mut UnwindToken, u_exit_code: u32) {
        unwind_token.unwind(UnwindReason::ExitProcess(u_exit_code));
    }

    fn GetStartupInfoA(&self, lp_startup_info: MutPtr<STARTUPINFOA>) {
        lp_startup_info.write_with(
            self.process_ctx.memory_ctx,
            STARTUPINFOA {
                cb: 68,
                lpReserved: PSTR(MutPtr::new(0)),
                lpDesktop: PSTR(MutPtr::new(0)),
                lpTitle: PSTR(MutPtr::new(0)),
                dwX: 0,
                dwY: 0,
                dwXSize: 0,
                dwYSize: 0,
                dwXCountChars: 0,
                dwYCountChars: 0,
                dwFillAttribute: 0,
                dwFlags: Default::default(),
                wShowWindow: 0,
                cbReserved2: 0,
                lpReserved2: MutPtr::new(0),
                hStdInput: Default::default(),
                hStdOutput: Default::default(),
                hStdError: Default::default(),
            },
        )
    }
}
