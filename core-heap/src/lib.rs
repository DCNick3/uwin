use btreemultimap::{btreemultimap, BTreeMultiMap};
use core_mem::ptr::PtrRepr;
use core_memmgr::{AddressRange, MemoryManager, Protection};
use derive_more::{From, Into};
use std::collections::{HashMap, HashSet};
use std::mem::ManuallyDrop;
use thiserror::Error;

// newtypes for addresses and sizes not to mix them up
#[derive(Copy, Clone, PartialEq, Eq, From, Into)]
struct Ptr(PtrRepr);
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, From, Into)]
struct Size(PtrRepr);

#[derive(Debug, Error)]
pub enum Error {
    #[error("Memory manager error occurred: {0}")]
    MemoryManagerError(#[from] core_memmgr::Error),
    #[error(
        "Attempt to perform an operation with a memory block that was not allocated by this heap"
    )]
    PointerUnknown,
}

type Result<T> = std::result::Result<T, Error>;

struct Entry {
    orig_size: Size,
    allocated_size: Size,
}

struct FreeEntry {
    address: Ptr,
}

struct Segment {
    region: AddressRange,
    entries: HashMap<Ptr, Entry>,
    allocated_size: Size,
    free_list: BTreeMultiMap<Size, FreeEntry>,
}

impl Segment {
    fn new(mgr: &mut MemoryManager, size: Size) -> Result<Self> {
        let region = mgr.reserve_and_commit_dynamic(size.into(), Protection::READ_WRITE)?;

        Ok(Self {
            region,
            entries: Default::default(),
            allocated_size: 0.into(),
            free_list: btreemultimap![region.size.into() => FreeEntry { address: region.start.into() }],
        })
    }

    /// !!! invalidates all allocations within the segment
    fn free(self, mgr: &mut MemoryManager) {
        // can't easily make it fallible, because we already consumed "self"
        mgr.uncommit_and_unreserve(self.region.start).unwrap();
        std::mem::forget(self);
    }
}

impl Drop for Segment {
    fn drop(&mut self) {
        panic!("Forgot to free the segment")
    }
}

pub struct Heap {
    growable: bool,
    segments: ManuallyDrop<Vec<Segment>>,
    mmap_allocations: ManuallyDrop<HashSet<Ptr>>,
}

impl Drop for Heap {
    fn drop(&mut self) {
        panic!("Forgot to free the Heap")
    }
}

pub struct HeapOptions {
    // there also are flags HEAP_GENERATE_EXCEPTIONS and HEAP_NO_SERIALIZE, but not implemented yet
    pub initial_size: Option<PtrRepr>,
    pub maximum_size: Option<PtrRepr>,
}

impl Heap {
    pub fn new(mgr: &mut MemoryManager, options: HeapOptions) -> Result<Self> {
        let first_segment_size: Size = options
            .maximum_size
            .or(options.initial_size)
            .unwrap_or(4 * 1024 * 1024 /* 4 MiB */)
            .into();
        let growable = options.maximum_size.is_none();

        let segment = Segment::new(mgr, first_segment_size)?;

        Ok(Self {
            growable,
            segments: ManuallyDrop::new(vec![segment]),
            mmap_allocations: Default::default(),
        })
    }

    pub fn handle(&self) -> PtrRepr {
        // shouldn't be deallocated
        self.segments[0].region.start
    }

    fn free(mut self, mgr: &mut MemoryManager) {
        let mmap_allocations = unsafe { ManuallyDrop::take(&mut self.mmap_allocations) };
        for mmap_alloc in mmap_allocations {
            mgr.uncommit_and_unreserve(mmap_alloc.into())
                .expect("Deallocating an mmap'ed block")
        }

        let segments = unsafe { ManuallyDrop::take(&mut self.segments) };
        for segment in segments {
            segment.free(mgr)
        }

        std::mem::forget(self)
    }
}
