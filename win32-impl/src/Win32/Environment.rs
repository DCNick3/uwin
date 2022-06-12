use crate::ProcessContext;
use core_heap::RawHeapBox;
use core_str::heap_helper::AnsiStringHeapBox;
use core_str::{AnsiString, PCSTR, PSTR, PWSTR};
use std::sync::Arc;
use win32::Win32::Foundation::BOOL;
use win32_fs::WindowsFsManager;

pub struct Environment {
    pub process_ctx: ProcessContext,
    pub command_line_ansi: AnsiStringHeapBox,
    pub environment_strings_oem: Vec<u8>,
    pub fs_manager: Arc<WindowsFsManager>,
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

    fn GetCurrentDirectoryA(&self, n_buffer_length: u32, lp_buffer: PSTR) -> u32 {
        let ctx = self.process_ctx.memory_ctx;

        let current_directory = self.fs_manager.get_current_directory();
        let current_directory =
            AnsiString::from_utf8(&current_directory, self.process_ctx.ansi_encoding);

        let required_buffer_size = (current_directory.len() + 1) as u32;

        if n_buffer_length < required_buffer_size {
            // If the buffer that is pointed to by lpBuffer is not large enough, the return value
            //      specifies the required size of the buffer, in characters, including the null-terminating character.
            return required_buffer_size;
        } else {
            lp_buffer.write_with(ctx, n_buffer_length, &current_directory);

            // If the function succeeds, the return value specifies the number of characters
            //      that are written to the buffer, not including the terminating null character.
            current_directory.len() as u32
        }
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
