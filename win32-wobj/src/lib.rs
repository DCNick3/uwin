use core_mem::ptr::PtrRepr;
use core_memmgr::{MemoryManager, RegionHolder};
use std::cmp::min;
use std::sync::{Arc, Mutex};

pub enum WindowsObject {
    Icon(),   // dummy
    Cursor(), // dummy
}

pub struct WindowsHandleTable {
    region: RegionHolder,
    table: Vec<Option<Box<WindowsObject>>>,
    start_free_search_idx: usize,
}

impl WindowsHandleTable {
    fn index_to_handle(&self, index: usize) -> PtrRepr {
        let region = self.region.region();
        assert!(index < region.size as usize);

        region.start + index as u32
    }

    fn handle_to_index(&self, handle: PtrRepr) -> usize {
        let region = self.region.region();
        assert!(region.contains_addr(handle));

        (handle - region.start) as usize
    }

    pub fn new(mgr: Arc<Mutex<MemoryManager>>, region_size: PtrRepr) -> Self {
        let region = {
            let mut mgr = mgr.lock().unwrap();
            mgr.reserve_dynamic(region_size).unwrap()
        };

        let region = RegionHolder::new(mgr, region);

        let mut table = Vec::new();
        table.resize_with(region.region().size as usize, || None);

        Self {
            region,
            table,
            start_free_search_idx: 0,
        }
    }

    pub fn put(&mut self, object: WindowsObject) -> PtrRepr {
        for (i, v) in self
            .table
            .iter_mut()
            .enumerate()
            .skip(self.start_free_search_idx as usize)
        {
            if v.is_none() {
                *v = Some(Box::new(object));
                self.start_free_search_idx = i;
                return self.index_to_handle(i);
            }
        }

        panic!(
            "Could not find free slots in handle table, doubling the size (size = {})",
            self.table.len()
        );
    }

    pub fn find(&self, handle: PtrRepr) -> Option<&WindowsObject> {
        self.table
            .get(self.handle_to_index(handle))?
            .as_ref()
            .map(|obj| obj.as_ref())
    }

    pub fn find_mut(&mut self, handle: PtrRepr) -> Option<&mut WindowsObject> {
        let index = self.handle_to_index(handle);
        self.table.get_mut(index)?.as_mut().map(|obj| obj.as_mut())
    }

    pub fn remove(&mut self, handle: PtrRepr) -> Option<Box<WindowsObject>> {
        let index = self.handle_to_index(handle);
        let obj = self.table.get_mut(index)?;
        obj.take().map(|obj| {
            self.start_free_search_idx = min(self.start_free_search_idx, index);
            obj
        })
    }
}
