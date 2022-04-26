use std::collections::HashMap;
use win32::Win32::Foundation::HINSTANCE;

pub struct ModuleTable {
    name_to_handle: HashMap<String, HINSTANCE>,
    handle_to_name: HashMap<HINSTANCE, String>,
    main_program_handle: HINSTANCE,
}

impl ModuleTable {
    pub fn new(
        value: impl Iterator<Item = (String, HINSTANCE)> + Clone,
        main_program_handle: HINSTANCE,
    ) -> Self {
        Self {
            name_to_handle: value.clone().collect(),
            handle_to_name: value.map(|(nm, h)| (h, nm)).collect(),
            main_program_handle,
        }
    }

    pub fn get_main_program_handle(&self) -> HINSTANCE {
        self.main_program_handle
    }

    pub fn get_module_handle(&self, filename: &str) -> Option<HINSTANCE> {
        self.name_to_handle.get(filename).cloned()
    }

    pub fn get_handle_file_name(&self, handle: HINSTANCE) -> Option<&str> {
        self.handle_to_name.get(&handle).map(|nm| nm.as_str())
    }
}
