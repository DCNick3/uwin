use btreemultimap::{btreemultimap, BTreeMultiMap};
use core_mem::align;
use core_mem::ptr::PtrRepr;
use core_memmgr::{AddressRange, MemoryManager, Protection};
use derive_more::{Add, From, Into, Sub};
use std::collections::{HashMap, HashSet};
use std::mem::ManuallyDrop;
use std::ops::Add;
use std::sync::Mutex;
use thiserror::Error;

// newtypes for addresses and sizes not to mix them up
#[derive(Copy, Clone, PartialEq, Eq, From, Into, Hash)]
struct Ptr(PtrRepr);
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, From, Into, Add, Sub, Hash)]
struct Size(PtrRepr);

impl Add<Size> for Ptr {
    type Output = Ptr;

    fn add(self, rhs: Size) -> Self::Output {
        Ptr(self.0 + rhs.0)
    }
}

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
    free_list: BTreeMultiMap<Size, FreeEntry>,
}

const MIN_ALLOC: Size = Size(16);

impl Segment {
    fn new(mgr: &mut MemoryManager, size: Size) -> Result<Self> {
        let region = mgr.reserve_and_commit_dynamic(size.into(), Protection::READ_WRITE)?;

        Ok(Self {
            region,
            entries: Default::default(),
            free_list: btreemultimap![region.size.into() => FreeEntry { address: region.start.into() }],
        })
    }

    fn try_alloc(&mut self, size: Size) -> Result<Option<Ptr>> {
        let orig_size = size;
        let size = Size(align::ceil(size.0, MIN_ALLOC.0));

        let full_size = match self.free_list.range(size..).next() {
            None => return Ok(None),
            Some((&k, _)) => k,
        };

        let leftover_size = full_size - size;
        assert!(leftover_size == Size(0) || leftover_size >= MIN_ALLOC);

        let size_list = self.free_list.get_vec_mut(&full_size).unwrap();

        let free_entry = size_list.pop().unwrap(); // can this fail?

        if size_list.is_empty() {
            self.free_list.remove(&full_size).unwrap();
        }

        if leftover_size > Size(0) {
            let leftover_entry = free_entry.address + size;
            let leftover_entry = FreeEntry {
                address: leftover_entry,
            };
            self.free_list.insert(leftover_size, leftover_entry)
        }

        let address = free_entry.address;

        let entry = Entry {
            allocated_size: size,
            orig_size,
        };

        assert!(self.entries.insert(address, entry).is_none());

        Ok(Some(address))
    }

    /// !!! invalidates all allocations within the segment
    fn drop(self, mgr: &mut MemoryManager) {
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

const MMAP_THRESHOLD: Size = Size(64 * 1024); // 64 KiB

impl Heap {
    pub fn new(mgr: &mut MemoryManager, options: HeapOptions) -> Result<Self> {
        let first_segment_size: Size = options
            .maximum_size
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

    pub fn alloc(
        &mut self,
        mgr: &Mutex<MemoryManager>, /* will only lock the mm is needed */
        size: PtrRepr,
        zero: bool,
    ) -> Result<PtrRepr> {
        let size = Size(size);

        if size > MMAP_THRESHOLD {
            todo!("Allocation using mmap")
        }

        let mut iter = self.segments.iter_mut();
        let ptr = loop {
            if let Some(seg) = iter.next() {
                let r = seg.try_alloc(size)?;
                if r.is_some() {
                    break r;
                }
            } else {
                break None;
            }
        };

        let ptr = match ptr {
            None => todo!("Heap growing"),
            Some(ptr) => ptr,
        };

        if zero {
            // TODO: This is clearly suboptimal (we lock mutex just to get a memory context...)
            unsafe {
                let ptr = mgr
                    .lock()
                    .unwrap()
                    .memory_context()
                    .to_native_ptr(ptr.into());
                std::ptr::write_bytes(ptr, 0, size.0 as usize);
            }
        }

        Ok(ptr.into())
    }

    pub fn drop(mut self, mgr: &mut MemoryManager) {
        let mmap_allocations = unsafe { ManuallyDrop::take(&mut self.mmap_allocations) };
        for mmap_alloc in mmap_allocations {
            mgr.uncommit_and_unreserve(mmap_alloc.into())
                .expect("Deallocating an mmap'ed block")
        }

        let segments = unsafe { ManuallyDrop::take(&mut self.segments) };
        for segment in segments {
            segment.drop(mgr)
        }

        std::mem::forget(self)
    }
}
