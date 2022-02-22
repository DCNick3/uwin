use crate::conv::FromIntoMemory;
use crate::ptr::TargetPtrRepr;
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
    fn read<N: FromIntoMemory>(&self, ptr: TargetPtrRepr) -> Result<N, N::Error>;
    fn write<N: FromIntoMemory>(&self, value: N, ptr: TargetPtrRepr) -> Result<(), N::Error>;
}

// this wouldn't work with 32-bit architectures
#[cfg(target_pointer_width = "64")]
#[derive(Copy, Clone)]
struct FlatMemoryCtx<'a> {
    base: *mut u8,
    _phantom: PhantomData<&'a ()>,
}

#[cfg(target_pointer_width = "64")]
impl<'a> FlatMemoryCtx<'a> {
    // SAFETY: don't do stupid things. Do not store references to the returned slice between yields to recompiled code
    // The thing is that this mut is not actually exclusive...
    #[inline]
    unsafe fn to_slice_mut(&self, ptr: TargetPtrRepr, len: usize) -> &'a mut [u8] {
        std::slice::from_raw_parts_mut(self.to_host_ptr(ptr), len)
    }

    #[inline]
    pub fn to_host_ptr(&self, ptr: TargetPtrRepr) -> *mut u8 {
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
        // TBH, this is probably filled with UB, but I am not sure if it is even defined as UB...
        // rust wants slice to come from one allocation, but the flat memory context comes from
        let unsafe_data = unsafe { self.to_slice_mut(ptr, size) };
        // we could pass unsafe_data slice directly, but it's inherently unsafe slice (it can be changed by recompiled code),
        // so we copy it before allowing other code to work with it
        // TODO: optimize for small cases (prolly alloc on stack for size < 256 or smth)
        let data = unsafe_data.to_vec();
        N::try_from_bytes(&data)
    }

    fn write<N: FromIntoMemory>(&self, value: N, ptr: TargetPtrRepr) -> Result<(), N::Error> {
        let size = N::size();
        let mut data = vec![0u8; size];
        value.try_into_bytes(&mut data)?;
        let unsafe_data = unsafe { self.to_slice_mut(ptr, size) };
        unsafe_data.copy_from_slice(&data);
        Ok(())
    }
}

// TODO: lifetimes?
type DefaultMemoryCtx<'a> = FlatMemoryCtx<'a>;
