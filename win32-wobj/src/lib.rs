use core_handletable::{Handle, HandleTable};
use core_mem::ptr::PtrRepr;
use core_memmgr::{MemoryManager, RegionHolder};
use std::sync::{Arc, Mutex};

pub enum WindowsObject {
    Icon(),   // dummy
    Cursor(), // dummy
}

#[derive(Clone, Copy)]
struct HandleWrapper(PtrRepr);

impl Handle<RegionHolder> for HandleWrapper {
    fn to_index(self, region: &RegionHolder) -> usize {
        let region = region.region();
        let handle = self.into();
        assert!(region.contains_addr(handle));

        (handle - region.start) as usize
    }

    fn from_index(index: usize, region: &RegionHolder) -> Self {
        let region = region.region();
        assert!(index < region.size as usize);

        (region.start + index as u32).into()
    }
}

impl From<PtrRepr> for HandleWrapper {
    fn from(handle: PtrRepr) -> Self {
        Self(handle)
    }
}

impl From<HandleWrapper> for PtrRepr {
    fn from(handle: HandleWrapper) -> Self {
        handle.0
    }
}

pub struct WindowsHandleTable {
    table: HandleTable<RegionHolder, HandleWrapper, WindowsObject, true>,
}

impl WindowsHandleTable {
    pub fn new(mgr: Arc<Mutex<MemoryManager>>, region_size: PtrRepr) -> Self {
        let region = {
            let mut mgr = mgr.lock().unwrap();
            mgr.reserve_dynamic(region_size).unwrap()
        };

        let region = RegionHolder::new(mgr, region);

        let table = HandleTable::new(region, 16);

        Self { table }
    }

    pub fn put(&mut self, object: WindowsObject) -> PtrRepr {
        self.table.put(object).into()
    }

    pub fn find(&self, handle: PtrRepr) -> Option<&WindowsObject> {
        self.table.find(handle.into())
    }

    pub fn find_mut(&mut self, handle: PtrRepr) -> Option<&mut WindowsObject> {
        self.table.find_mut(handle.into())
    }

    pub fn remove(&mut self, handle: PtrRepr) -> Option<WindowsObject> {
        self.table.remove(handle.into())
    }
}
