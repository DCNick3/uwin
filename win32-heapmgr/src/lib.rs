use core_heap::{Heap, HeapOptions};
use core_mem::ptr::PtrRepr;
use core_memmgr::MemoryManager;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct HeapMgr {
    mem_mgr: Arc<Mutex<MemoryManager>>,
    heaps: HashMap<PtrRepr, Heap>,
}

impl HeapMgr {
    pub fn new(mem_mgr: Arc<Mutex<MemoryManager>>) -> Self {
        Self {
            mem_mgr,
            heaps: Default::default(),
        }
    }

    // TODO: mapping to win32 errors?
    // being infallible for now
    pub fn create(&mut self, initial_size: PtrRepr, maximum_size: PtrRepr) -> PtrRepr {
        fn size(v: PtrRepr) -> Option<PtrRepr> {
            if v == 0 {
                None
            } else {
                Some(v)
            }
        }

        let options = HeapOptions {
            initial_size: size(initial_size),
            maximum_size: size(maximum_size),
        };

        let mut mgr = self.mem_mgr.lock().unwrap();
        let heap = Heap::new(mgr.borrow_mut(), options).expect("Creating heap failed");

        let handle = heap.handle();

        assert!(self.heaps.insert(handle, heap).is_none());

        handle
    }
}
