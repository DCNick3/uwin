use crate::ProcessContext;
use core_str::{AnsiString, PCSTR, PSTR};
use win32::Win32::Foundation::HINSTANCE;
use win32_module_table::ModuleTable;

pub struct LibraryLoader {
    pub process_ctx: ProcessContext,
    pub module_table: ModuleTable,
}

#[allow(non_snake_case)]
impl win32::Win32::System::LibraryLoader::Api for LibraryLoader {
    fn GetModuleFileNameA(&self, mut h_module: HINSTANCE, lp_filename: PSTR, n_size: u32) -> u32 {
        if h_module == HINSTANCE(0) {
            h_module = self.module_table.get_main_program_handle();
        }
        let name = self.module_table.get_handle_file_name(h_module);
        let name = match name {
            None => return 0,
            Some(name) => name,
        };

        let ansi_str = AnsiString::from_utf8(name, self.process_ctx.ansi_encoding);
        // TODO: handle overflows gracefully
        lp_filename.write_with(self.process_ctx.memory_ctx, n_size, &ansi_str);

        ansi_str.len() as u32
    }

    fn GetModuleHandleA(&self, lp_module_name: PCSTR) -> HINSTANCE {
        if lp_module_name.0.repr() == 0 {
            return self.module_table.get_main_program_handle();
        }
        todo!("GetModuleHandleA")
    }

    fn LoadLibraryA(&self, _lp_lib_file_name: PCSTR) -> HINSTANCE {
        HINSTANCE(0)
    }
}
