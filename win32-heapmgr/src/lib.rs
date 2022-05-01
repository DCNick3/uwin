use core_heap::{Heap, HeapOptions};
use core_mem::ptr::PtrRepr;
use core_memmgr::MemoryManager;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use win32::Win32::Foundation::BOOL;
use win32::Win32::System::Memory::{HeapHandle, HEAP_FLAGS, HEAP_ZERO_MEMORY};

pub struct HeapMgr {
    mem_mgr: Arc<Mutex<MemoryManager>>,
    heaps: HashMap<HeapHandle, Arc<Mutex<Heap>>>,
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
    pub fn create(&mut self, initial_size: PtrRepr, maximum_size: PtrRepr) -> HeapHandle {
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

        let heap = Heap::new(self.mem_mgr.clone(), options).expect("Creating heap failed");
        let handle = HeapHandle(heap.handle());

        assert!(self
            .heaps
            .insert(handle, Arc::new(Mutex::new(heap)))
            .is_none());

        handle
    }

    pub fn get_heap(&self, heap_handle: HeapHandle) -> Arc<Mutex<Heap>> {
        self.heaps
            .get(&heap_handle)
            .expect("Tried to get unknown heap")
            .clone()
    }

    pub fn alloc(&self, heap_handle: HeapHandle, size: PtrRepr, flags: HEAP_FLAGS) -> PtrRepr {
        assert_eq!(flags & !HEAP_ZERO_MEMORY, HEAP_FLAGS(0));

        let zero = flags & HEAP_ZERO_MEMORY != HEAP_FLAGS(0); // TODO: codegen an operator/function to test flags

        let heap = self.heaps.get(&heap_handle).expect("Unknown heap used");
        let mut heap = heap.lock().unwrap();

        heap.alloc(size, zero).expect("Allocation failed")
    }

    pub fn free(&self, heap_handle: HeapHandle, ptr: PtrRepr, flags: HEAP_FLAGS) -> BOOL {
        assert_eq!(flags, HEAP_FLAGS(0));

        let heap = self.heaps.get(&heap_handle).expect("Unknown heap used");
        let mut heap = heap.lock().unwrap();

        heap.free(ptr).expect("Freeing memory failed");

        BOOL(1)
    }
}
