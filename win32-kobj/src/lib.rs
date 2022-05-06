use core_console::Console;
use core_handletable::{Handle, HandleTable};
use core_mem::ptr::PtrRepr;
use std::sync::Arc;
use win32::Win32::Foundation::HANDLE;

#[non_exhaustive]
pub enum KernelObject {
    // TODO: actually store handler for writing/reading?
    Console(Arc<dyn Console>),
}

#[derive(Clone, Copy)]
struct HANDLE_(PtrRepr);

impl Handle for HANDLE_ {
    fn to_index(self, _: &()) -> usize {
        let handle: usize = self.0.try_into().unwrap();
        (handle >> 2) - 10
    }

    fn from_index(index: usize, _: &()) -> Self {
        let index = (index + 10) << 2;
        Self(index.try_into().unwrap())
    }
}

impl From<HANDLE_> for HANDLE {
    fn from(handle: HANDLE_) -> Self {
        Self(handle.0)
    }
}

impl From<HANDLE> for HANDLE_ {
    fn from(handle: HANDLE) -> Self {
        Self(handle.0)
    }
}

pub struct KernelHandleTable {
    inner: HandleTable<(), HANDLE_, Arc<KernelObject>, true>,
}

impl KernelHandleTable {
    pub fn new() -> Self {
        Self {
            inner: HandleTable::new((), 16),
        }
    }

    pub fn put(&mut self, object: Arc<KernelObject>) -> HANDLE {
        self.inner.put(object).into()
    }

    pub fn find(&self, handle: HANDLE) -> Option<Arc<KernelObject>> {
        self.inner.find(handle.into()).cloned()
    }

    pub fn remove(&mut self, handle: HANDLE) -> Option<Arc<KernelObject>> {
        self.inner.remove(handle.into())
    }
}

impl Default for KernelHandleTable {
    fn default() -> Self {
        KernelHandleTable::new()
    }
}

#[cfg(test)]
mod test {
    use crate::{KernelHandleTable, KernelObject};
    use core_console::DummyConsole;
    use std::sync::Arc;

    fn new_obj() -> Arc<KernelObject> {
        Arc::new(KernelObject::Console(Arc::new(DummyConsole {})))
    }

    #[test]
    fn test() {
        let mut table = KernelHandleTable::new();

        assert_eq!(table.inner.capacity(), 16);

        let mut handles = Vec::new();
        for _ in 0..17 {
            handles.push(table.put(new_obj()));
        }

        println!("Allocated handles: {:?}", handles);

        for &handle in handles.iter() {
            assert!(table.find(handle).is_some());
        }

        // handle table was resized!
        assert_eq!(table.inner.capacity(), 32);

        assert!(table.remove(handles[0]).is_some());

        // handle was reused!
        assert_eq!(table.put(new_obj()), handles[0]);
    }
}
