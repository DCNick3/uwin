use crate::ProcessContext;
use core_mem::ptr::{ConstPtr, MutPtr, PtrRepr};
use std::ffi::c_void;
use std::sync::Mutex;
use win32::Win32::Foundation::BOOL;
use win32::Win32::System::Memory::{
    HeapHandle, HEAP_FLAGS, HEAP_NO_SERIALIZE, PAGE_PROTECTION_FLAGS, VIRTUAL_ALLOCATION_TYPE,
};
use win32_heapmgr::HeapMgr;
use win32_virtmem::VirtualMemoryManager;

pub struct Memory {
    pub process_ctx: ProcessContext,
    pub virtmem_mgr: VirtualMemoryManager,
    pub heap_mgr: Mutex<HeapMgr>,
    pub process_heap_handle: HeapHandle,
}

#[allow(non_snake_case)]
impl win32::Win32::System::Memory::Api for Memory {
    fn GetProcessHeap(&self) -> HeapHandle {
        self.process_heap_handle
    }

    fn HeapAlloc(
        &self,
        h_heap: HeapHandle,
        dw_flags: HEAP_FLAGS,
        dw_bytes: PtrRepr,
    ) -> MutPtr<c_void> {
        let heap_mgr = self.heap_mgr.lock().unwrap();

        let res = heap_mgr.alloc(h_heap, dw_bytes, dw_flags);

        MutPtr::new(res)
    }

    fn HeapCreate(
        &self,
        fl_options: HEAP_FLAGS,
        dw_initial_size: PtrRepr,
        dw_maximum_size: PtrRepr,
    ) -> HeapHandle {
        // we ignore HEAP_NO_SERIALIZE (it's hard in Rust, lol)
        assert_eq!(fl_options & !HEAP_NO_SERIALIZE, HEAP_FLAGS(0));

        let mut heap_mgr = self.heap_mgr.lock().unwrap();

        heap_mgr.create(dw_initial_size, dw_maximum_size)
    }

    fn HeapFree(&self, h_heap: HeapHandle, dw_flags: HEAP_FLAGS, lp_mem: ConstPtr<c_void>) -> BOOL {
        let heap_mgr = self.heap_mgr.lock().unwrap();

        heap_mgr.free(h_heap, lp_mem.repr(), dw_flags)
    }

    fn VirtualAlloc(
        &self,
        lp_address: ConstPtr<c_void>,
        dw_size: PtrRepr,
        fl_allocation_type: VIRTUAL_ALLOCATION_TYPE,
        fl_protect: PAGE_PROTECTION_FLAGS,
    ) -> MutPtr<c_void> {
        MutPtr::new(self.virtmem_mgr.alloc(
            lp_address.repr(),
            dw_size,
            fl_allocation_type,
            fl_protect,
        ))
    }
}
