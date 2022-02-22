use crate::conv::FromIntoMemory;
use crate::ptr::TargetPtrRepr;
use smallvec::{smallvec, SmallVec};
use std::marker::PhantomData;

pub trait MemoryCtx<'a>: Copy + 'a {
    // TODO: should those be safe?
    // On one hand, they are unsafe, because they allow non-memory safe stuff to happen,
    //   like segfaulting and writing to random regions of the memory
    // On the other hand, this is really contained to the guest address space
    //   and we are not prohibited from doing any kind of despotism with it
    // segfaults in this case are __kind of__ deterministic, so no UB there
    // otherwise every glue operation with the target address space would have
    // to be unsafe which is noisy
    // TODO: does it make sense to return error sometimes? Maybe straight away panic is better?
    fn read<N: FromIntoMemory>(&self, ptr: TargetPtrRepr) -> Result<N, N::Error>;
    fn write<N: FromIntoMemory>(&self, value: N, ptr: TargetPtrRepr) -> Result<(), N::Error>;
}

/// Memory context represented as one contiguous virtual memory space allocation of 2^32 bytes
///
/// Not that not all the bytes are addressable; they are merely reserved for possible allocations
///
/// The lifetime denotes the period of validity of the allocated virtual space
#[cfg(target_pointer_width = "64")]
#[derive(Copy, Clone)]
pub struct FlatMemoryCtx<'a> {
    base: *mut u8,
    _phantom: PhantomData<&'a ()>,
}

#[cfg(target_pointer_width = "64")]
impl<'a> FlatMemoryCtx<'a> {
    pub fn new(base: *mut u8) -> Self {
        Self {
            base,
            _phantom: Default::default(),
        }
    }

    #[inline]
    pub fn to_native_ptr(&self, ptr: TargetPtrRepr) -> *mut u8 {
        // SAFETY: we are on 64-bit platform and allocated base ptr should point to 2 ** 32 bytes of addressable vitual address space
        // thus there would be no overflow and .add should work
        // It is also arguably part of the same "allocated object" but how to defined it is a bit hard to answer
        unsafe { self.base.add(ptr as usize) }
    }
}

#[cfg(target_pointer_width = "64")]
impl<'a> MemoryCtx<'a> for FlatMemoryCtx<'a> {
    fn read<N: FromIntoMemory>(&self, ptr: TargetPtrRepr) -> Result<N, N::Error> {
        let size = N::size();
        let unsafe_data = self.to_native_ptr(ptr);

        // Using SmallVec here actually allows rustc to optimize most cases down to just simple copy operation
        // size is subject to change (is there even a penalty for it being too large, with rustc optimizing stuff out?)
        let mut data: SmallVec<[_; 0x800]> = smallvec![0u8; size];
        unsafe { std::ptr::copy_nonoverlapping(unsafe_data, data.as_mut_ptr(), size) }
        N::try_from_bytes(&data)
    }

    fn write<N: FromIntoMemory>(&self, value: N, ptr: TargetPtrRepr) -> Result<(), N::Error> {
        let size = N::size();
        let mut data: SmallVec<[_; 0x800]> = smallvec![0u8; size];
        value.try_into_bytes(&mut data)?;
        let unsafe_data = self.to_native_ptr(ptr);
        unsafe { std::ptr::copy_nonoverlapping(data.as_ptr(), unsafe_data, size) }
        Ok(())
    }
}

// TODO: lifetimes?
pub type DefaultMemoryCtx<'a> = FlatMemoryCtx<'a>;
