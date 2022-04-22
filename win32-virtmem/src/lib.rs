use core_mem::ptr::PtrRepr;
use core_memmgr::{AddressRange, MemoryManager, Protection};
use std::sync::{Arc, Mutex};
use win32::Win32::System::Memory::{
    MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE, PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE,
    PAGE_NOACCESS, PAGE_PROTECTION_FLAGS, PAGE_READONLY, PAGE_READWRITE, VIRTUAL_ALLOCATION_TYPE,
};

pub struct VirtualMemoryManager {
    mem_mgr: Arc<Mutex<MemoryManager>>,
}

impl VirtualMemoryManager {
    pub fn new(mem_mgr: Arc<Mutex<MemoryManager>>) -> Self {
        Self { mem_mgr }
    }

    fn flags_to_protection(protect: PAGE_PROTECTION_FLAGS) -> Protection {
        if protect == PAGE_NOACCESS {
            Protection::NONE
        } else if protect == PAGE_READONLY {
            Protection::READ
        } else if protect == PAGE_READWRITE {
            Protection::READ_WRITE
        } else if protect == PAGE_EXECUTE {
            Protection::EXECUTE
        } else if protect == PAGE_EXECUTE_READ {
            Protection::READ_EXECUTE
        } else if protect == PAGE_EXECUTE_READWRITE {
            Protection::READ_WRITE_EXECUTE
        } else {
            unimplemented!("Unsupported protection flags: {:?}", protect)
        }
    }

    pub fn alloc(
        &self,
        address: PtrRepr,
        size: PtrRepr,
        allocation_type: VIRTUAL_ALLOCATION_TYPE,
        protect: PAGE_PROTECTION_FLAGS,
    ) -> PtrRepr {
        assert_eq!(
            allocation_type & !(MEM_COMMIT | MEM_RESERVE),
            VIRTUAL_ALLOCATION_TYPE(0)
        );
        let allocation_type = allocation_type & (MEM_COMMIT | MEM_RESERVE);

        let protection = Self::flags_to_protection(protect);

        let mut mgr = self.mem_mgr.lock().unwrap();

        if allocation_type == MEM_RESERVE | MEM_COMMIT {
            if address != 0 {
                todo!("Static reserve & commit")
            } else {
                todo!("Dynamic reserve & commit")
            }
        } else if allocation_type == MEM_RESERVE {
            if address != 0 {
                todo!("Static reserve")
            } else {
                mgr.reserve_dynamic(size).expect("Reserving memory").start
            }
        } else if allocation_type == MEM_COMMIT {
            assert_ne!(address, 0, "Address must be specified for commit");

            mgr.commit(AddressRange::new(address, size), protection)
                .expect("Memory commit")
                .start
        } else {
            panic!("No operation specified for virtual memory alloc?")
        }
    }
}
