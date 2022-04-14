use crate::conv::FromIntoMemory;
use crate::ptr::PtrRepr;
use smallvec::{smallvec, SmallVec};

pub trait MemoryCtx: Copy + PartialEq {
    // TODO: should those be safe?
    // On one hand, they are unsafe, because they allow non-memory safe stuff to happen,
    //   like segfaulting and writing to random regions of the memory
    // On the other hand, this is really contained to the guest address space
    //   and we are not prohibited from doing any kind of despotism with it
    // segfaults in this case are __kind of__ deterministic, so no UB there
    // otherwise every glue operation with the target address space would have
    // to be unsafe which is noisy
    // TODO: does it make sense to return error sometimes? Maybe straight away panic is better?
    fn read<N: FromIntoMemory>(&self, ptr: PtrRepr) -> N;
    fn write<N: FromIntoMemory>(&self, value: N, ptr: PtrRepr);
}

// A future idea: make

/// Memory context represented as one contiguous virtual memory space allocation of 2^32 bytes
///
/// Not that not all the bytes are addressable; they are merely reserved for possible allocations
///
/// The lifetime denotes the period of validity of the allocated virtual space
#[cfg(target_pointer_width = "64")]
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)] // to pass FlatMemoryCtx into recompiled functions instead of raw pointers
pub struct FlatMemoryCtx {
    base: *mut u8,
}

#[cfg(target_pointer_width = "64")]
impl FlatMemoryCtx {
    /// # Safety
    /// You must ensure that lifetime of FlatMemoryCtx will not outlive the memory mapping
    pub unsafe fn new(base: *mut u8) -> Self {
        Self { base }
    }

    #[inline]
    pub fn to_native_ptr(&self, ptr: PtrRepr) -> *mut u8 {
        // SAFETY: we are on 64-bit platform and allocated base ptr should point to 2 ** 32 bytes of addressable vitual address space
        // thus there would be no overflow and .add should work
        // It is also arguably part of the same "allocated object" but how to defined it is a bit hard to answer
        unsafe { self.base.add(ptr as usize) }
    }
}

impl From<FlatMemoryCtx> for *mut u8 {
    fn from(ctx: FlatMemoryCtx) -> Self {
        ctx.base
    }
}

#[cfg(target_pointer_width = "64")]
impl MemoryCtx for FlatMemoryCtx {
    fn read<N: FromIntoMemory>(&self, ptr: PtrRepr) -> N {
        let size = N::size();
        let unsafe_data = self.to_native_ptr(ptr);

        // Using SmallVec here actually allows rustc to optimize most cases down to just simple copy operation
        // size is subject to change (is there even a penalty for it being too large, with rustc optimizing stuff out?)
        let mut data: SmallVec<[_; 0x800]> = smallvec![0u8; size];
        unsafe { std::ptr::copy_nonoverlapping(unsafe_data, data.as_mut_ptr(), size) }
        N::from_bytes(&data)
    }

    fn write<N: FromIntoMemory>(&self, value: N, ptr: PtrRepr) {
        let size = N::size();
        let mut data: SmallVec<[_; 0x800]> = smallvec![0u8; size];
        value.into_bytes(&mut data);
        let unsafe_data = self.to_native_ptr(ptr);
        unsafe { std::ptr::copy_nonoverlapping(data.as_ptr(), unsafe_data, size) }
    }
}

pub type DefaultMemoryCtx = FlatMemoryCtx;
