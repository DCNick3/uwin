use core_mem::ptr::PtrRepr;
use std::cmp::min;
use std::sync::Arc;
use tracing::trace;
use win32::Win32::Foundation::HANDLE;

pub enum ConsoleType {
    Stdin,
    Stdout,
}

pub enum KernelObject {
    // TODO: actually store handler for writing/reading?
    Console(ConsoleType),
}

pub struct KernelHandleTable {
    table: Vec<Option<Arc<KernelObject>>>,
    start_free_search_idx: usize,
}

impl KernelHandleTable {
    pub fn new() -> Self {
        Self {
            table: vec![None; 16],
            start_free_search_idx: 0,
        }
    }

    fn handle_to_index(handle: HANDLE) -> usize {
        let handle = handle.0 as usize;
        (handle >> 2) - 10
    }

    fn index_to_handle(index: usize) -> HANDLE {
        let index = (index + 10) << 2;
        HANDLE(index as PtrRepr)
    }

    pub fn put(&mut self, object: Arc<KernelObject>) -> HANDLE {
        for (i, v) in self
            .table
            .iter_mut()
            .enumerate()
            .skip(self.start_free_search_idx as usize)
        {
            if let None = *v {
                *v = Some(object);
                self.start_free_search_idx = i;
                return Self::index_to_handle(i);
            }
        }

        // no free slot found, need  to expand the table
        trace!(
            "Could not find free slots in handle table, doubling the size (current size = {})",
            self.table.len()
        );

        self.table.resize(self.table.len() * 2, None);

        self.put(object)
    }

    pub fn find(&mut self, handle: HANDLE) -> Option<Arc<KernelObject>> {
        self.table.get(Self::handle_to_index(handle)).cloned()?
    }

    pub fn remove(&mut self, handle: HANDLE) -> Option<Arc<KernelObject>> {
        let index = Self::handle_to_index(handle);
        let obj = self.table.get_mut(index)?;
        obj.take().map(|obj| {
            self.start_free_search_idx = min(self.start_free_search_idx, index);
            obj
        })
    }
}

impl Default for KernelHandleTable {
    fn default() -> Self {
        KernelHandleTable::new()
    }
}

#[cfg(test)]
mod test {
    use crate::{ConsoleType, KernelHandleTable, KernelObject};
    use std::sync::Arc;

    fn newobj() -> Arc<KernelObject> {
        Arc::new(KernelObject::Console(ConsoleType::Stdout))
    }

    #[test]
    fn test() {
        let mut table = KernelHandleTable::new();

        assert_eq!(table.table.len(), 16);

        let mut handles = Vec::new();
        for _ in 0..17 {
            handles.push(table.put(newobj()));
        }

        println!("Allocated handles: {:?}", handles);

        for &handle in handles.iter() {
            assert!(table.find(handle).is_some());
        }

        // handle table was resized!
        assert_eq!(table.table.len(), 32);

        assert!(table.remove(handles[0]).is_some());

        // handle was reused!
        assert_eq!(table.put(newobj()), handles[0]);
    }
}
