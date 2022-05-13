use crate::ProcessContext;

pub struct SystemInformation {
    pub process_ctx: ProcessContext,
}

#[allow(non_snake_case)]
impl win32::Win32::System::SystemInformation::Api for SystemInformation {
    fn GetVersion(&self) -> u32 {
        0x0ece0205 // (I think?) corresponds to windows 98
    }
}
