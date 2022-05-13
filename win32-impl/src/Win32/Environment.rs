use crate::ProcessContext;
use core_heap::RawHeapBox;
use core_str::heap_helper::AnsiStringHeapBox;
use core_str::{PCSTR, PSTR, PWSTR};
use win32::Win32::Foundation::BOOL;

pub struct Environment {
    pub process_ctx: ProcessContext,
    pub command_line_ansi: AnsiStringHeapBox,
    pub environment_strings_oem: Vec<u8>,
}

#[allow(non_snake_case)]
impl win32::Win32::System::Environment::Api for Environment {
    fn FreeEnvironmentStringsA(&self, penv: PCSTR) -> BOOL {
        let mut heap = self.process_ctx.process_heap.lock().unwrap();

        heap.free(penv.0.repr()).unwrap();

        BOOL(1)
    }

    fn GetCommandLineA(&self) -> PSTR {
        self.command_line_ansi.ptr_mut()
    }

    fn GetEnvironmentStrings(&self) -> PSTR {
        let res = RawHeapBox::new_init(
            self.process_ctx.memory_ctx,
            self.process_ctx.process_heap.clone(),
            &self.environment_strings_oem,
        )
        .unwrap();

        PSTR::new(res.leak())
    }

    // report no unicode support, like on Windows 9x
    fn GetEnvironmentStringsW(&self) -> PWSTR {
        PWSTR::new(0)
    }
}
