use bimap::BiMap;
use core_handletable::{Handle, HandleTable};
use core_message_queue::Sender;
use core_windows::{WindowCreation, WindowId, WindowsContext};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use win32::core::prelude::{PtrDiffRepr, PtrRepr};
use win32::Win32::Foundation::HWND;
use win32::Win32::UI::WindowsAndMessaging::WNDPROC;
use win32_atoms::AtomTable;

pub struct WindowClass {
    pub name: String,
    pub wndproc: WNDPROC,
}

pub struct Window {
    pub window_id: WindowId,
    pub class: Arc<WindowClass>,
    #[allow(unused)]
    pub wndproc_argument: PtrRepr,
    pub size: (u32, u32),
}

impl Window {
    pub fn class(&self) -> &Arc<WindowClass> {
        &self.class
    }

    pub fn wndproc(&self) -> WNDPROC {
        self.class.wndproc
    }
}

pub struct ClassRegistry {
    user_atom_table: Arc<Mutex<AtomTable>>,
    classes: BTreeMap<u16, Arc<WindowClass>>,
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
        assert!(self.classes.insert(atom, Arc::new(class)).is_none());

        atom
    }

    pub fn find(&mut self, name: &str) -> Option<Arc<WindowClass>> {
        let atom_table = self.user_atom_table.lock().unwrap();
        let atom = atom_table.find_atom(name)?;
        self.classes.get(&atom).cloned()
    }
}

#[derive(Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq)]
struct HWND_(PtrRepr);

impl Handle for HWND_ {
    fn to_index(self, _: &()) -> usize {
        let index: usize = self.0.try_into().unwrap();
        index.checked_sub(10).unwrap()
    }

    fn from_index(index: usize, _: &()) -> Self {
        let value: PtrRepr = index.try_into().unwrap();
        HWND_(value.checked_add(10).unwrap())
    }
}

impl From<HWND> for HWND_ {
    fn from(hwnd: HWND) -> Self {
        Self(hwnd.0 as PtrRepr)
    }
}

impl From<HWND_> for HWND {
    fn from(hwnd: HWND_) -> Self {
        Self(hwnd.0 as PtrDiffRepr)
    }
}

pub struct WindowsRegistry {
    windows: HandleTable<(), HWND_, Arc<Window>, true>,
    hwnd_and_window_id_assoc: BiMap<HWND_, WindowId>,
    windows_context: WindowsContext,
}

impl WindowsRegistry {
    pub fn new() -> Self {
        Self {
            windows: HandleTable::new((), 16),
            hwnd_and_window_id_assoc: BiMap::new(),
            windows_context: WindowsContext::new(),
        }
    }

    pub fn create(
        &mut self,
        class: Arc<WindowClass>,
        wndproc_argument: PtrRepr,
        size: (u32, u32),
        message_queue: Sender,
    ) -> HWND {
        let window_id = self.windows_context.create_window(WindowCreation {
            message_queue,
            size,
        });

        let window = Window {
            window_id,
            class,
            wndproc_argument,
            size,
        };

        let hwnd = self.windows.put(Arc::new(window));

        assert!(!self
            .hwnd_and_window_id_assoc
            .insert(hwnd, window_id)
            .did_overwrite());

        hwnd.into()
    }

    pub fn find(&self, handle: HWND) -> Option<Arc<Window>> {
        self.windows.find(handle.into()).cloned()
    }

    pub fn core_windows_context(&self) -> &WindowsContext {
        &self.windows_context
    }

    pub fn window_id_to_hwnd(&self, window_id: WindowId) -> Option<HWND> {
        self.hwnd_and_window_id_assoc
            .get_by_right(&window_id)
            .map(|&hwnd| hwnd.into())
    }
}

impl Default for WindowsRegistry {
    fn default() -> Self {
        WindowsRegistry::new()
    }
}
