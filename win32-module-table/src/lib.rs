use recompiler::ModuleExports;
use std::collections::HashMap;
use std::sync::Arc;
use win32::Win32::Foundation::HINSTANCE;

pub struct Module {
    pub name: String,
    pub handle: HINSTANCE,
    pub exports: ModuleExports,
}

pub struct ModuleTable {
    name_to_module: HashMap<String, Arc<Module>>,
    handle_to_module: HashMap<HINSTANCE, Arc<Module>>,
    main_program_handle: HINSTANCE,
}

impl ModuleTable {
    pub fn new(modules: impl Iterator<Item = Module>, main_program_handle: HINSTANCE) -> Self {
        let mut name_to_module = HashMap::new();
        let mut handle_to_module = HashMap::new();

        for module in modules {
            let module = Arc::new(module);
            name_to_module.insert(module.name.clone(), module.clone());
            handle_to_module.insert(module.handle, module);
        }

        Self {
            name_to_module,
            handle_to_module,
            main_program_handle,
        }
    }

    pub fn get_main_program_handle(&self) -> HINSTANCE {
        self.main_program_handle
    }

    pub fn get_module_by_name(&self, filename: &str) -> Option<&Module> {
        self.name_to_module.get(filename).map(|m| m.as_ref())
    }

    pub fn get_module_by_handle(&self, handle: HINSTANCE) -> Option<&Module> {
        self.handle_to_module.get(&handle).map(|m| m.as_ref())
    }
}
