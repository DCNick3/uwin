use core_heap::{Heap, HeapOptions};
use core_mem::ptr::PtrRepr;
use core_memmgr::MemoryManager;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use win32::Win32::System::Memory::{HEAP_FLAGS, HEAP_ZERO_MEMORY};

pub struct HeapMgr {
    mem_mgr: Arc<Mutex<MemoryManager>>,
    heaps: HashMap<PtrRepr, Mutex<Heap>>,
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

        assert!(self.heaps.insert(handle, Mutex::new(heap)).is_none());

        handle
    }

    pub fn alloc(&self, heap_handle: PtrRepr, size: PtrRepr, flags: HEAP_FLAGS) -> PtrRepr {
        assert_eq!(flags & !HEAP_ZERO_MEMORY, HEAP_FLAGS(0));

        let zero = flags & HEAP_ZERO_MEMORY != HEAP_FLAGS(0); // TODO: codegen an operator/function to test flags

        let heap = self.heaps.get(&heap_handle).expect("Unknown heap used");
        let mut heap = heap.lock().unwrap();

        let res = heap
            .alloc(&*self.mem_mgr, size, zero)
            .expect("Allocation failed");

        res
    }
}
