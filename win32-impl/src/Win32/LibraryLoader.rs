use crate::ProcessContext;
use core_str::{AnsiString, PCSTR, PSTR};
use win32::Win32::Foundation::{FARPROC, HINSTANCE};
use win32_module_table::ModuleTable;

pub struct LibraryLoader {
    pub process_ctx: ProcessContext,
    pub module_table: ModuleTable,
}

impl LibraryLoader {
    /// Maps a module handle to the actual module handle. If the handle is 0, it is mapped to the main module handle.
    fn map_module_handle(&self, h_module: HINSTANCE) -> HINSTANCE {
        if h_module == HINSTANCE(0) {
            self.module_table.get_main_program_handle()
        } else {
            h_module
        }
    }
}

#[allow(non_snake_case)]
impl win32::Win32::System::LibraryLoader::Api for LibraryLoader {
    fn GetModuleFileNameA(&self, h_module: HINSTANCE, lp_filename: PSTR, n_size: u32) -> u32 {
        let h_module = self.map_module_handle(h_module);

        let name = self
            .module_table
            .get_module_by_handle(h_module)
            .map(|m| m.name.as_str());
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

        // TODO: this is definitely too many characters for the API
        let lp_module_name = lp_module_name
            .to_utf8(self.process_ctx.memory_ctx, self.process_ctx.ansi_encoding)
            .to_ascii_lowercase();

        if let Some(result) = self
            .module_table
            .get_module_by_name(&lp_module_name)
            .or_else(|| {
                let name_with_dll = format!("{}.dll", lp_module_name);
                self.module_table.get_module_by_name(&name_with_dll)
            })
            .map(|module| module.handle)
        {
            return result;
        } else {
            panic!("GetModuleHandleA: module not found: {}", lp_module_name);
        }
    }

    fn GetProcAddress(&self, h_module: HINSTANCE, lp_proc_name: PCSTR) -> FARPROC {
        let h_module = self.map_module_handle(h_module);

        let repr = lp_proc_name.0.repr();
        if repr < 0x10000 {
            todo!("GetProcAddress by ordinal")
        }

        let lp_proc_name =
            lp_proc_name.to_utf8(self.process_ctx.memory_ctx, self.process_ctx.ansi_encoding);

        if let Some(module) = self.module_table.get_module_by_handle(h_module) {
            let proc = module.exports.by_name.get(&lp_proc_name).cloned();
            if let Some(proc) = proc {
                return FARPROC::new(proc);
            } else {
                panic!("GetProcAddress: procedure not found: {}", lp_proc_name);
            }
        } else {
            panic!("GetProcAddress: module not found: {:?}", h_module);
        }
    }

    fn LoadLibraryA(&self, _lp_lib_file_name: PCSTR) -> HINSTANCE {
        HINSTANCE(0)
    }
}
