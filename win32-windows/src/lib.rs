use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use win32::Win32::UI::WindowsAndMessaging::WNDPROC;
use win32_atoms::AtomTable;

pub struct WindowClass {
    pub name: String,
    pub wndproc: WNDPROC,
}

pub struct ClassRegistry {
    user_atom_table: Arc<Mutex<AtomTable>>,
    classes: BTreeMap<u16, WindowClass>,
}

impl ClassRegistry {
    pub fn new(user_atom_table: Arc<Mutex<AtomTable>>) -> Self {
        Self {
            user_atom_table,
            classes: BTreeMap::new(),
        }
    }

    pub fn register(&mut self, class: WindowClass) -> u16 /* ATOM */ {
        let mut atom_table = self.user_atom_table.lock().unwrap();

        // TODO: currently we ignore the hInstance parameter which is used to identify the window class (along with the class name)
        // (see https://web.archive.org/web/20190315125554/https://devblogs.microsoft.com/oldnewthing/20050418-59/?p=35873)
        let atom = atom_table.add(&class.name);
        assert!(self.classes.insert(atom, class).is_none());

        atom
    }
}
