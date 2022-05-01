use btreemultimap::{btreemultimap, BTreeMultiMap};
use core_mem::align;
use core_mem::ctx::MemoryCtx;
use core_mem::ptr::{ConstPtr, MutPtr, PtrDiffRepr, PtrRepr};
use core_memmgr::{AddressRange, MemoryManager, Protection};
use derive_more::{Add, From, Into, Sub};
use std::collections::{HashMap, HashSet};
use std::mem::ManuallyDrop;
use std::ops::Add;
use std::sync::{Arc, Mutex};
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
    #[allow(unused)]
    orig_size: Size,
    #[allow(unused)]
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

const ALLOC_ALIGN: Size = Size(16);

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
        let size = Size(align::ceil(size.0, ALLOC_ALIGN.0));

        let full_size = match self.free_list.range(size..).next() {
            None => return Ok(None),
            Some((&k, _)) => k,
        };

        let leftover_size = full_size - size;
        assert!(leftover_size == Size(0) || leftover_size >= ALLOC_ALIGN);

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

    fn contains_ptr(&self, ptr: Ptr) -> bool {
        self.region.contains_addr(ptr.into())
    }

    pub fn free(&mut self, ptr: Ptr) -> Result<()> {
        let entry = self.entries.remove(&ptr).ok_or(Error::PointerUnknown)?;

        // TODO: coalesce adjacent free blocks
        // Right now we wouldn't be able to use freed block for larger allocations...
        self.free_list
            .insert(entry.allocated_size, FreeEntry { address: ptr });

        Ok(())
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
    memory_manager: Arc<Mutex<MemoryManager>>,
    #[allow(unused)]
    growable: bool,
    segments: ManuallyDrop<Vec<Segment>>,
    mmap_allocations: ManuallyDrop<HashSet<Ptr>>,
}

pub struct HeapOptions {
    // there also are flags HEAP_GENERATE_EXCEPTIONS and HEAP_NO_SERIALIZE, but not implemented yet
    pub initial_size: Option<PtrRepr>,
    pub maximum_size: Option<PtrRepr>,
}

const MMAP_THRESHOLD: Size = Size(64 * 1024); // 64 KiB

// TODO: TEST TEST TEST
impl Heap {
    pub fn new(mgr: Arc<Mutex<MemoryManager>>, options: HeapOptions) -> Result<Self> {
        let mgr_locked = mgr.clone();
        let mut mgr_locked = mgr_locked.lock().unwrap();
        let first_segment_size: Size = options
            .maximum_size
            .unwrap_or(4 * 1024 * 1024 /* 4 MiB */)
            .into();
        let growable = options.maximum_size.is_none();

        let segment = Segment::new(&mut mgr_locked, first_segment_size)?;

        Ok(Self {
            memory_manager: mgr,
            growable,
            segments: ManuallyDrop::new(vec![segment]),
            mmap_allocations: Default::default(),
        })
    }

    pub fn handle(&self) -> PtrDiffRepr {
        // shouldn't be deallocated
        self.segments[0].region.start as PtrDiffRepr
    }

    pub fn alloc(&mut self, size: PtrRepr, zero: bool) -> Result<PtrRepr> {
        let size = Size(size);

        if size > MMAP_THRESHOLD {
            let mut mgr = self.memory_manager.lock().unwrap();
            let region = mgr.reserve_and_commit_dynamic(size.into(), Protection::READ_WRITE)?;
            self.mmap_allocations.insert(Ptr(region.start));

            return Ok(region.start);
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
                let ptr = self
                    .memory_manager
                    .lock()
                    .unwrap()
                    .memory_context()
                    .to_native_ptr(ptr.into());
                std::ptr::write_bytes(ptr, 0, size.0 as usize);
            }
        }

        Ok(ptr.into())
    }

    pub fn free(&mut self, ptr: PtrRepr) -> Result<()> {
        let ptr: Ptr = ptr.into();
        if self.mmap_allocations.contains(&ptr) {
            let mut mgr = self.memory_manager.lock().unwrap();
            mgr.uncommit_and_unreserve(ptr.into())?;
            assert!(self.mmap_allocations.remove(&ptr));
            return Ok(());
        }
        let segment = self
            .segments
            .iter_mut()
            .find(|s| s.contains_ptr(ptr))
            .ok_or(Error::PointerUnknown)?;

        segment.free(ptr)
    }
}

impl Drop for Heap {
    fn drop(&mut self) {
        let mut mgr = self.memory_manager.lock().unwrap();

        let mmap_allocations = unsafe { ManuallyDrop::take(&mut self.mmap_allocations) };
        for mmap_alloc in mmap_allocations {
            mgr.uncommit_and_unreserve(mmap_alloc.into())
                .expect("Deallocating an mmap'ed block")
        }

        let segments = unsafe { ManuallyDrop::take(&mut self.segments) };
        for segment in segments {
            segment.drop(&mut mgr)
        }
    }
}

pub struct RawHeapBox {
    ptr: Ptr,
    heap: Arc<Mutex<Heap>>,
}

impl RawHeapBox {
    pub fn new(heap: Arc<Mutex<Heap>>, size: PtrRepr) -> Result<Self> {
        let ptr = {
            let mut heap = heap.lock().unwrap();
            heap.alloc(size, false)?
        };
        Ok(Self {
            heap,
            ptr: ptr.into(),
        })
    }

    pub fn new_init(
        memory_ctx: impl MemoryCtx,
        heap: Arc<Mutex<Heap>>,
        value: &[u8],
    ) -> Result<Self> {
        let ptr = {
            let mut heap = heap.lock().unwrap();
            MutPtr::<u8>::new(heap.alloc(value.len() as PtrRepr, false)?)
        };
        for (i, &v) in value.iter().enumerate() {
            ptr.offset(i as PtrDiffRepr).write_with(memory_ctx, v)
        }
        Ok(Self {
            heap,
            ptr: ptr.repr().into(),
        })
    }

    pub fn repr(&self) -> PtrRepr {
        self.ptr.0
    }

    pub fn leak(self) -> PtrRepr {
        let res = self.ptr.0;
        std::mem::forget(self);
        res
    }

    pub fn ptr<T>(&self) -> ConstPtr<T> {
        ConstPtr::new(self.ptr.0)
    }

    pub fn ptr_mut<T>(&self) -> MutPtr<T> {
        MutPtr::new(self.ptr.0)
    }
}

impl Drop for RawHeapBox {
    fn drop(&mut self) {
        let mut heap = self.heap.lock().unwrap();
        heap.free(self.ptr.0)
            .expect("Freeing a value in a RawHeapBox");
    }
}
