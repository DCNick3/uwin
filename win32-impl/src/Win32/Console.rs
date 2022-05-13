use crate::ProcessContext;
use win32::Win32::Foundation::{HANDLE, INVALID_HANDLE_VALUE};
use win32::Win32::System::Console::{
    STD_ERROR_HANDLE, STD_HANDLE, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE,
};

pub struct Console {
    pub process_ctx: ProcessContext,
    pub stdin_handle: HANDLE,
    pub stdout_handle: HANDLE,
    pub stderr_handle: HANDLE,
}

#[allow(non_snake_case)]
impl win32::Win32::System::Console::Api for Console {
    fn GetStdHandle(&self, n_std_handle: STD_HANDLE) -> HANDLE {
        if n_std_handle == STD_INPUT_HANDLE {
            self.stdin_handle
        } else if n_std_handle == STD_OUTPUT_HANDLE {
            self.stdout_handle
        } else if n_std_handle == STD_ERROR_HANDLE {
            self.stderr_handle
        } else {
            INVALID_HANDLE_VALUE
        }
    }
}
